//! The interface module provides functionality both for calling from an API and also running the code from `main`.
//!
#![allow(non_snake_case)]
#![allow(clippy::needless_return)]
use std::cell::RefCell;

use sxd_document::parser;
use sxd_document::Package;
use sxd_document::dom::*;
use crate::errors::*;
use regex::Regex;


use crate::navigate::*;
use crate::pretty_print::mml_to_string;
use crate::xpath_functions::is_leaf;

// wrap up some common functionality between the call from 'main' and AT
fn cleanup_mathml(mathml: Element) -> Result<Element> {
    trim_element(&mathml);
    let mathml = crate::canonicalize::canonicalize(mathml)?;
    let mathml = add_ids(mathml);
    return Ok(mathml);
}


thread_local!{
    /// The current node being navigated (also spoken and brailled) is stored in `MATHML_INSTANCE`.
    pub static MATHML_INSTANCE: RefCell<Package> = init_mathml_instance();
}

fn init_mathml_instance() -> RefCell<Package> {
    let package = parser::parse("<math></math>")
        .expect("Internal error in 'init_mathml_instance;: didn't parse initializer string");
    return RefCell::new( package );
}

/// Set the Rules directory
/// IMPORTANT: this should be the very first call to MathCAT unless the environment var MathCATRulesDir is set
pub fn set_rules_dir(dir: String) -> Result<()> {
    use std::path::PathBuf;
    let pref_manager = crate::prefs::PreferenceManager::get();
    return pref_manager.borrow_mut().initialize(PathBuf::from(dir));
}

/// Returns the version number (from Cargo.toml) of the build
pub fn get_version() -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    return VERSION.to_string();
}


/// This will override any previous MathML that was set.
/// This returns canonical MathML with 'id's set on any node that doesn't have an id.
/// The ids can be used for sync highlighting if the `Bookmark` API preference is true.
pub fn set_mathml(mathml_str: String) -> Result<String> {
    lazy_static! {
        // if these are present when resent to MathJaX, MathJaX crashes (https://github.com/mathjax/MathJax/issues/2822)
        static ref MATHJAX_V2: Regex = Regex::new(r#"class *= *['"]MJX-.*?['"]"#).unwrap();
        static ref MATHJAX_V3: Regex = Regex::new(r#"class *= *['"]data-mjx-.*?['"]"#).unwrap();
    }

    NAVIGATION_STATE.with(|nav_stack| {
        nav_stack.borrow_mut().reset();
    });
    return MATHML_INSTANCE.with(|old_package| {
        // need to deal with character data and convert to something the parser knows
        // potentially all of the names HTML knows could be here, but these are hopefully the important ones
        let mathml_str = mathml_str.replace("&lt;", "&#x3c;")
                                          .replace("&gt;", "&#x3e;")
                                          .replace("&amp;", "&#x26;")
                                          .replace("&nbsp;", "&#xa0;");

        let mathml_str = MATHJAX_V2.replace_all(&mathml_str, "");
        let mathml_str = MATHJAX_V3.replace_all(&mathml_str, "");
        let new_package = parser::parse(&mathml_str);    
        if let Err(e) = new_package {
            bail!("Invalid MathML input:\n{}\nError is: {}", &mathml_str, &e.to_string());
        }
        crate::speech::SpeechRules::initialize_all_rules()?;

        let new_package = new_package.unwrap();
        let mathml = cleanup_mathml(get_element(&new_package))?;
        let mathml_string = mml_to_string(&mathml);
        old_package.replace(new_package);

        return Ok( mathml_string );
    })
}

/// Get the spoken text of the MathML that was set.
/// The speech takes into account any AT or user preferences.
pub fn get_spoken_text() -> Result<String> {
    // use std::time::{Instant};
    // let instant = Instant::now();
    return MATHML_INSTANCE.with(|package_instance| {
        let package_instance = package_instance.borrow();
        let mathml = get_element(&*package_instance);
        let new_package = Package::new();
        let intent = crate::speech::intent_from_mathml(mathml, new_package.as_document())?;
        debug!("Intent tree:\n{}", mml_to_string(&intent));
        let speech = crate::speech::speak_intent(intent)?;
        // info!("Time taken: {}ms", instant.elapsed().as_millis());
        return Ok( speech );
    });
}

/// Get the spoken text for an overview of the MathML that was set.
/// The speech takes into account any AT or user preferences.
/// Note: this implementation for is currently minimal and should not be used.
pub fn get_overview_text() -> Result<String> {
    // use std::time::{Instant};
    // let instant = Instant::now();
    return MATHML_INSTANCE.with(|package_instance| {
        let package_instance = package_instance.borrow();
        let mathml = get_element(&*package_instance);
        let speech = crate::speech::overview_mathml(mathml)?;
        // info!("Time taken: {}ms", instant.elapsed().as_millis());
        return Ok( speech );
    });
}

/// Get the value of the named preference.
/// None is returned if `name` is not a known preference.
pub fn get_preference(name: String) -> Option<String> {
    use yaml_rust::Yaml;
    return crate::speech::SPEECH_RULES.with(|rules| {
        let rules = rules.borrow();
        let pref_manager = rules.pref_manager.borrow();
        let prefs = pref_manager.merge_prefs();
        return match prefs.get(&name) {
            None => None,
            Some(yaml) => match yaml {
                Yaml::String(s) => Some(s.clone()),
                Yaml::Boolean(b)  => Some( (if *b {"true"} else {"false"}).to_string() ),
                Yaml::Integer(i)   => Some( format!("{}", *i)),
                Yaml::Real(s)   => Some(s.clone()),
                _                      => None,            
            },
        }
    });
}

/// Set a MathCAT preference. The preference name should be a known preference name.
/// The value should either be a string or a number (depending upon the preference being set)
/// The list of known user preferences is in the MathCAT user documentation.
/// Here are common preferences set by programs (not settable by the user):
/// * TTS -- SSML, SAPI5, None
/// * Pitch -- normalized at '1.0'
/// * Rate -- words per minute (should match current speech rate).
///       There is a separate "MathRate" that is user settable that causes a relative percentage change from this rate.
/// * Volume -- default 100
/// * Voice -- set a voice to use (not implemented)
/// * Gender -- set pick any voice of the given gender (not implemented)
/// * Bookmark -- set to `true` if a `mark`/`bookmark` should be part of the returned speech (used for sync highlighting)
///
/// Important: both the preference name and value are case-sensitive
/// 
/// This function can be called multiple times to set different values.
/// The values are persistent and extend beyond calls to [`set_mathml`].
/// A value can be overwritten by calling this function again with a different value.
/// 
/// FIX: Some preferences are both API and user preferences and something such as '!name' should be used for overrides. Not implemented yet.
pub fn set_preference(name: String, value: String) -> Result<()> {
    return crate::speech::SPEECH_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        if let Some(error_string) = rules.get_error() {
            bail!("{}", error_string);
        }
        // note: Rust complains if I set
        //    pref_manager = rules.pref_manager.borrow_mut()
        // here/upfront, so it is borrowed separately below. That way its borrowed lifetime is small
        let files_changed;
        {
            let mut pref_manager = rules.pref_manager.borrow_mut();
            if !pref_manager.get_api_prefs().to_string(&name).is_empty() {
                match name.as_str() {
                    "Pitch" | "Rate" | "Volume" => {
                        pref_manager.set_api_float_pref(&name, to_float(&name, &value)?);    
                    },
                    "Bookmark" => {
                        pref_manager.set_api_boolean_pref(&name, value.to_lowercase()=="true");    
                    },
                    _ => {
                        pref_manager.set_api_string_pref(&name, &value);
                    }
                }
                files_changed = None;
            } else if pref_manager.get_user_prefs().to_string(name.as_str()).is_empty() {
                bail!("set_preference: {} is not a known preference", &name); 
            } else {
                files_changed = pref_manager.set_user_prefs(&name, &value);     // assume string valued
            }
            pref_manager.merge_prefs();
        }

        match name.as_str() {
            "SpeechStyle" => {
                if let Some(files_changed) = files_changed {
                    rules.invalidate(files_changed);
                }
            },
            "Language" => {
                // check the format
                if !( value.len() == 2 ||
                      (value.len() == 5 && value.as_bytes()[2] == b'-') ) {
                        bail!("Improper format for 'Language' preference '{}'. Should be of form 'en' or 'en-gb'", value);
                      }
                if let Some(files_changed) = files_changed {
                    rules.invalidate(files_changed);
                }
            },
            "BrailleCode" => {
                crate::speech::BRAILLE_RULES.with(|braille_rules| {
                    if let Some(files_changed) = files_changed {
                        braille_rules.borrow_mut().invalidate(files_changed);
                    }
                })
            },
            _ => {
            }
        }
        return Ok( () );
    });

    fn to_float(name: &str, value: &str) -> Result<f64> {
        match value.parse::<f64>() {
            Ok(val) => return Ok(val),
            Err(_) => bail!("SetPreference: preference'{}'s value '{}' must be a float", name, value),
        };
    }
}

/// Get the braille associated with the MathML that was set by [`set_mathml`].
/// The braille returned depends upon the preference for the `code` preference (default `Nemeth`).
pub fn get_braille(nav_node_id: String) -> Result<String> {
    // use std::time::{Instant};
    // let instant = Instant::now();
    return MATHML_INSTANCE.with(|package_instance| {
        let package_instance = package_instance.borrow();
        let mathml = get_element(&*package_instance);
        let braille = crate::braille::braille_mathml(mathml, nav_node_id)?;
        // info!("Time taken: {}ms", instant.elapsed().as_millis());
        return Ok( braille );
    });
}

/// Given a key code along with the modifier keys, the current node is moved accordingly (or value reported in some cases).
/// `key` is the [keycode](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/keyCode#constants_for_keycode_value) for the key (in JavaScript, `ev.key_code`)
/// The spoken text for the new current node is returned.
pub fn do_navigate_keypress(key: usize, shift_key: bool, control_key: bool, alt_key: bool, meta_key: bool) -> Result<String> {
    return MATHML_INSTANCE.with(|package_instance| {
        let package_instance = package_instance.borrow();
        let mathml = get_element(&*package_instance);
        return do_mathml_navigate_key_press(mathml, key, shift_key, control_key, alt_key, meta_key);
    });
}

/// Given a navigation command, the current node is moved accordingly.
/// This is a higher level interface than `do_navigate_keypress` for applications that want to interpret the keys themselves.
/// The valid commands are:
/// * Standard move commands:
/// `MovePrevious`, `MoveNext`, `MoveStart`, `MoveEnd`, `MoveLineStart`, `MoveLineEnd`
/// * Movement in a table or elementary math:
/// `MoveCellPrevious`, `MoveCellNext`, `MoveCellUp`, `MoveCellDown`, `MoveColumnStart`, `MoveColumnEnd`
/// * Moving into children or out to parents:
/// `ZoomIn`, `ZoomOut`, `ZoomOutAll`, `ZoomInAll`
/// * Undo the last movement command:
/// `MoveLastLocation`
/// * Read commands (standard speech):
/// `ReadPrevious`, `ReadNext`, `ReadCurrent`, `ReadCellCurrent`, `ReadStart`, `ReadEnd`, `ReadLineStart`, `ReadLineEnd`
/// * Describe commands (overview):
/// `DescribePrevious`, `DescribeNext`, `DescribeCurrent`
/// * Location information:
/// `WhereAmI`, `WhereAmIAll`
/// * Change navigation modes (circle up/down):
///  `ToggleZoomLockUp`, `ToggleZoomLockDown`
/// * Speak the current navigation mode
/// `ToggleSpeakMode`
/// 
/// There are 10 place markers that can be set/read/described or moved to.
/// * Setting:
/// `SetPlacemarker0`, `SetPlacemarker1`, `SetPlacemarker2`, `SetPlacemarker3`, `SetPlacemarker4`, `SetPlacemarker5`, `SetPlacemarker6`, `SetPlacemarker7`, `SetPlacemarker8`, `SetPlacemarker9`
/// * Reading:
/// `Read0`, `Read1`, `Read2`, `Read3`, `Read4`, `Read5`, `Read6`, `Read7`, `Read8`, `Read9`
/// * Describing:
/// `Describe0`, `Describe1`, `Describe2`, `Describe3`, `Describe4`, `Describe5`, `Describe6`, `Describe7`, `Describe8`, `Describe9`
/// * Moving:
/// `MoveTo0`, `MoveTo1`, `MoveTo2`, `MoveTo3`, `MoveTo4`, `MoveTo5`, `MoveTo6`, `MoveTo7`, `MoveTo8`, `MoveTo9`
/// 
/// When done with Navigation, call with `Exit`
pub fn do_navigate_command(command: String) -> Result<String> {
    let command = NAV_COMMANDS.get_key(&command);       // gets a &'static version of the command
    if command.is_none() {
        bail!("Unknown command in call to DoNavigateCommand()");
    };
    let command = *command.unwrap();
    return MATHML_INSTANCE.with(|package_instance| {
        let package_instance = package_instance.borrow();
        let mathml = get_element(&*package_instance);
        return do_navigate_command_string(mathml, command);
    });
}

/// Return the MathML associated with the current (navigation) node.
/// The returned result is the `id` of the node and the offset (0-based) from that node (not yet implemented)
/// The offset is needed for token elements that have multiple characters.
pub fn get_navigation_mathml() -> Result<(String, usize)> {
    return MATHML_INSTANCE.with(|package_instance| {
        let package_instance = package_instance.borrow();
        let mathml = get_element(&*package_instance);
        return NAVIGATION_STATE.with(|nav_stack| {
            return match nav_stack.borrow_mut().get_navigation_mathml(mathml) {
                Err(e) => Err(e),
                Ok( (found, offset) ) => Ok( (mml_to_string(&found), offset) ),
            }
        } )
    });
}

/// Return the `id` and `offset` (0-based) associated with the current (navigation) node.
/// `offset` (not yet implemented)
/// The offset is needed for token elements that have multiple characters.
pub fn get_navigation_mathml_id() -> Result<(String, usize)> {
    return MATHML_INSTANCE.with(|package_instance| {
        let package_instance = package_instance.borrow();
        let mathml = get_element(&*package_instance);
        return Ok( NAVIGATION_STATE.with(|nav_stack| {
            return nav_stack.borrow().get_navigation_mathml_id(mathml);
        }) )
    });
}


/// Convert the returned error from set_mathml, etc., to a useful string for display
pub fn errors_to_string(e:&Error) -> String {
    let mut result = String::default();
    let mut first_time = true;
    for e in e.iter() {
        if first_time {
            result = format!("{}\n", e);
            first_time = false;
        } else {
            result += &format!("caused by: {}\n", e);
        }
    }
    return result;
}


fn add_ids<'a>(mathml: Element<'a>) -> Element<'a> {
    use std::time::SystemTime;
    let time = if cfg!(target_family = "wasm") {
        rand::random::<usize>()
    } else {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as usize
    };
    let time_part = radix_fmt::radix(time, 36).to_string();
    let random_part = radix_fmt::radix(rand::random::<usize>(), 36).to_string();
    let prefix = "M".to_string() + &time_part[time_part.len()-3..] + &random_part[random_part.len()-4..] + "-"; // begin with letter
    add_ids_to_all(mathml, &prefix, 0);
    return mathml;

    fn add_ids_to_all<'a>(mathml: Element<'a>, id_prefix: &str, count: usize) -> usize {
        let mut count = count;
        if mathml.attribute("id").is_none() {
            mathml.set_attribute_value("id", (id_prefix.to_string() + &count.to_string()).as_str());
            mathml.set_attribute_value("data-id-added", "true");
            count += 1;
        };

        if crate::xpath_functions::is_leaf(mathml) {
            return count;
        }
        
        for child in mathml.children() {
            let child = crate::canonicalize::as_element(child);
            count = add_ids_to_all(child, id_prefix, count);
        }
        return count;
    }
}

pub fn get_element<'a>(package: &'a Package) -> Element<'a> {
    let doc = package.as_document();
    let mut result = None;
    for root_child in doc.root().children() {
        if let ChildOfRoot::Element(e) = root_child {
           assert!(result == None);
           result = Some(e);
        }
    };
    return result.unwrap();
}

#[allow(dead_code)]
fn trim_doc(doc: &Document) {
    for root_child in doc.root().children() {
        if let ChildOfRoot::Element(e) = root_child {
            trim_element(&e);
        } else {
            doc.root().remove_child(root_child);        // comment or processing instruction
        }
    };
}

/// Not really meant to be public -- used by tests in some packages
pub fn trim_element(e: &Element) {
    // "<mtext>this is text</mtext" results in 3 text children
    // these are combined into one child as it makes code downstream simpler
    if is_leaf(*e) {
        // Assume it is HTML inside of the leaf -- turn the HTML into a string
        make_leaf_element(*e);
        return;
    }

    let mut single_text = "".to_string();
    for child in e.children() {
        match child {
            ChildOfElement::Element(c) => {
                    trim_element(&c);
            },
            ChildOfElement::Text(t) => {
                single_text += t.text();
                e.remove_child(child);
            },
            _ => {
                e.remove_child(child);
            }
        }
    }

    // hack to avoid non-breaking whitespace from being removed -- move to a unique non-whitespace char then back
    const TEMP_NBSP: &str = "\u{F8FB}";
    let trimmed_text = single_text.replace(" ", TEMP_NBSP).trim().replace(TEMP_NBSP, " ");
    if !e.children().is_empty() && !trimmed_text.is_empty() {
        // FIX: we have a problem -- what should happen???
        // FIX: For now, just keep the children and ignore the text and log an error -- shouldn't panic/crash
        error!("trim_element: both element and textual children which shouldn't happen -- ignoring text '{}'", single_text);
    }
    if e.children().is_empty() && !single_text.is_empty() {
        // debug!("Combining text in {}: '{}' -> '{}'", e.name().local_part(), single_text, trimmed_text);
        e.set_text(&trimmed_text);
    }

    fn make_leaf_element(mathml_leaf: Element) {
        // MathML leaves like <mn> really shouldn't have non-textual content, but you could have embedded HTML
        // Here, we take convert them to leaves by grabbing up all the text and making that the content
        // Potentially, we leave them and let (default) rules do something, but it makes other parts of the code
        //   messier because checking the text of a leaf becomes Option<&str> rather than just &str
        let children = mathml_leaf.children();
        if children.is_empty() {
            return;
        }

        // gather up the text
        let mut text ="".to_string();
        for child in children {
            match child {
                ChildOfElement::Element(e) => {
                    make_leaf_element(e);
                    match e.children()[0] {
                        ChildOfElement::Text(t) => text += t.text(),
                        _ => panic!("as_text: internal error -- make_leaf_element found non-text child"),
                    }
                }
                ChildOfElement::Text(t) => text += t.text(),
                _ => (),
            }
        }

        // get rid of the old children and replace with the text we just built
        mathml_leaf.clear_children();
        mathml_leaf.set_text(&text);
    }
}


// used for testing trim
// returns true if two Documents are equal
#[allow(dead_code)]
fn is_same_doc(doc1: &Document, doc2: &Document) -> bool {
    if doc1.root().children().len() != doc2.root().children().len() {
        return false;
    }
    for root_child in doc1.root().children().iter().zip(doc2.root().children().iter()) {
        let (c1, c2) = root_child;
        match c1 {
            ChildOfRoot::Element(e1) => {
                if let ChildOfRoot::Element(e2) = c2 {
                    if is_same_element(e1, e2) {
                        continue;
                    }
                }
                return false;
            },
            ChildOfRoot::Comment(com1) => {
                if let ChildOfRoot::Comment(com2) = c2 {
                    if com1.text() == com2.text() {
                        continue;
                    }
                }
                return false;
            }
            ChildOfRoot::ProcessingInstruction(p1) => {
                if let ChildOfRoot::ProcessingInstruction(p2) = c2 {
                    if p1.target() == p2.target() && p1.value() == p2.value() {
                        continue;
                    }
                }
                return false;
            }
        }
    };
    return true;
}

/// Not really meant to be public -- used by tests in some packages
#[allow(dead_code)]
pub fn is_same_element(e1: &Element, e2: &Element) -> bool {
    // assume 'e' doesn't have element children until proven otherwise
    // this means we keep Text children until we are proven they aren't needed
    if e1.children().len() != e2.children().len() {
        return false;
    }
    for element_child in e1.children().iter().zip(e2.children().iter()) {
        let (c1, c2) = element_child;
        match c1 {
            ChildOfElement::Element(child1) => {
                if let ChildOfElement::Element(child2) = c2 {
                    if is_same_element(child1, child2) {
                        continue;
                    }
                }
                return false;
            },
            ChildOfElement::Comment(com1) => {
                if let ChildOfElement::Comment(com2) = c2 {
                    if com1.text() == com2.text() {
                        continue;
                    }
                }
                return false;
            }
            ChildOfElement::ProcessingInstruction(p1) => {
                if let ChildOfElement::ProcessingInstruction(p2) = c2 {
                    if p1.target() == p2.target() && p1.value() == p2.value() {
                        continue;
                    }
                }
                return false;
            }
            ChildOfElement::Text(t1) => {
                if let ChildOfElement::Text(t2) = c2 {
                    if t1.text() == t2.text() {
                        continue;
                    }
                    // debug!("#1 '{}[{}]', #2 '{}[{}]'", t1.text(), t1.text().len(),
                    //         t2.text(), t2.text().len());
                    // t1.text().chars().enumerate()
                    //     .for_each(|(i, ch1)| {
                    //         let ch2 = t2.text().chars().nth(i).unwrap();
                    //         debug!("  {}: {}/{}, {}", i, ch1,  ch2, ch1==ch2)
                    //     })
                }
                return false;
            }
        }
    };
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn are_parsed_strs_equal(str1: &str, str2: &str) -> bool {
        let package1 = &parser::parse(str1).expect("Failed to parse input");
        let doc1 = package1.as_document();
        trim_doc(&doc1);
        debug!("doc1:\n{}", mml_to_string(&get_element(&package1)));
        
        let package2 = parser::parse(str2).expect("Failed to parse input");
        let doc2 = package2.as_document();
        trim_doc(&doc2);
        debug!("doc2:\n{}", mml_to_string(&get_element(&package2)));
            
        is_same_doc(&doc1, &doc2)
    }

    #[test]
    fn trim_same() {
        let trimmed_str = "<math><mrow><mo>-</mo><mi>a</mi></mrow></math>";
        assert!(are_parsed_strs_equal(trimmed_str, trimmed_str));
    }

    #[test]
    fn trim_whitespace() {
        let trimmed_str = "<math><mrow><mo>-</mo><mi> a </mi></mrow></math>";
        let whitespace_str = "<math> <mrow ><mo>-</mo><mi> a </mi></mrow ></math>";
        assert!(are_parsed_strs_equal(trimmed_str, whitespace_str));
    }

    #[test]
    fn no_trim_whitespace_nbsp() {
        let trimmed_str = "<math><mrow><mo>-</mo><mtext> &#x00A0;a </mtext></mrow></math>";
        let whitespace_str = "<math> <mrow ><mo>-</mo><mtext> &#x00A0;a </mtext></mrow ></math>";
        assert!(are_parsed_strs_equal(trimmed_str, whitespace_str));
    }

    #[test]
    fn trim_comment() {
        let whitespace_str = "<math> <mrow ><mo>-</mo><mi> a </mi></mrow ></math>";
        let comment_str = "<math><mrow><mo>-</mo><!--a comment --><mi> a </mi></mrow></math>";
        assert!(are_parsed_strs_equal(comment_str, whitespace_str));
    }
 
    #[test]
    fn trim_differs() {
        let whitespace_str = "<math> <mrow ><mo>-</mo><mi> a </mi></mrow ></math>";
        let different_str = "<math> <mrow ><mo>-</mo><mi> b </mi></mrow ></math>";
        assert!(!are_parsed_strs_equal(different_str, whitespace_str));
    }
}