/// Tests for geometry listed in intent
///   ABC as mtext and as separated letters
use crate::common::*;

#[test]
fn arc() {
  let expr = "<math>  <mover><mrow><mi>B</mi><mi>C</mi></mrow><mo>⌒</mo></mover> </math>";
  test("sv", "SimpleSpeak", expr, "bågen versal b versal c");
}

#[test]
fn ray() {
  let expr = "<math> <mover><mrow><mi>X</mi><mi>Y</mi></mrow><mo>&#xAF;</mo></mover> </math>";
  test("sv", "SimpleSpeak", expr, "linje-segmentet versal x versal y");
}

#[test]
fn arc_mtext() {
  let expr = "<math> <mover><mtext>BC</mtext><mo>⌒</mo></mover> </math>";
  test("sv", "SimpleSpeak", expr, "bågen versal b versal c");
}

#[test]
fn ray_mtext() {
  let expr = "<math> <mover><mtext>XY</mtext><mo>→</mo></mover> </math>";
  test("sv", "SimpleSpeak", expr, "den riktade sträckan versal x versal y");
}
