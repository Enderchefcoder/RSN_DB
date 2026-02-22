# Patch Notes

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
