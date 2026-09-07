//! Navigation ZoomIn speech vs NavigationParts and intent fixity.
//!
//! Uses only real intents / MathML from `definitions.yaml` and `Rules/Intent/*.yaml`
//! (same shapes as other en language tests). Fixities covered are those that actually
//! appear there: prefix, infix, postfix, function, silent, nofix.

use crate::common::*;
use anyhow::Result;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn init_nav(mathml: &str) -> Result<()> {
    set_rules_dir(abs_rules_dir_path())?;
    set_preference("Language", "hu")?;
    set_preference("SpeechStyle", "SimpleSpeak")?;
    set_preference("Verbosity", "Medium")?;
    set_preference("NavMode", "Enhanced")?;
    set_preference("NavVerbosity", "Verbose")?;
    set_preference("AutoZoomOut", "False")?;
    set_preference("Overview", "False")?;
    set_mathml(mathml)?;
    Ok(())
}

fn assert_zoom_in(command: &str, mathml: &str, expected: &str) -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_nav(mathml)?;
        let speech = do_navigate_command(command)?;
        let trimmed_speech = speech.trim_end_matches([' ', ',', ';']).to_string();
        assert_eq!(expected, trimmed_speech);
        Ok(())
    }));
    report_any_panic(result)
}

// --- Intents in NavigationParts (prefix, infix, function, silent; no postfix/nofix) ---

#[test]
fn parts_prefix_logarithm_with_base() -> Result<()> {
    // Intent/general.yaml log-with-base → logarithm-with-base:prefix; parts "base"
    let expr = r#"
      <math>
        <msub id="log">
          <mi>log</mi>
          <mi id="b">b</mi>
        </msub>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; in alap; b")
}

#[test]
fn parts_infix_power() -> Result<()> {
    // power:infix; parts "base; exponent"
    let expr = r#"
      <math>
        <msup id="pow">
          <mi id="alap">x</mi>
          <mn id="kitevő">2</mn>
        </msup>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; in alap; x")
}

#[test]
fn parts_infix_indexed_by() -> Result<()> {
    // indexed-by:infix; parts "base; subscript"
    let expr = r#"
      <math>
        <msub id="sub">
          <mi id="alap">x</mi>
          <mn id="i">1</mn>
        </msub>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; in alap; x")
}

#[test]
fn parts_function_fraction() -> Result<()> {
    // fraction (from mfrac); parts "numerator; denominator"
    let expr = r#"
      <math>
        <mfrac id="frac">
          <mn id="számláló">1</mn>
          <mn id="nevező">2</mn>
        </mfrac>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; in számláló; 1")
}

#[test]
fn parts_function_square_root() -> Result<()> {
    // square-root:function; parts "root"
    let expr = r#"
      <math>
        <msqrt id="root">
          <mi id="x">x</mi>
        </msqrt>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; in gyök; x")
}

#[test]
fn parts_silent_skip_super() -> Result<()> {
    // skip-super:silent (degree); parts "base; superscript"
    let expr = r#"
      <math>
        <msup id="deg">
          <mi id="alap">x</mi>
          <mo id="deg-mark">°</mo>
        </msup>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; in alap; x")
}

// --- Intents not in NavigationParts ---

#[test]
fn no_parts_prefix_unary_minus() -> Result<()> {
    // minus:prefix (Intent/general.yaml positive-or-negative) — silent "in"
    let expr = r#"
      <math>
        <mrow id="neg">
          <mo>-</mo>
          <mi id="b">b</mi>
        </mrow>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; b")
}

#[test]
fn no_parts_prefix_limit() -> Result<()> {
    // limit:prefix — silent "in"
    let expr = r#"
      <math>
        <mrow intent="limit:prefix($x)" id="lim">
          <mi arg="x" id="x">x</mi>
        </mrow>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; x")
}

#[test]
fn no_parts_prefix_vector() -> Result<()> {
    // vector:prefix from mover + arrow
    let expr = r#"
      <math>
        <mover id="vec">
          <mi id="v">v</mi>
          <mo>→</mo>
        </mover>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; v")
}

#[test]
fn no_parts_infix_binomial() -> Result<()> {
    // binomial:infix (choose); no NavigationParts → "in part 1"
    let expr = r#"
      <math>
        <mrow id="bin">
          <mo>(</mo>
          <mfrac linethickness="0" id="choose">
            <mn id="n">7</mn>
            <mn id="k">3</mn>
          </mfrac>
          <mo>)</mo>
        </mrow>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; in része 1; 7")
}

#[test]
fn no_parts_postfix_transpose() -> Result<()> {
    // transpose:postfix — not prefix, so still announces "in …"
    let expr = r#"
      <math>
        <msup id="tr">
          <mi id="m">M</mi>
          <mi>T</mi>
        </msup>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; in transzponált; nagy m")
}

#[test]
fn no_parts_function_absolute_value() -> Result<()> {
    // absolute-value:function
    let expr = r#"
      <math>
        <mrow id="abs">
          <mo>|</mo>
          <mi id="x">x</mi>
          <mo>|</mo>
        </mrow>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; in az abszolút érték; x")
}

#[test]
fn no_parts_silent_modified_variable() -> Result<()> {
    // modified-variable:silent (x-hat) — no NavigationParts → silent "in"
    let expr = r#"
      <math>
        <mover id="hat">
          <mi id="x">x</mi>
          <mo>^</mo>
        </mover>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; x")
}

#[test]
fn no_parts_nofix_set_of_reals() -> Result<()> {
    // set-of-reals:nofix — leaf
    let expr = r#"
      <math>
        <mi intent="set-of-reals:nofix" id="r">R</mi>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "teljesen ráközelített; az összes valós szám halmaza")
}

// --- No intent= attribute (native MathML only; may still be inferred) ---

#[test]
fn no_intent_sum_mrow() -> Result<()> {
    let expr = r#"
      <math>
        <mrow id="sum">
          <mi id="x">x</mi>
          <mo>+</mo>
          <mi id="y">y</mi>
        </mrow>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; x")
}

#[test]
fn no_intent_times_mrow() -> Result<()> {
    let expr = r#"
      <math>
        <mrow id="prod">
          <mn id="two">2</mn>
          <mo>&#x2062;</mo>
          <mi id="a">a</mi>
        </mrow>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; 2")
}

#[test]
fn no_intent_mi() -> Result<()> {
    let expr = r#"
      <math>
        <mi id="x">x</mi>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "teljesen ráközelített; x")
}
