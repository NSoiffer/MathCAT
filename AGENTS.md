# Agent Notes (MathCAT)

Purpose: fast orientation for coding agents. try to keep low overlap with README.md etc., 
but add common mistakes of AI agents here instead.

## Project Scope
- MathCAT converts MathML to speech, braille, and navigation output.
- Core flow: `set_mathml()` -> `canonicalize.rs` -> optional `infer_intent.rs` -> `speech.rs` / `braille.rs`.

## Rules System (`Rules/`)
- YAML rules are loaded at runtime by domain:
- Common per-language files:
  - `ClearSpeak_Rules.yaml`, `SimpleSpeak_Rules.yaml`
  - `SharedRules/`, `unicode.yaml`, `unicode-full.yaml`, `definitions.yaml`, `navigate.yaml`
- `build.rs` can bundle rules into `rules.zip` when `include-zip` is enabled (always the minimized tree). The `lang-*` / `braille-*` features (all on by default via `rules-all`) choose which `Languages/` and `Braille/` directories it embeds; every rules directory needs a feature listed in `lang-all` / `braille-all`, and `build.rs` fails without one. Shared zipper: `src/rules_archive.rs`, invoked as `cargo run --bin package-rules -- Rules <output> [--minimize]`.
- CI packages `Rules.zip` (verbatim) and `Rules-minimized.zip` (comments stripped from `Languages/**/unicode.yaml` and `unicode-full.yaml` only). Each language/braille subdir is stored as `<name>/<name>.zip`; top-level and `Intent/` YAML stay loose. Inner language zips use BZIP2; the outer downloadable archive uses DEFLATE so `unzip` works. Both omit `Languages/zz`; the `test-rules-package` job restores `zz` from git after unzip so unit tests still run against release-like archives.

## Translation Conventions
- `t:` means untranslated or unverified.
- `T:` means translated and verified.
- don't demote from `T:` to `t:` unless instructed to.
- tool for comparing rules across languages: `uv run --project PythonScripts audit-translations <language-code>`

## Python Tooling (`uv`)
- `uv` is the Python dependency and project manager for repo tooling. Use `uv run <command-name>`
- discuss new packages before adding them. use `uv add <package-name>` and `uv sync` on confirmation
- In sandboxed runs, if needed:
  - set `UV_CACHE_DIR=/tmp/uv-cache`
  - rerun with escalated permissions if macOS `system-configuration` panics occur.
- self-validate appropriately with: `uv run pytest`

## Agent Instructions
- Do not mirror README content here; keep guidance agent-specific.
- Avoid broad formatting sweeps; do not run `cargo fmt` in this repo.
- Keep code/rule changes focused and validate with targeted tests first: `cargo test <relevant-tests>`
- Do **not** change test goldens (expected braille/speech/nav strings) unless the user explicitly says to. Prefer fixing code/rules to match existing goldens; if a golden looks wrong vs the spec, ask before editing it.
- do not do any git commands unless explicitly asked for
- Rust coverage is in `target/coverage/`.
- When working with GitHub, e.g. looking at PRs and issues, check if the GitHub CLI is installed (`gh --version`).
- When writing tests, add a brief description explaining the purpose and expected behaviour. Avoid complex setups like using heavily parameterized tests (eg `@pytest.mark.parametrize` for Python)

## Fuzzing (`fuzz/` + cargo-fuzz)
- Install: `cargo install cargo-fuzz`, use a **nightly** toolchain (`rustup run nightly cargo fuzz …`).
- **Dictionary:** `fuzz/mathml.dict` is passed to libFuzzer as `-dict=mathml.dict` (path relative to the `fuzz/` crate). It lists MathML 4 presentation elements and attributes to improve coverage.
- **Harness:** `fuzz/fuzz_targets/fuzz_target_1.rs` calls `set_mathml`, then (on success) `get_spoken_text`, `get_braille` (with `""` and a lossy tail id), `do_navigate_command` with valid command names from `src/navigate.rs` `NAV_COMMANDS`, then speech/braille again.
- **CI:** `.github/workflows/fuzz.yml` builds `fuzz_target_1` with default and `--features no-unsafe`, runs with `mathml.dict` and **`corpus/fuzz_target_1`** (repo: `fuzz/corpus/fuzz_target_1`). The corpus is **restored/saved via GitHub Actions cache** so inputs improve across runs. Push/PR: `-max_total_time=60` each; **weekly schedule: 3600s (1 hour) per configuration**. Weekly runs also **minimize the corpus** (`cargo fuzz cmin`) to keep the cache lean. Workflows use **`actions/checkout@v5`**, **`actions/cache@v5`**, **`actions/upload-artifact@v7`**, and **`actions/download-artifact@v8`** (Node 24 runtimes).
- **Crash artifacts:** uploaded as GitHub Actions artifacts (90-day retention) on any fuzz failure. Download them from the failed workflow run's "Artifacts" section.
- **Regression tests from crashes:** run `python PythonScripts/fuzz_to_test.py` (optionally pass artifact paths; defaults to `fuzz/artifacts/fuzz_target_1/`). This appends `#[test]` functions to `tests/fuzz_regressions.rs`, deduplicating by content hash. Tests call `set_mathml` + `get_spoken_text` + `get_braille` on the crash input.
- **Coverage:** weekly CI runs `cargo fuzz coverage` against the corpus, then produces an HTML report via `llvm-cov show`. The report is uploaded as the **`fuzz-coverage-report`** Actions artifact (30-day retention). Download and open `index.html` to see which source lines the fuzzer has reached.
- **Windows:** prefer **WSL** for local `cargo fuzz` (ASAN/libFuzzer); CI runs on `ubuntu-latest`.
