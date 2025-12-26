"""
YAML file parsing functions.

Handles parsing of rule files and unicode files to extract rule information.
"""

import os
import re
from typing import List, Tuple

from .dataclasses import RuleInfo


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
                untranslated_keys=untranslated
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
                untranslated_keys=untranslated
            ))

            i = j
        else:
            i += 1

    return rules


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
