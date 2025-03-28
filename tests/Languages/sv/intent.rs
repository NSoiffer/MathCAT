/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;


#[test]
fn silent_intent_mi() {
    let expr = "<math> <mn>2</mn> <mi intent=':silent'>x</mi></math>";
    test("sv", "SimpleSpeak", expr, "2");
    test("sv", "ClearSpeak", expr, "2");
}

#[test]
fn silent_intent_msup() {
    let expr = "<math>
        <msup intent='index:silent($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("sv", "SimpleSpeak", expr, "versal h 2");
    test("sv", "ClearSpeak", expr, "versal h 2");
}

#[test]
fn silent_intent_underscore() {
    let expr = "<math>
        <msup intent='_($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("sv", "SimpleSpeak", expr, "versal h 2");
    test("sv", "ClearSpeak", expr, "versal h 2");
}

#[test]
fn intent_prob_x() {
    let expr = "<math>
    <msup intent='$op($arg)'>
        <mi arg='arg'>x</mi>
        <mi arg='op' intent='sannolikheten' mathvariant='normal'>P</mi>
    </msup></math>";
    test("sv", "ClearSpeak", expr, "sannolikheten av x");
}
