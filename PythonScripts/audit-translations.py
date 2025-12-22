#!/usr/bin/env python3
"""
MathCAT Translation Audit Tool

Compares English YAML rule files with translated versions to identify translation
gaps and issues. This tool helps translators ensure their translations are complete
and properly formatted.

Detection Capabilities:
    1. Missing rules - Rules present in English but not in the translation
    2. Extra rules - Rules in translation but absent from English (flagged as
       potentially intentional language-specific additions)
    3. Untranslated text - Detects text keys that still use lowercase formatting,
       indicating they haven't been reviewed/translated yet

Text Key Abbreviations (t/ot/ct):
    In MathCAT YAML rule files, text output is marked with special keys:
    - t:  "text" - Simple text output spoken as-is
    - ot: "open text" - Text spoken at the start of a construct (e.g., "start fraction")
    - ct: "close text" - Text spoken at the end of a construct (e.g., "end fraction")

    The translation convention is:
    - Lowercase (t, ot, ct): Untranslated or unverified text (needs review)
    - Uppercase (T, OT, CT): Translated and verified text

    Example:
        English:  - t: "square root"      # lowercase = original English
        Spanish:  - T: "raíz cuadrada"    # uppercase = verified translation

File Type Handling:
    - Standard rule files: Uses name/tag identifiers to match rules
      (e.g., ClearSpeak_Rules.yaml, SimpleSpeak_Rules.yaml, SharedRules/*.yaml)
    - Unicode files: Uses character/range keys like "a-z", "!", "0-9"
      (unicode.yaml, unicode-full.yaml)

Convenience Features:
    - --list flag to show available languages
    - --file option to audit a specific file only
    - Summary statistics after each run

Usage:
    python audit-translations.py <language> [--file <specific_file>]
    python audit-translations.py --list

Examples:
    # List available languages
    python audit-translations.py --list

    # Audit all Spanish translation files
    python audit-translations.py es

    # Audit only a specific file
    python audit-translations.py es --file SharedRules/default.yaml

    # Audit German translations
    python audit-translations.py de
"""




import argparse
import io
import os
import re
import sys
from pathlib import Path
from typing import List, Tuple, Optional
from dataclasses import dataclass

# Ensure UTF-8 output on Windows
if sys.platform == 'win32':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')


@dataclass
class RuleInfo:
    """Information about a single rule"""
    name: Optional[str]  # None for unicode entries
    tag: Optional[str]   # None for unicode entries
    key: str             # For unicode entries, this is the character/range
    line_number: int
    raw_content: str
    has_untranslated_text: bool = False
    untranslated_keys: List[str] = None

    def __post_init__(self):
        if self.untranslated_keys is None:
            self.untranslated_keys = []


@dataclass
class ComparisonResult:
    """Results from comparing English and translated files"""
    missing_rules: List[RuleInfo]           # Rules in English but not in translation
    extra_rules: List[RuleInfo]             # Rules in translation but not in English
    untranslated_text: List[Tuple[RuleInfo, List[str]]]  # Rules with lowercase t/ot/ct
    file_path: str
    english_rule_count: int
    translated_rule_count: int


def get_rules_dir() -> Path:
    """Get the Rules/Languages directory path"""
    script_dir = Path(__file__).parent
    return script_dir.parent / "Rules" / "Languages"


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

    rules = []
    is_unicode = is_unicode_file(file_path)

    if is_unicode:
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


def compare_files(english_path: str, translated_path: str) -> ComparisonResult:
    """Compare English and translated YAML files"""

    english_rules, _ = parse_yaml_file(english_path)

    if os.path.exists(translated_path):
        translated_rules, _ = parse_yaml_file(translated_path)
    else:
        translated_rules = []

    # Create lookup dictionaries
    english_by_key = {r.key: r for r in english_rules}
    translated_by_key = {r.key: r for r in translated_rules}

    # Find missing rules (in English but not in translation)
    missing_rules = []
    for key, rule in english_by_key.items():
        if key not in translated_by_key:
            missing_rules.append(rule)

    # Find extra rules (in translation but not in English)
    extra_rules = []
    for key, rule in translated_by_key.items():
        if key not in english_by_key:
            extra_rules.append(rule)

    # Find untranslated text in translated file
    untranslated_text = []
    for rule in translated_rules:
        if rule.has_untranslated_text:
            untranslated_text.append((rule, rule.untranslated_keys))

    return ComparisonResult(
        missing_rules=missing_rules,
        extra_rules=extra_rules,
        untranslated_text=untranslated_text,
        file_path=translated_path,
        english_rule_count=len(english_rules),
        translated_rule_count=len(translated_rules)
    )


def print_warnings(result: ComparisonResult, file_name: str) -> int:
    """Print warnings to console. Returns count of issues found."""
    issues = 0

    if result.missing_rules or result.untranslated_text or result.extra_rules:
        print(f"\n{'='*60}")
        print(f"File: {file_name}")
        print(f"English rules: {result.english_rule_count}, Translated rules: {result.translated_rule_count}")
        print(f"{'='*60}")

    if result.missing_rules:
        print(f"\n  MISSING RULES ({len(result.missing_rules)} rules in English but not in translation):")
        for rule in result.missing_rules:
            if rule.name:
                print(f"    - name: {rule.name}, tag: {rule.tag} (line {rule.line_number} in English)")
            else:
                print(f"    - key: \"{rule.key}\" (line {rule.line_number} in English)")
            issues += 1

    if result.untranslated_text:
        print(f"\n  UNTRANSLATED TEXT ({len(result.untranslated_text)} rules with lowercase t/ot/ct):")
        for rule, texts in result.untranslated_text:
            if rule.name:
                print(f"    - name: {rule.name}, tag: {rule.tag} (line {rule.line_number}):")
            else:
                print(f"    - key: \"{rule.key}\" (line {rule.line_number}):")
            for text in texts[:3]:  # Show first 3 examples
                print(f"        \"{text}\"")
            if len(texts) > 3:
                print(f"        ... and {len(texts) - 3} more")
            issues += 1

    if result.extra_rules:
        print(f"\n  EXTRA RULES ({len(result.extra_rules)} rules in translation but not in English):")
        print(f"  (These may be intentional language-specific rules)")
        for rule in result.extra_rules:
            if rule.name:
                print(f"    - name: {rule.name}, tag: {rule.tag} (line {rule.line_number})")
            else:
                print(f"    - key: \"{rule.key}\" (line {rule.line_number})")

    return issues


def get_yaml_files(lang_dir: Path) -> List[str]:
    """Get all YAML files to audit for a language"""
    files = []

    # Direct files in the language directory
    for f in lang_dir.glob("*.yaml"):
        if f.name != "prefs.yaml":  # Skip prefs.yaml as it's not translated
            files.append(str(f.relative_to(lang_dir)))

    # SharedRules subdirectory
    shared_dir = lang_dir / "SharedRules"
    if shared_dir.exists():
        for f in shared_dir.glob("*.yaml"):
            files.append(str(f.relative_to(lang_dir)))

    return sorted(files)


def audit_language(language: str, specific_file: Optional[str] = None):
    """Audit translations for a specific language"""
    rules_dir = get_rules_dir()
    english_dir = rules_dir / "en"
    translated_dir = rules_dir / language

    if not english_dir.exists():
        print(f"Error: English rules directory not found: {english_dir}")
        sys.exit(1)

    if not translated_dir.exists():
        print(f"Error: Translation directory not found: {translated_dir}")
        sys.exit(1)

    # Get list of files to audit
    if specific_file:
        files = [specific_file]
    else:
        files = get_yaml_files(english_dir)

    total_issues = 0
    total_missing = 0
    total_untranslated = 0
    total_extra = 0
    files_with_issues = 0

    print(f"\nAuditing {language} translations against English")
    print(f"Files to check: {len(files)}")

    for file_name in files:
        english_path = english_dir / file_name
        translated_path = translated_dir / file_name

        if not english_path.exists():
            print(f"Warning: English file not found: {english_path}")
            continue

        result = compare_files(str(english_path), str(translated_path))

        issues = print_warnings(result, file_name)
        if issues > 0:
            files_with_issues += 1
        total_issues += issues
        total_missing += len(result.missing_rules)
        total_untranslated += len(result.untranslated_text)
        total_extra += len(result.extra_rules)

    # Print summary
    print(f"\n{'='*60}")
    print("SUMMARY")
    print(f"{'='*60}")
    print(f"Files checked: {len(files)}")
    print(f"Files with issues: {files_with_issues}")
    print(f"Missing rules: {total_missing}")
    print(f"Rules with untranslated text: {total_untranslated}")
    print(f"Extra rules (translation only): {total_extra}")

    return total_issues


def list_languages():
    """List available languages for auditing"""
    rules_dir = get_rules_dir()

    print("\nAvailable languages:")
    for lang_dir in sorted(rules_dir.iterdir()):
        if lang_dir.is_dir() and lang_dir.name != "en":
            yaml_files = list(lang_dir.glob("*.yaml"))
            print(f"  {lang_dir.name}: {len(yaml_files)} YAML files")


def main():
    parser = argparse.ArgumentParser(
        description="Audit MathCAT translation files against English originals",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
    python audit-translations.py es
    python audit-translations.py de --file SharedRules/default.yaml
    python audit-translations.py --list
        """ # text to display after the argument help (https://docs.python.org/3/library/argparse.html#epilog)
    )

    parser.add_argument(
        "language",
        nargs="?",
        help="Language code to audit (e.g., 'es', 'de', 'fi')"
    )

    parser.add_argument(
        "--file",
        dest="specific_file",
        help="Audit only a specific file (e.g., 'SharedRules/default.yaml')"
    )

    parser.add_argument(
        "--list",
        action="store_true",
        help="List available languages"
    )

    args = parser.parse_args()

    if args.list:
        list_languages()
        return

    if not args.language:
        parser.print_help()
        print("\nError: Please specify a language code or use --list to see available languages")
        sys.exit(1)

    audit_language(args.language, args.specific_file)


if __name__ == "__main__":
    main()
