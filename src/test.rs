//!  Useful functionality for testing
#[cfg(test)]

extern crate regex;
use regex::Regex;
extern crate lazy_static;
use crate::interface::speak_mathml;

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
pub fn test(mathml: &str, speech: &str) {
    assert_eq!(speech, strip_spaces(speak_mathml(mathml)));
}

// Compare the result of speaking the mathml input to the output 'speech'
// This forces the use of ClearSpeak and sets a single ClearSpeak preference
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_ClearSpeak(pref_name: &str, pref_value: &str, mathml: &str, speech: &str) {
    crate::speech::SPEECH_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        let pref_manager = rules.pref_manager.as_mut();
        pref_manager.set_user_prefs(pref_name, pref_value);
    });
    assert_eq!(speech, strip_spaces(speak_mathml(mathml)));
}

// Compare the result of speaking the mathml input to the output 'speech'
// This forces the use of ClearSpeak and sets multiple ClearSpeak preferences
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_ClearSpeak_prefs(prefs: Vec<(&str, &str)>, mathml: &str, speech: &str) {
    crate::speech::SPEECH_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        let pref_manager = rules.pref_manager.as_mut();
        for (pref_name, pref_value) in prefs {
            pref_manager.set_user_prefs(pref_name, pref_value);
        }
    });
    assert_eq!(speech, strip_spaces(speak_mathml(mathml)));
}
