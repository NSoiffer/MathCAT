/// Tests for geometry listed in intent
///   ABC as mtext and as separated letters
use crate::common::*;

#[test]
fn arc() {
  let expr = "<math>  <mover><mrow><mi>B</mi><mi>C</mi></mrow><mo>⌒</mo></mover> </math>";
  test("zh-tw", "SimpleSpeak", expr, "弧 大寫 b 大寫 c");
}

#[test]
fn ray() {
  let expr = "<math> <mover><mrow><mi>X</mi><mi>Y</mi></mrow><mo>&#xAF;</mo></mover> </math>";
  test("zh-tw", "SimpleSpeak", expr, "線段 大寫 x 大寫 y");
}

#[test]
fn arc_mtext() {
  let expr = "<math> <mover><mtext>BC</mtext><mo>⌒</mo></mover> </math>";
  test("zh-tw", "SimpleSpeak", expr, "弧 大寫 b 大寫 c");
}

#[test]
fn ray_mtext() {
  let expr = "<math> <mover><mtext>XY</mtext><mo>→</mo></mover> </math>";
  test("zh-tw", "SimpleSpeak", expr, "向量 大寫 x 大寫 y");
}
