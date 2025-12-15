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
        match c {
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
}
