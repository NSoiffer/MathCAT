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
    test("ru", "SimpleSpeak", expr, "одна вторая")?;
    return Ok(());
}

#[test]
fn common_fraction_thirds() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("ru", "SimpleSpeak", expr, "две третьих")?;
    return Ok(());
}

#[test]
fn common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test("ru", "SimpleSpeak", expr, "семнадцать десятых")?;
    return Ok(());
}

#[test]
#[allow(non_snake_case)]
fn not_SimpleSpeak_common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test("ru", "SimpleSpeak", expr, "89 разделить на 10")?;
    return Ok(());
}

#[test]
fn non_simple_fraction() -> Result<()> {
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
    test("ru", "SimpleSpeak", expr, "дробь, числитель: икс плюс игрек, знаменатель: икс минус игрек, конец дроби")?;
    return Ok(());
}

#[test]
fn nested_fraction() -> Result<()> {
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
    test("ru", "SimpleSpeak", expr, "дробь, числитель: икс плюс, дробь, числитель: 1, знаменатель: игрек, конец дроби; знаменатель: икс минус игрек, конец дроби")?;
    return Ok(());
}

#[test]
fn deeply_nested_fraction_msqrt() -> Result<()> {
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
    test("ru", "SimpleSpeak", expr, "дробь, числитель: икс плюс, квадратный корень из выражения 1 разделить на игрек, конец корня; знаменатель: икс минус игрек, конец дроби")?;
    return Ok(());
}

#[test]
fn deeply_nested_fraction_mrow_msqrt() -> Result<()> {
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
    test("ru", "SimpleSpeak", expr, "дробь, числитель: икс плюс, квадратный корень из выражения 2 плюс 1 разделить на игрек, конец корня; знаменатель: икс минус игрек, конец дроби")?;
    return Ok(());
}

#[test]
fn numerator_simple_fraction() -> Result<()> {
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
    test("ru", "SimpleSpeak", expr, "дробь, числитель: икс, знаменатель: икс минус игрек, конец дроби")?;
    return Ok(());
}

#[test]
fn denominator_simple_fraction() -> Result<()> {
    let expr = "
    <math>
        <mfrac>
            <mrow> <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
            <mrow> <mi>x</mi></mrow>
        </mfrac>
    </math>
                            ";
    test("ru", "SimpleSpeak", expr, "дробь, числитель: икс минус игрек, знаменатель: икс, конец дроби")?;
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
    test("ru", "SimpleSpeak", expr, "62 мили в час")?;
    return Ok(());
}

#[test]
fn singular_frac_with_units() -> Result<()> {
    let expr = "
    <math>
        <mrow>
        <mn>1</mn>
        <mfrac>
        <mi intent=':unit'>gal</mi>
        <mi intent=':unit'>mi</mi>
        </mfrac>
        </mrow>
    </math>";
    test("ru", "SimpleSpeak", expr, "1 галлон на милю")?;
    return Ok(());
}

#[test]
fn mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("ru", "SimpleSpeak", expr, "3 целых одна вторая")?;
    return Ok(());
}

#[test]
fn explicit_mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("ru", "SimpleSpeak", expr, "3 целых одна восьмая")?;
    return Ok(());
}

#[test]
fn mixed_number_big() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("ru", "SimpleSpeak", expr, "3 целых и семь восемьдесят третьих")?;
    return Ok(());
}

#[test]
fn simple_text() -> Result<()> {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("ru", "SimpleSpeak", expr, "подъём на длину")?;
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
    test("ru", "SimpleSpeak", expr, "дробь, числитель: 2 мили, знаменатель: 3 галлона, конец дроби")?;
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
    test("ru", "SimpleSpeak", expr, "дробь, числитель: одна вторая, знаменатель: две третьих, конец дроби")?;
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
    test("ru", "SimpleSpeak", expr, "2 умножить на число сочетаний из 7 по 3")?;
    return Ok(());
}

#[test]
fn binomial_non_simple_top() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mrow><mi>n</mi><mo>+</mo><mn>7</mn></mrow> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("ru", "SimpleSpeak", expr, "2 умножить на, число сочетаний из эн плюс 7 по 3")?;
    return Ok(());
}

#[test]
fn binomial_non_simple_bottom() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mrow><mi>k</mi><mo>+</mo><mn>3</mn></mrow> </mfrac>
                    <mo>)</mo>
                </math>";
    test("ru", "SimpleSpeak", expr, "2 умножить на, число сочетаний из 7 по ка плюс 3 конец числа сочетаний")?;
    return Ok(());
}

#[test]
fn binomial_non_simple_top_and_bottom() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mrow><mi>n</mi><mo>+</mo><mn>7</mn></mrow> <mrow><mi>k</mi><mo>+</mo><mn>3</mn></mrow> </mfrac>
                    <mo>)</mo>
                </math>";
    test("ru", "SimpleSpeak", expr, "2 умножить на, число сочетаний из эн плюс 7 по ка плюс 3 конец числа сочетаний")?;
    return Ok(());
}
