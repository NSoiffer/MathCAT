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
#[ignore]
fn letter_number_1_1_1() {
    // 5x=40b
    let expr = "<math><mn>5</mn><mi>x</mi><mo>=</mo><mn>40</mn><mi>b</mi></math>";
    test_braille("CMU", expr, "⠼⠑⠭⠶⠼⠙⠚⠐⠃");
}

#[test]
#[ignore]
fn dot_1_1_2() {
    let expr = "<math><mover><mi>p</mi><mo>&#x2D9;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠐⠏");
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
#[ignore]   // need to add a pref that 
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
    test_braille("CMU", expr, "⠼⠉⠁⠄⠛⠃⠚");
}

#[test]
fn number_2_2_2() {
    let expr = "<math><mn>3 802 197</mn></math>";
    test_braille("CMU", expr, "⠼⠉⠄⠓⠚⠃⠄⠁⠊⠛");
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
    test_braille("CMU", expr, "⠤⠼⠃⠑⠄⠉⠙⠛");
}

#[test]
fn number_2_3_1() {
    let expr = "<math><mn>3.2</mn></math>";
    test_braille("CMU", expr, "⠼⠉⠂⠃");
}

#[test]
fn number_2_3_2() {
    let expr = "<math><mn>3,2</mn></math>";
    test_braille("CMU", expr, "⠼⠉⠂⠃");
}

#[test]
fn number_2_3_3() {
    let expr = "<math><mn>3’2</mn></math>";
    test_braille("CMU", expr, "⠼⠉⠂⠃");
}

#[test]
fn number_2_3_3_wiris() {
    let expr = "<math><mn>3</mn><mo>'</mo><mn>2</mn></math>";
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
    test_braille("CMU", expr, "⠵⠡⠤⠼⠁⠆");
}

#[test]
fn script_4_2_1_13() {
    // z_i_0
    let expr = "<math><msub><mi>z</mi><msub><mi>i</mi><mn>0</mn></msub></msub></math>";
    test_braille("CMU", expr, "⠵⠌⠊⠌⠼⠚");
}

#[test]
fn script_4_2_2_1() {
    // just first entry because 2D not supported yet
    let expr = "<math><msub><mi>a</mi><mn>11</mn></msub></math>";
    test_braille("CMU", expr, "⠁⠂⠂");
}

#[test]
#[ignore] // this appears to be optional -- implementation seems subjective especially when not in a table
fn script_4_2_2_2() {
    // just first entry because 2D not supported yet
    let expr = "<math><msub><mi>a</mi><mrow><mn>1</mn><mo>,</mo><mn>1</mn></mrow></msub></math>";
    test_braille("CMU", expr, "⠁⠂⠂");
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
    // z^{****}
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
        <msub><mrow/><mrow><mo>+</mo><mo>+</mo></mrow></msub>
        <msup>
        <mi>z</mi>
        <msup><mi/><mo>&#x2033;</mo></msup>
        </msup>
    </math>";
    test_braille("CMU", expr, "⠵⠳⠳⠂⠌⠖⠖⠄");
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
    let expr = "<math><msup><mi>z</mi><msup><mo>'</mo><mn>3</mn></msup></msup></math>";
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
    test_braille("CMU", expr, "⠈⠉⠷⠵⠳⠌⠼⠚⠾⠡⠼⠃");
}

#[test]
fn both_scripts_4_4_1_7() {
    let expr = "<math><msup>
            <mover><msubsup><mi>z</mi><mn>0</mn><mi>&#x2032;</mi></msubsup><mo>&#x2015;</mo></mover>
            <mn>2</mn>
        </msup></math>";
    test_braille("CMU", expr, "⠈⠉⠢⠵⠌⠼⠚⠔");
}

#[test]
fn scripts_4_4_2_3() {
    let expr = "<math><msubsup><mi>T</mi><mi>s</mi><mi>r</mi></msubsup></math>";
    test_braille("CMU", expr, "⠈⠉⠢⠵⠳⠌⠼⠚⠔⠡⠼⠃");
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
fn list_9_1_2() {
    let expr = "<math><mo>(</mo><msub><mi>x</mi><mn>1</mn></msub><mo>,</mo><msub><mi>x</mi><mn>2</mn></msub><mo>)</mo></math>";
    test_braille("CMU", expr, "⠣⠭⠌⠼⠁⠀⠂⠭⠌⠼⠃⠜");
}

#[test]
fn list_9_1_3() {
    let expr = "<math><mfenced><mrow><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>,</mo><mfrac><mn>3</mn><mn>2</mn></mfrac></mrow></mfenced></math>";
    test_braille("CMU", expr, "⠣⠼⠁⠆⠀⠂⠼⠉⠆⠀⠜");
}



// *********************************************************

#[test]
#[ignore]
fn greek_dot_1_1_4() {
    let expr = "<math><mover><mi>π</mi><mo>&#x2D9;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠈⠏");
}

#[test]
#[ignore]
fn strike_1_1_5() {
    let expr = "<math><menclose notation=\"horizontalstrike\"><mi>p</mi></menclose></math>";
    test_braille("CMU", expr, "⠘⠐⠏");
}
#[test]
fn strike_1_1_7() {
    let expr = "<math><menclose notation=\"horizontalstrike\"><mi>Ω</mi></menclose></math>";
    test_braille("CMU", expr, "⠘⠘⠺");
}
#[test]
#[ignore]
fn strike_1_1_8() {
    let expr = "<math><menclose notation=\"horizontalstrike\"><mi>β</mi></menclose></math>";
    test_braille("CMU", expr, "⠘⠈⠃");
}

#[test]
#[ignore]
fn greater_o_1_1_9() {
    let expr = "<math><mi>a</mi><mo>≫</mo><mi>o</mi></math>";
    test_braille("CMU", expr, "⠁⠕⠕⠐⠕");
}


#[test]
#[ignore]
fn grouping_1_3_3() {
    let expr = "<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow></mfrac></math>";
    test_braille("CMU", expr, "⠢⠁⠖⠃⠔⠲⠢⠉⠖⠙⠔");
}

#[test]
#[ignore]
fn grouping_1_3_4() {
    let expr = "<math><mfrac>
            <mfenced><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow></mfenced>
            <mfenced><mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow></mfenced>
        </mfrac></math>";
    test_braille("CMU", expr, "⠣⠁⠖⠃⠜⠲⠣⠉⠖⠙⠜");
}

#[test]
#[ignore]
fn grouping_1_3_5() {
    let expr = "<math><mfrac>
            <mrow><mi>a</mi><mo>+</mo>
                <mfrac><mi>b</mi><mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow></mfrac>
            </mrow>
            <mi>e</mi>
        </mfrac></math>";
    test_braille("CMU", expr, "⠢⠁⠖⠃⠲⠢⠉⠖⠙⠔⠔⠲⠑");
}



// FIX: add tests for 2.4.2 (Special cases for scripts)

// FIX: add 2.6 (Chemistry)

// FIX: add 3.2.1 (repeating numbers)

//  FIX: add 3.4 math variants


// FIX: add 3.6 ordinals (drop numbers)

// FIX: ad 3.7 Roman numerals

#[test]
fn units_3_8_1() {
    let expr = "<math><mn>8</mn><mi mathvariant='normal'>m</mi></math>";
    test_braille("CMU", expr, "⠼⠓⠍");
}

#[test]
fn units_3_8_3() {
    let expr = "<math><mn>12</mn><mi>cm</mi></math>";
    test_braille("CMU", expr, "⠼⠁⠃⠐⠉⠍");
}

#[test]
fn units_3_8_6() {
    let expr = "<math><mn>1</mn><msup><mtext>km</mtext><mn>2</mn></msup></math>";
    test_braille("CMU", expr, "⠼⠁⠅⠍⠡⠼⠃");
}

#[test]
fn units_3_8_14() {
    let expr = "<math><mn>17</mn><mo>&#xB0;</mo></math>";
    test_braille("CMU", expr, "⠼⠁⠛⠴");
}

#[test]
fn units_3_8_15() {
    let expr = "<math><mn>2</mn><mo>&#xB0;</mo><mn>4</mn><mo>'</mo></math>";
    test_braille("CMU", expr, "⠼⠃⠴⠼⠙⠳");
}

#[test]
fn units_3_8_19() {
    let expr = "<math><mtext>h</mtext><mo>.</mo><mn>5</mn><mo>.</mo><mn>30</mn></math>";
    test_braille("CMU", expr, "⠓⠄⠼⠑⠄⠼⠉⠚");
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
    let expr = "<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><msup><mn>0</mn><mo>-</mo></msup></mrow></munder></math>";
    test_braille("CMU", expr, "⠇⠊⠍⠄⠭⠒⠂⠼⠚⠤⠄⠱");
}

#[test]
fn limit_9_2_6() {
    let expr = "<math><mrow><munder><mrow><mi>lim</mi></mrow><mrow><mi>x</mi><mo>↓</mo><mi>c</mi></mrow></munder></mrow></math>";
    test_braille("CMU", expr, "⠇⠊⠍⠄⠭⠸⠄⠉⠱");
}

#[test]
fn deriv_7_3_1() {
    let expr = "<math><mfrac><mi>d</mi><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>";
    test_braille("CMU", expr, "⠙⠲⠢⠙⠭⠔");
}

#[test]
fn deriv_7_3_2() {
    let expr = "<math><mfrac><mrow><mi>d</mi><mi>f</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>";
    test_braille("CMU", expr, "⠢⠙⠋⠔⠲⠢⠙⠭⠔");
}

#[test]
fn deriv_7_3_3() {
    let expr = "<math><mfrac>
            <msup><mi>d</mi><mi>n</mi></msup>
            <mrow><mi>d</mi><msup><mi>x</mi><mi>n</mi></msup></mrow>
        </mfrac></math>";
    test_braille("CMU", expr, "⠙⠡⠝⠲⠢⠙⠭⠡⠝⠔");
}

#[test]
fn deriv_7_3_4() {
    let expr = "<math><mfrac>
            <mrow><msup><mi>d</mi><mi>n</mi></msup><mi>f</mi></mrow>
            <mrow><mi>d</mi><msup><mi>x</mi><mi>n</mi></msup></mrow>
        </mfrac></math>";
    test_braille("CMU", expr, "⠢⠙⠡⠝⠋⠔⠲⠢⠙⠭⠡⠝⠔");
}

#[test]
fn partial_7_3_9() {
    // Note: fixed apparent bug with division symbol in manual
    let expr = "<math><mfrac>
            <msup><mo>&#x2202;</mo><mn>2</mn></msup>
            <mrow><mo>&#x2202;</mo><mi>x</mi><mo>&#x2202;</mo><mi>y</mi></mrow>
        </mfrac></math>";
    test_braille("CMU", expr, "⠸⠙⠡⠼⠃⠲⠢⠸⠙⠭⠸⠙⠽⠔");
}

#[test]
fn partial_7_3_10() {
    // Note: fixed apparent bug with division symbol in manual
    let expr = "<math><mfrac>
            <mrow><msup><mo>&#x2202;</mo><mn>2</mn></msup><mi>f</mi></mrow>
            <mrow><mo>&#x2202;</mo><mi>x</mi><mo>&#x2202;</mo><mi>y</mi></mrow>
        </mfrac></math>";
    test_braille("CMU", expr, "⠢⠸⠙⠡⠼⠃⠐⠋⠔⠲⠢⠸⠙⠭⠸⠙⠽⠔");
}

#[test]
fn partial_7_3_11() {
    let expr = "<math><mfrac>
            <msup><mo>&#x2202;</mo><mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow></msup>
            <mrow><mo>&#x2202;</mo><msup><mi>x</mi><mi>m</mi></msup><mo>&#x2202;</mo><msup><mi>y</mi><mi>n</mi></msup></mrow>
        </mfrac></math>";
    test_braille("CMU", expr, "⠸⠙⠡⠢⠍⠖⠝⠔⠲⠢⠸⠙⠭⠡⠍⠸⠙⠽⠡⠝⠔");
}

#[test]
fn partial_7_3_12() {
    let expr = "<math><mfrac>
            <mrow><msup><mo>&#x2202;</mo><mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow></msup><mi>f</mi></mrow>
            <mrow><mo>&#x2202;</mo><msup><mi>x</mi><mi>m</mi></msup><mo>&#x2202;</mo><msup><mi>y</mi><mi>n</mi></msup></mrow>
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
fn succession_7_5_1_2() {
    let expr = "<math><mo>(</mo><msub><mi>s</mi><mi>n</mi></msub><mo>)</mo></math>";
    test_braille("CMU", expr, "⠣⠎⠌⠝⠜");
}

#[test]
fn lim_7_5_1_3() {
    let expr = "<math><munder><mi>lim</mi><mrow><mi>n</mi><mo>&#x2192;</mo><mo>&#x221E;</mo></mrow></munder>
                        <msub><mi>s</mi><mi>n</mi></msub></math>";
    test_braille("CMU", expr, "⠇⠊⠍⠄⠝⠒⠂⠼⠳⠱⠎⠌⠝");
}

#[test]
fn log_7_5_2_1() {
    let expr = "<math><mi>log</mi><mi>x</mi></math>";
    test_braille("CMU", expr, "⠇⠕⠛⠄⠭");
}

#[test]
fn log_7_5_2_2() {
    let expr = "<math><mi>antilog</mi><mi>x</mi></math>";
    test_braille("CMU", expr, "⠁⠝⠞⠊⠇⠕⠛⠄⠭");
}

#[test]
fn log_7_5_2_3() {
    let expr = "<math><mi>alog</mi><mi>x</mi></math>";
    test_braille("CMU", expr, "⠁⠇⠕⠛⠄⠭");
}

#[test]
fn log_7_5_2_5() {
    let expr = "<math><mi>ln</mi><mi>x</mi></math>";
    test_braille("CMU", expr, "⠇⠝⠄⠭");
}

#[test]
fn log_7_5_2_8() {
    let expr = "<math><msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></math>";
    test_braille("CMU", expr, "⠇⠕⠛⠄⠃⠱⠭");
}

#[test]
fn log_7_5_2_10() {
    let expr = "<math><mi>log</mi><mfrac><mi>x</mi><mi>y</mi></mfrac></math>";
    test_braille("CMU", expr, "⠇⠕⠛⠄⠢⠭⠲⠽⠔");
}

#[test]
fn log_7_5_2_11() {
    let expr = "<math><mi>log</mi><mfrac><mn>1</mn><mn>100</mn></mfrac><mo>=</mo><mo>-</mo><mn>2</mn></math>";
    test_braille("CMU", expr, "⠇⠕⠛⠄⠼⠂⠁⠚⠚⠶⠤⠼⠃");
}

#[test]
fn log_7_5_2_12() {
    let expr = "<math><msub><mi>log</mi><mn>4</mn></msub><mn>64</mn></math>";
    test_braille("CMU", expr, "⠇⠕⠛⠄⠼⠙⠱⠼⠋⠙");
}

#[test]
fn trig_7_5_3_1() {
    let expr = "<math><mi>sen</mi><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo></math>";
    test_braille("CMU", expr, "⠎⠑⠝⠄⠣⠁⠖⠃⠜");
}

#[test]
fn trig_7_5_3_3() {
    let expr = "<math><mi>tan</mi><mo>(</mo><mi>α</mi><mo>)</mo><mo>=</mo><mfrac>
                <mrow><mi>sin</mi><mo>(</mo><mi>α</mi><mo>)</mo></mrow>
                <mrow><mi>cos</mi><mo>(</mo><mi>α</mi><mo>)</mo></mrow>
                </mfrac></math>";
    test_braille("CMU", expr, "⠞⠁⠝⠄⠣⠈⠁⠜⠶⠎⠊⠝⠄⠣⠈⠁⠜⠲⠉⠕⠎⠄⠣⠈⠁⠜");
}

#[test]
fn example_7_7_1() {
    let expr = "<math>
        <mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo>
        <mfrac>
            <mrow><mi>sen</mi><mi>x</mi></mrow>
            <mrow><msup><mi>sen</mi><mn>2</mn></msup><mi>x</mi><mo>+</mo><mn>1</mn></mrow>
        </mfrac>
    </math>";
    test_braille("CMU", expr, "⠋⠣⠭⠜⠶⠎⠑⠝⠄⠭⠲⠢⠎⠑⠝⠄⠡⠼⠃⠭⠖⠼⠁⠔");
}

#[test]
fn log_7_7_2() {
    let expr = "<math><mi>log</mi><mfrac>
            <mrow><mi>r</mi><mo>+</mo><mn>1</mn></mrow>
            <mrow><mi>r</mi><mo>-</mo><mn>1</mn></mrow>
        </mfrac></math>";
    test_braille("CMU", expr, "⠇⠕⠛⠄⠢⠢⠗⠖⠼⠁⠔⠲⠢⠗⠤⠼⠁⠔⠔");
}

#[test]
fn log_7_7_3() {
    let expr = "<math><mi>log</mi><mfenced><mfrac>
            <mrow><mi>r</mi><mo>+</mo><mn>1</mn></mrow>
            <mrow><mi>r</mi><mo>-</mo><mn>1</mn></mrow>
        </mfrac></mfenced></math>";
    test_braille("CMU", expr, "⠇⠕⠛⠄⠣⠢⠗⠖⠼⠁⠔⠲⠢⠗⠤⠼⠁⠔⠜");
}

#[test]
#[ignore]
fn sum_7_7_5() {
    let expr = "<math>
        <mrow>
        <munder>
        <mo>&#x2211;</mo>
        <mtable columnalign='left'>
            <mtr><mtd><mrow><mn>1</mn><mo>&#x2264;</mo><mi>i</mi><mo>&#x2264;</mo><mn>4</mn></mrow></mtd></mtr>
            <mtr><mtd><mrow><mi>i</mi><mo>&#x2260;</mo><mn>3</mn></mrow></mtd></mtr>
        </mtable>
        </munder>
        <mn>1</mn><mo>+</mo><mn>2</mn><mo>+</mo><mn>4</mn><mo>=</mo><mn>7</mn></mrow>
   </math>";
    test_braille("CMU", expr, "⠘⠎⠼⠁⠪⠶⠊⠣⠶⠼⠙⠒⠊⠘⠶⠼⠉⠱⠊⠶⠼⠁⠖⠼⠃⠖⠼⠙⠶⠼⠛");
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
