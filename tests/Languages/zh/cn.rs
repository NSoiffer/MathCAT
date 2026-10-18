//! Core Simplified Chinese speech and navigation regression tests.

use crate::common::*;
use anyhow::Result;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[test]
fn fractions_use_denominator_first_order() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>",
        "2 分之 1",
    )?;

    let complex =
        "<math><mfrac><mi>a</mi><mrow><mi>b</mi><mo>+</mo><mn>1</mn></mrow></mfrac></math>";
    test(
        "zh",
        "SimpleSpeak",
        complex,
        "分数, b 加 1, 分之 a, 结束分数",
    )?;
    test("zh", "ClearSpeak", complex, "分数，分子为 a; 分母为 b 加 1")
}

#[test]
fn zh_cn_locale_falls_back_to_simplified_chinese_rules() -> Result<()> {
    // Regional Chinese locales without overrides must use the base zh rules.
    test(
        "zh-cn",
        "SimpleSpeak",
        "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>",
        "2 分之 1",
    )
}

#[test]
fn roots_and_powers_use_chinese_word_order() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><msqrt><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow></msqrt></math>",
        "根号 x 加 y 结束根号",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mroot><mi>x</mi><mi>n</mi></mroot></math>",
        "x 的 n 次方根",
    )?;
    test_prefs(
        "zh",
        "SimpleSpeak",
        vec![("Verbosity", "Terse")],
        "<math><mroot><mi>x</mi><mn>3</mn></mroot></math>",
        "x 的 立方根",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mroot><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mi>n</mi></mroot></math>",
        "x 加 y 的 n 次方根, 结束根号",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><msup><mi>x</mi><mi>n</mi></msup></math>",
        "x 的 n 次方",
    )?;
    test(
        "zh",
        "LiteralSpeak",
        "<math><mroot><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mi>n</mi></mroot></math>",
        "根指数 n 根号, x 加 y, 结束根号",
    )
}

#[test]
fn clearspeak_ordinal_exponents_keep_the_chinese_power_noun() -> Result<()> {
    // Chinese exponent readings use "次方" rather than an inflected ordinal form.
    test_prefs(
        "zh",
        "ClearSpeak",
        vec![("ClearSpeak_Exponents", "Ordinal")],
        "<math><msup><mi>x</mi><mn>4</mn></msup></math>",
        "x 的 4 次方",
    )
}

#[test]
fn confirmed_repairs_have_regression_coverage() -> Result<()> {
    // Keep the repaired readings consistent across both Chinese speech styles.
    let cases = [
        (
            "floor-and-ceiling",
            "<math><mrow><mo>&#x230a;</mo><mi>x</mi><mo>&#x230b;</mo></mrow><mo>+</mo><mrow><mo>&#x2308;</mo><mi>y</mi><mo>&#x2309;</mo></mrow></math>",
            "x 向下取整 加 y 向上取整",
        ),
        (
            "second-partial-derivative",
            "<math><mfrac><mrow><msup><mo>&#x2202;</mo><mn>2</mn></msup><mi>f</mi></mrow><mrow><mo>&#x2202;</mo><msup><mi>x</mi><mn>2</mn></msup></mrow></mfrac></math>",
            "f 对 x 的二阶偏导数",
        ),
        (
            "square-meter",
            "<math><mn>3</mn><msup><mi intent=':unit'>m</mi><mn>2</mn></msup></math>",
            "3 平方米",
        ),
        (
            "cubic-meter",
            "<math><mn>2</mn><msup><mi intent=':unit'>m</mi><mn>3</mn></msup></math>",
            "2 立方米",
        ),
        (
            "general-unit-power",
            "<math><msup><mi intent=':unit'>m</mi><mn>4</mn></msup></math>",
            "米的 4 次方",
        ),
        (
            "tilde-modified-variable",
            "<math><mover><mi>z</mi><mo>&#x303;</mo></mover></math>",
            "z 上加波浪线",
        ),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for (name, mathml, expected) in cases {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn absolute_values_use_argument_first_order() -> Result<()> {
    let expr =
        "<math><mrow><mo>|</mo><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mo>|</mo></mrow></math>";
    test("zh", "SimpleSpeak", expr, "x 加 1 的绝对值")?;
    test("zh", "ClearSpeak", expr, "x 加 1 的绝对值")?;
    test_prefs(
        "zh",
        "ClearSpeak",
        vec![("ClearSpeak_AbsoluteValue", "AbsEnd")],
        expr,
        "x 加 1 的绝对值, 结束绝对值",
    )?;
    test_prefs(
        "zh",
        "ClearSpeak",
        vec![("ClearSpeak_AbsoluteValue", "Cardinality")],
        "<math><mrow><mo>|</mo><mi>S</mi><mo>|</mo></mrow></math>",
        "大写 s 的基数",
    )
}

#[test]
fn literal_speech_matches_documented_examples() -> Result<()> {
    test(
        "zh",
        "LiteralSpeak",
        "<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>",
        "2 分之 1",
    )?;
    test(
        "zh",
        "LiteralSpeak",
        "<math><msqrt><mi>x</mi></msqrt></math>",
        "根号 x, 结束根号",
    )
}

#[test]
fn units_increment_and_calculus_operators_use_standard_terms() -> Result<()> {
    // Differential operators put their operand before the operator name in Chinese.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>1</mn><mi intent=':unit'>kat</mi></math>",
        "1 开特",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2206;</mo><mi>x</mi></math>",
        "增量 x",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='laplacian($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的拉普拉斯算子",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='divergence($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的散度",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='curl($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的旋度",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='gradient($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的梯度",
    )
}

#[test]
fn limit_arrows_and_variable_bars_use_mathematical_context() -> Result<()> {
    // Diagonal arrows denote one-sided limits only in a limit, and a bar over a variable is not a phonetic macron.
    test(
        "zh",
        "SimpleSpeak",
        "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2197;</mo><mn>0</mn></mrow></munder></math>",
        "极限，当 x 从下方趋于 0",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2198;</mo><mn>0</mn></mrow></munder></math>",
        "极限，当 x 从上方趋于 0",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>x</mi><mo>&#x2197;</mo><mi>y</mi><mo>,</mo><mi>x</mi><mo>&#x2198;</mo><mi>y</mi></math>",
        "x 右上箭头 y, 逗号; x 右下箭头 y",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mover><mi>x</mi><mo>&#xaf;</mo></mover></math>",
        "x 上横线",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#xaf;</mo></math>",
        "长音符",
    )
}

#[test]
fn permutation_cycles_and_repeating_decimals_use_concise_terms() -> Result<()> {
    // Use the standard nouns "轮换" and "循环节", without literal English-style expansion.
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='permutation-cycle($x)'><mi arg='x'>x</mi></mrow></math>",
        "轮换 x",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='repeating-decimal($a,$b)'><mn arg='a'>0.1</mn><mn arg='b'>6</mn></mrow></math>",
        "0.1 循环节为 6",
    )
}

#[test]
fn permutation_counts_use_mainland_textbook_word_order() -> Result<()> {
    // All supported P-notation layouts mean the number of permutations obtained by taking k from n.
    let cases = [
        "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>",
        "<math><mmultiscripts><mi>P</mi><mi>k</mi><none/><mprescripts/><none/><mi>n</mi></mmultiscripts></math>",
        "<math><msubsup><mi>P</mi><mi>k</mi><mi>n</mi></msubsup></math>",
    ];

    for mathml in cases {
        test("zh", "SimpleSpeak", mathml, "n 取 k 的排列数")?;
    }
    Ok(())
}

#[test]
fn ellipses_use_the_standard_symbol_name() -> Result<()> {
    // U+2026 is the general ellipsis; U+22EF keeps its distinct midline name.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>1</mn><mo>&#x2026;</mo><mn>3</mn><mo>,</mo><mo>&#x22ef;</mo></math>",
        "1 省略号 3, 逗号, 中线水平省略号",
    )
}

#[test]
fn chemical_equilibrium_arrows_use_standard_reaction_terms() -> Result<()> {
    // Distinguish a reversible reaction from equilibria biased to either side.
    let equation = |arrow: &str| {
        format!(
            "<math><mrow data-chem-equation='3'><mi mathvariant='normal' data-chem-element='1'>H</mi><mo data-chem-equation-op='1'>{arrow}</mo><mi mathvariant='normal' data-chem-element='1'>I</mi></mrow></math>"
        )
    };
    test(
        "zh",
        "SimpleSpeak",
        &equation("&#x21cc;"),
        "大写 h, 可逆反应 大写 i",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        &equation("&#x1f8d1;"),
        "大写 h, 可逆反应 大写 i",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        &equation("&#x1f8d3;"),
        "大写 h, 平衡偏左 大写 i",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        &equation("&#x1f8d2;"),
        "大写 h, 平衡偏右 大写 i",
    )
}

#[test]
fn chemical_quadruple_bond_uses_the_standard_bond_order_term() -> Result<()> {
    // A bond formed by four shared electron pairs is a 四重键, parallel to 单键、双键、三键.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow data-chem-formula='3'><mi mathvariant='normal' data-chem-element='1'>C</mi><mo data-chemical-bond='true' data-chem-formula-op='1'>&#x2263;</mo><mi mathvariant='normal' data-chem-element='1'>C</mi></mrow></math>",
        "大写 c, 四重键 大写 c",
    )
}

#[test]
fn function_intents_use_natural_chinese_argument_order() -> Result<()> {
    // Chinese property names follow their argument; binary relationships keep their semantic order.
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='domain($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的定义域",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='complex-conjugate($x)'><mi arg='x'>z</mi></mrow></math>",
        "z 的共轭复数",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='fractional-part($x)'><mi arg='x'>x</mi></mrow></math>",
        "x 的小数部分",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='floor($x)'><mi arg='x'>x</mi></mrow></math>",
        "x 向下取整",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='round($x)'><mi arg='x'>x</mi></mrow></math>",
        "x 四舍五入后的值",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='greatest-common-divisor($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 与 y 的最大公约数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='least-common-multiple($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 与 y 的最小公倍数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='conditional-probability($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "在 b 条件下 a 的概率",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='tends-to-from-above($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 从上方趋于 y",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='tends-to-from-below($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 从下方趋于 y",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='set-difference($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "a 与 b 的差集",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='least-common-denominator($x,$y,$z)'><mi arg='x'>x</mi><mi arg='y'>y</mi><mi arg='z'>z</mi></mrow></math>",
        "x, y 与 z 的最小公分母",
    )
}

#[test]
fn argument_owned_function_variants_keep_their_standard_terms() -> Result<()> {
    // Exercise every remaining branch of the shared argument-first function rule.
    let cases = [
        ("inverse", "f", "f 的逆"),
        ("codomain", "f", "f 的陪域"),
        ("image", "f", "f 的像"),
        ("max", "s", "s 的最大值"),
        ("min", "s", "s 的最小值"),
        ("complex-arg", "z", "z 的辐角"),
        ("real-part", "z", "z 的实部"),
        ("imaginary-part", "z", "z 的虚部"),
        ("complement", "a", "a 的补集"),
        ("cardinality", "s", "s 的基数"),
        ("probability", "a", "a 的概率"),
        ("volume", "v", "v 的体积"),
        ("chemistry-concentration", "c", "c 的浓度"),
        ("ceiling", "x", "x 向上取整"),
    ];

    for (intent, argument, expected) in cases {
        let mathml = format!(
            "<math><mrow intent='{intent}($x)'><mi arg='x'>{argument}</mi></mrow></math>"
        );
        test("zh", "ClearSpeak", &mathml, expected)
            .map_err(|error| anyhow::anyhow!("{intent}: {error}"))?;
    }
    Ok(())
}

#[test]
fn linear_algebra_intents_use_standard_noun_phrases() -> Result<()> {
    // These intents denote a property of an object or a map between spaces.
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='determinant($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的行列式",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='adjugate($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的伴随矩阵",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='magnitude($x)'><mi arg='x'>v</mi></mrow></math>",
        "v 的模",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='transpose($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的转置",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='trace($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的迹",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='dimension($x)'><mi arg='x'>v</mi></mrow></math>",
        "v 的维数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='kernel($x)'><mi arg='x'>f</mi></mrow></math>",
        "f 的核",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='span($u,$v)'><mi arg='u'>u</mi><mi arg='v'>v</mi></mrow></math>",
        "由 u 与 v 张成的空间",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='span($u)'><mi arg='u'>u</mi></mrow></math>",
        "由 u 张成的空间",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='homomorphism($m)'><mi arg='m'>m</mi></mrow></math>",
        "m 的同态",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='homomorphism($m,$n)'><mi arg='m'>m</mi><mi arg='n'>n</mi></mrow></math>",
        "m 到 n 的同态",
    )
}

#[test]
fn non_divisibility_uses_the_textbook_relation() -> Result<()> {
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='does-not-divide($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "a 不整除 b",
    )
}

#[test]
fn long_multiscripts_state_the_remaining_order_naturally() -> Result<()> {
    // Once explicit pairs are exhausted, state that the remaining lower and upper scripts alternate.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mmultiscripts><mi>T</mi><mprescripts/><mi>k</mi><mi>l</mi><mi>m</mi><mi>n</mi><mi>o</mi><mi>p</mi></mmultiscripts></math>",
        "大写 t 有 3 组前置上下标, 前下标 k 且 前上标 l, 前下标 m 且 前上标 n 其余前置下标、上标依次交替 o p 结束前置上下标",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mmultiscripts><mi>T</mi><mi>a</mi><mi>b</mi><mi>c</mi><mi>d</mi><mi>e</mi><mi>f</mi><mi>g</mi><mi>h</mi><mi>i</mi><mi>j</mi></mmultiscripts></math>",
        "大写 t 有 5 组后置上下标, 下标 a 且 上标 b 下标 c 且 上标 d 下标 e 且 上标 f 下标 g 且 上标 h 其余后置下标、上标依次交替 i j 结束后置上下标",
    )
}

#[test]
fn set_relations_use_textbook_containment_terms() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>A</mi><mo>&#x2282;</mo><mi>B</mi><mo>,</mo><mi>C</mi><mo>&#x2286;</mo><mi>D</mi><mo>,</mo><mi>E</mi><mo>&#x228a;</mo><mi>F</mi><mo>,</mo><mi>G</mi><mo>&#x2acb;</mo><mi>H</mi></math>",
        "大写 a 子集 大写 b, 逗号; 大写 c 子集或等于 大写 d; 逗号; 大写 e 真子集 大写 f; 逗号; 大写 g 真子集 大写 h",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>A</mi><mo>&#x2284;</mo><mi>B</mi><mo>,</mo><mi>C</mi><mo>&#x2288;</mo><mi>D</mi></math>",
        "大写 a 非子集 大写 b; 逗号; 大写 c 既非子集也不等于, 大写 d",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='subset($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 子集 y",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='subset-or-equal($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 子集或等于 y",
    )
}

#[test]
fn set_operations_use_standard_verb_forms() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>A</mi><mo>&#x2229;</mo><mi>B</mi><mo>&#x222a;</mo><mi>C</mi></math>",
        "大写 a 交 大写 b, 并 大写 c",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='intersection($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 交 y",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='union($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 并 y",
    )
}

#[test]
fn textbook_logic_and_geometry_symbols_use_standard_readings() -> Result<()> {
    // Prefer the relation and shape names used in mainland textbooks over visual Unicode descriptions.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2200;</mo><mi>x</mi><mo>&#x2208;</mo><mi>R</mi><mo>,</mo><mi>p</mi><mo>&#x2227;</mo><mi>q</mi><mo>&#x21d2;</mo><mi>r</mi><mo>&#x2228;</mo><mi>s</mi></math>",
        "任意 x 属于 大写 r; 逗号; p 且 q 推出 r 或 s",
    )?;
    test("zh", "SimpleSpeak", "<math><mo>&#x2205;</mo></math>", "空集")?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>x</mi><mo>&#x21a6;</mo><msup><mi>x</mi><mn>2</mn></msup><mo>,</mo><mi>A</mi><mo>&#x2216;</mo><mi>B</mi></math>",
        "x 映射到 x 平方, 逗号; 大写 a 差集 大写 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>x</mi><mo>&#x27fc;</mo><msup><mi>x</mi><mn>2</mn></msup></math>",
        "x 映射到 x 平方",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>p</mi><mo>&#x27f9;</mo><mi>q</mi></math>",
        "p 推出 q",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='similar($a,$b)'><mrow arg='a'><mo>&#x25b3;</mo><mi>A</mi><mi>B</mi><mi>C</mi></mrow><mrow arg='b'><mo>&#x25b3;</mo><mi>D</mi><mi>E</mi><mi>F</mi></mrow></mrow></math>",
        "三角形, 大写 a 大写 b 大写 c 相似于; 三角形, 大写 d 大写 e 大写 f",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x223d;</mo><mi>b</mi></math>",
        "a 反转波浪号 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x25b3;</mo><mi>A</mi><mi>B</mi><mi>C</mi><mo>&#x2245;</mo><mo>&#x25b3;</mo><mi>D</mi><mi>E</mi><mi>F</mi></math>",
        "三角形, 大写 a 大写 b 大写 c; 近似等于; 三角形, 大写 d 大写 e 大写 f",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='congruent($a,$b)'><mrow arg='a'><mo>&#x25b3;</mo><mi>A</mi><mi>B</mi><mi>C</mi></mrow><mrow arg='b'><mo>&#x25b3;</mo><mi>D</mi><mi>E</mi><mi>F</mi></mrow></mrow></math>",
        "三角形, 大写 a 大写 b 大写 c 全等于; 三角形, 大写 d 大写 e 大写 f",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2299;</mo><mi>O</mi><mo>,</mo><mo>&#x25b1;</mo><mi>A</mi><mi>B</mi><mi>C</mi><mi>D</mi></math>",
        "圆 大写 o, 逗号; 白色平行四边形; 大写 a 大写 b 大写 c 大写 d",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x2299;</mo><mi>b</mi></math>",
        "a 带圈点运算符 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x22bf;</mo><mi>A</mi><mi>B</mi><mi>C</mi></math>",
        "直角三角形, 大写 a 大写 b 大写 c",
    )
}

#[test]
fn plus_minus_symbols_follow_standard_contextual_readings() -> Result<()> {
    // GB 3102.11 distinguishes signs used alone from binary plus/minus operations.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#xb1;</mo></math>",
        "正或负",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#xb1;</mo><mi>x</mi></math>",
        "正或负 x",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>20</mn><mo>&#xb1;</mo><mn>0.5</mn></math>",
        "20 加或减 0.5",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2213;</mo></math>",
        "负或正",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x2213;</mo><mi>b</mi></math>",
        "a 减或加 b",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='plus-or-minus($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "a 加或减 b",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='minus-or-plus($a,$b)'><mi arg='a'>c</mi><mi arg='b'>d</mi></mrow></math>",
        "c 减或加 d",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x2266;</mo><mi>b</mi></math>",
        "a 小于等于 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>c</mi><mo>&#x2267;</mo><mi>d</mi></math>",
        "c 大于等于 d",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>a</mi><mo>&#x2a7d;</mo><mi>b</mi></math>",
        "a 小于或倾斜等于 b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>c</mi><mo>&#x2a7e;</mo><mi>d</mi></math>",
        "c 大于或倾斜等于 d",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mi>f</mi><mo>&#x2243;</mo><mi>g</mi></math>",
        "f 渐近等于 g",
    )
}

#[test]
fn double_factorial_distinguishes_math_and_literal_contexts() -> Result<()> {
    // U+203C is a double factorial in formulas but remains punctuation in literal content.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x203c;</mo></math>",
        "双阶乘",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent=':literal'><mo>&#x203c;</mo></mrow></math>",
        "双感叹号",
    )
}

#[test]
fn standard_number_sets_use_textbook_names() -> Result<()> {
    let cases = [
        ("set-of-integers", "ℤ", "全体整数的集合"),
        ("set-of-reals", "ℝ", "全体实数的集合"),
        ("set-of-rationals", "ℚ", "全体有理数的集合"),
        ("set-of-natural-numbers", "ℕ", "全体自然数的集合"),
        ("set-of-complex-numbers", "ℂ", "全体复数的集合"),
        ("set-of-primes", "ℙ", "全体素数的集合"),
    ];

    for (intent, symbol, expected) in cases {
        let expr = format!("<math><mi intent='{intent}'>{symbol}</mi></math>");
        test("zh", "ClearSpeak", &expr, expected)?;
    }
    Ok(())
}

#[test]
fn set_builder_description_is_natural() -> Result<()> {
    let expr = "<math><mrow><mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&gt;</mo><mn>2</mn></mrow><mo>}</mo></mrow></math>";
    test("zh", "ClearSpeak", expr, "满足 x 大于 2 的所有 x 组成的集合")?;
    test_prefs(
        "zh",
        "ClearSpeak",
        vec![("ClearSpeak_Sets", "woAll")],
        expr,
        "满足 x 大于 2 的 x 组成的集合",
    )?;
    test("zh", "SimpleSpeak", expr, "满足 x 大于 2 的所有 x 组成的集合")
}

#[test]
fn multi_argument_set_intent_is_not_mistaken_for_empty_set() -> Result<()> {
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='set($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "集合 x 逗号, y",
    )
}

#[test]
fn finite_intervals_use_mainland_textbook_readings() -> Result<()> {
    let cases = [
        (
            "open",
            "<math><mrow intent='open-interval($start,$end)'><mi arg='start'>a</mi><mi arg='end'>b</mi></mrow></math>",
            "开区间 a 到 b",
            "从 a 到 b 的开区间",
        ),
        (
            "closed",
            "<math><mrow><mo>[</mo><mrow><mi>a</mi><mo>,</mo><mi>b</mi></mrow><mo>]</mo></mrow></math>",
            "闭区间 a 到 b",
            "从 a 到 b 的闭区间",
        ),
        (
            "open-closed",
            "<math><mrow><mo>(</mo><mrow><mi>a</mi><mo>,</mo><mi>b</mi></mrow><mo>]</mo></mrow></math>",
            "左开右闭区间 a 到 b",
            "从 a 到 b 的左开右闭区间",
        ),
        (
            "closed-open",
            "<math><mrow><mo>[</mo><mrow><mi>a</mi><mo>,</mo><mi>b</mi></mrow><mo>)</mo></mrow></math>",
            "左闭右开区间 a 到 b",
            "从 a 到 b 的左闭右开区间",
        ),
        (
            "decimal-membership",
            "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>η</mi><mo>∈</mo><mrow><mo stretchy='false'>[</mo><mrow><mn>0.15</mn><mo separator='true'>,</mo><mn>0.25</mn></mrow><mo stretchy='false'>]</mo></mrow></math>",
            "eta 属于 闭区间 0.15 到 0.25",
            "eta 属于 从 0.15 到 0.25 的闭区间",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn quotient_and_remainder_follow_operand_order() -> Result<()> {
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='quotient($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 除以 y 的商",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='remainder($a,$b)'><mi arg='a'>x</mi><mi arg='b'>y</mi></mrow></math>",
        "x 除以 y 的余数",
    )
}

#[test]
fn coordinates_and_angles_use_standard_terms() -> Result<()> {
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='cartesian-coordinate($a,$b,$c)'><mi arg='a'>x</mi><mi arg='b'>y</mi><mi arg='c'>z</mi></mrow></math>",
        "直角坐标 x 逗号, y 逗号, z",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='measured-angle:prefix($x)'><mi arg='x'>x</mi></mrow></math>",
        "角 x 的度数",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='angle-measure:prefix($x)'><mi arg='x'>y</mi></mrow></math>",
        "角 y 的度数",
    )
}

#[test]
fn geometry_objects_use_textbook_word_order() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='line-segment($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "线段 a b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='ray($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "射线 a b",
    )?;
    test_prefs(
        "zh",
        "SimpleSpeak",
        vec![("Verbosity", "Verbose")],
        "<math><mrow intent='line-segment($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "线段 a b",
    )?;
    test_prefs(
        "zh",
        "SimpleSpeak",
        vec![("Verbosity", "Verbose")],
        "<math><mrow intent='ray($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "射线 a b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='arc($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
        "弧 a b",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow intent='measure-of-angle($a,$b,$c)'><mi arg='a'>a</mi><mi arg='b'>b</mi><mi arg='c'>c</mi></mrow></math>",
        "角 a b c 的度数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='length($x)'><mi arg='x'>a</mi></mrow></math>",
        "a 的长度",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow intent='area($x)'><mi arg='x'>s</mi></mrow></math>",
        "s 的面积",
    )
}

#[test]
fn definite_integral_announces_limits() -> Result<()> {
    test(
        "zh",
        "SimpleSpeak",
        "<math><msubsup><mo>&#x222b;</mo><mn>0</mn><mn>1</mn></msubsup><mi>f</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mi>d</mi><mi>x</mi></math>",
        "积分 从 0 到 1, f x d x",
    )
}

#[test]
fn matrix_announces_dimensions_and_rows() -> Result<()> {
    // Matrix row numbers follow the Chinese ordinal pattern "第 n 行".
    test(
        "zh",
        "SimpleSpeak",
        "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>)</mo></mrow></math>",
        "2 乘 2 矩阵; 第 1 行; 1, 2; 第 2 行; 3, 4",
    )
}

#[test]
fn menclose_names_the_mark_instead_of_the_result() -> Result<()> {
    // The notation describes the drawn enclosure: a radical sign or two crossing strike lines.
    test(
        "zh",
        "SimpleSpeak",
        "<math><menclose notation='radical'><mi>x</mi></menclose></math>",
        "根号, 包围 x",
    )?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><menclose notation='updiagonalstrike downdiagonalstrike'><mi>x</mi></menclose></math>",
        "交叉, 划掉, 包围 x",
    )
}

#[test]
fn tau_and_dotted_minus_symbols_are_distinguishable() -> Result<()> {
    test("zh", "SimpleSpeak", "<math><mi>&#x03c4;</mi></math>", "tau")?;
    test(
        "zh",
        "SimpleSpeak",
        "<math><mo>&#x2a2b;</mo><mo>,</mo><mo>&#x2a2c;</mo></math>",
        "带下降点列的减号, 逗号, 带上升点列的减号",
    )
}

#[test]
fn unicode_letter_currency_and_operator_names_match_character_identity() -> Result<()> {
    // These names distinguish characters that were blank or assigned to a different symbol.
    let cases = [
        ("0306", "上加短音符"),
        ("030c", "上加倒抑扬符"),
        ("0430", "西里尔字母阿"),
        ("043b", "西里尔字母埃勒"),
        ("0440", "西里尔字母埃尔"),
        ("0607", "阿拉伯-印度四次方根"),
        ("20b3", "奥斯特拉尔货币符号"),
        ("20e7", "年金符号"),
        ("2106", "每个"),
        ("210e", "普朗克常量"),
        ("210f", "约化普朗克常量"),
        ("2114", "磅符号"),
        ("2116", "序号符号"),
        ("2127", "姆欧"),
        ("2129", "倒置希腊小写字母 iota"),
        ("223c", "波浪运算符"),
        ("223d", "反转波浪号"),
        ("2240", "圈积"),
        ("2244", "不渐近等于"),
        ("2246", "近似但不等于"),
        ("2257", "圆圈等于"),
        ("225c", "delta 等于"),
        ("226d", "不等价于"),
        ("22a3", "左断言符"),
        ("22a6", "断言符"),
        ("22a8", "为真"),
        ("22b8", "多重映射"),
        ("22c6", "星号运算符"),
        ("2327", "矩形框内的 X"),
        ("2332", "锥度"),
        ("23e3", "带圆圈的苯环"),
        ("260c", "合"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn unicode_arrows_shapes_and_ornaments_name_visible_features() -> Result<()> {
    // Direction, fill, quadrant, and bracket shape must remain distinguishable in speech.
    let cases = [
        ("219e", "向左双头箭头"),
        ("21ba", "逆时针开口圆箭头"),
        ("21bb", "顺时针开口圆箭头"),
        ("21dc", "向左曲线箭头"),
        ("21dd", "向右曲线箭头"),
        ("21c4", "上方右箭头下方左箭头"),
        ("21c5", "左侧上箭头右侧下箭头"),
        ("21c6", "上方左箭头下方右箭头"),
        ("21f5", "左侧下箭头右侧上箭头"),
        ("21ea", "从横线向上的白色箭头"),
        ("25cd", "竖直线填充的圆"),
        ("25d4", "右上象限为黑色的圆"),
        ("25d5", "除左上象限外为黑色的圆"),
        ("25d9", "反白圆"),
        ("25da", "上半反白圆"),
        ("25db", "下半反白圆"),
        ("25e6", "复合"),
        ("29eb", "黑色长菱形"),
        ("2661", "空心红桃"),
        ("2665", "实心红桃"),
        ("2680", "骰子一点"),
        ("2688", "右侧带白点的实心圆"),
        ("2768", "中等左圆括号装饰符"),
        ("2774", "中等左花括号装饰符"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn cjk_brackets_and_dashes_use_standard_simplified_names() -> Result<()> {
    // Match the Unicode CLDR zh TTS names for CJK brackets and the wavy dash.
    let cases = [
        ("3008", "左尖括号"),
        ("3009", "右尖括号"),
        ("300a", "左双尖括号"),
        ("300b", "右双尖括号"),
        ("300c", "左角括号"),
        ("300d", "右角括号"),
        ("300e", "左中空角括号"),
        ("300f", "右中空角括号"),
        ("3010", "左黑色透镜状方括号"),
        ("3011", "右黑色透镜状方括号"),
        ("3016", "左中空透镜状方括号"),
        ("3017", "右中空透镜状方括号"),
        ("301c", "波浪号"),
        ("3030", "波浪型破折号"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn unicode_dash_and_quill_bracket_names_follow_mainland_usage() -> Result<()> {
    // CLDR zh uses 连接号、破折号、横条 for U+2013..U+2015. The quill
    // brackets use the same 羽毛笔 wording as U+2E20/U+2E21 above.
    let cases = [
        ("2013", "连接号"),
        ("2014", "破折号"),
        ("2015", "横条"),
        ("2045", "带羽毛笔的左方括号"),
        ("2046", "带羽毛笔的右方括号"),
    ];
    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn unicode_punctuation_and_currency_names_use_mainland_terms() -> Result<()> {
    // “数字线” is the Chinese name used for FIGURE DASH; the other names
    // avoid Traditional wording and distinguish a minus sign from a negative sign.
    let cases = [
        ("2012", "数字线"),
        ("2040", "字符连接符"),
        ("2052", "商业减号"),
        ("20b6", "图尔里弗尔"),
        ("20b8", "坚戈符号"),
    ];
    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn unicode_composite_symbols_preserve_feature_and_spatial_order() -> Result<()> {
    // Expected names follow the Unicode character identity, including which component is above.
    let cases = [
        ("2105", "转交"),
        ("231c", "左上角"),
        ("23dc", "上置圆括号"),
        ("23df", "下置花括号"),
        ("23e0", "上置六角括号"),
        ("23e1", "下置六角括号"),
        ("2681", "骰子二点"),
        ("26aa", "中等白色圆"),
        ("27c1", "内含小型白色三角形的白色三角形"),
        ("27c3", "开子集"),
        ("27c5", "左 S 形多重集定界符"),
        ("27c6", "右 S 形多重集定界符"),
        ("27ca", "带横线的竖线"),
        ("27d0", "中心带点的白色菱形"),
        ("27e0", "被横线分割的长菱形"),
        ("2772", "细左六角括号装饰符"),
        ("2773", "细右六角括号装饰符"),
        ("27ec", "左白六角括号"),
        ("27ed", "右白六角括号"),
        ("27f2", "逆时针缺口圆箭头"),
        ("27f3", "顺时针缺口圆箭头"),
        ("27f4", "带圈加号的向右箭头"),
        ("27ff", "向右长曲线箭头"),
        ("2938", "顺时针右侧弧形箭头"),
        ("2942", "短向左箭头上方的向右箭头"),
        (
            "294a",
            "倒钩向上的向左鱼叉箭头与倒钩向下的向右鱼叉箭头",
        ),
        ("2970", "圆头向右双线箭头"),
        ("2971", "向右箭头上方的等号"),
        ("2976", "向左箭头上方的小于号"),
        ("298d", "上角带短线的左方括号"),
        ("2997", "左黑六角括号"),
        ("2998", "右黑六角括号"),
        ("29a8", "开口边末端带向上偏右箭头的测量角"),
        ("29ac", "开口边末端带向右偏上箭头的测量角"),
        ("29b1", "上方带横线的空集"),
        ("29b5", "带横线的圆"),
        ("2a22", "上方带小圆圈的加号"),
        ("2a48", "并集号、横线、交集号从上到下排列"),
        ("2a81", "上方带点的小于或倾斜等号"),
        ("2a82", "上方带点的大于或倾斜等号"),
        ("2a83", "右上方带点的小于或倾斜等号"),
        ("2a84", "左上方带点的大于或倾斜等号"),
        ("2a8b", "小于号、双线等号、大于号从上到下排列"),
        ("2aa8", "曲线闭合的小于号在斜等号上方"),
        ("2aa9", "曲线闭合的大于号在斜等号上方"),
        ("2acd", "左侧开口的方框运算符"),
        ("2acf", "闭子集"),
        ("2ada", "顶部带丁字的叉形符号"),
        ("2b00", "向右上方的白色箭头"),
        ("2b1a", "点状方框"),
        ("2b27", "黑色中等长菱形"),
        ("2b39", "带箭尾和竖线的向左箭头"),
        ("3248", "黑色方块上的带圈数字十"),
        ("1f8d1", "上方为长向右鱼叉箭头，下方为长向左鱼叉箭头"),
        ("1f8d2", "上方为长向右鱼叉箭头，下方为短向左鱼叉箭头"),
        ("1f8d3", "上方为短向右鱼叉箭头，下方为长向左鱼叉箭头"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn unicode_full_relations_keep_distinct_mathematical_meanings() -> Result<()> {
    // These look similar to common relations, so retain the Unicode-defined distinction in speech.
    let cases = [
        (
            "natural-join",
            "<math><mi>R</mi><mo>&#x22c8;</mo><mi>S</mi></math>",
            "大写 r 自然连接 大写 s",
        ),
        (
            "slanted-less-or-equal",
            "<math><mi>a</mi><mo>&#x2a7d;</mo><mi>b</mi></math>",
            "a 小于或倾斜等于 b",
        ),
        (
            "slanted-greater-or-equal",
            "<math><mi>a</mi><mo>&#x2a7e;</mo><mi>b</mi></math>",
            "a 大于或倾斜等于 b",
        ),
        (
            "less-above-similar-or-equal",
            "<math><mi>a</mi><mo>&#x2a8d;</mo><mi>b</mi></math>",
            "a 小于、相似或等于 b",
        ),
        (
            "greater-above-similar-or-equal",
            "<math><mi>a</mi><mo>&#x2a8e;</mo><mi>b</mi></math>",
            "a 大于、相似或等于 b",
        ),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for (name, mathml, expected) in cases {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn unicode_full_structured_symbols_have_verified_readings() -> Result<()> {
    // Use real MathML structures where a symbol's role affects its spoken form.
    let cases = [
        (
            "U+2A00 n-ary circled dot",
            "<math><munderover><mo>&#x2a00;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>a</mi><mi>i</mi></msub></math>",
            "n 元带圈点运算符 从 i 等于 1 到 n; a 下标 i",
        ),
        (
            "U+2A01 n-ary circled plus",
            "<math><munderover><mo>&#x2a01;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>a</mi><mi>i</mi></msub></math>",
            "n 元带圈加号运算符 从 i 等于 1 到 n; a 下标 i",
        ),
        (
            "U+2A02 n-ary circled times",
            "<math><munderover><mo>&#x2a02;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>a</mi><mi>i</mi></msub></math>",
            "n 元带圈乘号运算符 从 i 等于 1 到 n; a 下标 i",
        ),
        (
            "U+2A03 n-ary union with dot",
            "<math><munderover><mo>&#x2a03;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "带点的 n 元并集运算符 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "U+2A04 n-ary union with plus",
            "<math><munderover><mo>&#x2a04;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "带加号的 n 元并集运算符 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "U+2A05 n-ary square intersection",
            "<math><munderover><mo>&#x2a05;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "n 元方交集运算符 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "U+2A06 n-ary square union",
            "<math><munderover><mo>&#x2a06;</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "n 元方并集运算符 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "U+228E multiset union",
            "<math><mi>A</mi><mo>&#x228e;</mo><mi>B</mi></math>",
            "大写 a 多重集并集 大写 b",
        ),
        (
            "U+2A0B summation with integral",
            "<math><mo>&#x2a0b;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "带积分号的求和 f x d x",
        ),
        (
            "U+2A0C quadruple integral",
            "<math><mo>&#x2a0c;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "四重积分运算符 f x d x",
        ),
        (
            "U+2A0D finite-part integral",
            "<math><mo>&#x2a0d;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "有限部积分 f x d x",
        ),
        (
            "U+2A0E integral with double stroke",
            "<math><mo>&#x2a0e;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "双线积分 f x d x",
        ),
        (
            "U+2A0F average integral",
            "<math><mo>&#x2a0f;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "带斜线的平均积分号 f x d x",
        ),
        (
            "U+2A10 circulation function",
            "<math><mo>&#x2a10;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "环流函数 f x d x",
        ),
        (
            "U+2A11 anticlockwise integration",
            "<math><mo>&#x2a11;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "逆时针积分 f x d x",
        ),
        (
            "U+2A12 rectangular path around pole",
            "<math><mo>&#x2a12;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "绕极点矩形路径线积分 f x d x",
        ),
        (
            "U+2A13 semicircular path around pole",
            "<math><mo>&#x2a13;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "绕极点半圆路径线积分 f x d x",
        ),
        (
            "U+2A14 line integration not including pole",
            "<math><mo>&#x2a14;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "不包含极点的线积分 f x d x",
        ),
        (
            "U+2A15 integral around a point",
            "<math><mo>&#x2a15;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "绕点积分运算符 f x d x",
        ),
        (
            "U+2A16 quaternion integral",
            "<math><mo>&#x2a16;</mo><mi>f</mi><mo>&#x2061;</mo><mi>x</mi><mi>d</mi><mi>x</mi></math>",
            "四元数积分运算符 f x d x",
        ),
        (
            "U+2AC3 subset or equal with dot above",
            "<math><mi>A</mi><mo>&#x2ac3;</mo><mi>B</mi></math>",
            "大写 a, 上方带点的子集或等于号, 大写 b",
        ),
        (
            "U+2AC4 superset or equal with dot above",
            "<math><mi>A</mi><mo>&#x2ac4;</mo><mi>B</mi></math>",
            "大写 a, 上方带点的超集或等于号, 大写 b",
        ),
        (
            "U+2ADB transversal intersection",
            "<math><mi>A</mi><mo>&#x2adb;</mo><mi>B</mi></math>",
            "大写 a 横截相交 大写 b",
        ),
        (
            "U+2ADC forking",
            "<math><mi>A</mi><mo>&#x2adc;</mo><mi>B</mi></math>",
            "大写 a 分叉 大写 b",
        ),
        (
            "U+2ADD nonforking",
            "<math><mi>A</mi><mo>&#x2add;</mo><mi>B</mi></math>",
            "大写 a 非分叉 大写 b",
        ),
        (
            "U+2AF9 double-line slanted less or equal",
            "<math><mi>a</mi><mo>&#x2af9;</mo><mi>b</mi></math>",
            "a 双线倾斜小于或等于 b",
        ),
        (
            "U+2AFA double-line slanted greater or equal",
            "<math><mi>a</mi><mo>&#x2afa;</mo><mi>b</mi></math>",
            "a 双线倾斜大于或等于 b",
        ),
        (
            "U+2947 right arrow through x",
            "<math><mi>A</mi><mo>&#x2947;</mo><mi>B</mi></math>",
            "大写 a 穿过叉号的向右箭头, 大写 b",
        ),
        (
            "U+2948 left-right arrow through circle",
            "<math><mi>A</mi><mo>&#x2948;</mo><mi>B</mi></math>",
            "大写 a, 穿过小圆圈的左右箭头, 大写 b",
        ),
        (
            "U+2949 upward two-headed arrow from circle",
            "<math><mo>&#x2949;</mo><mi>x</mi></math>",
            "从小圆圈出发的向上双头箭头 x",
        ),
        (
            "U+297C left fish tail",
            "<math><mi>A</mi><mo>&#x297c;</mo><mi>B</mi></math>",
            "大写 a 左鱼尾 大写 b",
        ),
        (
            "U+297D right fish tail",
            "<math><mi>A</mi><mo>&#x297d;</mo><mi>B</mi></math>",
            "大写 a 右鱼尾 大写 b",
        ),
        (
            "U+297E upward fish tail",
            "<math><mi>A</mi><mo>&#x297e;</mo><mi>B</mi></math>",
            "大写 a 上鱼尾 大写 b",
        ),
        (
            "U+297F downward fish tail",
            "<math><mi>A</mi><mo>&#x297f;</mo><mi>B</mi></math>",
            "大写 a 下鱼尾 大写 b",
        ),
        (
            "U+29C4 squared rising diagonal slash",
            "<math><mo>&#x29c4;</mo><mi>x</mi></math>",
            "方框内上升斜线 x",
        ),
        (
            "U+29CA triangle with dot above",
            "<math><mo>&#x29ca;</mo><mi>x</mi></math>",
            "上方带点的三角形 x",
        ),
        (
            "U+20E1 combining left-right arrow above",
            "<math><mover><mi>v</mi><mo>&#x20e1;</mo></mover></math>",
            "v 上加左右箭头",
        ),
        (
            "U+20DE combining enclosing square",
            "<math><mover><mi>x</mi><mo>&#x20de;</mo></mover></math>",
            "x 外加方形",
        ),
        (
            "U+20E0 combining enclosing circle backslash",
            "<math><mover><mi>x</mi><mo>&#x20e0;</mo></mover></math>",
            "x 外加圆圈反斜杠",
        ),
        (
            "U+20DC combining four dots above",
            "<math><mover><mi>x</mi><mo>&#x20dc;</mo></mover></math>",
            "x 上加四点",
        ),
        (
            "U+20E9 combining wide bridge above",
            "<math><mover><mi>x</mi><mo>&#x20e9;</mo></mover></math>",
            "x 上加宽桥形符",
        ),
        (
            "U+20EB combining long double solidus overlay",
            "<math><mover><mi>x</mi><mo>&#x20eb;</mo></mover></math>",
            "x 叠加长双斜杠",
        ),
        (
            "U+20EF combining right arrow below",
            "<math><munder><mi>v</mi><mo>&#x20ef;</mo></munder></math>",
            "v 下加向右箭头",
        ),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for (name, mathml, expected) in cases {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn unicode_script_characters_keep_standalone_and_structural_readings() -> Result<()> {
    let superscripts = [
        ("2070", "mn", "零次方", "x 的 0 次方"),
        ("2071", "mi", "i次方", "x 的 i 次方"),
        ("2074", "mn", "四次方", "x 的 4 次方"),
        ("2075", "mn", "五次方", "x 的 5 次方"),
        ("2076", "mn", "六次方", "x 的 6 次方"),
        ("2077", "mn", "七次方", "x 的 7 次方"),
        ("2078", "mn", "八次方", "x 的 8 次方"),
        ("2079", "mn", "九次方", "x 的 9 次方"),
        ("207a", "mo", "上标加号", "x 上标 +"),
        ("207b", "mo", "上标减号", "x 上标 −"),
        ("207c", "mo", "上标等号", "x 上标 ="),
        ("207d", "mo", "上标左圆括号", "x 上标 ("),
        ("207e", "mo", "上标右圆括号", "x 上标 )"),
        ("207f", "mi", "n次方", "x 的 n 次方"),
    ];
    let subscripts = [
        ("2080", "mn", "下标零", "x 下标 0"),
        ("2081", "mn", "下标一", "x 下标 1"),
        ("2082", "mn", "下标二", "x 下标 2"),
        ("2083", "mn", "下标三", "x 下标 3"),
        ("2084", "mn", "下标四", "x 下标 4"),
        ("2085", "mn", "下标五", "x 下标 5"),
        ("2086", "mn", "下标六", "x 下标 6"),
        ("2087", "mn", "下标七", "x 下标 7"),
        ("2088", "mn", "下标八", "x 下标 8"),
        ("2089", "mn", "下标九", "x 下标 9"),
        ("208a", "mo", "下标加号", "x 下标 + 结束下标"),
        ("208b", "mo", "下标减号", "x 下标 − 结束下标"),
        ("208c", "mo", "下标等号", "x 下标 = 结束下标"),
        ("208d", "mo", "下标左圆括号", "x 下标 ( 结束下标"),
        ("208e", "mo", "下标右圆括号", "x 下标 ) 结束下标"),
        ("2090", "mi", "下标a", "x 下标 a"),
        ("2091", "mi", "下标e", "x 下标 e"),
        ("2092", "mi", "下标o", "x 下标 o"),
        ("2093", "mi", "下标x", "x 下标 x"),
        ("2095", "mi", "下标h", "x 下标 h"),
        ("2096", "mi", "下标k", "x 下标 k"),
        ("2097", "mi", "下标l", "x 下标 l"),
        ("2098", "mi", "下标m", "x 下标 m"),
        ("2099", "mi", "下标n", "x 下标 n"),
        ("209a", "mi", "下标p", "x 下标 p"),
        ("209b", "mi", "下标s", "x 下标 s"),
        ("209c", "mi", "下标t", "x 下标 t"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, tag, standalone, structured) in &superscripts {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, standalone)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/alone: {error}"))?;
            let expr =
                format!("<math><msup><mi>x</mi><{tag}>&#x{codepoint};</{tag}></msup></math>");
            test("zh", style, &expr, structured)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/msup: {error}"))?;
        }
        for &(codepoint, tag, standalone, structured) in &subscripts {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, standalone)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/alone: {error}"))?;
            let expr =
                format!("<math><msub><mi>x</mi><{tag}>&#x{codepoint};</{tag}></msub></math>");
            test("zh", style, &expr, structured)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/msub: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn audited_unicode_gap_symbols_have_direct_and_contextual_coverage() -> Result<()> {
    // TINY and MINY are proper operator names, not translations of the size adjectives.
    let direct = [
        ("20e2", "外加屏幕"),
        ("220b", "包含"),
        ("220c", "不包含"),
        ("220d", "小型包含"),
        ("2256", "环等于"),
        ("228b", "真超集"),
        ("228d", "多重集乘法"),
        ("22e4", "方形像或不等于"),
        ("22e5", "方形原像或不等于"),
        ("22ee", "垂直省略号"),
        ("22f0", "右上对角线省略号"),
        ("22f1", "右下对角线省略号"),
        ("2a07", "双逻辑与运算符"),
        ("2a08", "双逻辑或运算符"),
        ("2a87", "小于且单线不等于"),
        ("2a88", "大于且单线不等于"),
        ("2a89", "小于且不约等于"),
        ("2a8a", "大于且不约等于"),
        ("2acc", "真超集"),
        ("2298", "带圆圈除号斜线"),
        ("22d4", "真相交"),
        ("29fe", "Tiny 算子"),
        ("29ff", "Miny 算子"),
        ("2ae1", "带 S 的垂直符号"),
    ];
    let contextual = [
        ("220b", "x", "大写 a 包含 x"),
        ("220c", "x", "大写 a 不包含 x"),
        ("220d", "x", "大写 a 小型包含 x"),
        ("2298", "B", "大写 a 带圆圈除号斜线, 大写 b"),
        ("22d4", "B", "大写 a 真相交 大写 b"),
        ("2a87", "b", "大写 a 小于且单线不等于 b"),
        ("2a88", "b", "大写 a 大于且单线不等于 b"),
        ("2ae1", "B", "大写 a 带 S 的垂直符号, 大写 b"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, expected) in &direct {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/direct: {error}"))?;
        }
        for &(codepoint, rhs, expected) in &contextual {
            let expr = format!(
                "<math><mi>A</mi><mo>&#x{codepoint};</mo><mi>{rhs}</mi></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/context: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn remaining_unicode_phase_symbols_have_verified_readings() -> Result<()> {
    // Unicode NamesList supplies the identities; the structured cases verify placement too.
    let combining = [
        ("20d0", "mover", "v", "v 上加左鱼叉箭头"),
        ("20d1", "mover", "v", "v 上加右鱼叉箭头"),
        ("20d2", "mover", "x", "x 叠加长竖直线"),
        ("20d3", "mover", "x", "x 叠加短竖直线"),
        ("20d4", "mover", "x", "x 上加逆时针箭头"),
        ("20d5", "mover", "x", "x 上加顺时针箭头"),
        ("20d6", "mover", "v", "v 上加向左箭头"),
        ("20d7", "mover", "v", "v 上加向右箭头"),
        ("20d8", "mover", "x", "x 叠加圆环"),
        ("20d9", "mover", "x", "x 叠加顺时针圆环"),
        ("20da", "mover", "x", "x 叠加逆时针圆环"),
        ("20db", "mover", "x", "x 上加三点"),
        ("20dd", "mover", "x", "x 外加圆圈"),
        ("20df", "mover", "x", "x 外加菱形"),
        ("20e3", "mover", "x", "x 外加键帽"),
        ("20e4", "mover", "x", "x 外加向上三角形"),
        ("20e5", "mover", "x", "x 叠加反斜杠"),
        ("20e6", "mover", "x", "x 叠加双竖线"),
        ("20e8", "munder", "x", "x 下加三点"),
        ("20ea", "mover", "x", "x 叠加向左箭头"),
        (
            "20ec",
            "munder",
            "v",
            "v 下加带向下倒钩的向右鱼叉箭头",
        ),
        (
            "20ed",
            "munder",
            "v",
            "v 下加带向下倒钩的向左鱼叉箭头",
        ),
        ("20ee", "munder", "v", "v 下加向左箭头"),
        ("20f0", "mover", "x", "x 上加星号"),
    ];
    let relations = [
        ("22d0", "a 双子集 b"),
        ("22d1", "a 双超集 b"),
        ("22d2", "a 双交集 b"),
        ("22d3", "a 双并集 b"),
        ("22d5", "a 等于且平行于 b"),
        ("22d6", "a 带点小于 b"),
        ("22d7", "a 带点大于 b"),
        // Keep these distinct from U+226A/U+226B ("远小于/远大于").
        ("22d8", "a 极小于 b"),
        ("22d9", "a 极大于 b"),
        ("22da", "a 小于、等于或大于 b"),
        ("22db", "a 大于、等于或小于 b"),
        ("22dd", "a 等于或大于 b"),
        ("22de", "a 等于或先于 b"),
        ("22df", "a 等于或后于 b"),
    ];
    let phase_relations = [
        ("22e0", "a 既不先于也不等于 b"),
        ("22e1", "a 既不后于也不等于 b"),
        ("22e2", "a 既非方形像也不等于 b"),
        ("22e3", "a 既非方形原像也不等于 b"),
        ("22e4", "a 方形像或不等于 b"),
        ("22e5", "a 方形原像或不等于 b"),
        ("22e6", "a 小于但不等价于 b"),
        ("22e7", "a 大于但不等价于 b"),
        ("22e8", "a 先于但不等价于 b"),
        ("22e9", "a 后于但不等价于 b"),
        ("22ea", "a 不是正规子群 b"),
        ("22eb", "a 不包含正规子群 b"),
        ("22ec", "a 既不是正规子群也不等于 b"),
        ("22ed", "a 既不包含正规子群也不等于 b"),
    ];
    let ellipses = [
        ("22ee", "垂直省略号"),
        ("22ef", "中线水平省略号"),
        ("22f0", "右上对角线省略号"),
        ("22f1", "右下对角线省略号"),
    ];
    let set_relations = [
        ("22f2", "x 带长横线的属于号 a"),
        ("22f3", "x 横线末端带竖线的属于号 a"),
        ("22f4", "x 横线末端带竖线的较小属于号 a"),
        ("22f5", "x 上方带点的属于号 a"),
        ("22f6", "x 上方带横线的属于号 a"),
        ("22f7", "x 上方带横线的较小属于号 a"),
        ("22f8", "x 下方带横线的属于号 a"),
        ("22f9", "x 带两条横线的属于号 a"),
        ("22fa", "a 带长横线的包含号 x"),
        ("22fb", "a 横线末端带竖线的包含号 x"),
        ("22fc", "a 横线末端带竖线的较小包含号 x"),
        ("22fd", "a 上方带横线的包含号 x"),
        ("22fe", "a 上方带横线的较小包含号 x"),
        ("22ff", "x Z 记号多重集隶属符 a"),
    ];
    let integrals = [
        ("2a0a", "模二和 f"),
        ("2a17", "带钩向左箭头的积分号 f"),
        ("2a18", "带乘号的积分号 f"),
        ("2a19", "带交集号的积分号 f"),
        ("2a1a", "带并集号的积分号 f"),
        ("2a1b", "带上横线的积分号 f"),
        ("2a1c", "带下横线的积分号 f"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, tag, base, expected) in &combining {
            let expr = format!(
                "<math><{tag}><mi>{base}</mi><mo>&#x{codepoint};</mo></{tag}></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/structured: {error}"))?;
        }
        for &(codepoint, expected) in &relations {
            let expr = format!(
                "<math><mi>a</mi><mo>&#x{codepoint};</mo><mi>b</mi></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/relation: {error}"))?;
        }
        for &(codepoint, expected) in &phase_relations {
            let expr = format!(
                "<math><mi>a</mi><mo>&#x{codepoint};</mo><mi>b</mi></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/phase-relation: {error}"))?;
        }
        for &(codepoint, expected) in &ellipses {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/ellipsis: {error}"))?;
        }
        for &(codepoint, expected) in &set_relations {
            let (left, right) = if matches!(codepoint, "22fa" | "22fb" | "22fc" | "22fd" | "22fe") {
                ("a", "x")
            } else {
                ("x", "a")
            };
            let expr = format!(
                "<math><mi>{left}</mi><mo>&#x{codepoint};</mo><mi>{right}</mi></math>"
            );
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/set-relation: {error}"))?;
        }
        for &(codepoint, expected) in &integrals {
            let expr = format!("<math><mo>&#x{codepoint};</mo><mi>f</mi></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}/integral: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn miscellaneous_technical_symbols_have_verified_readings() -> Result<()> {
    // Cover every U+2300-U+23E7 entry not already exercised by another zh test.
    let cases = [
        ("2300", "直径"),
        ("2301", "电箭头"),
        ("2302", "房屋"),
        ("2304", "向下箭头尖"),
        ("2305", "射影"),
        ("2306", "透视"),
        ("2307", "波浪线"),
        ("230c", "右下裁切符"),
        ("230d", "左下裁切符"),
        ("230e", "右上裁切符"),
        ("230f", "左上裁切符"),
        ("2310", "反向非"),
        ("2311", "方菱形"),
        ("2312", "弧"),
        ("2313", "弓形"),
        ("2314", "扇形"),
        ("2315", "电话记录器符号"),
        ("2316", "位置指示十字线"),
        ("2317", "视图数据方框"),
        ("2318", "命令键"),
        ("2319", "倒置非"),
        ("231a", "手表"),
        ("231b", "沙漏"),
        ("231d", "右上角"),
        ("231e", "左下角"),
        ("231f", "右下角"),
        ("2320", "积分号上半部"),
        ("2321", "积分号下半部"),
        ("2322", "皱眉"),
        ("2323", "微笑"),
        ("2324", "回车键"),
        ("2325", "选项键"),
        ("2326", "向前删除键"),
        ("2328", "键盘"),
        ("2329", "左尖括号"),
        ("232a", "右尖括号"),
        ("232b", "退格键"),
        ("232c", "苯环"),
        ("232d", "圆柱度"),
        ("232e", "全周轮廓符号"),
        ("232f", "对称度"),
        ("2330", "全跳动"),
        ("2331", "尺寸原点"),
        ("2334", "沉孔"),
        ("2335", "沉头孔"),
        ("2336", "APL 功能符号工字梁"),
        ("233d", "APL 功能符号圆竖线"),
        ("233f", "APL 功能符号斜杠横线"),
        ("2370", "APL 功能符号方框问号"),
        ("237c", "带向下之字形箭头的直角"),
        ("2394", "六边形"),
        ("2395", "APL 功能符号方框"),
        ("23b4", "上置方括号"),
        ("23b5", "下置方括号"),
        ("23b6", "上方为下置方括号，下方为上置方括号"),
        ("23dd", "下置圆括号"),
        ("23de", "上置花括号"),
        ("23e2", "白色梯形"),
        ("23e4", "直线度"),
        ("23e5", "平面度"),
        ("23e7", "电路交叉点"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, expected) in &cases {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn ceiling_and_floor_glyphs_keep_side_and_rounding_direction() -> Result<()> {
    // U+2308..U+230B are the four individual left/right ceiling/floor glyphs.
    // Keep the side words in standalone speech; paired delimiters are tested
    // separately as the mathematical functions "向上取整" and "向下取整".
    let cases = [
        ("2308", "左上取整符号"),
        ("2309", "右上取整符号"),
        ("230a", "左下取整符号"),
        ("230b", "右下取整符号"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, expected) in &cases {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn circled_numbers_keep_enclosure_and_value() -> Result<()> {
    // U+2460..U+2468 are CIRCLED DIGIT ONE..NINE and use the shared
    // "带圈" prefix plus the digit value. U+2469..U+2471 have explicit names.
    let cases = [
        ("2460", "带圈 1"),
        ("2468", "带圈 9"),
        ("2469", "带圈数字十"),
        ("246a", "带圈数字十一"),
        ("246b", "带圈数字十二"),
        ("246c", "带圈数字十三"),
        ("246d", "带圈数字十四"),
        ("246e", "带圈数字十五"),
        ("246f", "带圈数字十六"),
        ("2470", "带圈数字十七"),
        ("2471", "带圈数字十八"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, expected) in &cases {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn parenthesized_digits_keep_enclosure_and_value() -> Result<()> {
    // U+2473 is CIRCLED NUMBER TWENTY. U+2474..U+247C are
    // PARENTHESIZED DIGIT ONE..NINE and use the shared "带括号" prefix.
    let cases = [
        ("2473", "带圈数字二十"),
        ("2474", "带括号 1"),
        ("247c", "带括号 9"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, expected) in &cases {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn enclosed_numbers_and_letters_keep_their_punctuation_and_value() -> Result<()> {
    // U+247D..U+2487 are parenthesized numbers ten through twenty.  U+2488..U+2490
    // are digits followed by a full stop, and U+2491..U+249B are the corresponding
    // numbers ten through twenty.  U+249C..U+24B5 are parenthesized Latin small
    // letters.  Check every explicit rule and both ends of each range rule.
    let cases = [
        ("247d", "带括号数字十"),
        ("247e", "带括号数字十一"),
        ("247f", "带括号数字十二"),
        ("2480", "带括号数字十三"),
        ("2481", "带括号数字十四"),
        ("2482", "带括号数字十五"),
        ("2483", "带括号数字十六"),
        ("2484", "带括号数字十七"),
        ("2485", "带括号数字十八"),
        ("2486", "带括号数字十九"),
        ("2487", "带括号数字二十"),
        ("2488", "带句点数字 1"),
        ("2490", "带句点数字 9"),
        ("2491", "带句点数字十"),
        ("2492", "带句点数字十一"),
        ("2493", "带句点数字十二"),
        ("2494", "带句点数字十三"),
        ("2495", "带句点数字十四"),
        ("2496", "带句点数字十五"),
        ("2497", "带句点数字十六"),
        ("2498", "带句点数字十七"),
        ("2499", "带句点数字十八"),
        ("249a", "带句点数字十九"),
        ("249b", "带句点数字二十"),
        ("249c", "带括号 a"),
        ("24b5", "带括号 z"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for &(codepoint, expected) in &cases {
            let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn remaining_p2460_symbol_families_keep_verified_readings() -> Result<()> {
    // Representative cases cover every family in the 275-entry audit, with
    // both directions of paired symbols and every higher-risk mathematical symbol.
    let cases = [
        ("24ea", "带圈数字零"),
        ("24eb", "带圈反白数字十一"),
        ("24f4", "带圈反白数字二十"),
        ("24ff", "带圈反白数字零"),
        ("25a0", "黑色方块"),
        ("25a6", "横竖交叉线填充方块"),
        ("25a7", "左上至右下斜线填充方块"),
        ("25a8", "右上至左下斜线填充方块"),
        ("25b3", "三角形"),
        ("25c9", "鱼眼"),
        ("25ca", "长菱形"),
        ("25cc", "点状圆圈"),
        ("25ce", "靶心"),
        ("25d0", "左半部为黑色的圆"),
        ("25d3", "上半部为黑色的圆"),
        ("25d8", "反白圆点"),
        ("25dc", "左上四分之一圆弧"),
        ("25df", "左下四分之一圆弧"),
        ("25e7", "左半部为黑色的方块"),
        ("25e9", "左上对角半部为黑色的方块"),
        ("25ef", "大空心圆"),
        ("25f0", "带左上象限的白色方块"),
        ("25f7", "带右上象限的白色圆"),
        ("25ff", "右下三角形"),
        ("2609", "太阳符号"),
        ("263d", "上弦月"),
        ("263e", "下弦月"),
        ("2662", "空心方块"),
        ("2666", "实心方块"),
        ("266e", "还原号"),
        ("2686", "右侧带点的空心圆"),
        ("2689", "带两个白点的实心圆"),
        ("26b2", "无性别符号"),
        ("272a", "带圈白星"),
        ("2769", "中等右圆括号装饰符"),
        ("276a", "中等扁平左圆括号装饰符"),
        ("276b", "中等扁平右圆括号装饰符"),
        ("276e", "粗左角引号装饰符"),
        ("276f", "粗右角引号装饰符"),
        ("2770", "粗左角括号装饰符"),
        ("2771", "粗右角括号装饰符"),
        ("2776", "带圈反白数字一"),
        ("277f", "带圈反白数字十"),
        ("2780", "无衬线带圈数字一"),
        ("2789", "无衬线带圈数字十"),
        ("278a", "无衬线带圈反白数字一"),
        ("2793", "无衬线带圈反白数字十"),
        ("2795", "粗加号"),
        ("2797", "粗除号"),
        ("279b", "制图笔尖向右箭头"),
        ("279f", "虚线三角头向右箭头"),
        ("27a0", "粗虚线三角头向右箭头"),
        ("27a3", "底部高亮的立体向右箭头尖"),
        ("27a5", "粗黑色向下再向右弯箭头"),
        ("27a6", "粗黑色向上再向右弯箭头"),
        ("27a9", "右侧带阴影的白色向右箭头"),
        ("27aa", "左侧带阴影的白色向右箭头"),
        ("27af", "带凹口且右下有阴影的白色向右箭头"),
        ("27b1", "带凹口且右上有阴影的白色向右箭头"),
        ("27b2", "带圆圈的粗白色向右箭头"),
        ("27b3", "白色羽状向右箭头"),
        ("27b5", "黑色羽状向右箭头"),
        ("27b6", "黑色羽状向右上箭头"),
        ("27b7", "粗黑色羽状向右下箭头"),
        ("27b9", "粗黑色羽状向右上箭头"),
        ("27ba", "泪滴形倒钩向右箭头"),
        ("27bb", "粗泪滴形箭杆向右箭头"),
        ("27bd", "粗楔尾向右箭头"),
        ("27be", "轮廓开口的向右箭头"),
        ("27c0", "三维角"),
        ("27c2", "垂直于"),
        ("27c4", "开超集"),
        ("27c7", "内含点的逻辑或"),
        ("27c8", "前置反斜杠的子集号"),
        ("27c9", "后置斜杠的超集号"),
        ("27cb", "数学上升对角线"),
        ("27cc", "长除号"),
        ("27cd", "数学下降对角线"),
        ("27ce", "带方框的逻辑与"),
        ("27cf", "带方框的逻辑或"),
        ("27d1", "带点的逻辑与"),
        ("27d2", "开口向上的属于号"),
        ("27d3", "带点的右下角"),
        ("27d4", "带点的左上角"),
        ("27d5", "左外连接"),
        ("27d6", "右外连接"),
        ("27d7", "全外连接"),
        ("27d8", "大上丁字"),
        ("27d9", "大下丁字"),
        ("27da", "左右双断言符"),
        ("27db", "左右丁字"),
        ("27dc", "左多重映射"),
        ("27dd", "长右丁字"),
        ("27de", "长左丁字"),
        ("27df", "上方带圆圈的上丁字"),
        ("27e1", "白色凹边菱形"),
        ("27e2", "带左短线的白色凹边菱形"),
        ("27e3", "带右短线的白色凹边菱形"),
        ("27e4", "带左短线的白色方块"),
        ("27e5", "带右短线的白色方块"),
        ("27e6", "数学左白方括号"),
        ("27e7", "数学右白方括号"),
        ("27e8", "数学左角括号"),
        ("27e9", "数学右角括号"),
        ("27ea", "数学左双角括号"),
        ("27eb", "数学右双角括号"),
        ("27ee", "左扁平圆括号"),
        ("27ef", "右扁平圆括号"),
        ("27f0", "向上四重箭头"),
        ("27f1", "向下四重箭头"),
        ("27f5", "长向左箭头"),
        ("27f6", "长向右箭头"),
        ("27f7", "长左右箭头"),
        ("27f8", "长向左双线箭头"),
        ("27f9", "推出"),
        ("27fa", "当且仅当"),
        ("27fb", "从竖线出发的长向左箭头"),
        ("27fc", "映射到"),
        ("27fd", "从竖线出发的长向左双线箭头"),
        ("27fe", "从竖线出发的长向右双线箭头"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn remaining_p2a00_symbols_keep_verified_readings() -> Result<()> {
    // Cover every audited P-2A00 symbol that did not already have a direct
    // codepoint test. Paired symbols are included in both directions.
    let cases = [
        ("2a1d", "连接"),
        ("2a1e", "大左三角运算符"),
        ("2a1f", "Z 记号模式组合"),
        ("2a20", "Z 记号模式管道"),
        ("2a21", "Z 记号模式投影"),
        ("2a23", "上方带抑扬符的加号"),
        ("2a24", "上方带波浪线的加号"),
        ("2a25", "下方带点的加号"),
        ("2a26", "下方带波浪线的加号"),
        ("2a27", "带下标二的加号"),
        ("2a28", "带黑色三角形的加号"),
        ("2a29", "上方带逗号的减号"),
        ("2a2a", "下方带点的减号"),
        ("2a2d", "左半圆内的加号"),
        ("2a2e", "右半圆内的加号"),
        ("2a2f", "向量叉乘"),
        ("2a30", "上方带点的乘号"),
        ("2a31", "下方带横线的乘号"),
        ("2a32", "底部闭合的半直积"),
        ("2a34", "左半圆内的乘号"),
        ("2a35", "右半圆内的乘号"),
        ("2a36", "上方带抑扬符的圈乘号"),
        ("2a37", "双圆圈内的乘号"),
        ("2a38", "带圈除号"),
        ("2a39", "三角形内的加号"),
        ("2a3a", "三角形内的减号"),
        ("2a3b", "三角形内的乘号"),
        ("2a3e", "Z 记号关系复合"),
        ("2a3f", "合并积或余积"),
        ("2a40", "带点的交集号"),
        ("2a41", "带减号的并集号"),
        ("2a42", "上方带横线的并集号"),
        ("2a43", "上方带横线的交集号"),
        ("2a44", "交集号与逻辑与号组合"),
        ("2a45", "并集号与逻辑或号组合"),
        ("2a46", "并集号在交集号上方"),
        ("2a47", "交集号在并集号上方"),
        ("2a49", "交集号、横线、并集号从上到下排列"),
        ("2a4a", "两个并列相连的并集号"),
        ("2a4b", "两个并列相连的交集号"),
        ("2a4c", "带衬线的闭合并集"),
        ("2a4d", "带衬线的闭合交集"),
        ("2a4e", "双重方形交集"),
        ("2a4f", "双重方形并集"),
        ("2a51", "上加点的逻辑与"),
        ("2a52", "上加点的逻辑或"),
        ("2a53", "双逻辑与"),
        ("2a54", "双逻辑或"),
        ("2a55", "两个相交的逻辑与"),
        ("2a56", "两个相交的逻辑或"),
        ("2a57", "倾斜大型逻辑或"),
        ("2a58", "倾斜大型逻辑与"),
        ("2a59", "逻辑或与逻辑与叠加"),
        ("2a5a", "带中间竖线的逻辑与"),
        ("2a5b", "带中间竖线的逻辑或"),
        ("2a5c", "带横线的逻辑与"),
        ("2a5d", "带横线的逻辑或"),
        ("2a5e", "带双上横线的逻辑与"),
        ("2a5f", "带下横线的逻辑与"),
        ("2a60", "带双下横线的逻辑与"),
        ("2a61", "带下横线的小逻辑或"),
        ("2a62", "带双上横线的逻辑或"),
        ("2a63", "带双下横线的逻辑或"),
        ("2a64", "Z 记号定义域反限制"),
        ("2a65", "Z 记号值域反限制"),
        ("2a66", "下方带点的等号"),
        ("2a67", "上方带点的恒等号"),
        ("2a68", "带两条竖线的三重横线"),
        ("2a69", "带三条竖线的三重横线"),
        ("2a6a", "上方带点的波浪运算符"),
        ("2a6b", "带上升点列的波浪运算符"),
        ("2a6c", "相似号、减号、相似号"),
        ("2a6d", "上方带点的全等号"),
        ("2a6e", "带星号的等号"),
        ("2a6f", "上方带抑扬符的约等号"),
        ("2a70", "约等于或等于"),
        ("2a71", "等号在加号上方"),
        ("2a72", "加号在等号上方"),
        ("2a73", "等号在波浪运算符上方"),
        ("2a74", "双冒号等号"),
        ("2a75", "连续两个等号"),
        ("2a76", "连续三个等号"),
        ("2a77", "上下各带两点的等号"),
        ("2a78", "上方带四点的等价号"),
        ("2a79", "内含圆圈的小于号"),
        ("2a7a", "内含圆圈的大于号"),
        ("2a7b", "上方带问号的小于号"),
        ("2a7c", "上方带问号的大于号"),
        ("2a7f", "内含点的小于或斜等号"),
        ("2a80", "内含点的大于或斜等号"),
        ("2a8c", "大于号、双线等号、小于号从上到下排列"),
        ("2a8f", "小于号、相似号、大于号从上到下排列"),
        ("2a90", "大于号、相似号、小于号从上到下排列"),
        ("2a91", "小于号、大于号、双线等号从上到下排列"),
        ("2a92", "大于号、小于号、双线等号从上到下排列"),
        ("2a93", "小于号、倾斜等号、大于号、倾斜等号从上到下排列"),
        ("2a94", "大于号、倾斜等号、小于号、倾斜等号从上到下排列"),
        ("2a95", "倾斜等号或小于"),
        ("2a96", "倾斜等号或大于"),
        ("2a97", "内含点的斜等号或小于号"),
        ("2a98", "内含点的斜等号或大于号"),
        ("2a99", "双线等号或小于"),
        ("2a9a", "双线等号或大于"),
        ("2a9b", "双线倾斜等号或小于"),
        ("2a9c", "双线倾斜等号或大于"),
        ("2a9d", "相似或小于"),
        ("2a9e", "相似或大于"),
        ("2a9f", "相似号、小于号与等号从上到下排列"),
        ("2aa0", "相似号、大于号与等号从上到下排列"),
        ("2aa1", "双嵌套小于"),
        ("2aa2", "双嵌套大于"),
        ("2aa3", "带下划线的双嵌套小于"),
        ("2aa4", "大于号与小于号叠加"),
        ("2aa5", "并列的大于号与小于号"),
        ("2aa6", "曲线闭合的小于号"),
        ("2aa7", "曲线闭合的大于号"),
        ("2aaa", "小于"),
        ("2aab", "大于"),
        ("2aac", "小于或等于"),
        ("2aad", "大于或等于"),
        ("2aae", "上方带波形线的等号"),
        ("2aaf", "先于号在单线等号上方"),
        ("2ab0", "后于号在单线等号上方"),
        ("2ab1", "先于号在单线不等号上方"),
        ("2ab2", "后于号在单线不等号上方"),
        ("2ab3", "先于号在等号上方"),
        ("2ab4", "后于号在等号上方"),
        ("2ab5", "先于号在不等号上方"),
        ("2ab6", "后于号在不等号上方"),
        ("2ab7", "先于号在约等号上方"),
        ("2ab8", "后于号在约等号上方"),
        ("2ab9", "先于号在不约等号上方"),
        ("2aba", "后于号在不约等号上方"),
        ("2abb", "双先于"),
        ("2abc", "双后于"),
        ("2abf", "下方带加号的子集符号"),
        ("2ac0", "下方带加号的超集符号"),
        ("2ac1", "下方带乘号的子集符号"),
        ("2ac2", "下方带乘号的超集符号"),
        ("2ac5", "子集符号在等号上方"),
        ("2ac6", "超集符号在等号上方"),
        ("2ac7", "子集符号在波浪运算符上方"),
        ("2ac8", "超集符号在波浪运算符上方"),
        ("2ac9", "子集符号在约等号上方"),
        ("2aca", "超集符号在约等号上方"),
        ("2ace", "右侧开口的方框运算符"),
        ("2ad0", "闭超集"),
        ("2ad1", "闭子集或等于"),
        ("2ad2", "闭超集或等于"),
        ("2ad7", "超集号与子集号并列"),
        ("2ad8", "超集号与子集号并列并以短横线连接"),
        ("2ad9", "开口向下的属于号"),
        ("2ade", "短左丁字"),
        ("2adf", "短下丁字"),
        ("2ae0", "短上丁字"),
        ("2ae2", "竖直线三重右断言符"),
        ("2ae3", "双竖直线左断言符"),
        ("2ae4", "竖直线双左断言符"),
        ("2ae5", "双竖直线双左断言符"),
        ("2ae6", "从双竖线左线伸出的长横线"),
        ("2ae7", "带上横线的短下丁字"),
        ("2ae8", "带下横线的短上丁字"),
        ("2ae9", "短上丁字在短下丁字上方"),
        ("2aea", "双下丁字"),
        ("2aeb", "双上丁字"),
        ("2aec", "双线非号"),
        ("2aed", "反向双线非号"),
        ("2aee", "带反向否定斜线的不整除"),
        ("2aef", "上方带圆圈的竖线"),
        ("2af0", "下方带圆圈的竖线"),
        ("2af1", "下方带圆圈的下丁字"),
        ("2af2", "带横线的平行号"),
        ("2af3", "带波浪线的平行号"),
        ("2af4", "三竖线二元关系符"),
        ("2af5", "带横线的三重竖线"),
        ("2af6", "三重冒号运算符"),
        ("2af7", "三重嵌套小于"),
        ("2af8", "三重嵌套大于"),
        ("2afb", "三重斜杠关系"),
        ("2afc", "大三重竖直线运算符"),
        ("2afd", "双斜杠运算符"),
        ("2afe", "白色竖直线"),
        ("2aff", "n 元白色竖线运算符"),
        ("2b01", "向左上方的白色箭头"),
        ("2b02", "向右下方的白色箭头"),
        ("2b03", "向左下方的白色箭头"),
        ("2b04", "白色左右双向箭头"),
        ("2b05", "向左的黑色箭头"),
        ("2b06", "向上的黑色箭头"),
        ("2b07", "向下的黑色箭头"),
        ("2b08", "向右上方的黑色箭头"),
        ("2b09", "向左上方的黑色箭头"),
        ("2b0a", "向右下方的黑色箭头"),
        ("2b0b", "向左下方的黑色箭头"),
        ("2b0c", "黑色左右双向箭头"),
        ("2b0d", "黑色上下双向箭头"),
        ("2b0e", "箭头尖向下的向右箭头"),
        ("2b0f", "箭头尖向上的向右箭头"),
        ("2b10", "箭头尖向下的向左箭头"),
        ("2b11", "箭头尖向上的向左箭头"),
        ("2b12", "上半部为黑色的方块"),
        ("2b13", "下半部为黑色的方块"),
        ("2b14", "右上对角半部为黑色的方块"),
        ("2b15", "左下对角半部为黑色的方块"),
        ("2b16", "左半部为黑色的菱形"),
        ("2b17", "右半部为黑色的菱形"),
        ("2b18", "上半部为黑色的菱形"),
        ("2b19", "下半部为黑色的菱形"),
        ("2b1b", "黑色大方块"),
        ("2b1c", "白色大方块"),
        ("2b1e", "白色极小方块"),
        ("2b1f", "黑色五边形"),
        ("2b20", "白色五边形"),
        ("2b21", "白色六边形"),
        ("2b22", "黑色六边形"),
        ("2b23", "横向黑色六边形"),
        ("2b24", "黑色大圆"),
        ("2b25", "黑色中等菱形"),
        ("2b26", "白色中等菱形"),
        ("2b28", "白色中等长菱形"),
        ("2b29", "黑色小菱形"),
        ("2b2b", "白色小长菱形"),
        ("2b2d", "白色横向椭圆"),
        ("2b2e", "黑色纵向椭圆"),
        ("2b2f", "白色纵向椭圆"),
        ("2b31", "三个向左箭头"),
        ("2b32", "带圈加号的向左箭头"),
        ("2b34", "带竖线的向左双头箭头"),
        ("2b35", "带双竖线的向左双头箭头"),
        ("2b36", "从竖线出发的向左双头箭头"),
        ("2b37", "箭杆为三段短线的向左双头箭头"),
        ("2b38", "点状箭杆的向左箭头"),
        ("2b3a", "带箭尾和双竖线的向左箭头"),
        ("2b3b", "带箭尾的向左双头箭头"),
        ("2b3c", "带箭尾和竖线的向左双头箭头"),
        ("2b3d", "带箭尾和双竖线的向左双头箭头"),
        ("2b3e", "穿过叉号的向左箭头"),
        ("2b3f", "水平向左波浪箭头"),
        ("2b40", "向左箭头上方的等号"),
        ("2b41", "向左箭头上方的反向波浪运算符"),
        ("2b42", "反向约等号上方的向左箭头"),
        ("2b43", "穿过大于号的向右箭头"),
        ("2b44", "穿过超集号的向右箭头"),
        ("2b45", "向左四重箭头"),
        ("2b46", "向右四重箭头"),
        ("2b47", "向右箭头上方的反向波浪运算符"),
        ("2b48", "反向约等号上方的向右箭头"),
        ("2b49", "向左箭头上方的波浪运算符"),
        ("2b4a", "约等号上方的向左箭头"),
        ("2b4b", "反向波浪运算符上方的向左箭头"),
        ("2b4c", "反向波浪运算符上方的向右箭头"),
        ("2b50", "白色中号星"),
        ("2b52", "白色小星"),
        ("2b53", "向右黑色五边形"),
        ("2b54", "向右白色五边形"),
        ("2b55", "粗大圆圈"),
        ("2b56", "内含椭圆的粗椭圆"),
        ("2b57", "内含圆的粗圆圈"),
        ("2b58", "粗圆圈"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn audited_unicode_symbols_keep_precise_names() -> Result<()> {
    // Each case locks a corrected identity, direction, shape, or spatial relationship.
    let cases = [
        ("00ab", "左双角引号"),
        ("21a8", "带底线的上下箭头"),
        ("21ab", "向左带环箭头"),
        ("21ac", "向右带环箭头"),
        ("21fd", "向左开口箭头"),
        ("21fe", "向右开口箭头"),
        ("21ff", "左右开口箭头"),
        ("224c", "全等于"),
        ("2247", "既不近似等于也不等于"),
        ("2298", "带圆圈除号斜线"),
        ("229f", "方框减号"),
        ("22a0", "方框乘号"),
        ("22a1", "方框点运算符"),
        ("22a9", "力迫"),
        ("22ae", "不力迫"),
        ("22dc", "等于或小于"),
        ("22e2", "既非方形像也不等于"),
        ("22e3", "既非方形原像也不等于"),
        ("2303", "向上箭头尖"),
        ("2333", "斜度"),
        ("23e6", "交流电"),
        ("2472", "带圈数字十九"),
        ("2736", "黑色六角星"),
        ("2794", "粗宽头向右箭头"),
        ("290a", "向上三重箭头"),
        ("290b", "向下三重箭头"),
        ("2983", "左白色花括号"),
        ("2984", "右白色花括号"),
        ("2993", "左弧小于括号"),
        ("2994", "右弧大于括号"),
        ("2995", "双左弧大于括号"),
        ("2996", "双右弧小于括号"),
        ("299b", "开口向左的测量角"),
        ("299d", "带点的测量直角"),
        ("2abd", "带点的子集号"),
        ("2abe", "带点的超集号"),
        ("2ad3", "子集号在超集号上方"),
        ("2ad4", "超集号在子集号上方"),
        ("2ad5", "子集号在子集号上方"),
        ("2ad6", "超集号在超集号上方"),
        ("e920", "双重方形并集"),
        ("e921", "双重方形交集"),
        ("e92c", "带点的恒等号"),
        ("e994", "带双斜杠的恒等号"),
        ("e997", "竖直正比号"),
        ("ea70", "带竖线的既不是正规子群也不等于"),
        ("ea71", "带竖线的既不包含正规子群也不等于"),
        ("eb60", "否定向右波浪箭头"),
        ("eb61", "否定向右弯曲箭头"),
        ("ec44", "水平全长三键"),
        ("ec47", "竖直全长三键"),
        ("ec4c", "水平半长三键"),
        ("fe35", "上置圆括号"),
        ("fe36", "下置圆括号"),
        ("fe37", "上置花括号"),
        ("fe38", "下置花括号"),
        ("fe3f", "上置角括号"),
        ("fe40", "下置角括号"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn audited_unicode_math_names_match_symbol_identity() -> Result<()> {
    // Each entry covers a corrected Unicode name or a standard mainland mathematical reading.
    let cases = [
        ("02d9", "上点符"),
        ("02ef", "修饰字母低位向下箭头尖"),
        ("02f0", "修饰字母低位向上箭头尖"),
        ("02f1", "修饰字母低位向左箭头尖"),
        ("02f2", "修饰字母低位向右箭头尖"),
        ("0332", "下加下划线"),
        ("0333", "下加双下划线"),
        ("2135", "阿列夫"),
        ("2136", "贝特"),
        ("2137", "吉梅尔"),
        ("2138", "达列特"),
        ("2140", "双线体求和号"),
        ("222f", "曲面积分"),
        ("2231", "顺时针积分"),
        ("225f", "问号等于"),
        ("22d8", "极小于"),
        ("22d9", "极大于"),
        ("299a", "竖直之字形线"),
        ("29e2", "混洗积"),
        ("2a00", "n 元带圈点运算符"),
        ("2a01", "n 元带圈加号运算符"),
        ("2a02", "n 元带圈乘号运算符"),
        ("2a03", "带点的 n 元并集运算符"),
        ("2a04", "带加号的 n 元并集运算符"),
        ("2a05", "n 元方交集运算符"),
        ("2a06", "n 元方并集运算符"),
        ("2a09", "n 元乘号运算符"),
        ("2a0f", "带斜线的平均积分号"),
        ("2a33", "压缩积"),
        ("2a3c", "内积"),
        ("2a3d", "右内积"),
        ("2a50", "带衬线和压缩积的闭合并集"),
        ("2a85", "小于或约等于"),
        ("2a86", "大于或约等于"),
        ("2af9", "双线倾斜小于或等于"),
        ("2afa", "双线倾斜大于或等于"),
        ("3372", "道尔顿"),
        ("3375", "小写 o 大写 V"),
        ("fb05", "长 s t 连字"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn audited_mathtype_private_use_symbols_match_their_source_names() -> Result<()> {
    // MathType PUA symbols have no Unicode fallback, so each corrected identity needs a direct check.
    let cases = [
        ("e916", "带点的超集号"),
        ("e917", "带点的子集号"),
        ("e918", "下方带点的等号"),
        ("e92e", "竖线运算符"),
        ("e92f", "双竖线运算符"),
        ("e930", "三重竖线运算符"),
        ("e949", "反转波浪号"),
        ("e950", "带竖线的正规包含于"),
        ("e951", "带竖线的包含正规子群"),
        ("e982", "带方框的直角变体"),
        ("e98f", "自由基点"),
        ("e991", "恒等于且平行于"),
        ("e992", "压缩积"),
        ("e993", "带横线的三重竖线运算符"),
        ("e995", "带三条竖线的三重横线"),
        ("e9a0", "负正弦波"),
        ("ea06", "既不小于也不等于"),
        ("ea07", "既不大于也不等于"),
        ("ea15", "既不后于也不相似于"),
        ("ea1d", "既不小于也不等于"),
        ("ea1e", "既不大于也不等于"),
        ("ea2e", "否定竖线运算符"),
        ("ea2f", "否定双竖线运算符"),
        ("ea30", "否定三重竖线运算符"),
        ("ea50", "带竖线的不是正规子群"),
        ("ea51", "带竖线的不包含正规子群"),
        ("ea55", "既不等于也不相似于"),
        ("ea63", "不严格等价于"),
        ("eb01", "小型向左箭头上方的向右箭头"),
        ("eb02", "向左箭头上方的小型向右箭头"),
        ("eb03", "小型向左鱼叉箭头上方的向右鱼叉箭头"),
        ("eb04", "向左鱼叉箭头上方的小型向右鱼叉箭头"),
        ("eb0f", "带斜线的大型左右箭头"),
        ("eb11", "带斜线的大型左右双箭头"),
        ("eb12", "左侧下箭头右侧上箭头"),
        ("eb18", "带尾部和斜线的向右箭头"),
        (
            "eb36",
            "左侧倒钩向下、右侧倒钩向上的双鱼叉箭头",
        ),
        (
            "eb37",
            "左侧倒钩向上、右侧倒钩向下的双鱼叉箭头",
        ),
        ("eb3f", "右上与右下双箭头"),
        ("eb42", "左端带空心圆点的左右箭头"),
        ("eb4c", "粗短黑色向左箭头"),
        ("eb52", "左端带空心圆点的左右鱼叉箭头"),
        ("ed00", "约化普朗克常量"),
        ("ed01", "倒置无衬线体大写 G"),
        ("ee1a", "左端带空心圆点的横线重音符"),
        ("ee1b", "左端带实心圆点的横线重音符"),
        ("ee1c", "右端带空心圆点的横线重音符"),
        ("ee23", "右端带实心圆点的横线重音符"),
        ("ef80", "顺时针围道积分"),
        ("ef81", "四元数积分运算符"),
        ("ef82", "带斜线的平均积分号"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn remaining_ea00_mathtype_private_use_symbols_match_verified_sources() -> Result<()> {
    // Audit every previously uncovered MathType PUA slot against its original character table.
    let cases = [
        (0xea00, "带竖线的非远小于"),
        (0xea01, "带竖线的非远大于"),
        (0xea02, "非远小于变体"),
        (0xea03, "非远大于变体"),
        (0xea04, "带竖线的小于且不等于"),
        (0xea05, "带竖线的大于且不等于"),
        (0xea09, "既不等于也不小于"),
        (0xea0a, "不包含也不等于"),
        (0xea0b, "不包含也不等于"),
        (0xea0c, "不包含于也不等于"),
        (0xea0d, "带反斜杠的子集符号"),
        (0xea0e, "既不等于也不大于"),
        (0xea0f, "否定减波浪运算符"),
        (0xea10, "既不等于也不小于"),
        (0xea11, "否定波浪号运算符"),
        (0xea12, "既不属于也不等于"),
        (0xea13, "既不等于也不大于"),
        (0xea14, "不约等于"),
        (0xea16, "带斜线的小于或倾斜等于"),
        (0xea17, "带斜线的大于或倾斜等于"),
        (0xea1a, "带斜杠的超集符号"),
        (0xea1b, "不包含"),
        (0xea1f, "不约等于减号"),
        (0xea22, "上加点的否定集合隶属符号"),
        (0xea2c, "带竖线的否定角符号"),
        (0xea2d, "倾斜不平行符号"),
        (0xea32, "小于但不约等于"),
        (0xea33, "大于但不约等于"),
        (0xea34, "小于或不等于"),
        (0xea35, "大于或不等于"),
        (0xea36, "非嵌套小于号"),
        (0xea37, "非嵌套大于号"),
        (0xea38, "非远小于"),
        (0xea39, "非远大于"),
        (0xea3a, "先于但不等价于"),
        (0xea3b, "后于但不等价于"),
        (0xea3c, "先于但不等于"),
        (0xea3d, "后于但不等于"),
        (0xea3e, "既不等于也不先于"),
        (0xea3f, "既不等于也不后于"),
        (0xea40, "先于但不等于"),
        (0xea41, "后于但不等于"),
        (0xea42, "不包含于也不等于"),
        (0xea43, "不包含也不等于"),
        (0xea44, "包含于或不等于"),
        (0xea45, "包含或不等于"),
        (0xea46, "不包含于也不等于"),
        (0xea47, "不包含也不等于"),
        (0xea48, "非三重小于号"),
        (0xea49, "非三重大于号"),
        (0xea4c, "不先于且不等于"),
        (0xea4d, "不后于且不等于"),
        (0xea52, "非差符号"),
        (0xea53, "几何不等价于"),
        (0xea54, "带竖线的非相似符号"),
        (0xea56, "带竖线的非约等符号"),
        (0xea57, "不近似恒等于"),
        (0xea58, "非波形等号"),
        (0xea59, "非波形单等号"),
        (0xea5a, "带点的不等号"),
        (0xea5b, "反向不等价号"),
        (0xea60, "非方形子集符号"),
        (0xea61, "非方形超集符号"),
        (0xea62, "等号上方的不约等于"),
        (0xea64, "带点的不全等符号"),
        (0xea65, "反向不等号"),
        (0xea80, "否定偏微分符号"),
        (0xeb00, "箭头修饰符延伸段"),
        (0xeb05, "右上至左下双箭头"),
        (0xeb06, "左上至右下双箭头"),
        (0xeb07, "水平鱼叉箭头延伸段"),
        (0xeb08, "逆时针弧形向左箭头"),
        (0xeb09, "逆时针弧形向右箭头"),
        (0xeb0b, "大型向右箭头重音符"),
        (0xeb0c, "大型向左箭头重音符"),
        (0xeb0d, "左箭头尖"),
        (0xeb0e, "右箭头尖"),
        (0xeb10, "水平双箭头延伸段"),
        (0xeb13, "带向下折角的向左箭头"),
        (0xeb14, "带向上折角的向右箭头"),
        (0xeb15, "带向上折角的向左箭头"),
        (0xeb16, "带加号的逆时针上半圆箭头"),
        (0xeb17, "带减号的顺时针上半圆箭头"),
        (0xeb19, "倒钩向下的向右鱼叉箭头"),
        (0xeb1a, "倒钩向下的向左鱼叉箭头"),
        (0xeb1b, "倒钩向下的左右鱼叉箭头"),
        (0xeb1c, "倒钩向上的左右鱼叉箭头"),
        (0xeb1d, "倒钩向左的上下鱼叉箭头"),
        (0xeb1e, "倒钩向右的上下鱼叉箭头"),
        (0xeb1f, "向下箭头右侧的向上箭头"),
        (0xeb20, "带倒钩向上的指向竖线的左鱼叉箭头"),
        (0xeb21, "带倒钩向上的指向竖线的右鱼叉箭头"),
        (0xeb22, "带倒钩向下的指向竖线的左鱼叉箭头"),
        (0xeb23, "带倒钩向下的指向竖线的右鱼叉箭头"),
        (0xeb24, "带倒钩向上的从竖线出发的左鱼叉箭头"),
        (0xeb25, "带倒钩向上的从竖线出发的右鱼叉箭头"),
        (0xeb26, "带倒钩向下的从竖线出发的左鱼叉箭头"),
        (0xeb27, "带倒钩向下的从竖线出发的右鱼叉箭头"),
        (0xeb28, "带倒钩向左的指向横线的上鱼叉箭头"),
        (0xeb29, "带倒钩向左的指向横线的下鱼叉箭头"),
        (0xeb2a, "带倒钩向右的指向横线的上鱼叉箭头"),
        (0xeb2b, "带倒钩向右的指向横线的下鱼叉箭头"),
        (0xeb2c, "带倒钩向左的从横线出发的上鱼叉箭头"),
        (0xeb2d, "带倒钩向左的从横线出发的下鱼叉箭头"),
        (0xeb2e, "带倒钩向右的从横线出发的上鱼叉箭头"),
        (0xeb2f, "带倒钩向右的从横线出发的下鱼叉箭头"),
        (0xeb30, "指向横线的向上箭头"),
        (0xeb31, "指向横线的向下箭头"),
        (0xeb32, "下鱼叉箭头左侧的上鱼叉箭头"),
        (0xeb33, "下鱼叉箭头右侧的上鱼叉箭头"),
        (0xeb34, "上箭头尖"),
        (0xeb35, "下箭头尖"),
        (0xeb36, "左侧倒钩向下、右侧倒钩向上的双鱼叉箭头"),
        (0xeb37, "左侧倒钩向上、右侧倒钩向下的双鱼叉箭头"),
        (0xeb38, "线上方的向左箭头"),
        (0xeb39, "线上方的向右箭头"),
        (0xeb3a, "线下方的向左箭头"),
        (0xeb3b, "线下方的向右箭头"),
        (0xeb3c, "左右三重箭头"),
        (0xeb40, "逆时针左半圆箭头"),
        (0xeb41, "顺时针左半圆箭头"),
        (0xeb44, "波浪号上方的向右箭头"),
        (0xeb45, "波浪号上方的向左箭头"),
        (0xeb48, "线上方的左鱼叉箭头"),
        (0xeb49, "线上方的右鱼叉箭头"),
        (0xeb4a, "线下方的左鱼叉箭头"),
        (0xeb4b, "线下方的右鱼叉箭头"),
        (0xeb50, "顺时针右半圆箭头"),
        (0xeb51, "逆时针右半圆箭头"),
        (0xeb58, "竖线左侧的向上箭头"),
        (0xeb59, "竖线左侧的向下箭头"),
        (0xeb5a, "竖线右侧的向上箭头"),
        (0xeb5b, "竖线右侧的向下箭头"),
        (0xeb5c, "带延长下钩的向右箭头"),
        (0xeb5d, "带延长钩的向左箭头"),
        (0xeb5e, "带延长下钩的向左箭头"),
        (0xeb5f, "带延长钩的向右箭头"),
        (0xeb68, "竖线左侧的向上鱼叉箭头"),
        (0xeb69, "竖线左侧的向下鱼叉箭头"),
        (0xeb6a, "竖线右侧的向上鱼叉箭头"),
        (0xeb6b, "竖线右侧的向下鱼叉箭头"),
        (0xeb6c, "竖直双箭头延伸段"),
        (0xeb6d, "倒钩向左的竖直鱼叉箭头延伸段"),
        (0xeb6e, "倒钩向右的竖直鱼叉箭头延伸段"),
        (0xeb6f, "左鱼叉箭头右部上方的右鱼叉箭头"),
        (0xeb70, "左鱼叉箭头左部上方的右鱼叉箭头"),
        (0xeb71, "右鱼叉箭头右部上方的左鱼叉箭头"),
        (0xeb72, "右鱼叉箭头左部上方的左鱼叉箭头"),
        (0xeb73, "从竖线出发的向左箭头尖"),
        (0xeb74, "从竖线出发的左右箭头延伸段"),
        (0xeb75, "从竖线出发的向左箭头尾部"),
        (0xeb76, "从竖线出发的向右箭头尾部"),
        (0xeb77, "从竖线出发的向右箭头尖"),
        (0xeb78, "带倒钩左箭头尖的从横线出发的上鱼叉箭头"),
        (0xeb79, "向左箭头右部上方的向右箭头"),
        (0xeb7a, "向左箭头左部上方的向右箭头"),
        (0xeb7b, "向右箭头右部上方的向左箭头"),
        (0xeb7c, "向右箭头左部上方的向左箭头"),
        (0xeb7d, "从横线出发的向上箭头尖"),
        (0xeb7e, "从横线出发的向上箭头尾部"),
        (0xeb7f, "从横线出发的向下箭头尾部"),
        (0xeb80, "从横线出发的向下箭头尖"),
        (0xeb81, "带倒钩右箭头尖的从横线出发的下鱼叉箭头"),
        (0xeb82, "下鱼叉箭头底部左侧的上鱼叉箭头"),
        (0xeb83, "下鱼叉箭头延伸段左侧的上鱼叉箭头"),
        (0xeb84, "上鱼叉箭头顶部左侧的下鱼叉箭头"),
        (0xeb85, "下鱼叉箭头顶部左侧的上鱼叉箭头"),
        (0xeb86, "上鱼叉箭头延伸段左侧的下鱼叉箭头"),
        (0xeb87, "上鱼叉箭头底部左侧的下鱼叉箭头"),
        (0xeb88, "向下箭头底部左侧的向上箭头"),
        (0xeb89, "向上箭头顶部左侧的向下箭头"),
        (0xeb8a, "向下箭头顶部左侧的向上箭头"),
        (0xeb8b, "向上箭头底部左侧的向下箭头"),
        (0xeb8c, "左右箭头延伸段"),
        (0xeb8d, "右上箭头延伸段"),
        (0xeb8e, "左上箭头延伸段"),
        (0xec00, "向下的花括号左部"),
        (0xec01, "向下的花括号中部"),
        (0xec02, "向下的花括号右部"),
        (0xec03, "水平花括号延伸段"),
        (0xec04, "向上的花括号左部"),
        (0xec05, "向上的花括号中部"),
        (0xec06, "向上的花括号右部"),
        (0xec07, "左竖线"),
        (0xec08, "右竖线"),
        (0xec09, "左双竖线"),
        (0xec0a, "右双竖线"),
        (0xec0b, "水平方括号延伸段"),
        (0xec0c, "下置方括号"),
        (0xec0d, "上置方括号"),
        (0xec0e, "下方方括号左部"),
        (0xec0f, "下方方括号右部"),
        (0xec10, "上方方括号左部"),
        (0xec11, "上方方括号右部"),
        (0xec12, "左圆括号部件一"),
        (0xec13, "左圆括号部件二"),
        (0xec14, "左圆括号部件三"),
        (0xec15, "左圆括号部件四"),
        (0xec16, "右圆括号部件一"),
        (0xec17, "右圆括号部件二"),
        (0xec18, "右圆括号部件三"),
        (0xec19, "右圆括号部件四"),
        (0xec1a, "根号部件一"),
        (0xec1b, "根号部件二"),
        (0xec1c, "根号部件三"),
        (0xec1d, "根号部件四"),
        (0xec1e, "根号部件五"),
        (0xec1f, "根号底部"),
        (0xec20, "根号竖直延伸段"),
        (0xec21, "根号顶部"),
        (0xec22, "左白色方括号顶部"),
        (0xec23, "左白色方括号延伸段"),
        (0xec24, "左白色方括号底部"),
        (0xec25, "右白色方括号顶部"),
        (0xec26, "右白色方括号延伸段"),
        (0xec27, "右白色方括号底部"),
        (0xec30, "左白色花括号"),
        (0xec31, "右白色花括号"),
        (0xec32, "长除号"),
        (0xec33, "长除号延伸段"),
        (0xec34, "短除号"),
        (0xec40, "左下至右上全长双键"),
        (0xec41, "左上至右下全长双键"),
        (0xec42, "水平全长单键"),
        (0xec43, "水平全长双键"),
        (0xec45, "竖直全长单键"),
        (0xec46, "竖直全长双键"),
        (0xec48, "小于号形全长键"),
        (0xec49, "大于号形全长键"),
        (0xec4a, "水平半长单键"),
        (0xec4b, "水平半长双键"),
        (0xec80, "矩形左上部"),
        (0xec81, "矩形左下部"),
        (0xec90, "矩形右上部"),
        (0xec91, "矩形右下部"),
        (0xec92, "综合除法角部"),
        (0xec93, "综合除法水平延伸段"),
        (0xec94, "综合除法竖直延伸段"),
        (0xec95, "左侧上下取整符号延伸段"),
        (0xec96, "右侧上下取整符号延伸段"),
        (0xec97, "上方括号延伸段"),
        (0xec98, "竖线延伸段"),
        (0xec99, "左双竖线延伸段"),
        (0xec9a, "横线延伸段"),
        (0xec9c, "下方括号延伸段"),
        (0xec9d, "向下的圆括号右部"),
        (0xec9e, "向下的圆括号延伸段"),
        (0xec9f, "向下的圆括号左部"),
        (0xeca0, "向上的花括号延伸段"),
        (0xeca1, "向上的圆括号左部"),
        (0xeca2, "向上的圆括号延伸段"),
        (0xeca3, "向上的圆括号右部"),
        (0xeca4, "向下的花括号延伸段"),
        (0xed02, "无点 j"),
        (0xed03, "digamma"),
        (0xed10, "d"),
        (0xed11, "e"),
        (0xed12, "i"),
        (0xed13, "j"),
        (0xed16, "大写 d"),
        (0xee00, "逆时针围道积分环"),
        (0xee01, "顺时针围道积分环"),
        (0xee0d, "连接状态修饰符"),
        (0xee0e, "连接状态左侧修饰符"),
        (0xee0f, "连接状态右侧修饰符"),
        (0xee10, "连接状态延伸段"),
        (0xee11, "积分环"),
        (0xee12, "双积分环"),
        (0xee13, "三重积分环"),
        (0xee15, "双扩展积分环"),
        (0xee16, "三重扩展积分环"),
        (0xee17, "渐近等于重音符"),
        (0xee18, "等号重音符"),
        (0xee19, "四撇号"),
        (0xee1d, "带上点的横线重音符"),
        (0xee1e, "带下点的横线重音符"),
        (0xee1f, "带上方双点的横线重音符"),
        (0xee20, "带下方双点的横线重音符"),
        (0xee21, "带脱字符的横线重音符"),
        (0xee22, "粗下横线重音符"),
        (0xee24, "上加大点"),
        (0xef00, "对齐标记"),
        (0xef41, "缺项"),
        (0xef83, "反向积分号"),
        (0xef90, "双零上方的双零"),
        (0xef91, "带斜线的零"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint:x};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("MathType PUA U+{codepoint:04X}: {error}"))?;
    }

    // Wide accents and spacing controls are layout-only and must remain silent.
    let silent = (0xee04..=0xee0c)
        .chain(0xef01..=0xef0a)
        .chain(0xef22..=0xef24)
        .chain(std::iter::once(0xef29));
    for codepoint in silent {
        let expr = format!("<math><mo>&#x{codepoint:x};</mo></math>");
        test("zh", "SimpleSpeak", &expr, "")
            .map_err(|error| anyhow::anyhow!("silent MathType PUA U+{codepoint:04X}: {error}"))?;
    }
    Ok(())
}

#[test]
fn mathtype_double_struck_greek_matches_the_original_character_table() -> Result<()> {
    // Check every MathType PUA slot because the source order is non-alphabetic in both ranges.
    let capitals = [
        ("f201", "delta"),
        ("f202", "zai"),
        ("f203", "lambda"),
        ("f204", "pi"),
        ("f205", "sigma"),
        ("f206", "theta"),
        ("f207", "gamma"),
        ("f208", "omega"),
        ("f209", "upsilon"),
    ];
    for (codepoint, name) in capitals {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        let expected = format!("双线体 大写 {name}");
        test("zh", "SimpleSpeak", &expr, &expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }

    let lowercase = [
        ("f220", "alpha"),
        ("f221", "beta"),
        ("f222", "phi"),
        ("f223", "zeta"),
        ("f224", "psi"),
        ("f225", "delta"),
        ("f226", "epsilon"),
        ("f227", "gamma"),
        ("f228", "eta"),
        ("f229", "iota"),
        ("f22a", "zai"),
        ("f22b", "kappa"),
        ("f22c", "lambda"),
        ("f22d", "mu"),
        ("f22e", "nu"),
        ("f22f", "epsilon"),
        ("f230", "pi"),
        ("f231", "theta"),
        ("f232", "rho"),
        ("f233", "sigma"),
        ("f234", "tau"),
        ("f235", "theta"),
        ("f236", "omega"),
    ];
    for (codepoint, name) in lowercase {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        let expected = format!("双线体 {name}");
        test("zh", "SimpleSpeak", &expr, &expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }

    let special = [
        ("f237", "双线体 final sigma"),
        ("f250", "双线体 rho"),
        ("f251", "双线体 phi"),
    ];
    for (codepoint, expected) in special {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn mathematical_capital_theta_symbols_remain_speakable_in_every_variant() -> Result<()> {
    // Each 25-character range has an extra theta-symbol slot where U+03A2 must not be used.
    let cases = [
        ("1d6b9", "粗体 大写 theta"),
        ("f419", "粗体 大写 theta"),
        ("1d6f3", "大写 theta"),
        ("f453", "大写 theta"),
        ("1d72d", "粗体 大写 theta"),
        ("f48d", "粗体 大写 theta"),
        ("1d767", "粗体 大写 theta"),
        ("f4c7", "粗体 大写 theta"),
        ("1d7a1", "粗体 大写 theta"),
        ("f501", "粗体 大写 theta"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn audited_unicode_ranges_keep_their_boundaries() -> Result<()> {
    // The first and last characters catch off-by-one and shifted translate mappings.
    let cases = [
        ("03aa", "大写 iota 带分音符"),
        ("03ab", "大写 upsilon 带分音符"),
        ("03cf", "大写 kai"),
        ("24b6", "带圈 大写 a"),
        ("24cf", "带圈 大写 z"),
        ("24d0", "带圈 a"),
        ("24e9", "带圈 z"),
        ("1d538", "双线体 大写 a"),
        ("1d550", "双线体 大写 y"),
        ("f000", "哥特体 大写 a"),
        ("f018", "哥特体 大写 y"),
        ("f01a", "哥特体 a"),
        ("f033", "哥特体 z"),
        ("f040", "哥特体 粗体 大写 a"),
        ("f059", "哥特体 粗体 大写 z"),
        ("f05a", "哥特体 粗体 a"),
        ("f073", "哥特体 粗体 z"),
        ("f080", "双线体 大写 a"),
        ("f098", "双线体 大写 y"),
        ("f09a", "双线体 a"),
        ("f0b3", "双线体 z"),
        ("f0c0", "双线体 0"),
        ("f0c9", "双线体 9"),
        ("f0ca", "双线体纳布拉"),
        ("f0cb", "双线体欧拉常数"),
        ("f100", "花体 大写 a"),
        ("f119", "花体 大写 z"),
        ("f11a", "花体 a"),
        ("f133", "花体 z"),
        ("f140", "花体 粗体 大写 a"),
        ("f159", "花体 粗体 大写 z"),
        ("f15a", "花体 粗体 a"),
        ("f173", "花体 粗体 z"),
        ("f180", "大写 a"),
        ("f199", "大写 z"),
        ("f19a", "大写 ae 连字"),
        ("f19b", "大写 德语双 s"),
        ("f19c", "大写 带斜线的 o"),
        ("f260", "粗体 大写 a"),
        ("f279", "粗体 大写 z"),
        ("f27a", "粗体 a"),
        ("f293", "粗体 z"),
        ("f294", "大写 a"),
        ("f2ad", "大写 z"),
        ("f2ae", "a"),
        ("f2c7", "z"),
        ("f2c8", "粗体 大写 a"),
        ("f2e1", "粗体 大写 z"),
        ("f2e2", "粗体 a"),
        ("f2fb", "粗体 z"),
        ("f300", "大写 a"),
        ("f319", "大写 z"),
        ("f31a", "a"),
        ("f333", "z"),
        ("f334", "粗体 大写 a"),
        ("f34d", "粗体 大写 z"),
        ("f34e", "粗体 a"),
        ("f367", "粗体 z"),
        ("f368", "大写 a"),
        ("f381", "大写 z"),
        ("f382", "a"),
        ("f39b", "z"),
        ("f39c", "粗体 大写 a"),
        ("f3b5", "粗体 大写 z"),
        ("f3b6", "粗体 a"),
        ("f3cf", "粗体 z"),
        ("f3d0", "大写 a"),
        ("f3e9", "大写 z"),
        ("f3ea", "a"),
        ("f403", "z"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn confirmed_greek_unicode_variants_keep_chinese_modifiers_and_english_cores() -> Result<()> {
    let cases = [
        ("02e0", "修饰用小写 gamma"),
        ("03ca", "带分音符的 iota"),
        ("03cb", "带分音符的 upsilon"),
        ("03cc", "带重音符的 omicron"),
        ("03cd", "带重音符的 upsilon"),
        ("03ce", "带重音符的 omega"),
        ("03d0", "beta"),
        ("03d1", "theta"),
        ("03d2", "带钩的 upsilon"),
        ("03d3", "带锐音与钩的 upsilon"),
        ("03d4", "带分音与钩的 upsilon"),
        ("03d7", "kai"),
        ("03d8", "大写 archaic koppa"),
        ("03d9", "archaic koppa"),
        ("03dd", "digamma"),
        ("213c", "双线体 pi"),
        ("213d", "双线体 gamma"),
        ("f250", "双线体 rho"),
        ("f251", "双线体 phi"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn mathtype_greek_and_digit_ranges_keep_verified_character_order() -> Result<()> {
    // Check every mapped MathType slot in U+F404-U+F555, including easily shifted variants.
    let capitals = [
        "alpha",
        "beta",
        "gamma",
        "delta",
        "epsilon",
        "zeta",
        "eta",
        "theta",
        "iota",
        "kappa",
        "lambda",
        "mu",
        "nu",
        "zai",
        "omicron",
        "pi",
        "rho",
        "theta",
        "sigma",
        "tau",
        "upsilon",
        "phi",
        "chi",
        "psi",
        "omega",
    ];
    let lowercase = [
        "alpha",
        "beta",
        "gamma",
        "delta",
        "epsilon",
        "zeta",
        "eta",
        "theta",
        "iota",
        "kappa",
        "lambda",
        "mu",
        "nu",
        "zai",
        "omicron",
        "pi",
        "rho",
        "final sigma",
        "sigma",
        "tau",
        "upsilon",
        "phi",
        "chi",
        "psi",
        "omega",
    ];
    let variants = ["偏导数", "epsilon", "theta", "kappa", "phi", "rho", "pi"];
    let families = [
        (0xf408, 0xf421, 0xf422, 0xf43b, "粗体 ", "粗体纳布拉"),
        (0xf442, 0xf45b, 0xf45c, 0xf475, "", "斜体纳布拉"),
        (0xf47c, 0xf495, 0xf496, 0xf4af, "粗体 ", "粗斜体纳布拉"),
        (0xf4b6, 0xf4cf, 0xf4d0, 0xf4e9, "粗体 ", "粗体纳布拉"),
        (0xf4f0, 0xf509, 0xf50a, 0xf523, "粗体 ", "粗体纳布拉"),
    ];

    let check = |codepoint: u32, expected: &str| -> Result<()> {
        let expr = format!("<math><mi>&#x{codepoint:x};</mi></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint:04X}: {error}"))
    };

    check(0xf404, "无点 i")?;
    for (capital, nabla, small, variant, prefix, nabla_name) in families {
        for (offset, name) in capitals.iter().enumerate() {
            check(capital + offset as u32, &format!("{prefix}大写 {name}"))?;
        }
        check(nabla, nabla_name)?;
        for (offset, name) in lowercase.iter().enumerate() {
            check(small + offset as u32, &format!("{prefix}{name}"))?;
        }
        for (offset, name) in variants.iter().enumerate() {
            check(variant + offset as u32, &format!("{prefix}{name}"))?;
        }
    }
    for start in [0xf52e, 0xf54c] {
        for digit in 0..=9 {
            check(start + digit, &format!("粗体 {digit}"))?;
        }
    }
    Ok(())
}

#[test]
fn remaining_mathtype_and_unicode_slots_match_verified_sources() -> Result<()> {
    // Cover the remaining digit ranges and source-identified MathType PUA glyphs slot by slot.
    for start in [0xf556, 0x1d7e2, 0x1d7f6] {
        for digit in 0..=9 {
            let codepoint = start + digit;
            let expr = format!("<math><mi>&#x{codepoint:x};</mi></math>");
            test("zh", "SimpleSpeak", &expr, &digit.to_string())
                .map_err(|error| anyhow::anyhow!("U+{codepoint:04X}: {error}"))?;
        }
    }

    let cases = [
        (0xf700, "未知字符"),
        (0xf726, "左下和右下三角形"),
        (0xf72d, "水平省略号延伸段"),
        (0xf72e, "中线水平省略号延伸段"),
        (0xf8e5, "根号延伸段"),
        (0xf8e6, "竖直箭头延伸段"),
        (0xf8e7, "水平箭头延伸段"),
        (0xf8e8, "无衬线体注册符号"),
        (0xf8e9, "无衬线体版权符号"),
        (0xf8ea, "无衬线体商标符号"),
        (0xf8eb, "左圆括号顶部"),
        (0xf8ec, "左圆括号延伸段"),
        (0xf8ed, "左圆括号底部"),
        (0xf8ee, "左方括号顶部"),
        (0xf8ef, "左方括号延伸段"),
        (0xf8f0, "左方括号底部"),
        (0xf8f1, "左花括号顶部"),
        (0xf8f2, "左花括号中部"),
        (0xf8f3, "左花括号底部"),
        (0xf8f4, "花括号延伸段"),
        (0xf8f5, "积分号延伸段"),
        (0xf8f6, "右圆括号顶部"),
        (0xf8f7, "右圆括号延伸段"),
        (0xf8f8, "右圆括号底部"),
        (0xf8f9, "右方括号顶部"),
        (0xf8fa, "右方括号延伸段"),
        (0xf8fb, "右方括号底部"),
        (0xf8fc, "右花括号顶部"),
        (0xf8fd, "右花括号中部"),
        (0xf8fe, "右花括号底部"),
        (0xf8ff, "苹果标志"),
    ];
    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint:x};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint:04X}: {error}"))?;
    }
    Ok(())
}

#[test]
fn remaining_ligatures_combining_halves_and_small_forms_match_unicode() -> Result<()> {
    // Cover the compatibility and combining-form names not already checked by focused tests.
    let cases = [
        ("fb00", "ff 连字"),
        ("fb01", "fi 连字"),
        ("fb02", "fl 连字"),
        ("fb03", "ffi 连字"),
        ("fb04", "ffl 连字"),
        ("fb06", "st 连字"),
        ("fb29", "希伯来文替代加号"),
        ("fe20", "上加连字左半部"),
        ("fe21", "上加连字右半部"),
        ("fe22", "上加双波浪号左半部"),
        ("fe23", "上加双波浪号右半部"),
        ("fe24", "上加长音符左半部"),
        ("fe25", "上加长音符右半部"),
        ("fe26", "上加连接长音符"),
        ("fe61", "小型星号"),
        ("fe62", "小型加号"),
        ("fe63", "小型连字符减号"),
        ("fe66", "小型等号"),
    ];
    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn remaining_fullwidth_and_replacement_characters_have_verified_readings() -> Result<()> {
    // Cover the final Unicode compatibility characters, including NVDA's established caret term.
    let cases = [
        ("ff0b", "全角加号"),
        ("ff1c", "小于"),
        ("ff1d", "等于"),
        ("ff1e", "大于"),
        ("ff3c", "反斜杠"),
        ("ff3e", "全角脱字符"),
        ("ff5c", "竖线"),
        ("ff5e", "波浪号"),
        ("ffe2", "非"),
        ("ffe9", "向左箭头"),
        ("ffea", "向上箭头"),
        ("ffeb", "向右箭头"),
        ("ffec", "向下箭头"),
        ("fffc", "未知或缺失的对象"),
        ("fffd", "缺失字符"),
    ];
    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    test(
        "zh",
        "SimpleSpeak",
        "<math><mover accent='true'><mi>x</mi><mo>&#xff3e;</mo></mover></math>",
        "x 上方有 帽符",
    )?;
    Ok(())
}

#[test]
fn mathjax_v4_private_use_chemistry_symbols_match_their_source_semantics() -> Result<()> {
    // MathJax mhchem emits these PUA code points for partial bonds and arrows.
    let cases = [
        ("e410", "部分单键"),
        ("e411", "部分双键"),
        ("e412", "部分三键"),
        ("e413", "部分三键"),
        ("e428", "长向左箭头"),
        ("e429", "长向右箭头"),
        ("e42a", "长左右箭头"),
        ("e408", "可逆反应"),
        ("e409", "平衡偏右"),
        ("e40a", "平衡偏左"),
        ("e42b", "上方右箭头下方左箭头"),
        ("e42c", "向左箭头"),
        ("e42d", "向右箭头"),
        ("e42e", "左右箭头"),
    ];
    for (codepoint, expected) in cases {
        let expr = format!(
            "<math><mo mathvariant='-mhchem' data-mjx-texclass='REL'>&#x{codepoint};</mo></math>"
        );
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("MathJax PUA U+{codepoint}: {error}"))?;
    }

    for (codepoint, latex, expected) in [
        ("e408", r"\mhchemlongrightleftharpoons", "大写 h 可逆反应 大写 i"),
        ("e409", r"\mhchemlongRightleftharpoons", "大写 h 平衡偏右 大写 i"),
        ("e40a", r"\mhchemlongLeftrightharpoons", "大写 h 平衡偏左 大写 i"),
    ] {
        let expr = format!(
            "<math><mrow data-mjx-texclass='ORD'><mi mathvariant='normal'>H</mi><mo mathvariant='-mhchem' data-mjx-texclass='REL' stretchy='true' data-latex='{latex}'>&#x{codepoint};</mo><mi mathvariant='normal'>I</mi></mrow></math>"
        );
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("MathJax chemistry U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn enclosed_alphanumeric_names_use_consistent_mainland_terms() -> Result<()> {
    // Negative circled capitals and double-circled digits must match NVDA's established terms.
    let cases = [
        ("1f150", "带圈反白 大写 a"),
        ("1f169", "带圈反白 大写 z"),
        ("24f5", "双圈 1"),
        ("24fd", "双圈 9"),
        ("24fe", "双圈数字十"),
    ];

    for style in ["SimpleSpeak", "ClearSpeak"] {
        for (codepoint, expected) in cases {
            let expr = format!("<math><mi>&#x{codepoint};</mi></math>");
            test("zh", style, &expr, expected)
                .map_err(|error| anyhow::anyhow!("{style}/U+{codepoint}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn unicode_dingbats_editorial_marks_and_compatibility_units_are_precise() -> Result<()> {
    // Less common symbols still need exact names because visual context is unavailable to speech users.
    // U+33C2/U+33D8 decompose to a.m./p.m.; they are not atto-/picometre units.
    let cases = [
        ("2798", "粗向右下箭头"),
        ("27a2", "顶部高亮的立体向右箭头尖"),
        ("27b4", "黑色羽状向右下箭头"),
        ("27bc", "楔尾向右箭头"),
        ("2999", "点状围栏"),
        ("29cc", "三角形内字母 S"),
        ("2b1d", "黑色极小方块"),
        ("2b2a", "黑色小长菱形"),
        ("2b2c", "黑色横向椭圆"),
        ("2b30", "带小圆圈的向左箭头"),
        ("2b33", "向左长曲线箭头"),
        ("2b51", "黑色小星"),
        ("2b59", "粗圆圈内的叉号"),
        ("2e00", "直角替换标记"),
        ("2e08", "点状换位标记"),
        ("2e13", "带点奥贝洛斯符号"),
        ("2e16", "带点的右指角"),
        ("2e18", "倒置疑问感叹号"),
        ("2e19", "棕榈枝"),
        ("2e1b", "上方带圆环的波浪线"),
        ("2e30", "圆环点"),
        ("3014", "左六角括号"),
        ("3015", "右六角括号"),
        ("3018", "左白六角括号"),
        ("3019", "右白六角括号"),
        ("33c2", "上午"),
        ("33d8", "下午"),
        ("33da", "拍伦琴"),
        ("33d4", "毫巴"),
        ("33c7", "公司"),
        ("33ff", "伽"),
        ("fe64", "小型小于号"),
        ("fe65", "小型大于号"),
    ];

    for (codepoint, expected) in cases {
        let expr = format!("<math><mo>&#x{codepoint};</mo></math>");
        test("zh", "SimpleSpeak", &expr, expected)
            .map_err(|error| anyhow::anyhow!("U+{codepoint}: {error}"))?;
    }
    Ok(())
}

#[test]
fn gallon_unit_remains_distinct_from_the_gal_acceleration_symbol() -> Result<()> {
    // The unit token "gal" means gallon; U+33FF is the Gal acceleration unit and is tested above.
    test(
        "zh",
        "SimpleSpeak",
        "<math><mn>1</mn><mi intent=':unit'>gal</mi></math>",
        "1 加仑",
    )
}

fn init_navigation(mathml: &str) -> Result<()> {
    set_rules_dir(abs_rules_dir_path())?;
    set_preference("Language", "zh")?;
    set_preference("SpeechStyle", "SimpleSpeak")?;
    set_preference("Verbosity", "Medium")?;
    set_preference("NavMode", "Enhanced")?;
    set_preference("NavVerbosity", "Verbose")?;
    set_preference("AutoZoomOut", "False")?;
    set_preference("Overview", "False")?;
    set_mathml(mathml)?;
    Ok(())
}

#[test]
fn navigation_enters_fraction_numerator() -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_navigation(
            "<math><mfrac id='frac'><mn id='num'>1</mn><mn id='den'>2</mn></mfrac></math>",
        )?;
        let speech = do_navigate_command("SetPlacemarker1")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("设置位置标记 1; 2 分之 1", speech);

        let speech = do_navigate_command("DescribeCurrent")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("概述 当前项; 2 分之 1", speech);

        let speech = do_navigate_command("ZoomIn")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("进入下一层; 进入 分子; 1", speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn navigation_uses_chinese_row_and_column_number_order() -> Result<()> {
    // Cell location announcements use "第 n 行，第 n 列" rather than noun-first order.
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_navigation(
            "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn id='r1c1'>1</mn></mtd><mtd><mn id='r1c2'>2</mn></mtd></mtr><mtr><mtd><mn id='r2c1'>3</mn></mtd><mtd><mn id='r2c2'>4</mn></mtd></mtr></mtable><mo>)</mo></mrow></math>",
        )?;
        set_navigation_node("r2c2", 0)?;
        let speech = do_navigate_command("ReadCellCurrent")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("朗读当前单元格; 第 2 行, 第 2 列, 4", speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn navigation_overview_keeps_the_root_possessive_marker() -> Result<()> {
    // Overview speech must keep 的 after a complex radicand, just like full speech.
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        init_navigation(
            "<math><mroot><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mi>n</mi></mroot></math>",
        )?;
        let speech = do_navigate_command("DescribeCurrent")?;
        let speech = speech.trim_end_matches([' ', ',', ';']);
        assert_eq!("概述 当前项; x 加 y 的 n 次方根", speech);
        Ok(())
    }));
    report_any_panic(result)
}

#[test]
fn clearspeak_multiline_labels_use_chinese_count_and_ordinal_order() -> Result<()> {
    // Overview counts take classifiers, while each branch is introduced as an ordinal.
    test(
        "zh",
        "ClearSpeak",
        "<math><mrow><mo>{</mo><mtable><mtr><mtd><mi>x</mi><mo>&gt;</mo><mn>0</mn></mtd></mtr><mtr><mtd><mi>x</mi><mo>&lt;</mo><mn>0</mn></mtd></mtr></mtable></mrow></math>",
        "2 个分支; 第 1 个分支; x 大于 0; 第 2 个分支; x 小于 0",
    )
}

#[test]
fn calculus_formulas_cover_limits_derivatives_and_improper_integrals() -> Result<()> {
    // Exercise complete textbook formulas so contextual operators are not only tested in isolation.
    let cases = [
        (
            "limit-at-infinity",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2192;</mo><mi>&#x221e;</mi></mrow></munder><mfrac><mn>1</mn><mi>x</mi></mfrac></math>",
            "极限，当 x 趋于 无穷大; x 分之 1",
        ),
        (
            "left-hand-limit",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2197;</mo><mn>2</mn></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>",
            "极限，当 x 从下方趋于 2; f x",
        ),
        (
            "right-hand-limit",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2198;</mo><mn>2</mn></mrow></munder><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>",
            "极限，当 x 从上方趋于 2; f x",
        ),
        (
            "first-derivative",
            "<math><mfrac><mrow><mo>d</mo><mi>y</mi></mrow><mrow><mo>d</mo><mi>x</mi></mrow></mfrac><mo>=</mo><mn>2</mn><mi>x</mi></math>",
            "分数, d x, 分之, d y, 结束分数; 等于 2 x",
        ),
        (
            "second-derivative",
            "<math><mfrac><mrow><msup><mo>d</mo><mn>2</mn></msup><mi>y</mi></mrow><mrow><mi>d</mi><msup><mi>x</mi><mn>2</mn></msup></mrow></mfrac></math>",
            "分数, d x 平方, 分之, d 平方 y, 结束分数",
        ),
        (
            "improper-integral",
            "<math><msubsup><mo>&#x222b;</mo><mn>0</mn><mi>&#x221e;</mi></msubsup><msup><mi>e</mi><mrow><mo>&#x2212;</mo><mi>x</mi></mrow></msup><mo>&#x2062;</mo><mi>d</mi><mi>x</mi></math>",
            "积分 从 0 到 无穷大; e 的 负 x 次方, d x",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn nested_numbers_powers_and_functions_have_stable_readings() -> Result<()> {
    // Cover structures whose meaning depends on nesting or adjacent notation.
    let cases = [
        (
            "nested-fraction",
            "<math><mfrac><mn>1</mn><mfrac><mn>1</mn><mi>x</mi></mfrac></mfrac></math>",
            "分数, 分数, x 分之 1, 结束分数; 分之 1, 结束分数",
        ),
        (
            "mixed-number",
            "<math><mn>2</mn><mo>&#x2064;</mo><mfrac><mn>1</mn><mn>3</mn></mfrac></math>",
            "2 又 3 分之 1",
        ),
        (
            "decimal-fraction",
            "<math><mfrac><mn>0.25</mn><mn>0.5</mn></mfrac></math>",
            "0.5 分之 0.25",
        ),
        (
            "negative-exponent",
            "<math><msup><mi>x</mi><mrow><mo>&#x2212;</mo><mn>2</mn></mrow></msup></math>",
            "x 的 负 2 次方",
        ),
        (
            "natural-logarithm",
            "<math><mi>ln</mi><mo>&#x2061;</mo><mi>x</mi></math>",
            "自然对数 x",
        ),
        (
            "exponential-function",
            "<math><msup><mi>e</mi><mi>x</mi></msup></math>",
            "e 的 x 次方",
        ),
        (
            "factorial",
            "<math><mn>5</mn><mo>!</mo></math>",
            "5 阶乘",
        ),
        (
            "double-factorial",
            "<math><mn>5</mn><mo>&#x203c;</mo></math>",
            "5 双阶乘",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn symbolic_sets_logic_and_probability_read_as_complete_expressions() -> Result<()> {
    // Test the presentation notation users encounter, in addition to semantic intent rules.
    let cases = [
        (
            "set-membership",
            "<math><mi>x</mi><mo>&#x2208;</mo><mi>&#x211d;</mi><mo>,</mo><mi>n</mi><mo>&#x2209;</mo><mi>&#x2124;</mi></math>",
            "x 属于 实数集, 逗号; n 不属于 整数集",
        ),
        (
            "nested-quantifiers",
            "<math><mo>&#x2200;</mo><mi>x</mi><mo>&#x2203;</mo><mi>y</mi><mo>:</mo><mi>P</mi><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>)</mo></math>",
            "任意 x 存在 y, 冒号; 大写 p, 左括号 x 逗号, y, 右括号",
        ),
        (
            "probability-function",
            "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>)</mo></math>",
            "大写 a 的概率",
        ),
        (
            "logical-equivalence",
            "<math><mi>p</mi><mo>&#x21d2;</mo><mi>q</mi><mo>&#x21d4;</mo><mi>r</mi><mo>&#x2227;</mo><mi>s</mi></math>",
            "p 推出, q 当且仅当 r 且 s",
        ),
        (
            "conditional-probability-identity",
            "<math><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo><mo>=</mo><mfrac><mrow><mi>P</mi><mo>(</mo><mi>A</mi><mo>&#x2229;</mo><mi>B</mi><mo>)</mo></mrow><mrow><mi>P</mi><mo>(</mo><mi>B</mi><mo>)</mo></mrow></mfrac></math>",
            "在 大写 b 条件下 大写 a 的概率, 等于; 分数, 大写 b 的概率, 分之, 大写 a 交 大写 b 的概率, 结束分数",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn vectors_geometry_and_complex_conjugates_keep_mathematical_context() -> Result<()> {
    // Combine notation so arrows, bars, angle signs, and relation symbols retain their mathematical meanings.
    let cases = [
        (
            "vector-components",
            "<math><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>=</mo><mi>a</mi><mi>i</mi><mo>+</mo><mi>b</mi><mi>j</mi></math>",
            "向量 v, 等于, a i 加 b j",
        ),
        (
            "right-angle-and-parallel-lines",
            "<math><mo>&#x2220;</mo><mi>A</mi><mo>=</mo><mn>90</mn><mo>&#xb0;</mo><mo>,</mo><mi>l</mi><mo>&#x2225;</mo><mi>m</mi></math>",
            "角 大写 a 等于 90 度; 逗号, l 平行于 m",
        ),
        (
            "complex-conjugate-bar",
            "<math><mi>z</mi><mo>=</mo><mn>3</mn><mo>+</mo><mn>4</mn><mi>i</mi><mo>,</mo><mover><mi>z</mi><mo>&#xaf;</mo></mover></math>",
            "z 等于 3 加 4 i; 逗号, z 上横线",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn common_compound_units_use_natural_chinese_order() -> Result<()> {
    // Keep number-unit adjacency and numerator-before-denominator unit order in real measurements.
    let cases = [
        (
            "speed",
            "<math><mfrac><mrow><mn>90</mn><mi intent=':unit'>km</mi></mrow><mi intent=':unit'>h</mi></mfrac></math>",
            "90 千米每 小时",
        ),
        (
            "temperature",
            "<math><mn>25</mn><mi intent=':unit'>&#xb0;C</mi></math>",
            "25 摄氏度",
        ),
    ];

    for (name, mathml, expected) in cases {
        test("zh", "SimpleSpeak", mathml, expected)
            .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn additional_real_world_readings() -> Result<()> {
    let cases = [
        (
            "trig-simple",
            "SimpleSpeak",
            "Medium",
            "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi><mo>+</mo><msup><mi>cos</mi><mn>2</mn></msup><mi>x</mi><mo>=</mo><mn>1</mn></math>",
            "正弦 平方 x, 加 余弦 平方 x; 等于 1",
        ),
        (
            "trig-clear",
            "ClearSpeak",
            "Medium",
            "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi><mo>+</mo><msup><mi>cos</mi><mn>2</mn></msup><mi>x</mi><mo>=</mo><mn>1</mn></math>",
            "正弦 平方 x, 加 余弦 平方 x; 等于 1",
        ),
        (
            "log-base-simple",
            "SimpleSpeak",
            "Medium",
            "<math><msub><mi>log</mi><mn>2</mn></msub><mn>8</mn><mo>=</mo><mn>3</mn></math>",
            "以 2 为底, 8 的对数, 等于 3",
        ),
        (
            "log-base-clear",
            "ClearSpeak",
            "Medium",
            "<math><msub><mi>log</mi><mn>2</mn></msub><mn>8</mn><mo>=</mo><mn>3</mn></math>",
            "以 2 为底, 8 的对数, 等于 3",
        ),
        (
            "limit-simple",
            "SimpleSpeak",
            "Medium",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>&#x2192;</mo><mn>0</mn></mrow></munder><mfrac><mrow><mi>sin</mi><mo>&#x2061;</mo><mi>x</mi></mrow><mi>x</mi></mfrac><mo>=</mo><mn>1</mn></math>",
            "极限，当 x 趋于 0; 分数, x 分之, 正弦 x, 结束分数; 等于 1",
        ),
        (
            "partial-derivative",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='partial-derivative($x)'><mi arg='x'>f</mi></mrow></math>",
            "f 的偏导数",
        ),
        (
            "sum",
            "SimpleSpeak",
            "Medium",
            "<math><munderover><mo>&#x2211;</mo><mrow><mi>k</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msup><mi>k</mi><mn>2</mn></msup></math>",
            "求和 从 k 等于 1 到 n, k 平方",
        ),
        (
            "product",
            "SimpleSpeak",
            "Medium",
            "<math><munderover><mo>&#x220f;</mo><mrow><mi>k</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><mi>k</mi></math>",
            "连乘 从 k 等于 1 到 n k",
        ),
        (
            "prime-derivative",
            "SimpleSpeak",
            "Medium",
            "<math><msup><mi>f</mi><mo>&#x2032;</mo></msup><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mo>=</mo><mn>2</mn><mi>x</mi></math>",
            "f 撇号, x, 等于 2 x",
        ),
        (
            "dot-product",
            "SimpleSpeak",
            "Medium",
            "<math><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>&#x22c5;</mo><mover><mi>w</mi><mo>&#x2192;</mo></mover></math>",
            "向量 v, 点乘 向量 w",
        ),
        (
            "explicit-dot-product-intent",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='dot-product($a,$b)'><mi arg='a'>v</mi><mi arg='b'>w</mi></mrow></math>",
            "v 与 w 的数量积",
        ),
        (
            "scalar-centered-dot",
            "SimpleSpeak",
            "Medium",
            "<math><mi>v</mi><mo>&#x22c5;</mo><mi>w</mi></math>",
            "v 乘 w",
        ),
        (
            "cross-product",
            "SimpleSpeak",
            "Medium",
            "<math><mover><mi>v</mi><mo>&#x2192;</mo></mover><mo>&#xd7;</mo><mover><mi>w</mi><mo>&#x2192;</mo></mover></math>",
            "向量 v, 叉乘 向量 w",
        ),
        (
            "explicit-cross-product-intent",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='cross-product($a,$b)'><mi arg='a'>v</mi><mi arg='b'>w</mi></mrow></math>",
            "v 与 w 的叉积",
        ),
        (
            "plain-norm",
            "SimpleSpeak",
            "Medium",
            "<math><mrow><mo>&#x2225;</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>&#x2225;</mo></mrow></math>",
            "x 加 y 的范数",
        ),
        (
            "norm",
            "SimpleSpeak",
            "Medium",
            "<math><msub><mrow><mo>&#x2225;</mo><mi>x</mi><mo>&#x2225;</mo></mrow><mn>2</mn></msub></math>",
            "x 的 2 范数",
        ),
        (
            "determinant",
            "SimpleSpeak",
            "Medium",
            "<math><mrow><mo>|</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>|</mo></mrow></math>",
            "2 乘 2 行列式; 第 1 行; a, b; 第 2 行; c, d",
        ),
        (
            "mean",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='mean($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的平均值",
        ),
        (
            "standard-deviation",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='standard-deviation($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的标准差",
        ),
        (
            "variance",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='variance($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的方差",
        ),
        (
            "median",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='median($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的中位数",
        ),
        (
            "mode",
            "ClearSpeak",
            "Medium",
            "<math><mrow intent='mode($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的众数",
        ),
        (
            "water-terse",
            "SimpleSpeak",
            "Terse",
            "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>",
            "大写 h, 2 大写 o",
        ),
        (
            "water-medium",
            "SimpleSpeak",
            "Medium",
            "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>",
            "大写 h, 下标 2, 大写 o",
        ),
        (
            "water-verbose",
            "SimpleSpeak",
            "Verbose",
            "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>",
            "大写 h, 下标 2, 大写 o",
        ),
        (
            "sulfate-medium",
            "SimpleSpeak",
            "Medium",
            "<math><msup><mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow><mrow><mn>2</mn><mo>&#x2212;</mo></mrow></msup></math>",
            "左方括号, 大写 s, 大写 o, 下标 4; 右方括号 上标 2 负",
        ),
        (
            "aqueous-terse",
            "SimpleSpeak",
            "Terse",
            "<math><mi>Fe</mi><msub><mi>Cl</mi><mn>3</mn></msub><mrow><mo>(</mo><mi>aq</mi><mo>)</mo></mrow></math>",
            "大写 f e, 大写 c l, 3 水溶液",
        ),
        (
            "acceleration-unit",
            "SimpleSpeak",
            "Medium",
            "<math><mfrac><mrow><mn>3</mn><mi intent=':unit'>m</mi></mrow><msup><mi intent=':unit'>s</mi><mn>2</mn></msup></mfrac></math>",
            "3 米每 平方秒",
        ),
        (
            "piecewise",
            "SimpleSpeak",
            "Medium",
            "<math><mi>f</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mo>=</mo><mrow><mo>{</mo><mtable><mtr><mtd><msup><mi>x</mi><mn>2</mn></msup><mtext> 当 </mtext><mi>x</mi><mo>&#x2265;</mo><mn>0</mn></mtd></mtr><mtr><mtd><mo>&#x2212;</mo><mi>x</mi><mtext> 当 </mtext><mi>x</mi><mo>&lt;</mo><mn>0</mn></mtd></mtr></mtable></mrow></math>",
            "f x 等于; 2 个分支; 第 1 个分支; x 平方 当 x, 大于等于 0; 第 2 个分支; 负 x 当 x, 小于 0",
        ),
        (
            "system-of-equations",
            "SimpleSpeak",
            "Medium",
            "<math><mtable><mtr><mtd><mi>x</mi><mo>+</mo><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>7</mn></mtd></mtr><mtr><mtd><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>17</mn></mtd></mtr></mtable></math>",
            "2 个方程; 第 1 个方程; x 加 y 等于 7; 第 2 个方程; 2 x 加 3 y; 等于 17",
        ),
        (
            "quadratic-formula-simple",
            "SimpleSpeak",
            "Medium",
            "<math><mi>x</mi><mo>=</mo><mfrac><mrow><mo>&#x2212;</mo><mi>b</mi><mo>&#xb1;</mo><msqrt><mrow><msup><mi>b</mi><mn>2</mn></msup><mo>&#x2212;</mo><mn>4</mn><mi>a</mi><mi>c</mi></mrow></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>",
            "x 等于; 分数, 2 a, 分之, 负 b 加或减; 根号 b 平方 减 4 a c 结束根号; 结束分数",
        ),
        (
            "quadratic-formula-literal",
            "LiteralSpeak",
            "Medium",
            "<math><mi>x</mi><mo>=</mo><mfrac><mrow><mo>&#x2212;</mo><mi>b</mi><mo>&#xb1;</mo><msqrt><mrow><msup><mi>b</mi><mn>2</mn></msup><mo>&#x2212;</mo><mn>4</mn><mi>a</mi><mi>c</mi></mrow></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>",
            "x 等于; 分数, 2 a, 分之, 减 b 加或减; 根号 b 上标 2 结束上标, 减 4 a c, 结束根号; 结束分数",
        ),
        (
            "binomial",
            "SimpleSpeak",
            "Medium",
            "<math><mmultiscripts><mi>C</mi><mi>k</mi><none/><mprescripts/><mi>n</mi><none/></mmultiscripts></math>",
            "n 取 k",
        ),
    ];

    for (name, style, verbosity, mathml, expected) in cases {
        test_prefs(
            "zh",
            style,
            vec![("Verbosity", verbosity)],
            mathml,
            expected,
        )
        .map_err(|error| anyhow::anyhow!("{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn additional_cn_formula_cases_cover_common_notation() -> Result<()> {
    // Exercise complete formulas in both Chinese speech styles, not isolated symbols only.
    let cases = [
        (
            "linear-equation",
            "<math><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mo>=</mo><mn>7</mn></math>",
            "2 x 加 3, 等于 7",
            "2 x 加 3, 等于 7",
        ),
        (
            "quadratic-equation",
            "<math><mi>a</mi><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mi>b</mi><mi>x</mi><mo>+</mo><mi>c</mi><mo>=</mo><mn>0</mn></math>",
            "a x 平方, 加 b x 加 c; 等于 0",
            "a x 平方, 加 b x 加 c; 等于 0",
        ),
        (
            "quadratic-formula",
            "<math><mi>x</mi><mo>=</mo><mfrac><mrow><mo>−</mo><mi>b</mi><mo>±</mo><msqrt><msup><mi>b</mi><mn>2</mn></msup><mo>−</mo><mn>4</mn><mi>a</mi><mi>c</mi></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac></math>",
            "x 等于; 分数, 2 a, 分之, 负 b 加或减; 根号 b 平方 减 4 a c 结束根号; 结束分数",
            "x 等于; 分数，分子为; 根号 b 平方 减 4 a c; 分母为 2 a",
        ),
        (
            "inequality-chain",
            "<math><mrow><mo>−</mo><mn>1</mn><mo>&lt;</mo><mi>x</mi><mo>≤</mo><mn>3</mn></mrow></math>",
            "负 1 小于 x 小于等于 3",
            "负 1 小于 x 小于等于 3",
        ),
        (
            "absolute-inequality",
            "<math><mrow><mo>|</mo><mi>x</mi><mo>−</mo><mn>2</mn><mo>|</mo><mo>&lt;</mo><mn>5</mn></mrow></math>",
            "x 减 2 的绝对值, 小于 5",
            "x 减 2 的绝对值; 小于 5",
        ),
        (
            "rational-expression",
            "<math><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>x</mi><mo>−</mo><mn>1</mn></mrow></mfrac></math>",
            "分数, x 减 1, 分之, x 加 1, 结束分数",
            "分数，分子为; x 加 1; 分母为 x 减 1",
        ),
        (
            "exponent-law",
            "<math><msup><mi>a</mi><mi>m</mi></msup><mo>×</mo><msup><mi>a</mi><mi>n</mi></msup><mo>=</mo><msup><mi>a</mi><mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow></msup></math>",
            "a 的 m 次方 乘 a 的 n 次方; 等于 a 的 m 加 n 次方",
            "a 的 m 次方 乘 a 的 n 次方; 等于 a 的 m 加 n 次方",
        ),
        (
            "cube-root",
            "<math><mroot><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mn>3</mn></mroot></math>",
            "x 加 1 的 立方根, 结束根号",
            "x 加 1 的 立方根",
        ),
        (
            "double-angle",
            "<math><mi>sin</mi><mo>⁡</mo><mrow><mo>(</mo><mn>2</mn><mi>x</mi><mo>)</mo></mrow><mo>=</mo><mn>2</mn><mi>sin</mi><mo>⁡</mo><mi>x</mi><mi>cos</mi><mo>⁡</mo><mi>x</mi></math>",
            "正弦 2 x, 等于, 2 正弦 x 余弦 x",
            "正弦 2 x, 等于, 2 正弦 x 余弦 x",
        ),
        (
            "binomial-coefficient",
            "<math><mrow><mo>(</mo><mfrac linethickness='0em'><mi>n</mi><mi>k</mi></mfrac><mo>)</mo></mrow></math>",
            "n 取 k",
            "n 取 k",
        ),
        (
            "sequence-term",
            "<math><msub><mi>a</mi><mi>n</mi></msub><mo>=</mo><msub><mi>a</mi><mn>1</mn></msub><mo>+</mo><mrow><mo>(</mo><mi>n</mi><mo>−</mo><mn>1</mn><mo>)</mo></mrow><mi>d</mi></math>",
            "a 下标 n, 等于; a 下标 1, 加, 左括号 n 减 1 右括号; 乘 d",
            "a 下标 n, 等于; a 下标 1, 加, 左括号 n 减 1 右括号; 乘 d",
        ),
        (
            "arithmetic-series",
            "<math><msub><mi>S</mi><mi>n</mi></msub><mo>=</mo><mfrac><mrow><mi>n</mi><mo>(</mo><msub><mi>a</mi><mn>1</mn></msub><mo>+</mo><msub><mi>a</mi><mi>n</mi></msub><mo>)</mo></mrow><mn>2</mn></mfrac></math>",
            "大写 s 下标 n; 等于; 分数, 2 分之, n 乘; 左括号, a 下标 1, 加 a 下标 n; 右括号, 结束分数",
            "大写 s 下标 n; 等于; 分数，分子为; n 乘; 左括号, a 下标 1, 加 a 下标 n; 右括号; 分母为 2",
        ),
        (
            "geometric-sum",
            "<math><munderover><mo>∑</mo><mrow><mi>k</mi><mo>=</mo><mn>0</mn></mrow><mi>n</mi></munderover><msup><mi>r</mi><mi>k</mi></msup></math>",
            "求和 从 k 等于 0 到 n, r 的 k 次方",
            "求和 从 k 等于 0 到 n, r 的 k 次方",
        ),
        (
            "definite-integral",
            "<math><msubsup><mo>∫</mo><mn>0</mn><mn>1</mn></msubsup><msup><mi>x</mi><mn>2</mn></msup><mi>d</mi><mi>x</mi></math>",
            "积分 从 0 到 1, x 平方 d x",
            "积分 从 0 到 1, x 平方 d x",
        ),
        (
            "derivative-equation",
            "<math><mfrac><mrow><mo>d</mo><mi>y</mi></mrow><mrow><mo>d</mo><mi>x</mi></mrow></mfrac><mo>=</mo><mn>3</mn><msup><mi>x</mi><mn>2</mn></msup></math>",
            "分数, d x, 分之, d y, 结束分数; 等于 3 x 平方",
            "分数，分子为; d y; 分母为 d x; 等于 3 x 平方",
        ),
        (
            "limit-sine",
            "<math><munder><mo>lim</mo><mrow><mi>x</mi><mo>→</mo><mn>0</mn></mrow></munder><mfrac><mrow><mi>sin</mi><mo>⁡</mo><mi>x</mi></mrow><mi>x</mi></mfrac></math>",
            "极限，当 x 趋于 0; 分数, x 分之, 正弦 x, 结束分数",
            "极限，当 x 趋于 0; x 分之 正弦 x",
        ),
        (
            "differential-equation",
            "<math><mfrac><mrow><mo>d</mo><mi>y</mi></mrow><mrow><mo>d</mo><mi>x</mi></mrow></mfrac><mo>=</mo><mi>k</mi><mi>y</mi></math>",
            "分数, d x, 分之, d y, 结束分数; 等于 k y",
            "分数，分子为; d y; 分母为 d x; 等于 k y",
        ),
        (
            "matrix-two-by-two",
            "<math><mrow><mo>[</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 2 矩阵; 第 1 行; a, b; 第 2 行; c, d",
            "2 乘 2 矩阵; 第 1 行; a, b; 第 2 行; c, d",
        ),
        (
            "determinant-formula",
            "<math><mrow><mo>|</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>|</mo></mrow><mo>=</mo><mi>a</mi><mi>d</mi><mo>−</mo><mi>b</mi><mi>c</mi></math>",
            "2 乘 2 行列式; 第 1 行; a, b; 第 2 行; c, d; 等于, a d 减 b c",
            "2 乘 2 行列式; 第 1 行; a, b; 第 2 行; c, d; 等于, a d 减 b c",
        ),
        (
            "transpose-notation",
            "<math><msup><mi>A</mi><mi>T</mi></msup></math>",
            "大写 a 的转置",
            "大写 a 的转置",
        ),
        (
            "eigen-equation",
            "<math><mi>A</mi><mi>v</mi><mo>=</mo><mi>λ</mi><mi>v</mi></math>",
            "大写 a v, 等于 lambda v",
            "大写 a v, 等于 lambda v",
        ),
        (
            "distance-formula",
            "<math><mi>d</mi><mo>=</mo><msqrt><msup><mrow><mo>(</mo><msub><mi>x</mi><mn>2</mn></msub><mo>−</mo><msub><mi>x</mi><mn>1</mn></msub><mo>)</mo></mrow><mn>2</mn></msup><mo>+</mo><msup><mrow><mo>(</mo><msub><mi>y</mi><mn>2</mn></msub><mo>−</mo><msub><mi>y</mi><mn>1</mn></msub><mo>)</mo></mrow><mn>2</mn></msup></msqrt></math>",
            "d 等于; 根号 左括号, x 下标 2, 减 x 下标 1; 右括号 平方; 加; 左括号, y 下标 2, 减 y 下标 1; 右括号 平方 结束根号",
            "d 等于; 根号 左括号, x 下标 2, 减 x 下标 1; 右括号 平方; 加; 左括号, y 下标 2, 减 y 下标 1; 右括号 平方",
        ),
        (
            "circle-equation",
            "<math><msup><mrow><mo>(</mo><mi>x</mi><mo>−</mo><mi>h</mi><mo>)</mo></mrow><mn>2</mn></msup><mo>+</mo><msup><mrow><mo>(</mo><mi>y</mi><mo>−</mo><mi>k</mi><mo>)</mo></mrow><mn>2</mn></msup><mo>=</mo><msup><mi>r</mi><mn>2</mn></msup></math>",
            "左括号 x 减 h 右括号 平方; 加, 左括号 y 减 k 右括号 平方; 等于 r 平方",
            "左括号 x 减 h 右括号 平方; 加, 左括号 y 减 k 右括号 平方; 等于 r 平方",
        ),
        (
            "pythagorean-theorem",
            "<math><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup><mo>=</mo><msup><mi>c</mi><mn>2</mn></msup></math>",
            "a 平方 加 b 平方, 等于 c 平方",
            "a 平方 加 b 平方, 等于 c 平方",
        ),
        (
            "right-angle",
            "<math><mo>∠</mo><mi>A</mi><mi>B</mi><mi>C</mi><mo>=</mo><mn>90</mn><mo>°</mo></math>",
            "角, 大写 a 大写 b 大写 c; 等于 90 度",
            "角, 大写 a 大写 b 大写 c; 等于 90 度",
        ),
        (
            "set-union-intersection",
            "<math><mi>A</mi><mo>∪</mo><mi>B</mi><mo>=</mo><mi>A</mi><mo>∩</mo><mi>B</mi></math>",
            "大写 a 并 大写 b, 等于, 大写 a 交 大写 b",
            "大写 a 并 大写 b, 等于, 大写 a 交 大写 b",
        ),
        (
            "subset-chain",
            "<math><mi>A</mi><mo>⊂</mo><mi>B</mi><mo>⊆</mo><mi>C</mi></math>",
            "大写 a 子集, 大写 b 子集或等于 大写 c",
            "大写 a 子集, 大写 b 子集或等于 大写 c",
        ),
        (
            "interval",
            "<math><mrow><mo>[</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>)</mo></mrow></math>",
            "左闭右开区间 0 到 1",
            "从 0 到 1 的左闭右开区间",
        ),
        (
            "universal-inequality",
            "<math><mo>∀</mo><mi>x</mi><mo>∈</mo><mi>ℝ</mi><mo>,</mo><msup><mi>x</mi><mn>2</mn></msup><mo>≥</mo><mn>0</mn></math>",
            "任意 x 属于 实数集; 逗号; x 平方 大于等于 0",
            "任意 x 属于 实数集; 逗号; x 平方 大于等于 0",
        ),
        (
            "complex-number",
            "<math><mi>z</mi><mo>=</mo><mn>3</mn><mo>+</mo><mn>4</mn><mi>i</mi></math>",
            "z 等于 3 加 4 i",
            "z 等于 3 加 4 i",
        ),
        (
            "euler-identity",
            "<math><msup><mi>e</mi><mrow><mi>i</mi><mi>π</mi></mrow></msup><mo>+</mo><mn>1</mn><mo>=</mo><mn>0</mn></math>",
            "e 的 i pi 次方, 加 1; 等于 0",
            "e 的 i pi 次方, 加 1; 等于 0",
        ),
        (
            "speed-unit",
            "<math><mfrac><mrow><mn>72</mn><mi intent=':unit'>km</mi></mrow><mi intent=':unit'>h</mi></mfrac></math>",
            "72 千米每 小时",
            "72 千米每 小时",
        ),
        (
            "acceleration-unit",
            "<math><mfrac><mrow><mn>9.8</mn><mi intent=':unit'>m</mi></mrow><msup><mi intent=':unit'>s</mi><mn>2</mn></msup></mfrac></math>",
            "9.8 米每 平方秒",
            "9.8 米每 平方秒",
        ),
        (
            "temperature-unit",
            "<math><mn>20</mn><mi intent=':unit'>°C</mi></math>",
            "20 摄氏度",
            "20 摄氏度",
        ),
        (
            "chemical-water",
            "<math><mrow data-chem-formula='3'><msub><mi mathvariant='normal' data-chem-element='1'>H</mi><mn>2</mn></msub><mi mathvariant='normal' data-chem-element='1'>O</mi></mrow></math>",
            "大写 h, 下标 2, 大写 o",
            "大写 h, 下标 2, 大写 o",
        ),
        (
            "scientific-notation",
            "<math><mn>6.02</mn><mo>×</mo><msup><mn>10</mn><mn>23</mn></msup></math>",
            "6.02 乘 10 的 23 次方",
            "6.02 乘 10 的 23 次方",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn additional_cn_intent_cases_cover_named_terms() -> Result<()> {
    // Explicit intents provide deterministic checks for terms that presentation markup can hide.
    let cases = [
        (
            "conditional-probability",
            "<math><mrow intent='conditional-probability($A,$B)'><mi arg='A'>A</mi><mi arg='B'>B</mi></mrow></math>",
            "在 大写 b 条件下 大写 a 的概率",
        ),
        (
            "gcd",
            "<math><mrow intent='greatest-common-divisor($a,$b)'><mi arg='a'>12</mi><mi arg='b'>18</mi></mrow></math>",
            "12 与 18 的最大公约数",
        ),
        (
            "lcm",
            "<math><mrow intent='least-common-multiple($a,$b)'><mi arg='a'>4</mi><mi arg='b'>6</mi></mrow></math>",
            "4 与 6 的最小公倍数",
        ),
        (
            "quotient",
            "<math><mrow intent='quotient($a,$b)'><mi arg='a'>7</mi><mi arg='b'>3</mi></mrow></math>",
            "7 除以 3 的商",
        ),
        (
            "remainder",
            "<math><mrow intent='remainder($a,$b)'><mi arg='a'>7</mi><mi arg='b'>3</mi></mrow></math>",
            "7 除以 3 的余数",
        ),
        (
            "mean",
            "<math><mrow intent='mean($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的平均值",
        ),
        (
            "variance",
            "<math><mrow intent='variance($x)'><mi arg='x'>x</mi></mrow></math>",
            "x 的方差",
        ),
        (
            "complex-conjugate",
            "<math><mrow intent='complex-conjugate($z)'><mi arg='z'>z</mi></mrow></math>",
            "z 的共轭复数",
        ),
        (
            "real-part",
            "<math><mrow intent='real-part($z)'><mi arg='z'>z</mi></mrow></math>",
            "z 的实部",
        ),
        (
            "dot-product",
            "<math><mrow intent='dot-product($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
            "a 与 b 的数量积",
        ),
        (
            "cross-product",
            "<math><mrow intent='cross-product($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
            "a 与 b 的叉积",
        ),
        (
            "line-segment",
            "<math><mrow intent='line-segment($A,$B)'><mi arg='A'>A</mi><mi arg='B'>B</mi></mrow></math>",
            "线段 大写 a 大写 b",
        ),
        (
            "cartesian-coordinate",
            "<math><mrow intent='cartesian-coordinate($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>",
            "直角坐标 x 逗号, y",
        ),
        (
            "trace",
            "<math><mrow intent='trace($A)'><mi arg='A'>A</mi></mrow></math>",
            "大写 a 的迹",
        ),
        (
            "kernel",
            "<math><mrow intent='kernel($T)'><mi arg='T'>T</mi></mrow></math>",
            "大写 t 的核",
        ),
        (
            "arcsine",
            "<math><mrow intent='arcsine($x)'><mi arg='x'>x</mi></mrow></math>",
            "反正弦 x",
        ),
        (
            "hyperbolic-sine",
            "<math><mi>sinh</mi><mo>⁡</mo><mi>x</mi></math>",
            "双曲正弦 x",
        ),
    ];

    for (name, mathml, expected) in cases {
        for style in ["SimpleSpeak", "ClearSpeak"] {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }
    Ok(())
}

#[test]
fn additional_cn_edge_formulas_cover_uncommon_structure() -> Result<()> {
    // These formulas exercise structural branches that are easy to mis-order in Chinese speech.
    let cases = [
        (
            "polar-coordinate",
            "<math><mrow intent='polar-coordinate($x,$y)'><mi arg='x'>r</mi><mi arg='y'>θ</mi></mrow></math>",
            "极坐标 r 逗号, theta",
            "极坐标 r 逗号, theta",
        ),
        (
            "spherical-coordinate",
            "<math><mrow intent='spherical-coordinate($x,$y,$z)'><mi arg='x'>r</mi><mi arg='y'>θ</mi><mi arg='z'>φ</mi></mrow></math>",
            "球坐标 r 逗号, theta 逗号, phi",
            "球坐标 r 逗号, theta 逗号, phi",
        ),
        (
            "coordinate",
            "<math><mrow intent='coordinate($x,$y,$z)'><mi arg='x'>x</mi><mi arg='y'>y</mi><mi arg='z'>z</mi></mrow></math>",
            "坐标 x 逗号, y 逗号, z",
            "坐标 x 逗号, y 逗号, z",
        ),
        (
            "open-interval-to-infinity",
            "<math><mrow><mo>(</mo><mi>c</mi><mo>,</mo><mo>∞</mo><mo>)</mo></mrow></math>",
            "从 c 到无穷大的区间, 不包括端点 c",
            "从 c 到无穷大的区间, 不包括端点 c",
        ),
        (
            "closed-open-interval-to-infinity",
            "<math><mrow><mo>[</mo><mi>c</mi><mo>,</mo><mo>+</mo><mo>∞</mo><mo>)</mo></mrow></math>",
            "从 c 到正无穷大的区间, 包括端点 c",
            "从 c 到正无穷大的区间, 包括端点 c",
        ),
        (
            "negative-infinity-to-open",
            "<math><mrow><mo>(</mo><mo>−</mo><mo>∞</mo><mo>,</mo><mi>d</mi><mo>)</mo></mrow></math>",
            "从负无穷大到 d 的区间, 不包括端点 d",
            "从负无穷大到 d 的区间, 不包括端点 d",
        ),
        (
            "negative-infinity-to-closed",
            "<math><mrow><mo>(</mo><mo>−</mo><mo>∞</mo><mo>,</mo><mi>d</mi><mo>]</mo></mrow></math>",
            "从负无穷大到 d 的区间, 包括端点 d",
            "从负无穷大到 d 的区间, 包括端点 d",
        ),
        (
            "whole-real-line",
            "<math><mrow><mo>(</mo><mo>−</mo><mo>∞</mo><mo>,</mo><mo>∞</mo><mo>)</mo></mrow></math>",
            "实数集",
            "实数集",
        ),
        (
            "whole-real-line-explicit-positive-infinity",
            "<math><mrow><mo>(</mo><mo>−</mo><mo>∞</mo><mo>,</mo><mo>+</mo><mo>∞</mo><mo>)</mo></mrow></math>",
            "实数集",
            "实数集",
        ),
        (
            "arccosine",
            "<math><mrow intent='arccosine($x)'><mi arg='x'>x</mi></mrow></math>",
            "反余弦 x",
            "反余弦 x",
        ),
        (
            "inverse-hyperbolic-tangent",
            "<math><mrow intent='arc-hyperbolic-tangent($x)'><mi arg='x'>x</mi></mrow></math>",
            "反双曲正切 x",
            "反双曲正切 x",
        ),
        (
            "evaluated-at",
            "<math><mrow intent='evaluated-at($x,$y)'><mi arg='x'>f</mi><mi arg='y'>2</mi></mrow></math>",
            "f 取值于 2",
            "f 取值于 2",
        ),
        (
            "power-intent",
            "<math><mrow intent='power($x,$y)'><mi arg='x'>x</mi><mi arg='y'>n</mi></mrow></math>",
            "x 的 n 次方",
            "x 的 n 次方",
        ),
        (
            "mixed-partial-derivative",
            "<math><mfrac><mrow><msup><mo>∂</mo><mn>2</mn></msup><mi>f</mi></mrow><mrow><mo>∂</mo><mi>x</mi><mo>∂</mo><mi>y</mi></mrow></mfrac></math>",
            "分数, 偏导数 x 偏导数 y, 分之, 偏导数 平方 f, 结束分数",
            "分数，分子为; 偏导数 平方 f; 分母为 偏导数 x 偏导数 y",
        ),
        (
            "third-derivative",
            "<math><mfrac><mrow><msup><mo>d</mo><mn>3</mn></msup><mi>y</mi></mrow><mrow><mi>d</mi><msup><mi>x</mi><mn>3</mn></msup></mrow></mfrac></math>",
            "分数, d x 立方, 分之, d 立方 y, 结束分数",
            "分数，分子为; d 立方 y; 分母为 d x 立方",
        ),
        (
            "derivative-of-parenthesized-function",
            "<math><mfrac><mrow><mi>d</mi><mrow><mo>(</mo><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>)</mo></mrow></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>",
            "分数, d x, 分之, d, 左括号 f x 右括号, 结束分数",
            "分数，分子为; d, 左括号 f x 右括号; 分母为 d x",
        ),
        (
            "fractional-exponent",
            "<math><msup><mi>x</mi><mfrac><mn>1</mn><mn>2</mn></mfrac></msup></math>",
            "x 的 2 分之 1 次方",
            "x 的 2 分之 1 次方",
        ),
        (
            "fourth-root",
            "<math><mroot><mi>x</mi><mn>4</mn></mroot></math>",
            "x 的 4 次方根",
            "x 的 4 次方根",
        ),
        (
            "negative-cube-radicand",
            "<math><mroot><mrow><mo>−</mo><mn>8</mn></mrow><mn>3</mn></mroot></math>",
            "负 8 的 立方根, 结束根号",
            "负 8 的 立方根",
        ),
        (
            "subscript-and-superscript",
            "<math><msubsup><mi>x</mi><mi>i</mi><mi>j</mi></msubsup></math>",
            "x 下标 i, 的 j 次方",
            "x 下标 i, 的 j 次方",
        ),
        (
            "tensor-postscripts",
            "<math><mmultiscripts><mi>R</mi><mi>i</mi><none/><none/><mi>j</mi><mi>k</mi><none/></mmultiscripts></math>",
            "大写 r 有 3 组后置上下标, 下标 i 上标 j 下标 k",
            "大写 r 有 3 组后置上下标, 下标 i 上标 j 下标 k",
        ),
        (
            "augmented-matrix",
            "<math><mrow><mo>[</mo><mtable columnlines='none solid'><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd><mtd><mn>6</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 3 增广矩阵; 第 1 行; 1, 2, 列分隔线, 3; 第 2 行; 4, 5, 列分隔线, 6",
            "2 乘 3 增广矩阵; 第 1 行; 1, 2, 列分隔线, 3; 第 2 行; 4, 5, 列分隔线, 6",
        ),
        (
            "matrix-product",
            "<math><mrow><mo>(</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>)</mo><mo>×</mo><mo>(</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mtr><mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>)</mo></mrow></math>",
            "2 乘 2 矩阵; 第 1 行; 1, 2; 第 2 行; 3, 4; 乘, 2 乘 2 矩阵; 第 1 行; a, b; 第 2 行; c, d",
            "2 乘 2 矩阵; 第 1 行; 1, 2; 第 2 行; 3, 4; 乘, 2 乘 2 矩阵; 第 1 行; a, b; 第 2 行; c, d",
        ),
        (
            "three-branch-piecewise",
            "<math><mrow><mo>{</mo><mtable><mtr><mtd><mrow><msup><mi>x</mi><mn>2</mn></msup><mtext> 当 </mtext><mi>x</mi><mo>&lt;</mo><mn>0</mn></mrow></mtd></mtr><mtr><mtd><mrow><mn>0</mn><mtext> 当 </mtext><mi>x</mi><mo>=</mo><mn>0</mn></mrow></mtd></mtr><mtr><mtd><mrow><mo>−</mo><mi>x</mi><mtext> 当 </mtext><mi>x</mi><mo>&gt;</mo><mn>0</mn></mrow></mtd></mtr></mtable></mrow></math>",
            "3 个分支; 第 1 个分支; x 平方 当 x, 小于 0; 第 2 个分支; 0 当 x, 等于 0; 第 3 个分支; 负 x 当 x, 大于 0",
            "3 个分支; 第 1 个分支; x 平方 当 x, 小于 0; 第 2 个分支; 0 当 x, 等于 0; 第 3 个分支; 负 x 当 x, 大于 0",
        ),
        (
            "piecewise-without-condition",
            "<math><mrow><mo>{</mo><mtable><mtr><mtd><mrow><mi>x</mi><mtext> 当 </mtext><mi>x</mi><mo>&lt;</mo><mn>0</mn></mrow></mtd></mtr><mtr><mtd><mo>−</mo><mi>x</mi></mtd></mtr></mtable></mrow></math>",
            "2 个分支; 第 1 个分支; x 当 x, 小于 0; 第 2 个分支; 负 x",
            "2 个分支; 第 1 个分支; x 当 x, 小于 0; 第 2 个分支; 负 x",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }

    // ClearSpeak intentionally omits the closing marker for a nested radical; keep the
    // regression assertion on the complete SimpleSpeak reading instead.
    test(
        "zh",
        "SimpleSpeak",
        "<math><msqrt><mrow><mn>1</mn><mo>+</mo><msqrt><mi>x</mi></msqrt></mrow></msqrt></math>",
        "根号 1 加 根号 x, 结束根号",
    )?;
    Ok(())
}

#[test]
fn additional_cn_semantic_formulas_cover_probability_chemistry_and_algebra() -> Result<()> {
    // Cover named semantic branches and notation where operand order changes the meaning.
    let cases = [
        (
            "complement-event",
            "<math><mrow><mi>P</mi><mo>(</mo><mover><mi>A</mi><mo>¯</mo></mover><mo>)</mo><mo>=</mo><mn>1</mn><mo>−</mo><mi>P</mi><mo>(</mo><mi>A</mi><mo>)</mo></mrow></math>",
            "大写 a 上横线, 的概率, 等于, 1 减 大写 a 的概率",
            "大写 a 上横线, 的概率, 等于, 1 减 大写 a 的概率",
        ),
        (
            "ordered-pair",
            "<math><mrow intent='ordered-pair($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>",
            "有序对 x 和 y",
            "有序对 x 和 y",
        ),
        (
            "cartesian-product",
            "<math><mrow intent='cartesian-product($x,$y)'><mi arg='x'>A</mi><mi arg='y'>B</mi></mrow></math>",
            "大写 a 笛卡尔积 大写 b",
            "大写 a 笛卡尔积 大写 b",
        ),
        (
            "direct-product",
            "<math><mrow intent='direct-product($x,$y)'><mi arg='x'>G</mi><mi arg='y'>H</mi></mrow></math>",
            "大写 g 直积 大写 h",
            "大写 g 直积 大写 h",
        ),
        (
            "inner-product",
            "<math><mrow intent='inner-product($x,$y)'><mi arg='x'>u</mi><mi arg='y'>v</mi></mrow></math>",
            "u 内积 v",
            "u 内积 v",
        ),
        (
            "outer-product",
            "<math><mrow intent='outer-product($x,$y)'><mi arg='x'>u</mi><mi arg='y'>v</mi></mrow></math>",
            "u 外积 v",
            "u 外积 v",
        ),
        (
            "rate",
            "<math><mrow intent='rate($x,$y)'><mi arg='x'>d</mi><mi arg='y'>t</mi></mrow></math>",
            "d 每 t",
            "d 每 t",
        ),
        (
            "constraint",
            "<math><mrow intent='constraint($x,$y)'><mi arg='x'>x</mi><mrow arg='y'><mi>x</mi><mo>&gt;</mo><mn>0</mn></mrow></mrow></math>",
            "x 条件为 x 大于 0",
            "x 条件为 x 大于 0",
        ),
        (
            "translation",
            "<math><mrow intent='translation($x,$y)'><mi arg='x'>f</mi><mi arg='y'>g</mi></mrow></math>",
            "平移 f 逗号, g",
            "平移 f 逗号, g",
        ),
        (
            "pochhammer-symbol",
            "<math><mrow intent='pochhammer($x,$y)'><mi arg='x'>x</mi><mi arg='y'>n</mi></mrow></math>",
            "升阶乘 x 逗号, n",
            "升阶乘 x 逗号, n",
        ),
        (
            "directed-line-segment",
            "<math><mrow intent='directed-line-segment($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "有向线段 大写 a 大写 b",
            "有向线段 大写 a 大写 b",
        ),
        (
            "line",
            "<math><mrow intent='line($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "直线 大写 a 大写 b",
            "直线 大写 a 大写 b",
        ),
        (
            "point",
            "<math><mrow intent='point($x,$y,$z)'><mi arg='x'>A</mi><mi arg='y'>B</mi><mi arg='z'>C</mi></mrow></math>",
            "点 大写 a 大写 b 大写 c",
            "点 大写 a 大写 b 大写 c",
        ),
        (
            "perpendicular",
            "<math><mrow intent='perpendicular($a,$b)'><mi arg='a'>l</mi><mi arg='b'>m</mi></mrow></math>",
            "l 垂直于 m",
            "l 垂直于 m",
        ),
        (
            "proportional",
            "<math><mrow intent='proportional($a,$b)'><mi arg='a'>y</mi><mi arg='b'>x</mi></mrow></math>",
            "y 正比于 x",
            "y 正比于 x",
        ),
        (
            "xor",
            "<math><mrow intent='xor($a,$b)'><mi arg='a'>p</mi><mi arg='b'>q</mi></mrow></math>",
            "p 异或 q",
            "p 异或 q",
        ),
        (
            "logical-not",
            "<math><mrow intent='not($x)'><mi arg='x'>p</mi></mrow></math>",
            "非 p",
            "非 p",
        ),
        (
            "there-does-not-exist",
            "<math><mrow intent='there-does-not-exist($x)'><mi arg='x'>x</mi></mrow></math>",
            "不存在 x",
            "不存在 x",
        ),
        (
            "evaluates-to",
            "<math><mrow intent='evaluates-to($x,$y)'><mi arg='x'>f</mi><mi arg='y'>3</mi></mrow></math>",
            "f 结果为 3",
            "f 结果为 3",
        ),
        (
            "approximately",
            "<math><mrow intent='approximately($a,$b)'><mi arg='a'>π</mi><mi arg='b'>3.14</mi></mrow></math>",
            "pi 约等于 3.14",
            "pi 约等于 3.14",
        ),
        (
            "identically-equals",
            "<math><mrow intent='identically-equals($a,$b)'><mi arg='a'>f</mi><mi arg='b'>g</mi></mrow></math>",
            "f 恒等于 g",
            "f 恒等于 g",
        ),
        (
            "golden-ratio",
            "<math><mi intent='golden-ratio'>φ</mi></math>",
            "黄金分割比",
            "黄金分割比",
        ),
        (
            "upper-limit",
            "<math><mrow intent='lim-sup($x)'><mi arg='x'>a</mi></mrow></math>",
            "上极限，当 a",
            "上极限，当 a",
        ),
        (
            "lower-limit",
            "<math><mrow intent='lim-inf($x)'><mi arg='x'>a</mi></mrow></math>",
            "下极限，当 a",
            "下极限，当 a",
        ),
        (
            "unit-vector",
            "<math><mrow intent='unit-vector($x)'><mi arg='x'>v</mi></mrow></math>",
            "单位向量 v",
            "单位向量 v",
        ),
        (
            "identity-matrix",
            "<math><mi intent='identity-matrix'>I</mi></math>",
            "单位矩阵",
            "单位矩阵",
        ),
        (
            "diagonal-matrix",
            "<math><mrow><mo>[</mo><mtable><mtr><mtd><mn>2</mn></mtd><mtd><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mn>3</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 2 对角矩阵; 第 1 列; 2; 第 2 列; 3",
            "2 乘 2 对角矩阵; 第 1 列; 2; 第 2 列; 3",
        ),
        (
            "matrix-row-separator",
            "<math><mrow><mo>[</mo><mtable rowlines='solid'><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 2 矩阵; 第 1 行; 1, 2, 行分隔线; 第 2 行; 3, 4",
            "2 乘 2 矩阵; 第 1 行; 1, 2, 行分隔线; 第 2 行; 3, 4",
        ),
        (
            "charged-ion",
            "<math><msup><mrow><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub></mrow><mrow><mn>2</mn><mo>−</mo></mrow></msup></math>",
            "大写 s, 大写 o, 下标 4 上标 2 负",
            "大写 s, 大写 o, 下标 4 上标 2 负",
        ),
        (
            "chemical-states",
            "<math><mrow><mi>Na</mi><mrow><mo>(</mo><mi>aq</mi><mo>)</mo></mrow><mo>+</mo><mi>Cl</mi><mrow><mo>(</mo><mi>s</mi><mo>)</mo></mrow></mrow></math>",
            "大写 n a, 水溶液; 加, 大写 c l, 固体",
            "大写 n a, 水溶液; 加, 大写 c l, 固体",
        ),
        (
            "chemical-single-bond",
            "<math><mrow><mi>C</mi><msub><mi>H</mi><mn>3</mn></msub><mo>−</mo><mi>O</mi><mi>H</mi></mrow></math>",
            "大写 c, 大写 h, 下标 3, 单键 大写 o, 大写 h",
            "大写 c, 大写 h, 下标 3, 单键 大写 o, 大写 h",
        ),
        (
            "isotope-prescripts",
            "<math><mmultiscripts><mtext>C</mtext><mprescripts/><mn>6</mn><mn>14</mn></mmultiscripts></math>",
            "上标 14, 下标 6, 大写 c",
            "上标 14, 下标 6, 大写 c",
        ),
        (
            "inverse-square-unit",
            "<math><mrow><mn>5</mn><msup><mi intent=':unit'>m</mi><mrow><mo>−</mo><mn>2</mn></mrow></msup></mrow></math>",
            "5 米的 负 2 次方",
            "5 米的 负 2 次方",
        ),
        (
            "micro-meter",
            "<math><mn>3</mn><mi intent=':unit'>µm</mi></math>",
            "3 微米",
            "3 微米",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }

    let bayes = "<math><mrow><mi>P</mi><mo>(</mo><mi>A</mi><mo>|</mo><mi>B</mi><mo>)</mo><mo>=</mo><mfrac><mrow><mi>P</mi><mo>(</mo><mi>B</mi><mo>|</mo><mi>A</mi><mo>)</mo><mi>P</mi><mo>(</mo><mi>A</mi><mo>)</mo></mrow><mrow><mi>P</mi><mo>(</mo><mi>B</mi><mo>)</mo></mrow></mfrac></mrow></math>";
    test(
        "zh",
        "SimpleSpeak",
        bayes,
        "在 大写 b 条件下 大写 a 的概率, 等于; 分数, 大写 b 的概率, 分之, 在 大写 a 条件下 大写 b 的概率 乘 大写 a 的概率, 结束分数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        bayes,
        "在 大写 b 条件下 大写 a 的概率, 等于; 分数，分子为; 在 大写 a 条件下 大写 b 的概率 乘 大写 a 的概率; 分母为 大写 b 的概率",
    )?;

    let mean = "<math><mover><mi>x</mi><mo>¯</mo></mover><mo>=</mo><mfrac><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mn>2</mn></mfrac></math>";
    test(
        "zh",
        "SimpleSpeak",
        mean,
        "x 上横线, 等于, 分数, 2 分之, x 加 y, 结束分数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        mean,
        "x 上横线, 等于, 分数，分子为; x 加 y; 分母为 2",
    )?;

    let variance = "<math><mi>σ</mi><mo>²</mo><mo>=</mo><mfrac><mrow><munderover><mo>∑</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msup><mrow><mo>(</mo><mi>x</mi><mo>−</mo><mover><mi>x</mi><mo>¯</mo></mover><mo>)</mo></mrow><mn>2</mn></msup></mrow><mi>n</mi></mfrac></math>";
    test(
        "zh",
        "SimpleSpeak",
        variance,
        "sigma 上标 二; 等于; 分数, n 分之, 求和 从 i 等于 1 到 n; 左括号, x 减 x 上横线; 右括号 平方, 结束分数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        variance,
        "sigma 上标 二; 等于; 分数，分子为; 求和 从 i 等于 1 到 n; 左括号, x 减 x 上横线; 右括号 平方; 分母为 n",
    )?;

    let reaction = "<math><mmultiscripts><mtext>C</mtext><mprescripts/><mn>6</mn><mn>14</mn></mmultiscripts><mo>→</mo><mmultiscripts><mtext>N</mtext><mprescripts/><mn>7</mn><mn>14</mn></mmultiscripts><mo>+</mo><mmultiscripts><mtext>e</mtext><mprescripts/><mrow><mo>−</mo><mn>1</mn></mrow><mn>0</mn></mmultiscripts></math>";
    for style in ["SimpleSpeak", "ClearSpeak"] {
        test_prefs(
            "zh",
            style,
            vec![("Verbosity", "Terse")],
            reaction,
            "14, 6, 大写 c; 形成, 14, 7, 大写 n; 加 0, 负 1, e",
        )
        .map_err(|error| anyhow::anyhow!("{style}/chemical-reaction: {error}"))?;
    }
    Ok(())
}

#[test]
fn additional_cn_calculus_and_operator_cases_cover_high_risk_structure() -> Result<()> {
    // These formulas exercise operator order, limits, and nested operands that are easy to
    // misread when English word order is copied directly into Chinese.
    let cases = [
        (
            "principal-logarithm",
            "<math><mi>Log</mi><mo>⁡</mo><mi>z</mi></math>",
            "主值对数 z",
        ),
        (
            "logarithm-with-natural-base",
            "<math><msub><mi>log</mi><mi>e</mi></msub><mi>x</mi></math>",
            "以 e 为底, x 的对数",
        ),
        (
            "contour-integral",
            "<math><mo>∮</mo><mi>f</mi><mo>⁡</mo><mi>z</mi><mi>d</mi><mi>z</mi></math>",
            "围道积分 f z d z",
        ),
        (
            "surface-integral",
            "<math><mo>∯</mo><mi>f</mi><mo>⁡</mo><mi>x</mi><mi>d</mi><mi>S</mi></math>",
            "曲面积分 f x d 大写 s",
        ),
        (
            "volume-integral",
            "<math><mo>∰</mo><mi>f</mi><mo>⁡</mo><mi>x</mi><mi>d</mi><mi>V</mi></math>",
            "体积分 f x d 大写 v",
        ),
        (
            "triple-integral",
            "<math><mo>∭</mo><mi>f</mi><mo>⁡</mo><mi>x</mi><mi>y</mi><mi>z</mi><mi>d</mi><mi>x</mi><mi>d</mi><mi>y</mi><mi>d</mi><mi>z</mi></math>",
            "三重积分 f xyzdxdydz",
        ),
        (
            "intersection-with-limits",
            "<math><munderover><mo>⋂</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "n 元交集 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "union-with-limits",
            "<math><munderover><mo>⋃</mo><mrow><mi>i</mi><mo>=</mo><mn>1</mn></mrow><mi>n</mi></munderover><msub><mi>A</mi><mi>i</mi></msub></math>",
            "n 元并集 从 i 等于 1 到 n; 大写 a 下标 i",
        ),
        (
            "product-with-limits-and-expression",
            "<math><munderover><mo>∏</mo><mrow><mi>j</mi><mo>=</mo><mn>1</mn></mrow><mi>m</mi></munderover><mrow><mo>(</mo><mn>1</mn><mo>+</mo><msub><mi>x</mi><mi>j</mi></msub><mo>)</mo></mrow></math>",
            "连乘 从 j 等于 1 到 m; 左括号, 1 加 x 下标 j; 右括号",
        ),
        (
            "function-composition",
            "<math><mi>f</mi><mo>∘</mo><mi>g</mi></math>",
            "f 复合 g",
        ),
        (
            "evaluated-at-bar",
            "<math><mrow><mi>f</mi><mo>⁡</mo><mi>x</mi></mrow><msub><mo>|</mo><mn>0</mn></msub></math>",
            "f x 在 0 处的值",
        ),
        (
            "integration-by-parts",
            "<math><mo>∫</mo><mi>u</mi><mi>d</mi><mi>v</mi><mo>=</mo><mi>u</mi><mi>v</mi><mo>−</mo><mo>∫</mo><mi>v</mi><mi>d</mi><mi>u</mi></math>",
            "积分 u d v, 等于, u v 减 积分 v d u",
        ),
        (
            "binomial-expansion",
            "<math><msup><mrow><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></mrow><mn>2</mn></msup><mo>=</mo><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>2</mn><mi>x</mi><mi>y</mi><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></math>",
            "左括号 x 加 y 右括号 平方; 等于, x 平方 加 2 x y, 加 y 平方",
        ),
        (
            "divisibility-pair",
            "<math><mn>3</mn><mo>∣</mo><mn>12</mn><mo>,</mo><mn>5</mn><mo>∤</mo><mn>12</mn></math>",
            "3 整除 12, 逗号, 5 不整除 12",
        ),
        (
            "prime-sequence-term",
            "<math><msub><mi>p</mi><mi>n</mi></msub><mo>≥</mo><mn>2</mn></math>",
            "p 下标 n; 大于等于 2",
        ),
    ];

    for (name, mathml, expected) in cases {
        for style in ["SimpleSpeak", "ClearSpeak"] {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }

    test(
        "zh",
        "SimpleSpeak",
        "<math><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac><mo>=</mo><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>u</mi></mrow></mfrac><mfrac><mrow><mi>d</mi><mi>u</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>",
        "分数, d x, 分之, d y, 结束分数; 等于; 分数, d u, 分之, d y, 结束分数; 分数, d x, 分之, d u, 结束分数",
    )?;
    test(
        "zh",
        "ClearSpeak",
        "<math><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac><mo>=</mo><mfrac><mrow><mi>d</mi><mi>y</mi></mrow><mrow><mi>d</mi><mi>u</mi></mrow></mfrac><mfrac><mrow><mi>d</mi><mi>u</mi></mrow><mrow><mi>d</mi><mi>x</mi></mrow></mfrac></math>",
        "d x 分之 d y, 等于; d u 分之 d y; d x 分之 d u",
    )
}

#[test]
fn additional_cn_probability_linear_algebra_and_geometry_cases() -> Result<()> {
    // These are complete textbook formulas rather than isolated symbols, so operand order and
    // matrix boundaries are checked together.
    let cases = [
        (
            "covariance",
            "<math><mi>Cov</mi><mo>⁡</mo><mrow><mo>(</mo><mi>X</mi><mo>,</mo><mi>Y</mi><mo>)</mo></mrow></math>",
            "协方差, 左括号, 大写 x 逗号, 大写 y; 右括号",
            "协方差, 左括号, 大写 x 逗号, 大写 y; 右括号",
        ),
        (
            "correlation-coefficient",
            "<math><mi>ρ</mi><mo>=</mo><mfrac><mrow><mi>Cov</mi><mo>⁡</mo><mrow><mo>(</mo><mi>X</mi><mo>,</mo><mi>Y</mi><mo>)</mo></mrow></mrow><mrow><msub><mi>σ</mi><mi>X</mi></msub><msub><mi>σ</mi><mi>Y</mi></msub></mrow></mfrac></math>",
            "rho 等于; 分数, sigma 下标 大写 x; sigma 下标 大写 y; 分之, 协方差, 左括号, 大写 x 逗号, 大写 y; 右括号, 结束分数",
            "rho 等于; 分数，分子为; 协方差, 左括号, 大写 x 逗号, 大写 y; 右括号; 分母为 sigma 下标 大写 x; sigma 下标 大写 y",
        ),
        (
            "normal-density",
            "<math><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mfrac><mn>1</mn><mrow><mi>σ</mi><msqrt><mn>2</mn><mi>π</mi></msqrt></mrow></mfrac><msup><mi>e</mi><mrow><mo>−</mo><mfrac><mrow><mo>(</mo><mi>x</mi><mo>−</mo><mi>μ</mi><mo>)</mo></mrow><mn>2</mn></mfrac></mrow></msup></math>",
            "f x 等于; 分数, sigma 乘 根号 2 pi 结束根号; 分之 1, 结束分数; 乘; e 的 负 分数, 2 分之, 左括号 x 减 mu 右括号, 结束分数; 次方",
            "f x 等于; 分数，分子为 1; 分母为 根号 2 pi; 乘; e 的 分数，分子为; 左括号 x 减 mu 右括号; 分母为 2; 次方",
        ),
        (
            "binomial-probability",
            "<math><mi>P</mi><mo>(</mo><mi>X</mi><mo>=</mo><mi>k</mi><mo>)</mo><mo>=</mo><mrow><mo>(</mo><mfrac linethickness='0em'><mi>n</mi><mi>k</mi></mfrac><mo>)</mo></mrow><msup><mi>p</mi><mi>k</mi></msup><msup><mrow><mo>(</mo><mn>1</mn><mo>−</mo><mi>p</mi><mo>)</mo></mrow><mrow><mi>n</mi><mo>−</mo><mi>k</mi></mrow></msup></math>",
            "大写 x 等于 k 的概率, 等于; n 取 k p 的 k 次方 乘, 左括号 1 减 p 右括号 的 n 减 k 次方",
            "大写 x 等于 k 的概率, 等于; n 取 k p 的 k 次方 乘, 左括号 1 减 p 右括号 的 n 减 k 次方",
        ),
        (
            "complex-modulus",
            "<math><mo>|</mo><mi>z</mi><mo>|</mo><mo>=</mo><msqrt><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msup><mi>y</mi><mn>2</mn></msup></msqrt></math>",
            "z 的绝对值 等于, 根号 x 平方 加 y 平方 结束根号",
            "z 的绝对值, 等于, 根号 x 平方 加 y 平方",
        ),
        (
            "polar-complex-form",
            "<math><mi>z</mi><mo>=</mo><mi>r</mi><mrow><mo>(</mo><mi>cos</mi><mo>⁡</mo><mi>θ</mi><mo>+</mo><mi>i</mi><mi>sin</mi><mo>⁡</mo><mi>θ</mi><mo>)</mo></mrow></math>",
            "z 等于; r; 左括号; 余弦 theta, 加, i 正弦 theta; 右括号",
            "z 等于; r; 左括号; 余弦 theta, 加, i 正弦 theta; 右括号",
        ),
        (
            "slope-formula",
            "<math><mi>m</mi><mo>=</mo><mfrac><mrow><msub><mi>y</mi><mn>2</mn></msub><mo>−</mo><msub><mi>y</mi><mn>1</mn></msub></mrow><mrow><msub><mi>x</mi><mn>2</mn></msub><mo>−</mo><msub><mi>x</mi><mn>1</mn></msub></mrow></mfrac></math>",
            "m 等于; 分数, x 下标 2, 减 x 下标 1; 分之, y 下标 2, 减 y 下标 1; 结束分数",
            "m 等于; 分数，分子为; y 下标 2, 减 y 下标 1; 分母为 x 下标 2, 减 x 下标 1",
        ),
        (
            "law-of-cosines",
            "<math><msup><mi>c</mi><mn>2</mn></msup><mo>=</mo><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup><mo>−</mo><mn>2</mn><mi>a</mi><mi>b</mi><mi>cos</mi><mo>⁡</mo><mi>C</mi></math>",
            "c 平方 等于; a 平方 加 b 平方 减, 2 a b 余弦 大写 c",
            "c 平方 等于; a 平方 加 b 平方 减, 2 a b 余弦 大写 c",
        ),
        (
            "triangle-area",
            "<math><mi>S</mi><mo>=</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mi>a</mi><mi>h</mi></math>",
            "大写 s 等于, 2 分之 1 a h",
            "大写 s 等于, 2 分之 1 a h",
        ),
        (
            "inverse-matrix-identity",
            "<math><msup><mi>A</mi><mrow><mo>−</mo><mn>1</mn></mrow></msup><mi>A</mi><mo>=</mo><mi>I</mi></math>",
            "大写 a 的 负 1 次方, 大写 a; 等于 大写 i",
            "大写 a 的 负 1 次方, 大写 a; 等于 大写 i",
        ),
        (
            "trace-function-notation",
            "<math><mi>tr</mi><mo>⁡</mo><mrow><mo>(</mo><mi>A</mi><mo>)</mo></mrow></math>",
            "大写 a 的迹",
            "大写 a 的迹",
        ),
        (
            "explicit-vector",
            "<math><mrow intent='vector($v)'><mover><mi arg='v'>v</mi><mo>→</mo></mover></mrow></math>",
            "向量 v",
            "向量 v",
        ),
        (
            "one-by-one-matrix",
            "<math><mrow><mo>[</mo><mtable><mtr><mtd><mi>a</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "1 乘 1 矩阵 元素为 a",
            "1 乘 1 矩阵 元素为 a",
        ),
        (
            "three-by-three-matrix",
            "<math><mrow><mo>[</mo><mtable><mtr><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd><mtd><mi>c</mi></mtd></mtr><mtr><mtd><mi>d</mi></mtd><mtd><mi>e</mi></mtd><mtd><mi>f</mi></mtd></mtr><mtr><mtd><mi>g</mi></mtd><mtd><mi>h</mi></mtd><mtd><mi>i</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "3 乘 3 矩阵; 第 1 行; a, b, c; 第 2 行; d, e, f; 第 3 行; g, h, i",
            "3 乘 3 矩阵; 第 1 行; a, b, c; 第 2 行; d, e, f; 第 3 行; g, h, i",
        ),
        (
            "matrix-row-and-column-separators",
            "<math><mrow><mo>[</mo><mtable rowlines='solid' columnlines='solid'><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr><mtr><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd><mtd><mn>6</mn></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 3 增广矩阵; 第 1 行; 1, 列分隔线, 2, 列分隔线, 3, 行分隔线; 第 2 行; 4, 列分隔线, 5, 列分隔线, 6",
            "2 乘 3 增广矩阵; 第 1 行; 1, 列分隔线, 2, 列分隔线, 3, 行分隔线; 第 2 行; 4, 列分隔线, 5, 列分隔线, 6",
        ),
        (
            "labeled-matrix",
            "<math><mrow><mo>[</mo><mtable><mlabeledtr><mtd><mi>A</mi></mtd><mtd><mi>a</mi></mtd><mtd><mi>b</mi></mtd></mlabeledtr><mtr><mtd><mi>B</mi></mtd><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr></mtable><mo>]</mo></mrow></math>",
            "2 乘 3 矩阵; 第 1 行 带有标签 大写 a; a, b; 第 2 行; 大写 b, c, d",
            "2 乘 3 矩阵; 第 1 行 标签为 大写 a; a, b; 第 2 行; 大写 b, c, d",
        ),
        (
            "non-simple-binomial-coefficient",
            "<math><mrow><mo>(</mo><mfrac linethickness='0em'><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>k</mi><mo>−</mo><mn>1</mn></mrow></mfrac><mo>)</mo></mrow></math>",
            "二项式系数 n 加 1 取 k 减 1 结束二项式系数",
            "二项式系数 n 加 1 取 k 减 1 结束二项式系数",
        ),
        (
            "nested-power",
            "<math><msup><mrow><mo>(</mo><msup><mi>x</mi><mn>2</mn></msup><mo>)</mo></mrow><mn>3</mn></msup></math>",
            "左括号 x 平方 右括号 立方",
            "左括号 x 平方 右括号 立方",
        ),
        (
            "decimal-power",
            "<math><msup><mi>x</mi><mn>2.5</mn></msup></math>",
            "x 的 2.5 次方",
            "x 的 2.5 次方",
        ),
        (
            "zero-power",
            "<math><msup><mi>x</mi><mn>0</mn></msup></math>",
            "x 的 0 次方",
            "x 的 0 次方",
        ),
        (
            "fraction-under-radical",
            "<math><msqrt><mfrac><mn>1</mn><mn>2</mn></mfrac></msqrt></math>",
            "根号 2 分之 1 结束根号",
            "根号 2 分之 1",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn additional_cn_units_chemistry_and_adornment_cases() -> Result<()> {
    // Units and chemical notation combine ordinary symbols with special contextual rules.
    let cases = [
        (
            "radian-unit",
            "<math><mfrac><mi>π</mi><mn>2</mn></mfrac><mi intent=':unit'>rad</mi></math>",
            "2 分之 pi, 弧度",
            "2 分之 pi 弧度",
        ),
        (
            "si-prefix",
            "<math><mn>5</mn><mi intent=':unit'>kN</mi></math>",
            "5 千牛顿",
            "5 千牛顿",
        ),
        (
            "density-unit",
            "<math><mfrac><mi intent=':unit'>kg</mi><msup><mi intent=':unit'>m</mi><mn>3</mn></msup></mfrac></math>",
            "千克每 立方米",
            "千克每 立方米",
        ),
        (
            "molarity",
            "<math><mfrac><mrow><mn>0.5</mn><mi intent=':unit'>mol</mi></mrow><mi intent=':unit'>L</mi></mfrac></math>",
            "0.5 摩尔每 升",
            "0.5 摩尔每 升",
        ),
        (
            "fahrenheit-unit",
            "<math><mn>68</mn><mi intent=':unit'>°F</mi></math>",
            "68 华氏度",
            "68 华氏度",
        ),
        (
            "ohm-unit",
            "<math><mn>10</mn><mi intent=':unit'>Ω</mi></math>",
            "10 欧姆",
            "10 欧姆",
        ),
        (
            "aluminum-sulfate",
            "<math><mrow><msub><mi mathvariant='normal'>Al</mi><mn>2</mn></msub><msub><mrow><mo>(</mo><mi mathvariant='normal'>S</mi><msub><mi mathvariant='normal'>O</mi><mn>4</mn></msub><mo>)</mo></mrow><mn>3</mn></msub></mrow></math>",
            "大写 a l, 下标 2; 左括号, 大写 s, 大写 o, 下标 4; 右括号 下标 3",
            "大写 a l, 下标 2; 左括号, 大写 s, 大写 o, 下标 4; 右括号 下标 3",
        ),
        (
            "precipitation-reaction",
            "<math><mrow><mi mathvariant='normal'>Ag</mi><mo>+</mo><mi mathvariant='normal'>Cl</mi><mo>→</mo><mi mathvariant='normal'>AgCl</mi><mo>↓</mo></mrow></math>",
            "大写 a g, 加 大写 c l; 反应形成, 大写 a g, 大写 c l; 向下箭头",
            "大写 a g, 加 大写 c l; 反应形成, 大写 a g, 大写 c l; 向下箭头",
        ),
        (
            "reaction-condition",
            "<math><mrow><mi mathvariant='normal'>A</mi><mover><mo>→</mo><mi>Δ</mi></mover><mi mathvariant='normal'>B</mi></mrow></math>",
            "大写 a, 向右箭头 上方有 大写 delta, 大写 b",
            "大写 a, 向右箭头 上方有 大写 delta, 大写 b",
        ),
        (
            "hydrate-dot",
            "<math><mrow><mi mathvariant='normal'>Cu</mi><msub><mi mathvariant='normal'>SO</mi><mn>4</mn></msub><mo>·</mo><mn>5</mn><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub><mi mathvariant='normal'>O</mi></mrow></math>",
            "大写 c u, 大写 s, 大写 o, 下标 4; 点, 5, 大写 h, 下标 2, 大写 o",
            "大写 c u, 大写 s, 大写 o, 下标 4; 乘, 5, 大写 h, 下标 2, 大写 o",
        ),
        (
            "chemical-triple-bond",
            "<math><mrow data-chem-formula='3'><mi mathvariant='normal' data-chem-element='1'>N</mi><mo data-chemical-bond='true' data-chem-formula-op='1'>≡</mo><mi mathvariant='normal' data-chem-element='1'>N</mi></mrow></math>",
            "大写 n, 三键 大写 n",
            "大写 n, 三键 大写 n",
        ),
        (
            "double-prime",
            "<math><msup><mi>f</mi><mo>″</mo></msup><mo>⁡</mo><mi>x</mi></math>",
            "f 双撇号, x",
            "f 双撇号, x",
        ),
        (
            "triple-prime",
            "<math><msup><mi>f</mi><mo>‴</mo></msup><mo>⁡</mo><mi>x</mi></math>",
            "f 三撇号, x",
            "f 三撇号, x",
        ),
        (
            "hat-variable",
            "<math><mover><mi>x</mi><mo>^</mo></mover></math>",
            "x 帽符",
            "x 帽符",
        ),
        (
            "dot-variable",
            "<math><mover><mi>x</mi><mo>˙</mo></mover></math>",
            "x 上点符",
            "x 上点符",
        ),
        (
            "overbrace",
            "<math><mover><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>⏞</mo></mover></math>",
            "a 加 b 上方有 上置花括号",
            "a 加 b 上方有 上置花括号",
        ),
        (
            "underbrace",
            "<math><munder><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>⏟</mo></munder></math>",
            "a 加 b 下方有 下置花括号",
            "a 加 b 下方有 下置花括号",
        ),
    ];

    for (name, mathml, simple, clear) in cases {
        test("zh", "SimpleSpeak", mathml, simple)
            .map_err(|error| anyhow::anyhow!("SimpleSpeak/{name}: {error}"))?;
        test("zh", "ClearSpeak", mathml, clear)
            .map_err(|error| anyhow::anyhow!("ClearSpeak/{name}: {error}"))?;
    }
    Ok(())
}

#[test]
fn additional_cn_semantic_function_cases_cover_unseen_terms() -> Result<()> {
    // Explicit intents make the intended Chinese mathematical term unambiguous.
    let cases = [
        (
            "arctangent",
            "<math><mrow intent='arctangent($x)'><mi arg='x'>x</mi></mrow></math>",
            "反正切 x",
        ),
        (
            "arcsecant",
            "<math><mrow intent='arcsecant($x)'><mi arg='x'>x</mi></mrow></math>",
            "反正割 x",
        ),
        (
            "hyperbolic-cosine",
            "<math><mrow intent='hyperbolic-cosine($x)'><mi arg='x'>x</mi></mrow></math>",
            "双曲余弦 x",
        ),
        (
            "arc-hyperbolic-sine",
            "<math><mrow intent='arc-hyperbolic-sine($x)'><mi arg='x'>x</mi></mrow></math>",
            "反双曲正弦 x",
        ),
        (
            "tuple",
            "<math><mrow intent='tuple($x,$y)'><mi arg='x'>x</mi><mi arg='y'>y</mi></mrow></math>",
            "元组 x 逗号, y",
        ),
        (
            "defined-as",
            "<math><mrow intent='defined-as($a,$b)'><mi arg='a'>f</mi><mi arg='b'>g</mi></mrow></math>",
            "f 定义为 g",
        ),
        (
            "equivalent-to",
            "<math><mrow intent='equivalent-to($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
            "a 等价于 b",
        ),
        (
            "similar",
            "<math><mrow intent='similar($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "大写 a 相似于 大写 b",
        ),
        (
            "ratio",
            "<math><mrow intent='ratio($a,$b)'><mi arg='a'>a</mi><mi arg='b'>b</mi></mrow></math>",
            "a 比 b",
        ),
        (
            "superset",
            "<math><mrow intent='superset($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "大写 a 超集 大写 b",
        ),
        (
            "not-superset",
            "<math><mrow intent='not-superset($a,$b)'><mi arg='a'>A</mi><mi arg='b'>B</mi></mrow></math>",
            "大写 a 非超集 大写 b",
        ),
    ];

    for (name, mathml, expected) in cases {
        for style in ["SimpleSpeak", "ClearSpeak"] {
            test("zh", style, mathml, expected)
                .map_err(|error| anyhow::anyhow!("{style}/{name}: {error}"))?;
        }
    }
    Ok(())
}
