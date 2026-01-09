"""
Tests for auditor helpers.
"""

from ..auditor import collect_issues, compare_files, console, get_yaml_files, list_languages
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
