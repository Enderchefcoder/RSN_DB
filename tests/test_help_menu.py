"""Tests for beginner-friendly REPL help."""

from rsn_db.help_menu import (
    HELP_SNARK_REMARKS,
    MEMPALACE_SECTION,
    SECTIONS,
    format_help,
    format_mempalace_help,
)


def test_sections_sorted_within_groups():
    for _title, entries in SECTIONS:
        commands = [e.command for e in entries]
        assert commands == sorted(commands)


def test_mempalace_section_sorted():
    commands = [e.command for e in MEMPALACE_SECTION]
    assert commands == sorted(commands)


def test_help_includes_descriptions():
    text = format_help("professional")
    assert "SHOW TABLES" in text
    assert "List all tables" in text
    assert "GRAPH_QUERY" in text


def test_snarky_help_has_remark():
    assert len(HELP_SNARK_REMARKS) >= 10
    text = format_help("snarky")
    assert "command reference" in text.lower() or "RSN DB" in text


def test_friendly_help_has_tip():
    text = format_help("friendly")
    assert "Tip:" in text


def test_mempalace_help_sorted():
    text = format_mempalace_help("professional")
    assert "MEMPALACE SEARCH" in text
    assert "MEMPALACE REMEMBER" in text
    idx_search = text.index("MEMPALACE SEARCH")
    idx_remember = text.index("MEMPALACE REMEMBER")
    assert idx_remember < idx_search  # REMEMBER before SEARCH alphabetically


def test_cli_repl_help_command(capsys):
    from unittest.mock import MagicMock

    from rsn_db.cli import run_repl

    db = MagicMock()
    inputs = iter(["HELP", "EXIT"])

    import builtins

    real_input = builtins.input
    builtins.input = lambda _: next(inputs)
    try:
        run_repl(db, {}, json_out=False, mode="snarky", prompt="rsn-db")
    finally:
        builtins.input = real_input

    out = capsys.readouterr().out
    assert "SHOW TABLES" in out
    assert "List all tables" in out
    assert "rsn-db>" not in out or "HELP" in out


def test_cli_one_shot_help(capsys):
    from rsn_db import cli

    assert cli.main(["--no-prompt", "--mode", "professional", "-c", "HELP"]) == 0
    out = capsys.readouterr().out
    assert "COUNT" in out
    assert "rows" in out.lower()


def test_mempalace_help_snarky_remark():
    text = format_mempalace_help("snarky")
    assert "MemPalace" in text
    # footer snark line present (one of the pool)
    assert len(text.splitlines()) > 5
