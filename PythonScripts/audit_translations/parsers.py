"""
YAML file parsing functions.

Handles parsing of rule files and unicode files to extract rule information.
"""

import os
import re
from typing import List, Tuple

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

    if is_unicode_file(file_path):
        rules = parse_unicode_file(content)
    else:
        rules = parse_rules_file(content)

    return rules, content


def parse_rules_file(content: str) -> List[RuleInfo]:
    """Parse a standard rules file with name/tag entries"""
    rules = []
    lines = content.split('\n')

    # Pattern to match rule start: "- name: rulename"
    name_pattern = re.compile(r'^- name:\s*(.+)$')
    tag_pattern = re.compile(r'^\s+tag:\s*(.+)$')

    i = 0
    while i < len(lines):
        line = lines[i]
        match = name_pattern.match(line)
        if match:
            rule_name = match.group(1).strip().strip('"\'')
            rule_start = i

            # Look for tag on next few lines
            tag = None
            rule_lines = [line]
            j = i + 1

            # Collect the entire rule block
            while j < len(lines):
                if lines[j].startswith('- name:') or (lines[j].strip() and not lines[j].startswith(' ') and not lines[j].startswith('#')):
                    break
                rule_lines.append(lines[j])

                # Look for tag
                if tag is None:
                    tag_match = tag_pattern.match(lines[j])
                    if tag_match:
                        tag_value = tag_match.group(1).strip()
                        # Handle array tags like [mo, mtext]
                        if tag_value.startswith('['):
                            tag = tag_value
                        else:
                            tag = tag_value.strip('"\'')
                j += 1

            raw_content = '\n'.join(rule_lines)

            # Check for untranslated text keys
            untranslated = find_untranslated_text_keys(raw_content)

            rule_key = f"{rule_name}|{tag or 'unknown'}"
            rules.append(RuleInfo(
                name=rule_name,
                tag=tag,
                key=rule_key,
                line_number=rule_start + 1,  # 1-indexed
                raw_content=raw_content,
                has_untranslated_text=len(untranslated) > 0,
                untranslated_keys=untranslated,
                audit_ignore=has_audit_ignore(raw_content)
            ))

            i = j
        else:
            i += 1

    return rules


def parse_unicode_file(content: str) -> List[RuleInfo]:
    """Parse a unicode file with character/range keys"""
    rules = []
    lines = content.split('\n')

    # Pattern to match unicode entry: ' - "a":' or ' - "0-9":'
    entry_pattern = re.compile(r'^[\s]*-\s*"([^"]+)":\s*(.*)$')

    i = 0
    while i < len(lines):
        line = lines[i]
        match = entry_pattern.match(line)
        if match:
            char_key = match.group(1)
            entry_start = i
            entry_lines = [line]

            # Collect the entire entry block
            j = i + 1
            while j < len(lines):
                next_line = lines[j]
                # Check if this is a new entry or end
                if entry_pattern.match(next_line):
                    break
                if next_line.strip() == '---':
                    break
                entry_lines.append(next_line)
                j += 1

            raw_content = '\n'.join(entry_lines)

            # Check for untranslated text keys
            untranslated = find_untranslated_text_keys(raw_content)

            rules.append(RuleInfo(
                name=None,
                tag=None,
                key=char_key,
                line_number=entry_start + 1,
                raw_content=raw_content,
                has_untranslated_text=len(untranslated) > 0,
                untranslated_keys=untranslated,
                audit_ignore=has_audit_ignore(raw_content)
            ))

            i = j
        else:
            i += 1

    return rules


def has_audit_ignore(content: str) -> bool:
    """Check if the rule content contains an audit-ignore comment"""
    return '# audit-ignore' in content


def find_untranslated_text_keys(content: str) -> List[str]:
    """
    Find lowercase text keys (t, ot, ct) that should be uppercase in translations.
    Returns list of the untranslated text values found.
    """
    untranslated = []

    # Patterns for lowercase text keys that indicate untranslated content
    # These appear as: t: "text", ot: "text", ct: "text"
    # or in array form: [t: "text"], {t: "text"}
    patterns = [
        r'\bt:\s*"([^"]+)"',      # t: "text"
        r'\bt:\s*\'([^\']+)\'',   # t: 'text'
        r'\bot:\s*"([^"]+)"',     # ot: "text"
        r'\bot:\s*\'([^\']+)\'',  # ot: 'text'
        r'\bct:\s*"([^"]+)"',     # ct: "text"
        r'\bct:\s*\'([^\']+)\'',  # ct: 'text'
    ]

    for pattern in patterns:
        for match in re.finditer(pattern, content):
            text = match.group(1)
            # Skip if it's just whitespace or a single punctuation
            if text.strip() and not (len(text) == 1 and not text.isalpha()):
                # Skip XPath expressions and variable references
                if not text.startswith('$') and not text.startswith('@'):
                    untranslated.append(text)

    return untranslated


def extract_match_pattern(content: str) -> str:
    """Extract the match pattern from a rule, handling multi-line YAML arrays"""
    lines = content.split('\n')
    match_lines = []
    in_match = False

    for line in lines:
        stripped = line.strip()
        if stripped.startswith('match:'):
            in_match = True
            # Check if it's a single-line match: "pattern"
            inline = re.search(r'match:\s*["\']([^"\']+)["\']', line)
            if inline:
                return inline.group(1)
            continue
        if in_match:
            # Stop at next top-level key (replace:, variables:, tag:, etc.)
            if re.match(r'^[a-z_]+:', stripped) and not stripped.startswith('- '):
                break
            # Collect match array items
            if stripped.startswith('- '):
                # Extract quoted content from array item
                quoted = re.search(r'-\s*["\']([^"\']+)["\']', stripped)
                if quoted:
                    match_lines.append(quoted.group(1))

    return ' '.join(match_lines)


def extract_conditions(content: str) -> List[str]:
    """Extract all if/else conditions from a rule"""
    conditions = []
    for match in re.finditer(r'if:\s*["\']([^"\']+)["\']', content):
        conditions.append(match.group(1))
    return conditions


def extract_variables(content: str) -> List[Tuple[str, str]]:
    """Extract variable definitions from a rule"""
    variables = []
    for match in re.finditer(r'variables:\s*\[([^\]]+)\]', content, re.DOTALL):
        var_block = match.group(1)
        for var_match in re.finditer(r'(\w+):\s*["\']([^"\']+)["\']', var_block):
            variables.append((var_match.group(1), var_match.group(2)))
    return variables


def extract_structure_elements(content: str) -> List[str]:
    """Extract structural elements (test, with, replace blocks) ignoring text content"""
    elements = []
    # Extract test/if/then/else structure
    for match in re.finditer(r'(- test:|if:|then:|else:|then_test:|else_test:|with:|replace:|intent:)', content):
        elements.append(match.group(1))
    return elements


def diff_rules(english_rule: RuleInfo, translated_rule: RuleInfo) -> List[RuleDifference]:
    """
    Compare two rules and return fine-grained differences.
    Ignores text content differences (T/t values) but catches structural changes.
    """
    differences = []
    en_content = english_rule.raw_content
    translated_content = translated_rule.raw_content

    # Check match pattern differences
    en_match = extract_match_pattern(en_content)
    translated_match = extract_match_pattern(translated_content)
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
    en_conditions = extract_conditions(en_content)
    tr_conditions = extract_conditions(translated_content)
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
    en_vars = extract_variables(en_content)
    tr_vars = extract_variables(translated_content)
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
    en_structure = extract_structure_elements(en_content)
    tr_structure = extract_structure_elements(translated_content)
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
