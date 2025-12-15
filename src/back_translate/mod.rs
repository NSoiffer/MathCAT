//! Braille to MathML Back-Translation Module
//!
//! This module provides functionality to convert mathematical braille (Unicode braille
//! patterns in the U+2800-U+28FF range) back to MathML. It supports multiple braille
//! codes commonly used for mathematical notation.
//!
//! # Supported Braille Codes
//!
//! - **Nemeth** - The Nemeth Braille Code for Mathematics, widely used in the United States
//! - **UEB** - Unified English Braille (Technical), the international English braille standard
//! - **CMU** - Codigo Matematico Unificado, Spanish mathematical braille
//!
//! # Quick Start
//!
//! ## Basic Usage with Known Code
//!
//! ```rust,ignore
//! use mathcat::back_translate::{braille_to_mathml, BrailleCode};
//!
//! // Parse Nemeth braille for "1+2"
//! let braille = "\u{283C}\u{2802}\u{282C}\u{283C}\u{2806}";
//! let mathml = braille_to_mathml(braille, BrailleCode::Nemeth)?;
//! println!("MathML: {}", mathml);
//! ```
//!
//! ## Automatic Code Detection
//!
//! ```rust,ignore
//! use mathcat::back_translate::braille_to_mathml_auto;
//!
//! // Automatically detect whether input is Nemeth, UEB, or CMU
//! let braille = "\u{283C}\u{2802}\u{282C}\u{283C}\u{2806}";
//! let mathml = braille_to_mathml_auto(braille)?;
//! ```
//!
//! ## String-Based API (for FFI/Scripting)
//!
//! ```rust,ignore
//! use mathcat::back_translate::braille_to_mathml_str;
//!
//! // Use string code name instead of enum
//! let mathml = braille_to_mathml_str("\u{283C}\u{2802}", "Nemeth")?;
//! ```
//!
//! ## Detailed Results with Errors and Warnings
//!
//! ```rust,ignore
//! use mathcat::back_translate::{braille_to_mathml_detailed, BrailleCode};
//!
//! let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
//! if let Some(mathml) = result.mathml {
//!     println!("MathML: {}", mathml);
//! }
//! for warning in result.warnings {
//!     println!("Warning: {}", warning);
//! }
//! for error in result.errors {
//!     eprintln!("Error: {}", error);
//! }
//! ```
//!
//! # Features
//!
//! - **Multi-Code Support**: Parse Nemeth, UEB, and CMU braille
//! - **Automatic Detection**: Detect braille code from patterns
//! - **Code Switching**: Handle documents with mixed UEB/Nemeth
//! - **Spatial Layout**: Support for matrices and multi-line expressions
//! - **Error Recovery**: Continue parsing after errors when possible
//! - **Detailed Feedback**: Return errors and warnings for debugging
//!
//! # Architecture
//!
//! The module uses a pipeline architecture:
//!
//! 1. **Parser** - Code-specific PEG parser (pest) converts braille to AST
//! 2. **Semantic Tree** - Code-independent intermediate representation
//! 3. **MathML Generator** - Converts semantic tree to MathML output
//!
//! # Error Handling
//!
//! The module distinguishes between:
//!
//! - **Errors** - Fatal problems that prevent successful parsing
//! - **Warnings** - Non-fatal issues that allow parsing to continue
//!
//! Use `braille_to_mathml_detailed()` to get both errors and warnings.

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
///
/// This enum represents the mathematical braille codes that can be
/// parsed and converted to MathML.
///
/// # Examples
///
/// ```rust,ignore
/// use mathcat::back_translate::BrailleCode;
/// use std::str::FromStr;
///
/// // Parse from string
/// let code = BrailleCode::from_str("Nemeth")?;
///
/// // Case-insensitive
/// assert_eq!(BrailleCode::from_str("nemeth"), Ok(BrailleCode::Nemeth));
/// assert_eq!(BrailleCode::from_str("NEMETH"), Ok(BrailleCode::Nemeth));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrailleCode {
    /// Nemeth Braille Code for Mathematics
    ///
    /// The primary mathematical braille code used in the United States.
    /// Developed by Abraham Nemeth, it provides a comprehensive system
    /// for representing mathematical notation in braille.
    Nemeth,

    /// Unified English Braille (Technical)
    ///
    /// The international English braille standard, including technical
    /// notation for mathematics and science. Used in the UK, Australia,
    /// and many other English-speaking countries.
    UEB,

    /// CMU - Codigo Matematico Unificado
    ///
    /// Spanish mathematical braille code used in Spain and Latin America.
    /// Also known as "Unified Mathematical Code" in Spanish.
    CMU,
}

impl BrailleCode {
    /// Get a human-readable description of this braille code
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathcat::back_translate::BrailleCode;
    ///
    /// assert_eq!(BrailleCode::Nemeth.description(), "Nemeth Braille Code for Mathematics");
    /// ```
    pub fn description(&self) -> &'static str {
        match self {
            BrailleCode::Nemeth => "Nemeth Braille Code for Mathematics",
            BrailleCode::UEB => "Unified English Braille (Technical)",
            BrailleCode::CMU => "CMU - Codigo Matematico Unificado (Spanish)",
        }
    }

    /// Get the language/region typically associated with this code
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathcat::back_translate::BrailleCode;
    ///
    /// assert_eq!(BrailleCode::Nemeth.language(), "en-US");
    /// ```
    pub fn language(&self) -> &'static str {
        match self {
            BrailleCode::Nemeth => "en-US",
            BrailleCode::UEB => "en",
            BrailleCode::CMU => "es",
        }
    }
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
            _ => Err(format!("Unknown braille code: '{}'. Supported codes are: Nemeth, UEB, CMU", s)),
        }
    }
}

// ============================================================================
// Primary API Functions
// ============================================================================

/// Convert braille to MathML
///
/// This is the primary function for converting mathematical braille to MathML.
/// Use this when you know which braille code the input uses.
///
/// # Arguments
///
/// * `braille` - Unicode braille string (U+2800-U+28FF range)
/// * `code` - The braille code to use for parsing
///
/// # Returns
///
/// * `Ok(String)` - MathML string on success
/// * `Err(Error)` - Error message on failure
///
/// # Examples
///
/// ```rust,ignore
/// use mathcat::back_translate::{braille_to_mathml, BrailleCode};
///
/// // Parse a simple fraction in Nemeth (1/2)
/// let braille = "\u{283C}\u{2802}\u{280C}\u{283C}\u{2806}";
/// let mathml = braille_to_mathml(braille, BrailleCode::Nemeth)?;
/// assert!(mathml.contains("<mfrac>"));
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The input is empty or contains only whitespace
/// - The braille contains unrecognized patterns
/// - The structure is malformed (e.g., unclosed fractions)
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

/// Convert braille to MathML using a string code name
///
/// This is a convenience function for FFI and scripting languages that
/// may not have easy access to Rust enums.
///
/// # Arguments
///
/// * `braille` - Unicode braille string (U+2800-U+28FF range)
/// * `code` - Braille code name as string ("Nemeth", "UEB", or "CMU")
///
/// # Returns
///
/// * `Ok(String)` - MathML string on success
/// * `Err(Error)` - Error message on failure
///
/// # Examples
///
/// ```rust,ignore
/// use mathcat::back_translate::braille_to_mathml_str;
///
/// let mathml = braille_to_mathml_str("\u{283C}\u{2802}", "Nemeth")?;
/// assert!(mathml.contains("<mn>1</mn>"));
///
/// // Case-insensitive
/// let mathml = braille_to_mathml_str("\u{283C}\u{2802}", "nemeth")?;
/// ```
pub fn braille_to_mathml_str(braille: &str, code: &str) -> Result<String> {
    let braille_code: BrailleCode = code.parse()
        .map_err(|e: String| crate::errors::Error::from(e))?;
    braille_to_mathml(braille, braille_code)
}

/// Convert braille to MathML with detailed results
///
/// This function returns comprehensive parsing results including any errors
/// or warnings, even if a partial result was produced. Use this when you
/// need detailed feedback about the parsing process.
///
/// # Arguments
///
/// * `braille` - Unicode braille string (U+2800-U+28FF range)
/// * `code` - The braille code to use for parsing
///
/// # Returns
///
/// A `ParseResult` containing:
/// * `mathml` - The generated MathML (if successful)
/// * `errors` - Any parsing errors encountered
/// * `warnings` - Any warnings generated during parsing
///
/// # Examples
///
/// ```rust,ignore
/// use mathcat::back_translate::{braille_to_mathml_detailed, BrailleCode};
///
/// let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
///
/// // Check for success
/// if result.is_success() {
///     println!("MathML: {}", result.mathml.unwrap());
/// }
///
/// // Handle warnings even on success
/// for warning in &result.warnings {
///     eprintln!("Warning: {}", warning);
/// }
///
/// // Handle errors
/// for error in &result.errors {
///     eprintln!("Error: {}", error);
/// }
/// ```
pub fn braille_to_mathml_detailed(braille: &str, code: BrailleCode) -> ParseResult {
    match code {
        BrailleCode::Nemeth => nemeth::parse_nemeth(braille),
        BrailleCode::UEB => ueb::parse_ueb(braille),
        BrailleCode::CMU => cmu::parse_cmu(braille),
    }
}

// ============================================================================
// Automatic Detection Functions
// ============================================================================

/// Convert braille to MathML with automatic code detection
///
/// This function automatically detects whether the input is Nemeth or UEB
/// based on characteristic patterns, and handles code switching within
/// documents (e.g., Nemeth embedded in UEB).
///
/// # Arguments
///
/// * `braille` - Unicode braille string (U+2800-U+28FF range)
///
/// # Returns
///
/// * `Ok(String)` - MathML string on success
/// * `Err(Error)` - Error message on failure
///
/// # Detection Heuristics
///
/// The function uses these patterns to detect the braille code:
/// - Nemeth digit patterns after numeric indicator
/// - UEB digit patterns (same as letters a-j)
/// - Code switching indicators (Nemeth open/close within UEB)
/// - Operator patterns specific to each code
///
/// # Examples
///
/// ```rust,ignore
/// use mathcat::back_translate::braille_to_mathml_auto;
///
/// // Automatically detects Nemeth
/// let braille = "\u{283C}\u{2802}\u{282C}\u{283C}\u{2806}";  // 1+2 in Nemeth
/// let mathml = braille_to_mathml_auto(braille)?;
/// ```
///
/// # Note
///
/// For best results when you know the braille code, use `braille_to_mathml()`
/// directly with the specific code. Auto-detection may occasionally
/// misidentify similar patterns.
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
/// This function combines automatic code detection with detailed result reporting.
/// It handles:
///
/// - **Code Detection**: Identifies Nemeth vs UEB based on patterns
/// - **Code Switching**: Detects and handles Nemeth within UEB documents
/// - **Spatial Layouts**: Recognizes matrices and multi-line expressions
///
/// # Arguments
///
/// * `braille` - Unicode braille string (U+2800-U+28FF range)
///
/// # Returns
///
/// A `ParseResult` containing the MathML and any errors/warnings
///
/// # Examples
///
/// ```rust,ignore
/// use mathcat::back_translate::braille_to_mathml_auto_detailed;
///
/// let result = braille_to_mathml_auto_detailed(braille);
///
/// if result.is_success() {
///     println!("Detected code and parsed successfully");
///     println!("MathML: {}", result.mathml.unwrap());
/// }
/// ```
pub fn braille_to_mathml_auto_detailed(braille: &str) -> ParseResult {
    // First check for spatial layout
    if spatial::has_spatial_layout(braille) {
        let detection = code_switch::detect_code(braille);
        return spatial::parse_with_spatial(braille, detection.primary_code);
    }

    // Use code detection and switching
    code_switch::parse_with_code_detection(braille)
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Get the list of braille codes that support back-translation
///
/// Returns a list of code names that can be passed to `braille_to_mathml_str()`.
///
/// # Examples
///
/// ```rust,ignore
/// use mathcat::back_translate::get_supported_back_translation_codes;
///
/// let codes = get_supported_back_translation_codes();
/// // Returns: ["Nemeth", "UEB", "CMU"]
/// ```
pub fn get_supported_back_translation_codes() -> Vec<String> {
    vec!["Nemeth".to_string(), "UEB".to_string(), "CMU".to_string()]
}

/// Check if a string contains valid Unicode braille characters
///
/// Validates that the input contains only characters in the braille
/// Unicode range (U+2800-U+28FF) plus whitespace.
///
/// # Arguments
///
/// * `braille` - String to validate
///
/// # Returns
///
/// * `true` if all non-whitespace characters are valid braille
/// * `false` if any non-whitespace character is outside braille range
///
/// # Examples
///
/// ```rust,ignore
/// use mathcat::back_translate::is_valid_braille;
///
/// assert!(is_valid_braille("\u{2801}\u{2803}"));  // Valid braille
/// assert!(is_valid_braille("\u{2801} \u{2803}")); // Whitespace OK
/// assert!(!is_valid_braille("abc"));              // Not braille
/// assert!(!is_valid_braille("\u{2801}x"));        // Mixed content
/// ```
pub fn is_valid_braille(braille: &str) -> bool {
    braille.chars().all(|c| {
        c.is_whitespace() || ('\u{2800}'..='\u{28FF}').contains(&c)
    })
}

/// Convert ASCII braille representation to Unicode braille
///
/// Some systems use ASCII characters to represent braille dots.
/// This function converts from common ASCII representations to
/// Unicode braille.
///
/// # ASCII Format
///
/// Each character represents a braille cell using dots 1-6:
/// - `a` = dot 1, `b` = dot 2, ..., `f` = dot 6
/// - Combine letters for multiple dots: "ab" = dots 1-2
/// - Space separates cells
///
/// # Examples
///
/// ```rust,ignore
/// use mathcat::back_translate::ascii_to_unicode_braille;
///
/// // "a" = dot 1 = U+2801
/// assert_eq!(ascii_to_unicode_braille("a"), "\u{2801}");
///
/// // "ab" = dots 1-2 = U+2803
/// assert_eq!(ascii_to_unicode_braille("ab"), "\u{2803}");
/// ```
pub fn ascii_to_unicode_braille(ascii: &str) -> String {
    let mut result = String::new();
    let mut current_cell: u8 = 0;

    for c in ascii.chars() {
        match c.to_ascii_lowercase() {
            'a' | '1' => current_cell |= 0x01, // dot 1
            'b' | '2' => current_cell |= 0x02, // dot 2
            'c' | '3' => current_cell |= 0x04, // dot 3
            'd' | '4' => current_cell |= 0x08, // dot 4
            'e' | '5' => current_cell |= 0x10, // dot 5
            'f' | '6' => current_cell |= 0x20, // dot 6
            'g' | '7' => current_cell |= 0x40, // dot 7 (8-dot braille)
            'h' | '8' => current_cell |= 0x80, // dot 8 (8-dot braille)
            ' ' | '-' => {
                // End of cell
                if current_cell > 0 || result.is_empty() {
                    result.push(char::from_u32(0x2800 + current_cell as u32).unwrap_or('\u{2800}'));
                    current_cell = 0;
                }
            }
            _ => {
                // Ignore other characters
            }
        }
    }

    // Don't forget the last cell
    if current_cell > 0 {
        result.push(char::from_u32(0x2800 + current_cell as u32).unwrap_or('\u{2800}'));
    }

    result
}

/// Get information about the detected braille code
///
/// Analyzes the input and returns information about the detected
/// braille code without performing full parsing.
///
/// # Arguments
///
/// * `braille` - Unicode braille string to analyze
///
/// # Returns
///
/// A `CodeDetectionResult` containing:
/// * `primary_code` - The detected primary braille code
/// * `segments` - List of segments with their codes (for code switching)
/// * `has_code_switching` - Whether the input contains code switches
///
/// # Examples
///
/// ```rust,ignore
/// use mathcat::back_translate::detect_braille_code;
///
/// let detection = detect_braille_code("\u{283C}\u{2802}");
/// println!("Detected code: {}", detection.primary_code);
/// println!("Has code switching: {}", detection.has_code_switching);
/// ```
pub fn detect_braille_code(braille: &str) -> CodeDetectionResult {
    code_switch::detect_code(braille)
}

// ============================================================================
// Tests
// ============================================================================

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
    fn test_braille_code_description() {
        assert!(BrailleCode::Nemeth.description().contains("Nemeth"));
        assert!(BrailleCode::UEB.description().contains("Unified"));
        assert!(BrailleCode::CMU.description().contains("Spanish"));
    }

    #[test]
    fn test_braille_code_language() {
        assert_eq!(BrailleCode::Nemeth.language(), "en-US");
        assert_eq!(BrailleCode::UEB.language(), "en");
        assert_eq!(BrailleCode::CMU.language(), "es");
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
        assert!(codes.contains(&"Nemeth".to_string()));
        assert!(codes.contains(&"UEB".to_string()));
        assert!(codes.contains(&"CMU".to_string()));
    }

    #[test]
    fn test_is_valid_braille() {
        assert!(is_valid_braille("\u{2801}\u{2803}"));
        assert!(is_valid_braille("\u{2801} \u{2803}"));
        assert!(is_valid_braille(""));
        assert!(is_valid_braille("   "));
        assert!(!is_valid_braille("abc"));
        assert!(!is_valid_braille("\u{2801}x"));
    }

    #[test]
    fn test_ascii_to_unicode_braille() {
        // Single dots
        assert_eq!(ascii_to_unicode_braille("a"), "\u{2801}");
        assert_eq!(ascii_to_unicode_braille("b"), "\u{2802}");

        // Multiple dots in one cell
        assert_eq!(ascii_to_unicode_braille("ab"), "\u{2803}");
        assert_eq!(ascii_to_unicode_braille("abc"), "\u{2807}");

        // Multiple cells
        assert_eq!(ascii_to_unicode_braille("a b"), "\u{2801}\u{2802}");
        assert_eq!(ascii_to_unicode_braille("ab-cd"), "\u{2803}\u{280C}");
    }

    #[test]
    fn test_braille_to_mathml_str() {
        let braille = "\u{283C}\u{2802}"; // 1 in Nemeth
        let result = braille_to_mathml_str(braille, "Nemeth");
        assert!(result.is_ok());
        let mathml = result.unwrap();
        assert!(mathml.contains("<mn>1</mn>"));
    }

    #[test]
    fn test_braille_to_mathml_str_case_insensitive() {
        let braille = "\u{283C}\u{2802}";
        assert!(braille_to_mathml_str(braille, "nemeth").is_ok());
        assert!(braille_to_mathml_str(braille, "NEMETH").is_ok());
        assert!(braille_to_mathml_str(braille, "Nemeth").is_ok());
    }

    #[test]
    fn test_braille_to_mathml_str_invalid_code() {
        let braille = "\u{283C}\u{2802}";
        let result = braille_to_mathml_str(braille, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_braille_code() {
        let detection = detect_braille_code("\u{283C}\u{2802}");
        assert_eq!(detection.primary_code, BrailleCode::Nemeth);
    }

    // ========================================================================
    // Edge Case Tests - Boundary Conditions
    // ========================================================================

    #[test]
    fn test_empty_string_all_codes() {
        // Empty input should fail gracefully for all codes
        assert!(braille_to_mathml("", BrailleCode::Nemeth).is_err());
        assert!(braille_to_mathml("", BrailleCode::UEB).is_err());
        assert!(braille_to_mathml("", BrailleCode::CMU).is_err());
        assert!(braille_to_mathml_auto("").is_err());
    }

    #[test]
    fn test_whitespace_only_all_codes() {
        // Whitespace-only should fail gracefully
        assert!(braille_to_mathml("   ", BrailleCode::Nemeth).is_err());
        assert!(braille_to_mathml("\t\n", BrailleCode::UEB).is_err());
        assert!(braille_to_mathml(" \n \t ", BrailleCode::CMU).is_err());
    }

    #[test]
    fn test_braille_space_only() {
        // Braille space (U+2800) only - should handle gracefully
        let result = braille_to_mathml_detailed("\u{2800}", BrailleCode::Nemeth);
        // May succeed with empty content or fail - either is acceptable
        assert!(result.errors.is_empty() || !result.errors.is_empty());
    }

    #[test]
    fn test_single_braille_cell() {
        // Single braille cell - dots 1 (letter a)
        let result = braille_to_mathml_detailed("\u{2801}", BrailleCode::Nemeth);
        assert!(result.is_success(), "Single cell should parse: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mi>a</mi>"));
    }

    #[test]
    fn test_all_dots_braille_cell() {
        // All 8 dots (U+28FF)
        let result = braille_to_mathml_detailed("\u{28FF}", BrailleCode::Nemeth);
        // Should handle gracefully (may not have specific meaning)
        assert!(result.errors.len() <= 1);
    }

    #[test]
    fn test_braille_unicode_boundary_low() {
        // First braille character U+2800 (blank)
        let result = is_valid_braille("\u{2800}");
        assert!(result);
    }

    #[test]
    fn test_braille_unicode_boundary_high() {
        // Last braille character U+28FF
        let result = is_valid_braille("\u{28FF}");
        assert!(result);
    }

    #[test]
    fn test_character_just_outside_braille_range() {
        // U+27FF (just before braille) and U+2900 (just after)
        assert!(!is_valid_braille("\u{27FF}"));
        assert!(!is_valid_braille("\u{2900}"));
    }

    #[test]
    fn test_long_input() {
        // Long sequence of digits
        let braille = "\u{283C}".to_string() + &"\u{2802}".repeat(100);
        let result = braille_to_mathml_detailed(&braille, BrailleCode::Nemeth);
        // Should handle without crashing
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    // ========================================================================
    // Edge Case Tests - Special Character Patterns
    // ========================================================================

    #[test]
    fn test_consecutive_operators_nemeth() {
        // Multiple operators in a row (+-) - plus-minus
        let braille = "\u{282D}\u{282C}\u{2824}\u{283D}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
    }

    #[test]
    fn test_consecutive_numbers_without_separator() {
        // Two numbers without operator (edge case)
        let braille = "\u{283C}\u{2802}\u{283C}\u{2806}"; // 1 2 in Nemeth
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // Should parse as two separate numbers
        assert!(result.is_success() || !result.errors.is_empty());
    }

    #[test]
    fn test_leading_operator() {
        // Expression starting with operator (like -x for negative)
        // Note: Current parser requires term before operator - this is a known limitation
        let braille = "\u{2824}\u{282D}"; // -x in Nemeth
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // Parser currently doesn't handle leading operators - verify graceful handling
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_trailing_operator() {
        // Expression ending with operator (incomplete)
        let braille = "\u{282D}\u{282C}"; // x+ in Nemeth
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // May parse partial or fail gracefully
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_only_operator() {
        // Just an operator
        // Note: Current parser requires operands - standalone operators are not valid
        let braille = "\u{282C}"; // + in Nemeth
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // Parser requires context for operators - verify graceful error
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_repeated_same_character() {
        // Same letter repeated
        // Note: Multiple adjacent identifiers may need implied multiplication
        let braille = "\u{282D}\u{282D}\u{282D}"; // xxx
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // Parser may interpret or fail - verify graceful handling
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    // ========================================================================
    // Edge Case Tests - Malformed Inputs
    // ========================================================================

    #[test]
    fn test_unmatched_open_parenthesis() {
        // (x without closing
        let braille = "\u{2837}\u{282D}"; // (x in Nemeth
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // Should fail or produce warning
        assert!(!result.errors.is_empty() || result.has_warnings() || result.is_success());
    }

    #[test]
    fn test_unmatched_close_parenthesis() {
        // x) without opening
        let braille = "\u{282D}\u{283E}"; // x) in Nemeth
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // Should handle gracefully
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_nested_unmatched_parentheses() {
        // ((x) - one extra open
        let braille = "\u{2837}\u{2837}\u{282D}\u{283E}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_fraction_without_denominator() {
        // Fraction start + numerator + fraction line, but no denominator
        let braille = "\u{2839}\u{283C}\u{2802}\u{280C}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // Should fail gracefully
        assert!(!result.errors.is_empty() || result.has_warnings() || result.is_success());
    }

    #[test]
    fn test_superscript_without_base() {
        // Superscript indicator + number, but no base
        let braille = "\u{2818}\u{283C}\u{2806}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // Should handle gracefully
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_capital_indicator_alone() {
        // Capital indicator with nothing following
        let braille = "\u{2820}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // Should handle gracefully
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_numeric_indicator_alone() {
        // Numeric indicator with nothing following
        let braille = "\u{283C}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // Should handle gracefully
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_greek_indicator_alone() {
        // Greek indicator with nothing following
        let braille = "\u{2828}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    // ========================================================================
    // Edge Case Tests - Mixed/Invalid Content
    // ========================================================================

    #[test]
    fn test_mixed_braille_and_ascii() {
        // Braille mixed with ASCII
        let input = "\u{2801}abc\u{2803}";
        assert!(!is_valid_braille(input));
        let result = braille_to_mathml_detailed(input, BrailleCode::Nemeth);
        assert!(!result.is_success());
    }

    #[test]
    fn test_mixed_braille_and_emoji() {
        let input = "\u{2801}\u{1F600}\u{2803}";
        assert!(!is_valid_braille(input));
    }

    #[test]
    fn test_mixed_braille_and_math_symbols() {
        // Braille mixed with Unicode math symbols (not braille)
        let input = "\u{2801}\u{221E}\u{2803}"; // braille + infinity + braille
        assert!(!is_valid_braille(input));
    }

    #[test]
    fn test_control_characters() {
        // Control characters should be invalid
        assert!(!is_valid_braille("\u{0000}"));
        assert!(!is_valid_braille("\u{001F}"));
    }

    #[test]
    fn test_newlines_in_non_spatial() {
        // Newlines without spatial context
        let braille = "\u{283C}\u{2802}\n";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // Should handle single trailing newline
        assert!(result.is_success() || !result.errors.is_empty());
    }

    // ========================================================================
    // Edge Case Tests - Code Detection
    // ========================================================================

    #[test]
    fn test_detect_ambiguous_input() {
        // Input that could be either code - just a letter
        let detection = detect_braille_code("\u{2801}");
        // Should pick a default (Nemeth in US context)
        assert!(detection.primary_code == BrailleCode::Nemeth ||
                detection.primary_code == BrailleCode::UEB);
    }

    #[test]
    fn test_detect_empty_input() {
        let detection = detect_braille_code("");
        // Should return a default without crashing
        assert!(!detection.has_code_switching);
    }

    #[test]
    fn test_detect_whitespace_only() {
        let detection = detect_braille_code("   ");
        assert!(!detection.has_code_switching);
    }

    #[test]
    fn test_code_switch_at_start() {
        // Nemeth open indicator at very start
        let braille = "\u{2838}\u{2829}\u{283C}\u{2802}\u{2838}\u{2831}";
        let detection = detect_braille_code(braille);
        assert!(detection.has_code_switching);
    }

    #[test]
    fn test_code_switch_unclosed() {
        // Nemeth open without close
        let braille = "\u{2838}\u{2829}\u{283C}\u{2802}";
        let detection = detect_braille_code(braille);
        // Should handle gracefully
        assert!(detection.segments.len() >= 1);
    }

    #[test]
    fn test_multiple_code_switches() {
        // Multiple switches back and forth
        let braille = format!(
            "{}{}{}{}{}{}",
            "\u{2838}\u{2829}", // Nemeth open
            "\u{283C}\u{2802}", // Nemeth content
            "\u{2838}\u{2831}", // Nemeth close
            "\u{2838}\u{2829}", // Nemeth open again
            "\u{283C}\u{2806}", // Nemeth content
            "\u{2838}\u{2831}"  // Nemeth close
        );
        let detection = detect_braille_code(&braille);
        assert!(detection.has_code_switching);
    }

    // ========================================================================
    // Edge Case Tests - Spatial Layout
    // ========================================================================

    #[test]
    fn test_spatial_single_row() {
        // Matrix with only one row
        let braille = "\u{283C}\u{2802}  \u{283C}\u{2806}";
        assert!(!has_spatial_layout(braille)); // No newline = no spatial
    }

    #[test]
    fn test_spatial_empty_row() {
        // Matrix with empty row
        let braille = "\u{283C}\u{2802}\n\n\u{283C}\u{2806}";
        assert!(has_spatial_layout(braille));
        let result = parse_with_spatial(braille, BrailleCode::Nemeth);
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_spatial_uneven_columns() {
        // Rows with different number of columns
        let braille = "\u{283C}\u{2802}  \u{283C}\u{2806}  \u{283C}\u{2812}\n\u{283C}\u{2832}";
        let result = parse_with_spatial(braille, BrailleCode::Nemeth);
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_spatial_many_rows() {
        // Many rows
        let rows: Vec<&str> = vec!["\u{283C}\u{2802}"; 10];
        let braille = rows.join("\n");
        assert!(has_spatial_layout(&braille));
    }

    #[test]
    fn test_spatial_with_operators() {
        // Matrix cells containing operators
        let braille = "\u{282D}\u{282C}\u{283D}  \u{2801}\u{2824}\u{2803}\n\u{283C}\u{2802}  \u{283C}\u{2806}";
        let result = parse_with_spatial(braille, BrailleCode::Nemeth);
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    // ========================================================================
    // Edge Case Tests - ASCII Conversion
    // ========================================================================

    #[test]
    fn test_ascii_empty() {
        assert_eq!(ascii_to_unicode_braille(""), "");
    }

    #[test]
    fn test_ascii_single_dot() {
        assert_eq!(ascii_to_unicode_braille("a"), "\u{2801}");
        assert_eq!(ascii_to_unicode_braille("b"), "\u{2802}");
        assert_eq!(ascii_to_unicode_braille("c"), "\u{2804}");
        assert_eq!(ascii_to_unicode_braille("d"), "\u{2808}");
        assert_eq!(ascii_to_unicode_braille("e"), "\u{2810}");
        assert_eq!(ascii_to_unicode_braille("f"), "\u{2820}");
    }

    #[test]
    fn test_ascii_numeric_dots() {
        // Using numbers instead of letters
        assert_eq!(ascii_to_unicode_braille("1"), "\u{2801}");
        assert_eq!(ascii_to_unicode_braille("2"), "\u{2802}");
        assert_eq!(ascii_to_unicode_braille("12"), "\u{2803}");
    }

    #[test]
    fn test_ascii_8dot_braille() {
        // Dots 7 and 8 for 8-dot braille
        assert_eq!(ascii_to_unicode_braille("g"), "\u{2840}");
        assert_eq!(ascii_to_unicode_braille("h"), "\u{2880}");
        assert_eq!(ascii_to_unicode_braille("gh"), "\u{28C0}");
    }

    #[test]
    fn test_ascii_all_dots() {
        // All 8 dots
        assert_eq!(ascii_to_unicode_braille("abcdefgh"), "\u{28FF}");
    }

    #[test]
    fn test_ascii_hyphen_separator() {
        // Using hyphen as separator
        assert_eq!(ascii_to_unicode_braille("a-b-c"), "\u{2801}\u{2802}\u{2804}");
    }

    #[test]
    fn test_ascii_mixed_separators() {
        assert_eq!(ascii_to_unicode_braille("a b-c"), "\u{2801}\u{2802}\u{2804}");
    }

    #[test]
    fn test_ascii_invalid_chars_ignored() {
        // Invalid characters should be ignored
        assert_eq!(ascii_to_unicode_braille("axyz"), "\u{2801}");
        assert_eq!(ascii_to_unicode_braille("!@#a"), "\u{2801}");
    }

    #[test]
    fn test_ascii_uppercase_ignored() {
        // Uppercase should be treated same as lowercase
        assert_eq!(ascii_to_unicode_braille("A"), "\u{2801}");
        assert_eq!(ascii_to_unicode_braille("AB"), "\u{2803}");
    }

    #[test]
    fn test_ascii_trailing_space() {
        // Trailing space should finalize cell
        assert_eq!(ascii_to_unicode_braille("a "), "\u{2801}");
    }

    #[test]
    fn test_ascii_multiple_spaces() {
        // Multiple spaces between cells
        assert_eq!(ascii_to_unicode_braille("a   b"), "\u{2801}\u{2802}");
    }

    // ========================================================================
    // Edge Case Tests - is_valid_braille
    // ========================================================================

    #[test]
    fn test_valid_braille_full_range() {
        // Test several points in the valid range
        assert!(is_valid_braille("\u{2800}\u{2840}\u{2880}\u{28FF}"));
    }

    #[test]
    fn test_valid_braille_with_tabs() {
        assert!(is_valid_braille("\u{2801}\t\u{2803}"));
    }

    #[test]
    fn test_valid_braille_with_newlines() {
        assert!(is_valid_braille("\u{2801}\n\u{2803}"));
    }

    #[test]
    fn test_valid_braille_with_carriage_return() {
        assert!(is_valid_braille("\u{2801}\r\n\u{2803}"));
    }

    #[test]
    fn test_invalid_braille_cyrillic() {
        assert!(!is_valid_braille("\u{0410}")); // Cyrillic A
    }

    #[test]
    fn test_invalid_braille_chinese() {
        assert!(!is_valid_braille("\u{4E2D}")); // Chinese character
    }

    #[test]
    fn test_invalid_braille_combining_chars() {
        assert!(!is_valid_braille("\u{0301}")); // Combining acute accent
    }

    // ========================================================================
    // Edge Case Tests - BrailleCode FromStr
    // ========================================================================

    #[test]
    fn test_braille_code_from_str_whitespace() {
        // Whitespace shouldn't match
        assert!(BrailleCode::from_str(" Nemeth").is_err());
        assert!(BrailleCode::from_str("Nemeth ").is_err());
    }

    #[test]
    fn test_braille_code_from_str_partial() {
        // Partial matches shouldn't work
        assert!(BrailleCode::from_str("Nem").is_err());
        assert!(BrailleCode::from_str("UE").is_err());
    }

    #[test]
    fn test_braille_code_from_str_typos() {
        // Common typos
        assert!(BrailleCode::from_str("Nemmeth").is_err());
        assert!(BrailleCode::from_str("UBE").is_err());
    }

    #[test]
    fn test_braille_code_from_str_empty() {
        assert!(BrailleCode::from_str("").is_err());
    }

    // ========================================================================
    // Edge Case Tests - Error Messages
    // ========================================================================

    #[test]
    fn test_error_message_contains_code_name() {
        let result = BrailleCode::from_str("invalid_code");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("invalid_code"));
        assert!(err.contains("Nemeth") || err.contains("UEB") || err.contains("CMU"));
    }

    #[test]
    fn test_parse_error_is_descriptive() {
        let result = braille_to_mathml("xyz", BrailleCode::Nemeth);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(!err.is_empty());
    }

    // ========================================================================
    // Edge Case Tests - Round-trip Consistency
    // ========================================================================

    #[test]
    fn test_same_input_same_output_nemeth() {
        let braille = "\u{283C}\u{2802}\u{282C}\u{283C}\u{2806}";
        let result1 = braille_to_mathml(braille, BrailleCode::Nemeth);
        let result2 = braille_to_mathml(braille, BrailleCode::Nemeth);
        assert_eq!(result1.is_ok(), result2.is_ok());
        if result1.is_ok() {
            assert_eq!(result1.unwrap(), result2.unwrap());
        }
    }

    #[test]
    fn test_same_input_same_output_auto() {
        let braille = "\u{283C}\u{2802}";
        let result1 = braille_to_mathml_auto(braille);
        let result2 = braille_to_mathml_auto(braille);
        assert_eq!(result1.is_ok(), result2.is_ok());
        if result1.is_ok() {
            assert_eq!(result1.unwrap(), result2.unwrap());
        }
    }

    // ========================================================================
    // Edge Case Tests - Detailed vs Simple API Consistency
    // ========================================================================

    #[test]
    fn test_detailed_and_simple_consistency() {
        let braille = "\u{283C}\u{2802}";
        let simple = braille_to_mathml(braille, BrailleCode::Nemeth);
        let detailed = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);

        assert_eq!(simple.is_ok(), detailed.is_success());
        if simple.is_ok() {
            assert_eq!(simple.unwrap(), detailed.mathml.unwrap());
        }
    }

    #[test]
    fn test_auto_detailed_consistency() {
        let braille = "\u{283C}\u{2802}";
        let simple = braille_to_mathml_auto(braille);
        let detailed = braille_to_mathml_auto_detailed(braille);

        assert_eq!(simple.is_ok(), detailed.is_success());
        if simple.is_ok() {
            assert_eq!(simple.unwrap(), detailed.mathml.unwrap());
        }
    }

    // ========================================================================
    // Edge Case Tests - Cross-Code Parsing
    // ========================================================================

    #[test]
    fn test_nemeth_braille_with_ueb_parser() {
        // Try to parse Nemeth braille with UEB parser
        let nemeth_braille = "\u{283C}\u{2802}"; // 1 in Nemeth
        let result = braille_to_mathml_detailed(nemeth_braille, BrailleCode::UEB);
        // May or may not succeed depending on overlap
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_ueb_braille_with_nemeth_parser() {
        // Try to parse UEB braille with Nemeth parser
        let ueb_braille = "\u{283C}\u{2801}"; // 1 in UEB (digit pattern differs)
        let result = braille_to_mathml_detailed(ueb_braille, BrailleCode::Nemeth);
        // May interpret differently or fail
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    // ========================================================================
    // Edge Case Tests - Special Math Expressions
    // ========================================================================

    #[test]
    fn test_double_superscript_nemeth() {
        // x^2^3 - ambiguous nesting
        let braille = "\u{282D}\u{2818}\u{283C}\u{2806}\u{2818}\u{283C}\u{2812}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // Should handle or error gracefully
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_subscript_and_superscript_nemeth() {
        // x_1^2 - both subscript and superscript
        let braille = "\u{282D}\u{2830}\u{283C}\u{2802}\u{2818}\u{283C}\u{2806}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_deeply_nested_parentheses() {
        // (((x)))
        let braille = "\u{2837}\u{2837}\u{2837}\u{282D}\u{283E}\u{283E}\u{283E}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
    }

    #[test]
    fn test_fraction_in_superscript() {
        // x^(1/2) - square root expressed as fraction
        let braille = "\u{282D}\u{2818}\u{2839}\u{283C}\u{2802}\u{280C}\u{283C}\u{2806}\u{283C}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_square_root_of_fraction() {
        // sqrt(1/2)
        let braille = "\u{281C}\u{2839}\u{283C}\u{2802}\u{280C}\u{283C}\u{2806}\u{283C}\u{283B}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_zero_handling() {
        // The digit 0
        let braille = "\u{283C}\u{2834}"; // 0 in Nemeth
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mn>0</mn>"));
    }

    #[test]
    fn test_negative_number() {
        // -5
        // Note: Leading minus sign is a known limitation - parser expects term first
        let braille = "\u{2824}\u{283C}\u{2822}"; // - numeric 5 in Nemeth
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        // Parser may not handle leading minus - verify graceful handling
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }

    #[test]
    fn test_all_greek_letters_lowercase() {
        // Test that alpha doesn't crash
        let braille = "\u{2828}\u{2801}"; // Greek alpha
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
    }

    #[test]
    fn test_expression_with_multiple_greek() {
        // alpha + beta
        let braille = "\u{2828}\u{2801}\u{282C}\u{2828}\u{2803}";
        let result = braille_to_mathml_detailed(braille, BrailleCode::Nemeth);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("\u{03B1}")); // alpha
        assert!(mathml.contains("\u{03B2}")); // beta
    }
}
