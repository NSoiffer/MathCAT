//!  Useful functionality for testing
#[cfg(test)]

use regex::Regex;
extern crate lazy_static;
use lazy_static::lazy_static;
use libmathcat::interface::{speak_mathml, braille_mathml};

// Strip spaces from 'str' so comparison doesn't need to worry about spacing
#[allow(dead_code)]     // used in testing
fn strip_spaces(str: String) -> String {
    lazy_static! {
        static ref SPACES: Regex = Regex::new(r"  +").unwrap();
    }
    return String::from( SPACES.replace_all(&str, " ") );
}

// Compare the result of speaking the mathml input to the output 'speech'
// This uses default preferences
#[allow(dead_code)]     // used in testing
pub fn test(style: &str, mathml: &str, speech: &str) {
    libmathcat::speech::SPEECH_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        let changes = rules.pref_manager.borrow_mut().set_user_prefs("SpeechStyle", style);
        rules.invalidate(changes);
    });

    assert_eq!(speech, strip_spaces(speak_mathml(mathml)));
}


// Compare the result of speaking the mathml input to the output 'speech'
// This takes the speech style along with a vector of (pref_name, pref_value)
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_prefs(speech_style: &str, prefs: Vec<(&str, &str)>, mathml: &str, speech: &str) {
    libmathcat::speech::SPEECH_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        let mut changes = rules.pref_manager.borrow_mut().set_user_prefs("SpeechStyle", speech_style);
        for (pref_name, pref_value) in prefs {
            changes.add_changes(rules.pref_manager.borrow_mut().set_user_prefs(pref_name, pref_value));
        };
        rules.invalidate(changes);
    });
    assert_eq!(speech, strip_spaces(speak_mathml(mathml)));
}

// Compare the result of speaking the mathml input to the output 'speech'
// This forces the use of ClearSpeak and sets a single ClearSpeak preference
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_ClearSpeak(pref_name: &str, pref_value: &str, mathml: &str, speech: &str) {
    libmathcat::speech::SPEECH_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        let mut changes = rules.pref_manager.borrow_mut().set_user_prefs("SpeechStyle", "ClearSpeak");
        changes.add_changes( rules.pref_manager.borrow_mut().set_user_prefs(pref_name, pref_value) );
        rules.invalidate(changes);
    });
    assert_eq!(speech, strip_spaces(speak_mathml(mathml)));
}

// Compare the result of speaking the mathml input to the output 'speech'
// This forces the use of ClearSpeak and sets multiple ClearSpeak preferences
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_ClearSpeak_prefs(prefs: Vec<(&str, &str)>, mathml: &str, speech: &str) {
    libmathcat::speech::SPEECH_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        let mut changes = rules.pref_manager.borrow_mut().set_user_prefs("SpeechStyle", "ClearSpeak");
        for (pref_name, pref_value) in prefs {
            changes.add_changes(rules.pref_manager.borrow_mut().set_user_prefs(pref_name, pref_value));
        };
        rules.invalidate(changes);
    });
    assert_eq!(speech, strip_spaces(speak_mathml(mathml)));
}

// Compare the result of brailling the mathml input to the output (Unicode) 'braille'
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_braille(code: &str, mathml: &str, braille: &str) {
    libmathcat::speech::BRAILLE_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        let changes = rules.pref_manager.borrow_mut().set_user_prefs("Code", code);
        rules.invalidate(changes);
    });
    assert_eq!(braille, strip_spaces(braille_mathml(mathml, "".to_string())));
}
