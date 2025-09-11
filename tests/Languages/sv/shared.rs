/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;

#[test]
fn modified_vars() {
    let expr = "<math> <mrow>
        <mover> <mi>a</mi> <mo>`</mo> </mover>
        <mover> <mi>b</mi> <mo>~</mo> </mover>
        <mover> <mi>c</mi> <mo>&#x0306;</mo> </mover>
        <mover> <mi>b</mi> <mo>&#x030c;</mo> </mover>
        <mover> <mi>c</mi> <mo>`</mo> </mover>  <mo>+</mo>
        <mover> <mi>x</mi> <mo>.</mo> </mover>
        <mover> <mi>y</mi> <mo>&#x2D9;</mo> </mover>
        <mover> <mi>z</mi> <mo>&#x00A8;</mo> </mover>
        <mover> <mi>u</mi> <mo>&#x20DB;</mo> </mover>
        <mover> <mi>v</mi> <mo>&#x20DC;</mo> </mover> <mo>+</mo>
        <mover> <mi>x</mi> <mo>^</mo> </mover> <mo>+</mo>
        <mover> <mi>t</mi> <mo>→</mo> </mover>
        </mrow> </math>";
    test("sv", "SimpleSpeak", expr, 
        "a grav accent, b tilde, c hake, b hake, c grav accent; plus; x prick, y prick, z prick prick, u trippel prick; v fyra prickar; plus x hatt, plus vektorn t");
}

#[test]
fn limit() {
    let expr = "<math>
            <munder>
            <mo>lim</mo>
            <mrow>  <mi>x</mi> <mo>&#x2192;</mo>  <mn>0</mn>  </mrow>
            </munder>
            <mrow>
            <mfrac>
                <mrow>  <mi>sin</mi>  <mo>&#x2061;</mo> <mi>x</mi> </mrow>
                <mi>x</mi>
            </mfrac>
            </mrow>
        </math>";
    test("sv", "SimpleSpeak", expr, "gränsvärdet då x går mot 0, av, division, sinus av x, genom x, slut division");
}

#[test]
fn limit_from_below() {
    let expr = "<math>
            <munder>
            <mo>lim</mo>
            <mrow>  <mi>x</mi> <mo>↗</mo>  <mn>0</mn>  </mrow>
            </munder>
            <mrow>
                <mrow>  <mi>sin</mi>  <mo>&#x2061;</mo> <mi>x</mi> </mrow>
            </mrow>
        </math>";
    test("sv", "SimpleSpeak", expr, "gränsvärdet då x går vänsterifrån mot 0, av sinus av x");
}


#[test]
fn binomial_mmultiscripts() {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("sv", "SimpleSpeak", expr, "n över m");
}


#[test]
fn permutation_mmultiscripts() {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("sv", "SimpleSpeak", expr, "antalet permutationer av k element ur n");
}

#[test]
fn permutation_mmultiscripts_sup() {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("sv", "SimpleSpeak", expr, "antalet permutationer av k element ur n");
}

#[test]
fn permutation_msubsup() {
    let expr = "<math><msubsup><mi>P</mi><mi>k</mi><mi>n</mi></msubsup></math>";
    test("sv", "SimpleSpeak", expr, "antalet permutationer av k element ur n");
}

#[test]
fn tensor_mmultiscripts() {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> 
        </mmultiscripts></math>";
    test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "versal r med 4 höger index, nedsänkt i upphöjt j nedsänkt k nedsänkt l");
    test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Medium")], expr,
            "versal r med 4 höger index, nedsänkt i upphöjt j nedsänkt k nedsänkt l");
}

#[test]
fn huge_num_mmultiscripts() {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> <mi>m</mi><none/>
            <mprescripts/> <mi>I</mi><none/> <none/><mi>J</mi> <mi>K</mi><none/> <mi>L</mi><none/>
        </mmultiscripts></math>";
    test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "versal r med 4 vänster index, nedsänkt versal i, upphöjt versal j och resterande vänster index versal k none versal l none slut vänster index och med 5 höger index, nedsänkt i upphöjt j nedsänkt k nedsänkt l och resterande höger index m none slut index");
}

#[test]
fn prime() {
    let expr = "<math> <msup><mi>x</mi><mo >&#x2032;</mo></msup> </math>";
    test("sv", "SimpleSpeak", expr, "x prim");
}

#[test]
fn given() {
    let expr = "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo></math>";
    test("sv", "SimpleSpeak", expr, "versal p; vänster-parentes; versal a givet versal b; höger-parentes");
    test("sv", "ClearSpeak", expr,  "versal p; vänster-parentes; versal a givet versal b; höger-parentes");
}

#[test]
fn simple_msubsup() {
    let expr = "<math>
            <mstyle displaystyle='true' scriptlevel='0'>
            <msubsup>
                <mi>x</mi>
                <mrow>
                <mi>k</mi>
                </mrow>
                <mrow>
                <mi>i</mi>
                </mrow>
            </msubsup>
            </mstyle>
        </math>";
    test("sv", "ClearSpeak", expr, "x nedsänkt k upphöjt till i");
}

#[test]
fn non_simple_msubsup() {
    let expr = "<math><msubsup><mi>i</mi><mrow><mi>j</mi><mo>&#x2212;</mo><mn>2</mn></mrow><mi>k</mi></msubsup></math>";
    test("sv", "SimpleSpeak", expr, "i nedsänkt j minus 2 slut nedsänkt, upphöjt till k");
    test("sv", "ClearSpeak", expr, "i nedsänkt j minus 2 slut nedsänkt, upphöjt till k");
}

#[test]
fn presentation_mathml_in_semantics() {
    let expr = "<math>
        <semantics>
            <annotation encoding='application/x-tex'>{\\displaystyle x_k^i}</annotation>
            <annotation-xml encoding='MathML-Presentation'>
                <msubsup>
                    <mi>x</mi>
                    <mrow>
                    <mi>k</mi>
                    </mrow>
                    <mrow>
                    <mi>i</mi>
                    </mrow>
                </msubsup>
            </annotation-xml>
        </semantics>
    </math>";
    test("sv", "ClearSpeak", expr, "x nedsänkt k upphöjt till i");
}

#[test]
fn ignore_period() {
    // from https://en.wikipedia.org/wiki/Probability
    let expr = "<math>
    <semantics>
    <annotation encoding='application/x-tex'>{\\displaystyle x_k^i}</annotation>
    <annotation-xml encoding='MathML-Presentation'>
      <mrow>
        <mstyle displaystyle='true' scriptlevel='0'>
          <mi>P</mi>
          <mo stretchy='false'>(</mo>
          <mi>A</mi>
          <mrow>
            <mstyle displaystyle='false' scriptlevel='0'>
              <mtext>&nbsp;and&nbsp;</mtext>
            </mstyle>
          </mrow>
          <mi>B</mi>
          <mo stretchy='false'>)</mo>
          <mo>=</mo>
          <mi>P</mi>
          <mo stretchy='false'>(</mo>
          <mi>A</mi>
          <mo>∩<!-- ∩ --></mo>
          <mi>B</mi>
          <mo stretchy='false'>)</mo>
          <mo>=</mo>
          <mi>P</mi>
          <mo stretchy='false'>(</mo>
          <mi>A</mi>
          <mo stretchy='false'>)</mo>
          <mi>P</mi>
          <mo stretchy='false'>(</mo>
          <mi>B</mi>
          <mo stretchy='false'>)</mo>
          <mo>.</mo>
        </mstyle>
      </mrow>
      </annotation-xml>
    </semantics>  
  </math>";
    test("sv", "SimpleSpeak", expr, "versal p; vänster-parentes; versal a and versal b; höger-parentes; lika med; versal p; vänster-parentes; versal a snittet versal b; höger-parentes; lika med; versal p av versal a, versal p av versal b");
}

#[test]
fn ignore_mtext_period() {
    let expr = "<math><mrow><mrow><mo>{</mo><mn>2</mn><mo>}</mo></mrow><mtext>.</mtext></mrow></math>";
    test("sv", "SimpleSpeak", expr, "mängden 2");
}

#[test]
fn ignore_comma() {
    // from https://en.wikipedia.org/wiki/Probability
    let expr = "<math>
    <mrow>
      <mstyle displaystyle='true' scriptlevel='0'>
        <mi>ϕ<!-- ϕ --></mi>
        <mo stretchy='false'>(</mo>
        <mi>x</mi>
        <mo stretchy='false'>)</mo>
        <mo>=</mo>
        <mi>c</mi>
        <msup>
          <mi>e</mi>
          <mrow>
            <mo>−<!-- − --></mo>
            <msup>
              <mi>h</mi>
              <mrow>
                <mn>2</mn>
              </mrow>
            </msup>
            <msup>
              <mi>x</mi>
              <mrow>
                <mn>2</mn>
              </mrow>
            </msup>
          </mrow>
        </msup>
        <mo>,</mo>
      </mstyle>
    </mrow>
</math>";
    test("sv", "SimpleSpeak", expr, "fi av x lika med; c gånger, e upphöjt till minus h kvadrat, x kvadrat");
}

#[test]
#[ignore] // issue #14
fn ignore_period_and_space() {
    // from https://en.wikipedia.org/wiki/Probability
    let expr = "<math>
      <mrow>
        <mstyle displaystyle='true' scriptlevel='0'>
          <mi>P</mi>
          <mo stretchy='false'>(</mo>
          <mi>A</mi>
          <mo>∣<!-- ∣ --></mo>
          <mi>B</mi>
          <mo stretchy='false'>)</mo>
          <mo>=</mo>
          <mrow>
            <mfrac>
              <mrow>
                <mi>P</mi>
                <mo stretchy='false'>(</mo>
                <mi>A</mi>
                <mo>∩<!-- ∩ --></mo>
                <mi>B</mi>
                <mo stretchy='false'>)</mo>
              </mrow>
              <mrow>
                <mi>P</mi>
                <mo stretchy='false'>(</mo>
                <mi>B</mi>
                <mo stretchy='false'>)</mo>
              </mrow>
            </mfrac>
          </mrow>
          <mo>.</mo>
          <mspace width='thinmathspace'></mspace>
        </mstyle>
      </mrow>
</math>";
    test("sv", "ClearSpeak", expr, "phi of x is equal to; c, e raised to the negative h squared x squared power");
}


#[test]
fn mn_with_space() {
  let expr = "<math><mn>1 234 567</mn></math>";
  test_prefs("sv", "SimpleSpeak", vec![("DecimalSeparators",","), ("BlockSeparators", " .")], expr, "1234567");
}

#[test]
fn mn_with_block_and_decimal_separators() {
  let expr = "<math><mn>1.234,56</mn></math>";                                       // may want to change this for another language
  test_prefs("en", "SimpleSpeak", vec![("DecimalSeparators", ","), ("BlockSeparators", " .")], expr, "1234,56");
}
