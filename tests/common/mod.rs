//!  Useful functionality for testing
#![allow(clippy::needless_return)]

use regex::Regex;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::LazyLock;
pub use libmathcat::interface::*;
use anyhow::Result;


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
          "Rules".to_string()
    } else {
        // Package root (works for `cargo test` and `cargo llvm-cov`, which uses
        // target/llvm-cov-target/... so relative paths from current_exe() break).
        return std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("Rules")
            .to_str()
            .expect("CARGO_MANIFEST_DIR and Rules path must be UTF-8")
            .to_string();
        }
    }
}


// Strip spaces from 'str' so comparison doesn't need to worry about spacing
#[allow(dead_code)]     // used in testing
fn strip_spaces(str: &str) -> String {
    static SPACES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"  +").unwrap());
    String::from(SPACES.replace_all(str, " "))
}

#[allow(dead_code)]     // used in testing
fn check_answer(test: &str, target: &str, failure_message: &str) {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        if let Err(e) = set_mathml(test) {
            panic!("{}", errors_to_string(&e));
        };
        match get_spoken_text() {
            Ok(speech) => assert_eq!(target, strip_spaces(&speech), "\ntest with {} failed", failure_message),
            Err(e) => panic!("{}", errors_to_string(&e)),
        };
        Ok(())
    }));
    if let Err(e) = report_any_panic(result) {
        panic!("{}", e);
    }
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

/// Initialize navigation prefs + MathML for `tests/Languages/*/navigate.rs`.
/// Uses the same prefs as `src/navigate.rs` tests via [`set_navigation_test_preferences`],
/// except `AutoZoomOut` is false (language ZoomIn speech tests need that).
#[allow(dead_code)]     // used in testing
pub fn init_nav(language: &str, mathml: &str) -> Result<()> {
    set_rules_dir(abs_rules_dir_path())?;
    set_navigation_test_preferences(language, "Enhanced", false)?;
    set_mathml(mathml)?;
    Ok(())
}
// Compare the result of speaking the mathml input to the output 'speech'
// This uses default preferences
#[allow(dead_code)]     // used in testing
pub fn test(language: &str, style: &str, mathml: &str, speech: &str) -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        set_default_speech_prefs();
        set_preference("Language", language).unwrap();
        set_preference("SpeechStyle", style).unwrap();
        check_answer(mathml, speech, &format!("{}/{}", language, style));
        Ok(())
    }));
    report_any_panic(result)
}

// Compare the result of speaking the mathml input to the output 'speech'
// This takes the speech style along with a vector of (pref_name, pref_value)
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_prefs(language: &str, speech_style: &str, test_prefs: Vec<(&str, &str)>, mathml: &str, speech: &str) -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        set_default_speech_prefs();
        set_preference("Language", language).unwrap();
        set_preference("SpeechStyle", speech_style).unwrap();
        for &(pref_name, pref_value) in &test_prefs {
            set_preference(pref_name, pref_value).unwrap();
        };
        check_answer(mathml, speech, &format!("{}/{} with prefs {:#?}", language, speech_style, test_prefs));
        Ok(())
    }));
    report_any_panic(result)
}

// Compare the result of speaking the mathml input to the output 'speech'
// This forces the use of ClearSpeak and sets a single ClearSpeak preference
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_ClearSpeak(language: &str, pref_name: &str, pref_value: &str, mathml: &str, speech: &str) -> Result<()> {
    let prefs = vec![(pref_name, pref_value)];
    test_prefs(language, "ClearSpeak", prefs, mathml, speech)
}

// Compare the result of speaking the mathml input to the output 'speech'
// This forces the use of ClearSpeak and sets multiple ClearSpeak preferences
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_ClearSpeak_prefs(language: &str, prefs: Vec<(&str, &str)>, mathml: &str, speech: &str) -> Result<()> {
    test_prefs(language, "ClearSpeak", prefs, mathml, speech)
}

/// Speech language to use when testing a given `BrailleCode`.
/// Nemeth, UEB, LaTeX, and ASCIIMath use English; other codes use their natural language pack.
fn language_for_braille_code(code: &str) -> &'static str {
    match code {
        "Vietnam" => "vi",
        "CMU" => "es",
        "Swedish" => "sv",
        "Finnish" => "fi",
        "French" => "fr",
        "Russian" => "ru",
        _ => "en",
    }
}

fn set_language_for_braille_code(code: &str) {
    set_preference("Language", language_for_braille_code(code)).unwrap();
}

// Compare the result of brailling the mathml input to the output (Unicode) 'braille'
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_braille(code: &str, mathml: &str, braille: &str) -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        set_rules_dir(abs_rules_dir_path()).unwrap();
        set_preference("DecimalSeparator", "Auto").unwrap();
        set_preference("BrailleNavHighlight", "Off").unwrap();
        set_preference("BrailleNavHighlight", "Off").unwrap();
        set_preference("BrailleCode", code).unwrap();
        set_preference("LaTeX_UseShortName", "false").unwrap();
        set_language_for_braille_code(code);
        if let Err(e) = set_mathml(mathml) {
            panic!("{}", errors_to_string(&e));
        };
        match get_braille("") {
            Ok(result) => assert_eq!(braille, &result),
            Err(e) => panic!("{}", errors_to_string(&e)),
        };
        Ok(())
    }));
    report_any_panic(result)
}

#[allow(dead_code)]     // used in testing
pub fn test_braille_prefs(code: &str, test_prefs: Vec<(&str, &str)>, mathml: &str, braille: &str) -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        set_rules_dir(abs_rules_dir_path()).unwrap();
        set_preference("DecimalSeparator", "Auto").unwrap();
        set_preference("BrailleCode", code).unwrap();

        set_language_for_braille_code(code);

        set_preference("UseSpacesAroundAllOperators", "false").unwrap();         // makes testing simpler
        for &(pref_name, pref_value) in &test_prefs {
            set_preference(pref_name, pref_value).unwrap();
        };

        if let Err(e) = set_mathml(mathml) {
            panic!("{}", errors_to_string(&e));
        };
        match get_braille("") {
            Ok(result) => assert_eq!(braille, &result),
            Err(e) => panic!("{}", errors_to_string(&e)),
        };
        Ok(())
    }));
    report_any_panic(result)
}

/// Setup braille and assert `get_navigation_node_from_braille_position` result.
#[allow(dead_code)]     // used in testing
pub fn test_braille_nav_position(
    code: &str,
    mathml: &str,
    braille_position: usize,
    expected_node_id: &str,
    expected_offset: usize,
) -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        set_rules_dir(abs_rules_dir_path()).unwrap();
        set_preference("DecimalSeparator", "Auto").unwrap();
        set_preference("BrailleNavHighlight", "Off").unwrap();
        set_preference("BrailleCode", code).unwrap();
        set_preference("LaTeX_UseShortName", "false").unwrap();
        set_language_for_braille_code(code);
        if let Err(e) = set_mathml(mathml) {
            panic!("{}", errors_to_string(&e));
        };
        match get_navigation_node_from_braille_position(braille_position) {
            Ok((node_id, offset)) => {
                assert_eq!(node_id, expected_node_id);
                assert_eq!(offset, expected_offset);
            }
            Err(e) => panic!("{}", errors_to_string(&e)),
        };
        Ok(())
    }));
    report_any_panic(result)
}

#[allow(dead_code)]
pub fn test_intent(mathml: &str, target: &str, test_prefs: Vec<(&str, &str)>) -> Result<()> {
    use sxd_document_no_unsafe::{dom::Element, parser};
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        set_rules_dir(abs_rules_dir_path()).unwrap();
        libmathcat::speech::SPEECH_RULES.with(|rules| {
            let rules = rules.borrow_mut();
            let mut prefs = rules.pref_manager.borrow_mut();
            prefs.set_user_prefs("DecimalSeparators", ".").unwrap();
            prefs.set_user_prefs("BlockSeparators", ", ").unwrap();
        });

        set_preference("IntentErrorRecovery", "Error").unwrap();
        set_preference("SpeechStyle", "SimpleSpeak").unwrap();      // avoids possibility of "LiteralSpeak"
        for &(pref_name, pref_value) in &test_prefs {
            set_preference(pref_name, pref_value).unwrap();
        };

        let package = &parser::parse(target).expect("Failed to parse target input");
        let target_elem = get_element(package);
        trim_element(target_elem, true);

        let new_package = parser::parse(mathml);
        if let Err(e) = new_package {
            panic!("Invalid MathML:\n{}\nError is: {}", mathml, e);
        }

        let new_package = new_package.unwrap();
        let mathml_elem = get_element(&new_package);
        let computed_intent = match libmathcat::get_intent(mathml_elem, new_package.as_document()) {
            Ok(e) => e,
            Err(e) => panic!("in intent_from_mathml: {}", libmathcat::errors_to_string(&e)),
        };

        // remove some attrs that make it harder to handwrite what the intent is:
        //    'id' and 'data-id-added'; leaving 'data-from-mathml' as that is used by the code
        clean_attrs(computed_intent);

        match is_same_element(computed_intent, target_elem, &[]) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("target:\n{}", libmathcat::pretty_print::mml_to_string(target_elem));
                println!("computed intent:\n{}", libmathcat::pretty_print::mml_to_string(computed_intent));
                panic!("{}", e)
            },
        }
    }));

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
        mathml
    }

    report_any_panic(result)
}

/// This is a prototype function to test whether 'from_braille' (or whatever gets called) is cannonically the same as 'mathml'
#[allow(dead_code)]     // used in testing
#[allow(non_snake_case)]
pub fn test_from_braille(code: &str, mathml: &str, braille: &str) -> Result<()> {
    init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        set_rules_dir(abs_rules_dir_path()).unwrap();
        set_preference("DecimalSeparator", "Auto").unwrap();
        set_preference("BrailleNavHighlight", "Off").unwrap();
        set_preference("BrailleNavHighlight", "Off").unwrap();
        set_preference("BrailleCode", code).unwrap();
        set_preference("LaTeX_UseShortName", "false").unwrap();
        set_language_for_braille_code(code);
        if let Err(e) = set_mathml(mathml) {
            panic!("{}", errors_to_string(&e));
        };

        // FIX: call from_braille
        // let braille = from_braille(....);
        assert!(libmathcat::are_strs_canonically_equal(mathml, braille, &["data-changed", "data-id-added"]));
        Ok(())
    }));
    report_any_panic(result)
}
