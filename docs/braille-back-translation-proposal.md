# Braille to MathML Back-Translation Implementation Proposal

**Related Issue**: [#318 - Add braille -> MathML functionality](https://github.com/NSoiffer/MathCAT/issues/318)

**Date**: December 2025

---

## Executive Summary

This document proposes an implementation strategy for adding braille-to-MathML back-translation functionality to MathCAT. This feature would enable blind and visually impaired users to input mathematical expressions in braille (Nemeth, UEB, etc.) and have them converted to MathML for use in documents, educational materials, and assistive technologies.

---

## Table of Contents

1. [Background and Motivation](#background-and-motivation)
2. [Existing Work and Prior Art](#existing-work-and-prior-art)
3. [Technical Challenges](#technical-challenges)
4. [Proposed Architecture](#proposed-architecture)
5. [Parser Technology Selection](#parser-technology-selection)
6. [Implementation Phases](#implementation-phases)
7. [Braille Code Considerations](#braille-code-considerations)
8. [Error Handling Strategy](#error-handling-strategy)
9. [Testing Strategy](#testing-strategy)
10. [API Design](#api-design)
11. [Future Considerations](#future-considerations)
12. [References and Resources](#references-and-resources)

---

## Background and Motivation

### The Need for Back-Translation

MathCAT currently excels at forward translation (MathML to braille), but the reverse direction is increasingly important:

1. **Braille Input Devices**: Refreshable braille displays (BrailleNote, BrailleSense, Monarch) support braille keyboard input
2. **Educational Equity**: Blind students need to create mathematical content, not just consume it
3. **Document Authoring**: Enable blind users to author accessible math documents
4. **Round-Trip Editing**: Edit braille math and convert back to visual formats

### Use Cases

- **Students**: Completing math homework and exams
- **Professionals**: Blind mathematicians, scientists, and educators creating content
- **Transcribers**: Quality-checking braille transcriptions
- **Accessibility Tools**: Real-time math input in educational software

---

## Existing Work and Prior Art

### BackNem (Susan Jolly / Dotless Braille)

- **Language**: Java with ANTLR 4
- **Scope**: Linear Nemeth to MathML (spatial layout not fully supported)
- **Status**: Beta version from ~2004, NSF-funded (Award No. IIS-0312487)
- **Key Insight**: ANTLR-based parser approach works well for Nemeth
- **Repository**: [MML2Nem](https://github.com/SusanJ/MML2Nem) (forward direction)

Susan Jolly's [TUGboat paper](https://www.tug.org/TUGboat/tb40-1/tb124jolly-braille.pdf) provides valuable insights into Nemeth's structure and parsing considerations.

### SBT (Spanish Braille to MathML)

- **Academic Paper**: [SpringerLink](https://link.springer.com/chapter/10.1007/11788713_174)
- **Key Features**:
  - Portable programming library design
  - Intermediate code representation
  - Ambiguity resolution for Spanish mathematical braille
- **Lesson**: Intermediate representations help manage complexity

### Liblouis

- **Language**: C
- **Scope**: General braille translation/back-translation, including math
- **Supports**: Nemeth, Marburg, and other codes
- **Website**: [liblouis.io](https://liblouis.io/)
- **Limitation**: Table-based approach may struggle with complex nested math

### Speech Rule Engine (Volker Sorge)

- **Language**: JavaScript
- **Scope**: MathML to speech/braille (forward only)
- **Supports**: Full Nemeth output via semantic tree transformation
- **GitHub**: [Speech-Rule-Engine](https://github.com/Speech-Rule-Engine/speech-rule-engine)
- **Key Architecture**: Uses internal semantic tree representation

### The Equalize Editor

- **Language**: JavaScript (browser-based)
- **Scope**: Document translation with UEB/Nemeth math
- **Website**: [Knowbility](https://knowbility.org/programs/be-a-digital-ally/december-2025)
- **Note**: Web-based workflow, may have back-translation components

---

## Technical Challenges

### 1. Structural Ambiguity

Unlike programming languages, braille math can have ambiguous interpretations:

```
Example: In Nemeth, certain character sequences could be interpreted
multiple ways depending on mathematical context.
```

**Mitigation**: Use heuristics, preference ordering, and potentially allow user disambiguation.

### 2. Context-Sensitive Semantics

Nemeth and UEB use indicators that change meaning based on context:
- Numeric indicator (Nemeth: dots 3-4-5-6) signals number mode
- Letter indicators distinguish variables from numbers
- Typeform indicators (bold, italic) have scope

**Mitigation**: Maintain parser state machine tracking current mode/context.

### 3. Nested Structures

Mathematical expressions have deep nesting:
- Fractions within fractions
- Radicals with indices
- Scripts on scripts

Nemeth uses "nesting indicators" (repeated level markers) that must be tracked:

```yaml
# From MathCAT's Nemeth_Rules.yaml:
variables: [NestingChars: "NestingChars(., '...')"]
```

**Mitigation**: Use a proper parser with recursive descent or similar capability.

### 4. Partial/Invalid Input

Real-world input may be incomplete or erroneous:
- Missing end indicators
- Unbalanced grouping symbols
- Incomplete structures

**Mitigation**: Error-tolerant parsing with meaningful error recovery.

### 5. Spatial vs. Linear Layout

Nemeth supports both linear and spatial (2D) layouts for matrices, long division, etc.

**Mitigation**: Phase 1 focuses on linear; spatial layout is Phase 2+.

### 6. Code Switching (UEB with Nemeth)

Documents may switch between UEB literary and Nemeth technical:

```
UEB text ... [Nemeth indicator] math expression [Nemeth terminator] ... UEB text
```

**Mitigation**: Detect mode indicators and switch parsing strategies.

---

## Proposed Architecture

### High-Level Design

```
+------------------+     +----------------+     +------------------+
|  Braille Input   | --> |    Parser      | --> | Semantic/Intent  |
| (Unicode braille)|     | (code-specific)|     |      Tree        |
+------------------+     +----------------+     +------------------+
                                                        |
                                                        v
+------------------+     +----------------+     +------------------+
|  MathML Output   | <-- |   MathML Gen   | <-- | Tree Transform   |
|                  |     |                |     |                  |
+------------------+     +----------------+     +------------------+
```

### Component Breakdown

#### 1. Input Layer
- Accept Unicode braille (U+2800-U+28FF range)
- Support 6-dot and 8-dot braille
- Handle ASCII braille input (optional conversion)

#### 2. Lexer/Tokenizer
- Convert braille cells to meaningful tokens
- Handle multi-cell symbols (e.g., Nemeth fraction indicators)
- Track position for error reporting

#### 3. Parser (Code-Specific)
- Separate grammar for each braille code:
  - `NemethParser`
  - `UEBMathParser`
  - `CMUParser` (Spanish)
  - etc.
- Build abstract syntax tree (AST) or semantic tree

#### 4. Semantic Tree
- Code-independent intermediate representation
- Similar to MathCAT's intent tree concept
- Captures mathematical meaning, not just structure

#### 5. MathML Generator
- Transform semantic tree to MathML
- Can leverage existing MathCAT `pretty_print` infrastructure
- Generate both Presentation and Content MathML (configurable)

### Integration with MathCAT

The back-translation module should:
1. Share definitions with forward translation (unicode mappings, etc.)
2. Reuse existing MathML generation utilities
3. Follow MathCAT's preference system for configuration
4. Integrate with navigation for cursor position tracking

---

## Parser Technology Selection

### Evaluation Criteria

| Criterion | Weight | Description |
|-----------|--------|-------------|
| Error Recovery | High | Handle incomplete/invalid input gracefully |
| Rust Native | High | No external runtime dependencies |
| Maintainability | High | Easy to modify grammars as codes evolve |
| Performance | Medium | Fast enough for real-time input |
| Precedence Handling | Medium | Math operators have complex precedence |
| Documentation | Medium | Good learning resources |

### Options Analysis

#### 1. **pest** (PEG Parser)

**Pros**:
- Clean, readable grammar files (`.pest`)
- Excellent documentation ([pest.rs/book](https://pest.rs/book/))
- Zero-copy parsing, good performance
- Built-in Pratt parser for operator precedence
- Mature, widely used in Rust ecosystem

**Cons**:
- Limited error recovery (fails at first error by default)
- PEG ordered choice can be tricky for ambiguous grammars

**Example Grammar Sketch**:
```pest
// nemeth.pest
math = { SOI ~ expression ~ EOI }
expression = { term ~ (operator ~ term)* }
term = { fraction | radical | script | grouped | number | letter }

fraction = {
    fraction_open ~ expression ~ fraction_over ~ expression ~ fraction_close
}
fraction_open = { "\u{2820}\u{2839}" }  // Nemeth open fraction
fraction_over = { "\u{280C}" }          // Nemeth fraction line
fraction_close = { "\u{2820}\u{283C}" } // Nemeth close fraction

number = { numeric_indicator? ~ digit+ }
numeric_indicator = { "\u{283C}" }
digit = { '\u{2801}'..'\u{280A}' | "\u{281A}" }  // 1-9, 0
```

#### 2. **lalrpop** (LR(1) Parser)

**Pros**:
- Handles left-recursion naturally
- Excellent for expression grammars with precedence
- Compile-time grammar validation
- Good error messages during development

**Cons**:
- Grammar must be unambiguous (LR(1) requirement)
- More complex setup than pest
- Error recovery requires explicit handling

**Example Grammar Sketch**:
```lalrpop
// nemeth.lalrpop
grammar;

pub Math: MathExpr = {
    <e:Expression> => e,
};

Expression: MathExpr = {
    <l:Expression> <op:AddOp> <r:Term> => MathExpr::BinOp(op, Box::new(l), Box::new(r)),
    Term,
};

Fraction: MathExpr = {
    FRAC_OPEN <num:Expression> FRAC_OVER <den:Expression> FRAC_CLOSE
        => MathExpr::Fraction(Box::new(num), Box::new(den)),
};
```

#### 3. **nom** (Parser Combinators)

**Pros**:
- Maximum flexibility
- Excellent for binary/streaming data
- Best performance
- Easy incremental parsing

**Cons**:
- Grammars are code, not declarative
- Harder to read/maintain complex grammars
- No automatic error recovery

#### 4. **tree-sitter**

**Pros**:
- Excellent error recovery (continues parsing after errors)
- Incremental reparsing (ideal for editors)
- Produces partial ASTs with error nodes
- Battle-tested in editors (VS Code, Neovim, etc.)

**Cons**:
- Grammar written in JavaScript, compiled to C
- Rust bindings add complexity
- Overkill if not doing editor integration

### Recommendation: **pest** (Primary) with **tree-sitter** (Future)

**Rationale**:

1. **pest** is the best starting point:
   - Clean grammar syntax for rapid iteration
   - Good documentation for learning curve
   - Pratt parser support for operator precedence
   - Sufficient for CLI/library use case

2. **tree-sitter** for Phase 3 (Editor Integration):
   - When real-time editing support is needed
   - Error recovery becomes critical for incomplete input
   - Incremental parsing for performance

**Hybrid Approach**: Start with pest, validate the grammar design, then potentially port to tree-sitter if editor integration becomes a priority.

---

## Implementation Phases

### Phase 1: Nemeth Linear (MVP)

**Goal**: Parse linear Nemeth expressions to MathML

**Scope**:
- Basic arithmetic (+, -, x, /)
- Fractions (simple and complex)
- Radicals (square root, nth root)
- Superscripts and subscripts
- Greek letters
- Basic grouping (parentheses, brackets, braces)
- Numbers and variables

**Deliverables**:
1. Nemeth lexer
2. Nemeth parser (pest grammar)
3. Semantic tree representation
4. MathML generator
5. Basic test suite
6. API: `braille_to_mathml(braille: &str, code: &str) -> Result<String>`

**Estimated Effort**: 3-4 months

### Phase 2: Extended Nemeth + Error Handling

**Goal**: Production-ready Nemeth with robust error handling

**Scope**:
- All Nemeth symbols (full unicode.yaml coverage)
- Matrices and vectors (linear representation)
- Complex nested expressions
- Comprehensive error messages
- Partial input handling
- Ambiguity resolution heuristics

**Deliverables**:
1. Extended grammar
2. Error recovery strategy
3. Disambiguation preferences
4. Expanded test suite (edge cases)
5. Documentation

**Estimated Effort**: 2-3 months

### Phase 3: UEB Technical

**Goal**: Support UEB math notation

**Scope**:
- UEB Technical symbols
- Grade 1 indicators and modes
- Typeform indicators
- UEB-specific grouping

**Deliverables**:
1. UEB lexer and parser
2. Shared semantic tree (reuse from Nemeth)
3. UEB-specific tests

**Estimated Effort**: 2 months

### Phase 4: Code Switching & Spatial

**Goal**: Handle mixed documents and 2D layout

**Scope**:
- UEB/Nemeth switching (BANA guidelines)
- Spatial matrices/determinants
- Long division and other 2D formats
- Multi-line expressions

**Deliverables**:
1. Mode detection and switching
2. 2D layout parser
3. Complex document tests

**Estimated Effort**: 3-4 months

### Phase 5: Additional Codes

**Goal**: Support international braille codes

**Scope**:
- CMU (Spanish)
- German math braille
- French math braille
- Others based on demand

**Deliverables**:
1. Additional parsers
2. Locale-specific tests

**Estimated Effort**: 1-2 months per code

### Phase 6: Editor Integration (Optional)

**Goal**: Real-time editing support

**Scope**:
- tree-sitter port for error tolerance
- Incremental parsing
- Cursor position tracking
- IDE/editor plugins

**Estimated Effort**: 3-4 months

---

## Braille Code Considerations

### Nemeth Code Structure

Key structural elements to parse:

| Element | Opening Indicator | Closing Indicator | Notes |
|---------|------------------|-------------------|-------|
| Fraction | `...` (dots 4-5-6, 1-4-5-6) | `...` (dots 4-5-6, 3-4-5-6) | Nesting requires level markers |
| Radical | `...` (dots 3-4-5) | `...` (dots 1-2-4-5-6) | Index precedes if present |
| Superscript | `...` (dots 4-5) | Baseline return or next operator | Multipurpose indicator |
| Subscript | `...` (dots 5-6) | Baseline return | |
| Grouping | Standard parens/brackets | Matching close | May need shape indicators |

### UEB Technical Structure

UEB uses a different indicator system:

| Element | Indicator | Notes |
|---------|-----------|-------|
| Grade 1 Word | `...` (dots 5-6) | Single word in grade 1 |
| Grade 1 Passage | `...` (dots 5-6, 5-6, 5-6) | Until terminator |
| Numeric | `...` (dots 3-4-5-6) | Before numbers |
| Fraction | `...` open, `...` line, `...` close | Different from Nemeth |

### Symbol Mapping

Leverage MathCAT's existing `unicode.yaml` files in reverse:

```yaml
# Forward (in MathCAT):
"+": "..."  # Nemeth plus

# Back-translation needs inverse:
"...": "+"  # Or map to MathML <mo>+</mo>
```

Consider auto-generating reverse mappings from existing YAML files.

---

## Error Handling Strategy

### Error Categories

1. **Lexical Errors**: Unrecognized braille patterns
2. **Syntactic Errors**: Invalid structure (missing indicators)
3. **Semantic Errors**: Valid syntax but nonsensical math

### Error Recovery Approaches

#### 1. Panic Mode Recovery
Skip to next synchronization point (operator, end of expression)

```rust
fn recover_to_sync_point(input: &str) -> &str {
    // Skip until we find an operator or end indicator
}
```

#### 2. Phrase-Level Recovery
Insert/delete minimal tokens to continue parsing

```rust
// Missing fraction close? Insert one and warn
if at_end_of_input && fraction_open_count > 0 {
    emit_warning("Missing fraction close indicator");
    insert_virtual_token(FractionClose);
}
```

#### 3. Error Productions
Include error rules in grammar

```pest
fraction = {
    fraction_open ~ expression ~ fraction_over ~ expression ~ fraction_close
    | fraction_open ~ expression ~ fraction_over ~ expression ~ !fraction_close ~ error_missing_close
}
```

### Error Messages

Follow MathCAT's existing error handling patterns:

```rust
pub enum BackTranslationError {
    UnrecognizedSymbol { position: usize, braille: String },
    UnclosedFraction { open_position: usize },
    UnbalancedGrouping { expected: char, found: Option<char>, position: usize },
    AmbiguousExpression { possibilities: Vec<String>, position: usize },
    // ...
}
```

### User Feedback

Return structured errors that can be:
1. Displayed to users
2. Used for editor squiggles
3. Logged for debugging

```rust
pub struct ParseResult {
    pub mathml: Option<String>,
    pub errors: Vec<BackTranslationError>,
    pub warnings: Vec<BackTranslationWarning>,
}
```

---

## Testing Strategy

### Test Categories

1. **Unit Tests**: Individual parser rules
2. **Integration Tests**: Full braille-to-MathML pipelines
3. **Round-Trip Tests**: MathML -> Braille -> MathML consistency
4. **Fuzzing**: Random/malformed input handling
5. **Regression Tests**: Known edge cases

### Test Data Sources

1. **Invert Existing Tests**: Use MathCAT's braille test outputs as inputs
2. **Textbooks**: Sample problems from Nemeth braille math texts
3. **BANA Examples**: Official Nemeth code examples
4. **User Submissions**: Real-world braille from users

### Round-Trip Validation

```rust
#[test]
fn test_round_trip_fraction() {
    let mathml = "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>";
    let braille = mathml_to_braille(mathml, "Nemeth").unwrap();
    let recovered = braille_to_mathml(&braille, "Nemeth").unwrap();
    assert_mathml_equivalent(mathml, &recovered);
}
```

Note: Round-trip may not produce identical MathML, but should be semantically equivalent.

### Fuzzing

Use `cargo-fuzz` for discovering edge cases:

```rust
fuzz_target!(|data: &[u8]| {
    if let Ok(braille) = std::str::from_utf8(data) {
        // Should not panic, even on garbage input
        let _ = braille_to_mathml(braille, "Nemeth");
    }
});
```

---

## API Design

### Public Interface

```rust
/// Convert braille to MathML
///
/// # Arguments
/// * `braille` - Unicode braille string (U+2800-U+28FF)
/// * `code` - Braille code ("Nemeth", "UEB", "CMU", etc.)
///
/// # Returns
/// * `Ok(String)` - MathML string
/// * `Err(BackTranslationError)` - Parse error with details
pub fn braille_to_mathml(braille: &str, code: &str) -> Result<String>;

/// Convert braille to MathML with detailed results
pub fn braille_to_mathml_detailed(braille: &str, code: &str) -> ParseResult;

/// Set back-translation preferences
pub fn set_back_translation_preference(name: &str, value: &str) -> Result<()>;

/// Get supported braille codes for back-translation
pub fn get_supported_back_translation_codes() -> Vec<String>;
```

### Preference Options

```rust
// Disambiguation preferences
set_back_translation_preference("AmbiguityResolution", "Strict")?;  // or "Heuristic", "Interactive"

// Output format
set_back_translation_preference("MathMLFormat", "Presentation")?;  // or "Content", "Both"

// Error handling
set_back_translation_preference("ErrorRecovery", "Continue")?;  // or "Strict"
```

### Python Binding (for NVDA/etc.)

```python
def BrailleToMathML(braille: str, code: str = "Nemeth") -> str:
    """Convert braille string to MathML."""
    pass

def BrailleToMathMLDetailed(braille: str, code: str = "Nemeth") -> dict:
    """Convert with error/warning details."""
    return {
        "mathml": "...",
        "errors": [...],
        "warnings": [...]
    }
```

---

## Future Considerations

### 1. Machine Learning Enhancements

For ambiguous cases, ML models could learn from:
- User corrections
- Document context
- Common mathematical patterns

### 2. Bi-Directional Editing

Synchronize braille and visual math editors:
- Edit in braille, see visual update
- Edit visually, see braille update
- Cursor position synchronization

### 3. Voice Integration

Combine with speech input:
- "Open fraction, x, over, y, close fraction"
- Confirm with braille display

### 4. LaTeX Output

Direct braille-to-LaTeX for academic writing:
```rust
pub fn braille_to_latex(braille: &str, code: &str) -> Result<String>;
```

### 5. Accessibility Validation

Verify back-translated MathML is accessible:
- Check for proper semantics
- Validate intent attributes
- Ensure round-trip stability

---

## References and Resources

### Official Specifications

- [Nemeth Code 2022 (BANA)](https://www.brailleauthority.org/sites/default/files/2024-02/Nemeth_2022.pdf)
- [UEB Guidelines for Technical Material (ICEB)](http://iceb.org/ueb.html)
- [Nemeth within UEB Guidance (BANA)](https://www.brailleauthority.org/ueb/nemeth-guidance/Nemeth%20Guidance%20Final.pdf)

### Academic Papers

- Susan Jolly, "Nemeth braille math and LaTeX source as braille", [TUGboat Vol 40 No 1 (2019)](https://www.tug.org/TUGboat/tb40-1/tb124jolly-braille.pdf)
- "SBT: A Translator from Spanish Mathematical Braille to MathML", [SpringerLink](https://link.springer.com/chapter/10.1007/11788713_174)
- "Translating MathML into Nemeth Braille Code", [SpringerLink](https://link.springer.com/chapter/10.1007/11788713_170)

### Related Projects

- [MML2Nem (Susan Jolly)](https://github.com/SusanJ/MML2Nem) - ANTLR-based MathML to Nemeth
- [liblouis](https://liblouis.io/) - Open-source braille translator
- [Speech Rule Engine](https://github.com/Speech-Rule-Engine/speech-rule-engine) - MathML to speech/braille
- [The Equalize Editor](https://knowbility.org/programs/be-a-digital-ally/december-2025) - Web-based braille math editor

### Rust Parser Resources

- [pest Book](https://pest.rs/book/) - PEG parser tutorial
- [LALRPOP Guide](https://github.com/lalrpop/lalrpop) - LR(1) parser generator
- [tree-sitter](https://tree-sitter.github.io/tree-sitter/) - Incremental parsing library
- [nom](https://docs.rs/nom/latest/nom/) - Parser combinators

### MathCAT Resources

- [MathCAT Documentation](https://nsoiffer.github.io/MathCAT/)
- [MathCAT GitHub](https://github.com/NSoiffer/MathCAT)
- Existing braille rules: `Rules/Braille/Nemeth/`, `Rules/Braille/UEB/`

---

## Appendix A: Nemeth Symbol Quick Reference

| Math Concept | Nemeth Braille | Unicode Sequence |
|--------------|----------------|------------------|
| Plus | `...` | U+282E |
| Minus | `...` | U+2824 |
| Times | `...` | U+2810 |
| Divide | `...` | U+280C |
| Equals | `...` | U+2836 |
| Open Paren | `...` | U+2837 |
| Close Paren | `...` | U+283E |
| Fraction Open | `...` | U+2820 U+2839 |
| Fraction Close | `...` | U+2820 U+283C |
| Numeric Indicator | `...` | U+283C |
| Capital | `...` | U+2820 |

(Refer to `Rules/Braille/Nemeth/unicode.yaml` for complete mappings)

---

## Appendix B: Sample Grammar Sketch (pest)

```pest
// nemeth_grammar.pest - Partial example

WHITESPACE = _{ " " | "\t" | "\n" | "\r" }

// Entry point
math = { SOI ~ expression ~ EOI }

// Expressions with operator precedence (handled by Pratt parser)
expression = { prefix* ~ primary ~ (infix ~ prefix* ~ primary)* }

infix = _{ add | subtract | multiply | divide | equals | comparison }
prefix = _{ negative }

add = { "\u{282E}" }           // plus
subtract = { "\u{2824}" }      // minus
multiply = { "\u{2810}" }      // times
divide = { "\u{280C}" }        // division
equals = { "\u{2836}" }        // equals
negative = { "\u{2824}" }      // negative sign

primary = _{
    fraction
    | radical
    | superscript
    | subscript
    | grouped
    | number
    | letter
    | greek
}

// Fractions
fraction = {
    fraction_open ~ expression ~ fraction_over ~ expression ~ fraction_close
}
fraction_open = { "\u{2820}\u{2839}" }
fraction_over = { "\u{280C}" }
fraction_close = { "\u{2820}\u{283C}" }

// Numbers
number = { numeric_indicator? ~ digit+ ~ ("." ~ digit+)? }
numeric_indicator = { "\u{283C}" }
digit = {
    "\u{2801}" | "\u{2803}" | "\u{2809}" | "\u{2819}" | "\u{2811}" |  // 1-5
    "\u{280B}" | "\u{281B}" | "\u{2813}" | "\u{280A}" | "\u{281A}"    // 6-9, 0
}

// Letters (variables)
letter = { capital? ~ letter_char }
capital = { "\u{2820}" }
letter_char = { 'a'..'z' }  // Simplified - needs proper Nemeth letter mappings

// Grouping
grouped = {
    open_paren ~ expression ~ close_paren
    | open_bracket ~ expression ~ close_bracket
}
open_paren = { "\u{2837}" }
close_paren = { "\u{283E}" }
open_bracket = { "\u{2808}\u{2837}" }
close_bracket = { "\u{2808}\u{283E}" }

// Radicals
radical = { radical_start ~ expression ~ radical_end }
radical_start = { "\u{281C}" }
radical_end = { "\u{283B}" }

// Scripts (simplified)
superscript = { base ~ superscript_indicator ~ script_content }
subscript = { base ~ subscript_indicator ~ script_content }
superscript_indicator = { "\u{2818}" }
subscript_indicator = { "\u{2830}" }
script_content = { primary }
base = { letter | number | grouped }
```

---

*This proposal is a living document. Feedback and contributions welcome via GitHub Issues.*
