"""Python API for RSN DB — Rust engine with optional official MemPalace integration."""

from ._core import Database, Query, Record
from .easy import RsnDatabase, open_db
from .mempalace_bridge import MemPalaceBridge, MEMPALACE_INSTALL, OFFICIAL_DOCS

__version__ = "0.3.0"

__all__ = [
    "Database",
    "Query",
    "Record",
    "RsnDatabase",
    "open_db",
    "MemPalaceBridge",
    "MEMPALACE_INSTALL",
    "OFFICIAL_DOCS",
    "__version__",
]
