use crate::common::*;

use anyhow::Result;

#[test]
fn msqrt_simple() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test("ru", "ClearSpeak", expr, "квадратный корень из икс")?;
    return Ok(());
}

#[test]
fn msqrt_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "RootEnd", expr, "квадратный корень из икс, конец корня")?;
    return Ok(());
}

#[test]
fn msqrt_simple_positive() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "PosNegSqRoot", expr, "квадратный корень из икс")?;
    return Ok(());
}

#[test]
fn msqrt_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                    <msqrt> <mi>x</mi> </msqrt>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "квадратный корень из икс, конец корня")?;
    return Ok(());
}

#[test]
fn msqrt_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, 
    "минус квадратный корень из икс, конец корня; минус, кубический корень из икс, конец корня")?;
    return Ok(());
}

#[test]
fn mroot_simple_pos_end_with_neg_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mroot> <mi>x</mi> <mn>3</mn></mroot>
                    <mo>-</mo> <msqrt> <mi>x</mi> </msqrt>

                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "PosNegSqRoot", expr, 
    "минус кубический корень из икс; минус квадратный корень из икс")?;
    return Ok(());
}

#[test]
fn neg_without_root() -> Result<()> {
    let expr = "<math>
                    <mo>-</mo> <mi>x</mi> <mo>-</mo> <mi>y</mi>
                </math>";
    test("ru", "ClearSpeak", expr, "минус икс минус игрек")?;
    return Ok(());
}

#[test]
fn msqrt() -> Result<()> {
    let expr = "<math>
                    <msqrt>
                        <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                    </msqrt>
                </math>";
    test("ru", "ClearSpeak", expr, "квадратный корень из выражения икс плюс игрек")?;
    return Ok(());
}

#[test]
fn mroot_as_square_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                </math>";
    test("ru", "ClearSpeak", expr, "квадратный корень из икс")?;
    return Ok(());
}

#[test]
fn cube_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                </math>";
    test("ru", "ClearSpeak", expr, "кубический корень из икс")?;
    return Ok(());
}

#[test]
fn ordinal_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                </math>";
    test("ru", "ClearSpeak", expr, "корень девятой степени из икс")?;
    return Ok(());
}

#[test]
fn simple_mi_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                </math>";
    test("ru", "ClearSpeak", expr, "корень степени эн из икс")?;
    return Ok(());
}

#[test]
fn mroot_simple_pos_end_root() -> Result<()> {
    let expr = "<math>
                <mroot> <mi>x</mi> <mi>t</mi> </mroot>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "PosNegSqRootEnd", expr, "корень степени тэ из икс, конец корня")?;
    return Ok(());
}

#[test]
fn mroot_simple_end_root() -> Result<()> {
    let expr = "<math>
                    <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                    <mn>21</mn></mroot>
                </math>";
    test_ClearSpeak("ru", "ClearSpeak_Roots", "RootEnd", expr, "корень двадцать первой степени из выражения икс плюс игрек, конец корня")?;
    return Ok(());
}

#[test]
fn simple_fraction_power() -> Result<()> {
    let expr = "<math>
                    <mroot>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </mroot>
                </math>";
    test("ru", "ClearSpeak", expr, "корень степени одна третья из икс")?;
    return Ok(());
}
