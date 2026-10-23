/// Тесты для правил, общих для различных стилей речи:
/// * модифицированная переменная
use crate::common::*;

use anyhow::Result;

#[test]
fn modified_vars() -> Result<()> {
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
    test("ru", "SimpleSpeak", expr, 
        "а гравис, б тильда, цэ брэвэ, б гачек, цэ гравис; плюс \
            эр гачек; плюс икс точка, игрек точка, зэт две точки, у три точки, вэ четыре точки; плюс икс циркумфлекс, плюс вектор тэ")?;
            return Ok(());
}

#[test]
fn double_overline() -> Result<()> {
    // A nested overline should be announced as one standard Russian mathematical term.
    let expr = "<math>
        <mover accent='false' class='mml-overline'>
            <mover accent='true'><mi>z</mi><mo>¯</mo></mover>
            <mo accent='true'>¯</mo>
        </mover>
    </math>";
    let expected = "зэт с двойным надчёркиванием";
    test("ru", "LiteralSpeak", expr, expected)?;
    test("ru", "ClearSpeak", expr, expected)?;
    test("ru", "SimpleSpeak", expr, expected)?;
    Ok(())
}

#[test]
fn complex_number_modulus_identity() -> Result<()> {
    // An overlined variable and an absolute value should use natural Russian terminology.
    let expr = "<math><mrow>
        <mrow><mi>z</mi><mo>&#x2062;</mo><mover accent='true'><mi>z</mi><mo>¯</mo></mover></mrow>
        <mo>=</mo>
        <mrow><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></mrow>
        <mo>=</mo>
        <msup><mrow><mo>|</mo><mi>z</mi><mo>|</mo></mrow><mn>2</mn></msup>
    </mrow></math>";
    test("ru", "ClearSpeak", expr,
        "зэт зэт с чертой равно икс в квадрате плюс игрек в квадрате равно модуль зэт, в квадрате")?;
    test("ru", "SimpleSpeak", expr,
        "зэт зэт с чертой равно икс в квадрате плюс игрек в квадрате равно модуль зэт в квадрате")?;
    test("ru", "LiteralSpeak", expr,
        "зэт зэт с чертой равно икс верхний индекс 2 конец верхнего индекса плюс игрек верхний индекс 2 конец верхнего индекса равно вертикальная черта зэт вертикальная черта верхний индекс 2 конец верхнего индекса")?;
    Ok(())
}

#[test]
fn limit() -> Result<()> {
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
    test("ru", "SimpleSpeak", expr, "предел при икс стремящемся к 0, от, дроби, синус икс, делить на, икс, конец дроби")?;
    test_prefs("ru", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "предел при икс стремящемся к 0, от; синус икс, делить на икс")?;
            return Ok(());
}

#[test]
fn limit_from_below() -> Result<()> {
    let expr = "<math>
            <munder>
            <mo>lim</mo>
            <mrow>  <mi>x</mi> <mo>↗</mo>  <mn>0</mn>  </mrow>
            </munder>
            <mrow>
                <mrow>  <mi>sin</mi>  <mo>&#x2061;</mo> <mi>x</mi> </mrow>
            </mrow>
        </math>";
    test("ru", "SimpleSpeak", expr, "предел при икс стремящемся к 0 снизу, от синус икс")?;
    return Ok(());
}

#[test]
fn binomial_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("ru", "SimpleSpeak", expr, "число сочетаний из эн по эм")?;
    return Ok(());
}

#[test]
fn binomial_mmultiscripts_other() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>C</mi><mi>m</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("ru", "SimpleSpeak", expr, "число сочетаний из эн по эм")?;
    return Ok(());
}

#[test]
fn binomial_subscript() -> Result<()> {  // C_{n,k}
    let expr = "<math><msub><mi>C</mi><mrow><mi>n</mi><mo>,</mo><mi>m</mi></mrow></msub></math>";
    test("ru", "SimpleSpeak", expr, "число сочетаний из эн по эм")?;
    return Ok(());
}

#[test]
fn permutation_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>";
    test("ru", "SimpleSpeak", expr, "число размещений из эн по ка")?;
    return Ok(());
}

#[test]
fn permutation_mmultiscripts_sup() -> Result<()> {
    let expr = "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>";
    test("ru", "SimpleSpeak", expr, "число размещений из эн по ка")?;
    return Ok(());
}

#[test]
fn permutation_msubsup() -> Result<()> {
    let expr = "<math><msubsup><mi>P</mi><mi>k</mi><mi>n</mi></msubsup></math>";
    test("ru", "SimpleSpeak", expr, "число размещений из эн по ка")?;
    return Ok(());
}

#[test]
fn tensor_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> 
        </mmultiscripts></math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "заглавная эр с 4 индексами после, нижний индекс и верхний индекс йот нижний индекс ка нижний индекс эль")?;
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Medium")], expr,
            "заглавная эр с 4 индексами после, нижний и верхний йот нижний ка нижний эль")?;
            return Ok(());
}

#[test]
fn huge_num_mmultiscripts() -> Result<()> {
    let expr = "<math><mmultiscripts>
            <mi>R</mi> <mi>i</mi><none/> <none/><mi>j</mi> <mi>k</mi><none/> <mi>l</mi><none/> <mi>m</mi><none/>
            <mprescripts/> <mi>I</mi><none/> <none/><mi>J</mi> <mi>K</mi><none/> <mi>L</mi><none/>
        </mmultiscripts></math>";
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "заглавная эр с 4 индексами перед, левый нижний индекс заглавная и, левый верхний индекс заглавная йот и далее левый нижний индекс заглавная ка левый нижний индекс заглавная эль, и с 5 индексами после, нижний индекс и верхний индекс йот нижний индекс ка нижний индекс эль и далее нижний индекс эм")?;
            return Ok(());
}

#[test]
fn prime() -> Result<()> {
    let expr = "<math> <msup><mi>x</mi><mo >&#x2032;</mo></msup> </math>";
    test("ru", "SimpleSpeak", expr, "икс штрих")?;
    return Ok(());
}

#[test]
fn given() -> Result<()> {
    let expr = "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo></math>";
    test("ru", "SimpleSpeak", expr, "пэ большое, скобка открывается, а большое при условии бэ большое, скобка закрывается")?;
    test("ru", "ClearSpeak", expr,  "пэ большое, скобка открывается, а большое при условии бэ большое, скобка закрывается")?;
    return Ok(());
}

#[test]
fn compact_unicode_uses_mathematical_terminology() -> Result<()> {
    // The compact Unicode table takes precedence over unicode-full, so its
    // common relation and operator names must retain their mathematical meaning.
    test("ru", "SimpleSpeak", "<math><mi>a</mi><mo>⇒</mo><mi>b</mi></math>",
        "а следует бэ")?;
    test("ru", "SimpleSpeak", "<math><mo>∐</mo></math>", "копроизведение")?;
    test("ru", "SimpleSpeak", "<math><mi>a</mi><mo>≦</mo><mi>b</mi><mo>≧</mo><mi>c</mi></math>",
        "а меньше или равно бэ больше или равно цэ")?;
    test("ru", "SimpleSpeak", "<math><mi>x</mi><mo>∊</mo><mi>A</mi></math>",
        "икс принадлежит заглавной а")?;
    return Ok(());
}

#[test]
fn simple_msubsup() -> Result<()> {
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
    test("ru", "ClearSpeak", expr, "икс нижний индекс ка, в степени и")?;
    return Ok(());
}

#[test]
fn non_simple_msubsup() -> Result<()> {
  let expr = "<math><msubsup><mi>i</mi><mrow><mi>j</mi><mo>&#x2212;</mo><mn>2</mn></mrow><mi>k</mi></msubsup></math>";
  test("ru", "SimpleSpeak", expr, "и нижний индекс йот минус 2, в степени ка")?;
  test("ru", "ClearSpeak", expr, "и нижний индекс йот минус 2, в степени ка")?;
  test_prefs("ru", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
          "и нижний индекс йот минус 2, в степени ка")?;
          return Ok(());
}

#[test]
fn presentation_mathml_in_semantics() -> Result<()> {
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
    test("ru", "ClearSpeak", expr, "икс нижний индекс ка, в степени и")?;
    return Ok(());
}

#[test]
fn ignore_period() -> Result<()> {
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
    test("ru", "SimpleSpeak", expr, "заглавная пэ; открывающая круглая скобка, заглавная а и заглавная бэ; закрывающая круглая скобка; равно; заглавная пэ, открывающая круглая скобка, заглавная а пересечение заглавная бэ; закрывающая круглая скобка; равно, заглавная пэ от заглавной а, заглавная пэ от заглавной бэ")?;
    return Ok(());
}

#[test]
fn ignore_mtext_period() -> Result<()> {
    let expr = "<math><mrow><mrow><mo>{</mo><mn>2</mn><mo>}</mo></mrow><mtext>.</mtext></mrow></math>";
    test("ru", "SimpleSpeak", expr, "множество 2")?;
    return Ok(());
}

#[test]
fn ignore_comma() -> Result<()> {
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
    test("ru", "SimpleSpeak", expr, "фи от икс равно; цэ, е в степени минус аш в квадрате, икс в квадрате")?;
    return Ok(());
}

#[test]
fn ignore_period_and_space() -> Result<()> {
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
    test("ru", "ClearSpeak", expr, "заглавная пэ; открывающая круглая скобка, заглавная а при условии заглавная бэ, закрывающая круглая скобка равно дробь, числитель: заглавная пэ, открывающая круглая скобка, заглавная а пересечение заглавная бэ, закрывающая круглая скобка; знаменатель: заглавная пэ от заглавной бэ")?;
    return Ok(());
}

#[test]
fn bug_199_2pi() -> Result<()> {
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
  test("ru", "SimpleSpeak",expr, "полуинтервал от 0 до 2 пи")?;
  return Ok(());
}

#[test]
fn caret_and_hat() -> Result<()> {
  let expr = "<math><mi>x</mi><mo>^</mo><mn>2</mn><mo>+</mo><mover><mi>y</mi><mo>^</mo></mover></math>";
  test("ru", "SimpleSpeak",expr, "икс крышка 2 плюс игрек циркумфлекс")?;
  return Ok(());
}

#[test]
fn mn_with_space() -> Result<()> {
  let expr = "<math><mn>1 234 567</mn></math>";
  test_prefs("ru", "SimpleSpeak", vec![("DecimalSeparators", "."), ("BlockSeparators", " ,")], expr, "1234567")?;
  return Ok(());
}

#[test]
fn mn_with_block_and_decimal_separators() -> Result<()> {
  let expr = "<math><mn>1,234.56</mn></math>";                                      
  test_prefs("ru", "SimpleSpeak", vec![("DecimalSeparators", "."), ("BlockSeparators", " ,")], expr, "1234.56")?;
  return Ok(());
}

#[test]
fn named_operators_with_lower_limits() -> Result<()> {
  // TeX4ht wraps named operators and the invisible apply character in the munder base.
  let max_expr = "<math><munder><mrow><mi class='qopname' mathvariant='normal'>max</mi><mo>⁡</mo></mrow><mi>i</mi></munder></math>";
  let lim_expr = "<math><munder><mrow><mi class='qopname' mathvariant='normal'>lim</mi><mo>⁡</mo></mrow><mrow><mi>x</mi><mo>→</mo><mi>∞</mi></mrow></munder></math>";
  test("ru", "SimpleSpeak", max_expr, "максимум по и")?;
  test("ru", "ClearSpeak", max_expr, "максимум по и")?;
  test("ru", "SimpleSpeak", lim_expr, "предел при икс стремящемся к бесконечности")?;
  test("ru", "ClearSpeak", lim_expr, "предел при икс стремящемся к бесконечности")?;
  return Ok(());
}

#[test]
fn divergence() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xB7;</mo><mi mathvariant='normal'>F</mi></math>";                                       
  test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "див заглавной эф")?;
  test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "дивергенция заглавной эф")?;
  return Ok(());
}

#[test]
fn curl() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mo>&#xD7;</mo><mi mathvariant='normal'>F</mi></math>";          
    test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "ротор заглавной эф")?;
  test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "ротор заглавной эф")?;
  return Ok(());
}

#[test]
fn gradient() -> Result<()> {
  let expr = "<math><mo>&#x2207;</mo><mi mathvariant='normal'>F</mi></math>";          
  test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "набла заглавной эф")?;
  test_prefs("ru", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "градиент заглавной эф")?;
  return Ok(());
}

#[test]
fn literal_speak() -> Result<()> {
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
  test("ru", "LiteralSpeak", expr, "заглавная а со стрелкой перпендикулярно заглавная бэ со стрелкой")?;
  return Ok(());
}

#[test]
fn literal_speak_with_name() -> Result<()> {
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
  test("ru", "LiteralSpeak", expr, "форсид эф, открывающая круглая скобка икс восклицательный знак, закрывающая круглая скобка")?;
  return Ok(());
}

#[test]
fn literal_speak_with_property() -> Result<()> {
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
  test("ru", "LiteralSpeak", expr, "эф, открывающая круглая скобка икс восклицательный знак, закрывающая круглая скобка")?;
  return Ok(());
}

#[test]
fn literal_intent_property() -> Result<()> {
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
  test("ru", "SimpleSpeak", expr, "заглавная а со стрелкой перпендикулярно заглавная бэ со стрелкой")?;
  return Ok(());
}

#[test]
fn literal_intent_property_with_name() -> Result<()> {
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
  test("ru", "SimpleSpeak", expr, "форсид эф, открывающая круглая скобка икс восклицательный знак, закрывающая круглая скобка")?;
  return Ok(());
}
