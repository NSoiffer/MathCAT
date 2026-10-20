//! Use heuristics to infer the intent.
//! For example, an `mfrac` with `linethickness=0` would be a binomial
//! The inference is added to the MathML
//!
//! The implementation of the module is on hold until the MathML committee figures out how it wants to do this.
#![allow(clippy::needless_return)]

use sxd_document_no_unsafe::dom::{Element, Document, ChildOfElement};
use sxd_document_no_unsafe::as_str;
use crate::prefs::PreferenceManager;
use crate::speech::SpeechRulesWithContext;
use crate::canonicalize::{as_element, as_text, name, create_mathml_element, set_mathml_name, INTENT_ATTR, MATHML_FROM_NAME_ATTR};
use crate::errors::*;
use std::fmt;
use std::sync::LazyLock;
use crate::pretty_print::mml_to_string;
use crate::xpath_functions::is_leaf;
use regex::Regex;
use phf::phf_set;
#[allow(unused_imports)]
use log::{debug, error, warn};


const IMPLICIT_FUNCTION_NAME: &str = "apply-function";

pub fn infer_intent<'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, mathml: Element<'c>) -> Result<Element<'m>> {
    match catch_errors_building_intent(rules_with_context, mathml) {
        Ok(intent) => return Ok(intent),
        Err(e) => {
            // lookup what we should do for error recovery
            let intent_preference = rules_with_context.get_rules().pref_manager.borrow().pref_to_string("IntentErrorRecovery");
            if intent_preference == "Error" {
                return Err(e);
            } else {
                let saved_intent_attr = mathml.attribute_value(INTENT_ATTR).unwrap();
                mathml.remove_attribute(INTENT_ATTR);
                // can't call intent_from_mathml() because we have already borrowed_mut -- we call a more internal version
                let intent_tree =  match rules_with_context.match_pattern::<Element<'m>>(mathml)
                                            .context("Pattern match/replacement failure!") {
                    Err(e) => Err(e),
                    Ok(intent) => {
                        intent.set_attribute_value(INTENT_ATTR, as_str!(saved_intent_attr)); //  so attr can be potentially be viewed later
                        Ok(intent)
                    },
                };
                mathml.set_attribute_value(INTENT_ATTR, as_str!(saved_intent_attr));
                return intent_tree;
            }
        }
    }

    fn catch_errors_building_intent<'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, mathml: Element<'c>) -> Result<Element<'m>> {
        if let Some(intent_str) = mathml.attribute_value(INTENT_ATTR) {
            // debug!("Before intent: {}", crate::pretty_print::mml_to_string(mathml));
            let mut lex_state = LexState::init(intent_str.trim())?;
            let mut intent_offset = 0;
            let result = build_intent(rules_with_context, &mut lex_state, mathml, &mut intent_offset)
                        .with_context(|| format!("occurs before '{}' in intent attribute value '{}'", lex_state.remaining_str, intent_str))?;
            if lex_state.token != Token::None {
                bail!("Error in intent value: extra unparsed intent '{}' in intent attribute value '{}'", lex_state.remaining_str, intent_str);
            }
            assert!(lex_state.remaining_str.is_empty());
            // debug!("Resulting intent:\n{}", crate::pretty_print::mml_to_string(result));
            return Ok(result);
        }
        bail!("Internal error: infer_intent() called on MathML with no intent arg:\n{}", mml_to_string(mathml));
    }
}


static FIXITIES: phf::Set<&str> = phf_set! {
    "function", "infix", "prefix", "postfix", "silent", "nofix", "other",
};

/// Eliminate all but the last fixity property
pub fn simplify_fixity_properties(properties: &str) -> String {
    let parts: Vec<&str> = properties.split(':').collect();
    // debug!("simplify_fixity_properties {} parts from input: '{}'", parts.len(), properties);
    let mut fixity_property = "";
    let mut answer = ":".to_string();
    for part in parts {
        if FIXITIES.contains(part) {
            fixity_property = part;
        } else if !part.is_empty() {
            answer.push_str(part);
            answer.push(':');
        }
    }
    if !fixity_property.is_empty() {
        answer.push_str(fixity_property);
        answer.push(':');
    }
    return answer;
}

/// Given the intent add the fixity property for the intent if it isn't given (and one exists)
fn add_fixity(intent: Element) {
    let raw_properties = intent.attribute_value(INTENT_PROPERTY);
    let properties = raw_properties.as_deref().unwrap_or_default();
    if properties.split(":").all(|property| !FIXITIES.contains(property)) {
        let intent_name = name(intent);
        crate::definitions::SPEECH_DEFINITIONS.with(|definitions| {
            let definitions = definitions.borrow();
            // debug!("    add_fixity: intent_name: {}, ", intent_name);
            if let Some(definition) = definitions.get_hashmap("IntentMappings").unwrap().get(as_str!(intent_name)) &&
               let Some((fixity, _)) = definition.split_once("=") {
                // `nofix` is IntentMappings' leaf default (authors should not write :nofix).
                // Only auto-apply it on leaves so concepts like `nofix=... || function=...`
                // (e.g. volume) still pick up :function: when they have arguments.
                let is_leaf = intent.children().iter().all(|c| c.element().is_none());
                if fixity != "nofix" || is_leaf {
                    let new_properties = (if properties.is_empty() {":"} else {properties}).to_string() + fixity + ":";
                    intent.set_attribute_value(INTENT_PROPERTY, &new_properties);
                    // debug!("Added fixity: new value '{}'", intent.attribute_value(INTENT_PROPERTY).unwrap());
                }
            }
        });
    }
}


/// Given some MathML, expand out any intents taking into account their fixity property
/// This is recursive
pub fn add_fixity_children(intent: Element) -> Element {
    let children = intent.children();
    if children.is_empty() || (children.len() == 1 && children[0].element().is_none()) {
        return intent;
    }

    for child in children {
        let child = as_element(child);
        if child.attribute_value(INTENT_ATTR).is_some() {
            add_fixity_child(child);
        }
    }
    return intent;

    fn add_fixity_child(mathml: Element) -> Element {        
        let mut children = mathml.children();
        if children.is_empty() {
            return mathml;
        }
        // we also exclude fixity on mtable because they mess up the counts (see 'en::mtable::unknown_mtable_property')
        if mathml.attribute_value(MATHML_FROM_NAME_ATTR).as_deref().unwrap_or_default() == "mtable" {
            return mathml;
        }
        let doc = mathml.document();
        let raw_properties = mathml.attribute_value(INTENT_PROPERTY);
        let properties = raw_properties.as_deref().unwrap_or_default();
        let fixity = properties.rsplit(':').find(|&property| FIXITIES.contains(property)).unwrap_or_default();
        let intent_name = as_str!(name(mathml));
        // debug!("add_fixity_child:  fixity '{}', intent_name '{}'", fixity, intent_name);
    
        let raw_op_id = mathml.attribute_value("id");
        let op_name_id = raw_op_id.as_deref().unwrap_or("new-id");
        match fixity {
            "infix" => {
                let mut new_children = Vec::with_capacity(2*children.len()-1);
                new_children.push(children[0]);
                for (i, &child) in children.iter().enumerate().skip(1) {
                    new_children.push(create_operator_element(intent_name, fixity, op_name_id, i, &doc));
                    new_children.push(child);
                }
                mathml.replace_children(new_children);
            },
            "prefix" => { 
                children.insert(0, create_operator_element(intent_name, fixity, op_name_id, 1, &doc));                       
                mathml.replace_children(children);
            },
            "postfix" => { 
                children.push( create_operator_element(intent_name, fixity, op_name_id, 1, &doc));                       
                mathml.replace_children(children);
            },
            "nofix" => {
                if children.len() == 1 {
                    // Fix type error: can't assign ChildOfElement to children[0] (which is an Element)
                    // Instead, replace the only child with the operator child using replace_children
                    children[0] = create_operator_element(intent_name, fixity, op_name_id, 1, &doc);
         
                }
            },
            "silent" => {
                // children remain the same -- nothing to do
            },
            "other" => {
                // a special case -- will be handled with specific rules (e.g., intervals need to add "from" and "to", not a single word)
            },
            _ => {  // "function" is the default
                // build a function like notation function-name U+2061 <mrow> children </mrow>
                let mut new_children = Vec::with_capacity(3);
                let function_name = create_operator_element(intent_name, "function", op_name_id, 1, &doc);
                new_children.push(function_name);
                let invisible_apply_function = create_operator_element("mo", "infix", op_name_id, 2, &doc);
                invisible_apply_function.element().unwrap().set_text("\u{2061}");
                new_children.push(invisible_apply_function);
                let mrow_wrapper = create_mathml_element(&doc, "mrow");
                mrow_wrapper.set_attribute_value("id", (op_name_id.to_string() + "3").as_str());
                mrow_wrapper.append_children(children);
                new_children.push(ChildOfElement::Element(mrow_wrapper));
                mathml.replace_children(new_children);
                if fixity.is_empty() {
                    mathml.set_attribute_value(INTENT_PROPERTY, ":function:");
                }
            },
        }
        return mathml;
    
        fn create_operator_element<'a>(intent_name: &str, fixity: &str, id: &str, id_inc: usize, doc: &Document<'a>) -> ChildOfElement<'a> {
            let intent_name = intent_speech_for_name(intent_name, &PreferenceManager::get().borrow().pref_to_string("NavMode"), fixity);
            let element = create_mathml_element(doc, &intent_name);
            element.set_attribute_value("id", &format!("{id}-fixity-{id_inc}"));
            element.set_attribute_value(MATHML_FROM_NAME_ATTR, "mo");
            return ChildOfElement::Element(element);
        }
    }
}

// -------------------------------------------------------------------------------------------------
// IntentMappings parsing
//
// A mapping value is `fixity=<body> [|| fixity2=<body2> ...]`. Each `<body>` is parsed by
// `parse_fixity_mapping` into a `FixityMapping` (resolved for the current verbosity). Two forms
// are supported:
//   * Standard:    `[open;] name [;close]  [: glue]*`
//                  `|` inside any piece selects terse|medium|verbose. The `:`-separated glue
//                  options describe how arguments are joined (arity templates or a binary word).
//   * Ratio-style: `name : glue | name2 : glue2 | ...`
//                  each `|`-alternative is a whole `name : glue` pair and verbosity picks one.
// All the query functions below operate on the parsed `FixityMapping`, so the delimiter rules
// live in exactly one place.
// -------------------------------------------------------------------------------------------------

/// A fixity mapping (`fixity=...` text) parsed and resolved for one verbosity level.
struct FixityMapping {
    /// Bracketing phrase spoken before the arguments (only present for an `open; name; close` form).
    open: String,
    /// The main spoken name/phrase.
    name: String,
    /// Bracketing phrase spoken after the arguments.
    close: String,
    /// Argument-glue options (the `:`-separated parts after the name). An arity template has one
    /// entry per supported arity (comma-separated glue words); a binary separator is a single entry
    /// with no comma; a plain function has none.
    glue: Vec<String>,
}

pub fn intent_speech_for_name(intent_name: &str, verbosity: &str, fixity: &str) -> String {
    match fixity_mapping(intent_name, fixity, verbosity) {
        Some(mapping) => mapping.name,
        None => intent_name.replace(['_', '-'], " ").trim().to_string(),
    }
}

/// Bracketing phrase for an intent's `open; name; close` speech form. `at_start` selects the
/// opening phrase, otherwise the closing one. Empty when the mapping has no bracketing or the
/// fixity is not found.
pub fn intent_bracketing_word(intent_name: &str, fixity: &str, verbosity: &str, at_start: bool) -> String {
    match fixity_mapping(intent_name, fixity, verbosity) {
        Some(mapping) => if at_start { mapping.open } else { mapping.close },
        None => String::new(),
    }
}

/// Look up `intent_name`'s body for `fixity` and parse it, resolving `|` variants for `verbosity`.
fn fixity_mapping(intent_name: &str, fixity: &str, verbosity: &str) -> Option<FixityMapping> {
    crate::definitions::SPEECH_DEFINITIONS.with(|definitions| {
        let definitions = definitions.borrow();
        let mappings = definitions.get_hashmap("IntentMappings").unwrap();
        let pattern = mappings.get(intent_name)?;
        let body = pattern.split("||").find(|entry| entry.trim().starts_with(fixity))?;
        let (_, after_eq) = body.split_once('=')?;
        Some(parse_fixity_mapping(after_eq.trim(), verbosity))
    })
}

fn parse_fixity_mapping(after_eq: &str, verbosity: &str) -> FixityMapping {
    // Ratio-style: `name : glue | name2 : glue2 | ...` — verbosity picks one (name, glue) pair.
    if let Some((name, glue)) = ratio_style_pair(after_eq, verbosity) {
        return FixityMapping { open: String::new(), name, close: String::new(), glue: vec![glue] };
    }

    // Standard: the name/bracketing part (before the first `:`), then argument-glue options.
    let mut sections = after_eq.split(':');
    let (open, name, close) = split_bracketing(sections.next().unwrap_or_default());
    let glue = sections
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|option| pick_verbosity(option, verbosity))
        .collect();
    FixityMapping {
        open: pick_verbosity(open, verbosity),
        name: pick_verbosity(name, verbosity),
        close: pick_verbosity(close, verbosity),
        glue,
    }
}

/// Split the name portion into `(open, name, close)` on `;`. Bracketing words are only present
/// when there are exactly three parts; otherwise the second part (if any) is the name.
fn split_bracketing(speech_part: &str) -> (&str, &str, &str) {
    let parts: Vec<&str> = speech_part.split(';').collect();
    match parts.len() {
        1 => ("", parts[0], ""),
        3 => (parts[0], parts[1], parts[2]),
        _ => ("", parts.get(1).copied().unwrap_or_default(), ""),
    }
}

/// The index into a `|`-separated option (or ratio-style pair list) for `verbosity`, clamped to
/// the number of available forms: terse=0, medium=1, verbose=2.
fn verbosity_index(verbosity: &str, len: usize) -> usize {
    match verbosity {
        _ if len <= 1 => 0,
        "Terse" => 0,
        "Medium" => 1.min(len - 1),
        _ => 2.min(len - 1),
    }
}

/// Selects the form for `verbosity` from a `|`-separated option, e.g. `sin | sine` or `to | to`.
fn pick_verbosity(option: &str, verbosity: &str) -> String {
    let forms: Vec<&str> = option.split('|').map(str::trim).filter(|s| !s.is_empty()).collect();
    match forms.as_slice() {
        [] => String::new(),
        _ => forms[verbosity_index(verbosity, forms.len())].to_string(),
    }
}

/// Ratio-style mapping: `name : glue | name2 : glue2 | ...`. Returns the (name, glue) pair for
/// `verbosity`, or `None` if the text is not a set of `name : glue` alternatives.
fn ratio_style_pair(after_eq: &str, verbosity: &str) -> Option<(String, String)> {
    let alternatives: Vec<&str> = after_eq.split('|').map(str::trim).filter(|s| !s.is_empty()).collect();
    if alternatives.len() < 2 {
        return None;
    }
    let pairs: Vec<(&str, &str)> = alternatives.iter()
        .map(|alternative| {
            let (name, glue) = alternative.split_once(':')?;
            let glue = glue.trim();
            (!glue.is_empty() && !glue.contains(':')).then_some((name.trim(), glue))
        })
        .collect::<Option<_>>()?;
    let (name, glue) = pairs[verbosity_index(verbosity, pairs.len())];
    Some((name.to_string(), glue.to_string()))
}

fn get_verbosity_pref() -> String {
    PreferenceManager::get().borrow().pref_to_string("Verbosity")
}

/// A binary separator is a single glue option with no comma (e.g. `integer part: divided by`).
fn binary_separator(mapping: &FixityMapping) -> Option<&str> {
    match mapping.glue.as_slice() {
        [only] if !only.contains(',') => Some(only),
        _ => None,
    }
}

/// True when the glue is an arity template (multiple options, or one option with comma-separated
/// glue words) rather than a binary separator or a plain function.
fn is_arity_mode(mapping: &FixityMapping) -> bool {
    binary_separator(mapping).is_none()
        && (mapping.glue.len() > 1 || mapping.glue.iter().any(|option| option.contains(',')))
}

/// The glue words for an exact arity match: the option with `arg_count - 1` comma-separated words.
fn arity_glue(mapping: &FixityMapping, arg_count: usize) -> Option<Vec<&str>> {
    if arg_count == 0 || binary_separator(mapping).is_some() {
        return None;
    }
    let want = arg_count - 1;
    mapping.glue.iter().find_map(|option| {
        let words: Vec<&str> = option.split(',').map(str::trim).filter(|s| !s.is_empty()).collect();
        (words.len() == want).then_some(words)
    })
}


/// Whether the function intent matches an arity-template pattern for the current argument count.
pub fn intent_function_has_arity_match(intent_name: &str, fixity: &str, arg_count: usize) -> bool {
    let verbosity = get_verbosity_pref();
    let Some(mapping) = fixity_mapping(intent_name, fixity, &verbosity) else { return false; };
    is_arity_mode(&mapping) && arity_glue(&mapping, arg_count).is_some()
}

/// Returns the glue word immediately before a specific argument when using an arity template.
pub fn intent_function_glue_before(intent_name: &str, fixity: &str, arg_index: usize, arg_count: usize) -> String {
    let verbosity = get_verbosity_pref();
    let Some(mapping) = fixity_mapping(intent_name, fixity, &verbosity) else { return String::new(); };
    if !is_arity_mode(&mapping) {
        return String::new();
    }
    if arg_index == arg_count {
        return function_application_word();       // last argument is preceded by "of"
    }
    match arity_glue(&mapping, arg_count) {
        Some(words) if (1..arg_count).contains(&arg_index) => words[arg_index - 1].to_string(),
        _ => String::new(),
    }
}

/// Returns the separator used between arguments when not using an arity-template path.
pub fn intent_function_arg_separator(intent_name: &str, fixity: &str, arg_count: usize) -> String {
    let verbosity = get_verbosity_pref();
    let Some(mapping) = fixity_mapping(intent_name, fixity, &verbosity) else { return ",".to_string(); };
    if is_arity_mode(&mapping) {
        // On an exact arity match the glue words are inserted individually (no separator).
        return if arity_glue(&mapping, arg_count).is_some() { String::new() } else { ",".to_string() };
    }
    if arg_count == 2
        && let Some(separator) = binary_separator(&mapping)
        && !separator.eq_ignore_ascii_case("comma")
    {
        return separator.to_string();
    }
    ",".to_string()
}

/// Returns true when the function-intent should use the arity-template word order instead of
/// a generic `of`/separator pattern.
pub fn intent_function_use_arity_path(intent_name: &str, fixity: &str, arg_count: usize) -> bool {
    intent_function_has_arity_match(intent_name, fixity, arg_count)
}

/// The word inserted before a function's final argument (e.g. English "of"), from definitions.
fn function_application_word() -> String {
    crate::definitions::SPEECH_DEFINITIONS.with(|definitions| {
        let definitions = definitions.borrow();
        definitions.get_vec("FunctionApplicationWord")
            .and_then(|words| words.first().cloned())
            .unwrap_or_default()
    })
}



// intent             := self-property-list | expression
// self-property-list := property+ S    
// expression         := S ( term property* | application ) S 
// term               := concept-or-literal | number | reference 
// concept-or-literal := NCName
// number             := '-'? \d+ ( '.' \d+ )?
// reference          := '$' NCName
// application        := expression '(' arguments? S ')'
// arguments          := expression ( ',' expression )*
// property           := S ':' NCName
// S                  := [ \t\n\r]*

// The practical restrictions of NCName are that it cannot contain several symbol characters like
//  !, ", #, $, %, &, ', (, ), *, +, ,, /, :, ;, <, =, >, ?, @, [, \, ], ^, `, {, |, }, ~, and whitespace characters
//  Furthermore an NCName cannot begin with a number, dot or minus character although they can appear later in an NCName.
// NC_NAME defined in www.w3.org/TR/REC-xml/#sec-common-syn, but is complicated
//   We follow NC_NAME for the basic latin block, but then allow everything
static CONCEPT_OR_LITERAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^[^\s\u{0}-\u{40}\[\\\]^`\u{7B}-\u{BF}][^\s\u{0}-\u{2C}/:;<=>?@\[\\\]^`\u{7B}-\u{BF}]*"#     // NC_NAME but simpler
    ).unwrap()
});
static PROPERTY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^:[^\s\u{0}-\u{40}\[\\\]^`\u{7B}-\u{BF}][^\s\u{0}-\u{2C}/:;<=>?@\[\\\]^`\u{7B}-\u{BF}]*"#    // : NC_NAME
    ).unwrap()
});
static ARG_REF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\$[^\s\u{0}-\u{40}\[\\\]^`\u{7B}-\u{BF}][^\s\u{0}-\u{2C}/:;<=>?@\[\\\]^`\u{7B}-\u{BF}]*"#   // $ NC_NAME
    ).unwrap()
});
static NUMBER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^-?[0-9]+(\.[0-9]+)?"#).unwrap());

static TERMINALS_AS_U8: [u8; 3] = *b"(,)";
// static TERMINALS: [char; 3] = ['(', ',',')'];

// 'i -- "i" for the lifetime of the INTENT_ATTR string
#[derive(Debug, PartialEq, Eq, Clone)]
enum Token<'i> {
    Terminal(&'i str),  // "(", ",", ")"
    Property(&'i str),
    ArgRef(&'i str),
    ConceptOrLiteral(&'i str),
    Number(&'i str),
    None,               // out of characters
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}",
            match self {
                Token::Terminal(str) => format!("Terminal('{str}')"),
                Token::Property(str) => format!("Property({str})"),
                Token::ArgRef(str) => format!("ArgRef({str})"),
                Token::ConceptOrLiteral(str) => format!("Literal({str})"),
                Token::Number(str) => format!("Number({str})"),
                Token::None => "None".to_string(),
            }
        );
    }
}

impl Token<'_> {
    fn is_terminal(&self, terminal: &str) -> bool {
        if let Token::Terminal(value) = *self {
            return value == terminal;
        } else {
            return false;
        }
    }

    fn as_str(&self) -> &str {
        return match self {
            Token::Terminal(str) => str,
            Token::Property(str) => str,
            Token::ArgRef(str) => str,
            Token::ConceptOrLiteral(str) => str,
            Token::Number(str) => str,
            Token::None => "",
        }
    }
}

struct LexState<'i> {
    token: Token<'i>,
    remaining_str: &'i str,     // always trimmed
}

impl fmt::Display for LexState<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return writeln!(f, "token: {}, remaining: '{}'", self.token, self.remaining_str);
    }
}

impl<'i> LexState<'i> {
    fn init(str: &'i str) -> Result<LexState<'i>> {
        let mut lex_state = LexState {  token: Token::None, remaining_str: str.trim() };
        lex_state.get_next()?;
        return Ok(lex_state);
    }

    // helper function for LexState -- do not call outside of the impl
    fn set_token(&mut self, str: &'i str) -> Result<()> {
        // Note: 'str' is already trimmed
        if str.is_empty() {
            self.token = Token::None;
        } else if TERMINALS_AS_U8.contains(&str.as_bytes()[0]) {
            self.token = Token::Terminal(str);
        } else if let Some(matched_property) = PROPERTY.find(str) {
            self.token = Token::Property(matched_property.as_str());
        } else if let Some(matched_arg_ref) = ARG_REF.find(str) {
            self.token = Token::ArgRef(matched_arg_ref.as_str());
        } else if  let Some(matched_literal) = CONCEPT_OR_LITERAL.find(str) {
            self.token = Token::ConceptOrLiteral(matched_literal.as_str());
        } else if  let Some(matched_number) = NUMBER.find(str) {
            self.token = Token::Number(matched_number.as_str());
        } else {
            bail!("Illegal 'intent' syntax: {}", str);
        }
        return Ok( () );
    }

    fn get_next(&mut self) -> Result<&Token<'_>> {
        if self.remaining_str.is_empty() {
            self.token = Token::None;
        } else if TERMINALS_AS_U8.contains(&self.remaining_str.as_bytes()[0]) {
            self.token = Token::Terminal(&self.remaining_str[..1]);
            self.remaining_str = self.remaining_str[1..].trim_start();
        } else {
            self.set_token(self.remaining_str)?;
            self.remaining_str = self.remaining_str[self.token.as_str().len()..].trim_start();
}    
        return Ok(&self.token);
    }

    fn is_terminal(&self, terminal: &str) -> bool {
        return self.token.is_terminal(terminal);
    }
}

fn build_intent<'b, 'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
                                         lex_state: &mut LexState<'b>,
                                         mathml: Element<'c>,
                                         intent_offset: &mut u32) -> Result<Element<'m>> {
    // intent             := self-property-list | expression
    // self-property-list := property+ S    
    // expression         := S ( term property* | application ) S 
    // term               := concept-or-literal | number | reference 
    // concept-or-literal := NCName
    // number             := '-'? \d+ ( '.' \d+ )?
    // reference          := '$' NCName
    // application        := expression '(' arguments? S ')'
    //
    // When we flatten intent we have this implementation looking for Tokens or '(' [for application]
    // Essentially, the grammar we deal with here is:
    // intent := property+ | (concept-or-literal | number | reference) property* '('?
    // debug!("  start build_intent: state: {}", lex_state);
    let doc = rules_with_context.get_document();
    let mut intent;
    // debug!("    build_intent: start mathml name={}, intent_offset={}", name(mathml), intent_offset);
    match lex_state.token {
        Token::Property(_) => {
            // We only have a property -- we want to keep this tag/element
            // There are two paths:
            // 1. If there is a function call, then the children are dealt with there
            // 2. If there is *no* function call, then the children are kept, which means we return to pattern matching
            //    Note: to avoid infinite loop, we need to remove the 'intent' so we don't end up back here; we put it back later
            let properties = get_properties(lex_state)?;    // advance state to see if funcall
            if lex_state.is_terminal("(") {
                intent = create_mathml_element(&doc, as_str!(name(mathml)));
                intent.set_attribute_value(INTENT_PROPERTY, &properties);
                intent.set_attribute_value(MATHML_FROM_NAME_ATTR, as_str!(name(mathml)));
                intent.set_attribute_value("id", as_str!(mathml.attribute_value("id")
                      .ok_or_else(|| anyhow!("no id on intent function name"))?));
            } else {
                let saved_intent = mathml.attribute_value(INTENT_ATTR).unwrap();
                mathml.remove_attribute(INTENT_ATTR);
                mathml.set_attribute_value(INTENT_PROPERTY, &properties);   // needs to be set before the pattern match
                intent = rules_with_context.match_pattern::<Element<'m>>(mathml)?;
                // debug!("Intent after pattern match:\n{}", mml_to_string(intent));
                mathml.set_attribute_value(INTENT_ATTR, as_str!(saved_intent));
            }
            add_fixity(intent);
            return Ok(intent);      // if we start with properties, then there can only be properties
        },
        Token::ConceptOrLiteral(word) | Token::Number(word) => {
            let (leaf_name, leaf_text) = if let Token::Number(_) = lex_state.token {
                ("mn", word)
                } else if let Token::ConceptOrLiteral(word) = lex_state.token && is_concept_name(lex_state.remaining_str) {
                    (word, if is_leaf(mathml) { as_str!(as_text(mathml)) } else { "" })
                }
                else {
                    ("mi", word)
                };
            intent = create_mathml_element(&doc, leaf_name);
            // if the str is part of a larger intent and not the head (e.g., "a" in "f($x, a)", but not the "f" in it), then it is "made up"
            // debug!("    Token::ConceptOrLiteral, word={}, leaf_name={}", word, leaf_name);
            let mathml_name = name(mathml);
            let raw_intent_attr = mathml.attribute_value(INTENT_ATTR);
            intent.set_attribute_value(MATHML_FROM_NAME_ATTR, 
                if word == raw_intent_attr.as_deref().unwrap_or_default() {as_str!(mathml_name)} else {leaf_name});
            intent.set_text(leaf_text);       // '-' and '_' get removed by the rules.
            if let Some(id) = mathml.attribute_value("id") {
               intent.set_attribute_value("id", &format!("{}-literal-{}", id, intent_offset));
               *intent_offset += 1;
            }
            lex_state.get_next()?;
            if let Token::Property(_) = lex_state.token {
                let properties = get_properties(lex_state)?;
                intent.set_attribute_value(INTENT_PROPERTY, &properties);
            }
        },
        Token::ArgRef(word) => {
            intent = match find_arg(rules_with_context, &word[1..], mathml, intent_offset, true, false)? {
                Some(e) => {
                    lex_state.get_next()?;
                    e
                },
                None => bail!("intent arg '{}' not found", word),
            };
            if let Token::Property(_) = lex_state.token {
                let properties = get_properties(lex_state)?;
                intent.set_attribute_value(INTENT_PROPERTY, &properties);
            }
        },
        _ => bail!("Illegal 'intent' syntax: found {}", lex_state.token),
    };
    if lex_state.is_terminal("(") {
        intent = build_function(intent, rules_with_context, lex_state, mathml, intent_offset)?;
        // Arg refs like `$op($arg)` resolve `op`'s intent as a leaf first (so `nofix=` may
        // already be set). Once arguments are attached, drop `:nofix:` so `add_fixity`
        // / the default function path can apply (e.g. `probability` → "probability of x").
        clear_nofix_property(intent);
    }
    // debug!("    end build_intent: state: {}     piece:\n{}", lex_state, mml_to_string(intent));
    add_fixity(intent);
    return Ok(intent);
}

/// Remove `:nofix:` from `data-intent-property` (used after a leaf concept becomes a function head).
fn clear_nofix_property(intent: Element) {
    let Some(raw) = intent.attribute_value(INTENT_PROPERTY) else { return; };
    if !raw.split(':').any(|p| p == "nofix") {
        return;
    }
    let mut cleaned = String::from(":");
    for part in raw.split(':') {
        if !part.is_empty() && part != "nofix" {
            cleaned.push_str(part);
            cleaned.push(':');
        }
    }
    if cleaned == ":" {
        intent.remove_attribute(INTENT_PROPERTY);
    } else {
        intent.set_attribute_value(INTENT_PROPERTY, &cleaned);
    }
}

fn is_concept_name(s: &str) -> bool {
    let mut chars = s.chars();
    
    if let Some(':') = chars.next() {
        // Use find to consume all ASCII letters. 
        // The first character that FAILS the condition is returned by find.
        match chars.by_ref().find(|c| !c.is_ascii_alphabetic()) {
            // If find returns None, it means the rest of the string was entirely letters
            None => true, 
            // If it found a non-letter, that character IS the "next char" to check
            Some(next_c) => matches!(next_c, '(' | ',' | ')'),
        }
    } else {
        // Reset iterator if it doesn't start with ':'
        let mut chars = s.chars();
        match chars.next() {
            None => true,
            Some(c) => matches!(c, '(' | ',' | ')'),
        }
    }
}

pub const INTENT_PROPERTY: &str = "data-intent-property";

/// Get all the properties, stopping we don't have any more
/// Returns the string of the properties terminated with an additional ":"
fn get_properties(lex_state: &mut LexState) -> Result<String> {
    // return the 'hint' leaving the state
    assert!(matches!(lex_state.token, Token::Property(str) if str.starts_with(':')));
    let mut properties = String::with_capacity(60);
    properties.push_str(lex_state.token.as_str());
    loop {
        let token = lex_state.get_next()?;
        if let Token::Property(property) = token {
            properties.push_str(property);
        } else {
            properties.push(':');
            // debug!("      get_properties: returns {}", properties);
            return Ok(simplify_fixity_properties(&properties));
        }
    }
}

/// Build a function 'f(...)' where '...' can be empty
///
/// Also handles nested functions like f(...)(...)
/// 
/// Start state: at '('
/// 
/// End state: after ')'
fn build_function<'b, 'r, 'c, 's:'c, 'm:'c>(
            function_name: Element<'m>,
            rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
            lex_state: &mut LexState<'b>,
            mathml: Element<'c>,
            intent_offset: &mut u32) -> Result<Element<'m>> {
    // debug!("  start build_function: name: {}, state: {}", name(function_name), lex_state);
    // application := intent '(' arguments? S ')'  where 'function_name' is 'intent'
    assert!(lex_state.is_terminal("("));
    let mut function = function_name;
    function.set_attribute_value(MATHML_FROM_NAME_ATTR, as_str!(name(mathml)));
    while lex_state.is_terminal("(") {
        lex_state.get_next()?;
        if lex_state.is_terminal(")") {
            // grammar requires at least one argument
            bail!("Illegal 'intent' syntax: missing argument for intent name '{}'", name(function_name));
        }
        let children = build_arguments(rules_with_context, lex_state, mathml, intent_offset)?;
        function = lift_function_name(rules_with_context.get_document(), function, children);

        if !lex_state.is_terminal(")") {
            bail!("Illegal 'intent' syntax: missing ')' for intent name '{}'", name(function_name));
        }
        lex_state.get_next()?;
    }

    // debug!("  end build_function/# children: {}, #state: {}  ..[bfa] function name: {}",
    //     function.children().len(), lex_state, mml_to_string(function));
    return Ok(function);
}

// process all the args of a function
// Start state: after '('
// End state: on ')'
fn build_arguments<'b, 'r, 'c, 's:'c, 'm:'c>(
            rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
            lex_state: &mut LexState<'b>,
            mathml: Element<'c>,
            intent_offset: &mut u32) -> Result<Vec<Element<'m>>> {
    // arguments := intent ( ',' intent )*' 
    // debug!("    start build_args state: {}", lex_state);

    // there is at least one arg
    let mut children = Vec::with_capacity(lex_state.remaining_str.len()/3 + 1);   // conservative estimate ('3' - "$x,");
    children.push( build_intent(rules_with_context, lex_state, mathml, intent_offset)? );   // arg before ','
    // debug!("  build_args: # children {};  state: {}", children.len(), lex_state);

    while lex_state.is_terminal(",") {
        lex_state.get_next()?;
        children.push( build_intent(rules_with_context, lex_state, mathml, intent_offset)? );   // arg before ','
        // debug!("    build_args, # children {};  state: {}", children.len(), lex_state);
    }

    // debug!("    end build_args, # children {};  state: {}", children.len(), lex_state);
    return Ok(children);
}

/// lift the children up to LITERAL_NAME
fn lift_function_name<'m>(doc: Document<'m>, function_name: Element<'m>, children: Vec<Element<'m>>) -> Element<'m> {
    // debug!("    lift_function_name: {} ({} children)", name(function_name), children.len());
    // debug!("    lift_function_name: {}", mml_to_string(function_name));
    if name(function_name) == "mi" || name(function_name) == "mn" {   // FIX -- really want to test for all leaves, but not "data-from-mathml"
        // simple/normal case of f(x,y)
        // don't want to say that this is a leaf -- doing so messes up because it potentially has children
        set_mathml_name(function_name, as_str!(as_text(function_name)));
        function_name.set_text("");
        function_name.replace_children(children);
        if name(function_name).find(|ch: char| ch!='_' && ch!='-').is_none() {
            let properties = function_name.attribute_value(INTENT_PROPERTY).as_deref().unwrap_or(":").to_owned();
            function_name.set_attribute_value(INTENT_PROPERTY, &(properties + "silent:"));
        }
        return function_name;
    } else if function_name.children().is_empty() ||
              (function_name.children().len() == 1 && matches!(function_name.children()[0], ChildOfElement::Text(_))) {
        // "...  :property(...)" -- no function name
        function_name.replace_children(children);
        return function_name;
    } else {
        // more complicated case of nested name: f(x)(y,z)
        // create an apply_function(f(x), y, z)
        let result = create_mathml_element(&doc, IMPLICIT_FUNCTION_NAME);
        result.set_attribute_value(MATHML_FROM_NAME_ATTR, "mrow");
        result.append_child(function_name);
        result.append_children(children);
        return result;
    }
}


/// look for @arg=name in mathml
/// if 'check_intent', then look at an @intent for this element (typically false for non-recursive calls)
fn find_arg<'r, 'c, 's:'c, 'm:'c>(
    rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
    name: &str,
    mathml: Element<'c>,
    intent_offset: &mut u32,
    skip_self: bool,
    no_check_inside: bool) -> Result<Option<Element<'m>>> {
    // debug!("Looking for '{}' in\n{}", name, mml_to_string(mathml));
    if !skip_self &&
        let Some(arg_val) = mathml.attribute_value("arg") {
            // debug!("looking for '{}', found arg='{}'", name, arg_val);
            if name == arg_val {
                // check to see if this mathml has an intent value -- if so the value is the value of its intent value
                if let Some(intent_str) = mathml.attribute_value(INTENT_ATTR) {
                    let mut lex_state = LexState::init(intent_str.trim())?;
                    return Ok( Some( build_intent(rules_with_context, &mut lex_state, mathml, intent_offset)? ) );
                } else {
                    return Ok( Some( rules_with_context.match_pattern::<Element<'m>>(mathml)? ) );
                }
            } else if no_check_inside {
                return Ok(None);       // don't look inside 'arg'
            }
        }

    if no_check_inside && mathml.attribute_value(INTENT_ATTR).is_some() {
        return Ok(None);           // don't look inside 'intent'
    }

    if is_leaf(mathml){
        return Ok(None);
    }

    for child in mathml.children() {
        let child = as_element(child);
        if let Some(element) = find_arg(rules_with_context, name, child, intent_offset, false, true)? {
            return Ok( Some(element) );
        }
    }

    return Ok(None);               // not present
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::init_logger;
    use crate::errors::Result;
    #[allow(unused_imports)]
    use log::debug;
    use sxd_document_no_unsafe::parser;
    use std::panic::{catch_unwind, AssertUnwindSafe};


    fn test_intent(mathml: &str, target: &str, intent_error_recovery: &str) -> bool {
		use crate::interface::*;
        use crate::pretty_print::mml_to_string;
		// this forces initialization
        init_panic_handler();
        let result = catch_unwind(AssertUnwindSafe(|| {
            crate::interface::set_rules_dir(super::super::abs_rules_dir_path()).unwrap();
            set_preference("Language", "en").unwrap();
            set_preference("IntentErrorRecovery", intent_error_recovery).unwrap();
            set_preference("SpeechStyle", "SimpleSpeak").unwrap();      // avoids possibility of "LiteralSpeak"
            let package1 = &parser::parse(mathml).expect("Failed to parse test input");
            let mathml = get_element(package1);
            trim_element(mathml, false);
            // debug!("test:\n{}", mml_to_string(mathml));
            
            let package2 = &parser::parse(target).expect("Failed to parse target input");
            let target = get_element(package2);
            trim_element(target,true);
            // debug!("target:\n{}", mml_to_string(target));

            match crate::speech::intent_from_mathml(mathml, package2.as_document()) {
                Ok(_result) => {
                    // debug!("result:\n{}", mml_to_string(_result));
                    Ok(())
                },
                Err(e) => {
                    panic!("Error in intent: {}\nMathML: {}", e, mml_to_string(mathml));
                }
            }
        }));
        match crate::interface::report_any_panic(result) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        }
    }

    #[test]
    fn infer_binomial() -> Result<()> {
        let mathml = "<mrow intent='binomial($n, $m)'>
                <mo>(</mo>
                <mfrac linethickness='0'> <mn arg='n'>7</mn> <mn arg='m'>3</mn> </mfrac>
                <mo>)</mo>
            </mrow>";
        let intent = "<binomial data-from-mathml='mrow' data-intent-property=':infix:'> <mn data-from-mathml='mn' arg='n'>7</mn> <mn data-from-mathml='mn' arg='m'>3</mn>  </binomial>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn infer_binomial_intent_arg() -> Result<()> {
        let mathml = "<msubsup intent='$op($n,$m)'>
                <mi arg='op' intent='binomial'>C</mi>
                <mi arg='n'>n</mi>
                <mi arg='m'>m</mi>
            </msubsup>";
        let intent = "<binomial data-from-mathml='msubsup' data-intent-property=':infix:'> <mi data-from-mathml='mi' arg='n'>n</mi> <mi data-from-mathml='mi' arg='m'>m</mi></binomial>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn parse_intent_mapping_forms() -> Result<()> {
        use super::{parse_fixity_mapping, ratio_style_pair, pick_verbosity, binary_separator, is_arity_mode, arity_glue};

        // Ratio-style: verbosity selects a whole `name : glue` pair; glue acts as a binary separator.
        let ratio = "ratio : to | the ratio : to | the ratio : to";
        assert_eq!(ratio_style_pair(ratio, "Terse"), Some(("ratio".to_string(), "to".to_string())));
        assert_eq!(ratio_style_pair(ratio, "Medium"), Some(("the ratio".to_string(), "to".to_string())));
        let ratio_terse = parse_fixity_mapping(ratio, "Terse");
        assert_eq!(ratio_terse.name, "ratio");
        assert_eq!(binary_separator(&ratio_terse), Some("to"));
        assert!(!is_arity_mode(&ratio_terse));
        assert_eq!(arity_glue(&ratio_terse, 2), None);

        // `|` selects terse/medium/verbose within one piece.
        assert_eq!(pick_verbosity("sin | sine", "Terse"), "sin");
        assert_eq!(pick_verbosity("sin | sine", "Medium"), "sine");

        // Arity template: `:`-separated glue options, `,`-separated glue words imply arity.
        let sum = parse_fixity_mapping("sum: over: from,to", "Medium");
        assert_eq!(sum.name, "sum");
        assert_eq!(sum.glue, vec!["over".to_string(), "from,to".to_string()]);
        assert!(is_arity_mode(&sum));
        assert_eq!(arity_glue(&sum, 3), Some(vec!["from", "to"]));
        assert_eq!(arity_glue(&sum, 1), None);       // no zero-word option -> falls back to "of"

        // Binary separator: a single non-comma glue option.
        let quotient = parse_fixity_mapping("integer part: divided by", "Medium");
        assert_eq!(quotient.name, "integer part");
        assert_eq!(binary_separator(&quotient), Some("divided by"));
        assert!(!is_arity_mode(&quotient));

        // Bracketing: `open; name; close`, with `|` verbosity inside the name.
        let norm = parse_fixity_mapping("; norm| norm| norm; end norm", "Verbose");
        assert_eq!((norm.open.as_str(), norm.name.as_str(), norm.close.as_str()), ("", "norm", "end norm"));
        Ok(())
    }

    #[test]
    fn silent_underscore() -> Result<()> {
        let mathml = "<mrow><mi intent='__-'>silent</mi><mo>+</mo><mi>e</mi></mrow>";
        let intent = "<mrow data-from-mathml='mrow'>
                                <mi data-from-mathml='mi'>__-</mi>
                                <mo data-from-mathml='mo'>+</mo>
                                <mi data-from-mathml='mi'>e</mi>
                            </mrow>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }


    #[test]
    fn silent_underscore_function() -> Result<()> {
        let mathml = "<mrow intent='__-_(speak, this)'></mrow>";
        let intent = "<__-_ data-from-mathml='mrow' data-intent-property=':silent:'>
                                <mi data-from-mathml='mi'>speak</mi>
                                <mi data-from-mathml='mi'>this</mi>
                            </__-_>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn nofix_intent_trivial() -> Result<()> {
        // Authors write bare concept names; IntentMappings `nofix=` supplies the leaf fixity.
        let mathml = "<mi intent='set-of-integers'>ℤ</mi>";
        let intent = "<set-of-integers data-from-mathml='mi' data-intent-property=':nofix:'>ℤ</set-of-integers>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }


    #[test]
    fn nofix_intent_args() -> Result<()> {
        let mathml = "<mi intent='foo:nofix(first, second)'>foo content</mi>";
        let intent = r#"<foo id='Mfo9mib6-0-literal-0' data-intent-property=':nofix:' data-from-mathml='mi'>
                <first data-from-mathml='first' id='Mfo9mib6-0-literal-1'>ℤ</first>
                <second data-from-mathml='second' id='Mfo9mib6-0-literal-2'>ℤ</second>
            </foo>"#;
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_multiple_properties() -> Result<()> {
        let mathml = "<mrow intent='foo:silent:int(bar:positive-int:int, $a:foo:bar:foo-bar, $b:number)'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b' intent=':negative-int:int'>b</mi>
            </mrow>";
        let intent = "<foo data-intent-property=':int:silent:' data-from-mathml='mrow'>
                                <mi data-from-mathml='mi' data-intent-property=':positive-int:int:'>bar</mi>
                                <mi data-from-mathml='mi' arg='a' data-intent-property=':foo:bar:foo-bar:'>a</mi>
                                <mi data-from-mathml='mi' arg='b' data-intent-property=':number:'>b</mi>
                            </foo>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }
    #[test]
    fn intent_nest_no_arg_call() -> Result<()> {
        let mathml = "<mrow intent='foo(bar())'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<foo><bar></bar></foo>";
        assert!(!test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_hints() -> Result<()> {
        let mathml = "<mrow intent='foo:silent(bar:postfix(3))'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<foo data-intent-property=':silent:' data-from-mathml='mrow'>
                                <bar data-intent-property=':postfix:' data-from-mathml='mrow'>
                                    <mn data-from-mathml='mn'>3</mn>
                                </bar>
                            </foo>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }
    
    #[test]
    fn intent_hints_and_type() -> Result<()> {
        let mathml = "<mrow intent='foo:is-foolish:function($b)'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi intent='b:int' arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<foo data-intent-property=':is-foolish:function:' data-from-mathml='mrow'>
                                <mi data-intent-property=':int:' data-from-mathml='mi'>b</mi>
                            </foo>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_in_intent_first_arg() -> Result<()> {
        let mathml = "<mrow intent='p(f(b), a)'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<p data-from-mathml='mrow'>
                                <f data-from-mathml='mrow'>
                                    <mi data-from-mathml='mi'>b</mi>
                                </f>
                                <mi data-from-mathml='mi'>a</mi>
                            </p>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_in_intent_second_arg() -> Result<()> {
        let mathml = "<mrow intent='$p(a,$f(b))'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<plus data-from-mathml='mrow' data-intent-property=':infix:'>
                                <mi data-from-mathml='mi'>a</mi>
                                <factorial data-from-mathml='mrow'>
                                    <mi data-from-mathml='mi'>b</mi>
                                </factorial>
                            </plus>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_with_whitespace() -> Result<()> {
        let mathml = "<mrow intent='  $arrow    ( $a ,  $b,$c )  '>
                <mi arg='a'>A</mi>
                <mover>
                    <mo movablelimits='false' arg='arrow' intent='map'>⟶</mo>
                    <mo arg='U2245' intent='congruence'>≅</mo>
                </mover>
                <mi arg='b'>B</mi>
                <mi arg='c'>C</mi>
            </mrow>";
        let intent = "<map data-from-mathml='mrow'> <mi data-from-mathml='mi' arg='a'>A</mi> <mi data-from-mathml='mi' arg='b'>B</mi> <mi data-from-mathml='mi' arg='c'>C</mi> </map>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_template_at_toplevel() -> Result<()> {
        let mathml = "<msup intent='$H $n'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
            </msup>";
        let intent = "<mrow><mi arg='H' mathvariant='normal'>H</mi><mn arg='n'>2</mn></mrow>";
        assert!(!test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_with_nested_indirect_head() -> Result<()> {
        let mathml = "<mrow intent='$op($a,$b)'>
                <mi arg='a'>A</mi>
                <mover arg='op' intent='$ra($cong)'>
                    <mo movablelimits='false' arg='ra' intent='map'>⟶</mo>
                    <mo arg='cong' intent='congruence'>≅</mo>
                </mover>
                <mi arg='b'>B</mi>
            </mrow>";
        let intent = "<apply-function data-from-mathml='mrow'>
                                <map data-from-mathml='mrow'>
                                    <mi data-from-mathml='mo'>congruence</mi>
                                </map>
                                <mi data-from-mathml='mi' arg='a'>A</mi>
                                <mi data-from-mathml='mi' arg='b'>B</mi>
                            </apply-function>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_with_literals() -> Result<()> {
        let mathml = "<mrow intent='vector(1, 0.0, 0.1, -23, -0.1234, last)'>
                <mi>x</mi>
            </mrow>";
        let intent = "<vector data-from-mathml='mrow' data-intent-property=':function:'>
                                <mn data-from-mathml='mn'>1</mn>
                                <mn data-from-mathml='mn'>0.0</mn>
                                <mn data-from-mathml='mn'>0.1</mn>
                                <mn data-from-mathml='mn'>-23</mn>
                                <mn data-from-mathml='mn'>-0.1234</mn>
                                <mi data-from-mathml='mi'>last</mi>
                            </vector>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_with_template_literals() -> Result<()> {
        let mathml = "<mrow intent='1 0.0 0.1 -23 -0.1234 last'>
                <mi>x</mi>
            </mrow>";
        let intent = "<mrow><mn>1</mn><mn>0.</mn><mn>.1</mn><mn>-23</mn><mn>-.1234</mn><mi>last</mi></mrow>";
        assert!(!test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_with_nested_head() -> Result<()> {
        let mathml = "<mrow intent='$ra($cong)($a,$b)'>
                <mi arg='a'>A</mi>
                <mover>
                    <mo movablelimits='false' arg='ra' intent='map'>⟶</mo>
                    <mo arg='cong' intent='congruence'>≅</mo>
                </mover>
                <mi arg='b'>B</mi>
            </mrow>";
        let intent = "<apply-function data-from-mathml='mrow'>
                                <map data-from-mathml='mrow'>
                                    <mi data-from-mathml='mo'>congruence</mi>
                                </map>
                                <mi data-from-mathml='mi' arg='a'>A</mi>
                                <mi data-from-mathml='mi' arg='b'>B</mi>
                            </apply-function>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }


    #[test]
    fn intent_with_nested_head_and_hints() -> Result<()> {
        let mathml = "<mrow intent='pre:prefix(in:infix($a, x))(post:postfix($b))'>
                <mi arg='a'>A</mi>
                <mover>
                    <mo intent='map'>⟶</mo>
                    <mo intent='congruence'>≅</mo>
                </mover>
                <mi arg='b'>B</mi>
            </mrow>";
        let intent = "<apply-function data-from-mathml='mrow'>
                <pre data-intent-property=':prefix:' data-from-mathml='mrow'>
                    <in data-intent-property=':infix:' data-from-mathml='mrow'>
                        <mi data-from-mathml='mi' arg='a'>A</mi>
                        <mi data-from-mathml='mi'>x</mi>
                    </in>
                </pre>
                <post data-intent-property=':postfix:' data-from-mathml='mrow'>
                    <mi data-from-mathml='mi' arg='b'>B</mi>
                </post>
            </apply-function>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }


    #[test]
    fn intent_double_indirect_head() -> Result<()> {
        let mathml = "<mrow intent='$m:prefix($c)($a,$b)'>
                <mi arg='a'>A</mi>
                <mover>
                    <mo movablelimits='false' arg='m' intent='map'>⟶</mo>
                    <mo arg='c' intent='congruence'>≅</mo>
                </mover>
                <mi arg='b'>B</mi>
            </mrow>";
        let intent = "<apply-function data-from-mathml='mrow'>
                                <map data-intent-property=':prefix:' data-from-mathml='mrow'>
                                    <mi data-from-mathml='mo'>congruence</mi>
                                </map>
                                <mi data-from-mathml='mi' arg='a'>A</mi>
                                <mi data-from-mathml='mi' arg='b'>B</mi>
                            </apply-function>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_missing_open() -> Result<()> {
        let mathml = "<mrow intent='$p $a,$f($b))'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<plus> <mi arg='a'>a</mi> <factorial><mi arg='b'>b</mi></factorial> </plus>";
        assert!(!test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_no_comma() -> Result<()> {
        let mathml = "<mrow intent='$p($a $f($b))'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<plus>
                <mrow>
                    <mi arg='a'>a</mi>
                    <factorial> <mi arg='b'>b</mi> </factorial>
                </mrow>
            </plus>";
        assert!(!test_intent(mathml, intent, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_no_arg() -> Result<()> {
        let mathml = "<mrow intent='factorial()'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let target = "<factorial></factorial>";
        assert!(!test_intent(mathml, target, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_illegal_no_arg() -> Result<()> {
        let mathml = "<mrow intent='factorial()'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let target = "<factorial></factorial>";
        assert!(!test_intent(mathml, target, "Error"));
        return Ok(());
    }

    #[test]
    fn intent_illegal_no_arg_ignore() -> Result<()> {
        let mathml = "<mrow intent='factorial()'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let target = "<mrow data-from-mathml='mrow' intent='factorial()'>
                                <mi data-from-mathml='mi' arg='a'>a</mi>
                                <mi data-from-mathml='mo'>plus</mi>
                                <mi data-from-mathml='mi' arg='b'>b</mi>
                                <mi data-from-mathml='mo'>factorial</mi>
                            </mrow>";
        assert!(test_intent(mathml, target, "IgnoreIntent"));
        return Ok(());
    }

    #[test]
    fn intent_illegal_self_ref() -> Result<()> {
        let mathml = "<mrow intent='foo:is-foolish:function($b)'>
                <mi intent='$b:int' arg='b'>b</mi>
            </mrow>";
        let target = "<foo data-intent-property=':function:' data-intent-type='is-foolish'><mi data-intent-type='int'>b</mi></foo>";
        assert!(!test_intent(mathml, target, "Error"));
        return Ok(());
    }

    #[test]
    fn infer_missing_second_arg() -> Result<()> {
        let mathml = "<mrow intent='binomial($n,)'>
                <mo>(</mo>
                <mfrac linethickness='0'> <mn arg='n'>7</mn> <mn arg='m'>3</mn> </mfrac>
                <mo>)</mo>
            </mrow>";
        let target = "<binomial data-intent-property='binomial($n,)'> \n
                             <mn data-from-mathml='mn' arg='n'>7</mn> <mn data-from-mathml='mn' arg='m'>3</mn>  </binomial>";
        assert!(!test_intent(mathml, target, "Error"));
        return Ok(());
    }

    #[test]
    fn infer_missing_second_arg_ignore() -> Result<()> {
        let mathml = "<mrow intent='binomial($n,)'>
                <mo>(</mo>
                <mfrac linethickness='0'> <mn arg='n'>7</mn> <mn arg='m'>3</mn> </mfrac>
                <mo>)</mo>
            </mrow>";
        let target = "<mrow data-from-mathml='mrow' intent='binomial($n,)'>
                <mo data-from-mathml='mo'>(</mo>
                <fraction data-from-mathml='mfrac' linethickness='0'> <mn data-from-mathml='mn' arg='n'>7</mn> <mn data-from-mathml='mn' arg='m'>3</mn> </fraction>
                <mo data-from-mathml='mo'>)</mo>
            </mrow>";
            assert!(test_intent(mathml, target, "IgnoreIntent"));
        return Ok(());
    }   

    #[test]
    fn plane1_char_in_concept_name() -> Result<()> {
        let mathml = "<math><mrow><mo intent='🐇'>&#x1F407;</mo><mi>X</mi></mrow></math>";
        let intent = "<math data-from-mathml='math'>
                                <mrow data-from-mathml='mrow'>
                                    <mi data-from-mathml='mo'>🐇</mi>
                                    <mi data-from-mathml='mi'>X</mi>
                                </mrow>
                            </math>";
        assert!(test_intent(mathml, intent, "Error"));
        return Ok(());
    }   
}
