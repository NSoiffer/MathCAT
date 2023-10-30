/// Tests for superscripts
///   simple superscripts
///   complex/nested superscripts
use crate::common::*;

#[test]
fn squared() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2</mn> </msup>
                </math>";
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x squared");
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x to the second");
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x to the second power");
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x raised to the power 2,");

}

#[test]
fn cubed() {
  let expr = "<math>
                  <msup> <mi>x</mi> <mn>3</mn> </msup>
              </math>";
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x cubed");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x to the third");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x to the third power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x raised to the power 3,");
}

#[test]
fn ordinal_power() {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5</mn> </msup>
              </math>";
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 to the fifth power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 to the fifth");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 to the fifth power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 raised to the power 5,");
}


#[test]
fn zero_power() {
  let expr = "<math>
                    <msup> <mn>3</mn> <mn>0</mn> </msup>
                </math>";
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 to the 0 power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 to the 0");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 to the 0 power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 raised to the power 0,");
}

#[test]
fn simple_mi_power() {
  let expr = "<math>
                    <msup> <mn>4</mn> <mi>x</mi> </msup>
                </math>";
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "4 to the x-th power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "4 to the x-th");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "4 to the x-th power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "4 raised to the power x,");
}

#[test]
fn decimal_power() {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5.0</mn> </msup>
              </math>";
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 raised to the 5.0 power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 raised to the 5.0 power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 raised to the 5.0 power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 raised to the power 5.0,");
}

#[test]
fn non_simple_power() {
  let expr = "<math>
        <msup> <mn>3</mn>  <mrow> <mi>y</mi><mo>+</mo><mn>2</mn></mrow>  </msup>
    </math>";
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 raised to the y plus 2 power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 raised to the y plus 2 power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 raised to the y plus 2 power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 raised to the power y plus 2,");
}

#[test]
fn negative_power() {
  let expr = "<math>
                  <msup> <mn>3</mn> <mrow> <mo>-</mo> <mn>2</mn> </mrow> </msup>
              </math>";
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 to the negative 2 power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 to the negative 2");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 to the negative 2 power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 raised to the power negative 2,");
}

#[test]
fn simple_fraction_power() {
  let expr = "<math>
                    <msup>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </msup>
                </math>";
  test("en", "ClearSpeak", expr, "x raised to the 1 third power");
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
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 raised to the 2 x squared power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 raised to the exponent, 2 x to the second, end exponent");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 raised to the exponent, 2 x to the second power, end exponent");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 raised to the exponent, 2 x raised to the power 2; end exponent");

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
  test("en", "ClearSpeak", expr, "3 raised to the negative 2 x squared power");
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
  test("en", "ClearSpeak", expr, "y raised to the 4 fifths cubed power");
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
  test("en", "ClearSpeak", expr, "y raised to the negative 4 fifths cubed power");
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
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e raised to the 1 half x squared power");
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
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e raised to the negative 1 half x squared power");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "e raised to the exponent, negative 1 half x to the second, end exponent");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "e raised to the exponent, negative 1 half x to the second power, end exponent");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "e raised to the exponent, negative 1 half x raised to the power 2; end exponent");
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
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 raised to the exponent, 3 to the tenth power, end exponent");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 raised to the exponent, 3 to the tenth, end exponent");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 raised to the exponent, 3 to the tenth power, end exponent");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 raised to the exponent, 3 raised to the power 10; end exponent");

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
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 raised to the exponent, open paren x plus 1, close paren squared, end exponent");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 raised to the exponent, open paren x plus 1, close paren to the second, end exponent");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 raised to the exponent, open paren x plus 1, close paren to the second power, end exponent");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 raised to the exponent, open paren x plus 1, close paren raised to the power 2; end exponent");
}

#[test]
fn nested_default_power() {
  let expr = "<math>
    <msup>
    <mi>t</mi> 
    <msup>
        <mfrac><mn>4</mn><mn>5</mn></mfrac>
        <mi>n</mi>
    </msup>
  </msup>
</math>";
  test("en", "ClearSpeak", expr, "t raised to the exponent, 4 fifths to the n-th power, end exponent");
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
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr,
       "e raised to the exponent, negative 1 half times; open paren; the fraction with numerator; x minus mu; and denominator sigma; close paren squared, end exponent");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr,
       "e raised to the exponent, negative 1 half times; open paren; the fraction with numerator; x minus mu; and denominator sigma; close paren to the second, end exponent");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr,
       "e raised to the exponent, negative 1 half times; open paren; the fraction with numerator; x minus mu; and denominator sigma; close paren to the second power, end exponent");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr,
       "e raised to the exponent, negative 1 half times; open paren; the fraction with numerator; x minus mu; and denominator sigma; close paren raised to the power 2; end exponent");
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
  test("en", "ClearSpeak", expr, "t raised to the fraction with numerator; b plus 1; and denominator 3; power");
}
