// These are additional Nemeth tests from other sources, mainly from bugs/issues
use crate::common::*;


#[test]
fn not_number_space_blocks() {
    // https://github.com/NSoiffer/MathCAT/issues/144
    let expr = "<math><mn>123</mn><mtext>&nbsp;&#x2063;</mtext><mn>456</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠆⠒⠀⠼⠲⠢⠖");
}

#[test]
fn space_between_digits() {
    // https://github.com/NSoiffer/MathCAT/issues/144
    let expr = "<math><mn>1</mn><mtext>&#x00a0;</mtext><mn>3</mn><mtext>&#x00a0;</mtext><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠀⠼⠒⠀⠼⠢");
}

#[test]
fn space_hack_between_digits() {
    // https://github.com/NSoiffer/MathCAT/issues/144
    let expr = "<math><mn>1</mn><mtext>&#x00a0;&#x2063;</mtext><mn>3</mn><mtext>&#x00a0;&#x2063;</mtext><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠀⠼⠒⠀⠼⠢");
}

#[test]
fn tilde_prefix_bug_244() {
    // https://github.com/NSoiffer/MathCAT/issues/244
    let expr = "<math> <mo>~</mo> <mi>p</mi> </math>";
    test_braille("Nemeth", expr, "⠈⠱⠏");
}

#[test]
fn double_struck_bug_334() {
    // https://github.com/NSoiffer/MathCAT/issues/334 -- double struck was problem (⠼ was missing); test all of the scripted numbers here
    let expr = "<math><mn>𝟙</mn><mo>,</mo><mn>𝟐</mn><mo>,</mo><mn>𝟯</mn><mo>,</mo><mn>𝟺</mn></math>";
    test_braille("Nemeth", expr, "⠈⠼⠂⠠⠀⠸⠼⠆⠠⠀⠈⠰⠸⠼⠒⠠⠀⠼⠲");
}

