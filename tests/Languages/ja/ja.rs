use crate::common::*;
use anyhow::Result;

/// Verifies that basic arithmetic operators use the seeded Japanese names.
#[test]
fn arithmetic_operators() -> Result<()> {
    let expr = "<math><mn>1</mn><mo>+</mo><mn>2</mn><mo>&#x2212;</mo><mn>3</mn><mo>=</mo><mn>0</mn></math>";
    test("ja", "ClearSpeak", expr, "1 プラス 2 マイナス 3; イコール 0")?;
    return Ok(());
}

/// A fraction of two numbers is read denominator-first in Japanese:
/// 21/22 is "22 分の 21", literally "of 22, 21". Reading it the other way round
/// says 22/21.
#[test]
fn simple_fraction() -> Result<()> {
    let expr = "<math><mfrac><mn>21</mn><mn>22</mn></mfrac></math>";
    test("ja", "ClearSpeak", expr, "22 分の 21")?;
    test("ja", "SimpleSpeak", expr, "22 分の 21")?;
    return Ok(());
}

/// The denominator-first pattern is not limited to the small numbers that English
/// has ordinals for ("three fourths"); it is how any two numbers are read.
#[test]
fn numeric_fraction_large_denominator() -> Result<()> {
    let expr = "<math><mfrac><mn>3</mn><mn>128</mn></mfrac></math>";
    test("ja", "ClearSpeak", expr, "128 分の 3")?;
    return Ok(());
}

/// When the parts are not plain numbers, Japanese keeps the written order and
/// borrows the English preposition as "オーバー" instead (Yamaguchi et al. 1996).
#[test]
fn fraction_of_variables() -> Result<()> {
    let expr = "<math><mfrac><mi>x</mi><mi>y</mi></mfrac></math>";
    test("ja", "ClearSpeak", expr, "x オーバー y")?;
    test("ja", "SimpleSpeak", expr, "x オーバー y")?;
    return Ok(());
}

/// Verifies that a square root uses the seeded Japanese root wording.
#[test]
fn square_root() -> Result<()> {
    let expr = "<math><msqrt><mn>9</mn></msqrt></math>";
    test("ja", "ClearSpeak", expr, "平方根 オブ 9")?;
    return Ok(());
}

/// An exponent is read "<base> の <exponent> 乗"; 乗 closes it. Japanese has no
/// ordinal form here, so 2 is 2 and not "second".
#[test]
fn squared() -> Result<()> {
    let expr = "<math><msup><mn>3</mn><mn>2</mn></msup></math>";
    test("ja", "ClearSpeak", expr, "3 の 2 乗")?;
    test("ja", "SimpleSpeak", expr, "3 の 2 乗")?;
    return Ok(());
}

/// Same shape for a cube.
#[test]
fn cubed() -> Result<()> {
    let expr = "<math><msup><mn>5</mn><mn>3</mn></msup></math>";
    test("ja", "ClearSpeak", expr, "5 の 3 乗")?;
    test("ja", "SimpleSpeak", expr, "5 の 3 乗")?;
    return Ok(());
}

/// The pattern does not change for exponents above three.
#[test]
fn integer_exponent() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mn>5</mn></msup></math>";
    test("ja", "ClearSpeak", expr, "x の 5 乗")?;
    test("ja", "SimpleSpeak", expr, "x の 5 乗")?;
    return Ok(());
}

/// A variable exponent is read the same way (English adds "-th" here; Japanese does not).
#[test]
fn variable_exponent() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mi>n</mi></msup></math>";
    test("ja", "ClearSpeak", expr, "x の n 乗")?;
    test("ja", "SimpleSpeak", expr, "x の n 乗")?;
    return Ok(());
}

/// A complex exponent is read as a superscript instead, with an explicit close:
/// 「の上付き … 上付き終了」. Reading it as 乗 would end a nested exponent 「… 乗 乗」.
#[test]
fn complex_exponent() -> Result<()> {
    let expr = "<math><msup><mi>x</mi><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow></msup></math>";
    test("ja", "SimpleSpeak", expr, "x の上付き y プラス 1 上付き終了")?;
    return Ok(());
}

/// A leading minus is read マイナス, the same word as the binary operator.
/// 負の / 正の name the *kind* of number (負の数 = "the negative numbers") and are
/// not how −5 is read aloud.
#[test]
fn negative_number() -> Result<()> {
    let expr = "<math><mo>&#x2212;</mo><mn>5</mn></math>";
    test("ja", "ClearSpeak", expr, "マイナス 5")?;
    test("ja", "SimpleSpeak", expr, "マイナス 5")?;
    return Ok(());
}

/// Verifies both Japanese gradient readings selected by verbosity.
#[test]
fn gradient() -> Result<()> {
    let expr = "<math><mo>&#x2207;</mo><mi mathvariant='normal'>F</mi></math>";
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "デル 大文字 f")?;
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "勾配 オブ 大文字 f")?;
    return Ok(());
}

/// These four reach the generic function-application rule through the
/// function= entries in definitions.yaml, so they take オブ like any other
/// function. SharedRules/calculus.yaml also names them, but neither ja nor en
/// includes that file today, so it is not the path under test here.
#[test]
fn vector_calculus_operators() -> Result<()> {
    for (intent, expected) in [
        ("curl", "回転 オブ x"),
        ("divergence", "発散 オブ x"),
        ("gradient", "勾配 オブ x"),
        ("laplacian", "ラプラシアン オブ x"),
    ] {
        let expr = format!("<math><mrow intent='{intent}($x)'><mi arg='x'>x</mi></mrow></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// Verifies that multiplication and division symbols use Japanese operator names.
#[test]
fn multiplication_and_division() -> Result<()> {
    let expr = "<math><mn>6</mn><mo>&#x00D7;</mo><mn>4</mn><mo>&#x00F7;</mo><mn>2</mn></math>";
    test("ja", "ClearSpeak", expr, "6 掛ける 4 割る 2")?;
    return Ok(());
}

/// Verifies that explicit parentheses retain Japanese opening and closing cues.
#[test]
fn parenthesized_expression() -> Result<()> {
    let expr = "<math><mrow><mo>(</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>)</mo></mrow></math>";
    test("ja", "ClearSpeak", expr, "丸括弧 1 プラス 2, 丸括弧閉じ")?;
    return Ok(());
}

/// Square and curly brackets follow the same pattern: the opening bracket names
/// the shape and the closing one adds the postposed cue (Yamaguchi et al. 1996).
#[test]
fn square_and_curly_brackets() -> Result<()> {
    let square = "<math><mrow><mo>[</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>]</mo></mrow></math>";
    test("ja", "ClearSpeak", square, "角括弧 1 プラス 2, 角括弧閉じ")?;
    let curly = "<math><mrow><mo>{</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>}</mo></mrow></math>";
    test("ja", "ClearSpeak", curly, "中括弧 1 プラス 2, 中括弧閉じ")?;
    return Ok(());
}

/// Verifies the Japanese ClearSpeak wording for an absolute value.
#[test]
fn absolute_value() -> Result<()> {
    let expr = "<math><mrow><mo>|</mo><mi>x</mi><mo>|</mo></mrow></math>";
    test("ja", "ClearSpeak", expr, "絶対値 x")?;
    return Ok(());
}

/// With AbsEnd the closing cue is spoken after the contents, not before it, so
/// the bar that ends the group is heard where it actually is.
#[test]
fn absolute_value_abs_end() -> Result<()> {
    let expr = "<math><mrow><mo>|</mo><mi>x</mi><mo>|</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_AbsoluteValue", "AbsEnd", expr, "絶対値 x, 絶対値 閉じ")?;
    return Ok(());
}

/// The four bounded interval forms. English says "not including c or d"; a slot-by-slot
/// translation would use または and change the meaning, because negation does not
/// distribute over a Japanese disjunction. Each endpoint takes its own verb instead,
/// with the first in the continuative form.
#[test]
fn interval_open_open() -> Result<()> {
    let expr = "<math><mrow><mo>(</mo><mrow><mi>c</mi><mo>,</mo><mi>d</mi></mrow><mo>)</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Paren", "Interval", expr,
        "区間 c から d まで, c を含まず d を含まない")?;
    return Ok(());
}

#[test]
fn interval_closed_closed() -> Result<()> {
    let expr = "<math><mrow><mo>[</mo><mrow><mi>c</mi><mo>,</mo><mi>d</mi></mrow><mo>]</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Paren", "Interval", expr,
        "区間 c から d まで, c を含み d を含む")?;
    return Ok(());
}

#[test]
fn interval_closed_open() -> Result<()> {
    let expr = "<math><mrow><mo>[</mo><mrow><mi>c</mi><mo>,</mo><mi>d</mi></mrow><mo>)</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Paren", "Interval", expr,
        "区間 c から d まで, c を含み d を含まない")?;
    return Ok(());
}

#[test]
fn interval_open_closed() -> Result<()> {
    let expr = "<math><mrow><mo>(</mo><mrow><mi>c</mi><mo>,</mo><mi>d</mi></mrow><mo>]</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Paren", "Interval", expr,
        "区間 c から d まで, c を含まず d を含む")?;
    return Ok(());
}

/// Verifies that an indexed cube root receives the Japanese cube-root cue.
#[test]
fn cube_root() -> Result<()> {
    let expr = "<math><mroot><mn>8</mn><mn>3</mn></mroot></math>";
    test("ja", "ClearSpeak", expr, "立方根 オブ 8")?;
    return Ok(());
}

/// An n-th root is 「n 乗根」 -- the index, then 乗根. English builds an ordinal
/// ("the fifth root"); Japanese has no such form and reads the number as it is.
#[test]
fn nth_root() -> Result<()> {
    let expr = "<math><mroot><mi>x</mi><mn>5</mn></mroot></math>";
    test("ja", "ClearSpeak", expr, "5 乗根 オブ x")?;
    test("ja", "SimpleSpeak", expr, "5 乗根 オブ x")?;
    return Ok(());
}

/// The same holds when the index is a variable (English appends "-th" here).
#[test]
fn variable_index_root() -> Result<()> {
    let expr = "<math><mroot><mi>x</mi><mi>n</mi></mroot></math>";
    test("ja", "ClearSpeak", expr, "n 乗根 オブ x")?;
    return Ok(());
}

/// Verifies the basic Japanese SimpleSpeak subscript pattern.
#[test]
fn subscript() -> Result<()> {
    let expr = "<math><msub><mi>x</mi><mn>1</mn></msub></math>";
    test("ja", "SimpleSpeak", expr, "x サブ 1")?;
    return Ok(());
}

/// Verifies that a common trigonometric function is spoken in Japanese SimpleSpeak.
#[test]
fn sine_function() -> Result<()> {
    let expr = "<math><mi>sin</mi><mo>&#x2061;</mo><mi>x</mi></math>";
    test("ja", "SimpleSpeak", expr, "サイン オブ x")?;
    return Ok(());
}

/// Verifies Japanese speech for the less-than comparison operator.
#[test]
fn less_than() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&lt;</mo><mn>5</mn></math>";
    test("ja", "ClearSpeak", expr, "x は 小なり 5")?;
    return Ok(());
}

/// 小なり / 大なり are the pair the source gives for < and >. より大きい stranded the
/// より, which needs its comparand in front of it, so "x は より大きい 5" read as
/// "x is a bigger 5"; and it did not match the 小なり already used for <.
#[test]
fn greater_than() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&gt;</mo><mn>5</mn></math>";
    test("ja", "ClearSpeak", expr, "x は 大なり 5")?;
    return Ok(());
}

/// ≠ is ノット・イコール, the partner of the イコール already used for =.
#[test]
fn not_equal() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&#x2260;</mo><mn>5</mn></math>";
    test("ja", "ClearSpeak", expr, "x は ノット イコール 5")?;
    return Ok(());
}

/// ∈ is 要素オブ. 要素の attaches backwards -- 要素の A is "A of an element" -- and
/// the rules for ∈ and ∊ did not even agree with each other.
#[test]
fn element_of() -> Result<()> {
    for ch in ["&#x2208;", "&#x220a;"] {
        let expr = format!("<math><mi>x</mi><mo>{ch}</mo><mi>A</mi></math>");
        test("ja", "SimpleSpeak", &expr, "x は 要素オブ, 大文字 エー")?;
        test_ClearSpeak("ja", "ClearSpeak_SetMemberSymbol", "Element", &expr,
            "x 要素オブ 大文字 エー")?;
    }
    return Ok(());
}

/// Inside a set the ClearSpeak options take a different branch of the same rule.
/// 中へ is "into", a direction of motion, not membership. The shape is the one
/// en/ClearSpeak/sets.rs uses for set-builder notation.
#[test]
fn set_builder_member_symbol() -> Result<()> {
    for ch in ["&#x2208;", "&#x220a;"] {
        let expr = format!(
            "<math><mo>{{</mo><mi>x</mi><mo>{ch}</mo><mi>&#x2124;</mi><mo>:</mo>             <mi>x</mi><mo>&#x003E;</mo><mn>5</mn><mo>}}</mo></math>"
        );
        test_ClearSpeak("ja", "ClearSpeak_SetMemberSymbol", "In", &expr,
            "集合 すべて x イン 整数 そのようなこと x は 大なり 5")?;
        test_ClearSpeak("ja", "ClearSpeak_SetMemberSymbol", "Element", &expr,
            "集合 すべて x 要素オブ 整数 そのようなこと x は 大なり 5")?;
    }
    return Ok(());
}

/// ∾ is "most positive" in the numeric sense; 最も肯定的な is "most affirmative".
#[test]
fn most_positive() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&#x223e;</mo><mi>y</mi></math>";
    test("ja", "ClearSpeak", expr, "x は 最も正の y")?;
    return Ok(());
}

/// A hat over a variable is ハット. 帽子 is the thing you wear, and it was said
/// twice.
#[test]
fn accent_hat() -> Result<()> {
    let expr = "<math><mover><mi>x</mi><mo>^</mo></mover></math>";
    test("ja", "ClearSpeak", expr, "x ハット")?;
    return Ok(());
}

/// A tilde is チルダ. チルド is the loanword for "chilled" (as in chilled food).
#[test]
fn accent_tilde() -> Result<()> {
    let expr = "<math><mover><mi>x</mi><mo>~</mo></mover></math>";
    test("ja", "ClearSpeak", expr, "x チルダ")?;
    return Ok(());
}

/// The degree sign is 度. キーワード ("keyword") is not a unit of angle.
#[test]
fn degree_sign() -> Result<()> {
    let expr = "<math><mn>90</mn><mo>&#xb0;</mo></math>";
    test("ja", "ClearSpeak", expr, "90 度")?;
    return Ok(());
}

/// The negation sign is ノット. The seed said コメントはありません -- "there are no
/// comments" -- because the English source word is "not".
#[test]
fn logical_not() -> Result<()> {
    let expr = "<math><mo>&#xac;</mo><mi>p</mi></math>";
    test("ja", "ClearSpeak", expr, "ノット p")?;
    return Ok(());
}

/// Geometry has settled Japanese terms: 線分, 半直線, 弧, 点. The katakana
/// transliterations of the English words are not used for these.
#[test]
fn geometry_terms() -> Result<()> {
    for (intent, expected) in [
        ("line-segment", "線分 x y"),
        ("directed-line-segment", "有向線分 x y"),
        ("line", "直線 x y"),
        ("ray", "半直線 x y"),
        ("arc", "弧 x y"),
    ] {
        let expr = format!(
            "<math><mrow intent='{intent}($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>"
        );
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// A point is 点, not ポイント.
#[test]
fn geometry_point() -> Result<()> {
    let expr = "<math><mrow intent='point($x,$y,$z)'><mi arg='x'>x</mi><mi arg='y'>y</mi><mi arg='z'>z</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "点 x y z")?;
    return Ok(());
}

/// Verbose keeps the head term first, as the reference asks, but a ray and a
/// segment differ in what the second point is: a segment stops at it, a ray only
/// passes through it. 「まで」 would claim the ray ends at B.
#[test]
fn geometry_verbose_from_to() -> Result<()> {
    let seg = "<math><mrow intent='line-segment($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>";
    test_prefs("ja", "ClearSpeak", vec![("Verbosity", "Verbose")], seg, "線分 x から y まで")?;
    let ray = "<math><mrow intent='ray($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>";
    test_prefs("ja", "ClearSpeak", vec![("Verbosity", "Verbose")], ray, "半直線 x を始点として y を通る")?;
    return Ok(());
}

/// The trigonometric functions have settled Japanese names. 接線 and 割線 are the
/// tangent *line* and the secant *line* -- curves, not the functions -- and 探す is
/// the verb "to search", so none of them can be spoken for tan/sec.
#[test]
fn trigonometric_function_names() -> Result<()> {
    for (name, expected) in [
        ("cos", "コサイン オブ x"),
        ("tan", "タンジェント オブ x"),
        ("sec", "セカント オブ x"),
        ("csc", "コセカント オブ x"),
        ("cot", "コタンジェント, オブ x"),
    ] {
        let expr = format!("<math><mi>{name}</mi><mo>&#x2061;</mo><mi>x</mi></math>");
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// The six hyperbolic functions are 双曲線正弦 through 双曲線余接.
#[test]
fn hyperbolic_function_names() -> Result<()> {
    for (name, expected) in [
        ("sinh", "双曲線正弦 オブ x"),
        ("cosh", "双曲線余弦 オブ x"),
        ("tanh", "双曲線正接 オブ x"),
        ("sech", "双曲線正割 オブ x"),
        ("csch", "双曲線余割 オブ x"),
        ("coth", "双曲線余接 オブ x"),
    ] {
        let expr = format!("<math><mi>{name}</mi><mo>&#x2061;</mo><mi>x</mi></math>");
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// Terse reads the abbreviation aloud instead of the formal name.
#[test]
fn hyperbolic_function_terse() -> Result<()> {
    let expr = "<math><mi>tanh</mi><mo>&#x2061;</mo><mi>x</mi></math>";
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "ハイパーボリックタンジェント, x")?;
    return Ok(());
}

/// The parts of a complex number are 実部 and 虚部, and its conjugate is 複素共役.
/// 実際の部分 ("the actual portion") and 想像上の部分 ("an imagined portion") are
/// the everyday senses of "real" and "imaginary", not the mathematical ones.
#[test]
fn complex_number_parts() -> Result<()> {
    for (intent, expected) in [
        ("real-part", "実部 オブ x"),
        ("imaginary-part", "虚部 オブ x"),
        ("complex-conjugate", "複素共役 オブ x"),
        ("complex-arg", "偏角 オブ x"),
    ] {
        let expr = format!("<math><mrow intent='{intent}($x)'><mi arg='x'>x</mi></mrow></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// The inverse trigonometric functions are 逆正弦 through 逆余接.
#[test]
fn inverse_trigonometric_names() -> Result<()> {
    for (intent, expected) in [
        ("arcsine", "逆正弦 オブ x"),
        ("arccosine", "逆余弦 オブ x"),
        ("arctangent", "逆正接 オブ x"),
        ("arcsecant", "逆正割 オブ x"),
        ("arccosecant", "逆余割 オブ x"),
        ("arccotangent", "逆余接 オブ x"),
    ] {
        let expr = format!("<math><mrow intent='{intent}($x)'><mi arg='x'>x</mi></mrow></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// Verifies that common lowercase Greek letters use their Japanese names.
#[test]
fn greek_letters() -> Result<()> {
    let expr = "<math><mi>&#x03B1;</mi><mo>+</mo><mi>&#x03B2;</mi></math>";
    test("ja", "ClearSpeak", expr, "アルファ プラス ベータ")?;
    return Ok(());
}

/// Verifies Japanese SimpleSpeak wording for set membership.
#[test]
fn set_membership() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&#x2208;</mo><mi mathvariant='double-struck'>R</mi></math>";
    test("ja", "SimpleSpeak", expr, "x は 要素オブ 実数")?;
    return Ok(());
}

/// The seed read the variable a as イーグル -- an eagle. The English rule says
/// "eigh" only because the letter a and the article "a" sound alike in English;
/// the note to translators at the top of unicode.yaml says most languages do not
/// need this. Japanese reads the letter as エー, and 大文字 エー when capital.
#[test]
fn letter_a() -> Result<()> {
    let expr = "<math><mi>a</mi><mo>+</mo><mi>b</mi></math>";
    test("ja", "ClearSpeak", expr, "エー プラス b")?;
    return Ok(());
}

/// Five Greek letters were wrong and ξ was silent: it had an empty string, so a
/// formula using it simply skipped the variable. ゼタ is the SI prefix zetta,
/// オクタ is "octa", and プッシー is an offensive English word.
#[test]
fn greek_letters_that_were_wrong() -> Result<()> {
    for (letter, expected) in [
        ("&#x3b6;", "ゼータ"),
        ("&#x3b9;", "イオタ"),
        ("&#x3be;", "クサイ"),
        ("&#x3c5;", "ウプシロン"),
        ("&#x3c8;", "プサイ"),
    ] {
        let expr = format!("<math><mi>{letter}</mi></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// ∂ said 部分的な派生物 -- "a partially derived object". The symbol is read
/// ラウンド and the concept is 偏微分.
#[test]
fn partial_derivative_symbol() -> Result<()> {
    let expr = "<math><mo>&#x2202;</mo></math>";
    test("ja", "ClearSpeak", expr, "偏微分")?;
    return Ok(());
}

/// Set membership was worded as club membership: メンバー, 会員でない
/// ("not a club member") and 所属団体 ("the organization one belongs to").
#[test]
fn set_non_membership() -> Result<()> {
    let expr = "<math><mi>x</mi><mo>&#x2209;</mo><mi>y</mi></math>";
    test("ja", "ClearSpeak", expr, "x 元でない y")?;
    return Ok(());
}

/// 平行 and 垂直 are the geometric relations. The seed used 平行へ and 垂直へ,
/// which attach a direction particle that cannot follow a noun this way, and
/// wrote the negative form with 並行 -- a different word, meaning "concurrent".
#[test]
fn parallel_and_perpendicular() -> Result<()> {
    for (op, expected) in [
        ("&#x2225;", "x は 平行 y"),
        ("&#x2226;", "x は 平行でない y"),
    ] {
        let expr = format!("<math><mi>x</mi><mo>{op}</mo><mi>y</mi></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// ≤ said より少しまたは等しい ("a little more, or equal") and ≥ ended in the
/// particle へ. They then said より小さいか等しい / より大きいか等しい, which strand the
/// より the same way > did; the source gives 小なり・オア・イコール.
#[test]
fn comparison_with_equality() -> Result<()> {
    for (op, expected) in [
        ("&#x2264;", "x は 小なり オア イコール 5"),
        ("&#x2265;", "x は 大なり オア イコール 5"),
    ] {
        let expr = format!("<math><mi>x</mi><mo>{op}</mo><mn>5</mn></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// A factorial is 階乗. ファクシャル is not a Japanese word.
#[test]
fn factorial() -> Result<()> {
    let expr = "<math><mn>5</mn><mo>!</mo></math>";
    test("ja", "ClearSpeak", expr, "5 階乗")?;
    return Ok(());
}

/// Verifies the seeded Japanese cues for a summation with limits.
#[test]
fn summation() -> Result<()> {
    let expr = "<math><munderover><mo>&#x2211;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><mi>i</mi></math>";
    test("ja", "SimpleSpeak", expr, "総和 i イコール 1 から, n まで オブ i")?;
    return Ok(());
}

/// A large operator with only a lower limit takes the postposed "over" cue.
#[test]
fn summation_lower_limit_only() -> Result<()> {
    let expr = "<math><munder><mo>&#x2211;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow></munder><mi>i</mi></math>";
    test("ja", "SimpleSpeak", expr, "総和 i イコール 1 にわたる オブ i")?;
    return Ok(());
}

/// The same shape is used for the other large operators.
#[test]
fn product_with_limits() -> Result<()> {
    let expr = "<math><munderover><mo>&#x220F;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><mi>i</mi></math>";
    test("ja", "SimpleSpeak", expr, "プロダクト i イコール 1 から, n まで オブ i")?;
    return Ok(());
}

/// Verifies the seeded Japanese cues for a definite integral.
#[test]
fn definite_integral() -> Result<()> {
    let expr = "<math><msubsup><mo>&#x222B;</mo><mn>0</mn><mn>1</mn></msubsup><mi>x</mi><mo>&#x2146;</mo><mi>x</mi></math>";
    test("ja", "SimpleSpeak", expr, "積分 0 から, 1 まで オブ; x 微分 d x")?;
    return Ok(());
}

/// Set-theory names. セット/空のセット are the loanword for a set of objects; the
/// mathematical term is 集合.
#[test]
fn set_terminology() -> Result<()> {
    let expr = "<math><mrow intent='set($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "集合 オブ x コンマ, y")?;
    return Ok(());
}

/// 規範 is a social norm; the norm of a vector is ノルム.
#[test]
fn norm_terminology() -> Result<()> {
    let expr = "<math><mrow intent='norm($x)'><mi arg='x'>x</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "ノルム オブ x")?;
    return Ok(());
}

/// 限界 is a bound or a ceiling and 傾向がある is "has a tendency"; neither is how
/// x → a is read. The mathematical terms are 極限 and に近づく.
#[test]
fn limit_terminology() -> Result<()> {
    let expr = "<math><mrow intent='tends-to($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "x に近づく y")?;
    return Ok(());
}

/// モード is the loanword; the statistical term is 最頻値.
#[test]
fn statistics_mode() -> Result<()> {
    let expr = "<math><mrow intent='mode($x)'><mi arg='x'>x</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "最頻値 オブ x")?;
    return Ok(());
}

/// A matrix is 行列, not the loanword マトリクス, and the dimension separator is
/// かける, not によって ("by means of").
#[test]
fn matrix_terminology() -> Result<()> {
    let expr = "<math><mrow><mo>[</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>";
    test("ja", "ClearSpeak", expr, "1 かける 2 行 行列; 1, 2")?;
    return Ok(());
}

/// Every number set, not just ℝ, has a settled Japanese name. ℝ was the only one
/// with a test, which is how 合理的な数字 ("reasonable numbers") and 整数者
/// ("integer person") survived in the other branches of the same rule.
#[test]
fn number_set_names() -> Result<()> {
    for (letter, expected) in [
        ("C", "x は 要素オブ 複素数"),
        ("N", "x は 要素オブ 自然数"),
        ("Q", "x は 要素オブ 有理数"),
        ("R", "x は 要素オブ 実数"),
        ("Z", "x は 要素オブ 整数"),
    ] {
        let expr = format!(
            "<math><mi>x</mi><mo>&#x2208;</mo><mi mathvariant='double-struck'>{letter}</mi></math>"
        );
        test("ja", "SimpleSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// In geometry a translation is 平行移動. 翻訳 is translation between languages.
#[test]
fn geometry_translation() -> Result<()> {
    let expr = "<math><mrow intent='translation($x)'><mi arg='x'>x</mi></mrow></math>";
    test("ja", "ClearSpeak", expr, "平行移動 オブ x")?;
    return Ok(());
}

/// 分割する is to split something into parts; divisibility is 割り切る. Similar
/// figures are 相似, not と同様 ("the same as").
#[test]
fn divides_and_similar() -> Result<()> {
    for (intent, expected) in [
        ("divides", "x 割り切る y"),
        ("similar", "x 相似 y"),
    ] {
        let expr = format!(
            "<math><mrow intent='{intent}($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>"
        );
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// nPk permutation notation is read with the standard Japanese school phrase
/// and in display order: (superscript 5) P (subscript 2) is
/// "5 個から 2 個取る順列" ("permutations taking 2 out of 5").
/// en reverses the arguments ("2 permutations of 5"); Japanese does not need to.
#[test]
fn permutation_p_notation() -> Result<()> {
    let expr = "<math><msubsup><mi>P</mi><mn>2</mn><mn>5</mn></msubsup></math>";
    test("ja", "ClearSpeak", expr, "5 個から 2 個取る順列")?;
    test("ja", "SimpleSpeak", expr, "5 個から 2 個取る順列")?;
    return Ok(());
}

/// Multi-line labels: a system of equations labelled as cases is announced as
/// "2 ケース" and each line as "ケース 1", "ケース 2" (the seed doubled the word
/// and used 設備 "equipment" for equation).
#[test]
fn multiline_case_label() -> Result<()> {
    let expr = "<math>
     <mrow>
      <mtable>
       <mtr> <mtd> <mrow> <mi>x</mi><mo>+</mo><mi>y</mi></mrow> </mtd>  <mtd><mo>=</mo> </mtd>  <mtd><mn>7</mn></mtd>  </mtr>
       <mtr> <mtd> <mrow> <mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mrow></mtd>  <mtd><mo>=</mo></mtd>  <mtd><mrow><mn>17</mn></mrow></mtd> </mtr>
      </mtable></mrow>
    </math>";
    // SimpleSpeak reaches the same labels through SharedRules/general.yaml, and
    // classifies this as a system of equations rather than cases: 2 式, 式 1, 式 2.
    test("ja", "SimpleSpeak", expr, "2 式; 式 1; x プラス y, イコール 7; 式 2; 2 x プラス 3 y; イコール 17")?;
    test_ClearSpeak("ja", "ClearSpeak_MultiLineLabel", "Case", expr,
        "2 ケース; ケース 1; x プラス y, イコール 7; ケース 2; 2 x プラス 3 y; イコール 17")?;
    return Ok(());
}

/// menclose with a line on one side: the side comes first and the noun last,
/// "左に 線" ("a line on the left"); the seed said ライン アクセス for line-on-right.
#[test]
fn menclose_line_on_left() -> Result<()> {
    let expr = "<math>
                    <menclose notation='left'>  <mfrac><mn>3</mn><mn>2</mn></mfrac> </menclose>
                </math>";
    test("ja", "ClearSpeak", expr, "左に 線, 囲み 2 分の 3 囲み終了")?;
    return Ok(());
}

/// The letter names in the dimension form of a number set (N squared) are the
/// katakana letter names. ネクタイ is a necktie.
#[test]
fn number_set_dimension_letters() -> Result<()> {
    for (letter, expected) in [
        ("ℂ", "シー 2"),
        ("ℕ", "エヌ 2"),
        ("ℚ", "キュー 2"),
        ("ℝ", "アール 2"),
        ("ℤ", "ゼット 2"),
    ] {
        let expr = format!("<math><msup><mi>{letter}</mi><mn>2</mn></msup></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// A vector is ベクトル; コンテンツ is "content". The end marker is one word with
/// the noun first, matching 行列終了 already used in the same file.
#[test]
fn column_vector_end_vector() -> Result<()> {
    let expr = "<math display='block'>
        <mrow><mo>(</mo><mrow><mtable>
          <mtr><mtd><mn>1</mn></mtd></mtr>
          <mtr><mtd><mn>2</mn></mtd></mtr>
          <mtr><mtd><mn>3</mn></mtd></mtr>
        </mtable></mrow><mo>)</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "EndVector",
        expr, "3 かける 1 列 ベクトル; 1; 2; 3; ベクトル終了")?;
    return Ok(());
}

/// The row form, and the matrix form of the same end marker: 行列終了, not
/// リリース 行列 ("release matrix").
#[test]
fn row_vector_and_matrix_end_markers() -> Result<()> {
    let row = "<math display='block'>
        <mrow><mo>[</mo><mrow><mtable>
          <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr>
        </mtable></mrow><mo>]</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "EndVector",
        row, "1 かける 2 行 ベクトル; 1, 2; ベクトル終了")?;
    let square = "<math display='block'>
        <mrow><mo>(</mo><mrow><mtable>
          <mtr><mtd><mn>2</mn></mtd><mtd><mn>1</mn></mtd></mtr>
          <mtr><mtd><mn>7</mn></mtd><mtd><mn>5</mn></mtd></mtr>
        </mtable></mrow><mo>)</mo></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "EndMatrix",
        square, "2 かける 2 行列; 行 1; 2, 1; 行 2; 7, 5; 行列終了")?;
    return Ok(());
}

/// Chemical states use the terms taught in Japanese chemistry: 固体 液体 気体
/// 水溶液. ガスレンジ is a gas cooker and アキュース is not a word.
#[test]
fn chemical_states() -> Result<()> {
    for (state, expected) in [
        ("s", "大文字 n エー; 固体"),
        ("l", "大文字 n エー; 液体"),
        ("g", "大文字 n エー; 気体"),
        ("aq", "大文字 n エー; 水溶液"),
    ] {
        let expr = format!(
            "<math><mrow><mi>Na</mi><mrow><mo>(</mo><mrow><mi>{state}</mi></mrow><mo>)</mo></mrow></mrow></math>"
        );
        test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], &expr, expected)?;
    }
    return Ok(());
}

/// Bond names are 単結合 二重結合 三重結合 四重結合, not transliterations;
/// 四倍の結束 is "quadruple solidarity", not a chemistry term.
#[test]
fn chemical_bonds() -> Result<()> {
    let ethylene = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>=</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
    test_prefs("ja", "SimpleSpeak", vec![("Verbosity", "Terse")], ethylene,
        "大文字 h, 2 大文字 c, 二重結合 大文字 c, 大文字 h, 2")?;
    return Ok(());
}

/// The short readings of sub and superscript are 下付き and 上付き, the same
/// stems as the verbose 下付き文字 / 上付き文字 in the branch beside them.
/// サブサブ is the word "sub" doubled and スーパー is a supermarket.
#[test]
fn sub_and_superscript_medium() -> Result<()> {
    let sulfate = "<math><mrow><msup>
          <mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
          <mrow><mn>2</mn><mo>&#x2212;</mo></mrow>
      </msup></mrow></math>";
    test_prefs("ja", "ClearSpeak", vec![("Verbosity", "Medium")], sulfate,
        "角括弧, 大文字 s, 大文字 o, 下付き 4; 角括弧閉じ 上付き 2 マイナス")?;
    return Ok(());
}

/// A determinant ends with 行列式終了, built the same way as 行列終了.
#[test]
fn determinant_end_marker() -> Result<()> {
    let expr = "<math><mrow><mrow><mo>|</mo>
        <mtable>
          <mtr><mtd><mn>2</mn></mtd><mtd><mn>1</mn></mtd></mtr>
          <mtr><mtd><mn>7</mn></mtd><mtd><mn>5</mn></mtd></mtr>
        </mtable>
      <mo>|</mo></mrow></mrow></math>";
    test_ClearSpeak("ja", "ClearSpeak_Matrix", "EndMatrix",
        expr, "2 かける 2 行列式; 行 1; 2, 1; 行 2; 7, 5; 行列式終了")?;
    return Ok(());
}

/// LiteralSpeak reads what is written, and Japanese writes the denominator
/// first: 3/4 is "4 分の 3". "3 分の 4" is the other number, 4/3.
#[test]
fn literal_speak_simple_fraction() -> Result<()> {
    let expr = "<math><mfrac><mn>3</mn><mn>4</mn></mfrac></math>";
    test("ja", "LiteralSpeak", expr, "4 分の 3")?;
    return Ok(());
}

/// A fraction that is not two leaves is bracketed instead, and then the written
/// order is kept with the borrowed オーバー, as ClearSpeak already does.
#[test]
fn literal_speak_bracketed_fraction() -> Result<()> {
    let expr = "<math><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mn>2</mn></mfrac></math>";
    test("ja", "LiteralSpeak", expr, "分数, x プラス 1, オーバー 2, 分数終了")?;
    return Ok(());
}

/// The overview rules (what the "describe" navigation command speaks) have their
/// own fraction rule, and it has to say the denominator first as well.
#[test]
fn overview_fraction_is_denominator_first() -> Result<()> {
    set_rules_dir(abs_rules_dir_path())?;
    set_preference("Language", "ja")?;
    set_preference("SpeechStyle", "SimpleSpeak")?;
    set_preference("Verbosity", "Medium")?;
    set_mathml("<math><mfrac><mn>3</mn><mn>4</mn></mfrac></math>")?;
    let spoken = get_overview_text()?;
    assert_eq!("4 分の 3", spoken.trim_end_matches([' ', ',', ';']));
    return Ok(());
}

/// A strike names its direction and then the mark. 対角形 ("diagonal shape") does
/// not say which diagonal, and クロスアウト is the English "cross out".
#[test]
fn menclose_strikes() -> Result<()> {
    for (notation, expected) in [
        ("updiagonalstrike", "右上がり方向, 取り消し線, 囲み x 囲み終了"),
        ("downdiagonalstrike", "右下がり方向, 取り消し線, 囲み x 囲み終了"),
    ] {
        let expr = format!("<math><menclose notation='{notation}'><mi>x</mi></menclose></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// The shapes and the long division mark were transliterations of the English.
#[test]
fn menclose_shapes_and_long_division() -> Result<()> {
    for (notation, expected) in [
        ("circle", "円, 囲み x 囲み終了"),
        ("roundedbox", "角丸の四角, 囲み x 囲み終了"),
        ("longdiv", "割り算の筆算の記号, 囲み x 囲み終了"),
    ] {
        let expr = format!("<math><menclose notation='{notation}'><mi>x</mi></menclose></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// 上下矢印 is an arrow with a head at each end, so it was the wrong name for the
/// single up arrow. The double ended arrows had the English "double ended" in them;
/// unicode-full.yaml already calls ↕ 上下矢印 and ⤢ 北東・南西矢印.
#[test]
fn menclose_arrows() -> Result<()> {
    for (notation, expected) in [
        ("uparrow", "上矢印, 囲み x 囲み終了"),
        ("updownarrow", "上下矢印, 囲み x 囲み終了"),
        ("northeastsouthwestarrow", "北東・南西矢印, 囲み x 囲み終了"),
    ] {
        let expr = format!("<math><menclose notation='{notation}'><mi>x</mi></menclose></math>");
        test("ja", "ClearSpeak", &expr, expected)?;
    }
    return Ok(());
}

/// 詳しくはこちら is the web phrase "click here for details". The word wanted here
/// is "above", the partner of the 下 in the rule next to it.
#[test]
fn something_above_and_below_an_expression() -> Result<()> {
    let over = "<math><mover><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mi>z</mi></mover></math>";
    test("ja", "ClearSpeak", over, "数量 x プラス 1 付き z 上")?;
    let both = "<math><munderover><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mi>y</mi><mi>z</mi></munderover></math>";
    test("ja", "ClearSpeak", both, "数量 x プラス 1 付き y 下および z 上")?;
    return Ok(());
}

/// ノルダム is Notre-Dame. The Japanese for a norm is ノルム.
#[test]
fn subscripted_norm() -> Result<()> {
    let expr = "<math><msub><mrow><mo>&#x2225;</mo><mi>x</mi><mo>&#x2225;</mo></mrow><mn>2</mn></msub></math>";
    test("ja", "ClearSpeak", expr, "2 ノルム の x")?;
    return Ok(());
}

/// A matrix has 成分, not エントリー.
#[test]
fn matrix_entry() -> Result<()> {
    let matrix = "<math><mrow><mo>(</mo><mtable><mtr><mtd><mi>x</mi></mtd></mtr></mtable><mo>)</mo></mrow></math>";
    test("ja", "ClearSpeak", matrix, "1 かける 1 行列 成分 x")?;
    return Ok(());
}

/// A coordinate is a 点, the word geometry.yaml already uses.
#[test]
fn coordinate_point() -> Result<()> {
    let coordinate = "<math><mrow intent='point($x,$y)'><mn arg='x'>1</mn><mo>,</mo><mn arg='y'>2</mn></mrow></math>";
    test("ja", "ClearSpeak", coordinate, "点 1 コンマ 2")?;
    return Ok(());
}
