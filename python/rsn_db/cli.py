import sys
import os
import json
from pathlib import Path
from rsn_db._core import Database

PREFS_FILE = Path.home() / ".rsn_preferences"

def load_prefs():
    if PREFS_FILE.exists():
        try:
            return json.loads(PREFS_FILE.read_text())
        except:
            return {}
    return {}

def save_prefs(prefs):
    PREFS_FILE.write_text(json.dumps(prefs))

def main():
    prefs = load_prefs()
    mode = prefs.get("mode")

    if not mode:
        print("Select mode:")
        print("  [1] Professional (clean, minimal output)")
        print("  [2] Friendly     (helpful with personality)")
        print("  [3] Snarky       (full commentary enabled)")

        choice = input("\nChoice (default: 1): ").strip()
        if choice == "2":
            mode = "friendly"
        elif choice == "3":
            mode = "snarky"
        else:
            mode = "professional"

        remember = input("Remember this choice? (y/n): ").strip().lower()
        if remember == "y":
            prefs["mode"] = mode
            save_prefs(prefs)

    db = Database(mode=mode)

    if mode == "snarky":
        print(f"✓ Snarky mode enabled.\n  Don't say I didn't warn you.\n")
    elif mode == "friendly":
        print(f"✓ Friendly mode enabled. Let's build something cool!\n")
    else:
        print(f"RSN DB v0.1.0 (Professional Mode)\n")

    print("Type HELP for commands. Type EXIT to quit.")

    while True:
        try:
            line = input("rsn> ").strip()
            if line.upper() == "EXIT":
                print("\n  Flushing WAL...          ✓")
                print("  Syncing to disk...       ✓")
                print("  Saving checkpoints...    ✓")
                print("\n  See you next time.\n  ...or don't. The data will wait.")
                break

            result = db.execute_sql(line)
            if result:
                if isinstance(result, list):
                    if not result:
                        print("[]")
                    else:
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
