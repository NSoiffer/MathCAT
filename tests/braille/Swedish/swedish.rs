// Swedish braille tests for the basic mathml tags
// These tests are from the Swedish braille authority's publication "Punktskriftens skrivregeler för matematik och naturvetenskap"
//  https://www.mtm.se/globalassets/punktskriftsnamnden/punktskriftens_skrivregler_matematik.pdf
use crate::common::*;

#[test]
fn ex_1_4() {
    let expr= r#"<math><mi>a</mi><mo>&gt;</mo><mi>o</mi><mo>&#x226B;</mo><mi>b</mi></math>"#;
    test_braille("Swedish", expr, "⠁⠀⠼⠕⠀⠕⠀⠼⠕⠕⠀⠃");
}

// CHAPTER 5

#[test]
fn ex_5_1() {
    let expr= r#"<math><mrow><mn>5</mn><mo>+</mo><mn>12</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠑⠀⠲⠀⠼⠁⠃");
}
#[test]
fn ex_5_2() {
    let expr= r#"<math><mrow><mn>9,99</mn><mo>+</mo><mn>0,001</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠊⠂⠊⠊⠀⠲⠀⠼⠚⠂⠚⠚⠁");
}
#[test]
fn ex_5_3() {
    let expr= r#"<math><mrow><mrow><mn>1</mn><mo>/</mo><mn>2</mn></mrow><mo>+</mo><mrow><mn>1</mn><mo>/</mo><mn>2</mn></mrow></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠁⠌⠼⠃⠀⠲⠀⠼⠁⠌⠼⠃");
}
#[test]
fn ex_5_4() {
    let expr= r#"<math><mrow><mi>y</mi><mo>=</mo><mn>5</mn><mo>+</mo><mi>x</mi></mrow></math>"#;
    test_braille("Swedish", expr, "⠽⠀⠶⠀⠼⠑⠀⠲⠀⠭");
}
#[test]
fn ex_5_5() {
    let expr= r#"<math><mrow><mn>613</mn><mo>&#x2212;</mo><mn>221</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠋⠁⠉⠀⠤⠀⠼⠃⠃⠁");
}
#[test]
fn ex_5_6() {
    let expr= r#"<math><mrow><mn>10,1</mn><mo>&#x2212;</mo><mn>3,05</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠁⠚⠂⠁⠀⠤⠀⠼⠉⠂⠚⠑");
}
#[test]
fn ex_5_7() {
    let expr= r#"<math><mrow><mn>3</mn><mfrac><mn>1</mn><mn>4</mn></mfrac><mo>&#x2212;</mo><mn>1</mn><mfrac><mn>3</mn><mn>4</mn></mfrac></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠉⠼⠁⠳⠼⠙⠀⠤⠀⠼⠁⠼⠉⠳⠼⠙");
}
#[test]
fn ex_5_8() {
    let expr= r#"<math><mrow><mn>0,5</mn><mo>+</mo><mn>3,4</mn><mo>+</mo><mn>6</mn><mo>&#x2212;</mo><mn>7,5</mn><mo>&#x2212;</mo><mn>0,02</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠚⠂⠑⠀⠲⠀⠼⠉⠂⠙⠀⠲⠀⠼⠋⠀⠤⠀⠼⠛⠂⠑⠀⠤⠀⠼⠚⠂⠚⠃");
}
#[test]
fn ex_5_9() {
    let expr= r#"<math><mrow><mi>&#x03B1;</mi><mo>&#x00B1;</mo><mn>2</mn><mi>&#x03C0;</mi></mrow></math>"#;
    test_braille("Swedish", expr, "⠰⠁⠀⠨⠲⠤⠀⠼⠃⠰⠏");
}
#[test]
fn ex_5_10() {
    let expr= r#"<math><mrow><mn>15</mn><mo>&#x22C5;</mo><mn>13</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠁⠑⠀⠻⠄⠀⠼⠁⠉");
}
#[test]
fn ex_5_11() {
    let expr= r#"<math><mrow><mn>4.5</mn><mo>&#x22C5;</mo><mn>1.4</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠙⠄⠑⠀⠻⠄⠀⠼⠁⠄⠙");
}
#[test]
fn ex_5_12() {
    let expr= r#"<math><mrow><mn>2</mn><mi>a</mi><mi>b</mi><mo>&#x22C5;</mo><mn>2</mn><mi>a</mi><mi>b</mi><mo>&#x22C5;</mo><mn>2</mn><mi>a</mi><mi>b</mi></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠃⠱⠁⠃⠀⠻⠄⠀⠼⠃⠱⠁⠃⠀⠻⠄⠀⠼⠃⠱⠁⠃");
}
#[test]
fn ex_5_13() {
    let expr= r#"<math><mi mathvariant="bold">r</mi><mo mathvariant="bold">&#x22C5;</mo><mi mathvariant="bold">n</mi><mo>=</mo><mi mathvariant="bold">s</mi><mo mathvariant="bold">&#x22C5;</mo><mi mathvariant="bold">n</mi></math>"#;
    // FIXED: added spaces around · because that is in the TeX list of binary ops -- it is also that way in the spec
    test_braille("Swedish", expr, "⠨⠗⠀⠨⠻⠄⠀⠨⠝⠀⠶⠀⠨⠎⠀⠨⠻⠄⠀⠨⠝");
}
#[test]
fn ex_5_14() {
    let expr= r#"<math><mrow><mtext>LET&#x00A0;</mtext><mi>C</mi><mo>=</mo><mi>A</mi><mo>*</mo><mi>B</mi></mrow></math>"#;
    test_braille("Swedish", expr, "⠠⠠⠇⠑⠞⠀⠠⠉⠀⠶⠀⠠⠁⠔⠠⠃");
}
#[test]
fn ex_5_15() {
    let expr= r#"<math><mrow><mn>24</mn><mo>&#x00D7;</mo><mn>36</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠃⠙⠀⠻⠭⠀⠼⠉⠋");
}
#[test]
fn ex_5_16() {
    let expr= r#"<math><mrow><mfrac><mrow><mn>231</mn></mrow><mn>7</mn></mfrac></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠃⠉⠁⠳⠼⠛");
}
#[test]
fn ex_5_17() {
    let expr= r#"<math><mrow><mfrac><mrow><mn>0,64</mn></mrow><mrow><mn>0,08</mn></mrow></mfrac></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠚⠂⠋⠙⠳⠼⠚⠂⠚⠓");
}
#[test]
fn ex_5_18() {
    let expr= r#"<math><mrow><mrow><mrow><mn>0,2</mn></mrow><mo>/</mo><mrow><mn>0,004</mn></mrow></mrow></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠚⠂⠃⠌⠼⠚⠂⠚⠚⠙");
}
#[test]
fn ex_5_19() {
    let expr= r#"<math><mrow><mtext>Ritningen&#x00A0;var&#x00A0;i&#x00A0;skala&#x00A0;</mtext><mn>1</mn><mo>:</mo><mn>100</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠠⠗⠊⠞⠝⠊⠝⠛⠑⠝⠀⠧⠁⠗⠀⠊⠀⠎⠅⠁⠇⠁⠀⠼⠁⠒⠼⠁⠚⠚");
}

// CHAPTER 6

#[test]
fn ex_6_1() {
    let expr= r#"<math><mrow><mfrac><mi>x</mi><mn>2</mn></mfrac><mo>=</mo><mn>9</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠭⠳⠼⠃⠀⠶⠀⠼⠊");
}
#[test]
fn ex_6_2() {
    let expr= r#"<math><mrow><mrow><mn>5</mn><mo>/</mo><mn>5</mn></mrow><mo>=</mo><mn>1</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠑⠌⠼⠑⠀⠶⠀⠼⠁");
}
#[test]
fn ex_6_3() {
    let expr= r#"<math><mrow><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>+</mo><mfrac><mn>1</mn><mn>3</mn></mfrac><mo>=</mo>
                <mfrac><mn>9</mn><mrow><mn>12</mn></mrow></mfrac><mo>+</mo><mfrac><mn>4</mn><mrow><mn>12</mn></mrow>
                </mfrac><mo>=</mo><mfrac><mrow><mn>13</mn></mrow><mrow><mn>12</mn></mrow></mfrac></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠉⠳⠼⠙⠀⠲⠀⠼⠁⠳⠼⠉⠀⠶⠀⠼⠊⠳⠼⠁⠃⠀⠲⠀⠼⠙⠳⠼⠁⠃⠀⠶⠀⠼⠁⠉⠳⠼⠁⠃");
}
#[test]
fn ex_6_4() {
    let expr= r#"<math><mrow><mfrac><mrow><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
                <mrow><mo>(</mo><mi>x</mi><mo>&#x2212;</mo><mn>1</mn><mo>)</mo></mrow></mfrac></mrow></math>"#;
    test_braille("Swedish", expr, "⠦⠭⠲⠼⠁⠴⠳⠦⠭⠤⠼⠁⠴");
}
#[test]
fn ex_6_5() {
    let expr= r#"<math><mrow><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow>
                <mi>x</mi><mo>&#x2212;</mo><mn>1</mn></mrow></mfrac></mrow></math>"#;
    test_braille("Swedish", expr, "⠿⠦⠭⠲⠼⠁⠳⠭⠤⠼⠁⠿⠴");
}
#[test]
fn ex_6_6() {
    let expr= r#"<math><mrow><mn>2</mn><mfrac><mi>a</mi><mi>b</mi></mfrac><mo>=</mo><mfrac><mrow>
                <mn>2</mn><mi>a</mi></mrow><mi>b</mi></mfrac></mrow></math>"#;
    // FIXED: the example made both fractions have start/end indicators, but the first one doesn't need them and so they were removed from the output
    test_braille("Swedish", expr, "⠼⠃⠱⠁⠳⠃⠀⠶⠀⠿⠦⠼⠃⠱⠁⠳⠃⠿⠴");
}
#[test]
fn ex_6_7() {
    let expr= r#"<math><mrow><mfrac><mrow><mi>lg</mi><mi>x</mi></mrow><mrow><mn>10</mn></mrow></mfrac>
                <mo>=</mo><mn>0,1</mn><mi>lg</mi><mi>x</mi></mrow></math>"#;
    // FIXED: removed space before "lg" as that is not *required* in braille
    test_braille("Swedish", expr, "⠿⠦⠇⠛⠀⠭⠳⠼⠁⠚⠿⠴⠀⠶⠀⠼⠚⠂⠁⠇⠛⠀⠭");
}
#[test]
fn ex_6_8() {
    let expr= r#"<math><mrow><mi>lg</mi><mfrac><mi>x</mi><mrow><mn>10</mn></mrow></mfrac>
                <mo>=</mo><mi>lg</mi><mi>x</mi><mo>&#x2212;</mo><mn>1</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠇⠛⠿⠦⠭⠳⠼⠁⠚⠿⠴⠀⠶⠀⠇⠛⠀⠭⠀⠤⠀⠼⠁");
}
#[test]
fn ex_6_9() {
    let expr= r#"<math><mrow><mn>3</mn><mfrac><mn>1</mn><mn>4</mn></mfrac><mo>&#x2212;</mo><mn>1</mn><mfrac><mn>3</mn><mn>4</mn>
                </mfrac><mo>=</mo><mn>2</mn><mfrac><mn>5</mn><mn>4</mn></mfrac><mo>&#x2212;</mo><mn>1</mn><mfrac><mn>3</mn><mn>4</mn>
                </mfrac><mo>=</mo><mn>1</mn><mfrac><mn>2</mn><mn>4</mn></mfrac><mo>=</mo><mn>1</mn><mfrac><mn>1</mn><mn>2</mn>
                </mfrac></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠉⠼⠁⠳⠼⠙⠀⠤⠀⠼⠁⠼⠉⠳⠼⠙⠀⠶⠀⠼⠃⠼⠑⠳⠼⠙⠀⠤⠀⠼⠁⠼⠉⠳⠼⠙⠀⠶⠀⠼⠁⠼⠃⠳⠼⠙⠀⠶⠀⠼⠁⠼⠁⠳⠼⠃");
}
#[test]
fn ex_6_10() {
    let expr= r#"<math><mrow><mn>3</mn><mrow><mn>1</mn><mo>/</mo><mn>4</mn></mrow><mo>&#x2212;</mo><mn>1</mn><mrow><mn>3</mn><mo>/</mo><mn>4</mn></mrow>
                <mo>=</mo><mn>2</mn><mrow><mn>5</mn><mo>/</mo><mn>4</mn></mrow><mo>&#x2212;</mo><mn>1</mn><mrow><mn>3</mn><mo>/</mo><mn>4</mn></mrow>
                <mo>=</mo><mn>1</mn><mrow><mn>2</mn><mo>/</mo><mn>4</mn></mrow><mo>=</mo><mn>1</mn><mrow><mn>1</mn><mo>/</mo><mn>2</mn></mrow>
                </mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠉⠀⠼⠁⠌⠼⠙⠀⠤⠀⠼⠁⠀⠼⠉⠌⠼⠙⠀⠶⠀⠼⠃⠀⠼⠑⠌⠼⠙⠀⠤⠀⠼⠁⠀⠼⠉⠌⠼⠙⠀⠶⠀⠼⠁⠀⠼⠃⠌⠼⠙⠀⠶⠀⠼⠁⠀⠼⠁⠌⠼⠃");
}
#[test]
fn ex_6_11() {
    let expr= r#"<math><mrow><mfrac><mrow><mn>13</mn><mo>&#x22C5;</mo><mn>7</mn></mrow><mn>2</mn></mfrac></mrow></math>"#;
    // FIXED: removed spacing around &#x22C5; (dot multiply) in numerator
    test_braille("Swedish", expr, "⠿⠦⠼⠁⠉⠻⠄⠼⠛⠳⠼⠃⠿⠴");
}
#[test]
fn ex_6_12() {
    let expr= r#"<math><mrow><mfrac><mrow><mn>55</mn><mo>+</mo><mo>(</mo><mo>&#x2212;</mo><mn>18</mn><mo>)</mo><mo>&#x22C5;</mo><mn>2</mn><mo>&#x2212;</mo>
                <mo>(</mo><mo>&#x2212;</mo><mn>63</mn><mo>)</mo></mrow><mrow><mo>(</mo><mo>&#x2212;</mo><mn>3</mn><mo>)</mo><mo>&#x2212;</mo><mo>(</mo>
                <mo>&#x2212;</mo><mn>7</mn><mo>)</mo></mrow></mfrac></mrow></math>"#;
    // FIXED: removed spacing around &#x22C5; (dot multiply) in numerator [adding space here makes no sense because if anything "-" should have space]
    // FIXED: removed spacing around fraction line sign (⠳)
    test_braille("Swedish", expr, "⠿⠦⠼⠑⠑⠲⠦⠤⠼⠁⠓⠴⠻⠄⠼⠃⠤⠦⠤⠼⠋⠉⠴⠳⠦⠤⠼⠉⠴⠤⠦⠤⠼⠛⠴⠿⠴");
}
#[test]
fn ex_6_13() {
    let expr= r#"<math><mrow><mfrac><mrow><mi>n</mi><mo>(</mo><mi>n</mi><mo>&#x2212;</mo><mn>1</mn><mo>)</mo><mo>&#x2026;</mo><mo>(</mo><mi>n</mi>
                <mo>&#x2212;</mo><mi>k</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow><mrow><mi>k</mi><mo>!</mo></mrow></mfrac></mrow></math>"#;
    test_braille("Swedish", expr, "⠿⠦⠝⠦⠝⠤⠼⠁⠴⠀⠄⠄⠄⠀⠦⠝⠤⠅⠲⠼⠁⠴⠳⠅⠖⠿⠴");
}
#[test]
fn ex_6_14() {
    let expr= r#"<math><mrow><mi>P</mi><mo>(</mo><mi>A</mi><mo>)</mo><mo>=</mo><mfrac><mtext>Number of outcomes in A</mtext>
                <mtext>Total number of outcomes</mtext></mfrac></mrow></math>"#;
    // FIXED: removed spaces around fraction line (⠳)
    test_braille("Swedish", expr, "⠠⠏⠦⠠⠁⠴⠀⠶⠀⠿⠦⠠⠝⠥⠍⠃⠑⠗⠀⠕⠋⠀⠕⠥⠞⠉⠕⠍⠑⠎⠀⠊⠝⠀⠠⠁⠳⠠⠞⠕⠞⠁⠇⠀⠝⠥⠍⠃⠑⠗⠀⠕⠋⠀⠕⠥⠞⠉⠕⠍⠑⠎⠿⠴");
}
#[test]
fn ex_6_15() {
    let expr= r#"<math><mrow><msub><mi>b</mi><mn>0</mn></msub><mo>+</mo><mfrac><mrow><msub><mi>a</mi><mn>1</mn></msub></mrow>
                <mrow><msub><mi>b</mi><mn>1</mn></msub><mo>+</mo><mfrac><mrow><msub><mi>a</mi><mn>2</mn></msub></mrow>
                <mrow><msub><mi>b</mi><mn>2</mn></msub><mo>+</mo><mo>&#x2026;</mo><mo>+</mo><mfrac><mrow><msub><mi>a</mi><mi>n</mi></msub></mrow>
                <mrow><msub><mi>b</mi><mi>n</mi></msub></mrow></mfrac></mrow></mfrac></mrow></mfrac></mrow></math>"#;
    // FIXED: removed spacing around "+"s in continued fraction (left spaces in around ellipsis)
    test_braille("Swedish", expr, "⠃⠣⠼⠚⠀⠲⠀⠿⠦⠁⠣⠼⠁⠳⠃⠣⠼⠁⠲⠿⠦⠁⠣⠼⠃⠳⠃⠣⠼⠃⠲⠀⠄⠄⠄⠀⠲⠁⠣⠝⠳⠃⠣⠝⠿⠴⠿⠴");
}
#[test]
fn ex_6_16() {
    let expr= r#"<math><mrow><mi>z</mi><mo>=</mo><mfrac><mrow><mfrac><mrow><mn>7</mn><mi>x</mi><mo>&#x2212;</mo><mn>6</mn><mi>y</mi></mrow>
                <mrow><mn>5</mn><mi>x</mi><mo>+</mo><mn>8</mn><mi>y</mi></mrow></mfrac></mrow><mrow><mfrac><mrow><mn>7</mn><mi>x</mi><mo>&#x2212;</mo><mn>6</mn><mi>y</mi></mrow>
                <mrow><mn>8</mn><mi>y</mi><mo>+</mo><mn>5</mn><mi>x</mi></mrow></mfrac></mrow></mfrac></mrow></math>"#;
    // FIXED: removed spacing around complex fraction line sign (⠳⠳)
    test_braille("Swedish", expr, "⠵⠀⠶⠀⠿⠦⠼⠛⠭⠤⠼⠋⠽⠳⠼⠑⠭⠲⠼⠓⠽⠿⠴⠳⠳⠿⠦⠼⠛⠭⠤⠼⠋⠽⠳⠼⠓⠽⠲⠼⠑⠭⠿⠴");
}
#[test]
fn ex_6_17() {
let expr= r#"<math><mrow><mi>z</mi><mo>=</mo><mfrac><mrow><mn>1</mn><mo>+</mo><mfrac><mrow><mn>7</mn><mi>x</mi><mo>&#x2212;</mo><mn>6</mn><mi>y</mi></mrow>
                <mrow><mn>5</mn><mi>x</mi><mo>+</mo><mn>8</mn><mi>y</mi></mrow></mfrac></mrow><mrow><mfrac><mrow><mn>7</mn><mi>x</mi><mo>&#x2212;</mo><mn>6</mn><mi>y</mi></mrow>
                <mrow><mn>8</mn><mi>y</mi><mo>+</mo><mn>5</mn><mi>x</mi></mrow></mfrac></mrow></mfrac></mrow></math>"#;
    // FIXED: removed spacing around complex fraction line sign (⠳⠳)
    test_braille("Swedish", expr, "⠵⠀⠶⠀⠿⠦⠼⠁⠲⠿⠦⠼⠛⠭⠤⠼⠋⠽⠳⠼⠑⠭⠲⠼⠓⠽⠿⠴⠳⠳⠿⠦⠼⠛⠭⠤⠼⠋⠽⠳⠼⠓⠽⠲⠼⠑⠭⠿⠴⠿⠴");
}
#[test]
fn ex_6_18() {
    let expr= r#"<math><mrow><mrow><mrow><mfrac><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mrow><mn>6</mn><mi>x</mi></mrow></mfrac></mrow><mo>/</mo><mrow>
                <mfrac><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mrow><mn>2</mn><mi>x</mi></mrow></mfrac><mo>=</mo><mfrac><mn>1</mn><mn>3</mn></mfrac>
                </mrow></mrow></mrow></math>"#;
    // FIXED: removed spacing around complex fraction line sign (⠌⠌)
    test_braille("Swedish", expr, "⠿⠦⠭⠲⠽⠳⠼⠋⠭⠿⠴⠌⠌⠿⠦⠭⠲⠽⠳⠼⠃⠭⠿⠴⠀⠶⠀⠼⠁⠳⠼⠉");
}
#[test]
fn ex_6_19() {
    // FIXED: Added non-breaking spaces around the text
    let expr= r#"<math><mrow><mfrac><mrow><mfrac><mn>9</mn><mn>6</mn></mfrac></mrow><mn>3</mn></mfrac><mo>,</mo><mrow><mrow><mfrac><mn>9</mn><mn>6</mn></mfrac>
                </mrow><mo>/</mo><mn>3</mn></mrow><mtext>&#xA0;och&#xA0;</mtext><mfrac><mrow><mrow><mn>9</mn><mo>/</mo><mn>6</mn></mrow></mrow><mn>3</mn></mfrac>
                <mtext>&#xA0;betecknar talet&#xA0;</mtext><mfrac><mrow><mn>1,5</mn></mrow><mn>3</mn></mfrac></mrow></math>"#;
    // FIXED: removed spacing around complex fraction line signs (⠳⠳ and ⠌⠌)
    test_braille("Swedish", expr, "⠿⠦⠼⠊⠳⠼⠋⠳⠳⠼⠉⠿⠴⠂⠀⠿⠦⠼⠊⠳⠼⠋⠌⠌⠼⠉⠿⠴⠀⠕⠉⠓⠀⠿⠦⠼⠊⠌⠼⠋⠳⠳⠼⠉⠿⠴⠀⠃⠑⠞⠑⠉⠅⠝⠁⠗⠀⠞⠁⠇⠑⠞⠀⠼⠁⠂⠑⠳⠼⠉");
}
#[test]
fn ex_6_20() {
    let expr= r#"<math><mrow><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mrow><mi>a</mi><mo>&#x2212;</mo><mi>b</mi></mrow></mfrac><mo>=</mo><mfrac>
                <mrow><mi>tan</mi><mfrac><mrow><mi>&#x03B1;</mi><mo>+</mo><mi>&#x03B2;</mi></mrow><mn>2</mn></mfrac></mrow><mrow>
                <mi>tan</mi><mfrac><mrow><mi>&#x03B1;</mi><mo>&#x2212;</mo><mi>&#x03B2;</mi></mrow><mn>2</mn></mfrac></mrow></mfrac></mrow></math>"#;
    // FIXED: removed spacing around complex fraction line sign (⠳⠳)
    test_braille("Swedish", expr, "⠿⠦⠁⠲⠃⠳⠁⠤⠃⠿⠴⠀⠶⠀⠿⠦⠞⠁⠝⠿⠦⠰⠁⠲⠰⠃⠳⠼⠃⠿⠴⠳⠳⠞⠁⠝⠿⠦⠰⠁⠤⠰⠃⠳⠼⠃⠿⠴⠿⠴");
}
#[test]
fn ex_6_21() {
    // FIXED: added 'intent' to disambiguate
    let expr= r#"<math><mrow><mo>(</mo><mfrac linethickness='0'><mi>n</mi><mi>k</mi></mfrac><mo>)</mo>
                </mrow><mo>=</mo><mfrac><mrow><mi>n</mi><mo>!</mo></mrow><mrow><mo>(</mo><mi>n</mi><mo>&#x2212;</mo><mi>k</mi><mo>)</mo><mo>!</mo><mi>k</mi><mo>!</mo></mrow>
                </mfrac></math>"#;
    test_braille("Swedish", expr, "⠦⠝⠘⠳⠅⠴⠀⠶⠀⠿⠦⠝⠖⠳⠦⠝⠤⠅⠴⠖⠅⠖⠿⠴");
}
#[test]
fn ex_6_21_mfrac() {
    let expr= r#"<math><mrow intent='binomial($n,$k)'><mo>(</mo><mtable equalrows='true' equalcolumns='true'><mtr><mtd arg='n'><mi>n</mi></mtd></mtr><mtr><mtd arg='k'><mi>k</mi></mtd></mtr></mtable><mo>)</mo>
                </mrow><mo>=</mo><mfrac><mrow><mi>n</mi><mo>!</mo></mrow><mrow><mo>(</mo><mi>n</mi><mo>&#x2212;</mo><mi>k</mi><mo>)</mo><mo>!</mo><mi>k</mi><mo>!</mo></mrow>
                </mfrac></math>"#;
    test_braille("Swedish", expr, "⠦⠝⠘⠳⠅⠴⠀⠶⠀⠿⠦⠝⠖⠳⠦⠝⠤⠅⠴⠖⠅⠖⠿⠴");
}

// CHAPTER 7

#[test]
fn ex_7_1() {
    let expr= r#"<math><mrow><msup><mrow><mo>(</mo><msup><mn>2</mn><mn>5</mn></msup><mo>)</mo></mrow><mn>3</mn></msup><mo>=</mo><msup><mn>2</mn><mrow>
                <mn>15</mn></mrow></msup></mrow></math>"#;
    test_braille("Swedish", expr, "⠦⠼⠃⠬⠼⠑⠴⠬⠼⠉⠀⠶⠀⠼⠃⠬⠼⠁⠑");
}
#[test]
fn ex_7_2() {
    let expr= r#"<math><mrow><msup><mi>x</mi><mn>2</mn></msup><mi>y</mi><mo>&#x22C5;</mo><msup><mi>x</mi><mn>3</mn></msup><msup><mi>y</mi><mn>2</mn></msup></mrow></math>"#;
    test_braille("Swedish", expr, "⠭⠬⠼⠃⠽⠀⠻⠄⠀⠭⠬⠼⠉⠽⠬⠼⠃");
}
#[test]
fn ex_7_3() {
    let expr= r#"<math><mrow><msup><mi>f</mi><mrow><mo>(</mo><mn>2</mn><mi>m</mi><mo>&#x2212;</mo><mn>1</mn><mo>)</mo></mrow></msup></mrow></math>"#;
    test_braille("Swedish", expr, "⠋⠬⠦⠼⠃⠍⠤⠼⠁⠴");
}
#[test]
fn ex_7_4() {
    let expr= r#"<math><mrow><msup><mrow><mn>10</mn></mrow><mrow><mo>&#x2212;</mo><mn>12</mn></mrow></msup></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠁⠚⠬⠤⠼⠁⠃");
}
#[test]
fn ex_7_5() {
    let expr= r#"<math><mrow><msup><mi>z</mi><mi>n</mi></msup><mo>=</mo><msup><mrow><mo>(</mo><mi>r</mi><msup><mi>e</mi><mrow><mi>i</mi><mi>&#x03B8;</mi></mrow></msup>
                <mo>)</mo></mrow><mi>n</mi></msup><mo>=</mo><msup><mi>r</mi><mi>n</mi></msup><msup><mi>e</mi><mrow><mi>i</mi><mi>n</mi><mi>&#x03B8;</mi></mrow></msup>
                </mrow></math>"#;
    test_braille("Swedish", expr, "⠵⠬⠝⠀⠶⠀⠦⠗⠑⠘⠬⠊⠰⠓⠱⠴⠬⠝⠀⠶⠀⠗⠬⠝⠑⠘⠬⠊⠝⠰⠓⠱");
}
#[test]
fn ex_7_6() {
    let expr= r#"<math><mrow><msup><mi>e</mi><mrow><mo>&#x2212;</mo><mo>(</mo><mfrac><mi>&#x03C0;</mi><mn>2</mn></mfrac><mo>+</mo><mn>2</mn><mi>n</mi><mi>&#x03C0;</mi>
                <mo>)</mo><mo>+</mo><mi>i</mi><mi>ln</mi><mn>2</mn></mrow></msup></mrow></math>"#;
    test_braille("Swedish", expr, "⠑⠘⠬⠤⠦⠰⠏⠳⠼⠃⠲⠼⠃⠝⠰⠏⠴⠲⠊⠇⠝⠼⠃⠱");
}
#[test]
fn ex_7_7() {
    let expr= r#"<math><mrow><msup><mn>2</mn><mrow><msup><mn>2</mn><mrow><mi>n</mi><mo>&#x2212;</mo><mn>1</mn></mrow></msup></mrow></msup></mrow></math>"#;
    test_braille("Swedish", expr, "⠼⠃⠬⠼⠃⠘⠬⠝⠤⠼⠁⠱");
}
#[test]
fn ex_7_8() {
    let expr= r#"<math><mrow><mmultiscripts><mi>log</mi><mprescripts/><none/><mi>a</mi></mmultiscripts><mi>x</mi><mo>+</mo><mmultiscripts><mi>log</mi><mprescripts/>
                <none/><mi>a</mi></mmultiscripts><mi>y</mi><mo>=</mo><mmultiscripts><mi>log</mi><mprescripts/><none/><mi>a</mi></mmultiscripts><mi>x</mi>
                <mi>y</mi></mrow></math>"#;
    test_braille("Swedish", expr, "⠬⠁⠇⠕⠛⠀⠭⠀⠲⠀⠬⠁⠇⠕⠛⠀⠽⠀⠶⠀⠬⠁⠇⠕⠛⠀⠭⠽");
}
#[test]
fn ex_7_9() {
    let expr= r#"<math><mrow><msup><mi>e</mi><mrow><mo>&#x2212;</mo><msup><mi>x</mi><mn>2</mn></msup><mo>/</mo><mn>4</mn><mi>a</mi><mo>&#x2212;</mo>
                <msup><mi>y</mi><mn>2</mn></msup><mo>/</mo><mn>4</mn><mi>b</mi><mo>&#x2212;</mo><msup><mi>z</mi><mn>2</mn></msup><mo>/</mo><mn>4</mn><mi>c</mi></mrow>
                </msup></mrow></math>"#;
    test_braille("Swedish", expr, "⠑⠘⠬⠤⠭⠬⠼⠃⠌⠼⠙⠱⠁⠤⠽⠬⠼⠃⠌⠼⠙⠱⠃⠤⠵⠬⠼⠃⠌⠼⠙⠱⠉⠱");
}
#[test]
fn ex_7_10() {
    let expr= r#"<math><mrow><msub><mi>x</mi><mn>1</mn></msub><mo>,</mo><msub><mi>x</mi><mn>2</mn></msub><mo>,</mo><mo>&#x2026;</mo><mo>,</mo><msub><mi>x</mi>
                <mrow><mi>m</mi><mo>&#x2212;</mo><mn>1</mn></mrow></msub></mrow></math>"#;
    test_braille("Swedish", expr, "⠭⠣⠼⠁⠂⠀⠭⠣⠼⠃⠂⠀⠄⠄⠄⠂⠀⠭⠘⠣⠍⠤⠼⠁⠱");
}
#[test]
fn ex_7_11() {
    let expr= r#"<math><mrow><mmultiscripts><mi mathvariant="normal">U</mi><mprescripts/><mrow><mn>92</mn></mrow><mrow><mn>238</mn></mrow></mmultiscripts></mrow></math>"#;
    test_braille("Swedish", expr, "⠣⠼⠊⠃⠬⠼⠃⠉⠓⠠⠥");
}
#[test]
fn ex_7_12() {
    let expr= r#"<math><mrow><msubsup><mrow><mi mathvariant="normal">S</mi><mi mathvariant="normal">O</mi></mrow><mn>4</mn><mrow><mn>2</mn><mo>&#x2212;</mo></mrow></msubsup></mrow></math>"#;
    test_braille("Swedish", expr, "⠠⠠⠎⠕⠣⠼⠙⠘⠬⠼⠃⠤⠱");
}
#[test]
fn ex_7_13() {
    let expr= r#"<math><mrow><mmultiscripts><mi mathvariant="normal">H</mi><mn>1</mn><mo>+</mo><mprescripts/><mn>1</mn><mn>1</mn></mmultiscripts></mrow></math>"#;
    test_braille("Swedish", expr, "⠣⠼⠁⠬⠼⠁⠠⠓⠣⠼⠁⠬⠲");
}
#[test]
fn ex_7_14() {
    let expr= r#"<math><mrow><munder><mrow><mi>lim</mi></mrow><mrow><mi>n</mi><mo>&#x2192;</mo><mo>&#x221E;</mo></mrow></munder><msub><mi>a</mi><mi>n</mi></msub>
                <mtext>&#x00A0;existerar&#x00A0;</mtext><mo>&#x21D4;</mo><munder><mrow><mi>lim</mi></mrow><mtable columnalign='left'><mtr><mtd><mrow><mi>m</mi>
                <mo>&#x2192;</mo><mo>&#x221E;</mo></mrow></mtd></mtr><mtr><mtd><mrow><mi>n</mi><mo>&#x2192;</mo><mo>&#x221E;</mo></mrow></mtd></mtr></mtable></munder>
                <mo>&#x007C;</mo><msub><mi>a</mi><mi>m</mi></msub><mo>&#x2212;</mo><msub><mi>a</mi><mi>n</mi></msub><mo>&#x007C;</mo><mo>=</mo><mn>0</mn></mrow></math>"#;
    // FIXED: added space around '-' (⠤)
    test_braille("Swedish", expr, "⠇⠊⠍⠘⠣⠣⠝⠒⠕⠼⠿⠱⠀⠁⠣⠝⠀⠑⠭⠊⠎⠞⠑⠗⠁⠗⠀⠪⠶⠕⠀⠇⠊⠍⠘⠣⠣⠍⠒⠕⠼⠿⠱⠘⠣⠣⠝⠒⠕⠼⠿⠱⠀⠸⠁⠣⠍⠀⠤⠀⠁⠣⠝⠸⠀⠶⠀⠼⠚");
}
#[test]
fn ex_7_15() {
    let expr= r#"<math><mrow><msup><mi>a</mi><mrow><mo>&#x2212;</mo><mi>b</mi></mrow></msup></mrow></math>"#;
    test_braille("Swedish", expr, "⠁⠬⠤⠃");
}
#[test]
fn ex_7_16() {
    let expr= r#"<math><mrow><mi>a</mi><mover accent='true'><mi>b</mi><mo>&#x00AF;</mo></mover></mrow></math>"#;
    test_braille("Swedish", expr, "⠁⠃⠬⠬⠤");
}
#[test]
fn ex_7_17() {
    let expr= r#"<math><mrow><mover accent='true'><mi>y</mi><mo>&#x02DC;</mo></mover><mo>=</mo><mi>y</mi></mrow></math>"#;
    test_braille("Swedish", expr, "⠽⠬⠬⠒⠀⠶⠀⠽");
}
#[test]
fn ex_7_18() {
    let expr= r#"<math><mrow><munder accentunder='true'><munder accentunder='true'><mi>v</mi><mo stretchy='true'>&#xAF;</mo></munder><mo stretchy='true'>&#xAF;</mo></munder>
                <mo>=</mo><munder accentunder='true'><mi>v</mi><mo stretchy='true'>_</mo></munder></mrow></math>"#;
    test_braille("Swedish", expr, "⠧⠣⠣⠶⠀⠶⠀⠧⠣⠣⠤");
}
#[test]
fn ex_7_19() {
    let expr= r#"<math><mrow><msup><mover accent='true'><mi>&#x03C3;</mi><mo>&#x005E;</mo></mover><mn>2</mn></msup><mo>=</mo><msup><mi>s</mi><mn>2</mn></msup></mrow></math>"#;
    test_braille("Swedish", expr, "⠰⠎⠬⠬⠖⠬⠼⠃⠀⠶⠀⠎⠬⠼⠃");
}
#[test]
fn ex_7_20() {
    let expr= r#"<math><mrow><msup><mover accent='true'><mi>&#x03C9;</mi><mo>&#x00AF;</mo></mover><mn>2</mn></msup><mo>=</mo><mfrac><mi>K</mi><mi>M</mi></mfrac></mrow></math>"#;
    test_braille("Swedish", expr, "⠰⠺⠬⠬⠤⠬⠼⠃⠀⠶⠀⠠⠅⠳⠠⠍");
}
#[test]
fn ex_7_21() {
    let expr= r#"<math><mrow><mi>b</mi><mo>*</mo><mi>a</mi><mo>=</mo><mover accent='true'><mrow><mi>a</mi><mo>*</mo><mi>b</mi></mrow><mo stretchy='true'>&#x00AF;</mo>
                </mover></mrow></math>"#;
    test_braille("Swedish", expr, "⠃⠔⠁⠀⠶⠀⠘⠁⠔⠃⠱⠬⠬⠤");
}
#[test]
fn ex_7_22() {
    let expr= r#"<math><mrow><mover accent='true'><mrow><mi>A</mi><mi>B</mi></mrow><mo stretchy='true'>&#x2192;</mo></mover></mrow></math>"#;
    test_braille("Swedish", expr, "⠘⠠⠠⠁⠃⠱⠬⠬⠒⠕");
}
#[test]
fn ex_7_23() {
    let expr= r#"<math><mrow><mover accent='true'><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo stretchy='true'>&#x005E;</mo></mover><mo>=</mo><mover accent='true'><mi>x</mi>
                <mo>&#x005E;</mo></mover><mo>+</mo><mover accent='true'><mi>y</mi><mo>&#x005E;</mo></mover></mrow></math>"#;
    // FIXED: added spaces around "+" (⠲) on the lefthand side of '=' to be consistent
    test_braille("Swedish", expr, "⠘⠭⠀⠲⠀⠽⠱⠬⠬⠖⠀⠶⠀⠭⠬⠬⠖⠀⠲⠀⠽⠬⠬⠖");
}
#[test]
fn ex_7_24() {
    // FIXED: this was changed to match what 7.24 in the spec (both mathml and braille)
    let expr= r#"<math><mrow><mmultiscripts><munder accentunder='true'><mover accent='true'><mi>x</mi><mo>^</mo></mover><mo>ˇ</mo></munder><mi>a</mi><mi>b</mi>
                <mprescripts/><mn>1</mn><mn>2</mn></mmultiscripts></mrow></math>"#;
    test_braille("Swedish", expr, "⠣⠼⠁⠬⠼⠃⠭⠣⠣⠶⠬⠬⠖⠣⠁⠬⠃");
}
#[test]
fn ex_7_24_munderover() {
    // This uses the "correct" MathML
    let expr= r#"<math><mrow><mmultiscripts><munderover accentunder='true' accent='true'><mi>x</mi><mo>ˇ</mo><mo>^</mo></munderover><mi>a</mi><mi>b</mi>
                <mprescripts/><mn>1</mn><mn>2</mn></mmultiscripts></mrow></math>"#;
    test_braille("Swedish", expr, "⠣⠼⠁⠬⠼⠃⠭⠣⠣⠶⠬⠬⠖⠣⠁⠬⠃");
}
#[test]
fn ex_7_25() {
    let expr= r#"<math><mrow><msqrt><mrow><mn>25</mn></mrow></msqrt><mo>=</mo><mn>5</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠩⠼⠃⠑⠀⠶⠀⠼⠑");
}
#[test]
fn ex_7_26() {
    let expr= r#"<math><mrow><msqrt><mrow><mn>1,5</mn></mrow></msqrt></mrow></math>"#;
    test_braille("Swedish", expr, "⠩⠼⠁⠂⠑");
}
#[test]
fn ex_7_27() {
    let expr= r#"<math><mrow><msqrt><mrow><mo>&#x007C;</mo><mn>5</mn><msub><mi>c</mi><mi>n</mi></msub><mo>&#x007C;</mo></mrow></msqrt></mrow></math>"#;
    test_braille("Swedish", expr, "⠩⠸⠼⠑⠱⠉⠣⠝⠸");
}
#[test]
fn ex_7_28() {
    let expr= r#"<math><mrow><msqrt><mrow><mfrac><mi>a</mi><mn>2</mn></mfrac></mrow></msqrt><mo>=</mo><msqrt><mrow><mo>(</mo><mfrac><mi>a</mi><mn>2</mn></mfrac>
                <mo>)</mo></mrow></msqrt></mrow></math>"#;
    test_braille("Swedish", expr, "⠘⠩⠁⠳⠼⠃⠱⠀⠶⠀⠩⠦⠁⠳⠼⠃⠴");
}
#[test]
fn ex_7_29() {
    let expr= r#"<math><mrow><mi>r</mi><mo>=</mo><mfrac><mi>a</mi><mn>4</mn></mfrac><msqrt><mrow><mfrac><mrow><mn>50</mn><mo>+</mo><mn>22</mn><msqrt><mn>5</mn></msqrt></mrow>
                <mn>5</mn></mfrac></mrow></msqrt></mrow></math>"#;
    // FIXED: removed spaces around fraction line (⠳)
    test_braille("Swedish", expr, "⠗⠀⠶⠀⠁⠳⠼⠙⠩⠿⠦⠼⠑⠚⠲⠼⠃⠃⠩⠼⠑⠳⠼⠑⠿⠴");
}
#[test]
fn ex_7_30() {
    let expr= r#"<math><mrow><msub><mi>s</mi><mi>a</mi></msub><mo>=</mo><msqrt><mrow><mi>b</mi><mi>c</mi><mo>[</mo><mn>1</mn><mo>&#x2212;</mo><msup><mrow><mo>(</mo>
                <mfrac><mi>a</mi><mi>b</mi></mfrac><mo>+</mo><mi>c</mi><mo>)</mo></mrow><mn>2</mn></msup><mo>]</mo></mrow></msqrt></mrow></math>"#;
    // FIXED: added space around "+" (⠲) -- it's on the baseline and there is a space around the "-"
    test_braille("Swedish", expr, "⠎⠣⠁⠀⠶⠀⠘⠩⠃⠉⠷⠼⠁⠀⠤⠀⠦⠁⠳⠃⠀⠲⠀⠉⠴⠬⠼⠃⠾⠱");
}
#[test]
fn ex_7_31() {
    let expr= r#"<math><mrow><mroot><mrow><mn>27</mn></mrow><mn>3</mn></mroot><mo>=</mo><mn>3</mn></mrow></math>"#;
    test_braille("Swedish", expr, "⠬⠼⠉⠩⠼⠃⠛⠀⠶⠀⠼⠉");
}
#[test]
fn ex_7_32() {
    let expr= r#"<math><mrow><mroot><mi>a</mi><mi>n</mi></mroot><mroot><mi>a</mi><mi>m</mi></mroot><mo>=</mo><mroot><mrow><msup><mi>a</mi><mrow><mi>m</mi><mo>+</mo>
                <mi>n</mi></mrow></msup></mrow><mrow><mi>m</mi><mi>n</mi></mrow></mroot></mrow></math>"#;
    test_braille("Swedish", expr, "⠬⠝⠩⠁⠱⠬⠍⠩⠁⠀⠶⠀⠘⠬⠍⠝⠱⠩⠁⠘⠬⠍⠲⠝⠱");
}


#[test]
fn ex_11_4() {
    let expr= r#"<math><mi>a</mi><mo>=</mo>
            <mfrac><mi>F</mi><mi>m</mi></mfrac><mo>=</mo>
            <mfrac><mrow><mi>m</mi><mo>&#xB7;</mo><mfrac><msup><mi>v</mi><mn>2</mn></msup><mi>r</mi></mfrac></mrow><mi>m</mi></mfrac><mo>&#x21D2;</mo>
            <mi>a</mi><mo>=</mo><mi>r</mi><mo>&#xB7;</mo><msup><mi>&#x3C9;</mi><mn>2</mn></msup></math>"#;
    test_braille("Swedish", expr, "⠁⠀⠶⠀⠠⠋⠳⠍⠀⠶⠀⠿⠦⠍⠻⠄⠧⠬⠼⠃⠳⠗⠳⠳⠍⠿⠴⠀⠶⠕⠀⠁⠀⠶⠀⠗⠀⠻⠄⠀⠰⠺⠬⠼⠃");
}

