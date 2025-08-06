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
    test("fi", "SimpleSpeak", expr, "raja-arvo kun x lähestyy 0; arvolla, murtoluku, sini arvolla x, per x, loppu murtoluku");
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
    test("fi", "SimpleSpeak", expr, "x pilkku");
}

#[test]
fn given() {
    let expr = "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo></math>";
    test("fi", "SimpleSpeak", expr, "iso p, auki sulku, iso a ehdolla iso b, kiinni sulku");
    test("fi", "ClearSpeak", expr,  "iso p, auki sulku, iso a ehdolla iso b, kiinni sulku");
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
    test("fi", "ClearSpeak", expr, "x ala k potenssiin i");
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
    test("fi", "ClearSpeak", expr, "x ala k potenssiin i");
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
    test("fi", "SimpleSpeak", expr, "suora fii arvolla x, on yhtä suuri kuin; c kertaa, e potenssiin negatiivinen h toiseen, x toiseen");
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

/// Tests for expressions that appear in the Finnish matriculation exams (FinME)

#[test]
fn FinME_difference_quotinent() {
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
test("fi", "ClearSpeak", expr, "iso d f arvolla a; on yhtä suuri kuin, f pilkku, arvolla a, on yhtä suuri kuin; murtoluku osoittaja; f arvolla x, miinus f arvolla a; ja nimittäjä x miinus a; on yhtä suuri kuin; raja-arvo kun h lähestyy 0; arvolla; murtoluku osoittaja; f arvolla, auki sulku a plus h, kiinni sulku; miinus f arvolla a; ja nimittäjä h");
test("fi", "SimpleSpeak", expr, "iso d f arvolla a; on yhtä suuri kuin, f pilkku, arvolla a, on yhtä suuri kuin; raja-arvo kun x lähestyy a; arvolla; murtoluku, f arvolla x, miinus f arvolla a, per, x miinus a, loppu murtoluku; on yhtä suuri kuin; raja-arvo kun h lähestyy 0; arvolla; murtoluku, f arvolla, auki sulku a plus h, kiinni sulku; miinus f arvolla a, per h, loppu murtoluku");
}

#[test]
fn FinME_Quadratic_equation() {
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
    test("fi", "ClearSpeak", expr ,"x on yhtä suuri kuin; murtoluku osoittaja; negatiivinen b plus-miinus; neliöjuuri b toiseen miinus 4 a b; ja nimittäjä 2 a");
    test("fi", "SimpleSpeak", expr, "x on yhtä suuri kuin; murtoluku, negatiivinen b plus-miinus; neliöjuuri b toiseen miinus 4 a b loppu juuri; per, 2 a, loppu murtoluku")
}

#[test]
fn FinME_normal_distribution_e() {
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
fn FinME_triangle_inequality() {
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
    test("fi", "ClearSpeak", expr, "itseisarvo itseisarvo a, miinus itseisarvo b; on pienempi tai yhtä suuri kuin; itseisarvo a plus b; on pienempi tai yhtä suuri kuin; itseisarvo a, plus itseisarvo b");
    test("fi", "SimpleSpeak", expr, "itseisarvo itseisarvo a, miinus itseisarvo b; loppu itseisarvo; on pienempi tai yhtä suuri kuin; itseisarvo a plus b, loppu itseisarvo; on pienempi tai yhtä suuri kuin; itseisarvo a, plus itseisarvo b");
}

#[test]
fn FinME_arithmetic_sum() {
    let expr="<math>
    <msub>
      <mi>a</mi>
      <mn>1</mn>
    </msub>
    <mo>+</mo>
    <msub>
      <mi>a</mi>
      <mn>2</mn>
    </msub>
    <mo>+</mo>
    <mo>⋯</mo>
    <mo>+</mo>
    <msub>
      <mi>a</mi>
      <mi>n</mi>
    </msub>
    <mo>=</mo>
    <mrow>
      <mi>n</mi>
      <mo>⁢</mo>
      <mfrac>
        <mrow>
          <msub>
            <mi>a</mi>
            <mn>1</mn>
          </msub>
          <mo>+</mo>
          <msub>
            <mi>a</mi>
            <mn>2</mn>
          </msub>
        </mrow>
        <mn>2</mn>
      </mfrac>
    </mrow>
  </math>
    ";
    test("fi", "ClearSpeak", expr, "a ala 1 plus a ala 2 plus piste piste piste plus a ala n; on yhtä suuri kuin; n; murtoluku osoittaja; a ala 1 plus a ala 2; ja nimittäjä 2");
    test("fi", "SimpleSpeak", expr, "a ala 1 plus a ala 2 plus piste piste piste plus a ala n; on yhtä suuri kuin; n; murtoluku, a ala 1 plus a ala 2, per 2, loppu murtoluku")
}

#[test]
fn FinME_geometric_sum() {
    let expr ="
    <math>
      <msub>
        <mi>S</mi>
        <mi>n</mi>
      </msub>
      <mo>=</mo>
      <msub>
        <mi>a</mi>
        <mn>1</mn>
      </msub>
      <mo>+</mo>
      <msub>
        <mi>a</mi>
        <mn>1</mn>
      </msub>
      <mo>⁢</mo>
      <mi>q</mi>
      <mo>+</mo>
      <msub>
        <mi>a</mi>
        <mn>1</mn>
      </msub>
      <mo>⁢</mo>
      <msup>
        <mi>q</mi>
        <mn>2</mn>
      </msup>
      <mo>+</mo>
      <mo>⋯</mo>
      <mo>+</mo>
      <msub>
        <mi>a</mi>
        <mn>1</mn>
      </msub>
      <mo>⁢</mo>
      <msup>
        <mi>q</mi>
        <mrow>
          <mi>n</mi>
          <mo>−</mo>
          <mn>1</mn>
        </mrow>
      </msup>
      <mo>=</mo>
      <msub>
        <mi>a</mi>
        <mn>1</mn>
      </msub>
      <mo>⋅</mo>
      <mfrac>
        <mrow>
          <mn>1</mn>
          <mo>−</mo>
          <msup>
            <mi>q</mi>
            <mi>n</mi>
          </msup>
        </mrow>
        <mrow>
          <mn>1</mn>
          <mo>−</mo>
          <mi>q</mi>
        </mrow>
      </mfrac>
    </math>
    ";
    test("fi", "ClearSpeak", expr, "iso s ala n on yhtä suuri kuin; a ala 1 plus a ala 1 q plus a ala 1 q toiseen, plus piste piste piste plus, a ala 1 q potenssiin n miinus 1; on yhtä suuri kuin; a ala 1 kertaa; murtoluku osoittaja; 1 miinus q potenssiin n; ja nimittäjä 1 miinus q");
    test("fi", "SimpleSpeak", expr, "iso s ala n on yhtä suuri kuin; a ala 1 plus a ala 1 q plus a ala 1 q toiseen, plus piste piste piste plus, a ala 1 kertaa q potenssiin n miinus 1; on yhtä suuri kuin; a ala 1 kertaa; murtoluku, 1 miinus q potenssiin n, per, 1 miinus q, loppu murtoluku")
}


#[test]
fn FinME_absolute_value_defition() {
    let expr ="<math>
    <mrow>
      <mo>|</mo>
      <mi>a</mi>
      <mo>|</mo>
    </mrow>
    <mo>=</mo>
    <mrow>
      <mo>{</mo>
      <mrow>
        <mtable>
          <mtr>
            <mtd>
              <mrow>
                <mi>a</mi><mtext>&#x00A0;jos&#x00A0;</mtext><mi>a</mi>
                <mo>≥</mo><mn>0</mn>
              </mrow>
            </mtd>
          </mtr>
          <mtr>
            <mtd>
              <mrow>
                <mo>−</mo><mi>a</mi><mtext>&#x00A0;jos&#x00A0;</mtext>
                <mi>a</mi><mo>&lt;</mo><mn>0</mn>
              </mrow>
            </mtd>
          </mtr>
        </mtable>
      </mrow>
    </mrow>
  </math>";
  test("fi", "ClearSpeak", expr, "itseisarvo a; on yhtä suuri kuin; 2 tapausta; tapaus 1; a jos a; on suurempi tai yhtä suuri kuin 0; tapaus 2; negatiivinen a jos a; on pienempi kuin 0");
  test("fi", "SimpleSpeak", expr, "itseisarvo a; on yhtä suuri kuin; 2 tapausta; tapaus 1; a jos a; on suurempi tai yhtä suuri kuin 0; tapaus 2; negatiivinen a jos a; on pienempi kuin 0")
}

#[test]
fn FinME_mroot_msup_rule() {
    let expr = "<math>
    <mrow>
      <msup>
        <mi>a</mi>
        <mfrac>
          <mi>m</mi>
          <mi>n</mi>
        </mfrac>
      </msup>
      <mo>=</mo>
      <mroot>
        <msup>
          <mi>a</mi>
          <mi>m</mi>
        </msup>
        <mi>n</mi>
      </mroot>
      <mo>=</mo>
      <mo>(</mo>
      <mroot>
        <mi>a</mi>
        <mi>n</mi>
      </mroot>
      <msup>
        <mo>)</mo>
        <mi>m</mi>
      </msup>
    </mrow>
  </math>";
  test("fi", "ClearSpeak", expr, "a potenssiin m per n, on yhtä suuri kuin, nnes juuri a potenssiin m; on yhtä suuri kuin; auki sulku nnes juuri a; kiinni sulku potenssiin m");
  test("fi", "SimpleSpeak", expr, "a potenssiin m per n; on yhtä suuri kuin, nnes juuri a potenssiin m loppu juuri; on yhtä suuri kuin; auki sulku nnes juuri a; kiinni sulku potenssiin m")
}

#[test]
fn FinME_newton_binomial() {
    let expr = "<math>
    <mrow>
      <msup>
        <mrow>
          <mo>(</mo>
          <mi>a</mi>
          <mo>+</mo>
          <mi>b</mi>
          <mo>)</mo>
        </mrow>
        <mi>n</mi>
      </msup>
      <mo>=</mo>
      <mrow>
        <munderover>
          <mo>∑</mo>
          <mrow>
            <mi>k</mi>
            <mo>=</mo>
            <mn>0</mn>
          </mrow>
          <mi>n</mi>
        </munderover>
      </mrow>
      <mrow>
      <mo>(</mo>
        <mfrac linethickness='0'>
          <mi>n</mi>
          <mi>k</mi>
        </mfrac>
        <mo>)</mo>
      </mrow>
      <mo>&#8290;</mo>
      <msup>
        <mi>a</mi>
        <mrow>
          <mi>n</mi>
          <mo>−</mo>
          <mi>k</mi>
        </mrow>
      </msup>
      <mo>&#8290;</mo>
      <msup>
        <mi>b</mi>
        <mi>k</mi>
      </msup>
      <mo>=</mo>
      <mrow>
        <munderover>
          <mo>∑</mo>
          <mrow>
            <mi>k</mi>
            <mo>=</mo>
            <mn>0</mn>
          </mrow>
          <mi>n</mi>
        </munderover>
      </mrow>
      <mfrac>
        <mrow>
          <mi>n</mi>
          <mo>!</mo>
        </mrow>
        <mrow>
          <mi>k</mi>
          <mo>!</mo>
          <mo>(</mo>
          <mi>n</mi>
          <mo>−</mo>
          <mi>k</mi>
          <mo>)</mo>
          <mo>!</mo>
        </mrow>
      </mfrac>
      <mo>&#8290;</mo>
      <msup>
        <mi>a</mi>
        <mrow>
          <mi>n</mi>
          <mo>−</mo>
          <mi>k</mi>
        </mrow>
      </msup>
      <mo>&#8290;</mo>
      <msup>
        <mi>b</mi>
        <mi>k</mi>
      </msup>
    </mrow>
  </math>";
  test("fi", "ClearSpeak", expr, "auki sulku a plus b, kiinni sulku potenssiin n; on yhtä suuri kuin; summa käy, luvusta k on yhtä suuri kuin 0, lukuun n; n yli k a potenssiin n miinus k, b potenssiin k; on yhtä suuri kuin; summa käy, luvusta k on yhtä suuri kuin 0, lukuun n; murtoluku osoittaja; n kertoma; ja nimittäjä k kertoma, auki sulku n miinus k, kiinni sulku; kertoma; a potenssiin n miinus k, b potenssiin k");
  test("fi", "SimpleSpeak", expr, "auki sulku a plus b, kiinni sulku potenssiin n; on yhtä suuri kuin; summa käy, luvusta k on yhtä suuri kuin 0, lukuun n; n yli k kertaa a potenssiin n miinus k, b potenssiin k; on yhtä suuri kuin; summa käy, luvusta k on yhtä suuri kuin 0, lukuun n; murtoluku, n kertoma, per, k kertoma, auki sulku n miinus k, kiinni sulku; kertoma, loppu murtoluku; kertaa a potenssiin n miinus k, b potenssiin k")
}

#[test]
fn FinME_secant_algorithm() {
    let expr ="<math>
    <mrow>
      <msub>
        <mi>x</mi>
        <mrow>
          <mi>n</mi>
          <mo>+</mo>
          <mn>1</mn>
        </mrow>
      </msub>
      <mo>=</mo>
    </mrow>
    <mrow>
      <msub>
        <mi>x</mi>
        <mi>n</mi>
      </msub>
      <mo>−</mo>
    </mrow>
    <mrow>
      <mfrac>
        <mrow>
          <msub>
            <mi>x</mi>
            <mi>n</mi>
          </msub>
          <mo>−</mo>
          <msub>
            <mi>x</mi>
            <mrow>
              <mi>n</mi>
              <mo>−</mo>
              <mn>1</mn>
            </mrow>
          </msub>
        </mrow>
        <mrow>
          <mi>f</mi>
          <mo>&#x2061;</mo>
          <mo>(</mo>
          <msub>
            <mi>x</mi>
            <mi>n</mi>
          </msub>
          <mo>)</mo>
          <mo>−</mo>
          <mi>f</mi>
          <mo>&#x2061;</mo>
          <mo>(</mo>
          <msub>
            <mi>x</mi>
            <mrow>
              <mi>n</mi>
              <mo>−</mo>
              <mn>1</mn>
            </mrow>
          </msub>
          <mo>)</mo>
        </mrow>
      </mfrac>
      <mo>⁢</mo>
      <mi>f</mi>
      <mo>&#x2061;</mo>
      <mo>(</mo>
      <msub>
        <mi>x</mi>
        <mi>n</mi>
      </msub>
      <mo>)</mo>
    </mrow>
  </math>";
  test("fi", "ClearSpeak", expr, "x ala n plus 1 loppu ala; on yhtä suuri kuin; x ala n miinus; murtoluku osoittaja; x ala n miinus, x ala n miinus 1 loppu ala; ja nimittäjä f arvolla, auki sulku x ala n kiinni sulku; miinus; f arvolla; auki sulku, x ala n miinus 1 loppu ala; kiinni sulku; f arvolla, auki sulku x ala n kiinni sulku");
  test("fi", "SimpleSpeak", expr, "x ala n plus 1 loppu ala; on yhtä suuri kuin; x ala n miinus; murtoluku, x ala n miinus, x ala n miinus 1 loppu ala; per, f arvolla, auki sulku x ala n kiinni sulku; miinus; f arvolla; auki sulku, x ala n miinus 1 loppu ala; kiinni sulku, loppu murtoluku; f arvolla, auki sulku x ala n kiinni sulku");
}

#[test]
fn Fin_ME_law_of_sines() {
    let expr = "<math>
    <mfrac>
      <mi>a</mi>
      <mrow>
        <mi>sin</mi>
        <mo>⁡</mo>
        <mi>α</mi>
      </mrow>
    </mfrac>
    <mo>=</mo>
    <mfrac>
      <mi>b</mi>
      <mrow>
        <mi>sin</mi>
        <mo>⁡</mo>
        <mi>β</mi>
      </mrow>
    </mfrac>
    <mo>=</mo>
    <mfrac>
      <mi>c</mi>
      <mrow>
        <mi>sin</mi>
        <mo>⁡</mo>
        <mi>γ</mi>
      </mrow>
    </mfrac>
  </math>";
  test("fi", "ClearSpeak", expr, "a per sini alfa, on yhtä suuri kuin, b per sini beeta, on yhtä suuri kuin, c per sini gamma");
  test("fi", "SimpleSpeak", expr, "murtoluku, a per, sini arvolla alfa, loppu murtoluku; on yhtä suuri kuin; murtoluku, b per, sini arvolla beeta, loppu murtoluku; on yhtä suuri kuin; murtoluku, c per, sini arvolla gamma, loppu murtoluku");
}

#[test]
fn FinME_vectors() {
let expr = "<math>
<mrow>
  <mrow>
    <mover>
      <mi>v</mi>
      <mo>→</mo>
    </mover>
  </mrow>
  <mo>=</mo>
</mrow>
<mrow>
  <msub>
    <mi>x</mi>
    <mn>1</mn>
  </msub>
  <mrow>
    <mover>
      <mi>i</mi>
      <mo>→</mo>
    </mover>
  </mrow>
  <mo>+</mo>
</mrow>
<mrow>
  <msub>
    <mi>y</mi>
    <mn>1</mn>
  </msub>
  <mrow>
    <mover>
      <mi>j</mi>
      <mo>→</mo>
    </mover>
  </mrow>
</mrow>
</math>";
test("fi", "ClearSpeak", expr, "vektori v, on yhtä suuri kuin; x ala 1 vektori i; plus; y ala 1 vektori j");
test("fi", "SimpleSpeak", expr, "vektori v, on yhtä suuri kuin; x ala 1 vektori i; plus; y ala 1 vektori j");
}

#[test]
fn FinME_compound_derivate_rule() {
let expr = "<math>
<mrow>
  <mrow>
    <mi>D</mi>
  </mrow>
  <mrow>
    <mo>(</mo>
    <mi>g</mi>
    <mo>∘</mo>
    <mi>f</mi>
    <mo>)</mo>
  </mrow>
  <mo>=</mo>
</mrow>
<mrow>
  <mrow>
    <mi>D</mi>
  </mrow>
  <mi>g</mi>
  <mo>(</mo>
  <mi>f</mi>
  <mo>(</mo>
  <mi>x</mi>
  <mo>)</mo>
  <mo>)</mo>
  <mo>=</mo>
</mrow>
<mrow>
  <msup>
    <mi>g</mi>
    <mo>′</mo>
  </msup>
  <mo>(</mo>
  <mi>f</mi>
  <mo>(</mo>
  <mi>x</mi>
  <mo>)</mo>
  <mo>)</mo>
  <msup>
    <mi>f</mi>
    <mo>′</mo>
  </msup>
  <mo>(</mo>
  <mi>x</mi>
  <mo>)</mo>
</mrow>
</math>";
test("fi", "ClearSpeak", expr, "iso d, auki sulku g yhdistetty f, kiinni sulku; on yhtä suuri kuin; iso d, g arvolla f arvolla x; on yhtä suuri kuin; g pilkku, arvolla f arvolla x; f pilkku, arvolla x");
test("fi", "SimpleSpeak", expr, "iso d, auki sulku g yhdistetty f, kiinni sulku; on yhtä suuri kuin; iso d, g arvolla f arvolla x; on yhtä suuri kuin; g pilkku, arvolla f arvolla x; f pilkku, arvolla x");
}

#[test]
fn FinME_integration_in_parts() {
let expr = "<math>
<mrow>
  <mo>∫</mo>
  <msup>
    <mi>f</mi>
    <mo>′</mo>
  </msup>
  <mrow>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
  </mrow>
  <mi>g</mi>
  <mrow>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
  </mrow>
  <mtext>&#x2009;</mtext>
  <mi>d</mi>
  <mi>x</mi>
  <mo>=</mo>
</mrow>
<mrow>
  <mi>f</mi>
  <mrow>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
  </mrow>
  <mi>g</mi>
  <mrow>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
  </mrow>
  <mo>−</mo>
</mrow>
<mrow>
  <mo>∫</mo>
  <msup>
    <mi>g</mi>
    <mo>′</mo>
  </msup>
  <mrow>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
  </mrow>
  <mi>f</mi>
  <mrow>
    <mo>(</mo>
    <mi>x</mi>
    <mo>)</mo>
  </mrow>
  <mtext>&#x2009;</mtext>
  <mi>d</mi>
  <mi>x</mi>
</mrow>
</math>";
test("fi", "ClearSpeak", expr, "integraali f pilkku, arvolla x, g arvolla x, d x; on yhtä suuri kuin; f arvolla x, g arvolla x; miinus; integraali g pilkku, arvolla x, f arvolla x, d x");
test("fi", "SimpleSpeak", expr, "integraali f pilkku, arvolla x, g arvolla x, d x; on yhtä suuri kuin; f arvolla x, g arvolla x; miinus; integraali g pilkku, arvolla x, f arvolla x, d x");
}

#[test]
fn FinME_simpsons_rule() {
let expr = "<math>
<mrow>
  <msubsup>
    <mo>∫</mo>
    <mi>a</mi>
    <mi>b</mi>
  </msubsup>
  <mi>f</mi>
  <mo>(</mo>
  <mi>x</mi>
  <mo>)</mo>
  <mtext>&#x2009;</mtext>
  <mi>d</mi>
  <mi>x</mi>
  <mo>≈</mo>
</mrow>
<mrow>
  <mfrac>
    <mi>h</mi>
    <mn>3</mn>
  </mfrac>
  <mrow>
    <mo>(</mo>
    <mi>f</mi>
    <mo>(</mo>
    <msub>
      <mi>x</mi>
      <mn>0</mn>
    </msub>
    <mo>)</mo>
    <mo>+</mo>
    <mn>4</mn>
    <mi>f</mi>
    <mo>(</mo>
    <msub>
      <mi>x</mi>
      <mn>1</mn>
    </msub>
    <mo>)</mo>
    <mo>+</mo>
    <mn>2</mn>
    <mi>f</mi>
    <mo>(</mo>
    <msub>
      <mi>x</mi>
      <mn>2</mn>
    </msub>
    <mo>)</mo>
    <mo>+</mo>
    <mn>4</mn>
    <mi>f</mi>
    <mo>(</mo>
    <msub>
      <mi>x</mi>
      <mn>3</mn>
    </msub>
    <mo>)</mo>
    <mo>+</mo>
    <mo>⋯</mo>
    <mo>+</mo>
    <mn>4</mn>
    <mi>f</mi>
    <mo>(</mo>
    <msub>
      <mi>x</mi>
      <mrow>
        <mi>n</mi>
        <mo>−</mo>
        <mn>1</mn>
      </mrow>
    </msub>
    <mo>)</mo>
    <mo>+</mo>
    <mi>f</mi>
    <mo>(</mo>
    <msub>
      <mi>x</mi>
      <mi>n</mi>
    </msub>
    <mo>)</mo>
    <mo>)</mo>
  </mrow>
</mrow>
</math>";
test("fi", "ClearSpeak", expr, "integraali, alaraja a, yläraja b; f arvolla x, d x; on likimäärin yhtä suuri kuin; h per 3 kertaa; auki sulku; f arvolla, auki sulku x ala 0 kiinni sulku; plus, 4, f arvolla, auki sulku x ala 1 kiinni sulku; plus, 2, f arvolla, auki sulku x ala 2 kiinni sulku; plus, 4, f arvolla, auki sulku x ala 3 kiinni sulku; plus piste piste piste plus; 4; f arvolla; auki sulku, x ala n miinus 1 loppu ala; kiinni sulku; plus, f arvolla, auki sulku x ala n kiinni sulku; kiinni sulku");
test("fi", "SimpleSpeak", expr, "integraali, alaraja a, yläraja b; f arvolla x, d x; on likimäärin yhtä suuri kuin; h per 3, kertaa; auki sulku; f arvolla, auki sulku x ala 0 kiinni sulku; plus, 4, f arvolla, auki sulku x ala 1 kiinni sulku; plus, 2, f arvolla, auki sulku x ala 2 kiinni sulku; plus, 4, f arvolla, auki sulku x ala 3 kiinni sulku; plus piste piste piste plus; 4; f arvolla; auki sulku, x ala n miinus 1 loppu ala; kiinni sulku; plus, f arvolla, auki sulku x ala n kiinni sulku; kiinni sulku");
}

#[test]
fn FinME_binomials_cumulative_function() {
let expr = "<math>
<mrow>
  <mi>P</mi>
  <mo>&#x2061;</mo>
  <mo>(</mo>
  <mi>X</mi>
  <mo>≤</mo>
  <mi>k</mi>
  <mo>)</mo>
  <mo>=</mo>
</mrow>
<mrow>
  <msubsup>
    <mo>∑</mo>
    <mrow>
      <mi>i</mi>
      <mo>=</mo>
      <mn>0</mn>
    </mrow>
    <mrow>
      <mo>|</mo>
      <mi>k</mi>
      <mo>|</mo>
    </mrow>
  </msubsup>
  <mrow>
    <mo>(</mo>
    <mfrac linethickness='0'>
      <mi>n</mi>
      <mi>i</mi>
    </mfrac>
    <mo>)</mo>
  </mrow>
  <msup>
    <mi>p</mi>
    <mi>i</mi>
  </msup>
  <mo>(</mo>
  <mn>1</mn>
  <mo>−</mo>
  <mi>p</mi>
  <msup>
    <mo>)</mo>
    <mrow>
      <mi>n</mi>
      <mo>−</mo>
      <mi>i</mi>
    </mrow>
  </msup>
</mrow>
</math>";
test("fi", "ClearSpeak",  expr, "iso p arvolla; auki sulku, iso x on pienempi tai yhtä suuri kuin k; kiinni sulku; on yhtä suuri kuin; summa käy, luvusta i on yhtä suuri kuin 0, lukuun itseisarvo k; n yli i p potenssiin i kertaa; auki sulku 1 miinus p, kiinni sulku potenssiin n miinus i");
test("fi", "SimpleSpeak", expr, "iso p arvolla; auki sulku, iso x on pienempi tai yhtä suuri kuin k; kiinni sulku; on yhtä suuri kuin; summa käy, luvusta i on yhtä suuri kuin 0, lukuun itseisarvo k; n yli i p potenssiin i kertaa; auki sulku 1 miinus p, kiinni sulku potenssiin n miinus i");
}

#[test]
fn FinME_standard_normal_disributon_cumul() {
let expr = "<math>
<mrow>
  <mrow>
    <mi>Φ</mi>
  </mrow>
  <mo>(</mo>
  <mi>x</mi>
  <mo>)</mo>
  <mo>=</mo>
</mrow>
<mrow>
  <mfrac>
    <mn>1</mn>
    <msqrt>
      <mrow>
        <mn>2</mn>
        <mi>π</mi>
      </mrow>
    </msqrt>
  </mfrac>
  <msubsup>
    <mo>∫</mo>
    <mrow>
      <mo>−</mo>
      <mi>∞</mi>
    </mrow>
    <mi>x</mi>
  </msubsup>
  <msup>
    <mi>e</mi>
    <mrow>
      <mo>−</mo>
      <mfrac>
        <msup>
          <mi>t</mi>
          <mn>2</mn>
        </msup>
        <mn>2</mn>
      </mfrac>
    </mrow>
  </msup>
  <mtext>&#x2009;</mtext>
  <mi>d</mi>
  <mi>x</mi>
</mrow>
</math>";
test("fi", "ClearSpeak", expr, "iso fii arvolla x, on yhtä suuri kuin; murtoluku osoittaja 1; ja nimittäjä neliöjuuri 2 pii; integraali, alaraja negatiivinen ääretön, yläraja x; e potenssiin negatiivinen murtoluku osoittaja; t toiseen; ja nimittäjä 2; d x");
test("fi", "SimpleSpeak", expr, "iso fii arvolla x, on yhtä suuri kuin; murtoluku, 1 per, neliöjuuri 2 pii loppu juuri; loppu murtoluku; integraali, alaraja negatiivinen ääretön, yläraja x; e potenssiin negatiivinen murtoluku, t toiseen, per 2, loppu murtoluku; d x");
}
