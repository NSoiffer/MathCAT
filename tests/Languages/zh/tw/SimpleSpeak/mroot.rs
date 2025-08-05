use crate::common::*;

#[test]
fn msqrt_simple() {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x");
}

#[test]
fn neg_without_root() {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "負 x 減 y");
}

#[test]
fn msqrt() {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x 加 y 結束根號");
}

#[test]
fn mroot_as_square_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x");
}

#[test]
fn cube_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x 的 立方根");
}

#[test]
fn ordinal_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x 的 9 次方根");
}
#[test]
fn ordinal_root_2() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9.1</mn> </mroot>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x 的 9.1 次方根");
}

#[test]
fn simple_mi_root() {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x 的 n 次方根");
}


#[test]
fn simple_fraction_power() {
    let expr = "<math>
                    <mroot>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </mroot>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "根號 x 的 3 分之 1 次方根");
}
