use crate::common::*;

#[test]
fn msub_simple() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1");
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "x, senket 1");
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x, senket 1");
  }

#[test]
fn msub_not_simple() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1,2</mn> </msub> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse"), ("DecimalSeparators", ","), ("BlockSeparators", ".")], expr, "x, senket 1,2");
  }

#[test]
fn msubsup_not_simple() {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1,2</mn> <mn>3</mn></msubsup> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse"), ("DecimalSeparators", ","), ("BlockSeparators", ".")], expr, "x, senket 1,2, i tredje");
  }

#[test]
fn msub_simple_mi() {
    let expr = "<math> <msub> <mi>x</mi> <mi>i</mi> </msub> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x, senket i");
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x, senket i");
}

#[test]
fn msub_simple_number_follows() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mn>10</mn><mn>2</mn></msup> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, 10 i andre");
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x, senket 1; 10 i andre");
}

#[test]
fn msub_simple_non_number_follows() {
    let expr = "<math> <msubsup> <mi>x</mi> <mn>1</mn> <mn>2</mn> </msubsup> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, i andre");
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x, senket 1, i andre");
}

#[test]
fn msubsup_simple() {
    let expr = "<math> <msub> <mi>x</mi> <mn>1</mn> </msub> <msup><mi>x</mi>,<mn>2</mn></msup> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "x 1, x i andre");
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "x, senket 1; x i andre");
}