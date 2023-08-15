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
use yaml_rust::Yaml;
use crate::errors::*; 
use crate::prefs::*;
use std::{cell::RefCell, cell::Ref, cell::RefMut, collections::HashSet,  rc::Rc};
use std::{collections::HashMap, path::Path};
use crate::shim_filesystem::read_to_string_shim;

/// An enum to paper over the different types of data access needed.
///
/// Having a Rc<RefCell<FromFileVariable>> seems a bit complicated in terms of types but...
/// 1. The rust book seems to endorse the Rc<RefCell<...>>> approach when there are multiple owners of mutable date.
///     See <https://doc.rust-lang.org/book/ch15-05-interior-mutability.html> towards the end
/// 2. When a file is read, we need to clear and add data to the structure being read (reassigning could work for clearing).
///    When we use the data, we either want to index into it or test if an item is there.
///    The structures we use are either a Vec or a HashMap, so we need to abstract that away in `FromFileVariable`.
///    Unfortunately, traits don't quite work as an option here:
///    *  Vec implements extends (`add`), but there is no test/contains
///    *  Hashmap implements `index`, but panics if the item isn't there
///
///    Because of the above limitations, we introduce the enum [`Contains`] which dispatches appropriately to Vec/Hashmap
#[derive(Debug, Clone)]
pub enum Contains {
    Vec(Rc<RefCell<Vec<String>>>),
    Set(Rc<RefCell<HashSet<String>>>),
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

    pub fn get_hashset(&self, name: &str) -> Option<Ref<HashSet<String>>> {
        let names = self.name_to_var_mapping.get(name);
        return match names {
            None => None,
            Some(contains) => match contains {
                Contains::Vec(_) => None,
                Contains::Set(hashset) => Some(hashset.borrow()),
            }
        }
    }

    pub fn get_vec(&self, name: &str) -> Option<Ref<Vec<String>>> {
        let names = self.name_to_var_mapping.get(name);
        return match names {
            None => None,
            Some(contains) => match contains {
                Contains::Vec(v) => Some(v.borrow()),
                Contains::Set(_) => None,
            }
        }
    }
}

thread_local!{
    /// Global variable containing all of the definitions.
    /// See [`Definitions`] for more details.
    pub static DEFINITIONS: RefCell<Definitions> = RefCell::new( Definitions::new() );
}

/// Reads the `definitions.yaml` files specified by `locations`.
///
/// If there is a failure during read, the error is propagated to the caller
pub fn read_definitions_file(locations: &Locations) -> Result<()> {
    // for each file in `locations`, read the contents and process them
    // we cache the last location (saves 3-4ms on startup/switching): creating the SpeechRules calls this for each rule
    thread_local!{
        static LOCATION_CACHE: RefCell<Locations> =
                RefCell::new( Locations::default() );
    }
    
    if LOCATION_CACHE.with(|cache| are_locations_same(&cache.borrow(), locations)) {
        return Ok( () );
    } else {
        LOCATION_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            cache[0]= locations[0].clone();
            cache[1]= locations[1].clone();
            cache[2]= locations[2].clone();
        })
    }
    let result = locations.iter().try_for_each(|path|
            match path {
                None => Ok(()),
                Some(path) => read_one_definitions_file(path)
            });
    verify_definitions()?;

    // merge the contents of `TrigFunctions` into a set that contains all the function names (from `AdditionalFunctionNames`).
    DEFINITIONS.with(|defs| {
        let mut defs = defs.borrow_mut();
        let all_functions = build_all_functions_set(&defs);
        let name_to_mapping = &mut defs.name_to_var_mapping;
        name_to_mapping.insert("FunctionNames".to_string(), Contains::Set( Rc::new( RefCell::new( all_functions ) ) ));
    });
    return result;

    fn build_all_functions_set(defs: &RefMut<Definitions>) -> HashSet<String> {
        let trig_functions = defs.get_hashset("TrigFunctionNames").unwrap();
        let mut all_functions = defs.get_hashset("AdditionalFunctionNames").unwrap().clone();
        for trig_function in trig_functions.iter() {
            all_functions.insert(trig_function.clone());
        }
        return all_functions;
    }
}

fn verify_definitions() -> Result<()> {
    // all of the 'numbers-xxx' files should be either size 0 or multiples of tens except:
    //   ...-ones
    //   numbers-plural, which should have a single entry
    lazy_static! {
        static ref USED_SETS: Vec<&'static str> = vec!["TrigFunctionNames", "AdditionalFunctionNames", "LikelyFunctionNames", 
                                "LargeOperators"];
        static ref USED_VECTORS: Vec<&'static str> = vec![
                "NumbersHundreds", "NumbersTens", "NumbersOnes",
                "NumbersOrdinalPluralLarge", "NumbersOrdinalLarge", "NumbersLarge",
                "NumbersOrdinalPluralHundreds", "NumbersOrdinalPluralTens", "NumbersOrdinalPluralOnes",
                "NumbersOrdinalHundreds", "NumbersOrdinalTens", "NumbersOrdinalOnes",
                "NumbersOrdinalFractionalPluralOnes", "NumbersOrdinalFractionalOnes"
        ];
    }
    return DEFINITIONS.with(|definitions| {
        // verify that all the named functions used in the code exist
        // FIX: is there a way to gather them automatically?
        let definitions = definitions.borrow();
        let name_definition_map = &definitions.name_to_var_mapping;

        for name in USED_SETS.iter() {
            if !name_definition_map.contains_key(*name) {
                bail!("Required (set) name '{}' is missing from 'definitions.yaml'", *name);
            }
        }
        for name in USED_VECTORS.iter() {
            if !name_definition_map.contains_key(*name) {
                bail!("Required (array) name '{}' is missing from 'definitions.yaml'", *name);
            }
        }
        for (name,collection) in name_definition_map.iter() {
            if name.contains("number") && !name.contains("fraction") {
                match collection {
                    Contains::Vec(v) => {
                        let v = v.borrow();
                        if v.is_empty() || v.len() % 10 != 0 {
                            bail!("{} has wrong number of values: {}", name, v.len());
                        }
                    },
                    _ =>  bail!("{} is not a vector!", name),
                }
            }
        };
        return Ok( () )
    });
}

use crate::speech::*;
fn read_one_definitions_file(path: &Path) -> Result<()> {
    // read in the file contents   
    let definition_file_contents = read_to_string_shim(path)
            .chain_err(|| format!("trying to read {}", path.to_str().unwrap()))?;

    // callback to do the work of building up the defined vectors/hashmaps (in 'build_values') from YAML
    let defs_build_fn = |variable_def_list: &Yaml| {
        // Rule::DefinitionList
        // debug!("variable_def_list {} is\n{}", yaml_to_type(variable_def_list), yaml_to_string(variable_def_list));
        let vec = crate::speech::as_vec_checked(variable_def_list)
                    .chain_err(||format!("in file {:?}", path.to_str()))?;
        for variable_def in vec {
            build_values(variable_def).chain_err(||format!("in file {:?}", path.to_str()))?;
        }
        return Ok(());
    };

    // Convert the file contents to YAML and call the callback
    return crate::speech::compile_rule(&definition_file_contents, defs_build_fn)
        .chain_err(|| format!("In file '{}'", path.to_str().unwrap()));
}

// Do the work of converting a single YAML def into the vec/hashset
fn build_values(definition: &Yaml) -> Result<()> {
    // Rule::Definition
    let dictionary = crate::speech::as_hash_checked(definition)?;
    if dictionary.len()!=1 {
        bail!("Should only be one definition rule: {}", yaml_to_type(definition));
    }
    let (key, value) = dictionary.iter().next().unwrap();
    let name = key.as_str().ok_or_else(|| format!("definition list name '{}' is not a string", yaml_to_type(key)))?;
    let values = value.as_vec().ok_or_else(|| format!("definition list value '{}' is not an array", yaml_to_type(value)))?;

    return DEFINITIONS.with(|definitions| {
        let name_definition_map = &mut definitions.borrow_mut().name_to_var_mapping;
        let collection = name_definition_map.entry(name.to_string()).or_insert_with_key(|key| {
            if key.starts_with("Numbers") || key.ends_with("_vec") {
                Contains::Vec( Rc::new( RefCell::new( vec![] ) ) )
            } else {
                Contains::Set( Rc::new( RefCell::new( HashSet::new() ) ) )
            }
        });
        match collection {
            Contains::Vec(v) => v.borrow_mut().clear(),
            Contains::Set(s) => s.borrow_mut().clear(),
        };
        for yaml_value in values {
            let value = yaml_value.as_str()
                .ok_or_else(|| format!("list entry '{}' is not a string", yaml_to_type(yaml_value)))?
                .to_string();
            match collection {
                Contains::Vec(v) => { v.borrow_mut().push(value); },
                Contains::Set(s) => { s.borrow_mut().insert(value); },
            }
        }
        return Ok( () );
    });
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_def() {
        let str = r#"[LikelyFunctionNames: ["f", "g", "h", "F", "G", "H", "[A-Za-z]+"]]"#;
        let defs_build_fn = |variable_def_list: &Yaml| {
            // Rule::DefinitionList
            //debug!("variable_def_list {} is\n{}", yaml_to_type(variable_def_list), yaml_to_string(variable_def_list, 0));
            for variable_def in variable_def_list.as_vec().unwrap() {
                if let Err(e) = build_values(variable_def) {
                    bail!("{}", crate::interface::errors_to_string(&e.chain_err(||format!("in file {:?}", str))));
                }
            }
            return Ok(());
        };
        compile_rule(&str, defs_build_fn).unwrap();
        DEFINITIONS.with(|defs| {
            let defs = defs.borrow();
            let names = defs.get_hashset("LikelyFunctionNames");
            assert!(names.is_some());
            let names = names.unwrap();
            assert_eq!(names.len(), 7);
            assert!(names.contains("f"));
            assert!(!names.contains("a"));
        });
    }
}