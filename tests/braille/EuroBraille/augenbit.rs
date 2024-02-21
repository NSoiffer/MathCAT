// Eurobraille tests for the basic mathml tags.
// These come from three pages:
//   https://augenbit.de/wiki/index.php?title=LaTeX-Manual_LaTeX_Grundregeln
//   https://augenbit.de/wiki/index.php?title=LaTeX-Manual_Sekundarstufe_1
//   https://augenbit.de/wiki/index.php?title=LaTeX-Manual_Sekundarstufe_2
// The names of the tests refer to these as augenbit0, augenbit1, and augenbit2.
// Each table on the page are numbered starting at 1, and then the examples in the table are numbered.
// Thus the first example on the first page is named "augenbit0_1_1" and the "a_{12}" example on that pages is "augenbit0_2_6".
// This naming scheme makes it easy to find the source of the example and what the translation should be.
use crate::common::*;


#[test]
fn augenbit0_1_1 () {
    let expr = "<math><mn>2</mn><mo>+</mo><mi>x</mi><mo>=</mo><mn>5</mn></math>";
    test_braille("EuroBraille", expr, "⠣⠀⠖⠭⠀⠶⠱");
}

#[test]
fn augenbit0_1_2 () {
    let expr = "<math><mo>|</mo><mi>x</mi><mo>-</mo><mn>1</mn><mo>|</mo><mo>=</mo><mo>|</mo><mn>1</mn><mo>-</mo><mi>x</mi><mo>|</mo></math>";
    test_braille("EuroBraille", expr, "⠌⠀⠭⠀⠤⠡⠌⠀⠶⠌⠀⠡⠀⠤⠭⠌");
}

#[test]
fn augenbit0_1_3 () {
    let expr = "<math><mi>n</mi><mo>!</mo><mo>=</mo><mi>n</mi><mo>*</mo><mo>(</mo><mi>n</mi><mo>-</mo><mn>1</mn><mo>)</mo><mo>!</mo></math>";
    test_braille("EuroBraille", expr, "⠝⠐⠀⠶⠝⠀⠔⠦⠝⠀⠤⠡⠴⠐");
}

#[test]
fn augenbit0_2_1 () {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup></math>";
    test_braille("EuroBraille", expr, "⠭⡮⠣");
}

#[test]
fn augenbit0_2_2 () {
    let expr = "<math><msup><mi>x</mi><mn>10</mn></msup></math>";
    test_braille("EuroBraille", expr, "⠭⡮⠷⠡⠬⠾");
}

#[test]
fn augenbit0_2_3 () {
    let expr = "<math><msub><mi>a</mi><mn>1</mn></msub><mo>+</mo><msub><mi>a</mi><mi>n</mi></msub></math>";
    test_braille("EuroBraille", expr, "⠁⡸⠡⠀⠖⠁⡸⠝");
}

#[test]
fn augenbit0_2_4 () {
    let expr = "<math><msup><mi>x</mi><mrow><mi>n</mi><mo>+</mo><mi>m</mi></mrow></msup></math>";
    test_braille("EuroBraille", expr, "⠭⡮⠷⠝⠀⠖⠍⠾");
}

#[test]
fn augenbit0_3_1 () {
    let expr = "<math><mi>n</mi><mo>&#x2192;</mo><mo>&#x221E;</mo></math>";
    test_braille("EuroBraille", expr, "⠝⠀⡌⠞⠕⠀⡌⠊⠝⠋⠞⠽");
}

#[test]
fn augenbit0_3_2 () {
    let expr = "<math><mi>x</mi><mo>∉</mo><mo>{</mo><mn>3</mn><mo>;</mo><mn>4</mn><mo>}</mo></math>";
    test_braille("EuroBraille", expr, "⠭⠀⡌⠝⠕⠞⠊⠝⠀⡌⠷⠩⠆⠀⠹⡌⠾");
}

#[test]
fn augenbit0_4_1 () {
    let expr = "<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mrow><mi>a</mi><mo>-</mo><mi>b</mi></mrow></mfrac></math>";
    test_braille("EuroBraille", expr, "⡌⠋⠗⠁⠉⠷⠁⠀⠖⠃⠾⠷⠁⠀⠤⠃⠾");
}

#[test]
fn augenbit0_4_2 () {
    let expr = "<math><msqrt><mi>a</mi><mo>+</mo><mi>b</mi></msqrt></math>";
    test_braille("EuroBraille", expr, "⡌⠎⠟⠗⠞⠷⠁⠀⠖⠃⠾");
}

#[test]
fn augenbit0_4_3 () {
    let expr = r#"<math>
            <munderover><mo>&#x2211;</mo><mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow><mi mathvariant="normal">&#x221E;</mi></munderover>
            <mfrac><mn>1</mn><msup><mn>2</mn><mi>n</mi></msup></mfrac><mo>=</mo><mn>1</mn>
        </math>"#;
    test_braille("EuroBraille", expr, "⡌⠎⠥⠍⡸⠷⠝⠀⠶⠡⠾⡮⡌⠊⠝⠋⠞⠽⠀⡌⠋⠗⠁⠉⠷⠡⠾⠷⠣⡮⠝⠾⠀⠶⠡");
}

#[test]
fn augenbit0_5_1 () {
    let expr = r#"<math><mover><mrow><mi>A</mi><mo>&#x222A;</mo><mi>B</mi></mrow><mo accent="true">&#x2015;</mo></mover></math>"#;
    test_braille("EuroBraille", expr, "⡌⠕⠧⠑⠗⠇⠊⠝⠑⠷⡁⠀⡌⠉⠥⠏⠀⡃⠾");
}

#[test]
fn augenbit1_1_1 () {
    let expr = r#"<math><mo>{</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo>,</mo><mn>4</mn><mo>}</mo></math>"#;
    test_braille("EuroBraille", expr, "⡌⠷⠀⠡⠂⠀⠣⠂⠀⠩⠂⠀⠹⠀⡌⠾");
}

#[test]
fn augenbit1_3_3 () {
    let expr = r#"<math><mi>x</mi><mo>&#xB1;</mo><mn>3</mn></math>"#;
    test_braille("EuroBraille", expr, "⠭⠀⡌⠏⠍⠀⠩");
}

#[test]
fn augenbit1_6_8 () {
    let expr = r#"<math>
            <mroot><msup><mi>a</mi><mn>2</mn></msup><mn>3</mn></mroot><mo>=</mo>
            <msup><mi>a</mi><mrow><mn>2</mn><mo>/</mo><mn>3</mn></mrow></msup>
        </math>"#;
    test_braille("EuroBraille", expr, "⡌⠎⠟⠗⠞⡷⠩⡾⠷⠁⡮⠣⠾⠀⠶⠁⡮⠷⠣⠲⠩⠾");
}

#[test]
fn augenbit1_6_11() {
    // this is a slightly cleaned up version that comes for the original example (via MathJax)
    let expr = r#" <math> <mrow>
        <msubsup>
            <mrow></mrow>
            <mrow><mn>95</mn></mrow>
            <mrow><mn>238</mn></mrow>
        </msubsup>
        <mrow><mi mathvariant="normal">U</mi></mrow>
        </mrow></math>"#;
    test_braille("EuroBraille", expr, "⡸⠷⠪⠱⠾⡮⠷⠣⠩⠳⠾⡥");
}


#[test]
fn augenbit1_7_7 () {
    let expr = r#"<math><msub><mi>log</mi><mi>a</mi></msub><mi>x</mi></math>"#;
    test_braille("EuroBraille", expr, "⡌⠇⠕⠛⡸⠁⠀⠭");
}

#[test]
fn augenbit1_7_10 () {
    let expr = r#"<math><msup><mi>cos</mi><mn>2</mn></msup><mi>&#x3B2;</mi></math>"#;
    test_braille("EuroBraille", expr, "⡌⠉⠕⠎⡮⠣⠀⡌⠃⠑⠞⠁");
}

#[test]
fn augenbit1_7_12 () {
    let expr = r#"<math><mi>cot</mi><mn>45</mn><mo>&#xB0;</mo></math>"#;
    test_braille("EuroBraille", expr, "⡌⠉⠕⠞⠀⠹⠱⢸");
}

#[test]
fn augenbit2_1_4 () {
    // original display code contains forced spaces not in the output -- they are cleaned up here
    init_logger();
    let expr = r#"<math>
            <msup><mi>f</mi><mo>'</mo> </msup><mo>(</mo><mi>x</mi><mo>)</mo><mo>,</mo>
            <msup><mi>f</mi><mo>''</mo></msup><mo>(</mo><mi>x</mi><mo>)</mo>
        </math>"#;
    test_braille("EuroBraille", expr, "⠋⠠⠦⠭⠴⠂⠀⠋⠠⠠⠦⠭⠴");
}

#[test]
fn augenbit2_3_2 () {
    let expr = r#"<math>
            <mover><mi>q</mi><mo stretchy="false">&#x2192;</mo></mover><mo>=</mo>
            <mfenced>
                <mtable columnspacing="1em">
                    <mtr><mtd><mo>&#x2212;</mo><mn>5</mn></mtd></mtr>
                    <mtr><mtd><mn>0</mn><mo>,</mo><mn>5</mn></mtd></mtr>
                    <mtr><mtd><mi>k</mi><mo>+</mo><mn>4</mn></mtd></mtr>
                </mtable>
            </mfenced></math>"#;
    // set number preferences to European style
    libmathcat::set_preference("BlockSeparators".to_string(), ". ".to_string()).unwrap();
    libmathcat::set_preference("DecimalSeparators".to_string(), ",".to_string()).unwrap();  
    test_braille("EuroBraille", expr, "⡌⠧⠑⠉⠷⠟⠾⠀⠶⠀⡌⠃⠑⠛⠊⠝⠷⠏⠍⠁⠞⠗⠊⠭⠾⠀⠤⠱⠀⡌⡌⠀⠬⠂⠱⠀⡌⡌⠀⠅⠖⠹⠀⡌⠑⠝⠙⠷⠏⠍⠁⠞⠗⠊⠭⠾");
}
