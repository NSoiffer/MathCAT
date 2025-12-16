//! UEB (Unified English Braille) Technical to MathML parser
//!
//! This module parses UEB technical/math braille and converts it to MathML.
//! Phase 3: UEB Technical Support

use pest::Parser;
use pest_derive::Parser;

use crate::back_translate::errors::{BackTranslationError, BackTranslationWarning, ParseResult};
use crate::back_translate::mathml_gen::{generate_mathml, MathMLOptions};
use crate::back_translate::semantic::{MathNode, IdentifierInfo, GreekLetter};

#[derive(Parser)]
#[grammar = "back_translate/ueb.pest"]
struct UEBParser;

/// Parse UEB braille and convert to MathML
pub fn parse_ueb(braille: &str) -> ParseResult {
    if braille.trim().is_empty() {
        return ParseResult::failure(BackTranslationError::EmptyInput);
    }

    // Pre-process to validate braille characters
    let validation = validate_braille(braille);
    if !validation.errors.is_empty() {
        return validation;
    }

    // Parse using pest
    let parse_result = UEBParser::parse(Rule::math, braille);

    match parse_result {
        Ok(mut pairs) => {
            let math_pair = pairs.next().unwrap();
            let mut warnings = Vec::new();
            match build_ast(math_pair, &mut warnings) {
                Ok(ast) => {
                    let mathml = generate_mathml(&ast, &MathMLOptions::default());
                    if warnings.is_empty() {
                        ParseResult::success(mathml)
                    } else {
                        ParseResult::partial(mathml, warnings)
                    }
                }
                Err(e) => ParseResult::failure(e),
            }
        }
        Err(e) => {
            let position = match e.location {
                pest::error::InputLocation::Pos(p) => Some(p),
                pest::error::InputLocation::Span((s, _)) => Some(s),
            };

            // Attempt error recovery
            if let Some(recovered) = attempt_error_recovery(braille, position) {
                return recovered;
            }

            ParseResult::failure(BackTranslationError::ParseError {
                message: format_pest_error(&e),
                position,
            })
        }
    }
}

/// Validate that all characters are valid braille
fn validate_braille(braille: &str) -> ParseResult {
    let mut errors = Vec::new();

    for (pos, ch) in braille.char_indices() {
        // Valid braille Unicode range: U+2800 to U+28FF
        // Also allow ASCII space, tab, newline, carriage return
        if !('\u{2800}'..='\u{28FF}').contains(&ch)
            && ch != ' ' && ch != '\t' && ch != '\n' && ch != '\r'
        {
            errors.push(BackTranslationError::UnrecognizedSymbol {
                position: pos,
                braille: ch.to_string(),
            });
        }
    }

    if errors.is_empty() {
        ParseResult {
            mathml: None,
            errors: vec![],
            warnings: vec![],
        }
    } else {
        ParseResult {
            mathml: None,
            errors,
            warnings: vec![],
        }
    }
}

/// Format pest error for better user messages
fn format_pest_error(e: &pest::error::Error<Rule>) -> String {
    let line_col = match &e.line_col {
        pest::error::LineColLocation::Pos((l, c)) => format!("line {}, column {}", l, c),
        pest::error::LineColLocation::Span((l1, c1), (l2, c2)) => {
            format!("from line {} col {} to line {} col {}", l1, c1, l2, c2)
        }
    };

    match &e.variant {
        pest::error::ErrorVariant::ParsingError { positives, negatives } => {
            let expected = if !positives.is_empty() {
                format!("Expected: {:?}", positives.iter().map(rule_name).collect::<Vec<_>>())
            } else {
                String::new()
            };
            let unexpected = if !negatives.is_empty() {
                format!("Unexpected: {:?}", negatives.iter().map(rule_name).collect::<Vec<_>>())
            } else {
                String::new()
            };
            format!("Parse error at {}: {} {}", line_col, expected, unexpected).trim().to_string()
        }
        pest::error::ErrorVariant::CustomError { message } => {
            format!("Error at {}: {}", line_col, message)
        }
    }
}

/// Get a human-readable name for a rule
fn rule_name(rule: &Rule) -> &'static str {
    match rule {
        Rule::math => "math expression",
        Rule::expression => "expression",
        Rule::term => "term",
        Rule::atom => "atom",
        Rule::number => "number",
        Rule::letter => "letter",
        Rule::fraction => "fraction",
        Rule::radical => "radical",
        Rule::grouped => "grouped expression",
        Rule::operator => "operator",
        Rule::superscript => "superscript",
        Rule::subscript => "subscript",
        Rule::greek_letter => "Greek letter",
        Rule::special_symbol => "special symbol",
        _ => "element",
    }
}

/// Attempt to recover from parse errors
fn attempt_error_recovery(braille: &str, _position: Option<usize>) -> Option<ParseResult> {
    // Try removing trailing incomplete structures
    let trimmed = braille.trim_end_matches(|c| {
        matches!(c, '\u{2837}' | '\u{280C}' | '\u{2810}' | '\u{2814}' | '\u{2822}')
    });

    if trimmed != braille && !trimmed.is_empty() {
        let result = UEBParser::parse(Rule::math, trimmed);
        if let Ok(mut pairs) = result {
            if let Some(pair) = pairs.next() {
                let mut warnings = vec![BackTranslationWarning::AutoInserted {
                    element: "truncated incomplete structure".to_string(),
                    position: trimmed.len(),
                }];
                if let Ok(ast) = build_ast(pair, &mut warnings) {
                    let mathml = generate_mathml(&ast, &MathMLOptions::default());
                    return Some(ParseResult::partial(mathml, warnings));
                }
            }
        }
    }

    None
}

/// Build AST from pest parse pairs
fn build_ast(pair: pest::iterators::Pair<Rule>, warnings: &mut Vec<BackTranslationWarning>) -> Result<MathNode, BackTranslationError> {
    match pair.as_rule() {
        Rule::math => {
            let inner = pair.into_inner().next().unwrap();
            build_ast(inner, warnings)
        }

        Rule::expression => {
            let mut nodes: Vec<MathNode> = Vec::new();
            for inner in pair.into_inner() {
                let node = build_ast(inner, warnings)?;
                if !node.is_empty() {
                    nodes.push(node);
                }
            }
            Ok(MathNode::row_or_single(nodes))
        }

        Rule::term => {
            let inner = pair.into_inner().next().unwrap();
            build_ast(inner, warnings)
        }

        Rule::atom => {
            let inner = pair.into_inner().next().unwrap();
            build_ast(inner, warnings)
        }

        Rule::scripted_atom => {
            let mut inner = pair.into_inner();
            let base = build_ast(inner.next().unwrap(), warnings)?;

            let mut result = base;
            for script_pair in inner {
                let script_rule = script_pair.as_rule();
                let mut script_inner = script_pair.into_inner();
                let actual_script = script_inner.next().unwrap();

                result = match actual_script.as_rule() {
                    Rule::superscript => {
                        let script_content = actual_script.into_inner().nth(1);
                        if let Some(content) = script_content {
                            let script = build_ast(content, warnings)?;
                            MathNode::Superscript {
                                base: Box::new(result),
                                superscript: Box::new(script),
                            }
                        } else {
                            result
                        }
                    }
                    Rule::subscript => {
                        let script_content = actual_script.into_inner().nth(1);
                        if let Some(content) = script_content {
                            let script = build_ast(content, warnings)?;
                            MathNode::Subscript {
                                base: Box::new(result),
                                subscript: Box::new(script),
                            }
                        } else {
                            result
                        }
                    }
                    _ => {
                        if script_rule == Rule::script {
                            continue;
                        }
                        result
                    }
                };
            }
            Ok(result)
        }

        Rule::script => {
            let inner = pair.into_inner().next().unwrap();
            build_ast(inner, warnings)
        }

        Rule::number => {
            let mut number_str = String::new();
            for inner in pair.into_inner() {
                match inner.as_rule() {
                    Rule::ueb_digit => {
                        let digit = ueb_braille_to_digit(inner.as_str());
                        number_str.push(digit);
                    }
                    Rule::decimal_point => {
                        number_str.push('.');
                    }
                    Rule::numeric_indicator => {
                        // Skip numeric indicator
                    }
                    _ => {}
                }
            }
            Ok(MathNode::Number(number_str))
        }

        Rule::letter => {
            let mut is_capital = false;
            let mut letter_char = 'x';

            for inner in pair.into_inner() {
                match inner.as_rule() {
                    Rule::capital_indicator => {
                        is_capital = true;
                    }
                    Rule::letter_sign | Rule::grade1_symbol_indicator | Rule::grade1_word_indicator => {
                        // Skip indicators
                    }
                    Rule::letter_char => {
                        letter_char = ueb_braille_to_letter(inner.as_str());
                    }
                    _ => {}
                }
            }

            let mut info = IdentifierInfo::new(&letter_char.to_string());
            if is_capital {
                info = info.with_capital();
            }
            Ok(MathNode::Identifier(info))
        }

        Rule::greek_letter => {
            let mut is_capital = false;
            let mut greek_char = '\u{03B1}'; // alpha

            for inner in pair.into_inner() {
                match inner.as_rule() {
                    Rule::greek_indicator => {
                        // Skip Greek indicator
                    }
                    Rule::capital_indicator => {
                        is_capital = true;
                    }
                    Rule::greek_char => {
                        greek_char = ueb_braille_to_greek(inner.as_str(), is_capital);
                    }
                    _ => {}
                }
            }

            Ok(MathNode::Greek(if is_capital {
                GreekLetter::uppercase(greek_char)
            } else {
                GreekLetter::lowercase(greek_char)
            }))
        }

        Rule::special_symbol => {
            let inner = pair.into_inner().next().unwrap();
            let symbol = match inner.as_rule() {
                Rule::infinity => "\u{221E}",
                Rule::empty_set => "\u{2205}",
                Rule::element_of => "\u{2208}",
                Rule::not_element_of => "\u{2209}",
                Rule::forall => "\u{2200}",
                Rule::exists => "\u{2203}",
                Rule::nabla => "\u{2207}",
                Rule::partial_derivative => "\u{2202}",
                Rule::degree => "\u{00B0}",
                Rule::percent => "%",
                Rule::therefore => "\u{2234}",
                Rule::because => "\u{2235}",
                Rule::prime => "\u{2032}",
                Rule::double_prime => "\u{2033}",
                _ => "?",
            };
            Ok(MathNode::Operator(symbol.to_string()))
        }

        Rule::fraction => {
            let mut inner = pair.into_inner().peekable();
            // Skip fraction_open
            while let Some(p) = inner.peek() {
                match p.as_rule() {
                    Rule::fraction_open => { inner.next(); }
                    _ => break,
                }
            }
            let numerator = build_ast(inner.next().unwrap(), warnings)?;
            // Skip fraction_line
            while let Some(p) = inner.peek() {
                match p.as_rule() {
                    Rule::fraction_line => { inner.next(); }
                    _ => break,
                }
            }
            let denominator = build_ast(inner.next().unwrap(), warnings)?;

            Ok(MathNode::Fraction {
                numerator: Box::new(numerator),
                denominator: Box::new(denominator),
            })
        }

        Rule::radical => {
            let mut inner = pair.into_inner().peekable();

            let first = inner.next().unwrap();
            if first.as_rule() == Rule::radical_index {
                // nth root
                let index_inner = first.into_inner().nth(1).unwrap();
                let index = build_ast(index_inner, warnings)?;

                // Skip radical_start
                while let Some(p) = inner.peek() {
                    match p.as_rule() {
                        Rule::radical_start => { inner.next(); }
                        _ => break,
                    }
                }
                let radicand = build_ast(inner.next().unwrap(), warnings)?;

                Ok(MathNode::Radical {
                    index: Some(Box::new(index)),
                    radicand: Box::new(radicand),
                })
            } else {
                // Square root
                while let Some(p) = inner.peek() {
                    match p.as_rule() {
                        Rule::radical_start => { inner.next(); }
                        _ => break,
                    }
                }
                let radicand_pair = if first.as_rule() == Rule::expression {
                    first
                } else {
                    inner.next().unwrap()
                };
                let radicand = build_ast(radicand_pair, warnings)?;

                Ok(MathNode::Radical {
                    index: None,
                    radicand: Box::new(radicand),
                })
            }
        }

        Rule::grouped => {
            let mut inner = pair.into_inner();
            let open = inner.next().unwrap();
            let (open_char, close_char) = match open.as_rule() {
                Rule::open_paren => ("(", ")"),
                Rule::open_bracket => ("[", "]"),
                Rule::open_brace => ("{", "}"),
                Rule::open_angle => ("\u{27E8}", "\u{27E9}"),
                _ => ("(", ")"),
            };

            let content = build_ast(inner.next().unwrap(), warnings)?;

            Ok(MathNode::Grouped {
                open: open_char.to_string(),
                close: close_char.to_string(),
                content: Box::new(content),
            })
        }

        Rule::operator => {
            let inner = pair.into_inner().next().unwrap();
            let actual_op = inner.into_inner().next().unwrap();
            build_operator(actual_op)
        }

        Rule::comparison_operator | Rule::set_operator | Rule::logical_operator
        | Rule::arrow_operator | Rule::arithmetic_operator => {
            let inner = pair.into_inner().next().unwrap();
            build_operator(inner)
        }

        Rule::superscript => {
            let content = pair.into_inner().nth(1);
            if let Some(c) = content {
                build_ast(c, warnings)
            } else {
                Ok(MathNode::Empty)
            }
        }

        Rule::subscript => {
            let content = pair.into_inner().nth(1);
            if let Some(c) = content {
                build_ast(c, warnings)
            } else {
                Ok(MathNode::Empty)
            }
        }

        Rule::script_content => {
            let inner = pair.into_inner().next().unwrap();
            build_ast(inner, warnings)
        }

        // Skip indicator rules
        Rule::numeric_indicator
        | Rule::capital_indicator
        | Rule::letter_sign
        | Rule::grade1_symbol_indicator
        | Rule::grade1_word_indicator
        | Rule::greek_indicator
        | Rule::fraction_open
        | Rule::fraction_line
        | Rule::fraction_close
        | Rule::radical_start
        | Rule::radical_end
        | Rule::index_indicator
        | Rule::superscript_indicator
        | Rule::subscript_indicator => {
            Ok(MathNode::Empty)
        }

        _ => {
            Ok(MathNode::Empty)
        }
    }
}

/// Build operator node from rule
fn build_operator(pair: pest::iterators::Pair<Rule>) -> Result<MathNode, BackTranslationError> {
    let op = match pair.as_rule() {
        // Arithmetic
        Rule::plus => "+",
        Rule::minus => "-",
        Rule::times => "\u{00D7}",
        Rule::divide => "\u{00F7}",
        Rule::plus_minus => "\u{00B1}",
        Rule::minus_plus => "\u{2213}",
        Rule::dot_operator => "\u{22C5}",

        // Comparison
        Rule::equals => "=",
        Rule::not_equal => "\u{2260}",
        Rule::less_than => "<",
        Rule::greater_than => ">",
        Rule::less_equal => "\u{2264}",
        Rule::greater_equal => "\u{2265}",
        Rule::approximately_equal => "\u{2248}",
        Rule::congruent => "\u{2245}",
        Rule::equivalent => "\u{2261}",

        // Set operations
        Rule::subset => "\u{2282}",
        Rule::superset => "\u{2283}",
        Rule::subset_equal => "\u{2286}",
        Rule::superset_equal => "\u{2287}",
        Rule::element_of => "\u{2208}",
        Rule::not_element_of => "\u{2209}",
        Rule::union => "\u{222A}",
        Rule::intersection => "\u{2229}",

        // Logical
        Rule::logical_and => "\u{2227}",
        Rule::logical_or => "\u{2228}",
        Rule::logical_not => "\u{00AC}",
        Rule::implies => "\u{21D2}",
        Rule::iff => "\u{21D4}",
        Rule::forall => "\u{2200}",
        Rule::exists => "\u{2203}",

        // Arrows
        Rule::left_arrow => "\u{2190}",
        Rule::right_arrow => "\u{2192}",
        Rule::left_right_arrow => "\u{2194}",
        Rule::double_left_arrow => "\u{21D0}",
        Rule::double_right_arrow => "\u{21D2}",
        Rule::double_left_right_arrow => "\u{21D4}",

        _ => "?",
    };
    Ok(MathNode::Operator(op.to_string()))
}

/// Convert UEB braille digit to ASCII digit
fn ueb_braille_to_digit(braille: &str) -> char {
    match braille {
        "\u{2801}" => '1', // dots 1 (like 'a')
        "\u{2803}" => '2', // dots 12 (like 'b')
        "\u{2809}" => '3', // dots 14 (like 'c')
        "\u{2819}" => '4', // dots 145 (like 'd')
        "\u{2811}" => '5', // dots 15 (like 'e')
        "\u{280B}" => '6', // dots 124 (like 'f')
        "\u{281B}" => '7', // dots 1245 (like 'g')
        "\u{2813}" => '8', // dots 125 (like 'h')
        "\u{280A}" => '9', // dots 24 (like 'i')
        "\u{281A}" => '0', // dots 245 (like 'j')
        _ => '?',
    }
}

/// Convert UEB braille letter to ASCII letter
fn ueb_braille_to_letter(braille: &str) -> char {
    match braille {
        "\u{2801}" => 'a',
        "\u{2803}" => 'b',
        "\u{2809}" => 'c',
        "\u{2819}" => 'd',
        "\u{2811}" => 'e',
        "\u{280B}" => 'f',
        "\u{281B}" => 'g',
        "\u{2813}" => 'h',
        "\u{280A}" => 'i',
        "\u{281A}" => 'j',
        "\u{2805}" => 'k',
        "\u{2807}" => 'l',
        "\u{280D}" => 'm',
        "\u{281D}" => 'n',
        "\u{2815}" => 'o',
        "\u{280F}" => 'p',
        "\u{281F}" => 'q',
        "\u{2817}" => 'r',
        "\u{280E}" => 's',
        "\u{281E}" => 't',
        "\u{2825}" => 'u',
        "\u{2827}" => 'v',
        "\u{283A}" => 'w',
        "\u{282D}" => 'x',
        "\u{283D}" => 'y',
        "\u{2835}" => 'z',
        _ => '?',
    }
}

/// Convert UEB braille Greek character to Unicode Greek
fn ueb_braille_to_greek(braille: &str, is_capital: bool) -> char {
    let lowercase = match braille {
        "\u{2801}" => '\u{03B1}', // alpha
        "\u{2803}" => '\u{03B2}', // beta
        "\u{281B}" => '\u{03B3}', // gamma
        "\u{2819}" => '\u{03B4}', // delta
        "\u{2811}" => '\u{03B5}', // epsilon
        "\u{2835}" => '\u{03B6}', // zeta
        "\u{2831}" => '\u{03B7}', // eta
        "\u{2839}" => '\u{03B8}', // theta
        "\u{280A}" => '\u{03B9}', // iota
        "\u{2805}" => '\u{03BA}', // kappa
        "\u{2807}" => '\u{03BB}', // lambda
        "\u{280D}" => '\u{03BC}', // mu
        "\u{281D}" => '\u{03BD}', // nu
        "\u{282D}" => '\u{03BE}', // xi
        "\u{2815}" => '\u{03BF}', // omicron
        "\u{280F}" => '\u{03C0}', // pi
        "\u{2817}" => '\u{03C1}', // rho
        "\u{280E}" => '\u{03C3}', // sigma
        "\u{281E}" => '\u{03C4}', // tau
        "\u{2825}" => '\u{03C5}', // upsilon
        "\u{280B}" => '\u{03C6}', // phi
        "\u{282F}" => '\u{03C7}', // chi
        "\u{283D}" => '\u{03C8}', // psi
        "\u{283A}" => '\u{03C9}', // omega
        _ => '\u{03B1}',
    };

    if is_capital {
        match lowercase {
            '\u{03B1}'..='\u{03C1}' => char::from_u32(lowercase as u32 - 0x20).unwrap_or(lowercase),
            '\u{03C3}'..='\u{03C9}' => char::from_u32(lowercase as u32 - 0x20).unwrap_or(lowercase),
            _ => lowercase,
        }
    } else {
        lowercase
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_parses(braille: &str, expected_contains: &str) {
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(
            mathml.contains(expected_contains),
            "Expected '{}' in:\n{}",
            expected_contains,
            mathml
        );
    }

    // Basic tests
    #[test]
    fn test_empty_input() {
        let result = parse_ueb("");
        assert!(!result.is_success());
        assert!(matches!(
            result.errors.first(),
            Some(BackTranslationError::EmptyInput)
        ));
    }

    #[test]
    fn test_single_digit() {
        // UEB: numeric indicator + 1
        let braille = "\u{283C}\u{2801}";
        assert_parses(braille, "<mn>1</mn>");
    }

    #[test]
    fn test_multi_digit_number() {
        // UEB: numeric indicator + 1 + 2 + 3
        let braille = "\u{283C}\u{2801}\u{2803}\u{2809}";
        assert_parses(braille, "<mn>123</mn>");
    }

    #[test]
    fn test_single_letter() {
        // UEB: letter sign + x
        let braille = "\u{2830}\u{282D}";
        assert_parses(braille, "<mi>x</mi>");
    }

    #[test]
    fn test_capital_letter() {
        // UEB: capital indicator + letter sign + x
        let braille = "\u{2820}\u{2830}\u{282D}";
        assert_parses(braille, "<mi>X</mi>");
    }

    #[test]
    fn test_simple_addition() {
        // UEB: x + 1
        let braille = "\u{2830}\u{282D}\u{2810}\u{282E}\u{283C}\u{2801}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>+</mo>"));
        assert!(mathml.contains("<mn>1</mn>"));
    }

    #[test]
    fn test_simple_subtraction() {
        // UEB: x - 1
        let braille = "\u{2830}\u{282D}\u{2810}\u{2824}\u{283C}\u{2801}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>-</mo>"));
        assert!(mathml.contains("<mn>1</mn>"));
    }

    #[test]
    fn test_simple_fraction() {
        // UEB: fraction open + 1 + fraction line + 2 + fraction close
        let braille = "\u{2837}\u{283C}\u{2801}\u{280C}\u{283C}\u{2803}\u{283E}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mfrac>"));
        assert!(mathml.contains("<mn>1</mn>"));
        assert!(mathml.contains("<mn>2</mn>"));
    }

    #[test]
    fn test_parentheses() {
        // UEB: open paren + x + close paren
        let braille = "\u{2810}\u{2823}\u{2830}\u{282D}\u{2810}\u{281C}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mo>(</mo>"));
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>)</mo>"));
    }

    #[test]
    fn test_superscript() {
        // UEB: x + superscript indicator + 2
        let braille = "\u{2830}\u{282D}\u{2814}\u{283C}\u{2803}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<msup>"));
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mn>2</mn>"));
    }

    #[test]
    fn test_subscript() {
        // UEB: x + subscript indicator + 1
        let braille = "\u{2830}\u{282D}\u{2822}\u{283C}\u{2801}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<msub>"));
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mn>1</mn>"));
    }

    #[test]
    fn test_equals() {
        // UEB: x = 1
        let braille = "\u{2830}\u{282D}\u{2810}\u{2836}\u{283C}\u{2801}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>=</mo>"));
        assert!(mathml.contains("<mn>1</mn>"));
    }

    #[test]
    fn test_greek_alpha() {
        // UEB: Greek indicator + a (alpha)
        let braille = "\u{2828}\u{2801}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>\u{03B1}</mi>"));
    }

    #[test]
    fn test_greek_capital_delta() {
        // UEB: Greek indicator + capital indicator + d (Delta)
        let braille = "\u{2828}\u{2820}\u{2819}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>\u{0394}</mi>"));
    }

    #[test]
    fn test_pi() {
        // UEB: Greek indicator + p (pi)
        let braille = "\u{2828}\u{280F}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("\u{03C0}"), "Expected pi in: {}", mathml);
    }

    #[test]
    fn test_invalid_character() {
        let result = parse_ueb("abc");
        assert!(!result.is_success());
        assert!(matches!(
            result.errors.first(),
            Some(BackTranslationError::UnrecognizedSymbol { .. })
        ));
    }

    #[test]
    fn test_brackets() {
        // UEB: open bracket + x + close bracket
        let braille = "\u{2828}\u{2823}\u{2830}\u{282D}\u{2828}\u{281C}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mo>[</mo>"));
        assert!(mathml.contains("<mo>]</mo>"));
    }

    #[test]
    fn test_braces() {
        // UEB: open brace + x + close brace
        let braille = "\u{2838}\u{2823}\u{2830}\u{282D}\u{2838}\u{281C}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mo>{</mo>"));
        assert!(mathml.contains("<mo>}</mo>"));
    }

    #[test]
    fn test_multiplication() {
        // UEB: x times y
        let braille = "\u{2830}\u{282D}\u{2810}\u{282C}\u{2830}\u{283D}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("\u{00D7}") || mathml.contains("times"));
        assert!(mathml.contains("<mi>y</mi>"));
    }

    #[test]
    fn test_whitespace_handling() {
        // UEB: x + y with braille space
        let braille = "\u{2830}\u{282D}\u{2800}\u{2810}\u{282E}\u{2800}\u{2830}\u{283D}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
    }

    #[test]
    fn test_multiple_operations() {
        // UEB: a + b - c
        let braille = "\u{2830}\u{2801}\u{2810}\u{282E}\u{2830}\u{2803}\u{2810}\u{2824}\u{2830}\u{2809}";
        let result = parse_ueb(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>a</mi>"));
        assert!(mathml.contains("<mo>+</mo>"));
        assert!(mathml.contains("<mi>b</mi>"));
        assert!(mathml.contains("<mo>-</mo>"));
        assert!(mathml.contains("<mi>c</mi>"));
    }
}
