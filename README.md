<div align="center">
  <h1>RSN DB</h1>
  <p><strong>A Rust-powered embedded database with a personality.</strong></p>
  <p>
    <img alt="rust" src="https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust" />
    <img alt="python" src="https://img.shields.io/badge/Python-3.9%2B-blue?logo=python" />
    <img alt="security" src="https://img.shields.io/badge/Security-Insane-red" />
  </p>
</div>

## Features

- **Insane Safety & Security**: AES-GCM Encryption, Zstd Compression, and SHA-256 Checksums.
- **Dynamic Personality**: Choose your experience—Professional, Friendly, or **Snarky** with richer reactions, achievements, and batch summaries.
- **Interactive CLI**: Just type `rsn` and get to work.
- **Smart Data Handling**: Constraints, Aliases, Batch Operations, and Smart Type Coercion.
- **Command UX Features**: Talk to your data with `FIND`, inspect schemas with `DESCRIBE`, and review recent activity via `HISTORY`.

---

## Interactive Session Demo

### Snarky Mode Activation
*Caption: Setting up RSN DB for the first time and enabling full snark mode.*
```text
$ rsn
Select mode:
  [1] Professional (clean, minimal output)
  [2] Friendly     (helpful with personality)
  [3] Snarky       (full commentary enabled)

Choice (default: 1): 3
Remember this choice? (y/n): y

✓ Snarky mode enabled.
  Don't say I didn't warn you.

rsn>
```

### Natural Language Query
*Caption: Using the FIND command to translate complex questions into database queries.*
```text
rsn> FIND users older than Bob who follow someone
⚙ Translating...
  Interpreted as: READ users WHERE age > (SELECT age FROM users WHERE name = "Bob") AND has_outbound_edge("FOLLOWS")
Is that it?
Y for yes, N or blank for no
r>y
╭── Results ────────────────────╮
│  • Alice (30)                 │
│  • Charlie (35)               │
╰───────────────────────────────╯
```

### Batch Operations & Achievement
*Caption: Executing a fast batch of inserts and getting roasted/rewarded by the engine.*
```text
rsn> BATCH
batch> INSERT users (name: "User1", age: 20)
batch> INSERT users (name: "User2", age: 21)
batch> INSERT users (name: "User3", age: 22)
batch> COMMIT

✓ Batch executed: 3 operations.
  (Wow! You're fast! Very hard work. Good job. My grandmother can't do that. Not that I have one, since I'm a robot.)

... (after 50 perfect commands) ...

[SYSTEM]: Achievement unlocked: "Actually Competent"
[SYSTEM]: Updating user classification from "Hopeless" to "Occasionally Capable"
```

---

## Quickstart

### Installation
```bash
pip install rsn_db
```

### CLI Usage
```bash
rsn
```

### Library Usage
```python
from rsn_db import Database

# The library is all business. No snark here.
db = Database(storage_path="my_data.rsn", encryption_key="secret-key")
db.create_table("users", {"name": {"type": "string", "required": True}})
db.insert("users", {"name": "Alice"})
```

---

## Security & Safety

RSN DB takes your data seriously (even if it doesn't take *you* seriously).
- **Encryption**: AES-256-GCM keeps your data private.
- **Integrity**: SHA-256 checksums prevent tampering.
- **Protection**: Built-in path traversal protection plus stricter command validation to avoid parser crashes.

---

## License
MIT

## GraphRAG (New in v0.2.0)
RSN DB now includes a built-in GraphRAG engine for knowledge retrieval without heavy LLMs.
Usage:
```python
db.ingest("Large text document...", source="my_doc")
print(db.graph_query("What are the key entities?"))
```
Or via SQL:
```sql
INGEST "My interesting text..."
GRAPH_QUERY Who is related to what?
```
