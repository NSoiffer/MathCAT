/// Tests for fractions
///   includes simple fractions and more complex fractions
///   also tests mixed fractions (implicit and explicit)
use crate::common::*;

#[test]
fn common_fraction_half() {
    let expr = "<math>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("de", "ClearSpeak", expr, "1 hälfte");
}

#[test]
fn common_fraction_thirds() {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("de", "ClearSpeak", expr, "2 dritte");
}

#[test]
fn common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "17 zehnte");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "17 zehnte");
}

#[test]
#[allow(non_snake_case)]
fn not_ClearSpeak_common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "89 durch 10");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "89 zehnte");
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
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "der bruch mit zähler; x plus y; und nenner x minus y");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "der bruch mit zähler; x plus y; und nenner x minus y");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Over")], expr, "x plus y durch x minus y");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "FracOver")], expr, "der bruch x plus y durch x minus y");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "General")], expr, "der bruch mit zähler; x plus y; und nenner x minus y");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "EndFrac")], expr, "der bruch mit zähler; x plus y; und nenner x minus y; ende des bruchs");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "GeneralEndFrac")], expr, "der bruch mit zähler; x plus y; und nenner x minus y; ende des bruchs");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "OverEndFrac")], expr, "x plus y durch x minus y, ende des bruchs");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Per")], expr, "x plus y pro x minus y");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Verbose"),("ClearSpeak_Fractions", "Auto")], expr, "der bruch mit zähler; x plus y; und nenner x minus y; ende des bruchs");
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
    test("de", "ClearSpeak", expr, "62 meiles pro stunde");
}


#[test]
fn mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("de", "ClearSpeak", expr, "3 und 1 hälfte");
}

#[test]
fn explicit_mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("de", "ClearSpeak", expr, "3 und 1 achtel");
}

#[test]
fn mixed_number_big() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("de", "ClearSpeak", expr, "3 und 7 durch 83");
}

#[test]
fn simple_text() {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("de", "ClearSpeak", expr, "rise durch run");
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
    test("de", "ClearSpeak", expr, "2 miles durch 3 gallons");
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
    test_prefs("de", "ClearSpeak", vec![("ClearSpeak_Fractions", "Auto")], expr, "1 hälfte durch 2 dritte");
    test_prefs("de", "ClearSpeak", vec![("ClearSpeak_Fractions", "Ordinal")], expr, "1 hälfte durch 2 dritte");
    test_prefs("de", "ClearSpeak", vec![("ClearSpeak_Fractions", "Over")], expr, "1 durch 2 durch 2 durch 3");
    test_prefs("de", "ClearSpeak", vec![("ClearSpeak_Fractions", "FracOver")], expr,
            "der bruch der bruch 1 durch 2 durch der bruch 2 durch 3");
    test_prefs("de", "ClearSpeak", vec![("ClearSpeak_Fractions", "General")], expr,
            "der bruch mit zähler der bruch mit zähler 1; und nenner 2; und nenner der bruch mit zähler 2; und nenner 3");
    test_prefs("de", "ClearSpeak", vec![("ClearSpeak_Fractions", "EndFrac")], expr, "1 hälfte durch 2 dritte");
    test_prefs("de", "ClearSpeak", vec![("ClearSpeak_Fractions", "GeneralEndFrac")], expr,
            "der bruch mit zähler der bruch mit zähler 1; und nenner 2; ende des bruchs; und nenner der bruch mit zähler 2; und nenner 3; ende des bruchs; ende des bruchs");
    test_prefs("de", "ClearSpeak", vec![("ClearSpeak_Fractions", "OverEndFrac")], expr,
            "1 durch 2, ende des bruchs, durch 2 durch 3, ende des bruchs; ende des bruchs");
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
    test("de", "ClearSpeak", expr, "2 dritte x durch 6");
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
    test("de", "ClearSpeak", expr, "der bruch mit zähler; 10 durch n; und nenner 2 durch n");
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
    test("de", "ClearSpeak", expr, "der bruch mit zähler; der bruch mit zähler; n plus 10; und nenner n; und nenner 2 durch n");
}

#[test]
fn simple_function() {
    let expr = "<math><mfrac><mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mn>2</mn></mfrac></math>";
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f von x durch 2");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f von x durch 2, ende des bruchs");
}

#[test]
fn function_over_function() {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f von x durch g von x");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f von x durch g von x, ende des bruchs");
}

#[test]
fn non_simple_function_over_function() {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr,
             "der bruch mit zähler; f von, klammer auf x plus 1, klammer zu; und nenner g von x");
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr,
             "der bruch mit zähler; f von, klammer auf x plus 1, klammer zu; und nenner g von x; ende des bruchs");
}

#[test]
fn binomial() {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("de", "ClearSpeak", expr, "2 mal 7 wählen 3");
}
