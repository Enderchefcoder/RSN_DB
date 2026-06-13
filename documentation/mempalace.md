# MemPalace integration (official tool)

RSN DB integrates with **[MemPalace](https://github.com/MemPalace/mempalace)** — the open-source AI memory system (wings → rooms → drawers, ChromaDB, local-first). RSN DB does **not** reimplement the palace; it calls the official Python package and CLI.

## Official sources only

- https://github.com/MemPalace/mempalace  
- https://pypi.org/project/mempalace/  
- https://mempalaceofficial.com  

Do not use impostor domains (e.g. `mempalace.tech`).

## Install

```bash
pip install rsn_db[mempalace]
# or: pip install mempalace>=3.3.5
```

MemPalace pulls ChromaDB and an embedding model (~300 MB). No API key is required for core search.

## Python API

```python
from rsn_db import RsnDatabase, open_db, MemPalaceBridge

# Optional palace path; default reads ~/.mempalace/config.json
with open_db("app.rsndb", mempalace=True) as db:
    db.create_table("notes", {"body": {"type": "string", "required": True}})
    db.insert("notes", {"body": "Ship v0.3 with MemPalace bridge"})
    db.remember("User wants MemPalace, not a custom palace reimplementation.")
    print(db.palace_search("MemPalace integration"))
    print(db.palace_wake_up())
    db.sync_to_mempalace()
```

Standalone bridge:

```python
from rsn_db import MemPalaceBridge

bridge = MemPalaceBridge()  # or palace_path="/path/to/palace"
bridge.init_palace("~/projects/myapp")
bridge.remember("Verbatim fact stored in a drawer.")
bridge.mine_path("~/projects/myapp")
print(bridge.search_text("why did we choose Rust"))
```

## REPL commands

```
MEMPALACE HELP
MEMPALACE SEARCH <query>
MEMPALACE REMEMBER <text>
MEMPALACE WAKEUP
MEMPALACE STATUS
MEMPALACE INIT [project_dir]
MEMPALACE MINE <path>
```

## MCP (optional)

To attach MemPalace directly to Claude Code or other MCP clients:

```bash
mempalace mcp
```

See [mempalaceofficial.com/reference/mcp-tools](https://mempalaceofficial.com/reference/mcp-tools.html).

## How RSN DB and MemPalace fit together

| Layer | Role |
|-------|------|
| **RSN DB (Rust)** | Structured tables, encryption, GraphRAG entity graph, batch SQL REPL |
| **MemPalace (Python)** | Verbatim AI memory, semantic recall, wake-up context, knowledge graph |

Use RSN DB for typed records and exports; use MemPalace for conversation and document recall. `sync_to_mempalace()` pushes GraphRAG/table context into the palace via official `mine`.
