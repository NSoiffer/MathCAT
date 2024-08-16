// Tests based on the Finnish flavor of ASCIIMath located on the Finnish delegation for braille's "Matematiikan, fysiikan ja kemian merkinnät elektronisissa oppikirjoissa" (https://www.pistekirjoitus.fi/julkaisut/matematiikka-ja-tietotekniikka/).
// Tests will be named according to the page and its position on the page in the specification and then some identifying word.
use crate::common::*;


#[test]
fn p12_1_equal () {
    let expr = r#"<math><mn>3</mn><mo>+</mo><mn>4</mn><mo>=</mo><mn>7</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"3 +4 =7");
}

#[test]
fn p12_2_not_equal () {
    let expr = r#"<math><mn>5</mn><mo>&#x2212;</mo><mn>2</mn><mo>&#8800;</mo><mn>2</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"5 -2 !=2");
}

#[test]
fn p12_3_opposite () {
    let expr = r#"<math><mn>9</mn><mo>&#x2212;</mo><mn>3</mn><mo>&#x2260;</mo><mn>5</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"9 -3 != 5");
}

#[test]
fn p12_4_multiplication_visible_op () {
    let expr = r#"<math><mn>27</mn><mo>&#183;</mo><mn>3</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"27 *3");
}

#[test]
fn p12_5_simple_frac () {
    let expr = r#"<math><mfrac><mn>1</mn><mn>3</mn></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"1/3");
}

#[test]
fn p12_9_ratio () {
    let expr = r#"<math><mn>1</mn><mo>:</mo><mn>1000</mn></math>"#;
    test_braille_prefs("ASCIIMath-fi", expr, r"1 :1000");
}

#[test]
fn p12_10_fractional () {
    let expr = r#"<math><mfrac><mrow><mn>6</mn><mo>&#8290;</mo><mi>x</mi><mo>+</mo><mn>3</mn><mo>&#8290;</mo><mi>x</mi></mrow><mrow><mrow><mn>6</mn><mo>&#8290;</mo><mi>x</mi><mo>&#x2212;</mo><mn>4</mn><mo>&#8290;</mo><mi>x</mi></mrow></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"(6 x +3 x) /(6 x -4 x)");
}

#[test]
fn p12_11_absolute_value_eq () {
    let expr = r#"<math><mo>|</mo><mo>&#x2212;</mo><mo>(</mo><mn>2</mn><mo>+</mo><mn>5</mn><mo>)</mo><mo>|</mo><mo>=</mo><mo>|</mo><mn>&#x2212;7</mn><mo>|</mo><mo>=</mo><mn>7</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"|-(2 +5)| =|-7| =7");
}

#[test]
fn p12_13_natural_numbers () {
    let expr = r#"<math><mi>&#x2115;</mi><mo>=</mo><mo>{</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo>&#8230;</mo><mo>}</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"NN ={0, 1, 2, 3, ...}");
}

#[test]
fn p12_14_whole_numbers () {
    let expr = r#"<math><mi>&#8484;</mi><mo>=</mo><mo>{</mo><mo>&#8230;</mo><mo>,</mo><mo>&#x2212;</mo><mn>2</mn><mo>,</mo><mo>&#x2212;</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mo>&#8230;</mo><mo>}</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"ZZ ={..., -2, 1, 0, 1, 2, ...}");
}

#[test]
fn p23_3_pi () {
    let expr = r#"<math><mi>&#x3C0;</mi><mo>&#x2248;</mo><mn>3</mn><mo>,</mo><mn>14</mn></math>"#;
    test_braille_prefs("ASCIIMath", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, r"~p ~~3,14");
}

#[test]
fn p13_4_less_than () {
    let expr = r#"<math><mi>x</mi><mo>&#60;</mo><mn>18</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"x < 18");
}

#[test]
fn p13_5_greater_or_equal () {
    let expr = r#"<math><mn>2</mn><mo>&#8290;</mo><mi>x</mi><mo>&#8805;</mo><mn>6</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"2 x >= 6");
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
    test_braille("ASCIIMath-fi", expr, r"\ _(95)^(238)U");
}


#[test]
fn augenbit1_7_7 () {
    let expr = r#"<math><msub><mi>log</mi><mi>a</mi></msub><mi>x</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"log_a x");
}

#[test]
fn augenbit1_7_10 () {
    let expr = r#"<math><msup><mi>cos</mi><mn>2</mn></msup><mi>&#x3B2;</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"cos^2 beta");
}

#[test]
fn augenbit1_7_12 () {
    let expr = r#"<math><mi>cot</mi><mn>45</mn><mo>&#xB0;</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"cot 45°");
}

#[test]
fn augenbit1_8_2 () {
    let expr = r#"<math><mi mathvariant="normal">&#x25B3;</mi><mi>A</mi><mi>B</mi><mi>C</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"/_\ABC");
}

#[test]
fn augenbit1_8_4 () {
    let expr = r#"<math><mi>&#x3B1;</mi><mo>,</mo><mi>&#x3B2;</mi><mo>,</mo><mi>&#x3B3;</mi><mo>,</mo><mi>&#x3B4;</mi><mo>,</mo><mi>&#x3B5;</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"alpha, beta, gamma, delta, epsilon");
}

#[test]
fn augenbit2_1_3 () {
    // original display code contains forced spaces not in the output -- they are cleaned up here
    let expr = r#"<math>
        <munder>
        <mo movablelimits="true">lim</mo>
        <mrow>
            <mi>x</mi>
            <mo accent="false" stretchy="false">&#x2192;</mo>
            <msub><mi>x</mi><mn>0</mn></msub>
        </mrow>
        </munder>
    </math>"#;
    test_braille("ASCIIMath-fi", expr, r"lim_(x->x_0)");
}

#[test]
fn augenbit2_1_4 () {
    // original display code contains forced spaces not in the output -- they are cleaned up here
    let expr = r#"<math>
            <msup><mi>f</mi><mo>'</mo> </msup><mo>(</mo><mi>x</mi><mo>)</mo><mo>,</mo>
            <msup><mi>f</mi><mo>''</mo></msup><mo>(</mo><mi>x</mi><mo>)</mo>
        </math>"#;
    test_braille("ASCIIMath-fi", expr, r"f'(x), f''(x)");
}

#[test]
fn augenbit2_2_2 () {
    // original display code contains forced spaces not in the output -- they are cleaned up here
    let expr = r#"<math>
        <mo minsize="2.047em" maxsize="2.047em">(</mo>
        <mfrac linethickness="0"><mi>n</mi><mi>k</mi></mfrac>
        <mo minsize="2.047em" maxsize="2.047em">)</mo>
    </math>"#;
    test_braille("ASCIIMath-fi", expr, r"((n), (k))");
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
    test_braille_prefs("ASCIIMath-fi", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, 
                r"vec q = ([-5], [0,5], [k+4])");
}

#[test]
fn augenbit2_3_4 () {
    let expr = r#"<math>
        <mo>(</mo>
        <mtable columnspacing="1em" rowspacing="4pt">
        <mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd><mtd><mi>c</mi></mtd></mtr>
        <mtr><mtd><mi>d</mi></mtd><mtd><mi>e</mi></mtd><mtd><mi>f</mi></mtd></mtr>
        </mtable>
        <mo>)</mo>
    </math>"#;
    // set number preferences to European style
    test_braille_prefs("ASCIIMath-fi", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, 
                r"([a, b, c], [d, e, f])");
}
