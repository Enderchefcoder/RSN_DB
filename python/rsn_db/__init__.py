"""Python API for RSN DB — Rust engine with optional official MemPalace integration."""

from ._core import Database, Query, Record
from .ai_memory import MemoryTurn, SessionMemory
from . import beginners
from .easy import RsnDatabase, open_db
from .mempalace_bridge import MemPalaceBridge, MEMPALACE_INSTALL, OFFICIAL_DOCS

__version__ = "0.4.6"

__all__ = [
    "Database",
    "Query",
    "Record",
    "RsnDatabase",
    "open_db",
    "MemPalaceBridge",
    "SessionMemory",
    "MemoryTurn",
    "MEMPALACE_INSTALL",
    "OFFICIAL_DOCS",
    "beginners",
    "__version__",
]
