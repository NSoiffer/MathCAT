//! Use heuristics to infer the intent.
//! For example, an `mfrac` with `linethickness=0` would be a binomial
//! The inference is added to the MathML
//!
//! The implementation of the module is on hold until the MathML committee figures out how it wants to do this.

use sxd_document::dom::*;
use crate::speech::SpeechRulesWithContext;
use crate::canonicalize::{as_element, as_text, name, create_mathml_element,set_mathml_name};
use crate::errors::*;
use crate::pretty_print::mml_to_string;
use crate::xpath_functions::is_leaf;
use regex::Regex;

pub const LITERAL_NAME: &'static str = "literal";
const IMPLICIT_FUNCTION_NAME: &'static str = "apply-function";

lazy_static! {
    static ref LITERAL: Regex = Regex::new(r"^[\d\w_-]+$").unwrap();
    static ref ARG_REF: Regex = Regex::new(r#"^\$[\w_][^\s:`!@#\$%&*=:;'"\\/+;.,?(){}\[\]<>]*$"#).unwrap();
}

impl<'c, 's:'c, 'r, 'm:'c> SpeechRulesWithContext<'c, 's,'m> {
}

pub fn infer_intent<'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, mathml: Element<'c>) -> Result<Element<'m>> {
    if let Some(intent_str) = mathml.attribute_value("intent") {
        let (result, remaining_str) = build_intent(rules_with_context, intent_str.trim(), mathml) 
                    .chain_err(|| format!("in intent attribute value '{}'\nIgnoring 'intent' attribute", intent_str))?;
        if !remaining_str.is_empty() {
            error!("Ignoring extra characters '{}' at end of 'intent' attribute value {}", remaining_str, intent_str);
        };
        debug!("intent attr result:\n{}", mml_to_string(&result));
        return Ok(result);
    }
    bail!("Internal error: infer_intent() called on MathML with no intent arg:\n{}", mml_to_string(&mathml));
}

// intent := literal | selector | intent '(' intent [ ',' intent ]* ')'
// literal := [letters|digits|_|-]*
// selector := argref
// argref := '$' NCName
// The practical restrictions of NCName are that it cannot contain several symbol characters like
//  :, @, $, %, &, /, +, ,, ;, whitespace characters or different parenthesis.
//  Furthermore an NCName cannot begin with a number, dot or minus character although they can appear later in an NCName.
fn build_intent<'b, 'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, intent_str: &'b str, mathml: Element<'c>) -> Result<(Element<'m>, &'b str)> {
    // Note: intent_str is assumed trimmed at both ends
    if LITERAL.is_match(intent_str) {
        let result = create_mathml_element(&rules_with_context.get_document(), LITERAL_NAME);
        result.set_text(intent_str);
        return Ok( (result, &intent_str[intent_str.len()..]) );  // empty string
    } else if ARG_REF.is_match(intent_str) {
        match find_arg(rules_with_context, &intent_str[1..], mathml, false)? {
            Some(e) => return Ok( (e, &intent_str[intent_str.len()..]) ),  // empty string
            None => bail!("intent arg '{}' not found", intent_str),
        }
    } else if let Some(i) = intent_str.find('(') {
        // deal with f ( x, y, ...) where f, x, y (etc) can be "intent"
        let (function_name, before_paren) = build_intent(rules_with_context, &intent_str[..i].trim_end(), mathml)?;
        assert_eq!(before_paren, "");        // shouldn't be anything left after literal/arg ref
        debug!("function name:\n  {}", crate::pretty_print::mml_to_string(&function_name));

        let (children, rest) = build_intent_children(rules_with_context, &intent_str[i..].trim_start(), mathml)?;
        let mut head = lift_function_name(rules_with_context.get_document(), function_name, children);
        let mut rest_intent_str = rest;

        let mut terminator = rest_intent_str.find('(');
        while terminator.is_some() {
            let i = terminator.unwrap();
            let (children, rest) = build_intent_children(rules_with_context, &rest_intent_str[i..].trim_start(), mathml)?;
            head = lift_function_name(rules_with_context.get_document(), head, children);
            rest_intent_str = rest.trim_start();
            terminator = rest_intent_str.find('(');
        }
        return Ok( (head, rest_intent_str) );
    }

    bail!("illegal 'intent' syntax in '{}'", intent_str);


    fn lift_function_name<'m>(doc: Document<'m>, function_name: Element<'m>, mut children: Vec<Element<'m>>) -> Element<'m> {
        debug!("intent_rules name: {}", name(&function_name));
        if name(&function_name) == LITERAL_NAME {
            set_mathml_name(function_name, as_text(function_name));
            function_name.clear_children();
            function_name.append_children(children);
            return function_name;
        } else {
            let result = create_mathml_element(&doc, IMPLICIT_FUNCTION_NAME);
            let mut new_children = Vec::with_capacity(children.len()+1);
            new_children.push(function_name);
            new_children.append(&mut children);
            result.append_children(new_children);
            return result;
        }
    }
}

/// 'intent_str' should start with '(' -- this will return when it hits the match ')' and the return string will start after that
fn build_intent_children<'b, 'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, intent_str: &'b str, mathml: Element<'c>) -> Result<(Vec<Element<'m>>, &'b str)> {
    // loop over args building up 'new_children'
    // we are done when we hit ')' unless it is followed by '(', in which case we have a nested 'function name'
    assert_eq!(char::from(intent_str.as_bytes()[0]), '(');
    let mut children = Vec::with_capacity(intent_str.len()/3 + 1);   // conservative estimate ('3' - "$x,")

    // there must be at least one arg in '(...)'
    let mut rest_intent_str = intent_str[1..].trim_start();
    let mut terminator = rest_intent_str.find(&[',', '(', ')'][..]);

    while terminator.is_some() {
        let i = terminator.unwrap();        // found either ',' '(', or ')'
        let char_found = char::from(rest_intent_str.as_bytes()[i]);
        match char_found {
            '(' => {
                let (child, rest) = build_intent(rules_with_context, rest_intent_str, mathml)?;
                children.push(child);
                rest_intent_str = rest.trim_start();
            },
            ',' => {
                let (child, _) = build_intent(rules_with_context, &rest_intent_str[..i].trim_end(), mathml)?;
                debug!("    before ',':\n      {}", crate::pretty_print::mml_to_string(&child));
                children.push(child);    
                rest_intent_str = &rest_intent_str[i+1..].trim_start();
            }
            _ => {
                assert_eq!(char_found, ')');
                let unparsed_intent_str = &rest_intent_str[..i].trim_end();
                if !unparsed_intent_str.is_empty() {        // empty happens with f(g(x)) on second ')'
                    let (child, _) = build_intent(rules_with_context, unparsed_intent_str, mathml)?;
                    debug!("    before ')':\n      {}", crate::pretty_print::mml_to_string(&child));
                    children.push(child);        
                }
                return Ok( (children, &rest_intent_str[i+1..].trim_start() ) );
            }
        };
        terminator = rest_intent_str.find(&[',', '(', ')'][..]);
    }
    bail!("missing closing paren in '{}'", rest_intent_str);
}

/// look for @arg=name in mathml
/// if 'check_intent', then look at an @intent for this element (typically false for non-recursive calls)
fn find_arg<'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, name: &str, mathml: Element<'c>, no_check_inside: bool) -> Result<Option<Element<'m>>> {
    debug!("Looking for '{}' in\n{}", name, mml_to_string(&mathml));
    if let Some(arg_val) = mathml.attribute_value("arg") {
        debug!("looking for '{}', found arg='{}'", name, arg_val);
        if name == arg_val {
            // check to see if this mathml has an intent value -- if so the value is the value of its intent value
            if let Some(intent_str) = mathml.attribute_value("intent") {
                return Ok( Some( build_intent(rules_with_context, intent_str, mathml)?.0 ) ); 
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

// fn deep_copy(&self, e: Element<'a>) -> Element<'m> {
//     let new_element = create_mathml_element(&self.doc, name(&e));
//     for attr in e.attributes() {
//         new_element.set_attribute_value(attr.name(), attr.value());
//     }
//     let mut new_children = Vec::with_capacity(e.children().len());
//     for child in e.children() {
//         match child {
//             ChildOfElement::Element(child) => new_children.push( ChildOfElement::Element( self.deep_copy(child) ) ),
//             ChildOfElement::Text(t) => new_children.push( ChildOfElement::Text(t) ),
//             _ => panic!("deep_copy: non-element/text found in\n{}", mml_to_string(&e)),
//         }
//     }
//     new_element.append_children(new_children);
//     return new_element;
// }
    
#[cfg(test)]
mod infer_tests {
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
        let result = crate::speech::intent_from_mathml(mathml, package2.as_document()).unwrap();
        debug!("result: {}", crate::pretty_print::mml_to_string(&result));
        return crate::interface::is_same_element(&result, &target);
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
        // init_logger();
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
        // init_logger();
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
    fn intent_with_nested_head() {
        // init_logger();
        let mathml = "<mrow intent='$U27F6($U2245)($a,$b)'>
                <mi arg='a'>A</mi>
                <mover>
                    <mo movablelimits='false' arg='U27F6' intent='map'>⟶</mo>
                    <mo arg='U2245' intent='congruence'>≅</mo>
                </mover>
                <mi arg='b'>B</mi>
            </mrow>";
        let intent = "<apply-function><map> <literal>congruence</literal></map> <mi arg='a'>A</mi> <mi arg='b'>B</mi> </apply-function>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_with_whitespace() {
        // init_logger();
        let mathml = "<mrow intent='  $U27F6  (   $U2245 )  ( $a ,  $b )  '>
                <mi arg='a'>A</mi>
                <mover>
                    <mo movablelimits='false' arg='U27F6' intent='map'>⟶</mo>
                    <mo arg='U2245' intent='congruence'>≅</mo>
                </mover>
                <mi arg='b'>B</mi>
            </mrow>";
        let intent = "<apply-function><map> <literal>congruence</literal></map> <mi arg='a'>A</mi> <mi arg='b'>B</mi> </apply-function>";
        assert!(test_intent(mathml, intent));
    }

    #[test]
    fn intent_with_nested_indirect_head() {
        // init_logger();
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
}