//! This interface module provides a stateless API for converting from LaTeX to verbalized text.
//!
#![allow(non_snake_case)]
#![allow(clippy::needless_return)]
use std::rc::Rc;
use std::cell::RefCell;

use regex::{Captures, Regex};

use crate::canonicalize::CanonicalizeContext;
use crate::element_util::{add_ids, get_element, trim_element};
use crate::errors::*;
use crate::prefs::{PreferenceManager, PreferenceManagerBuilder};
use crate::pretty_print::mml_to_string;
use crate::speech::RulesFor;
use crate::speech::SpeechRules;
use sxd_document::{Package, parser};
use sxd_document::dom::Element;

// Used by include!("entities.in") call below.
use phf::phf_map;

/// Context for MathML to verbalized text conversion.
pub struct MathCat {
    speech_rules: SpeechRules,
    canonicalize_context: CanonicalizeContext,
}

/// Builds a MathCat instance.
///
/// Builders are not thread-safe, i.e. multiple builders should not be used concurrently.
pub struct MathCatBuilder {
    pref_manager_builder: PreferenceManagerBuilder,
}

impl MathCatBuilder {
    pub fn new() -> MathCatBuilder {
        MathCatBuilder { pref_manager_builder: PreferenceManagerBuilder::new() }
    }

    /// Set the string-valued preference.
    pub fn set_pref(&mut self, key: &str, value: &str) {
        self.pref_manager_builder.set_string_pref(key, value);
    }

    pub fn build(self) -> Result<MathCat> {
        let pref_manager: Rc<RefCell<PreferenceManager>> = self.pref_manager_builder.build()?;
        let canonicalize_context = CanonicalizeContext::new_uncached(&pref_manager.borrow());
        let mut speech_rules = load_rules(RulesFor::Speech, pref_manager)?;
        speech_rules.read_files()?;
        return Ok(MathCat {
            speech_rules: speech_rules,
            canonicalize_context: canonicalize_context,
        });
    }
}

impl MathCat {
    /// Returns the spoken text of the given MathML string using the given rules.
    pub fn mathml_to_spoken_text(&self, mathml: &str) -> Result<String> {
        let (package, _) = create_mathml_instance_with_text(&self.canonicalize_context, mathml)?;
        let mathml = get_element(&package);
        let new_package = Package::new();
        let intent = crate::speech::intent_from_rules_and_mathml(&self.speech_rules, mathml, new_package.as_document())?;
        return crate::speech::mathml_node_to_spoken_text(&self.speech_rules, intent, "");
    }
}

/// Loads the rules for a given preference manager.
fn load_rules(rules_for: RulesFor, pref_manager: Rc<RefCell<PreferenceManager>>) -> Result<SpeechRules> {
    let translate_single_chars_only = rules_for == RulesFor::Braille;
    let mut rules = SpeechRules::new(rules_for, translate_single_chars_only, pref_manager);
    rules.read_files()?;
    return Ok(rules);
}

// wrap up some common functionality between the call from 'main' and AT
pub (crate) fn cleanup_mathml<'a>(context: &CanonicalizeContext, mathml: Element<'a>) -> Result<Element<'a>> {
    // TODO: Canonicalization does not seem to actually use rules?
    trim_element(mathml, false);
    let mathml = context.canonicalize(mathml)?;
    let mathml = add_ids(mathml);
    return Ok(mathml);
}

/// Populates the `package` MathML instance.
/// This returns  canonical MathML with 'id's set on any node that doesn't have an id.
/// The ids can be used for sync highlighting if the `Bookmark` API preference is true.
pub(crate) fn create_mathml_instance_with_text(canonicalize_context: &CanonicalizeContext, mathml_str: &str) -> Result<(Package, String)> {
    lazy_static! {
        // if these are present when resent to MathJaX, MathJaX crashes (https://github.com/mathjax/MathJax/issues/2822)
        static ref MATHJAX_V2: Regex = Regex::new(r#"class *= *['"]MJX-.*?['"]"#).unwrap();
        static ref MATHJAX_V3: Regex = Regex::new(r#"class *= *['"]data-mjx-.*?['"]"#).unwrap();
        static ref NAMESPACE_DECL: Regex = Regex::new(r#"xmlns:[[:alpha:]]+"#).unwrap();     // very limited namespace prefix match
        static ref PREFIX: Regex = Regex::new(r#"(</?)[[:alpha:]]+:"#).unwrap();     // very limited namespace prefix match
        static ref HTML_ENTITIES: Regex = Regex::new(r#"&([a-zA-Z]+?);"#).unwrap();
    }

    static HTML_ENTITIES_MAPPING: phf::Map<&str, &str> = include!("entities.in");

    let mut error_message = "".to_string(); // can't return a result inside the replace_all, so we do this hack of setting the message and then returning the error
                                            // need to deal with character data and convert to something the parser knows
    let mathml_str =
        HTML_ENTITIES.replace_all(&mathml_str, |cap: &Captures| match HTML_ENTITIES_MAPPING.get(&cap[1]) {
            None => {
                error_message = format!("No entity named '{}'", &cap[0]);
                cap[0].to_string()
            }
            Some(&ch) => ch.to_string(),
        });

    if !error_message.is_empty() {
        bail!(error_message);
    }
    let mathml_str = MATHJAX_V2.replace_all(&mathml_str, "");
    let mathml_str = MATHJAX_V3.replace_all(&mathml_str, "");

    // the speech rules use the xpath "name" function and that includes the prefix
    // getting rid of the prefix properly probably involves a recursive replacement in the tree
    // if the prefix is used, it is almost certainly something like "m" or "mml", so this cheat will work.
    let mathml_str = NAMESPACE_DECL.replace(&mathml_str, "xmlns"); // do this before the PREFIX replace!
    let mathml_str = PREFIX.replace_all(&mathml_str, "$1");

    let package = parser::parse(&mathml_str);
    if let Err(e) = package {
        bail!("Invalid MathML input:\n{}\nError is: {}", &mathml_str, &e.to_string());
    }

    let package = package.unwrap();
    let mathml = get_element(&package);
    let mathml = cleanup_mathml(&canonicalize_context, mathml)?;
    let mathml_string = mml_to_string(mathml);

    Ok((package, mathml_string))
}
