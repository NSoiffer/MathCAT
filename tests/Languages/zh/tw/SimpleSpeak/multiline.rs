use crate::common::*;

#[test]
fn case_1() {
    // init_logger();
    let expr = "<math>
            <mrow>
            <mi>f</mi><mrow><mo>(</mo>
            <mi>x</mi>
            <mo>)</mo></mrow><mo>=</mo><mrow><mo>{</mo> <mrow>
            <mtable>
            <mtr>
                <mtd>
                <mrow>
                <mo>&#x2212;</mo><mn>1</mn><mtext>&#x00A0;若&#x00A0;</mtext><mi>x</mi><mo>&#x003C;</mo><mn>0</mn></mrow>
                </mtd>
            </mtr>
            <mtr>
                <mtd>
                <mrow>
                <mn>0</mn><mtext>&#x00A0;若&#x00A0;</mtext><mi>x</mi><mo>=</mo><mn>0</mn></mrow>
                </mtd>
            </mtr>
            <mtr>
                <mtd>
                <mrow>
                <mn>1</mn><mtext>&#x00A0;若&#x00A0;</mtext><mi>x</mi><mo>&#x003E;</mo><mn>0</mn></mrow>
                </mtd>
            </mtr>
            </mtable></mrow> </mrow></mrow>
        </math>
   ";
    test("zh-tw", "SimpleSpeak", expr, "f x 等於; 3 情況; \
                情況 1; 負 1 若 x, 小於 0; \
                情況 2; 0 若 x, 等於 0; \
                情況 3; 1 若 x, 大於 0");
}

#[test]
fn equation_1() {
    // init_logger();
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
    test("zh-tw", "SimpleSpeak", expr, "2 方程; \
                方程 1; x 加 y 等於 7; \
                方程 2; 2 x 加 3 y; 等於 17");
}
