use crate::common::*;


#[test]
fn log_sup_sub() {
    let expr = "<math><mrow><msubsup><mi mathvariant='normal' ame-texclass='op'>log</mi><mn>10</mn><mn>20</mn></msubsup><mo>&#x2061;</mo><mi>x</mi></mrow></math>";
    test("vi", "ClearSpeak", expr, "lóc mũ 20 cơ số 10; của x");
}

#[test]
fn number_1() {
    let expr = "<math><mn>3.000,12</mn></math>";
    test("vi", "ClearSpeak", expr, "3.000,12");
}

#[test]
fn number_2() {
    let expr = "<math><mn>3,14</mn></math>";
    test("vi", "ClearSpeak", expr, "3,14");
}

#[test]
fn number_1a() {
    let expr = "<math><mn>3,000.12</mn></math>";
    test("vi", "ClearSpeak", expr, "3.000,12");
}

#[test]
fn number_2a() {
    let expr = "<math><mn>3.14</mn></math>";
    test("vi", "ClearSpeak", expr, "3,14");
}

#[test]
fn vi_units_1() {
    let expr = "<math><mrow><mn>1</mn><mi>t</mi><mi>&#x1EA5;</mi><mi>n</mi><mn>10</mn><mi>t</mi><mi>&#x1EA1;</mi><mn>100</mn><mi>y</mi><mi>&#x1EBF;</mi><mi>n</mi><mi>v</mi><mi>&#xE0;</mi><mn>4</mn><mi>l</mi><mi>&#xED;</mi><mi>t</mi></mrow></math>";
    test("vi", "ClearSpeak", expr, "1 tấn 10 tạ 100 yến và 4 lít");
}
