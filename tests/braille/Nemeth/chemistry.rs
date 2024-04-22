// Nemeth tests from the 2023 chemistry spec
//  https://www.brailleauthority.org/sites/default/files/chemistry/Chemical%20Notation%20Using%20the%20Nemeth%20Braille%20Code%202023.pdf
// The numbering refers to the sections in that reference.

// The MathML comes from ChemType (basically WIRIS's tweak to the online MathType editor) or mhchem (indicated in test name)
use crate::common::*;

#[test]
fn bond_2_1() {
    let expr = r#"<math><mi mathvariant="normal">H</mi><mo>-</mo><mi>Be</mi><mo>-</mo><mi mathvariant="normal">H</mi></math>"#;
    test_braille("Nemeth", expr, "⠠⠓⠸⠒⠻⠠⠃⠑⠸⠒⠻⠠⠓");
}

#[test]
fn bond_2_2() {
    let expr = r#"<math><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">C</mi><mo>=</mo>
                        <msub><mi>CH</mi><mn>2</mn></msub></math>"#;
    test_braille("Nemeth", expr, "⠠⠓⠆⠠⠉⠸⠶⠻⠠⠉⠠⠓⠆");
}

#[test]
fn bond_2_2_mhchem() {
    let expr = r#"<math>
    <mrow data-mjx-texclass="ORD">
      <mi mathvariant="normal">H</mi>
      <msub>
        <mpadded width="0">
          <mphantom>
            <mi>A</mi>
          </mphantom>
        </mpadded>
        <mpadded height="0">
          <mn>2</mn>
        </mpadded>
      </msub>
      <mi mathvariant="normal">C</mi>
      <mo>=</mo>
      <mi data-mjx-auto-op="false">CH</mi>
      <msub>
        <mpadded width="0">
          <mphantom>
            <mi>A</mi>
          </mphantom>
        </mpadded>
        <mpadded height="0">
          <mn>2</mn>
        </mpadded>
      </msub>
    </mrow>
  </math>"#;
    test_braille("Nemeth", expr, "⠠⠓⠆⠠⠉⠸⠶⠻⠠⠉⠠⠓⠆");
}

#[test]
fn bond_2_3() {
    let expr = r#"<math><mi mathvariant="normal">H</mi><mo>-</mo><mi mathvariant="normal">C</mi><mo>&#x2261;</mo>
                        <mi mathvariant="normal">C</mi><mo>-</mo><mi mathvariant="normal">H</mi></math>"#;
    test_braille("Nemeth", expr, "⠠⠓⠸⠒⠻⠠⠉⠸⠿⠻⠠⠉⠸⠒⠻⠠⠓");
}

#[test]
fn bond_2_5() {
    let expr = r#"<math><mi>Fe</mi><mo>+</mo><msub><mi>Cl</mi><mn>2</mn></msub><mo>&#xA0;</mo><mo>=</mo>
                            <msub><mi>FeCl</mi><mn>3</mn></msub></math>"#;
    test_braille("Nemeth", expr, "⠠⠋⠑⠬⠠⠉⠇⠆⠀⠨⠅⠀⠠⠋⠑⠠⠉⠇⠒");
}


// FIX: add tests for 2.2.2: Horizontal Arrow Bonds (these are not currently known by MathCAT)


#[test]
fn arrow_bond_2_2_2_c() {
    let expr = r#"<math><mn>2</mn><mi>Na</mi><mo>+</mo><msub><mi>Cl</mi><mn>2</mn></msub><mo>&#x2192;</mo>
                            <mn>2</mn><mi>NaCl</mi></math>"#;
    test_braille("Nemeth", expr, "⠼⠆⠠⠝⠁⠬⠠⠉⠇⠆⠀⠫⠕⠀⠼⠆⠠⠝⠁⠠⠉⠇");
}

#[test]
fn arrow_bond_2_2_2_d() {
    let expr = r#"<math>
         <msub><mi>RNH</mi><mn>2</mn></msub><mo>+</mo><mi>RX</mi><mo>&#x2192;</mo>
         <msub><mi mathvariant="normal">R</mi><mn>2</mn></msub>
            <msup><msub><mi>NH</mi><mn>2</mn></msub><mo>+</mo></msup><msup><mi mathvariant="normal">X</mi><mo>-</mo></msup></math>"#;
    test_braille("Nemeth", expr, "⠠⠗⠠⠝⠠⠓⠆⠬⠠⠗⠠⠭⠀⠫⠕⠀⠠⠗⠆⠠⠝⠠⠓⠆⠐⠘⠬⠐⠠⠭⠘⠤");
}

#[test]
fn arrow_bond_2_2_2_d_mhchem() {
    let expr = r#"<math>
    <mrow data-mjx-texclass="ORD">
      <mi data-mjx-auto-op="false">RNH</mi>
      <msub>
        <mpadded width="0">
          <mphantom>
            <mi>A</mi>
          </mphantom>
        </mpadded>
        <mpadded height="0">
          <mn>2</mn>
        </mpadded>
      </msub>
      <mrow data-mjx-texclass="ORD">
  
      </mrow>
      <mo>+</mo>
      <mrow data-mjx-texclass="ORD">
  
      </mrow>
      <mi data-mjx-auto-op="false">RX</mi>
      <mrow data-mjx-texclass="ORD">
  
      </mrow>
      <mo stretchy="false">&#x27F6;</mo>
      <mrow data-mjx-texclass="ORD">
  
      </mrow>
      <mi mathvariant="normal">R</mi>
      <msub>
        <mpadded width="0">
          <mphantom>
            <mi>A</mi>
          </mphantom>
        </mpadded>
        <mpadded height="0">
          <mn>2</mn>
        </mpadded>
      </msub>
      <mtext>&#xA0;</mtext>
      <mi data-mjx-auto-op="false">NH</mi>
      <msub>
        <mpadded width="0">
          <mphantom>
            <mi>A</mi>
          </mphantom>
        </mpadded>
        <mpadded height="0">
          <mn>2</mn>
        </mpadded>
      </msub>
      <msup>
        <mpadded width="0">
          <mphantom>
            <mi>A</mi>
          </mphantom>
        </mpadded>
        <mo>+</mo>
      </msup>
      <mtext>&#xA0;</mtext>
      <mi mathvariant="normal">X</mi>
      <msup>
        <mpadded width="0">
          <mphantom>
            <mi>A</mi>
          </mphantom>
        </mpadded>
        <mo>&#x2212;</mo>
      </msup>
    </mrow>
  </math>"#;
    test_braille("Nemeth", expr, "⠠⠗⠠⠝⠠⠓⠆⠬⠠⠗⠠⠭⠀⠫⠕⠀⠠⠗⠆⠠⠝⠠⠓⠆⠐⠘⠬⠐⠠⠭⠘⠤");
}

#[test]
fn lewis_2_20() {
    let expr = r#"<math><msub><mi>HO</mi><mn>2</mn></msub><mo>+</mo><mi>NO</mi><mo>&#x2192;</mo>
                            <msub><mi>NO</mi><mn>2</mn></msub><mo>+</mo><mo>&#x2022;</mo><mi>OH</mi></math>"#;
    test_braille("Nemeth", expr, "⠠⠓⠠⠕⠆⠬⠠⠝⠠⠕⠀⠫⠕⠀⠠⠝⠠⠕⠆⠬⠡⠠⠕⠠⠓");
}

#[test]
fn lewis_2_21() {
    let expr = r#"<math><msup><mi mathvariant="normal">R</mi><mo>&#x2022;</mo></msup><mo>+</mo><mmultiscripts><mi>CH</mi><mn>3</mn><none/><mprescripts/><none/><mo>&#x2022;</mo>
                             </mmultiscripts><mo>&#x2192;</mo><msub><mi>RCH</mi><mn>3</mn></msub></math>"#;
    test_braille("Nemeth", expr, "⠠⠗⠡⠬⠡⠠⠉⠠⠓⠒⠀⠫⠕⠀⠠⠗⠠⠉⠠⠓⠒");
}

#[test]
fn lewis_2_22() {
    let expr = r#"<math><msup><mrow><mo>[</mo><msub><mi>CH</mi><mn>3</mn></msub><mo>]</mo></mrow><mo>&#x2022;</mo></msup></math>"#;
    test_braille("Nemeth", expr, "⠈⠷⠠⠉⠠⠓⠒⠈⠾⠡");
}
