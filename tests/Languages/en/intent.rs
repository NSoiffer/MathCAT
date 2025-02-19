/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;


#[test]
fn silent_intent() {
    let expr = "<math> <mrow intent='testing:silent($arg1, $arg2)'><mn arg='arg1'>2</mn> <mi arg='arg2'>x</mi></mrow> </math>";
    test("en", "SimpleSpeak", expr, "2 x");
    test("en", "LiteralSpeak", expr, "2 x");
}

#[test]
fn prefix_intent() {
    let expr = r#"<math><msup intent='testing:prefix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("en", "SimpleSpeak", expr, "testing x");
}

#[test]
fn postfix_intent() {
    let expr = r#"<math><msup intent='testing:postfix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("en", "SimpleSpeak", expr, "x testing");
}

#[test]
fn infix_intent() {
    let expr = r#"<math><mrow intent='testing:infix($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("en", "SimpleSpeak", expr, "x testing y testing z testing 2");
}

#[test]
fn infix_intent_no_args() {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='testing:infix()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("en", "SimpleSpeak", expr, "x");
}

#[test]
fn infix_intent_one_arg() {
    let expr = r#"<math><mrow intent='testing:infix($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    // Note: we say the intent name because there are infix plus/minus with a single arg due to continued rows or combined columns
    test("en", "SimpleSpeak", expr, "testing x");
}

#[test]
fn function_intent() {
    let expr = r#"<math><mrow intent='testing:function($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("en", "SimpleSpeak", expr, "testing of x comma, y comma, z comma, 2");
}

#[test]
fn function_no_args_intent() {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='testing:function()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("en", "SimpleSpeak", expr, "x");
}

#[test]
fn function_one_arg_intent() {
    let expr = r#"<math><mrow intent='testing:function($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("en", "SimpleSpeak", expr, "testing of x");
}

#[test]
fn silent_intent_mi() {
    let expr = "<math> <mn>2</mn> <mi intent=':silent'>x</mi></math>";
    test("en", "SimpleSpeak", expr, "2");
    test("en", "ClearSpeak", expr, "2");
}

#[test]
fn silent_intent_msup() {
    let expr = "<math>
        <msup intent='index:silent($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("en", "SimpleSpeak", expr, "cap h 2");
    test("en", "ClearSpeak", expr, "cap h 2");
}

#[test]
fn silent_intent_underscore() {
    let expr = "<math>
        <msup intent='_($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("en", "SimpleSpeak", expr, "cap h 2");
    test("en", "ClearSpeak", expr, "cap h 2");
}

#[test]
fn intent_prob_x() {
    let expr = "<math>
    <msup intent='$op($arg)'>
        <mi arg='arg'>x</mi>
        <mi arg='op' intent='probability' mathvariant='normal'>P</mi>
    </msup></math>";
    test("en", "ClearSpeak", expr, "probability of x");
}
