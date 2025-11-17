/// Tests for fractions
///   includes simple fractions and more complex fractions
///   also tests mixed fractions (implicit and explicit)
use crate::common::*;

#[test]
fn common_fraction_half() {
    let expr = "<math>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("en", "SimpleSpeak", expr, "1 half");
}

#[test]
fn common_fraction_thirds() {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("en", "SimpleSpeak", expr, "2 thirds");
}

#[test]
fn common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test("en", "SimpleSpeak", expr, "17 tenths");
}

#[test]
#[allow(non_snake_case)]
fn not_SimpleSpeak_common_fraction_tenths() {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test("en", "SimpleSpeak", expr, "89 over 10");
}

#[test]
fn non_simple_fraction() {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi><mo>+</mo><mi>y</mi> </mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("en", "SimpleSpeak", expr, "fraction, x plus y, over, x minus y, end fraction");
}

#[test]
fn nested_fraction() {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi><mo>+</mo>  <mfrac><mn>1</mn><mi>y</mi></mfrac>  </mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("en", "SimpleSpeak", expr, "fraction, x plus, fraction, 1 over y, end fraction; over, x minus y, end fraction");
}


#[test]
fn deeply_nested_fraction_msqrt() {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi><mo>+</mo>  <msqrt><mrow><mfrac><mn>1</mn><mi>y</mi></mfrac></mrow> </msqrt> </mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("en", "SimpleSpeak", expr, "fraction, x plus, the square root of 1 over y; end root; over, x minus y, end fraction");
}

#[test]
fn deeply_nested_fraction_mrow_msqrt() {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi><mo>+</mo>  <msqrt><mrow><mn>2</mn><mo>+</mo><mfrac><mn>1</mn><mi>y</mi></mfrac></mrow> </msqrt> </mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("en", "SimpleSpeak", expr, "fraction, x plus, the square root of 2 plus 1 over y; end root; over, x minus y, end fraction");
}

#[test]
fn numerator_simple_fraction() {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow> <mi>x</mi></mrow>
        <mrow>
            <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>
                            ";
    test("en", "SimpleSpeak", expr, "fraction, x over, x minus y, end fraction");
}

#[test]
fn denominator_simple_fraction() {
    let expr = "
    <math>
        <mfrac>
            <mrow> <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
            <mrow> <mi>x</mi></mrow>
        </mfrac>
    </math>
                            ";
    test("en", "SimpleSpeak", expr, "fraction, x minus y, over x, end fraction");
}


#[test]
fn frac_with_units() {
    let expr = "
    <math>
        <mrow>
        <mn>62</mn>
        <mfrac>
        <mi intent=':unit'>mi</mi>
        <mi intent=':unit'>hr</mi>
        </mfrac>
        </mrow>
    </math>";
    test("en", "SimpleSpeak", expr, "62 miles per hour");
}

#[test]
fn singular_frac_with_units() {
    let expr = "
    <math>
        <mrow>
        <mn>1</mn>
        <mfrac>
        <mi intent=':unit'>gal</mi>
        <mi intent=':unit'>mi</mi>
        </mfrac>
        </mrow>
    </math>";
    test("en", "SimpleSpeak", expr, "1 gallon per mile");
}

#[test]
fn number_in_numerator_with_units() {
    let expr = "
    <math>
        <mfrac>
            <mrow>
                <mn>3</mn>
                <mi intent=':unit'>gal</mi>
            </mrow>
            <mi intent=':unit'>mi</mi>
        </mfrac>
    </math>";
    test("en", "SimpleSpeak", expr, "3 gallons per mile");
}

#[test]
fn units_with_powers() {
    let expr = "
    <math>
        <mfrac>
            <mrow> <mn>3</mn> <mi intent=':unit'>m</mi> </mrow>
            <msup> <mi intent=':unit'>s</mi><mn>2</mn> </msup>
        </mfrac>
    </math>";
    test("en", "SimpleSpeak", expr, "3 metres per second squared");
}


#[test]
fn mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("en", "SimpleSpeak", expr, "3 and 1 half");
}

#[test]
fn explicit_mixed_number() {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("en", "SimpleSpeak", expr, "3 and 1 eighth");
}

#[test]
fn mixed_number_big() {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("en", "SimpleSpeak", expr, "3 and 7 eighty thirds");
}

#[test]
fn simple_text() {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("en", "SimpleSpeak", expr, "rise over run");
}

#[test]
fn number_and_text() {
    let expr = "<math>
            <mfrac>
            <mrow>
                <mn>2</mn><mtext>miles</mtext></mrow>
            <mrow>
                <mn>3</mn><mtext>gallons</mtext></mrow>
            </mfrac>
        </math>";
    test("en", "SimpleSpeak", expr, "fraction, 2 miles, over, 3 gallons, end fraction");
}


#[test]
fn nested_simple_fractions() {
    let expr = "<math>
                <mrow>
                <mfrac>
                    <mrow>
                    <mfrac>
                        <mn>1</mn>
                        <mn>2</mn>
                    </mfrac>
                    </mrow>
                    <mrow>
                    <mfrac>
                        <mn>2</mn>
                        <mn>3</mn>
                    </mfrac>
                    </mrow>
                </mfrac>
                </mrow>
            </math>";
    test("en", "SimpleSpeak", expr, "fraction, 1 half, over, 2 thirds, end fraction");
}

#[test]
fn binomial() {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("en", "SimpleSpeak", expr, "2 times 7 choose 3");
}

#[test]
fn binomial_non_simple_top() {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mrow><mi>n</mi><mo>+</mo><mn>7</mn></mrow> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("en", "SimpleSpeak", expr, "2 times, binomial n plus 7 choose 3");
}

#[test]
fn binomial_non_simple_bottom() {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mrow><mi>k</mi><mo>+</mo><mn>3</mn></mrow> </mfrac>
                    <mo>)</mo>
                </math>";
    test("en", "SimpleSpeak", expr, "2 times, 7 choose k plus 3 end binomial");
}

#[test]
fn binomial_non_simple_top_and_bottom() {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mrow><mi>n</mi><mo>+</mo><mn>7</mn></mrow> <mrow><mi>k</mi><mo>+</mo><mn>3</mn></mrow> </mfrac>
                    <mo>)</mo>
                </math>";
    test("en", "SimpleSpeak", expr, "2 times, binomial n plus 7 choose k plus 3 end binomial");
}
