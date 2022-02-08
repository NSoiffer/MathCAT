// UEB tests for the basic mathml tags
// Initial tests are from BANA guidelines, mostly about initial chars for code switching
//   http://www.brailleauthority.org/ueb/ueb_math_guidance/final_for_posting_ueb_math_guidance_may_2019_102419.pdf
// These tests start with "bana_"
//
// Many come from (refer to) https://iceb.org/guidelines_for_technical_material_2014.pdf
// For example, "fraction_6_1_1" is a fraction example from section 6.1, and is the first example there.
use crate::common::*;

#[test]
fn bana_2_1() {
    let expr = "<math><mn>6</mn><mo>=</mo><mn>1</mn><mo>&#xD7;</mo><mn>2</mn><mo>&#xD7;</mo><mn>3</mn>
                <mo>=</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>+</mo><mn>3</mn></math>";
    test_braille("UEB", expr, "⠼⠋⠀⠐⠶⠀⠼⠁⠐⠦⠼⠃⠐⠦⠼⠉⠀⠐⠶⠀⠼⠁⠐⠖⠼⠃⠐⠖⠼⠉");
}

#[test]
fn bana_5_1() {
    let expr = "<math><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mn>6</mn></math>";
    test_braille("UEB", expr, "⠭⠐⠖⠽⠀⠐⠶⠀⠼⠋");
}

#[test]
fn bana_5_2() {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup><mo>=</mo><mi>C</mi></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠐⠖⠽⠔⠼⠃⠀⠐⠶⠀⠰⠠⠉");
}

#[test]
fn bana_5_3() {
    let expr = "<math><mfrac><mi>a</mi><mi>b</mi></mfrac><mo>+</mo><mfrac><mi>c</mi><mi>d</mi></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠁⠨⠌⠃⠾⠐⠖⠷⠉⠨⠌⠙⠾");
}

#[test]
fn bana_5_4() {
    let expr = "<math><msup><mi>a</mi><mi>n</mi></msup><mo>&#xD7;</mo><msup><mi>a</mi><mi>m</mi></msup><mo>=</mo>
                    <msup><mi>a</mi><mrow><mi>n</mi><mo>+</mo><mi>m</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠁⠔⠝⠐⠦⠁⠔⠍⠀⠐⠶⠀⠁⠔⠣⠝⠐⠖⠍⠜⠰⠄");
}

#[test]
fn bana_5_5() {
    let expr = "<math><msub><mi>log</mi><mi>x</mi></msub><mi>y</mi></math>";
    test_braille("UEB", expr, "⠰⠰⠇⠕⠛⠢⠭⠀⠰⠽");
}

#[test]
fn bana_5a_1() {
    let expr = "<math><msup><mn>100</mn><mo>&#xB0;</mo></msup><mi>F</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠚⠚⠘⠚⠠⠋");
}

#[test]
fn bana_5a_1_baseline() {
    let expr = "<math><mn>100</mn><mo>&#xB0;</mo><mi class='MathML-unit'>F</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠚⠚⠘⠚⠠⠋");
}

#[test]
fn bana_5a_2() {
    let expr = "<math><mn>25</mn><mo>&#xA0;</mo><msup><mi class='MathML-unit'>km</mi><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠼⠃⠑⠀⠅⠍⠰⠔⠼⠃");
}

#[test]
fn bana_5a_2_mtext() {
    let expr = "<math><mn>25</mn><mo>&#xA0;</mo><msup><mtext>km</mtext><mn>2</mn></msup></math>";
    test_braille("UEB", expr, "⠼⠃⠑⠀⠅⠍⠰⠔⠼⠃");
}

#[test]
fn bana_5a_3() {
    let expr = "<math><mn>6</mn><mo>&#xA0;</mo><mi>m</mi><mo>&#xA0;</mo>
            <msup><mi>s</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup></math>";
    test_braille("UEB", expr, "⠼⠃⠑⠀⠅⠍⠰⠔⠼⠃");
}

#[test]
fn bana_6_1() {
    let expr = "<math><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mi>z</mi>
                        <mo>=</mo><msup><mi>t</mi><mn>2</mn></msup><mo>.</mo></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠐⠖⠽⠀⠐⠶⠀⠵⠀⠐⠶⠀⠞⠔⠼⠃⠲⠰⠄");
}

#[test]
fn number_2_1_2() {
    let expr = "<math><mn>3,000</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠂⠚⠚⠚");
}

#[test]
fn number_2_1_3() {
    let expr = "<math><mn>5 000 000</mn></math>";
    test_braille("UEB", expr, "⠼⠑⠐⠚⠚⠚⠐⠚⠚⠚");
}

#[test]
fn number_2_2_3() {
    let expr = "<math><mn>.7</mn></math>";
    test_braille("UEB", expr, "⠼⠲⠛");
}

#[test]
fn time_2_4_1() {
    let expr = "<math><mn>5</mn><mo>:</mo><mn>30</mn><mo>&#xA0;</mo><mtext>pm</mtext></math>";
    test_braille("UEB", expr, "⠼⠑⠒⠼⠉⠚ ⠏⠍");
}

#[test]
fn bold_2_7_3() {
    let expr = "<math><mn>67</mn><mn mathvariant='bold'>84</mn><mn>5</mn></math>";
    test_braille("UEB", expr, "⠼⠋⠛⠘⠂⠼⠓⠙⠘⠄⠼⠑");
}

#[test]
fn bold_2_7_4() {
    let expr = "<math><menclose notation='bottom'><mn>678</mn></menclose><mn>45</mn></math>";
    test_braille("UEB", expr, "⠸⠂⠼⠋⠛⠓⠸⠄⠼⠙⠑");
}

#[test]
fn signs_2_10_2() {
    init_logger();
    let expr = "<math><mo>$</mo><mn>0.30</mn><mo>,</mo><mo>&#xA0;</mo>
                <mn>30</mn><mi mathvariant='normal'>c</mi><mo>&#xA0;</mo>
                <mtext>or</mtext><mo>&#xA0;</mo><mn>30</mn><mo>&#xA2;</mo></math>";
    test_braille("UEB", expr, "⠈⠎⠼⠚⠲⠉⠚⠂⠀⠼⠉⠚⠰⠉⠀⠕⠗⠀⠼⠉⠚⠈⠉⠲");
}

#[test]
fn signs_2_10_5() {
    let expr = "<math><mn>1</mn><mo>&#xA0;</mo><mi>ft</mi><mo>&#xA0;</mo><mn>6</mn><mo>&#xA0;</mo><mi>in</mi>
        <mo>&#xA0;</mo><mtext>or</mtext><mo>&#xA0;</mo>
        <mn>1</mn><mo>&#x2032;</mo><mo>&#xA0;</mo><mn>6</mn><mo>&#x2032;</mo><mo>&#x2032;</mo></math>";
    test_braille("UEB", expr, "⠼⠁⠀⠋⠞⠀⠼⠋⠀⠔⠀⠕⠗⠀⠼⠁⠶⠀⠼⠋⠶⠶");
}

#[test]
fn signs_2_10_8() {
    let expr = "<math><mn>0</mn><mo>&#xB0;</mo><mi mathvariant='normal'>C</mi><mo>&#xA0;</mo><mtext>or</mtext>
        <mo>&#xA0;</mo><mn>32</mn><mo>&#xB0;</mo><mi mathvariant='normal'>F</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠀⠋⠞⠀⠼⠋⠀⠔⠀⠕⠗⠀⠼⠁⠶⠀⠼⠋⠶⠶");
}

#[test]
fn signs_2_10_16() {
    let expr = "<math><mn>1</mn><mi mathvariant='normal'>&#xC5;</mi><mo>=</mo>
        <mfrac><mn>1</mn><mrow><mn>10</mn><mo>,</mo><mn>000</mn></mrow></mfrac>
        <mi mathvariant='normal'>&#x3BC;</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠀⠠⠘⠫⠁⠀⠐⠶⠀⠼⠁⠌⠁⠚⠂⠚⠚⠚⠀⠨⠍");
}

#[test]
fn ratio_3_1_12() {
    let expr = "<math><mn>2</mn><mo>:</mo><mn>4</mn><mo>=</mo><mn>6</mn><mo>:</mo><mn>12</mn></math>";
    test_braille("UEB", expr, "⠼⠃⠒⠼⠙⠀⠐⠶⠀⠼⠋⠒⠼⠁⠃");
}

#[test]
fn ratio_3_2_6() {
    // the difference from ratio_3_1_12 is this involves letters
    let expr = "<math><mi>x</mi><mo>:</mo><mi>y</mi></math>";
    test_braille("UEB", expr, "⠭⠰⠒⠽");
}

#[test]
fn fraction_6_1_1() {
    let expr = "<math><mfrac><mn>5</mn><mn>8</mn></mfrac></math>";
    test_braille("UEB", expr, "⠼⠑⠌⠓");
}

#[test]
fn fraction_6_1_2() {
    let expr = "<math><mfrac><mrow><mn>5</mn><mo>.</mo><mn>7</mn></mrow><mrow><mn>2</mn><mo>,</mo><mn>000</mn></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠼⠑⠲⠛⠌⠃⠂⠚⠚⠚");
}

#[test]
fn fraction_6_2_1() {
    let expr = "<math><mn>2</mn><mfrac bevelled='true'><mn>1</mn><mn>2</mn></mfrac></math>";
    test_braille("UEB", expr, "⠼⠃⠼⠁⠌⠃");
}

#[test]
fn fraction_6_2_2() {
    let expr = "<math><mn>1750</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-Unit'>cm</mi><mo>=</mo>
                <mn>1</mn><mfrac bevelled='true'><mn>3</mn><mn>4</mn></mfrac>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-Unit'>m</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠛⠑⠚⠀⠉⠍⠀⠐⠶⠀⠼⠁⠼⠉⠌⠙⠀⠰⠍");
}

#[test]
fn fraction_6_2_2_unicode_frac() {
    let expr = "<math><mn>1750</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-Unit'>cm</mi><mo>=</mo>
                <mn>1</mn><mn>&#xBE;</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-Unit'>m</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠛⠑⠚⠀⠉⠍⠀⠐⠶⠀⠼⠁⠼⠉⠌⠙⠀⠰⠍");
}

#[test]
fn fraction_6_3_1() {
    let expr = "<math><mn>3</mn><mo>/</mo><mn>8</mn></math>";
    test_braille("UEB", expr, "⠼⠉⠸⠌⠼⠓");
}

#[test]
fn fraction_6_4_1() {
    let expr = "<math><mi>y</mi><mo>=</mo><mfrac><mi>x</mi><mn>2</mn></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠽⠀⠐⠶⠀⠷⠭⠨⠌⠼⠃⠾⠰⠄");
}

#[test]
fn fraction_6_4_2() {
    let expr = "<math><mfrac>
        <mrow><mn>2</mn><mfrac><mn>1</mn><mn>2</mn></mfrac></mrow>
        <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        </mfrac></math>";
    test_braille("UEB", expr, "⠰⠷⠼⠃⠼⠁⠌⠃⠨⠌⠭⠐⠖⠽⠾");
}

#[test]
fn fraction_6_4_3() {
    let expr = "<math><mfrac><mrow><mn>2</mn><mo>/</mo><mn>3</mn></mrow><mn>5</mn></mfrac></math>";
    test_braille("UEB", expr, "⠰⠷⠼⠃⠸⠌⠼⠉⠨⠌⠼⠑⠾");
}

#[test]
fn fraction_6_4_4() {
    let expr = "<math><mfrac>
    <mrow><mfrac><mi>x</mi><mn>2</mn></mfrac><mo>+</mo><mfrac><mi>y</mi><mn>3</mn></mfrac></mrow>
    <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        </mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠷⠭⠨⠌⠼⠃⠾⠐⠖⠷⠽⠨⠌⠼⠉⠾⠨⠌⠭⠐⠖⠽⠾");
}

#[test]
fn fraction_6_4_5() {
    let expr = "<math><mfrac>
        <mrow><mfrac><mi>x</mi><mn>2</mn></mfrac><mo>+</mo><mfrac><mi>y</mi><mn>3</mn></mfrac></mrow>
        <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        </mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠷⠭⠨⠌⠼⠃⠾⠐⠖⠷⠽⠨⠌⠼⠉⠾⠨⠌⠭⠐⠖⠽⠾");
}

#[test]
fn fraction_6_4_6() {
    let expr = "<math><mtext>speed</mtext><mo>=</mo><mfrac><mtext>distance</mtext><mtext>time</mtext></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠎⠏⠑⠑⠙⠀⠐⠶⠀⠷⠙⠊⠎⠞⠁⠝⠉⠑⠨⠌⠞⠊⠍⠑⠾⠰⠄");
}


#[test]
fn msup_7_3_2() {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mi>y</mi></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠽");
}

#[test]
fn msup_7_3_3() {
    let expr = "<math><msup><mi>x</mi><mrow><mn>2</mn><mi>y</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠣⠼⠃⠽⠜");
}

#[test]
fn msup_7_3_4() {
    let expr = "<math><msup><mi>x</mi><mi>y</mi></msup><mo>+</mo><mn>1</mn></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠽⠐⠖⠼⠁");
}

#[test]
fn msup_7_3_6() {
    let expr = "<math><msup><mi>x</mi><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow></msup><mo>+</mo><mn>3</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠣⠽⠐⠖⠼⠁⠜⠐⠖⠼⠉");
}

#[test]
fn msup_7_3_7() {
    let expr = "<math><msup><mi>x</mi><mn>⅔</mn></msup></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠌⠉");
}

#[test]
fn msup_7_3_11() {
    let expr = "<math><msup><mi>x</mi><mfrac><mi>a</mi><mi>b</mi></mfrac></msup><mi>y</mi><mo>=</mo><mi>x</mi></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠔⠷⠁⠨⠌⠃⠾⠽⠀⠐⠶⠀⠭⠰⠄");
}

#[test]
fn msup_7_4_1() {
    let expr = "<math><msup><mi>e</mi><msup><mi>x</mi><mn>2</mn></msup></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠑⠔⠣⠭⠔⠼⠃⠜");
}

#[test]
fn msup_7_4_2() {
    let expr = "<math><msup><mi>e</mi><mrow><mo>(</mo><msup><mi>x</mi><mn>2</mn></msup><mo>)</mo></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠑⠔⠐⠣⠭⠔⠼⠃⠐⠜");
}

#[test]
fn msub_7_4_3() {
    let expr = "<math><msub><mi>P</mi><msub><mi>x</mi><mi>i</mi></msub></msub></math>";
    test_braille("UEB", expr, "⠰⠰⠠⠏⠢⠣⠭⠢⠊⠜");
}

#[test]
fn msup_7_5_1() {
    let expr = "<math><mn>0</mn><mo>.</mo><mn>0045</mn><mo>=</mo>
        <mn>4</mn><mo>.</mo><mn>5</mn><mo>&#xD7;</mo><msup><mn>10</mn><mrow><mo>-</mo><mn>3</mn></mrow></msup>
        </math>";
    test_braille("UEB", expr, "⠼⠚⠲⠚⠚⠙⠑⠀⠐⠶⠀⠼⠙⠲⠑⠐⠦⠼⠁⠚⠔⠣⠐⠤⠼⠉⠜");
}

#[test]
fn msup_7_5_3() {
    let expr = "<math><msup><mi>a</mi><mrow><mo>-</mo><mn>2</mn><mi>b</mi></mrow></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠁⠔⠣⠐⠤⠼⠃⠰⠃⠜");
}

#[test]
fn msup_7_6_2() {
    let expr = "<math><msup><mi mathvariant='normal'>H</mi><mo>+</mo></msup></math>";
    test_braille("UEB", expr, "⠠⠓⠰⠔⠐⠖");
}

#[test]
fn msubsup_7_7_1() {
    let expr = "<math><msubsup><mi>x</mi><mn>1</mn><mn>2</mn></msubsup><mo>=</mo><msubsup><mi>y</mi><mn>2</mn><mn>3</mn></msubsup></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠢⠼⠁⠔⠼⠃⠀⠐⠶⠀⠽⠢⠼⠃⠔⠼⠉⠰⠄");
}

#[test]
fn msubsup_7_7_2() {
    let expr = "<math><msub><msup><mi>x</mi><mn>2</mn></msup><mi>k</mi></msub></math>";
    test_braille("UEB", expr, "⠭⠰⠔⠼⠃⠢⠅");
}

#[test]
fn pre_msubsup_7_8_1() {
    // Note: modified because word indicator is not needed
    let expr = "<math><mmultiscripts><mi>U</mi><mprescripts/><mn>92</mn><mn>238</mn></mmultiscripts></math>";
    test_braille("UEB", expr, "⠰⠢⠼⠊⠃⠔⠼⠃⠉⠓⠠⠥");
}

#[test]
fn pre_sup_7_8_2() {
    let expr = "<math><mmultiscripts><mn>2</mn><mprescripts/><none/><mo>-</mo></mmultiscripts>
            <mo>+</mo><mmultiscripts><mn>3</mn><mprescripts/><none/><mo>-</mo></mmultiscripts>
            <mo>=</mo><mmultiscripts><mn>5</mn><mprescripts/><none/><mo>-</mo></mmultiscripts>
        </math>";
    test_braille("UEB", expr, "⠰⠰⠰⠔⠐⠤⠼⠃⠐⠖⠔⠐⠤⠼⠉⠀⠐⠶⠀⠔⠐⠤⠼⠑⠰⠄");
}


#[test]
fn sum_7_9_1() {
    let expr = "<math><munderover><mo>&#x2211;</mo><mrow><mi>x</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover>
            <msubsup><mi>x</mi><mi>i</mi><mn>2</mn></msubsup></math>";
    test_braille("UEB", expr, "⠰⠰⠠⠨⠎⠨⠢⠣⠭⠐⠶⠼⠁⠜⠨⠔⠝⠭⠢⠊⠔⠼⠃");
}

#[test]
fn lim_7_9_2() {
    let expr = "<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><mi>a</mi></mrow></munder>
            <mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mn>1</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠇⠊⠍⠨⠢⠣⠭⠳⠕⠁⠜⠋⠐⠣⠭⠐⠜⠀⠐⠶⠀⠼⠁⠰⠄");
}

#[test]
fn sqrt_8_1_1() {
    let expr = "<math><msqrt><mn>9</mn></msqrt><mo>=</mo><mn>3</mn></math>";
    test_braille("UEB", expr, "⠰⠩⠼⠊⠬⠀⠐⠶⠀⠼⠉");
}

#[test]
fn sqrt_8_1_4() {
    let expr = "<math>
      <mi>x</mi> <mo>=</mo>
      <mfrac>
       <mrow>
        <mo>&#x2212;</mo><mi>b</mi><mo>&#x00B1;</mo>
        <msqrt>
          <msup><mi>b</mi> <mn>2</mn></msup>
          <mo>&#x2212;</mo><mn>4</mn><mi>a</mi><mi>c</mi>
        </msqrt>
        </mrow>
       <mrow><mn>2</mn><mi>a</mi></mrow>
      </mfrac>
      </math>
    ";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠀⠐⠶⠀⠷⠐⠤⠃⠸⠖⠩⠃⠔⠼⠃⠐⠤⠼⠙⠰⠁⠉⠬⠨⠌⠼⠃⠰⠁⠾⠰⠄");
}

#[test]
fn root_8_2_1() {
    let expr = "<math><mroot><mn>8</mn><mn>3</mn></mroot><mo>=</mo><mn>2</mn></math>";
    test_braille("UEB", expr, "⠰⠩⠔⠼⠉⠼⠓⠬⠀⠐⠶⠀⠼⠃");
}

#[test]
fn root_8_2_3() {
    let expr = "<math><mroot><mrow><mi>x</mi><mi>y</mi></mrow><mrow><mi>m</mi><mi>n</mi></mrow></mroot></math>";
    test_braille("UEB", expr, "⠰⠰⠩⠔⠣⠍⠝⠜⠭⠽⠬");
}

#[test]
fn root_letter_base() {
    // none of the guides cover this case, but it seems that an a-j base needs a grade 1 indicator
    let expr = "<math><mroot><mi>b</mi><mn>3</mn></mroot><mroot><mi>x</mi><mn>3</mn></mroot></math>";
    test_braille("UEB", expr, "⠰⠰⠩⠔⠼⠉⠰⠃⠬⠩⠔⠼⠉⠭⠬");
}

#[test]
fn spacing_9_3_1_3() {
    let expr = "<math><mn>4</mn><mi>cos</mi><mn>5</mn><mi>x</mi></math>";
    test_braille("UEB", expr, "⠼⠙⠰⠉⠕⠎⠼⠑⠭");
}

#[test]
fn spacing_9_3_2_1() {
    let expr = "<math><mi>log</mi><mi>y</mi></math>";
    test_braille("UEB", expr, "⠇⠕⠛⠀⠰⠽");
}

#[test]
fn spacing_9_3_2_2() {
    let expr = "<math><mi>sin</mi><mi>&#x3B8;</mi></math>";
    test_braille("UEB", expr, "⠎⠊⠝⠨⠹");
}

#[test]
fn spacing_9_3_2_4() {
    let expr = "<math><mi>log</mi><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠇⠕⠛⠐⠣⠭⠐⠖⠽⠐⠜");
}

#[test]
fn spacing_9_3_3_1() {
    let expr = "<math><mi>x</mi><mi>sin</mi><mn>60</mn></math>";
    test_braille("UEB", expr, "⠰⠭⠀⠎⠊⠝⠼⠋⠚");
}

#[test]
fn spacing_9_3_3_2() {
    let expr = "<math><mi>x</mi><mrow><mi>Sin</mi><mo>&#x2061;</mo><mn>60</mn></mrow></math>";
    test_braille("UEB", expr, "⠭⠠⠎⠊⠝⠼⠋⠚");
}

#[test]
fn spacing_9_3_3_3() {
    let expr = "<math><mi>X</mi><mi>log</mi><mi>y</mi></math>";
    test_braille("UEB", expr, "⠰⠠⠭⠀⠇⠕⠛⠀⠰⠽");
}

#[test]
fn spacing_9_3_3_6() {
    let expr = "<math><mi>sin</mi><mn>2</mn><mi>&#x3B2;</mi><mo>=</mo>
                <mn>2</mn><mi>sin</mi><mi>&#x3B2;</mi><mi>cos</mi><mi>&#x3B2;</mi></math>";
    test_braille("UEB", expr, "⠎⠊⠝⠼⠃⠨⠃⠀⠐⠶⠀⠼⠃⠎⠊⠝⠨⠃⠉⠕⠎⠨⠃");
}

#[test]
fn text_9_7_1_mtext() {
    let expr = "<math><mtext>Pr</mtext><mo>(</mo><mi>A</mi><mo>&#xA0;</mo><mtext>and</mtext><mo>&#xA0;</mo><mi>B</mi><mo>)</mo><mo>=</mo>
         <mtext>Pr&#xA0;</mtext><mi>A</mi><mo>+</mo><mtext>Pr&#xA0;</mtext><mi>B</mi></math>";
    test_braille("UEB", expr, "⠠⠏⠗⠐⠣⠠⠁⠀⠯⠀⠰⠠⠃⠐⠜⠀⠐⠶⠀⠠⠏⠗⠠⠁⠐⠖⠠⠏⠗⠠⠃");
}
#[test]
fn text_9_7_1() {
    let expr = "<math><mi>Pr</mi><mo>(</mo><mi>A</mi><mo>&#xA0;</mo><mi>and</mi><mo>&#xA0;</mo><mi>B</mi><mo>)</mo><mo>=</mo>
         <mi>Pr</mi><mi>A</mi><mo>+</mo><mi>Pr</mi><mi>B</mi></math>";
    test_braille("UEB", expr, "⠠⠏⠗⠐⠣⠠⠁⠀⠯⠀⠰⠠⠃⠐⠜⠀⠐⠶⠀⠠⠏⠗⠠⠁⠐⠖⠠⠏⠗⠠⠃");
}

#[test]
fn example_11_5_1_2() {
    let expr = "<math><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠙⠽⠨⠌⠙⠭⠾");
}

#[test]
fn example_11_5_1_3() {
    let expr = "<math><mi>f</mi><mo>'</mo><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠰⠰⠋⠶⠐⠣⠭⠐⠜");
}

#[test]
fn example_11_5_1_4() {
    let expr = "<math><mfrac><mrow><mo>&#x2202;</mo><mi>y</mi></mrow><mrow><mo>&#x2202;</mo><mi>x</mi></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠷⠈⠙⠽⠨⠌⠈⠙⠭⠾");
}

#[test]
fn example_11_5_2() {
    let expr = "<math><msubsup><mo>&#x222B;</mo><mn>2</mn><mn>3</mn></msubsup><mo>(</mo><mn>2</mn><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo><mo>d</mo><mi>x</mi>
        <mo>=</mo><msubsup><mfenced open='[' close=']'><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mi>x</mi></mrow></mfenced><mn>2</mn><mn>3</mn></msubsup>
        <mo>=</mo><mo>(</mo><msup><mn>3</mn><mn>2</mn></msup><mo>+</mo><mn>3</mn><mo>)</mo><mo>-</mo><mo>(</mo><msup><mn>2</mn><mn>2</mn></msup><mo>+</mo><mn>2</mn><mo>)</mo>
        <mo>=</mo><mn>12</mn><mo>-</mo><mn>6</mn><mo>=</mo><mn>6</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠮⠢⠼⠃⠔⠼⠉⠐⠣⠼⠃⠭⠐⠖⠼⠁⠐⠜⠙⠭⠀⠐⠶⠀⠨⠣⠭⠔⠼⠃⠐⠖⠭⠨⠜⠢⠼⠃⠔⠼⠉⠀⠐⠶⠀⠐⠣⠼⠉⠔⠼⠃⠐⠖⠼⠉⠐⠜⠐⠤⠐⠣⠼⠃⠔⠼⠃⠐⠖⠼⠃⠐⠜⠀⠐⠶⠀⠼⠁⠃⠐⠤⠼⠋⠀⠐⠶⠀⠼⠋⠰⠄");
}

#[test]
fn example_11_5_3() {
    // modified to use "shape" as recommended in a comment on this example
    let expr = "<math><msub><mmultiscripts><mi>C</mi><mprescripts/><none/><mi>n</mi></mmultiscripts><mi>r</mi></msub><mo>=</mo>
            <mo>(</mo><mfrac linethickness='0'><mi>n</mi><mi>r</mi></mfrac><mo>)</mo><mo>=</mo>
            <mfrac><mrow><mi>n</mi><mo>!</mo></mrow><mrow><mi>r</mi><mo>!</mo><mo>(</mo><mi>n</mi><mo>-</mo><mi>r</mi><mo>)</mo><mo>!</mo></mrow></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠔⠝⠠⠉⠢⠗⠀⠐⠶⠀⠐⠣⠝⠰⠻⠗⠐⠜⠀⠐⠶⠀⠷⠝⠖⠨⠌⠗⠖⠐⠣⠝⠐⠤⠗⠐⠜⠖⠾⠰⠄");
}

#[test]
fn example_11_5_4() {
    let expr = "<math><mi>a</mi><mo>&#x2217;</mo><mo>(</mo><mi>b</mi><mo>&#x25E6;</mo><mi>c</mi><mo>)</mo>
        <mo>=</mo><mo>(</mo><mi>a</mi><mo>&#x2217;</mo><mi>b</mi><mo>)</mo><mo>&#x25E6;</mo><mo>(</mo><mi>a</mi><mo>&#x2217;</mo><mi>c</mi><mo>)</mo></math>";
    test_braille("UEB", expr, "⠁⠐⠔⠐⠣⠃⠐⠴⠉⠐⠜⠀⠐⠶⠀⠐⠣⠁⠐⠔⠃⠐⠜⠐⠴⠐⠣⠁⠐⠔⠉⠐⠜");
}

#[test]
fn bar_over_12_1_1() {
    let expr = "<math><mover><mi>x</mi><mo>_</mo></mover><mo>=</mo>
        <mfrac><mrow><mn>10</mn><mo>+</mo><mn>11</mn><mo>+</mo><mn>12</mn></mrow><mn>3</mn></mfrac></math>";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠱⠀⠐⠶⠀⠷⠼⠁⠚⠐⠖⠼⠁⠁⠐⠖⠼⠁⠃⠨⠌⠼⠉⠾⠰⠄");
}

#[test]
fn bar_under_12_1_2() {
    let expr = "<math><munder><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>_</mo></munder></math>";
    test_braille("UEB", expr, "⠰⠰⠣⠭⠐⠖⠽⠜⠠⠱");
}

#[test]
fn bar_menclose_12_1_2() {
    let expr = "<math><menclose notation='bottom'><mi>x</mi><mo>+</mo><mi>y</mi></menclose></math>";
    test_braille("UEB", expr, "⠰⠰⠣⠭⠐⠖⠽⠜⠠⠱");
}

#[test]
fn dot_12_1_4() {
    let expr = "<math><mn>0</mn><mo>.</mo><mover><mn>3</mn><mo>.</mo></mover></math>";
    test_braille("UEB", expr, "⠼⠚⠲⠣⠼⠉⠜⠘⠲");
}

#[test]
fn dot_12_1_4_dot_above_char() {
    let expr = "<math><mn>0</mn><mo>.</mo><mn>56</mn><mover><mn>1</mn><mo>&#x2D9;</mo></mover>
            <mn>2</mn><mover><mn>3</mn><mo>&#x2D9;</mo></mover></math>";
    test_braille("UEB", expr, "⠼⠚⠲⠑⠋⠣⠼⠁⠜⠘⠲⠼⠃⠣⠼⠉⠜⠘⠲");
}

#[test]
fn dot_12_1_5_single() {
    let expr = "<math><mover><mi>x</mi><mo>&#x2D9;</mo></mover></math>";
    test_braille("UEB", expr, "⠭⠘⠲");
}

#[test]
fn dot_12_1_5_double() {
    let expr = "<math><mover><mi>x</mi><mo>&#xA8;</mo></mover></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠨⠔⠣⠲⠲⠜");
}

#[test]
fn hat_12_1_7() {
    // This is modified from the example because in reality, the hat is over all the chars
    let expr = "<math><mover><mrow><mi>A</mi><mi>B</mi><mi>C</mi></mrow><mo>^</mo></mover></math>";
    test_braille("UEB", expr, "⠣⠠⠠⠁⠐⠉⠜⠐⠱");
}

#[test]
fn arrow_12() {
    // This is not directly an example, but the text at the start says this is the result
    let expr = "<math><mover><mi>x</mi><mo>→</mo></mover></math>";
    test_braille("UEB", expr, "⠭⠘⠱");
}

#[test]
fn bar_12_2_1() {
    let expr = "<math><msup><mi>x</mi><mover><mi>y</mi><mo>&#xAF;</mo></mover></msup></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠣⠽⠱⠜");
}

#[test]
fn bar_12_2_2() {
    let expr = "<math><mover><msup><mi>x</mi><mi>y</mi></msup><mo>&#xAF;</mo></mover></math>";
    test_braille("UEB", expr, "⠰⠰⠣⠭⠔⠽⠜⠱");
}

#[test]
fn shape_14_1_1_1() {
    let expr = "<math><mo>&#x25B3;</mo><mo>&#xA0;</mo><mtext>ABC</mtext></math>";
    test_braille("UEB", expr, "⠰⠫⠼⠉⠀⠠⠠⠁⠃⠉");
}

#[test]
fn shape_14_1_2_1() {
    let expr = "<math><mo>&#x25B3;</mo><mtext>ABC</mtext></math>";
    test_braille("UEB", expr, "⠰⠫⠼⠉⠱⠠⠠⠁⠃⠉");
}

#[test]
fn shape_14_1_2_2() {
    // the <mo> for the shapes are wrong -- but it isn't clear what they should be (from WIRIS editor)
    let expr = "<math><mo>{</mo><mo>&#x25A1;</mo><mo>,</mo>
                            <mo>&#xA0;</mo><mo>&#x25CD;</mo><mo>,</mo>
                            <mo>&#xA0;</mo><mo>&#x25B2;</mo><mo>,</mo>
                            <mo>&#xA0;</mo><mo>&#x25A7;</mo><mo>&#xA0;</mo><mo>&#x2026;</mo><mo>}</mo></math>";
    test_braille("UEB", expr, "⠸⠣⠰⠫⠼⠙⠱⠂⠀⠨⠫⠿⠱⠂⠀⠸⠫⠼⠉⠱⠂⠀⠨⠫⠼⠙⠀⠲⠲⠲⠸⠜");
}

#[test]
fn binomial_14_3_3_2() {
    let expr = "<math><mfenced><mfrac linethickness='0'><mi>n</mi><mi>r</mi></mfrac></mfenced></math>";
    test_braille("UEB", expr, "⠐⠣⠝⠰⠻⠗⠐⠜");
}

#[test]
fn chem_16_2_8() {
    let expr = "<math><mi>Ca</mi><msub><mrow><mo>(</mo><mi>OH</mi><mo>)</mo></mrow><mn>2</mn></msub></math>";
    test_braille("UEB", expr, "⠠⠉⠁⠐⠣⠠⠕⠠⠓⠐⠜⠰⠢⠼⠃");
}
