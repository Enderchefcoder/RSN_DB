"""Raise coverage on Python wrapper modules (mocks for heavy paths)."""

from unittest.mock import MagicMock, patch

import pytest

from rsn_db.easy import RsnDatabase, open_db
from rsn_db.mempalace_bridge import MemPalaceBridge


def test_rsn_database_mempalace_paths(tmp_path):
    db = RsnDatabase(str(tmp_path / "e.rsndb"), enable_mempalace=False)
    with pytest.raises(RuntimeError, match="MemPalace"):
        db.palace_search("x")
    with pytest.raises(RuntimeError, match="MemPalace"):
        db.palace_wake_up()
    with pytest.raises(RuntimeError, match="MemPalace"):
        db.sync_to_mempalace()


@patch("rsn_db.mempalace_bridge._require_mempalace")
def test_mempalace_init_and_mine(_mock_req, tmp_path):
    with patch("mempalace.config.MempalaceConfig") as cfg, patch(
        "subprocess.run"
    ) as run:
        cfg.return_value.palace_path = str(tmp_path)
        run.return_value = MagicMock(returncode=0, stdout="ok", stderr="")
        bridge = MemPalaceBridge(palace_path=str(tmp_path))
        assert "ok" in bridge.init_palace(str(tmp_path))
        run.return_value.stdout = "mined"
        assert "mined" in bridge.mine_path(str(tmp_path))


@patch("rsn_db.mempalace_bridge._require_mempalace")
def test_mempalace_sync_graph(_mock_req, tmp_path):
    from rsn_db import Database

    with patch("mempalace.config.MempalaceConfig") as cfg, patch(
        "subprocess.run"
    ) as run:
        cfg.return_value.palace_path = str(tmp_path)
        run.return_value = MagicMock(returncode=0, stdout="", stderr="")
        bridge = MemPalaceBridge(palace_path=str(tmp_path))
        inner = Database(str(tmp_path / "d.rsndb"))
        inner.create_table("t", {"n": {"type": "string", "required": True}})
        inner.insert("t", {"n": "a"})
        inner.ingest("Some graph text here.", source="s")
        assert bridge.sync_rsn_graph_ingest(inner) >= 0


@patch("rsn_db.mempalace_bridge._require_mempalace")
def test_mempalace_wake_status(_mock_req, tmp_path):
    with patch("mempalace.config.MempalaceConfig") as cfg, patch(
        "mempalace.layers.MemoryStack"
    ) as stack_cls:
        cfg.return_value.palace_path = str(tmp_path)
        stack_cls.return_value.wake_up.return_value = "wake"
        stack_cls.return_value.status.return_value = "status"
        bridge = MemPalaceBridge(palace_path=str(tmp_path))
        assert bridge.wake_up() == "wake"
        assert bridge.status() == "status"


@patch("rsn_db.mempalace_bridge._require_mempalace")
def test_mempalace_search_empty(_mock_req):
    with patch("mempalace.searcher.search_memories") as search, patch(
        "mempalace.config.MempalaceConfig"
    ) as cfg:
        cfg.return_value.palace_path = "/tmp/p"
        search.return_value = {"query": "q", "results": []}
        bridge = MemPalaceBridge(palace_path="/tmp/p")
        assert "No MemPalace" in bridge.search_text("q")


def test_remember_session_only(tmp_path):
    db = RsnDatabase(str(tmp_path / "m.rsndb"), enable_mempalace=False)
    msg = db.remember("note", role="user")
    assert "session" in msg.lower()
    assert db.memory is not None
    assert len(db.memory.recall()) == 1


def test_cli_parser_and_one_shot(tmp_path):
    from rsn_db.cli import build_parser, main

    with pytest.raises(SystemExit) as exc:
        build_parser().parse_args(["--version"])
    assert exc.value.code == 0

    storage = tmp_path / "cli.rsndb"
    assert (
        main(
            [
                "--no-prompt",
                "--mode",
                "professional",
                "--storage",
                str(storage),
                "-c",
                "SHOW TABLES",
            ]
        )
        == 0
    )


def test_cli_mempalace_branch(capsys):
    from rsn_db.cli import _handle_mempalace

    with patch("rsn_db.mempalace_bridge.MemPalaceBridge") as cls:
        inst = cls.return_value
        inst.search_text.return_value = "hits"
        assert _handle_mempalace("MEMPALACE SEARCH hello", {}) is True
        assert "hits" in capsys.readouterr().out


def test_easy_open_db_saves(tmp_path):
    with open_db(str(tmp_path / "o.rsndb")) as db:
        db.create_table("z", {"n": {"type": "string", "required": True}})
    assert (tmp_path / "o.rsndb").exists()
