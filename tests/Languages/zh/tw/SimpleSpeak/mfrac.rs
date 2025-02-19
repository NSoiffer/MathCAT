/// Tests for fractions
///   includes simple fractions and more complex fractions
///   also tests mixed fractions (implicit and explicit)
use crate::common::*;

#[test]
fn common_fraction_half() {
    let expr = "<math>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "2 分之 1");
}

#[test]
fn common_fraction_thirds() {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "3 分之 2");
}

#[test]
fn common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "10 分之 17");
}

#[test]
#[allow(non_snake_case)]
fn not_SimpleSpeak_common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "10 分之 89");
}

#[test]
fn non_simple_fraction() {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi><mo>+</mo><mi>y</mi> </mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("zh-tw", "SimpleSpeak", expr, "分數 x 減 y, 分之, x 加 y 結束分數");
}

#[test]
fn nested_fraction() {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi><mo>+</mo>  <mfrac><mn>1</mn><mi>y</mi></mfrac>  </mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("zh-tw", "SimpleSpeak", expr, "分數 x 減 y, 分之, x 加 分數 y 分之 1 結束分數; 結束分數");
}


#[test]
fn deeply_nested_fraction_msqrt() {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi><mo>+</mo>  <msqrt><mrow><mfrac><mn>1</mn><mi>y</mi></mfrac></mrow> </msqrt> </mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("zh-tw", "SimpleSpeak", expr, "分數 x 減 y, 分之, x 加 根號 y 分之 1 結束根號; 結束分數");
}

#[test]
fn deeply_nested_fraction_mrow_msqrt() {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi><mo>+</mo>  <msqrt><mrow><mn>2</mn><mo>+</mo><mfrac><mn>1</mn><mi>y</mi></mfrac></mrow> </msqrt> </mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("zh-tw", "SimpleSpeak", expr, "分數 x 減 y, 分之, x 加, 根號 2 加 y 分之 1 結束根號; 結束分數");
}

#[test]
fn numerator_simple_fraction() {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi></mrow>
        <mrow>
            <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("zh-tw", "SimpleSpeak", expr, "分數 x 減 y, 分之 x 結束分數");
}

#[test]
fn denominator_simple_fraction() {
    let expr = "
    <math>
        <mfrac>
            <mrow> <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
            <mrow> <mi>x</mi></mrow>
        </mfrac>
    </math>
                            ";
    test("zh-tw", "SimpleSpeak", expr, "分數 x 分之, x 減 y 結束分數");
}


#[test]
fn mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "3 又 2 分之 1");
}

#[test]
fn explicit_mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "3 又 8 分之 1");
}

#[test]
fn mixed_number_big() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "3 又 83 分之 7");
}

#[test]
fn simple_text() {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "run 分之 rise");
}

#[test]
fn number_and_text() {
    let expr = "<math>
            <mfrac>
            <mrow>
                <mn>2</mn><mtext>miles</mtext></mrow>
            <mrow>
                <mn>3</mn><mtext>gallons</mtext></mrow>
            </mfrac>
        </math>";
    test("zh-tw", "SimpleSpeak", expr, "分數 3 gallons, 分之, 2 miles 結束分數");
}


#[test]
fn nested_simple_fractions() {
    let expr = "<math>
                <mrow>
                <mfrac>
                    <mrow>
                    <mfrac>
                        <mn>1</mn>
                        <mn>2</mn>
                    </mfrac>
                    </mrow>
                    <mrow>
                    <mfrac>
                        <mn>2</mn>
                        <mn>3</mn>
                    </mfrac>
                    </mrow>
                </mfrac>
                </mrow>
            </math>";
    test("zh-tw", "SimpleSpeak", expr, "分數 分數 3 分之 2 結束分數; 分之, 分數 2 分之 1 結束分數; 結束分數");
}

#[test]
fn binomial() {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("zh-tw", "SimpleSpeak", expr, "2 乘 7 選 3");
}
