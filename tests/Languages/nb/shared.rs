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
        <mover> <mi>r</mi> <mo>ˇ</mo> </mover>  <mo>+</mo>
        <mover> <mi>x</mi> <mo>.</mo> </mover>
        <mover> <mi>y</mi> <mo>&#x2D9;</mo> </mover>
        <mover> <mi>z</mi> <mo>&#x00A8;</mo> </mover>
        <mover> <mi>u</mi> <mo>&#x20DB;</mo> </mover>
        <mover> <mi>v</mi> <mo>&#x20DC;</mo> </mover> <mo>+</mo>
        <mover> <mi>x</mi> <mo>^</mo> </mover> <mo>+</mo>
        <mover> <mi>t</mi> <mo>→</mo> </mover>
        </mrow> </math>";
    test("nb", "SimpleSpeak", expr, 
        "a grav aksent, b tilde, c breve, b hake, c grav aksent; pluss \
            r hake pluss; x prikk, y prikk, z trema, u trippel prikk; v fire prikker; pluss x hatt, pluss vektoren t");
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
    test("nb", "SimpleSpeak", expr, "grenseverdien når x går mot 0, av, brøk, sinus av x, over x, slutt brøk");
    test_prefs("nb", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "grenseverdien når x går mot 0, av, brøk, sinus av x, over x, slutt brøk");
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
    test("nb", "SimpleSpeak", expr, "grenseverdien når x går fra venstre mot 0, av sinus av x");
}


#[test]
fn binomial_mmultiscripts() {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("nb", "SimpleSpeak", expr, "n over m");
}

#[test]
fn binomial_mmultiscripts_other() {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("nb", "SimpleSpeak", expr, "n over m");
}

#[test]
fn binomial_subscript() {  // C_{n,k}
    let expr = "<math><msub><mi>C</mi><mrow><mi>n</mi><mo>,</mo><mi>m</mi></mrow></msub></math>";
    test("nb", "SimpleSpeak", expr, "n over m");
}

#[test]
fn permutation_mmultiscripts() {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("nb", "SimpleSpeak", expr, "antallet permutasjoner av k elementer av n");
}

#[test]
fn permutation_mmultiscripts_sup() {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("nb", "SimpleSpeak", expr, "antallet permutasjoner av k elementer av n");
}

#[test]
fn permutation_msubsup() {
    let expr = "<math><msubsup><mi>P</mi><mi>k</mi><mi>n</mi></msubsup></math>";
    test("nb", "SimpleSpeak", expr, "antallet permutasjoner av k elementer av n");
}

#[test]
fn tensor_mmultiscripts() {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> 
        </mmultiscripts></math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "stor r med 4 høyre indeks, senket i, hevet j, senket k, senket l");
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")], expr,
            "stor r med 4 høyre indeks, senket i, hevet j, senket k, senket l");
}

#[test]
fn huge_num_mmultiscripts() {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> <mi>m</mi><none/>
            <mprescripts/> <mi>I</mi><none/> <none/><mi>J</mi> <mi>K</mi><none/> <mi>L</mi><none/>
        </mmultiscripts></math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "stor r med 4 venstre indeks, senket stor i; hevet stor j, og resterende venstre indeks stor k none stor l none slutt venstre indeks og med 5 høyre indeks, senket i, hevet j, senket k, senket l, og resterende høyre indeks m none slutt indeks");
}

#[test]
fn prime() {
    let expr = "<math> <msup><mi>x</mi><mo >&#x2032;</mo></msup> </math>";
    test("nb", "SimpleSpeak", expr, "x primtegn");
}

#[test]
fn given() {
    let expr = "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo></math>";
    test("nb", "SimpleSpeak", expr, "stor p; startparentes; stor a, gitt stor b; sluttparentes");
    test("nb", "ClearSpeak", expr,  "stor p; startparentes; stor a, gitt stor b; sluttparentes");  // not good, but follows the spec
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
    test("nb", "ClearSpeak", expr, "x, senket k, opphøyd i i");
}

#[test]
fn non_simple_msubsup() {
  let expr = "<math><msubsup><mi>i</mi><mrow><mi>j</mi><mo>&#x2212;</mo><mn>2</mn></mrow><mi>k</mi></msubsup></math>";
  test("nb", "SimpleSpeak", expr, "i senket j minus 2 slutt senket, opphøyd i k");
  test("nb", "ClearSpeak", expr, "i senket j minus 2 slutt senket, opphøyd i k");
  test_prefs("nb", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
          "i senket j minus 2, opphøyd i k");
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
    test("nb", "ClearSpeak", expr, "x, senket k, opphøyd i i");
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
    test("nb", "SimpleSpeak", expr, "stor p; startparentes; stor a and stor b; sluttparentes; er lik; stor p; startparentes; stor a snitt stor b; sluttparentes; er lik, stor p av stor a, stor p av stor b");
}

#[test]
fn ignore_mtext_period() {
    let expr = "<math><mrow><mrow><mo>{</mo><mn>2</mn><mo>}</mo></mrow><mtext>.</mtext></mrow></math>";
    test("nb", "SimpleSpeak", expr, "mengden 2");
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
    test("nb", "SimpleSpeak", expr, "fi av x er lik, c ganger, e opphøyd i minus h i andre, x i andre");
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
    test("nb", "ClearSpeak", expr, "phi of x is equal to; c, e raised to the negative h squared x squared power");
}


#[test]
fn bug_199_2pi() {
  let expr = "<math>
      <mrow>
        <mo stretchy=\"false\" form=\"prefix\">[</mo>
        <mspace width=\"0.333em\"></mspace>
        <mn>0</mn>
        <mspace width=\"0.333em\"></mspace>
        <mo>,</mo>
        <mspace width=\"0.333em\"></mspace>
        <mn>2</mn>
        <mi>π</mi>
        <mspace width=\"0.333em\"></mspace>
        <mo stretchy=\"false\" form=\"postfix\">)</mo>
      </mrow>
    </math>";
  test("nb", "SimpleSpeak",expr, "det halvåpne intervallet fra og med 0 til 2 pi");
}

#[test]
fn caret_and_hat() {
  let expr = "<math><mi>x</mi><mo>^</mo><mn>2</mn><mo>+</mo><mover><mi>y</mi><mo>^</mo></mover></math>";
  test("nb", "SimpleSpeak",expr, "x hatt 2 pluss y hatt");
}

#[test]
fn mn_with_space() {
  let expr = "<math><mn>1 234 567</mn></math>";
  test_prefs("nb", "SimpleSpeak", vec![("DecimalSeparators", "."), ("BlockSeparators", " ,")], expr, "1234567");
}

#[test]
fn mn_with_block_and_decimal_separators() {
  let expr = "<math><mn>1.234,56</mn></math>";                                       // may want to change this for another language
  test_prefs("nb", "SimpleSpeak", vec![("DecimalSeparators", ","), ("BlockSeparators", " .")], expr, "1234,56");
}

#[test]
fn divergence() {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xB7;</mo><mi mathvariant='normal'>F</mi></math>";                                       // may want to change this for another language
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "div stor f");
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "divergensen til stor f");
}

#[test]
fn curl() {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xD7;</mo><mi mathvariant='normal'>F</mi></math>";          
  // may want to change this for another language
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "curl stor f");
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "curlen til stor f");
}

#[test]
fn gradient() {
  let expr = "<math><mo>&#x2207;</mo><mi mathvariant='normal'>F</mi></math>";          
  // may want to change this for another language
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "gradienten til stor f");
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "gradienten til stor f");
}

#[test]
fn literal_speak() {
  let expr = r#"<math data-latex='\vec{A} \perp \vec{B}' display='block'>
  <mrow data-changed='added'>
    <mover data-latex='\vec{A}'>
      <mi data-latex='A'>A</mi>
      <mo stretchy='false'>→</mo>
    </mover>
    <mo intent='perpendicular-to'>⊥</mo>
    <mover data-latex='\vec{B}'>
      <mi data-latex='B'>B</mi>
      <mo stretchy='false'>→</mo>
    </mover>
  </mrow>
 </math>"#; 
  test("nb", "LiteralSpeak", expr, "stor a høyrepil, perpendicular to, stor b høyrepil");
}

#[test]
fn literal_speak_with_name() {
  let expr = r#"<math intent='forced($x)'>
      <mrow arg="x">
        <mi>f</mi>
        <mo data-changed='added'>&#x2061;</mo>
        <mrow data-changed='added'>
          <mo>(</mo>
          <mrow data-changed='added'>
            <mi>x</mi>
            <mo>!</mo>
          </mrow>
          <mo>)</mo>
        </mrow>
      </mrow>
    </math>"#;
  test("nb", "LiteralSpeak", expr, "forced f, venstreparentes; x utropstegn; høyreparentes");
}

#[test]
fn literal_speak_with_property() {
  let expr = r#"<math intent=':prefix'>
      <mrow arg="x">
        <mi>f</mi>
        <mo data-changed='added'>&#x2061;</mo>
        <mrow data-changed='added'>
          <mo>(</mo>
          <mrow data-changed='added'>
            <mi>x</mi>
            <mo>!</mo>
          </mrow>
          <mo>)</mo>
        </mrow>
      </mrow>
    </math>"#; 
  test("nb", "LiteralSpeak", expr, "f, venstreparentes; x utropstegn; høyreparentes");
}

#[test]
fn literal_intent_property() {
  let expr = r#"<math data-latex='\vec{A} \perp \vec{B}' display='block'>
  <mrow intent=":literal">
    <mover data-latex='\vec{A}'>
      <mi data-latex='A'>A</mi>
      <mo stretchy='false'>→</mo>
    </mover>
    <mo intent='perpendicular-to'>⊥</mo>
    <mover data-latex='\vec{B}'>
      <mi data-latex='B'>B</mi>
      <mo stretchy='false'>→</mo>
    </mover>
  </mrow>
 </math>"#; 
  test("nb", "SimpleSpeak", expr, "stor a høyrepil, perpendicular to, stor b høyrepil");
}

#[test]
fn literal_intent_property_with_name() {
  let expr = r#"<math intent='forced:literal($x)'>
      <mrow arg="x">
        <mi>f</mi>
        <mo data-changed='added'>&#x2061;</mo>
        <mrow data-changed='added'>
          <mo>(</mo>
          <mrow data-changed='added'>
            <mi>x</mi>
            <mo>!</mo>
          </mrow>
          <mo>)</mo>
        </mrow>
      </mrow>
    </math>"#; 
  test("nb", "SimpleSpeak", expr, "forced f; startparentes; x utropstegn; sluttparentes");
}
