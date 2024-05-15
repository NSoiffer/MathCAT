/// Tests for fractions
///   includes simple fractions and more complex fractions
///   also tests mixed fractions (implicit and explicit)
use crate::common::*;

#[test]
fn common_fraction_half() {
    let expr = "<math>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("sv", "ClearSpeak", expr, ", en halv");
}

#[test]
fn common_fraction_thirds() {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("sv", "ClearSpeak", expr, ", 2 tredjedelar");
}

#[test]
fn common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "17 tiondelar");
///    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "17 tenths");
}

#[test]
#[allow(non_snake_case)]
fn not_ClearSpeak_common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "89 genom 10,");
///    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "89 tenths");
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
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "division med täljaren; x plus y; och nämnaren x minus y;");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "division med täljaren; x plus y; och nämnaren x minus y;");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Over")], expr, ", division, x plus y genom x minus y, slut division,");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "FracOver")], expr, "the fraction x plus y over x minus y");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "General")], expr, "division med täljaren; x plus y; och nämnaren x minus y;");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "EndFrac")], expr, "the fraction with numerator; x plus y; and denominator x minus y; end fraction,");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "GeneralEndFrac")], expr, "the fraction with numerator; x plus y; and denominator x minus y; end fraction,");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "OverEndFrac")], expr, "x plus y over x minus y, end fraction,");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Per")], expr, "x plus y per x minus y");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Verbose"),("ClearSpeak_Fractions", "Auto")], expr, "the fraction with numerator; x plus y; and denominator x minus y; end fraction,");
}


#[test]
fn mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("sv", "ClearSpeak", expr, "3 och en halv");
}

#[test]
fn explicit_mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("sv", "ClearSpeak", expr, "3 och, en åttondel");
}

#[test]
fn mixed_number_big() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("sv", "ClearSpeak", expr, "3 och; bråk, 7 genom 83, slut bråk");
}

#[test]
fn simple_text() {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("sv", "ClearSpeak", expr, ", rise genom run");
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
    test("sv", "ClearSpeak", expr, ", 2 miles genom 3 gallons");
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
    test_prefs("sv", "ClearSpeak", vec![("ClearSpeak_Fractions", "Auto")], expr, ", division; en halv genom, 2 tredjedelar, slut division,");
    test_prefs("sv", "ClearSpeak", vec![("ClearSpeak_Fractions", "Ordinal")], expr, ", division; en halv genom, 2 tredjedelar, slut division,");
    test_prefs("sv", "ClearSpeak", vec![("ClearSpeak_Fractions", "Over")], expr, ", division; bråk, 1 genom 2, slut bråk, genom, bråk, 2 genom 3, slut bråk; slut division,");
    test_prefs("sv", "ClearSpeak", vec![("ClearSpeak_Fractions", "FracOver")], expr,
            "the fraction the fraction 1 over 2 over the fraction 2 over 3");
    test_prefs("sv", "ClearSpeak", vec![("ClearSpeak_Fractions", "General")], expr,
            "the fraction with numerator the fraction with numerator 1; and denominator 2; and denominator the fraction with numerator 2; and denominator 3;");
    test_prefs("sv", "ClearSpeak", vec![("ClearSpeak_Fractions", "EndFrac")], expr, ", division; en halv genom, 2 tredjedelar, slut division,");
    test_prefs("sv", "ClearSpeak", vec![("ClearSpeak_Fractions", "GeneralEndFrac")], expr,
            "the fraction with numerator the fraction with numerator 1; and denominator 2; end fraction; and denominator the fraction with numerator 2; and denominator 3; end fraction; end fraction,");
    test_prefs("sv", "ClearSpeak", vec![("ClearSpeak_Fractions", "OverEndFrac")], expr,
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
    test("sv", "ClearSpeak", expr, ", division; 2 tredjedelar x genom 6, slut division,");
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
    test("sv", "ClearSpeak", expr, "division med täljaren; bråk, 10 genom n, slut bråk; och nämnaren, bråk, 2 genom n, slut bråk;");
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
    test("sv", "ClearSpeak", expr, "division med täljaren; division med täljaren; n plus 10; och nämnaren n; och nämnaren, bråk, 2 genom n, slut bråk;");
}

#[test]
fn simple_function() {
    let expr = "<math><mfrac><mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mn>2</mn></mfrac></math>";
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, ", division, f av x genom 2, slut division,");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, ", division, f av x genom 2, slut division,");
}

#[test]
fn function_over_function() {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, ", division, f av x genom g av x, slut division,");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, ", division, f av x genom g av x, slut division,");
}

#[test]
fn non_simple_function_over_function() {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr,
             "division med täljaren; f av, vänster-parentes; x plus 1; höger-parentes; och nämnaren g av x;");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr,
    "division med täljaren; f av, vänster-parentes; x plus 1; höger-parentes; och nämnaren g av x; slut division,");
}

#[test]
fn binomial() {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("sv", "ClearSpeak", expr, "2 gånger, 7 över 3");
}
