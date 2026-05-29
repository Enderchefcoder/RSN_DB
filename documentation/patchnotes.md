# Patch Notes

## [v0.4.4] - 2026-05-29
### Added
- **`rsn-db` console script** — Windows-friendly alias for `rsn` (same CLI; avoids cmd/PowerShell name conflicts).
- CLI help shows the invoked command name and documents the Windows alternative.

### Changed
- Documentation updated for dual entry points.

---

## [v0.4.3] - 2026-05-29
### Changed
- **Documentation overhaul**: Professional README for PyPI/GitHub, documentation index, refreshed examples.
- Package metadata and project description updated for clarity.

---

## [v0.4.2] - 2026-05-29
### Added
- **100% Python wrapper test coverage** (`fail_under=99`, 73+ tests).
- CLI fix when MemPalace is not installed (no crash on import error).

### Changed
- `rsn_db.beginners` exported from package root.

---

## [v0.4.0] - 2026-05-29
### Added
- **Alive system**: `PULSE`, `MOOD`, `VITALS`; mood tracking and ambient snark.
- **Session memory**: `SessionMemory` sidecar + optional MemPalace sync.
- **Beginners**: `quick_start`, `insert_many` — `documentation/BEGINNERS.md`.
- **CLI**: `rsn -c`, `--mode`, `--json`, `--version` (agent-friendly).
- **Tests**: 40 pytest cases; expanded Rust module tests in `alive` / `graph_rag`.

---

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
