"""Beginner helpers."""

from rsn_db.beginners import insert_many, quick_start, records_to_dicts, tutorial_commands
from rsn_db import Query


def test_quick_start_and_insert_many(tmp_path):
    db = quick_start(str(tmp_path / "b.rsndb"), mode="professional")
    db.create_table("items", {"name": {"type": "string", "required": True}})
    ids = insert_many(db, "items", [{"name": "a"}, {"name": "b"}])
    assert len(ids) == 2
    rows = records_to_dicts(db.query(Query("items")))
    assert len(rows) == 2


def test_tutorial_commands_non_empty():
    cmds = tutorial_commands()
    assert any("PULSE" in c for c in cmds)
