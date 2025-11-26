use crate::common::*;

#[test]
fn msqrt_simple() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("de", "ClearSpeak", expr, "die quadratwurzel von x");
}

#[test]
fn msqrt_simple_end_root() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("de", "ClearSpeak_Roots", "RootEnd", expr, "die quadratwurzel von x, ende der wurzel");
}

#[test]
fn msqrt_simple_positive() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("de", "ClearSpeak_Roots", "PosNegSqRoot", expr, "die plus quadratwurzel von x");
}

#[test]
fn msqrt_simple_pos_end_root() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("de", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "die plus quadratwurzel von x, ende der wurzel");
}

#[test]
fn msqrt_simple_pos_end_with_neg_root() {
    let expr = "<math>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                </math>";
    test_ClearSpeak("de", "ClearSpeak_Roots", "PosNegSqRootEnd", expr,
    "die minus quadratwurzel von x, ende der wurzel; minus, die positive kubikwurzel von x, ende der wurzel");
}

#[test]
fn mroot_simple_pos_end_with_neg_root() {
    let expr = "<math>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>

                </math>";
    test_ClearSpeak("de", "ClearSpeak_Roots", "PosNegSqRoot", expr,
    "die negative kubikwurzel von x; minus die plus quadratwurzel von x");
}

#[test]
fn neg_without_root() {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("de", "ClearSpeak", expr, "negative x minus y");
}

#[test]
fn msqrt() {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("de", "ClearSpeak", expr, "die quadratwurzel von x plus y");
}

#[test]
fn mroot_as_square_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("de", "ClearSpeak", expr, "die quadratwurzel von x");
}

#[test]
fn cube_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("de", "ClearSpeak", expr, "die kubikwurzel von x");
}

#[test]
fn ordinal_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("de", "ClearSpeak", expr, "die neunte Wurzel von x");
}


/* // should have n-te wurze
#[test]
fn simple_mi_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("de", "ClearSpeak", expr, "die n-th wurzel von x");
}

#[test]
fn mroot_simple_pos_end_root() {
    let expr = "<math>
                <mroot> <mi>x</mi> <mi>t</mi> </mroot>
                </math>";
    test_ClearSpeak("de", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "the positive t-th root of x, end root");
}

 */

#[test]
fn mroot_simple_end_root() {
    let expr = "<math>
                    <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                    <mn>21</mn></mroot>
                </math>";
    test_ClearSpeak("de", "ClearSpeak_Roots", "RootEnd", expr, "die zwanzig erste Wurzel von x plus y, ende der wurzel");
}

#[test]
fn simple_fraction_power() {
    let expr = "<math>
                    <mroot>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </mroot>
                </math>";
    test("de", "ClearSpeak", expr, "die 1 drittel wurzel von x");
}
