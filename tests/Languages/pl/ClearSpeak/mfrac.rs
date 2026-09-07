/// Tests for fractions
///   includes simple fractions and more complex fractions
///   also tests mixed fractions (implicit and explicit)
use crate::common::*;
use anyhow::Result;

#[test]
fn common_fraction_half() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("pl", "ClearSpeak", expr, "jedna druga")?;
    return Ok(());

}

#[test]
fn common_fraction_thirds() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("pl", "ClearSpeak", expr, "dwie trzecie")?;
    return Ok(());

}

#[test]
fn common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "17 dziesiąte")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "17 dziesiąte")?;
    return Ok(());

}

#[test]
#[allow(non_snake_case)]
fn not_ClearSpeak_common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "89 przez 10")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "89 dziesiąte")?;
    return Ok(());

}

#[test]
fn non_simple_fraction() -> Result<()> {
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
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "ułamek z licznikiem; x plus y; i mianownikiem x minus y")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "ułamek z licznikiem; x plus y; i mianownikiem x minus y")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Over")], expr, "x plus y przez x minus y")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "FracOver")], expr, "ułamek x plus y przez x minus y")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "General")], expr, "ułamek z licznikiem; x plus y; i mianownikiem x minus y")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "EndFrac")], expr, "ułamek z licznikiem; x plus y; i mianownikiem x minus y; koniec ułamka")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "GeneralEndFrac")], expr, "ułamek z licznikiem; x plus y; i mianownikiem x minus y; koniec ułamka")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "OverEndFrac")], expr, "x plus y przez x minus y, koniec ułamka")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Per")], expr, "x plus y na x minus y")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Verbose"),("ClearSpeak_Fractions", "Auto")], expr, "ułamek z licznikiem; x plus y; i mianownikiem x minus y; koniec ułamka")?;
    return Ok(());

}

#[test]
fn frac_with_units() -> Result<()> {
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
    test("pl", "ClearSpeak", expr, "62 mile na godzinę")?;
    return Ok(());

}


#[test]
fn mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("pl", "ClearSpeak", expr, "3 i jedna druga")?;
    return Ok(());

}

#[test]
fn explicit_mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("pl", "ClearSpeak", expr, "3 i jedna ósma")?;
    return Ok(());

}

#[test]
fn mixed_number_big() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("pl", "ClearSpeak", expr, "3 i 7 przez 83")?;
    return Ok(());

}

#[test]
fn simple_text() -> Result<()> {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("pl", "ClearSpeak", expr, "rise przez run")?;
    return Ok(());

}

#[test]
fn number_and_text() -> Result<()> {
    let expr = "<math>
            <mfrac>
            <mrow>
                <mn>2</mn><mtext>miles</mtext></mrow>
            <mrow>
                <mn>3</mn><mtext>gallons</mtext></mrow>
            </mfrac>
        </math>";
    test("pl", "ClearSpeak", expr, "2 miles przez 3 gallons")?;
    return Ok(());

}


#[test]
fn nested_simple_fractions() -> Result<()> {
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
    test_prefs("pl", "ClearSpeak", vec![("ClearSpeak_Fractions", "Auto")], expr, "jedna druga przez dwie trzecie")?;
    test_prefs("pl", "ClearSpeak", vec![("ClearSpeak_Fractions", "Ordinal")], expr, "jedna druga przez dwie trzecie")?;
    test_prefs("pl", "ClearSpeak", vec![("ClearSpeak_Fractions", "Over")], expr, "1 przez 2 przez 2 przez 3")?;
    test_prefs("pl", "ClearSpeak", vec![("ClearSpeak_Fractions", "FracOver")], expr,
            "ułamek ułamek 1 przez 2 przez ułamek 2 przez 3")?;
    test_prefs("pl", "ClearSpeak", vec![("ClearSpeak_Fractions", "General")], expr,
            "ułamek z licznikiem ułamek z licznikiem 1; i mianownikiem 2; i mianownikiem ułamek z licznikiem 2; i mianownikiem 3")?;
    test_prefs("pl", "ClearSpeak", vec![("ClearSpeak_Fractions", "EndFrac")], expr, "jedna druga przez dwie trzecie")?;
    test_prefs("pl", "ClearSpeak", vec![("ClearSpeak_Fractions", "GeneralEndFrac")], expr,
            "ułamek z licznikiem ułamek z licznikiem 1; i mianownikiem 2; koniec ułamka; i mianownikiem ułamek z licznikiem 2; i mianownikiem 3; koniec ułamka; koniec ułamka")?;
    test_prefs("pl", "ClearSpeak", vec![("ClearSpeak_Fractions", "OverEndFrac")], expr,
            "1 przez 2, koniec ułamka, przez 2 przez 3, koniec ułamka; koniec ułamka")?;
            return Ok(());

}


#[test]
fn semi_nested_fraction() -> Result<()> {
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
    test("pl", "ClearSpeak", expr, "dwie trzecie x przez 6")?;
    return Ok(());

}

#[test]
fn general_nested_fraction() -> Result<()> {
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
    test("pl", "ClearSpeak", expr, "ułamek z licznikiem; 10 przez n; i mianownikiem 2 przez n")?;
    return Ok(());

}

#[test]
fn complex_nested_fraction() -> Result<()> {
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
    test("pl", "ClearSpeak", expr, "ułamek z licznikiem; ułamek z licznikiem; n plus 10; i mianownikiem n; i mianownikiem 2 przez n")?;
    return Ok(());

}

#[test]
fn simple_function() -> Result<()> {
    let expr = "<math><mfrac><mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mn>2</mn></mfrac></math>";
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f z x przez 2")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f z x przez 2, koniec ułamka")?;
    return Ok(());

}

#[test]
fn function_over_function() -> Result<()> {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f z x przez g z x")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f z x przez g z x, koniec ułamka")?;
    return Ok(());

}

#[test]
fn non_simple_function_over_function() -> Result<()> {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr,
             "ułamek z licznikiem; f z, nawias otwierający, x plus 1, nawias zamykający; i mianownikiem g z x")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr,
             "ułamek z licznikiem; f z, nawias otwierający, x plus 1, nawias zamykający; i mianownikiem g z x; koniec ułamka")?;
             return Ok(());

}

#[test]
fn binomial() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("pl", "ClearSpeak", expr, "2 razy 7 po 3")?;
    return Ok(());

}
