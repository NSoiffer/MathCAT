use crate::common::*;

use anyhow::Result;

#[test]
fn strict_subset_terminology() -> Result<()> {
    // The agreed convention makes the plain subset sign strict at every verbosity.
    let expr = "<math><mi>A</mi><mo>⊂</mo><mi>B</mi></math>";
    for verbosity in ["Terse", "Medium", "Verbose"] {
        test_prefs("ru", "ClearSpeak", vec![("Verbosity", verbosity)], expr,
            "заглавная а строгое подмножество заглавная бэ")?;
    }
    return Ok(());
}

#[test]
fn subset_or_equal_terminology() -> Result<()> {
    // The non-strict sign must retain the equality alternative.
    let expr = "<math><mi>A</mi><mo>⊆</mo><mi>B</mi></math>";
    test("ru", "ClearSpeak", expr, "заглавная а подмножество или равно заглавная бэ")?;
    test("ru", "SimpleSpeak", expr, "заглавная а подмножество или равно заглавная бэ")?;
    return Ok(());
}

#[test]
fn negated_subset_terminology() -> Result<()> {
    // Negation is expressed as a complete predicate, including for the full Unicode table.
    let expr = "<math><mi>A</mi><mo>⊄</mo><mi>B</mi></math>";
    test("ru", "ClearSpeak", expr, "заглавная а не является строгим подмножеством заглавная бэ")?;
    let expr = "<math><mi>A</mi><mo>⊈</mo><mi>B</mi></math>";
    test("ru", "ClearSpeak", expr, "заглавная а не является подмножеством заглавная бэ")?;
    return Ok(());
}

#[test]
fn complex() -> Result<()> {
    let expr = "<math>
                    <mi>ℂ</mi>
                </math>";
    test("ru", "ClearSpeak", expr, "комплексные числа")?;
    return Ok(());
}

#[test]
fn natural() -> Result<()> {
    let expr = "<math>
                    <mi>ℕ</mi>
                </math>";
    test("ru", "ClearSpeak", expr, "натуральные числа")?;
    return Ok(());
}

#[test]
fn rationals() -> Result<()> {
    let expr = "<math>
                    <mi>ℚ</mi>
                </math>";
    test("ru", "ClearSpeak", expr, "рациональные числа")?;
    return Ok(());
}

#[test]
fn reals() -> Result<()> {
    let expr = "<math>
                    <mi>ℝ</mi>
                </math>";
    test("ru", "ClearSpeak", expr, "действительные числа")?;
    return Ok(());
}

#[test]
fn integers() -> Result<()> {
    let expr = "<math>
                    <mi>ℤ</mi>
                </math>";
    test("ru", "ClearSpeak", expr, "целые числа")?;
    return Ok(());
}

#[test]
fn msup_complex() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℂ</mi>
                    <mn>2</mn>
                </msup>
                </math>";
    test("ru", "ClearSpeak", expr, "цэ 2")?;
    return Ok(());
}

#[test]
fn msup_natural() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℕ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("ru", "ClearSpeak", expr, "эн 2")?;
    return Ok(());
}

#[test]
fn msup_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("ru", "ClearSpeak", expr, "ку 2")?;
    return Ok(());
}

#[test]
fn msup_reals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℝ</mi>
                    <mn>3</mn>
                </msup>
            </math>";
    test("ru", "ClearSpeak", expr, "эр 3")?;
    return Ok(());
}

#[test]
fn msup_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mn>4</mn>
                </msup>
            </math>";
    test("ru", "ClearSpeak", expr, "зэт 4")?;
    return Ok(());
}

#[test]
fn msup_positive_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("ru", "ClearSpeak", expr, "положительные целые числа")?;
    return Ok(());
}

#[test]
fn msup_negative_integers() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("ru", "ClearSpeak", expr, "отрицательные целые числа")?;
    return Ok(());
}

#[test]
fn msup_positive_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("ru", "ClearSpeak", expr, "положительные рациональные числа")?;
    return Ok(());
}

#[test]
fn msup_negative_rationals() -> Result<()> {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("ru", "ClearSpeak", expr, "отрицательные рациональные числа")?;
    return Ok(());
}

#[test]
fn empty_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mo>}</mo>
            </math>";
    test("ru", "ClearSpeak", expr, "пустое множество")?;
    return Ok(());
}

#[test]
fn single_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>12</mn><mo>}</mo>
            </math>";
    test("ru", "ClearSpeak", expr, "множество 12")?;
    return Ok(());
}

#[test]
fn multiple_element_set() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test("ru", "ClearSpeak", expr, "множество 5 запятая, 10 запятая, 15")?;
    return Ok(());
}

#[test]
fn set_with_colon() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("ru", "ClearSpeak", expr, "множество всех икс таких, что икс больше 2")?;
    return Ok(());
}

#[test]
fn set_with_bar() -> Result<()> {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("ru", "ClearSpeak", expr, "множество всех икс таких, что икс больше 2")?;
    return Ok(());
}

#[test]
fn element_alone() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test("ru", "ClearSpeak", expr, "3 плюс 2 и, не принадлежит множеству действительных чисел")?;
    return Ok(());
}

#[test]
fn element_under_sum() -> Result<()> {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup>  <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test("ru", "ClearSpeak", expr,
                    "сумма по и, принадлежащему множеству целых чисел, от; дробь, числитель: 1; знаменатель: и в квадрате")?;
                    return Ok(());
}

#[test]
fn complicated_set_with_colon() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mn>2</mn>
            <mo>&#x003C;</mo>
            <mi>x</mi>
            <mo>&#x003C;</mo>
            <mn>7</mn>
            <mo>}</mo>
        </math>";
    test("ru", "ClearSpeak", expr, "множество всех икс из целых чисел, таких что 2 меньше икс меньше 7")?;
    return Ok(());
}

#[test]
fn complicated_set_with_mtext() -> Result<()> {
    let expr = "<math>
        <mo>{</mo>
        <mrow> <mi>x</mi><mo>∈</mo><mi>ℕ</mi></mrow>
        <mo>|</mo>
        <mrow><mi>x</mi> <mtext>is an even number</mtext> </mrow>
        <mo>}</mo>
        </math>";
    test("ru", "ClearSpeak", expr, 
            "множество всех икс из натуральных чисел, таких что икс — чётное число")?;
            return Ok(());
}

#[test]
fn set_with_bar_member() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "множество всех икс, принадлежащих множеству целых чисел, таких что икс больше 5")?;
                return Ok(());
}

#[test]
fn element_alone_member() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "3 плюс 2 и, не принадлежит множеству действительных чисел")?;
                return Ok(());
}

#[test]
fn element_under_sum_member() -> Result<()> {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup>  <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "сумма по и, принадлежащему множеству целых чисел, от; дробь, числитель: 1; знаменатель: и в квадрате")?;
                return Ok(());
}

#[test]
fn set_with_bar_element() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "множество всех икс, являющихся элементами множества целых чисел, таких что икс больше 5")?;
                return Ok(());
}

#[test]
fn element_alone_element() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "3 плюс 2 и, не является элементом множества действительных чисел")?;
                return Ok(());
}

#[test]
fn element_under_sum_element() -> Result<()> {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup>  <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "сумма по и, являющемуся элементом множества целых чисел, от; дробь, числитель: 1; знаменатель: и в квадрате")?;
                return Ok(());
}

#[test]
fn set_with_bar_in() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "In",
                expr, "множество всех икс из целых чисел, таких что икс больше 5")?;
                return Ok(());
}

#[test]
fn element_alone_in() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "In",
                expr, "3 плюс 2 и, не принадлежит множеству действительных чисел")?;
                return Ok(());
}

#[test]
fn element_under_sum_in() -> Result<()> {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup>  <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "In",
                expr, "сумма по и из целых чисел, от; дробь, числитель: 1; знаменатель: и в квадрате")?;
                return Ok(());
}

#[test]
fn set_with_bar_belongs() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "множество всех икс, входящих в множество целых чисел, таких что икс больше 5")?;
                return Ok(());
}

#[test]
fn element_alone_belongs() -> Result<()> {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "3 плюс 2 и, не входит в множество действительных чисел")?;
                return Ok(());
}

#[test]
fn element_under_sum_belongs() -> Result<()> {
    let expr = "<math>
            <munder>
                <mo>∑</mo>
                <mrow> <mi>i</mi> <mo>∈</mo> <mi>ℤ</mi> </mrow>
            </munder>
            <mfrac>
                <mn>1</mn>
                <mrow> <msup>  <mi>i</mi> <mn>2</mn> </msup> </mrow>
            </mfrac>
        </math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "сумма по и, входящему в множество целых чисел, от; дробь, числитель: 1; знаменатель: и в квадрате")?;
                return Ok(());
}

#[test]
fn set_member_woall() -> Result<()> {
    let expr = "<math>
            <mo>{</mo>
            <mi>x</mi>
            <mo>∈</mo>
            <mi>ℤ</mi>
            <mo>:</mo>
            <mi>x</mi>
            <mo>&#x003E;</mo>
            <mn>5</mn>
            <mo>}</mo>
            </math>";
    test_ClearSpeak_prefs("ru", vec![("ClearSpeak_SetMemberSymbol", "Member"), ("ClearSpeak_Sets", "woAll")],
                expr, "множество икс, принадлежащих множеству целых чисел, таких что икс больше 5")?;
                return Ok(());
}

#[test]
fn standard_number_sets_use_explicit_set_terminology() -> Result<()> {
    // A number belongs to a set of numbers; the preference must not make it an
    // element "of numbers" or say that it enters the numbers themselves.
    let natural = "<math><mo>{</mo><mi>x</mi><mo>∈</mo><mi>ℕ</mi><mo>:</mo><mi>x</mi><mo>&gt;</mo><mn>0</mn><mo>}</mo></math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Member", natural,
        "множество всех икс, принадлежащих множеству натуральных чисел, таких что икс больше 0")?;

    let rational = "<math><mo>{</mo><mi>x</mi><mo>∈</mo><mi>ℚ</mi><mo>:</mo><mi>x</mi><mo>&gt;</mo><mn>0</mn><mo>}</mo></math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Element", rational,
        "множество всех икс, являющихся элементами множества рациональных чисел, таких что икс больше 0")?;

    let real = "<math><mo>{</mo><mi>x</mi><mo>∈</mo><mi>ℝ</mi><mo>:</mo><mi>x</mi><mo>&gt;</mo><mn>0</mn><mo>}</mo></math>";
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Belongs", real,
        "множество всех икс, входящих в множество действительных чисел, таких что икс больше 0")?;
    return Ok(());
}

#[test]
fn standalone_membership_uses_explicit_set_terminology() -> Result<()> {
    // The spoken predicate must govern "множество", with the number-set name in the genitive.
    let expr = "<math><mi>x</mi><mo>∈</mo><mi>ℝ</mi></math>";
    test("ru", "SimpleSpeak", expr,
        "икс принадлежит множеству действительных чисел")?;
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Member", expr,
        "икс принадлежит множеству действительных чисел")?;
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "Element", expr,
        "икс является элементом множества действительных чисел")?;
    test_ClearSpeak("ru", "ClearSpeak_SetMemberSymbol", "In", expr,
        "икс находится в множестве действительных чисел")?;
    return Ok(());
}

#[test]
fn reverse_membership_uses_element_terminology() -> Result<()> {
    // The reverse membership signs describe an element, not the nonstandard calque "член".
    test("ru", "ClearSpeak", "<math><mi>A</mi><mo>∋</mo><mi>x</mi></math>",
        "заглавная а содержит элемент икс")?;
    test("ru", "SimpleSpeak", "<math><mi>A</mi><mo>∌</mo><mi>x</mi></math>",
        "заглавная а не содержит элемента икс")?;
    return Ok(());
}

#[test]
fn multiple_element_set_woall() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test_ClearSpeak("ru", "ClearSpeak_Sets", "woAll", expr, "множество 5 запятая, 10 запятая, 15")?;
    return Ok(());
}

#[test]
fn multiple_element_set_silent_bracket() -> Result<()> {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test_ClearSpeak("ru", "ClearSpeak_Sets", "SilentBracket", expr, "5 запятая, 10 запятая, 15")?;
    return Ok(());
}

#[test]
fn silent_bracket() -> Result<()> {
    let expr = "<math>
                <mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow><mo>}</mo>
            </math>";
    test_ClearSpeak("ru", "ClearSpeak_Sets", "SilentBracket", expr,
                    "множество всех икс таких, что икс больше 2")?;
                    return Ok(());
}

