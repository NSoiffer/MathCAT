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
   test_ClearSpeak("nb", "ClearSpeak_MultiLineLabel", "Auto", expr,
    "f av x er lik; 3 tilfeller; \
                tilfelle 1; minus 1 if x er mindre enn 0; \
                tilfelle 2; 0 if x er lik 0; \
                tilfelle 3; 1 if x er større enn 0"
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
   test_ClearSpeak("nb", "ClearSpeak_MultiLineLabel", "Auto", expr,
                "2 rader; \
                rad 1; x pluss y er lik 7; \
                rad 2; 2 x pluss 3 y; er lik 17");
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
   test_ClearSpeak("nb", "ClearSpeak_MultiLineLabel", "Auto", expr, "2 rader; \
                rad 1; x pluss y er lik 7; \
                rad 2; 2 x, pluss 3 y, er lik 17");
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
   test_ClearSpeak("nb", "ClearSpeak_MultiLineLabel", "Case", expr, 
   "2 tilfeller; tilfelle 1; x pluss y er lik 7; tilfelle 2; 2 x pluss 3 y; er lik 17");
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
   test_ClearSpeak("nb", "ClearSpeak_MultiLineLabel", "Constraint", expr, "2 betingelser; \
                betingelse 1; x pluss y er lik 7; \
                betingelse 2; 2 x pluss 3 y; er lik 17");
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
   test_ClearSpeak("nb", "ClearSpeak_MultiLineLabel", "Equation", expr, "2 likninger; \
                likning 1; x pluss y er lik 7; \
                likning 2; 2 x pluss 3 y; er lik 17");
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
   test_ClearSpeak("nb", "ClearSpeak_MultiLineLabel", "Line", expr, "2 rader; \
                rad 1; x pluss y er lik 7; \
                rad 2; 2 x pluss 3 y; er lik 17");
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
   test_ClearSpeak("nb", "ClearSpeak_MultiLineLabel", "None", expr,
        "2; x pluss y er lik 7; 2 x pluss 3 y; er lik 17");
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
   test_ClearSpeak("nb", "ClearSpeak_MultiLineLabel", "Row", expr, "2 rader; \
                rad 1; x pluss y er lik 7; \
                rad 2; 2 x pluss 3 y; er lik 17");
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
   test_ClearSpeak("nb", "ClearSpeak_MultiLineLabel", "Step", expr, "2 steg; \
                steg 1; x pluss y er lik 7; \
                steg 2; 2 x pluss 3 y; er lik 17");
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
test("nb", "SimpleSpeak", expr,
     "2 likninger; likning 1; x er lik y pluss 1; likning 2; y er lik 1");
}
