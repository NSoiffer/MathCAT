#[macro_use]
extern crate lazy_static;

#[cfg(test)]
/// Tests for fractions
///   includes simple fractions and more complex fractions
///   also tests mixed fractions (implicit and explicit)
mod mfrac {
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
    fn common_fraction_half() {
        let expr = "<math>
                        <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                    </math>";
        test(expr, "1 half");
    }

    #[test]
    fn common_fraction_thirds() {
        let expr = "<math>
                        <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                    </math>";
        test(expr, "2 thirds");
    }

    #[test]
    fn common_fraction_tenths() {
        let expr = "<math>
                        <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                    </math>";
        test(expr, "17 tenths");
    }

    #[test]
    #[allow(non_snake_case)]
    fn not_ClearSpeak_common_fraction_tenths() {
        let expr = "<math>
                        <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                    </math>";
        test(expr, "89 over 10");
    }

    #[test]
    fn non_simple_fraction() {
        let expr = "
        <math>
         <mrow>
          <mfrac>
           <mrow>
            <mi>x</mi><mo>+</mo><mi>y</mi></mrow>
           <mrow>
            <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
          </mfrac>
          </mrow>
        </math>
                              ";
        test(expr, "the fraction with numerator; x plus y; and denominator x minus y;");
    }

    
    #[test]
    fn mixed_number() {
        let expr = "<math>
                        <mn>3</mn>
                        <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                    </math>";
        test(expr, "3 and 1 half");
    }
    
    #[test]
    fn explicit_mixed_number() {
        let expr = "<math>
                        <mn>3</mn>
                        <mo>&#x2064;</mo>
                        <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                    </math>";
        test(expr, "3 and 1 eighth");
    }

    #[test]
    fn mixed_number_big() {
        let expr = "<math>
                        <mn>3</mn>
                        <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                    </math>";
        test(expr, "3 and 7 eighty thirds");
    }

    #[test]
    fn simple_text() {
        let expr = "<math>
        <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                    </math>";
        test(expr, "rise over run");
    }

    #[test]
    fn number_and_text() {
        let expr = "<math>
                <mfrac>
                <mrow>
                    <mn>2</mn><mtext>miles</mtext></mrow>
                <mrow>
                    <mn>3</mn><mtext>gallons</mtext></mrow>
                </mfrac>
            </math>";
        test(expr, "2 miles over 3 gallons");
    }


    #[test]
    fn nested_simple_fractions() {
        let expr = "<math>
                    <mrow>
                    <mfrac>
                        <mrow>
                        <mfrac>
                            <mn>1</mn>
                            <mn>2</mn>
                        </mfrac>
                        </mrow>
                        <mrow>
                        <mfrac>
                            <mn>2</mn>
                            <mn>3</mn>
                        </mfrac>
                        </mrow>
                    </mfrac>
                    </mrow>
                </math>";
        test(expr, "1 half over 2 thirds");
    }


    #[test]
    fn semi_nested_fraction() {
        let expr = "<math>
                    <mrow>
                            <mfrac>
                            <mrow>
                            <mfrac>
                            <mn>2</mn>
                            <mn>3</mn>
                            </mfrac>
                            <mi>x</mi>
                        </mrow>
                        <mn>6</mn>
                        </mfrac>
                    </mrow>
                    </math>";
        test(expr, "2 thirds x over 6");
    }

    #[test]
    fn general_nested_fraction() {
        let expr = "
        <math>
        <mrow>
         <mfrac>
          <mrow>
           <mfrac>
             <mn>10</mn>
             <mi>n</mi>
           </mfrac>
           </mrow>
          <mrow>
           <mfrac>
            <mn>2</mn>
            <mi>n</mi>
           </mfrac>
           </mrow>
         </mfrac>
         </mrow>
       </math>
                      ";
        test(expr, "the fraction with numerator; 10 over n; and denominator 2 over n;");
    }

    #[test]
    fn complex_nested_fraction() {
        let expr = "
        <math>
        <mrow>
         <mfrac>
          <mrow>
           <mfrac>
             <mrow> <mi>n</mi> <mo>+</mo> <mn>10</mn> </mrow>
             <mi>n</mi>
           </mfrac>
           </mrow>
          <mrow>
           <mfrac>
            <mn>2</mn>
            <mi>n</mi>
           </mfrac>
           </mrow>
         </mfrac>
         </mrow>
       </math>
                      ";
        test(expr, "the fraction with numerator; the fraction with numerator; n plus 10; and denominator n; and denominator 2 over n;");
    }

    #[test]
    fn binomial() {
        let expr = "<math>
                        <mn>2</mn>
                        <mo>(</mo>
                        <mfrac linethickness='0'> <mn>7</mn> <mn>3</mn> </mfrac>
                        <mo>)</mo>
                    </math>";
        test(expr, "2 times 7 choose 3");
    }
}
