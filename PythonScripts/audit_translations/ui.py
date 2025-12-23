"""
Terminal UI utilities for the audit tool.

Provides colors, symbols, and formatted printing helpers.
"""

import sys
from typing import List

from .dataclasses import RuleInfo


class Colors:
    """ANSI color codes for terminal output"""
    _enabled = hasattr(sys.stdout, 'isatty') and sys.stdout.isatty() # check if colors are supported

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
    c = Colors
    s = Symbols

    print(f"\n{c.CYAN}{s.BOX_TL}{s.BOX_H * (width - 2)}{s.BOX_TR}{c.RESET}")
    print(f"{c.CYAN}{s.BOX_V}{c.RESET}{c.BOLD}{text.center(width - 2)}{c.RESET}{c.CYAN}{s.BOX_V}{c.RESET}")
    print(f"{c.CYAN}{s.BOX_BL}{s.BOX_H * (width - 2)}{s.BOX_BR}{c.RESET}")


def print_file_header(file_name: str, english_count: int, translated_count: int, width: int = 70):
    """Print a file section header"""
    c = Colors
    s = Symbols

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
    c = Colors
    desc = f" {c.DIM}({description}){c.RESET}" if description else ""
    print(f"\n  {color}{icon}{c.RESET} {c.BOLD}{title}{c.RESET} [{color}{count}{c.RESET}]{desc}")


def print_rule_item(rule: RuleInfo, is_unicode: bool = False, context: str = ""):
    """Print a single rule item"""
    c = Colors
    s = Symbols

    if is_unicode or rule.name is None:
        key_display = f'"{rule.key}"'
        print(f"      {c.GRAY}{s.BULLET}{c.RESET} {c.YELLOW}{key_display}{c.RESET} {c.DIM}(line {rule.line_number}{context}){c.RESET}")
    else:
        print(f"      {c.GRAY}{s.BULLET}{c.RESET} {c.CYAN}{rule.name}{c.RESET} {c.DIM}[{rule.tag}]{c.RESET} {c.DIM}(line {rule.line_number}{context}){c.RESET}")


def print_text_samples(texts: List[str], max_show: int = 3):
    """Print sample untranslated text strings"""
    c = Colors
    s = Symbols

    for text in texts[:max_show]:
        # Truncate long text
        display_text = text if len(text) <= 40 else text[:37] + "..."
        print(f"          {c.GRAY}{s.ARROW}{c.RESET} {c.YELLOW}\"{display_text}\"{c.RESET}")

    if len(texts) > max_show:
        remaining = len(texts) - max_show
        print(f"          {c.DIM}... and {remaining} more{c.RESET}")


def print_summary_box(stats, width=70):
    """Print a formatted summary box"""
    c = Colors
    s = Symbols

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
