use crate::common::*;

#[test]
fn menclose_actuarial() {
    let expr = "<math>
                    <menclose notation='actuarial'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "versicherungsmathematische symbol, einschließen 3 plus 2 i ende der einschliessung");
}

#[test]
fn menclose_box() {
    let expr = "<math>
                    <menclose notation='box circle'>  <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "box, kreis, einschließen 3 plus 2 i ende der einschliessung");
}

#[test]
fn menclose_left() {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "linie links, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_right() {
    let expr = "<math>
                    <menclose notation='right'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "linie rechts, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_top_bottom() {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "linie oben, unten, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_updiagonalstrike() {
    let expr = "<math>
                    <menclose notation='updiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "diagonal nach oben, durchstreichen, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_downdiagonalstrike() {
    let expr = "<math>
                    <menclose notation='downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "diagonal nach unten, durchstreichen, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_cross_out() {
    let expr = "<math>
                    <menclose notation='updiagonalstrike downdiagonalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "x, durchstreichen, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_vertical_horizontal_strike() {
    let expr = "<math>
                    <menclose notation='verticalstrike horizontalstrike'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "vertikal, horizontal, durchstreichen, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_leftarrow() {
    let expr = "<math>
                    <menclose notation='leftarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "pfeil nach links, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_right_up_down_arrow() {
    let expr = "<math>
                    <menclose notation=' rightarrow downarrow  uparrow  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "pfeil nach oben, pfeil nach unten, pfeil nach rechts, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_northeastarrow() {
    let expr = "<math>
                    <menclose notation='northeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "pfeil nach nordost, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_other_single_arrows() {
    let expr = "<math>
                    <menclose notation='northwestarrow southwestarrow southeastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "pfeil nach südost, pfeil nach südwest, pfeil nach nordwest, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_northwestsoutheastarrow() {
    let expr = "<math>
                    <menclose notation='northwestsoutheastarrow'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "doppelpfeil diagonal nach unten, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_other_double_arrows() {
    let expr = "<math>
                    <menclose notation='updownarrow leftrightarrow northeastsouthwestarrow'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "doppelpfeil nach oben und unten, doppelpfeil nach links und rechts, doppelpfeil diagonal nach oben, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_madrub() {
    let expr = "<math>
                    <menclose notation='madrub'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "arabisches faktor-symbol, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_phasorangle() {
    let expr = "<math>
                    <menclose notation='phasorangle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "phasenwinkel, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_circle_phasorangle() {
    let expr = "<math>
                    <menclose notation='phasorangle circle'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "kreis, phasenwinkel, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn menclose_longdiv() {
    let expr = "<math>
                    <menclose notation='longdiv'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "symbol für schriftliche division, einschließen 3 hälften ende der einschliessung");
}

/*
#[test]
fn menclose_longdiv_default() {
    let expr = "<math>
                    <menclose> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "symbol für schriftliche division, einschließen 3 hälften ende der einschliessung");
}


#[test]
fn menclose_longdiv_empty_string() {
    let expr = "<math>
                    <menclose notation=''> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "long division symbol, enclosing 3 halves end enclosure");
}

#[test]
fn menclose_longdiv_whitespace_string() {
    let expr = "<math>
                    <menclose notation='  '> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "long division symbol, enclosing 3 halves end enclosure");
}

 */

#[test]
fn menclose_radical() {
    let expr = "<math>
                    <menclose notation='radical'> <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "ClearSpeak", expr, "quadratwurzel, einschließen 3 hälften ende der einschliessung");
}

#[test]
fn simple_speak_menclose_top_bottom() {
    let expr = "<math>
                    <menclose notation='top bottom'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("de", "SimpleSpeak", expr, "linie oben, unten, einschließen 3 hälften ende der einschliessung");
}
