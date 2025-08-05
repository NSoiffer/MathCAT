/// Tests for superscripts
///   simple superscripts
///   complex/nested superscripts
use crate::common::*;

#[test]
fn squared() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2</mn> </msup>
                </math>";
    test("en", "SimpleSpeak", expr, "x squared");
}

#[test]
fn cubed() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>3</mn> </msup>
                </math>";
    test("en", "SimpleSpeak", expr, "x cubed");
}

#[test]
    fn ordinal_power() {
        let expr = "<math>
                        <msup> <mi>x</mi> <mn>4</mn> </msup>
                    </math>";
        test("en", "SimpleSpeak", expr, "x to the fourth");
    }

#[test]
fn simple_mi_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mi>n</mi> </msup>
                </math>";
  test("en", "SimpleSpeak", expr, "x to the n-th");
}

#[test]
fn zero_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>0</mn> </msup>
                </math>";
    test("en", "SimpleSpeak", expr, "x to the 0");
}


#[test]
fn decimal_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2.0</mn> </msup>
                </math>";
    test("en", "SimpleSpeak", expr, "x to the 2.0");
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
    test("en", "SimpleSpeak", expr, "3 raised to the y plus 2 power");
}

#[test]
fn negative_power() {
    let expr = "<math>
                    <msup>
                        <mi>x</mi>
                        <mrow> <mo>-</mo> <mn>2</mn> </mrow>
                    </msup>
                </math>";
    test("en", "SimpleSpeak", expr, "x to the negative 2");
}

#[test]
fn simple_fraction_power() {
  let expr = "<math>
                  <msup>
                      <mi>x</mi> 
                      <mfrac><mn>1</mn><mn>3</mn></mfrac>
                  </msup>
              </math>";
  test("en", "SimpleSpeak", expr, "x raised to the 1 third power");
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
  test("en", "SimpleSpeak", expr, "3 raised to the 2 x squared power");
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
  test("en", "SimpleSpeak", expr, "3 raised to the negative 2 x squared power");
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
  test("en", "SimpleSpeak", expr, "y raised to the 4 fifths cubed power");
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
  test("en", "SimpleSpeak", expr, "y raised to the negative 4 fifths cubed power");
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
  test("en", "SimpleSpeak", expr, "e raised to the 1 half x squared power");
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
  test("en", "SimpleSpeak", expr, "e raised to the negative 1 half x squared power");
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
  test("en", "SimpleSpeak", expr, "3 raised to the 3 to the tenth power");
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
  test("en", "SimpleSpeak", expr, "3 raised to the open paren x plus 1, close paren squared power");
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
  test("en", "SimpleSpeak", expr, "t raised to the 4 fifths to the n-th power");
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
  test("en", "SimpleSpeak", expr, "t raised to the 4 fifths raised to the n plus 1 power; end exponent");
  test_prefs("en", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
  "t raised to the 4 fifths raised to the n plus 1 power");
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
  test("en", "SimpleSpeak", expr, "t raised to the 4 fifths to the negative 3, end exponent");
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
  test("en", "SimpleSpeak", expr, "e raised to the negative 1 half times; open paren, fraction, x minus mu, over sigma, end fraction; close paren squared power");
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
  test("en", "SimpleSpeak", expr, "t raised to the fraction, b plus 1, over 3, end fraction; power");
}
