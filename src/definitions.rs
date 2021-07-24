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
//! ## Usage
//! There is one field in the `DEFINITIONS` structure corresponding to each of the definitions.
//! A typical example of usage is
//! ```
//!# libmathcat::speech::SPEECH_RULES.with(|_| true);  // forces initialization
//! libmathcat::definitions::DEFINITIONS.with(|definitions| {
//!    let numbers_large = definitions.numbers_large.as_vec().borrow();
//!    assert_eq!(numbers_large[3], "billion");
//! })
//! ```
//! Note: some of the variable are `vec`s and some are `hashset`s.
//! Numbers are typically vectors so that indexing a digit is easy.
//! Others such a `functions_names` are a hashset because you just want to know if an `mi` is a known name or not.
//! The functions `as_vec` and `as_hashset` should be used on the appropriate variable.
//! ## Names
//! The names of "variables" in the definition files use camel case (e.g., "FunctionNames"). In the code, to fit with rust
//! naming conventions, snake case is used (e.g, "function_names"). 
//!
//! See the struct [`Definitions`] for the variables that are read in.

extern crate yaml_rust;
use yaml_rust::{Yaml};
use crate::errors::*; 
use crate::prefs::*;
use std::{cell::RefCell, collections::HashSet,  rc::Rc};
use std::{collections::HashMap, path::PathBuf};

/// An enum to paper over the different types of data access needed.
///
/// Having a Rc<RefCell<FromFileVariable>> seems a bit complicated in terms of types but...
/// 1. The rust book seems to endorse the Rc<RefCell<...>>> approach when there are multiple owners of mutable date.
///     See https://doc.rust-lang.org/book/ch15-05-interior-mutability.html towards the end
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

    /// Used to create a new vector-based definition
    fn vec_with_capacity(size: usize) -> Contains {
        return Contains::Vec(  Rc::new(RefCell::new( Vec::with_capacity(size) )));
    }

    /// Used to get the vector-based value
    pub fn as_vec(&self) -> &Rc<RefCell<Vec<String>>> {
        match self {
            Contains::Vec(v) => { return v; },
            Contains::Set(_) => { panic!("Internal error: as_vec -- self is not a Vec!"); }
        }
    }

    /// Used to create a new hashset-based definition
    fn hashset_with_capacity(size: usize) -> Contains {
        return Contains::Set(  Rc::new(RefCell::new( HashSet::with_capacity(size) )));
    }

    /// Used to get the hashset-based value
    pub fn as_hashset(&self) -> &Rc<RefCell<HashSet<String>>> {
        match self {
            Contains::Set(s) => { return s; },
            Contains::Vec(_) => { panic!("Internal error: as_hashset -- self is not a HashSet!"); }
        }
    }
}
pub type CollectionFromFile = Contains;
type VariableDefHashMap = HashMap<&'static str, CollectionFromFile>;

/// Global structure containing all of the definitions.
/// Each field in the structure corresponds to a named value read in from the `definitions.yaml` files.
///
/// The names of "variables" in the definition files use camel case (e.g., "FunctionNames"). In the code, to fit with rust
/// naming conventions, snake case is used (e.g, "function_names").
///
/// There should only be one instance of this structure ([`DEFINITIONS`])

// FIX: this probably can done with a macro to remove all the repetition
pub struct Definitions {
    pub name_to_var_mapping: Rc<VariableDefHashMap>,
    pub large_operators: CollectionFromFile,
    pub trig_function_names: CollectionFromFile,
    pub function_names: CollectionFromFile,     // trig functions + additional functions

    pub likely_function_names:  CollectionFromFile,
    pub numbers_ones: CollectionFromFile,
    pub numbers_tens: CollectionFromFile,
    pub numbers_hundreds: CollectionFromFile,
    pub numbers_large: CollectionFromFile,

    pub numbers_ordinal_ones: CollectionFromFile,
    pub numbers_ordinal_tens: CollectionFromFile,
    pub numbers_ordinal_hundreds: CollectionFromFile,
    pub numbers_ordinal_large: CollectionFromFile,

    pub numbers_ordinal_plural_ones: CollectionFromFile,
    pub numbers_ordinal_plural_tens: CollectionFromFile,
    pub numbers_ordinal_plural_hundreds: CollectionFromFile,
    pub numbers_ordinal_plural_large: CollectionFromFile,

    pub numbers_ordinal_fractional_ones: CollectionFromFile,
    pub numbers_ordinal_fractional_plural_ones: CollectionFromFile,
}

impl Default for Definitions {
    fn default() -> Self {
        let large_operators =  Contains::hashset_with_capacity(50);
        let trig_function_names =  Contains::hashset_with_capacity(30);
        let function_names =  Contains::hashset_with_capacity(60);
        let likely_function_names =  Contains::hashset_with_capacity(50);

        let numbers_ones =  Contains::vec_with_capacity(50);
        let numbers_tens =  Contains::vec_with_capacity(10);
        let numbers_hundreds =  Contains::vec_with_capacity(10);
        let numbers_large =  Contains::vec_with_capacity(10);

        let numbers_ordinal_ones =  Contains::vec_with_capacity(50);
        let numbers_ordinal_tens =  Contains::vec_with_capacity(10);
        let numbers_ordinal_hundreds =  Contains::vec_with_capacity(10);
        let numbers_ordinal_large =  Contains::vec_with_capacity(10);

        let numbers_ordinal_plural_ones =  Contains::vec_with_capacity(50);
        let numbers_ordinal_plural_tens =  Contains::vec_with_capacity(10);
        let numbers_ordinal_plural_hundreds =  Contains::vec_with_capacity(10);
        let numbers_ordinal_plural_large =  Contains::vec_with_capacity(10);

        let numbers_ordinal_fractional_ones =  Contains::vec_with_capacity(10);
        let numbers_ordinal_fractional_plural_ones =  Contains::vec_with_capacity(10);

        // These are the names of the definitions that get read/stored
        let mut map: VariableDefHashMap = HashMap::with_capacity(20);
        map.insert("LargeOperators", large_operators.clone());
        map.insert("TrigFunctionNames", trig_function_names.clone());
        map.insert("AdditionalFunctionNames", function_names.clone());
        map.insert("LikelyFunctionNames", likely_function_names.clone());

        map.insert("NumbersOnes", numbers_ones.clone());
        map.insert("NumbersTens", numbers_tens.clone());
        map.insert("NumbersHundreds", numbers_hundreds.clone());
        map.insert("NumbersLarge", numbers_large.clone());

        map.insert("NumbersOrdinalOnes", numbers_ordinal_ones.clone());
        map.insert("NumbersOrdinalTens", numbers_ordinal_tens.clone());
        map.insert("NumbersOrdinalHundreds", numbers_ordinal_hundreds.clone());
        map.insert("NumbersOrdinalLarge", numbers_ordinal_large.clone());

        map.insert("NumbersOrdinalPluralOnes", numbers_ordinal_plural_ones.clone());
        map.insert("NumbersOrdinalPluralTens", numbers_ordinal_plural_tens.clone());
        map.insert("NumbersOrdinalPluralHundreds", numbers_ordinal_plural_hundreds.clone());
        map.insert("NumbersOrdinalPluralLarge", numbers_ordinal_plural_large.clone());

        map.insert("NumbersOrdinalFractionalOnes", numbers_ordinal_fractional_ones.clone());
        map.insert("NumbersOrdinalFractionalPluralOnes", numbers_ordinal_fractional_plural_ones.clone());

        Definitions {
            name_to_var_mapping: Rc::new(map),
            large_operators,
            trig_function_names,
            function_names,
            likely_function_names,
            
            numbers_ones,
            numbers_tens,
            numbers_hundreds,
            numbers_large,

            numbers_ordinal_ones,
            numbers_ordinal_tens,
            numbers_ordinal_hundreds,
            numbers_ordinal_large,

            numbers_ordinal_plural_ones,
            numbers_ordinal_plural_tens,
            numbers_ordinal_plural_hundreds,
            numbers_ordinal_plural_large,

            numbers_ordinal_fractional_ones,
            numbers_ordinal_fractional_plural_ones,
        }
    }
}

thread_local!{
    /// Global variable containing all of the definitions.
    /// See [`Definitions`] for more details.
    pub static DEFINITIONS: Definitions = Definitions::default();
}

/// Reads the `definitions.yaml` files specified by `locations`.
///
/// If there is a failure during read, the error is propagated to the caller
pub fn read_definitions_file(locations: &Locations) -> Result<()> {
    // for each file in `locations`, read the contents and process them
    let result = locations.iter()
        .map(|path| 
                match path {
                    None => Ok(()),
                    Some(path) => read_one_definitions_file(&path)
                }
            )
        .collect();
    verify_definitions()?;

    // merge the contents of `TrigFunctions` into a set that contains all the function names (from `AdditionalFunctionNames`).
    DEFINITIONS.with(|definitions| {
        let trig_functions = definitions.trig_function_names.as_hashset().borrow();
        let mut all_functions = definitions.function_names.as_hashset().borrow_mut();
        for trig_function in trig_functions.iter() {
            all_functions.insert(trig_function.clone());
        }
    });
    return result;
}

fn verify_definitions() -> Result<()> {
    // all of the 'numbers-xxx' files should be either size 0 or multiples of tens except:
    //   ...-ones
    //   numbers-plural, which should have a single entry
    return DEFINITIONS.with(|definitions| {
        let name_definition_map = &definitions.name_to_var_mapping;
        for (name,collection) in name_definition_map.iter() {
            if name.find("number").is_some() && name.find("fraction").is_none() {
                match collection {
                    Contains::Vec(v) => {
                        let v = v.borrow();
                        if v.len() == 0 || v.len() % 10 != 0 {
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
fn read_one_definitions_file(path: &PathBuf) -> Result<()> {
    use std::fs; 
    // read in the file contents   
    let definition_file_contents = fs::read_to_string(path)
            .chain_err(|| format!("trying to read {}", path.to_str().unwrap()))?;

    // callback to do the work of building up the defined vectors/hashmaps (in 'build_values') from YAML
    let defs_build_fn = |variable_def_list: &Yaml| {
        // Rule::DefinitionList
        // println!("variable_def_list {} is\n{}", yaml_to_type(variable_def_list), yaml_to_string(variable_def_list));
        match crate::speech::as_vec_checked(&variable_def_list) {
            Err(e) => {
                crate::speech::print_errors(&e.chain_err(||format!("in file {:?}", path.to_str())));
            }
            Ok(vec) => {
                // for each variable that was defined in the file, build the corresponding rust variable in DEFINITIONS
                for variable_def in vec {
                    if let Err(e) = build_values(variable_def) {
                        crate::speech::print_errors(&e.chain_err(||format!("in file {:?}", path.to_str())));
                    }    
                }
            }
        };
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
    let name = &*key.as_str().ok_or_else(|| format!("definition list name '{}' is not a string", yaml_to_type(key)))?;
    let values = value.as_vec().ok_or_else(|| format!("definition list value '{}' is not an array", yaml_to_type(value)))?;

    DEFINITIONS.with(|definitions| {
        let name_definition_map = &definitions.name_to_var_mapping;
        if let Some(collection) = name_definition_map.get::<str>(&name) {
            // found the variable/collection to set -- clear it and then add the values to it
            match collection {
                Contains::Vec(v) => v.borrow_mut().clear(),
                Contains::Set(s) => s.borrow_mut().clear(),
            }
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
        } else {
            return Err( format!("Variable name {} is unknown!", name));
        }
    })?;
    return Ok( () );
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_def() {
        let str = r#"[LikelyFunctionNames: ["f", "g", "h", "F", "G", "H", "[A-Za-z]+"]]"#;
        let defs_build_fn = |variable_def_list: &Yaml| {
            // Rule::DefinitionList
            //println!("variable_def_list {} is\n{}", yaml_to_type(variable_def_list), yaml_to_string(variable_def_list, 0));
            for variable_def in variable_def_list.as_vec().unwrap() {
                if let Err(e) = build_values(variable_def) {
                    crate::speech::print_errors(&e.chain_err(||format!("in file {:?}", str)));
                }
            }
        };
        compile_rule(&str, defs_build_fn).unwrap();
        DEFINITIONS.with(|defs| {
            let names = defs.likely_function_names.as_hashset().borrow();
            assert_eq!(names.len(), 7);
            assert!(names.contains("f"));
            assert!(!names.contains("a"));
        });
    }
}