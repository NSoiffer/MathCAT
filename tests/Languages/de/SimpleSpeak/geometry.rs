/// Tests for geometry listed in intent
///   ABC as mtext and as separated letters
use crate::common::*;

#[test]
fn arc() {
  let expr = "<math>  <mover><mrow><mi>B</mi><mi>C</mi></mrow><mo>⌒</mo></mover> </math>";
  test("de", "SimpleSpeak", expr, "arc cap b cap c");
}

#[test]
fn ray() {
  let expr = "<math> <mover><mrow><mi>X</mi><mi>Y</mi></mrow><mo>&#xAF;</mo></mover> </math>";
  test("de", "SimpleSpeak", expr, "line segment cap x cap y");
}

#[test]
fn arc_mtext() {
  let expr = "<math> <mover><mtext>BC</mtext><mo>⌒</mo></mover> </math>";
  test("de", "SimpleSpeak", expr, "arc cap b cap c");
}

#[test]
fn ray_mtext() {
  let expr = "<math> <mover><mtext>XY</mtext><mo>→</mo></mover> </math>";
  test("de", "SimpleSpeak", expr, "ray cap x cap y");
}
