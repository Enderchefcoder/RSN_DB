from rsn_db import Database, Query
import pytest


def test_end_to_end(tmp_path):
    db = Database(str(tmp_path / "state.json"))
    db.create_table(
        "users",
        {
            "name": {"type": "string", "required": True},
            "email": {"type": "string", "required": True, "unique": True},
            "age": {"type": "integer"},
            "is_active": {"type": "boolean"},
        },
    )

    alice_id = db.insert(
        "users",
        {
            "name": "Alice",
            "email": "alice@example.com",
            "age": 30,
            "is_active": True,
        },
    )
    db.insert(
        "users",
        {
            "name": "Bob",
            "email": "bob@example.com",
            "age": 25,
            "is_active": False,
        },
    )

    db.update("users", alice_id, {"age": 31})

    result = db.query(Query("users").where_eq("is_active", True).order_by("age", True).take(5))
    assert len(result) == 1
    assert result[0].data["name"] == "Alice"

    assert db.execute_sql("COUNT users") == 2


def test_unknown_field_rejected(tmp_path):
    db = Database(str(tmp_path / "state.json"))
    db.create_table(
        "users",
        {
            "name": {"type": "string", "required": True},
            "email": {"type": "string", "required": True, "unique": True},
        },
    )

    with pytest.raises(ValueError, match="not part of the schema"):
        db.insert("users", {"name": "Eve", "email": "eve@example.com", "role": "admin"})


def test_jsonl_and_sqlite_roundtrip(tmp_path):
    db = Database(str(tmp_path / "state.json"))
    db.create_table(
        "users",
        {
            "name": {"type": "string", "required": True},
            "email": {"type": "string", "required": True, "unique": True},
            "age": {"type": "integer"},
        },
    )
    db.insert("users", {"name": "Ana", "email": "ana@example.com", "age": 30})

    jsonl_path = tmp_path / "users.jsonl"
    sqlite_path = tmp_path / "users.sqlite"

    db.export_jsonl("users", str(jsonl_path))
    db.export_sqlite("users", str(sqlite_path))

    db.create_table(
        "users_imported",
        {
            "name": {"type": "string", "required": True},
            "email": {"type": "string", "required": True, "unique": True},
            "age": {"type": "integer"},
        },
    )
    imported_jsonl = db.import_jsonl("users_imported", str(jsonl_path))
    assert imported_jsonl == 1

    db.create_table(
        "users_from_sqlite",
        {
            "name": {"type": "string", "required": True},
            "email": {"type": "string", "required": True, "unique": True},
            "age": {"type": "integer"},
        },
    )
    imported_sqlite = db.import_sqlite("users_from_sqlite", str(sqlite_path), "users")
    assert imported_sqlite == 1

    rows = db.fetch_all("users_from_sqlite")
    assert len(rows) == 1
    assert rows[0].data["name"] == "Ana"
