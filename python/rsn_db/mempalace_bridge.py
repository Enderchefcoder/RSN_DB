"""
Bridge to the official MemPalace package (https://github.com/MemPalace/mempalace).

Install with: pip install rsn_db[mempalace]
"""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any, Optional

MEMPALACE_INSTALL = "pip install 'rsn_db[mempalace]'  # or: pip install mempalace>=3.3.5"
OFFICIAL_DOCS = "https://mempalaceofficial.com/reference/python-api.html"


def _require_mempalace() -> None:
    try:
        import mempalace  # noqa: F401
    except ImportError as exc:
        raise ImportError(
            "MemPalace is not installed. "
            f"{MEMPALACE_INSTALL}\n"
            f"Docs: {OFFICIAL_DOCS}"
        ) from exc


class MemPalaceBridge:
    """Wrapper around the official MemPalace Python API and CLI."""

    DEFAULT_WING = "rsn_db"
    DEFAULT_ROOM = "memory"
    DEFAULT_AGENT = "rsn_db"

    def __init__(self, palace_path: Optional[str] = None) -> None:
        _require_mempalace()
        from mempalace.config import MempalaceConfig

        cfg = MempalaceConfig()
        self.palace_path = os.path.abspath(
            os.path.expanduser(palace_path or cfg.palace_path)
        )

    @property
    def available(self) -> bool:
        try:
            _require_mempalace()
            return True
        except ImportError:
            return False

    def _env(self) -> dict[str, str]:
        env = os.environ.copy()
        env["MEMPALACE_PALACE_PATH"] = self.palace_path
        return env

    def init_palace(self, project_dir: Optional[str] = None, *, yes: bool = True) -> str:
        _require_mempalace()
        target = project_dir or str(Path(self.palace_path).parent)
        cmd = [
            sys.executable,
            "-m",
            "mempalace",
            "--palace",
            self.palace_path,
            "init",
            target,
        ]
        if yes:
            cmd.append("--yes")
        result = subprocess.run(cmd, capture_output=True, text=True, env=self._env(), check=False)
        if result.returncode != 0:
            raise RuntimeError(
                f"mempalace init failed: {result.stderr.strip() or result.stdout.strip()}"
            )
        return result.stdout.strip() or f"MemPalace initialized at {self.palace_path}"

    def remember(
        self,
        content: str,
        *,
        wing: Optional[str] = None,
        room: Optional[str] = None,
        source_file: str = "rsn_db://remember",
    ) -> str:
        _require_mempalace()
        from mempalace.miner import add_drawer
        from mempalace.palace import get_collection

        wing_name = wing or self.DEFAULT_WING
        room_name = room or self.DEFAULT_ROOM
        collection = get_collection(self.palace_path, create=True)
        add_drawer(
            collection,
            wing_name,
            room_name,
            content,
            source_file,
            0,
            self.DEFAULT_AGENT,
        )
        return f"Stored in MemPalace wing={wing_name} room={room_name}"

    def search(
        self,
        query: str,
        *,
        wing: Optional[str] = None,
        room: Optional[str] = None,
        n_results: int = 5,
    ) -> dict[str, Any]:
        _require_mempalace()
        from mempalace.searcher import search_memories

        return search_memories(
            query,
            self.palace_path,
            wing=wing,
            room=room,
            n_results=n_results,
        )

    def search_text(self, query: str, **kwargs: Any) -> str:
        payload = self.search(query, **kwargs)
        lines = [f"Query: {payload.get('query', query)}"]
        results = payload.get("results") or []
        if not results:
            return "\n".join(lines + ["No MemPalace matches."])
        for idx, hit in enumerate(results, 1):
            if isinstance(hit, dict):
                doc = hit.get("document") or hit.get("content") or hit
                meta = hit.get("metadata") or {}
                lines.append(f"\n[{idx}] {meta.get('wing', '?')}/{meta.get('room', '?')}")
                lines.append(str(doc)[:2000])
            else:
                lines.append(f"\n[{idx}] {hit}")
        return "\n".join(lines)

    def wake_up(self) -> str:
        _require_mempalace()
        from mempalace.layers import MemoryStack

        return MemoryStack(palace_path=self.palace_path).wake_up()

    def mine_path(self, path: str, *, mode: str = "files") -> str:
        _require_mempalace()
        cmd = [sys.executable, "-m", "mempalace", "--palace", self.palace_path, "mine", path]
        if mode == "convos":
            cmd.extend(["--mode", "convos"])
        result = subprocess.run(cmd, capture_output=True, text=True, env=self._env(), check=False)
        if result.returncode != 0:
            raise RuntimeError(f"mempalace mine failed: {result.stderr.strip() or result.stdout.strip()}")
        return result.stdout.strip() or f"Mined {path}"

    def sync_rsn_graph_ingest(self, db: Any, *, wing: Optional[str] = None) -> int:
        _require_mempalace()
        tmp = tempfile.mkdtemp(prefix="rsn_mempalace_")
        try:
            manifest = Path(tmp) / "rsn_graph_export.md"
            chunks: list[str] = []
            try:
                graph_result = db.graph_query(" ")
                if graph_result and "No relevant" not in graph_result:
                    chunks.append(graph_result)
            except Exception:
                pass
            tables = db.execute_sql("TABLES")
            if isinstance(tables, list):
                for table in tables:
                    for row in db.fetch_all(str(table)):
                        chunks.append(f"Table {table} id={row.id}: {row.data}")
            if not chunks:
                return 0
            manifest.write_text("\n\n---\n\n".join(chunks), encoding="utf-8")
            self.mine_path(str(manifest.parent))
            return len(chunks)
        finally:
            import shutil

            shutil.rmtree(tmp, ignore_errors=True)

    def status(self) -> str:
        _require_mempalace()
        from mempalace.layers import MemoryStack

        return MemoryStack(palace_path=self.palace_path).status()
