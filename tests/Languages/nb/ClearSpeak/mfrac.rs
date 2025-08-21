/// Tests for fractions
///   includes simple fractions and more complex fractions
///   also tests mixed fractions (implicit and explicit)
use crate::common::*;

#[test]
fn common_fraction_half() {
    let expr = "<math>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("nb", "ClearSpeak", expr, "1 halv");
}

#[test]
fn common_fraction_thirds() {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("nb", "ClearSpeak", expr, "2 tredjedeler");
}

#[test]
fn common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "17 tideler");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "17 tideler");
}

#[test]
#[allow(non_snake_case)]
fn not_ClearSpeak_common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "89 over 10");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "89 tideler");
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
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "brøken med teller; x pluss y; og nevner x minus y");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "brøken med teller; x pluss y; og nevner x minus y");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Over")], expr, "x pluss y over x minus y");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "FracOver")], expr, "brøken x pluss y over x minus y");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "General")], expr, "brøken med teller; x pluss y; og nevner x minus y");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "EndFrac")], expr, "brøken med teller; x pluss y; og nevner x minus y; slutt brøk");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "GeneralEndFrac")], expr, "brøken med teller; x pluss y; og nevner x minus y; slutt brøk");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "OverEndFrac")], expr, "x pluss y over x minus y, slutt brøk");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Per")], expr, "x pluss y per x minus y");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Verbose"),("ClearSpeak_Fractions", "Auto")], expr, "brøken med teller; x pluss y; og nevner x minus y; slutt brøk");
}

#[test]
fn frac_with_units() {
    let expr = "
    <math>
        <mrow>
        <mn>62</mn>
        <mfrac>
        <mi intent=':unit'>mi</mi>
        <mi intent=':unit'>hr</mi>
        </mfrac>
        </mrow>
    </math>";
    test("nb", "ClearSpeak", expr, "62 miles per time");
}


#[test]
fn mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("nb", "ClearSpeak", expr, "3 og 1 halv");
}

#[test]
fn explicit_mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("nb", "ClearSpeak", expr, "3 og 1 åttedel");
}

#[test]
fn mixed_number_big() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("nb", "ClearSpeak", expr, "3 og 7 over 83");
}

#[test]
fn simple_text() {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("nb", "ClearSpeak", expr, ", rise over run");
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
    test("nb", "ClearSpeak", expr, ", 2 miles over 3 gallons");
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
    test_prefs("nb", "ClearSpeak", vec![("ClearSpeak_Fractions", "Auto")], expr, "1 halv over 2 tredjedeler");
    test_prefs("nb", "ClearSpeak", vec![("ClearSpeak_Fractions", "Ordinal")], expr, "1 halv over 2 tredjedeler");
    test_prefs("nb", "ClearSpeak", vec![("ClearSpeak_Fractions", "Over")], expr, "1 over 2 over 2 over 3");
    test_prefs("nb", "ClearSpeak", vec![("ClearSpeak_Fractions", "FracOver")], expr,
            "brøken brøken 1 over 2 over brøken 2 over 3");
    test_prefs("nb", "ClearSpeak", vec![("ClearSpeak_Fractions", "General")], expr,
            "brøken med teller brøken med teller 1; og nevner 2; og nevner brøken med teller 2; og nevner 3");
    test_prefs("nb", "ClearSpeak", vec![("ClearSpeak_Fractions", "EndFrac")], expr, "1 halv over 2 tredjedeler");
    test_prefs("nb", "ClearSpeak", vec![("ClearSpeak_Fractions", "GeneralEndFrac")], expr,
            "brøken med teller brøken med teller 1; og nevner 2; slutt brøk; og nevner brøken med teller 2; og nevner 3; slutt brøk; slutt brøk");
    test_prefs("nb", "ClearSpeak", vec![("ClearSpeak_Fractions", "OverEndFrac")], expr,
            "1 over 2, slutt brøk, over 2 over 3, slutt brøk; slutt brøk");
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
    test("nb", "ClearSpeak", expr, "2 tredjedeler x over 6");
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
    test("nb", "ClearSpeak", expr, "brøken med teller; 10 over n; og nevner 2 over n");
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
    test("nb", "ClearSpeak", expr, "brøken med teller; brøken med teller; n pluss 10; og nevner n; og nevner 2 over n");
}

#[test]
fn simple_function() {
    let expr = "<math><mfrac><mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mn>2</mn></mfrac></math>";
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f av x over 2");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f av x over 2, slutt brøk");
}

#[test]
fn function_over_function() {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f av x over g av x");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f av x over g av x, slutt brøk");
}

#[test]
fn non_simple_function_over_function() {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr,
             "brøken med teller; f av; startparentes; x pluss 1; sluttparentes; og nevner g av x");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr,
             "brøken med teller; f av; startparentes; x pluss 1; sluttparentes; og nevner g av x; slutt brøk");
}

#[test]
fn binomial() {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("nb", "ClearSpeak", expr, "2 ganger 7 over 3");
}
