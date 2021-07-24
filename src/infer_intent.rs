//! Use heuristics to infer the intent.
//! For example, an `mfrac` with `linethickness=0` would be a binomial
//! The inference is added to the MathML
//!
//! The implementation of the module is on hold until the MathML committee figures out how it wants to do this.

use sxd_document::dom::*;

pub fn infer_intent<'a>(mathml: &'a Element<'a>) -> &'a Element<'a> {
    // FIX: not yet implemented
    mathml
}
