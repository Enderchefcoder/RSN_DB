import json
import sys
from pathlib import Path

from rsn_db._core import Database

PREFS_FILE = Path.home() / ".rsn_preferences"


def load_prefs():
    if PREFS_FILE.exists():
        try:
            return json.loads(PREFS_FILE.read_text())
        except json.JSONDecodeError:
            return {}
    return {}


def save_prefs(prefs):
    PREFS_FILE.write_text(json.dumps(prefs))


def _handle_mempalace(line: str, prefs: dict) -> bool:
    upper = line.strip()
    if not upper.upper().startswith("MEMPALACE"):
        return False
    try:
        from rsn_db.mempalace_bridge import MemPalaceBridge, MEMPALACE_INSTALL
    except ImportError:
        print(f"MemPalace integration unavailable. {MEMPALACE_INSTALL}")
        return True
    bridge = MemPalaceBridge(palace_path=prefs.get("mempalace_path"))
    parts = upper.split(maxsplit=2)
    sub = parts[1].upper() if len(parts) > 1 else "HELP"
    rest = parts[2] if len(parts) > 2 else ""
    try:
        if sub in ("HELP", "?"):
            print(
                "MemPalace (official — mempalaceofficial.com):\n"
                "  MEMPALACE SEARCH <query>\n"
                "  MEMPALACE REMEMBER <text>\n"
                "  MEMPALACE WAKEUP\n"
                "  MEMPALACE STATUS\n"
                "  MEMPALACE INIT [dir]\n"
                "  MEMPALACE MINE <path>\n"
            )
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
            print(f"Unknown or incomplete MEMPALACE command: {sub}")
    except Exception as exc:
        print(f"MemPalace error: {exc}")
    return True


def main():
    prefs = load_prefs()
    mode = prefs.get("mode")
    if not mode:
        print("Select mode:")
        print("  [1] Professional (clean, minimal output)")
        print("  [2] Friendly     (helpful with personality)")
        print("  [3] Snarky       (full commentary enabled)")
        choice = input("\nChoice (default: 1): ").strip()
        mode = {"2": "friendly", "3": "snarky"}.get(choice, "professional")
        if input("Remember this choice? (y/n): ").strip().lower() == "y":
            prefs["mode"] = mode
            save_prefs(prefs)

    db = Database(storage_path=prefs.get("storage_path"), mode=mode)
    print("✓ Snarky mode enabled.\n  Don't say I didn't warn you.\n" if mode == "snarky" else (
        "✓ Friendly mode enabled. Let's build something cool!\n" if mode == "friendly"
        else "RSN DB v0.3.0 (Professional Mode)\n"
    ))
    print("Type HELP or MEMPALACE HELP. EXIT to quit.")

    while True:
        try:
            line = input("rsn> ").strip()
            if line.upper() == "EXIT":
                db.save()
                print("\n  See you next time.\n")
                break
            if _handle_mempalace(line, prefs):
                continue
            if line.upper() == "HELP":
                print("TABLES | COUNT | INGEST | GRAPH_QUERY | BATCH | MEMPALACE HELP")
                continue
            result = db.execute_sql(line)
            if result:
                if isinstance(result, list):
                    for item in result:
                        print(f" • {item}")
                else:
                    print(result)
        except EOFError:
            break
        except Exception as e:
            print(e)


if __name__ == "__main__":
    main()
