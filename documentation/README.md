# RSN DB documentation

Welcome to the RSN DB documentation index. Start here if you are integrating the library, hardening deployments, or connecting AI memory.

## Getting started

| Guide | Audience | Summary |
|-------|----------|---------|
| [BEGINNERS.md](BEGINNERS.md) | New Python users | Install, `quick_start`, `open_db`, REPL tips |
| [examples/example_usage.md](examples/example_usage.md) | All users | Python and CLI examples |

## Integrations

| Guide | Summary |
|-------|---------|
| [mempalace.md](mempalace.md) | Official [MemPalace](https://github.com/MemPalace/mempalace) bridge — search, remember, sync |

## Security

| Guide | Summary |
|-------|---------|
| [security.md](security.md) | Encryption, integrity, input validation, operational guidance |
| [threat_model.md](threat_model.md) | STRIDE analysis and mitigations |

## Release & publishing

| Guide | Summary |
|-------|---------|
| [patchnotes.md](patchnotes.md) | Version history and breaking changes |
| [pypi_publish.md](pypi_publish.md) | Maintainer notes for PyPI releases |

## Package surface (Python)

```
rsn_db
├── Database, Query, Record     # Rust engine (PyO3)
├── RsnDatabase, open_db        # Ergonomic wrapper + save/load
├── SessionMemory, MemoryTurn   # JSON session sidecar
├── MemPalaceBridge             # Official mempalace package
└── beginners                   # quick_start, insert_many, records_to_dicts
```

## CLI reference

```bash
rsn --help
rsn --no-prompt -c "SHOW TABLES"
rsn --mode professional --storage ./data.rsndb
```

Personality modes: `professional` · `friendly` · `snarky`

Alive / Snarky REPL: `PULSE`, `MOOD`, `VITALS`, `ACHIEVEMENT`

MemPalace REPL: `MEMPALACE HELP` for subcommands.

## External links

- [GitHub repository](https://github.com/Enderchefcoder/RSN_DB)
- [PyPI package](https://pypi.org/project/rsn-db/)
- [MemPalace official docs](https://mempalaceofficial.com)
