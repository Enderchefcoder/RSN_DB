#!/usr/bin/env python3
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

lib = ROOT / "src/lib.rs"
t = lib.read_text()
if "snark_pool" not in t:
    t = t.replace("pub mod personality;", "pub mod personality;\npub mod snark_pool;", 1)
lib.write_text(t)

pers = ROOT / "src/personality.rs"
pt = pers.read_text()
if "snark_pool::EXTRA_SNARK" not in pt:
    pt = "use crate::snark_pool::EXTRA_SNARK;\n" + pt
old_pick = (
    "    fn pick(&self, options: &[&str]) -> String {\n"
    "        options\n"
    "            .choose(&mut thread_rng())\n"
    '            .unwrap_or(&"Error generating sarcasm. Even my snark module is disappointed in you.")\n'
    "            .to_string()\n"
    "    }"
)
new_pick = (
    "    fn pick(&self, options: &[&str]) -> String {\n"
    "        let pool: Vec<&str> = options.iter().copied().chain(EXTRA_SNARK.iter().copied()).collect();\n"
    "        pool.choose(&mut thread_rng())\n"
    '            .unwrap_or(&"Error generating sarcasm. Even my snark module is disappointed in you.")\n'
    "            .to_string()\n"
    "    }"
)
pt = pt.replace(old_pick, new_pick)
pers.write_text(pt)

init = ROOT / "python/rsn_db/__init__.py"
it = init.read_text()
if "beginners" not in it:
    it = it.replace("from .ai_memory", "from . import beginners\nfrom .ai_memory")
    it = it.replace('"MemoryTurn",', '"MemoryTurn",\n    "beginners",')
init.write_text(it)
print("patched ok")
