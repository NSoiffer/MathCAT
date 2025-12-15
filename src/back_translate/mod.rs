//! Braille to MathML back-translation module
//!
//! This module provides functionality to convert braille (Unicode braille patterns)
//! back to MathML. It supports multiple braille codes including Nemeth and UEB.
//!
//! # Example
//! ```ignore
//! use mathcat::back_translate::{braille_to_mathml, BrailleCode};
//!
//! let braille = "\u{283C}\u{2802}\u{280C}\u{2806}";  // "1/2" in Nemeth
//! let mathml = braille_to_mathml(braille, BrailleCode::Nemeth)?;
//! ```

#![allow(clippy::needless_return)]

mod errors;
mod semantic;
mod mathml_gen;
mod nemeth;
mod ueb;

pub use errors::{BackTranslationError, BackTranslationWarning, ParseResult};
pub use semantic::MathNode;

use crate::errors::Result;

/// Supported braille codes for back-translation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrailleCode {
    /// Nemeth braille code (US mathematical braille)
    Nemeth,
    /// Unified English Braille (UEB) technical
    UEB,
    /// CMU Spanish mathematical braille
    CMU,
}

impl std::fmt::Display for BrailleCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrailleCode::Nemeth => write!(f, "Nemeth"),
            BrailleCode::UEB => write!(f, "UEB"),
            BrailleCode::CMU => write!(f, "CMU"),
        }
    }
}

impl std::str::FromStr for BrailleCode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nemeth" => Ok(BrailleCode::Nemeth),
            "ueb" => Ok(BrailleCode::UEB),
            "cmu" => Ok(BrailleCode::CMU),
            _ => Err(format!("Unknown braille code: {}", s)),
        }
    }
}

/// Convert braille to MathML
///
/// # Arguments
/// * `braille` - Unicode braille string (U+2800-U+28FF range)
/// * `code` - The braille code to use for parsing
///
/// # Returns
/// * `Ok(String)` - MathML string on success
/// * `Err(Error)` - Parse error with details
pub fn braille_to_mathml(braille: &str, code: BrailleCode) -> Result<String> {
    let result = braille_to_mathml_detailed(braille, code);
    if let Some(mathml) = result.mathml {
        Ok(mathml)
    } else if !result.errors.is_empty() {
        bail!("{}", result.errors[0])
    } else {
        bail!("Failed to parse braille: unknown error")
    }
}

/// Convert braille to MathML with detailed results
///
/// This function returns parsing results including any errors or warnings,
/// even if a partial result was produced.
///
/// # Arguments
/// * `braille` - Unicode braille string (U+2800-U+28FF range)
/// * `code` - The braille code to use for parsing
///
/// # Returns
/// A `ParseResult` containing:
/// * `mathml` - The generated MathML (if successful)
/// * `errors` - Any parsing errors encountered
/// * `warnings` - Any warnings generated during parsing
pub fn braille_to_mathml_detailed(braille: &str, code: BrailleCode) -> ParseResult {
    match code {
        BrailleCode::Nemeth => nemeth::parse_nemeth(braille),
        BrailleCode::UEB => ueb::parse_ueb(braille),
        BrailleCode::CMU => {
            // CMU parser - placeholder for Phase 5
            ParseResult {
                mathml: None,
                errors: vec![BackTranslationError::UnsupportedCode { code: "CMU".to_string() }],
                warnings: vec![],
            }
        }
    }
}

/// Get the list of braille codes that support back-translation
pub fn get_supported_back_translation_codes() -> Vec<String> {
    vec!["Nemeth".to_string(), "UEB".to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_braille_code_from_str() {
        assert_eq!(BrailleCode::from_str("nemeth"), Ok(BrailleCode::Nemeth));
        assert_eq!(BrailleCode::from_str("Nemeth"), Ok(BrailleCode::Nemeth));
        assert_eq!(BrailleCode::from_str("NEMETH"), Ok(BrailleCode::Nemeth));
        assert_eq!(BrailleCode::from_str("ueb"), Ok(BrailleCode::UEB));
        assert_eq!(BrailleCode::from_str("cmu"), Ok(BrailleCode::CMU));
        assert!(BrailleCode::from_str("unknown").is_err());
    }

    #[test]
    fn test_braille_code_display() {
        assert_eq!(format!("{}", BrailleCode::Nemeth), "Nemeth");
        assert_eq!(format!("{}", BrailleCode::UEB), "UEB");
        assert_eq!(format!("{}", BrailleCode::CMU), "CMU");
    }
}
