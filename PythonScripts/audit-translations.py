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


class UI:
    """Master class for all Terminal UI logic, for example colors"""

    class Colors:
        """ANSI color codes for terminal output"""
        # Check if colors are supported
        _enabled = hasattr(sys.stdout, 'isatty') and sys.stdout.isatty()

        # Basic colors
        RED = '\033[91m' if _enabled else ''
        GREEN = '\033[92m' if _enabled else ''
        YELLOW = '\033[93m' if _enabled else ''
        BLUE = '\033[94m' if _enabled else ''
        MAGENTA = '\033[95m' if _enabled else ''
        CYAN = '\033[96m' if _enabled else ''
        WHITE = '\033[97m' if _enabled else ''
        GRAY = '\033[90m' if _enabled else ''

        # Styles
        BOLD = '\033[1m' if _enabled else ''
        DIM = '\033[2m' if _enabled else ''
        UNDERLINE = '\033[4m' if _enabled else ''

        # Reset
        RESET = '\033[0m' if _enabled else ''

        @classmethod
        def disable(cls):
            """Disable all colors"""
            cls.RED = cls.GREEN = cls.YELLOW = cls.BLUE = ''
            cls.MAGENTA = cls.CYAN = cls.WHITE = cls.GRAY = ''
            cls.BOLD = cls.DIM = cls.UNDERLINE = cls.RESET = ''


    # Symbols for visual clarity
    class Symbols:
        """Unicode symbols for better visual output"""
        CHECK = '✓'
        CROSS = '✗'
        WARNING = '⚠'
        INFO = 'ℹ'
        ARROW = '→'
        BULLET = '•'
        BOX_H = '─'
        BOX_V = '│'
        BOX_TL = '┌'
        BOX_TR = '┐'
        BOX_BL = '└'
        BOX_BR = '┘'
        BOX_T = '┬'
        BOX_B = '┴'
        BOX_L = '├'
        BOX_R = '┤'
        BOX_X = '┼'


    def print_header(text: str, width: int = 70):
        """Print a styled header box"""
        c = UI.Colors
        s = UI.Symbols
        padding = (width - len(text) - 2) // 2

        print(f"\n{c.CYAN}{s.BOX_TL}{s.BOX_H * (width - 2)}{s.BOX_TR}{c.RESET}")
        print(f"{c.CYAN}{s.BOX_V}{c.RESET}{c.BOLD}{text.center(width - 2)}{c.RESET}{c.CYAN}{s.BOX_V}{c.RESET}")
        print(f"{c.CYAN}{s.BOX_BL}{s.BOX_H * (width - 2)}{s.BOX_BR}{c.RESET}")


    def print_subheader(text: str, width: int = 70):
        """Print a styled subheader"""
        c = Colors
        s = Symbols
        print(f"\n{c.BLUE}{s.BOX_L}{s.BOX_H * 2}{c.RESET} {c.BOLD}{text}{c.RESET} {c.BLUE}{s.BOX_H * (width - len(text) - 6)}{s.BOX_R}{c.RESET}")


    def print_file_header(file_name: str, english_count: int, translated_count: int, width: int = 70):
        """Print a file section header"""
        c = UI.Colors
        s = UI.Symbols

        # Determine status color based on counts
        if translated_count == english_count:
            status_color = c.GREEN
            status_icon = s.CHECK
        elif translated_count == 0:
            status_color = c.RED
            status_icon = s.CROSS
        else:
            status_color = c.YELLOW
            status_icon = s.WARNING

        print(f"\n{c.CYAN}{s.BOX_H * width}{c.RESET}")
        print(f"{status_color}{status_icon}{c.RESET} {c.BOLD}{file_name}{c.RESET}")
        print(f"  {c.DIM}English: {english_count} rules  {s.ARROW}  Translated: {translated_count} rules{c.RESET}")
        print(f"{c.CYAN}{s.BOX_H * width}{c.RESET}")


    def print_issue_category(icon: str, color: str, title: str, count: int, description: str = ""):
        """Print an issue category header"""
        c = UI.Colors
        desc = f" {c.DIM}({description}){c.RESET}" if description else ""
        print(f"\n  {color}{icon}{c.RESET} {c.BOLD}{title}{c.RESET} [{color}{count}{c.RESET}]{desc}")


    def print_rule_item(rule, is_unicode: bool = False, context: str = ""):
        """Print a single rule item"""
        c = UI.Colors
        s = UI.Symbols

        if is_unicode or rule.name is None:
            key_display = f'"{rule.key}"'
            print(f"      {c.GRAY}{s.BULLET}{c.RESET} {c.YELLOW}{key_display}{c.RESET} {c.DIM}(line {rule.line_number}{context}){c.RESET}")
        else:
            print(f"      {c.GRAY}{s.BULLET}{c.RESET} {c.CYAN}{rule.name}{c.RESET} {c.DIM}[{rule.tag}]{c.RESET} {c.DIM}(line {rule.line_number}{context}){c.RESET}")


    def print_text_samples(texts: List[str], max_show: int = 3):
        """Print sample untranslated text strings"""
        c = UI.Colors
        s = UI.Symbols

        for text in texts[:max_show]:
            # Truncate long text
            display_text = text if len(text) <= 40 else text[:37] + "..."
            print(f"          {c.GRAY}{s.ARROW}{c.RESET} {c.YELLOW}\"{display_text}\"{c.RESET}")

        if len(texts) > max_show:
            remaining = len(texts) - max_show
            print(f"          {c.DIM}... and {remaining} more{c.RESET}")


    def print_summary_box(stats: dict, width: int = 70):
        """Print a formatted summary box"""
        c = UI.Colors
        s = UI.Symbols

        print(f"\n{c.CYAN}{s.BOX_TL}{s.BOX_H * (width - 2)}{s.BOX_TR}{c.RESET}")
        print(f"{c.CYAN}{s.BOX_V}{c.RESET}{c.BOLD}{'SUMMARY'.center(width - 2)}{c.RESET}{c.CYAN}{s.BOX_V}{c.RESET}")
        print(f"{c.CYAN}{s.BOX_L}{s.BOX_H * (width - 2)}{s.BOX_R}{c.RESET}")

        # Calculate column widths
        label_width = 30
        value_width = width - label_width - 6

        for label, value, color in stats:
            value_str = str(value)
            colored_value = f"{color}{value_str}{c.RESET}" if color else value_str
            padding = value_width - len(value_str)
            print(f"{c.CYAN}{s.BOX_V}{c.RESET}  {label:<{label_width}}{colored_value}{' ' * padding}  {c.CYAN}{s.BOX_V}{c.RESET}")

        print(f"{c.CYAN}{s.BOX_BL}{s.BOX_H * (width - 2)}{s.BOX_BR}{c.RESET}")


    def print_progress(current: int, total: int, file_name: str, width: int = 50):
        """Print a progress indicator"""
        c = UI.Colors
        s = UI.Symbols

        percentage = (current / total) * 100 if total > 0 else 0
        filled = int((current / total) * width) if total > 0 else 0
        bar = f"{c.GREEN}{'█' * filled}{c.GRAY}{'░' * (width - filled)}{c.RESET}"

        # Use \r to overwrite the line
        status = f"  {bar} {percentage:5.1f}% {c.DIM}({current}/{total}){c.RESET} {file_name}"
        print(f"\r{status}", end='', flush=True)


    def clear_progress():
        """Clear the progress line"""
        print("\r" + " " * 100 + "\r", end='', flush=True)


if sys.platform == 'win32': # Ensure UTF-8 output on Windows
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')
    # Enable ANSI escape sequences on Windows 10+
    try:
        import ctypes
        kernel32 = ctypes.windll.kernel32
        kernel32.SetConsoleMode(kernel32.GetStdHandle(-11), 7)
    except:
        pass


# =============================================================================
# Data Classes
# =============================================================================

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


# =============================================================================
# File Parsing Functions
# =============================================================================

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


# =============================================================================
# Comparison Functions
# =============================================================================

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


# =============================================================================
# Output Functions
# =============================================================================

def print_warnings(result: ComparisonResult, file_name: str) -> int:
    """Print warnings to console. Returns count of issues found."""
    c = UI.Colors
    s = UI.Symbols
    issues = 0

    has_issues = result.missing_rules or result.untranslated_text or result.extra_rules

    if has_issues:
        UI.print_file_header(file_name, result.english_rule_count, result.translated_rule_count)

    if result.missing_rules:
        UI.print_issue_category(
            s.CROSS, c.RED,
            "Missing Rules",
            len(result.missing_rules),
            "in English but not in translation"
        )
        for rule in result.missing_rules:
            UI.print_rule_item(rule, context=" in English")
            issues += 1

    if result.untranslated_text:
        UI.print_issue_category(
            s.WARNING, c.YELLOW,
            "Untranslated Text",
            len(result.untranslated_text),
            "lowercase t/ot/ct keys"
        )
        for rule, texts in result.untranslated_text:
            UI.print_rule_item(rule)
            UI.print_text_samples(texts)
            issues += 1

    if result.extra_rules:
        UI.print_issue_category(
            s.INFO, c.BLUE,
            "Extra Rules",
            len(result.extra_rules),
            "may be intentional"
        )
        for rule in result.extra_rules:
            UI.print_rule_item(rule)

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
    c = UI.Colors
    s = UI.Symbols

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
    UI.print_header(f"MathCAT Translation Audit: {language.upper()}")
    print(f"\n  {c.DIM}Comparing against English (en) reference files{c.RESET}")
    print(f"  {c.DIM}Files to check: {len(files)}{c.RESET}")

    total_issues = 0
    total_missing = 0
    total_untranslated = 0
    total_extra = 0
    files_with_issues = 0
    files_ok = 0

    for i, file_name in enumerate(files):
        english_path = english_dir / file_name
        translated_path = translated_dir / file_name

        # Show progress for multiple files
        if len(files) > 1:
            UI.print_progress(i + 1, len(files), file_name)

        if not english_path.exists():
            UI.clear_progress()
            print(f"\n{c.YELLOW}{s.WARNING} Warning:{c.RESET} English file not found: {english_path}")
            continue

        result = compare_files(str(english_path), str(translated_path))

        has_issues = result.missing_rules or result.untranslated_text or result.extra_rules

        if has_issues:
            if len(files) > 1:
                UI.clear_progress()
            issues = print_warnings(result, file_name)
            if issues > 0:
                files_with_issues += 1
            total_issues += issues
        else:
            files_ok += 1

        total_missing += len(result.missing_rules)
        total_untranslated += len(result.untranslated_text)
        total_extra += len(result.extra_rules)

    # Clear progress bar
    if len(files) > 1:
        UI.clear_progress()

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

    UI.print_summary_box(stats)

    # Overall status
    status_color, status_icon, status_text = overall_status
    print(f"\n  {status_color}{status_icon} {status_text}{c.RESET}\n")

    return total_issues


def list_languages():
    """List available languages for auditing"""
    c = UI.Colors
    s = UI.Symbols

    rules_dir = get_rules_dir()

    UI.print_header("Available Languages")

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


def main():
    parser = argparse.ArgumentParser(
        description="Audit MathCAT translation files against English originals",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
    python audit-translations.py es
    python audit-translations.py de --file SharedRules/default.yaml
    python audit-translations.py --list
        """
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

    parser.add_argument(
        "--no-color",
        action="store_true",
        help="Disable colored output"
    )

    args = parser.parse_args()

    if args.no_color:
        UI.Colors.disable()

    if args.list:
        list_languages()
        return

    if not args.language:
        parser.print_help()
        c = UI.Colors
        print(f"\n{c.RED}Error:{c.RESET} Please specify a language code or use --list to see available languages")
        sys.exit(1)

    audit_language(args.language, args.specific_file)


if __name__ == "__main__":
    main()
