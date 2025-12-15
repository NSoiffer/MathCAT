//! Code Switching Module
//!
//! This module handles detection and switching between different braille codes
//! within the same document, particularly UEB/Nemeth switching as per BANA guidelines.
//!
//! Phase 4: Code Switching & Spatial Layout

use crate::back_translate::errors::{BackTranslationError, BackTranslationWarning, ParseResult};
use crate::back_translate::{nemeth, ueb, BrailleCode};

/// Mode indicators for UEB/Nemeth switching
/// Per BANA guidelines, Nemeth code begins with opening indicator
/// and ends with closing indicator
pub mod indicators {
    /// Nemeth code opening indicator in UEB context
    /// Dots 456, 146
    pub const NEMETH_OPEN: &str = "\u{2838}\u{2829}";

    /// Nemeth code closing indicator (return to UEB)
    /// Dots 456, 156
    pub const NEMETH_CLOSE: &str = "\u{2838}\u{2831}";

    /// UEB Grade 1 symbol indicator
    /// Dots 56
    pub const UEB_GRADE1_SYMBOL: &str = "\u{2830}";

    /// UEB Grade 1 word indicator
    /// Dots 56, 56
    pub const UEB_GRADE1_WORD: &str = "\u{2830}\u{2830}";

    /// UEB Grade 1 passage indicator
    /// Dots 56, 56, 56
    pub const UEB_GRADE1_PASSAGE: &str = "\u{2830}\u{2830}\u{2830}";

    /// UEB Grade 1 terminator
    /// Dots 56, 3
    pub const UEB_GRADE1_TERMINATOR: &str = "\u{2830}\u{2804}";
}

/// Represents a segment of braille with its detected code
#[derive(Debug, Clone)]
pub struct BrailleSegment {
    /// The braille content
    pub content: String,
    /// The detected braille code for this segment
    pub code: BrailleCode,
    /// Starting position in original input
    pub start: usize,
    /// Ending position in original input
    pub end: usize,
}

/// Result of code detection
#[derive(Debug, Clone)]
pub struct CodeDetectionResult {
    /// Detected primary code
    pub primary_code: BrailleCode,
    /// List of segments with potentially different codes
    pub segments: Vec<BrailleSegment>,
    /// Whether code switching was detected
    pub has_code_switching: bool,
}

/// Detect the braille code(s) used in input
///
/// This function analyzes the input to determine:
/// 1. The primary braille code used
/// 2. Whether code switching occurs (e.g., Nemeth within UEB)
/// 3. The segments with their respective codes
pub fn detect_code(braille: &str) -> CodeDetectionResult {
    let mut segments = Vec::new();
    let mut has_code_switching = false;
    let mut current_code = detect_initial_code(braille);
    let primary_code = current_code;

    // Scan for code switching indicators
    let chars: Vec<char> = braille.chars().collect();
    let mut i = 0;
    let mut segment_start = 0;

    while i < chars.len() {
        // Check for Nemeth opening indicator (in UEB context)
        if current_code == BrailleCode::UEB && check_indicator(&chars, i, indicators::NEMETH_OPEN) {
            // Save UEB segment before switch
            if i > segment_start {
                segments.push(BrailleSegment {
                    content: chars[segment_start..i].iter().collect(),
                    code: BrailleCode::UEB,
                    start: segment_start,
                    end: i,
                });
            }

            has_code_switching = true;
            current_code = BrailleCode::Nemeth;
            i += indicators::NEMETH_OPEN.chars().count();
            segment_start = i;
            continue;
        }

        // Check for Nemeth closing indicator (return to UEB)
        if current_code == BrailleCode::Nemeth && check_indicator(&chars, i, indicators::NEMETH_CLOSE) {
            // Save Nemeth segment
            if i > segment_start {
                segments.push(BrailleSegment {
                    content: chars[segment_start..i].iter().collect(),
                    code: BrailleCode::Nemeth,
                    start: segment_start,
                    end: i,
                });
            }

            current_code = BrailleCode::UEB;
            i += indicators::NEMETH_CLOSE.chars().count();
            segment_start = i;
            continue;
        }

        i += 1;
    }

    // Add final segment
    if segment_start < chars.len() {
        segments.push(BrailleSegment {
            content: chars[segment_start..].iter().collect(),
            code: current_code,
            start: segment_start,
            end: chars.len(),
        });
    }

    // If no code switching, return single segment
    if segments.is_empty() {
        segments.push(BrailleSegment {
            content: braille.to_string(),
            code: primary_code,
            start: 0,
            end: braille.len(),
        });
    }

    CodeDetectionResult {
        primary_code,
        segments,
        has_code_switching,
    }
}

/// Detect the initial/primary braille code based on patterns
fn detect_initial_code(braille: &str) -> BrailleCode {
    // Check for explicit Nemeth indicators at start
    if braille.starts_with(indicators::NEMETH_OPEN) {
        return BrailleCode::UEB; // Document is UEB with Nemeth switch
    }

    // Heuristic detection based on common patterns
    let chars: Vec<char> = braille.chars().collect();

    // Nemeth numeric indicator is dots 3456 (U+283C)
    // In Nemeth, numbers look different than in UEB
    // UEB numeric indicator is also dots 3456, but digit patterns differ

    // Check for Nemeth-specific digit patterns after numeric indicator
    for i in 0..chars.len().saturating_sub(1) {
        if chars[i] == '\u{283C}' {
            let next = chars.get(i + 1);
            if let Some(&ch) = next {
                // Nemeth digits: 1=dots 2, 2=dots 23, 3=dots 25, etc.
                // UEB digits: 1=dots 1, 2=dots 12, 3=dots 14, etc.
                // Check if next char is a Nemeth digit pattern
                if matches!(ch, '\u{2802}' | '\u{2806}' | '\u{2812}' | '\u{2832}' |
                               '\u{2822}' | '\u{2816}' | '\u{2836}' | '\u{2826}' |
                               '\u{2814}' | '\u{2834}') {
                    return BrailleCode::Nemeth;
                }
                // Check if it's a UEB digit pattern (same as letters a-j)
                if matches!(ch, '\u{2801}' | '\u{2803}' | '\u{2809}' | '\u{2819}' |
                               '\u{2811}' | '\u{280B}' | '\u{281B}' | '\u{2813}' |
                               '\u{280A}' | '\u{281A}') {
                    return BrailleCode::UEB;
                }
            }
        }
    }

    // Check for Nemeth-specific operators
    // Nemeth plus: dots 346 (U+282C)
    // UEB plus: dots 5, 2346 (U+2810, U+282E)
    for i in 0..chars.len().saturating_sub(1) {
        // Single-cell plus is more likely Nemeth
        if chars[i] == '\u{282C}' {
            return BrailleCode::Nemeth;
        }
        // Two-cell plus pattern is UEB
        if chars[i] == '\u{2810}' && chars.get(i + 1) == Some(&'\u{282E}') {
            return BrailleCode::UEB;
        }
    }

    // Default to Nemeth for math expressions (most common in US)
    BrailleCode::Nemeth
}

/// Check if an indicator exists at the given position
fn check_indicator(chars: &[char], pos: usize, indicator: &str) -> bool {
    let indicator_chars: Vec<char> = indicator.chars().collect();
    if pos + indicator_chars.len() > chars.len() {
        return false;
    }
    for (j, &ind_char) in indicator_chars.iter().enumerate() {
        if chars[pos + j] != ind_char {
            return false;
        }
    }
    true
}

/// Parse braille with automatic code detection and switching
pub fn parse_with_code_detection(braille: &str) -> ParseResult {
    if braille.trim().is_empty() {
        return ParseResult::failure(BackTranslationError::EmptyInput);
    }

    let detection = detect_code(braille);

    if !detection.has_code_switching {
        // Simple case: single code throughout
        match detection.primary_code {
            BrailleCode::Nemeth => nemeth::parse_nemeth(braille),
            BrailleCode::UEB => ueb::parse_ueb(braille),
            BrailleCode::CMU => crate::back_translate::cmu::parse_cmu(braille),
        }
    } else {
        // Complex case: multiple codes
        parse_with_switching(&detection)
    }
}

/// Parse document with code switching
fn parse_with_switching(detection: &CodeDetectionResult) -> ParseResult {
    let mut all_warnings: Vec<BackTranslationWarning> = Vec::new();
    let mut all_errors: Vec<BackTranslationError> = Vec::new();

    for segment in &detection.segments {
        if segment.content.trim().is_empty() {
            continue;
        }

        let result = match segment.code {
            BrailleCode::Nemeth => nemeth::parse_nemeth(&segment.content),
            BrailleCode::UEB => ueb::parse_ueb(&segment.content),
            BrailleCode::CMU => crate::back_translate::cmu::parse_cmu(&segment.content),
        };

        if !result.errors.is_empty() {
            all_errors.extend(result.errors);
        }
        all_warnings.extend(result.warnings);

        // Extract the node from the result if successful
        // For now, we just collect results - proper merging would need AST access
        if let Some(_mathml) = &result.mathml {
            // Add a warning that code switching occurred
            all_warnings.push(BackTranslationWarning::UnexpectedIndicator {
                indicator: format!("Code switch to {}", segment.code),
                position: segment.start,
            });
        }
    }

    // For now, concatenate by parsing the entire content with the primary code
    // A more sophisticated approach would merge the ASTs
    let primary_result = match detection.primary_code {
        BrailleCode::Nemeth => {
            // Strip the code switching indicators and parse
            let cleaned = strip_code_switch_indicators(&detection.segments.iter()
                .map(|s| s.content.as_str())
                .collect::<Vec<_>>()
                .join(""));
            nemeth::parse_nemeth(&cleaned)
        }
        BrailleCode::UEB => {
            let cleaned = strip_code_switch_indicators(&detection.segments.iter()
                .map(|s| s.content.as_str())
                .collect::<Vec<_>>()
                .join(""));
            ueb::parse_ueb(&cleaned)
        }
        BrailleCode::CMU => {
            let cleaned = strip_code_switch_indicators(&detection.segments.iter()
                .map(|s| s.content.as_str())
                .collect::<Vec<_>>()
                .join(""));
            crate::back_translate::cmu::parse_cmu(&cleaned)
        }
    };

    ParseResult {
        mathml: primary_result.mathml,
        errors: if all_errors.is_empty() { primary_result.errors } else { all_errors },
        warnings: all_warnings,
    }
}

/// Strip code switching indicators from braille string
fn strip_code_switch_indicators(braille: &str) -> String {
    braille
        .replace(indicators::NEMETH_OPEN, "")
        .replace(indicators::NEMETH_CLOSE, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_nemeth_code() {
        // Nemeth number: numeric indicator + Nemeth digits
        let braille = "\u{283C}\u{2802}\u{2806}\u{2812}"; // 123 in Nemeth
        let result = detect_code(braille);
        assert_eq!(result.primary_code, BrailleCode::Nemeth);
        assert!(!result.has_code_switching);
    }

    #[test]
    fn test_detect_ueb_code() {
        // UEB number: numeric indicator + UEB digits (same as letters)
        let braille = "\u{283C}\u{2801}\u{2803}\u{2809}"; // 123 in UEB
        let result = detect_code(braille);
        assert_eq!(result.primary_code, BrailleCode::UEB);
        assert!(!result.has_code_switching);
    }

    #[test]
    fn test_detect_code_switching() {
        // UEB text with Nemeth switch
        let braille = format!(
            "{}{}{}",
            indicators::NEMETH_OPEN,
            "\u{283C}\u{2802}", // Nemeth number
            indicators::NEMETH_CLOSE
        );
        let result = detect_code(&braille);
        assert!(result.has_code_switching);
        assert!(result.segments.len() >= 1);
    }

    #[test]
    fn test_strip_indicators() {
        let braille = format!(
            "{}content{}",
            indicators::NEMETH_OPEN,
            indicators::NEMETH_CLOSE
        );
        let stripped = strip_code_switch_indicators(&braille);
        assert_eq!(stripped, "content");
    }

    #[test]
    fn test_parse_with_detection_nemeth() {
        // Simple Nemeth expression
        let braille = "\u{283C}\u{2802}"; // 1 in Nemeth
        let result = parse_with_code_detection(braille);
        assert!(result.is_success(), "Parse failed: {:?}", result.errors);
        let mathml = result.mathml.unwrap();
        assert!(mathml.contains("<mn>1</mn>"));
    }

    #[test]
    fn test_parse_empty_input() {
        let result = parse_with_code_detection("");
        assert!(!result.is_success());
        assert!(matches!(
            result.errors.first(),
            Some(BackTranslationError::EmptyInput)
        ));
    }

    #[test]
    fn test_nemeth_operator_detection() {
        // Nemeth x + y (single-cell plus)
        let braille = "\u{282D}\u{282C}\u{283D}";
        let result = detect_code(braille);
        assert_eq!(result.primary_code, BrailleCode::Nemeth);
    }
}
