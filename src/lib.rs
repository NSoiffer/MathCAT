//! A library for generating speech and braille from MathML
//! 
//! Typical usage is:
//! 1. Set the rules directory [`set_rules_dir`]
//! 2. Set whatever preferences are need with repeated calls to [`set_preference`].
//! 3. Set MathML via [`set_mathml`]
//!    A string representing the cleaned up MathML along with `id`s on each node is returned for highlighting if desired
//! 4. Get the speech [`get_spoken_text`] or (Unicode) braille [`get_braille`].
//!
//! The expression can be navigated also.
//! This is done in one of two ways:
//! 1. Pass key strokes to allow a user to navigate the MathML by calling [`do_navigate_keypress`]; the speech is returned.
//! 2. Pass the MathCAT navigation command directory by called [`do_navigate_command`]; the speech is return returned.
//! 
//! To get the MathML associated with the current navigation node, call [`get_navigation_mathml`].
//! To just get the `id` and offset from the id of the current navigation node, call [`get_navigation_mathml_id`].
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        // foreign_links {
        //     Io(std::io::Error);
        //     HttpRequest(reqwest::Error);
        // }
    }
}

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate log;

#[macro_use]
extern crate cfg_if;


pub mod interface;
#[cfg(feature = "include-zip")]
pub use shim_filesystem::ZIPPED_RULE_FILES;

mod canonicalize;
mod infer_intent;
pub mod speech;
mod braille;
mod navigate;
mod prefs;
mod tts;
mod xpath_functions;
mod definitions;
pub mod pretty_print;
mod chemistry;

pub mod shim_filesystem; // really just for override_file_for_debugging_rules, but the config seems to throw it off
pub use interface::*;

#[cfg(test)]
pub fn init_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .is_test(true)
        .format_timestamp(None)
        .format_module_path(false)
        .format_indent(None)
        .format_level(false)
        .init();
}

#[cfg(test)]
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

#[cfg(test)]
pub fn are_strs_canonically_equal_with_locale(test: &str, target: &str, block_separators: &str, decimal_separators: &str) -> bool {
    use crate::{interface::*, pretty_print::mml_to_string};
    use sxd_document::parser;
    use crate::canonicalize::canonicalize;
    // this forces initialization
    crate::interface::set_rules_dir(abs_rules_dir_path()).unwrap();
    crate::speech::SPEECH_RULES.with(|rules|  rules.borrow_mut().read_files().unwrap());
    set_preference("Language".to_string(), "en".to_string()).unwrap();
    set_preference("BlockSeparators".to_string(), block_separators.to_string()).unwrap();
    set_preference("DecimalSeparators".to_string(), decimal_separators.to_string()).unwrap();
    
    let package1 = &parser::parse(test).expect("Failed to parse test input");
    let mathml = get_element(package1);
    trim_element(mathml, false);
    // debug!("test:\n{}", mml_to_string(mathml));
    let mathml_test = canonicalize(mathml).unwrap();
   
    let package2 = &parser::parse(target).expect("Failed to parse target input");
    let mathml_target = get_element(package2);
    trim_element(mathml_target, false);
    // debug!("target:\n{}", mml_to_string(mathml_target));

    match is_same_element(mathml_test, mathml_target) {
        Ok(_) => return true,
        Err(e) => panic!("{}\nResult:\n{}\nTarget:\n{}", e, mml_to_string(mathml_test), mml_to_string(mathml_target)),
    }
}

#[cfg(test)]
// sets locale to be US standard
pub fn are_strs_canonically_equal(test: &str, target: &str) -> bool {
    return are_strs_canonically_equal_with_locale(test, target, ", \u{00A0}\u{202F}", ".");
}

