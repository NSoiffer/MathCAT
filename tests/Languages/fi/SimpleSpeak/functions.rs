/// Tests for:
/// *  functions including trig functions, logs, and functions to powers
/// *  implied times/functional call and explicit times/function call
/// *  parens
/// These are all intertwined, so they are in one file
use crate::common::*;

#[test]
fn trig_names() {
    let expr = "<math><mrow>
    <mi>sin</mi><mi>x</mi><mo>+</mo>
    <mi>cos</mi><mi>y</mi><mo>+</mo>
    <mi>tan</mi><mi>z</mi><mo>+</mo>
    <mi>sec</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csc</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>cot</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("fi", "SimpleSpeak", expr, "sini arvolla x, plus kosini arvolla y, plus tangentti arvolla z, plus sekantti arvolla alfa, plus, kosekantti arvolla suora fii, plus kotangentti arvolla fii");
}

#[test]
fn hyperbolic_trig_names() {
    let expr = "<math><mrow>
    <mi>sinh</mi><mi>x</mi><mo>+</mo>
    <mi>cosh</mi><mi>y</mi><mo>+</mo>
    <mi>tanh</mi><mi>z</mi><mo>+</mo>
    <mi>sech</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csch</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>coth</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("fi", "SimpleSpeak", expr, "hyperbolinen sini arvolla x, plus, hyperbolinen kosini arvolla y, plus, hyperbolinen tangentti, arvolla z, plus, hyperbolinen sekantti, arvolla alfa; plus, hyperbolinen kosekantti, arvolla suora fii; plus, hyperbolinen kotangentti, arvolla fii");
}


#[test]
fn inverse_trig() {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("fi", "SimpleSpeak", expr, "käänteis sini arvolla x");
}

#[test]
fn trig_squared() {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("fi", "SimpleSpeak", expr, "sini toiseen arvolla x");
}

#[test]
fn trig_cubed() {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("fi", "SimpleSpeak", expr, "tangentti kolmanteen arvolla x");
}

#[test]
fn trig_fourth() {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("fi", "SimpleSpeak", expr, "sekantti potenssiin 4, arvolla x");
}


#[test]
fn trig_power_other() {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("fi", "SimpleSpeak", expr, "hyperbolinen sini potenssiin n miinus 1; arvolla x");
}

#[test]
fn simple_log() {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test_prefs("fi", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "log x");
    test_prefs("fi", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "log arvolla x");
    test_prefs("fi", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "log arvolla x");
}

#[test]
fn normal_log() {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("fi", "SimpleSpeak", expr, "log arvolla, auki sulku x plus y, kiinni sulku");
}

#[test]
fn simple_lg() {
    let expr = "<math> <mrow>  <mi>lg</mi><mi>x</mi></mrow> </math>";
    test_prefs("fi", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "l g x");
    test_prefs("fi", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "l g arvolla x");
    test_prefs("fi", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "kymmenkantainen logaritmi, arvolla x");
}

#[test]
fn normal_lg() {
    let expr = "<math><mrow><mi>lg</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("fi", "SimpleSpeak", expr, "l g arvolla, auki sulku x plus y, kiinni sulku");
    test_prefs("fi", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "l g, auki x plus y kiinni");
}

#[test]
fn simple_log_with_base() {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("fi", "SimpleSpeak", expr, "log kanta b, arvolla x");
}

#[test]
fn normal_log_with_base() {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("fi", "SimpleSpeak", expr, "log kanta b, arvolla, auki sulku x plus y, kiinni sulku");
}

#[test]
fn simple_ln() {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test("fi", "SimpleSpeak", expr, "l n arvolla x");
}

#[test]
fn normal_ln() {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test_prefs("fi", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n, auki x plus y kiinni");
    test_prefs("fi", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "l n arvolla, auki sulku x plus y, kiinni sulku");
    test_prefs("fi", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "luonnollinen logaritmi, arvolla, auki sulku x plus y, kiinni sulku");
}

#[test]
fn explicit_function_call_with_parens() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("fi", "SimpleSpeak", expr, "t arvolla x");
}


#[test]
fn explicit_times_with_parens() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("fi", "SimpleSpeak", expr, "t kertaa x");
}

#[test]
fn explicit_function_call() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("fi", "SimpleSpeak", expr, "t arvolla x");
}

#[test]
fn explicit_times() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("fi", "SimpleSpeak", expr, "t x");
}


/*
    * Tests for times
    */
#[test]
fn no_times_binomial() {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("fi", "SimpleSpeak", expr, "x y");
}

#[test]
fn times_following_paren() {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("fi", "SimpleSpeak", expr, "2 kertaa 3");
}

#[test]
fn times_preceding_paren() {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("fi", "SimpleSpeak", expr, "2 kertaa 3");
}

#[test]
fn no_times_sqrt() {
    let expr = "<math><mrow>
        <msqrt> <mi>a</mi>  </msqrt>
        <msqrt> <mi>b</mi>  </msqrt>
        <mo>=</mo>
        <msqrt> <mrow>  <mi>a</mi><mi>b</mi></mrow> </msqrt>
        </mrow></math>";
    test("fi", "SimpleSpeak", expr, 
            "neliöjuuri a, neliöjuuri b; on yhtä suuri kuin, neliöjuuri a b loppu juuri");
}

/*
    * Tests for parens
    */
    #[test]
    fn no_parens_number() {
        let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mn>25</mn>
        <mo>)</mo></mrow>
        <mi>x</mi>
        </mrow></math>";
        test("fi", "SimpleSpeak", expr, "25 kertaa x");
    }

    #[test]
    fn no_parens_monomial() {
        let expr = "<math><mrow>
        <mi>b</mi>
        <mrow><mo>(</mo>
        <mrow><mi>x</mi><mi>y</mi></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
        test("fi", "SimpleSpeak", expr, "b x y");
    }

    #[test]
    fn no_parens_negative_number() {
        let expr = "<math><mrow>
        <mn>2</mn><mo>+</mo>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
        test("fi", "SimpleSpeak", expr, "2 plus negatiivinen 2");
    }


    #[test]
    fn no_parens_negative_number_with_var() {
        let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow><mi>x</mi>
        <mo>)</mo></mrow>
        <mo>+</mo><mn>1</mn>
        </mrow></math>";
        test("fi", "SimpleSpeak", expr, "negatiivinen 2 x, plus 1");
    }

    #[test]
    fn parens_superscript() {
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
        test("fi", "SimpleSpeak", expr, "auki sulku 2 x kiinni sulku toiseen");
    }

    #[test]
    fn no_parens_fraction() {
        let expr = "<math><mrow>
        <mn>2</mn>
        <mo>+</mo>
        <mrow>
            <mrow><mo>(</mo>
            <mfrac> <mn>1</mn><mn>2</mn></mfrac>
            <mo>)</mo></mrow></mrow>
    </mrow></math>";
        test("fi", "SimpleSpeak", expr, "2 plus 1 kahdesosa");
    }


    // Tests for the four types of intervals in SimpleSpeak
    #[test]
    fn parens_interval_open_open() {
        let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("fi", "SimpleSpeak",expr, "avoin väli luvusta c lukuun d");
}

#[test]
    fn parens_interval_closed_open() {
        let expr = "<math> 
        <mrow intent='closed-open-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
            <mo>)</mo></mrow>
        </math>";
    test("fi", "SimpleSpeak",expr, "oikealta puoliavoin väli luvusta c lukuun d");
}


#[test]
fn parens_interval_open_closed() {
    let expr = "<math> 
    <mrow intent='open-closed-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("fi", "SimpleSpeak",expr,"vasemmalta puoliavoin väli luvusta c lukuun d");
}


#[test]
fn parens_interval_closed_closed() {
    let expr = "<math> 
        <mrow intent='closed-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
            <mo>]</mo></mrow>
    </math>";
    test("fi", "SimpleSpeak",expr, "suljettu väli luvusta c lukuun d");
}

    #[test]
    fn parens_interval_neg_infinity_open_open() {
        let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("fi", "SimpleSpeak",expr,
    "avoin väli luvusta negatiivinen ääretön lukuun d");
}

    #[test]
    fn parens_interval_neg_infinity_open_closed() {
        let expr = "<math> 
        <mrow intent='open-closed-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("fi", "SimpleSpeak",expr,
    "vasemmalta puoliavoin väli luvusta negatiivinen ääretön lukuun d");
}

