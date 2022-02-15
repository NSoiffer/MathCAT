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
mod errors {
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
mod canonicalize;
mod infer_intent;
pub mod speech;
mod braille;
mod navigate;
mod prefs;
mod tts;
mod xpath_functions;
mod definitions;
mod pretty_print;

pub mod shim_filesystem;
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
    return std::env::current_exe().unwrap().parent().unwrap()
                .join("../../../Rules")
                .to_str().unwrap().to_string();
}
