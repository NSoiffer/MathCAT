use crate::common::*;

#[test]
fn test_00() {
    let expr = "<math><mo>∗<!-- ∗ --></mo></math>";
    test_braille("Nemeth", expr, "⠈⠼");
}

#[test]
fn test_01() {
    let expr = "<math><mo>†<!-- † --></mo></math>";
    test_braille("Nemeth", expr, "⠸⠻");
}

#[test]
fn test_02() {
    let expr = "<math><mo>‡<!-- ‡ --></mo></math>";
    test_braille("Nemeth", expr, "⠸⠸⠻");
}

#[test]
fn test_03() {
    let expr = "<math><mi mathvariant=\"normal\">§<!-- § --></mi></math>";
    test_braille("Nemeth", expr, "⠈⠠⠎");
}

#[test]
fn test_04() {
    let expr = "<math><mi mathvariant=\"normal\">§<!-- § --></mi><mi mathvariant=\"normal\">§<!-- § --></mi></math>";
    test_braille("Nemeth", expr, "⠈⠠⠎⠀⠈⠠⠎");
}

#[test]
fn test_05() {
    let expr = "<math><mo>☆</mo></math>";
    test_braille("Nemeth", expr, "⠫⠎");
}

#[test]
fn test_06() {
    let expr = "<math>
        <msup><mtext>A Cantor</mtext><mo>∗<!-- ∗ --></mo></msup><mtext> set is</mtext><mo>…<!-- … --></mo></math>";
    test_braille("Nemeth", expr, "⠠⠁⠀⠠⠉⠁⠝⠞⠕⠗⠘⠈⠼⠐⠀⠎⠑⠞⠀⠊⠎⠀⠄⠄⠄");
}

#[test]
fn test_07() {
    let expr = "<math><mi>f</mi><mo>∗<!-- ∗ --></mo><mi>g</mi></math>";
    test_braille("Nemeth", expr, "⠋⠈⠼⠛");
}

#[test]
fn test_08() {
    let expr = "<math>
        <msup><mtext>Find the index</mtext><mn>1</mn></msup><mtext> of the radical.</mtext></math>";
    test_braille("Nemeth", expr, "⠠⠋⠊⠝⠙⠀⠞⠓⠑⠀⠊⠝⠙⠑⠭⠘⠂⠐⠀⠕⠋⠀⠞⠓⠑⠀⠗⠁⠙⠊⠉⠁⠇⠨");
}

#[test]
fn test_09() {
    let expr = "<math>
        <msup><mi></mi><mo>∗<!-- ∗ --></mo></msup><mtext>Irrational numbers</mtext><mo>…<!-- … --></mo></math>";
    test_braille("Nemeth", expr, "⠘⠈⠼⠐⠠⠊⠗⠗⠁⠞⠊⠕⠝⠁⠇⠀⠝⠥⠍⠃⠑⠗⠎⠀⠄⠄⠄");
}

#[test]
fn test_10() {
    let expr = "<math>
        <msup><mtext>Irrational</mtext><mo>∗<!-- ∗ --></mo></msup><mtext> numbers</mtext><mo>…<!-- … --></mo></math>";
    test_braille("Nemeth", expr, "⠠⠊⠗⠗⠁⠞⠊⠕⠝⠁⠇⠘⠈⠼⠐⠀⠝⠥⠍⠃⠑⠗⠎⠀⠄⠄⠄");
}

#[test]
fn test_11() {
    let expr = "<math>
        <msup><mi></mi><mo>∗<!-- ∗ --></mo></msup><mtext> Irrational numbers</mtext><mo>…<!-- … --></mo></math>";
    test_braille("Nemeth", expr, "⠘⠈⠼⠐⠀⠠⠊⠗⠗⠁⠞⠊⠕⠝⠁⠇⠀⠝⠥⠍⠃⠑⠗⠎⠀⠄⠄⠄");
}

#[test]
fn test_12() {
    let expr = "<math><mo>…<!-- … --></mo>
        <msup><mtext>sets.</mtext><mo>∗<!-- ∗ --></mo></msup></math>";
    test_braille("Nemeth", expr, "⠀⠄⠄⠄⠎⠑⠞⠎⠨⠈⠼");
}

#[test]
fn test_13() {
    let expr = "<math><mo>…<!-- … --></mo>
        <msup><mtext>sets</mtext><mo>∗<!-- ∗ --></mo></msup><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠀⠄⠄⠄⠀⠎⠑⠞⠎⠈⠼⠨⠐");
}

#[test]
fn test_14() {
    let expr = "<math>
        <msup><mtext>A Cantor</mtext><mn>1</mn></msup><mtext> set is</mtext><mo>…<!-- … --></mo></math>";
    test_braille("Nemeth", expr, "⠠⠁⠀⠠⠉⠁⠝⠞⠕⠗⠘⠂⠐⠀⠎⠑⠞⠀⠊⠎⠀⠄⠄⠄");
}

#[test]
fn test_15() {
    let expr = "<math><msup><mi></mi><mo>∗<!-- ∗ --></mo></msup><mn>10.</mn></math>";
    test_braille("Nemeth", expr, "⠘⠈⠼⠐⠂⠴⠨");
}

#[test]
fn test_16() {
    let expr = "<math><msup><mn>1</mn><mo>∗<!-- ∗ --></mo></msup><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠼⠂⠘⠈⠼⠨⠐");
}

#[test]
fn test_17() {
    let expr = "<math><msup><mn>1.</mn><mo>∗<!-- ∗ --></mo></msup></math>";
    test_braille("Nemeth", expr, "⠼⠂⠨⠘⠈⠼");
}

#[test]
fn test_18() {
    let expr = "<math>
        <msup><mi></mi><mo>∗<!-- ∗ --></mo></msup><mtext> For extra credit.</mtext></math>";
    test_braille("Nemeth", expr, "⠘⠈⠼⠐⠀⠠⠋⠕⠗⠀⠑⠭⠞⠗⠁⠀⠉⠗⠑⠙⠊⠞⠨");
}

#[test]
fn test_19() {
    let expr = "<math><mn>0</mn></math>";
    test_braille("Nemeth", expr, "⠼⠴");
}

#[test]
fn test_20() {
    let expr = "<math><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂");
}

#[test]
fn test_21() {
    let expr = "<math><mn>2</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆");
}

#[test]
fn test_22() {
    let expr = "<math><mn>3</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒");
}

#[test]
fn test_23() {
    let expr = "<math><mn>4</mn></math>";
    test_braille("Nemeth", expr, "⠼⠲");
}

#[test]
fn test_24() {
    let expr = "<math><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠢");
}

#[test]
fn test_25() {
    let expr = "<math><mn>6</mn></math>";
    test_braille("Nemeth", expr, "⠼⠖");
}

#[test]
fn test_26() {
    let expr = "<math><mn>7</mn></math>";
    test_braille("Nemeth", expr, "⠼⠶");
}

#[test]
fn test_27() {
    let expr = "<math><mn>8</mn></math>";
    test_braille("Nemeth", expr, "⠼⠦");
}

#[test]
fn test_28() {
    let expr = "<math><mn>9</mn></math>";
    test_braille("Nemeth", expr, "⠼⠔");
}

#[test]
fn test_29() {
    let expr = "<math><mo>,</mo></math>";
    test_braille("Nemeth", expr, "⠠");
}

#[test]
fn test_30() {
    let expr = "<math><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠨⠐");
}

#[test]
fn test_31() {
    let expr = "<math><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠨⠐");
}

#[test]
fn test_32() {
    let expr = "<math><mo>,</mo></math>";
    test_braille("Nemeth", expr, "⠠");
}

#[test]
fn test_33() {
    let expr = "<math><mn>1</mn><mo>,</mo><mn>378</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠠⠀⠼⠒⠶⠦");
}

#[test]
fn test_34() {
    let expr = "<math><mn>1.378</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠨⠒⠶⠦");
}

#[test]
fn test_35() {
    let expr = "<math><mn>3.76</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠨⠶⠖");
}

#[test]
fn test_36() {
    let expr = "<math><mn>3</mn><mo>,</mo><mn>76</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠠⠀⠼⠶⠖");
}

#[test]
fn test_37() {
    let expr = "<math><mn>1</mn><mo>,</mo><mn>478</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠠⠀⠼⠲⠶⠦");
}

#[test]
fn test_38() {
    let expr = "<math><mn>100</mn><mo>,</mo><mn>200</mn><mo>,</mo><mn>300</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠴⠴⠠⠀⠼⠆⠴⠴⠠⠀⠼⠒⠴⠴");
}

#[test]
fn test_39() {
    let expr = "<math><mn>.35</mn></math>";
    test_braille("Nemeth", expr, "⠼⠨⠒⠢");
}

#[test]
fn test_40() {
    let expr = "<math><mn>3.14</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠨⠂⠲");
}

#[test]
fn test_41() {
    let expr = "<math><mn>.2</mn><msub><mi>a</mi><mn>1</mn></msub><msub><mi>a</mi><mn>2</mn></msub><msub><mi>a</mi><mn>3</mn></msub></math>";
    test_braille("Nemeth", expr, "⠼⠨⠆⠁⠂⠁⠆⠁⠒");
}

#[test]
fn test_42() {
    let expr = "<math><mo>.</mo><msub><mi>a</mi><mn>1</mn></msub><msub><mi>a</mi><mn>2</mn></msub><msub><mi>a</mi><mn>3</mn></msub></math>";
    test_braille("Nemeth", expr, "⠨⠐⠁⠂⠁⠆⠁⠒");
}

#[test]
fn test_43() {
    let expr = "<math><mn>.1</mn><mo>+</mo><mn>.2</mn><mo>=</mo><mo>.</mo><mstyle displaystyle=\"false\" scriptlevel=\"0\"><mtext>---</mtext></mstyle></math>";
    test_braille("Nemeth", expr, "⠼⠨⠂⠬⠨⠆⠀⠨⠅⠀⠨⠐⠤⠤⠤");
}

#[test]
fn test_44() {
    let expr = "<math><mn>27</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠶");
}

#[test]
fn test_45() {
    let expr = "<math><mtext>There were </mtext><mn>7</mn><mtext> balls</mtext><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠠⠞⠓⠑⠗⠑⠀⠺⠑⠗⠑⠀⠼⠶⠀⠃⠁⠇⠇⠎⠨⠐");
}

#[test]
fn test_46() {
    let expr = "<math><mn>1</mn><mo>+</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mn>0</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠬⠭⠬⠽⠀⠨⠅⠀⠼⠴");
}

#[test]
fn test_47() {
    let expr = "<math><mi>y</mi><mo>=</mo><mn>2</mn><mi>sin</mi><mo>⁡<!-- ⁡ --></mo><mi>x</mi></math>";
    test_braille("Nemeth", expr, "⠽⠀⠨⠅⠀⠼⠆⠎⠊⠝⠀⠭");
}

#[test]
fn test_48() {
    let expr = "<math></math>";
    test_braille("Nemeth", expr, "");
}

#[test]
fn test_49() {
    let expr = "<math><mi>sin</mi><mo>⁡<!-- ⁡ --></mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠎⠊⠝⠀⠼⠂");
}

#[test]
fn test_50() {
    let expr = "<math>
        <msup><mi>sin</mi><mn>2</mn></msup><mo>⁡<!-- ⁡ --></mo><mn>2</mn><mi>x</mi></math>";
    test_braille("Nemeth", expr, "⠎⠊⠝⠘⠆⠀⠼⠆⠭");
}

#[test]
fn test_51() {
    let expr = "<math><mn>0.333</mn><mo>…<!-- … --></mo><mn>3</mn><mo>…<!-- … --></mo></math>";
    test_braille("Nemeth", expr, "⠼⠴⠨⠒⠒⠒⠀⠄⠄⠄⠀⠼⠒⠀⠄⠄⠄");
}

#[test]
fn test_52() {
    let expr = "<math><mi mathvariant=\"normal\">∠<!-- ∠ --></mi><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠫⠪⠀⠼⠂");
}

#[test]
fn test_53() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mi>x</mi><mo>=</mo><mn>0</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠭⠀⠨⠅⠀⠼⠴⠾");
}

#[test]
fn test_54() {
    let expr = "<math><mfrac><mn>11</mn><mn>5</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠂⠂⠌⠢⠼");
}

#[test]
fn test_55() {
    let expr = "<math>
        <mfrac>
        <mfrac>
        <mrow><mn>1</mn><mo>+</mo><mn>3</mn></mrow>
        <mrow><mn>4</mn><mo>+</mo><mn>5</mn></mrow></mfrac>
        <mfrac>
        <mrow><mn>3</mn><mo>+</mo><mn>4</mn></mrow>
        <mrow><mn>5</mn><mo>+</mo><mn>6</mn></mrow></mfrac></mfrac></math>";
    test_braille("Nemeth", expr, "⠠⠹⠹⠂⠬⠒⠌⠲⠬⠢⠼⠠⠌⠹⠒⠬⠲⠌⠢⠬⠖⠼⠠⠼");
}

#[test]
fn test_56() {
    let expr = "<math>
        <mfrac>
        <mfrac>
        <mrow><mo stretchy=\"false\">(</mo><mn>1</mn><mo>−<!-- − --></mo><mi>x</mi><mo stretchy=\"false\">)</mo>
        <mfrac><mi>d</mi>
        <mrow><mi>d</mi><mi>x</mi></mrow></mfrac><mo stretchy=\"false\">(</mo><mn>2</mn><mi>x</mi><mo stretchy=\"false\">)</mo><mo>−<!-- − --></mo><mn>2</mn><mi>x</mi>
        <mfrac><mi>d</mi>
        <mrow><mi>d</mi><mi>x</mi></mrow></mfrac><mo stretchy=\"false\">(</mo><mn>1</mn><mo>−<!-- − --></mo><mi>x</mi><mo stretchy=\"false\">)</mo></mrow>
        <mrow><mo stretchy=\"false\">(</mo><mn>1</mn><mo>−<!-- − --></mo><mi>x</mi>
        <msup><mo stretchy=\"false\">)</mo><mn>2</mn></msup></mrow></mfrac>
        <mrow><mn>1</mn><mo>+</mo><mo stretchy=\"false\">(</mo>
        <mfrac>
        <mrow><mn>2</mn><mi>x</mi></mrow>
        <mrow><mn>1</mn><mo>−<!-- − --></mo><mi>x</mi></mrow></mfrac>
        <msup><mo stretchy=\"false\">)</mo><mn>2</mn></msup></mrow></mfrac></math>";
    test_braille("Nemeth", expr, "⠠⠠⠹⠠⠹⠷⠂⠤⠭⠾⠹⠙⠌⠙⠭⠼⠷⠆⠭⠾⠤⠆⠭⠹⠙⠌⠙⠭⠼⠷⠂⠤⠭⠾⠠⠌⠷⠂⠤⠭⠾⠘⠆⠐⠠⠼⠠⠠⠌⠂⠬⠷⠹⠆⠭⠌⠂⠤⠭⠼⠾⠘⠆⠐⠠⠠⠼");
}

#[test]
fn test_57() {
    let expr = "<math><mo>−<!-- − --></mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠤⠼⠂");
}

#[test]
fn test_58() {
    let expr = "<math><mo>−<!-- − --></mo><mn>.3</mn></math>";
    test_braille("Nemeth", expr, "⠤⠼⠨⠒");
}

#[test]
fn test_59() {
    let expr = "<math><mo>\"</mo><mn>3</mn><mstyle displaystyle=\"false\" scriptlevel=\"0\"><mtext> dogs</mtext></mstyle><mo>\"</mo></math>";
    test_braille("Nemeth", expr, "⠄⠄⠀⠼⠒⠀⠙⠕⠛⠎⠄⠄");
}

#[test]
fn test_60() {
    let expr = "<math><mtext>Probability ---</mtext><mn>0</mn></math>";
    test_braille("Nemeth", expr, "⠠⠏⠗⠕⠃⠁⠃⠊⠇⠊⠞⠽⠀⠤⠤⠤⠼⠴");
}

#[test]
fn test_61() {
    let expr = "<math><mo>\"</mo><mn>.5</mn></math>";
    test_braille("Nemeth", expr, "⠄⠄⠼⠨⠢");
}

#[test]
fn test_62() {
    let expr = "<math><mo>\"</mo><mo>−<!-- − --></mo><mn>4</mn></math>";
    test_braille("Nemeth", expr, "⠄⠄⠤⠼⠲");
}

#[test]
fn test_63() {
    let expr = "<math>
        <mrow><mo>|</mo><mtable columnspacing=\"1em\" rowspacing=\"4pt\"><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mo>−<!-- − --></mo><mn>3</mn></mtd><mtd><mo>−<!-- − --></mo><mn>4</mn></mtd></mtr></mtable><mo>|</mo></mrow></math>";
    test_braille("Nemeth", expr, "⠠⠳⠼⠂⠀⠼⠆⠠⠳⠀⠠⠳⠤⠼⠒⠀⠤⠼⠲⠠⠳");
}

#[test]
fn test_64() {
    let expr = "<math>
        <mrow><mo>|</mo><mtable columnspacing=\"1em\" rowspacing=\"4pt\"><mtr><mtd><mn>1</mn></mtd><mtd>
        <mfrac><mn>1</mn><mn>2</mn></mfrac></mtd></mtr><mtr><mtd>
        <mfrac><mn>1</mn><mn>2</mn></mfrac></mtd><mtd>
        <mfrac><mn>1</mn>
        <mrow><mn>4</mn><mi>r</mi></mrow></mfrac></mtd></mtr></mtable><mo>|</mo></mrow></math>";
    test_braille("Nemeth", expr, "⠠⠳⠼⠂⠀⠹⠂⠌⠆⠼⠠⠳⠀⠠⠳⠹⠂⠌⠆⠼⠀⠹⠂⠌⠲⠗⠼⠠⠳");
}

#[test]
fn test_65() {
    let expr = "<math><mn>3</mn><mi mathvariant=\"normal\">§<!-- § --></mi><mn>4</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠈⠠⠎⠀⠼⠲");
}

#[test]
fn test_66() {
    let expr = "<math><mn>3</mn><mi mathvariant=\"normal\">#<!-- # --></mi><mn>4</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠨⠼⠀⠼⠲");
}

#[test]
fn test_67() {
    let expr = "<math><mn>3</mn><mo>∗<!-- ∗ --></mo><mn>4</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠈⠼⠲");
}

#[test]
fn test_68() {
    let expr = "<math><mtext>See page </mtext>
        <msup><mn>15</mn><mn>1</mn></msup><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠠⠎⠑⠑⠀⠏⠁⠛⠑⠀⠼⠂⠢⠘⠂⠨⠐");
}

#[test]
fn test_69() {
    let expr = "<math><mo>†<!-- † --></mo><mn>3</mn></math>";
    test_braille("Nemeth", expr, "⠸⠻⠼⠒");
}

#[test]
fn test_70() {
    let expr = "<math><mn mathvariant=\"italic\">3</mn></math>";
    test_braille("Nemeth", expr, "⠨⠼⠒");
}

#[test]
fn test_71() {
    let expr = "<math><mn mathvariant=\"bold\">0</mn></math>";
    test_braille("Nemeth", expr, "⠸⠼⠴");
}

#[test]
fn test_72() {
    let expr = "<math><mn mathvariant=\"italic\">.3</mn></math>";
    test_braille("Nemeth", expr, "⠨⠼⠒");
}

#[test]
fn test_73() {
    let expr = "<math><mn mathvariant=\"script\">2</mn></math>";
    test_braille("Nemeth", expr, "⠈⠼⠆");
}

#[test]
fn test_74() {
    let expr = "<math><mn mathvariant=\"bold\">43</mn><mn mathvariant=\"bold\">56</mn></math>";
    test_braille("Nemeth", expr, "⠸⠼⠲⠒⠸⠼⠢⠖");
}

#[test]
fn test_75() {
    let expr = "<math></math>";
    test_braille("Nemeth", expr, "");
}

#[test]
fn test_76() {
    let expr = "<math></math>";
    test_braille("Nemeth", expr, "");
}

#[test]
fn test_77() {
    let expr = "<math>
        <mrow>
        <mrow><mn>1</mn><mo>−<!-- − --></mo></mrow><mtext>to</mtext>
        <mrow><mo>−<!-- − --></mo><mn>1</mn></mrow><mtext> correspondence</mtext></mrow></math>";
    test_braille("Nemeth", expr, "⠼⠂⠤⠞⠕⠤⠼⠂⠀⠉⠕⠗⠗⠑⠎⠏⠕⠝⠙⠑⠝⠉⠑");
}

