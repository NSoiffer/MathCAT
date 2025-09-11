#![allow(clippy::needless_return)]

// Chemistry terms used here:
// chemical formula -- this references a molecule (one or more elements with bonds between them), including its state.
// chemical equation -- this is a notation specialized to chemistry -- it has concentration, arrows, equality, "addition" along with 
//    some special symbols for operators and (mostly) chemical formulas for operands.
//    Operand exceptions are the equilibrium constant, numbers, and identifiers.
//    Although a chemical equation is a superset of a chemical formula, because we want to distinguish the two (e.g., '=' is in both),
//      we require that chemical equation is an mrow
//    FIX?? -- can it be an adorned mrow?
//    Note: with the current definition, if any element in a potential chem equation is ruled out, the entire mrow is ruled out.
//
// The general flow is that for every element that looks like a chem formula/equation, we mark it with data-likely-[equation/formula]
// After we are done marking "likely", we go back and either delete them or replace them with data-[equation/formula].
// Note: anything already marked with data-[equation/formula] doesn't need recomputation later (essentially the result is cached)
//
// There is a chicken and egg problem with detecting chemistry: to more reliably detect it, we need good structure.
// However, to get the structure right (e.,g "=" being a double bond, not equality; chem elements being in 'mi's; ...),
//   we need to know "=" is part of a chemical formula.
// The imperfect solution used is:
//   As the final step of each recursive call to 'clean_mathml',
//     1. mi/mtext: is it a chemical element(s) or one of the symbols used in chemical formulas (not equations).
//        If so, mark it MAYBE_CHEMISTRY.
//     2. msub/msup/msubsup/mmultiscripts: is base marked MAYBE_CHEMISTRY and the scripts are potential adornments, mark it MAYBE_CHEMISTRY
//     3. mrows: these take a few passes (remember, they aren't structured properly yet)
//        On the assumption that chemistry is not common we implement a "show me" attitude before changing the structure.
//        Pass 1:
//        a) for any run of mi/mtext that can be re-split into chem elements, split them and mark them if it is at least 3 chars long
//        b) if there are any potential chem formula operators (e.g., "=" and ":") and the previous node is marked MAYBE_CHEMISTRY,
//           mark this as MAYBE_CHEMISTRY
//        Pass 2: (assuming something was marked in pass 1)
//        a) find the first marked child and then the last consecutive marked child and trim any mo's from the ends
//        b) evaluate the likelihood that the sequence is chemistry
//           yes: replace mathml children with new (potentially restructured) children
//           no: clear all the marks for the old children
// After canonicalization, we take another pass looking for chemical equations and marking them if found.

use sxd_document::dom::*;
use crate::canonicalize::*;
use crate::pretty_print::mml_to_string;
use crate::xpath_functions::{is_leaf, IsNode};
use regex::Regex;
use crate::xpath_functions::IsBracketed;
use phf::{phf_map, phf_set};
use std::convert::TryInto;
use std::collections::HashSet;
use std::cmp::Ordering;
use crate::errors::*;


pub static NOT_CHEMISTRY: isize = -10000;  // should overwhelm any positive signal
static NOT_CHEMISTRY_THRESHOLD: isize = -10000/2;  // value for testing -- that way some can be added to NOT_CHEMISTRY and still meet the test
static CHEMISTRY_THRESHOLD: isize = 5;   // if this changes, change CHEMISTRY_THRESHOLD_STR


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
static CHEM_STATE: &str = "data-chem-state";

/// mark a new chem element that happened due to splitting a leaf
pub static SPLIT_TOKEN: &str = "data-split";

/// mark a new chem element that happened due to merging two leaves
static MERGED_TOKEN: &str = "data-merged";

/// these can be in the base of an under/over script
static CHEM_EQUATION_ARROWS: phf::Set<char> = phf_set! {
    '→', '➔', '←', '⟶', '⟵', '⤻', '⇋', '⇌',
    '↑', '↓', '↿', '↾', '⇃', '⇂', '⥮', '⥯', '⇷', '⇸', '⤉', '⤈',
    '⥂', '⥄', '⥃',
    '\u{1f8d0}', '\u{1f8d1}', '\u{1f8d2}', '\u{1f8d3}', '\u{1f8d4}', '\u{1f8d5}',         // proposed Unicode equilibrium arrows
};

// Returns true if the 'property' (should have ":") is in the intent
fn has_chem_intent(mathml: Element, property: &str) -> bool {
    if let Some(intent) = mathml.attribute_value(INTENT_ATTR) {
        let head = intent.split('(').next().unwrap();
        return head.contains(property);
    }
    return false;
}

pub fn is_chemistry_off(mathml: Element) -> bool {
    if has_chem_intent(mathml, ":chemical-formula") || has_chem_intent(mathml, ":chemical-equation") {
        return false;
    }
    let pref_manager = crate::prefs::PreferenceManager::get();
    return pref_manager.borrow().pref_to_string("Chemistry") == "Off";
}

pub fn clean_chemistry_mrow(mathml: Element) {
    if is_chemistry_off(mathml) {
        return;
    }
    // debug!("clean_chemistry_mrow:\n{}", mml_to_string(mathml));
    let mut children = mathml.children().iter()
                .map(|child| as_element(*child))
                .collect::<Vec<Element>>();
    if let Some(new_children) = clean_mrow_children_restructure_pass(&children) {
        mathml.replace_children(&new_children);
        children = new_children;
    }
    clean_mrow_children_mark_pass(&children);
}

/// Do some aggressive structural changes and if they make this look like a chemistry formula, mark it as one else remove other marks
/// Note: the element is replaced with a new restructured element if it is marked as chemistry
///        Pass 1:
///        a) for any run of mi/mtext that can be re-split into chem elements, split them and mark them if it is at least 3 chars long.
///           Also split "(g)", etc., when in mi/mtext
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
        } else {
            let child = old_children[i];
            let child_name = name(child);
            if  child_name == "mi" || (child_name == "mtext" && as_text(child).len() < 4) {
                // break mi/mtext that is done as "(g)", etc. Even if it isn't 'g', 'l', etc., it probably shouldn't be an mi/text.
                let text = as_text(child);
                if text.starts_with('(') && text.ends_with(')') {
                    let doc = child.document();
                    let state = create_mathml_element(&doc, "mi");
                    state.set_text(&text[1..text.len()-1]);
                    let open = create_mathml_element(&doc, "mo");
                    open.set_text("(");
                    let close = create_mathml_element(&doc, "mo");
                    close.set_text(")");
                    let mrow = create_mathml_element(&doc, "mrow");
                    mrow.append_children(&[open,state,close]);
                    new_children.push(mrow);
                    i += 1;
                    changed = true;
                    continue;
                }
            } else if i + 2 < old_children.len() {
                // wrap with an mrow if we are not already an 'mrow'
                let parent = get_parent(child); // safe since 'math' is always at root
                if !(name(parent) == "mrow" && i == 0 && old_children.len() == 3) {
                    if let Some(paren_mrow) = make_mrow(old_children[i..i+3].try_into().unwrap()) {
                        // debug!("make_mrow added mrow");
                        new_children.push(paren_mrow);
                        i += 3;
                        changed = true;
                        continue;
                    }
                }
            }
            if child_name == "mo" {
                let likely_chemistry_op = likely_chem_formula_operator(child);
                // debug!("clean_mrow_children_restructure_pass -- in mo: likely {}, {}", likely_chemistry_op, mml_to_string(child));
                if likely_chemistry_op >= 0 {
                    // if possible chemistry to left and right, then override text for operator lookup
                    // note: on the right, we haven't set chem flag for operators yet, so we skip them
                    let preceding = child.preceding_siblings();
                    let following = child.following_siblings();
                    if !preceding.is_empty() && preceding.iter().all(|&child| {
                        let child = as_element(child);
                        name(child)=="mn" || child.attribute(MAYBE_CHEMISTRY).is_some()}) &&
                        !following.is_empty() && following.iter().all(|&child| {
                            let child = as_element(child);
                            name(child)=="mo" || name(child)=="mn" || child.attribute(MAYBE_CHEMISTRY).is_some()
                        }) {
                        // "=", etc., should be treated as high priority separators
                        // debug!("clean_mrow_children_restructure: child = {}", mml_to_string(child));
                        child.set_attribute_value(CHEMICAL_BOND, "true");
                        child.set_attribute_value(CHEM_FORMULA_OPERATOR, &likely_chemistry_op.to_string());
                        child.set_attribute_value(MAYBE_CHEMISTRY, &likely_chemistry_op.to_string());
                    }
                } else {
                    likely_chem_equation_operator(child);   // need to mark MAYBE_CHEMISTRY for CHEMICAL_BOND tests
                }
            } else if child_name == "mrow" {
                if let Some(latex_value) = child.attribute_value("data-latex") {
                    if latex_value == r"\mathrel{\longrightleftharpoons}" {
                        child.set_attribute_value("data-unicode", "\u{1f8d2}");
                        child.set_attribute_value(MAYBE_CHEMISTRY, "2");    // same as is_hack_for_missing_arrows()
                    }
                }               
            }
            i += 1;
            new_children.push(child);
        }
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

    /// Converts  "( child )" to mrow with those elements as children.
    /// This is to make ascertaining whether this is a chemical state easier, but it is correct even if not a chemical state.
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
            if start.is_none() {
                if name(child) == "mo" {
                    // debug!(" start.is_none(): removing MAYBE_CHEMISTRY on {}", as_text(child));
                    child.remove_attribute(MAYBE_CHEMISTRY);
                    child.remove_attribute(CHEM_FORMULA_OPERATOR);
                    child.remove_attribute(CHEM_EQUATION_OPERATOR);
                    child.remove_attribute(CHEMICAL_BOND);
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
        // debug!("  looking for ops at end of {}..{}, last is:{}", start, end, mml_to_string(children[end-1]));
        for stop in (start..end).rev() {
            let end_child = children[stop];
            if name(end_child) == "mo" {
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
    if !(name(mathml) == "mi" || name(mathml) == "mtext") {
        return None;       // do nothing
    }

    // we play games with the string to avoid allocation...
    let token_string = as_text(mathml);
    if !token_string.is_ascii() {
        return None;    // chemical elements are ASCII
    }
    let doc = mathml.document();
    if token_string.len() > 1 {   // safe because all chars are ASCII
        return split_string_chem_element(&doc, mathml);
    }   
    let parent = get_parent(mathml);
    let parent_name = name(parent);
    if !(parent_name == "mrow" || parent_name == "math") {  // not canonicalized yet
        return None;    // only try to merge if in an mrow
    }
    let answer = merge_tokens_chem_element(&doc, mathml, &mathml.following_siblings());
    return answer;


    fn merge_tokens_chem_element<'a>(doc: &Document<'a>, leaf: Element<'a>, following_siblings: &[ChildOfElement<'a>]) -> Option<Vec<Element<'a>>> {
        if following_siblings.is_empty() {
            return None;
        }
        let second_element = as_element(following_siblings[0]);
        let second_element_name = name(second_element);
        if second_element_name != "mi" && second_element_name != "mtext" {
            return None;
        }
        let second_element_text = as_text(second_element);
        if second_element_text.len() != 1 {
            return None;
        }
        let token_string = as_text(leaf);
        let chem_token_string = vec![token_string.as_bytes()[0], second_element_text.as_bytes()[0]];
        if let Some(chem_element) = get_chem_element(doc, &chem_token_string, 2) {
            chem_element.set_text(as_text(chem_element));
            chem_element.set_attribute_value(MAYBE_CHEMISTRY, chem_element.attribute_value(MAYBE_CHEMISTRY).unwrap());
            chem_element.set_attribute_value(MERGED_TOKEN, "true");
            second_element.remove_from_parent();
            return Some(vec![chem_element]);
        }
        return None;
    }

    /// split the string which has been checked to be all ASCII chars
    fn split_string_chem_element<'a>(doc: &Document<'a>, leaf: Element<'a>) -> Option<Vec<Element<'a>>> {
        let token_string = as_text(leaf).as_bytes();
        let token_len = token_string.len();
        let mut j = 0;
        let mut new_children = Vec::with_capacity(token_string.len());
        while j < token_len {
            // try elements of length 2 and 1, preferring longer elements (e.g., prefer "Na" over "N")
            if let Some(chem_element) = get_chem_element(doc, &token_string[j..], 2) {
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
        if new_children.len() <= 1 {
            return None;
        }
        add_attrs(new_children[new_children.len()-1], &leaf.attributes());
        new_children[new_children.len()-1].set_attribute_value(SPLIT_TOKEN, "true");
        // debug!("split_string_chem_element: {} -> {}", String::from_utf8(token_string.to_vec()).unwrap(), new_children.len());
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
        if CHEMICAL_ELEMENT_ELECTRONEGATIVITY.contains_key( chem_element ) {
            return Some( new_chemical_element(doc, chem_element) );
        }

        return None;
    }

    fn new_chemical_element<'a>(doc: &Document<'a>, chem_element_str: &str) -> Element<'a> {
        let result = create_mathml_element(doc, "mi");
        result.set_text(chem_element_str);
        result.set_attribute_value(MAYBE_CHEMISTRY, if chem_element_str.len() == 1 {"1"} else {"3"});
        if chem_element_str.len() == 1 {
            result.set_attribute_value("mathvariant", "normal");
        }
        return result;
    }
}

/// Looks at the children of the element and uses heuristics to decide whether this is a chemical equation/formula
/// If it is, it is marked with either data-chem-equation or data-chem-formula
/// This function assumes proper structure
/// 
/// Returns true if not chemistry -- added attrs, mrows, and leaves are removed in preparation for a second parse
pub fn scan_and_mark_chemistry(mathml: Element) -> bool {
    if is_chemistry_off(mathml) {
        return true;
    }

    let child = as_element(mathml.children()[0]);
    // debug!("scan_and_mark_chemistry:\n{}", mml_to_string(child));
    assert_eq!(name(mathml), "math");
    let is_chemistry = if let Some(latex) = mathml.attribute_value("data-latex") {
        // MathJax v4 includes this really useful info -- if it starts \ce -- we have Chemistry
        // need to determine if it is an equation or a formula
        latex.trim_start().starts_with(r"\ce") 
    } else {
        has_chem_intent(mathml, ":chemical-formula") || has_chem_intent(mathml, ":chemical-equation")
    };

    if is_chemistry || is_chemistry_sanity_check(mathml) {
        assert_eq!(mathml.children().len(), 1);
        let likelihood = likely_chem_formula(child);
        if likelihood >= CHEMISTRY_THRESHOLD || has_chem_intent(mathml, ":chemical-formula") {
            child.set_attribute_value(MAYBE_CHEMISTRY, likelihood.to_string().as_str());
            set_marked_chemistry_attr(child, CHEM_FORMULA);
        }

        if child.attribute(CHEM_FORMULA).is_none() {
            // can't be both an equation and a formula...
            let likelihood = likely_chem_equation(child);
            if is_chemistry || likelihood >= CHEMISTRY_THRESHOLD || has_chem_intent(mathml, ":chemical-equation") {
                child.set_attribute_value(MAYBE_CHEMISTRY, likelihood.to_string().as_str());
                set_marked_chemistry_attr(child, CHEM_EQUATION);
            }
        }
    }
    // debug!("...after marking:\n{}", mml_to_string(child));

    if child.attribute(CHEM_FORMULA).is_none() && child.attribute(CHEM_EQUATION).is_none() {
        if !has_maybe_chemistry(mathml) {
            return true;    // quick check avoids needing a second parse due to removing added elements
        }
        return !is_changed_after_unmarking_chemistry(mathml);
    } else {
        return true;
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
fn set_marked_chemistry_attr(mathml: Element, chem: &str) {
    let tag_name = name(mathml);
    if let Some(maybe_attr) = mathml.attribute(MAYBE_CHEMISTRY) {
        maybe_attr.remove_from_parent();

        match tag_name {
            "mi" | "mtext" => {mathml.set_attribute_value(CHEM_ELEMENT, maybe_attr.value());},
            "mo" => {
                if mathml.attribute(CHEM_FORMULA_OPERATOR).is_none() && mathml.attribute(CHEM_EQUATION_OPERATOR).is_none(){
                    // don't mark as both formula and equation
                    mathml.set_attribute_value(if chem == CHEM_FORMULA {CHEM_FORMULA_OPERATOR} else {CHEM_EQUATION_OPERATOR}, maybe_attr.value());
                }
            },
            "mn" => (),
            "mrow" | "msub" | "msup" | "msubsup" | "mmultiscripts" => {
                let mut chem_name = chem;
                if tag_name != "mrow" && chem != CHEM_FORMULA{
                    // look at base -- if an mi/mtext then this is really a chemical formula
                    let base = as_element(mathml.children()[0]);
                    let base_name = name(base);
                    if base_name == "mi" || base_name == "mtext" {
                        chem_name = CHEM_FORMULA;
                    }
                }

                if mathml.attribute(CHEM_FORMULA).is_none() {
                    // don't mark as both formula and equation
                    mathml.set_attribute_value(chem_name, maybe_attr.value());
                }
                for child in mathml.children() {
                    set_marked_chemistry_attr(as_element(child), chem);
                };
            }
            "mfrac" => {
                let children = mathml.children();
                // debug!("mfrac children: {}", mml_to_string(mathml));
                let numerator_is_chem_equation = IsBracketed::is_bracketed(as_element(children[0]), "[", "]", false, true);
                let denominator_is_chem_equation = IsBracketed::is_bracketed(as_element(children[1]), "[", "]", false, true);
                if  numerator_is_chem_equation && denominator_is_chem_equation {
                    mathml.set_attribute_value(CHEM_EQUATION, "true");
                }
            }
            _ => error!("Internal error: {tag_name} should not be marked as 'MAYBE_CHEMISTRY'"),
        }
    } else if tag_name == "mrow" {
        // could have been added during canonicalization, so never marked. Recurse to the children
        for child in mathml.children() {
            set_marked_chemistry_attr(as_element(child), chem);
        };
    }
}

/// returns true if MAYBE_CHEMISTRY's occur within the element
fn has_maybe_chemistry(mathml: Element) -> bool {
    if mathml.attribute(MAYBE_CHEMISTRY).is_some() {
        return true;
    }
    if !is_leaf(mathml) {
        for child in mathml.children() {
            if has_maybe_chemistry(as_element(child)) {
                return true;
            }
        }
    }
    return false;
}

/// Clears MAYBE_CHEMISTRY from this element and its decedents
/// Also deletes added mrows and leaves; returns true if anything is deleted
fn is_changed_after_unmarking_chemistry(mathml: Element) -> bool {
    mathml.remove_attribute(MAYBE_CHEMISTRY);
    if is_leaf(mathml) {
        // don't bother testing for the attr -- just remove and nothing bad happens if they aren't there
        mathml.remove_attribute(CHEM_FORMULA_OPERATOR);
        mathml.remove_attribute(CHEM_EQUATION_OPERATOR);
        mathml.remove_attribute(CHEMICAL_BOND);
        if mathml.attribute(MERGED_TOKEN).is_some() {
            unmerge_element(mathml);
            return true;    // need to re-parse
        } else if mathml.attribute(SPLIT_TOKEN).is_some() {
            if let Err(err) = merge_element(mathml) {
                panic!("{}", err);
            }
            // debug!("After merge_element:{}", mml_to_string(mathml));
            // let parent = get_parent(mathml);
            // debug!("After merge_element: -- parent{}", mml_to_string(parent));

        } else if let Some(changed_value) = mathml.attribute_value(CHANGED_ATTR) {
            if changed_value == ADDED_ATTR_VALUE {
                mathml.remove_from_parent();
                return true;
            }
        }
        return false;
    } else if IsNode::is_scripted(mathml) &&
              name(as_element(mathml.children()[0])) == "mi" &&
              as_element(mathml.children()[0]).attribute(SPLIT_TOKEN).is_some() {
        // Undo a split that happened in a scripted element.
        // We put the preceding elements into the base and call merge_element on the last element of the base
        // The first and/or the last child in the sequence could be a script that needs to be unwrapped
        let mut parent = get_parent(mathml);   // there is always a "math" node
        // debug!("mathml:\n{}", mml_to_string(mathml));
        // debug!("parent before merge:\n{}", mml_to_string(parent));
        // debug!("grandparent before merge:\n{}", mml_to_string(get_parent(parent)));

        let mut preceding_children = mathml.preceding_siblings();
        // could be no preceding children to canonicalization creating mrows (see issue #303), so might need to use parent, etc
        while preceding_children.is_empty() {
            preceding_children = parent.preceding_siblings();
            parent = get_parent(parent);
            if name(parent) == "math" {
                // this shouldn't happen -- rather than crash, let's do something
                error!("is_changed_after_unmarking_chemistry: error no preceding children to merge. mathml=\n{}", mml_to_string(mathml));
                return false;
            }
        }

        // deal with the first element (if it needs unwrapping, it has only prescripts)
        let first_element_of_split = as_element(preceding_children[preceding_children.len()-1]);
        if name(first_element_of_split) == "mmultiscripts" {
            // take the base and make it the first child of preceding_children (what will get merged)
            // put the rest of the elements (the prescripts) at the end of the parent last element (mathml) which must be an mmultiscripts
            let first_element_children = first_element_of_split.children();
            assert_eq!(name(mathml), "mmultiscripts");
            let mut script_children = mathml.children();
            assert_eq!(script_children.len() % 2, 1);  // doesn't have <mprescripts/>
            preceding_children[0] = first_element_children[0];
            script_children.push(first_element_children[1]);
            script_children.push(first_element_children[2]);
            script_children.push(first_element_children[3]);
            mathml.replace_children(script_children);
            first_element_of_split.remove_from_parent();
        }
        let mut children_of_script = mathml.children();
        let split_child = as_element(children_of_script[0]);
        let mut new_script_children = vec![ChildOfElement::Element(first_element_of_split)];
        new_script_children.append(&mut children_of_script);
        mathml.replace_children(new_script_children);     // temporarily has bad number of children 
        // debug!("After making bad script:\n{}", mml_to_string(mathml));
        if let Err(err) = merge_element(split_child) {
            panic!("{}", err);
        }
        return true;
    } else {
        let mut answer = false;
        for child in mathml.children() {
            let child = as_element(child);
            if name(child) == "mtd" && child.attribute(MAYBE_CHEMISTRY).is_some() {
                answer = true;  // each mtd acts as a potential island for chemistry, so don't clear it
            } else {
                answer |= is_changed_after_unmarking_chemistry(child);
            }
        }
        if name(mathml) == "mrow" {
            if let Some(changed_value) = mathml.attribute_value(CHANGED_ATTR) {
                // we added an mrow, we can remove it -- but this might be already processed which is the case if "data-id-added" is true (exists)
                if changed_value == ADDED_ATTR_VALUE && mathml.attribute("data-id-added").is_none() {
                    // mrows get added for several reasons. One of them is to canonicalize elements like msqrt that can have 1 or more children;
                    //   those should not get removed because the re-parse doesn't add those
                    // Although they would never be added, elements with fixed number of children also shouldn't have the mrow go away
                    // We are left with only removing mrows with one child or mrows that are children of mrows (simpler test than ELEMENTS_WITH_ONE_CHILD)
                    let parent = get_parent(mathml);   // mathml is mrow, so parent always exists
                    if mathml.children().len() == 1 || name(parent) == "mrow" {
                        let children = mathml.children().iter().map(|&el| as_element(el)).collect::<Vec<Element>>();                        // debug!("is_changed_after_unmarking: before replace - mathml\n{}", mml_to_string(mathml));
                        mathml.remove_attribute(CHANGED_ATTR);  // if just one child, the attrs are pushed onto the child
                        // debug!("is_changed_after_unmarking: before replace - parent\n{}", mml_to_string(parent));
                        replace_children(mathml, children);
                        // debug!("is_changed_after_unmarking: parent\n{}", mml_to_string(parent));

                    }
                }
            }
            return true;
        }
        return answer;
    }

    fn unmerge_element(mathml: Element) {
        // a merged token occurs when two single letters get merged into one. Here we recreate the two tokens
        assert!(is_leaf(mathml));
        // debug!("unmerge_element: {}", mml_to_string(mathml));
        let mut token_str = as_text(mathml).chars();
        let first = create_mathml_element(&mathml.document(), name(mathml));
        first.set_text(&token_str.next().unwrap().to_string());
        let second = create_mathml_element(&mathml.document(), name(mathml));
        second.set_text(&token_str.next().unwrap().to_string());
        replace_children(mathml, vec![first, second]);
    }

    /// Put the split pieces back together (undo the split)
    fn merge_element(mathml: Element) -> Result<()> {
        // debug!("merge_element: {}", mml_to_string(mathml));
        // debug!("merge_element parent: {}", mml_to_string(get_parent(mathml)));
        assert!(is_leaf(mathml));
        let mut preceding_children = mathml.preceding_siblings();
        // debug!("preceding_children: {}", preceding_children.iter().map(|&el| name(as_element(el)).to_string()).collect::<Vec<String>>().join(", "));
        if preceding_children.is_empty() {
            // handle:
            // * case where we have mi mmultiscripts mi ... where the second mi needs to join with the first (see test mhchem_so4)
            // * case where the child got buried in an added mrow (can only happen one level deep because invisible times should get inserted)
            let parent = get_parent(mathml);   // mathml is leaf, so parent always exists
            preceding_children = parent.preceding_siblings();
            if preceding_children.is_empty() ||
               !(name(parent) == "mmultiscripts" ||
                (name(parent) == "mrow" && parent.attribute_value(CHANGED_ATTR).is_some() &&
                 parent.attribute_value(CHANGED_ATTR).unwrap() == ADDED_ATTR_VALUE)) {
                    bail!("Internal error: {} should not have been split'", mml_to_string(mathml));
            }
        }
        // Note: there was an invisible U+2063, but it was removed before we got here
        // The parent mrow could have many children that couldn't have been part of a split -- only consider feasible children to split (mi/mtext)
        // To figure this out, we walk backwards adding the text in reverse and then reverse that text in the end
        let mut merged_text = Vec::default();
        for &child in preceding_children.iter().rev() {
            let child = as_element(child);
            // because this is before canonicalization, there could be an mrow with just mi/mtext
            if name(child) == "mrow" && child.children().len() == 1 && child.attribute(INTENT_ATTR).is_none() {
                // "lift" the child up so all the links (e.g., siblings) are correct
                let child = as_element(child.children()[0]);
                set_mathml_name(child, name(child));
                crate::canonicalize::add_attrs(child, &child.attributes());
                child.replace_children(child.children());
            }
            if name(child) != "mi" && name(child) != "mtext" {
                break;
            }
            merged_text.push(as_text(child));
            child.remove_from_parent();
        }
        merged_text.reverse();
        let mut merged_text = merged_text.join("");
        merged_text.push_str(as_text(mathml));
        mathml.set_text(&merged_text);
        mathml.remove_attribute("mathvariant");
        mathml.remove_attribute(ADDED_ATTR_VALUE);
        mathml.remove_attribute(MAYBE_CHEMISTRY);
        mathml.remove_attribute(SPLIT_TOKEN);
        return Ok( () );
    }
}

/// Returns true only if 'mathml' potentially is chemistry.
/// This assumes canonicalization has happened and that 'mathml' is the 'math' element
fn is_chemistry_sanity_check(mathml: Element) -> bool {
    // This does some sanity checking. More can definitely be done
    // Checks:
    // * there should be chemical elements
    // * if the child is an mrow with three children, the operator should be '=' (not CHEMICAL_BOND) or  an arrow
    //   in this case, we gather up the elements on the lhs and rhs. The sets should be equal and non-empty.
    //   the exception is if there are prescripts, in which as we might have radioactive decay so we don't require the sets to be equal
    // * otherwise, we gather up all the chemical elements and make sure the set is non-empty
    // * if it isn't an mrow, we leave it to likely_chem_equation() to rule it out
    assert_eq!(name(mathml), "math");
    assert_eq!(mathml.children().len(), 1);
    let mathml = as_element(mathml.children()[0]);
    if name(mathml) == "mrow" {
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
        if name(base) != "mo" || mathml.attribute(CHEMICAL_BOND).is_some() {
            return false;
        }
        let text = as_text(base);
        return text.len() == 1 && (text == "=" || CHEM_EQUATION_ARROWS.contains(&text.chars().next().unwrap()));

    }

    fn gather_chemical_elements<'a>(mathml: Element<'a>, chem_elements: &mut HashSet<&'a str>) -> bool {
        match name(mathml) {
            "mi" | "mtext" => {
                // debug!("gather_chemical_elements: {}", mml_to_string(mathml));
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
    // mfrac -- could be a ratio of concentrations
    if name(mathml) != "mrow" && name(mathml) != "mtd" && name(mathml) != "mfrac" {
        return NOT_CHEMISTRY;
    }

    // debug!("start likely_chem_equation:\n{}", mml_to_string(mathml));
	// mrow -- check the children to see if we are likely to be a chemical equation

    // concentrations should either be unscripted or have a superscript that isn't a charge
    // they occur in mrows or mfracs
    if IsBracketed::is_bracketed(mathml, "[", "]", false, true) {
        let parent_name = name(get_parent(mathml));
        if parent_name == "mfrac" || parent_name == "mrow"  || parent_name == "math" || 
           (parent_name == "msup" && likely_chem_superscript(as_element(mathml.following_siblings()[0])) < 0){
            return if as_element(mathml.children()[0]).attribute(CHEM_FORMULA).is_some() {CHEMISTRY_THRESHOLD}  else {NOT_CHEMISTRY};
        }
    }
    
    // possible improvement -- give bonus points for consecutive (not counting invisible separators) chemical elements on top of the existing points
	let mut likelihood = 0;						// indicator of likely match
	let mut has_equilibrium_constant = false;
    let children = mathml.children();
	for i in 0..children.len() {
		let child = as_element(children[i]);
        // debug!("   i={}, likelihood={}, child={}", i, likelihood, crate::canonicalize::element_summary(child));
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
        let tag_name = name(child);
        let likely = match tag_name {
            "mi" => likely_chem_element(child),
            "mn" => 0,       // not much info
            "mo" | "mover" | "munder" | "munderover" =>  likely_chem_equation_operator(child),
            "msub" | "msup" | "msubsup" | "mmultiscripts" => {
                if is_equilibrium_constant(child) {
                    has_equilibrium_constant = true;
                    2
                } else {
                    likely_adorned_chem_formula(child)
                }
            },
            "mfrac" => {
                if has_equilibrium_constant {
                    2
                } else {
                    -3    // fraction tend only to appear after an equilibrium constant
                }
            },
            "mrow" => {
                let likely = likely_chem_formula(child);
                if likely < 0 {
                    likely_chem_equation(child)
                } else {
                    likely
                }     
            },
            // no need to check for mtr or mtd because they only exist in a table and the recursion is dealt with here.
            "mtable" => {
                for mrow in child.children() {
                    let mrow = as_element(mrow);
                    for mtd in mrow.children() {
                        let mtd = as_element(mtd);
                        let mut likely = likely_chem_formula(mtd);
                        if likely < CHEMISTRY_THRESHOLD {
                            likely = likely_chem_equation(mtd);
                        }     
                        if likely < CHEMISTRY_THRESHOLD {
                            is_changed_after_unmarking_chemistry(mtd);
                        }     
                    }
                }
                NOT_CHEMISTRY
            },
            "semantics" => {
                likely_chem_equation(get_presentation_element(mathml).1)
            },
            _ => NOT_CHEMISTRY,
        };
        if likely >= 0 {
            child.set_attribute_value(MAYBE_CHEMISTRY, &likely.to_string());
        }
        likelihood += likely;
        if likelihood < NOT_CHEMISTRY_THRESHOLD {
            return NOT_CHEMISTRY;
        }
    }

    if likelihood >= 0 {
        mathml.set_attribute_value(MAYBE_CHEMISTRY, &likelihood.to_string());
    }
    return likelihood;
}


/// could be a number, a state ("(l)", "(g)", etc), or a number followed by a state
fn likely_chem_subscript(subscript: Element) -> isize {
    let subscript_name = name(subscript);
    if  subscript_name == "mn" && !as_text(subscript).contains('.') {
        return 0;       // not really much chem info about an integer subscript
    } else if subscript_name == "mi" {
        let text = as_text(subscript);
        if text == "s" || text == "l" ||text == "g" ||text == "aq" {
            subscript.set_attribute_value(CHEM_STATE, "true");
            return 2;
        }
    } else if subscript_name == "mrow" {
        // debug!("likely_chem_subscript:\n{}", mml_to_string(subscript));
        let children = subscript.children();
        if children.len() == 3 && IsBracketed::is_bracketed(subscript, "(", ")", false, true) {
            return likely_chem_subscript(as_element(children[1]));
        }
        let i_first_child = as_element(children[0]);
        if children.len() == 2 &&
           name(i_first_child) == "mn" && !as_text(i_first_child).contains('.') &&
           name(as_element(children[1])) == "mrow" &&
           likely_chem_state(as_element(children[1])) > 0 { // notation used in en.wikipedia.org/wiki/Electrolyte#Formation
                return 2;
        }     
    }
    // could be a variable 'n' or something else -- just not likely
    return -3
}

fn small_roman_to_number(text: &str) -> &str {
    // simplest to do a look up
    static ROMAN_TO_NUMBER: phf::Map<&str, &str> = phf_map! {
        "I" => "1", "II" => "2", "III" => "3", "IV" => "4", "V" => "5", "VI" => "6", "VII" => "7", "VIII" => "8", "IX" => "9",
    };
    return ROMAN_TO_NUMBER.get(text).unwrap_or(&"");

}

fn likely_chem_superscript(sup: Element) -> isize {
    // either one or more '+'s (or '-'s) or a number followed by +/-
    // also could be state (en.wikipedia.org/wiki/Nuclear_chemistry#PUREX_chemistry)
    // bullet is radical (en.wikipedia.org/wiki/Radical_(chemistry)#Depiction_in_chemical_reactions); mhchem uses dot operator
    //  these can stand alone, be followed by +/- or have a number in front "(2•)-"" [examples from mhchem documentation]
    // roman numerals are "oxidation state" and range from -4 to +9
    lazy_static! {
        static ref MULTIPLE_PLUS_OR_MINUS_OR_DOT: Regex = Regex::new(r"^\++$|^-+$|^\U{2212}+$|^[⋅∙•][-+\U{2212}]*$").unwrap();
        static ref SINGLE_PLUS_OR_MINUS_OR_DOT: Regex = Regex::new(r"^[+-\U{2212}⋅∙•]$").unwrap();
    }
    static DOTS: &[char; 3] = &['⋅', '∙', '•'];
    let sup_name = name(sup);
    if sup_name == "mo" && MULTIPLE_PLUS_OR_MINUS_OR_DOT.is_match(as_text(sup)) {
        if as_text(sup).find(DOTS).is_some() {
            sup.set_attribute_value(MAYBE_CHEMISTRY, "1");
            sup.set_attribute_value(CHEM_FORMULA_OPERATOR, "1");   // value doesn't really matter
        }
        return if as_text(sup).len()==1 {1} else {2};
    } else if (sup_name == "mi" || sup_name == "mn" || sup_name=="mtext") && SMALL_UPPER_ROMAN_NUMERAL.is_match(as_text(sup)){
        sup.set_attribute_value("data-number", small_roman_to_number(as_text(sup)));
        sup.set_attribute_value(MAYBE_CHEMISTRY, "2");
        return 2;
    } else if sup_name == "mrow" {
        // look for something like '2+'
        let children = sup.children();
        if children.len() == 2 {
            let first = as_element(children[0]);
            let second = as_element(children[1]);
            if name(first) == "mn" && name(second) == "mo" && !as_text(first).contains('.') {
                let second_text = as_text(second);
                if SINGLE_PLUS_OR_MINUS_OR_DOT.is_match(second_text) {
                    if second_text.find(DOTS).is_some() {
                        second.set_attribute_value(MAYBE_CHEMISTRY, "2");
                        second.set_attribute_value(CHEM_FORMULA_OPERATOR, "2");   // value doesn't really matter
                    }
                    sup.set_attribute_value(MAYBE_CHEMISTRY, "3");
                    return 3;   // ending with a +/- makes it likely this is an ion
                }
            }
        }
        // gather up the text and see if it is all +, -, etc
        let mut text = "".to_string();
        for child in &children {    // 'children' used later, so need to borrow rather than move
            let child = as_element(*child);
            if name(child) == "mo" {
                text.push_str(as_text(child));
            } else {
                // could have something like 'mrow(mrow 2n, -)  (chem example 5-9) -- so fallback to still ok if ends with + or -
                let last_super_child = as_element(children[children.len()-1]);
                if name(last_super_child) == "mo" {
                    let text = as_text(last_super_child);
                    if text == "+" || text == "-" {
                        sup.set_attribute_value(MAYBE_CHEMISTRY, "3");
                        return 3;
                    }
                }
                return NOT_CHEMISTRY;
            }
        }
        if MULTIPLE_PLUS_OR_MINUS_OR_DOT.is_match(&text) {
            for child in children {
                let child = as_element(child);
                if name(child) == "mo" && as_text(child).find(DOTS).is_some() {
                    child.set_attribute_value(MAYBE_CHEMISTRY, "1");
                    child.set_attribute_value(CHEM_FORMULA_OPERATOR, "1");   // value doesn't really matter
                }
            }
            let likely = 2*text.len() as isize;
            sup.set_attribute_value(MAYBE_CHEMISTRY, &likely.to_string());
            return likely;
        }
    }
    return NOT_CHEMISTRY
}


/// chem_formula is likely if it is one of:
/// * a (possibly adorned) chemical element
/// * an operator that represents a bond
/// * fences around a chemical formula
/// * an mrow made up of only chemical formulas
fn likely_chem_formula(mathml: Element) -> isize {
    // debug!("start likely_chem_formula:\n{}", mml_to_string(mathml));
    if let Some(value) = get_marked_value(mathml) {
        return value;       // already marked
    }

    let tag_name = name(mathml);
    let likelihood = match tag_name {
        // a parent may clear the chem flags if something says can't be chemistry (e.g, a non chemically valid script)
        "mi" => likely_chem_element(mathml),
        "mo" => likely_chem_formula_operator(mathml),
        "mtext" => 0,    // definitely need to skip empty mtext, but others are probably neutral also
        "mn" => 0,       // no info
        "msub" | "msup" | "msubsup" | "mmultiscripts" => {
            likely_chem_formula(as_element(mathml.children()[0]));  // set MAYBE_CHEMISTRY attribute
            likely_adorned_chem_formula(mathml)
        },
        "mrow" => {
            let chem_state = likely_chem_state(mathml);
            if chem_state > 0 {
                chem_state
            } else {
                likely_mrow_chem_formula(mathml)
            }
        },
        "mfrac" => {
            let children = mathml.children();
            let num_likely = likely_chem_formula(as_element(children[0]));
            let denom_likely = likely_chem_formula(as_element(children[1]));
            let likely = num_likely.max(denom_likely);
            if likely < CHEMISTRY_THRESHOLD {NOT_CHEMISTRY} else {likely}
        }
        "mtd" => {
            let mut likely = likely_chem_formula(as_element(mathml.children()[0]));
            if likely < CHEMISTRY_THRESHOLD {
                likely = likely_chem_equation(mathml);
            }
            likely
        }
        "mtable" => {
            for mrow in mathml.children() {
                let mrow = as_element(mrow);
                for mtd in mrow.children() {
                    let mtd = as_element(mtd);
                    let mut likely = likely_chem_formula(mtd);
                    if likely < CHEMISTRY_THRESHOLD {
                        likely = likely_chem_equation(mtd);
                    }     
                    if likely < CHEMISTRY_THRESHOLD {
                        is_changed_after_unmarking_chemistry(mtd);
                    }     
                }
            }
            NOT_CHEMISTRY
        },
        "semantics" => {
            likely_chem_formula(get_presentation_element(mathml).1)
        },
        _ => {
            if !is_leaf(mathml) {
                // mfrac, msqrt, etc
                for child in mathml.children() {
                    let child = as_element(child);
                    let likelihood = likely_chem_formula(child);
                    if  likelihood > 0 {
                        child.set_attribute_value(MAYBE_CHEMISTRY, likelihood.to_string().as_str());
                    };
                }
            }
            // debug!("NOT_CHEMISTRY:\n{}", mml_to_string(mathml));
            NOT_CHEMISTRY
        }
    };
    if likelihood >= 0 {
        mathml.set_attribute_value(MAYBE_CHEMISTRY, &likelihood.to_string());
    }
    // debug!("likely_chem_formula {}:\n{}", likelihood, mml_to_string(mathml));

    return likelihood;

    fn likely_mrow_chem_formula(mrow: Element) -> isize {
        // For parens, the only reason to add them is to group the children and then indicate that there is more than one molecule
        if IsBracketed::is_bracketed(mrow, "(", ")", false, false) ||
           IsBracketed::is_bracketed(mrow, "[", "]", false, false) {
            // If it is bracketed, it should have a subscript to indicate the number of the element.
            // We give a pass to unadorned bracketing chars
            if mrow.children().len() != 3 {
                return NOT_CHEMISTRY;
            }
            let contents = as_element(mrow.children()[1]);
            let parent = get_parent(mrow);
            let parent_is_scripted = IsNode::is_scripted(parent);
            if name(contents) != "mrow" && !parent_is_scripted {
                return NOT_CHEMISTRY;
            }
            let likely = likely_chem_formula(contents);
            if parent_is_scripted {
                return likely + 3;
            } else {
                return likely;
            }
        }

        let mut likelihood = if is_order_ok(mrow) {0} else {-4};

        // check all the children and compute the likelihood of that this is a chemical formula
        // bonus point for consecutive chemical formula children (not counting invisible children)
        let mut last_was_likely_formula = 0;        // 0 is false, 1 is true
        let mut is_chem_formula = true;              // assume true until we prove otherwise (still want to mark the children)
        for child in mrow.children() {
            let child = as_element(child);
            let likely = likely_chem_formula(child);
            // debug!("   in mrow: likely={}, likelihood={}", likely, likelihood);
            match likely.cmp(&0) {
                Ordering::Greater => { 
                    likelihood += likely + last_was_likely_formula;
                    last_was_likely_formula = if name(child) == "mo" {0} else {1};
                },
                Ordering::Less => {
                    // debug!("in likely_chem_formula: FALSE: likelihood={}, child\n{}", likelihood, mml_to_string(child));
                    is_chem_formula = false;
                    last_was_likely_formula = 0;
                    likelihood += likely;
                },
                Ordering::Equal => {
                    if name(child) == "mo" {
                        let text = as_text(child);
                        if text != "\u{2062}" && text != "\u{2063}" {   // one of these, we don't change the status
                            last_was_likely_formula = 0;
                        }
                    }
                },
            }
            // debug!("in likely_chem_formula likelihood={}, child\n{}", likelihood, mml_to_string(child));
            // debug!("   likelihood={} (likely={})", likelihood, likely);
        }

        if !is_chem_formula || likelihood <= NOT_CHEMISTRY {
            // the children may have looked have looked right, but something has said "not likely"
            return NOT_CHEMISTRY;
        } else if likelihood < CHEMISTRY_THRESHOLD && is_short_formula(mrow) {
                    // debug!("is_short_formula is true for:\n{}", mml_to_string(mrow));
                    return CHEMISTRY_THRESHOLD
        }
        return likelihood;
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
/// Note: "(OH)" doesn't fit with the above, and Susan Jolly suggests allowing any sequence that ends with H, so we allow that.
/// Also, Susan Jolly suggested allowing any compound with C, H, and O
fn is_order_ok(mrow: Element) -> bool {
    assert_eq!(name(mrow), "mrow");
    if let Some(elements) = collect_elements(mrow) {
        if elements.iter().any(|&e| !CHEMICAL_ELEMENT_ELECTRONEGATIVITY.contains_key(e)) {
            return false;
        }
        let n_elements = elements.len();
        if n_elements < 2 {
            return true;
        } else if has_noble_element(&elements) {
            return false;    // noble elements don't form compounds
        } else {
            return elements[n_elements-1] == "H"   ||        // special case that includes "OH"
                    // has_non_metal_element(&elements) && !has_non_metal_element(&elements) &&    // must have a metal and non-metal
                    has_c_h_o(&elements) ||
                    is_structural(&elements) ||
                    is_alphabetical(&elements) ||
                    is_ordered_by_electronegativity(&elements) ||
                    is_generalized_salt(&elements);
        }
    } else {
        return false;
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

fn has_c_h_o(elements: &[&str]) -> bool {
    return elements.contains(&"C") && elements.contains(&"H") && elements.contains(&"O");
}


fn is_structural(elements: &[&str]) -> bool {
    assert!(!elements.len() > 1);   // already handled

    // debug!("is_structural: {:?}", elements);
    let mut element_set = HashSet::with_capacity(elements.len());
    elements.iter().for_each(|&e| {element_set.insert(e);});
    return element_set.len() < elements.len();
}

/// collect up all the elements in the mrow.
///  Returns the elements (which can be an empty vector) or None if something (right now an operator) rules out them being elements
fn collect_elements(mrow: Element<'_>) -> Option<Vec<&str>> {
    let mut elements = Vec::with_capacity(mrow.children().len()/2+1);       // don't bother with slots for operators
    for child in mrow.children() {
        let child = as_element(child);
        match name(child) {
            "mi" | "mtext" => elements.push(as_text(child)),
            "msub" | "msup" | "mmultiscripts" => {
                let base = as_element(child.children()[0]);
                let base_name = name(base);
                if base_name == "mi" || base_name == "mtext" {
                    elements.push(as_text(base));
                }   // else skip and let recursive likely_chem_formula call check the contents
            },
            "mo" => {
                if likely_chem_formula_operator(child) < 0 {
                    return None;
                }
            },
            _ => (),    // let loop in likely_chem_formula() deal with all the negatives
        }
    }
    return Some(elements);
}

/// check to make sure elements are ordered alphabetically
/// Actually check Hill's system that puts 'C' followed by 'H' first if 'C' is present
fn is_alphabetical(elements: &[&str]) -> bool {
    assert!(!elements.len() > 1);   // already handled
    // debug!("is_alphabetical: {:?}", elements);
    let mut elements = elements;
    if elements[1..].contains(&"C") {  // "C" must be first if present
        return false;
    }
    if elements[0] == "C" {
        elements = if elements[1]=="H" {&elements[2..]} else {&elements[1..]};
    }
    return elements.len() < 2 || elements.windows(2).all(|pair| pair[0] < pair[1]);
}

fn is_ordered_by_electronegativity(elements: &[&str]) -> bool {
    // HPO_4^2 (Mono-hydrogen phosphate) doesn't fit this pattern, nor does HCO_3^- (Hydrogen carbonate) and some others
    // FIX: drop "H" from the ordering??
    assert!(!elements.len() > 1);   // already handled
    return elements.windows(2).all(|pair| CHEMICAL_ELEMENT_ELECTRONEGATIVITY.get(pair[0]).unwrap() < CHEMICAL_ELEMENT_ELECTRONEGATIVITY.get(pair[1]).unwrap());
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
///
/// Note: 'mathml' is not necessarily canonicalized   
pub fn likely_adorned_chem_formula(mathml: Element) -> isize {
    // some simple sanity checks on the scripts...
    let tag_name = name(mathml);
    let children = mathml.children();
    let mut likelihood = 0;
    let mut is_empty_subscript = false;
    // debug!("likely_adorned_chem_formula:\n{}", mml_to_string(mathml));
    if tag_name == "msub" || tag_name == "msubsup" {
        // subscripts should be just a number, although they could be 'n' or '2n' or other exprs.
        let subscript = as_element(children[1]);
        is_empty_subscript = name(subscript) == "mtext" && as_text(subscript).trim().is_empty();
        if !is_empty_subscript {
            likelihood += likely_chem_subscript(subscript);
        }
    }

    let mut empty_superscript = false;
    if tag_name == "msup" || tag_name == "msubsup" {
        // debug!("likely_adorned_chem_formula: mathml\n{}", mml_to_string(mathml));
        let superscript = as_element(children[if tag_name == "msup" {1} else {2}]);
        empty_superscript = name(superscript) == "mtext" && as_text(superscript).trim().is_empty();
        if !empty_superscript {
            likelihood += likely_chem_superscript(superscript);
        }
    }
    if tag_name == "msubsup" && (is_empty_subscript || empty_superscript) {
        likelihood += 1; // might be trying to vertically align scripts as in done in chemistry
    }

    if tag_name == "mmultiscripts" {
        // prescripts are normally positive integers, chem 2.5.1 allows for a superscript for a Lewis dot
        // postscript should be a charge

        let prescripts;
        let postscripts;
        if children.len() == 4 && name(as_element(children[1]))=="mprescripts" { // just prescripts
            prescripts = &children[2..4];
            postscripts = &children[0..0]; // empty
        } else if children.len() == 6 && name(as_element(children[3]))=="mprescripts" {  // pre and postscripts
            prescripts = &children[4..6];
            postscripts = &children[1..3]; // empty
        } else if children.len() == 3 || children.len() == 5 {   // just postscripts (simultaneous or offset)
            prescripts = &children[0..0]; // empty
            postscripts = &children[1..];
        } else {
            return NOT_CHEMISTRY;
        };

        if !prescripts.is_empty() {
            let pre_subscript = as_element(prescripts[0]);
            let pre_subscript_name = name(pre_subscript);

            let pre_superscript = as_element(prescripts[1]);
            let pre_superscript_name = name(pre_superscript);

            // deal with special case of 'e' with prescripts of -1 and 0
            if is_adorned_electron(children[0], prescripts) {
                return 100;     // very likely chemistry
            }
            
            if pre_superscript_name == "mo" {
                // Lewis dot prescript case
                if pre_subscript_name != "none" {
                    return NOT_CHEMISTRY;
                }
                likelihood += likely_chem_superscript(pre_superscript);
            } else if pre_superscript_name == "mn" { // must have a pre-superscript (neutrons + protons)
                // fix could make sure they are integers
                likelihood += 1;        // looking like an atomic number                
                if pre_subscript_name == "mn" {
                    // make sure the atomic number matches the base
                    let base = as_element(children[0]);
                    let base_name = name(base);
                    if base_name == "mi" || base_name == "mtext" {
                        if let Some(atomic_number) = CHEMICAL_ELEMENT_ATOMIC_NUMBER.get(as_text(base)) {
                            if as_text(pre_subscript) == atomic_number.to_string() {
                                likelihood = CHEMISTRY_THRESHOLD;
                            }
                        }
                    }
                }
            } else {
                return NOT_CHEMISTRY;
            }
        }

        if !postscripts.is_empty() {
            let mut i = 0;
            while i < postscripts.len() {
                let sub = as_element(postscripts[i]);
                // debug!("sub: {}", mml_to_string(sub));
                if name(sub) != "none" {
                    likelihood += likely_chem_subscript(sub);
                } 
                let sup = as_element(postscripts[i+1]);
                if name(sup) != "none" {
                    // debug!("sup: {}", mml_to_string(sub));
                    likelihood += likely_chem_superscript(sup);
                }
                i += 2;
            }
        }
    }

    let base = as_element(children[0]);
    let base_name = name(base);
    if base_name == "mi" || base_name == "mtext" {
        likelihood += likely_chem_element(base);
    } else if base_name == "mrow" {
        // debug!("mrow addition:\n{}", mml_to_string(base));
        // a safe minor canonicalization that allows "short_form" calculations if appropriate
        if (IsBracketed::is_bracketed(base, "(", ")", false, false) ||
            IsBracketed::is_bracketed(base, "[", "]", false, false)) &&
           base.children().len() > 3 {
            let inner_mrow = create_mathml_element(&base.document(), "mrow");
            inner_mrow.set_attribute_value(CHANGED_ATTR, ADDED_ATTR_VALUE);
            let mut children = base.children();
            let inside_of_parens = children.drain(1..children.len()-1);
            inner_mrow.append_children(inside_of_parens);
            base.replace_children(vec![children[0], ChildOfElement::Element(inner_mrow), children[children.len()-1]]);
        }
        likelihood += likely_chem_formula(base);
    } else {
        likelihood += likely_chem_formula(base);
    }
    
    return likelihood;


    fn is_adorned_electron(base: ChildOfElement, prescripts: &[ChildOfElement]) -> bool {
        // looking for 'e' with prescripts of -1 and 0
        let base = as_element(base);
        let pre_lower = as_element(prescripts[0]);
        let pre_upper = as_element(prescripts[1]);
        if (name(base) == "mi" || name(base) == "mtext") && as_text(base) == "e" &&
           name(pre_upper) == "mn" && as_text(pre_upper) == "0" && 
           name(pre_lower) == "mrow" && pre_lower.children().len() == 2 {
            // looking '-' and '1'
            let lower_children = pre_lower.children();
            let minus = as_element(lower_children[0]);
            let one = as_element(lower_children[1]);
            // not yet normalized, so we need to compare against ASCII minus and u+2212
            return name(minus) == "mo" && (as_text(minus) == "-" || as_text(minus) == "−") && 
                   name(one) == "mn"   && as_text(one) == "1";
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
    // also en.wikipedia.org/wiki/Chemical_formula#Condensed_formula
    #[derive(PartialEq, Eq)]
    enum BondType {DoubleBond, TripleBond}      // options for is_legal_bond()
    // "⋅" is used in GTM 16.2 and en.wikipedia.org/wiki/Cement_chemist_notation -- may want to add some similar chars
    static CHEM_FORMULA_OPERATORS: phf::Set<&str> = phf_set! {
        "-", "\u{2212}", "⋅", ":", "=", "∷", "≡", ":::", "≣", "::::", // bond symbols (need both 2212 and minus because maybe not canonicalized)
        "⋮", // lewis dots, part of "⋮⋮" - triple bond (see Nemeth chem guide 2.5.4)
    };
    static CHEM_FORMULA_OK: phf::Set<char> = phf_set! {
        '(', ')', '[', ']',
        // FIX: the invisible operator between elements should be well-defined, but this likely needs work, so both accepted for now
        '\u{2062}', '\u{2063}' // invisible separators
        };

    assert_eq!(name(mathml), "mo");
    let leaf_text = as_text(mathml);
    if CHEM_FORMULA_OPERATORS.contains(leaf_text) &&
       ( !(leaf_text == "=" || leaf_text == "∷" ) || is_legal_bond(mathml, BondType::DoubleBond) )  &&
       ( !(leaf_text == "≡" || leaf_text == ":::" ) || is_legal_bond(mathml, BondType::TripleBond) ) {
        mathml.set_attribute_value(MAYBE_CHEMISTRY, "1");
        mathml.set_attribute_value(CHEM_FORMULA_OPERATOR, "1");
        return 1;
    } else if is_in_set(leaf_text, &CHEM_FORMULA_OK) {
        return 0;  // not much info
    } else {
        return -3; // still a small chance;
    }

    fn is_legal_bond(mathml: Element, bond_type: BondType) -> bool {
        let preceding = mathml.preceding_siblings();
        let following = mathml.following_siblings();
        if preceding.is_empty() || following.is_empty() {
            return false;
        }

        let mut preceding_element = as_element(preceding[preceding.len()-1]);
        // special check for CH_2 -- double bond is really with C
        if bond_type == BondType::DoubleBond && name(preceding_element) == "msub" &&
           preceding.len() > 1 &&  &convert_to_short_form(preceding_element).unwrap_or_default() == "H_2" {
            preceding_element = as_element(preceding[preceding.len()-2]);
            if !is_leaf(preceding_element) || as_text(preceding_element) != "C" {
                return false;
            }
        } else if name(preceding_element) != "mi" && name(preceding_element) != "mtext" {
            return false;
        }
        let following_element = get_possible_embellished_node(as_element(following[0]));
        if name(following_element) != "mi" && name(following_element) != "mtext" {
            return false;
        }
        let preceding_text = as_text(preceding_element);
        let following_text = as_text(following_element);
        return match bond_type {
            BondType::DoubleBond => is_legal_double_bond(preceding_text, following_text),
            BondType::TripleBond => is_legal_triple_bond(preceding_text, following_text),
        };

        fn is_legal_double_bond(left: &str, right: &str) -> bool {
            // this is based on table in en.wikipedia.org/wiki/Double_bond#Types_of_double_bonds_between_atoms
            static DOUBLE_BOND_TO_SELF: phf::Set<&str> = phf_set! {
                "C", "O", "N", "S", "Si", "Ge", "Sn", "Pb"
            };
                // "C" => &["O", "N", "S"],
                // "O" => &["N", "S"],
            if left == right && DOUBLE_BOND_TO_SELF.contains(left) {
                return true;
            }
            return match left {
                "C" => right=="O" || right=="N" || right=="S",
                "O" => right=="N" || right=="S",
                "Si" => right=="C",
                _ => false,
            }
        }

        fn is_legal_triple_bond(left: &str, right: &str) -> bool {
            // According to https://tinyurl.com/rkynhwj3 (from physics.org)
            // triple bonds can be formed between any of B, C, N, and O
            // Apparently they can also be forced in other cases, but they are rare.
            // 'B' is from studiousguy.com/triple-bond-examples/
            return  (left == "B"  || left == "C"  || left == "N"  || left == "O") &&
                    (right == "B" || right == "C" || right == "N" || right == "O");
        }
    }
}

/// This assumes canonicalization of characters has happened
fn likely_chem_equation_operator(mathml: Element) -> isize {

    // mostly from chenzhijin.com/en/article/Useful%20Unicode%20for%20Chemists (Arrows and Other)
    static CHEM_EQUATION_OPERATORS: phf::Set<char> = phf_set! {
        '+', '=', '-',
        '·', '℃', '°', '‡', '∆', '×', '\u{2062}' // invisible times
    };

    let elem_name = name(mathml);
    if elem_name == "munder" || elem_name == "mover" || elem_name == "munderover" {
        let base = as_element(mathml.children()[0]);
        if name(base) == "mo" && is_in_set(as_text(base), &CHEM_EQUATION_ARROWS) {
            base.set_attribute_value(MAYBE_CHEMISTRY, "1");
            base.set_attribute_value(CHEM_EQUATION_OPERATOR, "1");
            return 1;
        } else if elem_name == "mover" && is_hack_for_missing_arrows(mathml) {
            return 2;
        } else {
            return NOT_CHEMISTRY;
        }    
    }

    if name(mathml) == "mo" {
        let text = as_text(mathml);
        if is_in_set(text, &CHEM_EQUATION_OPERATORS) || is_in_set(text, &CHEM_EQUATION_ARROWS) {
            mathml.set_attribute_value(MAYBE_CHEMISTRY, "1");
            mathml.set_attribute_value(CHEM_EQUATION_OPERATOR, "1");
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
        assert_eq!(name(mover), "mover");
        let children = mover.children();
        let base = as_element(children[0]);
        let mo_base = if name(base) == "mrow" && base.children().len() == 2 {
            as_element(base.children()[0])
        } else {
            base
        };
        let upper = as_element(children[1]);
        let mo_upper = if name(upper) == "mrow" && upper.children().len() == 2 {
            as_element(upper.children()[1])
        } else {
            upper
        };
        // slightly sloppy match, but almost certainly good enough
        return name(mo_base) == "mo" && name(mo_upper) == "mo" && 
                as_text(mo_base) == "↽" && as_text(mo_upper) == "⇀";
        }
}

fn is_equilibrium_constant(mut mathml: Element) -> bool {
    if name(mathml) == "msub" {
        mathml = as_element(mathml.children()[0]);
    }

    return name(mathml) == "mi" && as_text(mathml) == "K";
}

lazy_static! {
    // Oxidation states range from -4 to 9 and are written with (a subset of) roman numerals.
    // All instances seem to be upper case that I've seen.
    static ref SMALL_UPPER_ROMAN_NUMERAL: Regex = Regex::new(r"^\s*^(IX|IV|V?I{0,3})\s*$").unwrap();
}

/// look for "(s), "(l)", "(g)", "(aq)" (could also use [...])
/// this might be called before canonicalization, but in clean_chemistry_mrow, we made sure "( xxx )" is grouped properly
pub fn likely_chem_state(mathml: Element) -> isize {
    
    if IsBracketed::is_bracketed(mathml, "(", ")", false, false) ||
       IsBracketed::is_bracketed(mathml, "[", "]", false, false) {
        let contents = as_element(mathml.children()[1]);
        let contents_name = name(contents);
        if contents_name == "mi" || contents_name == "mtext" {
            let text = as_text(contents);
            if text == "s" || text == "l" ||text == "g" ||text == "aq" {
                return text.len() as isize + 1;       // hack to count chars -- works because all are ASCII 
            };
        }
     }
     return NOT_CHEMISTRY;
}

/// Returns the likelihood that the arg is an element
pub fn likely_chem_element(mathml: Element) -> isize {
    static NUCLEAR_SYMBOLS: [&str; 6] = ["e", "p", "n", "α", "β","γ"];

    assert!(name(mathml) == "mi" || name(mathml) == "mtext", "{} is not 'mi' or 'mtext'", name(mathml));
    let text = as_text(mathml);
    if as_text(mathml).trim().is_empty() {
        return 0;   // whitespace
    } else if is_chemical_element(mathml) {
        // single letter = 1; single letter with mathvarinat="normal" = 2; double = 3 -- all elements are ASCII
        return (if text.len() == 1 {
            if mathml.attribute_value("mathvariant").unwrap_or_default() == "normal" {2} else {1}
        } else {
            3
        }) as isize;
    } else if NUCLEAR_SYMBOLS.contains(&text) {
        return 0;
        // not much special about them;
    } else {
        return NOT_CHEMISTRY;
    }
}

static SHORT_SINGLE_LETTER_ELEMENT_FORMULAE: phf::Set<&str> = phf_set! {
    // from en.wikipedia.org/wiki/Glossary_of_chemical_formulae (via chem_formula_from_wikipedia.py)
    "BF_3", "BI_3", "BN", "BP", "B_2F_4", "B_2H_6", "B_2O_3", "B_2S_3", "B_4C",
    "CB_4", "CF_4", "CH_2", "CH_4", "CO", "CO_2", "CO_3", "CS_2", "CW", "C_2F_4",
    "C_2H_4", "C_2H_6", "C_2U", "C_2Y", "C_3H_4", "C_3H_6", "C_3H_8", "C_4H_2",
    "C_4H_8", "C_4I_2", "C_6H_6", "C_6N_4", "C_7H_8", "C_8H_8", "DI", "D_2O",
    "FI", "FI_2", "FK", "FN", "FO", "FO_2", "FP", "FS", "FW", "FY", "F_2",
    "F_2N", "F_2O", "F_2O_2", "F_2P", "F_2S", "F_2S_2", "F_2W", "F_2Y", "F_3B",
    "F_3P", "F_3S", "F_3W", "F_3Y", "F_4B_2", "F_4C", "F_4C_2", "F_4N_2",
    "F_4S", "F_4U", "F_4W", "F_5I", "F_5P", "F_5S", "F_5U", "F_5W", "F_6S",
    "F_6W", "F_7I", "HF", "HI", "HK", "HN_3", "H_2", "H_2C", "H_2C_2", "H_2C_4",
    "H_2O", "H_2O_2", "H_2S", "H_3N", "H_3P", "H_4C", "H_4C_2", "H_4C_3",
    "H_4N_2", "H_4N_4", "H_6B_2", "H_6C_2", "H_6C_3", "H_6C_6", "H_8C_3",
    "H_8C_7", "H_8C_8", "ID", "IF", "IF_5", "IF_7", "IH", "IK", "IO_3", "I_2",
    "I_2F", "I_2O_5", "I_2W", "I_3B", "I_3N", "I_3U", "I_3V", "I_4P_2", "I_4W",
    "KH", "KI", "K_2F_2", "K_2O", "K_2O_2", "K_2S", "NB", "NF", "NF_2", "NF_3",
    "NI_3", "NO", "NO_2", "NU", "NV", "N_2", "N_2F_4", "N_2H_2", "N_2H_4",
    "N_2O_3", "N_2O_4", "N_2O_5", "N_3H", "N_4C_6", "N_4H_4", "N_5P_3", "O",
    "OD_2", "OF", "OF_2", "OH_2", "OK_2", "ON", "ON_2", "OT_2", "O_2", "O_2C",
    "O_2F_2", "O_2H_2", "O_2K_2", "O_2N", "O_2S", "O_2U", "O_2W", "O_3",
    "O_3C", "O_3I", "O_3N_2", "O_3S", "O_3U", "O_3V_2", "O_3W", "O_3Y_2",
    "O_5I_2", "O_5N_2", "O_5P_2", "O_5V_2", "O_8U_3", "PB", "PF", "PF_2", "PF_3",
    "PH_3", "PY", "P_2F_4", "P_2I_4", "P_2O_5", "P_2S_3", "P_3N_5", "SF", "SF_2",
    "SF_4", "SF_5", "SF_6", "SH_2", "SK_2", "SO_2", "SO_3", "S_2C", "S_2F_2",
    "S_2W", "S_3B_2", "S_3P_2", "S_3W", "S_3Y_2", "T_2O", "UC_2", "UF_4", "UF_5",
    "UI_3", "UN", "UO_2", "UO_3", "US_2", "U_3O_8", "VI_3", "VN", "V_2O_3",
    "WC", "WF", "WF_2", "WF_3", "WF_4", "WF_5", "WF_6", "WI_2", "WI_4", "WO_2",
    "WS_2", "WS_3", "YB_6", "YC_2", "YF", "YF_2", "YF_3", "YP", "Y_2O_3",

    // from en.wikipedia.org/wiki/Ion#Common_ions (via chem_formula_from_wikipedia.py)
    "CH_3COO^−", "CN^−", "CO_3^2−", "C^−", "C_2O_4^2−", "F^−", "HCOO^−", 
    "HPO_4^2−", "HSO_3^−", "HSO_4^−", "H^+", "H^−", "H_2PO_4^−", "H_3O^+", "I^−", 
    "NH_4^+", "NO_2^−", "NO_3^−", "N^3−", "N_3^−", "OH^−", "O^2−", "O_2^2−", 
    "PO_4^3−", "P^3−", "SO_3^2−", "SO_4^2−", "S^2−", "S_2O_3^2−",

    // from gchem.cm.utexas.edu/canvas.php?target=bonding/ionic/polyatomic-ions.html
    "PO_3^3−", "IO_3^−",

    // others
    "CH_3", /* methyl */
    "NH_3",  // ammonium
};

/// Returns true if the formula is composed of 1 or 2 single letter elements and it matches a known compound/ion
/// This might be called (via likely_adorned_chem_formula) unparsed
fn is_short_formula(mrow: Element) -> bool {
    assert_eq!(name(mrow), "mrow");
    let children = mrow.children();
    let n_children = children.len();
    if n_children == 0 || n_children > 3 || (n_children == 3 && name(as_element(children[1])) != "mo") {
        return false;
    }

    let first_element = convert_to_short_form( as_element(children[0]) );
    if n_children == 1 {
        return first_element.is_ok();
    }
    let second_element = convert_to_short_form( as_element(children[if n_children == 2 {1} else {2}]) );
    return match (first_element, second_element) {
        (Ok(first), Ok(second)) => {
            let short_form = first + second.as_str();
            // debug!("short_form: {}", short_form);
            return SHORT_SINGLE_LETTER_ELEMENT_FORMULAE.contains(&short_form);
        },
        _ => false,
    }
}

fn convert_to_short_form(mathml: Element) -> Result<String> {
    let mathml_name = name(mathml);
    return match mathml_name {
        "mi" | "mtext" | "mn" | "mo" => Ok( as_text(mathml).to_string() ),
        "none" => Ok( "".to_string() ),
        "msub" | "msup" | "msubsup" | "mmultiscripts"=> {
            let is_mmultiscripts = mathml_name == "mmultiscripts";
            let children = mathml.children();
            let mut result = convert_to_short_form(as_element(children[0]))?;
            if is_mmultiscripts && children.len() != 3 {
                bail!("mmultiscripts found with {} children -- not part of chemical formula", children.len());
            }
            if mathml_name == "msub" || mathml_name == "msubsup" || (is_mmultiscripts && name(as_element(children[1])) != "none") {
                result += "_";
                result += &convert_to_short_form(as_element(children[1]))?;
            }
            if mathml_name == "msup" || mathml_name == "msubsup" || (is_mmultiscripts && name(as_element(children[2])) != "none") {
                result += "^";
                result += &convert_to_short_form(as_element(children[if mathml_name=="msup" {1} else {2}]))?;
            }
            Ok( result )
        },
        "mrow" => {
            // the only time this is valid is if the superscript is something like "+" or "2+", so we do a few checks and short circuit false now
            let mrow_children = mathml.children();
            if mrow_children.len() == 1 || mrow_children.len() == 2 {
                let mut result = convert_to_short_form(as_element(mrow_children[0]))?;
                if mrow_children.len() == 2 {
                    result += &convert_to_short_form(as_element(mrow_children[1]))?;
                }
                return Ok(result)
            } else {
                bail!("mrow found with {} children -- not part of chemical formula", mrow_children.len());
            }
        }
        _ => bail!("{} found -- not part of chemical formula", mathml_name),
    }
}

/// A map of chemical elements and their relative IUPAC electronegativity (https://i.stack.imgur.com/VCSzW.png)
/// That list uses a horizontal line for the Lanthanide and Actinide Series.
/// Because I had already ordered the elements before realizing that, I opened a gap and started the higher ones again with a '1' in front.
/// The list is missing recent (unstable) elements -- I added them with the same value as the element above them in the periodic table.
static CHEMICAL_ELEMENT_ELECTRONEGATIVITY: phf::Map<&str, u32> = phf_map! {
	"Ac" => 40, "Ag" => 155, "Al" => 163, "Am" => 29, "Ar" => 4, "As" => 172, "At" => 181, "Au" => 154,
    "B" => 164, "Ba" => 14, "Be" => 18, "Bh" => 137, "Bi" => 170, "Bk" => 27, "Br" => 183,
	"C" => 169, "Ca" => 16, "Cd" => 158, "Ce" => 56, "Cf" => 26, "Cl" => 184, "Cm" => 28, "Cn" => 157, "Co" => 148, "Cr" => 136, "Cs" => 8, "Cu" => 156,
    "Db" => 129, "Ds" => 149, "Dy" => 48, 
	"Er" => 46, "Es" => 25, "Eu" => 51, "F" => 185, "Fe" => 144, "Fl" => 165, "Fm" => 24, "Fr" => 7, "Ga" => 162, "Gd" => 50, "Ge" => 167,
	"H" => 175, "He" => 6, "Hf" => 126, "Hg" => 157, "Ho" => 47, "Hs" => 141, "I" => 182, "In" => 161, "Ir" => 146, "K" => 10, "Kr" => 3,
	"La" => 62, "Li" => 12, "Lr" => 19, "Lu" => 41, "Lv" => 176, "Mc" => 170, "Md" => 23, "Mg" => 17, "Mn" => 140, "Mo" => 135, "Mt" => 145, 
	"N" => 174, "Na" => 11, "Nb" => 131, "Nd" => 54, "Ne" => 5, "Nh" => 160, "Ni" => 152, "No" => 22, "Np" => 31, "O" => 180, "Og" => 1, "Os" => 142, 
	"P" => 173, "Pa" => 33, "Pb" => 165, "Pd" => 151, "Pm" => 53, "Po" => 176, "Pr" => 55, "Pt" => 150, "Pu" => 30,
	"Ra" => 13, "Rb" => 9, "Re" => 138, "Rf" => 125, "Rg" => 153, "Rh" => 147, "Rn" => 1, "Ru" => 143, 
	"S" => 179, "Sb" => 171, "Sc" => 124, "Se" => 178, "Sg" => 133, "Si" => 168, "Sm" => 52, "Sn" => 166, "Sr" => 15,
	"Ta" => 130, "Tb" => 49, "Tc" => 139, "Te" => 177, "Th" => 34, "Ti" => 128, "Tl" => 160, "Tm" => 45, "Ts" => 181, 
	"U" => 32, "V" => 132, "W" => 134, "Xe" => 2, "Y" => 123, "Yb" => 44, "Zn" => 159, "Zr" => 127,
    // The following come from E.A. Moore who said to treat them like chemicals 
    // These stand for methyl, ethyl, alkyl, acetyl and phenyl and apparently are quite commonly used ("Ac" is already a chemical)
    // A full(er?) list is at en.wikipedia.org/wiki/Skeletal_formula#Alkyl_groups and in following sections
    "Me" => 0, "Et" => 0, "R" => 0, /* "Ac" => 0, */ "Ph" => 0,
    "X" => 0, /* treated as an unknown */
};

// A map of the chemical elements and their atomic numbers
static CHEMICAL_ELEMENT_ATOMIC_NUMBER: phf::Map<&str, u32> = phf_map! {
    "H" => 1, "He" => 2, "Li" => 3, "Be" => 4, "B" => 5, "C" => 6, "N" => 7, "O" => 8, "F" => 9, "Ne" => 10,
    "Na" => 11, "Mg" => 12, "Al" => 13, "Si" => 14, "P" => 15, "S" => 16, "Cl" => 17, "Ar" => 18, "K" => 19, "Ca" => 20,
    "Sc" => 21, "Ti" => 22, "V" => 23, "Cr" => 24, "Mn" => 25, "Fe" => 26, "Co" => 27, "Ni" => 28, "Cu" => 29, "Zn" => 30,
    "Ga" => 31, "Ge" => 32, "As" => 33, "Se" => 34, "Br" => 35, "Kr" => 36, "Rb" => 37, "Sr" => 38, "Y" => 39, "Zr" => 40,
    "Nb" => 41, "Mo" => 42, "Tc" => 43, "Ru" => 44, "Rh" => 45, "Pd" => 46, "Ag" => 47, "Cd" => 48, "In" => 49, "Sn" => 50,
    "Sb" => 51, "Te" => 52, "I" => 53, "Xe" => 54, "Cs" => 55, "Ba" => 56, "La" => 57, "Ce" => 58, "Pr" => 59, "Nd" => 60, 
    "Pm" => 61, "Sm" => 62, "Eu" => 63, "Gd" => 64, "Tb" => 65, "Dy" => 66, "Ho" => 67, "Er" => 68, "Tm" => 69, "Yb" => 70,
    "Lu" => 71, "Hf" => 72, "Ta" => 73, "W" => 74, "Re" => 75, "Os" => 76, "Ir" => 77, "Pt" => 78, "Au" => 79, "Hg" => 80,
    "Tl" => 81, "Pb" => 82, "Bi" => 83, "Po" => 84, "At" => 85, "Rn" => 86, "Fr" => 87, "Ra" => 88, "Ac" => 89, "Th" => 90,
    "Pa" => 91, "U" => 92, "Np" => 93, "Pu" => 94, "Am" => 95, "Cm" => 96, "Bk" => 97, "Cf" => 98, "Es" => 99, "Fm" => 100,
    "Md" => 101, "No" => 102, "Lr" => 103, "Rf" => 104, "Db" => 105, "Sg" => 106, "Bh" => 107, "Hs" => 108, "Mt" => 109, "Ds" => 110,
    "Rg" => 111, "Cn" => 112, "Nh" => 113, "Fl" => 114, "Mc" => 115, "Lv" => 116, "Ts" => 117, "Og" => 118, 
};

pub fn is_chemical_element(node: Element) -> bool {
	// FIX: allow name to be in an mrow (e.g., <mi>N</mi><mi>a</mi>
	let name = name(node);
	if name != "mi" && name != "mtext" {
		return false;
	}

	let text = as_text(node);
	return CHEMICAL_ELEMENT_ELECTRONEGATIVITY.contains_key(text);
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
        trim_element(mathml, false);
        return test_mathml(mathml);
    }

    #[test]
    fn test_noble_element() {
        // mathml test strings need to be canonical MathML since we aren't testing canonicalize()
        let test = "<mrow> <mi>Na</mi> <mo>&#x2063;</mo> <mi>Cl</mi> </mrow>"; // 
        assert!( !parse_mathml_string(test, |mathml| has_noble_element( &collect_elements(mathml).unwrap() )) );
        let test = "<mrow> <mi>Ar</mi> <mo>&#x2063;</mo> <mi>Cl</mi> </mrow>"; // 
        assert!( parse_mathml_string(test, |mathml| has_noble_element( &collect_elements(mathml).unwrap() )) );
        let test = "<mrow> <mi>Ne</mi> </mrow>"; // 
        assert!( parse_mathml_string(test, |mathml| has_noble_element( &collect_elements(mathml).unwrap() )) );
    }

    #[test]
    fn test_alphabetical_order() {
        // mathml test strings need to be canonical MathML since we aren't testing canonicalize()
        let test = r#"<mrow>  
            <msub><mi>C</mi><mn>6</mn></msub><mo>&#x2063;</mo> 
            <msub><mi>H</mi><mn>14</mn></msub>
             </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_alphabetical( &collect_elements(mathml).unwrap() )) );
        let test = r#"<mrow>  
             <msub><mi>C</mi><mn>6</mn></msub><mo>&#x2063;</mo> 
             <msub><mi>H</mi><mn>12</mn></msub><mo>&#x2063;</mo>
             <msub><mi>O</mi><mn>6</mn></msub>
              </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_alphabetical( &collect_elements(mathml).unwrap() )) );
        let test = "<mrow> <mi>B</mi> <mo>&#x2063;</mo> <mi>C</mi> <mo>&#x2063;</mo> <mi>O</mi></mrow>"; // "C" should be first
        assert!( !parse_mathml_string(test, |mathml| is_alphabetical( &collect_elements(mathml).unwrap() )) );
        let test = "<mrow> <mi>P</mi> <mo>&#x2063;</mo> <mi>B</mi> <mo>&#x2063;</mo> <mi>O</mi></mrow>"; // not alphabetical
        assert!( !parse_mathml_string(test, |mathml| is_alphabetical( &collect_elements(mathml).unwrap() )) );
    }

    #[test]
    fn test_is_structural() {
        // mathml test strings need to be canonical MathML since we aren't testing canonicalize()
        let test = r#"<mrow>  
            <msub><mi>C</mi><mn>6</mn></msub><mo>&#x2063;</mo> 
            <msub><mi>H</mi><mn>14</mn></msub>
             </mrow>"#;
        assert!( !parse_mathml_string(test, |mathml| is_structural( &collect_elements(mathml).unwrap() )) );
        let test = "<mrow> <mi>B</mi> <mo>&#x2063;</mo> <mi>C</mi> <mo>&#x2063;</mo> <mi>O</mi></mrow>";
        assert!( !parse_mathml_string(test, |mathml| is_structural( &collect_elements(mathml).unwrap() )) );
        let test = "<mrow> <mi>H</mi> <mo>&#x2063;</mo> <mi>O</mi> <mo>&#x2063;</mo> <mi>H</mi></mrow>";
        assert!( parse_mathml_string(test, |mathml| is_structural( &collect_elements(mathml).unwrap() )) );
        let test = "<mrow data-chem-formula='9'>
                <mmultiscripts data-chem-formula='1'>
                <mi mathvariant='normal' data-chem-element='1'>H</mi>
                <mn>2</mn>
                <none></none>
                </mmultiscripts>
                <mo data-changed='added'>&#x2063;</mo>
                <mi mathvariant='normal' data-chem-element='1'>C</mi>
                <mo data-chemical-bond='true' data-chem-formula-op='1'>=</mo>
                <mi mathvariant='normal' data-chem-element='1'>C</mi>
                <mo data-changed='added'>&#x2063;</mo>
                <mmultiscripts data-chem-formula='1'>
                <mi mathvariant='normal' data-chem-element='1'>H</mi>
                <mn>2</mn>
                <none></none>
                </mmultiscripts>
            </mrow>";
        assert!( parse_mathml_string(test, |mathml| is_structural( &collect_elements(mathml).unwrap() )) );
    }


    #[test]
    fn test_electronegativity_order() {
        // mathml test strings need to be canonical MathML since we aren't testing canonicalize()
        let test = r#"<mrow>  
            <mi>N</mi><mo>&#x2063;</mo> 
            <msub><mi>H</mi><mn>3</mn></msub>
             </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_ordered_by_electronegativity( &collect_elements(mathml).unwrap() )) );
        let test = r#"<mrow>  
            <mi>O</mi><mo>&#x2063;</mo> 
            <msub><mi>F</mi><mn>2</mn></msub>
             </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_ordered_by_electronegativity( &collect_elements(mathml).unwrap() )) );
        let test = r#"<mrow>  
            <msub><mi>Rb</mi><mn>15</mn></msub><mo>&#x2063;</mo> 
            <msub><mi>Hg</mi><mn>16</mn></msub>
             </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_ordered_by_electronegativity( &collect_elements(mathml).unwrap() )) );
        let test = r#" 
            <msup>
                <mo>[</mo>
                    <mi>Si</mi><mo>&#x2063;</mo> 
                    <msub><mi>As</mi><mn>4</mn></msub>
                <mo>]</mo>
                <mrow><mn>8</mn><mo>-</mo></mrow>
            </msup>"#;
        assert!( parse_mathml_string(test, |mathml| is_ordered_by_electronegativity( &collect_elements(mathml).unwrap() )) );
        let test = r#"<mrow>  
                <mi>Si</mi><mo>&#x2063;</mo> 
                <msub><mi>H</mi><mn>2</mn></msub>
                <mi>Br</mi><mo>&#x2063;</mo> 
                <mi>Cl</mi>
                </mrow>"#;
        assert!( parse_mathml_string(test, |mathml| is_ordered_by_electronegativity( &collect_elements(mathml).unwrap() )) );
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
        assert!( parse_mathml_string(test, |mathml| likely_chem_formula(mathml)==5) );
    }

    #[test]
    fn test_simple_double_bond() {
        let test1 = r#"<mrow><mi>C</mi><mo>=</mo><mi>C</mi></mrow>"#;
        assert!( parse_mathml_string(test1, |mathml| likely_chem_formula(mathml) < CHEMISTRY_THRESHOLD) ); // just under threshold
        let test2 = r#"<mrow><mi>C</mi><mo>∷</mo><mi>O</mi></mrow>"#;
        assert!( parse_mathml_string(test2, |mathml| likely_chem_formula(mathml)==CHEMISTRY_THRESHOLD) );
        let test3 = r#"<mrow><mi>N</mi><mo>=</mo><mi>N</mi></mrow>"#;
        assert!( parse_mathml_string(test3, |mathml| likely_chem_formula(mathml) < CHEMISTRY_THRESHOLD) ); // just under threshold
        let test4 = r#"<mrow><mi>Sn</mi><mo>=</mo><mi>Sn</mi></mrow>"#;
        assert!( parse_mathml_string(test4, |mathml| likely_chem_formula(mathml) == 8) );
        let test5 = r#"<mrow><mi>O</mi><mo>=</mo><mi>S</mi></mrow>"#;
        assert!( parse_mathml_string(test5, |mathml| likely_chem_formula(mathml) < CHEMISTRY_THRESHOLD) );  // just under threshold
        let test10 = r#"<mrow><mi>K</mi><mo>=</mo><mi>K</mi></mrow>"#;
        assert!( parse_mathml_string(test10, |mathml| likely_chem_formula(mathml) == NOT_CHEMISTRY) );
        let test11 = r#"<mrow><mi>C</mi><mo>=</mo><mi>K</mi></mrow>"#;
        assert!( parse_mathml_string(test11, |mathml| likely_chem_formula(mathml) == NOT_CHEMISTRY) );
    }

    #[test]
    fn test_double_bond() {
        let test1 = r#"<mrow><mi mathvariant='normal'>C</mi><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub><mo>=</mo><mi>C</mi></mrow>"#;
        assert!( parse_mathml_string(test1, |mathml| likely_chem_formula(mathml)==8) );
        let test2 = r#"<mrow><mi mathvariant='normal'>C</mi><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub><mo>=</mo>
        <mi>C</mi><mi>H</mi><mi>R</mi></mrow>"#;
        assert!( parse_mathml_string(test2, |mathml| likely_chem_formula(mathml)==12) );
        let test3 = r#"<mrow><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub><mi mathvariant='normal'>C</mi><mo>=</mo>
                <mi>C</mi><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub></mrow>"#;
        assert!( parse_mathml_string(test3, |mathml| likely_chem_formula(mathml)==11) );
        let test4 = r#"<mrow><mi>H</mi><mo>-</mo><mi>N</mi><mo>=</mo><mi>N</mi><mo>-</mo><mi>H</mi></mrow>"#;
        assert!( parse_mathml_string(test4, |mathml| likely_chem_formula(mathml)==10) );
        let test10 = r#"<mrow><mi mathvariant='normal'>C</mi><msub><mi mathvariant='normal'>H</mi><mn>3</mn></msub><mo>=</mo><mi>C</mi></mrow>"#;
        assert!( parse_mathml_string(test10, |mathml| likely_chem_formula(mathml)==NOT_CHEMISTRY) );
    }

    #[test]
    #[ignore]   // It would be good to say "not chemistry" for this, but there aren't rules for that at the moment
    fn test_water_bond() {
        let test11 = r#"<mrow><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub><mi mathvariant='normal'>O</mi><mo>=</mo><mi>O</mi></mrow>"#;
        assert!( parse_mathml_string(test11, |mathml| {println!("val={}", likely_chem_formula(mathml)); likely_chem_formula(mathml)==8}) );
        // assert!( parse_mathml_string(test11, |mathml| likely_chem_formula(mathml)==NOT_CHEMISTRY) );
    }


    #[test]
    fn test_triple_bond() {
        let test1 = r#"<mrow><mi>C</mi><mo>≡</mo><mi>C</mi></mrow>"#;
        assert!( parse_mathml_string(test1, |mathml| likely_chem_formula(mathml) < CHEMISTRY_THRESHOLD) );
        let test2 = r#"<mrow><mi>C</mi><mo>:::</mo><mi>O</mi></mrow>"#;
        assert!( parse_mathml_string(test2, |mathml| likely_chem_formula(mathml)==CHEMISTRY_THRESHOLD) );
        let test3 = r#"<mrow><mi>H</mi><mo>-</mo><mi>C</mi><mo>≡</mo><mi>C</mi><mo>-</mo><mi>H</mi></mrow>"#;
        assert!( parse_mathml_string(test3, |mathml| likely_chem_formula(mathml)==10) );
        let test4 = r#"<mrow><mi>H</mi><mo>-</mo><mi>C</mi><mo>≡</mo><mi>C</mi><mo>-</mo><mi>H</mi></mrow>"#;
        assert!( parse_mathml_string(test4, |mathml| likely_chem_formula(mathml)==10) );
        let test5 = r#"<mrow><mi>N</mi><mo>-</mo><mi>C</mi><mo>≡</mo><mi>C</mi><mo>-</mo><mi>N</mi></mrow>"#;
        assert!( parse_mathml_string(test5, |mathml| likely_chem_formula(mathml)==10) );
        let test6 = r#"<mrow><mi>H</mi><mo>-</mo><mi>C</mi><mo>≡</mo>
            <mi>C</mi><mo>-</mo><mi mathvariant='normal'>C</mi><msub><mi mathvariant='normal'>H</mi><mn>3</mn></msub></mrow>"#; // 1-Propyne
        assert!( parse_mathml_string(test6, |mathml| likely_chem_formula(mathml)==14) );
        // assert!( parse_mathml_string(test6, |mathml| {println!("val={}", likely_chem_formula(mathml)); likely_chem_formula(mathml)==10}) );
        let test10 = r#"<mrow><mi>O</mi><mo>:::</mo><mi>S</mi></mrow>"#;
        assert!( parse_mathml_string(test10, |mathml| likely_chem_formula(mathml)==NOT_CHEMISTRY) );
        let test11 = r#"<mrow><mi>Pb</mi><mo>≡</mo><mi>Pb</mi></mrow>"#;
        assert!( parse_mathml_string(test11, |mathml| likely_chem_formula(mathml)==NOT_CHEMISTRY) );
        let test12 = r#"<mrow><mi>C</mi><mo>≡</mo><mi>K</mi></mrow>"#;
        assert!( parse_mathml_string(test12, |mathml| likely_chem_formula(mathml)==NOT_CHEMISTRY) );
    }

    #[test]
    fn split_mi() {
        let test = "<math><mi>LiF</mi></math>";
        let target = "<math>
            <mrow data-changed='added' data-chem-formula='5'>
                <mi data-chem-element='3'>Li</mi>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mi mathvariant='normal' data-split='true' data-chem-element='1'>F</mi>
            </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn no_split_mi() {
        let test = "<math><mi>HC</mi></math>";
        let target = "<math>
             <mi>HC</mi>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn combine_mi() {
        let test = "<math><mi>H</mi><mi>C</mi><mi>l</mi></math>";
        let target = " <math>
            <mrow data-changed='added' data-chem-formula='5'>
            <mi data-chem-element='1'>H</mi>
            <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
            <mi data-merged='true' data-chem-element='3'>Cl</mi>
            </mrow>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn no_combine() {
        let test = "<math><mi>C</mi><mi>l</mi></math>";
        let target = "<math>
            <mrow data-changed='added'>
                <mi>C</mi>
                <mo data-changed='added'>&#x2062;</mo>
                <mi>l</mi>
            </mrow>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn add_script() {
        let test = "<math> <mi>SO</mi>  <msub> <mrow></mrow> <mn>2</mn> </msub> </math>";
        let target = "<math>
            <mrow data-changed='added' data-chem-formula='5'>
                <mi mathvariant='normal' data-chem-element='1'>S</mi>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mmultiscripts data-chem-formula='2'>
                    <mi mathvariant='normal' data-split='true' data-chem-element='1'>O</mi>
                    <mn>2</mn>
                    <none></none>
                </mmultiscripts>
            </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn add_script_bug_287() {
        let test = r#"<math><mrow>
            <msubsup>
                <mrow><mi mathvariant="normal">SO</mi></mrow>
                <mn>4</mn>
                <mrow><mn>2</mn><mo>&#x2212;</mo></mrow>
            </msubsup>
            </mrow></math>"#;
        let target = r#"<math>
            <mrow data-changed='added' data-chem-formula='7'>
                <mi mathvariant='normal' data-chem-element='1'>S</mi>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <msubsup data-chem-formula='5'>
                    <mi mathvariant='normal' data-split='true' data-chem-element='1'>O</mi>
                    <mn>4</mn>
                    <mrow data-chem-formula='3'><mn>2</mn><mo>-</mo></mrow>
                </msubsup>
            </mrow>
            </math>"#;
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn salt() {
        let test = "<math><mi>Na</mi><mi>Cl</mi></math>";
        let target = "<math>
            <mrow data-changed='added' data-chem-formula='7'>
                <mi data-chem-element='3'>Na</mi>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mi data-chem-element='3'>Cl</mi>
            </mrow>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn water() {
        let test = "<math><msub><mi mathvariant='normal'>H</mi><mn>2</mn></msub><mi mathvariant='normal'>O</mi></math>";
        let target = "<math>
            <mrow data-changed='added' data-chem-formula='5'>
                <msub data-chem-formula='2'>
                    <mi mathvariant='normal' data-chem-element='2'>H</mi>
                    <mn>2</mn>
                </msub>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mi mathvariant='normal' data-chem-element='2'>O</mi>
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
            <mrow data-chem-formula='5'>
                <mmultiscripts data-chem-formula='2'>
                    <mi mathvariant='normal' data-chem-element='2'>H</mi>
                    <mn>2</mn>
                    <none></none>
                </mmultiscripts>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mi mathvariant='normal' data-chem-element='2'>O</mi>
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
        <msup data-chem-formula='9'>
          <mrow data-chem-formula='6'>
            <mo>[</mo>
            <mrow data-changed='added' data-chem-formula='3'>
              <mi data-chem-element='1'>S</mi>
              <mo data-changed='added'>&#x2063;</mo>
              <msub data-chem-formula='1'>
                <mi data-chem-element='1'>O</mi>
                <mn>4</mn>
              </msub>
            </mrow>
            <mo>]</mo>
          </mrow>
          <mrow data-chem-formula='3'>
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
        let target = " <math>
                <mrow data-chem-formula='10'>
                    <msub data-chem-formula='3'>
                        <mi data-chem-element='3'>Al</mi>
                        <mn>2</mn>
                    </msub>
                    <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                    <msub data-chem-formula='6'>
                        <mrow data-chem-formula='6'>
                        <mo>(</mo>
                        <mrow data-changed='added' data-chem-formula='3'>
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
        <mrow data-chem-formula='13'>
          <mi data-chem-element='1'>C</mi>
          <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
          <msub data-chem-formula='1'>
            <mi data-chem-element='1'>H</mi>
            <mn>3</mn>
          </msub>
          <mo data-chemical-bond='true' data-chem-formula-op='1'>-</mo>
          <mi data-chem-element='1'>C</mi>
          <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
          <msub data-chem-formula='1'>
            <mi data-chem-element='1'>H</mi>
            <mn>2</mn>
          </msub>
          <mo data-chemical-bond='true' data-chem-formula-op='1'>-</mo>
          <mi data-chem-element='1'>O</mi>
          <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
          <mi data-chem-element='1'>H</mi>
        </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn dichlorine_hexoxide() {
        // init_logger();
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
        let target = "<math>
            <mrow data-chem-formula='19'>
                <msup data-chem-formula='9'>
                    <mrow data-chem-formula='8'>
                    <mo>[</mo>
                    <mrow data-changed='added' data-chem-formula='5'>
                        <mi data-chem-element='3'>Cl</mi>
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
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <msup data-chem-formula='9'>
                    <mrow data-chem-formula='8'>
                    <mo>[</mo>
                    <mrow data-changed='added' data-chem-formula='5'>
                        <mi data-chem-element='3'>Cl</mi>
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
            <mrow data-chem-formula='8'>
                <msub data-chem-formula='1'>
                    <mi data-chem-element='1'>H</mi>
                    <mn>2</mn>
                </msub>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mi data-chem-element='1'>C</mi>
                <mo data-chemical-bond='true' data-chem-formula-op='1'>=</mo>
                <mi data-chem-element='1'>C</mi>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <msub data-chem-formula='1'>
                    <mi data-chem-element='1'>H</mi>
                    <mn>2</mn>
                </msub>
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
            <mrow data-chem-formula='11'>
                <mi data-chem-element='3'>Fe</mi>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <msub data-chem-formula='3'>
                    <mi data-chem-element='3'>Cl</mi>
                    <mn>3</mn>
                </msub>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mrow data-chem-formula='3'>
                    <mo>(</mo>
                    <mi>aq</mi>
                    <mo>)</mo>
                </mrow>
            </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn ferric_chloride_aq_as_mi() {
        let test = "<math><mrow>
            <mi>Fe</mi>
            <msub><mi>Cl</mi><mn>3</mn></msub>
            <mi>(aq)</mi>
        </mrow></math>";
        let target = "<math>
            <mrow data-chem-formula='11'>
                <mi data-chem-element='3'>Fe</mi>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <msub data-chem-formula='3'>
                    <mi data-chem-element='3'>Cl</mi>
                    <mn>3</mn>
                </msub>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mrow data-chem-formula='3'>
                    <mo>(</mo>
                    <mi>aq</mi>
                    <mo>)</mo>
                </mrow>
            </mrow>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn chemtype_ammonia() {
        let test = r#"<math><msub><mi>NH</mi><mn>3</mn></msub></math>"#;
        let target = " <math>
            <mrow data-changed='added' data-chem-formula='5'>
            <mi mathvariant='normal' data-chem-element='1'>N</mi>
            <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
            <msub data-chem-formula='2'>
                <mi mathvariant='normal' data-chem-element='1' data-split='true'>H</mi>
                <mn>3</mn>
            </msub>
            </mrow>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn mhchem_ammonia() {
        let test = r#"<math>
            <mrow>
                <mi data-mjx-auto-op="false">NH</mi>
                <msub>
                    <mpadded width="0">
                    <mphantom>
                        <mi>A</mi>
                    </mphantom>
                    </mpadded>
                    <mpadded height="0">
                    <mn>3</mn>
                    </mpadded>
                </msub>
            </mrow>
        </math>"#;
        let target = "<math>
            <mrow data-chem-formula='5'>
                <mi mathvariant='normal' data-chem-element='1'>N</mi>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mmultiscripts data-mjx-auto-op='false' data-chem-formula='2'>
                <mi mathvariant='normal' data-mjx-auto-op='false' data-split='true' data-chem-element='1'>H</mi>
                <mn>3</mn>
                <none></none>
                </mmultiscripts>
            </mrow>
            </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn mhchem_so4() {
        let test = "<math>
            <mrow>
            <mi>SO</mi>
            <msub>
                <mpadded width='0'>
                <mphantom>
                    <mi>A</mi>
                </mphantom>
                </mpadded>
                <mpadded height='0'>
                <mn>4</mn>
                </mpadded>
            </msub>
            <msup>
                <mpadded width='0'>
                <mphantom>
                    <mi>A</mi>
                </mphantom>
                </mpadded>
                <mrow>
                <mn>2</mn>
                <mo>&#x2212;</mo>
                </mrow>
            </msup>
            </mrow>
        </math>";
        let target = "<math>
            <mrow data-chem-formula='7'>
                <mi mathvariant='normal' data-chem-element='1'>S</mi>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mmultiscripts data-chem-formula='5'>
                    <mi mathvariant='normal' data-split='true' data-chem-element='1'>O</mi>
                    <mn>4</mn>
                    <none/>
                    <none/>
                    <mrow data-chem-formula='3'>
                    <mn>2</mn>
                    <mo>-</mo>
                    </mrow>
                </mmultiscripts>
            </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn mhchem_short_ion() {
        let test = "  <math>
                <mrow>
                <mi mathvariant='normal'>H</mi>
                <msub>
                    <mpadded width='0'> <mphantom> <mi>A</mi> </mphantom>  </mpadded>
                    <mpadded height='0'> <mn>3</mn></mpadded>
                </msub>
                <mi mathvariant='normal'>O</mi>
                <msup>
                    <mpadded width='0'> <mphantom> <mi>A</mi> </mphantom>  </mpadded>
                    <mo>+</mo>
                </msup>
                </mrow>
            </math>";
        let target = "<math>
            <mrow data-chem-formula='6'>
                <mmultiscripts data-chem-formula='2'>
                    <mi mathvariant='normal' data-chem-element='2'>H</mi>
                    <mn>3</mn>
                    <none></none>
                </mmultiscripts>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mmultiscripts data-chem-formula='3'>
                    <mi mathvariant='normal' data-chem-element='2'>O</mi>
                    <none></none>
                    <mo>+</mo>
                </mmultiscripts>
            </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn mhchem_ions_and_state() {
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
            <mrow data-chem-formula='18'>
                <mmultiscripts data-chem-formula='4'>
                    <mi data-chem-element='3'>Na</mi>
                    <none></none>
                    <mo>+</mo>
                </mmultiscripts>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mrow data-changed='added' data-chem-formula='3'>
                    <mo stretchy='false'>(</mo>
                    <mi>aq</mi>
                    <mo stretchy='false'>)</mo>
                </mrow>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mmultiscripts data-chem-formula='5'>
                    <mi data-chem-element='3'>Cl</mi>
                    <none></none>
                    <mo>-</mo>
                </mmultiscripts>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mrow data-changed='added' data-chem-formula='3'>
                    <mo stretchy='false' data-previous-space-width='0.111'>(</mo>
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
            <mrow data-chem-formula='8'>
                <msub data-chem-formula='1'>
                    <mi data-chem-element='1'>H</mi>
                    <mn>2</mn>
                </msub>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <mi data-chem-element='1'>C</mi>
                <mo data-chemical-bond='true' data-chem-formula-op='1'>∷</mo>
                <mi data-chem-element='1'>C</mi>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                <msub data-chem-formula='1'>
                    <mi data-chem-element='1'>H</mi>
                    <mn>2</mn>
                </msub>
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
            <mmultiscripts data-previous-space-width='-0.083'>
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
            <mrow data-chem-formula='9'>
                <mn>2</mn>
                <mo data-changed='added' data-chem-formula-op='0'>&#x2062;</mo>
                <mrow data-changed='added' data-chem-formula='9'>
                    <mi mathvariant='normal' data-previous-space-width='0.167' data-chem-element='1'>H</mi>
                    <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                    <mi data-split='true' data-chem-element='3'>Cl</mi>
                    <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
                    <mrow data-changed='added' data-chem-formula='3'>
                    <mo stretchy='false' data-previous-space-width='0.111'>(</mo>
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
    let target = "<math>
        <mmultiscripts data-chem-formula='8'>
            <mrow data-changed='added' data-chem-formula='8'>
                <mo stretchy='false'>(</mo>
                <mrow data-changed='added' data-chem-formula='5'>
                <mi mathvariant='normal' data-chem-element='1'>C</mi>
                <mo data-changed='added'>&#x2063;</mo>
                <mmultiscripts data-chem-formula='2'>
                    <mi mathvariant='normal' data-split='true' data-chem-element='1'>H</mi>
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

    #[test]
    fn mhchem_isotopes() {
        // from \ce{^{18}O{}^{16}O}
        let test = "<math>
        <mrow>
          <msubsup>
            <mpadded width='0'>
              <mphantom>
                <mi>A</mi>
              </mphantom>
            </mpadded>
            <mpadded height='0' depth='0'>
              <mphantom></mphantom>
            </mpadded>
            <mpadded height='0' depth='0'>
              <mphantom>
                <mn>18</mn>
              </mphantom>
            </mpadded>
          </msubsup>
          <mspace width='-0.083em'></mspace>
          <msubsup>
            <mpadded width='0'>
              <mphantom>
                <mi>A</mi>
              </mphantom>
            </mpadded>
            <mrow>
              <mpadded width='0'>
                <mphantom>
                  <mn>2</mn>
                </mphantom>
              </mpadded>
              <mpadded width='0' lspace='-1width'>
                <mpadded height='0'></mpadded>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded height='0'>
                <mpadded width='0'>
                  <mphantom>
                    <mn>2</mn>
                  </mphantom>
                </mpadded>
              </mpadded>
              <mpadded width='0' lspace='-1width'>
                <mn>18</mn>
              </mpadded>
            </mrow>
          </msubsup>
          <mi mathvariant='normal'>O</mi>
          <mspace width='0.111em'></mspace>
          <msubsup>
            <mpadded width='0'>
              <mphantom>
                <mi>A</mi>
              </mphantom>
            </mpadded>
            <mpadded height='0' depth='0'>
              <mphantom></mphantom>
            </mpadded>
            <mpadded height='0' depth='0'>
              <mphantom>
                <mn>16</mn>
              </mphantom>
            </mpadded>
          </msubsup>
          <mspace width='-0.083em'></mspace>
          <msubsup>
            <mpadded width='0'>
              <mphantom>
                <mi>A</mi>
              </mphantom>
            </mpadded>
            <mrow>
              <mpadded width='0'>
                <mphantom>
                  <mn>2</mn>
                </mphantom>
              </mpadded>
              <mpadded width='0' lspace='-1width'>
                <mpadded height='0'></mpadded>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded height='0'>
                <mpadded width='0'>
                  <mphantom>
                    <mn>2</mn>
                  </mphantom>
                </mpadded>
              </mpadded>
              <mpadded width='0' lspace='-1width'>
                <mn>16</mn>
              </mpadded>
            </mrow>
          </msubsup>
          <mi mathvariant='normal'>O</mi>
        </mrow>
      </math>";
    let target = "<math>
        <mrow data-chem-formula='7'>
            <mmultiscripts data-previous-space-width='-0.083' data-chem-formula='3'>
                <mi mathvariant='normal' data-chem-element='2'>O</mi>
                <mprescripts></mprescripts>
                <none></none>
                <mn>18</mn>
            </mmultiscripts>
            <mo data-changed='added' data-chem-formula-op='0'>&#x2063;</mo>
            <mmultiscripts data-previous-space-width='0.027999999999999997' data-chem-formula='3'>
                <mi mathvariant='normal' data-chem-element='2'>O</mi>
                <mprescripts></mprescripts>
                <none></none>
                <mn>16</mn>
            </mmultiscripts>
        </mrow>
    </math>";
    assert!(are_strs_canonically_equal(test, target));
    }

    
    #[test]
    fn merge_bug_274() {
        let test = r#"
        <math>
            <mrow>
                <mtable>
                    <mtr>
                        <mtd>
                            <mrow>
                                <msub><mtext>H</mtext><mn>2</mn></msub>
                                <mtext>g</mtext>
                                <mtext/>
                                <mtext>+</mtext>
                                <mtext/>
                                <msub><mrow><mtext>Cl</mtext></mrow><mn>2</mn></msub>
                                <mo stretchy="false">(</mo>
                                <mtext>g</mtext>
                                <mo stretchy="false">)</mo>
                                <mo>&#x2192;</mo>
                                <mn>2</mn>
                                <mtext>HCl(g)</mtext>
                            </mrow>
                        </mtd>
                    </mtr>
                    <mtr>
                        <mtd>
                            <mrow>
                                <mn>1</mn>
                                <mo>:</mo>
                                <mn>1</mn>
                                <mo>:</mo>
                                <mn>2</mn>
                            </mrow>
                        </mtd>
                    </mtr>
                    <mtr>
                        <mtd>
                            <mrow>
                                <mn>1</mn>
                                <mtext/>
                                <msub><mtext>H</mtext><mn>2</mn></msub>
                                <mtext/>
                                <mtext>to</mtext>
                                <mtext/>
                                <mn>1</mn>
                                <mtext/>
                                <msub><mrow><mtext>Cl</mtext></mrow><mn>2</mn></msub>
                                <mtext/>
                                <mtext>to</mtext>
                                <mtext/>
                                <mtext>2</mtext>
                                <mtext/>
                                <mtext>HCl</mtext>
                            </mrow>
                        </mtd>
                    </mtr>
                </mtable>
            </mrow>
        </math>
        "#;
        let target = "
            <math>
            <mtable>
                <mtr>
                <mtd data-maybe-chemistry='9'>
                    <mrow data-maybe-chemistry='9'>
                    <mrow data-changed='added' data-maybe-chemistry='8'>
                        <mrow data-changed='added' data-maybe-chemistry='1'>
                        <msub data-maybe-chemistry='1'>
                            <mtext data-maybe-chemistry='1'>H</mtext>
                            <mn>2</mn>
                        </msub>
                        <mo data-changed='added' data-maybe-chemistry='0'>&#x2062;</mo>
                        <mtext data-maybe-chemistry='0'>g</mtext>
                        </mrow>
                        <mo data-chem-equation-op='1' data-maybe-chemistry='1'>+</mo>
                        <mrow data-changed='added' data-maybe-chemistry='6'>
                        <msub data-maybe-chemistry='3'>
                            <mtext data-maybe-chemistry='3'>Cl</mtext>
                            <mn>2</mn>
                        </msub>
                        <mo data-changed='added' data-maybe-chemistry='0'>&#x2063;</mo>
                        <mrow data-changed='added' data-maybe-chemistry='2'>
                            <mo stretchy='false'>(</mo>
                            <mtext>g</mtext>
                            <mo stretchy='false'>)</mo>
                        </mrow>
                        </mrow>
                    </mrow>
                    <mo data-chem-equation-op='1' data-maybe-chemistry='1'>→</mo>
                    <mrow data-changed='added' data-maybe-chemistry='0'>
                        <mn data-maybe-chemistry='0'>2</mn>
                        <mo data-changed='added' data-maybe-chemistry='0'>&#x2062;</mo>
                        <mtext data-maybe-chemistry='0'>HCl(g)</mtext>
                    </mrow>
                    </mrow>
                </mtd>
                </mtr>
                <mtr>
                <mtd>
                    <mrow>
                    <mn>1</mn>
                    <mo>:</mo>
                    <mn>1</mn>
                    <mo>:</mo>
                    <mn>2</mn>
                    </mrow>
                </mtd>
                </mtr>
                <mtr>
                <mtd data-maybe-chemistry='7'>
                    <mrow data-maybe-chemistry='7'>
                    <mn data-maybe-chemistry='0'>1</mn>
                    <mo data-changed='added' data-maybe-chemistry='0'>&#x2062;</mo>
                    <msub data-maybe-chemistry='1'>
                        <mtext data-maybe-chemistry='1'>H</mtext>
                        <mn>2</mn>
                    </msub>
                    <mo data-changed='added' data-maybe-chemistry='0'>&#x2062;</mo>
                    <mtext data-maybe-chemistry='0'>to</mtext>
                    <mo data-changed='added' data-maybe-chemistry='0'>&#x2062;</mo>
                    <mn data-maybe-chemistry='0'>1</mn>
                    <mo data-changed='added' data-maybe-chemistry='0'>&#x2062;</mo>
                    <msub data-maybe-chemistry='3'>
                        <mtext data-maybe-chemistry='3'>Cl</mtext>
                        <mn>2</mn>
                    </msub>
                    <mo data-changed='added' data-maybe-chemistry='0'>&#x2062;</mo>
                    <mtext data-maybe-chemistry='0'>to</mtext>
                    <mo data-changed='added' data-maybe-chemistry='0'>&#x2062;</mo>
                    <mtext data-maybe-chemistry='0'>2</mtext>
                    <mo data-changed='added' data-maybe-chemistry='0'>&#x2062;</mo>
                    <mi data-maybe-chemistry='1' mathvariant='normal'>H</mi>
                    <mo data-changed='added' data-maybe-chemistry='0'>&#x2062;</mo>
                    <mi data-maybe-chemistry='3' data-split='true'>Cl</mi>
                    </mrow>
                </mtd>
                </mtr>
            </mtable>
            </math>
        ";
        assert!(are_strs_canonically_equal(test, target));
    }
    
    #[test]
    fn merge_bug_303() {
        let test = r#"
            <math>
                <mn>2</mn>
                <msup><mtext>OH</mtext><mo>−</mo></msup>
                <mo stretchy="false">(</mo>
                <mtext>aq</mtext>
                <mo stretchy="false">)</mo>
                <mo>+</mo>
                <mtext>C</mtext>
                <msup><mtext>u</mtext><mrow><mn>2</mn><mo>+</mo></mrow></msup>
            </math>
        "#;
        let target = "
            <math>
                <mrow data-changed='added'>
                <mrow data-changed='added'>
                    <mn>2</mn>
                    <mo data-changed='added'>&#x2062;</mo>
                    <mrow data-changed='added'>
                        <msup><mi>OH</mi><mo>-</mo></msup>
                        <mo data-changed='added'>&#x2061;</mo>
                        <mrow data-changed='added'>
                            <mo stretchy='false'>(</mo>
                            <mtext>aq</mtext>
                            <mo stretchy='false'>)</mo>
                        </mrow>
                    </mrow>
                </mrow>
                <mo>+</mo>
                <mrow data-changed='added'>
                    <mtext>C</mtext>
                    <mo data-changed='added'>&#x2062;</mo>
                    <msup> <mtext>u</mtext> <mrow><mn>2</mn><mo>+</mo></mrow> </msup>
                </mrow>
                </mrow>
            </math>
           ";
        assert!(are_strs_canonically_equal(test, target));
    }
    
    #[test]
    fn mtd_assert_bug_393() {
        let test = r#"
        <math display="block">
            <mtable>
                <mtr>
                <mtd>
                    <mrow>
                    <mi>A</mi>
                    <mi>c</mi>
                    </mrow>
                </mtd>
                <mtd>
                    <mi>A</mi>
                    <mfenced>
                    <mtable>
                        <mtr>
                        <mtd>
                            <mrow>
                            <mi>c</mi>
                            <mi>n</mi>
                            </mrow>
                        </mtd>
                        </mtr>
                    </mtable>
                    </mfenced>
                </mtd>
                </mtr>
            </mtable>
        </math>"#;
        let target = "
        <math display='block'>
            <mtable>
            <mtr>
                <mtd>
                <mi>A</mi>
                <mi>c</mi>
                </mtd>
                <mtd>
                <mrow data-changed='added'>
                    <mi>A</mi>
                    <mrow>
                    <mo data-changed='from_mfenced'>(</mo>
                    <mtable>
                        <mtr>
                        <mtd>
                            <mrow>
                            <mi>c</mi>
                            <mi>n</mi>
                            </mrow>
                        </mtd>
                        </mtr>
                    </mtable>
                    <mo data-changed='from_mfenced'>)</mo>
                    </mrow>
                </mrow>
                </mtd>
            </mtr>
            </mtable>
        </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

}
