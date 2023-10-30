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
    test("en", "ClearSpeak",  expr, "the 1 by 1 matrix with entry 3;");
    test("en", "SimpleSpeak", expr, "the 1 by 1 matrix with entry 3;");
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
    test("en", "ClearSpeak",  expr, "the 1 by 1 determinant with entry 3;");
    test("en", "SimpleSpeak", expr, "the 1 by 1 determinant with entry 3;");
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
    test("en", "ClearSpeak",  expr, "the 1 by 2 row matrix; 3, 5;");
    test("en", "SimpleSpeak", expr, "the 1 by 2 row matrix; 3, 5;");
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
    test("en", "ClearSpeak", expr, "the 1 by 3 row matrix; negative x, 5, 12;");
    test("en", "SimpleSpeak", expr, "the 1 by 3 row matrix; negative x, 5, 12;");
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
    test("en", "ClearSpeak", expr, "the 2 by 1 column matrix; row 1; x plus 1; row 2; x minus 1;");
    test("en", "SimpleSpeak", expr, "the 2 by 1 column matrix; row 1; x plus 1; row 2; x minus 1;");
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
    test("en", "SimpleSpeak", expr, "the 3 by 1 column matrix; \
            row 1; x; \
            row 2; eigh; \
            row 3; fraction, x over, x plus 1, end fraction;");
    test("en", "ClearSpeak",  expr, "the 3 by 1 column matrix; \
            row 1; x; \
            row 2; eigh; \
            row 3; the fraction with numerator x; and denominator x plus 1;");
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
    test("en", "ClearSpeak",  expr, "the 2 by 2 determinant; row 1; 2, 1; row 2; 7, 5;");
    test("en", "SimpleSpeak", expr, "the 2 by 2 determinant; row 1; 2, 1; row 2; 7, 5;");
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
    test("en", "ClearSpeak",  expr, "the 2 by 3 matrix; row 1; 3, 1, 4; row 2; 0, 2, 6;");
    test("en", "SimpleSpeak", expr, "the 2 by 3 matrix; row 1; 3, 1, 4; row 2; 0, 2, 6;");
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
    test("en", "ClearSpeak",  expr,
        "the 2 by 3 matrix; row 1 with label (3.1); column 2; 3, column 3; 1, column 4; 4; \
                                   row 2; column 1; 0, column 2; 2, column 3; 6;");
    test("en", "SimpleSpeak", expr,
        "the 2 by 3 matrix; row 1 with label (3.1); column 2; 3, column 3; 1, column 4; 4; \
                                   row 2; column 1; 0, column 2; 2, column 3; 6;");
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
    test("en", "ClearSpeak",  expr, "the 3 by 1 column matrix; 1; 2; 3;");
    test("en", "SimpleSpeak", expr, "the 3 by 1 column matrix; 1; 2; 3;");
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
    test("en", "ClearSpeak",  expr, "the 4 by 1 column matrix; row 1; 3; row 2; 6; row 3; 1; row 4; 2;");
    test("en", "SimpleSpeak", expr, "the 4 by 1 column matrix; row 1; 3; row 2; 6; row 3; 1; row 4; 2;");
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
    test("en", "ClearSpeak",  expr,
        "the 4 by 1 column matrix; row 1; 3; row 2; 6; row 3; 1; row 4 with label (3.1); 2;");
    test("en", "SimpleSpeak", expr,
        "the 4 by 1 column matrix; row 1; 3; row 2; 6; row 3; 1; row 4 with label (3.1); 2;");
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
    test("en", "ClearSpeak",  expr, "the 1 by 4 row matrix; column 1; 3, column 2; 6, column 3; 1, column 4; 2;");
    test("en", "SimpleSpeak", expr, "the 1 by 4 row matrix; column 1; 3, column 2; 6, column 3; 1, column 4; 2;");
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
    test("en", "ClearSpeak",  expr, "the 4 by 4 matrix; \
          row 1; column 1; 0, column 2; 3, column 3; 4, column 4; 3; \
          row 2; column 1; 2, column 2; 1, column 3; 0, column 4; 9; \
          row 3; column 1; 3, column 2; 0, column 3; 2, column 4; 1; \
          row 4; column 1; 6, column 2; 2, column 3; 9, column 4; 0;");
    test("en", "SimpleSpeak", expr, "the 4 by 4 matrix; \
          row 1; column 1; 0, column 2; 3, column 3; 4, column 4; 3; \
          row 2; column 1; 2, column 2; 1, column 3; 0, column 4; 9; \
          row 3; column 1; 3, column 2; 0, column 3; 2, column 4; 1; \
          row 4; column 1; 6, column 2; 2, column 3; 9, column 4; 0;");}

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
    test("en", "ClearSpeak",  expr, "the 4 by 2 matrix; \
              row 1; column 1; 1, column 2; 3; \
              row 2; column 1; 4, column 2; 2; \
              row 3; column 1; 2, column 2; 1; \
              row 4; column 1; 0, column 2; 5;\
    ");
    test("en", "SimpleSpeak", expr, "the 4 by 2 matrix; \
              row 1; column 1; 1, column 2; 3; \
              row 2; column 1; 4, column 2; 2; \
              row 3; column 1; 2, column 2; 1; \
              row 4; column 1; 0, column 2; 5;\
    ");}

// put absolute value test here since it is related to determinate and is small for its own file
#[test]
fn simple_absolute_value() {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("en", "SimpleSpeak", expr, "the absolute value of x,");
  test("en", "ClearSpeak",  expr, "the absolute value of x,");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "absolute value of x,");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "the absolute value of x, end absolute value,");
}
  
#[test]
fn absolute_value_plus_1() {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("en", "ClearSpeak", expr, "the absolute value of x plus 1,");
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "absolute value of x plus 1, end absolute value,");
}

#[test]
fn simple_cardinality_value() {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "the cardinality of cap s,");
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
  test_ClearSpeak("en", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "the 2 by 2 matrix; row 1; column 1; 2, column 2; 1; row 2; column 1; 7, column 2; 5;");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "the 3 by 1 column matrix; row 1; 1; row 2; 2; row 3; 3;");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "the 1 by 2 row matrix; column 1; 1, column 2; 2;");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "the 2 by 2 matrix; row 1; column 1; b sub 1 1, column 2; b sub 1 2; \
                                                row 2; column 1; b sub 2 1, column 2; b sub 2 2;");
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
  test_ClearSpeak("en", "ClearSpeak_Matrix", "SilentColNum",
        expr, "the 2 by 2 matrix; row 1; 2, 1; row 2; 7, 5;");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "SilentColNum",
        expr, "the 3 by 1 column matrix; 1; 2; 3;");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "SilentColNum",
        expr, "the 1 by 2 row matrix; 1, 2;");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "SilentColNum",
        expr, "the 2 by 2 matrix; row 1; b sub 1 1, b sub 1 2; \
                                                row 2; b sub 2 1, b sub 2 2;");
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
  test_ClearSpeak("en", "ClearSpeak_Matrix", "EndMatrix",
        expr, "the 2 by 2 matrix; row 1; 2, 1; row 2; 7, 5; end matrix");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "EndMatrix",
        expr, "the 3 by 1 column matrix; 1; 2; 3; end matrix");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "EndMatrix",
        expr, "the 1 by 2 row matrix; 1, 2; end matrix");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "EndMatrix",
        expr, "the 2 by 2 matrix; row 1; column 1; b sub 1 1, column 2; b sub 1 2; \
                                                row 2; column 1; b sub 2 1, column 2; b sub 2 2; end matrix");
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
  test_ClearSpeak("en", "ClearSpeak_Matrix", "Vector",
        expr, "the 2 by 2 matrix; row 1; 2, 1; row 2; 7, 5;");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "Vector",
        expr, "the 3 by 1 column vector; 1; 2; 3;");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "Vector",
        expr, "the 1 by 2 row vector; 1, 2;");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "Vector",
        expr, "the 2 by 2 matrix; row 1; column 1; b sub 1 1, column 2; b sub 1 2; \
                                                row 2; column 1; b sub 2 1, column 2; b sub 2 2;");
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
  test_ClearSpeak("en", "ClearSpeak_Matrix", "EndVector",
        expr, "the 2 by 2 matrix; row 1; 2, 1; row 2; 7, 5; end matrix");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "EndVector",
        expr, "the 3 by 1 column vector; 1; 2; 3; end vector");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "EndVector",
        expr, "the 1 by 2 row vector; 1, 2; end vector");
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
test_ClearSpeak("en", "ClearSpeak_Matrix", "EndVector",
        expr, "the 2 by 2 matrix; row 1; column 1; b sub 1 1, column 2; b sub 1 2; \
                                                row 2; column 1; b sub 2 1, column 2; b sub 2 2; end matrix");
}



#[test]
fn matrix_binomial() {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("en", "ClearSpeak_Matrix", "Combinatorics", expr, "3 choose 2");
}
