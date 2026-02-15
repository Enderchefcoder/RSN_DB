# RSN_DB

<div align="center">
  <h3>Rust speed + Python ergonomics for document, graph, and key-value workloads.</h3>
  <p>
    <a href="#quickstart">Quickstart</a> ·
    <a href="#usage-patterns">Usage Patterns</a> ·
    <a href="#feature-map">Feature Map</a> ·
    <a href="#development">Development</a>
  </p>
</div>

---

## Why this project exists

`RSN_DB` is a small hybrid database runtime designed for prototyping and local-first systems:

- **Document-style records** (table + row operations with typed schema hints)
- **Graph edges** (link rows and walk relationships)
- **Key-value cache** (fast shared state in the same engine)
- **Version safety** (checkpoint, undo, redo, rollback)
- **Native speed** from Rust with an idiomatic Python package surface

## Quickstart

### 1) Prerequisites

- Python 3.9+
- Rust toolchain (`rustup`)
- `maturin`

### 2) Local setup

```bash
python -m venv .venv
source .venv/bin/activate
pip install -U pip maturin
maturin develop
```

### 3) Smoke test

```bash
python - <<'PY'
from rsn_db import RSNDatabase

db = RSNDatabase()
db.create_table("users", {"name": "String", "age": "Int"})
row_id = db.insert("users", {"name": "Alice", "age": 30})
print(row_id)
print(db.read("users"))
PY
```

## Usage patterns

### Core workflow

```python
from rsn_db import RSNDatabase

db = RSNDatabase()

db.create_table("users", {"name": "String", "age": "Int"}, flexible=False)
db.create_table("posts", {"title": "String", "likes": "Int"})

alice = db.insert("users", {"name": "Alice", "age": 30})
post = db.insert("posts", {"title": "Rust + Python", "likes": 1})

db.link("users", alice, "WROTE", "posts", post)
print(db.walk("users", alice, "WROTE"))
```

### Filtering and updates

```python
older_users = db.read("users", ("age", ">", 25))
updated = db.update("users", ("name", "=", "Alice"), {"age": 31})
removed = db.remove("users", ("name", "=", "Spammer"))
```

### History controls

```python
db.checkpoint("before_migration")
db.insert("users", {"name": "Temp", "age": 99})

db.undo()      # returns True if something was undone
db.redo()      # returns True if something was redone

db.rollback_to("before_migration")
```

### Persistence

```python
db.save("./state.json")

reloaded = RSNDatabase()
reloaded.load("./state.json")
print(reloaded.tables())
```

## GIF demos

<p>
  <img src="https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExMzY3MGE5dTY2eTFkNWJ3aW5wbzI4Y2xvMjlqZW5ja3V4azNkbjdvNiZlcD12MV9naWZzX3NlYXJjaCZjdD1n/coxQHKASG60HrHtvkt/giphy.gif" alt="Terminal coding gif" width="380" />
  <img src="https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExb2gwN2k3Y3dremN6c2RuODI0N2s4M3Z4cnBocmE5eDY2cnI4aHFpeCZlcD12MV9naWZzX3NlYXJjaCZjdD1n/l0HlBO7eyXzSZkJri/giphy.gif" alt="Data visualization gif" width="380" />
</p>

> You can pair this package with notebooks, CLI wrappers, or backend services.

## Feature map

| Area | Methods | Notes |
|---|---|---|
| Tables | `create_table`, `tables`, `describe` | Schema-aware with optional flexible fields |
| Rows | `insert`, `read`, `update`, `remove` | Condition tuple style: `(field, operator, value)` |
| Graph | `link`, `unlink`, `walk` | Directed labeled edges with traversal |
| KV | `put`, `get`, `drop_key` | Embedded key-value data |
| History | `checkpoint`, `rollback_to`, `undo`, `redo` | Snapshot-based control |
| Durability | `save`, `load` | JSON state persistence |

## Development

```bash
cargo test
maturin develop
pytest -q
```

## License

MIT
