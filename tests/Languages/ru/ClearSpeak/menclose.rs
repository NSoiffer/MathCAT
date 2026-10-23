use crate::common::*;

use anyhow::Result;

#[test]
fn menclose_actuarial() -> Result<()> {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "актуарный символ, внутри 3 плюс 2 и конец")?;
    return Ok(());
}

#[test]
fn menclose_box() -> Result<()> {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "рамка, окружность, внутри 3 плюс 2 и конец")?;
    return Ok(());
}

#[test]
fn menclose_left() -> Result<()> {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "линия слева, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_right() -> Result<()> {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "линия справа, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "линия сверху, снизу, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_updiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "диагональ вверх, перечёркивание, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_downdiagonalstrike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "диагональ вниз, перечёркивание, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_cross_out() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "икс, перечёркивание, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_vertical_horizontal_strike() -> Result<()> {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "вертикальная, горизонтальная, перечёркивание, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_leftarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "стрелка влево, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_right_up_down_arrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "стрелка вверх, стрелка вниз, стрелка вправо, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_northeastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "стрелка на северо-восток, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_other_single_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "стрелка на юго-восток, стрелка на юго-запад, стрелка на северо-запад, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_northwestsoutheastarrow() -> Result<()> {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "двунаправленная диагональная стрелка вниз, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_other_double_arrows() -> Result<()> {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "двунаправленная вертикальная стрелка, двунаправленная горизонтальная стрелка, двунаправленная диагональная стрелка вверх, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_madrub() -> Result<()> {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "арабский символ факториала, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "угол фазы, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_circle_phasorangle() -> Result<()> {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "окружность, угол фазы, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_longdiv() -> Result<()> {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "знак деления уголком, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_longdiv_default() -> Result<()> {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "знак деления уголком, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_longdiv_empty_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "знак деления уголком, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_longdiv_whitespace_string() -> Result<()> {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "знак деления уголком, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn menclose_radical() -> Result<()> {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "ClearSpeak", expr, "квадратный корень, внутри три вторых конец")?;
    return Ok(());
}

#[test]
fn simple_speak_menclose_top_bottom() -> Result<()> {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ru", "SimpleSpeak", expr, "линия сверху, снизу, внутри три вторых конец")?;
    return Ok(());
}
