// Based on UEB test cases and apply for Vietnamese Math Braille cases.
// Many test cases are taken from the official Vietnamese Braille code 2019, and from Mr. Nguyễn Quyết Thắng, a blind math teacher at Saigon NDC school for the blind.
// Functions are named as its type + section number.
use crate::common::*;

#[test]
fn subset_1a() {
    let expr = "<math><mrow><mi>A</mi><mo>=</mo><mfenced close='}' open='{'><mrow><mn>1</mn><mo>;</mo><mn>2</mn><mo>;</mo><mn>3</mn><mo>;</mo><mn>4</mn><mo>;</mo><mn>5</mn><mo>;</mo><mn>...</mn><mo>;</mo><mn>100</mn></mrow></mfenced></mrow></math>";
    test_braille("Vietnam", expr, "⠨⠁⠐⠶⠸⠣⠼⠁⠆⠼⠃⠆⠼⠉⠆⠼⠙⠆⠼⠑⠆⠄⠄⠄⠆⠼⠁⠚⠚⠸⠜");
}

#[test]
fn subset_1b() {
    let expr = "<math><mrow><mi>x</mi><mo>&#x2208;</mo><mi>N</mi><mo>&#x007C;</mo><mn>1</mn><mo>&#x2264;</mo><mi>x</mi><mo>&#x2264;</mo><mn>10</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠭⠈⠑⠨⠝⠸⠳⠼⠁⠐⠪⠶⠭⠐⠪⠶⠼⠁⠚");
}

#[test]
fn subset_1c() {
    let expr = "<math><mrow><mo>&#x2200;</mo><mi>x</mi><mo>&#x2208;</mo><mi>R</mi><mo>&#x007C;</mo><msup><mi>x</mi><mn>2</mn></msup><mo>&#x2265;</mo><mn>0</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠘⠁⠭⠈⠑⠨⠗⠸⠳⠭⠔⠼⠃⠐⠕⠶⠼⠚");
}

#[test]
fn subset_1d() {
    let expr = "<math> <mrow> <mo>&#x2203;</mo><mi>x</mi><mo>&#x2208;</mo><mi>R</mi><mo>&#x007C;</mo><msup><mi>x</mi><mn>2</mn> </msup> <mo>&#x2264;</mo><mn>0</mn></mrow> </math>";
    test_braille("Vietnam", expr, "⠘⠑⠭⠈⠑⠨⠗⠸⠳⠭⠔⠼⠃⠐⠪⠶⠼⠚");
}

#[test]
fn subset_1e() {
    let expr = "<math> <mrow> <mi>x</mi><mo>&#x2209;</mo><mi>N</mi></mrow> </math>";
    test_braille("Vietnam", expr, "⠭⠈⠑⠈⠨⠝");
}

#[test]
fn subset_1f() {
    let expr = "<math> <mrow> <mi>A</mi><mo>&#x2282;</mo><mi>B</mi></mrow> </math>";
    test_braille("Vietnam", expr, "⠨⠁⠘⠣⠨⠃");
}

#[test]
fn subset_1g() {
    let expr = "<math><mrow> <mi>B</mi><mo>&#x2283;</mo><mi>A</mi></mrow></math>";
    test_braille("Vietnam", expr, "⠨⠃⠘⠜⠨⠁");
}

#[test]
fn subset_1h() {
    let expr = "<math> <mrow> <mi>A</mi><mo>&#x2229;</mo><mi>B</mi><mo>=</mo><mo>&#x2205;</mo></mrow> </math>";
    test_braille("Vietnam", expr, "⠨⠁⠨⠦⠨⠃⠐⠶⠈⠚");
}

#[test]
fn subset_1i() {
    let expr = "<math><mrow> <mi>A</mi><mo>=</mo><mi>B</mi><mo>&#x222A;</mo><mi>C</mi></mrow></math>";
    test_braille("Vietnam", expr, "⠨⠁⠐⠶⠨⠃⠨⠖⠨⠉");
}

#[test]
fn subset_1j() {
    let expr = "<math><mrow><mo>&#x2200;</mo><mi>n</mi><mo>&#x2208;</mo><mi>N</mi><mo>&#x007C;</mo><mn>2</mn><mi>n</mi><mo>&#x22EE;</mo><mn>2</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠘⠁⠝⠈⠑⠨⠝⠸⠳⠼⠃⠝⠣⠴⠜⠼⠃");
}

#[test]
fn subset_1k() {
    let expr = "<math><mrow><mo>&#x2203;</mo><mi>x</mi><mo>&#x2208;</mo><mi>Q</mi><mo>&#x007C;</mo><mn>3</mn><mi>x</mi><mo>∤</mo> <mn>3</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠘⠑⠭⠈⠑⠨⠟⠸⠳⠼⠉⠭⠣⠼⠜⠼⠉");
}

#[test]
fn subset_1l() {
    let expr = "<math> <mrow> <mi>D</mi><mo>=</mo><mi>R</mi><mo>&#x005C;</mo><mfenced close='}' open='{'><mrow> <mn>1</mn><mo>;</mo><mn>2</mn></mrow> </mfenced></mrow></math>";
    test_braille("Vietnam", expr, "⠨⠙⠐⠶⠨⠗⠸⠡⠸⠣⠼⠁⠆⠼⠃⠸⠜");
}

#[test]
fn frac_2a() {
    let expr = "<math><mrow> <mfrac><mn>1</mn><mn>2</mn> </mfrac> <mo>+</mo><mfrac><mn>3</mn><mn>4</mn> </mfrac> <mo>&#x2212;</mo><mfrac><mn>3</mn><mn>7</mn> </mfrac> </mrow></math>";
    test_braille("Vietnam", expr, "⠼⠁⠌⠃⠐⠖⠼⠉⠌⠙⠐⠤⠼⠉⠌⠛");
}

#[test]
fn frac_2b() {
    let expr = "<math><mrow> <mfrac><mn>2</mn><mn>3</mn> </mfrac> <mo>&#x00D7;</mo><mfenced><mrow> <mfrac><mn>2</mn><mn>5</mn> </mfrac> <mo>&#x2212;</mo><mfrac><mn>4</mn><mn>9</mn> </mfrac> </mrow> </mfenced><mo>&#x00F7;</mo><mfrac><mn>7</mn><mrow> <mn>15</mn></mrow> </mfrac> </mrow></math>";
    test_braille("Vietnam", expr, "⠼⠃⠌⠉⠐⠦⠈⠣⠼⠃⠌⠑⠐⠤⠼⠙⠌⠊⠈⠜⠐⠲⠼⠛⠌⠁⠑");
}

#[test]
fn frac_2c() {
    let expr = "<math><mrow> <mfrac><mrow> <mi>x</mi><mo>+</mo><mn>2</mn></mrow><mrow> <mi>x</mi><mo>&#x2212;</mo><mn>2</mn></mrow> </mfrac> <mo>=</mo><mfrac><mrow> <mn>2</mn><mi>x</mi><mo>&#x2212;</mo><mn>1</mn></mrow><mrow> <mn>4</mn><mi>x</mi><mo>+</mo><mn>3</mn></mrow> </mfrac> </mrow></math>";
    test_braille("Vietnam", expr, "⠆⠭⠐⠖⠼⠃⠌⠭⠐⠤⠼⠃⠰⠐⠶⠆⠼⠃⠭⠐⠤⠼⠁⠌⠼⠙⠭⠐⠖⠼⠉⠰");
}


#[test]
fn exponent_3a () {
    let expr = "<math> <mrow> <msup><mn>2</mn><mn>3</mn> </msup> <mo>+</mo><msup><mn>2</mn><mn>4</mn> </msup> <mo>&#x2212;</mo><msup><mn>2</mn><mrow> <mn>2022</mn></mrow> </msup> </mrow></math>";
    test_braille("Vietnam", expr, "⠼⠃⠔⠼⠉⠐⠖⠼⠃⠔⠼⠙⠐⠤⠼⠃⠔⠼⠃⠚⠃⠃");
}

#[test]
fn exponent_3b () {
    let expr = "<math><mrow> <msup><mrow> <mo stretchy='false'>(</mo><mi>x</mi><mo>+</mo><mn>2</mn><mo stretchy='false'>)</mo></mrow><mn>3</mn> </msup> <mo>=</mo><msup><mi>x</mi><mn>3</mn> </msup> <mo>+</mo><mn>6</mn><msup><mi>x</mi><mn>2</mn> </msup> <mo>+</mo><mn>12</mn><mi>x</mi><mo>+</mo><mn>8</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠈⠣⠭⠐⠖⠼⠃⠈⠜⠔⠼⠉⠐⠶⠭⠔⠼⠉⠐⠖⠼⠋⠭⠔⠼⠃⠐⠖⠼⠁⠃⠭⠐⠖⠼⠓");
}

#[test]
fn exponent_3c () {
    let expr = "<math><mrow> <msup><mrow> <mfrac><mn>2</mn><mn>3</mn> </mfrac> </mrow><mn>5</mn> </msup> <mo>&#x00D7;</mo><msup><mrow> <mfrac><mn>3</mn><mn>2</mn> </mfrac> </mrow><mn>5</mn> </msup> <mo>=</mo><mn>1</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠼⠃⠌⠉⠔⠼⠑⠐⠦⠼⠉⠌⠃⠔⠼⠑⠐⠶⠼⠁");
}

#[test]
fn exponent_3d () {
    let expr = "<math><mrow> <mfrac><mrow> <msup><mi>x</mi><mn>2</mn> </msup> <mo>+</mo><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow> <mfrac><mn>1</mn><mn>3</mn> </mfrac> <mi>x</mi><mo>&#x2212;</mo><mn>1</mn></mrow> </mfrac> </mrow></math>";
    test_braille("Vietnam", expr, "⠆⠭⠔⠼⠃⠐⠖⠭⠐⠖⠼⠁⠌⠼⠁⠌⠉⠭⠐⠤⠼⠁⠰");
}

#[test]
fn exponent_3e () {
    let expr = "<math><mrow> <msup><mn>2</mn><mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>1</mn></mrow> </msup> <mo>&#x2212;</mo><mn>3</mn><mo>&#x00D7;</mo><msup><mn>2</mn><mi>x</mi> </msup> <mo>+</mo><mn>1</mn><mo>=</mo><mn>0</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠼⠃⠔⠣⠼⠃⠭⠐⠖⠼⠁⠱⠐⠤⠼⠉⠐⠦⠼⠃⠔⠭⠐⠖⠼⠁⠐⠶⠼⠚");
}

#[test]
fn exponent_3f () {
    let expr = "<math> <mrow> <mo>&#x21D4;</mo><mn>2</mn><mo>&#x00D7;</mo><msup><mn>2</mn><mrow> <mn>2</mn><mi>x</mi></mrow> </msup> <mo>&#x2212;</mo><mn>3</mn><mo>&#x00D7;</mo><msup><mn>2</mn><mi>x</mi> </msup> <mo>+</mo><mn>1</mn><mo>=</mo><mn>0</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠳⠪⠶⠕⠼⠃⠐⠦⠼⠃⠔⠣⠼⠃⠭⠱⠐⠤⠼⠉⠐⠦⠼⠃⠔⠭⠐⠖⠼⠁⠐⠶⠼⠚");
}

#[test]
fn exponent_3g () {
    let expr = "<math><mrow> <mi>t</mi><mo>=</mo><msup><mn>2</mn><mi>x</mi> </msup> </mrow></math>";
    test_braille("Vietnam", expr, "⠞⠐⠶⠼⠃⠔⠭");
}

#[test]
fn exponent_3h () {
    let expr = "<math><mrow> <mo stretchy='false'>(</mo><mi>t</mi><mo>&#x2265;</mo><mn>0</mn><mo stretchy='false'>)</mo></mrow></math>";
    test_braille("Vietnam", expr, "⠈⠣⠞⠐⠕⠶⠼⠚⠈⠜");
}

#[test]
fn exponent_3i () {
    let expr = "<math><mrow> <mn>2</mn><msup><mi>t</mi><mn>2</mn> </msup> <mo>&#x2212;</mo><mn>3</mn><mi>t</mi><mo>+</mo><mn>1</mn><mo>=</mo><mn>0</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠼⠃⠞⠔⠼⠃⠐⠤⠼⠉⠞⠐⠖⠼⠁⠐⠶⠼⠚");
}

#[test]
fn exponent_3j () {
    let expr = "<math><mrow> <mi>t</mi><mo>=</mo><mn>1</mn><mo>&#x21D4;</mo><msup><mn>2</mn><mi>x</mi> </msup> <mo>=</mo><mn>1</mn><mo>&#x21D4;</mo><mi>x</mi><mo>=</mo><mn>0</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠞⠐⠶⠼⠁⠳⠪⠶⠕⠼⠃⠔⠭⠐⠶⠼⠁⠳⠪⠶⠕⠭⠐⠶⠼⠚");
}

#[test]
fn exponent_3k () {
    let expr = "<math><mrow> <mi>t</mi><mo>=</mo><mfrac><mn>1</mn><mn>2</mn> </mfrac> <mo>=</mo><msup><mn>2</mn><mi>x</mi> </msup> <mo>=</mo><mfrac><mn>1</mn><mn>2</mn> </mfrac> <mo>&#x21D4;</mo><mi>x</mi><mo>=</mo><mo>&#x2212;</mo><mn>1</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠞⠐⠶⠼⠁⠌⠃⠐⠶⠼⠃⠔⠭⠐⠶⠼⠁⠌⠃⠳⠪⠶⠕⠭⠐⠶⠐⠤⠼⠁");
}

#[test]
fn log_b1 () {
    let expr = "<math><mrow> <msub><mrow> <mi>log</mi></mrow><mn>2</mn> </msub> <mi>x</mi><mo>+</mo><mn>1</mn><mo>+</mo><msub><mrow> <mi>log</mi></mrow><mn>2</mn> </msub> <mi>x</mi><mo>+</mo><mn>2</mn><mo>=</mo><msub><mrow> <mi>log</mi></mrow><mn>2</mn> </msub> <mn>2</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠇⠕⠛⠢⠼⠃⠀⠭⠐⠖⠼⠁⠐⠖⠇⠕⠛⠢⠼⠃⠀⠭⠐⠖⠼⠃⠐⠶⠇⠕⠛⠢⠼⠃⠀⠼⠃");
}


#[test]
fn root_4a () {
    let expr = "<math><mrow> <msqrt><mn>2</mn> </msqrt> <mo>+</mo><msqrt><mn>3</mn> </msqrt> <mo>&#x2212;</mo><msqrt><mn>5</mn> </msqrt> </mrow></math>";
    test_braille("Vietnam", expr, "⠩⠼⠃⠱⠐⠖⠩⠼⠉⠱⠐⠤⠩⠼⠑⠱");
}

#[test]
fn root_4b () {
    let expr = "<math><mrow> <mroot><mrow> <mn>27</mn></mrow><mn>3</mn> </mroot> <mo>+</mo><mroot><mrow> <mn>32</mn></mrow><mn>5</mn> </mroot> </mrow></math>";
    test_braille("Vietnam", expr, "⠩⠔⠼⠉⠼⠃⠛⠱⠐⠖⠩⠔⠼⠑⠼⠉⠃⠱");
}

#[test]
fn root_4c () {
    let expr = "<math> <mrow> <msqrt><mrow> <mi>x</mi><mo>+</mo><mn>1</mn></mrow> </msqrt> <mo>+</mo><msqrt><mrow> <mi>x</mi><mo>+</mo><mn>3</mn></mrow> </msqrt> </mrow></math>";
    test_braille("Vietnam", expr, "⠩⠭⠐⠖⠼⠁⠱⠐⠖⠩⠭⠐⠖⠼⠉⠱");
}

#[test]
fn root_4d () {
    let expr = "<math> <mrow> <mfrac><mrow> <msqrt><mi>x</mi> </msqrt> <mo>+</mo><mn>3</mn></mrow><mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn></mrow> </mfrac> <mo>+</mo><mfrac><mn>5</mn><mrow> <mi>x</mi><mo>+</mo><mn>2</mn><msqrt><mi>x</mi> </msqrt> <mo>&#x2212;</mo><mn>5</mn></mrow> </mfrac> <mo>+</mo><msqrt><mrow> <mfrac><mn>1</mn><mn>2</mn> </mfrac> <mi>x</mi><mo>+</mo><mfrac><mn>3</mn><mn>2</mn> </mfrac> </mrow> </msqrt> </mrow> </math>";
    test_braille("Vietnam", expr, "⠆⠩⠭⠱⠐⠖⠼⠉⠌⠼⠃⠭⠐⠖⠼⠉⠰⠐⠖⠆⠼⠑⠌⠭⠐⠖⠼⠃⠩⠭⠱⠐⠤⠼⠑⠰⠐⠖⠩⠼⠁⠌⠃⠭⠐⠖⠼⠉⠌⠃⠱");
}

#[test]
fn quadratic_5a1 () {
    let expr = "<math> <mrow> <mi>a</mi><msup><mi>x</mi><mn>2</mn> </msup> <mo>+</mo><mi>b</mi><mi>x</mi><mo>+</mo><mi>c</mi><mo>=</mo><mn>0</mn></mrow> <mrow><mtext>&#xA0;</mtext> <mo stretchy='false'>(</mo><mi>a</mi><mo>&#x2260;</mo><mn>0</mn><mo stretchy='false'>)</mo></mrow></math>";
    test_braille("Vietnam", expr, "⠁⠭⠔⠼⠃⠐⠖⠃⠭⠐⠖⠉⠐⠶⠼⠚⠀⠈⠣⠁⠐⠾⠼⠚⠈⠜");
}

#[test]
fn quadratic_5a2 () {
    let expr = "<math><mrow> <mi>&#x0394;</mi><mo>=</mo><msup><mi>b</mi><mn>2</mn> </msup> <mo>&#x2212;</mo><mn>4</mn><mi>a</mi><mi>c</mi></mrow></math>";
    test_braille("Vietnam", expr, "⠸⠙⠐⠶⠃⠔⠼⠃⠐⠤⠼⠙⠠⠁⠉");
}

#[test]
fn quadratic_5a3 () {
    let expr = "<math><mrow> <mi>&#x0394;</mi><mo>&#x003C;</mo><mn>0</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠸⠙⠐⠪⠼⠚");
}

#[test]
fn quadratic_5a4 () {
    let expr = "<math> <mrow> <mi>&#x0394;</mi><mo>=</mo><mn>0</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠸⠙⠐⠶⠼⠚");
}

#[test]
fn quadratic_5a5 () {
    let expr = "<math><mrow> <msub><mi>x</mi><mn>1</mn> </msub> <mo>=</mo><msub><mi>x</mi><mn>2</mn> </msub> <mo>=</mo><mfrac><mrow> <mo>&#x2212;</mo><mi>b</mi></mrow><mrow> <mn>2</mn><mi>a</mi></mrow> </mfrac> </mrow></math>";
    test_braille("Vietnam", expr, "⠭⠢⠼⠁⠐⠶⠭⠢⠼⠃⠐⠶⠆⠐⠤⠃⠌⠼⠃⠠⠁⠰");
}

#[test]
fn quadratic_5a6 () {
    let expr = "<math><mrow> <mi>&#x0394;</mi><mo>&#x003E;</mo><mn>0</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠸⠙⠐⠕⠼⠚");
}

#[test]
fn quadratic_5a7 () {
    let expr = "<math> <mrow> <msub><mi>x</mi><mn>1</mn> </msub> <mo>=</mo><mfrac><mrow> <mo>&#x2212;</mo><mi>b</mi><mo>+</mo><msqrt><mi>&#x0394;</mi> </msqrt> </mrow><mrow> <mn>2</mn><mi>a</mi></mrow> </mfrac> </mrow></math>";
    test_braille("Vietnam", expr, "⠭⠢⠼⠁⠐⠶⠆⠐⠤⠃⠐⠖⠩⠸⠙⠱⠌⠼⠃⠠⠁⠰");
}

#[test]
fn quadratic_5a8 () {
    let expr = "<math><mrow> <msub><mi>x</mi><mn>2</mn> </msub> <mo>=</mo><mfrac><mrow> <mo>&#x2212;</mo><mi>b</mi><mo>&#x2212;</mo><msqrt><mi>&#x0394;</mi> </msqrt> </mrow><mrow> <mn>2</mn><mi>a</mi></mrow> </mfrac> </mrow></math>";
    test_braille("Vietnam", expr, "⠭⠢⠼⠃⠐⠶⠆⠐⠤⠃⠐⠤⠩⠸⠙⠱⠌⠼⠃⠠⠁⠰");
}

#[test]
fn sin_7 () {
    let expr = "<math><mrow> <mi>sin</mi><mi>B</mi><mo>=</mo><mfrac><mrow> <mi>A</mi><mi>C</mi></mrow><mrow> <mi>B</mi><mi>C</mi></mrow> </mfrac> </mrow></math>";
    test_braille("Vietnam", expr, "⠻⠎⠨⠃⠐⠶⠆⠸⠁⠉⠌⠸⠃⠉⠰");
}

#[test]
fn cos_7 () {
    let expr = "<math><mrow> <mi>cos</mi><mi>B</mi><mo>=</mo><mfrac><mrow> <mi>A</mi><mi>B</mi></mrow><mrow> <mi>B</mi><mi>C</mi></mrow> </mfrac> </mrow></math>";
    test_braille("Vietnam", expr, "⠻⠉⠨⠃⠐⠶⠆⠸⠁⠃⠌⠸⠃⠉⠰");
}

#[test]
fn tan_7 () {
    let expr = "<math><mrow> <mi>tan</mi><mi>B</mi><mo>=</mo><mfrac><mrow> <mi>A</mi><mi>C</mi></mrow><mrow> <mi>A</mi><mi>B</mi></mrow> </mfrac> </mrow></math>";
    test_braille("Vietnam", expr, "⠻⠞⠨⠃⠐⠶⠆⠸⠁⠉⠌⠸⠁⠃⠰");
}

#[test]
fn cot_7 () {
    let expr = "<math><mrow> <mi>cot</mi><mi>B</mi><mo>=</mo><mfrac><mrow> <mi>A</mi><mi>B</mi></mrow><mrow> <mi>A</mi><mi>C</mi></mrow> </mfrac> </mrow></math>";
    test_braille("Vietnam", expr, "⠻⠉⠞⠨⠃⠐⠶⠆⠸⠁⠃⠌⠸⠁⠉⠰");
}

#[test]
fn sincos_7e () {
    let expr = "<math><mrow> <mi>sin</mi><mi>x</mi><mo>+</mo><mi>cos</mi><mi>x</mi><mo>=</mo><msqrt><mn>2</mn> </msqrt> <mi>sin</mi><mfenced><mrow> <mi>x</mi><mo>+</mo><mfrac><mi>&#x03C0;</mi><mn>4</mn> </mfrac> </mrow> </mfenced></mrow></math>";
    test_braille("Vietnam", expr, "⠻⠎⠭⠐⠖⠻⠉⠭⠐⠶⠩⠼⠃⠱⠀⠻⠎⠈⠣⠭⠐⠖⠆⠰⠏⠌⠼⠙⠰⠈⠜");
}

#[test]
fn lim_8a () {
    let expr = "<math><mrow> <munder><mrow> <mi>lim</mi></mrow><mrow> <mi>x</mi><mo>&#x2192;</mo><mo>&#x221E;</mo></mrow> </munder> <mfrac><mrow> <msup><mi>x</mi><mn>2</mn> </msup> <mo>+</mo><mn>3</mn><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow> <mn>3</mn><msup><mi>x</mi><mn>2</mn> </msup> <mo>&#x2212;</mo><mn>4</mn></mrow> </mfrac> </mrow> </math>";
    test_braille("Vietnam", expr, "⠇⠊⠍⠢⠭⠳⠕⠼⠕⠆⠭⠔⠼⠃⠐⠖⠼⠉⠭⠐⠖⠼⠁⠌⠼⠉⠭⠔⠼⠃⠐⠤⠼⠙⠰");
}

#[test]
fn prime_9a () {
    let expr = "<math><mrow> <msup><mi>y</mi><mo>&#x2032;</mo> </msup> <mo>=</mo><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠽⠄⠐⠶⠼⠃⠭⠐⠖⠼⠉");
}

#[test]
fn integral_9b () {
    let expr = "<math><mrow> <mstyle displaystyle='true'><mrow><mo>&#x222B;</mo> <mrow><msup> <mi>x</mi> <mn>2</mn></msup><mo>+</mo><mn>3</mn><mi>x</mi><mo>+</mo><mn>2</mn></mrow></mrow> </mstyle><mi>d</mi><mi>x</mi></mrow></math>";
    test_braille("Vietnam", expr, "⠾⠭⠔⠼⠃⠐⠖⠼⠉⠭⠐⠖⠼⠃⠠⠙⠭");
}

#[test]
fn integral_9c () {
    let expr = "<math><mrow> <mstyle displaystyle='true'><mrow><munderover> <mo>&#x222B;</mo> <mn>0</mn> <mrow><mfrac> <mi>&#x03C0;</mi> <mn>4</mn></mfrac></mrow></munderover><mrow> <msup><mrow> <mi>sin</mi></mrow><mn>2</mn> </msup> <mi>x</mi></mrow> </mrow></mstyle><mi>d</mi><mi>x</mi></mrow></math>";
    test_braille("Vietnam", expr, "⠾⠢⠼⠚⠔⠆⠰⠏⠌⠼⠙⠰⠻⠎⠔⠼⠃⠭⠙⠭");
}

#[test]
fn angle_10a () {
    let expr = "<math><mrow> <mover accent='true'><mrow> <mi>A</mi><mi>B</mi><mi>C</mi></mrow><mo stretchy='true'>&#x005E;</mo> </mover> <mo>=</mo><mn>40</mn><mo>&#x00B0;</mo></mrow></math>";
    test_braille("Vietnam", expr, "⠫⠛⠸⠁⠃⠉⠐⠶⠼⠙⠚⠔⠚");
}

#[test]
fn parallel_10b () {
    let expr = "<math><mrow> <mi>A</mi><mi>B</mi><mo>&#x2225;</mo><mi>C</mi><mi>D</mi></mrow></math>";
    test_braille("Vietnam", expr, "⠸⠁⠃⠫⠶⠸⠉⠙");
}

#[test]
fn perp_10b1 () {
    let expr = "<math><mrow> <mi>S</mi><mi>A</mi><mo>&#x22A5;</mo><mo stretchy='false'>(</mo><mi>A</mi><mi>B</mi><mi>C</mi><mi>D</mi><mo stretchy='false'>)</mo></mrow></math>";
    test_braille("Vietnam", expr, "⠸⠎⠁⠫⠧⠈⠣⠸⠁⠃⠉⠙⠈⠜");
}

#[test]
fn vector_10c () {
    let expr = "<math><mrow> <mover accent='true'><mrow> <mi>A</mi><mi>B</mi></mrow><mo stretchy='true'>&#x2192;</mo> </mover> <mo>+</mo><mover accent='true'><mrow> <mi>B</mi><mi>C</mi></mrow><mo stretchy='true'>&#x2192;</mo> </mover> <mo>=</mo><mover accent='true'><mrow> <mi>A</mi><mi>C</mi></mrow><mo stretchy='true'>&#x2192;</mo> </mover> </mrow></math>";
    test_braille("Vietnam", expr, "⠸⠁⠃⠨⠔⠳⠕⠐⠖⠸⠃⠉⠨⠔⠳⠕⠐⠶⠸⠁⠉⠨⠔⠳⠕");
}

#[test]
fn vector_10d () {
    let expr = "<math><mrow> <mo>&#x007C;</mo><mover accent='true'><mrow> <mi>A</mi><mi>B</mi></mrow><mo stretchy='true'>&#x2192;</mo> </mover> <mo>&#x007C;</mo><mo>=</mo><msqrt><mrow> <msubsup><mi>x</mi><mrow> <mi>A</mi><mi>B</mi></mrow><mn>2</mn> </msubsup> <mo>+</mo><msubsup><mi>y</mi><mrow> <mi>A</mi><mi>B</mi></mrow><mn>2</mn> </msubsup> </mrow> </msqrt> </mrow></math>";
    test_braille("Vietnam", expr, "⠸⠳⠸⠁⠃⠨⠔⠳⠕⠸⠳⠐⠶⠩⠭⠢⠣⠸⠁⠃⠜⠔⠼⠃⠐⠖⠽⠢⠣⠸⠁⠃⠜⠔⠼⠃⠱");
}

