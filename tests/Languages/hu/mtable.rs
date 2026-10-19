use crate::common::*;
use anyhow::Result;

// AI generated
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
    test("hu", "ClearSpeak",  expr, "a(z) egy-szer-egyes mátrix eleme: 3")?;
    test("hu", "SimpleSpeak", expr, "a(z) egy-szer-egyes mátrix eleme: 3")?;
    return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak",  expr, "a(z) egy-szer-egyes determináns eleme: 3")?;
    test("hu", "SimpleSpeak", expr, "a(z) egy-szer-egyes determináns eleme: 3")?;
    return Ok(());

}


// AI generated
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
    test("hu", "ClearSpeak",  expr, "az 1-szer 2 -es sor mátrix; 3, 5")?;
    test("hu", "SimpleSpeak", expr, "az 1-szer 2 -es sor mátrix; 3, 5")?;
    return Ok(());

}


// AI generated
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
    test("hu", "ClearSpeak", expr, "az 1-szer 3 -es sor mátrix; negatív x, 5, 12")?;
    test("hu", "SimpleSpeak", expr, "az 1-szer 3 -es sor mátrix; negatív x, 5, 12")?;
    return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak", expr, "a(z) 2 x 1-es oszlop mátrix; 1 . sor; x plusz 1; 2 . sor; x mínusz 1")?;
    test("hu", "SimpleSpeak", expr, "a(z) 2 x 1-es oszlop mátrix; 1 . sor; x plusz 1; 2 . sor; x mínusz 1")?;
    return Ok(());

}
// AI generated
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
    test("hu", "SimpleSpeak", expr, "a(z) 3 x 1-es oszlop mátrix; 1 . sor; x; 2 . sor; a; 3 . sor; tört, x per, x plusz 1, tört vége")?;
    test("hu", "ClearSpeak",  expr, "a(z) 3 x 1-es oszlop mátrix; 1 . sor; x; 2 . sor; a; 3 . sor; a tört, melynek számlálója x; nevezője pedig x plusz 1")?;
            return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak",  expr, "a(z) 2 x 2 determináns; 1 . sor; 2, 1; 2 . sor; 7, 5")?;
    test("hu", "SimpleSpeak", expr, "a(z) 2 x 2 determináns; 1 . sor; 2, 1; 2 . sor; 7, 5")?;
    return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak",  expr, "a(z) 2 x 3 mátrix; 1 . sor; 3, 1, 4; 2 . sor; 0, 2, 6")?;
    test("hu", "SimpleSpeak", expr, "a(z) 2 x 3 mátrix; 1 . sor; 3, 1, 4; 2 . sor; 0, 2, 6")?;
    return Ok(());

}

#[test]
fn augmented_matrix_2x3() -> Result<()> {
    let expr = "
    <math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow>
      <mrow><mo>[</mo>
        <mtable columnlines='none solid'>
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
    test("hu", "ClearSpeak",  expr, "a(z) 2 x 3 mátrix; 1 . sor; 3, 1, elválasztó, 4; 2 . sor; 0, 2, elválasztó, 6")?;
    test("hu", "SimpleSpeak", expr, "a(z) 2 x 3 mátrix; 1 . sor; 3, 1, elválasztó, 4; 2 . sor; 0, 2, elválasztó, 6")?;
    Ok(())
}

#[test]
fn dashed_augmented_matrix_separator() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow><mo>[</mo>
        <mtable columnlines='dashed'>
          <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr>
        </mtable>
      <mo>]</mo></mrow>
    </math>";
    test("hu", "ClearSpeak", expr, "az 1-szer 3 -es sor mátrix; 1, elválasztó, 2, elválasztó, 3")?;
    test("hu", "SimpleSpeak", expr, "az 1-szer 3 -es sor mátrix; 1, elválasztó, 2, elválasztó, 3")?;
    Ok(())
}

/// A horizontal line is announced once, after the row it separates from the next row.
#[test]
fn matrix_row_separator() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow><mo>[</mo>
        <mtable rowlines='solid'>
          <mtr>
            <mtd><mn>1</mn></mtd>
            <mtd><mn>2</mn></mtd>
          </mtr>
          <mtr>
            <mtd><mn>3</mn></mtd>
            <mtd><mn>4</mn></mtd>
          </mtr>
        </mtable>
      <mo>]</mo></mrow>
    </math>";
    test("hu", "ClearSpeak", expr, "a(z) 2 x 2 mátrix; 1 . sor; 1, 2, sorelválasztó; 2 . sor; 3, 4")?;
    test("hu", "SimpleSpeak", expr, "a(z) 2 x 2 mátrix; 1 . sor; 1, 2, sorelválasztó; 2 . sor; 3, 4")?;
    Ok(())
}

/// Horizontal and vertical lines use distinct announcements at their respective boundaries.
#[test]
fn matrix_row_and_column_separators() -> Result<()> {
    let expr = "
    <math xmlns='http://www.w3.org/1998/Math/MathML'>
      <mrow><mo>[</mo>
        <mtable rowlines='dashed' columnlines='solid'>
          <mtr>
            <mtd><mn>1</mn></mtd>
            <mtd><mn>2</mn></mtd>
          </mtr>
          <mtr>
            <mtd><mn>3</mn></mtd>
            <mtd><mn>4</mn></mtd>
          </mtr>
        </mtable>
      <mo>]</mo></mrow>
    </math>";
    test("hu", "ClearSpeak", expr, "a(z) 2 x 2 mátrix; 1 . sor; 1, elválasztó, 2, sorelválasztó; 2 . sor; 3, elválasztó, 4")?;
    test("hu", "SimpleSpeak", expr, "a(z) 2 x 2 mátrix; 1 . sor; 1, elválasztó, 2, sorelválasztó; 2 . sor; 3, elválasztó, 4")?;
    Ok(())
}

// AI generated
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
    test("hu", "ClearSpeak",  expr,
        "a(z) 2 x 3 mátrix; 1 . sor címkéje: (3.1); oszlop 1; 3, oszlop 2; 1, oszlop 3; 4; 2 . sor; oszlop 1; 0, oszlop 2; 2, oszlop 3; 6")?;
    test("hu", "SimpleSpeak", expr,
        "a(z) 2 x 3 mátrix; 1 . sor címkéje: (3.1); oszlop 1; 3, oszlop 2; 1, oszlop 3; 4; 2 . sor; oszlop 1; 0, oszlop 2; 2, oszlop 3; 6")?;
                                   return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak",  expr, "a(z) 3 x 1-es oszlop mátrix; 1; 2; 3")?;
    test("hu", "SimpleSpeak", expr, "a(z) 3 x 1-es oszlop mátrix; 1; 2; 3")?;
    return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak",  expr, "a(z) 4 x 1-es oszlop mátrix; 1 . sor; 3; 2 . sor; 6; 3 . sor; 1; 4 . sor; 2")?;
    test("hu", "SimpleSpeak", expr, "a(z) 4 x 1-es oszlop mátrix; 1 . sor; 3; 2 . sor; 6; 3 . sor; 1; 4 . sor; 2")?;
    return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak",  expr,
        "a(z) 4 x 1-es oszlop mátrix; 1 . sor; 3; 2 . sor; 6; 3 . sor; 1; 4 . sor címkéje: (3.1); 2")?;
    test("hu", "SimpleSpeak", expr,
        "a(z) 4 x 1-es oszlop mátrix; 1 . sor; 3; 2 . sor; 6; 3 . sor; 1; 4 . sor címkéje: (3.1); 2")?;
        return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak",  expr, "az 1-szer 4 -es sor mátrix; oszlop 1; 3, oszlop 2; 6, oszlop 3; 1, oszlop 4; 2")?;
    test("hu", "SimpleSpeak", expr, "az 1-szer 4 -es sor mátrix; oszlop 1; 3, oszlop 2; 6, oszlop 3; 1, oszlop 4; 2")?;
    return Ok(());

}

// AI generated
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
    test("hu", "ClearSpeak",  expr, "a(z) 4 x 4 mátrix; 1 . sor; oszlop 1; 0, oszlop 2; 3, oszlop 3; 4, oszlop 4; 3; 2 . sor; oszlop 1; 2, oszlop 2; 1, oszlop 3; 0, oszlop 4; 9; 3 . sor; oszlop 1; 3, oszlop 2; 0, oszlop 3; 2, oszlop 4; 1; 4 . sor; oszlop 1; 6, oszlop 2; 2, oszlop 3; 9, oszlop 4; 0")?;
    test("hu", "SimpleSpeak", expr, "a(z) 4 x 4 mátrix; 1 . sor; oszlop 1; 0, oszlop 2; 3, oszlop 3; 4, oszlop 4; 3; 2 . sor; oszlop 1; 2, oszlop 2; 1, oszlop 3; 0, oszlop 4; 9; 3 . sor; oszlop 1; 3, oszlop 2; 0, oszlop 3; 2, oszlop 4; 1; 4 . sor; oszlop 1; 6, oszlop 2; 2, oszlop 3; 9, oszlop 4; 0")?;
    return Ok(());
}

// AI generated
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
    test("hu", "ClearSpeak",  expr, "a(z) 4 x 2 mátrix; 1 . sor; oszlop 1; 1, oszlop 2; 3; 2 . sor; oszlop 1; 4, oszlop 2; 2; 3 . sor; oszlop 1; 2, oszlop 2; 1; 4 . sor; oszlop 1; 0, oszlop 2; 5")?;
    test("hu", "SimpleSpeak", expr, "a(z) 4 x 2 mátrix; 1 . sor; oszlop 1; 1, oszlop 2; 3; 2 . sor; oszlop 1; 4, oszlop 2; 2; 3 . sor; oszlop 1; 2, oszlop 2; 1; 4 . sor; oszlop 1; 0, oszlop 2; 5")?;
    return Ok(());
}

// put absolute value test here since it is related to determinate and is small for its own file
// AI generated
#[test]
fn simple_absolute_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("hu", "SimpleSpeak", expr, "az abszolút érték x")?;
  test("hu", "ClearSpeak",  expr, "x abszolút értéke")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "x abszolút értéke")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "x abszolút értéke, vége az abszolút értéknek")?;
  return Ok(());
}
  
// AI generated
#[test]
fn absolute_value_plus_1() -> Result<()> {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("hu", "ClearSpeak", expr, "x plusz 1 abszolút értéke")?;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "x plusz 1 abszolút értéke, vége az abszolút értéknek")?;
  return Ok(());
}

// AI generated
#[test]
fn simple_cardinality_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "nagy s számossága")?;
    return Ok(());
}
  
// Test preferences
// AI generated
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
  test_ClearSpeak("hu", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "a(z) 2 x 2 mátrix; 1 . sor; oszlop 1; 2, oszlop 2; 1; 2 . sor; oszlop 1; 7, oszlop 2; 5")?;
    return Ok(());
}

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "a(z) 3 x 1-es oszlop mátrix; 1 . sor; 1; 2 . sor; 2; 3 . sor; 3")?;
    return Ok(());
}

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "az 1-szer 2 -es sor mátrix; oszlop 1; 1, oszlop 2; 2")?;
    return Ok(());
}

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "a(z) 2 x 2 mátrix; 1 . sor; oszlop 1; b alsó index 1 láthatatlan elválasztó 1; oszlop 2; b alsó index 1 láthatatlan elválasztó 2; 2 . sor; oszlop 1; b alsó index 2 láthatatlan elválasztó 1; oszlop 2; b alsó index 2 láthatatlan elválasztó 2")?;
    return Ok(());
}


// AI generated
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
  test_ClearSpeak("hu", "ClearSpeak_Matrix", "SilentColNum",
        expr, "a(z) 2 x 2 mátrix; 1 . sor; 2, 1; 2 . sor; 7, 5")?;
    return Ok(());
}

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "SilentColNum",
        expr, "a(z) 3 x 1-es oszlop mátrix; 1; 2; 3")?;
    return Ok(());
}

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "SilentColNum",
        expr, "az 1-szer 2 -es sor mátrix; 1, 2")?;
    return Ok(());
}

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "SilentColNum",
        expr, "a(z) 2 x 2 mátrix; 1 . sor; b alsó index 1 láthatatlan elválasztó 1; b alsó index 1 láthatatlan elválasztó 2; 2 . sor; b alsó index 2 láthatatlan elválasztó 1; b alsó index 2 láthatatlan elválasztó 2")?;
    return Ok(());
  }


// AI generated
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
  test_ClearSpeak("hu", "ClearSpeak_Matrix", "EndMatrix",
        expr, "a(z) 2 x 2 mátrix; 1 . sor; 2, 1; 2 . sor; 7, 5; vége a mátrixnak")?;
    return Ok(());
  }

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "EndMatrix",
        expr, "a(z) 3 x 1-es oszlop mátrix; 1; 2; 3; vége a mátrixnak")?;
    return Ok(());
  }

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "EndMatrix",
        expr, "az 1-szer 2 -es sor mátrix; 1, 2; vége a mátrixnak")?;
    return Ok(());
  }

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "EndMatrix",
        expr, "a(z) 2 x 2 mátrix; 1 . sor; oszlop 1; b alsó index 1 láthatatlan elválasztó 1; oszlop 2; b alsó index 1 láthatatlan elválasztó 2; 2 . sor; oszlop 1; b alsó index 2 láthatatlan elválasztó 1; oszlop 2; b alsó index 2 láthatatlan elválasztó 2; vége a mátrixnak")?;
    return Ok(());
  }

#[test]
fn augmented_matrix_3x4_end_matrix() -> Result<()> {
let expr = "<math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  <mrow>
    <mrow><mo>[</mo>
      <mtable columnalign='right right right right' columnlines='none none solid'>
        <mtr>
          <mtd><mn>1</mn></mtd>
          <mtd><mn>2</mn></mtd>
          <mtd><mrow><mo>-</mo><mn>1</mn></mrow></mtd>
          <mtd><mn>3</mn></mtd>
        </mtr>
        <mtr>
          <mtd><mrow><mo>-</mo><mn>3</mn></mrow></mtd>
          <mtd><mn>3</mn></mtd>
          <mtd><mrow><mo>-</mo><mn>1</mn></mrow></mtd>
          <mtd><mn>2</mn></mtd>
        </mtr>
        <mtr>
          <mtd><mn>2</mn></mtd>
          <mtd><mn>3</mn></mtd>
          <mtd><mn>2</mn></mtd>
          <mtd><mrow><mo>-</mo><mn>1</mn></mrow></mtd>
        </mtr>
      </mtable>
    <mo>]</mo></mrow>
  </mrow>
</math>";
test_ClearSpeak("hu", "ClearSpeak_Matrix", "EndMatrix",
        expr, "a(z) 3 x 4 mátrix; 1 . sor; oszlop 1; 1, oszlop 2; 2, oszlop 3; negatív 1, elválasztó, oszlop 4; 3; 2 . sor; oszlop 1; negatív 3, oszlop 2; 3, oszlop 3; negatív 1, elválasztó, oszlop 4; 2; 3 . sor; oszlop 1; 2, oszlop 2; 3, oszlop 3; 2, elválasztó, oszlop 4; negatív 1; vége a mátrixnak")?;
    test("hu", "SimpleSpeak",
        expr, "a(z) 3 x 4 mátrix; 1 . sor; oszlop 1; 1, oszlop 2; 2, oszlop 3; negatív 1, elválasztó, oszlop 4; 3; 2 . sor; oszlop 1; negatív 3, oszlop 2; 3, oszlop 3; negatív 1, elválasztó, oszlop 4; 2; 3 . sor; oszlop 1; 2, oszlop 2; 3, oszlop 3; 2, elválasztó, oszlop 4; negatív 1; vége a mátrixnak")?;
    Ok(())
  }

// AI generated
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
  test_ClearSpeak("hu", "ClearSpeak_Matrix", "Vector",
        expr, "a(z) 2 x 2 mátrix; 1 . sor; 2, 1; 2 . sor; 7, 5")?;
    return Ok(());
  }

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "Vector",
        expr, "a(z) 3 x 1-es oszlop vektor; 1; 2; 3")?;
    return Ok(());
  }

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "Vector",
        expr, "az 1-szer 2 -es sor vektor; 1, 2")?;
    return Ok(());
  }

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "Vector",
        expr, "a(z) 2 x 2 mátrix; 1 . sor; oszlop 1; b alsó index 1 láthatatlan elválasztó 1; oszlop 2; b alsó index 1 láthatatlan elválasztó 2; 2 . sor; oszlop 1; b alsó index 2 láthatatlan elválasztó 1; oszlop 2; b alsó index 2 láthatatlan elválasztó 2")?;
    return Ok(());
  }


// AI generated
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
  test_ClearSpeak("hu", "ClearSpeak_Matrix", "EndVector",
        expr, "a(z) 2 x 2 mátrix; 1 . sor; 2, 1; 2 . sor; 7, 5; vége a mátrixnak")?;
    return Ok(());
  }

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "EndVector",
        expr, "a(z) 3 x 1-es oszlop vektor; 1; 2; 3; vége a vektornak")?;
    return Ok(());
  }

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "EndVector",
        expr, "az 1-szer 2 -es sor vektor; 1, 2; vége a vektornak")?;
    return Ok(());
  }

// AI generated
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
test_ClearSpeak("hu", "ClearSpeak_Matrix", "EndVector",
        expr, "a(z) 2 x 2 mátrix; 1 . sor; oszlop 1; b alsó index 1 láthatatlan elválasztó 1; oszlop 2; b alsó index 1 láthatatlan elválasztó 2; 2 . sor; oszlop 1; b alsó index 2 láthatatlan elválasztó 1; oszlop 2; b alsó index 2 láthatatlan elválasztó 2; vége a mátrixnak")?;
    return Ok(());
  }



// AI generated
#[test]
fn matrix_binomial() -> Result<()> {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("hu", "ClearSpeak_Matrix", "Combinatorics", expr, "3 alatt a 2")?;
    return Ok(());
  }

// AI generated
#[test]
fn matrix_times() -> Result<()> {
  let expr = "<math>
    <mfenced><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable></mfenced>
    <mfenced><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable></mfenced>
  </math>";
  test("hu", "SimpleSpeak", expr,
    "a(z) 2 x 2 mátrix; 1 . sor; 1, 2; 2 . sor; 3, 4; szorozva; a(z) 2 x 2 mátrix; 1 . sor; a, b; 2 . sor; c, d")?;
    return Ok(());
  }

// AI generated
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
    test("hu", "ClearSpeak",  expr,
         "2 sor; 1 . sor; a egyenlő, b plusz c mínusz d; 2 . sor; plusz e mínusz f")?;
    return Ok(());
  }


// AI generated
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
  test("hu", "SimpleSpeak", expr,
    "a(z) 2 x 2 zérusmátrix")?;
    return Ok(());
  }

// AI generated
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
  test("hu", "SimpleSpeak", expr,
    "a(z) 3 x 3 egységmátrix")?;
    return Ok(());
  }

// AI generated
#[test]
fn identity_matrix_false_positive_negative_one() -> Result<()> {
  let expr = "<math>
      <mo>[</mo>
      <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>-1</mn></mtd></mtr>
      </mtable>
      <mo>]</mo>
  </math>";
  test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "a(z) 2 x 2 diagonális mátrix; oszlop 1; 1; oszlop 2; negatív 1")?;
  Ok(())
}

// AI generated
#[test]
fn identity_matrix_false_positive_zero_diagonal() -> Result<()> {
  let expr = "<math>
      <mo>[</mo>
      <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
      </mtable>
      <mo>]</mo>
  </math>";
  test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "a(z) 2 x 2 diagonális mátrix; oszlop 1; 1")?;
  Ok(())
}

// AI generated
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
  test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "a(z) 3 x 3 diagonális mátrix; oszlop 1; 2; oszlop 2; 1; oszlop 3; x a négyzeten")?;
  // test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Verbose")],
  //     expr, "the 3 by 3 diagonal matrix; row 1, column 1, 2; row 2, column 2, 1; row 3, column 3, x squared");
    return Ok(());
  }

// AI generated
#[test]
fn single_line_with_label() -> Result<()> {
  let expr = r#"<math>
  <mtable class="gather" displaystyle="true" intent=":system-of-equations">
    <mtr>
      <mtd intent=":equation-label"> <mtext>(2)</mtext> </mtd>
      <mtd> <mi>𝑏</mi> <mo>=</mo> <mn>2</mn> </mtd>
    </mtr>
  </mtable>
  </math>"#;
  test_prefs("hu", "ClearSpeak", vec![("Verbosity", "Terse")],
      expr, "1 sor, címkével: 2; b egyenlő 2")?;
  test_prefs("hu", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "1 egyenlet, címkével 2; b egyenlő 2")?;
    return Ok(());
  }
