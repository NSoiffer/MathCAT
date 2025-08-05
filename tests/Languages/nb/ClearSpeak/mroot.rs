use crate::common::*;

#[test]
fn msqrt_simple() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("nb", "ClearSpeak", expr, "kvadratroten av x");
}

#[test]
fn msqrt_simple_end_root() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("nb", "ClearSpeak_Roots", "RootEnd", expr, "kvadratroten av x, slutt rot");
}

#[test]
fn msqrt_simple_positive() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("nb", "ClearSpeak_Roots", "PosNegSqRoot", expr, "den positive kvadratroten av x");
}

#[test]
fn msqrt_simple_pos_end_root() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("nb", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "den positive kvadratroten av x, slutt rot");
}

#[test]
fn msqrt_simple_pos_end_with_neg_root() {
    let expr = "<math>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                </math>";
    test_ClearSpeak("nb", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, 
    "den negative kvadratroten av x, slutt rot; minus, den positive kubikkroten av x, slutt rot");
}

#[test]
fn mroot_simple_pos_end_with_neg_root() {
    let expr = "<math>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>

                </math>";
    test_ClearSpeak("nb", "ClearSpeak_Roots", "PosNegSqRoot", expr, 
    "den negative kubikkroten av x; minus den positive kvadratroten av x");
}

#[test]
fn neg_without_root() {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("nb", "ClearSpeak", expr, "minus x minus y");
}

#[test]
fn msqrt() {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("nb", "ClearSpeak", expr, "kvadratroten av x pluss y");
}

#[test]
fn mroot_as_square_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("nb", "ClearSpeak", expr, "kvadratroten av x");
}

#[test]
fn cube_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("nb", "ClearSpeak", expr, "kubikkroten av x");
}

#[test]
fn ordinal_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("nb", "ClearSpeak", expr, "niende roten av x");
}

#[test]
fn simple_mi_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("nb", "ClearSpeak", expr, "roten av grad n, av x");
}

#[test]
fn mroot_simple_pos_end_root() {
    let expr = "<math>
                <mroot> <mi>x</mi> <mi>t</mi> </mroot>
                </math>";
    test_ClearSpeak("nb", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "den positive roten av grad t, av x, slutt rot");
}

#[test]
fn mroot_simple_end_root() {
    let expr = "<math>
                    <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                    <mn>21</mn></mroot>
                </math>";
    test_ClearSpeak("nb", "ClearSpeak_Roots", "RootEnd", expr, "tjue første roten av x pluss y, slutt rot");
}

#[test]
fn simple_fraction_power() {
    let expr = "<math>
                    <mroot>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </mroot>
                </math>";
    test("nb", "ClearSpeak", expr, "roten av grad 1 tredjedel, av x");
}
