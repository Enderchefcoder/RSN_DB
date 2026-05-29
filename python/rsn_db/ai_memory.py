"""Local AI session memory (JSON sidecar). Pairs with MemPalace for long-term recall."""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterator, Optional

MAX_TURNS = 10_000
MAX_CONTENT_CHARS = 32_000


@dataclass
class MemoryTurn:
    role: str
    content: str
    ts: str = field(default_factory=lambda: datetime.now(timezone.utc).isoformat())

    def to_dict(self) -> dict[str, str]:
        return {"role": self.role, "content": self.content, "ts": self.ts}


class SessionMemory:
    """
    Lightweight conversation memory stored next to your ``.rsndb`` file.

    Example::

        mem = SessionMemory.for_database("app.rsndb")
        mem.add("user", "Remember: deploy on Fridays is banned")
        mem.add("assistant", "Noted.")
        mem.save()
    """

    def __init__(self, path: Optional[str] = None) -> None:
        self.path = Path(path) if path else Path("session_memory.json")
        self.turns: list[MemoryTurn] = []

    @classmethod
    def for_database(cls, storage_path: str) -> SessionMemory:
        base = Path(storage_path)
        sidecar = base.with_suffix(base.suffix + ".memory.json")
        return cls(str(sidecar))

    def add(self, role: str, content: str) -> None:
        role = role.strip().lower()[:64]
        text = content.strip()[:MAX_CONTENT_CHARS]
        if not text:
            raise ValueError("memory content cannot be empty")
        if len(self.turns) >= MAX_TURNS:
            self.turns = self.turns[MAX_TURNS // 10 :]
        self.turns.append(MemoryTurn(role=role, content=text))

    def recall(self, limit: int = 20) -> list[dict[str, str]]:
        return [t.to_dict() for t in self.turns[-limit:]]

    def context_block(self, limit: int = 10) -> str:
        lines = []
        for t in self.turns[-limit:]:
            lines.append(f"[{t.role}] {t.content}")
        return "\n".join(lines)

    def save(self) -> None:
        self.path.parent.mkdir(parents=True, exist_ok=True)
        payload = {"version": 1, "turns": [t.to_dict() for t in self.turns]}
        self.path.write_text(json.dumps(payload, indent=2), encoding="utf-8")

    def load(self) -> None:
        if not self.path.exists():
            return
        data = json.loads(self.path.read_text(encoding="utf-8"))
        self.turns = [MemoryTurn(**row) for row in data.get("turns", [])]

    def sync_to_mempalace(self, bridge: Any, *, wing: str = "rsn_db", room: str = "sessions") -> int:
        """Push each turn into official MemPalace drawers."""
        count = 0
        for t in self.turns:
            bridge.remember(f"[{t.role}] {t.content}", wing=wing, room=room, source_file=str(self.path))
            count += 1
        return count

    def __iter__(self) -> Iterator[MemoryTurn]:
        return iter(self.turns)
