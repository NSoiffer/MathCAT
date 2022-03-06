//!  Useful functionality for testing
#[cfg(test)]

use regex::Regex;
extern crate lazy_static;
use lazy_static::lazy_static;
use libmathcat::interface::*;

#[allow(dead_code)] 
pub fn init_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .is_test(true)
        .format_timestamp(None)
        .format_module_path(false)
        .format_indent(None)
        .format_level(false)
        .init();
}

/// Build Absolute path to rules dir for testing
pub fn abs_rules_dir_path() -> String {
    return std::env::current_exe().unwrap().parent().unwrap()
                .join("../../../Rules")
                .to_str().unwrap().to_string();
}


// Strip spaces from 'str' so comparison doesn't need to worry about spacing
#[allow(dead_code)]     // used in testing
fn strip_spaces(str: String) -> String {
    lazy_static! {
        static ref SPACES: Regex = Regex::new(r"  +").unwrap();
    }
    return String::from( SPACES.replace_all(&str, " ") );
}

#[allow(dead_code)]     // used in testing
fn check_answer(test: &str, target: &str) {
    if let Err(e) = set_mathml(test.to_string()) {
        panic!("{}", errors_to_string(&e));
    };
    match get_spoken_text() {
        Ok(speech) => assert_eq!(target, strip_spaces(speech)),
        Err(e) => panic!("{}", errors_to_string(&e)),
    };    
}

// Compare the result of speaking the mathml input to the output 'speech'
// This uses default preferences
#[allow(dead_code)]     // used in testing
pub fn test(style: &str, mathml: &str, speech: &str) {
    set_rules_dir(abs_rules_dir_path()).unwrap();
    libmathcat::speech::SPEECH_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        let changes = rules.pref_manager.borrow_mut().set_user_prefs("SpeechStyle", style);
        rules.invalidate(changes);
    });
    check_answer(mathml, speech);
}


// Compare the result of speaking the mathml input to the output 'speech'
// This takes the speech style along with a vector of (pref_name, pref_value)
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_prefs(speech_style: &str, prefs: Vec<(&str, &str)>, mathml: &str, speech: &str) {
    set_rules_dir(abs_rules_dir_path()).unwrap();
    libmathcat::speech::SPEECH_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        let mut changes = rules.pref_manager.borrow_mut().set_user_prefs("SpeechStyle", speech_style);
        for (pref_name, pref_value) in prefs {
            changes.add_changes(rules.pref_manager.borrow_mut().set_user_prefs(pref_name, pref_value));
        };
        rules.invalidate(changes);
    });
    check_answer(mathml, speech);
}

// Compare the result of speaking the mathml input to the output 'speech'
// This forces the use of ClearSpeak and sets a single ClearSpeak preference
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_ClearSpeak(pref_name: &str, pref_value: &str, mathml: &str, speech: &str) {
    set_rules_dir(abs_rules_dir_path()).unwrap();
    libmathcat::speech::SPEECH_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        let mut changes = rules.pref_manager.borrow_mut().set_user_prefs("SpeechStyle", "ClearSpeak");
        changes.add_changes( rules.pref_manager.borrow_mut().set_user_prefs(pref_name, pref_value) );
        rules.invalidate(changes);
    });
    check_answer(mathml, speech);
}

// Compare the result of speaking the mathml input to the output 'speech'
// This forces the use of ClearSpeak and sets multiple ClearSpeak preferences
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_ClearSpeak_prefs(prefs: Vec<(&str, &str)>, mathml: &str, speech: &str) {
    set_rules_dir(abs_rules_dir_path()).unwrap();
    libmathcat::speech::SPEECH_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        let mut changes = rules.pref_manager.borrow_mut().set_user_prefs("SpeechStyle", "ClearSpeak");
        for (pref_name, pref_value) in prefs {
            changes.add_changes(rules.pref_manager.borrow_mut().set_user_prefs(pref_name, pref_value));
        };
        rules.invalidate(changes);
    });
    check_answer(mathml, speech);
}

// Compare the result of brailling the mathml input to the output (Unicode) 'braille'
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_braille(code: &str, mathml: &str, braille: &str) {
    set_rules_dir(abs_rules_dir_path()).unwrap();
    libmathcat::speech::BRAILLE_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        let changes = rules.pref_manager.borrow_mut().set_user_prefs("BrailleCode", code);
        rules.invalidate(changes);
    });
    if let Err(e) = set_mathml(mathml.to_string()) {
        panic!("{}", errors_to_string(&e));
    };
    match get_braille("".to_string()) {
        Ok(result) => assert_eq!(braille, &result),
        Err(e) => panic!("{}", errors_to_string(&e)),
    };    
}
