# RSN DB — usage examples

Practical examples for **v0.4.x**. For a guided intro, see [BEGINNERS.md](../BEGINNERS.md).

---

## 1. Create, insert, query

```python
from rsn_db import Database, Query

db = Database(storage_path="demo.rsndb")
db.create_table("users", {
    "name": {"type": "string", "required": True},
    "age": {"type": "integer", "required": False},
})

uid = db.insert("users", {"name": "Alice", "age": 30})
rows = db.query(Query("users").where_eq("name", "Alice"))
print(rows[0].id, rows[0].data)
db.save()
```

---

## 2. Encryption and compression

```python
from rsn_db import Database

db = Database(
    storage_path="secure.rsndb",
    encryption_key="use-a-long-random-secret",
    compression="zstd",
)
db.create_table("secrets", {"payload": {"type": "string", "required": True}})
db.insert("secrets", {"payload": "classified"})
db.save()
```

---

## 3. Save, load, snapshot

```python
from rsn_db import RsnDatabase

db = RsnDatabase("app.rsndb")
db.create_table("logs", {"msg": {"type": "string", "required": True}})
db.insert("logs", {"msg": "started"})
db.save()
db.snapshot("app-backup.rsndb")

db.load()  # reload from disk
```

---

## 4. GraphRAG

```python
from rsn_db import Database

db = Database("knowledge.rsndb")
db.ingest(
    "Alice maintains the RSN DB Rust core. Bob writes Python bindings.",
    source="team_notes",
)
print(db.graph_query("Who maintains the Rust core?"))
```

---

## 5. Session memory

```python
from rsn_db import SessionMemory

mem = SessionMemory.for_database("agent.rsndb")
mem.add("user", "Prefer UTC timestamps in logs")
mem.add("assistant", "Will use UTC.")
mem.save()

for turn in mem:
    print(turn.role, turn.content)
```

---

## 6. MemPalace (optional)

Requires `pip install rsn_db[mempalace]`.

```python
from rsn_db import open_db

with open_db("agent.rsndb", mempalace=True) as db:
    db.remember("Project codename: Nightingale")
    print(db.palace_search("codename"))
    print(db.palace_wake_up())
```

See [mempalace.md](../mempalace.md).

---

## 7. CLI — one-shot commands

Use `rsn` or `rsn-db` (identical; prefer `rsn-db` on Windows when `rsn` conflicts):

```bash
rsn-db --no-prompt -c "SHOW TABLES"
rsn-db --storage ./demo.rsndb -c "INGEST RSN DB supports GraphRAG."
rsn-db --mode snarky -c "PULSE"
rsn-db --json -c "COUNT users"
```

---

## 8. CLI — MemPalace

```bash
rsn-db -c "MEMPALACE HELP"
rsn-db -c "MEMPALACE REMEMBER Deploy only on Tuesdays"
rsn-db -c "MEMPALACE SEARCH deploy"
rsn-db -c "MEMPALACE STATUS"
```

---

## 9. Personality modes (Python)

```python
from rsn_db import Database

pro = Database("x.rsndb", mode="professional")
friendly = Database("x.rsndb", mode="friendly")
snarky = Database("x.rsndb", mode="snarky")

print(snarky.execute_sql("PULSE"))
print(snarky.execute_sql("MOOD"))
```

Snarky mode includes an expanded remark pool and alive-system feedback (`VITALS`, `ACHIEVEMENT`).

---

## 10. Batch transactions

```python
from rsn_db import Database

db = Database("batch.rsndb")
db.create_table("items", {"n": {"type": "string", "required": True}})
db.execute_sql("BATCH")
db.insert("items", {"n": "a"})
db.insert("items", {"n": "b"})
db.execute_sql("COMMIT")  # or ROLLBACK to abort
db.save()
```

---

## Related documentation

- [Security](../security.md)
- [Threat model](../threat_model.md)
- [Patch notes](../patchnotes.md)
