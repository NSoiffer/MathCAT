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
    test("fi", "SimpleSpeak", expr, "suora fii arvolla x, on yhtä suuri kuin; c, e potenssiin negatiivinen h toiseen, x toiseen");
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

/// Tests for expressions that appear in the Finnish matriculation exams 

#[test]
fn difference_quotinent() {
  let expr = "<math>
  <mrow>
    <mrow>
      <mi>D</mi>
    </mrow>
    <mi>f</mi>
    <mo>(</mo>
    <mi>a</mi>
    <mo>)</mo>
    <mo>=</mo>
    <msup>
      <mi>f</mi>
      <mo>′</mo>
    </msup>
    <mo>(</mo>
    <mi>a</mi>
    <mo>)</mo>
    <mo>=</mo>
    <munder>
      <mi>lim</mi>
      <mrow>
        <mi>x</mi>
        <mo>→</mo>
        <mi>a</mi>
      </mrow>
    </munder>
    <mo>⁡</mo>
    <mfrac>
      <mrow>
        <mi>f</mi>
        <mo>(</mo>
        <mi>x</mi>
        <mo>)</mo>
        <mo>−</mo>
        <mi>f</mi>
        <mo>(</mo>
        <mi>a</mi>
        <mo>)</mo>
      </mrow>
      <mrow>
        <mi>x</mi>
        <mo>−</mo>
        <mi>a</mi>
      </mrow>
    </mfrac>
    <mo>=</mo>
    <munder>
      <mi>lim</mi>
      <mrow>
        <mi>h</mi>
        <mo>→</mo>
        <mn>0</mn>
      </mrow>
    </munder>
    <mo>⁡</mo>
    <mfrac>
      <mrow>
        <mi>f</mi>
        <mo>(</mo>
        <mi>a</mi>
        <mo>+</mo>
        <mi>h</mi>
        <mo>)</mo>
        <mo>−</mo>
        <mi>f</mi>
        <mo>(</mo>
        <mi>a</mi>
        <mo>)</mo>
      </mrow>
      <mi>h</mi>
    </mfrac>
  </mrow>
</math>";
test("fi", "ClearSpeak", expr, "iso d f arvolla a; on yhtä suuri kuin, f pilkku, arvolla a, on yhtä suuri kuin; murtoluku osoittaja; f arvolla x, miinus f arvolla a; ja nimittäjä x miinus a; on yhtä suuri kuin; raja-arvo kun h lähestyy 0; arvolla; murtoluku osoittaja; f arvolla, auki sulku a plus h, kiinni sulku; miinus f arvolla a; ja nimittäjä h;");
test("fi", "SimpleSpeak", expr, "iso d f arvolla a; on yhtä suuri kuin, f pilkku, arvolla a, on yhtä suuri kuin; raja-arvo kun x lähestyy a; arvolla; murtoluku, f arvolla x, miinus f arvolla a, per, x miinus a, loppu murtoluku; on yhtä suuri kuin; raja-arvo kun h lähestyy 0; arvolla; murtoluku, f arvolla, auki sulku a plus h, kiinni sulku; miinus f arvolla a, per h, loppu murtoluku;");
}

#[test]
fn Quadratic_equation() {
    let expr ="<math>
    <mi>x</mi>
    <mo>=</mo>
    <mfrac>
    <mrow>
    <mo>−</mo>
    <mi>b</mi>
    <mi>±</mi>
    <msqrt>
    <msup>
    <mi>b</mi>
    <mn>2</mn>
    </msup>
    <mo>−</mo>
    <mn>4</mn>
    <mi>a</mi>
    <mi>b</mi>
    </msqrt>
    </mrow>
    <mrow>
    <mn>2</mn>
    <mn>a</mn>
    </mrow>
    </mfrac>
    </math>
    ";
    test("fi", "ClearSpeak", expr ,"x on yhtä suuri kuin; murtoluku osoittaja; negatiivinen b plus-miinus; neliöjuuri b toiseen miinus 4 a b; ja nimittäjä 2 a;");
    test("fi", "SimpleSpeak", expr, "x on yhtä suuri kuin; murtoluku, negatiivinen b plus-miinus; neliöjuuri b toiseen miinus 4 a b loppu juuri; per, 2 a, loppu murtoluku;")
}

#[test]
fn normal_distribution_e() {
    let expr = "<math>
    <mrow>
      <msup>
        <mi>e</mi>
        <mrow>
          <mo>−</mo>
          <mfrac>
            <mn>1</mn>
            <mn>2</mn>
          </mfrac>
          <msup>
            <mrow>
              <mrow>
                <mo>(</mo>
                <mrow>
                  <mfrac>
                    <mrow>
                      <mi>x</mi>
                      <mo>−</mo>
                      <mi>μ</mi>
                    </mrow>
                    <mi>σ</mi>
                  </mfrac>
                </mrow>
                <mo>)</mo>
              </mrow>
            </mrow>
            <mn>2</mn>
          </msup>
        </mrow>
      </msup>
    </mrow>
  </math>";
  test("fi", "ClearSpeak", expr, "e potenssiin, negatiivinen 1 kahdesosa, kertaa; auki sulku; murtoluku osoittaja; x miinus myy; ja nimittäjä sigma; kiinni sulku toiseen, loppu potenssi");
  test("fi", "SimpleSpeak", expr, "e potenssiin negatiivinen 1 kahdesosa, kertaa; auki sulku; murtoluku, x miinus myy, per sigma, loppu murtoluku; kiinni sulku toiseen");
}

#[test]
fn triangle_inequality() {
    let expr = "<math>
    
    <mo>|</mo>
    <mrow>
    <mo>|</mo>
    <mi>a</mi>
    <mo>|</mo>
    <mo>-</mo>
    <mo>|</mo>
    <mi>b</mi>
    <mo>|</mo>
    </mrow>
    <mo>|</mo>
    <mo>≤</mo>
    <mo>|</mo>
    <mi>a</mi>
    <mo>+</mo>
    <mi>b</mi>
    <mo>|</mo>
    <mo>≤</mo>
    <mo>|</mo>
    <mi>a</mi>
    <mo>|</mo>
    <mo>+</mo>
    <mo>|</mo>
    <mi>b</mi>
    <mo>|</mo>
    </math>
    ";
    test("fi", "ClearSpeak", expr, "itseisarvo itseisarvo a, miinus itseisarvo b; on pienempi tai yhtä suuri kuin; itseisarvo a plus b; on pienempi tai yhtä suuri kuin; itseisarvo a, plus itseisarvo b,");
    test("fi", "SimpleSpeak", expr, "itseisarvo itseisarvo a, miinus itseisarvo b; loppu itseisarvo; on pienempi tai yhtä suuri kuin; itseisarvo a plus b, loppu itseisarvo; on pienempi tai yhtä suuri kuin; itseisarvo a, plus itseisarvo b,");
}