# Beginner's guide to RSN DB

## Install

```bash
pip install rsn_db
# Optional AI memory (official MemPalace):
pip install rsn_db[mempalace]
```

## Fastest start

```python
from rsn_db.beginners import quick_start, insert_many, records_to_dicts
from rsn_db import Query

db = quick_start("my_app.rsndb", mode="friendly")
db.create_table("tasks", {"title": {"type": "string", "required": True}})
insert_many(db, "tasks", [{"title": "Learn RSN DB"}, {"title": "Ship v0.4"}])
print(records_to_dicts(db.query(Query("tasks"))))
db.save()
```

## Context manager

```python
from rsn_db import open_db

with open_db("shop.rsndb", mempalace=True) as db:
    db.remember("Boss wants receipts in SQLite")
    print(db.palace_search("receipts"))
```

## Session memory (no extra install)

```python
from rsn_db import SessionMemory

mem = SessionMemory.for_database("app.rsndb")
mem.add("user", "Deploy only on Tuesdays")
mem.save()
```

## REPL tips

Use `rsn` or `rsn-db` (same CLI; on Windows prefer `rsn-db` if `rsn` is taken):

```bash
rsn-db --mode snarky -c "PULSE"
rsn-db --storage ./data.rsndb -c "SHOW TABLES"
rsn-db --help
```

Snarky mode includes **100+ extra remarks**, mood tracking (`MOOD`), and ambient lines (`PULSE`, `VITALS`).

See also: [mempalace.md](mempalace.md), [security.md](security.md).
