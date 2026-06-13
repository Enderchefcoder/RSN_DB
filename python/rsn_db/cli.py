"""RSN DB CLI — agent-friendly (non-interactive flags, clear help)."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from rsn_db import __version__
from rsn_db._core import Database
from rsn_db.help_menu import format_help, format_mempalace_help

PREFS_FILE = Path.home() / ".rsn_preferences"

CLI_COMMANDS = ("rsn", "rsn-db")


def cli_prog_name(argv: list[str] | None = None) -> str:
    """Return ``rsn-db`` when invoked via that entry point (Windows-friendly)."""
    invoked = Path((argv or sys.argv)[0]).name.lower()
    if invoked.startswith("rsn-db"):
        return "rsn-db"
    return "rsn"


def load_prefs() -> dict:
    if PREFS_FILE.exists():
        try:
            return json.loads(PREFS_FILE.read_text())
        except json.JSONDecodeError:
            return {}
    return {}


def save_prefs(prefs: dict) -> None:
    PREFS_FILE.write_text(json.dumps(prefs))


def _handle_mempalace(line: str, prefs: dict) -> bool:
    upper = line.strip()
    if not upper.upper().startswith("MEMPALACE"):
        return False
    try:
        from rsn_db.mempalace_bridge import MemPalaceBridge, MEMPALACE_INSTALL
    except ImportError:
        print(
            "MemPalace unavailable. pip install 'rsn_db[mempalace]'",
            file=sys.stderr,
        )
        return True
    bridge = MemPalaceBridge(palace_path=prefs.get("mempalace_path"))
    parts = upper.split(maxsplit=2)
    sub = parts[1].upper() if len(parts) > 1 else "HELP"
    rest = parts[2] if len(parts) > 2 else ""
    try:
        if sub in ("HELP", "?"):
            print(format_mempalace_help(prefs.get("mode", "professional")))
        elif sub == "SEARCH" and rest:
            print(bridge.search_text(rest))
        elif sub == "REMEMBER" and rest:
            print(bridge.remember(rest))
        elif sub in ("WAKEUP", "WAKE-UP"):
            print(bridge.wake_up())
        elif sub == "STATUS":
            print(bridge.status())
        elif sub == "INIT":
            print(bridge.init_palace(rest.strip() or None))
        elif sub == "MINE" and rest:
            print(bridge.mine_path(rest.strip()))
        else:
            print(f"Unknown MEMPALACE command: {sub}", file=sys.stderr)
    except Exception as exc:
        print(f"MemPalace error: {exc}", file=sys.stderr)
    return True


def _print_result(result, *, as_json: bool) -> None:
    if not result:
        return
    if as_json:
        print(json.dumps(result, default=str))
        return
    if isinstance(result, list):
        for item in result:
            print(f" • {item}")
    else:
        print(result)


def run_repl(
    db: Database,
    prefs: dict,
    *,
    json_out: bool,
    mode: str = "professional",
    prompt: str | None = None,
) -> None:
    prefs = {**prefs, "mode": mode}
    prog = prompt or cli_prog_name()
    print(f"Type HELP for commands. MEMPALACE HELP, PULSE, MOOD, VITALS. EXIT to quit.")
    if prog == "rsn-db":
        print("  (Using rsn-db — Windows-friendly alias for rsn.)")
    while True:
        try:
            line = input(f"{prog}> ").strip()
        except EOFError:
            break
        if line.upper() == "EXIT":
            db.save()
            print("Goodbye.")
            break
        if _handle_mempalace(line, prefs):
            continue
        if line.upper() in ("HELP", "?"):
            print(format_help(mode))
            continue
        try:
            _print_result(db.execute_sql(line), as_json=json_out)
        except Exception as exc:
            print(exc, file=sys.stderr)


def build_parser(prog: str | None = None) -> argparse.ArgumentParser:
    name = prog or cli_prog_name()
    parser = argparse.ArgumentParser(
        prog=name,
        description="RSN DB interactive shell and one-shot SQL runner.",
        epilog=(
            "Examples:\n"
            f"  {{name}} -c 'SHOW TABLES'\n"
            f"  {{name}} --mode snarky\n"
            f"  {{name}} --storage ./app.rsndb -c 'PULSE'\n\n"
            "On Windows, if ``rsn`` conflicts with another program, use ``rsn-db`` "
            "(same command; installed as a second console script)."
        ).format(name=name),
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("--version", action="version", version=f"rsn_db {__version__}")
    parser.add_argument(
        "-c",
        "--command",
        help="Run a single SQL/REPL command and exit (non-interactive).",
    )
    parser.add_argument(
        "--mode",
        choices=["professional", "friendly", "snarky"],
        help="Personality mode (overrides saved preference).",
    )
    parser.add_argument("--storage", help="Path to .rsndb storage file.")
    parser.add_argument("--json", action="store_true", help="Emit command results as JSON.")
    parser.add_argument(
        "--no-prompt",
        action="store_true",
        help="Skip interactive mode selection on first run.",
    )
    return parser


def main(argv: list[str] | None = None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    args = build_parser().parse_args(argv)
    prefs = load_prefs()
    mode = args.mode or prefs.get("mode")
    if not mode and not args.no_prompt and not args.command:
        print("Select mode: [1] Professional  [2] Friendly  [3] Snarky")
        choice = input("Choice (default 1): ").strip()
        mode = {"2": "friendly", "3": "snarky"}.get(choice, "professional")
        if input("Remember? (y/n): ").strip().lower() == "y":
            prefs["mode"] = mode
            save_prefs(prefs)
    mode = mode or "professional"
    storage = args.storage or prefs.get("storage_path")
    db = Database(storage_path=storage, mode=mode)

    prefs["mode"] = mode
    if args.command:
        cmd_upper = args.command.strip().upper()
        if cmd_upper in ("HELP", "?"):
            print(format_help(mode))
            return 0
        if _handle_mempalace(args.command, prefs):
            return 0
        try:
            _print_result(db.execute_sql(args.command), as_json=args.json)
            db.save()
            return 0
        except Exception as exc:
            print(exc, file=sys.stderr)
            return 1

    if mode == "snarky":
        print(db.execute_sql("PULSE") or "Snarky mode on.")
    run_repl(db, prefs, json_out=args.json, mode=mode)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
