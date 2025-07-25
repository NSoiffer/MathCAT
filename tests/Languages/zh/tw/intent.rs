/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;


#[test]
fn silent_intent_mi() {
    let expr = "<math> <mn>2</mn> <mi intent=':silent'>x</mi></math>";
    test("zh-tw", "SimpleSpeak", expr, "2");
}

#[test]
fn silent_intent_msup() {
    let expr = "<math>
        <msup intent='index:silent($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("zh-tw", "SimpleSpeak", expr, "大寫 h 2");
}

#[test]
fn silent_intent_underscore() {
    let expr = "<math>
        <msup intent='_($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("zh-tw", "SimpleSpeak", expr, "大寫 h 2");
}

#[test]
fn intent_prob_x() {
    let expr = "<math>
    <msup intent='$op($arg)'>
        <mi arg='arg'>x</mi>
        <mi arg='op' intent='probability' mathvariant='normal'>P</mi>
    </msup></math>";
    test("zh-tw", "SimpleSpeak", expr, "probability x");
}
