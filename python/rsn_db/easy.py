"""Ergonomic Python API: context managers, save/load, MemPalace, session memory."""

from __future__ import annotations

from contextlib import contextmanager
from typing import Any, Iterator, Optional

from ._core import Database, Query, Record
from .ai_memory import SessionMemory
from .mempalace_bridge import MemPalaceBridge, MEMPALACE_INSTALL

__all__ = ["RsnDatabase", "open_db", "Query", "Record", "MEMPALACE_INSTALL"]


class RsnDatabase:
    """Rust ``Database`` with optional MemPalace and session memory."""

    def __init__(
        self,
        storage_path: Optional[str] = None,
        *,
        encryption_key: Optional[str] = None,
        compression: str = "zstd",
        mode: str = "professional",
        palace_path: Optional[str] = None,
        enable_mempalace: bool = False,
        session_memory: bool = True,
    ) -> None:
        self._inner = Database(
            storage_path=storage_path,
            encryption_key=encryption_key,
            compression=compression,
            mode=mode,
        )
        self._palace: Optional[MemPalaceBridge] = None
        self._memory: Optional[SessionMemory] = None
        if storage_path and session_memory:
            self._memory = SessionMemory.for_database(storage_path)
            self._memory.load()
        if enable_mempalace:
            self.enable_mempalace(palace_path)

    @property
    def inner(self) -> Database:
        return self._inner

    @property
    def memory(self) -> Optional[SessionMemory]:
        return self._memory

    def enable_mempalace(self, palace_path: Optional[str] = None) -> MemPalaceBridge:
        self._palace = MemPalaceBridge(palace_path=palace_path)
        return self._palace

    def save(self) -> None:
        self._inner.save()
        if self._memory:
            self._memory.save()

    def load(self) -> None:
        self._inner.load()
        if self._memory:
            self._memory.load()

    def snapshot(self, dest: str) -> None:
        self._inner.snapshot(dest)

    def remember(self, text: str, **kwargs: Any) -> str:
        if self._memory:
            role = kwargs.pop("role", "user")
            self._memory.add(role, text)
        if not self._palace:
            if self._memory:
                return "Stored in session memory."
            raise RuntimeError(f"MemPalace not enabled. {MEMPALACE_INSTALL}")
        return self._palace.remember(text, **kwargs)

    def palace_search(self, query: str, **kwargs: Any) -> str:
        if not self._palace:
            raise RuntimeError(f"MemPalace not enabled. {MEMPALACE_INSTALL}")
        return self._palace.search_text(query, **kwargs)

    def palace_wake_up(self) -> str:
        if not self._palace:
            raise RuntimeError(f"MemPalace not enabled. {MEMPALACE_INSTALL}")
        return self._palace.wake_up()

    def sync_to_mempalace(self, **kwargs: Any) -> int:
        if not self._palace:
            raise RuntimeError(f"MemPalace not enabled. {MEMPALACE_INSTALL}")
        if self._memory:
            return self._memory.sync_to_mempalace(self._palace, **kwargs)
        return self._palace.sync_rsn_graph_ingest(self._inner, **kwargs)

    def __getattr__(self, name: str) -> Any:
        return getattr(self._inner, name)


@contextmanager
def open_db(
    storage_path: Optional[str] = None,
    *,
    encryption_key: Optional[str] = None,
    compression: str = "zstd",
    mode: str = "professional",
    palace_path: Optional[str] = None,
    mempalace: bool = False,
) -> Iterator[RsnDatabase]:
    db = RsnDatabase(
        storage_path,
        encryption_key=encryption_key,
        compression=compression,
        mode=mode,
        palace_path=palace_path,
        enable_mempalace=mempalace,
    )
    try:
        yield db
    finally:
        if storage_path:
            db.save()
