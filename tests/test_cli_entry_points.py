"""Console script entry points and Windows-friendly ``rsn-db`` alias."""

from __future__ import annotations

import re
import shutil
import subprocess
import sys
from pathlib import Path

import pytest


def test_pyproject_declares_both_console_scripts():
    root = Path(__file__).resolve().parents[1]
    text = (root / "pyproject.toml").read_text()
    scripts = re.findall(r"^\s*([\w-]+)\s*=\s*\"rsn_db\.cli:main\"", text, re.M)
    assert "rsn" in scripts
    assert "rsn-db" in scripts


def test_cli_prog_name_rsn_db():
    from rsn_db.cli import cli_prog_name

    assert cli_prog_name(["rsn-db"]) == "rsn-db"
    assert cli_prog_name(["rsn-db.exe"]) == "rsn-db"
    assert cli_prog_name(["rsn"]) == "rsn"
    assert cli_prog_name(["rsn.exe"]) == "rsn"


def test_build_parser_uses_rsn_db_prog():
    from rsn_db.cli import build_parser

    parser = build_parser(prog="rsn-db")
    assert parser.prog == "rsn-db"
    help_text = parser.format_help()
    assert "rsn-db" in help_text
    assert "Windows" in help_text


def test_rsn_db_module_invocation(tmp_path):
    storage = tmp_path / "entry.rsndb"
    r = subprocess.run(
        [
            sys.executable,
            "-m",
            "rsn_db.cli",
            "--no-prompt",
            "--storage",
            str(storage),
            "-c",
            "SHOW TABLES",
        ],
        capture_output=True,
        text=True,
    )
    assert r.returncode == 0


@pytest.mark.skipif(shutil.which("rsn-db") is None, reason="rsn-db not on PATH")
def test_rsn_db_console_script_version():
    r = subprocess.run(["rsn-db", "--version"], capture_output=True, text=True)
    assert r.returncode == 0
    assert "0.4" in r.stdout
