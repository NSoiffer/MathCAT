"""JSONL output snapshot tests."""

import json
from io import StringIO
from pathlib import Path

from ..auditor import IssueWriter, collect_issues, compare_files


def load_jsonl(text: str) -> list[dict]:
    return [json.loads(line) for line in text.splitlines() if line.strip()]


def test_jsonl_output_matches_golden():
    """Ensure jsonl output matches golden."""
    base_dir = Path(__file__).parent
    fixtures_dir = base_dir / "fixtures"
    english_dir = fixtures_dir / "en"
    translated_dir = fixtures_dir / "de"
    files = sorted(path.name for path in english_dir.glob("*.yaml"))

    stream = StringIO()
    writer = IssueWriter("jsonl", stream)

    for file_name in files:
        result = compare_files(
            str(english_dir / file_name),
            str(translated_dir / file_name),
        )
        issues = collect_issues(result, file_name, "de")
        for issue in issues:
            writer.write(issue)

    actual = load_jsonl(stream.getvalue())

    golden_path = base_dir / "golden" / "jsonl" / "de.jsonl"
    expected = load_jsonl(golden_path.read_text(encoding="utf-8"))

    assert actual == expected
