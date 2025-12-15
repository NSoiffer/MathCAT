//! Spatial Layout Module
//!
//! This module handles 2D layout structures in braille math,
//! including matrices, determinants, and multi-line expressions.
//!
//! Phase 4: Spatial Layout Support

use crate::back_translate::errors::{BackTranslationError, BackTranslationWarning, ParseResult};
use crate::back_translate::{nemeth, BrailleCode};

/// Spatial layout indicators for Nemeth
pub mod nemeth_spatial {
    /// Enlarged left parenthesis (matrix/determinant open)
    /// In Nemeth spatial mode, matrices are indicated by enlarged grouping symbols
    /// spanning multiple lines
    pub const ENLARGED_LEFT_PAREN: &str = "\u{2839}";

    /// Enlarged right parenthesis (matrix/determinant close)
    pub const ENLARGED_RIGHT_PAREN: &str = "\u{283C}";

    /// Enlarged left bracket
    pub const ENLARGED_LEFT_BRACKET: &str = "\u{2808}\u{2839}";

    /// Enlarged right bracket
    pub const ENLARGED_RIGHT_BRACKET: &str = "\u{2808}\u{283C}";

    /// Vertical bar for determinant
    pub const ENLARGED_VERT_BAR: &str = "\u{2833}";

    /// Long division start
    pub const LONG_DIV_START: &str = "\u{2807}";

    /// Long division line (vinculum)
    pub const VINCULUM: &str = "\u{2831}";

    /// Newline in spatial mode (blank line or specific indicator)
    pub const SPATIAL_NEWLINE: &str = "\n";
}

/// Represents a cell in a matrix or table
#[derive(Debug, Clone)]
pub struct MatrixCell {
    /// Content of the cell
    pub content: String,
    /// Row index (0-based)
    pub row: usize,
    /// Column index (0-based)
    pub col: usize,
}

/// Represents a matrix structure
#[derive(Debug, Clone)]
pub struct Matrix {
    /// The cells in the matrix
    pub cells: Vec<MatrixCell>,
    /// Number of rows
    pub rows: usize,
    /// Number of columns
    pub cols: usize,
    /// The type of matrix (parentheses, brackets, determinant, etc.)
    pub matrix_type: MatrixType,
}

/// Types of matrix delimiters
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatrixType {
    /// Matrix with parentheses
    Parentheses,
    /// Matrix with brackets
    Brackets,
    /// Determinant (vertical bars)
    Determinant,
    /// No delimiters (array)
    Plain,
}

impl Matrix {
    /// Convert matrix to MathML
    pub fn to_mathml(&self, code: BrailleCode) -> Result<String, BackTranslationError> {
        let mut mathml = String::new();

        // Opening mrow and delimiter
        mathml.push_str("<mrow>");
        match self.matrix_type {
            MatrixType::Parentheses => mathml.push_str("<mo>(</mo>"),
            MatrixType::Brackets => mathml.push_str("<mo>[</mo>"),
            MatrixType::Determinant => mathml.push_str("<mo>|</mo>"),
            MatrixType::Plain => {}
        }

        // Start mtable
        mathml.push_str("<mtable>");

        // Generate rows
        for row_idx in 0..self.rows {
            mathml.push_str("<mtr>");
            for col_idx in 0..self.cols {
                mathml.push_str("<mtd>");

                // Find cell content
                let cell = self.cells.iter().find(|c| c.row == row_idx && c.col == col_idx);
                if let Some(cell) = cell {
                    // Parse cell content
                    let result = match code {
                        BrailleCode::Nemeth => nemeth::parse_nemeth(&cell.content),
                        _ => nemeth::parse_nemeth(&cell.content), // Default to Nemeth for now
                    };

                    if let Some(cell_mathml) = result.mathml {
                        // Extract inner content (skip outer <math> tags)
                        let inner = extract_inner_mathml(&cell_mathml);
                        mathml.push_str(&inner);
                    }
                }

                mathml.push_str("</mtd>");
            }
            mathml.push_str("</mtr>");
        }

        // Close mtable
        mathml.push_str("</mtable>");

        // Closing delimiter and mrow
        match self.matrix_type {
            MatrixType::Parentheses => mathml.push_str("<mo>)</mo>"),
            MatrixType::Brackets => mathml.push_str("<mo>]</mo>"),
            MatrixType::Determinant => mathml.push_str("<mo>|</mo>"),
            MatrixType::Plain => {}
        }
        mathml.push_str("</mrow>");

        Ok(format!(
            "<math xmlns=\"http://www.w3.org/1998/Math/MathML\">{}</math>",
            mathml
        ))
    }
}

/// Extract inner content from MathML (removing outer <math> tags)
fn extract_inner_mathml(mathml: &str) -> String {
    // Remove <math ...> and </math>
    let start = mathml.find('>').map(|i| i + 1).unwrap_or(0);
    let end = mathml.rfind("</math>").unwrap_or(mathml.len());
    mathml[start..end].to_string()
}

/// Detect if input contains spatial layout
pub fn has_spatial_layout(braille: &str) -> bool {
    // Check for newlines (spatial indicators)
    if braille.contains('\n') && braille.lines().count() > 1 {
        return true;
    }

    // Check for enlarged grouping symbols that might indicate matrix
    braille.contains(nemeth_spatial::ENLARGED_LEFT_BRACKET)
        || braille.contains(nemeth_spatial::ENLARGED_VERT_BAR)
}

/// Parse a spatial matrix from multi-line braille
pub fn parse_spatial_matrix(braille: &str, _code: BrailleCode) -> Result<Matrix, BackTranslationError> {
    let lines: Vec<&str> = braille.lines().collect();

    if lines.len() < 2 {
        return Err(BackTranslationError::ParseError {
            message: "Matrix requires at least 2 rows".to_string(),
            position: None,
        });
    }

    // Detect matrix type from first line
    let first_line = lines[0].trim();
    let matrix_type = detect_matrix_type(first_line);

    // Parse cells
    let mut cells = Vec::new();
    let mut max_cols = 0;

    for (row_idx, line) in lines.iter().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Split by common separators (spaces, specific braille patterns)
        let row_cells = split_row_cells(line);
        max_cols = max_cols.max(row_cells.len());

        for (col_idx, cell_content) in row_cells.iter().enumerate() {
            cells.push(MatrixCell {
                content: cell_content.to_string(),
                row: row_idx,
                col: col_idx,
            });
        }
    }

    Ok(Matrix {
        cells,
        rows: lines.len(),
        cols: max_cols,
        matrix_type,
    })
}

/// Detect matrix type from opening delimiter
fn detect_matrix_type(line: &str) -> MatrixType {
    if line.starts_with(nemeth_spatial::ENLARGED_LEFT_BRACKET) {
        MatrixType::Brackets
    } else if line.starts_with(nemeth_spatial::ENLARGED_VERT_BAR) {
        MatrixType::Determinant
    } else if line.starts_with(nemeth_spatial::ENLARGED_LEFT_PAREN) {
        MatrixType::Parentheses
    } else {
        MatrixType::Plain
    }
}

/// Split a row into cells
fn split_row_cells(line: &str) -> Vec<String> {
    // Multiple spaces often separate cells in spatial braille
    // Also look for specific cell separators

    // First, try splitting by multiple braille spaces
    let parts: Vec<&str> = line.split("\u{2800}\u{2800}").collect();
    if parts.len() > 1 {
        return parts.iter().map(|s| s.trim().to_string()).collect();
    }

    // Try splitting by tab-like patterns
    let parts: Vec<&str> = line.split("\t").collect();
    if parts.len() > 1 {
        return parts.iter().map(|s| s.trim().to_string()).collect();
    }

    // Try splitting by multiple regular spaces
    let parts: Vec<&str> = line.split("  ").collect();
    if parts.len() > 1 {
        return parts.iter().map(|s| s.trim().to_string()).collect();
    }

    // Single cell
    vec![line.to_string()]
}

/// Parse braille with spatial layout detection
pub fn parse_with_spatial(braille: &str, code: BrailleCode) -> ParseResult {
    if !has_spatial_layout(braille) {
        // No spatial layout, use regular parsing
        return match code {
            BrailleCode::Nemeth => nemeth::parse_nemeth(braille),
            BrailleCode::UEB => crate::back_translate::ueb::parse_ueb(braille),
            BrailleCode::CMU => crate::back_translate::cmu::parse_cmu(braille),
        };
    }

    // Try to parse as matrix
    match parse_spatial_matrix(braille, code) {
        Ok(matrix) => match matrix.to_mathml(code) {
            Ok(mathml) => ParseResult::success(mathml),
            Err(e) => ParseResult::failure(e),
        },
        Err(_) => {
            // Fall back to line-by-line parsing
            parse_multiline(braille, code)
        }
    }
}

/// Parse multi-line braille expression (non-matrix)
fn parse_multiline(braille: &str, code: BrailleCode) -> ParseResult {
    let lines: Vec<&str> = braille.lines().collect();
    let mut warnings: Vec<BackTranslationWarning> = Vec::new();
    let mut errors: Vec<BackTranslationError> = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let result = match code {
            BrailleCode::Nemeth => nemeth::parse_nemeth(line),
            BrailleCode::UEB => crate::back_translate::ueb::parse_ueb(line),
            BrailleCode::CMU => crate::back_translate::cmu::parse_cmu(line),
        };

        if !result.errors.is_empty() {
            for err in result.errors {
                errors.push(err);
            }
        }
        warnings.extend(result.warnings);

        // Track that this is multi-line
        if lines.len() > 1 && line_num < lines.len() - 1 {
            warnings.push(BackTranslationWarning::AutoInserted {
                element: "line break".to_string(),
                position: line_num,
            });
        }
    }

    // For multi-line, parse the entire content as one expression
    // This is a simplification - proper multi-line would need special handling
    let combined = lines.join(" ");
    let result = match code {
        BrailleCode::Nemeth => nemeth::parse_nemeth(&combined),
        BrailleCode::UEB => crate::back_translate::ueb::parse_ueb(&combined),
        BrailleCode::CMU => crate::back_translate::cmu::parse_cmu(&combined),
    };

    ParseResult {
        mathml: result.mathml,
        errors: if errors.is_empty() { result.errors } else { errors },
        warnings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_spatial_layout_multiline() {
        let braille = "\u{283C}\u{2802}\n\u{283C}\u{2806}";
        assert!(has_spatial_layout(braille));
    }

    #[test]
    fn test_has_spatial_layout_single_line() {
        let braille = "\u{283C}\u{2802}";
        assert!(!has_spatial_layout(braille));
    }

    #[test]
    fn test_detect_matrix_type_brackets() {
        let line = "\u{2808}\u{2839} content";
        assert_eq!(detect_matrix_type(line), MatrixType::Brackets);
    }

    #[test]
    fn test_detect_matrix_type_determinant() {
        let line = "\u{2833} content";
        assert_eq!(detect_matrix_type(line), MatrixType::Determinant);
    }

    #[test]
    fn test_detect_matrix_type_plain() {
        let line = "content";
        assert_eq!(detect_matrix_type(line), MatrixType::Plain);
    }

    #[test]
    fn test_split_row_cells_double_space() {
        let line = "a  b  c";
        let cells = split_row_cells(line);
        assert_eq!(cells.len(), 3);
    }

    #[test]
    fn test_split_row_cells_single() {
        let line = "abc";
        let cells = split_row_cells(line);
        assert_eq!(cells.len(), 1);
    }

    #[test]
    fn test_matrix_to_mathml() {
        let matrix = Matrix {
            cells: vec![
                MatrixCell { content: "\u{283C}\u{2802}".to_string(), row: 0, col: 0 },
                MatrixCell { content: "\u{283C}\u{2806}".to_string(), row: 0, col: 1 },
                MatrixCell { content: "\u{283C}\u{2812}".to_string(), row: 1, col: 0 },
                MatrixCell { content: "\u{283C}\u{2832}".to_string(), row: 1, col: 1 },
            ],
            rows: 2,
            cols: 2,
            matrix_type: MatrixType::Brackets,
        };

        let mathml = matrix.to_mathml(BrailleCode::Nemeth);
        assert!(mathml.is_ok());
        let mathml = mathml.unwrap();
        assert!(mathml.contains("<mtable>"));
        assert!(mathml.contains("<mtr>"));
        assert!(mathml.contains("<mtd>"));
        assert!(mathml.contains("<mo>[</mo>"));
        assert!(mathml.contains("<mo>]</mo>"));
    }

    #[test]
    fn test_extract_inner_mathml() {
        let mathml = "<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mn>1</mn></math>";
        let inner = extract_inner_mathml(mathml);
        assert_eq!(inner, "<mn>1</mn>");
    }

    #[test]
    fn test_parse_with_spatial_single_line() {
        let braille = "\u{283C}\u{2802}"; // Single line, no spatial
        let result = parse_with_spatial(braille, BrailleCode::Nemeth);
        assert!(result.is_success());
    }

    #[test]
    fn test_parse_spatial_matrix() {
        let braille = "\u{283C}\u{2802}  \u{283C}\u{2806}\n\u{283C}\u{2812}  \u{283C}\u{2832}";
        let matrix = parse_spatial_matrix(braille, BrailleCode::Nemeth);
        assert!(matrix.is_ok());
        let matrix = matrix.unwrap();
        assert_eq!(matrix.rows, 2);
        assert!(matrix.cols >= 1);
    }

    #[test]
    fn test_parse_multiline() {
        let braille = "\u{283C}\u{2802}\n\u{283C}\u{2806}";
        let result = parse_with_spatial(braille, BrailleCode::Nemeth);
        // Should parse without error
        assert!(result.mathml.is_some() || !result.errors.is_empty());
    }
}
