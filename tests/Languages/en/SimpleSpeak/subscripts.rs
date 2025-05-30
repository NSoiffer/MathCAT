use crate::common::*;

#[test]
fn msub_simple() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> </math>";
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1");
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "x sub 1");
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x sub 1");
  }

#[test]
fn msub_not_simple() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1.2</mn> </msub> </math>";
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x sub 1.2");
  }

#[test]
fn msubsup_not_simple() {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1.2</mn> <mn>3</mn></msubsup> </math>";
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x sub 1.2, cubed");
  }

#[test]
fn msub_simple_mi() {
    let expr = "<math> <msub> <mi>x</mi> <mi>i</mi> </msub> </math>";
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x sub i");
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x sub i");
}

#[test]
fn msub_simple_number_follows() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mn>10</mn><mn>2</mn></msup> </math>";
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, 10 squared");
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x sub 1, 10 squared");
}

#[test]
fn msub_simple_non_number_follows() {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1</mn> <mn>2</mn> </msubsup> </math>";
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, squared");
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x sub 1, squared");
}

#[test]
fn msubsup_simple() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mi>x</mi>,<mn>2</mn></msup> </math>";
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, x squared");
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x sub 1, x squared");
}