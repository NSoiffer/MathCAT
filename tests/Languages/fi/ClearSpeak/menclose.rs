use crate::common::*;

#[test]
fn menclose_actuarial() {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "kirjanpitomerkki, rajaus 3 plus 2 i loppu rajaus");
}

#[test]
fn menclose_box() {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "laatikko, ympyrä, rajaus 3 plus 2 i loppu rajaus");
}

#[test]
fn menclose_left() {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "viiva vasemmalla, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_right() {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "viiva oikealla, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_top_bottom() {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "viiva yläpuolella, alapuolella, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_updiagonalstrike() {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "ylös viistoon, yliviivaus, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_downdiagonalstrike() {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "alas viistoon, yliviivaus, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_cross_out() {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "x, yliviivaus, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_vertical_horizontal_strike() {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "pystysuora, vaakasuora, yliviivaus, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_leftarrow() {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "nuoli vasemmalle, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_right_up_down_arrow() {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "nuoli ylös, nuoli alas, nuoli oikealle, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_northeastarrow() {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "nuoli oikealle ylös, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_other_single_arrows() {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "nuoli oikealle alas, nuoli vasemmalle alas, nuoli vasemmalle ylös, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_northwestsoutheastarrow() {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "nuoli ylös alas oikealle alas, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_other_double_arrows() {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "nuoli ylös alas, nuoli vasemmalle oikealle, nuoli ylös alas oikealle ylös, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_madrub() {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "arabialainen kertoma, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_phasorangle() {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "vaihekulma, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_circle_phasorangle() {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "ympyrä, vaihekulma, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_longdiv() {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "jakokulma, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_longdiv_default() {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "jakokulma, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_longdiv_empty_string() {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "jakokulma, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_longdiv_whitespace_string() {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "jakokulma, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn menclose_radical() {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "ClearSpeak", expr, "neliöjuuri, rajaus 3 kahdesosaa loppu rajaus");
}

#[test]
fn simple_speak_menclose_top_bottom() {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("fi", "SimpleSpeak", expr, "viiva yläpuolella, alapuolella, rajaus 3 kahdesosaa loppu rajaus");
}
