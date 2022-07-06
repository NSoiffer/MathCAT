
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
///     2. msub/msup/msubsup: is base marked MAYBE_CHEMISTRY and the scripts are potential adornments, mark it MAYBE_CHEMISTRY
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
///           no: clear all the marks for the old children and replace mathml children with old one (links are messed up)
/// After canonicalization, we take another pass looking for chemical equations and marking them if found.

use sxd_document::dom::*;
use crate::canonicalize::{as_element, as_text, name};
use crate::pretty_print::mml_to_string;
use crate::xpath_functions::is_leaf;
use regex::Regex;
use crate::xpath_functions::IsBracketed;
use phf::phf_set;


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


pub fn clean_chemistry_mathml(mut mathml: Element) {
    let pref_manager = crate::prefs::PreferenceManager::get();
    if pref_manager.borrow().get_user_prefs().to_string("Chemistry") == "Off" {
        return;
    }
    let mut tag_name = name(&mathml);
    if tag_name == "math" && mathml.children().len() == 1 {
        mathml = as_element(mathml.children()[0]);
        tag_name = name(&mathml);
        // if child_tag_name == "mrow" {       // mrow wrapper maybe added (FIX: check for data-changed='added'?)
        //     mathml = child;
        //     tag_name = child_tag_name;
        // }
    }
    debug!("check_for_chem_formula: {}", mml_to_string(&mathml));
    if tag_name != "mrow" {
        error!("clean_chemistry_mathml: found none mrow\n{}", mml_to_string(&mathml));
        return;
    }
    let old_children = mathml.children().iter()
                .map(|child| as_element(*child))
                .collect::<Vec<Element>>();
    if let Some(new_children) = mark_possible_chemistry_children(&old_children) {
        if mark_actual_chemistry_children(&new_children) {
            mathml.replace_children(&new_children);
        } else {
            unset_marked_chemistry(mathml);
        }
    } // else nothing was done

}

/// Do some aggressive structural changes and if they make this look like a chemistry formula, make it as one else remove other marks
/// Note: the element is replaced with a new restructured element if it is marked as chemistry
///        Pass 1:
///        a) for any run of mi/mtext that can be re-split into chem elements, split them and mark them if it is at least 3 chars long
///        b) if there are any potential chem formula operators (e.g., "=" and ":") and the previous node is marked MAYBE_CHEMISTRY,
///           mark this as MAYBE_CHEMISTRY

fn mark_possible_chemistry_children<'a>(old_children: &[Element<'a>]) -> Option<Vec<Element<'a>>> {
    let mut new_children = Vec::with_capacity(2*old_children.len());
    for &child in old_children {
        let tag_name = name(&child);
        // FIX -- implement gathering up mi/mtext
        if tag_name == "mo" {
            likely_chem_formula_operator(child);
        }
        new_children.push(child.clone());
    }

    return Some(new_children);
}

/// Pass 2: (assuming something was marked in pass 1)
/// a) find the first marked child and then the last consecutive marked child and trim any mo's from the ends
/// b) evaluate the likelihood that the sequence is chemistry
fn mark_actual_chemistry_children(children: &[Element]) -> bool {
    let mut start = None;
    let mut likelihood = 0;
    for i in 0..children.len() {
        let child = children[i];
        if let Some(attr_val) = child.attribute_value(MAYBE_CHEMISTRY)  {
            if start ==None {
                if name(&child) == "mo" {
                    child.remove_attribute(MAYBE_CHEMISTRY);
                } else {
                    start = Some(i);
                    likelihood = attr_val.parse::<isize>().unwrap();
                }
            } else {
                likelihood += attr_val.parse::<isize>().unwrap();
            }
        } else if let Some(seq_start) = start {
            for stop in (seq_start..i).rev() {
                let end_child = children[stop];
                if name(&end_child) == "mo" {
                    let attr_val = end_child.attribute_value(MAYBE_CHEMISTRY).unwrap();
                    likelihood -= attr_val.parse::<isize>().unwrap();
                    end_child.remove_attribute(MAYBE_CHEMISTRY);
                } else {
                    if likelihood < IS_CHEMISTRY_THRESHOLD {
                        for k in seq_start..stop {
                            children[k].remove_attribute(MAYBE_CHEMISTRY);
                        }
                    }
                    start = None;
                    break;
                }
            }
        }
    }

    return true;
}


/// Looks at the children of the element and uses heuristics to decide whether this is a chemical equation/formula
/// If it is, it is marked with either data-chem-equation or data-chem-formula
/// This function assumes proper structure
pub fn scan_and_mark_chemistry(mathml: Element) {
    // FIX -- need to implement
    debug!("mark_if_likely_chemistry (after): {}", mml_to_string(&mathml));
    likely_chem_formula(mathml);
    unset_marked_chemistry(mathml)

    // if is_leaf(mathml) {
    //     match name(&mathml) {
    //         "mi" | "mo" => set_chem_attr(mathml, CHEM_ELEMENT),
    //         _ => NOT_CHEMISTRY,
    //     };
    //     return;
    // }
    // set_chem_attr(mathml, CHEM_FORMULA);   // FIX: this might be CHEM_EQUATION

    // for child in mathml.children() {
    //     let child = as_element(child);
    //     mark_if_likely_chemistry(child);
    // }
}

// returns the marked attr value or None
fn get_marked_value(mathml: Element) -> Option<isize> {
    if let Some(value) = mathml.attribute_value(CHEM_ELEMENT) {
        return Some(value.parse().unwrap());
    } else  if let Some(value) = mathml.attribute_value(CHEM_FORMULA) {
        return Some(value.parse().unwrap());
    } else if let Some(value) = mathml.attribute_value(CHEM_EQUATION) {
        return Some(value.parse().unwrap());
    } else {
        return None;
    }
}

/// Sets the attr 'chem'
/// Recurse through all the children that have MAYBE_CHEMISTRY set
pub fn set_marked_chemistry_attr(mathml: Element, chem: &str) {
    if let Some(maybe) = mathml.attribute(MAYBE_CHEMISTRY) {
        maybe.remove_from_parent();

        if is_leaf(mathml) {
            mathml.set_attribute_value(CHEM_ELEMENT, maybe.value());
        } else {
            mathml.set_attribute_value(chem, maybe.value());
            for child in mathml.children() {
                set_marked_chemistry_attr(as_element(child), chem);
            }
        }    
    }
}

/// Clears MAYBE_CHEMISTRY from this element and its decedents
fn unset_marked_chemistry(mathml: Element) {
    // If MAYBE_CHEMISTRY is not set, we don't need to recurse
    if let Some(maybe_chem_attr) = mathml.attribute(MAYBE_CHEMISTRY) {
        maybe_chem_attr.remove_from_parent();
        if !is_leaf(mathml) {
            for child in mathml.children() {
                unset_marked_chemistry(as_element(child));
            }
        }
    }
}

/// Looks at the children of the element and uses heuristics to decide whether this is a chemical equation 
fn likely_chem_equation(mathml: Element) -> isize {
    if name(&mathml) != "mrow" {
        return NOT_CHEMISTRY;
    }

	// mrow -- check the children to see if we are likely to be a chemical formula

    // possible improvement -- give bonus points for consecutive (not counting invisible separators) chemical elements on top of the existing points
	let mut likelihood = 0;						// indicator of likely match
	let mut has_equilibrium_constant = false;
    let children = mathml.children();
	for i in 0..children.len() {
		let child = as_element(children[i]);
		if i == children.len()-1 {
			if is_chem_state(child) {
                likelihood += 2;
                break;
			}
            // otherwise, check the last element as normal
        }
        let tag_name = name(&child);
        match tag_name {
            "mi" => likelihood += likely_chem_element(mathml),
            "mn" => (),       // not much info
            "mo" => likelihood += likely_chem_equation_operator(mathml),
            "msub" | "msup" | "msubsup" | "mmultiscripts" => {
                if is_equilibrium_constant(mathml) {
                    has_equilibrium_constant = true;
                    likelihood += 2;
                } else {
                    likelihood += likely_adorned_chem_formula(mathml);
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
            _ => return NOT_CHEMISTRY,
        };

        if likelihood < NOT_CHEMISTRY_THRESHOLD {
            return NOT_CHEMISTRY;
        }
    }

    return likelihood;
}



fn likely_chem_subscript(supscript: Element) -> isize {
    let subscript_name = name(&supscript);
    if  subscript_name == "mn" && !as_text(supscript).contains(".") {
        return 0;       // not really much chem info about an integer subscript
    } else if subscript_name == "mi" {
        let text = as_text(supscript);
        if text == "s" || text == "l" ||text == "g" ||text == "aq" {
            return 2;
        }
    } else if is_chem_state(supscript) { // notation used in en.wikipedia.org/wiki/Electrolyte
        return 2;       
    }
    return NOT_CHEMISTRY;     
}

fn likely_valid_chem_superscript(sup: Element) -> isize {
    // either one or more '+'s (or '-'s) or a number followed by +/-
    // also could be state (en.wikipedia.org/wiki/Nuclear_chemistry#PUREX_chemistry)
    lazy_static! {
        static ref PLUS_OR_MINUS: Regex = Regex::new(r"^\++ | -+$").unwrap(); 
    }

    let sup_name = name(&sup);
    if sup_name == "mo" && PLUS_OR_MINUS.is_match(as_text(sup)) {
        return if as_text(sup).len()==1 {1} else {2};
    } else if sup_name == "mrow" {
        let children = sup.children();
        if children.len() == 2 {
            let first = as_element(children[0]);
            let second = as_element(children[1]);
            if name(&first) == "mn" && name(&second) == "mo" &&
                !as_text(first).contains(".") &&
                (as_text(second) == "+" || as_text(second) == "-") {
                    return 2;   // ending with a +/- makes it likely this is an ion
            }
        }
    }
    return NOT_CHEMISTRY;
}


/// chem_formula is likely if it is one of:
/// * a (possibly adorned) chemical element
/// * an operator that represents a bond
/// * fences around a chemical formula
/// * an mrow made up of only chemical formulas
fn likely_chem_formula(mathml: Element) -> isize {
    if let Some(value) = get_marked_value(mathml) {
        return value;       // already marked
    }

    // if is_leaf(mathml) {
    //     if let Some(likely) = mathml.attribute_value(MAYBE_CHEMISTRY) {
    //         return likely.parse().unwrap();
    //     } else {
    //         return NOT_CHEMISTRY;
    //     }
    // }

    let tag_name = name(&mathml);
    match tag_name {
        // a parent may clear the chem flags if something says can't be chemistry (e.g, a non chemically valid script)
        "mi" => return likely_chem_element(mathml),
        "mo" => return likely_chem_formula_operator(mathml),
        "msub" | "msup" | "msubsup" | "mmultiscripts" => return likely_adorned_chem_formula(mathml),
        "mrow" => {
            let mut mrow = mathml;
            // check if it is bracketed -- doesn't add much info
            if IsBracketed::is_bracketed(&mrow, "(", ")", false, false) ||
               IsBracketed::is_bracketed(&mrow, "[", "]", false, false) {
                mrow = as_element(mrow.children()[1]);
            } 

            // check all the children and compute the likelihood of that this is a chemical formula
            let mut likelihood = 0;
            for child in mrow.children() {
                let child = as_element(child);
                likelihood += likely_chem_formula(child);
                debug!("in likely_chem_formula likelihood={}, child {}", likelihood, mml_to_string(&child));
                if likelihood < NOT_CHEMISTRY_THRESHOLD {
                    return NOT_CHEMISTRY;
                }
            }

            if likelihood < IS_CHEMISTRY_THRESHOLD {
                // the children may have looked have looked right, but something as said "not likely"
                // mark this element so that unset_marked_chemistry will unset the children
                mrow.set_attribute_value(MAYBE_CHEMISTRY, "-1000000");
                unset_marked_chemistry(mrow);
                return NOT_CHEMISTRY;
            } else {
                let likelihood_str = likelihood.to_string();
                if mathml != mrow {
                    mrow.set_attribute_value(MAYBE_CHEMISTRY, &likelihood_str);
                }
                mathml.set_attribute_value(MAYBE_CHEMISTRY, &likelihood_str);
                set_marked_chemistry_attr(mathml, CHEM_FORMULA);
                return likelihood;
            }
        },
        _ => {
            if !is_leaf(mathml) {
                for child in mathml.children() {
                    if likely_chem_formula(as_element(child)) < NOT_CHEMISTRY_THRESHOLD {
                    };
                }
                // mark this element so that unset_marked_chemistry will unset the children
                mathml.set_attribute_value(MAYBE_CHEMISTRY, "-1000000");
            }
            return NOT_CHEMISTRY;
        },
    }
}

// Returns the likelihood that the arg is an adorned chem formula
// Adornments are:
//   superscripts with +/- and optionally a number (charge)
//   numeric subscripts (e.g. H_2)
// In addition to chemical elements, we include nuclear decay since there is a lot of overlap in notation
// The nuclear decay notation is mostly taken from https://tinyurl.com/2f6b8e3a
// Basically it is a chemical element or 'e', 'p', 'n', 'α', 'β', or 'γ' with pre-sub/superscript
// There is also an instance with a charge on the referenced page, so we allow that also.
// Note: https://tinyurl.com/ysmr8cw2 says "++"/"--", etc., is sometimes used in a superscript particle physics instead of a "2"
pub fn likely_adorned_chem_formula(mathml: Element) -> isize {
    // some simple sanity checks on the scripts...
    let tag_name = name(&mathml);
    let mut likelihood = 0;
    if tag_name == "msub" || tag_name == "msubsup" {
        // subscripts should be just a number
        let subscript = as_element(mathml.children()[1]);
        likelihood += likely_chem_subscript(subscript);
    }

    if tag_name == "msup" || tag_name == "msubsup" {
        let superscript = as_element(mathml.children()[if tag_name == "msup" {1} else {2}]);
        likelihood += likely_valid_chem_superscript(superscript);
    }

    if tag_name == "mmultiscripts" {
        // prescripts should be positive integers
        let children = mathml.children();
        let prescripts = if children.len() == 4 && name(&as_element(children[1]))=="mprescripts" {
            &children[2..3]
        } else if children.len() == 6 && name(&as_element(children[3]))=="mprescripts" {
            &children[1..2]
        } else {
            return NOT_CHEMISTRY;
        };
        if name(&as_element(prescripts[0])) != "mn" || name(&as_element(prescripts[1])) != "mn" {
            // fix could make sure they are integers
            return NOT_CHEMISTRY;
        }
        likelihood += 1;        // looking like an atomic number

        // if there is a post superscript, it should be a charge
        if children.len() == 6 {
            if name(&as_element(children[1]))!="none" {
                return NOT_CHEMISTRY;
            }
            likelihood += likely_valid_chem_superscript(as_element(children[2]));
        }
    }

    let base = as_element(mathml.children()[0]);
    if let Some(likely) = get_marked_value(base) {
        likelihood += likely;
    } else if let Some(likely) = mathml.attribute_value(MAYBE_CHEMISTRY) {
        likelihood += likely.parse::<isize>().unwrap();
    }
    
    return likelihood;
}

/// useful function to see if the str is a single char that is in 'set'
fn is_in_set(leaf_text: &str, set: &phf::Set<char> ) -> bool {
    let mut chars = leaf_text.chars();
    let ch = chars.next();
    if chars.next().is_none() {     // only one char
        if let Some(ch) = ch {
            return set.contains(&ch);
        }
    }
    return false;
}

fn likely_chem_formula_operator(mathml: Element) -> isize {
    // mostly from chenzhijin.com/en/article/Useful%20Unicode%20for%20Chemists (Arrows and Other)
    static CHEM_FORMULA_OPERATORS: phf::Set<char> = phf_set! {
        '-', '=', '≡', '≣', '⠆', '⠇', '⠿',  // bond symbols
        // FIX: the invisible operator between elements should be well-defined, but this likely needs work, so both accepted for now
        '\u{2061}', '\u{2063}' // invisible separators
    };

    assert_eq!(name(&mathml), "mo");
    if is_in_set(as_text(mathml), &CHEM_FORMULA_OPERATORS) {
        return 1;
    } else {
        return -3; // still a small chance;
    } 
}

fn likely_chem_equation_operator(mathml: Element) -> isize {
    // mostly from chenzhijin.com/en/article/Useful%20Unicode%20for%20Chemists (Arrows and Other)
    static CHEM_EQUATION_OPERATORS: phf::Set<char> = phf_set! {
        '+', '=',
        '·', '℃', '°', '‡', '∆', '×',
        // FIX: the invisible operator between elements should be well-defined, but this likely needs work, so both accepted for now
        '\u{2061}', '\u{2063}' // invisible separators
    };

    // these can be in the base of an under/over script
    static CHEM_EQUATION_ARROWS: phf::Set<char> = phf_set! {
        '→', '➔', '←', '⟶', '⟵', '⤻', '⇋', '⇌',
        '↿', '↾', '⇃', '⇂', '⥮', '⥯', '⇷', '⇸', '⤉', '⤈',
        '⥂', '⥄',
    };

    let mathml = mathml;
    let elem_name = name(&mathml);
    if elem_name == "munder" || elem_name == "mover" || elem_name == "munderover" {
        let mathml = as_element(mathml.children()[0]);
        if name(&mathml) == "mo" && is_in_set(as_text(mathml), &CHEM_EQUATION_ARROWS) {
            return 1;
        } else {
            return NOT_CHEMISTRY;
        }    
    }

    if name(&mathml) == "mo" {
        let text = as_text(mathml);
        if is_in_set(text, &CHEM_EQUATION_OPERATORS) || is_in_set(text, &CHEM_EQUATION_ARROWS) {
            return 1;
        }
    }
    return -3;  // there is still a chance
}

fn is_equilibrium_constant(mut mathml: Element) -> bool {
    if name(&mathml) == "msub" {
        mathml = as_element(mathml.children()[0]);
    }

    return name(&mathml) == "mi" && as_text(mathml) == "K";
}

/// look for "(s), "(l)", "(g)", "(aq)" (could also use [...])
fn is_chem_state(mathml: Element) -> bool {
    if IsBracketed::is_bracketed(&mathml, "(", ")", false, false) ||
       IsBracketed::is_bracketed(&mathml, "[", "]", false, false) {
        let contents = as_element(mathml.children()[1]);
        if name(&contents) == "mi" {
            let text = as_text(contents);
            return text == "s" || text == "l" ||text == "g" ||text == "aq";
        }
     }
     return false;
}

/// Returns the likelihood that the arg is an element
pub fn likely_chem_element(mathml: Element) -> isize {
    static NUCLEAR_SYMBOLS: [&str; 6] = ["e", "p", "n", "α", "β","γ"];

    assert_eq!(name(&mathml), "mi");   
    let text = as_text(mathml) ;
    if is_chemical_element(mathml) {
        // single letter = 1; double =2; triple = 3
        return ((3 * text.chars().count() as isize) /2) as isize;
    } else if NUCLEAR_SYMBOLS.contains(&text) {
        // not much special about them;
        return 0;
    } else {
        return NOT_CHEMISTRY;
    }
}


static CHEMICAL_ELEMENTS: phf::Set<&str> = phf_set! {
	"Ac", "Ag", "Al", "Am", "Ar", "As", "At", "Au", "B", "Ba", "Be", "Bh", "Bi", "Bk", "Br",
	"C", "Ca", "Cd", "Ce", "Cf", "Cl", "Cm", "Cn", "Co", "Cr", "Cs", "Cu", "Db", "Ds", "Dy", 
	"Er", "Es", "Eu", "F", "Fe", "Fl", "Fm", "Fr", "Ga", "Gd", "Ge",
	"H", "He", "Hf", "Hg", "Ho", "Hs", "I", "In", "Ir", "K", "Kr",
	"La", "Li", "Lr", "Lu", "Lv", "Mc", "Md", "Mg", "Mn", "Mo", "Mt", 
	"N", "Na", "Nb", "Nd", "Ne", "Nh", "Ni", "No", "Np", "O", "Og", "Os", 
	"P", "Pa", "Pb", "Pd", "Pm", "Po", "Pr", "Pt", "Pu",
	"Ra", "Rb", "Re", "Rf", "Rg", "Rh", "Rn", "Ru", 
	"S", "Sb", "Sc", "Se", "Sg", "Si", "Sm", "Sn", "Sr",
	"Ta", "Tb", "Tc", "Te", "Th", "Ti", "Tl", "Tm", "Ts", 
	"U", "V", "W", "Xe", "Y", "Yb", "Zn", "Zr"};

pub fn is_chemical_element(node: Element) -> bool {
	// FIX: allow name to be in an mrow (e.g., <mi>N</mi><mi>a</mi>
	let name = name(&node);
	if name != "mi" && name != "mtext" {
		return false;
	}

	let text = as_text(node);
	return CHEMICAL_ELEMENTS.contains(text);
}


#[cfg(test)]
mod chem_tests {
	#[allow(unused_imports)]
	use super::super::init_logger;
	use super::super::are_strs_canonically_equal;

    #[test]
    fn salt() {
        init_logger();
        let test = "<math><mi>Na</mi><mi>Cl</mi></math>";
        let target = " <math>
        <mrow data-changed='added' data-chem-formula='7'>
          <mi data-chem-element='3'>Na</mi>
          <mo data-changed='added'>&#x2063;</mo>
          <mi data-chem-element='3'>Cl</mi>
        </mrow>
       </math>";
        assert!(are_strs_canonically_equal(test, target));
    }

    #[test]
    fn water() {
        let test = "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>";
        let target = " <math>
            <mrow data-changed='added' data-chem-formula='3'>
                <msub data-chem-formula='1'>
                    <mi data-chem-element='1'>H</mi>
                    <mn>2</mn>
                </msub>
                <mo data-changed='added'>&#x2063;</mo>
                <mi data-chem-element='1'>O</mi>
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
}