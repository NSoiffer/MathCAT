use crate::common::*;

#[test]
fn test_000() {
    let expr = "<math><mo>-</mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠤⠼⠂");
}

#[test]
fn test_001() {
    let expr = "<math><mo>-</mo><mn>.3</mn></math>";
    test_braille("Nemeth", expr, "⠤⠼⠨⠒");
}

#[test]
fn test_002() {
    let expr = "<math><mi>n</mi><mo>!</mo></math>";
    test_braille("Nemeth", expr, "⠝⠯");
}

#[test]
fn test_003() {
    let expr = "<math><mn>1,378</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠠⠒⠶⠦");
}

#[test]
fn test_004() {
    let expr = "<math><mn>3.76</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠨⠶⠖");
}

#[test]
fn test_005() {
    let expr = "<math><mn>1,478</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠠⠲⠶⠦");
}

#[test]
fn test_006() {
    // modified to add space as in number_list_8_b_2
    let expr = "<math><mn>100</mn><mo>,</mo><mtext>&#xA0;</mtext><mn>200</mn><mo>,</mo><mtext>&#xA0;</mtext><mn>300</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠴⠴⠠⠀⠼⠆⠴⠴⠠⠀⠼⠒⠴⠴");
}

#[test]
fn test_007() {
    // number_8_c_5
    let expr = "<math><mn>.35</mn></math>";
    test_braille("Nemeth", expr, "⠼⠨⠒⠢");
}

#[test]
fn test_008() {
    let expr = "<math><mn>3.14</mn></math>";
    test_braille("Nemeth", expr, "⠼⠒⠨⠂⠲");
}

#[test]
fn test_009() {
    let expr = "<math><mn>.2</mn><msub><mi>a</mi><mn>1</mn></msub><msub><mi>a</mi><mn>2</mn></msub><msub><mi>a</mi><mn>3</mn></msub></math>";
    test_braille("Nemeth", expr, "⠼⠨⠆⠁⠂⠁⠆⠁⠒");
}

#[test]
fn test_010() {
    let expr = "<math><mn>.</mn><msub><mi>a</mi><mn>1</mn></msub><msub><mi>a</mi><mn>2</mn></msub><msub><mi>a</mi><mn>3</mn></msub></math>";
    test_braille("Nemeth", expr, "⠨⠐⠁⠂⠁⠆⠁⠒");
}

#[test]
fn test_011() {
    let expr = "<math><mn>.1</mn><mo>+</mo><mn>.2</mn><mo>=</mo><mo>.</mo><mo>----</mo></math>";
    test_braille("Nemeth", expr, "⠼⠨⠂⠬⠨⠆⠀⠨⠅⠀⠨⠐⠤⠤⠤⠤");
}

#[test]
fn test_012() {
    let expr = "<math><mn>27</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆⠶");
}

#[test]
fn test_013() {
    let expr = "<math><mn>1</mn><mo>+</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mn>0</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠬⠭⠬⠽⠀⠨⠅⠀⠼⠴");
}

#[test]
fn test_014() {
    let expr = "<math><mi>y</mi><mo>=</mo><mn>2</mn><mi>sin</mi><mo>⁡</mo><mi>x</mi></math>";
    test_braille("Nemeth", expr, "⠽⠀⠨⠅⠀⠼⠆⠎⠊⠝⠀⠭");
}

#[test]
fn test_015() {
    let expr = "<math><mi>sin</mi><mo>⁡</mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠎⠊⠝⠀⠼⠂");
}

#[test]
fn test_016() {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mo>⁡</mo><mn>2</mn><mi>x</mi></math>";
    test_braille("Nemeth", expr, "⠎⠊⠝⠘⠆⠀⠼⠆⠭");
}

#[test]
fn test_017() {
    let expr = "<math><mn>0.333</mn><mo>…</mo><mn>3</mn><mo>…</mo></math>";
    test_braille("Nemeth", expr, "⠼⠴⠨⠒⠒⠒⠀⠄⠄⠄⠀⠼⠒⠀⠄⠄⠄");
}

#[test]
fn test_018() {
    let expr = "<math><msub><mi>log</mi>
        <mrow><mn>10</mn></mrow></msub><mo>⁡</mo><mn>2</mn></math>";
    test_braille("Nemeth", expr, "⠇⠕⠛⠂⠴⠀⠼⠆");
}

#[test]
fn test_019() {
    let expr = "<math><mo>(</mo><mi>x</mi><mo>=</mo><mn>0</mn><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠭⠀⠨⠅⠀⠼⠴⠾");
}

#[test]
fn test_020() {
    let expr = "<math><mfrac><mn>11</mn><mn>5</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠂⠂⠌⠢⠼");
}

#[test]
fn test_021() {
    let expr = "<math><mo>-</mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠤⠼⠂");
}

#[test]
fn test_022() {
    let expr = "<math><mo>-</mo><mn>.3</mn></math>";
    test_braille("Nemeth", expr, "⠤⠼⠨⠒");
}

#[test]
fn test_023() {
    // (modified to be a single mtext) 9_b_1
    let expr = "<math><mtext>“3&#xA0;dogs”</mtext></math>";
    test_braille("Nemeth", expr, "⠦⠼⠒⠀⠙⠕⠛⠎⠴");
}

#[test]
fn test_024() {
    // 9_b_2 -- changed dash to be char that outputs the appropriate Nemeth
    let expr = "<math><mtext>Probability</mtext><mo>—</mo><mn>0</mn></math>";
    test_braille("Nemeth", expr, "⠠⠏⠗⠕⠃⠁⠃⠊⠇⠊⠞⠽⠤⠤⠼⠴");
}

#[test]
fn test_025() {
    let expr = "<math><mtext>“</mtext><mn>.5</mn></math>";
    test_braille("Nemeth", expr, "⠦⠼⠨⠢");
}

#[test]
fn test_026() {
    let expr = "<math><mtext>“</mtext><mo>-</mo><mn>4</mn></math>";
    test_braille("Nemeth", expr, "⠦⠤⠼⠲");
}

#[test]
fn test_027() {
    let expr = "<math><mfrac><mn>1</mn><mn>3</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠂⠌⠒⠼");
}

#[test]
fn test_028() {
    let expr = "<math><msup><mi>x</mi><mfrac><mn>1</mn><mn>2</mn></mfrac></msup></math>";
    test_braille("Nemeth", expr, "⠭⠘⠹⠂⠌⠆⠼");
}

#[test]
fn test_029() {
    let expr = "<math>
        <mfrac>
        <mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mi>c</mi></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠁⠬⠃⠌⠉⠼");
}

#[test]
fn test_030() {
    let expr = "<math>
        <mfrac>
        <msup><mi>x</mi>
        <mfrac><mn>1</mn><mn>2</mn></mfrac></msup><mn>2</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠭⠘⠹⠂⠌⠆⠼⠐⠌⠆⠼");
}

#[test]
fn test_031() {
    let expr = "<math><mtext>rate</mtext><mo>=</mo>
        <mfrac><mtext>distance</mtext><mtext>time</mtext></mfrac></math>";
    test_braille("Nemeth", expr, "⠗⠁⠞⠑⠀⠨⠅⠀⠹⠙⠊⠎⠞⠁⠝⠉⠑⠌⠞⠊⠍⠑⠼");
}

#[test]
fn test_032() {
    let expr = "<math><mfrac bevelled=\"true\">
        <mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow>
        <mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠁⠬⠃⠸⠌⠉⠬⠙⠼");
}

#[test]
fn test_033() {
    let expr = "<math><mfrac><mfrac><mn>3</mn><mn>8</mn></mfrac><mn>5</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠠⠹⠹⠒⠌⠦⠼⠠⠌⠢⠠⠼");
}

#[test]
fn test_034() {
    let expr = "<math>
        <mfrac>
        <mrow><mn>1</mn>
        <mrow><mo>/</mo></mrow><mn>2</mn></mrow>
        <mrow><mn>2</mn>
        <mfrac><mn>2</mn><mn>3</mn></mfrac></mrow></mfrac></math>";
    test_braille("Nemeth", expr, "⠠⠹⠂⠸⠌⠆⠠⠌⠆⠸⠹⠆⠌⠒⠸⠼⠠⠼");
}

#[test]
fn test_035() {
    let expr = "<math>
        <mfrac><mn>5</mn>
        <mrow><mn>4</mn>
        <mfrac><mn>3</mn><mn>8</mn></mfrac></mrow></mfrac></math>";
    test_braille("Nemeth", expr, "⠠⠹⠢⠠⠌⠲⠸⠹⠒⠌⠦⠸⠼⠠⠼");
}

#[test]
fn test_036() {
    let expr = "<math><mfrac bevelled=\"true\">
        <mfrac><mn>1</mn><mn>2</mn></mfrac>
        <mfrac><mn>3</mn><mn>4</mn></mfrac></mfrac></math>";
    test_braille("Nemeth", expr, "⠠⠹⠹⠂⠌⠆⠼⠠⠸⠌⠹⠒⠌⠲⠼⠠⠼");
}

#[test]
fn test_037() {
    let expr = "<math>
        <mfrac><mi>a</mi>
        <msup><mi>b</mi>
        <mrow>
        <mfrac>
        <mfrac><mn>3</mn><mn>4</mn></mfrac>
        <mfrac><mn>5</mn><mn>6</mn></mfrac></mfrac></mrow></msup></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠁⠌⠃⠘⠠⠹⠹⠒⠌⠲⠼⠠⠌⠹⠢⠌⠖⠼⠠⠼⠐⠼");
}

#[test]
fn test_038() {
    let expr = "<math>
        <mfrac>
        <mfrac>
        <mrow><mn>1</mn>
        <mfrac><mn>1</mn><mn>4</mn></mfrac></mrow>
        <mrow><mn>1</mn>
        <mfrac><mn>3</mn><mn>5</mn></mfrac></mrow></mfrac><mn>5</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠠⠠⠹⠠⠹⠂⠸⠹⠂⠌⠲⠸⠼⠠⠌⠂⠸⠹⠒⠌⠢⠸⠼⠠⠼⠠⠠⠌⠢⠠⠠⠼");
}

#[test]
fn test_039() {
    let expr = "<math>
        <mfrac>
        <mfrac>
        <mrow><mo stretchy=\"false\">(</mo><mn>1</mn><mo>−</mo><mi>x</mi><mo stretchy=\"false\">)</mo>
        <mfrac><mi>d</mi><mi>dx</mi></mfrac><mo stretchy=\"false\">(</mo><mn>2</mn><mi>x</mi><mo stretchy=\"false\">)</mo><mo>−</mo><mn>2</mn><mi>x</mi>
        <mfrac><mi>d</mi><mi>dx</mi></mfrac><mo stretchy=\"false\">(</mo><mn>1</mn><mo>−</mo><mi>x</mi><mo stretchy=\"false\">)</mo></mrow>
        <mrow><mo stretchy=\"false\">(</mo><mn>1</mn><mo>−</mo><mi>x</mi>
        <msup><mo stretchy=\"false\">)</mo><mn>2</mn></msup></mrow></mfrac>
        <mrow><mn>1</mn><mo>+</mo>
        <msup>
        <mrow><mo>(</mo>
        <mfrac>
        <mrow><mn>2</mn><mi>x</mi></mrow>
        <mrow><mn>1</mn><mo>−</mo><mi>x</mi></mrow></mfrac><mo>)</mo></mrow><mn>2</mn></msup></mrow></mfrac></math>";
    test_braille("Nemeth", expr, "⠠⠠⠹⠠⠹⠷⠂⠤⠭⠾⠹⠙⠌⠙⠭⠼⠷⠆⠭⠾⠤⠆⠭⠹⠙⠌⠙⠭⠼⠷⠂⠤⠭⠾⠠⠌⠷⠂⠤⠭⠾⠘⠆⠐⠠⠼⠠⠠⠌⠂⠬⠷⠹⠆⠭⠌⠂⠤⠭⠼⠾⠘⠆⠐⠠⠠⠼");
}

#[test]
fn test_040() {
    let expr = "<math>
        <msqrt><mn>2</mn></msqrt><mo>=</mo>
        <mrow><mn>1</mn><mo>+</mo>
        <mfrac><mn>1</mn>
        <mrow><mn>2</mn><mo>+</mo>
        <mfrac><mn>1</mn>
        <mrow><mn>2</mn><mo>+</mo>
        <mfrac><mn>1</mn>
        <mrow><mn>2</mn><mo>+</mo>
        <mfrac><mn>1</mn>
        <mrow><mn>2</mn><mo>+</mo><mo>…</mo></mrow></mfrac></mrow></mfrac></mrow></mfrac></mrow></mfrac></mrow></math>";
    test_braille("Nemeth", expr, "⠜⠆⠻⠀⠨⠅⠀⠼⠂⠬⠠⠠⠠⠹⠂⠠⠠⠠⠌⠆⠬⠠⠠⠹⠂⠠⠠⠌⠆⠬⠠⠹⠂⠠⠌⠆⠬⠹⠂⠌⠆⠬⠀⠄⠄⠄⠼⠠⠼⠠⠠⠼⠠⠠⠠⠼");
}

#[test]
fn test_041() {
    let expr = "<math><msqrt><mn>2</mn></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠆⠻");
}

#[test]
fn test_042() {
    let expr = "<math><msqrt><mi>x</mi><mo>+</mo><mi>y</mi></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠬⠽⠻");
}

#[test]
fn test_043() {
    let expr = "<math>
        <msqrt>
        <msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>1</mn></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠘⠆⠐⠬⠂⠻");
}

#[test]
fn test_044() {
    let expr = "<math>
        <msqrt>
        <msup><mi>x</mi><mn>2</mn></msup><mo>+</mo>
        <msup><mi>y</mi><mn>2</mn></msup></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠘⠆⠐⠬⠽⠘⠆⠐⠻");
}

#[test]
fn test_045() {
    let expr = "<math><msqrt><mfrac><mi>x</mi><mi>y</mi></mfrac></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠹⠭⠌⠽⠼⠻");
}

#[test]
fn test_046() {
    let expr = "<math><mn>3</mn><msqrt><mi>a</mi></msqrt></math>";
    test_braille("Nemeth", expr, "⠼⠒⠜⠁⠻");
}

#[test]
fn test_047() {
    let expr = "<math><msup><msqrt><mi>x</mi></msqrt><mn>3</mn></msup></math>";
    test_braille("Nemeth", expr, "⠜⠭⠻⠘⠒");
}

