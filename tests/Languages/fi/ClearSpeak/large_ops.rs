use crate::common::*;

#[test]
fn sum_both() {
    let expr = "<math>
        <munderover>
            <mo>∑</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </munderover>
        <mi>n</mi>
    </math>";
    test("fi", "ClearSpeak", expr, "summa käy, luvusta n on yhtä suuri kuin 1, lukuun 10; n");
}

#[test]
fn sum_under() {
    let expr = "<math>
        <munder>
            <mo>∑</mo>
            <mi>S</mi>
        </munder>
        <mi>i</mi>
    </math>";
    test("fi", "ClearSpeak", expr, "summa yli iso s, i");
}
#[test]
fn sum_both_msubsup() {
    let expr = "<math>
        <msubsup>
            <mo>∑</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </msubsup>
        <mi>n</mi>
    </math>";
    test("fi", "ClearSpeak", expr, "summa käy, luvusta n on yhtä suuri kuin 1, lukuun 10; n");
}

#[test]
fn sum_sub() {
    let expr = "<math>
        <msub>
            <mo>∑</mo>
            <mi>S</mi>
        </msub>
        <mi>i</mi>
    </math>";
    test("fi", "ClearSpeak", expr, "summa yli iso s, i");
}

#[test]
fn sum() {
    let expr = "<math>
            <mo>∑</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("fi", "ClearSpeak", expr, "summa a ala i");
}

#[test]
fn product_both() {
    let expr = "<math>
        <munderover>
            <mo>∏</mo>
            <mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow>
            <mrow><mn>10</mn></mrow>
        </munderover>
        <mi>n</mi>
    </math>";
    test("fi", "ClearSpeak", expr, "tulo käy, luvusta n on yhtä suuri kuin 1, lukuun 10; n");
}

#[test]
fn product_under() {
    let expr = "<math>
        <munder>
            <mo>∏</mo>
            <mi>S</mi>
        </munder>
        <mi>i</mi>
    </math>";
    test("fi", "ClearSpeak", expr, "tulo yli iso s, i");
}

#[test]
fn product() {
    let expr = "<math>
            <mo>∏</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("fi", "ClearSpeak", expr, "tulo a ala i");
}

#[test]
fn intersection_both() {
    let expr = "<math>
        <munderover>
            <mo>⋂</mo>
            <mrow><mi>i</mi><mo>=</mo><mn>1</mn> </mrow>
            <mn>10</mn>
        </munderover>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("fi", "ClearSpeak", expr, "monikkoleikkaus käy, luvusta i on yhtä suuri kuin 1, lukuun 10; iso s ala i");
}

#[test]
fn intersection_under() {
    let expr = "<math>
        <munder>
            <mo>⋂</mo>
            <mi>C</mi>
        </munder>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("fi", "ClearSpeak", expr, "monikkoleikkaus yli iso c; iso s ala i");
}

#[test]
fn intersection() {
    let expr = "<math>
            <mo>⋂</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("fi", "ClearSpeak", expr, "monikkoleikkaus iso s ala i");
}

#[test]
fn union_both() {
    let expr = "<math>
        <munderover>
            <mo>⋃</mo>
            <mrow><mi>i</mi><mo>=</mo><mn>1</mn> </mrow>
            <mn>10</mn>
        </munderover>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("fi", "ClearSpeak", expr, "monikkounioni käy, luvusta i on yhtä suuri kuin 1, lukuun 10; iso s ala i");
}

#[test]
fn union_under() {
    let expr = "<math>
        <munder>
            <mo>⋃</mo>
            <mi>C</mi>
        </munder>
        <msub><mi>S</mi><mi>i</mi></msub>
    </math>";
    test("fi", "ClearSpeak", expr, "monikkounioni yli iso c; iso s ala i");
}

#[test]
fn union() {
    let expr = "<math>
            <mo>⋃</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("fi", "ClearSpeak", expr, "monikkounioni iso s ala i");
}

#[test]
fn integral_both() {
    let expr = "<math>
            <mrow>
                <msubsup>
                    <mo>∫</mo>
                    <mn>0</mn>
                    <mn>1</mn>
                </msubsup>
                <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            </mrow>
            <mtext>&#x2009;</mtext><mi>d</mi><mi>x</mi>
        </math>";
    test("fi", "ClearSpeak", expr, "integraali, alaraja 0, yläraja 1; f arvolla x; d x");
}

#[test]
fn integral_under() {
    let expr = "<math>
        <munder>
            <mo>∫</mo>
            <mi>ℝ</mi>
        </munder>
        <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
        <mi>d</mi><mi>x</mi>
        </math>";
    test("fi", "ClearSpeak", expr, "integraali yli reaaliluvut; f arvolla x, d x");
}

#[test]
fn integral() {
    let expr = "<math>
            <mo>∫</mo>
            <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            <mi>d</mi><mi>x</mi>
            </math>";
    test("fi", "ClearSpeak", expr, "integraali f arvolla x, d x");
}