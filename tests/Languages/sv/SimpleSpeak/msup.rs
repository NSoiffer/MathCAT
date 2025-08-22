/// Tests for superscripts
///   simple superscripts
///   complex/nested superscripts
use crate::common::*;

#[test]
fn squared() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2</mn> </msup>
                </math>";
    test("sv", "SimpleSpeak", expr, "x kvadrat");
}

#[test]
fn cubed() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>3</mn> </msup>
                </math>";
    test("sv", "SimpleSpeak", expr, "x kubik");
}

#[test]
    fn ordinal_power() {
        let expr = "<math>
                        <msup> <mi>x</mi> <mn>4</mn> </msup>
                    </math>";
        test("sv", "SimpleSpeak", expr, "x upphöjt till 4");
    }

#[test]
fn simple_mi_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mi>n</mi> </msup>
                </math>";
  test("sv", "SimpleSpeak", expr, "x upphöjt till n");
}

#[test]
fn zero_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>0</mn> </msup>
                </math>";
    test("sv", "SimpleSpeak", expr, "x upphöjt till 0");
}


#[test]
fn decimal_power() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2,0</mn> </msup>
                </math>";
    test("sv", "SimpleSpeak", expr, "x upphöjt till 2,0");
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
    test("sv", "SimpleSpeak", expr, "3 upphöjt till y plus 2");
}

#[test]
fn negative_power() {
    let expr = "<math>
                    <msup>
                        <mi>x</mi>
                        <mrow> <mo>-</mo> <mn>2</mn> </mrow>
                    </msup>
                </math>";
    test("sv", "SimpleSpeak", expr, "x upphöjt till minus 2");
}

#[test]
fn simple_fraction_power() {
  let expr = "<math>
                  <msup>
                      <mi>x</mi> 
                      <mfrac><mn>1</mn><mn>3</mn></mfrac>
                  </msup>
              </math>";
  test("sv", "SimpleSpeak", expr, "x upphöjt till 1 tredjedel");
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
  test("sv", "SimpleSpeak", expr, "3 upphöjt till 2 x kvadrat");
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
  test("sv", "SimpleSpeak", expr, "3 upphöjt till minus 2 x kvadrat");
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
  test("sv", "SimpleSpeak", expr, "y upphöjt till 4 femtedelar kubik");
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
  test("sv", "SimpleSpeak", expr, "y upphöjt till minus 4 femtedelar kubik");
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
  test("sv", "SimpleSpeak", expr, "e upphöjt till 1 halv x kvadrat");
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
  test("sv", "SimpleSpeak", expr, "e upphöjt till minus 1 halv x kvadrat");
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
  test("sv", "SimpleSpeak", expr, "3 upphöjt till 3 upphöjt till 10");
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
  test("sv", "SimpleSpeak", expr, "3 upphöjt till vänster-parentes; x plus 1; höger-parentes kvadrat");
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
  test("sv", "SimpleSpeak", expr, "t upphöjt till 4 femtedelar upphöjt till n");
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
  test("sv", "SimpleSpeak", expr, "t upphöjt till 4 femtedelar upphöjt till n plus 1, slut exponent");
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
  test("sv", "SimpleSpeak", expr, "t upphöjt till 4 femtedelar upphöjt till minus 3; slut exponent");
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
  test("sv", "SimpleSpeak", expr, "e upphöjt till minus 1 halv gånger; vänster-parentes; division, x minus my, genom sigma, slut division; höger-parentes kvadrat");
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
  test("sv", "SimpleSpeak", expr, "t upphöjt till division, b plus 1, genom 3, slut division");
}
