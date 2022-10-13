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
use std::cell::RefCell;
use std::rc::Rc;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::env;
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
        let mut prefs = PreferenceHashMap::with_capacity(7);
        prefs.insert("Language".to_string(), Yaml::String("en".to_string()));
        prefs.insert("SpeechStyle".to_string(), Yaml::String("ClearSpeak".to_string()));
        prefs.insert("Verbosity".to_string(), Yaml::String("medium".to_string()));
        prefs.insert("SpeechOverrides_CapitalLetters".to_string(), Yaml::String("".to_string())); // important for testing
        prefs.insert("Blind".to_string(), Yaml::Boolean(true));
        prefs.insert("NavMode".to_string(), Yaml::String("enhanced".to_string()));
        prefs.insert("Overview".to_string(), Yaml::String("read".to_string()));
        prefs.insert("ResetOverView".to_string(), Yaml::Boolean(true));
        prefs.insert("NavVerbosity".to_string(), Yaml::String("verbose".to_string()));
        prefs.insert("AutoZoomOut".to_string(), Yaml::Boolean(true));
        prefs.insert("BrailleCode".to_string(), Yaml::String("Nemeth".to_string()));
        prefs.insert("BrailleNavHighlight".to_string(), Yaml::String("EndPoints".to_string()));
    
        return Preferences{ prefs };
    }

    // default values needed in case nothing else gets set 
    fn api_defaults() -> Preferences {
        let mut prefs = PreferenceHashMap::with_capacity(7);
        prefs.insert("TTS".to_string(), Yaml::String("none".to_string()));
        prefs.insert("Pitch".to_string(), Yaml::Real("1.0".to_string()));
        prefs.insert("Rate".to_string(), Yaml::Real("180.0".to_string()));
        prefs.insert("Volume".to_string(), Yaml::Real("100.0".to_string()));
        prefs.insert("Voice".to_string(), Yaml::String("none".to_string()));
        prefs.insert("Gender".to_string(), Yaml::String("none".to_string()));
        prefs.insert("Bookmark".to_string(), Yaml::Boolean(false));
        return Preferences{ prefs };
    }

    // Before we can get the other files, we need the preferences.
    // To get them we need to read pref files, so the pref file reading is different than the other files
    fn from_file(rules_dir: &Path) -> Result<(Preferences, FileAndTime)> {
        let files = Preferences::get_prefs_file_and_time(rules_dir);
        return DEFAULT_USER_PREFERENCES.with(|defaults| {
            let system_prefs = Preferences::read_file(&files.files[0], defaults.clone())?;
            let system_prefs = Preferences::read_file(&files.files[1], system_prefs)?;
            return Ok((system_prefs, files));
        });
    }

    fn get_prefs_file_and_time(rules_dir: &Path) -> FileAndTime {
        let mut system_prefs_file = rules_dir.to_path_buf();
        system_prefs_file.push("prefs.yaml");

        let mut result: [Option<PathBuf>; 3] = [None, None, None];
        if is_file_shim(&system_prefs_file) {
            result[0] = Some( system_prefs_file );
        } else {
            error!("Couldn't open file {}.\nUsing fallback defaults which may be inappropriate.",
                        system_prefs_file.to_str().unwrap());
        }

        let user_dir = dirs::config_dir();
        if let Some(mut user_prefs_file) = user_dir {
            user_prefs_file.push("MathCAT/prefs.yaml");
            if is_file_shim(&user_prefs_file) {
                result[1] = Some( user_prefs_file );
            }            
        }

        return FileAndTime {
            time: if cfg!(target_family = "wasm") {None} else {Some( SystemTime::now() )},
            files: result
        }
    }

    fn read_file(file: &Option<PathBuf>, mut base_prefs: Preferences) -> Result<Preferences> {
        let unwrapped_file = match file {
            None => return Ok(base_prefs),
            Some(f) => f.as_path(),
        };

        let file_name = unwrapped_file.to_str().unwrap();
        let docs;
        match read_to_string_shim(unwrapped_file) {
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
        verify_keys(doc, "Speech", file_name)?;
        verify_keys(doc, "Navigation", file_name)?;
        verify_keys(doc, "Braille", file_name)?;

        let prefs = &mut base_prefs.prefs;
        add_prefs(prefs, &doc["Speech"], "", file_name);
        add_prefs(prefs, &doc["Navigation"], "", file_name);
        add_prefs(prefs, &doc["Braille"], "", file_name);
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
            if new_prefs.is_badvalue() || new_prefs.as_hash().is_none() {
                return;
            }
            let new_prefs = new_prefs.as_hash().unwrap();
            for (yaml_name, yaml_value) in new_prefs {
                let name = as_str_checked(yaml_name);
                if let Err(e) = name {
                    error!("{}", (&e.chain_err(||
                        format!("name '{}' is not a string in file {}", yaml_to_string(yaml_name, 0), file_name))));                   
                } else if yaml_value.as_hash().is_some() {
                        add_prefs(map, yaml_value, &(name.unwrap().to_string() + "_"), file_name);
                } else if yaml_value.as_vec().is_some() {
                    error!("name '{}' has illegal array value {} in file '{}'",
                            yaml_to_string(yaml_name, 0), yaml_to_string(yaml_value, 0), file_name);
                    return;
                } else {
                    let trimmed_name = name_prefix.to_string() + name.unwrap().trim();
                    let mut trimmed_yaml_value = yaml_value.to_owned();
                    if let Some(value) = trimmed_yaml_value.as_str() {
                        trimmed_yaml_value = Yaml::String(value.trim().to_string());
                    }
                    map.insert(trimmed_name, trimmed_yaml_value);
                }
            }
        }
    }

    /// returns value associated with 'name' or string NO_PREFERENCE
    /// 
    /// Note: Option/Result not used because most of the time we know the preference exists, so no unwrapping is needed for 95% of calls
    pub fn to_string(&self, name: &str) -> String {
        let value = self.prefs.get(name);
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

    #[allow(dead_code)]     // used in testing
    fn set_string_value(&mut self, name: &str, value: &str) {
        self.prefs.insert(name.to_string(), Yaml::String(value.trim().to_string()));
    }

    #[allow(dead_code)]     // used in testing
    fn set_bool_value(&mut self, name: &str, value: bool) {
        self.prefs.insert(name.to_string(), Yaml::Boolean(value));
    }
}


/// When looking for a file, there are up to three possible locations tracked by this type
/// in a non-error situation, at least the first slot should be Some(...).
///
/// Preferences can be found in a few places:
/// 1. Language-independent prefs found in the Rules dir
/// 2. Language-specific prefs
/// 3. Language-region-specific prefs
/// If there are multiple definitions, the later ones overwrite the former ones.
/// This means that region-specific variants will overwrite more general variants.
///
/// Note: the first entry is the first thing found, which might be '2' or '3' in the list above.
pub type Locations = [Option<PathBuf>; 3];

#[derive(Debug, Clone, Default)]
struct FileAndTime {
    files: Locations,
    time: Option<SystemTime>       // ~time file was read (used to see if it was updated and needs to be re-read) 
}

impl PartialEq for FileAndTime {
    fn eq(&self, other: &Self) -> bool {
        // FIX: anticipating changing Locations to single PathBuf
        return self.files[0] == other.files[0] && self.time == other.time;
    }
}
impl Eq for FileAndTime {}

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
    rules_dir: Option<PathBuf>,         // full path to rules dir
    error: String,                      // empty/default string if fields are set, otherwise error message
    user_prefs: Preferences,
    api_prefs: Preferences,
    pref_files: FileAndTime,            // the "raw" user preference files (converted to 'user_prefs')
    intent: FileAndTime,                // the intent rule style file(s)
    speech: FileAndTime,                // the speech rule style file(s)
    overview: FileAndTime,              // the overview rule file(s)
    navigation: FileAndTime,            // the navigation rule file(s)
    speech_unicode: FileAndTime,        // short unicode.yaml file(s)
    speech_unicode_full: FileAndTime,   // full unicode.yaml file(s)
    braille: FileAndTime,               // the braille rule file
    braille_unicode: FileAndTime,       // short braille unicode file
    braille_unicode_full: FileAndTime,  // full braille unicode file
    defs: FileAndTime,                  // the definition.yaml file(s)
}



impl fmt::Display for PreferenceManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "PreferenceManager:")?;
        if self.error.is_empty() {
            writeln!(f, "  not initialized!!! Error is {}", &self.error)?;
        } else {
            writeln!(f, "  user prefs:\n{}", self.user_prefs)?;
            writeln!(f, "  api prefs:\n{}", self.api_prefs)?;
            writeln!(f, "  style files: {:?}", self.speech.files)?;
            writeln!(f, "  unicode files: {:?}", self.speech_unicode.files)?;
            writeln!(f, "  intent files: {:?}", self.intent.files)?;
            writeln!(f, "  definition files: {:?}", self.defs.files)?;
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
    pub defs: bool
}

impl fmt::Display for FilesChanged {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "FilesChanged {{\n  Speech: rules {}, short {}, full {}", self.speech_rules, self.speech_unicode_short, self.speech_unicode_full)?;
        writeln!(f, "  Braille: rules {}, short {}, full {}", self.braille_rules, self.braille_unicode_short, self.braille_unicode_full)?;
        writeln!(f, "  Intent {}, Defs {}", self.intent, self.defs)?;
        return Ok(());
    }
}

impl FilesChanged {
    // FIX: this should include all files
    pub fn add_changes(&mut self, additional_changes: FilesChanged) {
        self.speech_rules |= additional_changes.speech_rules;
        self.speech_unicode_short |= additional_changes.speech_unicode_short;
        self.speech_unicode_full  |= additional_changes.speech_unicode_full;
        self.braille_rules |= additional_changes.braille_rules;
        self.braille_unicode_short |= additional_changes.braille_unicode_short;
        self.braille_unicode_full  |= additional_changes.braille_unicode_full;
        self.intent |= additional_changes.intent;
        self.defs |= additional_changes.defs;
    }
}

impl PreferenceManager {
    /// Initialize (the) PreferenceManager (a global var).
    /// 'rules_dir' is the path to "Rules" unless the env var MathCATRulesDir is set
    pub fn initialize(&mut self, rules_dir: PathBuf) -> Result<()> {
        // first, read in the preferences -- need to determine which files to read next
        // the prefs files are in the rules dir and the user dir; differs from other files
        if self.api_prefs.prefs.is_empty() {
            self.api_prefs = Preferences{ prefs: DEFAULT_API_PREFERENCES.with(|defaults| defaults.prefs.clone()) };
        }

        match PreferenceManager::find_rules_dir(&rules_dir) {
            Ok(rules_dir) => {
                let (user_prefs, pref_files) = Preferences::from_file(&rules_dir)?;
                match self.set_all_files(&rules_dir, user_prefs, pref_files) {
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
        return PREF_MANAGER.with( |pf| pf.clone() );
    }

    pub fn get_error(&self) -> &str {
        return &self.error;
    }

    /// Return a `PreferenceHashMap` that is the merger of the api prefs into the user prefs.
    pub fn merge_prefs(&self) -> PreferenceHashMap {
        let mut merged_prefs = self.user_prefs.prefs.clone();
        merged_prefs.extend(self.api_prefs.prefs.clone());
        return merged_prefs;
    }

    fn set_all_files(&mut self, rules_dir: &Path, prefs: Preferences, pref_files: FileAndTime) -> Result<()> {
        // try to find ./Rules/lang/style.yaml and ./Rules/lang/style.yaml
        // we go through a series of fallbacks -- we try to maintain the language if possible

        let style_file_name = prefs.to_string("SpeechStyle") + "_Rules.yaml";
        // FIX: should look for other style files in the same language dir if one is not found before move to default
        
        let language = prefs.to_string("Language");
        let language = language.as_str();       // avoid 'temp value dropped while borrowed' error

        self.rules_dir = Some(rules_dir.to_path_buf());
        self.pref_files = pref_files;
        self.user_prefs = prefs.clone();
        self.intent = PreferenceManager::get_file_and_time(
            rules_dir, language, Some("en"), "intent.yaml")?;
        let mut speech_rules_dir = rules_dir.to_path_buf();
        speech_rules_dir.push("Languages");
        self.speech = PreferenceManager::get_file_and_time(
                        &speech_rules_dir, language, Some("en"), &style_file_name)?;
        self.overview = PreferenceManager::get_file_and_time(
                        &speech_rules_dir, language, Some("en"), "overview.yaml")?;
        self.navigation = PreferenceManager::get_file_and_time(
                        &speech_rules_dir, language, Some("en"), "navigate.yaml")?;

        self.speech_unicode = PreferenceManager::get_file_and_time(
                        &speech_rules_dir, language, Some("en"), "unicode.yaml")?;
        self.speech_unicode_full = PreferenceManager::get_file_and_time(
                        &speech_rules_dir, language, Some("en"), "unicode-full.yaml")?;

        let mut braille_rules_dir = rules_dir.to_path_buf();
        braille_rules_dir.push("Braille");
        let braille_code = prefs.to_string("BrailleCode");
        let braille_file = braille_code.clone() + "_Rules.yaml";
        self.braille = PreferenceManager::get_file_and_time(
                        &braille_rules_dir, &braille_code, Some("Nemeth"), &(braille_file))?;

        self.braille_unicode = PreferenceManager::get_file_and_time(
                        &braille_rules_dir, &braille_code, Some("Nemeth"), "unicode.yaml")?;
        self.braille_unicode_full = PreferenceManager::get_file_and_time(
                        &braille_rules_dir, &braille_code, Some("Nemeth"), "unicode-full.yaml")?;

        self.intent = PreferenceManager::get_file_and_time(
            &speech_rules_dir, language, Some("en"), "intent.yaml")?;
        self.defs = PreferenceManager::get_file_and_time(
            &speech_rules_dir, language, Some("en"), "definitions.yaml")?;
        return Ok(());
    }


    fn get_file_and_time(rules_dir: &Path, lang: &str, default_lang: Option<&str>, file_name: &str) -> Result<FileAndTime> {
        use std::fs;
        let files = PreferenceManager::get_files(rules_dir, lang, default_lang, file_name)?;
        return Ok(FileAndTime {
            time: if cfg!(target_family = "wasm") {None} else {get_metadata(&files[0])},
            files
        });

        fn get_metadata(path: &Option<PathBuf>) -> Option<SystemTime> {
            if let Some(path) = path {
                let metadata = fs::metadata(path);
                if let Ok(metadata) = metadata {
                    if let Ok(mod_time) = metadata.modified() {
                        return Some(mod_time);
                    }
                }
            }
            return None;
        }
    }

   fn get_files(rules_dir: &Path, lang: &str, default_lang: Option<&str>, file_name: &str) -> Result<Locations> {
        // rules_dir: is the root of the search
        //   to that we add the language dir(s)
        //   if file_name doesn't exist in the language dir(s), we try to find it in the default dir
        // returns all the locations of the file_name from Rules downward

        // start by trying to find a dir that exists
        let mut lang_dir = PreferenceManager::get_language_dir(rules_dir, lang);
        let mut default_lang = default_lang;
        if lang_dir.is_none() {
            // try again with the default lang if there is one
            if default_lang.is_some() {
                lang_dir = PreferenceManager::get_language_dir(rules_dir, default_lang.unwrap());
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
        // since the order of the rules should be Rules, Rules/lang, Rules/lang/region,
        //   found files are added starting at the end
        let mut result: Locations = [None, None, None];
        let mut i = 3;
        for os_path in lang_dir.unwrap().ancestors() {
            let path = PathBuf::from(os_path).join(file_name);
            if is_file_shim(&path) {
                i -= 1;
                result[i] =  Some(path);
            };
            if os_path.ends_with("Rules") {
                break;
            }
        }

        if i < 3 {
            result.rotate_left(i);      // move the 'None(s) to the end
            return Ok(result);     // found at least one file
        }

        if let Some(default_lang) = default_lang {
            // didn't find a file -- retry with default
            // FIX: give a warning that default dir is being used
            return PreferenceManager::get_files(rules_dir, default_lang, None, file_name);
        }
        
        // We are done for -- MathCAT can't do anything without the required files!
        bail!("Wasn't able to find/read MathCAT required file in directory: {}\n\
               Initially looked in there for language specific directory: {}\n\
               Looking for file: {}",
            rules_dir.to_str().unwrap(), lang, file_name);
    }

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
    
    fn find_rules_dir(rules_dir: &Path) -> Result<PathBuf> {
        let mut bad_env_value = String::default();
        if let Ok(env_var) = env::var("MathCATRulesDir") {
            let path_buf = PathBuf::from(&env_var);
            if is_dir_shim(&path_buf) {
                return Ok(path_buf);
            }
            bad_env_value = format!("MathCATRulesDir value {} is not a directory -- ignoring\n", &env_var);
            warn!("{}", &bad_env_value);
        }
        
        if is_dir_shim(rules_dir) {
            return Ok(PathBuf::from(rules_dir));
        };

        // we are done for -- can't do anything without a rules dir
        bail!("MathCAT could not find a rules dir -- something failed in installation?\n{}Could not find rules dir in {} or lacking permissions to read the dir!",
                    &bad_env_value, rules_dir.to_str().unwrap_or("rules dir is none???"));
    }

    pub fn is_up_to_date(&mut self) -> Option<FilesChanged> {
        // this will work even if self is invalid
        let mut files_changed = FilesChanged {
            speech_rules: !PreferenceManager::is_file_up_to_date(&self.speech),
            speech_unicode_short: !PreferenceManager::is_file_up_to_date(&self.speech_unicode),
            speech_unicode_full: !PreferenceManager::is_file_up_to_date(&self.speech_unicode_full),
            braille_rules: !PreferenceManager::is_file_up_to_date(&self.braille),
            braille_unicode_short: !PreferenceManager::is_file_up_to_date(&self.braille_unicode),
            braille_unicode_full: !PreferenceManager::is_file_up_to_date(&self.braille_unicode_full),
            intent: !PreferenceManager::is_file_up_to_date(&self.intent),
            defs: !PreferenceManager::is_file_up_to_date(&self.defs),
        };

        if !PreferenceManager::is_file_up_to_date(&self.pref_files) {
            let old_lang = self.user_prefs.to_string("Language");
            let old_speech_style = self.user_prefs.to_string("SpeechStyle");
            let old_braille_code = self.user_prefs.to_string("BrailleCode");
            match self.initialize(self.rules_dir.clone().unwrap()) {
                Err(e) => error!("Failed to reread prefs.yaml: {}", e),  // probably in big trouble, but continue on and maybe ok
                Ok(_) => {
                    if old_speech_style != self.user_prefs.to_string("SpeechStyle") {
                        files_changed.speech_rules = true;
                    }
                    if old_lang != self.user_prefs.to_string("Language") {
                        files_changed.speech_rules = true;
                        files_changed.speech_unicode_short = true;
                        files_changed.speech_unicode_full = true;
                    }
                    if old_braille_code != self.user_prefs.to_string("BrailleCode") {
                        files_changed.braille_rules = true;
                        files_changed.braille_unicode_short = true;
                        files_changed.braille_unicode_full = true;
                    }
                }
            } 
        }
        if files_changed.speech_rules ||
           files_changed.speech_unicode_short ||
           files_changed.speech_unicode_full ||
           files_changed.braille_rules ||
           files_changed.braille_unicode_short ||
           files_changed.braille_unicode_full ||
           files_changed.intent ||
           files_changed.defs {
            return Some(files_changed);
        } else {
            return None;
        }
    }

    fn is_file_up_to_date(ft: &FileAndTime) -> bool {
        if ft.time.is_none() {
            // wasn't able to determine a time -- just claim it is up to date
            return true;
        }
        let time = ft.time.unwrap();
        return  is_older(&ft.files[0], time) &&
                is_older(&ft.files[1], time) &&
                is_older(&ft.files[2], time);

        fn is_older(path: &Option<PathBuf>, time: SystemTime) -> bool {
            // if let Some(path_buf) = path {
            //     debug!("p={}", path_buf.to_str().unwrap());
            //     debug!("p.metadata={:?}", path_buf.metadata());
            //     debug!("p.metadata.modified={:?}", path_buf.metadata().unwrap().modified());
            //     debug!("p.metadata.modified.duration_since={:?}", path_buf.metadata().unwrap().modified().unwrap().duration_since(time));    
            // }
            return match path {
                Some(p) => {
                    let file_mod_time = p.metadata().unwrap().modified().unwrap();
                    return file_mod_time <= time;
                },
                None => true,
            }           
        }
    }

    /// Return the speech rule style file locations.
    pub fn get_rule_file(&self, name: &RulesFor) -> &Locations {
        if !self.error.is_empty() {
            panic!("Internal error: get_rule_file called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        return match name {
            RulesFor::Intent => &self.intent.files,
            RulesFor::Speech => &self.speech.files,
            RulesFor::OverView => &self.overview.files,
            RulesFor::Navigation => &self.navigation.files,
            RulesFor::Braille => &self.braille.files,
        };
    }

    /// Return the unicode.yaml file locations.
    pub fn get_speech_unicode_file(&self) ->(PathBuf, PathBuf) {
        if !self.error.is_empty() {
            panic!("Internal error: get_speech_unicode_file called on invalid PreferenceManager -- error message\n{}", &self.error);
        };
        return (self.speech_unicode.files[0].as_ref().unwrap().to_path_buf(), self.speech_unicode_full.files[0].as_ref().unwrap().to_path_buf());
    }

    /// Return the speech rule style file locations.
    pub fn get_braille_file(&self) -> &Locations {
        if !self.error.is_empty() {
            panic!("Internal error: get_braille_file called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        return &self.braille.files;
    }

    /// Return the unicode.yaml file locations.
    pub fn get_braille_unicode_file(&self) -> (PathBuf, PathBuf) {
        if !self.error.is_empty() {
            panic!("Internal error: get_braille_unicode_file called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        return (self.braille_unicode.files[0].as_ref().unwrap().to_path_buf(), self.braille_unicode_full.files[0].as_ref().unwrap().to_path_buf());
    }

    /// Return the definitions.yaml file locations.
    pub fn get_definitions_file(&self) -> &Locations {
        if !self.error.is_empty() {
            panic!("Internal error: get_definitions_file called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        return &self.defs.files;
    }

    /// Return the TTS engine currently in use.
    pub fn get_tts(&self) -> TTS {
        if !self.error.is_empty() {
            panic!("Internal error: get_tts called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        return match self.api_prefs.to_string("TTS").as_str().to_ascii_lowercase().as_str() {
            "none" => TTS::None,
            "ssml" => TTS::SSML,
            "sapi5" => TTS::SAPI5,
            _ => {
                warn!("found unknown value for TTS: '{}'", self.api_prefs.to_string("TTS").as_str());
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

        return match &self.api_prefs.to_string("Rate").parse::<f64>() {
            Ok(val) => *val,
            Err(_) => {
                warn!("Rate ('{}') can't be converted to a floating point number", &self.api_prefs.to_string("Rate"));
                DEFAULT_API_PREFERENCES.with(|defaults| defaults.prefs["Rate"].as_f64().unwrap())
            }
        };
    }

    /// Return the current language. The will be the most specific version (e.g, "en-gb")
    pub fn get_language(&self) -> String {
        return self.user_prefs.to_string("Language");
    }

    pub fn get_api_prefs(&self) -> &Preferences {
        return &self.api_prefs;
    }

    pub fn get_user_prefs(&self) -> &Preferences {
        return &self.user_prefs;
    }

    // occasionally useful to check a pref value when debugging
    // fn get_pref(&self, pref_name: &str) -> String {
    //     return yaml_to_string(self.user_prefs.prefs.get(pref_name).unwrap(), 1);
    // }

    pub fn set_user_prefs(&mut self, name: &str, value: &str) -> Option<FilesChanged> {
        if !self.error.is_empty() {
            panic!("Internal error: set_user_prefs called on invalid PreferenceManager -- error message\n{}", &self.error);
        };

        self.user_prefs.set_string_value(name, value);
        if name == "Language" || name == "SpeechStyle" || name == "BrailleCode" {
            let old_speech = self.speech.clone();
            let old_speech_unicode= self.speech_unicode.clone();
            let old_speech_unicode_full = self.speech_unicode_full.clone();
            let old_braille = self.braille.clone();
            let old_braille_unicode= self.braille_unicode.clone();
            let old_braille_unicode_full = self.braille_unicode_full.clone();
            let old_intent= self.intent.clone();
            let old_defs= self.defs.clone();

            if let Some(rules_dir) = self.rules_dir.clone() {
                self.set_all_files(&rules_dir, self.user_prefs.clone(), self.pref_files.clone()).unwrap();
                let changed = FilesChanged {
                    speech_rules: old_speech != self.speech,
                    speech_unicode_short: old_speech_unicode != self.speech_unicode,
                    speech_unicode_full: old_speech_unicode_full != self.speech_unicode_full,
                    braille_rules: old_braille != self.braille,
                    braille_unicode_short: old_braille_unicode != self.braille_unicode,
                    braille_unicode_full: old_braille_unicode_full != self.braille_unicode_full,
                    intent: old_intent != self.intent,
                    defs: old_defs != self.defs,
                };
                return Some(changed);
            }
        }
        return None;
    }
}


#[cfg(test)]
mod tests {

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
    fn rel_path(rules_dir: &Option<PathBuf>, path: &Option<PathBuf>) -> PathBuf {
        match rules_dir {
            None => panic!("Rules dir not found"),
            Some(rules_dir) => {
                match path {
                    None => panic!("No path found"),
                    Some(path) => {
                        let stripped_path = path.strip_prefix(rules_dir).unwrap();
                        return stripped_path.to_path_buf();        
                    }
                }
            }
        }
    }

    #[test]
    fn find_simple_style() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            assert_eq!(rel_path(&pref_manager.rules_dir, &pref_manager.speech.files[0]), PathBuf::from("Languages/en/ClearSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn find_style_other_language() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "zz");
            assert_eq!(rel_path(&pref_manager.rules_dir, &pref_manager.speech.files[0]), PathBuf::from("Languages/zz/ClearSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn find_unicode_files() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "zz-aa");
            
            assert_eq!(rel_path(&pref_manager.rules_dir, &pref_manager.speech.files[0]), PathBuf::from("Languages/zz/ClearSpeak_Rules.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, &pref_manager.speech.files[1]), PathBuf::from("Languages/zz/aa/ClearSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn find_style_no_sublanguage() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "zz-ab");
            
            assert_eq!(rel_path(&pref_manager.rules_dir, &pref_manager.speech.files[0]), PathBuf::from("Languages/zz/ClearSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn found_all_files() {
        fn count_files(ft: &FileAndTime) -> usize {
            return ft.files.iter().filter(|path| path.is_some()).count();
        }
        fn assert_helper(count: usize, correct: usize, file_name: &str) {
            assert_eq!(count, correct, "In looking for '{}', found {} files, should have found {}", 
                    file_name, count, correct);
        }

        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "zz-aa");
            pref_manager.set_user_prefs("BrailleCode", "UEB");
            
            assert_helper(count_files(&pref_manager.speech), 2, "ClearSpeak_Rules.yaml");
            assert_helper(count_files(&pref_manager.speech_unicode), 1, "unicode.yaml");
            assert_helper(count_files(&pref_manager.braille), 1, "Nemeth_Rules.yaml");
            assert_helper(count_files(&pref_manager.braille_unicode), 1, "unicode.yaml");
            assert_helper(count_files(&pref_manager.intent), 1, "intent.yaml");
            assert_helper(count_files(&pref_manager.defs), 3,"definitions.yaml");
    
            pref_manager.set_user_prefs("Language", "zz-ab");
            assert_helper(count_files(&pref_manager.speech), 1, "ClearSpeak_Rules.yaml");
            assert_helper(count_files(&pref_manager.speech_unicode), 1, "unicode.yaml");
            assert_helper(count_files(&pref_manager.braille), 1, "Nemeth_Rules.yaml");
            assert_helper(count_files(&pref_manager.braille_unicode), 1, "unicode.yaml");
            assert_helper(count_files(&pref_manager.intent), 1, "intent.yaml");
            assert_helper(count_files(&pref_manager.defs), 2, "definitions.yaml");
        })
    }

    #[test]
    fn file_found_order() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "zz-aa");

            let mut iter = pref_manager.defs.files.iter();
            assert_eq!(rel_path(&pref_manager.rules_dir, iter.next().unwrap()), Path::new("definitions.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, iter.next().unwrap()), Path::new("Languages/zz/definitions.yaml"));
            assert_eq!(rel_path(&pref_manager.rules_dir, iter.next().unwrap()), Path::new("Languages/zz/aa/definitions.yaml"));
            assert_eq!(iter.next(), None, "Should not be any files left")
        });
    }

    #[test]
    fn test_prefs() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            let prefs = pref_manager.get_user_prefs();
            assert_eq!(prefs.to_string("Language").as_str(), "en");
            assert_eq!(prefs.to_string("SubjectArea").as_str(), "General");
            assert_eq!(prefs.to_string("ClearSpeak_AbsoluteValue").as_str(), "Auto");
            assert_eq!(prefs.to_string("ResetNavMode").as_str(), "false");
            assert_eq!(prefs.to_string("BrailleCode").as_str(), "Nemeth");
            assert_eq!(prefs.to_string("X_Y_Z").as_str(), NO_PREFERENCE);
        });
    }

    #[test]
    fn test_language_change() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("SpeechStyle", "ClearSpeak");

            assert_eq!(rel_path(&pref_manager.rules_dir, &pref_manager.get_rule_file(&RulesFor::Speech)[0]), PathBuf::from("Languages/en/ClearSpeak_Rules.yaml"));

            pref_manager.set_user_prefs("Language", "zz");
            
            assert_eq!(rel_path(&pref_manager.rules_dir, &pref_manager.get_rule_file(&RulesFor::Speech)[0]), PathBuf::from("Languages/zz/ClearSpeak_Rules.yaml"));
        });
    }

    #[test]
    fn test_some_changes() {
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Verbosity", "Terse");

            assert_eq!(&pref_manager.get_user_prefs().to_string("Verbosity"), "Terse");

            pref_manager.set_user_prefs("BrailleCode", "UEB");
            
            assert_eq!(rel_path(&pref_manager.rules_dir, &pref_manager.get_rule_file(&RulesFor::Braille)[0]), PathBuf::from("Braille/UEB/UEB_Rules.yaml"));
        });
    }

    use std::fs;
    #[test]
    fn test_up_to_date() {
        use std::thread::sleep;
        use std::time::Duration;
        PREF_MANAGER.with(|pref_manager| {
            let mut pref_manager = pref_manager.borrow_mut();
            pref_manager.initialize(abs_rules_dir_path()).unwrap();
            pref_manager.set_user_prefs("Language", "zz-aa");
            
            // First test to make sure the up_to_date check works -- need to do in this test since the order of testing is random
            let files_changed = pref_manager.is_up_to_date();        
            assert!(files_changed.is_none(), "files_changed={}", files_changed.unwrap());


            // Note: need to use pattern match to avoid borrow problem
            // Don't change a speech related file because 'test_is_up_to_date' might fail 
            if let Some(file_name) = &pref_manager.get_definitions_file()[0] {
                let file_name_as_str = file_name.to_str().unwrap();
                let contents = fs::read(file_name).expect(&format!("Failed to write file {} during test", file_name_as_str));
                #[allow(unused_must_use)] { 
                    fs::write(file_name, contents);
                    sleep(Duration::from_millis(10));
                }
                let files_changed = pref_manager.is_up_to_date();
                assert!(files_changed.is_some());
                let files_changed = files_changed.unwrap();
                assert!(&files_changed.defs);
                assert!(!&files_changed.speech_rules);
                assert!(!&files_changed.speech_unicode_short);
            } else {
                panic!("First path is 'None'");
            }

            // open the file, read all the contents, then write them back so the time changes
        });
    }
}
