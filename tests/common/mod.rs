//!  Useful functionality for testing
#[cfg(test)]

use regex::Regex;
extern crate lazy_static;
use lazy_static::lazy_static;
pub use libmathcat::interface::*;

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
    cfg_if::cfg_if! {
    if #[cfg(feature = "include-zip")] {
          return "Rules".to_string();
    } else {
        return std::env::current_exe().unwrap().parent().unwrap()
                    .join("../../../Rules")
                    .to_str().unwrap().to_string();
        }
    }
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
fn check_answer(test: &str, target: &str, failure_message: &str) {
    if let Err(e) = set_mathml(test.to_string()) {
        panic!("{}", errors_to_string(&e));
    };
    match get_spoken_text() {
        Ok(speech) => assert_eq!(target, strip_spaces(speech), "\ntest with {} failed", failure_message),
        Err(e) => panic!("{}", errors_to_string(&e)),
    };
}

fn set_default_speech_prefs() {
    set_rules_dir(abs_rules_dir_path()).unwrap();
    libmathcat::speech::SPEECH_RULES.with(|rules| {
        let rules = rules.borrow_mut();
        let mut prefs = rules.pref_manager.borrow_mut();
        prefs.set_user_prefs("DecimalSeparator", "Auto").unwrap();
        prefs.set_user_prefs("SpeechOverrides_CapitalLetters", "").unwrap();         // makes testing simpler
        prefs.set_user_prefs("MathRate", "100").unwrap();                            // makes testing simpler
        prefs.set_user_prefs("PauseFactor", "100").unwrap();                         // makes testing simpler
        prefs.set_user_prefs("Verbosity", "Medium").unwrap();
        prefs.set_user_prefs("Impairment", "Blindness").unwrap();
    });
}
// Compare the result of speaking the mathml input to the output 'speech'
// This uses default preferences
#[allow(dead_code)]     // used in testing
pub fn test(language: &str, style: &str, mathml: &str, speech: &str) {
    set_default_speech_prefs();
    set_preference("Language".to_string(), language.to_string()).unwrap();
    set_preference("SpeechStyle".to_string(), style.to_string()).unwrap();
    check_answer(mathml, speech, &format!("{}/{}", language, style));
}

// Compare the result of speaking the mathml input to the output 'speech'
// This takes the speech style along with a vector of (pref_name, pref_value)
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_prefs(language: &str, speech_style: &str, test_prefs: Vec<(&str, &str)>, mathml: &str, speech: &str) {
    set_default_speech_prefs();
    set_preference("Language".to_string(), language.to_string()).unwrap();
    set_preference("SpeechStyle".to_string(), speech_style.to_string()).unwrap();
    for (pref_name, pref_value) in test_prefs.clone() {
        set_preference(pref_name.to_string(), pref_value.to_string()).unwrap();
    };
    check_answer(mathml, speech, &format!("{}/{} with prefs {:#?}", language, speech_style, test_prefs));
}

// Compare the result of speaking the mathml input to the output 'speech'
// This forces the use of ClearSpeak and sets a single ClearSpeak preference
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_ClearSpeak(language: &str, pref_name: &str, pref_value: &str, mathml: &str, speech: &str) {
    let prefs = vec![(pref_name, pref_value)];
    test_prefs(language, "ClearSpeak", prefs, mathml, speech);
}

// Compare the result of speaking the mathml input to the output 'speech'
// This forces the use of ClearSpeak and sets multiple ClearSpeak preferences
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_ClearSpeak_prefs(language: &str, prefs: Vec<(&str, &str)>, mathml: &str, speech: &str) {
    test_prefs(language, "ClearSpeak", prefs, mathml, speech);
}

// Compare the result of brailling the mathml input to the output (Unicode) 'braille'
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_braille(code: &str, mathml: &str, braille: &str) {
    set_rules_dir(abs_rules_dir_path()).unwrap();
    set_preference("DecimalSeparator".to_string(), "Auto".to_string()).unwrap();
    set_preference("BrailleNavHighlight".to_string(), "Off".to_string()).unwrap();
    set_preference("BrailleNavHighlight".to_string(), "Off".to_string()).unwrap();
    set_preference("BrailleCode".to_string(), code.to_string()).unwrap();
    set_preference("LaTeX_UseShortName".to_string(), "false".to_string()).unwrap();
    // FIX: this shouldn't need to be done -- need to figure out how to get definitions set automatically
    // log::debug!("\nsetting Language");
    match code {
        "Vietnam" => set_preference("Language".to_string(), "vi".to_string()).unwrap(),
        "CMU" => set_preference("Language".to_string(), "es".to_string()).unwrap(),
        "UEB" | "Nemeth" | _ => set_preference("Language".to_string(), "en".to_string()).unwrap(),
    }
    if let Err(e) = set_mathml(mathml.to_string()) {
        panic!("{}", errors_to_string(&e));
    };
    match get_braille("".to_string()) {
        Ok(result) => assert_eq!(braille, &result),
        Err(e) => panic!("{}", errors_to_string(&e)),
    };
}

#[allow(dead_code)]     // used in testing
pub fn test_braille_prefs(code: &str, test_prefs: Vec<(&str, &str)>, mathml: &str, braille: &str) {
    set_rules_dir(abs_rules_dir_path()).unwrap();
    set_preference("DecimalSeparator".to_string(), "Auto".to_string()).unwrap();
    set_preference("BrailleCode".to_string(), code.to_string()).unwrap();

    // FIX: this shouldn't need to be done -- need to figure out how to get definitions set automatically
    // log::debug!("\nsetting Language");
    match code {
        "Vietnam" => set_preference("Language".to_string(), "vi".to_string()).unwrap(),
        "CMU" => set_preference("Language".to_string(), "es".to_string()).unwrap(),
        "UEB" | "Nemeth" | _ => set_preference("Language".to_string(), "en".to_string()).unwrap(),
    }

    set_preference("UseSpacesAroundAllOperators".to_string(), "false".to_string()).unwrap();         // makes testing simpler
    for (pref_name, pref_value) in test_prefs.clone() {
        set_preference(pref_name.to_string(), pref_value.to_string()).unwrap();
    };

    if let Err(e) = set_mathml(mathml.to_string()) {
        panic!("{}", errors_to_string(&e));
    };
    match get_braille("".to_string()) {
        Ok(result) => assert_eq!(braille, &result),
        Err(e) => panic!("{}", errors_to_string(&e)),
    };
}

#[allow(dead_code)]
pub fn test_intent(mathml: &str, target: &str, test_prefs: Vec<(&str, &str)>) {
    use sxd_document::{parser, dom::Element};
    set_rules_dir(abs_rules_dir_path()).unwrap();
    libmathcat::speech::SPEECH_RULES.with(|rules| {
        let rules = rules.borrow_mut();
        let mut prefs = rules.pref_manager.borrow_mut();
        prefs.set_user_prefs("DecimalSeparators", ".").unwrap();
        prefs.set_user_prefs("BlockSeparators", ", ").unwrap();
    });

    // crate::speech::SpeechRules::initialize_all_rules().unwrap();
    set_preference("IntentErrorRecovery".to_string(), "Error".to_string()).unwrap();
    set_preference("SpeechStyle".to_string(), "SimpleSpeak".to_string()).unwrap();      // avoids possibility of "LiteralSpeak"
    for (pref_name, pref_value) in test_prefs.clone() {
        set_preference(pref_name.to_string(), pref_value.to_string()).unwrap();
    };

    let package = &parser::parse(target).expect("Failed to parse target input");
    let target = get_element(package);
    trim_element(target, true);

    let new_package = parser::parse(mathml);
    if let Err(e) = new_package {
        panic!("Invalid MathML:\n{}\nError is: {}", &mathml, &e.to_string());
    }

    let new_package = new_package.unwrap();
    let mathml = get_element(&new_package);
    let computed_intent = match libmathcat::get_intent(mathml, new_package.as_document()) {
        Ok(e) => e,
        Err(e) => panic!("in intent_from_mathml: {}", libmathcat::errors_to_string(&e)),
    };
    
    // remove some attrs that make it harder to handwrite what the intent is:
    //    'id' and 'data-id-added'; leaving 'data-from-mathml' as that is used by the code
    clean_attrs(computed_intent);

    match is_same_element(computed_intent, target) {
        Ok(_) => return ,
        Err(e) => {
            println!("target:\n{}", libmathcat::pretty_print::mml_to_string(target));
            println!("computed intent:\n{}", libmathcat::pretty_print::mml_to_string(computed_intent));
            panic!("{}", e)
        },
    }

    fn clean_attrs<'a>(mathml: Element<'a>) -> Element<'a> {
        mathml.remove_attribute("id");
        mathml.remove_attribute("data-id-added");

        let children = mathml.children();
        if children.is_empty() || (children.len() == 1 && children[0].element().is_none()) {
            return mathml;
        }
        
        for child in children {
            clean_attrs(child.element().unwrap());
        }
        return mathml;
    }
}

