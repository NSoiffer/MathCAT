#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod mroot {
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

    #[allow(non_snake_case)]
    fn test_ClearSpeak(pref_name: &str, pref_value: &str, mathml: &str, speech: &str) {
        libmathcat::speech::SPEECH_RULES.with(|rules| {
            let mut rules = rules.borrow_mut();
            let pref_manager = rules.pref_manager.as_mut();
            pref_manager.set_user_prefs(pref_name, pref_value);
        });
        assert_eq!(speech, strip_spaces(libmathcat::interface::speak_mathml(mathml)));
    }

    #[test]
    fn msqrt_simple() {
        let expr = "<math>
                        <msqrt> <mi>x</mi> </msqrt>
                    </math>";
        test(expr, "the square root of x,");
    }

    #[test]
    fn msqrt_simple_end_root() {
        let expr = "<math>
                        <msqrt> <mi>x</mi> </msqrt>
                    </math>";
        test_ClearSpeak("ClearSpeak_Roots", "EndRoot", expr, "the square root of x, end root;");
    }

    #[test]
    fn msqrt_simple_positive() {
        let expr = "<math>
                        <msqrt> <mi>x</mi> </msqrt>
                    </math>";
        test_ClearSpeak("ClearSpeak_Roots", "PosNegSqRoot", expr, "the positive square root of x,");
    }

    #[test]
    fn msqrt_simple_pos_end_root() {
        let expr = "<math>
                        <msqrt> <mi>x</mi> </msqrt>
                    </math>";
        test_ClearSpeak("ClearSpeak_Roots", "PosNegSqRootEnd", expr, "the positive square root of x, end root;");
    }

    #[test]
    fn msqrt() {
        let expr = "<math>
                        <msqrt>
                          <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow>
                        </msqrt>
                    </math>";
        test(expr, "the square root of x plus y;");
    }

    #[test]
    fn mroot_as_square_root() {
        let expr = "<math>
                        <mroot> <mi>x</mi> <mn>2</mn> </mroot>
                    </math>";
        test(expr, "the square root of x,");
    }

    #[test]
    fn cube_root() {
        let expr = "<math>
                        <mroot> <mi>x</mi> <mn>3</mn> </mroot>
                    </math>";
        test(expr, "the cube root of x,");
    }

    #[test]
        fn ordinal_root() {
            let expr = "<math>
                            <mroot> <mi>x</mi> <mn>9</mn> </mroot>
                        </math>";
            test(expr, "the ninth root of x,");
        }

    #[test]
    fn simple_mi_root() {
        let expr = "<math>
                        <mroot> <mi>x</mi> <mi>n</mi> </mroot>
                    </math>";
    test(expr, "the n-th root of x,");
    }

    #[test]
    fn mroot_simple_pos_end_root() {
        let expr = "<math>
                    <mroot> <mi>x</mi> <mi>t</mi> </mroot>
                    </math>";
        test_ClearSpeak("ClearSpeak_Roots", "PosNegSqRootEnd", expr, "the positive t-th root of x, end root;");
    }

    #[test]
    fn mroot_simple_end_root() {
        let expr = "<math>
                        <mroot> <mrow> <mi>x</mi> <mo>+</mo> <mi>y</mi> </mrow> 
                        <mn>21</mn></mroot>
                    </math>";
        test_ClearSpeak("ClearSpeak_Roots", "EndRoot", expr, "the twenty first root of x plus y, end root;");
    }

    #[test]
    fn simple_fraction_power() {
        let expr = "<math>
                        <mroot>
                           <mi>x</mi> 
                           <mfrac><mn>1</mn><mn>3</mn></mfrac>
                        </mroot>
                    </math>";
    test(expr, "the 1 third root of x,");
    }

}