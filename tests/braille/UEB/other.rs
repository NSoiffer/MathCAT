// UEB tests for the basic mathml tags
// These are additional tests from other sources
// Those labeled with "aph_" come from the APH lessons:
//   https://uebmath.aphtech.org/
use crate::common::*;


// Extra tests targeted at special cases in MathCAT

#[test]
fn word_symbol_aph_6_7_5() {
    // this test was added because ≟ (U+225F) uses a 'G1 Word mode' char, so is different than others
    let expr = "<math><mn>4</mn><mo>+</mo><mn>5</mn><mo>≟</mo><mn>12</mn></math>";
    test_braille("UEB", expr, "⠼⠙⠐⠖⠼⠑⠀⠰⠰⠦⠻⠐⠶⠀⠼⠁⠃");
}

#[test]
fn blank_aph_7_1_ex5() {
    // this test was added because #153 (both the blank and spaces around "::")
    let expr = "<math><mn>3</mn><mo>:</mo><mn>15</mn> <mo>::</mo> <mn>60</mn><mo>:</mo><mo>_</mo></math>";
    test_braille("UEB", expr, "⠼⠉⠒⠼⠁⠑⠀⠒⠒⠀⠼⠋⠚⠒⠨⠤");
}

// Extra tests targeted at special cases in MathCAT
#[test]
fn number_space_before() {
    let expr = "<math><mtext>&#xA0;</mtext><mn>2</mn></math>";
    test_braille("UEB", expr, "⠀⠼⠃");
}

#[test]
fn number_space_after() {
    let expr = "<math><mn>2</mn><mtext>&#xA0;</mtext></math>";
    test_braille("UEB", expr, "⠼⠃⠀");
}

#[test]
fn number_space_before_and_after() {
    let expr = "<math><mtext>&#xA0;</mtext><mn>2</mn><mtext>&#xA0;</mtext></math>";
    test_braille("UEB", expr, "⠀⠼⠃⠀");
}

#[test]
fn forced_g1_symbol_mode() {
    // A forced G1 should not count against starting in G2 mode
    // This is issue #170 -- surprisingly not tested by other tests 
    let expr = "<math><mo>∫</mo><mn>3</mn><msup><mi>x</mi><mn>2</mn></msup><mi>dx</mi></math>";
    test_braille("UEB", expr, "⠰⠮⠼⠉⠭⠔⠼⠃⠰⠙⠭");
}

// extra tests targeted at contractions based on function names
#[test]
fn contractions_1() {
    let expr = "<math>
        <mi>sech</mi><mo>&#x2061;</mo><mi>x</mi><mo>+</mo>
        <mi>cosh</mi><mo>&#x2061;</mo><mi>y</mi><mo>+</mo>
        <mi>arccos</mi><mo>&#x2061;</mo><mi>t</mi>
    </math>";
    // Note: "arccos" does not use the "cc" contraction -- RUEB 10.6.5 lists "arccosine" without the contraction
    test_braille("UEB", expr, "⠎⠑⠡⠀⠭⠐⠖⠉⠕⠩⠀⠽⠐⠖⠜⠉⠉⠕⠎⠀⠰⠞");
}
#[test]
fn contractions_2() {
    let expr = "<math><mi>ker</mi><mo>&#x2061;</mo><mi>h</mi></math>";
    test_braille("UEB", expr, "⠅⠻⠀⠰⠓");
}

#[test]
fn contractions_3() {
    let expr = "<math><mi>argument</mi><mo>&#x2061;</mo><mo>(</mo><mi>f</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠜⠛⠥⠰⠞⠐⠣⠋⠐⠜");
}

#[test]
fn contractions_4() {
    let expr = "<math><mtext>error&#xA0;function&#xA0;</mtext><mi>erf</mi></math>";
    test_braille("UEB", expr, "⠻⠗⠕⠗⠀⠋⠥⠝⠉⠰⠝⠀⠻⠋");
}

#[test]
fn contractions_5() {
    let expr = "<math><mi>Real</mi><mo>(</mo><mi>z</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠠⠗⠂⠇⠐⠣⠵⠐⠜");
}
