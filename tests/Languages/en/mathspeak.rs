use crate::common::*;

#[test]
fn rule_7_1_1() {
    let expr = r#"<math><mi>a</mi><mo>-</mo>
                    <mfrac><mrow><mi>b</mi><mo>+</mo><mi>c</mi></mrow><mrow><mi>d</mi><mo>-</mo><mi>e</mi></mrow></mfrac>
                    <mo>&#xD7;</mo><mi>f</mi></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "eigh minus, frac b plus c over d minus e end frac; times f");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "eigh minus; start frac b plus c over d minus e end frac; times f");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "eigh minus; start fraction b plus c over d minus e end fraction; times f");
}

#[test]
fn rule_7_1_2() {
    let expr = r#"<math><mfrac><mn>1</mn><mi>x</mi></mfrac></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "frac 1 over x end frac");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "start frac 1 over x end frac");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "start fraction 1 over x end fraction");
}

#[test]
fn rule_7_2_1() {
    let expr = r#"<math><mfrac><mstyle displaystyle="true"><mfrac><mi>x</mi><mi>y</mi></mfrac></mstyle><mi>z</mi></mfrac><mo>&#x2260;</mo>
            <mfrac><mi>x</mi><mstyle displaystyle="true"><mfrac><mi>y</mi><mi>z</mi></mfrac></mstyle></mfrac></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "nest frac frac x over y end frac nest over z nest end frac; not equal to, nest frac x nest over frac y over z end frac nest end frac");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "start start frac start frac x over y end frac over over z end end frac; is not equal to; start start frac x over over start frac y over z end frac end end frac");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "start start fraction start fraction x over y end fraction over over z end end fraction; is not equal to; start start fraction x over over start fraction y over z end fraction end end fraction");
}

#[test]
fn rule_7_3_2() {
    let expr = r#"<math><msub><mi>a</mi><mn>0</mn></msub><mo>+</mo>
                    <mfrac><mn>1</mn><mrow><msub><mi>a</mi><mn>1</mn></msub><mo>+</mo><mstyle displaystyle="true">
                        <mfrac><mn>1</mn><mrow><msub><mi>a</mi><mn>2</mn></msub><mo>+</mo><mstyle displaystyle="true">
                            <mfrac><mn>1</mn><mrow><mo>&#x2026;</mo><mo>+</mo><mstyle displaystyle="true">
                                <mfrac><mn>1</mn><msub><mi>a</mi><mi>n</mi></msub></mfrac>
                            </mstyle></mrow></mfrac>
                        </mstyle></mrow></mfrac>
                    </mstyle></mrow></mfrac></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "eigh 0 plus; nest 3 frac 1 nest 3 over eigh 1 plus; nest twice frac 1 nest twice over eigh 2 plus; \
                        nest frac 1 nest over ellipsis plus frac 1 over eigh sub n base end frac nest end frac nest twice end frac nest 3 end frac");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "eigh 0 plus; start start start start frac 1 over over over over eigh 1 plus; start start start frac 1 over over over eigh 2 plus; \
                            start start frac 1 over over ellipsis plus \
                            start frac 1 over eigh sub n base end frac end end frac end end end frac end end end end frac");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "eigh 0 plus; start start start start fraction 1 over over over over eigh 1 plus; \
                            start start start fraction 1 over over over eigh 2 plus; start start fraction 1 over over ellipsis plus \
                            start fraction 1 over eigh subscript n baseline end fraction end end fraction end end end fraction end end end end fraction");
}

#[test]
fn rule_7_4_1() {
    let expr = r#"<math><mfrac><mn>20</mn><mn>5</mn></mfrac><mo>&#xD7;</mo>
            <mfrac><mn>1</mn><mn>100</mn></mfrac><mo>=</mo>
            <mfrac><mn>1</mn><mn>25</mn></mfrac></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "frac 20 over 5 end frac, times frac 1 over 100 end frac; equals 1 twenty fifth");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "start frac 20 over 5 end frac, times start frac 1 over 100 end frac; is equal to 1 twenty fifth");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "start fraction 20 over 5 end fraction, times, start fraction 1 over 100 end fraction; is equal to 1 twenty fifth");
}

#[test]
fn rule_7_5_1() {
    let expr = r#"<math><mn>3</mn><mfrac><mn>5</mn><mn>8</mn></mfrac><mo>=</mo><mfrac><mn>29</mn><mn>8</mn></mfrac></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "3 and 5 eighths, equals frac 29 over 8 end frac");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "3 and 5 eighths, is equal to, start frac 29 over 8 end frac");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "3 and 5 eighths, is equal to, start fraction 29 over 8 end fraction");
}

#[test]
fn rule_8_1_1() {
    let expr = r#"<math><msup><mi>x</mi><mn>3</mn></msup><mo>+</mo><mn>6</mn><msup><mi>x</mi><mn>2</mn></msup><mo>-</mo><mi>x</mi><mo>=</mo><mn>30</mn></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "x cubed plus 6 x squared, minus x; equals 30");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "x cubed plus 6 x squared, minus x; is equal to 30");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "x cubed plus 6 x squared, minus x; is equal to 30");
}

#[test]
fn rule_8_2_1() {
    let expr = r#"<math><msup><mi>x</mi><mi>a</mi></msup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "x soup eigh");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "x soup eigh");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "x superscript eigh");
}

#[test]
fn rule_8_2_2() {
    let expr = r#"<math><msub><mi>x</mi><mi>n</mi></msub></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "x sub n");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "x sub n");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "x subscript n");
}

#[test]
fn rule_8_2_3() {
    let expr = r#"<math><msup><mi>x</mi><mfrac><mn>1</mn><mn>2</mn></mfrac></msup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "x soup 1 half");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "x soup 1 half");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "x superscript 1 half");
}

#[test]
fn rule_8_3_1() {
    let expr = r#"<math><msup><mi>x</mi><mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow></msup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "x soup m plus n");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "x soup m plus n");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "x superscript m plus n");
}

#[test]
fn rule_8_3_2() {
    let expr = r#"<math><msub><mi>T</mi><mrow><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msub>
                            <mo>+</mo><mn>5</mn><mo>=</mo><mn>0</mn></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "upper t sub n minus 1 base, plus 5; equals 0");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "upper t sub n minus 1 base, plus 5; is equal to 0");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "upper t subscript n minus 1 baseline; plus 5; is equal to 0");
}

#[test]
fn rule_8_3_3() {
    let expr = r#"<math><msup><mi>x</mi><mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow></msup>
                            <mo>=</mo><msup><mi>x</mi><mi>m</mi></msup><msup><mi>x</mi><mi>n</mi></msup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "x soup m plus n base, equals, x soup m base x soup n");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "x soup m plus n base, is equal to, x soup m base x soup n");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "x superscript m plus n baseline; is equal to, x superscript m baseline, x superscript n");
}

#[test]
fn rule_8_4_1() {
    let expr = r#"<math><msup><mi>x</mi><mrow>
                        <msub><mi>a</mi><mi>n</mi></msub><mo>+</mo><msub><mi>a</mi><mrow><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msub></mrow></msup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "x soup eigh soup sub n soup plus eigh soup sub n minus 1");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "x soup eigh soup sub n soup plus eigh soup sub n minus 1");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "x superscript eigh super subscript n superscript plus eigh super subscript n minus 1");
}

#[test]
fn rule_8_4_3() {
    let expr = r#"<math><msup><mi>x</mi><msup><mi>a</mi><mi>b</mi></msup></msup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "x soup eigh soup soup b");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "x soup eigh soup soup b");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "x superscript eigh super superscript b");
}

#[test]
fn rule_8_4_4() {
    let expr = r#"<math><msub><mi>x</mi><msub><mi>a</mi><mi>b</mi></msub></msub></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "x sub eigh sub sub b");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "x sub eigh sub sub b");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "x subscript eigh sub subscript b");
}

#[test]
fn rule_8_4_5() {
    let expr = r#"<math><msub><mi>x</mi><msup><mi>a</mi><mi>b</mi></msup></msub></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "x sub eigh sub soup b");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "x sub eigh sub soup b");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "x subscript eigh sub superscript b");
}

#[test]
fn rule_8_4_6() {
    let expr = r#"<math><msup><mi>x</mi><msub><mi>a</mi><mi>b</mi></msub></msup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "x soup eigh soup sub b");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "x soup eigh soup sub b");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "x superscript eigh super subscript b");
}

#[test]
fn rule_8_4_7() {
    let expr = r#"<math><msup><mi>y</mi><msup><mi>a</mi><msub><mi>b</mi><mi>c</mi></msub></msup></msup><mo>&#x2260;</mo>
            <msup><mi>y</mi><mrow><msup><mi>a</mi><mi>b</mi></msup><mi>c</mi></mrow></msup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "y soup eigh soup soup b soup soup sub c base; not equal to, y soup eigh soup soup b soup, c");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "y soup eigh soup soup b soup soup sub c base; is not equal to, y soup eigh soup soup b soup, c");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "y superscript eigh super superscript b super super subscript c baseline; is not equal to; y superscript eigh super superscript b superscript, c");
}

#[test]
fn rule_8_4_8() {
    let expr = r#"<math><msup><mi>T</mi>
            <mrow><mo>(</mo><msup><mi>x</mi><mi>a</mi></msup><mo>+</mo><msup><mi>y</mi><mi>b</mi></msup><mo>)</mo></mrow>
        </msup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "upper t soup l paren, x soup soup eigh soup, plus y soup soup b soup; r paren");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "upper t soup left paren, x soup soup eigh soup, plus y soup soup b soup; right paren");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "upper t superscript left parenthesis; x super superscript eigh superscript, plus y super superscript b superscript; right parenthesis");
}

#[test]
fn rule_8_5_1() {
    let expr = r#"<math><msub><mi>x</mi><mn>1</mn></msub></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "x 1");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "x 1");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "x 1");
}

#[test]
fn rule_8_5_2() {
    let expr = r#"<math><msub><mi>T</mi>
                <mrow><msub><mi>n</mi><mn>1</mn></msub><mo>+</mo><msub><mi>n</mi><mn>0</mn></msub></mrow>
            </msub></math>"#;
    // MathSpeak spec has the following, but that is not right as the numeric subscript rule only applies to a base that is not nested
    // test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
    //             "upper t sub n 1 plus n 0");
    // test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
    //             "upper t sub n 1 plus n 0");
    // test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
    //             "upper t subscript n 1 plus n 0");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "upper t sub n sub sub 1 sub plus n sub sub 0");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "upper t sub n sub sub 1 sub plus n sub sub 0");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "upper t subscript n sub subscript 1 subscript plus n sub subscript 0");
}

#[test]
fn rule_8_8_2() {
    let expr = r#"<math><msubsup><mi>T</mi><mrow><mi>n</mi><mo>-</mo><mn>1</mn></mrow><mn>2</mn></msubsup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "upper t sub n minus 1 soup 2");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "upper t sub n minus 1 soup 2");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "upper t subscript n minus 1 superscript 2");
}

#[test]
fn rule_8_9_2() {
    let expr = r#"<math><msup><mi>x</mi><mo>′</mo></msup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "x prime");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "x prime");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "x prime");
}

#[test]
fn rule_8_10_1() {
    let expr = r#"<math><msubsup><mi>T</mi><mi>n</mi><mo>'</mo></msubsup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "upper t prime sub n");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "upper t prime sub n");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "upper t prime subscript n");
}

#[test]
fn rule_8_10_2() {
    let expr = r#"<math><msubsup><mi>x</mi><mn>10</mn><mo>'</mo></msubsup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "x prime 10");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "x prime 10");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "x prime 10");
}

#[test]
fn rule_9_1_1() {
    let expr = r#"<math><msqrt><mi>m</mi><mo>+</mo><mi>n</mi></msqrt></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "root m plus n end root");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "start root m plus n end root");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "start root m plus n end root");
}

#[test]
fn rule_9_1_2() {
    let expr = r#"<math><msqrt><mn>2</mn></msqrt></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "root 2 end root");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "start root 2 end root");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "start root 2 end root");
}

#[test]
fn rule_9_2_1() {
    let expr = r#"<math><mroot><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow></mroot></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr, "index m plus n root x plus y end root");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr, "root index m plus n start root x plus y end root");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr, "root index m plus n start root x plus y end root");
}

#[test]
fn rule_9_2_2() {
    let expr = r#"<math><mroot><mi>x</mi><mn>3</mn></mroot><mo>=</mo><msup><mi>x</mi><mfrac><mn>1</mn><mn>3</mn></mfrac></msup></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "index 3 root x end root, equals x soup 1 third");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "root index 3 start root x end root, is equal to x soup 1 third");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "root index 3 start root x end root, is equal to x superscript 1 third");
}

#[test]
fn rule_9_3_1() {
    let expr = r#"<math>
            <msup>
                <mi>x</mi>
                <mrow>
                    <mi>e</mi>
                    <mo>-</mo>
                    <mn>2</mn>
                </mrow>
            </msup>
            <mo>=</mo>
            <msqrt>
                <mi>x</mi>
                <mroot>
                    <mrow>
                        <mi>x</mi>
                        <mroot>
                            <mrow>
                                <mi>x</mi>
                                <mroot><mrow><mi>x</mi><mo>&#x2026;</mo></mrow><mn>5</mn></mroot>
                            </mrow>
                            <mn>4</mn>
                        </mroot>
                    </mrow>
                    <mn>3</mn>
                </mroot>
            </msqrt>
            <mo>,</mo><mo>&#xA0;</mo><mi>x</mi><mo>&#x2208;</mo><mi mathvariant="normal">&#x211D;</mi>
        </math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "x soup e minus 2 base, equals; nest 3 root x; nest twice index 3 nest twice root x; \
                        nest index 4 nest root x, index 5 root x ellipsis end root nest end root \
                        nest twice end root nest 3 end root; comma; x an element of, double-struck upper r");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "x soup e minus 2 base, is equal to; nest 3 start root x; nest twice root index 3 nest twice start root x; \
                        nest root index 4 nest start root x, root index 5 start root x ellipsis end root nest end root \
                        nest twice end root nest 3 end root; comma; x is an element of, double-struck upper r");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "x superscript e minus 2 baseline; is equal to; nested 3 start root x; nested twice root index 3 nested twice start root x; \
                        nested root index 4 nested start root x, root index 5 start root x ellipsis end root nested \
                        end root nested twice end root nested 3 end root; comma; x is an element of, double-struck upper r");
}

#[test]
fn rule_9_3_2() {
    let expr = r#"<math><msqrt>
            <msqrt><mi>x</mi><mo>+</mo><mn>1</mn></msqrt><mo>+</mo>
            <msqrt><mi>y</mi><mo>+</mo><mn>1</mn></msqrt></msqrt></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                    "nest root root x plus 1 end root, plus root y plus 1 end root nest end root");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "nest start root start root x plus 1 end root, plus, start root y plus 1 end root nest end root");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "nested start root start root x plus 1 end root, plus, start root y plus 1 end root nested end root");
}

#[test]
fn rule_9_3_3() {
    let expr = r#"<math><mroot><mroot><mi>x</mi><mi>m</mi></mroot><mi>n</mi></mroot><mo>=</mo>
                        <mroot><mroot><mi>x</mi><mi>n</mi></mroot><mi>m</mi></mroot></math>"#;
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Terse")], expr,
                "nest index n nest root index m root x end root nest end root; equals, \
                        nest index m nest root index n root x end root nest end root");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Medium")], expr,
                "nest root index n nest start root root index m start root x end root nest end root; is equal to; \
                        nest root index m nest start root root index n start root x end root nest end root");
    test_prefs("en", "MathSpeak", vec![("Verbosity", "Verbose")], expr,
                "nested root index n nested start root root index m start root x end root nested end root; is equal to; \
                        nested root index m nested start root root index n start root x end root nested end root");
}
