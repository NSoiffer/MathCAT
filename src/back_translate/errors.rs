//! Error types for braille back-translation

use std::fmt;

/// Errors that can occur during braille back-translation
#[derive(Debug, Clone, PartialEq)]
pub enum BackTranslationError {
    /// Unrecognized braille symbol at the given position
    UnrecognizedSymbol {
        position: usize,
        braille: String,
    },
    /// Unclosed fraction (missing close indicator)
    UnclosedFraction {
        open_position: usize,
    },
    /// Unclosed radical (missing close indicator)
    UnclosedRadical {
        open_position: usize,
    },
    /// Unbalanced grouping symbols
    UnbalancedGrouping {
        expected: Option<char>,
        found: Option<char>,
        position: usize,
    },
    /// Invalid script (superscript/subscript) structure
    InvalidScript {
        position: usize,
        message: String,
    },
    /// Ambiguous expression that could be parsed multiple ways
    AmbiguousExpression {
        possibilities: Vec<String>,
        position: usize,
    },
    /// Empty input
    EmptyInput,
    /// Braille code not yet supported for back-translation
    UnsupportedCode {
        code: String,
    },
    /// General parse error
    ParseError {
        message: String,
        position: Option<usize>,
    },
}

impl fmt::Display for BackTranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackTranslationError::UnrecognizedSymbol { position, braille } => {
                write!(f, "Unrecognized braille symbol '{}' at position {}", braille, position)
            }
            BackTranslationError::UnclosedFraction { open_position } => {
                write!(f, "Unclosed fraction starting at position {}", open_position)
            }
            BackTranslationError::UnclosedRadical { open_position } => {
                write!(f, "Unclosed radical starting at position {}", open_position)
            }
            BackTranslationError::UnbalancedGrouping { expected, found, position } => {
                match (expected, found) {
                    (Some(exp), Some(fnd)) => {
                        write!(f, "Expected '{}' but found '{}' at position {}", exp, fnd, position)
                    }
                    (Some(exp), None) => {
                        write!(f, "Expected '{}' but reached end of input at position {}", exp, position)
                    }
                    (None, Some(ch)) => {
                        write!(f, "Unexpected '{}' at position {}", ch, position)
                    }
                    (None, None) => {
                        write!(f, "Grouping error at position {}", position)
                    }
                }
            }
            BackTranslationError::InvalidScript { position, message } => {
                write!(f, "Invalid script at position {}: {}", position, message)
            }
            BackTranslationError::AmbiguousExpression { possibilities, position } => {
                write!(f, "Ambiguous expression at position {}: could be {}",
                    position, possibilities.join(" or "))
            }
            BackTranslationError::EmptyInput => {
                write!(f, "Empty braille input")
            }
            BackTranslationError::UnsupportedCode { code } => {
                write!(f, "Braille code '{}' is not yet supported for back-translation", code)
            }
            BackTranslationError::ParseError { message, position } => {
                if let Some(pos) = position {
                    write!(f, "Parse error at position {}: {}", pos, message)
                } else {
                    write!(f, "Parse error: {}", message)
                }
            }
        }
    }
}

impl std::error::Error for BackTranslationError {}

/// Warnings generated during parsing (non-fatal issues)
#[derive(Debug, Clone, PartialEq)]
pub enum BackTranslationWarning {
    /// Missing expected indicator (e.g., numeric indicator before number)
    MissingIndicator {
        indicator: String,
        position: usize,
    },
    /// Unexpected indicator in context
    UnexpectedIndicator {
        indicator: String,
        position: usize,
    },
    /// Automatically inserted missing structure element
    AutoInserted {
        element: String,
        position: usize,
    },
}

impl fmt::Display for BackTranslationWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackTranslationWarning::MissingIndicator { indicator, position } => {
                write!(f, "Missing {} indicator at position {}", indicator, position)
            }
            BackTranslationWarning::UnexpectedIndicator { indicator, position } => {
                write!(f, "Unexpected {} indicator at position {}", indicator, position)
            }
            BackTranslationWarning::AutoInserted { element, position } => {
                write!(f, "Auto-inserted {} at position {}", element, position)
            }
        }
    }
}

/// Result of parsing braille to MathML
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// The generated MathML string (None if parsing failed completely)
    pub mathml: Option<String>,
    /// Errors encountered during parsing
    pub errors: Vec<BackTranslationError>,
    /// Warnings generated during parsing
    pub warnings: Vec<BackTranslationWarning>,
}

impl ParseResult {
    /// Create a successful result with MathML
    pub fn success(mathml: String) -> Self {
        ParseResult {
            mathml: Some(mathml),
            errors: vec![],
            warnings: vec![],
        }
    }

    /// Create a failed result with an error
    pub fn failure(error: BackTranslationError) -> Self {
        ParseResult {
            mathml: None,
            errors: vec![error],
            warnings: vec![],
        }
    }

    /// Create a partial success with MathML and warnings
    pub fn partial(mathml: String, warnings: Vec<BackTranslationWarning>) -> Self {
        ParseResult {
            mathml: Some(mathml),
            errors: vec![],
            warnings,
        }
    }

    /// Check if parsing was successful (has MathML and no errors)
    pub fn is_success(&self) -> bool {
        self.mathml.is_some() && self.errors.is_empty()
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = BackTranslationError::UnrecognizedSymbol {
            position: 5,
            braille: "\u{2801}".to_string(),
        };
        assert!(err.to_string().contains("position 5"));

        let err = BackTranslationError::EmptyInput;
        assert_eq!(err.to_string(), "Empty braille input");
    }

    #[test]
    fn test_parse_result() {
        let success = ParseResult::success("<math><mn>1</mn></math>".to_string());
        assert!(success.is_success());
        assert!(!success.has_warnings());

        let failure = ParseResult::failure(BackTranslationError::EmptyInput);
        assert!(!failure.is_success());
        assert!(!failure.has_warnings());
    }
}
