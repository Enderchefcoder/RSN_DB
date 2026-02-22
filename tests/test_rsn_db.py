from rsn_db import Database, Query
import pytest
import os

def test_end_to_end(tmp_path):
    # Use relative path for db
    db_path = "state.rsndb"
    db = Database(db_path)
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
    if os.path.exists(db_path): os.remove(db_path)

def test_unknown_field_rejected(tmp_path):
    db = Database("state2.rsndb")
    db.create_table(
        "users",
        {
            "name": {"type": "string", "required": True},
            "email": {"type": "string", "required": True, "unique": True},
        },
    )

    with pytest.raises(ValueError, match="not part of the schema"):
        db.insert("users", {"name": "Eve", "email": "eve@example.com", "role": "admin"})
    if os.path.exists("state2.rsndb"): os.remove("state2.rsndb")

def test_jsonl_and_sqlite_roundtrip(tmp_path):
    db = Database("state3.rsndb")
    db.create_table(
        "users",
        {
            "name": {"type": "string", "required": True},
            "email": {"type": "string", "required": True, "unique": True},
            "age": {"type": "integer"},
        },
    )
    db.insert("users", {"name": "Ana", "email": "ana@example.com", "age": 30})

    jsonl_path = "users.jsonl"
    sqlite_path = "users.sqlite"

    db.export_jsonl("users", jsonl_path)
    db.export_sqlite("users", sqlite_path)

    db.create_table(
        "users_imported",
        {
            "name": {"type": "string", "required": True},
            "email": {"type": "string", "required": True, "unique": True},
            "age": {"type": "integer"},
        },
    )
    imported_jsonl = db.import_jsonl("users_imported", jsonl_path)
    assert imported_jsonl == 1

    db.create_table(
        "users_from_sqlite",
        {
            "name": {"type": "string", "required": True},
            "email": {"type": "string", "required": True, "unique": True},
            "age": {"type": "integer"},
        },
    )
    imported_sqlite = db.import_sqlite("users_from_sqlite", sqlite_path, "users")
    assert imported_sqlite == 1

    rows = db.fetch_all("users_from_sqlite")
    assert len(rows) == 1
    assert rows[0].data["name"] == "Ana"

    for f in ["state3.rsndb", jsonl_path, sqlite_path]:
        if os.path.exists(f): os.remove(f)

def test_command_safety_and_discovery_features(tmp_path):
    db = Database("state4.rsndb", mode="friendly")
    db.create_table(
        "users",
        {
            "name": {"type": "string", "required": True},
            "email": {"type": "string", "required": True, "unique": True},
        },
    )

    with pytest.raises(ValueError, match="COUNT requires"):
        db.execute_sql("COUNT")

    with pytest.raises(ValueError, match="ALIAS format"):
        db.execute_sql("ALIAS shortcut")

    fields = db.execute_sql("DESCRIBE users")
    assert fields == ["email", "name"]

    db.execute_sql("TABLES")
    db.execute_sql("COUNT users")
    history = db.execute_sql("HISTORY")
    assert "TABLES" in history
    if os.path.exists("state4.rsndb"): os.remove("state4.rsndb")

def test_path_traversal_rejected(tmp_path):
    db = Database("state5.rsndb")
    db.create_table(
        "users",
        {
            "name": {"type": "string", "required": True},
            "email": {"type": "string", "required": True, "unique": True},
        },
    )
    db.insert("users", {"name": "Ana", "email": "ana@example.com"})

    with pytest.raises(ValueError, match="Potential path traversal"):
        db.export_jsonl("users", "../unsafe.jsonl")

    with pytest.raises(ValueError, match="Potential path traversal"):
        db.import_sqlite("users", "../unsafe.sqlite", "users")
    if os.path.exists("state5.rsndb"): os.remove("state5.rsndb")
