#![allow(clippy::needless_return)]

/// Chemistry terms used here:
/// chemical formula -- this references a molecule (one or more elements with bonds between them), including its state.
/// chemical equation -- this is a notation specialized to chemistry -- it has concentration, arrows, equality, "addition" along with 
///    some special symbols for operators and (mostly) chemical formulas for operands.
///    Operand exceptions are the equilibrium constant, numbers, and identifiers.
///    Although a chemical equation is a superset of a chemical formula, because we want to distinguish the two (e.g., '=' is in both),
///      we require that chemical equation is an mrow
///    FIX?? -- can it be an adorned mrow?
///    Note: with the current definition, if any element in a potential chem equation is ruled out, the entire mrow is ruled out.
///
/// The general flow is that for every element that looks like a chem formula/equation, we mark it with data-likely-[equation/formula]
/// After we are done marking "likely", we go back and either delete them or replace them with data-[equation/formula].
/// Note: anything already marked with data-[equation/formula] doesn't need recomputation later (essentially the result is cached)


/// There is a chicken and egg problem with detecting chemistry: to more reliably detect it, we need good structure.
/// However, to get the structure right (e.,g "=" being a double bond, not equality; chem elements being in 'mi's; ...),
///   we need to know "=" is part of a chemical formula.
/// The imperfect solution used is:
///   As the final step of each recursive call to 'clean_mathml',
///     1. mi/mtext: is it a chemical element(s) or one of the symbols used in chemical formulas (not equations).
///        If so, mark it MAYBE_CHEMISTRY.
///     2. msub/msup/msubsup/mmultiscripts: is base marked MAYBE_CHEMISTRY and the scripts are potential adornments, mark it MAYBE_CHEMISTRY
///     3. mrows: these take a few passes (remember, they aren't structured properly yet)
///        On the assumption that chemistry is not common we implement a "show me" attitude before changing the structure.
///        Pass 1:
///        a) for any run of mi/mtext that can be re-split into chem elements, split them and mark them if it is at least 3 chars long
///        b) if there are any potential chem formula operators (e.g., "=" and ":") and the previous node is marked MAYBE_CHEMISTRY,
///           mark this as MAYBE_CHEMISTRY
///        Pass 2: (assuming something was marked in pass 1)
///        a) find the first marked child and then the last consecutive marked child and trim any mo's from the ends
///        b) evaluate the likelihood that the sequence is chemistry
///           yes: replace mathml children with new (potentially restructured) children
///           no: clear all the marks for the old children
/// After canonicalization, we take another pass looking for chemical equations and marking them if found.

use sxd_document::dom::*;
use crate::canonicalize::*;
use crate::pretty_print::mml_to_string;
use crate::xpath_functions::is_leaf;
use regex::Regex;
use crate::xpath_functions::IsBracketed;
use phf::{phf_map, phf_set};
use std::convert::TryInto;
use std::collections::HashSet;


pub static NOT_CHEMISTRY: isize = -10000;  // should overwhelm any positive signal
static NOT_CHEMISTRY_THRESHOLD: isize = -10000/2;  // value for testing -- that way some can be added to NOT_CHEMISTRY and still meet the test
static IS_CHEMISTRY_THRESHOLD: isize = 3;


/// this might be chemistry -- should only exist during canonicalization
pub static MAYBE_CHEMISTRY: &str = "data-maybe-chemistry";

/// Attr flag to indicate chemical equation
static CHEM_EQUATION: &str = "data-chem-equation";
/// Attr flag to indicate chemical formula
static CHEM_FORMULA: &str = "data-chem-formula";
/// Attr flag to indicate chemical element
static CHEM_ELEMENT: &str = "data-chem-element";
static CHEM_FORMULA_OPERATOR: &str = "data-chem-formula-op";
static CHEM_EQUATION_OPERATOR: &str = "data-chem-equation-op";

/// these can be in the base of an under/over script
static CHEM_EQUATION_ARROWS: phf::Set<char> = phf_set! {
    '→', '➔', '←', '⟶', '⟵', '⤻', '⇋', '⇌',
    '↿', '↾', '⇃', '⇂', '⥮', '⥯', '⇷', '⇸', '⤉', '⤈',
    '⥂', '⥄',
    // '\u{2B96}', '\u{2B74}', '\u{2B75}',         // uncomment when defined in Unicode
};


pub fn is_chemistry_off() -> bool {
    let pref_manager = crate::prefs::PreferenceManager::get();
    return pref_manager.borrow().get_user_prefs().to_string("Chemistry") == "Off";
}

pub fn clean_chemistry_mrow(mathml: Element) {
    if is_chemistry_off() {
        return;
    }
    // debug!("clean_chemistry_mrow:\n{}", mml_to_string(&mathml));
    let mut children = mathml.children().iter()
                .map(|child| as_element(*child))
                .collect::<Vec<Element>>();
    if let Some(new_children) = clean_mrow_children_restructure_pass(&children) {
        mathml.replace_children(&new_children);
        children = new_children;
    }
    clean_mrow_children_mark_pass(&children);
}

/// Do some aggressive structural changes and if they make this look like a chemistry formula, make it as one else remove other marks
/// Note: the element is replaced with a new restructured element if it is marked as chemistry
///        Pass 1:
///        a) for any run of mi/mtext that can be re-split into chem elements, split them and mark them if it is at least 3 chars long
///        b) if there are any potential chem formula operators (e.g., "=" and ":") and the previous node is marked MAYBE_CHEMISTRY,
///           mark this as MAYBE_CHEMISTRY
fn clean_mrow_children_restructure_pass<'a>(old_children: &[Element<'a>]) -> Option<Vec<Element<'a>>> {
    let mut changed = false;
    let mut new_children = Vec::with_capacity(2*old_children.len());
    let mut i = 0;
    while i < old_children.len() {
        if let Some(paren_mrow_aq) = clean_aq_state(old_children, i) {
            new_children.push(paren_mrow_aq);
            i += 4;                                 // skipping "( a q )"
            changed = true;
            continue;
        } else if i + 2 < old_children.len() {
            if let Some(paren_mrow) = make_mrow(old_children[i..i+3].try_into().unwrap()) {
                // debug!("make_mrow added mrow");
                new_children.push(paren_mrow);
                i += 3;
                changed = true;
                continue;
            }
        }
        let child = old_children[i];
        let tag_name = name(&child);
        if tag_name == "mo" {
            let likely_chemistry_op = likely_chem_formula_operator(child);
            // debug!("clean_mrow_children_restructure_pass -- in mo: likely {}, {}", likely_chemistry_op, mml_to_string(&child));
            if likely_chemistry_op >= 0 {
                child.set_attribute_value(MAYBE_CHEMISTRY, likely_chemistry_op.to_string().as_str());
                // if possible chemistry to left and right, then override text for operator lookup
                // note: on the right, we haven't set chem flag for operators yet, so we skip them
                let preceding = child.preceding_siblings();
                let following = child.following_siblings();
                if !preceding.is_empty() && preceding.iter().all(|&child| as_element(child).attribute(MAYBE_CHEMISTRY).is_some()) &&
                   !following.is_empty() && following.iter().all(|&child| {
                        let child = as_element(child);
                        name(&child)=="mo" || child.attribute(MAYBE_CHEMISTRY).is_some()
                    }) {
                    // "=", etc., should be treated as high priority separators
                    child.set_attribute_value(OPERATOR_OVERRIDE, HIGH_PRIORITY_OPERATOR);
                }
            }
        }
        i += 1;
        new_children.push(child);
    }

    return if changed {Some(new_children)} else {None};
    

    /// if it looks like we have ChemFormula ( a q ), merge the 'a' and 'q' together into an 'mi'
    /// if not already true, structure '( aq )' into a single mrow (might be other elements on either side)
    /// returns the last char matched
    fn clean_aq_state<'a>(children: &[Element<'a>], i: usize) -> Option<Element<'a>> {
        if i+3 >= children.len() || (i > 0 && children[i-1].attribute(MAYBE_CHEMISTRY).is_none()) {
            return None;       // can't be '( a q )' -- not enough elements left or not Chem Formula on left
        }
        
        // this is a little sloppy in that we allow matching text in any leaf element, but we can use the same function
        if is_text(children[i], "(") &&
           is_text(children[i+1], "a") && is_text(children[i+2], "q") &&
           is_text(children[i+3], ")") {
            let mi = create_mathml_element(&children[i].document(), "mi");
            mi.set_text("aq");
            return make_mrow([children[i], mi, children[i+3]]);
        }
        return None;
    }

    fn is_text(node: Element, target: &str) -> bool {
        return is_leaf(node) && as_text(node) == target;
    }

    // converts  "( child )" to mrow with those elements as children 
    // this is to make ascertaining with this is a chemical state easier, but it is correct even if not a chemical state
    fn make_mrow(children: [Element; 3]) -> Option<Element> {
        // this is a little sloppy in that we allow matching text in any leaf element, but we can use the same function
        if is_text(children[0], "(") &&
           is_text(children[2], ")") {
			let mrow = create_mathml_element(&children[0].document(), "mrow");
			mrow.set_attribute_value(CHANGED_ATTR, ADDED_ATTR_VALUE);
			mrow.append_children(children);
            return Some(mrow);
        }
        return None;
    }
}

/// Pass 2: (assuming something was marked in pass 1)
/// a) find the first marked child and then the last consecutive marked child and trim any mo's from the ends
/// b) evaluate the likelihood that the sequence is chemistry
fn clean_mrow_children_mark_pass(children: &[Element]) {
    let mut start = None;
    for i in 0..children.len() {
        let child = children[i];
        if child.attribute(MAYBE_CHEMISTRY).is_some()  {
            if start == None {
                if name(&child) == "mo" {
                    // debug!(" start == None: removing MAYBE_CHEMISTRY on {}", as_text(child));
                    child.remove_attribute(MAYBE_CHEMISTRY);
                    child.remove_attribute(OPERATOR_OVERRIDE);
                } else {
                    start = Some(i);
                }
            }
        } else if let Some(seq_start) = start {
            if remove_operators_at_end_of_sequence(children, seq_start, i) {
                start = None;
            }
        }
    }

    if let Some(seq_start) = start {
        remove_operators_at_end_of_sequence(children, seq_start, children.len());
    }
    return;


    fn remove_operators_at_end_of_sequence(children: &[Element], start: usize, end: usize) -> bool {
        // debug!("  looking for ops at end of {}..{}, last is:{}", start, end, mml_to_string(&children[end-1]));
        for stop in (start..end).rev() {
            let end_child = children[stop];
            if name(&end_child) == "mo" {
                end_child.remove_attribute(MAYBE_CHEMISTRY);
            } else {
                return true;
            }
        }
        return false
}
}


/// Very little software gets the token elements for chemistry right.
/// Sometimes multiple elements are in a single token (e.g. "NaCl") and sometimes
/// a single element is spread across multiple tokens (e.g. "N", "a").
/// 
/// Here we attempt one or the other repair, but not both on the assumption there is 
/// consistency in the error.
/// 
/// Returns a Vec of the chemical elements or None. If a merge happened, the tree is altered.
pub fn convert_leaves_to_chem_elements(mathml: Element) -> Option<Vec<Element>> {
    // gather up all the consecutive mi/mtext
    if !(name(&mathml) == "mi" || name(&mathml) == "mtext") {
        return None;       // do nothing
    }

    // we play games with the string to avoid allocation...
    let token_string = as_text(mathml).as_bytes();
    if token_string.iter().any(|&ch| ch >=128) {
        return None;    // chemical elements are ASCII
    }
    let doc = mathml.document();
    let token_len = token_string.len();
    if token_len > 1 {
        return split_string_chem_element(&doc, token_string);
    }   
    let parent = mathml.parent().unwrap().element().unwrap();
    let parent_name = name(&parent);
    if !(parent_name == "mrow" || parent_name == "math") {  // not canonicalized yet
        return None;    // only try to merge if in an mrow
    }
    let answer = merge_tokens_chem_element(&doc, mathml, token_string, &mathml.following_siblings());
    if answer.is_none() && is_chemical_element(mathml) {
        mathml.set_attribute_value(MAYBE_CHEMISTRY, "1");
    }
    return answer;


    fn merge_tokens_chem_element<'a>(doc: &Document<'a>, leaf: Element<'a>, token_string: &[u8], following_siblings: &[ChildOfElement<'a>]) -> Option<Vec<Element<'a>>> {
        // FIX: need to handle three char chem elements (make a nested fn to deal with each char)
        if following_siblings.is_empty() {
            return None;
        }
        let second_element = as_element(following_siblings[0]);
        let second_element_name = name(&second_element);
        if second_element_name != "mi" && second_element_name != "mtext" {
            return None;
        }
        let second_element_text = as_text(second_element);
        if second_element_text.len() != 1 {
            return None;
        }
        let chem_token_string = vec![token_string[0], second_element_text.as_bytes()[0] as u8];
        if let Some(chem_element) = get_chem_element(doc, &chem_token_string, 2) {
            leaf.set_text(as_text(chem_element));
            leaf.set_attribute_value(MAYBE_CHEMISTRY, "2");
            second_element.remove_from_parent();
            return Some(vec![chem_element]);
        }
        return None;
    }

    fn split_string_chem_element<'a>(doc: &Document<'a>, token_string: &[u8]) -> Option<Vec<Element<'a>>> {
        let token_len = token_string.len();
        let mut j = 0;
        let mut new_children = Vec::with_capacity(token_string.len());
        while j < token_len {
            // try elements of length 3, 2, 1, preferring longer elements (e.g., prefer "Na" over "N")
            if let Some(chem_element) = get_chem_element(doc, &token_string[j..], 3) {
                new_children.push(chem_element);
                j += 3;
                continue;
            } else if let Some(chem_element) = get_chem_element(doc, &token_string[j..], 2) {
                new_children.push(chem_element);
                j += 2;
                continue;
            } else if let Some(chem_element) = get_chem_element(doc, &token_string[j..], 1) {
                new_children.push(chem_element);
                j += 1;
                continue;
            }
            return None;    // didn't find a valid chem element
        }

        return Some(new_children);
    }

    /// Returns element or None
    fn get_chem_element<'a>(doc: &Document<'a>, bytes_str: &[u8], n: usize) -> Option<Element<'a>> {
        use std::str;
        let len = bytes_str.len();
        if n > len {
            return None;    // can't be an chemical letter
        }
        let chem_element = unsafe{ str::from_utf8_unchecked(&bytes_str[..n]) };
        if CHEMICAL_ELEMENTS.contains_key( chem_element ) {
            return Some( new_chemical_element(doc, chem_element) );
        }

        return None;
    }

    fn new_chemical_element<'a>(doc: &Document<'a>, chem_element_str: &str) -> Element<'a> {
        let result = create_mathml_element(doc, "mi");
        result.set_text(chem_element_str);
        result.set_attribute_value(MAYBE_CHEMISTRY, chem_element_str.len().to_string().as_str());
        if chem_element_str.len() == 1 {
            result.set_attribute_value("mathvariant", "normal");
        }
        return result;
    }
}

/// Looks at the children of the element and uses heuristics to decide whether this is a chemical equation/formula
/// If it is, it is marked with either data-chem-equation or data-chem-formula
/// This function assumes proper structure
pub fn scan_and_mark_chemistry(mathml: Element) {
    if is_chemistry_off() {
        return;
    }

    debug!("scan_and_mark_chemistry:\n{}", mml_to_string(&mathml));
    assert_eq!(name(&mathml), "math");
    if is_chemistry_sanity_check(mathml) {
        assert_eq!(mathml.children().len(), 1);
        let child = as_element(mathml.children()[0]);
        let likelihood = likely_chem_formula(child);
        if likelihood >= IS_CHEMISTRY_THRESHOLD && child.attribute(CHEM_FORMULA).is_none() {
            child.set_attribute_value(MAYBE_CHEMISTRY, likelihood.to_string().as_str());
            set_marked_chemistry_attr(child, CHEM_FORMULA);
        }

        if name(&child) == "mrow" {
            let likelihood = likely_chem_equation(child);
            if likelihood >= IS_CHEMISTRY_THRESHOLD {
                child.set_attribute_value(MAYBE_CHEMISTRY, likelihood.to_string().as_str());
                set_marked_chemistry_attr(child, CHEM_EQUATION);
            }
        }
    }

    if mathml.attribute(CHEM_FORMULA).is_none() && mathml.attribute(CHEM_EQUATION).is_none() {
        unset_marked_chemistry(mathml);
    }
}

// returns the marked attr value or None
fn get_marked_value(mathml: Element) -> Option<isize> {
    if let Some(value) = mathml.attribute_value(MAYBE_CHEMISTRY) {
        return Some(value.parse().unwrap());
    } else {
        return None;
    }
}

/// Sets the attr 'chem'
/// Recurse through all the children that have MAYBE_CHEMISTRY set
pub fn set_marked_chemistry_attr(mathml: Element, chem: &str) {
    let tag_name = name(&mathml);
    if let Some(maybe) = mathml.attribute(MAYBE_CHEMISTRY) {
        maybe.remove_from_parent();

        match tag_name {
            "mi" | "mtext" => {mathml.set_attribute_value(CHEM_ELEMENT, maybe.value());},
            "mo" => {
                if mathml.attribute(CHEM_FORMULA_OPERATOR).is_none() {
                    // don't mark as both formula and equation
                    mathml.set_attribute_value(if chem == CHEM_FORMULA {CHEM_FORMULA_OPERATOR} else {CHEM_EQUATION_OPERATOR}, maybe.value());
                }
                },
            "mrow" | "msub" | "msup" | "msubsup" | "mmultiscripts" => {
                let mut chem_name = chem;
                if tag_name != "mrow" && chem != CHEM_FORMULA{
                    // look at base -- if an mi/mtext then this is really a chemical formula
                    let base = as_element(mathml.children()[0]);
                    let base_name = name(&base);
                    if base_name == "mi" || base_name == "mtext" {
                        chem_name = CHEM_FORMULA;
                    }
                }

                if mathml.attribute(CHEM_FORMULA).is_none() {
                    // don't mark as both formula and equation
                    mathml.set_attribute_value(chem_name, maybe.value());
                }
                for child in mathml.children() {
                    set_marked_chemistry_attr(as_element(child), chem);
                };
            }
            _ => error!("Internal error: {} should not be marked as 'MAYBE_CHEMISTRY'", tag_name),
        }
    } else if tag_name == "mrow" {
        // could have been added during canonicalization, so never marked. Recurse to the children
        for child in mathml.children() {
            set_marked_chemistry_attr(as_element(child), chem);
        };
    }
}

/// Clears MAYBE_CHEMISTRY from this element and its decedents
fn unset_marked_chemistry(mathml: Element) {
    // If MAYBE_CHEMISTRY is not set, we don't need to recurse
    mathml.remove_attribute(MAYBE_CHEMISTRY);
    if !is_leaf(mathml) {
        for child in mathml.children() {
            unset_marked_chemistry(as_element(child));
        }
    }
}

/// Returns true only if 'mathml' potentially is chemistry.
/// This assumes canonicalization has happened and that 'mathml' is the 'math' element
fn is_chemistry_sanity_check(mathml: Element) -> bool {
    // This does some sanity checking. More can definitely be done
    // Checks:
    // * if the child is an mrow with three children, the operator should be '=' or an arrow
    //   in this case, we gather up the elements on the lhs and rhs. The sets should be equal and non-empty.
    //   the exception is if there are prescripts, in which as we might have radioactive decay so we don't require the sets to be equal
    // * otherwise, we gather up all the chemical elements and make sure the set is non-empty
    // * if it isn't an mrow, we leave it to likely_chem_equation() to rule it out

    assert_eq!(name(&mathml), "math");
    assert_eq!(mathml.children().len(), 1);
    let mathml = as_element(mathml.children()[0]);
    if name(&mathml) == "mrow" {
        let mrow_children = mathml.children();
        if mrow_children.len() == 3 && is_arrow_or_equal(as_element(mrow_children[1])) {
            let mut lhs_elements = HashSet::with_capacity(8);   // likely more than anything we'll encounter -- bigger affects '=' op
            let lhs_has_prescripts = gather_chemical_elements(as_element(mrow_children[0]), &mut lhs_elements);
            let mut rhs_elements = HashSet::with_capacity(8);  // likely more than anything we'll encounter -- bigger affects '=' op
            let rhs_has_prescripts = gather_chemical_elements(as_element(mrow_children[2]), &mut rhs_elements);
            if lhs_elements.is_empty() {
                return false;
            }
            if lhs_elements == rhs_elements {
                return !(lhs_has_prescripts ^ rhs_has_prescripts);      // seems reasonable that if the lhs has prescripts, so should the rhs
            }
            return lhs_has_prescripts && rhs_has_prescripts;    // non-equal sets only if radioactive decay.
        }
    }
    let mut chem_elements = HashSet::with_capacity(8);   // likely more than anything we'll encounter -- bigger affects '=' op
    gather_chemical_elements(mathml, &mut chem_elements);
    return !chem_elements.is_empty();

    
    fn is_arrow_or_equal(mathml: Element) -> bool {
        let base = get_possible_embellished_node(mathml);
        if name(&base) != "mo" {
            return false;
        }
        let text = as_text(base);
        return text.len() == 1 && (text == "=" || CHEM_EQUATION_ARROWS.contains(&text.chars().next().unwrap()));

    }

    fn gather_chemical_elements<'a>(mathml: Element<'a>, chem_elements: &mut HashSet<&'a str>) -> bool {
        match name(&mathml) {
            "mi" | "mtext" => {
                if is_chemical_element(mathml) {
                    chem_elements.insert(as_text(mathml));
                }
                return false;
            },
            "msub" | "msup" | "msubsup" | "mmultiscripts" => return gather_chemical_elements(get_possible_embellished_node(mathml), chem_elements),
            "semantics" => {
                return gather_chemical_elements( get_presentation_element(mathml).1, chem_elements );
            },
           _ => if is_leaf(mathml) { return false; },
        }
    
        // mrow, msqrt, etc
        let mut has_prescripts = false;
        for child in mathml.children() {
            let child = as_element(child);
            has_prescripts |= gather_chemical_elements(child, chem_elements);
        }
        return has_prescripts;
    }
}

/// Looks at the children of the element and uses heuristics to decide whether this is a chemical equation.
/// This assumes canonicalization of characters has happened
fn likely_chem_equation(mathml: Element) -> isize {
    if name(&mathml) != "mrow" {
        return NOT_CHEMISTRY;
    }

	// mrow -- check the children to see if we are likely to be a chemical equation

    // possible improvement -- give bonus points for consecutive (not counting invisible separators) chemical elements on top of the existing points
	let mut likelihood = 0;						// indicator of likely match
	let mut has_equilibrium_constant = false;
    let children = mathml.children();
	for i in 0..children.len() {
		let child = as_element(children[i]);
        if let Some(likely) = get_marked_value(child) {
            likelihood += likely;
            continue;
        }
		if i == children.len()-1 {
            let likely = likely_chem_state(child);
            if likely > 0 {
                likelihood += likely;
                break;
			}
            // otherwise, check the last element as normal
        }
        let tag_name = name(&child);
        match tag_name {
            "mi" => likelihood += likely_chem_element(child),
            "mn" => (),       // not much info
            "mo" | "mover" | "munder" | "munderover" => {
                let likely = likely_chem_equation_operator(child);
                likelihood += likely;
            },
            "msub" | "msup" | "msubsup" | "mmultiscripts" => {
                if is_equilibrium_constant(child) {
                    has_equilibrium_constant = true;
                    likelihood += 2;
                } else {
                    likelihood += likely_adorned_chem_formula(child);
                }
            },
            "mfrac" => {
                if has_equilibrium_constant {
                    likelihood += 2;
                } else {
                    likelihood -= 3;    // fraction tend only to appear after an equilibrium constant
                }
            },
            "mrow" => {
                let likely = likely_chem_formula(child);
                if likely < NOT_CHEMISTRY_THRESHOLD {
                    likelihood += likely_chem_equation(child);
                } else {
                    likelihood += likely;
                }
                if likelihood < NOT_CHEMISTRY_THRESHOLD {
                    return NOT_CHEMISTRY;
                }
            },
            "semantics" => {
                return likely_chem_equation(get_presentation_element(mathml).1);
            }
            _ => return NOT_CHEMISTRY,
        };

        if likelihood < NOT_CHEMISTRY_THRESHOLD {
            return NOT_CHEMISTRY;
        }
    }

    if likelihood > 0 {
        mathml.set_attribute_value(MAYBE_CHEMISTRY, likelihood.to_string().as_str());
    }
    return likelihood;
}


/// could be a number, a state ("(l)", "(g)", etc), or a number followed by a state
fn likely_chem_subscript(supscript: Element) -> isize {
    let subscript_name = name(&supscript);
    if  subscript_name == "mn" && !as_text(supscript).contains('.') {
        return 0;       // not really much chem info about an integer subscript
    } else if subscript_name == "mi" {
        let text = as_text(supscript);
        if text == "s" || text == "l" ||text == "g" ||text == "aq" {
            return 2;
        }
    } else if subscript_name == "mrow" {
        let children = supscript.children();
        let i_first_child = as_element(children[0]);
        if children.len() == 2 &&
           name(&i_first_child) == "mn" && !as_text(i_first_child).contains('.') &&
           name (&as_element(children[1])) == "mrow" &&
           likely_chem_state(as_element(children[1])) > 0 { // notation used in en.wikipedia.org/wiki/Electrolyte#Formation
                return 2;
        }     
    }
    return NOT_CHEMISTRY;     
}

fn small_roman_to_number(text: &str) -> &str {
    // simplest to do a look up
    static ROMAN_TO_NUMBER: phf::Map<&str, &str> = phf_map! {
        "I" => "1", "II" => "2", "III" => "3", "IV" => "4", "V" => "5", "VI" => "6", "VII" => "7", "VIII" => "8", "IX" => "9",
    };
    return ROMAN_TO_NUMBER.get(text).unwrap_or(&"");

}

fn likely_valid_chem_superscript(sup: Element) -> isize {
    // either one or more '+'s (or '-'s) or a number followed by +/-
    // also could be state (en.wikipedia.org/wiki/Nuclear_chemistry#PUREX_chemistry)
    // bullet is radical (en.wikipedia.org/wiki/Radical_(chemistry)#Depiction_in_chemical_reactions); mhchem uses dot operator
    //  these can stand alone, be followed by +/- or have a number in front "(2•)-"" [examples from mhchem documentation]
    // roman numerals are "oxidation state" and range from -4 to +9
    lazy_static! {
        static ref MULTIPLE_PLUS_OR_MINUS_OR_DOT: Regex = Regex::new(r"^\++$|^-+$|^\U{2212}+$|^[⋅•]$").unwrap(); 
        static ref SINGLE_PLUS_OR_MINUS_OR_DOT: Regex = Regex::new(r"^[+-\U{2212}⋅•]$").unwrap(); 
    }

    let sup_name = name(&sup);
    if sup_name == "mo" && MULTIPLE_PLUS_OR_MINUS_OR_DOT.is_match(as_text(sup)) {
        return if as_text(sup).len()==1 {1} else {2};
    } else if (sup_name == "mi" || sup_name=="mtext") && SMALL_UPPER_ROMAN_NUMERAL.is_match(as_text(sup)){
        sup.set_attribute_value("data-number", small_roman_to_number(as_text(sup)));
        return 2;
    } else if sup_name == "mrow" {
        // look for something like '2+'
        let children = sup.children();
        if children.len() == 2 {
            let first = as_element(children[0]);
            let second = as_element(children[1]);
            if name(&first) == "mn" && name(&second) == "mo" && !as_text(first).contains('.') {
                let second_text = as_text(second);
                if SINGLE_PLUS_OR_MINUS_OR_DOT.is_match(second_text) {
                    return 2;   // ending with a +/- makes it likely this is an ion
                }
            }
        }
        // gather up the text and see if it is all +, -, etc
        let mut text = "".to_string();
        for child in children {
            let child = as_element(child);
            if name(&child) == "mo" {
                text.push_str(as_text(child));
            } else {
                return NOT_CHEMISTRY;
            }
        }
        if MULTIPLE_PLUS_OR_MINUS_OR_DOT.is_match(&text) {
            return if text.len()==1 {1} else {2};
        }
    }
    return NOT_CHEMISTRY;
}


/// chem_formula is likely if it is one of:
/// * a (possibly adorned) chemical element
/// * an operator that represents a bond
/// * fences around a chemical formula
/// * an mrow made up of only chemical formulas
/// * there needs to be at least chemical element (don't want a+b+c+d to be thought of as chemistry)
fn likely_chem_formula(mathml: Element) -> isize {
    if let Some(value) = get_marked_value(mathml) {
        return value;       // already marked
    }

    let tag_name = name(&mathml);
    match tag_name {
        // a parent may clear the chem flags if something says can't be chemistry (e.g, a non chemically valid script)
        "mi" => return likely_chem_element(mathml),
        "mo" => return likely_chem_formula_operator(mathml),
        "msub" | "msup" | "msubsup" | "mmultiscripts" => {
            likely_chem_formula(as_element(mathml.children()[0]));  // set MAYBE_CHEMISTRY attribute
            let likelihood = likely_adorned_chem_formula(mathml);
            if likelihood > 0 {
                mathml.set_attribute_value(MAYBE_CHEMISTRY, likelihood.to_string().as_str());
            }
            return likelihood;
        },
        "semantics" => {
            return likely_chem_formula(get_presentation_element(mathml).1);
        },
        "mrow" => {
            let chem_state = likely_chem_state(mathml);
            if chem_state > 0 {
                mathml.set_attribute_value(MAYBE_CHEMISTRY, chem_state.to_string().as_str());
                return chem_state;
            }

            let mut mrow = mathml;
            // check if it is bracketed -- doesn't add much info
            if (IsBracketed::is_bracketed(&mrow, "(", ")", false, false) ||
                IsBracketed::is_bracketed(&mrow, "[", "]", false, false)) &&
               name(&as_element(mrow.children()[1]))  == "mrow" {
                mrow = as_element(mrow.children()[1]);
            }

            let mut likelihood = if is_order_ok(mrow) {0} else {-4};

            // check all the children and compute the likelihood of that this is a chemical formula
            // bonus point for consecutive chemical formula children (not counting invisible children)
            let mut last_was_likely_formula = 0;        // 0 is false, 1 is true
            for child in mrow.children() {
                let child = as_element(child);
                let likely = likely_chem_formula(child);
                if likely > 0 {
                    likelihood += likely + last_was_likely_formula;
                    last_was_likely_formula = 1;
                } else if likely == 0 && name(&child) == "mo" {
                    let text = as_text(child);
                    if text != "\u{2062}" && text != "\u{2063}" {   // one of these, we don't change the status
                        last_was_likely_formula = 0;
                    }
                } else {
                    last_was_likely_formula = 0;
                    likelihood += likely;
                }
                debug!("in likely_chem_formula likelihood={}, child\n{}", likelihood, mml_to_string(&child));
                if likelihood < NOT_CHEMISTRY_THRESHOLD {
                    return NOT_CHEMISTRY;
                }
                debug!("likelihood={} (likely={})", likelihood, likely);
            }

            if likelihood <= NOT_CHEMISTRY {
                // the children may have looked have looked right, but something as said "not likely"
                return NOT_CHEMISTRY;
            } else {
                let likelihood_str = likelihood.to_string();
                if mathml != mrow {
                    mrow.set_attribute_value(MAYBE_CHEMISTRY, &likelihood_str);
                }
                mathml.set_attribute_value(MAYBE_CHEMISTRY, &likelihood_str);
                // set here so calls to unset_marked_chemistry from a parent won't change it being chem
                // set_marked_chemistry_attr(mathml, CHEM_FORMULA);
                return likelihood;
            }
        },
        _ => {
            if !is_leaf(mathml) {
                for child in mathml.children() {
                    let child = as_element(child);
                    let likelihood = likely_chem_formula(child);
                    if  likelihood >= IS_CHEMISTRY_THRESHOLD {
                        child.set_attribute_value(MAYBE_CHEMISTRY, likelihood.to_string().as_str());
                        // set_marked_chemistry_attr(child, CHEM_FORMULA);
                    };
                }
            }
            return NOT_CHEMISTRY;
        },
    }

}

/// This does some checks that sort of follow IUPAC's "Red Book" in section IR-4.4.
/// Those rules require knowledge that the program doesn't have (e.g., which bond is closest to the central atom).
/// Instead, we mainly use the two main types of orderings: alphabetical and electronegativity.
/// We first do a test to see if this looks like a structural formula -- if so, ordering doesn't apply.
/// If a formula has groupings, each grouping is checked independently of the rest since
///   there are cases where the outer ordering doesn't match the inner ordering.
/// For "generalized salts", we need to split the elements into positive and negative ions, and within each group
///   the order is suppose to be alphabetical but many use electronegativity (the point being there are two separate groups).
/// This site has a nice summary of the rules: https://chemistry.stackexchange.com/questions/537/why-is-arsenous-acid-denoted-h3aso3/538#538
/// Note: "(OH)" doesn't fit with the above, so a special case is made for that.
fn is_order_ok(mrow: Element) -> bool {
    assert_eq!(name(&mrow), "mrow");
    if is_structural(mrow) {
        return true;
    }
    
    let elements = collect_elements(mrow);
    if elements.iter().any(|&e| !CHEMICAL_ELEMENTS.contains_key(e)) {
        return false;
    }
    let n_elements = elements.len();
    if n_elements < 2 {
        return true;
    } else if n_elements == 2 && elements[0] == "O" && elements[1] == "H" {
        return true;    // special case "OH"
    } else {
        return !has_noble_element(&elements) &&
                // has_non_metal_element(&elements) && !has_non_metal_element(&elements) &&    // must have a metal and non-metal
                (is_alphabetical(&elements) || is_ordered_by_electronegativity(&elements) || is_generalized_salt(&elements));
    }
}

// from https://learnwithdrscott.com/ionic-bond-definition/
// I don't include the noble gases since they don't interact with other elements and are ruled out elsewhere
// fn has_non_metal_element(elements: &[&str]) -> bool {
//     static NON_METAL_ELEMENTS: phf::Set<&str> = phf_set! {
//         "H", "B", "C", "N", "O", "F", "Si", "P", "S", "Cl", "As", "Se", "Br", "Te", "I", "At",
//     };
//     return elements.iter().any(|&e| NON_METAL_ELEMENTS.contains(e));
// }


fn has_noble_element(elements: &[&str]) -> bool {
    static NOBLE_ELEMENTS: phf::Set<&str> = phf_set! {
        "He", "Ne", "Ar", "Kr", "Xe", "Rn", "Og" // Og might be reactive, but it is unstable
    };
    return elements.iter().any(|&e| NOBLE_ELEMENTS.contains(e));
}


fn is_structural(mrow: Element) -> bool {
    // FIX: implement
    assert_eq!(name(&mrow), "mrow");
    return false;
}

fn collect_elements<'a>(mrow: Element<'a>) -> Vec<&'a str> {
    let mut elements = Vec::with_capacity(mrow.children().len()/2+1);       // don't bother with slots for operators
    for child in mrow.children() {
        let child = as_element(child);
        match name(&child) {
            "mi" | "mtext" => elements.push(as_text(child)),
            "msub" | "msup" => {
                let base = as_element(child.children()[0]);
                let base_name = name(&base);
                if base_name == "mi" || base_name == "mtext" {
                    elements.push(as_text(base));
                }   // else skip and let recursive likely_chem_formula call check the contents
            },
            _ => (),    // let loop in likely_chem_formula() deal with all the negatives
        }
    }
    return elements;
}

/// check to make sure elements are ordered alphabetically
/// Actually check Hill's system that puts 'C' followed by 'H' first if 'C' is present
fn is_alphabetical(elements: &[&str]) -> bool {
    assert!(!elements.is_empty());
    if elements.len() == 1 {
        return true;    // trivially sorted
    }
    debug!("elements: {:?}", elements);
    let mut elements = elements;
    if elements[1..].iter().any(|&e| e=="C") {  // "C" must be first if present
        return false;
    }
    if elements[0] == "C" {
        elements = if elements[1]=="H" {&elements[2..]} else {&elements[1..]};
    }
    return elements.len() < 2 || elements.windows(2).all(|pair| pair[0] < pair[1]);
}

fn is_ordered_by_electronegativity(elements: &[&str]) -> bool {
    assert!(!elements.is_empty());
    return elements.len() < 2 || elements.windows(2).all(|pair| CHEMICAL_ELEMENTS.get(pair[0]).unwrap() < CHEMICAL_ELEMENTS.get(pair[1]).unwrap());
}

fn is_generalized_salt(elements: &[&str]) -> bool {
    assert!(!elements.is_empty());
    return false;
}


/// Returns the likelihood that the arg is an adorned chem formula
/// Adornments are:
///   superscripts with +/- and optionally a number (charge)
///  numeric subscripts (e.g. H_2)
/// In addition to chemical elements, we include nuclear decay since there is a lot of overlap in notation
/// The nuclear decay notation is mostly taken from https://tinyurl.com/2f6b8e3a
/// Basically it is a chemical element or 'e', 'p', 'n', 'α', 'β', or 'γ' with pre-sub/superscript
/// There is also an instance with a charge on the referenced page, so we allow that also.
/// 
/// Note: https://tinyurl.com/ysmr8cw2 says "++"/"--", etc., is sometimes used in a superscript particle physics instead of a "2"
/// 
/// Note:  msubsup cleaning for an empty script hasn't happened and we consider an empty script a sign of attempting to vertically align sub/superscripts

pub fn likely_adorned_chem_formula(mathml: Element) -> isize {
    // some simple sanity checks on the scripts...
    let tag_name = name(&mathml);
    let children = mathml.children();
    let mut likelihood = 0;
    let mut empty_subscript = false;
    debug!("likely_adorned_chem_formula:\n{}", mml_to_string(&mathml));
    if tag_name == "msub" || tag_name == "msubsup" {
        // subscripts should be just a number
        let subscript = as_element(children[1]);
        empty_subscript = name(&subscript) == "mtext" && as_text(subscript).trim().is_empty();
        if !empty_subscript {
            likelihood += likely_chem_subscript(subscript);
        }
    }

    let mut empty_superscript = false;
    if tag_name == "msup" || tag_name == "msubsup" {
        let superscript = as_element(children[if tag_name == "msup" {1} else {2}]);
        empty_superscript = name(&superscript) == "mtext" && as_text(superscript).trim().is_empty();
        if !empty_superscript {
            likelihood += likely_valid_chem_superscript(superscript);
        }
    }
    if tag_name == "msubsup" && (empty_subscript || empty_superscript) {
        likelihood += 1; // might be trying to vertically align scripts as in done in chemistry
    }

    if tag_name == "mmultiscripts" {
        // prescripts should be positive integers
        let prescripts = if children.len() == 4 && name(&as_element(children[1]))=="mprescripts" { // just prescripts
            &children[2..4]
        } else if children.len() == 6 && name(&as_element(children[3]))=="mprescripts" {  // pre and postscripts
            // postscript should be a charge
            let sub = as_element(children[1]);
            let sup = as_element(children[2]);
            if name(&sub)!="none" || name(&sup)=="none" {
                return NOT_CHEMISTRY;
            }
            likelihood += likely_valid_chem_superscript(sup);
            &children[1..3] 
        } else if children.len() % 2 == 1 {   // just postscripts
            for i in (1..children.len()).step_by(2) {
                let sub = as_element(children[i]);
                debug!("sub: {}", mml_to_string(&sub));
                if name(&sub) != "none" {
                    likelihood += likely_chem_subscript(sub);
                } 
                let sup = as_element(children[i+1]);
                if name(&sup) != "none" {
                    likelihood += likely_valid_chem_superscript(sup);
                } 
            }
            &children[0..0] // empty
        } else {
            return NOT_CHEMISTRY;
        };

        if !prescripts.is_empty() {
            if name(&as_element(prescripts[1])) != "mn" { // must have a pre-superscript (neutrons + protons)
                // fix could make sure they are integers
                return NOT_CHEMISTRY;
            }
            // deal with special case of 'e' with prescripts of -1 and 0
            if is_adorned_electron(children[0], prescripts) {
                return 100;     // very likely chemistry
            }
            let pre_subscript = as_element(prescripts[0]);
            let pre_subscript_name = name(&pre_subscript);
    
            if pre_subscript_name != "none" && pre_subscript_name!= "mn" {
                // fix could make sure they are integers
                return NOT_CHEMISTRY;
            }
            likelihood += 1;        // looking like an atomic number                
        }
    }

    let base = as_element(children[0]);
    let base_name = name(&base);
    if base_name == "mi" || base_name == "mtext" {
        likelihood += likely_chem_element(base);
    } else if base_name == "mrow" {
        likelihood += likely_chem_formula(base);
    } else {
        likelihood = NOT_CHEMISTRY;
    }
    
    return likelihood;


    fn is_adorned_electron(base: ChildOfElement, prescripts: &[ChildOfElement]) -> bool {
        // looking for 'e' with prescripts of -1 and 0
        let base = as_element(base);
        let pre_lower = as_element(prescripts[0]);
        let pre_upper = as_element(prescripts[1]);
        if (name(&base) == "mi" || name(&base) == "mtext") && as_text(base) == "e" &&
           name(&pre_upper) == "mn" && as_text(pre_upper) == "0" && 
           name(&pre_lower) == "mrow" && pre_lower.children().len() == 2 {
            // looking '-' and '1'
            let lower_children = pre_lower.children();
            let minus = as_element(lower_children[0]);
            let one = as_element(lower_children[1]);
            // not yet normalized, so we need to compare against ASCII minus and u+2022
            return name(&minus) == "mo" && (as_text(minus) == "-" || as_text(minus) == "−") && 
                   name(&one) == "mn"   && as_text(one) == "1";
        } else {
            return false;
        }
    }
}

/// useful function to see if the str is a single char that is in 'set'
fn is_in_set(leaf_text: &str, set: &phf::Set<char> ) -> bool {
    let mut chars = leaf_text.chars();
    let ch = chars.next();
    if chars.next().is_none() {     // only one char
        if let Some(first_ch) = ch {
            return set.contains(&first_ch);
        }
    }
    return false;
}

fn likely_chem_formula_operator(mathml: Element) -> isize {
    // mostly from chenzhijin.com/en/article/Useful%20Unicode%20for%20Chemists (Arrows and Other)
    static CHEM_FORMULA_OPERATORS: phf::Set<char> = phf_set! {
        '-', '\u{2212}', '=', '≡', '≣', '⠆', '⠇', '⠿', // bond symbols (need both 2212 and minus because maybe not canonicalized)
    };
    static CHEM_FORMULA_OK: phf::Set<char> = phf_set! {
        '(', ')', '[', ']',
        // FIX: the invisible operator between elements should be well-defined, but this likely needs work, so both accepted for now
        '\u{2062}', '\u{2063}' // invisible separators
        };

    assert_eq!(name(&mathml), "mo");
    let leaf_text = as_text(mathml);
    if is_in_set(leaf_text, &CHEM_FORMULA_OPERATORS) || leaf_text == "::" {
        mathml.set_attribute_value(MAYBE_CHEMISTRY, "1");
        return 1;
    } else if is_in_set(leaf_text, &CHEM_FORMULA_OK) {
        return 0;  // not much info
    } else {
        return -3; // still a small chance;
    } 
}

/// This assumes canonicalization of characters has happened
fn likely_chem_equation_operator(mathml: Element) -> isize {

    // mostly from chenzhijin.com/en/article/Useful%20Unicode%20for%20Chemists (Arrows and Other)
    static CHEM_EQUATION_OPERATORS: phf::Set<char> = phf_set! {
        '+', '=', '-',
        '·', '℃', '°', '‡', '∆', '×',
    };


    let elem_name = name(&mathml);
    if elem_name == "munder" || elem_name == "mover" || elem_name == "munderover" {
        let base = as_element(mathml.children()[0]);
        if name(&base) == "mo" && is_in_set(as_text(base), &CHEM_EQUATION_ARROWS) {
            base.set_attribute_value(MAYBE_CHEMISTRY, "1");
            return 1;
        } else if elem_name == "mover" && is_hack_for_missing_arrows(mathml) {
            return 2;
        } else {
            return NOT_CHEMISTRY;
        }    
    }

    if name(&mathml) == "mo" {
        let text = as_text(mathml);
        if is_in_set(text, &CHEM_EQUATION_OPERATORS) || is_in_set(text, &CHEM_EQUATION_ARROWS) {
            mathml.set_attribute_value(MAYBE_CHEMISTRY, "1");
            return 1;
        } else if text == "\u{2062}" || text == "\u{2063}" {
            // FIX: the invisible operator between elements should be well-defined, but this likely needs work, so both accepted for now
            return 0;
        }
    }
    return -3;  // there is still a chance

    /// Detects output of mhchem for some equilibrium arrows that currently (11/22) don't have Unicode points
    /// See github.com/NSoiffer/MathCAT/issues/60 for the patterns being matched
    fn is_hack_for_missing_arrows(mover: Element) -> bool {
        assert_eq!(name(&mover), "mover");
        let children = mover.children();
        let base = as_element(children[0]);
        let mo_base = if name(&base) == "mrow" && base.children().len() == 2 {
            as_element(base.children()[0])
        } else {
            base
        };
        let upper = as_element(children[1]);
        let mo_upper = if name(&upper) == "mrow" && upper.children().len() == 2 {
            as_element(upper.children()[1])
        } else {
            upper
        };
        // slightly sloppy match, but almost certainly good enough
        return name(&mo_base) == "mo" && name(&mo_upper) == "mo" && 
                as_text(mo_base) == "↽" && as_text(mo_upper) == "⇀";
        }
}

fn is_equilibrium_constant(mut mathml: Element) -> bool {
    if name(&mathml) == "msub" {
        mathml = as_element(mathml.children()[0]);
    }

    return name(&mathml) == "mi" && as_text(mathml) == "K";
}

lazy_static! {
    // Oxidation states range from -4 to 9 and are written with (a subset of) roman numerals.
    // All instances seem to be upper case that I've seen.
    static ref SMALL_UPPER_ROMAN_NUMERAL: Regex = Regex::new(r"^\s*^(IX|IV|V?I{0,3})\s*$").unwrap();
}

/// look for "(s), "(l)", "(g)", "(aq)" (could also use [...])
/// this might be called before canonicalization, but in clean_chemistry_mrow, we made sure "( xxx )" is grouped properly
pub fn likely_chem_state(mathml: Element) -> isize {
    
    if IsBracketed::is_bracketed(&mathml, "(", ")", false, false) ||
       IsBracketed::is_bracketed(&mathml, "[", "]", false, false) {
        let contents = as_element(mathml.children()[1]);
        let contents_name = name(&contents);
        if contents_name == "mi" || contents_name == "mtext" {
            let text = as_text(contents);
            if text == "s" || text == "l" ||text == "g" ||text == "aq" {
                return text.as_bytes().len() as isize;       // hack to count chars -- works because all are ASCII
            };
        }
     }
     return NOT_CHEMISTRY;
}

/// Returns the likelihood that the arg is an element
pub fn likely_chem_element(mathml: Element) -> isize {
    static NUCLEAR_SYMBOLS: [&str; 6] = ["e", "p", "n", "α", "β","γ"];

    assert!(name(&mathml) == "mi" || name(&mathml) == "mtext", "{} is not 'mi' or 'mtext'", name(&mathml));   
    let text = as_text(mathml) ;
    if is_chemical_element(mathml) {
        // single letter = 1; double =2; triple = 3
        return text.chars().count() as isize;
    } else if NUCLEAR_SYMBOLS.contains(&text) {
        // not much special about them;
        return 0;
    } else {
        return NOT_CHEMISTRY;
    }
}

/// A map of chemical elements and their relative IUPAC electronegativity (https://i.stack.imgur.com/VCSzW.png)
/// That list is only 85 elements and leaves out much of the Lanthanide and Actinide Series.
/// https://i0.wp.com/chemistry.com.pk/wp-content/uploads/2018/09/electronegativity-chart.jpg?fit=1697%2C1117&ssl=1 gives more
///   electronegativity values and they are added here (values in the 100s) and a values above pushed into the 200s.
///   The missing values range from 1.1 to 1.38 and fall between Ca (1.00) and Ti 
static CHEMICAL_ELEMENTS: phf::Map<&str, u32> = phf_map! {
	"Ac" => 20, "Ag" => 255, "Al" => 263, "Am" => 133, "Ar" => 4, "As" => 272, "At" => 281, "Au" => 254,
    "B" => 264, "Ba" => 14, "Be" => 18, "Bh" => 237, "Bi" => 270, "Bk" => 135, "Br" => 283,
	"C" => 269, "Ca" => 16, "Cd" => 258, "Ce" => 111, "Cf" => 136, "Cl" => 284, "Cm" => 134, "Cn" => 170, "Co" => 248, "Cr" => 236, "Cs" => 8, "Cu" => 256,
    "Db" => 159, "Ds" => 249, "Dy" => 124, 
	"Er" => 126, "Es" => 137, "Eu" => 120, "F" => 285, "Fe" => 244, "Fl" => 200, "Fm" => 138, "Fr" => 7, "Ga" => 262, "Gd" => 121, "Ge" => 267,
	"H" => 275, "He" => 6, "Hf" => 141, "Hg" => 257, "Ho" => 125, "Hs" => 241, "I" => 282, "In" => 261, "Ir" => 246, "K" => 10, "Kr" => 3,
	"La" => 22, "Li" => 12, "Lr" => 19, "Lu" => 21, "Lv" => 200, "Mc" => 200, "Md" => 139, "Mg" => 17, "Mn" => 240, "Mo" => 235, "Mt" => 245, 
	"N" => 274, "Na" => 11, "Nb" => 231, "Nd" => 116, "Ne" => 5, "Nh" => 200, "Ni" => 252, "No" => 140, "Np" => 145, "O" => 280, "Og" => 8, "Os" => 242, 
	"P" => 273, "Pa" => 147, "Pb" => 265, "Pd" => 251, "Pm" => 115, "Po" => 276, "Pr" => 113, "Pt" => 250, "Pu" => 129,
	"Ra" => 13, "Rb" => 9, "Re" => 238, "Rf" => 155, "Rg" => 253, "Rh" => 247, "Rn" => 1, "Ru" => 243, 
	"S" => 279, "Sb" => 271, "Sc" => 154, "Se" => 278, "Sg" => 233, "Si" => 268, "Sm" => 117, "Sn" => 266, "Sr" => 15,
	"Ta" => 230, "Tb" => 123, "Tc" => 239, "Te" => 277, "Th" => 132, "Ti" => 158, "Tl" => 260, "Tm" => 127, "Ts" => 200, 
	"U" => 146, "V" => 232, "W" => 234, "Xe" => 2, "Y" => 23, "Yb" => 110, "Zn" => 259, "Zr" => 143,
    // The following come from E.A. Moore who said to treat them like chemicals 
    // These stand for methyl, ethyl, alkyl, acetyl and phenyl and apparently are quite commonly used ("Ac" is already a chemical)
    // FIX:
    "Me" => 130, "Et" => 130, "R" => 130, /* "Ac" => 130, */ "Ph" => 130,
};

pub fn is_chemical_element(node: Element) -> bool {
	// FIX: allow name to be in an mrow (e.g., <mi>N</mi><mi>a</mi>
	let name = name(&node);
	if name != "mi" && name != "mtext" {
		return false;
	}

	let text = as_text(node);
	return CHEMICAL_ELEMENTS.contains_key(text);
}


#[cfg(test)]
mod chem_tests {
	#[allow(unused_imports)]
	use super::super::init_logger;
	use super::super::are_strs_canonically_equal;
    use super::*;

    fn parse_mathml_string<F>(test: &str, test_mathml: F) -> bool
            where F: Fn(Element) -> bool {
        use sxd_document::parser;
        use crate::interface::{get_element, trim_element};

        let new_package = parser::parse(&test);    
        if let Err(e) = new_package {
            panic!("Invalid MathML input:\n{}\nError is: {}", &test, &e.to_string());
        }

        let new_package = new_package.unwrap();
        let mathml = get_element(&new_package);
        trim_element(&mathml);
        return test_mathml(mathml);
    }

    #[test]
    fn test_noble_element() {
        // mathml test strings need to be canonical MathML since we aren't testing canonicalize()
        let test = "<mrow> <mi>Na</mi> <mo>&#x2063;</mo> <mi>Cl</mi> </mrow>"; // 
        assert!( !parse_mathml_string(test, |mathml| has_noble_element( &collect_elements(mathml))) );
        let test = "<mrow> <mi>Ar</mi> <mo>&#x2063;</mo> <mi>Cl</mi> </mrow>"; // 
        assert!( parse_mathml_string(test, |mathml| has_noble_element( &collect_elements(mathml))) );
        let test = "<mrow> <mi>Ne</mi> </mrow>"; // 
        assert!( parse_mathml_string(test, |mathml| has_noble_element( &collect_elements(mathml))) );
    }

    #[test]
    fn test_alphabetical_order() {
        // mathml test strings need to be canonical MathML since we aren't testing canonicalize()
        let test = r#"<mrow>  
            <msub><mi>C</mi><mn>6</mn></msub><mo>&#x2063;</mo> 
            <msub><mi>H</mi><mn>14</mn></msub>
             </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_alphabetical( &collect_elements(mathml))) );
        let test = r#"<mrow>  
             <msub><mi>C</mi><mn>6</mn></msub><mo>&#x2063;</mo> 
             <msub><mi>H</mi><mn>12</mn></msub><mo>&#x2063;</mo>
             <msub><mi>O</mi><mn>6</mn></msub>
              </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_alphabetical( &collect_elements(mathml))) );
        let test = "<mrow> <mi>B</mi> <mo>&#x2063;</mo> <mi>C</mi> <mo>&#x2063;</mo> <mi>O</mi></mrow>"; // "C" should be first
        assert!( !parse_mathml_string(test, |mathml| is_alphabetical( &collect_elements(mathml))) );
        let test = "<mrow> <mi>P</mi> <mo>&#x2063;</mo> <mi>B</mi> <mo>&#x2063;</mo> <mi>O</mi></mrow>"; // not alphabetical
        assert!( !parse_mathml_string(test, |mathml| is_alphabetical( &collect_elements(mathml))) );
    }


    #[test]
    fn test_electronegativity_order() {
        // mathml test strings need to be canonical MathML since we aren't testing canonicalize()
        let test = r#"<mrow>  
            <mi>N</mi><mo>&#x2063;</mo> 
            <msub><mi>H</mi><mn>3</mn></msub>
             </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_ordered_by_electronegativity( &collect_elements(mathml))) );
        let test = r#"<mrow>  
            <mi>O</mi><mo>&#x2063;</mo> 
            <msub><mi>F</mi><mn>2</mn></msub>
             </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_ordered_by_electronegativity( &collect_elements(mathml))) );
        let test = r#"<mrow>  
            <msub><mi>Rb</mi><mn>15</mn></msub><mo>&#x2063;</mo> 
            <msub><mi>Hg</mi><mn>16</mn></msub>
             </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_ordered_by_electronegativity( &collect_elements(mathml))) );
        let test = r#" 
            <msup>
                <mo>[</mo>
                    <mi>Si</mi><mo>&#x2063;</mo> 
                    <msub><mi>As</mi><mn>4</mn></msub>
                <mo>]</mo>
                <mrow><mn>8</mn><mo>-</mo></mrow>
            </msup>"#;
        assert!( parse_mathml_string(test, |mathml| is_ordered_by_electronegativity( &collect_elements(mathml))) );
        let test = r#"<mrow>  
                <mi>Si</mi><mo>&#x2063;</mo> 
                <msub><mi>H</mi><mn>2</mn></msub>
                <mi>Br</mi><mo>&#x2063;</mo> 
                <mi>Cl</mi>
                </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_ordered_by_electronegativity( &collect_elements(mathml))) );
    }

    #[test]
    fn test_order() {
        let test = r#"<mrow>  
            <msub><mi>C</mi><mn>2</mn></msub><mo>&#x2063;</mo> 
            <msub><mi>H</mi><mn>4</mn></msub><mo>&#x2063;</mo>
            <msub><mrow> <mo>(</mo><mi>N</mi> <mo>&#x2063;</mo> <msub> <mi>H</mi> <mn>2</mn> </msub><mo>)</mo> </mrow><mn>2</mn></msub>
             </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_order_ok(mathml)) );
        let test = r#"<mrow>
            <mi>Fe</mi><mo>&#x2063;</mo> 
            <mi>O</mi><mo>&#x2063;</mo> 
            <mrow> <mo>(</mo><mrow><mi>O</mi> <mo>&#x2063;</mo><mi>H</mi> </mrow><mo>)</mo> </mrow>
             </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_order_ok(mathml)) );
        let test = r#"<mrow>  // R-4.4.3.3 -- Chain compound doesn't fit rules but should be accepted
                <mi>Br</mi><mo>&#x2063;</mo> 
                <mi>S</mi><mo>&#x2063;</mo> 
                <mi>C</mi><mo>&#x2063;</mo> 
                <mi>N</mi>
                </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| likely_chem_formula(mathml)==4) );
    }

    #[test]
    fn split_mi() {
        let test = "<math><mi>LiF</mi></math>";
        let target = " <math>
        <mrow data-changed='added' data-chem-formula='4'>
          <mi data-chem-element='2'>Li</mi>
          <mo data-changed='added'>&#x2063;</mo>
          <mi data-chem-element='1' mathvariant='normal'>F</mi>
        </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn combine_mi() {
        let test = "<math><mi>H</mi><mi>C</mi><mi>l</mi></math>";
        let target = " <math>
        <mrow data-changed='added' data-chem-formula='4'>
          <mi data-chem-element='1'>H</mi>
          <mo data-changed='added'>&#x2063;</mo>
          <mi data-chem-element='2'>Cl</mi>
        </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn add_script() {
        let test = "<math> <mi>SO</mi>  <msub> <mrow></mrow> <mn>2</mn> </msub> </math>";
        let target = " <math>
        <mrow data-changed='added' data-chem-formula='3'>
          <mi data-chem-element='1' mathvariant='normal'>S</mi>
          <mo data-changed='added'>&#x2063;</mo>
          <mmultiscripts data-chem-formula='1'>
            <mi data-chem-element='1' mathvariant='normal'>O</mi>
            <mn>2</mn>
            <none></none>
          </mmultiscripts>
        </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn salt() {
        let test = "<math><mi>Na</mi><mi>Cl</mi></math>";
        let target = " <math>
        <mrow data-changed='added' data-chem-formula='5'>
          <mi data-chem-element='2'>Na</mi>
          <mo data-changed='added'>&#x2063;</mo>
          <mi data-chem-element='2'>Cl</mi>
        </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn water() {
        let test = "<math><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub><mi mathvariant='normal'>O</mi></math>";
        let target = " <math>
            <mrow data-changed='added' data-chem-formula='3'>
                <msub data-chem-formula='1'>
                    <mi data-chem-element='1' mathvariant='normal'>H</mi>
                    <mn>2</mn>
                </msub>
                <mo data-changed='added'>&#x2063;</mo>
                <mi data-chem-element='1' mathvariant='normal'>O</mi>
            </mrow>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn mhchem_water() {
        let test = "<math>
            <mrow>
            <mrow>
                <mi mathvariant='normal'>H</mi>
            </mrow>
            <msub>
                <mrow>
                <mrow>
                    <mpadded width='0'>
                    <mphantom>
                        <mi>A</mi>
                    </mphantom>
                    </mpadded>
                </mrow>
                </mrow>
                <mrow>
                <mrow>
                    <mpadded height='0'>
                    <mn>2</mn>
                    </mpadded>
                </mrow>
                </mrow>
            </msub>
            <mrow>
                <mi mathvariant='normal'>O</mi>
            </mrow>
            </mrow>
        </math>";
        let target = "<math>
            <mrow data-chem-formula='3'>
            <mmultiscripts data-chem-formula='1'>
                <mi mathvariant='normal' data-chem-element='1'>H</mi>
                <mn>2</mn>
                <none></none>
            </mmultiscripts>
            <mo data-changed='added'>&#x2063;</mo>
            <mi mathvariant='normal' data-chem-element='1'>O</mi>
            </mrow>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn carbon() {
        let test = "<math><mi>C</mi></math>";     // not enough to trigger recognition
        let target = " <math>
            <mi>C</mi>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn sulfate() {
        let test = "<math><mrow><msup>
                <mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
                <mrow><mn>2</mn><mo>&#x2212;</mo></mrow>
            </msup></mrow></math>";
        let target = "<math>
        <msup data-chem-formula='5'>
          <mrow data-chem-formula='3'>
            <mo>[</mo>
            <mrow data-changed='added'>
              <mi data-chem-element='1'>S</mi>
              <mo data-changed='added'>&#x2063;</mo>
              <msub data-chem-formula='1'>
                <mi data-chem-element='1'>O</mi>
                <mn>4</mn>
              </msub>
            </mrow>
            <mo>]</mo>
          </mrow>
          <mrow>
            <mn>2</mn>
            <mo>-</mo>
          </mrow>
        </msup>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn aluminum_sulfate() {
        let test = "<math><mrow><msub><mi>Al</mi><mn>2</mn></msub>
                <msub><mrow><mo>(</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>)</mo></mrow><mn>3</mn></msub></mrow></math>";
        let target = "<math>
        <mrow data-chem-formula='6'>
          <msub data-chem-formula='2'>
            <mi data-chem-element='2'>Al</mi>
            <mn>2</mn>
          </msub>
          <mo data-changed='added'>&#x2063;</mo>
          <msub data-chem-formula='3'>
            <mrow data-chem-formula='3'>
              <mo>(</mo>
              <mrow data-changed='added'>
                <mi data-chem-element='1'>S</mi>
                <mo data-changed='added'>&#x2063;</mo>
                <msub data-chem-formula='1'>
                  <mi data-chem-element='1'>O</mi>
                  <mn>4</mn>
                </msub>
              </mrow>
              <mo>)</mo>
            </mrow>
            <mn>3</mn>
          </msub>
        </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn ethanol_bonds() {
        let test = "<math>
                <mrow>
                    <mi>C</mi>
                    <msub>  <mi>H</mi> <mn>3</mn> </msub>
                    <mo>&#x2212;</mo>
                    <mi>C</mi>
                    <msub>  <mi>H</mi> <mn>2</mn> </msub>
                    <mo>&#x2212;</mo>
                    <mi>O</mi>
                    <mi>H</mi>
                </mrow>
            </math>";
        let target = "<math>
                <mrow data-chem-formula='15'>
                    <mi data-chem-element='1'>C</mi>
                    <mo data-changed='added'>&#x2063;</mo>
                    <mrow data-changed='added' data-chem-formula='13'>
                    <msub data-chem-formula='1'>
                        <mi data-chem-element='1'>H</mi>
                        <mn>3</mn>
                    </msub>
                    <mo data-operator='&#xf8ff;' data-chem-formula-op='1'>-</mo>
                    <mrow data-changed='added' data-chem-formula='9'>
                        <mi data-chem-element='1'>C</mi>
                        <mo data-changed='added'>&#x2063;</mo>
                        <mrow data-changed='added' data-chem-formula='7'>
                        <msub data-chem-formula='1'>
                            <mi data-chem-element='1'>H</mi>
                            <mn>2</mn>
                        </msub>
                        <mo data-operator='&#xf8ff;' data-chem-formula-op='1'>-</mo>
                        <mrow data-changed='added' data-chem-formula='3'>
                            <mi data-chem-element='1'>O</mi>
                            <mo data-changed='added'>&#x2063;</mo>
                            <mi data-chem-element='1'>H</mi>
                        </mrow>
                        </mrow>
                    </mrow>
                </mrow>
            </mrow>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn dichlorine_hexoxide() {
        let test = "<math><mrow>
        <msup>
          <mrow><mo>[</mo><mi>Cl</mi><msub><mi>O</mi><mn>2</mn></msub><mo>]</mo></mrow>
          <mo>+</mo>
        </msup>
        <msup>
          <mrow><mo>[</mo><mi>Cl</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
          <mo>-</mo>
        </msup>
      </mrow></math>";
        let target = " <math>
            <mrow data-chem-formula='11'>
                <msup data-chem-formula='5'>
                    <mrow data-chem-formula='4'>
                    <mo>[</mo>
                    <mrow data-changed='added'>
                        <mi data-chem-element='2'>Cl</mi>
                        <mo data-changed='added'>&#x2063;</mo>
                        <msub data-chem-formula='1'>
                        <mi data-chem-element='1'>O</mi>
                        <mn>2</mn>
                        </msub>
                    </mrow>
                    <mo>]</mo>
                    </mrow>
                    <mo>+</mo>
                </msup>
                <mo data-changed='added'>&#x2063;</mo>
                <msup data-chem-formula='5'>
                    <mrow data-chem-formula='4'>
                    <mo>[</mo>
                    <mrow data-changed='added'>
                        <mi data-chem-element='2'>Cl</mi>
                        <mo data-changed='added'>&#x2063;</mo>
                        <msub data-chem-formula='1'>
                        <mi data-chem-element='1'>O</mi>
                        <mn>4</mn>
                        </msub>
                    </mrow>
                    <mo>]</mo>
                    </mrow>
                    <mo>-</mo>
                </msup>
            </mrow>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn ethylene_with_bond() {
        let test = "<math><mrow>
                <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
                <mo>=</mo>
                <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
            </mrow></math>";
        let target = "<math>
            <mrow data-chem-formula='9'>
                <msub data-chem-formula='1'>
                    <mi data-chem-element='1'>H</mi>
                    <mn>2</mn>
                </msub>
                <mo data-changed='added'>&#x2063;</mo>
                <mrow data-changed='added' data-chem-formula='7'>
                    <mi data-chem-element='1'>C</mi>
                    <mo data-operator='&#xf8ff;' data-chem-formula-op='1'>=</mo>
                    <mrow data-changed='added'  data-chem-formula='3'>
                    <mi data-chem-element='1'>C</mi>
                    <mo data-changed='added'>&#x2063;</mo>
                    <msub data-chem-formula='1'>
                        <mi data-chem-element='1'>H</mi>
                        <mn>2</mn>
                    </msub>
                    </mrow>
                </mrow>
            </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn ferric_chloride_aq() {
        let test = "<math><mrow>
            <mi>Fe</mi>
            <msub><mi>Cl</mi><mn>3</mn></msub>
            <mrow><mo>(</mo><mrow><mi>aq</mi></mrow><mo>)</mo></mrow>
        </mrow></math>";
        let target = "<math>
                <mrow data-chem-formula='8'>
                    <mi data-chem-element='2'>Fe</mi>
                    <mo data-changed='added'>&#x2063;</mo>
                    <msub data-chem-formula='2'>
                        <mi data-chem-element='2'>Cl</mi>
                        <mn>3</mn>
                    </msub>
                    <mo data-changed='added'>&#x2063;</mo>
                    <mrow data-chem-formula='2'>
                        <mo>(</mo>
                        <mi>aq</mi>
                        <mo>)</mo>
                    </mrow>
                </mrow>
            </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn mchem_so4() {
        let test = "<math>
            <mstyle mathcolor='#a33e00'>
            <mrow>
                <mi>SO</mi>
                <msub>
                <mrow>
                    <mrow>
                    <mpadded width='0'>
                        <mphantom>
                        <mi>A</mi>
                        </mphantom>
                    </mpadded>
                    </mrow>
                </mrow>
                <mrow>
                    <mrow>
                    <mpadded height='0'>
                        <mn>4</mn>
                    </mpadded>
                    </mrow>
                </mrow>
                </msub>
            </mrow>
            </mstyle>
        </math>";
        let target = "<math>
            <mrow  mathcolor='#a33e00' data-chem-formula='3'>
                <mi data-chem-element='1' mathvariant='normal'>S</mi>
                <mo data-changed='added'>&#x2063;</mo>
                <mmultiscripts data-chem-formula='1'>
                    <mi data-chem-element='1' mathvariant='normal'>O</mi>
                    <mn>4</mn>
                    <none></none>
                </mmultiscripts>
            </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn mchem_so4_with_extra_mrow() {
        let test = "<math>
            <mstyle mathcolor='#a33e00'>
            <mrow>
                <mrow>
                <mi>SO</mi>
                </mrow>
                <msub>
                <mrow>
                    <mrow>
                    <mpadded width='0'>
                        <mphantom>
                        <mi>A</mi>
                        </mphantom>
                    </mpadded>
                    </mrow>
                </mrow>
                <mrow>
                    <mrow>
                    <mpadded height='0'>
                        <mn>4</mn>
                    </mpadded>
                    </mrow>
                </mrow>
                </msub>
            </mrow>
            </mstyle>
        </math>";
        let target = "<math>
                <mrow mathcolor='#a33e00' data-chem-formula='3'>
                    <mi data-chem-element='1' mathvariant='normal'>S</mi>
                    <mo data-changed='added'>&#x2063;</mo>
                    <mmultiscripts data-chem-formula='1'>
                        <mi data-chem-element='1' mathvariant='normal'>O</mi>
                        <mn>4</mn>
                        <none></none>
                    </mmultiscripts>
                </mrow>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn mchem_ions_and_state() {
        let test = "<math>
            <mrow>
            <mrow>
                <mi>Na</mi>
            </mrow>
            <msup>
                <mrow>
                <mrow>
                    <mpadded width='0'>
                    <mphantom>
                        <mi>A</mi>
                    </mphantom>
                    </mpadded>
                </mrow>
                </mrow>
                <mrow>
                <mo>+</mo>
                </mrow>
            </msup>
            <mo stretchy='false'>(</mo>
            <mrow>
                <mi>aq</mi>
            </mrow>
            <mo stretchy='false'>)</mo>
            <mrow>
                <mi>Cl</mi>
            </mrow>
            <msup>
                <mrow>
                <mrow>
                    <mpadded width='0'>
                    <mphantom>
                        <mi>A</mi>
                    </mphantom>
                    </mpadded>
                </mrow>
                </mrow>
                <mrow>
                <mo>&#x2212;</mo>
                </mrow>
            </msup>
            <mspace width='0.111em'></mspace>
            <mo stretchy='false'>(</mo>
            <mrow>
                <mi>aq</mi>
            </mrow>
            <mo stretchy='false'>)</mo>
            </mrow>
            </math>";
        let target = "<math>
            <mrow data-chem-formula='14'>
            <mmultiscripts data-chem-formula='3'>
                <mi data-chem-element='2'>Na</mi>
                <none></none>
                <mo>+</mo>
            </mmultiscripts>
            <mo data-changed='added'>&#x2063;</mo>
            <mrow data-changed='added' data-chem-formula='2'>
                <mo stretchy='false'>(</mo>
                <mi>aq</mi>
                <mo stretchy='false'>)</mo>
            </mrow>
            <mo data-changed='added'>&#x2063;</mo>
            <mmultiscripts data-chem-formula='4'>
                <mi data-chem-element='2'>Cl</mi>
                <none></none>
                <mo>-</mo>
            </mmultiscripts>
            <mo data-changed='added'>&#x2063;</mo>
            <mrow data-changed='added' data-chem-formula='2'>
                <mo stretchy='false'>(</mo>
                <mi>aq</mi>
                <mo stretchy='false'>)</mo>
            </mrow>
            </mrow>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn ethylene_with_colon_bond() {
        let test = "<math><mrow>
                <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
                <mo>::</mo>
                <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
            </mrow></math>";
        let target = "<math>
            <mrow data-chem-formula='9'>
                <msub data-chem-formula='1'>
                    <mi data-chem-element='1'>H</mi>
                    <mn>2</mn>
                </msub>
                <mo data-changed='added'>&#x2063;</mo>
                <mrow data-changed='added' data-chem-formula='7'>
                    <mi data-chem-element='1'>C</mi>
                    <mo data-operator='&#xf8ff;' data-chem-formula-op='1'>::</mo>
                    <mrow data-changed='added' data-chem-formula='3'>
                    <mi data-chem-element='1'>C</mi>
                    <mo data-changed='added'>&#x2063;</mo>
                    <msub data-chem-formula='1'>
                        <mi data-chem-element='1'>H</mi>
                        <mn>2</mn>
                    </msub>
                    </mrow>
                </mrow>
            </mrow>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn mhchem_u238() {
        let test = "<math>
        <mrow>
          <msubsup>
            <mrow>
              <mrow>
                <mpadded width='0'>
                  <mphantom>
                    <mi>A</mi>
                  </mphantom>
                </mpadded>
              </mrow>
            </mrow>
            <mrow>
              <mrow>
                <mpadded height='0' depth='0'>
                  <mphantom></mphantom>
                </mpadded>
              </mrow>
            </mrow>
            <mrow>
              <mrow>
                <mpadded height='0' depth='0'>
                  <mphantom>
                    <mn>238</mn>
                  </mphantom>
                </mpadded>
              </mrow>
            </mrow>
          </msubsup>
          <mspace width='-0.083em' linebreak='nobreak'></mspace>
          <msubsup>
            <mrow>
              <mrow>
                <mpadded width='0'>
                  <mphantom>
                    <mi>A</mi>
                  </mphantom>
                </mpadded>
              </mrow>
            </mrow>
            <mrow>
              <mrow>
                <mpadded width='0'>
                  <mphantom>
                    <mn>2</mn>
                  </mphantom>
                </mpadded>
              </mrow>
              <mrow>
                <mpadded width='0' lspace='-1width'>
                  <mrow>
                    <mpadded height='0'></mpadded>
                  </mrow>
                </mpadded>
              </mrow>
            </mrow>
            <mrow>
              <mrow>
                <mpadded height='0'>
                  <mrow>
                    <mpadded width='0'>
                      <mphantom>
                        <mn>2</mn>
                      </mphantom>
                    </mpadded>
                  </mrow>
                </mpadded>
              </mrow>
              <mrow>
                <mpadded width='0' lspace='-1width'>
                  <mn>238</mn>
                </mpadded>
              </mrow>
            </mrow>
          </msubsup>
          <mrow>
            <mi mathvariant='normal'>U</mi>
          </mrow>
        </mrow>
      </math>";
        let target = "<math>
            <mmultiscripts>
            <mi mathvariant='normal'>U</mi>
            <mprescripts></mprescripts>
            <none/>
            <mn>238</mn>
            </mmultiscripts>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn mhchem_hcl_aq() {
        let test = "<math>
        <mrow>
          <mn>2</mn>
          <mstyle scriptlevel='0'>
            <mspace width='0.167em'></mspace>
          </mstyle>
          <mrow>
            <mi>HCl</mi>
          </mrow>
          <mspace width='0.111em'></mspace>
          <mo stretchy='false'>(</mo>
          <mrow>
            <mi>aq</mi>
          </mrow>
          <mo stretchy='false'>)</mo>
        </mrow>
      </math>";
        let target = "<math>
        <mrow data-chem-equation='7'>
          <mn>2</mn>
          <mo data-changed='added'>&#x2062;</mo>
          <mrow data-changed='added' data-chem-equation='7'>
            <mi mathvariant='normal' data-chem-element='1'>H</mi>
            <mo data-changed='added'>&#x2063;</mo>
            <mi data-chem-element='2'>Cl</mi>
            <mo data-changed='added'>&#x2063;</mo>
            <mrow data-changed='added' data-chem-equation='2'>
              <mo stretchy='false'>(</mo>
              <mi>aq</mi>
              <mo stretchy='false'>)</mo>
            </mrow>
          </mrow>
        </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn mhchem_nested_sub() {
        // from \ce{(CH3)3}
        let test = "<math>
        <mrow>
          <mo stretchy='false'>(</mo>
          <mrow>
            <mi>CH</mi>
          </mrow>
          <msub>
            <mrow>
              <mrow>
                <mpadded width='0'>
                  <mphantom>
                    <mi>A</mi>
                  </mphantom>
                </mpadded>
              </mrow>
            </mrow>
            <mrow>
              <mrow>
                <mpadded height='0'>
                  <mn>3</mn>
                </mpadded>
              </mrow>
            </mrow>
          </msub>
          <mo stretchy='false'>)</mo>
          <msub>
            <mrow>
              <mrow>
                <mpadded width='0'>
                  <mphantom>
                    <mi>A</mi>
                  </mphantom>
                </mpadded>
              </mrow>
            </mrow>
            <mrow>
              <mrow>
                <mpadded height='0'>
                  <mn>3</mn>
                </mpadded>
              </mrow>
            </mrow>
          </msub>
        </mrow>
      </math>";
    let target = " <math>
        <mmultiscripts data-chem-formula='3'>
            <mrow data-changed='added' data-chem-formula='3'>
                <mo stretchy='false'>(</mo>
                <mrow data-changed='added'>
                <mi mathvariant='normal' data-chem-element='1'>C</mi>
                <mo data-changed='added'>&#x2063;</mo>
                <mmultiscripts data-chem-formula='1'>
                    <mi mathvariant='normal' data-chem-element='1'>H</mi>
                    <mn>3</mn>
                    <none></none>
                </mmultiscripts>
                </mrow>
                <mo stretchy='false'>)</mo>
            </mrow>
            <mn>3</mn>
            <none></none>
        </mmultiscripts>
    </math>";
    assert!(are_strs_canonically_equal(test, target));
    }

}
