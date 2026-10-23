use crate::common::*;

use anyhow::Result;

#[test]
fn matrix_1x1() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable><mtr><mtd>
        <mn>3</mn>
      </mtd> </mtr></mtable>
        <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("ru", "ClearSpeak",  expr, "матрица 1 на 1 с элементом 3")?;
    test("ru", "SimpleSpeak", expr, "матрица 1 на 1 с элементом 3")?;
    return Ok(());
}

#[test]
fn determinant_1x1() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>|</mo>
        <mtable><mtr><mtd>
        <mn>3</mn>
      </mtd> </mtr></mtable>
        <mo>|</mo></mrow></mrow>
    </math>
                                ";
    test("ru", "ClearSpeak",  expr, "определитель 1 на 1 с элементом 3")?;
    test("ru", "SimpleSpeak", expr, "определитель 1 на 1 с элементом 3")?;
    return Ok(());
}


#[test]
fn matrix_1x2() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>5</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("ru", "ClearSpeak",  expr, "матрица-строка 1 на 2; 3, 5")?;
    test("ru", "SimpleSpeak", expr, "матрица-строка 1 на 2; 3, 5")?;
    return Ok(());
}


#[test]
fn matrix_1x3() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mrow><mo>-</mo><mi>x</mi></mrow>
          </mtd>
          <mtd>
            <mn>5</mn>
          </mtd>
          <mtd>
            <mn>12</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("ru", "ClearSpeak", expr, "матрица-строка 1 на 3; минус икс, 5, 12")?;
    test("ru", "SimpleSpeak", expr, "матрица-строка 1 на 3; минус икс, 5, 12")?;
    return Ok(());
}

#[test]
fn matrix_2x1_not_simple() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mrow>
            <mi>x</mi><mo>+</mo><mn>1</mn>
            </mrow>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mrow>
            <mi>x</mi><mo>-</mo><mn>1</mn></mrow>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("ru", "ClearSpeak", expr, "матрица-столбец 2 на 1; строка 1; икс плюс 1; строка 2; икс минус 1")?;
    test("ru", "SimpleSpeak", expr, "матрица-столбец 2 на 1; строка 1; икс плюс 1; строка 2; икс минус 1")?;
    return Ok(());
}
#[test]
fn matrix_3x1_not_simple() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mrow>
            <mi>x</mi>
            </mrow>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mrow>
            <mi>a</mi>
            </mrow>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mfrac>
              <mi>x</mi>
              <mrow>
                <mi>x</mi><mo>+</mo><mn>1</mn>
              </mrow>
            </mfrac>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>";
    test("ru", "SimpleSpeak", expr, "матрица-столбец 3 на 1; \
            строка 1; икс; \
            строка 2; а; \
            строка 3; дробь, числитель: икс, знаменатель: икс плюс 1, конец дроби")?;
    test("ru", "ClearSpeak",  expr, "матрица-столбец 3 на 1; \
            строка 1; икс; \
            строка 2; а; \
            строка 3; дробь, числитель: икс; знаменатель: икс плюс 1")?;
            return Ok(());
}

#[test]
fn determinant_2x2() -> Result<()> {
    let expr = "<math>
      <mrow>
      <mrow><mo>|</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>7</mn>
          </mtd>
          <mtd>
            <mn>5</mn>
          </mtd>
          </mtr>
          
        </mtable>
      <mo>|</mo></mrow></mrow>
                        </math>";
    test("ru", "ClearSpeak",  expr, "определитель 2 на 2; строка 1; 2, 1; строка 2; 7, 5")?;
    test("ru", "SimpleSpeak", expr, "определитель 2 на 2; строка 1; 2, 1; строка 2; 7, 5")?;
    return Ok(());
}

#[test]
fn matrix_2x3() -> Result<()> {
    let expr = "
    <math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>[</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>4</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>6</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>]</mo></mrow></mrow>
    </math>
                                ";
    test("ru", "ClearSpeak",  expr, "матрица 2 на 3; строка 1; 3, 1, 4; строка 2; 0, 2, 6")?;
    test("ru", "SimpleSpeak", expr, "матрица 2 на 3; строка 1; 3, 1, 4; строка 2; 0, 2, 6")?;
    return Ok(());
}

#[test]
fn matrix_2x3_labeled() -> Result<()> {
    let expr = "
    <math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>[</mo>
        <mtable>
          <mlabeledtr>
          <mtd>
            <mtext>(3.1)</mtext>
          </mtd>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>4</mn>
          </mtd>
          </mlabeledtr>
          <mtr>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>6</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>]</mo></mrow></mrow>
    </math>
                                ";
    test("ru", "ClearSpeak",  expr,
        "матрица 2 на 3; строка 1 с меткой (3.1); столбец 2; 3, столбец 3; 1, столбец 4; 4; \
                                   строка 2; столбец 1; 0, столбец 2; 2, столбец 3; 6")?;
    test("ru", "SimpleSpeak", expr,
        "матрица 2 на 3; строка 1 с меткой (3.1); столбец 2; 3, столбец 3; 1, столбец 4; 4; \
                                   строка 2; столбец 1; 0, столбец 2; 2, столбец 3; 6")?;
                                   return Ok(());
}

#[test]
fn matrix_3x1() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>[</mo>
        <mtable>
        <mtr>
          <mtd>
          <mn>1</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>2</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>3</mn>
          </mtd>
        </mtr>           
        </mtable> <mo>]</mo></mrow></mrow>
    </math>
                                ";
    test("ru", "ClearSpeak",  expr, "матрица-столбец 3 на 1; 1; 2; 3")?;
    test("ru", "SimpleSpeak", expr, "матрица-столбец 3 на 1; 1; 2; 3")?;
    return Ok(());
}

#[test]
fn matrix_4x1() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>6</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>2</mn>
          </mtd>
          </mtr>            
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("ru", "ClearSpeak",  expr, "матрица-столбец 4 на 1; строка 1; 3; строка 2; 6; строка 3; 1; строка 4; 2")?;
    test("ru", "SimpleSpeak", expr, "матрица-столбец 4 на 1; строка 1; 3; строка 2; 6; строка 3; 1; строка 4; 2")?;
    return Ok(());
}

#[test]
fn matrix_4x1_labeled() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>6</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
          <mlabeledtr>
          <mtd>
            <mtext>(3.1)</mtext>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          </mlabeledtr>            
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("ru", "ClearSpeak",  expr,
        "матрица-столбец 4 на 1; строка 1; 3; строка 2; 6; строка 3; 1; строка 4 с меткой (3.1); 2")?;
    test("ru", "SimpleSpeak", expr,
        "матрица-столбец 4 на 1; строка 1; 3; строка 2; 6; строка 3; 1; строка 4 с меткой (3.1); 2")?;
        return Ok(());
}

#[test]
fn matrix_1x4() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>6</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("ru", "ClearSpeak",  expr, "матрица-строка 1 на 4; столбец 1; 3, столбец 2; 6, столбец 3; 1, столбец 4; 2")?;
    test("ru", "SimpleSpeak", expr, "матрица-строка 1 на 4; столбец 1; 3, столбец 2; 6, столбец 3; 1, столбец 4; 2")?;
    return Ok(());
}

#[test]
fn matrix_4x4() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>(</mo>
        <mtable>
          <mtr>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>4</mn>
          </mtd>
          <mtd>
            <mn>3</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>9</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>3</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>1</mn>
          </mtd>
          </mtr>
          <mtr>
          <mtd>
            <mn>6</mn>
          </mtd>
          <mtd>
            <mn>2</mn>
          </mtd>
          <mtd>
            <mn>9</mn>
          </mtd>
          <mtd>
            <mn>0</mn>
          </mtd>
          </mtr>
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
                                ";
    test("ru", "ClearSpeak",  expr, "матрица 4 на 4; \
          строка 1; столбец 1; 0, столбец 2; 3, столбец 3; 4, столбец 4; 3; \
          строка 2; столбец 1; 2, столбец 2; 1, столбец 3; 0, столбец 4; 9; \
          строка 3; столбец 1; 3, столбец 2; 0, столбец 3; 2, столбец 4; 1; \
          строка 4; столбец 1; 6, столбец 2; 2, столбец 3; 9, столбец 4; 0")?;
    test("ru", "SimpleSpeak", expr, "матрица 4 на 4; \
          строка 1; столбец 1; 0, столбец 2; 3, столбец 3; 4, столбец 4; 3; \
          строка 2; столбец 1; 2, столбец 2; 1, столбец 3; 0, столбец 4; 9; \
          строка 3; столбец 1; 3, столбец 2; 0, столбец 3; 2, столбец 4; 1; \
          строка 4; столбец 1; 6, столбец 2; 2, столбец 3; 9, столбец 4; 0")?;
          return Ok(());
}

#[test]
fn matrix_4x2() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
    <mrow>
      <mrow><mo>(</mo>
        <mtable>
        <mtr>
          <mtd>
          <mn>1</mn>
          </mtd>
          <mtd>
          <mn>3</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>4</mn>
          </mtd>
          <mtd>
          <mn>2</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>2</mn>
          </mtd>
          <mtd>
          <mn>1</mn>
          </mtd>
        </mtr>
        <mtr>
          <mtd>
          <mn>0</mn>
          </mtd>
          <mtd>
          <mn>5</mn>
          </mtd>
        </mtr>
        
        </mtable>
      <mo>)</mo></mrow></mrow>
    </math>
      ";
    test("ru", "ClearSpeak",  expr, "матрица 4 на 2; \
              строка 1; столбец 1; 1, столбец 2; 3; \
              строка 2; столбец 1; 4, столбец 2; 2; \
              строка 3; столбец 1; 2, столбец 2; 1; \
              строка 4; столбец 1; 0, столбец 2; 5\
    ")?;
    test("ru", "SimpleSpeak", expr, "матрица 4 на 2; \
              строка 1; столбец 1; 1, столбец 2; 3; \
              строка 2; столбец 1; 4, столбец 2; 2; \
              строка 3; столбец 1; 2, столбец 2; 1; \
              строка 4; столбец 1; 0, столбец 2; 5\
    ")?;
    return Ok(());
}

// поместим тест для абсолютной величины сюда, так как он связан с определителем и слишком мал для отдельного файла
#[test]
fn simple_absolute_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("ru", "SimpleSpeak", expr, "модуль икс")?;
  test("ru", "ClearSpeak",  expr, "модуль икс")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "модуль икс")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "модуль икс, конец модуля")?;
             return Ok(());
}
  
#[test]
fn absolute_value_plus_1() -> Result<()> {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("ru", "ClearSpeak", expr, "модуль икс плюс 1")?;
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "модуль икс плюс 1, конец модуля")?;
             return Ok(());
}

#[test]
fn simple_cardinality_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("ru", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "мощность множества заглавной эс")?;
             return Ok(());
}
  
// Тестирование предпочтений
#[test]
fn simple_matrix_speak_col_num() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("ru", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "матрица 2 на 2; строка 1; столбец 1; 2, столбец 2; 1; строка 2; столбец 1; 7, столбец 2; 5")?;
        return Ok(());
}

#[test]
fn col_matrix_3x1_speak_col_num() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "матрица-столбец 3 на 1; строка 1; 1; строка 2; 2; строка 3; 3")?;
        return Ok(());
}

#[test]
fn row_matrix_1x2_speak_col_num() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "матрица-строка 1 на 2; столбец 1; 1, столбец 2; 2")?;
        return Ok(());
}

#[test]
fn matrix_2x2_speak_col_num() -> Result<()> {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "матрица 2 на 2; строка 1; столбец 1; бэ с индексом 1 1; столбец 2; бэ с индексом 1 2; \
                                                строка 2; столбец 1; бэ с индексом 2 1; столбец 2; бэ с индексом 2 2")?;
                                                return Ok(());
}


#[test]
fn simple_matrix_silent_col_num() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("ru", "ClearSpeak_Matrix", "SilentColNum",
        expr, "матрица 2 на 2; строка 1; 2, 1; строка 2; 7, 5")?;
        return Ok(());
}

#[test]
fn col_matrix_3x1_silent_col_num() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "SilentColNum",
        expr, "матрица-столбец 3 на 1; 1; 2; 3")?;
        return Ok(());
}

#[test]
fn row_matrix_1x2_silent_col_num() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "SilentColNum",
        expr, "матрица-строка 1 на 2; 1, 2")?;
        return Ok(());
}

#[test]
fn matrix_2x2_silent_col_num() -> Result<()> {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "SilentColNum",
        expr, "матрица 2 на 2; строка 1; бэ с индексом 1 1; бэ с индексом 1 2; \
                                                строка 2; бэ с индексом 2 1; бэ с индексом 2 2")?;
                                                return Ok(());
}


#[test]
fn simple_matrix_end_matrix() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndMatrix",
        expr, "матрица 2 на 2; строка 1; 2, 1; строка 2; 7, 5; конец матрицы")?;
        return Ok(());
}

#[test]
fn col_matrix_3x1_end_matrix() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndMatrix",
        expr, "матрица-столбец 3 на 1; 1; 2; 3; конец матрицы")?;
        return Ok(());
}

#[test]
fn row_matrix_1x2_end_matrix() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndMatrix",
        expr, "матрица-строка 1 на 2; 1, 2; конец матрицы")?;
        return Ok(());
}

#[test]
fn matrix_2x2_end_matrix() -> Result<()> {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndMatrix",
        expr, "матрица 2 на 2; строка 1; столбец 1; бэ с индексом 1 1; столбец 2; бэ с индексом 1 2; \
                                                строка 2; столбец 1; бэ с индексом 2 1; столбец 2; бэ с индексом 2 2; конец матрицы")?;
                                                return Ok(());
}


#[test]
fn simple_matrix_vector() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("ru", "ClearSpeak_Matrix", "Vector",
        expr, "матрица 2 на 2; строка 1; 2, 1; строка 2; 7, 5")?;
        return Ok(());
}

#[test]
fn col_matrix_3x1_vector() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "Vector",
        expr, "вектор-столбец 3 на 1; 1; 2; 3")?;
        return Ok(());
}

#[test]
fn row_matrix_1x2_vector() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "Vector",
        expr, "вектор-строка 1 на 2; 1, 2")?;
        return Ok(());
}

#[test]
fn matrix_2x2_vector() -> Result<()> {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "Vector",
        expr, "матрица 2 на 2; строка 1; столбец 1; бэ с индексом 1 1; столбец 2; бэ с индексом 1 2; \
                                                строка 2; столбец 1; бэ с индексом 2 1; столбец 2; бэ с индексом 2 2")?;
                                                return Ok(());
}


#[test]
fn simple_matrix_end_vector() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd> <mn>2</mn> </mtd>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>7</mn> </mtd>
        <mtd><mn>5</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
  test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndVector",
        expr, "матрица 2 на 2; строка 1; 2, 1; строка 2; 7, 5; конец матрицы")?;
        return Ok(());
}

#[test]
fn col_matrix_3x1_end_vector() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>(</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>2</mn> </mtd>
      </mtr>
      <mtr>
        <mtd><mn>3</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>)</mo></mrow></mrow>
  </math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndVector",
        expr, "вектор-столбец 3 на 1; 1; 2; 3; конец вектора")?;
        return Ok(());
}

#[test]
fn row_matrix_1x2_end_vector() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
    <mrow>
      <mtable>
      <mtr>
        <mtd><mn>1</mn> </mtd> <mtd><mn>2</mn> </mtd>
      </mtr>
      </mtable></mrow>
    <mo>]</mo></mrow></mrow>
  </math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndVector",
        expr, "вектор-строка 1 на 2; 1, 2; конец вектора")?;
        return Ok(());
}

#[test]
fn matrix_2x2_end_vector() -> Result<()> {
let expr = "<math><mrow><mrow><mo>(</mo><mrow>
    <mtable>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    <mtr>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>1</mn></mrow></msub></mrow></mtd>
        <mtd><mrow><msub><mi>b</mi><mrow><mn>2</mn><mn>2</mn></mrow></msub></mrow></mtd>
    </mtr>
    </mtable>
    </mrow><mo>)</mo></mrow></mrow></math>";
test_ClearSpeak("ru", "ClearSpeak_Matrix", "EndVector",
        expr, "матрица 2 на 2; строка 1; столбец 1; бэ с индексом 1 1; столбец 2; бэ с индексом 1 2; \
                                                 строка 2; столбец 1; бэ с индексом 2 1; столбец 2; бэ с индексом 2 2; конец матрицы")?;
                                                 return Ok(());
}



#[test]
fn matrix_binomial() -> Result<()> {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("ru", "ClearSpeak_Matrix", "Combinatorics", expr, "число сочетаний из 3 по 2")?;
  return Ok(());
}

#[test]
fn matrix_times() -> Result<()> {
  let expr = "<math>
    <mfenced><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable></mfenced>
    <mfenced><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable></mfenced>
  </math>";
  test("ru", "SimpleSpeak", expr,
    "матрица 2 на 2; строка 1; 1, 2; строка 2; 3, 4; умножить на матрица 2 на 2; строка 1; а, бэ; строка 2; цэ, дэ")?;
    return Ok(());
}

#[test]
fn unknown_mtable_property() -> Result<()> {
  let expr = "<math display='block'>
      <mtable intent=':system-of-equations:prefix($e1,$e1x)'>
        <mtr arg='e1'>
        <mtd columnalign='right'>
          <mi>a</mi>
        </mtd>
        <mtd columnalign='center'>
          <mo>=</mo>
        </mtd>
        <mtd intent='_($lhs)' columnalign='left'>
          <mrow arg='lhs'>
          <mi>b</mi>
          <mo>+</mo>
          <mi>c</mi>
          <mo>&#x2212;</mo>
          <mi>d</mi>
        </mrow>
        </mtd>
        </mtr>
        <mtr arg='e1x'>
        <mtd intent='_' columnalign='right'></mtd>
        <mtd intent='_' columnalign='center'></mtd>
        <mtd arg='rhs' columnalign='left'>
          <mo form='infix'>+</mo>
          <mi>e</mi>
          <mo>&#x2212;</mo>
          <mi>f</mi>
        </mtd>
        </mtr>
      </mtable>
    </math>";
    test("ru", "ClearSpeak",  expr,
         "2 строки; строка 1; а равно бэ плюс цэ минус дэ; строка 2; плюс е минус эф")?;
         return Ok(());
}

#[test]
fn gathered_continued_equation() -> Result<()> {
  // A gathered display is one derivation split across lines; do not duplicate
  // the equality at the wrap or announce its final sentence punctuation.
  let expr = "<math><mtable class='gathered'>
    <mtr><mtd><mrow><mi>a</mi><mo>=</mo></mrow></mtd></mtr>
    <mtr><mtd><mrow><mo>=</mo><mi>b</mi><mo>.</mo></mrow></mtd></mtr>
  </mtable></math>";
  test("ru", "ClearSpeak", expr, "2 строки, строка 1; а; строка 2; равно бэ")?;
  test("ru", "SimpleSpeak", expr, "2 строки; строка 1; а; строка 2; равно бэ")?;
  test("ru", "LiteralSpeak", expr,
       "таблица с 2 строками и 1 столбцом; строка 1; столбец 1; а равно; строка 2; столбец 1; равно бэ точка")?;
  Ok(())
}


#[test]
fn zero_matrix() -> Result<()> {
  let expr = "<math>
      <mo>[</mo>
      <mtable>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
      </mtable>
      <mo>]</mo>
  </math>";
  test("ru", "SimpleSpeak", expr,
    "нулевая матрица 2 на 2")?;
    return Ok(());
}

#[test]
fn identity_matrix() -> Result<()> {
  let expr = "<math>
      <mo>(</mo>
      <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr>
      </mtable>
      <mo>)</mo>
  </math>";
  test("ru", "SimpleSpeak", expr,
    "единичная матрица 3 на 3")?;
    return Ok(());
}

#[test]
fn diagonal_matrix() -> Result<()> {
  let expr = "<math>
      <mo>(</mo>
      <mtable>
        <mtr><mtd><mn>2</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd><mtd><msup><mi>x</mi><mn>2</mn></msup></mtd></mtr>
      </mtable>
      <mo>)</mo>
  </math>";
  test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "диагональная матрица 3 на 3; столбец 1; 2; столбец 2; 1; столбец 3; икс в квадрате")?;
  // test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")],
  //     expr, "диагональная матрица 3 на 3; строка 1, столбец 1, 2; строка 2, столбец 2, 1; строка 3, столбец 3, икс в квадрате")?;
  return Ok(());
}
