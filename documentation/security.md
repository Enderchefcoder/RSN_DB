# RSN DB Security Guide

## Threat model
RSN DB protects data at rest in local database files and enforces strict input validation for command processing and import/export APIs. It is not a networked service and does not provide transport security.

## Storage protections
- AES-256-GCM encryption is used when `encryption_key` is configured.
- SHA-256 checksum validation is performed before decode to detect tampering/corruption.
- Compression is applied before encryption.

## Input and parser hardening
RSN DB enforces explicit limits to reduce denial-of-service risk:
- Max SQL command length: 4096 bytes.
- Max queued batch operations: 512.
- Max ingest payload: 2 MiB.
- Max JSONL import file size: 10 MiB.
- Max JSONL import line count: 100,000.
- Max alias/JSON recursion depth: 64.

## Path safety model
- Import/export paths must be relative.
- Paths with parent traversal (`..`) are rejected.
- Import/export absolute, root, and platform prefix paths are rejected.
- Database storage paths reject traversal and invalid platform prefixes.
- Import/export paths must include a file name.

## SQLite import/export safety
- Table and source table names are validated as strict identifiers (`[A-Za-z0-9_]+`) before SQL statement construction.
- SQLite values are converted into JSON-safe values during import.

## Operational guidance
- Use high-entropy encryption keys.
- Store database files in a dedicated application directory with least-privilege permissions.
- Keep backups and test restore flows after upgrading versions.
