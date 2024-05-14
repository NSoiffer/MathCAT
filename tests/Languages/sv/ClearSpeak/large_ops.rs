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
    test("sv", "ClearSpeak", expr, "summa från n lika med 1, till 10, av n");
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
    test("sv", "ClearSpeak", expr, "summa över versal s, av i");
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
    test("sv", "ClearSpeak", expr, "summa från n lika med 1, till 10, av n");
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
    test("sv", "ClearSpeak", expr, "summa över versal s, av i");
}

#[test]
fn sum() {
    let expr = "<math>
            <mo>∑</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("sv", "ClearSpeak", expr, "summa av a nedsänkt i");
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
    test("sv", "ClearSpeak", expr, "produkten från n lika med 1, till 10, av n");
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
    test("sv", "ClearSpeak", expr, "produkten över versal s, av i");
}

#[test]
fn product() {
    let expr = "<math>
            <mo>∏</mo>
            <msub><mi>a</mi><mi>i</mi></msub>
    </math>";
    test("sv", "ClearSpeak", expr, "produkten av a nedsänkt i");
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
    test("sv", "ClearSpeak", expr, "snittet från i lika med 1, till 10, av; versal s nedsänkt i");
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
    test("sv", "ClearSpeak", expr, "snittet över versal c, av, versal s nedsänkt i");
}

#[test]
fn intersection() {
    let expr = "<math>
            <mo>⋂</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("sv", "ClearSpeak", expr, "snittet av versal s nedsänkt i");
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
    test("sv", "ClearSpeak", expr, "unionen från i lika med 1, till 10, av; versal s nedsänkt i");
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
    test("sv", "ClearSpeak", expr, "unionen över versal c, av, versal s nedsänkt i");
}

#[test]
fn union() {
    let expr = "<math>
            <mo>⋃</mo>
            <msub><mi>S</mi><mi>i</mi></msub>
            </math>";
    test("sv", "ClearSpeak", expr, "unionen av versal s nedsänkt i");
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
    test("sv", "ClearSpeak", expr, "integralen från 0, till 1, av, f av x; d x");
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
    test("sv", "ClearSpeak", expr, "integralen över de reella talen, av; f av x d x");
}

#[test]
fn integral() {
    let expr = "<math>
            <mo>∫</mo>
            <mrow><mi>f</mi><mrow><mo>(</mo><mi>x</mi> <mo>)</mo></mrow></mrow>
            <mi>d</mi><mi>x</mi>
            </math>";
    test("sv", "ClearSpeak", expr, "integralen av f av x d x");
}