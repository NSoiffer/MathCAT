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
    test("en", "ClearSpeak",  expr, "the 1 by 1 matrix with entry 3")?;
    test("en", "SimpleSpeak", expr, "the 1 by 1 matrix with entry 3")?;
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
    test("en", "ClearSpeak",  expr, "the 1 by 1 determinant with entry 3")?;
    test("en", "SimpleSpeak", expr, "the 1 by 1 determinant with entry 3")?;
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
    test("en", "ClearSpeak",  expr, "the 1 by 2 row matrix; 3, 5")?;
    test("en", "SimpleSpeak", expr, "the 1 by 2 row matrix; 3, 5")?;
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
    test("en", "ClearSpeak", expr, "the 1 by 3 row matrix; negative x, 5, 12")?;
    test("en", "SimpleSpeak", expr, "the 1 by 3 row matrix; negative x, 5, 12")?;
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
    test("en", "ClearSpeak", expr, "the 2 by 1 column matrix; row 1; x plus 1; row 2; x minus 1")?;
    test("en", "SimpleSpeak", expr, "the 2 by 1 column matrix; row 1; x plus 1; row 2; x minus 1")?;
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
    test("en", "SimpleSpeak", expr, "the 3 by 1 column matrix; \
            row 1; x; \
            row 2; eigh; \
            row 3; fraction, x over, x plus 1, end fraction")?;
    test("en", "ClearSpeak",  expr, "the 3 by 1 column matrix; \
            row 1; x; \
            row 2; eigh; \
            row 3; the fraction with numerator x; and denominator x plus 1")?;
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
    test("en", "ClearSpeak",  expr, "the 2 by 2 determinant; row 1; 2, 1; row 2; 7, 5")?;
    test("en", "SimpleSpeak", expr, "the 2 by 2 determinant; row 1; 2, 1; row 2; 7, 5")?;
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
    test("en", "ClearSpeak",  expr, "the 2 by 3 matrix; row 1; 3, 1, 4; row 2; 0, 2, 6")?;
    test("en", "SimpleSpeak", expr, "the 2 by 3 matrix; row 1; 3, 1, 4; row 2; 0, 2, 6")?;
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
    test("en", "ClearSpeak",  expr, "the 2 by 3 augmented matrix; row 1; 3, 1, separator, 4; row 2; 0, 2, separator, 6")?;
    test("en", "SimpleSpeak", expr, "the 2 by 3 augmented matrix; row 1; 3, 1, separator, 4; row 2; 0, 2, separator, 6")?;
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
    test("en", "ClearSpeak", expr, "the 1 by 3 row matrix; 1, separator, 2, separator, 3")?;
    test("en", "SimpleSpeak", expr, "the 1 by 3 row matrix; 1, separator, 2, separator, 3")?;
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
    test("en", "ClearSpeak", expr, "the 2 by 2 matrix; row 1; 1, 2, row separator; row 2; 3, 4")?;
    test("en", "SimpleSpeak", expr, "the 2 by 2 matrix; row 1; 1, 2, row separator; row 2; 3, 4")?;
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
    test("en", "ClearSpeak", expr, "the 2 by 2 augmented matrix; row 1; 1, separator, 2, row separator; row 2; 3, separator, 4")?;
    test("en", "SimpleSpeak", expr, "the 2 by 2 augmented matrix; row 1; 1, separator, 2, row separator; row 2; 3, separator, 4")?;
    Ok(())
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
    test("en", "ClearSpeak",  expr,
        "the 2 by 3 matrix; row 1 with label (3.1); column 1; 3, column 2; 1, column 3; 4; \
                                   row 2; column 1; 0, column 2; 2, column 3; 6")?;
    test("en", "SimpleSpeak", expr,
        "the 2 by 3 matrix; row 1 with label (3.1); column 1; 3, column 2; 1, column 3; 4; \
                                   row 2; column 1; 0, column 2; 2, column 3; 6")?;
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
    test("en", "ClearSpeak",  expr, "the 3 by 1 column matrix; 1; 2; 3")?;
    test("en", "SimpleSpeak", expr, "the 3 by 1 column matrix; 1; 2; 3")?;
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
    test("en", "ClearSpeak",  expr, "the 4 by 1 column matrix; row 1; 3; row 2; 6; row 3; 1; row 4; 2")?;
    test("en", "SimpleSpeak", expr, "the 4 by 1 column matrix; row 1; 3; row 2; 6; row 3; 1; row 4; 2")?;
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
    test("en", "ClearSpeak",  expr,
        "the 4 by 1 column matrix; row 1; 3; row 2; 6; row 3; 1; row 4 with label (3.1); 2")?;
    test("en", "SimpleSpeak", expr,
        "the 4 by 1 column matrix; row 1; 3; row 2; 6; row 3; 1; row 4 with label (3.1); 2")?;
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
    test("en", "ClearSpeak",  expr, "the 1 by 4 row matrix; column 1; 3, column 2; 6, column 3; 1, column 4; 2")?;
    test("en", "SimpleSpeak", expr, "the 1 by 4 row matrix; column 1; 3, column 2; 6, column 3; 1, column 4; 2")?;
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
    test("en", "ClearSpeak",  expr, "the 4 by 4 matrix; \
          row 1; column 1; 0, column 2; 3, column 3; 4, column 4; 3; \
          row 2; column 1; 2, column 2; 1, column 3; 0, column 4; 9; \
          row 3; column 1; 3, column 2; 0, column 3; 2, column 4; 1; \
          row 4; column 1; 6, column 2; 2, column 3; 9, column 4; 0")?;
    test("en", "SimpleSpeak", expr, "the 4 by 4 matrix; \
          row 1; column 1; 0, column 2; 3, column 3; 4, column 4; 3; \
          row 2; column 1; 2, column 2; 1, column 3; 0, column 4; 9; \
          row 3; column 1; 3, column 2; 0, column 3; 2, column 4; 1; \
          row 4; column 1; 6, column 2; 2, column 3; 9, column 4; 0")?;
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
    test("en", "ClearSpeak",  expr, "the 4 by 2 matrix; \
              row 1; column 1; 1, column 2; 3; \
              row 2; column 1; 4, column 2; 2; \
              row 3; column 1; 2, column 2; 1; \
              row 4; column 1; 0, column 2; 5\
    ")?;
    test("en", "SimpleSpeak", expr, "the 4 by 2 matrix; \
              row 1; column 1; 1, column 2; 3; \
              row 2; column 1; 4, column 2; 2; \
              row 3; column 1; 2, column 2; 1; \
              row 4; column 1; 0, column 2; 5\
    ")?;
    return Ok(());
}

// put absolute value test here since it is related to determinate and is small for its own file
#[test]
fn simple_absolute_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("en", "SimpleSpeak", expr, "the absolute value of x")?;
  test("en", "ClearSpeak",  expr, "the absolute value of x")?;
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "absolute value of x")?;
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "the absolute value of x, end absolute value")?;
  return Ok(());
}
  
#[test]
fn absolute_value_plus_1() -> Result<()> {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("en", "ClearSpeak", expr, "the absolute value of x plus 1")?;
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "absolute value of x plus 1, end absolute value")?;
  return Ok(());
}

#[test]
fn simple_cardinality_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "the cardinality of cap s")?;
    return Ok(());
}
  
// Test preferences
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
  test_ClearSpeak("en", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "the 2 by 2 matrix; row 1; column 1; 2, column 2; 1; row 2; column 1; 7, column 2; 5")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "the 3 by 1 column matrix; row 1; 1; row 2; 2; row 3; 3")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "the 1 by 2 row matrix; column 1; 1, column 2; 2")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "the 2 by 2 matrix; row 1; column 1; b sub 1 1; column 2; b sub 1 2; \
                                                row 2; column 1; b sub 2 1; column 2; b sub 2 2")?;
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
  test_ClearSpeak("en", "ClearSpeak_Matrix", "SilentColNum",
        expr, "the 2 by 2 matrix; row 1; 2, 1; row 2; 7, 5")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "SilentColNum",
        expr, "the 3 by 1 column matrix; 1; 2; 3")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "SilentColNum",
        expr, "the 1 by 2 row matrix; 1, 2")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "SilentColNum",
        expr, "the 2 by 2 matrix; row 1; b sub 1 1; b sub 1 2; \
                                                row 2; b sub 2 1; b sub 2 2")?;
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
  test_ClearSpeak("en", "ClearSpeak_Matrix", "EndMatrix",
        expr, "the 2 by 2 matrix; row 1; 2, 1; row 2; 7, 5; end matrix")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "EndMatrix",
        expr, "the 3 by 1 column matrix; 1; 2; 3; end matrix")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "EndMatrix",
        expr, "the 1 by 2 row matrix; 1, 2; end matrix")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "EndMatrix",
        expr, "the 2 by 2 matrix; row 1; column 1; b sub 1 1; column 2; b sub 1 2; \
                                                row 2; column 1; b sub 2 1; column 2; b sub 2 2; end matrix")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "EndMatrix",
        expr, "the 3 by 4 augmented matrix; row 1; column 1; 1, column 2; 2, column 3; negative 1, separator, column 4; 3; \
               row 2; column 1; negative 3, column 2; 3, column 3; negative 1, separator, column 4; 2; \
               row 3; column 1; 2, column 2; 3, column 3; 2, separator, column 4; negative 1; end matrix")?;
    test("en", "SimpleSpeak",
        expr, "the 3 by 4 augmented matrix; row 1; column 1; 1, column 2; 2, column 3; negative 1, separator, column 4; 3; \
               row 2; column 1; negative 3, column 2; 3, column 3; negative 1, separator, column 4; 2; \
               row 3; column 1; 2, column 2; 3, column 3; 2, separator, column 4; negative 1; end matrix")?;
    Ok(())
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
  test_ClearSpeak("en", "ClearSpeak_Matrix", "Vector",
        expr, "the 2 by 2 matrix; row 1; 2, 1; row 2; 7, 5")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "Vector",
        expr, "the 3 by 1 column vector; 1; 2; 3")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "Vector",
        expr, "the 1 by 2 row vector; 1, 2")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "Vector",
        expr, "the 2 by 2 matrix; row 1; column 1; b sub 1 1; column 2; b sub 1 2; \
                                                row 2; column 1; b sub 2 1; column 2; b sub 2 2")?;
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
  test_ClearSpeak("en", "ClearSpeak_Matrix", "EndVector",
        expr, "the 2 by 2 matrix; row 1; 2, 1; row 2; 7, 5; end matrix")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "EndVector",
        expr, "the 3 by 1 column vector; 1; 2; 3; end vector")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "EndVector",
        expr, "the 1 by 2 row vector; 1, 2; end vector")?;
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "EndVector",
        expr, "the 2 by 2 matrix; row 1; column 1; b sub 1 1; column 2; b sub 1 2; \
                                                 row 2; column 1; b sub 2 1; column 2; b sub 2 2; end matrix")?;
  return Ok(());
}


#[test]
fn matrix_binomial() -> Result<()> {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("en", "ClearSpeak_Matrix", "Combinatorics", expr, "3 choose 2")?;
  return Ok(());
}

#[test]
fn matrix_simple_table() -> Result<()> {
  let expr = "<math>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
    </math>";
  test("en", "ClearSpeak", expr, "array of; row 1; column 1; 3; comma; row 2; column 1; 2")
}

#[test]
fn mtable_prefix_op() -> Result<()>{
    // When a table is prefixed with a non-blank operator, assume it is something besides array intent
    for (op, speech) in [("(", "open paren"),
			 ("[", "open bracket"),
			 ("|", "vertical line"),
			 ("f", "f")] {
	let expr = format!("<math>
        <mo>{op}</mo><mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
    </math>");
	test("en", "ClearSpeak", &expr, &format!("{speech}, 2 lines; line 1; 3; line 2; 2"))?;
    }
    Ok(())
}

#[test]
fn mtable_blank_op() -> Result<()>{
    // When a table is prefixed with a blank operator, still assume array intent
    let expr = "<math>
        <mo>\u{2062}</mo><mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
    </math>";
    test("en", "ClearSpeak", expr, "; array of; row 1; column 1; 3; comma; row 2; column 1; 2")
}



#[test]
fn mtable_colspan_table() -> Result<()>{
  let expr = "<math>
        <mtable><mtr><mtd colspan=\"2\"><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable>
    </math>";
  test("en", "ClearSpeak", expr, "array of; row 1; column 1; 3; comma; row 2; column 1; 2, column 2; 4")
}

#[test]
fn bug_mtable_rowspan_colspan() -> Result<()>{
  // Currently, the code correctly computes the number of rows and
  // columns, but it does not correctly compute the column number whnen
  // colspans are involved.
  let expr = "<math>
        <mtable>
           <mtr>
             <mtd rowspan=\"2\" colspan=\"2\"><mi>a</mi></mtd>
             <mtd rowspan=\"2\"><mi>b</mi></mtd>
             <mtd colspan=\"2\"><mi>c</mi></mtd>
           </mtr>
           <mtr>
             <mtd><mi>d</mi></mtd><mtd><mi>e</mi></mtd>
           </mtr>
        </mtable>
    </math>";
    test("en", "ClearSpeak", expr,
	 "array of; row 1; column 1; eigh, column 2; b, column 3; c; comma; row 2; column 1; d, column 2; e")
}



#[test]
fn matrix_times() {
  let expr = "<math>
    <mfenced><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable></mfenced>
    <mfenced><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable></mfenced>
  </math>";
  let _ = test("en", "SimpleSpeak", expr,
    "the 2 by 2 matrix; row 1; 1, 2; row 2; 3, 4; times, the 2 by 2 matrix; row 1; eigh, b; row 2; c, d");
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
    test("en", "ClearSpeak",  expr,
         "2 lines; line 1; eigh is equal to, b plus c minus d; line 2; plus e minus f")?;
    return Ok(());
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
  test("en", "SimpleSpeak", expr,
    "the 2 by 2 zero matrix")?;
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
  test("en", "SimpleSpeak", expr,
    "the 3 by 3 identity matrix")?;
    return Ok(());
  }

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
  test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "the 2 by 2 diagonal matrix; column 1; 1; column 2; negative 1")?;
  Ok(())
}

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
  test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "the 2 by 2 diagonal matrix; column 1; 1")?;
  Ok(())
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
  test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "the 3 by 3 diagonal matrix; column 1; 2; column 2; 1; column 3; x squared")?;
  // test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Verbose")],
  //     expr, "the 3 by 3 diagonal matrix; row 1, column 1, 2; row 2, column 2, 1; row 3, column 3, x squared");
    return Ok(());
  }

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
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Terse")],
      expr, "1 line, with label 2; b equals 2")?;
  test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "1 equation, with label 2; b equals 2")?;
    return Ok(());
  }

#[test]
fn matrix_raised_to_power() -> Result<()> {
  // A parenthesized matrix as the base of msup must speak as "the matrix ... squared".
  // Regression for #762: the mrow 'matrix' intent rule returns a bare `x:`, so the intent tree
  // carried a nested TEMP_NAME wrapper into `power` and NVDA read "TEMP NAME of the 2 by 2 matrix ...".
  let expr = "<math><msup>
      <mrow><mo>(</mo>
        <mtable>
          <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr>
          <mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr>
        </mtable>
      <mo>)</mo></mrow>
      <mn>2</mn>
    </msup></math>";
  test("en", "ClearSpeak", expr, "the 2 by 2 matrix; row 1; 1, 2; row 2; 3, 4; squared")
}

#[test]
fn matrix_raised_to_power_lualatex_spacing() -> Result<()> {
  // The exact MathML from #762 (LuaLaTeX pmatrix output): negative-width mspace around the
  // table and fence attributes on the parens. Canonicalization must fold the spacing away so
  // the matrix is still recognized as the base of the power.
  let expr = "<math display='block'>
    <msup>
      <mrow>
        <mo fence='true' lspace='0' rspace='0' symmetric='true'>(</mo>
        <mspace width='-4.981pt'/>
        <mrow>
          <mspace width='4.981pt'/>
          <mtable>
            <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr>
            <mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr>
          </mtable>
          <mspace width='4.981pt'/>
        </mrow>
        <mspace width='-4.981pt'/>
        <mo fence='true' lspace='0' rspace='0' symmetric='true'>)</mo>
      </mrow>
      <mn>2</mn>
    </msup>
  </math>";
  test("en", "ClearSpeak", expr, "the 2 by 2 matrix; row 1; 1, 2; row 2; 3, 4; squared")
}
