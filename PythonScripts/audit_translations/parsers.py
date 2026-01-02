"""
YAML file parsing functions.

Handles parsing of rule files and unicode files to extract rule information.
"""

import os
from typing import Any, List, Tuple

from ruamel.yaml import YAML
from ruamel.yaml.scanner import ScannerError

from .dataclasses import RuleInfo, RuleDifference


def is_unicode_file(file_path: str) -> bool:
    """Check if this is a unicode.yaml or unicode-full.yaml file"""
    basename = os.path.basename(file_path)
    return basename in ("unicode.yaml", "unicode-full.yaml")


def parse_yaml_file(file_path: str) -> Tuple[List[RuleInfo], str]:
    """
    Parse a YAML file and extract rules.
    Returns list of RuleInfo and the raw file content.

    For standard rule files: extracts rules with name/tag
    For unicode files: extracts entries with character/range keys
    """
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    yaml = YAML()
    yaml.preserve_quotes = True
    try:
        data = yaml.load(content)
    except ScannerError as exc:
        if "\t" in content:
            sanitized = content.replace("\t", "    ")
            data = yaml.load(sanitized)
        else:
            raise exc

    if is_unicode_file(file_path):
        rules = parse_unicode_file(content, data)
    else:
        rules = parse_rules_file(content, data)

    return rules, content



def format_tag(tag_value: Any) -> Optional[str]:
    if tag_value is None:
        return None
    if isinstance(tag_value, list):
        return "[" + ", ".join(str(item) for item in tag_value) + "]"
    return str(tag_value)


def build_raw_blocks(lines: List[str], starts: List[int]) -> List[str]:
    blocks = []
    if not starts:
        return blocks
    for idx, start in enumerate(starts):
        end = starts[idx + 1] if idx + 1 < len(starts) else len(lines)
        blocks.append("\n".join(lines[start:end]))
    return blocks


def parse_rules_file(content: str, data: Any) -> List[RuleInfo]:
    """Parse a standard rules file with name/tag entries"""
    if not isinstance(data, list):
        return []

    rules: List[RuleInfo] = []
    lines = content.splitlines()

    start_lines: List[int] = []
    rule_items: List[Any] = []
    for idx, item in enumerate(data):
        if isinstance(item, dict) and "name" in item:
            line = data.lc.item(idx)[0] if hasattr(data, "lc") else 0
            start_lines.append(line)
            rule_items.append(item)

    raw_blocks = build_raw_blocks(lines, start_lines)

    for item, raw_content, line_idx in zip(rule_items, raw_blocks, start_lines):
        rule_name = str(item.get("name"))
        tag = format_tag(item.get("tag"))
        untranslated = find_untranslated_text_values(item)
        rule_key = f"{rule_name}|{tag or 'unknown'}"
        rules.append(RuleInfo(
            name=rule_name,
            tag=tag,
            key=rule_key,
            line_number=line_idx + 1,
            raw_content=raw_content,
            data=item,
            has_untranslated_text=len(untranslated) > 0,
            untranslated_keys=untranslated,
            audit_ignore=has_audit_ignore(raw_content)
        ))

    return rules


def parse_unicode_file(content: str, data: Any) -> List[RuleInfo]:
    """Parse a unicode file with character/range keys"""
    if not isinstance(data, list):
        return []

    rules: List[RuleInfo] = []
    lines = content.splitlines()

    start_lines: List[int] = []
    entries: List[Tuple[str, Any]] = []
    for idx, item in enumerate(data):
        if isinstance(item, dict) and len(item) == 1:
            key = next(iter(item.keys()))
            value = item[key]
            line = data.lc.item(idx)[0] if hasattr(data, "lc") else 0
            start_lines.append(line)
            entries.append((str(key), value))

    raw_blocks = build_raw_blocks(lines, start_lines)

    for (char_key, value), raw_content, line_idx in zip(entries, raw_blocks, start_lines):
        untranslated = find_untranslated_text_values(value)
        rules.append(RuleInfo(
            name=None,
            tag=None,
            key=char_key,
            line_number=line_idx + 1,
            raw_content=raw_content,
            data=value,
            has_untranslated_text=len(untranslated) > 0,
            untranslated_keys=untranslated,
            audit_ignore=has_audit_ignore(raw_content)
        ))

    return rules


def has_audit_ignore(content: str) -> bool:
    """Check if the rule content contains an audit-ignore comment"""
    return '# audit-ignore' in content


def find_untranslated_text_values(node: Any) -> List[str]:
    """
    Find lowercase text keys (t, ot, ct) that should be uppercase in translations.
    Returns list of the untranslated text values found.
    """
    untranslated: List[str] = []

    def should_add(text: str) -> bool:
        if not text.strip():
            return False
        if len(text) == 1 and not text.isalpha():
            return False
        if text.startswith('$') or text.startswith('@'):
            return False
        return True

    def walk(value: Any) -> None:
        if isinstance(value, dict):
            for key, child in value.items():
                if key in ("t", "ot", "ct") and isinstance(child, str):
                    if should_add(child):
                        untranslated.append(child)
                walk(child)
        elif isinstance(value, list):
            for item in value:
                walk(item)

    walk(node)
    return untranslated


def normalize_match(value: Any) -> str:
    if isinstance(value, list):
        return " ".join(str(item) for item in value)
    if isinstance(value, str):
        return value
    return ""


def normalize_xpath(value: str) -> str:
    return " ".join(value.split())


def extract_match_pattern(rule_data: Any) -> str:
    if isinstance(rule_data, dict):
        return normalize_match(rule_data.get("match"))
    return ""


def extract_conditions(rule_data: Any) -> List[str]:
    """Extract all if/else conditions from a rule"""
    conditions: List[str] = []

    def walk(value: Any) -> None:
        if isinstance(value, dict):
            for key, child in value.items():
                if key in ("if", "else_if") and isinstance(child, str):
                    conditions.append(child)
                walk(child)
        elif isinstance(value, list):
            for item in value:
                walk(item)

    walk(rule_data)
    return conditions


def extract_variables(rule_data: Any) -> List[Tuple[str, str]]:
    """Extract variable definitions from a rule"""
    variables: List[Tuple[str, str]] = []

    def add_from_value(value: Any) -> None:
        if isinstance(value, dict):
            for name, expr in value.items():
                variables.append((str(name), str(expr)))
        elif isinstance(value, list):
            for item in value:
                if isinstance(item, dict):
                    for name, expr in item.items():
                        variables.append((str(name), str(expr)))

    def walk(value: Any) -> None:
        if isinstance(value, dict):
            for key, child in value.items():
                if key == "variables":
                    add_from_value(child)
                walk(child)
        elif isinstance(value, list):
            for item in value:
                walk(item)

    walk(rule_data)
    return variables


def extract_structure_elements(rule_data: Any) -> List[str]:
    """Extract structural elements (test, with, replace blocks) ignoring text content"""
    elements: List[str] = []
    tokens = {"test", "if", "else_if", "then", "else", "then_test", "else_test", "with", "replace", "intent"}

    def walk(value: Any) -> None:
        if isinstance(value, dict):
            for key, child in value.items():
                if key in tokens:
                    elements.append(f"{key}:")
                walk(child)
        elif isinstance(value, list):
            for item in value:
                walk(item)

    walk(rule_data)
    return elements


def diff_rules(english_rule: RuleInfo, translated_rule: RuleInfo) -> List[RuleDifference]:
    """
    Compare two rules and return fine-grained differences.
    Ignores text content differences (T/t values) but catches structural changes.
    """
    differences = []
    # Check match pattern differences
    en_match_raw = extract_match_pattern(english_rule.data)
    translated_match_raw = extract_match_pattern(translated_rule.data)
    en_match = normalize_xpath(en_match_raw)
    translated_match = normalize_xpath(translated_match_raw)
    if en_match != translated_match and en_match and translated_match:
        differences.append(RuleDifference(
            english_rule=english_rule,
            translated_rule=translated_rule,
            diff_type='match',
            description='Match pattern differs',
            english_snippet=en_match,
            translated_snippet=translated_match
        ))

    # Check condition differences
    en_conditions_raw = extract_conditions(english_rule.data)
    tr_conditions_raw = extract_conditions(translated_rule.data)
    en_conditions = [normalize_xpath(c) for c in en_conditions_raw]
    tr_conditions = [normalize_xpath(c) for c in tr_conditions_raw]
    if en_conditions != tr_conditions:
        # Find specific differences
        en_set, tr_set = set(en_conditions), set(tr_conditions)
        if en_set != tr_set:
            differences.append(RuleDifference(
                english_rule=english_rule,
                translated_rule=translated_rule,
                diff_type='condition',
                description='Conditions differ',
                english_snippet=', '.join(sorted(en_set)) or '(none)',
                translated_snippet=', '.join(sorted(tr_set)) or '(none)'
            ))

    # Check variable differences
    en_vars = extract_variables(english_rule.data)
    tr_vars = extract_variables(translated_rule.data)
    if en_vars != tr_vars:
        en_var_names = {v[0] for v in en_vars}
        tr_var_names = {v[0] for v in tr_vars}
        if en_var_names != tr_var_names:
            differences.append(RuleDifference(
                english_rule=english_rule,
                translated_rule=translated_rule,
                diff_type='variables',
                description='Variable definitions differ',
                english_snippet=', '.join(sorted(en_var_names)) or '(none)',
                translated_snippet=', '.join(sorted(tr_var_names)) or '(none)'
            ))

    # Check structural differences (test/if/then/else blocks)
    en_structure = extract_structure_elements(english_rule.data)
    tr_structure = extract_structure_elements(translated_rule.data)
    if en_structure != tr_structure:
        differences.append(RuleDifference(
            english_rule=english_rule,
            translated_rule=translated_rule,
            diff_type='structure',
            description='Rule structure differs (test/if/then/else blocks)',
            english_snippet=' '.join(en_structure),
            translated_snippet=' '.join(tr_structure)
        ))

    return differences
