"""Run the full Rust test suite and report which rule YAML files it exercises."""

import json
import shutil
import subprocess
import sys
import webbrowser
from collections import Counter, defaultdict
from pathlib import Path, PurePosixPath

from .rule_coverage_report import RuleKey, coverage, render_html

ROOT = Path(__file__).resolve().parents[3]
OUTPUT = ROOT / "target" / "rule-coverage"
EVENTS = OUTPUT / "events"


def read_events() -> tuple[set[RuleKey], dict[RuleKey, Counter[str]], list[str]]:
    defined_rules: set[RuleKey] = set()
    hits_by_test: dict[RuleKey, Counter[str]] = defaultdict(Counter)
    errors: list[str] = []
    for event_file in sorted(EVENTS.glob("*.jsonl")):
        for number, line in enumerate(event_file.read_text(encoding="utf-8").splitlines(), 1):
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                errors.append(f"Invalid JSON in {event_file.name}:{number}")
                continue
            if not isinstance(event, dict):
                errors.append(f"Invalid event in {event_file.name}:{number}")
                continue
            kind = event.get("kind")
            path = event.get("path")
            if not isinstance(path, str):
                errors.append(f"Invalid event in {event_file.name}:{number}")
                continue
            path = path.replace("\\", "/")
            parts = PurePosixPath(path).parts
            if (
                not parts
                or PurePosixPath(path).is_absolute()
                or ".." in parts
                or PurePosixPath(path).suffix not in (".yaml", ".yml")
            ):
                errors.append(f"Invalid event in {event_file.name}:{number}")
            elif kind in ("defined-rule", "matched-rule") and {"name", "tag"} <= event.keys():
                name, tag = event["name"], event["tag"]
                if not isinstance(name, str) or not isinstance(tag, str) or not name or not tag:
                    errors.append(f"Empty rule identity in {event_file.name}:{number}")
                elif kind == "defined-rule" and event.keys() == {"kind", "path", "name", "tag"}:
                    defined_rules.add((path, name, tag))
                elif (
                    kind == "matched-rule"
                    and event.keys() == {"kind", "path", "name", "tag", "test"}
                    and isinstance(event["test"], str)
                    and event["test"]
                ):
                    key = (path, name, tag)
                    hits_by_test[key][event["test"]] += 1
                else:
                    errors.append(f"Invalid event in {event_file.name}:{number}")
            else:
                errors.append(f"Invalid event in {event_file.name}:{number}")
    return defined_rules, dict(hits_by_test), errors


def run() -> int:
    """Generate the HTML report and open it when the run finishes."""
    OUTPUT.mkdir(parents=True, exist_ok=True)
    (OUTPUT / "report.md").unlink(missing_ok=True)
    if EVENTS.exists():
        shutil.rmtree(EVENTS)
    EVENTS.mkdir()

    command = ["cargo", "test", "--features", "rule-coverage"]
    log_path = OUTPUT / "test.log"
    print(f"Running {' '.join(command)}; saving output to {log_path.relative_to(ROOT)}", flush=True)
    with log_path.open("w", encoding="utf-8") as log:
        try:
            result = subprocess.run(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT, check=False)
            test_status = result.returncode
        except OSError as error:
            log.write(f"Could not run cargo: {error}\n")
            test_status = 1

    defined_rules, hits_by_test, errors = read_events()
    matched_rules = set(hits_by_test)
    rule_files = {path for path, _, _ in defined_rules}
    matched_files = {path for path, _, _ in matched_rules}
    if test_status:
        errors.insert(0, f"cargo test failed (exit status {test_status}); see test.log")
    if not defined_rules:
        errors.append("No active rule definitions found")
    if not matched_rules:
        errors.append("No matched rule events found")
    if matched_rules - defined_rules:
        errors.append("Matched rules lack definition events")

    status = "Incomplete" if errors else "Complete"
    html_path = OUTPUT / "index.html"
    html_path.write_text(render_html(defined_rules, hits_by_test, errors), encoding="utf-8")
    print(
        f"{status}: {coverage(len(matched_files), len(rule_files))} rule files matched; "
        f"{coverage(len(matched_rules), len(defined_rules))} rules matched; report: {html_path}"
    )
    try:
        opened = webbrowser.open(html_path.resolve().as_uri())
    except OSError, webbrowser.Error:
        opened = False
    if not opened:
        print(f"Browser unavailable; open {html_path} manually")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(run())
