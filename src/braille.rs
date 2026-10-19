#![allow(clippy::needless_return)]
use strum::Display;
use sxd_document_no_unsafe::dom::{Element, ChildOfElement};
use sxd_document_no_unsafe::Package;
use sxd_document_no_unsafe::as_str;
use crate::definitions::SPEECH_DEFINITIONS;
use crate::errors::*;
use crate::pretty_print::mml_to_string;
use crate::prefs::PreferenceManager;
use std::cell::Ref;
use regex::{Captures, Regex, RegexSet};
use phf::{phf_map, phf_set};
use crate::speech::{BRAILLE_RULES, SpeechRulesWithContext, braille_replace_chars, make_quoted_string};
use crate::canonicalize::get_parent;
use std::borrow::Cow;
use std::ops::Range;
use std::sync::LazyLock;
#[allow(unused_imports)]
use log::{debug, error};

fn is_ueb_prefix(ch: char) -> bool {
    matches!(ch, '⠼' | '⠈' | '⠘' | '⠸' | '⠐' | '⠨' | '⠰' | '⠠')
}

/// Returns the braille *char* at the given position in the braille string.
fn braille_at(braille: &str, index: usize) -> char {
    // braille is always 3 bytes per char
    return braille[index..index+3].chars().next().unwrap();

}

/// braille the MathML
/// If 'nav_node_id' is not an empty string, then the element with that id will have dots 7 & 8 turned on as per the pref
/// Returns the braille string (highlighted) along with the *character* start/end of the highlight (whole string if no highlight)
pub fn braille_mathml(mathml: Element, nav_node_id: &str) -> Result<(String, usize, usize)> {
    return BRAILLE_RULES.with(|rules| {
        rules.borrow_mut().read_files()?;
        let rules = rules.borrow();
        let new_package = Package::new();
        let mut rules_with_context = SpeechRulesWithContext::new(&rules, new_package.as_document(), nav_node_id, 0);
        let braille_string = rules_with_context.match_pattern::<String>(mathml)
                        .context("Pattern match/replacement failure!")?;
        // debug!("braille_mathml: braille string: {}", &braille_string);
        let braille_string = braille_string.replace(' ', "");
        let pref_manager = rules_with_context.get_rules().pref_manager.borrow();
        let highlight_style = pref_manager.pref_to_string("BrailleNavHighlight");
        let braille_code = pref_manager.pref_to_string("BrailleCode");
        let braille = match get_braille_code(&braille_code) {
            Some(code) => code.cleanup(pref_manager, braille_string),
            // probably needs cleanup if someone has another code, but this will have to get added by hand
            None => braille_string.trim_matches('⠀').to_string(),
        };

        return Ok(
            if highlight_style != "Off" {
                highlight_braille_chars(braille, &braille_code, highlight_style == "All")
            } else {
                let end = braille.len()/3;
                (braille, 0, end)
            }
        );
    });
    
    /// highlight with dots 7 & 8 based on the highlight style
    /// both the start and stop points will be extended to deal with indicators such as capitalization
    /// if 'fill_range' is true, the interior will be highlighted
    /// Returns the braille string (highlighted) along with the [start, end) *character* of the highlight (whole string if no highlight)
    fn highlight_braille_chars(braille: String, braille_code: &str, fill_range: bool) -> (String, usize, usize) {
        let mut braille = braille;
        // some special (non-braille) chars weren't converted to having dots 7 & 8 to indicate navigation position
        // they need to be added to the start

        // find start and end (byte) indexes of the highlighted region (braille chars have length=3 bytes)
        let start = braille.find(is_highlighted);
        let end = braille.rfind(is_highlighted);
        if start.is_none() {
            assert!(end.is_none());
            let end = braille.len();
            return (braille, 0, end/3);
        };

        let start = start.unwrap();
        let mut end = end.unwrap() + 3;         // always exists if start exists ('end' is exclusive)
        // debug!("braille highlight: start/end={}/{}; braille={}", start/3, end/3, braille);
        let mut start = highlight_first_indicator(&mut braille, braille_code, start, end);
        if let Some(new_range) = expand_highlight(&mut braille, braille_code, start, end) {
            (start, end) = new_range
        }

        if start == end {
            return (braille, start/3, end/3);
        }

        if !fill_range {
            return (braille, start/3, end/3);
        }

        let mut result = String::with_capacity(braille.len());
        result.push_str(&braille[..start]);
        let highlight_region =&mut braille[start..end];
        for ch in highlight_region.chars() {
            result.push( highlight(ch) );
        };
        result.push_str(&braille[end..]);
        return (result, start/3, end/3);

        /// Return the byte index of the first place to highlight
        fn highlight_first_indicator(braille: &mut String, braille_code: &str, start_index: usize, end_index: usize) -> usize {
            // chars in the braille block range use 3 bytes -- we can use that to optimize the code some
            let first_ch = unhighlight(braille_at(braille, start_index));

            // need to highlight (optional) capital/number, language, and style (max 2 chars) also in that (rev) order
            let mut prefix_ch_index = std::cmp::max(0, start_index as isize - 5*3) as usize;
            if prefix_ch_index == 0 {
                // don't count the word or passage mode as part of a indicator (UEB); other codes return 0
                prefix_ch_index = get_braille_code(braille_code).map_or(0, |code| code.highlight_word_passage_prefix(braille));
            }
            let indicators = &braille[prefix_ch_index..start_index];   // chars to be examined
            // treat unknown codes like UEB because they probably have similar number and letter prefixes
            let n_indicator_chars = get_braille_code(braille_code)
                .map_or_else(|| i_start_ueb(indicators), |code| code.highlight_first_indicator_len(indicators, first_ch));
            let i_byte_start = start_index - 3 * n_indicator_chars;
            if i_byte_start < start_index {
                // remove old highlight as long as we don't wipe out the end highlight
                if start_index < end_index {
                    let old_first_char_bytes = start_index..start_index+3;
                    let replacement_str = unhighlight(braille_at(braille, start_index)).to_string();
                    braille.replace_range(old_first_char_bytes, &replacement_str);
                }

                // add new highlight
                let new_first_char_bytes = i_byte_start..i_byte_start+3;
                let replacement_str = highlight(braille_at(braille, i_byte_start)).to_string();
                braille.replace_range(new_first_char_bytes, &replacement_str);
            }

            return i_byte_start;
        }

        /// Return the byte indexes of the first and last place to highlight
        /// Currently, this only does something for CMU braille (see `Cmu::expand_highlight`)
        fn expand_highlight(braille: &mut String, braille_code: &str, start_index: usize, end_index: usize) -> Option<(usize, usize)> {
            if start_index == 0 || end_index == braille.len() {
                return None;
            }
            return get_braille_code(braille_code)?.expand_highlight(braille, start_index, end_index);
        }
    }
}

/// Given a position in a Nemeth string, what is the position character that starts it (e.g, the prev char for capital letter)
fn i_start_nemeth(braille_prefix: &str, first_ch: char) -> usize {
    fn is_nemeth_number(ch: char) -> bool {
        matches!(ch, '⠂' | '⠆' | '⠒' | '⠲' | '⠢' | '⠖' | '⠶' | '⠦' | '⠔' | '⠴' | '⠨')
    }
    let mut n_chars = 0;
    let prefix = &mut braille_prefix.chars().rev().peekable();
    if prefix.peek() == Some(&'⠠') ||  // cap indicator
       (prefix.peek() == Some(&'⠼') && is_nemeth_number(first_ch)) ||  // number indicator
       [Some(&'⠸'), Some(&'⠈'), Some(&'⠨')].contains(&prefix.peek()) {         // bold, script/blackboard, italic indicator
        n_chars += 1;
        prefix.next();
    } 

    if [Some(&'⠰'), Some(&'⠸'), Some(&'⠨')].contains(&prefix.peek()) {   // English, German, Greek
        n_chars += 1;
    } else if prefix.peek() == Some(&'⠈') {  
        let ch = prefix.next();                              // Russian/Greek Variant
        if ch == Some('⠈') || ch == Some('⠨') {
            n_chars += 2;
        }
    } else if prefix.peek() == Some(&'⠠')  { // Hebrew 
        let ch = prefix.next();                              // Russian/Greek Variant
        if ch == Some('⠠') {
            n_chars += 2;
        }
    };
    return n_chars;
}

/// Given a position in a UEB string, what is the position character that starts it (e.g, the prev char for capital letter)
fn i_start_ueb(braille_prefix: &str) -> usize {
    let prefix = &mut braille_prefix.chars().rev().peekable();
    let mut n_chars = 0;
    while let Some(ch) = prefix.next() {
        if is_ueb_prefix(ch) {
            n_chars += 1;
        } else if ch == '⠆' {
            let n_typeform_chars = check_for_typeform(prefix);
            if n_typeform_chars > 0 {
                n_chars += n_typeform_chars;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    return n_chars;
}


fn check_for_typeform(prefix: &mut dyn std::iter::Iterator<Item=char>) -> usize {
    fn is_ueb_typeform_prefix(ch: char) -> bool {
        matches!(ch, '⠈' | '⠘' | '⠸' | '⠨')
    }

    if let Some(typeform_indicator) = prefix.next() {
        if is_ueb_typeform_prefix(typeform_indicator) {
            return 2;
        } else if typeform_indicator == '⠼' &&
                  let Some(user_defined_typeform_indicator) = prefix.next() &&
                  (is_ueb_typeform_prefix(user_defined_typeform_indicator) || user_defined_typeform_indicator == '⠐') {
                    return 3;
                }
    }
    return 0;
}

/// A braille code (Nemeth, UEB, ...).
///
/// Each code encapsulates its per-code post-processing ("cleanup"), leaf-char generation,
/// grouping decision, and navigation-highlight knobs. To add a new code: create a unit struct,
/// implement this trait, and register it in `braille_code()`. The four dispatch sites
/// (cleanup in `braille_mathml`, `BrailleChars::get_braille_chars`, `NeedsToBeGrouped`,
/// and `highlight_braille_chars`) all go through the registry, so no other code needs to change.
trait BrailleCode: Sync {
    /// Name of the code, as used by the `BrailleCode` preference and in rule files.
    fn name(&self) -> &'static str;

    /// Post-process the raw (rule-generated) braille into the final braille string.
    fn cleanup(&self, pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String;

    /// Braille chars for a *leaf* node (mn/mi/mo/mtext/ms). Default: the code has no leaf handling.
    fn get_braille_chars(&self, _node: Element, _text_range: Option<Range<usize>>) -> Result<String> {
        bail!("get_braille_chars: braille code '{}' does not implement leaf char generation", self.name());
    }

    /// Whether `mathml` needs grouping indicators around it. Default: the code doesn't use grouping.
    fn needs_grouping(&self, _mathml: Element, _is_base: bool) -> StdResult<bool, XPathError> {
        return Err(XPathError::Other { what: format!(
            "NeedsToBeGrouped: braille code arg '{}' is not a known code ('UEB', 'CMU', or 'Swedish')", self.name()) });
    }

    // --- navigation-highlight knobs (UEB-like defaults; override per code as needed) ---

    /// Number of prefix braille cells (before the first highlighted char) that belong to that char.
    fn highlight_first_indicator_len(&self, indicators: &str, _first_ch: char) -> usize {
        return i_start_ueb(indicators);
    }
    /// Number of leading cells that are a word/passage indicator and should not be counted as an
    /// indicator run (only relevant when the highlight starts at the very beginning). UEB only.
    fn highlight_word_passage_prefix(&self, _braille: &str) -> usize { return 0; }
    /// Expand the highlight to include grouping indicators around the selection. CMU only.
    fn expand_highlight(&self, _braille: &mut String, _start_index: usize, _end_index: usize) -> Option<(usize, usize)> {
        return None;
    }
}

/// Look up the implementation for a braille code name. Returns `None` for unknown codes.
fn get_braille_code(code: &str) -> Option<&'static dyn BrailleCode> {
    return Some(match code {
        "Nemeth" => &Nemeth,
        "UEB" => &Ueb,
        "Vietnam" => &Vietnam,
        "CMU" => &Cmu,
        "Finnish" => &Finnish,
        "Russian" => &Russian,
        "Swedish" => &Swedish,
        "French" => &French,
        "LaTeX" => &LaTeX,
        "ASCIIMath" => &AsciiMath,
        _ => return None,
    });
}

struct Nemeth;
struct Ueb;
struct Vietnam;
struct Cmu;
struct Finnish;
struct Russian;
struct Swedish;
struct French;
#[allow(non_camel_case_types)]
struct LaTeX;
struct AsciiMath;

impl BrailleCode for Nemeth {
    fn name(&self) -> &'static str { "Nemeth" }
    fn cleanup(&self, pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String { nemeth_cleanup(pref_manager, raw_braille) }
    fn get_braille_chars(&self, node: Element, text_range: Option<Range<usize>>) -> Result<String> { BrailleChars::get_braille_nemeth_chars(node, text_range) }
    fn highlight_first_indicator_len(&self, indicators: &str, first_ch: char) -> usize { i_start_nemeth(indicators, first_ch) }
}

impl BrailleCode for Ueb {
    fn name(&self) -> &'static str { "UEB" }
    fn cleanup(&self, pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String { ueb_cleanup(pref_manager, raw_braille) }
    fn get_braille_chars(&self, node: Element, text_range: Option<Range<usize>>) -> Result<String> { BrailleChars::get_braille_ueb_chars(node, text_range) }
    fn needs_grouping(&self, mathml: Element, is_base: bool) -> StdResult<bool, XPathError> { Ok(NeedsToBeGrouped::needs_grouping_for_ueb(mathml, is_base)) }
    fn highlight_word_passage_prefix(&self, braille: &str) -> usize {
        // don't count the word or passage mode as part of an indicator
        if braille.starts_with("⠰⠰⠰") { return 9; } else if braille.starts_with("⠰⠰") { return 6; } else { return 0; }
    }
}

impl BrailleCode for Vietnam {
    fn name(&self) -> &'static str { "Vietnam" }
    fn cleanup(&self, pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String { vietnam_cleanup(pref_manager, raw_braille) }
    fn get_braille_chars(&self, node: Element, text_range: Option<Range<usize>>) -> Result<String> { BrailleChars::get_braille_vietnam_chars(node, text_range) }
}

impl BrailleCode for Cmu {
    fn name(&self) -> &'static str { "CMU" }
    fn cleanup(&self, pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String { cmu_cleanup(pref_manager, raw_braille) }
    fn get_braille_chars(&self, node: Element, text_range: Option<Range<usize>>) -> Result<String> { BrailleChars::get_braille_cmu_chars(node, text_range) }
    fn needs_grouping(&self, mathml: Element, is_base: bool) -> StdResult<bool, XPathError> { Ok(NeedsToBeGrouped::needs_grouping_for_cmu(mathml, is_base)) }
    fn expand_highlight(&self, braille: &mut String, start_index: usize, end_index: usize) -> Option<(usize, usize)> {
        // For CMU, we want to expand mrows to include the opening and closing grouping indicators if they exist
        let first_ch = unhighlight(braille_at(braille, start_index));
        let last_ch = unhighlight(braille_at(braille, end_index-3));
        // We need to be careful not to expand the selection if we are already on a grouping indicator
        if first_ch == '⠢' && last_ch == '⠔'{
            return None;
        }
        let preceding_ch = braille_at(braille, start_index-3);
        if preceding_ch != '⠢' {
            return None;
        }

        let following_ch = braille_at(braille, end_index);
        if following_ch != '⠔' {
            return None;
        }

        let preceding_ch = highlight(preceding_ch);
        braille.replace_range(start_index-3..start_index+3, format!("{preceding_ch}{first_ch}").as_str());
        let following_ch = highlight(following_ch);
        braille.replace_range(end_index-3..end_index+3, format!("{last_ch}{following_ch}").as_str());
        return Some( (start_index-3, end_index + 3) );
    }
}

impl BrailleCode for Finnish {
    fn name(&self) -> &'static str { "Finnish" }
    fn cleanup(&self, pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String { finnish_cleanup(pref_manager, raw_braille) }
    fn get_braille_chars(&self, node: Element, text_range: Option<Range<usize>>) -> Result<String> { BrailleChars::get_braille_ueb_chars(node, text_range) }    // FIX: need to figure out what to implement
    fn needs_grouping(&self, mathml: Element, is_base: bool) -> StdResult<bool, XPathError> { Ok(NeedsToBeGrouped::needs_grouping_for_finnish(mathml, is_base)) }
}

impl BrailleCode for Russian {
    fn name(&self) -> &'static str { "Russian" }
    fn cleanup(&self, pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String { russian_cleanup(pref_manager, raw_braille) }
    fn get_braille_chars(&self, node: Element, text_range: Option<Range<usize>>) -> Result<String> { BrailleChars::get_braille_russian_chars(node, text_range) }    // FIX: need to figure out what to implement
}

impl BrailleCode for Swedish {
    fn name(&self) -> &'static str { "Swedish" }
    fn cleanup(&self, pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String { swedish_cleanup(pref_manager, raw_braille) }
    fn get_braille_chars(&self, node: Element, text_range: Option<Range<usize>>) -> Result<String> { BrailleChars::get_braille_ueb_chars(node, text_range) }    // FIX: need to figure out what to implement
    fn needs_grouping(&self, mathml: Element, is_base: bool) -> StdResult<bool, XPathError> { Ok(NeedsToBeGrouped::needs_grouping_for_swedish(mathml, is_base)) }
}

impl BrailleCode for French {
    fn name(&self) -> &'static str { "French" }
    fn cleanup(&self, pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String { french_cleanup(pref_manager, raw_braille) }
    fn get_braille_chars(&self, node: Element, text_range: Option<Range<usize>>) -> Result<String> { BrailleChars::get_braille_ueb_chars(node, text_range) }
    fn needs_grouping(&self, mathml: Element, is_base: bool) -> StdResult<bool, XPathError> { Ok(NeedsToBeGrouped::needs_grouping_for_french(mathml, is_base)) }
}

impl BrailleCode for LaTeX {
    fn name(&self) -> &'static str { "LaTeX" }
    fn cleanup(&self, pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String { LaTeX_cleanup(pref_manager, raw_braille) }
}

impl BrailleCode for AsciiMath {
    fn name(&self) -> &'static str { "ASCIIMath" }
    fn cleanup(&self, pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String { ASCIIMath_cleanup(pref_manager, raw_braille) }
}

// FIX: if 8-dot braille is needed, perhaps the highlights can be shifted to a "highlighted" 256 char block in private space 
//   they would need to be unshifted for the external world
fn is_highlighted(ch: char) -> bool {
    let ch_as_u32 = ch as u32;
    return (0x28C0..0x28FF).contains(&ch_as_u32) || ch == '𝑏';           // 0x28C0..0x28FF all have dots 7 & 8 on
}

fn highlight(ch: char) -> char {
    // safe because we have checked the range
    return unsafe{char::from_u32_unchecked(ch as u32 | 0xC0)};    // 0x28C0..0x28FF all have dots 7 & 8 on
}

fn unhighlight(ch: char) -> char {
    let ch_as_u32 = ch as u32;
    if (0x28C0..0x28FF).contains(&ch_as_u32) {              // 0x28C0..0x28FF all have dots 7 & 8 on
        return unsafe{char::from_u32_unchecked(ch_as_u32 & 0x283F)};  // safe because we have checked the range
    } else {
        return ch;
    }
}

use std::cell::RefCell;
thread_local!{
    /// Count number of probes -- get a sense of how well algorithm is working (for debugging)
    static N_PROBES: RefCell<usize> = const { RefCell::new(0) };
}


/// Given a 0-based braille position, return the id of the smallest MathML node enclosing it.
/// This node might be a leaf with an offset.
pub fn get_navigation_node_from_braille_position(mathml: Element, position: usize) -> Result<(String, usize)> {
    // This works via a "smart" binary search (the trees aren't binary or balanced, we estimate the child to look in):
    //   braille the mathml with a nav node and see where 'position' is in relation to the start/end of the nav node
    // Each call to find_navigation_node() returns a search state that tell us where to look next if not found
    #[derive(Debug, Display)]
    enum SearchStatus {
        LookInParent,       // look up a level for exact match
        LookLeft,           // went too far, backup
        LookRight,          // continue searching right
        Found,
    }

    struct SearchState<'e> {
        status: SearchStatus,
        node: Element<'e>,
        highlight_start: usize,     // if status is Found, then this is the offset within a leaf node
        highlight_end: usize,       // if status is Found, this is ignored
    }

    // save the current highlight state, set the state to be the end points so we can find the braille, then restore the state
    // FIX: this can fail if there is 8-dot braille
    use crate::interface::{get_preference, set_preference};
    let saved_highlight_style = get_preference("BrailleNavHighlight").unwrap();
    set_preference("BrailleNavHighlight", "EndPoints").unwrap();

    N_PROBES.with(|n| {*n.borrow_mut() = 0});
    // dive into the child of the <math> element (should only be one)
    let search_state = find_navigation_node(mathml, as_element(mathml.children()[0]), position)?;
    set_preference("BrailleNavHighlight", saved_highlight_style.as_str()).unwrap();

    // we know the attr value exists because it was found internally
    // FIX: what should be done if we never did the search?
    match search_state.status {
        SearchStatus::Found | SearchStatus::LookInParent => {
            return Ok( (search_state.node.attribute_value("id").unwrap().to_string(), search_state.highlight_start) )
        },
        _ => {
            // weird state -- return the entire expr
            match mathml.attribute_value("id") {
                None => bail!("'id' is not present on mathml: {}", mml_to_string(mathml)),
                Some(id) => return Ok( (id.to_string(), 0) ),
            }
        }
    } 

    /// find the navigation node that most tightly encapsulates the target position (0-based)
    /// 'node' is the current node we are on inside of 'mathml'
    fn find_navigation_node<'e>(mathml: Element<'e>, node: Element<'e>, target_position: usize) -> Result<SearchState<'e>> {
        let raw_node_id = node.attribute_value("id");
        let node_id = match raw_node_id.as_deref() {
            Some(id) => id,
            None => bail!("'id' is not present on mathml: {}", mml_to_string(node)),
        };
        N_PROBES.with(|n| {*n.borrow_mut() += 1});
        let (braille, char_start, char_end) = braille_mathml(mathml, node_id)?;
        let mut status = None;
        // debug!("find_navigation_node ({}, id={}): highlight=[{}, {});  target={}", name(node), node_id, char_start, char_end, target_position);
        if is_leaf(node) {
            if char_start == 0 && char_end == braille.len()/3 {
                // nothing highlighted -- probably invisible char not represented in braille -- continue looking to the right
                // debug!("  return due invisible char (?)' ");
                status = Some(SearchStatus::LookRight);
            } else if char_start <= target_position && target_position < char_end {
                // FIX: need to handle multi-char leaves and set the offset (char_start) appropriately
                // debug!("  return due to target_position inside leaf: {} <= {} < {}", char_start, target_position, char_end);
                return Ok( SearchState {
                    status: SearchStatus::Found,
                    node,
                    highlight_start: target_position - char_start,
                    highlight_end: 0,
                });
            } else if name(node) == "mo" {
                // if there is whitespace before or after the operator, consider the operator to be a match
                if (char_start > 0 && target_position == char_start - 1 && 
                    braille_at(&braille, 3*(char_start - 1)) == '⠀' && is_operator_that_adds_whitespace(node)) ||
                   (3*char_end < braille.len() && target_position == char_end &&
                    braille_at(&braille, 3*char_end) == '⠀' && is_operator_that_adds_whitespace(node)) {
                    return Ok( SearchState {
                        status: SearchStatus::Found,
                        node,
                        highlight_start: 0,
                        highlight_end: 0,
                    } );
                }
            }
        }
        if status.is_none() {
            if target_position < char_start {
                // debug!("  return due to target_position {} < start {}", target_position, char_start);
                status = Some(SearchStatus::LookLeft);
            } else if target_position >= char_end {
                // debug!("  return due to target_position {} >= end {}", target_position, char_end);
                status = Some(SearchStatus::LookRight);
            }
        }
        if let Some(status) = status {
            return Ok( SearchState {
                status,
                node,
                highlight_start: char_start,
                highlight_end: char_end,
            } );
        }

        let children = node.children();
        let mut i_left_child = 0;                         // inclusive
        let mut i_right_child = children.len();           // exclusive
        let mut call_start = char_start;
        let mut guess_fn: Box<dyn Fn(usize, usize, usize, usize) -> usize> = Box::new(|i_left, i_right, start, target: usize| guess_child_node_ltr(&children, i_left, i_right, start, target));
        while i_left_child < i_right_child {
            let i_guess_child = guess_fn(i_left_child, i_right_child, call_start, target_position);
            let status = find_navigation_node(mathml, as_element(children[i_guess_child]), target_position)?;
            // debug!("  in {} loop: status: {}, child: left/guess/right {}/({},{})/{}; highlight=[{}, {})", 
            //         name(node), status.status,
            //         i_left_child, i_guess_child, name(as_element(children[i_guess_child])),i_right_child,
            //         status.highlight_start, status.highlight_end);
            match status.status {
                SearchStatus::Found => {
                    return Ok(status);
                },
                SearchStatus::LookInParent => {
                    let (_, start, end) = braille_mathml(mathml, node_id)?;
                    // debug!("  parent ({}) braille: start/end={}/{};  target_position={}", name(node), start, end, target_position);
                    if start <= target_position && target_position < end {
                        // debug!("  ..found: id={}", node_id);
                        return Ok( SearchState{
                            status: SearchStatus::Found,
                            node,
                            highlight_start: 0,
                            highlight_end: 0,
                        } );      // done or look up another level
                    }
                    return Ok(status);  // look up a level
                },
                SearchStatus::LookLeft => {
                    i_right_child = if i_guess_child == 0 {0} else {i_guess_child};         // exclusive
                    call_start = status.highlight_start-1;
                    guess_fn = Box::new(|i_left, i_right, start, target| guess_child_node_rtl(&children, i_left, i_right, start, target));
                },
                SearchStatus::LookRight => {
                    i_left_child = i_guess_child+1;
                    call_start = status.highlight_end+1;
                    guess_fn = Box::new(|i_left, i_right, start, target| guess_child_node_ltr(&children, i_left, i_right, start, target));
                },
            }
        }
        // debug!("Didn't child in node {}: left/right={}/{};  target_position={}", name(node), i_left_child, i_right_child, target_position);

        // if we get here, we didn't find it in the children
        // debug!("..end of loop: look in parent of {} has start/end={}/{}", name(node), char_start, char_end);
        return Ok( SearchState{
            status: if char_start <= target_position && target_position <= char_end {SearchStatus::Found} else {SearchStatus::LookInParent},
            node,
            highlight_start: 0,
            highlight_end: 0,
        } );
    }

    fn is_operator_that_adds_whitespace(node: Element) -> bool {
        use crate::definitions::BRAILLE_DEFINITIONS;
        if PreferenceManager::get().borrow().pref_to_string("UseSpacesAroundAllOperators") == "true" {
            return true;
        } 

        return BRAILLE_DEFINITIONS.with(|definitions| {
            let definitions = definitions.borrow();
            let comparison_operators = definitions.get_hashset("ComparisonOperators").unwrap();
            return comparison_operators.contains(as_str!(as_text(node)));
        });        
    }

    /// look in children[i_left..i_right] for a count that exceeds target
    fn guess_child_node_ltr(children: &[ChildOfElement], i_left: usize, i_right: usize, start: usize, target: usize) -> usize {
        let mut estimated_position = start;
        // number of chars to add for number indicators
        let n_number_indicator = if PreferenceManager::get().borrow().pref_to_string("BrailleCode") == "Nemeth" {0} else {1};   // Nemeth doesn't typically need number or letter indicators
        #[allow(clippy::needless_range_loop)]  // I don't like enumerate/take/skip here
        for i in i_left..i_right {
            estimated_position += estimate_braille_chars(children[i], n_number_indicator);
            if estimated_position >= target {
                return i;
            }
        }
        return i_right-1;       // estimate was too large, return the last child as a guess
    }

    /// look in children[i_left..i_right].rev for a count that is less than target
    fn guess_child_node_rtl(children: &[ChildOfElement], i_left: usize, i_right: usize, start: usize, target: usize) -> usize {
        let mut estimated_position = start;
        let n_number_indicator = if PreferenceManager::get().borrow().pref_to_string("BrailleCode") == "Nemeth" {0} else {1};   // Nemeth doesn't typically need number or letter indicators
        for i in (i_left..i_right).rev() {
            estimated_position -= estimate_braille_chars(children[i], n_number_indicator);
            if estimated_position <= target {
                return i;
            }
        }
        return i_left;       // estimate was too small, return the first child as a guess
    }

    fn estimate_braille_chars(child: ChildOfElement, n_number_indicator: usize) -> usize {
        let node = as_element(child);
        let leaf_name = as_str!(name(node));
        if is_leaf(node) {
            let text = as_text(node);
            // len() is close since mn's probably have ASCII digits and lower case vars are common (count as) and other chars need extra braille chars
            // don't want to count invisible chars since they don't display and would give a length = 3
            if text == "\u{2061}" || text == "\u{2062}"  {       // invisible function apply/times (most common by far)
                return 0;
            }
            // We can't know what encoding the user is using for LaTeX and ASCIIMath -- assume 8-dot braille
            let braille_code = PreferenceManager::get().borrow().pref_to_string("BrailleCode");
            if braille_code == "LaTeX" || braille_code == "ASCIIMath" {
                return text.len();
            } else {    
                return match leaf_name {
                    "mn" => n_number_indicator + text.len(),
                    "mo" => 2,  // could do better by actually brailling char, but that is more expensive
                    _ => text.len(),
                }
            }
        }
        let mut estimate = if leaf_name == "mrow" {0} else {node.children().len() + 1};     // guess extra chars need for mfrac, msub, etc (start+intermediate+end).
        if leaf_name == "msup" || leaf_name == "msub" || leaf_name == "msubsup" {
            estimate -= 1;   // opening superscript/subscript indicator not needed
        }
        for child in node.children() {
            estimate += estimate_braille_chars(child, n_number_indicator);
        }
        // debug!("estimate_braille_chars for {}: {}", crate::canonicalize::element_summary(as_element(child)), estimate);
        return estimate;
    }
}

fn nemeth_cleanup(_pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String {
    // Typeface: S: sans-serif, B: bold, T: script/blackboard, I: italic, R: Roman
    // Language: E: English, D: German, G: Greek, V: Greek variants, H: Hebrew, U: Russian
    // Indicators: C: capital, N: number, P: punctuation, M: multipurpose
    // Others:
    //      W -- whitespace that should be kept (e.g, in a numeral)
    //      𝑁 -- hack for special case of a lone decimal pt -- not considered a number but follows rules mostly 
    // SRE doesn't have H: Hebrew or U: Russian, so not encoded (yet)
    // Note: some "positive" patterns find cases to keep the char and transform them to the lower case version
    static NEMETH_INDICATOR_REPLACEMENTS: phf::Map<&str, &str> = phf_map! {
        "S" => "⠠⠨",    // sans-serif
        "B" => "⠸",     // bold
        "𝔹" => "⠠⠸",     // blackboard
        "T" => "⠈",     // script
        "I" => "⠨",     // italic -- used for digits (not math letters)
        "R" => "",      // roman
        "E" => "⠰",     // English
        "D" => "⠸",     // German (Deutsche)
        "G" => "⠨",     // Greek
        "V" => "⠨⠈",    // Greek Variants
        "H" => "⠠⠠",    // Hebrew
        "U" => "⠈⠈",    // Russian
        "C" => "⠠",     // capital
        "P" => "⠸",     // punctuation
        "𝐏" => "⠸",     // hack for punctuation after a roman numeral -- never removed
        "L" => "",      // letter
        "l" => "",      // letter inside enclosed list
        "M" => "",      // multipurpose indicator
        "m" => "⠐",     // required multipurpose indicator
        "N" => "",      // potential number indicator before digit
        "n" => "⠼",     // required number indicator before digit
        "𝑁" => "",      // hack for special case of a lone decimal pt -- not considered a number but follows rules mostly
        "W" => "⠀",     // whitespace
        "w" => "⠀",     // whitespace from comparison operator
        "," => "⠠⠀",    // comma
        "b" => "⠐",     // baseline
        "𝑏" => "⣐",     // highlight baseline (it's a hack)
        "↑" => "⠘",     // superscript
        "↓" => "⠰",     // subscript
    };

    // Add an English Letter indicator. This involves finding "single letters".
    // The green book has a complicated set of cases, but the Nemeth UEB Rule book (May 2020), 4.10 has a much shorter explanation:
    //   punctuation or whitespace on the left and right ignoring open/close chars
    //   https://nfb.org/sites/www.nfb.org/files/files-pdf/braille-certification/lesson-4--provisional-5-9-20.pdf
    static ADD_ENGLISH_LETTER_INDICATOR: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?P<start>^|W|P.[\u2800-\u28FF]?|,)(?P<open>[\u2800-\u28FF]?⠷)?(?P<letter>C?L.)(?P<close>[\u2800-\u28FF]?⠾)?(?P<end>W|P|,|$)").unwrap()
    });
        
    // Trim braille spaces before and after braille indicators
    // In order: fraction, /, cancellation, letter, baseline
    // Note: fraction over is not listed due to example 42(4) which shows a space before the "/"
    static REMOVE_SPACE_BEFORE_BRAILLE_INDICATORS: LazyLock<Regex> = 
        LazyLock::new(|| Regex::new(r"(⠄⠄⠄|⠤⠤⠤⠤)[Ww]+([⠼⠸⠪])").unwrap());
    static REMOVE_SPACE_AFTER_BRAILLE_INDICATORS: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"([⠹⠻Llb])[Ww]+(⠄⠄⠄|⠤⠤⠤⠤)").unwrap());

    // Hack to convert non-numeric '.' to numeric '.'
    // The problem is that the numbers are hidden inside of mover -- this might be more general than rule 99_2.
    static DOTS_99_A_2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"𝑁⠨mN").unwrap());

    // Punctuation is one or two chars. There are (currently) only 3 2-char punct chars (—‘’) -- we explicitly list them below
    static REMOVE_SPACE_BEFORE_PUNCTUATION_151: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"w(P.[⠤⠦⠠]?|[\u2800-\u28FF]?⠾)").unwrap());
    static REMOVE_SPACE_AFTER_PUNCTUATION_151: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(P.[⠤⠦⠠]?|[\u2800-\u28FF]?⠷)w").unwrap());

    // Multipurpose indicator insertion
    // 149 -- consecutive comparison operators have no space -- instead a multipurpose indicator is used (doesn't require a regex)

    // 177.2 -- add after a letter and before a digit (or decimal pt) -- digits will start with N
    static MULTI_177_2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([Ll].)[N𝑁]").unwrap());

    // keep between numeric subscript and digit ('M' added by subscript rule)
    static MULTI_177_3: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([N𝑁].)M([N𝑁].)").unwrap());

    // Add after decimal pt for non-digits except for comma and punctuation
    // Note: since "." can be in the middle of a number, there is not necessarily a "N"
    // Although not mentioned in 177_5, don't add an 'M' before an 'm'
    static MULTI_177_5: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"([N𝑁]⠨)([^⠂⠆⠒⠲⠢⠖⠶⠦⠔N𝑁,Pm])").unwrap());

    // Pattern for rule II.9a (add numeric indicator at start of line or after a space)
    // 1. start of line
    // 2. optional minus sign (⠤)
    // 3. optional typeface indicator
    // 4. number (N)
    static NUM_IND_9A: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?P<start>^|[,Ww])(?P<minus>⠤?)N").unwrap());

    // Needed after section mark(§), paragraph mark(¶), #, or *
    static NUM_IND_9C: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(⠤?)(⠠⠷|⠠⠳|⠠⠈⠷)N").unwrap());

    // Needed after section mark(§), paragraph mark(¶), #, or *
    static NUM_IND_9D: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(⠈⠠⠎|⠈⠠⠏|⠨⠼|⠈⠼)N").unwrap());

    // Needed after a typeface change or interior shape modifier indicator
    static NUM_IND_9E: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?P<face>[SB𝔹TIR]+?)N").unwrap());
    static NUM_IND_9E_SHAPE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?P<mod>⠸⠫)N").unwrap());

    // Needed after hyphen that follows a word, abbreviation, or punctuation (caution about rule 11d)
    // Note -- hyphen might encode as either "P⠤" or "⠤" depending on the tag used
    static NUM_IND_9F: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([Ll].[Ll].|P.)(P?⠤)N").unwrap());

    // Enclosed list exception
    // Normally we don't add numeric indicators in enclosed lists (done in get_braille_nemeth_chars).
    // The green book says "at the start" of an item, don't add the numeric indicator.
    // The NFB list exceptions after function abbreviations and angles, but what this really means is "after a space"
    static NUM_IND_ENCLOSED_LIST: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"w([⠂⠆⠒⠲⠢⠖⠶⠦⠔⠴])").unwrap());

    // Punctuation chars (Rule 38.6 says don't use before ",", "hyphen", "-", "…")
    // Never use punctuation indicator before these (38-6)
    //      "…": "⠀⠄⠄⠄"
    //      "-": "⠸⠤" (hyphen and dash)
    //      ",": "⠠⠀"     -- spacing already added
    // Rule II.9b (add numeric indicator after punctuation [optional minus[optional .][digit]
    //  because this is run after the above rule, some cases are already caught, so don't
    //  match if there is already a numeric indicator
    static NUM_IND_9B: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?P<punct>P..?)(?P<minus>⠤?)N").unwrap());

    // Before 79b (punctuation)
    static REMOVE_LEVEL_IND_BEFORE_SPACE_COMMA_PUNCT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:[↑↓]+[b𝑏]?|[b𝑏])([Ww,P]|$)").unwrap());

    // Most commas have a space after them, but not when followed by a close quote (others?)
    static NO_SPACE_AFTER_COMMA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r",P⠴").unwrap()); // captures both single and double close quote
    static REMOVE_LEVEL_IND_BEFORE_BASELINE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:[↑↓mb𝑏]+)([b𝑏])").unwrap());

    // Except for the four chars above, the unicode rules always include a punctuation indicator.
    // The cases to remove them (that seem relevant to MathML) are:
    //   Beginning of line or after a space (V 38.1)
    //   After a word (38.4)
    //   2nd or subsequent punctuation (includes, "-", etc) (38.7)
    static REMOVE_AFTER_PUNCT_IND: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(^|[Ww]|[Ll].[Ll].)P(.)").unwrap());
    static REPLACE_INDICATORS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([SB𝔹TIREDGVHUP𝐏CLlMmb𝑏↑↓Nn𝑁Ww,])").unwrap());
    static COLLAPSE_SPACES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"⠀⠀+").unwrap());

//   debug!("Before:  \"{}\"", raw_braille);
    // replacements might overlap at boundaries (e.g., whitespace) -- need to repeat
    let mut start = 0;
    let mut result = String::with_capacity(raw_braille.len()+ raw_braille.len()/4);  // likely upper bound
    while let Some(matched) = ADD_ENGLISH_LETTER_INDICATOR.find_at(&raw_braille, start) {
        result.push_str(&raw_braille[start..matched.start()]);
        let replacement = ADD_ENGLISH_LETTER_INDICATOR.replace(
                &raw_braille[matched.start()..matched.end()], "${start}${open}E${letter}${close}");
        // debug!("matched='{}', start/end={}/{}; replacement: {}", &raw_braille[matched.start()..matched.end()], matched.start(), matched.end(), replacement);
        result.push_str(&replacement);
        // put $end back on because needed for next match (e.g., whitespace at end and then start of next match)
        // but it could also match because it was at the end, in which case "-1" is wrong -- tested after loop for that
        start = matched.end() - 1;
    }
    if !raw_braille.is_empty() && ( start < raw_braille.len()-1 || "WP,".contains(raw_braille.chars().nth_back(0).unwrap()) ) {       // see comment about $end above
        result.push_str(&raw_braille[start..]);
    }
//   debug!("ELIs:    \"{}\"", result);

    let result = NUM_IND_ENCLOSED_LIST.replace_all(&result, "wn${1}");

    // Remove blanks before and after braille indicators
    let result = REMOVE_SPACE_BEFORE_BRAILLE_INDICATORS.replace_all(&result, "$1$2");
    let result = REMOVE_SPACE_AFTER_BRAILLE_INDICATORS.replace_all(&result, "$1$2");

    let result = REMOVE_SPACE_BEFORE_PUNCTUATION_151.replace_all(&result, "$1");
    let result = REMOVE_SPACE_AFTER_PUNCTUATION_151.replace_all(&result, "$1");
//   debug!("spaces:  \"{}\"", result);

    let result = DOTS_99_A_2.replace_all(&result, "N⠨mN");

    // Multipurpose indicator
    let result = result.replace("ww", "m"); // 149
    let result = MULTI_177_2.replace_all(&result, "${1}m${2}");
    let result = MULTI_177_3.replace_all(&result, "${1}m$2");
    let result = MULTI_177_5.replace_all(&result, "${1}m$2");
//   debug!("MULTI:   \"{}\"", result);

    let result = NUM_IND_9A.replace_all(&result, "${start}${minus}n");
    // debug!("IND_9A:  \"{}\"", result);
    let result = NUM_IND_9C.replace_all(&result, "${1}${2}n");
    let result = NUM_IND_9D.replace_all(&result, "${1}n");
    let result = NUM_IND_9E.replace_all(&result, "${face}n");
    let result = NUM_IND_9E_SHAPE.replace_all(&result, "${mod}n");
    let result = NUM_IND_9F.replace_all(&result, "${1}${2}n");

//   debug!("IND_9F:  \"{}\"", result);

    // 9b: insert after punctuation (optional minus sign)
    // common punctuation adds a space, so 9a handled it. Here we deal with other "punctuation" 
    // FIX other punctuation and reference symbols (9d)
    let result = NUM_IND_9B.replace_all(&result, "$punct${minus}n");
//   debug!("A PUNCT: \"{}\"", &result);

    // strip level indicators
    // check first to remove level indicators before baseline, then potentially remove the baseline
    let mut result = REMOVE_LEVEL_IND_BEFORE_BASELINE.replace_all(&result, "$1");
//   debug!("Punct  : \"{}\"", &result);
    // checks for punctuation char, so needs to before punctuation is stripped.
    // if '𝑏' is removed, then the highlight needs to be shifted to the left in some cases
    let result = remove_baseline_before_space_or_punctuation(&mut result);
//   debug!("Removed: \"{}\"", &result);

    let result = NO_SPACE_AFTER_COMMA.replace_all(&result, "⠠P⠴");

    let result = REMOVE_AFTER_PUNCT_IND.replace_all(&result, "$1$2");
//   debug!("Punct38: \"{}\"", &result);

    let result = REPLACE_INDICATORS.replace_all(&result, |cap: &Captures| {
        match NEMETH_INDICATOR_REPLACEMENTS.get(&cap[0]) {
            None => {error!("REPLACE_INDICATORS and NEMETH_INDICATOR_REPLACEMENTS are not in sync"); ""},
            Some(&ch) => ch,
        }
    });

    // Remove unicode blanks at start and end -- do this after the substitutions because ',' introduces spaces
    let result = result.trim_start_matches('⠀').trim_end_matches('⠀');
    let result = COLLAPSE_SPACES.replace_all(result, "⠀");
   
    return result.to_string();

    fn remove_baseline_before_space_or_punctuation<'a>(braille: &'a mut Cow<'a, str>) -> Cow<'a, str> {
        // If the baseline highlight is at the end of the string and it is going to be deleted by the regex,
        //   then we need to shift the highlight to the left if what is to it's left is not whitespace (which should never be a highlight end)
        // This only happens when BrailleNavHighlight == "EndPoints".
        let highlight_style = PreferenceManager::get().borrow().pref_to_string("BrailleNavHighlight");
        if highlight_style == "EndPoints" &&
            let Some(last_highlighted) = braille.rfind(is_highlighted) &&
            braille[last_highlighted..].starts_with('𝑏') {
                    let i_after_baseline = last_highlighted + '𝑏'.len_utf8();
                    if i_after_baseline == braille.len() || braille[i_after_baseline..].starts_with(['W', 'w', ',', 'P']) {
                        // shift the highlight to the left after doing just the replacement (if any) that the regex below does
                        // the shift runs until a non blank braille char is found
                        let mut bytes_deleted = 0;
                        let mut char_to_highlight = "".to_string();   // illegal value
                        for ch in braille[..last_highlighted].chars().rev() {
                            bytes_deleted += ch.len_utf8();
                            if (0x2801..0x28FF).contains(&(ch as u32)) {
                                char_to_highlight = highlight(ch).to_string();
                                break;
                            }
                        }
                        braille.to_mut().replace_range(last_highlighted-bytes_deleted..last_highlighted+'𝑏'.len_utf8(),
                                                        &char_to_highlight);
                    }
                }
        return REMOVE_LEVEL_IND_BEFORE_SPACE_COMMA_PUNCT.replace_all(braille, "$1");

    }
}

// Typeface: S: sans-serif, B: bold, T: script/blackboard, I: italic, R: Roman
// Language: E: English, D: German, G: Greek, V: Greek variants, H: Hebrew, U: Russian
// Indicators: C: capital, N: number, P: punctuation, M: multipurpose
// Others:
//      W -- whitespace that should be kept (e.g, in a numeral)
//      𝑁 -- hack for special case of a lone decimal pt -- not considered a number but follows rules mostly 
// Note: some "positive" patterns find cases to keep the char and transform them to the lower case version
static UEB_INDICATOR_REPLACEMENTS: phf::Map<&str, &str> = phf_map! {
    "S" => "XXX",    // sans-serif -- from prefs
    "B" => "⠘",     // bold
    "𝔹" => "XXX",     // blackboard -- from prefs
    "T" => "⠈",     // script
    "I" => "⠨",     // italic -- used for digits (not math letters)
    "R" => "",      // roman
    // "E" => "⠰",     // English
    "1" => "⠰",      // Grade 1 symbol
    "𝟙" => "⠰⠰",     // Grade 1 word
    "L" => "",       // Letter left in to assist in locating letters
    "A" => "",       // Letter accent indicator -- should be followed by accent cells and then "L" for char
    "D" => "XXX",    // German (Deutsche) -- from prefs
    "G" => "⠨",      // Greek
    "V" => "⠨⠈",     // Greek Variants
    // "H" => "⠠⠠",  // Hebrew
    // "U" => "⠈⠈",  // Russian
    "C" => "⠠",      // capital
    "𝐶" => "⠠",      // capital that never should get word indicator (from chemical element)
    "N" => "⠼",     // number indicator
    "t" => "⠱",     // shape terminator
    "W" => "⠀",     // whitespace
    "𝐖"=> "⠀",     // whitespace (hard break -- basically, it separates exprs)
    "s" => "⠆",     // typeface single char indicator
    "w" => "⠂",     // typeface word indicator
    "e" => "⠄",     // typeface & capital terminator 
    "o" => "",       // flag that what follows is an open indicator (used for standing alone rule)
    "c" => "",       // flag that what follows is an close indicator (used for standing alone rule)
    "b" => "",       // flag that what follows is an open or close indicator (used for standing alone rule)
    "," => "⠂",     // comma
    "." => "⠲",     // period
    "-" => "-",     // hyphen
    "—" => "⠠⠤",   // normal dash (2014) -- assume all normal dashes are unified here [RUEB appendix 3]
    "―" => "⠐⠠⠤",  // long dash (2015) -- assume all long dashes are unified here [RUEB appendix 3]
    "#" => "",      // signals end of script
    // '(', '{', '[', '"', '\'', '“', '‘', '«',    // opening chars
    // ')', '}', ']', '\"', '\'', '”', '’', '»',           // closing chars
    // ',', ';', ':', '.', '…', '!', '?'                    // punctuation           

};

// static LETTERS: phf::Set<char> = phf_set! {
//     '⠁', '⠃', '⠉', '⠙', '⠑', '⠋', '⠛', '⠓', '⠊', '⠚', '⠅', '⠇', '⠍', 
//     '⠝', '⠕', '⠏', '⠟', '⠗', '⠎', '⠞', '⠥', '⠧', '⠺', '⠭', '⠽', '⠵',
// };

fn is_letter_number(ch: char) -> bool {
    matches!(ch, '⠁' | '⠃' | '⠉' | '⠙' | '⠑' | '⠋' | '⠛' | '⠓' | '⠊' | '⠚')
}

fn is_braille_char(ch: char) -> bool {
    ('\u{2800}'..='\u{28ff}').contains(&ch)
}

/// After an 'A' accent indicator, skip accent braille cells and return the index of the following 'L'.
/// Returns None if the accented-letter pattern is malformed.
fn index_after_accent_to_l(chars: &[char], after_a: usize) -> Option<usize> {
    let mut j = after_a;
    while j < chars.len() && is_braille_char(chars[j]) {
        j += 1;
    }
    if j < chars.len() && chars[j] == 'L' {
        Some(j)
    } else {
        None
    }
}

static SHORT_FORMS: phf::Set<&str> = phf_set! {
    "L⠁L⠃", "L⠁L⠃L⠧", "L⠁L⠉", "L⠁L⠉L⠗", "L⠁L⠋",
    "L⠁L⠋L⠝", "L⠁L⠋L⠺", "L⠁L⠛", "L⠁L⠛L⠌", "L⠁L⠇",
     "L⠁L⠇L⠍", "L⠁L⠇L⠗", "L⠁L⠇L⠞", "L⠁L⠇L⠹", "L⠁L⠇L⠺",
     "L⠃L⠇", "L⠃L⠗L⠇", "L⠉L⠙", "L⠙L⠉L⠇", "L⠙L⠉L⠇L⠛",
     "L⠙L⠉L⠧", "L⠙L⠉L⠧L⠛", "L⠑L⠊", "L⠋L⠗", "L⠋L⠌", "L⠛L⠙",
     "L⠛L⠗L⠞", "L⠓L⠍", "L⠓L⠍L⠋", "L⠓L⠻L⠋", "L⠊L⠍L⠍", "L⠇L⠇", "L⠇L⠗",
     "L⠍L⠽L⠋", "L⠍L⠡", "L⠍L⠌", "L⠝L⠑L⠉", "L⠝L⠑L⠊", "L⠏L⠙",
     "L⠏L⠻L⠉L⠧", "L⠏L⠻L⠉L⠧L⠛", "L⠏L⠻L⠓", "L⠟L⠅", "L⠗L⠉L⠧",
     "L⠗L⠉L⠧L⠛", "L⠗L⠚L⠉", "L⠗L⠚L⠉L⠛", "L⠎L⠙", "L⠎L⠡", "L⠞L⠙",
     "L⠞L⠛L⠗", "L⠞L⠍", "L⠞L⠝", "L⠭L⠋", "L⠭L⠎", "L⠽L⠗", "L⠽L⠗L⠋",
     "L⠽L⠗L⠧L⠎", "L⠮L⠍L⠧L⠎", "L⠡L⠝", "L⠩L⠙", "L⠹L⠽L⠋", "L⠳L⠗L⠧L⠎",
     "L⠺L⠙", "L⠆L⠉", "L⠆L⠋", "L⠆L⠓", "L⠆L⠇", "L⠆L⠝", "L⠆L⠎", "L⠆L⠞",
     "L⠆L⠽", "L⠒L⠉L⠧", "L⠒L⠉L⠧L⠛", "L⠐L⠕L⠋"
};

fn is_letter_prefix(ch: char) -> bool {
    matches!(ch, 'B' | 'I' | '𝔹' | 'S' | 'T' | 'D' | 'C' | '𝐶' | '𝑐' | 'A')
}

// Trim braille spaces before and after braille indicators
// In order: fraction, /, cancellation, letter, baseline
// Note: fraction over is not listed due to example 42(4) which shows a space before the "/"
// static ref REMOVE_SPACE_BEFORE_BRAILLE_INDICATORS: Regex =
//     Regex::new(r"(⠄⠄⠄|⠤⠤⠤)W+([⠼⠸⠪])").unwrap();
static REPLACE_INDICATORS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([1𝟙SB𝔹TIREDGVHP𝐶𝑐CLAMNW𝐖swe,.-—―#ocb])").unwrap());
static COLLAPSE_SPACES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"⠀⠀+").unwrap());

/// Transcriber-defined typeforms pulled from prefs (their braille is configurable), used by the
/// UEB-family indicator replacement for blackboard `𝔹`, sans-serif `S`, fraktur `D`, Greek variant `V`.
struct UserTypeforms {
    double_struck: String,
    sans_serif: String,
    fraktur: String,
    greek_variant: String,
}

impl UserTypeforms {
    /// Read the four typeforms from prefs named `<prefix>_DoubleStruck`, `<prefix>_SansSerif`, etc.
    fn from_prefs(pref_manager: &Ref<PreferenceManager>, prefix: &str) -> UserTypeforms {
        return UserTypeforms {
            double_struck: pref_manager.pref_to_string(&format!("{prefix}_DoubleStruck")),
            sans_serif: pref_manager.pref_to_string(&format!("{prefix}_SansSerif")),
            fraktur: pref_manager.pref_to_string(&format!("{prefix}_Fraktur")),
            greek_variant: pref_manager.pref_to_string(&format!("{prefix}_GreekVariant")),
        };
    }
}

/// Replace the symbolic indicator chars (matched by `regex`) with their braille cells using `map`,
/// pulling the transcriber-defined typeforms from `typeforms`. Shared by the UEB-family codes
/// (UEB, Vietnam, Finnish, Swedish). `map_name` is only used in the "out of sync" error message.
fn apply_indicator_replacements(
    raw_braille: &str,
    regex: &Regex,
    map: &phf::Map<&'static str, &'static str>,
    map_name: &str,
    typeforms: &UserTypeforms,
) -> String {
    return regex.replace_all(raw_braille, |cap: &Captures| {
        let matched_char = &cap[0];
        match matched_char {
            "𝔹" => typeforms.double_struck.as_str(),
            "S" => typeforms.sans_serif.as_str(),
            "D" => typeforms.fraktur.as_str(),
            "V" => typeforms.greek_variant.as_str(),
            _ => match map.get(matched_char) {
                None => {error!("REPLACE_INDICATORS and {map_name} are not in sync: missing '{matched_char}'"); ""},
                Some(&ch) => ch,
            },
        }
    }).to_string();
}

fn is_short_form(chars: &[char]) -> bool {
    let chars_as_string = chars.iter().map(|ch| ch.to_string()).collect::<String>();
    return SHORT_FORMS.contains(&chars_as_string);
}

fn ueb_cleanup(pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String {
    // debug!("ueb_cleanup: start={}", raw_braille);
    let result = typeface_to_word_mode(&raw_braille);
    let result = capitals_to_word_mode(&result);

    let use_only_grade1 = pref_manager.pref_to_string("UEB_START_MODE").as_str() == "Grade1";
    
    // 𝐖 is a unit hard-break. For GTM 1.7 it is still a symbols-sequence boundary (same as W),
    // so G1 passage decisions (1.7.3(b) / 1.7.5(c)) see the whole expression.
    let result = pick_start_mode(&result, use_only_grade1);

    let result = result.replace("tW", "W");

    // these typeforms need to get pulled from user-prefs as they are transcriber-defined
    let typeforms = UserTypeforms::from_prefs(&pref_manager, "UEB");
    let result = apply_indicator_replacements(&result, &REPLACE_INDICATORS, &UEB_INDICATOR_REPLACEMENTS,
        "UEB_INDICATOR_REPLACEMENTS", &typeforms);

    // Remove unicode blanks at start and end -- do this after the substitutions because ',' introduces spaces
    // let result = result.trim_start_matches('⠀').trim_end_matches('⠀');
    let result = COLLAPSE_SPACES.replace_all(&result, "⠀");
   
    return result.to_string();

    fn pick_start_mode(raw_braille: &str, use_only_grade1: bool) -> String {
        // Decide grade 1 indicator placement per ICEB GTM 1.7 (2025).
        // BANA's 2026 guidance adopts the same GTM §1.7 rules:
        //   https://www.brailleauthority.org/sites/default/files/2026-07/Guidance%20on%20Transcribing%20Math%20and%20Science%20in%20UEB%202026.pdf
        // GTM 1.7.3 decides indicators per *symbols-sequence* (RUEB 2.1 -- text separated by spaces):
        //   (a) allow one grade 1 symbol per symbols-sequence, or a grade 1 word indicator if a sequence needs more than one;
        //   (b) use a grade 1 passage only if three or more symbols-sequences each need a grade 1 symbol or word indicator.
        // Grade 1 symbol indicators forced by 'a-j' following a digit are not counted (GTM 1.7.3 note).
        // debug!("before determining mode:  '{}'", raw_braille);

        // Capital passage (RUEB §8.5): like word mode's extra 'C' (CC…), passage uses CCC…Ce.
        // Count G1 on the original (with capital markers) so standing-alone letters still force
        // passage when needed (BANA Ex 5-15), but apply mode changes after stripping C/𝐶 so
        // Grade 2 contractions are not blocked by cap_word_mode (chem "ch" → ⠡).
        let had_unit_break = raw_braille.contains('𝐖');
        let raw_braille = raw_braille.replace('𝐖', "W");
        let use_cap_passage = is_cap_passage_mode_good(&raw_braille);
        let stripped_caps = raw_braille.replace(['C', '𝐶'], "");
        let apply_to = if use_cap_passage { stripped_caps.as_str() } else { raw_braille.as_str() };
        let result = if use_only_grade1 {
            remove_unneeded_mode_changes(apply_to, UEB_Mode::Grade1, UEB_Duration::Passage)
        } else {
            gtm_1_7_mode(&raw_braille, apply_to, had_unit_break)
        };
        if use_cap_passage {
            return convert_to_cap_passage_mode(&result);
        }
        return result;

        /// Return true if capital passage mode should be used (RUEB §8.5 / BANA Ex 5-15).
        /// Requires no lowercase letters, and either ≥5 capital indicators (chem / dense caps)
        /// or ≥3 letter-bearing symbols-sequences that are fully capitalized.
        /// Grade 1 / numeric / other non-letter markers are transparent (do not abort the scan).
        fn is_cap_passage_mode_good(braille: &str) -> bool {
            let mut n_caps = 0;
            let mut n_letter_seqs = 0;
            let mut seq_has_letter = false;
            let mut is_cap_mode = false;
            let mut cap_mode = UEB_Duration::Symbol;    // real value set when is_cap_mode is set to true
            let mut chars = braille.chars();

            // look CL or CCL for caps (CC runs until we get whitespace)
            // if we find an L not in caps mode, we return false
            // Note: caps can be C𝐶, whitespace can be W𝐖
            while let Some(ch) = chars.next() {
                if ch == 'L' {
                    if !is_cap_mode {
                        return false;
                    }
                    seq_has_letter = true;
                    chars.next();       // skip letter
                    if cap_mode == UEB_Duration::Symbol {
                        is_cap_mode = false;
                    }
                } else if ch == 'C' || ch == '𝐶' {
                    if is_cap_mode {
                        if cap_mode == UEB_Duration::Symbol {
                            cap_mode = UEB_Duration::Word;
                        }
                    } else {
                        is_cap_mode = true;
                        cap_mode = UEB_Duration::Symbol;
                    }
                    n_caps += 1;
                } else if ch == 'W' || ch == '𝐖' {
                    if seq_has_letter {
                        n_letter_seqs += 1;
                        seq_has_letter = false;
                    }
                    is_cap_mode = false;
                }
                // else: '1', '𝟙', 'N', operators, typeforms, etc. — transparent
            }
            if seq_has_letter {
                n_letter_seqs += 1;
            }
            return n_caps >= 5 || n_letter_seqs >= 3;
        }

        /// After G1 placement: strip any leftover capital markers and wrap with intermediate
        /// `CCC`…`Ce` (same map as word mode: C→⠠, e→⠄ → ⠠⠠⠠…⠠⠄). Applied after
        /// remove_unneeded_mode_changes so parsers never treat passage opener as CL letters.
        /// If G1 passage is present (⠰⠰⠰…⠰⠄), capital passage goes inside (BANA Ex 5-15).
        fn convert_to_cap_passage_mode(braille: &str) -> String {
            let body = braille.replace(['C', '𝐶'], "");
            const G1_START: &str = "⠰⠰⠰";
            const G1_END: &str = "⠰⠄";
            if let Some(rest) = body.strip_prefix(G1_START)
                && let Some(mid) = rest.strip_suffix(G1_END) {
                    return format!("{G1_START}CCC{mid}Ce{G1_END}");
                }
            return format!("CCC{body}Ce");
        }

        /// Return true if the sequence of chars forces a '1' at the `i`th position
        /// Note: `chars[i]` should be '1'
        fn is_forced_grade1(chars: &[char], i: usize) -> bool {
            // A '1' is forced if 'a-j' follows a digit
            assert_eq!(chars[i], '1', "'is_forced_grade1' didn't start with '1'");
            // check that a-j follows the '1' -- we have '1Lx' where 'x' is the letter to check
            if i+2 < chars.len() && is_letter_number(unhighlight(chars[i+2])) {
                // check for a number before the '1'
                // this will be 'N' followed by LETTER_NUMBERS or the number ".", ",", or " "
                for j in (0..i).rev() {
                    let ch = chars[j];
                    if !(is_letter_number(unhighlight(ch)) || ".,W𝐖".contains(ch)) {
                        return ch == 'N'
                    }
                }
            }
            return false;
        }

        /// Count the number of non-forced grade 1 indicators needed for a single symbols-sequence.
        /// (Forced indicators -- 'a-j' following a digit -- are excluded per the GTM 1.7.3 note.)
        fn grade1_count(raw_word: &str) -> usize {
            let grade2 = remove_unneeded_mode_changes(raw_word, UEB_Mode::Grade2, UEB_Duration::Symbol);
            let chars = grade2.chars().collect::<Vec<char>>();
            return chars.iter().enumerate()
                .filter(|&(i, &ch)| ch == '1' && !is_forced_grade1(&chars, i))
                .count();
        }

        /// True if this symbols-sequence contains a literary English word (4+ letters).
        /// GTM 1.7.5: short math names (sin, cos, lim, min, …) and single-letter variables
        /// are not treated as words.
        fn sequence_contains_word(raw_word: &str) -> bool {
            let chars: Vec<char> = raw_word.chars().collect();
            let mut run = 0usize;
            let mut i = 0usize;
            while i < chars.len() {
                let ch = chars[i];
                if ch == 'L' {
                    run += 1;
                    i += 1;
                    if i < chars.len() {
                        i += 1; // skip the braille cell after 'L'
                    }
                    if run >= 4 {
                        return true;
                    }
                } else if ch == 'A' {
                    run += 1;
                    i += 1;
                    if run >= 4 {
                        return true;
                    }
                } else if is_math_alphabet_typeform(ch) {
                    // Double-struck/script/fraktur/sans-serif letters are not part of an English word.
                    let end = math_typeform_item_end(&chars, i);
                    i = if end > i { end } else { i + 1 };
                    run = 0;
                } else if matches!(ch, 'C' | '𝐶' | 'G' | 'V' | 'B' | 'I' | 's' | 'w' | 'e') {
                    i += 1; // letter prefixes / literary typeform — do not break the run
                } else {
                    run = 0;
                    i += 1;
                }
            }
            false
        }

        /// G2 form of a general-fraction sequence when two symbol indicators keep words
        /// contracted (GTM 1.7.5(a)). `None` if more than two G1 symbols would remain.
        fn g2_two_symbol_form(raw_word: &str) -> Option<String> {
            // Only the GTM 1.7.5(a) case: a general fraction whose numerator/denominator
            // contain literary words. Superscripts, radicals, etc. keep 1.7.3 word indicators.
            if !(raw_word.contains('⠷') && raw_word.contains("⠨⠌")) {
                return None;
            }
            let g2 = remove_unneeded_mode_changes(raw_word, UEB_Mode::Grade2, UEB_Duration::Symbol);
            let chars: Vec<char> = g2.chars().collect();
            let n_g1 = chars.iter().enumerate()
                .filter(|&(i, &ch)| ch == '1' && !is_forced_grade1(&chars, i))
                .count();
            if n_g1 <= 2 {
                Some(g2)
            } else {
                None
            }
        }

        /// ICEB GTM 1.7 grade 1 indicator placement (per symbols-sequence).
        /// GTM 1.7.3: one grade 1 symbol per sequence, or a word indicator if more than one.
        /// GTM 1.7.5(a): for a sequence containing word(s), allow two grade 1 symbol
        /// indicators when that keeps the words in their usual contracted form
        /// (e.g. ⠷…⠾ around work/distance). More than two still uses a word indicator.
        /// `count_src` decides passage vs word vs symbol; changes are applied to `apply_to`
        /// (may differ when capital markers were stripped for capital-passage mode).
        fn gtm_1_7_mode(count_src: &str, apply_to: &str, had_unit_break: bool) -> String {
            let count_words: Vec<&str> = count_src.split('W').collect();
            let apply_words: Vec<&str> = apply_to.split('W').collect();
            assert_eq!(count_words.len(), apply_words.len(),
                "gtm_1_7_mode: count_src and apply_to must have the same number of symbols-sequences");

            // Count how many symbols-sequences (whitespace-separated) need a non-forced grade 1 indicator.
            let n_seq_needing = count_words.iter().filter(|raw_word| grade1_count(raw_word) >= 1).count();

            // GTM 1.7.3(b) / 1.7.5(c): use a grade 1 passage if three or more sequences
            // each need grade 1. 1.7.5(c) starts the passage at the first such sequence so
            // preceding words (e.g. "speed") stay contracted. That delayed start is used when
            // spaced units (𝐖) joined the expression; otherwise a passage wraps the whole
            // expression (chem, roots, labelled equations).
            if n_seq_needing >= 3 {
                let first = count_words.iter().position(|w| grade1_count(w) >= 1)
                    .expect("n_seq_needing >= 3 implies a sequence that needs grade 1");
                let delay_start = had_unit_break && first > 0;
                if delay_start {
                    let prefix: Vec<String> = apply_words[..first].iter()
                        .map(|w| remove_unneeded_mode_changes(w, UEB_Mode::Grade2, UEB_Duration::Symbol))
                        .collect();
                    let rest = apply_words[first..].join("W");
                    let passage = remove_unneeded_mode_changes(&rest, UEB_Mode::Grade1, UEB_Duration::Passage);
                    let mut out = prefix.join("W");
                    if !out.is_empty() {
                        out.push('W');
                    }
                    out.push_str("⠰⠰⠰");
                    out.push_str(&passage);
                    out.push_str("⠰⠄");
                    return out;
                }
                let grade1_passage = remove_unneeded_mode_changes(apply_to, UEB_Mode::Grade1, UEB_Duration::Passage);
                return "⠰⠰⠰".to_string() + &grade1_passage + "⠰⠄";
            }

            // GTM 1.7.3(a): per sequence, allow one grade 1 symbol indicator, or a grade 1 word
            // indicator if the sequence needs more than one.
            let words = count_words.iter().zip(apply_words.iter())
                .map(|(&count_word, &apply_word)| {
                    let n_g1 = grade1_count(count_word);
                    if n_g1 >= 2 {
                        // GTM 1.7.5(a): two symbol indicators if that preserves contractions
                        if sequence_contains_word(count_word)
                            && let Some(g2) = g2_two_symbol_form(apply_word) {
                                g2
                            } else {
                                "⠰⠰".to_string() + &remove_unneeded_mode_changes(apply_word, UEB_Mode::Grade1, UEB_Duration::Word)
                            }
                    } else {
                        // 0 or 1 grade 1 indicators: the grade 2 form leaves a single symbol indicator inline
                        remove_unneeded_mode_changes(apply_word, UEB_Mode::Grade2, UEB_Duration::Symbol)
                    }
                })
                .collect::<Vec<String>>();
            return words.join("W");
        }
    }
}

fn typeface_to_word_mode(braille: &str) -> String {
    static HAS_TYPEFACE: LazyLock<Regex> = LazyLock::new(|| Regex::new("[BI𝔹STD]").unwrap());
    // debug!("before typeface fix:  '{}'", braille);

    let mut result = "".to_string();
    let chars = braille.chars().collect::<Vec<char>>();
    let mut word_mode = Vec::with_capacity(5);
    let mut word_mode_end = Vec::with_capacity(5);
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        if HAS_TYPEFACE.is_match(ch.to_string().as_str()) {
            let i_next_char_target = find_next_char(&chars[i+1..], ch);
            if word_mode.contains(&ch) {
                if i_next_char_target.is_none() {
                    word_mode.retain(|&item| item!=ch);  // drop the char since word mode is done
                    word_mode_end.push(ch);   // add the char to signal to add end sequence
                }
            } else {
                result.push(ch);
                if i_next_char_target.is_some() {
                    result.push('w');     // typeface word indicator
                    word_mode.push(ch);      // starting word mode for this char
                } else {
                    result.push('s');     // typeface single char indicator
                }
            }
            i += 1; // eat "B", etc
        } else if ch == 'L' || ch == 'N' {
            result.push(chars[i]);
            result.push(chars[i+1]);
            if !word_mode_end.is_empty() && i+2 < chars.len() && !(chars[i+2] == 'W'|| chars[i+2] == '𝐖') {
                // add terminator unless word sequence is terminated by end of string or whitespace
                for &ch in &word_mode_end {
                    result.push(ch);
                    result.push('e');
                };
                word_mode_end.clear();
            }
            i += 2; // eat Ll/Nd
        } else {
            result.push(ch);
            i += 1;
        }
    }
    return result;

}

fn capitals_to_word_mode(braille: &str) -> String {
    use std::iter::FromIterator;
    // debug!("before capitals fix:  '{}'", braille);

    let mut result = "".to_string();
    let chars = braille.chars().collect::<Vec<char>>();
    let mut is_word_mode = false;
    let mut i = 0;
    // look for a sequence of CLxCLy... and create CCLxLy...
    while i < chars.len() {
        let ch = chars[i];
        if ch == 'C' {
            // '𝑐' should only occur after a 'C', so we don't have top-level check for it
            let mut next_non_cap = i+1;
            while let Some(i_next) = find_next_char(&chars[next_non_cap..], '𝑐') {
                next_non_cap += i_next + 1; // C/𝑐, L, letter
            }
            if find_next_char(&chars[next_non_cap..], 'C').is_some() { // next letter sequence "C..."
                if is_next_char_start_of_section_12_modifier(&chars[next_non_cap+1..]) {
                    // to me this is tricky -- section 12 modifiers apply to the previous item
                    // the last clause of the "item" def is the previous indivisible symbol" which ICEB 2.1 say is:
                    //   braille sign: one or more consecutive braille characters comprising a unit,
                    //     consisting of a root on its own or a root preceded by one or more
                    //     prefixes (also referred to as braille symbol)
                    // this means the capital indicator needs to be stated and can't be part of a word or passage
                    is_word_mode = false;
                    result.push_str(String::from_iter(&chars[i..next_non_cap]).as_str());
                    i = next_non_cap;
                    continue;
                }
                if is_word_mode {
                    i += 1;     // skip the 'C'
                } else {
                    // start word mode -- need an extra 'C'
                    result.push('C');
                    is_word_mode = true;
                }
            } else if is_word_mode {
                i += 1;         // skip the 'C'
            }
            if chars[next_non_cap] == 'G' {
                // Greek letters are a bit exceptional in that the pattern is "CGLx" -- bump 'i'
                next_non_cap += 1;
            }
            if chars[next_non_cap] == 'A' {
                // Accented capital: CA...Lx
                match index_after_accent_to_l(&chars, next_non_cap + 1) {
                    Some(i_l) => next_non_cap = i_l,
                    None => {
                        error!("capitals_to_word_mode: internal error: didn't find L after CA in '{}'.",
                               chars[i..].iter().take(8).collect::<String>());
                    }
                }
            }
            if chars[next_non_cap] != 'L' {
                error!("capitals_to_word_mode: internal error: didn't find L after C in '{}'.",
                       chars[i..next_non_cap+2].iter().collect::<String>().as_str());
            }
            let i_braille_char = next_non_cap + 2;
            result.push_str(String::from_iter(&chars[i..i_braille_char]).as_str());
            i = i_braille_char;
        } else if ch == 'A' {
            // Lowercase accented letter: A...Lx
            if is_word_mode {
                result.push('e');
                is_word_mode = false;
            }
            match index_after_accent_to_l(&chars, i + 1) {
                Some(i_l) => {
                    let i_braille_char = i_l + 2;
                    result.push_str(String::from_iter(&chars[i..i_braille_char]).as_str());
                    i = i_braille_char;
                }
                None => {
                    error!("capitals_to_word_mode: internal error: didn't find L after A in '{}'.",
                           chars[i..].iter().take(8).collect::<String>());
                    result.push(ch);
                    i += 1;
                }
            }
        } else if ch == 'L' {       // must be lowercase -- uppercase consumed above
            // assert!(LETTERS.contains(&unhighlight(chars[i+1]))); not true for other alphabets
            if is_word_mode {
                result.push('e');       // terminate Word mode (letter after caps)
                is_word_mode = false;
            }
            result.push('L');
            result.push(chars[i+1]);
            i += 2; // eat L, letter
        } else {
            is_word_mode = false;   // non-letters terminate cap word mode
            result.push(ch);
            i += 1;
        }
    }
    return result;

    fn is_next_char_start_of_section_12_modifier(chars: &[char]) -> bool {
        // first find the L and eat the char so that we are at the potential start of where the target lies
        let chars_len = chars.len();
        let mut i_cap = 0;
        while chars[i_cap] != 'C' {     // we know 'C' is in the string, so no need to check for exceeding chars_len
            i_cap += 1;
        }
        for i_end in i_cap+1..chars_len {
            if chars[i_end] == 'L' {
                // skip the next char to get to the real start, and then look for the modifier string or next L/N
                // debug!("   after L '{}'", chars[i_end+2..].iter().collect::<String>());
                for i in i_end+2..chars_len {
                    let ch = chars[i];
                    if ch == '1' {
                        // Fix: there's probably a much better way to check if we have a match against one of "⠱", "⠘⠱", "⠘⠲", "⠸⠱", "⠐⠱ ", "⠨⠸⠱"
                        if chars[i+1] == '⠱' {
                            return true;
                        } else if i+2 < chars_len {
                            let mut str = chars[i+1].to_string();
                            str.push(chars[i+2]);
                            if str == "⠘⠱" || str == "⠘⠲" || str == "⠸⠱" || str == "⠐⠱" {
                                return true;
                            } else if i+3 < chars_len {
                                str.push(chars[i+3]);
                                return str == "⠨⠸⠱";
                            }
                            return false;
                        }
                    }
                    if ch == 'L' || ch == 'N' || !is_letter_prefix(ch) {
                        return false;
                    }
                }
            }
        }
        return false;
    }    
}

fn find_next_char(chars: &[char], target: char) -> Option<usize> {        
    // first find the L or N and eat the char so that we are at the potential start of where the target lies
    // debug!("Looking for '{}' in '{}'", target, chars.iter().collect::<String>());
    for i_end in 0..chars.len() {
        if chars[i_end] == 'L' || chars[i_end] == 'N' {
            // skip the next char to get to the real start, and then look for the target
            // stop when L/N signals past potential target or we hit some non L/N char (actual braille)
            // debug!("   after L/N '{}'", chars[i_end+2..].iter().collect::<String>());
            for (i, &ch) in chars.iter().enumerate().skip(i_end+2) {
                if ch == 'L' || ch == 'N' || !is_letter_prefix(ch) {
                    return None;
                } else if ch == target {
                    // debug!("   found target");
                    return Some(i);
                }
            }
        }
    }
    return None;
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum UEB_Mode {
    Numeric,        // also includes Grade1
    Grade1,
    Grade2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum UEB_Duration {
    // Standing alone: A braille symbol that is standing alone may have a contracted (grade 2) meaning.
    // A letter or unbroken sequence of letters is “standing alone” if the symbols before and after the letter or
    //   sequence are spaces, hyphens, dashes or any combination thereof, including some common punctuation.
    // Item: An “item” is defined as the next symbol or one of seven groupings listed in Rules of Unified English Braille, §11.4.1.
    Symbol,

    // The grade 1 word indicator sets grade 1 mode for the next word or symbol sequence.
    // A symbol sequence in UEB is defined as an unbroken string of braille signs,
    //   whether alphabetic or non-alphabetic, preceded and followed by a space.
    Word,
    Passage,
}

// used to determine standing alone (on left side)
fn is_left_intervening_char(ch: char) -> bool {
    matches!(ch, 'B' | 'I' | '𝔹' | 'S' | 'T' | 'D' | 'C' | '𝐶' | 's' | 'w')
}

/// Double-struck / script / fraktur / sans-serif: math alphabets, not literary emphasis.
fn is_math_alphabet_typeform(ch: char) -> bool {
    matches!(ch, '𝔹' | 'T' | 'D' | 'S')
}

/// Advance past one letter with optional capital, Greek, and accent prefixes (`C? G|V? A…? L cell`).
fn skip_one_prefixed_letter(chars: &[char], mut i: usize) -> usize {
    if i >= chars.len() {
        return i;
    }
    if matches!(chars[i], 'C' | '𝐶') {
        i += 1;
    }
    if i < chars.len() && matches!(chars[i], 'G' | 'V') {
        i += 1;
    }
    if i < chars.len() && chars[i] == 'A' {
        match index_after_accent_to_l(chars, i + 1) {
            Some(i_l) => i = i_l,
            None => return i,
        }
    }
    if i < chars.len() && chars[i] == 'L' {
        i += 1;
        if i < chars.len() {
            i += 1; // braille cell
        }
    }
    i
}

/// End index of a math-alphabet typeform item starting at `i` (`𝔹`/`T`/`D`/`S`).
/// `s` covers one letter; `w` covers the typeform word until its terminator.
fn math_typeform_item_end(chars: &[char], i: usize) -> usize {
    if i >= chars.len() || !is_math_alphabet_typeform(chars[i]) {
        return i;
    }
    let typeform = chars[i];
    let mut j = i + 1;
    let word_mode = j < chars.len() && chars[j] == 'w';
    if j < chars.len() && matches!(chars[j], 's' | 'w') {
        j += 1;
    }
    if word_mode {
        loop {
            let next = skip_one_prefixed_letter(chars, j);
            if next == j {
                break;
            }
            j = next;
        }
        if j + 1 < chars.len() && chars[j] == typeform && chars[j + 1] == 'e' {
            j += 2;
        } else if j < chars.len() && chars[j] == 'e' {
            j += 1;
        }
        return j;
    }
    return skip_one_prefixed_letter(chars, j);
}

/// RUEB 10.6.2: lower groupsigns "be", "con", "dis" only at the beginning of a word.
/// Beginning of a word = letters-sequence after space, hyphen or dash, optionally with
/// intervening punctuation/indicators from RUEB 2.6.2 (opening brackets/quotes, typeform,
/// capitals). Math operators and fraction indicators are not word boundaries.
/// `word_start` is the index of the first char of a grade-2 letter run ('L', 'C', or 'A').
fn allows_lower_word_sign_at(chars: &[char], word_start: usize) -> bool {
    if word_start >= chars.len() {
        return false;
    }
    if word_start == 0 {
        return true;
    }
    let mut j = word_start;
    while j > 0 {
        j -= 1;
        let ch = chars[j];
        // 2.6.2 intervening symbols (opening punctuation flags, typeform, capitals, etc.)
        if is_left_intervening_char(ch) || matches!(ch, 'e' | 'c' | 's' | 'w' | 'o' | 'b' | 'G' | 'V') {
            continue;
        }
        // Must be space, hyphen or dash (RUEB 10.6.2 Note)
        return "W𝐖-—―".contains(ch);
    }
    true
}

/// Return value for use_g1_word_mode()
#[derive(Debug, PartialEq)]
enum Grade1WordIndicator {
    NotInWord,        // no '𝟙' in the current/next word
    InWord,           // '𝟙' in the current/next word
    NotInChars,       // no '𝟙' in the entire string (optimization for common case)
}

fn remove_unneeded_mode_changes(raw_braille: &str, start_mode: UEB_Mode, start_duration: UEB_Duration) -> String {
    // FIX: need to be smarter about moving on wrt to typeforms/typefaces, caps, bold/italic. [maybe just let them loop through the default?]
    let mut mode = start_mode;
    let mut duration = start_duration;
    let mut start_g2_letter = None;    // used for start of contraction checks
    let mut i_g2_start = None;  // set to 'i' when entering G2 mode; None in other modes. '1' indicator goes here if standing alone
    let mut cap_word_mode = false;     // only set to true in G2 to prevent contractions
    let mut result = String::default();
    let chars = raw_braille.chars().collect::<Vec<char>>();
    let mut g1_word_indicator = Grade1WordIndicator::NotInChars;        // almost always true (and often irrelevant)
    if mode == UEB_Mode::Grade2 || duration == UEB_Duration::Symbol {
        g1_word_indicator = use_g1_word_mode(&chars);
        if g1_word_indicator == Grade1WordIndicator::InWord {
            mode = UEB_Mode::Grade1;
            if duration == UEB_Duration::Symbol {
                duration = UEB_Duration::Word;     // if Passage mode, leave as is
                result.push('𝟙')
            }
        }
    }
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        match mode {
            UEB_Mode::Numeric => {
                // Numeric Mode: (from https://uebmath.aphtech.org/lesson1.0 and lesson4.0)
                // Symbols that can appear within numeric mode include the ten digits, comma, period, simple fraction line,
                // line continuation indicator, and numeric space digit symbols.
                // A space or any other symbol not listed here terminates numeric mode.
                // Numeric mode is also terminated by the "!" -- used after a script
                //
                // The numeric indicator also turns on grade 1 mode.
                // When grade 1 mode is set by the numeric indicator,
                //   grade 1 indicators are not used unless a single lower-case letter a-j immediately follows a digit.
                // Grade 1 mode when set by the numeric indicator is terminated by a space, hyphen, dash, or a grade 1 indicator.
                i_g2_start = None;
                // debug!("Numeric: ch={}, duration: {:?}", ch, duration);
                match ch {
                    'L' => {
                        // terminate numeric mode -- duration doesn't change
                        // let the default case handle pushing on the chars for the letter
                        if is_letter_number(unhighlight(chars[i+1])) {
                            result.push('1');   // need to distinguish a-j from a digit
                        }
                        result.push(ch);
                        i += 1;
                        mode = UEB_Mode::Grade1;
                        // duration remains Word
                    },
                    '1' | '𝟙' => {
                        // numeric mode implies grade 1, so don't output indicator;
                        i += 1;
                        mode = UEB_Mode::Grade1;
                        if start_duration == UEB_Duration::Passage {
                            duration = UEB_Duration::Passage;      // otherwise it remains at Word
                        }
                    },
                    '#' => {
                        // terminate numeric mode -- duration doesn't change
                        i += 1;
                        if i+1 < chars.len() && chars[i] == 'L' && is_letter_number(unhighlight(chars[i+1])) {
                            // special case where the script was numeric and a letter follows, so need to put out G1 indicator
                            result.push('1');
                            // the G1 case should work with 'L' now
                        }
                        mode = UEB_Mode::Grade1;
                    },
                    'N' => {
                        // stay in the same mode (includes numeric "," and "." space) -- don't let default get these chars
                        result.push(chars[i+1]);
                        i += 2;
                    },
                    _ => {
                        // moving out of numeric mode
                        result.push(ch);
                        i += 1;
                        if "W𝐖-—―".contains(ch) {
                            mode = start_mode;
                            if mode == UEB_Mode::Grade2 {
                                start_g2_letter = None;        // will be set to real letter
                            }
                            if start_duration != UEB_Duration::Passage {
                                duration = UEB_Duration::Symbol;
                            }
                        } else {
                            mode = UEB_Mode::Grade1
                        }
                    },
                }
            },
            UEB_Mode::Grade1 => {
                // Grade 1 Mode:
                // The numeric indicator also sets grade 1 mode.
                // Grade 1 mode, when initiated by the numeric indicator, is terminated by a space, hyphen, dash or grade 1 terminator.
                // Grade 1 mode is also set by grade 1 indicators.
                i_g2_start = None;
                // debug!("Grade 1: ch={}, duration: {:?}", ch, duration);
                match ch {
                    'L' => {
                        // note: be aware of '#' case for Numeric because '1' might already be generated
                        // let the default case handle pushing on the chars for the letter
                        result.push(ch);
                        i += 1;
                    },
                    'A' => {
                        // Accented letter: keep A...Lx together so accent cells don't end G1 symbol mode
                        match index_after_accent_to_l(&chars, i + 1) {
                            Some(i_l) => {
                                result.extend(chars[i..i_l + 2].iter().copied()); // A + accents + L + letter
                                i = i_l + 2;
                            }
                            None => {
                                result.push(ch);
                                i += 1;
                            }
                        }
                    },
                    '1' | '𝟙' => {
                        assert!(ch == '1' || duration != UEB_Duration::Symbol);     // if '𝟙', should be Word or Passage duration
                        // nothing to do -- let the default case handle the following chars
                        i += 1;
                    },
                    'N' => {
                        result.push(ch);
                        result.push(chars[i+1]);
                        i += 2;
                        mode = UEB_Mode::Numeric;
                        duration = UEB_Duration::Word;
                    },
                    'W' | '𝐖' => {
                        // this terminates a word mode if there was one
                        result.push(ch);
                        i += 1;
                        if start_duration != UEB_Duration::Passage {
                            duration = UEB_Duration::Symbol;
                            mode = UEB_Mode::Grade2;
                        }
                    },
                    _ => {
                        result.push(ch);
                        i += 1;
                        if duration == UEB_Duration::Symbol && !is_letter_prefix(ch) {
                            mode = start_mode;
                        }
                    }
                }
                if mode == UEB_Mode::Grade2 {
                    start_g2_letter = None;        // will be set to real letter
                }

            },
            UEB_Mode::Grade2 => {
                // note: if we ended up using a '1', it only extends to the next char, which is also dealt with, so mode doesn't change
               if i_g2_start.is_none() {
                   i_g2_start = Some(i);
                   cap_word_mode = false;
               }
                // debug!("Grade 2: ch={}, duration: {:?}", ch, duration);
                match ch {
                    '𝔹' | 'T' | 'D' | 'S' => {
                        // Math-alphabet typeform: this letter is not part of an English word
                        // (GTM 11.6). Close any current contraction run, emit the typeform
                        // item on its own, then let following letters start a new word.
                        if let Some(start) = start_g2_letter {
                            if !cap_word_mode {
                                result = handle_contractions(&chars, start, i, result);
                            }
                            cap_word_mode = false;
                            start_g2_letter = None;
                        }
                        let unit_end = math_typeform_item_end(&chars, i);
                        // Typeform prefix (`𝔹s`) must precede the grade 1 indicator: ⠈⠆⠰⠠⠗, not ⠰⠈⠆⠠⠗.
                        let mut letter_start = i + 1;
                        if letter_start < unit_end && matches!(chars[letter_start], 's' | 'w') {
                            letter_start += 1;
                        }
                        result.extend(chars[i..letter_start].iter().copied());
                        if let Some(letter_idx) = chars[i..unit_end].iter().position(|&c| c == 'L') {
                            let (is_alone, _, n_letters) = stands_alone(&chars, i + letter_idx);
                            if is_alone && n_letters == 1 {
                                result.push('1');
                                mode = UEB_Mode::Grade1;
                            }
                        }
                        result.extend(chars[letter_start..unit_end].iter().copied());
                        i = unit_end;
                    },
                    'L' => {
                        if start_g2_letter.is_none() {
                            start_g2_letter = Some(i);
                        }
                        let (is_alone, right_matched_chars, n_letters) = stands_alone(&chars, i);
                        // GTM 1.2.1 says we only need to use G1 for single letters or sequences that are a shortform (e.g, "ab")
                        if is_alone && (n_letters == 1 || is_short_form(&right_matched_chars[..2*n_letters])) {
                            // debug!("  is_alone -- pushing '1'");
                            result.push('1');
                            mode = UEB_Mode::Grade1;
                        }
                        // debug!("  pushing {:?}", right_matched_chars);
                        right_matched_chars.iter().for_each(|&ch| result.push(ch));
                        i += right_matched_chars.len();
                    },
                    'C' => {
                        // Want 'C' before 'L' (or 'CA...L' / 'CGL'). Could be CC for word cap -- if so, eat it and move on
                        // Note: guaranteed that there is a char after the 'C', so chars[i+1] is safe
                        if chars[i+1] == 'C' {
                            cap_word_mode = true;
                            i += 1;
                        } else {
                            let mut letter_idx = i + 1;
                            let is_greek = chars[letter_idx] == 'G';
                            if is_greek {
                                letter_idx += 1;
                            }
                            if letter_idx < chars.len() && chars[letter_idx] == 'A' {
                                match index_after_accent_to_l(&chars, letter_idx + 1) {
                                    Some(i_l) => letter_idx = i_l,
                                    None => {
                                        error!("Internal error: 'CA' without L at index={i} in '{raw_braille}'");
                                        result.push('C');
                                        i += 1;
                                        continue;
                                    }
                                }
                            }
                            let (is_alone, right_matched_chars, n_letters) = stands_alone(&chars, letter_idx);
                            // GTM 1.2.1 says we only need to use G1 for single letters or sequences that are a shortform (e.g, "ab")
                            if is_alone && (n_letters == 1 || is_short_form(&right_matched_chars[..2*n_letters])) {
                                // debug!("  is_alone -- pushing '1'");
                                result.push('1');
                                mode = UEB_Mode::Grade1;
                            }
                            if cap_word_mode {
                                result.push('C');   // first 'C' if cap word
                            }
                            result.push('C');
                            if is_greek {
                                result.push('G');
                            }
                            start_g2_letter = Some(i);
                            // right_matched starts at A or L (not including C/G)
                            right_matched_chars.iter().for_each(|&ch| result.push(ch));
                            i += 1 + if is_greek {1} else {0} + right_matched_chars.len();
                        }
                    },
                    'A' => {
                        // Accented lowercase letter: A...Lx
                        match index_after_accent_to_l(&chars, i + 1) {
                            Some(i_l) => {
                                if start_g2_letter.is_none() {
                                    start_g2_letter = Some(i);
                                }
                                let (is_alone, right_matched_chars, n_letters) = stands_alone(&chars, i_l);
                                if is_alone && (n_letters == 1 || is_short_form(&right_matched_chars[..2*n_letters])) {
                                    result.push('1');
                                    mode = UEB_Mode::Grade1;
                                }
                                right_matched_chars.iter().for_each(|&ch| result.push(ch));
                                i += right_matched_chars.len();
                            }
                            None => {
                                error!("Internal error: 'A' without L at index={i} in '{raw_braille}'");
                                result.push(ch);
                                i += 1;
                            }
                        }
                    },
                    '1' => {
                        // Contract the preceding letter run before consuming '1'. The '1' arm used
                        // to increment first, so handle_contractions saw "work1" and the whole-word
                        // patterns `^work$` / `^time$` failed (GTM 1.7.9 work/distance, 1.7.5(a) time).
                        if let Some(start) = start_g2_letter {
                            if !cap_word_mode {
                                result = handle_contractions(&chars, start, i, result);
                            }
                            cap_word_mode = false;
                            start_g2_letter = None;
                        }
                        result.push(ch);
                        i += 1;
                        mode = UEB_Mode::Grade1;
                        duration = UEB_Duration::Symbol;
                    },
                    '𝟙' => {
                        // '𝟙' should have forced G1 Word mode
                        error!("Internal error: '𝟙' found in G2 mode: index={i} in '{raw_braille}'");
                        i += 1;
                    }
                    'N' => {
                        result.push(ch);
                        result.push(chars[i+1]);
                        i += 2;
                        mode = UEB_Mode::Numeric;
                        duration = UEB_Duration::Word;
                    },
                    _ => {
                        if let Some(start) = start_g2_letter {
                            if !cap_word_mode {
                                result = handle_contractions(&chars, start, i, result);
                            }
                            cap_word_mode = false;
                            start_g2_letter = None;     // not start of char sequence
                        }
                        result.push(ch);
                        i += 1;
                        if !is_left_intervening_char(ch) {
                            cap_word_mode = false;
                            i_g2_start = Some(i);
                        }

                    }
                }
                if mode != UEB_Mode::Grade2 && !cap_word_mode &&
                   let Some(start) = start_g2_letter {
                        result = handle_contractions(&chars, start, i, result);
                        start_g2_letter = None;     // not start of char sequence
                    }
            },
        }

        if (ch == 'W' || ch == '𝐖') && g1_word_indicator != Grade1WordIndicator::NotInChars &&
           (mode == UEB_Mode::Grade2 || duration == UEB_Duration::Symbol) {
            g1_word_indicator = use_g1_word_mode(&chars[i..]);
            if g1_word_indicator == Grade1WordIndicator::InWord {
                mode = UEB_Mode::Grade1;
                if duration == UEB_Duration::Symbol {
                    duration = UEB_Duration::Word;     // if Passage mode, leave as is
                    result.push('𝟙')
                }
            }
        }
    }
    if mode == UEB_Mode::Grade2 &&
       let Some(start) = start_g2_letter {
            result = handle_contractions(&chars, start, i, result);
        }

    return result;


    fn use_g1_word_mode(chars: &[char]) -> Grade1WordIndicator {
        // debug!("use_g1_word_mode: chars='{:?}'", chars);
        for &ch in chars {
            if ch == 'W' || ch == '𝐖' {
                return Grade1WordIndicator::NotInWord;       // reached a word boundary
            }
            if ch == '𝟙' {
                return Grade1WordIndicator::InWord;        // need word mode in this "word"
            }
        }
        return Grade1WordIndicator::NotInChars;               // 
    }
}

/// Returns a tuple:
///   true if the ith char "stands alone" (UEB 2.6)
///   the chars on the right that are part of the standing alone sequence
///   the number of letters in that sequence
/// This basically means a letter sequence surrounded by white space with some potentially intervening chars
/// The intervening chars can be typeform/cap indicators, along with various forms of punctuation
/// The ith char should be an "L"
/// This assumes that there is whitespace before and after the character string
fn stands_alone(chars: &[char], i: usize) -> (bool, &[char], usize) {
    // scan backward and check the conditions for "standing-alone"
    // we scan forward and check the conditions for "standing-alone"
    assert_eq!(chars[i], 'L', "'stands_alone' starts with non 'L'");
    // debug!("stands_alone: i={}, chars: {:?}", i, chars);
    // Exclude accent prefix A... immediately before this L (part of the same letter)
    let mut letter_start = i;
    let mut left_end = i;
    if left_end > 0 {
        let mut j = left_end;
        while j > 0 && is_braille_char(chars[j - 1]) {
            j -= 1;
        }
        if j > 0 && chars[j - 1] == 'A' {
            letter_start = j - 1;
            left_end = j - 1;
        }
    }
    if !left_side_stands_alone(&chars[0..left_end]) {
        return (false, &chars[letter_start..i+2], 0);
    }

    let (mut is_alone, n_letters, n_right_matched) = right_side_stands_alone(&chars[i+2..]);
    // debug!("left is alone, right is alone: {}, : n_letters={}, n_right_matched={}", is_alone, n_letters, n_right_matched);

    if is_alone && n_letters == 1 {
        let ch = chars[i+1];
        if ch=='⠁' || ch=='⠊' || ch=='⠕' {      // a, i, o
            is_alone = false;
        }
    }
    // Include any A... prefix so callers emit the full accented letter
    return (is_alone, &chars[letter_start..i+2+n_right_matched], n_letters);

    /// chars before 'L'
    fn left_side_stands_alone(chars: &[char]) -> bool {
        // scan backwards to skip letters and intervening chars
        // once we hit an intervening char, only intervening chars are allowed if standing alone
        let mut intervening_chars_mode = false; // true when we are on the final stretch
        let mut i = chars.len();
        while i > 0 {
            i -= 1;
            let ch = chars[i];
            let prev_ch = if i > 0 {chars[i-1]} else {' '};  // ' ' is a char not in input
            // debug!("  left alone: prev/ch {}/{}", prev_ch, ch);
            if (!intervening_chars_mode && prev_ch == 'L') ||
               (prev_ch == 'o' || prev_ch == 'b') {
                intervening_chars_mode = true;
                i -= 1;       // ignore 'Lx' and also ignore 'ox'
            } else if is_left_intervening_char(ch) {
                intervening_chars_mode = true;
            } else {
                return "W𝐖-—―".contains(ch);
            }
        }

        return true;
    }

    // chars after character we are testing
    fn right_side_stands_alone(chars: &[char]) -> (bool, usize, usize) {
        // see RUEB 2.6.3
        fn is_right_intervening_char(ch: char) -> bool {
            matches!(ch, 'B' | 'I' | '𝔹' | 'S' | 'T' | 'D' | 'C' | '𝐶' | 's' | 'w' | 'e')
        }
        // scan forward to skip letters and intervening chars
        // once we hit an intervening char, only intervening chars are allowed if standing alone ('c' and 'b' are part of them)
        let mut intervening_chars_mode = false; // true when we are on the final stretch
        let mut i = 0;
        let mut n_letters = 1;      // we have skipped the first letter
        while i < chars.len() {
            let ch = chars[i];
            // debug!("  right alone: ch/next {}/{}", ch, if i+1<chars.len() {chars[i+1]} else {' '});
            if !intervening_chars_mode && ch == 'L' {
                n_letters += 1;
                i += 1;       // ignore 'Lx' and also ignore 'ox'
            } else if !intervening_chars_mode && ch == 'A' {
                // Accented letter A...Lx counts as one letter
                match index_after_accent_to_l(chars, i + 1) {
                    Some(i_l) => {
                        n_letters += 1;
                        i = i_l + 1; // now at letter braille; loop end advances past it
                    }
                    None => {
                        return (false, n_letters, i);
                    }
                }
            } else if ch == 'c' || ch == 'b' {
                i += 1;       // ignore 'Lx' and also ignore 'ox'
            } else if is_right_intervening_char(ch) {  
                intervening_chars_mode = true;
            } else {
                return if "W𝐖-—―".contains(ch) {(true, n_letters, i)} else {(false, n_letters, i)};
            }
            i += 1;
        }

        return (true, n_letters, chars.len());
    }
}


/// Return a modified result if chars can be contracted.
/// Otherwise, the original string is returned
fn handle_contractions(full_chars: &[char], start: usize, end: usize, mut result: String) -> String {
    struct Replacement {
        pattern: String,
        replacement: &'static str,
        skip_if_word_in: Option<&'static phf::Set<&'static str>>,
        word_start_only: bool,
    }

    const ASCII_TO_UNICODE: &[char] = &[
        '⠀', '⠮', '⠐', '⠼', '⠫', '⠩', '⠯', '⠄', '⠷', '⠾', '⠡', '⠬', '⠠', '⠤', '⠨', '⠌',
        '⠴', '⠂', '⠆', '⠒', '⠲', '⠢', '⠖', '⠶', '⠦', '⠔', '⠱', '⠰', '⠣', '⠿', '⠜', '⠹',
        '⠈', '⠁', '⠃', '⠉', '⠙', '⠑', '⠋', '⠛', '⠓', '⠊', '⠚', '⠅', '⠇', '⠍', '⠝', '⠕',
        '⠏', '⠟', '⠗', '⠎', '⠞', '⠥', '⠧', '⠺', '⠭', '⠽', '⠵', '⠪', '⠳', '⠻', '⠘', '⠸',
    ];

    fn to_unicode_braille(ascii: &str) -> String {
        let mut unicode = String::with_capacity(4*ascii.len());   // 'L' + 3 bytes for braille char
        for ch in ascii.as_bytes() {
            unicode.push('L');
            unicode.push(ASCII_TO_UNICODE[(ch.to_ascii_uppercase() - 32) as usize])
        }
        return unicode;
    }

    // Whole words where the lower groupsign 'cc' must not apply (10.11.1 compound/function names).
    static CC_EXCEPTION_WORDS: phf::Set<&str> = phf_set! {
        "L⠁L⠗L⠉L⠉L⠕L⠎",       // arccos
        "L⠁L⠗L⠉L⠉L⠕L⠎L⠊L⠝L⠑", // arccosine
        "L⠁L⠗L⠉L⠉L⠕L⠎L⠓",     // arccosh
        "L⠁L⠗L⠉L⠉L⠕L⠎L⠑L⠉",   // arccosec
        "L⠁L⠗L⠉L⠉L⠕L⠎L⠑L⠉L⠁L⠝L⠞", // arccosecant
        "L⠁L⠗L⠉L⠉L⠕L⠞",       // arccot
        "L⠁L⠗L⠉L⠉L⠕L⠞L⠁L⠝L⠛L⠑L⠝L⠞", // arccotangent
        "L⠁L⠗L⠉L⠉L⠎L⠉",       // arccsc
    };

    // Words where the 'ea' lower groupsign must not apply (re- prefix morpheme boundary).
    static EA_EXCEPTION_WORDS: phf::Set<&str> = phf_set! {
        "L⠗L⠑L⠁L⠉L⠞L⠁L⠝L⠉L⠑", // reactance
        "L⠗L⠑L⠁L⠉L⠞L⠊L⠕L⠝",     // reaction
        "L⠗L⠑L⠁L⠉L⠞L⠊L⠧L⠊L⠞L⠽", // reactivity
    };

    // Words where "be" is not the first syllable (RUEB 10.6.1 "But:" cases and similar).
    static BE_EXCEPTION_WORDS: phf::Set<&str> = phf_set! {
        "L⠃L⠑L⠁L⠗L⠊L⠝L⠛",     // bearing
        "L⠃L⠑L⠝L⠉L⠓L⠍L⠁L⠗L⠅", // benchmark
    };

    // It would be much better from an extensibility point of view to read the table in from a file
    static CONTRACTIONS: LazyLock<Vec<Replacement>> = LazyLock::new(|| { vec![
            // 10.9: initial-letter (dot-5) wordsigns -- whole word only
            Replacement{ pattern: format!("^{}$", to_unicode_braille("time")), replacement: "⠐⠞", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: format!("^{}$", to_unicode_braille("work")), replacement: "⠐⠺", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: format!("^{}$", to_unicode_braille("leverage")), replacement: "⠇⠐⠑⠁⠛⠑", skip_if_word_in: None, word_start_only: false },

            // 10.7.1: dot-5 initial-letter contractions (as wordsigns / groupsigns)
            Replacement{ pattern: format!("^{}", to_unicode_braille("through")), replacement: "⠐⠹", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: format!("(?P<s>L.){}(?P<e>L.)", to_unicode_braille("part")), replacement: "${s}⠐⠏${e}", skip_if_word_in: None, word_start_only: false },

            // 10.3: Strong contractions
            Replacement{ pattern: to_unicode_braille("and"), replacement: "L⠯", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("for"), replacement: "L⠿", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("of"), replacement: "L⠷", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("the"), replacement: "L⠮", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("with"), replacement: "L⠾", skip_if_word_in: None, word_start_only: false },

            // 10.6.1-10.6.4 / 10.10.4: lower groupsigns be/con/dis -- only at word start,
            // and preferred over other groupsigns (e.g. "distance" is ⠲⠞⠨⠑, not di+st+ance).
            Replacement{ pattern: format!("^{}", to_unicode_braille("be")), replacement: "⠆", skip_if_word_in: Some(&BE_EXCEPTION_WORDS), word_start_only: true },
            Replacement{ pattern: format!("^{}", to_unicode_braille("con")), replacement: "⠒", skip_if_word_in: None, word_start_only: true },
            Replacement{ pattern: format!("^{}", to_unicode_braille("dis")), replacement: "⠲", skip_if_word_in: None, word_start_only: true },
            
            // 10.8: final-letter group signs (these need to precede 'en' and any other shorter contraction)
            Replacement{ pattern: "(?P<s>L.)L⠍L⠑L⠝L⠞".to_string(), replacement: "${s}L⠰L⠞", skip_if_word_in: None, word_start_only: false }, // ment
            Replacement{ pattern: "(?P<s>L.)L⠞L⠊L⠕L⠝".to_string(), replacement: "${s}L⠰L⠝", skip_if_word_in: None, word_start_only: false }, // tion
            Replacement{ pattern: "(?P<s>L.)L⠊L⠞L⠽".to_string(), replacement: "${s}L⠰L⠽", skip_if_word_in: None, word_start_only: false }, // ity
            Replacement{ pattern: "(?P<s>L.)L⠁L⠝L⠉L⠑".to_string(), replacement: "${s}L⠨L⠑", skip_if_word_in: None, word_start_only: false }, // ance
            Replacement{ pattern: "(?P<s>L.)L⠎L⠊L⠕L⠝".to_string(), replacement: "${s}L⠨L⠝", skip_if_word_in: None, word_start_only: false }, // sion
            Replacement{ pattern: "(?P<s>L.)L⠑L⠝L⠉L⠑".to_string(), replacement: "${s}L⠰L⠑", skip_if_word_in: None, word_start_only: false }, // ence
            Replacement{ pattern: "(?P<s>L.)L⠝L⠑L⠎L⠎".to_string(), replacement: "${s}L⠰L⠎", skip_if_word_in: None, word_start_only: false }, // ness
            Replacement{ pattern: "(?P<s>L.)L⠕L⠥L⠝L⠙".to_string(), replacement: "${s}L⠨L⠙", skip_if_word_in: None, word_start_only: false }, // ound
            Replacement{ pattern: "(?P<s>L.)L⠕L⠥L⠝L⠞".to_string(), replacement: "${s}L⠨L⠞", skip_if_word_in: None, word_start_only: false }, // ount
            Replacement{ pattern: "(?P<s>L.)L⠇L⠑L⠎L⠎".to_string(), replacement: "${s}L⠨L⠎", skip_if_word_in: None, word_start_only: false }, // less
            Replacement{ pattern: "(?P<s>L.)L⠕L⠝L⠛".to_string(), replacement: "${s}L⠰L⠛", skip_if_word_in: None, word_start_only: false }, // ong
            Replacement{ pattern: "(?P<s>L.)L⠋L⠥L⠇".to_string(), replacement: "${s}L⠰L⠇", skip_if_word_in: None, word_start_only: false }, // ful

            // 10.4: Strong group signs
            Replacement{ pattern: to_unicode_braille("ch"), replacement: "L⠡", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("gh"), replacement: "L⠣", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("sh"), replacement: "L⠩", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("th"), replacement: "L⠹", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("wh"), replacement: "L⠱", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("ed"), replacement: "L⠫", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("er"), replacement: "L⠻", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("ou"), replacement: "L⠳", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("ow"), replacement: "L⠪", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("st"), replacement: "L⠌", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: "(?P<s>L.)L⠊L⠝L⠛".to_string(), replacement: "${s}L⠬", skip_if_word_in: None, word_start_only: false },  // 'ing', not at start
            Replacement{ pattern: to_unicode_braille("ar"), replacement: "L⠜", skip_if_word_in: None, word_start_only: false },

            // 10.6.5: Lower group signs preceded and followed by letters
            // FIX: don't match if after/before a cap letter -- can't use negative pattern (?!...) in regex package
            Replacement{ pattern: "(?P<s>L.)L⠑L⠁(?P<e>L.)".to_string(), replacement: "${s}L⠂${e}", skip_if_word_in: Some(&EA_EXCEPTION_WORDS), word_start_only: false },  // ea
            Replacement{ pattern: "(?P<s>L.)L⠃L⠃(?P<e>L.)".to_string(), replacement: "${s}L⠆${e}", skip_if_word_in: None, word_start_only: false },  // bb
            Replacement{ pattern: "(?P<s>L.)L⠉L⠉(?P<e>L.)".to_string(), replacement: "${s}L⠒${e}", skip_if_word_in: Some(&CC_EXCEPTION_WORDS), word_start_only: false },  // cc
            Replacement{ pattern: "(?P<s>L.)L⠋L⠋(?P<e>L.)".to_string(), replacement: "${s}L⠖${e}", skip_if_word_in: None, word_start_only: false },  // ff
            Replacement{ pattern: "(?P<s>L.)L⠛L⠛(?P<e>L.)".to_string(), replacement: "${s}L⠶${e}", skip_if_word_in: None, word_start_only: false },  // gg

            // 10.6.8: lower group signs; also 10.6.2 word signs when at word start ("sin", etc.)
            // FIX: these need restrictions about only applying when upper dots are present
            Replacement{ pattern: to_unicode_braille("en"), replacement: "⠢", skip_if_word_in: None, word_start_only: false },
            Replacement{ pattern: to_unicode_braille("in"), replacement: "⠔", skip_if_word_in: None, word_start_only: false },
           
        ]
    });

    static CONTRACTION_PATTERNS: LazyLock<RegexSet> = LazyLock::new(|| init_patterns(&CONTRACTIONS));
    static CONTRACTION_REGEX: LazyLock<Vec<Regex>> = LazyLock::new(|| init_regex(&CONTRACTIONS));

    let word_start_ok = allows_lower_word_sign_at(full_chars, start);
    let chars = &full_chars[start..end];
    // A non-English letter glued to an English word (GTM 1.7.9 Δtime, or a
    // double-struck/script/fraktur letter) is not part of the English
    // letters-sequence; contract the following word on its own (time → ⠐⠞).
    let skip = leading_non_word_letter_len(chars);
    if skip > 0 && skip < chars.len() {
        return handle_contractions(full_chars, start + skip, end, result);
    }
    let mut chars_as_str = chars.iter().collect::<String>();
    let original_chars_as_str = chars_as_str.clone();
    // debug!("  handle_contractions: examine '{}'", &chars_as_str);
    let matches = CONTRACTION_PATTERNS.matches(&chars_as_str);
    for i in matches.iter() {
        let element = &CONTRACTIONS[i];
        if let Some(exceptions) = element.skip_if_word_in
            && exceptions.contains(&original_chars_as_str) {
                continue;
            }
        if element.word_start_only && !word_start_ok {
            continue;
        }
        // debug!("  replacing '{}' with '{}' in '{}'", element.pattern, element.replacement, &chars_as_str);
        result.truncate(result.len() - chars_as_str.len());
        chars_as_str = CONTRACTION_REGEX[i].replace_all(&chars_as_str, element.replacement).to_string();
        result.push_str(&chars_as_str);
        // debug!("  result after replace '{}'", result);
    }
    return result;



    fn leading_non_word_letter_len(chars: &[char]) -> usize {
        if chars.is_empty() {
            return 0;
        }
        if is_math_alphabet_typeform(chars[0]) {
            let end = math_typeform_item_end(chars, 0);
            return if end < chars.len() { end } else { 0 };
        }
        let mut i = 0;
        if i < chars.len() && matches!(chars[i], 'C' | '𝐶') {
            i += 1;
        }
        if i < chars.len() && matches!(chars[i], 'G' | 'V') {
            i += 1;
            if i < chars.len() && chars[i] == 'L' {
                i += 1;
                if i < chars.len() {
                    i += 1; // braille cell
                }
                return i;
            }
        }
        0
    }

    fn init_patterns(contractions: &[Replacement]) -> RegexSet {
        let patterns = contractions.iter().map(|c| c.pattern.as_str());
        return RegexSet::new(patterns).unwrap();
    }

    fn init_regex(contractions: &[Replacement]) -> Vec<Regex> {
        return contractions.iter()
            .map(|c| Regex::new(&c.pattern).unwrap())
            .collect();
    }
}




static VIETNAM_INDICATOR_REPLACEMENTS: phf::Map<&str, &str> = phf_map! {
    "S" => "XXX",    // sans-serif -- from prefs
    "B" => "⠘",     // bold
    "𝔹" => "XXX",     // blackboard -- from prefs
    "T" => "⠈",     // script
    // "E" => "⠰",     // English
    "1" => "⠠",     // Grade 1 symbol
    "L" => "",     // Letter left in to assist in locating letters
    "D" => "XXX",     // German (Deutsche) -- from prefs
    "G" => "⠰",     // Greek
    "V" => "XXX",    // Greek Variants
    // "H" => "⠠⠠",    // Hebrew
    // "U" => "⠈⠈",    // Russian
    "C" => "⠨",      // capital
    "𝑐" => "",       // second or latter braille cell of a capital letter
    "𝐶" => "⠨",      // capital that never should get word indicator (from chemical element)
    "N" => "⠼",     // number indicator
    "t" => "⠱",     // shape terminator
    "W" => "⠀",     // whitespace"
    "𝐖"=> "⠀",     // whitespace
    "s" => "⠆",     // typeface single char indicator
    "w" => "",     // typeface word indicator
    "e" => "",     // typeface & capital terminator 
    "o" => "",       // flag that what follows is an open indicator (used for standing alone rule)
    "c" => "",     // flag that what follows is an close indicator (used for standing alone rule)
    "b" => "",       // flag that what follows is an open or close indicator (used for standing alone rule)
    "," => "⠂",     // comma
    "." => "⠲",     // period
    "-" => "-",     // hyphen
    "—" => "⠠⠤",   // normal dash (2014) -- assume all normal dashes are unified here [RUEB appendix 3]
    "―" => "⠐⠠⠤",  // long dash (2015) -- assume all long dashes are unified here [RUEB appendix 3]
    "#" => "",      // signals end of script
    "!" => "",      // Hack used to prevent some regular expression matches
};

fn vietnam_cleanup(pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String {
    // Deal with Vietnamese "rhymes" -- moving accents around
    // See "Vietnamese Uncontracted Braille Update in MathCAT" or maybe https://icanreadvietnamese.com/blog/14-rule-of-tone-mark-placement
    // Note: I don't know how to write (for example) I_E_RULE so that it excludes "qu" and "gi", so I use two rules
    // The first rule rewrites the patterns with "qu" and "gi" to add "!" to prevent a match of the second rule -- "!" is dropped later
    static QU_GI_RULE_EXCEPTION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(L⠟L⠥|L⠛L⠊)").unwrap());
    static IUOY_E_RULE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"L(⠊|⠥|⠕|⠽)(L[⠔⠰⠢⠤⠠])L(⠑|⠣)").unwrap()); // ie, ue, oe, and ye rule
    static UO_A_RULE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"L(⠥|⠕)(L[⠔⠰⠢⠤⠠])L(⠁|⠡|⠜)").unwrap()); // ua, oa rule
    static UU_O_RULE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"L(⠥|⠳)(L[⠔⠰⠢⠤⠠])L(⠪|⠹)").unwrap()); // uo, ưo rule
    static UYE_RULE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"L⠥L([⠔⠰⠢⠤⠠])L⠽L⠣").unwrap()); // uo, ưo rule
    static UY_RULE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"L⠥L([⠔⠰⠢⠤⠠])L⠽").unwrap()); // uo, ưo rule
    static REPLACE_INDICATORS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([1𝟙SB𝔹TIREDGVHP𝐶𝑐CLMNW𝐖swe,.-—―#ocb!])").unwrap());
    // debug!("vietnam_cleanup: start={}", raw_braille);
    let result = typeface_to_word_mode(&raw_braille);
    let result = capitals_to_word_mode(&result);

    let result = result.replace("tW", "W");
    let result = result.replace("CG", "⠸");    // capital Greek letters are problematic in Vietnam braille
    let result = result.replace("CC", "⠸");    // capital word more is the same as capital Greek letters
    // debug!("   after typeface/caps={}", &result);

    // deal with "rhymes"
    let result = QU_GI_RULE_EXCEPTION.replace_all(&result, "${1}!");
    // debug!("          after except={}", &result);
    let result = IUOY_E_RULE.replace_all(&result, "${2}L${1}L${3}");
    // debug!("          after IUOY_E={}", &result);
    let result = UO_A_RULE.replace_all(&result, "${2}L${1}L${3}");
    // debug!("          after   UO_A={}", &result);
    let result = UU_O_RULE.replace_all(&result, "${2}L${1}L${3}");
    // debug!("          after   UO_O={}", &result);
    let result = UYE_RULE.replace_all(&result, "${1}L⠥L⠽L⠣");  // longer match first
    // debug!("          after    UYE={}", &result);
    let result = UY_RULE.replace_all(&result, "${1}L⠥L⠽");
    // debug!("          after     UY={}", &result);

    // these typeforms need to get pulled from user-prefs as they are transcriber-defined
    let typeforms = UserTypeforms::from_prefs(&pref_manager, "Vietnam");

    // This reuses the code just for getting rid of unnecessary "L"s and "N"s
    let result = remove_unneeded_mode_changes(&result, UEB_Mode::Grade1, UEB_Duration::Passage);


    let result = apply_indicator_replacements(&result, &REPLACE_INDICATORS, &VIETNAM_INDICATOR_REPLACEMENTS,
        "VIETNAM_INDICATOR_REPLACEMENTS", &typeforms);

    // Remove unicode blanks at start and end -- do this after the substitutions because ',' introduces spaces
    // let result = result.trim_start_matches('⠀').trim_end_matches('⠀');
    let result = COLLAPSE_SPACES.replace_all(&result, "⠀");
   
    return result.to_string();
}


static CMU_INDICATOR_REPLACEMENTS: phf::Map<&str, &str> = phf_map! {
    // "S" => "XXX",    // sans-serif -- from prefs
    "B" => "⠔",     // bold
    "𝔹" => "⠬",     // blackboard -- from prefs
    // "T" => "⠈",     // script
    "I" => "⠔",     // italic -- used for digits (not math letters); same cell as bold in CMU
    // "R" => "",      // roman
    // "E" => "⠰",     // English
    "1" => "⠐",     // Grade 1 symbol -- used here for a-j after number
    "L" => "",     // Letter left in to assist in locating letters
    "D" => "⠠",     // German (Gothic)
    "G" => "⠈",     // Greek
    "V" => "⠈⠬",    // Greek Variants
    // "H" => "⠠⠠",    // Hebrew
    // "U" => "⠈⠈",    // Russian
    "C" => "⠨",      // capital
    "𝐶" => "⠨",      // capital that never should get word indicator (from chemical element)
    "N" => "⠼",     // number indicator
    "𝑁" => "",      // continue number
    // "t" => "⠱",     // shape terminator
    "W" => "⠀",     // whitespace"
    "𝐖"=> "⠀",     // whitespace
    // "𝘄" => "⠀",    // add whitespace if char to the left has dots 1, 2, or 3 -- special rule handled separately, so commented out
    "s" => "",     // typeface single char indicator
    // "w" => "⠂",     // typeface word indicator
    // "e" => "⠄",     // typeface & capital terminator 
    // "o" => "",       // flag that what follows is an open indicator (used for standing alone rule)
    // "c" => "",       // flag that what follows is an close indicator (used for standing alone rule)
    // "b" => "",       // flag that what follows is an open or close indicator (used for standing alone rule)
    "," => "⠂",     // comma
    "." => "⠄",     // period
    "-" => "⠤",     // hyphen
    "—" => "⠤⠤",   // normal dash (2014) -- assume all normal dashes are unified here [RUEB appendix 3]
    // "―" => "⠐⠤⠤",  // long dash (2015) -- assume all long dashes are unified here [RUEB appendix 3]
    "#" => "⠼",      // signals to end/restart of numeric mode (mixed fractions)
};


fn cmu_cleanup(_pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String {
    static ADD_WHITE_SPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"𝘄(.)|𝘄$").unwrap());

    // debug!("cmu_cleanup: start={}", raw_braille);
    // let result = typeface_to_word_mode(&raw_braille);

    // let result = result.replace("tW", "W");
    let result = raw_braille.replace("CG", "⠘")
                                .replace("𝔹C", "⠩")
                                .replace("DC", "⠰");
    // let result = result.replace("CC", "⠸");

    // these typeforms need to get pulled from user-prefs as they are transcriber-defined
    // let double_struck = pref_manager.pref_to_string("CMU_DoubleStruck");
    // let sans_serif = pref_manager.pref_to_string("CMU_SansSerif");
    // let fraktur = pref_manager.pref_to_string("CMU_Fraktur");

    // debug!("Before remove mode changes: '{}'", &result);
    // This reuses the code just for getting rid of unnecessary "L"s and "N"s
    let result = remove_unneeded_mode_changes(&result, UEB_Mode::Grade1, UEB_Duration::Passage);
    let result = result.replace("𝑁N", "");
    // debug!(" After remove mode changes: '{}'", &result);

    let result = REPLACE_INDICATORS.replace_all(&result, |cap: &Captures| {
        match CMU_INDICATOR_REPLACEMENTS.get(&cap[0]) {
            None => {error!("REPLACE_INDICATORS and CMU_INDICATOR_REPLACEMENTS are not in sync"); ""},
            Some(&ch) => ch,
        }
    });
    let result = ADD_WHITE_SPACE.replace_all(&result, |cap: &Captures| {
        if cap.get(1).is_none() {
            return "⠀".to_string();
        } else {
            // debug!("ADD_WHITE_SPACE match='{}', has left dots = {}", &cap[1], has_left_dots(cap[1].chars().next().unwrap()));
            let mut next_chars = cap[1].chars();
            let next_char = next_chars.next().unwrap();
            assert!(next_chars.next().is_none());
            return (if has_left_dots(next_char) {"⠀"} else {""}).to_string() + &cap[1];
        }
    });
    
    // Remove unicode blanks at start and end -- do this after the substitutions because ',' introduces spaces
    let result = COLLAPSE_SPACES.replace_all(&result, "⠀");
    let result = result.trim_start_matches('⠀');            // don't trip end (e.g., see once::vector_11_2_5)
    return result.to_string();

    fn has_left_dots(ch: char) -> bool {
        // Unicode braille is set up so dot 1 is 2^0, dot 2 is 2^1, etc
        return ( (ch as u32 - 0x2800) >> 4 ) > 0;
    }
}



static SWEDISH_INDICATOR_REPLACEMENTS: phf::Map<&str, &str> = phf_map! {
    // FIX: this needs cleaning up -- not all of these are used
    "S" => "XXX",    // sans-serif -- from prefs
    "B" => "⠨",     // bold
    "𝔹" => "XXX",     // blackboard -- from prefs
    "T" => "⠈",     // script
    "I" => "⠨",     // italic -- used for digits (not math letters)
    "R" => "",      // roman
    "1" => "⠱",     // Grade 1 symbol (used for number followed by a letter)
    "L" => "",     // Letter left in to assist in locating letters
    "D" => "XXX",     // German (Deutsche) -- from prefs
    "G" => "⠰",     // Greek
    "V" => "XXX",    // Greek Variants
    // "H" => "⠠⠠",    // Hebrew
    // "U" => "⠈⠈",    // Russian
    "C" => "⠠",      // capital
    "𝑐" => "",       // second or latter braille cell of a capital letter
    "𝐶" => "⠠",      // capital that never should get word indicator (from chemical element)
    "N" => "⠼",     // number indicator
    "t" => "⠱",     // shape terminator
    "W" => "⠀",     // whitespace"
    "𝐖"=> "⠀",     // whitespace
    "w" => "⠀",     // whitespace after function name
    "s" => "",     // typeface single char indicator
    "e" => "",     // typeface & capital terminator 
    "E" => "⠱",     // empty base -- see index of radical
    "o" => "",       // flag that what follows is an open indicator (used for standing alone rule)
    "c" => "",     // flag that what follows is an close indicator (used for standing alone rule)
    "b" => "",       // flag that what follows is an open or close indicator (used for standing alone rule)
    "," => "⠂",     // comma
    "." => "⠲",     // period
    "-" => "-",     // hyphen
    "—" => "⠠⠤",   // normal dash (2014) -- assume all normal dashes are unified here [RUEB appendix 3]
    "―" => "⠐⠠⠤",  // long dash (2015) -- assume all long dashes are unified here [RUEB appendix 3]
    "#" => "",      // signals end of script

};


static FINNISH_INDICATOR_REPLACEMENTS: phf::Map<&str, &str> = phf_map! {
    // FIX: this needs cleaning up -- not all of these are used
    "S" => "XXX",    // sans-serif -- from prefs
    "B" => "⠨",     // bold
    "𝔹" => "XXX",     // blackboard -- from prefs
    "T" => "⠈",     // script
    "I" => "⠨",     // italic -- used for digits (not math letters)
    "R" => "",      // roman
    "E" => "⠰",     // English
    "1" => "⠀",     // Grade 1 symbol (used for number followed by a letter)
    "L" => "",     // Letter left in to assist in locating letters
    "D" => "XXX",     // German (Deutsche) -- from prefs
    "G" => "⠨",     // Greek
    "V" => "XXX",    // Greek Variants
    // "H" => "⠠⠠",    // Hebrew
    // "U" => "⠈⠈",    // Russian
    "C" => "⠠",      // capital
    "𝑐" => "",       // second or latter braille cell of a capital letter
    "𝐶" => "⠠",      // capital that never should get whitespace in front (from chemical element)
    "N" => "⠼",     // number indicator
    "n" => "⠼",     // number indicator for drop numbers (special case with close parens)
    "t" => "⠱",     // shape terminator
    "W" => "⠀",     // whitespace"
    "𝐖"=> "⠀",     // whitespace
    "s" => "⠆",     // typeface single char indicator
    "w" => "",     // typeface word indicator
    "e" => "",     // typeface & capital terminator 
    "," => "⠂",     // comma
    "." => "⠲",     // period
    "-" => "-",     // hyphen
    "—" => "⠠⠤",   // normal dash (2014) -- assume all normal dashes are unified here [RUEB appendix 3]
    "―" => "⠐⠠⠤",  // long dash (2015) -- assume all long dashes are unified here [RUEB appendix 3]
    "(" => "⠦",     // Not really needed, but done for consistency with ")"
    ")" => "⠴",     // Needed for rules with drop numbers to avoid mistaking for dropped 0
    "↑" => "⠬",     // superscript
    "↓" => "⠡",     // subscript
    "#" => "",      // signals end of script
    "Z" => "⠐",     // signals end of index of root, integrand/lim from function ("zone change")

};

fn finnish_cleanup(pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String {
    static REPLACE_INDICATORS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([SB𝔹TIREDGVHUP𝐏C𝐶LlMmb↑↓Nn𝑁WwZ,()])").unwrap());
    // Numbers need to end with a space, but sometimes there is one there for other reasons
    static DROP_NUMBER_SEPARATOR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(n.)\)").unwrap());
    static NUMBER_MATCH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"((N.)+[^WN𝐶#↑↓Z])").unwrap());

    // debug!("finnish_cleanup: start={}", raw_braille);
    let result = DROP_NUMBER_SEPARATOR.replace_all(&raw_braille, |cap: &Captures| {
        // match includes the char after the number -- insert the whitespace before it
        // debug!("DROP_NUMBER_SEPARATOR match='{}'", &cap[1]);
        return cap[1].to_string() + "𝐶)";       // hack to use "𝐶" instead of dot 6 directly, but works for NUMBER_MATCH
    });
    let result = result.replace('n', "N");  // avoids having to modify remove_unneeded_mode_changes()
    let result = NUMBER_MATCH.replace_all(&result, |cap: &Captures| {
        // match includes the char after the number -- insert the whitespace before it
        // debug!("NUMBER_MATCH match='{}'", &cap[1]);
        let mut chars = cap[0].chars();
        let last_char = chars.next_back().unwrap(); // unwrap safe since several chars were matched
        return chars.as_str().to_string() + "W" + &last_char.to_string();
    });

    // FIX: need to implement this -- this is just a copy of the Vietnam code
    let result = result.replace("CG", "⠘")
                                    .replace("𝔹C", "⠩")
                                    .replace("DC", "⠰");

    // debug!("   after typeface/caps={}", &result);

    // these typeforms need to get pulled from user-prefs as they are transcriber-defined (Finnish reuses the Vietnam prefs)
    let typeforms = UserTypeforms::from_prefs(&pref_manager, "Vietnam");

    // This reuses the code just for getting rid of unnecessary "L"s and "N"s
    let result = remove_unneeded_mode_changes(&result, UEB_Mode::Grade1, UEB_Duration::Passage);
    // debug!("   remove_unneeded_mode_changes={}", &result);


    // Note: the "out of sync" message intentionally references SWEDISH (pre-existing quirk)
    let result = apply_indicator_replacements(&result, &REPLACE_INDICATORS, &FINNISH_INDICATOR_REPLACEMENTS,
        "SWEDISH_INDICATOR_REPLACEMENTS", &typeforms);

    // Remove unicode blanks at start and end -- do this after the substitutions because ',' introduces spaces
    // let result = result.trim_start_matches('⠀').trim_end_matches('⠀');
    let result = COLLAPSE_SPACES.replace_all(&result, "⠀");
   
    return result.to_string();
}


fn swedish_cleanup(pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String {
    // FIX: need to implement this -- this is just a copy of the Vietnam code
    // Empty bases are ok if they follow whitespace
    static EMPTY_BASE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(^|[W𝐖w])E").unwrap());
    // debug!("swedish_cleanup: start={}", raw_braille);
    let result = typeface_to_word_mode(&raw_braille);
    let result = capitals_to_word_mode(&result);

    let result = result.replace("CG", "⠘")
                                    .replace("𝔹C", "⠩")
                                    .replace("DC", "⠰");

    // debug!("   after typeface/caps={}", &result);

    // these typeforms need to get pulled from user-prefs as they are transcriber-defined (Swedish reuses the Vietnam prefs)
    let typeforms = UserTypeforms::from_prefs(&pref_manager, "Vietnam");

    // This reuses the code just for getting rid of unnecessary "L"s and "N"s
    let result = remove_unneeded_mode_changes(&result, UEB_Mode::Grade1, UEB_Duration::Passage);
    // debug!("   after removing mode changes={}", &result);


    let result = EMPTY_BASE.replace_all(&result, "$1");
    let result = apply_indicator_replacements(&result, &REPLACE_INDICATORS, &SWEDISH_INDICATOR_REPLACEMENTS,
        "SWEDISH_INDICATOR_REPLACEMENTS", &typeforms);

    // Remove unicode blanks at start and end -- do this after the substitutions because ',' introduces spaces
    // let result = result.trim_start_matches('⠀').trim_end_matches('⠀');
    let result = COLLAPSE_SPACES.replace_all(&result, "⠀");
   
    return result.to_string();
}

static FRENCH_INDICATOR_REPLACEMENTS: phf::Map<&str, &str> = phf_map! {
    // Keys match indicator chars emitted by French unicode.yaml, unicode-full.yaml, and French_Rules.yaml
    "S" => "XXX",    // sans-serif -- from prefs
    "B" => "⠸",     // bold
    "𝔹" => "⠨",     // blackboard
    "T" => "⠈",     // script
    "I" => "⠸",     // italic
    "1" => "",     // Grade 1 symbol (used for number followed by a letter, but irrelevant for French, but generated by UEB cleanup)
    "L" => "",     // Letter left in to assist in locating letters
    "D" => "XXX",     // German (Deutsche) -- from prefs
    "G" => "⠘",     // Greek
    "H" => "⠘⠘",     // Hebrew
    "C" => "⠨",      // capital
    "𝐶" => "⠨",      // capital that never should get whitespace in front (from chemical element)
    "N" => "",     // number indicator
    "W" => "⠀",     // whitespace"
    "s" => "⠆",     // typeface single char indicator
    "o" => "⠰",     // Opening group indicator
    "c" => "⠆",     // Closing group indicator
    "b" => "⠰",     // Optional opening group indicator
    "e" => "⠆",     // Optional closing group indicator
    ";" => "⠆",     // Need to be able to detect ';' and potentially add dot-6 (see 7° Ponctuations)
    "P" => "⠠⠄",     // Math modifier and code change indicator
};

/// Remove optional group indicators `b`/`e` when not needed; promote to mandatory `o`/`c` when they are.
/// Optional pairs nest like parentheses. Resolve outer pairs before inner ones.
/// A pair is kept when its content ends with a letter/number (`L`/`N` + braille) before `e`, and a
/// letter/number follows the next `b` (or directly) after `e`.
/// Because it fits in the algorithm, dealing with ';' (need to add dot-6 if inside of a block) is also handled
fn remove_optional_group_indicators(braille: &str) -> String {
    let chars: Vec<char> = braille.chars().collect();
    let mut stack: Vec<usize> = Vec::new();
    let mut pairs: Vec<(usize, usize)> = Vec::new();

    for (i, &ch) in chars.iter().enumerate() {
        if ch == 'b' {
            stack.push(i);
        } else if ch == 'e'
            && let Some(b_idx) = stack.pop()
        {
            pairs.push((b_idx, i));
        }
    }

    let mut convert_to_oc = vec![false; chars.len()];
    let mut remove = vec![false; chars.len()];

    for (b_idx, e_idx) in pairs.into_iter().rev() {
        let needed = letter_or_number_before_e(b_idx, e_idx, &chars, &remove, &convert_to_oc)
            && letter_or_number_after_b(e_idx, &chars, &remove, &convert_to_oc);
        if needed {
            convert_to_oc[b_idx] = true;
            convert_to_oc[e_idx] = true;
        } else {
            remove[b_idx] = true;
            remove[e_idx] = true;
        }
    }

    // 7° Ponctuations: when the formula contains any block symbols (start/end of block, dots 5-6 /
    // dots 2-3), a ';' is preceded by dot-6 so it isn't confused with the end-of-block indicator
    // (which is also dots 2-3). This applies to every ';' in the formula, not just those nested
    // inside a block. Blocks in the output are the surviving 'o'/'c' (literal or converted b/e).
    let has_block = convert_to_oc.iter().any(|&converted| converted)
        || chars.iter().zip(remove.iter()).any(|(&ch, &removed)| !removed && (ch == 'o' || ch == 'c'));

    let mut result = String::with_capacity(braille.len());
    for (i, &ch) in chars.iter().enumerate() {
        if remove[i] {
            continue;
        }
        // A ';' gets a leading dot-6; the dots-23 itself comes from the ';' -> "⠆" mapping in
        // FRENCH_INDICATOR_REPLACEMENTS.
        if ch == ';' && has_block {
            result.push('⠠');
        }
        if convert_to_oc[i] {
            result.push(if ch == 'b' { 'o' } else { 'c' });
        } else {
            result.push(ch);
        }
    }
    return result;
}

fn is_letter_or_number_start(chars: &[char], i: usize, remove: &[bool]) -> bool {
    if i + 1 >= chars.len() || remove[i] {
        return false;
    }
    if chars[i] != 'L' && chars[i] != 'N' {
        return false;
    }
    return !remove[i + 1] && !is_optional_group_or_mode_marker(chars[i + 1]);
}

fn is_optional_group_or_mode_marker(ch: char) -> bool {
    return matches!(ch, 'b' | 'e' | 'o' | 'c' | 'L' | 'N' | 'W' | 'P' | 'C' | '1' | '#');
}

fn letter_or_number_before_e(
    b_idx: usize,
    e_idx: usize,
    chars: &[char],
    remove: &[bool],
    convert_to_oc: &[bool],
) -> bool {
    let mut i = e_idx;
    while i > b_idx {
        i -= 1;
        if remove[i] {
            continue;
        }
        if chars[i] == 'e' && !convert_to_oc[i] {
            continue;
        }
        if chars[i] == 'e' && convert_to_oc[i] {
            continue;
        }
        if chars[i] == 'L' || chars[i] == 'N' {
            return true;
        }
        if i > b_idx {
            let mut j = i;
            loop {
                if j <= b_idx {
                    return false;
                }
                j -= 1;
                if remove[j] {
                    continue;
                }
                if chars[j] == 'e' && !convert_to_oc[j] {
                    continue;
                }
                if chars[j] == 'e' && convert_to_oc[j] {
                    continue;
                }
                return chars[j] == 'L' || chars[j] == 'N';
            }
        }
        return false;
    }
    return false;
}

fn letter_or_number_after_b(
    e_idx: usize,
    chars: &[char],
    remove: &[bool],
    convert_to_oc: &[bool],
) -> bool {
    let mut i = e_idx + 1;
    while i < chars.len() {
        if remove[i] {
            i += 1;
            continue;
        }
        if chars[i] == 'e' && convert_to_oc[i] {
            i += 1;
            continue;
        }
        if chars[i] == 'b' {
            return is_letter_or_number_start(chars, i + 1, remove);
        }
        return is_letter_or_number_start(chars, i, remove);
    }
    return false;
}

fn french_cleanup(_pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String {
    // FIX: need to implement this -- this is just a copy of the Vietnam code
    static REPLACE_INDICATORS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([1SB𝔹TILDGHC𝐶NWscocbe;])").unwrap());
    debug!("french_cleanup: start={}", raw_braille);
    // let result = typeface_to_word_mode(&raw_braille);
    // let result = capitals_to_word_mode(&result);

    let result = remove_optional_group_indicators(&raw_braille);
    // check to see if math modifier and code change indicator are needed at start (p8)
    let result = if let Some(stripped) = result.strip_prefix('P') {
        let prefix = if result.contains('o') { "⠠⠄" } else { "⠠" };
        let mut new_string = String::with_capacity(stripped.len() + prefix.len());
        new_string.push_str(prefix);
        new_string.push_str(stripped);
        new_string
    } else {
        result
    };

    // This reuses the code just for getting rid of unnecessary "L"s and "N"s
    let result = remove_unneeded_mode_changes(&result, UEB_Mode::Grade1, UEB_Duration::Passage);
    debug!("   after removing mode changes={}", result);

    // remove any grouping pair at the start or end -- we ensure they are a pair but making sure there is no "o" inside the string
    let result = if result.starts_with('o') && result.ends_with('c') && !result[1..result.len() - 1].contains('o') {
        // Remove the first and last characters by slicing
        &result[1..result.len() - 1]
    } else {
        &result
    };

    let result: Cow<'_, str> = REPLACE_INDICATORS.replace_all(result, |cap: &Captures| {
        match FRENCH_INDICATOR_REPLACEMENTS.get(&cap[0]) {
                None => {error!("REPLACE_INDICATORS and FRENCH_INDICATOR_REPLACEMENTS are not in sync: missing '{}'", &cap[0]); ""},
                Some(&ch) => ch,
        }
    });

    // Remove unicode blanks at start and end -- do this after the substitutions because ',' introduces spaces
    // let result = result.trim_start_matches('⠀').trim_end_matches('⠀');
    let result = COLLAPSE_SPACES.replace_all(&result, "⠀");
   
    return result.to_string();
}

fn russian_cleanup(_pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String {
    static REPLACE_INDICATORS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([LNW#])").unwrap());
    static COLLAPSE_SPACES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"⠀+").unwrap());
    static PERIODIC_DECIMAL_NUM_INDICATOR: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(N[⠚⠁⠃⠉⠙⠑⠋⠛⠓⠊]+⠂[⠚⠁⠃⠉⠙⠑⠋⠛⠓⠊]*⠣)N([⠚⠁⠃⠉⠙⠑⠋⠛⠓⠊]+⠜)").unwrap()
    });

    let mut raw_braille_without_repeated_number_indicators = String::with_capacity(raw_braille.len());
    let mut previous_char_was_digit = false;
    for ch in raw_braille.chars() {
        if ch == 'N' && previous_char_was_digit {
            previous_char_was_digit = false;
            continue;
        }
        raw_braille_without_repeated_number_indicators.push(ch);
        previous_char_was_digit = matches!(ch, '⠚' | '⠁' | '⠃' | '⠉' | '⠙' | '⠑' | '⠋' | '⠛' | '⠓' | '⠊');
    }

    let raw_braille_without_periodic_number_indicator = PERIODIC_DECIMAL_NUM_INDICATOR
        .replace_all(&raw_braille_without_repeated_number_indicators, "$1$2");
    let result = add_russian_typeform_indicators(&raw_braille_without_periodic_number_indicator);
    let result = add_russian_alphabet_indicators(&result);
    let result = REPLACE_INDICATORS.replace_all(&result, |cap: &Captures| {
        match &cap[0] {
            "C" => "⠨",
            "L" => "",
            "N" => "⠼",
            "W" => "⠀",
            "#" => "",
            _ => "",
        }
    });
    let result = result.replace("⠠⠙⠊⠧", "⠫⠙⠊⠧");
    return COLLAPSE_SPACES.replace_all(&result, "⠀")
                          .trim_matches('⠀')
                          .to_string();

    fn add_russian_typeform_indicators(raw_braille: &str) -> String {
        let mut result = String::with_capacity(raw_braille.len());
        let mut active_typeforms: Vec<char> = Vec::with_capacity(2);
        let mut pending_typeforms: Vec<char> = Vec::with_capacity(2);
        let mut previous_was_capital_marker = false;

        for ch in raw_braille.chars() {
            if is_russian_typeform_marker(ch) {
                if !pending_typeforms.contains(&ch) {
                    pending_typeforms.push(ch);
                }
                previous_was_capital_marker = false;
                continue;
            }
            if matches!(ch, 's' | 'w' | 'e') && (!pending_typeforms.is_empty() || !active_typeforms.is_empty()) {
                continue;
            }

            if is_russian_token_start(ch) {
                if !previous_was_capital_marker {
                    close_inactive_typeforms(&mut result, &mut active_typeforms, &pending_typeforms);
                }
                for &typeform in &pending_typeforms {
                    if !active_typeforms.contains(&typeform) {
                        result.push_str(russian_typeform_indicator(typeform));
                        active_typeforms.push(typeform);
                    }
                }
                pending_typeforms.clear();
            } else if !pending_typeforms.is_empty() {
                close_inactive_typeforms(&mut result, &mut active_typeforms, &pending_typeforms);
                for &typeform in &pending_typeforms {
                    if !active_typeforms.contains(&typeform) {
                        result.push_str(russian_typeform_indicator(typeform));
                        result.push_str(russian_typeform_indicator(typeform));
                    }
                }
                pending_typeforms.clear();
            } else if !active_typeforms.is_empty() && !is_russian_token_body(ch) {
                close_all_typeforms(&mut result, &mut active_typeforms);
            }

            result.push(ch);
            previous_was_capital_marker = ch == 'C';
        }

        close_all_typeforms(&mut result, &mut active_typeforms);
        return result;
    }

    fn is_russian_typeform_marker(ch: char) -> bool {
        matches!(ch, 'B' | 'I' | 'T' | 'D' | 'S' | '𝔹')
    }

    fn is_russian_token_start(ch: char) -> bool {
        matches!(ch, 'l' | 'u' | 'g' | 'v' | 'L' | 'N' | 'C')
    }

    fn is_russian_token_body(ch: char) -> bool {
        ('\u{2801}'..='\u{28ff}').contains(&ch)
    }

    fn close_inactive_typeforms(result: &mut String, active_typeforms: &mut Vec<char>, pending_typeforms: &[char]) {
        let mut i = active_typeforms.len();
        while i > 0 {
            i -= 1;
            if !pending_typeforms.contains(&active_typeforms[i]) {
                let typeform = active_typeforms.remove(i);
                result.push_str(russian_typeform_indicator(typeform));
            }
        }
    }

    fn close_all_typeforms(result: &mut String, active_typeforms: &mut Vec<char>) {
        while let Some(typeform) = active_typeforms.pop() {
            result.push_str(russian_typeform_indicator(typeform));
        }
    }

    fn russian_typeform_indicator(typeform: char) -> &'static str {
        match typeform {
            'B' => "⠻",                         // ГОСТ Р 58511: жирный шрифт, точки 12456.
            'I' | 'T' | 'D' | 'S' | '𝔹' => "⠸", // ГОСТ Р 58511: курсив/шрифтовое выделение, точки 456.
            _ => "",
        }
    }

    fn add_russian_alphabet_indicators(raw_braille: &str) -> String {
        let mut result = String::with_capacity(raw_braille.len());
        let mut alphabet_mode = None;
        let mut capital_marker_pending = false;
        for ch in raw_braille.chars() {
            match ch {
                'C' => {
                    if alphabet_mode != Some('u') {
                        result.push('⠨');
                        alphabet_mode = Some('u');
                    }
                    capital_marker_pending = true;
                },
                'l' if capital_marker_pending => {
                    capital_marker_pending = false;
                },
                'l' | 'u' | 'g' | 'v' => {
                    capital_marker_pending = false;
                    if alphabet_mode != Some(ch) {
                        result.push_str(match ch {
                            'l' => "⠠",  // Latin lowercase: dots 6
                            'u' => "⠨",  // Latin uppercase: dots 4-6
                            'g' => "⠰",  // Greek lowercase: dots 5-6
                            'v' => "⠸",  // Greek uppercase: dots 4-5-6
                            _ => unreachable!(),
                        });
                        alphabet_mode = Some(ch);
                    }
                },
                'N' | '#' => {
                    capital_marker_pending = false;
                    alphabet_mode = None;
                    result.push(ch);
                },
                _ => {
                    capital_marker_pending = false;
                    result.push(ch);
                },
            }
        }
        return result;
    }
}

#[allow(non_snake_case)]
fn LaTeX_cleanup(_pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String {
    static REMOVE_SPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" ([\^_,;)\]}])").unwrap()); // '^', '_', ',', ';', ')', ']', '}'
    static COLLAPSE_SPACES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" +").unwrap());
    // debug!("LaTeX_cleanup: start={}", raw_braille);
    let result = raw_braille.replace('𝐖', " ");
    // let result = COLLAPSE_SPACES.replace_all(&raw_braille, "⠀");
    let result = COLLAPSE_SPACES.replace_all(&result, " ");
    // debug!("After collapse: {}", &result);
    let result = REMOVE_SPACE.replace_all(&result, "$1");
    // debug!("After remove: {}", &result);
    // let result = result.trim_matches('⠀');
    let result = result.trim_matches(' ');
   
    return result.to_string();
}

#[allow(non_snake_case)]
fn ASCIIMath_cleanup(_pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String {
    static REMOVE_SPACE_BEFORE_OP: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"([\w\d]) +([^\w\d"]|[\^_,;)\]}])"#).unwrap());
    static REMOVE_SPACE_AFTER_OP: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"([^\^_,;)\]}\w\d"]) +([\w\d])"#).unwrap());
    static COLLAPSE_SPACES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" +").unwrap());
    // debug!("ASCIIMath_cleanup: start={}", raw_braille);
    let result  = raw_braille.replace("|𝐖__|", "|𝐰__|");    // protect the whitespace to prevent misinterpretation as lfloor
    let result = result.replace('𝐖', " ");
    let result = COLLAPSE_SPACES.replace_all(&result, " ");
    // debug!("After collapse: {}", &result);
    let result = REMOVE_SPACE_BEFORE_OP.replace_all(&result, "$1$2");
    let result = REMOVE_SPACE_AFTER_OP.replace_all(&result, "$1$2");
    let result = result.replace('𝐰', " ");     // spaces around relational operators
    let result = COLLAPSE_SPACES.replace_all(&result, " ");
    // debug!("After remove: {}", &result);
    // let result = result.trim_matches('⠀');
    let result = result.trim_matches(' ');
   
    return result.to_string();
}


/************** Braille xpath functionality ***************/
use crate::canonicalize::{as_element, as_text, name};
use crate::xpath_functions::{IsBracketed, is_leaf, validate_one_node};
use std::result::Result as StdResult;
use sxd_document_no_unsafe::dom::ParentOfChild;
use sxd_xpath_no_unsafe::function::Error as XPathError;
use sxd_xpath_no_unsafe::function::{Args, Function};
use sxd_xpath_no_unsafe::{context, nodeset::*, Value};

pub struct NemethNestingChars;
const NEMETH_FRAC_LEVEL: &str = "data-nemeth-frac-level";    // name of attr where value is cached
const FIRST_CHILD_ONLY: &[&str] = &["mroot", "msub", "msup", "msubsup", "munder", "mover", "munderover", "mmultiscripts"];
impl NemethNestingChars {
    // returns a 'repeat_char' corresponding to the Nemeth rules for nesting
    // note: this value is likely one char too long because the starting fraction is counted
    fn nemeth_frac_value(node: Element, repeat_char: &str) -> String {
        let children = node.children();
        let name = name(node);
        if is_leaf(node) {
            return "".to_string();
        } else if name == "mfrac" {
            // have we already computed the value?
            if let Some(value) = node.attribute_value(NEMETH_FRAC_LEVEL) {
                return value.to_string();
            }

            let num_value = NemethNestingChars::nemeth_frac_value(as_element(children[0]), repeat_char);
            let denom_value = NemethNestingChars::nemeth_frac_value(as_element(children[1]), repeat_char);
            let mut max_value = if num_value.len() > denom_value.len() {num_value} else {denom_value};
            max_value += repeat_char;
            node.set_attribute_value(NEMETH_FRAC_LEVEL, &max_value);
            return max_value;
        } else if FIRST_CHILD_ONLY.contains(&as_str!(name)) {
            // only look at the base -- ignore scripts/index
            return NemethNestingChars::nemeth_frac_value(as_element(children[0]), repeat_char);
        } else {
            let mut result = "".to_string();
            for child in children {
                let value = NemethNestingChars::nemeth_frac_value(as_element(child), repeat_char);
                if value.len() > result.len() {
                    result = value;
                }
            }
            return result;
        }
    }

    fn nemeth_root_value(node: Element, repeat_char: &str) -> StdResult<String, XPathError> {
        // returns the correct number of repeat_chars to use
        // note: because the highest count is toward the leaves and
        //    because this is a loop and not recursive, caching doesn't work without a lot of overhead
        let parent = node.parent().unwrap();
        if let ParentOfChild::Element(e) =  parent {
            let mut parent = e;
            let mut result = "".to_string();
            loop {
                let name = name(parent);
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
                    return Err( sxd_xpath_no_unsafe::function::Error::Other { what: "Internal error in nemeth_root_value: didn't find 'math' tag".to_string() } );
                }
            }
        }
        return Err( XPathError::Other { what: "Internal error in nemeth_root_value: didn't find 'math' tag".to_string() } );
    }
}

impl Function for NemethNestingChars {
/**
 * Returns a string with the correct number of nesting chars (could be an empty string)
 * @param(node) -- current node
 * @param(char) -- char (string) that should be repeated
 * Note: as a side effect, an attribute with the value so repeated calls to this or a child will be fast
 */
 fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> StdResult<Value<'d>, XPathError>
    {
        let mut args = Args(args);
        args.exactly(2)?;
        let repeat_char = args.pop_string()?;
        let node = crate::xpath_functions::validate_one_node(args.pop_nodeset()?, "NestingChars")?;
        if let Node::Element(el) = node {
            let name = name(el);
            // it is likely a bug to call this one a non mfrac
            if name == "mfrac" {
                // because it is called on itself, the fraction is counted one too many times -- chop one off
                // this is slightly messy because we are chopping off a char, not a byte
                const BRAILLE_BYTE_LEN: usize = "⠹".len();      // all Unicode braille symbols have the same number of bytes
                return Ok( Value::String( NemethNestingChars::nemeth_frac_value(el, &repeat_char)[BRAILLE_BYTE_LEN..].to_string() ) );
            } else if name == "msqrt" || name == "mroot" {
                return Ok( Value::String( NemethNestingChars::nemeth_root_value(el, &repeat_char)? ) );
            } else {
                return Err(XPathError::Other { what: format!("NestingChars chars should be used only on 'mfrac'. '{}' was passed in", name) });
            }
        } else {
            // not an element, so nothing to do
            return Ok( Value::String("".to_string()) );
        }
    }
}

pub struct BrailleChars;
impl BrailleChars {
    // returns a string for the chars in the *leaf* node.
    // this string follows the Nemeth rules typefaces and deals with mathvariant
    //  which has partially turned chars to the alphanumeric block
    fn get_braille_chars(node: Element, code: &str, text_range: Option<Range<usize>>) -> StdResult<String, XPathError> {
        let result = match get_braille_code(code) {
            Some(braille_code) => braille_code.get_braille_chars(node, text_range),
            None => return Err(sxd_xpath_no_unsafe::function::Error::Other { what: format!("get_braille_chars: unknown braille code '{code}'") }),
        };
        return match result {
            Ok(string) => Ok(make_quoted_string(string)),
            Err(err) => return Err(sxd_xpath_no_unsafe::function::Error::Other { what: err.to_string() }),
        }
    }

    /// Italic typeform is for digits (and numeric punctuation like '.'), not math letters.
    fn uses_italic_typeform(node_name: &str, text: &str) -> bool {
        if node_name == "mn" {
            return true;
        }
        !text.is_empty() && text.chars().all(|ch| ch.is_ascii_digit() || matches!(ch, '.' | ',' | '−' | '-' | '+'))
    }

    fn get_braille_nemeth_chars(node: Element, text_range: Option<Range<usize>>) -> Result<String> {
        // To greatly simplify typeface/language generation, the chars have unique ASCII chars for them:
        // Typeface: S: sans-serif, B: bold, 𝔹: blackboard, T: script, I: italic, R: Roman
        // Language: E: English, D: German, G: Greek, V: Greek variants, H: Hebrew, U: Russian
        // Indicators: C: capital, L: letter, N: number, P: punctuation, M: multipurpose
        // Italic typeform is used for digits; math letters do not get italic.
        static PICK_APART_CHAR: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?P<face>[SB𝔹TIR]*)(?P<lang>[EDGVHU]?)(?P<cap>C?)(?P<letter>L?)(?P<num>[N]?)(?P<char>.)").unwrap()
        });
        let raw_math_variant = node.attribute_value("mathvariant");
        let math_variant = raw_math_variant.as_deref();
        let text = BrailleChars::substring(as_str!(as_text(node)), &text_range);
        let node_name = as_str!(name(node));
        let italic_ok = BrailleChars::uses_italic_typeform(node_name, &text);
        let  attr_typeface = match math_variant {
            None => "R",
            Some(variant) => match variant {
                "bold" => "B",
                "italic" => if italic_ok {"I"} else {"R"},
                "bold-italic" => if italic_ok {"BI"} else {"B"},
                "double-struck" => "𝔹",
                "script" => "T",
                "fraktur" => "D",
                "sans-serif" => "S",
                _ => "R",       // normal and unknown
            },
        };
        let braille_chars = braille_replace_chars(&text, node)?;
        // debug!("Nemeth chars: text='{}', braille_chars='{}'", &text, &braille_chars);
        
        // we want to pull the prefix (typeface, language) out to the front until a change happens
        // the same is true for number indicator
        // also true (sort of) for capitalization -- if all caps, use double cap in front (assume abbr or Roman Numeral)
        
        // we only care about this for numbers and identifiers/text, so we filter for only those
        let is_in_enclosed_list = node_name != "mo" && BrailleChars::is_in_enclosed_list(node);
        let is_mn_in_enclosed_list = is_in_enclosed_list && node_name == "mn";
        let mut typeface = "R".to_string();     // assumption is "R" and if attr or letter is different, something happens
        let mut is_all_caps = true;
        let mut is_all_caps_valid = false;      // all_caps only valid if we did a replacement
        let result = PICK_APART_CHAR.replace_all(&braille_chars, |caps: &Captures| {
            // debug!("  face: {:?}, lang: {:?}, num {:?}, letter: {:?}, cap: {:?}, char: {:?}",
            //        &caps["face"], &caps["lang"], &caps["num"], &caps["letter"], &caps["cap"], &caps["char"]);
            let mut nemeth_chars = "".to_string();
            let face_stripped;
            let char_face = if caps["face"].is_empty() {
                attr_typeface
            } else if italic_ok {
                &caps["face"]
            } else {
                face_stripped = caps["face"].replace('I', "");
                &face_stripped
            };
            let typeface_changed =  typeface != char_face;
            if typeface_changed {
                typeface = char_face.to_string();   // needs to outlast this instance of the loop
                nemeth_chars += &typeface;
                nemeth_chars +=  &caps["lang"];
            } else {
                nemeth_chars +=  &caps["lang"];
            }
            // debug!("  typeface changed: {}, is_in_list: {}; num: {}", typeface_changed, is_in_enclosed_list, !caps["num"].is_empty());
            if !caps["num"].is_empty() && (typeface_changed || !is_mn_in_enclosed_list) {
                nemeth_chars += "N";
            }
            is_all_caps_valid = true;
            is_all_caps &= !&caps["cap"].is_empty();
            nemeth_chars += &caps["cap"];       // will be stripped later if all caps
            if is_in_enclosed_list {
                nemeth_chars += &caps["letter"].replace('L', "l");
            } else {
                nemeth_chars += &caps["letter"];
            }
            nemeth_chars += &caps["char"];
            return nemeth_chars;
        });
        // debug!("  result: {}", &result);
        let mut text_chars = text.chars();     // see if more than one char
        if is_all_caps_valid && is_all_caps && text_chars.next().is_some() &&  text_chars.next().is_some() {
            return Ok( "CC".to_string() + &result.replace('C', ""));
        } else {
            return Ok( result.to_string() );
        }
    }

    fn get_braille_ueb_chars(node: Element, text_range: Option<Range<usize>>) -> Result<String> {
        // Because in UEB typeforms and caps may extend for multiple tokens,
        //   this routine merely deals with the mathvariant attr.
        // Canonicalize has already transformed all chars it can to math alphanumerics, but not all have bold/italic 
        // The typeform/caps transforms to (potentially) word mode are handled later.
        static HAS_TYPEFACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(".*?(double-struck|script|fraktur|sans-serif).*").unwrap());
        static PICK_APART_CHAR: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?P<bold>B??)(?P<italic>I??)(?P<face>[S𝔹TD]??)s??(?P<cap>C??)(?P<greek>G??)(?P<char>[NL].)").unwrap()
        });
    
        let raw_math_variant = node.attribute_value("mathvariant");
        let math_variant = raw_math_variant.as_deref();
        let text = BrailleChars::substring(as_str!(as_text(node)), &text_range);
        let mut braille_chars = braille_replace_chars(&text, node)?;

        // debug!("get_braille_ueb_chars: before/after unicode.yaml: '{}'/'{}'", text, braille_chars);
        if math_variant.is_none() {         // nothing we need to do
            return Ok(braille_chars);
        }
        // mathvariant could be "sans-serif-bold-italic" -- get the parts
        // Italic typeform for digits (GTM 2.7); math letters do not get italic (GTM 1.5).
        let math_variant = math_variant.unwrap();
        let italic_ok = BrailleChars::uses_italic_typeform(as_str!(name(node)), &text);
        let italic = italic_ok && math_variant.contains("italic");
        if italic && !braille_chars.contains('I') {
            braille_chars = "I".to_string() + &braille_chars;
        }
        if !italic_ok {
            braille_chars = braille_chars.replace('I', "");
        }
        let bold = math_variant.contains("bold");
        if bold & !braille_chars.contains('B') {
            braille_chars = "B".to_string() + &braille_chars;
        }
        let typeface = match HAS_TYPEFACE.find(math_variant) {
            None => "",
            Some(m) => match m.as_str() {
                "double-struck" => "𝔹",
                "script" => "T",
                "fraktur" => "D",
                "sans-serif" => "S",
                //  don't consider monospace as a typeform
                _ => "",
            },
        };
        let result = PICK_APART_CHAR.replace_all(&braille_chars, |caps: &Captures| {
            // debug!("captures: {:?}", caps);
            // debug!("  bold: {:?}, italic: {:?}, face: {:?}, cap: {:?}, char: {:?}",
            //        &caps["bold"], &caps["italic"], &caps["face"], &caps["cap"], &caps["char"]);
            if bold || !caps["bold"].is_empty() {"B"} else {""}.to_string()
                + if italic || !caps["italic"].is_empty() {"I"} else {""}
                + if !&caps["face"].is_empty() {&caps["face"]} else {typeface}
                + &caps["cap"]
                + &caps["greek"]
                + &caps["char"]
        });
        // debug!("get_braille_ueb_chars: '{}'", &result);
        return Ok(result.to_string())
    }

    fn get_braille_russian_chars(node: Element, text_range: Option<Range<usize>>) -> Result<String> {
        // Russian italic typeform for letters is left unchanged pending external advice.
        let text = BrailleChars::substring(as_str!(as_text(node)), &text_range);
        let braille_chars = braille_replace_chars(&text, node)?;
        let Some(raw_math_variant) = node.attribute_value("mathvariant") else {
            return Ok(braille_chars);
        };
        let math_variant = as_str!(raw_math_variant);

        let mut prefix = String::new();
        let mut suffix = String::new();
        if math_variant.contains("bold") && !braille_chars.contains('B') {
            prefix.push('⠻');
            suffix.insert(0, '⠻');
        }
        if math_variant.contains("italic") && !braille_chars.contains('I') {
            prefix.push('⠸');
            suffix.insert(0, '⠸');
        }
        let has_typeface_marker = braille_chars.contains('T')
            || braille_chars.contains('D')
            || braille_chars.contains('S')
            || braille_chars.contains('𝔹');
        if !has_typeface_marker {
            let typeface = match math_variant {
                "double-struck" | "script" | "fraktur" | "sans-serif" => Some("⠸"),
                _ => None,
            };
            if let Some(indicator) = typeface {
                prefix.push_str(indicator);
                suffix.insert_str(0, indicator);
            }
        }

        return Ok(prefix + &braille_chars + &suffix);
    }

    fn get_braille_cmu_chars(node: Element, text_range: Option<Range<usize>>) -> Result<String> {
        // In CMU, we need to replace spaces used for number blocks with "."
        // For other numbers, we need to add "." to create digit blocks

        static HAS_TYPEFACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(".*?(double-struck|script|fraktur|sans-serif).*").unwrap());
        static PICK_APART_CHAR: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?P<bold>B??)(?P<italic>I??)(?P<face>[S𝔹TD]??)s??(?P<cap>C??)(?P<greek>G??)(?P<char>[NL].)").unwrap()
        });
    
        let raw_math_variant = node.attribute_value("mathvariant");
        let math_variant = raw_math_variant.as_deref();
        let text = BrailleChars::substring(as_str!(as_text(node)), &text_range);
        let text = add_separator(text);

        let mut braille_chars = braille_replace_chars(&text, node)?;

        // debug!("get_braille_ueb_chars: before/after unicode.yaml: '{}'/'{}'", text, braille_chars);
        if math_variant.is_none() {         // nothing we need to do
            return Ok(braille_chars);
        }
        // mathvariant could be "sans-serif-bold-italic" -- get the parts
        // CMU: italic typeform for digits only (not math letters)
        let math_variant = math_variant.unwrap();
        let italic_ok = BrailleChars::uses_italic_typeform(as_str!(name(node)), &text);
        let italic = italic_ok && math_variant.contains("italic");
        if italic && !braille_chars.contains('I') {
            braille_chars = "I".to_string() + &braille_chars;
        }
        if !italic_ok {
            braille_chars = braille_chars.replace('I', "");
        }
        let bold = math_variant.contains("bold");
        if bold && !braille_chars.contains('B') {
            braille_chars = "B".to_string() + &braille_chars;
        }
        let typeface = match HAS_TYPEFACE.find(math_variant) {
            None => "",
            Some(m) => match m.as_str() {
                "double-struck" => "𝔹",
                "script" => "T",
                "fraktur" => "D",
                "sans-serif" => "S",
                //  don't consider monospace as a typeform
                _ => "",
            },
        };
        let result = PICK_APART_CHAR.replace_all(&braille_chars, |caps: &Captures| {
            // debug!("captures: {:?}", caps);
            // debug!("  bold: {:?}, italic: {:?}, face: {:?}, cap: {:?}, char: {:?}",
            //        &caps["bold"], &caps["italic"], &caps["face"], &caps["cap"], &caps["char"]);
            if bold || !caps["bold"].is_empty() {"B"} else {""}.to_string()
                + if italic || !caps["italic"].is_empty() {"I"} else {""}
                + if !&caps["face"].is_empty() {&caps["face"]} else {typeface}
                + &caps["cap"]
                + &caps["greek"]
                + &caps["char"]
        });
        return Ok(result.to_string());

        fn add_separator(text: String) -> String {
            use crate::definitions::BRAILLE_DEFINITIONS;
            if let Some(text_without_arc) = text.strip_prefix("arc") {
                // "." after arc (7.5.3)
                let is_function_name = BRAILLE_DEFINITIONS.with(|definitions| {
                    let definitions = definitions.borrow();
                    let set = definitions.get_hashset("CMUFunctionNames").unwrap();
                    return set.contains(&text);
                });
                if is_function_name {
                    return "arc.".to_string() + text_without_arc;
                }
            } 
            return text;
        }
    }

    fn get_braille_vietnam_chars(node: Element, text_range: Option<Range<usize>>) -> Result<String> {
        // this is basically the same as for ueb except:
        // 1. we deal with switching '.' and ',' if in English style for numbers
        // 2. if it is identified as a Roman Numeral, we make all but the first char lower case because they shouldn't get a cap indicator
        // 3. double letter chemical elements should NOT be part of a cap word sequence
        if name(node) == "mn" {
            // text of element is modified by these if needed
            lower_case_roman_numerals(node);
            switch_if_english_style_number(node);
        }
        let result = BrailleChars::get_braille_ueb_chars(node, text_range)?;
        return Ok(result);

        fn lower_case_roman_numerals(mn_node: Element) {
            if mn_node.attribute("data-roman-numeral").is_some() {
                // if a roman numeral, all ASCII so we can optimize
                let text = as_text(mn_node);
                let mut new_text = String::from(&text[..1]);
                new_text.push_str(text[1..].to_ascii_lowercase().as_str());    // works for single char too
                mn_node.set_text(&new_text);
            }
        }
        fn switch_if_english_style_number(mn_node: Element) {
            let text = as_text(mn_node);
            let dot = text.find('.');
            let comma = text.find(',');
            match (dot, comma) {
                (None, None) => (),
                (Some(dot), Some(comma)) => {
                    if comma < dot {
                        // switch dot/comma -- using "\x01" as a temp when switching the two chars
                        let switched = text.replace('.', "\x01").replace(',', ".").replace('\x01', ",");
                        mn_node.set_text(&switched);
                    }
                },
                (Some(dot), None) => {
                    // If it starts with a '.', a leading 0, or if there is only one '.' and not three chars after it
                    if dot==0 ||
                       (dot==1 && text.starts_with('0')) ||
                       (text[dot+1..].find('.').is_none() && text[dot+1..].len()!=3) {
                        mn_node.set_text(&text.replace('.', ","));
                    }
                },
                (None, Some(comma)) => {
                    // if there is more than one ",", than it can't be a decimal separator
                    if text[comma+1..].find(',').is_some() {
                        mn_node.set_text(&text.replace(',', "."));
                    }
                },
            }
        }

    }


    fn is_in_enclosed_list(node: Element) -> bool {
        // Nemeth Rule 10 defines an enclosed list:
        // 1: begins and ends with fence
        // 2: FIX: not implemented -- must contain no word, abbreviation, ordinal or plural ending
        // 3: function names or signs of shape and the signs which follow them are a single item (not a word)
        // 4: an item of the list may be an ellipsis or any sign used for omission
        // 5: no relational operator may appear within the list
        // 6: the list must have at least 2 items.
        //       Items are separated by commas, can not have other punctuation (except ellipsis and dash)
        let mut parent = get_parent(node); // safe since 'math' is always at root
        while name(parent) == "mrow" {
            if IsBracketed::is_bracketed(parent, "", "", true, false) {
                for child in parent.children() {
                    if !child_meets_conditions(as_element(child)) {
                        return false;
                    }
                }
                return true;
            }
            parent = get_parent(parent);
        }
        return false;

        fn child_meets_conditions(node: Element) -> bool {
            let name = as_str!(name(node));
            return match name {
                "mi" | "mn" => true,
                "mo"  => !crate::canonicalize::is_relational_op(node),
                "mtext" => {
                    let raw_text = as_text(node);
                    let text = raw_text.trim();
                    return text=="?" || text=="-?-" || text.is_empty();
                },
                "mrow" => {
                    if IsBracketed::is_bracketed(node, "", "", false, false) {
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
                "menclose" => {
                    if let Some(notation) = node.attribute_value("notation").as_deref() {
                        if notation != "bottom" || notation != "box" {
                            return false;
                        }
                        let child = as_element(node.children()[0]);     // menclose has exactly one child
                        return is_leaf(child) && as_text(child) == "?";
                    }
                    return false;
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

    /// Extract the `char`s from `str` within `range` (these are chars, not byte offsets)
    fn substring(str: &str, text_range: &Option<Range<usize>>) -> String {
        return match text_range {
            None => str.to_string(),
            Some(range) => str.chars().skip(range.start).take(range.end - range.start).collect(),
        }
    }
}

impl Function for BrailleChars {
    /**
     * Returns a string with the correct number of nesting chars (could be an empty string)
     * @param(node) -- current node or string
     * @param(char) -- char (string) that should be repeated
     * Note: as a side effect, an attribute with the value so repeated calls to this or a child will be fast
     */
    fn evaluate<'d>(&self,
                        context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> StdResult<Value<'d>, XPathError>
    {
        use crate::canonicalize::create_mathml_element;
        let mut args = Args(args);
        if let Err(e) = args.exactly(2).or_else(|_| args.exactly(4)) {
            return Err( XPathError::Other { what: format!("BrailleChars requires 2 or 4 args: {e}") });
        };

        let range = if args.len() == 4 {
            let end = args.pop_number()? as usize - 1;      // non-inclusive at end, 0-based
            let start = args.pop_number()? as usize - 1;    // inclusive at start, a 0-based
            Some(start..end)
        } else {
            None
        };
        let braille_code = args.pop_string()?;
        let v: Value<'_> = args.0.pop().ok_or(XPathError::ArgumentMissing)?;
        let node = match v {
            Value::Nodeset(nodes) => {
                validate_one_node(nodes, "BrailleChars")?.element().unwrap()
            },
            Value::Number(n) => {
                let new_node = create_mathml_element(&context.node.document(), "mn");
                new_node.set_text(&n.to_string());
                new_node
            },
            Value::String(s) => {
                let new_node = create_mathml_element(&context.node.document(), "mi");   // FIX: try to guess mi vs mo???
                new_node.set_text(&s);
                new_node
            },
            _ => {
                return Ok( Value::String("".to_string()) ) // not an element, so nothing to do
            },
        };

        if !is_leaf(node) {
            return Err( XPathError::Other { what: format!("BrailleChars called on non-leaf element '{}'", mml_to_string(node)) } );
        }
        return Ok( Value::String( BrailleChars::get_braille_chars(node, &braille_code, range)? ) );
    }
}

pub struct NeedsToBeGrouped;
impl NeedsToBeGrouped {
    // ordinals often have an irregular start (e.g., "half") before becoming regular.
    // if the number is irregular, return the ordinal form, otherwise return 'None'.
    fn needs_grouping_for_cmu(element: Element, _is_base: bool) -> bool {
        let node_name = name(element);
        let children = element.children();
        if node_name == "mrow" {
            // check for bracketed exprs
            if IsBracketed::is_bracketed(element, "", "", false, true) {
                return false;
            }

            // check for prefix and postfix ops at start or end (=> len()==2, prefix is first op, postfix is last op)
            if children.len() == 2 &&
                (name(as_element(children[0])) == "mo" || name(as_element(children[1])) == "mo") {
                return false;
            }

            if children.len() != 3 {  // ==3, need to check if it a linear fraction
                return true;
            }
            let operator = as_element(children[1]);
            if name(operator) != "mo" || as_text(operator) != "/" {
                return true;
            }
        }

        if !(node_name == "mrow" || node_name == "mfrac") {
            return false;
        }
        // check for numeric fractions (regular fractions need brackets, not numeric fractions), either as an mfrac or with "/"
        // if the fraction starts with a "-", it is still a numeric fraction that doesn't need parens
        let mut numerator = as_element(children[0]);
        let denominator = as_element(children[children.len()-1]);
        let decimal_separator = crate::interface::get_preference("DecimalSeparators").unwrap()
                                                        .chars().next().unwrap_or('.');
        if is_integer(denominator, decimal_separator) {
            // check numerator being either an integer "- integer"
            if name(numerator) == "mrow" {
                let numerator_children = numerator.children();
                if !(numerator_children.len() == 2 &&
                        name(as_element(numerator_children[0])) == "mo" &&
                        as_text(as_element(numerator_children[0])) == "-") {
                    return true;
                }
                numerator = as_element(numerator_children[1]);
            }
            return !is_integer(numerator, decimal_separator);
        }
        return true;

        fn is_integer(mathml: Element, decimal_separator: char) -> bool {
            return name(mathml) == "mn" && !as_text(mathml).contains(decimal_separator)
        }
    }

    /// FIX: what needs to be implemented?
    fn needs_grouping_for_finnish(mathml: Element, is_base: bool) -> bool {
        use crate::xpath_functions::IsInDefinition;
        let mut node_name = as_str!(name(mathml));
        if mathml.attribute_value("data-roman-numeral").is_some() {
            node_name = "mi";           // roman numerals don't follow number rules
        }

        // FIX: the leaf rules are from UEB -- check the Swedish rules
        match node_name {
            "mn" => {   
                if !is_base {
                    return false;
                }                                                                                        // clause 1
                // two 'mn's can be adjacent, in which case we need to group the 'mn' to make it clear it is separate (see bug #204)
                let parent = get_parent(mathml);   // there is always a "math" node
                let grandparent = if name(parent) == "math" {parent} else {get_parent(parent)};
                if name(grandparent) != "mrow" {
                    return false;
                }
                let preceding = parent.preceding_siblings();
                if preceding.len()  < 2 {
                    return false;
                }
                // any 'mn' would be separated from this node by invisible times
                let previous_child = as_element(preceding[preceding.len()-1]);
                if name(previous_child) == "mo" && as_text(previous_child) == "\u{2062}" {
                    let previous_child = as_element(preceding[preceding.len()-2]);
                    return name(previous_child) == "mn"
                } else {
                    return false;
                }
            },
            "mi" | "mo" | "mtext" => {
                let text = as_str!(as_text(mathml));
                let parent = get_parent(mathml);   // there is always a "math" node
                let parent_name = name(parent);   // there is always a "math" node
                if is_base && (parent_name == "msub" || parent_name == "msup" || parent_name == "msubsup") && !text.contains([' ', '\u{00A0}']) {
                    return false;
                }
                let mut chars = text.chars();
                let first_char = chars.next().unwrap();             // canonicalization assures it isn't empty;
                let is_one_char = chars.next().is_none();
                // '¨', etc., brailles as two chars -- there probably is some exception list but I haven't found it -- these are the ones I know about
                return !((is_one_char && !['¨', '″', '‴', '⁗'].contains(&first_char)) ||                       // clause 8
                            // "lim", "cos", etc., appear not to get parens, but the rules don't mention it (tests show it)
                            IsInDefinition::is_defined_in(text, &SPEECH_DEFINITIONS, "FunctionNames").unwrap() ||
                            IsInDefinition::is_defined_in(text, &SPEECH_DEFINITIONS, "Arrows").unwrap() ||          // clause 4
                            IsInDefinition::is_defined_in(text, &SPEECH_DEFINITIONS, "GeometryShapes").unwrap());   // clause 5
            },
            "mrow" => {
                // check for bracketed exprs
                if IsBracketed::is_bracketed(mathml, "", "", false, true) {
                    return false;
                }

                let parent = get_parent(mathml); // safe since 'math' is always at root
                if name(parent) == "mfrac" {
                    let children = mathml.children();
                    if mathml.preceding_siblings().is_empty() {
                        // numerator: check for multiplication -- doesn't need grouping in numerator
                        if children.len() >= 3 {
                            let operator = as_element(children[1]);
                            if name(operator) == "mo" {
                                let ch = as_text(operator);
                                if ch == "\u{2062}" || ch == "⋅" || ch == "×"  {
                                    return false;
                                }
                            }
                        }
                        return true;
                    } else {
                        // denominator
                        return true;
                    }

                }
                // check for prefix at start
                // example 7.12 has "2-" in superscript and is grouped, so we don't consider postfix ops
                let children = mathml.children();
                if children.len() == 2 &&
                    (name(as_element(children[0])) == "mo") {
                    return false;
                }
                return true;
            },
            _ => return false,
        }
    }

    fn needs_grouping_for_french(mathml: Element, _is_base: bool) -> bool {
        // French rules are from p26: Blocks (also mentioned elsewhere in the spec)
        // if we have a "- xxx", we treat the MathML as if it were just "xxx".
        //   apparently, based on function_partial_derivative_14_1_04, the same goes for '∂' -- generalizing to all prefix ops...
        // Grouping is also needed if what follows (w/o regard to structure) is a digit or letter.
        // use crate::xpath_functions::IsInDefinition;
        let children = mathml.children();
        let mathml = if name(mathml) == "mrow" && children.len() == 2 &&
                                     name(as_element(children[0])) == "mo" {    // assuming it is a prefix op since it starts an mrow
            as_element(children[1])
        } else {
            mathml
        };

        debug!("needs_grouping_for_french: mathml (sans '-') = {}", mml_to_string(mathml));
        let node_name = as_str!(name(mathml));
        let children = mathml.children();
        match node_name {
            "mn" | "mi" | "mtext" => {
                return as_text(mathml).contains(char::is_whitespace);
            },
            "mo" => { return as_text(mathml).chars().nth(1).is_some(); },  // guessing that single chars operator don't need grouping based on set_not_belong_tor_6_4_04
            "ms" => {
                return false; // FIX: is this true? Do quotes group the contents?
            },
            "mrow" => {
                if IsBracketed::is_bracketed(mathml, "", "", false, false) {
                    return false;
                }
                // look for "a", "a b", etc -- last example on p26 shows "b d" doesn't need grouping
                // also apparently brackets are also acceptable (see last example on p43 where numerator isn't grouped)
                if children.iter().enumerate().all(|(i, &child)| {
                    let child = as_element(child);
                    if i % 2 == 1 {
                        name(child) == "mo" && as_text(child) == "\u{2062}"   // invisible times
                    } else {
                        name(child) == "mi" || (name(child) == "mrow" && IsBracketed::is_bracketed(child, "", "", false, false))
                    }
                }) {
                    return false;
                }
                return !(
                    children.len() == 3 &&
                    name(as_element(children[0])) == "mi" &&  // maybe don't need this check? E.g, sin^2 x.
                    name(as_element(children[1])) == "mo" && as_text(as_element(children[1])) == "\u{2061}"
                );
            },
            _ => return true,
        }
    }

    fn needs_grouping_for_swedish(mathml: Element, is_base: bool) -> bool {
        use crate::xpath_functions::IsInDefinition;
        let mut node_name = as_str!(name(mathml));
        if mathml.attribute_value("data-roman-numeral").is_some() {
            node_name = "mi";           // roman numerals don't follow number rules
        }

        match node_name {
            "mn" => return false,
            "mi" | "mo" | "mtext" => {
                let text = as_str!(as_text(mathml));
                let parent = get_parent(mathml);   // there is always a "math" node
                let parent_name = name(parent);   // there is always a "math" node
                if is_base && (parent_name == "msub" || parent_name == "msup" || parent_name == "msubsup") && !text.contains([' ', '\u{00A0}']) {
                    return false;
                }
                let mut chars = text.chars();
                let first_char = chars.next().unwrap();             // canonicalization assures it isn't empty;
                let is_one_char = chars.next().is_none();
                // '¨', etc., brailles as two chars -- there probably is some exception list but I haven't found it -- these are the ones I know about
                return !((is_one_char && !['¨', '″', '‴', '⁗'].contains(&first_char)) ||                       // clause 8
                            // "lim", "cos", etc., appear not to get parens, but the rules don't mention it (tests show it)
                            IsInDefinition::is_defined_in(text, &SPEECH_DEFINITIONS, "FunctionNames").unwrap() ||
                            IsInDefinition::is_defined_in(text, &SPEECH_DEFINITIONS, "Arrows").unwrap() ||          // clause 4
                            IsInDefinition::is_defined_in(text, &SPEECH_DEFINITIONS, "GeometryShapes").unwrap());   // clause 5
            },
            "mrow" => {
                // check for bracketed exprs
                if IsBracketed::is_bracketed(mathml, "", "", false, true) {
                    return false;
                }

                // check for prefix at start
                // example 7.12 has "2-" in superscript and is grouped, so we don't consider postfix ops
                let children = mathml.children();
                if children.len() == 2 &&
                    (name(as_element(children[0])) == "mo") {
                    return false;
                }
                return true;
            },
            "mfrac" => {
                // exclude simple fractions -- they are not bracketed with start/end marks
                let children = mathml.children();
                return !(NeedsToBeGrouped::needs_grouping_for_swedish(as_element(children[0]), true) ||
                         NeedsToBeGrouped::needs_grouping_for_swedish(as_element(children[0]), true));
            },
            // At least for msup (Ex 7.7, and 7.32 and maybe more), spec seems to feel grouping is not needed.
            // "msub" | "msup" | "msubsup" | "munder" | "mover" | "munderover" => return true,
            "mtable" => return true,    // Fix: should check for trivial cases that don't need grouping
            _ => return false,
        }
    }

    /// Returns true if the element needs grouping symbols
    /// Bases need extra attention because if they are a number and the item to the left is one, that needs distinguishing
    fn needs_grouping_for_ueb(mathml: Element, is_base: bool) -> bool {
        // From GTM 7.1
        // 1. An entire number, i.e. the initiating numeric symbol and all succeeding symbols within the numeric mode thus
        //     established (which would include any interior decimal points, commas, separator spaces, or simple numeric fraction lines).
        // 2. An entire general fraction, enclosed in fraction indicators.
        // 3. An entire radical expression, enclosed in radical indicators.
        // 4. An arrow.
        // 5. An arbitrary shape.
        // 6. Any expression enclosed in matching pairs of round parentheses, square brackets or curly braces.
        // 7. Any expression enclosed in the braille grouping indicators.   [Note: not possible here]
        // 8. If none of the foregoing apply, the item is simply the [this element's] individual symbol.

        use crate::xpath_functions::IsInDefinition;
        let mut node_name = as_str!(name(mathml));
        if mathml.attribute_value("data-roman-numeral").is_some() {
            node_name = "mi";           // roman numerals don't follow number rules
        }
        match node_name {
            "mn" => {   
                if !is_base {
                    return false;
                }                                                                                        // clause 1
                // two 'mn's can be adjacent, in which case we need to group the 'mn' to make it clear it is separate (see bug #204)
                let parent = get_parent(mathml);   // there is always a "math" node
                let grandparent = if name(parent) == "math" {parent} else {get_parent(parent)};
                if name(grandparent) != "mrow" {
                    return false;
                }
                let preceding = parent.preceding_siblings();
                if preceding.len()  < 2 {
                    return false;
                }
                // any 'mn' would be separated from this node by invisible times
                let previous_child = as_element(preceding[preceding.len()-1]);
                if name(previous_child) == "mo" && as_text(previous_child) == "\u{2062}" {
                    let previous_child = as_element(preceding[preceding.len()-2]);
                    return name(previous_child) == "mn"
                } else {
                    return false;
                }
            },
            "mi" | "mo" | "mtext" => {
                let text = as_str!(as_text(mathml));
                let parent = get_parent(mathml);   // there is always a "math" node
                let parent_name = name(parent);   // there is always a "math" node
                if is_base && (parent_name == "msub" || parent_name == "msup" || parent_name == "msubsup") && !text.contains([' ', '\u{00A0}']) {
                    return false;
                }
                let mut chars = text.chars();
                let first_char = chars.next().unwrap();             // canonicalization assures it isn't empty;
                let is_one_char = chars.next().is_none();
                // '¨', etc., brailles as two chars -- there probably is some exception list but I haven't found it -- these are the ones I know about
                return !((is_one_char && !['¨', '″', '‴', '⁗'].contains(&first_char)) ||                       // clause 8
                            // "lim", "cos", etc., appear not to get parens, but the rules don't mention it (tests show it)
                            IsInDefinition::is_defined_in(text, &SPEECH_DEFINITIONS, "FunctionNames").unwrap() ||
                            IsInDefinition::is_defined_in(text, &SPEECH_DEFINITIONS, "Arrows").unwrap() ||          // clause 4
                            IsInDefinition::is_defined_in(text, &SPEECH_DEFINITIONS, "GeometryShapes").unwrap());   // clause 5
            },
            "mfrac" => return false,                                                     // clause 2 (test GTM 8.2(4) shows numeric fractions are not special)                                 
            "msqrt" | "mroot" => return false,                                           // clause 3
                    // clause 6 only mentions three grouping chars, I'm a little suspicious of that, but that's what it says
            "mrow" => return !(IsBracketed::is_bracketed(mathml, "(", ")", false, false) ||  
                                IsBracketed::is_bracketed(mathml, "[", "]", false, false) || 
                                IsBracketed::is_bracketed(mathml, "{", "}", false, false) ),
            "msub" | "msup" | "msubsup" => {
                // I'm a little dubious about the false value, but see GTM 7.7(2)
                if !is_base {
                    return true;
                } 
                // need to group nested scripts in base -- see GTM 12.2(2)                                         
                let parent = get_parent(mathml);   // there is always a "math" node
                let parent_name = name(parent);   // there is always a "math" node
                return parent_name == "munder" || parent_name == "mover" || parent_name == "munderover";
            },
            _ => return true,
        }

    }
}

impl Function for NeedsToBeGrouped {
    // convert a node to an ordinal number
    fn evaluate<'d>(&self,
                        _context: &context::Evaluation<'_, 'd>,
                        args: Vec<Value<'d>>)
                        -> StdResult<Value<'d>, XPathError>
    {
        let mut args = Args(args);
        args.exactly(3)?;
        let is_base = args.pop_boolean()?;
        let braille_code = args.pop_string()?;
        let node = validate_one_node(args.pop_nodeset()?, "NeedsToBeGrouped")?;
        if let Node::Element(e) = node {
            let answer = match get_braille_code(&braille_code) {
                Some(code) => code.needs_grouping(e, is_base)?,
                None => return Err(XPathError::Other { what: format!("NeedsToBeGrouped: braille code arg '{braille_code:?}' is not a known code ('UEB', 'CMU', or 'Swedish')") }),
            };
            return Ok( Value::Boolean( answer ) );
        }

        return Err(XPathError::Other { what: format!("NeedsToBeGrouped: first arg '{node:?}' is not a node") });
    }
}
    
    
    
#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::init_logger;
    use crate::interface::*;
    use log::debug;

    fn braille_test<F>(f: F) -> Result<()>
    where
        F: FnOnce() -> Result<()> + std::panic::UnwindSafe,
    {
        use std::panic::{catch_unwind, AssertUnwindSafe};
        init_panic_handler();
        let result = catch_unwind(AssertUnwindSafe(f));
        return report_any_panic(result);
    }

    fn init_braille_mathml(mathml: &str) -> Result<()> {
        use std::panic::{catch_unwind, AssertUnwindSafe};
        init_panic_handler();
        let result = catch_unwind(AssertUnwindSafe(|| {
            set_rules_dir(super::super::abs_rules_dir_path())?;
            set_mathml(mathml)?;
            return Ok( () );
        }));
        return report_any_panic(result);
    }

    #[test]
    fn ueb_highlight_24() -> Result<()> {       // issue 24
        return braille_test(|| {
        let mathml_str = "<math display='block' id='id-0'>
            <mrow id='id-1'>
                <mn id='id-2'>4</mn>
                <mo id='id-3'>&#x2062;</mo>
                <mi id='id-4'>a</mi>
                <mo id='id-5'>&#x2062;</mo>
                <mi id='id-6'>c</mi>
            </mrow>
        </math>";
        init_braille_mathml(mathml_str)?;
        set_preference("BrailleCode", "UEB")?;
        set_preference("BrailleNavHighlight", "All")?;
        let braille = get_braille("id-2")?;
        assert_eq!("⣼⣙⠰⠁⠉", braille);
        set_navigation_node("id-2", 0)?;
        assert_eq!( get_braille_position()?, (0,2));

        let braille = get_braille("id-4")?;
        assert_eq!("⠼⠙⣰⣁⠉", braille);
        set_navigation_node("id-4", 0)?;
        assert_eq!( get_braille_position()?, (2,4));
        return Ok( () );
        });
    }
    
    #[test]
    fn ueb_lower_word_sign_contractions() -> Result<()> {
        return braille_test(|| {
        init_braille_mathml("<math><mtext>distance</mtext></math>")?;
        set_preference("BrailleCode", "UEB")?;
        let distance = get_braille("")?;
        // RUEB 10.6.1 / 10.10.4: distance at word start uses dis lower groupsign
        assert_eq!(distance, "⠲⠞⠨⠑", "dis lower groupsign at word start: got '{distance}'");
        // After fraction open (not a RUEB 10.6.2 word start), dis must not apply; st+ance may.
        init_braille_mathml("<math><mfrac><mtext>distance</mtext><mtext>time</mtext></mfrac></math>")?;
        let frac = get_braille("")?;
        assert!(!frac.contains("⠲"), "dis wrongly used after fraction open: got '{frac}'");
        assert!(frac.contains("⠙⠊⠌⠨⠑") || frac.contains("⠙⠊⠎⠞⠁⠝⠉⠑"),
            "expected st+ance or uncontracted distance after fraction open: got '{frac}'");
        init_braille_mathml("<math><mtext>include</mtext></math>")?;
        assert!(get_braille("")?.starts_with("⠔"), "in lower groupsign at word start");
        return Ok(());
        });
    }

    #[test]
    // This test probably should be repeated for each braille code and be taken out of here
    fn find_mathml_from_braille() -> Result<()> { 
        return braille_test(|| {
        use std::time::Instant;
        let mathml_str = "<math id='id-0'>
        <mrow data-changed='added' id='id-1'>
          <mi id='id-2'>x</mi>
          <mo id='id-3'>=</mo>
          <mfrac id='id-4'>
            <mrow id='id-5'>
              <mrow data-changed='added' id='id-6'>
                <mo id='id-7'>-</mo>
                <mi id='id-8'>b</mi>
              </mrow>
              <mo id='id-9'>±</mo>
              <msqrt id='id-10'>
                <mrow data-changed='added' id='id-11'>
                  <msup id='id-12'>
                    <mi id='id-13'>b</mi>
                    <mn id='id-14'>2</mn>
                  </msup>
                  <mo id='id-15'>-</mo>
                  <mrow data-changed='added' id='id-16'>
                    <mn id='id-17'>4</mn>
                    <mo data-changed='added' id='id-18'>&#x2062;</mo>
                    <mi id='id-19'>a</mi>
                    <mo data-changed='added' id='id-20'>&#x2062;</mo>
                    <mi id='id-21'>c</mi>
                  </mrow>
                </mrow>
              </msqrt>
            </mrow>
            <mrow id='id-22'>
              <mn id='id-23'>2</mn>
              <mo data-changed='added' id='id-24'>&#x2062;</mo>
              <mi id='id-25'>a</mi>
            </mrow>
          </mfrac>
        </mrow>
       </math>";
        init_braille_mathml(mathml_str)?;
        set_preference("BrailleNavHighlight", "Off")?;

        set_preference("BrailleCode", "Nemeth")?;
        let _braille = get_braille("")?;
        let answers= &[2, 3, 3, 3, 3, 4, 7, 8, 9, 9,   10, 13, 12, 14, 12, 15, 17, 19, 21, 10,   4, 23, 25, 4];
        let answers = answers.map(|num| format!("id-{}", num));
        debug!("\n*** Testing Nemeth ***");
        for (i, answer) in answers.iter().enumerate() {
            debug!("\n===  i={}  ===", i);
            let instant = Instant::now();
            let (id, _offset) = crate::interface::get_navigation_node_from_braille_position(i)?;
            N_PROBES.with(|n| {debug!("test {:2} #probes = {}", i, n.borrow())});
            debug!("Time taken: {}ms", instant.elapsed().as_millis());
            assert_eq!(*answer, id, "\nNemeth test ith position={}", i);
        }

        set_preference("BrailleCode", "UEB")?;
        let _braille = get_braille("")?;
        let answers= &[2, 2, 3, 3, 3, 3, 4, 4, 4, 7,   7, 8, 9, 9, 10, 13, 12, 14, 14, 15,   15, 17, 17, 19, 19, 21, 10, 4, 4, 23,   23, 25, 25, 4];
        let answers = answers.map(|num| format!("id-{}", num));
        debug!("\n\n*** Testing UEB ***");
        for (i, answer) in answers.iter().enumerate() {
            debug!("\n===  i={}  ===", i);
            let instant = Instant::now();
            let (id, _offset) = crate::interface::get_navigation_node_from_braille_position(i)?;
            N_PROBES.with(|n| {debug!("test {:2} #probes = {}", i, n.borrow())});
            debug!("Time taken: {}ms", instant.elapsed().as_millis());
            assert_eq!(*answer, id, "\nUEB test ith position={}", i);
        }
        set_preference("BrailleCode", "CMU")?;
        let braille = get_braille("")?;
        let answers= &[2, 3, 5, 7, 8, 9, 9, 9, 10, 10,   11, 13, 12, 14, 14, 15, 17, 17, 19, 19,   21, 11, 5, 4, 22, 23, 23, 25, 25, 22,];
        let answers = answers.map(|num| format!("id-{}", num));
        debug!("\n\n*** Testing CMU ***");
        debug!("Braille: {}", braille);
        for (i, answer) in answers.iter().enumerate() {
            debug!("\n===  i={}  ===", i);
            let instant = Instant::now();
            let (id, _offset) = crate::interface::get_navigation_node_from_braille_position(i)?;
            N_PROBES.with(|n| {debug!("test {:2} #probes = {}", i, n.borrow())});
            debug!("Time taken: {}ms", instant.elapsed().as_millis());
            assert_eq!(*answer, id, "\nCMU test ith position={}", i);
        }
        return Ok( () );
        });
    }
    
    #[test]
    #[allow(non_snake_case)]
    fn test_UEB_start_mode() -> Result<()> {
        return braille_test(|| {
        let mathml_str = "<math><msup><mi>x</mi><mi>n</mi></msup></math>";
        init_braille_mathml(mathml_str)?;
        set_preference("BrailleCode", "UEB")?;
        set_preference("UEB_START_MODE", "Grade2")?;
        let braille = get_braille("")?;
        assert_eq!("⠭⠰⠔⠝", braille, "Grade2");
        set_preference("UEB_START_MODE", "Grade1")?;
        let braille = get_braille("")?;
        assert_eq!("⠭⠔⠝", braille, "Grade1");
        return Ok( () );
        });
    }

    #[test]
    fn french_remove_optional_group_indicators() {
        assert_eq!(remove_optional_group_indicators("bLxe+"), "Lx+");
        assert_eq!(remove_optional_group_indicators("bLxeLy"), "oLxcLy");
        assert_eq!(remove_optional_group_indicators("bLxbe+NeLz"), "oLx+NcLz");
        // inner e is followed by outer e, then L — resolve outer pair first
        assert_eq!(remove_optional_group_indicators("bbLx+NeeLy"), "ooLx+NccLy");
        // adjacent pairs: second is removed first, then first sees L after it
        assert_eq!(
            remove_optional_group_indicators("bL⠈N⠹⠜L⠁ebL⠈N⠩⠜L⠃e"),
            "oL⠈N⠹⠜L⠁cL⠈N⠩⠜L⠃"
        );
    }

    /// Letter-to-Unicode-braille mapping (matches handle_contractions).
    const TEST_ASCII_TO_UNICODE: &[char] = &[
        '⠀', '⠮', '⠐', '⠼', '⠫', '⠩', '⠯', '⠄', '⠷', '⠾', '⠡', '⠬', '⠠', '⠤', '⠨', '⠌',
        '⠴', '⠂', '⠆', '⠒', '⠲', '⠢', '⠖', '⠶', '⠦', '⠔', '⠱', '⠰', '⠣', '⠿', '⠜', '⠹',
        '⠈', '⠁', '⠃', '⠉', '⠙', '⠑', '⠋', '⠛', '⠓', '⠊', '⠚', '⠅', '⠇', '⠍', '⠝', '⠕',
        '⠏', '⠟', '⠗', '⠎', '⠞', '⠥', '⠧', '⠺', '⠭', '⠽', '⠵', '⠪', '⠳', '⠻', '⠘', '⠸',
    ];

    fn ascii_to_uncontracted_braille(word: &str) -> String {
        word.bytes()
            .map(|b| TEST_ASCII_TO_UNICODE[(b.to_ascii_uppercase() - 32) as usize])
            .collect()
    }

    /// Run handle_contractions on a lowercase ASCII word (L-cell input) and return Unicode braille.
    fn contract_word(word: &str) -> String {
        let l_chars: Vec<char> = word.bytes()
            .flat_map(|b| {
                let ch = TEST_ASCII_TO_UNICODE[(b.to_ascii_uppercase() - 32) as usize];
                ['L', ch]
            })
            .collect();
        let input: String = l_chars.iter().collect();
        let output = handle_contractions(&l_chars, 0, l_chars.len(), input);
        output.chars().filter(|&c| c != 'L').collect()
    }

    /// Grade-2 contraction coverage for technical vocabulary (311 words).
    /// Cross-check goldens at https://brailletranslators.com/ (Grade 2 / UEB).
    /// Should contract (225): force, work, time, speed, distance, area, height, weight, power, energy, density, gravity, acceleration, length, width, depth, cost, profit, interest, revenue, clearance, conductance, capacitance, resistance, impedance, reactance, variance, tolerance, luminosity, reliability, velocity, viscosity, precision, tension, efficiency, frequency, valence, stiffness, thickness, displacement, concentration, consumption, duration, elevation, fraction, friction, inflation, iteration, population, position, potential, production, resolution, rotation, charge, chord, pitch, growth, head, heat, reach, shear, margin, salary, offset, income, input, inventory, current, error, leverage, temperature, turnover, accuracy, breadth, strength, spread, discount, count, version, inductance, intensity, interval, perimeter, probability, utility, validity, parity, permeability, proportion, uncertainty, circumference, capacity, deceleration, dilution, enthalpy, entropy, equity, expense, gradient, headroom, inertia, latency, longitude, permittivity, period, principal, sensitivity, stress, throughput, thrust, wavelength, absorption, activation, activity, admittance, affinity, albedo, aperture, attenuation, average, bandwidth, baseline, bearing, benchmark, boiling, boundary, cohesion, compliance, compression, conductivity, coverage, damping, deflection, departure, depreciation, deviation, diffusion, dispersion, dissipation, distortion, divergence, downforce, elasticity, emissivity, emission, erosion, evaporation, expansion, expectation, extent, feedback, fidelity, filament, hardness, humidity, hysteresis, illuminance, incidence, index, inflow, influx, inhibition, injection, irradiance, kinetic, leakage, luminance, maturity, mean, median, melting, minimum, mobility, molarity, moisture, opacity, oscillation, outflow, overlap, parallax, parameter, peak, percentile, percentage, polarization, propagation, purity, quality, quantity, quenching, quotient, radiance, radiation, reaction, reactivity, redshift, reflectance, reflection, refraction, reluctance, remanence, resistivity, resonance, restitution, retardation, rigidity, salinity, saturation, scalar, scattering, separation, shrinkage, similarity, battery, diameter, epoch, gain, jitter, joule, momentum, shift, step, stride, zenith
    /// Should not contract (86): mass, volume, base, rate, drag, drift, gap, lag, loss, pace, radius, range, ratio, risk, run, scale, size, slope, tax, torque, capital, impulse, latitude, magnitude, premium, pressure, response, return, trajectory, voltage, aspect, backlog, curvature, dosage, dose, equilibrium, excess, exposure, fatigue, hydraulic, lifetime, likelihood, maximum, metric, orbit, osmosis, pulse, quanta, quantum, regret, residual, ripple, root, rotor, sag, secant, signal, altitude, amplitude, angle, bias, bitrate, budget, bulk, decay, deficit, degree, delay, duty, lift, limit, load, modulus, payload, phase, price, sample, skew, slack, span, supply, total, value, vector, wage, yield
    /// Note: arccos (cc exception, partial ar groupsign) is covered in tests/braille/UEB/other.rs contractions_1.
    #[test]
    fn ueb_technical_word_contractions() {
        const SHOULD_CONTRACT: &[(&str, &str)] = &[
            ("force", "⠿⠉⠑"),
            ("work", "⠐⠺"),
            ("time", "⠐⠞"),
            ("speed", "⠎⠏⠑⠫"),
            ("distance", "⠲⠞⠨⠑"),
            ("area", "⠜⠑⠁"),
            ("height", "⠓⠑⠊⠣⠞"),
            ("weight", "⠺⠑⠊⠣⠞"),
            ("power", "⠏⠪⠻"),
            ("energy", "⠢⠻⠛⠽"),
            ("density", "⠙⠢⠎⠰⠽"),
            ("gravity", "⠛⠗⠁⠧⠰⠽"),
            ("acceleration", "⠁⠒⠑⠇⠻⠁⠰⠝"),
            ("length", "⠇⠢⠛⠹"),
            ("width", "⠺⠊⠙⠹"),
            ("depth", "⠙⠑⠏⠹"),
            ("cost", "⠉⠕⠌"),
            ("profit", "⠏⠗⠷⠊⠞"),
            ("interest", "⠔⠞⠻⠑⠌"),
            ("revenue", "⠗⠑⠧⠢⠥⠑"),
            ("clearance", "⠉⠇⠑⠜⠨⠑"),
            ("conductance", "⠒⠙⠥⠉⠞⠨⠑"),
            ("capacitance", "⠉⠁⠏⠁⠉⠊⠞⠨⠑"),
            ("resistance", "⠗⠑⠎⠊⠌⠨⠑"),
            ("impedance", "⠊⠍⠏⠫⠨⠑"),
            ("reactance", "⠗⠑⠁⠉⠞⠨⠑"),
            ("variance", "⠧⠜⠊⠨⠑"),
            ("tolerance", "⠞⠕⠇⠻⠨⠑"),
            ("luminosity", "⠇⠥⠍⠔⠕⠎⠰⠽"),
            ("reliability", "⠗⠑⠇⠊⠁⠃⠊⠇⠰⠽"),
            ("velocity", "⠧⠑⠇⠕⠉⠰⠽"),
            ("viscosity", "⠧⠊⠎⠉⠕⠎⠰⠽"),
            ("precision", "⠏⠗⠑⠉⠊⠨⠝"),
            ("tension", "⠞⠢⠨⠝"),
            ("efficiency", "⠑⠖⠊⠉⠊⠢⠉⠽"),
            ("frequency", "⠋⠗⠑⠟⠥⠢⠉⠽"),
            ("valence", "⠧⠁⠇⠰⠑"),
            ("stiffness", "⠌⠊⠖⠰⠎"),
            ("thickness", "⠹⠊⠉⠅⠰⠎"),
            ("displacement", "⠲⠏⠇⠁⠉⠑⠰⠞"),
            ("concentration", "⠒⠉⠢⠞⠗⠁⠰⠝"),
            ("consumption", "⠒⠎⠥⠍⠏⠰⠝"),
            ("duration", "⠙⠥⠗⠁⠰⠝"),
            ("elevation", "⠑⠇⠑⠧⠁⠰⠝"),
            ("fraction", "⠋⠗⠁⠉⠰⠝"),
            ("friction", "⠋⠗⠊⠉⠰⠝"),
            ("inflation", "⠔⠋⠇⠁⠰⠝"),
            ("iteration", "⠊⠞⠻⠁⠰⠝"),
            ("population", "⠏⠕⠏⠥⠇⠁⠰⠝"),
            ("position", "⠏⠕⠎⠊⠰⠝"),
            ("potential", "⠏⠕⠞⠢⠞⠊⠁⠇"),
            ("production", "⠏⠗⠕⠙⠥⠉⠰⠝"),
            ("resolution", "⠗⠑⠎⠕⠇⠥⠰⠝"),
            ("rotation", "⠗⠕⠞⠁⠰⠝"),
            ("charge", "⠡⠜⠛⠑"),
            ("chord", "⠡⠕⠗⠙"),
            ("pitch", "⠏⠊⠞⠡"),
            ("growth", "⠛⠗⠪⠹"),
            ("head", "⠓⠂⠙"),
            ("heat", "⠓⠂⠞"),
            ("reach", "⠗⠂⠡"),
            ("shear", "⠩⠑⠜"),
            ("margin", "⠍⠜⠛⠔"),
            ("salary", "⠎⠁⠇⠜⠽"),
            ("offset", "⠷⠋⠎⠑⠞"),
            ("income", "⠔⠉⠕⠍⠑"),
            ("input", "⠔⠏⠥⠞"),
            ("inventory", "⠔⠧⠢⠞⠕⠗⠽"),
            ("current", "⠉⠥⠗⠗⠢⠞"),
            ("error", "⠻⠗⠕⠗"),
            ("leverage", "⠇⠐⠑⠁⠛⠑"),
            ("temperature", "⠞⠑⠍⠏⠻⠁⠞⠥⠗⠑"),
            ("turnover", "⠞⠥⠗⠝⠕⠧⠻"),
            ("accuracy", "⠁⠒⠥⠗⠁⠉⠽"),
            ("breadth", "⠃⠗⠂⠙⠹"),
            ("strength", "⠌⠗⠢⠛⠹"),
            ("spread", "⠎⠏⠗⠂⠙"),
            ("discount", "⠲⠉⠨⠞"),
            ("count", "⠉⠨⠞"),
            ("version", "⠧⠻⠨⠝"),
            ("inductance", "⠔⠙⠥⠉⠞⠨⠑"),
            ("intensity", "⠔⠞⠢⠎⠰⠽"),
            ("interval", "⠔⠞⠻⠧⠁⠇"),
            ("perimeter", "⠏⠻⠊⠍⠑⠞⠻"),
            ("probability", "⠏⠗⠕⠃⠁⠃⠊⠇⠰⠽"),
            ("utility", "⠥⠞⠊⠇⠰⠽"),
            ("validity", "⠧⠁⠇⠊⠙⠰⠽"),
            ("parity", "⠏⠜⠰⠽"),
            ("permeability", "⠏⠻⠍⠂⠃⠊⠇⠰⠽"),
            ("proportion", "⠏⠗⠕⠏⠕⠗⠰⠝"),
            ("uncertainty", "⠥⠝⠉⠻⠞⠁⠔⠞⠽"),
            ("circumference", "⠉⠊⠗⠉⠥⠍⠋⠻⠰⠑"),
            ("capacity", "⠉⠁⠏⠁⠉⠰⠽"),
            ("deceleration", "⠙⠑⠉⠑⠇⠻⠁⠰⠝"),
            ("dilution", "⠙⠊⠇⠥⠰⠝"),
            ("enthalpy", "⠢⠹⠁⠇⠏⠽"),
            ("entropy", "⠢⠞⠗⠕⠏⠽"),
            ("equity", "⠑⠟⠥⠰⠽"),
            ("expense", "⠑⠭⠏⠢⠎⠑"),
            ("gradient", "⠛⠗⠁⠙⠊⠢⠞"),
            ("headroom", "⠓⠂⠙⠗⠕⠕⠍"),
            ("inertia", "⠔⠻⠞⠊⠁"),
            ("latency", "⠇⠁⠞⠢⠉⠽"),
            ("longitude", "⠇⠰⠛⠊⠞⠥⠙⠑"),
            ("permittivity", "⠏⠻⠍⠊⠞⠞⠊⠧⠰⠽"),
            ("period", "⠏⠻⠊⠕⠙"),
            ("principal", "⠏⠗⠔⠉⠊⠏⠁⠇"),
            ("sensitivity", "⠎⠢⠎⠊⠞⠊⠧⠰⠽"),
            ("stress", "⠌⠗⠑⠎⠎"),
            ("throughput", "⠐⠹⠏⠥⠞"),
            ("thrust", "⠹⠗⠥⠌"),
            ("wavelength", "⠺⠁⠧⠑⠇⠢⠛⠹"),
            ("absorption", "⠁⠃⠎⠕⠗⠏⠰⠝"),
            ("activation", "⠁⠉⠞⠊⠧⠁⠰⠝"),
            ("activity", "⠁⠉⠞⠊⠧⠰⠽"),
            ("admittance", "⠁⠙⠍⠊⠞⠞⠨⠑"),
            ("affinity", "⠁⠖⠔⠰⠽"),
            ("albedo", "⠁⠇⠃⠫⠕"),
            ("aperture", "⠁⠏⠻⠞⠥⠗⠑"),
            ("attenuation", "⠁⠞⠞⠢⠥⠁⠰⠝"),
            ("average", "⠁⠧⠻⠁⠛⠑"),
            ("bandwidth", "⠃⠯⠺⠊⠙⠹"),
            ("baseline", "⠃⠁⠎⠑⠇⠔⠑"),
            ("bearing", "⠃⠑⠜⠬"),
            ("benchmark", "⠃⠢⠡⠍⠜⠅"),
            ("boiling", "⠃⠕⠊⠇⠬"),
            ("boundary", "⠃⠨⠙⠜⠽"),
            ("cohesion", "⠉⠕⠓⠑⠨⠝"),
            ("compliance", "⠉⠕⠍⠏⠇⠊⠨⠑"),
            ("compression", "⠉⠕⠍⠏⠗⠑⠎⠨⠝"),
            ("conductivity", "⠒⠙⠥⠉⠞⠊⠧⠰⠽"),
            ("coverage", "⠉⠕⠧⠻⠁⠛⠑"),
            ("damping", "⠙⠁⠍⠏⠬"),
            ("deflection", "⠙⠑⠋⠇⠑⠉⠰⠝"),
            ("departure", "⠙⠑⠐⠏⠥⠗⠑"),
            ("depreciation", "⠙⠑⠏⠗⠑⠉⠊⠁⠰⠝"),
            ("deviation", "⠙⠑⠧⠊⠁⠰⠝"),
            ("diffusion", "⠙⠊⠖⠥⠨⠝"),
            ("dispersion", "⠲⠏⠻⠨⠝"),
            ("dissipation", "⠲⠎⠊⠏⠁⠰⠝"),
            ("distortion", "⠲⠞⠕⠗⠰⠝"),
            ("divergence", "⠙⠊⠧⠻⠛⠰⠑"),
            ("downforce", "⠙⠪⠝⠿⠉⠑"),
            ("elasticity", "⠑⠇⠁⠌⠊⠉⠰⠽"),
            ("emissivity", "⠑⠍⠊⠎⠎⠊⠧⠰⠽"),
            ("emission", "⠑⠍⠊⠎⠨⠝"),
            ("erosion", "⠻⠕⠨⠝"),
            ("evaporation", "⠑⠧⠁⠏⠕⠗⠁⠰⠝"),
            ("expansion", "⠑⠭⠏⠁⠝⠨⠝"),
            ("expectation", "⠑⠭⠏⠑⠉⠞⠁⠰⠝"),
            ("extent", "⠑⠭⠞⠢⠞"),
            ("feedback", "⠋⠑⠫⠃⠁⠉⠅"),
            ("fidelity", "⠋⠊⠙⠑⠇⠰⠽"),
            ("filament", "⠋⠊⠇⠁⠰⠞"),
            ("hardness", "⠓⠜⠙⠰⠎"),
            ("humidity", "⠓⠥⠍⠊⠙⠰⠽"),
            ("hysteresis", "⠓⠽⠌⠻⠑⠎⠊⠎"),
            ("illuminance", "⠊⠇⠇⠥⠍⠔⠨⠑"),
            ("incidence", "⠔⠉⠊⠙⠰⠑"),
            ("index", "⠔⠙⠑⠭"),
            ("inflow", "⠔⠋⠇⠪"),
            ("influx", "⠔⠋⠇⠥⠭"),
            ("inhibition", "⠔⠓⠊⠃⠊⠰⠝"),
            ("injection", "⠔⠚⠑⠉⠰⠝"),
            ("irradiance", "⠊⠗⠗⠁⠙⠊⠨⠑"),
            ("kinetic", "⠅⠔⠑⠞⠊⠉"),
            ("leakage", "⠇⠂⠅⠁⠛⠑"),
            ("luminance", "⠇⠥⠍⠔⠨⠑"),
            ("maturity", "⠍⠁⠞⠥⠗⠰⠽"),
            ("mean", "⠍⠂⠝"),
            ("median", "⠍⠫⠊⠁⠝"),
            ("melting", "⠍⠑⠇⠞⠬"),
            ("minimum", "⠍⠔⠊⠍⠥⠍"),
            ("mobility", "⠍⠕⠃⠊⠇⠰⠽"),
            ("molarity", "⠍⠕⠇⠜⠰⠽"),
            ("moisture", "⠍⠕⠊⠌⠥⠗⠑"),
            ("opacity", "⠕⠏⠁⠉⠰⠽"),
            ("oscillation", "⠕⠎⠉⠊⠇⠇⠁⠰⠝"),
            ("outflow", "⠳⠞⠋⠇⠪"),
            ("overlap", "⠕⠧⠻⠇⠁⠏"),
            ("parallax", "⠏⠜⠁⠇⠇⠁⠭"),
            ("parameter", "⠏⠜⠁⠍⠑⠞⠻"),
            ("peak", "⠏⠂⠅"),
            ("percentile", "⠏⠻⠉⠢⠞⠊⠇⠑"),
            ("percentage", "⠏⠻⠉⠢⠞⠁⠛⠑"),
            ("polarization", "⠏⠕⠇⠜⠊⠵⠁⠰⠝"),
            ("propagation", "⠏⠗⠕⠏⠁⠛⠁⠰⠝"),
            ("purity", "⠏⠥⠗⠰⠽"),
            ("quality", "⠟⠥⠁⠇⠰⠽"),
            ("quantity", "⠟⠥⠁⠝⠞⠰⠽"),
            ("quenching", "⠟⠥⠢⠡⠬"),
            ("quotient", "⠟⠥⠕⠞⠊⠢⠞"),
            ("radiance", "⠗⠁⠙⠊⠨⠑"),
            ("radiation", "⠗⠁⠙⠊⠁⠰⠝"),
            ("reaction", "⠗⠑⠁⠉⠰⠝"),
            ("reactivity", "⠗⠑⠁⠉⠞⠊⠧⠰⠽"),
            ("redshift", "⠗⠫⠩⠊⠋⠞"),
            ("reflectance", "⠗⠑⠋⠇⠑⠉⠞⠨⠑"),
            ("reflection", "⠗⠑⠋⠇⠑⠉⠰⠝"),
            ("refraction", "⠗⠑⠋⠗⠁⠉⠰⠝"),
            ("reluctance", "⠗⠑⠇⠥⠉⠞⠨⠑"),
            ("remanence", "⠗⠑⠍⠁⠝⠰⠑"),
            ("resistivity", "⠗⠑⠎⠊⠌⠊⠧⠰⠽"),
            ("resonance", "⠗⠑⠎⠕⠝⠨⠑"),
            ("restitution", "⠗⠑⠌⠊⠞⠥⠰⠝"),
            ("retardation", "⠗⠑⠞⠜⠙⠁⠰⠝"),
            ("rigidity", "⠗⠊⠛⠊⠙⠰⠽"),
            ("salinity", "⠎⠁⠇⠔⠰⠽"),
            ("saturation", "⠎⠁⠞⠥⠗⠁⠰⠝"),
            ("scalar", "⠎⠉⠁⠇⠜"),
            ("scattering", "⠎⠉⠁⠞⠞⠻⠬"),
            ("separation", "⠎⠑⠏⠜⠁⠰⠝"),
            ("shrinkage", "⠩⠗⠔⠅⠁⠛⠑"),
            ("similarity", "⠎⠊⠍⠊⠇⠜⠰⠽"),
            ("battery", "⠃⠁⠞⠞⠻⠽"),
            ("diameter", "⠙⠊⠁⠍⠑⠞⠻"),
            ("epoch", "⠑⠏⠕⠡"),
            ("gain", "⠛⠁⠔"),
            ("jitter", "⠚⠊⠞⠞⠻"),
            ("joule", "⠚⠳⠇⠑"),
            ("momentum", "⠍⠕⠰⠞⠥⠍"),
            ("shift", "⠩⠊⠋⠞"),
            ("step", "⠌⠑⠏"),
            ("stride", "⠌⠗⠊⠙⠑"),
            ("zenith", "⠵⠢⠊⠹"),
        ];
        const SHOULD_NOT_CONTRACT: &[&str] = &[
            "mass", "volume", "base", "rate", "drag", "drift", "gap", "lag", "loss", "pace",
            "radius", "range", "ratio", "risk", "run", "scale", "size", "slope", "tax", "torque",
            "capital", "impulse", "latitude", "magnitude", "premium", "pressure", "response", "return",
            "trajectory", "voltage", "aspect", "backlog", "curvature", "dosage", "dose", "equilibrium",
            "excess", "exposure", "fatigue", "hydraulic", "lifetime", "likelihood", "maximum", "metric",
            "orbit", "osmosis", "pulse", "quanta", "quantum", "regret", "residual", "ripple", "root",
            "rotor", "sag", "secant", "signal", "altitude", "amplitude", "angle", "bias", "bitrate",
            "budget", "bulk", "decay", "deficit", "degree", "delay", "duty", "lift", "limit", "load",
            "modulus", "payload", "phase", "price", "sample", "skew", "slack", "span", "supply",
            "total", "value", "vector", "wage", "yield",
        ];

        for &(word, expected) in SHOULD_CONTRACT {
            let got = contract_word(word);
            assert_eq!(expected, got, "word '{word}'");
            assert_ne!(ascii_to_uncontracted_braille(word), got, "word '{word}' should contract");
        }
        for &word in SHOULD_NOT_CONTRACT {
            let got = contract_word(word);
            let uncontracted = ascii_to_uncontracted_braille(word);
            assert_eq!(uncontracted, got, "word '{word}' should not contract");
        }
    }

    #[test]
    #[ignore]
    fn ueb_technical_word_contractions_probe_batch2() {
        let try_contract = [
            "circumference", "capacity", "deceleration", "dilution", "enthalpy", "entropy", "equity", "expense",
            "gradient", "headroom", "inertia", "latency", "longitude", "permittivity", "period", "principal",
            "sensitivity", "stress", "throughput", "thrust", "wavelength", "absorption", "activation", "activity",
            "admittance", "affinity", "albedo", "aperture", "attenuation", "average", "bandwidth", "baseline",
            "bearing", "benchmark", "boiling", "boundary", "cohesion", "compliance", "compression", "conductivity",
            "coverage", "damping", "deflection", "departure", "depreciation", "deviation", "diffusion", "dispersion",
            "dissipation", "distortion", "divergence", "downforce", "elasticity", "emissivity", "emission", "erosion",
            "evaporation", "expansion", "expectation", "extent", "feedback", "fidelity", "filament", "hardness",
            "humidity", "hysteresis", "illuminance", "incidence", "index", "inflow", "influx", "inhibition",
            "injection", "irradiance", "kinetic", "leakage", "luminance", "maturity", "mean", "median", "melting",
            "minimum", "mobility", "molarity", "moisture", "opacity", "oscillation", "outflow", "overlap", "parallax",
            "parameter", "peak", "percentile", "percentage", "polarization", "propagation", "purity", "quality",
            "quantity", "quenching", "quotient", "radiance", "radiation", "reaction", "reactivity", "redshift",
            "reflectance", "reflection", "refraction", "reluctance", "remanence", "resistivity", "resonance",
            "restitution", "retardation", "rigidity", "salinity", "saturation", "scalar", "scattering", "separation",
            "shrinkage", "similarity", "battery", "diameter", "epoch", "gain", "jitter", "joule", "momentum", "shift",
            "step", "stride", "zenith",
        ];
        let try_not = [
            "capital", "impulse", "latitude", "magnitude", "premium", "pressure", "response", "return", "trajectory",
            "voltage", "aspect", "backlog", "curvature", "dosage", "dose", "equilibrium", "excess", "exposure",
            "fatigue", "hydraulic", "lifetime", "likelihood", "maximum", "metric", "orbit", "osmosis", "pulse",
            "quanta", "quantum", "regret", "residual", "ripple", "root", "rotor", "sag", "secant", "signal",
            "altitude", "amplitude", "angle", "bias", "bitrate", "budget", "bulk", "decay", "deficit", "degree",
            "delay", "duty", "lift", "limit", "load", "modulus", "payload", "phase", "price", "sample", "skew",
            "slack", "span", "supply", "total", "value", "vector", "wage", "yield",
        ];
        let mut out = String::new();
        for w in try_contract {
            let c = contract_word(w);
            out.push_str(&format!("(\"{w}\", \"{c}\"),\n"));
        }
        for w in try_not {
            out.push_str(&format!("// no: {w}\n"));
        }
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("notes/_batch2_goldens.txt");
        std::fs::write(path, out).unwrap();
    }

    #[test]
    #[ignore]
    fn ueb_technical_word_contractions_probe() {
        let should_contract = [
            "force", "work", "time", "speed", "distance", "area", "height", "weight", "power", "energy",
            "density", "gravity", "acceleration", "length", "width", "depth", "cost", "profit", "interest", "revenue",
            "clearance", "conductance", "capacitance", "resistance", "impedance", "reactance", "variance", "tolerance",
            "luminosity", "reliability", "velocity", "viscosity", "precision", "tension", "efficiency", "frequency",
            "valence", "stiffness", "thickness", "displacement", "concentration", "consumption", "duration", "elevation",
            "fraction", "friction", "inflation", "iteration", "population", "position", "potential", "production",
            "resolution", "rotation", "charge", "chord", "pitch", "growth", "head", "heat", "reach", "shear", "margin",
            "salary", "offset", "income", "input", "inventory", "current", "error", "leverage", "temperature",
            "turnover", "accuracy", "breadth", "strength", "spread", "discount", "count", "version", "inductance",
            "intensity", "interval", "perimeter", "probability", "utility", "validity", "parity", "permeability",
            "proportion", "uncertainty",
        ];
        let should_not = [
            "mass", "volume", "base", "rate", "drag", "drift", "gap", "lag", "loss", "pace",
            "radius", "range", "ratio", "risk", "run", "scale", "size", "slope", "tax", "torque",
        ];
        eprintln!("=== SHOULD CONTRACT ===");
        for w in should_contract {
            let c = contract_word(w);
            let u = ascii_to_uncontracted_braille(w);
            eprintln!("(\"{w}\", \"{c}\"), // uncontracted={u} changed={}", c != u);
        }
        eprintln!("=== SHOULD NOT CONTRACT ===");
        for w in should_not {
            let c = contract_word(w);
            let u = ascii_to_uncontracted_braille(w);
            eprintln!("{w}: contracted={c} uncontracted={u} ok={}", c == u);
        }
    }
}
