//! The interface module provides functionality both for calling from an API and also running the code from `main`.
//!
//! When calling from python, the general ordering is:
//! 1. whatever preferences the AT needs to set, it is done with calls to [`SetPreference`].
//! 2. the MathML is sent over via [`SetMathML`].
//! 3. AT calls to get the speech [`GetSpokenText`] and calls [`GetBraille`] to get the (Unicode) braille.
//!
//! AT can pass key strokes to allow a user to navigate the MathML by calling [`DoNavigateKeyPress`]; the speech is returned.
//! To get the MathML associated with the current navigation node, call [`GetNavigationMathML`].
//!
//! When calling from `main`, getting speech is done with [`speak_mathml`] which will parse the MathML, canonicalize it,
//! then invoke the speech rules on it.

// for Python interfaces --#[...] doesn't help on name mangled python function names
#![allow(non_snake_case)]
#![allow(clippy::needless_return)]
use std::cell::RefCell;

use sxd_document::parser;
use sxd_document::Package;
use sxd_document::dom::*;
use crate::errors::*;
use regex::Regex;


use crate::prefs::PreferenceManager;
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
pub fn SetRulesDir(dir: String) -> Result<()> {
    use std::path::PathBuf;
    return crate::prefs::PreferenceManager::initialize(PathBuf::from(dir));
}
/// The MathML to be spoken, brailled, or navigated.
///
/// This will override any previous MathML that was set.
pub fn SetMathML(mathml_str: String) -> Result<String> {
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

        // this forces initialization of things beyond just the speech rules (e.g, the defs.yaml files get read)
        crate::speech::SPEECH_RULES.with(|speech_rules| -> Result<()> {
            if let Some(e) = speech_rules.borrow().get_error() {bail!("{}", e)} else {Ok(())}
        })?;
        let new_package = new_package.unwrap();
        let mathml = cleanup_mathml(get_element(&new_package))?;
        let mathml_string = mml_to_string(&mathml);
        old_package.replace(new_package);

        return Ok( mathml_string );
    })
}

/// Get the spoken text of the MathML that was set.
/// The speech takes into account any AT or user preferences.
pub fn GetSpokenText() -> Result<String> {
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

/// Get the spoken text of the MathML that was set.
/// The speech takes into account any AT or user preferences.
pub fn GetOverviewText() -> Result<String> {
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

/// Set an API preference. The preference name should be a known preference name.
/// The value should either be a string or a number (depending upon the preference being set)
///
/// This function can be called multiple times to set different values.
/// The values are persistent but can be overwritten by setting a preference with the same name and a different value.
pub enum StringOrFloat {
    AsString(String),
    AsFloat(f64),
}

pub fn GetPreference(name: String) -> Option<String> {
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

pub fn SetPreference(name: String, value: StringOrFloat) -> Result<()> {
    return crate::speech::SPEECH_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        if let Some(error_string) = rules.get_error() {
            bail!("{}", error_string);
        }
        // note: Rust complains if I set
        //    pref_manager = rules.pref_manager.borrow_mut()
        // here/upfront, so it is borrowed separately below. That way its borrowed lifetime is small
        //let value_as_py_float = value.downcast::<PyFloat>();

        match name.to_lowercase().as_str() {
            "speechstyle" => {
                let files_changed;
                { 
                    let mut pref_manager = rules.pref_manager.borrow_mut();
                    files_changed = pref_manager.set_user_prefs("SpeechStyle", to_string(&name, value)?.as_str());
                };
                rules.invalidate(files_changed);
            },
            "verbosity" | "navverbosity" => {
                let pref_name = if name.to_lowercase().as_str()=="verbosity" {"Verbosity"} else {"NavVerbosity"};
                let value = match to_string(&name, value)?.to_lowercase().as_str() {
                    "terse" => "Terse".to_string(),
                    "medium" => "Medium".to_string(),
                    "verbose" => "Verbose".to_string(),
                    _ => rules.pref_manager.borrow().get_user_prefs().to_string(pref_name),
                    };

                rules.pref_manager.borrow_mut().set_user_prefs(pref_name, value.as_str());
            },
            "navmode" => {
                let value = match to_string(&name, value)?.to_lowercase().as_str() {
                    "enhanced" => "Enhanced".to_string(),
                    "simple" => "Simple".to_string(),
                    "character" => "Character".to_string(),
                    _ => rules.pref_manager.borrow().get_user_prefs().to_string("NavMode"),
                    };

                rules.pref_manager.borrow_mut().set_user_prefs("NavMode", value.as_str());
            },
            "speechtags" | "tts" => {
                return set_speech_tags(&mut rules.pref_manager.borrow_mut(), to_string(&name, value)?);
            },
            "language" => {
                let value_as_string = to_string(&name, value)?;
                // check the format
                if !( value_as_string.len() == 2 ||
                      (value_as_string.len() == 5 && value_as_string.as_bytes()[2] == b'-') ) {
                        bail!("Improper format for 'Language' preference '{}'. Should be of form 'en' or 'en-gb'", value_as_string);
                      }
                let files_changed = rules.pref_manager.borrow_mut().set_user_prefs("Language", value_as_string.as_str());  
                rules.invalidate(files_changed);  
            },
            "code" => {
                let files_changed = rules.pref_manager.borrow_mut().set_user_prefs("Code", to_string(&name, value)?.as_str());    
                crate::speech::BRAILLE_RULES.with(|braille_rules| {
                    braille_rules.borrow_mut().invalidate(files_changed);
                })
            },
            "braillenavhighlight" => {
                rules.pref_manager.borrow_mut().set_user_prefs("BrailleNavHighlight", to_string(&name, value)?.as_str());    
            },
            "pitch" => {
                rules.pref_manager.borrow_mut().set_api_float_pref("Pitch".to_string(), to_float(&name, value)?);    
            },
            "rate" => {
                rules.pref_manager.borrow_mut().set_api_float_pref("Rate".to_string(), to_float(&name, value)?);    
            },
            "volume" => {
                rules.pref_manager.borrow_mut().set_api_float_pref("Volume".to_string(), to_float(&name, value)?);    
            },
            "gender" => {
                rules.pref_manager.borrow_mut().set_api_string_pref("Gender".to_string(), to_string(&name, value)?);    
            },
            "voice" => {
                rules.pref_manager.borrow_mut().set_api_string_pref("Voice".to_string(), to_string(&name, value)?);    
            },
            "bookmark" => {
                rules.pref_manager.borrow_mut().set_api_boolean_pref("Bookmark".to_string(), to_string(&name, value)?.to_lowercase()=="true");    
            },
                _ => {

            }
        }
        return Ok( () );
    });

    fn to_string(name: &str, value: StringOrFloat) -> Result<String> {
        return match value {
            StringOrFloat::AsString(s) => Ok(s),
            StringOrFloat::AsFloat(f) => bail!("SetPreference: preference'{}'s value '{}' must be a string", name, f),
        };
    }

    fn to_float(name: &str, value: StringOrFloat) -> Result<f64> {
        return match value {
            StringOrFloat::AsString(s) => bail!("SetPreference: preference'{}'s value '{}' must be a float", name, s),
            StringOrFloat::AsFloat(f) => Ok(f),
        };
    }
}

fn set_speech_tags(pref_manager: &mut PreferenceManager, speech_tags: String ) -> Result<()> {
    let tts = match speech_tags.to_lowercase().as_str() {
        "0" | "none" => "none",
        // 1 => "sapi4",
        "2" | "sapi5" => "sapi5",
        // 3 => "Mac",
        "4" | "ssml" => "ssml",
        // 6 => "eloquence",
        _ => bail!("Unknown value '{}' for SetSpeechTags", speech_tags),
    };
    pref_manager.set_api_string_pref("TTS".to_string(), tts.to_string());
    return Ok( () );
}


/// Get the braille associated with the MathML that was set by [`SetMathML`].
/// The braille returned depends upon the preference for braille output.
pub fn GetBraille(nav_node_id: String) -> Result<String> {
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
///
/// The spoken text for the new current node is returned.
pub fn DoNavigateKeyPress(key: usize, shift_key: bool, control_key: bool, alt_key: bool, meta_key: bool) -> Result<String> {
    return MATHML_INSTANCE.with(|package_instance| {
        let package_instance = package_instance.borrow();
        let mathml = get_element(&*package_instance);
        return do_navigate_key_press(mathml, key, shift_key, control_key, alt_key, meta_key);
    });
}

pub fn DoNavigateCommand(command: String) -> Result<String> {
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
pub fn GetNavigationMathML() -> Result<(String, usize)> {
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

pub fn GetNavigationMathMLId() -> Result<(String, usize)> {
    return MATHML_INSTANCE.with(|package_instance| {
        let package_instance = package_instance.borrow();
        let mathml = get_element(&*package_instance);
        return Ok( NAVIGATION_STATE.with(|nav_stack| {
            return nav_stack.borrow().get_navigation_mathml_id(mathml);
        }) )
    });
}


/// Convert the returned error from SetMathML, etc., to a useful string for display
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