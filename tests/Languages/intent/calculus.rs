/// Tests for:
/// *  calculus-related inference
use crate::common::*;


#[test]
fn laplacian() {
  let mathml = r#"<math>
        <msup> <mo>∇</mo> <mn>2</mn> </msup>
        <mi>&#x3C8;</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math' >
      <laplacian data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
        <mi data-from-mathml='mi'>ψ</mi>
      </laplacian>
   </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn laplacian_as_vector() {
  let mathml = r#"<math>
        <msup> <mover><mo>∇</mo><mo>&#x2192;</mo></mover> <mn>2</mn> </msup>
        <mi>&#x3C8;</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math' >
      <laplacian data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
        <mi data-from-mathml='mi'>ψ</mi>
      </laplacian>
   </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn laplacian_as_operator() {
  let mathml = r#"<math>
    <msup> <mo>𝛁</mo> <mn>2</mn> </msup>
    </math>"#;
  let intent = r#"<math data-from-mathml='math' >
      <laplacian data-from-mathml='msup' />
   </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn simple_gradient() {
  let mathml = r#"<math>
        <mi mathvariant="normal">∇</mi>
        <mi>f</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <gradient data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
            <mi data-from-mathml='mi'>f</mi>
        </gradient>
    </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn simple_bold_gradient() {
  let mathml = r#"<math>
        <mi mathvariant="bold">∇</mi>
        <mi>f</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <gradient data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
            <mi data-from-mathml='mi'>f</mi>
        </gradient>
       </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn simple_div() {
  let mathml = r#"<math>
        <mi mathvariant="normal">∇</mi>
        <mo>⋅</mo>
        <mi mathvariant="bold">f</mi>
     </math>"#;
  let intent = r#"<math data-from-mathml='math'>
    <divergence data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
        <mi data-from-mathml='mi' mathvariant='bold'>𝐟</mi>
    </divergence> 
   </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn simple_curl() {
  let mathml = r#"<math>
        <mi mathvariant="normal">∇</mi>
        <mo>&#xD7;</mo>
        <mi mathvariant="bold">f</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <curl data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
            <mi data-from-mathml='mi' mathvariant='bold'>𝐟</mi>
        </curl>
    </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn curl_in_mrow() {
  let mathml = r#"<math>
        <mi>r</mi>
        <mi mathvariant="normal">&#x2207;</mi>
        <mo>&#xD7;</mo>
        <mrow >
            <mi mathvariant="bold">A</mi>
        </mrow>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
    <mrow data-from-mathml='mrow' data-changed='added'>
        <mi data-from-mathml='mi'>r</mi>
        <mo data-from-mathml='mo' data-changed='added'>&#x2062;</mo>
        <curl data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
        <mi data-from-mathml='mi' mathvariant='bold'>𝐀</mi>
        </curl>
    </mrow>
    </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn gradient_vector() {
  let mathml = r#"<math>
        <mover>
            <mi mathvariant="normal">&#x2207;</mi>
            <mo stretchy="false">&#x2192;</mo>
        </mover>
        <mi>f</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <gradient data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
            <mi data-from-mathml='mi'>f</mi>
        </gradient>
    </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn curl_vector() {
  let mathml = r#"<math>
        <mover>
            <mi mathvariant="normal">&#x2207;</mi>
            <mo stretchy="false">&#x2192;</mo>
        </mover>
        <mo>&#xD7;</mo>
        <mover>
            <mi>f</mi>
            <mo stretchy="false">&#x2192;</mo>
        </mover>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
        <curl data-from-mathml='mrow' data-changed='added' data-intent-property=':function:'>
            <vector data-from-mathml='mover' data-intent-property=':prefix:'>
                <mi data-from-mathml='mi'>f</mi>
            </vector>
        </curl>
    </math>"#;
    test_intent(mathml, intent, vec![]);
}
