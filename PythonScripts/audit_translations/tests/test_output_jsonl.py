"""JSONL output snapshot tests."""

import json
from io import StringIO
from pathlib import Path

from ..auditor import IssueWriter, collect_issues, compare_files


def load_jsonl(text: str) -> list[dict]:
    return [json.loads(line) for line in text.splitlines() if line.strip()]

def load_json_array(path: Path) -> list[dict]:
    items = json.loads(path.read_text(encoding="utf-8"))
    for item in items:
        item.pop("_explanation", None) # only for humans
    return items


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

    golden_path = base_dir / "golden" / "jsonl" / "de.json"
    expected = load_json_array(golden_path)

    assert actual == expected
