"""
CLI coverage tests for audit_translations.
"""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

import pytest

from .. import cli as audit_cli
from ..renderer import console
from .conftest import strip_ansi


def fixture_rules_dir() -> Path:
    return Path(__file__).resolve().parent / "fixtures" / "Rules" / "Languages"


def make_definitions_rules_dir(tmp_path: Path) -> Path:
    """Create definitions with one missing, extra, and mismatched entry."""
    rules_dir = tmp_path / "Rules" / "Languages"
    source_dir = rules_dir / "en"
    target_dir = rules_dir / "de"
    source_dir.mkdir(parents=True)
    target_dir.mkdir(parents=True)
    (source_dir / "definitions.yaml").write_text(
        "- Missing: [one]\n- Different: [one]\n",
        encoding="utf-8",
    )
    (target_dir / "definitions.yaml").write_text(
        "- Extra: {key: value}\n- Different: {key: value}\n",
        encoding="utf-8",
    )
    return rules_dir


def run_definitions_cli(tmp_path, capsys, monkeypatch, only: str) -> str:
    """Run the definitions CLI fixture with one existing --only category."""
    rules_dir = make_definitions_rules_dir(tmp_path)
    args = ["de", "--rules-dir", str(rules_dir), "--file", "definitions.yaml", "--only", only]
    monkeypatch.setattr(sys, "argv", ["audit_translations", *args])

    audit_cli.main()
    return strip_ansi(capsys.readouterr().out)


def test_cli_definitions_missing_filter_shows_only_missing_findings(tmp_path, capsys, monkeypatch) -> None:
    """The CLI missing filter renders missing definitions but no extra or mismatch findings."""
    output = run_definitions_cli(tmp_path, capsys, monkeypatch, "missing")

    assert "Files to check: 1" in output
    assert "Missing in Translation" in output
    assert "Definition Type Mismatch" not in output
    assert "Info: Extra Definitions" not in output


def test_cli_definitions_extra_filter_shows_only_extra_findings(tmp_path, capsys, monkeypatch) -> None:
    """The CLI extra filter renders informational target-only definitions and no issues."""
    output = run_definitions_cli(tmp_path, capsys, monkeypatch, "extra")

    assert "Files to check: 1" in output
    assert "Info: Extra Definitions" in output
    assert "Missing in Translation" not in output
    assert "Definition Type Mismatch" not in output


def test_cli_definitions_diffs_filter_shows_only_type_mismatches(tmp_path, capsys, monkeypatch) -> None:
    """The CLI diffs filter renders type mismatches but no coverage findings."""
    output = run_definitions_cli(tmp_path, capsys, monkeypatch, "diffs")

    assert "Files to check: 1" in output
    assert "Definition Type Mismatch" in output
    assert "Missing in Translation" not in output
    assert "Info: Extra Definitions" not in output


def test_cli_definitions_untranslated_filter_shows_no_definition_findings(tmp_path, capsys, monkeypatch) -> None:
    """The CLI untranslated filter produces no definition-specific findings."""
    output = run_definitions_cli(tmp_path, capsys, monkeypatch, "untranslated")

    assert "Files to check: 1" in output
    assert "Missing definitions" in output
    assert "Definition type mismatches" in output
    assert "Extra definitions" in output
    assert "Definition Issues" not in output
    assert "Info: Extra Definitions" not in output


def test_cli_definitions_all_filter_shows_every_definition_finding(tmp_path, capsys, monkeypatch) -> None:
    """The CLI all filter renders missing, extra, and type-mismatch definition findings."""
    output = run_definitions_cli(tmp_path, capsys, monkeypatch, "all")

    assert "Files to check: 1" in output
    assert "Missing in Translation" in output
    assert "Definition Type Mismatch" in output
    assert "Info: Extra Definitions" in output


def test_cli_main_rich_only_filters_issue_groups(capsys, monkeypatch) -> None:
    """
    Ensure --only also filters visible rich subgroup sections.

    We expect missing/extra groups to remain while untranslated and all diff
    subgroup labels are omitted from the rendered output.
    """
    args = ["es", "--rules-dir", str(fixture_rules_dir()), "--only", "missing,extra"]

    old_width = console.width
    console.width = 80
    try:
        monkeypatch.setattr(sys, "argv", ["audit_translations", *args])
        audit_cli.main()
        output = strip_ansi(capsys.readouterr().out)
    finally:
        console.width = old_width

    assert "Missing in Translation" in output
    assert "Extra in Translation" in output
    assert "Untranslated Text" not in output
    assert "Match Pattern Differences" not in output
    assert "Condition Differences" not in output
    assert "Variable Differences" not in output
    assert "Structure Differences" not in output


def test_cli_main_accepts_source_language(capsys, monkeypatch) -> None:
    """
    Ensure --source changes the reference language without changing target semantics.
    """
    args = ["en", "--source", "es", "--rules-dir", str(fixture_rules_dir()), "--file", "overview.yaml"]

    old_width = console.width
    console.width = 80
    try:
        monkeypatch.setattr(sys, "argv", ["audit_translations", *args])
        audit_cli.main()
        output = strip_ansi(capsys.readouterr().out)
    finally:
        console.width = old_width

    assert "Comparing against es reference files" in output
    assert "es:" in output
    assert "en:" in output


def test_cli_main_rich_output_groups_by_rule_and_type(capsys, monkeypatch) -> None:
    """
    Ensure rich CLI output is grouped by rule and subgrouped by issue type.

    This is a behavioral assertion test (not snapshot-based): it checks that
    core grouping markers and subgroup ordering are visible in user-facing CLI
    output for a representative fixture file.
    """
    args = ["es", "--rules-dir", str(fixture_rules_dir()), "--file", "SharedRules/calculus.yaml", "--verbose"]

    old_width = console.width
    console.width = 80
    try:
        monkeypatch.setattr(sys, "argv", ["audit_translations", *args])
        audit_cli.main()
        output = strip_ansi(capsys.readouterr().out)
    finally:
        console.width = old_width

    assert "≠ Rule Issues [13] (grouped by rule and issue type)" in output
    assert "• divergence (divergence)" in output
    assert "Untranslated Text [3]" in output
    assert "Match Pattern Differences [1]" in output
    assert "Condition Differences [1]" in output

    untranslated_idx = output.index("Untranslated Text [3]")
    match_idx = output.index("Match Pattern Differences [1]")
    condition_idx = output.index("Condition Differences [1]")
    assert untranslated_idx < match_idx < condition_idx


def test_cli_main_rich_output_matches_grouped_golden(capsys, monkeypatch) -> None:
    """
    Ensure rich CLI grouped rendering stays stable for a multi-rule fixture.

    The golden file captures overall visual layout so formatting regressions in
    grouped sections are caught even when functional issue counts stay the same.
    """
    args = ["es", "--rules-dir", str(fixture_rules_dir()), "--file", "SharedRules/calculus.yaml", "--verbose"]
    golden_path = Path(__file__).resolve().parent / "golden" / "rich" / "cli_calculus_verbose.golden"

    old_width = console.width
    old_legacy_windows = console.legacy_windows
    console.width = 80
    console.legacy_windows = False  # on Windows rich would otherwise replace the rounded box corners

    try:
        monkeypatch.setattr(sys, "argv", ["audit_translations", *args])
        audit_cli.main()
        output = strip_ansi(capsys.readouterr().out)
    finally:
        console.width = old_width
        console.legacy_windows = old_legacy_windows

    assert output == golden_path.read_text(encoding="utf-8")


def test_cli_main_requires_language_or_list(capsys, monkeypatch) -> None:
    """
    Ensure CLI exits with a clear error when neither language nor --list is set.

    This protects the expected help/error UX for accidental empty invocations.
    """
    monkeypatch.setattr(sys, "argv", ["audit_translations"])

    with pytest.raises(SystemExit) as exc:
        audit_cli.main()
    output = strip_ansi(capsys.readouterr().out)

    assert exc.value.code == 1
    assert "Please specify a language code or use --list" in output


def test_cli_main_rejects_unknown_only_token(capsys, monkeypatch) -> None:
    """
    Ensure unsupported --only tokens are rejected before audit execution.

    This keeps filter behavior explicit and prevents silently ignored typos.
    """
    args = ["es", "--rules-dir", str(fixture_rules_dir()), "--only", "missing,bogus"]
    monkeypatch.setattr(sys, "argv", ["audit_translations", *args])

    with pytest.raises(SystemExit) as exc:
        audit_cli.main()
    output = strip_ansi(capsys.readouterr().out)

    assert exc.value.code == 1
    assert "Unknown issue types: bogus" in output


def test_cli_main_reports_missing_region_directory(capsys, monkeypatch) -> None:
    """
    Ensure region variants fail fast when the requested subdirectory is absent.

    This validates the error path for languages like es-mx when only es exists.
    """
    args = ["es-mx", "--rules-dir", str(fixture_rules_dir())]
    monkeypatch.setattr(sys, "argv", ["audit_translations", *args])

    with pytest.raises(SystemExit) as exc:
        audit_cli.main()
    output = strip_ansi(capsys.readouterr().out)

    assert exc.value.code == 1
    assert "Target region directory not found" in output


def test_cli_module_rich_output_groups_by_rule_and_type() -> None:
    """
    Ensure `python -m audit_translations` rich output also shows grouped sections.

    This complements the in-process CLI test by validating module execution in
    a subprocess with environment wiring and terminal width constraints.
    """
    args = ["es", "--rules-dir", str(fixture_rules_dir()), "--file", "SharedRules/calculus.yaml", "--verbose"]

    python_scripts_dir = Path(__file__).resolve().parents[2]
    env = os.environ.copy()
    env["PYTHONPATH"] = os.pathsep.join([str(python_scripts_dir), env.get("PYTHONPATH", "")]).strip(os.pathsep)
    env["COLUMNS"] = "80"
    env["PYTHONIOENCODING"] = "utf-8"  # output contains characters like "≠" that e.g. Windows code pages lack

    result = subprocess.run(
        [sys.executable, "-m", "audit_translations", *args],
        capture_output=True,
        text=True,
        encoding="utf-8",
        cwd=str(python_scripts_dir),
        env=env,
        check=True,
    )

    output = strip_ansi(result.stdout)
    assert "≠ Rule Issues [13] (grouped by rule and issue type)" in output
    assert "• laplacian (laplacian)" in output
    assert "• divergence (divergence)" in output
    assert "Structure Differences [1]" in output
