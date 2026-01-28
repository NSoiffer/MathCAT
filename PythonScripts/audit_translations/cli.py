"""
Command-line interface for the audit tool.

Handles argument parsing and the main entry point.
"""

import argparse
import sys

from .auditor import audit_language, list_languages, console


def main():
    """Main entry point for the audit tool"""

    parser = argparse.ArgumentParser(
        description="Audit MathCAT translation files against English originals",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
    uv run audit-translations es
    uv run audit-translations de --file SharedRules/default.yaml
    uv run audit-translations --list
        """
    )

    parser.add_argument("language", nargs="?", help="Language code to audit (e.g., 'es', 'de', 'fi')")
    parser.add_argument("--file", dest="specific_file", help="Audit only a specific file (e.g., 'SharedRules/default.yaml')")
    parser.add_argument("--list", action="store_true", help="List available languages")
    parser.add_argument("--rules-dir", help="Override Rules/Languages directory path")
    parser.add_argument(
        "--format",
        choices=["rich", "jsonl"],
        default="rich",
        help="Output format (default: rich)",
    )
    parser.add_argument("--output", help="Write output to a file instead of stdout")
    parser.add_argument(
        "--only",
        help="Comma-separated issue types: missing, untranslated, extra, diffs, all",
    )
    parser.add_argument(
        "--verbose",
        action="store_true",
        help="Show detailed output including rule snippets (only affects rich format)",
    )

    args = parser.parse_args()

    if args.list:
        list_languages(args.rules_dir)
    elif not args.language:
        parser.print_help()
        console.print("\n[red]Error:[/] Please specify a language code or use --list to see available languages")
        sys.exit(1)
    else:
        issue_filter = None
        if args.only:
            tokens = [token.strip().lower() for token in args.only.split(",") if token.strip()]
            if "all" not in tokens:
                allowed = {"missing", "untranslated", "extra", "diffs"}
                unknown = set(tokens) - allowed
                if unknown:
                    console.print(
                        "\n[red]Error:[/] Unknown issue types: "
                        + ", ".join(sorted(unknown))
                    )
                    sys.exit(1)
                issue_filter = set(tokens)

        audit_language(
            args.language,
            args.specific_file,
            args.format,
            args.output,
            args.rules_dir,
            issue_filter,
            args.verbose,
        )


if __name__ == "__main__":
    main()
