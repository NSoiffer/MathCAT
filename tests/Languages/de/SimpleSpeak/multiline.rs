use crate::common::*;

#[test]
fn case_1() {
    let expr = "<math>
            <mrow>
            <mi>f</mi><mrow><mo>(</mo>
            <mi>x</mi>
            <mo>)</mo></mrow><mo>=</mo><mrow><mo>{</mo> <mrow>
            <mtable>
            <mtr>
                <mtd>
                <mrow>
                <mo>&#x2212;</mo><mn>1</mn><mtext>&#x00A0;if&#x00A0;</mtext><mi>x</mi><mo>&#x003C;</mo><mn>0</mn></mrow>
                </mtd>
            </mtr>
            <mtr>
                <mtd>
                <mrow>
                <mn>0</mn><mtext>&#x00A0;if&#x00A0;</mtext><mi>x</mi><mo>=</mo><mn>0</mn></mrow>
                </mtd>
            </mtr>
            <mtr>
                <mtd>
                <mrow>
                <mn>1</mn><mtext>&#x00A0;if&#x00A0;</mtext><mi>x</mi><mo>&#x003E;</mo><mn>0</mn></mrow>
                </mtd>
            </mtr>
            </mtable></mrow> </mrow></mrow>
        </math>
   ";
    test("de", "SimpleSpeak", expr, "f of x is equal to; 3 cases; \
                case 1; negative 1 if x; is less than 0; \
                case 2; 0 if x, is equal to 0; \
                case 3; 1 if x, is greater than 0");
}

#[test]
fn equation_1() {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr>
        <mtd>
         <mrow>
          <mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        </mtd>
        <mtd>
         <mo>=</mo>
        </mtd>
        <mtd>
         <mn>7</mn>
        </mtd>
       </mtr>
       <mtr>
        <mtd>
         <mrow>
          <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow>
        </mtd>
        <mtd>
         <mo>=</mo>
        </mtd>
        <mtd>
         <mrow>
          <mn>17</mn></mrow>
        </mtd>
       </mtr>
       
      </mtable></mrow>
    </math>
   ";
    test("de", "SimpleSpeak", expr, "2 equations; \
                equation 1; x plus y, is equal to 7; \
                equation 2; 2 x plus 3 y; is equal to 17");
}
