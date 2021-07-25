//! The speech module is where the speech rules are read in and speech generated.
//!
//! The main external call, [`speak_mathml`] returns a string for the speech associated with the `mathml`
//!   param based on the user preferences.
//!
//! The speech rules call out to the preferences and tts modules and the dividing line is not always clean.
//! A number of useful utility functions used by other modules are defined here.

use std::{collections::HashMap, process::exit};
use std::cell::RefCell;
use std::rc::Rc;
use sxd_document::dom::Element;
use sxd_xpath::{Context, Factory, Value, XPath, nodeset};
use sxd_xpath::nodeset::Node;
use std::fmt;
use crate::{errors::*, pretty_print};
use crate::prefs::*;
use yaml_rust::{YamlLoader, Yaml, yaml::Hash};
use crate::tts::*;
use crate::pretty_print::yaml_to_string;

/// The main external call, `speak_mathml` returns a string for the speech associated with the `mathml`.
///   It matches against the rules that are computed by user prefs such as "Language" and "SpeechStyle".
///
/// The speech rules assume `mathml` has been "cleaned" via the canonicalization step.
///
/// If the preferences change (and hence the speech rules to use change), or if the rule file changes,
///   `speak_mathml` will detect that and (re)load the proper rules.
///
/// A string is returned in call cases.
/// If there is an error, the speech string will indicate an error.
pub fn speak_mathml(mathml: &Element) -> String {
    SPEECH_RULES.with(|rules| {
        let rules = rules.borrow_mut();
        CONTEXT_STACK.with(|cs| {
            let mut cs = cs.borrow_mut();
            cs.init(&rules.pref_manager)
        });
        match rules.match_pattern(mathml) {
            Ok(speech_string) => {
                return rules.pref_manager.get_tts()
                            .merge_pauses(remove_optional_indicators(
                                &speech_string.replace(CONCAT_STRING, "")
                                                  .replace(CONCAT_INDICATOR, "")                            
                                          )
                            .trim()); },
            Err(e)             => { 
                print_errors(&e.chain_err(|| "Pattern match/replacement failure!"));
                return String::from("Error in speaking math; see error log.")
            }
        }
    })
}


/// Converts its argument to a string that can be used in a debugging message.
pub fn yaml_to_type(yaml: &Yaml) -> String {
    return match yaml {
        Yaml::Real(v)=> format!("real='{:#}'", v),
        Yaml::Integer(v)=> format!("integer='{:#}'", v),
        Yaml::String(v)=> format!("string='{:#}'", v),
        Yaml::Boolean(v)=> format!("boolean='{:#}'", v),
        Yaml::Array(v)=> match v.len() {
            0 => "array with no entries".to_string(),
            1 => format!("array with the entry: {}", yaml_to_type(&v[0]).to_string()),
            _ => format!("array with {} entries. First entry: {}", v.len(), yaml_to_type(&v[0]).to_string()),
        }
        Yaml::Hash(h)=> {
            let first_pair = 
                if h.len()==0 {
                    "no pairs".to_string()
                } else {
                    let (key, val) = h.iter().next().unwrap();
                    format!("({}, {})", yaml_to_type(key), yaml_to_type(val))
                };
            format!("dictionary with {} pair{}. A pair: {}", h.len(), if h.len()==1 {""} else {"s"}, first_pair)
        }
        Yaml::Alias(_)=> format!("Alias"),
        Yaml::Null=> format!("Null"),
        Yaml::BadValue=> format!("BadValue"),       
    }
}

fn yaml_type_err(yaml: &Yaml, str: &str) -> String {
    return format!("Expected {}, found {}", str, yaml_to_type(yaml));
}

fn yaml_key_err(dict: &Yaml, key: &str, yaml_type: &str) -> String {
    if dict.as_hash().is_none() {
       return format!("Expected dictionary with key '{}', found\n{}", key, yaml_to_string(dict, 1));
    }
    let str = &dict[key];
    if str.is_badvalue() {
        return format!("Did not find '{}' in\n{}", key,  yaml_to_string(dict, 1));
    }
    return format!("Type of '{}' is not a {}.\nIt is a {}. YAML value is\n{}", 
            key, yaml_type, yaml_to_type(str), yaml_to_string(dict, 0));
}

fn find_str<'a>(dict: &'a Yaml, key: &'a str) -> Result<&'a str> {
    let value = dict[key].as_str();
    let result = value.ok_or_else(|| yaml_key_err(dict, key, "str"))?;
    return Ok( result );
}

/// Returns the Yaml as a `Hash` or an error if it isn't.
pub fn as_hash_checked<'a>(value: &Yaml) -> Result<&Hash> {
    let result = value.as_hash();
    let result = result.ok_or_else(|| yaml_type_err(value, "hashmap"))?;
    return Ok( result );
}

/// Returns the Yaml as a `Vec` or an error if it isn't.
pub fn as_vec_checked<'a>(value: &Yaml) -> Result<&Vec<Yaml>> {
    let result = value.as_vec();
    let result = result.ok_or_else(|| yaml_type_err(value, "array"))?;
    return Ok( result );
}

/// Returns the Yaml as a `&str` or an error if it isn't.
pub fn as_str_checked<'a>(yaml: &'a Yaml) -> Result<&'a str> {
    return Ok( yaml.as_str().ok_or_else(|| yaml_type_err(yaml, "string"))? );
}


/// A bit of a hack to concatenate replacements (without a ' ').
/// The CONCAT_INDICATOR is added by a "ct:" (instead of 't:') in the speech rules
/// and checked for by the tts code.
pub const CONCAT_INDICATOR: &str = "\u{F8FE}";

// This is the pattern that needs to be matched (and deleted)
const CONCAT_STRING: &str = " \u{F8FE}";

// a similar hack to potentially delete (repetitive) optional replacements
// the OPTIONAL_INDICATOR is added by "ot:" before and after the optional string
const OPTIONAL_INDICATOR: &str  = "\u{F8FD}";
const OPTIONAL_INDICATOR_LEN: usize = OPTIONAL_INDICATOR.len();

fn remove_optional_indicators(str: &str) -> String {
    return str.replace(OPTIONAL_INDICATOR, "");
}

/// Given a string that should be Yaml, it calls `build_fn` with that string.
/// The build function/closure should process the Yaml as appropriate and capture any errors and write them to `std_err`.
pub fn compile_rule<F>(str: &str, mut build_fn: F) -> Result<()> where
    F: FnMut(&Yaml) {
    let docs = YamlLoader::load_from_str(str);
    match docs {
        Err(e) => {
            bail!("Parse error!!: {}", e);
        },
        Ok(docs) => {
            if docs.len() != 1 {
                bail!("Didn't find rules!");
            }
            build_fn(&docs[0]);
            return Ok( () );
        }
    }
}



// 'Replacement' is an enum that contains all the potential replacement types/structs
// Hence there are fields 'Test' ("test:"), 'Text" ("t:"), etc
#[derive(Debug, Clone)]
enum Replacement {
    // Note: all of these are pointer types
    Test(Box<Test>),
    Text(String),
    XPath(MyXPath),
    TTS(Box<TTSCommandRule>),
    Insert(Box<InsertChildren>),
}

impl fmt::Display for Replacement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}",
            match self {
                Replacement::Test(c) => c.to_string(),
                Replacement::Text(t) => format!("{{t: \"{}}}\"", t),
                Replacement::XPath(x) => x.to_string(),
                Replacement::TTS(t) => t.to_string(),
                Replacement::Insert(ic) => ic.to_string(),
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
        if dictionary.len() == 0 { 
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
        let key = key.as_str().ok_or_else(|| format!("replacement key(e.g, 't') is not a string"))?;
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
            "pause" | "rate" | "pitch" | "volume" | "gender" | "voice" | "spell" => {
                return Ok( Replacement::TTS( TTS::build(key, value)? ) );
            },
            "test" => {
                return Ok( Replacement::Test( Test::build(value)? ) );
            },
            "insert" => {
                return Ok( Replacement::Insert( InsertChildren::build(value)? ) );
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
        let nodes = as_str_checked(&nodes)?;
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
    fn replace(&self, rules: &SpeechRules, mathml: &Element) -> Result<String> {
        let result = self.xpath.evaluate(mathml)
                .chain_err(||"replacing after pattern match" )?;
        match result {
            Value::Nodeset(nodes) => {
                if nodes.size() == 0 {
                    bail!("During replacement, no matching element found");
                };
                let n_nodes = nodes.size();
                let mut expanded_result = Vec::with_capacity(n_nodes + (n_nodes+1)*self.replacements.replacements.len());
                expanded_result.push(
                    Replacement::XPath(
                        MyXPath::new(format!("{}[{}]", self.xpath.string , 1))?
                    )
                );
                for i in 2..n_nodes+1 {
                    expanded_result.extend_from_slice(&self.replacements.replacements);
                    expanded_result.push(
                        Replacement::XPath(
                            MyXPath::new(format!("{}[{}]", self.xpath.string , i))?
                        )
                    );
                }
                let replacements = ReplacementArray{ replacements: expanded_result };
                return replacements.replace(rules, mathml);
            },

            // FIX: should the options be errors???
            Value::String(t) => { return rules.replace_chars(&t, mathml); },
            Value::Number(num)  => { return Ok( num.to_string() ); },
            Value::Boolean(b)  => { return Ok( b.to_string() ); },          // FIX: is this right???
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
        let result;
        if replacements.is_array() {
            let replacements = replacements.as_vec().unwrap();
            result = replacements
                .iter()
                .enumerate()    // useful for errors
                .map(|(i, r)| Replacement::build(r)
                            .chain_err(|| format!("replacement #{} of {}", i+1, replacements.len())))
                .collect::<Result<Vec<Replacement>>>()?;
        } else {
            result = vec![ Replacement::build(replacements)?];
        };

        return Ok( ReplacementArray{ replacements: result } );
    }

    /// Do all the replacements in `mathml` using `rules`.
    pub fn replace(&self, rules: &SpeechRules, mathml: &Element) -> Result<String> {
        // do the replacements
        //   remove the empty strings (the later 'join' would add extraneous spaces)
        //  collect the strings together into an array
        let mut replacement_strings =
            self.replacements.iter()
                        .map(|group| rules.replace(group, mathml))
                        .filter(|result| if let Ok(str) = result {str.len() > 0} else {true})
                        .collect::<Result<Vec<String>>>()?;

        if replacement_strings.len() == 0 {
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
                        
        // find the replacement for 'pause: auto' and compute the real value based on its neighbors
        // if replacement_strings.len() > 1 {
        //     println!("Strings({})", replacement_strings.len());
        // };
        for i in 0..replacement_strings.len() {
            if replacement_strings[i].contains(PAUSE_AUTO_STR) {
                let before = if i == 0 {""} else {&replacement_strings[i-1]};
                let after = if i+1 == replacement_strings.len() {""} else {&replacement_strings[i+1]};
                replacement_strings[i] = replacement_strings[i].replace(
                    PAUSE_AUTO_STR,
                    &rules.pref_manager.get_tts().compute_auto_pause(&rules.pref_manager, before, after));
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
                            // println!("check if '{}' is repetitive",  optional_word);
                            // println!("   prev: '{}', next '{}'", prev, optional);
                            let prev = prev.trim_end().as_bytes();
                            if &prev[prev.len()-optional_word.len()..] == optional_word.as_bytes() {
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

    /// Return true if there are no replacements.
    pub fn is_empty(&self) -> bool {
        return self.replacements.len() == 0;
    }
    
    fn pretty_print_replacements(&self) -> String {
        let mut group_string = String::with_capacity(128);
        if self.replacements.len() == 1 {
            group_string += &format!("[{}]", self.replacements[0]);
        } else {
            group_string += &self.replacements.iter()
                    .map(|replacement| format!("\n  - {}", replacement))
                    .collect::<Vec<String>>()
                    .join("");
            group_string += "\n";
        }
        return group_string;
    }
}



// MyXPath is a wrapper around an 'XPath' that keeps around the original xpath expr (as a string) so it can be used in error reporting.
// It supports the standard SpeechRule functionality of building and replacing.
#[derive(Debug)]
struct MyXPath {
    xpath: XPath,
    string: String,        // store for error reporting
}

impl fmt::Display for MyXPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{{x: \"{}\"}}", self.string);
    }
}

impl Clone for MyXPath {
    fn clone(&self) -> Self {
        return MyXPath{
            xpath: MyXPath::compile_xpath(&self.string).unwrap(),
            string: self.string.clone()}
    }
}

impl MyXPath {
    fn new(xpath: String) -> Result<MyXPath> {
        return Ok (
            MyXPath {
                xpath: MyXPath::compile_xpath(&xpath)?,
                string: xpath,
            } 
        );
    }

    fn build(xpath: &Yaml) -> Result<MyXPath> {
        let xpath = match xpath {
            Yaml::String(s) => s.to_string(),
            Yaml::Integer(i) => i.to_string(),
            Yaml::Real(s) => s.to_string(),
            Yaml::Boolean(s) => s.to_string(),
            Yaml::Array(v) =>
                // array of strings -- concatenate them together
                v.iter()
                    .map(|str| as_str_checked(str))
                    .collect::<Result<Vec<&str>>>()?
                    .join(" "),
            _ => bail!("Bad value when trying to create an xpath: {}", yaml_to_string(xpath, 1)),
        };
        return MyXPath::new(xpath);
    }

    fn compile_xpath(xpath: &str) -> Result<XPath> {
        let factory = Factory::new();
        //let xpath = convert_hex(xpath);
        let xpath_with_debug_info = add_debug_string_arg(xpath)?;
        let xpath = factory.build(&xpath_with_debug_info)
                        .chain_err(|| format!(
                            "Could not compile XPath for pattern:\n{}{}",
                            &xpath, more_details(&xpath)))?;
        return Ok(xpath.unwrap());

        fn add_debug_string_arg(xpath: &str) -> Result<String> {
            // Find all the DEBUG(...) commands in 'xpath' and adds a string argument.
            // The DEBUG function that is used internally takes two arguments, the second one being a string version of the DEBUG arg.
            //   Being a string, any quotes need to be escaped, and DEBUGs inside of DEBUGs need more escaping.
            //   This is done via recursive calls to this function.
            let debug_start = xpath.find("DEBUG(");
            if debug_start.is_none() {
                return Ok( xpath.to_string() );
            }
            let debug_start = debug_start.unwrap();
            let string_start = xpath[..debug_start+6].to_string();   // includes "DEBUG("
            let mut count = 1;  // open/close count -- starting after "(" in "DEBUG("
            let mut remainder: &str = &xpath[debug_start+6..];
             
            loop {
                let next = remainder.find(|c| c=='(' || c==')');
                match next {
                    None => bail!("Did not find closing paren for DEBUG in\n{}", xpath),
                    Some(i_paren) => {
                        if remainder.as_bytes()[i_paren] == b'(' {
                            count += 1;
                        } else {            // must be ')'
                            count -= 1;
                            if count == 0 {
                                let i_end = xpath.len() - remainder.len() + i_paren; 
                                let escaped_arg = &xpath[debug_start+6..i_end].to_string().replace("\"", "\\\"");
                                let contents = add_debug_string_arg(&xpath[debug_start+6..i_end])?;
                                return Ok( string_start + &contents + ", \"" + &escaped_arg + "\" "
                                             + &add_debug_string_arg(&xpath[i_end..])? );
                            }
                        }
                        remainder = &remainder[i_paren+1..];
                    }
                }
            }
        }
        
        fn more_details(xpath: &str) -> String {
            // try to give a better error message by counting [], (), 's, and "s
            let as_bytes = xpath.trim().as_bytes();
            if as_bytes[0] == b'\'' && as_bytes[as_bytes.len()-1] != b'\'' {
                return "\nmissing \"'\"".to_string();
            }
            if (as_bytes[0] == b'"' && as_bytes[as_bytes.len()-1] != b'"') ||
               (as_bytes[0] != b'"' && as_bytes[as_bytes.len()-1] == b'"'){
                return "\nmissing '\"'".to_string();
            }

            let mut paren_count = 0;    // counter to make sure they are balanced
            let mut i_paren = 0;      // position of the outermost open paren
            let mut bracket_count = 0;
            let mut i_bracket = 0;
            for (i, ch) in xpath.chars().enumerate() {
                if ch == '(' {
                    if paren_count == 0 {
                        i_paren = i;
                    }
                    paren_count += 1;
                } else if ch == '[' {
                    if bracket_count == 0 {
                        i_bracket = i;
                    }
                    bracket_count += 1;
                } else if ch == ')' {
                    if paren_count == 0 {
                        return format!("\nExtra ')' found after '{}'", &xpath[i_paren..i]);
                    }
                    paren_count -= 1;
                    if paren_count == 0 && bracket_count > 0 && i_bracket > i_paren {
                        return format!("\nUnclosed brackets found at '{}'", &xpath[i_paren..i]);
                    }
                } else if ch == ']' {
                    if bracket_count == 0 {
                        return format!("\nExtra ']' found after '{}'", &xpath[i_bracket..i]);
                    }
                    bracket_count -= 1;
                    if bracket_count == 0 && paren_count > 0 && i_paren > i_bracket {
                        return format!("\nUnclosed parens found at '{}'", &xpath[i_bracket..i]);
                    }
                }
            }
            return "".to_string();
        }
    }


    fn is_true(&self, mathml: &Element) -> Result<bool> {
        // return true if there is no condition or if the condition evaluates to true
        return Ok(
            match self.evaluate(mathml)? {
                Value::Boolean(b) => b,
                Value::Nodeset(nodes) => nodes.size() > 0,
                _                      => false,      
            }
        )
    }

    fn replace(&self, rules: &SpeechRules, mathml: &Element) -> Result<String> {
        let result = self.evaluate(mathml)
                .chain_err(||"replacing after pattern match" )?;
        let answer;
        match result {
            Value::Nodeset(nodes) => {
                if nodes.size() == 0 {
                    bail!("During replacement, no matching element found");
                }
                return rules.replace_nodes(nodes, mathml);
            },
            Value::String(t) => { return rules.replace_chars(&t, mathml); },
            Value::Number(num) => { answer = num.to_string(); },
            Value::Boolean(b) => { answer = b.to_string(); },          // FIX: is this right???
        }
        return Ok( answer );
    }
    
    fn evaluate<'a,'c>(&'a self, mathml: &'c Element) -> Result<Value<'c>> {
        return CONTEXT_STACK.with(|context_stack| {
            let context_stack = context_stack.borrow();
            let context = context_stack.top();
            // println!("evaluate: {}", self);
            let result = self.xpath.evaluate(&context, *mathml);
            return match result {
                Ok(val) => Ok( val ),
                Err(e) => {
                    bail!( "{}\n\n",
                    e.to_string()           // remove confusing parts of error message from xpath
                    .replace("OwnedPrefixedName { prefix: None, local_part:", "")
                    .replace(" }", "") );    
                }
            };
        });
    }
}

// Used for speech rules with "variables: ..."
#[derive(Debug)]
struct VariableDefinition{
    name: String,  // name of variable
    value: Yaml,   // value, typically a constant like "true" or "0", but could be "*/*[1]" to store some nodes   
}

impl fmt::Display for VariableDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{{name: {}, value: {}}}", self.name, yaml_to_string(&self.value, 0));
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
    pattern: MyXPath,               // the xpath expr to attempt to match
    var_defs: PreferenceHashMap,    // any variable definitions [can be and probably is an empty vector most of the time]
    replacements: ReplacementArray, // the replacements in case there is a match
}

impl fmt::Display for SpeechPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{{name: {}, tag: {}, pattern: {}, replacement: {}}}",
                self.pattern_name, self.tag_name, self.pattern,
                self.replacements.pretty_print_replacements());
    }
}

impl SpeechPattern  {
    fn build(dict: &Yaml, file_name: &str) -> Result<Vec<SpeechPattern>> {
        // Rule::SpeechPattern
        // build { "pattern_name", "tag_name", "pattern", "replacement" }
        // println!("\nbuild_speech_pattern: dict:\n{}", yaml_to_string(dict, 0));
        let pattern_name = find_str(dict, "name");

        // tag_named can be either a string (most common) or an array of strings
        let mut tag_names: Vec<&str> = Vec::new();
        match find_str(dict, "tag") {
            Ok(str) => tag_names.push(str),
            Err(_) => {
                // check for array
                let tag_array  = &dict["tag"];
                tag_names = vec![];
                if tag_array.is_array() {
                    let mut i = 0;
                    for name in tag_array.as_vec().unwrap() {
                        match as_str_checked(name) {
                            Err(e) => return Err(
                                e.chain_err(||
                                    format!("tag name '{}' is not a string in:\n{}",
                                        &yaml_to_string(&tag_array.as_vec().unwrap()[i], 0),
                                        &yaml_to_string(dict, 1)))
                            ),
                            Ok(str) => tag_names.push(str),
                        };
                        i += 1;
                    }
                } else {
                    bail!("Errors trying to find 'tag' in:\n{}", &yaml_to_string(dict, 1));
                }
            }
        }

        if pattern_name.is_err() {
            if dict.is_null() {
                bail!("Error trying to find 'name': empty value (two consecutive '-'s?");
            } else {
                bail!("Errors trying to find 'name' in:\n{}", &yaml_to_string(dict, 1));
            };
        };
        let pattern_name = pattern_name.unwrap();

        // FIX: add check to make sure tag_name is a valid MathML tag name
        if dict["match"].is_badvalue() {
            bail!("Did not find 'match' in\n{}", yaml_to_string(dict, 1));
        }
        if dict["replace"].is_badvalue() {
            bail!("Did not find 'replace' in\n{}", yaml_to_string(dict, 1));
        }
    
        // xpath's can't be cloned, so we need to do a 'build_xxx' for each tag name
        let mut patterns = Vec::new();
        for tag_name in tag_names {
            patterns.push(
                SpeechPattern{
                    pattern_name: pattern_name.to_string(),
                    tag_name: tag_name.to_string(),
                    file_name: file_name.to_string(),
                    pattern: MyXPath::build(&dict["match"])
                        .chain_err(|| {
                            format!("value for 'match' in rule ({}: {}):\n{}",
                                    tag_name, pattern_name, yaml_to_string(dict, 1))
                        })?,
                    var_defs: ContextStack::build(&dict["variables"])
                        .chain_err(|| {
                            format!("value for 'variables' in rule ({}: {}):\n{}",
                                    tag_name, pattern_name, yaml_to_string(dict, 1))
                        })?, 
                    replacements: ReplacementArray::build(&dict["replace"])
                        .chain_err(|| {
                            format!("value for 'replace' in rule ({}: {}). Replacements:\n{}",
                                    tag_name, pattern_name, yaml_to_string(&dict["replace"], 1))
                    })?,
                }
            )
        }

        return Ok( patterns );
    }
    
    fn is_match(&self, mathml: &Element) -> Result<bool> {
        if self.tag_name != mathml.name().local_part() && self.tag_name != "unknown" {
            return Ok( false );
        }

        // println!("\nis_match: pattern='{}'", self.pattern_name);
        // println!("    pattern_expr {:?}", self.pattern_expr);
        // print!("is_match: mathml is\n{}", crate::pretty_print::mml_to_string(mathml));
        return Ok(
            match self.pattern.evaluate(mathml)? {
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
struct Test {
    condition: MyXPath,
    then_part: ReplacementArray,
    else_part: Option<ReplacementArray>,
    else_if_part: Option<Box<Test>>,
}
impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "test: {{ ")?;
        write!(f, "  if: '{}'", self.condition)?;
        write!(f, "  then '{}'", self.then_part)?;
        if let Some(else_part) = &self.else_part {
            write!(f, "  else: '{}'", else_part)?;
        }
        return write!(f, "}}");
    }
}

impl Test {
    fn build(test: &Yaml) -> Result<Box<Test>> {
        // 'test:' -- 'if': xxx 'then': xxx optional 'else': xxx
        if test.as_hash().is_none() {
            bail!("")
        }
        let if_part = &test["if"];
        if if_part.is_badvalue() { 
            bail!("Missing 'if' as part of 'test'.\n    \
                  Suggestion: add 'if:' or if present, indent so it is contained in 'test'");
        }

        let then_part = &test["then"];
        if then_part.is_badvalue() { 
            bail!("Missing 'then' as part of 'test'.\n    \
                  Suggestion: add 'then:' or if present, indent so it is contained in 'test'");
        }
        
        // at most one of 'else:' or 'else_if:' should be present
        let else_part = &test["else"];
        let else_if_part = &test["else_if"];
        let num_entries = test.as_hash().unwrap().len();
        // error if more than 3 entries or exactly 3 entries and third entry isn't 'else' or 'else_if'
        if num_entries > 3 ||
           (num_entries == 3 && else_part.is_badvalue() && else_if_part.is_badvalue()) {
            bail!("A key other than 'if', 'then', 'else', or 'else_if' was found in 'test'");
        }

        return Ok( Box::new( Test {
            condition: MyXPath::build(if_part)?,
            then_part: ReplacementArray::build(then_part).chain_err(|| "'then:'")?,
            else_part: if else_part.is_badvalue() {
                    None
                } else {
                    Some( ReplacementArray::build(else_part).chain_err(|| "'else:'")? )
                },
            else_if_part: if else_if_part.is_badvalue() {
                    None
                } else {
                    Some( Test::build(else_if_part).chain_err(|| "'else_if:'")? )
                },
        } ) );
    }

    fn replace(&self, rules: &SpeechRules, mathml: &Element) -> Result<String> {
        // println!("in replace, testing condition: \"{}\"", replacement.condition);
        if self.condition.is_true(mathml)
                    .chain_err(||"Failure in conditional test")? {
            //println!("..in replace: {:?}", self.replacement);
            return self.then_part.replace(rules, mathml);
        } else if let Some(else_part) = &self.else_part {
            return else_part.replace(rules, mathml);
        } else if let Some(else_if_part) = &self.else_if_part {
            return else_if_part.replace(rules, mathml);
        } else {
            // println!("... replace returns '{}'", speech_string);
            return Ok("".to_string());
        }
    }

}


struct ContextStack<'c>{
    // FIX: this really should just clone the top of the stack and add on new vars.
    //   However, Context does not support 'clone' because the functions it stores are traits, not types
    // Instead, we recreate the base each time and we store the vars to add in a stack (yuck!)
    // Fortunately, adding new vars is rare
    new_defs: Vec<PreferenceHashMap>,
    contexts: Vec<Rc<Context<'c>>>,
}

impl<'c> ContextStack<'static>{
    fn init<'d>(&mut self, pref_manager: &'d PreferenceManager) {
        let prefs = pref_manager.merge_prefs();
        let context = ContextStack::base_context(&prefs);
        self.new_defs.push(prefs);
        self.contexts.push(context);
    }

    fn base_context<'a>(var_defs: &'a PreferenceHashMap) -> Rc<Context<'c>> {
        let mut context  = Context::new();
        context.set_namespace("m", "http://www.w3.org/1998/Math/MathML");
        crate::xpath_functions::add_builtin_functions(&mut context);
        for (key, value) in var_defs {
            context.set_variable(key.as_str(), yaml_to_value(value));
        };
        return Rc::new( context );
    }

    fn new_def(name_value_def: &Yaml) -> Result<(String, Yaml)> {
        match name_value_def.as_hash() {
            Some(map) => {
                if map.len() != 1 {
                    bail!("definition is not a key/value pair. Found {}",
                            yaml_to_string(name_value_def, 1) );
                }
                let (name, value) = map.iter().next().unwrap();
                let name = as_str_checked( &name)
                    .chain_err(|| format!( "definition name is not a string: {}",
                            yaml_to_string(name, 1) ))?.to_string();
                match value {
                    Yaml::Boolean(_) | Yaml::String(_)  | Yaml::Integer(_) | Yaml::Real(_) => (),
                    _ => bail!("definition value is not a string, boolean, or number. Found {}",
                            yaml_to_string(value, 1) )
                };
                return Ok( (name, value.to_owned()) );
            },
            None => bail!("definition is not a key/value pair. Found {}",
                            yaml_to_string(name_value_def, 1) )
        }
    }

    fn build(defs: &Yaml) -> Result<PreferenceHashMap> {
        if defs.is_badvalue() {
            return Ok( HashMap::with_capacity(0) );
        };
        if defs.is_array() {
            let mut definitions = HashMap::new();
            for def in defs.as_vec().unwrap() {
                let (name, value) = ContextStack::new_def(def)
                        .chain_err(|| "definition of 'variables'")?;
                definitions.insert(name, value);
            };
            return Ok (definitions );
        }
        bail!( "'variables' is not an array of {{name: xpath-value}} definitions. Found {}'",
                yaml_to_string(defs, 1) );
    }

    fn push(&mut self, var_defs: &PreferenceHashMap) {
        self.new_defs.push(var_defs.clone());       // do first so they are included in context
        let mut context  = Context::new();
        context.set_namespace("m", "http://www.w3.org/1998/Math/MathML");
        crate::xpath_functions::add_builtin_functions(&mut context);
        for defs in &self.new_defs {
            for (name, value) in defs {
                context.set_variable(name.as_str(), yaml_to_value(&value));
            };            
        }
        self.contexts.push(Rc::new( context ));
    }

    fn pop(&mut self) {
        self.new_defs.pop();
        self.contexts.pop();
    }

    fn top<'a>(&self) -> Rc<Context<'a>>{
        return Rc::clone(&self.contexts[self.contexts.len()-1] );
    }
}


fn yaml_to_value<'a, 'b>(yaml: &'a Yaml) -> Value<'b> {
    return match yaml {
        Yaml::String(s) => Value::String(s.clone()),
        Yaml::Boolean(b)  => Value::Boolean(*b),
        Yaml::Integer(i)   => Value::Number(*i as f64),
        Yaml::Real(s)   => Value::Number(s.parse::<f64>().unwrap()),
        _  => {
            eprintln!("yaml_to_value: illegal type found in Yaml value: {}", yaml_to_string(yaml, 1));
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
    fn build(unicode_def: &Yaml) -> Result<UnicodeDef> {
        // key: char, value is replacement or array of replacements
        let dictionary = unicode_def.as_hash();
        if dictionary.is_none() {
            bail!("Expected a unicode definition (e.g, '+':{{t: plus}}'), found {}", yaml_to_string(unicode_def, 0));
        }
        let dictionary = dictionary.unwrap();
        assert!(dictionary.len()==1);
        let (ch, replacements) = dictionary.iter().next().ok_or_else(||  format!("Expected a unicode definition (e.g, '+':{{t: plus}}'), found {}", yaml_to_string(unicode_def, 0)))?;
        let ch = UnicodeDef::get_unicode_char(ch)?;
        return Ok( UnicodeDef{
            ch,
            speech: ReplacementArray::build(replacements)?
        });
    }
    
    fn get_unicode_char(ch: &Yaml) -> Result<u32> {
        // either "a" or 0x1234 (number)
        let ch_as_str = ch.as_str();
        if let Some(ch) = ch_as_str {
            let mut ch_iter = ch.chars();
            let unicode_ch = ch_iter.next();
            if unicode_ch.is_none() || ch_iter.next().is_some() {
                bail!("Wanted unicode char, found string {}", ch);
            };
            return Ok( unicode_ch.unwrap() as u32 );
        }
    
        if let Some(num) = ch.as_i64() {
            return Ok( num as u32 );
        }
        bail!("Unicode character '{}' can't be converted to an code point", yaml_to_string(ch, 0));
    }    
}

/// Print out the errors to `stderr`.
///
/// Useful functionality that had to had a home in some crate, so it is here.
pub fn print_errors(e:&Error) {
    use std::io::Write;
    let stderr = &mut ::std::io::stderr();
    let error_message = "Error writing to stderr";

    writeln!(stderr, "\nError: {}", e).expect(error_message);

    for e in e.iter().skip(1) {
        writeln!(stderr, "caused by: {}", e).expect(error_message);
    }

    // The backtrace is not always generated. Try to run this example
    // with `RUST_BACKTRACE=1`.
    // if let Some(backtrace) = e.backtrace() {
    //     writeln!(stderr, "backtrace: {:?}", backtrace).expect(error_message);
    // }
}


// Fix: there should be a cache so subsequent library calls don't have to read in the same speech rules
//   likely a cache of size 1 is fine
// Fix: all statics should be gathered together into one structure that is a Mutex
//   for each library call, we should grab a lock on the Mutex in case others try to call
//   at the same time.
//   If this turns out to be something that others actually do, then a cache > 1 would be good

 type RuleTable = HashMap<String, Vec<Box<SpeechPattern>>>;
 type UnicodeTable = HashMap<u32,Vec<Replacement>>;

/// `SpeechRules` encapsulates a named group of speech rules (e.g, "ClearSpeak")
/// along with the preferences to be used for speech.
pub struct SpeechRules{
    name: String,
    pub pref_manager: Box<PreferenceManager>,
    rules: RuleTable,           // the speech rules used (partitioned into MathML tags in hashmap, then linearly searched)
    unicode: UnicodeTable,      // the speech rules used for Unicode characters
}

impl fmt::Display for SpeechRules{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SpeechRules({}, {})", self.name, self.pref_manager)?;
        for (tag_name, rules) in &self.rules {
            writeln!(f, "   {} speech patterns for {}", rules.len(), tag_name)?;
        };
        return writeln!(f, "   {} unicode entries", &self.unicode.len());
    }
}

thread_local!{
    /// The current set of speech rules
    // maybe this should be a small cache of rules in case people switch rules/prefs?
    pub static SPEECH_RULES: RefCell<SpeechRules> =
            RefCell::new( SpeechRules::new("initial") );

    static CONTEXT_STACK: RefCell<ContextStack<'static>> = RefCell::new( ContextStack{ new_defs: vec![], contexts: vec![] } );
}

impl SpeechRules {
    fn new(name: &str) -> SpeechRules {
        let pref_manager = PreferenceManager::new();
        if let Err(e) = crate::definitions::read_definitions_file(pref_manager.get_definitions_file()) {
            print_errors(&e);
            exit(1);
        };
        let rules = SpeechRules{
            name: String::from(name),
            rules: SpeechRules::read_patterns(pref_manager.get_style_file()),  // rules before prefs to borrow/move problem
            unicode: SpeechRules::read_unicode(pref_manager.get_unicode_file()),
            pref_manager: pref_manager,
        };
        return rules;
    }

    fn read_patterns(path: &Locations) -> RuleTable {
        // FIX: should read first (lang), then supplement with second (region)
        use std::fs;
        if let Some(p) = &path[0] {
            let rule_file_contents = fs::read_to_string(p).expect("cannot read file");
            let mut hashmap: RuleTable = HashMap::new();
            let rules_build_fn = |pattern: &Yaml| {
                match SpeechRules::build_speech_patterns(pattern, p.to_str().unwrap()) {
                    Ok(patterns) => { hashmap = patterns; },
                    Err(e)       => { print_errors(&e.chain_err(||format!("in file {:?}", p.to_str().unwrap()))); }
                }    
            };
            if let Err(e) = compile_rule(&rule_file_contents, rules_build_fn) {
                print_errors(&e.chain_err(||format!("in file {:?}", p.to_str().unwrap())));
            }
            return hashmap;    
        } else {
            return HashMap::new();
        }
    }

    fn build_speech_patterns(patterns: &Yaml, file_name: &str) -> Result<RuleTable> {
        // Rule::SpeechPatternList
        let mut rules: RuleTable = HashMap::with_capacity(31);
        let patterns_vec = patterns.as_vec();
        if patterns_vec.is_none() {
            bail!(yaml_type_err(patterns, "array"));
        }
        let patterns_vec = patterns.as_vec().unwrap();

        for (i, entry) in patterns_vec.iter().enumerate() {
            let built_patterns = SpeechPattern::build(entry, file_name);
            if let Err(e) = built_patterns {
                if e.description().contains("name:") {
                    return Err(e);
                } else {
                    return Err( e.chain_err(|| format!("rule #{}. Previous rule is:\n{}",
                        i+1, 
                        if i==0 {"none".to_string()} else {yaml_to_string(&patterns_vec[i-1], 1)}))
                    );
                }
            }

            for pattern in built_patterns.unwrap() {
                // println!("Pattern = {}", pattern);
                let pattern = Box::new( pattern );
                let value = rules.entry(pattern.tag_name.clone()).or_insert( Vec::new());
                value.push(pattern);
            }
        }
        return Ok(rules);  
    }
    
        fn read_unicode(path: &Locations) -> UnicodeTable {
        // FIX: should read first (lang), then supplement with second (region)
        if let Some(p) = &path[0] {
            use std::fs;    
            let unicode_file_contents = fs::read_to_string(p).expect("cannot read file");
            let mut unicode_chars: UnicodeTable = HashMap::with_capacity(6997);
            let unicode_build_fn = |unicode_def_list: &Yaml| {
                let unicode_defs = unicode_def_list.as_vec();
                if unicode_defs == None {
                   format!("File '{}' does not being with an array", yaml_to_type(unicode_def_list));
                };
                for unicode_def in unicode_defs.unwrap() {
                    match UnicodeDef::build(unicode_def) {
                        Ok(def) => { unicode_chars.insert(def.ch, def.speech.replacements); },
                        Err(e)       => { print_errors(&e.chain_err(|| {
                            format!("In file {:?}", p)
                        })); }
                    }        
                }
            };
            if let Err(e) =compile_rule(&unicode_file_contents, unicode_build_fn) {
                print_errors(&e.chain_err(||format!("in file {:?}", p.to_str().unwrap())));
            }
            println!("Hashmap has {} entries", unicode_chars.len());
            return unicode_chars;  
        }
        return HashMap::with_capacity(1);
    }

    fn match_pattern(&self, mathml: &Element) -> Result<String> {
        // println!("Looking for a match for: \n{}", crate::pretty_print::mml_to_string(mathml));
        let mut tag_name = mathml.name().local_part();
        if !self.rules.contains_key(tag_name) {
            tag_name = "unknown";       // should be rules for 'unknown'
        }
        let rule_value = self.rules.get(tag_name);
        match rule_value {
            Some(rule_vector) => {
                for pattern in rule_vector {
                    // println!("Pattern: {}", pattern);
                    if pattern.is_match(mathml)
                        .chain_err(||
                            format!(
                                "error during pattern match using: \"{}\" for \"{}\".\n\
                                Pattern is \n{}\nMathML for the match:\n\
                                {}\
                                The patterns are in {}.\n",
                                pattern.pattern_name, pattern.tag_name,
                                pattern.pattern,
                                pretty_print::mml_to_string(mathml),
                                pattern.file_name
                            )
                    )? {
                        if pattern.var_defs.len() > 0 {
                            CONTEXT_STACK.with(|context_stack| {
                                let mut context = context_stack.borrow_mut();
                                context.push(&pattern.var_defs);
                            });
                        }
                        let result = pattern.replacements.replace(self, mathml);
                        if pattern.var_defs.len() > 0 {
                            CONTEXT_STACK.with(|context_stack| {
                                let mut context = context_stack.borrow_mut();
                                context.pop();
                            });
                        }
                        return result.chain_err(||
                                format!(
                                    "attempting replacement pattern: \"{}\" for \"{}\".\n\
                                    Replacement \"{}\" due to matching the following MathML with the pattern \"{}\".\n\
                                    {}\
                                    The patterns are in {}.\n",
                                    pattern.pattern_name, pattern.tag_name,
                                    pattern.replacements.pretty_print_replacements(),pattern.pattern,
                                    pretty_print::mml_to_string(mathml),
                                    pattern.file_name
                                )
                            );
                    }
                }
            },
            None => {
                // unknown element -- should have rules to handle this -- let flow through to default error
            }    
        }
        let mut file_name = "unknown";
        if let Some(path) = &self.pref_manager.get_style_file()[0] {
            file_name= path.to_str().unwrap();
        }
        if self.rules.get("math").is_none() {
            bail!("No rules found for any matches!!! See the error log.");
        } else {
            // FIX: handle error appropriately
            bail!("\nNo match found!\nMissing patterns in {} or bad MathML.\n{}", file_name, crate::pretty_print::mml_to_string(mathml)); 
        }
    }
    
    fn replace(&self, replacement: &Replacement, mathml: &Element) -> Result<String> {
        return Ok(
            match &*replacement {
                Replacement::Text(t) => t.trim().to_string(),
                Replacement::XPath(path) => path.replace(&self, mathml)?,
                Replacement::TTS(tts) => {
                    self.pref_manager.get_tts().replace(&tts, &self.pref_manager, self, mathml)?
                },
                Replacement::Test(test) => {
                    test.replace(self, mathml)?                     
                },
                Replacement::Insert(ic) => {
                    ic.replace(self, mathml)?                     
                },
            }
        )
    }

    fn replace_nodes(&self, nodes: nodeset::Nodeset, mathml: &Element) -> Result<String> {
        //println!("replace_nodes: working on {} nodes", nodes.size());
        let result = nodes.document_order()
            .iter()
            .map(|node|
                match node {
                    Node::Element(mathml) => self.match_pattern(&mathml),
                    Node::Text(t) =>  self.replace_chars(&t.text(), mathml),
                    _ => {eprintln!("replace_nodes: found unexpected node type!!! (ignored)"); Ok( "".to_string() )}
                })
            .collect::<Result<Vec<String>>>()?
            .join(" ");
        return Ok( result );
    }

    fn replace_chars(&self, str: &str, mathml: & Element) -> Result<String> {
        // Lookup unicode "pronunciation" of char
        // This is only done for a single char; longer strings are untouched.
        // Note: TTS is not supported here (not needed and a little less efficient)
        let mut chars = str.chars();
        let ch_as_u32 = chars.next().unwrap_or(' ') as u32;
        if chars.next().is_some() {
            // more than one char (don't use str.len() since that is bytes, not chars)
            return Ok(String::from(str));
        };
        let replacements = self.unicode.get( &ch_as_u32 );
        if replacements.is_none() {
            return Ok(String::from(str));   // no replacement, so just return the char and hope for the best
        };

        // map across all the parts of the replacement, collect them up into a Vec, and then concat them together
        return Ok(
            replacements.unwrap()
                           .iter()
                           .map(|replacement|
                                self.replace(replacement, mathml)
                                    .chain_err(|| format!("Unicode replacement error: {}", replacement)) )
                           .collect::<Result<Vec<String>>>()?
                           .join(" ")
        );
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_statement() {
        let str = r#"---
        {name: default, tag: math, match: ".", replace: [x: "./*"] }"#;
        let doc = YamlLoader::load_from_str(str).unwrap();
        assert_eq!(doc.len(), 1);

        let speech_patterns = SpeechPattern::build(&doc[0], "testing").unwrap();
        let speech_pattern = &speech_patterns[0];
        assert_eq!(speech_pattern.pattern_name, "default", "\npattern name failure");
        assert_eq!(speech_pattern.tag_name, "math", "\ntag name failure");
        assert_eq!(speech_pattern.pattern.string, ".", "\npattern failure");
        assert_eq!(speech_pattern.replacements.replacements.len(), 1, "\nreplacement failure");
        assert_eq!(speech_pattern.replacements.replacements[0].to_string(), r#"{x: "./*"}"#, "\nreplacement failure");
    }
}