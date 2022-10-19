// Nemeth tests for the basic mathml tags
use crate::common::*;

#[test]
fn non_list_10_4() {
    let expr = "<math><mo>(</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mtext>and&#xA0;</mtext><mn>3</mn><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠠⠀⠼⠆⠠⠀⠁⠝⠙⠀⠼⠒⠾");
}

#[test]
fn list_num_ind_11_a_1() {
    let expr = "<math><mo>[</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>]</mo></math>";
    test_braille("Nemeth", expr, "⠈⠷⠴⠠⠀⠂⠈⠾");
}

#[test]
fn list_num_ind_11_a_2() {
    let expr = "<math><mo>(</mo><mo>-</mo><mn>1</mn><mo>,</mo><mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>3</mn><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠤⠂⠠⠀⠤⠆⠠⠀⠤⠒⠾");
}

#[test]
fn list_num_ind__11_a_3() {
    let expr = "<math><mo>(</mo><mn>1</mn><mo>+</mo><mi>h</mi><mo>,</mo><mn>2</mn><mo>+</mo><mi>k</mi><mo>,</mo><mn>0</mn><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠬⠓⠠⠀⠆⠬⠅⠠⠀⠴⠾");
}

#[test]
fn list_num_ind_11_a_7() {
    let expr = "<math><mo>(</mo><mi>x</mi><mo>,</mo><mn>7</mn><mo>,</mo><mn mathvariant='bold'>8</mn><mo>,</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠭⠠⠀⠶⠠⠀⠸⠼⠦⠠⠀⠽⠾");
}

#[test]
fn cap_roman_numeral_18_a_3() {
    let expr = "<math><mtext>VII</mtext><mo>+</mo><mtext>V</mtext><mo>=</mo><mtext>XII</mtext></math>";
    test_braille("Nemeth", expr, "⠠⠠⠧⠊⠊⠬⠠⠧⠀⠨⠅⠀⠠⠠⠭⠊⠊");
}

#[test]
fn lower_roman_numeral_18_b_4() {
    let expr = "<math><mtext>vi</mtext><mo>+</mo><mtext>iv</mtext><mo>=</mo><mtext>x</mtext></math>";
    test_braille("Nemeth", expr, "⠧⠊⠬⠊⠧⠀⠨⠅⠀⠭");
}

#[test]
fn cap_22_a_1() {
    // from WIRIS
    let expr = "<math><mo>&#x25B3;</mo><mo>&#xA0;</mo><mi>A</mi><mi>B</mi><mi>C</mi></math>";
    test_braille("Nemeth", expr, "⠫⠞⠀⠠⠁⠠⠃⠠⠉");
}

#[test]
fn letter_26_b_19() {
    let expr = "<math><mo>(</mo>
            <mi mathvariant='normal'>l</mi><mo>,</mo>
            <mi mathvariant='normal'>m</mi><mo>,</mo>
            <mi mathvariant='normal'>n</mi><mo>,</mo>
            <mtext>are in set&#xa0;</mtext>
            <mi mathvariant='normal'>R</mi>
        <mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠇⠠⠀⠍⠠⠀⠝⠠⠀⠁⠗⠑⠀⠊⠝⠀⠎⠑⠞⠀⠠⠗⠾");
}

#[test]
fn boldface_32_b_3() {
    let expr = "<math><mn mathvariant='bold'>345</mn></math>";
    test_braille("Nemeth", expr, "⠸⠼⠒⠲⠢");
}

#[test]
fn boldface_32_b_6() {
    let expr = "<math><mn>𝟒35</mn></math>";
    test_braille("Nemeth", expr, "⠸⠼⠲⠼⠒⠢");
}

#[test]
fn punct_37_1_2() {
    let expr = "<math>
            <mover> <mi>velocity</mi> <mo>_</mo> </mover>
            <mtext>.</mtext>
        </math>";
    test_braille("Nemeth", expr, "⠐⠧⠑⠇⠕⠉⠊⠞⠽⠣⠱⠻⠸⠲");
}

#[test]
fn punct_37_2_2() {
    let expr = "<math><mtext>“</mtext> <mn>49</mn> <mtext>”</mtext></math>";
    test_braille("Nemeth", expr, "⠦⠼⠲⠔⠸⠴");
}

#[test]
fn punct_37_7_1() {
    let expr = "<math><mi>a</mi><mo>,</mo><mi>b</mi><mo>,</mo><mi>c</mi><mo>.</mo></math>";
    test_braille("Nemeth", expr, "⠁⠠⠀⠃⠠⠀⠉⠸⠲");
}

#[test]
fn dash_42_4() {
    let expr = "<math><mfrac><mo>&#x2015;</mo><mn>15</mn></mfrac><mo>=</mo><mfrac><mn>2</mn><mn>3</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠤⠤⠤⠤⠀⠌⠂⠢⠼⠀⠨⠅⠀⠹⠆⠌⠒⠼");
}
#[test]
fn dash_42_6() {
    let expr = "<math><mo>$</mo><mn>2</mn><mo>+</mo><mo>$</mo><mn>3</mn><mo>=</mo><mo>$</mo><mo>&#x2015;</mo></math>";
    test_braille("Nemeth", expr, "⠈⠎⠆⠬⠈⠎⠒⠀⠨⠅⠀⠈⠎⠤⠤⠤⠤");
}

#[test]
fn ellipsis_43_b_3() {
    let expr = "<math>
        <msubsup><mi>p</mi><mn>1</mn><msub><mi>&#x3B1;</mi><mn>1</mn></msub></msubsup>
        <mo>&#x2026;</mo>
        <msubsup><mi>p</mi><mi>r</mi><msub><mi>&#x3B1;</mi><mi>r</mi></msub></msubsup>
        </math>";
    test_braille("Nemeth", expr, "⠏⠂⠘⠨⠁⠘⠰⠂⠐⠄⠄⠄⠀⠏⠰⠗⠘⠨⠁⠘⠰⠗");
}

#[test]
fn simple_frac_62_a_3() {
    let expr = "<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mi>c</mi></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠁⠬⠃⠌⠉⠼");
}

#[test]
fn beveled_frac_62_b_1() {
    let expr = "<math><mfrac bevelled='true'>
        <mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow>
        <mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow>
        </mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠁⠬⠃⠸⠌⠉⠬⠙⠼");
}

#[test]
fn mixed_frac_63_a_1() {
    let expr = "<math><mn>4</mn><mfrac><mn>3</mn><mn>8</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠼⠲⠸⠹⠒⠌⠦⠸⠼");
}

#[test]
fn mixed_frac_64_2() {
    let expr = "<math><mn>4</mn><mn>3</mn><mo>/</mo><mn>8</mn></math>";
    test_braille("Nemeth", expr, "⠼⠲⠸⠹⠒⠸⠌⠦⠸⠼");
}

#[test]
fn complex_frac_66_1() {
    let expr = "<math><mfrac><mfrac><mn>3</mn><mn>8</mn></mfrac><mn>5</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠠⠹⠹⠒⠌⠦⠼⠠⠌⠢⠠⠼");
}

#[test]
fn non_hyper_complex_frac_67_1() {
    let expr = "<math><mfrac><mi>a</mi><msup><mi>b</mi>
            <mfrac>
                <mfrac><mn>3</mn><mn>4</mn></mfrac>
                <mfrac><mn>5</mn><mn>6</mn></mfrac>
            </mfrac>
        </msup></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠁⠌⠃⠘⠠⠹⠹⠒⠌⠲⠼⠠⠌⠹⠢⠌⠖⠼⠠⠼⠐⠼");
}

#[test]
fn hyper_complex_frac_68_a_1() {
    // book uses 2d layout -- linear is used here
    let expr = "<math><mfrac>
            <mfrac>
            <mrow><mn>1</mn><mfrac><mn>1</mn><mn>4</mn></mfrac></mrow>
            <mrow><mn>1</mn><mfrac><mn>3</mn><mn>5</mn></mfrac></mrow>
            </mfrac>
            <mn>5</mn>
        </mfrac></math>";
    test_braille("Nemeth", expr, "⠠⠠⠹⠠⠹⠂⠸⠹⠂⠌⠲⠸⠼⠠⠌⠂⠸⠹⠒⠌⠢⠸⠼⠠⠼⠠⠠⠌⠢⠠⠠⠼");
}

#[test]
fn nested_sup_74_b_1() {
    let expr = "<math><msup><mi>n</mi><msup><mi>x</mi><mi>y</mi></msup></msup></math>";
    test_braille("Nemeth", expr, "⠝⠘⠭⠘⠘⠽");
}

#[test]
fn nested_sup_mmultiscripts_74_b_1() {
    let expr = "<math><mmultiscripts><mi>n</mi><none/><msup><mi>x</mi><mi>y</mi></msup></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠝⠘⠭⠘⠘⠽");
}

#[test]
fn nested_sup_74_b_4() {
    let expr = "<math><msub><mi>n</mi><msub><mi>x</mi><mi>y</mi></msub></msub></math>";
    test_braille("Nemeth", expr, "⠝⠰⠭⠰⠰⠽");
}

#[test]
fn nested_sub_sup_74_c_5() {
    let expr = "<math><msup><mi>n</mi><msub><mi>x</mi><msub><mi>a</mi><mi>j</mi></msub></msub></msup></math>";
    test_braille("Nemeth", expr, "⠝⠘⠭⠘⠰⠁⠘⠰⠰⠚");
}

#[test]
fn as_multiscript_nested_sub_sup_74_c_5() {
    let expr = "<math><mmultiscripts><mi>n</mi><none/><msub><mi>x</mi><msub><mi>a</mi><mi>j</mi></msub></msub></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠝⠘⠭⠘⠰⠁⠘⠰⠰⠚");
}

#[test]
fn left_sup_75_1() {
    let expr = "<math><mmultiscripts><mi>n</mi><mprescripts/><none/><mi>x</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠘⠭⠐⠝");
}

#[test]
fn left_sup_75_4() {
    let expr = "<math><mmultiscripts><mi>n</mi><mi>y</mi><none/><mprescripts/><mi>x</mi><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠰⠭⠐⠝⠰⠽");
}

#[test]
fn left_sup_75_7() {
    let expr = "<math><mmultiscripts><mi>x</mi><mprescripts/><none/>
            <mmultiscripts><mi>n</mi><mprescripts/><mi>a</mi><none/></mmultiscripts>
        </mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠘⠰⠁⠘⠝⠐⠭");
}

#[test]
fn left_sup_75_8() {
    let expr = "<math><mmultiscripts><mi>x</mi><mprescripts/><msup><mi>n</mi><mi>a</mi></msup><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠰⠝⠰⠘⠁⠐⠭");
}

#[test]
fn left_sup_75_12() {
    let expr = "<math><msup><mi>p</mi><mi>b</mi></msup><mmultiscripts><mi>x</mi><mprescripts/><none/><mi>c</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠏⠘⠃⠘⠉⠐⠭");
}


#[test]
fn german_base_77_4_3() {
    let expr = "<math><msub> <mi>𝔄</mi> <mn>1</mn> </msub></math>";
    test_braille("Nemeth", expr, "⠸⠠⠁⠂");
}

#[test]
fn prime_77_4_4() {
    let expr = "<math><msub> <msup><mi>x</mi><mo>'</mo></msup> <mn>1</mn> </msub></math>";
    test_braille("Nemeth", expr, "⠭⠄⠂");
}

#[test]
fn prescript_77_4_6() {
    let expr = "<math><mmultiscripts> <mi>x</mi> <mprescripts/> <mn>3</mn><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠰⠒⠐⠭");
}

#[test]
fn prescript_77_4_7() {
    let expr = "<math><msub><mi>x</mi><msub><mi>i</mi><mn>1</mn></msub></msub></math>";
    test_braille("Nemeth", expr, "⠭⠰⠊⠰⠰⠂");
}

#[test]
fn log_77_4_8() {
    let expr = "<math><msub><mi>log</mi><mn>2</mn></msub> <mi>x</mi></math>";
    test_braille("Nemeth", expr, "⠇⠕⠛⠆⠀⠭");
}

#[test]
fn mmultiscripts_77_4_10() {
    // not  right to use msub because the subscripts should be aligned -- nested msub's won't align subscripts -- mmultiscripts solves this
    let expr = "<math>
    <mmultiscripts>
        <mrow>
            <mo>(</mo>
            <mi mathvariant='normal'>C</mi>
            <mmultiscripts>  <mi mathvariant='normal'>O</mi> <mn>3</mn> <none/> </mmultiscripts>
            <mo>)</mo>
        </mrow>
        <mn>2</mn>
        <none/>
    </mmultiscripts>
</math>
";
    test_braille("Nemeth", expr, "⠷⠠⠉⠠⠕⠒⠾⠰⠆");
}

#[test]
fn word_77_4_12() {
    let expr = "<math><msub><mi>seven</mi><mn>3</mn></msub></math>";
    test_braille("Nemeth", expr, "⠎⠑⠧⠑⠝⠰⠒");
}

#[test]
fn comma_number_77_4_20() {
    // mathml from mathjax output for "x_{10,000}"
    let expr = "<math><msub><mi>x</mi><mrow><mn>10</mn><mo>,</mo><mn>000</mn></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠭⠂⠴⠠⠴⠴⠴");
}

#[test]
fn sum_77_4_23() {
    let expr = "<math><msubsup><mo>&#x2211;</mo><mn>0</mn><mi>n</mi></msubsup><msub><mi>a</mi><mi>k</mi></msub></math>";
    test_braille("Nemeth", expr, "⠨⠠⠎⠴⠘⠝⠐⠁⠰⠅");
}

#[test]
fn product_77_4_24() {
    let expr = "<math><msubsup><mo>&#x220F;</mo><mn>0</mn><mi>n</mi></msubsup><msub><mi>a</mi><mi>k</mi></msub></math>";
    test_braille("Nemeth", expr, "⠨⠠⠏⠴⠘⠝⠐⠁⠰⠅");
}

#[test]
fn integral_77_4_26() {
    let expr = "<math>
            <msubsup>
                <mo>&#x222B;</mo>
                <mn>0</mn>
                <msqrt><mn>1</mn><mo>-</mo><msup><mi>x</mi><mn>2</mn></msup></msqrt> 
            </msubsup>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mi>d</mi><mi>x</mi></mrow>
        </math>";
    test_braille("Nemeth", expr, "⠮⠰⠴⠘⠜⠂⠤⠭⠘⠘⠆⠘⠻⠐⠋⠷⠭⠾⠙⠭");
}

#[test]
fn comma_space_78_1() {
    // WIRIS output when typed with spaces (which I doubt people do)
    let expr = "<math><msub><mi>x</mi>
         <mrow><mi>i</mi><mo>,</mo><mo>&#xA0;</mo><mi>j</mi><mo>,</mo><mo>&#xA0;</mo><mi>k</mi></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠭⠰⠊⠪⠚⠪⠅");
}

#[test]
fn comma_78_2() {
    let expr = "<math><msub><mi>x</mi><mrow><mo>(</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo>)</mo></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠭⠰⠷⠁⠪⠃⠾");
}

#[test]
fn comma_78_2_invisible() { // test with invisible comma -- should be the same (issue #40)
    let expr = "<math><msub><mi>x</mi><mrow><mo>(</mo><mi>a</mi><mo>&#x2063;</mo><mi>b</mi><mo>)</mo></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠭⠰⠷⠁⠪⠃⠾");
}

#[test]
fn comma_78_3() {
    let expr = "<math><msub><mi>x</mi><mrow><mn>1</mn><mo>,</mo><mn>2</mn></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠭⠰⠂⠪⠆");
}

#[test]
fn comma_78_6() {
    // WIRIS output when typed with spaces
    let expr = "<math><mo>(</mo><mi>x</mi><mo>,</mo><mo>&#xA0;</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠭⠠⠀⠽⠾");
}

#[test]
fn nested_super_79_a_2() {
    let expr = "<math><msub><mi>x</mi><mi>a</mi></msub><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></math>";
    test_braille("Nemeth", expr, "⠭⠰⠁⠐⠬⠽⠘⠆");
}

#[test]
fn nested_super_79_a_3() {
    let expr = "<math><mfrac><mrow><msup><mi>e</mi><mrow><msup><mi>x</mi><mn>2</mn></msup></mrow></msup></mrow><mn>2</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠑⠘⠭⠘⠘⠆⠐⠌⠆⠼");
}

#[test]
fn punctuation_after_sup_79_b_2() {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mo>,</mo><msup><mi>x</mi><mn>3</mn></msup></math>";
    test_braille("Nemeth", expr, "⠭⠘⠆⠠⠀⠭⠘⠒");
}

#[test]
fn comma_in_number_in_sup_79_b_3() {
    // bad mn from Wiris
    let expr = "<math><msup><mi>x</mi><mrow><mn>10</mn><mo>,</mo><mn>000</mn></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠭⠘⠂⠴⠠⠴⠴⠴");
}

#[test]
fn comma_in_sup_79_b_4() {
    let expr = "<math><msub><mi>x</mi><mrow><mi>i</mi><mo>,</mo><mi>j</mi></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠭⠰⠊⠪⠚");
}


#[test]
fn comma_ellipsis_in_sub_79_b_5() {
    let expr = "<math> <msub><mi>P</mi>
        <mrow><msub><mi>n</mi><mn>1</mn></msub>
          <mo>,</mo>
          <msub><mi>n</mi><mn>2</mn></msub>
          <mo>,</mo><mo>&#x2026;</mo>
          </mrow></msub></math>";
    test_braille("Nemeth", expr, "⠠⠏⠰⠝⠰⠰⠂⠰⠪⠝⠰⠰⠆⠰⠪⠀⠄⠄⠄");
}
#[test]
fn text_after_sup_79_c_3() {
    // bad mn from Wiris; also &A0;
    let expr = "<math><mn>6</mn><mo>.</mo><mn>696</mn><mo>×</mo><msup><mn>10</mn><mn>8</mn></msup><mo>&#xA0;</mo><mtext>mph</mtext></math>";
    test_braille("Nemeth", expr, "⠼⠖⠨⠖⠔⠖⠈⠡⠂⠴⠘⠦⠀⠍⠏⠓");
}

#[test]
fn table_entry_after_sup_79_c_4() {
    let expr = "<math><mrow><mo>(</mo>
        <mtable><mtr>
          <mtd><msup><mi>x</mi><mn>2</mn></msup></mtd>
          <mtd><msup><mi>y</mi><mn>2</mn></msup></mtd>
        </mtr></mtable>
        <mo>)</mo></mrow></math>";
    test_braille("Nemeth", expr, "⠷⠭⠘⠆⠀⠽⠘⠆⠐⠾");
}

#[test]
fn nested_super_space_79_d_3() {
    let expr = "<math><msup><mi>cos</mi><mn>2</mn></msup><mi>x</mi></math>";
    test_braille("Nemeth", expr, "⠉⠕⠎⠘⠆⠀⠭");
}

#[test]
fn nested_super_space_79_d_7() {
    let expr = "<math><mrow><msup><mi>e</mi><mrow><msup><mi>cos</mi><mn>2</mn></msup><mi>x</mi></mrow></msup></mrow></math>";
    test_braille("Nemeth", expr, "⠑⠘⠉⠕⠎⠘⠘⠆⠀⠭");
}

#[test]
fn nested_sup_sup_space_79_d_9() {
    let expr = "<math><msup><mi>q</mi><mrow><msub><mi>log</mi><mi>q</mi></msub><mi>a</mi></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠟⠘⠇⠕⠛⠘⠰⠟⠀⠁");
}

#[test]
fn whitespace_in_sup_79_e_1() {
    let expr = "<math><msup><mi>e</mi><mn>3.14159 26535</mn></msup></math>";
    test_braille("Nemeth", expr, "⠑⠘⠒⠨⠂⠲⠂⠢⠔⠀⠆⠖⠢⠒⠢");
}

#[test]
fn ellipsis_level_79_f_1() {
    let expr = "<math><msup><mi>x</mi>
        <mrow><mn>1</mn><mo>+</mo><mn>1</mn><mo>/</mo><mn>2</mn><mo>+</mo><mn>1</mn><mo>/</mo><mn>3</mn><mo>+</mo>
        <mo>…</mo><mo>+</mo><mn>1</mn><mo>/</mo><mi>n</mi></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠭⠘⠂⠬⠂⠸⠌⠆⠬⠂⠸⠌⠒⠬⠀⠄⠄⠄⠀⠬⠂⠸⠌⠝");
}

#[test]
fn comparison_79_g_2() {
    let expr = "<math><msup><mn>2</mn><mi>x</mi></msup><mo>&lt;</mo><msup><mn>3</mn><mi>x</mi></msup></math>";
    test_braille("Nemeth", expr, "⠼⠆⠘⠭⠀⠐⠅⠀⠼⠒⠘⠭");
}

#[test]
fn sub_ind_79_g_4() {
    let expr = "<math><msub><mo>∫</mo><mrow><mi>u</mi><mo>=</mo><mi>a</mi></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠮⠰⠥⠀⠰⠨⠅⠀⠁");
}

#[test]
fn baseline_80_a_1() {
    let expr = "<math><msqrt><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠘⠆⠐⠬⠽⠘⠆⠐⠻");
}

#[test]
fn superscript_80_a_2() {
    let expr = "<math><msup><mi>e</mi><msqrt><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></msqrt></msup></math>";
    test_braille("Nemeth", expr, "⠑⠘⠜⠭⠘⠘⠆⠘⠬⠽⠘⠘⠆⠘⠻");
}

#[test]
fn sub_ind_80_b_3() {
    let expr = "<math><msub><mi>P</mi><mn>1</mn></msub><mmultiscripts><mi>Q</mi><mprescripts/><mn>2</mn><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠠⠏⠂⠰⠆⠐⠠⠟");
}

#[test]
fn sub_ind_mmultiscripts_80_b_3() {
    let expr = "<math><mmultiscripts><mi>P</mi><mn>1</mn><none/></mmultiscripts>
                           <mmultiscripts><mi>Q</mi><mprescripts/><mn>2</mn><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠠⠏⠂⠰⠆⠐⠠⠟");
}

#[test]
fn sub_ind_80_b_4() {
    let expr = "<math><msub><mi>A</mi><mrow><mover><mi>x</mi><mo>~</mo></mover><mo>+</mo><mover><mi>y</mi><mo>~</mo></mover></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠠⠁⠰⠐⠭⠣⠈⠱⠻⠬⠰⠐⠽⠣⠈⠱⠻");
}

#[test]
fn numeric_sub_81_a_1() {
    let expr = "<math><mo>(</mo><msub><mi>x</mi><mn>1</mn></msub><mo>+</mo><mn>1</mn><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠭⠂⠬⠂⠾");
}

#[test]
fn msubsup_82_a_1() {
    let expr = "<math><msubsup><mi>x</mi><mi>a</mi><mi>n</mi></msubsup></math>";
    test_braille("Nemeth", expr, "⠭⠰⠁⠘⠝");
}

#[test]
fn msubsup_82_a_3() {
    let expr = "<math><msubsup><mi>x</mi><mn>1</mn><mn>2</mn></msubsup></math>";
    test_braille("Nemeth", expr, "⠭⠂⠘⠆");
}

#[test]
fn mmultiscripts_82_a_1() {
    let expr = "<math><mmultiscripts><mi>x</mi><mi>a</mi><mi>n</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠭⠰⠁⠘⠝");
}

#[test]
fn mmultiscripts_82_a_2() {
    let expr = "<math><mmultiscripts><mi>x</mi><mprescripts/><mi>a</mi><mi>n</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠰⠁⠘⠝⠐⠭");
}

#[test]
fn mmultiscripts_82_a_3() {
    let expr = "<math><mmultiscripts><mi>x</mi><mn>1</mn><mn>2</mn></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠭⠂⠘⠆");
}

#[test]
fn sub_sup_82_b_1() {
    let expr = "<math><msub><msup><mi>a</mi><mi>n</mi></msup><mi>m</mi></msub></math>";
    test_braille("Nemeth", expr, "⠁⠘⠝⠐⠰⠍");
}

#[test]
fn sub_sup_82_b_2() {
    let expr = "<math><msup><msub><mi>a</mi><mi>m</mi></msub><mi>n</mi></msup></math>";
    test_braille("Nemeth", expr, "⠁⠰⠍⠐⠘⠝");
}

#[test]
fn mmultiscripts_82_b_1() {
    let expr = "<math><mmultiscripts><mi>a</mi><none/><mi>n</mi><mi>m</mi><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠁⠘⠝⠐⠰⠍");
}

#[test]
fn mmultiscripts_82_b_2() {
    let expr = "<math><mmultiscripts><mi>a</mi><mi>m</mi><none/><none/><mi>n</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠁⠰⠍⠐⠘⠝");
}

#[test]
fn mmultiscripts_82_b_3() {
    let expr = "<math><mmultiscripts><mi>x</mi><mprescripts/><none/><mi>a</mi><mi>b</mi><none/></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠘⠁⠐⠰⠃⠐⠭");
}

#[test]
fn mmultiscripts_82_b_4() {
    let expr = "<math><mmultiscripts><mi>x</mi><mprescripts/><mi>b</mi><none/><none/><mi>a</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠰⠃⠐⠘⠁⠐⠭");
}

#[test]
fn mmultiscripts_82_b_5() {
    let expr = "<math><mmultiscripts><mi>x</mi><mn>1</mn><none/><none/><mn>2</mn></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠭⠂⠐⠘⠆");
}

#[test]
fn mmultiscripts_82_b_6() {
    let expr = "<math><mmultiscripts><mi>x</mi><mi>a</mi><mo>'</mo><none/><mi>b</mi></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠭⠄⠰⠁⠐⠘⠃");
}

#[test]
fn prime_83_b_1() {
    let expr = "<math><msubsup><mi>x</mi><mi>a</mi><mo>'</mo></msubsup></math>";
    test_braille("Nemeth", expr, "⠭⠄⠰⠁");
}

#[test]
fn prime_83_b_4() {
    let expr = "<math><msubsup> <msup><mi>x</mi><mo>''</mo></msup> <mn>1</mn> <mn>3</mn></msubsup></math>";
    test_braille("Nemeth", expr, "⠭⠄⠄⠂⠘⠒");
}

#[test]
fn prime_mmultiscripts_83_b_4() {
    let expr = "<math><mmultiscripts> <mi>x</mi> <none/><mo>''</mo> <mn>1</mn><mn>3</mn></mmultiscripts></math>";
    test_braille("Nemeth", expr, "⠭⠄⠄⠂⠘⠒");
}


#[test]
fn underbar_86_a_1() {
    // Note: NFB lessons added a contracted form (lesson 12.5.1.b)
    let expr = "<math><munder><mi>x</mi><mo>&#xAF;</mo></munder></math>";
    test_braille("Nemeth", expr, "⠭⠩⠱");
}

#[test]
fn menclose_86_a_1() {
    // Note: NFB lessons added a contracted form (lesson 12.5.1.b)
    let expr = "<math><menclose notation='bottom'><mi>x</mi></menclose></math>";
    test_braille("Nemeth", expr, "⠭⠩⠱");
}

#[test]
fn lim_86_a_3() {
    let expr = "<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><mn>0</mn></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠐⠇⠊⠍⠩⠭⠀⠫⠕⠀⠼⠴⠻⠀⠋⠷⠭⠾");
}

#[test]
fn overbar_86_a_4() {
    let expr = "<math><mover><msup><mi>x</mi><mn>2</mn></msup><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠘⠆⠐⠣⠱⠻");
}

#[test]
fn menclose_86_a_4() {
    let expr = "<math><menclose notation='top'><msup><mi>x</mi><mn>2</mn></msup></menclose></math>";
    test_braille("Nemeth", expr, "⠐⠭⠘⠆⠐⠣⠱⠻");
}

#[test]
fn overbar_86_a_5() {
    let expr = "<math><mover><msup><mi>x</mi><mn>2</mn></msup><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠘⠆⠐⠣⠱⠻");
}

#[test]
fn mathml_spec_example_86_a() {
    let expr = "<math>
        <munder><mo>(</mo><mo>&#x5F;<!--LOW LINE--></mo></munder>
        <mfrac><mi>a</mi><mi>b</mi></mfrac>
        <mover><mo>)</mo><mo>&#x203E;<!--OVERLINE--></mo></mover>
    </math>";
    test_braille("Nemeth", expr, "⠐⠷⠩⠱⠻⠹⠁⠌⠃⠼⠐⠾⠣⠱⠻");
}

#[test]
fn menclose_lesson_12_5_5_5() {
    // this is what WIRIS exports
    let expr = "<math><mi>A</mi><mo>(</mo><menclose notation='bottom'><mi>s</mi></menclose><mi>n</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠁⠷⠎⠩⠱⠝⠾");
}

#[test]
fn munder_lesson_12_5_5_5() {
    let expr = "<math><mi>A</mi><mo>(</mo><munder><mi>s</mi><mo>&#xAF;</mo></munder><mi>n</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠁⠷⠎⠩⠱⠝⠾");
}

#[test]
fn overbar_86_b_1() {
    let expr = "<math><mover><mi>x</mi><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠭⠱");
}

#[test]
fn menclose_86_b_1() {
    let expr = "<math><menclose notation='top'><mi>x</mi></menclose></math>";
    test_braille("Nemeth", expr, "⠭⠱");
}

#[test]
fn primed_86_b_6() {
    let expr = "<math><msup><mrow><mover><mi>x</mi><mo>&#xAF;</mo></mover></mrow><mo>&#x2032;</mo></msup></math>";
    test_braille("Nemeth", expr, "⠭⠱⠄");
}

#[test]
fn menclose_primed_86_b_6() {
    let expr = "<math><msup><menclose notation='top'><mi>x</mi></menclose><mo>&#x2032;</mo></msup></math>";
    test_braille("Nemeth", expr, "⠭⠱⠄");
}

#[test]
fn overbar_86_b_10() {
    let expr = "<math><mn>3</mn><mo>.</mo><mn>5</mn><mover><mn>4</mn><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠼⠒⠨⠢⠲⠱");
}

#[test]
fn overbar_86_b_11() {
    let expr = "<math><mover><mfenced>
            <mrow><mover><mi>a</mi><mo>&#xAF;</mo></mover><mi mathvariant='bold'>A</mi><mo>+</mo>
                <mover><mi>b</mi><mo>&#xAF;</mo></mover><mi mathvariant='bold'>B</mi></mrow>
        </mfenced><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠐⠷⠁⠱⠸⠰⠠⠁⠬⠃⠱⠸⠰⠠⠃⠾⠣⠱⠻");
}

#[test]
fn menclose_86_b_11() {
    let expr = "<math><menclose notation='top'><mfenced>
            <mrow><menclose notation='top'><mi>a</mi></menclose><mi mathvariant='bold'>A</mi><mo>+</mo>
            <menclose notation='top'><mi>b</mi></menclose><mi mathvariant='bold'>B</mi></mrow>
        </mfenced></menclose></math>";
    test_braille("Nemeth", expr, "⠐⠷⠁⠱⠸⠰⠠⠁⠬⠃⠱⠸⠰⠠⠃⠾⠣⠱⠻");
}

#[test]
fn order2_overbar_87_a_1() {
    let expr = "<math><mover>
            <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
            <mover><mo>&#xAF;</mo><mrow><mi>a</mi><mo>=</mo><mn>3</mn></mrow></mover>
        </mover></math>";
    // this is a possible other interpretation of 87a(1), but I think the above is the right one
    // let expr = "<math><mover>
    //         <mover><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>&#xAF;</mo></mover>
    //         <mrow><mi>a</mi><mo>=</mo><mn>3</mn></mrow>
    //      </mover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠬⠽⠣⠱⠣⠣⠁⠀⠨⠅⠀⠼⠒⠻");
}

#[test]
fn bar_above_and_below_88_1() {
    let expr = "<math><munderover>
            <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
            <mo>&#xAF;</mo>
            <mo>&#xAF;</mo>
        </munderover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠬⠽⠩⠱⠣⠱⠻");
}

#[test]
fn above_and_below_88_2() {
    let expr = "<math><munderover><mo>&#x2211;</mo><mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow><mo>&#x221E;</mo></munderover>
                            <mfrac><mn>1</mn><msup><mn>2</mn><mi>n</mi></msup></mfrac><mo>=</mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠐⠨⠠⠎⠩⠝⠀⠨⠅⠀⠼⠂⠣⠠⠿⠻⠹⠂⠌⠆⠘⠝⠐⠼⠀⠨⠅⠀⠼⠂");
}

#[test]
fn menclose_top_bottom_88_1() {
    let expr = "<math><menclose notation='top bottom'><mi>x</mi><mo>+</mo><mi>y</mi></menclose></math>";
    test_braille("Nemeth", expr, "⠐⠭⠬⠽⠩⠱⠣⠱⠻");
}

#[test]
fn binomial_90_1() {
    let expr = "<math><mo>(</mo><mfrac linethickness='0'><mi>n</mi><mi>k</mi></mfrac><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠝⠩⠅⠾");
}

#[test]
fn modifier_in_script_91_1() {
    let expr = "<math><msub><mi>A</mi><mover><mi>x</mi><mo>~</mo></mover></msub></math>";
    test_braille("Nemeth", expr, "⠠⠁⠰⠐⠭⠣⠈⠱⠻");
}

#[test]
fn arrow_96_1() {
    let expr = "<math>
        <mover>
        <mrow><mi mathvariant='normal'>A</mi> <mi mathvariant='normal'>B</mi></mrow>
        <mo>→</mo>
        </mover>
    </math>";
    test_braille("Nemeth", expr, "⠐⠠⠁⠠⠃⠣⠫⠕⠻");
}

#[test]
fn arrow_96_4() {
    let expr = "<math>
        <mi>X</mi>
        <mover>
        <mo>→</mo>
        <mrow><mi>f</mi> <mo>∘</mo><mi>g</mi></mrow>
        </mover>
        <mi>Y</mi>
    </math>";
    test_braille("Nemeth", expr, "⠠⠭⠀⠐⠫⠒⠒⠕⠣⠋⠨⠡⠛⠻⠀⠠⠽");
}

#[test]
fn bar_97_b_1() {
    let expr = "<math><mo>.</mo><mover><mn>3</mn><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠼⠨⠒⠱");
}

#[test]
fn menclose_bar_97_b_1() {
    let expr = "<math><mo>.</mo><menclose notation='top'><mn>3</mn></menclose></math>";
    test_braille("Nemeth", expr, "⠼⠨⠒⠱");
}

#[test]
fn menclose_bar_97_b_3() {
    let expr = "<math><mn>3.57</mn><mover><mn>29</mn><mo stretchy='true'>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠼⠒⠨⠢⠶⠐⠆⠔⠣⠱⠻");
}

#[test]
fn carrot_98_1() {
    let expr = "<math><mover><mi>x</mi><mo>^</mo></mover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠣⠸⠣⠻");
}

#[test]
fn dots_99_a_1() {
    init_logger();
    let expr = "<math><mo>.</mo><mover><mn>3</mn><mo>&#x2D9;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠼⠨⠐⠒⠣⠡⠻");
}

#[test]
fn dots_99_a_2() {
    let expr = "<math><mo>.</mo><mover><mn>1</mn><mo>&#x2D9;</mo></mover><mover><mn>3</mn><mo>&#x2D9;</mo></mover><mover><mn>5</mn><mo>&#x2D9;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠼⠨⠐⠂⠒⠢⠣⠡⠻");
}

#[test]
fn dots_99_a_3() {
    let expr = "<math><mn>.13</mn><mover><mn>5</mn><mo>&#x2D9;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠼⠨⠂⠒⠐⠢⠣⠡⠻");
}

#[test]
fn sqrt_103_a_2() {
    let expr = "<math><msqrt><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠬⠽⠻");
}

#[test]
fn sqrt_103_a_4() {
    let expr = "<math><msqrt>
            <msup><mi>x</mi><mn>2</mn></msup>
            <mo>+</mo>
            <msup><mi>y</mi><mn>2</mn></msup>
        </msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠘⠆⠐⠬⠽⠘⠆⠐⠻");
}

#[test]
fn sqrt_103_b_2() {
    let expr = "<math><mo>√</mo><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠜⠷⠭⠬⠽⠾");
}

#[test]
fn root_104_iii_1() {
    let expr = "<math><mroot><mn>2</mn><mn>3</mn></mroot></math>";
    test_braille("Nemeth", expr, "⠣⠒⠜⠆⠻");
}

#[test]
fn root_104_iii_4() {
    let expr = "<math><mroot>
            <mrow><mi>p</mi><mo>+</mo><mi>q</mi></mrow>
            <mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow>
        </mroot></math>";
    test_braille("Nemeth", expr, "⠣⠍⠬⠝⠜⠏⠬⠟⠻");
}

#[test]
fn nested_sqrt_105_1() {
    let expr = "<math><msqrt><mi>x</mi><mo>+</mo>
            <msqrt><mi>x</mi><mo>+</mo><mi>y</mi></msqrt>
            <mo>+</mo><mi>z</mi></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠬⠨⠜⠭⠬⠽⠨⠻⠬⠵⠻");
}

#[test]
fn nested_root_105_2() {
    let expr = "<math><mroot>
    <mrow>
        <msup> <mi>x</mi><mn>2</mn> </msup>
        <mo>+</mo>
        <mroot>
            <mrow>
                <msup> <mi>x</mi> <mn>2</mn> </msup>
                <mo>+</mo>
                <msup> <mi>y</mi> <mn>2</mn>  </msup>
            </mrow>
            <mn>3</mn>
        </mroot>
        <mo>+</mo>
        <msup> <mi>y</mi> <mn>2</mn> </msup>
    </mrow>
    <mn>3</mn>
</mroot></math>";
    test_braille("Nemeth", expr, "⠣⠒⠜⠭⠘⠆⠐⠬⠨⠣⠒⠜⠭⠘⠆⠐⠬⠽⠘⠆⠐⠨⠻⠬⠽⠘⠆⠐⠻");
}


#[test]
fn nested_sqrt_105_3() {
    let expr = "<math>
        <msqrt> <mroot> <mi>x</mi><mn>3</mn> </mroot> </msqrt>
        <mo>=</mo>
        <mroot> <msqrt><mi>x</mi></msqrt> <mn>3</mn></mroot>
    </math>";
    test_braille("Nemeth", expr, "⠜⠨⠣⠒⠜⠭⠨⠻⠻⠀⠨⠅⠀⠣⠒⠜⠨⠜⠭⠨⠻⠻");
}

#[test]
fn nested_sqrt_105_4() {
    let expr = "<math>
            <msqrt><mi>x</mi><mo>+</mo><msqrt><mi>y</mi><mo>+</mo><msqrt><mi>z</mi></msqrt></msqrt></msqrt>
        </math>";
    test_braille("Nemeth", expr, "⠜⠭⠬⠨⠜⠽⠬⠨⠨⠜⠵⠨⠨⠻⠨⠻⠻");
}

#[test]
fn shape_110_1() {
    let expr = "<math><mo>∠</mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠫⠪⠀⠼⠂");
}

#[test]
fn menclose_111_a_4() {
    let expr = "<math><menclose notation='phasorangle'><mrow><mn>30</mn><mo>&#xB0;</mo></mrow></menclose></math>";
    test_braille("Nemeth", expr, "⠫⠪⠸⠫⠼⠒⠴⠘⠨⠡⠐⠻");
}

#[test]
fn menclose_115_1() {
    let expr = "<math><menclose notation='circle'><mi>A</mi></menclose></math>";
    test_braille("Nemeth", expr, "⠫⠉⠸⠫⠠⠁⠻");
}

#[test]
fn function_space_119_c_3() {
    // this depends upon a canonicalization to get the degree sign into a superscript position
    let expr = "<math><mi>sin</mi><mn>30</mn><mo>&#xB0;</mo><mi>cos</mi><mn>45</mn><mo>&#xB0;</mo>
           <mo>+</mo><mi>cos</mi><mn>30</mn><mo>&#xB0;</mo><mi>sin</mi><mn>45</mn><mo>&#xB0;</mo></math>";
    test_braille("Nemeth", expr, "⠎⠊⠝⠀⠼⠒⠴⠘⠨⠡⠐⠉⠕⠎⠀⠼⠲⠢⠘⠨⠡⠐⠬⠉⠕⠎⠀⠼⠒⠴⠘⠨⠡⠐⠎⠊⠝⠀⠼⠲⠢⠘⠨⠡");
}

#[test]
fn identity_matrix_126() {
    let expr = "<math> <mrow><mo>(</mo> <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr>
      </mtable><mo>)</mo></mrow></math>";
    test_braille("Nemeth", expr, "⠈⠠⠷⠂⠀⠼⠴⠀⠼⠴⠈⠠⠾⠀⠈⠠⠷⠴⠀⠼⠂⠀⠼⠴⠈⠠⠾⠀⠈⠠⠷⠴⠀⠼⠴⠀⠼⠂⠈⠠⠾");
}

#[test]
fn set_vertical_bar_145_1() {
    let expr = "<math><mo>{</mo><mi>x</mi><mo>|</mo><mo>|</mo><mi>x</mi><mo>|</mo><mo>&lt;</mo><mn>10</mn><mo>}</mo></math>";
    test_braille("Nemeth", expr, "⠨⠷⠭⠀⠳⠀⠳⠭⠳⠀⠐⠅⠀⠼⠂⠴⠨⠾");
}

#[test]
fn vertical_bar_145_4() {
    // this test was added in an addendum
    let expr = "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠠⠏⠷⠠⠁⠀⠳⠀⠠⠃⠾");
}

#[test]
fn ratio_151_11() {
    let expr = "<math><mi>a</mi><mo>:</mo><mi>b</mi></math>";
    test_braille("Nemeth", expr, "⠁⠀⠐⠂⠀⠃");
}

#[test]
fn trilinear_not_ratio_151_11() {
    let expr = "<math><mi>a</mi><mo>:</mo><mi>b</mi><mo>:</mo><mi>c</mi></math>";
    test_braille("Nemeth", expr, "⠁⠸⠒⠀⠃⠸⠒⠀⠉");
}

#[test]
fn extension_field_not_ratio_151_11() {
    let expr = "<math><mo>[</mo><mi>K</mi><mo>:</mo><mi>F</mi><mo>]</mo></math>";
    test_braille("Nemeth", expr, "⠈⠷⠠⠅⠸⠒⠀⠠⠋⠈⠾");
}

#[test]
fn prime_172_5() {
    let expr = "<math><msubsup><mi>x</mi><mi>i</mi><mo>'</mo></msubsup></math>";
    test_braille("Nemeth", expr, "⠭⠄⠰⠊");
}

#[test]
fn prime_172_6() {
    let expr = "<math><msubsup><mi>x</mi><mn>1</mn><mo>'</mo></msubsup></math>";
    test_braille("Nemeth", expr, "⠭⠄⠂");
}

#[test]
fn prime_172_8() {
    let expr = "<math><msup><mover><mi>x</mi><mo>&#xAF;</mo></mover><mo>'</mo></msup></math>";
    test_braille("Nemeth", expr, "⠭⠱⠄");
}

#[test]
fn prime_172_9() {
    let expr = "<math><msup><mn>5</mn><mo>'</mo></msup><msup><mn>8</mn><mrow><mo>'</mo><mo>'</mo></mrow></msup></math>";
    test_braille("Nemeth", expr, "⠼⠢⠄⠦⠄⠄");
}

#[test]
fn multipurpose_177_2_1() {
    let expr = "<math> <mi>x5</mi> </math>";
    test_braille("Nemeth", expr, "⠭⠐⠢");
}

#[test]
fn multipurpose_177_2_2() {
    let expr = "<math> <mi>x</mi> <mn>.6</mn> </math>";
    test_braille("Nemeth", expr, "⠭⠐⠨⠖");
}

#[test]
fn multipurpose_177_3_1() {
    let expr = "<math>
            <msub><mi>c</mi><mn>0</mn></msub>
            <msup><mn>10</mn><mn>2</mn></msup>
            <mo>+</mo>
            <msub><mi>c</mi><mn>1</mn></msub>
            <mn>10</mn><mo>+</mo>
            <msub><mi>c</mi><mn>2</mn></msub>
        </math>";
    test_braille("Nemeth", expr, "⠉⠴⠐⠂⠴⠘⠆⠐⠬⠉⠂⠐⠂⠴⠬⠉⠆");
}

#[test]
fn multipurpose_177_5_1() {
    let expr = "<math><mn>0.</mn><msub><mi>a</mi><mn>1</mn></msub><msub><mi>a</mi><mn>2</mn></msub><mo>…</mo></math>";
    test_braille("Nemeth", expr, "⠼⠴⠨⠐⠁⠂⠁⠆⠀⠄⠄⠄");
}

#[test]
fn multipurpose_177_7_1() {
    let expr = "<math><mrow><mo>|</mo><mi>x</mi><mo>|</mo></mrow><mrow><mo>|</mo><mi>y</mi><mo>|</mo></mrow></math>";
    test_braille("Nemeth", expr, "⠳⠭⠳⠐⠳⠽⠳");
}

#[test]
fn lesson_11_24_1() {
    let expr = "<math><menclose notation='roundedbox'><msup><mi>x</mi><mn>2</mn></msup></menclose></math>";
    test_braille("Nemeth", expr, "⠫⠅⠭⠘⠆⠐⠻");
}

#[test]
fn ms_38_4_8() {
    let expr = "<math><mo>(</mo><ms lquote='“' rquote='”'>three</ms><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠸⠦⠞⠓⠗⠑⠑⠴⠾");
}

#[test]
fn ms() {
    let expr = "<math><ms>a string</ms><mo>,</mo><ms lquote='‘' rquote='’'>another string</ms></math>";
    // Not 100% sure this is the right output -- I am a little skeptical of "⠄⠄" being the braille for '"'
    // Note: no punct indicator after word (see 38_4_8)
    test_braille("Nemeth", expr, "⠄⠄⠁⠀⠎⠞⠗⠊⠝⠛⠄⠄⠠⠀⠸⠠⠦⠁⠝⠕⠞⠓⠑⠗⠀⠎⠞⠗⠊⠝⠛⠴⠠");
}

#[test]
fn full_binomial() {
    let expr = "<math>
    <mo stretchy='false'>(</mo>
    <mi>x</mi>
    <mo>+</mo>
    <mi>a</mi>
    <msup>
        <mo stretchy='false'>)</mo>
        <mrow>
            <mi>n</mi>
        </mrow>
    </msup>
    <mo>=</mo>
    <munderover>
        <mo>∑</mo>
        <mrow>
            <mi>k</mi>
            <mo>=</mo>
            <mn>0</mn>
        </mrow>
        <mrow>
            <mi>n</mi>
        </mrow>
    </munderover>
    <mrow>
        <mo>(</mo>
        <mfrac linethickness='0'>
            <mi>n</mi>
            <mi>k</mi>
        </mfrac>
        <mo>)</mo>
    </mrow>
    <msup>
        <mi>x</mi>
        <mrow>
            <mi>k</mi>
        </mrow>
    </msup>
    <msup>
        <mi>a</mi>
        <mrow>
            <mi>n</mi>
            <mo>−</mo>
            <mi>k</mi>
        </mrow>
    </msup>
</math>
";
    test_braille("Nemeth", expr, "⠷⠭⠬⠁⠾⠘⠝⠀⠨⠅⠀⠐⠨⠠⠎⠩⠅⠀⠨⠅⠀⠼⠴⠣⠝⠻⠷⠝⠩⠅⠾⠭⠘⠅⠐⠁⠘⠝⠤⠅");
}

// Extra tests targeted at special cases in MathCAT
#[test]
fn number_space_before() {
    let expr = "<math><mtext>&#xA0;</mtext><mn>2</mn></math>";
    test_braille("Nemeth", expr, "⠼⠆");
}

#[test]
fn number_space_after() {
    let expr = "<math><mn>2</mn><mtext>&#xA0;</mtext></math>";
    test_braille("Nemeth", expr, "⠼⠆");
}

#[test]
fn number_space_before_and_after() {
    let expr = "<math><mtext>&#xA0;</mtext><mn>2</mn><mtext>&#xA0;</mtext></math>";
    test_braille("Nemeth", expr, "⠼⠆");
}
