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