// French braille tests for the basic mathml tags
// These tests are from the French braille authority's publication "NOTATION PIATHÉMATIQUE BRAILLE" (Première édition janvier 2007)
//  https://www.avh.asso.fr/sites/default/files/notation_mathematique_braille2_0.pdf
use crate::common::*;

#[test]
fn fraction_7_1() {
    let expr= r#"<math><mfrac><mn>2</mn><mn>3</mn></mfrac><mo>=</mo><mfrac><mn>4</mn><mn>6</mn></mfrac></math>"#;
    test_braille("French", expr, "⠣⠌⠩⠶⠹⠌⠫");
}

#[test]
fn fraction_8_1() {
    let expr= r#"<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mi>a</mi></mfrac></math>"#;
    test_braille("French", expr, "⠰⠁⠖⠃⠆⠌⠁");
}

#[test]
fn superscript_8_4() {
    let expr= r#"<math><msup><mi>e</mi><mn>7</mn></msup></math>"#;
    test_braille("French", expr, "⠑⠈⠻");
}

#[test]
fn superscript_8_6() {
    let expr= r#"<math><msup><mi>e</mi><mrow><mo>-</mo><mi mathvariant="normal">&#x3C0;</mi></mrow></msup></math>"#;
    test_braille("French", expr, "⠑⠈⠤⠘⠏");
}

#[test]
fn subscript_9_2_1() {
    let expr= r#"<math><msub><mi>a</mi><mi>p</mi></msub></math>"#;
    test_braille("French", expr, "⠁⠢⠏");
}

#[test]
fn subscript_9_2_3() {
    let expr= r#"<math><msub><mi>u</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub></math>"#;
    test_braille("French", expr, "⠥⠢⠰⠝⠖⠡⠆");
}

#[test]
fn scripts_9_4_1() {
    let expr= r#"<math><msubsup><mi>a</mi><mn>0</mn><mo>'</mo></msubsup></math>"#;
    test_braille("French", expr, "⠁⠄⠢⠼");
}

#[test]
fn scripts_9_4_3() {
    let expr= r#"<math><msubsup><mi>C</mi><mi>m</mi><mi>p</mi></msubsup><mo>=</mo>
                             <msubsup><mi>C</mi><mi>m</mi><mrow><mi>m</mi><mo>-</mo><mi>p</mi></mrow></msubsup></math>"#;
    test_braille("French", expr, "⠨⠉⠢⠍⠈⠏⠶⠨⠉⠢⠍⠈⠰⠍⠤⠏⠆");
}

#[test]
fn scripts_9_5_1() {
    let expr= r#"<math><mmultiscripts><mi>A</mi><mprescripts/><none/><mi>t</mi></mmultiscripts></math>"#;
    test_braille("French", expr, "⠈⠞⠨⠁");
}

#[test]
fn chem_9_5_4() {
    let expr= r#"<math><mmultiscripts><mi mathvariant="normal">O</mi><mprescripts/><mn>8</mn><mn>16</mn></mmultiscripts></math>"#;
    test_braille("French", expr, "⠨⠕⠠⠢⠳⠠⠈⠡⠫");
}

#[test]
fn root_10_3() {
    let expr= r#"<math><mroot><msup><mrow><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo></mrow><mn>3</mn></msup><mn>6</mn></mroot><mo>=</mo>
                            <msqrt><mi>a</mi><mo>+</mo><mi>b</mi></msqrt></math>"#;
    test_braille("French", expr, "⠈⠫⠜⠰⠦⠁⠖⠃⠴⠈⠩⠆⠶⠜⠰⠁⠖⠃⠆");
}

#[test]
fn accent_12_1() {
    let expr= r#"<math><mover><mi>v</mi><mo>&#x2192;</mo></mover></math>"#;
    test_braille("French", expr, "⠨⠒⠧");
}

#[test]
fn accent_12_2() {
    let expr= r#"<math><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo>&#xAF;</mo></mover></math>"#;
    test_braille("French", expr, "⠸⠒⠨⠁⠨⠃");
}

#[test]
fn accent_12_3() {
    let expr= r#"<math><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo>⌒</mo></mover></math>"#;
    test_braille("French", expr, "⠈⠒⠨⠁⠨⠃");
}

#[test]
fn accent_12_5() {
    let expr= r#"<math><mover><mrow><mo>(</mo><mi>O</mi><mi>x</mi><mo>,</mo><mi>O</mi><mi>y</mi><mo>)</mo></mrow><mo>^</mo></mover></math>"#;
    test_braille("French", expr, "⠘⠒⠦⠨⠕⠭⠂⠨⠕⠽⠴");
}

#[test]
fn equations_13_4_1() {
    init_logger();
    let expr= r#"<math><mfenced open="{" close=""><mtable columnalign="left">
                <mtr><mtd><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi><mo>=</mo><mn>5</mn></mtd></mtr>
                <mtr><mtd><mn>3</mn><mi>x</mi><mo>-</mo><mn>7</mn><mi>y</mi><mo>=</mo><mn>8</mn></mtd></mtr>
            </mtable></mfenced></math>"#;
    test_braille("French", expr, "⠸⠦⠣⠭⠖⠩⠽⠶⠱⠀⠠⠜⠀⠩⠭⠤⠻⠽⠶⠳");
}

#[test]
fn lim_14_3() {
    let expr= r#"<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><mo>+</mo><mo>&#x221E;</mo></mrow></munder>
        <mfrac><mrow><mi>ln</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>2</mn><mo>)</mo></mrow><mi>x</mi></mfrac><mo>=</mo><mn>0</mn></math>"#;
    test_braille("French", expr, " ⠇⠊⠍⠢⠢⠰⠭⠸⠱⠖⠘⠉⠆⠇⠝⠦⠭⠖⠣⠴⠌⠭⠶⠼");
}

#[test]
fn integral_14_4_3() {
    let expr= r#"<math><msub><mo>&#x222F;</mo><mi>S</mi></msub><mi>f</mi><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>)</mo><mi>d</mi><mi>x</mi><mi>d</mi><mi>y</mi></math>"#;
    test_braille("French", expr, "⠨⠯⠯⠢⠨⠎⠋⠦⠭⠂⠽⠴⠙⠭⠙⠽");
}

#[test]
fn log_14_5_1() {
    let expr= r#"<math><mi>ln</mi><mfenced><mfrac><mi>a</mi><mi>b</mi></mfrac></mfenced></math>"#;
    test_braille("French", expr, "⠇⠝⠦⠁⠌⠃⠴");
}

#[test]
fn log_14_5_3() {
    let expr= r#"<math><msub><mi>log</mi><mn>7</mn></msub><mi>x</mi><mo>=</mo>
                        <mfrac><mrow><mi>ln</mi><mi>x</mi></mrow><mrow><mi>ln</mi><mn>7</mn></mrow></mfrac></math>"#;
    test_braille("French", expr, "⠇⠕⠛⠢⠻⠰⠭⠆⠶⠇⠝⠰⠭⠆⠌⠇⠝⠰⠻⠆");
}

#[test]
fn symbols_15_1() {
    let expr= r#"<math><mo>&#x2203;</mo><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x2208;</mo><mi>A</mi></math>"#;
    test_braille("French", expr, "⠸⠡⠭⠒⠭⠘⠡⠨⠁");
}
