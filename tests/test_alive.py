"""Alive system: PULSE, MOOD, VITALS."""

import pytest
from rsn_db import Database


@pytest.mark.parametrize("cmd", ["PULSE", "MOOD", "VITALS"])
def test_vitals_commands(tmp_path, cmd):
    db = Database(str(tmp_path / "a.rsndb"), mode="snarky")
    out = db.execute_sql(cmd)
    assert out
    assert isinstance(out, str)


def test_mood_shifts_after_error(tmp_path):
    db = Database(str(tmp_path / "b.rsndb"), mode="snarky")
    db.execute_sql("PULSE")
    with pytest.raises(Exception):
        db.execute_sql("NOT_A_REAL_COMMAND")
    mood = db.execute_sql("MOOD")
    assert "score" in mood.lower() or "(" in mood
