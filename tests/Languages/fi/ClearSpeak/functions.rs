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
    test("fi", "ClearSpeak", expr, "sini x plus kosini y plus tangentti z, plus sekantti alfa, plus kosekantti suora fii, plus kotangentti fii");
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
    test("fi", "ClearSpeak", expr, "hyperbolinen sini arvolla x, plus, hyperbolinen kosini arvolla y, plus, hyperbolinen tangentti, arvolla z, plus, hyperbolinen sekantti, arvolla alfa; plus, hyperbolinen kosekantti, arvolla suora fii; plus, hyperbolinen kotangentti, arvolla fii");
}


#[test]
fn inverse_trig() {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("fi", "ClearSpeak", expr, "käänteis sini arvolla x");
}

#[test]
fn inverse_trig_trig_inverse() {
    let expr = "<math><msup><mi>tan</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("fi", "ClearSpeak_Trig", "TrigInverse",expr,
        "käänteis tangentti arvolla x");
}

#[test]
fn inverse_trig_arc() {
    let expr = "<math><msup><mi>cosh</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test_ClearSpeak("fi", "ClearSpeak_Trig", "ArcTrig",expr,
        "arkus hyperbolinen kosini, arvolla x");
}

#[test]
fn trig_squared() {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("fi", "ClearSpeak", expr, "sini toiseen arvolla x");
}

#[test]
fn trig_cubed() {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("fi", "ClearSpeak", expr, "tangentti kolmanteen arvolla x");
}

#[test]
fn trig_fourth() {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("fi", "ClearSpeak", expr, "sekantti potenssiin 4, arvolla x");
}


#[test]
fn trig_power_other() {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("fi", "ClearSpeak", expr, "hyperbolinen sini potenssiin n miinus 1; arvolla x");
}

#[test]
fn simple_log() {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("fi", "ClearSpeak", expr, "log x");
}

#[test]
fn normal_log() {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("fi", "ClearSpeak", expr, "log arvolla, auki sulku x plus y, kiinni sulku");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Verbose")], expr,
            "logaritmi arvolla, auki sulku x plus y, kiinni sulku");
}

#[test]
fn simple_log_with_base() {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("fi", "ClearSpeak", expr, "log kanta b, arvolla x");
    
}

#[test]
fn normal_log_with_base() {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("fi", "ClearSpeak", expr, "log kanta b, arvolla, auki sulku x plus y, kiinni sulku");
}

#[test]
fn simple_ln() {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test("fi", "ClearSpeak", expr, "l n x");
}

#[test]
fn normal_ln() {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("fi", "ClearSpeak", expr, "l n arvolla, auki sulku x plus y, kiinni sulku");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Verbose")], expr,
            "luonnollinen logaritmi, arvolla, auki sulku x plus y, kiinni sulku");
}

#[test]
fn simple_lg() {
    let expr = "<math> <mrow>  <mi>lg</mi><mi>x</mi></mrow> </math>";
    test("fi", "ClearSpeak", expr, "l g x");
}

#[test]
fn normal_lg() {
    let expr = "<math><mrow><mi>lg</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("fi", "ClearSpeak", expr, "l g arvolla, auki sulku x plus y, kiinni sulku");
    test_prefs("fi", "ClearSpeak", vec![("Verbosity", "Verbose")], expr,
            "kymmenkantainen logaritmi, arvolla, auki sulku x plus y, kiinni sulku");
}
    
#[test]
fn simple_natural_log() {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_ClearSpeak("fi", "ClearSpeak_Log", "LnAsNaturalLog",expr,
        "luonnollinen log x");
}

    
#[test]
fn natural_log() {
    let expr = "<math><mi>ln</mi><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></math>";
    test_ClearSpeak("fi", "ClearSpeak_Log", "LnAsNaturalLog",expr,
        "luonnollinen logaritmi, arvolla, auki sulku x plus y, kiinni sulku");
        
}


#[test]
fn explicit_function_call_with_parens() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("fi", "ClearSpeak", expr, "t arvolla x");
}


#[test]
fn explicit_times_with_parens() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("fi", "ClearSpeak", expr, "t kertaa x");
}

#[test]
fn explicit_function_call() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("fi", "ClearSpeak", expr, "t arvolla x");
}

#[test]
fn explicit_times() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("fi", "ClearSpeak", expr, "t x");
}


#[test]
fn test_functions_none_pref() {
    let expr = "<math>
    <mi>log</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
    <mo>+</mo>
    <mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fi", "ClearSpeak_Functions", "None",expr,
        "log arvolla, auki sulku x plus y, kiinni sulku; plus, f kertaa, auki sulku x plus y, kiinni sulku");
}

#[test]
fn test_functions_none_pref_multiple_args() {
    let expr = "<math>
        <mi>B</mi> <mrow><mo>(</mo> <mrow> <mn>2</mn><mo>,</mo><mn>6</mn></mrow> <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fi", "ClearSpeak_Functions", "None",expr,
        "iso b kertaa, auki sulku 2 pilkku, 6, kiinni sulku");
}


/*
    * Tests for times
    */
#[test]
fn no_times_binomial() {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("fi", "ClearSpeak", expr, "x y");
}

#[test]
fn times_following_paren() {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("fi", "ClearSpeak", expr, "2 kertaa 3");
}

#[test]
fn times_preceding_paren() {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("fi", "ClearSpeak", expr, "2 kertaa 3");
}

#[test]
fn no_times_sqrt() {
    let expr = "<math><mrow>
        <msqrt> <mi>a</mi>  </msqrt>
        <msqrt> <mi>b</mi>  </msqrt>
        <mo>=</mo>
        <msqrt> <mrow>  <mi>a</mi><mi>b</mi></mrow> </msqrt>
        </mrow></math>";
    test("fi", "ClearSpeak", expr, "neliöjuuri a, neliöjuuri b; on yhtä suuri kuin, neliöjuuri a b");
}

#[test]
fn more_implied_times() {
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
    test_ClearSpeak("fi", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes",expr,
        "auki sulku 2 kertaa x, kiinni sulku toiseen");
}

#[test]
fn explicit_times_more_implied_times() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test_ClearSpeak("fi", "ClearSpeak_ImpliedTimes", "MoreImpliedTimes",expr, "t kertaa x");
}

#[test]
fn explicit_times_none_simple_right() {
    let expr = "<math><mn>2</mn><mo>[</mo><mn>3</mn> <mo>]</mo></math>";
    test_ClearSpeak("fi", "ClearSpeak_ImpliedTimes", "None",
        expr, "2, auki hakasulku 3 kiinni hakasulku");
}

#[test]
fn explicit_times_none_simple_left() {
    let expr = "<math><mo>(</mo><mn>2</mn><mo>&#x2212;</mo><mn>1</mn><mo>)</mo><mi>x</mi></math>";
    test_ClearSpeak("fi", "ClearSpeak_ImpliedTimes", "None",
        expr, "auki sulku 2 miinus 1, kiinni sulku; x");
}

#[test]
fn explicit_times_none_superscript() {
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
    test_ClearSpeak_prefs("fi", 
        vec![("ClearSpeak_ImpliedTimes", "None"), ("ClearSpeak_Functions", "None")],
        expr, "f, auki sulku x kiinni sulku; on yhtä suuri kuin; x toiseen, auki sulku x plus 1, kiinni sulku");
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
        test("fi", "ClearSpeak", expr, "25 kertaa x");
    }

    #[test]
    fn no_parens_monomial() {
        let expr = "<math><mrow>
        <mi>b</mi>
        <mrow><mo>(</mo>
        <mrow><mi>x</mi><mi>y</mi></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
        test("fi", "ClearSpeak", expr, "b x y");
    }

    #[test]
    fn no_parens_negative_number() {
        let expr = "<math><mrow>
        <mn>2</mn><mo>+</mo>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
        test("fi", "ClearSpeak", expr, "2 plus negatiivinen 2");
    }


    #[test]
    fn no_parens_negative_number_with_var() {
        let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow><mi>x</mi>
        <mo>)</mo>
        </mrow>
        <mo>+</mo><mn>1</mn>
        </mrow></math>";
        test("fi", "ClearSpeak", expr, "negatiivinen 2 x, plus 1");
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
        test("fi", "ClearSpeak", expr, "auki sulku 2 x kiinni sulku toiseen");
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
        test("fi", "ClearSpeak", expr, "2 plus 1 kahdesosa");
    }


    // Tests for the ten types of intervals in ClearSpeak
    #[test]
    fn parens_interval_open_open() {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fi", "ClearSpeak_Paren", "Interval",expr,
    "väli c d, ei sisältäen c tai d");
}

#[test]
    fn parens_interval_closed_open() {
        let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fi", "ClearSpeak_Paren", "Interval ",expr,
    "väli c d, sisältäen c mutta ei sisältäen d");
}


#[test]
fn parens_interval_open_closed() {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("fi", "ClearSpeak_Paren", "Interval ",expr,
    "väli c d, ei sisältäen c mutta sisältäen d");
}


#[test]
fn parens_interval_closed_closed() {
    let expr = "<math> 
    <mrow><mo>[</mo>
    <mrow> <mi>c</mi><mo>,</mo><mi>d</mi></mrow>
    <mo>]</mo></mrow>
</math>";
test_ClearSpeak("fi", "ClearSpeak_Paren", "Interval ",expr,
"väli c d, sisältäen c ja d");
}

    #[test]
    fn parens_interval_neg_infinity_open_open() {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fi", "ClearSpeak_Paren", "Interval ",expr,
    "väli negatiivinen ääretön d, ei sisältäen d");
}

    #[test]
    fn parens_interval_neg_infinity_closed_open() {
        let expr = "<math> 
        <mrow><mo>(</mo>
        <mrow> <mo>-</mo> <mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak("fi", "ClearSpeak_Paren", "Interval ",expr,
    "väli negatiivinen ääretön d, sisältäen d");
}


#[test]
fn parens_interval_open_open_infinity() {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
    <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fi", "ClearSpeak_Paren", "Interval ",expr,
    "väli c ääretön, ei sisältäen c");
}


#[test]
fn parens_interval_closed_open_infinity() {
    let expr = "<math> 
        <mrow><mo>[</mo>
        <mrow> <mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fi", "ClearSpeak_Paren", "Interval ",expr,
"väli c ääretön, sisältäen c");
}

#[test]
fn parens_interval_neg_infinity_to_infinity() {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fi", "ClearSpeak_Paren", "Interval ",expr,
    "väli negatiivinen ääretön ääretön");
}

#[test]
fn parens_interval_neg_infinity_to_pos_infinity() {
    let expr = "<math> 
    <mrow><mo>(</mo>
        <mrow><mo>-</mo> <mi>∞</mi><mo>,</mo><mo>+</mo><mi>∞</mi></mrow>
    <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak("fi", "ClearSpeak_Paren", "Interval ",expr,
    "väli negatiivinen ääretön positiivinen ääretön");
}
