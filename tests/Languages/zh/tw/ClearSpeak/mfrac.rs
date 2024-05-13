/// Tests for fractions
///   includes simple fractions and more complex fractions
///   also tests mixed fractions (implicit and explicit)
use crate::common::*;

#[test]
fn common_fraction_half() {
    let expr = "<math>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("zh-tw", "ClearSpeak", expr, "2 分之 1");
}

#[test]
fn common_fraction_thirds() {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("zh-tw", "ClearSpeak", expr, "3 分之 2");
}

#[test]
fn common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "10 分之 17");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "10 分之 17");
}

#[test]
#[allow(non_snake_case)]
fn not_ClearSpeak_common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "10 分之 89");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "10 分之 89");
}

#[test]
fn non_simple_fraction() {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow>
        <mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>";
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "分數分子; x 加 y; 分母 x 減 y;");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "分數分子; x 加 y; 分母 x 減 y;");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Over")], expr, "x 減 y 分之 x 加 y");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "FracOver")], expr, "分數 x 減 y 分之 x 加 y");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "General")], expr, "分數分子; x 加 y; 分母 x 減 y;");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "EndFrac")], expr, "分數分子; x 加 y; 分母 x 減 y; 結束分數,");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "GeneralEndFrac")], expr, "分數分子; x 加 y; 分母 x 減 y; 結束分數,");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "OverEndFrac")], expr, "x 減 y 分之 x 加 y, 結束分數,");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Per")], expr, "x 加 y 每 x 減 y");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Verbose"),("ClearSpeak_Fractions", "Auto")], expr, "分數分子; x 加 y; 分母 x 減 y; 結束分數,");
}


#[test]
fn mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("zh-tw", "ClearSpeak", expr, "3 又 2 分之 1");
}

#[test]
fn explicit_mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("zh-tw", "ClearSpeak", expr, "3 又 8 分之 1");
}

#[test]
fn mixed_number_big() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("zh-tw", "ClearSpeak", expr, "3 又 83 分之 7");
}

#[test]
fn simple_text() {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("zh-tw", "ClearSpeak", expr, "run 分之 rise");
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
    test("zh-tw", "ClearSpeak", expr, "3 gallons 分之 2 miles");
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
    test_prefs("zh-tw", "ClearSpeak", vec![("ClearSpeak_Fractions", "Auto")], expr, "1 half over 2 thirds");
    test_prefs("zh-tw", "ClearSpeak", vec![("ClearSpeak_Fractions", "Ordinal")], expr, "1 half over 2 thirds");
    test_prefs("zh-tw", "ClearSpeak", vec![("ClearSpeak_Fractions", "Over")], expr, "1 over 2 over 2 over 3");
    test_prefs("zh-tw", "ClearSpeak", vec![("ClearSpeak_Fractions", "FracOver")], expr,
            "the fraction the fraction 1 over 2 over the fraction 2 over 3");
    test_prefs("zh-tw", "ClearSpeak", vec![("ClearSpeak_Fractions", "General")], expr,
            "the fraction with numerator the fraction with numerator 1; and denominator 2; and denominator the fraction with numerator 2; and denominator 3;");
    test_prefs("zh-tw", "ClearSpeak", vec![("ClearSpeak_Fractions", "EndFrac")], expr, "1 half over 2 thirds");
    test_prefs("zh-tw", "ClearSpeak", vec![("ClearSpeak_Fractions", "GeneralEndFrac")], expr,
            "the fraction with numerator the fraction with numerator 1; and denominator 2; end fraction; and denominator the fraction with numerator 2; and denominator 3; end fraction; end fraction,");
    test_prefs("zh-tw", "ClearSpeak", vec![("ClearSpeak_Fractions", "OverEndFrac")], expr,
            "1 over 2, end fraction, over 2 over 3, end fraction; end fraction,");
}


#[test]
fn semi_nested_fraction() {
    let expr = "<math>
                <mrow>
                        <mfrac>
                        <mrow>
                        <mfrac>
                        <mn>2</mn>
                        <mn>3</mn>
                        </mfrac>
                        <mi>x</mi>
                    </mrow>
                    <mn>6</mn>
                    </mfrac>
                </mrow>
                </math>";
    test("zh-tw", "ClearSpeak", expr, "2 thirds x over 6");
}

#[test]
fn general_nested_fraction() {
    let expr = "
    <math>
    <mrow>
        <mfrac>
        <mrow>
        <mfrac>
            <mn>10</mn>
            <mi>n</mi>
        </mfrac>
        </mrow>
        <mrow>
        <mfrac>
        <mn>2</mn>
        <mi>n</mi>
        </mfrac>
        </mrow>
        </mfrac>
        </mrow>
    </math>
                    ";
    test("zh-tw", "ClearSpeak", expr, "the fraction with numerator; 10 over n; and denominator 2 over n;");
}

#[test]
fn complex_nested_fraction() {
    let expr = "
    <math>
    <mrow>
        <mfrac>
        <mrow>
        <mfrac>
            <mrow> <mi>n</mi> <mo>+</mo> <mn>10</mn> </mrow>
            <mi>n</mi>
        </mfrac>
        </mrow>
        <mrow>
        <mfrac>
        <mn>2</mn>
        <mi>n</mi>
        </mfrac>
        </mrow>
        </mfrac>
        </mrow>
    </math>
                    ";
    test("zh-tw", "ClearSpeak", expr, "the fraction with numerator; the fraction with numerator; n plus 10; and denominator n; and denominator 2 over n;");
}

#[test]
fn simple_function() {
    let expr = "<math><mfrac><mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mn>2</mn></mfrac></math>";
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f of x over 2");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f of x over 2, end fraction,");
}

#[test]
fn function_over_function() {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f of x over g of x");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f of x over g of x, end fraction,");
}

#[test]
fn non_simple_function_over_function() {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr,
             "the fraction with numerator; f of, open paren x plus 1, close paren; and denominator g of x;");
    test_prefs("zh-tw", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr,
             "the fraction with numerator; f of, open paren x plus 1, close paren; and denominator g of x; end fraction,");
}

#[test]
fn binomial() {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("zh-tw", "ClearSpeak", expr, "2 times 7 choose 3");
}
