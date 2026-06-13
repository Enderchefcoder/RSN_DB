"""Full SessionMemory coverage."""

from unittest.mock import MagicMock, patch

import pytest

from rsn_db.ai_memory import SessionMemory


def test_sync_to_mempalace():
    mem = SessionMemory()
    mem.add("user", "hello")
    bridge = MagicMock()
    bridge.remember.return_value = "ok"
    assert mem.sync_to_mempalace(bridge) == 1
    bridge.remember.assert_called_once()


def test_iter_and_load_empty(tmp_path):
    path = tmp_path / "m.json"
    mem = SessionMemory(str(path))
    mem.load()
    assert list(mem) == []


def test_trim_on_max_turns():
    mem = SessionMemory()
    mem.turns = []
    from rsn_db.ai_memory import MAX_TURNS, MemoryTurn

    for i in range(MAX_TURNS + 5):
        mem.turns.append(MemoryTurn(role="user", content=f"x{i}"))
    mem.add("user", "new")
    assert len(mem.turns) <= MAX_TURNS
