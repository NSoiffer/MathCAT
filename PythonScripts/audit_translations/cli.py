"""
Command-line interface for the audit tool.

Handles argument parsing and the main entry point.
"""

import argparse
import io
import sys

from . import ui
from .auditor import audit_language, list_languages


def setup_encoding():
    """Ensure UTF-8 output on Windows"""
    if sys.platform == 'win32':
        sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
        sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')


def main():
    """Main entry point for the audit tool"""
    setup_encoding()

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
        ui.Colors.disable()

    if args.list:
        list_languages()
        return

    if not args.language:
        parser.print_help()
        c = ui.Colors
        print(f"\n{c.RED}Error:{c.RESET} Please specify a language code or use --list to see available languages")
        sys.exit(1)

    audit_language(args.language, args.specific_file)


if __name__ == "__main__":
    main()
