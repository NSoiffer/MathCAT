//! #Speech Engine Information
//!
//! ## Pitch (default 140hz)
//! ### SAPI4: Relative pitch
//! * Number is relative to the default/current pitch.
//! * 50 is 1/2 of the default/current pitch, 200 is 2 times the default/current pitch.
//!
//!  Note: no range is specified by the spec
//! ### SAPI5: Relative pitch
//! * Number is in range -24 to 24
//! * -24 is one octave below the default/current pitch, +24 is one octave above
//! * changes are logarithmic -- a change of +/-1 corresponds to multiplying/dividing by 24th root of 2
//! ### SSML: Relative pitch
//! * pitch in hertz (default/current man's voice is about 100hz, woman's 180hz)
//!
//! Note: other legal values for SSML are not supported, and all numbers are interpreted as relative changes
//! ### Eloquence: Absolute pitch (relative pitch not supported by Eloquence)
//! * Range is 0 - 100.  Guess is that 0 ~= 42hz, 100 ~= 422hz based on supported \"sapi\" values
//! ## Rate (default 180 words/min)
//! ### SAPI4: Absolute rate
//! * Number is relative to the default/current rate
//! * 50 is 1/2 of the default/current rate, 200 is 2 times the default/current rate
//!
//! Note: no range is specified by the spec
//! ### SAPI5: Relative rate
//! * Number is in range -10 to 10
//! * -10 is 1/3 of the default/current speed; 10 3 times the default/current speech
//! * changes are logarithmic -- a change of +/-1 corresponds to multiplying/dividing by 10th root of 3
//! ### SSML: Relative rate
//! * 0.5 is 1/2 of the default/current rate, 2.0 is 2 times the default/current rate
//!
//!  Note:  other legal values for SSML are not supported, and all numbers are interpreted as relative changes
//! ### Eloquence: Absolute rate (relative rate not supported by Eloquence)
//! * Range is 0 - 250, which manual seems to indicate corresponds to 70 - 1297 words/min.
//! * * Window-Eyes only seems to give values in range 1 - 150.
//! * On the low end, 1 ~= 72words/min
//! * On the high end, I can't tell, but 80 seems to be a bit over twice normal (~400 words/min?)
//!    250 ~= 1297 words/min based on supported "sapi" values
//!
//!  Note: this means words/min = 4.18 * Eloquence rate + 66
//!  So the relative pause rate is 180/computed value
//!
//!
//! ## Volume (default 100 \[full])
//! ### SAPI4: Relative volume
//! * Number is relative to the default/current rate
//! * Range is 0 - 065535
//! ### SAPI5: Relative volume
//! * Number is in range 0 to 100
//! ### SSML: Relative volume
//! * Number is in range 0 to 100
//!
//! Note:  other legal values for SSML are not supported, and all numbers are interpreted as relative changes
//! ### Eloquence: Absolute volume (relative volume not supported by Eloquence)
//! * Range is 0 - 100
//!
//! ## Pause
//! * All systems -- pauses are given in milliseconds
//!
//! Note: Pauses on output are scaled based on the ratio of the current rate to the default rate (180 wpm)
#![allow(clippy::needless_return)]

use crate::{errors::*, prefs::PreferenceManager, speech::ReplacementArray};
use sxd_document::dom::Element;
use yaml_rust::Yaml;

use std::{fmt};
use crate::speech::{SpeechRulesWithContext};
use strum_macros::IntoStaticStr;
use regex::Regex;

pub const PAUSE_SHORT:f64 = 150.0;  // ms
pub const PAUSE_MEDIUM:f64 = 300.0; // ms
pub const PAUSE_LONG:f64 = 600.0;   // ms
pub const PAUSE_AUTO:f64 = 987654321.5;   // ms -- hopefully unique
pub const PAUSE_AUTO_STR: &str = "\u{F8FA}\u{F8FA}";

/// TTSCommand are the supported TTS commands
/// When parsing the YAML rule files, they are converted to these enums
#[derive(Debug, Clone, PartialEq, IntoStaticStr)]
pub enum TTSCommand {
    Pause,
    Rate,
    Volume,
    Pitch ,
    Gender,
    Voice,
    Spell,
    Bookmark,
}

/// TTSCommands are either numbers (f64 because of YAML) or strings
#[derive(Debug, Clone)]
pub enum TTSCommandValue {
    Number(f64),
    String(String)
}

impl TTSCommandValue {
    fn get_num(&self) -> f64 {
        match self {
            TTSCommandValue::Number(n) => return *n,
            _                               => panic!("Internal error: TTSCommandValue is not a number"),
        }
    }

    fn get_string(&self) -> &String {
        match self {
            TTSCommandValue::String(s) => return &s,
            _                               => panic!("Internal error: TTSCommandValue is not a string"),
        }
    }
}

/// A TTS rule consists of the command, the value, and its replacement
#[derive(Debug, Clone)]
pub struct TTSCommandRule {
    command: TTSCommand,
    value: TTSCommandValue,
    replacements: ReplacementArray
}

impl fmt::Display for TTSCommandRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command: &'static str = "rate";
        let value = match &self.value {
            TTSCommandValue::String(s) => s.to_string(),
            TTSCommandValue::Number(f) => f.to_string()
        };
        if self.command == TTSCommand::Pause {
            return write!(f, "pause: {{value: {}}}", value);
        } else {
            return write!(f, "{}:\n  value: {}{}", 
                    command, value, self.replacements);
        };
    }
}


impl TTSCommandRule {
    pub fn new(command: TTSCommand, value: TTSCommandValue, replacements: ReplacementArray) -> TTSCommandRule {
        return TTSCommandRule{
            command,
            value,
            replacements
        }
    }
}

/// Supported TTS engines
/// These types should do something for all the TTSCommands
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TTS {
    None,
    SAPI5,
    SSML,
//    Mac,
//    Eloquence
}

impl TTS {
    /// Given the tts command ("pause", "rate", etc) and its value, build the TTS data structure for it.
    ///
    /// `tts_command`: one of "pause", "rate", etc
    ///
    /// `value`: keyword 'value' or dict with 'value' and 'replace' (optional) keys
    pub fn build(tts_command: &str, values: &Yaml) -> Result<Box<TTSCommandRule>> {
        use crate::pretty_print::yaml_to_string;
        let hashmap = values.as_hash();
        let tts_value;
        let replacements;
        if hashmap.is_some() {
            tts_value = &values["value"];
            if tts_value.is_badvalue() {
                bail!("{} TTS command is missing a 'value' sub-key. Found\n{}", tts_command, yaml_to_string(values, 1));
            };
            replacements = ReplacementArray::build(&values["replace"])?;
        }else {
            tts_value = values;
            replacements = ReplacementArray::build_empty();
        }
        let tts_value = yaml_to_string(&tts_value, 0);
        let tts_value = tts_value.trim(); // if not a number or string, value is bogus
        let tts_enum = match tts_command {
            "pause"    => TTSCommand::Pause,
            "rate"     => TTSCommand::Rate,
            "volume"   => TTSCommand::Volume,
            "pitch"    => TTSCommand::Pitch,
            "gender"   => TTSCommand::Gender,
            "voice"    => TTSCommand::Voice,
            "spell"    => TTSCommand::Spell,
            "bookmark" => TTSCommand::Bookmark,
            _          => panic!("Internal error in build_tts: unexpected rule ({:?}) encountered", tts_command)    
        };
    
        let tts_command_value;
        if !(tts_enum == TTSCommand::Gender || tts_enum == TTSCommand::Voice ||
             tts_enum == TTSCommand::Spell || tts_enum == TTSCommand::Bookmark) {
            let val = match tts_value {
                "short" => Ok( PAUSE_SHORT ),
                "medium" => Ok( PAUSE_MEDIUM ),
                "long" => Ok( PAUSE_LONG ),
                "auto" => Ok( PAUSE_AUTO ),
                _ => tts_value.parse::<f64>()
            };
        
            if val.is_err() {
                bail!(format!("\"{}: {}\" is not a number", &tts_command, tts_value));    
            }
    
            tts_command_value = TTSCommandValue::Number(val.unwrap());
        } else {
            tts_command_value = TTSCommandValue::String(tts_value.to_string());
    
        }
        return Ok( Box::new( TTSCommandRule::new(tts_enum, tts_command_value, replacements) ) );
    }
    
    /// The rule called to execute the TTSCommand `command`
    /// * `prefs` are used for scaling the speech rate
    /// * some rules have MathML nested inside, so we need to do replacements on them (hence `rules` and `mathml` are needed)
    ///
    /// A string is returned for the speech engine.
    ///
    /// `auto` pausing is handled at a later phase and a special char is used for it
    pub fn replace<'c, 's:'c, 'r>(&self, command: &TTSCommandRule, prefs: &PreferenceManager, rules_with_context: &'r mut SpeechRulesWithContext<'c, 's>, mathml: &'r Element<'c>) -> Result<String> {
        // The general idea is we handle the begin tag, the contents, and then the end tag
        // For the begin/end tag, we dispatch off to specialized code for each TTS engine
        let mut command = command.clone();
        if command.command == TTSCommand::Bookmark && command.value.get_string() == "auto" {
            match mathml.attribute_value("id") {
                None => return Ok("".to_string()),
                Some(id_val) => command.value = TTSCommandValue::String(id_val.to_string()),
            };
        }
        let mut result = String::with_capacity(255);
        result += &match self {
            TTS::None  => self.get_string_none(&command, prefs, true),
            TTS::SAPI5 => self.get_string_sapi5(&command, prefs, true),
            TTS::SSML  => self.get_string_ssml(&command, prefs, true),
        };


        if !command.replacements.is_empty()  {
            if result.is_empty() {
                result += " ";
            }
            result += &command.replacements.replace(rules_with_context, mathml)?;    
        }

        let end_tag = match self {
            TTS::None  => self.get_string_none(&command, prefs, false),
            TTS::SAPI5 => self.get_string_sapi5(&command, prefs, false),
            TTS::SSML  => self.get_string_ssml(&command, prefs, false),
        };

        if end_tag.is_empty() {
            return Ok( result ); // avoids adding in " "
        } else {
            return Ok( result + &end_tag );
        }
    }

    // auto pausing can't be known until neighboring strings are computed
    // we create a unique string in this case and compute the real value later 
    fn get_string_none(&self, command: &TTSCommandRule,  _prefs: &PreferenceManager, is_start_tag: bool) -> String  {
        // they only thing to do is handle "pause" with some punctuation hacks along with 'spell'        
        if is_start_tag {
            if command.command == TTSCommand::Pause {
                let amount = command.value.get_num();
                // only ',' and ';' are used as '.' didn't seem to reliably generate pauses in tests
                return crate::speech::CONCAT_INDICATOR.to_string() + (
                    if amount == PAUSE_AUTO {
                        PAUSE_AUTO_STR
                    } else if amount <= 250.0 {
                        ","
                    } else  {
                        ";"
                    }
                );
            } else if command.command == TTSCommand::Spell {
                // add a space between the chars
                let mut chars = command.value.get_string().chars();
                let first_char = chars.next();
                if first_char.is_none() {
                    return "".to_string();
                }
            
                let mut chars_with_spaces = vec![first_char.unwrap()];
                for ch in chars {
                    chars_with_spaces.push(' ');
                    chars_with_spaces.push(ch);
                }
                return chars_with_spaces.into_iter().collect();
            }
        };
        return "".to_string();
    }
    
    fn get_string_sapi5(&self, command: &TTSCommandRule, prefs: &PreferenceManager, is_start_tag: bool) -> String  {
        return match &command.command {
            TTSCommand::Pause => if is_start_tag {
                let amount = command.value.get_num();
                if amount == PAUSE_AUTO {
                    PAUSE_AUTO_STR.to_string()
                } else {
                    format!("<silence msec=='{}ms'/>", amount * 180.0/prefs.get_rate())
                }
            } else {
                "".to_string()
            },
            TTSCommand::Pitch => if is_start_tag {format!("<pitch middle='XXX{}%'>", command.value.get_num())} else {String::from("</prosody>")},
            TTSCommand::Rate =>  if is_start_tag {format!("<rate speed='XXX{}%'>", command.value.get_num())} else {String::from("</rate>")},
            TTSCommand::Volume =>if is_start_tag {format!("<volume level='{}'>", command.value.get_num())} else {String::from("</volume>")},
            TTSCommand::Gender =>if is_start_tag {format!("<prosody gender='XXX{}%'>", command.value.get_string())} else {String::from("</prosody>")},
            TTSCommand::Voice =>if is_start_tag {format!("<prosody voice='XXX{}%'>", command.value.get_string())} else {String::from("</prosody>")},
            TTSCommand::Spell =>if is_start_tag {format!("<spell>{}", command.value.get_string())} else {String::from("</spell>")},
            TTSCommand::Bookmark =>if is_start_tag {format!("<bookmark mark='{}'/>", command.value.get_string())} else {"".to_string()},
        };
    }

    fn get_string_ssml(&self, command: &TTSCommandRule, prefs: &PreferenceManager, is_start_tag: bool) -> String  {
        return match &command.command {
            TTSCommand::Pause => if is_start_tag {
                let amount = command.value.get_num();
                if amount == PAUSE_AUTO {
                    PAUSE_AUTO_STR.to_string()
                } else {
                    format!("<break time='{}ms'/>", amount * 180.0/prefs.get_rate())
                }
            } else {
                "".to_string()
            },
            TTSCommand::Pitch => if is_start_tag {format!("<prosody pitch='{}%'>", command.value.get_num())} else {String::from("</prosody>")},
            TTSCommand::Rate =>  if is_start_tag {format!("<prosody rate='{}%'>", command.value.get_num())} else {String::from("</prosody>")},
            TTSCommand::Volume =>if is_start_tag {format!("<prosody volume='{}%'>", command.value.get_num())} else {String::from("</prosody>")},
            TTSCommand::Gender =>if is_start_tag {format!("<voice required='gender=\"{}\"'>", command.value.get_string())} else {String::from("</voice>")},
            TTSCommand::Voice =>if is_start_tag {format!("<voice required='{}'>", command.value.get_string())} else {String::from("</voice>")},
            TTSCommand::Spell =>if is_start_tag {format!("<say-as interpret-as='characters'>{}", command.value.get_string())} else {String::from("</say-as>")},
            TTSCommand::Bookmark =>if is_start_tag {format!("<mark name='{}'/>", command.value.get_string())} else {"".to_string()},
        }
    }

    /// Compute the length of the pause to use.
    ///
    /// The computation is based on the length of the speech strings (after removing tagging).
    /// There is a bias towards pausing more _after_ longer strings.
    pub fn compute_auto_pause(&self, prefs: &PreferenceManager, before: &str, after: &str) -> String {
        lazy_static! {
            static ref REMOVE_XML: Regex = Regex::new(r"<.+?>").unwrap();    // punctuation ending with a '.'
        }
        let before_len;
        let after_len;
        match self {
            TTS::SSML | TTS::SAPI5 => {
                before_len = REMOVE_XML.replace_all(before, "").len();
                after_len = REMOVE_XML.replace_all(after, "").len();
            },
            _ => {
                before_len = before.len();
                after_len = after.len();
            },
        }

        // pause values are not cut in stone
        // the calculation bias to 'previous' is based on MathPlayer which used '30 * #-of-descendants-on-left
        // I think I did this as a sort of "take a breath" after saying something long although one might want to do that
        //   before speaking something long.
        if after_len < 3 {
            // hack to prevent pausing before "of" in exprs like "the fourth power of secant, of x"
            // if it should pause anywhere, it should be after the "of"
            return "".to_string(); 
        }
        let pause = std::cmp::min(3000, ((2 * before_len + after_len)/48) * 128);
        if pause <= 50 {
            // don't put out a lot of short pauses which probably can't be heard
            return "".to_string();
        }
        // create a TTSCommandRule so we reuse code
        let command = TTSCommandRule::new(
            TTSCommand::Pause,
            TTSCommandValue::Number(pause as f64),
            ReplacementArray::build_empty(),
        );
        return match self {
            TTS::None  => self.get_string_none(&command, prefs, true),
            TTS::SAPI5 => self.get_string_sapi5(&command, prefs, true),
            TTS::SSML  => self.get_string_ssml(&command, prefs, true),
        };

    }

    /// Take the longest of the pauses
    ///
    /// Two other options are:
    /// 1. average the pauses
    /// 2. add the pauses together.
    ///
    /// Until evidence points otherwise, use 'longest'.
    pub fn merge_pauses(&self, str: &str) -> String {
        // we need specialized merges for each TTS engine because we need to know the format of the commands
        return match self {
            TTS::None  => self.merge_pauses_none(str),
            TTS::SAPI5 => self.merge_pauses_sapi5(str),
            TTS::SSML  => self.merge_pauses_ssml(str),
        };        
    }

    fn merge_pauses_none(&self, str: &str) -> String {
        // punctuation used for pauses is ",", ";" 
        lazy_static! {
            static ref MULTIPLE_PAUSES: Regex = Regex::new(r"[,;][,;]+").unwrap();   // two or more pauses
        }
        // we reduce all sequences of two or more pauses to a single medium pause
        let mut merges_string = str.to_string();
        for cap in MULTIPLE_PAUSES.captures_iter(str) {
            merges_string = merges_string.replace(&cap[0], ";");
        }
        return merges_string;
    }

    fn merge_pauses_xml<F>(str: &str, full_attr_re: &Regex, sub_attr_re: &Regex, replace_with: F) -> String 
            where F: Fn(usize) -> String {
        // we reduce all sequences of two or more pauses to the max pause amount
        // other options would be the sum or an average
        // maybe some amount a little longer than the max would be best???
        let mut merges_string = str.to_string();
        for cap in full_attr_re.captures_iter(str) {
            let mut amount = 0;
            for c in sub_attr_re.captures_iter(&cap[0]) {
                amount = std::cmp::max(amount, c[1].parse::<usize>().unwrap());
            };
            merges_string = merges_string.replace(&cap[0], &replace_with(amount));
        }
        return merges_string;
    }

    fn merge_pauses_sapi5(&self, str: &str) -> String {
        lazy_static! {
            static ref CONSECUTIVE_BREAKS: Regex = Regex::new(r"(<silence msec[^>]+?> *){2,}").unwrap();   // two or more pauses
            static ref PAUSE_AMOUNT: Regex = Regex::new(r"msec=.*?(\d+)").unwrap();   // amount after 'time'
        }
        let replacement = |amount: usize| format!("<silence msec=='{}ms'/>", amount);
        return TTS::merge_pauses_xml(str, &CONSECUTIVE_BREAKS, &PAUSE_AMOUNT, replacement);
    }

    fn merge_pauses_ssml(&self, str: &str) -> String {
        lazy_static! {
            static ref CONSECUTIVE_BREAKS: Regex = Regex::new(r"(<break time=[^>]+?> *){2,}").unwrap();   // two or more pauses
            static ref PAUSE_AMOUNT: Regex = Regex::new(r"time=.*?(\d+)").unwrap();   // amount after 'time'
        }
        let replacement = |amount: usize| format!("<break time='{}ms'/>", amount);
        return TTS::merge_pauses_xml(str, &CONSECUTIVE_BREAKS, &PAUSE_AMOUNT, replacement);
    }
}