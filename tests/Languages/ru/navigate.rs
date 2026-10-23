//! Russian navigation speech for prefix/silent intents without NavigationParts.

use crate::common::*;
use anyhow::Result;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn overview_modes(expr: &str, expected: [&str; 3]) -> Result<()> {
    for style in ["SimpleSpeak", "ClearSpeak"] {
        for (verbosity, answer) in ["Terse", "Medium", "Verbose"].into_iter().zip(expected) {
            init_nav("ru", expr)?;
            set_preference("SpeechStyle", style)?;
            set_preference("Verbosity", verbosity)?;
            let overview = get_overview_text()?.split_whitespace().collect::<Vec<_>>().join(" ");
            assert_eq!(overview, answer, "{style}/{verbosity}");
        }
    }
    Ok(())
}

#[test]
fn overview_labeled_table() -> Result<()> {
    // A row label is not a data column; both rows have two data cells.
    let expr = "<math><mtable><mlabeledtr><mtd><mtext>label</mtext></mtd><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mlabeledtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable></math>";
    overview_modes(expr, ["2 на 2 таблица"; 3])
}

#[test]
fn overview_numeric_roots() -> Result<()> {
    // Explicit square, cube, and general degree all have a valid Russian overview.
    overview_modes("<math><mroot><mi>x</mi><mn>2</mn></mroot></math>",
                   ["квадратный корень икс", "квадратный корень из икс", "квадратный корень из икс"])?;
    overview_modes("<math><mroot><mi>x</mi><mn>3</mn></mroot></math>",
                   ["кубический корень икс", "кубический корень из икс", "кубический корень из икс"])?;
    overview_modes("<math><mroot><mi>x</mi><mn>4</mn></mroot></math>",
                   ["корень степени 4 икс", "корень степени 4 из икс", "корень степени 4 из икс"])
}

#[test]
fn overview_fraction_and_long_row() -> Result<()> {
    // Check both fraction branches and the five-item cutoff used to shorten long rows.
    overview_modes("<math><mfrac><mn>2</mn><mn>3</mn></mfrac></math>", ["2 разделить на 3"; 3])?;
    overview_modes("<math><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mi>y</mi></mfrac></math>",
                   ["дробь, числитель: икс плюс 1 знаменатель: игрек"; 3])?;
    overview_modes("<math><mi>a</mi><mo>+</mo><mi>b</mi><mo>+</mo><mi>c</mi><mo>+</mo><mi>d</mi></math>",
                   ["а плюс бэ плюс цэ и так далее"; 3])
}


#[test]
fn overview_table_dimensions() -> Result<()> {
    // Dimensions contain one separator, with no translated English article before them.
    let expr = "<math><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr></mtable></math>";
    for verbosity in ["Terse", "Medium", "Verbose"] {
        init_nav("ru", expr)?;
        set_preference("Verbosity", verbosity)?;
        assert_eq!(get_overview_text()?, "1 на 2 таблица");
    }
    Ok(())
}

#[test]
fn overview_symbolic_root() -> Result<()> {
    // A symbolic degree must be read as a degree, without an English TTS ordinal suffix.
    let expr = "<math><mroot><mi>x</mi><mi>n</mi></mroot></math>";
    for verbosity in ["Terse", "Medium", "Verbose"] {
        init_nav("ru", expr)?;
        set_preference("Verbosity", verbosity)?;
        let expected = if verbosity == "Terse" { "корень степени эн икс" } else { "корень степени эн из икс" };
        assert_eq!(get_overview_text()?, expected);
    }
    Ok(())
}

#[test]
fn overview_matrix_dimensions() -> Result<()> {
    // Bracketed matrices use the matrix overview at every verbosity.
    let expr = "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr></mtable><mo>)</mo></mrow></math>";
    for verbosity in ["Terse", "Medium", "Verbose"] {
        init_nav("ru", expr)?;
        set_preference("Verbosity", verbosity)?;
        assert_eq!(get_overview_text()?, "1 на 2 матрица");
    }
    Ok(())
}

#[test]
fn overview_determinant_dimensions() -> Result<()> {
    // Vertical bars select the determinant overview, without an initial "на".
    let expr = "<math><mrow><mo>|</mo><mtable><mtr><mtd><mn>1</mn></mtd></mtr></mtable><mo>|</mo></mrow></math>";
    for verbosity in ["Terse", "Medium", "Verbose"] {
        init_nav("ru", expr)?;
        set_preference("Verbosity", verbosity)?;
        assert_eq!(get_overview_text()?, "1 на 1 определитель");
    }
    Ok(())
}

fn assert_zoom_in(mathml: &str, expected: &str) -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_nav("ru", mathml)?;
        let speech = do_navigate_command("ZoomIn")?;
        let trimmed_speech = speech.trim_end_matches([' ', ',', ';']).to_string();
        assert_eq!(expected, trimmed_speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn no_parts_prefix_vector_suppresses_base_announcement() -> Result<()> {
    let expr = r#"
      <math>
        <mover id="vec">
          <mi id="v">v</mi>
          <mo>→</mo>
        </mover>
      </math>
    "#;
    assert_zoom_in(expr, "переход внутрь; вэ")
}

#[test]
fn no_parts_silent_modified_variable_suppresses_base_announcement() -> Result<()> {
    let expr = r#"
      <math>
        <mover id="hat">
          <mi id="x">x</mi>
          <mo>^</mo>
        </mover>
      </math>
    "#;
    assert_zoom_in(expr, "переход внутрь; икс")
}

#[test]
fn move_cell_next_from_row_reports_column_boundary() -> Result<()> {
    // MoveCellNext acts on columns; the boundary message must not mention rows or moving up.
    let expr = "<math><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr></mtable></math>";
    init_nav("ru", expr)?;
    set_preference("NavVerbosity", "Verbose")?;
    do_navigate_command("ZoomIn")?;
    let speech = do_navigate_command("MoveCellNext")?;
    assert_eq!(speech.trim_end_matches([' ', ',', ';']),
        "перейти к следующему столбцу; нет следующего столбца");
    Ok(())
}
