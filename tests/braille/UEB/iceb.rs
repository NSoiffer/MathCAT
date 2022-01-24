// UEB tests for the basic mathml tags
// These come from (refer to) https://iceb.org/guidelines_for_technical_material_2014.pdf
// For example, "fraction_6_1_1" is a fraction example from section 6.1, and is the first example there.
use crate::common::*;

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
    let expr = "<math><mn>2</mn><mo>&#xA0;</mo><mfrac bevelled='true'><mn>1</mn><mn>2</mn></mfrac></math>";
    test_braille("UEB", expr, "⠼⠃⠼⠁⠌⠃");
}

#[test]
fn fraction_6_2_2() {
    let expr = "<math><mn>1750</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-Unit'>cm</mi><mo>&#xA0;</mo><mo>=</mo>
                <mo>&#xA0;</mo><mn>1</mn><mfrac bevelled='true'><mn>3</mn><mn>4</mn></mfrac>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-Unit'm</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠛⠑⠚⠀⠉⠍⠀⠐⠶⠀⠼⠁⠼⠉⠌⠙⠀⠰⠍");
}

#[test]
fn fraction_6_2_2_unicode_frac() {
    let expr = "<math><mn>1750</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-Unit'>cm</mi><mo>&#xA0;</mo><mo>=</mo>
                <mo>&#xA0;</mo><mn>1</mn><mn>&#xBE;</mn>
                <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-Unit'm</mi></math>";
    test_braille("UEB", expr, "⠼⠁⠛⠑⠚⠀⠉⠍⠀⠐⠶⠀⠼⠁⠼⠉⠌⠙⠀⠰⠍");
}

#[test]
fn fraction_6_3_1() {
    let expr = "<math><mfrac><mn>3</mn><mn>8</mn></mfrac></math>";
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
fn msup_7_6_2() {
    let expr = "<math><msup><mi mathvariant='normal'>H</mi><mo>+</mo></msup></math>";
    test_braille("UEB", expr, "⠠⠓⠰⠔⠐⠖");
}

#[test]
fn msubsup_7_7_1() {
    let expr = "<math><msubsup><mi>x</mi><mn>1</mn><mn>2</mn></msubsup><mo>=</mo><msubsup><mi>y</mi><mn>2</mn><mn>3</mn></msubsup></math>";
    test_braille("UEB", expr, "⠠⠰⠰⠰⠭⠢⠼⠁⠔⠼⠃⠀⠐⠶⠀⠽⠢⠼⠃⠔⠼⠉⠰⠄");
}

#[test]
fn msubsup_7_7_2() {
    let expr = "<math><msub><msup><mi>x</mi><mn>2</mn></msup><mi>k</mi></msub></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠼⠃⠢⠅");
}

#[test]
fn msubsup_7_7_2() {
    let expr = "<math><msub><msup><mi>x</mi><mn>2</mn></msup><mi>k</mi></msub></math>";
    test_braille("UEB", expr, "⠰⠰⠭⠔⠼⠃⠢⠅");
}

#[test]
fn pre_msubsup_7_8_1() {
    let expr = "<math><mmultiscripts><mi>U</mi><mprescripts/><mn>92</mn><mn>238</mn></mmultiscripts></math>";
    test_braille("UEB", expr, "⠰⠰⠢⠼⠊⠃⠔⠼⠃⠉⠓⠠⠥");
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
    let expr = "<math><munderover><mo>&#x2211;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover>
            <msubsup><mi>x</mi><mi>i</mi><mn>2</mn></msubsup></math>";
    test_braille("UEB", expr, "⠰⠰⠠⠨⠎⠨⠢⠣⠭⠐⠶⠼⠁⠜⠨⠔⠝⠭⠢⠊⠔⠼⠃");
}

#[test]
fn sum_7_9_2() {
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
    let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
         <mrow>
      <mfrac>
       <mrow>
        <mo>&#x2212;</mo><mi>b</mi><mo>&#x00B1;</mo><msqrt>
         <mrow>
          <msup>
           <mi>b</mi>
           <mn>2</mn>
          </msup>
          <mo>&#x2212;</mo><mn>4</mn><mi>a</mi><mi>c</mi></mrow>
        </msqrt>
        </mrow>
       <mrow>
        <mn>2</mn><mi>a</mi></mrow>
      </mfrac>
      </mrow>
      </math>
    ";
    test_braille("UEB", expr, "⠰⠰⠰⠭⠀⠐⠶⠀⠷⠐⠤⠃⠸⠖⠩⠃⠔⠼⠃⠐⠤⠼⠙⠰⠁⠉⠬⠨⠌⠼⠃⠰⠁⠾⠰⠄");
}

#[test]
fn sqrt_8_2_1() {
    let expr = "<math><mroot><mn>8</mn><mn>3</mn></mroot><mo>=</mo><mn>2</mn></math>";
    test_braille("UEB", expr, "⠰⠰⠩⠔⠼⠉⠼⠓⠬⠀⠐⠶⠀⠼⠃");
}

#[test]
fn sqrt_8_2_3() {
    let expr = "<math><mroot><mrow><mi>x</mi><mi>y</mi></mrow><mrow><mi>m</mi><mi>n</mi></mrow></mroot></math>";
    test_braille("UEB", expr, "⠰⠰⠩⠔⠣⠍⠝⠜⠭⠽⠬");
}
