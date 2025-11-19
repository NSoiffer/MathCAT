//! # Definitions module
//! This module is responsible for reading in the definitions files and converting them to either vectors or hashmaps so that
//! the definitions can be used by the program.
//!
//! ## Leaked Implementation Details
//! There is no escaping some implementation details.
//! Because these definitions are stored in global variables, the variables need to be protected
//!   in some way so they can be written at runtime when the files are read.
//!   This is done by putting them in side of a lock (`thread_local`).
//!
//! Furthermore, it was necessary to use use `RefCell` and `Rc` to deal with interior mutability.
//! All of this means that a lock needs to be obtained _and_ the contents borrowed to access a definition.
//!
//! To minimize the global variable footprint, all of the definitions are put inside of a single global variable [`DEFINITIONS`].
//!
//! //! Note: some of the variable are `vec`s and some are `hashset`s.
//! Numbers are typically vectors so that indexing a digit is easy.
//! Others such a `functions_names` are a hashset because you just want to know if an `mi` is a known name or not.
//! The functions `as_vec` and `as_hashset` should be used on the appropriate variable.
//! ## Names
//! The names of "variables" in the definition files use camel case (e.g., "FunctionNames"). In the code, to fit with rust
//! naming conventions, snake case is used (e.g, "function_names"). 
//!
//! See the struct [`Definitions`] for the variables that are read in.
#![allow(clippy::needless_return)]

extern crate yaml_rust;
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;
use crate::errors::*;
use crate::prefs::*;
use std::{cell::RefCell, cell::Ref, cell::RefMut, rc::Rc};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use crate::shim_filesystem::read_to_string_shim;

/// An enum to paper over the different types of data access needed.
///
/// Having a Rc<RefCell<FromFileVariable>> seems a bit complicated in terms of types but...
/// 1. The rust book seems to endorse the Rc<RefCell<...>>> approach when there are multiple owners of mutable date.
///    See <https://doc.rust-lang.org/book/ch15-05-interior-mutability.html> towards the end
/// 2. When a file is read, we need to clear and add data to the structure being read (reassigning could work for clearing).
///    When we use the data, we either want to index into it or test if an item is there.
///    The structures we use are either a Vec or a HashMap, so we need to abstract that away in `FromFileVariable`.
///    Unfortunately, traits don't quite work as an option here:
///    *  Vec implements extends (`add`), but there is no test/contains
///    *  Hashmap implements `index`, but panics if the item isn't there
///
/// Because of the above limitations, we introduce the enum [`Contains`] which dispatches appropriately to Vec/Hashmap
#[derive(Debug, Clone)]
pub enum Contains {
    Vec(Rc<RefCell<Vec<String>>>),
    Set(Rc<RefCell<HashSet<String>>>),
    Map(Rc<RefCell<HashMap<String, String>>>),
}

impl Contains {
    // fn add(&mut self, item: String) {
    //     match self {
    //         Contains::Vec(v) => { v.borrow_mut().push(item); },
    //         Contains::Set(s) => { s.borrow_mut().insert(item); }
    //     }
    // }

    // fn clear(&mut self) {
    //     match self {
    //         Contains::Vec(v) => { v.borrow_mut().clear(); },
    //         Contains::Set(s) => { s.borrow_mut().clear(); }
    //     }
    // }
}
pub type CollectionFromFile = Contains;
type VariableDefHashMap = HashMap<String, CollectionFromFile>;

/// Global structure containing all of the definitions.
/// Each field in the structure corresponds to a named value read in from the `definitions.yaml` files.
///
/// The names of "variables" in the definition files use camel case (e.g., "FunctionNames"). In the code, to fit with rust
/// naming conventions, snake case is used (e.g, "function_names").
///
/// There should only be one instance of this structure ([`DEFINITIONS`])
// FIX: this probably can done with a macro to remove all the repetition
pub struct Definitions {
    pub name_to_var_mapping: VariableDefHashMap,
}

impl Default for Definitions {
    fn default() -> Self {
        Definitions {
            name_to_var_mapping: HashMap::with_capacity(30),
        }
    }
}

impl Definitions {
    fn new() -> Self {
        Definitions {
            name_to_var_mapping: HashMap::with_capacity(30),
        }
    }

    pub fn get_hashset(&self, name: &str) -> Option<Ref<'_, HashSet<String>>> {
        let names = self.name_to_var_mapping.get(name);
        if let Some(Contains::Set(set)) = names {
            return Some(set.borrow());
        }
        return None;
    }

    pub fn get_hashmap(&self, name: &str) ->  Option<Ref<'_, HashMap<String, String>>> {
        let names = self.name_to_var_mapping.get(name);
        if let Some(Contains::Map(map)) = names {
            return Some(map.borrow());
        }
        return None;
    }

    pub fn get_vec(&self, name: &str) -> Option<Ref<'_, Vec<String>>> {
        let names = self.name_to_var_mapping.get(name);
        if let Some(Contains::Vec(vec)) = names {
            return Some(vec.borrow());
        }
        return None;
    }
}

thread_local!{
    /// Global variable containing all of the definitions.
    /// See [`Definitions`] for more details.
    pub static SPEECH_DEFINITIONS: RefCell<Definitions> = RefCell::new( Definitions::new() );
    pub static BRAILLE_DEFINITIONS: RefCell<Definitions> = RefCell::new( Definitions::new() );
    pub static DEFINITIONS: &'static std::thread::LocalKey<RefCell<Definitions>> = const { &SPEECH_DEFINITIONS };
}

/// Reads the `definitions.yaml` files specified by current_files -- these are presumed to need updating. 
///
/// If there is a failure during read, the error is propagated to the caller
pub fn read_definitions_file(use_speech_defs: bool) -> Result<Vec<PathBuf>> {
    // for each file in `locations`, read the contents and process them
    let pref_manager = PreferenceManager::get();
    let pref_manager = pref_manager.borrow();
    let file_path = pref_manager.get_definitions_file(use_speech_defs);
    let definitions = if use_speech_defs {&SPEECH_DEFINITIONS} else {&BRAILLE_DEFINITIONS};
    definitions.with( |defs| defs.borrow_mut().name_to_var_mapping.clear() );
    let mut new_files = vec![file_path.to_path_buf()];
    let mut files_read = read_one_definitions_file(use_speech_defs, file_path).chain_err(|| format!("in file '{}", file_path.to_string_lossy()))?;
    new_files.append(&mut files_read);

    // merge the contents of `TrigFunctions` into a set that contains all the function names (from `AdditionalFunctionNames`).
    return definitions.with(|defs| {
        let mut defs = defs.borrow_mut();
        make_all_set_references_valid(&mut defs);
        return Ok(new_files);
    });
    

    /// Make references to all used set be valid by creating empty sets if they weren't defined
    fn make_all_set_references_valid(defs: &mut RefMut<Definitions>) {
        // FIX: this list is created by hand -- it would be better if there was a way to create the list Automatically
        // Note: "FunctionNames" is created in build_all_functions_set() if not already set
        let used_set_names = ["GeometryPrefixOperators", "LikelyFunctionNames", "TrigFunctionNames", "AdditionalFunctionNames", "Arrows", "GeometryShapes"];
        // let name_to_mapping = defs.name_to_var_mapping.borrow_mut();
        for set_name in used_set_names {
            if defs.get_hashset(set_name).is_none() {
                defs.name_to_var_mapping.insert(set_name.to_string(), Contains::Set( Rc::new( RefCell::new( HashSet::with_capacity(0) ) ) ));
            }
        }
        if defs.get_hashset("FunctionNames").is_none() {
            let all_functions = build_all_functions_set(defs);
            defs.name_to_var_mapping.insert("FunctionNames".to_string(), Contains::Set( Rc::new( RefCell::new( all_functions ) ) ));
        }
    }

    /// merge "TrigFunctions" and "AdditionalFunctionNames" into a new set named "FunctionNames"
    fn build_all_functions_set(defs: &mut RefMut<Definitions>) -> HashSet<String> {
        let trig_functions = defs.get_hashset("TrigFunctionNames").unwrap();
        let mut all_functions = defs.get_hashset("AdditionalFunctionNames").unwrap().clone();
        for trig_name in trig_functions.iter() {
            all_functions.insert(trig_name.clone());
        }
        return all_functions;
    }
}

use crate::speech::*;
fn read_one_definitions_file(use_speech_defs: bool, path: &Path) -> Result<Vec<PathBuf>> {
    // read in the file contents   
    let definition_file_contents = read_to_string_shim(path)
            .chain_err(|| format!("trying to read {}", path.to_str().unwrap()))?;

    // callback to do the work of building up the defined vectors/hashmaps (in 'build_values') from YAML
    let defs_build_fn = |variable_def_list: &Yaml| {
        // Rule::DefinitionList
        // debug!("variable_def_list {} is\n{}", yaml_to_type(variable_def_list), yaml_to_string(variable_def_list));
        let mut files_read = vec![path.to_path_buf()];
        let vec = crate::speech::as_vec_checked(variable_def_list)
                    .chain_err(||format!("in file {:?}", path.to_str()))?;
        for variable_def in vec {
            if let Some(mut added_files) = build_values(variable_def, use_speech_defs, path).chain_err(||format!("in file {:?}", path.to_str()))? {
                files_read.append(&mut added_files);
            }
        }
        return Ok(files_read);
    };

    // Convert the file contents to YAML and call the callback
    return crate::speech::compile_rule(&definition_file_contents, defs_build_fn)
        .chain_err(|| format!("In file '{}'", path.to_str().unwrap()));
}

/// Do the work of converting a single YAML def into the vec/hashset/hashmap
/// name: [a, b, c] -- assume an indexed vector
/// name: {a, b, c} -- assume a hash set
/// name: {a: A, b: B, c: C} -- assume a hashmap
/// Returns all the files that were read
fn build_values(definition: &Yaml, use_speech_defs: bool, path: &Path) -> Result<Option<Vec<PathBuf>>> {
    // Rule::Definition
    let dictionary = crate::speech::as_hash_checked(definition)?;
    if dictionary.len()!=1 {
        bail!("Should only be one definition rule: {}", yaml_to_type(definition));
    }
    let (key, value) = dictionary.iter().next().unwrap();
    let def_name = key.as_str().ok_or_else(|| format!("definition list name '{}' is not a string", yaml_to_type(key)))?;
    if def_name == "include" {
        let do_include_fn = |new_file: &Path| {
            read_one_definitions_file(use_speech_defs, new_file)
        };
        let include_file_name = value.as_str().ok_or_else(|| format!("definition list include name '{}' is not a string", yaml_to_type(value)))?;
        return Ok( Some(crate::speech::process_include(path, include_file_name, do_include_fn)?) );
    }

    let result;
    if def_name.starts_with("Numbers") || def_name.ends_with("_vec") {
         result = Contains::Vec( Rc::new( RefCell::new( get_vec_values(value.as_vec().unwrap())? ) ) );
    } else {
        // match value.as_vec() {
        //     Some(vec) => {
        //         result = Contains::Set( Rc::new( RefCell::new( get_set_values(vec)? ) ) );            },
        //     None => {
        //         let dict = value.as_hash().ok_or_else(|| format!("definition list value '{}' is not an array or dictionary", yaml_to_type(value)))?;
        //         result = Contains::Map( Rc::new( RefCell::new( get_map_values(dict)
        //                     .chain_err(||format!("while reading value '{}'", def_name))? ) ) );

        //     },
        // }
        let dict = value.as_hash().ok_or_else(|| format!("definition list value '{}' is not an array or dictionary", yaml_to_type(value)))?;
        if dict.is_empty() {
            result = Contains::Set( Rc::new( RefCell::new( HashSet::with_capacity(0) ) ) );
        } else {
            // peak and see if this is a set or a map
            let (_, entry_value) = dict.iter().next().unwrap();
            if entry_value.is_null() {
                result = Contains::Set( Rc::new( RefCell::new( get_set_values(dict)
                            .chain_err(||format!("while reading value '{def_name}'"))? ) ) );
            } else {
                // peak and see if this is a set or a map
                let (_, entry_value) = dict.iter().next().unwrap();
                if entry_value.is_null() {
                    result = Contains::Set( Rc::new( RefCell::new( get_set_values(dict)
                                .chain_err(||format!("while reading value '{def_name}'"))? ) ) );
                } else {
                    result = Contains::Map( Rc::new( RefCell::new( get_map_values(dict)
                                .chain_err(||format!("while reading value '{def_name}'"))? ) ) );
                }
            }
        }
    };

    let definitions = if use_speech_defs {&SPEECH_DEFINITIONS} else {&BRAILLE_DEFINITIONS};
    return definitions.with(|definitions| {
        let name_definition_map = &mut definitions.borrow_mut().name_to_var_mapping;
        name_definition_map.insert(def_name.to_string(), result);
        return Ok(None);
    });

    fn get_vec_values(values: &Vec<Yaml>) -> Result<Vec<String>> {
        let mut result = Vec::with_capacity(values.len());
        for yaml_value in values {
            let value = yaml_value.as_str()
                .ok_or_else(|| format!("list entry '{}' is not a string", yaml_to_type(yaml_value)))?
                .to_string();
            result.push(value);
        }
        return Ok(result);
    }

    fn get_set_values(values: &Hash) -> Result<HashSet<String>> {
        let mut result = HashSet::with_capacity(2*values.len());
        for (key, value) in values {
            let key = key.as_str()
                .ok_or_else(|| format!("list entry '{}' is not a string", yaml_to_type(key)))?
                .to_string();
            if let Yaml::Null = value {
            } else {
                bail!("list entry '{}' is not a string", yaml_to_type(value));
            }
            result.insert(key);
        }
        return Ok(result);
    }

    fn get_map_values(values: &Hash) -> Result<HashMap<String, String>> {
        let mut result = HashMap::with_capacity(2*values.len());
        for (key, value) in values {
            let key = key.as_str()
                .ok_or_else(|| format!("list entry '{}' is not a string", yaml_to_type(key)))?
                .to_string();
            let value = value.as_str()
                .ok_or_else(|| format!("list entry '{}' is not a string", yaml_to_type(value)))?
                .to_string();
            result.insert(key, value);
        }
        return Ok(result);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        let numbers = r#"[NumbersTens: ["", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"]]"#;
        let defs_build_fn = |variable_def_list: &Yaml| {
            // Rule::DefinitionList
            //debug!("variable_def_list {} is\n{}", yaml_to_type(variable_def_list), yaml_to_string(variable_def_list, 0));
            for variable_def in variable_def_list.as_vec().unwrap() {
                if let Err(e) = build_values(variable_def, true, &Path::new("")) {
                    bail!("{}", crate::interface::errors_to_string(&e.chain_err(||format!("in file {:?}", numbers))));
                }
            }
            return Ok(vec![]);
        };
        compile_rule(&numbers, defs_build_fn).unwrap();
        SPEECH_DEFINITIONS.with(|defs| {
            let defs = defs.borrow();
            let names = defs.get_vec("NumbersTens");
            assert!(names.is_some());
            let names = names.unwrap();
            assert_eq!(names.len(), 10);
            assert_eq!(names[0], "");
            assert_eq!(names[9], "ninety");
        });
    }


    #[test]
    fn test_set() {
        let likely_function_names = r#"[LikelyFunctionNames: {"f", "g", "h", "F", "G", "H", "[A-Za-z]+"}]"#;
        let defs_build_fn = |variable_def_list: &Yaml| {
            // Rule::DefinitionList
            //debug!("variable_def_list {} is\n{}", yaml_to_type(variable_def_list), yaml_to_string(variable_def_list, 0));
            for variable_def in variable_def_list.as_vec().unwrap() {
                if let Err(e) = build_values(variable_def, true, &Path::new("")) {
                    bail!("{}", crate::interface::errors_to_string(&e.chain_err(||format!("in file {:?}", likely_function_names))));
                }
            }
            return Ok(vec![]);
        };
        compile_rule(&likely_function_names, defs_build_fn).unwrap();
        SPEECH_DEFINITIONS.with(|defs| {
            let defs = defs.borrow();
            let names = defs.get_hashset("LikelyFunctionNames");
            assert!(names.is_some());
            let names = names.unwrap();
            assert_eq!(names.len(), 7);
            assert!(names.contains("f"));
            assert!(!names.contains("a"));
        });
    }

    #[test]
    fn test_hashmap() {
        let units = r#"[Units: {"A": "amp", "g": "gram", "m": "meter", "sec": "second"}]"#;
        let defs_build_fn = |variable_def_list: &Yaml| {
            // Rule::DefinitionList
            //debug!("variable_def_list {} is\n{}", yaml_to_type(variable_def_list), yaml_to_string(variable_def_list, 0));
            for variable_def in variable_def_list.as_vec().unwrap() {
                if let Err(e) = build_values(variable_def, true, &Path::new("")) {
                    bail!("{}", crate::interface::errors_to_string(&e.chain_err(||format!("in file {:?}", units))));
                }
            }
            return Ok(vec![]);
        };
        compile_rule(&units, defs_build_fn).unwrap();
        SPEECH_DEFINITIONS.with(|defs| {
            let defs = defs.borrow();
            let names = defs.get_hashmap("Units");
            assert!(names.is_some());
            let names = names.unwrap();
            assert_eq!(names.len(), 4);
            assert_eq!(names.get("A").unwrap(), "amp");
            assert_eq!(names.get("sec").unwrap(), "second");
            assert_eq!(names.get("xxx"), None);
        });
    }
}