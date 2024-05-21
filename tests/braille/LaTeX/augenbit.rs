// LaTeX tests for the basic mathml tags.
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
    test_braille("LaTeX", expr, r"2 +x =5");
}

#[test]
fn augenbit0_1_2 () {
    let expr = "<math><mo>|</mo><mi>x</mi><mo>-</mo><mn>1</mn><mo>|</mo><mo>=</mo><mo>|</mo><mn>1</mn><mo>-</mo><mi>x</mi><mo>|</mo></math>";
    test_braille("LaTeX", expr, r"| x -1| =| 1 -x|");
}

#[test]
fn augenbit0_1_3 () {
    let expr = "<math><mi>n</mi><mo>!</mo><mo>=</mo><mi>n</mi><mo>*</mo><mo>(</mo><mi>n</mi><mo>-</mo><mn>1</mn><mo>)</mo><mo>!</mo></math>";
    test_braille("LaTeX", expr, r"n! =n *(n -1)!");
}

#[test]
fn augenbit0_2_1 () {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup></math>";
    test_braille("LaTeX", expr, r"x^2");
}

#[test]
fn augenbit0_2_2 () {
    let expr = "<math><msup><mi>x</mi><mn>10</mn></msup></math>";
    test_braille("LaTeX", expr, r"x^{10}");
}

#[test]
fn augenbit0_2_3 () {
    let expr = "<math><msub><mi>a</mi><mn>1</mn></msub><mo>+</mo><msub><mi>a</mi><mi>n</mi></msub></math>";
    test_braille("LaTeX", expr, r"a_1 +a_n");
}

#[test]
fn augenbit0_2_4 () {
    let expr = "<math><msup><mi>x</mi><mrow><mi>n</mi><mo>+</mo><mi>m</mi></mrow></msup></math>";
    test_braille("LaTeX", expr, r"x^{n +m}");
}

#[test]
fn augenbit0_3_1 () {
    let expr = "<math><mi>n</mi><mo>→</mo><mo>∞</mo></math>";
    test_braille("LaTeX", expr, r"n \to \infty");
}

#[test]
fn augenbit0_3_2 () {
    let expr = "<math><mi>x</mi><mo>∉</mo><mo>{</mo><mn>3</mn><mo>;</mo><mn>4</mn><mo>}</mo></math>";
    test_braille("LaTeX", expr, r"x \notin \{3; 4\}");
}

#[test]
fn augenbit0_4_1 () {
    let expr = "<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mrow><mi>a</mi><mo>-</mo><mi>b</mi></mrow></mfrac></math>";
    test_braille("LaTeX", expr, r"\frac{a +b}{a -b}");
}

#[test]
fn augenbit0_4_2 () {
    let expr = "<math><msqrt><mi>a</mi><mo>+</mo><mi>b</mi></msqrt></math>";
    test_braille("LaTeX", expr, r"\sqrt{a +b}");
}

#[test]
fn augenbit0_4_3 () {
    let expr = r#"<math>
            <munderover><mo>&#x2211;</mo><mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow><mi mathvariant="normal">&#x221E;</mi></munderover>
            <mfrac><mn>1</mn><msup><mn>2</mn><mi>n</mi></msup></mfrac><mo>=</mo><mn>1</mn>
        </math>"#;
    test_braille("LaTeX", expr, r"\sum_{n =1}^\infty \frac{1}{2^n} =1");
}

#[test]
fn augenbit0_5_1 () {
    let expr = r#"<math><mover><mrow><mi>A</mi><mo>&#x222A;</mo><mi>B</mi></mrow><mo accent="true">&#x2015;</mo></mover></math>"#;
    test_braille("LaTeX", expr, r"\overline{A \cup B}");
}

#[test]
fn augenbit1_1_1 () {
    let expr = r#"<math><mo>{</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo>,</mo><mn>4</mn><mo>}</mo></math>"#;
    test_braille("LaTeX", expr, r"\{1, 2, 3, 4\}");
}

#[test]
fn augenbit1_2_3 () {
    let expr = r#"<math><msubsup><mi mathvariant="normal">&#x2124;</mi><mn>0</mn><mo>-</mo></msubsup></math>"#;
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "false" )], expr, r"\mathbb Z_0^-");
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "true")], expr, r"\Z_0^-");
}

#[test]
fn augenbit1_3_2 () {
    let expr = r#"<math><mn>9</mn><mo>&#x2212;</mo><mn>3</mn><mo>&#x2260;</mo><mn>5</mn></math>"#;
    test_braille("LaTeX", expr, r"9 -3 \not= 5");
}

#[test]
fn augenbit1_3_3 () {
    let expr = r#"<math><mi>x</mi><mo>&#xB1;</mo><mn>3</mn></math>"#;
    test_braille("LaTeX", expr, r"x \pm 3");
}

#[test]
fn augenbit1_3_6 () {
    let expr = r#"<math><mi>x</mi><mo>&#x2264;</mo><mn>10</mn></math>"#;
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "false" )], expr, r"x \le 10");
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "true")], expr, r"x <=10");
}

#[test]
fn augenbit1_3_10 () {
    let expr = r#"<math><mi>&#x3C0;</mi><mo>&#x2248;</mo><mn>3</mn><mo>,</mo><mn>14</mn></math>"#;
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "false"), ("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, r"\pi \approx 3,14");
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "true" ), ("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, r"~p \apx 3,14");
}

#[test]
fn augenbit1_3_14 () {
    let expr = r#"<math><mi>a</mi><mover><mo>=</mo><mo>^</mo></mover><mi>b</mi></math>"#;
    test_braille("LaTeX", expr, r"a \hat{=} b");
}

#[test]
fn augenbit1_5_2 () {
    let expr = r#"<math><mfrac><mn>1</mn><mi>x</mi></mfrac></math>"#;
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "false" )], expr, r"\frac{1}{x}");
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "true")], expr, r"\f{1}{x}");
}

#[test]
fn augenbit1_5_7 () {
    // put number together -- editor created "<mn>0</mn><mo>,</mo><mn>1</mn>" and canonicalize didn't fix it
    let expr = r#"<math><mrow>
            <mn>0,1</mn>
            <mover><mn>6</mn><mo accent="true">&#x2015;</mo></mover>
            <mo>=</mo>
            <mn>1</mn><mrow><mo>/</mo></mrow><mn>6</mn>
        </mrow></math>"#;
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "false" )], expr, r"0,1\overline{6} =1/6");
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "true")], expr, r"0,1\ol{6} =1/6");
}

#[test]
fn augenbit1_5_8 () {
    let expr = r#"<math><mn>75</mn><mo>%</mo><mo>=</mo><mn>3</mn><mo>/</mo><mn>4</mn></math>"#;
    test_braille("LaTeX", expr, r"75% =3/4");
}

#[test]
fn augenbit1_6_8 () {
    let expr = r#"<math>
            <mroot><msup><mi>a</mi><mn>2</mn></msup><mn>3</mn></mroot><mo>=</mo>
            <msup><mi>a</mi><mrow><mn>2</mn><mo>/</mo><mn>3</mn></mrow></msup>
        </math>"#;
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "false" )], expr, r"\sqrt[3]{a^2} =a^{2/3}");
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "true")], expr, r"\s[3]{a^2} =a^{2/3}");
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
    test_braille("LaTeX", expr, r"_{95}^{238}U");
}


#[test]
fn augenbit1_7_7 () {
    let expr = r#"<math><msub><mi>log</mi><mi>a</mi></msub><mi>x</mi></math>"#;
    test_braille("LaTeX", expr, r"\log_a x");
}

#[test]
fn augenbit1_7_10 () {
    let expr = r#"<math><msup><mi>cos</mi><mn>2</mn></msup><mi>&#x3B2;</mi></math>"#;
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "false" )], expr, r"\cos^2 \beta");
    test_braille_prefs("LaTeX", vec![("LaTeX_UseShortName", "true")], expr, r"cos^2 ~b");
}

#[test]
fn augenbit1_7_12 () {
    let expr = r#"<math><mi>cot</mi><mn>45</mn><mo>&#xB0;</mo></math>"#;
    test_braille("LaTeX", expr, r"\cot 45°");
}

#[test]
fn augenbit1_8_2 () {
    let expr = r#"<math><mi mathvariant="normal">&#x25B3;</mi><mi>A</mi><mi>B</mi><mi>C</mi></math>"#;
    test_braille("LaTeX", expr, r"\triangle ABC");
}

#[test]
fn augenbit1_8_4 () {
    let expr = r#"<math><mi>&#x3B1;</mi><mo>,</mo><mi>&#x3B2;</mi><mo>,</mo><mi>&#x3B3;</mi><mo>,</mo><mi>&#x3B4;</mi><mo>,</mo><mi>&#x3F5;</mi></math>"#;
    test_braille("LaTeX", expr, r"\alpha, \beta, \gamma, \delta, \epsilon");
}

#[test]
fn augenbit2_1_3 () {
    // original display code contains forced spaces not in the output -- they are cleaned up here
    let expr = r#"<math >
        <munder>
        <mo movablelimits="true">lim</mo>
        <mrow>
            <mi>x</mi>
            <mo accent="false" stretchy="false">&#x2192;</mo>
            <msub><mi>x</mi><mn>0</mn></msub>
        </mrow>
        </munder>
    </math>"#;
    test_braille("LaTeX", expr, r"\lim_{x \to x_0}");
}

#[test]
fn augenbit2_1_4 () {
    // original display code contains forced spaces not in the output -- they are cleaned up here
    let expr = r#"<math>
            <msup><mi>f</mi><mo>'</mo> </msup><mo>(</mo><mi>x</mi><mo>)</mo><mo>,</mo>
            <msup><mi>f</mi><mo>''</mo></msup><mo>(</mo><mi>x</mi><mo>)</mo>
        </math>"#;
    test_braille("LaTeX", expr, r"f'(x), f''(x)");
}

#[test]
fn augenbit2_2_2 () {
    // original display code contains forced spaces not in the output -- they are cleaned up here
    let expr = r#"<math>
        <mo minsize="2.047em" maxsize="2.047em">(</mo>
        <mfrac linethickness="0"><mi>n</mi><mi>k</mi></mfrac>
        <mo minsize="2.047em" maxsize="2.047em">)</mo>
    </math>"#;
    test_braille("LaTeX", expr, r"\binom{n}{k}");
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
    test_braille_prefs("LaTeX", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, 
                r"\vec{q} = \begin{pmatrix} -5 \\ 0,5 \\ k +4 \end{pmatrix}");
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
    test_braille_prefs("LaTeX", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, 
                r"\begin{pmatrix} a & b & c \\ d & e & f \end{pmatrix}");
}
