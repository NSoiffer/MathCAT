#![cfg_attr(coverage, feature(coverage_attribute))]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_option_as_deref)]

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
///
/// This module re-exports anyhow types. Use `bail!` for early returns and
/// `context()`/`with_context()` on Result to add context (replacing old `chain_err()`).
pub mod errors {
    pub use anyhow::{anyhow, bail, Error, Result, Context};
}

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
#[cfg(feature = "rule-coverage")]
mod rule_coverage;
pub mod pretty_print;
mod chemistry;

pub mod shim_filesystem; // really just for override_file_for_debugging_rules, but the config seems to throw it off
pub use interface::*;
use crate::errors::{bail, Result};

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

/// Build Absolute path to rules dir for testing
pub fn abs_rules_dir_path() -> String {
    cfg_if::cfg_if! {
    if #[cfg(feature = "include-zip")] {
          return "Rules".to_string();
    } else {
        // Package root (see tests/common/mod.rs `abs_rules_dir_path` for rationale).
        return std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("Rules")
            .to_str()
            .expect("CARGO_MANIFEST_DIR and Rules path must be UTF-8")
            .to_string();
        }
    }
}

pub fn are_strs_canonically_equal_with_locale(test: &str, target: &str, ignore_attrs: &[&str], block_separators: &str, decimal_separators: &str) -> Result<()> {
    use crate::{interface::*, pretty_print::mml_to_string};
    use sxd_document_no_unsafe::parser;
    use crate::canonicalize::canonicalize;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    crate::interface::init_panic_handler();
    let result = catch_unwind(AssertUnwindSafe(|| {
        // this forces initialization
        crate::interface::set_rules_dir(abs_rules_dir_path()).unwrap();
        set_preference("Language", "en").unwrap();
        set_preference("BlockSeparators", block_separators).unwrap();
        set_preference("DecimalSeparators", decimal_separators).unwrap();
        crate::speech::SPEECH_RULES.with(|rules|  rules.borrow_mut().read_files().unwrap());

        let package1 = &parser::parse(test).expect("Failed to parse test input");
        let mathml = get_element(package1);
        trim_element(mathml, false);
        let mathml_test = canonicalize(mathml).unwrap();

        let package2 = &parser::parse(target).expect("Failed to parse target input");
        let mathml_target = get_element(package2);
        trim_element(mathml_target, false);

        match is_same_element(mathml_test, mathml_target, ignore_attrs) {
            Ok(_) => Ok( () ),
            Err(e) => {
                bail!("{}\nResult:\n{}\nTarget:\n{}", e, mml_to_string(mathml_test), mml_to_string(mathml_target));
            },
        }
    }));
    match crate::interface::report_any_panic(result) {
        Ok(()) => Ok(()),
        Err(e) => {
            Err(e)
        }
    }
}

/// sets locale to be US standard
pub fn are_strs_canonically_equal(test: &str, target: &str, ignore_attrs: &[&str]) -> bool {
    are_strs_canonically_equal_with_locale(test, target, ignore_attrs, ", \u{00A0}\u{202F}", ".").is_ok()
}

/// Like `are_strs_canonically_equal` but returns `Result` for use in `#[test]` functions that return `Result<()>`.
pub fn are_strs_canonically_equal_result(test: &str, target: &str, ignore_attrs: &[&str]) -> Result<()> {
    are_strs_canonically_equal_with_locale(test, target, ignore_attrs, ", \u{00A0}\u{202F}", ".")
}
