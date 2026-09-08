#![allow(clippy::needless_return)]
//! XPath underlies rule matching and speech generation. The version of xpath used is based on xpath 1.0
//! and includes the ability to define functions and variables.
//! The variables defined are all the preferences and also variables set in speech rules via the `variables` keyword.
//! The function defined here are:
//! * `IsNode(node, kind)`:  returns true if the node matches the "kind".
//!   Valid values are "leaf", "2D", "simple", "common_fraction", "trig_name".
//! * `ToOrdinal(number, fractional, plural)`: converts the number to an ordinal (e.g, third)
//!   * `number` -- the number to translate
//!   * `fractional` -- true if this is a fractional ordinal (e.g, "half")
//!   * `plural` -- true if answer should be plural
//! * `ToCommonFraction(mfrac)` -- converts the fraction to an ordinal version (e.g, 2 thirds)
//! * `IsLargeOp(node)` -- returns true if the node is a large operator (e.g, integral or sum)
//! * `IsBracketed(node, left, right, requires_comma)` -- returns true if the first/last element in the mrow match `left`/`right`.
//!   If the optional `requires_comma` argument is given and is `true`, then there also must be a "," in the mrow (e.g., "f(x,y)")
//! * `DEBUG(xpath)` -- _Very_ useful function for debugging speech rules.
//!   This can be used to surround a whole or part of an xpath expression in a match or output.
//!   The result will be printed to standard output and the result returned so that `DEBUG` does not affect the computation.    

use sxd_document_no_unsafe::dom::{Element, ChildOfElement};
use sxd_document_no_unsafe::{as_str, as_qname};
use sxd_xpath_no_unsafe::{Value, Context, context, function::*, nodeset::*};
use crate::definitions::{Definitions, SPEECH_DEFINITIONS, BRAILLE_DEFINITIONS};
use regex::Regex;
use crate::pretty_print::mml_to_string;
use std::{cell::{Ref, RefCell}, collections::HashMap};
use log::{debug, error, warn};
use std::sync::LazyLock;
use std::thread::LocalKey;
use phf::phf_set;
use sxd_xpath_no_unsafe::function::Error as XPathError;
use crate::canonicalize::{as_element, name, get_parent, MATHML_FROM_NAME_ATTR};

// useful utility functions
// note: child of an element is a ChildOfElement, so sometimes it is useful to have parallel functions,
//   one for Element and one for ChildOfElement.

// @returns {String} -- the text of the (leaf) element otherwise an empty string
fn get_text_from_element(e: Element) -> String {
    if e.children().len() == 1 &&
       let ChildOfElement::Text(t) = e.children()[0] {
            return t.text().to_string();
        }
    return "".to_string();
}

#[allow(non_snake_case)]
// Same as 'is_tag', but for ChildOfElement
fn get_text_from_COE(coe: &ChildOfElement) -> String {
    coe.element().map_or_else(String::new, get_text_from_element)
}

// make sure that there is only one node in the NodeSet
// Returns the node or an Error
pub fn validate_one_node<'n>(nodes: Nodeset<'n>, func_name: &str) -> Result<Node<'n>, Error> {
    if nodes.size() == 0 {
        return Err(Error::Other { what: format!("Missing argument for {func_name}") });
    } else if nodes.size() > 1 {
        return Err( Error::Other { what: format!("{} arguments for {}; expected 1 argument", nodes.size(), func_name) } );
    }
    return Ok( nodes.iter().next().unwrap() );
}

// Return true if the element's name is 'name'
fn is_tag(e: Element, tag_name: &str) -> bool {
    return name(e) == tag_name || e.attribute_value(MATHML_FROM_NAME_ATTR).as_deref().unwrap_or_default() == tag_name;
}

#[allow(non_snake_case)]
// Same as 'is_tag', but for ChildOfElement
fn is_COE_tag(coe: ChildOfElement, name: &str) -> bool {
    coe.element().is_some_and(|element| is_tag(element, name))
}

/// Should be an internal structure for implementation of the IsNode, but it was useful in one place in a separate module.
/// This should probably be restructured slightly.
pub struct IsNode;

impl IsNode {
    /// implements ClearSpeak's definition of "simple"
    /// this is fairly detailed, so we define a few local functions (at end) to help out
    /// Also, it doesn't help that the structure is a bit complicated Elements->ChildOfElement->Element/Text
    pub fn is_simple(elem: Element) -> bool {
        if is_trivially_simple(elem) {
            return true;
        }

        if is_negative_of_trivially_simple(elem) {
            // -3 or -x
            return true;
        }

        if !is_tag(elem, "mrow") || elem.children().is_empty() {
            return false;
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
        fn to_str(e: Element<'_>) -> String {
            if e.children().len() == 1 {
                let text_node = e.children()[0];
                if let Some(t) = text_node.text() {
                    return t.text().to_string();
                }
            }               
            return String::new();
        }

        fn coe_to_str(coe: ChildOfElement<'_>) -> String {
            let element_node = coe.element();
            if let Some(e) = element_node
                && e.children().len() == 1
            {
                let text_node = e.children()[0];
                if let Some(t) = text_node.text() {
                    return t.text().to_string();
                }
            }
            return String::new();
        }

        // returns true if the string is just a single *char* (which can be multiple bytes)
        fn is_single_char(str: &str) -> bool {
            let mut chars =  str.chars();
            return chars.next().is_some() && chars.next().is_none();
        }

        // checks the single element to see if it is simple (mn, mi that is a single char, common fraction)
        fn is_trivially_simple(elem: Element) -> bool {
            if is_tag(elem, "mn")  {
                return true;
            }
            if is_tag(elem, "mi") && is_single_char(&to_str(elem)) {
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
        fn is_negative_of_trivially_simple(elem: Element) -> bool {
            if is_tag(elem, "mrow") && elem.children().len() == 2 {
                let children = elem.children();
                // better be negative of something at this point...
                if is_COE_tag(children[0], "mo") && is_equal(children[0], '-') &&
                   children[1].element().is_some() && is_trivially_simple(children[1].element().unwrap()) {
                    return true;
                }
            }
            if is_tag(elem, "minus") && elem.children().len() == 1 {
                let child = elem.children()[0];
                if let Some(e) = child.element() {
                    return is_trivially_simple(e);
                }
            }

            return false;
        }

        // return true if ChildOfElement has exactly text 'ch'
        fn is_equal(coe: ChildOfElement, ch: char) -> bool {
            return coe_to_str(coe).starts_with(ch);
        }

        // true if mrow(xxx, &it;, mi) or mrow(xxx, &it; mi, &it;, mi) where mi's have len==1
        fn is_times_mi(mrow: Element) -> bool {
            assert!( is_tag(mrow, "mrow") );
            let children = mrow.children();
            if !(children.len() == 3 || children.len() == 5) {
                return false;
            }
            if children[0].element().is_none() {
                return false;
            }

            let first_child = children[0].element().unwrap();
            if !is_trivially_simple(first_child) {
                if !is_negative_of_trivially_simple(first_child) {
                    return false;
                }
                if children.len() == 5 && 
                   ( (name(first_child) == "minus" && first_child.children().len() == 1 && !is_COE_tag(first_child.children()[0], "mn")) ||
                     (name(first_child) == "mrow"  && !is_COE_tag(first_child.children()[1], "mn")) ) {
                    return false;      // '-x y z' is too complicated () -- -2 x y is ok
                }
            }

            if !(is_COE_tag(children[1], "mo") && 
                    is_equal(children[1], '\u{2062}') &&
                 is_COE_tag(children[2], "mi") &&
                    coe_to_str(children[2]).len()==1 ) {
                return false;
            }

            if children.len() == 3 {
                return true;
            }

            // len == 5
            return  is_COE_tag(children[3], "mo") && 
                        is_equal(children[3], '\u{2062}') &&       // invisible times
                    is_COE_tag(children[4], "mi") &&
                        coe_to_str(children[4]).len()==1 ;
        }

        // return true if the mrow is var° or num°
        fn is_degrees(mrow: Element) -> bool {
            assert!( is_tag(mrow, "mrow") );
            let children = mrow.children();
            return children.len() == 2 &&
                is_equal(children[1], '°') &&
                (is_COE_tag(children[0], "mi") ||
                 is_COE_tag(children[0], "mn") );
        }

        // fn_name &af; [simple arg or (simple arg)]
        fn is_function(mrow: Element) -> bool {
            assert!( is_tag(mrow, "mrow") );
            let children = mrow.children();
            if children.len() != 3 {
                return false;
            }
            if !(is_COE_tag(children[1], "mo") && 
                 is_equal(children[1], '\u{2061}') ) {    // invisible function application
                return false;
            }
            if !is_COE_tag(children[0], "mi") {
                return false;
            }
            let function_arg = children[2].element().unwrap();
            if IsBracketed::is_bracketed(function_arg, "(", ")", false, false) {
                return IsNode::is_simple(function_arg.children()[1].element().unwrap());
            } else {
                return IsNode::is_simple(function_arg);
            }
        }
    }

    // Returns true if 'frac' is a common fraction
    // In this case, the numerator and denominator can be no larger than 'num_limit' and 'denom_limit'
    fn is_common_fraction(frac: Element, num_limit: usize, denom_limit: usize) -> bool {
        static ALL_DIGITS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap()); // match one or more digits

        if !is_tag(frac, "mfrac") &&  !is_tag(frac, "fraction"){
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
        if !is_tag(num, "mn") || !is_tag(denom, "mn") {
            return false
        };

        let num = get_text_from_element(num);
        let denom = get_text_from_element(denom);
        if num.is_empty() || denom.is_empty() {
            return false;
        }

        return ALL_DIGITS.is_match(&num)   && is_small_enough(&num, num_limit) &&
               ALL_DIGITS.is_match(&denom) && is_small_enough(&denom, denom_limit);

        fn is_small_enough(val: &str, upper_bound: usize) -> bool {
            return if let Ok(value) = val.parse::<usize>() { value <= upper_bound } else { false };
        }
    }

    pub fn is_mathml(elem: Element) -> bool {
        // doesn't check MATHML_FROM_NAME_ATTR because we are interested in if it is an intent.
        return ALL_MATHML_ELEMENTS.contains(as_str!(name(elem)));
    }

    #[allow(non_snake_case)]
    pub fn is_2D(elem: Element) -> bool {
        return MATHML_2D_NODES.contains(as_str!(elem.attribute_value(MATHML_FROM_NAME_ATTR).unwrap_or(name(elem))));
    }

    pub fn is_scripted(elem: Element) -> bool {
        return MATHML_SCRIPTED_NODES.contains(as_str!(elem.attribute_value(MATHML_FROM_NAME_ATTR).unwrap_or(name(elem))));
    }

    pub fn is_modified(elem: Element) -> bool {
        return MATHML_MODIFIED_NODES.contains(as_str!(elem.attribute_value(MATHML_FROM_NAME_ATTR).unwrap_or(name(elem))));
    }
    }

/// All MathML elements, including a few that get cleaned away
/// "semantics", "annotation-xml", "annotation" and Content MathML are not included
static ALL_MATHML_ELEMENTS: phf::Set<&str> = phf_set!{
    "mi", "mo", "mn", "mtext", "ms", "mspace", "mglyph",
    "mfrac", "mroot", "msub", "msup", "msubsup","munder", "mover", "munderover", "mmultiscripts",
    "mstack", "mlongdiv", "msgroup", "msrow", "mscarries", "mscarry", "msline",
    "none", "mprescripts", "malignmark", "maligngroup",
    "math", "msqrt", "merror", "mpadded", "mphantom", "menclose", "mtd", "mstyle",
    "mrow", "a", "mfenced", "mtable", "mtr", "mlabeledtr",
};

static MATHML_LEAF_NODES: phf::Set<&str> = phf_set! {
    "mi", "mo", "mn", "mtext", "ms", "mspace", "mglyph",
    "none", "annotation", "ci", "cn", "csymbol",    // content could be inside an annotation-xml (faster to allow here than to check lots of places)
};


// Should mstack and mlongdiv be included here?
static MATHML_2D_NODES: phf::Set<&str> = phf_set! {
    "mfrac", "msqrt", "mroot", "menclose",
    "msub", "msup", "msubsup", "munder", "mover", "munderover", "mmultiscripts",
    "mtable", "mtr", "mlabeledtr", "mtd",
};

// Should mstack and mlongdiv be included here?
static MATHML_MODIFIED_NODES: phf::Set<&str> = phf_set! {
    "msub", "msup", "msubsup", "munder", "mover", "munderover", "mmultiscripts",
};

// Should mstack and mlongdiv be included here?
static MATHML_SCRIPTED_NODES: phf::Set<&str> = phf_set! {
    "msub", "msup", "msubsup", "mmultiscripts",
};

pub fn is_leaf(element: Element) -> bool {
    return MATHML_LEAF_NODES.contains(as_str!(name(element)));
}

impl Function for IsNode {
    // eval function for IsNode
    // errors happen for wrong number/kind of arg
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {

        let mut args = Args(args);
        args.exactly(2)?;
        let kind = args.pop_string()?;
        // FIX: there is some conflict problem with xpath errors and error-chain
        //                .chain_err(|e| format!("Second arg to is_leaf is not a string: {}", e.to_string()))?;
        match kind.as_str() {
            "simple" | "leaf" | "common_fraction" | "2D" | "modified" | "scripted" | "mathml" => (), 
            _ => return Err( Error::Other { what: format!("Unknown argument value '{}' for IsNode",  kind.as_str()) } ),
        };

        let nodes = args.pop_nodeset()?;
        if nodes.size() == 0 {
            return Ok (Value::Boolean(false));  // like xpath, don't make this an error
        };
        return Ok(
            Value::Boolean( 
                nodes.iter()
                    .all(|node|
                        if let Node::Element(e) = node {
                            match kind.as_str() {
                                "simple" => IsNode::is_simple(e),
                                "leaf"   => is_leaf_any_name(e),
                                "2D" => IsNode::is_2D(e),
                                "modified" => IsNode::is_modified(e),
                                "scripted" => IsNode::is_scripted(e),
                                "mathml" => IsNode::is_mathml(e),
                                "common_fraction" => IsNode::is_common_fraction(e, usize::MAX, usize::MAX), 
                                _        => true,       // can't happen due to check above
                            }    
                        } else {
                            // xpath is something besides an element, so no match
                            false
                        }
                    )
            )
        );

        fn is_leaf_any_name(e: Element) -> bool {
            let children = e.children();
            if children.is_empty() {
                return true;
            } else if children.len() == 1 &&
                      let ChildOfElement::Text(_) = children[0] {
                    return true;
                }
            return false
        }
    }
}

struct ToOrdinal;
impl ToOrdinal {
    // ordinals often have an irregular start (e.g., "half") before becoming regular.
    // if the number is irregular, return the ordinal form, otherwise return 'None'.
    fn compute_irregular_fractional_speech(number: &str, plural: bool) -> Option<String> {
        SPEECH_DEFINITIONS.with(|definitions| {
            let definitions = definitions.borrow();
            let words = if plural {
                definitions.get_vec("NumbersOrdinalFractionalPluralOnes")?
            } else {
                definitions.get_vec("NumbersOrdinalFractionalOnes")?
            };
            let number_as_int: usize = number.parse().unwrap(); // already verified it is only digits
            if number_as_int < words.len() {
                // use the words associated with this irregular pattern.
                return Some( words[number_as_int].clone() );
            };
            return None;
        })
    }

    /**
     * Translates a number of up to twelve digits into a string representation.
     *   number -- the number to translate
     *   fractional -- true if this is a fractional ordinal (e.g, "half")
     *   plural -- true if answer should be plural
     * Returns the string representation of that number or an error message
     */
    fn convert(number: &str, fractional: bool, plural: bool) -> Option<String> {
        static NO_DIGIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^\d]").unwrap()); // match anything except a digit
        return SPEECH_DEFINITIONS.with(|definitions| {
            let definitions = definitions.borrow();
            let numbers_large = definitions.get_vec("NumbersLarge")?;

            let pref_manager = crate::prefs::PreferenceManager::get();
            let pref_manager = pref_manager.borrow();
            let block_separators = pref_manager.pref_to_string("BlockSeparators");
            let decimal_separator = pref_manager.pref_to_string("DecimalSeparators");
            // check number validity (has digits, not a decimal)
            if number.is_empty() ||  number.contains(&decimal_separator) {
                return Some(String::from(number));
            }
            // remove any block separators
            let number = match clean_number(number, &block_separators) {
                None => return Some(String::from(number)),
                Some(num) => num,
            };
    
            // check to see if the number is too big or is not an integer or has non-digits
            if number.len() > 3*numbers_large.len() {
                return Some(number);
            }
            if NO_DIGIT.is_match(&number) {
                // this shouldn't have been part of an mn, so likely an error. Log a warning
                // FIX: log a warning that a non-number was passed to convert()
                return Some(number);
            }

            // first deal with the abnormalities of fractional ordinals (one half, etc). That simplifies what remains
            if fractional &&
               let Some(string) = ToOrdinal::compute_irregular_fractional_speech(&number, plural) {
                    return Some(string);
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
            let large_words = numbers_large;
            if digits.len() > 3 { 
                // speak this first groups as cardinal numbers
                let words = [
                    definitions.get_vec("NumbersHundreds")?,
                    definitions.get_vec("NumbersTens")?,
                    definitions.get_vec("NumbersOnes")?,
                ];
                answer = digits[0..digits.len()-3]
                            .chunks(3)
                            .enumerate()
                            .map(|(i, chunk)| {
                                if chunk[0] != 0 || chunk[1] != 0 || chunk[2] != 0 {
                                    Some(ToOrdinal::hundreds_to_words(chunk, &words)? + " " + 
                                        &large_words[num_thousands_at_end + digits.len()/3 - 1 - i] + " ")
                                } else {
                                    Some("".to_string())
                                }
                            })
                            .collect::<Option<Vec<String>>>()?
                            .join("");  // can't use " " because 1000567 would get extra space in the middle
                if num_thousands_at_end > 0 {
                    // add on "billionths", etc and we are done
                    let large_words = if plural {
                        definitions.get_vec("NumbersOrdinalPluralLarge")
                    } else {
                        definitions.get_vec("NumbersOrdinalLarge")
                    };
                    return Some(answer + &large_words?[num_thousands_at_end]);
                }
            };

            // all that is left is to speak the hundreds part, possibly followed by "thousands", "billions", etc
            let words = match (num_thousands_at_end > 0, plural) {
                (true, _) => [
                    definitions.get_vec("NumbersHundreds")?,
                    definitions.get_vec("NumbersTens")?,
                    definitions.get_vec("NumbersOnes")?,
                ],
                (false, true) => [
                    definitions.get_vec("NumbersOrdinalPluralHundreds")?,
                    definitions.get_vec("NumbersOrdinalPluralTens")?,
                    definitions.get_vec("NumbersOrdinalPluralOnes")?,
                ],
                (false, false) => [
                    definitions.get_vec("NumbersOrdinalHundreds")?,
                    definitions.get_vec("NumbersOrdinalTens")?,
                    definitions.get_vec("NumbersOrdinalOnes")?,
                ],
            };
            answer += &ToOrdinal::hundreds_to_words(&digits[digits.len()-3..], &words)?;
            if num_thousands_at_end > 0 {
                let large_words = if plural {
                    definitions.get_vec("NumbersOrdinalPluralLarge")?
                } else {
                    definitions.get_vec("NumbersOrdinalLarge")?
                };
                answer = answer + " " + &large_words[num_thousands_at_end];
            }
            return Some(answer);
        });

        /// Remove block separators and convert alphanumeric digits to ascii digits
        fn clean_number(number: &str, block_separators: &str) -> Option<String> {
            let mut answer = String::with_capacity(number.len());
            for ch in number.chars() {
                if block_separators.contains(ch) {
                    continue;
                }
                if ch.is_ascii_digit() {
                    answer.push(ch);
                } else {
                    let shifted_ch = match ch {
                        '𝟎'..='𝟗' => ch as u32 -'𝟎' as u32 + '0' as u32,
                        '𝟘'..='𝟡' => ch as u32 -'𝟘' as u32 + '0' as u32,
                        '𝟢'..='𝟫' => ch as u32 -'𝟢' as u32 + '0' as u32,
                        '𝟬'..='𝟵' => ch as u32 -'𝟬' as u32 + '0' as u32,
                        '𝟶'..='𝟿' => ch as u32 -'𝟶' as u32 + '0' as u32,
                        _ => return None,
                    };
                    answer.push(char::from_u32(shifted_ch).unwrap());
                }
            }
            return Some(answer);
        }
    }


    fn hundreds_to_words(number: &[usize], words: &[Ref<Vec<String>>; 3]) -> Option<String> {
        assert!( number.len() == 3 );
        return SPEECH_DEFINITIONS.with(|definitions| {
            let definitions = definitions.borrow();
            if number[0] != 0 && number[1] == 0 && number[2] == 0 {
                return Some(words[0][number[0]].clone());
            }

            let mut hundreds = definitions.get_vec("NumbersHundreds")?[number[0]].clone();
            if !hundreds.is_empty() {
                hundreds += " ";
            }

            if number[1] != 0 && number[2] == 0 {
                return Some(hundreds + &words[1][number[1]]);
            }

            if 10*number[1] < words[2].len() {
                // usurp regular ordering to handle something like '14'
                return Some(hundreds + &words[2][10*number[1] + number[2]]);
            } else {
                return Some(hundreds + &definitions.get_vec("NumbersTens")?[number[1]] + " " + &words[2][number[2]]);
            }
        });
    }
}

impl Function for ToOrdinal {
    // convert a node to an ordinal number
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        if let Err(e) = args.exactly(1).or_else(|_| args.exactly(3)) {
            return Err( XPathError::Other { what: format!("ToOrdinal requires 1 or 3 args: {e}") });
        };
        let mut fractional = false;
        let mut plural = false;
        if args.len() == 3 {
            plural = args.pop_boolean()?;
            fractional = args.pop_boolean()?;
        }
        let node = validate_one_node(args.pop_nodeset()?, "ToOrdinal")?;
        return match node {
            Node::Text(t) =>  Ok( Value::String(
                match ToOrdinal::convert(as_str!(t.text()), fractional, plural) {
                    None => t.text().to_string(),
                    Some(ord) => ord,
                } ) ),
            Node::Element(e) => Ok( Value::String(
                match ToOrdinal::convert(&get_text_from_element(e), fractional, plural) {
                    None => get_text_from_element(e).to_string(),
                    Some(ord) => ord,
                } ) ),
            _   =>  Err( Error::ArgumentNotANodeset{actual: ArgumentType::String} ),
        }
    }
}


struct ToCommonFraction;

impl Function for ToCommonFraction {
    // convert a node to a common fraction (if the numerator and denominator are within given limits)
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(1)?;

        // FIX: should probably handle errors by logging them and then trying to evaluate any children
        let node = validate_one_node(args.pop_nodeset()?, "ToCommonFraction")?;
        if let Node::Element(frac) = node {
            if !IsNode::is_common_fraction(frac, usize::MAX, usize::MAX) {
                return Err( Error::Other { what: format!("ToCommonFraction -- argument is not an 'mfrac': {}': ", mml_to_string(frac)) } );
            }
    
            // everything has been verified, so we can just get the pieces and ignore potential error results
            let children = frac.children();
            let num = children[0].element().unwrap();
            let num =   get_text_from_element( num );
            let denom = children[1].element().unwrap();
            let denom = get_text_from_element( denom );
            let mut answer = num.clone() + " ";
            answer += &match ToOrdinal::convert(&denom, true, num!="1") {
                None => denom,
                Some(ord) => ord,
            };

            return Ok( Value::String( answer ) )
        } else {
            return Err( Error::Other { what: "ToCommonFraction -- argument is not an element".to_string() } );
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

    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
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

    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
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


struct BaseNode;
/**
 * Returns true if the node is a large op
 * @param(node)     -- node(s) to test -- should be an <mo>
 */
 impl BaseNode {
    /// Recursively find the base node
    /// The base node of a non scripted element is the element itself
    fn base_node(node: Element) -> Element {
        let name = as_str!(node.attribute_value(MATHML_FROM_NAME_ATTR).unwrap_or(name(node)));
        if ["msub", "msup", "msubsup", "munder", "mover", "munderover", "mmultiscripts"].contains(&name) {
            return BaseNode::base_node(as_element(node.children()[0]));
        } else {
            return node;
        }
    }
 }
 impl Function for BaseNode {

    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(1)?;
        let node = validate_one_node(args.pop_nodeset()?, "BaseNode")?;
        if let Node::Element(e) = node {
            let mut node_set = Nodeset::new();
            node_set.add(BaseNode::base_node(e));
            return Ok( Value::Nodeset(node_set) );
        } else {
            // xpath is something besides an element, so no match
            return Err( Error::Other { what: "Argument other than a node given to BaseNode".to_string() } );
        }
    }
}


struct IfThenElse;
 impl Function for IfThenElse {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let args = Args(args);
        args.exactly(3)?;
        let if_val = &args[0];
        let then_val = &args[1];
        let else_val = &args[2];
        let is_true = match if_val {
            Value::Nodeset(nodes) => nodes.size() > 0,
            Value::Boolean(b) => *b,
            Value::Number(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
        };
        return Ok( if is_true {then_val.clone()} else {else_val.clone()});
    }
}


struct Debug;
/**
 * Prints it's argument along with the string that was evaluated
 * @param(node)     -- node(s) to be evaluated/printed
 * @param(string)   -- string showing what is being evaluated
 */
 impl Function for Debug {

    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(2)?;
        let xpath_str = args.pop_string()?;
        let eval_result = &args[0];
        debug!("  -- Debug: value of '{xpath_str}' is ");
        match eval_result {
            Value::Nodeset(nodes) => {
                if nodes.size() == 0 {
                    debug!("0 nodes (false)");
                } else {
                    let singular = nodes.size()==1;
                    debug!("{} node{}. {}:", nodes.size(),
                        if singular {""} else {"s"},
                        if singular {"Node is"} else {"Nodes are"});
                    nodes.document_order()
                        .iter()
                        .enumerate()
                        .for_each(|(i, node)| {
                            match node {
                                Node::Element(mathml) => debug!("#{}:\n{}",
                                        i, mml_to_string(*mathml)),
                                _ => debug!("'{node:?}'"),
                            }   
                        })    
                }
            },
            _ => debug!("'{eval_result:?}'"),
        }
        return Ok( eval_result.clone() );
    }
}


/// Should be an internal structure for implementation of the IsBracketed, but it was useful in one place in a separate module.
/// This should probably be restructured slightly.
pub struct IsBracketed;
impl IsBracketed {
    pub fn is_bracketed(element: Element, left: &str, right: &str, requires_comma: bool, requires_mrow: bool) -> bool {
        use crate::canonicalize::is_fence;
        if requires_mrow && !is_tag(element, "mrow") {
            return false;
        }
        let children = element.children();
        let n_children = children.len();
        if (n_children == 0 ||
            !left.is_empty() && !right.is_empty() && n_children < 2) ||
            requires_comma && element.children().len() < 3 {
            // not enough argument for there to be a match
            return false;
        }

        let first_child = as_element(children[0]);
        let last_child = as_element(children[children.len()-1]);
        // debug!("first_child: {}", crate::pretty_print::mml_to_string(first_child));
        // debug!("last_child: {}", crate::pretty_print::mml_to_string(last_child));
        if (left.is_empty()  && (name(first_child) != "mo" || !is_fence(first_child))) ||
           (right.is_empty() && (name(last_child) != "mo"  || !is_fence(last_child))) {
            return false;
        }

        if (!left.is_empty() && get_text_from_COE(&children[0]) != left) ||
           (!right.is_empty() && get_text_from_COE(&children[children.len()-1]) != right) {
            // left or right don't match
            return false;
        }

        if requires_comma {
            if let ChildOfElement::Element(contents) = children[1] {
                let children = contents.children();
                if !is_tag(contents, "mrow") || children.len() <= 1 {
                    return false;
                }
                // finally, we can check for a comma -- we might not have operands, so we to check first and second entry
                if get_text_from_COE(&children[0]).as_str() == "," {
                    return true;
                }
                if children.len() > 1 && get_text_from_COE(&children[1]).as_str() == "," {
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
 * requires_comma - boolean, optional (check the top level of 'node' for commas)
 */
// 'requiresComma' is useful for checking parenthesized expressions vs function arg lists and other lists
 impl Function for IsBracketed {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.at_least(3)?;
        args.at_most(5)?;
        let mut requires_comma = false;
        let mut requires_mrow = true;
        if args.len() == 5 {
            requires_mrow = args.pop_boolean()?;
        }
        if args.len() >= 4 {
            requires_comma = args.pop_boolean()?;
        }
        let right = args.pop_string()?;
        let left = args.pop_string()?;
        return Ok( Value::Boolean(
            match validate_one_node(args.pop_nodeset()?, "IsBracketed") {
                Err(_) => false,  // be fault tolerant, like xpath,
                Ok(node) => {
                    if let Node::Element(e) = node {
                        IsBracketed::is_bracketed(e, &left, &right, requires_comma, requires_mrow)
                    } else {
                        false
                    }
                }
            }) );
        }
}

pub struct IsInDefinition;
impl IsInDefinition {
    /// Returns true if `test_str` is in `set_name`
    /// Returns an error if `set_name` is not defined
    pub fn is_defined_in(test_str: &str, defs: &'static LocalKey<RefCell<Definitions>>, set_name: &str) -> Result<bool, Error> {
        return defs.with(|definitions| {
            if let Some(set) = definitions.borrow().get_hashset(set_name) {
                return Ok( set.contains(test_str) );
            }
            if let Some(hashmap) = definitions.borrow().get_hashmap(set_name) {
                return Ok( hashmap.contains_key(test_str) );
            }
            return Err( Error::Other { what: format!("\n  IsInDefinition: '{set_name}' is not defined in definitions.yaml") } );
        });
    }
}

/**
 * Returns true if the text is contained in the set defined in Speech or Braille.
 * element/string -- element (converted to string)/string to test
 * speech or braille
 * set_name -- the set in which the string is to be searched
 */
// 'requiresComma' is useful for checking parenthesized expressions vs function arg lists and other lists
 impl Function for IsInDefinition {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        // FIX: temporarily accept two args as assume SPEECH_DEFINITIONS until the Rule files are fixed
        args.at_least(2)?;
        args.at_most(3)?;
        let set_name = args.pop_string()?;
        // FIX: this (len == 1) is temporary until all the usages are switched to the (new) 3-arg form
        let definitions = if args.len() == 2 {
            match args.pop_string()?.as_str() {
                "Speech" => &SPEECH_DEFINITIONS,
                "Braille" => &BRAILLE_DEFINITIONS,
                _ => return Err( Error::Other { what: "IsInDefinition:: second argument must be either 'Speech' or 'Braille'".to_string() } )
            }
        } else {
            &SPEECH_DEFINITIONS
        };
        match &args[0] {
            Value::String(str) => return match IsInDefinition::is_defined_in(str, definitions, &set_name) {
                Ok(result) => Ok( Value::Boolean( result ) ),
                Err(e) => Err(e),
            },
            Value::Nodeset(nodes) => {
                return if nodes.size() == 0 {
                    Ok( Value::Boolean(false) )    // trivially not in definition
                } else {
                    let node = validate_one_node(nodes.clone(), "IsInDefinition")?;
                    if let Node::Element(e) = node {
                        let text = get_text_from_element(e);
                        if text.is_empty() {
                            Ok( Value::Boolean(false) )
                        } else {
                            match IsInDefinition::is_defined_in(&text, definitions, &set_name) {
                                Ok(result) => Ok( Value::Boolean( result ) ),
                                Err(e) => Err(e),
                            }          
                        }
                    } else {
                        Ok( Value::Boolean(false))       // trivially not in definition                    }
                    }
                }
            },
            _ => Err( Error::Other { what: "IsInDefinition:: neither a node nor a string is passed for first argument".to_string() } ),
        }
    }
}


pub struct DefinitionValue;
impl DefinitionValue {
    /// Returns the value associated with `key` in `set_name`. If `key` is not in `set_name`, an empty string is returned
    /// Returns an error if `set_name` is not defined
    pub fn definition_value(key: &str, defs: &'static LocalKey<RefCell<Definitions>>, set_name: &str) -> Result<String, Error> {
        return defs.with(|definitions| {
            if let Some(map) = definitions.borrow().get_hashmap(set_name) {
                return Ok( match map.get(key) {
                    None => "".to_string(),
                    Some(str) => str.clone(),
                });
            }
            return Err( Error::Other { what: format!("\n  DefinitionValue: '{set_name}' is not defined in definitions.yaml") } );
        });
    }
}

/**
 * Returns true if the node is a bracketed expr with the indicated left/right chars
 * element/string -- element (converted to string)/string to test
 * left -- string (like "[") or empty
 * right -- string (like "]") or empty
 * requires_comma - boolean, optional (check the top level of 'node' for commas
 */
// 'requiresComma' is useful for checking parenthesized expressions vs function arg lists and other lists
 impl Function for DefinitionValue {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(3)?;
        let set_name = args.pop_string()?;
        let definitions = match args.pop_string()?.as_str() {
            "Speech" => &SPEECH_DEFINITIONS,
            "Braille" => &BRAILLE_DEFINITIONS,
            _ => return Err( Error::Other { what: "IsInDefinition:: second argument must be either 'Speech' or 'Braille'".to_string() } )
        };
        match &args[0] {
            Value::String(str) => return match DefinitionValue::definition_value(str, definitions, &set_name) {
                Ok(result) => Ok( Value::String( result ) ),
                Err(e) => Err(e),
            },
            Value::Nodeset(nodes) => {
                return if nodes.size() == 0 {
                    Ok( Value::String("".to_string()) )    // trivially not in definition
                } else {
                    let node = validate_one_node(nodes.clone(), "DefinitionValue")?;
                    if let Node::Element(e) = node {
                        let text = get_text_from_element(e);
                        if text.is_empty() {
                            Ok( Value::String("".to_string()) )
                        } else {
                            match DefinitionValue::definition_value(&text, definitions, &set_name) {
                                Ok(result) => Ok( Value::String( result ) ),
                                Err(e) => Err(e),
                            }          
                        }
                    } else {
                        Ok( Value::String("".to_string()) )       // trivially not in definition                    }
                    }
                }
            },
            _ => Err( Error::Other { what: "DefinitionValue:: neither a node nor a string is passed for first argument".to_string() } ),
        }
    }
}

pub struct DistanceFromLeaf;
impl DistanceFromLeaf {
    fn distance(element: Element, use_left_side: bool, treat_2d_elements_as_tokens: bool) -> usize {
        // FIX: need to handle char level (i.e., chars in a leaf element)
        let mut element = element;
        let mut distance = 1;
        loop {
            // debug!("distance={} -- element: {}", distance, mml_to_string(element));
            if MATHML_LEAF_NODES.contains(as_str!(element.attribute_value(MATHML_FROM_NAME_ATTR).unwrap_or(name(element)))) {
                return distance;
            }
            if treat_2d_elements_as_tokens && (IsNode::is_2D(element) || !IsNode::is_mathml(element)) {
                return distance;
            }
            let children = element.children();
            assert!(!children.is_empty());
            element = as_element( if use_left_side {children[0]} else {children[children.len()-1]} );
            distance += 1;
        }
    }
}

/**
 * Returns distance from the current node to the leftmost/rightmost leaf (if char, then = 0, if token, then 1).
 * If the node is a bracketed expr with the indicated left/right chars
 * node -- node(s) to test
 * left_side -- (bool) traverse leftmost child to leaf
 * treat2D_elements_as_tokens -- (bool) 2D notations such as fractions are treated like leaves 
 */
impl Function for DistanceFromLeaf {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(3)?;
        let treat_2d_elements_as_tokens = args.pop_boolean()?;
        let use_left_side = args.pop_boolean()?;
        let node = validate_one_node(args.pop_nodeset()?, "DistanceFromLeaf")?;
        if let Node::Element(e) = node {
            return Ok( Value::Number( DistanceFromLeaf::distance(e, use_left_side, treat_2d_elements_as_tokens) as f64) );
        }

        // FIX: should having a non-element be an error instead??
        return Err(Error::Other { what: format!("DistanceFromLeaf: first arg '{node:?}' is not a node") });
    }
}



pub struct EdgeNode;
impl EdgeNode {
    // Return the root of the ancestor tree if we are at the left/right side of a path from that to 'element'
    fn edge_node<'a>(element: Element<'a>, use_left_side: bool, stop_node_name: &str) -> Option<Element<'a>> {
        let element_name = element.attribute_value(MATHML_FROM_NAME_ATTR).unwrap_or(name(element));
        if element_name == "math" {
            return Some(element);
        };

        let parent = get_parent(element);   // there is always a "math" node
        let parent_name = parent.attribute_value(MATHML_FROM_NAME_ATTR).unwrap_or(name(parent));

        // first check to see if we have the special case of punctuation as last child of math/mrow element
        // it only matters if we are looking at the right edge

        // debug!("EdgeNode: there are {} preceding siblings",element.preceding_siblings().len() );
        if use_left_side  && !element.preceding_siblings().is_empty() {// not at left side
            return None;
        };

        if !use_left_side && !element.following_siblings().is_empty() {  // not at right side
            // check for the special case that the parent is an mrow and the grandparent is <math> and we have punctuation
            let grandparent = get_parent(parent);
            let grandparent_name = grandparent.attribute_value(MATHML_FROM_NAME_ATTR).unwrap_or(name(grandparent));
            if grandparent_name == "math" &&
               parent_name == "mrow" && parent.children().len() == 2 {      // right kind of mrow
                let text = get_text_from_element( as_element(parent.children()[1]) );
                if text == "," || text == "." || text == ";" || text == "?" {
                    return Some(grandparent);
                }
            }
             return None;
        };

        // at an edge -- check to see the parent is desired root
        if parent_name == stop_node_name || 
           (stop_node_name == "2D" && IsNode::is_2D(parent)) {
            return Some(parent);
        };
        
        // debug!("EdgeNode: recurse to {}", parent_name);
        return EdgeNode::edge_node(parent, use_left_side, stop_node_name)
    }
}

// EdgeNode(node, "left"/"right", stopNodeName)
// 		-- returns the stopNode if at left/right edge of named ancestor node. "stopNodeName' can also be "2D'
// 		   returns original node match isn't found
//  Note: if stopNodeName=="math", then punctuation is taken into account since it isn't really part of the math
impl Function for EdgeNode {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(3)?;
        let stop_node_name = args.pop_string()?;
        let use_left_side = args.pop_string()?.to_lowercase() == "left";
        let node = validate_one_node(args.pop_nodeset()?, "EdgeNode")?;
        if let Node::Element(e) = node {
            let result = match EdgeNode::edge_node(e, use_left_side, &stop_node_name) {
                Some(found) => found,
                None => e,
            };
            let mut node_set = Nodeset::new();
            node_set.add(result);
            return Ok( Value::Nodeset(node_set) );
        }

        // FIX: should having a non-element be an error instead??
        return Err(Error::Other { what: format!("EdgeNode: first arg '{node:?}' is not a node") });
    }
}

pub struct SpeakIntentName;
/// SpeakIntentName(intent, verbosity, fixity)
/// Real-life example:
///   If a MathML element has intent="factorial($x)", in the rules you might see:
///      - x: "SpeakIntentName(name(.), $Verbosity, 'postfix')"
///   For English, MathCAT will resolve:
///      SpeakIntentName("factorial", "Verbose", "postfix") → "factorial"
///   This is used to generate speech like "x factorial" for expressions like "x!".
///   Returns a string corresponding to the intent name with the indicated verbosity
impl Function for SpeakIntentName {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(3)?;
        let fixity = args.pop_string()?;
        let verbosity = args.pop_string()?;
        let intent_name = args.pop_string()?;
        return Ok( Value::String(crate::infer_intent::intent_speech_for_name(&intent_name, &verbosity, &fixity)) );
    }
}

pub struct IntentFunctionUseArityPath;
/// IntentFunctionUseArityPath(name, fixity, argCount)
///   True when IntentMappings `|` arity templates match this argument count.
///   For example, in en/definitions.yaml for intent "choose":
///     "function=choose|from|to"
///   means for three arguments, templates like "choose n from k to l" apply.
///   This returns true if the `choose` function is used with three arguments.
impl Function for IntentFunctionUseArityPath {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(3)?;
        let arg_count = args.pop_number()? as usize;
        let fixity = args.pop_string()?;
        let intent_name = args.pop_string()?;
        return Ok( Value::Boolean(
            crate::infer_intent::intent_function_use_arity_path(&intent_name, &fixity, arg_count)
        ) );
    }
}

pub struct IntentFunctionGlueBefore;
/// IntentFunctionGlueBefore(name, fixity, argIndex, argCount)
///   Glue spoken immediately before the given 1-based argument when using an arity template.
impl Function for IntentFunctionGlueBefore {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(4)?;
        let arg_count = args.pop_number()? as usize;
        let arg_index = args.pop_number()? as usize;
        let fixity = args.pop_string()?;
        let intent_name = args.pop_string()?;
        return Ok( Value::String(
            crate::infer_intent::intent_function_glue_before(&intent_name, &fixity, arg_index, arg_count)
        ) );
    }
}

pub struct IntentFunctionArgSeparator;
/// IntentFunctionArgSeparator(name, fixity, argCount)
///   Glue spoken immediately before the given 1-based argument when using an arity template.
///   For example, in en/definitions.yaml, for the intent name "choose", the mapping is:
///   "function=choose|from|to".
///   When called as IntentFunctionGlueBefore("choose", "function", 2, 3), this returns "from",
///   which is spoken before the second argument when reading an expression like "choose n from k".
///   Likewise, for arg_index = 3, it would return "to".
impl Function for IntentFunctionArgSeparator {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(3)?;
        let arg_count = args.pop_number()? as usize;
        let fixity = args.pop_string()?;
        let intent_name = args.pop_string()?;
        return Ok( Value::String(
            crate::infer_intent::intent_function_arg_separator(&intent_name, &fixity, arg_count)
        ) );
    }
}

pub struct GetBracketingIntentName;
/// GetBracketingIntentName(name, verbosity, at_start_or_end)
///   Returns a potentially empty string to use to bracket an intent expression (start foo... end foo)
/// 
impl GetBracketingIntentName {
    fn bracketing_words(intent_name: &str, verbosity: &str, fixity: &str, at_start: bool) -> String {
        crate::definitions::SPEECH_DEFINITIONS.with(|definitions| {
            let definitions = definitions.borrow();
            if let Some(intent_name_pattern) = definitions.get_hashmap("IntentMappings").unwrap().get(intent_name) {
                // Split the pattern is: fixity-def [|| fixity-def]*
                //   fixity-def := fixity=open; verbosity; close
                //   verbosity := terse | medium | verbose
                if let Some(matched_intent) = intent_name_pattern.split("||").find(|&entry| entry.trim().starts_with(fixity)) {
                    let (_, matched_intent) = matched_intent.split_once("=").unwrap_or_default();
                    let name_part = crate::infer_intent::intent_mapping_name_part(matched_intent);
                    let parts = name_part.split(";").collect::<Vec<&str>>();
                    if parts.len() == 1 {
                        return "".to_string();
                    }
                    if parts.len() != 3 {
                        error!("Intent '{}' has {} ';' separated parts, should have 3", intent_name, parts.len());
                        return "".to_string();
                    }
                    let mut speech = (if at_start {parts[0]} else {parts[2]}).split(":").collect::<Vec<&str>>();
                    match speech.len() {
                        1 => return speech[0].to_string(),
                        2 | 3 => {
                            if speech.len() == 2 {
                                warn!("Intent '{intent_name}'  has only two ':' separated parts, but should have three");
                                speech.push(speech[1]);
                            }
                            let bracketing_words = match verbosity {
                                "Terse" => speech[0],
                                "Medium" => speech[1],
                                _ => speech[2],
                            };
                            return bracketing_words.to_string();
                        },
                        _ => {
                            error!("Intent '{}' has too many ({}) operator names, should only have 2", intent_name, speech.len());
                        },
                    }
                }   
            };
            return "".to_string();
        })
    }
}

impl Function for GetBracketingIntentName {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(4)?;
        let start_or_end = args.pop_string()?;
        if start_or_end != "start" && start_or_end != "end" {
            return Err( Error::Other { what: "GetBracketingIntentName: first argument must be either 'start' or 'end'".to_string() } );
        }
        let fixity = args.pop_string()?;
        let verbosity = args.pop_string()?;
        let name = args.pop_string()?;
        return Ok( Value::String(GetBracketingIntentName:: bracketing_words(&name, &verbosity, &fixity, start_or_end == "start")) );
    }
}

pub struct GetNavigationPartName;
/// GetNavigationPartName(name, index)
/// Returns the name to use to speak the part of a navigation expression (e.g., 'numerator', 'denominator', 'base', 'exponent', ...).
/// If there is no match, an empty string is returned.
/// 'index' is 0-based
/// 
impl GetNavigationPartName {
    fn navigation_part_name(intent_name: &str, index: usize) -> String {
        crate::definitions::SPEECH_DEFINITIONS.with(|definitions| {
            let definitions = definitions.borrow();
            if let Some(navigation_names) = definitions.get_hashmap("NavigationParts") &&
               let Some(nav_part_names) = navigation_names.get(intent_name) {
                    // Split the pattern is: part [; part]*
                    if let Some(part_name) = nav_part_names.trim().split(";").nth(index) {
                        return part_name.trim().to_string();
                    }
                }
            return "".to_string();
        })
    }
}

impl Function for GetNavigationPartName {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(2)?;
        let index = args.pop_number()? as usize;
        let name = args.pop_string()?;
        return Ok( Value::String(GetNavigationPartName:: navigation_part_name(&name, index)) );
    }
}

pub struct FontSizeGuess;
/// FontSizeGuess(size_string)
///   returns a guess of the size in "ems"
/// Examples:
///    "0.278em" -> 0.278
///    ""
// 		   returns original node match isn't found
impl FontSizeGuess {
    /// Returns an estimated width in em units, using an assumed 12-point font for conversions.
    pub fn em_from_value(value_with_unit: &str) -> f64 {
        // match one or more digits followed by a unit -- there are many more units, but they tend to be large and rarer(?)
        static FONT_VALUE: LazyLock<Regex> = LazyLock::new(|| { Regex::new(r"(-?[0-9]*\.?[0-9]*)(px|cm|mm|Q|in|pc|pt|ex|em|rem)").unwrap() });
        let cap = FONT_VALUE.captures(value_with_unit);
        if let Some(cap) = cap {
            if cap.len() == 3 {
                let multiplier = match &cap[2] {    // guess based on 12pt font to convert to ems
                    "px" => 1.0/12.0,
                    "cm" => 2.37,
                    "mm" => 0.237,
                    "Q" => 0.059,  // 1/4 mm
                    "in" => 6.02,
                    "pc" => 1.0,
                    "pt" => 1.0/12.0,
                    "ex" => 0.5,
                    "em" => 1.0,
                    "rem" => 16.0/12.0,
                    default => {debug!("unit='{default}'"); 10.0}
                };
                // debug!("FontSizeGuess: {}->{}, val={}, multiplier={}", value_with_unit, value*multiplier, value, multiplier);
                return cap[1].parse::<f64>().unwrap_or(0.0) * multiplier;
            }  else {
                return 0.0;             // something bad happened
            }
        }else {
            let multiplier = match value_with_unit {    // guess based on 12pt font to convert to ems
                "veryverythinspace" => 1.0/18.0,
                "verythinspace" => 2.0/18.0,
                "thinspace" => 3.0/18.0,
                "mediumspace" => 4.0/18.0,
                "thickspace" => 5.0/18.0,
                "verythickspace" => 6.0/18.0,
                "veryverythickspace" => 7.0/18.0,
                _ => 0.0,
            };
            return multiplier;
        }
    }
}
impl Function for FontSizeGuess {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(1)?;
        let value_with_unit = args.pop_string()?;
        let em_value = FontSizeGuess::em_from_value(&value_with_unit);
        return Ok( Value::Number(em_value) );
    }
}

pub struct ReplaceAll;
/// ReplaceAll(haystack, needle, replacement)
///   Returns a string with all occurrences of 'needle' replaced with 'replacement'
impl Function for ReplaceAll {
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> Result<Value<'d>, Error>
    {
        let mut args = Args(args);
        args.exactly(3)?;
        let replacement = args.pop_string()?;
        let needle = args.pop_string()?;
        let haystack = args.pop_string()?;
        return Ok( Value::String(haystack.replace(&needle, &replacement)) );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CTDRowType {
    Normal,
    Labeled,
    Implicit,
}

/// A single-use structure for computing the proper dimensions of an
/// `mtable`.
struct CountTableDims {
    num_rows: usize,
    num_cols: usize,
    /// map from number of remaining in extra row-span to number of
    /// columns with that value.
    extended_cells: HashMap<usize, usize>,
    /// rowspan=0 cells extend for the rest of the table, however
    /// long that may be as determined by all other finite cells.
    permanent_cols: usize,
}

impl CountTableDims {

    fn new() -> CountTableDims {
        Self { num_rows: 0, num_cols: 0, extended_cells: HashMap::new(), permanent_cols: 0 }
    }

    /// Returns the number of columns the cell contributes to the
    /// current row. Also updates `extended_cells` as appropriate.
    fn process_cell_in_row<'d>(&mut self, mtd: Element<'d>, is_first: bool, row_type: CTDRowType) -> usize {
        // Rows can only contain `mtd`s.  If this is not an `mtd`, we will just skip it.
        if name(mtd) != "mtd" {
            return 0;
        }

        // Add the contributing columns, taking colspan into account. Don't contribute if
        // this is the first element of a labeled row.
        let colspan = mtd.attribute_value("colspan")
            .or_else(|| mtd.attribute_value("columnspan"))
            .map_or(1, |e| e.parse::<usize>().unwrap_or(1));
        if row_type == CTDRowType::Labeled && is_first {
            // This is a label for the row and does not contibute to
            // the size of the table. NOTE: Can this label have a
            // non-trivial rowspan? If so, can it otherwise extend the
            // size of the table?
            return 0;
        }

        let rowspan = mtd.attribute_value("rowspan").map_or(1, |e| {
            e.parse::<usize>().unwrap_or(1)
        });

        if rowspan > 1 {
            *self.extended_cells.entry(rowspan).or_default() += colspan;
        } else if rowspan == 0 {
            self.permanent_cols += colspan;
        }

        colspan
    }

    /// Update the number of rows, and update the extended cells.
    /// Returns the total number of columns accross all extended
    /// cells.
    fn next_row(&mut self) -> usize {
        self.num_rows += 1;
        let mut ext_cols = 0;
        self.extended_cells = self.extended_cells.iter().filter(|&(k, _)| *k > 1).map(|(k, v)| {
            ext_cols += *v;
            (k-1, *v)
        }).collect();
        ext_cols
    }

    /// For an `mtable` element, count the number of rows and columns in the table.
    ///
    /// This function is relatively permissive. Non-`mtr` rows are
    /// ignored. The number of columns is determined only from the first
    /// row, if it exists. Within that row, non-`mtd` elements are ignored. 
    fn count_table_dims<'d>(mut self, e: Element<'_>) -> Result<(Value<'d>, Value<'d>), Error> {
        for child in e.children() {
            let ChildOfElement::Element(row) = child else {
                continue
            };

            // Each child of mtable should be an mtr or mlabeledtr. According to the spec, though,
            // bare `mtd`s should also be treated as having an implicit wrapping `<mtr>`.
            // Other elements should be ignored.
            let row_name = as_str!(name(row));

            let row_type = match row_name {
		"mlabeledtr" => CTDRowType::Labeled,
		"mtr" => CTDRowType::Normal,
		"mtd" => CTDRowType::Implicit,
		_ => continue
            };

            let ext_cols = self.next_row();

            let mut num_cols_in_row = 0;
            match row_type {
                CTDRowType::Normal | CTDRowType::Labeled => {
                    let mut first_elem = true;
                    for row_child in row.children() {
                        let ChildOfElement::Element(mtd) = row_child else  {
                            continue;
                        };

                        num_cols_in_row += self.process_cell_in_row(mtd, first_elem, row_type);
                        first_elem= false;
                    }
                }
                CTDRowType::Implicit => {
                    num_cols_in_row += self.process_cell_in_row(row, true, row_type)
                }
            }
            // update the number of columns based on this row.
            self.num_cols = self.num_cols.max(num_cols_in_row + ext_cols + self.permanent_cols);
        }

        // At this point, the number of columns is correct. If we have
        // any leftover rows from rowspan extended cells, we need to
        // account for them here.
        //
        // NOTE: It does not appear that renderers respect these extra
        // columns, so we will not use them.
        let _extra_rows = self.extended_cells.keys().max().map(|k| k-1).unwrap_or(0);

        Ok((Value::Number(self.num_rows  as f64), Value::Number(self.num_cols as f64)))
    }

    fn evaluate<'d>(self, fn_name: &str,
                        args: Vec<Value<'d>>) -> Result<(Value<'d>, Value<'d>), Error> {
        let mut args = Args(args);
        args.exactly(1)?;
        let element = args.pop_nodeset()?;
        let node = validate_one_node(element, fn_name)?;
        if let Node::Element(e) = node {
            if is_tag(e, "mtable") {
                return self.count_table_dims(e);
            } else {
                return Err(Error::Other { what: format!("Input element was a <{}>, not an <mtable>",
                                                as_qname!(e.name()).local_part()) });
            }
        }

        Err( Error::Other { what: "Could not count dimensions of non-Element.".to_string() } )
    }
}

struct CountTableRows;
impl Function for CountTableRows {
    fn evaluate<'c, 'd>(&self,
                        _context: &context::Evaluation<'c, 'd>,
                        args: Vec<Value<'d>>) -> Result<Value<'d>, Error> {
        CountTableDims::new().evaluate("CountTableRows", args).map(|a| a.0)
    }
}

struct CountTableColumns;
impl Function for CountTableColumns {
    fn evaluate<'c, 'd>(&self,
                        _context: &context::Evaluation<'c, 'd>,
                        args: Vec<Value<'d>>) -> Result<Value<'d>, Error> {
        CountTableDims::new().evaluate("CountTableColumns", args).map(|a| a.1)
    }
}

#[derive(Clone, Copy)]
enum TableLineAxis {
    Row,
    Column,
}

/// Return whether a one-based mtable boundary has a visible line on the given axis.
///
/// MathML repeats the final line style for remaining boundaries. Boundaries after
/// the final row or column, and values other than `solid` and `dashed`, do not
/// describe a visible separator.
fn has_visible_table_line(table: Element, boundary: usize, axis: TableLineAxis) -> bool {
    if boundary == 0 || !is_tag(table, "mtable") {
        return false;
    }

    let Ok((Value::Number(row_count), Value::Number(column_count))) = CountTableDims::new().count_table_dims(table) else {
        return false;
    };
    let (line_count, attribute_name) = match axis {
        TableLineAxis::Row => (row_count, "rowlines"),
        TableLineAxis::Column => (column_count, "columnlines"),
    };
    if boundary as f64 >= line_count {
        return false;
    }

    return table
        .attribute_value(attribute_name)
        .map(|values| {
            matches!(
                values.split_whitespace().take(boundary).last(),
                Some("solid" | "dashed")
            )
        })
        .unwrap_or(false);
}

/// Validate and convert XPath arguments before delegating to the typed table-line helper.
fn evaluate_has_visible_table_line<'d>(
    args: Vec<Value<'d>>,
    function_name: &str,
    axis: TableLineAxis,
) -> Result<Value<'d>, Error> {
    let mut args = Args(args);
    args.exactly(2)?;
    let boundary = args.pop_number()?;
    let table = validate_one_node(args.pop_nodeset()?, function_name)?;
    let Node::Element(table) = table else {
        return Err(Error::Other { what: format!("{function_name} requires an mtable element") });
    };
    if !boundary.is_finite() || boundary < 1.0 || boundary.fract() != 0.0 {
        return Ok(Value::Boolean(false));
    }
    return Ok(Value::Boolean(has_visible_table_line(table, boundary as usize, axis)));
}

/// XPath function reporting whether an mtable column boundary has a visible line.
struct HasVisibleColumnLine;
impl Function for HasVisibleColumnLine {
    fn evaluate<'c, 'd>(&self,
                        _context: &context::Evaluation<'c, 'd>,
                        args: Vec<Value<'d>>) -> Result<Value<'d>, Error> {
        evaluate_has_visible_table_line(args, "HasVisibleColumnLine", TableLineAxis::Column)
    }
}

/// XPath function reporting whether an mtable row boundary has a visible line.
struct HasVisibleRowLine;
impl Function for HasVisibleRowLine {
    fn evaluate<'c, 'd>(&self,
                        _context: &context::Evaluation<'c, 'd>,
                        args: Vec<Value<'d>>) -> Result<Value<'d>, Error> {
        evaluate_has_visible_table_line(args, "HasVisibleRowLine", TableLineAxis::Row)
    }
}


/// Add all the functions defined in this module to `context`.
pub fn register_mathcat_xpath_functions(context: &mut Context) {
    context.set_function("NestingChars", crate::braille::NemethNestingChars);
    context.set_function("BrailleChars", crate::braille::BrailleChars);
    context.set_function("NeedsToBeGrouped", crate::braille::NeedsToBeGrouped);
    context.set_function("IsNode", IsNode);
    context.set_function("ToOrdinal", ToOrdinal);
    context.set_function("ToCommonFraction", ToCommonFraction);
    context.set_function("IsBracketed", IsBracketed);
    context.set_function("IsInDefinition", IsInDefinition);
    context.set_function("DefinitionValue", DefinitionValue);
    context.set_function("BaseNode", BaseNode);
    context.set_function("IfThenElse", IfThenElse);
    context.set_function("IFTHENELSE", IfThenElse);
    context.set_function("DistanceFromLeaf", DistanceFromLeaf);
    context.set_function("EdgeNode", EdgeNode);
    context.set_function("SpeakIntentName", SpeakIntentName);
    context.set_function("IntentFunctionUseArityPath", IntentFunctionUseArityPath);
    context.set_function("IntentFunctionGlueBefore", IntentFunctionGlueBefore);
    context.set_function("IntentFunctionArgSeparator", IntentFunctionArgSeparator);
    context.set_function("GetBracketingIntentName", GetBracketingIntentName);
    context.set_function("GetNavigationPartName", GetNavigationPartName);
    context.set_function("CountTableRows", CountTableRows);
    context.set_function("CountTableColumns", CountTableColumns);
    context.set_function("HasVisibleColumnLine", HasVisibleColumnLine);
    context.set_function("HasVisibleRowLine", HasVisibleRowLine);
    context.set_function("DEBUG", Debug);

    // Not used: remove??
    context.set_function("min", Min);       // missing in xpath 1.0
    context.set_function("max", Max);       // missing in xpath 1.0
    context.set_function("FontSizeGuess", FontSizeGuess);
    context.set_function("ReplaceAll", ReplaceAll);
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::Result;
    use crate::interface::{get_element, init_panic_handler, report_any_panic, trim_element};
    use sxd_document_no_unsafe::parser;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    fn xpath_test<F>(f: F) -> Result<()>
    where
        F: FnOnce() -> Result<()> + std::panic::UnwindSafe,
    {
        init_panic_handler();
        let result = catch_unwind(AssertUnwindSafe(f));
        return report_any_panic(result);
    }

    fn init_word_list() -> Result<()> {
        crate::interface::set_rules_dir(super::super::abs_rules_dir_path())?;
        crate::interface::set_preference("Language", "en")?;
        crate::definitions::read_definitions_file(true)?;
        return Ok( () );
    }

    // Pica widths must use the existing 12pt font estimate: 1pc, 12pt, and 1em are equivalent.
    #[test]
    fn font_size_guess_pica_widths() {
        assert_eq!(FontSizeGuess::em_from_value("1pc"), 1.0);
        assert_eq!(FontSizeGuess::em_from_value("0.5pc"), 0.5);
        assert_eq!(FontSizeGuess::em_from_value("-1pc"), -1.0);
        assert_eq!(FontSizeGuess::em_from_value("12pt"), 1.0);
        assert_eq!(FontSizeGuess::em_from_value("1em"), 1.0);
    }

    #[test]
    fn ordinal_one_digit() -> Result<()> {
        return xpath_test(|| {
        init_word_list()?;
        assert_eq!("zeroth", ToOrdinal::convert("0", false, false).unwrap());
        assert_eq!("second", ToOrdinal::convert("2", false, false).unwrap());
        assert_eq!("ninth", ToOrdinal::convert("9", false, false).unwrap());

        assert_eq!("zeroth", ToOrdinal::convert("0", false, true).unwrap());
        assert_eq!("seconds", ToOrdinal::convert("2", false, true).unwrap());
        assert_eq!("ninths", ToOrdinal::convert("9", false, true).unwrap());

        assert_eq!("first", ToOrdinal::convert("1", true, false).unwrap());
        assert_eq!("half", ToOrdinal::convert("2", true, false).unwrap());
        assert_eq!("half", ToOrdinal::convert("02", true, false).unwrap());
        assert_eq!("ninth", ToOrdinal::convert("9", true, false).unwrap());

        assert_eq!("halves", ToOrdinal::convert("2", true, true).unwrap());
        assert_eq!("halves", ToOrdinal::convert("002", true, true).unwrap());
        assert_eq!("ninths", ToOrdinal::convert("9", true, true).unwrap());
        return Ok( () );
        });
    }

    #[test]
    fn ordinal_two_digit() -> Result<()> {
        return xpath_test(|| {
        init_word_list()?;
        assert_eq!("tenth", ToOrdinal::convert("10", false, false).unwrap());
        assert_eq!("seventeenth", ToOrdinal::convert("17", false, false).unwrap());
        assert_eq!("thirty second", ToOrdinal::convert("32", false, false).unwrap());
        assert_eq!("fortieth", ToOrdinal::convert("40", false, false).unwrap());

        assert_eq!("tenths", ToOrdinal::convert("10", false, true).unwrap());
        assert_eq!("sixteenths", ToOrdinal::convert("16", false, true).unwrap());
        assert_eq!("eighty eighths", ToOrdinal::convert("88", false, true).unwrap());
        assert_eq!("fiftieths", ToOrdinal::convert("50", false, true).unwrap());

        assert_eq!("eleventh", ToOrdinal::convert("11", true, false).unwrap());
        assert_eq!("forty fourth", ToOrdinal::convert("44", true, false).unwrap());
        assert_eq!("ninth", ToOrdinal::convert("9", true, false).unwrap());
        assert_eq!("ninth", ToOrdinal::convert("00000009", true, false).unwrap());
        assert_eq!("sixtieth", ToOrdinal::convert("60", true, false).unwrap());

        assert_eq!("tenths", ToOrdinal::convert("10", true, true).unwrap());
        assert_eq!("tenths", ToOrdinal::convert("0010", true, true).unwrap());
        assert_eq!("elevenths", ToOrdinal::convert("11", true, true).unwrap());
        assert_eq!("nineteenths", ToOrdinal::convert("19", true, true).unwrap());
        assert_eq!("twentieths", ToOrdinal::convert("20", true, true).unwrap());
        assert_eq!("nineteenths", ToOrdinal::convert("𝟏𝟗", true, true).unwrap());
        return Ok( () );
        });
    }

    #[test]
    fn ordinal_three_digit() -> Result<()> {
        return xpath_test(|| {
        init_word_list()?;
        assert_eq!("one hundred first", ToOrdinal::convert("101", false, false).unwrap());
        assert_eq!("two hundred tenth", ToOrdinal::convert("210", false, false).unwrap());
        assert_eq!("four hundred thirty second", ToOrdinal::convert("432", false, false).unwrap());
        assert_eq!("four hundred second", ToOrdinal::convert("402", false, false).unwrap());

        assert_eq!("one hundred first", ToOrdinal::convert("101", true, false).unwrap());
        assert_eq!("two hundred second", ToOrdinal::convert("202", true, false).unwrap());
        assert_eq!("four hundred thirty second", ToOrdinal::convert("432", true, false).unwrap());
        assert_eq!("five hundred third", ToOrdinal::convert("503", true, false).unwrap());

        assert_eq!("three hundred elevenths", ToOrdinal::convert("311", false, true).unwrap());
        assert_eq!("four hundred ninety ninths", ToOrdinal::convert("499", false, true).unwrap());
        assert_eq!("nine hundred ninetieths", ToOrdinal::convert("990", false, true).unwrap());
        assert_eq!("six hundred seconds", ToOrdinal::convert("602", false, true).unwrap());

        assert_eq!("seven hundredths", ToOrdinal::convert("700", true, true).unwrap());
        assert_eq!("one hundredths", ToOrdinal::convert("100", true, true).unwrap());
        assert_eq!("eight hundred seventeenths", ToOrdinal::convert("817", true, true).unwrap());
        return Ok( () );
        });
    }
    #[test]
    fn ordinal_large() -> Result<()> {
        return xpath_test(|| {
        init_word_list()?;
        assert_eq!("one thousandth", ToOrdinal::convert("1000", false, false).unwrap());
        assert_eq!("two thousand one hundredth", ToOrdinal::convert("2100", false, false).unwrap());
        assert_eq!("thirty thousandth", ToOrdinal::convert("30000", false, false).unwrap());
        assert_eq!("four hundred thousandth", ToOrdinal::convert("400000", false, false).unwrap());

        assert_eq!("four hundred thousandth", ToOrdinal::convert("400000", true, false).unwrap());
        assert_eq!("five hundred thousand second", ToOrdinal::convert("500002", true, false).unwrap());
        assert_eq!("six millionth", ToOrdinal::convert("6000000", true, false).unwrap());
        assert_eq!("sixty millionth", ToOrdinal::convert("60000000", true, false).unwrap());

        assert_eq!("seven billionths", ToOrdinal::convert("7000000000", false, true).unwrap());
        assert_eq!("eight trillionths", ToOrdinal::convert("8000000000000", false, true).unwrap());
        assert_eq!("nine quadrillionths", ToOrdinal::convert("9000000000000000", false, true).unwrap());
        assert_eq!("one quintillionth", ToOrdinal::convert("1000000000000000000", false, false).unwrap());

        assert_eq!("nine billion eight hundred seventy six million five hundred forty three thousand two hundred tenths", ToOrdinal::convert("9876543210", true, true).unwrap());
        assert_eq!("nine billion five hundred forty three thousand two hundred tenths", ToOrdinal::convert("9000543210", true, true).unwrap());
        assert_eq!("zeroth", ToOrdinal::convert("00000", false, false).unwrap());
        return Ok( () );
        });
    }


    fn test_is_simple(message: &'static str, mathml_str: &'static str) -> Result<()> {
		// this forces initialization
		crate::speech::SPEECH_RULES.with(|_| true);
        let package = parser::parse(mathml_str)
        .map_err(|e| anyhow::anyhow!("failed to parse XML: {e}"))?;
        let mathml = get_element(&package);
        trim_element(mathml, false);
        assert!(IsNode::is_simple(mathml), "{}", message);
        return Ok( () );
    }

    fn test_is_not_simple(message: &'static str, mathml_str: &'static str) -> Result<()> {
		// this forces initialization
		crate::speech::SPEECH_RULES.with(|_| true);
        let package = parser::parse(mathml_str)
        .map_err(|e| anyhow::anyhow!("failed to parse XML: {e}"))?;
        let mathml = get_element(&package);
        trim_element(mathml, false);
        assert!(!IsNode::is_simple(mathml), "{}", message);
        return Ok( () );
    }
    #[test]
    fn is_simple() -> Result<()> {
        return xpath_test(|| {
        test_is_simple("single variable", "<mi>x</mi>")?;
        test_is_simple("single number", "<mn>1.2</mn>")?;
        test_is_simple("negative number", "<mrow><mo>-</mo><mn>10</mn></mrow>")?;
        test_is_simple("negative variable", "<mrow><mo>-</mo><mi>x</mi></mrow>")?;
        test_is_simple("ordinal fraction", "<mfrac><mn>3</mn><mn>4</mn></mfrac>")?;
        test_is_simple("x y", "<mrow><mi>x</mi><mo>&#x2062;</mo><mi>y</mi></mrow>")?;
        test_is_simple("negative two vars", 
                "<mrow><mrow><mo>-</mo><mi>x</mi></mrow><mo>&#x2062;</mo><mi>y</mi></mrow>")?;
        test_is_simple("-2 x y", 
                "<mrow><mrow><mo>-</mo><mn>2</mn></mrow>
                             <mo>&#x2062;</mo><mi>x</mi><mo>&#x2062;</mo><mi>z</mi></mrow>")?;
        test_is_simple("sin x", "<mrow><mi>sin</mi><mo>&#x2061;</mo><mi>x</mi></mrow>")?;
        test_is_simple("f(x)", "<mrow><mi>f</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow>")?;
        test_is_simple("f(x+y)",
         "<mrow><mi>f</mi><mo>&#x2061;</mo>\
            <mrow><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></mrow></mrow>")?;
        return Ok( () );
        });
    }

    #[test]
    fn is_not_simple() -> Result<()> {
        return xpath_test(|| {
        test_is_not_simple("multi-char variable", "<mi>rise</mi>")?;
        test_is_not_simple("large ordinal fraction", "<mfrac><mn>30</mn><mn>4</mn></mfrac>")?;
        test_is_not_simple("fraction with var in numerator", "<mfrac><mi>x</mi><mn>4</mn></mfrac>")?;
        test_is_not_simple("square root", "<msqrt><mi>x</mi></msqrt>")?;
        test_is_not_simple("subscript", "<msub><mi>x</mi><mn>4</mn></msub>")?;
        test_is_not_simple("-x y z", 
                "<mrow><mrow><mo>-</mo><mi>x</mi></mrow>
                            <mo>&#x2062;</mo><mi>y</mi><mo>&#x2062;</mo><mi>z</mi></mrow>")?;
        test_is_not_simple("C(-2,1,4)",             // github.com/daisy/MathCAT/issues/199
                    "<mrow><mi>C</mi><mrow><mo>(</mo><mo>−</mo><mn>2</mn><mo>,</mo><mn>1</mn><mo>,</mo><mn>4</mn><mo>)</mo></mrow></mrow>")?;
        return Ok( () );
        });
    }

    fn check_table_dims(mathml: &str, dims: (usize, usize)) -> Result<()> {
        let package = parser::parse(mathml).map_err(|e| anyhow::anyhow!("failed to parse XML: {e}"))?;
        let math_elem = get_element(&package);
        let child = as_element(math_elem.children()[0]);
        assert_eq!(
            CountTableDims::new().count_table_dims(child),
            Ok((Value::Number(dims.0 as f64), Value::Number(dims.1 as f64)))
        );
        return Ok( () );
    }

    #[test]
    fn table_dim() -> Result<()> {
        return xpath_test(|| {
        check_table_dims("<math><mtable><mtr><mtd>a</mtd></mtr></mtable></math>", (1, 1))?;
        check_table_dims("<math><mtable><mtr><mtd colspan=\"3\">a</mtd><mtd>b</mtd></mtr><mtr><mtd></mtd></mtr></mtable></math>", (2, 4))?;

        check_table_dims("<math><mtable><mlabeledtr><mtd>label</mtd><mtd>a</mtd><mtd>b</mtd></mlabeledtr><mtr><mtd>c</mtd><mtd>d</mtd></mtr></mtable></math>", (2, 2))?;
        // extended rows beyond the `mtr`s do *not* count towards the row count.
        check_table_dims("<math><mtable><mtr><mtd rowspan=\"3\">a</mtd></mtr></mtable></math>", (1, 1))?;

        check_table_dims("<math><mtable><mtr><mtd rowspan=\"3\">a</mtd></mtr>
<mtr><mtd columnspan=\"2\">b</mtd></mtr></mtable></math>", (2, 3))?;
        return Ok( () );
        });
    }

    fn check_table_line(mathml: &str, boundary: usize, axis: TableLineAxis, expected: bool) -> Result<()> {
        let package = parser::parse(mathml).map_err(|e| anyhow::anyhow!("failed to parse XML: {e}"))?;
        let math = get_element(&package);
        let table = math
            .children()
            .iter()
            .find_map(|child| match child {
                ChildOfElement::Element(table) => Some(*table),
                _ => None,
            })
            .expect("test MathML should contain an mtable element");
        assert_eq!(has_visible_table_line(table, boundary, axis), expected);
        return Ok(());
    }

    /// Verifies visible table-line styles, repeated styles, and boundaries outside the table.
    #[test]
    fn visible_table_lines() -> Result<()> {
        return xpath_test(|| {
            // The three values map in order to the three boundaries between four columns.
            let mixed_column_lines: &str = "<math>
            <mtable columnlines=' none  solid dashed '>
                <mtr>
                    <mtd>column 1</mtd>
                    <mtd>column 2</mtd>
                    <mtd>column 3</mtd>
                    <mtd>column 4</mtd>
                </mtr>
            </mtable></math>";
            check_table_line(mixed_column_lines, 1, TableLineAxis::Column, false)?;
            check_table_line(mixed_column_lines, 2, TableLineAxis::Column, true)?;
            check_table_line(mixed_column_lines, 3, TableLineAxis::Column, true)?;

            // The final `dashed` value repeats for the third column boundary.
            let repeated_column_line: &str = "<math>
            <mtable columnlines='solid dashed'>
                <mtr>
                    <mtd>column 1</mtd>
                    <mtd>column 2</mtd>
                    <mtd>column 3</mtd>
                    <mtd>column 4</mtd>
                </mtr>
            </mtable></math>";
            check_table_line(repeated_column_line, 3, TableLineAxis::Column, true)?;

            // A table without `columnlines` has no visible column boundary.
            let no_column_lines: &str = "<math>
            <mtable>
                <mtr>
                    <mtd>column 1</mtd>
                    <mtd>column 2</mtd>
                </mtr>
            </mtable></math>";
            check_table_line(no_column_lines, 1, TableLineAxis::Column, false)?;

            // `none` explicitly makes the column boundary invisible.
            let invisible_column_line: &str = "<math>
            <mtable columnlines='none'>
                <mtr>
                    <mtd>column 1</mtd>
                    <mtd>column 2</mtd>
                </mtr>
            </mtable></math>";
            check_table_line(invisible_column_line, 1, TableLineAxis::Column, false)?;

            // Only `solid` and `dashed` describe visible table lines.
            let unsupported_column_line: &str = "<math>
            <mtable columnlines='double'>
                <mtr>
                    <mtd>column 1</mtd>
                    <mtd>column 2</mtd>
                </mtr>
            </mtable></math>";
            check_table_line(unsupported_column_line, 1, TableLineAxis::Column, false)?;

            // Boundary 2 is after the final column, not between two columns.
            let two_column_table: &str = "<math><mtable columnlines='solid'>
                <mtr>
                    <mtd>column 1</mtd>
                    <mtd>column 2</mtd>
                </mtr>
            </mtable></math>";
            check_table_line(two_column_table, 2, TableLineAxis::Column, false)?;

            // Four rows have three interior boundaries. `none` applies after row 1,
            // `dashed` applies after row 2, and the final `dashed` repeats after row 3.
            let mixed_row_lines: &str = "<math>
            <mtable rowlines='none dashed'>
                <mtr><mtd>row 1</mtd></mtr>
                <mtr><mtd>row 2</mtd></mtr>
                <mtr><mtd>row 3</mtd></mtr>
                <mtr><mtd>row 4</mtd></mtr>
            </mtable></math>";
            check_table_line(mixed_row_lines, 1, TableLineAxis::Row, false)?;
            check_table_line(mixed_row_lines, 2, TableLineAxis::Row, true)?;
            check_table_line(mixed_row_lines, 3, TableLineAxis::Row, true)?;
            // Boundary 4 is after the final row, not between two rows.
            check_table_line(mixed_row_lines, 4, TableLineAxis::Row, false)?;
            // Boundary zero is invalid for both axes.
            check_table_line(mixed_row_lines, 0, TableLineAxis::Row, false)?;
            return Ok(());
        });
    }

    #[test]
    fn at_left_edge() -> Result<()> {
        return xpath_test(|| {
        let mathml = "<math><mfrac><mrow><mn>30</mn><mi>x</mi></mrow><mn>4</mn></mfrac></math>";
        let package = parser::parse(mathml).map_err(|e| anyhow::anyhow!("failed to parse XML: {e}"))?;
        let mathml = get_element(&package);
        trim_element(mathml, false);
        let fraction = as_element(mathml.children()[0]);
        let mn = as_element(as_element(fraction.children()[0]).children()[0]);
        assert_eq!(EdgeNode::edge_node(mn, true, "2D"), Some(fraction));
        assert_eq!(EdgeNode::edge_node(mn, false, "2D"), None);

        let mi = as_element(as_element(fraction.children()[0]).children()[1]);
        assert_eq!(EdgeNode::edge_node(mi, true, "2D"), None);
        return Ok( () );
        });
    }

    #[test]
    fn at_right_edge() -> Result<()> {
        return xpath_test(|| {
        let mathml = "<math><mrow><mfrac><mn>4</mn><mrow><mn>30</mn><mi>x</mi></mrow></mfrac><mo>.</mo></mrow></math>";
        let package = parser::parse(mathml).map_err(|e| anyhow::anyhow!("failed to parse XML: {e}"))?;
        let mathml = get_element(&package);
        trim_element(mathml, false);
        let fraction = as_element(as_element(mathml.children()[0]).children()[0]);
        let mi = as_element(as_element(fraction.children()[1]).children()[1]);
        assert_eq!(EdgeNode::edge_node(mi, true, "2D"), None);
        assert_eq!(EdgeNode::edge_node(mi, false, "2D"), Some(fraction));
        assert_eq!(EdgeNode::edge_node(mi, false, "math"), Some(mathml));

        let mn = as_element(as_element(fraction.children()[1]).children()[0]);
        assert_eq!(EdgeNode::edge_node(mn, true, "2D"), None);
        return Ok( () );
        });
    }
}
