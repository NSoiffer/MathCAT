// Tests from https://www.once.es/servicios-sociales/braille/comision-braille-espanola/documentos-tecnicos/documentos-tecnicos-relacionados-con-el-braille/documentos/b5-signografia-matematica.pdf
// This seems to be official guide.
use crate::common::*;

#[test]
fn letter_number_1_0_1() {
    let expr = "<math><mn>234a5</mn></math>";
    test_braille("CMU", expr, "⠼⠃⠉⠙⠐⠁⠑");
}

#[test]
fn letter_number_1_0_2() {
    let expr = "<math><mn>234ae</mn></math>";
    test_braille("CMU", expr, "⠼⠃⠉⠙⠐⠁⠐⠑");
}

#[test]
fn greek_1_2_1() {
    let expr = "<math><mi>π</mi></math>";
    test_braille("CMU", expr, "⠈⠏");
}

#[test]
fn greek_upper_1_2_2() {
    let expr = "<math><mi>Σ</mi></math>";
    test_braille("CMU", expr, "⠘⠎");
}

#[test]
fn greek_var_1_2_3() {
    let expr = "<math><mi>ϵ</mi></math>";
    test_braille("CMU", expr, "⠈⠬⠑");
}

#[test]
fn gothic_1_3_1() {
    let expr = "<math><mi>𝔞𝔉</mi></math>";
    test_braille("CMU", expr, "⠠⠁⠰⠋");
}

#[test]
#[ignore]   // need to add a transcriber note pref for different fonts 
fn double_struck_1_3_2() {
    let expr = "<math><mi>𝕔𝕎</mi></math>";
    test_braille("CMU", expr, "⠬⠉⠩⠺");
}


#[test]
fn number_3_1_1() {
    let expr = "<math><mn>1.720</mn></math>";
    test_braille("CMU", expr, "⠼⠁⠄⠛⠃⠚");
}

#[test]
fn number_2_2_1() {
    let expr = "<math><mn>31.720</mn></math>";
    test_braille_prefs("CMU", vec![("DecimalSeparators", "."), ("BlockSeparators", ", ")], expr, "⠼⠉⠁⠄⠛⠃⠚");
}

#[test]
fn number_2_2_2() {
    let expr = "<math><mn>3 802 197</mn></math>";
    test_braille_prefs("CMU", vec![("DecimalSeparators", "."), ("BlockSeparators", ", ")], expr, "⠼⠉⠄⠓⠚⠃⠄⠁⠊⠛");
}

#[test]
fn number_2_2_3() {
    let expr = "<math><mn>46781</mn></math>";
    test_braille("CMU", expr, "⠼⠙⠋⠛⠓⠁");
}

#[test]
fn number_2_2_4() {
    let expr = "<math><mo>-</mo><mn>7</mn></math>";
    test_braille("CMU", expr, "⠤⠼⠛");
}

#[test]
fn number_2_2_5() {
    let expr = "<math><mo>-</mo><mn>29</mn></math>";
    test_braille("CMU", expr, "⠤⠼⠃⠊");
}

#[test]
fn number_2_2_6() {
    let expr = "<math><mo>-</mo><mn>25 347</mn></math>";
    test_braille_prefs("CMU", vec![("DecimalSeparators", "."), ("BlockSeparators", ", ")], expr, "⠤⠼⠃⠑⠄⠉⠙⠛");
}

#[test]
fn number_2_3_1() {
    let expr = "<math><mn>3.2</mn></math>";
    test_braille_prefs("CMU", vec![("DecimalSeparators", "."), ("BlockSeparators", ", ")], expr, "⠼⠉⠂⠃");

}

#[test]
fn number_2_3_2() {
    let expr = "<math><mn>3,2</mn></math>";
    test_braille_prefs("CMU", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠉⠂⠃");
}

#[test]
fn number_2_3_3() {
    let expr = "<math><mn>3’2</mn></math>";
    test_braille_prefs("CMU", vec![("DecimalSeparators", "’,"), ("BlockSeparators", ". ")], expr, "⠼⠉⠂⠃");
}

#[test]
#[ignore]
fn number_2_3_3_wiris() {
    let expr = "<math><mn>3</mn><mo>’</mo><mn>2</mn></math>";
    test_braille("CMU", expr, "⠼⠉⠂⠃");
}

#[test]
fn number_2_3_4() {
    let expr = "<math><mn>3,2</mn><mover><mn>54</mn><mo>^</mo></mover></math>";
    test_braille("CMU", expr, "⠼⠉⠂⠃⠂⠑⠙");
}

#[test]
fn number_2_3_5() {
    let expr = "<math><mn>7,</mn><mover><mn>29</mn><mo>^</mo></mover></math>";
    test_braille("CMU", expr, "⠼⠛⠂⠂⠃⠊");
}

#[test]
fn number_2_3_7() {
    let expr = "<math><mn>3,1415</mn><mo>.</mo><mo>.</mo><mo>.</mo></math>";
    test_braille("CMU", expr, "⠼⠉⠂⠁⠙⠁⠑⠄⠄⠄");
}


#[test]
fn numeric_fraction_2_4_1() {
    let expr = "<math><mfrac><mn>3</mn><mn>4</mn></mfrac></math>";
    test_braille("CMU", expr, "⠼⠉⠲⠀");
}

#[test]
fn numeric_fraction_2_4_2() {
    let expr = "<math><mfrac><mn>38</mn><mn>63</mn></mfrac></math>";
    test_braille("CMU", expr, "⠼⠉⠓⠖⠒⠀");
}

#[test]
fn numeric_fraction_2_4_3() {
    let expr = "<math><mn>3</mn><mfrac><mn>1</mn><mn>5</mn></mfrac></math>";
    test_braille("CMU", expr, "⠼⠉⠼⠁⠢⠀");
}

#[test]
fn letter_number_2_5_b_1() {
    let expr = "<math><mn>1B4D</mn></math>";
    test_braille("CMU", expr, "⠼⠁⠨⠃⠙⠨⠙");
}

#[test]
#[ignore]  // really requires a transcriber's note
fn color_2_5_c_2() {
    let expr = "<math><mn>3</mn><mn mathcolor='#FF0000'>4</mn><mn>2</mn><mn mathcolor='#00FF00'>9</mn></math>";
    test_braille("CMU", expr, "⠼⠉⠩⠙⠃⠰⠊");
}


#[test]
fn number_base_2_5_d_1() {
    let expr = "<math><msub><mn>101</mn><mn>2</mn></msub></math>";
    test_braille("CMU", expr, "⠼⠁⠚⠁⠌⠼⠃");
}

#[test]
fn number_set_2_6() {
    // Double Struck N, Z, Q, R, C , I, P, D
    // this combines all the examples in the 3.5 into a set, so not exactly the example in the guide
    let expr = "<math><mo>{</mo>
            <mi>&#x2115;</mi><mo>,</mo><mi>&#x2124;</mi><mo>,</mo><mi>&#x211A;</mi><mo>,</mo>
            <mi>&#x211D;</mi><mo>,</mo><mi>&#x2102;</mi><mo>,</mo><mi>&#x210D;</mi><mo>,</mo>
            <mi>&#x212D;</mi><mo>,</mo><mi>&#x2119;</mi>
        <mo>}</mo></math>";
    test_braille("CMU", expr, "⠐⠇⠸⠝⠀⠂⠸⠵⠀⠂⠸⠟⠀⠂⠸⠗⠀⠂⠸⠉⠀⠂⠸⠓⠀⠂⠸⠕⠀⠂⠸⠏⠸⠂");
}



#[test]
fn script_4_2_1_1() {
    let expr = "<math><mmultiscripts><mi>z</mi><mprescripts/><mi>r</mi><none/></mmultiscripts></math>";
    test_braille("CMU", expr, "⠵⠠⠌⠗");
}

#[test]
fn script_4_2_1_2() {
    let expr = "<math><mmultiscripts><mi>z</mi><mprescripts/><none/><mi>r</mi></mmultiscripts></math>";
    test_braille("CMU", expr, "⠵⠈⠡⠗");
}

#[test]
fn script_4_2_1_3() {
    let expr = "<math><munder><mi>z</mi><mi>r</mi></munder></math>";
    test_braille("CMU", expr, "⠵⠌⠌⠗");
}

#[test]
fn script_4_2_1_4() {
    let expr = "<math><mover><mi>z</mi><mi>r</mi></mover></math>";
    test_braille("CMU", expr, "⠵⠡⠡⠗");
}

#[test]
fn script_4_2_1_5() {
    let expr = "<math><msub><mi>z</mi><mi>r</mi></msub></math>";
    test_braille("CMU", expr, "⠵⠌⠗");
}

#[test]
fn script_4_2_1_6() {
    let expr = "<math><msup><mi>z</mi><mi>r</mi></msup></math>";
    test_braille("CMU", expr, "⠵⠡⠗");
}


#[test]
fn script_4_2_1_7() {
    // z_{n-1}
    let expr = "<math><msub><mi>z</mi><mrow><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msub></math>";
    test_braille("CMU", expr, "⠵⠌⠢⠝⠤⠼⠁⠔");
}

#[test]
fn script_4_2_1_8() {
    let expr = "<math><msup><mi>z</mi><mrow><mi>i</mi><mo>,</mo><mi>j</mi></mrow></msup></math>";
    test_braille("CMU", expr, "⠵⠡⠢⠊⠀⠂⠚⠔");
}

#[test]
fn script_4_2_1_9() {
    // z_{i_r -1}
    let expr = "<math><msub><mi>z</mi><mrow><msub><mi>i</mi><mi>r</mi></msub><mo>-</mo><mn>1</mn></mrow></msub></math>";
    test_braille("CMU", expr, "⠵⠌⠢⠊⠌⠗⠤⠼⠁⠔");
}

#[test]
fn script_4_2_1_10() {
    // z_i_{r-1}
    let expr = "<math><msub><mi>z</mi><msub><mi>i</mi><mrow><mi>r</mi><mo>-</mo><mn>1</mn></mrow></msub></msub></math>";
    test_braille("CMU", expr, "⠵⠌⠊⠌⠢⠗⠤⠼⠁⠔");
}

#[test]
fn script_4_2_1_11() {
    // {}^{n-1}z
    let expr = "<math><mmultiscripts><mi>z</mi><mprescripts/><none/><mrow><mi>n</mi><mo>-</mo><mn>1</mn></mrow></mmultiscripts></math>";
    test_braille("CMU", expr, "⠵⠈⠡⠢⠝⠤⠼⠁⠔");
}

#[test]
fn script_4_2_1_12() {
    // z^{-1/2}
    let expr = "<math><msup><mi>z</mi><mrow><mo>-</mo><mn>1</mn><mo>/</mo><mn>2</mn></mrow></msup></math>";
    test_braille("CMU", expr, "⠵⠡⠤⠼⠁⠆⠀");
}

#[test]
fn script_4_2_1_13() {
    // z_i_0
    let expr = "<math><msub><mi>z</mi><msub><mi>i</mi><mn>0</mn></msub></msub></math>";
    test_braille("CMU", expr, "⠵⠌⠊⠌⠼⠚");
}

#[test]
fn prime_4_3_1_0() {
    let expr = "<math><msup><mi>z</mi><mo>″</mo></msup></math>";
    test_braille("CMU", expr, "⠵⠳⠳");
}

#[test]
fn super_4_3_1_1() {
    // z^{+}
    let expr = "<math><msup><mi>z</mi><mo>+</mo></msup></math>";
    test_braille("CMU", expr, "⠵⠖⠄");
}

#[test]
fn degree_4_3_1_3() {
    let expr = "<math><msup><mi>z</mi><mo>°</mo></msup></math>";
    test_braille("CMU", expr, "⠵⠴⠄");
}

#[test]
fn sup_plus_4_3_1_5() {
    // z^{+++}
    let expr = "<math><msup><mi>z</mi><mrow><mo>+</mo><mo>+</mo><mo>+</mo></mrow></msup></math>";
    test_braille("CMU", expr, "⠵⠖⠖⠖⠄");
}

#[test]
fn sup_grave_4_3_1_6() {
    // z^{``}
    let expr = "<math><msup><mi>z</mi><mrow><mo>`</mo><mo>`</mo></mrow></msup></math>";
    test_braille("CMU", expr, "⠵⠢⠢⠄");
}

#[test]
fn sup_star_minus_4_3_1_7() {
    // z^{*-}
    let expr = "<math><msup><mi>z</mi><mrow><mo>*</mo><mo>-</mo></mrow></msup></math>";
    test_braille("CMU", expr, "⠵⠲⠤⠄");
}

#[test]
fn sup_4_3_1_8() {
    // z^{4*}
    let expr = "<math><msup><mi>z</mi><mrow><mn>4</mn><mo>*</mo></mrow></msup></math>";
    test_braille("CMU", expr, "⠵⠡⠼⠙⠲⠄");
}

#[test]
fn sup_4_3_1_9() {
    // z^{****} -- need to convert to 4*
    let expr = "<math><msup><mi>z</mi><mrow><mo>*</mo><mo>*</mo><mo>*</mo><mo>*</mo></mrow></msup></math>";
    test_braille("CMU", expr, "⠵⠡⠼⠙⠲⠄");
}

#[test]
fn sup_4_3_1_10() {
    // z^\infty
    let expr = "<math><msup><mi>z</mi><mo>&#x221E;</mo></msup></math>";
    test_braille("CMU", expr, "⠵⠡⠼⠳⠄");
}

#[test]
fn bar_4_3_2_1() {
    let expr = "<math><mover><mi>z</mi><mo>&#xAF;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠉⠵");
}

#[test]
fn unicode_bar_4_3_2_1() {
    // Using U+2550
    let expr = "<math><mover><mi>z</mi><mo>&#x2550;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠉⠈⠉⠵");
}

#[test]
fn bar_4_3_2_2() {
    // A double bar probably should be U+2550, but I don't know how to get a MathML generator to create this
    // Here we use nested mover's, which doesn't look great
    let expr = "<math><mover> <mover><mi>Z</mi><mo>&#xAF;</mo></mover> <mo>&#xAF;</mo></mover> </math>";
    test_braille("CMU", expr, "⠈⠉⠈⠉⠨⠵");
}

#[test]
fn tilde_4_3_2_3() {
    let expr = "<math><mover><mi>z</mi><mo>~</mo></mover></math>";
    test_braille("CMU", expr, "⠐⠢⠵");
}

#[test]
fn hat_4_3_2_5() {
    let expr = "<math><mover><mi>z</mi><mo>^</mo></mover></math>";
    test_braille("CMU", expr, "⠰⠒⠵");
}

#[test]
fn frown_4_3_2_5() {
    let expr = "<math><mover><mi>z</mi><mo>&#x23DC;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠒⠵");
}

#[test]
fn greek_dot_4_3_2_10() {
    let expr = "<math><mover><mi>ζ</mi><mo>¨</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠈⠈⠵");
}

#[test]
fn dot_4_3_2_11() {
    // three dots above
    let expr = "<math><mover><mi>r</mi><mo>&#x20DB;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠈⠈⠐⠗");
}

#[test]
fn bar_4_3_2_12() {
    let expr = "<math><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo stretchy=\"false\">&#xAF;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠉⠢⠨⠁⠨⠃⠔");
}

#[test]
fn bar_menclose_4_3_2_12() {
    let expr = "<math><menclose notation=\"top\"><mi>A</mi><mi>B</mi></menclose></math>";
    test_braille("CMU", expr, "⠈⠉⠢⠨⠁⠨⠃⠔");
}

#[test]
fn bar_menclose_4_3_2_13() {
    let expr = "<math><menclose notation=\"top\"><mi>z</mi><mo>''</mo></menclose></math>";
    test_braille("CMU", expr, "⠈⠉⠢⠵⠳⠳⠔");
}

#[test]
fn over_operator_4_3_2_14() {
    let expr = "<math><mover><mi>z</mi><mrow><mo>+</mo><mo>+</mo></mrow></mover></math>";
    test_braille("CMU", expr, "⠵⠡⠡⠖⠖⠄");
}

#[test]
fn wavy_4_3_3_1() {
    let expr = "<math><munder><mi>z</mi><mo>〰</mo></munder></math>";
    test_braille("CMU", expr, "⠂⠢⠵");
}

#[test]
fn underbar_4_3_3_2() {
    let expr = "<math><munder><mi>z</mi><mo>_</mo></munder></math>";
    test_braille("CMU", expr, "⠠⠤⠵");
}

#[test]
fn underbar_4_3_3_3() {
    let expr = "<math><menclose notation='bottom'><menclose notation='bottom'><mi>z</mi></menclose></menclose></math>";
    test_braille("CMU", expr, "⠠⠤⠠⠤⠵");
}


#[test]
fn bar_menclose_4_3_3_4() {
    let expr = "<math><menclose notation=\"bottom\"><mi>a</mi><mi>b</mi></menclose></math>";
    test_braille("CMU", expr, "⠠⠤⠢⠁⠃⠔");
}

#[test]
fn bar_menclose_4_3_3_5() {
    let expr = "<math><menclose notation=\"bottom\"><mi>z</mi><mo>''</mo></menclose></math>";
    test_braille("CMU", expr, "⠠⠤⠢⠵⠳⠳⠔");
}

#[test]
fn other_4_3_3_6() {
    let expr = "<math><munder><mi>z</mi><mo>&lt;</mo></munder></math>";
    test_braille("CMU", expr, "⠵⠌⠌⠪⠄");
}

#[test]
fn scripts_4_3_4_1() {
    let expr = "<math><msub><mi>z</mi><mo>+</mo></msub></math>";
    test_braille("CMU", expr, "⠵⠌⠖⠄");
}

#[test]
fn scripts_4_3_4_2() {
    // needs to convert the "----" to "4-"
    let expr = "<math><mmultiscripts><mi>z</mi><mprescripts/>
                        <none/><mrow><mo>-</mo><mo>-</mo><mo>-</mo><mo>-</mo></mrow></mmultiscripts></math>";
    test_braille("CMU", expr, "⠵⠈⠡⠼⠙⠤⠄");
}

#[test]
fn scripts_4_3_4_3() {
    let expr = "<math>
        <mmultiscripts>
            <mi>z</mi>
            <none/>
            <mo>″</mo>
            <mprescripts/>
            <mrow><mo>+</mo><mo>+</mo></mrow>
            <none/>
        </mmultiscripts>
    </math>";
    test_braille("CMU", expr, "⠵⠳⠳⠠⠌⠖⠖⠄");
}

#[test]
fn both_scripts_4_4_1_1() {
    let expr = "<math><msubsup><mi>z</mi><mn>4</mn><mn>3</mn></msubsup></math>";
    test_braille("CMU", expr, "⠵⠌⠼⠙⠡⠼⠉");
}

#[test]
fn both_multiscripts_4_4_1_1() {
    let expr = "<math><mmultiscripts><mi>z</mi><mi>4</mi><mi>3</mi></mmultiscripts></math>";
    test_braille("CMU", expr, "⠵⠌⠼⠙⠡⠼⠉");
}

#[test]
fn both_scripts_4_4_1_2() {
    let expr = "<math><msubsup><mi>z</mi><mrow><mi>i</mi><mo>,</mo><mi>j</mi></mrow><mn>2</mn></msubsup></math>";
    test_braille("CMU", expr, "⠵⠌⠢⠊⠀⠂⠚⠔⠡⠼⠃");
}

#[test]
fn both_multiscripts_4_4_1_2() {
    let expr = "<math><mmultiscripts><mi>z</mi><mrow><mi>i</mi><mo>,</mo><mi>j</mi></mrow><mi>2</mi></mmultiscripts></math>";
    test_braille("CMU", expr, "⠵⠌⠢⠊⠀⠂⠚⠔⠡⠼⠃");
}

#[test]
fn both_scripts_4_4_1_3() {
    let expr = "<math><msubsup><mi>z</mi><mn>0</mn><mo>'</mo></msubsup></math>";
    test_braille("CMU", expr, "⠵⠳⠌⠼⠚");
}

#[test]
fn both_scripts_4_4_1_4() {
    let expr = "<math><msup><mrow><mi>z</mi><mo>'</mo></mrow><mn>3</mn></msup></math>";
    test_braille("CMU", expr, "⠵⠳⠡⠼⠉");
}

#[test]
fn both_scripts_4_4_1_5() {
    let expr = "<math><mover><msub><mi>z</mi><mn>0</mn></msub><mo>&#x2015;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠉⠢⠵⠌⠼⠚⠔");
}

#[test]
fn both_scripts_4_4_1_6() {
    let expr = "<math>
        <msup>
        <mover>
            <mrow>
            <mo>(</mo>
            <msubsup><mi>z</mi><mn>0</mn><mi>&#x2032;</mi></msubsup>
            <mo>)</mo>
            </mrow>
            <mo>&#x2015;</mo>
        </mover>
        <mn>2</mn>
        </msup>
    </math>";
    test_braille("CMU", expr, "⠈⠉⠣⠵⠳⠌⠼⠚⠜⠡⠼⠃");
}

#[test]
fn both_scripts_4_4_1_7() {
    let expr = "<math><msup>
            <mover><msubsup><mi>z</mi><mn>0</mn><mi>&#x2032;</mi></msubsup><mo>&#x2015;</mo></mover>
            <mn>2</mn>
        </msup></math>";
    test_braille("CMU", expr, "⠈⠉⠢⠵⠳⠌⠼⠚⠔⠡⠼⠃");
}

#[test]
fn scripts_4_4_2_1() {
    let expr = "<math><msup><msub><mi>T</mi><mi>r</mi></msub><mi>s</mi></msup></math>";
    test_braille("CMU", expr, "⠨⠞⠌⠗⠘⠡⠎");
}

#[test]
fn mmultiscripts_4_4_2_1() {
    let expr = "<math><mmultiscripts><mi>T</mi><mi>r</mi><none/><none/><mi>s</mi></mmultiscripts></math>";
    test_braille("CMU", expr, "⠨⠞⠌⠗⠘⠡⠎");
}

#[test]
fn scripts_4_4_2_2() {
    let expr = "<math><msub><msup><mi>T</mi><mi>r</mi></msup><mi>s</mi></msub></math>";
    test_braille("CMU", expr, "⠨⠞⠡⠗⠰⠌⠎");
}

#[test]
fn mmultiscripts_4_4_2_2() {
    let expr = "<math><mmultiscripts><mi>T</mi><none/><mi>r</mi><mi>s</mi><none/></mmultiscripts></math>";
    test_braille("CMU", expr, "⠨⠞⠡⠗⠰⠌⠎");
}


#[test]
fn scripts_4_4_2_3() {
    let expr = "<math><msubsup><mi>T</mi><mi>s</mi><mi>r</mi></msubsup></math>";
    test_braille("CMU", expr, "⠨⠞⠌⠎⠡⠗");
}

#[test]
fn multi_scripts_4_4_2_3() {
    let expr = "<math><mmultiscripts><mi>T</mi><mi>s</mi><mi>r</mi></mmultiscripts></math>";
    test_braille("CMU", expr, "⠨⠞⠌⠎⠡⠗");
}


#[test]
fn arith_5_1_1() {
    let expr = "<math><mn>6</mn><mo>+</mo><mn>2</mn></math>";
    test_braille("CMU", expr, "⠼⠋⠖⠼⠃");
}

#[test]
fn arith_5_1_2() {
    let expr = "<math><mn>6</mn><mo>-</mo><mn>2</mn></math>";
    test_braille("CMU", expr, "⠼⠋⠤⠼⠃");
}

#[test]
fn arith_5_1_3() {
    let expr = "<math><mn>6</mn><mo>×</mo><mn>2</mn></math>";
    test_braille("CMU", expr, "⠼⠋⠦⠼⠃");
}

#[test]
fn arith_5_1_4() {
    let expr = "<math><mn>6</mn><mo>·</mo><mn>2</mn></math>";
    test_braille("CMU", expr, "⠼⠋⠠⠀⠼⠃");
}

#[test]
fn arith_5_1_5() {
    let expr = "<math><mn>6</mn><mo>⊚</mo><mn>2</mn></math>";
    test_braille("CMU", expr, "⠼⠋⠰⠄⠼⠃");
}

#[test]
fn arith_5_1_6() {
    let expr = "<math><mn>3</mn><mi>b</mi></math>";
    test_braille("CMU", expr, "⠼⠉⠐⠃");
}

#[test]
fn arith_5_1_7() {
    let expr = "<math><mn>3</mn><mi>a</mi><mo>+</mo><mn>5</mn><mi>x</mi></math>";
    test_braille("CMU", expr, "⠼⠉⠐⠁⠖⠼⠑⠭");
}

#[test]
fn arith_5_1_9() {
    let expr = "<math><mi>x</mi><mo>&#xB7;</mo><mi>y</mi></math>";
    test_braille("CMU", expr, "⠭⠠⠀⠽");
}

#[test]
fn arith_5_1_11() {
    let expr = "<math><mn>3</mn><mo>:</mo><mn>4</mn></math>";
    test_braille("CMU", expr, "⠼⠉⠐⠂⠼⠙");
}

#[test]
fn arith_5_1_14() {
    let expr = "<math><mfrac><mi>a</mi><mrow><mi>c</mi><mo>·</mo><mi>x</mi></mrow></mfrac></math>";
    test_braille("CMU", expr, "⠁⠲⠢⠉⠠⠀⠭⠔");
}

#[test]
fn arith_5_1_15() {
    let expr = "<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mi>c</mi></mfrac></math>";
    test_braille("CMU", expr, "⠢⠁⠖⠃⠔⠲⠉");
}

#[test]
fn arith_5_1_16() {
    let expr = "<math><mfrac>
        <mrow><mi>a</mi><mo>+</mo><mfrac><mi>b</mi><mi>c</mi></mfrac></mrow>
        <mrow><mi>d</mi><mo>+</mo><mi>e</mi></mrow>
    </mfrac></math>";
    test_braille("CMU", expr, "⠢⠁⠖⠃⠲⠉⠔⠲⠢⠙⠖⠑⠔");
}

#[test]
fn arith_5_1_17() {
    let expr = "<math><mi>a</mi><mo>+</mo><mi>b</mi><mo>/</mo><mi>c</mi></math>";
    test_braille("CMU", expr, "⠁⠖⠃⠲⠉");
}

#[test]
fn arith_5_1_18() {
    let expr = "<math><mfrac>
        <mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow></mfrac>
        <mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow>
    </mfrac></math>";
    test_braille("CMU", expr, "⠢⠢⠁⠖⠃⠔⠲⠢⠉⠖⠙⠔⠔⠲⠢⠭⠖⠽⠔");
}

#[test]
fn arith_5_1_19() {
    let expr = "<math>
        <mfrac><mn>3</mn><mn>5</mn></mfrac><mo>&#xB7;</mo>
        <mfrac><mn>2</mn><mn>7</mn></mfrac><mo>=</mo>
        <mfrac><mn>6</mn><mn>35</mn></mfrac></math>";
    // corrected: output corresponds to using 'x', not '·' as shown in example
    test_braille("CMU", expr, "⠼⠉⠢⠀⠠⠀⠼⠃⠶⠀⠶⠼⠋⠒⠢⠀");
}

#[test]
fn power_5_2_3() {
    let expr = "<math><msup><mi>x</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup></math>";
    test_braille("CMU", expr, "⠭⠡⠤⠼⠁");
}

#[test]
fn power_5_2_4() {
    let expr = "<math><msup><mi>x</mi><mfrac><mn>1</mn><mn>2</mn></mfrac></msup></math>";
    test_braille("CMU", expr, "⠭⠡⠼⠁⠆⠀");
}

#[test]
fn power_5_2_5() {
    let expr = "<math><msup><mi>x</mi><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow></msup></math>";
    test_braille("CMU", expr, "⠭⠡⠢⠁⠖⠃⠔");
}

#[test]
fn power_5_2_6() {
    let expr = "<math><msup><mi>x</mi><mrow><mo>-</mo><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo></mrow></msup></math>";
    test_braille("CMU", expr, "⠭⠡⠤⠣⠁⠖⠃⠜");
}

#[test]
fn power_5_2_7() {
    let expr = "<math><msup><mi>x</mi><msup><mi>n</mi><mn>2</mn></msup></msup></math>";
    test_braille("CMU", expr, "⠭⠡⠝⠡⠼⠃");
}

#[test]
fn power_5_2_8() {
    let expr = "<math><mn>7</mn><msup><mi>x</mi><mn>3</mn></msup><mo>-</mo>
                <mn>2</mn><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mi>x</mi><mo>+</mo><mn>1</mn></math>";
    test_braille("CMU", expr, "⠼⠛⠭⠡⠼⠉⠤⠼⠃⠭⠡⠼⠃⠖⠭⠖⠼⠁");
}

#[test]
fn power_5_2_9() {
    let expr = "<math><mn>3</mn><mi>a</mi><mo>+</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><msup><mi>x</mi><mn>2</mn></msup><msup><mi>y</mi><mn>3</mn></msup></math>";
    test_braille("CMU", expr, "⠼⠉⠐⠁⠖⠼⠁⠆⠀⠭⠡⠼⠃⠽⠡⠼⠉");
}

#[test]
fn power_5_2_11() {
    // 4/3a^2
    let expr = "<math><mfrac><mn>4</mn><mrow><mn>3</mn><msup><mi>a</mi><mn>2</mn></msup></mrow></mfrac></math>";
    test_braille("CMU", expr, "⠼⠙⠲⠢⠼⠉⠐⠁⠡⠼⠃⠔");
}

#[test]
fn power_5_2_12() {
    // x^a+b
    let expr = "<math><msup><mi>x</mi><mi>a</mi></msup><mo>+</mo><mi>b</mi></math>";
    test_braille("CMU", expr, "⠭⠡⠁⠖⠃");
}

#[test]
fn root_5_2_1_1() {
    let expr = "<math><mroot><mn>8</mn><mn>3</mn></mroot></math>";
    test_braille("CMU", expr, "⠫⠼⠉⠱⠼⠓");
}

#[test]
fn root_5_2_1_2() {
    let expr = "<math><msqrt><mn>8</mn></msqrt></math>";
    test_braille("CMU", expr, "⠫⠱⠼⠓");
}

#[test]
fn sqrt_5_3_3() {
    let expr = "<math><msqrt><mi>x</mi></msqrt></math>";
    test_braille("CMU", expr, "⠫⠱⠭");
}

#[test]
fn root_5_2_1_4() {
    let expr = "<math><mroot><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mi>n</mi></mroot></math>";
    test_braille("CMU", expr, "⠫⠝⠱⠢⠁⠖⠃⠔");
}

#[test]
fn root_5_2_1_5() {
    let expr = "<math><mroot><mi>a</mi><mi>n</mi></mroot><mo>+</mo><mi>b</mi></math>";
    test_braille("CMU", expr, "⠫⠝⠱⠁⠖⠃");
}

#[test]
fn root_5_2_1_6() {
    let expr = "<math><mroot><mi>a</mi><mi>n</mi></mroot><mo>+</mo><mi>b</mi></math>";
    test_braille("CMU", expr, "⠫⠝⠱⠁⠖⠃");
}

#[test]
fn root_5_2_1_7() {
    let expr = "<math><mroot><mfrac><mi>a</mi><mi>b</mi></mfrac><mrow><mi>n</mi><mo>-</mo><mn>1</mn></mrow></mroot></math>";
    test_braille("CMU", expr, "⠫⠝⠤⠼⠁⠱⠢⠁⠲⠃⠔");
}

#[test]
fn root_5_2_1_8() {
    // \sqrt{ \sqrt{8} }
    let expr = "<math><msqrt><msqrt><mn>16</mn></msqrt></msqrt></math>";
    test_braille("CMU", expr, "⠫⠱⠫⠱⠼⠁⠋");
}

#[test]
fn root_5_2_1_9() {
    // \sqrt{ x^2+y^2 }
    let expr = "<math><msqrt><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></msqrt></math>";
    test_braille("CMU", expr, "⠫⠱⠢⠭⠡⠼⠃⠖⠽⠡⠼⠃⠔");
}

#[test]
fn root_5_2_1_10() {
    let expr = "<math><mroot><mrow><mn>3</mn><msup><mi>a</mi><mn>2</mn></msup><mo>-</mo><mi>a</mi></mrow><mn>3</mn></mroot>
                        <mo>+</mo><mn>9</mn></math>";
    test_braille("CMU", expr, "⠫⠼⠉⠱⠢⠼⠉⠐⠁⠡⠼⠃⠤⠁⠔⠖⠼⠊");
}


#[test]
fn factorial_5_3_1() {
    let expr = "<math><mi>n</mi><mo>!</mo></math>";
    test_braille("CMU", expr, "⠝⠘⠄");
}

#[test]
fn binomial_5_3_3() {
    let expr = "<math><mrow>
        <mo>(</mo>
        <mfrac linethickness='0'><mi>n</mi><mi>r</mi></mfrac>
        <mo>)</mo>
    </mrow></math>";
    test_braille("CMU", expr, "⠨⠣⠝⠒⠗⠜");
}

#[test]
fn binomial_5_3_4() {
    let expr = "<math>
        <msubsup><mi>C</mi><mi>n</mi><mi>k</mi></msubsup>
        <mo>=</mo>
        <mrow><mo>(</mo><mfrac linethickness='0'><mi>n</mi><mi>k</mi></mfrac><mo>)</mo></mrow>
        <mo>=</mo>
        <mfrac>
        <mrow><mi>n</mi><mo>!</mo></mrow>
        <mrow><mi>k</mi><mo>!</mo><mo>(</mo><mi>n</mi><mo>&#x2212;</mo><mi>k</mi><mo>)</mo><mo>!</mo></mrow>
        </mfrac>
    </math>";
    test_braille("CMU", expr, "⠨⠉⠌⠝⠡⠅⠶⠨⠣⠝⠒⠅⠜⠶⠝⠘⠄⠲⠢⠅⠘⠄⠣⠝⠤⠅⠜⠘⠄⠔");
}

#[test]
fn binomial_5_3_5() {
    let expr = "<math>
        <mi>C</mi>
        <msubsup><mi>R</mi><mi>n</mi><mi>k</mi></msubsup>
        <mo>=</mo>
        <mrow>
        <mo>(</mo>
        <mfrac linethickness='0'><mrow><mi>n</mi><mo>+</mo><mi>k</mi><mo>&#x2212;</mo><mn>1</mn></mrow><mi>k</mi></mfrac>
        <mo>)</mo>
        </mrow>
    </math>";
    test_braille("CMU", expr, "⠨⠉⠨⠗⠌⠝⠡⠅⠶⠨⠣⠝⠖⠅⠤⠼⠁⠒⠅⠜");
}

#[test]
fn variation_5_3_6() {
    let expr = "<math><msubsup><mi>V</mi><mi>n</mi><mi>k</mi></msubsup><mo>=</mo>
        <msup><mi>n</mi><menclose notation='bottom'><mi>k</mi></menclose></msup><mo>=</mo>
        <mfrac><mrow><mi>n</mi><mo>!</mo></mrow><mrow><mo>(</mo><mi>n</mi><mo>-</mo><mi>k</mi><mo>)</mo><mo>!</mo></mrow></mfrac>
    </math>";
    test_braille("CMU", expr, "⠨⠧⠌⠝⠡⠅⠶⠝⠡⠠⠤⠅⠶⠝⠘⠄⠲⠣⠝⠤⠅⠜⠘⠄");
}

#[test]
fn variation_5_3_8() {
    let expr = "<math>
        <msub><mi>V</mi><mrow><mi>n</mi><mo>,</mo><mi>k</mi></mrow></msub><mo>=</mo>
        <mi>n</mi><mo>(</mo><mi>n</mi><mo>-</mo><mn>1</mn><mo>)</mo>
        <mo>(</mo><mi>n</mi><mo>-</mo><mn>2</mn><mo>)</mo><mo>&#x2026;</mo>
        <mo>(</mo><mi>n</mi><mo>-</mo><mi>k</mi><mo>+</mo><mn>1</mn><mo>)</mo></math>";
    test_braille("CMU", expr, "⠨⠧⠌⠢⠝⠀⠂⠅⠔⠶⠝⠣⠝⠤⠼⠁⠜⠣⠝⠤⠼⠃⠜⠄⠄⠄⠣⠝⠤⠅⠖⠼⠁⠜");
}

#[test]
fn operators_5_4_1() {
    let expr = "<math><mn>6</mn><mo>&#xB1;</mo><mn>2</mn></math>";
    test_braille("CMU", expr, "⠼⠋⠖⠒⠤⠼⠃");
}

#[test]
fn operators_5_4_2() {
    let expr = "<math><mn>21</mn><mo>%</mo></math>";
    test_braille("CMU", expr, "⠼⠃⠁⠸⠴");
}

#[test]
fn operators_5_4_3() {
    let expr = "<math><mo>|</mo><mo>α</mo><mo>|</mo><mo>=</mo><mn>1</mn></math>";
    test_braille("CMU", expr, "⠸⠈⠁⠸⠀⠶⠼⠁");
}


#[test]
fn sum_5_5_1_1() {
    let expr = "<math><munderover><mo>&#x2211;</mo><mrow><mi>j</mi><mo>=</mo><mn>1</mn></mrow><mn>4</mn></munderover><msup><mi>j</mi><mn>2</mn></msup></math>";
    test_braille("CMU", expr, "⠘⠎⠚⠶⠼⠁⠒⠼⠙⠱⠚⠡⠼⠃");
}

#[test]
fn sum_5_5_1_2() {
    let expr = "<math><msub><mo>&#x2211;</mo><mrow><mo>(</mo><mn>1</mn><mo>&#x2264;</mo><mi>j</mi><mo>&#x2264;</mo><mn>4</mn><mo>)</mo></mrow></msub>
                <msup><mi>j</mi><mn>2</mn></msup></math>";
    test_braille("CMU", expr, "⠘⠎⠼⠁⠪⠶⠚⠪⠶⠼⠙⠱⠚⠡⠼⠃");
}

#[test]
fn product_5_5_2_1() {
    let expr = "<math><munderover><mo>&#x220F;</mo><mrow><mi>j</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover>
            <msub><mi>S</mi><mi>j</mi></msub></math>";
    test_braille("CMU", expr, "⠘⠏⠚⠶⠼⠁⠒⠝⠱⠨⠎⠌⠚");
}

#[test]
fn product_5_5_2_2() {
    let expr = "<math><msub><mo>&#x220F;</mo><mrow><mn>1</mn><mo>&#x2264;</mo><mi>j</mi><mo>&#x2264;</mo><mi>n</mi></mrow></msub>
            <msub><mi>S</mi><mi>j</mi></msub></math>";
    test_braille("CMU", expr, "⠘⠏⠼⠁⠪⠶⠚⠪⠶⠝⠱⠨⠎⠌⠚");
}


#[test]
fn coproduct_5_5_3_1() {
    let expr = "<math><mi>X</mi><mo>=</mo><munder><mo>&#x2210;</mo><mrow><mi>j</mi><mo>&#x2208;</mo><mi>J</mi></mrow></munder>
                        <msub><mi>X</mi><mi>j</mi></msub></math>";
    test_braille("CMU", expr, "⠨⠭⠶⠘⠻⠚⠣⠂⠨⠚⠱⠨⠭⠌⠚");
}

#[test]
fn relations_6_1_1() {
    let expr = "<math><mi>p</mi><mo>≔</mo><mi>m</mi><mo>&#xB7;</mo><mi>v</mi></math>";
    test_braille("CMU", expr, "⠏⠰⠶⠍⠠⠀⠧");
}


#[test]
fn relations_6_1_2() {
    let expr = "<math><mi>A</mi><mo>&#x2248;</mo><mi>B</mi></math>";
    test_braille("CMU", expr, "⠨⠁⠐⠶⠄⠨⠃");
}

#[test]
fn relations_6_1_3() {
    let expr = "<math><mn>4</mn><mo>&#x2236;</mo><mn>3</mn><mo>&#x2237;</mo><mn>8</mn><mo>&#x2236;</mo><mn>6</mn></math>";
    test_braille("CMU", expr, "⠼⠙⠐⠂⠼⠉⠰⠆⠼⠓⠐⠂⠼⠋");
}

#[test]
fn set_7_2_1() {
    let expr = "<math><mi>B</mi><mo>=</mo><mo>{</mo><mi>x</mi><mo>:</mo><mo>&#xA0;</mo><mi>x</mi><mo>∈</mo><mi>ℕ</mi><mo>,</mo><mi>x</mi><mo>&lt;</mo><mn>7</mn><mo>}</mo></math>";
    test_braille("CMU", expr, "⠨⠃⠶⠐⠇⠭⠐⠂⠭⠣⠂⠸⠝⠀⠂⠭⠪⠼⠛⠸⠂");
}

#[test]
fn set_7_2_2() {
    let expr = "<math><mi>A</mi><mo>&#x2229;</mo><mi>B</mi><mo>=</mo><mo>{</mo><mn>2</mn><mo>,</mo><mn>4</mn><mo>,</mo><mn>6</mn><mo>}</mo></math>";
    test_braille("CMU", expr, "⠨⠁⠸⠱⠨⠃⠶⠐⠇⠼⠃⠀⠂⠼⠙⠀⠂⠼⠋⠸⠂");
}

#[test]
fn set_7_2_3() {
    let expr = "<math><msub><mi>⋃</mi><mrow><mi>j</mi><mo>∈</mo><mi>I</mi></mrow></msub><msub><mi>A</mi><mi>j</mi></msub></math>";
    test_braille("CMU", expr, "⠿⠜⠚⠣⠂⠨⠊⠱⠨⠁⠌⠚");
}

#[test]
fn logic_8_2_1() {
    let expr = "<math><mo>&#x2200;</mo><mi>p</mi><mo>,</mo><mi>q</mi><mo>&#xAC;</mo><mo>(</mo><mi>p</mi><mo>&#x2227;</mo><mi>q</mi><mo>)</mo><mo>⟺</mo><mo>&#xAC;</mo><mi>p</mi><mo>&#x2228;</mo><mo>&#xAC;</mo><mi>q</mi></math>";
    test_braille("CMU", expr, "⠨⠄⠏⠀⠂⠟⠠⠄⠣⠏⠸⠢⠟⠜⠪⠒⠕⠠⠄⠏⠸⠊⠠⠄⠟");
}

#[test]
fn logic_8_2_2() {
    let expr = "<math><mo>&#x2204;</mo><mi>p</mi><mo>:</mo><mi>q</mi><mo>&#x2227;</mo><mo>~</mo><mi>p</mi><mo>=</mo><mo>&#x22A9;</mo></math>";
    test_braille("CMU", expr, "⠘⠨⠢⠏⠐⠂⠟⠸⠢⠠⠄⠏⠶⠸⠶");
}

#[test]
fn inverse_9_1_1() {
    let expr = "<math><mover><mo>⟶</mo><mi>f</mi></mover></math>";
    test_braille("CMU", expr, "⠒⠋⠒⠂");
}

#[test]
fn list_9_1_2() {
    let expr = "<math><mo>(</mo><msub><mi>x</mi><mn>1</mn></msub><mo>,</mo><msub><mi>x</mi><mn>2</mn></msub><mo>)</mo></math>";
    test_braille("CMU", expr, "⠣⠭⠌⠼⠁⠀⠂⠭⠌⠼⠃⠜");
}

#[test]
fn list_9_1_3() {
    let expr = "<math><mfenced><mrow><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>,</mo><mfrac><mn>3</mn><mn>2</mn></mfrac></mrow></mfenced></math>";
    test_braille("CMU", expr, "⠣⠼⠁⠆⠀⠂⠼⠉⠆⠀⠜");
}

#[test]
fn function_map_9_1_4() {
    let expr = "<math><mi>f</mi><mo>:</mo><mi>A</mi><mo>⟶</mo><mi>B</mi></math>";
    test_braille("CMU", expr, "⠋⠐⠂⠨⠁⠒⠒⠂⠨⠃");
}

#[test]
fn arrow_inverse_9_1_6() {
    // note: there appears to be an abbreviated form for x^{-1}, maybe for 'x' being a single letter? Not explained in the spec.
    let expr = "<math><mi>B</mi><mover><mo>⟶</mo><msup><mi>f</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup></mover><mi>A</mi></math>";
    test_braille("CMU", expr, "⠨⠃⠒⠋⠡⠤⠼⠁⠒⠂⠨⠁");
}

#[test]
fn arrow_9_1_8() {
    let expr = "<math><mi>A</mi><mo>⟷</mo><mi>B</mi></math>";
    test_braille("CMU", expr, "⠨⠁⠐⠒⠒⠂⠨⠃");
}

#[test]
fn congruence_9_1_10() {
    let expr = "<math><mi>f</mi><mo>&#x2261;</mo><mn>0</mn></math>";
    test_braille("CMU", expr, "⠋⠶⠶⠼⠚");
}

#[test]
fn composition_9_1_11() {
    let expr = "<math><mi>f</mi><mo>&#x2218;</mo><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo>
                        <mi>f</mi><mo>(</mo><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>)</mo></math>";
    test_braille("CMU", expr, "⠋⠠⠆⠛⠣⠭⠜⠶⠋⠣⠛⠣⠭⠜⠜");
}

#[test]
fn interval_9_1_12() {
    // note: there appears to be an abbreviated form for x^{-1}, maybe for 'x' being a single letter? Not explained in the spec.
    let expr = "<math><mo>]</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo>[</mo></math>";
    test_braille("CMU", expr, "⠾⠁⠀⠂⠃⠷");
}


#[test]
fn limit_line_over_9_2_0() {
    let expr = "<math><mrow><mover accent='true'><mrow><mi>lim</mi></mrow><mo>&#x00AF;</mo></mover></mrow></math>";
    test_braille("CMU", expr, "⠈⠉⠇⠊⠍⠄");
}

#[test]
fn limit_line_under_9_2_0() {
    let expr = "<math><mrow><munder accent='true'><mrow><mi>lim</mi></mrow><mo>&#x00AF;</mo></munder></mrow></math>";
    test_braille("CMU", expr, "⠠⠤⠇⠊⠍⠄");
}

#[test]
fn limit_9_2_3() {
    let expr = "<math><mrow><munder><mrow><mi>lim</mi></mrow><mrow><mi>x</mi><mo>→</mo><mi>c</mi></mrow></munder></mrow></math>";
    test_braille("CMU", expr, "⠇⠊⠍⠄⠭⠒⠂⠉⠱");
}

#[test]
fn limit_9_2_4() {
    let expr = "<math><mrow><munder><mrow><mi>lim</mi></mrow><mrow><mi>x</mi><mo>↑</mo><mi>c</mi></mrow></munder></mrow></math>";
    test_braille("CMU", expr, "⠇⠊⠍⠄⠭⠸⠁⠉⠱");
}

#[test]
fn limit_9_2_5() {
    let expr = "<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>⟶</mo><msup><mn>0</mn><mo>-</mo></msup></mrow></munder></math>";
    test_braille("CMU", expr, "⠇⠊⠍⠄⠭⠒⠂⠼⠚⠤⠄⠱");
}

#[test]
fn limit_9_2_6() {
    let expr = "<math><mrow><munder><mrow><mi>lim</mi></mrow><mrow><mi>x</mi><mo>↓</mo><mi>c</mi></mrow></munder></mrow></math>";
    test_braille("CMU", expr, "⠇⠊⠍⠄⠭⠸⠄⠉⠱");
}

#[test]
fn limit_9_2_8() {
    let expr = "<math>
    <munder><mo>lim</mo><mrow><mi>x</mi><mo>⟶</mo><mi>c</mi></mrow></munder>
    <mo>(</mo><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>+</mo><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>)</mo>
    <mo>=</mo>
    <munder><mo>lim</mo><mrow><mi>x</mi><mo>⟶</mo><mi>c</mi></mrow></munder>
    <mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>+</mo>
    <munder><mo>lim</mo><mrow><mi>x</mi><mo>⟶</mo><mi>c</mi></mrow></munder>
    <mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo>
  </math>";
    test_braille("CMU", expr, "⠇⠊⠍⠄⠭⠒⠂⠉⠱⠣⠋⠣⠭⠜⠖⠛⠣⠭⠜⠜⠶⠇⠊⠍⠄⠭⠒⠂⠉⠱⠋⠣⠭⠜⠖⠇⠊⠍⠄⠭⠒⠂⠉⠱⠛⠣⠭⠜");
}

#[test]
fn deriv_9_3_1() {
    let expr = "<math><mfrac><mrow><mi>d</mi><mi>f</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>";
    test_braille("CMU", expr, "⠢⠙⠋⠔⠲⠢⠙⠭⠔");
}


#[test]
fn deriv_9_3_2() {
    let expr = "<math><mfrac><mi>d</mi><mrow><mi>d</mi><mi>x</mi></mrow></mfrac><mi>f</mi></math>";
    test_braille("CMU", expr, "⠙⠲⠢⠙⠭⠔⠋");
}

#[test]
fn deriv_9_3_5() {
    let expr = "<math><mfrac>
            <mrow><msup><mi>d</mi><mi>n</mi></msup><mi>f</mi></mrow>
            <mrow><mi>d</mi><msup><mi>x</mi><mi>n</mi></msup></mrow>
        </mfrac></math>";
    test_braille("CMU", expr, "⠢⠙⠡⠝⠋⠔⠲⠢⠙⠭⠡⠝⠔");
}

#[test]
fn deriv_9_3_6() {
    let expr = "<math><mfrac>
        <msup><mi>d</mi><mi>n</mi></msup>
        <mrow><mi>d</mi><msup><mi>x</mi><mi>n</mi></msup></mrow>
    </mfrac><mi>f</mi></math>";
    test_braille("CMU", expr, "⠙⠡⠝⠲⠢⠙⠭⠡⠝⠔⠋");
}


#[test]
fn partial_9_3_15() {
    let expr = "<math><mfrac>
            <msup><mo>∂</mo><mn>2</mn></msup>
            <mrow><mo>∂</mo><mi>x</mi><mo>∂</mo><mi>y</mi></mrow>
        </mfrac></math>";
    test_braille("CMU", expr, "⠸⠙⠡⠼⠃⠲⠢⠸⠙⠭⠸⠙⠽⠔");
}

#[test]
fn partial_9_3_16() {
    let expr = "<math><mfrac>
            <mrow><msup><mo>∂</mo><mn>2</mn></msup><mi>f</mi></mrow>
            <mrow><mo>∂</mo><mi>x</mi><mo>∂</mo><mi>y</mi></mrow>
        </mfrac></math>";
    test_braille("CMU", expr, "⠢⠸⠙⠡⠼⠃⠐⠋⠔⠲⠢⠸⠙⠭⠸⠙⠽⠔");
}

#[test]
fn partial_9_3_17() {
    let expr = "<math><mfrac>
            <msup><mo>∂</mo><mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow></msup>
            <mrow><mo>∂</mo><msup><mi>x</mi><mi>m</mi></msup><mo>∂</mo><msup><mi>y</mi><mi>n</mi></msup></mrow>
        </mfrac></math>";
    test_braille("CMU", expr, "⠸⠙⠡⠢⠍⠖⠝⠔⠲⠢⠸⠙⠭⠡⠍⠸⠙⠽⠡⠝⠔");
}

#[test]
fn partial_9_3_18() {
    let expr = "<math><mfrac>
            <mrow><msup><mo>∂</mo><mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow></msup><mi>f</mi></mrow>
            <mrow><mo>∂</mo><msup><mi>x</mi><mi>m</mi></msup><mo>∂</mo><msup><mi>y</mi><mi>n</mi></msup></mrow>
        </mfrac></math>";
    test_braille("CMU", expr, "⠢⠸⠙⠡⠢⠍⠖⠝⠔⠋⠔⠲⠢⠸⠙⠭⠡⠍⠸⠙⠽⠡⠝⠔");
}

#[test]
fn integral_9_4_0() {
    // countour integral over C
    let expr = "<math><msub><mo>&#x222E;</mo><mi>C</mi></msub></math>";
    test_braille("CMU", expr, "⠯⠴⠨⠉⠱");
}

#[test]
fn integral_9_4_1() {
    let expr = "<math><mo>∫</mo><msup><mi>x</mi><mn>2</mn></msup><mi>d</mi><mi>x</mi></math>";
    test_braille("CMU", expr, "⠯⠱⠭⠡⠼⠃⠐⠙⠭");
}

#[test]
fn integral_9_4_2() {
    let expr = "<math>
        <msubsup><mo>&#x222B;</mo><mn>1</mn><mn>4</mn></msubsup>
        <msup><mi>x</mi><mn>2</mn></msup>
        <mi>d</mi>
        <mi>x</mi>
        <mo>=</mo>
        <msubsup>
            <mfenced open='[' close=']'>
                <mrow><mfrac><mn>1</mn><mn>3</mn></mfrac><msup><mi>x</mi><mn>3</mn></msup></mrow>
            </mfenced>
            <mn>1</mn>
            <mn>4</mn>
        </msubsup>
        <mo>=</mo>
        <mn>21</mn>
    </math>";
    test_braille("CMU", expr, "⠯⠼⠁⠒⠼⠙⠱⠭⠡⠼⠃⠐⠙⠭⠶⠷⠼⠁⠒⠀⠭⠡⠼⠉⠾⠌⠼⠁⠡⠼⠙⠶⠼⠃⠁");
}

#[test]
fn succession_10_1_1_1() {
    let expr = "<math><mo>(</mo><msub><mi>s</mi><mi>n</mi></msub><mo>)</mo></math>";
    test_braille("CMU", expr, "⠣⠎⠌⠝⠜");
}

#[test]
fn lim_10_1_1_6() {
    let expr = "<math><munder><mi>lim</mi><mrow><mi>n</mi><mo>⟶</mo><mo>&#x221E;</mo></mrow></munder>
                        <msub><mi>S</mi><mi>n</mi></msub></math>";
    test_braille("CMU", expr, "⠇⠊⠍⠄⠝⠒⠂⠼⠳⠱⠨⠎⠌⠝");
}

#[test]
fn series_10_1_3_1() {
    let expr = "<math>
            <munderover><mo>&#x2211;</mo><mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow><mo>&#x221E;</mo></munderover>
            <mfrac><mn>1</mn><msup><mi>n</mi><mn>2</mn></msup></mfrac><mo>=</mo>
            <munder><mi>lim</mi><mrow><mi>k</mi><mo>⟶</mo><mo>&#x221E;</mo></mrow></munder>
            <munderover><mo>&#x2211;</mo><mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow><mi>k</mi></munderover>
            <mfrac><mn>1</mn><msup><mi>n</mi><mn>2</mn></msup></mfrac><mo>=</mo>
            <mn>1</mn><mo>+</mo>
            <mfrac><mn>1</mn><mn>4</mn></mfrac><mo>+</mo>
            <mfrac><mn>1</mn><mn>9</mn></mfrac><mo>+</mo>
            <mfrac><mn>1</mn><mn>16</mn></mfrac><mo>+</mo>
        <mo>&#x22EF;</mo></math>";
    // it looks like the spec is wrong -- correct to add "⠀⠖" near the end
    test_braille("CMU", expr, "⠘⠎⠝⠶⠼⠁⠒⠼⠳⠱⠼⠁⠲⠝⠡⠼⠃⠶⠇⠊⠍⠄⠅⠒⠂⠼⠳⠱⠘⠎⠝⠶⠼⠁⠒⠅⠱⠼⠁⠲⠝⠡⠼⠃⠶⠼⠁⠖⠼⠁⠲⠀⠖⠼⠁⠔⠀⠖⠼⠁⠂⠖⠀⠖⠄⠄⠄");
}


#[test]
fn log_10_2_2() {
    let expr = "<math><msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></math>";
    test_braille("CMU", expr, "⠇⠕⠛⠄⠃⠱⠭");
}

#[test]
fn log_10_2_4() {
    let expr = "<math><mi>Ln</mi><mi>x</mi></math>";
    test_braille("CMU", expr, "⠇⠝⠄⠭");
}

#[test]
fn log_10_2_5() {
    let expr = "<math><mi>ln</mi><mi>x</mi></math>";
    test_braille("CMU", expr, "⠇⠝⠄⠭");
}

#[test]
fn log_10_2_6() {
    let expr = "<math><mi>alog</mi><mi>x</mi></math>";
    test_braille("CMU", expr, "⠁⠇⠕⠛⠄⠭");
}

#[test]
fn log_10_2_7() {
    let expr = "<math><mi>antilog</mi><mi>x</mi></math>";
    test_braille("CMU", expr, "⠁⠝⠞⠊⠇⠕⠛⠄⠭");
}

#[test]
#[ignore]
fn log_10_2_10() {
    let expr = "<math><mi>log</mi><mn>0,2</mn><mo>=</mo><mover><mn>1</mn><mo>&#xAF;</mo></mover><mo>,</mo><mn>30103</mn></math>";
    test_braille("CMU", expr, "⠇⠕⠛⠄⠼⠚⠂⠃⠶⠼⠥⠂⠉⠚⠁⠚⠉");
}

#[test]
fn ray_11_1_1() {
    let expr = "<math><mover><mi>r</mi><mo>⟷</mo></mover></math>";
    test_braille("CMU", expr, "⠐⠒⠂⠗");
}

#[test]
fn ray_11_1_2() {
    let expr = "<math><mover><mrow><mi>P</mi><mi>Q</mi></mrow><mo>⟷</mo></mover></math>";
    test_braille("CMU", expr, "⠐⠒⠂⠢⠨⠏⠨⠟⠔");
}

#[test]
fn ray_11_1_3() {
    let expr = "<math><mover><mi>z</mi><mo>→</mo></mover></math>";
    test_braille("CMU", expr, "⠒⠂⠵");
}

#[test]
fn ray_11_1_4() {
    let expr = "<math><mover><mi>z</mi><mo>⟵</mo></mover></math>";
    test_braille("CMU", expr, "⠐⠒⠵");
}

#[test]
fn angle_11_1_7() {
    let expr = "<math><mover><mi>A</mi><mo>^</mo></mover></math>";
    test_braille("CMU", expr, "⠘⠒⠨⠁");
}

#[test]
fn angle_11_1_8() {
    let expr = "<math><mover><mrow><mi>a</mi><mi>o</mi><mi>b</mi></mrow><mo>^</mo></mover></math>";
    test_braille("CMU", expr, "⠘⠒⠢⠁⠕⠃⠔");
}

#[test]
fn vector_11_2_3() {
    let expr = "<math><mfenced open='|' close='|'><mover><mi>v</mi><mo>⟶</mo></mover></mfenced></math>";
    test_braille("CMU", expr, "⠸⠀⠒⠂⠧⠸⠀");
}

#[test]
fn vector_11_2_4() {
    let expr = "<math><mfenced open='‖' close='‖'><mover><mi>v</mi><mo>⟶</mo></mover></mfenced></math>";
    test_braille("CMU", expr, "⠸⠇⠒⠂⠧⠸⠇");
}

#[test]
fn vector_11_2_5() {
    let expr = "<math><mfenced open='|' close='|'><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo>⟶</mo></mover></mfenced></math>";
    // modified to include space at end as in vector_11_2_3
    test_braille("CMU", expr, "⠸⠀⠒⠂⠢⠨⠁⠨⠃⠔⠸⠀");
}

#[test]
fn vector_11_2_6() {
    let expr = "<math><mfenced open='‖' close='‖'><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo>⟶</mo></mover></mfenced></math>";
    test_braille("CMU", expr, "⠸⠇⠒⠂⠢⠨⠁⠨⠃⠔⠸⠇");
}

#[test]
fn vector_11_2_9() {
    let expr = "<math><mfenced open='[' close=']'><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo>⟶</mo></mover></mfenced></math>";
    test_braille("CMU", expr, "⠷⠒⠂⠢⠨⠁⠨⠃⠔⠾");
}

#[test]
fn parallel_11_3_1() {
    let expr = "<math><mover><mi>l</mi><mo>&#x2194;</mo></mover><mo>&#x2225;</mo><menclose notation='top'><mi>M</mi><mi>N</mi></menclose></math>";
    test_braille("CMU", expr, "⠐⠒⠂⠇⠸⠇⠈⠉⠢⠨⠍⠨⠝⠔");
}

#[test]
fn perpendicular_11_3_2() {
    let expr = "<math><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo>&#x2194;</mo></mover><mo>&#x22A5;</mo><mover><mrow><mi>O</mi><mi>X</mi></mrow><mo>&#x2192;</mo></mover></math>";
    test_braille("CMU", expr, "⠐⠒⠂⠢⠨⠁⠨⠃⠔⠼⠄⠒⠂⠢⠨⠕⠨⠭⠔");
}

#[test]
fn vector_11_4_1() {
    let expr = "<math><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>&#xB7;</mo><mover><mi>w</mi><mo>&#x2192;</mo></mover></math>";
    test_braille("CMU", expr, "⠒⠂⠧⠠⠀⠒⠂⠺");
}

#[test]
fn vector_11_4_2() {
    let expr = "<math><mfenced open='&lt;' close='&gt;'><mrow><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>,</mo><mover><mi>w</mi><mo>&#x2192;</mo></mover></mrow></mfenced></math>";
    test_braille("CMU", expr, "⠐⠅⠒⠂⠧⠀⠂⠒⠂⠺⠨⠂");
}


#[test]
fn vector_11_4_3() {
    let expr = "<math><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>×</mo><mover><mi>w</mi><mo>&#x2192;</mo></mover></math>";
    test_braille("CMU", expr, "⠒⠂⠧⠈⠦⠒⠂⠺");
}

#[test]
fn vector_11_4_4() {
    let expr = "<math><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>+</mo><mover><mi>w</mi><mo>&#x2192;</mo></mover></math>";
    test_braille("CMU", expr, "⠒⠂⠧⠖⠒⠂⠺");
}

#[test]
fn degrees_11_5_1() {
    let expr = "<math><mn>90</mn><mo>&#xB0;</mo></math>";
    test_braille("CMU", expr, "⠼⠊⠚⠴");
}

#[test]
fn degrees_11_5_2() {
    let expr = "<math><mn>37</mn><mo>&#xB0;</mo><mn>22</mn><mo>'</mo><mn>49</mn><mo>''</mo></math>";
    test_braille("CMU", expr, "⠼⠉⠛⠴⠼⠃⠃⠳⠼⠙⠊⠳⠳");
}

#[test]
fn triangle_11_6() {
    let expr = "<math><mo>&#x25B3;</mo><mi>a</mi><mi>b</mi><mi>c</mi></math>";
    test_braille("CMU", expr, "⠠⠾⠁⠃⠉");
}

#[test]
fn cancellation_14_3_3() {
    // this uses various forms of crossouts to make the test better -- the original only has horizontal crossouts. All should have the same braille.
    let expr = "<math>
        <mfrac>
            <mrow>
                <menclose notation='downdiagonalstrike updiagonalstrike'><mn>2</mn><mi>x</mi></menclose>
                <mo>(</mo><mi>x</mi><mo>-</mo><mn>2</mn><mo>)</mo>
                <menclose notation='updiagonalstrike'>
                    <msup> <mrow><mo>(</mo><mi>x</mi><mo>-</mo><mn>1</mn><mo>)</mo></mrow> <mn>3</mn> </msup>
                </menclose>
            </mrow>
            <mrow>
                <menclose notation='downdiagonalstrike updiagonalstrike'><mn>2</mn><mi>x</mi></menclose>
                <mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo>
                <menclose notation='horizontalstrike'>
                    <mo>(</mo><mi>x</mi><mo>-</mo><mn>1</mn><mo>)</mo>
                </menclose>
            </mrow>
        </mfrac>
    </math>";
    test_braille("CMU", expr, "⠢⠻⠢⠼⠃⠭⠔⠣⠭⠤⠼⠃⠜⠻⠢⠣⠭⠤⠼⠁⠜⠡⠼⠉⠔⠔⠲⠢⠻⠢⠼⠃⠭⠔⠣⠭⠖⠼⠁⠜⠻⠣⠭⠤⠼⠁⠜⠔");
}

// FIX: add tests for color


#[test]
fn omission_14_5_1() {
    // Single and double '_' are used (from WIRIS) for a more robust test -- example seems to use two "_"s for a blank (added 'intent')
    let expr = "<math><mn>5</mn><mi intent=':blank'>_</mi><mn>4</mn><mi intent=':blank'>_</mi><mi intent=':blank'>_</mi><mn>2</mn><mo>=</mo><mn>10</mn></math>";
    test_braille("CMU", expr, "⠼⠑⠰⠼⠙⠰⠼⠃⠶⠼⠁⠚");
}

#[test]
fn omission_14_5_2() {
    // copied from example and pasted into WIRIS
    let expr = "<math><mn>12</mn><mo>+</mo><mn>13</mn><mo>=</mo><mi intent=':blank'>_</mi><mo>&#xA0;</mo><mi intent=':blank'>_</mi></math>";
    test_braille("CMU", expr, "⠼⠁⠃⠖⠼⠁⠉⠶⠼⠰⠰");
}

#[test]
fn omission_14_5_3() {
    // copied from example and pasted into WIRIS
    let expr = "<math><mn>23</mn><mo>+</mo><mn>145</mn><mo>=</mo><mn>1</mn><mi intent=':blank'>_</mi><mo>&#xA0;</mo><mi intent=':blank'>_</mi></math>";
    test_braille("CMU", expr, "⠼⠃⠉⠖⠼⠁⠙⠑⠶⠼⠁⠰⠰");
}

#[test]
fn omission_14_5_4() {
    // copied from example and pasted into WIRIS
    let expr = "<math><mn>719</mn><mo>+</mo><mn>83</mn><mo>=</mo><mi intent=':blank'>_</mi><mi intent=':blank'>_</mi></math>";
    test_braille("CMU", expr, "⠼⠛⠁⠊⠖⠼⠓⠉⠶⠰⠤⠆");
}

#[test]
fn omission_14_5_5() {
    let expr = "<math><mn>3</mn><mo>&#xF7;</mo><mn>12</mn><mo>=</mo><mn>18</mn><mo>&#xF7;</mo><menclose notation='box'><mo>&#xA0;</mo><mo>&#xA0;</mo><mo>&#xA0;</mo></menclose></math>";
    test_braille("CMU", expr, "⠼⠉⠂⠆⠀⠶⠼⠁⠓⠲⠰⠤⠆");
}

#[test]
fn units_appendix_1_2_1() {
    let expr = "<math>
        <mi mathvariant='normal' intent=':unit'>J</mi>
        <mo>=</mo>
        <mi intent=':unit'>kg</mi>
        <mo>&#xA0;</mo>
        <msup><mi mathvariant='normal' intent=':unit'>m</mi><mn>2</mn></msup>
        <mo>&#xA0;</mo>
        <msup><mi mathvariant='normal' intent=':unit'>s</mi><mrow><mo>-</mo><mn>2</mn></mrow></msup>
    </math>";
    test_braille("CMU", expr, "⠨⠚⠶⠅⠛⠀⠍⠡⠼⠃⠀⠎⠡⠤⠼⠃");
}

#[test]
fn units_appendix_1_3_1() {
    // manually added "intent" -- should have another test
    let expr = "<math><mn>1</mn><mo>&#xA0;</mo><mi mathvariant='normal' intent=':unit'>m</mi><mo>=</mo><mn>100</mn><mo>&#xA0;</mo><mi intent=':unit'>cm</mi><mo>=</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>&#xA0;</mo><mi intent=':unit'>km</mi></math>";
    test_braille_prefs("CMU", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠁⠀⠍⠶⠼⠁⠚⠚⠀⠉⠍⠶⠼⠚⠂⠁⠀⠅⠍");

}
#[test]
#[ignore]  // remove 'ignore' once MathCAT adds code to deal with Units/inferring units
fn units_appendix_1_3_1_auto_intent() {
    // manually added "intent" -- should have another test
    let expr = "<math><mn>1</mn><mo>&#xA0;</mo><mi mathvariant='normal' >m</mi><mo>=</mo><mn>100</mn><mo>&#xA0;</mo><mi >cm</mi><mo>=</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>&#xA0;</mo><mi >km</mi></math>";
    test_braille_prefs("CMU", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "⠼⠁⠀⠍⠶⠼⠁⠚⠚⠀⠉⠍⠶⠼⠚⠂⠁⠀⠅⠍");

}



// *********************************************************



// FIX: add 2.6 (Chemistry)

//  FIX: add 3.4 math variants


// FIX: add 3.6 ordinals (drop numbers)


#[test]
fn roman_numerals_simple() {
    // not in spec, but the spec example is very complicated. Here's a simpler test that tests for a single cap indicator
    let expr = "<math><mi>XVI</mi></math>";
    test_braille("CMU", expr, "⠨⠭⠧⠊");
}

#[test]
#[ignore]
fn roman_numerals_appendix_2_2_1() {
    let expr = "<math>
        <menclose notation='top'><menclose notation='top'><mi>VI</mi></menclose></menclose>
        <menclose notation='top'><mi>XL</mi></menclose>
        <mi>DXXI</mi>
    </math>";
    test_braille("CMU", expr, "⠨⠧⠊⠒⠒⠭⠇⠒⠙⠭⠭⠊");
}

#[test]
fn money_appendix_2_3_1() {
    let expr = "<math><mo>$</mo><mn>10</mn><mo>=</mo><mn>1000</mn><mo>&#xA2;</mo></math>";
    test_braille("CMU", expr, "⠸⠏⠼⠁⠚⠶⠼⠁⠚⠚⠚⠘⠉");
}

#[test]
fn money_appendix_2_3_2() {
    let expr = "<math><mn>5</mn><mo>$</mo><mo>=</mo><mn>4,23</mn><mo>&#x20AC;</mo></math>";
    test_braille("CMU", expr, "⠼⠑⠸⠎⠶⠼⠙⠂⠃⠉⠸⠑");
}
