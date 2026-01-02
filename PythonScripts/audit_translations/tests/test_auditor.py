"""Tests for auditor helpers."""

from ..auditor import collect_issues
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
        untranslated_text=[(untranslated, ["x"])],
        rule_differences=[diff],
        file_path="",
        english_rule_count=1,
        translated_rule_count=1,
    )

    issues = collect_issues(result, "file.yaml", "xx")
    by_type = {issue["issue_type"]: issue for issue in issues}

    assert by_type["missing_rule"]["line_en"] == 10
    assert by_type["missing_rule"]["line_tr"] is None
    assert "english_raw" not in by_type["missing_rule"]

    assert by_type["extra_rule"]["line_tr"] == 20
    assert "translated_raw" not in by_type["extra_rule"]

    assert by_type["untranslated_text"]["untranslated_texts"] == ["x"]
    assert "translated_raw" not in by_type["untranslated_text"]

    assert by_type["rule_difference"]["diff_type"] == "match"
    assert by_type["rule_difference"]["english_snippet"] == "a"
    assert by_type["rule_difference"]["translated_snippet"] == "b"
    assert "english_raw" not in by_type["rule_difference"]
