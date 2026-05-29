"""Beginner-friendly helpers on top of RSN DB."""

from __future__ import annotations

from typing import Any, Iterator, Optional

from .easy import RsnDatabase, open_db
from ._core import Database, Query, Record

__all__ = [
    "quick_start",
    "insert_many",
    "records_to_dicts",
    "tutorial_commands",
    "RsnDatabase",
    "open_db",
    "Database",
    "Query",
    "Record",
]


def quick_start(
    path: str = "my_first.rsndb",
    *,
    mempalace: bool = False,
    mode: str = "friendly",
) -> RsnDatabase:
    """Open (or create) a database with sensible defaults for first-time users."""
    return RsnDatabase(path, mode=mode, enable_mempalace=mempalace)


def insert_many(db: Database, table: str, rows: list[dict[str, Any]]) -> list[int]:
    """Insert a list of dict rows; returns list of new row ids."""
    ids: list[int] = []
    for row in rows:
        ids.append(db.insert(table, row))
    return ids


def records_to_dicts(records: list[Record]) -> list[dict[str, Any]]:
    """Convert ``Record`` objects to plain dicts with ``id`` and fields."""
    out: list[dict[str, Any]] = []
    for rec in records:
        row = dict(rec.data) if isinstance(rec.data, dict) else {"data": rec.data}
        row["id"] = rec.id
        out.append(row)
    return out


def tutorial_commands() -> list[str]:
    """Sample REPL commands for learning."""
    return [
        "SHOW TABLES",
        "CREATE TABLE demo (name TEXT)  # use Python API for schemas",
        "INGEST RSN DB is a Rust-powered database with personality.",
        "GRAPH_QUERY personality",
        "MEMPALACE HELP",
        "PULSE",
        "MOOD",
        "ACHIEVEMENT",
    ]
