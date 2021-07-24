#[cfg(test)]
mod number_sets {
    #[test]
    fn complex() {
        let expr = "<math>
                        <mi>ℂ</mi>
                    </math>";
        assert_eq!("the complex numbers", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }

    #[test]
    fn natural() {
        let expr = "<math>
                        <mi>ℕ</mi>
                    </math>";
        assert_eq!("the natural numbers", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }

    #[test]
    fn rationals() {
        let expr = "<math>
                        <mi>ℚ</mi>
                    </math>";
        assert_eq!("the rational numbers", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }

    #[test]
    fn reals() {
        let expr = "<math>
                        <mi>ℝ</mi>
                    </math>";
        assert_eq!("the real numbers", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }

    #[test]
    fn integers() {
        let expr = "<math>
                        <mi>ℤ</mi>
                    </math>";
        assert_eq!("the integers", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }



    #[test]
    fn msup_complex() {
        let expr = "<math>
                    <msup>
                        <mi>ℂ</mi>
                        <mn>2</mn>
                    </msup>
                    </math>";
        assert_eq!("c 2", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }

    #[test]
    fn msup_natural() {
        let expr = "<math>
                    <msup>
                        <mi>ℕ</mi>
                        <mn>2</mn>
                    </msup>
                </math>";
        assert_eq!("n 2", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }

    #[test]
    fn msup_rationals() {
        let expr = "<math>
                    <msup>
                        <mi>ℚ</mi>
                        <mn>2</mn>
                    </msup>
                </math>";
        assert_eq!("q 2", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }

    #[test]
    fn msup_reals() {
        let expr = "<math>
                    <msup>
                        <mi>ℝ</mi>
                        <mn>3</mn>
                    </msup>
                </math>";
        assert_eq!("r 3", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }

    #[test]
    fn msup_integers() {
        let expr = "<math>
                    <msup>
                        <mi>ℤ</mi>
                        <mn>4</mn>
                    </msup>
                </math>";
        assert_eq!("z 4", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }

    #[test]
    fn msup_positive_integers() {
        let expr = "<math>
                    <msup>
                        <mi>ℤ</mi>
                        <mo>+</mo>
                    </msup>
                </math>";
        assert_eq!("the positive integers", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }

    #[test]
    fn msup_negative_integers() {
        let expr = "<math>
                    <msup>
                        <mi>ℤ</mi>
                        <mo>-</mo>
                    </msup>
                </math>";
        assert_eq!("the negative integers", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }

    #[test]
    fn msup_positive_rationals() {
        let expr = "<math>
                    <msup>
                        <mi>ℚ</mi>
                        <mo>+</mo>
                    </msup>
                </math>";
        assert_eq!("the positive rational numbers", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }

    #[test]
    fn msup_negative_rationals() {
        let expr = "<math>
                    <msup>
                        <mi>ℚ</mi>
                        <mo>-</mo>
                    </msup>
                </math>";
        assert_eq!("the negative rational numbers", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
    }

}