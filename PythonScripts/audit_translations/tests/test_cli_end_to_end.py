"""
CLI coverage tests for audit_translations.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
from collections import Counter
from pathlib import Path

from .. import cli as audit_cli


def fixture_rules_dir() -> Path:
    return Path(__file__).resolve().parent / "fixtures" / "Rules" / "Languages"


def parse_jsonl(output: str) -> list[dict]:
    return [json.loads(line) for line in output.splitlines() if line.strip()]


def assert_issue_counts(issues: list[dict]) -> None:
    counts = Counter(issue["issue_type"] for issue in issues)
    assert len(issues) == 19
    assert counts["missing_rule"] == 4
    assert counts["extra_rule"] == 3
    assert counts["untranslated_text"] == 6
    assert counts["rule_difference"] == 6


def test_cli_main_jsonl_output_matches_fixture(capsys, monkeypatch) -> None:
    """
    Exercise the CLI entrypoint in-process by patching sys.argv.

    This validates argparse wiring and output formatting without spawning a new process.
    """
    rules_dir = fixture_rules_dir()
    args = [
        "es",
        "--format",
        "jsonl",
        "--rules-dir",
        str(rules_dir),
    ]

    monkeypatch.setattr(sys, "argv", ["audit_translations", *args])
    audit_cli.main()
    in_process_output = capsys.readouterr().out
    assert_issue_counts(parse_jsonl(in_process_output))


def test_cli_module_jsonl_output_matches_fixture() -> None:
    """
    Exercise the CLI via python -m audit_translations in a subprocess.

    This validates module execution, environment wiring, and exit behavior.
    """
    rules_dir = fixture_rules_dir()
    args = [
        "es",
        "--format",
        "jsonl",
        "--rules-dir",
        str(rules_dir),
    ]

    python_scripts_dir = Path(__file__).resolve().parents[2]
    env = os.environ.copy()
    env["PYTHONPATH"] = os.pathsep.join(
        [str(python_scripts_dir), env.get("PYTHONPATH", "")]
    ).strip(os.pathsep)

    result = subprocess.run(
        [sys.executable, "-m", "audit_translations", *args],
        capture_output=True,
        text=True,
        cwd=str(python_scripts_dir),
        env=env,
        check=True,
    )
    assert_issue_counts(parse_jsonl(result.stdout))
