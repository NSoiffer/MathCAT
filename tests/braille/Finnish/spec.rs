// The tests are based on the Finnish specification for 6 dot math braille on the braille authority's web page (https://www.pistekirjoitus.fi/julkaisut/matematiikka-ja-tietotekniikka/) titled "Matematiikan, fysiikan ja kemain pistemerkinnät". Roughly translates to "Braille for mathematics, physics and chemistry." These tests are based on the edition published in 2022.
//
// Changes to the specifications in the rules and tests in MathCAT
// Some changes have been made to the rules and these tests test against the modified rules. The changes have been made, because the specification is for printed braille and intended for people authoring mathematics braille. Some things have been changed to be consistent in all situations and to work in the braille display context.
use crate::common::*;
use anyhow::Result;

// Finnish spec tests
// Page and some other identification has been added to the function name, so you can figure out which expression the test in based on. Some tests are variations of the "official" test.

// In general numbers and variables are separated from each other with a space, so the variables (most often letters) are not interpreted as numbers.
// Variables can be written with no space between them, since there is not similar misinterpretation risk. 

// Grouping numbers
// No example in the specs
#[test]
fn p7_no_grouping_in_four_digit_numbers() -> Result<()> {
    let expr = "<math><mn>2000</mn></math>";
    test_braille("Finnish", expr, "⠼⠃⠚⠚⠚")?;
    return Ok(());
}

#[test]
fn p7_thousands_nbsp() -> Result<()> {
    let expr = "<math><mn>2 000 000</mn></math>";
    test_braille("Finnish", expr, "⠼⠃⠄⠚⠚⠚⠄⠚⠚⠚")?;
    return Ok(());
}

#[test]
fn p7_thousands_point() -> Result<()> {
    let expr = "<math><mn>2.000.000</mn></math>";
    test_braille("Finnish", expr, "⠼⠃⠄⠚⠚⠚⠄⠚⠚⠚")?;
    return Ok(());
}

#[test]
fn p7_thousands_space() -> Result<()> {
    let expr = "<math><mn>2 000 000</mn></math>";
    test_braille("Finnish", expr, "⠼⠃⠄⠚⠚⠚⠄⠚⠚⠚")?;
    return Ok(());
}

#[test]
fn p7_decimal_comma() -> Result<()> {
    let expr = "<math><mn>5,12575</mn></math>";
    test_braille("Finnish", expr, "⠼⠑⠂⠁⠃⠑⠄⠛⠑")?;
    return Ok(());
}


// When there is a decimal point in the text then the grouping character is the number character.
#[test]
fn p7_decimal_point() -> Result<()> {
    let expr = "<math><mn>5.12575</mn></math>";
    test_braille("Finnish", expr, "⠼⠑⠄⠁⠃⠑⠛⠑")?;
    return Ok(());
}

#[test]
fn p7_decimal_point_endless_no_grouping() -> Result<()> {
    let expr = "<math><mn>1,234657234</mn><mi>...</mi></math>";
    test_braille("Finnish", expr, "⠼⠁⠂⠃⠉⠙⠄⠋⠑⠛⠄⠃⠉⠙⠄⠄⠄")?;
    return Ok(());
}


#[test]
fn p7_decimal_point_endless_with_grouping_space() -> Result<()> {
    let expr = "<math><mn>1,234 657 234</mn><mi>...</mi></math>";
    test_braille("Finnish", expr, "⠼⠁⠂⠃⠉⠙⠄⠋⠑⠛⠄⠃⠉⠙⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn p7_decimal_point_endless_with_grouping_nbsp() -> Result<()> {
    let expr = "<math><mn>1,234 657 234</mn><mi>...</mi></math>";
    test_braille("Finnish", expr, "⠼⠁⠂⠃⠉⠙⠄⠋⠑⠛⠄⠃⠉⠙⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn p7_decimal_point_endless_with_grouping_nbsp_ellipses() -> Result<()> {
    let expr = "<math><mn>1,234 657 234</mn><mi>…</mi></math>";
    test_braille("Finnish", expr, "⠼⠁⠂⠃⠉⠙⠄⠋⠑⠛⠄⠃⠉⠙⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn p7_decimal_point_endless_with_grouping_nbsp_ellipses_mathtype() -> Result<()> {
    let expr = "<math><mn>1</mn><mo>,</mo><mn>234</mn><mo>&#xA0;</mo><mn>657</mn><mo>&#xA0;</mo><mn>234</mn><mo>&#x2026;</mo></math>";
    test_braille("Finnish", expr, "⠼⠁⠂⠃⠉⠙⠄⠋⠑⠛⠄⠃⠉⠙⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn p7_decimal_point_endless_no_grouping_ellipses() -> Result<()> {
    let expr = "<math><mn>1,234657234</mn><mi>…</mi></math>";
    test_braille("Finnish", expr, "⠼⠁⠂⠃⠉⠙⠄⠋⠑⠛⠄⠃⠉⠙⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn p7_decimal_point_endless_with_grouping_space_ellipses() -> Result<()> {
    let expr = "<math><mn>1,234 657 234…</mn></math>";
    test_braille("Finnish", expr, "⠼⠁⠂⠃⠉⠙⠄⠋⠑⠛⠄⠃⠉⠙⠄⠄⠄")?;
    return Ok(());
}

#[test]
fn p9_units_percent() -> Result<()> {
    let expr = "<math><mn>99</mn><mi>%</mi></math>";
    test_braille("Finnish", expr, "⠼⠊⠊⠀⠹")?;
    return Ok(());
}

#[test]
fn p9_units_permille() -> Result<()> {
    let expr = "<math><mn>115</mn><mi>‰</mi></math>";
    test_braille("Finnish", expr, "⠼⠁⠁⠑⠀⠒⠹")?;
    return Ok(());
}

#[test]
fn p9_units_degrees() -> Result<()> {
    let expr = "<math><mn>100</mn><mi>˚</mi></math>";
    test_braille("Finnish", expr, "⠼⠁⠚⠚⠀⠴")?;
    return Ok(());
}

#[test]
fn p9_units_degrees_celsius() -> Result<()> {
    let expr = "<math><mn>37</mn><mi>˚</mi><mi>C</mi></math>";
    test_braille("Finnish", expr, "⠼⠉⠛⠀⠴⠠⠉")?;
    return Ok(());
}

#[test]
fn p9_units_degrees_fahrenheit() -> Result<()> {
    let expr = "<math><mn>−43</mn><mi>˚</mi><mi>F</mi></math>";
    test_braille("Finnish", expr, "⠤⠼⠙⠉⠀⠴⠠⠋")?;
    return Ok(());
}

#[test]
fn p10_currency_euro() -> Result<()> {
    let expr = "<math><mn>6</mn><mi>€</mi><mn>15</mn><mi>snt</mi></math>";
    test_braille("Finnish", expr, "⠼⠋⠀⠈⠑⠀⠼⠁⠑⠀⠎⠝⠞")?;
    return Ok(());
}

#[test]
fn p10_currency_dollar() -> Result<()> {
    let expr = "<math><mi>$</mi><mn>35</mn></math>";
    test_braille("Finnish", expr, "⠮⠼⠉⠑")?;
    return Ok(());
}

#[test]
fn p10_currency_pound() -> Result<()> {
    let expr = "<math><mi>£</mi><mn>25</mn></math>";
    test_braille("Finnish", expr, "⠇⠼⠃⠑")?;
    return Ok(());
}

#[test]
fn p11_mm_squared() -> Result<()> {
    let expr = "<math><msup><mi>mm</mi><mn>2</mn></msup></math>";
    test_braille("Finnish", expr, "⠍⠍⠬")?;
    return Ok(());
}

#[test]
fn p11_cm_cubed() -> Result<()> {
    let expr = "<math><msup><mi>cm</mi><mn>3</mn></msup></math>";
    test_braille("Finnish", expr, "⠉⠍⠬⠼⠉")?;
    return Ok(());
}

#[test]
fn p13_plus() -> Result<()> {
    let expr = "<math><mn>3</mn><mo>+</mo><mn>4</mn><mo>=</mo><mn>7</mn></math>";
    test_braille("Finnish", expr, "⠼⠉⠀⠖⠼⠙⠀⠶⠼⠛")?;
    return Ok(());
}

#[test]
fn p13_minus_not_equal() -> Result<()> {
    let expr = "<math><mn>5</mn><mo>−</mo><mn>2</mn><mo>≠</mo><mn>2</mn></math>";
    test_braille("Finnish", expr, "⠼⠑⠀⠤⠼⠃⠀⠐⠶⠼⠃")?;
    return Ok(());
}

// Question mark is the same as in literary text.
#[test]
fn p13_times_with_question_mark() -> Result<()> {
    let expr = "<math><mn>27</mn><mo>·</mo><mn>3</mn><mo>=</mo><mi>?</mi></math>";
    test_braille("Finnish", expr, "⠼⠃⠛⠀⠄⠼⠉⠀⠶⠢")?;
    return Ok(());
}

#[test]
fn p13_parentheses_invisible_times() -> Result<()> {
    let expr = "<math><mo>(</mo><mn>3</mn><mo>&#8290;</mo><mi>x</mi><mo>+</mo><mn>2</mn><mo>&#8290;</mo><mi>y</mi><mo>)</mo>
                    <mo>&#8290;</mo><mo>(</mo><mn>2</mn><mo>&#8290;</mo><mi>x</mi><mo>−</mo><mn>1</mn><mo>)</mo></math>";
    test_braille("Finnish", expr, "⠦⠼⠉⠀⠭⠀⠖⠼⠃⠀⠽⠴⠀⠦⠼⠃⠀⠭⠀⠤⠼⠁⠴")?;
    return Ok(());
}

#[test]
fn p15_less_than() -> Result<()> {
    let expr = "<math>
    <mi>x</mi>
    <mo>&lt;</mo>
    <mn>18</mn>
</math>";
    test_braille("Finnish", expr, "⠭⠀⠣⠀⠼⠁⠓")?;
    return Ok(());
}

#[test]
fn p15_greater_than() -> Result<()> {
    let expr = "<math>
    <mn>2</mn>
    <mi>x</mi>
    <mo>&gt;</mo>
    <mn>6</mn>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠀⠭⠀⠱⠀⠼⠋")?;
    return Ok(());
}

// No space required between variables.
#[test]
fn greater_than_with_more_variables() -> Result<()> {
    let expr = "<math>
    <mn>2</mn>
    <mi>x</mi>
    <mi>y</mi>
    <mo>&gt;</mo>
    <mn>6</mn>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠀⠭⠽⠀⠱⠀⠼⠋")?;
    return Ok(());
}

#[test]
fn p15_greater_than_or_equal_to() -> Result<()> {
    let expr = "<math>
    <mn>2</mn>
    <mi>x</mi>
    <mo>≥</mo>
    <mn>6</mn>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠀⠭⠀⠱⠶⠀⠼⠋")?;
    return Ok(());
}

#[test]
fn p15_less_than_or_equal_to() -> Result<()> {
    let expr = "<math>
    <mi>x</mi>
    <mo>≤</mo>
    <mn>18</mn>
</math>";
    test_braille("Finnish", expr, "⠭⠀⠣⠶⠀⠼⠁⠓")?;
    return Ok(());
}

// Fractions with 'dropped dots'.
// When the fraction ends in a parenthesis (dots 356), the dot 6 is used to end the 'dropped dots' zone, so it is not confused with 'dropped dots' for number 0 (dots 356). 
#[test]
fn p15_fractions_minus() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mo>(</mo>
    <mn>1</mn>
    <mo>&#8292;</mo>
    <mfrac>
      <mn>3</mn>
      <mn>5</mn>
    </mfrac>
    <mo>−</mo>
    <mfrac>
      <mn>5</mn>
      <mn>7</mn>
    </mfrac>
    <mo>)</mo>
    <mo>−</mo>
    <mo>(</mo>
    <mfrac>
      <mn>2</mn>
      <mn>5</mn>
    </mfrac>
    <mo>+</mo>
    <mfrac>
      <mn>1</mn>
      <mn>3</mn>
    </mfrac>
    <mo>)</mo>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠦⠼⠁⠼⠉⠢⠀⠤⠼⠑⠶⠠⠴⠀⠤⠦⠼⠃⠢⠀⠖⠼⠁⠒⠠⠴")?;
    return Ok(());
}

#[test]
fn p16_division() -> Result<()> {
    let expr = "<math>
    <mo>(</mo>
    <mn>6</mn>
    <mi>x</mi>
    <mo>+</mo>
    <mn>3</mn>
    <mi>x</mi>
    <mo>)</mo>
    <mo>:</mo>
    <mn>3</mn>
</math>";
    test_braille("Finnish", expr, "⠦⠼⠋⠀⠭⠀⠖⠼⠉⠀⠭⠴⠀⠒⠼⠉")?;
    return Ok(());
}

#[test]
fn p16_fractional() -> Result<()> {
    let expr = "<math>
  <mfrac>
    <mrow>
      <mn>6</mn>
      <mi>x</mi>
      <mo>+</mo>
      <mn>3</mn>
      <mi>x</mi>
    </mrow>
    <mn>3</mn>
  </mfrac>
</math>";
    test_braille("Finnish", expr, "⠦⠼⠋⠀⠭⠀⠖⠼⠉⠀⠭⠴⠀⠌⠼⠉")?;
    return Ok(());
}

#[test]
fn p16_absolute_value() -> Result<()> {
    let expr = "<math>
      <mrow>
        <mrow>
          <mo>|</mo>
          <mrow>
            <mo>-</mo>
            <mrow><mo>(</mo><mrow><mn>2</mn><mo>+</mo><mn>5</mn></mrow><mo>)</mo></mrow>
          </mrow>
          <mo>|</mo>
        </mrow>
        <mo>=</mo>
        <mrow><mo>|</mo><mrow><mo>-</mo><mn>7</mn></mrow><mo>|</mo></mrow>
        <mo>=</mo>
        <mn>7</mn>
      </mrow>
    </math>";
    test_braille("Finnish", expr, "⠸⠤⠦⠼⠃⠀⠖⠼⠑⠴⠸⠀⠶⠸⠤⠼⠛⠸⠀⠶⠼⠛")?;
    return Ok(());
}

#[test]
fn p16_parens_and_brackets() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mn>92</mn>
    <mo>+</mo>
    <mo>[</mo>
    <mo>−</mo>
    <mn>2</mn>
    <mo>⋅</mo>
    <mo>(</mo>
    <mn>18</mn>
    <mo>+</mo>
    <mn>5</mn>
    <mo>)</mo>
    <mo>]</mo>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠊⠃⠀⠖⠷⠤⠼⠃⠀⠄⠦⠼⠁⠓⠀⠖⠼⠑⠴⠾")?;
    return Ok(());
}

#[test]
fn p16_sequence_of_natural_numbers() -> Result<()> {
    let expr = "<math>
    <mi>N</mi>
    <mo>=</mo>
    <mo>{</mo>
    <mn>0</mn>
    <mo>,</mo>
    <mn>1</mn>
    <mo>,</mo>
    <mn>2</mn>
    <mo>,</mo>
    <mn>3</mn>
    <mo>,</mo>
    <mi>...</mi>
    <mo>}</mo>
</math>";
    test_braille("Finnish", expr, "⠠⠝⠀⠶⠫⠼⠚⠂⠀⠼⠁⠂⠀⠼⠃⠂⠀⠼⠉⠂⠀⠄⠄⠄⠻")?;
    return Ok(());
}

#[test]
fn p17_multiple_operations() -> Result<()> {
    let expr = "<math>
    <mn>5</mn>
    <mo>⋅</mo>
    <msup>
      <mn>3</mn>
      <mn>2</mn>
    </msup>
    <mo>+</mo>
    <mo>(</mo>
    <mn>81</mn>
    <mo>−</mo>
    <mn>60</mn>
    <mo>)</mo>
    <mo>:</mo>
    <mn>3</mn>
    <mo>−</mo>
    <mo>(</mo>
    <mn>2</mn>
    <mo>+</mo>
    <mn>4</mn>
    <msup>
      <mo>)</mo>
      <mn>2</mn>
    </msup>
</math>";
    test_braille("Finnish", expr, "⠼⠑⠀⠄⠼⠉⠬⠀⠖⠦⠼⠓⠁⠀⠤⠼⠋⠚⠴⠀⠒⠼⠉⠀⠤⠦⠼⠃⠀⠖⠼⠙⠴⠬")?;
    return Ok(());
}

#[test]
fn p19_fractions_sum() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mfrac>
      <mn>3</mn>
      <mn>4</mn>
    </mfrac>
    <mo>+</mo>
    <mfrac>
      <mn>1</mn>
      <mn>4</mn>
    </mfrac>
    <mo>=</mo>
    <mn>1</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠉⠲⠀⠖⠼⠁⠲⠀⠶⠼⠁")?;
    return Ok(());
}

#[test]
fn p19_fractions_mixed_operations() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mo>(</mo>
    <mfrac>
      <mn>3</mn>
      <mn>4</mn>
    </mfrac>
    <mo>+</mo>
    <mfrac>
      <mn>5</mn>
      <mn>6</mn>
    </mfrac>
    <mo>)</mo>
    <mo>−</mo>
    <mo>(</mo>
    <mfrac>
      <mn>3</mn>
      <mn>5</mn>
    </mfrac>
    <mo>+</mo>
    <mfrac>
      <mn>1</mn>
      <mn>2</mn>
    </mfrac>
    <mo>)</mo>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠦⠼⠉⠲⠀⠖⠼⠑⠖⠠⠴⠀⠤⠦⠼⠉⠢⠀⠖⠼⠁⠆⠠⠴")?;
    return Ok(());
}

// This is Finnish notation where the numerator and denomator is multiplied by the same expression that is in the pre-supercript. Structure is always: [multiplicator][parenthesis]
#[test]
fn p19_fraction_multiplication_Finnish_notation_multiscripts() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mmultiscripts>
      <mfrac>
        <mi>a</mi>
        <mi>c</mi>
      </mfrac>
      <mprescripts></mprescripts>
      <none></none>
      <mrow>
        <mi>b</mi>
        <mo>)</mo>
      </mrow>
    </mmultiscripts>
    <mo>=</mo>
    <mfrac>
      <mrow>
        <mi>a</mi>
        <mi>b</mi>
      </mrow>
      <mrow>
        <mi>b</mi>
        <mi>c</mi>
      </mrow>
    </mfrac>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠃⠜⠀⠁⠀⠌⠉⠀⠶⠁⠃⠀⠌⠃⠉")?;
    return Ok(());
}

// This is Finnish notation where the numerator and denomator is multiplied by the same expression that is in the pre-supercript. Structure is always: [multiplicator][parenthesis]
#[test]
fn p19_fraction_multiplication_Finnish_notation_multiscripts_2() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mmultiscripts>
      <mfrac>
        <mn>1</mn>
        <mn>2</mn>
      </mfrac>
      <mprescripts></mprescripts>
      <none></none>
      <mrow>
        <mn>6</mn>
        <mo>)</mo>
      </mrow>
    </mmultiscripts>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠋⠜⠼⠁⠆")?;
    return Ok(());
}

// This is Finnish notation for dividing the numerator and denominator by the same expression. It is always of the form: [parenthesis][divisor]
#[test]
fn p22_fraction_division_Finnish_notation_msup() -> Result<()> {
    let expr = "<math>
  <mrow>
    <msup>
      <mfrac>
        <mrow>
          <mi>a</mi>
          <mi>b</mi>
          <mi>c</mi>
        </mrow>
        <mrow>
          <mi>a</mi>
          <mi>d</mi>
        </mrow>
      </mfrac>
      <mrow>
        <mo>(</mo>
        <mi>a</mi>
      </mrow>
    </msup>
    <mo>=</mo>
    <mfrac>
      <mrow>
        <mi>b</mi>
        <mi>c</mi>
      </mrow>
      <mi>d</mi>
    </mfrac>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠁⠃⠉⠀⠌⠁⠙⠀⠣⠁⠀⠶⠃⠉⠀⠌⠙")?;
    return Ok(());
}

// This is Finnish notation for dividing the numerator and denominator by the same expression. It is always of the form: [parenthesis][divisor]
#[test]
fn p22_fraction_division_Finnish_notation_msup_2() -> Result<()> {
    let expr = "<math>
  <msup>
    <mfrac>
      <mn>2</mn>
      <mn>4</mn>
    </mfrac>
    <mrow>
      <mo>(</mo>
      <mn>2</mn>
    </mrow>
  </msup>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠲⠣⠼⠃")?;
    return Ok(());
}

// It could possible to have the same structure as a mmultiscript as before, so here is a test for that. Output is the same as before.
#[test]
fn p22_fraction_division_Finnish_notation_mmultiscripts() -> Result<()> {
    let expr = "<math>
  <mmultiscripts>
    <mfrac>
      <mn>2</mn>
      <mn>4</mn>
    </mfrac>
    <mrow></mrow>
    <mrow>
      <mo>(</mo>
      <mn>2</mn>
    </mrow>
  </mmultiscripts>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠲⠣⠼⠃")?;
    return Ok(());
}

#[test]
fn p23_equation_1_no_invisible_multiplication() -> Result<()> {
    let expr = "<math>
    <mn>2</mn>
    <mi>x</mi>
    <mo>+</mo>
    <mn>3</mn>
    <mo>=</mo>
    <mo>−</mo>
    <mn>3</mn>
    <mi>x</mi>
    <mo>−</mo>
    <mn>7</mn>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠀⠭⠀⠖⠼⠉⠀⠶⠤⠼⠉⠀⠭⠀⠤⠼⠛")?;
    return Ok(());
}

#[test]
fn p23_equation_1_with_invisible_multiplication() -> Result<()> {
    let expr = "<math>
    <mn>2</mn>
    <mo>&#8290;</mo>
    <mi>x</mi>
    <mo>+</mo>
    <mn>3</mn>
    <mo>=</mo>
    <mo>−</mo>
    <mn>3</mn>
    <mo>&#8290;</mo>
    <mi>x</mi>
    <mo>−</mo>
    <mn>7</mn>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠀⠭⠀⠖⠼⠉⠀⠶⠤⠼⠉⠀⠭⠀⠤⠼⠛")?;
    return Ok(());
}

// This is Finnish notation. The bar | is used to denote that an operation is done on both sides of the equation.
// Here the equation is divided by five on both sides.
#[test]
fn p22_equation_solving_notation() -> Result<()> {
    let expr = "<math>
    <mn>5</mn>
    <mi>x</mi>
    <mo>=</mo>
    <mo>−</mo>
    <mn>10</mn>
    <mo>|</mo>
    <mo>:</mo>
    <mn>5</mn>
</math>";
    test_braille("Finnish", expr, "⠼⠑⠀⠭⠀⠶⠤⠼⠁⠚⠀⠀⠸⠀⠒⠼⠑")?;
    
    return Ok(());
}

// This is Finnish notation. The bar | is used to denote that an operation is done on both sides of the equation. This is not a real life example, but a string of mark up that should produce the output in the test: [space][vertical bar][operation done on both sides].
#[test]
fn operate_by_on_both_sides() -> Result<()> {
    let expr = "<math>
    <mo>|</mo>
    <mo>+</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>-</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>−</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>·</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>×</mo>
    <mn>5</mn>
    <mo>|</mo>
    <msup>
    <mrow>
    <mo>(</mo>
    <mo>)</mo>
    </mrow>
    <mn>2</mn>
    </msup>
    <mo>|</mo>
    <mo>/</mo>
    <mn>5</mn>
</math>";
    test_braille("Finnish", expr, "⠀⠀⠸⠀⠖⠼⠑⠀⠀⠸⠀⠤⠼⠑⠀⠀⠸⠀⠄⠼⠑⠀⠀⠸⠀⠄⠼⠑⠀⠀⠸⠀⠒⠼⠑⠀⠀⠸ ⠦⠴⠬⠀⠀⠸⠀⠌⠼⠑")?;
    return Ok(());
  }

// Sometimes also two bars || are used.
#[test]
fn p22_equation_solving_notation_double_bars() -> Result<()> {
    let expr = "<math>
    <mn>5</mn>
    <mi>x</mi>
    <mo>=</mo>
    <mo>−</mo>
    <mn>10</mn>
    <mo>|</mo>
    <mo>|</mo>
    <mo>:</mo>
    <mn>5</mn>
</math>";
    test_braille("Finnish", expr, "⠼⠑⠀⠭⠀⠶⠤⠼⠁⠚⠀⠀⠸⠸⠀⠒⠼⠑")?;
    
    return Ok(());
}

// Sometimes also two bars || are used. This is not a real life example, but a string of mark up that should produce the output in the test: [space][vertical bar][vertical bar][operation done on both sides].
#[test]
fn operate_by_on_both_sides_double_bars() -> Result<()> {
    let expr = "<math>
    <mo>|</mo>
    <mo>|</mo>
    <mo>+</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>|</mo>
    <mo>-</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>|</mo>
    <mo>−</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>|</mo>
    <mo>·</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>|</mo>
    <mo>×</mo>
    <mn>5</mn>
    <mo>|</mo>
    <mo>|</mo>
    <msup>
    <mrow>
    <mo>(</mo>
    <mo>)</mo>
    </mrow>
    <mn>2</mn>
    </msup>
    <mo>|</mo>
    <mo>|</mo>
    <mo>/</mo>
    <mn>5</mn>
</math>";
    test_braille("Finnish", expr, "⠀⠀⠸⠸⠀⠖⠼⠑⠀⠀⠸⠸⠀⠤⠼⠑⠀⠀⠸⠸⠀⠄⠼⠑⠀⠀⠸⠸⠀⠄⠼⠑⠀⠀⠸⠸⠀⠒⠼⠑⠀⠀⠸⠸ ⠦⠴⠬⠀⠀⠸⠸⠀⠌⠼⠑")?;
    return Ok(());
  }

#[test]
fn p24_fraction() -> Result<()> {
    let expr = "<math>
  <mfrac>
    <mrow>
      <mn>5</mn>
      <mo>+</mo>
      <mn>7</mn>
    </mrow>
    <mrow>
      <mn>2</mn>
      <mo>⋅</mo>
      <mn>3</mn>
    </mrow>
  </mfrac>
</math>";
    test_braille("Finnish", expr, "⠦⠼⠑⠀⠖⠼⠛⠴⠀⠌⠦⠼⠃⠀⠄⠼⠉⠴")?;
    return Ok(());
}

#[test]
fn p24_fraction_2() -> Result<()> {
    let expr = "<math>
  <mfrac>
    <mrow>
      <mn>4</mn>
      <mo>&#8290;</mo>
      <mi>x</mi>
    </mrow>
    <mrow>
      <mn>6</mn>
      <mo>&#8290;</mo>
      <mo>(</mo>
      <mn>1</mn>
      <mo>−</mo>
      <mi>x</mi>
      <mo>)</mo>
    </mrow>
  </mfrac>
</math>";
    test_braille("Finnish", expr, "⠼⠙⠀⠭⠀⠌⠦⠼⠋⠀⠦⠼⠁⠀⠤⠭⠴⠴")?;
    return Ok(());
}

// Rule fix: Fractions' division requires double parentheses so that the order of operations is respected.
#[test]
fn p25_long_fraction() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mfrac>
      <mrow>
        <msup>
          <mi>x</mi>
          <mn>2</mn>
        </msup>
        <mo>−</mo>
        <mn>7</mn>
        <mi>x</mi>
        <mo>+</mo>
        <mn>12</mn>
      </mrow>
      <mrow>
        <mn>4</mn>
        <mi>x</mi>
        <mo>−</mo>
        <mn>20</mn>
      </mrow>
    </mfrac>
    <mo>:</mo>
    <mfrac>
      <mrow>
        <msup>
          <mi>x</mi>
          <mn>2</mn>
        </msup>
        <mo>−</mo>
        <mn>8</mn>
        <mi>x</mi>
        <mo>+</mo>
        <mn>15</mn>
      </mrow>
      <mrow>
        <mn>4</mn>
        <mi>x</mi>
        <mo>−</mo>
        <mn>16</mn>
      </mrow>
    </mfrac>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠦⠦⠭⠬⠀⠤⠼⠛⠀⠭⠀⠖⠼⠁⠃⠴⠀⠌⠦⠼⠙⠀⠭⠀⠤⠼⠃⠚⠴⠴⠀⠒⠦⠦⠭⠬⠀⠤⠼⠓⠀⠭⠀⠖⠼⠁⠑⠴⠀⠌⠦⠼⠙⠀⠭⠀⠤⠼⠁⠋⠴⠴")?;
    return Ok(());
}

#[test]
fn p25_fraction_in_fraction() -> Result<()> {
    let expr = "<math>
  <mfrac>
    <mrow>
      <mfrac>
        <mn>1</mn>
        <mn>2</mn>
      </mfrac>
      <mo>+</mo>
      <mfrac>
        <mn>1</mn>
        <mn>3</mn>
      </mfrac>
    </mrow>
    <mrow>
      <mfrac>
        <mn>1</mn>
        <mn>4</mn>
      </mfrac>
      <mo>−</mo>
      <mfrac>
        <mn>1</mn>
        <mn>5</mn>
      </mfrac>
    </mrow>
  </mfrac>
</math>";
    test_braille("Finnish", expr, "⠦⠼⠁⠆⠀⠖⠼⠁⠒⠠⠴⠀⠌⠦⠼⠁⠲⠀⠤⠼⠁⠢⠠⠴")?;
    return Ok(());
}

#[test]
fn p26_exponent_multiplication_with_parens() -> Result<()> {
    let expr = "<math>
  <mrow>
    <msup>
      <mn>2</mn>
      <mn>3</mn>
    </msup>
    <mo>⋅</mo>
    <msup>
      <mn>2</mn>
      <mn>5</mn>
    </msup>
    <mo>=</mo>
    <msup>
      <mn>2</mn>
      <mrow>
        <mo>(</mo>
        <mn>3</mn>
        <mo>+</mo>
        <mn>5</mn>
        <mo>)</mo>
      </mrow>
    </msup>
    <mo>=</mo>
    <msup>
      <mn>2</mn>
      <mn>8</mn>
    </msup>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠬⠼⠉⠀⠄⠼⠃⠬⠼⠑⠀⠶⠼⠃⠬⠦⠼⠉⠀⠖⠼⠑⠴⠀⠶⠼⠃⠬⠼⠓")?;
    return Ok(());
}

#[test]
fn p26_exponent_multiplication_no_parens() -> Result<()> {
    let expr = "<math>
  <mrow>
    <msup>
      <mn>2</mn>
      <mn>3</mn>
    </msup>
    <mo>⋅</mo>
    <msup>
      <mn>2</mn>
      <mn>5</mn>
    </msup>
    <mo>=</mo>
    <msup>
      <mn>2</mn>
      <mrow>
        <mn>3</mn>
        <mo>+</mo>
        <mn>5</mn>
      </mrow>
    </msup>
    <mo>=</mo>
    <msup>
      <mn>2</mn>
      <mn>8</mn>
    </msup>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠬⠼⠉⠀⠄⠼⠃⠬⠼⠑⠀⠶⠼⠃⠬⠦⠼⠉⠀⠖⠼⠑⠴⠀⠶⠼⠃⠬⠼⠓")?;
    return Ok(());
}

#[test]
fn p27_exponent_to_large_number() -> Result<()> {
    let expr = "<math>
  <mrow>
    <msup>
      <mn>2</mn>
      <mn>20</mn>
    </msup>
    <mo>=</mo>
    <mn>1048576</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠬⠼⠃⠚⠀⠶⠼⠁⠄⠚⠙⠓⠄⠑⠛⠋")?;
    return Ok(());
}

#[test]
fn p27_sqrt() -> Result<()> {
    let expr = "<math>
  <mrow>
    <msup>
      <mn>4</mn>
      <mfrac>
        <mn>1</mn>
        <mn>2</mn>
      </mfrac>
    </msup>
    <mo>=</mo>
    <msqrt>
      <mn>4</mn>
    </msqrt>
    <mo>=</mo>
    <mn>2</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠙⠬⠼⠁⠆⠀⠶⠩⠼⠙⠀⠶⠼⠃")?;
    return Ok(());
}

#[test]
fn p31_trig_tan_infinity() -> Result<()> {
    let expr = "<math>
      <mi>tan</mi>
      <mo>⁡</mo>
      <mn>90</mn>
      <mi>°</mi>
      <mo>=</mo>
      <mi>∞</mi>
</math>";
    test_braille("Finnish", expr, "⠞⠁⠝⠀⠼⠊⠚⠀⠴⠀⠶⠿")?;
    return Ok(());
}

#[test]
fn p31_degrees_minutes_seconds_primes() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mn>30</mn>
    <mi>°</mi>
    <msup>
      <mn>45</mn>
      <mo>′</mo>
    </msup>
    <msup>
      <mn>20</mn>
      <mrow>
        <mo>′′</mo>
      </mrow>
    </msup>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠉⠚⠀⠴⠀⠼⠙⠑⠀⠨⠀⠼⠃⠚⠀⠸")?;
    return Ok(());
}

#[test]
fn p31_degrees_minutes_seconds_alternative() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mn>30</mn>
    <mi>°</mi>
    <msup>
      <mn>45</mn>
      <mo>'</mo>
    </msup>
    <msup>
      <mn>20</mn>
      <mrow>
        <mo>''</mo>
      </mrow>
    </msup>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠉⠚⠀⠴⠀⠼⠙⠑⠀⠨⠀⠼⠃⠚⠀⠸")?;
    return Ok(());
}

#[test]
fn msub_singular() -> Result<()> {
  init_logger();
    let expr = "<math>
    <msub>
    <mi>x</mi>
    <mn>1</mn>
    </msub>
    <mo>+</mo>
    <mn>1</mn>
</math>";
    test_braille("Finnish", expr, "⠭⠡⠼⠁⠀⠖⠼⠁")?;
    return Ok(());
}

#[test]
fn msub_parentheses() -> Result<()> {
    let expr = "<math>
    <msub>
    <mi>a</mi>
    <mrow>
    <mi>x</mi>
    <mo>+</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    <mo>+</mo>
    <mn>1</mn>
</math>";
    test_braille("Finnish", expr, "⠁⠡⠦⠭⠀⠖⠼⠁⠴⠀⠖⠼⠁")?;
    return Ok(());
}

// Overline braille notation (dots 156) comes after the (sub)script.
// For the rule: order should be first subscript and then superscript
#[test]
fn msub_with_vector() -> Result<()> {
    let expr = "<math>
      <msub>
      <mover>
        <mi>a</mi>
        <mo>¯</mo>
      </mover>
        <mi>b</mi>
      </msub>
    <mo>+</mo>
    <mn>1</mn>
</math>";
    test_braille("Finnish", expr, "⠁⠡⠃⠱⠀⠖⠼⠁")?;
    return Ok(());
}

// Even when the vector is marked with an arrow, it should output the same dots (156).
#[test]
fn msub_with_vector_arrow() -> Result<()> {
    let expr = "<math>
      <msub>
      <mover>
        <mi>a</mi>
        <mo>→</mo>
      </mover>
        <mi>b</mi>
      </msub>
    <mo>+</mo>
    <mn>1</mn>
</math>";
    test_braille("Finnish", expr, "⠁⠡⠃⠱⠀⠖⠼⠁")?;
    return Ok(());
}

// Overline braille notation (dots 156) comes after the (sub)script.
#[test]
fn msub_with_vector_with_sub_text() -> Result<()> {
    let expr = "<math>
  <mrow data-changed='added'>
      <msub>
        <mover>
          <mi>a</mi>
          <mo>¯</mo>
        </mover>
        <mtext>tuki</mtext>
      </msub>
    <mo>+</mo>
    <mn>1</mn>
  </mrow>
 </math>";
    test_braille("Finnish", expr, "⠁⠡⠞⠥⠅⠊⠱⠀⠖⠼⠁")?;
    return Ok(());
}

// First the subscript, then superscript and then overline notation.
#[test]
fn msubsup_with_vector() -> Result<()> {
    let expr = "<math>
  <mrow>
      <msubsup>
        <mover>
          <mi>a</mi>
          <mo>¯</mo>
        </mover>
        <mn>5</mn>
        <mn>0</mn>
      </msubsup>
    <mo>+</mo>
    <mn>1</mn>
  </mrow>
 </math>";
    test_braille("Finnish", expr, "⠁⠡⠼⠑⠌⠼⠚⠱⠀⠖⠼⠁")?;
    return Ok(());
}

// First the subscript, then superscript and then overline notation.
#[test]
fn msubsup_with_vector_complex_with_parentheses() -> Result<()> {
    let expr = "<math>
  <mrow>
      <msubsup>
        <mover>
          <mi>a</mi>
          <mo>¯</mo>
        </mover>
        <mrow>
          <mo>(</mo>
          <mn>5</mn>
          <mo>+</mo>
          <mn>2</mn>
          <mo>)</mo>
        </mrow>
        <mrow>
          <mo>(</mo>
          <mn>3</mn>
          <mo>+</mo>
          <mi>x</mi>
          <mo>)</mo>
        </mrow>
      </msubsup>
    <mo>+</mo>
    <mn>1</mn>
  </mrow>
 </math>";
    test_braille("Finnish", expr, "⠁⠡⠦⠼⠑⠀⠖⠼⠃⠴⠌⠦⠼⠉⠀⠖⠭⠴⠱⠀⠖⠼⠁")?;
    return Ok(());
}

// First the subscript and then the superscript.
#[test]
fn msubsup() -> Result<()> {
    let expr = "<math>
    <msubsup>
    <mi>a</mi>
    <mn>1</mn>
    <mn>2</mn>
    </msubsup>
    <mo>+</mo>
    <mn>1</mn>
</math>";
    test_braille("Finnish", expr, "⠁⠡⠼⠁⠬⠼⠃⠀⠖⠼⠁")?;
    return Ok(());
}

// The scripts should have parentheses for easier reading.
#[test]
fn msubsup_parentheses_in_sub_and_sup() -> Result<()> {
    let expr = "<math>
    <msubsup>
    <mi>a</mi>
    <mrow>
    <mi>x</mi>
    <mo>+</mo>
    <mn>1</mn>
    </mrow>
    <mrow>
    <mi>x</mi>
    <mo>−</mo>
    <mn>10</mn>
    </mrow>
    </msubsup>
    <mo>+</mo>
    <mn>1</mn>
</math>";
    test_braille("Finnish", expr, "⠁⠡⠦⠭⠀⠖⠼⠁⠴⠬⠦⠭⠀⠤⠼⠁⠚⠴⠀⠖⠼⠁")?;
    return Ok(());
}

#[test]
fn p32_expected_value() -> Result<()> {
    let expr = "<math>
    <mi>E</mi>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
    <mo>=</mo>
    <mi>μ</mi>
    <mo>=</mo>
    <msub>
      <mo>∑</mo>
      <mi>i</mi>
    </msub>
    <msub>
      <mi>p</mi>
      <mi>i</mi>
    </msub>
    <msub>
      <mi>x</mi>
      <mi>i</mi>
    </msub>
</math>";
    test_braille("Finnish", expr, "⠠⠑⠦⠭⠴⠀⠶⠨⠍⠀⠶⠸⠎⠡⠊⠐⠏⠡⠊⠐⠭⠡⠊")?;
    return Ok(());
}

// Dots 156 to signify the bar over "AB"
#[test]
fn p33_vector() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mover>
        <mi>AB</mi>
        <mo>‾</mo>
    </mover>
    <mo>=</mo>
    <mn>3</mn>
    <mover>
      <mi>i</mi>
      <mo>‾</mo>
    </mover>
    <mo>−</mo>
    <mn>4</mn>
    <mover>
      <mi>j</mi>
      <mo>‾</mo>
    </mover>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠠⠁⠠⠃⠱⠀⠶⠼⠉⠀⠊⠱⠀⠤⠼⠙⠀⠚⠱")?;
    return Ok(());
}

#[test]
fn p33_parallel_vectors() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mover>
      <mi>a</mi>
      <mo>‾</mo>
    </mover>
    <mo>∥</mo>
    <mover>
      <mi>b</mi>
      <mo>‾</mo>
    </mover>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠁⠱⠀⠸⠸⠀⠃⠱")?;
    return Ok(());
}

#[test]
fn p34_vector_dot_product() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mover>
      <mi>v</mi>
      <mo>‾</mo>
    </mover>
    <mo>⋅</mo>
    <mover>
      <mi>u</mi>
      <mo>‾</mo>
    </mover>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠧⠱⠀⠄⠥⠱")?;
    return Ok(());
}

#[test]
fn p34_vector_cross_product() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mover>
      <mi>a</mi>
      <mo>‾</mo>
    </mover>
      <mo>×</mo>
    <mover>
      <mi>b</mi>
      <mo>‾</mo>
    </mover>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠁⠱⠀⠰⠄⠃⠱")?;
    return Ok(());
}

#[test]
fn p39_not_in_set() -> Result<()> {
    let expr = "<math>
    <mn>3</mn>
    <mo>∉</mo>
    <mi>B</mi>
</math>";
    test_braille("Finnish", expr, "⠼⠉⠀⠳⠐⠔⠠⠃")?;
    return Ok(());
}

#[test]
fn p39_subset() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mi>B</mi>
    <mo>⊂</mo>
    <mi>A</mi>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠠⠃⠀⠳⠪⠠⠁")?;
    return Ok(());
}

#[test]
fn p39_not_subset() -> Result<()> {
    let expr = "<math>
    <mi>F</mi>
    <mo>⊄</mo>
    <mi>E</mi>
</math>";
    test_braille("Finnish", expr, "⠠⠋⠀⠳⠐⠪⠠⠑")?;
    return Ok(());
}

#[test]
fn p39_equal_sets_with_mtext() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mi>C</mi>
    <mo>⊆</mo>
    <mi>D</mi>
    <mtext> ja </mtext>
    <mi>D</mi>
    <mo>⊆</mo>
    <mi>C</mi>
    <mo>⇔</mo>
    <mi>C</mi>
    <mo>=</mo>
    <mi>D</mi>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠠⠉⠀⠳⠶⠪⠠⠙⠀⠚⠁⠀⠠⠙⠀⠳⠶⠪⠠⠉⠀⠣⠤⠤⠱⠀⠠⠉⠀⠶⠠⠙")?;
    return Ok(());
}

#[test]
fn p39_set_of_points_in_R2() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mi>A</mi>
    <mo>×</mo>
    <mi>B</mi>
    <mo>=</mo>
    <mo>{</mo>
    <mo>(</mo>
    <mn>1</mn>
    <mo>,</mo>
    <mn>3</mn>
    <mo>)</mo>
    <mo>,</mo>
    <mo>(</mo>
    <mn>1</mn>
    <mo>,</mo>
    <mn>4</mn>
    <mo>)</mo>
    <mo>,</mo>
    <mo>(</mo>
    <mn>2</mn>
    <mo>,</mo>
    <mn>3</mn>
    <mo>)</mo>
    <mo>,</mo>
    <mo>(</mo>
    <mn>2</mn>
    <mo>,</mo>
    <mn>4</mn>
    <mo>)</mo>
    <mo>}</mo>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠠⠁⠀⠰⠄⠠⠃⠀⠶⠫⠦⠼⠁⠂⠀⠼⠉⠴⠂⠀⠦⠼⠁⠂⠀⠼⠙⠴⠂⠀⠦⠼⠃⠂⠀⠼⠉⠴⠂⠀⠦⠼⠃⠂⠀⠼⠙⠴⠻")?;
    return Ok(());
}

#[test]
fn p40_not_p() -> Result<()> {
    let expr = "<math>
    <mo>¬</mo>
    <mi>p</mi>
</math>";
    test_braille("Finnish", expr, "⠳⠲⠏")?;
    return Ok(());
}

#[test]
fn p40_p_and_q() -> Result<()> {
    let expr = "<math>
    <mi>p</mi>
    <mo>∧</mo>
    <mi>q</mi>
</math>";
    test_braille("Finnish", expr, "⠏⠀⠳⠩⠀⠟")?;
    return Ok(());
}

#[test]
fn p40_p_or_q() -> Result<()> {
    let expr = "<math>
    <mi>p</mi>
    <mo>∨</mo>
    <mi>q</mi>
</math>";
    test_braille("Finnish", expr, "⠏⠀⠳⠬⠀⠟")?;
    return Ok(());
}

// The colon : should be written directly after the function f (and similar). This is different compared to it being used as a division sign and as a ratio.
#[test]
fn p41_function_definition() -> Result<()> {
    let expr = "<math>
    <mi>f</mi>
    <mo>:</mo>
    <mi>x</mi>
    <mo>→</mo>
    <mi>f</mi>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
</math>";
    test_braille("Finnish", expr, "⠋⠒⠀⠭⠀⠤⠱⠀⠋⠦⠭⠴")?;
    return Ok(());
}

// Here the bar | should have whitespace on both sides of the character.
#[test]
fn p41_inverse_function() -> Result<()> {
    let expr = "<math>
    <msup>
      <mi>f</mi>
      <mrow>
        <mo>−</mo>
        <mn>1</mn>
      </mrow>
    </msup>
    <mo>=</mo>
  <mrow>
    <mo>{</mo>
    <mo>(</mo>
    <mi>y</mi>
    <mo>,</mo>
    <mi>x</mi>
    <mo>)</mo>
    <mo>∈</mo>
    <mi>B</mi>
    <mo>×</mo>
    <mi>A</mi>
    <mo>|</mo>
    <mi>y</mi>
    <mo>=</mo>
    <mi>f</mi>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
    <mo>}</mo>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠋⠬⠤⠼⠁⠀⠶⠫⠦⠽⠂⠀⠭⠴⠀⠳⠔⠠⠃⠀⠰⠄⠠⠁⠀⠸⠀⠽⠀⠶⠋⠦⠭⠴⠻")?;
    return Ok(());
}


// Two 5-dots separate the rows in the 2D math. Format: [whitespace][dot 5][dot 5][whitespace]. This is not an operator.
#[test]
fn p41_function_with_parts_with_arrow() -> Result<()> {
    let expr = "<math>
      <mi>f</mi>
      <mo>:</mo>
      <mi>x</mi>
      <mo>→</mo>
      <mrow>
         <mo>{</mo>
         <mrow>
            <mtable>
               <mtr>
                  <mtd>
                     <mrow>
                        <mn>&#x2212;1</mn>
                        <mo>,</mo>
                        <mi>x</mi>
                        <mo>≤</mo>
                        <mn>&#x2212;1</mn>
                     </mrow>
                  </mtd>
               </mtr>
               <mtr>
                  <mtd>
                     <mrow>
                        <msup>
                           <mi>x</mi>
                           <mn>2</mn>
                        </msup>
                        <mo>+</mo>
                        <mn>2</mn>
                        <mi>x</mi>
                        <mo>,</mo>
                        <mn>&#x2212;1</mn>
                        <mo>&lt;</mo>
                        <mi>x</mi>
                        <mo>≤</mo>
                        <mn>1</mn>
                     </mrow>
                  </mtd>
               </mtr>
               <mtr>
                  <mtd>
                     <mrow>
                        <mo>&#x2212;</mo>
                        <mi>x</mi>
                        <mo>+</mo>
                        <mn>3</mn>
                        <mo>,</mo>
                        <mi>x</mi>
                        <mo>&#x003E;</mo>
                        <mn>1</mn>
                     </mrow>
                  </mtd>
               </mtr>
            </mtable>
         </mrow>
      </mrow>
</math>
";
    test_braille("Finnish", expr, "⠋⠒⠀⠭⠀⠤⠱⠀⠫⠤⠼⠁⠂⠀⠭⠀⠣⠶⠀⠤⠼⠁⠀⠐⠐⠀⠭⠬⠀⠖⠼⠃⠀⠭⠂⠀⠤⠼⠁⠀⠣⠀⠭⠀⠣⠶⠀⠼⠁⠀⠐⠐⠀⠤⠭⠀⠖⠼⠉⠂⠀⠭⠀⠱⠀⠼⠁⠻")?;
    return Ok(());
}

// Two 5-dots separate the rows in the 2D math. Format: [whitespace][dot 5][dot 5][whitespace]. This is not an operator.
#[test]
fn p40_function_with_parts() -> Result<()> {
  let expr = "<math>
      <mi>f</mi>
      <mo>(</mo>
      <mi>x</mi>
      <mo>)</mo>
      <mo>=</mo>
      <mrow>
         <mo>{</mo>
         <mrow>
            <mtable>
               <mtr>
                  <mtd>
                     <mrow>
                        <mn>&#x2212;1</mn>
                        <mo>,</mo>
                        <mi>x</mi>
                        <mo>≤</mo>
                        <mn>&#x2212;1</mn>
                     </mrow>
                  </mtd>
               </mtr>
               <mtr>
                  <mtd>
                     <mrow>
                        <msup>
                           <mi>x</mi>
                           <mn>2</mn>
                        </msup>
                        <mo>+</mo>
                        <mn>2</mn>
                        <mi>x</mi>
                        <mo>,</mo>
                        <mn>&#x2212;1</mn>
                        <mo>&lt;</mo>
                        <mi>x</mi>
                        <mo>≤</mo>
                        <mn>1</mn>
                     </mrow>
                  </mtd>
               </mtr>
               <mtr>
                  <mtd>
                     <mrow>
                        <mo>&#x2212;</mo>
                        <mi>x</mi>
                        <mo>+</mo>
                        <mn>3</mn>
                        <mo>,</mo>
                        <mi>x</mi>
                        <mo>&#x003E;</mo>
                        <mn>1</mn>
                     </mrow>
                  </mtd>
               </mtr>
            </mtable>
         </mrow>
      </mrow>
</math>";
  test_braille("Finnish", expr, "⠋⠦⠭⠴⠀⠶⠫⠤⠼⠁⠂⠀⠭⠀⠣⠶⠀⠤⠼⠁⠀⠐⠐⠀⠭⠬⠀⠖⠼⠃⠀⠭⠂⠀⠤⠼⠁⠀⠣⠀⠭⠀⠣⠶⠀⠼⠁⠀⠐⠐⠀⠤⠭⠀⠖⠼⠉⠂⠀⠭⠀⠱⠀⠼⠁⠻")?;
    return Ok(());
}

// Two 5-dots separate the rows in the 2D math. Format: [whitespace][dot 5][dot 5][whitespace]. This is not an operator.
// In the matrix the parentheses are repeated for every row.
#[test]
fn p42_matrix() -> Result<()> {
    let expr = "<math>
<mrow>
<mo>(</mo>
<mtable>
	<mtr>
    <mtd>
    <mn>1</mn>
    </mtd>
    <mtd>
    <mn>0</mn>
    </mtd>
    <mtd>
    <mn>0</mn>
    </mtd>
    <mtd>
    <mn>1</mn>
    </mtd>
    </mtr>
	<mtr>
    <mtd>
    <mn>0</mn>
    </mtd>
    <mtd>
    <mn>1</mn>
    </mtd>
    <mtd>
    <mn>0</mn>
    </mtd>
    <mtd>
    <mn>0</mn>
    </mtd>
    </mtr>
	<mtr>
    <mtd>
    <mn>0</mn>
    </mtd>
    <mtd>
    <mn>0</mn>
    </mtd>
    <mtd>
    <mn>1</mn>
    </mtd>
    <mtd>
    <mn>0</mn>
    </mtd>
    </mtr>
	<mtr>
    <mtd>
    <mn>1</mn>
    </mtd>
    <mtd>
    <mn>0</mn>
    </mtd>
    <mtd>
    <mn>0</mn>
    </mtd>
    <mtd>
    <mn>1</mn>
    </mtd>
    </mtr>
</mtable>
<mo>)</mo>
</mrow>
</math>";
    test_braille("Finnish", expr, "⠦⠼⠁⠀⠼⠚⠀⠼⠚⠀⠼⠁⠴⠀⠐⠐⠀⠦⠼⠚⠀⠼⠁⠀⠼⠚⠀⠼⠚⠴⠀⠐⠐⠀⠦⠼⠚⠀⠼⠚⠀⠼⠁⠀⠼⠚⠴⠀⠐⠐⠀⠦⠼⠁⠀⠼⠚⠀⠼⠚⠀⠼⠁⠴")?;
    return Ok(());
}

// Empty cells are marked up with dots (25).
#[test]
fn matrix_with_empty_cells() -> Result<()> {
    let expr = "<math>
<mrow>
<mo>(</mo>
<mtable>
	<mtr>
    <mtd>
    <mn>1</mn>
    </mtd>
    <mtd>
    </mtd>
    <mtd>
    </mtd>
    <mtd>
    <mn>1</mn>
    </mtd>
    </mtr>
	<mtr>
    <mtd>
    </mtd>
    <mtd>
    <mn>1</mn>
    </mtd>
    <mtd>
    </mtd>
    <mtd>
    </mtd>
    </mtr>
	<mtr>
    <mtd>
    </mtd>
    <mtd>
    </mtd>
    <mtd>
    <mn>1</mn>
    </mtd>
    <mtd>
    </mtd>
    </mtr>
	<mtr>
    <mtd>
    <mn>1</mn>
    </mtd>
    <mtd>
    </mtd>
    <mtd>
    </mtd>
    <mtd>
    <mn>1</mn>
    </mtd>
    </mtr>
</mtable>
<mo>)</mo>
</mrow>
</math>";
    test_braille("Finnish", expr, "⠦⠼⠁⠀⠒⠀⠒⠀⠼⠁⠴⠀⠐⠐⠀⠦⠒⠀⠼⠁⠀⠒⠀⠒⠴⠀⠐⠐⠀⠦⠒⠀⠒⠀⠼⠁⠀⠒⠴⠀⠐⠐⠀⠦⠼⠁⠀⠒⠀⠒⠀⠼⠁⠴")?;
    return Ok(());
}

// Two 5-dots separate the rows in the 2D math. Format: [whitespace][dot 5][dot 5][whitespace]. This is not an operator.
// In the determinant the vertical bar is repeated for every row.
#[test]
fn p42_determinant() -> Result<()> {
    let expr = "<math>
    <mrow>
    <mo>|</mo>
    <mtable>
      <mtr>
        <mtd>
        <mi>a</mi>
        <mo>+</mo>
        <mi>b</mi>
        </mtd>
        <mtd>
        <mi>a</mi>
        <mo>−</mo>
        <mi>b</mi>
        </mtd>
        </mtr>
        <mtr>
        <mtd>
        <mi>a</mi>
        <mo>−</mo>
        <mi>b</mi>
        </mtd>
        <mtd>
        <mi>a</mi>
        <mo>+</mo>
        <mi>b</mi>
        </mtd>
        </mtr>
    </mtable>
    <mo>|</mo>
    </mrow>
    <mo>=</mo>
    <mn>4</mn>
    <mi>a</mi>
    <mi>b</mi>
    </math>";
    test_braille("Finnish", expr, "⠸⠁⠀⠖⠃⠀⠁⠀⠤⠃⠸⠀⠐⠐⠀⠸⠁⠀⠤⠃⠀⠁⠀⠖⠃⠸⠀⠶⠼⠙⠀⠁⠃")?;
    return Ok(());
}

#[test]
fn open_interval() -> Result<()> {
    let expr = "<math> 
  <mo>]</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>[</mo>
</math>";
    test_braille("Finnish", expr, "⠾⠉⠂⠀⠙⠷")?;
    return Ok(());
}

#[test]
fn open_interval_alt() -> Result<()> {
    let expr = "<math> 
  <mo>(</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>)</mo>
</math>";
    test_braille("Finnish", expr, "⠦⠉⠂⠀⠙⠴")?;
    return Ok(());
}

#[test]
fn half_open_interval_left() -> Result<()> {
    let expr = "<math> 
  <mo>]</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>]</mo>
</math>";
    test_braille("Finnish", expr, "⠾⠉⠂⠀⠙⠾")?;
    return Ok(());
}

#[test]
fn half_open_interval_left_alt() -> Result<()> {
    let expr = "<math> 
  <mo>(</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>]</mo>
</math>";
    test_braille("Finnish", expr, "⠦⠉⠂⠀⠙⠾")?;
    return Ok(());
}

#[test]
fn half_open_interval_right() -> Result<()> {
    let expr = "<math> 
  <mo>[</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>[</mo>
</math>";
    test_braille("Finnish", expr, "⠷⠉⠂⠀⠙⠷")?;
    return Ok(());
}

#[test]
fn half_open_interval_right_alt() -> Result<()> {
    let expr = "<math> 
  <mo>[</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>)</mo>
</math>";
    test_braille("Finnish", expr, "⠷⠉⠂⠀⠙⠴")?;
    return Ok(());
}

#[test]
fn closed_interval() -> Result<()> {
    let expr = "<math> 
  <mo>[</mo>
    <mi>c</mi>
    <mo>,</mo>
    <mi>d</mi>
    <mo>]</mo>
</math>";
    test_braille("Finnish", expr, "⠷⠉⠂⠀⠙⠾")?;
    return Ok(());
}

// Dots 156 to signify the bar over "arc".
#[test]
fn p45_arcsin_with_bar_over() -> Result<()> {
    let expr = "<math>
<mover>
<mi>arc</mi>
<mo>‾</mo>
</mover>
<mi>sin</mi>
<mi>x</mi>
</math>";
    test_braille("Finnish", expr, "⠁⠗⠉⠱⠀⠎⠊⠝⠀⠭")?;
    return Ok(());
}

#[test]
fn p46_simple_derivative_fraction() -> Result<()> {
    let expr = "<math><mfrac>
        <mrow><mi>d</mi><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        <mrow><mi>d</mi><mi>x</mi></mrow>
      </mfrac></math>";
    test_braille("Finnish", expr, "⠙⠋⠦⠭⠴⠀⠌⠙⠭")?;
    return Ok(());
}

#[test]
fn p46_first_order_derivative() -> Result<()> {
    let expr = "<math><mi>f</mi><mo>'</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    test_braille("Finnish", expr, "⠋⠔⠦⠭⠴")?;
    return Ok(());
}

#[test]
fn p46_first_order_derivative_alt() -> Result<()> {
    let expr = "<math><mi>f</mi><mo>’</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    test_braille("Finnish", expr, "⠋⠔⠦⠭⠴")?;
    return Ok(());
}

#[test]
fn second_order_derivative() -> Result<()> {
    let expr = "<math><mi>f</mi><mo>''</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    test_braille("Finnish", expr, "⠋⠔⠔⠦⠭⠴")?;
    return Ok(());
}

#[test]
fn second_order_derivative_alt_1() -> Result<()> {
    let expr = "<math><mi>f</mi><mo>’’</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    test_braille("Finnish", expr, "⠋⠔⠔⠦⠭⠴")?;
    return Ok(());
}

#[test]
fn second_order_derivative_alt_2() -> Result<()> {
    let expr = "<math><mi>f</mi><mo>”</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    test_braille("Finnish", expr, "⠋⠔⠔⠦⠭⠴")?;
    return Ok(());
}

#[test]
fn nth_order_derivative() -> Result<()> {
    let expr = "<math><msup><mi>f</mi><mrow><mo>(</mo><mi>n</mi><mo>)</mo></mrow></msup><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    test_braille("Finnish", expr, "⠋⠌⠦⠝⠴⠦⠭⠴")?;
    return Ok(());
}

// Line under a variable
#[test]
fn p46_random_variable() -> Result<()> {
    let expr = "<math>
<munder>
<mi>s</mi>
<mo>_</mo>
</munder>
</math>";
    test_braille("Finnish", expr, "⠎⠤")?;
    return Ok(());
}

#[test]
fn line_under_terminated_by_space() -> Result<()> {
    let expr = "<math>
<munder>
<mi>s</mi>
<mo>_</mo>
</munder>
<mo>+</mo>
<mn>1</mn>
</math>";
    test_braille("Finnish", expr, "⠎⠤⠀⠖⠼⠁")?;
    return Ok(());
}

// Needs parentheses so that grouping is clear.
#[test]
fn line_under_parentheses() -> Result<()> {
    let expr = "<math>
<munder>
<mrow>
<mi>s</mi>
<mo>/</mo>
<mn>2</mn>
</mrow>
<mo>_</mo>
</munder>
<mo>+</mo>
<mn>1</mn>
</math>";
    test_braille("Finnish", expr, "⠦⠎⠀⠌⠼⠃⠴⠤⠀⠖⠼⠁")?;
    return Ok(());
}

// 'Change of zone' dot 5 is used after the limit subscript notation. The subscript is in parentheses for easier reading.
#[test]
fn p48_limit_right_hand_approaching_Finnish_notation() -> Result<()> {
    let expr = "<math>
  <mrow>
    <msub>
      <mi>lim</mi>
      <mrow>
        <mi>x</mi>
        <mo>→</mo>
        <mn>0</mn>
        <mo>+</mo>
      </mrow>
    </msub>
  </mrow>
  <mrow>
    <mi>f</mi>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠇⠊⠍⠡⠦⠭⠀⠤⠱⠀⠼⠚⠀⠖⠴⠐⠋⠦⠭⠴")?;
    return Ok(());
}

// 'Change of zone' dot 5 is used after the limit subscript notation. The subscript is in parentheses for easier reading.
#[test]
fn p48_limit_right_hand_approaching_Finnish_notation_with_msup() -> Result<()> {
    let expr = "<math>
  <mrow>
    <msub>
      <mi>lim</mi>
      <mrow>
        <mi>x</mi>
        <mo>→</mo>
        <msup>
        <mn>0</mn>
        <mo>+</mo>
        </msup>
      </mrow>
    </msub>
  </mrow>
  <mrow>
    <mi>f</mi>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠇⠊⠍⠡⠦⠭⠀⠤⠱⠀⠼⠚⠬⠖⠴⠐⠋⠦⠭⠴")?;
    return Ok(());
}

// 'Change of zone' dot 5 is used after the limit subscript notation. The subscript is in parentheses for easier reading.
#[test]
fn limit_approaches_from_below() -> Result<()> {
    let expr = "<math>
<munder>
<mo>lim</mo>
<mrow>
<mi>x</mi>
<mo>↗</mo>
<mn>0</mn>
</mrow>
</munder>
<mrow>
<mrow>
<mi>f</mi>
<mo>(</mo>
<mi>x</mi>
<mo>)</mo>
</mrow>
</mrow>
</math>";
    test_braille("Finnish", expr, "⠇⠊⠍⠡⠦⠭⠀⠔⠱⠀⠼⠚⠴⠐⠋⠦⠭⠴")?;
    return Ok(());
}

// Finnish integral substitution notation. The intent might be wrong, but that is what the notation means.
#[test]
fn p49_integral_with_Finnish_notation_for_substitution() -> Result<()> {
    let expr = "<math>
  <mrow>
    <msubsup>
      <mo>∫</mo>
      <mn>0</mn>
      <mn>4</mn>
    </msubsup>
    <msqrt>
      <mi>x</mi>
    </msqrt>
    <mi>d</mi>
    <mi>x</mi>
    <mo>=</mo>
    <msubsup>
      <mo intent='substitution'>⧸</mo>
      <mn>0</mn>
      <mn>4</mn>
    </msubsup>
    <mfrac>
      <mn>2</mn>
      <mn>3</mn>
    </mfrac>
    <mi>x</mi>
    <msqrt>
      <mi>x</mi>
    </msqrt>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠮⠢⠼⠚⠔⠼⠙⠐⠩⠭⠀⠙⠭⠀⠶⠸⠢⠼⠚⠔⠼⠙⠐⠦⠼⠃⠒⠀⠭⠀⠩⠭⠴")?;
    return Ok(());
}

// Markup for lower and upper bounds for summation, product, intersection, union and such. "Equal to" character is replaced lower (dots 26) and upper (dots 35) bounds.
// Parentheses are added for clarity. Dot 5 is used to end the zone of the upper bound.
#[test]
fn p49_summation_with_lower_upper_bounds() -> Result<()> {
    let expr = "<math>
<munderover>
<mo>∑</mo>
<mrow>
<mi>i</mi>
<mo>=</mo>
<mn>0</mn>
</mrow>
<mi>n</mi>
</munderover>
<msub>
<mi>f</mi>
<mi>i</mi>
</msub>
<msub>
<mi>x</mi>
<mi>i</mi>
</msub>
</math>";
    test_braille("Finnish", expr, "⠸⠎⠡⠊⠢⠼⠚⠔⠝⠐⠦⠋⠡⠊⠀⠭⠡⠊⠴")?;
    return Ok(());
}

// Markup for lower and upper bounds for summation, product, intersection, union and such. "Equal to" character is replaced lower (dots 26) and upper (dots 35) bounds.
// Dot 5 is used to end the zone of the upper bound.
#[test]
fn p49_union_with_lower_upper_bounds() -> Result<()> {
    let expr = "<math>
<munderover>
<mo>∪</mo>
<mrow>
<mi>i</mi>
<mo>=</mo>
<mn>1</mn>
</mrow>
<mi>n</mi>
</munderover>
<msub>
<mi>A</mi>
<mi>i</mi>
</msub>
</math>";
    test_braille("Finnish", expr, "⠳⠖⠡⠊⠢⠼⠁⠔⠝⠐⠠⠁⠡⠊")?;
    return Ok(());
}

// The test combines subscript notation and the lower and upper bound notation. "Equal to" character is replaced lower (dots 26) and upper (dots 35) bounds.
// Dot 5 is used to end the zone of the upper bound.
#[test]
fn p49_sequence_with_lower_upper_bounds() -> Result<()> {
    let expr = "<math>
<msubsup>
<mrow>
<mo>(</mo>
<msub>
<mi>x</mi>
<mi>n</mi>
</msub>
<mo>)</mo>
</mrow>
<mrow>
<mi>n</mi>
<mo>=</mo>
<mn>1</mn>
</mrow>
<mi>∞</mi>
</msubsup>
</math>";
    test_braille("Finnish", expr, "⠦⠭⠡⠝⠴⠡⠝⠢⠼⠁⠔⠿")?;
    return Ok(());
}

// Regular letters are used in the bold lettering.
#[test]
fn p50_such_that_y_greater_than_x() -> Result<()> {
    let expr = "<math>
    <mi>∀</mi>
    <mi>x</mi>
    <mo>∈</mo>
    <mi>ℝ</mi>
    <mo>,</mo>
    <mi>∃</mi>
    <mi>y</mi>
    <mo>∈</mo>
    <mi>ℝ</mi>
    <mo>;</mo>
    <mi>y</mi>
    <mo>&gt;</mo>
    <mi>x</mi>
</math>";
    test_braille("Finnish", expr, "⠳⠂⠭⠀⠳⠔⠠⠗⠂⠀⠳⠢⠽⠀⠳⠔⠠⠗⠆⠀⠽⠀⠱⠀⠭")?;
    return Ok(());
}

// After the root sign, dot 5 is used to signify 'change of zone' in the braille.
#[test]
fn p50_cube_root() -> Result<()> {
    let expr = "<math>
  <mroot>
    <mrow>
      <mi>x</mi>
      <mo>+</mo>
      <mn>5</mn>
    </mrow>
    <mn>3</mn>
  </mroot>
</math>";
    test_braille("Finnish", expr, "⠩⠼⠉⠐⠦⠭⠀⠖⠼⠑⠴")?;
    return Ok(());
}

// The ⠯ character is used between the elements in the binomial.
#[test]
fn p50_binomial_with_factorial() -> Result<()> {
    let expr = r#"<math>
  <mrow>
    <mrow>
      <mo>(</mo>
      <mfrac linethickness="0px">
        <mn>6</mn>
        <mn>4</mn>
      </mfrac>
      <mo>)</mo>
    </mrow>
    <mo>=</mo>
    <mfrac>
      <mrow>
        <mn>6</mn>
        <mo>!</mo>
      </mrow>
      <mrow>
        <mn>4</mn>
        <mo>!</mo>
        <mo>(</mo>
        <mn>6</mn>
        <mo>−</mo>
        <mn>4</mn>
        <mo>)</mo>
        <mo>!</mo>
      </mrow>
    </mfrac>
  </mrow>
</math>"#;
    test_braille("Finnish", expr, "⠦⠼⠋⠯⠀⠼⠙⠴⠀⠶⠼⠋⠠⠲⠀⠌⠷⠼⠙⠠⠲⠀⠦⠼⠋⠀⠤⠼⠙⠠⠴⠠⠲⠾")?;
    return Ok(());
}

#[test]
fn p50_conditional_probability() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mi>P</mi>
    <mo>(</mo>
    <mi>B</mi>
    <mi>|</mi>
    <mi>A</mi>
    <mo>)</mo>
    <mo>=</mo>
    <mfrac>
      <mrow>
        <mi>P</mi>
        <mo>(</mo>
        <mi>A</mi>
        <mi>B</mi>
        <mo>)</mo>
      </mrow>
      <mrow>
        <mi>P</mi>
        <mo>(</mo>
        <mi>A</mi>
        <mo>)</mo>
      </mrow>
    </mfrac>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠠⠏⠦⠠⠃⠀⠸⠀⠠⠁⠴⠀⠶⠠⠏⠦⠠⠁⠠⠃⠴⠀⠌⠠⠏⠦⠠⠁⠴")?;
    return Ok(());
}

#[test]
fn p50_alternative_derivative_1() -> Result<()> {
    let expr = "<math>
  <mfrac>
    <mrow>
      <mi>d</mi>
      <mi>ln</mi>
      <mi>x</mi>
    </mrow>
    <mrow>
      <mi>d</mi>
      <mi>x</mi>
    </mrow>
  </mfrac>
</math>";
    test_braille("Finnish", expr, "⠙⠇⠝⠀⠭⠀⠌⠙⠭")?;
    return Ok(());
}

// Not sure about this MathML markup.
#[test]
fn p50_alternative_derivative_2() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mfrac>
      <mi>d</mi>
      <mrow>
        <mi>d</mi>
        <mi>x</mi>
      </mrow>
    </mfrac>
    <mi>ln</mi>
    <mi>x</mi>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠙⠀⠌⠙⠭⠀⠇⠝⠀⠭")?;
    return Ok(());
}

#[test]
fn p51_set_with_closure() -> Result<()> {
    let expr = "<math>
<mi>𝜕</mi>
<mi>A</mi> 
<mo>=</mo>
<mover>
<mi>A</mi>
<mo>‾</mo>
</mover>
<mo>∩</mo>
<mover>
<mrow>
<mo>(</mo>
<mi>X</mi>
<mo>∖</mo>
<mi>A</mi>
<mo>)</mo>
</mrow>
<mo>‾</mo>
</mover>
</math>";
    test_braille("Finnish", expr, "⠈⠙⠠⠁⠀⠶⠠⠁⠱⠀⠳⠦⠦⠠⠭⠀⠳⠡⠠⠁⠴⠱")?;
    return Ok(());
}

#[test]
fn p51_normal_distribution() -> Result<()> {
    let expr = "<math>
      <mrow>
        <mi>p</mi>
        <mo>~</mo>
        <mi>N</mi>
        <mo>(</mo>
        <mn>58</mn>
        <mo>,</mo>
        <mn>2</mn>
        <mo>)</mo>
      </mrow>
    </math>";
    test_braille("Finnish", expr, "⠏⠀⠨⠶⠠⠝⠦⠼⠑⠓⠂⠀⠼⠃⠴")?;
    return Ok(());
}

#[test]
fn p52_right_circular_cone_volume() -> Result<()> {
  let expr = "<math>
    <mi>V</mi>
    <mo>=</mo>
  <mrow>
    <mfrac>
      <mn>1</mn>
      <mn>3</mn>
    </mfrac>
    <mi>π</mi>
    <msup>
      <mi>r</mi>
      <mn>2</mn>
    </msup>
    <mi>h</mi>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠠⠧⠀⠶⠼⠁⠒⠀⠨⠏⠀⠗⠬⠀⠓")?;
    return Ok(());
}

#[test]
fn p52_circular_cone_area_equation() -> Result<()> {
    let expr = "<math>
    <mi>A</mi>
    <mo>=</mo>
    <mi>π</mi>
    <msup>
      <mi>r</mi>
      <mn>2</mn>
    </msup>
    <mo>+</mo>
  <mrow>
    <mi>π</mi>
    <mi>r</mi>
    <msqrt>
      <mrow>
        <msup>
          <mi>r</mi>
          <mn>2</mn>
        </msup>
        <mo>+</mo>
        <msup>
          <mi>h</mi>
          <mn>2</mn>
        </msup>
      </mrow>
    </msqrt>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠠⠁⠀⠶⠨⠏⠀⠗⠬⠀⠖⠨⠏⠀⠗⠀⠩⠦⠗⠬⠀⠖⠓⠬⠴")?;
    return Ok(());
}

#[test]
fn p52_quadratic_formula() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mi>x</mi>
    <mo>=</mo>
    <mfrac>
      <mrow>
        <mo>−</mo>
        <mi>b</mi>
        <mo>±</mo>
        <msqrt>
          <mrow>
            <msup>
              <mi>b</mi>
              <mn>2</mn>
            </msup>
            <mo>−</mo>
            <mn>4</mn>
            <mi>a</mi>
            <mi>c</mi>
          </mrow>
        </msqrt>
      </mrow>
      <mrow>
        <mn>2</mn>
        <mi>a</mi>
      </mrow>
    </mfrac>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠭⠀⠶⠦⠤⠃⠀⠖⠤⠩⠦⠃⠬⠀⠤⠼⠙⠀⠁⠉⠴⠴⠀⠌⠦⠼⠃⠀⠁⠴")?;
    return Ok(());
}

// Sub- and superscripts are used before the atom sign. 
#[test]
fn p63_O16() -> Result<()> {
    // From MathType
    let expr = r#"<math><mmultiscripts><mi mathvariant="normal">O</mi><mprescripts/><mn>8</mn><mn>16</mn></mmultiscripts></math>"#;
    test_braille("Finnish", expr, "⠡⠼⠓⠬⠼⠁⠋⠠⠕")?;
    return Ok(());
}

// The numbers are written as 'dropped dots' (like in fractions) right after the atom. No whitespace between the atoms.
#[test]
fn p64_chemical_equation_1() -> Result<()> {
    // From MathType
    let expr = "<math><msub><mtext>C</mtext><mn>2</mn></msub><msub><mtext>H</mtext><mn>5</mn></msub><mtext>OH</mtext></math>";
    test_braille("Finnish", expr, "⠠⠉⠆⠠⠓⠢⠠⠕⠠⠓")?;
    return Ok(());
}

#[test]
fn p64_sodium_ion() -> Result<()> {
    let expr = "<math><msup><mi>Na</mi><mo>+</mo></msup></math>";
    test_braille("Finnish", expr, "⠠⠝⠁⠬⠖")?;
    return Ok(());
}

// Whitespace before the ion's plus or minus sign if there is a number before it. Parentheses for clarity.
#[test]
fn p64_copper_ion() -> Result<()> {
    // From MathType
    let expr = "<math><msup><mrow><mtext>Cu</mtext></mrow><mrow><mn>2</mn><mo>+</mo></mrow></msup></math>";
    test_braille("Finnish", expr, "⠠⠉⠥⠬⠦⠼⠃⠀⠖⠴")?;
    return Ok(());
}

#[test]
fn p64_chemistry_1() -> Result<()> {
    let expr = "<math><msubsup><mrow><mi>S</mi><mi>O</mi></mrow><mn>4</mn><mrow><mn>2</mn><mo>-</mo></mrow></msubsup></math>";
    test_braille("Finnish", expr, "⠠⠎⠠⠕⠲⠬⠦⠼⠃⠀⠤⠴")?;
    return Ok(());
}

// Whitespace before the dash, not after.
#[test]
fn p64_chemistry_bond_1() -> Result<()> {
    let expr = "<math intent=':chemical-formula'><mi>H</mi><mo>-</mo><mi>H</mi></math>";
    test_braille("Finnish", expr, "⠠⠓⠀⠒⠠⠓")?;
    return Ok(());
}

// Whitespace before the colon, not after.
#[test]
fn p64_chemistry_bond_1_alternative() -> Result<()> {
    let expr = "<math intent=':chemical-formula'><mi>H</mi><mo>:</mo><mi>H</mi></math>";
    test_braille("Finnish", expr, "⠠⠓⠀⠆⠠⠓")?;
    return Ok(());
}

// Whitespace before the dashes, not after.
#[test]
fn p64_chemistry_bond_2() -> Result<()> {
    let expr = "<math intent=':chemical-formula'><mi>O</mi><mo>=</mo><mi>O</mi></math>";
    test_braille("Finnish", expr, "⠠⠕⠀⠒⠒⠠⠕")?;
    return Ok(());
}

// Whitespace before the double colon, not after.
#[test]
fn p64_chemistry_bond_2_alternative() -> Result<()> {
    let expr = "<math intent=':chemical-formula'><mi>O</mi><mo>∷</mo><mi>O</mi></math>";
    test_braille("Finnish", expr, "⠠⠕⠀⠶⠠⠕")?;
    return Ok(());
}

// Whitespace before the dashes, not after.
#[test]
fn p64_chemistry_bond_3() -> Result<()> {
    let expr = "<math intent=':chemical-formula'><mi>N</mi><mo>≡</mo><mi>N</mi></math>";
    test_braille("Finnish", expr, "⠠⠝⠀⠒⠒⠒⠠⠝")?;
    return Ok(());
}

// The specs use different parentheses for atomic groups. Dots (126) for left and dots (345) for right parenthesis.
// For MathCAT these are not used, since making the rule would probably be difficult. The parentheses that are used in the MathML are also output by MathCAT.
#[test]
fn chemical_formula_parentheses_1() -> Result<()> {
    let expr = "<math>
  <mi>Ca</mi>
  <msub>
    <mrow>
    <mo>(</mo>
    <mi>O</mi>
    <mi>H</mi>
    <mo>)</mo>
    </mrow>
  <mn>2</mn>
  </msub>
</math>";
    test_braille("Finnish", expr, "⠠⠉⠁⠦⠠⠕⠠⠓⠴⠆")?;
    return Ok(());
}

// Here the zone ending dot (5) is needed, so parenthesis are not confused with the 'dropped dots'.
#[test]
fn chemical_formula_parentheses_2() -> Result<()> {
    let expr = "<math>
  <msub>
    <mi>Ca</mi>
    <mn>3</mn>
  </msub>
  <msub>
    <mrow>
    <mo>(</mo>
    <mi>P</mi>
      <msub>
      <mi>O</mi>
      <mn>4</mn>
      </msub>
    <mo>)</mo>
    </mrow>
  <mn>2</mn>
  </msub>
</math>";
    test_braille("Finnish", expr, "⠠⠉⠁⠒⠦⠠⠏⠠⠕⠲⠐⠴⠆")?;
    return Ok(());
}

// New tests start here 11.9.2026

#[test]
fn approx_1() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mn>230</mn>
    <mo>≈</mo>
    <mn>200</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠉⠚⠀⠸⠶⠼⠃⠚⠚")?;
    return Ok(());
}

#[test]
fn approx_2() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mn>80</mn>
    <mo>+</mo>
    <mn>60</mn>
    <mo>≈</mo>
    <mn>150</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠓⠚⠀⠖⠼⠋⠚⠀⠸⠶⠼⠁⠑⠚")?;
    return Ok(());
}

#[test]
fn large_numbers_1() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mn>20.000</mn>
    <mo>+</mo>
    <mn>3000</mn>
    <mo>=</mo>
    <mn>23.000</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠚⠄⠚⠚⠚⠀⠖⠼⠉⠚⠚⠚⠀⠶⠼⠃⠉⠄⠚⠚⠚")?;
    return Ok(());
}

#[test]
fn large_numbers_2() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mn>88.888</mn>
    <mo>−</mo>
    <mn>8080</mn>
    <mo>=</mo>
    <mn>80.808</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠓⠓⠄⠓⠓⠓⠀⠤⠼⠓⠚⠓⠚⠀⠶⠼⠓⠚⠄⠓⠚⠓")?;
    return Ok(());
}

#[test]
fn comparison_multiple_numbers() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mn>1000</mn>
    <mo>&lt;</mo>
    <mn>9000</mn>
    <mo>&lt;</mo>
    <mn>10.000</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠁⠚⠚⠚⠀⠣⠀⠼⠊⠚⠚⠚⠀⠣⠀⠼⠁⠚⠄⠚⠚⠚")?;
    return Ok(());
}

#[test]
fn division_slash_1() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mn>9</mn>
    <mo>/</mo>
    <mn>3</mn>
    <mo>=</mo>
    <mn>3</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠊⠀⠌⠼⠉⠀⠶⠼⠉")?;
    return Ok(());
}

#[test]
fn division_slash_2() -> Result<()> {
    let expr = "<math>
  <mrow>
    <mn>100</mn>
    <mo>/</mo>
    <mn>2</mn>
    <mo>=</mo>
    <mn>50</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠁⠚⠚⠀⠌⠼⠃⠀⠶⠼⠑⠚")?;
    return Ok(());
}

#[test]
fn units_grams() -> Result<()> {
    let expr = "<math>
  <mn>380</mn>
  <mi>g</mi>
</math>";
    test_braille("Finnish", expr, "⠼⠉⠓⠚⠀⠛")?;
    return Ok(());
}

#[test]
fn units_kilograms_and_grams() -> Result<()> {
    let expr = "<math>
    <mn>4</mn>
    <mi>kg</mi>
    <mo>−</mo>
    <mn>150</mn>
    <mi>g</mi>
    <mo>=</mo>
    <mn>3,850</mn>
    <mi>kg</mi>
</math>";
    test_braille("Finnish", expr, "⠼⠙⠀⠅⠛⠀⠤⠼⠁⠑⠚⠀⠛⠀⠶⠼⠉⠂⠓⠑⠚⠀⠅⠛")?;
    return Ok(());
}

// Invisible plus is replaced with the number sign (dots 3456) in a mixed fraction.
#[test]
fn invisible_plus_with_mixed_fraction_1() -> Result<()> {
    let expr = "<math>
    <mn>1</mn>
    <mo>&#x2064;</mo>
    <mfrac>
      <mn>1</mn>
      <mn>2</mn>
    </mfrac>
</math>";
    test_braille("Finnish", expr, "⠼⠁⠼⠁⠆")?;
    return Ok(());
}

// Invisible plus is replaced with the number sign (dots 3456) in a mixed fraction.
#[test]
fn invisible_plus_with_mixed_fraction_2() -> Result<()> {
    let expr = "<math>
    <mn>1</mn>
    <mo>&#x2064;</mo>
    <mfrac>
      <mn>1</mn>
      <mn>2</mn>
    </mfrac>
    <mo>−</mo>
    <mn>2</mn>
    <mo>&#x2064;</mo>
    <mfrac>
      <mn>1</mn>
      <mn>3</mn>
    </mfrac>
</math>";
    test_braille("Finnish", expr, "⠼⠁⠼⠁⠆⠀⠤⠼⠃⠼⠁⠒")?;
    return Ok(());
}

// The same solution is used for counting in steps as is in the matrices. A custom [space][dot 5][dot 5][space] separates rows from each other.
#[test]
fn count_with_steps() -> Result<()> {
    let expr = "<math>
  <mtable>
    <mtr>
      <mtd>
        <mo>(</mo>
        <mn>10</mn>
        <mo>−</mo>
        <mn>6</mn>
        <mo>)</mo>
        <mo>&#x22C5;</mo>
        <mo>(</mo>
        <mn>5</mn>
        <mo>+</mo>
        <mn>5</mn>
        <mo>)</mo>
      </mtd>
    </mtr>
    <mtr>
      <mtd>
        <mo>=</mo>
        <mn>4</mn>
        <mo>&#x22C5;</mo>
        <mn>10</mn>
      </mtd>
    </mtr>
    <mtr>
      <mtd>
        <mo>=</mo>
        <mn>40</mn>
      </mtd>
    </mtr>
  </mtable>
</math>";
    test_braille("Finnish", expr, "⠦⠼⠁⠚⠀⠤⠼⠋⠴⠀⠄⠦⠼⠑⠀⠖⠼⠑⠴⠀⠐⠐⠀⠶⠼⠙⠀⠄⠼⠁⠚⠀⠐⠐⠀⠶⠼⠙⠚")?;
    return Ok(());
}