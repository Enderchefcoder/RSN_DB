<div align="center">
  <h1>RSN DB</h1>
  <p><strong>A Rust-powered embedded database packaged for Python via Maturin.</strong></p>
  <p>
    <img alt="rust" src="https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust" />
    <img alt="python" src="https://img.shields.io/badge/Python-3.9%2B-blue?logo=python" />
    <img alt="build" src="https://img.shields.io/badge/build-maturin-success" />
  </p>
</div>

## Why RSN DB?

RSN DB is designed for scripts, tools, automation flows, and prototypes where you want:

- **Fast local data access** from Rust internals.
- **Simple Python ergonomics** with a compact API.
- **Optional JSON persistence** for zero-ops storage.
- **Schema validation**, unique constraints, querying, CSV export, and transaction controls.

---

## Feature Snapshot

<table>
  <tr>
    <td><strong>Typed schemas</strong></td>
    <td>Define string/int/float/bool/json fields and required constraints.</td>
  </tr>
  <tr>
    <td><strong>Record lifecycle</strong></td>
    <td>Insert, update, delete, fetch, and basic SQL-like utility commands.</td>
  </tr>
  <tr>
    <td><strong>Query builder</strong></td>
    <td><code>where_eq</code>, <code>order_by</code>, and <code>take</code> in a chainable API.</td>
  </tr>
  <tr>
    <td><strong>Persistence + transactions</strong></td>
    <td>Save to disk and use begin/rollback/commit for safer multi-step changes.</td>
  </tr>
  <tr>
    <td><strong>Data interoperability</strong></td>
    <td>CSV, JSONL, and SQLite import/export for downstream analytics and migrations.</td>
  </tr>
</table>

---

## Setup

### 1) Prerequisites

- Rust toolchain (`rustup`)
- Python 3.9+
- `maturin`

```bash
python -m pip install maturin
```

### 2) Build and install (editable local dev)

```bash
maturin develop
```

### 3) Run tests

```bash
cargo test
python -m pytest
```

### 4) Build wheel/sdist for PyPI-style installation

```bash
maturin build --release
python -m pip install target/wheels/rsn_db-*.whl
```

---

## Quickstart

```python
from rsn_db import Database, Query

db = Database("./data/state.json")

db.create_table(
    "users",
    {
        "name": {"type": "string", "required": True},
        "email": {"type": "string", "required": True, "unique": True},
        "age": {"type": "integer"},
        "is_active": {"type": "boolean"},
    },
)

a_id = db.insert("users", {
    "name": "Alice",
    "email": "alice@example.com",
    "age": 30,
    "is_active": True,
})

db.update("users", a_id, {"age": 31})

active = db.query(
    Query("users")
    .where_eq("is_active", True)
    .order_by("age", True)
    .take(10)
)

print(active[0].data)
print(db.execute_sql("COUNT users"))
```

---

## Demo GIFs

> These show a terminal-driven flow that mirrors the RSN DB API style.

### Create table + query records

*Caption: Creating schema, inserting records, then querying active users from the terminal workflow.*
![RSN DB quick usage demo](https://raw.githubusercontent.com/charmbracelet/vhs/main/examples/demo.gif)

### Transaction safety workflow

*Caption: A rollback/commit transaction flow to keep writes safe while iterating in a terminal session.*
![RSN DB transaction workflow demo](https://raw.githubusercontent.com/asciinema/agg/main/demo/demo.gif)

### Cross-database conversion workflow

*Caption: Exporting from RSN DB into SQLite/JSONL and importing rows back into another table.*
![RSN DB conversion workflow demo](https://raw.githubusercontent.com/charmbracelet/vhs/main/examples/demo.gif)

---

## API Overview

### `Database(storage_path: str | None = None)`

- `create_table(name, schema)`
- `create_index(table, field)`
- `insert(table, payload) -> int`
- `update(table, record_id, patch)`
- `delete(table, record_id)`
- `fetch_all(table) -> list[Record]`
- `query(query: Query) -> list[Record]`
- `execute_sql(sql: str) -> Any` (`SHOW`, `COUNT <table>`)
- `export_csv(table, destination)`
- `export_jsonl(table, destination)`
- `import_jsonl(table, source) -> int`
- `export_sqlite(table, destination)`
- `import_sqlite(table, source, source_table=None) -> int`
- `begin_transaction()`, `rollback()`, `commit()`
- `save()`

### `Query(table: str)`

- `where_eq(field, value)`
- `order_by(field, descending=False)`
- `take(count)`

### `Record`

- `id: int`
- `data: dict`

---

## Persistence Example

```python
db = Database("./data/prod.json")
# ... mutations
# autosaved after mutating operations
db.save()  # explicit save if needed
```

## Conversion Example (SQLite + JSONL)

```python
db.export_sqlite("users", "./out/users.sqlite")
db.export_jsonl("users", "./out/users.jsonl")

db.create_table("users_migrated", {
    "name": {"type": "string", "required": True},
    "email": {"type": "string", "required": True, "unique": True},
    "age": {"type": "integer"},
})

db.import_sqlite("users_migrated", "./out/users.sqlite", "users")
```

## PyPI Publishing Guide

The project already uses a PyPI-ready `pyproject.toml` with maturin. See `documentation/pypi_publish.md` for a complete release flow and the exact `twine` upload commands.

---

## Project Structure

```text
.
├── Cargo.toml
├── pyproject.toml
├── src/lib.rs
├── python/rsn_db/__init__.py
└── tests/test_rsn_db.py
```

---

## License

MIT
