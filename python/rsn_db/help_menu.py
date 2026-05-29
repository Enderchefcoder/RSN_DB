"""Beginner-friendly, sorted REPL help with optional Snarky remarks."""

from __future__ import annotations

import random
from dataclasses import dataclass


@dataclass(frozen=True)
class HelpEntry:
    command: str
    description: str


# Sorted alphabetically within each section (beginner-first section order).
SECTIONS: tuple[tuple[str, tuple[HelpEntry, ...]], ...] = (
    (
        "Getting started",
        (
            HelpEntry("EXIT", "Save and quit the shell."),
            HelpEntry("HELP", "Show this command reference."),
        ),
    ),
    (
        "Tables & data",
        (
            HelpEntry("COUNT <table>", "Return the number of rows in a table."),
            HelpEntry("SHOW TABLES", "List all tables (alias: TABLES)."),
            HelpEntry("TABLES", "Same as SHOW TABLES."),
        ),
    ),
    (
        "GraphRAG (knowledge)",
        (
            HelpEntry("GRAPH_QUERY <text>", "Search ingested knowledge for related facts."),
            HelpEntry("INGEST <text>", "Add free-form text to the on-disk knowledge graph."),
        ),
    ),
    (
        "Transactions",
        (
            HelpEntry("BATCH", "Start a batch; following writes are held until COMMIT."),
            HelpEntry("COMMIT", "Apply all operations in the current batch."),
            HelpEntry("ROLLBACK", "Discard the current batch without saving changes."),
        ),
    ),
    (
        "Alive system (personality feedback)",
        (
            HelpEntry("ACHIEVEMENT", "Show unlocked achievements (Snarky mode)."),
            HelpEntry("MOOD", "Show the engine's current mood."),
            HelpEntry("PULSE", "Heartbeat / status line — proves the DB is awake."),
            HelpEntry("VITALS", "Show internal vitals (mood, streaks, activity)."),
        ),
    ),
    (
        "MemPalace (optional AI memory)",
        (
            HelpEntry("MEMPALACE HELP", "Official MemPalace commands (needs pip extra)."),
            HelpEntry("MEMPALACE REMEMBER <text>", "Store a memory in your palace."),
            HelpEntry("MEMPALACE SEARCH <query>", "Search palace memories."),
            HelpEntry("MEMPALACE STATUS", "Palace connection / stack status."),
            HelpEntry("MEMPALACE WAKEUP", "Run MemPalace wake-up routine."),
        ),
    ),
)

MEMPALACE_SECTION: tuple[HelpEntry, ...] = tuple(
    sorted(
        (
            HelpEntry("MEMPALACE HELP", "List MemPalace subcommands."),
            HelpEntry("MEMPALACE INIT [dir]", "Initialize a palace in a project directory."),
            HelpEntry("MEMPALACE MINE <path>", "Mine files or conversations into the palace."),
            HelpEntry("MEMPALACE REMEMBER <text>", "Store text in wing=rsn_db."),
            HelpEntry("MEMPALACE SEARCH <query>", "Semantic search across memories."),
            HelpEntry("MEMPALACE STATUS", "Show palace / stack status."),
            HelpEntry("MEMPALACE WAKEUP", "Wake-up summary from MemPalace."),
        ),
        key=lambda e: e.command,
    )
)

HELP_SNARK_REMARKS: tuple[str, ...] = (
    "You're reading help. That's already more effort than most users.",
    "These commands are sorted. Your schema probably isn't.",
    "Type EXIT when done. The data will wait; your patience might not.",
    "HELP won't fix your data model, but it's a start.",
    "Pro tip: SHOW TABLES before INSERT. Revolutionary, I know.",
    "INGEST text, then GRAPH_QUERY it. Like a brain, but with fewer feelings.",
    "BATCH + COMMIT: transactions for people who learned from their mistakes.",
    "ROLLBACK exists because you will need it. Trust me.",
    "PULSE means I'm alive. Your query plan, less so.",
    "MOOD tracks how this session is going. Spoiler: it's fragile.",
    "MEMPALACE HELP — for when your brain needs an external SSD.",
    "Still here? ACHIEVEMENT unlocked: Read The Manual.",
    "Windows users: if `rsn` is taken, use `rsn-db`. You're welcome.",
    "I have 130+ snark lines and I'm not afraid to use them.",
    "Friendly mode exists for people who can't handle the truth.",
)

FRIENDLY_TIPS: tuple[str, ...] = (
    "Tip: start with SHOW TABLES, then INGEST a sentence and try GRAPH_QUERY.",
    "Tip: use BATCH before bulk inserts, then COMMIT once.",
    "Tip: install MemPalace with pip install 'rsn_db[mempalace]' for AI memory.",
    "Tip: on Windows cmd/PowerShell, run rsn-db if rsn conflicts with another tool.",
)

PRO_TIPS: tuple[str, ...] = (
    "Use --storage path.rsndb to pin a database file.",
    "Non-interactive: rsn-db -c \"SHOW TABLES\" --no-prompt",
)


def _format_section(title: str, entries: tuple[HelpEntry, ...]) -> list[str]:
    lines = [title]
    width = max(len(e.command) for e in entries) if entries else 0
    for entry in entries:
        lines.append(f"  {entry.command.ljust(width)}  {entry.description}")
    return lines


def format_help(mode: str = "professional") -> str:
    """Return sorted, described help text for the REPL."""
    mode_key = mode.lower()
    lines: list[str] = ["RSN DB — command reference", ""]

    for title, entries in SECTIONS:
        lines.extend(_format_section(title, entries))
        lines.append("")

    if mode_key == "snarky":
        lines.append(random.choice(HELP_SNARK_REMARKS))
    elif mode_key == "friendly":
        lines.append(random.choice(FRIENDLY_TIPS))
    else:
        lines.append(random.choice(PRO_TIPS))

    return "\n".join(lines).rstrip()


def format_mempalace_help(mode: str = "professional") -> str:
    """Sorted MemPalace subcommand help."""
    lines = [
        "MemPalace (official — mempalaceofficial.com)",
        "Install: pip install 'rsn_db[mempalace]'",
        "",
    ]
    lines.extend(_format_section("Commands", MEMPALACE_SECTION))
    if mode.lower() == "snarky":
        lines.append("")
        lines.append(random.choice(HELP_SNARK_REMARKS))
    return "\n".join(lines)
