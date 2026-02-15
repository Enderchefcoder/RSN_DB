from rsn_db import Database, Query


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
