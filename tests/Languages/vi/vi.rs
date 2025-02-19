use crate::common::*;


#[test]
fn log_sup_sub() {
    let expr = "<math><mrow><msubsup><mi mathvariant='normal' ame-texclass='op'>log</mi><mn>10</mn><mn>20</mn></msubsup><mo>&#x2061;</mo><mi>x</mi></mrow></math>";
    test("vi", "ClearSpeak", expr, "lóc mũ 20 cơ số 10; của x");
}

#[test]
fn number_1() {
    let expr = "<math><mn>3.000,12</mn></math>";
    test("vi", "ClearSpeak", expr, "3.000,12");
}

#[test]
fn number_2() {
    let expr = "<math><mn>3,14</mn></math>";
    test("vi", "ClearSpeak", expr, "3,14");
}

#[test]
fn number_1a() {
    let expr = "<math><mn>3,000.12</mn></math>";
    test_prefs("vi", "ClearSpeak", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "3,000.12");
}

#[test]
fn number_2a() {
    let expr = "<math><mn>3.14</mn></math>";
    test_prefs("vi", "ClearSpeak", vec![("DecimalSeparators", ","), ("BlockSeparators", ". ")], expr, "3.14");
}

#[test]
fn roman_numeral() {
  let expr = "<math><mi>IX</mi><mo>+</mo><mi>VIII</mi><mo>=</mo><mi>XVII</mi></math>";
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "9 cộng 8, bằng 17");
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "9 la mã cộng 8 la mã, bằng 17 la mã");
}

#[test]
#[ignore]
fn vi_units_1() {
    let expr = "<math><mrow><mn>1</mn><mi>t</mi><mi>ấ</mi><mi>n</mi><mn>10</mn><mi>t</mi><mi>ạ</mi><mn>100</mn><mi>y</mi><mi>ế</mi><mi>n</mi><mi>v</mi><mi>à</mi><mn>4</mn><mi>l</mi><mi>í</mi><mi>t</mi></mrow></math>";
    test("vi", "ClearSpeak", expr, "1 tấn 10 tạ 100 yến và 4 lít");
}

#[test]
fn salt() {
  let expr = "<math><mi>Na</mi><mi>Cl</mi></math>";
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "n a, c l");
}

#[test]
fn water() {
  let expr = "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>";
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "h, 2 o");
}

#[test]
fn carbon() {
  let expr = "<math><mi>C</mi></math>";     // not enough to trigger recognition
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "c");
}

#[test]
fn sulfate() {
  let expr = "<math><mrow><msup>
          <mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
          <mrow><mn>2</mn><mo>&#x2212;</mo></mrow>
      </msup></mrow></math>";
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "mở ngoặc vuông, s, o, 4, đóng ngoặc vuông 2 trừ");
}

#[test]
fn aluminum_sulfate() {
  let expr = "<math><mrow><msub><mi>Al</mi><mn>2</mn></msub>
          <msub><mrow><mo>(</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>)</mo></mrow><mn>3</mn></msub></mrow></math>";
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "a l, 2; mở ngoặc đơn, s, o, 4, đóng ngoặc đơn 3");
  // "tất cả #X lần" phrase is just applied for chemistry case, not for math. "#X" is the sub 3.
}

#[test]
fn ethanol_bonds() {
  let expr = "<math>
          <mrow>
              <mi>C</mi>
              <msub>  <mi>H</mi> <mn>3</mn> </msub>
              <mo>&#x2212;</mo>
              <mi>C</mi>
              <msub>  <mi>H</mi> <mn>2</mn> </msub>
              <mo>&#x2212;</mo>
              <mi>O</mi>
              <mi>H</mi>
          </mrow>
      </math>";
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "c, h, 3 nối đơn c, h, 2 nối đơn o, h");
}

#[test]
fn dichlorine_hexoxide() {
  let expr = "<math><mrow>
      <msup>
        <mrow><mo>[</mo><mi>Cl</mi><msub><mi>O</mi><mn>2</mn></msub><mo>]</mo></mrow>
        <mo>+</mo>
      </msup>
      <msup>
        <mrow><mo>[</mo><mi>Cl</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
        <mo>-</mo>
      </msup>
    </mrow></math>";
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
             "mở ngoặc vuông, c l, o, 2, đóng ngoặc vuông cộng; mở ngoặc vuông, c l, o, 4, đóng ngoặc vuông trừ");
}

#[test]
fn ethylene_with_bond() {
  let expr = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>=</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "h, 2 c, nối đôi c, h, 2");
}

#[test]
fn ferric_chloride_aq() {
  let expr = "<math><mrow>
        <mi>Fe</mi>
        <msub><mi>Cl</mi><mn>3</mn></msub>
        <mrow><mo>(</mo><mrow><mi>aq</mi></mrow><mo>)</mo></mrow>
    </mrow></math>";
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "f e, c l, 3 thể lỏng");
  }

#[test]
fn ethylene_with_colon_bond() {
  let expr = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>::</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "h, 2 c, nối đôi c, h, 2");
}

#[test]
fn mhchem_roman_in_superscript() {
  let expr = " <math>
      <mrow>
        <mmultiscripts>
          <mi>Fe</mi>
          <none></none>
          <mi>II</mi>
        </mmultiscripts>
        <mo>&#x2063;</mo>
        <mmultiscripts>
          <mi>Fe</mi>
          <none></none>
          <mi data-number='3'>III</mi>
        </mmultiscripts>
        <mo>&#x2063;</mo>
        <mmultiscripts>
          <mi mathvariant='normal' >O</mi>
          <mn>4</mn>
          <none></none>
        </mmultiscripts>
      </mrow>
    </math>";
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "f hoa e, hóa trị 2 la mã; f hoa e, hóa trị 3 la mã; o hoa, 4");
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "f e, hóa trị 2; f e, hóa trị 3; o, 4");
  // when Roman numbers written on superscript at the middle, it should be added prefix text "hóa trị" then + the number
}

#[test]
fn overparen() {
  let expr = r#"<math><mover accent="false"><mrow><mi>A</mi><mi>B</mi></mrow><mo accent="true">&#x23DC;</mo></mover></math>"#;
  test_prefs("vi", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "cung a hoa b hoa");
}

