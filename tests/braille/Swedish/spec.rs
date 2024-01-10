use crate::common::*;

#[test]
fn arith_5_4() {
    let expr = "<math><mi>y</mi><mo>=</mo><mn>5</mn><mo>+</mo><mi>x</mi></math>";
    test_braille_prefs("Swedish", vec![("UseSpacesAroundAllOperators", "true")], expr, "⠽⠀⠶⠀⠼⠑⠀⠲⠀⠭");
    test_braille_prefs("Swedish", vec![("UseSpacesAroundAllOperators", "false")], expr, "⠽⠀⠶⠀⠼⠑⠲⠭");
}

#[test]
fn fraction_6_4() {
    let expr = "<math><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>x</mi><mo>-</mo><mn>1</mn></mrow></mfrac></math>";
    test_braille("Swedish", expr, "⠿⠦⠭⠲⠼⠁⠳⠭⠤⠼⠁⠿⠴");
}

#[test]
fn script_7_4() {
    let expr = "<math><msup><mn>10</mn><mrow><mo>-</mo><mn>12</mn></mrow></msup></math>";
    test_braille("Swedish", expr, "⠼⠁⠚⠬⠤⠼⠁⠃");
}

#[test]
fn script_7_7() {
    let expr = "<math><msup><mn>2</mn><msup><mn>2</mn><mrow><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup></msup></math>";
    test_braille("Swedish", expr, "⠼⠃⠬⠼⠃⠘⠬⠝⠤⠼⠁⠱");
}

#[test]
fn script_7_12() {
    // From ChemType
    init_logger();
    let expr = "<math><msubsup><mi>SO</mi><mn>4</mn><mrow><mn>2</mn><mo>-</mo></mrow></msubsup></math>";
    test_braille("Swedish", expr, "⠠⠠⠎⠕⠣⠼⠙⠘⠬⠼⠃⠤⠱");
}

#[test]
fn root_7_25() {
    let expr = "<math><msqrt><mn>25</mn></msqrt><mo>=</mo><mn>5</mn></math>";
    test_braille("Swedish", expr, "⠩⠼⠃⠑⠀⠶⠀⠼⠑");
}
