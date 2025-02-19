use crate::common::*;

#[test]
fn transpose() {
  let expr = "<math> <msup><mi>M</mi><mi>T</mi></msup> </math>";
  test("sv", "SimpleSpeak", expr, "versal m transponat");
}

#[test]
fn trace() {
  let expr = "<math> <mi>Tr</mi><mi>M</mi> </math>";
  test("sv", "SimpleSpeak", expr, "spåret av versal m");
}

#[test]
fn dimension() {
  let expr = "<math> <mi>Dim</mi><mi>M</mi> </math>";
  test("sv", "SimpleSpeak", expr, "dimensionen av versal m");
}

#[test]
fn homomorphism() {
  let expr = "<math> <mi>Hom</mi><mo>(</mo><mi>M</mi><mo>)</mo> </math>";
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")],
  expr, "endomorfismerna på versal m");
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Medium")],
  expr, "mängden av endomorfismer på versal m");
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Verbose")],
  expr, "mängden av endomorfismer på versal m");
}

#[test]
fn kernel() {
  let expr = "<math> <mi>ker</mi><mrow><mo>(</mo><mi>L</mi><mo>)</mo></mrow> </math>";
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")],
  expr, "noll-rummet versal l");
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Medium")],
  expr, "noll-rummet till versal l");
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Verbose")],
  expr, "noll-rummet till versal l");
}

#[test]
fn norm() {
  let expr = "  <math>
    <mrow>
      <mo>∥</mo>
      <mi>f</mi>
      <mo>∥</mo>
    </mrow>
</math>
";
  test("sv", "SimpleSpeak", expr, "normen av f");
}

#[test]
fn norm_subscripted() {
  let expr = "  <math>
    <msub>
      <mrow>
        <mo>∥</mo>
        <mi>f</mi>
        <mo>∥</mo>
      </mrow>
      <mi>p</mi>
    </msub>
</math>
";
  test("sv", "SimpleSpeak", expr, "p normen av f");
}
