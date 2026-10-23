"""
Auditing and comparison logic.

Contains functions for comparing source and translated files,
and for performing full language audits.
"""

from pathlib import Path

from .differ import diff_rules
from .errors import AuditError
from .models.audit import AuditSummary
from .models.definitions import DefinitionComparisonResult, DefinitionInfo, DefinitionTypeMismatch
from .models.rules import ComparisonResult, RuleInfo
from .parsers import parse_definitions_file, parse_yaml_file
from .renderer import (
    console,
    print_audit_header,
    print_audit_summary,
    print_definition_findings,
    print_language_list,
    print_order_findings,
    print_warnings,
)


def split_language_into_base_and_region(language: str) -> tuple[str, str | None]:
    """Split a language code into base and optional region."""
    normalized = language.lower().replace("_", "-")
    if "-" in normalized:
        base, region = normalized.split("-", 1)
        return base, region or None
    return normalized, None


def get_rules_dir(rules_dir: str | None = None) -> Path:
    """Get the Rules/Languages directory path"""
    if rules_dir:
        return Path(rules_dir).expanduser()
    # Navigate from the package directory to the Rules directory
    package_dir = Path(__file__).parent
    return package_dir.parent.parent / "Rules" / "Languages"


def is_definitions_file(file_path: str | Path) -> bool:
    """Return whether a file needs the dedicated definitions audit path."""
    return Path(file_path).name == "definitions.yaml"


def get_yaml_files(lang_dir: Path, region_dir: Path | None = None) -> list[Path]:
    """Get all YAML files to audit for a language, including region overrides."""
    files: set[Path] = set()

    def collect_from(directory: Path, root: Path) -> None:
        if not directory.exists():
            return
        for f in directory.glob("*.yaml"):
            if f.name != "prefs.yaml":
                files.add(f.relative_to(root))
        shared_dir = directory / "SharedRules"
        if shared_dir.exists():
            for f in shared_dir.glob("*.yaml"):
                files.add(f.relative_to(root))

    collect_from(lang_dir, lang_dir)
    if region_dir:
        collect_from(region_dir, region_dir)

    # sort by path parts (case-sensitive) so the order is the same on every platform;
    # Windows paths compare case-insensitively
    return sorted(files, key=lambda p: p.parts)


def compare_files(
    english_path: Path,
    translated_path: Path,
    issue_filter: set[str] | None = None,
    translated_region_path: Path | None = None,
    english_region_path: Path | None = None,
) -> ComparisonResult:
    """Compare source and translated YAML files"""

    def load_rules(path: Path | None) -> list[RuleInfo]:
        if path and path.exists():
            rules, _ = parse_yaml_file(path)
            return rules
        return []

    def merge_rules(base_rules: list[RuleInfo], region_rules: list[RuleInfo]) -> list[RuleInfo]:
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
    include_order = include_all

    first_order_mismatch_position = None
    if include_order:
        source_keys = {rule.key for rule in english_rules if rule.name is not None and not rule.audit_ignore}
        target_keys = {rule.key for rule in translated_rules if rule.name is not None and not rule.audit_ignore}
        shared_keys = source_keys & target_keys
        source_order = [rule for rule in english_rules if rule.key in shared_keys and not rule.audit_ignore]
        target_order = [rule for rule in translated_rules if rule.key in shared_keys and not rule.audit_ignore]
        for position, (source_rule, target_rule) in enumerate(zip(source_order, target_order, strict=True), start=1):
            if source_rule.key != target_rule.key:
                first_order_mismatch_position = position
                break

    # Find missing rules (in source but not in translation)
    missing_rules = []
    if include_missing:
        for key, rule in english_by_key.items():
            if key not in translated_by_key:
                missing_rules.append(rule)

    # Find extra rules (in translation but not in source)
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
        first_order_mismatch_position=first_order_mismatch_position,
        english_rule_count=len(english_rules),
        translated_rule_count=len(translated_rules),
    )


def compare_definition_files(
    source_path: Path,
    target_path: Path,
    issue_filter: set[str] | None = None,
    target_region_path: Path | None = None,
    source_region_path: Path | None = None,
) -> DefinitionComparisonResult:
    """Compare literal definitions by name and collection kind."""

    def load_definitions(path: Path | None) -> dict[str, DefinitionInfo]:
        if path and path.exists():
            definitions, _ = parse_definitions_file(path)
            return definitions
        return {}

    def merge_definitions(
        base_definitions: dict[str, DefinitionInfo],
        region_definitions: dict[str, DefinitionInfo],
    ) -> dict[str, DefinitionInfo]:
        merged = dict(base_definitions)
        merged.update(region_definitions)
        return merged

    source_definitions = merge_definitions(
        load_definitions(source_path),
        load_definitions(source_region_path),
    )
    target_definitions = merge_definitions(
        load_definitions(target_path),
        load_definitions(target_region_path),
    )

    include_all = issue_filter is None
    include_missing = include_all or "missing" in issue_filter
    include_extra = include_all or "extra" in issue_filter
    include_diffs = include_all or "diffs" in issue_filter

    missing_definitions = (
        [definition for name, definition in source_definitions.items() if name not in target_definitions]
        if include_missing
        else []
    )
    extra_definitions = (
        [definition for name, definition in target_definitions.items() if name not in source_definitions] if include_extra else []
    )
    type_mismatches = []
    if include_diffs:
        for name, source_definition in source_definitions.items():
            target_definition = target_definitions.get(name)
            if target_definition and source_definition.kind is not target_definition.kind:
                type_mismatches.append(DefinitionTypeMismatch(source_definition, target_definition))

    return DefinitionComparisonResult(
        missing_definitions=missing_definitions,
        extra_definitions=extra_definitions,
        type_mismatches=type_mismatches,
        source_definition_count=len(source_definitions),
        target_definition_count=len(target_definitions),
    )


def audit_language(
    language: str,
    specific_file: str | None = None,
    rules_dir: str | None = None,
    issue_filter: set[str] | None = None,
    verbose: bool = False,
    source_language: str = "en",
) -> int:
    """Audit translations for a specific language and return the total issue count.

    ``specific_file`` is the relative file path supplied by the CLI's ``--file``
    option. When set, the audit is limited to that file.
    """
    rules_dir_path = get_rules_dir(rules_dir)

    source_base_language, source_region = split_language_into_base_and_region(source_language)
    source_dir = rules_dir_path / source_base_language
    source_region_dir = source_dir / source_region if source_region else None

    target_base_language, target_region = split_language_into_base_and_region(language)
    translated_dir = rules_dir_path / target_base_language
    translated_region_dir = translated_dir / target_region if target_region else None

    if not source_dir.exists():
        raise AuditError(f"Source rules directory not found: {source_dir}")

    if source_region and not (source_region_dir and source_region_dir.exists()):
        raise AuditError(f"Source region directory not found: {source_region_dir}")

    if not translated_dir.exists():
        raise AuditError(f"Target rules directory not found: {translated_dir}")

    if target_region and not (translated_region_dir and translated_region_dir.exists()):
        raise AuditError(f"Target region directory not found: {translated_region_dir}")

    # Get list of files to audit
    files = [specific_file] if specific_file else get_yaml_files(source_dir, source_region_dir)

    print_audit_header(language, len(files), source_language)

    total_issues = 0
    total_missing = 0
    total_untranslated = 0
    total_extra = 0
    total_differences = 0
    total_missing_definitions = 0
    total_extra_definitions = 0
    total_definition_type_mismatches = 0
    files_with_issues = 0
    files_ok = 0
    order_findings: list[tuple[str, int]] = []  # One item per file, eg  [("calculus.yaml", 2), ...]

    for file_name in files:
        english_path = source_dir / file_name
        translated_path = translated_dir / file_name
        translated_region_path = translated_region_dir / file_name if translated_region_dir else None
        english_region_path = source_region_dir / file_name if source_region_dir else None

        if not english_path.exists():
            console.print(f"\n[yellow]⚠ Warning:[/] Source file not found: {english_path}")
            continue

        existing_translated_region_path = (
            translated_region_path if translated_region_path and translated_region_path.exists() else None
        )
        existing_english_region_path = english_region_path if english_region_path and english_region_path.exists() else None

        if is_definitions_file(file_name):
            definition_result = compare_definition_files(
                english_path,
                translated_path,
                issue_filter,
                existing_translated_region_path,
                existing_english_region_path,
            )
            issues = print_definition_findings(
                definition_result,
                file_name,
                language,
                source_language,
            )
            if issues > 0:
                files_with_issues += 1
            else:
                files_ok += 1
            total_issues += issues
            total_missing_definitions += len(definition_result.missing_definitions)
            total_extra_definitions += len(definition_result.extra_definitions)
            total_definition_type_mismatches += len(definition_result.type_mismatches)
        else:
            result = compare_files(
                english_path,
                translated_path,
                issue_filter,
                existing_translated_region_path,
                existing_english_region_path,
            )
            if result.first_order_mismatch_position is not None:
                order_findings.append((file_name, result.first_order_mismatch_position))

            if result.has_issues:
                issues = print_warnings(result, file_name, verbose, language, source_language)
                if issues > 0:
                    files_with_issues += 1
                total_issues += issues
            else:
                files_ok += 1

            total_missing += len(result.missing_rules)
            total_untranslated += sum(len(entries) for _rule, entries in result.untranslated_text)
            total_extra += len(result.extra_rules)
            total_differences += len(result.rule_differences)

    print_audit_summary(
        AuditSummary(
            files_checked=len(files),
            files_with_issues=files_with_issues,
            files_ok=files_ok,
            total_missing=total_missing,
            total_untranslated=total_untranslated,
            total_extra=total_extra,
            total_differences=total_differences,
            total_missing_definitions=total_missing_definitions,
            total_extra_definitions=total_extra_definitions,
            total_definition_type_mismatches=total_definition_type_mismatches,
            total_issues=total_issues,
        )
    )

    print_order_findings(order_findings)

    return total_issues


def list_languages(rules_dir: str | None = None) -> None:
    """List available languages for auditing"""
    rules_dir_path = get_rules_dir(rules_dir)

    languages: list[tuple[str, int]] = []
    for lang_dir in sorted(rules_dir_path.iterdir()):
        if not lang_dir.is_dir() or lang_dir.name == "en":
            continue
        languages.append((lang_dir.name, len(get_yaml_files(lang_dir))))

        for region_dir in sorted(lang_dir.iterdir()):
            if not region_dir.is_dir() or region_dir.name.lower() == "sharedrules":
                continue
            code = f"{lang_dir.name}-{region_dir.name}"
            languages.append((code, len(get_yaml_files(lang_dir, region_dir))))

    print_language_list(languages)
