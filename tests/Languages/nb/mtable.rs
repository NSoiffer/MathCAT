use crate::common::*;

#[test]
fn matrix_1x1() {
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
    test("nb", "ClearSpeak",  expr, "1 ganger 1 matrise med element 3");
    test("nb", "SimpleSpeak", expr, "1 ganger 1 matrise med element 3");
}

#[test]
fn determinant_1x1() {
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
    test("nb", "ClearSpeak",  expr, "1 ganger 1 determinant med element 3");
    test("nb", "SimpleSpeak", expr, "1 ganger 1 determinant med element 3");
}


#[test]
fn matrix_1x2() {
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
    test("nb", "ClearSpeak",  expr, "1 ganger 2 rad-matrise; 3, 5");
    test("nb", "SimpleSpeak", expr, "1 ganger 2 rad-matrise; 3, 5");
}


#[test]
fn matrix_1x3() {
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
    test("nb", "ClearSpeak", expr, "1 ganger 3 rad-matrise; minus x, 5, 12");
    test("nb", "SimpleSpeak", expr, "1 ganger 3 rad-matrise; minus x, 5, 12");
}

#[test]
fn matrix_2x1_not_simple() {
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
    test("nb", "ClearSpeak", expr, "2 ganger 1 kolonne-matrise; rad 1; x pluss 1; rad 2; x minus 1");
    test("nb", "SimpleSpeak", expr, "2 ganger 1 kolonne-matrise; rad 1; x pluss 1; rad 2; x minus 1");
}
#[test]
fn matrix_3x1_not_simple() {
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
    test("nb", "SimpleSpeak", expr, "3 ganger 1 kolonne-matrise; \
            rad 1; x; \
            rad 2; a; \
            rad 3; brøk, x over, x pluss 1, slutt brøk");
    test("nb", "ClearSpeak",  expr, "3 ganger 1 kolonne-matrise; \
            rad 1; x; \
            rad 2; a; \
            rad 3; brøken med teller x; og nevner x pluss 1");
}

#[test]
fn determinant_2x2() {
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
    test("nb", "ClearSpeak",  expr, "2 ganger 2 determinant; rad 1; 2, 1; rad 2; 7, 5");
    test("nb", "SimpleSpeak", expr, "2 ganger 2 determinant; rad 1; 2, 1; rad 2; 7, 5");
}

#[test]
fn matrix_2x3() {
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
    test("nb", "ClearSpeak",  expr, "2 ganger 3 matrise; rad 1; 3, 1, 4; rad 2; 0, 2, 6");
    test("nb", "SimpleSpeak", expr, "2 ganger 3 matrise; rad 1; 3, 1, 4; rad 2; 0, 2, 6");
}

#[test]
fn matrix_2x3_labeled() {
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
    test("nb", "ClearSpeak",  expr,
        "2 ganger 3 matrise; rad 1 med etiketten (3 punktum 1); kolonne 1; 3, kolonne 2; 1, kolonne 3; 4; rad 2; kolonne 1; 0, kolonne 2; 2, kolonne 3; 6");
    test("nb", "SimpleSpeak", expr,
        "2 ganger 3 matrise; rad 1 med etiketten (3 punktum 1); kolonne 1; 3, kolonne 2; 1, kolonne 3; 4; rad 2; kolonne 1; 0, kolonne 2; 2, kolonne 3; 6");
}

#[test]
fn matrix_3x1() {
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
    test("nb", "ClearSpeak",  expr, "3 ganger 1 kolonne-matrise; 1; 2; 3");
    test("nb", "SimpleSpeak", expr, "3 ganger 1 kolonne-matrise; 1; 2; 3");
}

#[test]
fn matrix_4x1() {
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
    test("nb", "ClearSpeak",  expr, "4 ganger 1 kolonne-matrise; rad 1; 3; rad 2; 6; rad 3; 1; rad 4; 2");
    test("nb", "SimpleSpeak", expr, "4 ganger 1 kolonne-matrise; rad 1; 3; rad 2; 6; rad 3; 1; rad 4; 2");
}

#[test]
fn matrix_4x1_labeled() {
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
    test("nb", "ClearSpeak",  expr,
        "4 ganger 1 kolonne-matrise; rad 1; 3; rad 2; 6; rad 3; 1; rad 4 med etiketten (3 punktum 1); 2");
    test("nb", "SimpleSpeak", expr,
        "4 ganger 1 kolonne-matrise; rad 1; 3; rad 2; 6; rad 3; 1; rad 4 med etiketten (3 punktum 1); 2");
}

#[test]
fn matrix_1x4() {
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
    test("nb", "ClearSpeak",  expr, "1 ganger 4 rad-matrise; kolonne 1; 3, kolonne 2; 6, kolonne 3; 1, kolonne 4; 2");
    test("nb", "SimpleSpeak", expr, "1 ganger 4 rad-matrise; kolonne 1; 3, kolonne 2; 6, kolonne 3; 1, kolonne 4; 2");
}

#[test]
fn matrix_4x4() {
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
    test("nb", "ClearSpeak",  expr, "4 ganger 4 matrise; \
          rad 1; kolonne 1; 0, kolonne 2; 3, kolonne 3; 4, kolonne 4; 3; \
          rad 2; kolonne 1; 2, kolonne 2; 1, kolonne 3; 0, kolonne 4; 9; \
          rad 3; kolonne 1; 3, kolonne 2; 0, kolonne 3; 2, kolonne 4; 1; \
          rad 4; kolonne 1; 6, kolonne 2; 2, kolonne 3; 9, kolonne 4; 0");
    test("nb", "SimpleSpeak", expr, "4 ganger 4 matrise; \
          rad 1; kolonne 1; 0, kolonne 2; 3, kolonne 3; 4, kolonne 4; 3; \
          rad 2; kolonne 1; 2, kolonne 2; 1, kolonne 3; 0, kolonne 4; 9; \
          rad 3; kolonne 1; 3, kolonne 2; 0, kolonne 3; 2, kolonne 4; 1; \
          rad 4; kolonne 1; 6, kolonne 2; 2, kolonne 3; 9, kolonne 4; 0");}

#[test]
fn matrix_4x2() {
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
    test("nb", "ClearSpeak",  expr, "4 ganger 2 matrise; \
              rad 1; kolonne 1; 1, kolonne 2; 3; \
              rad 2; kolonne 1; 4, kolonne 2; 2; \
              rad 3; kolonne 1; 2, kolonne 2; 1; \
              rad 4; kolonne 1; 0, kolonne 2; 5\
    ");
    test("nb", "SimpleSpeak", expr, "4 ganger 2 matrise; \
              rad 1; kolonne 1; 1, kolonne 2; 3; \
              rad 2; kolonne 1; 4, kolonne 2; 2; \
              rad 3; kolonne 1; 2, kolonne 2; 1; \
              rad 4; kolonne 1; 0, kolonne 2; 5\
    ");}

// put absolute value test here since it is related to determinate and is small for its own file
#[test]
fn simple_absolute_value() {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("nb", "SimpleSpeak", expr, "absoluttverdien av x");
  test("nb", "ClearSpeak",  expr, "absoluttverdien av x");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "absoluttverdien av x");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "absoluttverdien av x, slutt absoluttverdi");
}
  
#[test]
fn absolute_value_plus_1() {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("nb", "ClearSpeak", expr, "absoluttverdien av x pluss 1");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "absoluttverdien av x pluss 1, slutt absoluttverdi");
}

#[test]
fn simple_cardinality_value() {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "kardinaliteten til stor s");
}
  
// Test preferences
#[test]
fn simple_matrix_speak_col_num() {
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
  test_ClearSpeak("nb", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "2 ganger 2 matrise; rad 1; kolonne 1; 2, kolonne 2; 1; rad 2; kolonne 1; 7, kolonne 2; 5");
}

#[test]
fn col_matrix_3x1_speak_col_num() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "3 ganger 1 kolonne-matrise; rad 1; 1; rad 2; 2; rad 3; 3");
}

#[test]
fn row_matrix_1x2_speak_col_num() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "1 ganger 2 rad-matrise; kolonne 1; 1, kolonne 2; 2");
}

#[test]
fn matrix_2x2_speak_col_num() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "2 ganger 2 matrise; rad 1; kolonne 1; b, senket 1 1; kolonne 2; b, senket 1 2; rad 2; kolonne 1; b, senket 2 1; kolonne 2; b, senket 2 2");
}


#[test]
fn simple_matrix_silent_col_num() {
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
  test_ClearSpeak("nb", "ClearSpeak_Matrix", "SilentColNum",
        expr, "2 ganger 2 matrise; rad 1; 2, 1; rad 2; 7, 5");
}

#[test]
fn col_matrix_3x1_silent_col_num() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "SilentColNum",
        expr, "3 ganger 1 kolonne-matrise; 1; 2; 3");
}

#[test]
fn row_matrix_1x2_silent_col_num() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "SilentColNum",
        expr, "1 ganger 2 rad-matrise; 1, 2");
}

#[test]
fn matrix_2x2_silent_col_num() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "SilentColNum",
        expr, "2 ganger 2 matrise; rad 1; b, senket 1 1; b, senket 1 2; rad 2; b, senket 2 1; b, senket 2 2");
}


#[test]
fn simple_matrix_end_matrix() {
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
  test_ClearSpeak("nb", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 ganger 2 matrise; rad 1; 2, 1; rad 2; 7, 5; slutt matrise");
}

#[test]
fn col_matrix_3x1_end_matrix() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "EndMatrix",
        expr, "3 ganger 1 kolonne-matrise; 1; 2; 3; slutt matrise");
}

#[test]
fn row_matrix_1x2_end_matrix() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "EndMatrix",
        expr, "1 ganger 2 rad-matrise; 1, 2; slutt matrise");
}

#[test]
fn matrix_2x2_end_matrix() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 ganger 2 matrise; rad 1; kolonne 1; b, senket 1 1; kolonne 2; b, senket 1 2; rad 2; kolonne 1; b, senket 2 1; kolonne 2; b, senket 2 2; slutt matrise");
}


#[test]
fn simple_matrix_vector() {
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
  test_ClearSpeak("nb", "ClearSpeak_Matrix", "Vector",
        expr, "2 ganger 2 matrise; rad 1; 2, 1; rad 2; 7, 5");
}

#[test]
fn col_matrix_3x1_vector() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "Vector",
        expr, "3 ganger 1 kolonne-vektor; 1; 2; 3");
}

#[test]
fn row_matrix_1x2_vector() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "Vector",
        expr, "1 ganger 2 rad-vektor; 1, 2");
}

#[test]
fn matrix_2x2_vector() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "Vector",
        expr, "2 ganger 2 matrise; rad 1; kolonne 1; b, senket 1 1; kolonne 2; b, senket 1 2; rad 2; kolonne 1; b, senket 2 1; kolonne 2; b, senket 2 2");
}


#[test]
fn simple_matrix_end_vector() {
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
  test_ClearSpeak("nb", "ClearSpeak_Matrix", "EndVector",
        expr, "2 ganger 2 matrise; rad 1; 2, 1; rad 2; 7, 5; slutt matrise");
}

#[test]
fn col_matrix_3x1_end_vector() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "EndVector",
        expr, "3 ganger 1 kolonne-vektor; 1; 2; 3; slutt vektor");
}

#[test]
fn row_matrix_1x2_end_vector() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "EndVector",
        expr, "1 ganger 2 rad-vektor; 1, 2; slutt vektor");
}

#[test]
fn matrix_2x2_end_vector() {
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
test_ClearSpeak("nb", "ClearSpeak_Matrix", "EndVector",
        expr, "2 ganger 2 matrise; rad 1; kolonne 1; b, senket 1 1; kolonne 2; b, senket 1 2; rad 2; kolonne 1; b, senket 2 1; kolonne 2; b, senket 2 2; slutt matrise");
}



#[test]
fn matrix_binomial() {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("nb", "ClearSpeak_Matrix", "Combinatorics", expr, "3 over 2");
}

#[test]
fn matrix_times() {
  let expr = "<math>
    <mfenced><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable></mfenced>
    <mfenced><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable></mfenced>
  </math>";
  test("nb", "SimpleSpeak", expr,
    "2 ganger 2 matrise; rad 1; 1, 2; rad 2; 3, 4; ganger, 2 ganger 2 matrise; rad 1; a, b; rad 2; c, d");
}

#[test]
fn unknown_mtable_property() {
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
    test("nb", "ClearSpeak",  expr,
         "2 rader; rad 1; a er lik, b pluss c minus d; rad 2; pluss e minus f");
}


#[test]
fn zero_matrix() {
  let expr = "<math>
      <mo>[</mo>
      <mtable>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
      </mtable>
      <mo>]</mo>
  </math>";
  test("nb", "SimpleSpeak", expr,
    "2 ganger 2 null-matrise");
}

#[test]
fn identity_matrix() {
  let expr = "<math>
      <mo>(</mo>
      <mtable>
        <mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr>
      </mtable>
      <mo>)</mo>
  </math>";
  test("nb", "SimpleSpeak", expr,
    "3 ganger 3 identitetsmatrise");
}

#[test]
fn diagonal_matrix() {
  let expr = "<math>
      <mo>(</mo>
      <mtable>
        <mtr><mtd><mn>2</mn></mtd><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd><mtd><msup><mi>x</mi><mn>2</mn></msup></mtd></mtr>
      </mtable>
      <mo>)</mo>
  </math>";
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "3 ganger 3 diagonalmatrise; kolonne 1; 2; kolonne 2; 1; kolonne 3; x i andre");
  // test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Verbose")],
  //     expr, "the 3 by 3 diagonal matrix; row 1, column 1, 2; row 2, column 2, 1; row 3, column 3, x squared");
}
