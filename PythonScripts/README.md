# Python Scripts

Project management is done with [uv](https://docs.astral.sh/uv/).

For example, execute `uv run audit-translations de` to see the translation progress for the German language.

If you run from the repo root instead of inside `PythonScripts`, point uv at the project and make sure you've synced once:
```bash
uv sync --project PythonScripts
uv run --project PythonScripts audit-translations de
```

To run rule YAML coverage from the repo root, use the audit tool:

```bash
uv run --project PythonScripts audit-translations --rule-coverage
```

The tool opens `target/rule-coverage/index.html` in a browser when the run finishes. Expand a YAML file to see each active rule's name, tag, hit count, and match status. Hover over a rule to see which tests hit it. Coverage fractions include percentages.
The Rust test output remains at `target/rule-coverage/test.log`.
