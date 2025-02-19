use crate::common::*;

#[test]
fn complex() {
    let expr = "<math>
                    <mi>ℂ</mi>
                </math>";
    test("fi", "ClearSpeak", expr, "kompleksiluvut");
}

#[test]
fn natural() {
    let expr = "<math>
                    <mi>ℕ</mi>
                </math>";
    test("fi", "ClearSpeak", expr, "luonnolliset luvut");
}

#[test]
fn rationals() {
    let expr = "<math>
                    <mi>ℚ</mi>
                </math>";
    test("fi", "ClearSpeak", expr, "rationaaliluvut");
}

#[test]
fn reals() {
    let expr = "<math>
                    <mi>ℝ</mi>
                </math>";
    test("fi", "ClearSpeak", expr, "reaaliluvut");
}

#[test]
fn integers() {
    let expr = "<math>
                    <mi>ℤ</mi>
                </math>";
    test("fi", "ClearSpeak", expr, "kokonaisluvut");
}



#[test]
fn msup_complex() {
    let expr = "<math>
                <msup>
                    <mi>ℂ</mi>
                    <mn>2</mn>
                </msup>
                </math>";
    test("fi", "ClearSpeak", expr, "C 2");
}

#[test]
fn msup_natural() {
    let expr = "<math>
                <msup>
                    <mi>ℕ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("fi", "ClearSpeak", expr, "N 2");
}

#[test]
fn msup_rationals() {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("fi", "ClearSpeak", expr, "Q 2");
}

#[test]
fn msup_reals() {
    let expr = "<math>
                <msup>
                    <mi>ℝ</mi>
                    <mn>3</mn>
                </msup>
            </math>";
    test("fi", "ClearSpeak", expr, "R 3");
}

#[test]
fn msup_integers() {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mn>4</mn>
                </msup>
            </math>";
    test("fi", "ClearSpeak", expr, "Z 4");
}

#[test]
fn msup_positive_integers() {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("fi", "ClearSpeak", expr, "positiiviset kokonaisluvut");
}

#[test]
fn msup_negative_integers() {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("fi", "ClearSpeak", expr, "negatiiviset kokonaisluvut");
}

#[test]
fn msup_positive_rationals() {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("fi", "ClearSpeak", expr, "positiiviset rationaaliluvut");
}

#[test]
fn msup_negative_rationals() {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("fi", "ClearSpeak", expr, "negatiiviset rationaaliluvut");
}

#[test]
fn empty_set() {
    let expr = "<math>
                <mo>{</mo> <mo>}</mo>
            </math>";
    test("fi", "ClearSpeak", expr, "tyhjä joukko");
}

#[test]
fn single_element_set() {
    let expr = "<math>
                <mo>{</mo> <mn>12</mn><mo>}</mo>
            </math>";
    test("fi", "ClearSpeak", expr, "joukko 12");
}

#[test]
fn multiple_element_set() {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test("fi", "ClearSpeak", expr, "joukko 5 pilkku, 10 pilkku, 15");
}

#[test]
fn set_with_colon() {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("fi", "ClearSpeak", expr, "joukko kaikilla x siten että x on suurempi kuin 2");
}

#[test]
fn set_with_bar() {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("fi", "ClearSpeak", expr, "joukko kaikilla x siten että x on suurempi kuin 2");
}

#[test]
fn element_alone() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test("fi", "ClearSpeak", expr, "3 plus 2 i, ei kuulu reaaliluvut");
}

#[test]
fn element_under_sum() {
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
    test("fi", "ClearSpeak", expr,
                    "summa yli i kuuluu kokonaisluvut; murtoluku osoittaja 1; ja nimittäjä i toiseen");
}

#[test]
fn complicated_set_with_colon() {
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
    test("fi", "ClearSpeak", expr, "joukko kaikilla x kuuluu kokonaisluvut siten että 2 on pienempi kuin x on pienempi kuin 7");
}

#[test]
fn complicated_set_with_mtext() {
    // as of 8/5/21, parsing of "|" is problematic in the example, so <mrows> are needed for this test
    let expr = "<math>
        <mo>{</mo>
        <mrow> <mi>x</mi><mo>∈</mo><mi>ℕ</mi></mrow>
        <mo>|</mo>
        <mrow><mi>x</mi> <mtext>on parillinen luku</mtext> </mrow>
        <mo>}</mo>
        </math>";
    test("fi", "ClearSpeak", expr, 
            "joukko kaikilla x kuuluu luonnolliset luvut siten että x on parillinen luku");
}


#[test]
fn set_with_bar_member() {
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
    test_ClearSpeak("fi", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "joukko kaikilla x kuuluu kokonaisluvut siten että x on suurempi kuin 5");
}

#[test]
fn element_alone_member() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("fi", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "3 plus 2 i, ei kuulu reaaliluvut");
}

#[test]
fn element_under_sum_member() {
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
    test_ClearSpeak("fi", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "summa yli i kuuluu kokonaisluvut; murtoluku osoittaja 1; ja nimittäjä i toiseen");
}


#[test]
fn set_with_bar_element() {
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
    test_ClearSpeak("fi", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "joukko kaikilla x kuuluu kokonaisluvut siten että x on suurempi kuin 5");
}

#[test]
fn element_alone_element() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("fi", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "3 plus 2 i, ei kuulu reaaliluvut");
}

#[test]
fn element_under_sum_element() {
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
    test_ClearSpeak("fi", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "summa yli i kuuluu kokonaisluvut; murtoluku osoittaja 1; ja nimittäjä i toiseen");
}

#[test]
fn set_with_bar_in() {
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
    test_ClearSpeak("fi", "ClearSpeak_SetMemberSymbol", "In",
                expr, "joukko kaikilla x kuuluu kokonaisluvut siten että x on suurempi kuin 5");
}

#[test]
fn element_alone_in() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("fi", "ClearSpeak_SetMemberSymbol", "In",
                expr, "3 plus 2 i, ei ole joukossa reaaliluvut");
}

#[test]
fn element_under_sum_in() {
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
    test_ClearSpeak("fi", "ClearSpeak_SetMemberSymbol", "In",
                expr, "summa yli i kuuluu kokonaisluvut; murtoluku osoittaja 1; ja nimittäjä i toiseen");
}

#[test]
fn set_with_bar_belongs() {
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
    test_ClearSpeak("fi", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "joukko kaikilla x kuuluu joukkoon kokonaisluvut siten että x on suurempi kuin 5");
}

#[test]
fn element_alone_belongs() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("fi", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "3 plus 2 i, ei kuulu joukkoon reaaliluvut");
}

#[test]
fn element_under_sum_belongs() {
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
    test_ClearSpeak("fi", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "summa yli i kuuluu joukkoon kokonaisluvut; murtoluku osoittaja 1; ja nimittäjä i toiseen");
}


#[test]
fn set_member_woall() {
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
            test_ClearSpeak_prefs("fi", vec![("ClearSpeak_SetMemberSymbol", "Member"), ("ClearSpeak_Sets", "woAll")],
                expr, "joukko x kuuluu kokonaisluvut siten että x on suurempi kuin 5");
}

#[test]
fn multiple_element_set_woall() {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test_ClearSpeak("fi", "ClearSpeak_Sets", "woAll", expr, "joukko 5 pilkku, 10 pilkku, 15");
}

#[test]
fn multiple_element_set_silent_bracket() {
    let expr = "<math>joukko 5 pilkku, 10 pilkku, 15
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
            test_ClearSpeak("fi", "ClearSpeak_Sets", "SilentBracket", expr, "5 pilkku, 10 pilkku, 15");
        }

#[test]
fn silent_bracket() {
    let expr = "<math>
                <mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow><mo>}</mo>
            </math>";
            test_ClearSpeak("fi", "ClearSpeak_Sets", "SilentBracket", expr,
                    "joukko kaikilla x siten että x on suurempi kuin 2");
        }

