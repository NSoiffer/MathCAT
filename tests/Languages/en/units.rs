/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;

#[test]
fn si_basic1() {
    let expr = r#"<math><mn>1</mn><mi intent=":unit">l</mi><mo>,</mo><mo>&#xA0;</mo>
                            <mn>2</mn><mi intent=":unit">m</mi><mo>,</mo><mo>&#xA0;</mo>
                            <mi>x</mi><mo>&#xA0;</mo><mi intent=":unit">ms</mi><mo>,</mo><mo>&#xA0;</mo>
                            <mn>2.5</mn><mi intent=":unit">&#xB5;sec</mi><mo>,</mo><mo>&#xA0;</mo>
                            <mn>32.34</mn><mi intent=":unit">mol</mi></math>"#;
    test("en", "SimpleSpeak", expr, 
        "1 litre comma 2 metres, comma, x milli-seconds, comma, 2.5 micro-seconds, comma 32.34 moles");
}

#[test]
fn english() {
    let expr = r#"<math>
                <mn>1</mn><mi intent=":unit">cp</mi>
                <mo>+</mo><mn>2</mn><mi intent=":unit">tbl</mi>
                <mo>+</mo><mn>3</mn><mi intent=":unit">tsp</mi>
                </math>"#;
    test("en", "SimpleSpeak", expr, 
        "1 cup plus 2 tablespoons, plus 3 teaspoons");
}

#[test]
fn plural() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
        <mo fence="false" stretchy="false">[</mo>
            <mn>67</mn><mi intent=":unit">mph</mi> <mo>,</mo>
            <mn>2</mn><mi intent=":unit">ft</mi><mo>,</mo>
            <mn>3</mn><mi intent=":unit">in</mi>
        <mo fence="false" stretchy="false">]</mo>
        </math>"#;
    test("en", "SimpleSpeak", expr, 
        "open bracket; 67 mile per hours, comma 2 feet comma 3 inches; close bracket");
}
