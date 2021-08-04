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
//! TODO: Braille generation and navigation are just stubs at the moment.
//!
//! When calling from `main`, getting speech is done with [`speak_mathml`] which will parse the MathML, canonicalize it,
//! then invoke the speech rules on it.

// for Python interfaces --#[...] doesn't help on name mangled python function names
#![allow(non_snake_case)]
use std::cell::{RefCell};

use sxd_document::parser;
use sxd_document::Package;
use sxd_document::dom::*;

use crate::prefs::PreferenceManager;
use crate::navigate::*;

// wrap up some common functionality between the call from 'main' and AT
fn prepare_mathml_for_speech(package: &Package) {
    let mathml = get_element(package);
    trim_element(&mathml);
    let mathml = crate::canonicalize::canonicalize(mathml);
    crate::infer_intent::infer_intent(&mathml);
}

/// Given MathML, a string is return.
///
/// If there is an error, the string will give a general message (e.g., "MathML error" if the MathML is bad)
pub fn speak_mathml(mathml_str: &str) -> String {
    // this forces initialization
    crate::speech::SPEECH_RULES.with(|_| true);
    let package = parser::parse(mathml_str);
    if let Err(e) = package {
        eprintln!("Error in parsing MathML: {}.\nThe MathML is:\n{}", e.to_string(), mathml_str);       // FIX: log 
        return "Invalid MathML. Unable to speak math.".to_string();
    };
    let package = package.unwrap();
    prepare_mathml_for_speech(&package);
    let mathml = get_element(&package);
    return crate::speech::speak_mathml(&mathml);
}

thread_local!{
/// The current node being navigated (also spoken and brailled) is stored in `MATHML_INSTANCE`.
pub static MATHML_INSTANCE: RefCell<Package> = init_mathml_instance();
}

fn init_mathml_instance<'a>() -> RefCell<Package> {
    let package = parser::parse("<math></math>")
        .expect("Internal error in 'init_mathml_instance;: didn't parse initializer string");
    return RefCell::new( package );
}

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::{PyTypeError, PyValueError};

#[pyfunction]
/// The MathML to be spoken, brailled, or navigated.
///
/// This will override any previous MathML that was set.
pub fn SetMathML(_py: Python, mathml_str: String) -> PyResult<()> {
    return MATHML_INSTANCE.with(|old_package| {
        let new_package = parser::parse(&mathml_str);    
        if let Err(_) = new_package {
            panic!("MathML input was not valid"); // FIX: improve error
        } 
        let new_package = new_package.unwrap();
        prepare_mathml_for_speech(&new_package);
        old_package.replace(new_package);
        return Ok( () );
    })
}

#[pyfunction]
/// Get the spoken text of the MathML that was set.
/// The speech takes into account any AT or user preferences.
pub fn GetSpokenText(_py: Python) -> PyResult<String> {
    use std::time::{Instant};
    let instant = Instant::now();
    return MATHML_INSTANCE.with(|package_instance| {
        let package_instance = package_instance.borrow();
        let mathml = get_element(&*package_instance);
        let speech = crate::speech::speak_mathml(&mathml);
        eprintln!("Time taken: {}ms", instant.elapsed().as_millis());
        return Ok( speech );
    });
}

#[pyfunction]
/// Set an API preference. The preference name should be a known preference name.
/// The value should either be a string or a number (depending upon the preference being set)
///
/// This function can be called multiple times to set different values.
/// The values are persistent but can be overwritten by setting a preference with the same name and a different value.
pub fn SetPreference(_py: Python, name: String, value: &PyAny) -> PyResult<()> {

    return crate::speech::SPEECH_RULES.with(|rules| {
        let mut rules = rules.borrow_mut();
        let pref_manager = rules.pref_manager.as_mut();
        //let value_as_py_float = value.downcast::<PyFloat>();

        match name.to_lowercase().as_str() {
            "speechtags" | "tts" => {
                return set_speech_tags(pref_manager, to_string(&name, value)?);
            },
            "language" => {
                let value_as_string = to_string(&name, value)?;
                // check the format
                if !( value_as_string.len() == 2 ||
                      (value_as_string.len() == 5 && value_as_string.as_bytes()[2] == '-' as u8) ) {
                        return Err( PyValueError::new_err(
                            format!("Improper format for 'Language' preference '{}'. Should be of form 'en' or 'en-gb'", value_as_string)));
                      }
                pref_manager.set_api_string_pref("Language".to_string(), value_as_string);    
            },
            "pitch" => {
                pref_manager.set_api_float_pref("Pitch".to_string(), to_float(&name, value)?);    
            },
            "rate" => {
                pref_manager.set_api_float_pref("Rate".to_string(), to_float(&name, value)?);    
            },
            "volume" => {
                pref_manager.set_api_float_pref("Volume".to_string(), to_float(&name, value)?);    
            },
            _ => {

            }
        }
        return Ok( () );
    });

    fn to_string(name: &str, value: &PyAny) -> PyResult<String>{
        return match value.extract::<String>() {
            Ok(s) => Ok( s ),
            Err(_) => Err( PyTypeError::new_err(
                format!("SetPreference: preference'{}'s value '{}' must be a string", name, value))),
        };
    }

    fn to_float(name: &str, value: &PyAny) -> PyResult<f64>{
        return match value.extract::<f64>() {
            Ok(f) => Ok( f ),
            Err(_) => Err( PyTypeError::new_err(
                format!("SetPreference: preference'{}'s value '{}' must be a number", name, value))),
        };
    }
}

fn set_speech_tags(pref_manager: &mut PreferenceManager, speech_tags: String ) -> PyResult<()> {
    let tts = match speech_tags.to_lowercase().as_str() {
        "0" | "none" => "none",
        // 1 => "sapi4",
        "2" | "sapi5" => "sapi5",
        // 3 => "Mac",
        "4" | "ssml" => "ssml",
        // 6 => "eloquence",
        _ => return Err( PyValueError::new_err(
                    format!("Unknown value '{}' for SetSpeechTags", speech_tags))),
        // Err(),
    };
    pref_manager.set_api_string_pref("TTS".to_string(), tts.to_string());
    return Ok( () );
}


#[pyfunction]
#[allow(unused_variables)]
/// Get the braille associated with the MathML that was set by [`SetMathML`].
/// The braille returned depends upon the preference for braille output.
pub fn GetBraille(_py: Python) -> PyResult<String> {
    // FIX: not yet implemented (basically what the braille says)
    return Ok("⠠⠃⠗⠁⠊⠇⠇⠑ ⠛⠑⠝⠑⠗⠁⠞⠊⠕⠝ ⠝⠕⠞ ⠽⠑⠞ ⠊⠍⠏⠇⠑⠍⠑⠝⠞".to_string());
}

#[pyfunction]
/// Given a key code along with the modifier keys, the current node is moved accordingly (or value reported in some cases).
///
/// The spoken text for the new current node is returned.
pub fn DoNavigateKeyPress(_py: Python, key: usize, shift_key: bool, control_key: bool, alt_key: bool, meta_key: bool) -> PyResult<String> {
    return Ok( do_navigate_key_press(key, shift_key, control_key, alt_key, meta_key) );
}

#[pyfunction]
/// Return the MathML associated with the current (navigation) node.
pub fn GetNavigationMathML(_py: Python) -> PyResult<String> {
    // FIX: not yet implemented (basically what the braille says)
    return Ok( get_navigation_mathml() );
}

#[pymodule]
fn mathcat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(SetMathML, m)?).unwrap();
    m.add_function(wrap_pyfunction!(GetSpokenText, m)?).unwrap();
    m.add_function(wrap_pyfunction!(SetPreference, m)?).unwrap();
    m.add_function(wrap_pyfunction!(GetBraille, m)?).unwrap();
    m.add_function(wrap_pyfunction!(DoNavigateKeyPress, m)?).unwrap();
    m.add_function(wrap_pyfunction!(GetNavigationMathML, m)?).unwrap();

    return Ok( () );
}


/// Not really meant to be public -- used by tests in some packages
pub fn get_element<'d>(package: &'d Package) -> Element<'d> {
    let doc = package.as_document();
    let mut result = None;
    for root_child in doc.root().children() {
        if let ChildOfRoot::Element(e) = root_child {
           assert!(result == None);
           result = Some(e);
        }
    };
    let element = result.unwrap();
    element
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
    // assume 'e' doesn't have element children until proven otherwise
    // this means we keep Text children until we are proven they aren't needed
    for child in e.children() {
        match child {
            ChildOfElement::Element(_) => {
                for child in e.children() {
                    if let ChildOfElement::Element(el) = child {
                        trim_element(&el);
                    } else {
                        e.remove_child(child);        // text, comment, or processing instruction
                    }
                }
                return;
            },
            ChildOfElement::Text(t) => {
                let trimmed_text = t.text().trim();
                if trimmed_text.len() > 0 {     // don't throw out Text() that is only whitespace as that is meaningful in mtext
                    t.set_text( t.text().trim());
                }
            },
            _ => {
                e.remove_child(child);
            }
        }
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
        
        let package2 = parser::parse(str2).expect("Failed to parse input");
        let doc2 = package2.as_document();
        trim_doc(&doc2);
            
        is_same_doc(&doc1, &doc2)
    }

    #[test]
    fn trim_same() {
        let trimmed_str = "<math><mrow><mo>-</mo><mi>a</mi></mrow></math>";
        assert!(are_parsed_strs_equal(trimmed_str, trimmed_str));
    }

    #[test]
    fn trim_whitespace() {
        let trimmed_str = "<math><mrow><mo>-</mo><mi>a</mi></mrow></math>";
        let whitespace_str = "<math> <mrow ><mo>-</mo><mi> a </mi></mrow ></math>";
        assert!(are_parsed_strs_equal(trimmed_str, whitespace_str));
    }

    #[test]
    fn trim_comment() {
        let whitespace_str = "<math> <mrow ><mo>-</mo><mi> a </mi></mrow ></math>";
        let comment_str = "<math><mrow><mo>-</mo><!--a comment --><mi>a</mi></mrow></math>";
        assert!(are_parsed_strs_equal(comment_str, whitespace_str));
    }
 
    #[test]
    fn trim_differs() {
        let whitespace_str = "<math> <mrow ><mo>-</mo><mi> a </mi></mrow ></math>";
        let different_str = "<math> <mrow ><mo>-</mo><mi> b </mi></mrow ></math>";
        assert!(!are_parsed_strs_equal(different_str, whitespace_str));
    }
}