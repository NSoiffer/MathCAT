//! Preferences come from either the user or are programmatically set by the AT.
//! The either source can set any preference, but users and AT typically set different preferences.
//!
//! User prefs are read in from a YAML file (prefs.yaml). The can be written by hand.
//! In the future, there will hopefully be a nice UI that writes out the YAML file.
//!
//! AT prefs are set via the API given in the [crate::interface] module.
//! These in turn call [`PreferenceManager::set_string_pref`] and [`PreferenceManager::set_api_float_pref`].
//! Ultimately, user and api prefs are stored in a hashmap.
//!
//! Preferences can be found in a few places:
//! 1. Language-independent prefs found in the Rules dir
//! 2. Language-specific prefs
//! 3. Language-region-specific prefs
//! 
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
use std::cell::RefCell;
use std::rc::Rc;
use std::path::{Path, PathBuf};
use crate::speech::{as_str_checked, RulesFor, FileAndTime};
use std::collections::{HashMap, HashSet};
use phf::phf_set;
use crate::shim_filesystem::*;
use crate::errors::*;

/// Use to indicate preference not found with Preference::to_string()
pub static NO_PREFERENCE: &str = "\u{FFFF}";

lazy_static! {
    static ref DEFAULT_LANG: Yaml = Yaml::String("en".to_string());
}


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
        prefs.insert("LanguageAuto".to_string(), Yaml::String("".to_string()));     // illegal value so change will be recognized
        prefs.insert("SpeechStyle".to_string(), Yaml::String("ClearSpeak".to_string()));
        prefs.insert("Verbosity".to_string(), Yaml::String("Medium".to_string()));
        prefs.insert("SpeechOverrides_CapitalLetters".to_string(), Yaml::String("".to_string())); // important for testing
        prefs.insert("Blind".to_string(), Yaml::Boolean(true));
        prefs.insert("MathRate".to_string(), Yaml::Real("100.0".to_string()));
        prefs.insert("PauseFactor".to_string(), Yaml::Real("100.0".to_string()));
        prefs.insert("NavMode".to_string(), Yaml::String("Enhanced".to_string()));
        prefs.insert("Overview".to_string(), Yaml::Boolean(false));
        prefs.insert("ResetOverView".to_string(), Yaml::Boolean(true));
        prefs.insert("NavVerbosity".to_string(), Yaml::String("Verbose".to_string()));
        prefs.insert("AutoZoomOut".to_string(), Yaml::Boolean(true));
        prefs.insert("BrailleCode".to_string(), Yaml::String("Nemeth".to_string()));
        prefs.insert("BrailleNavHighlight".to_string(), Yaml::String("EndPoints".to_string()));
        prefs.insert("UEB_START_MODE".to_string(), Yaml::String("Grade2".to_string()));
        prefs.insert("DecimalSeparators".to_string(), Yaml::String(".".to_string()));
        prefs.insert("BlockSeparators".to_string(), Yaml::String(", \u{00A0}\u{202F}".to_string()));
    
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
        prefs.insert("CheckRuleFiles".to_string(), Yaml::String(
                    (if cfg!(target_family = "wasm") {"None"} else {"Prefs"}).to_string()));    // avoid checking for rule files being changed (40% speedup!) (All, Prefs, None)
        return Preferences{ prefs };
    }

    fn read_prefs_file(file: &Path, mut base_prefs: Preferences) -> Result<Preferences> {
        let file_name = file.to_str().unwrap();
        let docs;
        match read_to_string_shim(file) {
            Err(e) => {
                bail!("Couldn't read file {}\n{}", file_name, e);
            },
            Ok( file_contents) => {
                match YamlLoader::load_from_str(&file_contents) {
                    Err(e) => {
                        bail!("Yaml parse error ('{}') in preference file {}.", e, file_name);
                    },
                    Ok(d) => docs = d,
                }

            }
        }
        if docs.len() != 1 {
            bail!("MathCAT: error in prefs file '{}'.\nFound {} 'documents' -- should only be 1.", file_name, docs.len());
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
    rules_dir: PathBuf,                   // full path to rules dir
    error: String,                        // empty/default string if fields are set, otherwise error message
    user_prefs: Preferences,              // prefs that come from reading prefs.yaml (system and user locations)
    api_prefs: Preferences,               // prefs set by API calls (along with some defaults not in the user settings such as "pitch")
    sys_prefs_file: Option<FileAndTime>,  // the system prefs.yaml file
    user_prefs_file: Option<FileAndTime>, // the user prefs.yaml file
    intent: PathBuf,                      // the intent rule style file
    speech: PathBuf,                      // the speech rule style file
    overview: PathBuf,                    // the overview rule file
    navigation: PathBuf,                  // the navigation rule file
    speech_unicode: PathBuf,              // short unicode.yaml file
    speech_unicode_full: PathBuf,         // full unicode.yaml file
    speech_defs: PathBuf,                 // the definition.yaml file
    braille: PathBuf,                     // the braille rule file
    braille_unicode: PathBuf,             // short braille unicode file
    braille_unicode_full: PathBuf,        // full braille unicode file
    braille_defs: PathBuf,                // the definition.yaml file
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
            writeln!(f, "  speech definition files: {:?}", self.speech_defs)?;
            writeln!(f, "  braille definition files: {:?}", self.braille_defs)?;
        }
        return Ok(());
    }
}

impl PreferenceManager {
    /// Initialize (the) PreferenceManager (a global var).
    /// 'rules_dir' is the path to "Rules" unless the env var MathCATRulesDir is set
    /// 
    /// If rules_dir is an empty PathBuf, the existing rules_dir is used (an error if it doesn't exist)
    pub fn initialize(&mut self, rules_dir: PathBuf) -> Result<()> {
        #[cfg(not(target_family = "wasm"))]
        let rules_dir = match rules_dir.canonicalize() {
            Err(e) => bail!("set_rules_dir: could not canonicalize path {}: {}", rules_dir.display(), e.to_string()),
            Ok(rules_dir) =>  rules_dir,
        };

        self.set_rules_dir(&rules_dir)?;
        self.set_preference_files()?;
        self.set_all_files(&rules_dir)?;
        return Ok( () );
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

    /// Set the rules dir and return failure if it is a bad directory (non-existent, can't find all files, ...)
    fn set_rules_dir(&mut self, rules_dir: &Path) -> Result<()> {
        // Fix: should make sure all files exists -- fail if not true
        if !is_dir_shim(rules_dir) {
            bail!("Unable to find MathCAT Rules directory '{}'", rules_dir.to_string_lossy())
        }
        self.rules_dir = rules_dir.to_path_buf();
        return Ok( () );
    }

    /// Read the preferences from the files (if not up to date) and set the preferences and preference files
    /// Returns failure if the files don't exist or have errors
    pub fn set_preference_files(&mut self) -> Result<()> {
        // first, read in the preferences -- need to determine which files to read next
        // the prefs files are in the rules dir and the user dir; differs from other files
        if self.api_prefs.prefs.is_empty() {
            self.api_prefs = Preferences{ prefs: DEFAULT_API_PREFERENCES.with(|defaults| defaults.prefs.clone()) };
        }

        let should_update_system_prefs = self.sys_prefs_file.is_none() || !self.sys_prefs_file.as_ref().unwrap().is_up_to_date();
        let should_update_user_prefs = self.user_prefs_file.is_none() || !self.user_prefs_file.as_ref().unwrap().is_up_to_date();
        if !(should_update_system_prefs || should_update_user_prefs) {
            return Ok( () );            // no need to do anything else
        }

        let mut prefs = Preferences::default();

        let mut system_prefs_file = self.rules_dir.to_path_buf();
        system_prefs_file.push("prefs.yaml");
        if is_file_shim(&system_prefs_file) {
            let defaults = DEFAULT_USER_PREFERENCES.with(|defaults| defaults.clone());
            prefs = Preferences::read_prefs_file(&system_prefs_file, defaults)?;
            self.sys_prefs_file = Some( FileAndTime::new_with_time(system_prefs_file.clone()) );
        } else {
            error!("MathCAT couldn't open file system preference file '{}'.\nUsing fallback defaults which may be inappropriate.",
                        system_prefs_file.to_str().unwrap());
        };

        let mut user_prefs_file = dirs::config_dir();
        if let Some(mut user_prefs_file_path_buf) = user_prefs_file {
            user_prefs_file_path_buf.push("MathCAT/prefs.yaml");
            if is_file_shim(&user_prefs_file_path_buf) {
                prefs = Preferences::read_prefs_file(&user_prefs_file_path_buf, prefs)?;
            }
            // set the time otherwise keeps needing to do updates
            self.user_prefs_file = Some( FileAndTime::new_with_time(user_prefs_file_path_buf.clone()) );
            user_prefs_file = Some(user_prefs_file_path_buf);
        }

        if prefs.prefs.is_empty() {
            let user_prefs_file_name = match user_prefs_file {
                None => "No user config directory".to_string(),
                Some(file) => file.to_string_lossy().to_string(),
            };
            bail!("Didn't find preferences in rule directory ('{}') or user directory ('{}')", &system_prefs_file.to_string_lossy(), user_prefs_file_name);
        }
        self.set_files_based_on_changes(&prefs)?;
        self.user_prefs = prefs;

        // set computed values for BLOCK_SEPARATORS and DECIMAL_SEPARATORS (a little messy about the language due immutable and mutable borrows)
        let language = self.user_prefs.prefs.get("Language").unwrap_or(&DEFAULT_LANG).clone();
        let language = language.as_str().unwrap();
        self.set_separators(language)?;
        
        return Ok( () );
    }

    fn set_all_files(&mut self, rules_dir: &Path) -> Result<()> {
        // try to find ./Rules/lang/style.yaml and ./Rules/lang/style.yaml
        // we go through a series of fallbacks -- we try to maintain the language if possible

        let language = self.pref_to_string("Language");
        let language = if language.as_str() == "Auto" {"en"} else {language.as_str()};       // avoid 'temp value dropped while borrowed' error
        let language_dir = rules_dir.to_path_buf().join("Languages");
        self.set_speech_files(&language_dir, language, None)?;  // also sets style file

        let braille_code = self.pref_to_string("BrailleCode");
        let braille_dir = rules_dir.to_path_buf().join("Braille");
        self.set_braille_files(&braille_dir, &braille_code)?;
        return Ok(());
    }

    fn set_speech_files(&mut self, language_dir: &Path, language: &str, new_speech_style: Option<&str>) -> Result<()> {
        PreferenceManager::unzip_files(language_dir, language, Some("en"))?;
        self.intent = PreferenceManager::find_file(language_dir, language, Some("en"), "intent.yaml")?;
        self.overview = PreferenceManager::find_file(language_dir, language, Some("en"), "overview.yaml")?;
        self.navigation = PreferenceManager::find_file(language_dir, language, Some("en"), "navigate.yaml")?;

        self.speech_unicode = PreferenceManager::find_file(language_dir, language, Some("en"), "unicode.yaml")?;
        self.speech_unicode_full = PreferenceManager::find_file(language_dir, language, Some("en"), "unicode-full.yaml")?;

        self.speech_defs = PreferenceManager::find_file(language_dir, language, Some("en"), "definitions.yaml")?;

        match new_speech_style {
            Some(style_name) => self.set_style_file(language_dir, language, style_name)?,
            // use the old style name if one isn't given
            None => self.set_style_file(language_dir, language, &self.pref_to_string("SpeechStyle"))?,
        }
        return Ok( () );
    }

    fn set_style_file(&mut self, language_dir: &Path, language: &str, style_file_name: &str) -> Result<()> {
        let style_file_name = style_file_name.to_string() + "_Rules.yaml";
        self.speech = PreferenceManager::find_file(language_dir, language, Some("en"), &style_file_name)?;
        return Ok( () );
    }

    fn set_braille_files(&mut self, braille_rules_dir: &Path, braille_code_name: &str) -> Result<()> {
        // Fix: Currently the braille code and the directory it lives in have to have the same name
        PreferenceManager::unzip_files(braille_rules_dir, braille_code_name, Some("UEB"))?;

        let braille_file = braille_code_name.to_string() + "_Rules.yaml";

        self.braille = PreferenceManager::find_file(braille_rules_dir, braille_code_name, Some("UEB"), &(braille_file))?;

        self.braille_unicode = PreferenceManager::find_file(braille_rules_dir, braille_code_name, Some("UEB"), "unicode.yaml")?;
        self.braille_unicode_full = PreferenceManager::find_file(braille_rules_dir, braille_code_name, Some("UEB"), "unicode-full.yaml")?;

        self.braille_defs = PreferenceManager::find_file(braille_rules_dir, braille_code_name, Some("UEB"), "definitions.yaml")?;
        return Ok( () );
    }

    /// If some preferences have changed, we may need to recompute other ones
    /// The key prefs are Language, SpeechStyle, and BrailleCode, along with DecimalSeparator
    fn set_files_based_on_changes(&mut self, new_prefs: &Preferences) -> Result<()> {
        let old_language = self.user_prefs.prefs.get("Language");       // not set if first time
        if old_language.is_none() {
            return Ok( () );            // if "Language" isn't set yet, nothing else is either -- first time through, so no updating needed.
        }

        let old_language = old_language.unwrap();
        let new_language = new_prefs.prefs.get("Language").unwrap();
        if old_language != new_language {
            let language_dir = self.rules_dir.to_path_buf().join("Languages");
            self.set_speech_files(&language_dir, new_language.as_str().unwrap(), None)?;  // also sets style file
        } else {
            let old_speech_style = self.user_prefs.prefs.get("SpeechStyle").unwrap();
            let new_speech_style = new_prefs.prefs.get("SpeechStyle").unwrap();
            let language_dir = self.rules_dir.to_path_buf().join("Languages");
            if old_speech_style != new_speech_style {
                self.set_speech_files(&language_dir, new_language.as_str().unwrap(), new_speech_style.as_str())?;
            }
        }

        let old_braille_code = self.user_prefs.prefs.get("BrailleCode").unwrap();
        let new_braille_code = new_prefs.prefs.get("BrailleCode").unwrap();
        if old_braille_code != new_braille_code {
            let braille_code_dir = self.rules_dir.to_path_buf().join("Braille");
            self.set_braille_files(&braille_code_dir, new_braille_code.as_str().unwrap())?;  // also sets style file
        }

        return Ok( () );
    }

    /// Unzip the files if needed
    /// Returns true if it unzipped them
    pub fn unzip_files(path: &Path, language: &str, default_lang: Option<&str>) -> Result<bool> {
        thread_local!{
            /// when a language/braille code dir is unzipped, it is recorded here
            static UNZIPPED_FILES: RefCell<HashSet<String>> = RefCell::new( HashSet::with_capacity(31));
        }
        
        // ignore regional subdirs
        let dir = PreferenceManager::get_language_dir(path, language, default_lang)?;
        let zip_file_name = language.to_string() + ".zip";
        let zip_file_path = dir.join(&zip_file_name);
        let zip_file_string = zip_file_path.to_string_lossy().to_string();
        if UNZIPPED_FILES.with( |unzipped_files| unzipped_files.borrow().contains(&zip_file_string)) {
            return Ok(false);
        }

        let result = match zip_extract_shim(&dir, &zip_file_name) {
            Err(e) => {
                if language.contains('-') {
                    // try again in parent dir of regional language
                    let language = language.split('-').next().unwrap_or(language);
                    return PreferenceManager::unzip_files(path, language, default_lang);
                }
                bail!("Couldn't open zip file {}: {}.", zip_file_string, e)
            },
            Ok(result) => {
                result
            },
        };
        UNZIPPED_FILES.with( |unzipped_files| unzipped_files.borrow_mut().insert(zip_file_string) );
        return Ok(result);
    }

    /// Set BlockSeparators and DecimalSeparators
    /// FIX: changing these values could change the parse, so we really should reparse the original expr, but that doesn't exist anymore (store the original string???)
    fn set_separators(&mut self, language_country: &str) -> Result<()> {
        // This list was generated from https://en.wikipedia.org/wiki/Decimal_separator#Countries_using_decimal_point
        // The countries were then mapped to language(s) using https://en.wikipedia.org/wiki/List_of_official_languages_by_country_and_territory
        // When a language was used in other countries that used a "," separator, the language+country is listed 
        //   Sometimes there are multiple languages used in a country -- they are all listed, sometimes with a country code
        // The country code isn't used when the language is used in smaller countries (i.e, when "." is more likely correct)
        //   This decision is sometimes a bit arbitrary
        //   For example, Swahili (sw) is used in: Democratic Republic of the Congo, Kenya, Rwanda, Tanzania, and Uganda.
        //   Of these, Kenya, Tanzania, and Uganda are listed as using "." and I include Swahili in the list below.
        static USE_DECIMAL_SEPARATOR: phf::Set<&str> = phf_set! {
            "en", "bn", "km", "el-cy", "tr-cy", "zh", "es-do", "ar", "es-sv", "es-gt", "es-hn", "hi", "as", "gu", "kn", "ks",
            "ml", "mr", "ne", "or", "pa", "sa", "sd", "ta", "te", "ur", "he", "ja", "sw", "ko", "de-li", "ms", "dv", "mt", "es-mx", "my",
            "af-na", "es-ni", "es-pa", "fil", "ms-sg", "si", "th",
            "es-419", // latin america
        };
        
        let decimal_separator = self.pref_to_string("DecimalSeparator");
        if !["Auto", ",", "."].contains(&decimal_separator.as_str()) {
            return Ok( () );
        }

        if language_country == "Auto" && decimal_separator == "Auto" {
            return Ok( () );        // "Auto" doesn't tell us anything -- we will get called again when Language is set
        }

        let language_country = language_country.to_ascii_lowercase();
        let language_country = &language_country;
        let mut lang_country_split = language_country.split('-');
        let language = lang_country_split.next().unwrap_or("");
        let country = lang_country_split.next().unwrap_or("");
        let mut use_period = decimal_separator == ".";
        if decimal_separator == "Auto" {
            // if we don't have a match for the lang-country, then just try lang
            use_period = USE_DECIMAL_SEPARATOR.contains(language_country) || USE_DECIMAL_SEPARATOR.contains(language);
        }
        // debug!("set_separators: use_period: {}", use_period);
        self.user_prefs.prefs.insert("DecimalSeparators".to_string(), Yaml::String((if use_period {"."} else {","}).to_string()));
        let mut block_separators =  (if use_period {", \u{00A0}\u{202F}"} else {". \u{00A0}\u{202F}"}).to_string();
        if country == "ch" || country == "li" { // Switzerland and Liechtenstein also use ` as a block separator, at least in some cases
            block_separators.push('\'');
        }
        self.user_prefs.prefs.insert("BlockSeparators".to_string(), Yaml::String(block_separators));
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
        let lang_dir = PreferenceManager::get_language_dir(rules_dir, lang, default_lang)?;
        // now find the file name in the dirs
        // we start with the deepest dir and walk back to towards Rules
        let mut alternative_style_file = None;      // back up in case we don't find the target style in lang_dir
        let looking_for_style_file = file_name.ends_with("_Rules.yaml");
        for os_path in lang_dir.ancestors() {   // ancestor returns self and ancestors
            let path = PathBuf::from(os_path).join(file_name);
            // debug!("find_file: checking file: {}", path.to_string_lossy());
            if is_file_shim(&path) {
                // we make an exception for definitions.yaml -- there a language specific checks for Hundreds, etc
                if !(file_name == "definitions.yaml" && os_path.ends_with("Rules")) {
                    return Ok(path);
                }
            };
            if looking_for_style_file && alternative_style_file.is_none() {
                if let Ok(alt_file_path) = find_any_style_file(os_path) {
                    alternative_style_file = Some(alt_file_path);
                }
            }
            if os_path.ends_with("Rules") {
                // at root of Rules directory
                break;
            }
        }

        if let Some(result) = alternative_style_file {
            // debug!("find_file: found alternative_style_file '{}'", result.to_string_lossy());
            return Ok(result);     // found an alternative style file in the same lang dir
        }

        if let Some(default_lang) = default_lang {
            // try again with the default language (we're likely in trouble)
            return PreferenceManager::find_file(rules_dir, default_lang, None, file_name);
        }
        
        // We are done for -- MathCAT can't do anything without the required files!
        bail!("Wasn't able to find/read MathCAT required file in directory: {}\n\
               Initially looked in there for language specific directory: {}\n\
               Looking for file: {}",
            rules_dir.to_str().unwrap(), lang, file_name);

        // try to find a xxx_Rules.yaml file -- returns an error if none is found ()
        fn find_any_style_file(path: &Path) -> Result<PathBuf> {    
            // try to find a xxx_Rules.yaml file
            // we find the first file because this is the deepest (most language specific) speech rule file
            match find_file_in_dir_that_ends_with_shim(path, "_Rules.yaml") {
                None => bail!{"didn't find file"},
                Some(file_name) => return Ok(path.to_path_buf().join(file_name)),
            }
        }
    }

    fn get_language_dir(rules_dir: &Path, lang: &str, default_lang: Option<&str>) -> Result<PathBuf> {
        // return 'Rules/Language/fr', 'Rules/Language/en/gb', etc, if they exist.
        // fall back to main language, and then to default_dir if language dir doesn't exist
        let mut full_path = rules_dir.to_path_buf();
        full_path.push(lang.replace('-', "/"));
        for parent in full_path.ancestors() {
            if parent == rules_dir {
                break;
            } else if is_dir_shim(parent) {
                return Ok(parent.to_path_buf());
            }
        }

        // didn't find the language -- try again with the default language
        match default_lang {
            Some(default_lang) => {
                warn!("Couldn't find rules for language {}, ", lang);
                return PreferenceManager::get_language_dir(rules_dir, default_lang, None);
            },
            None => {
                // We are done for -- MathCAT can't do anything without the required files!
                bail!("Wasn't able to find/read directory for language {}\n
                        Wasn't able to find/read MathCAT default language directory: {}",
                        lang, rules_dir.join(default_lang.unwrap_or("")).as_os_str().to_str().unwrap());
            }
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
    pub fn get_definitions_file(&self, use_speech_defs: bool) -> &Path {
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
    /// 
    /// Note: changing the language, speech style, or braille code might fail if the files don't exist.
    ///   If this happens, the preference is not set and an error is returned.
    /// If "LanguageAuto" is set, we assume "Language" has already be checked to be "Auto"
    pub fn set_string_pref(&mut self, key: &str, value: &str) -> Result<()> {
        if !self.error.is_empty() {
            panic!("Internal error: set_string_pref called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        // don't do an update if the value hasn't changed
        let mut is_user_pref = true;
        if let Some(pref_value) = self.api_prefs.prefs.get(key) {
            if pref_value.as_str().unwrap() != value {
                is_user_pref = false;
                self.reset_files_from_preference_change(key, value)?;
            }
        } else if let Some(pref_value) = self.user_prefs.prefs.get(key) {
            if pref_value.as_str().unwrap() != value {
                self.reset_files_from_preference_change(key, value)?;
            }
        } else {
            bail!("{} is an unknown MathCAT preference!", key);
        }

        // debug!("Setting ({}) {} to '{}'", if is_user_pref {"user"} else {"sys"}, key, value);
        if is_user_pref {
            // a little messy about the DecimalSeparator due immutable and mutable borrows
            let current_decimal_separator = self.user_prefs.prefs.get("DecimalSeparator").unwrap().clone();
            let current_decimal_separator = current_decimal_separator.as_str().unwrap();
            let is_decimal_separators_changed = key == "DecimalSeparator" && current_decimal_separator != value;
            let is_language_changed = key == "Language" && self.user_prefs.prefs.get("Language").unwrap().as_str().unwrap() != value;
            self.user_prefs.prefs.insert(key.to_string(), Yaml::String(value.to_string()));
            if is_decimal_separators_changed || (current_decimal_separator == "Auto" && is_language_changed) {
                // a little messy about the language due immutable and mutable borrows)
                let language = self.user_prefs.prefs.get("Language").unwrap_or(&DEFAULT_LANG).clone();
                let language = language.as_str().unwrap();
                self.set_separators(language)?;
            }
        } else {
            self.api_prefs.prefs.insert(key.to_string(), Yaml::String(value.to_string()));
        }
        return Ok( () );
    }

    fn reset_files_from_preference_change(&mut self, changed_pref: &str, changed_value: &str) -> Result<()> {       
        if changed_pref == "Language" && changed_value == "Auto" {
            // Language must have had a non-Auto value -- set LanguageAuto to old value so (probable) next change to LanguageAuto works well
            self.api_prefs.prefs.insert("LanguageAuto".to_string(),
                                self.api_prefs.prefs.get("Language").unwrap_or(&DEFAULT_LANG).clone() );
            return Ok( () );
        }

        let changed_pref = if changed_pref == "LanguageAuto" {"Language"} else {changed_pref};
        let language_dir = self.rules_dir.to_path_buf().join("Languages");
        match changed_pref {
            "Language" => {
                self.set_speech_files(&language_dir, changed_value, None)?
            },
            "SpeechStyle" => {
                let language = self.pref_to_string("Language");
                let language = if language.as_str() == "Auto" {"en"} else {language.as_str()};       // avoid 'temp value dropped while borrowed' error
                self.set_style_file(&language_dir, language, changed_value)?
            },
            "BrailleCode" => {
                let braille_dir = self.rules_dir.to_path_buf().join("Braille");
                self.set_braille_files(&braille_dir, changed_value)?
            },
            _ => (),
        }
        return Ok( () );
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
    ///  set_preference() is the function that should be called.
    /// This differs from set_preference in that the user preferences are changed, not the api ones
    pub fn set_user_prefs(&mut self, key: &str, value: &str) -> Result<()> {
        if !self.error.is_empty() {
            panic!("Internal error: set_user_prefs called on invalid PreferenceManager -- error message\n{}", &self.error);
        };
        
        self.reset_files_from_preference_change(key, value)?;
        let is_decimal_separators_changed = key == "DecimalSeparator" && self.user_prefs.prefs.get("DecimalSeparator").unwrap().as_str().unwrap() != value;
        let is_language_changed = key == "Language" && self.user_prefs.prefs.get("Language").unwrap().as_str().unwrap() != value;
        self.user_prefs.prefs.insert(key.to_string(), Yaml::String(value.to_string()));
        if is_decimal_separators_changed || is_language_changed {
            // set computed values for BLOCK_SEPARATORS and DECIMAL_SEPARATORS (a little messy about the language due immutable and mutable borrows)
            let language = self.user_prefs.prefs.get("Language").unwrap_or(&DEFAULT_LANG).clone();
            let language = language.as_str().unwrap();
            self.set_separators(language)?;
        }

        return Ok(());
    }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::init_logger;

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
        return stripped_path
    }

    #[test]
    fn separators() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "en").unwrap();
            pref_manager.set_user_prefs("DecimalSeparator", "Auto").unwrap();
            assert_eq!(&pref_manager.pref_to_string("DecimalSeparators"), ".");
            assert_eq!(&pref_manager.pref_to_string("BlockSeparators"), ", \u{00A0}\u{202F}");

            pref_manager.set_user_prefs("Language", "sv").unwrap();
            assert_eq!(&pref_manager.pref_to_string("DecimalSeparators"), ",");
            assert_eq!(&pref_manager.pref_to_string("BlockSeparators"), ". \u{00A0}\u{202F}");

            // test potentially ambiguous language (defaults to comma decimal separator)
            pref_manager.set_user_prefs("Language", "es").unwrap();
            assert_eq!(&pref_manager.pref_to_string("DecimalSeparators"), ",");
            assert_eq!(&pref_manager.pref_to_string("BlockSeparators"), ". \u{00A0}\u{202F}");

            // test country override
            pref_manager.set_user_prefs("Language", "es-mx").unwrap();
            assert_eq!(&pref_manager.pref_to_string("DecimalSeparators"), ".");
            assert_eq!(&pref_manager.pref_to_string("BlockSeparators"), ", \u{00A0}\u{202F}");

            pref_manager.set_user_prefs("DecimalSeparator", ",").unwrap();
            assert_eq!(&pref_manager.pref_to_string("DecimalSeparators"), ",");
            assert_eq!(&pref_manager.pref_to_string("BlockSeparators"), ". \u{00A0}\u{202F}");

            pref_manager.set_user_prefs("DecimalSeparator", ".").unwrap();
            assert_eq!(&pref_manager.pref_to_string("DecimalSeparators"), ".");
            assert_eq!(&pref_manager.pref_to_string("BlockSeparators"), ", \u{00A0}\u{202F}");

            // set to illegal value -- should leave values as before
            pref_manager.set_user_prefs("DecimalSeparator", ";").unwrap();
            assert_eq!(&pref_manager.pref_to_string("DecimalSeparators"), ".");
            assert_eq!(&pref_manager.pref_to_string("BlockSeparators"), ", \u{00A0}\u{202F}");

            // manual
            pref_manager.set_user_prefs("DecimalSeparators", ",").unwrap();
            pref_manager.set_user_prefs("BlockSeparators", " ").unwrap();
            pref_manager.set_user_prefs("DecimalSeparator", "None").unwrap();
            assert_eq!(&pref_manager.pref_to_string("DecimalSeparators"), ",");
            assert_eq!(&pref_manager.pref_to_string("BlockSeparators"), " ");
        });
    }

    #[test]
    fn find_simple_style() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "en").unwrap();
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak").unwrap();
            assert_eq!(&pref_manager.pref_to_string("Language"), "en");
            assert_eq!(&pref_manager.pref_to_string("SpeechStyle"), "ClearSpeak");
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/en/ClearSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn find_style_other_language() {
        // zz dir should have both ClearSpeak and SimpleSpeak styles
        // zz-aa dir should have only ClearSpeak style and unicode.yaml that includes the zz unicode but overrides "+"
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "en").unwrap();
            pref_manager.set_user_prefs("SpeechStyle", "SimpleSpeak").unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/en/SimpleSpeak_Rules.yaml"));

            pref_manager.set_user_prefs("Language", "zz").unwrap();
            assert_eq!(&pref_manager.pref_to_string("Language"), "zz");
            assert_eq!(&pref_manager.pref_to_string("SpeechStyle"), "SimpleSpeak");
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/SimpleSpeak_Rules.yaml"));

            // make sure language stays the same
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak").unwrap();
            assert_eq!(&pref_manager.pref_to_string("SpeechStyle"), "ClearSpeak");
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/ClearSpeak_Rules.yaml"));

            // make sure language stays the same
            pref_manager.set_user_prefs("SpeechStyle", "SimpleSpeak").unwrap();
            assert_eq!(&pref_manager.pref_to_string("SpeechStyle"), "SimpleSpeak");
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/SimpleSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn find_regional_overrides() {
        // zz dir should have both ClearSpeak and SimpleSpeak styles
        // zz-aa dir should have ClearSpeak style and unicode.yaml that includes the zz unicode but overrides "+"
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak").unwrap();
            pref_manager.set_user_prefs("Language", "zz-aa").unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/aa/ClearSpeak_Rules.yaml"));

            pref_manager.set_user_prefs("SpeechStyle", "SimpleSpeak").unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/SimpleSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn find_style_no_sublanguage() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak").unwrap();
            pref_manager.set_user_prefs("Language", "zz-ab").unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/ClearSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn found_all_files() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak").unwrap();
            pref_manager.set_user_prefs("Language", "zz-aa").unwrap();
            pref_manager.set_user_prefs("BrailleCode", "UEB").unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.intent.as_path()), PathBuf::from("intent.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.overview.as_path()), PathBuf::from("Languages/zz/overview.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech_defs.as_path()), PathBuf::from("Languages/zz/aa/definitions.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/aa/ClearSpeak_Rules.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech_unicode.as_path()), PathBuf::from("Languages/zz/aa/unicode.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech_unicode_full.as_path()), PathBuf::from("Languages/zz/unicode-full.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.braille.as_path()), PathBuf::from("Braille/UEB/UEB_Rules.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.braille_unicode.as_path()), PathBuf::from("Braille/UEB/unicode.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.braille_unicode_full.as_path()), PathBuf::from("Braille/UEB/unicode-full.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.braille_defs.as_path()), PathBuf::from("Braille/UEB/definitions.yaml"));
    
            pref_manager.set_user_prefs("Language", "zz-ab").unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.intent.as_path()), PathBuf::from("intent.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.overview.as_path()), PathBuf::from("Languages/zz/overview.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech_defs.as_path()), PathBuf::from("Languages/zz/definitions.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/ClearSpeak_Rules.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech_unicode.as_path()), PathBuf::from("Languages/zz/unicode.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech_unicode_full.as_path()), PathBuf::from("Languages/zz/unicode-full.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.braille.as_path()), PathBuf::from("Braille/UEB/UEB_Rules.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.braille_unicode.as_path()), PathBuf::from("Braille/UEB/unicode.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.braille_unicode_full.as_path()), PathBuf::from("Braille/UEB/unicode-full.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.braille_defs.as_path()), PathBuf::from("Braille/UEB/definitions.yaml"));
        })
    }

    #[test]
    fn test_prefs() {
        PREF_MANAGER.with(|pref_manager| {
            // first test with internal settings
            {
                let mut pref_manager = pref_manager.borrow_mut();
                pref_manager.initialize(abs_rules_dir_path()).unwrap();
    
                pref_manager.set_user_prefs("Language", "en").unwrap();
                pref_manager.set_user_prefs("ClearSpeak_AbsoluteValue", "Determinant").unwrap();
                pref_manager.set_user_prefs("ResetNavMode", "true").unwrap();
                pref_manager.set_user_prefs("BrailleCode", "Nemeth").unwrap();
                assert_eq!(pref_manager.pref_to_string("Language").as_str(), "en");
                assert_eq!(pref_manager.pref_to_string("SubjectArea").as_str(), "General");
                assert_eq!(pref_manager.pref_to_string("ClearSpeak_AbsoluteValue").as_str(), "Determinant");
                assert_eq!(pref_manager.pref_to_string("ResetNavMode").as_str(), "true");
                assert_eq!(pref_manager.pref_to_string("BrailleCode").as_str(), "Nemeth");
                assert_eq!(pref_manager.pref_to_string("X_Y_Z").as_str(), NO_PREFERENCE);
            }

            // now test with the interface
            {
                use crate::interface::{set_preference, get_preference};
                set_preference("Language".to_string(), "zz".to_string()).unwrap();
                set_preference("ClearSpeak_AbsoluteValue".to_string(), "Cardinality".to_string()).unwrap();
                set_preference("Overview".to_string(), "true".to_string()).unwrap();
                set_preference("BrailleCode".to_string(), "UEB".to_string()).unwrap();
                assert_eq!(&get_preference("Language".to_string()).unwrap(), "zz");
                assert_eq!(&get_preference("ClearSpeak_AbsoluteValue".to_string()).unwrap(), "Cardinality");
                assert_eq!(&get_preference("Overview".to_string()).unwrap(), "true");
                assert_eq!(&get_preference("BrailleCode".to_string()).unwrap(), "UEB");
                assert!(&get_preference("X_Y_Z".to_string()).is_err());

            }
        });
    }

    #[test]
    fn test_language_change() {
        // set_preference borrows the pref manager, so the previous borrow's lifetime needs to be ended before using it
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
        });
        crate::interface::set_preference("Language".to_string(), "en".to_string()).unwrap();
        crate::interface::set_preference("SpeechStyle".to_string(), "ClearSpeak".to_string()).unwrap();
        PREF_MANAGER.with(|pref_manager| {
            let pref_manager = pref_manager.borrow_mut();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.get_rule_file(&RulesFor::Speech)), PathBuf::from("Languages/en/ClearSpeak_Rules.yaml"));
        });

        crate::interface::set_preference("Language".to_string(), "zz".to_string()).unwrap();
        PREF_MANAGER.with(|pref_manager| {
            let pref_manager = pref_manager.borrow_mut();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.get_rule_file(&RulesFor::Speech)), PathBuf::from("Languages/zz/ClearSpeak_Rules.yaml"));
        });
    }
    
    #[test]
    fn test_speech_style_change() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "en").unwrap();
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak").unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.get_rule_file(&RulesFor::Speech)), PathBuf::from("Languages/en/ClearSpeak_Rules.yaml"));

            pref_manager.set_user_prefs("SpeechStyle", "SimpleSpeak").unwrap();
            
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.get_rule_file(&RulesFor::Speech)), PathBuf::from("Languages/en/SimpleSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn test_some_changes() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Verbosity", "Terse").unwrap();

            assert_eq!(&pref_manager.pref_to_string("Verbosity"), "Terse");

            pref_manager.set_user_prefs("BrailleCode", "UEB").unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.get_rule_file(&RulesFor::Braille)), PathBuf::from("Braille/UEB/UEB_Rules.yaml"));

            // make sure they show up when building context for speech generation
            let merged_prefs = pref_manager.merge_prefs();
            assert_eq!(merged_prefs.get("Verbosity").unwrap().as_str().unwrap(), "Terse");
        });

        crate::interface::set_preference("NavVerbosity".to_string(), "Terse".to_string()).unwrap();
        PREF_MANAGER.with(|pref_manager| {
            let pref_manager = pref_manager.borrow_mut();
            let merged_prefs = pref_manager.merge_prefs();
            assert_eq!(merged_prefs.get("NavVerbosity").unwrap().as_str().unwrap(), "Terse");
        });
    }

    #[test]
    #[ignore]   // this is an ugly test for #262 -- it changes the prefs file and so is a bad thing in general
    fn test_up_to_date() {
        use std::fs;
        use std::thread::sleep;
        use std::time::Duration;
        use crate::interface;
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            assert_eq!(&pref_manager.pref_to_string("SpeechStyle"), "ClearSpeak");
            assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/ClearSpeak_Rules.yaml"));
        });
        interface::set_mathml("<math><mo>+</mo><mn>10</mn></math>".to_string()).unwrap();
        assert_eq!(interface::get_spoken_text().unwrap(), "ClearSpeak positive from zz 10");
        
        let mut file_path = PathBuf::default();
        let mut contents = vec![];
        PREF_MANAGER.with(|pref_manager| {
            let pref_manager = pref_manager.borrow();
            if let Some(file_name) = pref_manager.user_prefs_file.as_ref().unwrap().debug_get_file() {
                file_path = PathBuf::from(file_name);
                contents = fs::read(&file_path).expect(&format!("Failed to write file {} during test", file_name));
                let changed_contents = String::from_utf8(contents.clone()).unwrap()
                                .replace("SpeechStyle: ClearSpeak", "SpeechStyle: SimpleSpeak");
                fs::write(&file_path, changed_contents).unwrap();
                sleep(Duration::from_millis(5));  // make sure the time changes enough to be recognized
            }
        });
        assert_eq!(interface::get_spoken_text().unwrap(), "SimpleSpeak positive from zz 10");
        fs::write(&file_path, contents).unwrap();

                // assert_eq!(&pref_manager.pref_to_string("SpeechStyle"), "SimpleSpeak");
                // assert_eq!(rel_path(&pref_manager.rules_dir, pref_manager.speech.as_path()), PathBuf::from("Languages/zz/SimpleSpeak_Rules.yaml"));
    }
}
