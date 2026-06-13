"""Tests for official MemPalace integration."""

import os
import sys
from unittest.mock import MagicMock, patch

import pytest

from rsn_db.mempalace_bridge import MEMPALACE_INSTALL, OFFICIAL_DOCS, MemPalaceBridge


def test_install_hint_constants():
    assert "mempalace" in MEMPALACE_INSTALL.lower()
    assert "mempalaceofficial.com" in OFFICIAL_DOCS


def test_bridge_requires_mempalace_when_missing():
    with patch.dict(sys.modules, {"mempalace": None}):
        with pytest.raises(ImportError, match="MemPalace is not installed"):
            MemPalaceBridge()


@patch("rsn_db.mempalace_bridge._require_mempalace")
def test_remember_calls_add_drawer(_mock_req):
    mock_col = MagicMock()
    with patch("mempalace.palace.get_collection", return_value=mock_col), patch(
        "mempalace.miner.add_drawer"
    ) as add_drawer, patch("mempalace.config.MempalaceConfig") as cfg_cls:
        cfg_cls.return_value.palace_path = "/tmp/palace"
        bridge = MemPalaceBridge(palace_path="/tmp/palace")
        msg = bridge.remember("hello", wing="w", room="r")
        add_drawer.assert_called_once()
        assert "w" in msg


@patch("rsn_db.mempalace_bridge._require_mempalace")
def test_search_text_formats(_mock_req):
    with patch("mempalace.searcher.search_memories") as search, patch(
        "mempalace.config.MempalaceConfig"
    ) as cfg_cls:
        cfg_cls.return_value.palace_path = "/tmp/palace"
        search.return_value = {
            "query": "test",
            "results": [{"document": "hit", "metadata": {"wing": "rsn_db", "room": "facts"}}],
        }
        bridge = MemPalaceBridge(palace_path="/tmp/palace")
        out = bridge.search_text("test")
        assert "hit" in out


def test_easy_open_db(tmp_path):
    from rsn_db.easy import open_db

    with open_db(str(tmp_path / "t.rsndb")) as db:
        db.create_table("x", {"n": {"type": "string", "required": True}})
        db.insert("x", {"n": "a"})
    assert (tmp_path / "t.rsndb").exists()


def test_database_save_load_snapshot(tmp_path):
    from rsn_db import Database

    os.chdir(tmp_path)
    db = Database(storage_path="snap.rsndb")
    db.create_table("t", {"v": {"type": "string", "required": True}})
    db.insert("t", {"v": "1"})
    db.save()
    db.snapshot("copy.rsndb")
    assert (tmp_path / "copy.rsndb").exists()
    reloaded = Database(storage_path="snap.rsndb")
    assert reloaded.execute_sql("COUNT t") == 1
