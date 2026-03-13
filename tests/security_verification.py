import unittest
import os
from rsn_db import Database

class TestSecurity(unittest.TestCase):
    def setUp(self):
        self.db_path = "test_security.rsndb"
        if os.path.exists(self.db_path):
            os.remove(self.db_path)
        self.db = Database(storage_path=self.db_path)

    def tearDown(self):
        if os.path.exists(self.db_path):
            os.remove(self.db_path)

    def test_recursion_limit(self):
        # Create a deeply nested structure
        depth = 100
        nested = {"a": "b"}
        for _ in range(depth):
            nested = {"child": nested}

        self.db.create_table("test", {"data": {"type": "json"}})

        # This should fail if we try to convert it back to python after being saved/loaded
        # but first let us see if we can insert it.
        # Actually py_to_json also has a limit.
        with self.assertRaises(ValueError) as cm:
            self.db.insert("test", {"data": nested})
        self.assertIn("Max recursion depth exceeded", str(cm.exception))

    def test_rollback(self):
        self.db.create_table("users", {"name": {"type": "string"}})
        self.db.execute_sql("BATCH")
        self.db.execute_sql("INSERT users {\"name\": \"Alice\"}")
        self.db.execute_sql("INSERT users {\"name\": \"Bob\"}")

        # Rollback
        res = self.db.execute_sql("ROLLBACK")
        self.assertIn("rolled back", res.lower())

        # Verify no data inserted
        count = self.db.execute_sql("COUNT users")
        self.assertEqual(count, 0)

    def test_sqlite_import_hardening(self):
        import sqlite3
        sqlite_db = "test_import.db"
        if os.path.exists(sqlite_db):
            os.remove(sqlite_db)

        conn = sqlite3.connect(sqlite_db)
        conn.execute("CREATE TABLE people (name TEXT, data TEXT)")
        # String that LOOKS like JSON but should be treated as string if schema says so
        conn.execute("INSERT INTO people VALUES (?, ?)", ("Alice", "{\"not\": \"really\"}"))
        conn.commit()
        conn.close()

        # Case 1: Schema says data is STRING
        self.db.create_table("people_str", {"name": {"type": "string"}, "data": {"type": "string"}})
        self.db.import_sqlite("people_str", sqlite_db, "people")
        res = self.db.fetch_all("people_str")
        self.assertEqual(len(res), 1)
        self.assertIsInstance(res[0].data["data"], str)
        self.assertEqual(res[0].data["data"], "{\"not\": \"really\"}")

        # Case 2: Schema says data is JSON
        self.db.create_table("people_json", {"name": {"type": "string"}, "data": {"type": "json"}})
        self.db.import_sqlite("people_json", sqlite_db, "people")
        res = self.db.fetch_all("people_json")
        self.assertEqual(len(res), 1)
        self.assertIsInstance(res[0].data["data"], dict)
        self.assertEqual(res[0].data["data"]["not"], "really")

        if os.path.exists(sqlite_db):
            os.remove(sqlite_db)

if __name__ == "__main__":
    unittest.main()
