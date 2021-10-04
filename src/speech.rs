//! The speech module is where the speech rules are read in and speech generated.
//!
//! The main external call, [`speak_mathml`] returns a string for the speech associated with the `mathml`
//!   param based on the user preferences.
//!
//! The speech rules call out to the preferences and tts modules and the dividing line is not always clean.
//! A number of useful utility functions used by other modules are defined here.
#![allow(clippy::needless_return)]

use std::{collections::HashMap, process::exit};
use std::cell::RefCell;
use sxd_document::dom::Element;
use sxd_document::QName;
use sxd_xpath::context::Evaluation;
use sxd_xpath::{Context, Factory, Value, XPath, nodeset};
use sxd_xpath::nodeset::Node;
use std::fmt;
use crate::{errors::*, pretty_print};
use crate::prefs::*;
use yaml_rust::{YamlLoader, Yaml, yaml::Hash};
use crate::tts::*;
use crate::pretty_print::yaml_to_string;
use std::path::Path;
use regex::{Regex, Captures};
use phf::phf_map;


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
pub fn speak_mathml(mathml: Element) -> String {
    SPEECH_RULES.with(|rules| {
        {
            let mut mut_rules = rules.borrow_mut();
            mut_rules.update(true);    
        }
        let rules = rules.borrow();
        let mut rules_with_context = SpeechRulesWithContext::new(&rules);
        match rules_with_context.match_pattern(&mathml) {
            Ok(speech_string) => {
                return rules.pref_manager.get_tts()
                            .merge_pauses(remove_optional_indicators(
                                &speech_string.replace(CONCAT_STRING, "")
                                                  .replace(CONCAT_INDICATOR, "")                            
                                          )
                            .trim());
            },
            Err(e)             => { 
                print_errors(&e.chain_err(|| "Pattern match/replacement failure!"));
                return String::from("Error in speaking math; see error log.")
            }
        }
    })
}

pub fn braille_mathml(mathml: Element) -> String {
    BRAILLE_RULES.with(|rules| {
        {
            let mut mut_rules = rules.borrow_mut();
            mut_rules.update(false);    
        }
        let rules = rules.borrow();
        let mut rules_with_context = SpeechRulesWithContext::new(&rules);
        match rules_with_context.match_pattern(&&mathml) {
            // FIX: need to set name of speech rules so test Nemeth/UEB clean for
            Ok(speech_string) => {
                return nemeth_cleanup(speech_string.replace(" ", ""));
            },
            Err(e)             => { 
                print_errors(&e.chain_err(|| "Pattern match/replacement failure!"));
                return String::from("Error in speaking math; see error log.")
            }
        }
    })
}

fn nemeth_cleanup(raw_nemeth: String) -> String {
    // Typeface: S: sans-serif, B: bold, T: script/blackboard, I: italic, R: Roman
    // Language: E: English, D: German, G: Greek, V: Greek variants, H: Hebrew, U: Russian
    // Indicators: C: capital, N: number, P: punctuation, M: multipurpose
    // Others:
    //      W -- whitespace that should be kept (e.g, in a numeral)
    //      𝑁 -- hack for special case of a lone decimal pt -- not considered a number but follows rules mostly 
    // SRE doesn't have H: Hebrew or U: Russian, so not encoded (yet)
    // Note: some "positive" patterns find cases to keep the char and transform them to the lower case version
    static INDICATOR_REPLACEMENTS: phf::Map<&str, &str> = phf_map! {
        "S" => "⠈⠰",
        "B" => "⠸",
        "T" => "⠈",
        "I" => "⠨",
        "R" => "",
        "E" => "⠰",
        "D" => "⠸",
        "G" => "⠨",
        "V" => "⠨⠈",
        "H" => "⠠⠠",
        "U" => "⠈⠈",
        "C" => "⠠",
        "P" => "⠸",
        "M" => "",
        "m" => "⠐",
        "b" => "⠐",
        "N" => "",
        "n" => "⠼",
        "𝑁" => "",
        "W" => "⠀",
        "," => "⠠⠀"
    };

    lazy_static! {
        // Trim braille spaces before and after braille indicators
        // FIX: these lists are not complete
        // ellipsis, dashes, {parens, brackets, braces}
        static ref REMOVE_SPACE_AFTER_PARENS: Regex = 
            Regex::new(r"(⠷)⠀+").unwrap();
        // In order: fraction, /, cancellation, capitalization, baseline
        static ref REMOVE_SPACE_BEFORE_PARENS: Regex = 
            Regex::new(r"⠀+(⠾)|(⠈⠾)|(⠨⠾)").unwrap();

        // In order: fraction, /, cancellation, capitalization, baseline
        static ref REMOVE_SPACE_BEFORE_BRAILLE_INDICATORS: Regex = 
            Regex::new(r"(⠄⠄⠄)|(⠤⠤⠤)⠀+([⠼⠸⠌⠪])").unwrap();
        static ref REMOVE_SPACE_AFTER_BRAILLE_INDICATORS: Regex = 
            Regex::new(r"([⠹⠌⠻C⠐])⠀+(⠄⠄⠄)").unwrap();

        // Multipurpose indicator insertion
        // 177.2 -- add after a letter and before a digit (or decimal pt) -- these will start with N
        static ref MULTI_177_2: Regex = 
            Regex::new(r"([⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵])[N𝑁]").unwrap();

        // keep between numeric subscript and digit ('M' added by subscript rule)
        static ref MULTI_177_3: Regex = 
            Regex::new(r"([N𝑁].)M([N𝑁].)").unwrap(); 

        // add after decimal pt for non-digits except for comma and punctuation
        // note: since "." can be in the middle of a number, there is not necessarily a "N"
        static ref MULTI_177_5: Regex = 
            Regex::new(r"([N𝑁]⠨)([^⠂⠆⠒⠲⠢⠖⠶⠦⠔N𝑁,P])").unwrap(); 


        // Pattern for rule II.9a (add numeric indicator at start of line or after a space) and 9a (add after typeface)
        // 1. start of line
        // 2. optional minus sign (⠤)
        // 3. optional typeface indicator
        // 4. number (N)
        static ref NUM_IND_9A: Regex = 
            Regex::new(r"(?P<start>^|[,⠀])(?P<minus>⠤?)(?P<face>[SBTIR]*?)N").unwrap();  

        // FIX  add rule 9d after section mark, etc

        // Needed after a typeface change
        static ref NUM_IND_9E: Regex = Regex::new(r"(?P<face>[SBTIR]+?)N").unwrap();  

        // Punctuation chars (Rule 38.6 says don't use before ",", "hyphen", "-", "…")
        // Never use punctuation indicator before these (38-6)
        //      "…": "⠀⠄⠄⠄"
        //      "-": "⠸⠤" (hyphen and dash)
        //      ",": "⠠⠀"     -- spacing already added
        // Rule II.9b (add numeric indicator after punctuation [optional minus[optional .][digit]
        //  because this is run after the above rule, some cases are already caught, so don't
        //  match if there is already a numeric indicator
        static ref NUM_IND_AFTER_PUNCT: Regex =
            Regex::new(r"(?P<punct>P.)(?P<minus>⠤?)N").unwrap();  

        // Except for the four chars above, the unicode rules always include a punctuation indicator.
        // The cases to remove them (that seem relevant to MathML) are:
        //   Beginning of line or after a space (V 38.1)
        //   After a word (38.4)
        //   2nd or subsequent punctuation (includes, "-", etc) (38.7)
        static ref REMOVE_PUNCT_IND: Regex =
            Regex::new(r"(^|⠀|\w)P(.)").unwrap();  

        static ref REPLACE_INDICATORS: Regex =Regex::new(r"([SBTIREDGVHPCMmbNn𝑁W,])").unwrap();  
            
        static ref REMOVE_LEVEL_IND_BEFORE_BASELINE: Regex = Regex::new(r"(?:[⠘⠰]+b)").unwrap();

        // Before 79b (punctuation)
        static ref REMOVE_LEVEL_IND_BEFORE_SPACE_COMMA_PUNCT: Regex = Regex::new(r"(?:[⠘⠰]+b?|b)([⠀,P]|$)").unwrap();

        static ref COLLAPSE_SPACES: Regex = Regex::new(r"⠀⠀+").unwrap();
    }

    println!("Before:  \"{}\"", raw_nemeth);
    // Remove unicode blanks at start and end
    let result = raw_nemeth.trim_start_matches('⠀').trim_end_matches('⠀');

    // Remove blanks before and after "parens"
    let result = REMOVE_SPACE_BEFORE_PARENS.replace_all(result, "$1$2$3");
    let result = REMOVE_SPACE_AFTER_PARENS.replace_all(&result, "$1");

    // Remove blanks before and after braille indicators
    let result = REMOVE_SPACE_BEFORE_BRAILLE_INDICATORS.replace_all(&result, "$1$2$3");
    let result = REMOVE_SPACE_AFTER_BRAILLE_INDICATORS.replace_all(&result, "$1$2");
    println!("spaces:  \"{}\"", result);

    // Multipurpose indicator
    let result = MULTI_177_2.replace_all(&result, "${1}m${2}");
    let result = MULTI_177_3.replace_all(&result, "${1}m$2");
    let result = MULTI_177_5.replace_all(&result, "${1}m$2");
    println!("MULTI:   \"{}\"", result);

    let result = NUM_IND_9A.replace_all(&result, "$start$minus${face}n");
    println!("IND_9A:  \"{}\"", result);

    let result = NUM_IND_9E.replace_all(&result, "${face}n");
    println!("IND_9E:  \"{}\"", result);

    // 9b: insert after punctuation (optional minus sign)
    // common punctuation adds a space, so 9a handled it. Here we deal with other "punctuation" 
    // FIX other punctuation and reference symbols (9d)
    let result = NUM_IND_AFTER_PUNCT.replace_all(&result, "$punct${minus}n");
    println!("A PUNCT: \"{}\"", &result);

    // strip level indicators
    // checks for punctuation char, so needs to before punctuation is stripped.
    
    let result = REMOVE_LEVEL_IND_BEFORE_SPACE_COMMA_PUNCT.replace_all(&result, "$1");
    println!("Punct  : \"{}\"", &result);
    let result = REMOVE_LEVEL_IND_BEFORE_BASELINE.replace_all(&result, "b");
    println!("Bseline: \"{}\"", &result);

    let result = REMOVE_PUNCT_IND.replace_all(&result, "$1$2");
    println!("Punct38: \"{}\"", &result);

    
    let result = REPLACE_INDICATORS.replace_all(&result, |cap: &Captures| {
        match INDICATOR_REPLACEMENTS.get(&cap[0]) {
            None => panic!("REPLACE_INDICATORS and INDICATOR_REPLACEMENTS are not in sync"),
            Some(&ch) => ch,
        }
    });

    let result = COLLAPSE_SPACES.replace_all(&result, "⠀");

    return result.to_string();
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

fn process_include<F>(current_file: &Path, new_file_name: &str, mut read_new_file: F) -> Result<()> where F: FnMut(&Path) {
    let parent_path = current_file.parent();
    if parent_path.is_none() {
        bail!("Internal error: {:?} is not a valid file name", current_file);
    }
    let mut new_file = parent_path.unwrap().to_path_buf();
    new_file.push(new_file_name);
    let new_file = match new_file.as_path().canonicalize() {
        Ok(buf) => buf,
        Err(msg) => bail!("-include: constructed file name '{}' causes error '{}'",
                                 new_file.to_str().unwrap(), msg),
    };

    read_new_file(new_file.as_path());
    return Ok( () );
}


// 'Replacement' is an enum that contains all the potential replacement types/structs
// Hence there are fields 'Test' ("test:"), 'Text" ("t:"), etc
#[derive(Debug, Clone)]
enum Replacement {
    // Note: all of these are pointer types
    Test(Box<TestArray>),
    Text(String),
    XPath(MyXPath),
    TTS(Box<TTSCommandRule>),
    With(Box<With>),
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
                Replacement::With(w) => w.to_string(),
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
            "pause" | "rate" | "pitch" | "volume" | "gender" | "voice" | "spell" => {
                return Ok( Replacement::TTS( TTS::build(key, value)? ) );
            },
            "test" => {
                return Ok( Replacement::Test( Box::new( TestArray::build(value)? ) ) );
            },
            "with" => {
                return Ok( Replacement::With( With::build(value)? ) );
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

impl<'r> InsertChildren {
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
    fn replace<'c, 's:'c>(&self, rules_with_context: &'r mut SpeechRulesWithContext<'c, 's>, mathml: &'r Element<'c>) -> Result<String> {
        let result = self.xpath.evaluate(&rules_with_context.context_stack.base, mathml)
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
                return replacements.replace(rules_with_context, mathml);
            },

            // FIX: should the options be errors???
            Value::String(t) => { return rules_with_context.replace_chars(&t, mathml); },
            Value::Number(num)  => { return Ok( num.to_string() ); },
            Value::Boolean(b)  => { return Ok( b.to_string() ); },          // FIX: is this right???
        }
        
    }    
}


// structure used when "insert:" is encountered in a rule
// the 'replacements' are inserted between each node in the 'xpath'
#[derive(Debug, Clone)]
struct With {
    variables: VariableDefinitions,     // variables and values
    replacements: ReplacementArray,     // what to do with these vars
}

impl fmt::Display for With {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "With:\n  variables {}\n  replacements {}", &self.variables, &self.replacements);
    }
}

impl<'r> With {
    fn build(insert: &Yaml) -> Result<Box<With>> {
        // 'with:' -- 'variables': xxx 'replace': xxx
        if insert.as_hash().is_none() {
            bail!("Array found for contents of 'with' -- should be dictionary with keys 'variables and 'replace'")
        }
        let var_defs = &insert["variables"];
        if var_defs.is_badvalue() { 
            bail!("Missing 'variables' as part of 'with'.\n    \
                  Suggestion: add 'variables:' or if present, indent so it is contained in 'with'");
        }
        let replace = &insert["replace"];
        if replace.is_badvalue() { 
            bail!("Missing 'replace' as part of 'with'.\n    \
                  Suggestion: add 'replace:' or if present, indent so it is contained in 'with'");
        }
        return Ok( Box::new( With {
            variables: VariableDefinitions::build(var_defs).chain_err(|| "'variables'")?,
            replacements: ReplacementArray::build(replace).chain_err(|| "'replace:'")?,
        } ) );
    }
        
    fn replace<'c, 's:'c>(&self, rules_with_context: &'r mut SpeechRulesWithContext<'c, 's>, mathml: &'r Element<'c>) -> Result<String> {
        rules_with_context.context_stack.push(self.variables.clone(), mathml)?;
        let result = self.replacements.replace(rules_with_context, mathml)
                    .chain_err(||"replacing inside 'with'")?;
        rules_with_context.context_stack.pop();
        return Ok( result );
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

impl<'r> ReplacementArray {
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
    pub fn replace<'c, 's:'c>(&self, rules_with_context: &'r mut SpeechRulesWithContext<'c, 's>, mathml: &'r Element<'c>) -> Result<String> {
        // do the replacements
        //   remove the empty strings (the later 'join' would add extraneous spaces)
        //  collect the strings together into an array
        let mut replacement_strings = Vec::with_capacity(32);   // probably conservative guess
        for replacement in self.replacements.iter() {
            let string = rules_with_context.replace(replacement, mathml)?;
            if !string.is_empty() {
                replacement_strings.push(string);
            }
        }

        // let mut replacement_strings =
        //     self.replacements.iter()
        //                 .map(|group| rules_with_context.replace(group, mathml))
        //                 .filter(|result| if let Ok(str) = result {!str.is_empty()} else {true})
        //                 .collect::<Result<Vec<String>>>()?;

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
                    &rules_with_context.speech_rules.pref_manager.get_tts().compute_auto_pause(&rules_with_context.speech_rules.pref_manager, before, after));
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
        return self.replacements.is_empty();
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

impl<'r> MyXPath {
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
        let xpath_with_debug_info = MyXPath::add_debug_string_arg(xpath)?;
        let xpath = factory.build(&xpath_with_debug_info)
                        .chain_err(|| format!(
                            "Could not compile XPath for pattern:\n{}{}",
                            &xpath, more_details(&xpath)))?;
        return Ok(xpath.unwrap());

        
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

    fn add_debug_string_arg(xpath: &str) -> Result<String> {
        // lazy_static! {
        //     static ref OPEN_OR_CLOSE_PAREN: Regex = Regex::new("^['\"][()]").unwrap();    // match paren that doesn't follow a quote
        // }
        // Find all the DEBUG(...) commands in 'xpath' and adds a string argument.
        // The DEBUG function that is used internally takes two arguments, the second one being a string version of the DEBUG arg.
        //   Being a string, any quotes need to be escaped, and DEBUGs inside of DEBUGs need more escaping.
        //   This is done via recursive calls to this function.
        // FIX: this doesn't handle parens in strings correctly -- it only catches the common case of quoted parens
        // FIX: to do this right, one has to be careful about escape chars, so it gets ugly for nesting
        let debug_start = xpath.find("DEBUG(");
        if debug_start.is_none() {
            return Ok( xpath.to_string() );
        }
        let debug_start = debug_start.unwrap();
        let string_start = xpath[..debug_start+6].to_string();   // includes "DEBUG("
        let mut count = 1;  // open/close count -- starting after "(" in "DEBUG("
        let mut remainder: &str = &xpath[debug_start+6..];
            
        loop {
            // println!("  add_debug_string_arg: count={}, remainder='{}'", count, remainder);
            let next = remainder.find(|c| c=='(' || c==')');
            match next {
                None => bail!("Did not find closing paren for DEBUG in\n{}", xpath),
                Some(i_paren) => {
                    let remainder_as_bytes = remainder.as_bytes();

                    // if the paren is inside of quote (' or "), don't count it
                    // FIX: this could be on a non-char boundary
                    if i_paren == 0 || remainder_as_bytes[i_paren-1] != b'\'' ||
                       i_paren+1 >= remainder.len() || remainder_as_bytes[i_paren+1] != b'\'' {
                        // println!("     found '{}'", remainder_as_bytes[i_paren].to_string());
                        if remainder_as_bytes[i_paren] == b'(' {
                            count += 1;
                        } else {            // must be ')'
                            count -= 1;
                            if count == 0 {
                                let i_end = xpath.len() - remainder.len() + i_paren; 
                                let escaped_arg = &xpath[debug_start+6..i_end].to_string().replace("\"", "\\\"");
                                let contents = MyXPath::add_debug_string_arg(&xpath[debug_start+6..i_end])?;
                                return Ok( string_start + &contents + ", \"" + &escaped_arg + "\" "
                                                + &MyXPath::add_debug_string_arg(&xpath[i_end..])? );
                            }
                        }    
                    }
                    remainder = &remainder[i_paren+1..];
                }
            }
        }
    }

    fn is_true(&self, context: &Context, mathml: &Element) -> Result<bool> {
        // return true if there is no condition or if the condition evaluates to true
        return Ok(
            match self.evaluate(context, mathml)? {
                Value::Boolean(b) => b,
                Value::Nodeset(nodes) => nodes.size() > 0,
                _                      => false,      
            }
        )
    }

    fn replace<'c, 's:'c>(&self, rules_with_context: &'r mut SpeechRulesWithContext<'c, 's>, mathml: &'r Element<'c>) -> Result<String> {
        let result = self.evaluate(&rules_with_context.context_stack.base, mathml)
                .chain_err(||"replacing after pattern match" )?;
        return match result {
                Value::Nodeset(nodes) => {
                    if nodes.size() == 0 {
                        bail!("During replacement, no matching element found");
                    }
                    rules_with_context.replace_nodes(nodes, mathml)
                },
                Value::String(t) => Ok( t ),
                Value::Number(num) => Ok( num.to_string() ),
                Value::Boolean(b) => Ok( b.to_string() ),          // FIX: is this right???
        }
    }
    
    fn evaluate<'a, 'c>(&'a self, context: &'r Context<'c>, mathml: &'r Element<'c>) -> Result<Value<'c>> {
        // println!("evaluate: {}", self);
        let result = self.xpath.evaluate(context, *mathml);
        return match result {
            Ok(val) => {
                // println!("  result: '{:?}'", val);
                Ok( val )
            },
            Err(e) => {
                bail!( "{}\n\n",
                e.to_string()           // remove confusing parts of error message from xpath
                .replace("OwnedPrefixedName { prefix: None, local_part:", "")
                .replace(" }", "") );    
            }
        };
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
    var_defs: VariableDefinitions,    // any variable definitions [can be and probably is an empty vector most of the time]
    replacements: ReplacementArray,       // the replacements in case there is a match
}

impl<'a> fmt::Display for SpeechPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{{name: {}, tag: {}, variables: {:?}, pattern: {}, replacement: {}}}",
                self.pattern_name, self.tag_name, self.var_defs, self.pattern,
                self.replacements.pretty_print_replacements());
    }
}

impl SpeechPattern  {
    fn build(dict: &Yaml, file: &Path, rules: &mut SpeechRules) -> Result<()> {
        // Rule::SpeechPattern
        //   build { "pattern_name", "tag_name", "pattern", "replacement" }
        // or recurse via include: file_name

        // println!("\nbuild_speech_pattern: dict:\n{}", yaml_to_string(dict, 0));

        if let Ok(include_file_name) = find_str(dict, "include") {
            let do_include_fn = |new_file: &Path| {
                rules.read_patterns(&[Some(new_file.to_path_buf()), None, None]);
            };

            return process_include(file, include_file_name, do_include_fn);
        }

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

        if pattern_name.is_err() {
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
            let speech_pattern = 
                Box::new( SpeechPattern{
                    pattern_name: pattern_name.clone(),
                    tag_name: tag_name.clone(),
                    file_name: file.to_str().unwrap().to_string(),
                    pattern: MyXPath::build(&dict["match"])
                        .chain_err(|| {
                            format!("value for 'match' in rule ({}: {}):\n{}",
                                    tag_name, pattern_name, yaml_to_string(dict, 1))
                        })?,
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
            let rule_value = rules.rules.entry(tag_name).or_insert( Vec::new());

            // if the name exists, replace it. Otherwise add the new rule
            match rule_value.iter().enumerate().find(|&pattern| pattern.1.pattern_name == speech_pattern.pattern_name) {
                None => rule_value.push(speech_pattern),
                Some((i, _old_pattern)) => {
                    let old_rule = &rule_value[i];
                    eprintln!("Warning: replacing {}/'{}' in {} with rule from {}",
                            old_rule.tag_name, old_rule.pattern_name, old_rule.file_name, speech_pattern.file_name);
                    rule_value[i] = speech_pattern;
                },
            }
        }

        return Ok( () );
    }

    fn is_match(&self, context: &Context, mathml: &Element) -> Result<bool> {
        if self.tag_name != mathml.name().local_part() && self.tag_name != "unknown" {
            return Ok( false );
        }

        // println!("\nis_match: pattern='{}'", self.pattern_name);
        // println!("    pattern_expr {:?}", self.pattern_expr);
        // print!("is_match: mathml is\n{}", crate::pretty_print::mml_to_string(mathml));
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
            writeln!(f, "{}", test)?;
        }
        return Ok( () );
    }
}

impl<'r> TestArray {
    fn build(test: &Yaml) -> Result<TestArray> {
        // 'test:' for convenience takes either a dictionary with keys if/else_if/then/then_test/else/else_test or
        //      or an array of those values (there should be at most one else/else_test)

        // if 'test' is a dictionary ('Hash'), we convert it to an array with one entry and proceed
        let tests = if test.as_hash().is_some() {
            vec![test.to_owned()]
        } else if let Some(vec) = test.as_vec() {
            vec.to_owned()
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
                let then_part = TestOrReplacements::build(&test, "then", "then_test", true)?; 
                let else_part = TestOrReplacements::build(&test, "else", "else_test", false)?;
                let n_keys = if else_part.is_none() {2} else {3};
                if test.as_hash().unwrap().len() > n_keys {
                    bail!("A key other than 'if', 'else_if', 'then', 'then_test', 'else', or 'else_test' was found in the 'then' clause of 'test'");
                };
                test_array.push(
                    Test { condition, then_part, else_part }
                );
            } else {
                // second case: should be else/else_test
                let else_part = TestOrReplacements::build(&test, "else", "else_test", true)?;
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

    fn replace<'c, 's:'c>(&self, rules_with_context: &'r mut SpeechRulesWithContext<'c, 's>, mathml: &'r Element<'c>) -> Result<String> {
        for test in &self.tests {
            if test.is_true(&rules_with_context.context_stack.base, mathml)? {
                assert!(test.then_part.is_some());
                return test.then_part.as_ref().unwrap().replace(rules_with_context, mathml);
            } else if let Some(else_part) = test.else_part.as_ref() {
                return else_part.replace(rules_with_context, mathml);
            }
        }
        return Ok( "".to_string() );
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
            TestOrReplacements::Test(t) => write!(f, "{}", t),
            TestOrReplacements::Replacements(r) => write!(f, "{}", r),
        };
    }
}

impl<'r> TestOrReplacements {
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
                    replace_key, test_key, yaml_to_string(test, 2)));    
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

    fn replace<'c, 's:'c>(&self, rules_with_context: &'r mut SpeechRulesWithContext<'c, 's>, mathml: &'r Element<'c>) -> Result<String> {
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
        write!(f, "test: {{ ")?;
        if let Some(if_part) = &self.condition {
            write!(f, "  if: '{}'", if_part)?;
        }
        if let Some(then_part) = &self.then_part {
            write!(f, "  then{}", then_part)?;
        }
        if let Some(else_part) = &self.else_part {
            write!(f, "  else{}", else_part)?;
        }
        return write!(f, "}}");
    }
}

impl Test {
    fn is_true(&self, context: &Context, mathml: &Element) -> Result<bool> {
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
    name: String,       // name of variable
    value: MyXPath,   // xpath value, typically a constant like "true" or "0", but could be "*/*[1]" to store some nodes   
}

impl fmt::Display for VariableDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{{name: {}, value: {}}}", self.name, self.value);
    }   
}

// Used for speech rules with "variables: ..."
#[derive(Debug)]
struct VariableValue<'v> {
    name: String,       // name of variable
    value: Option<Value<'v>>,   // xpath value, typically a constant like "true" or "0", but could be "*/*[1]" to store some nodes   
}

impl<'v> fmt::Display for VariableValue<'v> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match &self.value {
            None => "unset".to_string(),
            Some(val) => format!("{:?}", val)
        };
        return write!(f, "{{name: {}, value: {}}}", self.name, value);
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
                let name = as_str_checked( &name)
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
            write!(f, "  {}={}", &def.name, &def.value)?;
        }
        return Ok( () );
    }
}

#[derive(Debug)]
struct VariableValues<'v> {
    defs: Vec<VariableValue<'v>>
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

impl<'c> fmt::Display for ContextStack<'c> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return writeln!(f, " {} old_values", self.old_values.len());
    }
}

impl<'c, 'r> ContextStack<'c> {
    fn new<'a>(pref_manager: &'a PreferenceManager) -> ContextStack<'c> {
        let prefs = pref_manager.merge_prefs();
        return ContextStack {
            base: ContextStack::base_context(prefs),
            old_values: Vec::with_capacity(31)      // should avoid allocations
        }
    }

    fn base_context(var_defs: PreferenceHashMap) -> Context<'c> {
        let mut context  = Context::new();
        context.set_namespace("m", "http://www.w3.org/1998/Math/MathML");
        crate::xpath_functions::add_builtin_functions(&mut context);
        for (key, value) in var_defs {
            context.set_variable(key.as_str(), yaml_to_value(&value));
            // if let Some(str_value) = value.as_str() {
            //     if str_value != "Auto" {
            //         println!("Set {}='{}'", key.as_str(), str_value);
            //     }
            // }
        };
        return context;
    }

    fn push<'a:'c>(&'r mut self, new_vars: VariableDefinitions, mathml: &'r Element<'c>) -> Result<()> {
        // store the old value and set the new one 
        let mut old_values = VariableValues {defs: Vec::with_capacity(new_vars.defs.len()) };
        let evaluation = Evaluation::new(&self.base, Node::Element(*mathml));
        for def in &new_vars.defs {
            // get the old value (might not be defined)
            let qname = QName::new(def.name.as_str());
            let old_value = match evaluation.value_of(qname) {
                Some(val) => Some( val.clone() ),   // &Value -> Value
                None => None,
            };
            old_values.defs.push( VariableValue{ name: def.name.clone(), value: old_value} );
        }

        // use a second loop because of borrow problem with self.base and 'evaluation'
        for def in &new_vars.defs {
            // set the new value
            let new_value = match def.value.evaluate(&self.base, mathml) {
                Ok(val) => val,
                Err(_) => bail!(format!("Can't evaluate variable def for {}", def)),
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
    fn build(unicode_def: &Yaml, file_name: &Path, rules: &mut SpeechRules) -> Result<()> {
        if let Ok(include_file_name) = find_str(unicode_def, "include") {
            let do_include_fn = |new_file: &Path| {
                rules.read_unicode(&[Some(new_file.to_path_buf()), None, None]);
            };

            return process_include(file_name, include_file_name, do_include_fn);
        }

        // key: char, value is replacement or array of replacements
        let dictionary = unicode_def.as_hash();
        if dictionary.is_none() {
            bail!("Expected a unicode definition (e.g, '+':{{t: plus}}'), found {}", yaml_to_string(unicode_def, 0));
        }

        let dictionary = dictionary.unwrap();
        if dictionary.len() != 1 {
            bail!("Expected a unicode definition (e.g, '+':{{t: plus}}'), found {}", yaml_to_string(unicode_def, 0));
        }

        let (ch, replacements) = dictionary.iter().next().ok_or_else(||  format!("Expected a unicode definition (e.g, '+':{{t: plus}}'), found {}", yaml_to_string(unicode_def, 0)))?;
        if let Some(str) = ch.as_str() {
            if str.len() > 1 && str.contains('-') {
                return process_range(str, replacements, rules);
            }
        }

        let ch = UnicodeDef::get_unicode_char(ch)?;
        rules.unicode.insert(ch, ReplacementArray::build(replacements)
                                        .chain_err(|| format!("In definition of char: '{}' (0x{})",
                                                                        char::from_u32(ch).unwrap(), ch))?.replacements);
        return Ok( () );

        fn process_range(def_range: &str, replacements: &Yaml, rules: &mut SpeechRules) -> Result<()> {
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
                rules.unicode.insert(ch, ReplacementArray::build(&substitute_ch(replacements, &ch_as_str))
                                        .chain_err(|| format!("In definition of char: '{}'", def_range))?.replacements);
            };

            return Ok( () );            
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
                Yaml::String(s) => Yaml::String( s.replace(".", ch) ),
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

/// `SpeechRulesWithContext` encapsulates a named group of speech rules (e.g, "ClearSpeak")
/// along with the preferences to be used for speech.
pub struct SpeechRules {
    name: String,
    pub pref_manager: Box<PreferenceManager>,
    rules: RuleTable,                       // the speech rules used (partitioned into MathML tags in hashmap, then linearly searched)
    translate_single_chars_only: bool,      // strings like "half" don't want 'a's translated, but braille does
    unicode: UnicodeTable,                  // the speech rules used for Unicode characters
}

impl fmt::Display for SpeechRules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "SpeechRules '{}'\n{})", self.name, self.pref_manager)?;
        let mut rules_vec: Vec<(&String, &Vec<Box<SpeechPattern>>)> = self.rules.iter().collect();
        rules_vec.sort_by(|(tag_name1, _), (tag_name2, _)| tag_name1.cmp(tag_name2));
        for (tag_name, rules) in rules_vec {
            writeln!(f, "   {}: #patterns {}", tag_name, rules.len())?;
        };
        return writeln!(f, "   {} unicode entries", &self.unicode.len());
    }
}


/// `SpeechRulesWithContext` encapsulates a named group of speech rules (e.g, "ClearSpeak")
/// along with the preferences to be used for speech.
/// Because speech rules can define variables, there is also a context that is carried with them
pub struct SpeechRulesWithContext<'c, 's:'c> {
    speech_rules: &'s SpeechRules,
    context_stack: ContextStack<'c>,   // current value of (context) variables
}

impl<'c, 's:'c> fmt::Display for SpeechRulesWithContext<'c, 's> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "SpeechRulesWithContext \n{})", self.speech_rules)?;
        return writeln!(f, "   {} context  entries", &self.context_stack);
    }
}

thread_local!{
    /// The current set of speech rules
    // maybe this should be a small cache of rules in case people switch rules/prefs?
    pub static SPEECH_RULES: RefCell<SpeechRules> =
            RefCell::new( SpeechRules::new("speech", true) );

    pub static BRAILLE_RULES: RefCell<SpeechRules> =
            RefCell::new( SpeechRules::new("braille", false) );
}

impl SpeechRules {
    fn new(name: &str, translate_single_chars_only: bool) -> SpeechRules {
        let pref_manager = PreferenceManager::new();
        if let Err(e) = crate::definitions::read_definitions_file(pref_manager.get_definitions_file()) {
            print_errors(&e);
            exit(1);
        };

        let rules = SpeechRules {
            name: String::from(name),
            rules: HashMap::with_capacity(31),           // lazy load them
            unicode: HashMap::with_capacity(6997),       // lazy load them
            translate_single_chars_only,
            pref_manager,
        };
        return rules;
    }

    pub fn invalidate(&mut self, changes: FilesChanged) {
        if changes.style {
            self.rules.clear();
        };

        if changes.unicode {
            self.unicode.clear();
        };

        // FIX: figure out flow of 'DEFINITIONS' setting/clearing
        // if changes.defs {
        //     self.defs.clear();
        // };
    }

    fn update(&mut self, speech_files: bool) {
        if self.rules.is_empty() || !self.pref_manager.is_up_to_date() {
            let rule_file = 
                if speech_files
                    {self.pref_manager.get_style_file()}
                else {self.pref_manager.get_braille_file()}
                .clone();
            self.read_patterns(&rule_file);
        }

        if self.unicode.is_empty() {
            let unicode_file = if speech_files
                     {self.pref_manager.get_speech_unicode_file()}
                else {self.pref_manager.get_braille_unicode_file()}
                .clone();
            self.read_unicode(&unicode_file);
        }
    }

    fn read_patterns(&mut self, path: &Locations) {
        // FIX: should read first (lang), then supplement with second (region)
        use std::fs;
        if let Some(p) = &path[0] {
            let rule_file_contents = fs::read_to_string(p).expect("cannot read file");
            let rules_build_fn = |pattern: &Yaml| {
                if let Err(e) = self.build_speech_patterns(pattern, p) {
                    print_errors(&e.chain_err(||format!("in file {:?}", p.to_str().unwrap())));
                }    
            };
            if let Err(e) = compile_rule(&rule_file_contents, rules_build_fn) {
                print_errors(&e.chain_err(||format!("in file {:?}", p.to_str().unwrap())));
            }
        }
    }

    fn build_speech_patterns(&mut self, patterns: &Yaml, file_name: &Path) -> Result<()> {
        // Rule::SpeechPatternList
        let patterns_vec = patterns.as_vec();
        if patterns_vec.is_none() {
            bail!(yaml_type_err(patterns, "array"));
        }
        let patterns_vec = patterns.as_vec().unwrap();

        for (i, entry) in patterns_vec.iter().enumerate() {
            let built_patterns = SpeechPattern::build(entry, file_name, self);
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
        }
        return Ok( () );  
    }
    
    fn read_unicode(&mut self, path: &Locations) {
        // FIX: should read first (lang), then supplement with second (region)
        if let Some(p) = &path[0] {
            use std::fs;    
            let unicode_file_contents = fs::read_to_string(p).expect("cannot read file");
            let unicode_build_fn = |unicode_def_list: &Yaml| {
                let unicode_defs = unicode_def_list.as_vec();
                if unicode_defs == None {
                   format!("File '{}' does not being with an array", yaml_to_type(unicode_def_list));
                };
                for unicode_def in unicode_defs.unwrap() {
                    if let Err(e) = UnicodeDef::build(unicode_def, p, self) {
                        print_errors(&e.chain_err(|| {format!("In file {:?}", p)}));
                    }        
                };
            };
            if let Err(e) =compile_rule(&unicode_file_contents, unicode_build_fn) {
                print_errors(&e.chain_err(||format!("in file {:?}", p.to_str().unwrap())));
            }
            println!("Hashmap has {} entries", self.unicode.len());
        }
    }

}

use crate::prefs::FilesChanged;
/// We track three different lifetimes:
///   'c -- the lifetime of the context and mathml
///   's -- the lifetime of the speech rules (which is static)
///   'r -- the lifetime of the reference (this seems to be key to keep the rust memory checker happy)
impl<'c, 's:'c, 'r> SpeechRulesWithContext<'c, 's> {
    fn new(speech_rules: &'s SpeechRules) -> SpeechRulesWithContext {
        return SpeechRulesWithContext {
            speech_rules,
            context_stack: ContextStack::new(&speech_rules.pref_manager)
        }
    }

    fn match_pattern(&'r mut self, mathml: &'r Element<'c>) -> Result<String> {
        // println!("Looking for a match for: \n{}", crate::pretty_print::mml_to_string(mathml));
        let mut tag_name = mathml.name().local_part();
        let rules = &self.speech_rules.rules;
        if !rules.contains_key(tag_name) {
            tag_name = "unknown";       // should be rules for 'unknown'
        }
        let rule_value = rules.get(tag_name);

        if let Some(rule_vector) = rule_value {
            for pattern in rule_vector {
                // println!("Pattern: {}", pattern);
                if pattern.is_match(&self.context_stack.base, mathml)
                        .chain_err(|| error_string(pattern, *mathml) )? {
                    if pattern.var_defs.len() > 0 {
                        self.context_stack.push(pattern.var_defs.clone(), mathml)?;
                    }
                    let result = pattern.replacements.replace(self, mathml);
                    if pattern.var_defs.len() > 0 {
                        self.context_stack.pop();
                    }
                    return result.chain_err(||
                        format!(
                            "attempting replacement pattern: \"{}\" for \"{}\".\n\
                            Replacement \"{}\" due to matching the following MathML with the pattern \"{}\".\n\
                            {}\
                            The patterns are in {}.\n",
                            pattern.pattern_name, pattern.tag_name,
                            pattern.replacements.pretty_print_replacements(),pattern.pattern,
                            pretty_print::mml_to_string(&mathml),
                            pattern.file_name
                        )
                    );
                }
            }
        }

        // unknown element -- should have rules to handle this -- let flow through to default error
        let mut file_name = "unknown";
        if let Some(path) = &self.speech_rules.pref_manager.get_style_file()[0] {
            file_name= path.to_str().unwrap();
        }
        if rules.get("math").is_none() {
            bail!("No rules found for any matches!!! See the error log.");
        } else {
            // FIX: handle error appropriately
            bail!("\nNo match found!\nMissing patterns in {} or bad MathML.\n{}",
                    file_name, crate::pretty_print::mml_to_string(&mathml)); 
        }

        fn error_string(pattern: &SpeechPattern, mathml: Element) -> String {
            return format!(
                "error during pattern match using: \"{}\" for \"{}\".\n\
                Pattern is \n{}\nMathML for the match:\n\
                {}\
                The patterns are in {}.\n",
                pattern.pattern_name, pattern.tag_name,
                pattern.pattern,
                pretty_print::mml_to_string(&mathml),
                pattern.file_name
            );
        }
    }
    
    fn replace(&'r mut self, replacement: &Replacement, mathml: &'r Element<'c>) -> Result<String> {
        return Ok(
            match &*replacement {
                Replacement::Text(t) => t.clone(),
                Replacement::XPath(path) => path.replace(self, mathml)?,
                Replacement::TTS(tts) => {
                    self.speech_rules.pref_manager.get_tts().replace(&tts, &self.speech_rules.pref_manager, self, mathml)?
                },
                Replacement::Test(test) => {
                    test.replace(self, mathml)?                     
                },
                Replacement::With(with) => {
                    with.replace(self, mathml)?                     
                },
                Replacement::Insert(ic) => {
                    ic.replace(self, mathml)?                     
                },
            }
        )
    }

    fn replace_nodes(&'r mut self, nodes: nodeset::Nodeset<'c>, mathml: &'r Element<'c>) -> Result<String> {
        //println!("replace_nodes: working on {} nodes", nodes.size());
        let mut result = String::with_capacity(3*nodes.size());   // guess (2 chars/node + space)
        let mut not_first_time = false;
        for node in nodes.document_order() {
            if not_first_time {
                result.push(' ');
                not_first_time = true;
            }
            let matched = match node {
                Node::Element(n) => self.match_pattern(&n)?,
                Node::Text(t) =>  self.replace_chars(&t.text(), mathml)?,
                _ => {eprintln!("replace_nodes: found unexpected node type!!! (ignored)"); "".to_string() }
            };
            result += &matched;
        }
        // let result = nodes.document_order()
        //     .iter()
        //     .map(|node: &'r Node<'c>|
        //         match node {
        //             Node::Element(n) => self.match_pattern(n),
        //             Node::Text(t) =>  self.replace_chars(&t.text(), mathml),
        //             _ => {eprintln!("replace_nodes: found unexpected node type!!! (ignored)"); Ok( "".to_string() )}
        //         })
        //     .collect::<Result<Vec<String>>>()?
        //     .join(" ");
        return Ok( result );
    }

    fn replace_chars(&'r mut self, str: &str, mathml: &'r Element<'c>) -> Result<String> {
        // Lookup unicode "pronunciation" of char
        // Note: TTS is not supported here (not needed and a little less efficient)
        let rules = self.speech_rules;
        let mut chars = str.chars();
        if rules.translate_single_chars_only {
            let ch = chars.next().unwrap_or(' ');
            if chars.next().is_none() {
                // single char
                return replace_single_char(self, ch, mathml)
            } else {
                // more than one char (don't use str.len() since that is bytes, not chars)
                return Ok(String::from(str));
            }
        };

        let result = chars
            .map(|ch| replace_single_char(self, ch, mathml))
            .collect::<Result<Vec<String>>>()?
            .join("");
        return Ok( result );

        fn replace_single_char<'c, 's:'c, 'r>(rules_with_context: &'r mut SpeechRulesWithContext<'c, 's>, ch: char, mathml: &'r Element<'c>) -> Result<String> {
            let ch_as_u32 = ch as u32;
            let replacements = rules_with_context.speech_rules.unicode.get( &ch_as_u32 );
            if replacements.is_none() {
                return Ok(String::from(ch));   // no replacement, so just return the char and hope for the best
            };

            // map across all the parts of the replacement, collect them up into a Vec, and then concat them together
            return Ok(
                replacements.unwrap()
                            .iter()
                            .map(|replacement|
                                rules_with_context.replace(replacement, mathml)
                                        .chain_err(|| format!("Unicode replacement error: {}", replacement)) )
                            .collect::<Result<Vec<String>>>()?
                            .join(" ")
            );
        }
    }
}

// Hack to allow replacement of `str` with braille chars.
pub fn braille_replace_chars(str: &str, mathml: Element) -> Result<String> {
    return BRAILLE_RULES.with(|rules| {
        let rules = rules.borrow();
        let mut rules_with_context = SpeechRulesWithContext::new(&rules);
        return rules_with_context.replace_chars(str, &mathml);
    })
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
        let mut rules = SpeechRules::new("testing", true);

        SpeechPattern::build(&doc[0], Path::new("testing"), &mut rules).unwrap();
        assert_eq!(rules.rules["math"].len(), 1, "\nshould only be one rule");

        let speech_pattern = &rules.rules["math"][0];
        assert_eq!(speech_pattern.pattern_name, "default", "\npattern name failure");
        assert_eq!(speech_pattern.tag_name, "math", "\ntag name failure");
        assert_eq!(speech_pattern.pattern.string, ".", "\npattern failure");
        assert_eq!(speech_pattern.replacements.replacements.len(), 1, "\nreplacement failure");
        assert_eq!(speech_pattern.replacements.replacements[0].to_string(), r#"{x: "./*"}"#, "\nreplacement failure");
    }

    #[test]
    fn test_read_statements_with_replace() {
        let str = r#"---
        {name: default, tag: math, match: ".", replace: [x: "./*"] }"#;
        let doc = YamlLoader::load_from_str(str).unwrap();
        assert_eq!(doc.len(), 1);
        let mut rules = SpeechRules::new("testing", true);
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
        assert_eq!(speech_pattern.pattern.string, ".", "\npattern failure");
        assert_eq!(speech_pattern.replacements.replacements.len(), 2, "\nreplacement failure");
    }

    #[test]
    fn test_read_statements_with_add() {
        let str = r#"---
        {name: default, tag: math, match: ".", replace: [x: "./*"] }"#;
        let doc = YamlLoader::load_from_str(str).unwrap();
        assert_eq!(doc.len(), 1);
        let mut rules = SpeechRules::new("testing", true);
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
        assert_eq!(speech_pattern.pattern.string, ".", "\npattern failure");
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
        assert_eq!(result.unwrap(), r#"DEBUG(*[2]/*[3][text()='3'], "*[2]/*[3][text()='3']" )"#);
    }

    #[test]
    fn test_debug_quoted_paren() {
        let str = r#"DEBUG(*[2]/*[3][text()='('])"#;
        let result = MyXPath::add_debug_string_arg(str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"DEBUG(*[2]/*[3][text()='('], "*[2]/*[3][text()='(']" )"#);
    }

    #[test]
    fn test_debug_quoted_paren_before_paren() {
        let str = r#"DEBUG(ClearSpeak_Matrix = 'Combinatorics') and IsBracketed(., '(', ')')"#;
        let result = MyXPath::add_debug_string_arg(str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"DEBUG(ClearSpeak_Matrix = 'Combinatorics', "ClearSpeak_Matrix = 'Combinatorics'" ) and IsBracketed(., '(', ')')"#);
    }

    // #[test]
    // fn test_nested_debug_quoted_paren() {
    //     let str = r#"DEBUG(*[2]/*[3][DEBUG(text()='(')])"#;
    //     let result = MyXPath::add_debug_string_arg(str);
    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), r#"DEBUG(*[2]/*[3][DEBUG(text()='(')], "DEBUG(*[2]/*[3][DEBUG(text()='(')], \"text()='(')]\")"#);
    // }

}