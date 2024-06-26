// Swedish braille tests for the basic mathml tags
// These tests are from the Swedish braille authority's publication "Punktskriftens skrivregeler för matematik och naturvetenskap"
//  https://www.mtm.se/globalassets/punktskriftsnamnden/punktskriftens_skrivregler_matematik.pdf
use crate::common::*;

// #[test]
// fn ex_1_1 {
//     let expr = "<math><mn>6</mn><mo>=</mo><mn>1</mn><mo>&#xD7;</mo><mn>2</mn><mo>&#xD7;</mo><mn>3</mn>
//                 <mo>=</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>+</mo><mn>3</mn></math>";
//     test_braille("Swedish", expr, "⠼⠋⠀⠐⠶⠀⠼⠁⠐⠦⠼⠃⠐⠦⠼⠉⠀⠐⠶⠀⠼⠁⠐⠖⠼⠃⠐⠖⠼⠉");
// }

#[test]
fn ex_6_1 {
    let expr = "<math><mrow><mfrac><mi>x</mi><mn>2</mn></mfrac><mo>=</mo><mn>9</mn></mrow></math>";
    test_braille("Swedish", expr, "p1346;p1256;p3456;p12; p2356; p3456;p24;");
}
#[test]
fn ex_6_2 {
    let expr = "<math><mrow><mrow><mn>5</mn><mo>/</mo><mn>5</mn></mrow><mo>=</mo><mn>1</mn></mrow></math>";
    test_braille("Swedish", expr, "p3456;p15;p34;p3456;p15; p2356; p3456;p1;");
}
#[test]
fn ex_6_3 {
    let expr = "<math><mrow><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>+</mo><mfrac><mn>1</mn><mn>3</mn></mfrac><mo>=</mo>
                <mfrac><mn>9</mn><mrow<mn>12</mn></mrow></mfrac><mo>+</mo><mfrac><mn>4</mn><mrow><mn>12</mn></mrow>
                </mfrac><mo>=</mo><mfrac><mrow><mn>13</mn></mrow><mrow><mn>12</mn></mrow></mfrac></mrow></math>";
    test_braille("Swedish", expr, "p3456;p14;p1256;p3456;p145; p256; p3456;p1;p1256;p3456;p14; p2356; p3456;p24;p1256;p3456;p1;p12; p256; p3456;p145;p1256;p3456;p1;p12; p2356; p3456;p1;p14;p1256;p3456;p1;p12;");
}
#[test]
fn ex_6_4 {
    let expr = "<math><mrow><mfrac><mrow><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
                <mrow><mo>(</mo><mi>x</mi><mo>&#x2212;</mo><mn>1</mn><mo>)</mo></mrow></mfrac></mrow></math>";
    test_braille("Swedish", expr, "p236;p1346;p256;p3456;p1;p356;p1256;p236;p1346;p36;p3456;p1;p356;");
}
#[test]
fn ex_6_5 {
    let expr = "<math><mrow><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow>
                <mi>x</mi><mo>&#x2212;</mo><mn>1</mn></mrow></mfrac></mrow></math>";
    test_braille("Swedish", expr, "p123456;p236;p1346;p256;p3456;p1;p1256;p1346;p36;p3456;p1;p123456;p356;");
}
#[test]
fn ex_6_6 {
    let expr = "<math><mrow><mn>2</mn><mfrac><mi>a</mi><mi>b</mi></mfrac><mo>=</mo><mfrac><mrow>
                <mn>2</mn><mi>a</mi></mrow><mi>b</mi></mfrac></mrow></math>";
    test_braille("Swedish", expr, "p3456;p12;p123456;p236;p1;p1256;p12;p123456;p356; p2356; p123456;p236;p3456;p12;p156;p1;p1256;p12;p123456;p356;");
}
#[test]
fn ex_6_7 {
    let expr = "<math><mrow><mfrac><mrow><mi>lg</mi><mi>x</mi></mrow><mrow><mn>10</mn></mrow></mfrac>
                <mo>=</mo><mn>0,1</mn><mi>lg</mi><mi>x</mi></mrow></math>";
    test_braille("Swedish", expr, "p123456;p236;p123;p1245; p1346;p1256;p3456;p1;p245;p123456;p356; p2356; p3456;p245;p2;p1; p123;p1245; p1346;");
}
#[test]
fn ex_6_8 {
    let expr = "<math><mrow><mi>lg</mi><mfrac><mi>x</mi><mrow><mn>10</mn></mrow></mfrac>
                <mo>=</mo><mi>lg</mi><mi>x</mi><mo>&#x2212;</mo><mn>1</mn></mrow></math>";
    test_braille("Swedish", expr, "p123;p1245;p123456;p236;p1346;p1256;p3456;p1;p245;p123456;p356; p2356; p123;p1245; p1346; p36; p3456;p1;");
}
#[test]
fn ex_6_9 {
    let expr = "<math><mrow><mn>3</mn><mfrac><mn>1</mn><mn>4</mn></mfrac><mo>&#x2212;</mo><mn>1</mn><mfrac><mn>3</mn><mn>4</mn>
                </mfrac><mo>=</mo><mn>2</mn><mfrac><mn>5</mn><mn>4</mn></mfrac><mo>&#x2212;</mo><mn>1</mn><mfrac><mn>3</mn><mn>4</mn>
                </mfrac><mo>=</mo><mn>1</mn><mfrac><mn>2</mn><mn>4</mn></mfrac><mo>=</mo><mn>1</mn><mfrac><mn>1</mn><mn>2</mn>
                </mfrac></mrow></math>";
    test_braille("Swedish", expr, "p3456;p14;p3456;p1;p1256;p3456;p145; p36; p3456;p1;p3456;p14;p1256;p3456;p145; p2356; p3456;p12;p3456;p15;p1256;p3456;p145; p36; p3456;p1;p3456;p14;p1256;p3456;p145; p2356; p3456;p1;p3456;p12;p1256;p3456;p145; p2356; p3456;p1;p3456;p1;p1256;p3456;p12;");
}
#[test]
fn ex_6_10 {
    let expr = "<math><mrow><mn>3</mn><mrow><mn>1</mn><mo>/</mo><mn>4</mn></mrow><mo>&#x2212;</mo><mn>1</mn><mrow><mn>3</mn><mo>/</mo><mn>4</mn></mrow>
                <mo>=</mo><mn>2</mn><mrow><mn>5</mn><mo>/</mo><mn>4</mn></mrow><mo>&#x2212;</mo><mn>1</mn><mrow><mn>3</mn><mo>/</mo><mn>4</mn></mrow>
                <mo>=</mo><mn>1</mn><mrow><mn>2</mn><mo>/</mo><mn>4</mn></mrow><mo>=</mo><mn>1</mn><mrow><mn>1</mn><mo>/</mo><mn>2</mn></mrow>
                </mrow></math>";
    test_braille("Swedish", expr, "p3456;p14; p3456;p1;p34;p3456;p145; p36; p3456;p1; p3456;p14;p34;p3456;p145; p2356; p3456;p12; p3456;p15;p34;p3456;p145; p36; p3456;p1; p3456;p14;p34;p3456;p145; p2356; p3456;p1; p3456;p12;p34;p3456;p145; p2356; p3456;p1; p3456;p1;p34;p3456;p12;");
}
#[test]
fn ex_6_11 {
    let expr = "<math><mrow><mfrac><mrow><mn>13</mn><mo>&#x22C5;</mo><mn>7</mn></mrow><mn>2</mn></mfrac></mrow></math>";
    test_braille("Swedish", expr, "p123456;p236;p3456;p1;p14; p12456;p3; p3456;p1245;p1256;p3456;p12;p123456;p356;");
}
#[test]
fn ex_6_12 {
    let expr = "<math><mrow><mfrac><mrow><mn>55</mn><mo>+</mo><mo>(</mo><mo>&#x2212;</mo><mn>18</mn><mo>)</mo><mo>&#x22C5;</mo><mn>2</mn><mo>&#x2212;</mo>
                <mo>(</mo><mo>&#x2212;</mo><mn>63</mn><mo>)</mo></mrow><mrow><mo>(</mo><mo>&#x2212;</mo><mn>3</mn><mo>)</mo><mo>&#x2212;</mo><mo>(</mo>
                <mo>&#x2212;</mo><mn>7</mn><mo>)</mo></mrow></mfrac></mrow></math>";
    test_braille("Swedish", expr, "p123456;p236;p3456;p15;p15;p256;p236;p36;p3456;p1;p125;p356; p12456;p3; p3456;p12;p36;p236;p36;p3456;p124;p14;p356; p1256; p236;p36;p3456;p14;p356;p36;p236;p36;p3456;p1245;p356;p123456;p356;");
}
#[test]
fn ex_6_13 {
    let expr = "<math><mrow><mfrac><mrow><mi>n</mi><mo>(</mo><mi>n</mi><mo>&#x2212;</mo><mn>1</mn><mo>)</mo><mo>&#x2026;</mo><mo>(</mo><mi>n</mi>
                <mo>&#x2212;</mo><mi>k</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow><mrow><mi>k</mi><mo>!</mo></mrow></mfrac></mrow></math>";
    test_braille("Swedish", expr, "p123456;p236;p1345;p236;p1345;p36;p1;p356; p3;p3;p3; p236;p1345;p36;p13;p256;p3456;p1;p356;p1256;p13;p235;p123456;p356;");
}
#[test]
fn ex_6_14 {
    let expr = "<math><mrow><mi>P</mi><mo>(</mo><mi>A</mi><mo>)</mo><mo>=</mo><mfrac><mtext>Number of outcomes in A</mtext>
                <mtext>Total number of outcomes</mtext></mfrac></mrow></math>";
    test_braille("Swedish", expr, "p6;p1234;p236;p6;p1;p356; p2356; p123456;p236;p6;p1345;p136;p134;p12;p15;p1235; p135;p124; p135;p136;p2345;p14;p135;p134;p15;p234; p24;p1345; p6;p1; p1256; p6;p2345;p135;p2345;p1;p123; p1345;p136;p134;p12;p15;p1235; p135;p124; p135;p136;p2345;p14;p135;p134;p15;p234;p123456;p356;");
}
#[test]
fn ex_6_15 {
    let expr = "<math><mrow><msub><mi>b</mi><mn>0</mn></msub><mo>+</mo><mfrac><mrow><msub><mi>a</mi><mn>1</mn></msub></mrow>
                <mrow><msub><mi>b</mi><mn>1</mn></msub><mo>+</mo><mfrac><mrow><msub><mi>a</mi><mn>2</mn></msub></mrow>
                <mrow><msub><mi>b</mi><mn>2</mn></msub><mo>+</mo><mo>&#x2026;</mo><mo>+</mo><mfrac><mrow><msub><mi>a</mi><mi>n</mi></msub></mrow>
                <mrow><msub><mi>b</mi><mi>n</mi></msub></mrow></mfrac></mrow></mfrac></mrow></mfrac></mrow></math>";
    test_braille("Swedish", expr, "p12;p126;p3456;p245; p256; p123456;p236;p1;p126;p3456;p1;p1256;p12;p126;p3456;p1; p256; p123456;p236;p1;p126;p3456;p12;p1256;p12;p126;p3456;p12; p256; p3;p3;p3; p256; p1;p126;p1345;p1256;p12;p126;p1345;p123456;p356;p123456;p356;");
}


