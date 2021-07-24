#[macro_use]
extern crate lazy_static;

#[cfg(test)]
/// Tests for:
///   functions including trig functions, logs, and functions to powers
///   implied times/functional call and explicit times/function call
///   silent parens
mod functions {
    extern crate regex;
    use regex::Regex;
    
    fn strip_spaces(str: String) -> String {
        lazy_static! {
           static ref SPACES: Regex = Regex::new(r"  +").unwrap();
        }
        return String::from( SPACES.replace_all(&str, " ") );
    }

    fn test(mathml: &str, speech: &str) {
        assert_eq!(speech, strip_spaces(libmathcat::interface::speak_mathml(mathml)));
    }

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
        test(expr, "sine of x plus cosine of y plus tangent of z plus secant of alpha, plus co-secant of phi, plus co-tangent of phi");
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
        test(expr, "hyperbolic sine of x, plus \
                                 hyperbolic cosine of y, plus \
                                 hyperbolic tangent of z, plus \
                                 hyperbolic secant of alpha, plus \
                                 hyperbolic co-secant of phi, plus \
                                 hyperbolic co-tangent of phi");
    }
   

    #[test]
    fn inverse_trig() {
        let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
        test(expr, "sine inverse of x");
    }

    #[test]
    fn trig_squared() {
        let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
        test(expr, "sine squared of x");
    }

    #[test]
    fn trig_cubed() {
        let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
        test(expr, "tangent cubed of x");
    }

    #[test]
    fn trig_fourth() {
        let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
        test(expr, "the fourth power of, secant of x");
    }


    #[test]
    fn trig_power_other() {
        let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
        test(expr, "the n minus 1 power of, hyperbolic sine of x");
    }

    #[test]
    fn simple_log() {
        let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
        test(expr, "log x");
    }

    #[test]
    fn normal_log() {
        let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
        test(expr, "the log of, open paren x plus y close paren");
    }

    #[test]
    fn simple_log_with_base() {
        let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
        test(expr, "the log base b of x");
    }

    #[test]
    fn normal_log_with_base() {
        let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
        test(expr, "the log base b of, open paren x plus y close paren");
    }

    #[test]
    fn simple_ln() {
        let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
        test(expr, "l n x");
    }

    #[test]
    fn normal_ln() {
        let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
        test(expr, "l n of, open paren x plus y close paren");
    }

    #[test]
    fn explicit_function_call_with_parens() {
        let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
        test(expr, "t of x");
    }


    #[test]
    fn explicit_times_with_parens() {
        let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
        test(expr, "t times x");
    }

    #[test]
    fn explicit_function_call() {
        let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
        test(expr, "t of x");
    }

    #[test]
    fn explicit_times() {
        let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
        test(expr, "t x");
    }



    /*
     * Tests for times
     */
    #[test]
    fn no_times_binomial() {
        let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
        test(expr, "x y");
    }

    #[test]
    fn times_following_paren() {
        let expr = "<math><mrow>
           <mn>2</mn>
           <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
         </mrow></math>";
        test(expr, "2 times 3");
    }

    #[test]
    fn times_preceding_paren() {
        let expr = "<math><mrow>
           <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
           <mn>3</mn>
         </mrow></math>";
        test(expr, "2 times 3");
    }

    #[test]
    fn no_times_sqrt() {
        let expr = "<math><mrow>
            <msqrt> <mi>a</mi>  </msqrt>
            <msqrt> <mi>b</mi>  </msqrt>
            <mo>=</mo>
            <msqrt> <mrow>  <mi>a</mi><mi>b</mi></mrow> </msqrt>
          </mrow></math>";
        test(expr, "the square root of eigh; the square root of b; equals, the square root of eigh b,");
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
         </mrow></math>";
         test(expr, "25");
     }
 
     #[test]
     fn no_parens_monomial() {
         let expr = "<math><mrow>
           <mrow><mo>(</mo>
            <mrow><mi>x</mi><mi>y</mi></mrow>
           <mo>)</mo></mrow>
         </mrow></math>";
         test(expr, "x y");
     }
 
     #[test]
     fn no_parens_negative_number() {
         let expr = "<math><mrow>
           <mn>2</mn><mo>+</mo>
           <mrow><mo>(</mo>
            <mrow><mo>&#x2212;</mo><mn>2</mn></mrow>
           <mo>)</mo></mrow>
           </mrow></math>";
         test(expr, "2 plus negative 2");
     }
 
 
     #[test]
     fn no_parens_negative_number_with_var() {
         let expr = "<math><mrow>
           <mrow><mo>(</mo>
            <mrow><mo>&#x2212;</mo><mn>2</mn></mrow><mi>x</mi>
           <mo>)</mo></mrow>
           </mrow></math>";
         test(expr, "negative 2 x");
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
         test(expr, "open paren 2 x close paren squared");
     }
 
     #[test]
     fn no_parens_fraction() {
         let expr = "<math><mrow>
            <mrow>
             <mrow><mo>(</mo>
              <mfrac> <mn>1</mn><mn>2</mn></mfrac>
             <mo>)</mo></mrow></mrow>
        </mrow></math>";
         test(expr, "1 half");
     }
 
 
}