/// Tests for superscripts
///   simple superscripts
///   complex/nested superscripts
use crate::common::*;

#[test]
fn squared() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2</mn> </msup>
                </math>";
    test("fi", "SimpleSpeak", expr, "x toiseen");
}

#[test]
fn cubed() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>3</mn> </msup>
                </math>";
    test("fi", "SimpleSpeak", expr, "x kolmanteen");
}

#[test]
    fn ordinal_power() {
        let expr = "<math>
                        <msup> <mi>x</mi> <mn>4</mn> </msup>
                    </math>";
        test("fi", "SimpleSpeak", expr, "x potenssiin 4");
    }

#[test]
fn simple_mi_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mi>n</mi> </msup>
                </math>";
  test("fi", "SimpleSpeak", expr, "x potenssiin n");
}

#[test]
fn zero_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>0</mn> </msup>
                </math>";
    test("fi", "SimpleSpeak", expr, "x potenssiin 0");
}


#[test]
fn decimal_period_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2.0</mn> </msup>
                </math>";
    test("fi", "SimpleSpeak", expr, "x potenssiin 2.0");
}

#[test]
fn decimal_comma_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2,0</mn> </msup>
                </math>";
    test("fi", "SimpleSpeak", expr, "x potenssiin 2,0");
}

#[test]
fn non_simple_power() {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <mi>y</mi><mo>+</mo><mn>2</mn></mrow>
      </msup>
      </mrow>
                </math>";
    test("fi", "SimpleSpeak", expr, "3 potenssiin y plus 2");
}

#[test]
fn negative_power() {
    let expr = "<math>
                    <msup>
                        <mi>x</mi>
                        <mrow> <mo>-</mo> <mn>2</mn> </mrow>
                    </msup>
                </math>";
    test("fi", "SimpleSpeak", expr, "x potenssiin negatiivinen 2");
}

#[test]
fn simple_fraction_power() {
  let expr = "<math>
                  <msup>
                      <mi>x</mi> 
                      <mfrac><mn>1</mn><mn>3</mn></mfrac>
                  </msup>
              </math>";
  test("fi", "SimpleSpeak", expr, "x potenssiin 1 kolmasosa");
}

#[test]
fn nested_squared_power_with_coef() {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <mn>2</mn>
        <msup>
          <mi>x</mi>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test("fi", "SimpleSpeak", expr, "3 potenssiin 2 x toiseen");
}

#[test]
fn nested_squared_power_with_neg_coef() {
    let expr = "<math>
    <mrow>
    <msup>
      <mn>3</mn>
      <mrow>
      <mo>-</mo>
      <mn>2</mn>
      <msup>
        <mi>x</mi>
        <mn>2</mn>
      </msup>
      </mrow>
    </msup>
    </mrow>
  </math>";
  test("fi", "SimpleSpeak", expr, "3 potenssiin negatiivinen 2 x toiseen");
}


#[test]
fn nested_cubed_power() {
    let expr = "<math>
      <msup>
      <mi>y</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mn>3</mn>
      </msup>
    </msup>
  </math>";
  test("fi", "SimpleSpeak", expr, "y potenssiin 4 viidesosaa kolmanteen");
}

#[test]
fn nested_cubed_power_with_neg_base() {
    let expr = "<math>
      <msup>
      <mi>y</mi> 
        <mrow>
            <mo>-</mo>
            <msup>
                <mfrac><mn>4</mn><mn>5</mn></mfrac>
                <mn>3</mn>
            </msup>
        </mrow>
    </msup>
    </math>";
  test("fi", "SimpleSpeak", expr, "y potenssiin negatiivinen 4 viidesosaa kolmanteen");
}

#[test]
fn nested_number_times_squared() {
    let expr = "<math>
        <mrow>
        <msup>
          <mi>e</mi>
          <mrow>
          <mfrac>
            <mn>1</mn>
            <mn>2</mn>
            </mfrac>
            <msup>
            <mi>x</mi>
            <mn>2</mn>
            </msup>
          </mrow>
        </msup>
        </mrow>
        </math>";
  test("fi", "SimpleSpeak", expr, "e potenssiin 1 kahdesosa x toiseen");
}

#[test]
fn nested_negative_number_times_squared() {
  let expr = "<math>
    <mrow>
    <msup>
      <mi>e</mi>
      <mrow>
      <mo>&#x2212;</mo><mfrac>
        <mn>1</mn>
        <mn>2</mn>
      </mfrac>
      <msup>
        <mi>x</mi>
        <mn>2</mn>
      </msup>
      </mrow>
    </msup>
    </mrow>
    </math>";
  test("fi", "SimpleSpeak", expr, "e potenssiin negatiivinen 1 kahdesosa, x toiseen");
}

#[test]
fn nested_expr_to_tenth() {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <msup>
          <mn>3</mn>
          <mrow>
          <mn>10</mn></mrow>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test("fi", "SimpleSpeak", expr, "3 potenssiin 3 potenssiin 10");
}

#[test]
fn nested_non_simple_squared_exp() {
    let expr = "<math>
      <mrow>
      <msup>
        <mn>3</mn>
        <mrow>
        <msup>
          <mrow>
          <mrow><mo>(</mo>
            <mrow>
            <mi>x</mi><mo>+</mo><mn>1</mn></mrow>
          <mo>)</mo></mrow></mrow>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test("fi", "SimpleSpeak", expr, "3 potenssiin auki sulku x plus 1, kiinni sulku toiseen");
}

#[test]
fn nested_simple_power() {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mi>n</mi>
      </msup>
    </msup>
  </math>";
  test("fi", "SimpleSpeak", expr, "t potenssiin 4 viidesosaa potenssiin n");
}

#[test]
fn nested_end_exponent_power() {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow>
      </msup>
    </msup>
  </math>";
  test("fi", "SimpleSpeak", expr, "t potenssiin 4 viidesosaa potenssiin n plus 1, loppu potenssi");
}

#[test]
fn nested_end_exponent_neg_power() {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <msup>
          <mfrac><mn>4</mn><mn>5</mn></mfrac>
          <mrow><mo>-</mo><mn>3</mn></mrow>
      </msup>
    </msup>
  </math>";
  test("fi", "SimpleSpeak", expr, "t potenssiin 4 viidesosaa potenssiin negatiivinen 3, loppu potenssi");
}

#[test]
fn nested_complex_power() {
    let expr = "<math>
      <mrow>
      <msup>
        <mi>e</mi>
        <mrow>
        <mo>&#x2212;</mo><mfrac>
          <mn>1</mn>
          <mn>2</mn>
        </mfrac>
        <msup>
          <mrow>
          <mrow><mo>(</mo>
            <mrow>
            <mfrac>
              <mrow>
              <mi>x</mi><mo>&#x2212;</mo><mi>&#x03BC;</mi></mrow>
              <mi>&#x03C3;</mi>
            </mfrac>
            </mrow>
          <mo>)</mo></mrow></mrow>
          <mn>2</mn>
        </msup>
        </mrow>
      </msup>
      </mrow>
      </math>";
  test("fi", "SimpleSpeak", expr, "e potenssiin negatiivinen 1 kahdesosa, kertaa; auki sulku; murtoluku, x miinus myy, per sigma, loppu murtoluku; kiinni sulku toiseen");
}

#[test]
fn default_power() {
    let expr = "<math>
      <msup>
      <mi>t</mi> 
      <mfrac>
          <mrow><mi>b</mi><mo>+</mo><mn>1</mn></mrow>
          <mn>3</mn>
      </mfrac>
    </msup>
  </math>";
  test("fi", "SimpleSpeak", expr, "t potenssiin murtoluku, b plus 1, per 3, loppu murtoluku");
}
