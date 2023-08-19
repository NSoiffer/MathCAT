// Tests from https://www.once.es/servicios-sociales/braille/comision-braille-espanola/documentos-tecnicos/documentos-tecnicos-relacionados-con-el-braille/documentos/b5-signografia-matematica.pdf
// This seems to be official guide.
use crate::common::*;

#[test]
fn letter_number_1_1_1() {
    // 5x=40b
    let expr = "<math><mn>5</mn><mi>x</mi><mo>=</mo><mn>40</mn><mi>b</mi></math>";
    test_braille("CMU", expr, "⠼⠑⠭⠶⠼⠙⠚⠐⠃");
}

#[test]
fn dot_1_1_2() {
    let expr = "<math><mover><mi>p</mi><mo>&#x2D9;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠐⠏");
}

#[test]
fn greek_1_1_3() {
    let expr = "<math><mi>π</mi></math>";
    test_braille("CMU", expr, "⠈⠏");
}

#[test]
fn greek_dot_1_1_4() {
    let expr = "<math><mover><mi>π</mi><mo>&#x2D9;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠈⠏");
}

#[test]
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
fn strike_1_1_8() {
    let expr = "<math><menclose notation=\"horizontalstrike\"><mi>β</mi></menclose></math>";
    test_braille("CMU", expr, "⠘⠈⠃");
}

#[test]
fn greater_o_1_1_9() {
    let expr = "<math><mi>a</mi><mo>≫</mo><mi>o</mi></math>";
    test_braille("CMU", expr, "⠁⠕⠕⠐⠕");
}

#[test]
fn no_space_after_bar_1_3_1() {
    // I think the rule is that dots 1,2,3 and three have to be empty after a bar. If not add a space
    let expr = "<math><mo>|</mo><mrow><mi>a</mi><mo>,</mo><mi>b</mi><mo>,</mo><mi>c</mi></mrow><mo>|</mo><mi>M</mi></math>";
    test_braille("CMU", expr, "⠸⠁⠂⠃⠂⠉⠸⠨⠍");
}

#[test]
fn space_after_bar_1_3_2() {
    // I think the rule is that dots 1,2,3 and three have to be empty after a bar. If not add a space
    let expr = "<math><mfenced open='|' close='|'><mrow><mi>a</mi><mo>,</mo><mi>b</mi><mo>,</mo><mi>c</mi></mrow></mfenced><mn>3</mn></math>";
    test_braille("CMU", expr, "⠸⠁⠂⠃⠂⠉⠸⠀⠼⠉");
}

#[test]
fn grouping_1_3_3() {
    let expr = "<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow></mfrac></math>";
    test_braille("CMU", expr, "⠢⠁⠖⠃⠔⠲⠢⠉⠖⠙⠔");
}

#[test]
fn grouping_1_3_4() {
    let expr = "<math><mfrac>
            <mfenced><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow></mfenced>
            <mfenced><mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow></mfenced>
        </mfrac></math>";
    test_braille("CMU", expr, "⠣⠁⠖⠃⠜⠲⠣⠉⠖⠙⠜");
}

#[test]
fn grouping_1_3_5() {
    let expr = "<math><mfrac>
            <mrow><mi>a</mi><mo>+</mo>
                <mfrac><mi>b</mi><mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow></mfrac>
            </mrow>
            <mi>e</mi>
        </mfrac></math>";
    test_braille("CMU", expr, "⠢⠁⠖⠃⠲⠢⠉⠖⠙⠔⠔⠲⠑");
}


#[test]
fn script_2_2_1() {
    let expr = "<math><msub><mi>z</mi><mi>r</mi></msub></math>";
    test_braille("CMU", expr, "⠵⠌⠗");
}

#[test]
fn script_2_2_2() {
    let expr = "<math><msup><mi>z</mi><mi>r</mi></msup></math>";
    test_braille("CMU", expr, "⠵⠡⠗");
}

#[test]
fn script_2_2_3() {
    let expr = "<math><mmultiscripts><mi>z</mi><mprescripts/><mi>r</mi><none/></mmultiscripts></math>";
    test_braille("CMU", expr, "⠵⠠⠌⠗");
}

#[test]
fn script_2_2_4() {
    let expr = "<math><mmultiscripts><mi>z</mi><mprescripts/><none/><mi>r</mi></mmultiscripts></math>";
    test_braille("CMU", expr, "⠵⠈⠡⠗");
}

#[test]
fn script_2_2_5() {
    let expr = "<math><munder><mi>z</mi><mi>r</mi></munder></math>";
    test_braille("CMU", expr, "⠵⠌⠌⠗");
}

#[test]
fn script_2_2_6() {
    let expr = "<math><mover><mi>z</mi><mi>r</mi></mover></math>";
    test_braille("CMU", expr, "⠵⠡⠡⠗");
}

#[test]
fn script_2_2_7() {
    // z_{n-1}
    let expr = "<math><msub><mi>z</mi><mrow><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msub></math>";
    test_braille("CMU", expr, "⠵⠌⠢⠝⠤⠼⠁⠔");
}

#[test]
fn script_2_2_8() {
    let expr = "<math><msup><mi>z</mi><mrow><mi>i</mi><mo>,</mo><mi>j</mi></mrow></msup></math>";
    test_braille("CMU", expr, "⠵⠡⠢⠊⠂⠚⠔");
}

#[test]
fn script_2_2_9() {
    // z_i_0
    let expr = "<math><msub><mi>z</mi><msub><mi>i</mi><mn>0</mn></msub></msub></math>";
    test_braille("CMU", expr, "⠵⠌⠢⠊⠌⠼⠚⠔");
}

#[test]
fn script_2_2_10() {
    // z_{i_r -1}
    let expr = "<math><msub><mi>z</mi><mrow><msub><mi>i</mi><mi>r</mi></msub><mo>-</mo><mn>1</mn></mrow></msub></math>";
    test_braille("CMU", expr, "⠵⠌⠢⠊⠌⠗⠤⠼⠁⠔");
}

#[test]
fn script_2_2_11() {
    // z_i_{r-1}
    let expr = "<math><msub><mi>z</mi><msub><mi>i</mi><mrow><mi>r</mi><mo>-</mo><mn>1</mn></mrow></msub></msub></math>";
    test_braille("CMU", expr, "⠵⠌⠢⠊⠌⠢⠗⠤⠼⠁⠔⠔");
}

#[test]
fn script_2_2_12() {
    // {}^{n-1}z
    let expr = "<math><mmultiscripts><mi>z</mi><mprescripts/><none/><mrow><mi>n</mi><mo>-</mo><mn>1</mn></mrow></mmultiscripts></math>";
    test_braille("CMU", expr, "⠵⠈⠡⠢⠝⠤⠼⠁⠔");
}

#[test]
fn prime_2_3_1_2() {
    let expr = "<math><msup><mi>z</mi><mo>″</mo></msup></math>";
    test_braille("CMU", expr, "⠵⠳⠳");
}

#[test]
fn super_2_3_1_4() {
    // z^{+}
    let expr = "<math><msup><mi>z</mi><mo>+</mo></msup></math>";
    test_braille("CMU", expr, "⠵⠖⠄");
}

#[test]
fn degree_2_3_1_6() {
    let expr = "<math><msup><mi>z</mi><mo>°</mo></msup></math>";
    test_braille("CMU", expr, "⠵⠴⠄");
}

#[test]
fn sup_minus_2_3_1_10() {
    // z^{--}
    let expr = "<math><msup><mi>z</mi><mrow><mo>-</mo><mo>-</mo></mrow></msup></math>";
    test_braille("CMU", expr, "⠵⠤⠤⠄");
}

#[test]
fn sup_2_3_1_11() {
    // z^{4+}
    let expr = "<math><msup><mi>z</mi><mrow><mn>4</mn><mo>+</mo></mrow></msup></math>";
    test_braille("CMU", expr, "⠵⠡⠼⠙⠖⠄");
}

#[test]
fn greek_dot_2_3_2() {
    let expr = "<math><mover><mi>ζ</mi><mo>¨</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠈⠈⠵");
}

#[test]
fn dot_2_3_3() {
    let expr = "<math><mover><mi>z</mi><mo>&#x20DB;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠈⠈⠐⠵");
}

#[test]
fn bar_2_3_4() {
    let expr = "<math><mover><mi>z</mi><mo>&#xAF;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠉⠵");
}

#[test]
fn bar_2_3_5() {
    // A double bar probably should be U+2550, but I don't know how to get a MathML generator to create this
    let expr = "<math><mover><mi>z</mi><mo>&#x2550;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠉⠈⠉⠵");
}

#[test]
fn underbar_2_3_6() {
    let expr = "<math><munder><mi>z</mi><mo>&#xAF;</mo></munder></math>";
    test_braille("CMU", expr, "⠠⠤⠵");
}

#[test]
fn tilde_2_3_7() {
    let expr = "<math><mover><mi>z</mi><mo>~</mo></mover></math>";
    test_braille("CMU", expr, "⠐⠢⠵");
}

    #[test]
fn bar_2_3_8() {
    let expr = "<math><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo stretchy=\"false\">&#xAF;</mo></mover></math>";
    test_braille("CMU", expr, "⠈⠉⠢⠨⠁⠨⠃⠔");
}

#[test]
fn bar_menclose_2_3_8() {
    let expr = "<math><menclose notation=\"top\"><mi>A</mi><mi>B</mi></menclose></math>";
    test_braille("CMU", expr, "⠈⠉⠢⠨⠁⠨⠃⠔");
}

#[test]
fn bar_menclose_2_3_9() {
    let expr = "<math><menclose notation=\"top\"><mi>z</mi><mo>''</mo></menclose></math>";
    test_braille("CMU", expr, "⠈⠉⠢⠵⠳⠳⠔");
}

#[test]
fn number_3_1_1() {
    let expr = "<math><mn>1.720</mn></math>";
    test_braille("CMU", expr, "⠼⠁⠄⠛⠃⠚");
}

#[test]
fn number_3_1_2() {
    let expr = "<math><mn>3.802.197</mn></math>";
    test_braille("CMU", expr, "⠼⠉⠄⠓⠚⠃⠄⠁⠊⠛");
}

#[test]
fn number_3_1_3() {
    let expr = "<math><mn>1 720</mn></math>";
    test_braille("CMU", expr, "⠼⠁⠄⠛⠃⠚");
}

#[test]
fn number_3_1_4() {
    let expr = "<math><mn>3 802 197</mn></math>";
    test_braille("CMU", expr, "⠼⠉⠄⠓⠚⠃⠄⠁⠊⠛");
}

#[test]
fn number_3_1_5() {
    let expr = "<math><mn>1000</mn><mtext>km</mtext><mo>=</mo><mn>1000000</mn><mtext>m</mtext></math>";
    test_braille("CMU", expr, "⠼⠁⠄⠚⠚⠚⠅⠍⠶⠼⠁⠄⠚⠚⠚⠄⠚⠚⠚⠍");
}

#[test]
fn numeric_fraction_3_2_1() {
    let expr = "<math><mfrac><mn>3</mn><mn>4</mn></mfrac></math>";
    test_braille("CMU", expr, "⠼⠒⠙");
}

#[test]
fn numeric_fraction_3_2_2() {
    let expr = "<math><mn>2</mn><mfrac><mn>3</mn><mn>4</mn></mfrac></math>";
    test_braille("CMU", expr, "⠼⠃⠼⠒⠙");
}

#[test]
fn number_base_3_3_1() {
    let expr = "<math><msub><mn>101</mn><mn>2</mn></msub></math>";
    test_braille("CMU", expr, "⠼⠁⠚⠁⠌⠼⠃");
}

#[test]
fn sqrt_5_3_1() {
    let expr = "<math><msqrt><mi>x</mi></msqrt></math>";
    test_braille("CMU", expr, "⠫⠱⠭");
}

#[test]
fn sqrt_5_3_2() {
    let expr = "<math><msqrt><msup><mi>x</mi><mn>3</mn></msup></msqrt></math>";
    test_braille("CMU", expr, "⠫⠱⠭⠡⠼⠉");
}

#[test]
fn sqrt_5_3_3() {
    let expr = "<math><msqrt><mfrac><mn>4</mn><mn>8</mn></mfrac></msqrt></math>";
    test_braille("CMU", expr, "⠫⠱⠼⠲⠓");
}

#[test]
fn root_5_3_4() {
    let expr = "<math><mroot><mi>x</mi><mn>3</mn></mroot></math>";
    test_braille("CMU", expr, "⠫⠼⠉⠱⠭");
}

#[test]
fn root_5_3_5() {
    let expr = "<math><mroot><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mi>n</mi></mroot></math>";
    test_braille("CMU", expr, "⠫⠝⠱⠢⠁⠖⠃⠔");
}

#[test]
fn root_5_3_6() {
    let expr = "<math><mroot><mi>a</mi><mi>n</mi></mroot><mo>+</mo><mi>b</mi></math>";
    test_braille("CMU", expr, "⠫⠝⠱⠁⠖⠃");
}

#[test]
fn root_5_3_7() {
    let expr = "<math><mroot><mrow><mi>m</mi><mo>-</mo><mn>1</mn></mrow><mrow><mi>n</mi><mo>-</mo><mn>1</mn></mrow></mroot></math>";
    test_braille("CMU", expr, "⠫⠝⠤⠼⠁⠱⠢⠍⠤⠼⠁⠔");
}

#[test]
fn root_5_3_8() {
    let expr = "<math><mroot>
            <mrow><mi>m</mi><mo>-</mo><mn>1</mn></mrow>
            <mfrac><mn>1</mn><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></mfrac>
        </mroot></math>";
    test_braille("CMU", expr, "⠫⠼⠁⠲⠢⠝⠖⠼⠁⠔⠱⠢⠍⠤⠼⠁⠔");
}
