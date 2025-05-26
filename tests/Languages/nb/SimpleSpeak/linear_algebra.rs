use crate::common::*;

#[test]
fn transpose() {
  let expr = "<math> <msup><mi>M</mi><mi>T</mi></msup> </math>";
  test("nb", "SimpleSpeak", expr, "stor m transponert");
}

#[test]
fn trace() {
  let expr = "<math> <mi>Tr</mi><mi>M</mi> </math>";
  test("nb", "SimpleSpeak", expr, "sporet av stor m");
}

#[test]
fn dimension() {
  let expr = "<math> <mi>Dim</mi><mi>M</mi> </math>";
  test("nb", "SimpleSpeak", expr, "dimensjonen til stor m");
}

#[test]
fn homomorphism() {
  let expr = "<math> <mi>Hom</mi><mo>(</mo><mi>M</mi><mo>)</mo> </math>";
  test("nb", "SimpleSpeak", expr, "mengden av homomorfier på stor m");
}

#[test]
fn kernel() {
  let expr = "<math> <mi>ker</mi><mrow><mo>(</mo><mi>L</mi><mo>)</mo></mrow> </math>";
  test("nb", "SimpleSpeak", expr, "nullrommet til stor l");
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
  test("nb", "SimpleSpeak", expr, "normen til f");
}

#[test]
fn norm_non_simple() {
  let expr = "  <math>
    <mrow>
      <mo>∥</mo>
      <mi>x</mi>
      <mo>+</mo>
      <mi>y</mi>
      <mo>∥</mo>
    </mrow>
</math>
";
  test("nb", "SimpleSpeak", expr, "normen til x pluss y, slutt norm");
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
  test("nb", "SimpleSpeak", expr, "p normen til f");
}

#[test]
fn not_gradient() {
  // the nabla is at the end, so it can't be gradient because it doesn't operate on anything
  let expr = r#"<math>
  <mo>(</mo>
  <mi>b</mi>
  <mo>&#x22C5;</mo>
  <mrow>
    <mo>&#x2207;</mo>
  </mrow>
  <mo>)</mo>
  <mi>a</mi>
</math>
"#;
  test("nb", "SimpleSpeak", expr, ", startparentes; b ganger nabla; sluttparentes; ganger a");
}