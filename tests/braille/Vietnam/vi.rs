// Based on UEB test cases and apply for Vietnamese Math Braille cases.
// Many test cases are taken from the official Vietnamese Braille code 2019, and from Mr. Nguyễn Quyết Thắng, a blind math teacher at Saigon NDC school for the blind.
// Functions are named as its type + section number.
use crate::common::*;

#[test]
fn subset_1a() {
    let expr = "<math><mrow><mi>A</mi><mo>=</mo><mfenced close="}" open="{"><mrow><mn>1</mn><mo>;</mo><mn>2</mn><mo>;</mo><mn>3</mn><mo>;</mo><mn>4</mn><mo>;</mo><mn>5</mn><mo>;</mo><mn>...</mn><mo>;</mo><mn>100</mn></mrow></mfenced></mrow></math>";
    test_braille("UEB", expr, "⠨⠁⠐⠶⠸⠣⠼⠁⠆⠼⠃⠆⠼⠉⠆⠼⠙⠆⠼⠑⠆⠄⠄⠄⠆⠼⠁⠚⠚⠸⠜");
}

#[test]
fn subset_1b() {
    let expr = "<math><mrow><mi>x</mi><mo>&#x2208;</mo><mi>N</mi><mo>&#x007C;</mo><mn>1</mn><mo>&#x2264;</mo><mi>x</mi><mo>&#x2264;</mo><mn>10</mn></mrow></math>";
    test_braille("UEB", expr, "⠭⠈⠑⠨⠝⠸⠳⠼⠁⠐⠪⠶⠭⠐⠪⠶⠼⠁⠚");
}

#[test]
fn subset_1c() {
    let expr = "<math><mrow><mo>&#x2200;</mo><mi>x</mi><mo>&#x2208;</mo><mi>R</mi><mo>&#x007C;</mo><msup><mi>x</mi><mn>2</mn></msup><mo>&#x2265;</mo><mn>0</mn></mrow></math>";
    test_braille("UEB", expr, "⠘⠁⠭⠈⠑⠨⠗⠸⠳⠭⠔⠼⠃⠐⠕⠶⠼⠚");
}

