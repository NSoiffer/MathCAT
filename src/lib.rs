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
pub mod canonicalize;
pub mod infer_intent;
pub mod speech;
pub mod navigate;
pub mod prefs;
pub mod tts;
pub mod xpath_functions;
pub mod definitions;
pub mod pretty_print;

pub mod shim_filesystem;

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
                .join("..\\..\\..\\..\\Rules")
                .to_str().unwrap().to_string();
}
