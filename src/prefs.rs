//! Preferences come from either the user or are programmatically set by the AT.
//! The either source can set any preference, but users and AT typically set different preferences.
//!
//! User prefs are read in from a YAML file (prefs.yaml). The can be written by hand.
//! In the future, there will hopefully be a nice UI that writes out the YAML file.
//!
//! AT prefs are set via the API given in the [crate::interface] module.
//! These in turn call [`PreferenceManager::set_api_string_pref`] and [`PreferenceManager::set_api_float_pref`].
//! Ultimately, user and api prefs are stored in a hashmap.
//!
//! Preferences can be found in a few places:
//! 1. Language-independent prefs found in the Rules dir
//! 2. Language-specific prefs
//! 3. Language-region-specific prefs
//! If there are multiple definitions, the later ones overwrite the former ones.
//! This means that region-specific variants will overwrite more general variants.
//!
//! Note: there are a number of public 'get_xxx' functions that really are meant to be public only to the [crate::speech] module as speech needs access
//! to the preferences to generate the speech.
#![allow(clippy::needless_return)]
use yaml_rust::{Yaml, YamlLoader};
use crate::pretty_print::yaml_to_string;
use crate::tts::TTS;
extern crate dirs;
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use crate::speech::{as_str_checked, RulesFor};
use crate::interface::errors_to_string;
use std::collections::HashMap;
use crate::shim_filesystem::*;
use crate::errors::*;

/// Use to indicate preference not found with Preference::to_string()
pub static NO_PREFERENCE: &str = "\u{FFFF}";

// Preferences are recorded here
/// Preferences are stored in a HashMap. It maps the name of the pref (a String) to its value (stored as YAML string/float)
pub type PreferenceHashMap = HashMap<String, Yaml>;
#[derive(Debug, Clone, Default)]
pub struct Preferences {
    prefs: PreferenceHashMap        // FIX: pub so can get at iterator, should add iterator to Preferences instead
}

use std::fmt; 
impl fmt::Display for Preferences {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pref_vec: Vec<(&String, &Yaml)> = self.prefs.iter().collect();
        pref_vec.sort();
        for (name, value) in pref_vec {
            writeln!(f, "    {}: {}", name, yaml_to_string(value, 0))?;
        }
        return Ok(());
    }
}

impl Preferences{
    // default values needed in case nothing else gets set 
    fn user_defaults() -> Preferences {
        let mut prefs = PreferenceHashMap::with_capacity(39);
        prefs.insert("Language".to_string(), Yaml::String("en".to_string()));
        prefs.insert("SpeechStyle".to_string(), Yaml::String("ClearSpeak".to_string()));
        prefs.insert("Verbosity".to_string(), Yaml::String("medium".to_string()));
        prefs.insert("SpeechOverrides_CapitalLetters".to_string(), Yaml::String("".to_string())); // important for testing
        prefs.insert("Blind".to_string(), Yaml::Boolean(true));
        prefs.insert("MathRate".to_string(), Yaml::String("100.0".to_string()));
        prefs.insert("PauseFactor".to_string(), Yaml::String("100.0".to_string()));
        prefs.insert("NavMode".to_string(), Yaml::String("enhanced".to_string()));
        prefs.insert("Overview".to_string(), Yaml::String("read".to_string()));
        prefs.insert("ResetOverView".to_string(), Yaml::Boolean(true));
        prefs.insert("NavVerbosity".to_string(), Yaml::String("verbose".to_string()));
        prefs.insert("AutoZoomOut".to_string(), Yaml::Boolean(true));
        prefs.insert("BrailleCode".to_string(), Yaml::String("Nemeth".to_string()));
        prefs.insert("BrailleNavHighlight".to_string(), Yaml::String("EndPoints".to_string()));
        prefs.insert("UEB_START_MODE".to_string(), Yaml::String("Grade2".to_string()));
    
        return Preferences{ prefs };
    }

    // default values needed in case nothing else gets set 
    fn api_defaults() -> Preferences {
        let mut prefs = PreferenceHashMap::with_capacity(19);
        prefs.insert("TTS".to_string(), Yaml::String("none".to_string()));
        prefs.insert("Pitch".to_string(), Yaml::Real("0.0".to_string()));
        prefs.insert("Rate".to_string(), Yaml::Real("180.0".to_string()));
        prefs.insert("Volume".to_string(), Yaml::Real("100.0".to_string()));
        prefs.insert("Voice".to_string(), Yaml::String("none".to_string()));
        prefs.insert("Gender".to_string(), Yaml::String("none".to_string()));
        prefs.insert("Bookmark".to_string(), Yaml::Boolean(false));
        prefs.insert("CapitalLetters_UseWord".to_string(), Yaml::Boolean(true));
        prefs.insert("CapitalLetters_Pitch".to_string(), Yaml::Real("0.0".to_string()));
        prefs.insert("CapitalLetters_Beep".to_string(), Yaml::Boolean(false));
        prefs.insert("IntentErrorRecovery".to_string(), Yaml::String("IgnoreIntent".to_string()));    // also Error
        return Preferences{ prefs };
    }

    // Before we can get the other files, we need the preferences.
    // To get them we need to read pref files, so the pref file reading is different than the other files
    fn from_file(rules_dir: &Path) -> Result<(Preferences, FilesAndTimes)> {
        let (system_pref_file, user_pref_file) = Preferences::get_system_and_user_prefs(rules_dir);
        
        let defaults = DEFAULT_USER_PREFERENCES.with(|defaults| defaults.clone());
        let merged_prefs = Preferences::read_file(system_pref_file.clone(), defaults)?;
        let merged_prefs = Preferences::read_file(user_pref_file.clone(), merged_prefs)?;

        let mut files = FilesAndTimes::default();
        if let Some(system) = system_pref_file {
            files.ft.push( FileAndTime::new(system) );
        }
        if let Some(user) = user_pref_file {
            files.ft.push( FileAndTime::new(user) );
        }

        return Ok((merged_prefs, files));
    }

    /// Returns a tuple (system prefs, user prefs)
    fn get_system_and_user_prefs(rules_dir: &Path) -> (Option<PathBuf>, Option<PathBuf>) {
        let mut system_prefs_file = rules_dir.to_path_buf();
        system_prefs_file.push("prefs.yaml");

        let system_prefs_file = if is_file_shim(&system_prefs_file) {
            Some(system_prefs_file)
        } else {
            error!("Couldn't open file {}.\nUsing fallback defaults which may be inappropriate.",
                        system_prefs_file.to_str().unwrap());
            None
        };

        let user_dir = dirs::config_dir();
        let mut user_prefs_file = None;
        if let Some(mut user_prefs_file_pathbuf) = user_dir {
            user_prefs_file_pathbuf.push("MathCAT/prefs.yaml");
            if is_file_shim(&user_prefs_file_pathbuf) {
                user_prefs_file = Some(user_prefs_file_pathbuf);
            }           
        };

        return (system_prefs_file, user_prefs_file);
    }

    fn read_file(file: Option<PathBuf>, mut base_prefs: Preferences) -> Result<Preferences> {
        let unwrapped_file = match file {
            None => return Ok(base_prefs),
            Some(f) => f,
        };

        let file_name = unwrapped_file.to_str().unwrap();
        let docs;
        debug!("read_file in prefs.rs");
        match read_to_string_shim(&unwrapped_file) {
            Err(e) => {
                bail!("Couldn't read file {}\n{}", file_name, e);
            },
            Ok( file_contents) => {
                match YamlLoader::load_from_str(&file_contents) {
                    Err(e) => {
                        error!("Yaml parse error ('{}') in file {}.\nUsing fallback defaults which may be inappropriate.",
                                    e, file_name);
                        return Ok(base_prefs);
                    },
                    Ok(d) => docs = d,
                }

            }
        }
        if docs.len() != 1 {
            error!("Yaml error in file {}.\nFound {} 'documents' -- should only be 1.",
                        file_name, docs.len());
            return Ok(base_prefs);
        }

        let doc = &docs[0];

        if cfg!(debug_assertions) {
            verify_keys(doc, "Speech", file_name)?;
            verify_keys(doc, "Navigation", file_name)?;
            verify_keys(doc, "Braille", file_name)?;
            verify_keys(doc, "Other", file_name)?;
        }

        let prefs = &mut base_prefs.prefs;
        add_prefs(prefs, &doc["Speech"], "", file_name);
        add_prefs(prefs, &doc["Navigation"], "", file_name);
        add_prefs(prefs, &doc["Braille"], "", file_name);
        add_prefs(prefs, &doc["Other"], "", file_name);
        return Ok( Preferences{ prefs: prefs.to_owned() } );



        fn verify_keys(dict: &Yaml, key: &str, file_name: &str) -> Result<()> {
            let prefs = &dict[key];
            if prefs.is_badvalue() {
                bail!("Yaml error in file {}.\nDidn't find '{}' key.", file_name, key);
            }
            if prefs.as_hash().is_none() {
                bail!("Yaml error in file {}.\n'{}' key is not a dictionary. Value found is {}.",
                            file_name, key, yaml_to_string(dict, 1));
            }
            return Ok(());
        }

        fn add_prefs(map: &mut PreferenceHashMap, new_prefs: &Yaml, name_prefix: &str, file_name: &str) {
            if new_prefs.is_badvalue() || new_prefs.is_null() || new_prefs.as_hash().is_none() {
                return;
            }
            let new_prefs = new_prefs.as_hash().unwrap();
            for (yaml_name, yaml_value) in new_prefs {
                let name = as_str_checked(yaml_name);
                if let Err(e) = name {
                    error!("{}", (&e.chain_err(||
                        format!("name '{}' is not a string in file {}", yaml_to_string(yaml_name, 0), file_name))));
                } else {
                    match yaml_value {
                        Yaml::Hash(_) => add_prefs(map, yaml_value, &(name.unwrap().to_string() + "_"), file_name),
                        Yaml::Array(_) => error!("name '{}' has illegal array value {} in file '{}'",
                                                 yaml_to_string(yaml_name, 0), yaml_to_string(yaml_value, 0), file_name),
                        Yaml::String(_) | Yaml::Boolean(_) | Yaml::Integer(_) | Yaml::Real(_) => {
                            let trimmed_name = name_prefix.to_string() + name.unwrap().trim();
                            let mut yaml_value = yaml_value.to_owned();
                            if let Some(value) = yaml_value.as_str() {
                                yaml_value = Yaml::String(value.to_string());
                            }
                            map.insert(trimmed_name, yaml_value);
                        },
                        _ => error!("name '{}' has illegal {:#?} value {} in file '{}'",
                                    yaml_to_string(yaml_name, 0), yaml_value, yaml_to_string(yaml_value, 0), file_name),
                    }
                }                  
            }
        }
    }

    #[allow(dead_code)]     // used in testing
    fn set_string_value(&mut self, name: &str, value: &str) {
        self.prefs.insert(name.to_string(), Yaml::String(value.trim().to_string()));
    }

    #[allow(dead_code)]     // used in testing
    fn set_bool_value(&mut self, name: &str, value: bool) {
        self.prefs.insert(name.to_string(), Yaml::Boolean(value));
    }
}

/// During initialization or when a preference is changed that requires reading a file, the following is done:
/// 1. We search for the starting file for the speech style, navigation, unicode, definitions, and braille
/// 2. We read the files in and follow any "include"s they have. A Vector of included files is returned along with the rules being stored.
/// 3. We set the FilesAndTimes (includes a timestamp) associated with that instance variable in the PreferenceManager.
/// 
/// Anytime one of those files is used, an "is_valid" check is made that checks with the the timestamp on the file has changed.
/// If so, we repeat the above steps.
/// 


#[derive(Debug, Clone)]
struct FileAndTime {
    file: PathBuf,
    time: SystemTime,
}

impl FileAndTime {
    fn new(file: PathBuf) -> FileAndTime {
        return FileAndTime {
            file: PathBuf::from(file),
            time: SystemTime::UNIX_EPOCH,
        }
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

    fn invalidate(&mut self) {
        // just enough so that is_valid() will say that this does match a flushed out value
        self.ft[0].time = SystemTime::UNIX_EPOCH;
    }

    pub fn is_valid(&self) -> bool {
        return !self.ft.is_empty() && self.ft[0].time != SystemTime::UNIX_EPOCH;
    }

    fn is_file_up_to_date(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        for file in &self.ft {
            let file_mod_time = file.file.metadata().unwrap().modified().unwrap();
            if file_mod_time > file.time {  // file was modified after we read it/set the time
                return false;
            }
        }
        return true;
    }

    pub fn paths(&self) -> Vec<PathBuf> {
        return self.ft.iter().map(|ft| ft.file.clone()).collect::<Vec<PathBuf>>();
    }

    fn set_files_and_times(&mut self, new_files: Option<Vec<PathBuf>>)  {
        self.ft.clear();
        if let Some(files) = new_files  {
            for path in files {
                let time = FilesAndTimes::get_metadata(&path);      // do before move below
                self.ft.push( FileAndTime{ file: path, time: time })
            }
        }
    }

    // fn set_rule_value(&mut self, rules_dir: &Path, lang: &str, default_lang: Option<&str>, file_name: &str) -> Result<()> {
    //     let start_file = PreferenceManager::find_file(rules_dir, lang, default_lang, file_name)?;
    //     if self.is_valid() && &start_file == self.as_path() {
    //         return Ok( () );
    //     }

    //     self.ft.clear();
    //     let new_files = match file_name {
    //             "overview.yaml" => crate::speech::OVERVIEW_RULES.with(|rules| rules.borrow_mut().read_patterns(&start_file))?,
    //             "navigate.yaml" => crate::speech::NAVIGATION_RULES.with(|rules| rules.borrow_mut().read_patterns(&start_file))?,
    //             "intent.yaml" => crate::speech::INTENT_RULES.with(|rules| rules.borrow_mut().read_patterns(&start_file))?,
    //             _ => bail!("set_value called with unknown file name='{}'", file_name),
    //     };

    //     if let Some(files) = new_files  {
    //         for path in files {
    //             let time = FilesAndTimes::get_metadata(&path);      // do before move below
    //             self.ft.push( FileAndTime{ file: path, time: time })
    //         }
    //     }
    //     return Ok( () );
    // }

    // fn set_unicode_value(&mut self, rules_dir: &Path, lang: &str, default_lang: Option<&str>, file_name: &str) -> Result<()> {
    //     let start_file = PreferenceManager::find_file(rules_dir, lang, default_lang, file_name)?;
    //     if self.is_valid() && &start_file == self.as_path() {
    //         return Ok( () );
    //     }

    //     self.ft.clear();
    //     let new_files = read_definitions_file(&self)?;  // FIX: should read unicode
    //     if let Some(files) = new_files  {
    //         for path in files {
    //             let time = FilesAndTimes::get_metadata(&path);      // do before move below
    //             self.ft.push( FileAndTime{ file: path, time: time })
    //         }
    //     }
    //     return Ok( () );
    // }

    // fn set_definition_value(&mut self, rules_dir: &Path, lang: &str, default_lang: Option<&str>, file_name: &str) -> Result<()> {
    //     let start_file = PreferenceManager::find_file(rules_dir, lang, default_lang, file_name)?;
    //     if self.is_valid() && &start_file == self.as_path() {
    //         return Ok( () );
    //     }

    //     self.ft.clear();
    //     let new_files = read_definitions_file(&self)?;
    //     if let Some(files) = new_files  {
    //         for path in files {
    //             let time = FilesAndTimes::get_metadata(&path);      // do before move below
    //             self.ft.push( FileAndTime{ file: path, time: time })
    //         }
    //     }
    //     return Ok( () );
    // }

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

    pub fn as_path(&self) -> &Path {
        return &self.ft[0].file;
    }
}


thread_local!{
    static DEFAULT_USER_PREFERENCES: Preferences = Preferences::user_defaults();
    static DEFAULT_API_PREFERENCES: Preferences = Preferences::api_defaults();
    static PREF_MANAGER: Rc<RefCell<PreferenceManager>> = 
            Rc::new( RefCell::new( PreferenceManager::default() ) );

}

/// PreferenceManager keeps track of user and api prefs along with current files
///
/// If one one the `FileAndTime` files changes while the program is running, the values will auto-update
/// Among other things, that means that a UI that changes a user pref will be reflected the next time someone gets speech, braille, etc.
//
// Note: I experimented with PREF_MANAGER being a Result<PreferenceManager> in the case of no rule files,
//   but it ended up being a mess (lots of unwrapping). Having a field is much cleaner.
//   Also note that if 'error' is not an empty string, SpeechRules can't work so using those requires a check.
#[derive(Debug, Default)]
pub struct PreferenceManager {
    rules_dir: PathBuf,                       // full path to rules dir
    error: String,                            // empty/default string if fields are set, otherwise error message
    user_prefs: Preferences,                  // prefs that come from reading prefs.yaml (system and user locations)
    api_prefs: Preferences,                   // prefs set by API calls (along with some defaults not in the user settings such as "pitch")
    pref_files: FilesAndTimes,                // the "raw" user preference files (converted to 'user_prefs')
    intent: FilesAndTimes,                    // the intent rule style file(s)
    speech: FilesAndTimes,                    // the speech rule style file(s)
    overview: FilesAndTimes,                  // the overview rule file(s)
    navigation: FilesAndTimes,                // the navigation rule file(s)
    speech_unicode: FilesAndTimes,            // short unicode.yaml file(s)
    speech_unicode_full: FilesAndTimes,       // full unicode.yaml file(s)
    braille: FilesAndTimes,                   // the braille rule file
    braille_unicode: FilesAndTimes,           // short braille unicode file
    braille_unicode_full: FilesAndTimes,      // full braille unicode file
    speech_defs: FilesAndTimes,               // the definition.yaml file(s)
    braille_defs: FilesAndTimes,              // the definition.yaml file(s)
}


impl fmt::Display for PreferenceManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "PreferenceManager:")?;
        if self.error.is_empty() {
            writeln!(f, "  not initialized!!! Error is {}", &self.error)?;
        } else {
            writeln!(f, "  user prefs:\n{}", self.user_prefs)?;
            writeln!(f, "  api prefs:\n{}", self.api_prefs)?;
            writeln!(f, "  style files: {:?}", self.speech.as_path())?;
            writeln!(f, "  unicode files: {:?}", self.speech_unicode.as_path())?;
            writeln!(f, "  intent files: {:?}", self.intent.as_path())?;
            writeln!(f, "  speech definition files: {:?}", self.speech_defs.ft.iter().map(|ft| ft.file.to_string_lossy()).collect::<Vec<Cow<str>>>().join(", "))?;
            writeln!(f, "  braille definition files: {:?}", self.braille_defs.ft.iter().map(|ft| ft.file.to_string_lossy()).collect::<Vec<Cow<str>>>().join(", "))?;
        }
        return Ok(());
    }
}

#[derive(Default)]
pub struct FilesChanged {
    pub speech_rules: bool,
    pub speech_unicode_short: bool,
    pub speech_unicode_full: bool,
    pub braille_rules: bool,
    pub braille_unicode_short: bool,
    pub braille_unicode_full: bool,
    pub intent: bool,
    pub speech_defs: bool,
    pub braille_defs: bool,
    pub navigate_rules: bool,
    pub overview_rules: bool,
}

impl fmt::Display for FilesChanged {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "FilesChanged {{\n  Speech: rules {}, short {}, full {}", self.speech_rules, self.speech_unicode_short, self.speech_unicode_full)?;
        writeln!(f, "  Braille: rules {}, short {}, full {}", self.braille_rules, self.braille_unicode_short, self.braille_unicode_full)?;
        writeln!(f, "  Intent {}, Defs {}/{}", self.intent, self.speech_defs, self.braille_defs)?;
        return Ok(());
    }
}

impl FilesChanged {
    pub fn new(pref_name: &str) -> Option<FilesChanged> {
        return match pref_name {
            "Language" => Some(FilesChanged{
                speech_rules: true,
                speech_unicode_short: true,
                speech_unicode_full: true,
                braille_rules: false,
                braille_unicode_short: false,
                braille_unicode_full: false,
                intent: false,
                speech_defs: true,
                braille_defs: true,
                navigate_rules: true,
                overview_rules: true,
            }),
            "SpeechStyle" => Some( FilesChanged {
                speech_rules: true, 
                ..Default::default()
            }),
            "BrailleCode" => Some( FilesChanged {
                braille_rules: true, 
                braille_unicode_short: true, 
                braille_unicode_full: true, 
                ..Default::default()
            }),
            _ => None,
        }
    }
}

impl PreferenceManager {
    /// Initialize (the) PreferenceManager (a global var).
    /// 'rules_dir' is the path to "Rules" unless the env var MathCATRulesDir is set
    /// 
    /// If rules_dir is an empty PathBuf, the existing rules_dir is used (an error if it doesn't exist)
    pub fn initialize(&mut self, rules_dir: PathBuf) -> Result<()> {
        let mut rules_dir = rules_dir;
        if rules_dir.as_os_str().is_empty() {
            rules_dir = self.rules_dir.clone();
        }

        // first, read in the preferences -- need to determine which files to read next
        // the prefs files are in the rules dir and the user dir; differs from other files
        if self.api_prefs.prefs.is_empty() {
            self.api_prefs = Preferences{ prefs: DEFAULT_API_PREFERENCES.with(|defaults| defaults.prefs.clone()) };
        }

        let pref_files;
        if  self.user_prefs.prefs.is_empty() || !self.pref_files.is_file_up_to_date() {
            let (user_prefs, pref_files_internal) = Preferences::from_file(&rules_dir)?;
            self.user_prefs = user_prefs;
            pref_files = Some(pref_files_internal);
        } else {
            pref_files = None;
        }

        match PreferenceManager::find_rules_dir(&rules_dir) {
            Ok(rules_dir) => {
                #[allow(clippy::unnecessary_unwrap)]  // using 'if let...' is messy here
                let pref_files = if pref_files.is_some() {pref_files.unwrap()} else {Preferences::from_file(&rules_dir)?.1};
                match self.set_all_files(&rules_dir, pref_files) {
                    Ok(_) => {
                        self.error = String::new();
                        return Ok(())
                    },
                    Err(e) => self.error = errors_to_string(&e),
                }
            },
            Err(e) => {
                self.error = errors_to_string(&e);
            },
        };
        bail!("{}", self.error);
    }


    pub fn get() -> Rc<RefCell<PreferenceManager>> {
        return PREF_MANAGER.with( |pm| pm.clone() );
    }

    pub fn get_error(&self) -> &str {
        return &self.error;
    }

    /// Return a `PreferenceHashMap` that is the merger of the api prefs onto the user prefs.
    pub fn merge_prefs(&self) -> PreferenceHashMap {
        let mut merged_prefs = self.user_prefs.prefs.clone();
        merged_prefs.extend(self.api_prefs.prefs.clone());
        return merged_prefs;
    }

    fn set_all_files(&mut self, rules_dir: &Path, pref_files: FilesAndTimes) -> Result<()> {
        // try to find ./Rules/lang/style.yaml and ./Rules/lang/style.yaml
        // we go through a series of fallbacks -- we try to maintain the language if possible

        let style_file_name = self.pref_to_string("SpeechStyle") + "_Rules.yaml";
        // FIX: should look for other style files in the same language dir if one is not found before move to default
        
        let language = self.pref_to_string("Language");
        let language = if language.as_str() == "Auto" {"en"} else {language.as_str()};       // avoid 'temp value dropped while borrowed' error

        self.rules_dir = rules_dir.to_path_buf();
        self.pref_files = pref_files;
        let mut speech_rules_dir = rules_dir.to_path_buf();
        speech_rules_dir.push("Languages");

    
        PreferenceManager::initialize_value(&mut self.intent, &speech_rules_dir, language, Some("en"), "intent.yaml")?;
        PreferenceManager::initialize_value(&mut self.speech, &speech_rules_dir, language, Some("en"), &style_file_name)?;
        PreferenceManager::initialize_value(&mut self.overview, &speech_rules_dir, language, Some("en"), "overview.yaml")?;
        PreferenceManager::initialize_value(&mut self.navigation, &speech_rules_dir, language, Some("en"), "navigate.yaml")?;

        PreferenceManager::initialize_value(&mut self.speech_unicode, &speech_rules_dir, language, Some("en"), "unicode.yaml")?;
        PreferenceManager::initialize_value(&mut self.speech_unicode_full, &speech_rules_dir, language, Some("en"), "unicode-full.yaml")?;
        PreferenceManager::initialize_value(&mut self.speech_defs, &speech_rules_dir, language, Some("en"), "definitions.yaml")?;

        let mut braille_rules_dir = rules_dir.to_path_buf();
        braille_rules_dir.push("Braille");
        let braille_code = self.pref_to_string("BrailleCode");
        let braille_file = braille_code.clone() + "_Rules.yaml";
        PreferenceManager::initialize_value(&mut self.braille, &braille_rules_dir, &braille_code, Some("Nemeth"), &(braille_file))?;

        PreferenceManager::initialize_value(&mut self.braille_unicode, &braille_rules_dir, &braille_code, Some("Nemeth"), "unicode.yaml")?;
        PreferenceManager::initialize_value(&mut self.braille_unicode_full, &braille_rules_dir, &braille_code, Some("Nemeth"), "unicode-full.yaml")?;

        // FIX: this is hack -- it should probably be part of the braille rules. See #12
        let lang_for_braille = match self.pref_to_string("BrailleCode").as_str() {
            "Vietnam" => "vi",
            "CMU" => if language == "pt" {"pt"} else {"es"},
            "UEB" => "en",
            "Nemeth" | _ => if language == "el" {"el"} else {"en"},
        };
        PreferenceManager::initialize_value(&mut self.braille_defs, &braille_rules_dir, &braille_code, Some("UEB"), "definitions.yaml")?;
        return Ok(());
    }

    fn initialize_value(value: &mut FilesAndTimes, rules_dir: &Path, lang: &str, default_lang: Option<&str>, file_name: &str) -> Result<()> {
        let start_path = PreferenceManager::find_file(rules_dir, lang, default_lang, file_name)?;
        *value = FilesAndTimes::new(start_path);
        return Ok( () );
    }



    /// Find a file matching `file_name` by starting in the regional directory and looking to the language.
    /// If that fails, fall back to looking for the default repeating the same process -- something needs to be found or MathCAT crashes
    fn find_file(rules_dir: &Path, lang: &str, default_lang: Option<&str>, file_name: &str) -> Result<PathBuf> {
        // rules_dir: is the root of the search
        //   to that we add the language dir(s)
        //   if file_name doesn't exist in the language dir(s), we try to find it in the default dir
        //   the exception to this is if it ends with _Rules.yaml, we look for other _Rules.yaml files
        // returns the location of the file_name found

        // start by trying to find a dir that exists
        let mut lang_dir = get_language_dir(rules_dir, lang);
        let mut default_lang = default_lang;
        if lang_dir.is_none() {
            // try again with the default lang if there is one
            if default_lang.is_some() {
                lang_dir = get_language_dir(rules_dir, default_lang.unwrap());
                if lang_dir.is_none() {
                    // We are done for -- MathCAT can't do anything without the required files!
                    bail!("Wasn't able to find/read directory for language {}\n
                           Wasn't able to find/read MathCAT default language directory: {}",
                          lang, rules_dir.join(default_lang.unwrap_or("")).as_os_str().to_str().unwrap());
                }

                // the default lang dir exists -- prevent retrying with it.
                default_lang = None;
                warn!("Couldn't find rules for language {}, ", lang)
            }
        }

        // now find the file name in the dirs
        // we start with the deepest dir and walk back to towards Rules
        let mut result = None;
        let lang_dir = lang_dir.unwrap();
        for os_path in lang_dir.ancestors() {
            let path = PathBuf::from(os_path).join(file_name);
            if is_file_shim(&path) {
                result = Some(path);
                break;
            };
            if os_path.ends_with("Rules") {
                break;
            }
        }

        if let Some(result) = result {
            return Ok(result);     // found at least one file
        }

        if let Some(default_lang) = default_lang {
            if file_name.ends_with("_Rules.yaml") {
                // try to find other style files
                match get_fallback_speech_style(lang_dir.as_path()) {
                    None => {
                        // didn't find a file -- retry with default
                        // FIX: give a warning that default dir is being used
                        return PreferenceManager::find_file(rules_dir, default_lang, None, file_name);
                    },
                    Some(file_name) => {
                        // found a speech style, retry with that
                        return PreferenceManager::find_file(rules_dir, lang, Some(default_lang), &file_name);
                    }
                }
            }
        }
        
        // We are done for -- MathCAT can't do anything without the required files!
        bail!("Wasn't able to find/read MathCAT required file in directory: {}\n\
               Initially looked in there for language specific directory: {}\n\
               Looking for file: {}",
            rules_dir.to_str().unwrap(), lang, file_name);

        fn get_language_dir(rules_dir: &Path, lang: &str) -> Option<PathBuf> {
            // return 'Rules/Language/fr', 'Rules/Language/en/gb', etc, if they exist.
            // fall back to main language, and then to default_dir if language dir doesn't exist
            let mut full_path = rules_dir.to_path_buf();
            let lang_parts = lang.split('-');
            for part in lang_parts {
                full_path.push(Path::new(part));
                if !is_dir_shim(&full_path) {
                    break;
                }
            }
    
            // make sure something got added...
            if rules_dir == full_path {
                return None;    // didn't find a dir
            } else {
                return Some(full_path);
            }
        }
    
        fn get_fallback_speech_style(lang_dir: &Path) -> Option<String> {
            for os_path in lang_dir.ancestors() {
                if os_path.ends_with("Rules") {
                    return None;
                }
                let entries = match os_path.read_dir() {
                    Err(_) => return None,          // I don't think this is possible
                    Ok(os_path) => os_path,
                };
                for dir_entry in entries.flatten() {
                    if dir_entry.file_name().to_str().unwrap_or("").ends_with("_Rules.yaml") {
                        return Some(dir_entry.file_name().to_str().unwrap().to_string());
                    }
                }
            }
            return None;
        }
    }
    
    fn find_rules_dir(rules_dir: &Path) -> Result<PathBuf> {
        let mut bad_env_value = String::default();
        if let Ok(env_var) = std::env::var("MathCATRulesDir") {
            let path_buf = PathBuf::from(&env_var);
            if is_dir_shim(&path_buf) {
                return Ok(path_buf);
            }
            bad_env_value = format!("MathCATRulesDir='{}' is not a directory -- ignoring\n", &env_var);
            warn!("{}", &bad_env_value);
        }
        
        if is_dir_shim(rules_dir) {
            return Ok(PathBuf::from(rules_dir));
        };

        // we are done for -- can't do anything without a rules dir
        bail!("MathCAT could not find a rules dir -- something failed in installation?\n{}Could not find rules dir in {} or lacking permissions to read the dir!",
                    &bad_env_value, rules_dir.to_str().unwrap_or("rules dir is none???"));
    }

    pub fn is_up_to_date(&mut self) -> Result<Option<FilesChanged>> {
        if !self.pref_files.is_file_up_to_date() {
            self.invalidate_old_prefs()?
        }

        let files_changed = FilesChanged {
            speech_rules: !self.speech.is_file_up_to_date(),
            speech_unicode_short: !self.speech_unicode.is_file_up_to_date(),
            speech_unicode_full: !self.speech_unicode_full.is_file_up_to_date(),
            braille_rules: !self.braille.is_file_up_to_date(),
            braille_unicode_short: !self.braille_unicode.is_file_up_to_date(),
            braille_unicode_full: !self.braille_unicode_full.is_file_up_to_date(),
            intent: !self.intent.is_file_up_to_date(),
            speech_defs: !self.speech_defs.is_file_up_to_date(),
            braille_defs: !self.braille_defs.is_file_up_to_date(),
            navigate_rules: !self.navigation.is_file_up_to_date(),
            overview_rules: !self.overview.is_file_up_to_date(),
        };

        if files_changed.speech_rules ||
           files_changed.speech_unicode_short ||
           files_changed.speech_unicode_full ||
           files_changed.braille_rules ||
           files_changed.braille_unicode_short ||
           files_changed.braille_unicode_full ||
           files_changed.intent ||
           files_changed.speech_defs ||
           files_changed.braille_defs ||
           files_changed.navigate_rules ||
           files_changed.overview_rules {
            return Ok( Some(files_changed) );
        } else {
            return Ok(None);
        }
    }

    fn invalidate_old_prefs(&mut self) -> Result<()> {
        // Note: to_string() is needed because &str is borrowed from self and resetting the value causes a problem
        let old_language = self.user_prefs.prefs.get("Language").unwrap().as_str().unwrap().to_string();
        let old_style = self.user_prefs.prefs.get("SpeechStyle").unwrap().as_str().unwrap().to_string();
        let old_braille = self.user_prefs.prefs.get("BrailleCode").unwrap().as_str().unwrap().to_string();
        (self.user_prefs, self.pref_files) = Preferences::from_file(self.rules_dir.as_path())?;

        let new_language = self.user_prefs.prefs.get("Language").unwrap().as_str().unwrap().to_string();
        let new_style = self.user_prefs.prefs.get("SpeechStyle").unwrap().as_str().unwrap().to_string();
        let new_braille = self.user_prefs.prefs.get("BrailleCode").unwrap().as_str().unwrap().to_string();
        if old_language != new_language {
            self.invalidate(&FilesChanged::new("Language").unwrap());
        }
        if old_style != new_style {
            self.invalidate(&FilesChanged::new("SpeechStyle").unwrap());
        }
        if old_braille != new_braille {
            self.invalidate(&FilesChanged::new("BrailleCode").unwrap());
        }
        return Ok( () )
    }

    pub fn invalidate(&mut self, files_changed: &FilesChanged) {
        if files_changed.speech_rules {
            self.speech.invalidate();
        }
        if files_changed.speech_unicode_short {
            self.speech_unicode.invalidate();
        }
        if files_changed.speech_unicode_full {
            self.speech_unicode_full.invalidate();
        }
        if files_changed.braille_rules {
            self.braille.invalidate();
        }
        if files_changed.braille_unicode_short {
            self.braille_unicode.invalidate();
        }
        if files_changed.braille_unicode_full {
            self.braille_unicode_full.invalidate();
        }
        if files_changed.intent {
            self.intent.invalidate();
        }
        if files_changed.speech_defs {
            self.speech_defs.invalidate();
        }
        if files_changed.braille_defs {
            self.braille_defs.invalidate();
        }
        if files_changed.navigate_rules {
            self.navigation.invalidate();
        }
        if files_changed.overview_rules {
            self.overview.invalidate();
        }
    }

    /// Return the speech rule style file locations.
    pub fn get_rule_file(&self, name: &RulesFor) -> &Path {
        if !self.error.is_empty() {
            panic!("Internal error: get_rule_file called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        let files = match name {
            RulesFor::Intent => &self.intent,
            RulesFor::Speech => &self.speech,
            RulesFor::OverView => &self.overview,
            RulesFor::Navigation => &self.navigation,
            RulesFor::Braille => &self.braille,
        };
        return files.as_path();
    }

    /// Return the unicode.yaml file locations.
    pub fn get_speech_unicode_file(&self) ->(&Path, &Path) {
        if !self.error.is_empty() {
            panic!("Internal error: get_speech_unicode_file called on invalid PreferenceManager -- error message\n{}", &self.error);
        };
        return (self.speech_unicode.as_path(), self.speech_unicode_full.as_path());
    }

    /// Return the unicode.yaml file locations.
    pub fn get_braille_unicode_file(&self) -> (&Path, &Path) {
        if !self.error.is_empty() {
            panic!("Internal error: get_braille_unicode_file called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        return (self.braille_unicode.as_path(), self.braille_unicode_full.as_path());
    }

    /// Return the definitions.yaml file locations.
    pub fn get_definitions_file(&mut self, use_speech_defs: bool) -> &FilesAndTimes {
        if !self.error.is_empty() {
            panic!("Internal error: get_definitions_file called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        let defs_file = if use_speech_defs {&self.speech_defs} else {&self.braille_defs};
        return defs_file;
    }

    /// Return the TTS engine currently in use.
    pub fn get_tts(&self) -> TTS {
        if !self.error.is_empty() {
            panic!("Internal error: get_tts called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        return match self.pref_to_string("TTS").as_str().to_ascii_lowercase().as_str() {
            "none" => TTS::None,
            "ssml" => TTS::SSML,
            "sapi5" => TTS::SAPI5,
            _ => {
                warn!("found unknown value for TTS: '{}'", self.pref_to_string("TTS").as_str());
                TTS::None
            }
        }
    }

    /// Set the string-valued preference.
    pub fn set_api_string_pref(&mut self, key: &str, value: &str) {
        if !self.error.is_empty() {
            panic!("Internal error: set_api_string_pref called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        self.api_prefs.prefs.insert(key.to_string(), Yaml::String(value.to_string()));
    }

    /// Set the number-valued preference.
    /// All number-valued preferences are stored with type `f64`.
    pub fn set_api_float_pref(&mut self, key: &str, value: f64) {
        if !self.error.is_empty() {
            panic!("Internal error: set_api_float_pref called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        self.api_prefs.prefs.insert(key.to_string(), Yaml::Real(value.to_string()));
    }

    pub fn set_api_boolean_pref(&mut self, key: &str, value: bool) {
        if !self.error.is_empty() {
            panic!("Internal error: set_api_boolean_pref called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        self.api_prefs.prefs.insert(key.to_string(), Yaml::Boolean(value));
    }

    /// Return the current speech rate.
    pub fn get_rate(&self) -> f64 {
        if !self.error.is_empty() {
            panic!("Internal error: get_rate called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        return match &self.pref_to_string("Rate").parse::<f64>() {
            Ok(val) => *val,
            Err(_) => {
                warn!("Rate ('{}') can't be converted to a floating point number", &self.pref_to_string("Rate"));
                DEFAULT_API_PREFERENCES.with(|defaults| defaults.prefs["Rate"].as_f64().unwrap())
            }
        };
    }

    pub fn get_api_prefs(&self) -> &Preferences {
        return &self.api_prefs;
    }

    /// returns value associated with 'name' or string NO_PREFERENCE
    /// 
    /// Note: Option/Result not used because most of the time we know the preference exists, so no unwrapping is needed for 95% of calls
    pub fn pref_to_string(&self, name: &str) -> String {
        let mut value = self.api_prefs.prefs.get(name);
        if value.is_none() {
            value = self.user_prefs.prefs.get(name);
        }
        return match value {
            None => NO_PREFERENCE.to_string(),
            Some(v) => match v {
                Yaml::String(s) => s.clone(),
                Yaml::Boolean(b)   => b.to_string(),
                Yaml::Integer(i)    => i.to_string(),
                Yaml::Real(s) => s.clone(),
                _  => NO_PREFERENCE.to_string(),       // shouldn't happen
            }
        }
    }

    // occasionally useful to check a pref value when debugging
    // fn get_pref(&self, pref_name: &str) -> String {
    //     return yaml_to_string(self.user_prefs.prefs.get(pref_name).unwrap(), 1);
    // }

    /// Warning!!! This is meant for testing only -- it overwrites any values from a user pref file and will be overwritten if the file is reread.
    /// Probably set_preference() is more appropriately called.
    pub fn set_user_prefs(&mut self, name: &str, value: &str) {
        if !self.error.is_empty() {
            panic!("Internal error: set_user_prefs called on invalid PreferenceManager -- error message\n{}", &self.error);
        };
        self.user_prefs.set_string_value(name, value);
    }
}


#[cfg(test)]
mod tests {

    use crate::set_preference;

    // For these tests, it is assumed that there are Rules subdirs zz and zz/aa dir; there is no zz/ab
    // definitions.yaml is in Rules, zz, aa dirs
    // unicode.yaml is in zz
    // ClearSpeak_Rules.yaml is in zz
    use super::*;

    /// Version of abs_rules_dir_path that returns a PathBuf
    fn abs_rules_dir_path() -> PathBuf {
        return PathBuf::from(super::super::abs_rules_dir_path());
    }
    /// Return a relative path to Rules dir (ie, .../Rules/zz... returns zz/...)
    /// strip .../Rules from file path
    fn rel_path<'a>(rules_dir: &'a Path, path: &'a Path) -> &'a Path {
        let stripped_path = path.strip_prefix(rules_dir).unwrap();
        return stripped_path;    
    }

    #[test]
    fn find_simple_style() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "en");
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak");
            pref_manager.invalidate(&FilesChanged::new("Language").unwrap());
            pref_manager.initialize(PathBuf::new()).unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/en/ClearSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn find_style_other_language() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "en");
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak");
            pref_manager.set_user_prefs("Language", "zz");
            pref_manager.invalidate(&FilesChanged::new("Language").unwrap());
            pref_manager.initialize(PathBuf::new()).unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/ClearSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn find_unicode_files() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak");
            pref_manager.set_user_prefs("Language", "zz-aa");
            pref_manager.invalidate(&FilesChanged::new("Language").unwrap());
            pref_manager.initialize(PathBuf::new()).unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/ClearSpeak_Rules.yaml"));

            pref_manager.set_user_prefs("Language", "zz-aa");
            pref_manager.invalidate(&FilesChanged::new("Language").unwrap());
            pref_manager.initialize(PathBuf::new()).unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/aa/ClearSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn find_style_no_sublanguage() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak");
            pref_manager.set_user_prefs("Language", "zz-ab");
            pref_manager.invalidate(&FilesChanged::new("Language").unwrap());
            pref_manager.initialize(PathBuf::new()).unwrap();

            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/ClearSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn found_all_files() {
        fn count_files(ft: &FilesAndTimes) -> usize {
            return ft.ft.len();
        }

        fn assert_helper(count: usize, correct: usize, file_name: &str) {
            assert_eq!(count, correct, "In looking for '{}', found {} files, should have found {}", 
                    file_name, count, correct);
        }

        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak");
            pref_manager.set_user_prefs("Language", "zz-aa");
            pref_manager.set_user_prefs("BrailleCode", "UEB");
            let mut files_changed = FilesChanged::new("Language").unwrap();
            files_changed.braille_rules = true;
            files_changed.braille_unicode_short = true;
            files_changed.braille_unicode_full = true;
            pref_manager.invalidate(&files_changed);
            pref_manager.initialize(PathBuf::new()).unwrap();
            
            assert_helper(count_files(&pref_manager.speech), 2, "ClearSpeak_Rules.yaml");
            assert_helper(count_files(&pref_manager.speech_unicode), 1, "unicode.yaml");
            assert_helper(count_files(&pref_manager.braille), 1, "Nemeth_Rules.yaml");
            assert_helper(count_files(&pref_manager.braille_unicode), 1, "unicode.yaml");
            assert_helper(count_files(&pref_manager.intent), 1, "intent.yaml");
            assert_helper(pref_manager.speech_defs.ft.len(), 3,"definitions.yaml");
    
            pref_manager.set_user_prefs("Language", "zz-ab");
            pref_manager.invalidate(&FilesChanged::new("Language").unwrap());
            pref_manager.initialize(PathBuf::new()).unwrap();

            assert_helper(count_files(&pref_manager.speech), 1, "ClearSpeak_Rules.yaml");
            assert_helper(count_files(&pref_manager.speech_unicode), 1, "unicode.yaml");
            assert_helper(count_files(&pref_manager.braille), 1, "Nemeth_Rules.yaml");
            assert_helper(count_files(&pref_manager.braille_unicode), 1, "unicode.yaml");
            assert_helper(count_files(&pref_manager.intent), 1, "intent.yaml");
            assert_helper(pref_manager.braille_defs.ft.len(), 2, "definitions.yaml");
        })
    }

    #[test]
    fn file_found_order() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "zz-aa");
            pref_manager.invalidate(&FilesChanged::new("Language").unwrap());
            pref_manager.initialize(PathBuf::new()).unwrap();

            assert_eq!(pref_manager.speech_defs.ft.len(), 3);
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech_defs.as_path()), Path::new("Languages/zz/aa/definitions.yaml"));
        });
    }

    #[test]
    fn test_prefs() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();

            pref_manager.set_user_prefs("Language", "en");
            pref_manager.set_user_prefs("SubjectArea", "General");
            pref_manager.set_user_prefs("ClearSpeak_AbsoluteValue", "Auto");
            pref_manager.set_user_prefs("ResetNavMode", "false");
            pref_manager.set_user_prefs("BrailleCode", "Nemeth");
            let mut files_changed = FilesChanged::new("Language").unwrap();
            files_changed.braille_rules = true;
            files_changed.braille_unicode_short = true;
            files_changed.braille_unicode_full = true;
            pref_manager.invalidate(&files_changed);
            pref_manager.initialize(PathBuf::new()).unwrap();

            assert_eq!(pref_manager.pref_to_string("Language").as_str(), "en");
            assert_eq!(pref_manager.pref_to_string("SubjectArea").as_str(), "General");
            assert_eq!(pref_manager.pref_to_string("ClearSpeak_AbsoluteValue").as_str(), "Auto");
            assert_eq!(pref_manager.pref_to_string("ResetNavMode").as_str(), "false");
            assert_eq!(pref_manager.pref_to_string("BrailleCode").as_str(), "Nemeth");
            assert_eq!(pref_manager.pref_to_string("X_Y_Z").as_str(), NO_PREFERENCE);
        });
    }

    #[test]
    fn test_language_change() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "en");
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak");
            pref_manager.invalidate(&FilesChanged::new("Language").unwrap());
            pref_manager.initialize(PathBuf::new()).unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.get_rule_file(&RulesFor::Speech)), PathBuf::from("Languages/en/ClearSpeak_Rules.yaml"));
        });


        // set_preference borrows the pref manager, so the previous borrow's lifetime needed to be ended by here
        set_preference("Language".to_string(), "zz".to_string()).unwrap();
        let pref_manager = PreferenceManager::get();
        let mut pref_manager = pref_manager.borrow_mut();
        pref_manager.initialize(PathBuf::new()).unwrap();
        assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.get_rule_file(&RulesFor::Speech)), PathBuf::from("Languages/zz/ClearSpeak_Rules.yaml"));
    }
    
    #[test]
    fn test_speech_style_change() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "en");
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak");
            pref_manager.invalidate(&FilesChanged::new("Language").unwrap());
            pref_manager.initialize(PathBuf::new()).unwrap();

            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.get_rule_file(&RulesFor::Speech)), PathBuf::from("Languages/en/ClearSpeak_Rules.yaml"));

            pref_manager.set_user_prefs("SpeechStyle", "SimpleSpeak");
            pref_manager.invalidate(&FilesChanged::new("SpeechStyle").unwrap());
            pref_manager.initialize(PathBuf::new()).unwrap();
            
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.get_rule_file(&RulesFor::Speech)), PathBuf::from("Languages/en/SimpleSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn test_some_changes() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Verbosity", "Terse");

            assert_eq!(&pref_manager.pref_to_string("Verbosity"), "Terse");

            pref_manager.set_user_prefs("BrailleCode", "UEB");
            pref_manager.invalidate(&FilesChanged::new("BrailleCode").unwrap());
            pref_manager.initialize(PathBuf::new()).unwrap();            
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.get_rule_file(&RulesFor::Braille)), PathBuf::from("Braille/UEB/UEB_Rules.yaml"));
        });
    }

    use std::fs;
    #[test]
    // #[ignore]
    fn test_up_to_date() {
        use std::thread::sleep;
        use std::time::Duration;
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "zz-aa");   // move to a directory where making a time change doesn't really matter
            pref_manager.invalidate(&FilesChanged::new("Language").unwrap());
            pref_manager.initialize(PathBuf::new()).unwrap();

            // First test to make sure the up_to_date check works -- need to do in this test since the order of testing is random
            let files_changed = pref_manager.is_up_to_date().unwrap();        
            assert!(files_changed.is_none(), "files_changed={}", files_changed.unwrap());
            
            // Note: need to use pattern match to avoid borrow problem
            // Don't change a speech related file because 'test_is_up_to_date' might fail 
            let defs_files = pref_manager.get_definitions_file(true);
            let main_file = defs_files.ft[0].file.as_path();
            let contents = fs::read(main_file).expect(&format!("Failed to write file {} during test", main_file.to_string_lossy()));
            #[allow(unused_must_use)] { 
                fs::write(main_file, contents);
                sleep(Duration::from_millis(10));
            }
            let files_changed = pref_manager.is_up_to_date().unwrap();
            assert!(files_changed.is_some());
            let files_changed = files_changed.unwrap();
            assert!(&files_changed.speech_defs);
            assert!(!&files_changed.speech_rules);
            assert!(!&files_changed.speech_unicode_short);

            // open the file, read all the contents, then write them back so the time changes
        });
    }
}
