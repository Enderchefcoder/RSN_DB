<div align="center">
  <h1>🦀 RSN DB 🦀</h1>
  <p><strong>A Rust-powered embedded database with a <i>personality</i>.</strong></p>
  <p>
    <a href="https://pypi.org/project/rsn-db/"><img src="https://img.shields.io/pypi/v/rsn-db?color=blue&logo=pypi" alt="PyPI version"></a>
    <img alt="rust" src="https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust" />
    <img alt="python" src="https://img.shields.io/badge/Python-3.9%2B-blue?logo=python" />
    <img alt="license" src="https://img.shields.io/badge/License-MIT-green" />
  </p>
</div>

---

## 🚀 Setup & Installation

Get up and running in seconds. RSN DB is a single package with everything baked in.

<img src="assets/setup.gif" width="100%" alt="RSN DB Setup">

```bash
pip install rsn_db
```

---

## ✨ Features

- **🛡️ Insane Safety**: AES-GCM Encryption, Zstd Compression, and SHA-256 Checksums.
- **🎭 Dynamic Personality**: Choose between **Professional**, **Friendly**, or **Snarky** modes.
- **💻 Interactive CLI**: A powerful REPL with syntax highlighting (simulated) and natural language queries.
- **🧠 GraphRAG**: Built-in knowledge retrieval engine without the LLM overhead.
- **⚡ High Performance**: Powered by Rust, utilizing `bincode` for O(1) serialization and optimized indexes.

---

## 🎮 Interactive Session

Watch RSN DB in action. Here we use the **Snarky** mode to create a table, insert data, and run a query.

<img src="assets/usage.gif" width="100%" alt="RSN DB Usage">

---

## 📖 Quickstart

### Python Library
The library is "all business"—no snark, just performance.

```python
from rsn_db import Database

# Initialize with encryption
db = Database(storage_path="data.rsn", encryption_key="super-secret")

# Create a table
db.create_table("users", {"name": {"type": "string", "required": True}})

# Insert data
db.insert("users", {"name": "Alice", "age": 30})

# Query data
results = db.execute_sql("FIND users WHERE age > 20")
print(results)
```

### CLI
Just run `rsn` to start the interactive shell.

```bash
rsn
```

---

## 🔒 Security & Safety

RSN DB is built with a security-first mindset:
- **Encryption at Rest**: AES-256-GCM for all data.
- **Integrity**: SHA-256 checksums on every block.
- **Path Guard**: Blocks absolute paths and directory traversal.
- **DoS Protection**: Strict limits on batch sizes and recursion depth.

---

## 🕸️ GraphRAG (New in v0.2.x)

Ingest unstructured text and query relationships directly.

```python
db.ingest("RSN DB was created by a team of caffeinated engineers.", source="engineers_doc")
print(db.graph_query("Who created RSN DB?"))
```

---

<div align="center">
  <sub>Built with ❤️ and 🦀 by the RSN DB Contributors</sub>
</div>
