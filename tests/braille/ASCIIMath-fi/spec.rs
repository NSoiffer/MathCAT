// Tests based on the Finnish flavor of ASCIIMath located on the Finnish delegation for braille's "Matematiikan, fysiikan ja kemian merkinnät elektronisissa oppikirjoissa" (https://www.pistekirjoitus.fi/julkaisut/matematiikka-ja-tietotekniikka/).
// Tests will be named according to the page and some identification.
use crate::common::*;


#[test]
fn p12_equal () {
    let expr = r#"<math><mn>3</mn><mo>+</mo><mn>4</mn><mo>=</mo><mn>7</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"3 +4 =7");
}

#[test]
fn p12_not_equal () {
    let expr = r#"<math><mn>5</mn><mo>&#x2212;</mo><mn>2</mn><mo>&#8800;</mo><mn>2</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"5 -2 !=2");
}

#[test]
fn p12_opposite () {
    let expr = r#"<math><mn>9</mn><mo>&#x2212;</mo><mn>3</mn><mo>&#x2260;</mo><mn>5</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"9 -3 != 5");
}

#[test]
fn p12_multiplication_visible_op () {
    let expr = r#"<math><mn>27</mn><mo>&#183;</mo><mn>3</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"27 *3");
}

#[test]
fn p12_simple_frac () {
    let expr = r#"<math><mfrac><mn>1</mn><mn>3</mn></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"1/3");
}

#[test]
fn p12_ratio () {
    let expr = r#"<math><mn>1</mn><mo>:</mo><mn>1000</mn></math>"#;
    test_braille_prefs("ASCIIMath-fi", expr, r"1 :1000");
}

#[test]
fn p12_fractional () {
    let expr = r#"<math><mfrac><mrow><mn>6</mn><mo>&#8290;</mo><mi>x</mi><mo>+</mo><mn>3</mn><mo>&#8290;</mo><mi>x</mi></mrow><mrow><mrow><mn>6</mn><mo>&#8290;</mo><mi>x</mi><mo>&#x2212;</mo><mn>4</mn><mo>&#8290;</mo><mi>x</mi></mrow></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"(6 x +3 x) /(6 x -4 x)");
}

#[test]
fn p12_absolute_value_eq () {
    let expr = r#"<math><mo>|</mo><mo>&#x2212;</mo><mo>(</mo><mn>2</mn><mo>+</mo><mn>5</mn><mo>)</mo><mo>|</mo><mo>=</mo><mo>|</mo><mn>&#x2212;7</mn><mo>|</mo><mo>=</mo><mn>7</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"|-(2 +5)| =|-7| =7");
}

#[test]
fn p12_natural_numbers () {
    let expr = r#"<math><mi>&#x2115;</mi><mo>=</mo><mo>{</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo>&#8230;</mo><mo>}</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"NN ={0, 1, 2, 3, ...}");
}

#[test]
fn p12_whole_numbers () {
    let expr = r#"<math><mi>&#8484;</mi><mo>=</mo><mo>{</mo><mo>&#8230;</mo><mo>,</mo><mo>&#x2212;</mo><mn>2</mn><mo>,</mo><mo>&#x2212;</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mo>&#8230;</mo><mo>}</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"ZZ ={..., -2, 1, 0, 1, 2, ...}");
}

#[test]
fn p13_pi () {
    let expr = r#"<math><mi>&#x3C0;</mi><mo>&#x2248;</mo><mn>3</mn><mo>,</mo><mn>14</mn></math>"#;
    test_braille_prefs("ASCIIMath", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, r"~p ~~3,14");
}

#[test]
fn p13_less_than () {
    let expr = r#"<math><mi>x</mi><mo>&#60;</mo><mn>18</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"x < 18");
}

#[test]
fn p13_greater_or_equal () {
    let expr = r#"<math><mn>2</mn><mo>&#8290;</mo><mi>x</mi><mo>&#8805;</mo><mn>6</mn></math>"#;
    test_braille("ASCIIMath-fi", expr, r"2 x >= 6");
}

#[test]
fn p13_fraction_with_invisible_plus () {
    let expr = r#"<math><mn>3</mn><mo>&#8292;</mo><mfrac><mn>5</mn><mn>6</mn></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"3#5/6");
}

#[test]
fn p13_fraction_without_invisible_plus () {
    let expr = r#"<math><mn>3</mn><mfrac><mn>5</mn><mn>6</mn></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"3#5/6");
}

#[test]
fn p13_fractional_no_paren () { 
    // The numerator doesn't require parentheses to be read correctly.
    let expr = r#"<math><mfrac><mrow><mn>4</mn><mo>&#8290;</mo><mi>x</mi></mrow><mrow><mo >(</mo><mn>1</mn><mo>−</mo><mi>x</mi><mo>)</mo></mrow></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"4 x /(1 -x)");
}

#[test]
fn p13_fractional () {
    let expr = r#"<math><mfrac><mrow><mn>5</mn><mo>+</mo><mi>x</mi></mrow><mrow><mn>5</mn><mo>&#8290;</mo><mi>x</mi></mrow></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"(5 +x) /(5 x)");
}

#[test]
fn p13_fractional_simplifying_with_paren () {
    let expr = r#"<math><mfrac><mrow><mn>5</mn><mo>+</mo><mn>7</mn></mrow><mrow><mn>2</mn><mo>&#183;</mo><mi>3</mi></mrow></mfrac><mo>=</mo><mfrac><mn>12</mn><mn>6</mn></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"(5 +7) /(2 *3) =12 /6");
}

#[test]
fn p14_long_fractional () {
    let expr = r#"<math><mfrac><mfrac><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>−</mo><mn>7</mn><mo>&#8290;</mo><mi>x</mi><mo>+</mo><mn>12</mn></mrow><mrow><mn>4</mn><mo>&#8290;</mo><mi>x</mi><mo>−</mo><mn>20</mn></mrow></mfrac><mfrac><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>−</mo><mn>8</mn><mo>&#8290;</mo><mi>x</mi><mo>+</mo><mn>15</mn></mrow><mrow><mn>4</mn><mo>&#8290;</mo><mi>x</mi><mo>−</mo><mn>16</mn></mrow></mfrac></mfrac></math>"#;
    test_braille("ASCIIMath-fi", expr, r"((x^2 -7 x +12) /(4 x -20)) /((x^2 -8 x +15) /(4 x -16))");
}

#[test]
fn p15_exponent_plus () {
    let expr = r#"<math><msup><mn>3</mn><mn>2</mn></msup><mo>+</mo><msup><mn>4</mn><mn>2</mn></msup</math>"#;
    test_braille("ASCIIMath-fi", expr, r"3^2 +4^2");
}

#[test]
fn p15_exponent_with_negative_base_in_paren () {
    let expr = r#"<math><msup><mrow><mo>(</mo><mo>−</mo><mn>2</mn><mo>)</mo></mrow><mn>2</mn></msup></math>"#;
    test_braille("ASCIIMath-fi", expr, r"(-2)^2");
}

#[test]
fn p15_exponent_with_plus_equation () {
    let expr = r#"<math><msup><mn>2</mn><mrow><mn>3</mn><mo>+</mo><mn>5</mn></mrow></msup></math>"#;
    test_braille("ASCIIMath-fi", expr, r"2^(3 +5)");
}

#[test]
fn p16_sqrt () {
    let expr = r#"<math><msqrt><mn>25</mn></msqrt></math>"#;
    test_braille("ASCIIMath-fi", expr, r"sqrt(25)");
}

#[test]
fn p16_root3 () {
    let expr = r#"<math><mroot><mn>27</mn><mn>3</mn></mroot></math>"#;
    test_braille("ASCIIMath-fi", expr, r"root3(27)");
}

#[test]
fn p16_root_equation () {
    let expr = r#"<math><mroot><mn>32</mn><mn>5</mn></mroot><mo>+</mo><mroot><mn>1</mn><mn>6</mn></mroot></math>"#;
    test_braille("ASCIIMath-fi", expr, r"root5(32) +root6(1)");
}

#[test]
fn p18_tangent_90_degrees_infinity () {
    let expr = r#"<math><mi>tan</mi><mo>&#8289;</mo><mo>⁡(</mo><mn>90</mn><mi>&#176;</mi><mo>)</mo><mo>=</mo><mi>∞</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"tan 90^@ =oo");
}

#[test]
fn p18_degrees () {
    let expr = r#"<math><mn>90</mn><mi>&#176;</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"90 ^@");
}

#[test]
fn p18_cosines () {
    let expr = r#"<math><msup><mi>cos</mi><mn>2</mn></msup><mo>&#8289;⁡</mo><mi>x</mi><mo>−</mo><mn>2</mn><mo>&#8290;</mo><mi>cos</mi><mo>&#8289;⁡</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>=</mo><mn>0</mn></math>
"#;
    test_braille("ASCIIMath-fi", expr, r"cos^2 x -2 cos x +1 =0");
}

#[test]
fn p19_vector_with_line () {
    let expr = r#"<math><mover><mi>OB</mi><mo accent='false'>¯</mo></mover></math>"#;
    test_braille("ASCIIMath-fi", expr, r"vec OB");
}

#[test]
fn p19_vector_with_arrow () {
    let expr = r#"<math><mover><mi>OB</mi><mo accent='false'>&#8594;</mo></mover></math>"#;
    test_braille("ASCIIMath-fi", expr, r"vec OB");
}

#[test]
fn p19_vector_projection () {
    let expr = r#"<math><msub><mover><mi>a</mi><mo accent='false'>¯</mo></mover><mi>b</mi></msub></math>"#;
    test_braille("ASCIIMath-fi", expr, r"vec a_b");
}

#[test]
fn p19_unit_vector () {
    let expr = r#"<math><msup><mover><mi>a</mi><mo accent='false'>¯</mo></mover><mn>0</mn></msup></math>"#;
    test_braille("ASCIIMath-fi", expr, r"vec a^0");
}

#[test]
fn p19_vector_dot_product () {
    // Notice that dot product (middle dot) in vectors has space around the dot.
    let expr = r#"<math><mover><mi>a</mi><mo accent='false'>¯</mo></mover><mo>&#183;</mo><mover><mi>b</mi><mo accent='false'>¯</mo></mover></math>"#;
    test_braille("ASCIIMath-fi", expr, r"vec a * vec b");
}

#[test]
fn p19_vector_cross_product () {
    let expr = r#"<math><mover><mi>a</mi><mo accent='false'>¯</mo></mover><mo>&#215;</mo><mover><mi>b</mi><mo accent='false'>¯</mo></mover></math>"#;
    test_braille("ASCIIMath-fi", expr, r"vec a xx vec b");
}

#[test]
fn p20_pair_of_equations () {
    let expr = r#"<math><mover><mi>a</mi><mo accent='false'>¯</mo></mover><mo>&#215;</mo><mover><mi>b</mi><mo accent='false'>¯</mo></mover></math>"#;
    test_braille("ASCIIMath-fi", expr, r"{2 x +y =0, x -y =5}");
}

#[test]
fn p22_belongs_to_a_set () {
    let expr = r#"<math><mi>x</mi><mo>&#8712;</mo><mi>A</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"x in A");
}

#[test]
fn p22_does_not_belong_to_a_set () {
    let expr = r#"<math><mn>3</mn><mo>&#8713;</mo><mi>B</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"3 !in B");
}

#[test]
fn p22_subset_right () {
    let expr = r#"<math><mi>A</mi><mo>&#8834;</mo><mi>B</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"A sub B");
}

#[test]
fn p22_subset_left () {
    let expr = r#"<math><mi>B</mi><mo>&#x2283;</mo><mi>A</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"B sup A");
}

#[test]
fn p22_not_subset () {
    let expr = r#"<math><mi>A</mi><mo>&#8836;</mo><mi>B</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"B !sup A");
}

#[test]
fn p22_union () {
    let expr = r#"<math><mi>A</mi><mo>&#8746;</mo><mi>B</mi><mo>=</mo><mo>{</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo>,</mo><mi>c</mi><mo>}</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"A uu B ={a, b, c}");
}



#[test]
fn p22_intersection_empty_set () {
    let expr = r#"<math><mi>A</mi><mo>&#8745;</mo><mi>B</mi><mo>=</mo><mi>&#8709;</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"G nn H =O");
}

#[test]
fn p22_negation () {
    let expr = r#"<math><mo>&#172;</mo><mi>p</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"not p");
}

#[test]
fn p23_logical_and () {
    let expr = r#"<math><mi>p</mi><mo>&#8743;</mo><mi>q</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"p ^^ q");
}

#[test]
fn p23_logical_or () {
    let expr = r#"<math><mi>p</mi><mo>&#8744;</mo><mi>q</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"p vv q");
}

#[test]
fn p23_logical_implication () {
    let expr = r#"<math><mi>p</mi><mo>&#8594;</mo><mi>q</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"p --> q");
}

#[test]
fn p23_function_definition () {
    let expr = r#"<math><mi>f</mi><mo>:</mo><mi>x</mi><mo>→</mo><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"f: x -> f(x)");
}

#[test]
fn augenbit1_6_11() {
    // this is a slightly cleaned up version that comes for the original example (via MathJax)
    let expr = r#" <math> <mrow>
        <msubsup>
            <mrow></mrow>
            <mrow><mn>95</mn></mrow>
            <mrow><mn>238</mn></mrow>
        </msubsup>
        <mrow><mi mathvariant="normal">U</mi></mrow>
        </mrow></math>"#;
    test_braille("ASCIIMath-fi", expr, r"\ _(95)^(238)U");
}


#[test]
fn augenbit1_7_7 () {
    let expr = r#"<math><msub><mi>log</mi><mi>a</mi></msub><mi>x</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"log_a x");
}

#[test]
fn augenbit1_7_10 () {
    let expr = r#"<math><msup><mi>cos</mi><mn>2</mn></msup><mi>&#x3B2;</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"cos^2 beta");
}

#[test]
fn augenbit1_7_12 () {
    let expr = r#"<math><mi>cot</mi><mn>45</mn><mo>&#xB0;</mo></math>"#;
    test_braille("ASCIIMath-fi", expr, r"cot 45°");
}

#[test]
fn augenbit1_8_2 () {
    let expr = r#"<math><mi mathvariant="normal">&#x25B3;</mi><mi>A</mi><mi>B</mi><mi>C</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"/_\ABC");
}

#[test]
fn augenbit1_8_4 () {
    let expr = r#"<math><mi>&#x3B1;</mi><mo>,</mo><mi>&#x3B2;</mi><mo>,</mo><mi>&#x3B3;</mi><mo>,</mo><mi>&#x3B4;</mi><mo>,</mo><mi>&#x3B5;</mi></math>"#;
    test_braille("ASCIIMath-fi", expr, r"alpha, beta, gamma, delta, epsilon");
}

#[test]
fn augenbit2_1_3 () {
    // original display code contains forced spaces not in the output -- they are cleaned up here
    let expr = r#"<math>
        <munder>
        <mo movablelimits="true">lim</mo>
        <mrow>
            <mi>x</mi>
            <mo accent="false" stretchy="false">&#x2192;</mo>
            <msub><mi>x</mi><mn>0</mn></msub>
        </mrow>
        </munder>
    </math>"#;
    test_braille("ASCIIMath-fi", expr, r"lim_(x->x_0)");
}

#[test]
fn augenbit2_1_4 () {
    // original display code contains forced spaces not in the output -- they are cleaned up here
    let expr = r#"<math>
            <msup><mi>f</mi><mo>'</mo> </msup><mo>(</mo><mi>x</mi><mo>)</mo><mo>,</mo>
            <msup><mi>f</mi><mo>''</mo></msup><mo>(</mo><mi>x</mi><mo>)</mo>
        </math>"#;
    test_braille("ASCIIMath-fi", expr, r"f'(x), f''(x)");
}

#[test]
fn augenbit2_2_2 () {
    // original display code contains forced spaces not in the output -- they are cleaned up here
    let expr = r#"<math>
        <mo minsize="2.047em" maxsize="2.047em">(</mo>
        <mfrac linethickness="0"><mi>n</mi><mi>k</mi></mfrac>
        <mo minsize="2.047em" maxsize="2.047em">)</mo>
    </math>"#;
    test_braille("ASCIIMath-fi", expr, r"((n), (k))");
}

#[test]
fn augenbit2_3_2 () {
    let expr = r#"<math>
            <mover><mi>q</mi><mo stretchy="false">&#x2192;</mo></mover><mo>=</mo>
            <mfenced>
                <mtable columnspacing="1em">
                    <mtr><mtd><mo>&#x2212;</mo><mn>5</mn></mtd></mtr>
                    <mtr><mtd><mn>0</mn><mo>,</mo><mn>5</mn></mtd></mtr>
                    <mtr><mtd><mi>k</mi><mo>+</mo><mn>4</mn></mtd></mtr>
                </mtable>
            </mfenced></math>"#;
    // set number preferences to European style
    test_braille_prefs("ASCIIMath-fi", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, 
                r"vec q = ([-5], [0,5], [k+4])");
}

#[test]
fn augenbit2_3_4 () {
    let expr = r#"<math>
        <mo>(</mo>
        <mtable columnspacing="1em" rowspacing="4pt">
        <mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd><mtd><mi>c</mi></mtd></mtr>
        <mtr><mtd><mi>d</mi></mtd><mtd><mi>e</mi></mtd><mtd><mi>f</mi></mtd></mtr>
        </mtable>
        <mo>)</mo>
    </math>"#;
    // set number preferences to European style
    test_braille_prefs("ASCIIMath-fi", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, 
                r"([a, b, c], [d, e, f])");
}
