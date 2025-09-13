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
    test("de", "ClearSpeak", expr, "die summe von n ist gleich 1 bis 10 von n");
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
    test("de", "ClearSpeak", expr, "die summe durch groß s von i");
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
    test("de", "ClearSpeak", expr, "die summe von n ist gleich 1 bis 10 von n");
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
    test("de", "ClearSpeak", expr, "die summe durch groß s von i");
}

#[test]
fn sum() {
    let expr = "<math>
            <mo>∑</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("de", "ClearSpeak", expr, "die summe von a sub i");
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
    test("de", "ClearSpeak", expr, "die produkt von n ist gleich 1 bis 10 von n");
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
    test("de", "ClearSpeak", expr, "die produkt durch groß s von i");
}

#[test]
fn product() {
    let expr = "<math>
            <mo>∏</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("de", "ClearSpeak", expr, "die produkt von a sub i");
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
    test("de", "ClearSpeak", expr, "die N-stufiger Durchschnitt von i ist gleich 1 bis 10 von; groß s sub i");
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
    test("de", "ClearSpeak", expr, "die N-stufiger Durchschnitt durch groß c von; groß s sub i");
}

#[test]
fn intersection() {
    let expr = "<math>
            <mo>⋂</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("de", "ClearSpeak", expr, "die N-stufiger Durchschnitt von groß s sub i");
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
    test("de", "ClearSpeak", expr, "die N-stufige Vereinigung von i ist gleich 1 bis 10 von; groß s sub i");
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
    test("de", "ClearSpeak", expr, "die N-stufige Vereinigung durch groß c von; groß s sub i");
}

#[test]
fn union() {
    let expr = "<math>
            <mo>⋃</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("de", "ClearSpeak", expr, "die N-stufige Vereinigung von groß s sub i");
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
    test("de", "ClearSpeak", expr, "die integral von 0 bis 1 von, f von x; d x");
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
    test("de", "ClearSpeak", expr, "die integral durch die reellen zahlen von; f von x d x");
}

#[test]
fn integral() {
    let expr = "<math>
            <mo>∫</mo>
            <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            <mi>d</mi><mi>x</mi>
            </math>";
    test("de", "ClearSpeak", expr, "die integral von f von x d x");
}