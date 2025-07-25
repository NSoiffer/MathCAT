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
    test("fi", "ClearSpeak",  expr, "1 kertaa 1 matriisi 3");
    test("fi", "SimpleSpeak", expr, "1 kertaa 1 matriisi 3");
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
    test("fi", "ClearSpeak",  expr, "1 kertaa 1 determinantti 3");
    test("fi", "SimpleSpeak", expr, "1 kertaa 1 determinantti 3");
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
    test("fi", "ClearSpeak",  expr, "1 kertaa 2 rivi matriisi; 3, 5");
    test("fi", "SimpleSpeak", expr, "1 kertaa 2 rivi matriisi; 3, 5");
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
    test("fi", "ClearSpeak", expr, "1 kertaa 3 rivi matriisi; negatiivinen x, 5, 12");
    test("fi", "SimpleSpeak", expr, "1 kertaa 3 rivi matriisi; negatiivinen x, 5, 12");
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
    test("fi", "ClearSpeak", expr, "2 kertaa 1 sarake matriisi; rivi 1; x plus 1; rivi 2; x miinus 1");
    test("fi", "SimpleSpeak", expr, "2 kertaa 1 sarake matriisi; rivi 1; x plus 1; rivi 2; x miinus 1");
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
    test("fi", "SimpleSpeak", expr, "3 kertaa 1 sarake matriisi; rivi 1; x; rivi 2; a; rivi 3; murtoluku, x per, x plus 1, loppu murtoluku");
    test("fi", "ClearSpeak",  expr, "3 kertaa 1 sarake matriisi; rivi 1; x; rivi 2; a; rivi 3; murtoluku osoittaja x; ja nimittäjä x plus 1");
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
    test("fi", "ClearSpeak",  expr, "2 kertaa 2 determinantti; rivi 1; 2, 1; rivi 2; 7, 5");
    test("fi", "SimpleSpeak", expr, "2 kertaa 2 determinantti; rivi 1; 2, 1; rivi 2; 7, 5");
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
    test("fi", "ClearSpeak",  expr, "2 kertaa 3 matriisi; rivi 1; 3, 1, 4; rivi 2; 0, 2, 6");
    test("fi", "SimpleSpeak", expr, "2 kertaa 3 matriisi; rivi 1; 3, 1, 4; rivi 2; 0, 2, 6");
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
    test("fi", "ClearSpeak",  expr,
        "2 kertaa 3 matriisi; rivi 1 merkinnällä (3.1); sarake 1; 3, sarake 2; 1, sarake 3; 4; rivi 2; sarake 1; 0, sarake 2; 2, sarake 3; 6");
    test("fi", "SimpleSpeak", expr,
        "2 kertaa 3 matriisi; rivi 1 merkinnällä (3.1); sarake 1; 3, sarake 2; 1, sarake 3; 4; rivi 2; sarake 1; 0, sarake 2; 2, sarake 3; 6");
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
    test("fi", "ClearSpeak",  expr, "3 kertaa 1 sarake matriisi; 1; 2; 3");
    test("fi", "SimpleSpeak", expr, "3 kertaa 1 sarake matriisi; 1; 2; 3");
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
    test("fi", "ClearSpeak",  expr, "4 kertaa 1 sarake matriisi; rivi 1; 3; rivi 2; 6; rivi 3; 1; rivi 4; 2");
    test("fi", "SimpleSpeak", expr, "4 kertaa 1 sarake matriisi; rivi 1; 3; rivi 2; 6; rivi 3; 1; rivi 4; 2");
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
    test("fi", "ClearSpeak",  expr,
        "4 kertaa 1 sarake matriisi; rivi 1; 3; rivi 2; 6; rivi 3; 1; rivi 4 merkinnällä (3.1); 2");
    test("fi", "SimpleSpeak", expr,
        "4 kertaa 1 sarake matriisi; rivi 1; 3; rivi 2; 6; rivi 3; 1; rivi 4 merkinnällä (3.1); 2");
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
    test("fi", "ClearSpeak",  expr, "1 kertaa 4 rivi matriisi; sarake 1; 3, sarake 2; 6, sarake 3; 1, sarake 4; 2");
    test("fi", "SimpleSpeak", expr, "1 kertaa 4 rivi matriisi; sarake 1; 3, sarake 2; 6, sarake 3; 1, sarake 4; 2");
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
    test("fi", "ClearSpeak",  expr, "4 kertaa 4 matriisi; rivi 1; sarake 1; 0, sarake 2; 3, sarake 3; 4, sarake 4; 3; rivi 2; sarake 1; 2, sarake 2; 1, sarake 3; 0, sarake 4; 9; rivi 3; sarake 1; 3, sarake 2; 0, sarake 3; 2, sarake 4; 1; rivi 4; sarake 1; 6, sarake 2; 2, sarake 3; 9, sarake 4; 0");
    test("fi", "SimpleSpeak", expr, "4 kertaa 4 matriisi; rivi 1; sarake 1; 0, sarake 2; 3, sarake 3; 4, sarake 4; 3; rivi 2; sarake 1; 2, sarake 2; 1, sarake 3; 0, sarake 4; 9; rivi 3; sarake 1; 3, sarake 2; 0, sarake 3; 2, sarake 4; 1; rivi 4; sarake 1; 6, sarake 2; 2, sarake 3; 9, sarake 4; 0");}

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
    test("fi", "ClearSpeak",  expr, "4 kertaa 2 matriisi; rivi 1; sarake 1; 1, sarake 2; 3; rivi 2; sarake 1; 4, sarake 2; 2; rivi 3; sarake 1; 2, sarake 2; 1; rivi 4; sarake 1; 0, sarake 2; 5");
    test("fi", "SimpleSpeak", expr, "4 kertaa 2 matriisi; rivi 1; sarake 1; 1, sarake 2; 3; rivi 2; sarake 1; 4, sarake 2; 2; rivi 3; sarake 1; 2, sarake 2; 1; rivi 4; sarake 1; 0, sarake 2; 5");}

// put absolute value test here since it is related to determinate and is small for its own file
#[test]
fn simple_absolute_value() {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>x</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test("fi", "SimpleSpeak", expr, "itseisarvo x");
  test("fi", "ClearSpeak",  expr, "itseisarvo x");
  test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "Auto")], expr, "itseisarvo x");
  test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "itseisarvo x, loppu itseisarvo");
}
  
#[test]
fn absolute_value_plus_1() {
let expr = "<math>
    <mrow><mrow><mo>|</mo>
      <mrow><mi>x</mi><mo>+</mo><mn>1</mn> </mrow>
    <mo>|</mo></mrow></mrow>
  </math>";
  test("fi", "ClearSpeak", expr, "itseisarvo x plus 1");
  test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Terse"), ("ClearSpeak_AbsoluteValue", "AbsEnd")],
             expr, "itseisarvo x plus 1, loppu itseisarvo");
}

#[test]
fn simple_cardinality_value() {
  let expr = "<math>
    <mrow><mrow><mo>|</mo> <mi>S</mi> <mo>|</mo></mrow></mrow>
  </math>";
  test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_AbsoluteValue", "Cardinality")], expr,
             "kardinaliteetti iso s");
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
  test_ClearSpeak("fi", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "2 kertaa 2 matriisi; rivi 1; sarake 1; 2, sarake 2; 1; rivi 2; sarake 1; 7, sarake 2; 5");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "3 kertaa 1 sarake matriisi; rivi 1; 1; rivi 2; 2; rivi 3; 3");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "1 kertaa 2 rivi matriisi; sarake 1; 1, sarake 2; 2");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "SpeakColNum",
        expr, "2 kertaa 2 matriisi; rivi 1; sarake 1; b ala 1 1, sarake 2; b ala 1 2; rivi 2; sarake 1; b ala 2 1, sarake 2; b ala 2 2");
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
  test_ClearSpeak("fi", "ClearSpeak_Matrix", "SilentColNum",
        expr, "2 kertaa 2 matriisi; rivi 1; 2, 1; rivi 2; 7, 5");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "SilentColNum",
        expr, "3 kertaa 1 sarake matriisi; 1; 2; 3");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "SilentColNum",
        expr, "1 kertaa 2 rivi matriisi; 1, 2");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "SilentColNum",
        expr, "2 kertaa 2 matriisi; rivi 1; b ala 1 1, b ala 1 2; rivi 2; b ala 2 1, b ala 2 2");
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
  test_ClearSpeak("fi", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 kertaa 2 matriisi; rivi 1; 2, 1; rivi 2; 7, 5; loppu matriisi");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "EndMatrix",
        expr, "3 kertaa 1 sarake matriisi; 1; 2; 3; loppu matriisi");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "EndMatrix",
        expr, "1 kertaa 2 rivi matriisi; 1, 2; loppu matriisi");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 kertaa 2 matriisi; rivi 1; sarake 1; b ala 1 1, sarake 2; b ala 1 2; rivi 2; sarake 1; b ala 2 1, sarake 2; b ala 2 2; loppu matriisi");
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
  test_ClearSpeak("fi", "ClearSpeak_Matrix", "Vector",
        expr, "2 kertaa 2 matriisi; rivi 1; 2, 1; rivi 2; 7, 5");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "Vector",
        expr, "3 kertaa 1 sarake vektori; 1; 2; 3");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "Vector",
        expr, "1 kertaa 2 rivi vektori; 1, 2");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "Vector",
        expr, "2 kertaa 2 matriisi; rivi 1; sarake 1; b ala 1 1, sarake 2; b ala 1 2; rivi 2; sarake 1; b ala 2 1, sarake 2; b ala 2 2");
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
  test_ClearSpeak("fi", "ClearSpeak_Matrix", "EndVector",
        expr, "2 kertaa 2 matriisi; rivi 1; 2, 1; rivi 2; 7, 5; loppu matriisi");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "EndVector",
        expr, "3 kertaa 1 sarake vektori; 1; 2; 3; loppu vektori");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "EndVector",
        expr, "1 kertaa 2 rivi vektori; 1, 2; loppu vektori");
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
test_ClearSpeak("fi", "ClearSpeak_Matrix", "EndVector",
        expr, "2 kertaa 2 matriisi; rivi 1; sarake 1; b ala 1 1, sarake 2; b ala 1 2; rivi 2; sarake 1; b ala 2 1, sarake 2; b ala 2 2; loppu matriisi");
}



#[test]
fn matrix_binomial() {
  let expr = "<math>
      <mo>(</mo><mrow>
        <mtable><mtr><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable>
      </mrow><mo>)</mo>
    </math>";
  test_ClearSpeak("fi", "ClearSpeak_Matrix", "Combinatorics", expr, "3 yli 2");
}
