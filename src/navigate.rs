//! Navigation has two public functions: [`do_navigate_key_press`] and [`get_navigation_mathml`].
//!
//! Navigation is controlled by a `Navigation_Rules.yaml` file in conjunction with preferences.
//! See preference documentation for more info on navigation preferences.
#![allow(clippy::needless_return)]

//use sxd_document::dom::*;
/*
 * We will need something like for the navigation rules, but it is very similar to speech rules and there is a lot of duplication.
 * It may make the most sense to simply add a 'nav: RuleTable' entry to SpeechRules.
thread_local!{
    /// The current set of navigation rules
    pub static NAV_RULES: RefCell<NavRules> =
            RefCell::new( NavRules::new("initial") );

    static CONTEXT_STACK: RefCell<ContextStack<'static>> = RefCell::new( ContextStack{ new_defs: vec![], contexts: vec![] } );
}
*/

/// Given a key code along with the modifier keys, the current node is moved accordingly (or value reported in some cases).
///
/// The spoken text for the new current node is returned.
#[allow(unused_variables)]
pub fn do_navigate_key_press(key: usize, shift_key: bool, control_key: bool, alt_key: bool, meta_key: bool) -> String {
    // FIX: not yet implemented (basically what the braille says)
    return "⠠⠝⠁⠧⠊⠛⠁⠞⠊⠕⠝ ⠝⠕⠞ ⠽⠑⠞ ⠊⠍⠏⠇⠑⠍⠑⠝⠞".to_string();
}

/// Return the MathML associated with the current (navigation) node.
pub fn get_navigation_mathml() -> String {
    // FIX: not yet implemented (basically what the braille says)
    return "⠠⠝⠁⠧⠊⠛⠁⠞⠊⠕⠝ ⠝⠕⠞ ⠽⠑⠞ ⠊⠍⠏⠇⠑⠍⠑⠝⠞".to_string();
}
