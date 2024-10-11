use crate::common::*;

#[test]
fn msub_simple() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> </math>";
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1");
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "x sub 1");
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x sub 1");
  }

#[test]
fn msub_simple_number_follows() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mn>10</mn>,<mn>2</mn></msup> </math>";
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1 10 squared");
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x sub 1 10 squared");
}

#[test]
fn msub_simple_non_number_follows() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mi>x</mi>,<mn>2</mn></msup> </math>";
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1 x squared");
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x sub 1 x squared");
}