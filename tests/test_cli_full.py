"""Full CLI coverage (non-interactive paths only)."""

from unittest.mock import patch

import pytest


def test_load_save_prefs(tmp_path, monkeypatch):
    from rsn_db import cli

    prefs_file = tmp_path / "prefs.json"
    monkeypatch.setattr(cli, "PREFS_FILE", prefs_file)
    cli.save_prefs({"mode": "snarky"})
    assert cli.load_prefs()["mode"] == "snarky"


def test_handle_mempalace_subcommands():
    from rsn_db.cli import _handle_mempalace

    with patch("rsn_db.mempalace_bridge.MemPalaceBridge") as cls:
        inst = cls.return_value
        inst.remember.return_value = "ok"
        inst.wake_up.return_value = "wake"
        inst.status.return_value = "st"
        inst.init_palace.return_value = "init"
        inst.mine_path.return_value = "mine"
        for cmd in (
            "MEMPALACE REMEMBER hi",
            "MEMPALACE WAKEUP",
            "MEMPALACE STATUS",
            "MEMPALACE INIT",
            "MEMPALACE MINE .",
            "MEMPALACE HELP",
        ):
            assert _handle_mempalace(cmd, {}) is True


def test_print_result_json(capsys):
    from rsn_db.cli import _print_result
    import json

    _print_result(["a"], as_json=True)
    assert json.loads(capsys.readouterr().out.strip()) == ["a"]


def test_print_result_list_plain(capsys):
    from rsn_db.cli import _print_result

    _print_result(["x"], as_json=False)
    assert "x" in capsys.readouterr().out


def test_main_error_exit(tmp_path):
    from rsn_db import cli

    with patch.object(cli, "Database") as db_cls:
        db_cls.return_value.execute_sql.side_effect = ValueError("x")
        assert (
            cli.main(
                [
                    "--no-prompt",
                    "-c",
                    "X",
                    "--storage",
                    str(tmp_path / "e.rsndb"),
                ]
            )
            == 1
        )


def test_main_json_output(tmp_path, capsys):
    from rsn_db import cli

    with patch.object(cli, "Database") as db_cls:
        db_cls.return_value.execute_sql.return_value = ["t"]
        assert (
            cli.main(
                [
                    "--no-prompt",
                    "--json",
                    "-c",
                    "TABLES",
                    "--storage",
                    str(tmp_path / "j.rsndb"),
                ]
            )
            == 0
        )
