╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║     ██████╗ ███████╗███╗   ██╗    ██████╗ ██████╗           ║
║     ██╔══██╗██╔════╝████╗  ██║    ██╔══██╗██╔══██╗          ║
║     ██████╔╝███████╗██╔██╗ ██║    ██║  ██║██████╔╝          ║
║     ██╔══██╗╚════██║██║╚██╗██║    ██║  ██║██╔══██╗          ║
║     ██║  ██║███████║██║ ╚████║    ██████╔╝██████╔╝          ║
║     ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝    ╚═════╝ ╚═════╝          ║
║                                                              ║
║     v0.1.0-alpha   |   engine: ember   |   mode: persistent ║
║     storage: ./rsn_data/              |   cache: 256MB      ║
║                                                              ║
║     Type HELP for commands. Type EXIT to quit.               ║
║     "Handle your data, or I will." — RSN_DB                 ║
╚══════════════════════════════════════════════════════════════╝

rsn> HELP

╭──────────────────────── COMMANDS ─────────────────────────╮
│                                                           │
│  STRUCTURE                                                │
│    CREATE table <name> (<field> <type>, ...)              │
│    CREATE row <table> (<field>: <value>, ...)             │
│    DELETE table <name>            ⚠ destructive           │
│    REMOVE <table> WHERE <cond>   ⚠ destructive           │
│    EMPTY <table> WHERE <cond>    ⚠ destructive           │
│                                                           │
│  DATA                                                     │
│    INSERT <table> (<field>: <value>, ...)                 │
│    READ <table>                                           │
│    READ <table> WHERE <cond>                              │
│    READ <table>.<field>                                   │
│    UPDATE <table> WHERE <cond> SET (<field>: <value>)     │
│                                                           │
│  GRAPH                                                    │
│    LINK <table>(id) -[<LABEL>]-> <table>(id)             │
│    UNLINK <table>(id) -[<LABEL>]-> <table>(id)           │
│    WALK <table>(id) -[<LABEL>]-> target                  │
│                                                           │
│  KV                                                       │
│    PUT <key> <value>                                      │
│    GET <key>                                              │
│    DROP <key>                    ⚠ destructive            │
│                                                           │
│  HISTORY                                                  │
│    CHECKPOINT <name>                                      │
│    UNDO                                                   │
│    REDO                                                   │
│    ROLLBACK TO <checkpoint>      ⚠ destructive           │
│    HISTORY                                                │
│                                                           │
│  SYSTEM                                                   │
│    TABLES                                                 │
│    DESCRIBE <table>                                       │
│    COUNT <table>                                          │
│    HELP                                                   │
│    EXIT                                                   │
│                                                           │
│  Notes:                                                   │
│    • Commands are NOT case-sensitive. go wild.            │
│    • Destructive actions require confirmation.            │
│    • Types: String, Int, Float, Bool, Doc, Array          │
╰───────────────────────────────────────────────────────────╯

rsn> create table users (name String, age Int, email String)

✓ Table 'users' created.
  Schema: name(String) | age(Int) | email(String)
  Mode: flexible (unknown fields allowed)

rsn> create table posts (title String, body String, likes Int)

✓ Table 'posts' created.
  Schema: title(String) | body(String) | likes(Int)

rsn> tables

╭────────────────────────────╮
│  TABLES (2)                │
│                            │
│   • users   0 rows  flex   │
│   • posts   0 rows  flex   │
╰────────────────────────────╯

rsn> INSERT users (name: "Alice", age: 30, email: "alice@example.com")

✓ Row inserted into 'users'.
  id: usr_00001

rsn> insert users (name: "Bob", age: 25, email: "bob@example.com")

✓ Row inserted into 'users'.
  id: usr_00002

rsn> INSERT users (name: "Charlie", age: 35, email: "charlie@example.com")

✓ Row inserted into 'users'.
  id: usr_00003

rsn> Insert Users (name: "Diana", age: 28, email: "diana@example.com")

✓ Row inserted into 'users'.
  id: usr_00004

rsn> read users

╭──────────┬─────────┬─────┬───────────────────────╮
│ id       │ name    │ age │ email                 │
├──────────┼─────────┼─────┼───────────────────────┤
│ usr_00001│ Alice   │  30 │ alice@example.com     │
│ usr_00002│ Bob     │  25 │ bob@example.com       │
│ usr_00003│ Charlie │  35 │ charlie@example.com   │
│ usr_00004│ Diana   │  28 │ diana@example.com     │
╰──────────┴─────────┴─────┴───────────────────────╯
  4 rows returned.

rsn> read users where age > 28

╭──────────┬─────────┬─────┬───────────────────────╮
│ id       │ name    │ age │ email                 │
├──────────┼─────────┼─────┼───────────────────────┤
│ usr_00001│ Alice   │  30 │ alice@example.com     │
│ usr_00003│ Charlie │  35 │ charlie@example.com   │
╰──────────┴─────────┴─────┴───────────────────────╯
  2 rows returned.

rsn> read users.name

╭──────────┬─────────╮
│ id       │ name    │
├──────────┼─────────┤
│ usr_00001│ Alice   │
│ usr_00002│ Bob     │
│ usr_00003│ Charlie │
│ usr_00004│ Diana   │
╰──────────┴─────────╯
  4 rows returned.

rsn> read users where name = "Alice"

╭──────────┬───────┬─────┬───────────────────╮
│ id       │ name  │ age │ email             │
├──────────┼───────┼─────┼───────────────────┤
│ usr_00001│ Alice │  30 │ alice@example.com │
╰──────────┴───────┴─────┴───────────────────╯
  1 row returned.

rsn> checkpoint before_graph_stuff

✓ Checkpoint 'before_graph_stuff' saved.
  State: 2 tables, 4 total rows.
  Timestamp: 2025-01-15 14:32:07

rsn> insert posts (title: "Hello World", body: "My first post!", likes: 0)

✓ Row inserted into 'posts'.
  id: pst_00001

rsn> insert posts (title: "Rust is Fire", body: "Literally building a DB in it", likes: 42)

✓ Row inserted into 'posts'.
  id: pst_00002

rsn> link users("usr_00001") -[WROTE]-> posts("pst_00001")

✓ Edge created.
  Alice -[WROTE]-> "Hello World"

rsn> link users("usr_00001") -[WROTE]-> posts("pst_00002")

✓ Edge created.
  Alice -[WROTE]-> "Rust is Fire"

rsn> link users("usr_00001") -[FOLLOWS]-> users("usr_00002")

✓ Edge created.
  Alice -[FOLLOWS]-> Bob

rsn> link users("usr_00002") -[FOLLOWS]-> users("usr_00003")

✓ Edge created.
  Bob -[FOLLOWS]-> Charlie

rsn> link users("usr_00003") -[FOLLOWS]-> users("usr_00004")

✓ Edge created.
  Charlie -[FOLLOWS]-> Diana

rsn> link users("usr_00004") -[FOLLOWS]-> users("usr_00001")

✓ Edge created.
  Diana -[FOLLOWS]-> Alice

rsn> walk users("usr_00001") -[WROTE]-> target

╭── WALK: Alice -[WROTE]-> ─────────────────────────╮
│                                                    │
│  → pst_00001  "Hello World"      (depth: 1)       │
│  → pst_00002  "Rust is Fire"     (depth: 1)       │
│                                                    │
│  2 nodes reached.                                  │
╰────────────────────────────────────────────────────╯

rsn> walk users("usr_00001") -[FOLLOWS*1..3]-> target

╭── WALK: Alice -[FOLLOWS (1..3 hops)]-> ───────────╮
│                                                    │
│  → usr_00002  Bob       (depth: 1)                 │
│  → usr_00003  Charlie   (depth: 2)                 │
│  → usr_00004  Diana     (depth: 3)                 │
│                                                    │
│  3 nodes reached.                                  │
╰────────────────────────────────────────────────────╯

rsn> put session:abc123 { token: "xyz789", user: "usr_00001", ttl: 3600 }

✓ Key 'session:abc123' stored.

rsn> get session:abc123

╭── KEY: session:abc123 ────────────────────╮
│  {                                        │
│    "token": "xyz789",                     │
│    "user": "usr_00001",                   │
│    "ttl": 3600                            │
│  }                                        │
╰───────────────────────────────────────────╯

rsn> describe users

╭── TABLE: users ───────────────────────────────────╮
│                                                    │
│  Mode:    flexible                                 │
│  Rows:    4                                        │
│  Edges:   6 (outbound from this table's rows)      │
│                                                    │
│  SCHEMA:                                           │
│    name    String                                  │
│    age     Int                                     │
│    email   String                                  │
│                                                    │
│  INDEXES:                                          │
│    • id       (primary, auto)                      │
│    • (none user-defined)                           │
│                                                    │
│  EDGE LABELS (outbound):                           │
│    • WROTE    → posts   (2 edges)                  │
│    • FOLLOWS  → users   (4 edges)                  │
╰────────────────────────────────────────────────────╯

rsn> update users where name = "Bob" set (age: 26)

✓ 1 row updated in 'users'.
  usr_00002: age 25 → 26

rsn> read users where name = "Bob"

╭──────────┬──────┬─────┬─────────────────╮
│ id       │ name │ age │ email           │
├──────────┼──────┼─────┼─────────────────┤
│ usr_00002│ Bob  │  26 │ bob@example.com │
╰──────────┴──────┴─────┴─────────────────╯
  1 row returned.

rsn> undo

✓ Undone: UPDATE users (usr_00002) age → reverted to 25.

rsn> read users where name = "Bob"

╭──────────┬──────┬─────┬─────────────────╮
│ id       │ name │ age │ email           │
├──────────┼──────┼─────┼─────────────────┤
│ usr_00002│ Bob  │  25 │ bob@example.com │
╰──────────┴──────┴─────┴─────────────────╯
  1 row returned.

rsn> redo

✓ Redone: UPDATE users (usr_00002) age 25 → 26.

rsn> history

╭── ACTION HISTORY ─────────────────────────────────────────╮
│ #   │ TIME     │ ACTION                                   │
├─────┼──────────┼──────────────────────────────────────────┤
│  1  │ 14:30:01 │ CREATE TABLE users                      │
│  2  │ 14:30:05 │ CREATE TABLE posts                      │
│  3  │ 14:30:12 │ INSERT users (Alice)                    │
│  4  │ 14:30:18 │ INSERT users (Bob)                      │
│  5  │ 14:30:22 │ INSERT users (Charlie)                  │
│  6  │ 14:30:30 │ INSERT users (Diana)                    │
│  7  │ 14:32:07 │ ★ CHECKPOINT 'before_graph_stuff'       │
│  8  │ 14:32:15 │ INSERT posts (Hello World)              │
│  9  │ 14:32:24 │ INSERT posts (Rust is Fire)             │
│ 10  │ 14:32:40 │ LINK Alice -[WROTE]-> Hello World       │
│ 11  │ 14:32:48 │ LINK Alice -[WROTE]-> Rust is Fire      │
│ 12  │ 14:32:55 │ LINK Alice -[FOLLOWS]-> Bob             │
│ 13  │ 14:33:02 │ LINK Bob -[FOLLOWS]-> Charlie           │
│ 14  │ 14:33:10 │ LINK Charlie -[FOLLOWS]-> Diana         │
│ 15  │ 14:33:18 │ LINK Diana -[FOLLOWS]-> Alice           │
│ 16  │ 14:34:01 │ PUT session:abc123                      │
│ 17  │ 14:35:10 │ UPDATE users (Bob) age → 26             │
│ 18  │ 14:35:20 │ UNDO #17                                │
│ 19  │ 14:35:25 │ REDO #17                                │
╰─────┴──────────┴──────────────────────────────────────────╯

rsn> insert users (name: "Eve", age: 22)

✓ Row inserted into 'users'.
  id: usr_00005
  ⚠ Note: 'email' was not provided (table is flexible, so that's fine).

rsn> checkpoint before_chaos

✓ Checkpoint 'before_chaos' saved.
  State: 2 tables, 6 total rows, 1 KV key.

rsn> empty users where name = "Eve"

⚠ DESTRUCTIVE: This will clear all properties from 1 row in 'users'.
  The row(s) will still exist, but will be empty shells.

  Affected:
    usr_00005 (Eve)

  Confirm? (yes/no): yes

✓ 1 row emptied in 'users'.
  usr_00005: all properties cleared.

rsn> read users where id = "usr_00005"

╭──────────┬──────┬──────┬───────╮
│ id       │ name │ age  │ email │
├──────────┼──────┼──────┼───────┤
│ usr_00005│ null │ null │ null  │
╰──────────┴──────┴──────┴───────╯
  1 row returned. (it's pretty empty in there)

rsn> remove users where name = "Charlie"

⚠ DESTRUCTIVE: This will permanently delete 1 row from 'users'.

  Affected:
    usr_00003 (Charlie, age 35)

  ⚠ This row has 2 edges that will also be removed:
    Bob -[FOLLOWS]-> Charlie
    Charlie -[FOLLOWS]-> Diana

  Confirm? (yes/no): yes

✓ 1 row removed from 'users'.
  ✓ 2 edges cleaned up.

rsn> count users

  users: 4 rows

rsn> remove users where age > 100

  0 rows match that condition. Nothing to remove.
  (...did you expect ghosts in here?)

rsn> DELTE table posts

✗ Unknown command: 'DELTE'

  Did you mean: DELETE ?

rsn> DELETE table posts

⚠ DESTRUCTIVE: This will permanently delete the ENTIRE table 'posts'.

  This table contains:
    • 2 rows
    • 2 inbound edges from other tables
  
  ALL of this will be gone. Forever. No pressure.

  Type the table name to confirm: posts

✓ Table 'posts' deleted.
  ✓ 2 rows destroyed.
  ✓ 2 orphaned edges cleaned up.

rsn> read posts

✗ Table 'posts' does not exist.

  (You literally just deleted it. Remember?)

rsn> tables

╭────────────────────────────╮
│  TABLES (1)                │
│                            │
│   • users   4 rows  flex   │
╰────────────────────────────╯

rsn> rollback to before_chaos

⚠ DESTRUCTIVE: Rolling back to checkpoint 'before_chaos'.

  This will UNDO all actions after 14:35:30:
    • EMPTY users (Eve)
    • REMOVE users (Charlie) + 2 edges
    • DELETE TABLE posts + 2 rows + 2 edges

  Your current state will be lost unless you checkpoint now.

  Confirm? (yes/no): yes

✓ Rolled back to 'before_chaos'.
  Restored: 2 tables, 6 rows, 1 KV key, 6 edges.
  Welcome back. Try not to break things this time.

rsn> read users

╭──────────┬─────────┬─────┬───────────────────────╮
│ id       │ name    │ age │ email                 │
├──────────┼─────────┼─────┼───────────────────────┤
│ usr_00001│ Alice   │  30 │ alice@example.com     │
│ usr_00002│ Bob     │  26 │ bob@example.com       │
│ usr_00003│ Charlie │  35 │ charlie@example.com   │
│ usr_00004│ Diana   │  28 │ diana@example.com     │
│ usr_00005│ Eve     │  22 │ null                  │
╰──────────┴─────────┴─────┴───────────────────────╯
  5 rows returned. (Charlie's back from the dead!)

rsn> read posts

╭──────────┬────────────────┬───────────────────────────────┬───────╮
│ id       │ title          │ body                          │ likes │
├──────────┼────────────────┼───────────────────────────────┼───────┤
│ pst_00001│ Hello World    │ My first post!                │     0 │
│ pst_00002│ Rust is Fire   │ Literally building a DB in it │    42 │
╰──────────┴────────────────┴───────────────────────────────┴───────╯
  2 rows returned.

rsn> YEET users where name = "Eve"

✗ Unknown command: 'YEET'

  Did you mean: REMOVE ?
  (I respect the energy though.)

rsn> create table users (name String)

✗ Table 'users' already exists.

  Use DESCRIBE users to see its schema.
  Or DELETE it first if you want to start over. Your funeral.

rsn> raed users

✗ Unknown command: 'RAED'

  Did you mean: READ ?
  (Spellcheck is free, by the way.)

rsn> insert users (name: "Frank", age: "twenty-nine", email: "frank@example.com")

✗ Type mismatch on field 'age':
  Expected: Int
  Got:      String ("twenty-nine")

  I can store numbers. Words that describe numbers? Not so much.

rsn> insert users (name: "Frank", age: 29, email: "frank@example.com")

✓ Row inserted into 'users'.
  id: usr_00006

rsn> insert users (name: "Frank", age: 31, email: "frank2@example.com")

✓ Row inserted into 'users'.
  id: usr_00007
  ⚠ Note: Another row with name="Frank" already exists (usr_00006).
  Not my problem, just thought you should know.

rsn>

rsn> insert posts ()

✗ Empty insert. You gave me nothing.

  INSERT <table> (field: value, ...)
  You need at least one field. I'm a database, not a void.

rsn> read

✗ Read what? You didn't specify a table.

  Usage: READ <table> [WHERE <condition>]
         READ <table>.<field>

rsn> delete table

✗ Delete which table? You forgot the name.

  Usage: DELETE TABLE <name>
  (I'm not going to guess. I'm not psychic.)

rsn> select * from users

✗ Unknown syntax. This isn't SQL.

  Did you mean: READ users ?
  (Old habits die hard, huh?)

rsn> drop table users

✗ Unknown command: 'DROP TABLE'

  Did you mean: DELETE TABLE users ?
  (This isn't Postgres. We have standards here. Different ones.)

rsn> exit

  Flushing WAL...          ✓
  Syncing to disk...       ✓
  Saving checkpoints...    ✓

╭──────────────────────────────────────╮
│  Session summary:                    │
│    Commands run:      38             │
│    Rows created:       7             │
│    Rows destroyed:     1 (restored)  │
│    Tables created:     2             │
│    Tables destroyed:   1 (restored)  │
│    Edges created:      6             │
│    KV keys stored:     1             │
│    Checkpoints:        2             │
│    Undos/Redos:        2             │
│    Typos:              3  (nice)     │
│                                      │
│    Data persisted to: ./rsn_data/    │
│                                      │
│  See you next time.                  │
│  ...or don't. The data will wait.    │
╰──────────────────────────────────────╯
