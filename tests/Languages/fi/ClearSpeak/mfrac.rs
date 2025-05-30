/// Tests for fractions
///   includes simple fractions and more complex fractions
///   also tests mixed fractions (implicit and explicit)
use crate::common::*;

#[test]
fn common_fraction_half() {
    let expr = "<math>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("fi", "ClearSpeak", expr, "1 kahdesosa");
}

#[test]
fn common_fraction_thirds() {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("fi", "ClearSpeak", expr, "2 kolmasosaa");
}

#[test]
fn common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "17 kymmenesosaa");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "17 kymmenesosaa");
}

#[test]
#[allow(non_snake_case)]
fn not_ClearSpeak_common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "89 per 10");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "89 kymmenesosaa");
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
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "murtoluku osoittaja; x plus y; ja nimittäjä x miinus y");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "murtoluku osoittaja; x plus y; ja nimittäjä x miinus y");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Over")], expr, "x plus y per x miinus y");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "FracOver")], expr, "murtoluku x plus y per x miinus y");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "General")], expr, "murtoluku osoittaja; x plus y; ja nimittäjä x miinus y");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "EndFrac")], expr, "murtoluku osoittaja; x plus y; ja nimittäjä x miinus y; loppu murtoluku");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "GeneralEndFrac")], expr, "murtoluku osoittaja; x plus y; ja nimittäjä x miinus y; loppu murtoluku");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "OverEndFrac")], expr, "x plus y per x miinus y, loppu murtoluku");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Per")], expr, "x plus y per x miinus y");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Verbose"),("ClearSpeak_Fractions", "Auto")], expr, "murtoluku osoittaja; x plus y; ja nimittäjä x miinus y; loppu murtoluku");
}


#[test]
fn mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("fi", "ClearSpeak", expr, "3 ja 1 kahdesosa");
}

#[test]
fn explicit_mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("fi", "ClearSpeak", expr, "3 ja 1 kahdeksasosa");
}

#[test]
fn mixed_number_big() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("fi", "ClearSpeak", expr, "3 ja 7 per 83");
}

#[test]
fn simple_text() {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("fi", "ClearSpeak", expr, "rise per run");
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
    test("fi", "ClearSpeak", expr, "2 miles per 3 gallons");
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
    test_prefs("fi", "ClearSpeak", vec![("ClearSpeak_Fractions", "Auto")], expr, "1 kahdesosa per 2 kolmasosaa");
    test_prefs("fi", "ClearSpeak", vec![("ClearSpeak_Fractions", "Ordinal")], expr, "1 kahdesosa per 2 kolmasosaa");
    test_prefs("fi", "ClearSpeak", vec![("ClearSpeak_Fractions", "Over")], expr, "1 per 2 per 2 per 3");
    test_prefs("fi", "ClearSpeak", vec![("ClearSpeak_Fractions", "FracOver")], expr,
            "murtoluku murtoluku 1 per 2 per murtoluku 2 per 3");
    test_prefs("fi", "ClearSpeak", vec![("ClearSpeak_Fractions", "General")], expr,
            "murtoluku osoittaja murtoluku osoittaja 1; ja nimittäjä 2; ja nimittäjä murtoluku osoittaja 2; ja nimittäjä 3");
    test_prefs("fi", "ClearSpeak", vec![("ClearSpeak_Fractions", "EndFrac")], expr, "1 kahdesosa per 2 kolmasosaa");
    test_prefs("fi", "ClearSpeak", vec![("ClearSpeak_Fractions", "GeneralEndFrac")], expr,
            "murtoluku osoittaja murtoluku osoittaja 1; ja nimittäjä 2; loppu murtoluku; ja nimittäjä murtoluku osoittaja 2; ja nimittäjä 3; loppu murtoluku; loppu murtoluku");
    test_prefs("fi", "ClearSpeak", vec![("ClearSpeak_Fractions", "OverEndFrac")], expr,
    "1 per 2, loppu murtoluku, per 2 per 3, loppu murtoluku; loppu murtoluku");
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
    test("fi", "ClearSpeak", expr, "2 kolmasosaa x per 6");
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
    test("fi", "ClearSpeak", expr, "murtoluku osoittaja; 10 per n; ja nimittäjä 2 per n");
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
    test("fi", "ClearSpeak", expr, "murtoluku osoittaja; murtoluku osoittaja; n plus 10; ja nimittäjä n; ja nimittäjä 2 per n");
}

#[test]
fn simple_function() {
    let expr = "<math><mfrac><mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mn>2</mn></mfrac></math>";
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f arvolla x per 2");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f arvolla x per 2, loppu murtoluku");
}

#[test]
fn function_over_function() {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f arvolla x per g arvolla x");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f arvolla x per g arvolla x, loppu murtoluku");
}

#[test]
fn non_simple_function_over_function() {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr,
             "murtoluku osoittaja; f arvolla, auki sulku x plus 1, kiinni sulku; ja nimittäjä g arvolla x");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr,
             "murtoluku osoittaja; f arvolla, auki sulku x plus 1, kiinni sulku; ja nimittäjä g arvolla x; loppu murtoluku");
}

#[test]
fn binomial() {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("fi", "ClearSpeak", expr, "2 kertaa 7 yli 3");
}
