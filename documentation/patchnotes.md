# Patch Notes

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

---
*Timestamp: 2026-03-13 21:31:05*
*Exploit: Stack Overflow (DoS) and Type Confusion*
*Severity: Critical (Fixed)*

## [v0.2.1] - 2026-02-22
### Added
- **Security Documentation**: Added `documentation/security.md` with threat model, limits, and hardening guidance.

### Fixed
- **DoS Hardening**:
  - Enforced maximum SQL command length.
  - Enforced maximum queued BATCH operation count.
  - Enforced maximum INGEST payload size.
  - Converted JSONL import to streaming I/O with max file-size and max-line guards.
- **Input Safety**:
  - Enforced strict table identifier validation in SQLite export/import SQL paths.
  - Hardened path sanitization using path-component validation against traversal, absolute, and prefixed paths.
- **Stability**:
  - Removed panic risk from GraphRAG regex compilation path.

---
*Timestamp: 2026-02-22 18:00:00*
*Exploit: DoS and SQL/path injection hardening*
*Severity: High (Fixed)*

## [v0.1.0] - 2025-01-15
### Added
- **Encryption**: Integrated AES-GCM 256-bit encryption for database persistence.
- **Compression**: Integrated Zstandard compression for storage efficiency.
- **Insane Safety**: SHA-256 checksum verification on load to prevent data corruption/tampering.
- **Dynamic Personality System**: Choose between Professional, Friendly, and Snarky modes.
- **REPL/CLI**: Interactive terminal interface (`rsn` command).
- **Natural Language Queries**: Basic FIND command for translating NL to database queries.
- **Batch Operations**: Support for BATCH...COMMIT blocks.
- **Command Aliases**: Define your own shortcuts with ALIAS.
- **Backups & Diffs**: Create backups and view changes with BACKUP/DIFF.
- **Smart Type Coercion**: Auto-converting strings to integers/floats/booleans when possible.

### Fixed
- **Security**: Prevented potential path traversal vulnerabilities.
- **Stability**: Fixed memory management in batch operations.

---
*Timestamp: 2025-01-15 16:00:00*
*Exploit: Path Traversal*
*Severity: High (Fixed)*

## [v0.2.0] - 2026-02-22
### Added
- **GraphRAG Engine**: Pure Rust implementation of GraphRAG. High-performance ingestion (<1s/1000 words) and query.
- **Binary Storage**: Switched to .rsndb binary format using bincode for better efficiency and security.
- **Extended Snark**: New personality responses for graph operations.
- **SQL Commands**: Added INGEST and GRAPH_QUERY commands.

### Fixed
- **Deep Security Audit**:
    - Fixed stack overflow vulnerability in ALIAS expansion.
    - Fixed recursion limit in JSON serialization.
    - Optimized unique field validation from O(N^2) to O(N).
    - Hardened path sanitization against absolute paths and advanced traversal.
    - Removed all potential panics in core engine.

---
*Timestamp: 2026-02-22 16:56:45*
*Exploit: Various (DoS, Injection, Traversal)*
*Severity: Critical (Fixed)*
