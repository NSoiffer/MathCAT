use crate::common::*;
use anyhow::Result;

#[test]
fn multiplication() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn><mo>×</mo><mn>3</mn>
                </math>";
    test("en", "ClearSpeak", expr, "2 times 3")?;
    return Ok(());

}

#[test]
fn multiplication_by() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn><mo>×</mo><mn>3</mn>
                </math>";
    test_ClearSpeak("en", "ClearSpeak_MultSymbolX", "By", expr, "2 by 3")?;
    return Ok(());

}

#[test]
fn multiplication_cross() -> Result<()> {
    let expr = "<math>
                    <mi>u</mi><mo>×</mo><mi>v</mi>
                </math>";
    test_ClearSpeak("en", "ClearSpeak_MultSymbolX", "Cross", expr, "u cross v")?;
    return Ok(());

}

#[test]
fn ellipses_auto_start() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn>
        </math>";
    test("en", "ClearSpeak", expr, "dot dot dot comma, negative 2 comma, negative 1 comma, 0")?;
    return Ok(());

}

#[test]
fn ellipses_auto_end() -> Result<()> {
    let expr = "<math>
            <mn>1</mn>
            <mo>,</mo>
            <mn>2</mn>
            <mo>,</mo>
            <mn>3</mn>
            <mo>,</mo>
            <mi>…</mi>
        </math>";
    test_ClearSpeak("en", "ClearSpeak_Ellipses", "Auto", expr, "1 comma, 2 comma, 3 comma, dot dot dot")?;
    return Ok(());

}

#[test]
fn ellipses_auto_middle() -> Result<()> {
    let expr = "<math>
            <mrow>
                <mn>1</mn>
                <mo>,</mo>
                <mn>2</mn>
                <mo>,</mo>
                <mn>3</mn>
                <mo>,</mo>
                <mi>…</mi>
                <mo>,</mo>
                <mn>20</mn>
            </mrow>
        </math>";
    test_ClearSpeak("en", "ClearSpeak_Ellipses", "Auto", expr,
            "1 comma, 2 comma, 3 comma, dot dot dot comma, 20")?;
            return Ok(());

}

#[test]
fn ellipses_auto_both() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn>
            <mo>,</mo><mi>…</mi>
       </math>";
    test_ClearSpeak("en", "ClearSpeak_Ellipses", "Auto", expr,
            "dot dot dot comma, negative 2 comma, negative 1 comma, 0 comma, 1 comma, 2 comma, dot dot dot")?;
            return Ok(());

}

#[test]
fn ellipses_and_so_on_start() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn>
        </math>";
        test_ClearSpeak("en", "ClearSpeak_Ellipses", "AndSoOn", expr, "dot dot dot comma, negative 2 comma, negative 1 comma, 0")?;
        return Ok(());

}

#[test]
fn ellipses_and_so_on_end() -> Result<()> {
    let expr = "<math>
            <mn>1</mn>
            <mo>,</mo>
            <mn>2</mn>
            <mo>,</mo>
            <mn>3</mn>
            <mo>,</mo>
            <mi>…</mi>
        </math>";
    test_ClearSpeak("en", "ClearSpeak_Ellipses", "AndSoOn", expr, "1 comma, 2 comma, 3 and so on")?;
    return Ok(());

}

#[test]
fn ellipses_and_so_on_middle() -> Result<()> {
    let expr = "<math>
            <mrow>
                <mn>1</mn>
                <mo>,</mo>
                <mn>2</mn>
                <mo>,</mo>
                <mn>3</mn>
                <mo>,</mo>
                <mi>…</mi>
                <mo>,</mo>
                <mn>20</mn>
            </mrow>
        </math>";
    test_ClearSpeak("en", "ClearSpeak_Ellipses", "AndSoOn", expr,
            "1 comma, 2 comma, 3 and so on up to 20")?;
            return Ok(());

}

#[test]
fn ellipses_and_so_on_both() -> Result<()> {
    let expr = "<math>
            <mi>…</mi><mo>,</mo>
            <mo>-</mo><mn>2</mn><mo>,</mo><mo>-</mo><mn>1</mn><mo>,</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>2</mn>
            <mo>,</mo><mi>…</mi>
       </math>";
    test_ClearSpeak("en", "ClearSpeak_Ellipses", "AndSoOn", expr,
            "dot dot dot comma, negative 2 comma, negative 1 comma, 0 comma, 1 comma, 2 comma, dot dot dot")?;
            return Ok(());

}

#[test]
fn vertical_line_auto() -> Result<()> {
    let expr = "<math>
        <mn>3</mn><mo>|</mo><mn>6</mn>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_VerticalLine", "Auto", expr,
            "3 divides 6")?;
            return Ok(());

}

#[test]
fn vertical_line_divides() -> Result<()> {
    let expr = "<math>
        <mn>3</mn><mo>|</mo><mn>6</mn>
    </math>";
    test_ClearSpeak("en", "ClearSpeak_VerticalLine", "Divides", expr,
            "3 divides 6")?;
            return Ok(());

}

    #[test]
    fn vertical_line_given() -> Result<()> {
        let expr = "<math>
            <mn>3</mn><mo>|</mo><mn>6</mn>
        </math>";
        test_ClearSpeak("en", "ClearSpeak_VerticalLine", "Given", expr,
                "3 given 6")?;
                return Ok(());

    }

    #[test]
    fn vertical_line_probability_given() -> Result<()> {
        let expr = "<math>
                <mi>P</mi>
                <mrow>
                    <mo>(</mo>
                    <mrow>
                        <mi>A</mi>
                        <mo>|</mo>
                        <mi>B</mi>
                    </mrow>
                    <mo>)</mo>
                </mrow>
            </math>";
        test_ClearSpeak_prefs("en", vec![("ClearSpeak_VerticalLine", "Given"), ("ClearSpeak_ImpliedTimes", "None")]
                        , expr, "cap p, open paren, cap eigh given cap b, close paren")?;
                        return Ok(());
    }

#[test]
fn vertical_line_set() -> Result<()> {
    let expr = "<math>
        <mo>{</mo>
        <mrow>
            <mi>x</mi>
            <mo>|</mo>
            <mi>x</mi>
            <mo>&gt;</mo>
            <mn>0</mn>
        </mrow>
        <mo>}</mo>    
    </math>";
    test_ClearSpeak("en", "ClearSpeak_VerticalLine", "Auto", expr,
            "the set of all x such that x is greater than 0")?;
            return Ok(());

}


#[test]
fn vertical_line_set_such_that() -> Result<()> {
    let expr = "<math>
        <mo>{</mo>
        <mrow>
            <mi>x</mi>
            <mo>|</mo>
            <mi>x</mi>
            <mo>&gt;</mo>
            <mn>0</mn>
        </mrow>
        <mo>}</mo>    
    </math>";
    test_ClearSpeak("en", "ClearSpeak_VerticalLine", "SuchThat", expr,
            "the set of all x such that x is greater than 0")?;
            return Ok(());

}

#[test]
fn vertical_line_set_given() -> Result<()> {
    let expr = "<math>
        <mo>{</mo>
        <mrow>
            <mi>x</mi>
            <mo>|</mo>
            <mi>x</mi>
            <mo>&gt;</mo>
            <mn>0</mn>
        </mrow>
        <mo>}</mo>    
    </math>";
    // the rules for set will override all the options -- ClearSpeak spec should be clarified
    test_ClearSpeak("en", "ClearSpeak_VerticalLine", "Given", expr,
            "the set of all x such that x is greater than 0")?;
            return Ok(());

}

#[test]
fn vertical_line_set_and_abs() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mrow>
                <mi>x</mi>
                <mo>&#x007C;</mo>
                <mrow>
                    <mo>|</mo>
                    <mi>x</mi>
                    <mo>|</mo>
                </mrow>
                <mo>&gt;</mo>
                <mn>2</mn>
            </mrow>
            <mo>}</mo>
        </math>";
    test_ClearSpeak("en", "ClearSpeak_VerticalLine", "Auto", expr,
        "the set of all x such that the absolute value of x; is greater than 2")?;
        return Ok(());

}

#[test]
fn vertical_line_evaluated_at() -> Result<()> {
    let expr = "<math>
            <mi>f</mi>
            <mrow>
                <mo>(</mo>
                <mi>x</mi>
                <mo>)</mo>
            </mrow>
            <msub>
                <mo>&#x007C;</mo>
                <mrow>
                    <mi>x</mi>
                    <mo>=</mo>
                    <mn>5</mn>
                </mrow>
            </msub>
        </math>";
    test_ClearSpeak("en", "ClearSpeak_VerticalLine", "Auto", expr,
        "f of x evaluated at, x is equal to 5")?;
        return Ok(());

}

#[test]
fn vertical_line_evaluated_at_both() -> Result<()> {
    let expr = "<math>
            <msup>
            <mi>x</mi>
            <mn>2</mn>
            </msup>
            <mo>+</mo>
            <mi>x</mi>
            <msubsup>
                <mstyle mathsize='140%' displaystyle='true'> <mo>&#x007C;</mo> </mstyle>
                <mn>0</mn>
                <mn>1</mn>
            </msubsup>
        </math>";
    test_ClearSpeak("en", "ClearSpeak_VerticalLine", "Auto", expr,
        "x squared plus x, evaluated at 1 minus the same expression evaluated at 0")?;
        return Ok(());

}
#[test]
fn vertical_line_evaluated_at_divides() -> Result<()> {
    let expr = "<math>
            <mi>f</mi>
            <mrow>
                <mo>(</mo>
                <mi>x</mi>
                <mo>)</mo>
            </mrow>
            <msub>
                <mo>&#x007C;</mo>
                <mrow>
                    <mi>x</mi>
                    <mo>=</mo>
                    <mn>5</mn>
                </mrow>
            </msub>
        </math>";
    test_ClearSpeak("en", "ClearSpeak_VerticalLine", "Divides", expr,
        "f of x evaluated at, x is equal to 5")?;
        return Ok(());

}

#[test]
fn vertical_line_evaluated_at_both_given() -> Result<()> {
    let expr = "<math>
            <msup>
            <mi>x</mi>
            <mn>2</mn>
            </msup>
            <mo>+</mo>
            <mi>x</mi>
            <msubsup>
                <mstyle mathsize='140%' displaystyle='true'> <mo>&#x007C;</mo> </mstyle>
                <mn>0</mn>
                <mn>1</mn>
            </msubsup>
        </math>";
    test_ClearSpeak("en", "ClearSpeak_VerticalLine", "Given", expr,
        "x squared plus x, evaluated at 1 minus the same expression evaluated at 0")?;
        return Ok(());

}

#[test]
fn double_factorial_test() -> Result<()> {
    let expr1 = "<math><mn>5</mn><mo>!!</mo></math>";
    let expr2 = "<math><mn>5</mn><mo>‼</mo></math>";
    test("en", "ClearSpeak", expr2, "5 double factorial")?;
    test("en", "ClearSpeak", expr1, "5 double factorial")?;
    test("en", "SimpleSpeak", expr1, "5 double factorial")?;
    Ok(())
}
