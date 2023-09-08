//! The interface module provides functionality both for calling from an API and also running the code from `main`.
//!
#![allow(non_snake_case)]
#![allow(clippy::needless_return)]
use std::cell::RefCell;

use sxd_document::parser;
use sxd_document::Package;
use sxd_document::dom::*;
use crate::errors::*;
use crate::prefs::FilesChanged;
use regex::{Regex, Captures};
use phf::phf_map;

use crate::canonicalize::{name, as_element};


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
        static ref NAMESPACE_DECL: Regex = Regex::new(r#"xmlns:[[:alpha:]]+"#).unwrap();     // very limited namespace prefix match
        static ref PREFIX: Regex = Regex::new(r#"(</?)[[:alpha:]]+:"#).unwrap();     // very limited namespace prefix match
        static ref HTML_ENTITIES: Regex = Regex::new(r#"&([a-zA-Z]+?);"#).unwrap();
    }

    NAVIGATION_STATE.with(|nav_stack| {
        nav_stack.borrow_mut().reset();
    });
    return MATHML_INSTANCE.with(|old_package| {
        // FIX: convert this to an included file once I get the full entity list
        static HTML_ENTITIES_MAPPING: phf::Map<&str, &str> = include!("entities.in");

        let mut error_message = "".to_string();     // can't return a result inside the replace_all, so we do this hack of setting the message and then returning the error
        // need to deal with character data and convert to something the parser knows
        let mathml_str = HTML_ENTITIES.replace_all(&mathml_str, |cap: &Captures| {
            match HTML_ENTITIES_MAPPING.get(&cap[1]) {
                None => {
                    error_message = format!("No entity named '{}'", &cap[0]);
                    cap[0].to_string()
                },
                Some(&ch) => ch.to_string(),
            }
        });

        if !error_message.is_empty() {
            bail!(error_message);
        }
        let mathml_str = MATHJAX_V2.replace_all(&mathml_str, "");
        let mathml_str = MATHJAX_V3.replace_all(&mathml_str, "");

        // the speech rules use the xpath "name" function and that includes the prefix
        // getting rid of the prefix properly probably involves a recursive replacement in the tree
        // if the prefix is used, it is almost certainly something like "m" or "mml", so this cheat will work.
        let mathml_str = NAMESPACE_DECL.replace(&mathml_str, "xmlns");  // do this before the PREFIX replace!
        let mathml_str = PREFIX.replace_all(&mathml_str, "$1");

        let new_package = parser::parse(&mathml_str);    
        if let Err(e) = new_package {
            bail!("Invalid MathML input:\n{}\nError is: {}", &mathml_str, &e.to_string());
        }
        crate::speech::SpeechRules::initialize_all_rules()?;

        let new_package = new_package.unwrap();
        let mathml = get_element(&new_package);
        let mathml = cleanup_mathml(mathml)?;
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
        let mathml = get_element(&package_instance);
        let new_package = Package::new();
        let intent = crate::speech::intent_from_mathml(mathml, new_package.as_document())?;
        debug!("Intent tree:\n{}", mml_to_string(&intent));
        let speech = crate::speech::speak_mathml(intent, "")?;
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
        let mathml = get_element(&package_instance);
        let speech = crate::speech::overview_mathml(mathml, "")?;
        // info!("Time taken: {}ms", instant.elapsed().as_millis());
        return Ok( speech );
    });
}

/// Get the value of the named preference.
/// None is returned if `name` is not a known preference.
pub fn get_preference(name: String) -> Result<String> {
    use crate::prefs::NO_PREFERENCE;
    return crate::speech::SPEECH_RULES.with(|rules| {
        let rules = rules.borrow();
        let pref_manager = rules.pref_manager.borrow();
        let mut value = pref_manager.pref_to_string(&name);
        if value == NO_PREFERENCE {
            value = pref_manager.pref_to_string(&name);
        }
        if value == NO_PREFERENCE {
            bail!("No preference named '{}'", &name);
        } else {
            return Ok(value);
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
/// Be careful setting preferences -- these potentially override user settings, so only preferences that really need setting should be set.
pub fn set_preference(name: String, value: String) -> Result<()> {
    let old_value = get_preference(name.clone())?;      // make sure it is a valid preference
    
    if name == "Language" {
        // check the format
        if !( value == "Auto" ||
                value.len() == 2 ||
                (value.len() == 5 && value.as_bytes()[2] == b'-') ) {
            bail!("Improper format for 'Language' preference '{}'. Should be of form 'en' or 'en-gb'", value);
        }
    }
    
    crate::speech::SPEECH_RULES.with(|rules| {
        let rules = rules.borrow_mut();
        if let Some(error_string) = rules.get_error() {
            bail!("{}", error_string);
        }

        // we set the value even if it was the same as the old value because this might override a potentially changed future user value
        let mut pref_manager = rules.pref_manager.borrow_mut();
        let lower_case_value = value.to_lowercase();
        if lower_case_value == "true" || lower_case_value == "false" {
            pref_manager.set_api_boolean_pref(&name, value.to_lowercase()=="true"); 
        } else { 
            match name.as_str() {
                "Pitch" | "Rate" | "Volume" | "CapitalLetters_Pitch"=> {
                    pref_manager.set_api_float_pref(&name, to_float(&name, &value)?);    
                },
                _ => {
                    pref_manager.set_api_string_pref(&name, &value);
                },
            }
        };
        return Ok::<(), Error>( () );
    })?;

    if old_value == value {
        return Ok( () );            // nothing changed
    }

    if let Some(changed) = FilesChanged::new(&name) {
        crate::speech::SpeechRules::invalidate(changed);
    }
    return Ok( () );

    fn to_float(name: &str, value: &str) -> Result<f64> {
        match value.parse::<f64>() {
            Ok(val) => return Ok(val),
            Err(_) => bail!("SetPreference: preference'{}'s value '{}' must be a float", name, value),
        };
    }
}

/// Get the braille associated with the MathML that was set by [`set_mathml`].
/// The braille returned depends upon the preference for the `code` preference (default `Nemeth`).
pub fn get_braille(nav_node_id: &str) -> Result<String> {
    // use std::time::{Instant};
    // let instant = Instant::now();
    return MATHML_INSTANCE.with(|package_instance| {
        let package_instance = package_instance.borrow();
        let mathml = get_element(&package_instance);
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
        let mathml = get_element(&package_instance);
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
        let mathml = get_element(&package_instance);
        return do_navigate_command_string(mathml, command);
    });
}

/// Return the MathML associated with the current (navigation) node.
/// The returned result is the `id` of the node and the offset (0-based) from that node (not yet implemented)
/// The offset is needed for token elements that have multiple characters.
pub fn get_navigation_mathml() -> Result<(String, usize)> {
    return MATHML_INSTANCE.with(|package_instance| {
        let package_instance = package_instance.borrow();
        let mathml = get_element(&package_instance);
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
        let mathml = get_element(&package_instance);
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


fn add_ids(mathml: Element) -> Element {
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

    fn add_ids_to_all(mathml: Element, id_prefix: &str, count: usize) -> usize {
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
            let child = as_element(child);
            count = add_ids_to_all(child, id_prefix, count);
        }
        return count;
    }
}

pub fn get_element(package: &Package) -> Element {
    let doc = package.as_document();
    let mut result = None;
    for root_child in doc.root().children() {
        if let ChildOfRoot::Element(e) = root_child {
           assert!(result.is_none());
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
    const TEMP_NBSP: &str = "\u{F8FB}";

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
    let trimmed_text = single_text.replace(' ', TEMP_NBSP).trim().replace(TEMP_NBSP, " ");
    if !(is_leaf(*e) || name(e) == "intent-literal" || single_text.is_empty()) {  // intent-literal comes from testing intent
        // FIX: we have a problem -- what should happen???
        // FIX: For now, just keep the children and ignore the text and log an error -- shouldn't panic/crash
        if !trimmed_text.is_empty() {
            error!("trim_element: both element and textual children which shouldn't happen -- ignoring text '{}'", single_text);
        }
        return;
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
        let mut previous_element_was_text = false;
        for child in children {
            let (child_text, space) = match child {
                ChildOfElement::Element(child) => {
                    previous_element_was_text = false;
                    if name(&child) == "mglyph" {
                        (child.attribute_value("alt").unwrap_or("").to_string(), " ")
                    } else {
                        (gather_text(child), " ")
                    }
                },
                ChildOfElement::Text(t) => {
                    let space = !previous_element_was_text;
                    previous_element_was_text = true;
                    (t.text().to_string(), if space {" "} else {""})
                },
                _ => ("".to_string(), ""),
            };
            if !child_text.is_empty() {
                // hack to avoid non-breaking whitespace from being removed -- move to a unique non-whitespace
                text = text + space + child_text.replace(' ', TEMP_NBSP).trim_start();
            } 

        }

        // get rid of the old children and replace with the text we just built
        mathml_leaf.clear_children();

        // move hack back to non-breaking whitespace
        let trimmed_text = text.trim().replace(TEMP_NBSP, " ");
        mathml_leaf.set_text(&trimmed_text);

        /// gather up all the contents of the element and return them with a leading space
        fn gather_text(html: Element) -> String {
            let mut text = "".to_string();      // since we are throwing out the element tag, add a space between the contents
            for child in html.children() {
                match child {
                    ChildOfElement::Element(child) => {
                        text = text + " " + gather_text(child).trim_start();
                    },
                    ChildOfElement::Text(t) => text += t.text(),
                    _ => (),
                }
            }
            return text;
        }
    }
}


// used for testing trim
/// returns Ok() if two Documents are equal or some info where they differ in the Err
#[allow(dead_code)]
fn is_same_doc(doc1: &Document, doc2: &Document) -> Result<()> {
    // assume 'e' doesn't have element children until proven otherwise
    // this means we keep Text children until we are proven they aren't needed
    if doc1.root().children().len() != doc2.root().children().len() {
        bail!("Children of docs have {} != {} children", doc1.root().children().len(), doc2.root().children().len());
    }

    for (i, (c1, c2)) in doc1.root().children().iter().zip(doc2.root().children().iter()).enumerate() {
        match c1 {
            ChildOfRoot::Element(e1) => {
                if let ChildOfRoot::Element(e2) = c2 {
                    is_same_element(e1, e2)?;
                } else {
                    bail!("child #{}, first is element, second is something else", i);
                }
            },
            ChildOfRoot::Comment(com1) => {
                if let ChildOfRoot::Comment(com2) = c2 {
                    if com1.text() != com2.text() {
                        bail!("child #{} -- comment text differs", i);
                    }
                } else {
                    bail!("child #{}, first is comment, second is something else", i);
                }
            }
            ChildOfRoot::ProcessingInstruction(p1) => {
                if let ChildOfRoot::ProcessingInstruction(p2) = c2 {
                    if p1.target() != p2.target() || p1.value() != p2.value() {
                        bail!("child #{} -- processing instruction differs", i);
                    }
                } else {
                    bail!("child #{}, first is processing instruction, second is something else", i);
                }
            }
        }
    };
    return Ok( () );
}

/// returns Ok() if two Documents are equal or some info where they differ in the Err
// Not really meant to be public -- used by tests in some packages
#[allow(dead_code)]
pub fn is_same_element(e1: &Element, e2: &Element) -> Result<()> {
    if name(e1) != name(e2) {
        bail!("Names not the same: {}, {}", name(e1), name(e2));
    }

    // assume 'e' doesn't have element children until proven otherwise
    // this means we keep Text children until we are proven they aren't needed
    if e1.children().len() != e2.children().len() {
        bail!("Children of {} have {} != {} children", name(e1), e1.children().len(), e2.children().len());
    }

    if let Err(e) = attrs_are_same(e1.attributes(), e2.attributes()) {
        bail!("In element {}, {}", name(e1), e);
    }

    for (i, (c1, c2)) in e1.children().iter().zip(e2.children().iter()).enumerate() {
        match c1 {
            ChildOfElement::Element(child1) => {
                if let ChildOfElement::Element(child2) = c2 {
                    is_same_element(child1, child2)?;
                } else {
                    bail!("{} child #{}, first is element, second is something else", name(e1), i);
                }
            },
            ChildOfElement::Comment(com1) => {
                if let ChildOfElement::Comment(com2) = c2 {
                    if com1.text() != com2.text() {
                        bail!("{} child #{} -- comment text differs", name(e1), i);
                    }
                } else {
                    bail!("{} child #{}, first is comment, second is something else", name(e1), i);
                }
            }
            ChildOfElement::ProcessingInstruction(p1) => {
                if let ChildOfElement::ProcessingInstruction(p2) = c2 {
                    if p1.target() != p2.target() || p1.value() != p2.value() {
                        bail!("{} child #{} -- processing instruction differs", name(e1), i);
                    }
                } else {
                    bail!("{} child #{}, first is processing instruction, second is something else", name(e1), i);
                }
            }
            ChildOfElement::Text(t1) => {
                if let ChildOfElement::Text(t2) = c2 {
                    if t1.text() != t2.text() {
                        bail!("{} child #{} --  text differs", name(e1), i);
                    }
                } else {
                    bail!("{} child #{}, first is text, second is something else", name(e1), i);
                }
            }
        }
    };
    return Ok( () );

    /// compares attributes -- '==' didn't seems to work
    fn attrs_are_same(attrs1: Vec<Attribute>, attrs2: Vec<Attribute>) -> Result<()> {
        if attrs1.len() != attrs2.len() {
            bail!("Attributes have different length: {:?} != {:?}", attrs1, attrs2);
        }
        // can't guarantee attrs are in the same order
        for attr1 in attrs1 {
            if let Some(found_attr2) = attrs2.iter().find(|&attr2| attr1.name().local_part() == attr2.name().local_part()) {
                if attr1.value() == found_attr2.value() {
                    continue;
                } else {
                    bail!("Attribute named {} has differing values:\n  '{}'\n  '{}'", attr1.name().local_part(), attr1.value(), found_attr2.value());
                }
            } else {
                bail!("Attribute name {} not in [{}]", print_attr(&attr1), print_attrs(&attrs2));
            }
        }
        return Ok( () );

        fn print_attr(attr: &Attribute) -> String {
            return format!("@{}='{}'", attr.name().local_part(), attr.value());
        }
        fn print_attrs(attrs: &[Attribute]) -> String {
            return attrs.iter()
                .map(print_attr)
                .collect::<Vec<String>>()
                .join(", ");
        }
    }
}


#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
	use super::super::init_logger;
    use super::*;
    
    fn are_parsed_strs_equal(test: &str, target: &str) -> bool {
        let target_package = &parser::parse(target).expect("Failed to parse input");
        let target_doc = target_package.as_document();
        trim_doc(&target_doc);
        debug!("target:\n{}", mml_to_string(&get_element(&target_package)));
        
        let test_package = &parser::parse(test).expect("Failed to parse input");
        let test_doc = test_package.as_document();
        trim_doc(&test_doc);
        debug!("test:\n{}", mml_to_string(&get_element(&test_package)));

        match is_same_doc(&test_doc, &target_doc) {
			Ok(_) => return true,
			Err(e) => panic!("{}", e),
		}
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
    fn replace_mglyph() {
        let mglyph_str = "<math>
                <mrow>
                    <mi>X<mglyph fontfamily='my-braid-font' index='2' alt='23braid' /></mi>
                    <mo>+</mo>
                    <mi>
                        <mglyph fontfamily='my-braid-font' index='5' alt='132braid' />Y
                    </mi>
                    <mo>=</mo>
                    <mi>
                        <mglyph fontfamily='my-braid-font' index='3' alt='13braid' />
                    </mi>
                </mrow>
            </math>";
        let result_str = "<math>
            <mrow>
                <mi>X 23braid</mi>
                <mo>+</mo>
                <mi>132braid Y</mi>
                <mo>=</mo>
                <mi>13braid</mi>
            </mrow>
        </math>";
    assert!(are_parsed_strs_equal(mglyph_str, result_str));
    }
 
    #[test]
    fn trim_differs() {
        let whitespace_str = "<math> <mrow ><mo>-</mo><mi> a </mi></mrow ></math>";
        let different_str = "<math> <mrow ><mo>-</mo><mi> b </mi></mrow ></math>";

        // need to manually do this since failure shouldn't be a panic
        let package1 = &parser::parse(whitespace_str).expect("Failed to parse input");
        let doc1 = package1.as_document();
        trim_doc(&doc1);
        debug!("doc1:\n{}", mml_to_string(&get_element(&package1)));
        
        let package2 = parser::parse(different_str).expect("Failed to parse input");
        let doc2 = package2.as_document();
        trim_doc(&doc2);
        debug!("doc2:\n{}", mml_to_string(&get_element(&package2)));

        assert!(is_same_doc(&doc1, &doc2).is_err());
    }
 
    #[test]
    fn test_entities() {
        // this forces initialization
        set_rules_dir(super::super::abs_rules_dir_path()).unwrap();

        let entity_str = set_mathml("<math><mrow><mo>&minus;</mo><mi>&mopf;</mi></mrow></math>".to_string()).unwrap();
        let converted_str = set_mathml("<math><mrow><mo>&#x02212;</mo><mi>&#x1D55E;</mi></mrow></math>".to_string()).unwrap();
        
        // need to remove unique ids
        lazy_static! {
            static ref ID_MATCH: Regex = Regex::new(r#"id='.+?' "#).unwrap();
        }
        let entity_str = ID_MATCH.replace_all(&entity_str, "");
        let converted_str = ID_MATCH.replace_all(&converted_str, "");
        assert_eq!(entity_str, converted_str, "normal entity test failed");


        let entity_str = set_mathml("<math data-quot=\"&quot;value&quot;\" data-apos='&apos;value&apos;'><mi>XXX</mi></math>".to_string()).unwrap();
        let converted_str = set_mathml("<math data-quot='\"value\"' data-apos=\"'value'\"><mi>XXX</mi></math>".to_string()).unwrap();
        let entity_str = ID_MATCH.replace_all(&entity_str, "");
        let converted_str = ID_MATCH.replace_all(&converted_str, "");
        assert_eq!(entity_str, converted_str, "special entities quote test failed");

        let entity_str = set_mathml("<math><mo>&lt;</mo><mo>&gt;</mo><mtext>&amp;lt;</mtext></math>".to_string()).unwrap();
        let converted_str = set_mathml("<math><mo>&#x003C;</mo><mo>&#x003E;</mo><mtext>&#x0026;lt;</mtext></math>".to_string()).unwrap();
        let entity_str = ID_MATCH.replace_all(&entity_str, "");
        let converted_str = ID_MATCH.replace_all(&converted_str, "");
        assert_eq!(entity_str, converted_str, "special entities <,>,& test failed");
    }
    

    #[test]
    fn can_recover_from_invalid_set_rules_dir() {
        use std::env;
        // MathCAT will check the env var "MathCATRulesDir" as an override, so the following test might succeed if we don't override the env var
        env::set_var("MathCATRulesDir", "MathCATRulesDir");
        assert!(set_rules_dir("someInvalidRulesDir".to_string()).is_err());
        assert!(set_rules_dir(super::super::abs_rules_dir_path()).is_ok(), "\nset_rules_dir to '{}' failed", super::super::abs_rules_dir_path());
        assert!(set_mathml("<math><mn>1</mn></math>".to_string()).is_ok());
    }

    #[test]
    fn single_html_in_mtext() {
        let test = "<math><mn>1</mn> <mtext>a<p>para 1</p>aa</mtext> <mi>y</mi></math>";
        let target = "<math><mn>1</mn> <mtext>a para 1 aa</mtext> <mi>y</mi></math>";
        assert!(are_parsed_strs_equal(test, target));
    }

    #[test]
    fn multiple_html_in_mtext() {
        let test = "<math><mn>1</mn> <mtext>a<p>para 1</p><p>para 2</p>aa</mtext> <mi>y</mi></math>";
        let target = "<math><mn>1</mn> <mtext>a para 1 para 2 aa</mtext> <mi>y</mi></math>";
        assert!(are_parsed_strs_equal(test, target));
    }

    #[test]
    fn nested_html_in_mtext() {
        let test = "<math><mn>1</mn> <mtext>a<ol><li>first</li><li>second</li></ol>aa</mtext> <mi>y</mi></math>";
        let target = "<math><mn>1</mn> <mtext>a first second aa</mtext> <mi>y</mi></math>";
        assert!(are_parsed_strs_equal(test, target));
    }

    #[test]
    fn empty_html_in_mtext() {
        let test = "<math><mn>1</mn> <mtext>a<br/>aa</mtext> <mi>y</mi></math>";
        let target = "<math><mn>1</mn> <mtext>a aa</mtext> <mi>y</mi></math>";
        assert!(are_parsed_strs_equal(test, target));
    }
}