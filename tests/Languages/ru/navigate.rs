//! Russian navigation speech for prefix/silent intents without NavigationParts.

use crate::common::*;
use anyhow::Result;
use std::panic::{catch_unwind, AssertUnwindSafe};

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
