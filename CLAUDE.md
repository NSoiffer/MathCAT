# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

MathCAT (Math Capable Assistive Technology) is a Rust library that converts MathML to:
- Speech strings (multiple languages) with embedded speech engine commands
- Braille (Nemeth, UEB Technical, CMU, and others)
- Navigation support for math expressions

The library is used by assistive technologies including NVDA and JAWS.

## Build Commands

```bash
# Standard build
cargo build

# Build with embedded rules (for WASM/distribution)
cargo build --features "include-zip"

# Run the test binary (for development/debugging)
RUST_LOG=DEBUG cargo run --features "include-zip"

# Run all tests
cargo test

# Run a single test
cargo test test_name

# Run tests for a specific language
cargo test Languages::en

# Run tests with debug logging
RUST_LOG=DEBUG cargo test test_name -- --nocapture
```

## Architecture

### Core Pipeline

1. **MathML Input** -> `set_mathml()` in `src/interface.rs`
2. **Canonicalization** -> `src/canonicalize.rs` cleans and normalizes MathML
3. **Intent Inference** -> `src/infer_intent.rs` adds semantic meaning
4. **Output Generation**:
   - Speech: `src/speech.rs` -> `src/tts.rs` for TTS formatting
   - Braille: `src/braille.rs`
   - Navigation: `src/navigate.rs`

### Key Source Files

- `src/lib.rs` - Library entry point, exports public API
- `src/interface.rs` - Public API functions (`set_rules_dir`, `set_mathml`, `get_spoken_text`, `get_braille`, etc.)
- `src/prefs.rs` - Preference management system
- `src/definitions.rs` - Rule definitions and loading
- `src/xpath_functions.rs` - Custom XPath functions for rule matching
- `src/chemistry.rs` - Chemical formula detection and handling
- `src/pretty_print.rs` - MathML output formatting

### Rules Directory Structure

Rules are YAML files under `Rules/`:
- `Rules/Languages/{lang}/` - Speech rules per language (e.g., `en/`, `de/`, `es/`)
  - `{Style}_Rules.yaml` - Speech style rules (SimpleSpeak, ClearSpeak)
  - `definitions.yaml` - Language-specific definitions
  - `unicode.yaml` - Character pronunciations
  - `SharedRules/` - Rules shared across speech styles
- `Rules/Braille/{code}/` - Braille code rules (Nemeth, UEB, CMU, etc.)
- `Rules/Intent/` - Intent inference rules

### Test Structure

Tests are in `tests/`:
- `tests/common/mod.rs` - Test helper functions (`test()`, `test_prefs()`, `test_braille()`)
- `tests/Languages/{lang}/` - Language-specific speech tests
- `tests/braille.rs` - Braille output tests

Test helper pattern:
```rust
test("en", "SimpleSpeak", mathml_input, expected_speech);
test_prefs("en", "ClearSpeak", vec![("Verbosity", "Verbose")], mathml, speech);
test_braille("Nemeth", mathml, expected_braille);
```

### Build System

`build.rs` creates `rules.zip` when `include-zip` feature is enabled. This bundles all YAML rules for distribution (especially WASM builds). Without this feature, rules are read from the filesystem at runtime.

### Library Types

The library produces both:
- `rlib` - Rust library for Rust consumers
- `cdylib` - C-compatible dynamic library for FFI (Python, C/C++ bindings)

## Key Preferences

Set via `set_preference()`:
- `Language` - Speech language ("en", "de", "es", etc.)
- `SpeechStyle` - "SimpleSpeak", "ClearSpeak", "LiteralSpeak"
- `BrailleCode` - "Nemeth", "UEB", "CMU", etc.
- `Verbosity` - "Terse", "Medium", "Verbose"
- `TTS` - "SSML", "SAPI5", "None"

## Environment Variables

- `MathCATRulesDir` - Override rules directory location (used when rules dir not passed to `set_rules_dir`)
- `RUST_LOG` - Control logging level (DEBUG, INFO, etc.)
