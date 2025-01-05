/// Tests for:
/// *  functions including trig functions, logs, and functions to powers
/// *  implied times/functional call and explicit times/function call
/// *  parens
/// These are all intertwined, so they are in one file
use crate::common::*;



#[test]
fn dot_product_vec_arrow() {
  let mathml = r#"<math>
      <mrow>
        <mover>
          <mi data-latex="A">A</mi>
          <mo stretchy="false">→</mo>
        </mover>
      </mrow>
      <mo data-latex="\cdot">&#x22C5;</mo>
      <mrow>
        <mover>
          <mi data-latex="B">B</mi>
          <mo stretchy="false">→</mo>
        </mover>
      </mrow>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
    <mrow data-from-mathml='mrow' data-changed='added'>
      <modified-variable data-from-mathml='mover'>
        <mi data-from-mathml='mi' data-latex='A' >A</mi>
        <mo data-from-mathml='mo' stretchy='false' >→</mo>
      </modified-variable>
      <dot-product data-from-mathml='mo' data-latex='\cdot' ></dot-product>
      <modified-variable data-from-mathml='mover'>
        <mi data-from-mathml='mi' data-latex='B'>B</mi>
        <mo data-from-mathml='mo' stretchy='false'>→</mo>
      </modified-variable>
    </mrow>
   </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn cross_product_vec_harpoon() {
  let mathml = r#"<math>
      <mrow>
        <mover>
          <mi data-latex="A">A</mi>
          <mo stretchy="false">⇀</mo>
        </mover>
      </mrow>
      <mo>×</mo>
      <mrow>
        <mover>
          <mi data-latex="B">B</mi>
          <mo stretchy="false">⇀</mo>
        </mover>
      </mrow>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
    <mrow data-from-mathml='mrow' data-changed='added'>
      <modified-variable data-from-mathml='mover'>
        <mi data-from-mathml='mi' data-latex='A' >A</mi>
        <mo data-from-mathml='mo' stretchy='false' >⇀</mo>
      </modified-variable>
      <cross-product data-from-mathml='mo'></cross-product>
      <modified-variable data-from-mathml='mover'>
        <mi data-from-mathml='mi' data-latex='B'>B</mi>
        <mo data-from-mathml='mo' stretchy='false'>⇀</mo>
      </modified-variable>
    </mrow>
   </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn dot_product_bold_math_variant() {
  let mathml = r#"<math>
      <mi mathvariant='bold'>x</mi>
      <mo>&#x22C5;</mo>
      <mi mathvariant='bold'>y</mi>
    </math>"#;
  let intent = r#"<math data-from-mathml='math'>
    <mrow data-from-mathml='mrow' data-changed='added'>
      <mi data-from-mathml='mi' mathvariant='bold'>𝐱</mi>
      <dot-product data-from-mathml='mo'></dot-product>
      <mi data-from-mathml='mi' mathvariant='bold'>𝐲</mi>
    </mrow>
   </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn cross_product_hat() {
  let mathml = r#"<math>
  <mrow data-changed='added'>
    <mrow data-changed='added'>
      <mover data-latex='\hat{x}'>
        <mi data-latex='x'>x</mi>
        <mo stretchy='false'>^</mo>
      </mover>
      <mo data-latex='\times'>×</mo>
      <mover data-latex='\hat{y}'>
        <mi data-latex='y'>y</mi>
        <mo stretchy='false'>^</mo>
      </mover>
    </mrow>
    <mo data-latex='='>=</mo>
    <mover data-latex='\hat{z}'>
      <mi data-latex='z'>z</mi>
      <mo stretchy='false'>^</mo>
    </mover>
  </mrow>
 </math>"#;
  let intent = r#" <math data-from-mathml='math'>
      <mrow data-from-mathml='mrow' data-changed='added'>
        <mrow data-from-mathml='mrow' data-changed='added'>
          <modified-variable data-from-mathml='mover' data-latex='\hat{x}'>
            <mi data-from-mathml='mi' data-latex='x'>x</mi>
            <mo data-from-mathml='mo' stretchy='false'>^</mo>
          </modified-variable>
          <cross-product data-from-mathml='mo' data-latex='\times'></cross-product>
          <modified-variable data-from-mathml='mover' data-latex='\hat{y}'>
            <mi data-from-mathml='mi' data-latex='y'>y</mi>
            <mo data-from-mathml='mo' stretchy='false'>^</mo>
          </modified-variable>
        </mrow>
        <mo data-from-mathml='mo' data-latex='='>=</mo>
        <modified-variable data-from-mathml='mover' data-latex='\hat{z}'>
          <mi data-from-mathml='mi' data-latex='z'>z</mi>
          <mo data-from-mathml='mo' stretchy='false'>^</mo>
        </modified-variable>
      </mrow>
    </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn magnetic_flux_dot_product() {
  let mathml = r#"<math>
      <mrow data-changed='added'>
        <msub data-latex='\Phi_B'>
          <mi data-latex='\Phi' mathvariant='normal'>Φ</mi>
          <mi data-latex='B'>B</mi>
        </msub>
        <mo data-latex='='>=</mo>
        <mrow data-changed='added'>
          <mo data-latex='\oint'>∮</mo>
          <mrow data-changed='added'>
            <mi data-latex='B' mathvariant='bold'>𝐁</mi>
            <mo data-latex='\cdot'>⋅</mo>
            <mrow data-changed='added'>
              <mi data-latex='d'>d</mi>
              <mo data-changed='added'>&#x2062;</mo>
              <mi data-latex='A' mathvariant='bold'>𝐀</mi>
            </mrow>
          </mrow>
        </mrow>
      </mrow>
    </math>"#;
  let intent = r#" <math data-from-mathml='math'>
      <mrow data-from-mathml='mrow' data-changed='added'>
        <particular-value-of data-from-mathml='msub' data-latex='\Phi_B'>
          <mi data-from-mathml='mi' data-latex='\Phi' mathvariant='normal'>Φ</mi>
          <mi data-from-mathml='mi' data-latex='B'>B</mi>
        </particular-value-of>
        <mo data-from-mathml='mo' data-latex='='>=</mo>
        <mrow data-from-mathml='mrow' data-changed='added'>
          <mo data-from-mathml='mo' data-latex='\oint'>∮</mo>
          <mrow data-from-mathml='mrow' data-changed='added'>
            <mi data-from-mathml='mi' data-latex='B' mathvariant='bold'>𝐁</mi>
            <dot-product data-from-mathml='mo' data-latex='\cdot'></dot-product>
            <mrow data-from-mathml='mrow' data-changed='added'>
              <mi data-from-mathml='mi' data-latex='d'>d</mi>
              <mo data-from-mathml='mo' data-changed='added'>&#x2062;</mo>
              <mi data-from-mathml='mi' data-latex='A' mathvariant='bold'>𝐀</mi>
            </mrow>
          </mrow>
        </mrow>
      </mrow>
    </math>"#;
    test_intent(mathml, intent, vec![]);
}

#[test]
fn magnetic_field_cross_product() {
  let mathml = r#"<math>
      <mrow data-changed='added'>
        <mrow data-changed='added'>
          <msub data-latex='\mathbf B_2'>
            <mi data-latex='B' mathvariant='bold'>𝐁</mi>
            <mn data-latex='2'>2</mn>
          </msub>
          <mo data-changed='added'>&#x2061;</mo>
          <mrow data-changed='added'>
            <mo data-latex='(' stretchy='false'>(</mo>
            <msub data-latex='\mathbf r_1'>
              <mi data-latex='r' mathvariant='bold'>𝐫</mi>
              <mn data-latex='1'>1</mn>
            </msub>
            <mo data-latex=')' stretchy='false'>)</mo>
          </mrow>
        </mrow>
        <mo data-latex='='>=</mo>
        <mrow data-changed='added'>
          <mfrac data-latex='\frac{\mu_0}{4\pi}'>
            <msub data-latex='\mu_0 '>
              <mi data-latex='\mu'>μ</mi>
              <mn data-latex='0'>0</mn>
            </msub>
            <mrow data-latex='4\pi'>
              <mn data-latex='4'>4</mn>
              <mo data-changed='added'>&#x2062;</mo>
              <mi data-latex='\pi'>π</mi>
            </mrow>
          </mfrac>
          <mo data-changed='added'>&#x2062;</mo>
          <mrow data-changed='added'>
            <munder data-latex='\limits_{C_2}'>
              <mo data-latex='\limits'>∮</mo>
              <msub data-latex='C_2'>
                <mi data-latex='C'>C</mi>
                <mn data-latex='2'>2</mn>
              </msub>
            </munder>
            <mrow data-changed='added'>
              <msub data-latex='I_2'>
                <mi data-latex='I'>I</mi>
                <mn data-latex='2'>2</mn>
              </msub>
              <mo data-changed='added'>&#x2062;</mo>
              <mi data-latex='d'>d</mi>
              <mo data-changed='added'>&#x2062;</mo>
              <msub data-latex='\mathbf s_2'>
                <mi data-latex='s' mathvariant='bold'>𝐬</mi>
                <mn data-latex='2'>2</mn>
              </msub>
              <mo data-latex='\times'>×</mo>
              <mfrac data-latex='\frac{\mathbf r_1 - \mathbf r_2}{|\mathbf r_1 - \mathbf r_2|^3}'>
                <mrow data-latex='\mathbf r_1  - \mathbf r_2 '>
                  <msub data-latex='\mathbf r_1'>
                    <mi data-latex='r' mathvariant='bold'>𝐫</mi>
                    <mn data-latex='1'>1</mn>
                  </msub>
                  <mo data-latex='-'>-</mo>
                  <msub data-latex='\mathbf r_2'>
                    <mi data-latex='r' mathvariant='bold'>𝐫</mi>
                    <mn data-latex='2'>2</mn>
                  </msub>
                </mrow>
                <msup data-latex='|\mathbf r_1  - \mathbf r_2 |^3 '>
                  <mrow data-changed='added'>
                    <mo data-latex='|' stretchy='false'>|</mo>
                    <mrow data-changed='added'>
                      <msub data-latex='\mathbf r_1'>
                        <mi data-latex='r' mathvariant='bold'>𝐫</mi>
                        <mn data-latex='1'>1</mn>
                      </msub>
                      <mo data-latex='-'>-</mo>
                      <msub data-latex='\mathbf r_2'>
                        <mi data-latex='r' mathvariant='bold'>𝐫</mi>
                        <mn data-latex='2'>2</mn>
                      </msub>
                    </mrow>
                    <mo data-latex='|' stretchy='false'>|</mo>
                  </mrow>
                  <mn data-latex='3'>3</mn>
                </msup>
              </mfrac>
            </mrow>
          </mrow>
        </mrow>
      </mrow>
    </math>"#;
  let intent = r#" <math data-from-mathml='math'>
      <mrow data-from-mathml='mrow' data-changed='added'>
        <mrow data-from-mathml='mrow' data-changed='added'>
          <particular-value-of data-from-mathml='msub' data-latex='\mathbf B_2'>
            <mi data-from-mathml='mi' data-latex='B' mathvariant='bold'>𝐁</mi>
            <mn data-from-mathml='mn' data-latex='2'>2</mn>
          </particular-value-of>
          <mo data-from-mathml='mo' data-changed='added'>&#x2061;</mo>
          <mrow data-from-mathml='mrow' data-changed='added'>
            <mo data-from-mathml='mo' data-latex='(' stretchy='false'>(</mo>
            <particular-value-of data-from-mathml='msub' data-latex='\mathbf r_1'>
              <mi data-from-mathml='mi' data-latex='r' mathvariant='bold'>𝐫</mi>
              <mn data-from-mathml='mn' data-latex='1'>1</mn>
            </particular-value-of>
            <mo data-from-mathml='mo' data-latex=')' stretchy='false'>)</mo>
          </mrow>
        </mrow>
        <mo data-from-mathml='mo' data-latex='='>=</mo>
        <mrow data-from-mathml='mrow' data-changed='added'>
          <fraction data-from-mathml='mfrac' data-latex='\frac{\mu_0}{4\pi}'>
            <particular-value-of data-from-mathml='msub' data-latex='\mu_0 '>
              <mi data-from-mathml='mi' data-latex='\mu'>μ</mi>
              <mn data-from-mathml='mn' data-latex='0'>0</mn>
            </particular-value-of>
            <mrow data-from-mathml='mrow' data-latex='4\pi'>
              <mn data-from-mathml='mn' data-latex='4'>4</mn>
              <mo data-from-mathml='mo' data-changed='added'>&#x2062;</mo>
              <mi data-from-mathml='mi' data-latex='\pi'>π</mi>
            </mrow>
          </fraction>
          <mo data-from-mathml='mo' data-changed='added'>&#x2062;</mo>
          <mrow data-from-mathml='mrow' data-changed='added'>
            <large-op data-from-mathml='munder' data-latex='\limits_{C_2}'>
              <mo data-from-mathml='mo' data-latex='\limits'>∮</mo>
              <particular-value-of data-from-mathml='msub' data-latex='C_2'>
                <mi data-from-mathml='mi' data-latex='C'>C</mi>
                <mn data-from-mathml='mn' data-latex='2'>2</mn>
              </particular-value-of>
            </large-op>
            <mrow data-from-mathml='mrow' data-changed='added'>
              <particular-value-of data-from-mathml='msub' data-latex='I_2'>
                <mi data-from-mathml='mi' data-latex='I'>I</mi>
                <mn data-from-mathml='mn' data-latex='2'>2</mn>
              </particular-value-of>
              <mo data-from-mathml='mo' data-changed='added'>&#x2062;</mo>
              <mi data-from-mathml='mi' data-latex='d'>d</mi>
              <mo data-from-mathml='mo' data-changed='added'>&#x2062;</mo>
              <particular-value-of data-from-mathml='msub' data-latex='\mathbf s_2'>
                <mi data-from-mathml='mi' data-latex='s' mathvariant='bold'>𝐬</mi>
                <mn data-from-mathml='mn' data-latex='2'>2</mn>
              </particular-value-of>
              <cross-product data-from-mathml='mo' data-latex='\times'></cross-product>
              <fraction data-from-mathml='mfrac' data-latex='\frac{\mathbf r_1 - \mathbf r_2}{|\mathbf r_1 - \mathbf r_2|^3}'>
                <mrow data-from-mathml='mrow' data-latex='\mathbf r_1  - \mathbf r_2 '>
                  <particular-value-of data-from-mathml='msub' data-latex='\mathbf r_1'>
                    <mi data-from-mathml='mi' data-latex='r' mathvariant='bold'>𝐫</mi>
                    <mn data-from-mathml='mn' data-latex='1'>1</mn>
                  </particular-value-of>
                  <mo data-from-mathml='mo' data-latex='-'>-</mo>
                  <particular-value-of data-from-mathml='msub' data-latex='\mathbf r_2'>
                    <mi data-from-mathml='mi' data-latex='r' mathvariant='bold'>𝐫</mi>
                    <mn data-from-mathml='mn' data-latex='2'>2</mn>
                  </particular-value-of>
                </mrow>
                <power data-from-mathml='msup' data-latex='|\mathbf r_1  - \mathbf r_2 |^3 '>
                  <absolute-value data-from-mathml='mrow' data-changed='added'>
                    <mrow data-from-mathml='mrow' data-changed='added'>
                      <particular-value-of data-from-mathml='msub' data-latex='\mathbf r_1'>
                        <mi data-from-mathml='mi' data-latex='r' mathvariant='bold'>𝐫</mi>
                        <mn data-from-mathml='mn' data-latex='1'>1</mn>
                      </particular-value-of>
                      <mo data-from-mathml='mo' data-latex='-'>-</mo>
                      <particular-value-of data-from-mathml='msub' data-latex='\mathbf r_2'>
                        <mi data-from-mathml='mi' data-latex='r' mathvariant='bold'>𝐫</mi>
                        <mn data-from-mathml='mn' data-latex='2'>2</mn>
                      </particular-value-of>
                    </mrow>
                  </absolute-value>
                  <mn data-from-mathml='mn' data-latex='3'>3</mn>
                </power>
              </fraction>
            </mrow>
          </mrow>
        </mrow>
      </mrow>
    </math>
    "#;
    test_intent(mathml, intent, vec![]);
}
