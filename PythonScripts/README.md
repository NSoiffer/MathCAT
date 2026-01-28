# Python Scripts

Project management is done with [uv](https://docs.astral.sh/uv/).

For example, execute `uv run audit-translations de` to see the translation progress for the German language.

If you run from the repo root instead of inside `PythonScripts`, point uv at the project and make sure you've synced once:
```bash
uv sync --project PythonScripts
uv run --project PythonScripts audit-translations de
```
