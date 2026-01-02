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


def test_collect_issues_includes_severity_and_raw() -> None:
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

    issues = collect_issues(result, "file.yaml", "xx", include_raw=True)
    by_type = {issue["issue_type"]: issue for issue in issues}

    assert by_type["missing_rule"]["severity"] == "high"
    assert by_type["missing_rule"]["english_raw"] == "missing raw"

    assert by_type["extra_rule"]["severity"] == "low"
    assert by_type["extra_rule"]["translated_raw"] == "extra raw"

    assert by_type["untranslated_text"]["severity"] == "high"
    assert by_type["untranslated_text"]["translated_raw"] == "untranslated raw"

    assert by_type["rule_difference"]["severity"] == "medium"
    assert by_type["rule_difference"]["english_raw"] == "diff en raw"
    assert by_type["rule_difference"]["translated_raw"] == "diff tr raw"
