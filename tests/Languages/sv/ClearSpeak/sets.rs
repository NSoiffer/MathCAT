use crate::common::*;

#[test]
fn complex() {
    let expr = "<math>
                    <mi>ℂ</mi>
                </math>";
    test("sv", "ClearSpeak", expr, "de komplexa talen");
}

#[test]
fn natural() {
    let expr = "<math>
                    <mi>ℕ</mi>
                </math>";
    test("sv", "ClearSpeak", expr, "de naturliga talen");
}

#[test]
fn rationals() {
    let expr = "<math>
                    <mi>ℚ</mi>
                </math>";
    test("sv", "ClearSpeak", expr, "de rationella talen");
}

#[test]
fn reals() {
    let expr = "<math>
                    <mi>ℝ</mi>
                </math>";
    test("sv", "ClearSpeak", expr, "de reella talen");
}

#[test]
fn integers() {
    let expr = "<math>
                    <mi>ℤ</mi>
                </math>";
    test("sv", "ClearSpeak", expr, "heltalen");
}



#[test]
fn msup_complex() {
    let expr = "<math>
                <msup>
                    <mi>ℂ</mi>
                    <mn>2</mn>
                </msup>
                </math>";
    test("sv", "ClearSpeak", expr, "C 2");
}

#[test]
fn msup_natural() {
    let expr = "<math>
                <msup>
                    <mi>ℕ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("sv", "ClearSpeak", expr, "N 2");
}

#[test]
fn msup_rationals() {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mn>2</mn>
                </msup>
            </math>";
    test("sv", "ClearSpeak", expr, "Q 2");
}

#[test]
fn msup_reals() {
    let expr = "<math>
                <msup>
                    <mi>ℝ</mi>
                    <mn>3</mn>
                </msup>
            </math>";
    test("sv", "ClearSpeak", expr, "R 3");
}

#[test]
fn msup_integers() {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mn>4</mn>
                </msup>
            </math>";
    test("sv", "ClearSpeak", expr, "Z 4");
}

#[test]
fn msup_positive_integers() {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("sv", "ClearSpeak", expr, "de positiva heltalen");
}

#[test]
fn msup_negative_integers() {
    let expr = "<math>
                <msup>
                    <mi>ℤ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("sv", "ClearSpeak", expr, "de negativa heltalen");
}

#[test]
fn msup_positive_rationals() {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>+</mo>
                </msup>
            </math>";
    test("sv", "ClearSpeak", expr, "de positiva rationella talen");
}

#[test]
fn msup_negative_rationals() {
    let expr = "<math>
                <msup>
                    <mi>ℚ</mi>
                    <mo>-</mo>
                </msup>
            </math>";
    test("sv", "ClearSpeak", expr, "de negativa rationella talen");
}

#[test]
fn empty_set() {
    let expr = "<math>
                <mo>{</mo> <mo>}</mo>
            </math>";
    test("sv", "ClearSpeak", expr, "den tomma mängden");
}

#[test]
fn single_element_set() {
    let expr = "<math>
                <mo>{</mo> <mn>12</mn><mo>}</mo>
            </math>";
    test("sv", "ClearSpeak", expr, "mängden 12");
}

#[test]
fn multiple_element_set() {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test("sv", "ClearSpeak", expr, "mängden 5 komma, 10 komma, 15");
}

#[test]
fn set_with_colon() {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>:</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("sv", "ClearSpeak", expr, "mängden av alla x sådana att x är större än 2");
}

#[test]
fn set_with_bar() {
    let expr = "<math>
                    <mo>{</mo> <mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow> <mo>}</mo>
            </math>";
    test("sv", "ClearSpeak", expr, "mängden av alla x sådana att x är större än 2");
}

#[test]
fn element_alone() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test("sv", "ClearSpeak", expr, "3 plus 2 i, är inte ett element i, de reella talen");
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
    test("sv", "ClearSpeak", expr,
                    "summa över i i heltalen, av; division med täljaren 1; och nämnaren i kvadrat");
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
    test("sv", "ClearSpeak", expr, "mängden av alla x i heltalen sådana att 2 är mindre än x är mindre än 7");
}

#[test]
fn complicated_set_with_mtext() {
    // as of 8/5/21, parsing of "|" is problematic in the example, so <mrows> are needed for this test
    let expr = "<math>
        <mo>{</mo>
        <mrow> <mi>x</mi><mo>∈</mo><mi>ℕ</mi></mrow>
        <mo>|</mo>
        <mrow><mi>x</mi> <mtext>är ett jämnt tal</mtext> </mrow>
        <mo>}</mo>
        </math>";
    test("sv", "ClearSpeak", expr, 
            "mängden av alla x i de naturliga talen sådana att x är ett jämnt tal");
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
    test_ClearSpeak("sv", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "mängden av alla x tillhörande heltalen sådana att x är större än 5");
}

#[test]
fn element_alone_member() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("sv", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "3 plus 2 i, tillhör inte de reella talen");
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
    test_ClearSpeak("sv", "ClearSpeak_SetMemberSymbol", "Member",
                expr, "summa över i tillhör heltalen, av; division med täljaren 1; och nämnaren i kvadrat");
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
    test_ClearSpeak("sv", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "mängden av alla x element i heltalen sådana att x är större än 5");
}

#[test]
fn element_alone_element() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("sv", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "3 plus 2 i, är inte ett element i, de reella talen");
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
    test_ClearSpeak("sv", "ClearSpeak_SetMemberSymbol", "Element",
                expr, "summa över i i heltalen, av; division med täljaren 1; och nämnaren i kvadrat");
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
    test_ClearSpeak("sv", "ClearSpeak_SetMemberSymbol", "In",
                expr, "mängden av alla x i heltalen sådana att x är större än 5");
}

#[test]
fn element_alone_in() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("sv", "ClearSpeak_SetMemberSymbol", "In",
                expr, "3 plus 2 i, är inte i de reella talen");
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
    test_ClearSpeak("sv", "ClearSpeak_SetMemberSymbol", "In",
                expr, "summa över i är i heltalen, av; division med täljaren 1; och nämnaren i kvadrat");
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
    test_ClearSpeak("sv", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "mängden av alla x tillhörande heltalen sådana att x är större än 5");
}

#[test]
fn element_alone_belongs() {
    let expr = "<math>
            <mn>3</mn><mo>+</mo><mn>2</mn><mi>i</mi><mo>∉</mo><mi>ℝ</mi>
        </math>";
    test_ClearSpeak("sv", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "3 plus 2 i, tillhör inte de reella talen");
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
    test_ClearSpeak("sv", "ClearSpeak_SetMemberSymbol", "Belongs",
                expr, "summa över i tillhör heltalen, av; division med täljaren 1; och nämnaren i kvadrat");
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
            test_ClearSpeak_prefs("sv", vec![("ClearSpeak_SetMemberSymbol", "Member"), ("ClearSpeak_Sets", "woAll")],
                expr, "mängden av x tillhörande heltalen sådana att x är större än 5");
}

#[test]
fn multiple_element_set_woall() {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
    test_ClearSpeak("sv", "ClearSpeak_Sets", "woAll", expr, "mängden 5 komma, 10 komma, 15");
}

#[test]
fn multiple_element_set_silent_bracket() {
    let expr = "<math>
                <mo>{</mo> <mn>5</mn> <mo>,</mo> <mn>10</mn>  <mo>,</mo> <mn>15</mn> <mo>}</mo>
            </math>";
            test_ClearSpeak("sv", "ClearSpeak_Sets", "SilentBracket", expr, "5 komma, 10 komma, 15");
        }

#[test]
fn silent_bracket() {
    let expr = "<math>
                <mo>{</mo><mrow><mi>x</mi><mo>|</mo><mi>x</mi><mo>&#x003E;</mo><mn>2</mn></mrow><mo>}</mo>
            </math>";
            test_ClearSpeak("sv", "ClearSpeak_Sets", "SilentBracket", expr,
                    "mängden av alla x sådana att x är större än 2");
        }

