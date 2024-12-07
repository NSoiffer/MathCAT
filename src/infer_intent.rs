//! Use heuristics to infer the intent.
//! For example, an `mfrac` with `linethickness=0` would be a binomial
//! The inference is added to the MathML
//!
//! The implementation of the module is on hold until the MathML committee figures out how it wants to do this.
#![allow(clippy::needless_return)]

use sxd_document::dom::*;
use crate::speech::SpeechRulesWithContext;
use crate::canonicalize::{as_element, as_text, name, create_mathml_element, set_mathml_name, INTENT_ATTR};
use crate::errors::*;
use std::fmt;
use crate::pretty_print::mml_to_string;
use crate::xpath_functions::is_leaf;
use regex::Regex;

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
            // debug!("Before intent: {}", crate::pretty_print::mml_to_string(&mathml));
            let mut lex_state = LexState::init(intent_str.trim())?;
            let result = build_intent(rules_with_context, &mut lex_state, mathml)
                        .chain_err(|| format!("occurs before '{}' in intent attribute value '{}'", lex_state.remaining_str, intent_str))?;
            if lex_state.token != Token::None {
                bail!("Error in intent value: extra unparsed intent '{}' in intent attribute value '{}'", lex_state.remaining_str, intent_str);
            }
            assert!(lex_state.remaining_str.is_empty());
            // debug!("Resulting intent: {}", crate::pretty_print::mml_to_string(&result));
            return Ok(result);
        }
        bail!("Internal error: infer_intent() called on MathML with no intent arg:\n{}", mml_to_string(&mathml));
    }
}

// intent             := S ( term property* | property+ | application ) S 
// term               := concept-or-literal | number | reference 
// concept-or-literal := NCName
// number             := '-'? \d+ ( '.' \d+ )?
// reference          := '$' NCName
// application        := intent '(' arguments? S ')'
// arguments          := intent ( ',' intent )*
// property           := S ':' NCName
// S                  := [ \t\n\r]*

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
                Token::Terminal(str) => format!("Terminal('{}')", str),
                Token::Property(str) => format!("Property({})", str),
                Token::ArgRef(str) => format!("ArgRef({})", str),
                Token::ConceptOrLiteral(str) => format!("Literal({})", str),
                Token::Number(str) => format!("Number({})", str),
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

    fn get_next(&mut self) -> Result<&Token> {
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
    // debug!("    start build_intent: state: {}", lex_state);
    let doc = rules_with_context.get_document();
    let mut intent;
    match lex_state.token {
        Token::Property(_) => {
            // We only have a property -- we want to keep this tag/element
            // There are two paths:
            // 1. If there is a function call, then the children are dealt with there
            // 2. If there is *no* function call, then the children are kept, which means we return to pattern matching
            //    Note: to avoid infinite loop, we need to remove the 'intent' so we don't end up back here; we put it back later
            let properties = get_properties(lex_state)?;    // advance state to see if funcall
            if lex_state.is_terminal("(") {
                intent = create_mathml_element(&doc, name(&mathml));
                intent.set_attribute_value(INTENT_PROPERTY, &properties);
            } else {
                let saved_intent = mathml.attribute_value(INTENT_ATTR).unwrap();
                mathml.remove_attribute(INTENT_ATTR);
                mathml.set_attribute_value(INTENT_PROPERTY, &properties);   // needs to be set before the pattern match
                intent = rules_with_context.match_pattern::<Element<'m>>(mathml)?;
                mathml.set_attribute_value(INTENT_ATTR, saved_intent);
            }
            return Ok(intent);      // if we start with properties, then there can only be properties
        },
        Token::ConceptOrLiteral(word) | Token::Number(word) => {
            let leaf_name = if let Token::Number(_) = lex_state.token {"mn"} else {"mi"};
            intent = create_mathml_element(&doc, leaf_name);
            intent.set_text(word);       // '-' and '_' get removed by the rules.
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
    // debug!("    end build_intent: state: {}     piece: {}", lex_state, mml_to_string(&intent));
    return Ok(intent);
}

const INTENT_PROPERTY: &str = "data-intent-property";
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
            return Ok(properties);
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
    // debug!("  start build_function: name: {}, state: {}", name(&function_name), lex_state);
    // application := intent '(' arguments? S ')'  where 'function_name' is 'intent'
    assert!(lex_state.is_terminal("("));
    let mut function = function_name;
    while lex_state.is_terminal("(") {
        lex_state.get_next()?;
        if lex_state.is_terminal(")") {
            // grammar requires at least one argument
            bail!("Illegal 'intent' syntax: missing argument for intent name '{}'", name(&function_name));
        }
        let children = build_arguments(rules_with_context, lex_state, mathml)?;
        function = lift_function_name(rules_with_context.get_document(), function, children);

        if !lex_state.is_terminal(")") {
            bail!("Illegal 'intent' syntax: missing ')' for intent name '{}'", name(&function_name));
        }
        lex_state.get_next()?;
    }
    // debug!("  end build_function/# children: {}, #state: {}  ..[bfa] function name: {}",
    //     function.children().len(), lex_state, mml_to_string(&function));
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
    // debug!("    lift_function_name: {}", name(&function_name));
    // debug!("    lift_function_name: {}", mml_to_string(&function_name));
    if is_leaf(function_name) {
        // simple/normal case of f(x,y)
        set_mathml_name(function_name, as_text(function_name));
        function_name.set_text("");
        function_name.replace_children(children);
        if name(&function_name).find(|ch| ch!='_' && ch!='-').is_none() {
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
        result.append_child(function_name);
        result.append_children(children);
        return result;
    }
}


/// look for @arg=name in mathml
/// if 'check_intent', then look at an @intent for this element (typically false for non-recursive calls)
fn find_arg<'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, name: &str, mathml: Element<'c>, skip_self: bool, no_check_inside: bool) -> Result<Option<Element<'m>>> {
    // debug!("Looking for '{}' in\n{}", name, mml_to_string(&mathml));
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

    if is_leaf(mathml) {
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
		// this forces initialization
        crate::interface::set_rules_dir(super::super::abs_rules_dir_path()).unwrap();
        // crate::speech::SpeechRules::initialize_all_rules().unwrap();
        set_preference("IntentErrorRecovery".to_string(), intent_error_recovery.to_string()).unwrap();
        let package1 = &parser::parse(mathml).expect("Failed to parse test input");
        let mathml = get_element(package1);
        trim_element(&mathml);
        debug!("test: {}", crate::pretty_print::mml_to_string(&mathml));
        
        let package2 = &parser::parse(target).expect("Failed to parse target input");
        let target = get_element(package2);
        trim_element(&target);
        debug!("target: {}", crate::pretty_print::mml_to_string(&target));

        let result = match crate::speech::intent_from_mathml(mathml, package2.as_document()) {
            Ok(e) => e,
            Err(e) => {
                debug!("{}", crate::interface::errors_to_string(&e));
                return false;       // could be intentional failure
            }
        };
        debug!("result: {}", crate::pretty_print::mml_to_string(&result));
        match is_same_element(&result, &target) {
			Ok(_) => return true,
			Err(e) => panic!("{}", e),
		}
    }

    #[test]
    fn infer_binomial() {
        let mathml = "<mrow intent='binomial($n, $m)'>
                <mo>(</mo>
                <mfrac linethickness='0'> <mn arg='n'>7</mn> <mn arg='m'>3</mn> </mfrac>
                <mo>)</mo>
            </mrow>";
        let intent = "<binomial> <mn arg='n'>7</mn> <mn arg='m'>3</mn>  </binomial>";
        assert!(test_intent(mathml, intent, "Error"));
    }

    #[test]
    fn infer_binomial_intent_arg() {
        let mathml = "<msubsup intent='$op($n,$m)'>
                <mi arg='op' intent='binomial'>C</mi>
                <mi arg='n'>n</mi>
                <mi arg='m'>m</mi>
            </msubsup>";
        let intent = "<binomial> <mi arg='n'>n</mi> <mi arg='m'>m</mi></binomial>";
        assert!(test_intent(mathml, intent, "Error"));
    }

    #[test]
    fn silent_underscore() {
        let mathml = "<mrow><mi intent='__-'>silent</mi><mo>+</mo><mi>e</mi></mrow>";
        let intent = "<mrow>=<mi>__-</mi><mo>+</mo><mi>e</mi></mrow>";
        assert!(test_intent(mathml, intent, "Error"));
    }


    #[test]
    fn silent_underscore_function() {
        let mathml = "<mrow intent='__-_(speak, this)'></mrow>";
        let intent = "<__-_ data-intent-property=':silent:'><mi>speak</mi><mi>this</mi></__-_>";
        assert!(test_intent(mathml, intent, "Error"));
    }

    #[test]
    fn intent_multiple_properties() {
        let mathml = "<mrow intent='foo:silent:int(bar:positive-int:int, $a:foo:bar:foo-bar, $b:number)'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b' intent=':negative-int:int'>b</mi>
            </mrow>";
        let intent = "<foo data-intent-property=':silent:int:'>
                                <mi data-intent-property=':positive-int:int:'>bar</mi>
                                <mi arg='a' data-intent-property=':foo:bar:foo-bar:'>a</mi>
                                <mi arg='b' data-intent-property=':number:'>b</mi>
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
        let intent = "<foo data-intent-property=':silent:'><bar data-intent-property=':postfix:'><mn>3</mn></bar></foo>";
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
        let intent = "<foo data-intent-property=':is-foolish:function:'><mi data-intent-property=':int:'>b</mi></foo>";
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
        let intent = "<p> <f><mi>b</mi></f> <mi>a</mi></p>";
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
        let intent = "<plus> <mi>a</mi> <factorial><mi>b</mi></factorial> </plus>";
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
        let intent = "<map> <mi arg='a'>A</mi> <mi arg='b'>B</mi> <mi arg='c'>C</mi> </map>";
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
        let intent = "<apply-function><map> <mi>congruence</mi></map> <mi arg='a'>A</mi> <mi arg='b'>B</mi> </apply-function>";
        assert!(test_intent(mathml, intent, "Error"));
    }

    #[test]
    fn intent_with_literals() {
        let mathml = "<mrow intent='vector(1, 0.0, 0.1, -23, -0.1234, last)'>
                <mi>x</mi>
            </mrow>";
        let intent = "<vector><mn>1</mn><mn>0.0</mn><mn>0.1</mn><mn>-23</mn><mn>-0.1234</mn><mi>last</mi></vector>";
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
        let intent = "<apply-function>
                    <map><mi>congruence</mi></map>
                    <mi arg='a'>A</mi> <mi arg='b'>B</mi>
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
        let intent = "<apply-function>
                <pre data-intent-property=':prefix:'>
                    <in data-intent-property=':infix:'>
                        <mi arg='a'>A</mi>
                        <mi>x</mi>
                    </in>
                </pre>
                <post data-intent-property=':postfix:'>
                    <mi arg='b'>B</mi>
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
        let intent = "<apply-function>
            <map data-intent-property=':prefix:'>  <mi>congruence</mi> </map>
            <mi arg='a'>A</mi>
            <mi arg='b'>B</mi>
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
        let target = "<mrow intent='factorial()'>
                <mi arg='a'>a</mi>
                <mi>plus</mi>
                <mi arg='b'>b</mi>
                <mi>factorial</mi>
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
        let target = "<binomial data-intent-property='binomial($n,)'> <mn arg='n'>7</mn> <mn arg='m'>3</mn>  </binomial>";
        assert!(!test_intent(mathml, target, "Error"));
    }

    #[test]
    fn infer_missing_second_arg_ignore() {
        let mathml = "<mrow intent='binomial($n,)'>
                <mo>(</mo>
                <mfrac linethickness='0'> <mn arg='n'>7</mn> <mn arg='m'>3</mn> </mfrac>
                <mo>)</mo>
            </mrow>";
        let target = "<mrow intent='binomial($n,)'>
                <mo>(</mo>
                <fraction linethickness='0'> <mn arg='n'>7</mn> <mn arg='m'>3</mn> </fraction>
                <mo>)</mo>
            </mrow>";
        assert!(test_intent(mathml, target, "IgnoreIntent"));
    }   

    #[test]
    fn plane1_char_in_concept_name() {
        let mathml = "<math><mrow><mo intent='🐇'>&#x1F407;</mo><mi>X</mi></mrow></math>";
        let intent = "<math><mrow><mi>🐇</mi><mi>X</mi></mrow></math>";
        assert!(test_intent(mathml, intent, "Error"));
    }   
}
