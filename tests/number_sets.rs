#[cfg(test)]
mod number_sets {
    use libmathcat::test::*;

    #[test]
    fn complex() {
        let expr = "<math>
                        <mi>ℂ</mi>
                    </math>";
        test(expr, "the complex numbers");
    }

    #[test]
    fn natural() {
        let expr = "<math>
                        <mi>ℕ</mi>
                    </math>";
        assert_eq!("the natural numbers", libmathcat::interface::speak_mathml(expr /*, tts: none*/));
        test(expr, "the natural numbers");
    }

    #[test]
    fn rationals() {
        let expr = "<math>
                        <mi>ℚ</mi>
                    </math>";
        test(expr, "the rational numbers");
    }

    #[test]
    fn reals() {
        let expr = "<math>
                        <mi>ℝ</mi>
                    </math>";
        test(expr, "the real numbers");
    }

    #[test]
    fn integers() {
        let expr = "<math>
                        <mi>ℤ</mi>
                    </math>";
        test(expr, "the integers");
    }



    #[test]
    fn msup_complex() {
        let expr = "<math>
                    <msup>
                        <mi>ℂ</mi>
                        <mn>2</mn>
                    </msup>
                    </math>";
        test(expr, "c 2");
    }

    #[test]
    fn msup_natural() {
        let expr = "<math>
                    <msup>
                        <mi>ℕ</mi>
                        <mn>2</mn>
                    </msup>
                </math>";
        test(expr, "n 2");
    }

    #[test]
    fn msup_rationals() {
        let expr = "<math>
                    <msup>
                        <mi>ℚ</mi>
                        <mn>2</mn>
                    </msup>
                </math>";
        test(expr, "q 2");
    }

    #[test]
    fn msup_reals() {
        let expr = "<math>
                    <msup>
                        <mi>ℝ</mi>
                        <mn>3</mn>
                    </msup>
                </math>";
        test(expr, "r 3");
    }

    #[test]
    fn msup_integers() {
        let expr = "<math>
                    <msup>
                        <mi>ℤ</mi>
                        <mn>4</mn>
                    </msup>
                </math>";
        test(expr, "z 4");
    }

    #[test]
    fn msup_positive_integers() {
        let expr = "<math>
                    <msup>
                        <mi>ℤ</mi>
                        <mo>+</mo>
                    </msup>
                </math>";
        test(expr, "the positive integers");
    }

    #[test]
    fn msup_negative_integers() {
        let expr = "<math>
                    <msup>
                        <mi>ℤ</mi>
                        <mo>-</mo>
                    </msup>
                </math>";
        test(expr, "the negative integers");
    }

    #[test]
    fn msup_positive_rationals() {
        let expr = "<math>
                    <msup>
                        <mi>ℚ</mi>
                        <mo>+</mo>
                    </msup>
                </math>";
        test(expr, "the positive rational numbers");
    }

    #[test]
    fn msup_negative_rationals() {
        let expr = "<math>
                    <msup>
                        <mi>ℚ</mi>
                        <mo>-</mo>
                    </msup>
                </math>";
        test(expr, "the negative rational numbers");
    }
}