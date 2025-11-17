use crate::common::*;

#[test]
fn menclose_actuarial() {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "aktuarielt symbol, omslutter 3 pluss 2 i, slutt omslutning");
}

#[test]
fn menclose_box() {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "boks, sirkel, omslutter 3 pluss 2 i, slutt omslutning");
}

#[test]
fn menclose_left() {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "linje venstre, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_right() {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "linje høyre, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_top_bottom() {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "linje over, under, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_updiagonalstrike() {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "diagonalt oppover, utstryking, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_downdiagonalstrike() {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "diagonalt nedover, utstryking, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_cross_out() {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "kryss, utstryking, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_vertical_horizontal_strike() {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "vertikal, horisontal, utstryking, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_leftarrow() {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "venstrepil, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_right_up_down_arrow() {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "oppoverpil, nedoverpil, høyrepil, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_northeastarrow() {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "nordøst-pil, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_other_single_arrows() {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "sørøst-pil, sørvest-pil, nordvest-pil, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_northwestsoutheastarrow() {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "dobbelendet diagonal nedoverpil, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_other_double_arrows() {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "dobbelendet vertikal pil, dobbelendet horisontal pil, dobbelendet diagonal oppoverpil, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_madrub() {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "arabisk fakultet, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_phasorangle() {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "fasevinkel, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_circle_phasorangle() {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "sirkel, fasevinkel, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_longdiv() {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "lang divisjon, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_longdiv_default() {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "lang divisjon, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_longdiv_empty_string() {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "lang divisjon, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_longdiv_whitespace_string() {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "lang divisjon, omslutter 3 halve, slutt omslutning");
}

#[test]
fn menclose_radical() {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "ClearSpeak", expr, "kvadratrot, omslutter 3 halve, slutt omslutning");
}

#[test]
fn simple_speak_menclose_top_bottom() {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("nb", "SimpleSpeak", expr, "linje over, under, omslutter 3 halve, slutt omslutning");
}
