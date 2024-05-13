use crate::common::*;

#[test]
fn transpose() {
  let expr = "<math> <msup><mi>M</mi><mi>T</mi></msup> </math>";
  test("sv", "SimpleSpeak", expr, "cap m transpose");
}

#[test]
fn trace() {
  let expr = "<math> <mi>Tr</mi><mi>M</mi> </math>";
  test("sv", "SimpleSpeak", expr, "trace of cap m");
}

#[test]
fn dimension() {
  let expr = "<math> <mi>Dim</mi><mi>M</mi> </math>";
  test("sv", "SimpleSpeak", expr, "dimension of cap m");
}

#[test]
fn homomorphism() {
  let expr = "<math> <mi>Hom</mi><mo>(</mo><mi>M</mi><mo>)</mo> </math>";
  test("sv", "SimpleSpeak", expr, "homomorphism of cap m");
}

#[test]
fn kernel() {
  let expr = "<math> <mi>ker</mi><mrow><mo>(</mo><mi>L</mi><mo>)</mo></mrow> </math>";
  test("sv", "SimpleSpeak", expr, "kernel of cap l");
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
  test("sv", "SimpleSpeak", expr, "norm of f");
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
  test("sv", "SimpleSpeak", expr, "p norm of f");
}