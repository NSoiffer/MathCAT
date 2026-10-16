# Contributing to MathCAT

Thanks for your interest in MathCAT! This is a short intro to get you started.
For more detail, see the [full documentation](https://daisy.github.io/MathCAT/).

## Ways to help

- **Report bugs / suggest features:** open an [issue](https://github.com/daisy/MathCAT/issues).
  Include the MathML input, the output you got, and the output you expected
  (the [MathCAT Demo](https://daisy.github.io/MathCATDemo/) is handy for this).
- **Code:** fixes and improvements to the Rust library (`src/`) or the rules (`Rules/`).
- **Translations:** translators have their own guide. Start with
  [information for translators/rule writers](https://daisy.github.io/MathCAT/helpers.html).
  If you want to add a new language, please open an issue first.

## Getting set up

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Fork and clone the repo.
3. Build and run the tests:
   ```bash
   cargo build
   cargo test
   ```
4. For the Python tooling (e.g. translation audits), use [uv](https://docs.astral.sh/uv/).
   See [`PythonScripts/README.md`](PythonScripts/README.md).

The [Developer Guide](docs/developers.md) has more on testing and coverage.

## Pull requests

- Keep PRs focused: one fix or feature per PR.
- **Add relevant unit tests whenever it makes sense.** A bug fix should usually include
  a test that would have caught it. Trivial changes (e.g. fixing a typo in a comment)
  don't need one.
- Make sure `cargo test` and `cargo clippy` pass. CI runs both and treats warnings as errors.
- Don't change expected outputs (speech/braille/navigation strings) in existing tests
  unless that change is the point of your PR. If so, explain why in the description.
- Link the issue your PR addresses, if there is one.

## Using AI tools

Using AI to help write code is fine. You are still responsible for what you submit:
review it, understand it, and test it.

Issues, PR descriptions, and comments should be **written by a human**. If you want to
include AI-generated text, put it in a clearly labeled block at the end, for example:

> **AI summary:** .

Nobody should have to read through walls of generated text to find out what you mean.

## Questions?

Open an issue or ask in the relevant issue/PR thread. We're happy to help.

