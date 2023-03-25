//! Use heuristics to infer the intent.
//! For example, an `mfrac` with `linethickness=0` would be a binomial
//! The inference is added to the MathML
//!
//! The implementation of the module is on hold until the MathML committee figures out how it wants to do this.
#![allow(clippy::needless_return)]

use sxd_document::dom::*;
use crate::speech::SpeechRulesWithContext;
use crate::canonicalize::{as_element, as_text, name, create_mathml_element};
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
            let intent_preference = rules_with_context.get_rules().pref_manager.borrow().get_api_prefs().to_string("IntentErrorRecovery");
            if intent_preference == "Error" {
                return Err(e);
            } else {
                const INTENT_ATTR: &str = "intent";
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
        if let Some(intent_str) = mathml.attribute_value("intent") {
            let mut lex_state = LexState::init(intent_str.trim())?;
            let result = build_pieces(rules_with_context, &mut lex_state, mathml)
                        .chain_err(|| format!("occurs before '{}' in intent attribute value '{}'", lex_state.remaining_str, intent_str))?;
            if lex_state.token != Token::None {
                bail!("Error in intent value: extra unparsed intent '{}' in intent attribute value '{}'", lex_state.remaining_str, intent_str);
            }
            assert!(lex_state.remaining_str.is_empty());
            debug!("Resulting intent: {}", crate::pretty_print::mml_to_string(&result));
            return Ok(result);
        }
        bail!("Internal error: infer_intent() called on MathML with no intent arg:\n{}", mml_to_string(&mathml));
    }
}

// From https://github.com/w3c/mathml/issues/446
// pieces        = space piece (spaces piece)* space
// spaces       = \s+
// space        = \s*
// piece          = space (property+ | word space property* | funcall) space
// word          = literal | reference
// literal         = [^\s:\$,;()]+
// property   = ':' NCName
// reference  = '$' NCName
// NCName = [\pL][\pL\pMn\-.\d·]*
// funcall = piece '(' pieces (',' pieces)* ')'
lazy_static! {
    // The practical restrictions of NCName are that it cannot contain several symbol characters like
    //  !, ", #, $, %, &, ', (, ), *, +, ,, /, :, ;, <, =, >, ?, @, [, \, ], ^, `, {, |, }, ~, and whitespace characters
    //  Furthermore an NCName cannot begin with a number, dot or minus character although they can appear later in an NCName.
    // NC_NAME from www.w3.org/TR/REC-xml/#sec-common-syn, with "\pL" for letters 

    static ref LITERAL: Regex = Regex::new(r"^[^\s:\$,;()]+").unwrap(); 
    static ref PROPERTY: Regex = Regex::new(r"^:[\pL][\pL\pMn\-.\d·]*").unwrap();    // : NC_NAME
    static ref ARG_REF: Regex = Regex::new(r"^\$[\pL][\pL\pMn\-.\d·]*").unwrap();  // $ NC_NAME
    static ref NUMBER: Regex = Regex::new(r"^-?([0-9]+.?[0-9]*|.[0-9]+)").unwrap();     // Token::Literal -- used for matching later on
    static ref NC_NAME: Regex = Regex::new(r"^[\pL][\pL\pMn\-.\d·]*").unwrap();     // Token::Literal -- used for matching later on
}

static TERMINALS_AS_U8: [u8; 4] = [b'(', b',', b')', b';'];
// static TERMINALS: [char; 4] = ['(', ',',')', ';'];

// 'i -- "i" for the lifetime of the "intent" string
#[derive(Debug, PartialEq, Eq, Clone)]
enum Token<'i> {
    Terminal(&'i str),  // "(", ",", ")"
    Property(&'i str),
    ArgRef(&'i str),
    Literal(&'i str),
    None,               // out of characters
}

impl<'i> fmt::Display for Token<'i> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}",
            match self {
                Token::Terminal(str) => format!("Terminal('{}')", str),
                Token::Property(str) => format!("Property({})", str),
                Token::ArgRef(str) => format!("ArgRef({})", str),
                Token::Literal(str) => format!("Literal({})", str),
                Token::None => "None".to_string(),
            }
        );
    }
}

impl<'i> Token<'i> {
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
            Token::Literal(str) => str,
            Token::None => "",
        }
    }
}

struct LexState<'i> {
    token: Token<'i>,
    remaining_str: &'i str,     // always trimmed
}

impl<'i> fmt::Display for LexState<'i> {
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
        } else if  let Some(matched_literal) = LITERAL.find(str) {
            self.token = Token::Literal(matched_literal.as_str());
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
            self.remaining_str = &self.remaining_str[1..].trim_start();
        } else {
            self.set_token(&self.remaining_str)?;
            self.remaining_str = &self.remaining_str[self.token.as_str().len()..].trim_start(); 
}    
        return Ok(&self.token);
    }

    fn is_terminal(&self, terminal: &str) -> bool {
        return self.token.is_terminal(terminal);
    }
}

fn build_piece<'b, 'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
                                         lex_state: &mut LexState<'b>,
                                         mathml: Element<'c>) -> Result<Element<'m>> {
    debug!("    start build_piece: state: {}", lex_state);
    let doc = rules_with_context.get_document();
    let mut piece;
    match lex_state.token {
        Token::Property(_) => {
            // We only have a property -- we want to keep this tag
            // There are two paths:
            // 1. If there is a function call, then the children are dealt with there
            // 2. If there is *no* function call, then the children are kept, which means we return to pattern matching
            //    Note: to avoid infinite loop, we need to remove the 'intent' so we don't end up back here; we put it back later
            let properties = get_properties(lex_state)?;    // advance state to see if funcall
            if lex_state.is_terminal("(") {
                piece = create_mathml_element(&doc, name(&mathml))
            } else {
                let saved_intent = mathml.attribute_value("intent").unwrap();
                mathml.remove_attribute("intent");
                piece = rules_with_context.match_pattern::<Element<'m>>(mathml)?;
                mathml.set_attribute_value("intent", saved_intent);
            }
            piece.set_attribute_value(INTENT_PROPERTY, &properties);
        },
        Token::Literal(word) => {
            let leaf_name = if NUMBER.is_match(word) {
                "mn"
            } else if NC_NAME.is_match(word) {
                "mi"
            } else {
                "mo"
            };
            piece = create_mathml_element(&doc, leaf_name);
            piece.set_text(word);       // '-' and '_' get removed by the rules.
            lex_state.get_next()?;
            if let Token::Property(_) = lex_state.token {
                let properties = get_properties(lex_state)?;
                piece.set_attribute_value(INTENT_PROPERTY, &properties);
            }
        },
        Token::ArgRef(word) => {
            piece = match find_arg(rules_with_context, &word[1..], mathml, true, false)? {
                Some(e) => {
                    lex_state.get_next()?;
                    e
                },
                None => bail!("intent arg '{}' not found", word),
            };
            if let Token::Property(_) = lex_state.token {
                let properties = get_properties(lex_state)?;
                piece.set_attribute_value(INTENT_PROPERTY, &properties);
            }
        },
        _ => bail!("Illegal 'intent' syntax: found {}", lex_state.token),
    };
    if lex_state.is_terminal("(") {
        piece = build_function(piece, rules_with_context, lex_state, mathml)?;
    }
    debug!("    end build_piece: state: {}     piece: {}", lex_state, mml_to_string(&piece));
    return Ok(piece);
}

const INTENT_PROPERTY: &str = "data-intent-property";
/// Build an intent
/// Start state: lex_state on token to build
/// End state: after built intent (a terminal or None)
/// Fix (maybe): this allows numbers as the head of a function which is not part of the spec even though an 'argref' can have the same effect
fn build_pieces<'b, 'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
                                        lex_state: &mut LexState<'b>,
                                        mathml: Element<'c>) -> Result<Element<'m>> {
    // pieces := piece (spaces piece)*
    debug!("  start build_pieces: state: {}", lex_state);
    let mut pieces = Vec::with_capacity(15);
    while !(lex_state.is_terminal("(") || lex_state.is_terminal(",") || lex_state.is_terminal(")")) {
        pieces.push(build_piece(rules_with_context, lex_state, mathml)?);
        if lex_state.remaining_str.is_empty() {     // can't be part of while test because we might start with nothing remaining
            debug!("  build_pieces empty string -- lex_state: {}", lex_state);
            match lex_state.token {
                Token::Terminal(_) | Token::None => (),
                _ => pieces.push(build_piece(rules_with_context, lex_state, mathml)?),
            };
            break;
        }
    }

    if pieces.is_empty() {
        bail!("Illegal 'intent' syntax: no content'");
    } else if pieces.len() == 1 {
        debug!("  end build_pieces: {}", mml_to_string(&pieces[0]));
        return Ok(pieces[0]);
    } else {
        // use an mrow to enclose all the pieces
        let template = create_mathml_element(&rules_with_context.get_document(), "mrow");
        template.append_children(pieces);
        debug!("  end build_pieces: {}", mml_to_string(&template));
        return Ok( template );
    }    
}

fn get_properties<'b>(lex_state: &mut LexState<'b>) -> Result<String> {
    // return the 'hint' leaving the state
    assert!(matches!(lex_state.token, Token::Property(str) if str.starts_with(":")));
    let mut properties = String::with_capacity(60);
    properties.push_str(lex_state.token.as_str());
    loop {
        let token = lex_state.get_next()?;
        if let Token::Property(property) = token {
            properties.push_str(property);
        } else {
            properties.push(':');
            debug!("      get_properties: returns {}", properties);
            return Ok(properties);
        }
    }
}

/// build a function 'f(...)' where '...' can be empty
/// also handles nested functions like f(...)(...)
/// Start state: at '('
/// End state: after ')'
fn build_function<'b, 'r, 'c, 's:'c, 'm:'c>(
            function_name: Element<'m>,
            rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
            lex_state: &mut LexState<'b>,
            mathml: Element<'c>) -> Result<Element<'m>> {
    debug!("  start build_function: name: {}, state: {}", name(&function_name), lex_state);
    // funcall = piece '(' piece (',' piece)* ')'
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
    debug!("  end build_function/# children: {}, #state: {}  ..[bfa] function name: {}",
        function.children().len(), lex_state, mml_to_string(&function));
    return Ok(function);
}

// process all the args of a function
// Start state: after '('
// End state: on ')'
fn build_arguments<'b, 'r, 'c, 's:'c, 'm:'c>(
            rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
            lex_state: &mut LexState<'b>,
            mathml: Element<'c>) -> Result<Vec<Element<'m>>> {
    // working on the '(' pieces (',' pieces)* ')' part
    debug!("    start build_args state: {}", lex_state);

    // there is at least one arg
    let mut children = Vec::with_capacity(lex_state.remaining_str.len()/3 + 1);   // conservative estimate ('3' - "$x,");
    children.push( build_pieces(rules_with_context, lex_state, mathml)? );   // arg before ','
    // debug!("  build_args: # children {};  state: {}", children.len(), lex_state);

    while lex_state.is_terminal(",") {
        lex_state.get_next()?;
        children.push( build_pieces(rules_with_context, lex_state, mathml)? );   // arg before ','
        // debug!("    build_args, # children {};  state: {}", children.len(), lex_state);
    }

    // debug!("    end build_args, # children {};  state: {}", children.len(), lex_state);
    return Ok(children);
}

/// lift the children up to LITERAL_NAME
fn lift_function_name<'m>(doc: Document<'m>, function_name: Element<'m>, children: Vec<Element<'m>>) -> Element<'m> {
    use crate::canonicalize::set_mathml_name;
    debug!("    lift_function_name: {}", name(&function_name));
    debug!("    lift_function_name: {}", mml_to_string(&function_name));
    if is_leaf(function_name) {
        // simple/normal case of f(x,y)
        set_mathml_name(function_name, as_text(function_name));
        function_name.set_text("");
        function_name.replace_children(children);
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
                if let Some(intent_str) = mathml.attribute_value("intent") {
                    let mut lex_state = LexState::init(intent_str.trim())?;
                    return Ok( Some( build_pieces(rules_with_context, &mut lex_state, mathml)? ) ); 
                } else {
                    return Ok( Some( rules_with_context.match_pattern::<Element<'m>>(mathml)? ) );
                }
            } else if no_check_inside {
                return Ok(None);       // don't look inside 'arg'
            }
        }
    }

    if no_check_inside && mathml.attribute_value("intent").is_some() {
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
    fn intent_hints_defaults() {
        let mathml = "<mrow intent=':silent($p:infix($a, $b, $f:postfix($x)))'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo intent='plus'>+</mo>
                <mi arg='x'>x</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<mrow data-intent-property=':silent:'>
                    <plus data-intent-property=':infix:'>
                        <mi arg='a'>a</mi>
                        <mi arg='b'>b</mi>
                        <factorial data-intent-property=':postfix:'><mi arg='x'>x</mi></factorial>
                    </plus>
                </mrow>";
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
        assert!(test_intent(mathml, intent, "Error"));
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
        let mathml = "<mrow intent='vector(1, 0., .1, -23, -.1234, last)'>
                <mi>x</mi>
            </mrow>";
        let intent = "<vector><mn>1</mn><mn>0.</mn><mn>.1</mn><mn>-23</mn><mn>-.1234</mn><mi>last</mi></vector>";
        assert!(test_intent(mathml, intent, "Error"));
    }

    #[test]
    fn intent_with_string_literals() {
        let mathml = "<mrow intent='1 0. .1 -23 -.1234 last'>
                <mi>x</mi>
            </mrow>";
        let intent = "<mrow><mn>1</mn><mn>0.</mn><mn>.1</mn><mn>-23</mn><mn>-.1234</mn><mi>last</mi></mrow>";
        assert!(test_intent(mathml, intent, "Error"));
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
        assert!(test_intent(mathml, intent, "Error"));
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
}