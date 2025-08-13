use crate::common::*;

#[test]
fn complex() {
    let expr = "<math>
                    <mi>ℂ</mi>
                </math>";
    test("nb", "ClearSpeak", expr, "de komplekse tallene");
}

#[test]
fn natural() {
    let expr = "<math>
                    <mi>ℕ</mi>
                </math>";
    test("nb", "ClearSpeak", expr, "de naturlige tallene");
}

#[test]
fn rationals() {
    let expr = "<math>
                    <mi>ℚ</mi>
                </math>";
    test("nb", "ClearSpeak", expr, "de rasjonale tallene");
}

#[test]
fn reals() {
    let expr = "<math>
                    <mi>ℝ</mi>
                </math>";
    test("nb", "ClearSpeak", expr, "de reelle tallene");
}

#[test]
fn integers() {
    let expr = "<math>
                    <mi>ℤ</mi>
                </math>";
    test("nb", "ClearSpeak", expr, "heltallene");
}



#[test]
fn msup_complex() {
    let expr = "<math>
                <msup>
                    <mi>ℂ</mi>
                    <mn>2</mn>
                </msup>
                </math>";
    test("nb", "ClearSpeak", expr, "C 2");
}

#[test]
fn msup_natural() {
    let expr = "<math>
                <msup>
                    <mi>ℕ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("nb", "ClearSpeak", expr, "N 2");
}

#[test]
fn msup_rationals() {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("nb", "ClearSpeak", expr, "Q 2");
}

#[test]
fn msup_reals() {
    let expr = "<math>
                <msup>
                    <mi>ℝ</mi>
                    <mn>3</mn>
                </msup>
            </math>";
    test("nb", "ClearSpeak", expr, "R 3");
}

#[test]
fn msup_integers() {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mn>4</mn>
                </msup>
            </math>";
    test("nb", "ClearSpeak", expr, "Z 4");
}

#[test]
fn msup_positive_integers() {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("nb", "ClearSpeak", expr, "de positive heltallene");
}

#[test]
fn msup_negative_integers() {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("nb", "ClearSpeak", expr, "de negative heltallene");
}

#[test]
fn msup_positive_rationals() {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("nb", "ClearSpeak", expr, "de positive rasjonale tallene");
}

#[test]
fn msup_negative_rationals() {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("nb", "ClearSpeak", expr, "de negative rasjonale tallene");
}

#[test]
fn empty_set() {
    let expr = "<math>
                <mo>{</mo> <mo>}</mo>
            </math>";
    test("nb", "ClearSpeak", expr, "den tomme mengden");
}

#[test]
fn single_element_set() {
    let expr = "<math>
                <mo>{</mo> <mn>12</mn><mo>}</mo>
            </math>";
    test("nb", "ClearSpeak", expr, "mengden 12");
}

#[test]
fn multiple_element_set() {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test("nb", "ClearSpeak", expr, "mengden 5 komma, 10 komma, 15");
}

#[test]
fn set_with_colon() {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("nb", "ClearSpeak", expr, "mengden av alle x slik at x er større enn 2");
}

#[test]
fn set_with_bar() {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("nb", "ClearSpeak", expr, "mengden av alle x slik at x er større enn 2");
}

#[test]
fn element_alone() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test("nb", "ClearSpeak", expr, "3 pluss 2 i, er ikke et element i, de reelle tallene");
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
    test("nb", "ClearSpeak", expr,
                    "sum over i i heltallene, av; brøken med teller 1; og nevner i i andre");
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
    test("nb", "ClearSpeak", expr, "mengden av alle x i heltallene slik at 2 er mindre enn x er mindre enn 7");
}

#[test]
fn complicated_set_with_mtext() {
    // as of 8/5/21, parsing of "|" is problematic in the example, so <mrows> are needed for this test
    let expr = "<math>
        <mo>{</mo>
        <mrow> <mi>x</mi><mo>∈</mo><mi>ℕ</mi></mrow>
        <mo>|</mo>
        <mrow><mi>x</mi> <mtext>is an even number</mtext> </mrow>
        <mo>}</mo>
        </math>";
    test("nb", "ClearSpeak", expr, 
            "mengden av alle x i de naturlige tallene slik at x is an even number");
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
    test_ClearSpeak("nb", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "mengden av alle x tilhørende heltallene slik at x er større enn 5");
}

#[test]
fn element_alone_member() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("nb", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "3 pluss 2 i, tilhører ikke de reelle tallene");
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
    test_ClearSpeak("nb", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "sum over i tilhører heltallene, av; brøken med teller 1; og nevner i i andre");
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
    test_ClearSpeak("nb", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "mengden av alle x element i heltallene slik at x er større enn 5");
}

#[test]
fn element_alone_element() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("nb", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "3 pluss 2 i, er ikke et element i, de reelle tallene");
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
    test_ClearSpeak("nb", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "sum over i er et element i heltallene, av; brøken med teller 1; og nevner i i andre");
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
    test_ClearSpeak("nb", "ClearSpeak_SetMemberSymbol", "In",
                expr, "mengden av alle x i heltallene slik at x er større enn 5");
}

#[test]
fn element_alone_in() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("nb", "ClearSpeak_SetMemberSymbol", "In",
                expr, "3 pluss 2 i, er ikke i de reelle tallene");
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
    test_ClearSpeak("nb", "ClearSpeak_SetMemberSymbol", "In",
                expr, "sum over i i heltallene, av; brøken med teller 1; og nevner i i andre");
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
    test_ClearSpeak("nb", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "mengden av alle x tilhørende heltallene slik at x er større enn 5");
}

#[test]
fn element_alone_belongs() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("nb", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "3 pluss 2 i, tilhører ikke de reelle tallene");
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
    test_ClearSpeak("nb", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "sum over i tilhører heltallene, av; brøken med teller 1; og nevner i i andre");
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
            test_ClearSpeak_prefs("nb", vec![("ClearSpeak_SetMemberSymbol", "Member"), ("ClearSpeak_Sets", "woAll")],
                expr, "mengden av x tilhørende heltallene slik at x er større enn 5");
}

#[test]
fn multiple_element_set_woall() {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test_ClearSpeak("nb", "ClearSpeak_Sets", "woAll", expr, "mengden 5 komma, 10 komma, 15");
}

#[test]
fn multiple_element_set_silent_bracket() {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
            test_ClearSpeak("nb", "ClearSpeak_Sets", "SilentBracket", expr, "5 komma, 10 komma, 15");
        }

#[test]
fn silent_bracket() {
    let expr = "<math>
                <mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow><mo>}</mo>
            </math>";
            test_ClearSpeak("nb", "ClearSpeak_Sets", "SilentBracket", expr,
                    "mengden av alle x slik at x er større enn 2");
        }

