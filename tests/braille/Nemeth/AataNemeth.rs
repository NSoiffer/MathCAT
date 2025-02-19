use crate::common::*;

#[test]
fn test_000() {
    let expr = "<math><mi>r</mi><mo>≡<!-- ≡ --></mo><mi>s</mi><mspace width=\"0.444em\"></mspace><mo stretchy=\"false\">(</mo><mi>mod</mi><mspace width=\"0.333em\"></mspace><mi>n</mi><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠗⠀⠸⠇⠀⠎⠷⠍⠕⠙⠀⠝⠾");
}

#[test]
fn test_001() {
    let expr = "<math><mi>y</mi><mo>=</mo>
        <msup><mi>x</mi><mi>E</mi></msup><mspace width=\"0.667em\"></mspace><mi>mod</mi><mspace width=\"thinmathspace\"></mspace><mspace width=\"thinmathspace\"></mspace><mi>n</mi></math>";
    test_braille("Nemeth", expr, "⠽⠀⠨⠅⠀⠭⠘⠠⠑⠐⠍⠕⠙⠀⠝");
}

#[test]
fn test_002() {
    let expr = "<math>
        <mrow><mo stretchy=\"false\">|</mo></mrow>
        <mrow><mover><mi>X</mi><mo>~<!-- ~ --></mo></mover></mrow>
        <mrow><mo stretchy=\"false\">|</mo></mrow><mo>=</mo>
        <msup><mn>2</mn><mn>4</mn></msup><mo>=</mo><mtext></mtext><mn>16</mn></math>";
    test_braille("Nemeth", expr, "⠳⠐⠠⠭⠣⠈⠱⠻⠳⠀⠨⠅⠀⠼⠆⠘⠲⠀⠨⠅⠀⠼⠂⠖");
}

#[test]
fn test_003() {
    let expr = "<math><mrow><mi mathvariant=\"script\">S</mi></mrow></math>";
    test_braille("Nemeth", expr, "⠈⠰⠠⠎");
}

#[test]
fn test_004() {
    let expr = "<math><mi>a</mi>
        <msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mi>b</mi><mi>x</mi><mo>+</mo><mi>c</mi><mo>=</mo><mn>0</mn></math>";
    test_braille("Nemeth", expr, "⠁⠭⠘⠆⠐⠬⠃⠭⠬⠉⠀⠨⠅⠀⠼⠴");
}

#[test]
fn test_005() {
    let expr = "<math><mi>A</mi>
        <mrow><mover>
        <mrow><mo stretchy=\"false\">→<!-- → --></mo></mrow>
        <mrow><mi>f</mi></mrow></mover></mrow><mi>B</mi></math>";
    test_braille("Nemeth", expr, "⠠⠁⠀⠐⠫⠒⠒⠕⠣⠋⠻⠀⠠⠃");
}

#[test]
fn test_006() {
    let expr = "<math><mi>g</mi><mo stretchy=\"false\">(</mo><mi>x</mi><mo stretchy=\"false\">)</mo><mo>=</mo>
        <mroot><mi>x</mi><mn>3</mn></mroot></math>";
    test_braille("Nemeth", expr, "⠛⠷⠭⠾⠀⠨⠅⠀⠣⠒⠜⠭⠻");
}

#[test]
fn test_007() {
    let expr = "<math><msubsup><mi>T</mi><mi>A</mi>
        <mrow><mo>−<!-- − --></mo><mn>1</mn></mrow></msubsup><mo>=</mo><msub><mi>T</mi>
        <mrow>
        <msup><mi>A</mi>
        <mrow><mo>−<!-- − --></mo><mn>1</mn></mrow></msup></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠠⠞⠰⠠⠁⠘⠤⠂⠀⠨⠅⠀⠠⠞⠰⠠⠁⠰⠘⠤⠂");
}

#[test]
fn test_008() {
    let expr = "<math><munder><mo>⋃<!-- ⋃ --></mo><mi>k</mi></munder><msub><mi>X</mi><mi>k</mi></msub><mo>=</mo><mi>X</mi></math>";
    test_braille("Nemeth", expr, "⠐⠨⠬⠩⠅⠻⠠⠭⠰⠅⠀⠨⠅⠀⠠⠭");
}

#[test]
fn test_009() {
    let expr = "<math><mstyle displaystyle=\"true\" scriptlevel=\"0\"><mi>f</mi><mo stretchy=\"false\">(</mo><mi>p</mi>
        <mrow><mo>/</mo></mrow><mi>q</mi><mo stretchy=\"false\">)</mo><mo>=</mo>
        <mfrac>
        <mrow><mi>p</mi><mo>+</mo><mn>1</mn></mrow>
        <mrow><mi>p</mi><mo>−<!-- − --></mo><mn>2</mn></mrow></mfrac></mstyle></math>";
    test_braille("Nemeth", expr, "⠋⠷⠏⠸⠌⠟⠾⠀⠨⠅⠀⠹⠏⠬⠂⠌⠏⠤⠆⠼");
}

#[test]
fn test_010() {
    let expr = "<math><mi>X</mi><mo>=</mo>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">N</mi></mrow></mrow><mo>∪<!-- ∪ --></mo><mo fence=\"false\" stretchy=\"false\">{</mo>
        <msqrt><mn>2</mn></msqrt><mspace width=\"thinmathspace\"></mspace><mo fence=\"false\" stretchy=\"false\">}</mo></math>";
    test_braille("Nemeth", expr, "⠠⠭⠀⠨⠅⠀⠨⠰⠠⠝⠨⠬⠨⠷⠜⠆⠻⠨⠾");
}

#[test]
fn test_011() {
    let expr = "<math><mover><mi>z</mi><mo accent=\"false\">¯<!-- ¯ --></mo></mover><mo>=</mo><mi>a</mi><mo>−<!-- − --></mo><mi>b</mi><mi>i</mi></math>";
    test_braille("Nemeth", expr, "⠵⠱⠀⠨⠅⠀⠁⠤⠃⠊");
}

#[test]
fn test_012() {
    let expr = "<math>
        <mrow><msub>
        <mrow><mi mathvariant=\"double-struck\">Z</mi></mrow><mn>8</mn></msub></mrow></math>";
    test_braille("Nemeth", expr, "⠨⠰⠠⠵⠦");
}

#[test]
fn test_013() {
    let expr = "<math><mi>C</mi><mo>=</mo>
        <mrow><mo>{</mo>
        <mrow>
        <mrow><mi mathvariant=\"bold\">y</mi></mrow></mrow><mo>:</mo><mi>G</mi>
        <mrow>
        <mrow><mi mathvariant=\"bold\">x</mi></mrow></mrow><mo>=</mo>
        <mrow>
        <mrow><mi mathvariant=\"bold\">y</mi></mrow></mrow><mtext>for</mtext>
        <mrow>
        <mrow><mi mathvariant=\"bold\">x</mi></mrow></mrow><mo>∈<!-- ∈ --></mo><msubsup>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">Z</mi></mrow></mrow><mn>2</mn><mi>k</mi></msubsup><mo>}</mo></mrow></math>";
    // corrected: added space after ":"
    test_braille("Nemeth", expr, "⠠⠉⠀⠨⠅⠀⠨⠷⠸⠰⠽⠸⠒⠀⠠⠛⠸⠰⠭⠀⠨⠅⠀⠸⠰⠽⠋⠕⠗⠸⠰⠭⠀⠈⠑⠀⠨⠰⠠⠵⠆⠘⠅⠐⠨⠾");
}

#[test]
fn test_014() {
    let expr = "<math><msqrt><mn>2</mn><mo>+</mo><msqrt><mn>3</mn></msqrt></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠆⠬⠨⠜⠒⠨⠻⠻");
}

#[test]
fn test_015() {
    let expr = "<math>
        <msqrt>
        <mroot><mn>2</mn><mn>3</mn></mroot><mo>−<!-- − --></mo><mi>i</mi></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠨⠣⠒⠜⠆⠨⠻⠤⠊⠻");
}

#[test]
fn test_016() {
    let expr = "<math>
        <msup>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">R</mi></mrow></mrow><mn>3</mn></msup></math>";
    test_braille("Nemeth", expr, "⠨⠰⠠⠗⠘⠒");
}

#[test]
fn test_017() {
    let expr = "<math><msub>
        <mrow><mover><mi>X</mi><mo>~<!-- ~ --></mo></mover></mrow>
        <mrow><mo stretchy=\"false\">(</mo><mn>1</mn><mo stretchy=\"false\">)</mo></mrow></msub><mo>=</mo>
        <mrow><mover><mi>X</mi><mo>~<!-- ~ --></mo></mover></mrow></math>";
    test_braille("Nemeth", expr, "⠐⠠⠭⠣⠈⠱⠻⠰⠷⠂⠾⠀⠨⠅⠀⠐⠠⠭⠣⠈⠱⠻");
}

#[test]
fn test_018() {
    let expr = "<math><msub><mi>G</mi><mn>0</mn></msub><mo>=</mo><mo fence=\"false\" stretchy=\"false\">{</mo><mi>A</mi><mo>:</mo><mo stretchy=\"false\">(</mo><mi>A</mi><mo>,</mo><mi>b</mi><mo stretchy=\"false\">)</mo><mo>∈<!-- ∈ --></mo><mi>G</mi><mtext>for some</mtext><mi>b</mi><mo fence=\"false\" stretchy=\"false\">}</mo></math>";
    // corrected: added space after ":"
    test_braille("Nemeth", expr, "⠠⠛⠴⠀⠨⠅⠀⠨⠷⠠⠁⠸⠒⠀⠷⠠⠁⠠⠀⠃⠾⠀⠈⠑⠀⠠⠛⠋⠕⠗⠀⠎⠕⠍⠑⠃⠨⠾");
}

#[test]
fn test_019() {
    let expr = "<math><mn>300</mn><mo>!</mo></math>";
    test_braille("Nemeth", expr, "⠼⠒⠴⠴⠯");
}

#[test]
fn test_020() {
    let expr = "<math><mi>A</mi><mo>∪<!-- ∪ --></mo><mo stretchy=\"false\">(</mo><mi>B</mi><mo>∪<!-- ∪ --></mo><mi>C</mi><mo stretchy=\"false\">)</mo><mo>=</mo><mo stretchy=\"false\">(</mo><mi>A</mi><mo>∪<!-- ∪ --></mo><mi>B</mi><mo stretchy=\"false\">)</mo><mo>∪<!-- ∪ --></mo><mi>C</mi></math>";
    test_braille("Nemeth", expr, "⠠⠁⠨⠬⠷⠠⠃⠨⠬⠠⠉⠾⠀⠨⠅⠀⠷⠠⠁⠨⠬⠠⠃⠾⠨⠬⠠⠉");
}

#[test]
fn test_021() {
    let expr = "<math>
        <mrow><mstyle scriptlevel=\"0\">
        <mrow><mo maxsize=\"1.2em\" minsize=\"1.2em\">(</mo></mrow></mstyle><mfrac linethickness=\"0\"><mi>n</mi><mi>k</mi></mfrac><mstyle scriptlevel=\"0\">
        <mrow><mo maxsize=\"1.2em\" minsize=\"1.2em\">)</mo></mrow></mstyle></mrow></math>";
    // corrected: was completely wrong (green book has same example as 90(1))
    test_braille("Nemeth", expr, "⠷⠝⠩⠅⠾");
}

#[test]
fn test_022() {
    let expr = "<math><mn>3</mn><mo>+</mo><mn>56</mn><mo>−<!-- − --></mo><mn>13</mn><mo>+</mo><mn>8</mn>
        <mrow><mo>/</mo></mrow><mn>2</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠬⠢⠖⠤⠂⠒⠬⠦⠸⠌⠆");
}

#[test]
fn test_023() {
    let expr = "<math><mi>w</mi><mo stretchy=\"false\">(</mo>
        <mrow>
        <mrow><mi mathvariant=\"bold\">x</mi></mrow></mrow><mo stretchy=\"false\">)</mo><mo>=</mo><mi>d</mi><mo stretchy=\"false\">(</mo>
        <mrow>
        <mrow><mi mathvariant=\"bold\">x</mi></mrow></mrow><mo>,</mo>
        <mrow>
        <mrow><mn mathvariant=\"bold\">0</mn></mrow></mrow><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠺⠷⠸⠰⠭⠾⠀⠨⠅⠀⠙⠷⠸⠰⠭⠠⠀⠸⠼⠴⠾");
}

#[test]
fn test_024() {
    let expr = "<math><mn>6.00000</mn><mo>+</mo><mn>0.00000</mn><mi>i</mi></math>";
    test_braille("Nemeth", expr, "⠼⠖⠨⠴⠴⠴⠴⠴⠬⠴⠨⠴⠴⠴⠴⠴⠊");
}

#[test]
fn test_025() {
    let expr = "<math><mi>p</mi><mo>=</mo><mn>0.0001</mn></math>";
    test_braille("Nemeth", expr, "⠏⠀⠨⠅⠀⠼⠴⠨⠴⠴⠴⠂");
}

#[test]
fn test_026() {
    let expr = "<math><mi>p</mi><mo>=</mo><mn>0.01</mn></math>";
    test_braille("Nemeth", expr, "⠏⠀⠨⠅⠀⠼⠴⠨⠴⠂");
}

#[test]
fn test_027() {
    let expr = "<math><mi>p</mi><mo>=</mo><mn>0.995</mn></math>";
    test_braille("Nemeth", expr, "⠏⠀⠨⠅⠀⠼⠴⠨⠔⠔⠢");
}

#[test]
fn test_028() {
    let expr = "<math><mi>p</mi><mo>=</mo><mn>0.999</mn></math>";
    test_braille("Nemeth", expr, "⠏⠀⠨⠅⠀⠼⠴⠨⠔⠔⠔");
}

#[test]
fn test_029() {
    let expr = "<math><mtext>A</mtext><mo>=</mo><mn>00</mn><mo>,</mo><mtext>B</mtext><mo>=</mo><mn>01</mn><mo>,</mo><mo>…<!-- … --></mo><mo>,</mo><mtext>Z</mtext><mo>=</mo><mn>25</mn></math>";
    test_braille("Nemeth", expr, "⠠⠁⠀⠨⠅⠀⠼⠴⠴⠠⠀⠠⠃⠀⠨⠅⠀⠼⠴⠂⠠⠀⠄⠄⠄⠠⠀⠠⠵⠀⠨⠅⠀⠼⠆⠢");
}

#[test]
fn test_030() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>000</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠴⠾");
}

#[test]
fn test_031() {
    let expr = "<math><mn>0000</mn></math>";
    test_braille("Nemeth", expr, "⠼⠴⠴⠴⠴");
}

#[test]
fn test_032() {
    let expr = "<math><msub>
        <mrow>
        <mrow><mi mathvariant=\"bold\">c</mi></mrow></mrow><mn>1</mn></msub><mo>=</mo><mo stretchy=\"false\">(</mo><mn>00000</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠸⠰⠉⠂⠀⠨⠅⠀⠷⠴⠴⠴⠴⠴⠾");
}

#[test]
fn test_033() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>00001</mn><mo stretchy=\"false\">)</mo><mo>+</mo><mi>C</mi></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠴⠴⠂⠾⠬⠠⠉");
}

#[test]
fn test_034() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>0000101100</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠴⠴⠂⠴⠂⠂⠴⠴⠾");
}

#[test]
fn test_035() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>00010</mn><mo stretchy=\"false\">)</mo><mo>+</mo><mi>C</mi></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠴⠂⠴⠾⠬⠠⠉");
}

#[test]
fn test_036() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">z</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><mn>00011</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠸⠰⠵⠀⠨⠅⠀⠷⠴⠴⠴⠂⠂⠾");
}

#[test]
fn test_037() {
    let expr = "<math><mn>001</mn></math>";
    test_braille("Nemeth", expr, "⠼⠴⠴⠂");
}

#[test]
fn test_038() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>00100</mn><mo stretchy=\"false\">)</mo><mo>+</mo><mi>C</mi></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠂⠴⠴⠾⠬⠠⠉");
}

#[test]
fn test_039() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>0010000101</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠂⠴⠴⠴⠴⠂⠴⠂⠾");
}

#[test]
fn test_040() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>001001</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠂⠴⠴⠂⠾");
}

#[test]
fn test_041() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>01000</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>00101</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>11011</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>10110</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠂⠴⠴⠴⠾⠷⠴⠴⠂⠴⠂⠾⠷⠂⠂⠴⠂⠂⠾⠷⠂⠴⠂⠂⠴⠾");
}

#[test]
fn test_042() {
    let expr = "<math><mn>0011</mn></math>";
    test_braille("Nemeth", expr, "⠼⠴⠴⠂⠂");
}

#[test]
fn test_043() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>00110</mn><mo stretchy=\"false\">)</mo><mo>+</mo><mi>C</mi></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠂⠂⠴⠾⠬⠠⠉");
}

#[test]
fn test_044() {
    let expr = "<math><msub>
        <mrow>
        <mrow><mi mathvariant=\"bold\">c</mi></mrow></mrow><mn>2</mn></msub><mo>=</mo><mo stretchy=\"false\">(</mo><mn>00111</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠸⠰⠉⠆⠀⠨⠅⠀⠷⠴⠴⠂⠂⠂⠾");
}

#[test]
fn test_045() {
    let expr = "<math><mn>010</mn></math>";
    test_braille("Nemeth", expr, "⠼⠴⠂⠴");
}

#[test]
fn test_046() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>0100</mn><mspace width=\"thickmathspace\"></mspace><mn>0101</mn><mo stretchy=\"false\">)</mo></math>";
    // Corrected: no numeric indicators should be used after space as this is a single number; also none after paren
    test_braille("Nemeth", expr, "⠷⠴⠂⠴⠴⠀⠴⠂⠴⠂⠾");
}

#[test]
fn test_047() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>01000</mn><mo stretchy=\"false\">)</mo><mo>+</mo><mi>C</mi></math>";
    test_braille("Nemeth", expr, "⠷⠴⠂⠴⠴⠴⠾⠬⠠⠉");
}

#[test]
fn test_048() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>00100</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>01001</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>10111</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>11010</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠂⠴⠴⠾⠷⠴⠂⠴⠴⠂⠾⠷⠂⠴⠂⠂⠂⠾⠷⠂⠂⠴⠂⠴⠾");
}

#[test]
fn test_049() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">x</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><mn>010011</mn>
        <msup><mo stretchy=\"false\">)</mo><mtext>t</mtext></msup></math>";
    test_braille("Nemeth", expr, "⠸⠰⠭⠀⠨⠅⠀⠷⠴⠂⠴⠴⠂⠂⠾⠘⠞");
}

#[test]
fn test_050() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>00111</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>01010</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>10100</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>11001</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠂⠂⠂⠾⠷⠴⠂⠴⠂⠴⠾⠷⠂⠴⠂⠴⠴⠾⠷⠂⠂⠴⠴⠂⠾");
}

#[test]
fn test_051() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>011100</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>011011</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>111011</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>100011</mn><mo stretchy=\"false\">)</mo><mspace linebreak=\"newline\"></mspace><mo stretchy=\"false\">(</mo><mn>000000</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>010101</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>110100</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>110011</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠂⠂⠂⠴⠴⠾⠷⠴⠂⠂⠴⠂⠂⠾⠷⠂⠂⠂⠴⠂⠂⠾⠷⠂⠴⠴⠴⠂⠂⠾⠷⠴⠴⠴⠴⠴⠴⠾⠷⠴⠂⠴⠂⠴⠂⠾⠷⠂⠂⠴⠂⠴⠴⠾⠷⠂⠂⠴⠴⠂⠂⠾");
}

#[test]
fn test_052() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>11110101</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>01010100</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠂⠂⠂⠴⠂⠴⠂⠾⠠⠀⠷⠴⠂⠴⠂⠴⠂⠴⠴⠾");
}

#[test]
fn test_053() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">z</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><mn>010111</mn>
        <msup><mo stretchy=\"false\">)</mo><mtext>t</mtext></msup></math>";
    test_braille("Nemeth", expr, "⠸⠰⠵⠀⠨⠅⠀⠷⠴⠂⠴⠂⠂⠂⠾⠘⠞");
}

#[test]
fn test_054() {
    let expr = "<math><mn>011</mn></math>";
    test_braille("Nemeth", expr, "⠼⠴⠂⠂");
}

#[test]
fn test_055() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>0110</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠂⠂⠴⠾");
}

#[test]
fn test_056() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>00001</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>01100</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>10010</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>11111</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠴⠴⠂⠾⠷⠴⠂⠂⠴⠴⠾⠷⠂⠴⠴⠂⠴⠾⠷⠂⠂⠂⠂⠂⠾");
}

#[test]
fn test_057() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">x</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><mn>011001</mn>
        <msup><mo stretchy=\"false\">)</mo><mtext>t</mtext></msup></math>";
    test_braille("Nemeth", expr, "⠸⠰⠭⠀⠨⠅⠀⠷⠴⠂⠂⠴⠴⠂⠾⠘⠞");
}

#[test]
fn test_058() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>011010</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>011100</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠂⠂⠴⠂⠴⠾⠠⠀⠷⠴⠂⠂⠂⠴⠴⠾");
}

#[test]
fn test_059() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>0110110</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>0111100</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>1110000</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>1111111</mn><mo stretchy=\"false\">)</mo><mspace linebreak=\"newline\"></mspace><mo stretchy=\"false\">(</mo><mn>1001001</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>1000011</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>0001111</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>0000000</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠂⠂⠴⠂⠂⠴⠾⠷⠴⠂⠂⠂⠂⠴⠴⠾⠷⠂⠂⠂⠴⠴⠴⠴⠾⠷⠂⠂⠂⠂⠂⠂⠂⠾⠷⠂⠴⠴⠂⠴⠴⠂⠾⠷⠂⠴⠴⠴⠴⠂⠂⠾⠷⠴⠴⠴⠂⠂⠂⠂⠾⠷⠴⠴⠴⠴⠴⠴⠴⠾");
}

#[test]
fn test_060() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>1001</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>0111</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠴⠴⠂⠾⠠⠀⠷⠴⠂⠂⠂⠾");
}

#[test]
fn test_061() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>10000</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>11101</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>00011</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>01110</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠴⠴⠴⠴⠾⠷⠂⠂⠂⠴⠂⠾⠷⠴⠴⠴⠂⠂⠾⠷⠴⠂⠂⠂⠴⠾");
}

#[test]
fn test_062() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>00010</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>01111</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>10001</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>11100</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠴⠂⠴⠾⠷⠴⠂⠂⠂⠂⠾⠷⠂⠴⠴⠴⠂⠾⠷⠂⠂⠂⠴⠴⠾");
}

#[test]
fn test_063() {
    let expr = "<math><mtext>A</mtext><mo>=</mo><mn>00</mn><mo>,</mo><mtext>B</mtext><mo>=</mo><mn>02</mn><mo>,</mo><mo>…<!-- … --></mo><mo>,</mo><mtext>Z</mtext><mo>=</mo><mn>25</mn></math>";
    test_braille("Nemeth", expr, "⠠⠁⠀⠨⠅⠀⠼⠴⠴⠠⠀⠠⠃⠀⠨⠅⠀⠼⠴⠆⠠⠀⠄⠄⠄⠠⠀⠠⠵⠀⠨⠅⠀⠼⠆⠢");
}

#[test]
fn test_064() {
    let expr = "<math><mi>c</mi><mo>=</mo><mn>4</mn><mspace width=\"thinmathspace\"></mspace><mn>598</mn><mspace width=\"thinmathspace\"></mspace><mn>037</mn><mspace width=\"thinmathspace\"></mspace><mn>234</mn></math>";
    test_braille("Nemeth", expr, "⠉⠀⠨⠅⠀⠼⠲⠀⠢⠔⠦⠀⠴⠒⠶⠀⠆⠒⠲");
}

#[test]
fn test_065() {
    let expr = "<math><mtext>E</mtext><mo>=</mo><mn>04</mn></math>";
    test_braille("Nemeth", expr, "⠠⠑⠀⠨⠅⠀⠼⠴⠲");
}

#[test]
fn test_066() {
    let expr = "<math><mn>10</mn><mrow><mo>/</mo></mrow><mn>5</mn><mo>=</mo><mn>2</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠴⠸⠌⠢⠀⠨⠅⠀⠼⠆");
}

#[test]
fn test_067() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>1254</mn>
        <msup><mo stretchy=\"false\">)</mo>
        <mrow><mn>100</mn></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠷⠂⠆⠢⠲⠾⠘⠂⠴⠴");
}

#[test]
fn test_068() {
    let expr = "<math><mn>1000</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠴⠴⠴");
}

#[test]
fn test_069() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>10000</mn><mo stretchy=\"false\">)</mo><mo>+</mo><mi>C</mi></math>";
    test_braille("Nemeth", expr, "⠷⠂⠴⠴⠴⠴⠾⠬⠠⠉");
}

#[test]
fn test_070() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>1001</mn><mspace width=\"thickmathspace\"></mspace><mn>1000</mn><mo stretchy=\"false\">)</mo></math>";
    // Corrected: no numeric indicators should be used after space as this is a single number; also none after paren
    test_braille("Nemeth", expr, "⠷⠂⠴⠴⠂⠀⠂⠴⠴⠴⠾");
}

#[test]
fn test_071() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>101</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠴⠂⠾");
}

#[test]
fn test_072() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">y</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><mn>1010</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠸⠰⠽⠀⠨⠅⠀⠷⠂⠴⠂⠴⠾");
}

#[test]
fn test_073() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>10100</mn><mo stretchy=\"false\">)</mo><mo>+</mo><mi>C</mi></math>";
    test_braille("Nemeth", expr, "⠷⠂⠴⠂⠴⠴⠾⠬⠠⠉");
}

#[test]
fn test_074() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">x</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><mn>10101</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠸⠰⠭⠀⠨⠅⠀⠷⠂⠴⠂⠴⠂⠾");
}

#[test]
fn test_075() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>101011</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠴⠂⠴⠂⠂⠾");
}

#[test]
fn test_076() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>1011</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠴⠂⠂⠾");
}

#[test]
fn test_077() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>000000</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>010111</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>101101</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>111010</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠴⠴⠴⠴⠾⠠⠀⠷⠴⠂⠴⠂⠂⠂⠾⠠⠀⠷⠂⠴⠂⠂⠴⠂⠾⠠⠀⠷⠂⠂⠂⠴⠂⠴⠾");
}

#[test]
fn test_078() {
    let expr = "<math><mo form=\"prefix\" movablelimits=\"true\">gcd</mo><mo stretchy=\"false\">(</mo><mn>120</mn><mo>,</mo><mn>102</mn><mo stretchy=\"false\">)</mo><mo>=</mo><mn>6</mn></math>";
    test_braille("Nemeth", expr, "⠛⠉⠙⠀⠷⠂⠆⠴⠠⠀⠂⠴⠆⠾⠀⠨⠅⠀⠼⠖");
}

#[test]
fn test_079() {
    let expr = "<math><mn>108</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠴⠦");
}

#[test]
fn test_080() {
    let expr = "<math><mn>110</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠂⠴");
}

#[test]
fn test_081() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">y</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><mn>1100</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠸⠰⠽⠀⠨⠅⠀⠷⠂⠂⠴⠴⠾");
}

#[test]
fn test_082() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>00110</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>01011</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>10101</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>11000</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠂⠂⠴⠾⠷⠴⠂⠴⠂⠂⠾⠷⠂⠴⠂⠴⠂⠾⠷⠂⠂⠴⠴⠴⠾");
}

#[test]
fn test_083() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>011010</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>011100</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>110111</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>110000</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠂⠂⠴⠂⠴⠾⠷⠴⠂⠂⠂⠴⠴⠾⠷⠂⠂⠴⠂⠂⠂⠾⠷⠂⠂⠴⠴⠴⠴⠾");
}

#[test]
fn test_084() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>000000</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>011100</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>110101</mn><mo stretchy=\"false\">)</mo><mspace width=\"thickmathspace\"></mspace><mo stretchy=\"false\">(</mo><mn>110001</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠴⠴⠴⠴⠴⠴⠾⠷⠴⠂⠂⠂⠴⠴⠾⠷⠂⠂⠴⠂⠴⠂⠾⠷⠂⠂⠴⠴⠴⠂⠾");
}

#[test]
fn test_085() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">x</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><mn>1101</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠸⠰⠭⠀⠨⠅⠀⠷⠂⠂⠴⠂⠾");
}

#[test]
fn test_086() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">y</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><mn>11010</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠸⠰⠽⠀⠨⠅⠀⠷⠂⠂⠴⠂⠴⠾");
}

#[test]
fn test_087() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">y</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><mn>110101</mn>
        <msup><mo stretchy=\"false\">)</mo><mtext>t</mtext></msup></math>";
    test_braille("Nemeth", expr, "⠸⠰⠽⠀⠨⠅⠀⠷⠂⠂⠴⠂⠴⠂⠾⠘⠞");
}

#[test]
fn test_088() {
    let expr = "<math><msub>
        <mrow>
        <mrow><mi mathvariant=\"bold\">c</mi></mrow></mrow><mn>4</mn></msub><mo>=</mo><mo stretchy=\"false\">(</mo><mn>11011</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠸⠰⠉⠲⠀⠨⠅⠀⠷⠂⠂⠴⠂⠂⠾");
}

#[test]
fn test_089() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>110110</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠂⠴⠂⠂⠴⠾");
}

#[test]
fn test_090() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>111</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠂⠂⠾");
}

#[test]
fn test_091() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>0110</mn><mspace width=\"thickmathspace\"></mspace><mn>1110</mn><mspace width=\"thickmathspace\"></mspace><mn>0110</mn><mo stretchy=\"false\">)</mo></math>";
    // Corrected: no numeric indicators should be used after space as this is a single number; also none after paren
    test_braille("Nemeth", expr, "⠷⠴⠂⠂⠴⠀⠂⠂⠂⠴⠀⠴⠂⠂⠴⠾");
}

#[test]
fn test_092() {
    let expr = "<math><msub>
        <mrow>
        <mrow><mi mathvariant=\"bold\">c</mi></mrow></mrow><mn>3</mn></msub><mo>=</mo><mo stretchy=\"false\">(</mo><mn>11100</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠸⠰⠉⠒⠀⠨⠅⠀⠷⠂⠂⠂⠴⠴⠾");
}

#[test]
fn test_093() {
    let expr = "<math><mn>1111</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠂⠂⠂");
}

#[test]
fn test_094() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">x</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><mn>111110</mn>
        <msup><mo stretchy=\"false\">)</mo><mtext>t</mtext></msup></math>";
    test_braille("Nemeth", expr, "⠸⠰⠭⠀⠨⠅⠀⠷⠂⠂⠂⠂⠂⠴⠾⠘⠞");
}

#[test]
fn test_095() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">y</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><mn>111111</mn>
        <msup><mo stretchy=\"false\">)</mo><mtext>t</mtext></msup></math>";
    test_braille("Nemeth", expr, "⠸⠰⠽⠀⠨⠅⠀⠷⠂⠂⠂⠂⠂⠂⠾⠘⠞");
}

#[test]
fn test_096() {
    let expr = "<math><mn>112135</mn><mn>25032</mn><mn>442</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠂⠆⠂⠒⠢⠆⠢⠴⠒⠆⠲⠲⠆");
}

#[test]
fn test_097() {
    let expr = "<math>
        <msup><mn>7</mn><mn>6</mn></msup><mo>=</mo><mn>117</mn><mspace width=\"thinmathspace\"></mspace><mn>649</mn></math>";
    test_braille("Nemeth", expr, "⠼⠶⠘⠖⠀⠨⠅⠀⠼⠂⠂⠶⠀⠖⠲⠔");
}

#[test]
fn test_098() {
    let expr = "<math><mo form=\"prefix\" movablelimits=\"true\">gcd</mo><mo stretchy=\"false\">(</mo><mn>24</mn><mo>,</mo><mn>36</mn><mo stretchy=\"false\">)</mo><mo>=</mo><mn>12</mn></math>";
    test_braille("Nemeth", expr, "⠛⠉⠙⠀⠷⠆⠲⠠⠀⠒⠖⠾⠀⠨⠅⠀⠼⠂⠆");
}

#[test]
fn test_099() {
    let expr = "<math><mi>n</mi><mo>=</mo><mn>120979</mn><mo>,</mo><mi>E</mi><mo>=</mo><mn>13251</mn><mo>,</mo><mi>x</mi><mo>=</mo><mn>142371</mn></math>";
    test_braille("Nemeth", expr, "⠝⠀⠨⠅⠀⠼⠂⠆⠴⠔⠶⠔⠠⠀⠠⠑⠀⠨⠅⠀⠼⠂⠒⠆⠢⠂⠠⠀⠭⠀⠨⠅⠀⠼⠂⠲⠆⠒⠶⠂");
}

#[test]
fn test_100() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>12345</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>678</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠆⠒⠲⠢⠾⠷⠖⠶⠦⠾");
}

#[test]
fn test_101() {
    let expr = "<math><mo stretchy=\"false\">[</mo><mo stretchy=\"false\">(</mo><mn>1235</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>467</mn><mo stretchy=\"false\">)</mo>
        <msup><mo stretchy=\"false\">]</mo>
        <mrow><mo>−<!-- − --></mo><mn>1</mn></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠈⠷⠷⠂⠆⠒⠢⠾⠷⠲⠖⠶⠾⠈⠾⠘⠤⠂");
}

#[test]
fn test_102() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mi>n</mi><mo>,</mo><mi>E</mi><mo stretchy=\"false\">)</mo><mo>=</mo><mo stretchy=\"false\">(</mo><mn>37986733</mn><mo>,</mo><mn>12371</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠝⠠⠀⠠⠑⠾⠀⠨⠅⠀⠷⠒⠶⠔⠦⠖⠶⠒⠒⠠⠀⠂⠆⠒⠶⠂⠾");
}

#[test]
fn test_103() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>12453</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠆⠲⠢⠒⠾");
}

#[test]
fn test_104() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>12</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>1253</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠆⠾⠷⠂⠆⠢⠒⠾");
}

#[test]
fn test_105() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>12537</mn>
        <msup><mo stretchy=\"false\">)</mo>
        <mrow><mo>−<!-- − --></mo><mn>1</mn></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠷⠂⠆⠢⠒⠶⠾⠘⠤⠂");
}

#[test]
fn test_106() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>1254</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>13</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>25</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠆⠢⠲⠾⠷⠂⠒⠾⠷⠆⠢⠾");
}

#[test]
fn test_107() {
    let expr = "<math><mn>1260</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠆⠖⠴");
}

#[test]
fn test_108() {
    let expr = "<math>
        <msup><mn>128</mn><mn>4</mn></msup><mo>=</mo><mn>268</mn><mo>,</mo><mn>435</mn><mo>,</mo><mn>456</mn></math>";
    // corrected to remove spaces and numeric indicators
    test_braille("Nemeth", expr, "⠼⠂⠆⠦⠘⠲⠀⠨⠅⠀⠼⠆⠖⠦⠠⠲⠒⠢⠠⠲⠢⠖");
}

#[test]
fn test_109() {
    let expr = "<math><mi>n</mi><mo>=</mo><mn>79403</mn><mo>,</mo><mi>D</mi><mo>=</mo><mn>671</mn><mo>,</mo><mi>y</mi><mo>=</mo><mn>129381</mn></math>";
    test_braille("Nemeth", expr, "⠝⠀⠨⠅⠀⠼⠶⠔⠲⠴⠒⠠⠀⠠⠙⠀⠨⠅⠀⠼⠖⠶⠂⠠⠀⠽⠀⠨⠅⠀⠼⠂⠆⠔⠒⠦⠂");
}

#[test]
fn test_110() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>1423</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>34</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>56</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>1324</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠲⠆⠒⠾⠷⠒⠲⠾⠷⠢⠖⠾⠷⠂⠒⠆⠲⠾");
}

#[test]
fn test_111() {
    let expr = "<math><mo fence=\"false\" stretchy=\"false\">{</mo><mo stretchy=\"false\">(</mo><mn>13</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>13</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>24</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>132</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>134</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>1324</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>1342</mn><mo stretchy=\"false\">)</mo><mo fence=\"false\" stretchy=\"false\">}</mo></math>";
    test_braille("Nemeth", expr, "⠨⠷⠷⠂⠒⠾⠠⠀⠷⠂⠒⠾⠷⠆⠲⠾⠠⠀⠷⠂⠒⠆⠾⠠⠀⠷⠂⠒⠲⠾⠠⠀⠷⠂⠒⠆⠲⠾⠠⠀⠷⠂⠒⠲⠆⠾⠨⠾");
}

#[test]
fn test_112() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>1345</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>234</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠒⠲⠢⠾⠷⠆⠒⠲⠾");
}

#[test]
fn test_113() {
    let expr = "<math><mn>14</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠲");
}

#[test]
fn test_114() {
    let expr = "<math><mi>x</mi><mo>=</mo><mn>142528</mn></math>";
    test_braille("Nemeth", expr, "⠭⠀⠨⠅⠀⠼⠂⠲⠆⠢⠆⠦");
}

#[test]
fn test_115() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>1426</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>142</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠲⠆⠖⠾⠷⠂⠲⠆⠾");
}

#[test]
fn test_116() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>142637</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠲⠆⠖⠒⠶⠾");
}

#[test]
fn test_117() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>14356</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠲⠒⠢⠖⠾");
}

#[test]
fn test_118() {
    let expr = "<math><mn>191</mn><mi>E</mi><mo>=</mo><mn>1</mn><mo>+</mo><mn>151</mn><mi>m</mi></math>";
    test_braille("Nemeth", expr, "⠼⠂⠔⠂⠠⠑⠀⠨⠅⠀⠼⠂⠬⠂⠢⠂⠍");
}

#[test]
fn test_119() {
    let expr = "<math><mn>1523</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠢⠆⠒");
}

#[test]
fn test_120() {
    let expr = "<math><mn>1531</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠢⠒⠂");
}

#[test]
fn test_121() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>17254</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>1423</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>154632</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠶⠆⠢⠲⠾⠷⠂⠲⠆⠒⠾⠷⠂⠢⠲⠖⠒⠆⠾");
}

#[test]
fn test_122() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>156</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>234</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠢⠖⠾⠷⠆⠒⠲⠾");
}

#[test]
fn test_123() {
    let expr = "<math><mi>s</mi><mo>=</mo><mo>−<!-- − --></mo><mn>16</mn></math>";
    test_braille("Nemeth", expr, "⠎⠀⠨⠅⠀⠤⠼⠂⠖");
}

#[test]
fn test_124() {
    let expr = "<math><mrow><mn mathvariant=\"bold\">16</mn></mrow></math>";
    test_braille("Nemeth", expr, "⠸⠼⠂⠖");
}

#[test]
fn test_125() {
    let expr = "<math><mn>160</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠖⠴");
}

#[test]
fn test_126() {
    let expr = "<math><mi>μ<!-- μ --></mi><mo>=</mo><mo stretchy=\"false\">(</mo><mn>1634</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠨⠍⠀⠨⠅⠀⠷⠂⠖⠒⠲⠾");
}

#[test]
fn test_127() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mi>n</mi><mo>,</mo><mi>E</mi><mo stretchy=\"false\">)</mo><mo>=</mo><mo stretchy=\"false\">(</mo><mn>16394854313</mn><mo>,</mo><mn>34578451</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠝⠠⠀⠠⠑⠾⠀⠨⠅⠀⠷⠂⠖⠒⠔⠲⠦⠢⠲⠒⠂⠒⠠⠀⠒⠲⠢⠶⠦⠲⠢⠂⠾");
}

#[test]
fn test_128() {
    let expr = "<math><mn>5</mn><mo>⋅<!-- ⋅ --></mo><mn>7</mn><mo>⋅<!-- ⋅ --></mo><mn>47</mn><mo>=</mo><mn>1645</mn></math>";
    test_braille("Nemeth", expr, "⠼⠢⠡⠶⠡⠲⠶⠀⠨⠅⠀⠼⠂⠖⠲⠢");
}

#[test]
fn test_129() {
    let expr = "<math><mn>165</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠖⠢");
}

#[test]
fn test_130() {
    let expr = "<math><mn>168</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠖⠦");
}

#[test]
fn test_131() {
    let expr = "<math><mn>41</mn><mo>≡<!-- ≡ --></mo><mn>17</mn><mspace width=\"0.444em\"></mspace><mo stretchy=\"false\">(</mo><mi>mod</mi><mspace width=\"0.333em\"></mspace><mn>8</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠼⠲⠂⠀⠸⠇⠀⠼⠂⠶⠷⠍⠕⠙⠀⠼⠦⠾");
}

#[test]
fn test_132() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>17352</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠶⠒⠢⠆⠾");
}

#[test]
fn test_133() {
    let expr = "<math><mn>1739</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠶⠒⠔");
}

#[test]
fn test_134() {
    let expr = "<math><mn>175</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠶⠢");
}

#[test]
fn test_135() {
    let expr = "<math><msup><mn>180</mn><mrow><mo>∘<!-- ∘ --></mo></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠼⠂⠦⠴⠘⠨⠡");
}

#[test]
fn test_136() {
    let expr = "<math><mn>19</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠔");
}

#[test]
fn test_137() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mi>n</mi><mo>,</mo><mi>E</mi><mo stretchy=\"false\">)</mo><mo>=</mo><mo stretchy=\"false\">(</mo><mn>3053</mn><mo>,</mo><mn>1921</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠝⠠⠀⠠⠑⠾⠀⠨⠅⠀⠷⠒⠴⠢⠒⠠⠀⠂⠔⠆⠂⠾");
}

#[test]
fn test_138() {
    let expr = "<math><mn>196,833</mn><mo>×<!-- × --></mo><mn>196,833</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠔⠖⠠⠦⠒⠒⠈⠡⠂⠔⠖⠠⠦⠒⠒");
}

#[test]
fn test_139() {
    let expr = "<math><mn>19945</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠔⠔⠲⠢");
}

#[test]
fn test_140() {
    let expr = "<math><mi>n</mi><mo>=</mo><mn>3551</mn><mo>,</mo><mi>D</mi><mo>=</mo><mn>1997</mn><mo>,</mo><mi>y</mi><mo>=</mo><mn>2791</mn></math>";
    test_braille("Nemeth", expr, "⠝⠀⠨⠅⠀⠼⠒⠢⠢⠂⠠⠀⠠⠙⠀⠨⠅⠀⠼⠂⠔⠔⠶⠠⠀⠽⠀⠨⠅⠀⠼⠆⠶⠔⠂");
}

#[test]
fn test_141() {
    let expr = "<math><mn>200</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠴⠴");
}

#[test]
fn test_142() {
    let expr = "<math><mn>2000</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠴⠴⠴");
}

#[test]
fn test_143() {
    let expr = "<math>
        <msup><mn>2071</mn>
        <mrow><mn>9521</mn></mrow></msup><mspace width=\"0.444em\"></mspace><mo stretchy=\"false\">(</mo><mi>mod</mi><mspace width=\"0.333em\"></mspace><mn>4724</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠼⠆⠴⠶⠂⠘⠔⠢⠆⠂⠐⠷⠍⠕⠙⠀⠼⠲⠶⠆⠲⠾");
}

#[test]
fn test_144() {
    let expr = "<math><mn>2134</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠂⠒⠲");
}

#[test]
fn test_145() {
    let expr = "<math><mi>x</mi><mo>≡<!-- ≡ --></mo><mn>214</mn><mspace width=\"0.444em\"></mspace><mo stretchy=\"false\">(</mo><mi>mod</mi><mspace width=\"0.333em\"></mspace><mn>2772</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠭⠀⠸⠇⠀⠼⠆⠂⠲⠷⠍⠕⠙⠀⠼⠆⠶⠶⠆⠾");
}

#[test]
fn test_146() {
    let expr = "<math><mn>2234</mn><mo>+</mo><mn>4121</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠆⠒⠲⠬⠲⠂⠆⠂");
}

#[test]
fn test_147() {
    let expr = "<math><mi>n</mi><mo>=</mo><mn>2257</mn><mo>,</mo><mi>E</mi><mo>=</mo><mn>47</mn><mo>,</mo><mi>x</mi><mo>=</mo><mn>23</mn></math>";
    test_braille("Nemeth", expr, "⠝⠀⠨⠅⠀⠼⠆⠆⠢⠶⠠⠀⠠⠑⠀⠨⠅⠀⠼⠲⠶⠠⠀⠭⠀⠨⠅⠀⠼⠆⠒");
}

#[test]
fn test_148() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mi>n</mi><mo>,</mo><mi>E</mi><mo stretchy=\"false\">)</mo><mo>=</mo><mo stretchy=\"false\">(</mo><mn>451</mn><mo>,</mo><mn>231</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠝⠠⠀⠠⠑⠾⠀⠨⠅⠀⠷⠲⠢⠂⠠⠀⠆⠒⠂⠾");
}

#[test]
fn test_149() {
    let expr = "<math><mi>n</mi><mo>=</mo><mn>45629</mn><mo>,</mo><mi>E</mi><mo>=</mo><mn>781</mn><mo>,</mo><mi>x</mi><mo>=</mo><mn>231561</mn></math>";
    test_braille("Nemeth", expr, "⠝⠀⠨⠅⠀⠼⠲⠢⠖⠆⠔⠠⠀⠠⠑⠀⠨⠅⠀⠼⠶⠦⠂⠠⠀⠭⠀⠨⠅⠀⠼⠆⠒⠂⠢⠖⠂");
}

#[test]
fn test_150() {
    let expr = "<math><mn>234</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠒⠲");
}

#[test]
fn test_151() {
    let expr = "<math><mn>23771</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠒⠶⠶⠂");
}

#[test]
fn test_152() {
    let expr = "<math><mn>41</mn><mo>−<!-- − --></mo><mn>17</mn><mo>=</mo><mn>24</mn></math>";
    test_braille("Nemeth", expr, "⠼⠲⠂⠤⠂⠶⠀⠨⠅⠀⠼⠆⠲");
}

#[test]
fn test_153() {
    let expr = "<math><mn>72</mn><mo>∈<!-- ∈ --></mo><msub>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">Z</mi></mrow></mrow>
        <mrow><mn>240</mn></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠼⠶⠆⠀⠈⠑⠀⠨⠰⠠⠵⠆⠲⠴");
}

#[test]
fn test_154() {
    let expr = "<math><mn>2415</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠲⠂⠢");
}

#[test]
fn test_155() {
    let expr = "<math><mi>N</mi><mo>=</mo><mn>250</mn></math>";
    test_braille("Nemeth", expr, "⠠⠝⠀⠨⠅⠀⠼⠆⠢⠴");
}

#[test]
fn test_156() {
    let expr = "<math><mn>255</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠢⠢");
}

#[test]
fn test_157() {
    let expr = "<math>
        <msup><mn>2557</mn>
        <mrow><mn>341</mn></mrow></msup><mspace width=\"0.444em\"></mspace><mo stretchy=\"false\">(</mo><mi>mod</mi><mspace width=\"0.333em\"></mspace><mn>5681</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠼⠆⠢⠢⠶⠘⠒⠲⠂⠐⠷⠍⠕⠙⠀⠼⠢⠖⠦⠂⠾");
}

#[test]
fn test_158() {
    let expr = "<math>
        <msup><mn>2</mn>
        <mrow><mn>8</mn></mrow></msup><mo>=</mo><mn>256</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠘⠦⠀⠨⠅⠀⠼⠆⠢⠖");
}

#[test]
fn test_159() {
    let expr = "<math><mn>5</mn><mi>x</mi><mo>+</mo><mn>1</mn><mo>≡<!-- ≡ --></mo><mn>13</mn><mspace width=\"0.444em\"></mspace><mo stretchy=\"false\">(</mo><mi>mod</mi><mspace width=\"0.333em\"></mspace><mn>26</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠼⠢⠭⠬⠂⠀⠸⠇⠀⠼⠂⠒⠷⠍⠕⠙⠀⠼⠆⠖⠾");
}

#[test]
fn test_160() {
    let expr = "<math><mn>2600</mn><mo>=</mo>
        <msup><mn>2</mn><mn>3</mn></msup><mo>×<!-- × --></mo>
        <msup><mn>5</mn><mn>2</mn></msup><mo>×<!-- × --></mo><mn>13</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠖⠴⠴⠀⠨⠅⠀⠼⠆⠘⠒⠐⠈⠡⠢⠘⠆⠐⠈⠡⠂⠒");
}

#[test]
fn test_161() {
    let expr = "<math><msub>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">Z</mi></mrow></mrow>
        <mrow><mn>27</mn></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠨⠰⠠⠵⠆⠶");
}

#[test]
fn test_162() {
    let expr = "<math>
        <msup><mn>271</mn>
        <mrow><mn>321</mn></mrow></msup><mspace width=\"0.444em\"></mspace><mo stretchy=\"false\">(</mo><mi>mod</mi><mspace width=\"0.333em\"></mspace><mn>481</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠼⠆⠶⠂⠘⠒⠆⠂⠐⠷⠍⠕⠙⠀⠼⠲⠦⠂⠾");
}

#[test]
fn test_163() {
    let expr = "<math><mi>n</mi><mo>=</mo><mn>120979</mn><mo>,</mo><mi>D</mi><mo>=</mo><mn>27331</mn><mo>,</mo><mi>y</mi><mo>=</mo><mn>112135</mn></math>";
    test_braille("Nemeth", expr, "⠝⠀⠨⠅⠀⠼⠂⠆⠴⠔⠶⠔⠠⠀⠠⠙⠀⠨⠅⠀⠼⠆⠶⠒⠒⠂⠠⠀⠽⠀⠨⠅⠀⠼⠂⠂⠆⠂⠒⠢");
}

#[test]
fn test_164() {
    let expr = "<math><mn>2791</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠶⠔⠂");
}

#[test]
fn test_165() {
    let expr = "<math><mn>28</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠦");
}

#[test]
fn test_166() {
    let expr = "<math><mi>q</mi><mo>=</mo><mn>29</mn></math>";
    test_braille("Nemeth", expr, "⠟⠀⠨⠅⠀⠼⠆⠔");
}

#[test]
fn test_167() {
    let expr = "<math>
        <msup><mn>292</mn>
        <mrow><mn>3171</mn></mrow></msup><mspace width=\"0.444em\"></mspace><mo stretchy=\"false\">(</mo><mi>mod</mi><mspace width=\"0.333em\"></mspace><mn>582</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠼⠆⠔⠆⠘⠒⠂⠶⠂⠐⠷⠍⠕⠙⠀⠼⠢⠦⠆⠾");
}

#[test]
fn test_168() {
    let expr = "<math><mn>2134</mn><mo>⋅<!-- ⋅ --></mo><mn>1531</mn><mo>=</mo><mn>3,267,154</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠂⠒⠲⠡⠂⠢⠒⠂⠀⠨⠅⠀⠼⠒⠠⠆⠖⠶⠠⠂⠢⠲");
}

#[test]
fn test_169() {
    let expr = "<math><mi>U</mi><mo stretchy=\"false\">(</mo><mn>30</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠥⠷⠒⠴⠾");
}

#[test]
fn test_170() {
    let expr = "<math><mi>n</mi><mo>=</mo><mn>3551</mn><mo>,</mo><mi>E</mi><mo>=</mo><mn>629</mn><mo>,</mo><mi>x</mi><mo>=</mo><mn>31</mn></math>";
    test_braille("Nemeth", expr, "⠝⠀⠨⠅⠀⠼⠒⠢⠢⠂⠠⠀⠠⠑⠀⠨⠅⠀⠼⠖⠆⠔⠠⠀⠭⠀⠨⠅⠀⠼⠒⠂");
}

#[test]
fn test_171() {
    let expr = "<math><mn>312</mn><mo>∈<!-- ∈ --></mo><msub>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">Z</mi></mrow></mrow>
        <mrow><mn>471</mn></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠼⠒⠂⠆⠀⠈⠑⠀⠨⠰⠠⠵⠲⠶⠂");
}

#[test]
fn test_172() {
    let expr = "<math><mn>342</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠲⠆");
}

#[test]
fn test_173() {
    let expr = "<math><mi>G</mi><mo>=</mo><mo fence=\"false\" stretchy=\"false\">{</mo><mo stretchy=\"false\">(</mo><mn>1</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>12</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>345</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>354</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>12</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>345</mn><mo stretchy=\"false\">)</mo><mo>,</mo><mo stretchy=\"false\">(</mo><mn>12</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>354</mn><mo stretchy=\"false\">)</mo><mo fence=\"false\" stretchy=\"false\">}</mo></math>";
    test_braille("Nemeth", expr, "⠠⠛⠀⠨⠅⠀⠨⠷⠷⠂⠾⠠⠀⠷⠂⠆⠾⠠⠀⠷⠒⠲⠢⠾⠠⠀⠷⠒⠢⠲⠾⠠⠀⠷⠂⠆⠾⠷⠒⠲⠢⠾⠠⠀⠷⠂⠆⠾⠷⠒⠢⠲⠾⠨⠾");
}

#[test]
fn test_174() {
    let expr = "<math><mn>44</mn><mspace width=\"thinmathspace\"></mspace><mn>352</mn><mspace width=\"thinmathspace\"></mspace><mn>000</mn></math>";
    test_braille("Nemeth", expr, "⠼⠲⠲⠀⠒⠢⠆⠀⠴⠴⠴");
}

#[test]
fn test_175() {
    let expr = "<math><msup><mn>360</mn><mrow><mo>∘<!-- ∘ --></mo></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠼⠒⠖⠴⠘⠨⠡");
}

#[test]
fn test_176() {
    let expr = "<math><mn>37</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠶");
}

#[test]
fn test_177() {
    let expr = "<math><mn>3754</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠶⠢⠲");
}

#[test]
fn test_178() {
    let expr = "<math><mn>38</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠦");
}

#[test]
fn test_179() {
    let expr = "<math><mn>39</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠔");
}

#[test]
fn test_180() {
    let expr = "<math><mn>391</mn><mo>=</mo><mn>17</mn><mo>⋅<!-- ⋅ --></mo><mn>23</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠔⠂⠀⠨⠅⠀⠼⠂⠶⠡⠆⠒");
}

#[test]
fn test_181() {
    let expr = "<math><mi>x</mi><mo>=</mo><mn>4</mn></math>";
    test_braille("Nemeth", expr, "⠭⠀⠨⠅⠀⠼⠲");
}

#[test]
fn test_182() {
    let expr = "<math><mn>40</mn></math>";
    test_braille("Nemeth", expr, "⠼⠲⠴");
}

#[test]
fn test_183() {
    let expr = "<math><mn>42</mn></math>";
    test_braille("Nemeth", expr, "⠼⠲⠆");
}

#[test]
fn test_184() {
    let expr = "<math><mn>43</mn><mo>−<!-- − --></mo><mn>18</mn><mi>i</mi></math>";
    test_braille("Nemeth", expr, "⠼⠲⠒⠤⠂⠦⠊");
}

#[test]
fn test_185() {
    let expr = "<math><mi>n</mi><mo>=</mo><mn>8779</mn><mo>⋅<!-- ⋅ --></mo><mn>4327</mn></math>";
    test_braille("Nemeth", expr, "⠝⠀⠨⠅⠀⠼⠦⠶⠶⠔⠡⠲⠒⠆⠶");
}

#[test]
fn test_186() {
    let expr = "<math><mo>−<!-- − --></mo><mn>4357</mn></math>";
    test_braille("Nemeth", expr, "⠤⠼⠲⠒⠢⠶");
}

#[test]
fn test_187() {
    let expr = "<math><mn>44</mn></math>";
    test_braille("Nemeth", expr, "⠼⠲⠲");
}

#[test]
fn test_188() {
    let expr = "<math><mn>46</mn></math>";
    test_braille("Nemeth", expr, "⠼⠲⠖");
}

#[test]
fn test_189() {
    let expr = "<math><mn>46,388</mn></math>";
    test_braille("Nemeth", expr, "⠼⠲⠖⠠⠒⠦⠦");
}

#[test]
fn test_190() {
    let expr = "<math><msub><mi>D</mi><mrow><mn>470448</mn></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠠⠙⠲⠶⠴⠲⠲⠦");
}

#[test]
fn test_191() {
    let expr = "<math><mn>471</mn></math>";
    test_braille("Nemeth", expr, "⠼⠲⠶⠂");
}

#[test]
fn test_192() {
    let expr = "<math><msub>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">Z</mi></mrow></mrow>
        <mrow><mn>48</mn></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠨⠰⠠⠵⠲⠦");
}

#[test]
fn test_193() {
    let expr = "<math><mi>E</mi><mo>=</mo><mn>487</mn></math>";
    test_braille("Nemeth", expr, "⠠⠑⠀⠨⠅⠀⠼⠲⠦⠶");
}

#[test]
fn test_194() {
    let expr = "<math><mi>U</mi><mo stretchy=\"false\">(</mo><mn>49</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠥⠷⠲⠔⠾");
}

#[test]
fn test_195() {
    let expr = "<math><mn>2</mn><mo>+</mo><mn>3</mn><mo>=</mo><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠬⠒⠀⠨⠅⠀⠼⠢");
}

#[test]
fn test_196() {
    let expr = "<math><mn>500</mn></math>";
    test_braille("Nemeth", expr, "⠼⠢⠴⠴");
}

#[test]
fn test_197() {
    let expr = "<math><mn>7</mn><mo>!</mo><mo>=</mo><mn>5040</mn></math>";
    // corrected: removed extra space
    test_braille("Nemeth", expr, "⠼⠶⠯⠀⠨⠅⠀⠼⠢⠴⠲⠴");
}

#[test]
fn test_198() {
    let expr = "<math><mn>51</mn></math>";
    test_braille("Nemeth", expr, "⠼⠢⠂");
}

#[test]
fn test_199() {
    let expr = "<math>
        <msup><mn>2</mn>
        <mrow><mn>511</mn></mrow></msup><mo>−<!-- − --></mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠘⠢⠂⠂⠐⠤⠂");
}

#[test]
fn test_200() {
    let expr = "<math><mn>52</mn></math>";
    test_braille("Nemeth", expr, "⠼⠢⠆");
}

#[test]
fn test_201() {
    let expr = "<math><mn>53</mn></math>";
    test_braille("Nemeth", expr, "⠼⠢⠒");
}

#[test]
fn test_202() {
    let expr = "<math><mn>54</mn></math>";
    test_braille("Nemeth", expr, "⠼⠢⠲");
}

#[test]
fn test_203() {
    let expr = "<math><mn>540</mn><mo>=</mo>
        <msup><mn>2</mn><mn>2</mn></msup><mo>⋅<!-- ⋅ --></mo>
        <msup><mn>3</mn><mn>3</mn></msup><mo>⋅<!-- ⋅ --></mo><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠢⠲⠴⠀⠨⠅⠀⠼⠆⠘⠆⠐⠡⠒⠘⠒⠐⠡⠢");
}

#[test]
fn test_204() {
    let expr = "<math><mn>561</mn></math>";
    test_braille("Nemeth", expr, "⠼⠢⠖⠂");
}

#[test]
fn test_205() {
    let expr = "<math><mn>562</mn></math>";
    test_braille("Nemeth", expr, "⠼⠢⠖⠆");
}

#[test]
fn test_206() {
    let expr = "<math><mn>57</mn><mo>=</mo>
        <msup><mn>2</mn><mn>0</mn></msup><mo>+</mo>
        <msup><mn>2</mn><mn>3</mn></msup><mo>+</mo>
        <msup><mn>2</mn><mn>4</mn></msup><mo>+</mo>
        <msup><mn>2</mn><mn>5</mn></msup></math>";
    test_braille("Nemeth", expr, "⠼⠢⠶⠀⠨⠅⠀⠼⠆⠘⠴⠐⠬⠆⠘⠒⠐⠬⠆⠘⠲⠐⠬⠆⠘⠢");
}

#[test]
fn test_207() {
    let expr = "<math><mn>58</mn></math>";
    test_braille("Nemeth", expr, "⠼⠢⠦");
}

#[test]
fn test_208() {
    let expr = "<math><mi>n</mi><mo>=</mo><mn>5893</mn><mo>,</mo><mi>D</mi><mo>=</mo><mn>81</mn><mo>,</mo><mi>y</mi><mo>=</mo><mn>34</mn></math>";
    test_braille("Nemeth", expr, "⠝⠀⠨⠅⠀⠼⠢⠦⠔⠒⠠⠀⠠⠙⠀⠨⠅⠀⠼⠦⠂⠠⠀⠽⠀⠨⠅⠀⠼⠒⠲");
}

#[test]
fn test_209() {
    let expr = "<math><mn>59</mn></math>";
    test_braille("Nemeth", expr, "⠼⠢⠔");
}

#[test]
fn test_210() {
    let expr = "<math><mn>2</mn><mi>x</mi><mo>=</mo><mn>6</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠭⠀⠨⠅⠀⠼⠖");
}

#[test]
fn test_211() {
    let expr = "<math><mn>6.00000</mn></math>";
    test_braille("Nemeth", expr, "⠼⠖⠨⠴⠴⠴⠴⠴");
}

#[test]
fn test_212() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>4</mn><mo>,</mo><mn>8</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>3</mn><mo>,</mo><mn>6.12</mn><mo>,</mo><mn>9</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>5</mn><mo>,</mo><mn>10</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mn>7</mn><mo>,</mo><mn>14</mn><mo>,</mo><mn>13</mn><mo>,</mo><mn>11</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠠⠀⠆⠠⠀⠲⠠⠀⠦⠾⠷⠒⠠⠀⠖⠨⠂⠆⠠⠀⠔⠾⠷⠢⠠⠀⠂⠴⠾⠷⠶⠠⠀⠂⠲⠠⠀⠂⠒⠠⠀⠂⠂⠾");
}

#[test]
fn test_213() {
    let expr = "<math><mi>z</mi><mo>=</mo><mn>2</mn><mi>cis</mi><mo>⁡<!-- ⁡ --></mo>
        <msup><mn>60</mn>
        <mrow><mo>∘<!-- ∘ --></mo></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠵⠀⠨⠅⠀⠼⠆⠉⠊⠎⠀⠼⠖⠴⠘⠨⠡");
}

#[test]
fn test_214() {
    let expr = "<math>
        <msup><mi>x</mi><mn>4</mn></msup><mo>−<!-- − --></mo><mo stretchy=\"false\">(</mo><mn>2</mn>
        <mrow><mo>/</mo></mrow><mn>3</mn><mo stretchy=\"false\">)</mo>
        <msup><mi>x</mi><mn>2</mn></msup><mo>−<!-- − --></mo><mn>62</mn>
        <mrow><mo>/</mo></mrow><mn>9</mn></math>";
    test_braille("Nemeth", expr, "⠭⠘⠲⠐⠤⠷⠆⠸⠌⠒⠾⠭⠘⠆⠐⠤⠖⠆⠸⠌⠔");
}

#[test]
fn test_215() {
    let expr = "<math><mo stretchy=\"false\">[</mo><mi>GF</mi><mo>⁡<!-- ⁡ --></mo><mo stretchy=\"false\">(</mo><mn>625</mn><mo stretchy=\"false\">)</mo><mo>:</mo><mi>GF</mi><mo>⁡<!-- ⁡ --></mo><mo stretchy=\"false\">(</mo><mn>25</mn><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">]</mo></math>";
    // corrected: field extension, not ratio so "⠸⠒" not "⠐⠂"
    test_braille("Nemeth", expr, "⠈⠷⠠⠠⠛⠋⠷⠖⠆⠢⠾⠸⠒⠠⠠⠛⠋⠷⠆⠢⠾⠈⠾");
}

#[test]
fn test_216() {
    let expr = "<math><mn>631</mn></math>";
    test_braille("Nemeth", expr, "⠼⠖⠒⠂");
}

#[test]
fn test_217() {
    let expr = "<math><mn>64</mn></math>";
    test_braille("Nemeth", expr, "⠼⠖⠲");
}

#[test]
fn test_218() {
    let expr = "<math>
        <msup><mn>2</mn>
        <mrow>
        <msup><mn>2</mn><mn>4</mn></msup></mrow></msup><mo>=</mo><mn>65,536</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠘⠆⠘⠘⠲⠀⠨⠅⠀⠼⠖⠢⠠⠢⠒⠖");
}

#[test]
fn test_219() {
    let expr = "<math><mn>66</mn></math>";
    test_braille("Nemeth", expr, "⠼⠖⠖");
}

#[test]
fn test_220() {
    let expr = "<math><mn>720</mn></math>";
    test_braille("Nemeth", expr, "⠼⠶⠆⠴");
}

#[test]
fn test_221() {
    let expr = "<math><mn>729</mn></math>";
    test_braille("Nemeth", expr, "⠼⠶⠆⠔");
}

#[test]
fn test_222() {
    let expr = "<math>
        <msup><mn>971</mn>
        <mrow><mn>321</mn></mrow></msup><mspace width=\"0.444em\"></mspace><mo stretchy=\"false\">(</mo><mi>mod</mi><mspace width=\"0.333em\"></mspace><mn>765</mn><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠼⠔⠶⠂⠘⠒⠆⠂⠐⠷⠍⠕⠙⠀⠼⠶⠖⠢⠾");
}

#[test]
fn test_223() {
    let expr = "<math><mn>771</mn></math>";
    test_braille("Nemeth", expr, "⠼⠶⠶⠂");
}

#[test]
fn test_224() {
    let expr = "<math><msub>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">Z</mi></mrow></mrow>
        <mrow><mn>10</mn></mrow></msub><mo>×<!-- × --></mo><msub>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">Z</mi></mrow></mrow>
        <mrow><mn>24</mn></mrow></msub><mo>×<!-- × --></mo><msub>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">Z</mi></mrow></mrow>
        <mrow><mn>80</mn></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠨⠰⠠⠵⠂⠴⠈⠡⠨⠰⠠⠵⠆⠲⠈⠡⠨⠰⠠⠵⠦⠴");
}

#[test]
fn test_225() {
    let expr = "<math>
        <msup><mn>7</mn><mn>5</mn></msup><mo>=</mo><mn>16</mn><mspace width=\"thinmathspace\"></mspace><mn>807</mn></math>";
    test_braille("Nemeth", expr, "⠼⠶⠘⠢⠀⠨⠅⠀⠼⠂⠖⠀⠦⠴⠶");
}

#[test]
fn test_226() {
    let expr = "<math><mn>811</mn></math>";
    test_braille("Nemeth", expr, "⠼⠦⠂⠂");
}

#[test]
fn test_227() {
    let expr = "<math><mn>95</mn><mo>⋅<!-- ⋅ --></mo><mn>97</mn><mo>⋅<!-- ⋅ --></mo><mn>98</mn><mo>⋅<!-- ⋅ --></mo><mn>99</mn><mo>=</mo><mn>89,403,930</mn></math>";
    test_braille("Nemeth", expr, "⠼⠔⠢⠡⠔⠶⠡⠔⠦⠡⠔⠔⠀⠨⠅⠀⠼⠦⠔⠠⠲⠴⠒⠠⠔⠒⠴");
}

#[test]
fn test_228() {
    let expr = "<math><msup><mn>90</mn><mrow><mo>∘<!-- ∘ --></mo></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠼⠔⠴⠘⠨⠡");
}

#[test]
fn test_229() {
    let expr = "<math><mn>945</mn></math>";
    test_braille("Nemeth", expr, "⠼⠔⠲⠢");
}

#[test]
fn test_230() {
    let expr = "<math><mn>95</mn></math>";
    test_braille("Nemeth", expr, "⠼⠔⠢");
}

#[test]
fn test_231() {
    let expr = "<math><mn>96</mn></math>";
    test_braille("Nemeth", expr, "⠼⠔⠖");
}

#[test]
fn test_232() {
    let expr = "<math><mn>97</mn></math>";
    test_braille("Nemeth", expr, "⠼⠔⠶");
}

#[test]
fn test_233() {
    let expr = "<math><mn>98</mn></math>";
    test_braille("Nemeth", expr, "⠼⠔⠦");
}

#[test]
fn test_234() {
    let expr = "<math><mn>99</mn></math>";
    test_braille("Nemeth", expr, "⠼⠔⠔");
}

#[test]
fn test_235() {
    let expr = "<math><mn>9923</mn></math>";
    test_braille("Nemeth", expr, "⠼⠔⠔⠆⠒");
}

#[test]
fn test_236() {
    let expr = "<math><mi>a</mi><mo>∈<!-- ∈ --></mo><mi>A</mi></math>";
    test_braille("Nemeth", expr, "⠁⠀⠈⠑⠀⠠⠁");
}

#[test]
fn test_237() {
    let expr = "<math><mi>Aut</mi><mo>⁡<!-- ⁡ --></mo><mo stretchy=\"false\">(</mo><mi>G</mi><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠁⠥⠞⠷⠠⠛⠾");
}

#[test]
fn test_238() {
    let expr = "<math><mi>A</mi><mo>⊂<!-- ⊂ --></mo><mi>B</mi></math>";
    test_braille("Nemeth", expr, "⠠⠁⠀⠸⠐⠅⠀⠠⠃");
}

#[test]
fn test_239() {
    let expr = "<math><mi>h</mi><mo>:</mo><mi>C</mi><mo stretchy=\"false\">→<!-- → --></mo><mi>D</mi></math>";
    // corrected to add English Letter indicator
    test_braille("Nemeth", expr, "⠰⠓⠸⠒⠀⠠⠉⠀⠫⠕⠀⠠⠙");
}

#[test]
fn test_240() {
    let expr = "<math><mi>G</mi><mo>×<!-- × --></mo><mi>G</mi><mo stretchy=\"false\">→<!-- → --></mo><mi>G</mi></math>";
    test_braille("Nemeth", expr, "⠠⠛⠈⠡⠠⠛⠀⠫⠕⠀⠠⠛");
}

#[test]
fn test_241() {
    let expr = "<math><mi>GF</mi><mo>⁡<!-- ⁡ --></mo><mo stretchy=\"false\">(</mo>
        <msup><mi>p</mi><mi>n</mi></msup><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠠⠛⠋⠷⠏⠘⠝⠐⠾");
}

#[test]
fn test_242() {
    let expr = "<math><mi>H</mi><mo>=</mo><mo fence=\"false\" stretchy=\"false\">{</mo><mi>e</mi><mo fence=\"false\" stretchy=\"false\">}</mo></math>";
    test_braille("Nemeth", expr, "⠠⠓⠀⠨⠅⠀⠨⠷⠑⠨⠾");
}

#[test]
fn test_243() {
    let expr = "<math><mrow><mrow><mi mathvariant=\"double-struck\">H</mi></mrow></mrow></math>";
    test_braille("Nemeth", expr, "⠨⠰⠠⠓");
}

#[test]
fn test_244() {
    let expr = "<math><mi>Hom</mi><mo>⁡<!-- ⁡ --></mo><mo stretchy=\"false\">(</mo><mi>V</mi><mo>,</mo><mi>W</mi><mo stretchy=\"false\">)</mo></math>";
    // corrected to add a space after "Hom" -- BANA (new book) says the list of function names is open ended
    test_braille("Nemeth", expr, "⠠⠓⠕⠍⠀⠷⠠⠧⠠⠀⠠⠺⠾");
}

#[test]
fn test_245() {
    let expr = "<math><mi>Inn</mi><mo>⁡<!-- ⁡ --></mo><mo stretchy=\"false\">(</mo><mi>G</mi><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠊⠝⠝⠷⠠⠛⠾");
}

#[test]
fn test_246() {
    let expr = "<math><mi>G</mi><msub><mi>L</mi><mn>2</mn></msub><mo stretchy=\"false\">(</mo>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">R</mi></mrow></mrow><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠛⠠⠇⠆⠷⠨⠰⠠⠗⠾");
}

#[test]
fn test_247() {
    let expr = "<math><mi>ϕ<!-- ϕ --></mi><mo>:</mo><msub>
        <mrow>
        <mrow><mi mathvariant=\"script\">L</mi></mrow></mrow><mi>H</mi></msub><mo stretchy=\"false\">→<!-- → --></mo><msub>
        <mrow>
        <mrow><mi mathvariant=\"script\">R</mi></mrow></mrow><mi>H</mi></msub></math>";
    test_braille("Nemeth", expr, "⠨⠋⠸⠒⠀⠈⠰⠠⠇⠰⠠⠓⠀⠫⠕⠀⠈⠰⠠⠗⠰⠠⠓");
}

#[test]
fn test_248() {
    let expr = "<math><msub>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">M</mi></mrow></mrow><mn>2</mn></msub><mo stretchy=\"false\">(</mo>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">R</mi></mrow></mrow><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠨⠰⠠⠍⠆⠷⠨⠰⠠⠗⠾");
}

#[test]
fn test_249() {
    let expr = "<math><mi>R</mi><mrow><mo>/</mo></mrow><mi>M</mi></math>";
    test_braille("Nemeth", expr, "⠠⠗⠸⠌⠠⠍");
}

#[test]
fn test_250() {
    let expr = "<math><mn>1</mn><mo>&lt;</mo><mi>n</mi><mo>&lt;</mo><mi>N</mi></math>";
    test_braille("Nemeth", expr, "⠼⠂⠀⠐⠅⠀⠝⠀⠐⠅⠀⠠⠝");
}

#[test]
fn test_251() {
    let expr = "<math><mi>Null</mi><mo>⁡<!-- ⁡ --></mo><mo stretchy=\"false\">(</mo><mi>H</mi><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠝⠥⠇⠇⠷⠠⠓⠾");
}

#[test]
fn test_252() {
    let expr = "<math><mi>O</mi><mo stretchy=\"false\">(</mo><mi>n</mi><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠕⠷⠝⠾");
}

#[test]
fn test_253() {
    let expr = "<math><mi>P</mi><mi>A</mi>
        <msup><mi>P</mi>
        <mrow><mo>−<!-- − --></mo><mn>1</mn></mrow></msup><mo>=</mo><mi>B</mi></math>";
    test_braille("Nemeth", expr, "⠠⠏⠠⠁⠠⠏⠘⠤⠂⠀⠨⠅⠀⠠⠃");
}

#[test]
fn test_254() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">P</mi></mrow></mrow><mo stretchy=\"false\">(</mo>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">R</mi></mrow></mrow><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠨⠰⠠⠏⠷⠨⠰⠠⠗⠾");
}

#[test]
fn test_255() {
    let expr = "<math><mi>Q</mi><mi>B</mi>
        <msup><mi>Q</mi>
        <mrow><mo>−<!-- − --></mo><mn>1</mn></mrow></msup><mo>=</mo><mi>C</mi></math>";
    test_braille("Nemeth", expr, "⠠⠟⠠⠃⠠⠟⠘⠤⠂⠀⠨⠅⠀⠠⠉");
}

#[test]
fn test_256() {
    let expr = "<math><mi>R</mi><mo>⊂<!-- ⊂ --></mo><mi>X</mi><mo>×<!-- × --></mo><mi>X</mi></math>";
    test_braille("Nemeth", expr, "⠠⠗⠀⠸⠐⠅⠀⠠⠭⠈⠡⠠⠭");
}

#[test]
fn test_257() {
    let expr = "<math><mi>S</mi><mo>=</mo><mo fence=\"false\" stretchy=\"false\">{</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo fence=\"false\" stretchy=\"false\">}</mo></math>";
    test_braille("Nemeth", expr, "⠠⠎⠀⠨⠅⠀⠨⠷⠂⠠⠀⠆⠠⠀⠒⠨⠾");
}

#[test]
fn test_258() {
    let expr = "<math><mtext>S</mtext><mo>=</mo><mn>18</mn></math>";
    test_braille("Nemeth", expr, "⠠⠎⠀⠨⠅⠀⠼⠂⠦");
}

#[test]
fn test_259() {
    let expr = "<math><msub><mi>T</mi><mi>A</mi></msub><mo>:</mo>
        <msup>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">R</mi></mrow></mrow><mn>2</mn></msup><mo stretchy=\"false\">→<!-- → --></mo>
        <msup>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">R</mi></mrow></mrow><mn>2</mn></msup></math>";
    test_braille("Nemeth", expr, "⠠⠞⠰⠠⠁⠸⠒⠀⠨⠰⠠⠗⠘⠆⠀⠫⠕⠀⠨⠰⠠⠗⠘⠆");
}

#[test]
fn test_260() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">T</mi></mrow></mrow><mo>=</mo><mo fence=\"false\" stretchy=\"false\">{</mo><mi>z</mi><mo>∈<!-- ∈ --></mo>
        <msup>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">C</mi></mrow></mrow><mo>∗<!-- ∗ --></mo></msup><mo>:</mo>
        <mrow><mo stretchy=\"false\">|</mo></mrow><mi>z</mi>
        <mrow><mo stretchy=\"false\">|</mo></mrow><mo>=</mo><mn>1</mn><mo fence=\"false\" stretchy=\"false\">}</mo></math>";
    // corrected: added space after ":"
    test_braille("Nemeth", expr, "⠨⠰⠠⠞⠀⠨⠅⠀⠨⠷⠵⠀⠈⠑⠀⠨⠰⠠⠉⠘⠈⠼⠸⠒⠀⠳⠵⠳⠀⠨⠅⠀⠼⠂⠨⠾");
}

#[test]
fn test_261() {
    let expr = "<math><mi>A</mi><mo>⊂<!-- ⊂ --></mo><mi>U</mi></math>";
    test_braille("Nemeth", expr, "⠠⠁⠀⠸⠐⠅⠀⠠⠥");
}

#[test]
fn test_262() {
    let expr = "<math><mi>v</mi><mo>∈<!-- ∈ --></mo><mi>V</mi></math>";
    test_braille("Nemeth", expr, "⠧⠀⠈⠑⠀⠠⠧");
}

#[test]
fn test_263() {
    let expr = "<math><mi>Y</mi><mo>=</mo><mo fence=\"false\" stretchy=\"false\">{</mo><mi>B</mi><mo>,</mo><mi>W</mi><mo fence=\"false\" stretchy=\"false\">}</mo></math>";
    test_braille("Nemeth", expr, "⠠⠽⠀⠨⠅⠀⠨⠷⠠⠃⠠⠀⠠⠺⠨⠾");
}

#[test]
fn test_264() {
    let expr = "<math><mi>f</mi><mo>:</mo><mi>X</mi><mo stretchy=\"false\">→<!-- → --></mo><mi>Y</mi></math>";
    // corrected to add English Letter indicator
    test_braille("Nemeth", expr, "⠰⠋⠸⠒⠀⠠⠭⠀⠫⠕⠀⠠⠽");
}

#[test]
fn test_265() {
    let expr = "<math><msub>
        <mrow><mi mathvariant=\"double-struck\">M</mi></mrow>
        <mrow><mi>m</mi><mo>×<!-- × --></mo><mi>n</mi></mrow></msub><mo stretchy=\"false\">(</mo><msub>
        <mrow><mi mathvariant=\"bold\">Z</mi></mrow><mn>2</mn></msub><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠨⠰⠠⠍⠰⠍⠈⠡⠝⠐⠷⠸⠰⠠⠵⠆⠾");
}

#[test]
fn test_266() {
    let expr = "<math><mo stretchy=\"false\">[</mo><mi>x</mi><mo stretchy=\"false\">]</mo><mo>=</mo><mo fence=\"false\" stretchy=\"false\">{</mo><mi>y</mi><mo>∈<!-- ∈ --></mo><mi>X</mi><mo>:</mo><mi>y</mi><mo>∼<!-- ∼ --></mo><mi>x</mi><mo fence=\"false\" stretchy=\"false\">}</mo></math>";
    // corrected: added space after ":"
    test_braille("Nemeth", expr, "⠈⠷⠭⠈⠾⠀⠨⠅⠀⠨⠷⠽⠀⠈⠑⠀⠠⠭⠸⠒⠀⠽⠀⠈⠱⠀⠭⠨⠾");
}

#[test]
fn test_267() {
    let expr = "<math><mi>F</mi><mo fence=\"false\" stretchy=\"false\">[</mo><mi>x</mi><mo fence=\"false\" stretchy=\"false\">]</mo></math>";
    test_braille("Nemeth", expr, "⠠⠋⠈⠷⠭⠈⠾");
}

#[test]
fn test_268() {
    let expr = "<math><mi>char</mi><mo>⁡<!-- ⁡ --></mo><mi>R</mi></math>";
    test_braille("Nemeth", expr, "⠉⠓⠁⠗⠀⠠⠗");
}

#[test]
fn test_269() {
    let expr = "<math><mi>r</mi><mi>cis</mi><mo>⁡<!-- ⁡ --></mo><mi>θ<!-- θ --></mi></math>";
    test_braille("Nemeth", expr, "⠗⠉⠊⠎⠀⠨⠹");
}

#[test]
fn test_270() {
    let expr = "<math><mi>α<!-- α --></mi><mi>β<!-- β --></mi><mo>=</mo>
        <mrow><mi mathvariant=\"normal\">i</mi><mi mathvariant=\"normal\">d</mi></mrow></math>";
    test_braille("Nemeth", expr, "⠨⠁⠨⠃⠀⠨⠅⠀⠊⠙");
}

#[test]
fn test_271() {
    let expr = "<math><mi>deg</mi><mo>⁡<!-- ⁡ --></mo><mi>f</mi><mo stretchy=\"false\">(</mo><mi>x</mi><mo stretchy=\"false\">)</mo><mo>=</mo><mi>n</mi></math>";
    test_braille("Nemeth", expr, "⠙⠑⠛⠀⠋⠷⠭⠾⠀⠨⠅⠀⠝");
}

#[test]
fn test_272() {
    let expr = "<math><mo form=\"prefix\" movablelimits=\"true\">det</mo><mi>A</mi><mo>=</mo><mi>a</mi><mi>d</mi><mo>−<!-- − --></mo><mi>b</mi><mi>c</mi><mo>≠<!-- ≠ --></mo><mn>0</mn></math>";
    test_braille("Nemeth", expr, "⠙⠑⠞⠀⠠⠁⠀⠨⠅⠀⠁⠙⠤⠃⠉⠀⠌⠨⠅⠀⠼⠴");
}

#[test]
fn test_273() {
    let expr = "<math><mi>dim</mi><mo>⁡<!-- ⁡ --></mo><mi>V</mi><mo>=</mo><mi>n</mi></math>";
    test_braille("Nemeth", expr, "⠙⠊⠍⠀⠠⠧⠀⠨⠅⠀⠝");
}

#[test]
fn test_274() {
    let expr = "<math>
        <msup><mi>f</mi>
        <mrow><mo>−<!-- − --></mo><mn>1</mn></mrow></msup><mo stretchy=\"false\">(</mo><mi>x</mi><mo stretchy=\"false\">)</mo><mo>=</mo>
        <msup><mi>e</mi><mi>x</mi></msup></math>";
    test_braille("Nemeth", expr, "⠋⠘⠤⠂⠐⠷⠭⠾⠀⠨⠅⠀⠑⠘⠭");
}

#[test]
fn test_275() {
    let expr = "<math><mi>f</mi><mo>⊂<!-- ⊂ --></mo><mi>A</mi><mo>×<!-- × --></mo><mi>B</mi></math>";
    test_braille("Nemeth", expr, "⠋⠀⠸⠐⠅⠀⠠⠁⠈⠡⠠⠃");
}

#[test]
fn test_276() {
    let expr = "<math><mi>g</mi><mo stretchy=\"false\">(</mo><mn>1</mn><mo stretchy=\"false\">)</mo><mo>=</mo><mi>a</mi></math>";
    test_braille("Nemeth", expr, "⠛⠷⠂⠾⠀⠨⠅⠀⠁");
}

#[test]
fn test_277() {
    let expr = "<math><mi>d</mi><mo>=</mo><mo form=\"prefix\" movablelimits=\"true\">gcd</mo><mo stretchy=\"false\">(</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo stretchy=\"false\">)</mo></math>";
    // corrected: missing space before '='
    test_braille("Nemeth", expr, "⠙⠀⠨⠅⠀⠛⠉⠙⠀⠷⠁⠠⠀⠃⠾");
}

#[test]
fn test_278() {
    let expr = "<math><msub><mi>X</mi><mi>i</mi></msub><mo>∩<!-- ∩ --></mo><msub><mi>X</mi><mi>j</mi></msub><mo>=</mo><mi mathvariant=\"normal\">∅<!-- ∅ --></mi></math>";
    test_braille("Nemeth", expr, "⠠⠭⠰⠊⠐⠨⠩⠠⠭⠰⠚⠀⠨⠅⠀⠸⠴");
}

#[test]
fn test_279() {
    let expr = "<math><mi>ker</mi><mo>⁡<!-- ⁡ --></mo><mi>ϕ<!-- ϕ --></mi></math>";
    test_braille("Nemeth", expr, "⠅⠑⠗⠀⠨⠋");
}

#[test]
fn test_280() {
    let expr = "<math><mi>s</mi><mo>−<!-- − --></mo><mi>t</mi><mo>=</mo><mi>l</mi><mi>n</mi></math>";
    test_braille("Nemeth", expr, "⠎⠤⠞⠀⠨⠅⠀⠇⠝");
}

#[test]
fn test_281() {
    let expr = "<math><mi>lcm</mi><mo>⁡<!-- ⁡ --></mo><mo stretchy=\"false\">(</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠇⠉⠍⠀⠷⠁⠠⠀⠃⠾");
}

#[test]
fn test_282() {
    let expr = "<math><munder><mo form=\"prefix\" movablelimits=\"true\">lim</mo>
        <mrow><mi>n</mi><mo stretchy=\"false\">→<!-- → --></mo><mi mathvariant=\"normal\">∞<!-- ∞ --></mi></mrow></munder><msub><mi>f</mi><mi>n</mi></msub>
        <mrow><mo>/</mo></mrow><msub><mi>f</mi>
        <mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub><mo>=</mo><mo stretchy=\"false\">(</mo>
        <msqrt><mn>5</mn></msqrt><mo>−<!-- − --></mo><mn>1</mn><mo stretchy=\"false\">)</mo>
        <mrow><mo>/</mo></mrow><mn>2</mn></math>";
    test_braille("Nemeth", expr, "⠐⠇⠊⠍⠩⠝⠀⠫⠕⠀⠠⠿⠻⠀⠋⠰⠝⠐⠸⠌⠋⠰⠝⠬⠂⠀⠨⠅⠀⠷⠜⠢⠻⠤⠂⠾⠸⠌⠆");
}

#[test]
fn test_283() {
    let expr = "<math><mi>f</mi><mo stretchy=\"false\">(</mo><mi>x</mi><mo stretchy=\"false\">)</mo><mo>=</mo><mi>ln</mi><mo>⁡<!-- ⁡ --></mo><mi>x</mi></math>";
    test_braille("Nemeth", expr, "⠋⠷⠭⠾⠀⠨⠅⠀⠇⠝⠀⠭");
}

#[test]
fn test_284() {
    let expr = "<math>
        <msup>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">R</mi></mrow></mrow><mi>m</mi></msup></math>";
    test_braille("Nemeth", expr, "⠨⠰⠠⠗⠘⠍");
}

#[test]
fn test_285() {
    let expr = "<math><mi>deg</mi><mo>⁡<!-- ⁡ --></mo><mo stretchy=\"false\">(</mo><mi>p</mi><mo stretchy=\"false\">(</mo><mi>x</mi><mo stretchy=\"false\">)</mo><mo>+</mo><mi>q</mi><mo stretchy=\"false\">(</mo><mi>x</mi><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">)</mo><mo>≤<!-- ≤ --></mo><mo form=\"prefix\" movablelimits=\"true\">max</mo><mo stretchy=\"false\">(</mo><mi>deg</mi><mo>⁡<!-- ⁡ --></mo><mi>p</mi><mo stretchy=\"false\">(</mo><mi>x</mi><mo stretchy=\"false\">)</mo><mo>,</mo><mi>deg</mi><mo>⁡<!-- ⁡ --></mo><mi>q</mi><mo stretchy=\"false\">(</mo><mi>x</mi><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠙⠑⠛⠀⠷⠏⠷⠭⠾⠬⠟⠷⠭⠾⠾⠀⠐⠅⠱⠀⠍⠁⠭⠀⠷⠙⠑⠛⠀⠏⠷⠭⠾⠠⠀⠙⠑⠛⠀⠟⠷⠭⠾⠾");
}

#[test]
fn test_286() {
    let expr = "<math><msub><mi>d</mi>
        <mrow><mo form=\"prefix\" movablelimits=\"true\">min</mo></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠙⠰⠍⠊⠝");
}

#[test]
fn test_287() {
    let expr = "<math><mi>r</mi><mo>=</mo><mi>s</mi></math>";
    test_braille("Nemeth", expr, "⠗⠀⠨⠅⠀⠎");
}

#[test]
fn test_288() {
    let expr = "<math><mrow><mrow><mi mathvariant=\"bold\">r</mi></mrow></mrow></math>";
    test_braille("Nemeth", expr, "⠸⠰⠗");
}

#[test]
fn test_289() {
    let expr = "<math><mi>f</mi><mo stretchy=\"false\">(</mo><mi>x</mi><mo stretchy=\"false\">)</mo><mo>=</mo><mi>sin</mi><mo>⁡<!-- ⁡ --></mo><mi>x</mi></math>";
    test_braille("Nemeth", expr, "⠋⠷⠭⠾⠀⠨⠅⠀⠎⠊⠝⠀⠭");
}

#[test]
fn test_290() {
    let expr = "<math><mi>r</mi>
        <mrow><mo>/</mo></mrow><mi>s</mi><mo>∼<!-- ∼ --></mo><mi>t</mi>
        <mrow><mo>/</mo></mrow><mi>u</mi></math>";
    test_braille("Nemeth", expr, "⠗⠸⠌⠎⠀⠈⠱⠀⠞⠸⠌⠥");
}

#[test]
fn test_291() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">b</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><mn>2</mn><mo>,</mo><mn>2</mn>
        <msup><mo stretchy=\"false\">)</mo><mtext>t</mtext></msup></math>";
        // should be corrected
        // <mi mathvariant=\"bold\">b</mi></mrow><mo>=</mo><msup><mrow><mo stretchy=\"false\">(</mo><mn>2</mn><mo>,</mo><mn>2</mn>
        // <mo stretchy=\"false\">)</mo></mrow><mtext>t</mtext></msup></math>";
test_braille("Nemeth", expr, "⠸⠰⠃⠀⠨⠅⠀⠷⠆⠠⠀⠆⠾⠘⠞");
}

#[test]
fn test_292() {
    let expr = "<math><mi>v</mi><mo>∈<!-- ∈ --></mo>
        <msup>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">R</mi></mrow></mrow><mn>2</mn></msup></math>";
    test_braille("Nemeth", expr, "⠧⠀⠈⠑⠀⠨⠰⠠⠗⠘⠆");
}

#[test]
fn test_293() {
    let expr = "<math><mrow><mrow><mi mathvariant=\"bold\">w</mi></mrow></mrow></math>";
    test_braille("Nemeth", expr, "⠸⠰⠺");
}

#[test]
fn test_294() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">x</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><msub><mi>x</mi><mn>1</mn></msub><mo>,</mo><mo>…<!-- … --></mo><mo>,</mo><msub><mi>x</mi><mi>n</mi></msub><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠸⠰⠭⠀⠨⠅⠀⠷⠭⠂⠠⠀⠄⠄⠄⠠⠀⠭⠰⠝⠐⠾");
}

#[test]
fn test_295() {
    let expr = "<math><mi>A</mi><mo>=</mo><mo fence=\"false\" stretchy=\"false\">{</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo fence=\"false\" stretchy=\"false\">}</mo></math>";
    test_braille("Nemeth", expr, "⠠⠁⠀⠨⠅⠀⠨⠷⠭⠠⠀⠽⠨⠾");
}

#[test]
fn test_296() {
    let expr = "<math>
        <mrow>
        <mrow><mi mathvariant=\"bold\">y</mi></mrow></mrow><mo>=</mo><mo stretchy=\"false\">(</mo><msub><mi>y</mi><mn>1</mn></msub><mo>,</mo><mo>…<!-- … --></mo><mo>,</mo><msub><mi>y</mi><mi>n</mi></msub><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠸⠰⠽⠀⠨⠅⠀⠷⠽⠂⠠⠀⠄⠄⠄⠠⠀⠽⠰⠝⠐⠾");
}

#[test]
fn test_297() {
    let expr = "<math>
        <mrow><mo stretchy=\"false\">|</mo></mrow><mi>x</mi><mo>−<!-- − --></mo><mi>y</mi>
        <mrow><mo stretchy=\"false\">|</mo></mrow><mo>≤<!-- ≤ --></mo><mn>4</mn></math>";
    test_braille("Nemeth", expr, "⠳⠭⠤⠽⠳⠀⠐⠅⠱⠀⠼⠲");
}

#[test]
fn test_298() {
    let expr = "<math>
        <mrow><mover><mi>σ<!-- σ --></mi><mo>~<!-- ~ --></mo></mover></mrow></math>";
    test_braille("Nemeth", expr, "⠐⠨⠎⠣⠈⠱⠻");
}

#[test]
fn test_299() {
    let expr = "<math><mi>A</mi><mo>×<!-- × --></mo><mi>B</mi></math>";
    test_braille("Nemeth", expr, "⠠⠁⠈⠡⠠⠃");
}

#[test]
fn test_300() {
    let expr = "<math><mi mathvariant=\"normal\">Δ<!-- Δ --></mi><mo>=</mo>
        <msup><mi>b</mi><mn>2</mn></msup><mo>−<!-- − --></mo><mn>4</mn><mi>a</mi><mi>c</mi></math>";
    test_braille("Nemeth", expr, "⠨⠠⠙⠀⠨⠅⠀⠃⠘⠆⠐⠤⠲⠁⠉");
}

#[test]
fn test_301() {
    let expr = "<math><mi>α<!-- α --></mi></math>";
    test_braille("Nemeth", expr, "⠨⠁");
}

#[test]
fn test_302() {
    let expr = "<math><mi>β<!-- β --></mi></math>";
    test_braille("Nemeth", expr, "⠨⠃");
}

#[test]
fn test_303() {
    let expr = "<math><mo fence=\"false\" stretchy=\"false\">⟨<!-- ⟨ --></mo><msub>
        <mrow>
        <mrow><mi mathvariant=\"bold\">a</mi></mrow></mrow><mi>r</mi></msub><mo>,</mo><msub>
        <mrow>
        <mrow><mi mathvariant=\"bold\">a</mi></mrow></mrow><mi>s</mi></msub><mo fence=\"false\" stretchy=\"false\">⟩<!-- ⟩ --></mo><mo>=</mo><msub><mi>δ<!-- δ --></mi>
        <mrow><mi>r</mi><mi>s</mi></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠨⠨⠷⠸⠰⠁⠰⠗⠠⠀⠸⠰⠁⠰⠎⠐⠨⠨⠾⠀⠨⠅⠀⠨⠙⠰⠗⠎");
}

#[test]
fn test_304() {
    let expr = "<math><mi>η<!-- η --></mi><mo>:</mo><mi>G</mi>
        <mrow><mo>/</mo></mrow><mi>K</mi><mo stretchy=\"false\">→<!-- → --></mo><mi>ψ<!-- ψ --></mi><mo stretchy=\"false\">(</mo><mi>G</mi><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠨⠱⠸⠒⠀⠠⠛⠸⠌⠠⠅⠀⠫⠕⠀⠨⠽⠷⠠⠛⠾");
}

#[test]
fn test_305() {
    let expr = "<math><mi>λ<!-- λ --></mi></math>";
    test_braille("Nemeth", expr, "⠨⠇");
}

#[test]
fn test_306() {
    let expr = "<math><msub><mi>μ<!-- μ --></mi><mn>1</mn></msub><msub><mi>ρ<!-- ρ --></mi><mn>1</mn></msub></math>";
    test_braille("Nemeth", expr, "⠨⠍⠂⠨⠗⠂");
}

#[test]
fn test_307() {
    let expr = "<math><mi>ν<!-- ν --></mi><mo>:</mo>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">Z</mi></mrow></mrow><mo stretchy=\"false\">[</mo>
        <msqrt><mn>3</mn></msqrt><mspace width=\"thinmathspace\"></mspace><mi>i</mi><mo stretchy=\"false\">]</mo><mo stretchy=\"false\">→<!-- → --></mo>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">N</mi></mrow></mrow><mo>∪<!-- ∪ --></mo><mo fence=\"false\" stretchy=\"false\">{</mo><mn>0</mn><mo fence=\"false\" stretchy=\"false\">}</mo></math>";
    test_braille("Nemeth", expr, "⠨⠝⠸⠒⠀⠨⠰⠠⠵⠈⠷⠜⠒⠻⠊⠈⠾⠀⠫⠕⠀⠨⠰⠠⠝⠨⠬⠨⠷⠴⠨⠾");
}

#[test]
fn test_308() {
    let expr = "<math><mi>π<!-- π --></mi><mo>:</mo><mi>S</mi><mo stretchy=\"false\">→<!-- → --></mo><mi>S</mi></math>";
    test_braille("Nemeth", expr, "⠨⠏⠸⠒⠀⠠⠎⠀⠫⠕⠀⠠⠎");
}

#[test]
fn test_309() {
    let expr = "<math><mi>ψ<!-- ψ --></mi></math>";
    test_braille("Nemeth", expr, "⠨⠽");
}

#[test]
fn test_310() {
    let expr = "<math><mi>w</mi><mo>=</mo><mi>s</mi><mi>cis</mi><mo>⁡<!-- ⁡ --></mo><mi>ϕ<!-- ϕ --></mi></math>";
    test_braille("Nemeth", expr, "⠺⠀⠨⠅⠀⠎⠉⠊⠎⠀⠨⠋");
}

#[test]
fn test_311() {
    let expr = "<math><msup><mi>A</mi><mo>′</mo></msup></math>";
    test_braille("Nemeth", expr, "⠠⠁⠄");
}

#[test]
fn test_312() {
    let expr = "<math><msup><mi>g</mi><mo>″</mo></msup></math>";
    test_braille("Nemeth", expr, "⠛⠄⠄");
}

#[test]
fn test_313() {
    let expr = "<math><mi>ℓ<!-- ℓ --></mi></math>";
    test_braille("Nemeth", expr, "⠈⠰⠇");
}

#[test]
fn test_314() {
    let expr = "<math><mi>f</mi><mo>:</mo><mi>A</mi><mo stretchy=\"false\">→<!-- → --></mo><mi>B</mi></math>";
    // corrected to add English Letter indicator
    test_braille("Nemeth", expr, "⠰⠋⠸⠒⠀⠠⠁⠀⠫⠕⠀⠠⠃");
}

#[test]
fn test_315() {
    let expr = "<math><mi>f</mi><mo>:</mo><mi>a</mi><mo stretchy=\"false\">↦<!-- ↦ --></mo><mi>b</mi></math>";
    // corrected to add English Letter indicator
    test_braille("Nemeth", expr, "⠰⠋⠸⠒⠀⠁⠀⠫⠳⠒⠒⠕⠀⠃");
}

#[test]
fn test_316() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mo stretchy=\"false\">⇐<!-- ⇐ --></mo><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠫⠪⠶⠶⠾");
}

#[test]
fn test_317() {
    let expr = "<math><mi mathvariant=\"normal\">∅<!-- ∅ --></mi></math>";
    test_braille("Nemeth", expr, "⠸⠴");
}

#[test]
fn test_318() {
    let expr = "<math><mo>−<!-- − --></mo><mn>3</mn><mo>∉<!-- ∉ --></mo><mi>E</mi></math>";
    test_braille("Nemeth", expr, "⠤⠼⠒⠀⠌⠈⠑⠀⠠⠑");
}

#[test]
fn test_319() {
    let expr = "<math>
        <msup>
        <mrow>
        <mrow><mi mathvariant=\"double-struck\">C</mi></mrow></mrow><mo>∗<!-- ∗ --></mo></msup></math>";
    test_braille("Nemeth", expr, "⠨⠰⠠⠉⠘⠈⠼");
}

#[test]
fn test_320() {
    let expr = "<math><mo stretchy=\"false\">(</mo><mi>g</mi><mo>∘<!-- ∘ --></mo><mi>f</mi><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">(</mo><mi>x</mi><mo stretchy=\"false\">)</mo><mo>=</mo><mi>g</mi><mo stretchy=\"false\">(</mo><mi>f</mi><mo stretchy=\"false\">(</mo><mi>x</mi><mo stretchy=\"false\">)</mo><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠛⠨⠡⠋⠾⠷⠭⠾⠀⠨⠅⠀⠛⠷⠋⠷⠭⠾⠾");
}

#[test]
fn test_321() {
    let expr = "<math><mi>a</mi><mo>∣<!-- ∣ --></mo><mi>b</mi></math>";
    test_braille("Nemeth", expr, "⠁⠳⠃");
}

#[test]
fn test_322() {
    let expr = "<math><mi>p</mi><mo>∤<!-- ∤ --></mo><mi>a</mi></math>";
    test_braille("Nemeth", expr, "⠏⠌⠳⠁");
}

#[test]
fn test_323() {
    let expr = "<math><mi>a</mi><mo>∧<!-- ∧ --></mo><mi>b</mi></math>";
    test_braille("Nemeth", expr, "⠁⠈⠩⠃");
}

#[test]
fn test_324() {
    let expr = "<math><mi>a</mi><mo>∨<!-- ∨ --></mo><mi>b</mi></math>";
    test_braille("Nemeth", expr, "⠁⠈⠬⠃");
}

#[test]
fn test_325() {
    let expr = "<math><mi>A</mi><mo>∪<!-- ∪ --></mo><mi>B</mi></math>";
    test_braille("Nemeth", expr, "⠠⠁⠨⠬⠠⠃");
}

#[test]
fn test_326() {
    let expr = "<math><mi>x</mi><mo>∼<!-- ∼ --></mo><mi>y</mi></math>";
    test_braille("Nemeth", expr, "⠭⠀⠈⠱⠀⠽");
}

#[test]
fn test_327() {
    let expr = "<math><mo>≅<!-- ≅ --></mo></math>";
    test_braille("Nemeth", expr, "⠈⠱⠨⠅");
}

#[test]
fn test_328() {
    let expr = "<math><mi>a</mi><mo>≠<!-- ≠ --></mo><mn>0</mn></math>";
    test_braille("Nemeth", expr, "⠁⠀⠌⠨⠅⠀⠼⠴");
}

#[test]
fn test_329() {
    let expr = "<math><mo>≡<!-- ≡ --></mo></math>";
    test_braille("Nemeth", expr, "⠸⠇");
}

#[test]
fn test_330() {
    let expr = "<math><mi>q</mi><mo>≢</mo><mn>1</mn><mspace width=\"0.444em\"></mspace><mo stretchy=\"false\">(</mo><mi>mod</mi><mspace width=\"0.333em\"></mspace><mi>p</mi><mo stretchy=\"false\">)</mo></math>";
    test_braille("Nemeth", expr, "⠟⠀⠌⠸⠇⠀⠼⠂⠷⠍⠕⠙⠀⠏⠾");
}

#[test]
fn test_331() {
    let expr = "<math><mi>x</mi><mo>≥<!-- ≥ --></mo><mi>y</mi></math>";
    test_braille("Nemeth", expr, "⠭⠀⠨⠂⠱⠀⠽");
}

#[test]
fn test_332() {
    let expr = "<math><mi>B</mi><mo>⊃<!-- ⊃ --></mo><mi>A</mi></math>";
    test_braille("Nemeth", expr, "⠠⠃⠀⠸⠨⠂⠀⠠⠁");
}

#[test]
fn test_333() {
    let expr = "<math><mi>A</mi><mo>⊄</mo><mi>B</mi></math>";
    test_braille("Nemeth", expr, "⠠⠁⠀⠌⠸⠐⠅⠀⠠⠃");
}

#[test]
fn test_334() {
    let expr = "<math><mi>W</mi><mo>=</mo><mi>U</mi><mo>⊕<!-- ⊕ --></mo><mi>V</mi></math>";
    test_braille("Nemeth", expr, "⠠⠺⠀⠨⠅⠀⠠⠥⠫⠉⠸⠫⠬⠻⠠⠧");
}

#[test]
fn test_335() {
    let expr = "<math><mn>2</mn><mo>⋅<!-- ⋅ --></mo><mn>4</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠡⠲");
}

#[test]
fn test_336() {
    let expr = "<math><msub><mi>Z</mi><mn>3</mn></msub><mo>⋊<!-- ⋊ --></mo><msub><mi>Z</mi><mn>4</mn></msub></math>";
    test_braille("Nemeth", expr, "⠠⠵⠒⠈⠡⠳⠠⠵⠲");
}

#[test]
fn test_337() {
    let expr = "<math><mo>△<!-- △ --></mo><mi>A</mi><mi>B</mi><mi>C</mi></math>";
    test_braille("Nemeth", expr, "⠫⠞⠀⠠⠁⠠⠃⠠⠉");
}

#[test]
fn test_338() {
    let expr = "<math><mi>a</mi><mo>⪯<!-- ⪯ --></mo><mi>b</mi></math>";
     // corrected: precedes part
     test_braille("Nemeth", expr, "⠁⠀⠨⠐⠅⠱⠀⠃");
}

#[test]
fn test_339() {
    let expr = "<math><mo>⪰<!-- ⪰ --></mo></math>";
    // corrected: succeeds part
    test_braille("Nemeth", expr, "⠨⠨⠂⠱");
}

