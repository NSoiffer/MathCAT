// UEB tests for the basic mathml tags
// Initial tests are from BANA guidelines, mostly about initial chars for code switching
//   http://www.brailleauthority.org/ueb/ueb_math_guidance/final_for_posting_ueb_math_guidance_may_2019_102419.pdf
// These tests start with "bana_"
//
// Many come from (refer to) https://iceb.org/guidelines_for_technical_material_2014.pdf
// For example, "fraction_6_1_1" is a fraction example from section 6.1, and is the first example there.
use crate::common::*;

#[test]
fn calculation_marks_1() {
    let expr = "<math><mn>3</mn><mo>+</mo><mn>4</mn><mo>=</mo><mn>7</mn></math>";
    test_braille("Finnish", expr, "⠼⠉⠀⠖⠼⠙⠀⠶⠼⠛");
}

#[test]
fn calculation_marks_7() {
    let expr = "<math><mrow><mi>&#x03C0;</mi><mo>&#x2248;</mo><mn>3,14</mn></mrow></math>";
    test_braille("Finnish", expr, "⠨⠏⠀⠸⠶⠼⠉⠂⠁⠙");
}

#[test]
fn fraction_13() {
    let expr = "<math><mfrac><mn>5</mn><mn>4</mn></mfrac><mo>=</mo><mn>1</mn><mfrac><mn>1</mn><mn>4</mn></mfrac></math>";
    test_braille("Finnish", expr, "⠼⠑⠲⠀⠶⠼⠁⠼⠁⠲");
}

#[test]
fn fraction_equations_3() {
    let expr = "<math>
        <mfrac>
            <mrow><mn>4</mn><mi>x</mi></mrow>
            <mrow><mn>6</mn><mo>(</mo><mn>1</mn><mo>&#x2212;</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac>
    </math>";
    test_braille("Finnish", expr, "⠼⠙⠀⠭⠀⠌⠦⠼⠋⠀⠦⠼⠁⠀⠤⠭⠴⠴");
}

#[test]
fn powers_3() {
    let expr = "<math><msup><mn>2</mn><mn>3</mn></msup><mo>+</mo><mn>5</mn></math>";
    test_braille("Finnish", expr, "⠼⠃⠬⠼⠉⠀⠖⠼⠑");
}

#[test]
fn roots_5() {
    let expr = "<math><msup><mn>27</mn><mfrac><mn>1</mn><mn>3</mn></mfrac></msup><mo>=</mo>
                        <mroot><mn>27</mn><mn>3</mn></mroot><mo>=</mo><mn>3</mn></math>";
    test_braille("Finnish", expr, "⠼⠃⠛⠬⠼⠁⠒⠀⠶⠩⠼⠉⠐⠼⠃⠛⠀⠶⠼⠉");
}

#[test]
fn roots_7() {
    let expr = "<math><msqrt><mn>20</mn><mo>+</mo><mn>5</mn></msqrt><mo>=</mo><msqrt><mn>25</mn></msqrt><mo>=</mo><mn>5</mn></math>";
    test_braille("Finnish", expr, "⠩⠦⠼⠃⠚⠀⠖⠼⠑⠀⠴⠀⠶⠩⠼⠃⠑⠀⠶⠼⠑");
}

#[test]
fn vectors_1() {
    let expr = "<math><mover><mi>a</mi><mo>&#xAF;</mo></mover></math>";
    test_braille("Finnish", expr, "⠁⠱");
}
