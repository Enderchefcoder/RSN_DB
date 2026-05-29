"""Additional coverage for v0.4.0 Python surface."""

from unittest.mock import patch

import pytest

from rsn_db import Database, Query, RsnDatabase, __version__


def test_version():
    assert __version__ == "0.4.0"


def test_snarky_create_table_message(tmp_path):
    db = Database(str(tmp_path / "s.rsndb"), mode="snarky")
    msg = db.create_table("t", {"a": {"type": "string"}})
    assert msg is not None


def test_graph_ingest_and_query(tmp_path):
    db = Database(str(tmp_path / "g.rsndb"))
    db.ingest("Alice built RSN DB with Rust.", source="doc")
    result = db.graph_query("Alice")
    assert isinstance(result, str)


def test_rsn_database_delegate(tmp_path):
    db = RsnDatabase(str(tmp_path / "r.rsndb"))
    db.create_table("u", {"n": {"type": "string", "required": True}})
    db.insert("u", {"n": "bob"})
    assert len(db.query(Query("u").where_eq("n", "bob"))) == 1


def test_cli_mempalace_help(capsys):
    from rsn_db.cli import _handle_mempalace

    with patch("rsn_db.mempalace_bridge.MemPalaceBridge"):
        assert _handle_mempalace("MEMPALACE HELP", {}) is True
    assert "SEARCH" in capsys.readouterr().out
