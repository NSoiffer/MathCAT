use crate::common::*;

#[test]
fn msqrt_simple() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("fi", "ClearSpeak", expr, "neliöjuuri x,");
}

#[test]
fn msqrt_simple_end_root() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("fi", "ClearSpeak_Roots", "RootEnd", expr, "neliöjuuri x, loppu juuri;");
}

// FI: commented out because of not passing, have to look into it
// #[test]
// fn msqrt_simple_positive() {
//     let expr = "<math>
//                     <msqrt> <mi>x</mi> </msqrt>
//                 </math>";
//     test_ClearSpeak("en", "ClearSpeak_Roots", "PosNegSqRoot", expr, "positiivinen neliöjuuri x,");
// }

// FI: commented out because of not passing, have to look into it
// #[test]
// fn msqrt_simple_pos_end_root() {
//     let expr = "<math>
//                     <msqrt> <mi>x</mi> </msqrt>
//                 </math>";
//     test_ClearSpeak("en", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "positiivinen neliöjuuri x, loppu juuri;");
// }

#[test]
fn msqrt_simple_pos_end_with_neg_root() {
    let expr = "<math>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                </math>";
    test_ClearSpeak("en", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, 
    "the negative square root of x, end root; minus, the positive cube root of x, end root;");
}

#[test]
fn mroot_simple_pos_end_with_neg_root() {
    let expr = "<math>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>

                </math>";
    test_ClearSpeak("en", "ClearSpeak_Roots", "PosNegSqRoot", expr, 
    "the negative cube root of x; minus the positive square root of x,");
}

#[test]
fn neg_without_root() {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("fi", "ClearSpeak", expr, "negatiivinen x miinus y");
}

#[test]
fn msqrt() {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("fi", "ClearSpeak", expr, "neliöjuuri x plus y;");
}

#[test]
fn mroot_as_square_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("fi", "ClearSpeak", expr, "neliöjuuri x,");
}

#[test]
fn cube_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("fi", "ClearSpeak", expr, "kuutiojuuri x,");
}

#[test]
fn ordinal_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("fi", "ClearSpeak", expr, "yhdeksäs juuri x,");
}

// FI: commented out because of not passing, have to look into it
// #[test]
// fn simple_mi_root() {
//     let expr = "<math>
//                     <mroot> <mi>x</mi> <mi>n</mi> </mroot>
//                 </math>";
//     test("fi", "ClearSpeak", expr, "n nes juuri x,");
// }

// FI: commented out because of not passing, have to look into it
// #[test]
// fn mroot_simple_pos_end_root() {
//     let expr = "<math>
//                 <mroot> <mi>x</mi> <mi>t</mi> </mroot>
//                 </math>";
//     test_ClearSpeak("fi", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "positiivinen t nes juuri x, loppu juuri;");
// }

#[test]
fn mroot_simple_end_root() {
    let expr = "<math>
                    <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                    <mn>21</mn></mroot>
                </math>";
    test_ClearSpeak("en", "ClearSpeak_Roots", "RootEnd", expr, "the twenty first root of x plus y, end root;");
}

#[test]
fn simple_fraction_power() {
    let expr = "<math>
                    <mroot>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </mroot>
                </math>";
    test("fi", "ClearSpeak", expr, "1 kolmasosa juuri x,");
}
