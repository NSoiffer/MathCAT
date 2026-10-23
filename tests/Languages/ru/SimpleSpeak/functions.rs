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
    test("ru", "SimpleSpeak", expr, "синус икс плюс косинус игрек плюс тангенс зэт плюс секанс альфа, плюс косеканс фи, плюс котангенс фи")?;
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
    test("ru", "SimpleSpeak", expr, "гиперболический синус икс, плюс \
                                гиперболический косинус игрек, плюс \
                                гиперболический тангенс зэт, плюс \
                                гиперболический секанс альфа, плюс \
                                гиперболический косеканс фи, плюс \
                                гиперболический котангенс фи")?;
                                return Ok(());
}

#[test]
fn inverse_trig() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("ru", "SimpleSpeak", expr, "арксинус икс")?;
    return Ok(());
}

#[test]
fn trig_squared() -> Result<()> {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("ru", "SimpleSpeak", expr, "синус в квадрате икс")?;
    return Ok(());
}

#[test]
fn trig_cubed() -> Result<()> {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("ru", "SimpleSpeak", expr, "тангенс в кубе икс")?;
    return Ok(());
}

#[test]
fn trig_fourth() -> Result<()> {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("ru", "SimpleSpeak", expr, "секанс в четвёртой степени икс")?;
    return Ok(());
}

#[test]
fn trig_power_other() -> Result<()> {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("ru", "SimpleSpeak", expr, "гиперболический синус икс в степени эн минус 1")?;
    return Ok(());
}

#[test]
fn simple_log() -> Result<()> {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("ru", "SimpleSpeak", expr, "логарифм икс")?;
    return Ok(());
}

#[test]
fn normal_log() -> Result<()> {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("ru", "SimpleSpeak", expr, "логарифм от, скобка открывается икс плюс игрек, скобка закрывается")?;
    return Ok(());
}

#[test]
fn simple_log_with_base() -> Result<()> {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("ru", "SimpleSpeak", expr, "логарифм по основанию бэ от икс")?;
    return Ok(());
}

#[test]
fn normal_log_with_base() -> Result<()> {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("ru", "SimpleSpeak", expr, "логарифм по основанию бэ от, скобка открывается икс плюс игрек, скобка закрывается")?;
    return Ok(());
}

#[test]
fn normal_ln() -> Result<()> {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "эл эн, открывается икс плюс игрек закрывается")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "натуральный логарифм от, скобка открывается икс плюс игрек, скобка закрывается")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "натуральный логарифм от, скобка открывается икс плюс игрек, скобка закрывается")?;
                return Ok(());
}

#[test]
fn simple_ln() -> Result<()> {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "эл эн икс")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "натуральный логарифм икс")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "натуральный логарифм икс")?;
                return Ok(());
}

#[test]
fn other_names() -> Result<()> {
    let expr = "<math> <mrow><mi>Cov</mi><mi>x</mi></mrow> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "ков икс")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "ковариация икс")?;
    let expr = "<math> <mrow><mi>exp</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow> </math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "эксп от икс")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "экспонента от икс")?;
                return Ok(());
}

#[test]
fn explicit_function_call_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("ru", "SimpleSpeak", expr, "тэ от икс")?;
    return Ok(());
}

#[test]
fn explicit_times_with_parens() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("ru", "SimpleSpeak", expr, "тэ умножить на икс")?;
    return Ok(());
}

#[test]
fn explicit_function_call() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("ru", "SimpleSpeak", expr, "тэ от икс")?;
    return Ok(());
}

#[test]
fn explicit_times() -> Result<()> {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("ru", "SimpleSpeak", expr, "тэ икс")?;
    return Ok(());
}

/*
    * Tests for times
    */
#[test]
fn no_times_binomial() -> Result<()> {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("ru", "SimpleSpeak", expr, "икс игрек")?;
    return Ok(());
}

#[test]
fn times_following_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("ru", "SimpleSpeak", expr, "2 умножить на 3")?;
    return Ok(());
}

#[test]
fn times_preceding_paren() -> Result<()> {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("ru", "SimpleSpeak", expr, "2 умножить на 3")?;
    return Ok(());
}

#[test]
fn no_times_sqrt() -> Result<()> {
    let expr = "<math><mrow>
        <msqrt> <mi>a</mi>  </msqrt>
        <msqrt> <mi>b</mi>  </msqrt>
        <mo>=</mo>
        <msqrt> <mrow>  <mi>a</mi><mi>b</mi></mrow> </msqrt>
        </mrow></math>";
    test("ru", "SimpleSpeak", expr, 
            "квадратный корень из а; умножить на квадратный корень из бэ; равно, квадратный корень из а бэ конец корня")?;
    test_prefs("ru", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "квадратный корень из а; умножить на квадратный корень из бэ; равно, квадратный корень из а бэ")?;
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
    test("ru", "SimpleSpeak", expr, "25 умножить на икс")?;
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
    test("ru", "SimpleSpeak", expr, "бэ, скобка открывается икс игрек скобка закрывается")?;
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
    test("ru", "SimpleSpeak", expr, "2 плюс минус 2")?;
    return Ok(());
}

#[test]
fn no_parens_negative_number_with_var() -> Result<()> {
    let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow><mi>x</mi>
        <mo>)</mo></mrow>
        <mo>+</mo><mn>1</mn>
        </mrow></math>";
    test("ru", "SimpleSpeak", expr, "минус 2 икс, плюс 1")?;
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
    test("ru", "SimpleSpeak", expr, "скобка открывается 2 икс скобка закрывается в квадрате")?;
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
    test("ru", "SimpleSpeak", expr, "2 плюс одна вторая")?;
    return Ok(());
}

// Tests for the four types of intervals in SimpleSpeak
#[test]
fn parens_interval_open_open() -> Result<()> {
    let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("ru", "SimpleSpeak", expr, "интервал от цэ до дэ")?;
    return Ok(());
}

#[test]
fn parens_interval_closed_open() -> Result<()> {
    let expr = "<math> 
        <mrow intent='closed-open-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
            <mo>)</mo></mrow>
        </math>";
    test("ru", "SimpleSpeak", expr, "полуинтервал от цэ до дэ")?;
    return Ok(());
}

#[test]
fn parens_interval_open_closed() -> Result<()> {
    let expr = "<math> 
    <mrow intent='open-closed-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("ru", "SimpleSpeak", expr, "полуинтервал от цэ до дэ")?;
    return Ok(());
}

#[test]
fn parens_interval_closed_closed() -> Result<()> {
    let expr = "<math> 
        <mrow intent='closed-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
            <mo>]</mo></mrow>
    </math>";
    test("ru", "SimpleSpeak", expr, "отрезок от цэ до дэ")?;
    return Ok(());
}

#[test]
fn parens_interval_neg_infinity_open_open() -> Result<()> {
    let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("ru", "SimpleSpeak", expr,
        "интервал от минус бесконечности до дэ")?;
        return Ok(());
}

#[test]
fn parens_interval_neg_infinity_open_closed() -> Result<()> {
    let expr = "<math> 
        <mrow intent='open-closed-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("ru", "SimpleSpeak", expr,
        "полуинтервал от минус бесконечности до дэ")?;
        return Ok(());
}

