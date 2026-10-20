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
    test("pl", "ClearSpeak",  expr, "jeden na jeden macierz z elementem 3")?;
    test("pl", "SimpleSpeak", expr, "jeden na jeden macierz z elementem 3")?;
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
    test("pl", "ClearSpeak",  expr, "jeden na jeden wyznacznik z elementem 3")?;
    test("pl", "SimpleSpeak", expr, "jeden na jeden wyznacznik z elementem 3")?;
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
    test("pl", "ClearSpeak",  expr, "jeden na 2 wiersz macierz; 3, 5")?;
    test("pl", "SimpleSpeak", expr, "jeden na 2 wiersz macierz; 3, 5")?;
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
    test("pl", "ClearSpeak", expr, "jeden na 3 wiersz macierz; minus x, 5, 12")?;
    test("pl", "SimpleSpeak", expr, "jeden na 3 wiersz macierz; minus x, 5, 12")?;
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
    test("pl", "ClearSpeak", expr, "2 na jedną kolumnę macierz; wiersz 1; x plus 1; wiersz 2; x minus 1")?;
    test("pl", "SimpleSpeak", expr, "2 na jedną kolumnę macierz; wiersz 1; x plus 1; wiersz 2; x minus 1")?;
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
    test("pl", "SimpleSpeak", expr, "3 na jedną kolumnę macierz; wiersz 1; x; wiersz 2; a; wiersz 3; ułamek, x przez, x plus 1, koniec ułamka")?;
    test("pl", "ClearSpeak",  expr, "3 na jedną kolumnę macierz; wiersz 1; x; wiersz 2; a; wiersz 3; ułamek z licznikiem x; i mianownikiem x plus 1")?;
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
    test("pl", "ClearSpeak",  expr, "2 na 2 wyznacznik; wiersz 1; 2, 1; wiersz 2; 7, 5")?;
    test("pl", "SimpleSpeak", expr, "2 na 2 wyznacznik; wiersz 1; 2, 1; wiersz 2; 7, 5")?;
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
    test("pl", "ClearSpeak",  expr, "2 na 3 macierz; wiersz 1; 3, 1, 4; wiersz 2; 0, 2, 6")?;
    test("pl", "SimpleSpeak", expr, "2 na 3 macierz; wiersz 1; 3, 1, 4; wiersz 2; 0, 2, 6")?;
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
    test("pl", "ClearSpeak",  expr, "2 na 3 macierz rozszerzona; wiersz 1; 3, 1, separator, 4; wiersz 2; 0, 2, separator, 6")?;
    test("pl", "SimpleSpeak", expr, "2 na 3 macierz rozszerzona; wiersz 1; 3, 1, separator, 4; wiersz 2; 0, 2, separator, 6")?;
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
    test("pl", "ClearSpeak",  expr,
        "2 na 3 macierz; wiersz 1 z etykietą (3.1); kolumna 1; 3, kolumna 2; 1, kolumna 3; 4; wiersz 2; kolumna 1; 0, kolumna 2; 2, kolumna 3; 6")?;
    test("pl", "SimpleSpeak", expr,
        "2 na 3 macierz; wiersz 1 z etykietą (3.1); kolumna 1; 3, kolumna 2; 1, kolumna 3; 4; wiersz 2; kolumna 1; 0, kolumna 2; 2, kolumna 3; 6")?;
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
    test("pl", "ClearSpeak",  expr, "3 na jedną kolumnę macierz; 1; 2; 3")?;
    test("pl", "SimpleSpeak", expr, "3 na jedną kolumnę macierz; 1; 2; 3")?;
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
    test("pl", "ClearSpeak",  expr, "4 na jedną kolumnę macierz; wiersz 1; 3; wiersz 2; 6; wiersz 3; 1; wiersz 4; 2")?;
    test("pl", "SimpleSpeak", expr, "4 na jedną kolumnę macierz; wiersz 1; 3; wiersz 2; 6; wiersz 3; 1; wiersz 4; 2")?;
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
    test("pl", "ClearSpeak",  expr,
        "4 na jedną kolumnę macierz; wiersz 1; 3; wiersz 2; 6; wiersz 3; 1; wiersz 4 z etykietą (3.1); 2")?;
    test("pl", "SimpleSpeak", expr,
        "4 na jedną kolumnę macierz; wiersz 1; 3; wiersz 2; 6; wiersz 3; 1; wiersz 4 z etykietą (3.1); 2")?;
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
    test("pl", "ClearSpeak",  expr, "jeden na 4 wiersz macierz; kolumna 1; 3, kolumna 2; 6, kolumna 3; 1, kolumna 4; 2")?;
    test("pl", "SimpleSpeak", expr, "jeden na 4 wiersz macierz; kolumna 1; 3, kolumna 2; 6, kolumna 3; 1, kolumna 4; 2")?;
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
    test("pl", "ClearSpeak",  expr, "4 na 4 macierz; wiersz 1; kolumna 1; 0, kolumna 2; 3, kolumna 3; 4, kolumna 4; 3; wiersz 2; kolumna 1; 2, kolumna 2; 1, kolumna 3; 0, kolumna 4; 9; wiersz 3; kolumna 1; 3, kolumna 2; 0, kolumna 3; 2, kolumna 4; 1; wiersz 4; kolumna 1; 6, kolumna 2; 2, kolumna 3; 9, kolumna 4; 0")?;
    test("pl", "SimpleSpeak", expr, "4 na 4 macierz; wiersz 1; kolumna 1; 0, kolumna 2; 3, kolumna 3; 4, kolumna 4; 3; wiersz 2; kolumna 1; 2, kolumna 2; 1, kolumna 3; 0, kolumna 4; 9; wiersz 3; kolumna 1; 3, kolumna 2; 0, kolumna 3; 2, kolumna 4; 1; wiersz 4; kolumna 1; 6, kolumna 2; 2, kolumna 3; 9, kolumna 4; 0")?;
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
    test("pl", "ClearSpeak",  expr, "4 na 2 macierz; wiersz 1; kolumna 1; 1, kolumna 2; 3; wiersz 2; kolumna 1; 4, kolumna 2; 2; wiersz 3; kolumna 1; 2, kolumna 2; 1; wiersz 4; kolumna 1; 0, kolumna 2; 5")?;
    test("pl", "SimpleSpeak", expr, "4 na 2 macierz; wiersz 1; kolumna 1; 1, kolumna 2; 3; wiersz 2; kolumna 1; 4, kolumna 2; 2; wiersz 3; kolumna 1; 2, kolumna 2; 1; wiersz 4; kolumna 1; 0, kolumna 2; 5")?;
    return Ok(());
}

// put absolute value test here since it is related to determinate and is small for its own file
#[test]
fn simple_absolute_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("pl", "SimpleSpeak", expr, "wartość bezwzględna z x")?;
  test("pl", "ClearSpeak",  expr, "wartość bezwzględna z x")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "wartość bezwzględna z x")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "wartość bezwzględna z x, koniec wartości bezwzględnej")?;
  return Ok(());
}
  
#[test]
fn absolute_value_plus_1() -> Result<()> {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("pl", "ClearSpeak", expr, "wartość bezwzględna z x plus 1")?;
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "wartość bezwzględna z x plus 1, koniec wartości bezwzględnej")?;
  return Ok(());
}

#[test]
fn simple_cardinality_value() -> Result<()> {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "moc z wielka s")?;
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
  test_ClearSpeak("pl", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "2 na 2 macierz; wiersz 1; kolumna 1; 2, kolumna 2; 1; wiersz 2; kolumna 1; 7, kolumna 2; 5")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "3 na jedną kolumnę macierz; wiersz 1; 1; wiersz 2; 2; wiersz 3; 3")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "jeden na 2 wiersz macierz; kolumna 1; 1, kolumna 2; 2")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "2 na 2 macierz; wiersz 1; kolumna 1; b indeks dolny 1 1; kolumna 2; b indeks dolny 1 2; wiersz 2; kolumna 1; b indeks dolny 2 1; kolumna 2; b indeks dolny 2 2")?;
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
  test_ClearSpeak("pl", "ClearSpeak_Matrix", "SilentColNum",
        expr, "2 na 2 macierz; wiersz 1; 2, 1; wiersz 2; 7, 5")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "SilentColNum",
        expr, "3 na jedną kolumnę macierz; 1; 2; 3")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "SilentColNum",
        expr, "jeden na 2 wiersz macierz; 1, 2")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "SilentColNum",
        expr, "2 na 2 macierz; wiersz 1; b indeks dolny 1 1; b indeks dolny 1 2; wiersz 2; b indeks dolny 2 1; b indeks dolny 2 2")?;
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
  test_ClearSpeak("pl", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 na 2 macierz; wiersz 1; 2, 1; wiersz 2; 7, 5; koniec macierzy")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "EndMatrix",
        expr, "3 na jedną kolumnę macierz; 1; 2; 3; koniec macierzy")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "EndMatrix",
        expr, "jeden na 2 wiersz macierz; 1, 2; koniec macierzy")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 na 2 macierz; wiersz 1; kolumna 1; b indeks dolny 1 1; kolumna 2; b indeks dolny 1 2; wiersz 2; kolumna 1; b indeks dolny 2 1; kolumna 2; b indeks dolny 2 2; koniec macierzy")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "EndMatrix",
        expr, "3 na 4 macierz rozszerzona; wiersz 1; kolumna 1; 1, kolumna 2; 2, kolumna 3; minus 1, separator, kolumna 4; 3; wiersz 2; kolumna 1; minus 3, kolumna 2; 3, kolumna 3; minus 1, separator, kolumna 4; 2; wiersz 3; kolumna 1; 2, kolumna 2; 3, kolumna 3; 2, separator, kolumna 4; minus 1; koniec macierzy")?;
    test("pl", "SimpleSpeak",
        expr, "3 na 4 macierz rozszerzona; wiersz 1; kolumna 1; 1, kolumna 2; 2, kolumna 3; minus 1, separator, kolumna 4; 3; wiersz 2; kolumna 1; minus 3, kolumna 2; 3, kolumna 3; minus 1, separator, kolumna 4; 2; wiersz 3; kolumna 1; 2, kolumna 2; 3, kolumna 3; 2, separator, kolumna 4; minus 1; koniec macierzy")?;
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
  test_ClearSpeak("pl", "ClearSpeak_Matrix", "Vector",
        expr, "2 na 2 macierz; wiersz 1; 2, 1; wiersz 2; 7, 5")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "Vector",
        expr, "3 na jedną kolumnę wektor; 1; 2; 3")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "Vector",
        expr, "jeden na 2 wiersz wektor; 1, 2")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "Vector",
        expr, "2 na 2 macierz; wiersz 1; kolumna 1; b indeks dolny 1 1; kolumna 2; b indeks dolny 1 2; wiersz 2; kolumna 1; b indeks dolny 2 1; kolumna 2; b indeks dolny 2 2")?;
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
  test_ClearSpeak("pl", "ClearSpeak_Matrix", "EndVector",
        expr, "2 na 2 macierz; wiersz 1; 2, 1; wiersz 2; 7, 5; koniec macierzy")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "EndVector",
        expr, "3 na jedną kolumnę wektor; 1; 2; 3; koniec wektora")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "EndVector",
        expr, "jeden na 2 wiersz wektor; 1, 2; koniec wektora")?;
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
test_ClearSpeak("pl", "ClearSpeak_Matrix", "EndVector",
        expr, "2 na 2 macierz; wiersz 1; kolumna 1; b indeks dolny 1 1; kolumna 2; b indeks dolny 1 2; wiersz 2; kolumna 1; b indeks dolny 2 1; kolumna 2; b indeks dolny 2 2; koniec macierzy")?;
  return Ok(());
}


#[test]
fn matrix_binomial() -> Result<()> {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("pl", "ClearSpeak_Matrix", "Combinatorics", expr, "3 po 2")?;
  return Ok(());
}

#[test]
fn matrix_simple_table() -> Result<()> {
  let expr = "<math>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
    </math>";
  test("pl", "ClearSpeak", expr, "tablica z; wiersz 1; kolumna 1; 3; przecinek; wiersz 2; kolumna 1; 2")
}

#[test]
fn mtable_prefix_op() -> Result<()>{
    // When a table is prefixed with a non-blank operator, assume it is something besides array intent
    for (op, expected) in [("(", "nawias otwierający, 2 linie; linia 1; 3; linia 2; 2"),
			 ("[", "nawias kwadratowy otwierający; 2 linie; linia 1; 3; linia 2; 2"),
			 ("|", "kreska pionowa, 2 linie; linia 1; 3; linia 2; 2"),
			 ("f", "f, 2 linie; linia 1; 3; linia 2; 2")] {
	let expr = format!("<math>
        <mo>{op}</mo><mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
    </math>");
	test("pl", "ClearSpeak", &expr, expected)?;
    }
    Ok(())
}

#[test]
fn mtable_blank_op() -> Result<()>{
    // When a table is prefixed with a blank operator, still assume array intent
    let expr = "<math>
        <mo>\u{2062}</mo><mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
    </math>";
    test("pl", "ClearSpeak", expr, "; tablica z; wiersz 1; kolumna 1; 3; przecinek; wiersz 2; kolumna 1; 2")
}



#[test]
fn mtable_colspan_table() -> Result<()>{
  let expr = "<math>
        <mtable><mtr><mtd colspan=\"2\"><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable>
    </math>";
  test("pl", "ClearSpeak", expr, "tablica z; wiersz 1; kolumna 1; 3; przecinek; wiersz 2; kolumna 1; 2, kolumna 2; 4")
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
    test("pl", "ClearSpeak", expr,
	 "tablica z; wiersz 1; kolumna 1; a, kolumna 2; b, kolumna 3; c; przecinek; wiersz 2; kolumna 1; d, kolumna 2; e")
}



#[test]
fn matrix_times() {
  let expr = "<math>
    <mfenced><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable></mfenced>
    <mfenced><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable></mfenced>
  </math>";
  let _ = test("pl", "SimpleSpeak", expr,
    "2 na 2 macierz; wiersz 1; 1, 2; wiersz 2; 3, 4; razy, 2 na 2 macierz; wiersz 1; a, b; wiersz 2; c, d");
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
    test("pl", "ClearSpeak",  expr,
         "2 linie; linia 1; a równa się, b plus c minus d; linia 2; plus e minus f")?;
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
  test("pl", "SimpleSpeak", expr,
    "2 na 2 macierz zerowa")?;
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
  test("pl", "SimpleSpeak", expr,
    "3 na 3 macierz jednostkowa")?;
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
  test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "2 na 2 macierz diagonalna; kolumna 1; 1; kolumna 2; minus 1")?;
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
  test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "2 na 2 macierz diagonalna; kolumna 1; 1")?;
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
  test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "3 na 3 macierz diagonalna; kolumna 1; 2; kolumna 2; 1; kolumna 3; x do kwadratu")?;
  // test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")],
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
  test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Terse")],
      expr, "1 linia, z etykietą 2; b równa się 2")?;
  test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "1 równanie, z etykietą 2; b równa się 2")?;
    return Ok(());
  }

/// Linia pionowa w tabeli jest zapowiadana jako separator (upstream #679).
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
    test("pl", "ClearSpeak", expr, "jeden na 3 wiersz macierz; 1, separator, 2, separator, 3")?;
    test("pl", "SimpleSpeak", expr, "jeden na 3 wiersz macierz; 1, separator, 2, separator, 3")?;
    Ok(())
}

/// Linia pozioma wybrzmiewa RAZ, po wierszu, ktory oddziela od nastepnego.
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
    test("pl", "ClearSpeak", expr, "2 na 2 macierz; wiersz 1; 1, 2, separator wiersza; wiersz 2; 3, 4")?;
    test("pl", "SimpleSpeak", expr, "2 na 2 macierz; wiersz 1; 1, 2, separator wiersza; wiersz 2; 3, 4")?;
    Ok(())
}
