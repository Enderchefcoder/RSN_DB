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
