"""
Tests for parsers.py.
"""

import pytest
from ruamel.yaml import YAML
from ruamel.yaml.scanner import ScannerError

from ..dataclasses import RuleInfo
from ..parsers import (
    diff_rules,
    extract_conditions,
    extract_match_pattern,
    extract_structure_elements,
    extract_variables,
    find_untranslated_text_values,
    has_audit_ignore,
    parse_rules_file,
    parse_unicode_file,
)


class TestHasAuditIgnore:
    def test_detects_audit_ignore_comment(self):
        """Ensure detects audit ignore comment."""
        assert has_audit_ignore("- name: foo\n  # audit-ignore\n  tag: bar")

    def test_detects_inline_audit_ignore(self):
        """Ensure detects inline audit ignore."""
        assert has_audit_ignore("- name: foo  # audit-ignore")

    def test_returns_false_when_absent(self):
        """Ensure returns false when absent."""
        assert not has_audit_ignore("- name: foo\n  tag: bar")

    def test_returns_false_for_similar_text(self):
        """Ensure returns false for similar text."""
        assert not has_audit_ignore("audit-ignored")
        assert not has_audit_ignore("# audit ignore")  # missing hyphen


class TestFindUntranslatedTextKeys:
    def test_finds_lowercase_t(self):
        """Ensure finds lowercase t."""
        content = {"t": "hello world"}
        assert find_untranslated_text_values(content) == ["hello world"]

    def test_finds_lowercase_ot(self):
        """Ensure finds lowercase ot."""
        content = {"ot": "open paren"}
        assert find_untranslated_text_values(content) == ["open paren"]

    def test_finds_lowercase_ct(self):
        """Ensure finds lowercase ct."""
        content = {"ct": "close paren"}
        assert find_untranslated_text_values(content) == ["close paren"]

    def test_finds_multiple(self):
        """Ensure finds multiple."""
        content = {"t": "one", "ot": "two", "ct": "three"}
        assert set(find_untranslated_text_values(content)) == {"one", "two", "three"}

    def test_ignores_uppercase_T(self):
        """Ensure ignores uppercase T."""
        content = {"T": "translated"}
        assert find_untranslated_text_values(content) == []

    def test_finds_spell_and_pronounce(self):
        """Detects lowercase spell and pronounce markers.

        Extends coverage beyond basic t/ot/ct fields.
        Flags auxiliary translation-bearing keys."""
        content = {"spell": "alpha", "pronounce": "beta"}
        assert set(find_untranslated_text_values(content)) == {"alpha", "beta"}

    def test_ignores_uppercase_variants(self):
        """Ignores uppercase variants of extended markers.

        Honors already-verified spell/pronounce/IfThenElse content.
        Avoids double-reporting translated data."""
        content = {"PRONOUNCE": "gamma", "IFTHENELSE": "delta"}
        assert find_untranslated_text_values(content) == []

    def test_ignores_variable_references(self):
        """Ensure ignores variable references."""
        content = {"t": "$variable"}
        assert find_untranslated_text_values(content) == []

    def test_ignores_xpath_expressions(self):
        """Ensure ignores xpath expressions."""
        content = {"t": "@attr"}
        assert find_untranslated_text_values(content) == []

    def test_ignores_single_punctuation(self):
        """Ensure ignores single punctuation."""
        content = {"t": "."}
        assert find_untranslated_text_values(content) == []


class TestParseRulesFile:
    def test_parses_simple_rule(self):
        """Ensure parses simple rule."""
        content = """- name: my-rule
  tag: mo
  match: "."
  replace:
    - T: "text"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert len(rules) == 1
        assert rules[0].name == "my-rule"
        assert rules[0].tag == "mo"
        assert rules[0].key == "my-rule|mo"
        assert rules[0].line_number == 1

    def test_parses_multiple_rules(self):
        """Ensure parses multiple rules."""
        content = """- name: rule1
  tag: mo
  match: "."

- name: rule2
  tag: mi
  match: "x"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert len(rules) == 2
        assert rules[0].name == "rule1"
        assert rules[1].name == "rule2"

    def test_detects_untranslated_text(self):
        """Ensure detects untranslated text."""
        content = """- name: untranslated
  tag: mo
  replace:
    - t: "not translated"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert rules[0].has_untranslated_text
        assert "not translated" in rules[0].untranslated_keys

    def test_detects_audit_ignore(self):
        """Ensure detects audit ignore."""
        content = """- name: ignored-rule
  tag: mo  # audit-ignore
  match: "."
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert rules[0].audit_ignore

    def test_handles_array_tag(self):
        """Ensure handles array tag."""
        content = """- name: multi-tag
  tag: [mo, mtext]
  match: "."
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert rules[0].tag == "[mo, mtext]"

    def test_sorts_tag_lists(self):
        """Normalizes unordered tag lists for stable comparison.

        Confirms sorting prevents false positives in diffs.
        Keeps tag-based keys consistent across translations."""
        content = """- name: multi-tag
  tag: [mtext, mo]
  match: "."
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_rules_file(content, data)
        assert rules[0].tag == "[mo, mtext]"

    def test_parse_yaml_file_handles_tabs(self, tmp_path):
        """Ensure parse yaml file handles tabs."""
        content = """- name: tabbed
  tag: mo
  match: "."
  replace:
    - t: "x"\t# tab before comment
"""
        file_path = tmp_path / "tabbed.yaml"
        file_path.write_text(content, encoding="utf-8")
        from ..parsers import parse_yaml_file

        rules, _ = parse_yaml_file(str(file_path))
        assert len(rules) == 1
        assert rules[0].name == "tabbed"

    def test_parse_yaml_file_strict_rejects_tabs(self, tmp_path):
        """Ensure parse yaml file strict rejects tabs."""
        content = """- name: tabbed
  tag: mo
  match: "."
  replace:
    - t: "x"\t# tab before comment
"""
        file_path = tmp_path / "tabbed.yaml"
        file_path.write_text(content, encoding="utf-8")
        from ..parsers import parse_yaml_file

        with pytest.raises(ScannerError):
            parse_yaml_file(str(file_path), strict=True)


class TestParseUnicodeFile:
    def test_parses_single_char_entry(self):
        """Ensure parses single char entry."""
        content = """- "a":
    - t: "a"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_unicode_file(content, data)
        assert len(rules) == 1
        assert rules[0].key == "a"
        assert rules[0].name is None
        assert rules[0].tag is None

    def test_parses_range_entry(self):
        """Ensure parses range entry."""
        content = """- "0-9":
    - t: "digit"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_unicode_file(content, data)
        assert rules[0].key == "0-9"

    def test_parses_multiple_entries(self):
        """Ensure parses multiple entries."""
        content = """- "a":
    - t: "a"
- "b":
    - t: "b"
"""
        yaml = YAML()
        data = yaml.load(content)
        rules = parse_unicode_file(content, data)
        assert len(rules) == 2


class TestExtractMatchPattern:
    def test_extracts_inline_match(self):
        """Ensure extracts inline match."""
        data = {"match": "self::m:mo"}
        assert extract_match_pattern(data) == "self::m:mo"

    def test_extracts_array_match(self):
        """Ensure extracts array match."""
        data = {"match": ["self::m:mo", "@intent"]}
        assert extract_match_pattern(data) == "self::m:mo @intent"

    def test_returns_empty_for_no_match(self):
        """Ensure returns empty for no match."""
        data = {"replace": [{"T": "text"}]}
        assert extract_match_pattern(data) == ""


class TestExtractConditions:
    def test_extracts_single_condition(self):
        """Ensure extracts single condition."""
        data = {"if": "$Verbosity"}
        assert extract_conditions(data) == ["$Verbosity"]

    def test_extracts_multiple_conditions(self):
        """Ensure extracts multiple conditions."""
        data = {"if": "condition1", "then": "something", "else_test": {"if": "condition2"}}
        conditions = extract_conditions(data)
        assert "condition1" in conditions
        assert "condition2" in conditions


class TestExtractVariables:
    def test_extracts_variables(self):
        """Ensure extracts variables."""
        data = {"variables": [{"name": "value"}, {"other": "val2"}]}
        variables = extract_variables(data)
        assert ("name", "value") in variables
        assert ("other", "val2") in variables

    def test_returns_empty_for_no_variables(self):
        """Ensure returns empty for no variables."""
        data = {"match": "."}
        assert extract_variables(data) == []


class TestExtractStructureElements:
    def test_extracts_test_structure(self):
        """Ensure extracts test structure."""
        data = {"test": {"if": "condition", "then": [{"T": "yes"}], "else": [{"T": "no"}]}}
        elements = extract_structure_elements(data)
        assert "test:" in elements
        assert "if:" in elements
        assert "then:" in elements
        assert "else:" in elements


def make_rule(name: str, tag: str, data) -> RuleInfo:
    """Helper to create RuleInfo for testing"""
    return RuleInfo(
        name=name,
        tag=tag,
        key=f"{name}|{tag}",
        line_number=1,
        raw_content="",
        data=data,
    )


class TestDiffRules:
    def test_identical_rules_no_diff(self):
        """Ensure identical rules no diff."""
        data = {"name": "test", "tag": "mo", "match": "self::m:mo", "replace": [{"T": "text"}]}
        en = make_rule("test", "mo", data)
        tr = make_rule("test", "mo", data)
        assert diff_rules(en, tr) == []

    def test_detects_match_pattern_difference(self):
        """Ensure detects match pattern difference."""
        en = make_rule("test", "mo", {"match": "self::m:mo"})
        tr = make_rule("test", "mo", {"match": "self::m:mi"})
        diffs = diff_rules(en, tr)
        assert len(diffs) == 1
        assert diffs[0].diff_type == "match"
        assert "self::m:mo" in diffs[0].english_snippet
        assert "self::m:mi" in diffs[0].translated_snippet

    def test_detects_condition_difference(self):
        """Ensure detects condition difference."""
        en = make_rule("test", "mo", {"if": "condition1"})
        tr = make_rule("test", "mo", {"if": "condition2"})
        diffs = diff_rules(en, tr)
        assert any(d.diff_type == "condition" for d in diffs)

    def test_detects_missing_condition(self):
        """Ensure detects missing condition."""
        en = make_rule("test", "mo", {"if": "condition1"})
        tr = make_rule("test", "mo", {"replace": [{"T": "text"}]})
        diffs = diff_rules(en, tr)
        assert any(d.diff_type == "condition" for d in diffs)

    def test_detects_variable_difference(self):
        """Ensure detects variable difference."""
        en = make_rule("test", "mo", {"variables": [{"foo": "bar"}]})
        tr = make_rule("test", "mo", {"variables": [{"baz": "qux"}]})
        diffs = diff_rules(en, tr)
        assert any(d.diff_type == "variables" for d in diffs)

    def test_detects_structure_difference(self):
        """Ensure detects structure difference."""
        en = make_rule("test", "mo", {"test": {"if": "cond", "then": [{"T": "yes"}], "else": [{"T": "no"}]}})
        tr = make_rule("test", "mo", {"test": {"if": "cond", "then": [{"T": "ja"}]}})
        diffs = diff_rules(en, tr)
        assert any(d.diff_type == "structure" for d in diffs)

    def test_multiple_differences(self):
        """Ensure multiple differences."""
        en = make_rule("test", "mo", {"match": "self::m:mo", "if": "cond1"})
        tr = make_rule("test", "mo", {"match": "self::m:mi", "if": "cond2"})
        diffs = diff_rules(en, tr)
        assert len(diffs) == 2
        types = {d.diff_type for d in diffs}
        assert "match" in types
        assert "condition" in types

    def test_ignores_text_content_differences(self):
        """Ensure ignores text content differences."""
        en = make_rule("test", "mo", {"replace": [{"T": "hello"}]})
        tr = make_rule("test", "mo", {"replace": [{"T": "hallo"}]})
        diffs = diff_rules(en, tr)
        assert diffs == []  # text differences are intentional translations
