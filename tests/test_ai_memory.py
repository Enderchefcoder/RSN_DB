"""Session memory sidecar."""

import json
from pathlib import Path

import pytest

from rsn_db.ai_memory import SessionMemory


def test_session_memory_roundtrip(tmp_path):
    path = tmp_path / "x.rsndb.memory.json"
    mem = SessionMemory(str(path))
    mem.add("user", "hello")
    mem.add("assistant", "hi")
    mem.save()
    mem2 = SessionMemory(str(path))
    mem2.load()
    assert len(mem2.turns) == 2
    assert "hello" in mem2.context_block()


def test_for_database_sidecar_name(tmp_path):
    p = tmp_path / "app.rsndb"
    mem = SessionMemory.for_database(str(p))
    assert mem.path.name == "app.rsndb.memory.json"


def test_empty_content_rejected():
    mem = SessionMemory()
    with pytest.raises(ValueError):
        mem.add("user", "   ")


def test_recall_limit():
    mem = SessionMemory()
    for i in range(5):
        mem.add("user", f"m{i}")
    assert len(mem.recall(2)) == 2
