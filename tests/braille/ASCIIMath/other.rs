// Other tests for LaTeX generation
use crate::common::*;


#[test]
fn menclose_strikes () {
    let expr = r#"<math><menclose notation='updiagonalstrike downdiagonalstrike verticalstrike horizontalstrike'>
        <mi>x</mi>
    </menclose></math>"#;
    test_braille("ASCIIMath", expr, r"\cancel(x)");
}

#[test]
fn menclose_box_and_circle () {
    let expr = r#"<math><menclose notation='box circle'>
        <mi>x</mi>
    </menclose></math>"#;
    // box and circle aren't part of ASCIIMath
    test_braille("ASCIIMath", expr, r"|overline(underline(x))|");
}

#[test]
fn menclose_sides () {
    let expr = r#"<math>
        <menclose notation='left right '><mi>x</mi> </menclose>
        <menclose notation='top bottom'><mi>x</mi> </menclose>
    </math>"#;
    test_braille("ASCIIMath", expr, r"|x|overline(underline(x))");
}

#[test]
fn menclose_all_sides() {
    let expr = r#"<math><menclose notation='left right top bottom'>
        <mi>x</mi>
    </menclose></math>"#;
    test_braille("ASCIIMath", expr, r"|overline(underline(x))|");
}

#[test]
fn menclose_diagonal_arrows() {
    let expr = r#"<math><menclose notation='northeastarrow southeastarrow southwestarrow northwestarrow'>
        <mi>x</mi>
    </menclose></math>"#;
    // arrows aren't part of ASCIIMath
    test_braille("ASCIIMath", expr, r"x");
}

#[test]
fn menclose_double_arrows() {
    // extra spaces are deliberate -- they shouldn't make a difference
    let expr = r#"<math><menclose notation='updownarrow leftrightarrow northeastsouthwestarrow  northwestsoutheastarrow '>
        <mi>x</mi>
    </menclose></math>"#;
    // arrows aren't part of ASCIIMath
    test_braille("ASCIIMath", expr, r"x");
}

#[test]
fn menclose_horiz_and_vert_arrows() {
    let expr = r#"<math><menclose notation='uparrow downarrow leftarrow rightarrow'>
        <mi>x</mi>
    </menclose></math>"#;
    // arrows aren't part of ASCIIMath
    test_braille("ASCIIMath", expr, r"x");
}


#[test]
fn char_test() {
    let expr = r#"<math>
        <mstyle displaystyle="true">
            <mstyle mathvariant="bold"><mi>A</mi></mstyle><mo>,</mo>
            <mstyle mathvariant="double-struck"><mi>A</mi></mstyle><mo>,</mo>
            <mstyle mathvariant="double-struck"><mi>C</mi></mstyle><mo>,</mo>
            <mstyle mathvariant="script"><mi>l</mi></mstyle><mo>,</mo>
            <mstyle mathvariant="script"><mi>P</mi></mstyle><mo>,</mo>
            <mstyle mathvariant="monospace"><mi>X</mi></mstyle><mo>,</mo>
            <mstyle mathvariant="fraktur"><mi>H</mi></mstyle><mo>,</mo>
            <mstyle mathvariant="fraktur"><mi>z</mi></mstyle><mo>,</mo>
            <mstyle mathvariant="sans-serif"><mi>t</mi></mstyle><mo>,</mo>
            <mi>x</mi><mo>,</mo><mi>h</mi>
        </mstyle>
    </math>"#;
    test_braille("ASCIIMath", expr, r"bb A, bbb A, CC, cc l, cc P, tt X, fr H, fr z, sf t, x, h");
}

#[test]
fn quadratic_formula() {
    let expr = r#"<math>
        <mi>x</mi>
        <mo>=</mo>
        <mfrac>
        <mrow>
            <mo>-</mo>
            <mi>b</mi>
            <mo>&#xB1;</mo>
            <msqrt><mrow><msup><mi>b</mi><mn>2</mn></msup><mo>-</mo><mn>4</mn><mi>a</mi><mi>c</mi></mrow></msqrt>
        </mrow>
        <mrow><mn>2</mn><mi>a</mi></mrow>
        </mfrac>
    </math>"#;
    test_braille("ASCIIMath", expr, r"x = (-b+-sqrt(b^2-4ac))/(2a)");
}

#[test]
fn porportional() {
    let expr = r#"<math>
        <mi>a</mi>
        <mo>&#x221D;</mo>
        <mi>b</mi>
    </math>"#;
    test_braille("ASCIIMath", expr, r"a prop b");
}

#[test]
fn bracketing() {
    let expr = r#"<math>
        <mi>x</mi>
        <mo>=</mo>
        <mrow>
        <mo>&#x2329;</mo>
        <mrow>
            <mo>&#x230A;</mo>
            <mrow><mo>|</mo><mrow><mi>a</mi><mo>-</mo><mi>b</mi></mrow><mo>|</mo></mrow>
            <mo>&#x230B;</mo>
        </mrow>
        <mo>&#x232A;</mo>
        </mrow>
    </math>"#;
    test_braille("ASCIIMath", expr, r"x = <<|__|a-b| __|>>");
}

#[test]
fn dydx() {
    let expr = r#"<math>
        <mi>f</mi>
        <mo>&#x2032;</mo>
        <mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        <mo>=</mo>
        <mfrac>
        <mrow><mi>d</mi><mi>y</mi></mrow>
        <mrow><mi>d</mi><mi>x</mi></mrow>
        </mfrac>
    </math>"#;
    test_braille("ASCIIMath", expr, r"f'(x) = dy/dx");
}

#[test]
fn dqdp() {
    let expr = r#"<math>
        <mfrac>
        <mrow><mi>d</mi><mi>q</mi></mrow>
        <mrow><mi>d</mi><mi>p</mi></mrow>
        </mfrac>
    </math>"#;
    test_braille("ASCIIMath", expr, r"(dq)/(dp)");
}

#[test]
fn norm() {
    let expr = r#"<math>
        <mo>&#x2225;</mo>
        <mover><mi>x</mi><mo stretchy="false">&#x2192;</mo></mover>
        <mo>&#x2225;</mo>
    </math>"#;
    test_braille("ASCIIMath", expr, r"norm(vec x)");
}

#[test]
fn above_and_below() {
    let expr = r#"<math>
        <munder>
        <mrow><mn>1</mn><mo>+</mo><mn>2</mn></mrow>
        <mo>&#x23DF;</mo>
        </munder>
        <mo>,</mo>
        <mover>
        <mrow><mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        <mo>&#x23DE;</mo>
        </mover>
        <mo>,</mo>
        <munder>
        <mrow><mo>&#x2192;</mo></mrow>
        <mrow><mi>x</mi><mo>&#x2192;</mo><mi>y</mi></mrow>
        </munder>
        <mo>,</mo>
        <mover>
        <mo>&#x21D0;</mo>
        <mrow><mi>x</mi><mo>&#x2192;</mo><mi>y</mi></mrow>
        </mover>
    </math>"#;
    test_braille("ASCIIMath", expr, r"ubrace(1+2), obrace(x-y), underset(x->y)(->), overset(x->y)(lArr)");
}

#[test]
fn dots_and_bars() {
    let expr = r#"<math>
        <mover><mi>x</mi><mo>^</mo></mover>
        <mo>,</mo>
        <mover><mi>x</mi><mo>&#xAF;</mo></mover>
        <mo>,</mo>
        <munder><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>&#x332;</mo></munder>
        <mo>,</mo>
        <mover><mstyle mathvariant="bold"><mi>x</mi></mstyle><mo>&#x2192;</mo></mover>
        <mo>,</mo>
        <mover><mi>x</mi><mo>.</mo></mover>
        <mo>,</mo>
        <mover><mi>y</mi><mo>..</mo></mover>
    </math>"#;
    test_braille("ASCIIMath", expr, r"hat x, bar x, ul(x+y), vec bb x, dot x, ddot y");
}

#[test]
fn backslash() {
    let expr = r#"<math>
        <mi>a</mi>
        <mo>\</mo>
        <mi>b</mi>
    </math>"#;
    test_braille("ASCIIMath", expr, r"a\\b");
}

#[test]
fn text_and_space() {
    let expr = r#"<math>
        <mi>a</mi>
        <mo>&#xA0;</mo>
        <mrow>
        <mtext>some text</mtext>
        </mrow>
        <mo>&#xA0;</mo>
        <mi>b</mi>
    </math>"#;
    test_braille("ASCIIMath", expr, r#"a\ "some text"\ b"#);
}

#[test]
fn mixed_fraction() {
    let expr = r#"<math><mn>2</mn><mfrac><mi>1</mi><mn>2</mn></mfrac></math>"#;
    test_braille("ASCIIMath", expr, r#"2 1/2"#);
}
