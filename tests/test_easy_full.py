"""Full RsnDatabase / open_db coverage."""

from unittest.mock import patch

from rsn_db.easy import RsnDatabase, open_db


def test_enable_mempalace_and_palace_methods(tmp_path):
    with patch("rsn_db.easy.MemPalaceBridge") as cls:
        inst = cls.return_value
        inst.search_text.return_value = "hits"
        inst.wake_up.return_value = "wake"
        inst.sync_rsn_graph_ingest.return_value = 2
        db = RsnDatabase(str(tmp_path / "p.rsndb"), session_memory=False)
        bridge = db.enable_mempalace("/tmp/palace")
        assert bridge is inst
        assert db.palace_search("q") == "hits"
        assert db.palace_wake_up() == "wake"
        assert db.sync_to_mempalace() == 2


def test_enable_mempalace_on_init(tmp_path):
    with patch("rsn_db.easy.MemPalaceBridge") as cls:
        RsnDatabase(str(tmp_path / "p.rsndb"), enable_mempalace=True, session_memory=False)
        cls.assert_called_once()


def test_remember_session_only(tmp_path):
    db = RsnDatabase(str(tmp_path / "p.rsndb"))
    assert "session memory" in db.remember("hello", role="user").lower()


def test_sync_via_session_memory(tmp_path):
    with patch("rsn_db.easy.MemPalaceBridge") as cls:
        inst = cls.return_value
        db = RsnDatabase(str(tmp_path / "p.rsndb"))
        db.enable_mempalace()
        db.remember("turn one", role="user")
        assert db.sync_to_mempalace() == 1
        inst.remember.assert_called()


def test_load_and_snapshot(tmp_path):
    import os

    os.chdir(tmp_path)
    db = RsnDatabase("s.rsndb")
    db.create_table("t", {"v": {"type": "string", "required": True}})
    db.insert("t", {"v": "1"})
    db.save()
    db.snapshot("copy.rsndb")
    db.load()
    assert (tmp_path / "copy.rsndb").exists()


def test_open_db_context(tmp_path):
    with open_db(str(tmp_path / "o.rsndb")) as db:
        db.create_table("z", {"n": {"type": "string", "required": True}})
    assert (tmp_path / "o.rsndb").exists()
