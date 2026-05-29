# Patch Notes

## [v0.3.0] - 2026-05-29
### Added
- **Official MemPalace integration** (`pip install rsn_db[mempalace]`): bridge to [MemPalace/mempalace](https://github.com/MemPalace/mempalace) — search, remember, wake-up, mine, init. See [documentation/mempalace.md](mempalace.md).
- **Python ergonomics**: `RsnDatabase`, `open_db()` context manager, explicit `save()` / `load()` / `snapshot()`.
- **REPL**: `MEMPALACE HELP|SEARCH|REMEMBER|WAKEUP|STATUS|INIT|MINE` commands.
- **Snark expansion**: 100+ extra Snarky remarks merged into personality pools.
- **CI**: Clippy (pedantic/nursery), Rust unit tests, pytest with coverage gate.

### Security
- MemPalace docs warn to use only official GitHub, PyPI, and mempalaceofficial.com sources.

---

## [v0.2.2] - 2026-03-13
### Added
- **Detailed Threat Model**: Added `documentation/threat_model.md` with STRIDE analysis.
- **Rollback Capability**: Added `ROLLBACK` command to safely abort batch sessions.

### Fixed
- **DoS Hardening**:
  - Implemented strict recursion depth limits for all Python/JSON object conversions.
- **SQLite Import Safety**:
  - Hardened SQLite import to only parse JSON for fields explicitly defined as `Json` in the schema, preventing type confusion.
- **Performance & Resource Efficiency**:
  - Optimized GraphRAG TF-IDF index rebuilding.
  - Reduced overhead by only triggering community detection when new entities are discovered.
