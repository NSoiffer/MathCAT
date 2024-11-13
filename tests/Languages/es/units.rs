/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;

#[test]
fn si_basic1() {
    let expr = r#"<math><mn>1</mn><mi intent=":unit">l</mi><mo>,</mo><mo>&#xA0;</mo>
                            <mn>2</mn><mi intent=":unit">m</mi><mo>,</mo><mo>&#xA0;</mo>
                            <mi>x</mi><mo>&#xA0;</mo><mi intent=":unit">ms</mi><mo>,</mo><mo>&#xA0;</mo>
                            <mn>1235</mn><mi intent=":unit">daN</mi><mo>,</mo><mo>&#xA0;</mo>
                            <mn>2.5</mn><mi intent=":unit">&#xB5;sec</mi><mo>,</mo><mo>&#xA0;</mo>
                            <mn>32.34</mn><mi intent=":unit">mol</mi></math>"#;
    test("es", "SimpleSpeak", expr, 
        "1 litro coma 2 metros, coma, x mili-segundos, coma, 1235 deka-newtons, coma, 25 micro-segundos, coma 3234 mols");
}

#[test]
fn test_mtext_inference() {
    let expr = r#"<math><mo>[</mo>
                <mn>1</mn><mtext>t</mtext><mo>,</mo><mo>&#xA0;</mo>
                <mn>2</mn><mtext>PA</mtext><mo>,</mo><mo>&#xA0;</mo>
                <mn>3</mn><mtext>Pa</mtext><mo>,</mo><mo>&#xA0;</mo>
                <mn>4.5</mn><mtext>mT</mtext>
            <mo>]</mo></math>"#;
    test("es", "SimpleSpeak", expr, 
        "abrir corchetes; 1 tonelada métrica, coma, 2 peta-amperios, coma 3 pascals, coma, 45 mili-teslas; cerrar corchetes");
}

