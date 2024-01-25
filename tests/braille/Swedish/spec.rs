use crate::common::*;

#[test]
fn number_letters_1_1() {
    // Note: test uses only one space to avoid triggering ommission code
    let expr = "<math><mn>4</mn><mi>a</mi><mo>&#xA0;</mo><mn>5</mn><mi>B</mi><mo>&#xA0;</mo><mn>7</mn><mi>a</mi><mi>b</mi></math>";
    test_braille("Swedish", expr, "⠼⠙⠱⠁⠀⠼⠑⠠⠃⠀⠼⠛⠱⠁⠃");
}

#[test]
fn arith_5_4() {
    let expr = "<math><mi>y</mi><mo>=</mo><mn>5</mn><mo>+</mo><mi>x</mi></math>";
    test_braille_prefs("Swedish", vec![("UseSpacesAroundAllOperators", "true")], expr, "⠽⠀⠶⠀⠼⠑⠀⠲⠀⠭");
    test_braille_prefs("Swedish", vec![("UseSpacesAroundAllOperators", "false")], expr, "⠽⠀⠶⠀⠼⠑⠲⠭");
}

#[test]
fn fraction_6_1() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mfrac><mi>x</mi><mn>2</mn></mfrac><mo>=</mo><mn>9</mn></math>"#;
    test_braille("Swedish", expr, "⠭⠳⠼⠃⠀⠶⠀⠼⠊");
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
    let expr = "<math><msubsup><mi>SO</mi><mn>4</mn><mrow><mn>2</mn><mo>-</mo></mrow></msubsup></math>";
    test_braille("Swedish", expr, "⠠⠠⠎⠕⠣⠼⠙⠘⠬⠼⠃⠤⠱");
}

#[test]
fn script_7_15() {
    let expr = "<math><msup><mi>a</mi><mrow><mo>-</mo><mi>b</mi></mrow></msup></math>";
    test_braille("Swedish", expr, "⠁⠬⠤⠃");
}

#[test]
fn script_7_16() {
    let expr = "<math><mi>a</mi><mover><mi>b</mi><mo>&#xAF;</mo></mover></math>";
    test_braille("Swedish", expr, "⠁⠃⠬⠬⠤");
}

#[test]
fn script_7_17() {
    let expr = "<math><mover><mi>y</mi><mo>~</mo></mover><mo>=</mo><mi>y</mi></math>";
    test_braille("Swedish", expr, "⠽⠬⠬⠒⠀⠶⠀⠽");
}

#[test]
fn root_7_25() {
    let expr = "<math><msqrt><mn>25</mn></msqrt><mo>=</mo><mn>5</mn></math>";
    test_braille("Swedish", expr, "⠩⠼⠃⠑⠀⠶⠀⠼⠑");
}

#[test]
fn root_7_32() {
    let expr = "<math><mroot><mn>27</mn><mn>3</mn></mroot><mo>=</mo><mn>3</mn></math>";
    test_braille("Swedish", expr, "⠬⠼⠉⠩⠼⠃⠛⠀⠶⠀⠼⠉");
}

#[test]
#[ignore]   // pending answer for what fraction indicators aren't used in the test
fn primes_9_26() {
    init_logger();
    let expr = "<math><mfrac><mi>d</mi><mrow><mi>d</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mfrac><mi>r</mi><mo>''</mo><mo>=</mo>
                        <mi>r</mi><mo>'</mo><mo>'</mo><mo>'</mo></math>";
    test_braille("Swedish", expr, "⠙⠳⠙⠦⠭⠴⠀⠗⠐⠐⠀⠶⠀⠗⠐⠐⠐");
}
