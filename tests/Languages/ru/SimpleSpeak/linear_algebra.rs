use crate::common::*;

use anyhow::Result;

#[test]
fn transpose() -> Result<()> {
  let expr = "<math> <msup><mi>M</mi><mi>T</mi></msup> </math>";
  // Name the transposed matrix before its letter, as requested for Russian speech.
  test("ru", "SimpleSpeak", expr, "транспонированная матрица эм")?;
  test("ru", "ClearSpeak", expr, "транспонированная матрица эм")?;
  return Ok(());
}

#[test]
fn determinant_verbose() -> Result<()> {
  // Explicit determinant intent must use the mathematical noun without an English article.
  let expr = r#"<math><mrow intent="determinant($a)"><mi arg="a">a</mi></mrow></math>"#;
  test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "определитель а")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "определитель а")?;
  return Ok(());
}

#[test]
fn trace() -> Result<()> {
  let expr = "<math> <mi>Tr</mi><mi>M</mi> </math>";
  test("ru", "SimpleSpeak", expr, "след от заглавной эм")?;
  return Ok(());
}

#[test]
fn dimension() -> Result<()> {
  let expr = "<math> <mi>Dim</mi><mi>M</mi> </math>";
  test("ru", "SimpleSpeak", expr, "размерность от заглавной эм")?;
  return Ok(());
}

#[test]
fn homomorphism() -> Result<()> {
  let expr = "<math> <mi>Hom</mi><mo>(</mo><mi>M</mi><mo>)</mo> </math>";
  test("ru", "SimpleSpeak", expr, "гомоморфизм от заглавной эм")?;
  return Ok(());
}

#[test]
fn kernel() -> Result<()> {
  let expr = "<math> <mi>ker</mi><mrow><mo>(</mo><mi>L</mi><mo>)</mo></mrow> </math>";
  test("ru", "SimpleSpeak", expr, "ядро от заглавной эль")?;
  return Ok(());
}

#[test]
fn norm() -> Result<()> {
  let expr = "  <math>
    <mrow>
      <mo>∥</mo>
      <mi>f</mi>
      <mo>∥</mo>
    </mrow>
</math>
";
  test("ru", "SimpleSpeak", expr, "норма от эф")?;
  return Ok(());
}

#[test]
fn norm_non_simple() -> Result<()> {
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
  test("ru", "SimpleSpeak", expr, "норма от выражения икс плюс игрек конец нормы")?;
  return Ok(());
}

#[test]
fn norm_subscripted() -> Result<()> {
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
  // Speak the norm's operand first, then identify its subscript explicitly.
  test("ru", "SimpleSpeak", expr, "норма эф с индексом пэ")?;
  test("ru", "ClearSpeak", expr, "норма эф с индексом пэ")?;
  return Ok(());
}

#[test]
fn norm_with_subscript_on_closing_bar() -> Result<()> {
  // Recognize the TeX4ht form in which only the closing norm bar carries the index.
  let expr = "<math><mrow><mo>∥</mo><mi>A</mi><msub><mrow><mo>∥</mo></mrow><mi>p</mi></msub></mrow></math>";
  test("ru", "SimpleSpeak", expr, "норма заглавной а с индексом пэ")?;
  test("ru", "ClearSpeak", expr, "норма заглавной а с индексом пэ")?;

  let doubled_bars = "<math><mrow><mo>|</mo><mo>|</mo><mi>x</mi><mo>|</mo><msub><mrow><mo>|</mo></mrow><mi>∞</mi></msub></mrow></math>";
  test("ru", "SimpleSpeak", doubled_bars, "норма икс с индексом бесконечность")?;
  test("ru", "ClearSpeak", doubled_bars, "норма икс с индексом бесконечность")?;
  return Ok(());
}

#[test]
fn norm_with_doubled_ordinary_bars() -> Result<()> {
  // TeX4ht sometimes emits a norm as two adjacent ordinary bars on each side.
  let expr = "<math><mrow><mo>|</mo><mo>|</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>|</mo><mo>|</mo></mrow></math>";
  test("ru", "SimpleSpeak", expr, "норма икс плюс игрек")?;
  test("ru", "ClearSpeak", expr, "норма икс плюс игрек")?;
  return Ok(());
}

#[test]
fn not_gradient() -> Result<()> {
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
  test("ru", "SimpleSpeak", expr, "скобка открывается бэ умножить набла скобка закрывается умножить на а")?;
  return Ok(());
}
