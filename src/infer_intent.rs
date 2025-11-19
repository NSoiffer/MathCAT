//! Use heuristics to infer the intent.
//! For example, an `mfrac` with `linethickness=0` would be a binomial
//! The inference is added to the MathML
//!
//! The implementation of the module is on hold until the MathML committee figures out how it wants to do this.
#![allow(clippy::needless_return)]

use sxd_document::dom::*;
use crate::prefs::PreferenceManager;
use crate::speech::SpeechRulesWithContext;
use crate::canonicalize::{as_element, as_text, name, create_mathml_element, set_mathml_name, INTENT_ATTR, MATHML_FROM_NAME_ATTR};
use crate::errors::*;
use std::fmt;
use crate::pretty_print::mml_to_string;
use crate::xpath_functions::is_leaf;
use regex::Regex;
use phf::phf_set;

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
                                            .chain_err(|| "Pattern match/replacement failure!") {
                    Err(e) => Err(e),
                    Ok(intent) => {
                        intent.set_attribute_value(INTENT_ATTR, saved_intent_attr); //  so attr can be potentially be viewed later
                        Ok(intent)
                    },
                };
                mathml.set_attribute_value(INTENT_ATTR, saved_intent_attr);
                return intent_tree;
            }
        }
    }

    fn catch_errors_building_intent<'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, mathml: Element<'c>) -> Result<Element<'m>> {
        if let Some(intent_str) = mathml.attribute_value(INTENT_ATTR) {
            // debug!("Before intent: {}", crate::pretty_print::mml_to_string(mathml));
            let mut lex_state = LexState::init(intent_str.trim())?;
            let result = build_intent(rules_with_context, &mut lex_state, mathml)
                        .chain_err(|| format!("occurs before '{}' in intent attribute value '{}'", lex_state.remaining_str, intent_str))?;
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
    "function", "infix", "prefix", "postfix", "silent", "other",
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
    let properties = intent.attribute_value(INTENT_PROPERTY).unwrap_or_default();
    if properties.split(":").all(|property| !FIXITIES.contains(property)) {
        let intent_name = name(intent);
        crate::definitions::SPEECH_DEFINITIONS.with(|definitions| {
            let definitions = definitions.borrow();
            if let Some(definition) = definitions.get_hashmap("IntentMappings").unwrap().get(intent_name) {
                if let Some((fixity, _)) = definition.split_once("=") {
                    let new_properties = (if properties.is_empty() {":"} else {properties}).to_string() + fixity + ":";
                    intent.set_attribute_value(INTENT_PROPERTY, &new_properties);
                    // debug!("Added fixity: new value '{}'", intent.attribute_value(INTENT_PROPERTY).unwrap());
                }
            };
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
        if mathml.attribute_value(MATHML_FROM_NAME_ATTR).unwrap_or_default() == "mtable" {
            return mathml;
        }
        let doc = mathml.document();
        let properties = mathml.attribute_value(INTENT_PROPERTY).unwrap_or_default();
        let fixity = properties.rsplit(':').find(|&property| FIXITIES.contains(property)).unwrap_or_default();
        let intent_name = name(mathml);
    
        let op_name_id = mathml.attribute_value("id").unwrap_or("new-id");
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
            element.set_attribute_value("id", &format!("{id}-{id_inc}"));
            element.set_attribute_value(MATHML_FROM_NAME_ATTR, "mo");
            return ChildOfElement::Element(element);
        }
    }
}

pub fn intent_speech_for_name(intent_name: &str, verbosity: &str, fixity: &str) -> String {
    crate::definitions::SPEECH_DEFINITIONS.with(|definitions| {
        let definitions = definitions.borrow();
        if let Some(intent_name_pattern) = definitions.get_hashmap("IntentMappings").unwrap().get(intent_name) {
            // Split the pattern is:
            //   fixity-def [|| fixity-def]*
            //   fixity-def := fixity=[open;] verbosity[; close]
            //   verbosity := terse | medium | verbose
            if let Some(matched_intent) = intent_name_pattern.split("||").find(|&entry| entry.trim().starts_with(fixity)) {
                let (_, matched_intent) = matched_intent.split_once("=").unwrap_or_default();
                let parts = matched_intent.trim().split(";").collect::<Vec<&str>>();
                let mut operator_names = (if parts.len() > 1 {parts[1]} else {parts[0]}).split(":").collect::<Vec<&str>>();
                match operator_names.len() {
                    1 => return operator_names[0].trim().to_string(),
                    2 | 3 => {
                        if operator_names.len() == 2 {
                            warn!("Intent '{intent_name}' has only two operator names, but should have three");
                            operator_names.push(operator_names[1]);
                        }
                        let intent_word = match verbosity {
                            "Terse" => operator_names[0],
                            "Medium" => operator_names[1],
                            _ => operator_names[2],
                        };
                        return intent_word.trim().to_string();
                    },
                    _ => {
                        error!("Intent '{}' has too many ({}) operator names, should only have 2", intent_name, operator_names.len());
                        return intent_name.to_string();
                    },
                }
            }
        };
        return intent_name.replace(['_', '-'], " ").trim().to_string();
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

lazy_static! {
    // The practical restrictions of NCName are that it cannot contain several symbol characters like
    //  !, ", #, $, %, &, ', (, ), *, +, ,, /, :, ;, <, =, >, ?, @, [, \, ], ^, `, {, |, }, ~, and whitespace characters
    //  Furthermore an NCName cannot begin with a number, dot or minus character although they can appear later in an NCName.
    // NC_NAME defined in www.w3.org/TR/REC-xml/#sec-common-syn, but is complicated
    //   We follow NC_NAME for the basic latin block, but then allow everything
    static ref CONCEPT_OR_LITERAL: Regex = Regex::new(
        r#"^[^\s\u{0}-\u{40}\[\\\]^`\u{7B}-\u{BF}][^\s\u{0}-\u{2C}/:;<=>?@\[\\\]^`\u{7B}-\u{BF}]*"#     // NC_NAME but simpler
    ).unwrap();
    static ref PROPERTY: Regex = Regex::new(
        r#"^:[^\s\u{0}-\u{40}\[\\\]^`\u{7B}-\u{BF}][^\s\u{0}-\u{2C}/:;<=>?@\[\\\]^`\u{7B}-\u{BF}]*"#    // : NC_NAME
    ).unwrap();
    static ref ARG_REF: Regex = Regex::new(
        r#"^\$[^\s\u{0}-\u{40}\[\\\]^`\u{7B}-\u{BF}][^\s\u{0}-\u{2C}/:;<=>?@\[\\\]^`\u{7B}-\u{BF}]*"#   // $ NC_NAME
    ).unwrap();
    static ref NUMBER: Regex = Regex::new(r#"^-?[0-9]+(\.[0-9]+)?"#).unwrap();
}

static TERMINALS_AS_U8: [u8; 3] = [b'(', b',', b')'];
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
                                         mathml: Element<'c>) -> Result<Element<'m>> {
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
    // debug!("    build_intent: start mathml name={}", name(mathml));
    match lex_state.token {
        Token::Property(_) => {
            // We only have a property -- we want to keep this tag/element
            // There are two paths:
            // 1. If there is a function call, then the children are dealt with there
            // 2. If there is *no* function call, then the children are kept, which means we return to pattern matching
            //    Note: to avoid infinite loop, we need to remove the 'intent' so we don't end up back here; we put it back later
            let properties = get_properties(lex_state)?;    // advance state to see if funcall
            if lex_state.is_terminal("(") {
                intent = create_mathml_element(&doc, name(mathml));
                intent.set_attribute_value(INTENT_PROPERTY, &properties);
                intent.set_attribute_value(MATHML_FROM_NAME_ATTR, name(mathml));
                intent.set_attribute_value("id", mathml.attribute_value("id")
                      .ok_or("no id on intent function name")?);
            } else {
                let saved_intent = mathml.attribute_value(INTENT_ATTR).unwrap();
                mathml.remove_attribute(INTENT_ATTR);
                mathml.set_attribute_value(INTENT_PROPERTY, &properties);   // needs to be set before the pattern match
                intent = rules_with_context.match_pattern::<Element<'m>>(mathml)?;
                // debug!("Intent after pattern match:\n{}", mml_to_string(intent));
                mathml.set_attribute_value(INTENT_ATTR, saved_intent);
            }
            add_fixity(intent);
            return Ok(intent);      // if we start with properties, then there can only be properties
        },
        Token::ConceptOrLiteral(word) | Token::Number(word) => {
            let leaf_name = if let Token::Number(_) = lex_state.token {"mn"} else {"mi"};
            intent = create_mathml_element(&doc, leaf_name);
            // if the str is part of a larger intent and not the head (e.g., "a" in "f($x, a)", but not the "f" in it), then it is "made up"
            // debug!("    Token::ConceptOrLiteral, word={}, leaf_name={}", word, leaf_name);
            intent.set_attribute_value(MATHML_FROM_NAME_ATTR, 
                if word == mathml.attribute_value(INTENT_ATTR).unwrap_or_default() {name(mathml)} else {leaf_name});
            intent.set_text(word);       // '-' and '_' get removed by the rules.
            if let Some(id) = mathml.attribute_value("id") {
                intent.set_attribute_value("id", id);
            }
            lex_state.get_next()?;
            if let Token::Property(_) = lex_state.token {
                let properties = get_properties(lex_state)?;
                intent.set_attribute_value(INTENT_PROPERTY, &properties);
            }
        },
        Token::ArgRef(word) => {
            intent = match find_arg(rules_with_context, &word[1..], mathml, true, false)? {
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
        intent = build_function(intent, rules_with_context, lex_state, mathml)?;
    }
    // debug!("    end build_intent: state: {}     piece: {}", lex_state, mml_to_string(intent));
    add_fixity(intent);
    return Ok(intent);
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
            mathml: Element<'c>) -> Result<Element<'m>> {
    // debug!("  start build_function: name: {}, state: {}", name(function_name), lex_state);
    // application := intent '(' arguments? S ')'  where 'function_name' is 'intent'
    assert!(lex_state.is_terminal("("));
    let mut function = function_name;
    function.set_attribute_value(MATHML_FROM_NAME_ATTR, name(mathml));
    while lex_state.is_terminal("(") {
        lex_state.get_next()?;
        if lex_state.is_terminal(")") {
            // grammar requires at least one argument
            bail!("Illegal 'intent' syntax: missing argument for intent name '{}'", name(function_name));
        }
        let children = build_arguments(rules_with_context, lex_state, mathml)?;
        function = lift_function_name(rules_with_context.get_document(), function, children);

        if !lex_state.is_terminal(")") {
            bail!("Illegal 'intent' syntax: missing ')' for intent name '{}'", name(function_name));
        }
        lex_state.get_next()?;
    }

    // debug!("  end build_function/# children: {}, #state: {}  ..[bfa] function name: {}",
        // function.children().len(), lex_state, mml_to_string(function));
    return Ok(function);
}

// process all the args of a function
// Start state: after '('
// End state: on ')'
fn build_arguments<'b, 'r, 'c, 's:'c, 'm:'c>(
            rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
            lex_state: &mut LexState<'b>,
            mathml: Element<'c>) -> Result<Vec<Element<'m>>> {
    // arguments := intent ( ',' intent )*' 
    // debug!("    start build_args state: {}", lex_state);

    // there is at least one arg
    let mut children = Vec::with_capacity(lex_state.remaining_str.len()/3 + 1);   // conservative estimate ('3' - "$x,");
    children.push( build_intent(rules_with_context, lex_state, mathml)? );   // arg before ','
    // debug!("  build_args: # children {};  state: {}", children.len(), lex_state);

    while lex_state.is_terminal(",") {
        lex_state.get_next()?;
        children.push( build_intent(rules_with_context, lex_state, mathml)? );   // arg before ','
        // debug!("    build_args, # children {};  state: {}", children.len(), lex_state);
    }

    // debug!("    end build_args, # children {};  state: {}", children.len(), lex_state);
    return Ok(children);
}

/// lift the children up to LITERAL_NAME
fn lift_function_name<'m>(doc: Document<'m>, function_name: Element<'m>, children: Vec<Element<'m>>) -> Element<'m> {
    // debug!("    lift_function_name: {}", name(function_name));
    // debug!("    lift_function_name: {}", mml_to_string(function_name));
    if name(function_name) == "mi" || name(function_name) == "mn" {   // FIX -- really want to test for all leaves, but not "data-from-mathml"
        // simple/normal case of f(x,y)
        // don't want to say that this is a leaf -- doing so messes up because it potentially has children
        set_mathml_name(function_name, as_text(function_name));
        function_name.set_text("");
        function_name.replace_children(children);
        if name(function_name).find(|ch| ch!='_' && ch!='-').is_none() {
            let properties = function_name.attribute_value(INTENT_PROPERTY).unwrap_or(":").to_owned();
            function_name.set_attribute_value(INTENT_PROPERTY, &(properties + "silent:"));
        }
        return function_name;
    } else if function_name.children().is_empty() {
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
fn find_arg<'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, name: &str, mathml: Element<'c>, skip_self: bool, no_check_inside: bool) -> Result<Option<Element<'m>>> {
    // debug!("Looking for '{}' in\n{}", name, mml_to_string(mathml));
    if !skip_self {
        if let Some(arg_val) = mathml.attribute_value("arg") {
            // debug!("looking for '{}', found arg='{}'", name, arg_val);
            if name == arg_val {
                // check to see if this mathml has an intent value -- if so the value is the value of its intent value
                if let Some(intent_str) = mathml.attribute_value(INTENT_ATTR) {
                    let mut lex_state = LexState::init(intent_str.trim())?;
                    return Ok( Some( build_intent(rules_with_context, &mut lex_state, mathml)? ) );
                } else {
                    return Ok( Some( rules_with_context.match_pattern::<Element<'m>>(mathml)? ) );
                }
            } else if no_check_inside {
                return Ok(None);       // don't look inside 'arg'
            }
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
        if let Some(element) = find_arg(rules_with_context, name, child, false, true)? {
            return Ok( Some(element) );
        }
    }

    return Ok(None);               // not present
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::init_logger;
    use sxd_document::parser;


    fn test_intent(mathml: &str, target: &str, intent_error_recovery: &str) -> bool {
		use crate::interface::*;
        use crate::pretty_print::mml_to_string;
		// this forces initialization
        crate::interface::set_rules_dir(super::super::abs_rules_dir_path()).unwrap();
        // crate::speech::SpeechRules::initialize_all_rules().unwrap();
        set_preference("IntentErrorRecovery".to_string(), intent_error_recovery.to_string()).unwrap();
        set_preference("SpeechStyle".to_string(), "SimpleSpeak".to_string()).unwrap();      // avoids possibility of "LiteralSpeak"
        let package1 = &parser::parse(mathml).expect("Failed to parse test input");
        let mathml = get_element(package1);
        trim_element(mathml, false);
        debug!("test:\n{}", mml_to_string(mathml));
        
        let package2 = &parser::parse(target).expect("Failed to parse target input");
        let target = get_element(package2);
        trim_element(target,true);
        debug!("target:\n{}", mml_to_string(target));

        let result = match crate::speech::intent_from_mathml(mathml, package2.as_document()) {
            Ok(e) => e,
            Err(e) => {
                debug!("{}", crate::interface::errors_to_string(&e));
                return false;       // could be intentional failure
            }
        };
        debug!("result:\n{}", mml_to_string(result));
        match is_same_element(result, target) {
			Ok(_) => return true,
			Err(e) => panic!("{}:\nresult: {}target: {}", e, mml_to_string(result), mml_to_string(target)),
		}
    }

    #[test]
    fn infer_binomial() {
        let mathml = "<mrow intent='binomial($n, $m)'>
                <mo>(</mo>
                <mfrac linethickness='0'> <mn arg='n'>7</mn> <mn arg='m'>3</mn> </mfrac>
                <mo>)</mo>
            </mrow>";
        let intent = "<binomial data-from-mathml='mrow' data-intent-property=':infix:'> <mn data-from-mathml='mn' arg='n'>7</mn> <mn data-from-mathml='mn' arg='m'>3</mn>  </binomial>";
        assert!(test_intent(mathml, intent, "Error"));
    }

    #[test]
    fn infer_binomial_intent_arg() {
        let mathml = "<msubsup intent='$op($n,$m)'>
                <mi arg='op' intent='binomial'>C</mi>
                <mi arg='n'>n</mi>
                <mi arg='m'>m</mi>
            </msubsup>";
        let intent = "<binomial data-from-mathml='msubsup' data-intent-property=':infix:'> <mi data-from-mathml='mi' arg='n'>n</mi> <mi data-from-mathml='mi' arg='m'>m</mi></binomial>";
        assert!(test_intent(mathml, intent, "Error"));
    }

    #[test]
    fn silent_underscore() {
        let mathml = "<mrow><mi intent='__-'>silent</mi><mo>+</mo><mi>e</mi></mrow>";
        let intent = "<mrow data-from-mathml='mrow'>
                                <mi data-from-mathml='mi'>__-</mi>
                                <mo data-from-mathml='mo'>+</mo>
                                <mi data-from-mathml='mi'>e</mi>
                            </mrow>";
        assert!(test_intent(mathml, intent, "Error"));
    }


    #[test]
    fn silent_underscore_function() {
        let mathml = "<mrow intent='__-_(speak, this)'></mrow>";
        let intent = "<__-_ data-from-mathml='mrow' data-intent-property=':silent:'>
                                <mi data-from-mathml='mi'>speak</mi>
                                <mi data-from-mathml='mi'>this</mi>
                            </__-_>";
        assert!(test_intent(mathml, intent, "Error"));
    }

    #[test]
    fn intent_multiple_properties() {
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
    }
    #[test]
    fn intent_nest_no_arg_call() {
        let mathml = "<mrow intent='foo(bar())'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<foo><bar></bar></foo>";
        assert!(!test_intent(mathml, intent, "Error"));
    }

    #[test]
    fn intent_hints() {
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
    }
    
    #[test]
    fn intent_hints_and_type() {
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
    }

    #[test]
    fn intent_in_intent_first_arg() {
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
    }

    #[test]
    fn intent_in_intent_second_arg() {
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
    }

    #[test]
    fn intent_with_whitespace() {
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
    }

    #[test]
    fn intent_template_at_toplevel() {
        let mathml = "<msup intent='$H $n'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
            </msup>";
        let intent = "<mrow><mi arg='H' mathvariant='normal'>H</mi><mn arg='n'>2</mn></mrow>";
        assert!(!test_intent(mathml, intent, "Error"));
    }

    #[test]
    fn intent_with_nested_indirect_head() {
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
    }

    #[test]
    fn intent_with_literals() {
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
    }

    #[test]
    fn intent_with_template_literals() {
        let mathml = "<mrow intent='1 0.0 0.1 -23 -0.1234 last'>
                <mi>x</mi>
            </mrow>";
        let intent = "<mrow><mn>1</mn><mn>0.</mn><mn>.1</mn><mn>-23</mn><mn>-.1234</mn><mi>last</mi></mrow>";
        assert!(!test_intent(mathml, intent, "Error"));
    }

    #[test]
    fn intent_with_nested_head() {
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
    }


    #[test]
    fn intent_with_nested_head_and_hints() {
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
    }


    #[test]
    fn intent_double_indirect_head() {
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
    }

    #[test]
    fn intent_missing_open() {
        let mathml = "<mrow intent='$p $a,$f($b))'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<plus> <mi arg='a'>a</mi> <factorial><mi arg='b'>b</mi></factorial> </plus>";
        assert!(!test_intent(mathml, intent, "Error"));
    }

    #[test]
    fn intent_no_comma() {
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
    }

    #[test]
    fn intent_no_arg() {
        let mathml = "<mrow intent='factorial()'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let target = "<factorial></factorial>";
        assert!(!test_intent(mathml, target, "Error"));
    }

    #[test]
    fn intent_illegal_no_arg() {
        let mathml = "<mrow intent='factorial(()))'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let target = "<factorial></factorial>";
        assert!(!test_intent(mathml, target, "Error"));
    }

    #[test]
    fn intent_illegal_no_arg_ignore() {
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
    }

    #[test]
    fn intent_illegal_self_ref() {
        let mathml = "<mrow intent='foo:is-foolish:function($b)'>
                <mi intent='$b:int' arg='b'>b</mi>
            </mrow>";
        let target = "<foo data-intent-property=':function:' data-intent-type='is-foolish'><mi data-intent-type='int'>b</mi></foo>";
        assert!(!test_intent(mathml, target, "Error"));
    }

    #[test]
    fn infer_missing_second_arg() {
        let mathml = "<mrow intent='binomial($n,)'>
                <mo>(</mo>
                <mfrac linethickness='0'> <mn arg='n'>7</mn> <mn arg='m'>3</mn> </mfrac>
                <mo>)</mo>
            </mrow>";
        let target = "<binomial data-intent-property='binomial($n,)'> \n
                             <mn data-from-mathml='mn' arg='n'>7</mn> <mn data-from-mathml='mn' arg='m'>3</mn>  </binomial>";
        assert!(!test_intent(mathml, target, "Error"));
    }

    #[test]
    fn infer_missing_second_arg_ignore() {
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
    }   

    #[test]
    fn plane1_char_in_concept_name() {
        let mathml = "<math><mrow><mo intent='🐇'>&#x1F407;</mo><mi>X</mi></mrow></math>";
        let intent = "<math data-from-mathml='math'>
                                <mrow data-from-mathml='mrow'>
                                    <mi data-from-mathml='mo'>🐇</mi>
                                    <mi data-from-mathml='mi'>X</mi>
                                </mrow>
                            </math>";
        assert!(test_intent(mathml, intent, "Error"));
    }   
}
