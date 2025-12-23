"""
Auditing and comparison logic.

Contains functions for comparing English and translated files,
and for performing full language audits.
"""

import os
import sys
from pathlib import Path
from typing import List, Optional

from .dataclasses import RuleInfo, ComparisonResult
from .parsers import parse_yaml_file
from . import ui


def get_rules_dir() -> Path:
    """Get the Rules/Languages directory path"""
    # Navigate from the package directory to the Rules directory
    package_dir = Path(__file__).parent
    return package_dir.parent.parent / "Rules" / "Languages"


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
    c = ui.Colors
    s = ui.Symbols
    issues = 0

    has_issues = result.missing_rules or result.untranslated_text or result.extra_rules

    if has_issues:
        ui.print_file_header(file_name, result.english_rule_count, result.translated_rule_count)

    if result.missing_rules:
        ui.print_issue_category(
            s.CROSS, c.RED,
            "Missing Rules",
            len(result.missing_rules),
            "in English but not in translation"
        )
        for rule in result.missing_rules:
            ui.print_rule_item(rule, context=" in English")
            issues += 1

    if result.untranslated_text:
        ui.print_issue_category(
            s.WARNING, c.YELLOW,
            "Untranslated Text",
            len(result.untranslated_text),
            "lowercase t/ot/ct keys"
        )
        for rule, texts in result.untranslated_text:
            ui.print_rule_item(rule)
            ui.print_text_samples(texts)
            issues += 1

    if result.extra_rules:
        ui.print_issue_category(
            s.INFO, c.BLUE,
            "Extra Rules",
            len(result.extra_rules),
            "may be intentional"
        )
        for rule in result.extra_rules:
            ui.print_rule_item(rule)

    return issues


def audit_language(language: str, specific_file: Optional[str] = None) -> int:
    """Audit translations for a specific language. Returns total issue count."""
    c = ui.Colors
    s = ui.Symbols

    rules_dir = get_rules_dir()
    english_dir = rules_dir / "en"
    translated_dir = rules_dir / language

    if not english_dir.exists():
        print(f"\n{c.RED}{s.CROSS} Error:{c.RESET} English rules directory not found: {english_dir}")
        sys.exit(1)

    if not translated_dir.exists():
        print(f"\n{c.RED}{s.CROSS} Error:{c.RESET} Translation directory not found: {translated_dir}")
        sys.exit(1)

    # Get list of files to audit
    if specific_file:
        files = [specific_file]
    else:
        files = get_yaml_files(english_dir)

    # Print header
    ui.print_header(f"MathCAT Translation Audit: {language.upper()}")
    print(f"\n  {c.DIM}Comparing against English (en) reference files{c.RESET}")
    print(f"  {c.DIM}Files to check: {len(files)}{c.RESET}")

    total_issues = 0
    total_missing = 0
    total_untranslated = 0
    total_extra = 0
    files_with_issues = 0
    files_ok = 0

    for file_name in files:
        english_path = english_dir / file_name
        translated_path = translated_dir / file_name

        if not english_path.exists():
            print(f"\n{c.YELLOW}{s.WARNING} Warning:{c.RESET} English file not found: {english_path}")
            continue

        result = compare_files(str(english_path), str(translated_path))

        has_issues = result.missing_rules or result.untranslated_text or result.extra_rules

        if has_issues:
            issues = print_warnings(result, file_name)
            if issues > 0:
                files_with_issues += 1
            total_issues += issues
        else:
            files_ok += 1

        total_missing += len(result.missing_rules)
        total_untranslated += len(result.untranslated_text)
        total_extra += len(result.extra_rules)

    # Determine overall status
    if total_missing == 0 and total_untranslated == 0:
        overall_status = (c.GREEN, s.CHECK, "All translations complete!")
    elif total_missing > 0 or total_untranslated > 0:
        overall_status = (c.YELLOW, s.WARNING, "Translation issues found")
    else:
        overall_status = (c.BLUE, s.INFO, "Review recommended")

    # Build summary stats
    stats = [
        ("Files checked", len(files), None),
        ("Files with issues", files_with_issues, c.YELLOW if files_with_issues > 0 else c.GREEN),
        ("Files OK", files_ok, c.GREEN if files_ok > 0 else None),
        ("", "", None),  # Spacer
        ("Missing rules", total_missing, c.RED if total_missing > 0 else c.GREEN),
        ("Untranslated text", total_untranslated, c.YELLOW if total_untranslated > 0 else c.GREEN),
        ("Extra rules", total_extra, c.BLUE if total_extra > 0 else None),
    ]

    ui.print_summary_box(stats)

    # Overall status
    status_color, status_icon, status_text = overall_status
    print(f"\n  {status_color}{status_icon} {status_text}{c.RESET}\n")

    return total_issues


def list_languages():
    """List available languages for auditing"""
    c = ui.Colors
    s = ui.Symbols

    rules_dir = get_rules_dir()

    ui.print_header("Available Languages")

    print(f"\n  {c.DIM}Language code │ YAML files{c.RESET}")
    print(f"  {c.DIM}{'─' * 14}┼{'─' * 15}{c.RESET}")

    for lang_dir in sorted(rules_dir.iterdir()):
        if lang_dir.is_dir() and lang_dir.name != "en":
            yaml_files = list(lang_dir.glob("*.yaml"))
            count = len(yaml_files)

            # Color based on file count
            if count >= 7:
                color = c.GREEN
            elif count >= 4:
                color = c.YELLOW
            else:
                color = c.RED

            print(f"  {c.CYAN}{lang_dir.name:^14}{c.RESET}│ {color}{count:>3}{c.RESET} files")

    print(f"\n  {c.DIM}Reference: en (English) - base translation{c.RESET}\n")
