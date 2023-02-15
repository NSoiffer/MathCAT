//! Use heuristics to infer the intent.
//! For example, an `mfrac` with `linethickness=0` would be a binomial
//! The inference is added to the MathML
//!
//! The implementation of the module is on hold until the MathML committee figures out how it wants to do this.
#![allow(clippy::needless_return)]

use sxd_document::dom::*;
use crate::speech::SpeechRulesWithContext;
use crate::canonicalize::{as_element, as_text, name, create_mathml_element,set_mathml_name};
use crate::{errors::*};
use std::fmt;
use crate::pretty_print::mml_to_string;
use crate::xpath_functions::is_leaf;
use regex::Regex;

pub const LITERAL_NAME: &str = "intent-literal";
const IMPLICIT_FUNCTION_NAME: &str = "apply-function";

impl<'c, 's:'c, 'm:'c> SpeechRulesWithContext<'c, 's,'m> {
}

pub fn infer_intent<'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, mathml: Element<'c>) -> Result<Element<'m>> {
    if let Some(intent_str) = mathml.attribute_value("intent") {
        let mut lex_state = LexState::init(intent_str.trim())?;
        let result = build_intent(rules_with_context, &mut lex_state, mathml) 
                    .chain_err(|| format!("in intent attribute value '{}'", intent_str))?;
        if lex_state.token != Token::None {
            bail!("Error in intent value: extra unparsed intent '{}' in intent attribute value '{}'", lex_state.remaining_str, intent_str);
        }
        assert!(lex_state.remaining_str.is_empty());
        debug!("Resulting intent: {}\n", crate::pretty_print::mml_to_string(&result));
        return Ok(result);
    }
    bail!("Internal error: infer_intent() called on MathML with no intent arg:\n{}", mml_to_string(&mathml));
}

// intent          := name-or-literal | number | reference | application 
// name-or-literal := NCName
// number          := '-'? digit+ ( '.' digit+ )?
// reference       := '$' NCName
// application     := intent hint? '(' arguments? ')'
// arguments       := intent ( ',' intent )*
// hint            :=  '@' ('prefix' | 'infix' | 'postfix'  | 'silent' | NCName)

// With isa/types
// intent          := typed-name-or-literal | number | reference | application 
// typed-name-or-literal := NCName type?
// type            := ':' NCName
// number          := '-'? digit+ ( '.' digit+ )?
// reference       := '$' NCName
// application     := intent hint? '(' arguments? ')'
// arguments       := intent ( ',' intent )*
// hint            := '@' ( 'prefix' | 'infix' | 'postfix' | 'function' | 'silent' )
lazy_static! {
    // The practical restrictions of NCName are that it cannot contain several symbol characters like
    //  !, ", #, $, %, &, ', (, ), *, +, ,, /, :, ;, <, =, >, ?, @, [, \, ], ^, `, {, |, }, ~, and whitespace characters
    //  Furthermore an NCName cannot begin with a number, dot or minus character although they can appear later in an NCName.
    static ref NUMBER: Regex = Regex::new(r"^-?([0-9]+.?[0-9]*|.[0-9]+)$").unwrap();
    static ref NC_NAME: Regex = Regex::new(r"^[:\pL_][:\pL\-.0-9·]*$").unwrap();  // from www.w3.org/TR/REC-xml/#sec-common-syn, with "\pL" for letters
    static ref ARG_REF: Regex = Regex::new(r"^\$[:\pL_][:\pL\-.0-9·]*$").unwrap();  // $ NC_NAME
}

static TERMINALS_AS_U8: [u8; 5] = [b'(', b',', b')', b'@', b':'];
static TERMINALS: [char; 5] = ['(', ',',')', '@', ':'];

// 'i -- "i" for the lifetime of the "intent" string
#[derive(Debug, PartialEq, Eq, Clone)]
enum Token<'i> {
    Terminal(&'i str),  // "(", ",", ")"
    NCName(&'i str),
    Number(&'i str),
    ArgRef(&'i str),
    None,               // out of characters
}

impl<'i> fmt::Display for Token<'i> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}",
            match self {
                Token::Terminal(str) => format!("Terminal({})", str),
                Token::NCName(str) => format!("Name({})", str),
                Token::Number(str) => format!("Number({})", str),
                Token::ArgRef(str) => format!("ArgRef({})", str),
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
        } else if NC_NAME.is_match(str) {
            self.token = Token::NCName(str);
        } else if NUMBER.is_match(str) {
            self.token = Token::Number(str);
        } else if ARG_REF.is_match(str) {
            self.token = Token::ArgRef(str);
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
            self.remaining_str = self.remaining_str[1..].trim_start();  // end is already trimmed
        } else {
            match self.remaining_str.find(TERMINALS) {
                None => {   // what remains should be a token (note: the string was trimmed)
                    self.set_token(self.remaining_str)?;
                    self.remaining_str = "";  // nothing left
                }
                Some(i) => {
                    self.set_token(self.remaining_str[..i].trim_end())?;
                    self.remaining_str = self.remaining_str[i..].trim_start();  // end is already trimmed
                }
            }
        }    
        return Ok(&self.token);
    }

    fn is_terminal(&self, terminal: &str) -> bool {
        return self.token.is_terminal(terminal);
    }
}

const INTENT_TYPE: &str = "data-intent-type";
const INTENT_HINT: &str = "data-intent-hint";
/// Build an intent
/// Start state: lex_state on token to build
/// End state: after built intent (a terminal or None)
/// Fix (maybe): this allows numbers as the head of a function which is not part of the spec even though an 'argref' can have the same effect
fn build_intent<'b, 'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
                                          lex_state: &mut LexState<'b>,
                                          mathml: Element<'c>) -> Result<Element<'m>> {
    // intent := name | number | reference | application
    // debug!("start build_intent:  state: {}", lex_state);
    let mut intent = get_element_from_token(rules_with_context, lex_state, mathml)?;
    let mut next_token = lex_state.get_next()?;
    let intent_type = if next_token.is_terminal(":") {
        let temp = Some(get_hint_or_type(lex_state, ":")?);
        next_token = lex_state.get_next()?;
        temp
    } else {
        None
    };
    let hint = if next_token.is_terminal("@") {
        let temp = Some(get_hint_or_type(lex_state, "@")?);
        lex_state.get_next()?;
        temp
    } else {
        None
    };
    if lex_state.is_terminal("(") {
        intent = build_function(intent, rules_with_context, lex_state, mathml)?;
        if let Some(type_str) = intent_type {
            intent.set_attribute_value(INTENT_TYPE, &type_str);
        }
        let mut hint_str = "function".to_string();
        // debug!("intent='{}'", mml_to_string(&intent));
        if let Some(found_hint_str) = hint {
            hint_str = found_hint_str;
        } else if name(&intent) == "_" {
            hint_str = "silent".to_string();
        }
        intent.set_attribute_value(INTENT_HINT, &hint_str);

    }
    // debug!("end build_intent:  state: {}..[bi] intent: {}", lex_state, mml_to_string(&intent));
    return Ok( intent );
}

fn get_hint_or_type<'b>(lex_state: &mut LexState<'b>, target: &str) -> Result<String> {
    // return the 'hint' leaving the state
    assert!(lex_state.is_terminal(target));
    let token = lex_state.get_next()?;
    if let Token::NCName(str) = token {
        return Ok( str.to_string() );       // note: there are lifetime/borrow issues if we just return 'str'
    } else {
        bail!("Illegal 'intent' syntax after '{}': {}", target, token);
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
    // application := function '(' arguments? ')'
    // function    := name | reference | application
    // debug!("  start build_function:  name: {}, state: {}", name(&function_name), lex_state);
    assert!(lex_state.is_terminal("("));
    let mut function = function_name;
    while lex_state.is_terminal("(") {
        lex_state.get_next()?;
        let children = if lex_state.is_terminal(")") { // no arguments
            vec![]
        } else {
            build_arguments(rules_with_context, lex_state, mathml)?
        };
        function = lift_function_name(rules_with_context.get_document(), function, children);

        if !lex_state.is_terminal(")") {
            bail!("Illegal 'intent' syntax: missing ')' for intent name '{}'", if name(&function_name)==LITERAL_NAME {as_text(function_name)} else {name(&function_name)});
        }
        lex_state.get_next()?;
    }
    // debug!("  end build_function/# children: {}, #state: {}  ..[bfa] function name: {}",
    //    function.children().len(), lex_state, mml_to_string(&function));
    return Ok(function);
}

// process all the args of a function
// Start state: after '('
// End state: on ')'
fn build_arguments<'b, 'r, 'c, 's:'c, 'm:'c>(
            rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
            lex_state: &mut LexState<'b>,
            mathml: Element<'c>) -> Result<Vec<Element<'m>>> {
    // arguments   := intent ( ',' intent )*
    // debug!("    start build_function_args state: {}", lex_state);

    // there is at least one arg
    let mut children = Vec::with_capacity(lex_state.remaining_str.len()/3 + 1);   // conservative estimate ('3' - "$x,");
    children.push( build_intent(rules_with_context, lex_state, mathml)? );   // arg before ','
    // debug!("  build_function_args: # children {};  state: {}", children.len(), lex_state);

    while lex_state.is_terminal(",") {
        lex_state.get_next()?;
        children.push( build_intent(rules_with_context, lex_state, mathml)? );   // arg before ','
        // debug!("    build_function_args, # children {};  state: {}", children.len(), lex_state);
    }

    // debug!("    end build_function_args, # children {};  state: {}", children.len(), lex_state);
    return Ok(children);
}


fn get_element_from_token<'b, 'r, 'c, 's:'c, 'm:'c>(
            rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
            lex_state: &mut LexState<'b>,
            mathml: Element<'c>) -> Result<Element<'m>> {
    return match lex_state.token {
        Token::None => bail!("Illegal 'intent' value: empty string"),
        Token::Terminal(str) => bail!("Illegal intent syntax: expected number, name, function but found {}", str),
        Token::NCName(str) | Token::Number(str) => {
            let result = create_mathml_element(&rules_with_context.get_document(), LITERAL_NAME);
            result.set_text(str);
            Ok(result)
        },
        Token::ArgRef(str) => {
            match find_arg(rules_with_context, &str[1..], mathml, false)? {
                Some(e) => Ok(e),
                None => bail!("intent arg '{}' not found", str),
            }
        }
    }
}

/// lift the children up to LITERAL_NAME
fn lift_function_name<'m>(doc: Document<'m>, function_name: Element<'m>, mut children: Vec<Element<'m>>) -> Element<'m> {
    // debug!("    lift_function_name: {}", name(&function_name));
    if name(&function_name) == LITERAL_NAME {
        set_mathml_name(function_name, as_text(function_name));
        function_name.clear_children();
        function_name.append_children(children);
        return function_name;
    } else {
        // FIX: remove -- no longer used because function names are not structured???
        // debug!("IMPLICIT_FUNCTION_NAME is being used");
        let result = create_mathml_element(&doc, IMPLICIT_FUNCTION_NAME);
        let mut new_children = Vec::with_capacity(children.len()+1);
        new_children.push(function_name);
        new_children.append(&mut children);
        result.append_children(new_children);
        return result;
    }
}


/// look for @arg=name in mathml
/// if 'check_intent', then look at an @intent for this element (typically false for non-recursive calls)
fn find_arg<'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, name: &str, mathml: Element<'c>, no_check_inside: bool) -> Result<Option<Element<'m>>> {
    // debug!("Looking for '{}' in\n{}", name, mml_to_string(&mathml));
    if let Some(mut arg_val) = mathml.attribute_value("arg") {
        // debug!("looking for '{}', found arg='{}'", name, arg_val);
        if let Some((arg_part, type_part)) = arg_val.split_once(':') {
            arg_val = arg_part;
            mathml.set_attribute_value(INTENT_TYPE, &type_part);    // ok to set more than once
        }
        if name == arg_val {
            // check to see if this mathml has an intent value -- if so the value is the value of its intent value
            if let Some(intent_str) = mathml.attribute_value("intent") {
                let mut lex_state = LexState::init(intent_str.trim())?;
                return Ok( Some( build_intent(rules_with_context, &mut lex_state, mathml)? ) ); 
            } else {
                return Ok( Some( rules_with_context.match_pattern::<Element<'m>>(mathml)? ) );
            }
        } else if no_check_inside {
            return Ok(None);       // don't look inside 'arg'
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
        if let Some(element) = find_arg(rules_with_context, name, child, true)? {
            return Ok( Some(element) );
        }
    }

    return Ok(None);               // not present
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::super::init_logger;
    use sxd_document::parser;    


    fn test_intent(mathml: &str, target: &str) -> bool {
		use crate::interface::*;
		// this forces initialization
        crate::interface::set_rules_dir(super::super::abs_rules_dir_path()).unwrap();
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
        let intent = "<binomial data-intent-hint='function'> <mn arg='n'>7</mn> <mn arg='m'>3</mn>  </binomial>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn infer_binomial_intent_arg() {
        let mathml = "<msubsup intent='$op($n,$m)'>
                <mi arg='op' intent='binomial'>C</mi>
                <mi arg='n'>n</mi>
                <mi arg='m'>m</mi>
            </msubsup>";
        let intent = "<binomial data-intent-hint='function'> <mi arg='n'>n</mi> <mi arg='m'>m</mi></binomial>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_nest_no_arg_call() {
        let mathml = "<mrow intent='foo(bar())'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<foo data-intent-hint='function'><bar data-intent-hint='function'></bar></foo>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_hints() {
        let mathml = "<mrow intent='foo@silent(bar@postfix())'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<foo data-intent-hint='silent'><bar data-intent-hint='postfix'></bar></foo>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_hints_and_type() {
        init_logger();
        let mathml = "<mrow intent='foo:is-foolish@function($b)'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b:int'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<foo data-intent-hint='function' data-intent-type='is-foolish'><mi  arg='b:int' data-intent-type='int'>b</mi></foo>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_hints_defaults() {
        let mathml = "<mrow intent='_($p@infix($a, $b, $f@postfix($x)))'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo intent='plus'>+</mo>
                <mi arg='x'>x</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<_ data-intent-hint='silent'>
                    <plus data-intent-hint='infix'>
                        <mi arg='a'>a</mi>
                        <mi arg='b'>b</mi>
                        <factorial data-intent-hint='postfix'><mi arg='x'>x</mi></factorial>
                    </plus>
                </_>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_in_intent_first_arg() {
        let mathml = "<mrow intent='p(f(b), a)'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<p data-intent-hint='function'> <f data-intent-hint='function'><intent-literal>b</intent-literal></f> <intent-literal>a</intent-literal></p>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_in_intent_second_arg() {
        let mathml = "<mrow intent='$p(a,$f(b))'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<plus data-intent-hint='function'> <intent-literal>a</intent-literal> <factorial data-intent-hint='function'><intent-literal>b</intent-literal></factorial> </plus>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_with_whitespace() {
        let mathml = "<mrow intent='  $U27F6    ( $a ,  $b,$c )  '>
                <mi arg='a'>A</mi>
                <mover>
                    <mo movablelimits='false' arg='U27F6' intent='map'>⟶</mo>
                    <mo arg='U2245' intent='congruence'>≅</mo>
                </mover>
                <mi arg='b'>B</mi>
                <mi arg='c'>C</mi>
            </mrow>";
        let intent = "<map data-intent-hint='function'> <mi arg='a'>A</mi> <mi arg='b'>B</mi> <mi arg='c'>C</mi> </map>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_with_nested_indirect_head() {
        let mathml = "<mrow intent='$op($a,$b)'>
                <mi arg='a'>A</mi>
                <mover arg='op' intent='$U27F6($U2245)'>
                    <mo movablelimits='false' arg='U27F6' intent='map'>⟶</mo>
                    <mo arg='U2245' intent='congruence'>≅</mo>
                </mover>
                <mi arg='b'>B</mi>
            </mrow>";
        let intent = "<apply-function data-intent-hint='function'><map data-intent-hint='function'> <intent-literal>congruence</intent-literal></map> <mi arg='a'>A</mi> <mi arg='b'>B</mi> </apply-function>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_with_literals() {
        let mathml = "<mrow intent='vector(1, 0., .1, -23, -.1234, last)'>
                <mi>x</mi>
            </mrow>";
        let intent = "<vector data-intent-hint='function'>
            <intent-literal>1</intent-literal><intent-literal>0.</intent-literal><intent-literal>.1</intent-literal><intent-literal>-23</intent-literal><intent-literal>-.1234</intent-literal><intent-literal>last</intent-literal>
            </vector>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_with_nested_head() {
        let mathml = "<mrow intent='$U27F6($U2245)($a,$b)'>
                <mi arg='a'>A</mi>
                <mover>
                    <mo movablelimits='false' arg='U27F6' intent='map'>⟶</mo>
                    <mo arg='U2245' intent='congruence'>≅</mo>
                </mover>
                <mi arg='b'>B</mi>
            </mrow>";
        let intent = "<apply-function data-intent-hint='function'>
                    <map><intent-literal>congruence</intent-literal></map>
                    <mi arg='a'>A</mi> <mi arg='b'>B</mi>
                </apply-function>";
        assert!(test_intent(mathml, intent));
    }


    #[test]
    fn intent_with_nested_head_and_hints() {
        let mathml = "<mrow intent='pre@prefix(in@infix($a, x))(post@postfix($b))'>
                <mi arg='a'>A</mi>
                <mover>
                    <mo intent='map'>⟶</mo>
                    <mo intent='congruence'>≅</mo>
                </mover>
                <mi arg='b'>B</mi>
            </mrow>";
        let intent = "<apply-function data-intent-hint='prefix'>
                <pre>
                    <in data-intent-hint='infix'>
                        <mi arg='a'>A</mi>
                        <intent-literal>x</intent-literal>
                    </in>
                </pre>
                <post data-intent-hint='postfix'>
                    <mi arg='b'>B</mi>
                </post>
            </apply-function>";
        assert!(test_intent(mathml, intent));
    }


    #[test]
    fn intent_at() {
        let mathml = "<mrow intent='$U27F6@prefix($U2245)($a,$b)'>
                <mi arg='a'>A</mi>
                <mover>
                    <mo movablelimits='false' arg='U27F6' intent='map'>⟶</mo>
                    <mo arg='U2245' intent='congruence'>≅</mo>
                </mover>
                <mi arg='b'>B</mi>
            </mrow>";
        let intent = "<apply-function data-intent-hint='prefix'><map> <intent-literal>congruence</intent-literal></map> <mi arg='a'>A</mi> <mi arg='b'>B</mi> </apply-function>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_missing_open() {
        let mathml = "<mrow intent='$p $a,$f($b))'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<plus data-intent-hint='function'> <mi arg='a'>a</mi> <factorial data-intent-hint='function'><mi arg='b'>b</mi></factorial> </plus>";
        assert!(!test_intent(mathml, intent));
    }

    #[test]
    fn intent_missing_comma() {
        let mathml = "<mrow intent='$p($a $f($b))'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<plus data-intent-hint='function'> <mi arg='a'>a</mi> <factorial data-intent-hint='function'><mi arg='b'>b</mi></factorial> </plus>";
        assert!(!test_intent(mathml, intent));
    }

    #[test]
    fn intent_no_arg() {
        let mathml = "<mrow intent='factorial()'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<factorial data-intent-hint='function'></factorial>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_illegal_no_arg() {
        let mathml = "<mrow intent='factorial(()))'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<factorial data-intent-hint='function'></factorial>";
        assert!(!test_intent(mathml, intent));
    }

    #[test]
    fn infer_missing_second_arg() {
        let mathml = "<mrow intent='binomial($n,)'>
                <mo>(</mo>
                <mfrac linethickness='0'> <mn arg='n'>7</mn> <mn arg='m'>3</mn> </mfrac>
                <mo>)</mo>
            </mrow>";
        let intent = "<binomial data-intent-hint='function'> <mn arg='n'>7</mn> <mn arg='m'>3</mn>  </binomial>";
        assert!(!test_intent(mathml, intent));
    }
}