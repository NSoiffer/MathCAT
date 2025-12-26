"""
Command-line interface for the audit tool.

Handles argument parsing and the main entry point.
"""

import argparse
import io
import sys

from .auditor import audit_language, list_languages, console


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
    python -m audit_translations es
    python -m audit_translations de --file SharedRules/default.yaml
    python -m audit_translations --list
        """
    )

    parser.add_argument("language", nargs="?", help="Language code to audit (e.g., 'es', 'de', 'fi')")
    parser.add_argument("--file", dest="specific_file", help="Audit only a specific file (e.g., 'SharedRules/default.yaml')")
    parser.add_argument("--list", action="store_true", help="List available languages")

    args = parser.parse_args()

    if args.list:
        list_languages()
    elif not args.language:
        parser.print_help()
        console.print("\n[red]Error:[/] Please specify a language code or use --list to see available languages")
        sys.exit(1)
    else:
        audit_language(args.language, args.specific_file)


if __name__ == "__main__":
    main()
