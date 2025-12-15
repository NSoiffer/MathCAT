//! Nemeth braille to MathML parser
//!
//! This module parses Nemeth braille and converts it to MathML.

use pest::Parser;
use pest_derive::Parser;

use crate::back_translate::errors::{BackTranslationError, ParseResult};
use crate::back_translate::mathml_gen::{generate_mathml, MathMLOptions};
use crate::back_translate::semantic::{MathNode, IdentifierInfo, GreekLetter};

#[derive(Parser)]
#[grammar = "back_translate/nemeth.pest"]
struct NemethParser;

/// Parse Nemeth braille and convert to MathML
pub fn parse_nemeth(braille: &str) -> ParseResult {
    if braille.trim().is_empty() {
        return ParseResult::failure(BackTranslationError::EmptyInput);
    }

    // Parse using pest
    let parse_result = NemethParser::parse(Rule::math, braille);

    match parse_result {
        Ok(mut pairs) => {
            let math_pair = pairs.next().unwrap();
            match build_ast(math_pair) {
                Ok(ast) => {
                    let mathml = generate_mathml(&ast, &MathMLOptions::default());
                    ParseResult::success(mathml)
                }
                Err(e) => ParseResult::failure(e),
            }
        }
        Err(e) => {
            let position = match e.location {
                pest::error::InputLocation::Pos(p) => Some(p),
                pest::error::InputLocation::Span((s, _)) => Some(s),
            };
            ParseResult::failure(BackTranslationError::ParseError {
                message: format!("{}", e),
                position,
            })
        }
    }
}

/// Build AST from pest parse pairs
fn build_ast(pair: pest::iterators::Pair<Rule>) -> Result<MathNode, BackTranslationError> {
    match pair.as_rule() {
        Rule::math => {
            // math contains expression
            let inner = pair.into_inner().next().unwrap();
            build_ast(inner)
        }

        Rule::expression => {
            let mut nodes: Vec<MathNode> = Vec::new();
            for inner in pair.into_inner() {
                let node = build_ast(inner)?;
                nodes.push(node);
            }
            Ok(MathNode::row_or_single(nodes))
        }

        Rule::term => {
            let inner = pair.into_inner().next().unwrap();
            build_ast(inner)
        }

        Rule::atom => {
            let inner = pair.into_inner().next().unwrap();
            build_ast(inner)
        }

        Rule::scripted_atom => {
            let mut inner = pair.into_inner();
            let base = build_ast(inner.next().unwrap())?;

            let mut result = base;
            for script_pair in inner {
                result = match script_pair.as_rule() {
                    Rule::superscript => {
                        let script_content = script_pair.into_inner().nth(1).unwrap();
                        let script = build_ast(script_content)?;
                        MathNode::Superscript {
                            base: Box::new(result),
                            superscript: Box::new(script),
                        }
                    }
                    Rule::subscript => {
                        let script_content = script_pair.into_inner().nth(1).unwrap();
                        let script = build_ast(script_content)?;
                        MathNode::Subscript {
                            base: Box::new(result),
                            subscript: Box::new(script),
                        }
                    }
                    _ => result,
                };
            }
            Ok(result)
        }

        Rule::number => {
            let mut number_str = String::new();
            for inner in pair.into_inner() {
                match inner.as_rule() {
                    Rule::digit => {
                        let digit = braille_to_digit(inner.as_str());
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
                    Rule::letter_indicator => {
                        // Skip letter indicator
                    }
                    Rule::letter_char => {
                        letter_char = braille_to_letter(inner.as_str());
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
                        greek_char = braille_to_greek(inner.as_str(), is_capital);
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

        Rule::fraction => {
            let mut inner = pair.into_inner();
            // Skip fraction_open
            inner.next();
            let numerator = build_ast(inner.next().unwrap())?;
            // Skip fraction_bar
            inner.next();
            let denominator = build_ast(inner.next().unwrap())?;
            // Skip fraction_close

            Ok(MathNode::Fraction {
                numerator: Box::new(numerator),
                denominator: Box::new(denominator),
            })
        }

        Rule::radical => {
            let mut inner = pair.into_inner().peekable();

            // Check if we have an index (nth root)
            let first = inner.next().unwrap();
            if first.as_rule() == Rule::radical_index {
                // nth root
                let index_inner = first.into_inner().nth(1).unwrap();
                let index = build_ast(index_inner)?;

                // Skip radical_start
                inner.next();
                let radicand = build_ast(inner.next().unwrap())?;
                // Skip radical_end

                Ok(MathNode::Radical {
                    index: Some(Box::new(index)),
                    radicand: Box::new(radicand),
                })
            } else {
                // Square root (first is radical_start)
                let radicand = build_ast(inner.next().unwrap())?;
                // Skip radical_end

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
                _ => ("(", ")"),
            };

            let content = build_ast(inner.next().unwrap())?;
            // Skip close

            Ok(MathNode::Grouped {
                open: open_char.to_string(),
                close: close_char.to_string(),
                content: Box::new(content),
            })
        }

        Rule::operator => {
            let inner = pair.into_inner().next().unwrap();
            let op = match inner.as_rule() {
                Rule::plus => "+",
                Rule::minus => "-",
                Rule::times => "\u{00D7}",  // multiplication sign
                Rule::divide => "\u{00F7}", // division sign
                Rule::equals => "=",
                Rule::less_than => "<",
                Rule::greater_than => ">",
                Rule::less_equal => "\u{2264}",    // less than or equal
                Rule::greater_equal => "\u{2265}", // greater than or equal
                Rule::not_equal => "\u{2260}",     // not equal
                Rule::plus_minus => "\u{00B1}",    // plus-minus
                _ => "?",
            };
            Ok(MathNode::Operator(op.to_string()))
        }

        Rule::script_content => {
            let inner = pair.into_inner().next().unwrap();
            build_ast(inner)
        }

        // Skip indicator rules when encountered directly
        Rule::numeric_indicator
        | Rule::capital_indicator
        | Rule::letter_indicator
        | Rule::greek_indicator
        | Rule::fraction_open
        | Rule::fraction_bar
        | Rule::fraction_close
        | Rule::radical_start
        | Rule::radical_end
        | Rule::superscript_indicator
        | Rule::subscript_indicator => {
            Ok(MathNode::Empty)
        }

        _ => {
            Ok(MathNode::Empty)
        }
    }
}

/// Convert Nemeth braille digit to ASCII digit
fn braille_to_digit(braille: &str) -> char {
    match braille {
        "\u{2802}" => '1',
        "\u{2806}" => '2',
        "\u{2812}" => '3',
        "\u{2832}" => '4',
        "\u{2822}" => '5',
        "\u{2816}" => '6',
        "\u{2836}" => '7',
        "\u{2826}" => '8',
        "\u{2814}" => '9',
        "\u{2834}" => '0',
        _ => '?',
    }
}

/// Convert Nemeth braille letter to ASCII letter
fn braille_to_letter(braille: &str) -> char {
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

/// Convert Nemeth braille Greek character to Unicode Greek
fn braille_to_greek(braille: &str, is_capital: bool) -> char {
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
        _ => '\u{03B1}',          // default to alpha
    };

    if is_capital {
        // Convert lowercase Greek to uppercase (subtract 0x20 for most)
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
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(
            mathml.contains(expected_contains),
            "Expected '{}' in:\n{}",
            expected_contains,
            mathml
        );
    }

    #[test]
    fn test_empty_input() {
        let result = parse_nemeth("");
        assert!(!result.is_success());
        assert!(matches!(
            result.errors.first(),
            Some(BackTranslationError::EmptyInput)
        ));
    }

    #[test]
    fn test_single_digit() {
        // Numeric indicator + digit 1
        let braille = "\u{283C}\u{2802}";
        assert_parses(braille, "<mn>1</mn>");
    }

    #[test]
    fn test_multi_digit_number() {
        // Numeric indicator + 1 + 2 + 3
        let braille = "\u{283C}\u{2802}\u{2806}\u{2812}";
        assert_parses(braille, "<mn>123</mn>");
    }

    #[test]
    fn test_decimal_number() {
        // Numeric indicator + 3 + decimal + 1 + 4
        let braille = "\u{283C}\u{2812}\u{2828}\u{2802}\u{2832}";
        assert_parses(braille, "<mn>3.14</mn>");
    }

    #[test]
    fn test_single_letter() {
        // Letter 'x'
        let braille = "\u{282D}";
        assert_parses(braille, "<mi>x</mi>");
    }

    #[test]
    fn test_capital_letter() {
        // Capital indicator + letter 'x'
        let braille = "\u{2820}\u{282D}";
        assert_parses(braille, "<mi>X</mi>");
    }

    #[test]
    fn test_simple_addition() {
        // x + 1
        let braille = "\u{282D}\u{282C}\u{283C}\u{2802}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>+</mo>"));
        assert!(mathml.contains("<mn>1</mn>"));
    }

    #[test]
    fn test_simple_subtraction() {
        // x - 1
        let braille = "\u{282D}\u{2824}\u{283C}\u{2802}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>-</mo>"));
        assert!(mathml.contains("<mn>1</mn>"));
    }

    #[test]
    fn test_simple_fraction() {
        // 1/2 in Nemeth: fraction_open + 1 + fraction_bar + 2 + fraction_close
        let braille = "\u{2839}\u{283C}\u{2802}\u{280C}\u{283C}\u{2806}\u{283C}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mfrac>"));
        assert!(mathml.contains("<mn>1</mn>"));
        assert!(mathml.contains("<mn>2</mn>"));
    }

    #[test]
    fn test_square_root() {
        // sqrt(2): radical_start + 2 + radical_end
        let braille = "\u{281C}\u{283C}\u{2806}\u{283B}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<msqrt>"));
        assert!(mathml.contains("<mn>2</mn>"));
    }

    #[test]
    fn test_parentheses() {
        // (x)
        let braille = "\u{2837}\u{282D}\u{283E}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mo>(</mo>"));
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>)</mo>"));
    }

    #[test]
    fn test_superscript() {
        // x^2: x + superscript_indicator + 2
        let braille = "\u{282D}\u{2818}\u{283C}\u{2806}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<msup>"));
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mn>2</mn>"));
    }

    #[test]
    fn test_subscript() {
        // x_1: x + subscript_indicator + 1
        let braille = "\u{282D}\u{2830}\u{283C}\u{2802}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<msub>"));
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mn>1</mn>"));
    }

    #[test]
    fn test_equals() {
        // x = 1
        let braille = "\u{282D}\u{2828}\u{2805}\u{283C}\u{2802}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>=</mo>"));
        assert!(mathml.contains("<mn>1</mn>"));
    }

    #[test]
    fn test_greek_alpha() {
        // Greek alpha: greek_indicator + a
        let braille = "\u{2828}\u{2801}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>\u{03B1}</mi>")); // alpha
    }

    #[test]
    fn test_greek_capital_delta() {
        // Greek capital delta: greek_indicator + capital + d
        let braille = "\u{2828}\u{2820}\u{2819}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>\u{0394}</mi>")); // Delta
    }

    #[test]
    fn test_complex_expression() {
        // x + y = z: x + plus + y + equals + z
        let braille = "\u{282D}\u{282C}\u{283D}\u{2828}\u{2805}\u{2835}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>+</mo>"));
        assert!(mathml.contains("<mi>y</mi>"));
        assert!(mathml.contains("<mo>=</mo>"));
        assert!(mathml.contains("<mi>z</mi>"));
    }

    #[test]
    fn test_quadratic_formula_parts() {
        // Test x^2: x + superscript + 2
        let braille = "\u{282D}\u{2818}\u{283C}\u{2806}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<msup>"));
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mn>2</mn>"));
    }
}
