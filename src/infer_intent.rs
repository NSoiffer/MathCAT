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
use crate::pretty_print::mml_to_string;
use crate::xpath_functions::is_leaf;
use regex::Regex;

pub const LITERAL_NAME: &str = "literal";
const IMPLICIT_FUNCTION_NAME: &str = "apply-function";

lazy_static! {
    static ref NUMBER: Regex = Regex::new(r"^[+-]?(\d+.?\d*|.\d+)$").unwrap();
    static ref NC_NAME: Regex = Regex::new(r"^[:\w_][:\w\-.d]*$").unwrap();  // from www.w3.org/TR/REC-xml/#sec-common-syn, with "\w" for letter ranges
    static ref ARG_REF: Regex = Regex::new(r"^\$[:\w_][:\w\-.d]*$").unwrap();  // from www.w3.org/TR/REC-xml/#sec-common-syn, with "\w" for letter ranges
}

impl<'c, 's:'c, 'r, 'm:'c> SpeechRulesWithContext<'c, 's,'m> {
}

pub fn infer_intent<'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, mathml: Element<'c>) -> Result<Element<'m>> {
    if let Some(intent_str) = mathml.attribute_value("intent") {
        let mut lex_state = LexState::init(intent_str.trim())?;
        let result = build_intent(rules_with_context, &mut lex_state, mathml) 
                    .chain_err(|| format!("in intent attribute value '{}'", intent_str))?;
        if !lex_state.remaining_str.is_empty() {
            bail!("Error in intent value: extra unparsed intent '{}'", lex_state.remaining_str);
        }
        debug!("intent attr result:\n{}", mml_to_string(&result));
        return Ok(result);
    }
    bail!("Internal error: infer_intent() called on MathML with no intent arg:\n{}", mml_to_string(&mathml));
}

// Old, more inclusive version...
// intent := literal | selector | intent '(' intent [ ',' intent ]* ')'
// literal := [letters|digits|_|-]*
// selector := argref
// argref := '$' NCName

// Build up intent from intent string and element
// For reuse, a third param controls whether function syntax is allowed (top-level call should use 'true')
// intent   := number | NCName | argref | function
// number   := ('+' | '-')? digit+ ('.' digit+)? 
// argref   := '$' NCName
// function := (NCName | argref) '(' intent [ ',' intent ]* ')'


// 'i -- "i" for the lifetime of the "intent" string
enum Token<'i> {
    NCName(&'i str),
    Number(&'i str),
    ArgRef(&'i str),
    None,
}

struct LexState<'i> {
    token: Token<'i>,
    separator: char,            // '(', ',', ')', ' '  -- ' ' anything else is an error (no more chars is ' ')
    remaining_str: &'i str,     // always trimmed
}

impl<'i> LexState<'i> {
    fn init(str: &'i str) -> Result<LexState<'i>> {
        let mut lex_state = LexState {  token: Token::None, separator: ' ', remaining_str: str.trim() };
        lex_state.get_next()?;
        return Ok(lex_state);
    }

    // helper function for LexState -- do not call outside of the impl
    fn set_token(&mut self, str: &'i str) -> Result<()> {
        // Note: 'str' is already trimmed
        if str.is_empty() {
            self.token = Token::None;
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

    fn get_next(&mut self) -> Result<&Self> {
        match self.remaining_str.find(&[',', '(', ')']) {
            None => {
               // should be a terminal or it is illegal syntax
               self.set_token(self.remaining_str)?;
            },
            Some(i) => {
                self.set_token(&self.remaining_str[..i].trim_end())?;
                self.separator = self.remaining_str.as_bytes()[i] as char;
                self.remaining_str = &self.remaining_str[i+1..].trim_start();  // end is already trimmed
            }
        }
        return Ok(self);
    }
}

// The practical restrictions of NCName are that it cannot contain several symbol characters like
//  !, ", #, $, %, &, ', (, ), *, +, ,, /, :, ;, <, =, >, ?, @, [, \, ], ^, `, {, |, }, ~, and whitespace characters
//  Furthermore an NCName cannot begin with a number, dot or minus character although they can appear later in an NCName.
fn build_intent<'b, 'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
                                          lex_state: &mut LexState<'b>,
                                          mathml: Element<'c>) -> Result<Element<'m>> {
    let element = get_element_from_token(rules_with_context, mathml, lex_state)?;

    if lex_state.separator == ' ' {
        return Ok(element); // nothing left -- all done
    } else if lex_state.separator == '(' {
        return build_function_args(rules_with_context, element, lex_state, mathml);
    } else {
        bail!("Illegal 'intent' syntax: expected '(' but found '{}'", lex_state.remaining_str);
    }

    // we have have the head of the function, now get all the children (there must be at least one)
    fn build_function_args<'b, 'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
                        function_name: Element<'m>,
                        lex_state: &mut LexState<'b>,
                        mathml: Element<'c>) -> Result<Element<'m>> {
        assert_eq!(lex_state.separator, '(');
        let mut children = Vec::with_capacity(lex_state.remaining_str.len()/3 + 1);   // conservative estimate ('3' - "$x,");
        lex_state.get_next()?;
        children.push( get_element_from_token(rules_with_context, mathml,  &lex_state)? );

        while lex_state.separator == ',' || lex_state.separator == '(' {
            if lex_state.separator == '(' {
                // 'function' -- recurse with the first child being the function name and replace it with parsed function
                let first_child = build_function_args(rules_with_context, children.pop().unwrap(), lex_state, mathml)?;
                children.push(first_child);
            } else {
                // arg of function
                children.push( get_element_from_token(rules_with_context, mathml, lex_state.get_next()? )? );
            }
        }

        if lex_state.separator == ')' {
            // done with function
            lex_state.get_next()?;    // advance so that we are prepared to consume next token
            return Ok( lift_function_name(rules_with_context.get_document(), function_name, children) );

        }
        bail!("Illegal 'intent' syntax: missing ')' in  '{}'", lex_state.remaining_str);
    }


    fn get_element_from_token<'b, 'r, 'c, 's:'c, 'm:'c>(
                rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>,
                mathml: Element<'c>,
                lex_state: &LexState<'b>,) -> Result<Element<'m>> {
        return match lex_state.token {
            Token::None => bail!("Illegal 'intent' value: empty string"),
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
        };
    }

    /// lift the children up to LITERAL_NAME
    fn lift_function_name<'m>(doc: Document<'m>, function_name: Element<'m>, mut children: Vec<Element<'m>>) -> Element<'m> {
        debug!("intent_rules name: {}", name(&function_name));
        if name(&function_name) == LITERAL_NAME {
            set_mathml_name(function_name, as_text(function_name));
            function_name.clear_children();
            function_name.append_children(children);
            return function_name;
        } else {
            // FIX: remove -- no longer used because function names are not structured???
            debug!("IMPLICIT_FUNCTION_NAME is being used");
            let result = create_mathml_element(&doc, IMPLICIT_FUNCTION_NAME);
            let mut new_children = Vec::with_capacity(children.len()+1);
            new_children.push(function_name);
            new_children.append(&mut children);
            result.append_children(new_children);
            return result;
        }
    }
}


/// look for @arg=name in mathml
/// if 'check_intent', then look at an @intent for this element (typically false for non-recursive calls)
fn find_arg<'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, name: &str, mathml: Element<'c>, no_check_inside: bool) -> Result<Option<Element<'m>>> {
    // debug!("Looking for '{}' in\n{}", name, mml_to_string(&mathml));
    if let Some(arg_val) = mathml.attribute_value("arg") {
        // debug!("looking for '{}', found arg='{}'", name, arg_val);
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

        // let result = infer_intent(&mut SpeechRulesWithContext::new(&rules.borrow(), package2.as_document(), "".to_string()), mathml);
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
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn infer_binomial_intent_arg() {
        let mathml = "<msubsup intent='$op($n,$m)'>
                <mi arg='op' intent='binomial'>C</mi>
                <mi arg='n'>n</mi>
                <mi arg='m'>m</mi>
            </msubsup>";
        let intent = "<binomial> <mi arg='n'>n</mi> <mi arg='m'>m</mi></binomial>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_in_intent() {
        let mathml = "<mrow intent='$p($a,$f($b))'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<plus> <mi arg='a'>a</mi> <factorial><mi arg='b'>b</mi></factorial> </plus>";
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
        let intent = "<map> <mi arg='a'>A</mi> <mi arg='b'>B</mi> <mi arg='c'>C</mi> </map>";
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
        let intent = "<apply-function><map> <literal>congruence</literal></map> <mi arg='a'>A</mi> <mi arg='b'>B</mi> </apply-function>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_with_literals() {
        let mathml = "<mrow intent='vector(1, 0., .1, -23, -.1234, last)'>
                <mi>x</mi>
            </mrow>";
        let intent = "<vector>
            <literal>1</literal><literal>0.</literal><literal>.1</literal><literal>-23</literal><literal>-.1234</literal><literal>last</literal>
            </vector>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_with_nested_head() {
        // nested head is illegal
        let mathml = "<mrow intent='$U27F6($U2245)($a,$b)'>
                <mi arg='a'>A</mi>
                <mover>
                    <mo movablelimits='false' arg='U27F6' intent='map'>⟶</mo>
                    <mo arg='U2245' intent='congruence'>≅</mo>
                </mover>
                <mi arg='b'>B</mi>
            </mrow>";
        let intent = "<apply-function><map> <literal>congruence</literal></map> <mi arg='a'>A</mi> <mi arg='b'>B</mi> </apply-function>";
        assert!(!test_intent(mathml, intent));
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
        let intent = "<plus> <mi arg='a'>a</mi> <factorial><mi arg='b'>b</mi></factorial> </plus>";
        assert!(!test_intent(mathml, intent));
    }

    #[test]
    fn intent_missing_arg() {
        let mathml = "<mrow intent='factorial()'>
                <mi arg='a'>a</mi>
                <mo arg='p' intent='plus'>+</mo>
                <mi arg='b'>b</mi>
                <mo arg='f' intent='factorial'>!</mo>
            </mrow>";
        let intent = "<factorial> </factorial>";
        assert!(!test_intent(mathml, intent));
    }

    #[test]
    fn infer_missing_second_arg() {
        let mathml = "<mrow intent='binomial($n,)'>
                <mo>(</mo>
                <mfrac linethickness='0'> <mn arg='n'>7</mn> <mn arg='m'>3</mn> </mfrac>
                <mo>)</mo>
            </mrow>";
        let intent = "<binomial> <mn arg='n'>7</mn> <mn arg='m'>3</mn>  </binomial>";
        assert!(!test_intent(mathml, intent));
    }
}