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
    test("fi", "SimpleSpeak", expr, 
        "a gravis, b tilde, c lyhyysmerkki, b hattu, c gravis; plus; x piste, y piste, z piste piste, u kolmoispiste, v nelinkertainen piste; plus x hattu, plus vektori t");
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
    test("fi", "SimpleSpeak", expr, "raja-arvo kun x lähestyy 0; arvolla, murtoluku, sini arvolla x, per x, loppu murtoluku;");
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
    test("fi", "SimpleSpeak", expr, "raja-arvo kun x lähestyy alhaalta 0; arvolla sini arvolla x");
}


#[test]
fn binomial_mmultiscripts() {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("fi", "SimpleSpeak", expr, "n yli m");
}


#[test]
fn permutation_mmultiscripts() {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("fi", "SimpleSpeak", expr, "k permutaatio n");
}

#[test]
fn permutation_mmultiscripts_sup() {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("fi", "SimpleSpeak", expr, "k permutaatio n");
}

#[test]
fn permutation_msubsup() {
    let expr = "<math><msubsup><mi>P</mi><mi>k</mi><mi>n</mi></msubsup></math>";
    test("fi", "SimpleSpeak", expr, "k permutaatio n");
}

#[test]
fn tensor_mmultiscripts() {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> 
        </mmultiscripts></math>";
    test_prefs("fi", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "iso r jolla on 4 jälkikirjoitusta, alaindeksi i yläindeksi j alaindeksi k alaindeksi l");
    test_prefs("fi", "SimpleSpeak", vec![("Verbosity", "Medium")], expr,
            "iso r jolla on 4 jälkikirjoitusta, ala i ylä j ala k ala l");
}

#[test]
fn huge_num_mmultiscripts() {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> <mi>m</mi><none/>
            <mprescripts/> <mi>I</mi><none/> <none/><mi>J</mi> <mi>K</mi><none/> <mi>L</mi><none/>
        </mmultiscripts></math>";
    test_prefs("fi", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "iso r 4 etumäärettä, etualaindeksi iso i, etuyläindeksi iso j ja vaihtelevat etumääreet iso k none iso l none loppu etumääreet ja jolla on 5 jälkikirjoitusta, alaindeksi i yläindeksi j alaindeksi k alaindeksi l ja vaihtelevia määreitä m none loppu määreet");
}

#[test]
fn prime() {
    let expr = "<math> <msup><mi>x</mi><mo >&#x2032;</mo></msup> </math>";
    test("fi", "SimpleSpeak", expr, "x pilkku,");
}

#[test]
fn given() {
    let expr = "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo></math>";
    test("fi", "SimpleSpeak", expr, "iso p; auki sulku, iso a pystysuora viiva iso b; kiinni sulku");
    test("fi", "ClearSpeak", expr,  "iso p, auki sulku, iso a jakaa iso b, kiinni sulku");  // not good, but follows the spec
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
    test("fi", "ClearSpeak", expr, "x ala k, potenssiin i");
}

#[test]
fn non_simple_msubsup() {
    let expr = "<math><msubsup><mi>i</mi><mrow><mi>j</mi><mo>&#x2212;</mo><mn>2</mn></mrow><mi>k</mi></msubsup></math>";
    test("fi", "SimpleSpeak", expr, "i ala j miinus 2 loppu ala, potenssiin k");
    test("fi", "ClearSpeak", expr, "i ala j miinus 2 loppu ala, potenssiin k");
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
    test("fi", "ClearSpeak", expr, "x ala k, potenssiin i");
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
    test("fi", "SimpleSpeak", expr, "iso p; auki sulku, iso a and iso b; kiinni sulku; on yhtä suuri kuin; iso p, auki sulku, iso a leikkaus iso b, kiinni sulku; on yhtä suuri kuin; iso p arvolla iso a; iso p arvolla iso b");
}

#[test]
fn ignore_mtext_period() {
    let expr = "<math><mrow><mrow><mo>{</mo><mn>2</mn><mo>}</mo></mrow><mtext>.</mtext></mrow></math>";
    test("fi", "SimpleSpeak", expr, "joukko 2");
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
    test("fi", "SimpleSpeak", expr, "suora fii arvolla x, on yhtä suuri kuin; c, e potenssiin negatiivinen h neliö, x neliö");
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
    test("fi", "ClearSpeak", expr, "phi of x is equal to; c, e raised to the negative h squared x squared power");
}


#[test]
fn mn_with_space() {
    let expr = "<math><mn>1 234 567</mn></math>";
    test("fi", "SimpleSpeak", expr, "1234567");
}
