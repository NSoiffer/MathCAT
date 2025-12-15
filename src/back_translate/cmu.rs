//! CMU (Spanish Mathematical Braille) Parser
//!
//! This module implements back-translation from CMU braille to MathML.
//! CMU is the Code Matematico Unificado used in Spanish-speaking countries.
//!
//! Phase 5: Additional Codes - CMU Spanish Support

use pest::Parser;
use pest_derive::Parser;

use crate::back_translate::errors::{BackTranslationError, BackTranslationWarning, ParseResult};
use crate::back_translate::mathml_gen::{generate_mathml, MathMLOptions};
use crate::back_translate::semantic::{MathNode, IdentifierInfo, GreekLetter};

#[derive(Parser)]
#[grammar = "back_translate/cmu.pest"]
struct CMUParser;

/// Parse CMU braille and convert to MathML
pub fn parse_cmu(braille: &str) -> ParseResult {
    if braille.trim().is_empty() {
        return ParseResult::failure(BackTranslationError::EmptyInput);
    }

    let mut warnings: Vec<BackTranslationWarning> = Vec::new();

    // Try to parse with pest grammar
    match CMUParser::parse(Rule::math, braille) {
        Ok(pairs) => {
            match build_ast(pairs, &mut warnings) {
                Ok(node) => {
                    let mathml = generate_mathml(&node, &MathMLOptions::default());
                    ParseResult {
                        mathml: Some(mathml),
                        errors: vec![],
                        warnings,
                    }
                }
                Err(e) => ParseResult {
                    mathml: None,
                    errors: vec![e],
                    warnings,
                },
            }
        }
        Err(e) => {
            // Fall back to direct interpretation
            match interpret_cmu_direct(braille, &mut warnings) {
                Ok(node) => {
                    let mathml = generate_mathml(&node, &MathMLOptions::default());
                    ParseResult {
                        mathml: Some(mathml),
                        errors: vec![],
                        warnings,
                    }
                }
                Err(_) => ParseResult {
                    mathml: None,
                    errors: vec![BackTranslationError::ParseError {
                        message: format!("CMU parse error: {}", e),
                        position: None,
                    }],
                    warnings,
                },
            }
        }
    }
}

/// Build AST from pest parse tree
fn build_ast(
    pairs: pest::iterators::Pairs<Rule>,
    warnings: &mut Vec<BackTranslationWarning>,
) -> Result<MathNode, BackTranslationError> {
    let mut nodes: Vec<MathNode> = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::math => {
                return build_ast(pair.into_inner(), warnings);
            }
            Rule::expression => {
                let node = build_expression(pair.into_inner(), warnings)?;
                nodes.push(node);
            }
            Rule::EOI => {}
            _ => {
                let node = build_term_from_pair(pair, warnings)?;
                nodes.push(node);
            }
        }
    }

    if nodes.is_empty() {
        return Err(BackTranslationError::ParseError {
            message: "Empty expression".to_string(),
            position: None,
        });
    }

    if nodes.len() == 1 {
        Ok(nodes.remove(0))
    } else {
        Ok(MathNode::Row(nodes))
    }
}

/// Build expression from terms and operators
fn build_expression(
    pairs: pest::iterators::Pairs<Rule>,
    warnings: &mut Vec<BackTranslationWarning>,
) -> Result<MathNode, BackTranslationError> {
    let mut nodes: Vec<MathNode> = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::term => {
                let node = build_term(pair.into_inner(), warnings)?;
                nodes.push(node);
            }
            Rule::operator | Rule::arithmetic_operator | Rule::comparison_operator
            | Rule::set_operator | Rule::logical_operator | Rule::arrow_operator => {
                let op = parse_operator(pair)?;
                nodes.push(MathNode::Operator(op));
            }
            _ => {
                let node = build_term_from_pair(pair, warnings)?;
                nodes.push(node);
            }
        }
    }

    if nodes.len() == 1 {
        Ok(nodes.remove(0))
    } else {
        Ok(MathNode::Row(nodes))
    }
}

/// Build term from inner pairs
fn build_term(
    pairs: pest::iterators::Pairs<Rule>,
    warnings: &mut Vec<BackTranslationWarning>,
) -> Result<MathNode, BackTranslationError> {
    let mut nodes: Vec<MathNode> = Vec::new();

    for pair in pairs {
        let node = build_term_from_pair(pair, warnings)?;
        nodes.push(node);
    }

    if nodes.len() == 1 {
        Ok(nodes.remove(0))
    } else {
        Ok(MathNode::Row(nodes))
    }
}

/// Build a term from a single pair
fn build_term_from_pair(
    pair: pest::iterators::Pair<Rule>,
    warnings: &mut Vec<BackTranslationWarning>,
) -> Result<MathNode, BackTranslationError> {
    match pair.as_rule() {
        Rule::number => {
            let num = parse_cmu_number(pair.into_inner())?;
            Ok(MathNode::Number(num))
        }
        Rule::letter => {
            let (letter, is_capital) = parse_cmu_letter(pair.into_inner())?;
            let mut info = IdentifierInfo::new(&letter);
            if is_capital {
                info = info.with_capital();
            }
            Ok(MathNode::Identifier(info))
        }
        Rule::greek_letter => {
            let greek = parse_greek_letter(pair.into_inner())?;
            Ok(MathNode::Greek(greek))
        }
        Rule::grouped => {
            build_grouped(pair.into_inner(), warnings)
        }
        Rule::fraction => {
            build_fraction(pair.into_inner(), warnings)
        }
        Rule::radical => {
            build_radical(pair.into_inner(), warnings)
        }
        Rule::scripted_atom => {
            build_scripted(pair.into_inner(), warnings)
        }
        Rule::atom => {
            build_atom(pair.into_inner(), warnings)
        }
        Rule::special_symbol => {
            parse_special_symbol(pair)
        }
        Rule::superscript => {
            // Standalone superscript - will be handled as exponent
            let content = build_script_content(pair.into_inner(), warnings)?;
            Ok(MathNode::Superscript {
                base: Box::new(MathNode::Empty),
                superscript: Box::new(content),
            })
        }
        Rule::subscript => {
            // Standalone subscript
            let content = build_script_content(pair.into_inner(), warnings)?;
            Ok(MathNode::Subscript {
                base: Box::new(MathNode::Empty),
                subscript: Box::new(content),
            })
        }
        Rule::term => {
            build_term(pair.into_inner(), warnings)
        }
        Rule::expression => {
            build_expression(pair.into_inner(), warnings)
        }
        _ => {
            warnings.push(BackTranslationWarning::UnexpectedIndicator {
                indicator: format!("Unknown rule: {:?}", pair.as_rule()),
                position: pair.as_span().start(),
            });
            Ok(MathNode::Text(pair.as_str().to_string()))
        }
    }
}

/// Build atom from inner pairs
fn build_atom(
    pairs: pest::iterators::Pairs<Rule>,
    warnings: &mut Vec<BackTranslationWarning>,
) -> Result<MathNode, BackTranslationError> {
    for pair in pairs {
        return build_term_from_pair(pair, warnings);
    }
    Err(BackTranslationError::ParseError {
        message: "Empty atom".to_string(),
        position: None,
    })
}

/// Parse CMU number
fn parse_cmu_number(pairs: pest::iterators::Pairs<Rule>) -> Result<String, BackTranslationError> {
    let mut num = String::new();
    let mut has_decimal = false;

    for pair in pairs {
        match pair.as_rule() {
            Rule::numeric_indicator => {
                // Skip numeric indicator
            }
            Rule::cmu_digit => {
                let digit = cmu_braille_to_digit(pair.as_str())?;
                num.push(digit);
            }
            Rule::decimal_separator => {
                if !has_decimal {
                    num.push('.');
                    has_decimal = true;
                }
            }
            _ => {}
        }
    }

    if num.is_empty() {
        return Err(BackTranslationError::ParseError {
            message: "Empty number".to_string(),
            position: None,
        });
    }

    Ok(num)
}

/// Convert CMU braille digit to character
/// CMU uses same digit patterns as UEB (letters a-j)
fn cmu_braille_to_digit(braille: &str) -> Result<char, BackTranslationError> {
    match braille {
        "\u{2801}" => Ok('1'), // dots 1 (a)
        "\u{2803}" => Ok('2'), // dots 12 (b)
        "\u{2809}" => Ok('3'), // dots 14 (c)
        "\u{2819}" => Ok('4'), // dots 145 (d)
        "\u{2811}" => Ok('5'), // dots 15 (e)
        "\u{280B}" => Ok('6'), // dots 124 (f)
        "\u{281B}" => Ok('7'), // dots 1245 (g)
        "\u{2813}" => Ok('8'), // dots 125 (h)
        "\u{280A}" => Ok('9'), // dots 24 (i)
        "\u{281A}" => Ok('0'), // dots 245 (j)
        _ => Err(BackTranslationError::ParseError {
            message: format!("Invalid CMU digit: {}", braille),
            position: None,
        }),
    }
}

/// Parse CMU letter
fn parse_cmu_letter(pairs: pest::iterators::Pairs<Rule>) -> Result<(String, bool), BackTranslationError> {
    let mut is_capital = false;
    let mut letter = String::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::capital_indicator => {
                is_capital = true;
            }
            Rule::letter_char => {
                letter = cmu_braille_to_letter(pair.as_str())?;
            }
            _ => {}
        }
    }

    if letter.is_empty() {
        return Err(BackTranslationError::ParseError {
            message: "Empty letter".to_string(),
            position: None,
        });
    }

    Ok((letter, is_capital))
}

/// Convert CMU braille letter to character
fn cmu_braille_to_letter(braille: &str) -> Result<String, BackTranslationError> {
    let letter = match braille {
        "\u{2801}" => "a",
        "\u{2803}" => "b",
        "\u{2809}" => "c",
        "\u{2819}" => "d",
        "\u{2811}" => "e",
        "\u{280B}" => "f",
        "\u{281B}" => "g",
        "\u{2813}" => "h",
        "\u{280A}" => "i",
        "\u{281A}" => "j",
        "\u{2805}" => "k",
        "\u{2807}" => "l",
        "\u{280D}" => "m",
        "\u{281D}" => "n",
        "\u{2815}" => "o",
        "\u{280F}" => "p",
        "\u{281F}" => "q",
        "\u{2817}" => "r",
        "\u{280E}" => "s",
        "\u{281E}" => "t",
        "\u{2825}" => "u",
        "\u{2827}" => "v",
        "\u{283A}" => "w",
        "\u{282D}" => "x",
        "\u{283D}" => "y",
        "\u{2835}" => "z",
        _ => return Err(BackTranslationError::ParseError {
            message: format!("Invalid CMU letter: {}", braille),
            position: None,
        }),
    };
    Ok(letter.to_string())
}

/// Parse Greek letter
fn parse_greek_letter(pairs: pest::iterators::Pairs<Rule>) -> Result<GreekLetter, BackTranslationError> {
    let mut is_capital = false;
    let mut letter_char: Option<char> = None;

    for pair in pairs {
        match pair.as_rule() {
            Rule::greek_indicator => {
                // Greek prefix indicator
            }
            Rule::capital_indicator => {
                is_capital = true;
            }
            Rule::greek_char => {
                letter_char = Some(cmu_braille_to_greek_char(pair.as_str())?);
            }
            _ => {}
        }
    }

    if let Some(ch) = letter_char {
        if is_capital {
            Ok(GreekLetter::uppercase(uppercase_greek_char(ch)))
        } else {
            Ok(GreekLetter::lowercase(ch))
        }
    } else {
        Err(BackTranslationError::ParseError {
            message: "Empty Greek letter".to_string(),
            position: None,
        })
    }
}

/// Convert CMU braille Greek character to Unicode char
fn cmu_braille_to_greek_char(braille: &str) -> Result<char, BackTranslationError> {
    let greek = match braille {
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
        _ => return Err(BackTranslationError::ParseError {
            message: format!("Invalid CMU Greek letter: {}", braille),
            position: None,
        }),
    };
    Ok(greek)
}

/// Convert lowercase Greek to uppercase
fn uppercase_greek_char(ch: char) -> char {
    match ch {
        '\u{03B1}' => '\u{0391}', // Alpha
        '\u{03B2}' => '\u{0392}', // Beta
        '\u{03B3}' => '\u{0393}', // Gamma
        '\u{03B4}' => '\u{0394}', // Delta
        '\u{03B5}' => '\u{0395}', // Epsilon
        '\u{03B6}' => '\u{0396}', // Zeta
        '\u{03B7}' => '\u{0397}', // Eta
        '\u{03B8}' => '\u{0398}', // Theta
        '\u{03B9}' => '\u{0399}', // Iota
        '\u{03BA}' => '\u{039A}', // Kappa
        '\u{03BB}' => '\u{039B}', // Lambda
        '\u{03BC}' => '\u{039C}', // Mu
        '\u{03BD}' => '\u{039D}', // Nu
        '\u{03BE}' => '\u{039E}', // Xi
        '\u{03BF}' => '\u{039F}', // Omicron
        '\u{03C0}' => '\u{03A0}', // Pi
        '\u{03C1}' => '\u{03A1}', // Rho
        '\u{03C3}' => '\u{03A3}', // Sigma
        '\u{03C4}' => '\u{03A4}', // Tau
        '\u{03C5}' => '\u{03A5}', // Upsilon
        '\u{03C6}' => '\u{03A6}', // Phi
        '\u{03C7}' => '\u{03A7}', // Chi
        '\u{03C8}' => '\u{03A8}', // Psi
        '\u{03C9}' => '\u{03A9}', // Omega
        _ => ch,
    }
}

/// Parse special symbol
fn parse_special_symbol(pair: pest::iterators::Pair<Rule>) -> Result<MathNode, BackTranslationError> {
    let inner = pair.into_inner().next();
    if let Some(symbol_pair) = inner {
        let symbol = match symbol_pair.as_rule() {
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
            Rule::ellipsis => "\u{2026}",
            Rule::prime => "\u{2032}",
            Rule::double_prime => "\u{2033}",
            _ => return Ok(MathNode::Text(symbol_pair.as_str().to_string())),
        };
        let info = IdentifierInfo::new(symbol);
        Ok(MathNode::Identifier(info))
    } else {
        Ok(MathNode::Empty)
    }
}

/// Parse operator and return operator string
fn parse_operator(pair: pest::iterators::Pair<Rule>) -> Result<String, BackTranslationError> {
    // Get the innermost operator rule
    let inner = pair.into_inner().next();
    if let Some(op_pair) = inner {
        return parse_operator_inner(op_pair);
    }
    Err(BackTranslationError::ParseError {
        message: "Empty operator".to_string(),
        position: None,
    })
}

/// Parse inner operator
fn parse_operator_inner(pair: pest::iterators::Pair<Rule>) -> Result<String, BackTranslationError> {
    let op = match pair.as_rule() {
        // Arithmetic
        Rule::plus => "+",
        Rule::minus => "\u{2212}",
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
        Rule::much_less => "\u{226A}",
        Rule::much_greater => "\u{226B}",
        Rule::approximately_equal => "\u{2248}",
        Rule::congruent => "\u{2245}",
        Rule::equivalent => "\u{2261}",

        // Set operators
        Rule::subset => "\u{2282}",
        Rule::superset => "\u{2283}",
        Rule::subset_equal => "\u{2286}",
        Rule::superset_equal => "\u{2287}",
        Rule::union => "\u{222A}",
        Rule::intersection => "\u{2229}",
        Rule::set_minus => "\u{2216}",
        Rule::element_of => "\u{2208}",
        Rule::not_element_of => "\u{2209}",

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
        Rule::up_arrow => "\u{2191}",
        Rule::down_arrow => "\u{2193}",

        // If it's a compound rule, recurse into it
        Rule::arithmetic_operator | Rule::comparison_operator | Rule::set_operator
        | Rule::logical_operator | Rule::arrow_operator | Rule::operator => {
            if let Some(inner) = pair.into_inner().next() {
                return parse_operator_inner(inner);
            }
            return Err(BackTranslationError::ParseError {
                message: "Empty compound operator".to_string(),
                position: None,
            });
        }

        _ => {
            return Err(BackTranslationError::ParseError {
                message: format!("Unknown operator: {:?}", pair.as_rule()),
                position: None,
            });
        }
    };
    Ok(op.to_string())
}

/// Build grouped expression
fn build_grouped(
    pairs: pest::iterators::Pairs<Rule>,
    warnings: &mut Vec<BackTranslationWarning>,
) -> Result<MathNode, BackTranslationError> {
    let mut open_delim = "(".to_string();
    let mut close_delim = ")".to_string();
    let mut content = MathNode::Empty;

    for pair in pairs {
        match pair.as_rule() {
            Rule::open_paren => open_delim = "(".to_string(),
            Rule::close_paren => close_delim = ")".to_string(),
            Rule::open_bracket => open_delim = "[".to_string(),
            Rule::close_bracket => close_delim = "]".to_string(),
            Rule::open_brace => open_delim = "{".to_string(),
            Rule::close_brace => close_delim = "}".to_string(),
            Rule::open_angle => open_delim = "\u{27E8}".to_string(),
            Rule::close_angle => close_delim = "\u{27E9}".to_string(),
            Rule::expression => {
                content = build_expression(pair.into_inner(), warnings)?;
            }
            _ => {}
        }
    }

    Ok(MathNode::Grouped {
        open: open_delim,
        close: close_delim,
        content: Box::new(content),
    })
}

/// Build fraction
fn build_fraction(
    pairs: pest::iterators::Pairs<Rule>,
    warnings: &mut Vec<BackTranslationWarning>,
) -> Result<MathNode, BackTranslationError> {
    let mut expressions: Vec<MathNode> = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::expression => {
                let expr = build_expression(pair.into_inner(), warnings)?;
                expressions.push(expr);
            }
            Rule::fraction_open | Rule::fraction_bar | Rule::fraction_close => {
                // Skip delimiters
            }
            _ => {}
        }
    }

    if expressions.len() >= 2 {
        Ok(MathNode::Fraction {
            numerator: Box::new(expressions.remove(0)),
            denominator: Box::new(expressions.remove(0)),
        })
    } else {
        Err(BackTranslationError::ParseError {
            message: "Fraction requires numerator and denominator".to_string(),
            position: None,
        })
    }
}

/// Build radical
fn build_radical(
    pairs: pest::iterators::Pairs<Rule>,
    warnings: &mut Vec<BackTranslationWarning>,
) -> Result<MathNode, BackTranslationError> {
    let mut index: Option<MathNode> = None;
    let mut radicand = MathNode::Empty;

    for pair in pairs {
        match pair.as_rule() {
            Rule::radical_index => {
                // The index comes before the radical symbol
                for inner in pair.into_inner() {
                    if inner.as_rule() == Rule::atom {
                        index = Some(build_atom(inner.into_inner(), warnings)?);
                    }
                }
            }
            Rule::expression => {
                radicand = build_expression(pair.into_inner(), warnings)?;
            }
            Rule::radical_symbol | Rule::radical_end => {
                // Skip
            }
            _ => {}
        }
    }

    Ok(MathNode::Radical {
        index: index.map(Box::new),
        radicand: Box::new(radicand),
    })
}

/// Build scripted atom (base with superscript/subscript)
fn build_scripted(
    pairs: pest::iterators::Pairs<Rule>,
    warnings: &mut Vec<BackTranslationWarning>,
) -> Result<MathNode, BackTranslationError> {
    let mut base = MathNode::Empty;
    let mut superscript: Option<MathNode> = None;
    let mut subscript: Option<MathNode> = None;

    for pair in pairs {
        match pair.as_rule() {
            Rule::atom => {
                base = build_atom(pair.into_inner(), warnings)?;
            }
            Rule::script => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::superscript => {
                            superscript = Some(build_script_content(inner.into_inner(), warnings)?);
                        }
                        Rule::subscript => {
                            subscript = Some(build_script_content(inner.into_inner(), warnings)?);
                        }
                        _ => {}
                    }
                }
            }
            Rule::superscript => {
                superscript = Some(build_script_content(pair.into_inner(), warnings)?);
            }
            Rule::subscript => {
                subscript = Some(build_script_content(pair.into_inner(), warnings)?);
            }
            _ => {}
        }
    }

    // Build the appropriate structure
    match (superscript, subscript) {
        (Some(sup), Some(sub)) => Ok(MathNode::SubSuperscript {
            base: Box::new(base),
            subscript: Box::new(sub),
            superscript: Box::new(sup),
        }),
        (Some(sup), None) => Ok(MathNode::Superscript {
            base: Box::new(base),
            superscript: Box::new(sup),
        }),
        (None, Some(sub)) => Ok(MathNode::Subscript {
            base: Box::new(base),
            subscript: Box::new(sub),
        }),
        (None, None) => Ok(base),
    }
}

/// Build script content
fn build_script_content(
    pairs: pest::iterators::Pairs<Rule>,
    warnings: &mut Vec<BackTranslationWarning>,
) -> Result<MathNode, BackTranslationError> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::script_content => {
                return build_script_content(pair.into_inner(), warnings);
            }
            Rule::grouped | Rule::fraction | Rule::number | Rule::greek_letter | Rule::letter => {
                return build_term_from_pair(pair, warnings);
            }
            Rule::superscript_indicator | Rule::subscript_indicator | Rule::baseline_indicator => {
                // Skip indicators
            }
            _ => {}
        }
    }

    Err(BackTranslationError::ParseError {
        message: "Empty script content".to_string(),
        position: None,
    })
}

/// Direct interpretation fallback for CMU braille
fn interpret_cmu_direct(
    braille: &str,
    warnings: &mut Vec<BackTranslationWarning>,
) -> Result<MathNode, BackTranslationError> {
    let chars: Vec<char> = braille.chars().collect();
    let mut nodes: Vec<MathNode> = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        // Skip whitespace/empty braille
        if ch == '\u{2800}' || ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            i += 1;
            continue;
        }

        // Check for numeric indicator
        if ch == '\u{283C}' {
            // Numeric indicator - collect digits
            i += 1;
            let mut num = String::new();
            while i < chars.len() {
                let digit_char = chars[i];
                if let Ok(d) = cmu_braille_to_digit(&digit_char.to_string()) {
                    num.push(d);
                    i += 1;
                } else if digit_char == '\u{2802}' {
                    // Decimal separator
                    num.push('.');
                    i += 1;
                } else {
                    break;
                }
            }
            if !num.is_empty() {
                nodes.push(MathNode::Number(num));
            }
            continue;
        }

        // Check for Greek indicator
        if ch == '\u{2828}' {
            i += 1;
            let mut is_capital = false;
            if i < chars.len() && chars[i] == '\u{2820}' {
                is_capital = true;
                i += 1;
            }
            if i < chars.len() {
                if let Ok(greek_ch) = cmu_braille_to_greek_char(&chars[i].to_string()) {
                    let letter = if is_capital {
                        GreekLetter::uppercase(uppercase_greek_char(greek_ch))
                    } else {
                        GreekLetter::lowercase(greek_ch)
                    };
                    nodes.push(MathNode::Greek(letter));
                    i += 1;
                    continue;
                }
            }
        }

        // Check for capital indicator
        if ch == '\u{2820}' {
            i += 1;
            if i < chars.len() {
                if let Ok(letter) = cmu_braille_to_letter(&chars[i].to_string()) {
                    let info = IdentifierInfo::new(&letter).with_capital();
                    nodes.push(MathNode::Identifier(info));
                    i += 1;
                    continue;
                }
            }
        }

        // Check for operators
        if let Some(op) = try_parse_cmu_operator(&chars, i) {
            nodes.push(MathNode::Operator(op.0));
            i += op.1;
            continue;
        }

        // Try as letter
        if let Ok(letter) = cmu_braille_to_letter(&ch.to_string()) {
            let info = IdentifierInfo::new(&letter);
            nodes.push(MathNode::Identifier(info));
            i += 1;
            continue;
        }

        // Unknown character
        warnings.push(BackTranslationWarning::UnexpectedIndicator {
            indicator: format!("Unknown character: {}", ch),
            position: i,
        });
        i += 1;
    }

    if nodes.is_empty() {
        return Err(BackTranslationError::ParseError {
            message: "No valid content found".to_string(),
            position: None,
        });
    }

    if nodes.len() == 1 {
        Ok(nodes.remove(0))
    } else {
        Ok(MathNode::Row(nodes))
    }
}

/// Try to parse CMU operator at position
fn try_parse_cmu_operator(chars: &[char], pos: usize) -> Option<(String, usize)> {
    let ch = chars[pos];

    // Single-character operators
    let op = match ch {
        '\u{282E}' => Some(("+".to_string(), 1)),      // dots 2346
        '\u{2824}' => Some(("\u{2212}".to_string(), 1)),     // dots 36 (minus)
        '\u{282C}' => Some(("\u{00D7}".to_string(), 1)),     // dots 346 (times)
        '\u{2832}' => Some(("\u{00F7}".to_string(), 1)),     // dots 256 (divide)
        '\u{2836}' => Some(("=".to_string(), 1)),      // dots 2356
        '\u{282A}' => Some(("<".to_string(), 1)),      // dots 246
        '\u{2815}' => Some((">".to_string(), 1)),      // dots 135
        _ => None,
    };

    if op.is_some() {
        return op;
    }

    // Two-character operators
    if pos + 1 < chars.len() {
        let next = chars[pos + 1];
        match (ch, next) {
            ('\u{2818}', '\u{2836}') => return Some(("\u{2260}".to_string(), 2)), // not equal
            ('\u{282A}', '\u{2836}') => return Some(("\u{2264}".to_string(), 2)), // less equal
            ('\u{2815}', '\u{2836}') => return Some(("\u{2265}".to_string(), 2)), // greater equal
            ('\u{282A}', '\u{282A}') => return Some(("\u{226A}".to_string(), 2)), // much less
            ('\u{2815}', '\u{2815}') => return Some(("\u{226B}".to_string(), 2)), // much greater
            ('\u{2836}', '\u{2836}') => return Some(("\u{2261}".to_string(), 2)), // equivalent
            ('\u{2823}', '\u{2804}') => return Some(("\u{2282}".to_string(), 2)), // subset
            ('\u{2823}', '\u{2806}') => return Some(("\u{2286}".to_string(), 2)), // subset equal
            ('\u{2838}', '\u{281C}') => return Some(("\u{222A}".to_string(), 2)), // union
            ('\u{2838}', '\u{2831}') => return Some(("\u{2229}".to_string(), 2)), // intersection
            ('\u{2838}', '\u{2822}') => return Some(("\u{2227}".to_string(), 2)), // and
            ('\u{2838}', '\u{280A}') => return Some(("\u{2228}".to_string(), 2)), // or
            ('\u{2812}', '\u{2815}') => return Some(("\u{21D2}".to_string(), 2)), // implies
            ('\u{2812}', '\u{2802}') => return Some(("\u{2192}".to_string(), 2)), // right arrow
            ('\u{2810}', '\u{2812}') => return Some(("\u{2190}".to_string(), 2)), // left arrow
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmu_single_digit() {
        // CMU uses numeric indicator + digit pattern
        let braille = "\u{283C}\u{2801}"; // numeric indicator + 1
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mn>1</mn>"));
    }

    #[test]
    fn test_cmu_multi_digit() {
        let braille = "\u{283C}\u{2801}\u{2803}\u{2809}"; // 123
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mn>123</mn>"));
    }

    #[test]
    fn test_cmu_decimal() {
        let braille = "\u{283C}\u{2809}\u{2802}\u{2801}\u{2819}"; // 3.14
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mn>3.14</mn>"));
    }

    #[test]
    fn test_cmu_letter() {
        let braille = "\u{282D}"; // x
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
    }

    #[test]
    fn test_cmu_capital_letter() {
        let braille = "\u{2820}\u{282D}"; // X (capital indicator + x)
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>X</mi>"));
    }

    #[test]
    fn test_cmu_addition() {
        // x + y in CMU
        let braille = "\u{282D}\u{282E}\u{283D}"; // x + y
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>+</mo>"));
        assert!(mathml.contains("<mi>y</mi>"));
    }

    #[test]
    fn test_cmu_subtraction() {
        // x - y in CMU
        let braille = "\u{282D}\u{2824}\u{283D}"; // x - y
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>") && (mathml.contains("-") || mathml.contains("\u{2212}")));
        assert!(mathml.contains("<mi>y</mi>"));
    }

    #[test]
    fn test_cmu_multiplication() {
        // x * y in CMU
        let braille = "\u{282D}\u{282C}\u{283D}"; // x times y
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>"));
        assert!(mathml.contains("<mi>y</mi>"));
    }

    #[test]
    fn test_cmu_equals() {
        // x = y in CMU
        let braille = "\u{282D}\u{2836}\u{283D}"; // x = y
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>=</mo>"));
        assert!(mathml.contains("<mi>y</mi>"));
    }

    #[test]
    fn test_cmu_less_than() {
        // x < y in CMU
        let braille = "\u{282D}\u{282A}\u{283D}"; // x < y
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>&lt;</mo>") || mathml.contains("<mo><</mo>"));
        assert!(mathml.contains("<mi>y</mi>"));
    }

    #[test]
    fn test_cmu_greater_than() {
        // x > y in CMU
        let braille = "\u{282D}\u{2815}\u{283D}"; // x > y
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
    }

    #[test]
    fn test_cmu_greek_alpha() {
        // Greek alpha in CMU
        let braille = "\u{2828}\u{2801}"; // Greek indicator + alpha
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("\u{03B1}") || mathml.contains("alpha"));
    }

    #[test]
    fn test_cmu_greek_capital() {
        // Capital Greek Delta in CMU
        let braille = "\u{2828}\u{2820}\u{2819}"; // Greek indicator + capital + delta
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("\u{0394}") || mathml.contains("Delta"));
    }

    #[test]
    fn test_cmu_expression_with_number() {
        // x + 1 in CMU
        let braille = "\u{282D}\u{282E}\u{283C}\u{2801}"; // x + 1
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>+</mo>"));
        assert!(mathml.contains("<mn>1</mn>"));
    }

    #[test]
    fn test_cmu_empty_input() {
        let result = parse_cmu("");
        assert!(!result.is_success());
        assert!(matches!(
            result.errors.first(),
            Some(BackTranslationError::EmptyInput)
        ));
    }

    #[test]
    fn test_cmu_whitespace_only() {
        let result = parse_cmu("   ");
        assert!(!result.is_success());
    }

    #[test]
    fn test_cmu_digit_to_char() {
        assert_eq!(cmu_braille_to_digit("\u{2801}").unwrap(), '1');
        assert_eq!(cmu_braille_to_digit("\u{2803}").unwrap(), '2');
        assert_eq!(cmu_braille_to_digit("\u{2809}").unwrap(), '3');
        assert_eq!(cmu_braille_to_digit("\u{2819}").unwrap(), '4');
        assert_eq!(cmu_braille_to_digit("\u{2811}").unwrap(), '5');
        assert_eq!(cmu_braille_to_digit("\u{280B}").unwrap(), '6');
        assert_eq!(cmu_braille_to_digit("\u{281B}").unwrap(), '7');
        assert_eq!(cmu_braille_to_digit("\u{2813}").unwrap(), '8');
        assert_eq!(cmu_braille_to_digit("\u{280A}").unwrap(), '9');
        assert_eq!(cmu_braille_to_digit("\u{281A}").unwrap(), '0');
    }

    #[test]
    fn test_cmu_letter_to_char() {
        assert_eq!(cmu_braille_to_letter("\u{2801}").unwrap(), "a");
        assert_eq!(cmu_braille_to_letter("\u{282D}").unwrap(), "x");
        assert_eq!(cmu_braille_to_letter("\u{283D}").unwrap(), "y");
        assert_eq!(cmu_braille_to_letter("\u{2835}").unwrap(), "z");
    }

    #[test]
    fn test_cmu_not_equal() {
        // x != y in CMU (dots 45 + dots 2356)
        let braille = "\u{282D}\u{2818}\u{2836}\u{283D}";
        let result = parse_cmu(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("\u{2260}") || mathml.contains("neq"));
        assert!(mathml.contains("<mi>y</mi>"));
    }
}
