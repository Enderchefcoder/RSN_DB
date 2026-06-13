"""Agent-friendly CLI (--command, --json, --version)."""

import subprocess
import sys


def test_cli_version():
    r = subprocess.run(
        [sys.executable, "-m", "rsn_db.cli", "--version"],
        capture_output=True,
        text=True,
    )
    assert r.returncode == 0
    assert "0.4" in r.stdout


def test_cli_one_shot_tables(tmp_path):
    storage = tmp_path / "c.rsndb"
    r = subprocess.run(
        [
            sys.executable,
            "-m",
            "rsn_db.cli",
            "--no-prompt",
            "--mode",
            "professional",
            "--storage",
            str(storage),
            "-c",
            "SHOW TABLES",
        ],
        capture_output=True,
        text=True,
    )
    assert r.returncode == 0
