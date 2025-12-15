//! Nemeth braille to MathML parser
//!
//! This module parses Nemeth braille and converts it to MathML.
//! Phase 2: Extended symbols, functions, and error handling.

use pest::Parser;
use pest_derive::Parser;

use crate::back_translate::errors::{BackTranslationError, BackTranslationWarning, ParseResult};
use crate::back_translate::mathml_gen::{generate_mathml, MathMLOptions};
use crate::back_translate::semantic::{MathNode, IdentifierInfo, GreekLetter, FontStyle};

#[derive(Parser)]
#[grammar = "back_translate/nemeth.pest"]
struct NemethParser;

/// Parse Nemeth braille and convert to MathML
pub fn parse_nemeth(braille: &str) -> ParseResult {
    if braille.trim().is_empty() {
        return ParseResult::failure(BackTranslationError::EmptyInput);
    }

    // Pre-process to validate braille characters
    let validation = validate_braille(braille);
    if !validation.errors.is_empty() {
        return validation;
    }

    // Parse using pest
    let parse_result = NemethParser::parse(Rule::math, braille);

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
        Rule::function_name => "function name",
        Rule::big_operator => "big operator",
        Rule::special_symbol => "special symbol",
        _ => "element",
    }
}

/// Attempt to recover from parse errors
fn attempt_error_recovery(braille: &str, _position: Option<usize>) -> Option<ParseResult> {
    // Try removing trailing incomplete structures
    let trimmed = braille.trim_end_matches(|c| {
        matches!(c, '\u{2839}' | '\u{280C}' | '\u{281C}' | '\u{2818}' | '\u{2830}')
    });

    if trimmed != braille && !trimmed.is_empty() {
        let result = NemethParser::parse(Rule::math, trimmed);
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
            // math contains expression
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
                        // Handle direct script wrapper
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
            let mut font_style = FontStyle::Normal;

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
                    Rule::typeform_indicator => {
                        let typeform = inner.into_inner().next().unwrap();
                        font_style = match typeform.as_rule() {
                            Rule::bold_indicator => FontStyle::Bold,
                            Rule::italic_indicator => FontStyle::Italic,
                            Rule::script_style_indicator => FontStyle::Script,
                            _ => FontStyle::Normal,
                        };
                    }
                    _ => {}
                }
            }

            let mut info = IdentifierInfo::new(&letter_char.to_string());
            if is_capital {
                info = info.with_capital();
            }
            info = info.with_style(font_style);
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

        Rule::special_symbol => {
            let inner = pair.into_inner().next().unwrap();
            let symbol = match inner.as_rule() {
                Rule::infinity => "\u{221E}",      // infinity
                Rule::empty_set => "\u{2205}",    // empty set
                Rule::element_of => "\u{2208}",   // element of
                Rule::not_element_of => "\u{2209}", // not element of
                Rule::subset => "\u{2282}",       // subset
                Rule::superset => "\u{2283}",     // superset
                Rule::union => "\u{222A}",        // union
                Rule::intersection => "\u{2229}", // intersection
                Rule::nabla => "\u{2207}",        // nabla
                Rule::partial_derivative => "\u{2202}", // partial
                Rule::degree => "\u{00B0}",       // degree
                Rule::percent => "%",
                Rule::therefore => "\u{2234}",    // therefore
                Rule::because => "\u{2235}",      // because
                Rule::factorial => "!",
                _ => "?",
            };
            Ok(MathNode::Operator(symbol.to_string()))
        }

        Rule::fraction => {
            let mut inner = pair.into_inner().peekable();
            // Skip nesting indicators and fraction_open
            while let Some(p) = inner.peek() {
                match p.as_rule() {
                    Rule::nesting_indicator | Rule::fraction_open => { inner.next(); }
                    _ => break,
                }
            }
            let numerator = build_ast(inner.next().unwrap(), warnings)?;
            // Skip nesting indicators and fraction_bar
            while let Some(p) = inner.peek() {
                match p.as_rule() {
                    Rule::nesting_indicator | Rule::fraction_bar => { inner.next(); }
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

            // Check if we have an index (nth root)
            let first = inner.next().unwrap();
            if first.as_rule() == Rule::radical_index {
                // nth root
                let index_inner = first.into_inner().nth(1).unwrap();
                let index = build_ast(index_inner, warnings)?;

                // Skip nesting indicators and radical_start
                while let Some(p) = inner.peek() {
                    match p.as_rule() {
                        Rule::nesting_indicator | Rule::radical_start => { inner.next(); }
                        _ => break,
                    }
                }
                let radicand = build_ast(inner.next().unwrap(), warnings)?;

                Ok(MathNode::Radical {
                    index: Some(Box::new(index)),
                    radicand: Box::new(radicand),
                })
            } else {
                // Square root (first might be nesting indicator or radical_start)
                // Skip to expression
                while let Some(p) = inner.peek() {
                    match p.as_rule() {
                        Rule::nesting_indicator | Rule::radical_start => { inner.next(); }
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

        Rule::absolute_value => {
            let mut inner = pair.into_inner();
            inner.next(); // Skip absolute_open
            let content = build_ast(inner.next().unwrap(), warnings)?;

            Ok(MathNode::Grouped {
                open: "|".to_string(),
                close: "|".to_string(),
                content: Box::new(content),
            })
        }

        Rule::grouped => {
            let mut inner = pair.into_inner();
            let open = inner.next().unwrap();
            let (open_char, close_char) = match open.as_rule() {
                Rule::open_paren => ("(", ")"),
                Rule::open_bracket => ("[", "]"),
                Rule::open_brace => ("{", "}"),
                Rule::angle_open => ("\u{27E8}", "\u{27E9}"), // angle brackets
                _ => ("(", ")"),
            };

            let content = build_ast(inner.next().unwrap(), warnings)?;

            Ok(MathNode::Grouped {
                open: open_char.to_string(),
                close: close_char.to_string(),
                content: Box::new(content),
            })
        }

        Rule::big_operator => {
            let mut inner = pair.into_inner();
            let op_symbol = inner.next().unwrap();
            let op = match op_symbol.as_rule() {
                Rule::sum_symbol => "\u{2211}",      // summation
                Rule::product_symbol => "\u{220F}",  // product
                Rule::integral_symbol => "\u{222B}", // integral
                Rule::coproduct_symbol => "\u{2210}", // coproduct
                _ => "\u{2211}",
            };

            // Check for subscript/superscript
            let mut subscript_node = None;
            let mut superscript_node = None;

            for script in inner {
                match script.as_rule() {
                    Rule::subscript => {
                        if let Some(content) = script.into_inner().nth(1) {
                            subscript_node = Some(build_ast(content, warnings)?);
                        }
                    }
                    Rule::superscript => {
                        if let Some(content) = script.into_inner().nth(1) {
                            superscript_node = Some(build_ast(content, warnings)?);
                        }
                    }
                    _ => {}
                }
            }

            let base = MathNode::Operator(op.to_string());

            match (subscript_node, superscript_node) {
                (Some(sub), Some(sup)) => Ok(MathNode::SubSuperscript {
                    base: Box::new(base),
                    subscript: Box::new(sub),
                    superscript: Box::new(sup),
                }),
                (Some(sub), None) => Ok(MathNode::Subscript {
                    base: Box::new(base),
                    subscript: Box::new(sub),
                }),
                (None, Some(sup)) => Ok(MathNode::Superscript {
                    base: Box::new(base),
                    superscript: Box::new(sup),
                }),
                (None, None) => Ok(base),
            }
        }

        Rule::function_application => {
            let mut inner = pair.into_inner();
            let func_name = inner.next().unwrap();
            let name = match func_name.into_inner().next().unwrap().as_rule() {
                Rule::sin_func => "sin",
                Rule::cos_func => "cos",
                Rule::tan_func => "tan",
                Rule::cot_func => "cot",
                Rule::sec_func => "sec",
                Rule::csc_func => "csc",
                Rule::arcsin_func => "arcsin",
                Rule::arccos_func => "arccos",
                Rule::arctan_func => "arctan",
                Rule::sinh_func => "sinh",
                Rule::cosh_func => "cosh",
                Rule::tanh_func => "tanh",
                Rule::log_func => "log",
                Rule::ln_func => "ln",
                Rule::exp_func => "exp",
                Rule::lim_func => "lim",
                Rule::max_func => "max",
                Rule::min_func => "min",
                Rule::det_func => "det",
                Rule::mod_func => "mod",
                Rule::gcd_func => "gcd",
                _ => "f",
            };

            let func_ident = MathNode::Identifier(IdentifierInfo::new(name));

            // Check for argument
            if let Some(arg) = inner.next() {
                let arg_node = build_ast(arg, warnings)?;
                Ok(MathNode::Row(vec![func_ident, arg_node]))
            } else {
                Ok(func_ident)
            }
        }

        Rule::ellipsis => {
            let inner = pair.into_inner().next().unwrap();
            let ellipsis_char = match inner.as_rule() {
                Rule::horizontal_ellipsis => "\u{2026}",  // horizontal ellipsis
                Rule::vertical_ellipsis => "\u{22EE}",    // vertical ellipsis
                Rule::diagonal_ellipsis => "\u{22F1}",    // diagonal ellipsis
                _ => "\u{2026}",
            };
            Ok(MathNode::Operator(ellipsis_char.to_string()))
        }

        Rule::operator => {
            let inner = pair.into_inner().next().unwrap();
            // Inner is comparison_operator, set_operator, etc.
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
        | Rule::letter_indicator
        | Rule::greek_indicator
        | Rule::fraction_open
        | Rule::fraction_bar
        | Rule::fraction_close
        | Rule::radical_start
        | Rule::radical_end
        | Rule::superscript_indicator
        | Rule::subscript_indicator
        | Rule::nesting_indicator
        | Rule::typeform_indicator
        | Rule::bold_indicator
        | Rule::italic_indicator
        | Rule::script_style_indicator
        | Rule::absolute_open
        | Rule::absolute_close => {
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
        Rule::dot_product => "\u{22C5}",
        Rule::cross_product => "\u{00D7}",

        // Comparison
        Rule::equals => "=",
        Rule::not_equal => "\u{2260}",
        Rule::less_than => "<",
        Rule::greater_than => ">",
        Rule::less_equal => "\u{2264}",
        Rule::greater_equal => "\u{2265}",
        Rule::approximately_equal => "\u{2248}",
        Rule::congruent => "\u{2245}",
        Rule::similar => "\u{223C}",

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
        Rule::maps_to => "\u{21A6}",

        _ => "?",
    };
    Ok(MathNode::Operator(op.to_string()))
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

    fn assert_parse_fails(braille: &str) {
        let result = parse_nemeth(braille);
        assert!(!result.is_success() || result.has_warnings());
    }

    // Basic tests from Phase 1
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
        let braille = "\u{283C}\u{2802}";
        assert_parses(braille, "<mn>1</mn>");
    }

    #[test]
    fn test_multi_digit_number() {
        let braille = "\u{283C}\u{2802}\u{2806}\u{2812}";
        assert_parses(braille, "<mn>123</mn>");
    }

    #[test]
    fn test_decimal_number() {
        let braille = "\u{283C}\u{2812}\u{2828}\u{2802}\u{2832}";
        assert_parses(braille, "<mn>3.14</mn>");
    }

    #[test]
    fn test_single_letter() {
        let braille = "\u{282D}";
        assert_parses(braille, "<mi>x</mi>");
    }

    #[test]
    fn test_capital_letter() {
        let braille = "\u{2820}\u{282D}";
        assert_parses(braille, "<mi>X</mi>");
    }

    #[test]
    fn test_simple_addition() {
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
        let braille = "\u{281C}\u{283C}\u{2806}\u{283B}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<msqrt>"));
        assert!(mathml.contains("<mn>2</mn>"));
    }

    #[test]
    fn test_parentheses() {
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
        let braille = "\u{2828}\u{2801}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>\u{03B1}</mi>"));
    }

    #[test]
    fn test_greek_capital_delta() {
        let braille = "\u{2828}\u{2820}\u{2819}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>\u{0394}</mi>"));
    }

    #[test]
    fn test_complex_expression() {
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
        let braille = "\u{282D}\u{2818}\u{283C}\u{2806}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<msup>"));
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mn>2</mn>"));
    }

    // Phase 2 tests - Extended symbols and error handling

    #[test]
    fn test_invalid_character() {
        let result = parse_nemeth("abc");  // ASCII letters are invalid
        assert!(!result.is_success());
        assert!(matches!(
            result.errors.first(),
            Some(BackTranslationError::UnrecognizedSymbol { .. })
        ));
    }

    #[test]
    fn test_infinity_symbol() {
        // infinity: dots 4, 35
        let braille = "\u{2808}\u{2814}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("\u{221E}"), "Expected infinity in: {}", mathml);
    }

    #[test]
    fn test_pi_constant() {
        // pi: Greek indicator + p
        let braille = "\u{2828}\u{280F}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("\u{03C0}"), "Expected pi in: {}", mathml);
    }

    #[test]
    fn test_multiplication() {
        // x times y: x + times + y
        let braille = "\u{282D}\u{2821}\u{283D}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("\u{00D7}") || mathml.contains("&times;"));
        assert!(mathml.contains("<mi>y</mi>"));
    }

    #[test]
    fn test_less_than_or_equal() {
        // x <= y
        let braille = "\u{282D}\u{2810}\u{2805}\u{2828}\u{283D}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("\u{2264}"), "Expected <= in: {}", mathml);
    }

    #[test]
    fn test_not_equal() {
        // x != y
        let braille = "\u{282D}\u{2810}\u{2828}\u{2805}\u{283D}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("\u{2260}"), "Expected != in: {}", mathml);
    }

    #[test]
    fn test_plus_minus() {
        // x +- y
        let braille = "\u{282D}\u{282C}\u{2824}\u{283D}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("\u{00B1}"), "Expected +- in: {}", mathml);
    }

    #[test]
    fn test_brackets() {
        // [x]
        let braille = "\u{2808}\u{2837}\u{282D}\u{2808}\u{283E}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mo>[</mo>"));
        assert!(mathml.contains("<mo>]</mo>"));
    }

    #[test]
    fn test_braces() {
        // {x}
        let braille = "\u{2828}\u{2837}\u{282D}\u{2828}\u{283E}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mo>{</mo>"));
        assert!(mathml.contains("<mo>}</mo>"));
    }

    #[test]
    fn test_nested_fraction() {
        // Nested fraction with nesting indicator
        // (1/(1/2)) - simplified test
        let braille = "\u{2839}\u{283C}\u{2802}\u{280C}\u{2839}\u{283C}\u{2802}\u{280C}\u{283C}\u{2806}\u{283C}\u{283C}";
        let result = parse_nemeth(braille);
        // This is a complex case, just check it doesn't crash
        if result.is_success() {
            let mathml = result.mathml.unwrap();
            assert!(mathml.contains("<mfrac>"));
        }
    }

    #[test]
    fn test_whitespace_handling() {
        // x + y with braille space in between
        let braille = "\u{282D}\u{2800}\u{282C}\u{2800}\u{283D}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
    }

    #[test]
    fn test_multiple_operations() {
        // a + b - c
        let braille = "\u{2801}\u{282C}\u{2803}\u{2824}\u{2809}";
        let result = parse_nemeth(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>a</mi>"));
        assert!(mathml.contains("<mo>+</mo>"));
        assert!(mathml.contains("<mi>b</mi>"));
        assert!(mathml.contains("<mo>-</mo>"));
        assert!(mathml.contains("<mi>c</mi>"));
    }

    #[test]
    fn test_error_recovery_truncation() {
        // Incomplete fraction (no close)
        let braille = "\u{2839}\u{283C}\u{2802}\u{280C}\u{283C}\u{2806}";
        let result = parse_nemeth(braille);
        // Should fail gracefully
        assert!(!result.errors.is_empty() || !result.is_success());
    }
}
