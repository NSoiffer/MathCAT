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
    test("de", "ClearSpeak",  expr, "die 1 mal 1 matrix mit eintrag 3")?;
    test("de", "SimpleSpeak", expr, "die 1 mal 1 matrix mit eintrag 3")?;
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
    test("de", "ClearSpeak",  expr, "die 1 mal 1 determinante mit eintrag 3")?;
    test("de", "SimpleSpeak", expr, "die 1 mal 1 determinante mit eintrag 3")?;
    return Ok(());

}

#[test]
fn single_line_with_label() -> Result<()> {
    let expr = r#"<math>
  <mtable class="gather" displaystyle="true" intent=":system-of-equations">
    <mtr>
      <mtd intent=":equation-label"><mtext>(2)</mtext></mtd>
      <mtd><mi>𝑏</mi><mo>=</mo><mn>2</mn></mtd>
    </mtr>
  </mtable>
  </math>"#;
    test_prefs("de", "ClearSpeak", vec![("Verbosity", "Terse")],
        expr, "1 zeile, mit Beschriftung 2; b ist gleich 2")?;
    Ok(())
}

// Added after a rule review found the German 1xN row-matrix rule repeated `die 1 mal` instead of saying `matrix`/`vektor`.
#[test]
fn matrix_1x2_row_regression() -> Result<()> {
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
    test("de", "ClearSpeak",  expr, "die 1 mal 2 Zeile matrix; 3, 5")?;
    test("de", "SimpleSpeak", expr, "die 1 mal 2 Zeile matrix; 3, 5")?;
    Ok(())
}


/*
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
    test("de", "ClearSpeak",  expr, "the 1 by 2 row matrix; 3, 5")?;
    test("de", "SimpleSpeak", expr, "the 1 by 2 row matrix; 3, 5")?;
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
    test("de", "ClearSpeak", expr, "the 1 by 3 row matrix; negative x, 5, 12")?;
    test("de", "SimpleSpeak", expr, "the 1 by 3 row matrix; negative x, 5, 12")?;
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
    test("de", "ClearSpeak", expr, "the 2 by 1 column matrix; row 1; x plus 1; row 2; x minus 1")?;
    test("de", "SimpleSpeak", expr, "the 2 by 1 column matrix; row 1; x plus 1; row 2; x minus 1")?;
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
    test("de", "SimpleSpeak", expr, "the 3 by 1 column matrix; \
            row 1; x; \
            row 2; eigh; \
            row 3; fraction, x over, x plus 1, end fraction")?;
    test("de", "ClearSpeak",  expr, "the 3 by 1 column matrix; \
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
    test("de", "ClearSpeak",  expr, "the 2 by 2 determinant; row 1; 2, 1; row 2; 7, 5")?;
    test("de", "SimpleSpeak", expr, "the 2 by 2 determinant; row 1; 2, 1; row 2; 7, 5")?;
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
    test("de", "ClearSpeak",  expr, "the 2 by 3 matrix; row 1; 3, 1, 4; row 2; 0, 2, 6")?;
    test("de", "SimpleSpeak", expr, "the 2 by 3 matrix; row 1; 3, 1, 4; row 2; 0, 2, 6")?;
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
    test("de", "ClearSpeak",  expr,
        "the 2 by 3 matrix; row 1 with label (3.1); column 1; 3, column 2; 1, column 3; 4; \
                                   row 2; column 1; 0, column 2; 2, column 3; 6")?;
    test("de", "SimpleSpeak", expr,
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
    test("de", "ClearSpeak",  expr, "the 3 by 1 column matrix; 1; 2; 3")?;
    test("de", "SimpleSpeak", expr, "the 3 by 1 column matrix; 1; 2; 3")?;
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
    test("de", "ClearSpeak",  expr, "die 4 nach 1 spalte matrix; reihe 1; 3; reihe 2; 6; reihe 3; 1; reihe 4; 2")?;
    test("de", "SimpleSpeak", expr, "die 4 nach 1 spalte matrix; reihe 1; 3; reihe 2; 6; reihe 3; 1; reihe 4; 2")?;
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
    test("de", "ClearSpeak",  expr,
        "die 4 nach 1 spalte matrix; reihe 1; 3; reihe 2; 6; reihe 3; 1; reihe 4 determinant (3.1); 2")?;
    test("de", "SimpleSpeak", expr,
        "die 4 nach 1 spalte matrix; reihe 1; 3; reihe 2; 6; reihe 3; 1; reihe 4 mit der etikette (3.1); 2")?;
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
    test("de", "ClearSpeak",  expr, "the 1 by 4 row matrix; column 1; 3, column 2; 6, column 3; 1, column 4; 2")?;
    test("de", "SimpleSpeak", expr, "the 1 by 4 row matrix; column 1; 3, column 2; 6, column 3; 1, column 4; 2")?;
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
    test("de", "ClearSpeak",  expr, "the 4 by 4 matrix; \
          row 1; column 1; 0, column 2; 3, column 3; 4, column 4; 3; \
          row 2; column 1; 2, column 2; 1, column 3; 0, column 4; 9; \
          row 3; column 1; 3, column 2; 0, column 3; 2, column 4; 1; \
          row 4; column 1; 6, column 2; 2, column 3; 9, column 4; 0")?;
    test("de", "SimpleSpeak", expr, "the 4 by 4 matrix; \
          row 1; column 1; 0, column 2; 3, column 3; 4, column 4; 3; \
          row 2; column 1; 2, column 2; 1, column 3; 0, column 4; 9; \
          row 3; column 1; 3, column 2; 0, column 3; 2, column 4; 1; \
          row 4; column 1; 6, column 2; 2, column 3; 9, column 4; 0");}

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
    test("de", "ClearSpeak",  expr, "the 4 by 2 matrix; \
              row 1; column 1; 1, column 2; 3; \
              row 2; column 1; 4, column 2; 2; \
              row 3; column 1; 2, column 2; 1; \
              row 4; column 1; 0, column 2; 5\
    ")?;
    test("de", "SimpleSpeak", expr, "the 4 by 2 matrix; \
              row 1; column 1; 1, column 2; 3; \
              row 2; column 1; 4, column 2; 2; \
              row 3; column 1; 2, column 2; 1; \
              row 4; column 1; 0, column 2; 5\
    ");}

// put absolute value test here since it is related to determinate and is small for its own file
#[test]
fn simple_absolute_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("de", "SimpleSpeak", expr, "the absolute value of x")?;
  test("de", "ClearSpeak",  expr, "the absolute value of x")?;
  test_prefs("de", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "absolute value of x")?;
  test_prefs("de", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "the absolute value of x, end absolute value")?;
}
  
#[test]
fn absolute_value_plus_1() -> Result<()> {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("de", "ClearSpeak", expr, "der Betrag von x plus 1")?;
  test_prefs("de", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "der betrag von x plus 1, end betrag")?;
}

#[test]
fn simple_cardinality_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("de", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "the cardinality of cap s")?;
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
  test_ClearSpeak("de", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "the 2 by 2 matrix; row 1; column 1; 2, column 2; 1; row 2; column 1; 7, column 2; 5")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "the 3 by 1 column matrix; row 1; 1; row 2; 2; row 3; 3")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "the 1 by 2 row matrix; column 1; 1, column 2; 2")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "the 2 by 2 matrix; row 1; column 1; b sub 1 1; column 2; b sub 1 2; \
                                                row 2; column 1; b sub 2 1; column 2; b sub 2 2")?;
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
  test_ClearSpeak("de", "ClearSpeak_Matrix", "SilentColNum",
        expr, "the 2 by 2 matrix; row 1; 2, 1; row 2; 7, 5")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "SilentColNum",
        expr, "the 3 by 1 column matrix; 1; 2; 3")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "SilentColNum",
        expr, "the 1 by 2 row matrix; 1, 2")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "SilentColNum",
        expr, "the 2 by 2 matrix; row 1; b sub 1 1; b sub 1 2; \
                                                row 2; b sub 2 1; b sub 2 2")?;
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
  test_ClearSpeak("de", "ClearSpeak_Matrix", "EndMatrix",
        expr, "the 2 by 2 matrix; row 1; 2, 1; row 2; 7, 5; end matrix")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "EndMatrix",
        expr, "the 3 by 1 column matrix; 1; 2; 3; end matrix")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "EndMatrix",
        expr, "the 1 by 2 row matrix; 1, 2; end matrix")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "EndMatrix",
        expr, "the 2 by 2 matrix; row 1; column 1; b sub 1 1; column 2; b sub 1 2; \
                                                row 2; column 1; b sub 2 1; column 2; b sub 2 2; end matrix")?;
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
  test_ClearSpeak("de", "ClearSpeak_Matrix", "Vector",
        expr, "the 2 by 2 matrix; row 1; 2, 1; row 2; 7, 5")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "Vector",
        expr, "the 3 by 1 column vector; 1; 2; 3")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "Vector",
        expr, "the 1 by 2 row vector; 1, 2")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "Vector",
        expr, "the 2 by 2 matrix; row 1; column 1; b sub 1 1; column 2; b sub 1 2; \
                                                row 2; column 1; b sub 2 1; column 2; b sub 2 2")?;
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
  test_ClearSpeak("de", "ClearSpeak_Matrix", "EndVector",
        expr, "the 2 by 2 matrix; row 1; 2, 1; row 2; 7, 5; end matrix")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "EndVector",
        expr, "the 3 by 1 column vector; 1; 2; 3; end vector")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "EndVector",
        expr, "the 1 by 2 row vector; 1, 2; end vector")?;
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
test_ClearSpeak("de", "ClearSpeak_Matrix", "EndVector",
        expr, "the 2 by 2 matrix; row 1; column 1; b sub 1 1; column 2; b sub 1 2; \
                                                 row 2; column 1; b sub 2 1; column 2; b sub 2 2; end matrix")?;
}



#[test]
fn matrix_binomial() -> Result<()> {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("de", "ClearSpeak_Matrix", "Combinatorics", expr, "3 choose 2")?;
}

#[test]
fn matrix_times() -> Result<()> {
  let expr = "<math>
    <mfenced><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable></mfenced>
    <mfenced><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable></mfenced>
  </math>";
  test("de", "SimpleSpeak", expr,
    "the 2 by 2 matrix; row 1; 1, 2; row 2; 3, 4; times, the 2 by 2 matrix; row 1; eigh, b; row 2; c, d")?;
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
    test("de", "ClearSpeak",  expr,
         "2 lines; line 1; eigh is equal to, b plus c minus d; line 2; plus e minus f")?;
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
  test("de", "SimpleSpeak", expr,
    "the 2 by 2 zero matrix")?;
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
  test("de", "SimpleSpeak", expr,
    "the 3 by 3 identity matrix")?;
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
  test_prefs("de", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "the 3 by 3 diagonal matrix; column 1; 2; column 2; 1; column 3; x squared")?;
  // test_prefs("de", "SimpleSpeak", vec![("Verbosity", "Verbose")],
  //     expr, "the 3 by 3 diagonal matrix; row 1, column 1, 2; row 2, column 2, 1; row 3, column 3, x squared");
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
  test_prefs("de", "ClearSpeak", vec![("Verbosity", "Terse")],
      expr, "1 line, with label 2; b equals 2")?;
  test_prefs("de", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "1 equation, with label 2; b equals 2")?;
}
return Ok(());

*/

#[test]
fn identity_matrix() -> Result<()> {
    let expr = "<math>
      <mo>[</mo>
      <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr>
      </mtable>
      <mo>]</mo>
  </math>";
    test("de", "SimpleSpeak", expr, "die 3 mal 3 identitätsmatrix")?;
    Ok(())
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
    test_prefs("de", "SimpleSpeak", vec![("Verbosity", "Terse")],
        expr, "die 2 mal 2 diagonalmatrix; spalte 1; 1; spalte 2; minus 1")?;
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
    test_prefs("de", "SimpleSpeak", vec![("Verbosity", "Terse")],
        expr, "die 2 mal 2 diagonalmatrix; spalte 1; 1")?;
    Ok(())
}
