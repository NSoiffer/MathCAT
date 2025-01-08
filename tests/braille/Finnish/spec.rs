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
fn calculation_marks_7() {
    let expr = "<math><mrow><mi>&#x03C0;</mi><mo>&#x2248;</mo><mn>3,14</mn></mrow></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn fraction_10() {
    let expr = "<math>
        <mfrac><mn>3</mn><mn>4</mn></mfrac><mo>+</mo>
        <mfrac><mn>1</mn><mn>4</mn></mfrac><mo>=</mo><mn>1</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn fraction_13() {
    let expr = "<math><mfrac><mn>5</mn><mn>4</mn></mfrac><mo>=</mo><mn>1</mn><mfrac><mn>1</mn><mn>4</mn></mfrac></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn fraction_equations_3() {
    let expr = "<math>
        <mfrac>
            <mrow><mn>4</mn><mi>x</mi></mrow>
            <mrow><mn>6</mn><mo>(</mo><mn>1</mn><mo>&#x2212;</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac>
    </math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn fraction_equations_6() {
    let expr = "<math><mfrac>
        <mrow><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>+</mo><mfrac><mn>1</mn><mn>3</mn></mfrac></mrow>
        <mrow><mfrac><mn>1</mn><mn>4</mn></mfrac><mo>&#x2212;</mo><mfrac><mn>1</mn><mn>5</mn></mfrac></mrow>
    </mfrac></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn powers_3() {
    let expr = "<math><msup><mn>2</mn><mn>3</mn></msup><mo>+</mo><mn>5</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn powers_6() {
    let expr = "<math>
        <msup><mn>2</mn><mn>20</mn></msup>
        <mo>=</mo>
        <mn>1</mn><mtext>&#x2009;</mtext><mn>048</mn><mtext>&#x2009;</mtext><mn>576</mn>
    </math>
   ";
    test_braille("Finnish", expr, "");
}

#[test]
fn roots_5() {
    let expr = "<math><msup><mn>27</mn><mfrac><mn>1</mn><mn>3</mn></mfrac></msup><mo>=</mo>
                        <mroot><mn>27</mn><mn>3</mn></mroot><mo>=</mo><mn>3</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn roots_7() {
    let expr = "<math><msqrt><mn>20</mn><mo>+</mo><mn>5</mn></msqrt><mo>=</mo><msqrt><mn>25</mn></msqrt><mo>=</mo><mn>5</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn vectors_1() {
    let expr = "<math><mover><mi>a</mi><mo>&#xAF;</mo></mover></math>";
    test_braille("Finnish", expr, "");
}

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
    test_braille("Finnish", expr, "");
}

#[test]
fn p7_thousands_nbsp() {
    let expr = "<math><mn>2 000 000</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn p7_thousands_point() {
    let expr = "<math><mn>2.000.000</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn p7_thousands_space() {
    let expr = "<math><mn>2 000 000</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn p7_decimal_comma() {
    let expr = "<math><mn>5,12575</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn p7_decimal_point() {
    let expr = "<math><mn>5.12575</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn p7_decimal_point_endless_no_grouping() {
    let expr = "<math><mn>1,234657234...</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn p7_decimal_point_endless_with_grouping_space() {
    let expr = "<math><mn>1,234 657 234...</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn p7_decimal_point_endless_with_grouping_nbsp() {
    let expr = "<math><mn>1,234 657 234...</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn p7_decimal_point_endless_with_grouping_nbsp_ellipses() {
    let expr = "<math><mn>1,234 657 234…</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn p7_decimal_point_endless_no_grouping_ellipses() {
    let expr = "<math><mn>1,234657234…</mn></math>";
    test_braille("Finnish", expr, "");
}

#[test]
fn p7_decimal_point_endless_with_grouping_space_ellpses() {
    let expr = "<math><mn>1,234 657 234…</mn></math>";
    test_braille("Finnish", expr, "");
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