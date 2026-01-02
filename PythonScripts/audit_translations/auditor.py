"""
Auditing and comparison logic.

Contains functions for comparing English and translated files,
and for performing full language audits.
"""

import csv
import json
import os
import sys
from pathlib import Path
from typing import Iterable, List, Optional, TextIO

from rich.console import Console
from rich.markup import escape
from rich.panel import Panel
from rich.table import Table

from .dataclasses import RuleInfo, RuleDifference, ComparisonResult
from .parsers import parse_yaml_file, diff_rules
console = Console()

ISSUE_FIELDS = [
    "language",
    "file",
    "issue_type",
    "diff_type",
    "rule_name",
    "rule_tag",
    "rule_key",
    "line_en",
    "line_tr",
    "description",
    "english_snippet",
    "translated_snippet",
    "untranslated_texts",
]


def get_rules_dir(rules_dir: Optional[str] = None) -> Path:
    """Get the Rules/Languages directory path"""
    if rules_dir:
        return Path(rules_dir).expanduser()
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


def compare_files(
    english_path: str,
    translated_path: str,
    issue_filter: Optional[set[str]] = None,
) -> ComparisonResult:
    """Compare English and translated YAML files"""

    english_rules, _ = parse_yaml_file(english_path)

    if os.path.exists(translated_path):
        translated_rules, _ = parse_yaml_file(translated_path)
    else:
        translated_rules = []

    # Create lookup dictionaries
    english_by_key = {r.key: r for r in english_rules}
    translated_by_key = {r.key: r for r in translated_rules}

    include_all = issue_filter is None
    include_missing = include_all or "missing" in issue_filter
    include_untranslated = include_all or "untranslated" in issue_filter
    include_extra = include_all or "extra" in issue_filter
    include_diffs = include_all or "diffs" in issue_filter

    # Find missing rules (in English but not in translation)
    missing_rules = []
    if include_missing:
        for key, rule in english_by_key.items():
            if key not in translated_by_key:
                missing_rules.append(rule)

    # Find extra rules (in translation but not in English)
    extra_rules = []
    if include_extra:
        for key, rule in translated_by_key.items():
            if key not in english_by_key:
                extra_rules.append(rule)

    # Find untranslated text in translated file (skip if audit-ignore)
    untranslated_text = []
    if include_untranslated:
        for rule in translated_rules:
            if rule.has_untranslated_text and not rule.audit_ignore:
                untranslated_text.append((rule, rule.untranslated_keys))

    # Find fine-grained differences in rules that exist in both files (skip if audit-ignore)
    rule_differences = []
    if include_diffs:
        for key, en_rule in english_by_key.items():
            if key in translated_by_key:
                tr_rule = translated_by_key[key]
                if not tr_rule.audit_ignore:
                    diffs = diff_rules(en_rule, tr_rule)
                    rule_differences.extend(diffs)

    return ComparisonResult(
        missing_rules=missing_rules,
        extra_rules=extra_rules,
        untranslated_text=untranslated_text,
        rule_differences=rule_differences,
        file_path=translated_path,
        english_rule_count=len(english_rules),
        translated_rule_count=len(translated_rules)
    )


def print_rule_item(rule: RuleInfo, context: str = ""):
    if rule.name is None:
        console.print(f"      [dim]•[/] [yellow]\"{escape(rule.key)}\"[/] [dim](line {rule.line_number}{context})[/]")
    else:
        tag = rule.tag or "unknown"
        console.print(f"      [dim]•[/] [cyan]{escape(rule.name)}[/] [dim][{escape(tag)}][/] [dim](line {rule.line_number}{context})[/]")


def print_diff_item(diff: RuleDifference):
    """Print a single rule difference"""
    rule = diff.english_rule
    if rule.name is None:
        console.print(f"      [dim]•[/] [yellow]\"{escape(rule.key)}\"[/] [dim](line {rule.line_number} en, {diff.translated_rule.line_number} tr)[/]")
    else:
        tag = rule.tag or "unknown"
        console.print(f"      [dim]•[/] [cyan]{escape(rule.name)}[/] [dim][{escape(tag)}][/] [dim](line {rule.line_number} en, {diff.translated_rule.line_number} tr)[/]")
    console.print(f"          [dim]{diff.description}:[/]")
    console.print(f"          [green]en:[/] {escape(diff.english_snippet)}")
    console.print(f"          [red]tr:[/] {escape(diff.translated_snippet)}")


def issue_base(rule: RuleInfo, file_name: str, language: str) -> dict:
    return {
        "language": language,
        "file": file_name,
        "rule_name": rule.name or "",
        "rule_tag": rule.tag or "",
        "rule_key": rule.key,
        "line_en": None,
        "line_tr": None,
    }


def collect_issues(
    result: ComparisonResult,
    file_name: str,
    language: str,
    include_raw: bool = False,
) -> List[dict]:
    issues = []

    for rule in result.missing_rules:
        issue = issue_base(rule, file_name, language)
        issue.update(
            issue_type="missing_rule",
            diff_type="",
            line_en=rule.line_number,
            description="Rule present in English but missing in translation",
            english_snippet="",
            translated_snippet="",
            untranslated_texts=[],
        )
        if include_raw:
            issue["english_raw"] = rule.raw_content
        issues.append(issue)

    for rule in result.extra_rules:
        issue = issue_base(rule, file_name, language)
        issue.update(
            issue_type="extra_rule",
            diff_type="",
            line_tr=rule.line_number,
            description="Rule present in translation but missing in English",
            english_snippet="",
            translated_snippet="",
            untranslated_texts=[],
        )
        if include_raw:
            issue["translated_raw"] = rule.raw_content
        issues.append(issue)

    for rule, texts in result.untranslated_text:
        issue = issue_base(rule, file_name, language)
        issue.update(
            issue_type="untranslated_text",
            diff_type="",
            line_tr=rule.line_number,
            description="Lowercase t/ot/ct keys indicate untranslated text",
            english_snippet="",
            translated_snippet="",
            untranslated_texts=texts,
        )
        if include_raw:
            issue["translated_raw"] = rule.raw_content
        issues.append(issue)

    for diff in result.rule_differences:
        rule = diff.english_rule
        issue = issue_base(rule, file_name, language)
        issue.update(
            issue_type="rule_difference",
            diff_type=diff.diff_type,
            line_en=diff.english_rule.line_number,
            line_tr=diff.translated_rule.line_number,
            description=diff.description,
            english_snippet=diff.english_snippet,
            translated_snippet=diff.translated_snippet,
            untranslated_texts=[],
        )
        if include_raw:
            issue["english_raw"] = diff.english_rule.raw_content
            issue["translated_raw"] = diff.translated_rule.raw_content
        issues.append(issue)

    return issues


class IssueWriter:
    def __init__(self, output_format: str, stream: TextIO):
        self.output_format = output_format
        self.stream = stream
        self.csv_writer: Optional[csv.DictWriter] = None
        if output_format == "csv":
            self.csv_writer = csv.DictWriter(stream, fieldnames=ISSUE_FIELDS)
            self.csv_writer.writeheader()

    def write(self, issue: dict) -> None:
        if self.output_format in ("jsonl", "tasks"):
            self.stream.write(json.dumps(issue, ensure_ascii=False) + "\n")
        elif self.output_format == "csv" and self.csv_writer is not None:
            row = issue.copy()
            if row.get("untranslated_texts"):
                row["untranslated_texts"] = "|".join(row["untranslated_texts"])
            else:
                row["untranslated_texts"] = ""
            row["line_en"] = "" if row.get("line_en") is None else row["line_en"]
            row["line_tr"] = "" if row.get("line_tr") is None else row["line_tr"]
            self.csv_writer.writerow(row)


def print_warnings(result: ComparisonResult, file_name: str) -> int:
    """Print warnings to console. Returns count of issues found."""
    issues = 0

    has_issues = result.missing_rules or result.untranslated_text or result.extra_rules or result.rule_differences

    if has_issues:
        # File header
        style, icon = ("green", "✓") if result.translated_rule_count == result.english_rule_count else \
                      ("red", "✗") if result.translated_rule_count == 0 else ("yellow", "⚠")
        console.print()
        console.rule(style="cyan")
        console.print(f"[{style}]{icon}[/] [bold]{escape(file_name)}[/]")
        console.print(f"  [dim]English: {result.english_rule_count} rules  →  Translated: {result.translated_rule_count} rules[/]")
        console.rule(style="cyan")

    if result.missing_rules:
        console.print(f"\n  [red]✗[/] [bold]Missing Rules[/] [[red]{len(result.missing_rules)}[/]] [dim](in English but not in translation)[/]")
        for rule in result.missing_rules:
            print_rule_item(rule, context=" in English")
            issues += 1

    if result.untranslated_text:
        console.print(f"\n  [yellow]⚠[/] [bold]Untranslated Text[/] [[yellow]{len(result.untranslated_text)}[/]] [dim](lowercase t/ot/ct keys)[/]")
        for rule, texts in result.untranslated_text:
            print_rule_item(rule)
            for text in texts:
                console.print(f"          [dim]→[/] [yellow]\"{escape(text)}\"[/]")
            issues += 1

    if result.rule_differences:
        # Group differences by type for clearer output
        by_type: dict[str, list[RuleDifference]] = {}
        for diff in result.rule_differences:
            by_type.setdefault(diff.diff_type, []).append(diff)

        total_diffs = len(result.rule_differences)
        console.print(f"\n  [magenta]≠[/] [bold]Rule Differences[/] [[magenta]{total_diffs}[/]] [dim](structural differences between en and translation)[/]")

        type_labels = {
            'match': 'Match Pattern',
            'condition': 'Conditions',
            'structure': 'Structure',
            'variables': 'Variables'
        }
        for diff_type, diffs in by_type.items():
            label = type_labels.get(diff_type, diff_type)
            console.print(f"\n    [dim]{label} ({len(diffs)}):[/]")
            for diff in diffs:
                print_diff_item(diff)
                issues += 1

    if result.extra_rules:
        console.print(f"\n  [blue]ℹ[/] [bold]Extra Rules[/] [[blue]{len(result.extra_rules)}[/]] [dim](may be intentional)[/]")
        for rule in result.extra_rules:
            print_rule_item(rule)
            issues += 1

    return issues


def audit_language(
    language: str,
    specific_file: Optional[str] = None,
    output_format: str = "rich",
    output_path: Optional[str] = None,
    rules_dir: Optional[str] = None,
    issue_filter: Optional[set[str]] = None,
) -> int:
    """Audit translations for a specific language. Returns total issue count."""
    rules_dir_path = get_rules_dir(rules_dir)
    english_dir = rules_dir_path / "en"
    translated_dir = rules_dir_path / language

    if not english_dir.exists():
        console.print(f"\n[red]✗ Error:[/] English rules directory not found: {english_dir}")
        sys.exit(1)

    if not translated_dir.exists():
        console.print(f"\n[red]✗ Error:[/] Translation directory not found: {translated_dir}")
        sys.exit(1)

    # Get list of files to audit
    files = [specific_file] if specific_file else get_yaml_files(english_dir)

    if output_format == "rich":
        # Print header
        console.print(Panel(f"MathCAT Translation Audit: {language.upper()}", style="bold cyan"))
        console.print(f"\n  [dim]Comparing against English (en) reference files[/]")
        console.print(f"  [dim]Files to check: {len(files)}[/]")

    out_stream: TextIO = sys.stdout
    if output_path:
        out_stream = open(output_path, "w", encoding="utf-8", newline="")

    writer = IssueWriter(output_format, out_stream) if output_format != "rich" else None

    total_issues = 0
    total_missing = 0
    total_untranslated = 0
    total_extra = 0
    total_differences = 0
    files_with_issues = 0
    files_ok = 0

    for file_name in files:
        english_path = english_dir / file_name
        translated_path = translated_dir / file_name

        if not english_path.exists():
            console.print(f"\n[yellow]⚠ Warning:[/] English file not found: {english_path}")
            continue

        result = compare_files(str(english_path), str(translated_path), issue_filter)

        # check for issues
        has_issues = result.missing_rules or result.untranslated_text or result.extra_rules or result.rule_differences
        if output_format == "rich":
            if has_issues:
                issues = print_warnings(result, file_name)
                if issues > 0:
                    files_with_issues += 1
                total_issues += issues
            else:
                files_ok += 1
        else:
            include_raw = output_format == "tasks"
            issues_list = collect_issues(result, file_name, language, include_raw=include_raw)
            for issue in issues_list:
                writer.write(issue)
            if issues_list:
                files_with_issues += 1
                total_issues += len(issues_list)
            else:
                files_ok += 1

        total_missing += len(result.missing_rules)
        total_untranslated += len(result.untranslated_text)
        total_extra += len(result.extra_rules)
        total_differences += len(result.rule_differences)

    if output_format == "rich":
        # Summary
        table = Table(title="SUMMARY", title_style="bold", box=None, show_header=False, padding=(0, 2))
        table.add_column(width=30)
        table.add_column()
        for label, value, color in [
            ("Files checked", len(files), None),
            ("Files with issues", files_with_issues, "yellow" if files_with_issues else "green"),
            ("Files OK", files_ok, "green" if files_ok else None),
            ("Missing rules", total_missing, "red" if total_missing else "green"),
            ("Untranslated text", total_untranslated, "yellow" if total_untranslated else "green"),
            ("Rule differences", total_differences, "magenta" if total_differences else "green"),
            ("Extra rules", total_extra, "blue" if total_extra else None),
        ]:
            table.add_row(label, f"[{color}]{value}[/]" if color else str(value))
        console.print(Panel(table, style="cyan"))

    if output_path:
        out_stream.close()
    return total_issues


def list_languages(rules_dir: Optional[str] = None):
    """List available languages for auditing"""
    console.print(Panel("Available Languages", style="bold cyan"))

    table = Table(show_header=True, header_style="dim")
    table.add_column("Language", justify="center", style="cyan")
    table.add_column("YAML files", justify="right")

    for lang_dir in sorted(get_rules_dir(rules_dir).iterdir()):
        if lang_dir.is_dir() and lang_dir.name != "en":
            count = len(get_yaml_files(lang_dir))
            color = "green" if count >= 7 else "yellow" if count >= 4 else "red"
            table.add_row(lang_dir.name, f"[{color}]{count}[/] files")

    console.print(table)
    console.print("\n  [dim]Reference: en (English) - base translation[/]\n")
