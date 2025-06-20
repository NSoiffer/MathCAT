use crate::common::*;

#[test]
fn case_1() {
  let expr = "<math>
    <mi>f</mi>
    <mrow>
      <mo>(</mo>
      <mi>x</mi>
      <mo>)</mo>
    </mrow>
    <mo>=</mo>
    <mrow>
      <mo stretchy='true'>{</mo>
      <mtable>
        <mtr><mtd><mo>-</mo><mn>1</mn></mtd><mtd><mtext>if</mtext></mtd><mtd><mi>x</mi><mo>&lt;</mo><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>0</mn></mtd><mtd><mtext>if</mtext></mtd><mtd><mi>x</mi><mo>=</mo><mn>0</mn></mtd></mtr>
        <mtr><mtd><mn>1</mn></mtd><mtd><mtext>if</mtext></mtd><mtd><mi>x</mi><mo>&gt;</mo><mn>0</mn></mtd></mtr>
      </mtable>
    </mrow>
  </math>
   ";
   test_ClearSpeak("en", "ClearSpeak_MultiLineLabel", "Auto", expr,
    "f of x is equal to; 3 cases, \
                case 1; negative 1 if x is less than 0; \
                case 2; 0 if x is equal to 0; \
                case 3; 1 if x is greater than 0"
    )
}

#[test]
fn equation_auto() {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("en", "ClearSpeak_MultiLineLabel", "Auto", expr,
                "2 lines, \
                line 1; x plus y, is equal to 7; \
                line 2; 2 x plus 3 y; is equal to 17");
}


#[test]
fn equation_plus_at_start() {
  let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mi>x</mi></mtd><mtd><mo>+</mo><mi>y</mi> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mn>2</mn><mi>x</mi></mtd><mtd><mo>+</mo><mn>3</mn><mi>y</mi></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("en", "ClearSpeak_MultiLineLabel", "Auto", expr, "2 lines, \
                line 1; x plus y is equal to 7; \
                line 2; 2 x, plus 3 y, is equal to 17");
}

#[test]
fn equation_case() {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("en", "ClearSpeak_MultiLineLabel", "Case", expr, 
   "2 cases, case 1; x plus y, is equal to 7; case 2; 2 x plus 3 y; is equal to 17");
}

#[test]
fn equation_constraint() {
  let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("en", "ClearSpeak_MultiLineLabel", "Constraint", expr, "2 constraints, \
                constraint 1; x plus y, is equal to 7; \
                constraint 2; 2 x plus 3 y; is equal to 17");
}

#[test]
fn equation_equation() {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("en", "ClearSpeak_MultiLineLabel", "Equation", expr, "2 equations, \
                equation 1; x plus y, is equal to 7; \
                equation 2; 2 x plus 3 y; is equal to 17");
}

#[test]
fn equation_line() {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("en", "ClearSpeak_MultiLineLabel", "Line", expr, "2 lines, \
                line 1; x plus y, is equal to 7; \
                line 2; 2 x plus 3 y; is equal to 17");
}

#[test]
fn equation_none() {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("en", "ClearSpeak_MultiLineLabel", "None", expr,
        "2 lines; \
                x plus y, is equal to 7; \
                2 x plus 3 y; is equal to 17");
}

#[test]
fn equation_row() {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("en", "ClearSpeak_MultiLineLabel", "Row", expr, "2 rows, \
                row 1; x plus y, is equal to 7; \
                row 2; 2 x plus 3 y; is equal to 17");
}

#[test]
fn equation_step() {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>
   ";
   test_ClearSpeak("en", "ClearSpeak_MultiLineLabel", "Step", expr, "2 steps, \
                step 1; x plus y, is equal to 7; \
                step 2; 2 x plus 3 y; is equal to 17");
}

#[test]
fn continued_row() {
  let expr = "<math>
  <mtable intent=':system-of-equations'>
   <mtr><mtd><mi>x</mi></mtd><mtd><mo>=</mo></mtd><mtd><mi>y</mi></mtd></mtr>
   <mtr intent=':continued-row'><mtd/><mtd/><mtd><mrow><mo>+</mo><mn>1</mn></mrow></mtd></mtr>
   <mtr><mtd><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>1</mn></mtd></mtr>
  </mtable>
</math>";
test("en", "SimpleSpeak", expr,
     "2 equations, equation 1; x is equal to y plus 1; equation 2; y is equal to 1");
}
