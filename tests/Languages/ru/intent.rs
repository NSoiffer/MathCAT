/// Тесты для правил, общих для различных стилей речи:
/// * здесь собраны тесты, посвящённые различным алфавитам
use crate::common::*;

use anyhow::Result;

#[test]
fn imaginary_part_name() -> Result<()> {
    // The imaginary part is a real component of a complex number, not a "complex part".
    let expr = r#"<math><mi intent="imaginary-part">Im</mi></math>"#;
    test("ru", "SimpleSpeak", expr, "мнимая часть")?;
    test("ru", "ClearSpeak", expr, "мнимая часть")?;
    return Ok(());
}

#[test]
fn silent_intent() -> Result<()> {
    let expr = "<math> <mrow intent='testing:silent($arg1, $arg2)'><mn arg='arg1'>2</mn> <mi arg='arg2'>x</mi></mrow> </math>";
    test("ru", "SimpleSpeak", expr, "2 икс")?;
    test("ru", "LiteralSpeak", expr, "2 икс")?;
    return Ok(());
}

#[test]
fn prefix_intent() -> Result<()> {
    let expr = r#"<math><msup intent='testing:prefix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("ru", "SimpleSpeak", expr, "testing икс")?;
    return Ok(());
}

#[test]
fn postfix_intent() -> Result<()> {
    let expr = r#"<math><msup intent='testing:postfix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("ru", "SimpleSpeak", expr, "икс testing")?;
    return Ok(());
}

#[test]
fn infix_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:infix($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "икс testing игрек testing зэт testing 2")?;
    return Ok(());
}

#[test]
fn infix_intent_no_args() -> Result<()> {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='testing:infix()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "икс")?;
    return Ok(());
}

#[test]
fn infix_intent_one_arg() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:infix($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    // Note: we say the intent name because there are infix plus/minus with a single arg due to continued rows or combined columns
    test("ru", "SimpleSpeak", expr, "testing икс")?;
    return Ok(());
}

#[test]
fn function_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:function($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "testing от икс запятая; игрек запятая, зэт запятая, 2")?;
    return Ok(());
}

#[test]
fn function_no_args_intent() -> Result<()> {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='testing:function()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "икс")?;
    return Ok(());
}

#[test]
fn function_one_arg_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:function($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("ru", "SimpleSpeak", expr, "testing от икс")?;
    return Ok(());
}

#[test]
fn silent_intent_mi() -> Result<()> {
    let expr = "<math> <mn>2</mn> <mi intent=':silent'>x</mi></math>";
    test("ru", "SimpleSpeak", expr, "2")?;
    test("ru", "ClearSpeak", expr, "2")?;
    return Ok(());
}

#[test]
fn silent_intent_msup() -> Result<()> {
    let expr = "<math>
        <msup intent='index:silent($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("ru", "SimpleSpeak", expr, "заглавная аш 2")?;
    test("ru", "ClearSpeak", expr, "заглавная аш 2")?;
    return Ok(());
}

#[test]
fn silent_intent_underscore() -> Result<()> {
    let expr = "<math>
        <msup intent='_($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("ru", "SimpleSpeak", expr, "заглавная аш 2")?;
    test("ru", "ClearSpeak", expr, "заглавная аш 2")?;
    return Ok(());
}

#[test]
fn intent_prob_x() -> Result<()> {
    let expr = "<math>
    <msup intent='$op($arg)'>
        <mi arg='arg'>x</mi>
        <mi arg='op' intent='probability' mathvariant='normal'>P</mi>
    </msup></math>";
    test("ru", "ClearSpeak", expr, "probability от икс")?;
    return Ok(());
}
