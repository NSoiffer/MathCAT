"""
Auditing and comparison logic.

Contains functions for comparing English and translated files,
and for performing full language audits.
"""

import json
import os
import sys
from pathlib import Path
from typing import Iterable, List, Optional, TextIO, Tuple

from rich.console import Console
from rich.markup import escape
from rich.panel import Panel
from rich.table import Table

from .dataclasses import RuleInfo, RuleDifference, ComparisonResult
from .parsers import parse_yaml_file, diff_rules, extract_structure_elements
console = Console()


def normalize_language(language: str) -> str:
    """Return a normalized language code (lowercase, '-' separators)."""
    return language.lower().replace("_", "-")


def split_language(language: str) -> Tuple[str, Optional[str]]:
    """Split a language code into base and optional region."""
    normalized = normalize_language(language)
    if "-" in normalized:
        base, region = normalized.split("-", 1)
        return base, region or None
    return normalized, None


def get_rules_dir(rules_dir: Optional[str] = None) -> Path:
    """Get the Rules/Languages directory path"""
    if rules_dir:
        return Path(rules_dir).expanduser()
    # Navigate from the package directory to the Rules directory
    package_dir = Path(__file__).parent
    return package_dir.parent.parent / "Rules" / "Languages"


def get_yaml_files(lang_dir: Path, region_dir: Optional[Path] = None) -> List[str]:
    """Get all YAML files to audit for a language, including region overrides."""
    files: set[str] = set()

    def collect_from(directory: Path, root: Path) -> None:
        if not directory.exists():
            return
        for f in directory.glob("*.yaml"):
            if f.name != "prefs.yaml":  # Skip prefs.yaml as it's not translated
                files.add(str(f.relative_to(root)))
        shared_dir = directory / "SharedRules"
        if shared_dir.exists():
            for f in shared_dir.glob("*.yaml"):
                files.add(str(f.relative_to(root)))

    collect_from(lang_dir, lang_dir)
    if region_dir:
        collect_from(region_dir, region_dir)

    return sorted(files)


def compare_files(
    english_path: str,
    translated_path: str,
    issue_filter: Optional[set[str]] = None,
    translated_region_path: Optional[str] = None,
    english_region_path: Optional[str] = None,
) -> ComparisonResult:
    """Compare English and translated YAML files"""

    def load_rules(path: Optional[str]) -> List[RuleInfo]:
        if path and os.path.exists(path):
            rules, _ = parse_yaml_file(path)
            return rules
        return []

    def merge_rules(base_rules: List[RuleInfo], region_rules: List[RuleInfo]) -> List[RuleInfo]:
        if not region_rules:
            return base_rules
        merged = {r.key: r for r in base_rules}
        for rule in region_rules:
            merged[rule.key] = rule
        return list(merged.values())

    english_rules = merge_rules(
        load_rules(english_path),
        load_rules(english_region_path),
    )
    translated_rules = merge_rules(
        load_rules(translated_path),
        load_rules(translated_region_path),
    )

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
                untranslated_text.append((rule, rule.untranslated_entries))

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


def rule_label(rule: RuleInfo) -> str:
    if rule.name is None:
        return f"[yellow]\"{escape(rule.key)}\"[/]"
    tag = rule.tag or "unknown"
    return f"[cyan]{escape(rule.name)}[/] [dim][{escape(tag)}][/]"


def print_rule_item(rule: RuleInfo, issue_line: int, context: str = ""):
    console.print(f"      [dim]•[/] {rule_label(rule)} [dim](line {issue_line}{context})[/]")


def print_diff_item(diff: RuleDifference, line_en: int, line_tr: int, verbose: bool = False):
    """Print a single rule difference"""
    rule = diff.english_rule
    console.print(
        f"      [dim]•[/] {rule_label(rule)} "
        f"[dim](line {line_en} en, {line_tr} tr)[/]"
    )
    console.print(f"          [dim]{diff.description}[/]")
    if verbose:
        console.print(f"          [green]en:[/] {escape(diff.english_snippet)}")
        console.print(f"          [red]tr:[/] {escape(diff.translated_snippet)}")


def issue_base(rule: RuleInfo, file_name: str, language: str) -> dict:
    return {
        "language": language,
        "file": file_name,
        "rule_name": rule.name or "",
        "rule_tag": rule.tag or "",
        "rule_key": rule.key,
        "issue_line_en": None,
        "issue_line_tr": None,
        "rule_line_en": None,
        "rule_line_tr": None,
    }


def first_structure_mismatch(
    english_tokens: List[str],
    translated_tokens: List[str],
) -> Tuple[Optional[str], Optional[str], int]:
    """
    Find the first structural mismatch between two token lists.

    Returns (en_token, tr_token, mismatch_position).
    Position is the index in the token list where they first differ.
    """
    min_len = min(len(english_tokens), len(translated_tokens))
    for idx in range(min_len):
        if english_tokens[idx] != translated_tokens[idx]:
            return english_tokens[idx], translated_tokens[idx], idx
    if len(english_tokens) > min_len:
        return english_tokens[min_len], None, min_len
    if len(translated_tokens) > min_len:
        return None, translated_tokens[min_len], min_len
    return None, None, -1


def resolve_issue_line_at_position(
    rule: RuleInfo,
    kind: str,
    token: Optional[str] = None,
    position: int = 0
) -> Optional[int]:
    """
    Resolve the line number for a specific occurrence of an element within a rule.

    Args:
        rule: The rule to search in
        kind: The kind of element ('match', 'condition', 'variables', 'structure')
        token: For structure kind, the specific token to find
        position: The occurrence index (0 for first, 1 for second, etc.)

    Returns:
        The line number if found, None if the element doesn't exist at that position.
    """
    if kind == "match":
        lines = rule.line_map.get("match", [])
    elif kind == "condition":
        lines = rule.line_map.get("condition", [])
    elif kind == "variables":
        lines = rule.line_map.get("variables", [])
    elif kind == "structure" and token:
        token_key = f"structure:{token.rstrip(':')}"
        lines = rule.line_map.get(token_key, [])
    else:
        lines = []

    if position < len(lines):
        return lines[position]
    return None


def resolve_issue_line(rule: RuleInfo, kind: str, token: Optional[str] = None) -> Optional[int]:
    """
    Resolve the line number for an issue within a rule.

    Returns the line number if found, None if the element doesn't exist in the rule.
    For 'structure' kind with a missing token, returns None instead of falling back
    to rule.line_number to avoid misleading line numbers when elements are missing.
    """
    if kind == "match":
        lines = rule.line_map.get("match", [])
    elif kind == "condition":
        lines = rule.line_map.get("condition", [])
    elif kind == "variables":
        lines = rule.line_map.get("variables", [])
    elif kind == "structure" and token:
        token_key = f"structure:{token.rstrip(':')}"
        lines = rule.line_map.get(token_key, [])
        # For structure differences, if the token doesn't exist, return None
        # rather than falling back to rule.line_number which is misleading
        return lines[0] if lines else None
    else:
        lines = []
    return lines[0] if lines else rule.line_number


def collect_issues(
    result: ComparisonResult,
    file_name: str,
    language: str,
) -> List[dict]:
    issues = []

    for rule in result.missing_rules:
        issue = issue_base(rule, file_name, language)
        issue.update(
            issue_type="missing_rule",
            diff_type="",
            issue_line_en=rule.line_number,
            rule_line_en=rule.line_number,
            description="Rule present in English but missing in translation",
            english_snippet="",
            translated_snippet="",
            untranslated_texts=[],
        )
        issues.append(issue)

    for rule in result.extra_rules:
        issue = issue_base(rule, file_name, language)
        issue.update(
            issue_type="extra_rule",
            diff_type="",
            issue_line_tr=rule.line_number,
            rule_line_tr=rule.line_number,
            description="Rule present in translation but missing in English",
            english_snippet="",
            translated_snippet="",
            untranslated_texts=[],
        )
        issues.append(issue)

    for rule, entries in result.untranslated_text:
        for key, text, line in entries:
            issue = issue_base(rule, file_name, language)
            issue.update(
                issue_type="untranslated_text",
                diff_type="",
                issue_line_tr=line or rule.line_number,
                rule_line_tr=rule.line_number,
                description="Lowercase t/ot/ct keys indicate untranslated text",
                english_snippet="",
                translated_snippet="",
                untranslated_texts=[text],
            )
            issues.append(issue)

    for diff in result.rule_differences:
        rule = diff.english_rule
        issue = issue_base(rule, file_name, language)
        if diff.diff_type == "structure":
            en_tokens = extract_structure_elements(diff.english_rule.data)
            tr_tokens = extract_structure_elements(diff.translated_rule.data)
            en_token, tr_token, mismatch_pos = first_structure_mismatch(en_tokens, tr_tokens)

            # Skip reporting when tokens are misaligned (both exist but differ)
            # This avoids misleading line numbers when entire blocks are missing/added
            # We only report when one is None (clear case of missing element)
            if en_token is not None and tr_token is not None and en_token != tr_token:
                continue

            issue_line_en = resolve_issue_line(diff.english_rule, "structure", en_token)
            issue_line_tr = resolve_issue_line(diff.translated_rule, "structure", tr_token)

            # Skip reporting structure differences where we can't find both tokens
            # This avoids misleading line numbers when blocks are missing
            if issue_line_en is None or issue_line_tr is None:
                continue
        else:
            issue_line_en = resolve_issue_line(diff.english_rule, diff.diff_type)
            issue_line_tr = resolve_issue_line(diff.translated_rule, diff.diff_type)
        issue.update(
            issue_type="rule_difference",
            diff_type=diff.diff_type,
            issue_line_en=issue_line_en,
            issue_line_tr=issue_line_tr,
            rule_line_en=diff.english_rule.line_number,
            rule_line_tr=diff.translated_rule.line_number,
            description=diff.description,
            english_snippet=diff.english_snippet,
            translated_snippet=diff.translated_snippet,
            untranslated_texts=[],
        )
        issues.append(issue)

    return issues


class IssueWriter:
    def __init__(self, output_format: str, stream: TextIO):
        if output_format != "jsonl":
            raise ValueError(f"Unsupported output format: {output_format}")
        self.stream = stream

    def write(self, issue: dict) -> None:
        self.stream.write(json.dumps(issue, ensure_ascii=False) + "\n")


def print_warnings(result: ComparisonResult, file_name: str, verbose: bool = False) -> int:
    """Print warnings to console. Returns count of issues found."""
    issues = 0

    has_issues = result.missing_rules or result.untranslated_text or result.extra_rules or result.rule_differences
    if not has_issues:
        return issues

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
            print_rule_item(rule, issue_line=rule.line_number, context=" in English")
            issues += 1

    if result.untranslated_text:
        untranslated_count = sum(len(entries) for _, entries in result.untranslated_text)
        console.print(f"\n  [yellow]⚠[/] [bold]Untranslated Text[/] [[yellow]{untranslated_count}[/]] [dim](lowercase t/ot/ct keys)[/]")
        for rule, entries in result.untranslated_text:
            for _, text, line in entries:
                issue_line = line or rule.line_number
                print_rule_item(rule, issue_line=issue_line)
                console.print(f"          [dim]→[/] [yellow]\"{escape(text)}\"[/]")
                issues += 1

    if result.rule_differences:
        # Count only diffs that will actually be displayed
        displayable_diffs = []
        for diff in result.rule_differences:
            if diff.diff_type == "structure":
                en_tokens = extract_structure_elements(diff.english_rule.data)
                tr_tokens = extract_structure_elements(diff.translated_rule.data)
                en_token, tr_token, mismatch_pos = first_structure_mismatch(en_tokens, tr_tokens)

                # Skip reporting when tokens are misaligned (both exist but differ)
                # This avoids misleading line numbers when entire blocks are missing/added
                if en_token is not None and tr_token is not None and en_token != tr_token:
                    continue

                line_en = resolve_issue_line(diff.english_rule, "structure", en_token)
                line_tr = resolve_issue_line(diff.translated_rule, "structure", tr_token)
                # Skip structure diffs where we can't find both tokens
                if line_en is None or line_tr is None:
                    continue
            else:
                line_en = resolve_issue_line(diff.english_rule, diff.diff_type)
                line_tr = resolve_issue_line(diff.translated_rule, diff.diff_type)
            displayable_diffs.append((diff, line_en, line_tr))

        if displayable_diffs:
            console.print(
                f"\n  [magenta]≠[/] [bold]Rule Differences[/] "
                f"[[magenta]{len(displayable_diffs)}[/]] [dim](structural differences between en and translation)[/]"
            )
            for diff, line_en, line_tr in displayable_diffs:
                print_diff_item(diff, line_en=line_en, line_tr=line_tr, verbose=verbose)
                issues += 1

    if result.extra_rules:
        console.print(f"\n  [blue]ℹ[/] [bold]Extra Rules[/] [[blue]{len(result.extra_rules)}[/]] [dim](may be intentional)[/]")
        for rule in result.extra_rules:
            print_rule_item(rule, issue_line=rule.line_number)
            issues += 1

    return issues


def audit_language(
    language: str,
    specific_file: Optional[str] = None,
    output_format: str = "rich",
    output_path: Optional[str] = None,
    rules_dir: Optional[str] = None,
    issue_filter: Optional[set[str]] = None,
    verbose: bool = False,
) -> int:
    """Audit translations for a specific language. Returns total issue count."""
    rules_dir_path = get_rules_dir(rules_dir)
    english_dir = rules_dir_path / "en"

    base_language, region = split_language(language)
    translated_dir = rules_dir_path / base_language
    translated_region_dir = translated_dir / region if region else None
    english_region_dir = english_dir / region if region else None

    if not english_dir.exists():
        console.print(f"\n[red]✗ Error:[/] English rules directory not found: {english_dir}")
        sys.exit(1)

    if not translated_dir.exists():
        console.print(f"\n[red]✗ Error:[/] Translation directory not found: {translated_dir}")
        sys.exit(1)

    if region and not (translated_region_dir and translated_region_dir.exists()):
        console.print(f"\n[red]✗ Error:[/] Region directory not found: {translated_region_dir}")
        sys.exit(1)

    # Get list of files to audit
    files = [specific_file] if specific_file else get_yaml_files(english_dir, english_region_dir)

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
        translated_region_path = translated_region_dir / file_name if translated_region_dir else None
        english_region_path = english_region_dir / file_name if english_region_dir else None

        if not english_path.exists():
            console.print(f"\n[yellow]⚠ Warning:[/] English file not found: {english_path}")
            continue

        result = compare_files(
            str(english_path),
            str(translated_path),
            issue_filter,
            str(translated_region_path) if translated_region_path and translated_region_path.exists() else None,
            str(english_region_path) if english_region_path and english_region_path.exists() else None,
        )

        # check for issues
        has_issues = result.missing_rules or result.untranslated_text or result.extra_rules or result.rule_differences
        if output_format == "rich":
            if has_issues:
                issues = print_warnings(result, file_name, verbose)
                if issues > 0:
                    files_with_issues += 1
                total_issues += issues
            else:
                files_ok += 1
        else:
            issues_list = collect_issues(result, file_name, language)
            for issue in issues_list:
                writer.write(issue)
            if issues_list:
                files_with_issues += 1
                total_issues += len(issues_list)
            else:
                files_ok += 1

        total_missing += len(result.missing_rules)
        total_untranslated += sum(len(entries) for _, entries in result.untranslated_text)
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

    rules_dir_path = get_rules_dir(rules_dir)
    for lang_dir in sorted(rules_dir_path.iterdir()):
        if not lang_dir.is_dir() or lang_dir.name == "en":
            continue
        base_count = len(get_yaml_files(lang_dir))
        color = "green" if base_count >= 7 else "yellow" if base_count >= 4 else "red"
        table.add_row(lang_dir.name, f"[{color}]{base_count}[/] files")

        for region_dir in sorted(lang_dir.iterdir()):
            if region_dir.is_dir():
                code = f"{lang_dir.name}-{region_dir.name}"
                count = len(get_yaml_files(lang_dir, region_dir))
                region_color = "green" if count >= 7 else "yellow" if count >= 4 else "red"
                table.add_row(code, f"[{region_color}]{count}[/] files")

    console.print(table)
    console.print("\n  [dim]Reference: en (English) - base translation[/]\n")
