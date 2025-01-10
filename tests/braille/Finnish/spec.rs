// The tests are based on the Finnish specification for 6 dot math braille on the braille authority's web page (https://www.pistekirjoitus.fi/julkaisut/matematiikka-ja-tietotekniikka/) titled "Matematiikan, fysiikan ja kemain pistemerkinnät". Roughly translates to "Braille for mathematics, physics and chemistry." These tests are based on the edition published in 2022.
//
// Changes to the specifications in the rules and tests in MathCAT
// Some changes have been made to the rules and these tests test against the modified rules. The changes have been made, because the specification is for printed braille and intended for people authoring mathematics braille. Some things have been changed to be consistent in all situations and to work in the braille display context.

// UEB tests for the basic mathml tags
// Initial tests are from BANA guidelines, mostly about initial chars for code switching
//   http://www.brailleauthority.org/ueb/ueb_math_guidance/final_for_posting_ueb_math_guidance_may_2019_102419.pdf
// These tests start with "bana_"
//
// Many come from (refer to) https://iceb.org/guidelines_for_technical_material_2014.pdf
// For example, "fraction_6_1_1" is a fraction example from section 6.1, and is the first example there.
use crate::common::*;

// Original UEB tests (not translated yet)

#[test]
fn chemistry_2_4() {
    // From MathType
    let expr = "<math><msub><mtext>C</mtext><mn>2</mn></msub><msub><mtext>H</mtext><mn>5</mn></msub><mtext>OH</mtext></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn chemistry_2_8() {
    // From MathType
    let expr = "<math><msup><mrow><mtext>Cu</mtext></mrow><mrow><mn>2</mn><mo>+</mo></mrow></msup></math>";
    test_braille("Finnish", expr, "");
}

// Finnish spec tests
// Page and some other identification has been added to the function name, so you can figure out which expression the test in based on. Some tests are variations of the "official" test.

// Grouping numbers

// No example in the specs
#[test]
fn p7_no_grouping_in_four_digit_numbers() {
    let expr = "<math><mn>2000</mn></math>";
    test_braille("Finnish", expr, "⠼⠃⠚⠚⠚");
}

#[test]
fn p7_thousands_nbsp() {
    let expr = "<math><mn>2 000 000</mn></math>";
    test_braille("Finnish", expr, "⠼⠃⠄⠚⠚⠚⠄⠚⠚⠚");
}

#[test]
fn p7_thousands_point() {
    let expr = "<math><mn>2.000.000</mn></math>";
    test_braille("Finnish", expr, "⠼⠃⠄⠚⠚⠚⠄⠚⠚⠚");
}

#[test]
fn p7_thousands_space() {
    let expr = "<math><mn>2 000 000</mn></math>";
    test_braille("Finnish", expr, "⠼⠃⠄⠚⠚⠚⠄⠚⠚⠚");
}

#[test]
fn p7_decimal_comma() {
    let expr = "<math><mn>5,12575</mn></math>";
    test_braille("Finnish", expr, "⠼⠑⠂⠁⠃⠑⠄⠛⠑");
}

// Now the point is transformed to comma. Maybe it should follow the text.
#[test]
fn p7_decimal_point() {
    let expr = "<math><mn>5.12575</mn></math>";
    test_braille("Finnish", expr, "⠼⠑⠂⠁⠃⠑⠄⠛⠑");
}

#[test]
fn p7_decimal_point_endless_no_grouping() {
    let expr = "<math><mn>1,234657234...</mn></math>";
    test_braille("Finnish", expr, "⠼⠁⠂⠃⠉⠙⠄⠋⠑⠛⠄⠃⠉⠙⠄⠄⠄");
}

#[test]
fn p7_decimal_point_endless_with_grouping_space() {
    let expr = "<math><mn>1,234 657 234...</mn></math>";
    test_braille("Finnish", expr, "⠼⠁⠂⠃⠉⠙⠄⠋⠑⠛⠄⠃⠉⠙⠄⠄⠄");
}

#[test]
fn p7_decimal_point_endless_with_grouping_nbsp() {
    let expr = "<math><mn>1,234 657 234...</mn></math>";
    test_braille("Finnish", expr, "⠼⠁⠂⠃⠉⠙⠄⠋⠑⠛⠄⠃⠉⠙⠄⠄⠄");
}

#[test]
fn p7_decimal_point_endless_with_grouping_nbsp_ellipses() {
    let expr = "<math><mn>1,234 657 234…</mn></math>";
    test_braille("Finnish", expr, "⠼⠁⠂⠃⠉⠙⠄⠋⠑⠛⠄⠃⠉⠙⠄⠄⠄");
}

#[test]
fn p7_decimal_point_endless_no_grouping_ellipses() {
    let expr = "<math><mn>1,234657234…</mn></math>";
    test_braille("Finnish", expr, "⠼⠁⠂⠃⠉⠙⠄⠋⠑⠛⠄⠃⠉⠙⠄⠄⠄");
}

#[test]
fn p7_decimal_point_endless_with_grouping_space_ellpses() {
    let expr = "<math><mn>1,234 657 234…</mn></math>";
    test_braille("Finnish", expr, "⠼⠁⠂⠃⠉⠙⠄⠋⠑⠛⠄⠃⠉⠙⠄⠄⠄");
}

#[test]
fn p9_units_percent() {
    let expr = "<math><mn>9</mn><mi>%</mi></math>";
    test_braille("Finnish", expr, "⠼⠊⠊⠀⠹");
}

#[test]
fn p9_units_permille() {
    let expr = "<math><mn>9</mn><mi>‰</mi></math>";
    test_braille("Finnish", expr, "⠼⠁⠁⠑⠀⠒⠹");
}

#[test]
fn p9_units_degrees() {
    let expr = "<math><mn>100</mn><mi>˚</mi></math>";
    test_braille("Finnish", expr, "⠼⠁⠚⠚⠀⠴");
}

#[test]
fn p9_units_degrees_celsius() {
    let expr = "<math><mn>37</mn><mi>˚</mi><mi>C</mi></math>";
    test_braille("Finnish", expr, "⠼⠉⠛⠀⠴⠠⠉");
}

#[test]
fn p9_units_degrees_fahrenheit() {
    let expr = "<math><mn>−43</mn><mi>˚</mi><mi>F</mi></math>";
    test_braille("Finnish", expr, "⠤⠼⠙⠉⠀⠴⠠⠋");
}

#[test]
fn p10_currency_euro() {
    let expr = "<math><mn>6</mn><mi>€</mi><mn>15</mn><mi>snt</mi></math>";
    test_braille("Finnish", expr, "⠼⠋⠀⠈⠑⠀⠼⠁⠑⠀⠎⠝⠞");
}

#[test]
fn p10_currency_dollar() {
    let expr = "<math><mi>$</mi><mn>25</mn></math>";
    test_braille("Finnish", expr, "⠠⠙⠼⠉⠑");
}

#[test]
fn p10_currency_pound() {
    let expr = "<math><mi>£</mi><mn>25</mn></math>";
    test_braille("Finnish", expr, "⠇⠼⠃⠑");
}

#[test]
fn p11_mm_squared() {
    let expr = "<math><msup><mi>mm</mi><mn>2</mn></msup></math>";
    test_braille("Finnish", expr, "⠍⠍⠬");
}

#[test]
fn p11_cm_cubed() {
    let expr = "<math><msup><mi>cm</mi><mn>3</mn></msup></math>";
    test_braille("Finnish", expr, "⠉⠍⠬⠼⠉");
}

#[test]
fn p13_plus() {
    let expr = "<math><mn>3</mn><mo>+</mo><mn>4</mn><mo>=</mo><mn>7</mn></math>";
    test_braille("Finnish", expr, "⠼⠉⠀⠖⠼⠙⠀⠶⠼⠛");
}

#[test]
fn p13_minus_not_equal() {
    let expr = "<math><mn>5</mn><mo>−</mo><mn>4</mn><mo>≠</mo><mn>2</mn></math>";
    test_braille("Finnish", expr, "⠼⠑⠀⠤⠼⠃⠀⠐⠶⠼⠃");
}

// Question mark is the same as in literary text.
#[test]
fn p13_times_with_question_mark() {
    let expr = "<math><mn>27</mn><mo>·</mo><mn>3</mn><mo>=</mo><mi>?</mi></math>";
    test_braille("Finnish", expr, "⠼⠃⠛⠀⠄⠼⠉⠀⠶⠢");
}

#[test]
fn p13_parentheses_invisible_times() {
    let expr = "<math><mo>(</mo><mn>3</mn><mo>&#8290;</mo><mi>x</mi><mo>+</mo><mn>2</mn><mo>&#8290;</mo><mi>y</mi><mo>)</mo><mo>&#8290;</mo><mo>(</mo><mn>2</mn><mo>&#8290;</mo><mi>x</mi><mo>−</mo><mn>1</mn><mo>)</mo></math>";
    test_braille("Finnish", expr, "⠦⠼⠉⠀⠭⠀⠖⠼⠃⠀⠽⠴⠀⠦⠼⠃⠀⠭⠀⠤⠼⠁⠴");
}

#[test]
fn p15_less_than() {
    let expr = "<math>
    <mi>x</mi>
    <mo>&lt;</mo>
    <mn>18</mn>
</math>";
    test_braille("Finnish", expr, "⠭⠀⠣⠀⠼⠁⠓");
}

#[test]
fn p15_greater_than() {
    let expr = "<math>
    <mn>2</mn>
    <mi>x</mi>
    <mo>&gt;</mo>
    <mn>6</mn>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠀⠭⠀⠱⠀⠼⠋");
}

#[test]
fn p15_greater_than_or_equal_to() {
    let expr = "<math>
    <mn>2</mn>
    <mi>x</mi>
    <mo>≥</mo>
    <mn>6</mn>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠀⠭⠀⠱⠶⠀⠼⠋");
}

#[test]
fn p15_less_than_or_equal_to() {
    let expr = "<math>
    <mi>x</mi>
    <mo>≤</mo>
    <mn>18</mn>
</math>";
    test_braille("Finnish", expr, "⠭⠀⠣⠶⠀⠼⠁⠓");
}

// Fractions with 'dropped dots'.
#[test]
fn p15_fractions_minus() {
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
  </mrow>
  <mrow>
    <mo>(</mo>
    <mfrac>
      <mn>2</mn>
      <mn>5</mn>
    </mfrac>
    <mo>−</mo>
    <mfrac>
      <mn>1</mn>
      <mn>3</mn>
    </mfrac>
    <mo>)</mo>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠦⠼⠁⠼⠉⠢⠀⠤⠼⠑⠶⠠⠴⠀⠤⠦⠼⠃⠢⠀⠖⠼⠁⠒⠠⠴");
}

#[test]
fn p16_division() {
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
    test_braille("Finnish", expr, "⠦⠼⠋⠀⠭⠀⠖⠼⠉⠀⠭⠴⠀⠌⠼⠉");
}

#[test]
fn p16_fractional() {
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
    test_braille("Finnish", expr, "⠦⠼⠋⠀⠭⠀⠖⠼⠉⠀⠭⠴⠀⠌⠼⠉");
}

#[test]
fn p16_absolute_value() {
    let expr = "<math>
  <mrow>
    <mi>|</mi>
    <mo>−</mo>
  </mrow>
  <mrow>
    <mo>(</mo>
    <mn>2</mn>
    <mo>+</mo>
    <mn>3</mn>
    <mo>)</mo>
    <mi>|</mi>
    <mo>=</mo>
  </mrow>
  <mrow>
    <mi>|</mi>
    <mo>−</mo>
  </mrow>
  <mrow>
    <mn>7</mn>
    <mi>|</mi>
    <mo>=</mo>
  </mrow>
  <mrow>
    <mn>7</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠸⠤⠦⠼⠃⠀⠖⠼⠑⠴⠸⠀⠶⠸⠤⠼⠛⠸⠀⠶⠼⠛");
}

#[test]
fn p16_parens_and_brackets() {
    let expr = "<math>
  <mrow>
    <mn>92</mn>
    <mo>+</mo>
  </mrow>
  <mrow>
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
    test_braille("Finnish", expr, "⠼⠊⠃⠀⠖⠷⠤⠼⠃⠀⠄⠦⠼⠁⠓⠀⠖⠼⠑⠴⠾");
}

#[test]
fn p16_sequence_of_natural_numbers() {
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
    test_braille("Finnish", expr, "⠠⠝⠀⠶⠫⠼⠚⠂⠀⠼⠁⠂⠀⠼⠃⠂⠀⠼⠉⠂⠀⠄⠄⠄⠻");
}

#[test]
fn p17_multiple_operations() {
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
    test_braille("Finnish", expr, "⠼⠑⠀⠄⠼⠉⠬⠀⠖⠦⠼⠓⠁⠀⠤⠼⠋⠚⠴⠀⠌⠼⠉⠀⠤⠦⠼⠃⠀⠖⠼⠙⠴⠬");
}

#[test]
fn p19_fractions_sum() {
    let expr = "<math>
  <mrow>
    <mfrac>
      <mn>3</mn>
      <mn>4</mn>
    </mfrac>
    <mo>+</mo>
  </mrow>
  <mrow>
    <mfrac>
      <mn>1</mn>
      <mn>4</mn>
    </mfrac>
    <mo>=</mo>
  </mrow>
  <mrow>
    <mn>1</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠉⠲⠀⠖⠼⠁⠲⠀⠶⠼⠁");
}

#[test]
fn p19_fractions_mixed_operations() {
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
  </mrow>
  <mrow>
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
    test_braille("Finnish", expr, "⠦⠼⠉⠲⠀⠖⠼⠑⠖⠠⠴⠀⠤⠦⠼⠉⠢⠀⠖⠼⠁⠆⠠⠴");
}

// This is Finnish notation where the numerator and denomator is multiplied by the same expression that is in the pre-supercript. Structure is always: [multiplicator][parenthesis]
#[test]
fn p19_fraction_multiplication_Finnish_notation_multiscripts() {
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
  </mrow>
  <mrow>
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
    test_braille("Finnish", expr, "⠃⠜⠀⠁⠀⠌⠉⠀⠶⠁⠃⠀⠌⠃⠉");
}

#[test]
fn p19_fraction_multiplication_Finnish_notation_multiscripts_2() {
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
    test_braille("Finnish", expr, "⠼⠋⠜⠼⠁⠆");
}

// This is Finnish notation for dividing the numerator and denominator by the same expression. It is always of the form: [parenthesis][divisor]
#[test]
fn p22_fraction_division_Finnish_notation_msup() {
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
  </mrow>
  <mrow>
    <mfrac>
      <mrow>
        <mi>b</mi>
        <mi>c</mi>
      </mrow>
      <mi>d</mi>
    </mfrac>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠁⠃⠉⠀⠌⠁⠙⠀⠣⠁⠀⠶⠃⠉⠀⠌⠙");
}

#[test]
fn p22_fraction_division_Finnish_notation_msup_2() {
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
    test_braille("Finnish", expr, "⠼⠃⠲⠣⠼⠃");
}

#[test]
fn p23_equation_1_no_invisible_multiplication() {
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
    test_braille("Finnish", expr, "⠼⠃⠀⠁⠉⠀⠌⠦⠼⠃⠀⠙⠴⠀⠣⠼⠃⠀⠶⠁⠉⠀⠌⠙");
}

#[test]
fn p23_equation_1_with_invisible_multiplication() {
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
    test_braille("Finnish", expr, "⠼⠃⠀⠁⠉⠀⠌⠦⠼⠃⠀⠙⠴⠀⠣⠼⠃⠀⠶⠁⠉⠀⠌⠙");
}

#[test]
fn p23_equation_solving_notation() {
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
    test_braille("Finnish", expr, "⠼⠑⠀⠭⠀⠶⠤⠼⠁⠚⠀⠀⠸⠀⠌⠼⠑");
}

#[test]
fn p24_fraction() {
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
    test_braille("Finnish", expr, "⠼⠉⠀⠽⠀⠌⠼⠙⠀⠶⠼⠉⠲⠀⠽");
}

#[test]
fn p24_fraction_2() {
    let expr = "<math>
  <mfrac>
    <mrow>
      <mn>4</mn>
      <mo>&#8290;</mo>
      <mi>x</mi>
    </mrow>
    <mrow>
      <mn>2</mn>
      <mo>&#8290;</mo>
      <mo>(</mo>
      <mn>1</mn>
      <mo>−</mo>
      <mi>x</mi>
      <mo>)</mo>
    </mrow>
  </mfrac>
</math>";
    test_braille("Finnish", expr, "⠼⠙⠀⠭⠀⠌⠦⠼⠋⠀⠦⠼⠁⠀⠤⠭⠴⠴");
}

#[test]
fn p25_long_fraction() {
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
  </mrow>
  <mrow>
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
    test_braille("Finnish", expr, "⠦⠦⠭⠬⠀⠤⠼⠛⠀⠭⠀⠖⠼⠁⠃⠴⠀⠌⠦⠼⠙⠀⠭⠀⠤⠼⠃⠚⠴⠴⠀⠌⠦⠦⠭⠬⠀⠤⠼⠓⠀⠭⠀⠖⠼⠁⠑⠴⠀⠌⠦⠼⠙⠀⠭⠀⠤⠼⠁⠋⠴⠴");
}

#[test]
fn p25_fraction_in_fraction() {
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
    test_braille("Finnish", expr, "⠦⠼⠁⠆⠀⠖⠼⠁⠒⠠⠴⠀⠌⠦⠼⠁⠲⠀⠤⠼⠁⠢⠠⠴");
}

#[test]
fn p26_exponent_multiplication_with_parens() {
    let expr = "<math>
  <mrow>
    <msup>
      <mn>2</mn>
      <mn>3</mn>
    </msup>
    <mo>⋅</mo>
  </mrow>
  <mrow>
    <msup>
      <mn>2</mn>
      <mn>5</mn>
    </msup>
    <mo>=</mo>
  </mrow>
  <mrow>
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
  </mrow>
  <mrow>
    <msup>
      <mn>2</mn>
      <mn>8</mn>
    </msup>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠬⠼⠉⠀⠄⠼⠃⠬⠼⠑⠀⠶⠼⠃⠬⠦⠼⠉⠀⠖⠼⠑⠴⠀⠶⠼⠃⠬⠼⠓");
}

#[test]
fn p26_exponent_multiplication_no_parens() {
    let expr = "<math>
  <mrow>
    <msup>
      <mn>2</mn>
      <mn>3</mn>
    </msup>
    <mo>⋅</mo>
  </mrow>
  <mrow>
    <msup>
      <mn>2</mn>
      <mn>5</mn>
    </msup>
    <mo>=</mo>
  </mrow>
  <mrow>
    <msup>
      <mn>2</mn>
      <mrow>
        <mn>3</mn>
        <mo>+</mo>
        <mn>5</mn>
      </mrow>
    </msup>
    <mo>=</mo>
  </mrow>
  <mrow>
    <msup>
      <mn>2</mn>
      <mn>8</mn>
    </msup>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠬⠼⠉⠀⠄⠼⠃⠬⠼⠑⠀⠶⠼⠃⠬⠦⠼⠉⠀⠖⠼⠑⠴⠀⠶⠼⠃⠬⠼⠓");
}

#[test]
fn p27_exponent_to_large_number() {
    let expr = "<math>
  <mrow>
    <msup>
      <mn>2</mn>
      <mn>20</mn>
    </msup>
    <mo>=</mo>
  </mrow>
  <mrow>
    <mn>1048576</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠃⠬⠼⠃⠚⠀⠶⠼⠁⠄⠚⠙⠓⠄⠑⠛⠋");
}

#[test]
fn p27_sqrt() {
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
  </mrow>
  <mrow>
    <msqrt>
      <mn>4</mn>
    </msqrt>
    <mo>=</mo>
  </mrow>
  <mrow>
    <mn>2</mn>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠼⠙⠬⠼⠁⠰⠀⠶⠩⠼⠙⠀⠶⠼⠃");
}

#[test]
fn p31_trig_tan_infinity() {
    let expr = "<math>
      <mi>tan</mi>
      <mo>⁡</mo>
      <mn>90</mn>
      <mi>°</mi>
      <mo>=</mo>
      <mi>∞</mi>
</math>";
    test_braille("Finnish", expr, "⠞⠁⠝⠀⠼⠊⠚⠀⠴⠀⠶⠿");
}

// This would probably need alternative markup and test case as well.
#[test]
fn p31_degrees_minutes_seconds() {
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
    test_braille("Finnish", expr, "⠼⠉⠚⠀⠴⠀⠼⠙⠑⠀⠨⠀⠼⠃⠚⠀⠸");
}

#[test]
fn p32_expected_value() {
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
    test_braille("Finnish", expr, "⠠⠑⠦⠭⠴⠀⠶⠨⠍⠀⠶⠸⠎⠡⠊⠐⠏⠡⠊⠐⠭⠡⠊");
}

// Dots 156 to signify the bar over "AB"
#[test]
fn p33_vector() {
    let expr = "<math>
  <mrow>
    <mover>
        <mi>AB</mi>
        <mo>‾</mo>
    </mover>
    <mo>=</mo>
  </mrow>
  <mrow>
    <mn>3</mn>
    <mover>
      <mi>i</mi>
      <mo>‾</mo>
    </mover>
    <mo>−</mo>
  </mrow>
  <mrow>
    <mn>4</mn>
    <mover>
      <mi>j</mi>
      <mo>‾</mo>
    </mover>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠠⠁⠠⠃⠱⠀⠶⠼⠉⠀⠊⠱⠀⠤⠼⠙⠀⠚⠱");
}

#[test]
fn p33_parallel_vectors() {
    let expr = "<math>
  <mrow>
    <mover>
      <mi>a</mi>
      <mo>‾</mo>
    </mover>
    <mo>∥</mo>
  </mrow>
  <mrow>
    <mover>
      <mi>b</mi>
      <mo>‾</mo>
    </mover>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠁⠱⠀⠸⠸⠀⠃⠱");
}

#[test]
fn p34_vector_dot_product() {
    let expr = "<math>
  <mrow>
    <mover>
      <mi>v</mi>
      <mo>‾</mo>
    </mover>
    <mo>⋅</mo>
  </mrow>
  <mrow>
    <mover>
      <mi>u</mi>
      <mo>‾</mo>
    </mover>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠧⠱⠀⠠⠥⠱");
}

#[test]
fn p34_vector_cross_product() {
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
    test_braille("Finnish", expr, "⠁⠱⠀⠰⠄⠃⠱");
}

#[test]
fn p39_not_in_set() {
    let expr = "<math>
    <mn>3</mn>
    <mo>∉</mo>
    <mi>B</mi>
</math>";
    test_braille("Finnish", expr, "⠼⠉⠀⠳⠐⠔⠠⠃");
}

#[test]
fn p39_subset() {
    let expr = "<math>
  <mrow>
    <mi>B</mi>
    <mo>⊂</mo>
  </mrow>
  <mrow>
    <mi>A</mi>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠠⠃⠀⠳⠪⠠⠁");
}

#[test]
fn p39_not_subset() {
    let expr = "<math>
    <mi>F</mi>
    <mo>⊄</mo>
    <mi>E</mi>
</math>";
    test_braille("Finnish", expr, "⠠⠋⠀⠳⠐⠪⠠⠑");
}

#[test]
fn p39_equal_sets_with_mtext() {
    let expr = "<math>
  <mrow>
    <mi>C</mi>
    <mo>⊆</mo>
  </mrow>
  <mrow>
    <mi>D</mi>
    <mtext> ja </mtext>
    <mi>D</mi>
    <mo>⊆</mo>
  </mrow>
  <mrow>
    <mi>C</mi>
    <mo>⇔</mo>
  </mrow>
  <mrow>
    <mi>C</mi>
    <mo>=</mo>
  </mrow>
  <mrow>
    <mi>D</mi>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠠⠉⠀⠳⠶⠪⠠⠙⠀⠚⠁⠀⠠⠙⠀⠳⠶⠪⠠⠉⠀⠣⠤⠤⠱⠀⠠⠉⠀⠶⠠⠙");
}

#[test]
fn p39_set_of_points_in_R2() {
    let expr = "<math>
  <mrow>
    <mi>A</mi>
    <mo>×</mo>
  </mrow>
  <mrow>
    <mi>B</mi>
    <mo>=</mo>
  </mrow>
  <mrow>
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
    test_braille("Finnish", expr, "⠠⠁⠀⠰⠄⠠⠃⠀⠶⠫⠦⠼⠁⠂⠀⠼⠉⠴⠂⠀⠦⠼⠁⠂⠀⠼⠙⠴⠂⠀⠦⠼⠃⠂⠀⠼⠉⠴⠂⠀⠦⠼⠃⠂⠀⠼⠙⠴⠻");
}

#[test]
fn p40_not_p() {
    let expr = "<math>
    <mo>¬</mo>
    <mi>p</mi>
</math>";
    test_braille("Finnish", expr, "⠳⠲⠏");
}

#[test]
fn p40_p_and_q() {
    let expr = "<math>
    <mi>p</mi>
    <mo>∧</mo>
    <mi>q</mi>
</math>";
    test_braille("Finnish", expr, "⠏⠀⠳⠩⠀⠟");
}

#[test]
fn p40_p_or_q() {
    let expr = "<math>
    <mi>p</mi>
    <mo>∨</mo>
    <mi>q</mi>
</math>";
    test_braille("Finnish", expr, "⠏⠀⠳⠬⠀⠟");
}

#[test]
fn p41_function_definition() {
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
    test_braille("Finnish", expr, "⠋⠒⠀⠭⠀⠤⠱⠀⠋⠦⠭⠴");
}

#[test]
fn p41_inverse_function() {
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
    test_braille("Finnish", expr, "⠋⠬⠤⠼⠁⠀⠶⠫⠦⠽⠂⠀⠭⠴⠀⠳⠔⠠⠃⠀⠰⠄⠠⠁⠀⠸⠀⠽⠀⠶⠋⠦⠭⠴⠻");
}

// Dots 156 to signify the bar over "arc"
#[test]
fn p45_arcsin_with_bar_over() {
    let expr = "<math>
<mover>
<mi>arc</mi>
<mo>‾</mo>
</mover>
<mi>sin</mi>
<mi>x</mi>
</math>";
    test_braille("Finnish", expr, "⠁⠗⠉⠱⠀⠎⠊⠝⠀⠭");
}

// 'Change of zone' dot 5 is used after the limit subscript notation.
#[test]
fn p48_limit_right_hand_approaching_Finnish_notation() {
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
    test_braille("Finnish", expr, "⠇⠊⠍⠡⠦⠭⠀⠤⠱⠀⠼⠚⠀⠖⠴⠐⠋⠦⠭⠴");
}

// 'Change of zone' dot 5 is used after the limit subscript notation.
#[test]
fn p48_limit_right_hand_approaching_Finnish_notation_with_msup() {
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
    test_braille("Finnish", expr, "⠇⠊⠍⠡⠦⠭⠀⠤⠱⠀⠼⠚⠬⠖⠴⠐⠋⠦⠭⠴");
}

// 'Change of zone' dot 5 is used after the limit subscript notation.
#[test]
fn limit_approaches_from_below() {
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
    test_braille("Finnish", expr, "⠇⠊⠍⠡⠦⠭⠀⠔⠱⠀⠼⠚⠴⠐⠋⠦⠭⠴");
}

// This is a hack, so the Finnish substitution notation for integral would work. The intent is might be wrong, but that is what the notation means.
#[test]
fn p49_integral_with_Finnish_notation_for_substitution() {
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
  </mrow>
  <mrow>
    <msubsup>
      <mo intent='substitution'>⧸</mo>
      <mn>0</mn>
      <mn>4</mn>
    </msubsup>
    <mfrac>
      <mn>2</mn>
      <mn>4</mn>
    </mfrac>
    <mi>x</mi>
    <msqrt>
      <mi>x</mi>
    </msqrt>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠮⠢⠼⠚⠔⠼⠙⠐⠩⠭⠀⠙⠭⠀⠶⠸⠢⠼⠚⠔⠼⠙⠐⠦⠼⠃⠒⠀⠭⠀⠩⠭⠴");
}

#[test]
fn p50_such_that_y_greater_than_x() {
    let expr = "<math>
    <mi>∀</mi>
    <mi>x</mi>
    <mo>∈</mo>
    <mi>ℝ</mi>
    <mo>,</mo>
    <mi>∃</mi>
    <mi>𝑦</mi>
    <mo>∈</mo>
    <mi>ℝ</mi>
    <mi>y</mi>
    <mo>;</mo>
    <mo>&gt;</mo>
    <mi>x</mi>
</math>";
    test_braille("Finnish", expr, "⠳⠂⠭⠀⠳⠔⠠⠗⠂⠀⠳⠢⠽⠀⠳⠔⠠⠗⠆⠀⠽⠀⠱⠀⠭");
}

// After the root sign, dot 5 is used to signify 'change of zone' in the braille.
#[test]
fn p50_cube_root() {
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
    test_braille("Finnish", expr, "⠩⠼⠉⠐⠦⠭⠀⠖⠼⠑⠴");
}

#[test]
fn p50_binomial_with_factorial() {
    let expr = "<math>
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
  </mrow>
  <mrow>
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
</math>";
    test_braille("Finnish", expr, "⠦⠼⠋⠯⠀⠼⠙⠴⠀⠶⠼⠋⠠⠲⠀⠌⠷⠼⠙⠠⠲⠀⠦⠼⠋⠀⠤⠼⠙⠠⠴⠠⠲⠾");
}

#[test]
fn p50_conditional_probability() {
    let expr = "<math>
  <mrow>
    <mi>P</mi>
    <mo>(</mo>
    <mi>B</mi>
    <mi>|</mi>
    <mi>A</mi>
    <mo>)</mo>
    <mo>=</mo>
  </mrow>
  <mrow>
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
    test_braille("Finnish", expr, "⠠⠏⠦⠠⠃⠀⠸⠀⠠⠁⠴⠀⠶⠠⠏⠦⠠⠁⠠⠎⠴⠀⠌⠠⠏⠦⠠⠁⠴");
}

// Not sure about this MathML markup.
#[test]
fn p50_alternative_derivative_1() {
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
    test_braille("Finnish", expr, "⠙⠇⠝⠭⠀⠌⠙⠭");
}

// Not sure about this MathML markup.
#[test]
fn p50_alternative_derivative_2() {
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
    test_braille("Finnish", expr, "⠙⠀⠌⠙⠭⠀⠇⠝⠭");
}



#[test]
fn p51_set_with_closure() {
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
<mo>−</mo>
<mi>A</mi>
<mo>)</mo>
</mrow>
<mo>‾</mo>
</mover>
</math>";
    test_braille("Finnish", expr, "⠈⠙⠠⠁⠀⠶⠠⠁⠱⠀⠳⠦⠦⠠⠭⠀⠳⠤⠠⠁⠴⠱");
}

#[test]
fn p51_normal_distribution() {
    let expr = "<math>
  <mrow>
    <mi>p</mi>
    <mo>~</mo>
  </mrow>
  <mrow>
    <mi>N</mi>
    <mo>(</mo>
    <mn>58</mn>
    <mo>,</mo>
    <mn>2</mo>
    <mo>)</mo>
  </mrow>
</math>";
    test_braille("Finnish", expr, "⠏⠀⠨⠶⠠⠝⠦⠼⠑⠓⠂⠀⠼⠃⠴");
}

#[test]
fn p52_right_circular_cone_volume() {
    let expr = "<math>
  <mrow>
    <mi>V</mi>
    <mo>=</mo>
  </mrow>
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
    test_braille("Finnish", expr, "⠠⠧⠀⠶⠼⠁⠒⠀⠨⠏⠀⠗⠬⠀⠓");
}

#[test]
fn p52_circular_cone_area_equation() {
    let expr = "<math>
  <mrow>
    <mi>A</mi>
    <mo>=</mo>
  </mrow>
  <mrow>
    <mi>π</mi>
    <msup>
      <mi>r</mi>
      <mn>2</mn>
    </msup>
    <mo>+</mo>
  </mrow>
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
    test_braille("Finnish", expr, "⠠⠁⠀⠶⠨⠏⠀⠗⠬⠀⠖⠨⠏⠀⠗⠀⠩⠦⠗⠬⠀⠖⠓⠬⠴");
}

#[test]
fn p52_quadratic_formula() {
    let expr = "<math>
  <mrow>
    <mi>x</mi>
    <mo>=</mo>
  </mrow>
  <mrow>
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
    test_braille("Finnish", expr, "⠭⠀⠶⠷⠤⠃⠀⠖⠤⠩⠦⠃⠬⠀⠤⠼⠙⠀⠁⠉⠴⠾⠀⠌⠦⠼⠃⠀⠁⠴");
}