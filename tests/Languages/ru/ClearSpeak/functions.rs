/// Tests for:
/// *  functions including trig functions, logs, and functions to powers
/// *  implied times/functional call and explicit times/function call
/// *  parens
/// These are all intertwined, so they are in one file
use crate::common::*;

use anyhow::Result;

#[test]
fn trig_names() -> Result<()> {
    let expr = "<math><mrow>
    <mi>sin</mi><mi>x</mi><mo>+</mo>
    <mi>cos</mi><mi>y</mi><mo>+</mo>
    <mi>tan</mi><mi>z</mi><mo>+</mo>
    <mi>sec</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csc</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>cot</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("ru", "ClearSpeak", expr, "синус икс плюс косинус игрек плюс тангенс зэт плюс секанс альфа, плюс косеканс фи, плюс котангенс фи")?;
    return Ok(());
}

#[test]
fn hyperbolic_trig_names() -> Result<()> {
    let expr = "<math><mrow>
    <mi>sinh</mi><mi>x</mi><mo>+</mo>
    <mi>cosh</mi><mi>y</mi><mo>+</mo>
    <mi>tanh</mi><mi>z</mi><mo>+</mo>
    <mi>sech</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csch</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>coth</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("ru", "ClearSpeak", expr, "гиперболический синус икс, плюс гиперболический косинус игрек, плюс гиперболический тангенс зэт, плюс гиперболический секанс альфа, плюс гиперболический косеканс фи, плюс гиперболический котангенс фи")?;
    return Ok(());
}

#[test]
fn inverse_trig() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("ru", "ClearSpeak", expr, "арксинус икс")?;
    return Ok(());
}

#[test]
fn inverse_trig_trig_inverse() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("ru", "ClearSpeak_Trig", "TrigInverse", expr,
        "обратный тангенс икс")?;
        return Ok(());
}

#[test]
fn inverse_trig_arc() -> Result<()> {
    let expr = "<math><msup><mi>cosh</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("ru", "ClearSpeak_Trig", "ArcTrig", expr,
        "ареакосинус икс")?;
        return Ok(());
}

#[test]
fn inverse_cosine_arc() -> Result<()> {
    // Inverse cosine is arccosine; area cosine belongs to inverse hyperbolic cosine.
    let expr = "<math><msup><mi>cos</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("ru", "ClearSpeak_Trig", "ArcTrig", expr, "арккосинус икс")?;
    return Ok(());
}

#[test]
fn trig_squared() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("ru", "ClearSpeak", expr, "синус в квадрате икс")?;
    return Ok(());
}

#[test]
fn trig_cubed() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("ru", "ClearSpeak", expr, "тангенс в кубе икс")?;
    return Ok(());
}

#[test]
fn trig_fourth() -> Result<()> {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("ru", "ClearSpeak", expr, "секанс в четвёртой степени икс")?;
    return Ok(());
}

#[test]
fn trig_power_other() -> Result<()> {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("ru", "ClearSpeak", expr, "гиперболический синус икс в степени эн минус 1")?;
    return Ok(());
}

#[test]
fn simple_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("ru", "ClearSpeak", expr, "логарифм икс")?;
    return Ok(());
}

#[test]
fn normal_log() -> Result<()> {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("ru", "ClearSpeak", expr, "логарифм от, скобка открывается икс плюс игрек, скобка закрывается")?;
    return Ok(());
}

#[test]
fn simple_log_with_base() -> Result<()> {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("ru", "ClearSpeak", expr, "логарифм икс, по основанию бэ")?;
    return Ok(());
}

#[test]
fn normal_log_with_base() -> Result<()> {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("ru", "ClearSpeak", expr, "логарифм скобка открывается икс плюс игрек, скобка закрывается, по основанию бэ")?;
    return Ok(());
}

#[test]
fn simple_ln() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test("ru", "ClearSpeak", expr, "эл эн икс")?;
    return Ok(());
}

#[test]
fn normal_ln() -> Result<()> {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("ru", "ClearSpeak", expr, "эл эн от, скобка открывается икс плюс игрек, скобка закрывается")?;
    return Ok(());
}

#[test]
fn simple_natural_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_ClearSpeak("ru", "ClearSpeak_Log", "LnAsNaturalLog", expr,
        "натуральный логарифм икс")?;
        return Ok(());
}

#[test]
fn natural_log() -> Result<()> {
    let expr = "<math><mi>ln</mi><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test_ClearSpeak("ru", "ClearSpeak_Log", "LnAsNaturalLog", expr,
        "натуральный логарифм от, скобка открывается икс плюс игрек, скобка закрывается")?;
        return Ok(());
}

#[test]
fn individual_log_mi() -> Result<()> {
    let expr = "<math><mi>log</mi></math>";
    test("ru", "ClearSpeak", expr, "логарифм")?;
    return Ok(());
}

#[test]
fn individual_ln_mi() -> Result<()> {
    let expr = "<math><mi>ln</mi></math>";
    test("ru", "ClearSpeak", expr, "эл эн")?;
    test_ClearSpeak("ru", "ClearSpeak_Log", "LnAsNaturalLog", expr,
        "натуральный логарифм")?;
    return Ok(());
}

#[test]
fn explicit_function_call_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("ru", "ClearSpeak", expr, "тэ от икс")?;
    return Ok(());
}

#[test]
fn explicit_times_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("ru", "ClearSpeak", expr, "тэ умножить на икс")?;
    return Ok(());
}

#[test]
fn explicit_function_call() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("ru", "ClearSpeak", expr, "тэ от икс")?;
    return Ok(());
}

#[test]
fn explicit_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("ru", "ClearSpeak", expr, "тэ икс")?;
    return Ok(());
}

#[test]
fn test_functions_none_pref() -> Result<()> {
    let expr = "<math>
    <mi>log</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
    <mo>+</mo>
    <mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_Functions", "None", expr,
        "логарифм от, скобка открывается икс плюс игрек, скобка закрывается; плюс, эф умножить на, скобка открывается икс плюс игрек, скобка закрывается")?;
        return Ok(());
}

#[test]
fn test_functions_none_pref_multiple_args() -> Result<()> {
    let expr = "<math>
        <mi>B</mi> <mrow><mo>(</mo> <mrow> <mn>2</mn><mo>,</mo><mn>6</mn></mrow> <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_Functions", "None", expr,
        "бэ большое умножить на, скобка открывается 2 запятая 6, скобка закрывается")?;
        return Ok(());
}

/*
    * Tests for times
    */
#[test]
fn no_times_binomial() -> Result<()> {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("ru", "ClearSpeak", expr, "икс игрек")?;
    return Ok(());
}

#[test]
fn times_following_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("ru", "ClearSpeak", expr, "2 умножить на 3")?;
    return Ok(());
}

#[test]
fn times_preceding_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("ru", "ClearSpeak", expr, "2 умножить на 3")?;
    return Ok(());
}

#[test]
fn times_sqrt() -> Result<()> {
    let expr = "<math><mrow>
        <msqrt> <mi>a</mi>  </msqrt>
        <msqrt> <mi>b</mi>  </msqrt>
        <mo>=</mo>
        <msqrt> <mrow>  <mi>a</mi><mi>b</mi></mrow> </msqrt>
        </mrow></math>";
    test("ru", "ClearSpeak", expr, "квадратный корень из а; умножить на квадратный корень из бэ; равно, квадратный корень из а умножить на бэ")?;
    return Ok(());
}

#[test]
fn more_implied_times() -> Result<()> {
    let expr = "<math><mrow>
    <mrow>
    <msup>
        <mrow>
        <mrow><mo>(</mo>
        <mrow> <mn>2</mn><mi>x</mi></mrow>
        <mo>)</mo></mrow></mrow>
        <mn>2</mn>
    </msup>
    </mrow>
    </mrow></math>";
    test_ClearSpeak("ru", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes", expr,
        "скобка открывается 2 умножить на икс скобка закрывается в квадрате")?;
        return Ok(());
}

#[test]
fn explicit_times_more_implied_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test_ClearSpeak("ru", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes", expr, "тэ умножить на икс")?;
    return Ok(());
}

#[test]
fn explicit_times_none_simple_right() -> Result<()> {
    let expr = "<math><mn>2</mn><mo>[</mo><mn>3</mn> <mo>]</mo></math>";
    test_ClearSpeak("ru", "ClearSpeak_ImpliedTimes", "None",
        expr, "2, квадратная скобка открывается 3 квадратная скобка закрывается")?;
        return Ok(());
}

#[test]
fn explicit_times_none_simple_left() -> Result<()> {
    let expr = "<math><mo>(</mo><mn>2</mn><mo>&#x2212;</mo><mn>1</mn><mo>)</mo><mi>x</mi></math>";
    test_ClearSpeak("ru", "ClearSpeak_ImpliedTimes", "None",
        expr, "скобка открывается 2 минус 1, скобка закрывается; икс")?;
        return Ok(());
}

#[test]
fn explicit_times_none_superscript() -> Result<()> {
    let expr = "<math> 
    <mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><msup>
<mi>x</mi>
<mn>2</mn>
</msup>
<mrow><mo>(</mo>
<mrow>
<mi>x</mi><mo>+</mo><mn>1</mn></mrow>
<mo>)</mo></mrow>
    </math>";
    test_ClearSpeak_prefs("ru", 
        vec![("ClearSpeak_ImpliedTimes", "None"), ("ClearSpeak_Functions", "None")],
        expr, "эф скобка открывается икс скобка закрывается; равно; икс в квадрате, скобка открывается икс плюс 1, скобка закрывается")?;
        return Ok(());
}

/*
    * Tests for parens
    */
#[test]
fn no_parens_number() -> Result<()> {
    let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mn>25</mn>
        <mo>)</mo></mrow>
        <mi>x</mi>
        </mrow></math>";
    test("ru", "ClearSpeak", expr, "25 умножить на икс")?;
    return Ok(());
}

#[test]
fn no_parens_monomial() -> Result<()> {
    let expr = "<math><mrow>
        <mi>b</mi>
        <mrow><mo>(</mo>
        <mrow><mi>x</mi><mi>y</mi></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
    test("ru", "ClearSpeak", expr, "бэ, скобка открывается икс игрек скобка закрывается")?;
    return Ok(());
}

#[test]
fn no_parens_negative_number() -> Result<()> {
    let expr = "<math><mrow>
        <mn>2</mn><mo>+</mo>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
    test("ru", "ClearSpeak", expr, "2 плюс минус 2")?;
    return Ok(());
}

#[test]
fn no_parens_negative_number_with_var() -> Result<()> {
    let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow><mi>x</mi>
        <mo>)</mo>
        </mrow>
        <mo>+</mo><mn>1</mn>
        </mrow></math>";
    test("ru", "ClearSpeak", expr, "минус 2 икс, плюс 1")?;
    return Ok(());
}

#[test]
fn parens_superscript() -> Result<()> {
    let expr = "<math><mrow>
        <mrow>
        <msup>
        <mrow>
            <mrow><mo>(</mo>
            <mrow> <mn>2</mn><mi>x</mi></mrow>
            <mo>)</mo></mrow></mrow>
        <mn>2</mn>
        </msup>
        </mrow>
    </mrow></math>";
    test("ru", "ClearSpeak", expr, "скобка открывается 2 икс скобка закрывается в квадрате")?;
    return Ok(());
}

#[test]
fn no_parens_fraction() -> Result<()> {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mo>+</mo>
        <mrow>
            <mrow><mo>(</mo>
            <mfrac> <mn>1</mn><mn>2</mn></mfrac>
            <mo>)</mo></mrow></mrow>
    </mrow></math>";
    test("ru", "ClearSpeak", expr, "2 плюс одна вторая")?;
    return Ok(());
}

// Tests for the ten types of intervals in ClearSpeak
#[test]
fn parens_interval_open_open() -> Result<()> {
    let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_Paren", "Interval", expr,
        "интервал от цэ до дэ, не включая цэ и дэ")?;
        return Ok(());
}

#[test]
fn parens_interval_closed_open() -> Result<()> {
    let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_Paren", "Interval", expr,
        "интервал от цэ до дэ, включая цэ, но не включая дэ")?;
        return Ok(());
}

#[test]
fn parens_interval_open_closed() -> Result<()> {
    let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_Paren", "Interval", expr,
        "интервал от цэ до дэ, не включая цэ, но включая дэ")?;
        return Ok(());
}

#[test]
fn parens_interval_closed_closed() -> Result<()> {
    let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_Paren", "Interval", expr,
        "интервал от цэ до дэ, включая цэ и дэ")?;
        return Ok(());
}

#[test]
fn parens_interval_neg_infinity_open_open() -> Result<()> {
    let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_Paren", "Interval", expr,
        "интервал от минус бесконечности до дэ, не включая дэ")?;
        return Ok(());
}

#[test]
fn parens_interval_neg_infinity_closed_open() -> Result<()> {
    let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow> <mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_Paren", "Interval", expr,
        "интервал от минус бесконечности до дэ, включая дэ")?;
        return Ok(());
}

#[test]
fn parens_interval_open_open_infinity() -> Result<()> {
    let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_Paren", "Interval", expr,
        "интервал от цэ до бесконечности, не включая цэ")?;
        return Ok(());
}

#[test]
fn parens_interval_closed_open_infinity() -> Result<()> {
    let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_Paren", "Interval", expr,
        "интервал от цэ до бесконечности, включая цэ")?;
        return Ok(());
}

#[test]
fn parens_interval_neg_infinity_to_infinity() -> Result<()> {
    let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_Paren", "Interval", expr,
        "интервал от минус бесконечности до бесконечности")?;
        return Ok(());
}

#[test]
fn parens_interval_neg_infinity_to_pos_infinity() -> Result<()> {
    let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mo>+</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("ru", "ClearSpeak_Paren", "Interval", expr,
        "интервал от минус бесконечности до плюс бесконечности")?;
        return Ok(());
}
