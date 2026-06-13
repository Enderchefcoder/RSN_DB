<div align="center">

# RSN DB

**A Rust-powered embedded database for Python — fast, secure, and optionally opinionated.**

[![PyPI version](https://img.shields.io/pypi/v/rsn-db?color=blue&logo=pypi)](https://pypi.org/project/rsn-db/)
[![Python](https://img.shields.io/badge/Python-3.9%2B-blue?logo=python)](https://pypi.org/project/rsn-db/)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

[Installation](#installation) · [Quick start](#quick-start) · [Documentation](#documentation) · [Security](#security) · [Changelog](documentation/patchnotes.md)

</div>

---

RSN DB is a single-file embedded database with a native Rust engine and a Python API. Use it like a lightweight local datastore, a graph-aware knowledge store (GraphRAG), or an agent-friendly shell with optional [MemPalace](https://github.com/MemPalace/mempalace) AI memory integration.

**Highlights**

- **Performance** — Rust core, zstd compression, optimized indexes
- **Security** — AES-256-GCM encryption at rest, SHA-256 integrity checks, path guards, DoS limits
- **Persistence** — Explicit `save()` / `load()` / `snapshot()` with JSON engine snapshots
- **GraphRAG** — Ingest text and query relationships without an external LLM
- **Personality modes** — Professional, Friendly, or Snarky CLI feedback (130+ snark lines, mood/vitals in Snarky mode)
- **AI memory** — Session memory sidecar plus optional official MemPalace bridge
- **Agent-friendly CLI** — Non-interactive `-c`, `--json`, `--no-prompt` for automation

<img src="assets/setup.gif" width="100%" alt="RSN DB installation">

---

## Installation

```bash
pip install rsn_db
```

**Optional extras**

```bash
# Official MemPalace integration (local-first AI memory)
pip install 'rsn_db[mempalace]'

# Development / tests
pip install 'rsn_db[dev]'
```

Requires **Python 3.9+**.
### Windows (cmd / PowerShell)

Prebuilt wheels are published for **Windows** (`win_amd64`). Install with:

```powershell
pip install -U rsn_db
rsn-db --version
```

If `pip` appears **stuck at "Building wheel"**, it is compiling Rust from source because no wheel matched your platform/Python. Either:

1. **Upgrade** to the latest version (needs a published Windows wheel): `pip install -U rsn_db`
2. **Use the alias** after install: `rsn-db` (not `rsn`) if another program owns that name
3. **Or** install [Rust](https://rustup.rs/) + [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and wait several minutes for the source build to finish

 Prebuilt wheels are published for common Linux targets; other platforms build from source via maturin (Rust toolchain required).

---

## Quick start

### Beginners (recommended)

```python
from rsn_db.beginners import quick_start, insert_many, records_to_dicts
from rsn_db import Query

db = quick_start("my_app.rsndb", mode="friendly")
db.create_table("tasks", {"title": {"type": "string", "required": True}})
insert_many(db, "tasks", [{"title": "Learn RSN DB"}, {"title": "Ship the release"}])
print(records_to_dicts(db.query(Query("tasks"))))
db.save()
```

### Context manager (auto-save on exit)

```python
from rsn_db import open_db

with open_db("shop.rsndb") as db:
    db.create_table("products", {"name": {"type": "string", "required": True}})
    db.insert("products", {"name": "Widget", "price": 9.99})
```

### Core API (full control)

```python
from rsn_db import Database, Query

db = Database(
    storage_path="data.rsndb",
    encryption_key="change-me-in-production",
    compression="zstd",
    mode="professional",
)

db.create_table("users", {"name": {"type": "string", "required": True}})
db.insert("users", {"name": "Alice", "age": 30})

rows = db.query(Query("users").where_eq("name", "Alice"))
db.save()
db.snapshot("backup.rsndb")
```

See [documentation/BEGINNERS.md](documentation/BEGINNERS.md) for a guided walkthrough.

---

## CLI

Two console commands are installed — same program, pick either:

| Command | When to use |
|---------|-------------|
| `rsn` | Default on macOS/Linux |
| `rsn-db` | **Recommended on Windows** if `rsn` conflicts with another tool in cmd/PowerShell |

Interactive shell:

```bash
rsn
# Windows alternative:
rsn-db
```

**Automation / agents**

```bash
rsn --no-prompt -c "SHOW TABLES"
rsn-db --mode snarky -c "PULSE"
rsn-db --storage ./app.rsndb --json -c "COUNT users"
rsn-db --help
```

**REPL help** — type `HELP` in the shell for a sorted, described command list (Snarky mode adds random remarks).

**REPL commands** (non-exhaustive)

| Category | Examples |
|----------|----------|
| Tables | `SHOW TABLES`, `DESCRIBE users`, `COUNT users` |
| GraphRAG | `INGEST …`, `GRAPH_QUERY …` |
| Alive (Snarky) | `PULSE`, `MOOD`, `VITALS`, `ACHIEVEMENT` |
| MemPalace | `MEMPALACE HELP`, `MEMPALACE SEARCH …`, `MEMPALACE REMEMBER …` |
| Transactions | `BATCH`, `COMMIT`, `ROLLBACK` |

<img src="assets/usage.gif" width="100%" alt="RSN DB interactive session">

---

## MemPalace integration

RSN DB wraps the **official** [MemPalace/mempalace](https://github.com/MemPalace/mempalace) package (wings → rooms → drawers, ChromaDB, local-first). Install the extra, then:

```python
from rsn_db import open_db

with open_db("app.rsndb", mempalace=True) as db:
    db.remember("User prefers Tuesday deploys")
    print(db.palace_search("deploy schedule"))
    db.sync_to_mempalace()
```

Use only official sources: [GitHub](https://github.com/MemPalace/mempalace), [PyPI](https://pypi.org/project/mempalace/), [mempalaceofficial.com](https://mempalaceofficial.com).

Details: [documentation/mempalace.md](documentation/mempalace.md)

---

## Session memory

Lightweight JSON sidecar for conversation turns — no extra install:

```python
from rsn_db import SessionMemory

mem = SessionMemory.for_database("app.rsndb")
mem.add("user", "Remember: staging uses port 8081")
mem.add("assistant", "Noted.")
mem.save()
```

With MemPalace enabled, `RsnDatabase.sync_to_mempalace()` pushes turns into your palace.

---

## GraphRAG

Ingest unstructured text and query it locally:

```python
db.ingest("RSN DB was built with Rust and exposed to Python via PyO3.", source="docs")
print(db.graph_query("What is RSN DB built with?"))
```

---

## Personality modes

| Mode | Behavior |
|------|----------|
| **Professional** | Minimal, neutral output |
| **Friendly** | Helpful tone with light personality |
| **Snarky** | Full commentary, 130+ remark pool, mood tracking, ambient `PULSE` / `VITALS` |

Set via Python (`mode="snarky"`), CLI (`--mode snarky`), or saved preference on first run.

---

## Security

RSN DB is designed with a security-first posture:

| Control | Description |
|---------|-------------|
| Encryption at rest | AES-256-GCM (optional `encryption_key`) |
| Integrity | SHA-256 checksums on persisted blocks |
| Path guard | Blocks absolute paths and directory traversal |
| DoS limits | Caps on batch size, recursion depth, command length |
| Safe imports | SQLite/JSON import respects declared schema types |

Full write-up: [documentation/security.md](documentation/security.md) · [documentation/threat_model.md](documentation/threat_model.md)

---

## Documentation

| Document | Description |
|----------|-------------|
| [BEGINNERS.md](documentation/BEGINNERS.md) | First-time user guide |
| [mempalace.md](documentation/mempalace.md) | Official MemPalace bridge |
| [security.md](documentation/security.md) | Security features and guidance |
| [threat_model.md](documentation/threat_model.md) | STRIDE threat model |
| [patchnotes.md](documentation/patchnotes.md) | Version history |
| [examples/](documentation/examples/) | Usage examples |

---

## Development

```bash
git clone https://github.com/Enderchefcoder/RSN_DB.git
cd RSN_DB
pip install maturin pytest pytest-cov 'mempalace>=3.3.5,<4'
maturin develop --release
pytest tests/ --cov=rsn_db --cov-config=pyproject.toml
cargo clippy -- -W clippy::pedantic
```

---

## License

MIT — see [LICENSE](LICENSE).

---

<div align="center">
  <sub>Built with Rust and Python · <a href="https://github.com/Enderchefcoder/RSN_DB">GitHub</a> · <a href="https://pypi.org/project/rsn-db/">PyPI</a></sub>
</div>
