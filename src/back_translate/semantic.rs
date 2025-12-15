//! Semantic tree representation for mathematical expressions
//!
//! This module defines a code-independent intermediate representation
//! for mathematical expressions parsed from braille.

use std::fmt;

/// A node in the semantic math tree
#[derive(Debug, Clone, PartialEq)]
pub enum MathNode {
    /// A number (integer or decimal)
    Number(String),

    /// A variable/identifier (single letter or multi-letter)
    Identifier(IdentifierInfo),

    /// A mathematical operator (+, -, *, /, =, etc.)
    Operator(String),

    /// A fraction with numerator and denominator
    Fraction {
        numerator: Box<MathNode>,
        denominator: Box<MathNode>,
    },

    /// A radical (square root or nth root)
    Radical {
        index: Option<Box<MathNode>>,
        radicand: Box<MathNode>,
    },

    /// Superscript (exponent)
    Superscript {
        base: Box<MathNode>,
        superscript: Box<MathNode>,
    },

    /// Subscript
    Subscript {
        base: Box<MathNode>,
        subscript: Box<MathNode>,
    },

    /// Both superscript and subscript
    SubSuperscript {
        base: Box<MathNode>,
        subscript: Box<MathNode>,
        superscript: Box<MathNode>,
    },

    /// Grouped expression (parentheses, brackets, braces)
    Grouped {
        open: String,
        close: String,
        content: Box<MathNode>,
    },

    /// A row of expressions (horizontal sequence)
    Row(Vec<MathNode>),

    /// Text content (for mtext)
    Text(String),

    /// Greek letter
    Greek(GreekLetter),

    /// Empty node (used for error recovery)
    Empty,
}

/// Information about an identifier (variable)
#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierInfo {
    /// The identifier name
    pub name: String,
    /// Whether it's capitalized
    pub is_capital: bool,
    /// Font style
    pub font_style: FontStyle,
    /// Language/alphabet
    pub language: IdentifierLanguage,
}

impl IdentifierInfo {
    pub fn new(name: &str) -> Self {
        IdentifierInfo {
            name: name.to_string(),
            is_capital: false,
            font_style: FontStyle::Normal,
            language: IdentifierLanguage::English,
        }
    }

    pub fn with_capital(mut self) -> Self {
        self.is_capital = true;
        self
    }

    pub fn with_style(mut self, style: FontStyle) -> Self {
        self.font_style = style;
        self
    }

    pub fn with_language(mut self, lang: IdentifierLanguage) -> Self {
        self.language = lang;
        self
    }
}

/// Font style for identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Bold,
    Italic,
    BoldItalic,
    Script,
    Fraktur,
    DoubleStruck,
    SansSerif,
    Monospace,
}

impl FontStyle {
    /// Get the MathML mathvariant attribute value
    pub fn mathvariant(&self) -> Option<&'static str> {
        match self {
            FontStyle::Normal => None,
            FontStyle::Bold => Some("bold"),
            FontStyle::Italic => Some("italic"),
            FontStyle::BoldItalic => Some("bold-italic"),
            FontStyle::Script => Some("script"),
            FontStyle::Fraktur => Some("fraktur"),
            FontStyle::DoubleStruck => Some("double-struck"),
            FontStyle::SansSerif => Some("sans-serif"),
            FontStyle::Monospace => Some("monospace"),
        }
    }
}

/// Language/alphabet for identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentifierLanguage {
    English,
    Greek,
    Hebrew,
    Russian,
    German,
}

/// Greek letters
#[derive(Debug, Clone, PartialEq)]
pub struct GreekLetter {
    /// The Unicode character for the Greek letter
    pub char: char,
    /// Whether it's uppercase
    pub is_uppercase: bool,
    /// Whether it's a variant form (e.g., phi vs varphi)
    pub is_variant: bool,
}

impl GreekLetter {
    pub fn lowercase(ch: char) -> Self {
        GreekLetter {
            char: ch,
            is_uppercase: false,
            is_variant: false,
        }
    }

    pub fn uppercase(ch: char) -> Self {
        GreekLetter {
            char: ch,
            is_uppercase: true,
            is_variant: false,
        }
    }

    pub fn variant(ch: char) -> Self {
        GreekLetter {
            char: ch,
            is_uppercase: false,
            is_variant: true,
        }
    }
}

impl MathNode {
    /// Check if this node is an operator
    pub fn is_operator(&self) -> bool {
        matches!(self, MathNode::Operator(_))
    }

    /// Check if this node is a number
    pub fn is_number(&self) -> bool {
        matches!(self, MathNode::Number(_))
    }

    /// Check if this node is an identifier
    pub fn is_identifier(&self) -> bool {
        matches!(self, MathNode::Identifier(_) | MathNode::Greek(_))
    }

    /// Check if this is an empty node
    pub fn is_empty(&self) -> bool {
        matches!(self, MathNode::Empty)
    }

    /// Create a simple identifier from a string
    pub fn ident(s: &str) -> Self {
        MathNode::Identifier(IdentifierInfo::new(s))
    }

    /// Create a capital identifier
    pub fn capital_ident(s: &str) -> Self {
        MathNode::Identifier(IdentifierInfo::new(s).with_capital())
    }

    /// Create a number node
    pub fn num(s: &str) -> Self {
        MathNode::Number(s.to_string())
    }

    /// Create an operator node
    pub fn op(s: &str) -> Self {
        MathNode::Operator(s.to_string())
    }

    /// Create a row from a vector of nodes, flattening if single element
    pub fn row_or_single(nodes: Vec<MathNode>) -> Self {
        match nodes.len() {
            0 => MathNode::Empty,
            1 => nodes.into_iter().next().unwrap(),
            _ => MathNode::Row(nodes),
        }
    }

    /// Create a fraction
    pub fn frac(num: MathNode, den: MathNode) -> Self {
        MathNode::Fraction {
            numerator: Box::new(num),
            denominator: Box::new(den),
        }
    }

    /// Create a square root
    pub fn sqrt(radicand: MathNode) -> Self {
        MathNode::Radical {
            index: None,
            radicand: Box::new(radicand),
        }
    }

    /// Create an nth root
    pub fn nroot(index: MathNode, radicand: MathNode) -> Self {
        MathNode::Radical {
            index: Some(Box::new(index)),
            radicand: Box::new(radicand),
        }
    }

    /// Create a superscript
    pub fn sup(base: MathNode, superscript: MathNode) -> Self {
        MathNode::Superscript {
            base: Box::new(base),
            superscript: Box::new(superscript),
        }
    }

    /// Create a subscript
    pub fn sub(base: MathNode, subscript: MathNode) -> Self {
        MathNode::Subscript {
            base: Box::new(base),
            subscript: Box::new(subscript),
        }
    }

    /// Create a grouped expression with parentheses
    pub fn parens(content: MathNode) -> Self {
        MathNode::Grouped {
            open: "(".to_string(),
            close: ")".to_string(),
            content: Box::new(content),
        }
    }

    /// Create a grouped expression with brackets
    pub fn brackets(content: MathNode) -> Self {
        MathNode::Grouped {
            open: "[".to_string(),
            close: "]".to_string(),
            content: Box::new(content),
        }
    }

    /// Create a grouped expression with braces
    pub fn braces(content: MathNode) -> Self {
        MathNode::Grouped {
            open: "{".to_string(),
            close: "}".to_string(),
            content: Box::new(content),
        }
    }
}

impl fmt::Display for MathNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathNode::Number(n) => write!(f, "{}", n),
            MathNode::Identifier(info) => {
                if info.is_capital {
                    write!(f, "{}", info.name.to_uppercase())
                } else {
                    write!(f, "{}", info.name)
                }
            }
            MathNode::Operator(op) => write!(f, "{}", op),
            MathNode::Fraction { numerator, denominator } => {
                write!(f, "({}/{})", numerator, denominator)
            }
            MathNode::Radical { index, radicand } => {
                if let Some(idx) = index {
                    write!(f, "root[{}]({})", idx, radicand)
                } else {
                    write!(f, "sqrt({})", radicand)
                }
            }
            MathNode::Superscript { base, superscript } => {
                write!(f, "{}^{}", base, superscript)
            }
            MathNode::Subscript { base, subscript } => {
                write!(f, "{}_{}", base, subscript)
            }
            MathNode::SubSuperscript { base, subscript, superscript } => {
                write!(f, "{}_{}^{}", base, subscript, superscript)
            }
            MathNode::Grouped { open, close, content } => {
                write!(f, "{}{}{}", open, content, close)
            }
            MathNode::Row(nodes) => {
                for node in nodes {
                    write!(f, "{}", node)?;
                }
                Ok(())
            }
            MathNode::Text(t) => write!(f, "\"{}\"", t),
            MathNode::Greek(g) => write!(f, "{}", g.char),
            MathNode::Empty => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_node_helpers() {
        let num = MathNode::num("123");
        assert!(num.is_number());

        let ident = MathNode::ident("x");
        assert!(ident.is_identifier());

        let op = MathNode::op("+");
        assert!(op.is_operator());

        let empty = MathNode::Empty;
        assert!(empty.is_empty());
    }

    #[test]
    fn test_row_or_single() {
        let empty = MathNode::row_or_single(vec![]);
        assert!(empty.is_empty());

        let single = MathNode::row_or_single(vec![MathNode::num("1")]);
        assert!(single.is_number());

        let row = MathNode::row_or_single(vec![MathNode::num("1"), MathNode::op("+")]);
        assert!(matches!(row, MathNode::Row(_)));
    }

    #[test]
    fn test_display() {
        let frac = MathNode::frac(MathNode::num("1"), MathNode::num("2"));
        assert_eq!(format!("{}", frac), "(1/2)");

        let sqrt = MathNode::sqrt(MathNode::num("2"));
        assert_eq!(format!("{}", sqrt), "sqrt(2)");
    }
}
