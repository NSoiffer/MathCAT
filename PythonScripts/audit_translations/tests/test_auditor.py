"""
Tests for auditor helpers.
"""

from pathlib import Path

from ..auditor import collect_issues, compare_files, console, get_yaml_files, list_languages, print_warnings
from ..dataclasses import ComparisonResult, RuleDifference, RuleInfo


def make_rule(name: str, tag: str, line: int, raw: str) -> RuleInfo:
    return RuleInfo(
        name=name,
        tag=tag,
        key=f"{name}|{tag}",
        line_number=line,
        raw_content=raw,
    )


def test_collect_issues_fields() -> None:
    """Ensure collect issues fields."""
    missing = make_rule("missing", "mo", 10, "missing raw")
    extra = make_rule("extra", "mi", 20, "extra raw")
    untranslated = make_rule("untranslated", "mn", 30, "untranslated raw")
    diff_en = make_rule("diff", "mrow", 40, "diff en raw")
    diff_tr = make_rule("diff", "mrow", 41, "diff tr raw")

    diff = RuleDifference(
        english_rule=diff_en,
        translated_rule=diff_tr,
        diff_type="match",
        description="Match differs",
        english_snippet="a",
        translated_snippet="b",
    )

    result = ComparisonResult(
        missing_rules=[missing],
        extra_rules=[extra],
        untranslated_text=[(untranslated, [("t", "x", 31)])],
        rule_differences=[diff],
        file_path="",
        english_rule_count=1,
        translated_rule_count=1,
    )

    issues = collect_issues(result, "file.yaml", "xx")
    by_type = {issue["issue_type"]: issue for issue in issues}

    assert by_type["missing_rule"]["issue_line_en"] == 10
    assert by_type["missing_rule"]["issue_line_tr"] is None
    assert by_type["missing_rule"]["rule_line_en"] == 10
    assert by_type["missing_rule"]["rule_line_tr"] is None
    assert "english_raw" not in by_type["missing_rule"]

    assert by_type["extra_rule"]["issue_line_tr"] == 20
    assert by_type["extra_rule"]["rule_line_tr"] == 20
    assert "translated_raw" not in by_type["extra_rule"]

    assert by_type["untranslated_text"]["untranslated_texts"] == ["x"]
    assert by_type["untranslated_text"]["issue_line_tr"] == 31
    assert by_type["untranslated_text"]["rule_line_tr"] == 30
    assert "translated_raw" not in by_type["untranslated_text"]

    assert by_type["rule_difference"]["diff_type"] == "match"
    assert by_type["rule_difference"]["english_snippet"] == "a"
    assert by_type["rule_difference"]["translated_snippet"] == "b"
    assert by_type["rule_difference"]["issue_line_en"] == 40
    assert by_type["rule_difference"]["issue_line_tr"] == 41
    assert by_type["rule_difference"]["rule_line_en"] == 40
    assert by_type["rule_difference"]["rule_line_tr"] == 41
    assert "english_raw" not in by_type["rule_difference"]


def test_compare_files_merges_region_rules(tmp_path) -> None:
    """
    Ensures region rule files merge with base language before comparison.

    Confirms region overrides are recognized alongside base translations.
    Prevents false positives when content is split across directories.
    """
    rules_dir = tmp_path / "Rules" / "Languages"
    english_dir = rules_dir / "en"
    lang_dir = rules_dir / "zz"
    region_dir = lang_dir / "aa"
    english_dir.mkdir(parents=True)
    lang_dir.mkdir(parents=True)
    region_dir.mkdir(parents=True)

    english_file = english_dir / "base.yaml"
    translated_file = lang_dir / "base.yaml"
    translated_region_file = region_dir / "base.yaml"

    english_file.write_text(
        """- name: base-one
  tag: mo
  match: "."
  replace:
    - t: "one"
- name: base-two
  tag: mi
  match: "."
  replace:
    - t: "two"
""",
        encoding="utf-8",
    )

    translated_file.write_text(
        """- name: base-one
  tag: mo
  match: "."
  replace:
    - T: "eins"
""",
        encoding="utf-8",
    )

    translated_region_file.write_text(
        """- name: base-two
  tag: mi
  match: "."
  replace:
    - T: "zwei"
""",
        encoding="utf-8",
    )

    result = compare_files(
        str(english_file),
        str(translated_file),
        None,
        str(translated_region_file),
    )

    assert result.missing_rules == []
    assert result.extra_rules == []


def test_get_yaml_files_includes_region(tmp_path) -> None:
    """
    Ensures get_yaml_files merges base and region file lists.

    Verifies both root and SharedRules entries are discovered.
    Confirms region-only files are added with their own relative paths.
    """
    lang_dir = tmp_path / "lang"
    region_dir = lang_dir / "aa"
    (lang_dir / "SharedRules").mkdir(parents=True)
    region_dir.mkdir(parents=True)

    (lang_dir / "base.yaml").write_text("---", encoding="utf-8")
    (lang_dir / "SharedRules" / "shared.yaml").write_text("---", encoding="utf-8")
    (region_dir / "unicode.yaml").write_text("---", encoding="utf-8")

    files = get_yaml_files(lang_dir, region_dir)
    assert set(files) == {"base.yaml", "SharedRules/shared.yaml", "unicode.yaml"}


def test_list_languages_includes_region_codes(tmp_path) -> None:
    """
    Ensures list_languages reports region variants.

    Confirms region directories appear alongside base language entries.
    Protects CLI output from hiding available variants.
    """
    rules_dir = tmp_path / "Rules" / "Languages"
    (rules_dir / "en").mkdir(parents=True)
    lang_dir = rules_dir / "zz"
    region_dir = lang_dir / "aa"
    lang_dir.mkdir(parents=True)
    region_dir.mkdir(parents=True)

    (lang_dir / "file.yaml").write_text("---", encoding="utf-8")
    (region_dir / "region.yaml").write_text("---", encoding="utf-8")

    with console.capture() as capture:
        list_languages(str(rules_dir))
    output = capture.get()

    assert "zz" in output
    assert "zz-aa" in output


def test_print_warnings_omits_snippets_when_not_verbose() -> None:
    """
    Ensure the print_warnings output matches the non-verbose golden snapshot.
    """
    base_dir = Path(__file__).parent
    fixtures_dir = base_dir / "fixtures"
    golden_path = base_dir / "golden" / "rich" / "structure_diff_nonverbose.golden"
    result = compare_files(
        str(fixtures_dir / "en" / "structure_diff.yaml"),
        str(fixtures_dir / "de" / "structure_diff.yaml"),
    )

    with console.capture() as capture:
        print_warnings(result, "structure_diff.yaml", verbose=False)
    output = capture.get()

    assert output == golden_path.read_text(encoding="utf-8")


def test_print_warnings_includes_snippets_when_verbose() -> None:
    """
    Ensure the print_warnings output matches the verbose golden snapshot.
    """
    base_dir = Path(__file__).parent
    fixtures_dir = base_dir / "fixtures"
    golden_path = base_dir / "golden" / "rich" / "structure_diff_verbose.golden"
    result = compare_files(
        str(fixtures_dir / "en" / "structure_diff.yaml"),
        str(fixtures_dir / "de" / "structure_diff.yaml"),
    )

    with console.capture() as capture:
        print_warnings(result, "structure_diff.yaml", verbose=True)
    output = capture.get()

    assert output == golden_path.read_text(encoding="utf-8")


def test_misaligned_structure_differences_are_skipped() -> None:
    """
    Test that structure differences with misaligned tokens are skipped.

    When English has a "test" block that Norwegian doesn't have (and vice versa),
    the structural tokens become misaligned. The fix skips reporting these
    to avoid showing confusing line numbers.
    """
    base_dir = Path(__file__).parent
    fixtures_dir = base_dir / "fixtures"

    result = compare_files(
        str(fixtures_dir / "en" / "structure_misaligned.yaml"),
        str(fixtures_dir / "de" / "structure_misaligned.yaml"),
    )

    # The result should detect that structures differ
    assert len(result.rule_differences) > 0
    assert any(diff.diff_type == "structure" for diff in result.rule_differences)

    # But when collecting issues, misaligned structure diffs should be filtered out
    issues = collect_issues(result, "structure_misaligned.yaml", "de")
    structure_issues = [i for i in issues if i["diff_type"] == "structure"]

    # CRITICAL: Before the fix, this would have structure issues with misleading line numbers
    # After the fix, misaligned structures are skipped, so we should have 0 structure issues
    assert len(structure_issues) == 0, (
        "Expected misaligned structure differences to be filtered out, "
        f"but found {len(structure_issues)} structure issues"
    )

    # Other differences (like conditions) should still be reported
    condition_issues = [i for i in issues if i["diff_type"] == "condition"]
    assert len(condition_issues) > 0, "Expected condition differences to still be reported"


def test_missing_else_block_is_still_reported() -> None:
    """
    Test that legitimate missing structural elements are still reported.

    When one file has an 'else' block and the other doesn't, this is a clear
    structural difference that should be reported with accurate line numbers.
    """
    base_dir = Path(__file__).parent
    fixtures_dir = base_dir / "fixtures"

    result = compare_files(
        str(fixtures_dir / "en" / "structure_missing_else.yaml"),
        str(fixtures_dir / "de" / "structure_missing_else.yaml"),
    )

    # Should detect structure difference
    assert len(result.rule_differences) > 0
    structure_diffs = [diff for diff in result.rule_differences if diff.diff_type == "structure"]
    assert len(structure_diffs) == 1

    # This case has one token None (missing else), so it should still be reported
    issues = collect_issues(result, "structure_missing_else.yaml", "de")
    structure_issues = [i for i in issues if i["diff_type"] == "structure"]

    # CRITICAL: This legitimate difference should still be reported
    # One file has else:, the other doesn't - a clear missing element
    assert len(structure_issues) == 1, (
        "Expected missing else block to be reported, "
        f"but found {len(structure_issues)} structure issues"
    )

    # Verify the issue has proper line numbers
    issue = structure_issues[0]
    assert issue["issue_line_en"] is not None
    # When else: doesn't exist in translation, we fall back to the rule line number
    assert issue["issue_line_tr"] == 1  # start of the rule


def test_print_warnings_skips_misaligned_structures() -> None:
    """
    Test that print_warnings doesn't display misaligned structure differences.
    """
    base_dir = Path(__file__).parent
    fixtures_dir = base_dir / "fixtures"

    result = compare_files(
        str(fixtures_dir / "en" / "structure_misaligned.yaml"),
        str(fixtures_dir / "de" / "structure_misaligned.yaml"),
    )

    # Raw result should have structure differences detected
    structure_diffs = [diff for diff in result.rule_differences if diff.diff_type == "structure"]
    assert len(structure_diffs) > 0

    with console.capture() as capture:
        issues_count = print_warnings(result, "structure_misaligned.yaml", verbose=False)
    output = capture.get()

    # CRITICAL: The output should not contain "Rule structure differs"
    # because misaligned structures are filtered during display
    assert "Rule structure differs" not in output, (
        "Expected misaligned structure differences to be filtered from display"
    )

    # The issues count should not include filtered structure differences
    # It should only count the condition differences
    condition_diffs = [diff for diff in result.rule_differences if diff.diff_type == "condition"]
    assert issues_count == len(condition_diffs), (
        f"Expected issues_count ({issues_count}) to equal condition_diffs ({len(condition_diffs)})"
    )


def test_print_warnings_still_shows_missing_else() -> None:
    """
    Test that print_warnings still displays legitimate missing elements.
    """
    base_dir = Path(__file__).parent
    fixtures_dir = base_dir / "fixtures"

    result = compare_files(
        str(fixtures_dir / "en" / "structure_missing_else.yaml"),
        str(fixtures_dir / "de" / "structure_missing_else.yaml"),
    )

    with console.capture() as capture:
        issues_count = print_warnings(result, "structure_missing_else.yaml", verbose=False)
    output = capture.get()

    # CRITICAL: This legitimate difference should appear in output
    assert "Rule structure differs" in output, (
        "Expected missing else block to be shown in output"
    )

    # Should report exactly 1 issue (the structure difference)
    assert issues_count == 1, f"Expected 1 issue but got {issues_count}"
