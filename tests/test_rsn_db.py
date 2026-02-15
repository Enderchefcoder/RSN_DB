from rsn_db import RSNDatabase


def test_table_crud_graph_and_history(tmp_path):
    db = RSNDatabase()

    db.create_table("users", {"name": "String", "age": "Int"}, flexible=False)
    db.create_table("posts", {"title": "String", "likes": "Int"})

    alice_id = db.insert("users", {"name": "Alice", "age": 30})
    bob_id = db.insert("users", {"name": "Bob", "age": 25})
    post_id = db.insert("posts", {"title": "Rust + Python", "likes": 10})

    assert len(db.read("users")) == 2
    assert len(db.read("users", ("age", ">", 26))) == 1

    updated = db.update("users", ("name", "=", "Alice"), {"age": 31})
    assert updated == 1
    assert db.read("users", ("age", "=", 31))[0]["name"] == "Alice"

    db.link("users", alice_id, "WROTE", "posts", post_id)
    assert db.walk("users", alice_id, "WROTE") == [("posts", post_id)]
    assert db.unlink("users", alice_id, "WROTE", "posts", post_id) == 1

    db.put("active_user", bob_id)
    assert db.get("active_user") == bob_id

    db.checkpoint("before_cleanup")
    assert db.remove("users", ("name", "=", "Bob")) == 1
    assert len(db.read("users")) == 1
    db.rollback_to("before_cleanup")
    assert len(db.read("users")) == 2

    save_path = tmp_path / "state.json"
    db.save(str(save_path))

    reloaded = RSNDatabase()
    reloaded.load(str(save_path))
    assert len(reloaded.read("users")) == 2
