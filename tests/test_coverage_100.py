"""Target remaining uncovered lines in python/rsn_db (CLI, bridge, easy)."""

from __future__ import annotations

from unittest.mock import MagicMock, patch

import pytest


def test_load_prefs_invalid_json(tmp_path, monkeypatch):
    from rsn_db import cli

    prefs_file = tmp_path / "prefs.json"
    prefs_file.write_text("{not json", encoding="utf-8")
    monkeypatch.setattr(cli, "PREFS_FILE", prefs_file)
    assert cli.load_prefs() == {}


def test_handle_mempalace_import_unavailable(capsys):
    from rsn_db.cli import _handle_mempalace

    with patch.dict(
        "sys.modules",
        {"rsn_db.mempalace_bridge": None},
    ):
        assert _handle_mempalace("MEMPALACE SEARCH q", {}) is True
    assert "MemPalace unavailable" in capsys.readouterr().err


def test_handle_mempalace_unknown_and_error(capsys):
    from rsn_db.cli import _handle_mempalace

    with patch("rsn_db.mempalace_bridge.MemPalaceBridge") as cls:
        inst = cls.return_value
        inst.search_text.side_effect = RuntimeError("boom")
        assert _handle_mempalace("MEMPALACE SEARCH q", {}) is True
        assert "MemPalace error" in capsys.readouterr().err

    with patch("rsn_db.mempalace_bridge.MemPalaceBridge"):
        assert _handle_mempalace("MEMPALACE NOPE", {}) is True
        assert "Unknown MEMPALACE" in capsys.readouterr().err


def test_print_result_empty_and_scalar(capsys):
    from rsn_db.cli import _print_result

    _print_result(None, as_json=False)
    _print_result("", as_json=False)
    assert capsys.readouterr().out == ""

    _print_result("scalar", as_json=False)
    assert "scalar" in capsys.readouterr().out


def test_run_repl_paths(monkeypatch, capsys):
    from rsn_db.cli import run_repl

    db = MagicMock()
    db.execute_sql.return_value = ["row"]
    inputs = iter(["HELP", "TABLES", "MEMPALACE HELP", "EXIT"])
    monkeypatch.setattr("builtins.input", lambda _: next(inputs))

    with patch("rsn_db.cli._handle_mempalace", side_effect=[False, False, True, False]):
        run_repl(db, {}, json_out=False)
    out = capsys.readouterr().out
    assert "TABLES" in out
    db.save.assert_called_once()


def test_run_repl_eof_and_sql_error(monkeypatch, capsys):
    from rsn_db.cli import run_repl

    db = MagicMock()
    db.execute_sql.side_effect = ValueError("bad sql")
    calls = iter(["TABLES", EOFError])

    def fake_input(_):
        val = next(calls)
        if val is EOFError:
            raise EOFError
        return val

    monkeypatch.setattr("builtins.input", fake_input)
    run_repl(db, {}, json_out=True)
    err = capsys.readouterr().err
    assert "bad sql" in err


def test_main_interactive_mode_selection(tmp_path, monkeypatch):
    from rsn_db import cli

    monkeypatch.setattr(cli, "PREFS_FILE", tmp_path / "prefs.json")
    monkeypatch.setattr(cli, "run_repl", lambda *a, **k: None)
    inputs = iter(["3", "y"])
    monkeypatch.setattr("builtins.input", lambda _: next(inputs))

    with patch.object(cli, "Database") as db_cls:
        db_cls.return_value.execute_sql.return_value = "pulse"
        assert cli.main([]) == 0
    assert cli.load_prefs().get("mode") == "snarky"


def test_main_snarky_starts_repl(monkeypatch):
    from rsn_db import cli

    called = []

    def capture_repl(db, prefs, *, json_out):
        called.append(json_out)

    monkeypatch.setattr(cli, "run_repl", capture_repl)
    with patch.object(cli, "Database") as db_cls:
        db_cls.return_value.execute_sql.return_value = "alive"
        assert cli.main(["--no-prompt", "--mode", "snarky"]) == 0
    assert called == [False]


def test_main_mempalace_one_shot():
    from rsn_db import cli

    with patch.object(cli, "_handle_mempalace", return_value=True):
        assert cli.main(["--no-prompt", "-c", "MEMPALACE STATUS"]) == 0


def test_easy_inner_and_remember_raises(tmp_path):
    from rsn_db.easy import RsnDatabase

    db = RsnDatabase(str(tmp_path / "x.rsndb"), session_memory=False)
    assert db.inner is db._inner
    with pytest.raises(RuntimeError, match="MemPalace"):
        db.remember("no palace")


def test_bridge_available_false(tmp_path):
    from rsn_db.mempalace_bridge import MemPalaceBridge

    bridge = MemPalaceBridge.__new__(MemPalaceBridge)
    bridge.palace_path = str(tmp_path)
    with patch("rsn_db.mempalace_bridge._require_mempalace", side_effect=ImportError):
        assert bridge.available is False


@patch("rsn_db.mempalace_bridge._require_mempalace")
def test_bridge_init_and_mine_failures(_mock, tmp_path):
    from rsn_db.mempalace_bridge import MemPalaceBridge

    with patch("mempalace.config.MempalaceConfig") as cfg, patch(
        "subprocess.run"
    ) as run:
        cfg.return_value.palace_path = str(tmp_path)
        run.return_value = MagicMock(returncode=1, stdout="", stderr="fail")
        bridge = MemPalaceBridge(palace_path=str(tmp_path))
        with pytest.raises(RuntimeError, match="init failed"):
            bridge.init_palace(str(tmp_path))
        with pytest.raises(RuntimeError, match="mine failed"):
            bridge.mine_path(str(tmp_path), mode="convos")


@patch("rsn_db.mempalace_bridge._require_mempalace")
def test_bridge_search_formats(_mock, tmp_path):
    from rsn_db.mempalace_bridge import MemPalaceBridge

    with patch("mempalace.config.MempalaceConfig") as cfg, patch(
        "mempalace.searcher.search_memories"
    ) as search, patch("mempalace.layers.MemoryStack") as stack:
        cfg.return_value.palace_path = str(tmp_path)
        search.return_value = {"query": "q", "results": ["plain-hit"]}
        stack.return_value.wake_up.return_value = "wake"
        stack.return_value.status.return_value = "status"
        bridge = MemPalaceBridge(palace_path=str(tmp_path))
        assert "plain-hit" in bridge.search_text("q")
        assert bridge.wake_up() == "wake"
        assert bridge.status() == "status"
        search.return_value = {"query": "q", "results": []}
        assert "No MemPalace matches" in bridge.search_text("q")


@patch("rsn_db.mempalace_bridge._require_mempalace")
def test_bridge_sync_empty(_mock, tmp_path):
    from rsn_db import Database
    from rsn_db.mempalace_bridge import MemPalaceBridge

    with patch("mempalace.config.MempalaceConfig") as cfg:
        cfg.return_value.palace_path = str(tmp_path)
        bridge = MemPalaceBridge(palace_path=str(tmp_path))
        inner = Database(str(tmp_path / "empty.rsndb"))
        assert bridge.sync_rsn_graph_ingest(inner) == 0


def test_cli_main_guard():
    import rsn_db.cli as cli_mod

    with patch.object(cli_mod, "main", return_value=0):
        with pytest.raises(SystemExit) as exc:
            cli_mod.__name__ = "__main__"
            exec(compile("raise SystemExit(main())", "rsn_db/cli.py", "exec"), cli_mod.__dict__)
        assert exc.value.code == 0

def test_bridge_available_true(tmp_path):
    from rsn_db.mempalace_bridge import MemPalaceBridge

    with patch("rsn_db.mempalace_bridge._require_mempalace"), patch(
        "mempalace.config.MempalaceConfig"
    ) as cfg:
        cfg.return_value.palace_path = str(tmp_path)
        bridge = MemPalaceBridge(palace_path=str(tmp_path))
        assert bridge.available is True


@patch("rsn_db.mempalace_bridge._require_mempalace")
def test_bridge_sync_ignores_graph_errors(_mock, tmp_path):
    from rsn_db.mempalace_bridge import MemPalaceBridge

    db = MagicMock()
    db.graph_query.side_effect = RuntimeError("graph down")
    db.execute_sql.return_value = []
    with patch("mempalace.config.MempalaceConfig") as cfg:
        cfg.return_value.palace_path = str(tmp_path)
        bridge = MemPalaceBridge(palace_path=str(tmp_path))
        assert bridge.sync_rsn_graph_ingest(db) == 0


def test_cli_py_main_entry(tmp_path):
    from pathlib import Path
    import os
    import subprocess
    import sys

    root = Path(__file__).resolve().parents[1]
    env = os.environ.copy()
    env["PYTHONPATH"] = str(root / "python")
    script = root / "python" / "rsn_db" / "cli.py"
    proc = subprocess.run(
        [
            sys.executable,
            str(script),
            "--no-prompt",
            "-c",
            "TABLES",
            "--storage",
            str(tmp_path / "cli.rsndb"),
        ],
        env=env,
        capture_output=True,
        text=True,
        check=False,
    )
    assert proc.returncode == 0, proc.stderr

@patch("rsn_db.mempalace_bridge._require_mempalace")
def test_bridge_sync_with_graph_chunk(_mock, tmp_path):
    from rsn_db.mempalace_bridge import MemPalaceBridge

    db = MagicMock()
    db.graph_query.return_value = "Knowledge graph excerpt"
    db.execute_sql.return_value = []
    with patch("mempalace.config.MempalaceConfig") as cfg:
        cfg.return_value.palace_path = str(tmp_path)
        bridge = MemPalaceBridge(palace_path=str(tmp_path))
        with patch.object(bridge, "mine_path", return_value="ok") as mine:
            count = bridge.sync_rsn_graph_ingest(db)
        assert count == 1
        mine.assert_called_once()
