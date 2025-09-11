#![allow(clippy::needless_return)]
use strum_macros::Display;
use sxd_document::dom::{Element, ChildOfElement};
use sxd_document::Package;
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

static UEB_PREFIXES: phf::Set<char> = phf_set! {
    '⠼', '⠈', '⠘', '⠸', '⠐', '⠨', '⠰', '⠠',
};

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
        let mut rules_with_context = SpeechRulesWithContext::new(&rules, new_package.as_document(), nav_node_id);
        let braille_string = rules_with_context.match_pattern::<String>(mathml)
                        .chain_err(|| "Pattern match/replacement failure!")?;
        // debug!("braille_mathml: braille string: {}", &braille_string);
        let braille_string = braille_string.replace(' ', "");
        let pref_manager = rules_with_context.get_rules().pref_manager.borrow();
        let highlight_style = pref_manager.pref_to_string("BrailleNavHighlight");
        let braille_code = pref_manager.pref_to_string("BrailleCode");
        let braille = match braille_code.as_str() {
            "Nemeth" => nemeth_cleanup(pref_manager, braille_string),
            "UEB" => ueb_cleanup(pref_manager, braille_string),
            "Vietnam" => vietnam_cleanup(pref_manager, braille_string),
            "CMU" => cmu_cleanup(pref_manager, braille_string), 
            "Finnish" => finnish_cleanup(pref_manager, braille_string),
            "Swedish" => swedish_cleanup(pref_manager, braille_string),
            "LaTeX" => LaTeX_cleanup(pref_manager, braille_string),
            "ASCIIMath" => ASCIIMath_cleanup(pref_manager, braille_string),
            _ => braille_string.trim_matches('⠀').to_string(),    // probably needs cleanup if someone has another code, but this will have to get added by hand
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
            if prefix_ch_index == 0 && braille_code == "UEB" {
                // don't count the word or passage mode as part of a indicator
                if braille.starts_with("⠰⠰⠰") {
                    prefix_ch_index = 9;
                } else if braille.starts_with("⠰⠰") {
                    prefix_ch_index = 6;
                }
            }
            let indicators = &braille[prefix_ch_index..start_index];   // chars to be examined
            let i_byte_start = start_index - 3 * match braille_code {
                "Nemeth" => i_start_nemeth(indicators, first_ch),
                _ => i_start_ueb(indicators),               // treat all the other like UEB because they probably have similar number and letter prefixes
            };
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
        /// Currently, this only does something for CMU braille
        fn expand_highlight(braille: &mut String, braille_code: &str, start_index: usize, end_index: usize) -> Option<(usize, usize)> {
            // For CMU, we want to expand mrows to include the opening and closing grouping indicators if they exist
            if start_index == 0 || end_index == braille.len() || braille_code != "CMU" {
                return None;
            }

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

    /// Given a position in a Nemeth string, what is the position character that starts it (e.g, the prev char for capital letter)
    fn i_start_nemeth(braille_prefix: &str, first_ch: char) -> usize {
        static NEMETH_NUMBERS: phf::Set<char> = phf_set! {
            '⠂', '⠆', '⠒', '⠲', '⠢', '⠖', '⠶', '⠦', '⠔', '⠴', '⠨' // 1, 2, ...9, 0, decimal pt
        };
        let mut n_chars = 0;
        let prefix = &mut braille_prefix.chars().rev().peekable();
        if prefix.peek() == Some(&'⠠') ||  // cap indicator
           (prefix.peek() == Some(&'⠼') && NEMETH_NUMBERS.contains(&first_ch)) ||  // number indicator
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
            if UEB_PREFIXES.contains(&ch) {
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
        static UEB_TYPEFORM_PREFIXES: phf::Set<char> = phf_set! {
            '⠈', '⠘', '⠸', '⠨',
        };

        if let Some(typeform_indicator) = prefix.next() {
            if UEB_TYPEFORM_PREFIXES.contains(&typeform_indicator) {
                return 2;
            } else if typeform_indicator == '⠼' {
                if let Some(user_defined_typeform_indicator) = prefix.next() {
                    if UEB_TYPEFORM_PREFIXES.contains(&user_defined_typeform_indicator) || user_defined_typeform_indicator == '⠐' {
                        return 3;
                    }
                }
            }
        }
        return 0;
    }
}

// FIX: if 8-dot braille is needed, perhaps the highlights can be shifted to a "highlighted" 256 char block in private space 
//   they would need to be unshifted for the external world
fn is_highlighted(ch: char) -> bool {
    let ch_as_u32 = ch as u32;
    return (0x28C0..0x28FF).contains(&ch_as_u32) || ch == '𝑏';           // 0x28C0..0x28FF all have dots 7 & 8 on
}

fn highlight(ch: char) -> char {
    return unsafe{char::from_u32_unchecked(ch as u32 | 0xC0)};    // 0x28C0..0x28FF all have dots 7 & 8 on
}

fn unhighlight(ch: char) -> char {
    let ch_as_u32 = ch as u32;
    if (0x28C0..0x28FF).contains(&ch_as_u32) {              // 0x28C0..0x28FF all have dots 7 & 8 on
        return unsafe{char::from_u32_unchecked(ch_as_u32 & 0x283F)};
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
    let saved_highlight_style = get_preference("BrailleNavHighlight".to_string()).unwrap();
    set_preference("BrailleNavHighlight".to_string(), "EndPoints".to_string()).unwrap();

    N_PROBES.with(|n| {*n.borrow_mut() = 0});
    // dive into the child of the <math> element (should only be one)
    let search_state = find_navigation_node(mathml, as_element(mathml.children()[0]), position)?;
    set_preference("BrailleNavHighlight".to_string(), saved_highlight_style.to_string()).unwrap();

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
        let node_id = match node.attribute_value("id") {
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
            return comparison_operators.contains(as_text(node));
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
        let leaf_name = name(node);
        if is_leaf(node) {
            let text = as_text(node);
            // len() is close since mn's probably have ASCII digits and lower case vars are common (count as) and other chars need extra braille chars
            // don't want to count invisible chars since they don't display and would give a length = 3
            if text == "\u{2061}" || text == "\u{2062}"  {       // invisible function apply/times (most common by far)
                return 0;
            }
            // FIX: this assumption is bad for 8-dot braille
            return match leaf_name {
                "mn" => n_number_indicator + text.len(),
                "mo" => 2,  // could do better by actually brailling char, but that is more expensive
                _ => text.len(),
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

fn nemeth_cleanup(pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String {
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
        "𝔹" => "⠨",     // blackboard
        "T" => "⠈",     // script
        "I" => "⠨",     // italic (mapped to be the same a blackboard)
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

    lazy_static! {
        // Add an English Letter indicator. This involves finding "single letters".
        // The green book has a complicated set of cases, but the Nemeth UEB Rule book (May 2020), 4.10 has a much shorter explanation:
        //   punctuation or whitespace on the left and right ignoring open/close chars
        //   https://nfb.org/sites/www.nfb.org/files/files-pdf/braille-certification/lesson-4--provisional-5-9-20.pdf
        static ref ADD_ENGLISH_LETTER_INDICATOR: Regex = 
            Regex::new(r"(?P<start>^|W|P.[\u2800-\u28FF]?|,)(?P<open>[\u2800-\u28FF]?⠷)?(?P<letter>C?L.)(?P<close>[\u2800-\u28FF]?⠾)?(?P<end>W|P|,|$)").unwrap();
        
        // Trim braille spaces before and after braille indicators
        // In order: fraction, /, cancellation, letter, baseline
        // Note: fraction over is not listed due to example 42(4) which shows a space before the "/"
        static ref REMOVE_SPACE_BEFORE_BRAILLE_INDICATORS: Regex = 
            Regex::new(r"(⠄⠄⠄|⠤⠤⠤⠤)[Ww]+([⠼⠸⠪])").unwrap();
        static ref REMOVE_SPACE_AFTER_BRAILLE_INDICATORS: Regex = 
            Regex::new(r"([⠹⠻Llb])[Ww]+(⠄⠄⠄|⠤⠤⠤⠤)").unwrap();

        // Hack to convert non-numeric '.' to numeric '.'
        // The problem is that the numbers are hidden inside of mover -- this might be more general than rule 99_2.
        static ref DOTS_99_A_2: Regex = Regex::new(r"𝑁⠨mN").unwrap();

        // Punctuation is one or two chars. There are (currently) only 3 2-char punct chars (—‘’) -- we explicitly list them below
        static ref REMOVE_SPACE_BEFORE_PUNCTUATION_151: Regex = 
            Regex::new(r"w(P.[⠤⠦⠠]?|[\u2800-\u28FF]?⠾)").unwrap();
        static ref REMOVE_SPACE_AFTER_PUNCTUATION_151: Regex = 
            Regex::new(r"(P.[⠤⠦⠠]?|[\u2800-\u28FF]?⠷)w").unwrap();

        // Multipurpose indicator insertion
        // 149 -- consecutive comparison operators have no space -- instead a multipurpose indicator is used (doesn't require a regex)

        // 177.2 -- add after a letter and before a digit (or decimal pt) -- digits will start with N
        static ref MULTI_177_2: Regex = 
            Regex::new(r"([Ll].)[N𝑁]").unwrap();

        // keep between numeric subscript and digit ('M' added by subscript rule)
        static ref MULTI_177_3: Regex = 
            Regex::new(r"([N𝑁].)M([N𝑁].)").unwrap();

        // Add after decimal pt for non-digits except for comma and punctuation
        // Note: since "." can be in the middle of a number, there is not necessarily a "N"
        // Although not mentioned in 177_5, don't add an 'M' before an 'm'
        static ref MULTI_177_5: Regex = 
            Regex::new(r"([N𝑁]⠨)([^⠂⠆⠒⠲⠢⠖⠶⠦⠔N𝑁,Pm])").unwrap();


        // Pattern for rule II.9a (add numeric indicator at start of line or after a space)
        // 1. start of line
        // 2. optional minus sign (⠤)
        // 3. optional typeface indicator
        // 4. number (N)
        static ref NUM_IND_9A: Regex = 
            Regex::new(r"(?P<start>^|[,Ww])(?P<minus>⠤?)N").unwrap();

        // Needed after section mark(§), paragraph mark(¶), #, or *
        static ref NUM_IND_9C: Regex = 
            Regex::new(r"(⠤?)(⠠⠷|⠠⠳|⠠⠈⠷)N").unwrap();

        // Needed after section mark(§), paragraph mark(¶), #, or *
        static ref NUM_IND_9D: Regex = 
            Regex::new(r"(⠈⠠⠎|⠈⠠⠏|⠨⠼|⠈⠼)N").unwrap();

        // Needed after a typeface change or interior shape modifier indicator
        static ref NUM_IND_9E: Regex = Regex::new(r"(?P<face>[SB𝔹TIR]+?)N").unwrap();
        static ref NUM_IND_9E_SHAPE: Regex = Regex::new(r"(?P<mod>⠸⠫)N").unwrap();

        // Needed after hyphen that follows a word, abbreviation, or punctuation (caution about rule 11d)
        // Note -- hyphen might encode as either "P⠤" or "⠤" depending on the tag used
        static ref NUM_IND_9F: Regex = Regex::new(r"([Ll].[Ll].|P.)(P?⠤)N").unwrap();

        // Enclosed list exception
        // Normally we don't add numeric indicators in enclosed lists (done in get_braille_nemeth_chars).
        // The green book says "at the start" of an item, don't add the numeric indicator.
        // The NFB list exceptions after function abbreviations and angles, but what this really means is "after a space"
        static ref NUM_IND_ENCLOSED_LIST: Regex = Regex::new(r"w([⠂⠆⠒⠲⠢⠖⠶⠦⠔⠴])").unwrap();

        // Punctuation chars (Rule 38.6 says don't use before ",", "hyphen", "-", "…")
        // Never use punctuation indicator before these (38-6)
        //      "…": "⠀⠄⠄⠄"
        //      "-": "⠸⠤" (hyphen and dash)
        //      ",": "⠠⠀"     -- spacing already added
        // Rule II.9b (add numeric indicator after punctuation [optional minus[optional .][digit]
        //  because this is run after the above rule, some cases are already caught, so don't
        //  match if there is already a numeric indicator
        static ref NUM_IND_9B: Regex = Regex::new(r"(?P<punct>P..?)(?P<minus>⠤?)N").unwrap();

        // Before 79b (punctuation)
        static ref REMOVE_LEVEL_IND_BEFORE_SPACE_COMMA_PUNCT: Regex = Regex::new(r"(?:[↑↓]+[b𝑏]?|[b𝑏])([Ww,P]|$)").unwrap();

        // Most commas have a space after them, but not when followed by a close quote (others?)
        static ref NO_SPACE_AFTER_COMMA: Regex = Regex::new(r",P⠴").unwrap();      // captures both single and double close quote
        static ref REMOVE_LEVEL_IND_BEFORE_BASELINE: Regex = Regex::new(r"(?:[↑↓mb𝑏]+)([b𝑏])").unwrap();

        // Except for the four chars above, the unicode rules always include a punctuation indicator.
        // The cases to remove them (that seem relevant to MathML) are:
        //   Beginning of line or after a space (V 38.1)
        //   After a word (38.4)
        //   2nd or subsequent punctuation (includes, "-", etc) (38.7)
        static ref REMOVE_AFTER_PUNCT_IND: Regex = Regex::new(r"(^|[Ww]|[Ll].[Ll].)P(.)").unwrap();
        static ref REPLACE_INDICATORS: Regex =Regex::new(r"([SB𝔹TIREDGVHUP𝐏CLlMmb𝑏↑↓Nn𝑁Ww,])").unwrap();
        static ref COLLAPSE_SPACES: Regex = Regex::new(r"⠀⠀+").unwrap();
    }

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

    // these typeforms need to get pulled from user-prefs as they are transcriber-defined
    let sans_serif = pref_manager.pref_to_string("Nemeth_SansSerif");
    let bold = pref_manager.pref_to_string("Nemeth_Bold");
    let double_struck = pref_manager.pref_to_string("Nemeth_DoubleStruck");
    let script = pref_manager.pref_to_string("Nemeth_Script");
    let italic = pref_manager.pref_to_string("Nemeth_Italic");

    let result = REPLACE_INDICATORS.replace_all(&result, |cap: &Captures| {
        let matched_char = &cap[0];
        match matched_char {
            "S" => &sans_serif,
            "B" => &bold,
            "𝔹" => &double_struck,
            "T" => &script,
            "I" => &italic,
            _ => match NEMETH_INDICATOR_REPLACEMENTS.get(&cap[0]) {
                None => {error!("REPLACE_INDICATORS and NEMETH_INDICATOR_REPLACEMENTS are not in sync"); ""},
                Some(&ch) => ch,
            }
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
        if highlight_style == "EndPoints" {
            if let Some(last_highlighted) = braille.rfind(is_highlighted) {
                if braille[last_highlighted..].starts_with('𝑏') {
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
    "I" => "⠨",     // italic
    "R" => "",      // roman
    // "E" => "⠰",     // English
    "1" => "⠰",      // Grade 1 symbol
    "𝟙" => "⠰⠰",     // Grade 1 word
    "L" => "",       // Letter left in to assist in locating letters
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

static LETTER_NUMBERS: phf::Set<char> = phf_set! {
    '⠁', '⠃', '⠉', '⠙', '⠑', '⠋', '⠛', '⠓', '⠊', '⠚',
};

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

static LETTER_PREFIXES: phf::Set<char> = phf_set! {
    'B', 'I', '𝔹', 'S', 'T', 'D', 'C', '𝐶', '𝑐',
};

lazy_static! {
    // Trim braille spaces before and after braille indicators
    // In order: fraction, /, cancellation, letter, baseline
    // Note: fraction over is not listed due to example 42(4) which shows a space before the "/"
    // static ref REMOVE_SPACE_BEFORE_BRAILLE_INDICATORS: Regex = 
    //     Regex::new(r"(⠄⠄⠄|⠤⠤⠤)W+([⠼⠸⠪])").unwrap();
    static ref REPLACE_INDICATORS: Regex =Regex::new(r"([1𝟙SB𝔹TIREDGVHP𝐶𝑐CLMNW𝐖swe,.-—―#ocb])").unwrap();
    static ref COLLAPSE_SPACES: Regex = Regex::new(r"⠀⠀+").unwrap();
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
    
    // '𝐖' is a hard break -- basically, it separates exprs
    let mut result = result.split('𝐖')
                        .map(|str| pick_start_mode(str, use_only_grade1) + "W")
                        .collect::<String>();
    result.pop();   // we added a 'W' at the end that needs to be removed.

    let result = result.replace("tW", "W");

    // these typeforms need to get pulled from user-prefs as they are transcriber-defined
    let double_struck = pref_manager.pref_to_string("UEB_DoubleStruck");
    let sans_serif = pref_manager.pref_to_string("UEB_SansSerif");
    let fraktur = pref_manager.pref_to_string("UEB_Fraktur");
    let greek_variant = pref_manager.pref_to_string("UEB_GreekVariant");

    let result = REPLACE_INDICATORS.replace_all(&result, |cap: &Captures| {
        let matched_char = &cap[0];
        match matched_char {
            "𝔹" => &double_struck,
            "S" => &sans_serif,
            "D" => &fraktur,
            "V" => &greek_variant,
            _ => match UEB_INDICATOR_REPLACEMENTS.get(matched_char) {
                None => {error!("REPLACE_INDICATORS and UEB_INDICATOR_REPLACEMENTS are not in sync: missing '{matched_char}'"); ""},
                Some(&ch) => ch,
            },
        }
    });

    // Remove unicode blanks at start and end -- do this after the substitutions because ',' introduces spaces
    // let result = result.trim_start_matches('⠀').trim_end_matches('⠀');
    let result = COLLAPSE_SPACES.replace_all(&result, "⠀");
   
    return result.to_string();

    fn pick_start_mode(raw_braille: &str, use_only_grade1: bool) -> String {
        // Need to decide what the start mode should be
        // From http://www.brailleauthority.org/ueb/ueb_math_guidance/final_for_posting_ueb_math_guidance_may_2019_102419.pdf
        //   Unless a math expression can be correctly represented with only a grade 1 symbol indicator in the first three cells
        //   or before a single letter standing alone anywhere in the expression,
        //   begin the expression with a grade 1 word indicator (or a passage indicator if the expression includes spaces)
        // Apparently "only a grade 1 symbol..." means at most one grade 1 symbol based on some examples (GTM 6.4, example 4)
        // debug!("before determining mode:  '{}'", raw_braille);

        // a bit ugly because we need to store the string if we have cap passage mode
        let raw_braille_string = if is_cap_passage_mode_good(raw_braille) {convert_to_cap_passage_mode(raw_braille)} else {String::default()};
        let raw_braille = if raw_braille_string.is_empty() {raw_braille} else {&raw_braille_string};
        if use_only_grade1 {
            return remove_unneeded_mode_changes(raw_braille, UEB_Mode::Grade1, UEB_Duration::Passage);
        }
        let grade2 = remove_unneeded_mode_changes(raw_braille, UEB_Mode::Grade2, UEB_Duration::Symbol);
        // debug!("Symbol mode:  '{}'", grade2);

        if is_grade2_string_ok(&grade2) {
            return grade2;
        } else {
            // BANA says use g1 word mode if spaces are present, but that's not what their examples do
            // A conversation with Ms. DeAndrea from BANA said that they mean use passage mode if ≥3 "segments" (≥2 blanks)
            // The G1 Word mode might not be at the start (iceb.rs:omission_3_6_7)
            let grade1_word = try_grade1_word_mode(raw_braille);
            // debug!("Word mode:    '{}'", grade1_word);
            if !grade1_word.is_empty() {
                return grade1_word;
            } else {
                let grade1_passage = remove_unneeded_mode_changes(raw_braille, UEB_Mode::Grade1, UEB_Duration::Passage);
                return "⠰⠰⠰".to_string() + &grade1_passage + "⠰⠄";
            }
        }

        /// Return true if at least five (= # of cap passage indicators) cap indicators and no lower case letters
        fn is_cap_passage_mode_good(braille: &str) -> bool {
            let mut n_caps = 0;
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
                    if is_cap_mode {
                        assert!(cap_mode == UEB_Duration::Word);
                    }
                    is_cap_mode = false;
                } else if ch == '1' && is_cap_mode {
                    break;
                }
            }
            return n_caps > 4;
        }

        fn convert_to_cap_passage_mode(braille: &str) -> String {
            return "⠠⠠⠠".to_string() + &braille.replace(['C', '𝐶'], "") + "⠠⠄";
        }

        /// Return true if the BANA or ICEB guidelines say it is ok to start with grade 2
        fn is_grade2_string_ok(grade2_braille: &str) -> bool {
            // BANA says use grade 2 if there is not more than one grade one symbol or single letter standing alone.
            // The exact quote from their guidance:
            //    Unless a math expression can be correctly represented with only a grade 1 symbol indicator in the first three cells
            //    or before a single letter standing alone anywhere in the expression,
            //    begin the expression with a grade 1 word indicator
            // Note: I modified this slightly to exclude the cap indicator in the count. That allows three more ICEB rule to pass and seems
            //    like it is a reasonable thing to do.
            // Another modification is allow a single G1 indicator to occur after whitespace later on
            //    because ICEB examples show it and it seems better than going to passage mode if it is the only G1 indicator

            // Because of the 'L's which go away, we have to put a little more work into finding the first three chars
            let chars = grade2_braille.chars().collect::<Vec<char>>();
            let mut n_real_chars = 0;  // actually number of chars
            let mut found_g1 = false;
            let mut i = 0;
            while i < chars.len() {
                let ch = chars[i];
                if ch == '1' && !is_forced_grade1(&chars, i) {
                    if found_g1 {
                        return false;
                    }
                    found_g1 = true;
                } else if !"𝐶CLobc".contains(ch) {
                    if n_real_chars == 2 {
                        i += 1;
                        break;              // this is the third real char
                    };
                    n_real_chars += 1;
                }
                i += 1
            }

            // if we find *another* g1 that isn't forced and isn't standing alone, we are done
            // I've added a 'follows whitespace' clause for test iceb.rs:omission_3_6_2 to the standing alone rule
            // we only allow one standing alone example -- not sure if BANA guidance has this limit, but GTM 11_5_5_3 seems better with it
            // Same for GTM 1_7_3_1 (passage mode is mentioned also)
            let mut is_standing_alone_already_encountered = false;
            let mut is_after_whitespace = false;
            while i < chars.len() {
                let ch = chars[i];
                if ch == 'W' {
                    is_after_whitespace = true;
                } else if ch == '1' && !is_forced_grade1(&chars, i) {
                    if is_standing_alone_already_encountered ||
                       ((found_g1 || !is_after_whitespace) && !is_single_letter_on_right(&chars, i)) {
                        return false;
                    }
                    found_g1 = true;
                    is_standing_alone_already_encountered = true;
                }
                i += 1;
            }
            return true;
        }

        /// Return true if the sequence of chars forces a '1' at the `i`th position
        /// Note: `chars[i]` should be '1'
        fn is_forced_grade1(chars: &[char], i: usize) -> bool {
            // A '1' is forced if 'a-j' follows a digit
            assert_eq!(chars[i], '1', "'is_forced_grade1' didn't start with '1'");
            // check that a-j follows the '1' -- we have '1Lx' where 'x' is the letter to check
            if i+2 < chars.len() && LETTER_NUMBERS.contains(&unhighlight(chars[i+2])) {
                // check for a number before the '1'
                // this will be 'N' followed by LETTER_NUMBERS or the number ".", ",", or " "
                for j in (0..i).rev() {
                    let ch = chars[j];
                    if !(LETTER_NUMBERS.contains(&unhighlight(ch)) || ".,W𝐖".contains(ch)) {
                        return ch == 'N'
                    }
                }
            }
            return false;
        }

        fn is_single_letter_on_right(chars: &[char], i: usize) -> bool {
            static SKIP_CHARS: phf::Set<char> = phf_set! {
                'B', 'I', '𝔹', 'S', 'T', 'D', 'C', '𝐶', 's', 'w'   // indicators
            };

            // find the first char (if any)
            let mut count = 0;      // how many letters
            let mut i = i+1;
            while i < chars.len() {
                let ch = chars[i];
                if !SKIP_CHARS.contains(&ch) {
                    if ch == 'L' {
                        if count == 1 {
                            return false;   // found a second letter in the sequence
                        }
                        count += 1;
                    } else {
                        return count==1;
                    }
                    i += 2;   // eat 'L' and actual letter
                } else {
                    i += 1;
                }
            }
            return true;
        }

        fn try_grade1_word_mode(raw_braille: &str) -> String {
            // this isn't quite right, but pretty close -- try splitting at 'W' (words)
            // only one of the parts can be in word mode and none of the others can have '1' unless forced
            let mut g1_words = Vec::default();
            let mut found_word_mode = false;
            for raw_word in raw_braille.split('W') {
                let word = remove_unneeded_mode_changes(raw_word, UEB_Mode::Grade2, UEB_Duration::Symbol);
                // debug!("try_grade1_word_mode: word='{}'", word);
                let word_chars = word.chars().collect::<Vec<char>>();
                let needs_word_mode = word_chars.iter().enumerate()
                    .any(|(i, &ch) | ch == '1' && !is_forced_grade1(&word_chars, i));
                if needs_word_mode {
                    if found_word_mode {
                        return "".to_string();
                    }
                    found_word_mode = true;
                    g1_words.push("⠰⠰".to_string() + &remove_unneeded_mode_changes(raw_word, UEB_Mode::Grade1, UEB_Duration::Word)
                    );
                } else {
                    g1_words.push(word);
                }
            }
            return if found_word_mode {g1_words.join("W")} else {"".to_string()};
        }
    }
}

fn typeface_to_word_mode(braille: &str) -> String {
    lazy_static! {
        static ref HAS_TYPEFACE: Regex = Regex::new("[BI𝔹STD]").unwrap();
    }
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
            if chars[next_non_cap] != 'L' {
                error!("capitals_to_word_mode: internal error: didn't find L after C in '{}'.",
                       chars[i..next_non_cap+2].iter().collect::<String>().as_str());
            }
            let i_braille_char = next_non_cap + 2;
            result.push_str(String::from_iter(&chars[i..i_braille_char]).as_str());
            i = i_braille_char;
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
                    if ch == 'L' || ch == 'N' || !LETTER_PREFIXES.contains(&ch) {
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
                if ch == 'L' || ch == 'N' || !LETTER_PREFIXES.contains(&ch) {
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
static LEFT_INTERVENING_CHARS: phf::Set<char> = phf_set! {  // see RUEB 2.6.2
    'B', 'I', '𝔹', 'S', 'T', 'D', 'C', '𝐶', 's', 'w',     // indicators
    // opening chars have prefix 'o', so not in set ['(', '{', '[', '"', '\'', '“', '‘', '«'] 
};

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
                        if LETTER_NUMBERS.contains(&unhighlight(chars[i+1])) {
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
                        if i+1 < chars.len() && chars[i] == 'L' && LETTER_NUMBERS.contains(&unhighlight(chars[i+1])) {
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
                        // let prev_ch = if i > 1 {chars[i-1]} else {'1'};   // '1' -- anything beside ',' or '.'
                        // if duration == UEB_Duration::Symbol || 
                        //     ( ",. ".contains(prev_ch) && LETTER_NUMBERS.contains(&unhighlight(chars[i+1])) ) {
                        //     result.push('1');        // need to retain grade 1 indicator (RUEB 6.5.2)
                        // }
                        // let the default case handle pushing on the chars for the letter
                        result.push(ch);
                        i += 1;
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
                        if duration == UEB_Duration::Symbol && !LETTER_PREFIXES.contains(&ch) {
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
                        // Want 'C' before 'L'; Could be CC for word cap -- if so, eat it and move on
                        // Note: guaranteed that there is a char after the 'C', so chars[i+1] is safe
                        if chars[i+1] == 'C' {
                            cap_word_mode = true;
                            i += 1;
                        } else {
                            let is_greek = chars[i+1] == 'G';
                            let (is_alone, right_matched_chars, n_letters) = stands_alone(&chars, if is_greek {i+2} else {i+1});
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
                                i += 1;
                            }
                            start_g2_letter = Some(i);
                            // debug!("  pushing 'C' + {:?}", right_matched_chars);
                            right_matched_chars.iter().for_each(|&ch| result.push(ch));
                            i += 1 + right_matched_chars.len();
                        }
                    },
                    '1' => {
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
                                result = handle_contractions(&chars[start..i], result);
                            }
                            cap_word_mode = false;
                            start_g2_letter = None;     // not start of char sequence
                        }
                        result.push(ch);
                        i += 1;
                        if !LEFT_INTERVENING_CHARS.contains(&ch) {
                            cap_word_mode = false;
                            i_g2_start = Some(i);
                        }

                    }
                }
                if mode != UEB_Mode::Grade2 && !cap_word_mode {
                    if let Some(start) = start_g2_letter {
                        result = handle_contractions(&chars[start..i], result);
                        start_g2_letter = None;     // not start of char sequence
                    }
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
    if mode == UEB_Mode::Grade2 {
        if let Some(start) = start_g2_letter {
            result = handle_contractions(&chars[start..i], result);
        }
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
    if !left_side_stands_alone(&chars[0..i]) {
        return (false, &chars[i..i+2], 0);
    }

    let (mut is_alone, n_letters, n_right_matched) = right_side_stands_alone(&chars[i+2..]);
    // debug!("left is alone, right is alone: {}, : n_letters={}, n_right_matched={}", is_alone, n_letters, n_right_matched);

    if is_alone && n_letters == 1 {
        let ch = chars[i+1];
        if ch=='⠁' || ch=='⠊' || ch=='⠕' {      // a, i, o
            is_alone = false;
        }
    }
    return (is_alone, &chars[i..i+2+n_right_matched], n_letters);

    /// chars before before 'L'
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
            } else if LEFT_INTERVENING_CHARS.contains(&ch) {
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
        static RIGHT_INTERVENING_CHARS: phf::Set<char> = phf_set! {
            'B', 'I', '𝔹', 'S', 'T', 'D', 'C', '𝐶', 's', 'w', 'e',   // indicators
            // ')', '}', ']', '\"', '\'', '”', '’', '»',      // closing chars
            // ',', ';', ':', '.', '…', '!', '?'              // punctuation           
        };
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
            } else if ch == 'c' || ch == 'b' {
                i += 1;       // ignore 'Lx' and also ignore 'ox'
            } else if RIGHT_INTERVENING_CHARS.contains(&ch) {  
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
fn handle_contractions(chars: &[char], mut result: String) -> String {
    struct Replacement {
        pattern: String,
        replacement: &'static str
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

    // It would be much better from an extensibility point of view to read the table in from a file
    lazy_static! {
        static ref CONTRACTIONS: Vec<Replacement> = vec![
            // 10.3: Strong contractions
            Replacement{ pattern: to_unicode_braille("and"), replacement: "L⠯"},
            Replacement{ pattern: to_unicode_braille("for"), replacement: "L⠿"},
            Replacement{ pattern: to_unicode_braille("of"), replacement: "L⠷"},
            Replacement{ pattern: to_unicode_braille("the"), replacement: "L⠮"},
            Replacement{ pattern: to_unicode_braille("with"), replacement: "L⠾"},
            
            // 10.8: final-letter group signs (this need to precede 'en' and any other shorter contraction)
            Replacement{ pattern: "(?P<s>L.)L⠍L⠑L⠝L⠞".to_string(), replacement: "${s}L⠰L⠞" }, // ment
            Replacement{ pattern: "(?P<s>L.)L⠞L⠊L⠕L⠝".to_string(), replacement: "${s}L⠰L⠝" } ,// tion

            // 10.4: Strong group signs
            Replacement{ pattern: to_unicode_braille("ch"), replacement: "L⠡"},
            Replacement{ pattern: to_unicode_braille("gh"), replacement: "L⠣"},
            Replacement{ pattern: to_unicode_braille("sh"), replacement: "L⠩"},
            Replacement{ pattern: to_unicode_braille("th"), replacement: "L⠹"},
            Replacement{ pattern: to_unicode_braille("wh"), replacement: "L⠱"},
            Replacement{ pattern: to_unicode_braille("ed"), replacement: "L⠫"},
            Replacement{ pattern: to_unicode_braille("er"), replacement: "L⠻"},
            Replacement{ pattern: to_unicode_braille("ou"), replacement: "L⠳"},
            Replacement{ pattern: to_unicode_braille("ow"), replacement: "L⠪"},
            Replacement{ pattern: to_unicode_braille("st"), replacement: "L⠌"},
            Replacement{ pattern: "(?P<s>L.)L⠊L⠝L⠛".to_string(), replacement: "${s}L⠬" },  // 'ing', not at start
            Replacement{ pattern: to_unicode_braille("ar"), replacement: "L⠜"},

            // 10.6.5: Lower group signs preceded and followed by letters
            // FIX: don't match if after/before a cap letter -- can't use negative pattern (?!...) in regex package
            // Note: removed cc because "arccos" shouldn't be contracted (10.11.1), but there is no way to know about compound words
            // Add it back after implementing a lookup dictionary of exceptions
            Replacement{ pattern: "(?P<s>L.)L⠑L⠁(?P<e>L.)".to_string(), replacement: "${s}L⠂${e}" },  // ea
            Replacement{ pattern: "(?P<s>L.)L⠃L⠃(?P<e>L.)".to_string(), replacement: "${s}L⠆${e}" },  // bb
            // Replacement{ pattern: "(?P<s>L.)L⠉L⠉(?P<e>L.)".to_string(), replacement: "${s}L⠒${e}" },  // cc
            Replacement{ pattern: "(?P<s>L.)L⠋L⠋(?P<e>L.)".to_string(), replacement: "${s}L⠖${e}" },  // ff
            Replacement{ pattern: "(?P<s>L.)L⠛L⠛(?P<e>L.)".to_string(), replacement: "${s}L⠶${e}" },  // gg

            // 10.6.8: Lower group signs ("in" also 10.5.4 lower word signs)
            // FIX: these need restrictions about only applying when upper dots are present
            Replacement{ pattern: to_unicode_braille("en"), replacement: "⠢"},
            Replacement{ pattern: to_unicode_braille("in"), replacement: "⠔"},
           
        ];

        static ref CONTRACTION_PATTERNS: RegexSet = init_patterns(&CONTRACTIONS);
        static ref CONTRACTION_REGEX: Vec<Regex> = init_regex(&CONTRACTIONS);
    }

    let mut chars_as_str = chars.iter().collect::<String>();
    // debug!("  handle_contractions: examine '{}'", &chars_as_str);
    let matches = CONTRACTION_PATTERNS.matches(&chars_as_str);
    for i in matches.iter() {
        let element = &CONTRACTIONS[i];
        // debug!("  replacing '{}' with '{}' in '{}'", element.pattern, element.replacement, &chars_as_str);
        result.truncate(result.len() - chars_as_str.len());
        chars_as_str = CONTRACTION_REGEX[i].replace_all(&chars_as_str, element.replacement).to_string();
        result.push_str(&chars_as_str);
        // debug!("  result after replace '{}'", result);
    }
    return result;



    fn init_patterns(contractions: &[Replacement]) -> RegexSet {
        let mut vec: Vec<&str> = Vec::with_capacity(contractions.len());
        for contraction in contractions {
            vec.push(&contraction.pattern);
        }
        return RegexSet::new(&vec).unwrap();
    }

    fn init_regex(contractions: &[Replacement]) -> Vec<Regex> {
        let mut vec = Vec::with_capacity(contractions.len());
        for contraction in contractions {
            vec.push(Regex::new(&contraction.pattern).unwrap());
        }
        return vec;
    }
}




static VIETNAM_INDICATOR_REPLACEMENTS: phf::Map<&str, &str> = phf_map! {
    "S" => "XXX",    // sans-serif -- from prefs
    "B" => "⠘",     // bold
    "𝔹" => "XXX",     // blackboard -- from prefs
    "T" => "⠈",     // script
    "I" => "⠨",     // italic
    "R" => "",      // roman
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
    lazy_static! {
        // Deal with Vietnamese "rhymes" -- moving accents around
        // See "Vietnamese Uncontracted Braille Update in MathCAT" or maybe https://icanreadvietnamese.com/blog/14-rule-of-tone-mark-placement
        // Note: I don't know how to write (for example) I_E_RULE so that it excludes "qu" and "gi", so I use two rules
        // The first rule rewrites the patterns with "qu" and "gi" to add "!" to prevent a match of the second rule -- "!" is dropped later
        static ref QU_GI_RULE_EXCEPTION: Regex = Regex::new(r"(L⠟L⠥|L⠛L⠊)").unwrap();
        static ref IUOY_E_RULE: Regex = Regex::new(r"L(⠊|⠥|⠕|⠽)(L[⠔⠰⠢⠤⠠])L(⠑|⠣)").unwrap();     // ie, ue, oe, and ye rule
        static ref UO_A_RULE: Regex = Regex::new(r"L(⠥|⠕)(L[⠔⠰⠢⠤⠠])L(⠁|⠡|⠜)").unwrap();     // ua, oa rule
        static ref UU_O_RULE: Regex = Regex::new(r"L(⠥|⠳)(L[⠔⠰⠢⠤⠠])L(⠪|⠹)").unwrap();     // uo, ưo rule
        static ref UYE_RULE: Regex = Regex::new(r"L⠥L([⠔⠰⠢⠤⠠])L⠽L⠣").unwrap();     // uo, ưo rule
        static ref UY_RULE: Regex = Regex::new(r"L⠥L([⠔⠰⠢⠤⠠])L⠽").unwrap();     // uo, ưo rule
        static ref REPLACE_INDICATORS: Regex =Regex::new(r"([1𝟙SB𝔹TIREDGVHP𝐶𝑐CLMNW𝐖swe,.-—―#ocb!])").unwrap();

    }
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
    let double_struck = pref_manager.pref_to_string("Vietnam_DoubleStruck");
    let sans_serif = pref_manager.pref_to_string("Vietnam_SansSerif");
    let fraktur = pref_manager.pref_to_string("Vietnam_Fraktur");
    let greek_variant = pref_manager.pref_to_string("Vietnam_GreekVariant");

    // This reuses the code just for getting rid of unnecessary "L"s and "N"s
    let result = remove_unneeded_mode_changes(&result, UEB_Mode::Grade1, UEB_Duration::Passage);


    let result = REPLACE_INDICATORS.replace_all(&result, |cap: &Captures| {
        let matched_char = &cap[0];
        match matched_char {
            "𝔹" => &double_struck,
            "S" => &sans_serif,
            "D" => &fraktur,
            "V" => &greek_variant,
            _ => match VIETNAM_INDICATOR_REPLACEMENTS.get(matched_char) {
                None => {error!("REPLACE_INDICATORS and VIETNAM_INDICATOR_REPLACEMENTS are not in sync: missing '{matched_char}'"); ""},
                Some(&ch) => ch,
            },
        }
    });

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
    "I" => "⠔",     // italic -- same as bold
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
    lazy_static! {
        static ref ADD_WHITE_SPACE: Regex = Regex::new(r"𝘄(.)|𝘄$").unwrap();
    }

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
    "I" => "⠨",     // italic
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
    "I" => "⠨",     // italic
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
    lazy_static! {
        static ref REPLACE_INDICATORS: Regex =Regex::new(r"([SB𝔹TIREDGVHUP𝐏C𝐶LlMmb↑↓Nn𝑁WwZ,()])").unwrap();
        // Numbers need to end with a space, but sometimes there is one there for other reasons
        static ref DROP_NUMBER_SEPARATOR: Regex = Regex::new(r"(n.)\)").unwrap();
        static ref NUMBER_MATCH: Regex = Regex::new(r"((N.)+[^WN𝐶#↑↓Z])").unwrap();
    }

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

    // these typeforms need to get pulled from user-prefs as they are transcriber-defined
    let double_struck = pref_manager.pref_to_string("Vietnam_DoubleStruck");
    let sans_serif = pref_manager.pref_to_string("Vietnam_SansSerif");
    let fraktur = pref_manager.pref_to_string("Vietnam_Fraktur");
    let greek_variant = pref_manager.pref_to_string("Vietnam_GreekVariant");

    // This reuses the code just for getting rid of unnecessary "L"s and "N"s
    let result = remove_unneeded_mode_changes(&result, UEB_Mode::Grade1, UEB_Duration::Passage);
    // debug!("   remove_unneeded_mode_changes={}", &result);


    let result = REPLACE_INDICATORS.replace_all(&result, |cap: &Captures| {
        let matched_char = &cap[0];
        match matched_char {
            "𝔹" => &double_struck,
            "S" => &sans_serif,
            "D" => &fraktur,
            "V" => &greek_variant,
            _ => match FINNISH_INDICATOR_REPLACEMENTS.get(matched_char) {
                None => {error!("REPLACE_INDICATORS and SWEDISH_INDICATOR_REPLACEMENTS are not in sync: missing '{matched_char}'"); ""},
                Some(&ch) => ch,
            },
        }
    });

    // Remove unicode blanks at start and end -- do this after the substitutions because ',' introduces spaces
    // let result = result.trim_start_matches('⠀').trim_end_matches('⠀');
    let result = COLLAPSE_SPACES.replace_all(&result, "⠀");
   
    return result.to_string();
}


fn swedish_cleanup(pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String {
    // FIX: need to implement this -- this is just a copy of the Vietnam code
    lazy_static! {
        // Empty bases are ok if they follow whitespace
        static ref EMPTY_BASE: Regex = Regex::new(r"(^|[W𝐖w])E").unwrap();
    }
    // debug!("swedish_cleanup: start={}", raw_braille);
    let result = typeface_to_word_mode(&raw_braille);
    let result = capitals_to_word_mode(&result);

    let result = result.replace("CG", "⠘")
                                    .replace("𝔹C", "⠩")
                                    .replace("DC", "⠰");

    // debug!("   after typeface/caps={}", &result);

    // these typeforms need to get pulled from user-prefs as they are transcriber-defined
    let double_struck = pref_manager.pref_to_string("Vietnam_DoubleStruck");
    let sans_serif = pref_manager.pref_to_string("Vietnam_SansSerif");
    let fraktur = pref_manager.pref_to_string("Vietnam_Fraktur");
    let greek_variant = pref_manager.pref_to_string("Vietnam_GreekVariant");

    // This reuses the code just for getting rid of unnecessary "L"s and "N"s
    let result = remove_unneeded_mode_changes(&result, UEB_Mode::Grade1, UEB_Duration::Passage);
    // debug!("   after removing mode changes={}", &result);


    let result = EMPTY_BASE.replace_all(&result, "$1");
    let result = REPLACE_INDICATORS.replace_all(&result, |cap: &Captures| {
        let matched_char = &cap[0];
        match matched_char {
            "𝔹" => &double_struck,
            "S" => &sans_serif,
            "D" => &fraktur,
            "V" => &greek_variant,
            _ => match SWEDISH_INDICATOR_REPLACEMENTS.get(matched_char) {
                None => {error!("REPLACE_INDICATORS and SWEDISH_INDICATOR_REPLACEMENTS are not in sync: missing '{matched_char}'"); ""},
                Some(&ch) => ch,
            },
        }
    });

    // Remove unicode blanks at start and end -- do this after the substitutions because ',' introduces spaces
    // let result = result.trim_start_matches('⠀').trim_end_matches('⠀');
    let result = COLLAPSE_SPACES.replace_all(&result, "⠀");
   
    return result.to_string();
}

#[allow(non_snake_case)]
fn LaTeX_cleanup(_pref_manager: Ref<PreferenceManager>, raw_braille: String) -> String {
    lazy_static! {
        static ref REMOVE_SPACE: Regex =Regex::new(r" ([\^_,;)\]}])").unwrap();          // '^', '_', ',', ';', ')', ']', '}'
        static ref COLLAPSE_SPACES: Regex = Regex::new(r" +").unwrap();
    }
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
    lazy_static! {
        static ref REMOVE_SPACE_BEFORE_OP: Regex = Regex::new(r#"([\w\d]) +([^\w\d"]|[\^_,;)\]}])"#).unwrap();
        static ref REMOVE_SPACE_AFTER_OP: Regex =  Regex::new(r#"([^\^_,;)\]}\w\d"]) +([\w\d])"#).unwrap();
        static ref COLLAPSE_SPACES: Regex = Regex::new(r" +").unwrap();
    }
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
use crate::canonicalize::{name, as_element, as_text};
use crate::xpath_functions::{is_leaf, IsBracketed, validate_one_node};
use sxd_document::dom::ParentOfChild;
use sxd_xpath::{Value, context, nodeset::*};
use sxd_xpath::function::{Function, Args};
use sxd_xpath::function::Error as XPathError;
use std::result::Result as StdResult;

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
        } else if FIRST_CHILD_ONLY.contains(&name) {
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
                    return Err( sxd_xpath::function::Error::Other("Internal error in nemeth_root_value: didn't find 'math' tag".to_string()) );
                }
            }
        }
        return Err( XPathError::Other("Internal error in nemeth_root_value: didn't find 'math' tag".to_string()) );
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
                panic!("NestingChars chars should be used only on 'mfrac'. '{}' was passed in", name);
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
        let result = match code {
            "Nemeth" => BrailleChars::get_braille_nemeth_chars(node, text_range),
            "UEB" => BrailleChars:: get_braille_ueb_chars(node, text_range),
            "CMU" => BrailleChars:: get_braille_cmu_chars(node, text_range),
            "Vietnam" => BrailleChars:: get_braille_vietnam_chars(node, text_range),
            "Swedish" => BrailleChars:: get_braille_ueb_chars(node, text_range),    // FIX: need to figure out what to implement
            "Finnish" => BrailleChars:: get_braille_ueb_chars(node, text_range),    // FIX: need to figure out what to implement
            _ => return Err(sxd_xpath::function::Error::Other(format!("get_braille_chars: unknown braille code '{code}'")))
        };
        return match result {
            Ok(string) => Ok(make_quoted_string(string)),
            Err(err) => return Err(sxd_xpath::function::Error::Other(err.to_string())),
        }
    }

    fn get_braille_nemeth_chars(node: Element, text_range: Option<Range<usize>>) -> Result<String> {
        lazy_static! {
            // To greatly simplify typeface/language generation, the chars have unique ASCII chars for them:
            // Typeface: S: sans-serif, B: bold, 𝔹: blackboard, T: script, I: italic, R: Roman
            // Language: E: English, D: German, G: Greek, V: Greek variants, H: Hebrew, U: Russian
            // Indicators: C: capital, L: letter, N: number, P: punctuation, M: multipurpose
            static ref PICK_APART_CHAR: Regex = 
                Regex::new(r"(?P<face>[SB𝔹TIR]*)(?P<lang>[EDGVHU]?)(?P<cap>C?)(?P<letter>L?)(?P<num>[N]?)(?P<char>.)").unwrap();
        }
        let math_variant = node.attribute_value("mathvariant");
        // FIX: cover all the options -- use phf::Map
        let  attr_typeface = match math_variant {
            None => "R",
            Some(variant) => match variant {
                "bold" => "B",
                "italic" => "I",
                "double-struck" => "𝔹",
                "script" => "T",
                "fraktur" => "D",
                "sans-serif" => "S",
                _ => "R",       // normal and unknown
            },
        };
        let text = BrailleChars::substring(as_text(node), &text_range);
        let braille_chars = braille_replace_chars(&text, node)?;
        // debug!("Nemeth chars: text='{}', braille_chars='{}'", &text, &braille_chars);
        
        // we want to pull the prefix (typeface, language) out to the front until a change happens
        // the same is true for number indicator
        // also true (sort of) for capitalization -- if all caps, use double cap in front (assume abbr or Roman Numeral)
        
        // we only care about this for numbers and identifiers/text, so we filter for only those
        let node_name = name(node);
        let is_in_enclosed_list = node_name != "mo" && BrailleChars::is_in_enclosed_list(node);
        let is_mn_in_enclosed_list = is_in_enclosed_list && node_name == "mn";
        let mut typeface = "R".to_string();     // assumption is "R" and if attr or letter is different, something happens
        let mut is_all_caps = true;
        let mut is_all_caps_valid = false;      // all_caps only valid if we did a replacement
        let result = PICK_APART_CHAR.replace_all(&braille_chars, |caps: &Captures| {
            // debug!("  face: {:?}, lang: {:?}, num {:?}, letter: {:?}, cap: {:?}, char: {:?}",
            //        &caps["face"], &caps["lang"], &caps["num"], &caps["letter"], &caps["cap"], &caps["char"]);
            let mut nemeth_chars = "".to_string();
            let char_face = if caps["face"].is_empty() {attr_typeface} else {&caps["face"]};
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
        lazy_static! {
            static ref HAS_TYPEFACE: Regex = Regex::new(".*?(double-struck|script|fraktur|sans-serif).*").unwrap();
            static ref PICK_APART_CHAR: Regex = 
                 Regex::new(r"(?P<bold>B??)(?P<italic>I??)(?P<face>[S𝔹TD]??)s??(?P<cap>C??)(?P<greek>G??)(?P<char>[NL].)").unwrap();
        }
    
        let math_variant = node.attribute_value("mathvariant");
        let text = BrailleChars::substring(as_text(node), &text_range);
        let mut braille_chars = braille_replace_chars(&text, node)?;

        // debug!("get_braille_ueb_chars: before/after unicode.yaml: '{}'/'{}'", text, braille_chars);
        if math_variant.is_none() {         // nothing we need to do
            return Ok(braille_chars);
        }
        // mathvariant could be "sans-serif-bold-italic" -- get the parts
        let math_variant = math_variant.unwrap();
        let italic = math_variant.contains("italic");
        if italic & !braille_chars.contains('I') {
            braille_chars = "I".to_string() + &braille_chars;
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

    fn get_braille_cmu_chars(node: Element, text_range: Option<Range<usize>>) -> Result<String> {
        // In CMU, we need to replace spaces used for number blocks with "."
        // For other numbers, we need to add "." to create digit blocks

        lazy_static! {
            static ref HAS_TYPEFACE: Regex = Regex::new(".*?(double-struck|script|fraktur|sans-serif).*").unwrap();
            static ref PICK_APART_CHAR: Regex = 
                 Regex::new(r"(?P<bold>B??)(?P<italic>I??)(?P<face>[S𝔹TD]??)s??(?P<cap>C??)(?P<greek>G??)(?P<char>[NL].)").unwrap();
        }
    
        let math_variant = node.attribute_value("mathvariant");
        let text = BrailleChars::substring(as_text(node), &text_range);
        let text = add_separator(text);

        let braille_chars = braille_replace_chars(&text, node)?;

        // debug!("get_braille_ueb_chars: before/after unicode.yaml: '{}'/'{}'", text, braille_chars);
        if math_variant.is_none() {         // nothing we need to do
            return Ok(braille_chars);
        }
        // mathvariant could be "sans-serif-bold-italic" -- get the parts
        let math_variant = math_variant.unwrap();
        let bold = math_variant.contains("bold");
        let italic = math_variant.contains("italic");
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
                        // switch dot/comma -- using "\x01" as a temp when switching the the two chars
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
            let name = name(node);
            return match name {
                "mi" | "mn" => true,
                "mo"  => !crate::canonicalize::is_relational_op(node),
                "mtext" => {
                    let text = as_text(node).trim();
                    return text=="?" || text=="-?-" || text.is_empty();   // various forms of "fill in missing content" (see also Nemeth_RULEs.yaml, "omissions")
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
                    if let Some(notation) = node.attribute_value("notation") {
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
            return Err( XPathError::Other(format!("BrailleChars requires 2 or 4 args: {e}")));
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
            return Err( XPathError::Other(format!("BrailleChars called on non-leaf element '{}'", mml_to_string(node))) );
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
        let decimal_separator = crate::interface::get_preference("DecimalSeparators".to_string()).unwrap()
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
        let mut node_name = name(mathml);
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
                let text = as_text(mathml);
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

    // ordinals often have an irregular start (e.g., "half") before becoming regular.
    // if the number is irregular, return the ordinal form, otherwise return 'None'.
    fn needs_grouping_for_swedish(mathml: Element, is_base: bool) -> bool {
        use crate::xpath_functions::IsInDefinition;
        let mut node_name = name(mathml);
        if mathml.attribute_value("data-roman-numeral").is_some() {
            node_name = "mi";           // roman numerals don't follow number rules
        }

        match node_name {
            "mn" => return false,
            "mi" | "mo" | "mtext" => {
                let text = as_text(mathml);
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
        let mut node_name = name(mathml);
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
                let text = as_text(mathml);
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
            let answer = match braille_code.as_str() {
                "CMU" => NeedsToBeGrouped::needs_grouping_for_cmu(e, is_base),
                "UEB" => NeedsToBeGrouped::needs_grouping_for_ueb(e, is_base),
                "Finnish" => NeedsToBeGrouped::needs_grouping_for_finnish(e, is_base),
                "Swedish" => NeedsToBeGrouped::needs_grouping_for_swedish(e, is_base),
                _ => return Err(XPathError::Other(format!("NeedsToBeGrouped: braille code arg '{braille_code:?}' is not a known code ('UEB', 'CMU', or 'Swedish')"))),
            };
            return Ok( Value::Boolean( answer ) );
        }

        return Err(XPathError::Other(format!("NeedsToBeGrouped: first arg '{node:?}' is not a node")));
    }
}
    
    
    
#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::init_logger;
    use crate::interface::*;
    
    #[test]
    fn ueb_highlight_24() -> Result<()> {       // issue 24
        let mathml_str = "<math display='block' id='id-0'>
            <mrow id='id-1'>
                <mn id='id-2'>4</mn>
                <mo id='id-3'>&#x2062;</mo>
                <mi id='id-4'>a</mi>
                <mo id='id-5'>&#x2062;</mo>
                <mi id='id-6'>c</mi>
            </mrow>
        </math>";
        crate::interface::set_rules_dir(super::super::abs_rules_dir_path()).unwrap();
        set_mathml(mathml_str.to_string()).unwrap();
        set_preference("BrailleCode".to_string(), "UEB".to_string()).unwrap();
        set_preference("BrailleNavHighlight".to_string(), "All".to_string()).unwrap();
        let braille = get_braille("id-2".to_string())?;
        assert_eq!("⣼⣙⠰⠁⠉", braille);
        set_navigation_node("id-2".to_string(), 0)?;
        assert_eq!( get_braille_position()?, (0,2));

        let braille = get_braille("id-4".to_string())?;
        assert_eq!("⠼⠙⣰⣁⠉", braille);
        set_navigation_node("id-4".to_string(), 0)?;
        assert_eq!( get_braille_position()?, (2,4));
        return Ok( () );
    }
    
    #[test]
    // This test probably should be repeated for each braille code and be taken out of here
    fn find_mathml_from_braille() -> Result<()> { 
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
        crate::interface::set_rules_dir(super::super::abs_rules_dir_path()).unwrap();
        set_mathml(mathml_str.to_string()).unwrap();
        set_preference("BrailleNavHighlight".to_string(), "Off".to_string()).unwrap();
        
        set_preference("BrailleCode".to_string(), "Nemeth".to_string()).unwrap();
        let braille = get_braille("".to_string())?;
        let answers= &[2, 3, 3, 3, 3, 4, 7, 8, 9, 9,   10, 13, 12, 14, 12, 15, 17, 19, 21, 10,   4, 23, 25, 4];
        let answers = answers.map(|num| format!("id-{}", num));
        debug!("\n*** Testing Nemeth ***");
        for i in 0..braille.chars().count() {
            debug!("\n===  i={}  ===", i);
            let instant = Instant::now();
            let (id, _offset) = crate::interface::get_navigation_node_from_braille_position(i)?;
            N_PROBES.with(|n| {debug!("test {:2} #probes = {}", i, n.borrow())});
            debug!("Time taken: {}ms", instant.elapsed().as_millis());
            assert_eq!(answers[i], id, "\nNemeth test ith position={}", i);
        }

        set_preference("BrailleCode".to_string(), "UEB".to_string()).unwrap();
        let braille = get_braille("".to_string())?;
        let answers= &[0, 0, 0, 2, 3, 3, 3, 3, 4, 7,   7, 8, 9, 9, 10, 13, 12, 14, 14, 15,   15, 17, 17, 19, 19, 21, 10, 4, 4, 23,   23, 25, 25, 4, 0, 0];
        let answers = answers.map(|num| format!("id-{}", num));
        debug!("\n\n*** Testing UEB ***");
        for i in 0..braille.chars().count() {
            debug!("\n===  i={}  ===", i);
            let instant = Instant::now();
            let (id, _offset) = crate::interface::get_navigation_node_from_braille_position(i)?;
            N_PROBES.with(|n| {debug!("test {:2} #probes = {}", i, n.borrow())});
            debug!("Time taken: {}ms", instant.elapsed().as_millis());
            assert_eq!(answers[i], id, "\nUEB test ith position={}", i);
        }
        set_preference("BrailleCode".to_string(), "CMU".to_string()).unwrap();
        let braille = get_braille("".to_string())?;
        let answers= &[2, 3, 5, 7, 8, 9, 9, 9, 10, 10,   11, 13, 12, 14, 14, 15, 17, 17, 19, 19,   21, 11, 5, 4, 22, 23, 23, 25, 25, 22,];
        let answers = answers.map(|num| format!("id-{}", num));
        debug!("\n\n*** Testing CMU ***");
        debug!("Braille: {}", braille);
        for i in 0..braille.chars().count() {
            debug!("\n===  i={}  ===", i);
            let instant = Instant::now();
            let (id, _offset) = crate::interface::get_navigation_node_from_braille_position(i)?;
            N_PROBES.with(|n| {debug!("test {:2} #probes = {}", i, n.borrow())});
            debug!("Time taken: {}ms", instant.elapsed().as_millis());
            assert_eq!(answers[i], id, "\nCMU test ith position={}", i);
        }
        return Ok( () );
    }
    
    #[test]
    #[allow(non_snake_case)]
    fn test_UEB_start_mode() -> Result<()> {
        let mathml_str = "<math><msup><mi>x</mi><mi>n</mi></msup></math>";
        crate::interface::set_rules_dir(super::super::abs_rules_dir_path()).unwrap();
        set_mathml(mathml_str.to_string()).unwrap();
        set_preference("BrailleCode".to_string(), "UEB".to_string()).unwrap();
        set_preference("UEB_START_MODE".to_string(), "Grade2".to_string()).unwrap();
        let braille = get_braille("".to_string())?;
        assert_eq!("⠭⠰⠔⠝", braille, "Grade2");
        set_preference("UEB_START_MODE".to_string(), "Grade1".to_string()).unwrap();
        let braille = get_braille("".to_string())?;
        assert_eq!("⠭⠔⠝", braille, "Grade1");
        return Ok( () );
    }
}
