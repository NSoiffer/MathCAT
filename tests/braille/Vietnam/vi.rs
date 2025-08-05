// Based on UEB test cases and apply for Vietnamese Math Braille cases.
// Many test cases are taken from the official Vietnamese Braille code 2019, and from Mr. Nguyễn Quyết Thắng, a blind math teacher at Saigon NDC school for the blind.
// Functions are named as its type + section number.
use crate::common::*;

#[test]
fn subset_1a() {
    let expr = "<math><mrow><mi>A</mi><mo>=</mo><mfenced close='}' open='{'><mrow><mn>1</mn><mo>;</mo><mn>2</mn><mo>;</mo><mn>3</mn><mo>;</mo><mn>4</mn><mo>;</mo><mn>5</mn><mo>;</mo><mi>...</mi><mo>;</mo><mn>100</mn></mrow></mfenced></mrow></math>";
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
fn drop_frac_2a() {
    let expr = "<math><mrow> <mfrac><mn>1</mn><mn>2</mn> </mfrac> <mo>+</mo><mfrac><mn>3</mn><mn>4</mn> </mfrac> <mo>&#x2212;</mo><mfrac><mn>3</mn><mn>7</mn> </mfrac> </mrow></math>";
    libmathcat::set_rules_dir(abs_rules_dir_path()).unwrap();
    libmathcat::set_preference("Vietnam_UseDropNumbers".to_string(), "true".to_string()).unwrap();
    test_braille("Vietnam", expr, "⠼⠁⠆⠐⠖⠼⠉⠲⠐⠤⠼⠉⠶");
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
    test_braille("Vietnam", expr, "⠇⠕⠛⠢⠼⠃⠭⠐⠖⠼⠁⠐⠖⠇⠕⠛⠢⠼⠃⠭⠐⠖⠼⠃⠐⠶⠇⠕⠛⠢⠼⠃⠼⠃");
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
    test_braille("Vietnam", expr, "⠻⠎⠭⠐⠖⠻⠉⠭⠐⠶⠩⠼⠃⠱⠻⠎⠈⠣⠭⠐⠖⠆⠰⠏⠌⠼⠙⠰⠈⠜");
}

#[test]
fn lim_8a () {
    let expr = "<math><mrow> <munder><mrow> <mi>lim</mi></mrow><mrow> <mi>x</mi><mo>&#x2192;</mo><mo>&#x221E;</mo></mrow> </munder> <mfrac><mrow> <msup><mi>x</mi><mn>2</mn> </msup> <mo>+</mo><mn>3</mn><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow> <mn>3</mn><msup><mi>x</mi><mn>2</mn> </msup> <mo>&#x2212;</mo><mn>4</mn></mrow> </mfrac> </mrow> </math>";
    test_braille("Vietnam", expr, "⠇⠊⠍⠢⠣⠭⠳⠕⠼⠕⠱⠆⠭⠔⠼⠃⠐⠖⠼⠉⠭⠐⠖⠼⠁⠌⠼⠉⠭⠔⠼⠃⠐⠤⠼⠙⠰");
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
    test_braille("Vietnam", expr, "⠣⠸⠁⠃⠜⠨⠔⠳⠕⠐⠖⠣⠸⠃⠉⠜⠨⠔⠳⠕⠐⠶⠣⠸⠁⠉⠜⠨⠔⠳⠕");
}

#[test]
fn vector_10d () {
    let expr = "<math><mrow> <mo>&#x007C;</mo><mover accent='true'><mrow> <mi>A</mi><mi>B</mi></mrow><mo stretchy='true'>&#x2192;</mo> </mover> <mo>&#x007C;</mo><mo>=</mo><msqrt><mrow> <msubsup><mi>x</mi><mrow> <mi>A</mi><mi>B</mi></mrow><mn>2</mn> </msubsup> <mo>+</mo><msubsup><mi>y</mi><mrow> <mi>A</mi><mi>B</mi></mrow><mn>2</mn> </msubsup> </mrow> </msqrt> </mrow></math>";
    test_braille("Vietnam", expr, "⠸⠳⠣⠸⠁⠃⠜⠨⠔⠳⠕⠸⠳⠐⠶⠩⠭⠢⠣⠸⠁⠃⠜⠔⠼⠃⠐⠖⠽⠢⠣⠸⠁⠃⠜⠔⠼⠃⠱");
}

#[test]
fn greek_1_1 () {
    let expr = "<math><mrow><mi>&#x3B1;</mi><mi>&#x391;</mi></mrow></math>";
    test_braille("Vietnam", expr, "⠰⠁⠸⠁");
}

#[test]
fn log_b2 () {
    let expr = "<math><mrow><msubsup><mi mathvariant='normal' ame-texclass='op'>log</mi><mn>3</mn><mn>2</mn></msubsup><mo>&#x2061;</mo><mi>x</mi><mo ame-texclass='bin' stretchy='false'>+</mo><mn>2</mn><msub><mi mathvariant='normal' ame-texclass='op'>log</mi><mn>2</mn></msub><mo>&#x2061;</mo><mi>x</mi><mo ame-texclass='bin' stretchy='false'>+</mo><mn>3</mn><mo stretchy='false'>=</mo><mn>0</mn></mrow></math>";
    test_braille("Vietnam", expr, "⠇⠕⠛⠔⠼⠃⠢⠼⠉⠭⠐⠖⠼⠃⠇⠕⠛⠢⠼⠃⠭⠐⠖⠼⠉⠐⠶⠼⠚");
}

#[test]
fn rnumber_1_1 () {
    let expr = r#"<math><mrow><mi>A</mi><mo stretchy='false'>=</mo><mo ame-texclass='open' fence='true' stretchy='false'>{</mo><mi>x</mi><mo stretchy='false'>&#x2208;</mo><mi>R</mi><mo ame-texclass='fence' fence='true' stretchy='false'>|</mo><msup><mi>x</mi><mn>2</mn></msup><mo stretchy='false'>&#x2265;</mo><mn>1</mn><mo ame-texclass='close' fence='true' stretchy='false'>}</mo></mrow></math>"#;
    test_braille("Vietnam", expr, "⠨⠁⠐⠶⠸⠣⠭⠈⠑⠨⠗⠸⠳⠭⠔⠼⠃⠐⠕⠶⠼⠁⠸⠜");
}

#[test]
fn mtable_1_1 () {
    let expr = "<math><mrow><mfenced close='' open='{'><mrow><mtable columnalign='left' equalrows='true' equalcolumns='true'><mtr columnalign='left'><mtd columnalign='left'><mrow><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mn>5</mn></mrow></mtd></mtr><mtr columnalign='left'><mtd columnalign='left'><mrow><mn>2</mn><mi>x</mi><mo>&#x2212;</mo><mi>y</mi><mo>=</mo><mn>1</mn></mrow></mtd></mtr></mtable></mrow></mfenced></mrow></math>";
    test_braille("Vietnam", expr, "⠸⠣⠭⠐⠖⠽⠐⠶⠼⠑⣍⠸⠣⠼⠃⠭⠐⠤⠽⠐⠶⠼⠁");
}

#[test]
fn mtable_1_2 () {
    let expr = r#"<math><mrow><mfenced close='' open='['><mrow><mtable equalrows='true' equalcolumns='true'><mtr><mtd><mrow><mi>x</mi><mo>=</mo><mn>5</mn></mrow></mtd></mtr><mtr><mtd><mrow><mi>x</mi><mo>=</mo><mo>&#x2212;</mo><mn>7</mn></mrow></mtd></mtr></mtable></mrow></mfenced></mrow></math>"#;
    test_braille("Vietnam", expr, "⠨⠣⠭⠐⠶⠼⠑⣍⠨⠣⠭⠐⠶⠐⠤⠼⠛");
}

#[test]
fn number_1 () {
    let expr = "<math><mn>3.000,12</mn></math>";
    test_braille("Vietnam", expr, "⠼⠉⠄⠚⠚⠚⠂⠁⠃");
}

#[test]
fn number_1a () {
    let expr = "<math><mn>3,000.12</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠉⠄⠚⠚⠚⠂⠁⠃");
}

#[test]
fn number_2 () {
    let expr = "<math><mn>3,14</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠉⠂⠁⠙");
}

#[test]
fn number_2a () {
    let expr = "<math><mn>3.14</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠉⠂⠁⠙");
}

#[test]
fn number_3 () {
    let expr = "<math><mn>1.000</mn></math>";
    test_braille("Vietnam", expr, "⠼⠁⠄⠚⠚⠚");
}

#[test]
#[ignore]
fn number_3a () {
    let expr = "<math><mn>1,000</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠁⠄⠚⠚⠚");
}

#[test]
fn number_3b () {
    let expr = "<math><mn>1.234</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠁⠄⠃⠉⠙");
}

#[test]
fn number_3c () {
    let expr = "<math><mn>1,234</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠁⠂⠃⠉⠙");
}

#[test]
fn number_4 () {
    let expr = "<math><mn>1.000.000</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠁⠄⠚⠚⠚⠄⠚⠚⠚");
}

#[test]
fn number_4a () {
    let expr = "<math><mn>1,000,000</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠁⠄⠚⠚⠚⠄⠚⠚⠚");
}

#[test]
fn number_5 () {
    let expr = "<math><mn>123.456.789,987</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠁⠃⠉⠄⠙⠑⠋⠄⠛⠓⠊⠂⠊⠓⠛");
}

#[test]
fn number_5a () {
    let expr = "<math><mn>123,456,789.987</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠁⠃⠉⠄⠙⠑⠋⠄⠛⠓⠊⠂⠊⠓⠛");
}

#[test]
fn number_6 () {
    let expr = "<math><mn>,57</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠂⠑⠛");
}

#[test]
fn number_6a () {
    let expr = "<math><mn>.57</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠂⠑⠛");
}

#[test]
fn number_6b () {
    let expr = "<math><mn>0,57</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠚⠂⠑⠛");
}

#[test]
fn number_6c () {
    let expr = "<math><mn>0.57</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")],  expr, "⠼⠚⠂⠑⠛");
}

#[test]
fn number_7 () {
    let expr = "<math><mn>,578</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠂⠑⠛⠓");
}

#[test]
fn number_7a () {
    let expr = "<math><mn>.578</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")],  expr, "⠼⠂⠑⠛⠓");
}

#[test]
fn number_7b () {
    let expr = "<math><mn>0,578</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠚⠂⠑⠛⠓");
}

#[test]
fn number_7c () {
    let expr = "<math><mn>0.578</mn></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")],  expr, "⠼⠚⠂⠑⠛⠓");
}

#[test]
fn meter_1 () {
    let expr = "<math><mrow><mn>5,72</mn><mi mathvariant='normal'>m</mi><mo ame-texclass='ord' stretchy='false'>/</mo><mn>10</mn><mo stretchy='false'>=</mo><mn>57,2</mn><mi>cm</mi></mrow></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠑⠂⠛⠃⠀⠍⠐⠲⠼⠁⠚⠐⠶⠼⠑⠛⠂⠃⠀⠉⠍");
}

#[test]
fn meter_2 () {
    let expr = "<math><mrow><mn>1</mn><mi>km</mi><mo stretchy='false'>=</mo><mn>10</mn><mi>hm</mi><mo stretchy='false'>=</mo><mn>100</mn><mi>dam</mi><mo stretchy='false'>=</mo><mn>1.000</mn><mi mathvariant='normal'>m</mi><mo stretchy='false'>=</mo><mn>10.000</mn><mi>dm</mi><mo stretchy='false'>=</mo><mn>100.000</mn><mi>cm</mi><mo stretchy='false'>=</mo><mn>1.000.000</mn><mi>mm</mi></mrow></math>";
    test_braille_prefs("Vietnam", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠁⠀⠅⠍⠐⠶⠼⠁⠚⠀⠓⠍⠐⠶⠼⠁⠚⠚⠀⠙⠁⠍⠐⠶⠼⠁⠄⠚⠚⠚⠀⠍⠐⠶⠼⠁⠚⠄⠚⠚⠚⠀⠙⠍⠐⠶⠼⠁⠚⠚⠄⠚⠚⠚⠀⠉⠍⠐⠶⠼⠁⠄⠚⠚⠚⠄⠚⠚⠚⠀⠍⠍");
}

#[test]
fn gram_1 () {
    let expr = "<math><mrow><mn>1</mn><mi>tấn</mi><mo stretchy='false'>=</mo><mn>10</mn><mi>tạ</mi><mo stretchy='false'>=</mo><mn>100</mn><mi>yến</mi><mo stretchy='false'>=</mo><mn>1.000</mn><mi>kg</mi><mo stretchy='false'>=</mo><mn>10.000</mn><mi>hg</mi><mo stretchy='false'>=</mo><mn>100.000</mn><mi>dag</mi><mo stretchy='false'>=</mo><mn>1.000.000</mn><mi mathvariant='normal'>g</mi></mrow></math>";
    test_braille("Vietnam", expr, "⠼⠁⠀⠞⠔⠡⠝⠐⠶⠼⠁⠚⠀⠞⠠⠁⠐⠶⠼⠁⠚⠚⠀⠔⠽⠣⠝⠐⠶⠼⠁⠄⠚⠚⠚⠀⠅⠛⠐⠶⠼⠁⠚⠄⠚⠚⠚⠀⠓⠛⠐⠶⠼⠁⠚⠚⠄⠚⠚⠚⠀⠙⠁⠛⠐⠶⠼⠁⠄⠚⠚⠚⠄⠚⠚⠚⠀⠛");
}

#[test]
fn liquid_1 () {
    let expr = "<math><mrow><mn>1</mn><mi>l&#xED;t</mi><mo stretchy='false'>=</mo><mn>1</mn><mi mathvariant='normal'>l</mi><mo stretchy='false'>=</mo><mn>1.000</mn><mi>ml</mi></mrow></math>";
    test_braille("Vietnam", expr, "⠼⠁⠀⠇⠔⠊⠞⠐⠶⠼⠁⠀⠇⠐⠶⠼⠁⠄⠚⠚⠚⠀⠍⠇");
}

#[test]
fn feet_1 () {
    let expr = "<math><mrow><mn>1</mn><mi>ft</mi><mo ame-texclass='bin' stretchy='false'>+</mo><mn>3</mn><mi>in</mi></mrow></math>";
    test_braille("Vietnam", expr, "⠼⠁⠀⠋⠞⠐⠖⠼⠉⠀⠊⠝");
}

#[test]
fn cap_1 () {
    let expr = "<math><mrow><mi>A</mi><mi>B</mi><mo ame-texclass='bin' stretchy='false'>+</mo><mi>C</mi><mi>d</mi><mo stretchy='false'>=</mo><mi>e</mi><mi>F</mi></mrow></math>";
    test_braille("Vietnam", expr, "⠸⠁⠃⠐⠖⠨⠉⠙⠐⠶⠑⠨⠋");
}

#[test]
fn cap_2 () {
    let expr = "<math><mrow><mi>AB</mi><mo ame-texclass='bin' stretchy='false'>+</mo><mi>Cd</mi><mo stretchy='false'>=</mo><mi>eF</mi></mrow></math>";
    test_braille("Vietnam", expr, "⠸⠁⠃⠐⠖⠨⠉⠙⠐⠶⠑⠨⠋");
}

#[test]
fn vi_letters () {
    let expr = "<math><mrow><mtext>Cho Ph&#x1B0;&#x1A1;ng Tr&#xEC;nh</mtext><mtext>&#xA0;</mtext><mi>A</mi><mi>x</mi><mo ame-texclass='bin' stretchy='false'>+</mo><mi>B</mi><mo stretchy='false'>=</mo><mn>0</mn><mtext>&#xA0;</mtext><mtext>TA &#x110;&#x1AF;&#x1EE2;C</mtext><mtext>&#xA0;</mtext><mi>x</mi></mrow></math>";
    test_braille("Vietnam", expr, "⠨⠉⠓⠕⠀⠨⠏⠓⠳⠪⠝⠛⠀⠨⠞⠗⠰⠊⠝⠓⠀⠨⠁⠭⠐⠖⠨⠃⠐⠶⠼⠚⠀⠸⠞⠁⠀⠸⠮⠳⠠⠪⠉⠀⠭");
}


// Chemistry test cases

#[test]
fn salt() {
  let expr = "<math><mi>Na</mi><mi>Cl</mi></math>";
  test_braille("Vietnam", expr, "⠨⠝⠁⠨⠉⠇");
}

#[test]
fn water() {
  let expr = "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>";
  test_braille("Vietnam", expr, "⠨⠓⠢⠼⠃⠨⠕");
}

#[test]
fn carbon() {
  let expr = "<math><mi>C</mi></math>";     // not enough to trigger recognition
  test_braille("Vietnam", expr, "⠨⠉");
}

#[test]
fn sulfate() {
  let expr = "<math><mrow><msup>
          <mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
          <mrow><mn>2</mn><mo>&#x2212;</mo></mrow>
      </msup></mrow></math>";
  test_braille("Vietnam", expr, "⠨⠣⠸⠎⠕⠢⠼⠙⠨⠜⠔⠣⠼⠃⠐⠤⠱");
  // When two or more continuous cap letters in one substance, and without sub or super script divided, dots 456 as word cap sign.
}

#[test]
fn aluminum_sulfate() {
  let expr = "<math><mrow><msub><mi>Al</mi><mn>2</mn></msub>
          <msub><mrow><mo>(</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>)</mo></mrow><mn>3</mn></msub></mrow></math>";
  test_braille("Vietnam", expr, "⠨⠁⠇⠢⠼⠃⠈⠣⠸⠎⠕⠢⠼⠙⠈⠜⠢⠼⠉");
}

#[test]
fn ethanol_bonds() {
  let expr = "<math>
          <mrow>
              <mi>C</mi>
              <msub>  <mi>H</mi> <mn>3</mn> </msub>
              <mo>&#x2212;</mo>
              <mi>C</mi>
              <msub>  <mi>H</mi> <mn>2</mn> </msub>
              <mo>&#x2212;</mo>
              <mi>O</mi>
              <mi>H</mi>
          </mrow>
      </math>";
  test_braille("Vietnam", expr, "⠸⠉⠓⠢⠼⠉⠤⠸⠉⠓⠢⠼⠃⠤⠸⠕⠓");
}

#[test]
fn dichlorine_hexoxide() {
  let expr = "<math><mrow>
      <msup>
        <mrow><mo>[</mo><mi>Cl</mi><msub><mi>O</mi><mn>2</mn></msub><mo>]</mo></mrow>
        <mo>+</mo>
      </msup>
      <msup>
        <mrow><mo>[</mo><mi>Cl</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
        <mo>-</mo>
      </msup>
    </mrow></math>";
  test_braille("Vietnam", expr, "⠨⠣⠨⠉⠇⠨⠕⠢⠼⠃⠨⠜⠔⠐⠖⠨⠣⠨⠉⠇⠨⠕⠢⠼⠙⠨⠜⠔⠐⠤");
}

#[test]
fn ethylene_with_bond() {
  let expr = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>=</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
  test_braille("Vietnam", expr, "⠨⠓⠢⠼⠃⠨⠉⠭⠸⠉⠓⠢⠼⠃");
}

#[test]
fn ferric_chloride_aq() {
  let expr = "<math><mrow>
        <mi>Fe</mi>
        <msub><mi>Cl</mi><mn>3</mn></msub>
        <mrow><mo>(</mo><mrow><mi>aq</mi></mrow><mo>)</mo></mrow>
    </mrow></math>";
  test_braille("Vietnam", expr, "⠨⠋⠑⠨⠉⠇⠢⠼⠉⠈⠣⠁⠟⠈⠜");
  }

#[test]
fn ethylene_with_colon_bond() {
  let expr = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>::</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
  test_braille("Vietnam", expr, "⠨⠓⠢⠼⠃⠨⠉⠭⠸⠉⠓⠢⠼⠃");
  // Each bond presented with column is translated with ⠒. So, triple column bonds should be ⠒⠒⠒.
}

#[test]
fn beta_decay() {
  let expr = "<math>
      <mmultiscripts>
        <mtext>C</mtext>
        <mprescripts />
        <mn>6</mn>
        <mn>14</mn>
      </mmultiscripts>
      <mo>&#x2192;</mo>
      <mmultiscripts>
        <mtext>N</mtext>
        <mprescripts />
        <mn>7</mn>
        <mn>14</mn>
      </mmultiscripts>
      <mo>+</mo>
      <mmultiscripts>
        <mtext>e</mtext>
        <mprescripts />
        <mrow>
          <mo>&#x2212;</mo>
          <mn>1</mn>
        </mrow>
        <mn>0</mn>
      </mmultiscripts>
    </math>";
  test_braille("Vietnam", expr, "⠨⠉⠢⠮⠼⠋⠔⠞⠼⠁⠙⠳⠕⠨⠝⠢⠮⠼⠛⠔⠞⠼⠁⠙⠐⠖⠑⠢⠮⠣⠐⠤⠼⠁⠜⠔⠞⠼⠚");
}

#[test]
fn hcl_na_yields() {
    let expr = "<math> <mrow>
      <mn>2</mn><mi>H</mi><mi>Cl</mi><mo>+</mo><mn>2</mn><mtext>Na</mtext>
      <mo>&#x2192;</mo>
      <mn>2</mn><mtext>Na</mtext><mi>Cl</mi><mo>+</mo>
      <msub> <mi>H</mi> <mn>2</mn> </msub>
      </mrow>
    </math>";
  test_braille("Vietnam", expr, "⠼⠃⠸⠓⠉⠇⠐⠖⠼⠃⠨⠝⠁⠳⠕⠼⠃⠨⠝⠁⠨⠉⠇⠐⠖⠨⠓⠢⠼⠃");
}

#[test]
fn mhchem_so4_2plus() {
  let expr = "<math>
    <mrow>
      <mrow>
        <mi>SO</mi>
      </mrow>
      <msub>
        <mrow>
          <mrow>
            <mpadded width='0'>
              <mphantom>
                <mi>A</mi>
              </mphantom>
            </mpadded>
          </mrow>
        </mrow>
        <mrow>
          <mrow>
            <mpadded height='0'>
              <mn>4</mn>
            </mpadded>
          </mrow>
        </mrow>
      </msub>
      <msup>
        <mrow>
          <mrow>
            <mpadded width='0'>
              <mphantom>
                <mi>A</mi>
              </mphantom>
            </mpadded>
          </mrow>
        </mrow>
        <mrow>
          <mn>2</mn>
          <mo>+</mo>
        </mrow>
      </msup>
    </mrow>
  </math>";
  test_braille("Vietnam", expr, "⠸⠎⠕⠢⠼⠙⠔⠣⠼⠃⠐⠖⠱");
}

#[test]
fn mhchem_hcl_aq_etc() {
  let expr = "<math>
    <mrow>
      <mn>2</mn>
      <mstyle scriptlevel='0'>
        <mspace width='0.167em'></mspace>
      </mstyle>
      <mrow>
        <mi>HCl</mi>
      </mrow>
      <mspace width='0.111em'></mspace>
      <mo stretchy='false'>(</mo>
      <mrow>
        <mi>aq</mi>
      </mrow>
      <mo stretchy='false'>)</mo>
      <mrow></mrow>
      <mo>+</mo>
      <mrow></mrow>
      <mn>2</mn>
      <mstyle scriptlevel='0'>
        <mspace width='0.167em'></mspace>
      </mstyle>
      <mrow>
        <mi>Na</mi>
      </mrow>
      <mspace width='0.111em'></mspace>
      <mo stretchy='false'>(</mo>
      <mrow>
        <mi mathvariant='normal'>s</mi>
      </mrow>
      <mo stretchy='false'>)</mo>
      <mrow></mrow>
      <mrow>
        <mo stretchy='false'>&#x27F6;</mo>
      </mrow>
      <mrow></mrow>
      <mn>2</mn>
      <mstyle scriptlevel='0'>
        <mspace width='0.167em'></mspace>
      </mstyle>
      <mrow>
        <mi>NaCl</mi>
      </mrow>
      <mspace width='0.111em'></mspace>
      <mo stretchy='false'>(</mo>
      <mrow>
        <mi>aq</mi>
      </mrow>
      <mo stretchy='false'>)</mo>
      <mrow></mrow>
      <mo>+</mo>
      <mrow></mrow>
      <mrow>
        <mi mathvariant='normal'>H</mi>
      </mrow>
      <msub>
        <mrow>
          <mrow>
            <mpadded width='0'>
              <mphantom>
                <mi>A</mi>
              </mphantom>
            </mpadded>
          </mrow>
        </mrow>
        <mrow>
          <mrow>
            <mpadded height='0'>
              <mn>2</mn>
            </mpadded>
          </mrow>
        </mrow>
      </msub>
      <mspace width='0.111em'></mspace>
      <mo stretchy='false'>(</mo>
      <mrow>
        <mi mathvariant='normal'>g</mi>
      </mrow>
      <mo stretchy='false'>)</mo>
    </mrow>
  </math>";
  test_braille("Vietnam", expr, "⠼⠃⠸⠓⠉⠇⠈⠣⠁⠟⠈⠜⠐⠖⠼⠃⠨⠝⠁⠈⠣⠎⠈⠜⠳⠕⠼⠃⠨⠝⠁⠨⠉⠇⠈⠣⠁⠟⠈⠜⠐⠖⠨⠓⠢⠼⠃⠈⠣⠛⠈⠜");
}

#[test]
fn mhchem_barbed_equilibrium() {
  let expr = "<math>
    <mrow data-mjx-texclass='ORD' data-chem-equation='14'>
      <mrow data-changed='added' data-chem-equation='3'>
        <mmultiscripts data-chem-formula='1'>
          <mi data-mjx-texclass='ORD' mathvariant='normal' data-chem-element='1'>H</mi>
          <mn data-mjx-texclass='ORD'>2</mn>
          <none></none>
        </mmultiscripts>
        <mo data-changed='added' data-function-guess='true'>&#x2063;</mo>
        <mrow data-changed='added' data-chem-equation='1'>
          <mo stretchy='false'>(</mo>
          <mi data-mjx-texclass='ORD' mathvariant='normal'>g</mi>
          <mo stretchy='false'>)</mo>
        </mrow>
      </mrow>
      <mo data-chem-equation-op='1'>+</mo>
      <mrow data-changed='added' data-chem-equation='10'>
        <mrow data-changed='added' data-chem-equation='3'>
          <mmultiscripts data-chem-formula='1'>
            <mi data-mjx-texclass='ORD' mathvariant='normal' data-chem-element='1'>I</mi>
            <mn data-mjx-texclass='ORD'>2</mn>
            <none></none>
          </mmultiscripts>
          <mo data-changed='added' data-function-guess='true'>&#x2063;</mo>
          <mrow data-changed='added' data-chem-equation='1'>
            <mo stretchy='false'>(</mo>
            <mi data-mjx-texclass='ORD' mathvariant='normal'>g</mi>
            <mo stretchy='false'>)</mo>
          </mrow>
        </mrow>
        <mo data-changed='added'>&#x2062;</mo>
        <mover data-mjx-texclass='REL'>
          <mrow data-mjx-texclass='ORD' depth='0' height='0' data-changed='added'>
            <mo data-mjx-texclass='ORD' stretchy='false'>↽</mo>
            <mo data-mjx-texclass='ORD'>-</mo>
          </mrow>
          <mrow data-mjx-texclass='ORD' displaystyle='false' scriptlevel='0' data-changed='added'>
            <mo data-mjx-texclass='ORD'>-</mo>
            <mo data-mjx-texclass='ORD' stretchy='false'>⇀</mo>
          </mrow>
        </mover>
        <mo data-changed='added'>&#x2062;</mo>
        <mn>2</mn>
        <mo data-changed='added'>&#x2062;</mo>
        <mrow data-changed='added' data-chem-equation='5'>
          <mi mathvariant='normal' data-chem-element='1'>H</mi>
          <mo data-changed='added'>&#x2063;</mo>
          <mi mathvariant='normal' data-chem-element='1'>I</mi>
          <mo data-changed='added' data-function-guess='true'>&#x2063;</mo>
          <mrow data-changed='added' data-chem-equation='1'>
            <mo stretchy='false'>(</mo>
            <mi data-mjx-texclass='ORD' mathvariant='normal'>g</mi>
            <mo stretchy='false'>)</mo>
          </mrow>
        </mrow>
      </mrow>
    </mrow>
  </math>";
  test_braille("Vietnam", expr, "⠨⠓⠢⠼⠃⠈⠣⠛⠈⠜⠐⠖⠨⠊⠢⠼⠃⠈⠣⠛⠈⠜⠳⠪⠕⠼⠃⠸⠓⠊⠈⠣⠛⠈⠜");
}

#[test]
fn mhchem_roman_in_superscript() {
      let expr = " <math>
      <mrow>
        <mmultiscripts>
          <mi>Fe</mi>
          <none></none>
          <mi>II</mi>
        </mmultiscripts>
        <mo>&#x2063;</mo>
        <mmultiscripts>
          <mi>Fe</mi>
          <none></none>
          <mi data-number='3'>III</mi>
        </mmultiscripts>
        <mo>&#x2063;</mo>
        <mmultiscripts>
          <mi mathvariant='normal' >O</mi>
          <mn>4</mn>
          <none></none>
        </mmultiscripts>
      </mrow>
    </math>";
  test_braille("Vietnam", expr, "⠨⠋⠑⠔⠣⠨⠊⠊⠱⠨⠋⠑⠔⠣⠨⠊⠊⠊⠱⠨⠕⠢⠼⠙");
  // all Roman numbers with single or multiple cap letters, are all used only dot 46.
}

#[test]
fn overparen() {
    let expr = r#"<math><mover accent="false"><mrow><mi>A</mi><mi>B</mi></mrow><mo accent="true">&#x23DC;</mo></mover></math>"#;
    test_braille("Vietnam", expr, "⠫⠠⠗⠸⠁⠃");
}

#[test]
fn vi_text1() {
    let expr = "<math><mtext>quyết giềng quá giệt hằng hỏi lỗi</mtext></math>";
    test_braille("Vietnam", expr, "⠟⠥⠔⠽⠣⠞⠀⠛⠊⠰⠣⠝⠛⠀⠟⠥⠔⠁⠀⠛⠊⠠⠣⠞⠀⠓⠰⠜⠝⠛⠀⠓⠢⠕⠊⠀⠇⠤⠹⠊");
}

#[test]
fn vi_text2() {
    let expr = "<math><mtext>thiết hiền biển diễn điện giết</mtext></math>";
    test_braille("Vietnam", expr, "⠞⠓⠔⠊⠣⠞⠀⠓⠰⠊⠣⠝⠀⠃⠢⠊⠣⠝⠀⠙⠤⠊⠣⠝⠀⠮⠠⠊⠣⠝⠀⠛⠊⠔⠣⠞");
}

#[test]
fn vi_text31() {
    let expr = "<math><mtext>thuấn thuần chuẩn luận quấn quần quẩn quẫn quận</mtext></math>";
    test_braille("Vietnam", expr, "⠞⠓⠔⠥⠡⠝⠀⠞⠓⠰⠥⠡⠝⠀⠉⠓⠢⠥⠡⠝⠀⠇⠠⠥⠡⠝⠀⠟⠥⠔⠡⠝⠀⠟⠥⠰⠡⠝⠀⠟⠥⠢⠡⠝⠀⠟⠥⠤⠡⠝⠀⠟⠥⠠⠡⠝");
}

#[test]
fn vi_text32() {
    let expr = "<math><mtext>thuế huề tuệ quế quệ</mtext></math>";
    test_braille("Vietnam", expr, "⠞⠓⠔⠥⠣⠀⠓⠰⠥⠣⠀⠞⠠⠥⠣⠀⠟⠥⠔⠣⠀⠟⠥⠠⠣");
}

#[test]
fn vi_text33() {
    let expr = "<math><mtext>muống truồng cuổng cuỗng chuộng thuở quở</mtext></math>";
    test_braille("Vietnam", expr, "⠍⠔⠥⠹⠝⠛⠀⠞⠗⠰⠥⠹⠝⠛⠀⠉⠢⠥⠹⠝⠛⠀⠉⠤⠥⠹⠝⠛⠀⠉⠓⠠⠥⠹⠝⠛⠀⠞⠓⠢⠥⠪⠀⠟⠥⠢⠪");
}

#[test]
fn vi_text34() {
    let expr = "<math><mtext>buýt thùy hủy lũy ngụy quý quýt quỳ quỷ quỹ quỵ</mtext></math>";
	test_braille("Vietnam", expr, "⠃⠔⠥⠽⠞⠀⠞⠓⠰⠥⠽⠀⠓⠢⠥⠽⠀⠇⠤⠥⠽⠀⠝⠛⠠⠥⠽⠀⠟⠥⠔⠽⠀⠟⠥⠔⠽⠞⠀⠟⠥⠰⠽⠀⠟⠥⠢⠽⠀⠟⠥⠤⠽⠀⠟⠥⠠⠽");
}

#[test]
fn vi_text35() {
    let expr = "<math><mtext>khuyết thuyền chuyển truyện quyết quyền quyển quyện</mtext></math>";
    test_braille("Vietnam", expr, "⠅⠓⠔⠥⠽⠣⠞⠀⠞⠓⠰⠥⠽⠣⠝⠀⠉⠓⠢⠥⠽⠣⠝⠀⠞⠗⠠⠥⠽⠣⠝⠀⠟⠥⠔⠽⠣⠞⠀⠟⠥⠰⠽⠣⠝⠀⠟⠥⠢⠽⠣⠝⠀⠟⠥⠠⠽⠣⠝");
}

#[test]
fn vi_text41() {
    let expr = "<math><mtext>thoát choàng loãng hoạt thoắc hoặc khoén</mtext></math>";
    test_braille("Vietnam", expr, "⠞⠓⠔⠕⠁⠞⠀⠉⠓⠰⠕⠁⠝⠛⠀⠇⠤⠕⠁⠝⠛⠀⠓⠠⠕⠁⠞⠀⠞⠓⠔⠕⠜⠉⠀⠓⠠⠕⠜⠉⠀⠅⠓⠔⠕⠑⠝");
}

#[test]
fn vi_text42() {
    let expr = "<math><mtext>thướt cược yến yển</mtext></math>";
    test_braille("Vietnam", expr, "⠞⠓⠔⠳⠪⠞⠀⠉⠠⠳⠪⠉⠀⠔⠽⠣⠝⠀⠢⠽⠣⠝");
}

