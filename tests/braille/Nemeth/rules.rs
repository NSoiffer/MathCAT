// Nemeth tests for the basic mathml tags
use crate::common::*;

#[test]
fn non_list_rule_10_6_4() {
    let expr = "<math><mo>(</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mtext>and&#xA0;</mtext><mn>3</mn><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠠⠀⠼⠆⠠⠀⠁⠝⠙⠀⠼⠒⠾");
}

#[test]
fn list_num_ind_rule_11_1() {
    let expr = "<math><mo>[</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>]</mo></math>";
    test_braille("Nemeth", expr, "⠈⠷⠴⠠⠀⠂⠈⠾");
}

#[test]
fn list_num_ind_rule_11_2() {
    let expr = "<math><mo>(</mo><mo>-</mo><mn>1</mn><mo>,</mo><mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>3</mn><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠤⠂⠠⠀⠤⠆⠠⠀⠤⠒⠾");
}

#[test]
fn list_num_ind_rule_11_3() {
    let expr = "<math><mo>(</mo><mn>1</mn><mo>+</mo><mi>h</mi><mo>,</mo><mn>2</mn><mo>+</mo><mi>k</mi><mo>,</mo><mn>0</mn><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠂⠬⠓⠠⠀⠆⠬⠅⠠⠀⠴⠾");
}

#[test]
fn list_num_ind_rule_11_7() {
    let expr = "<math><mo>(</mo><mi>x</mi><mo>,</mo><mn>7</mn><mo>,</mo><mn mathvariant='bold'>8</mn><mo>,</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠷⠭⠠⠀⠶⠠⠀⠸⠼⠦⠠⠀⠽⠾");
}

#[test]
fn simple_frac_rule_62_a_3() {
    let expr = "<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mi>c</mi></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠁⠬⠃⠌⠉⠼");
}

#[test]
fn beveled_frac_rule_62_b_1() {
    let expr = "<math><mfrac bevelled='true'>
        <mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow>
        <mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow>
        </mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠁⠬⠃⠸⠌⠉⠬⠙⠼");
}

#[test]
fn mixed_frac_rule_63_a_1() {
    let expr = "<math><mn>4</mn><mfrac><mn>3</mn><mn>8</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠼⠲⠸⠹⠒⠌⠦⠸⠼");
}

#[test]
fn mixed_frac_rule_64_2() {
    let expr = "<math><mn>4</mn><mn>3</mn><mo>/</mo><mn>8</mn></math>";
    test_braille("Nemeth", expr, "⠼⠲⠸⠹⠒⠸⠌⠦⠸⠼");
}

#[test]
fn complex_frac_rule_66_1() {
    let expr = "<math><mfrac><mfrac><mn>3</mn><mn>8</mn></mfrac><mn>5</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠠⠹⠹⠒⠌⠦⠼⠠⠌⠢⠠⠼");
}

#[test]
fn non_hyper_complex_frac_rule_67_1() {
    let expr = "<math><mfrac><mi>a</mi><msup><mi>b</mi>
            <mfrac>
                <mfrac><mn>3</mn><mn>4</mn></mfrac>
                <mfrac><mn>5</mn><mn>6</mn></mfrac>
            </mfrac>
        </msup></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠁⠌⠃⠘⠠⠹⠹⠒⠌⠲⠼⠠⠌⠹⠢⠌⠖⠼⠠⠼⠐⠼");
}

#[test]
fn hyper_complex_frac_rule_68_a_1() {
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
fn numeric_sub() {
    let expr = "<math><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>1</mn></math>";
    test_braille("Nemeth", expr, "⠭⠘⠆⠐⠬⠂");
}

#[test]
fn comma_sub() {
    let expr = "<math><msub><mi>x</mi><mrow><mn>1</mn><mo>,</mo><mn>2</mn></mrow></msub></math>";
    test_braille("Nemeth", expr, "⠭⠰⠂⠪⠆");
}

#[test]
fn primed_numeric_sub() {
    let expr = "<math><msub><mi>x</mi><mn>2</mn></msub><mo>+</mo><msub><msup><mi>x</mi><mo>&#x2032;</mo></msup><mn>1</mn></msub></math>";
    test_braille("Nemeth", expr, "⠭⠆⠬⠭⠄⠂");
}

#[test]
fn nested_super() {
    let expr = "<math><mfrac><mrow><msup><mi>e</mi><mrow><msup><mi>x</mi><mn>2</mn></msup></mrow></msup></mrow><mn>2</mn></mfrac></math>";
    test_braille("Nemeth", expr, "⠹⠑⠘⠭⠘⠘⠆⠐⠌⠆⠼");
}

#[test]
fn nested_super_space() {
    let expr = "<math><mrow><msup><mi>e</mi><mrow><msup><mo>sin</mo><mn>2</mn></msup><mi>x</mi></mrow></msup></mrow></math>";
    test_braille("Nemeth", expr, "⠑⠘⠎⠊⠝⠘⠘⠆⠀⠭");
}

#[test]
fn simple_sqrt() {
    let expr = "<math><msqrt><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠬⠽⠻");
}

#[test]
fn simple_sqrt_with_sup() {
    let expr = "<math><msqrt>
            <msup><mi>x</mi><mn>2</mn></msup>
            <mo>+</mo>
            <msup><mi>y</mi><mn>2</mn></msup>
        </msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠘⠆⠐⠬⠽⠘⠆⠐⠻");
}

#[test]
fn square_root_symbol() {
    let expr = "<math><mo>√</mo><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠜⠷⠭⠬⠽⠾");
}

#[test]
fn simple_cube_root() {
    let expr = "<math><mroot><mn>2</mn><mn>3</mn></mroot></math>";
    test_braille("Nemeth", expr, "⠣⠒⠜⠆⠻");
}

#[test]
fn mroot_non_trivial_index() {
    let expr = "<math><mroot>
            <mrow><mi>p</mi><mo>+</mo><mi>q</mi></mrow>
            <mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow>
        </mroot></math>";
    test_braille("Nemeth", expr, "⠣⠍⠬⠝⠜⠏⠬⠟⠻");
}

#[test]
fn nested_sqrt() {
    let expr = "<math><msqrt><mi>x</mi><mo>+</mo>
            <msqrt><mi>x</mi><mo>+</mo><mi>y</mi></msqrt>
            <mo>+</mo><mi>z</mi></msqrt></math>";
    test_braille("Nemeth", expr, "⠜⠭⠬⠨⠜⠭⠬⠽⠨⠻⠬⠵⠻");
}

#[test]
fn nested_cube_root() {
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
fn nested_sqrt_cube_root() {
    let expr = "<math>
        <msqrt> <mroot> <mi>x</mi><mn>3</mn> </mroot> </msqrt>
        <mo>=</mo>
        <mroot> <msqrt><mi>x</mi></msqrt> <mn>3</mn></mroot>
    </math>";
    test_braille("Nemeth", expr, "⠜⠨⠣⠒⠜⠭⠨⠻⠻⠀⠨⠅⠀⠣⠒⠜⠨⠜⠭⠨⠻⠻");
}

#[test]
fn nested_3_sqrt() {
    let expr = "<math>
            <msqrt><mi>x</mi><mo>+</mo><msqrt><mi>y</mi><mo>+</mo><msqrt><mi>z</mi></msqrt></msqrt></msqrt>
        </math>";
    test_braille("Nemeth", expr, "⠜⠭⠬⠨⠜⠽⠬⠨⠨⠜⠵⠨⠨⠻⠨⠻⠻");
}

#[test]
fn underbar_rule_86_a_1() {
    let expr = "<math><munder><mi>x</mi><mo>&#xAF;</mo></munder></math>";
    test_braille("Nemeth", expr, "⠐⠭⠩⠱⠻");
}

#[test]
fn lim_rule_86_a_3() {
    let expr = "<math><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><mn>0</mn></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>";
    test_braille("Nemeth", expr, "⠐⠇⠊⠍⠩⠭⠀⠫⠕⠀⠼⠴⠻⠀⠋⠷⠭⠾");
}

#[test]
fn overbar_rule_86_a_5() {
    let expr = "<math><mover><msup><mi>x</mi><mn>2</mn></msup><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠐⠭⠘⠆⠐⠣⠱⠻");
}

#[test]
fn overbar_rule_86_b_1() {
    let expr = "<math><mover><mi>x</mi><mo>&#xAF;</mo></mover></math>";
    test_braille("Nemeth", expr, "⠭⠱");
}
#[test]
fn order2_overbar_rule_87_a_1() {
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
