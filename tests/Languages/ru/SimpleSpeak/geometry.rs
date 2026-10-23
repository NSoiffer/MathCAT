/// Tests for geometry listed in intent
///   ABC as mtext and as separated letters
use crate::common::*;

use anyhow::Result;

#[test]
fn arc() -> Result<()> {
  let expr = "<math>  <mover><mrow><mi>B</mi><mi>C</mi></mrow><mo>⌒</mo></mover> </math>";
  test("ru", "SimpleSpeak", expr, "дуга бэ большое цэ большое")?;
  return Ok(());
}

#[test]
fn line_segment_verbose() -> Result<()> {
  // The verbose name of a line segment is "отрезок", not a literal English translation.
  let expr = "<math><mover><mrow><mi>X</mi><mi>Y</mi></mrow><mo>¯</mo></mover></math>";
  test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
      "отрезок от заглавной икс до заглавной игрек")?;
  return Ok(());
}

#[test]
fn arc_verbose() -> Result<()> {
  // Verbose arc speech must not introduce an article translated as a letter.
  let expr = "<math><mover><mrow><mi>B</mi><mi>C</mi></mrow><mo>⌒</mo></mover></math>";
  test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
      "дуга бэ большое цэ большое")?;
  return Ok(());
}

#[test]
fn coordinate_verbose() -> Result<()> {
  // The closing marker ends the coordinates, rather than naming an endpoint.
  let expr = r#"<math><mrow intent="coordinate($x,$y)"><mn arg="x">1</mn><mn arg="y">2</mn></mrow></math>"#;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Verbose")], expr,
      "точка с координатами, 1 запятая 2, конец координат")?;
  return Ok(());
}

#[test]
fn ray_verbose() -> Result<()> {
  // B specifies the ray's direction, not an endpoint.
  let expr = "<math><mover><mrow><mi>X</mi><mi>Y</mi></mrow><mo>→</mo></mover></math>";
  test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
      "луч с началом в заглавной икс проходящий через заглавную игрек")?;
  return Ok(());
}

#[test]
fn ray() -> Result<()> {
  let expr = "<math> <mover><mrow><mi>X</mi><mi>Y</mi></mrow><mo>&#xAF;</mo></mover> </math>";
  test("ru", "SimpleSpeak", expr, "отрезок икс большое игрек большое")?;
  return Ok(());
}

#[test]
fn arc_mtext() -> Result<()> {
  let expr = "<math> <mover><mtext>BC</mtext><mo>⌒</mo></mover> </math>";
  test("ru", "SimpleSpeak", expr, "дуга бэ большое цэ большое")?;
  return Ok(());
}

#[test]
fn ray_mtext() -> Result<()> {
  let expr = "<math> <mover><mtext>XY</mtext><mo>→</mo></mover> </math>";
  test("ru", "SimpleSpeak", expr, "луч икс большое игрек большое")?;
  return Ok(());
}
