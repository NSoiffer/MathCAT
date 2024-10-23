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
fn arrow_bond_sec_2_2_2_c() {
    let expr = r#"<math><mn>2</mn><mi>Na</mi><mo>+</mo><msub><mi>Cl</mi><mn>2</mn></msub><mo>&#x2192;</mo>
                            <mn>2</mn><mi>NaCl</mi></math>"#;
    test_braille("Nemeth", expr, "⠼⠆⠠⠝⠁⠬⠠⠉⠇⠆⠀⠫⠕⠀⠼⠆⠠⠝⠁⠠⠉⠇");
}

#[test]
fn arrow_bond_sec_2_2_2_d() {
    let expr = r#"<math>
         <msub><mi>RNH</mi><mn>2</mn></msub><mo>+</mo><mi>RX</mi><mo>&#x2192;</mo>
         <msub><mi mathvariant="normal">R</mi><mn>2</mn></msub>
            <msup><msub><mi>NH</mi><mn>2</mn></msub><mo>+</mo></msup><msup><mi mathvariant="normal">X</mi><mo>-</mo></msup></math>"#;
    test_braille("Nemeth", expr, "⠠⠗⠠⠝⠠⠓⠆⠬⠠⠗⠠⠭⠀⠫⠕⠀⠠⠗⠆⠠⠝⠠⠓⠆⠐⠘⠬⠐⠠⠭⠘⠤");
}

#[test]
fn arrow_bond_sec_2_2_2_d_mhchem() {
    // changed long right arrow to short right arrow as that is what the example (wrongly) uses
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
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
        <mo stretchy="false">→</mo>
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

#[test]
fn dots_2_37_a() {
    let expr = r#"<math><mo>:</mo><mi mathvariant="normal">N</mi><mo>:</mo><mo>:</mo><mo>:</mo><mi mathvariant="normal">N</mi><mo>:</mo></math>"#;
    test_braille("Nemeth", expr, "⠹⠠⠝⠨⠹⠠⠝⠹");
}

#[test]
fn dots_2_37_b() {
    // Note: this uses vertical ellipsis for the triple vertical dot. There is also U+205D, but's pretty ubscure.
    let expr = r#"<math><mi mathvariant="normal">H</mi><mo>:</mo><mi mathvariant="normal">C</mi>
                    <mo>&#x22EE;</mo><mo>&#x22EE;</mo>
                    <mi mathvariant="normal">C</mi><mo>:</mo><mi mathvariant="normal">H</mi></math>"#;
    test_braille("Nemeth", expr, "⠠⠓⠹⠠⠉⠨⠹⠠⠉⠹⠠⠓");
}

#[test]
fn yields_3_1() {
    let expr = r#"<math><msub><mi>CH</mi><mn>4</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo><mo>+</mo>
                <mn>2</mn><msub><mi mathvariant="normal">O</mi><mn>2</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo>
                <mo>&#x2192;</mo>
                <msub><mi>CO</mi><mn>2</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo><mo>+</mo>
                <mn>2</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo></math>"#;
    test_braille("Nemeth", expr, "⠠⠉⠠⠓⠲⠷⠛⠾⠬⠆⠠⠕⠆⠷⠛⠾⠀⠫⠕⠀⠠⠉⠠⠕⠆⠷⠛⠾⠬⠆⠠⠓⠆⠠⠕⠷⠛⠾");
}

#[test]
fn reverse_3_2() {
    let expr = r#"<math><msub><mi mathvariant="normal">N</mi><mn>2</mn></msub><mo>+</mo>
                    <mn>3</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mo>&#x2190;</mo>
                    <mn>2</mn><msub><mi>NH</mi><mn>3</mn></msub></math>"#;
    test_braille("Nemeth", expr, "⠠⠝⠆⠬⠒⠠⠓⠆⠀⠫⠪⠒⠒⠀⠼⠆⠠⠝⠠⠓⠒");
}

#[test]
fn reaction_arrow_3_9() {
    let expr = r#"<math><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo><mo>+</mo>
                <msub><mi mathvariant="normal">I</mi><mn>2</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo><mo>&#x21CC;</mo>
                <mn>2</mn><mi>HI</mi><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo></math>"#;
    test_braille("Nemeth", expr, "⠠⠓⠆⠷⠛⠾⠬⠠⠊⠆⠷⠛⠾⠀⠫⠒⠕⠫⠪⠒⠀⠼⠆⠠⠓⠠⠊⠷⠛⠾");
}

#[test]
fn reaction_arrow_3_13() {
    let expr = r#"<math><msub><mi mathvariant="normal">H</mi><mn>3</mn></msub><mi mathvariant="normal">C</mi><mo>-</mo>
            <mi mathvariant="normal">C</mi><mo>≡</mo>
            <msup><mi mathvariant="normal">C</mi><mo>-</mo></msup><msup><mi>Na</mi><mo>+</mo>
            </msup><mo>+</mo><msub><mi>NH</mi><mn>3</mn></msub><mo>⥃</mo>
            <msub><mi mathvariant="normal">H</mi><mn>3</mn></msub><mi mathvariant="normal">C</mi><mo>-</mo>
            <mi mathvariant="normal">C</mi><mo>≡</mo><mi mathvariant="normal">C</mi><mo>-</mo>
            <mi mathvariant="normal">H</mi><mo>+</mo>
            <msup><mi>Na</mi><mo>+</mo></msup><msup><msub><mi>NH</mi><mn>2</mn></msub><mo>-</mo></msup></math>"#;
    test_braille("Nemeth", expr, "⠠⠓⠒⠠⠉⠸⠒⠻⠠⠉⠸⠿⠻⠠⠉⠘⠤⠐⠠⠝⠁⠘⠬⠐⠬⠠⠝⠠⠓⠒⠀⠫⠪⠒⠒⠫⠒⠕⠀⠠⠓⠒⠠⠉⠸⠒⠻⠠⠉⠸⠿⠻⠠⠉⠸⠒⠻⠠⠓⠬⠠⠝⠁⠘⠬⠐⠠⠝⠠⠓⠆⠐⠘⠤");
}

#[test]
fn reaction_arrow_3_14_a() {
    // Note: this uses an arrow rather than harpoon because Unicode currently lacks the symbol (braille should be the same)
    // There's not enough chemistry to pick up (value=3), so intent is added
    let expr = r#"<math intent=':chemical-equation'><mi mathvariant="normal">R</mi><mo>⥂</mo><mi mathvariant="normal">P</mi></math>"#;
    test_braille("Nemeth", expr, "⠠⠗⠀⠫⠒⠒⠕⠫⠪⠒⠀⠠⠏");
}

#[test]
fn reaction_arrow_3_14_b() {
    // Note: this uses an arrow rather than harpoon because Unicode currently lacks the symbol (braille should be the same)
    // There's not enough chemistry to pick up (value=3), so intent is added
    let expr = r#"<math intent=':chemical-equation'><mi mathvariant="normal">R</mi><mo>⥄</mo><mi mathvariant="normal">P</mi></math>"#;
    test_braille("Nemeth", expr, "⠠⠗⠀⠫⠒⠕⠫⠪⠒⠒⠀⠠⠏");
}

#[test]
fn arrow_3_36() {
    let expr = r#"<math><mn>2</mn><mi>Al</mi><mo>+</mo><mn>2</mn><mi>NaOH</mi><mo>+</mo>
                <mn>2</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi><mo>&#x2192;</mo>
                <msub><mi>Na</mi><mn>2</mn></msub><msub><mi>Al</mi><mn>2</mn></msub><msub><mi mathvariant="normal">O</mi><mn>4</mn></msub><mo>+</mo>
                <mn>3</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mo>↑</mo></math>"#;
    test_braille("Nemeth", expr, "⠼⠆⠠⠁⠇⠬⠆⠠⠝⠁⠠⠕⠠⠓⠬⠆⠠⠓⠆⠠⠕⠀⠫⠕⠀⠠⠝⠁⠆⠠⠁⠇⠆⠠⠕⠲⠬⠒⠠⠓⠆⠫⠣");
}

#[test]
fn arrow_3_37() {
    let expr = r#"<math><mi>Ca</mi><msub><mrow><mo>(</mo><msub><mi>HCO</mi><mn>3</mn></msub><mo>)</mo></mrow><mn>2</mn></msub><mo>+</mo>
            <mi>Ca</mi><msub><mrow><mo>(</mo><mi>OH</mi><mo>)</mo></mrow><mn>2</mn></msub><mo>&#x2192;</mo>
            <mn>2</mn><msub><mi>CaCO</mi><mn>3</mn></msub><mo>&#x2193;</mo><mo>+</mo>
            <mn>2</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi></math>"#;
    test_braille("Nemeth", expr, "⠠⠉⠁⠷⠠⠓⠠⠉⠠⠕⠒⠾⠰⠆⠐⠬⠠⠉⠁⠷⠠⠕⠠⠓⠾⠰⠆⠀⠫⠕⠀⠼⠆⠠⠉⠁⠠⠉⠠⠕⠒⠫⠩⠀⠬⠆⠠⠓⠆⠠⠕");
}

#[test]
fn dot_4_5() {
    let expr = r#"<math><msub><mi mathvariant="normal">K</mi><mn>1</mn></msub>
            <mo>[</mo><msub><mi>PCl</mi><mn>5</mn></msub><mo>]</mo><mo>=</mo>
            <msub><mi mathvariant="normal">K</mi><mn>2</mn></msub><mo>[</mo><msub><mi>PCl</mi><mn>3</mn></msub><mo>]</mo><mo>&#xB7;</mo>
            <mo>[</mo><msub><mi>Cl</mi><mn>2</mn></msub><mo>]</mo></math>"#;
    test_braille("Nemeth", expr, "⠠⠅⠂⠈⠷⠠⠏⠠⠉⠇⠢⠈⠾⠀⠨⠅⠀⠠⠅⠆⠈⠷⠠⠏⠠⠉⠇⠒⠈⠾⠡⠈⠷⠠⠉⠇⠆⠈⠾");
}

#[test]
fn charge_5_1_c() {
    let expr = r#"<math><mn>2</mn><mi>Al</mi><mo>&#x2192;</mo>
                <mn>2</mn><msup><mi>Al</mi><mrow><mn>3</mn><mo>+</mo></mrow></msup><mo>+</mo>
                <mn>6</mn><msup><mi mathvariant="normal">e</mi><mo>-</mo></msup></math>"#;
    test_braille("Nemeth", expr, "⠼⠆⠠⠁⠇⠀⠫⠕⠀⠼⠆⠠⠁⠇⠘⠒⠬⠐⠬⠖⠑⠘⠤");
}

#[test]
fn charge_5_3_a() {
    let expr = r#"<math><msup><msub><mi>HPO</mi><mn>4</mn></msub><mrow><mo>-</mo><mo>-</mo></mrow></msup></math>"#;
    test_braille("Nemeth", expr, "⠠⠓⠠⠏⠠⠕⠲⠐⠘⠆⠤");
}

#[test]
fn charge_5_3_b() {
    let expr = r#"<math><msup>
        <mmultiscripts><mi>He</mi><mprescripts/><mn>2</mn><mn>4</mn></mmultiscripts>
        <mrow><mo>+</mo><mo>+</mo></mrow></msup></math>"#;
    test_braille("Nemeth", expr, "⠰⠆⠘⠲⠐⠠⠓⠑⠘⠆⠬");
}

#[test]
fn simultaneious_scripts_5_8_b() {
    let expr = r#"<math><msubsup><mi>SO</mi><mn>4</mn><mrow><mn>2</mn><mo>-</mo></mrow></msubsup></math>"#;
    test_braille("Nemeth", expr, "⠠⠎⠠⠕⠲⠘⠆⠤");
}

#[test]
fn staggered_scripts_5_9_a() {
    let expr = r#"<math><mn>5</mn><msub><mi mathvariant="normal">C</mi><mn>2</mn></msub>
            <msup><msub><mi mathvariant="normal">O</mi><mn>4</mn></msub><mrow><mn>2</mn><mo>-</mo></mrow></msup>
            <mo>(</mo><mi>a</mi><mi>q</mi><mo>)</mo></math>"#;
    test_braille("Nemeth", expr, "⠼⠢⠠⠉⠆⠠⠕⠲⠐⠘⠆⠤⠐⠷⠁⠟⠾");
}

#[test]
fn staggered_scripts_5_9_b() {
    let expr = r#"<math><msup><msub><mrow><mo>(</mo><msub><mi>SiO</mi><mn>3</mn></msub><mo>)</mo></mrow>
                    <mi>n</mi></msub>
                    <mrow><mn>2</mn><mi>n</mi><mo>-</mo></mrow></msup></math>"#;
    test_braille("Nemeth", expr, "⠷⠠⠎⠊⠠⠕⠒⠾⠰⠝⠐⠘⠆⠝⠤");
}

#[test]
fn state_5_10() {
    let expr = r#"<math><msubsup><mrow><mo>&#x2206;</mo><mi mathvariant="normal">H</mi></mrow><mrow><mo>(</mo><mi>reaction</mi><mo>)</mo></mrow><mo>&#xB0;</mo></msubsup></math>"#;
    // the chem book seems to have a bug in using dot 5 in the first delta expr. They have "⠨⠠⠙⠠⠓⠘⠨⠡⠐⠰⠷⠗⠑⠁⠉⠞⠊⠕⠝⠾".
    // I sent a bug report in bug haven't heard back. This tests assumes the book has a bug
    test_braille("Nemeth", expr, "⠨⠠⠙⠠⠓⠘⠨⠡⠰⠷⠗⠑⠁⠉⠞⠊⠕⠝⠾");
}

#[test]
fn state_as_subscript_5_13() {
    let expr = r#"<math><msubsup><mi>Sn</mi>
                            <mrow><mo>(</mo><mi>aq</mi><mo>)</mo></mrow>
                            <mrow><mn>2</mn><mo>+</mo></mrow></msubsup></math>"#;
    test_braille("Nemeth", expr, "⠠⠎⠝⠘⠆⠬⠐⠷⠁⠟⠾");
}

#[test]
fn prescripts_5_14() {
    let expr = r#"<math><mmultiscripts><mi>Es</mi><mprescripts/><mn>99</mn><mn>254</mn></mmultiscripts><mo>+</mo>
                <mmultiscripts><mi>He</mi><mprescripts/><mn>2</mn><mn>4</mn></mmultiscripts><mo>&#x2192;</mo>
                <mmultiscripts><mi>Md</mi><mprescripts/><mn>101</mn><mn>256</mn></mmultiscripts><mo>+</mo>
                <mn>2</mn><mmultiscripts><mi>n</mi><mprescripts/><mn>0</mn><mn>1</mn></mmultiscripts></math>"#;
    test_braille("Nemeth", expr, "⠰⠔⠔⠘⠆⠢⠲⠐⠠⠑⠎⠬⠰⠆⠘⠲⠐⠠⠓⠑⠀⠫⠕⠀⠰⠂⠴⠂⠘⠆⠢⠖⠐⠠⠍⠙⠬⠆⠰⠴⠘⠂⠐⠝");
}
