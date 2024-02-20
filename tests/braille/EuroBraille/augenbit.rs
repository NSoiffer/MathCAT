// Eurobraille tests for the basic mathml tags
//   https://augenbit.de/wiki/index.php?title=LaTeX-Manual_LaTeX_Grundregeln
use crate::common::*;


#[test]
fn keyboard_1 () {
    // this test was added because #220 (failed to add grouping around overscript)
    let expr = "<math><mn>2</mn><mo>+</mo><mi>x</mi><mo>=</mo><mn>5</mn></math>";
    test_braille("EuroBraille", expr, "");
}

#[test]
fn keyboard_2 () {
    // this test was added because #220 (failed to add grouping around overscript)
    let expr = "<math><mo>|</mo><mi>x</mi><mo>-</mo><mn>1</mn><mo>|</mo><mo>=</mo><mo>|</mo><mn>1</mn><mo>-</mo><mi>x</mi><mo>|</mo></math>";
    test_braille("EuroBraille", expr, "");
}

#[test]
fn keyboard_3 () {
    init_logger();
    // this test was added because #220 (failed to add grouping around overscript)
    let expr = "<math><mi>n</mi><mo>!</mo><mo>=</mo><mi>n</mi><mo>*</mo><mo>(</mo><mi>n</mi><mo>-</mo><mn>1</mn><mo>)</mo><mo>!</mo></math>";
    test_braille("EuroBraille", expr, "");
}

#[test]
fn script_1 () {
    // this test was added because #220 (failed to add grouping around overscript)
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup></math>";
    test_braille("EuroBraille", expr, "");
}

#[test]
fn script_2 () {
    // this test was added because #220 (failed to add grouping around overscript)
    let expr = "<math><msup><mi>x</mi><mn>10</mn></msup></math>";
    test_braille("EuroBraille", expr, "");
}

#[test]
fn script_3 () {
    // this test was added because #220 (failed to add grouping around overscript)
    let expr = "<math><msub><mi>a</mi><mn>1</mn></msub><mo>+</mo><msub><mi>a</mi><mi>n</mi></msub></math>";
    test_braille("EuroBraille", expr, "");
}
