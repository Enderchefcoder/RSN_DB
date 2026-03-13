# Threat Model: RSN DB

## System Overview
- **Architecture**: Embedded Database (Rust with Python Bindings)
- **Data Classification**: User-defined (can include PII, credentials, application state)
- **Trust Boundaries**: Python Application -> RSN DB (Native Module) -> File System

## STRIDE Analysis
| Threat           | Component           | Risk | Mitigation                                      |
|------------------|---------------------|------|-------------------------------------------------|
| **S**poofing     | Database File       | Med  | SHA-256 integrity checks on load.               |
| **T**ampering    | Database File       | High | AES-GCM encryption + SHA-256 checksums.         |
| **R**epudiation  | Audit Logs          | Low  | N/A (Embedded database, logging is app-side).   |
| **I**nfo Disclosure | Storage / Memory | High | AES-256-GCM at rest; memory-safe Rust core.    |
| **D**enial of Service | SQL / Ingest   | High | Resource limits (max command, recursion depth). |
| **E**levation of Priv | Path Traversal | High | Strict path sanitization and relative-only paths.|

## Attack Surface
- **External**: SQL-like command parser, JSONL/SQLite import interfaces.
- **Internal**: GraphRAG ingestion engine, Alias expansion logic.
- **Storage**: Binary .rsndb files (encrypted/compressed).

## Identified Risks & Remediation Plan
1. **Risk**: Stack overflow in `json_to_py` (DoS).
   - **Remediation**: Implement `MAX_RECURSION_DEPTH` check in recursive conversion.
2. **Risk**: Uncontrolled resource consumption in GraphRAG (DoS).
   - **Remediation**: Optimize graph rebuilding; consider background or lazy updates.
3. **Risk**: Incomplete batch control (Reliability).
   - **Remediation**: Implement `ROLLBACK` to allow safe cancellation of batch operations.
4. **Risk**: Type confusion during SQLite import.
   - **Remediation**: Use schema metadata to guide JSON parsing in `import_sqlite`.
