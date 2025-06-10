/// Tests for superscripts
///   simple superscripts
///   complex/nested superscripts
use crate::common::*;

#[test]
fn squared() {
    let expr = "<math>
                    <msup> <mi>x</mi> <mn>2</mn> </msup>
                </math>";
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x i andre");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x opphøyd i 2");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x opphøyd i 2");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x opphøyd i 2");

}

#[test]
fn cubed() {
  let expr = "<math>
                  <msup> <mi>x</mi> <mn>3</mn> </msup>
              </math>";
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "x i tredje");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "x opphøyd i 3");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "x opphøyd i 3");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "x opphøyd i 3");
}

#[test]
fn ordinal_power() {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5</mn> </msup>
              </math>";
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 opphøyd i 5");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 opphøyd i 5");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 opphøyd i 5");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 opphøyd i 5");
}


#[test]
fn zero_power() {
  let expr = "<math>
                    <msup> <mn>3</mn> <mn>0</mn> </msup>
                </math>";
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 opphøyd i 0");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 opphøyd i 0");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 opphøyd i 0");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 opphøyd i 0");
}

#[test]
fn simple_mi_power() {
  let expr = "<math>
                    <msup> <mn>4</mn> <mi>x</mi> </msup>
                </math>";
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "4 opphøyd i x");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "4 opphøyd i x");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "4 opphøyd i x");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "4 opphøyd i x");
}

#[test]
fn decimal_power() {
  let expr = "<math>
                  <msup> <mn>3</mn> <mn>5,0</mn> </msup>
              </math>";
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 opphøyd i 5,0");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 opphøyd i 5,0");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 opphøyd i 5,0");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 opphøyd i 5,0");
}

#[test]
fn non_simple_power() {
  let expr = "<math>
        <msup> <mn>3</mn>  <mrow> <mi>y</mi><mo>+</mo><mn>2</mn></mrow>  </msup>
    </math>";
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 opphøyd i y pluss 2");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 opphøyd i y pluss 2");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 opphøyd i y pluss 2");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 opphøyd i y pluss 2");
}

#[test]
fn negative_power() {
  let expr = "<math>
                  <msup> <mn>3</mn> <mrow> <mo>-</mo> <mn>2</mn> </mrow> </msup>
              </math>";
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 opphøyd i minus 2");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 opphøyd i minus 2");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 opphøyd i minus 2");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 opphøyd i minus 2");
}

#[test]
fn simple_fraction_power() {
  let expr = "<math>
                    <msup>
                        <mi>x</mi> 
                        <mfrac><mn>1</mn><mn>3</mn></mfrac>
                    </msup>
                </math>";
  test("nb", "ClearSpeak", expr, "x opphøyd i 1 tredjedel");
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
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 opphøyd i 2 x i andre");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 opphøyd i eksponenten, 2 x opphøyd i 2; slutt eksponent");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 opphøyd i eksponenten, 2 x opphøyd i 2; slutt eksponent");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 opphøyd i eksponenten, 2 x opphøyd i 2; slutt eksponent");

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
  test("nb", "ClearSpeak", expr, "3 opphøyd i minus 2 x i andre");
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
  test("nb", "ClearSpeak", expr, "y opphøyd i 4 femtedeler i tredje");
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
  test("nb", "ClearSpeak", expr, "y opphøyd i minus 4 femtedeler i tredje");
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
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e opphøyd i 1 halv x i andre");
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
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "e opphøyd i minus 1 halv x i andre");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "e opphøyd i eksponenten, minus 1 halv x opphøyd i 2; slutt eksponent");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "e opphøyd i eksponenten, minus 1 halv x opphøyd i 2; slutt eksponent");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "e opphøyd i eksponenten, minus 1 halv x opphøyd i 2; slutt eksponent");
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
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 opphøyd i eksponenten, 3 opphøyd i 10; slutt eksponent");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 opphøyd i eksponenten, 3 opphøyd i 10; slutt eksponent");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 opphøyd i eksponenten, 3 opphøyd i 10; slutt eksponent");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 opphøyd i eksponenten, 3 opphøyd i 10; slutt eksponent");

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
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr, "3 opphøyd i eksponenten; startparentes; x pluss 1; sluttparentes i andre, slutt eksponent");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr, "3 opphøyd i eksponenten; startparentes; x pluss 1; sluttparentes opphøyd i 2; slutt eksponent");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr, "3 opphøyd i eksponenten; startparentes; x pluss 1; sluttparentes opphøyd i 2; slutt eksponent");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr, "3 opphøyd i eksponenten; startparentes; x pluss 1; sluttparentes opphøyd i 2; slutt eksponent");
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
  test("nb", "ClearSpeak", expr, "t opphøyd i eksponenten, 4 femtedeler opphøyd i n; slutt eksponent");
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
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Auto")], expr,
       "e opphøyd i eksponenten, minus 1 halv ganger; startparentes; brøken med teller; x minus my; og nevner sigma; sluttparentes i andre, slutt eksponent");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "Ordinal")], expr,
       "e opphøyd i eksponenten, minus 1 halv ganger; startparentes; brøken med teller; x minus my; og nevner sigma; sluttparentes opphøyd i 2; slutt eksponent");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "OrdinalPower")], expr,
       "e opphøyd i eksponenten, minus 1 halv ganger; startparentes; brøken med teller; x minus my; og nevner sigma; sluttparentes opphøyd i 2; slutt eksponent");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Exponents", "AfterPower")], expr,
       "e opphøyd i eksponenten, minus 1 halv ganger; startparentes; brøken med teller; x minus my; og nevner sigma; sluttparentes opphøyd i 2; slutt eksponent");
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
  test("nb", "ClearSpeak", expr, "t opphøyd i brøken med teller; b pluss 1; og nevner 3");
}
