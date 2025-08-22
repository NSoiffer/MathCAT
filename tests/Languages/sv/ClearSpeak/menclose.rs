use crate::common::*;

#[test]
fn menclose_actuarial() {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "aktuariell symbol, omsluter 3 plus 2 i, slut omslutning");
}

#[test]
fn menclose_box() {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "box, cirkel, omsluter 3 plus 2 i, slut omslutning");
}

#[test]
fn menclose_left() {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "linje vänster, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_right() {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "linje höger, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_top_bottom() {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "linje ovanför, under, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_updiagonalstrike() {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "diagonal uppåt, överstrykning, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_downdiagonalstrike() {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "diagonal nedåt, överstrykning, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_cross_out() {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "kryss, överstrykning, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_vertical_horizontal_strike() {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "vertikal, horisontell, överstrykning, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_leftarrow() {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "vänster-pil, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_right_up_down_arrow() {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "uppåt-pil, nedåt-pil, höger-pil, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_northeastarrow() {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "nordost-pil, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_other_single_arrows() {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "sydost-pil, sydväst-pil, nordväst-pil, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_northwestsoutheastarrow() {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "dubbelriktad diagonal nedåtpil, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_other_double_arrows() {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "dubbelriktad vertikal pil, dubbelriktad horisontell pil, dubbelriktad diagonal uppåtpil, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_madrub() {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "arabisk fakultet, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_phasorangle() {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "fasorvinkel, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_circle_phasorangle() {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "cirkel, fasorvinkel, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_longdiv() {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "lång division, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_longdiv_default() {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "lång division, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_longdiv_empty_string() {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "lång division, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_longdiv_whitespace_string() {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "lång division, omsluter 3 halva, slut omslutning");
}

#[test]
fn menclose_radical() {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "ClearSpeak", expr, "kvadratrot, omsluter 3 halva, slut omslutning");
}

#[test]
fn simple_speak_menclose_top_bottom() {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("sv", "SimpleSpeak", expr, "linje ovanför, under, omsluter 3 halva, slut omslutning");
}
