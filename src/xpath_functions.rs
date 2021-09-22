//! XPath underlies rule matching and speech generation. The version of xpath used is based on xpath 1.0
//! and includes the ability to define functions and variables.
//! The variables defined are all the preferences and also variables set in speech rules via the `variables` keyword.
//! The function defined here are:
//! * `IsNode(node, kind)`:  returns true if the node matches the "kind".
//!    Valid values are "leaf", "simple", "common_fraction", "trig_name".
//! * `ToOrdinal(number, fractional, plural)`: converts the number to an ordinal (e.g, third)
//!   * `number` -- the number to translate
//!   * `fractional` -- true if this is a fractional ordinal (e.g, "half")
//!   * `plural` -- true if answer should be plural
//! * `ToCommonFraction(mfrac)` -- converts the fraction to an ordinal version (e.g, 2 thirds)
//! * `IsLargeOp(node)` -- returns true if the node is a large operator (e.g, integral or sum)
//! * `IsBracketed(node, left, right, requires_comma)` -- returns true if the first/last element in the mrow match `left`/`right`.
//!    If the optional `requires_comma` argument is given and is `true`, then there also must be a "," in the mrow (e.g., "f(x,y)")
//! * `DEBUG(xpath)` -- _Very_ useful function for debugging speech rules.
//!    This can be used to surround a whole or part of an xpath expression in a match or output.
//!    The result will be printed to standard output and the result returned so that `DEBUG` does not affect the computation.    
#![allow(clippy::needless_return)]

use sxd_document::dom::{Element, ChildOfElement, ParentOfChild};
use sxd_xpath::{Value, Context, context, function::*, nodeset::*};
use crate::{canonicalize::as_text, definitions::{ DEFINITIONS}};
use regex::{Regex, Captures};
use crate::pretty_print::mml_to_string;
use std::cell::Ref;
use crate::canonicalize::{as_element, name};

// useful utility functions
// note: child of an element is a ChildOfElement, so sometimes it is useful to have parallel functions,
//   one for Element and one for ChildOfElement.

// @returns {String} -- the text of the (leaf) element otherwise an empty string
fn get_text_from_element(e: &Element) -> String {
    if e.children().len() == 1 {
        if let ChildOfElement::Text(t) = e.children()[0] {
            return t.text().to_string();
        }
    }
    return "".to_string();
}

#[allow(non_snake_case)]
// Same as 'is_tag', but for ChildOfElement
fn get_text_from_COE(coe: &ChildOfElement) -> String {
    let element = coe.element();
    return match element {
        Some(e) => get_text_from_element(&e),
        None => "".to_string(),
    };
}

// make sure that there is only one node in the NodeSet
// Returns the node or either Error::ArgumentMissing or Error::TooManyArguments
fn validate_one_node(nodes: Nodeset) -> Result<Node, Error> {
    if nodes.size() == 0 {
        return Err(Error::ArgumentMissing{});
    } else if nodes.size() > 1 {
        return Err( Error::TooManyArguments{expected:1, actual: nodes.size()} );
    }
    return Ok( nodes.iter().next().unwrap() );
}

// Return true if the element's name is 'name'
fn is_tag(e: &Element, name: &str) -> bool {
    return e.name().local_part() == name;
}

#[allow(non_snake_case)]
// Same as 'is_tag', but for ChildOfElement
fn is_COE_tag(coe: &ChildOfElement, name: &str) -> bool {
    let element = coe.element();
    return element.is_some() && is_tag(&element.unwrap(), name);  
}

/// Should be an internal structure for implementation of the IsNode, but it was useful in one place in a separate module.
/// This should probably be restructured slightly.
pub struct IsNode;

impl IsNode {
    /// implements ClearSpeak's definition of "simple"
    /// this is fairly detailed, so we define a few local functions (at end) to help out
    /// Also, it doesn't help that the structure is a bit complicated Elements->ChildOfElement->Element/Text
    pub fn is_simple(elem: &Element) -> bool {
        if is_trivially_simple(elem) {
            return true;
        }

        if !is_tag(elem, "mrow") || elem.children().is_empty() {
            return false;
        }

        if is_negative_of_trivially_simple(elem) {
            // -3 or -x
            return true;
        }

        // x y or -x or -3 x or -x y or -3 x y or x° or n° or -x° or -n°
        #[allow(clippy::if_same_then_else)]
        if is_times_mi(elem) {
            return true;    // x y
        } else if is_degrees(elem) {
            return true;    // x° or n°
        } else if is_function(elem) {
            return true;
        }

        return false;


        // returns the element's text value
        fn to_str<'a>(e: &'a Element) -> &'a str {
            // typically usage assumes 'e' is a leaf
            // bad MathML is the following isn't true
            if e.children().len() == 1 {
                let text_node = e.children()[0];
                if let Some(t) = text_node.text() {
                    return t.text();
                }
            }               
            return "";
        }

        // same as 'to_str' but for ChildOfElement
        fn coe_to_str<'a>(coe: &'a ChildOfElement) -> &'a str {
            // typically usage assumes 'coe' is a leaf
            let element_node = coe.element();
            if let Some(e) = element_node {
                // bad MathML is the following isn't true
                if e.children().len() == 1 {
                    let text_node = e.children()[0];
                    if let Some(t) = text_node.text() {
                        return t.text();
                    }
                }
            }               
            return "";
        }

        // returns true if the string is just a single *char* (which can be multiple bytes)
        fn is_single_char(str: &str) -> bool {
            let mut chars =  str.chars();
            let first_char = chars.next();
            let second_char = chars.next();
            return first_char.is_some() && second_char.is_none();
        }

        // checks the single element to see if it is simple (mn, mi that is a single char, common fraction)
        fn is_trivially_simple(elem: &Element) -> bool {
            if is_tag(elem, "mn")  {
                return true;
            }
            if is_tag(elem, "mi") && is_single_char(to_str(&elem)) {
                // "simple" only if it is a single char (which can be multiple bytes)
                return true;
            }

            // FIX: need to consult preference Fraction_Ordinal
            if IsNode::is_common_fraction(elem, 10, 19) {
                return true;
            }
            return false;
        }

        // true if the negative of a single element that is simple
        fn is_negative_of_trivially_simple(elem: &Element) -> bool {
            if is_tag(elem, "mrow") && elem.children().len() == 2 {
                let children = elem.children();
                // better be negative of something at this point...
                if is_COE_tag(&children[0], "mo") && is_equal(&children[0], '-') &&
                   children[1].element().is_some() && is_trivially_simple(&children[1].element().unwrap()) {
                    return true;
                }
            }
            return false;
        }

        // return true if ChildOfElement has exactly text 'ch'
        fn is_equal(coe: &ChildOfElement, ch: char) -> bool {
            return coe_to_str(coe).starts_with(ch);
        }

        // true if mrow(xxx, &it;, mi) or mrow(xxx, &it; mi, &it;, mi) where mi's have len==1
        fn is_times_mi(mrow: &Element) -> bool {
            assert!( is_tag(mrow, "mrow") );
            let children = mrow.children();
            if !(children.len() == 3 || children.len() == 5) {
                return false;
            }
            if children[0].element().is_none() {
                return false;
            }

            let first_child = children[0].element().unwrap();
            if !is_trivially_simple(&first_child) {
                if !is_negative_of_trivially_simple(&first_child) {
                    return false;
                }
                if children.len() == 5 && !is_COE_tag(&first_child.children()[1], "mn") {
                    return false;      // '-x y z' is too complicated () -- -2 x y is ok
                }
            }

            if !(is_COE_tag(&children[1], "mo") && 
                    is_equal(&children[1], '\u{2062}') &&
                 is_COE_tag(&children[2], "mi") &&
                    coe_to_str(&children[2]).len()==1 ) {
                return false;
            }

            if children.len() == 3 {
                return true;
            }

            // len == 5
            return  is_COE_tag(&children[3], "mo") && 
                        is_equal(&children[3], '\u{2062}') &&       // invisible times
                    is_COE_tag(&children[4], "mi") &&
                        coe_to_str(&children[4]).len()==1 ;
        }

        // return true if the mrow is var° or num°
        fn is_degrees(mrow: &Element) -> bool {
            assert!( is_tag(mrow, "mrow") );
            let children = mrow.children();
            return children.len() == 2 &&
                is_equal(&children[1], '°') &&
                (is_COE_tag(&children[0], "mi") ||
                 is_COE_tag(&children[0], "mn") );
        }

        // fn_name &af; [simple arg or (simple arg)]
        fn is_function(mrow: &Element) -> bool {
            assert!( is_tag(mrow, "mrow") );
            let children = mrow.children();
            if children.len() != 3 {
                return false;
            }
            if !(is_COE_tag(&children[1], "mo") && 
                 is_equal(&children[1], '\u{2061}') ) {    // invisible function application
                return false;
            }
            if !is_COE_tag(&children[0], "mi") {
                return false;
            }
            let function_arg = children[2].element().unwrap();
            if IsBracketed::is_bracketed(&function_arg, "(", ")", false) {
                return IsNode::is_simple(&function_arg.children()[1].element().unwrap());
            } else {
                return IsNode::is_simple(&function_arg);
            }
        }
    }
    
    // returns true if the text of 'e' is in definitions.trig_function_names
    fn is_trig_name(e: &Element) -> bool {
        if e.name().local_part() != "mi" {
            return false;
        }

        let mi_text = get_text_from_element(e);
        return DEFINITIONS.with(|definitions| {
            return definitions.trig_function_names.as_hashset().borrow().contains(&mi_text);
        });
    }

    // Returns true if 'frac' is a common fraction
    // In this case, the numerator and denominator can be no larger than 'num_limit' and 'denom_limit'
    fn is_common_fraction(frac: &Element, num_limit: usize, denom_limit: usize) -> bool {
        lazy_static! {
            static ref ALL_DIGITS: Regex = Regex::new(r"\d+").unwrap();    // match one or more digits
        }

        if !is_tag(frac, "mfrac") {
            return false;
        }
        let children = frac.children();
        if children.len() != 2 {
            return false;
        }

        let num = children[0].element();
        let denom = children[1].element();
        if num.is_none() || denom.is_none() {
            return false;
        };

        let num = num.unwrap();
        let denom = denom.unwrap();
        if !is_tag(&num, "mn") || !is_tag(&denom, "mn") {
            return false
        };

        let num = get_text_from_element(&num);
        let denom = get_text_from_element(&denom);
        if num.is_empty() || denom.is_empty() {
            return false;
        }

        return ALL_DIGITS.is_match(&num)   && is_small_enough(&num, num_limit) &&
               ALL_DIGITS.is_match(&denom) && is_small_enough(&denom, denom_limit);

        fn is_small_enough(val: &str, upper_bound: usize) -> bool {
            return if let Ok(value) = val.parse::<usize>() { value <= upper_bound } else { false };
        }
    }

    fn is_punctuation(node:  &Element) -> bool {
        lazy_static! {
            // list of chars from rule VI (p41) [various dashes are from Unicode, not green book]
            static ref PUNCTUATION: Regex = Regex::new(r"':,–—―⸺⸻—…!-.?‘’“”;'").unwrap();    // match one or more digits
        }

        let text = get_text_from_element(node);
        return PUNCTUATION.is_match(&text);
    }
}

const MATHML_LEAF_NODES: &[&str] = &["mi", "mo", "mn", "mtext", "ms", "mspace", "mglyph"];
impl Function for IsNode {
    // eval function for IsNode
    // errors happen for wrong number/kind of arg
    fn evaluate<'c, 'd>(&self,
                        _context: &context::Evaluation<'c, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {

        let mut args = Args(args);
        args.exactly(2)?;
        let kind = args.pop_string()?;
        // FIX: there is some conflict problem with xpath errors and error-chain
        //                .chain_err(|e| format!("Second arg to is_leaf is not a string: {}", e.to_string()))?;
        match kind.as_str() {
            "simple" | "leaf" | "common_fraction" | "trig_name" | "nemeth_punctuation" => (), 
            _ => return Err( Error::ArgumentMissing ),
        };

        let nodes = args.pop_nodeset()?;
        if nodes.size() == 0 {
            return Err(Error::ArgumentMissing{});
        };
        return Ok(
            Value::Boolean( 
                nodes.iter()
                    .all(|node|
                        if let Node::Element(e) = node {
                            match kind.as_str() {
                                "simple" => IsNode::is_simple(&e),
                                "leaf"   => MATHML_LEAF_NODES.contains(&e.name().local_part()),
                                "trig_name" => IsNode::is_trig_name(&e),
                                "common_fraction" => IsNode::is_common_fraction(&e, usize::MAX, usize::MAX), 
                                "nemeth_punctuation" => IsNode::is_punctuation(&e),
                                _        => true,       // can't happen due to check above
                            }    
                        } else {
                            // xpath is something besides an element, so no match
                            false
                        }
                    )
            )
        );
    }
}

struct ToOrdinal;
impl ToOrdinal {
    /**
     * Translates a number of up to twelve digits into a string representation.
     *   number -- the number to translate
     *   fractional -- true if this is a fractional ordinal (e.g, "half")
     *   plural -- true if answer should be plural
     * Returns the string representation of that number or an error message
     */
    fn convert(number: &str, fractional: bool, plural: bool) -> String {
        lazy_static! {
            static ref NO_DIGIT: Regex = Regex::new(r"[^\d]").unwrap();    // match anything except a digit
        }
        DEFINITIONS.with(|definitions| {
            // check to see if the number is too big or is not an integer or has non-digits
            if number.is_empty() || number.len() > 3*definitions.numbers_large.as_vec().borrow().len() || number.contains(".,") {
                return String::from(number);
            }
            if NO_DIGIT.is_match(number) {
                // this shouldn't have been part of an mn, so likely an error. Log a warning
                // FIX: log a warning that a non-number was passed to convert()
                return String::from(number);
            }

            // first deal with the abnormalities of fractional ordinals (one half, etc). That simplifies what remains
            if fractional {
                if let Some(string) = ToOrdinal::compute_irregular_fractional_speech(number, plural) {
                    return string;
                }
            }

            // at this point, we only need to worry about singular/plural distinction

            // break into groups of three digits and add 10^3 word (thousands, millions, ...) after each chunk
            // FIX: add a pause between groups of three -- need to use TTS-specific pause

            // handle special case of trailing zeros
            // num_thousands_at_end represents the amount to shift NumbersLarge... (e.g., millions->thousands)
            let num_thousands_at_end = match number.rfind(|ch| ch > '0') { // last non-0 on right
                Some(n) => (number.len() - 1 - n) / 3 ,
                None => 0
            };
            let (number,_) = number.split_at(number.len() - 3 * num_thousands_at_end); // drop the 0s

            // everything is simplified if we add zeros at the start so that block size is a factor of 3
            let number = match number.len() % 3 {
                0 => "".to_string() + number,
                1 => "00".to_string() + number,
                _ => "0".to_string() + number, // can only be "2" -- compiler doesn't know there aren't other options
            };

            // At this point we have at least three "digits", and length is a multiple of 3
            // We have already verified that there are only ASCII digits, so we can subtract '0' to get an index
            const ASCII_0: usize = 48;
            let digits = number.as_bytes()
                        .iter()
                        .map(|&byte| byte as usize - ASCII_0)
                        .collect::<Vec<usize>>();

            let mut answer = String::with_capacity(255);  // reasonable max most of the time
            let large_words = definitions.numbers_large.as_vec().borrow();
            if digits.len() > 3 { 
                // speak this first groups as cardinal numbers
                let words = [
                    definitions.numbers_hundreds.as_vec().borrow(),
                    definitions.numbers_tens.as_vec().borrow(),
                    definitions.numbers_ones.as_vec().borrow()
                ];
                answer = digits[0..digits.len()-3]
                            .chunks(3)
                            .enumerate()
                            .map(|(i, chunk)| {
                                if chunk[0] != 0 || chunk[1] != 0 || chunk[2] != 0 {
                                    ToOrdinal::hundreds_to_words(chunk, &words) + " " + 
                                        &large_words[num_thousands_at_end + digits.len()/3 - 1 - i] + " "
                                } else {
                                    "".to_string()
                                }
                                })
                            .collect::<Vec<String>>()
                            .join("");  // can't use " " because 1000567 would get extra space in the middle
                if num_thousands_at_end > 0 {
                    // add on "billionths", etc and we are done
                    let large_words = if plural {&definitions.numbers_ordinal_plural_large} else {&definitions.numbers_ordinal_large};
                    let large_words = large_words.as_vec().borrow();
                        return answer + &large_words[num_thousands_at_end];
                }
            };

            // all that is left is to speak the hundreds part, possibly followed by "thousands", "billions", etc
            let words = match (num_thousands_at_end > 0, plural) {
                (true, _) => [
                    definitions.numbers_hundreds.as_vec().borrow(),
                    definitions.numbers_tens.as_vec().borrow(),
                    definitions.numbers_ones.as_vec().borrow()
                ],
                (false, true) => [
                    definitions.numbers_ordinal_plural_hundreds.as_vec().borrow(),
                    definitions.numbers_ordinal_plural_tens.as_vec().borrow(),
                    definitions.numbers_ordinal_plural_ones.as_vec().borrow()
                ],
                (false, false) => [
                    definitions.numbers_ordinal_hundreds.as_vec().borrow(),
                    definitions.numbers_ordinal_tens.as_vec().borrow(),
                    definitions.numbers_ordinal_ones.as_vec().borrow()
                ],
            };
            answer += &ToOrdinal::hundreds_to_words(&digits[digits.len()-3..], &words);
            if num_thousands_at_end > 0 {
                let large_words = if plural {&definitions.numbers_ordinal_plural_large} else {&definitions.numbers_ordinal_large};
                let large_words = large_words.as_vec().borrow();
                answer = answer + " " + &large_words[num_thousands_at_end];
            }
            return answer;
        })
    }

    // ordinals often have an irregular start (e.g., "half") before becoming regular.
    // if the number is irregular, return the ordinal form, otherwise return 'None'.
    fn compute_irregular_fractional_speech(number: &str, plural: bool) -> Option<String> {
        DEFINITIONS.with(|definitions| {
            let words = if plural {
                definitions.numbers_ordinal_fractional_plural_ones.as_vec().borrow()
            } else {
                definitions.numbers_ordinal_fractional_ones.as_vec().borrow()
            };
            let number_as_int: usize = number.parse().unwrap(); // already verified it is only digits
            if number_as_int < words.len() {
                // use the words associated with this irregular pattern.
                return Some( words[number_as_int].clone() );
            };
            return None;
        })
    }

    fn hundreds_to_words(number: &[usize], words: &[Ref<Vec<String>>; 3]) -> String {
        assert!( number.len() == 3 );
        return DEFINITIONS.with(|definitions| {
            if number[0] != 0 && number[1] == 0 && number[2] == 0 {
                return words[0][number[0]].clone();
            }

            let mut hundreds = definitions.numbers_hundreds.as_vec().borrow()[number[0]].clone();
            if !hundreds.is_empty() {
                hundreds += " ";
            }

            if number[1] != 0 && number[2] == 0 {
                return hundreds + &words[1][number[1]];
            }

            if 10*number[1] < words[2].len() {
                // usurp regular ordering to handle something like '14'
                return hundreds + &words[2][10*number[1] + number[2]];
            } else {
                return hundreds + &definitions.numbers_tens.as_vec().borrow()[number[1]] + " " + &words[2][number[2]];
            }
        });
    }
}

impl Function for ToOrdinal {
    // convert a node to an ordinal number
    fn evaluate<'c, 'd>(&self,
                        _context: &context::Evaluation<'c, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(1)?;
        let node = validate_one_node(args.pop_nodeset()?)?;
        return match node {
            Node::Text(t) =>  Ok( Value::String( ToOrdinal::convert(t.text(), false, false) ) ),
            Node::Element(e) => Ok( Value::String( ToOrdinal::convert(&get_text_from_element(&e), false, false) ) ),
            _   =>  Err( Error::ArgumentNotANodeset{actual: ArgumentType::String} ),
        }
    }
}


struct ToCommonFraction;

impl Function for ToCommonFraction {
    // convert a node to a common fraction (if the numerator and denominator are within given limits)
    fn evaluate<'c, 'd>(&self,
                        _context: &context::Evaluation<'c, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(1)?;

        // FIX: should probably handle errors by logging them and then trying to evaluate any children
        let node = validate_one_node(args.pop_nodeset()?)?;
        if let Node::Element(frac) = node {
            if !IsNode::is_common_fraction(&frac, usize::MAX, usize::MAX) {
                return Err( Error::Other( format!("ToCommonFraction -- argument is not an 'mfrac': {}': ", mml_to_string(&frac))) );
            }
    
            // everything has been verified, so we can just get the pieces and ignore potential error results
            let children = frac.children();
            let num = children[0].element().unwrap();
            let num =   get_text_from_element( &num );
            let denom = children[1].element().unwrap();
            let denom = get_text_from_element( &denom );
            let mut answer = num.clone() + " ";
            answer += &ToOrdinal::convert(&denom, true, num!="1");
            return Ok( Value::String( answer ) );    
        } else {
            return Err( Error::Other( "ToCommonFraction -- argument is not an element".to_string()) );
        }
    }
}

struct Min;
/**
 * Returns true the smallest of the two args
 * @param(num1) 
 * @param(num2)
 */
 impl Function for Min {

    fn evaluate<'c, 'd>(&self,
                        _context: &context::Evaluation<'c, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(2)?;
        let num1 = args.pop_number()?;
        let num2 = args.pop_number()?;
        return Ok( Value::Number( num1.min(num2) ) );
    }
}

struct Max;

impl Function for Max {

    fn evaluate<'c, 'd>(&self,
                        _context: &context::Evaluation<'c, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(2)?;
        let num1 = args.pop_number()?;
        let num2 = args.pop_number()?;
        return Ok( Value::Number( num1.max(num2) ) );
    }
}


struct NestingChars;
const NEMETH_FRAC_LEVEL: &'static str = "nemeth-frac-level";    // name of attr where value is cached
const FIRST_CHILD_ONLY: &[&str] = &["mroot", "msub", "msup", "msubsup", "munder", "mover", "munderover", "mmultiscripts"];
impl NestingChars {
    // returns a 'repeat_char' corresponding to the Nemeth rules for nesting
    // note: this value is likely one char too long because the starting fraction is counted
    fn nemeth_frac_value<'a>(node: &'a Element, repeat_char: &'a str) -> String {
        let children = node.children();
        let name = name(&node);
        if MATHML_LEAF_NODES.contains(&name) {
            return "".to_string();
        } else if name == "mfrac" {
            // have we already computed the value?
            if let Some(value) = node.attribute_value(NEMETH_FRAC_LEVEL) {
                return value.to_string();
            }

            let num_value = NestingChars::nemeth_frac_value(&as_element(children[0]), repeat_char);
            let denom_value = NestingChars::nemeth_frac_value(&as_element(children[1]), repeat_char);
            let mut max_value = if num_value.len() > denom_value.len() {num_value} else {denom_value};
            max_value += repeat_char;
            node.set_attribute_value(NEMETH_FRAC_LEVEL, &max_value);
            return max_value;
        } else if FIRST_CHILD_ONLY.contains(&name) {
            // only look at the base -- ignore scripts/index
            return NestingChars::nemeth_frac_value(&as_element(children[0]), repeat_char);
        } else {
            let mut result = "".to_string();
            for child in children {
                let value = NestingChars::nemeth_frac_value(&as_element(child), repeat_char);
                if value.len() > result.len() {
                    result = value;
                }
            }
            return result;
        }
    }

    fn nemeth_root_value<'a>(node: &'a Element, repeat_char: &'a str) -> Result<String, Error> {
        // returns the correct number of repeat_chars to use
        // note: because the highest count is toward the leaves and
        //    because this is a loop and not recursive, caching doesn't work without a lot of overhead
        let parent = node.parent().unwrap();
        if let ParentOfChild::Element(e) =  parent {
            let mut parent = e;
            let mut result = "".to_string();
            loop {
                let name = name(&parent);
                if name == "math" {
                    return Ok( result );
                }
                if name == "msqrt" || name == "mroot" {
                    result += repeat_char;
                }
                let parent_of_child = parent.parent().unwrap();
                if let ParentOfChild::Element(e) =  parent_of_child {
                    parent = e;
                } else {
                    return Err( Error::Other("Internal error in nemeth_root_value: didn't find 'math' tag".to_string()) );
                }
            }
        }
        return Err( Error::Other("Internal error in nemeth_root_value: didn't find 'math' tag".to_string()) );
    }
}

impl Function for NestingChars {
/**
 * Returns a string with the correct number of nesting chars (could be an empty string)
 * @param(node) -- current node
 * @param(char) -- char (string) that should be repeated
 * Note: as a side effect, an attribute with the value so repeated calls to this or a child will be fast
 */
 fn evaluate<'c, 'd>(&self,
                        _context: &context::Evaluation<'c, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(2)?;
        let repeat_char = args.pop_string()?;
        let node = validate_one_node(args.pop_nodeset()?)?;
        if let Node::Element(el) = node {
            let name = name(&el);
            // it is likely a bug to call this one a non mfrac
            if name == "mfrac" {
                // because it is called on itself, the fraction is counted one too many times -- chop one off
                // this is slightly messy because we are chopping off a char, not a byte
                const BRAILLE_BYTE_LEN: usize = "⠹".len();      // all Unicode braille symbols have the same number of bytes
                return Ok( Value::String( NestingChars::nemeth_frac_value(&el, &repeat_char)[BRAILLE_BYTE_LEN..].to_string() ) );
            } else if name == "msqrt" || name == "mroot" {
                return Ok( Value::String( NestingChars::nemeth_root_value(&el, &repeat_char)? ) );
            } else {
                panic!("NestingChars chars should be used only on 'mfrac'. '{}' was passed in", name);
            }
        } else {
            // not an element, so nothing to do
            return Ok( Value::String("".to_string()) );
        }
    }
}

struct NemethChars;
impl NemethChars {
    // returns a string for the chars in the *leaf* node.
    // this string follows the Nemeth rules typefaces and deals with mathvariant
    //  which has partially turned chars to the alphanumeric block
    fn get_nemeth_chars<'a>(node: &'a Element) -> Result<String, Error> {
        lazy_static! {
            // To greatly simplify typeface/language generation, the chars have unique ASCII chars for them:
            // Typeface: S: sans-serif, B: bold, T: script/blackboard, I: italic, R: Roman
            // Language: E: English, D: German, G: Greek, V: Greek variants, H: Hebrew, U: Russian
            // Indicators: N: number, P: punctuation, M: multipurpose
            static ref PICK_APART_CHAR: Regex = 
                Regex::new(r"(?P<face>[SBTIR]*)(?P<lang>[EDGVHU]??)(?P<cap>[⠠]??)(?P<num>[N]??)(?P<char>[⠴⠂⠆⠒⠲⠢⠖⠶⠦⠔⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵])").unwrap();
        }
    
        let math_variant = node.attribute_value("mathvariant");
        // FIX: cover all the options -- use phf::Map
        let  attr_typeface = match math_variant {
            None => "R",
            Some(variant) => match variant {
                "bold" => "B",
                "italic" => "I",
                "double-struck" | "script"=> "S",
                "fraktur" => "G",
                "sans-serif" => "T",
                _ => "R",       // normal and unknown
            },
        };
        return crate::speech::BRAILLE_RULES.with(|rules| {
            let text = as_text(*node);
            // start of pattern matching mutably borrows BRAILLE_RULES, so we can't use borrow.
            //   instead hack to get around borrow rules because we know no changes happen during call
            let braille_chars = unsafe {
                rules.as_ptr().as_ref().unwrap()
                    .replace_chars(text, node).unwrap_or("".to_string())
            };
            println!("braille_chars: '{}'", braille_chars);
            
            // we want to pull the prefix (typeface, language) out to the front until a change happens
            // the same is true for number indicator
            // also true (sort of) for capitalization -- if all caps, use double cap in front (assume abbr or Roman Numeral)
            let is_in_enclosed_list = name(node) == "mn" && NemethChars::is_in_enclosed_list(*node);
            let mut typeface = "".to_string();     // illegal value to force first value
            let mut is_all_caps = true;
            let result = PICK_APART_CHAR.replace_all(&braille_chars, |caps: &Captures| {
                println!("  face: {:?}, lang: {:?}, num {:?}, cap: {:?}, char: {:?}",
                        &caps["face"], &caps["lang"], &caps["num"], &caps["cap"], &caps["char"]);
                let mut nemeth_chars = "".to_string();
                let typeface_changed =  &typeface != &caps["face"];
                if typeface_changed {
                    typeface = caps["face"].to_string();   // needs to outlast this instance of the loop
                    nemeth_chars += if typeface.is_empty() {attr_typeface} else {&typeface};
                    nemeth_chars +=  &caps["lang"];
                }
                println!("is_in_list: {}; num: {}", is_in_enclosed_list, caps["num"].is_empty());
                if !caps["num"].is_empty() && (typeface_changed || !is_in_enclosed_list) {
                    nemeth_chars += "N";
                }
                is_all_caps &= !&caps["cap"].is_empty();
                nemeth_chars += &caps["cap"];       // will be stripped later if all caps
                nemeth_chars += &caps["char"];
                return nemeth_chars;
            });
            let mut text_chars = text.chars();     // see if more than one char
            if is_all_caps && text_chars.next().is_some() &&  text_chars.next().is_some() {
                return Ok( "⠠⠠".to_string() + &result.replace("⠠", ""));
            } else {
                return Ok( result.to_string() );
            }
        });
    }

    fn is_in_enclosed_list(node: Element) -> bool {
        // Nemeth Rule 10 defines an enclosed list:
        // 1: begins and ends with fence
        // 2: FIX: not implemented -- must contain no word, abbreviation, ordinal or plural ending
        // 3: function names or signs of shape and the signs which follow them are a single item (not a word)
        // 4: an item of the list may be an ellipsis or any sign used for mission
        // 5: no relational operator may appear within the list
        // 6: the list must have at least 2 items.
        //       Items are separated by commas, can not have other punctuation (except ellipsis and dash)
        let mut parent = node.parent().unwrap().element().unwrap(); // safe since 'math' is always at root
        while name(&parent) == "mrow" {
            if IsBracketed::is_bracketed(&parent, "", "", true) {
                for child in parent.children() {
                    if !child_meets_conditions(as_element(child)) {
                        return false;
                    }
                }
                return true;
            }
            parent = parent.parent().unwrap().element().unwrap();
        }
        return false;

        fn child_meets_conditions(node: Element) -> bool {
            let name = name(&node);
            return match name {
                "mi" | "mn" => true,
                "mo"  => !crate::canonicalize::is_relational_op(node),
                "mtext" => false, // FIX -- should be more nuanced,
                "mrow" => {
                    if IsBracketed::is_bracketed(&node, "", "", false) {
                        return child_meets_conditions(as_element(node.children()[1]));
                    } else {
                        for child in node.children() {
                            if !child_meets_conditions(as_element(child)) {
                                return false;
                            }
                        }
                    }  
                    true      
                },
                _ => {
                    for child in node.children() {
                        if !child_meets_conditions(as_element(child)) {
                            return false;
                        }
                    }
                    true
                },
            }
        }
    }
}

impl Function for NemethChars {
    /**
     * Returns a string with the correct number of nesting chars (could be an empty string)
     * @param(node) -- current node
     * @param(char) -- char (string) that should be repeated
     * Note: as a side effect, an attribute with the value so repeated calls to this or a child will be fast
     */
     fn evaluate<'c, 'd>(&self,
                            _context: &context::Evaluation<'c, 'd>,
                            args: Vec<Value<'d>>)
                            -> Result<Value<'d>, Error>
        {
            let mut args = Args(args);
            args.exactly(1)?;
            let node = validate_one_node(args.pop_nodeset()?)?;
            if let Node::Element(el) = node {
                assert!( MATHML_LEAF_NODES.contains(&name(&el)) );
                return Ok( Value::String( NemethChars::get_nemeth_chars(&el)? ) );
            } else {
                // not an element, so nothing to do
                return Ok( Value::String("".to_string()) );
            }
        }
    }
    

struct IsLargeOp;
/**
 * Returns true if the node is a large op
 * @param(node)     -- node(s) to test -- should be an <mo>
 */
 impl Function for IsLargeOp {

    fn evaluate<'c, 'd>(&self,
                        _context: &context::Evaluation<'c, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(1)?;
        let node = validate_one_node(args.pop_nodeset()?)?;
        if let Node::Element(e) = node {
            if !is_tag(&e, "mo") {
                return Ok( Value::Boolean(false) );
            }
            return DEFINITIONS.with(|definitions| {
                let text = get_text_from_element(&e);
                return Ok( Value::Boolean(definitions.large_operators.as_hashset().borrow().get(&text).is_some()) );
            });
        } else {
            // xpath is something besides an element, so no match
            return Ok( Value::Boolean(false) );
        }
    }
}

struct Debug;
/**
 * Returns true if the node is a large op
 * @param(node)     -- node(s) to test -- should be an <mo>
 */
 impl Function for Debug {

    fn evaluate<'c, 'd>(&self,
                        _context: &context::Evaluation<'c, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(2)?;
        let xpath_str = args.pop_string()?;
        let eval_result = &args[0];
        // print!("  -- Debug: value of '{}' is '{:?}'", xpath_str, eval_result);
        print!("  -- Debug: value of '{}' is ", xpath_str);
        match eval_result {
            Value::Nodeset(nodes) => {
                if nodes.size() == 0 {
                    println!("0 nodes (false)");
                } else {
                    let singular = nodes.size()==1;
                    println!("{} node{}. {}:", nodes.size(),
                        if singular {""} else {"s"},
                        if singular {"Node is"} else {"Nodes are"});
                    nodes.document_order()
                        .iter()
                        .enumerate()
                        .for_each(|(i, node)| {
                            match node {
                                Node::Element(mathml) => println!("#{}:\n{}",
                                        i, mml_to_string(mathml)),
                                _ => println!("'{:?}'", node),
                            }   
                        })    
                }
            },
            _ => println!("'{:?}'", eval_result),
        }
        return Ok( eval_result.clone() );
    }
}


/// Should be an internal structure for implementation of the IsBracketed, but it was useful in one place in a separate module.
/// This should probably be restructured slightly.
pub struct IsBracketed;
impl IsBracketed {
    pub fn is_bracketed(element: &Element, left: &str, right: &str, requires_comma: bool) -> bool {
        use crate::canonicalize::is_fence;
        if !is_tag(&element, "mrow") {
            return false;
        }
        let children = element.children();
        let n_children = children.len();
        if (!left.is_empty() && !right.is_empty() && n_children < 2) ||
           requires_comma && element.children().len() < 3 {
            // not enough argument for there to be a match
            return false;
        }

        let first_child = as_element(children[0]);
        let last_child = as_element(children[children.len()-1]);
        if (left.is_empty()  && (name(&first_child) != "mo" || !is_fence(first_child))) ||
           (right.is_empty() && (name(&last_child) != "mo"  || !is_fence(last_child))) {
            return false;
        }

        if !left.is_empty() && get_text_from_COE(&children[0]) != left ||
           !right.is_empty() && get_text_from_COE(&children[children.len()-1]) != right {
            // left or right don't match
            return false;
        }

        if requires_comma {
            if let ChildOfElement::Element(contents) = children[1] {
                let children = contents.children();
                if !is_tag(&contents, "mrow") || children.len() <= 1 {
                    return false;
                }
                // finally, we can check for a comma -- assume operand followed by a comma
                if get_text_from_COE(&children[1]).as_str() == "," {
                    return true;
                }
            }
            return false;
        } else {
            return true;
        }
    }
}

/**
 * Returns true if the node is a bracketed expr with the indicated left/right chars
 * node -- node(s) to test
 * left -- string (like "[") or empty
 * right -- string (like "]") or empty
 * requires_comma - boolean, optional (check the top level of 'node' for commas
 */
// 'requiresComma' is useful for checking parenthesized expressions vs function arg lists and other lists
 impl Function for IsBracketed {
    fn evaluate<'c, 'd>(&self,
                        _context: &context::Evaluation<'c, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.at_least(3)?;
        args.at_most(4)?;
        let mut requires_comma = false;
        if args.len() == 4 {
            requires_comma = args.pop_boolean()?;
        }
        let right = args.pop_string()?;
        let left = args.pop_string()?;
        let node = validate_one_node(args.pop_nodeset()?)?;
        if let Node::Element(e) = node {
            return Ok( Value::Boolean( IsBracketed::is_bracketed(&e, &left, &right, requires_comma) ) );
        }

        // FIX: should having a non-element be an error instead??
        return Ok( Value::Boolean(false) );
    }
}



/// Add all the functions defined in this module to `context`.
pub fn add_builtin_functions(context: &mut Context) {
    // FIX: should be a static cache that gets regenerated on update
    context.set_function("min", Min);       // missing in xpath 1.0
    context.set_function("max", Max);       // missing in xpath 1.0
    context.set_function("NestingChars", NestingChars);
    context.set_function("NemethChars", NemethChars);
    context.set_function("IsNode", IsNode);
    context.set_function("ToOrdinal", ToOrdinal);
    context.set_function("ToCommonFraction", ToCommonFraction);
    context.set_function("IsLargeOp", IsLargeOp);
    context.set_function("IsBracketed", IsBracketed);
    context.set_function("DEBUG", Debug);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::PathBuf};
    use sxd_document::parser;
    use crate::interface::{trim_element, get_element};


    fn init_word_list() {
        let result = crate::definitions::read_definitions_file(&[
            Some(PathBuf::from("Rules/en/definitions.yaml")),
            None,
            None
        ]);
        if let Err(e) = result {
            panic!("unable to read 'Rules/en/definitions.yaml\n{}", e.to_string());
        }
    }

    #[test]
    fn ordinal_one_digit() {
        init_word_list();
        assert_eq!("zeroth", ToOrdinal::convert("0", false, false));
        assert_eq!("second", ToOrdinal::convert("2", false, false));
        assert_eq!("ninth", ToOrdinal::convert("9", false, false));

        assert_eq!("zeroth", ToOrdinal::convert("0", false, true));
        assert_eq!("seconds", ToOrdinal::convert("2", false, true));
        assert_eq!("ninths", ToOrdinal::convert("9", false, true));

        assert_eq!("first", ToOrdinal::convert("1", true, false));
        assert_eq!("half", ToOrdinal::convert("2", true, false));
        assert_eq!("half", ToOrdinal::convert("02", true, false));
        assert_eq!("ninth", ToOrdinal::convert("9", true, false));

        assert_eq!("halves", ToOrdinal::convert("2", true, true));
        assert_eq!("halves", ToOrdinal::convert("002", true, true));
        assert_eq!("ninths", ToOrdinal::convert("9", true, true));
    }

    #[test]
    fn ordinal_two_digit() {
        init_word_list();
        assert_eq!("tenth", ToOrdinal::convert("10", false, false));
        assert_eq!("seventeenth", ToOrdinal::convert("17", false, false));
        assert_eq!("thirty second", ToOrdinal::convert("32", false, false));
        assert_eq!("fortieth", ToOrdinal::convert("40", false, false));

        assert_eq!("tenths", ToOrdinal::convert("10", false, true));
        assert_eq!("sixteenths", ToOrdinal::convert("16", false, true));
        assert_eq!("eighty eights", ToOrdinal::convert("88", false, true));
        assert_eq!("fiftieths", ToOrdinal::convert("50", false, true));

        assert_eq!("eleventh", ToOrdinal::convert("11", true, false));
        assert_eq!("forty fourth", ToOrdinal::convert("44", true, false));
        assert_eq!("ninth", ToOrdinal::convert("9", true, false));
        assert_eq!("ninth", ToOrdinal::convert("00000009", true, false));
        assert_eq!("sixtieth", ToOrdinal::convert("60", true, false));

        assert_eq!("tenths", ToOrdinal::convert("10", true, true));
        assert_eq!("tenths", ToOrdinal::convert("0010", true, true));
        assert_eq!("elevenths", ToOrdinal::convert("11", true, true));
        assert_eq!("nineteenths", ToOrdinal::convert("19", true, true));
        assert_eq!("twentieths", ToOrdinal::convert("20", true, true));
    }

    #[test]
    fn ordinal_three_digit() {
        init_word_list();
        assert_eq!("one hundred first", ToOrdinal::convert("101", false, false));
        assert_eq!("two hundred tenth", ToOrdinal::convert("210", false, false));
        assert_eq!("four hundred thirty second", ToOrdinal::convert("432", false, false));
        assert_eq!("four hundred second", ToOrdinal::convert("402", false, false));

        assert_eq!("one hundred first", ToOrdinal::convert("101", true, false));
        assert_eq!("two hundred second", ToOrdinal::convert("202", true, false));
        assert_eq!("four hundred thirty second", ToOrdinal::convert("432", true, false));
        assert_eq!("five hundred third", ToOrdinal::convert("503", true, false));

        assert_eq!("three hundred elevenths", ToOrdinal::convert("311", false, true));
        assert_eq!("four hundred ninety ninths", ToOrdinal::convert("499", false, true));
        assert_eq!("nine hundred ninetieths", ToOrdinal::convert("990", false, true));
        assert_eq!("six hundred seconds", ToOrdinal::convert("602", false, true));

        assert_eq!("seven hundredths", ToOrdinal::convert("700", true, true));
        assert_eq!("one hundredths", ToOrdinal::convert("100", true, true));
        assert_eq!("eight hundred seventeenths", ToOrdinal::convert("817", true, true));
    }
    #[test]
    fn ordinal_large() {
        init_word_list();
        assert_eq!("one thousandth", ToOrdinal::convert("1000", false, false));
        assert_eq!("two thousand one hundredth", ToOrdinal::convert("2100", false, false));
        assert_eq!("thirty thousandth", ToOrdinal::convert("30000", false, false));
        assert_eq!("four hundred thousandth", ToOrdinal::convert("400000", false, false));

        assert_eq!("four hundred thousandth", ToOrdinal::convert("400000", true, false));
        assert_eq!("five hundred thousand second", ToOrdinal::convert("500002", true, false));
        assert_eq!("six millionth", ToOrdinal::convert("6000000", true, false));
        assert_eq!("sixty millionth", ToOrdinal::convert("60000000", true, false));

        assert_eq!("seven billionths", ToOrdinal::convert("7000000000", false, true));
        assert_eq!("eight trillionths", ToOrdinal::convert("8000000000000", false, true));
        assert_eq!("nine quadrillionths", ToOrdinal::convert("9000000000000000", false, true));
        assert_eq!("one quintillionth", ToOrdinal::convert("1000000000000000000", false, false));

        assert_eq!("nine billion eight hundred seventy six million five hundred forty three thousand two hundred tenths", ToOrdinal::convert("9876543210", true, true));
        assert_eq!("nine billion five hundred forty three thousand two hundred tenths", ToOrdinal::convert("9000543210", true, true));
        assert_eq!("zeroth", ToOrdinal::convert("00000", false, false));
    }


    fn test_is_simple(message: &'static str, mathml_str: &'static str) {
		// this forces initialization
		crate::speech::SPEECH_RULES.with(|_| true);
        let package = parser::parse(mathml_str)
        .expect("failed to parse XML");
        let mathml = get_element(&package);
        trim_element(&mathml);
        assert!(IsNode::is_simple(&mathml), "{}", message);
    }
    fn test_is_not_simple(message: &'static str, mathml_str: &'static str) {
		// this forces initialization
		crate::speech::SPEECH_RULES.with(|_| true);
        let package = parser::parse(mathml_str)
        .expect("failed to parse XML");
        let mathml = get_element(&package);
        trim_element(&mathml);
        assert!(!IsNode::is_simple(&mathml), "{}", message);
    }
    #[test]
    fn is_simple() {
        test_is_simple("single variable", "<mi>x</mi>");
        test_is_simple("single number", "<mn>1.2</mn>");
        test_is_simple("negative number", "<mrow><mo>-</mo><mn>10</mn></mrow>");
        test_is_simple("negative variable", "<mrow><mo>-</mo><mi>x</mi></mrow>");
        test_is_simple("ordinal fraction", "<mfrac><mn>3</mn><mn>4</mn></mfrac>");
        test_is_simple("x y", "<mrow><mi>x</mi><mo>&#x2062;</mo><mi>y</mi></mrow>");
        test_is_simple("negative two vars", 
                "<mrow><mrow><mo>-</mo><mi>x</mi></mrow><mo>&#x2062;</mo><mi>y</mi></mrow>");
        test_is_simple("-2 x y", 
                "<mrow><mrow><mo>-</mo><mn>2</mn></mrow>
                             <mo>&#x2062;</mo><mi>x</mi><mo>&#x2062;</mo><mi>z</mi></mrow>");
        test_is_simple("sin x", "<mrow><mi>sin</mi><mo>&#x2061;</mo><mi>x</mi></mrow>");
        test_is_simple("f(x)", "<mrow><mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow>");
        test_is_simple("f(x+y)",
         "<mrow><mi>f</mi><mo>&#x2061;</mo>\
            <mrow><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></mrow></mrow>");
        
    }

    #[test]
    fn is_not_simple() {
        test_is_not_simple("multi-char variable", "<mi>rise</mi>");
        test_is_not_simple("large ordinal fraction", "<mfrac><mn>30</mn><mn>4</mn></mfrac>");
        test_is_not_simple("fraction with var in numerator", "<mfrac><mi>x</mi><mn>4</mn></mfrac>");
        test_is_not_simple("square root", "<msqrt><mi>x</mi></msqrt>");
        test_is_not_simple("subscript", "<msub><mi>x</mi><mn>4</mn></msub>");
        test_is_not_simple("-x y z", 
                "<mrow><mrow><mo>-</mo><mi>x</mi></mrow>
                            <mo>&#x2062;</mo><mi>y</mi><mo>&#x2062;</mo><mi>z</mi></mrow>");
    }
}