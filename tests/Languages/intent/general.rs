/// Tests for:
/// *  functions including trig functions, logs, and functions to powers
/// *  implied times/functional call and explicit times/function call
/// *  parens
/// These are all intertwined, so they are in one file
use crate::common::*;

#[test]
fn binomial() {
  let mathml = "<math><mrow>
        <mo>(</mo>
        <mfrac linethickness='0'> <mn arg='n'>7</mn> <mn arg='m'>3</mn> </mfrac>
        <mo>)</mo>
    </mrow></math>";
  let intent = "<math data-from-mathml='math'>
        <binomial data-from-mathml='mrow' data-intent-property=':infix:'>
          <mn data-from-mathml='mn' arg='n'>7</mn>
          <mn data-from-mathml='mn' arg='m'>3</mn>
        </binomial>
    </math>";
  test_intent(mathml, intent, vec![]);
}

#[test]
fn closed_interval() {
    let expr = r#"<math>
      <mo stretchy="false">[</mo>
      <mi>a</mi>
      <mo>,</mo>
      <mi>b</mi>
      <mo stretchy="false">]</mo>
    </math>"#;
    let target = "<math data-from-mathml='math'>
      <closed-interval data-from-mathml='mrow' data-changed='added'>
        <mi data-from-mathml='mi'>a</mi>
        <mi data-from-mathml='mi'>b</mi>
      </closed-interval>
    </math>";
    test_intent(expr, target, vec![]);
}

#[test]
fn nested_interval_bug_329() {
    let expr = r#"<math>
      <mo stretchy="false">[</mo>
      <mi>A</mi>
      <mo>,</mo>
      <mo stretchy="false">[</mo>
      <mi>B</mi>
      <mo>,</mo>
      <mi>C</mi>
      <mo stretchy="false">]</mo>
      <mo stretchy="false">]</mo>
    </math>"#;
    let target = "<math data-from-mathml='math'>
    <mrow data-from-mathml='mrow' data-changed='added'>
      <mo data-from-mathml='mo' stretchy='false'>[</mo>
      <mrow data-from-mathml='mrow' data-changed='added'>
        <mi data-from-mathml='mi'>A</mi>
        <mo data-from-mathml='mo'>,</mo>
        <mrow data-from-mathml='mrow' data-changed='added'>
          <mo data-from-mathml='mo' stretchy='false'>[</mo>
          <mrow data-from-mathml='mrow' data-changed='added'>
            <mi data-from-mathml='mi'>B</mi>
            <mo data-from-mathml='mo'>,</mo>
            <mi data-from-mathml='mi'>C</mi>
          </mrow>
          <mo data-from-mathml='mo' stretchy='false'>]</mo>
        </mrow>
      </mrow>
      <mo data-from-mathml='mo' stretchy='false'>]</mo>
    </mrow>
   </math>";
    test_intent(expr, target, vec![]);
}
