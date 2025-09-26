//! The speech module is where the speech rules are read in and speech generated.
//!
//! The speech rules call out to the preferences and tts modules and the dividing line is not always clean.
//! A number of useful utility functions used by other modules are defined here.
#![allow(clippy::needless_return)]
use std::path::PathBuf;
use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use sxd_document::dom::{ChildOfElement, Document, Element};
use sxd_document::{Package, QName};
use sxd_xpath::context::Evaluation;
use sxd_xpath::{Context, Factory, Value, XPath};
use sxd_xpath::nodeset::Node;
use std::fmt;
use std::time::SystemTime;
use crate::definitions::read_definitions_file;
use crate::errors::*;
use crate::prefs::*;
use yaml_rust::{YamlLoader, Yaml, yaml::Hash};
use crate::tts::*;
use crate::infer_intent::*;
use crate::pretty_print::{mml_to_string, yaml_to_string};
use std::path::Path;
use std::rc::Rc;
use crate::shim_filesystem::{read_to_string_shim, canonicalize_shim};
use crate::canonicalize::{as_element, create_mathml_element, set_mathml_name, name, MATHML_FROM_NAME_ATTR};
use regex::Regex;


pub const NAV_NODE_SPEECH_NOT_FOUND: &str = "NAV_NODE_NOT_FOUND";

/// Like lisp's ' (quote foo), this is used to block "replace_chars" being called.
///   Unlike lisp, this appended to the end of a string (more efficient)
/// At the moment, the only use is BrailleChars(...) -- internally, it calls replace_chars and we don't want it called again.
/// Note: an alternative to this hack is to add "xq" (execute but don't eval the result), but that's heavy-handed for the current need
const NO_EVAL_QUOTE_CHAR: char = '\u{e00A}';            // a private space char
const NO_EVAL_QUOTE_CHAR_AS_BYTES: [u8;3] = [0xee,0x80,0x8a];
const N_BYTES_NO_EVAL_QUOTE_CHAR: usize = NO_EVAL_QUOTE_CHAR.len_utf8();

/// Converts 'string' into a "quoted" string -- use is_quoted_string and unquote_string
pub fn make_quoted_string(mut string: String) -> String {
    string.push(NO_EVAL_QUOTE_CHAR);
    return string;
}

/// Checks the string to see if it is "quoted"
pub fn is_quoted_string(str: &str) -> bool {
    if str.len() < N_BYTES_NO_EVAL_QUOTE_CHAR {
        return false;
    }
    let bytes = str.as_bytes();
    return bytes[bytes.len()-N_BYTES_NO_EVAL_QUOTE_CHAR..] == NO_EVAL_QUOTE_CHAR_AS_BYTES;
}

/// Converts 'string' into a "quoted" string -- use is_quoted_string and unquote_string
/// IMPORTANT: this assumes the string is quoted -- no check is made
pub fn unquote_string(str: &str) -> &str {
    return &str[..str.len()-N_BYTES_NO_EVAL_QUOTE_CHAR];
}


/// The main external call, `intent_from_mathml` returns a string for the speech associated with the `mathml`.
///   It matches against the rules that are computed by user prefs such as "Language" and "SpeechStyle".
///
/// The speech rules assume `mathml` has been "cleaned" via the canonicalization step.
///
/// If the preferences change (and hence the speech rules to use change), or if the rule file changes,
///   `intent_from_mathml` will detect that and (re)load the proper rules.
///
/// A string is returned in call cases.
/// If there is an error, the speech string will indicate an error.
pub fn intent_from_mathml<'m>(mathml: Element, doc: Document<'m>) -> Result<Element<'m>> {
    let intent_tree = intent_rules(&INTENT_RULES, doc, mathml, "")?;
    doc.root().append_child(intent_tree);
    return Ok(intent_tree);
}

pub fn speak_mathml(mathml: Element, nav_node_id: &str) -> Result<String> {
    return speak_rules(&SPEECH_RULES, mathml, nav_node_id);
}

pub fn overview_mathml(mathml: Element, nav_node_id: &str) -> Result<String> {
    return speak_rules(&OVERVIEW_RULES, mathml, nav_node_id);
}


fn intent_rules<'m>(rules: &'static std::thread::LocalKey<RefCell<SpeechRules>>, doc: Document<'m>, mathml: Element, nav_node_id: &'m str) -> Result<Element<'m>> {
    rules.with(|rules| {
        rules.borrow_mut().read_files()?;
        let rules = rules.borrow();
        // debug!("intent_rules:\n{}", mml_to_string(mathml));
        let should_set_literal_intent = rules.pref_manager.borrow().pref_to_string("SpeechStyle").as_str() == "LiteralSpeak";
        let original_intent = mathml.attribute_value("intent");
        if should_set_literal_intent {
            if let Some(intent) = original_intent {
                let intent = if intent.contains('(') {intent.replace('(', ":literal(")} else {intent.to_string() + ":literal"};
                mathml.set_attribute_value("intent", &intent);
            } else {
                mathml.set_attribute_value("intent", ":literal");
            };
        }
        let mut rules_with_context = SpeechRulesWithContext::new(&rules, doc, nav_node_id);
        let intent =  rules_with_context.match_pattern::<Element<'m>>(mathml)
                    .chain_err(|| "Pattern match/replacement failure!")?;
        let answer = if name(intent) == "TEMP_NAME" {   // unneeded extra layer
            assert_eq!(intent.children().len(), 1);
            as_element(intent.children()[0])
        } else {
            intent
        };
        if should_set_literal_intent {
            if let Some(original_intent) = original_intent {
                mathml.set_attribute_value("intent", original_intent);
            } else {
                mathml.remove_attribute("intent");
            }
        }
        return Ok(answer);
    })
}

/// Speak the MathML
/// If 'nav_node_id' is not an empty string, then the element with that id will have [[...]] around it
fn speak_rules(rules: &'static std::thread::LocalKey<RefCell<SpeechRules>>, mathml: Element, nav_node_id: &str) -> Result<String> {
    return rules.with(|rules| {
        rules.borrow_mut().read_files()?;
        let rules = rules.borrow();
        // debug!("speak_rules:\n{}", mml_to_string(mathml));
        let new_package = Package::new();
        let mut rules_with_context = SpeechRulesWithContext::new(&rules, new_package.as_document(), nav_node_id);
        let speech_string = nestable_speak_rules(& mut rules_with_context, mathml, nav_node_id)?;
        return Ok( rules.pref_manager.borrow().get_tts()
            .merge_pauses(remove_optional_indicators(
                &speech_string.replace(CONCAT_STRING, "")
                                    .replace(CONCAT_INDICATOR, "")                            
                            )
            .trim_start().trim_end_matches([' ', ',', ';'])) );
    });

    fn nestable_speak_rules<'c, 's:'c, 'm:'c>(rules_with_context: &mut SpeechRulesWithContext<'c, 's, 'm>, mathml: Element<'c>, nav_node_id: &str) -> Result<String> {
        let mut speech_string = rules_with_context.match_pattern::<String>(mathml)
                    .chain_err(|| "Pattern match/replacement failure!")?;
        // debug!("speak_rules: nav_node_id={}, mathml id={}, speech_string='{}'", nav_node_id, mathml.attribute_value("id").unwrap_or_default(), &speech_string);
        // Note: [[...]] is added around a matching child, but if the "id" is on 'mathml', the whole string is used
        if !nav_node_id.is_empty() {
            // See https://github.com/NSoiffer/MathCAT/issues/174 for why we can just start the speech at the nav node
            let intent_attr = mathml.attribute_value("data-intent-property").unwrap_or_default();
            if let Some(start) = speech_string.find("[[") {
                match speech_string[start+2..].find("]]") {
                    None => bail!("Internal error: looking for '[[...]]' during navigation -- only found '[[' in '{}'", speech_string),
                    Some(end) => speech_string = speech_string[start+2..start+2+end].to_string(),
                }
            } else if !intent_attr.contains(":literal:") {
                // try again with LiteralSpeak -- some parts might have been elided in other SpeechStyles
                mathml.set_attribute_value("data-intent-property", (":literal:".to_string() + intent_attr).as_str());
                let speech = nestable_speak_rules(rules_with_context, mathml, nav_node_id);
                mathml.set_attribute_value("data-intent-property", intent_attr);
                return speech;
            } else {
                bail!(NAV_NODE_SPEECH_NOT_FOUND); //  NAV_NODE_SPEECH_NOT_FOUND is tested for later
            }
        }
        return Ok(speech_string);
    }
}

/// Converts its argument to a string that can be used in a debugging message.
pub fn yaml_to_type(yaml: &Yaml) -> String {
    return match yaml {
        Yaml::Real(v)=> format!("real='{v:#}'"),
        Yaml::Integer(v)=> format!("integer='{v:#}'"),
        Yaml::String(v)=> format!("string='{v:#}'"),
        Yaml::Boolean(v)=> format!("boolean='{v:#}'"),
        Yaml::Array(v)=> match v.len() {
            0 => "array with no entries".to_string(),
            1 => format!("array with the entry: {}", yaml_to_type(&v[0])),
            _ => format!("array with {} entries. First entry: {}", v.len(), yaml_to_type(&v[0])),
        }
        Yaml::Hash(h)=> {
            let first_pair = 
                if h.is_empty() {
                    "no pairs".to_string()
                } else {
                    let (key, val) = h.iter().next().unwrap();
                    format!("({}, {})", yaml_to_type(key), yaml_to_type(val))
                };
            format!("dictionary with {} pair{}. A pair: {}", h.len(), if h.len()==1 {""} else {"s"}, first_pair)
        }
        Yaml::Alias(_)=> "Alias".to_string(),
        Yaml::Null=> "Null".to_string(),
        Yaml::BadValue=> "BadValue".to_string(),       
    }
}

fn yaml_type_err(yaml: &Yaml, str: &str) -> String {
    return format!("Expected {}, found {}", str, yaml_to_type(yaml));
}

// fn yaml_key_err(dict: &Yaml, key: &str, yaml_type: &str) -> String {
//     if dict.as_hash().is_none() {
//        return format!("Expected dictionary with key '{}', found\n{}", key, yaml_to_string(dict, 1));
//     }
//     let str = &dict[key];
//     if str.is_badvalue() {
//         return format!("Did not find '{}' in\n{}", key,  yaml_to_string(dict, 1));
//     }
//     return format!("Type of '{}' is not a {}.\nIt is a {}. YAML value is\n{}", 
//             key, yaml_type, yaml_to_type(str), yaml_to_string(dict, 0));
// }

fn find_str<'a>(dict: &'a Yaml, key: &'a str) -> Option<&'a str> {
    return dict[key].as_str();
}

/// Returns the Yaml as a `Hash` or an error if it isn't.
pub fn as_hash_checked(value: &Yaml) -> Result<&Hash> {
    let result = value.as_hash();
    let result = result.ok_or_else(|| yaml_type_err(value, "hashmap"))?;
    return Ok( result );
}

/// Returns the Yaml as a `Vec` or an error if it isn't.
pub fn as_vec_checked(value: &Yaml) -> Result<&Vec<Yaml>> {
    let result = value.as_vec();
    let result = result.ok_or_else(|| yaml_type_err(value, "array"))?;
    return Ok( result );
}

/// Returns the Yaml as a `&str` or an error if it isn't.
pub fn as_str_checked(yaml: &Yaml) -> Result<&str> {
    return Ok( yaml.as_str().ok_or_else(|| yaml_type_err(yaml, "string"))? );
}


/// A bit of a hack to concatenate replacements (without a ' ').
/// The CONCAT_INDICATOR is added by a "ct:" (instead of 't:') in the speech rules
/// and checked for by the tts code.
pub const CONCAT_INDICATOR: &str = "\u{F8FE}";

// This is the pattern that needs to be matched (and deleted)
pub const CONCAT_STRING: &str = " \u{F8FE}";

// a similar hack to potentially delete (repetitive) optional replacements
// the OPTIONAL_INDICATOR is added by "ot:" before and after the optional string
const OPTIONAL_INDICATOR: &str  = "\u{F8FD}";
const OPTIONAL_INDICATOR_LEN: usize = OPTIONAL_INDICATOR.len();

pub fn remove_optional_indicators(str: &str) -> String {
    return str.replace(OPTIONAL_INDICATOR, "");
}

/// Given a string that should be Yaml, it calls `build_fn` with that string.
/// The build function/closure should process the Yaml as appropriate and capture any errors and write them to `std_err`.
/// The returned value should be a Vector containing the paths of all the files that were included.
pub fn compile_rule<F>(str: &str, mut build_fn: F) -> Result<Vec<PathBuf>> where
            F: FnMut(&Yaml) -> Result<Vec<PathBuf>> {
    let docs = YamlLoader::load_from_str(str);
    match docs {
        Err(e) => {
            bail!("Parse error!!: {}", e);
        },
        Ok(docs) => {
            if docs.len() != 1 {
                bail!("Didn't find rules!");
            }
            return build_fn(&docs[0]);
        }
    }
}

pub fn process_include<F>(current_file: &Path, new_file_name: &str, mut read_new_file: F) -> Result<Vec<PathBuf>>
                    where F: FnMut(&Path) -> Result<Vec<PathBuf>> {
    let parent_path = current_file.parent();
    if parent_path.is_none() {
        bail!("Internal error: {:?} is not a valid file name", current_file);
    }
    let mut new_file = match canonicalize_shim(parent_path.unwrap()) {
        Ok(path) => path,
        Err(e) => bail!("process_include: canonicalize failed for {} with message {}", parent_path.unwrap().display(), e.to_string()),
    };

    // the referenced file might be in a directory that hasn't been zipped up -- find the dir and call the unzip function
    for unzip_dir in new_file.ancestors() {
        if unzip_dir.ends_with("Rules") {
            break;      // nothing to unzip
        }
        if unzip_dir.ends_with("Languages") || unzip_dir.ends_with("Braille") {
            // get the subdir ...Rules/Braille/en/...
            // could have ...Rules/Braille/definitions.yaml, so 'next()' doesn't exist in this case, but the file wasn't zipped up
            if let Some(subdir) = new_file.strip_prefix(unzip_dir).unwrap().iter().next() {
                let default_lang = if unzip_dir.ends_with("Languages") {"en"} else {"UEB;"};
                PreferenceManager::unzip_files(unzip_dir, subdir.to_str().unwrap(), Some(default_lang)).unwrap_or_default();
            }
        }
    }
    new_file.push(new_file_name);
    info!("...processing include: {new_file_name}...");
    let new_file = match crate::shim_filesystem::canonicalize_shim(new_file.as_path()) {
        Ok(buf) => buf,
        Err(msg) => bail!("-include: constructed file name '{}' causes error '{}'",
                                 new_file.to_str().unwrap(), msg),
    };

    let mut included_files = read_new_file(new_file.as_path())?;
    let mut files_read = vec![new_file];
    files_read.append(&mut included_files);
    return Ok(files_read);
}

/// As the name says, TreeOrString is either a Tree (Element) or a String
/// It is used to share code during pattern matching
pub trait TreeOrString<'c, 'm:'c, T> {
    fn from_element(e: Element<'m>) -> Result<T>;
    fn from_string(s: String, doc: Document<'m>) -> Result<T>;
    fn replace_tts<'s:'c, 'r>(tts: &TTS, command: &TTSCommandRule, prefs: &PreferenceManager, rules_with_context: &'r mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<T>;
    fn replace<'s:'c, 'r>(ra: &ReplacementArray, rules_with_context: &'r mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<T>;
    fn replace_nodes<'s:'c, 'r>(rules: &'r mut SpeechRulesWithContext<'c, 's,'m>, nodes: Vec<Node<'c>>, mathml: Element<'c>) -> Result<T>;
    fn highlight_braille(braille: T, highlight_style: String) -> T;
    fn mark_nav_speech(speech: T) -> T;
}

impl<'c, 'm:'c> TreeOrString<'c, 'm, String> for String {
    fn from_element(_e: Element<'m>) -> Result<String> {
         bail!("from_element not allowed for strings");
    }

    fn from_string(s: String, _doc: Document<'m>) -> Result<String> {
        return Ok(s);
    }

    fn replace_tts<'s:'c, 'r>(tts: &TTS, command: &TTSCommandRule, prefs: &PreferenceManager, rules_with_context: &'r mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<String> {
        return tts.replace_string(command, prefs, rules_with_context, mathml);
    }

    fn replace<'s:'c, 'r>(ra: &ReplacementArray, rules_with_context: &'r mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<String> {
        return ra.replace_array_string(rules_with_context, mathml);
    }

    fn replace_nodes<'s:'c, 'r>(rules: &'r mut SpeechRulesWithContext<'c, 's,'m>, nodes: Vec<Node<'c>>, mathml: Element<'c>) -> Result<String> {
        return rules.replace_nodes_string(nodes, mathml);
    }

    fn highlight_braille(braille: String, highlight_style: String) -> String {
        return SpeechRulesWithContext::highlight_braille_string(braille, highlight_style);
    }

    fn mark_nav_speech(speech: String) -> String {
        return SpeechRulesWithContext::mark_nav_speech(speech);
    }
}

impl<'c, 'm:'c> TreeOrString<'c, 'm, Element<'m>> for Element<'m> {
    fn from_element(e: Element<'m>) -> Result<Element<'m>> {
         return Ok(e);
    }

    fn from_string(s: String, doc: Document<'m>) -> Result<Element<'m>> {
        // FIX: is 'mi' really ok?  Don't want to use TEMP_NAME because this name needs to move to the outside world
        let leaf = create_mathml_element(&doc, "mi");
        leaf.set_text(&s);
        return Ok(leaf);
}

    fn replace_tts<'s:'c, 'r>(_tts: &TTS, _command: &TTSCommandRule, _prefs: &PreferenceManager, _rules_with_context: &'r mut SpeechRulesWithContext<'c, 's,'m>, _mathml: Element<'c>) -> Result<Element<'m>> {
        bail!("Internal error: applying a TTS rule to a tree");
    }

    fn replace<'s:'c, 'r>(ra: &ReplacementArray, rules_with_context: &'r mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<Element<'m>> {
        return ra.replace_array_tree(rules_with_context, mathml);
    }

    fn replace_nodes<'s:'c, 'r>(rules: &'r mut SpeechRulesWithContext<'c, 's,'m>, nodes: Vec<Node<'c>>, mathml: Element<'c>) -> Result<Element<'m>> {
        return rules.replace_nodes_tree(nodes, mathml);
    }

    fn highlight_braille(_braille: Element<'c>, _highlight_style: String) -> Element<'m> {
        panic!("Internal error: highlight_braille called on a tree");
    }

    fn mark_nav_speech(_speech: Element<'c>) -> Element<'m> {
        panic!("Internal error: mark_nav_speech called on a tree");
    }
}

/// 'Replacement' is an enum that contains all the potential replacement types/structs
/// Hence there are fields 'Test' ("test:"), 'Text" ("t:"), "XPath", etc
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
enum Replacement {
    // Note: all of these are pointer types
    Text(String),
    XPath(MyXPath),
    Intent(Box<Intent>),
    Test(Box<TestArray>),
    TTS(Box<TTSCommandRule>),
    With(Box<With>),
    SetVariables(Box<SetVariables>),
    Insert(Box<InsertChildren>),
    Translate(TranslateExpression),
}

impl fmt::Display for Replacement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}",
            match self {
                Replacement::Test(c) => c.to_string(),
                Replacement::Text(t) => format!("t: \"{t}\""),
                Replacement::XPath(x) => x.to_string(),
                Replacement::Intent(i) => i.to_string(),
                Replacement::TTS(t) => t.to_string(),
                Replacement::With(w) => w.to_string(),
                Replacement::SetVariables(v) => v.to_string(),
                Replacement::Insert(ic) => ic.to_string(),
                Replacement::Translate(x) => x.to_string(),
            }
        );
    }
}

impl Replacement {   
    fn build(replacement: &Yaml) -> Result<Replacement> {
        // Replacement -- single key/value (see below for allowed values)
        let dictionary = replacement.as_hash();
        if dictionary.is_none() {
            bail!("  expected a key/value pair. Found {}.",  yaml_to_string(replacement, 0));
        };
        let dictionary = dictionary.unwrap();
        if dictionary.is_empty() { 
            bail!("No key/value pairs found for key 'replace'.\n\
                Suggestion: are the following lines indented properly?");
        }
        if dictionary.len() > 1 { 
            bail!("Should only be one key/value pair for the replacement.\n    \
                    Suggestion: are the following lines indented properly?\n    \
                    The key/value pairs found are\n{}", yaml_to_string(replacement, 2));
        }

        // get the single value
        let (key, value) = dictionary.iter().next().unwrap();
        let key = key.as_str().ok_or("replacement key(e.g, 't') is not a string")?;
        match key {
            "t" | "T" => {
                return Ok( Replacement::Text( as_str_checked(value)?.to_string() ) );
            },
            "ct" | "CT" => {
                return Ok( Replacement::Text( CONCAT_INDICATOR.to_string() + as_str_checked(value)? ) );
            },
            "ot" | "OT" => {
                return Ok( Replacement::Text( OPTIONAL_INDICATOR.to_string() + as_str_checked(value)? + OPTIONAL_INDICATOR ) );
            },
            "x" => {
                return Ok( Replacement::XPath( MyXPath::build(value)
                    .chain_err(|| "while trying to evaluate value of 'x:'")? ) );
            },
            "pause" | "rate" | "pitch" | "volume" | "audio" | "gender" | "voice" | "spell" | "SPELL" | "bookmark" | "pronounce" | "PRONOUNCE" => {
                return Ok( Replacement::TTS( TTS::build(&key.to_ascii_lowercase(), value)? ) );
            },
            "intent" => {
                return Ok( Replacement::Intent( Intent::build(value)? ) );
            },
            "test" => {
                return Ok( Replacement::Test( Box::new( TestArray::build(value)? ) ) );
            },
            "with" => {
                return Ok( Replacement::With( With::build(value)? ) );
            },
            "set_variables" => {
                return Ok( Replacement::SetVariables( SetVariables::build(value)? ) );
            },
            "insert" => {
                return Ok( Replacement::Insert( InsertChildren::build(value)? ) );
            },
            "translate" => {
                return Ok( Replacement::Translate( TranslateExpression::build(value)
                    .chain_err(|| "while trying to evaluate value of 'speak:'")? ) );
            },
            _ => {
                bail!("Unknown 'replace' command ({}) with value: {}", key, yaml_to_string(value, 0));
            }
        }
    }
}

// structure used when "insert:" is encountered in a rule
// the 'replacements' are inserted between each node in the 'xpath'
#[derive(Debug, Clone)]
struct InsertChildren {
    xpath: MyXPath,                     // the replacement nodes
    replacements: ReplacementArray,     // what is inserted between each node
}

impl fmt::Display for InsertChildren {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "InsertChildren:\n  nodes {}\n  replacements {}", self.xpath, &self.replacements);
    }
}

impl InsertChildren {
    fn build(insert: &Yaml) -> Result<Box<InsertChildren>> {
        // 'insert:' -- 'nodes': xxx 'replace': xxx
        if insert.as_hash().is_none() {
            bail!("")
        }
        let nodes = &insert["nodes"];
        if nodes.is_badvalue() { 
            bail!("Missing 'nodes' as part of 'insert'.\n    \
                  Suggestion: add 'nodes:' or if present, indent so it is contained in 'insert'");
        }
        let nodes = as_str_checked(nodes)?;
        let replace = &insert["replace"];
        if replace.is_badvalue() { 
            bail!("Missing 'replace' as part of 'insert'.\n    \
                  Suggestion: add 'replace:' or if present, indent so it is contained in 'insert'");
        }
        return Ok( Box::new( InsertChildren {
            xpath: MyXPath::new(nodes.to_string())?,
            replacements: ReplacementArray::build(replace).chain_err(|| "'replace:'")?,
        } ) );
    }
    
    // It would be most efficient to do an xpath eval, get the nodes (type: NodeSet) and then intersperse the node_replace()
    //   calls with replacements for the ReplacementArray parts. But that causes problems with the "pause: auto" calculation because
    //   the replacements are segmented (can't look to neighbors for the calculation there)
    // An alternative is to introduce another Replacement enum value, but that's a lot of complication for not that much
    //    gain (and Node's have contagious lifetimes)
    // The solution adopted is to find out the number of nodes and build up MyXPaths with each node selected (e.g, "*" => "*[3]")
    //    and put those nodes into a flat ReplacementArray and then do a standard replace on that.
    //    This is slower than the alternatives, but reuses a bunch of code and hence is less complicated.
    fn replace<'c, 's:'c, 'm: 'c, T:TreeOrString<'c, 'm, T>>(&self, rules_with_context: &mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<T> {
        let result = self.xpath.evaluate(&rules_with_context.context_stack.base, mathml)
                .chain_err(||format!("in '{}' replacing after pattern match", &self.xpath.rc.string) )?;
        match result {
            Value::Nodeset(nodes) => {
                if nodes.size() == 0 {
                    bail!("During replacement, no matching element found");
                };
                let nodes = nodes.document_order();
                let n_nodes = nodes.len();
                let mut expanded_result = Vec::with_capacity(n_nodes + (n_nodes+1)*self.replacements.replacements.len());
                expanded_result.push(
                    Replacement::XPath(
                        MyXPath::new(format!("{}[{}]", self.xpath.rc.string , 1))?
                    )
                );
                for i in 2..n_nodes+1 {
                    expanded_result.extend_from_slice(&self.replacements.replacements);
                    expanded_result.push(
                        Replacement::XPath(
                            MyXPath::new(format!("{}[{}]", self.xpath.rc.string , i))?
                        )
                    );
                }
                let replacements = ReplacementArray{ replacements: expanded_result };
                return replacements.replace(rules_with_context, mathml);
            },

            // FIX: should the options be errors???
            Value::String(t) => { return T::from_string(rules_with_context.replace_chars(&t, mathml)?, rules_with_context.doc); },
            Value::Number(num)  => { return T::from_string( num.to_string(), rules_with_context.doc ); },
            Value::Boolean(b)  => { return T::from_string( b.to_string(), rules_with_context.doc ); },          // FIX: is this right???
        }
        
    }    
}


lazy_static! {
    static ref ATTR_NAME_VALUE: Regex = Regex::new(
        // match name='value', where name is sort of an NCNAME (see CONCEPT_OR_LITERAL in infer_intent.rs)
        // The quotes can be either single or double quotes 
        r#"(?P<name>[^\s\u{0}-\u{40}\[\\\]^`\u{7B}-\u{BF}][^\s\u{0}-\u{2C}/:;<=>?@\[\\\]^`\u{7B}-\u{BF}]*)\s*=\s*('(?P<value>[^']+)'|"(?P<dqvalue>[^"]+)")"#
    ).unwrap();
}

// structure used when "intent:" is encountered in a rule
// the name is either a string or an xpath that needs evaluation. 99% of the time it is a string
#[derive(Debug, Clone)]
struct Intent {
    name: Option<String>,           // name of node
    xpath: Option<MyXPath>,         // alternative to directly using the string
    attrs: String,                  // optional attrs -- format "attr1='val1' [attr2='val2'...]"
    children: ReplacementArray,     // children of node
}

impl fmt::Display for Intent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = if self.name.is_some() {
            self.name.as_ref().unwrap().to_string()
        } else {
            self.xpath.as_ref().unwrap().to_string()
        };
        return write!(f, "intent: {}: {},  attrs='{}'>\n      children: {}",
                        if self.name.is_some() {"name"} else {"xpath-name"}, name,
                        self.attrs,
                        &self.children);
    }
}

impl Intent {
    fn build(yaml_dict: &Yaml) -> Result<Box<Intent>> {
        // 'intent:' -- 'name': xxx 'children': xxx
        if yaml_dict.as_hash().is_none() {
            bail!("Array found for contents of 'intent' -- should be dictionary with keys 'name' and 'children'")
        }
        let name = &yaml_dict["name"];
        let xpath_name = &yaml_dict["xpath-name"];
        if name.is_badvalue() && xpath_name.is_badvalue(){ 
            bail!("Missing 'name' or 'xpath-name' as part of 'intent'.\n    \
                  Suggestion: add 'name:' or if present, indent so it is contained in 'intent'");
        }
        let attrs = &yaml_dict["attrs"];
        let replace = &yaml_dict["children"];
        if replace.is_badvalue() {
            bail!("Missing 'children' as part of 'intent'.\n    \
                  Suggestion: add 'children:' or if present, indent so it is contained in 'intent'");
        }
        return Ok( Box::new( Intent {
            name: if name.is_badvalue() {None} else {Some(as_str_checked(name).chain_err(|| "'name'")?.to_string())},
            xpath: if xpath_name.is_badvalue() {None} else {Some(MyXPath::build(xpath_name).chain_err(|| "'intent'")?)},
            attrs: if attrs.is_badvalue() {"".to_string()} else {as_str_checked(attrs).chain_err(|| "'attrs'")?.to_string()},
            children: ReplacementArray::build(replace).chain_err(|| "'children:'")?,
        } ) );
    }
        
    fn replace<'c, 's:'c, 'm: 'c, T:TreeOrString<'c, 'm, T>>(&self, rules_with_context: &mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<T> {
        let result = self.children.replace::<Element<'m>>(rules_with_context, mathml)
                    .chain_err(||"replacing inside 'intent'")?;
        let mut result = lift_children(result);
        if name(result) != "TEMP_NAME" && name(result) != "Unknown" {
            // this case happens when you have an 'intent' replacement as a direct child of an 'intent' replacement
            let temp = create_mathml_element(&result.document(), "TEMP_NAME");
            temp.append_child(result);
            result = temp;
        }
        if let Some(intent_name) = &self.name {
            result.set_attribute_value(MATHML_FROM_NAME_ATTR, name(mathml));
            set_mathml_name(result, intent_name.as_str());
        }
        if let Some(my_xpath) = &self.xpath{    // self.xpath_name must be != None
            let xpath_value = my_xpath.evaluate(rules_with_context.get_context(), mathml)?;
            match xpath_value {
                Value::String(intent_name) => {
                    result.set_attribute_value(MATHML_FROM_NAME_ATTR, name(mathml));
                    set_mathml_name(result, intent_name.as_str())
                },
                _ => bail!("'xpath-name' value '{}' was not a string", &my_xpath),
            }
        }
        if self.name.is_none() && self.xpath.is_none() {
            panic!("Intent::replace: internal error -- neither 'name' nor 'xpath' is set");
        };
        
        for attr in mathml.attributes() {
            result.set_attribute_value(attr.name(), attr.value());
        }

        if !self.attrs.is_empty() {
            // debug!("MathML after children, before attr processing:\n{}", mml_to_string(mathml));
            // debug!("Result after children, before attr processing:\n{}", mml_to_string(result));
            // debug!("Intent::replace attrs = \"{}\"", &self.attrs);
            for cap in ATTR_NAME_VALUE.captures_iter(&self.attrs) {
                let matched_value = if cap["value"].is_empty() {&cap["dqvalue"]} else {&cap["value"]};
                let value_as_xpath = MyXPath::new(matched_value.to_string()).chain_err(||"attr value inside 'intent'")?;
                let value = value_as_xpath.evaluate(rules_with_context.get_context(), result)
                        .chain_err(||"attr xpath evaluation value inside 'intent'")?;
                let mut value = value.into_string();
                if &cap["name"] == INTENT_PROPERTY {
                    value = simplify_fixity_properties(&value);
                }
                // debug!("Intent::replace match\n  name={}\n  value={}\n  xpath value={}", &cap["name"], &cap["value"], &value);
                if &cap["name"] == INTENT_PROPERTY && value == ":" {
                    // should have been an empty string, so remove the attribute
                    result.remove_attribute(INTENT_PROPERTY);
                } else {
                    result.set_attribute_value(&cap["name"], &value);
                }
            };
        }

        // debug!("Result from 'intent:'\n{}", mml_to_string(result));
        return T::from_element(result);


        /// "lift" up the children any "TEMP_NAME" child -- could short circuit when only one child
        fn lift_children(result: Element) -> Element {
            // debug!("lift_children:\n{}", mml_to_string(result));
            // most likely there will be the same number of new children as result has, but there could be more
            let mut new_children = Vec::with_capacity(2*result.children().len());
            for child_of_element in result.children() {
                match child_of_element {
                    ChildOfElement::Element(child) => {
                        if name(child) == "TEMP_NAME" {
                            new_children.append(&mut child.children());  // almost always just one
                        } else {
                            new_children.push(child_of_element);
                        }
                    },
                    _ => new_children.push(child_of_element),      // text()
                }
            }
            result.replace_children(new_children);
            return result;
        }
    }    
}

// structure used when "with:" is encountered in a rule
// the variables are placed on (and later) popped of a variable stack before/after the replacement
#[derive(Debug, Clone)]
struct With {
    variables: VariableDefinitions,     // variables and values
    replacements: ReplacementArray,     // what to do with these vars
}

impl fmt::Display for With {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "with:\n      variables: {}\n      replace: {}", &self.variables, &self.replacements);
    }
}

impl With {
    fn build(vars_replacements: &Yaml) -> Result<Box<With>> {
        // 'with:' -- 'variables': xxx 'replace': xxx
        if vars_replacements.as_hash().is_none() {
            bail!("Array found for contents of 'with' -- should be dictionary with keys 'variables' and 'replace'")
        }
        let var_defs = &vars_replacements["variables"];
        if var_defs.is_badvalue() { 
            bail!("Missing 'variables' as part of 'with'.\n    \
                  Suggestion: add 'variables:' or if present, indent so it is contained in 'with'");
        }
        let replace = &vars_replacements["replace"];
        if replace.is_badvalue() { 
            bail!("Missing 'replace' as part of 'with'.\n    \
                  Suggestion: add 'replace:' or if present, indent so it is contained in 'with'");
        }
        return Ok( Box::new( With {
            variables: VariableDefinitions::build(var_defs).chain_err(|| "'variables'")?,
            replacements: ReplacementArray::build(replace).chain_err(|| "'replace:'")?,
        } ) );
    }
        
    fn replace<'c, 's:'c, 'm: 'c, T:TreeOrString<'c, 'm, T>>(&self, rules_with_context: &mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<T> {
        rules_with_context.context_stack.push(self.variables.clone(), mathml)?;
        let result = self.replacements.replace(rules_with_context, mathml)
                    .chain_err(||"replacing inside 'with'")?;
        rules_with_context.context_stack.pop();
        return Ok( result );
    }    
}

// structure used when "set_variables:" is encountered in a rule
// the variables are global and are placed in the base context and never popped off
#[derive(Debug, Clone)]
struct SetVariables {
    variables: VariableDefinitions,     // variables and values
}

impl fmt::Display for SetVariables {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "SetVariables: variables {}", &self.variables);
    }
}

impl SetVariables {
    fn build(vars: &Yaml) -> Result<Box<SetVariables>> {
        // 'set_variables:' -- 'variables': xxx (array)
        if vars.as_vec().is_none() {
            bail!("'set_variables' -- should be an array of variable name, xpath value");
        }
        return Ok( Box::new( SetVariables {
            variables: VariableDefinitions::build(vars).chain_err(|| "'set_variables'")?
        } ) );
    }
        
    fn replace<'c, 's:'c, 'm: 'c, T:TreeOrString<'c, 'm, T>>(&self, rules_with_context: &mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<T> {
        rules_with_context.context_stack.set_globals(self.variables.clone(), mathml)?;
        return T::from_string( "".to_string(), rules_with_context.doc );
    }    
}


/// Allow speech of an expression in the middle of a rule (used by "WhereAmI" for navigation)
#[derive(Debug, Clone)]
struct TranslateExpression {
    id: MyXPath,     // variables and values
}

impl fmt::Display for TranslateExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "speak: {}", &self.id);
    }
}
impl TranslateExpression {
    fn build(vars: &Yaml) -> Result<TranslateExpression> {
        // 'translate:' -- xpath (should evaluate to an id)
        return Ok( TranslateExpression { id: MyXPath::build(vars).chain_err(|| "'translate'")? } );
    }
        
    fn replace<'c, 's:'c, 'm:'c, T:TreeOrString<'c, 'm, T>>(&self, rules_with_context: &mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<T> {
        if self.id.rc.string.contains('@') {
            let xpath_value = self.id.evaluate(rules_with_context.get_context(), mathml)?;
            let id = match xpath_value {
                Value::String(s) => Some(s),
                Value::Nodeset(nodes) => {
                    if nodes.size() == 1 {
                        nodes.document_order_first().unwrap().attribute().map(|attr| attr.value().to_string())
                    } else {
                        None
                    }
                },
                _ => None,
            };
            match id {
                None => bail!("'translate' value '{}' is not a string or an attribute value (correct by using '@id'??):\n", self.id),
                Some(id) => {
                    let speech = speak_mathml(mathml, &id)?;
                    return T::from_string(speech, rules_with_context.doc);
                }
            }
        } else {
            return T::from_string(
                self.id.replace(rules_with_context, mathml).chain_err(||"'translate'")?,
                rules_with_context.doc
            );
        }  
    } 
}


/// An array of rule `Replacement`s (text, xpath, tts commands, etc)
#[derive(Debug, Clone)]
pub struct ReplacementArray {
    replacements: Vec<Replacement>
}

impl fmt::Display for ReplacementArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.pretty_print_replacements());
    }
}

impl ReplacementArray {
    /// Return an empty `ReplacementArray`
    pub fn build_empty() -> ReplacementArray {
        return ReplacementArray {
            replacements: vec![]
        }
    }

    /// Convert a Yaml input into a [`ReplacementArray`].
    /// Any errors are passed back out.
    pub fn build(replacements: &Yaml) -> Result<ReplacementArray> {
        // replacements is either a single replacement or an array of replacements
        let result= if replacements.is_array() {
            let replacements = replacements.as_vec().unwrap();
            replacements
                .iter()
                .enumerate()    // useful for errors
                .map(|(i, r)| Replacement::build(r)
                            .chain_err(|| format!("replacement #{} of {}", i+1, replacements.len())))
                .collect::<Result<Vec<Replacement>>>()?
        } else {
            vec![ Replacement::build(replacements)?]
        };

        return Ok( ReplacementArray{ replacements: result } );
    }

    /// Do all the replacements in `mathml` using `rules`.
    pub fn replace<'c, 's:'c, 'm:'c, T:TreeOrString<'c, 'm, T>>(&self, rules_with_context: &mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<T> {
        return T::replace(self, rules_with_context, mathml);
    }

    pub fn replace_array_string<'c, 's:'c, 'm:'c>(&self, rules_with_context: &mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<String> {
        // loop over the replacements and build up a vector of strings, excluding empty ones.
        // * eliminate any redundance
        // * add/replace auto-pauses
        // * join the remaining vector together
        let mut replacement_strings = Vec::with_capacity(self.replacements.len());   // probably conservative guess
        for replacement in self.replacements.iter() {
            let string: String = rules_with_context.replace(replacement, mathml)?;
            if !string.is_empty() {
                replacement_strings.push(string);
            }
        }

        if replacement_strings.is_empty() {
            return Ok( "".to_string() );
        }
        // delete an optional text that is repetitive
        // we do this by looking for the optional text marker, and if present, check for repetition at end of previous string
        // if repetitive, we delete the optional string
        // if not, we leave the markers because the repetition might happen several "levels" up
        // this could also be done in a final cleanup of the entire string (where we remove any markers),
        //   but the match is harder (rust regex lacks look behind pattern match) and it is less efficient
        // Note: we skip the first string since it can't be repetitive of something at this level
        for i in 1..replacement_strings.len()-1 {
            if let Some(bytes) = is_repetitive(&replacement_strings[i-1], &replacement_strings[i])  {
                replacement_strings[i] = bytes.to_string();
            } 
        }
                        
        for i in 0..replacement_strings.len() {
            if replacement_strings[i].contains(PAUSE_AUTO_STR) {
                let before = if i == 0 {""} else {&replacement_strings[i-1]};
                let after = if i+1 == replacement_strings.len() {""} else {&replacement_strings[i+1]};
                replacement_strings[i] = replacement_strings[i].replace(
                    PAUSE_AUTO_STR,
                    &rules_with_context.speech_rules.pref_manager.borrow().get_tts().compute_auto_pause(&rules_with_context.speech_rules.pref_manager.borrow(), before, after));
            }
        }

        // join the strings together with spaces in between
        // concatenation (removal of spaces) is saved for the top level because they otherwise are stripped at the wrong sometimes
        return Ok( replacement_strings.join(" ") );

        fn is_repetitive<'a>(prev: &str, optional: &'a str) -> Option<&'a str> {
            // OPTIONAL_INDICATOR surrounds the optional text
            // minor optimization -- lots of short strings and the OPTIONAL_INDICATOR takes a few bytes, so skip the check for those strings
            if optional.len() <=  2 * OPTIONAL_INDICATOR_LEN {
                return None;
            }
            
            // should be exactly one match -- ignore more than one for now
            match optional.find(OPTIONAL_INDICATOR) {
                None => return None,
                Some(start_index) => {
                    let optional_word_start_slice = &optional[start_index + OPTIONAL_INDICATOR_LEN..];
                    // now find the end
                    match optional_word_start_slice.find(OPTIONAL_INDICATOR) {
                        None => panic!("Internal error: missing end optional char -- text handling is corrupted!"),
                        Some(end_index) => {
                            let optional_word = &optional_word_start_slice[..end_index];
                            // debug!("check if '{}' is repetitive",  optional_word);
                            // debug!("   prev: '{}', next '{}'", prev, optional);
                            let prev = prev.trim_end().as_bytes();
                            if prev.len() > optional_word.len() &&
                               &prev[prev.len()-optional_word.len()..] == optional_word.as_bytes() {
                                return Some( optional_word_start_slice[optional_word.len() + OPTIONAL_INDICATOR_LEN..].trim_start() );
                            } else {
                                return None;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn replace_array_tree<'c, 's:'c, 'm:'c>(&self, rules_with_context: &mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<Element<'m>> {
        // shortcut for common case (don't build a new tree node)
        if self.replacements.len() == 1 {
            return rules_with_context.replace::<Element<'m>>(&self.replacements[0], mathml);
        }

        let new_element = create_mathml_element(&rules_with_context.doc, "Unknown");  // Hopefully set later (in Intent::Replace())
        let mut new_children = Vec::with_capacity(self.replacements.len());
        for child in self.replacements.iter() {
            let child = rules_with_context.replace::<Element<'m>>(child, mathml)?;
            new_children.push(ChildOfElement::Element(child));
        };
        new_element.append_children(new_children);
        return Ok(new_element);
    }


    /// Return true if there are no replacements.
    pub fn is_empty(&self) -> bool {
        return self.replacements.is_empty();
    }
    
    fn pretty_print_replacements(&self) -> String {
        let mut group_string = String::with_capacity(128);
        if self.replacements.len() == 1 {
            group_string += &format!("[{}]", self.replacements[0]);
        } else {
            group_string += &self.replacements.iter()
                    .map(|replacement| format!("\n  - {replacement}"))
                    .collect::<Vec<String>>()
                    .join("");
            group_string += "\n";
        }
        return group_string;
    }
}



// MyXPath is a wrapper around an 'XPath' that keeps around the original xpath expr (as a string) so it can be used in error reporting.
// Because we want to be able to clone them and XPath doesn't support clone(), this is a wrapper around an internal MyXPath.
// It supports the standard SpeechRule functionality of building and replacing.
#[derive(Debug)]
struct RCMyXPath {
    xpath: XPath,
    string: String,        // store for error reporting
}

#[derive(Debug, Clone)]
pub struct MyXPath {
    rc: Rc<RCMyXPath>        // rather than putting Rc around both 'xpath' and 'string', just use one and indirect to internal RCMyXPath
}


impl fmt::Display for MyXPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "\"{}\"", self.rc.string);
    }
}

// pub fn xpath_count() -> (usize, usize) {
//     return (XPATH_CACHE.with( |cache| cache.borrow().len()), unsafe{XPATH_CACHE_HITS} );
// }
thread_local!{
    static XPATH_CACHE: RefCell<HashMap<String, MyXPath>> = RefCell::new( HashMap::with_capacity(2047) );
}
// static mut XPATH_CACHE_HITS: usize = 0;

impl MyXPath {
    fn new(xpath: String) -> Result<MyXPath> {
        return XPATH_CACHE.with( |cache|  {
            let mut cache = cache.borrow_mut();
            return Ok(
                match cache.get(&xpath) {
                    Some(compiled_xpath) => {
                        // unsafe{ XPATH_CACHE_HITS += 1;};
                        compiled_xpath.clone()
                    },
                    None => {
                        let new_xpath = MyXPath {
                            rc: Rc::new( RCMyXPath {
                                xpath: MyXPath::compile_xpath(&xpath)?,
                                string: xpath.clone()
                            })};
                        cache.insert(xpath.clone(), new_xpath.clone());
                        new_xpath
                    },
                }
            )
        });
    }

    pub fn build(xpath: &Yaml) -> Result<MyXPath> {
        let xpath = match xpath {
            Yaml::String(s) => s.to_string(),
            Yaml::Integer(i) => i.to_string(),
            Yaml::Real(s) => s.to_string(),
            Yaml::Boolean(s) => s.to_string(),
            Yaml::Array(v) =>
                // array of strings -- concatenate them together
                v.iter()
                    .map(as_str_checked)
                    .collect::<Result<Vec<&str>>>()?
                    .join(" "),
            _ => bail!("Bad value when trying to create an xpath: {}", yaml_to_string(xpath, 1)),
        };
        return MyXPath::new(xpath);
    }

    fn compile_xpath(xpath: &str) -> Result<XPath> {
        let factory = Factory::new();
        let xpath_with_debug_info = MyXPath::add_debug_string_arg(xpath)?;
        let compiled_xpath = factory.build(&xpath_with_debug_info)
                        .chain_err(|| format!(
                            "Could not compile XPath for pattern:\n{}{}",
                            &xpath, more_details(xpath)))?;
        return match compiled_xpath {
            Some(xpath) => Ok(xpath),
            None => bail!("Problem compiling Xpath for pattern:\n{}{}",
                            &xpath, more_details(xpath)),
        };

        
        fn more_details(xpath: &str) -> String {
            // try to give a better error message by counting [], (), 's, and "s
            if xpath.is_empty() {
                return "xpath is empty string".to_string();
            }
            let as_bytes = xpath.trim().as_bytes();
            if as_bytes[0] == b'\'' && as_bytes[as_bytes.len()-1] != b'\'' {
                return "\nmissing \"'\"".to_string();
            }
            if (as_bytes[0] == b'"' && as_bytes[as_bytes.len()-1] != b'"') ||
               (as_bytes[0] != b'"' && as_bytes[as_bytes.len()-1] == b'"'){
                return "\nmissing '\"'".to_string();
            }

            let mut i_bytes = 0;      // keep track of # of bytes into string for error reporting
            let mut paren_count = 0;    // counter to make sure they are balanced
            let mut i_paren = 0;      // position of the outermost open paren
            let mut bracket_count = 0;
            let mut i_bracket = 0;
            for ch in xpath.chars() {
                if ch == '(' {
                    if paren_count == 0 {
                        i_paren = i_bytes;
                    }
                    paren_count += 1;
                } else if ch == '[' {
                    if bracket_count == 0 {
                        i_bracket = i_bytes;
                    }
                    bracket_count += 1;
                } else if ch == ')' {
                    if paren_count == 0 {
                        return format!("\nExtra ')' found after '{}'", &xpath[i_paren..i_bytes]);
                    }
                    paren_count -= 1;
                    if paren_count == 0 && bracket_count > 0 && i_bracket > i_paren {
                        return format!("\nUnclosed brackets found at '{}'", &xpath[i_paren..i_bytes]);
                    }
                } else if ch == ']' {
                    if bracket_count == 0 {
                        return format!("\nExtra ']' found after '{}'", &xpath[i_bracket..i_bytes]);
                    }
                    bracket_count -= 1;
                    if bracket_count == 0 && paren_count > 0 && i_paren > i_bracket {
                        return format!("\nUnclosed parens found at '{}'", &xpath[i_bracket..i_bytes]);
                    }
                }
                i_bytes += ch.len_utf8();
            }
            return "".to_string();
        }
    }

    /// Convert DEBUG(...) input to the internal function which is DEBUG(arg, arg_as_string)
    fn add_debug_string_arg(xpath: &str) -> Result<String> {
        // do a quick check to see if "DEBUG" is in the string -- this is the common case
        let debug_start = xpath.find("DEBUG(");
        if debug_start.is_none() {
            return Ok( xpath.to_string() );
        }

        let debug_start = debug_start.unwrap();
        let mut before_paren = xpath[..debug_start+5].to_string();   // includes "DEBUG"
        let chars = xpath[debug_start+5..].chars().collect::<Vec<char>>();     // begins at '('
        before_paren.push_str(&chars_add_debug_string_arg(&chars).chain_err(|| format!("In xpath='{xpath}'"))?);
        // debug!("add_debug_string_arg: {}", before_paren);
        return Ok(before_paren);

        fn chars_add_debug_string_arg(chars: &[char]) -> Result<String>  {
            // Find all the DEBUG(...) commands in 'xpath' and adds a string argument.
            // The DEBUG function that is used internally takes two arguments, the second one being a string version of the DEBUG arg.
            //   Being a string, any quotes need to be escaped, and DEBUGs inside of DEBUGs need more escaping.
            //   This is done via recursive calls to this function.
            assert_eq!(chars[0], '(', "{} does not start with ')'", chars.iter().collect::<String>());
            let mut count = 1;  // open/close count
            let mut i = 1;
            let mut inside_quote = false;
            while i < chars.len() {
                let ch = chars[i];
                match ch {
                    '\\' => {
                        if i+1 == chars.len() {
                            bail!("Syntax error in DEBUG: last char is escape char\n{}");
                        }
                        i += 1;
                    },
                    '\'' => inside_quote = !inside_quote,
                    '(' => {
                        if !inside_quote {
                            count += 1;
                        }
                        // FIX: it would be more efficient to spot "DEBUG" preceding this and recurse rather than matching the whole string and recursing
                    },
                    ')' => {
                        if !inside_quote {
                            count -= 1;
                            if count == 0 {
                                let arg = &chars[1..i].iter().collect::<String>();
                                let escaped_arg = arg.replace('"', "\\\"");
                                // DEBUG(...) may be inside 'arg' -- recurse
                                let processed_arg = MyXPath::add_debug_string_arg(arg)?;

                                // DEBUG(...) may be in the remainder of the string -- recurse
                                let processed_rest = MyXPath::add_debug_string_arg(&chars[i+1..].iter().collect::<String>())?;
                                return Ok( format!("({processed_arg}, \"{escaped_arg}\"){processed_rest}") );
                            }
                        }
                    },
                    _ => (),
                }
                i += 1;
            }
            bail!("Syntax error in DEBUG: didn't find matching closing paren\nDEBUG{}", chars.iter().collect::<String>());
        }
    }

    fn is_true(&self, context: &Context, mathml: Element) -> Result<bool> {
        // return true if there is no condition or if the condition evaluates to true
        return Ok(
            match self.evaluate(context, mathml)? {
                Value::Boolean(b) => b,
                Value::Nodeset(nodes) => nodes.size() > 0,
                _                      => false,      
            }
        )
    }

    pub fn replace<'c, 's:'c, 'm:'c, T:TreeOrString<'c, 'm, T>>(&self, rules_with_context: &mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<T> {
        if self.rc.string == "process-intent(.)" {
            return T::from_element( infer_intent(rules_with_context, mathml)? );
        }
        
        let result = self.evaluate(&rules_with_context.context_stack.base, mathml)
                .chain_err(|| format!("in '{}' replacing after pattern match", &self.rc.string) )?;
        let string = match result {
                Value::Nodeset(nodes) => {
                    if nodes.size() == 0 {
                        bail!("During replacement, no matching element found");
                    }
                    return rules_with_context.replace_nodes(nodes.document_order(), mathml);
                },
                Value::String(s) => s,
                Value::Number(num) => num.to_string(),
                Value::Boolean(b) => b.to_string(),          // FIX: is this right???
        };
        // Hack!: this test for input that starts with a '$' (defined variable), avoids a double evaluate;
        // We don't need NO_EVAL_QUOTE_CHAR here, but the more general solution of a quoted execute (- xq:) would avoid this hack
        let result = if self.rc.string.starts_with('$') {string} else {rules_with_context.replace_chars(&string, mathml)?};
        return T::from_string(result, rules_with_context.doc );
    }
    
    pub fn evaluate<'c>(&self, context: &Context<'c>, mathml: Element<'c>) -> Result<Value<'c>> {
        // debug!("evaluate: {}", self);
        let result = self.rc.xpath.evaluate(context, mathml);
        return match result {
            Ok(val) => Ok( val ),
            Err(e) => {
                // debug!("MyXPath::trying to evaluate:\n  '{}'\n caused the error\n'{}'", self, e.to_string().replace("OwnedPrefixedName { prefix: None, local_part:", "").replace(" }", ""));
                bail!( "{}\n\n",
                     // remove confusing parts of error message from xpath
                    e.to_string().replace("OwnedPrefixedName { prefix: None, local_part:", "").replace(" }", "") );
            }
        };
    }

    pub fn test_input<F>(self, f: F) -> bool where F: Fn(&str) -> bool {
        return f(self.rc.string.as_ref());
    }
}

// 'SpeechPattern' holds a single pattern.
// Some info is not needed beyond converting the Yaml to the SpeechPattern, but is useful for error reporting.
// The two main parts are the pattern to be matched and the replacements to do if there is a match.
// Any variables/prefs that are defined/set are also stored.
#[derive(Debug)]
struct SpeechPattern {
    pattern_name: String,
    tag_name: String,
    file_name: String,
    pattern: MyXPath,                     // the xpath expr to attempt to match
    match_uses_var_defs: bool,            // include var_defs in context for matching
    var_defs: VariableDefinitions,        // any variable definitions [can be and probably is an empty vector most of the time]
    replacements: ReplacementArray,       // the replacements in case there is a match
}

impl fmt::Display for SpeechPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "[name: {}, tag: {},\n  variables: {:?}, pattern: {},\n  replacement: {}]",
                self.pattern_name, self.tag_name, self.var_defs, self.pattern,
                self.replacements.pretty_print_replacements());
    }
}

impl SpeechPattern  {
    fn build(dict: &Yaml, file: &Path, rules: &mut SpeechRules) -> Result<Option<Vec<PathBuf>>> {
        // Rule::SpeechPattern
        //   build { "pattern_name", "tag_name", "pattern", "replacement" }
        // or recurse via include: file_name

        // debug!("\nbuild_speech_pattern: dict:\n{}", yaml_to_string(dict, 0));
        if let Some(include_file_name) = find_str(dict, "include") {
            let do_include_fn = |new_file: &Path| {
                rules.read_patterns(new_file)
            };

            return Ok( Some(process_include(file, include_file_name, do_include_fn)?) );
        }

        let pattern_name = find_str(dict, "name");

        // tag_named can be either a string (most common) or an array of strings
        let mut tag_names: Vec<&str> = Vec::new();
        match find_str(dict, "tag") {
            Some(str) => tag_names.push(str),
            None => {
                // check for array
                let tag_array  = &dict["tag"];
                tag_names = vec![];
                if tag_array.is_array() {
                    for (i, name) in tag_array.as_vec().unwrap().iter().enumerate() {
                        match as_str_checked(name) {
                            Err(e) => return Err(
                                e.chain_err(||
                                    format!("tag name '{}' is not a string in:\n{}",
                                        &yaml_to_string(&tag_array.as_vec().unwrap()[i], 0),
                                        &yaml_to_string(dict, 1)))
                            ),
                            Ok(str) => tag_names.push(str),
                        };
                    }
                } else {
                    bail!("Errors trying to find 'tag' in:\n{}", &yaml_to_string(dict, 1));
                }
            }
        }

        if pattern_name.is_none() {
            if dict.is_null() {
                bail!("Error trying to find 'name': empty value (two consecutive '-'s?");
            } else {
                bail!("Errors trying to find 'name' in:\n{}", &yaml_to_string(dict, 1));
            };
        };
        let pattern_name = pattern_name.unwrap().to_string();

        // FIX: add check to make sure tag_name is a valid MathML tag name
        if dict["match"].is_badvalue() {
            bail!("Did not find 'match' in\n{}", yaml_to_string(dict, 1));
        }
        if dict["replace"].is_badvalue() {
            bail!("Did not find 'replace' in\n{}", yaml_to_string(dict, 1));
        }
    
        // xpath's can't be cloned, so we need to do a 'build_xxx' for each tag name
        for tag_name in tag_names {
            let tag_name = tag_name.to_string();
            let pattern_xpath = MyXPath::build(&dict["match"])
                    .chain_err(|| {
                        format!("value for 'match' in rule ({}: {}):\n{}",
                                tag_name, pattern_name, yaml_to_string(dict, 1))
                    })?;
            let speech_pattern = 
                Box::new( SpeechPattern{
                    pattern_name: pattern_name.clone(),
                    tag_name: tag_name.clone(),
                    file_name: file.to_str().unwrap().to_string(),
                    match_uses_var_defs: dict["variables"].is_array() && pattern_xpath.rc.string.contains('$'),    // FIX: should look at var_defs for actual name
                    pattern: pattern_xpath,
                    var_defs: VariableDefinitions::build(&dict["variables"])
                        .chain_err(|| {
                            format!("value for 'variables' in rule ({}: {}):\n{}",
                                    tag_name, pattern_name, yaml_to_string(dict, 1))
                        })?, 
                    replacements: ReplacementArray::build(&dict["replace"])
                        .chain_err(|| {
                            format!("value for 'replace' in rule ({}: {}). Replacements:\n{}",
                                    tag_name, pattern_name, yaml_to_string(&dict["replace"], 1))
                    })?
                } );
            // get the array of rules for the tag name
            let rule_value = rules.rules.entry(tag_name).or_default();

            // if the name exists, replace it. Otherwise add the new rule
            match rule_value.iter().enumerate().find(|&pattern| pattern.1.pattern_name == speech_pattern.pattern_name) {
                None => rule_value.push(speech_pattern),
                Some((i, _old_pattern)) => {
                    let old_rule = &rule_value[i];
                    info!("\n\n***WARNING***: replacing {}/'{}' in {} with rule from {}\n",
                            old_rule.tag_name, old_rule.pattern_name, old_rule.file_name, speech_pattern.file_name);
                    rule_value[i] = speech_pattern;
                },
            }
        }

        return Ok(None);
    }

    fn is_match(&self, context: &Context, mathml: Element) -> Result<bool> {
        if self.tag_name != mathml.name().local_part() && self.tag_name != "*" && self.tag_name != "!*" {
            return Ok( false );
        }

        // debug!("\nis_match: pattern='{}'", self.pattern_name);
        // debug!("    pattern_expr {:?}", self.pattern);
        // debug!("is_match: mathml is\n{}", mml_to_string(mathml));
        return Ok(
            match self.pattern.evaluate(context, mathml)? {
                Value::Boolean(b)       => b,
                Value::Nodeset(nodes) => nodes.size() > 0,
                _                             => false,
            }
        );
    }
}


// 'Test' holds information used if the replacement is a "test:" clause.
// The condition is an xpath expr and the "else:" part is optional.

#[derive(Debug, Clone)]
struct TestArray {
    tests: Vec<Test>
}

impl fmt::Display for TestArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for test in &self.tests {
            writeln!(f, "{test}")?;
        }
        return Ok( () );
    }
}

impl TestArray {
    fn build(test: &Yaml) -> Result<TestArray> {
        // 'test:' for convenience takes either a dictionary with keys if/else_if/then/then_test/else/else_test or
        //      or an array of those values (there should be at most one else/else_test)

        // if 'test' is a dictionary ('Hash'), we convert it to an array with one entry and proceed
        let tests = if test.as_hash().is_some() {
            vec![test]
        } else if let Some(vec) = test.as_vec() {
            vec.iter().collect()
        } else {
            bail!("Value for 'test:' is neither a dictionary or an array.")
        };

        // each entry in 'tests' should be a dictionary with keys if/then/then_test/else/else_test
        // a valid entry is one of:
        //   if:/else_if:, then:/then_test: and optional else:/else_test:
        //   else:/else_test: -- if this case, it should be the last entry in 'tests'
        // 'if:' should only be the first entry in the array; 'else_if' should never be the first entry. Otherwise, they are the same
        let mut test_array = vec![];
        for test in tests {
            if test.as_hash().is_none() {
                bail!("Value for array entry in 'test:' must be a dictionary/contain keys");
            }
            let if_part = &test[if test_array.is_empty() {"if"} else {"else_if"}];
            if !if_part.is_badvalue() {
                // first case: if:, then:, optional else:
                let condition = Some( MyXPath::build(if_part)? );
                let then_part = TestOrReplacements::build(test, "then", "then_test", true)?;
                let else_part = TestOrReplacements::build(test, "else", "else_test", false)?;
                let n_keys = if else_part.is_none() {2} else {3};
                if test.as_hash().unwrap().len() > n_keys {
                    bail!("A key other than 'if', 'else_if', 'then', 'then_test', 'else', or 'else_test' was found in the 'then' clause of 'test'");
                };
                test_array.push(
                    Test { condition, then_part, else_part }
                );
            } else {
                // second case: should be else/else_test
                let else_part = TestOrReplacements::build(test, "else", "else_test", true)?;
                if test.as_hash().unwrap().len() > 1 {
                    bail!("A key other than 'if', 'else_if', 'then', 'then_test', 'else', or 'else_test' was found the 'else' clause of 'test'");
                };
                test_array.push(
                    Test { condition: None, then_part: None, else_part }
                );
                
                // there shouldn't be any trailing tests
                if test_array.len() < test.as_hash().unwrap().len() {
                    bail!("'else'/'else_test' key is not last key in 'test:'");
                }
            }
        };

        if test_array.is_empty() {
            bail!("No entries for 'test:'");
        }

        return Ok( TestArray { tests: test_array } );
    }

    fn replace<'c, 's:'c, 'm:'c, T:TreeOrString<'c, 'm, T>>(&self, rules_with_context: &mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<T> {
        for test in &self.tests {
            if test.is_true(&rules_with_context.context_stack.base, mathml)? {
                assert!(test.then_part.is_some());
                return test.then_part.as_ref().unwrap().replace(rules_with_context, mathml);
            } else if let Some(else_part) = test.else_part.as_ref() {
                return else_part.replace(rules_with_context, mathml);
            }
        }
        return T::from_string("".to_string(), rules_with_context.doc);
    }
}

#[derive(Debug, Clone)]
// Used to hold then/then_test and also else/else_test -- only one of these can be present at a time
enum TestOrReplacements {
    Replacements(ReplacementArray),     // replacements to use when a test is true
    Test(TestArray),                    // the array of if/then/else tests
}

impl fmt::Display for TestOrReplacements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let TestOrReplacements::Test(_) = self {
            write!(f, "  _test")?;
        }
        write!(f, ":")?;
        return match self {
            TestOrReplacements::Test(t) => write!(f, "{t}"),
            TestOrReplacements::Replacements(r) => write!(f, "{r}"),
        };
    }
}

impl TestOrReplacements {
    fn build(test: &Yaml, replace_key: &str, test_key: &str, key_required: bool) -> Result<Option<TestOrReplacements>> {
        let part = &test[replace_key];
        let test_part = &test[test_key];
        if !part.is_badvalue() && !test_part.is_badvalue() { 
            bail!(format!("Only one of '{}' or '{}' is allowed as part of 'test'.\n{}\n    \
                  Suggestion: delete one or adjust indentation",
                    replace_key, test_key, yaml_to_string(test, 2)));
        }
        if part.is_badvalue() && test_part.is_badvalue() {
            if key_required {
                bail!(format!("Missing one of '{}'/'{}:' as part of 'test:'\n{}\n   \
                    Suggestion: add the missing key or indent so it is contained in 'test'",
                    replace_key, test_key, yaml_to_string(test, 2)))
            } else {
                return Ok( None );
            }
        }
        // at this point, we have only one of the two options
        if test_part.is_badvalue() {
            return Ok( Some( TestOrReplacements::Replacements( ReplacementArray::build(part)? ) ) );
        } else {
            return Ok( Some( TestOrReplacements::Test( TestArray::build(test_part)? ) ) );
        }
    }

    fn replace<'c, 's:'c, 'm:'c, T:TreeOrString<'c, 'm, T>>(&self, rules_with_context: &mut SpeechRulesWithContext<'c, 's,'m>, mathml: Element<'c>) -> Result<T> {
        return match self {
            TestOrReplacements::Replacements(r) => r.replace(rules_with_context, mathml),
            TestOrReplacements::Test(t) => t.replace(rules_with_context, mathml),
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    condition: Option<MyXPath>,
    then_part: Option<TestOrReplacements>,
    else_part: Option<TestOrReplacements>,
}
impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "test: [ ")?;
        if let Some(if_part) = &self.condition {
            write!(f, " if: '{if_part}'")?;
        }
        if let Some(then_part) = &self.then_part {
            write!(f, " then{then_part}")?;
        }
        if let Some(else_part) = &self.else_part {
            write!(f, " else{else_part}")?;
        }
        return write!(f, "]");
    }
}

impl Test {
    fn is_true(&self, context: &Context, mathml: Element) -> Result<bool> {
        return match self.condition.as_ref() {
            None => Ok( false ),     // trivially false -- want to do else part
            Some(condition) => condition.is_true(context, mathml)
                                .chain_err(|| "Failure in conditional test"),
        }
    }
}

// Used for speech rules with "variables: ..."
#[derive(Debug, Clone)]
struct VariableDefinition {
    name: String,     // name of variable
    value: MyXPath,   // xpath value, typically a constant like "true" or "0", but could be "*/*[1]" to store some nodes   
}

impl fmt::Display for VariableDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "[name: {}={}]", self.name, self.value);
    }   
}

// Used for speech rules with "variables: ..."
#[derive(Debug)]
struct VariableValue<'v> {
    name: String,       // name of variable
    value: Option<Value<'v>>,   // xpath value, typically a constant like "true" or "0", but could be "*/*[1]" to store some nodes   
}

impl fmt::Display for VariableValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match &self.value {
            None => "unset".to_string(),
            Some(val) => format!("{val:?}")
        };
        return write!(f, "[name: {}, value: {}]", self.name, value);
    }   
}

impl VariableDefinition {
    fn build(name_value_def: &Yaml) -> Result<VariableDefinition> {
        match name_value_def.as_hash() {
            Some(map) => {
                if map.len() != 1 {
                    bail!("definition is not a key/value pair. Found {}",
                            yaml_to_string(name_value_def, 1) );
                }
                let (name, value) = map.iter().next().unwrap();
                let name = as_str_checked( name)
                    .chain_err(|| format!( "definition name is not a string: {}",
                            yaml_to_string(name, 1) ))?.to_string();
                match value {
                    Yaml::Boolean(_) | Yaml::String(_)  | Yaml::Integer(_) | Yaml::Real(_) => (),
                    _ => bail!("definition value is not a string, boolean, or number. Found {}",
                            yaml_to_string(value, 1) )
                };
                return Ok(
                    VariableDefinition{
                        name,
                        value: MyXPath::build(value)?
                    }
                );
            },
            None => bail!("definition is not a key/value pair. Found {}",
                            yaml_to_string(name_value_def, 1) )
        }
    }
}


#[derive(Debug, Clone)]
struct VariableDefinitions {
    defs: Vec<VariableDefinition>
}

impl fmt::Display for VariableDefinitions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for def in &self.defs {
            write!(f, "{def},")?;
        }
        return Ok( () );
    }
}

struct VariableValues<'v> {
    defs: Vec<VariableValue<'v>>
}

impl fmt::Display for VariableValues<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for value in &self.defs {
            write!(f, "{value}")?;
        }
        return writeln!(f);
    }
}

impl VariableDefinitions {
    fn new(len: usize) -> VariableDefinitions {
        return VariableDefinitions{ defs: Vec::with_capacity(len) };
    }

    fn build(defs: &Yaml) -> Result<VariableDefinitions> {
        if defs.is_badvalue() {
            return Ok( VariableDefinitions::new(0) );
        };
        if defs.is_array() {
            let defs = defs.as_vec().unwrap();
            let mut definitions = VariableDefinitions::new(defs.len());
            for def in defs {
                let variable_def = VariableDefinition::build(def)
                        .chain_err(|| "definition of 'variables'")?;
                definitions.push( variable_def);
            };
            return Ok (definitions );
        }
        bail!( "'variables' is not an array of {{name: xpath-value}} definitions. Found {}'",
                yaml_to_string(defs, 1) );
    }

    fn push(&mut self, var_def: VariableDefinition) {
        self.defs.push(var_def);
    }

    fn len(&self) -> usize {
        return self.defs.len();
    }
}

struct ContextStack<'c> {
    // Note: values are generated by calling value_of on an Evaluation -- that makes the two lifetimes the same
    old_values: Vec<VariableValues<'c>>,   // store old values so they can be set on pop 
    base: Context<'c>                      // initial context -- contains all the function defs and pref variables
}

impl fmt::Display for ContextStack<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, " {} old_values", self.old_values.len())?;
        for values in &self.old_values {
            writeln!(f, "  {values}")?;
        }
        return writeln!(f);
    }
}

impl<'c, 'r> ContextStack<'c> {
    fn new<'a,>(pref_manager: &'a PreferenceManager) -> ContextStack<'c> {
        let prefs = pref_manager.merge_prefs();
        let mut context_stack = ContextStack {
            base: ContextStack::base_context(prefs),
            old_values: Vec::with_capacity(31)      // should avoid allocations
        };
        // FIX: the list of variables to set should come from definitions.yaml
        // These can't be set on the <math> tag because of the "translate" command which starts speech at an 'id'
        context_stack.base.set_variable("MatchingPause", Value::Boolean(false));
        context_stack.base.set_variable("IsColumnSilent", Value::Boolean(false));


        return context_stack;
    }

    fn base_context(var_defs: PreferenceHashMap) -> Context<'c> {
        let mut context  = Context::new();
        context.set_namespace("m", "http://www.w3.org/1998/Math/MathML");
        crate::xpath_functions::add_builtin_functions(&mut context);
        for (key, value) in var_defs {
            context.set_variable(key.as_str(), yaml_to_value(&value));
            // if let Some(str_value) = value.as_str() {
            //     if str_value != "Auto" {
            //         debug!("Set {}='{}'", key.as_str(), str_value);
            //     }
            // }
        };
        return context;
    }

    fn set_globals(&'r mut self, new_vars: VariableDefinitions, mathml: Element<'c>) -> Result<()> {
        // for each var/value pair, evaluate the value and add the var/value to the base context
        for def in &new_vars.defs {
            // set the new value
            let new_value = match def.value.evaluate(&self.base, mathml) {
                Ok(val) => val,
                Err(_) => bail!(format!("Can't evaluate variable def for {}", def)),
            };
            let qname = QName::new(def.name.as_str());
            self.base.set_variable(qname, new_value);
        }
        return Ok( () );
    }

    fn push(&'r mut self, new_vars: VariableDefinitions, mathml: Element<'c>) -> Result<()> {
        // store the old value and set the new one 
        let mut old_values = VariableValues {defs: Vec::with_capacity(new_vars.defs.len()) };
        let evaluation = Evaluation::new(&self.base, Node::Element(mathml));
        for def in &new_vars.defs {
            // get the old value (might not be defined)
            let qname = QName::new(def.name.as_str());
            let old_value = evaluation.value_of(qname).cloned();
            old_values.defs.push( VariableValue{ name: def.name.clone(), value: old_value} );
        }

        // use a second loop because of borrow problem with self.base and 'evaluation'
        for def in &new_vars.defs {
            // set the new value
            let new_value = match def.value.evaluate(&self.base, mathml) {
                Ok(val) => val,
                Err(_) => Value::Nodeset(sxd_xpath::nodeset::Nodeset::new()),
            };
            let qname = QName::new(def.name.as_str());
            self.base.set_variable(qname, new_value);
        }
        self.old_values.push(old_values);
        return Ok( () );
    }

    fn pop(&mut self) {
        const MISSING_VALUE: &str = "-- unset value --";     // can't remove a variable from context, so use this value
        let old_values = self.old_values.pop().unwrap();
        for variable in old_values.defs {
            let qname = QName::new(&variable.name);
            let old_value = match variable.value {
                None => Value::String(MISSING_VALUE.to_string()),
                Some(val) => val,
            };
            self.base.set_variable(qname, old_value);
        }
    }
}


fn yaml_to_value<'b>(yaml: &Yaml) -> Value<'b> {
    return match yaml {
        Yaml::String(s) => Value::String(s.clone()),
        Yaml::Boolean(b)  => Value::Boolean(*b),
        Yaml::Integer(i)   => Value::Number(*i as f64),
        Yaml::Real(s)   => Value::Number(s.parse::<f64>().unwrap()),
        _  => {
            error!("yaml_to_value: illegal type found in Yaml value: {}", yaml_to_string(yaml, 1));
            Value::String("".to_string())
        },
    }
}


// Information for matching a Unicode char (defined in unicode.yaml) and building its replacement
struct UnicodeDef {
    ch: u32,
    speech: ReplacementArray
}

impl  fmt::Display for UnicodeDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "UnicodeDef{{ch: {}, speech: {:?}}}", self.ch, self.speech);
    }
}

impl UnicodeDef {
    fn build(unicode_def: &Yaml, file_name: &Path, speech_rules: &SpeechRules, use_short: bool) -> Result<Option<Vec<PathBuf>>> {
        if let Some(include_file_name) = find_str(unicode_def, "include") {
            let do_include_fn = |new_file: &Path| {
                speech_rules.read_unicode(Some(new_file.to_path_buf()), use_short)
            };
            return Ok( Some(process_include(file_name, include_file_name, do_include_fn)?) );
        }
        // key: char, value is replacement or array of replacements
        let dictionary = unicode_def.as_hash();
        if dictionary.is_none() {
            bail!("Expected a unicode definition (e.g, '+':[t: \"plus\"]'), found {}", yaml_to_string(unicode_def, 0));
        }

        let dictionary = dictionary.unwrap();
        if dictionary.len() != 1 {
            bail!("Expected a unicode definition (e.g, '+':[t: \"plus\"]'), found {}", yaml_to_string(unicode_def, 0));
        }

        let (ch, replacements) = dictionary.iter().next().ok_or_else(||  format!("Expected a unicode definition (e.g, '+':[t: \"plus\"]'), found {}", yaml_to_string(unicode_def, 0)))?;
        let mut unicode_table = if use_short {
            speech_rules.unicode_short.borrow_mut()
        } else {
            speech_rules.unicode_full.borrow_mut()
        };
        if let Some(str) = ch.as_str() {
            if str.is_empty() {
                bail!("Empty character definition. Replacement is {}", replacements.as_str().unwrap());
            }
            let mut chars = str.chars();
            let first_ch = chars.next().unwrap();       // non-empty string, so a char exists
            if chars.next().is_some() {                       // more than one char
                if str.contains('-')  {
                    return process_range(str, replacements, unicode_table);
                } else if first_ch != '0' {     // exclude 0xDDDD
                    for ch in str.chars() {     // restart the iterator
                        let ch_as_str = ch.to_string();
                        if unicode_table.insert(ch as u32, ReplacementArray::build(&substitute_ch(replacements, &ch_as_str))
                                            .chain_err(|| format!("In definition of char: '{str}'"))?.replacements).is_some() {
                            error!("*** Character '{}' (0x{:X}) is repeated", ch, ch as u32);
                        }
                    }
                    return Ok(None);
                }
            }
        }

        let ch = UnicodeDef::get_unicode_char(ch)?;
        if unicode_table.insert(ch, ReplacementArray::build(replacements)
                                        .chain_err(|| format!("In definition of char: '{}' (0x{})",
                                                                        char::from_u32(ch).unwrap(), ch))?.replacements).is_some() {
            error!("*** Character '{}' (0x{:X}) is repeated", char::from_u32(ch).unwrap(), ch);
        }
        return Ok(None);

        fn process_range(def_range: &str, replacements: &Yaml, mut unicode_table: RefMut<HashMap<u32,Vec<Replacement>>>) -> Result<Option<Vec<PathBuf>>> {
            // should be a character range (e.g., "A-Z")
            // iterate over that range and also substitute the char for '.' in the 
            let mut range = def_range.split('-');
            let first = range.next().unwrap().chars().next().unwrap() as u32;
            let last = range.next().unwrap().chars().next().unwrap() as u32;
            if range.next().is_some() {
                bail!("Character range definition has more than one '-': '{}'", def_range);
            }

            for ch in first..last+1 {
                let ch_as_str = char::from_u32(ch).unwrap().to_string();
                unicode_table.insert(ch, ReplacementArray::build(&substitute_ch(replacements, &ch_as_str))
                                        .chain_err(|| format!("In definition of char: '{def_range}'"))?.replacements);
            };

            return Ok(None)
        }

        fn substitute_ch(yaml: &Yaml, ch: &str) -> Yaml {
            return match yaml {
                Yaml::Array(ref v) => {
                    Yaml::Array(
                        v.iter()
                         .map(|e| substitute_ch(e, ch))
                         .collect::<Vec<Yaml>>()
                    )
                },
                Yaml::Hash(ref h) => {
                    Yaml::Hash(
                        h.iter()
                         .map(|(key,val)| (key.clone(), substitute_ch(val, ch)) )
                         .collect::<Hash>()
                    )
                },
                Yaml::String(s) => Yaml::String( s.replace('.', ch) ),
                _ => yaml.clone(),
            }
        }
    }
    
    fn get_unicode_char(ch: &Yaml) -> Result<u32> {
        // either "a" or 0x1234 (number)
        if let Some(ch) = ch.as_str() {
            let mut ch_iter = ch.chars();
            let unicode_ch = ch_iter.next();
            if unicode_ch.is_none() || ch_iter.next().is_some() {
                bail!("Wanted unicode char, found string '{}')", ch);
            };
            return Ok( unicode_ch.unwrap() as u32 );
        }
    
        if let Some(num) = ch.as_i64() {
            return Ok( num as u32 );
        }
        bail!("Unicode character '{}' can't be converted to an code point", yaml_to_string(ch, 0));
    }    
}

// Fix: there should be a cache so subsequent library calls don't have to read in the same speech rules
//   likely a cache of size 1 is fine
// Fix: all statics should be gathered together into one structure that is a Mutex
//   for each library call, we should grab a lock on the Mutex in case others try to call
//   at the same time.
//   If this turns out to be something that others actually do, then a cache > 1 would be good

 type RuleTable = HashMap<String, Vec<Box<SpeechPattern>>>;
 type UnicodeTable = Rc<RefCell<HashMap<u32,Vec<Replacement>>>>;
 type FilesAndTimesShared = Rc<RefCell<FilesAndTimes>>;

 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 pub enum RulesFor {
     Intent,
     Speech,
     OverView,
     Navigation,
     Braille,
 }

 impl fmt::Display for RulesFor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            RulesFor::Intent => "Intent",
            RulesFor::Speech => "Speech",
            RulesFor::OverView => "OverView",
            RulesFor::Navigation => "Navigation",
            RulesFor::Braille => "Braille",
        };
       return write!(f, "{name}");
    }
 }

 
#[derive(Debug, Clone)]
pub struct FileAndTime {
    file: PathBuf,
    time: SystemTime,
}

impl FileAndTime {
    fn new(file: PathBuf) -> FileAndTime {
        return FileAndTime {
            file,
            time: SystemTime::UNIX_EPOCH,
        }
    }

    // used for debugging preference settings
    pub fn debug_get_file(&self) -> Option<&str> {
        return self.file.to_str();
    }

    pub fn new_with_time(file: PathBuf) -> FileAndTime {
        return FileAndTime {
            time: FileAndTime::get_metadata(&file),
            file,
        }
    }

    pub fn is_up_to_date(&self) -> bool {
        let file_mod_time = FileAndTime::get_metadata(&self.file);
        return self.time >= file_mod_time;
    }

    fn get_metadata(path: &Path) -> SystemTime {
        use std::fs;
        if !cfg!(target_family = "wasm") {
            let metadata = fs::metadata(path);
            if let Ok(metadata) = metadata {
                if let Ok(mod_time) = metadata.modified() {
                    return mod_time;
                }
            }
        }
        return SystemTime::UNIX_EPOCH
    }

}
#[derive(Debug, Default)]
pub struct FilesAndTimes {
    // ft[0] is the main file -- other files are included by it (or recursively)
    // We could be a little smarter about invalidation by tracking what file is the parent (including file),
    // but it seems more complicated than it is worth
    ft: Vec<FileAndTime>
}

impl FilesAndTimes {
    pub fn new(start_path: PathBuf) -> FilesAndTimes {
        let mut ft = Vec::with_capacity(8);
        ft.push( FileAndTime::new(start_path) );
        return FilesAndTimes{ ft };
    }

    /// Returns true if the main file matches the corresponding preference location and files' times are all current
    pub fn is_file_up_to_date(&self, pref_path: &Path, should_ignore_file_time: bool) -> bool {

        // if the time isn't set or the path is different from the preference (which might have changed), return false
        if self.ft.is_empty() || self.as_path() != pref_path {
            return false;
        }
        if should_ignore_file_time || cfg!(target_family = "wasm") {
            return true;
        }
        if  self.ft[0].time == SystemTime::UNIX_EPOCH {
            return false;
        }


        // check the time stamp on the included files -- if the head file hasn't changed, the the paths for the included files will the same
        for file in &self.ft {
            if !file.is_up_to_date() {
                return false;
            }
        }
        return true;
    }

    fn set_files_and_times(&mut self, new_files: Vec<PathBuf>)  {
        self.ft.clear();
        for path in new_files {
            let time = FileAndTime::get_metadata(&path);      // do before move below
            self.ft.push( FileAndTime{ file: path, time })
        }
    }

    pub fn as_path(&self) -> &Path {
        assert!(!self.ft.is_empty());
        return &self.ft[0].file;
    }

    pub fn paths(&self) -> Vec<PathBuf> {
        return self.ft.iter().map(|ft| ft.file.clone()).collect::<Vec<PathBuf>>();
    }

}


/// `SpeechRulesWithContext` encapsulates a named group of speech rules (e.g, "ClearSpeak")
/// along with the preferences to be used for speech.
// Note: if we can't read the files, an error message is stored in the structure and needs to be checked.
// I tried using Result<SpeechRules>, but it was a mess with all the unwrapping.
// Important: the code needs to be careful to check this at the top level calls
pub struct SpeechRules {
    error: String,
    name: RulesFor,
    pub pref_manager: Rc<RefCell<PreferenceManager>>,
    rules: RuleTable,                              // the speech rules used (partitioned into MathML tags in hashmap, then linearly searched)
    rule_files: FilesAndTimes,                     // files that were read
    translate_single_chars_only: bool,             // strings like "half" don't want 'a's translated, but braille does
    unicode_short: UnicodeTable,                   // the short list of rules used for Unicode characters
    unicode_short_files: FilesAndTimesShared,     // files that were read
    unicode_full:  UnicodeTable,                   // the long remaining rules used for Unicode characters
    unicode_full_files: FilesAndTimesShared,      // files that were read
    definitions_files: FilesAndTimesShared,       // files that were read
}

impl fmt::Display for SpeechRules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "SpeechRules '{}'\n{})", self.name, self.pref_manager.borrow())?;
        let mut rules_vec: Vec<(&String, &Vec<Box<SpeechPattern>>)> = self.rules.iter().collect();
        rules_vec.sort_by(|(tag_name1, _), (tag_name2, _)| tag_name1.cmp(tag_name2));
        for (tag_name, rules) in rules_vec {
            writeln!(f, "   {}: #patterns {}", tag_name, rules.len())?;
        };
        return writeln!(f, "   {}+{} unicode entries", &self.unicode_short.borrow().len(), &self.unicode_full.borrow().len());
    }
}


/// `SpeechRulesWithContext` encapsulates a named group of speech rules (e.g, "ClearSpeak")
/// along with the preferences to be used for speech.
/// Because speech rules can define variables, there is also a context that is carried with them
pub struct SpeechRulesWithContext<'c, 's:'c, 'm:'c> {
    speech_rules: &'s SpeechRules,
    context_stack: ContextStack<'c>,   // current value of (context) variables
    doc: Document<'m>,
    nav_node_id: &'m str,
    pub inside_spell: bool,     // hack to allow 'spell' to avoid infinite loop (see 'spell' implementation in tts.rs)
    pub translate_count: usize, // hack to avoid 'translate' infinite loop (see 'spell' implementation in tts.rs)
}

impl<'c, 's:'c, 'm:'c> fmt::Display for SpeechRulesWithContext<'c, 's,'m> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "SpeechRulesWithContext \n{})", self.speech_rules)?;
        return writeln!(f, "   {} context entries, nav node id '{}'", &self.context_stack, self.nav_node_id);
    }
}

thread_local!{
    /// SPEECH_UNICODE_SHORT is shared among several rules, so "RC" is used
    static SPEECH_UNICODE_SHORT: UnicodeTable =
        Rc::new( RefCell::new( HashMap::with_capacity(500) ) );
        
    /// SPEECH_UNICODE_FULL is shared among several rules, so "RC" is used
    static SPEECH_UNICODE_FULL: UnicodeTable =
        Rc::new( RefCell::new( HashMap::with_capacity(6500) ) );
        
    /// BRAILLE_UNICODE_SHORT is shared among several rules, so "RC" is used
    static BRAILLE_UNICODE_SHORT: UnicodeTable =
        Rc::new( RefCell::new( HashMap::with_capacity(500) ) );
        
    /// BRAILLE_UNICODE_FULL is shared among several rules, so "RC" is used
    static BRAILLE_UNICODE_FULL: UnicodeTable =
        Rc::new( RefCell::new( HashMap::with_capacity(5000) ) );

    /// SPEECH_DEFINITION_FILES_AND_TIMES is shared among several rules, so "RC" is used
    static SPEECH_DEFINITION_FILES_AND_TIMES: FilesAndTimesShared =
        Rc::new( RefCell::new(FilesAndTimes::default()) );
        
    /// BRAILLE_DEFINITION_FILES_AND_TIMES is shared among several rules, so "RC" is used
    static BRAILLE_DEFINITION_FILES_AND_TIMES: FilesAndTimesShared =
        Rc::new( RefCell::new(FilesAndTimes::default()) );
        
    /// SPEECH_UNICODE_SHORT_FILES_AND_TIMES is shared among several rules, so "RC" is used
    static SPEECH_UNICODE_SHORT_FILES_AND_TIMES: FilesAndTimesShared =
        Rc::new( RefCell::new(FilesAndTimes::default()) );
        
    /// SPEECH_UNICODE_FULL_FILES_AND_TIMES is shared among several rules, so "RC" is used
    static SPEECH_UNICODE_FULL_FILES_AND_TIMES: FilesAndTimesShared =
        Rc::new( RefCell::new(FilesAndTimes::default()) );
        
    /// BRAILLE_UNICODE_SHORT_FILES_AND_TIMES is shared among several rules, so "RC" is used
    static BRAILLE_UNICODE_SHORT_FILES_AND_TIMES: FilesAndTimesShared =
        Rc::new( RefCell::new(FilesAndTimes::default()) );
        
    /// BRAILLE_UNICODE_FULL_FILES_AND_TIMES is shared among several rules, so "RC" is used
    static BRAILLE_UNICODE_FULL_FILES_AND_TIMES: FilesAndTimesShared =
        Rc::new( RefCell::new(FilesAndTimes::default()) );
        
    /// The current set of speech rules
    // maybe this should be a small cache of rules in case people switch rules/prefs?
    pub static INTENT_RULES: RefCell<SpeechRules> =
            RefCell::new( SpeechRules::new(RulesFor::Intent, true) );

    pub static SPEECH_RULES: RefCell<SpeechRules> =
            RefCell::new( SpeechRules::new(RulesFor::Speech, true) );

    pub static OVERVIEW_RULES: RefCell<SpeechRules> =
            RefCell::new( SpeechRules::new(RulesFor::OverView, true) );

    pub static NAVIGATION_RULES: RefCell<SpeechRules> =
            RefCell::new( SpeechRules::new(RulesFor::Navigation, true) );

    pub static BRAILLE_RULES: RefCell<SpeechRules> =
            RefCell::new( SpeechRules::new(RulesFor::Braille, false) );
}

impl SpeechRules {
    pub fn new(name: RulesFor, translate_single_chars_only: bool) -> SpeechRules {
        let globals = if name == RulesFor::Braille {
            (
                (BRAILLE_UNICODE_SHORT.with(Rc::clone), BRAILLE_UNICODE_SHORT_FILES_AND_TIMES.with(Rc::clone)),
                (BRAILLE_UNICODE_FULL. with(Rc::clone), BRAILLE_UNICODE_FULL_FILES_AND_TIMES.with(Rc::clone)),
                BRAILLE_DEFINITION_FILES_AND_TIMES.with(Rc::clone),
            )
        } else {
            (
                (SPEECH_UNICODE_SHORT.with(Rc::clone), SPEECH_UNICODE_SHORT_FILES_AND_TIMES.with(Rc::clone)),
                (SPEECH_UNICODE_FULL. with(Rc::clone), SPEECH_UNICODE_FULL_FILES_AND_TIMES.with(Rc::clone)),
                SPEECH_DEFINITION_FILES_AND_TIMES.with(Rc::clone),
            )
        };

        return SpeechRules {
            error: Default::default(),
            name,
            rules: HashMap::with_capacity(if name == RulesFor::Intent || name == RulesFor::Speech {500} else {50}),                       // lazy load them
            rule_files: FilesAndTimes::default(),
            unicode_short: globals.0.0,       // lazy load them
            unicode_short_files: globals.0.1,
            unicode_full: globals.1.0,        // lazy load them
            unicode_full_files: globals.1.1,
            definitions_files: globals.2,
            translate_single_chars_only,
            pref_manager: PreferenceManager::get(),
        };
}

    pub fn get_error(&self) -> Option<&str> {
        return if self.error.is_empty() {
             None
        } else {
            Some(&self.error)
        }
    }

    pub fn read_files(&mut self) -> Result<()> {
        let check_rule_files = self.pref_manager.borrow().pref_to_string("CheckRuleFiles");
        if check_rule_files != "None" {  // "Prefs" or "All" are other values
            self.pref_manager.borrow_mut().set_preference_files()?;
        }
        let should_ignore_file_time = self.pref_manager.borrow().pref_to_string("CheckRuleFiles") != "All";     // ignore for "None", "Prefs"
        let rule_file = self.pref_manager.borrow().get_rule_file(&self.name).to_path_buf();     // need to create PathBuf to avoid a move/use problem
        if self.rules.is_empty() || !self.rule_files.is_file_up_to_date(&rule_file, should_ignore_file_time) {
            self.rules.clear();
            let files_read = self.read_patterns(&rule_file)?;
            self.rule_files.set_files_and_times(files_read);
        }

        let pref_manager = self.pref_manager.borrow();
        let unicode_pref_files = if self.name == RulesFor::Braille {pref_manager.get_braille_unicode_file()} else {pref_manager.get_speech_unicode_file()};

        if !self.unicode_short_files.borrow().is_file_up_to_date(unicode_pref_files.0, should_ignore_file_time) {
            self.unicode_short.borrow_mut().clear();
            self.unicode_short_files.borrow_mut().set_files_and_times(self.read_unicode(None, true)?);
        }

        if self.definitions_files.borrow().ft.is_empty() || !self.definitions_files.borrow().is_file_up_to_date(
                            pref_manager.get_definitions_file(self.name != RulesFor::Braille),
                            should_ignore_file_time
        ) {
            self.definitions_files.borrow_mut().set_files_and_times(read_definitions_file(self.name != RulesFor::Braille)?);
        }
        return Ok( () );
    }

    fn read_patterns(&mut self, path: &Path) -> Result<Vec<PathBuf>> {
        // info!("Reading rule file: {}", p.to_str().unwrap());
        let rule_file_contents = read_to_string_shim(path).chain_err(|| format!("cannot read file '{}'", path.to_str().unwrap()))?;
        let rules_build_fn = |pattern: &Yaml| {
            self.build_speech_patterns(pattern, path)
                .chain_err(||format!("in file {:?}", path.to_str().unwrap()))
        };
        return compile_rule(&rule_file_contents, rules_build_fn)
                .chain_err(||format!("in file {:?}", path.to_str().unwrap()));
    }

    fn build_speech_patterns(&mut self, patterns: &Yaml, file_name: &Path) -> Result<Vec<PathBuf>> {
        // Rule::SpeechPatternList
        let patterns_vec = patterns.as_vec();
        if patterns_vec.is_none() {
            bail!(yaml_type_err(patterns, "array"));
        }
        let patterns_vec = patterns.as_vec().unwrap();
        let mut files_read = vec![file_name.to_path_buf()];
        for entry in patterns_vec.iter() {
            if let Some(mut added_files) = SpeechPattern::build(entry, file_name, self)? {
                files_read.append(&mut added_files);
            }
        }
        return Ok(files_read)
    }
    
    fn read_unicode(&self, path: Option<PathBuf>, use_short: bool) -> Result<Vec<PathBuf>> {
        let path = match path {
            Some(p) => p,
            None => {
                // get the path to either the short or long unicode file
                let pref_manager = self.pref_manager.borrow();
                let unicode_files = if self.name == RulesFor::Braille {
                    pref_manager.get_braille_unicode_file()
                } else {
                    pref_manager.get_speech_unicode_file()
                };
                let unicode_files = if use_short {unicode_files.0} else {unicode_files.1};
                unicode_files.to_path_buf()
            }
        };

        // FIX: should read first (lang), then supplement with second (region)
        // info!("Reading unicode file {}", path.to_str().unwrap());
        let unicode_file_contents = read_to_string_shim(&path)?;
        let unicode_build_fn = |unicode_def_list: &Yaml| {
            let unicode_defs = unicode_def_list.as_vec();
            if unicode_defs.is_none() {
                bail!("File '{}' does not begin with an array", yaml_to_type(unicode_def_list));
            };
            let mut files_read = vec![path.to_path_buf()];
            for unicode_def in unicode_defs.unwrap() {
                if let Some(mut added_files) = UnicodeDef::build(unicode_def, &path, self, use_short)
                                                                .chain_err(|| {format!("In file {:?}", path.to_str())})? {
                    files_read.append(&mut added_files);
                }
            };
            return Ok(files_read)
        };

        return compile_rule(&unicode_file_contents, unicode_build_fn)
                    .chain_err(||format!("in file {:?}", path.to_str().unwrap()));
    }

    pub fn print_sizes() -> String {
        // let _ = &SPEECH_RULES.with_borrow(|rules| {
        //     debug!("SPEECH RULES entries\n");
        //     let rules = &rules.rules;
        //     for (key, _) in rules.iter() {
        //         debug!("key: {}", key);
        //     }
        // });
        let mut answer = rule_size(&SPEECH_RULES, "SPEECH_RULES");
        answer += &rule_size(&INTENT_RULES, "INTENT_RULES");
        answer += &rule_size(&BRAILLE_RULES, "BRAILLE_RULES");
        answer += &rule_size(&NAVIGATION_RULES, "NAVIGATION_RULES");
        answer += &rule_size(&OVERVIEW_RULES, "OVERVIEW_RULES");
        SPEECH_RULES.with_borrow(|rule| {
            answer += &format!("Speech Unicode tables: short={}/{}, long={}/{}\n",
                                rule.unicode_short.borrow().len(), rule.unicode_short.borrow().capacity(),
                                rule.unicode_full.borrow().len(), rule.unicode_full.borrow().capacity());
        });
        BRAILLE_RULES.with_borrow(|rule| {
            answer += &format!("Braille Unicode tables: short={}/{}, long={}/{}\n",
                                rule.unicode_short.borrow().len(), rule.unicode_short.borrow().capacity(),
                                rule.unicode_full.borrow().len(), rule.unicode_full.borrow().capacity());
        });
        return answer;

        fn rule_size(rules: &'static std::thread::LocalKey<RefCell<SpeechRules>>, name: &str) -> String {
            rules.with_borrow(|rule| {
                let hash_map = &rule.rules;
                return format!("{}: {}/{}\n", name, hash_map.len(), hash_map.capacity());
            })
        }
    }
}


/// We track three different lifetimes:
///   'c -- the lifetime of the context and mathml
///   's -- the lifetime of the speech rules (which is static)
///   'r -- the lifetime of the reference (this seems to be key to keep the rust memory checker happy)
impl<'c, 's:'c, 'r, 'm:'c> SpeechRulesWithContext<'c, 's,'m> {
    pub fn new(speech_rules: &'s SpeechRules, doc: Document<'m>, nav_node_id: &'m str) -> SpeechRulesWithContext<'c, 's, 'm> {
        return SpeechRulesWithContext {
            speech_rules,
            context_stack: ContextStack::new(&speech_rules.pref_manager.borrow()),
            doc,
            nav_node_id,
            inside_spell: false,
            translate_count: 0,
        }
    }

    pub fn get_rules(&mut self) -> &SpeechRules {
        return self.speech_rules;
    }

    pub fn get_context(&mut self) -> &mut Context<'c> {
        return &mut self.context_stack.base;
    }

    pub fn get_document(&mut self) -> Document<'m> {
        return self.doc;
    }

    pub fn match_pattern<T:TreeOrString<'c, 'm, T>>(&'r mut self, mathml: Element<'c>) -> Result<T> {
        // debug!("Looking for a match for: \n{}", mml_to_string(mathml));
        let tag_name = mathml.name().local_part();
        let rules = &self.speech_rules.rules;

        // start with priority rules that apply to any node (should be a very small number)
        if let Some(rule_vector) = rules.get("!*") {
            if let Some(result) = self.find_match(rule_vector, mathml)? {
                return Ok(result);      // found a match
            }
        }
        
        if let Some(rule_vector) = rules.get(tag_name) {
            if let Some(result) = self.find_match(rule_vector, mathml)? {
                return Ok(result);      // found a match
            }
        }

        // no rules for specific element, fall back to rules for "*" which *should* be present in all rule files as fallback
        if let Some(rule_vector) = rules.get("*") {
            if let Some(result) = self.find_match(rule_vector, mathml)? {
                return Ok(result);      // found a match
            }
        }

        // no rules matched -- poorly written rule file -- let flow through to default error
        // report error message with file name
        let speech_manager = self.speech_rules.pref_manager.borrow();
        let file_name = speech_manager.get_rule_file(&self.speech_rules.name);
        // FIX: handle error appropriately 
        bail!("\nNo match found!\nMissing patterns in {} for MathML.\n{}", file_name.to_string_lossy(), mml_to_string(mathml));
    }

    fn find_match<T:TreeOrString<'c, 'm, T>>(&'r mut self, rule_vector: &[Box<SpeechPattern>], mathml: Element<'c>) -> Result<Option<T>> {
        for pattern in rule_vector {
            // debug!("Pattern name: {}", pattern.pattern_name);
            // always pushing and popping around the is_match would be a little cleaner, but push/pop is relatively expensive,
            //   so we optimize and only push first if the variables are needed to do the match
            if pattern.match_uses_var_defs {
                self.context_stack.push(pattern.var_defs.clone(), mathml)?;
            }
            if pattern.is_match(&self.context_stack.base, mathml)
                    .chain_err(|| error_string(pattern, mathml) )? {
                // debug!("  find_match: FOUND!!!");
                if !pattern.match_uses_var_defs && pattern.var_defs.len() > 0 { // don't push them on twice
                    self.context_stack.push(pattern.var_defs.clone(), mathml)?;
                }
                let result: Result<T> = pattern.replacements.replace(self, mathml);
                if pattern.var_defs.len() > 0 {
                    self.context_stack.pop();
                }
                return match result {
                    Ok(s) => {
                        // for all except braille and navigation, nav_node_id will be an empty string and will not match
                        if self.nav_node_id.is_empty() {
                            Ok( Some(s) )
                        } else {
                            // if self.nav_node_id == mathml.attribute_value("id").unwrap_or_default() {debug!("Matched pattern name/tag: {}/{}", pattern.pattern_name, pattern.tag_name)};
                            Ok ( Some(self.nav_node_adjust(s, mathml)) )
                        }
                    },
                    Err(e) => Err( e.chain_err(||
                        format!(
                            "attempting replacement pattern: \"{}\" for \"{}\".\n\
                            Replacement\n{}\n...due to matching the MathML\n{} with the pattern\n\
                            {}\n\
                            The patterns are in {}.\n",
                            pattern.pattern_name, pattern.tag_name,
                            pattern.replacements.pretty_print_replacements(),
                            mml_to_string(mathml), pattern.pattern,
                            pattern.file_name
                        )
                    ))
                }
            } else if pattern.match_uses_var_defs {
                self.context_stack.pop();
            }
        };
        return Ok(None);    // no matches

        fn error_string(pattern: &SpeechPattern, mathml: Element) -> String {
            return format!(
                "error during pattern match using: \"{}\" for \"{}\".\n\
                Pattern is \n{}\nMathML for the match:\n\
                {}\
                The patterns are in {}.\n",
                pattern.pattern_name, pattern.tag_name,
                pattern.pattern,
                mml_to_string(mathml),
                pattern.file_name
            );
        }

    }

    fn nav_node_adjust<T:TreeOrString<'c, 'm, T>>(&self, speech: T, mathml: Element<'c>) -> T {
        if let Some(id) = mathml.attribute_value("id") {
            if self.nav_node_id == id {
                if self.speech_rules.name == RulesFor::Braille {
                    let highlight_style =  self.speech_rules.pref_manager.borrow().pref_to_string("BrailleNavHighlight");
                    return T::highlight_braille(speech, highlight_style);
                } else {
                    return T::mark_nav_speech(speech)
                }
            }
        }
        return speech;

    }
    
    fn highlight_braille_string(braille: String, highlight_style: String) -> String {
        // add dots 7 & 8 to the Unicode braille (28xx)
        if &highlight_style == "Off" || braille.is_empty() {
            return braille;
        }
        
        // FIX: this seems needlessly complex. It is much simpler if the char can be changed in place...
        // find first char that can get the dots and add them
        let mut chars = braille.chars().collect::<Vec<char>>();

        // the 'b' for baseline indicator is really part of the previous token, so it needs to be highlighted but isn't because it is not Unicode braille
        let baseline_indicator_hack = PreferenceManager::get().borrow().pref_to_string("BrailleCode") == "Nemeth";
        // debug!("highlight_braille_string: highlight_style={}\n braille={}", highlight_style, braille);
        let mut i_first_modified = 0;
        for (i, ch) in chars.iter_mut().enumerate() {
            let modified_ch = add_dots_to_braille_char(*ch, baseline_indicator_hack);
            if *ch != modified_ch {
                *ch = modified_ch; 
                i_first_modified = i;
                break;
            };
        };

        let mut i_last_modified = i_first_modified;
        if &highlight_style != "FirstChar" {
            // find last char so that we know when to modify the char
            for i in (i_first_modified..chars.len()).rev(){
                let ch = chars[i];
                let modified_ch = add_dots_to_braille_char(ch, baseline_indicator_hack);
                chars[i] = modified_ch;
                if ch !=  modified_ch {
                    i_last_modified = i;
                    break;
                }
            }
        }

        if &highlight_style == "All" {
            // finish going through the string
			#[allow(clippy::needless_range_loop)]  // I don't like enumerate/take/skip here
            for i in i_first_modified+1..i_last_modified {
                chars[i] = add_dots_to_braille_char(chars[i], baseline_indicator_hack);
            };
        }

        let result = chars.into_iter().collect::<String>(); 
        // debug!("    result={}", result);
        return result;

        fn add_dots_to_braille_char(ch: char, baseline_indicator_hack: bool) -> char {
            let as_u32 = ch as u32;
            if (0x2800..0x28FF).contains(&as_u32) {
                return unsafe {char::from_u32_unchecked(as_u32 | 0xC0)};
            } else if baseline_indicator_hack && ch == 'b' {
                return '𝑏'
            } else {
                return ch;
            }
        }
    }

    fn mark_nav_speech(speech: String) -> String {
        // add unique markers (since speech is mostly ascii letters and digits, most any symbol will do)
        // debug!("mark_nav_speech: adding [[ {} ]] ", &speech);
        return "[[".to_string() + &speech + "]]";
    }

    fn replace<T:TreeOrString<'c, 'm, T>>(&'r mut self, replacement: &Replacement, mathml: Element<'c>) -> Result<T> {
        return Ok(
            match replacement {
                Replacement::Text(t) => T::from_string(t.clone(), self.doc)?,
                Replacement::XPath(xpath) => xpath.replace(self, mathml)?,
                Replacement::TTS(tts) => {
                    T::from_string(
                        self.speech_rules.pref_manager.borrow().get_tts().replace(tts, &self.speech_rules.pref_manager.borrow(), self, mathml)?,
                        self.doc
                    )?
                },
                Replacement::Intent(intent) => {
                    intent.replace(self, mathml)?                     
                },
                Replacement::Test(test) => {
                    test.replace(self, mathml)?                     
                },
                Replacement::With(with) => {
                    with.replace(self, mathml)?                     
                },
                Replacement::SetVariables(vars) => {
                    vars.replace(self, mathml)?                     
                },
                Replacement::Insert(ic) => {
                    ic.replace(self, mathml)?                     
                },
                Replacement::Translate(id) => {
                    id.replace(self, mathml)?                     
                },
            }
        )
    }

    /// Iterate over all the nodes, concatenating the result strings together with a ' ' between them
    /// If the node is an element, pattern match it
    /// For 'Text' and 'Attribute' nodes, convert them to strings
    fn replace_nodes<T:TreeOrString<'c, 'm, T>>(&'r mut self, nodes: Vec<Node<'c>>, mathml: Element<'c>) -> Result<T> {
        return T::replace_nodes(self, nodes, mathml);
    }

    /// Iterate over all the nodes finding matches for the elements
    /// For this case of returning MathML, everything else is an error
    fn replace_nodes_tree(&'r mut self, nodes: Vec<Node<'c>>, _mathml: Element<'c>) -> Result<Element<'m>> {
        let mut children = Vec::with_capacity(3*nodes.len());   // guess (2 chars/node + space)
        for node in nodes {
            let matched = match node {
                Node::Element(n) => self.match_pattern::<Element<'m>>(n)?,
                Node::Text(t) =>  {
                    let leaf = create_mathml_element(&self.doc, "TEMP_NAME");
                    // debug!("  from leaf with text '{}'", &t.text());
                    leaf.set_text(t.text());
                    leaf
                },
                Node::Attribute(attr) => {
                    // debug!("  from attr with text '{}'", attr.value());
                    let leaf = create_mathml_element(&self.doc, "TEMP_NAME");
                    leaf.set_text(attr.value());
                    leaf
                },
                _ => {
                    bail!("replace_nodes: found unexpected node type!!!");
                },
            };
            children.push(matched);
        }

        let result = create_mathml_element(&self.doc, "TEMP_NAME");    // FIX: what name should be used?
        result.append_children(children);
        // debug!("replace_nodes_tree\n{}\n====>>>>>\n", mml_to_string(result));
        return Ok( result );
    }

    fn replace_nodes_string(&'r mut self, nodes: Vec<Node<'c>>, mathml: Element<'c>) -> Result<String> {
        // debug!("replace_nodes: working on {} nodes", nodes.len());
        let mut result = String::with_capacity(3*nodes.len());   // guess (2 chars/node + space)
        let mut first_time = true;
        for node in nodes {
            if first_time {
                first_time = false;
            } else {
                result.push(' ');
            };
            let matched = match node {
                Node::Element(n) => self.match_pattern::<String>(n)?,
                Node::Text(t) =>  self.replace_chars(t.text(), mathml)?,
                Node::Attribute(attr) => self.replace_chars(attr.value(), mathml)?,
                _ => bail!("replace_nodes: found unexpected node type!!!"),
            };
            result += &matched;
        }
        return Ok( result );
    }

    /// Lookup unicode "pronunciation" of char.
    /// Note: TTS is not supported here (not needed and a little less efficient)
    pub fn replace_chars(&'r mut self, str: &str, mathml: Element<'c>) -> Result<String> {
        if is_quoted_string(str) {
            return Ok(unquote_string(str).to_string());
        }
        let rules = self.speech_rules;
        let mut chars = str.chars();
        // in a string, avoid "a" -> "eigh", "." -> "point", etc
        if rules.translate_single_chars_only {
            let ch = chars.next().unwrap_or(' ');
            if chars.next().is_none() {
                // single char
                return replace_single_char(self, ch, mathml)
            } else {
                // more than one char -- fix up non-breaking space
                return Ok(str.replace('\u{00A0}', " ").replace(['\u{2061}', '\u{2062}', '\u{2063}', '\u{2064}'], ""))
            }
        };

        let result = chars
            .map(|ch| replace_single_char(self, ch, mathml))
            .collect::<Result<Vec<String>>>()?
            .join("");
        return Ok( result );

        fn replace_single_char<'c, 's:'c, 'm, 'r>(rules_with_context: &'r mut SpeechRulesWithContext<'c,'s,'m>, ch: char, mathml: Element<'c>) -> Result<String> {
            let ch_as_u32 = ch as u32;
            let rules = rules_with_context.speech_rules;
            let mut unicode = rules.unicode_short.borrow();
            let mut replacements = unicode.get( &ch_as_u32 );
            if replacements.is_none() {
                // see if it in the full unicode table (if it isn't loaded already)
                let pref_manager = rules.pref_manager.borrow();
                let unicode_pref_files = if rules.name == RulesFor::Braille {pref_manager.get_braille_unicode_file()} else {pref_manager.get_speech_unicode_file()};
                let should_ignore_file_time = pref_manager.pref_to_string("CheckRuleFiles") == "All";
                if rules.unicode_full.borrow().is_empty() || !rules.unicode_full_files.borrow().is_file_up_to_date(unicode_pref_files.1, should_ignore_file_time) {
                    info!("*** Loading full unicode {} for char '{}'/{:#06x}", rules.name, ch, ch_as_u32);
                    rules.unicode_full.borrow_mut().clear();
                    rules.unicode_full_files.borrow_mut().set_files_and_times(rules.read_unicode(None, false)?);
                    info!("# Unicode defs = {}/{}", rules.unicode_short.borrow().len(), rules.unicode_full.borrow().len());
                }
                unicode = rules.unicode_full.borrow();
                replacements = unicode.get( &ch_as_u32 );
                if replacements.is_none() {
                    // debug!("*** Did not find unicode {} for char '{}'/{:#06x}", rules.name, ch, ch_as_u32);
                    rules_with_context.translate_count = 0;     // not in loop
                    return Ok(String::from(ch));   // no replacement, so just return the char and hope for the best
                }
            };

            // map across all the parts of the replacement, collect them up into a Vec, and then concat them together
            let result = replacements.unwrap()
                        .iter()
                        .map(|replacement|
                            rules_with_context.replace(replacement, mathml)
                                    .chain_err(|| format!("Unicode replacement error: {replacement}")) )
                        .collect::<Result<Vec<String>>>()?
                        .join(" ");
            rules_with_context.translate_count = 0;     // found a replacement, so not in a loop
            return Ok(result);
        }
    }
}

/// Hack to allow replacement of `str` with braille chars.
pub fn braille_replace_chars(str: &str, mathml: Element) -> Result<String> {
    return BRAILLE_RULES.with(|rules| {
        let rules = rules.borrow();
        let new_package = Package::new();
        let mut rules_with_context = SpeechRulesWithContext::new(&rules, new_package.as_document(), "");
        return rules_with_context.replace_chars(str, mathml);
    })
}



#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::init_logger;

    use super::*;

    #[test]
    fn test_read_statement() {
        let str = r#"---
        {name: default, tag: math, match: ".", replace: [x: "./*"] }"#;
        let doc = YamlLoader::load_from_str(str).unwrap();
        assert_eq!(doc.len(), 1);
        let mut rules = SpeechRules::new(RulesFor::Speech, true);

        SpeechPattern::build(&doc[0], Path::new("testing"), &mut rules).unwrap();
        assert_eq!(rules.rules["math"].len(), 1, "\nshould only be one rule");

        let speech_pattern = &rules.rules["math"][0];
        assert_eq!(speech_pattern.pattern_name, "default", "\npattern name failure");
        assert_eq!(speech_pattern.tag_name, "math", "\ntag name failure");
        assert_eq!(speech_pattern.pattern.rc.string, ".", "\npattern failure");
        assert_eq!(speech_pattern.replacements.replacements.len(), 1, "\nreplacement failure");
        assert_eq!(speech_pattern.replacements.replacements[0].to_string(), r#""./*""#, "\nreplacement failure");
    }

    #[test]
    fn test_read_statements_with_replace() {
        let str = r#"---
        {name: default, tag: math, match: ".", replace: [x: "./*"] }"#;
        let doc = YamlLoader::load_from_str(str).unwrap();
        assert_eq!(doc.len(), 1);
        let mut rules = SpeechRules::new(RulesFor::Speech, true);
        SpeechPattern::build(&doc[0], Path::new("testing"), &mut rules).unwrap();

        let str = r#"---
        {name: default, tag: math, match: ".", replace: [t: "test", x: "./*"] }"#;
        let doc2 = YamlLoader::load_from_str(str).unwrap();
        assert_eq!(doc2.len(), 1);
        SpeechPattern::build(&doc2[0], Path::new("testing"), &mut rules).unwrap();
        assert_eq!(rules.rules["math"].len(), 1, "\nfirst rule not replaced");

        let speech_pattern = &rules.rules["math"][0];
        assert_eq!(speech_pattern.pattern_name, "default", "\npattern name failure");
        assert_eq!(speech_pattern.tag_name, "math", "\ntag name failure");
        assert_eq!(speech_pattern.pattern.rc.string, ".", "\npattern failure");
        assert_eq!(speech_pattern.replacements.replacements.len(), 2, "\nreplacement failure");
    }

    #[test]
    fn test_read_statements_with_add() {
        let str = r#"---
        {name: default, tag: math, match: ".", replace: [x: "./*"] }"#;
        let doc = YamlLoader::load_from_str(str).unwrap();
        assert_eq!(doc.len(), 1);
        let mut rules = SpeechRules::new(RulesFor::Speech, true);
        SpeechPattern::build(&doc[0], Path::new("testing"), &mut rules).unwrap();

        let str = r#"---
        {name: another-rule, tag: math, match: ".", replace: [t: "test", x: "./*"] }"#;
        let doc2 = YamlLoader::load_from_str(str).unwrap();
        assert_eq!(doc2.len(), 1);
        SpeechPattern::build(&doc2[0], Path::new("testing"), &mut rules).unwrap();
        assert_eq!(rules.rules["math"].len(), 2, "\nsecond rule not added");

        let speech_pattern = &rules.rules["math"][0];
        assert_eq!(speech_pattern.pattern_name, "default", "\npattern name failure");
        assert_eq!(speech_pattern.tag_name, "math", "\ntag name failure");
        assert_eq!(speech_pattern.pattern.rc.string, ".", "\npattern failure");
        assert_eq!(speech_pattern.replacements.replacements.len(), 1, "\nreplacement failure");
    }

    #[test]
    fn test_debug_no_debug() {
        let str = r#"*[2]/*[3][text()='3']"#;
        let result = MyXPath::add_debug_string_arg(str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), str);
    }

    #[test]
    fn test_debug_no_debug_with_quote() {
        let str = r#"*[2]/*[3][text()='(']"#;
        let result = MyXPath::add_debug_string_arg(str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), str);
    }

    #[test]
    fn test_debug_no_quoted_paren() {
        let str = r#"DEBUG(*[2]/*[3][text()='3'])"#;
        let result = MyXPath::add_debug_string_arg(str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"DEBUG(*[2]/*[3][text()='3'], "*[2]/*[3][text()='3']")"#);
    }

    #[test]
    fn test_debug_quoted_paren() {
        let str = r#"DEBUG(*[2]/*[3][text()='('])"#;
        let result = MyXPath::add_debug_string_arg(str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"DEBUG(*[2]/*[3][text()='('], "*[2]/*[3][text()='(']")"#);
    }

    #[test]
    fn test_debug_quoted_paren_before_paren() {
        let str = r#"DEBUG(ClearSpeak_Matrix = 'Combinatorics') and IsBracketed(., '(', ')')"#;
        let result = MyXPath::add_debug_string_arg(str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"DEBUG(ClearSpeak_Matrix = 'Combinatorics', "ClearSpeak_Matrix = 'Combinatorics'") and IsBracketed(., '(', ')')"#);
    }


// zipped files do NOT include "zz", hence we need to exclude this test
cfg_if::cfg_if! {if #[cfg(not(feature = "include-zip"))] {  
    #[test]
    fn test_up_to_date() {
        use crate::interface::*;
        // initialize and move to a directory where making a time change doesn't really matter
        set_rules_dir(super::super::abs_rules_dir_path()).unwrap();
        set_preference("Language".to_string(), "zz-aa".to_string()).unwrap();
        // not much is support in zz
        if let Err(e) = set_mathml("<math><mi>x</mi></math>".to_string()) {
            error!("{}", crate::errors_to_string(&e));
            panic!("Should not be an error in setting MathML")
        }

        set_preference("CheckRuleFiles".to_string(), "All".to_string()).unwrap();
        assert!(!is_file_time_same(), "file's time did not get updated");
        set_preference("CheckRuleFiles".to_string(), "None".to_string()).unwrap();
        assert!(is_file_time_same(), "file's time was wrongly updated (preference 'CheckRuleFiles' should have prevented updating)");

        // change a file, cause read_files to be called, and return if MathCAT noticed the change and updated its time
        fn is_file_time_same() -> bool {
            // read and write a unicode file in a test dir
            // files are read in due to setting the MathML

            use std::time::Duration;
            return SPEECH_RULES.with(|rules| {
                let start_main_file = rules.borrow().unicode_short_files.borrow().ft[0].clone();

                // open the file, read all the contents, then write them back so the time changes
                let contents = std::fs::read(&start_main_file.file).expect(&format!("Failed to read file {} during test", &start_main_file.file.to_string_lossy()));
                std::fs::write(start_main_file.file, contents).unwrap();
                std::thread::sleep(Duration::from_millis(5));       // pause a little to make sure the time changes

                // speak should cause the file stored to have a new time
                if let Err(e) = get_spoken_text() {
                    error!("{}", crate::errors_to_string(&e));
                    panic!("Should not be an error in speech")
                }
                return rules.borrow().unicode_short_files.borrow().ft[0].time == start_main_file.time;
            });
        }    
    }
}}

    // #[test]
    // fn test_nested_debug_quoted_paren() {
    //     let str = r#"DEBUG(*[2]/*[3][DEBUG(text()='(')])"#;
    //     let result = MyXPath::add_debug_string_arg(str);
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), r#"DEBUG(*[2]/*[3][DEBUG(text()='(')], "DEBUG(*[2]/*[3][DEBUG(text()='(')], \"text()='(')]\")"#);
    // }

}