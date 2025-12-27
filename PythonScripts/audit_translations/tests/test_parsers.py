"""Tests for parsers.py"""

import pytest
from ..parsers import (
    has_audit_ignore,
    find_untranslated_text_keys,
    parse_rules_file,
    parse_unicode_file,
    extract_match_pattern,
    extract_conditions,
    extract_variables,
    extract_structure_elements,
)


class TestHasAuditIgnore:
    def test_detects_audit_ignore_comment(self):
        assert has_audit_ignore('- name: foo\n  # audit-ignore\n  tag: bar')

    def test_detects_inline_audit_ignore(self):
        assert has_audit_ignore('- name: foo  # audit-ignore')

    def test_returns_false_when_absent(self):
        assert not has_audit_ignore('- name: foo\n  tag: bar')

    def test_returns_false_for_similar_text(self):
        assert not has_audit_ignore('audit-ignored')
        assert not has_audit_ignore('# audit ignore')  # missing hyphen


class TestFindUntranslatedTextKeys:
    def test_finds_lowercase_t(self):
        content = 't: "hello world"'
        assert find_untranslated_text_keys(content) == ["hello world"]

    def test_finds_lowercase_ot(self):
        content = 'ot: "open paren"'
        assert find_untranslated_text_keys(content) == ["open paren"]

    def test_finds_lowercase_ct(self):
        content = 'ct: "close paren"'
        assert find_untranslated_text_keys(content) == ["close paren"]

    def test_finds_multiple(self):
        content = 't: "one"\not: "two"\nct: "three"'
        assert set(find_untranslated_text_keys(content)) == {"one", "two", "three"}

    def test_ignores_uppercase_T(self):
        content = 'T: "translated"'
        assert find_untranslated_text_keys(content) == []

    def test_ignores_variable_references(self):
        content = 't: "$variable"'
        assert find_untranslated_text_keys(content) == []

    def test_ignores_xpath_expressions(self):
        content = 't: "@attr"'
        assert find_untranslated_text_keys(content) == []

    def test_ignores_single_punctuation(self):
        content = 't: "."'
        assert find_untranslated_text_keys(content) == []


class TestParseRulesFile:
    def test_parses_simple_rule(self):
        content = '''- name: my-rule
  tag: mo
  match: "."
  replace:
    - T: "text"
'''
        rules = parse_rules_file(content)
        assert len(rules) == 1
        assert rules[0].name == "my-rule"
        assert rules[0].tag == "mo"
        assert rules[0].key == "my-rule|mo"
        assert rules[0].line_number == 1

    def test_parses_multiple_rules(self):
        content = '''- name: rule1
  tag: mo
  match: "."

- name: rule2
  tag: mi
  match: "x"
'''
        rules = parse_rules_file(content)
        assert len(rules) == 2
        assert rules[0].name == "rule1"
        assert rules[1].name == "rule2"

    def test_detects_untranslated_text(self):
        content = '''- name: untranslated
  tag: mo
  replace:
    - t: "not translated"
'''
        rules = parse_rules_file(content)
        assert rules[0].has_untranslated_text
        assert "not translated" in rules[0].untranslated_keys

    def test_detects_audit_ignore(self):
        content = '''- name: ignored-rule
  tag: mo  # audit-ignore
  match: "."
'''
        rules = parse_rules_file(content)
        assert rules[0].audit_ignore

    def test_handles_array_tag(self):
        content = '''- name: multi-tag
  tag: [mo, mtext]
  match: "."
'''
        rules = parse_rules_file(content)
        assert rules[0].tag == "[mo, mtext]"


class TestParseUnicodeFile:
    def test_parses_single_char_entry(self):
        content = '''- "a":
    - t: "a"
'''
        rules = parse_unicode_file(content)
        assert len(rules) == 1
        assert rules[0].key == "a"
        assert rules[0].name is None
        assert rules[0].tag is None

    def test_parses_range_entry(self):
        content = '''- "0-9":
    - t: "digit"
'''
        rules = parse_unicode_file(content)
        assert rules[0].key == "0-9"

    def test_parses_multiple_entries(self):
        content = '''- "a":
    - t: "a"
- "b":
    - t: "b"
'''
        rules = parse_unicode_file(content)
        assert len(rules) == 2


class TestExtractMatchPattern:
    def test_extracts_inline_match(self):
        content = 'match: "self::m:mo"'
        assert extract_match_pattern(content) == "self::m:mo"

    def test_extracts_array_match(self):
        content = '''match:
    - "self::m:mo"
    - "@intent"
'''
        assert extract_match_pattern(content) == "self::m:mo @intent"

    def test_returns_empty_for_no_match(self):
        content = 'replace:\n  - T: "text"'
        assert extract_match_pattern(content) == ""


class TestExtractConditions:
    def test_extracts_single_condition(self):
        content = 'if: "$Verbosity"'
        assert extract_conditions(content) == ["$Verbosity"]

    def test_extracts_multiple_conditions(self):
        content = '''if: "condition1"
then: something
else_test:
  if: "condition2"
'''
        conditions = extract_conditions(content)
        assert "condition1" in conditions
        assert "condition2" in conditions


class TestExtractVariables:
    def test_extracts_variables(self):
        content = 'variables: [name: "value", other: "val2"]'
        variables = extract_variables(content)
        assert ("name", "value") in variables
        assert ("other", "val2") in variables

    def test_returns_empty_for_no_variables(self):
        content = 'match: "."'
        assert extract_variables(content) == []


class TestExtractStructureElements:
    def test_extracts_test_structure(self):
        content = '''- test:
    if: "condition"
    then:
      - T: "yes"
    else:
      - T: "no"
'''
        elements = extract_structure_elements(content)
        assert "- test:" in elements
        assert "if:" in elements
        assert "then:" in elements
        assert "else:" in elements
