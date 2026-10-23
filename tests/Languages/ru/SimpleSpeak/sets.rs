use crate::common::*;

use anyhow::Result;

#[test]
fn complex() -> Result<()> {
    let expr = "<math>
                    <mi>ℂ</mi>
                </math>";
    test("ru", "SimpleSpeak", expr, "комплексные числа")?;
    return Ok(());
}

#[test]
fn natural() -> Result<()> {
    let expr = "<math>
                    <mi>ℕ</mi>
                </math>";
    test("ru", "SimpleSpeak", expr, "натуральные числа")?;
    return Ok(());
}

#[test]
fn rationals() -> Result<()> {
    let expr = "<math>
                    <mi>ℚ</mi>
                </math>";
    test("ru", "SimpleSpeak", expr, "рациональные числа")?;
    return Ok(());
}

#[test]
fn reals() -> Result<()> {
    let expr = "<math>
                    <mi>ℝ</mi>
                </math>";
    test("ru", "SimpleSpeak", expr, "действительные числа")?;
    return Ok(());
}

#[test]
fn integers() -> Result<()> {
    let expr = "<math>
                    <mi>ℤ</mi>
                </math>";
    test("ru", "SimpleSpeak", expr, "целые числа")?;
    return Ok(());
}

#[test]
fn msup_complex() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℂ</mi>
                    <mn>2</mn>
                </msup>
                </math>";
    test("ru", "SimpleSpeak", expr, "цэ 2")?;
    return Ok(());
}

#[test]
fn msup_natural() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℕ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("ru", "SimpleSpeak", expr, "эн 2")?;
    return Ok(());
}

#[test]
fn msup_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("ru", "SimpleSpeak", expr, "ку 2")?;
    return Ok(());
}

#[test]
fn msup_reals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℝ</mi>
                    <mn>3</mn>
                </msup>
            </math>";
    test("ru", "SimpleSpeak", expr, "эр 3")?;
    return Ok(());
}

#[test]
fn msup_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mn>4</mn>
                </msup>
            </math>";
    test("ru", "SimpleSpeak", expr, "зэт 4")?;
    return Ok(());
}

#[test]
fn msup_positive_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("ru", "SimpleSpeak", expr, "положительные целые числа")?;
    return Ok(());
}

#[test]
fn msup_negative_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("ru", "SimpleSpeak", expr, "отрицательные целые числа")?;
    return Ok(());
}

#[test]
fn msup_positive_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("ru", "SimpleSpeak", expr, "положительные рациональные числа")?;
    return Ok(());
}

#[test]
fn msup_negative_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("ru", "SimpleSpeak", expr, "отрицательные рациональные числа")?;
    return Ok(());
}

#[test]
fn empty_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mo>}</mo>
            </math>";
    test("ru", "SimpleSpeak", expr, "пустое множество")?;
    return Ok(());
}

#[test]
fn single_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>12</mn><mo>}</mo>
            </math>";
    test("ru", "SimpleSpeak", expr, "множество 12")?;
    return Ok(());
}

#[test]
fn multiple_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test("ru", "SimpleSpeak", expr, "множество 5 запятая, 10 запятая, 15")?;
    return Ok(());
}

#[test]
fn set_with_colon() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("ru", "SimpleSpeak", expr, "множество всех икс таких, что икс больше 2")?;
    return Ok(());
}

#[test]
fn set_with_bar() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("ru", "SimpleSpeak", expr, "множество всех икс таких, что икс больше 2")?;
    return Ok(());
}

#[test]
fn element_alone() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test("ru", "SimpleSpeak", expr, "3 плюс 2 и, не является элементом множества действительных чисел")?;
    return Ok(());
}

#[test]
fn element_under_sum() -> Result<()> {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup>  <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test("ru", "SimpleSpeak", expr,
                    "сумма по и, являющемуся элементом множества целых чисел, от; дробь, числитель: 1, знаменатель: и в квадрате, конец дроби")?;
                    return Ok(());
}

#[test]
fn complicated_set_with_colon() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mn>2</mn>
            <mo>&#x003C;</mo>
            <mi>x</mi>
            <mo>&#x003C;</mo>
            <mn>7</mn>
            <mo>}</mo>
        </math>";
    test("ru", "SimpleSpeak", expr, "множество всех икс из целых чисел, таких что 2 меньше икс меньше 7")?;
    return Ok(());
}

#[test]
fn complicated_set_with_mtext() -> Result<()> {
    let expr = "<math>
        <mo>{</mo>
        <mrow> <mi>x</mi><mo>∈</mo><mi>ℕ</mi></mrow>
        <mo>|</mo>
        <mrow><mi>x</mi> <mtext>&#x00A0;is&#x00A0;an&#x00A0;even&#x00A0;number</mtext> </mrow>
        <mo>}</mo>
        </math>";
    test("ru", "SimpleSpeak", expr, 
            "множество всех икс из натуральных чисел, таких что икс — чётное число")?;
            return Ok(());
}
