//! Navigation ZoomIn speech vs NavigationParts and intent fixity.
//!
//! Uses only real intents / MathML from `definitions.yaml` and `Rules/Intent/*.yaml`
//! (same shapes as other en language tests). Fixities covered are those that actually
//! appear there: prefix, infix, postfix, function, silent, nofix.

use crate::common::*;
use anyhow::Result;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn init_nav(mathml: &str) -> Result<()> {
    init_nav_with_auto_zoom_out(mathml, false)
}

/// Same setup as `init_nav`, but lets a test choose the "AutoZoomOut" preference.
///
/// With AutoZoomOut == "False" the navigation rules stop at the edge of a notation and announce it
/// ("nem tud jobbra mozogni" + the part). With "True" they leave the notation automatically and announce
/// that with the "elhagyása" fragment ("nevező elhagyása").
fn init_nav_with_auto_zoom_out(mathml: &str, auto_zoom_out: bool) -> Result<()> {
    set_rules_dir(abs_rules_dir_path())?;
    set_preference("Language", "hu")?;
    set_preference("SpeechStyle", "SimpleSpeak")?;
    set_preference("Verbosity", "Medium")?;
    set_preference("NavMode", "Enhanced")?;
    set_preference("NavVerbosity", "Verbose")?;
    set_preference("AutoZoomOut", if auto_zoom_out { "True" } else { "False" })?;
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
    assert_zoom_in("ZoomIn", expr, "nagyítás; alap logaritmus; b")
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
    assert_zoom_in("ZoomIn", expr, "nagyítás; alap; x")
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
    assert_zoom_in("ZoomIn", expr, "nagyítás; alap; x")
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
    assert_zoom_in("ZoomIn", expr, "nagyítás; számláló; 1")
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
    assert_zoom_in("ZoomIn", expr, "nagyítás; gyök; x")
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
    assert_zoom_in("ZoomIn", expr, "nagyítás; alap; x")
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
    // binomial:infix (choose); no NavigationParts → "part 1"
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
    assert_zoom_in("ZoomIn", expr, "nagyítás; rész 1; 7")
}

#[test]
fn no_parts_postfix_transpose() -> Result<()> {
    // transpose:postfix — not prefix, so still announces its name
    let expr = r#"
      <math>
        <msup id="tr">
          <mi id="m">M</mi>
          <mi>T</mi>
        </msup>
      </math>
    "#;
    assert_zoom_in("ZoomIn", expr, "nagyítás; transzponált; nagy m")
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
    assert_zoom_in("ZoomIn", expr, "nagyítás; az abszolút érték; x")
}

#[test]
fn no_parts_silent_modified_variable() -> Result<()> {
    // modified-variable:silent (x-hat) — no NavigationParts → silent
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

// --- Move2D words (out of / start of / end of) and the zoom out / zoom in all commands ---
//
// Hungarian puts a case suffix on the part name ("a számlálóban"), which cannot be built for arbitrary
// part names. So "in" is silent and the other words follow the part name: "számláló elhagyása",
// "nevező vége", "számláló eleje". These tests check the fragments in order (not the exact separators
// or the speech of the node that is landed on) and that no English navigation word is left over.

fn nav_speech(commands: &[&str], mathml: &str, auto_zoom_out: bool) -> Result<String> {
    init_nav_with_auto_zoom_out(mathml, auto_zoom_out)?;
    let mut speech = String::new();
    for command in commands {
        speech = do_navigate_command(command)?;
    }
    Ok(speech)
}

/// Runs `commands` one after the other and checks the speech of the LAST one: every fragment must occur,
/// in the given order, and no English navigation word may be left over (AutoZoomOut is off).
fn assert_nav_fragments(commands: &[&str], mathml: &str, fragments: &[&str]) -> Result<()> {
    assert_nav_fragments_with(commands, mathml, false, fragments)
}

/// Like `assert_nav_fragments`, but with AutoZoomOut switched on.
fn assert_nav_fragments_auto_zoom_out(commands: &[&str], mathml: &str, fragments: &[&str]) -> Result<()> {
    assert_nav_fragments_with(commands, mathml, true, fragments)
}

fn assert_nav_fragments_with(commands: &[&str], mathml: &str, auto_zoom_out: bool, fragments: &[&str]) -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        let speech = nav_speech(commands, mathml, auto_zoom_out)?;
        let mut rest = speech.as_str();
        for fragment in fragments {
            match rest.find(fragment) {
                Some(pos) => rest = &rest[pos + fragment.len()..],
                None => panic!("'{}' not found (in this order) in speech '{}'", fragment, speech),
            }
        }
        let english = ["in", "out", "of", "start", "end", "zoom", "move", "cannot", "read", "describe", "current"];
        for word in speech.split(|c: char| !c.is_alphanumeric()) {
            assert!(!english.contains(&word), "English word '{}' left in speech '{}'", word, speech);
        }
        Ok(())
    }));
    report_any_panic(result)
}

/// Runs `commands` and compares the speech of the LAST one exactly (trailing separators are ignored).
fn assert_nav_exact(commands: &[&str], mathml: &str, auto_zoom_out: bool, expected: &str) -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        let speech = nav_speech(commands, mathml, auto_zoom_out)?;
        assert_eq!(expected, speech.trim_end_matches([' ', ',', ';']));
        Ok(())
    }));
    report_any_panic(result)
}

const FRACTION: &str = r#"
      <math>
        <mfrac id="frac">
          <mn id="számláló">1</mn>
          <mn id="nevező">2</mn>
        </mfrac>
      </math>
    "#;

const POWER: &str = r#"
      <math>
        <msup id="pow">
          <mi id="alap">x</mi>
          <mn id="kitevő">2</mn>
        </msup>
      </math>
    "#;

/// 1/2 + y: the fraction has material to its right, so it is not at the edge of the math.
const FRACTION_PLUS_Y: &str = r#"
      <math>
        <mrow>
          <mfrac id="frac">
            <mn id="számláló">1</mn>
            <mn id="nevező">2</mn>
          </mfrac>
          <mo id="plusz">+</mo>
          <mi id="y">y</mi>
        </mrow>
      </math>
    "#;

/// y + 1/2: the fraction has material to its left.
const Y_PLUS_FRACTION: &str = r#"
      <math>
        <mrow>
          <mi id="y">y</mi>
          <mo id="plusz">+</mo>
          <mfrac id="frac">
            <mn id="számláló">1</mn>
            <mn id="nevező">2</mn>
          </mfrac>
        </mrow>
      </math>
    "#;

/// Native mmultiscripts: base x, postscripts (subscript 1, superscript 2), prescripts (subscript 3, superscript 4).
/// Children are: x, 1, 2, <mprescripts/>, 3, 4. Numbers are used for the scripts so the test does not depend
/// on how single letters are spoken.
const MULTISCRIPTS: &str = r#"
      <math>
        <mmultiscripts id="ms">
          <mi id="alap">x</mi>
          <mn id="alsó">1</mn>
          <mn id="felső">2</mn>
          <mprescripts/>
          <mn id="bal-alsó">3</mn>
          <mn id="bal-felső">4</mn>
        </mmultiscripts>
      </math>
    "#;

// --- ZoomOut / ZoomOutAll / ZoomInAll command phrases ---

#[test]
fn zoom_out_of_numerator() -> Result<()> {
    assert_nav_fragments(&["ZoomIn", "ZoomOut"], FRACTION, &["kicsinyítés", "számláló elhagyása"])
}

#[test]
fn zoom_out_of_power_base() -> Result<()> {
    assert_nav_fragments(&["ZoomIn", "ZoomOut"], POWER, &["kicsinyítés", "alap elhagyása"])
}

#[test]
fn zoom_out_all_of_numerator() -> Result<()> {
    // ZoomOutAll leaves every notation on the way up: the command phrase comes first, then the part that was left
    assert_nav_fragments(&["ZoomIn", "ZoomOutAll"], FRACTION, &["teljes kicsinyítés", "számláló elhagyása"])
}

#[test]
fn zoom_out_all_of_power_base() -> Result<()> {
    assert_nav_fragments(&["ZoomIn", "ZoomOutAll"], POWER, &["teljes kicsinyítés", "alap elhagyása"])
}

#[test]
fn zoom_out_at_top_of_math() -> Result<()> {
    // first ZoomOut leaves the numerator, the second one reaches the top: "zoomed out all of the way"
    assert_nav_fragments(&["ZoomIn", "ZoomOut", "ZoomOut"], FRACTION, &["teljesen kicsinyítve"])
}

#[test]
fn zoom_in_all_command_phrase() -> Result<()> {
    assert_nav_fragments(&["ZoomInAll"], FRACTION, &["teljes nagyítás"])
}

// --- MoveNext / MovePrevious inside a notation and at its edges (AutoZoomOut off) ---

#[test]
fn move_right_into_denominator() -> Result<()> {
    assert_nav_fragments(&["ZoomIn", "MoveNext"], FRACTION, &["ugrás jobbra", "nevező", "2"])
}

#[test]
fn move_right_at_end_of_denominator() -> Result<()> {
    assert_nav_fragments(&["ZoomIn", "MoveNext", "MoveNext"], FRACTION, &["nem tud jobbra mozogni", "nevező vége"])
}

#[test]
fn move_left_at_start_of_numerator() -> Result<()> {
    assert_nav_fragments(&["ZoomIn", "MovePrevious"], FRACTION, &["nem tud balra mozogni", "számláló eleje"])
}

#[test]
fn move_right_at_end_of_math_without_auto_zoom_out() -> Result<()> {
    // With AutoZoomOut off, the "edge of the notation" rule fires first, even at the right end of the whole
    // math: it says only "nem tud jobbra mozogni" (no part name, no "matematikai blokk vége").
    let expr = r#"
      <math>
        <mi id="x">x</mi>
      </math>
    "#;
    assert_nav_exact(&["ZoomIn", "MoveNext"], expr, false, "nem tud jobbra mozogni")
}

#[test]
fn move_left_at_start_of_math_without_auto_zoom_out() -> Result<()> {
    let expr = r#"
      <math>
        <mi id="x">x</mi>
      </math>
    "#;
    assert_nav_fragments(&["ZoomIn", "MovePrevious"], expr, &["nem tud balra mozogni"])
}

// --- AutoZoomOut on: leaving a notation automatically ("out of" -> "elhagyása") ---
//
// With AutoZoomOut == "True" the edge-of-notation rule is skipped. If there is more math beyond the notation,
// MoveNext/MovePrevious leave it automatically and say which part was left. Only at the very end of the math
// the "cannot move" announcement is made, followed by "matematikai blokk vége" / "matematikai blokk kezdete".
// Expected walk through 1/2 + y: ZoomIn (fraction), ZoomIn (numerator), MoveNext (denominator),
// MoveNext (leaves the denominator, lands on "+").

#[test]
fn auto_zoom_out_move_right_leaves_denominator() -> Result<()> {
    assert_nav_fragments_auto_zoom_out(
        &["ZoomIn", "ZoomIn", "MoveNext", "MoveNext"],
        FRACTION_PLUS_Y,
        &["ugrás jobbra", "nevező elhagyása"],
    )
}

#[test]
fn auto_zoom_out_move_left_leaves_numerator() -> Result<()> {
    // y + 1/2: ZoomIn (y), MoveNext (+), MoveNext (fraction), ZoomIn (numerator), MovePrevious (leaves it)
    assert_nav_fragments_auto_zoom_out(
        &["ZoomIn", "MoveNext", "MoveNext", "ZoomIn", "MovePrevious"],
        Y_PLUS_FRACTION,
        &["ugrás balra", "számláló elhagyása"],
    )
}

#[test]
fn auto_zoom_out_move_right_at_end_of_math() -> Result<()> {
    let expr = r#"
      <math>
        <mi id="x">x</mi>
      </math>
    "#;
    assert_nav_fragments_auto_zoom_out(&["ZoomIn", "MoveNext"], expr, &["nem tud jobbra mozogni", "matematikai blokk vége"])
}

#[test]
fn auto_zoom_out_read_and_describe_right_at_end_of_math() -> Result<()> {
    let expr = r#"
      <math>
        <mi id="x">x</mi>
      </math>
    "#;
    assert_nav_fragments_auto_zoom_out(&["ZoomIn", "ReadNext"], expr, &["nem tud jobbra olvasni", "matematikai blokk vége"])?;
    assert_nav_fragments_auto_zoom_out(&["ZoomIn", "DescribeNext"], expr, &["nem tud jobbra leírni", "matematikai blokk vége"])
}

#[test]
fn auto_zoom_out_move_left_at_start_of_math() -> Result<()> {
    let expr = r#"
      <math>
        <mi id="x">x</mi>
      </math>
    "#;
    assert_nav_fragments_auto_zoom_out(&["ZoomIn", "MovePrevious"], expr, &["matematikai blokk kezdete"])
}

// --- mmultiscripts part names ---
//
// Children of the mmultiscripts: base, subscript, superscript, <mprescripts/>, pre-subscript, pre-superscript.
// The names are "alap", "alsó index", "felső index", "bal alsó index" and "bal felső index".

#[test]
fn multiscripts_zoom_in_base() -> Result<()> {
    assert_nav_fragments(&["ZoomIn"], MULTISCRIPTS, &["nagyítás", "alap", "x"])
}

#[test]
fn multiscripts_move_right_to_subscript() -> Result<()> {
    assert_nav_fragments(&["ZoomIn", "MoveNext"], MULTISCRIPTS, &["ugrás jobbra", "alsó index", "1"])
}

#[test]
fn multiscripts_move_right_to_superscript() -> Result<()> {
    assert_nav_fragments(&["ZoomIn", "MoveNext", "MoveNext"], MULTISCRIPTS, &["ugrás jobbra", "felső index", "2"])
}

#[test]
fn multiscripts_move_right_to_pre_subscript() -> Result<()> {
    assert_nav_fragments(&["ZoomIn", "MoveNext", "MoveNext", "MoveNext"], MULTISCRIPTS, &["ugrás jobbra", "bal alsó index", "3"])
}

#[test]
fn multiscripts_move_right_to_pre_superscript() -> Result<()> {
    assert_nav_fragments(
        &["ZoomIn", "MoveNext", "MoveNext", "MoveNext", "MoveNext"],
        MULTISCRIPTS,
        &["ugrás jobbra", "bal felső index", "4"],
    )
}

#[test]
fn multiscripts_move_left_to_base() -> Result<()> {
    assert_nav_fragments(&["ZoomIn", "MoveNext", "MovePrevious"], MULTISCRIPTS, &["ugrás balra", "alap", "x"])
}

#[test]
fn multiscripts_zoom_out_of_base() -> Result<()> {
    assert_nav_fragments(&["ZoomIn", "ZoomOut"], MULTISCRIPTS, &["kicsinyítés", "alap elhagyása"])
}

#[test]
fn multiscripts_zoom_out_of_pre_superscript() -> Result<()> {
    assert_nav_fragments(
        &["ZoomIn", "MoveNext", "MoveNext", "MoveNext", "MoveNext", "ZoomOut"],
        MULTISCRIPTS,
        &["kicsinyítés", "bal felső index elhagyása"],
    )
}

// --- ReadCurrent / DescribeCurrent ---

#[test]
fn read_current() -> Result<()> {
    assert_nav_fragments(&["ZoomIn", "ReadCurrent"], FRACTION, &["az aktuális elem felolvasása"])
}

#[test]
fn describe_current() -> Result<()> {
    assert_nav_fragments(&["ZoomIn", "DescribeCurrent"], FRACTION, &["az aktuális elem leírása"])
}
