//! MathML generation from semantic tree
//!
//! This module converts the code-independent semantic tree representation
//! into MathML (Presentation MathML by default).

use crate::back_translate::semantic::MathNode;

#[cfg(test)]
use crate::back_translate::semantic::GreekLetter;

/// Options for MathML generation
#[derive(Debug, Clone)]
pub struct MathMLOptions {
    /// Whether to include XML declaration
    pub include_declaration: bool,
    /// Whether to add display="block" attribute
    pub display_block: bool,
    /// Indentation string (empty for no indentation)
    pub indent: String,
}

impl Default for MathMLOptions {
    fn default() -> Self {
        MathMLOptions {
            include_declaration: false,
            display_block: false,
            indent: String::new(),
        }
    }
}

/// Generate MathML from a semantic tree
pub fn generate_mathml(node: &MathNode, options: &MathMLOptions) -> String {
    let mut output = String::new();

    if options.include_declaration {
        output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    }

    output.push_str("<math xmlns=\"http://www.w3.org/1998/Math/MathML\"");
    if options.display_block {
        output.push_str(" display=\"block\"");
    }
    output.push('>');

    generate_node(&mut output, node, &options.indent, 1);

    output.push_str("</math>");

    output
}

/// Generate MathML for a single node
fn generate_node(output: &mut String, node: &MathNode, indent: &str, depth: usize) {
    match node {
        MathNode::Number(n) => {
            output.push_str("<mn>");
            output.push_str(&escape_xml(n));
            output.push_str("</mn>");
        }

        MathNode::Identifier(info) => {
            output.push_str("<mi");

            // Add mathvariant if not normal
            if let Some(variant) = info.font_style.mathvariant() {
                output.push_str(" mathvariant=\"");
                output.push_str(variant);
                output.push('"');
            }

            output.push('>');

            // Get the actual character
            let ch = if info.is_capital {
                info.name.chars().next().unwrap_or('x').to_uppercase().next().unwrap_or('X')
            } else {
                info.name.chars().next().unwrap_or('x')
            };

            output.push(ch);
            output.push_str("</mi>");
        }

        MathNode::Operator(op) => {
            output.push_str("<mo>");
            output.push_str(&escape_xml(op));
            output.push_str("</mo>");
        }

        MathNode::Fraction { numerator, denominator } => {
            output.push_str("<mfrac>");
            generate_row_wrapper(output, numerator, indent, depth + 1);
            generate_row_wrapper(output, denominator, indent, depth + 1);
            output.push_str("</mfrac>");
        }

        MathNode::Radical { index, radicand } => {
            if let Some(idx) = index {
                output.push_str("<mroot>");
                generate_row_wrapper(output, radicand, indent, depth + 1);
                generate_row_wrapper(output, idx, indent, depth + 1);
                output.push_str("</mroot>");
            } else {
                output.push_str("<msqrt>");
                generate_node(output, radicand, indent, depth + 1);
                output.push_str("</msqrt>");
            }
        }

        MathNode::Superscript { base, superscript } => {
            output.push_str("<msup>");
            generate_row_wrapper(output, base, indent, depth + 1);
            generate_row_wrapper(output, superscript, indent, depth + 1);
            output.push_str("</msup>");
        }

        MathNode::Subscript { base, subscript } => {
            output.push_str("<msub>");
            generate_row_wrapper(output, base, indent, depth + 1);
            generate_row_wrapper(output, subscript, indent, depth + 1);
            output.push_str("</msub>");
        }

        MathNode::SubSuperscript { base, subscript, superscript } => {
            output.push_str("<msubsup>");
            generate_row_wrapper(output, base, indent, depth + 1);
            generate_row_wrapper(output, subscript, indent, depth + 1);
            generate_row_wrapper(output, superscript, indent, depth + 1);
            output.push_str("</msubsup>");
        }

        MathNode::Grouped { open, close, content } => {
            output.push_str("<mrow>");
            output.push_str("<mo>");
            output.push_str(&escape_xml(open));
            output.push_str("</mo>");
            generate_node(output, content, indent, depth + 1);
            output.push_str("<mo>");
            output.push_str(&escape_xml(close));
            output.push_str("</mo>");
            output.push_str("</mrow>");
        }

        MathNode::Row(nodes) => {
            if nodes.len() == 1 {
                generate_node(output, &nodes[0], indent, depth);
            } else {
                output.push_str("<mrow>");
                for child in nodes {
                    generate_node(output, child, indent, depth + 1);
                }
                output.push_str("</mrow>");
            }
        }

        MathNode::Text(t) => {
            output.push_str("<mtext>");
            output.push_str(&escape_xml(t));
            output.push_str("</mtext>");
        }

        MathNode::Greek(letter) => {
            output.push_str("<mi>");
            output.push(letter.char);
            output.push_str("</mi>");
        }

        MathNode::Empty => {
            // Empty nodes don't produce output
        }
    }
}

/// Wrap node in mrow if it's a Row, otherwise just generate it
fn generate_row_wrapper(output: &mut String, node: &MathNode, indent: &str, depth: usize) {
    match node {
        MathNode::Row(nodes) if nodes.len() > 1 => {
            output.push_str("<mrow>");
            for child in nodes {
                generate_node(output, child, indent, depth + 1);
            }
            output.push_str("</mrow>");
        }
        _ => generate_node(output, node, indent, depth),
    }
}

/// Escape special XML characters
fn escape_xml(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&apos;"),
            _ => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::back_translate::semantic::IdentifierInfo;

    #[test]
    fn test_simple_number() {
        let node = MathNode::Number("123".to_string());
        let mathml = generate_mathml(&node, &MathMLOptions::default());
        assert_eq!(mathml, "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mn>123</mn></math>");
    }

    #[test]
    fn test_simple_identifier() {
        let node = MathNode::Identifier(IdentifierInfo::new("x"));
        let mathml = generate_mathml(&node, &MathMLOptions::default());
        assert_eq!(mathml, "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>x</mi></math>");
    }

    #[test]
    fn test_capital_identifier() {
        let node = MathNode::Identifier(IdentifierInfo::new("a").with_capital());
        let mathml = generate_mathml(&node, &MathMLOptions::default());
        assert_eq!(mathml, "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>A</mi></math>");
    }

    #[test]
    fn test_operator() {
        let node = MathNode::Operator("+".to_string());
        let mathml = generate_mathml(&node, &MathMLOptions::default());
        assert_eq!(mathml, "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mo>+</mo></math>");
    }

    #[test]
    fn test_fraction() {
        let node = MathNode::Fraction {
            numerator: Box::new(MathNode::Number("1".to_string())),
            denominator: Box::new(MathNode::Number("2".to_string())),
        };
        let mathml = generate_mathml(&node, &MathMLOptions::default());
        assert_eq!(
            mathml,
            "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mfrac><mn>1</mn><mn>2</mn></mfrac></math>"
        );
    }

    #[test]
    fn test_sqrt() {
        let node = MathNode::Radical {
            index: None,
            radicand: Box::new(MathNode::Number("2".to_string())),
        };
        let mathml = generate_mathml(&node, &MathMLOptions::default());
        assert_eq!(
            mathml,
            "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msqrt><mn>2</mn></msqrt></math>"
        );
    }

    #[test]
    fn test_nroot() {
        let node = MathNode::Radical {
            index: Some(Box::new(MathNode::Number("3".to_string()))),
            radicand: Box::new(MathNode::Number("8".to_string())),
        };
        let mathml = generate_mathml(&node, &MathMLOptions::default());
        assert_eq!(
            mathml,
            "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mroot><mn>8</mn><mn>3</mn></mroot></math>"
        );
    }

    #[test]
    fn test_superscript() {
        let node = MathNode::Superscript {
            base: Box::new(MathNode::ident("x")),
            superscript: Box::new(MathNode::Number("2".to_string())),
        };
        let mathml = generate_mathml(&node, &MathMLOptions::default());
        assert_eq!(
            mathml,
            "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msup><mi>x</mi><mn>2</mn></msup></math>"
        );
    }

    #[test]
    fn test_subscript() {
        let node = MathNode::Subscript {
            base: Box::new(MathNode::ident("x")),
            subscript: Box::new(MathNode::Number("1".to_string())),
        };
        let mathml = generate_mathml(&node, &MathMLOptions::default());
        assert_eq!(
            mathml,
            "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><msub><mi>x</mi><mn>1</mn></msub></math>"
        );
    }

    #[test]
    fn test_grouped() {
        let node = MathNode::Grouped {
            open: "(".to_string(),
            close: ")".to_string(),
            content: Box::new(MathNode::ident("x")),
        };
        let mathml = generate_mathml(&node, &MathMLOptions::default());
        assert_eq!(
            mathml,
            "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></math>"
        );
    }

    #[test]
    fn test_row() {
        let node = MathNode::Row(vec![
            MathNode::ident("x"),
            MathNode::op("+"),
            MathNode::num("1"),
        ]);
        let mathml = generate_mathml(&node, &MathMLOptions::default());
        assert_eq!(
            mathml,
            "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow></math>"
        );
    }

    #[test]
    fn test_xml_escape() {
        let node = MathNode::Operator("<".to_string());
        let mathml = generate_mathml(&node, &MathMLOptions::default());
        assert!(mathml.contains("&lt;"));
    }

    #[test]
    fn test_greek() {
        let node = MathNode::Greek(GreekLetter::lowercase('\u{03B1}')); // alpha
        let mathml = generate_mathml(&node, &MathMLOptions::default());
        assert_eq!(
            mathml,
            "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>\u{03B1}</mi></math>"
        );
    }
}
