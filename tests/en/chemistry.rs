/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;

#[test]
fn salt() {
    let expr = "<math><mi>Na</mi><mi>Cl</mi></math>";
    test("SimpleSpeak", expr, "N a C l");
}

#[test]
fn water() {
    let expr = "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>";
    test("ClearSpeak", expr, "H 2 O");
}

#[test]
fn carbon() {
    let expr = "<math><mi>C</mi></math>";     // not enough to trigger recognition
    test("SimpleSpeak", expr, "C");
}

#[test]
fn sulfate() {
    let expr = "<math><mrow><msup>
            <mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
            <mrow><mn>2</mn><mo>&#x2212;</mo></mrow>
        </msup></mrow></math>";
    test("ClearSpeak", expr, "open bracket S O 4 close bracket 2 minus");
}

#[test]
fn aluminum_sulfate() {
    let expr = "<math><mrow><msub><mi>Al</mi><mn>2</mn></msub>
            <msub><mrow><mo>(</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>)</mo></mrow><mn>3</mn></msub></mrow></math>";
    test("SimpleSpeak", expr, "A l 2, open paren S O 4 close paren 3");
}

#[test]
fn ethanol_bonds() {
    let expr = "<math>
            <mrow>
                <mi>C</mi>
                <msub>  <mi>H</mi> <mn>3</mn> </msub>
                <mo>&#x2212;</mo>
                <mi>C</mi>
                <msub>  <mi>H</mi> <mn>2</mn> </msub>
                <mo>&#x2212;</mo>
                <mi>O</mi>
                <mi>H</mi>
            </mrow>
        </math>";
    test("ClearSpeak", expr, "C, H 3 single bond, C H 2 single bond O H");
}

#[test]
fn dichlorine_hexoxide() {
    let expr = "<math><mrow>
    <msup>
      <mrow><mo>[</mo><mi>Cl</mi><msub><mi>O</mi><mn>2</mn></msub><mo>]</mo></mrow>
      <mo>+</mo>
    </msup>
    <msup>
      <mrow><mo>[</mo><mi>Cl</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
      <mo>-</mo>
    </msup>
  </mrow></math>";
    test("SimpleSpeak", expr, "open bracket C l O 2 close bracket plus; \
                                                    open bracket C l O 4 close bracket minus");
}


#[test]
fn ethylene_with_bond() {
    let expr = "<math><mrow>
            <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
            <mo>=</mo>
            <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
        </mrow></math>";
    test("SimpleSpeak", expr, "H 2 C double bond C H 2");
}

#[test]
fn ferric_chloride_aq() {
    let expr = "<math><mrow>
        <mi>Fe</mi>
        <msub><mi>Cl</mi><mn>3</mn></msub>
        <mrow><mo>(</mo><mrow><mi>aq</mi></mrow><mo>)</mo></mrow>
    </mrow></math>";
    test("SimpleSpeak", expr, "F e C l 3 dissolved in water");
}

#[test]
fn ethylene_with_colon_bond() {
    let expr = "<math><mrow>
            <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
            <mo>::</mo>
            <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
        </mrow></math>";
    test("SimpleSpeak", expr, "H 2 C double bond C H 2");
}

#[test]
fn beta_decay() {
    init_logger();
    let expr = "<math>
    <mmultiscripts>
      <mtext>C</mtext>
      <mprescripts />
      <mn>6</mn>
      <mn>14</mn>
    </mmultiscripts>
    <mo>&#x2192;</mo>
    <mmultiscripts>
      <mtext>N</mtext>
      <mprescripts />
      <mn>7</mn>
      <mn>14</mn>
    </mmultiscripts>
    <mo>+</mo>
    <mmultiscripts>
      <mtext>e</mtext>
      <mprescripts />
      <mrow>
        <mo>&#x2212;</mo>
        <mn>1</mn>
      </mrow>
      <mn>0</mn>
    </mmultiscripts>
  </math>";
    test("SimpleSpeak", expr, "14 6 C decays to 14 7 N plus 0 -1 electrons");
}

