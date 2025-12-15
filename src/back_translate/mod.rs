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
mod cmu;
mod code_switch;
mod spatial;

pub use errors::{BackTranslationError, BackTranslationWarning, ParseResult};
pub use semantic::MathNode;
pub use code_switch::{detect_code, parse_with_code_detection, CodeDetectionResult, BrailleSegment};
pub use spatial::{parse_with_spatial, has_spatial_layout, Matrix, MatrixType};

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
        BrailleCode::CMU => cmu::parse_cmu(braille),
    }
}

/// Get the list of braille codes that support back-translation
pub fn get_supported_back_translation_codes() -> Vec<String> {
    vec!["Nemeth".to_string(), "UEB".to_string(), "CMU".to_string()]
}

/// Convert braille to MathML with automatic code detection
///
/// This function automatically detects whether the input is Nemeth or UEB
/// and handles code switching within documents.
///
/// # Arguments
/// * `braille` - Unicode braille string (U+2800-U+28FF range)
///
/// # Returns
/// * `Ok(String)` - MathML string on success
/// * `Err(Error)` - Parse error with details
pub fn braille_to_mathml_auto(braille: &str) -> Result<String> {
    let result = braille_to_mathml_auto_detailed(braille);
    if let Some(mathml) = result.mathml {
        Ok(mathml)
    } else if !result.errors.is_empty() {
        bail!("{}", result.errors[0])
    } else {
        bail!("Failed to parse braille: unknown error")
    }
}

/// Convert braille to MathML with automatic code detection and detailed results
///
/// This function automatically detects the braille code and handles:
/// - Nemeth vs UEB detection based on patterns
/// - Code switching (e.g., Nemeth within UEB documents)
/// - Spatial layouts (matrices, multi-line expressions)
///
/// # Arguments
/// * `braille` - Unicode braille string (U+2800-U+28FF range)
///
/// # Returns
/// A `ParseResult` containing the MathML and any errors/warnings
pub fn braille_to_mathml_auto_detailed(braille: &str) -> ParseResult {
    // First check for spatial layout
    if spatial::has_spatial_layout(braille) {
        let detection = code_switch::detect_code(braille);
        return spatial::parse_with_spatial(braille, detection.primary_code);
    }

    // Use code detection and switching
    code_switch::parse_with_code_detection(braille)
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

    #[test]
    fn test_auto_detect_nemeth() {
        // Nemeth number pattern
        let braille = "\u{283C}\u{2802}\u{2806}\u{2812}"; // 123 in Nemeth
        let result = braille_to_mathml_auto_detailed(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mn>123</mn>"));
    }

    #[test]
    fn test_auto_detect_simple_expression() {
        // Nemeth x + 1
        let braille = "\u{282D}\u{282C}\u{283C}\u{2802}";
        let result = braille_to_mathml_auto_detailed(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>+</mo>"));
    }

    #[test]
    fn test_code_detection_api() {
        let braille = "\u{283C}\u{2802}"; // Nemeth 1
        let detection = detect_code(braille);
        assert_eq!(detection.primary_code, BrailleCode::Nemeth);
        assert!(!detection.has_code_switching);
    }

    #[test]
    fn test_spatial_detection_api() {
        let single_line = "\u{283C}\u{2802}";
        assert!(!has_spatial_layout(single_line));

        let multi_line = "\u{283C}\u{2802}\n\u{283C}\u{2806}";
        assert!(has_spatial_layout(multi_line));
    }

    #[test]
    fn test_cmu_parse_number() {
        // CMU number 123
        let braille = "\u{283C}\u{2801}\u{2803}\u{2809}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::CMU);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mn>123</mn>"));
    }

    #[test]
    fn test_cmu_parse_expression() {
        // CMU x + y
        let braille = "\u{282D}\u{282E}\u{283D}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::CMU);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>x</mi>"));
        assert!(mathml.contains("<mo>+</mo>"));
        assert!(mathml.contains("<mi>y</mi>"));
    }

    #[test]
    fn test_supported_codes_includes_cmu() {
        let codes = get_supported_back_translation_codes();
        assert!(codes.contains(&"CMU".to_string()));
    }
}
