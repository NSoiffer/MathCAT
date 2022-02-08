#![allow(clippy::needless_return)]
use sxd_document::dom::Element;
use sxd_document::Package;
use crate::errors::*;
use regex::{Captures, Regex};
use phf::{phf_map, phf_set};
use crate::speech::{BRAILLE_RULES, SpeechRulesWithContext};



/// braille the MathML
/// If 'nav_node_id' is not an empty string, then the element with that id will have dots 7 & 8 turned on as per the pref
pub fn braille_mathml(mathml: Element, nav_node_id: String) -> Result<String> {
    return BRAILLE_RULES.with(|rules| {
        {
            let mut mut_rules = rules.borrow_mut();
            mut_rules.update()?;    
        }
        let rules = rules.borrow();
        let new_package = Package::new();
        let mut rules_with_context = SpeechRulesWithContext::new(&rules, new_package.as_document(), nav_node_id);
        let speech_string = rules_with_context.match_pattern::<String>(mathml)
                        .chain_err(|| "Pattern match/replacement failure!")?;
            // FIX: need to set name of speech rules so test Nemeth/UEB clean for
        let pref_manager = rules_with_context.get_rules().pref_manager.borrow();
        let highlight_style = pref_manager.get_user_prefs().to_string("BrailleNavHighlight");
        let braille = if &pref_manager.get_user_prefs().to_string("Code") == "UEB" {
            ueb_cleanup(speech_string.replace(" ", ""))
        } else {
            nemeth_cleanup(speech_string.replace(" ", ""))
        };

        return Ok(
            if highlight_style != "Off" {
                highlight_braille_chars(braille, highlight_style == "All")
            } else {
             braille
            }
        );
    });

    // highlight with dots 7 & 8 based on the highlight style
    // both the start and stop points will be extended to deal with indicators such as capitalization
    // if 'fill_range' is true, the interior will be highlighted
    fn highlight_braille_chars(braille: String, fill_range: bool) -> String {
        let mut braille = braille;
        // some special chars weren't converted to having dots 7 & 8 to indicate navigation position -- add them

        // find start and end indexes
        let start = braille.find(|ch| is_highlighted(ch));
        let end = braille.rfind(|ch| is_highlighted(ch));
        if start.is_none() {
            assert!(end.is_none());
            return braille;
        };

        let start = start.unwrap();
        let end = end.unwrap();

        highlight_first_indicator(&mut braille, start);
        if start == end {
            return braille;
        }
        if !fill_range {
            return braille;
        }

        let mut result = String::with_capacity(braille.len());
        result.push_str(&braille[..start]);
        let highlight_region =&mut braille[start..end];
        for ch in highlight_region.chars() {
            result.push( highlight(ch) );
        };
        result.push_str(&braille[end..]);
        return result;

        fn is_highlighted(ch: char) -> bool {
            let ch_as_u32 = ch as u32;
            return 0x28C0 <= ch_as_u32 && ch_as_u32 <= 0x28FF;
        }

        fn highlight(ch: char) -> char {
            return unsafe{char::from_u32_unchecked(ch as u32 | 0xC0)};      
        }

        fn unhighlight(ch: char) -> char {
            return unsafe{char::from_u32_unchecked(ch as u32 & 0x283F)};      
        }

        fn highlight_first_indicator(braille: &mut String, ch_index: usize) {
            // need to highlight (optional) capital/number, language, and style also in that (rev) order
            // chars in the braille block range use 3 bytes
            let mut n_bytes = ch_index;     // how far to move back
            let prefix_ch_index = std::cmp::max(0, ch_index as isize - 12) as usize;
            let indicators = &braille[prefix_ch_index..ch_index];   // chars to be examined
            let prefix = &mut indicators.chars().rev().peekable();
            if prefix.peek() == Some(&&'⠠') { // cap indicator
                n_bytes -= 3;
                prefix.next();
            } else if prefix.peek() == Some(&&'⠼') { // number indicator
                n_bytes -= 3;
                prefix.next();
            } 
            if [Some(&'⠸'), Some(&'⠈'), Some(&'⠨')].contains(&prefix.peek()) { // bold, script/blackboard, italic indicator
                n_bytes -= 3;
                prefix.next();
            }

            if [Some(&'⠰'), Some(&'⠸'), Some(&'⠨')].contains(&prefix.peek()) {   // English, German, Greek
                n_bytes -= 3;
            } else if prefix.peek() == Some(&&'⠈') {  
                let ch = prefix.next();                              // Russian/Greek Variant
                if ch == Some('⠈') || ch == Some('⠨') {
                    n_bytes -= 6;
                }
            } else if prefix.peek() == Some(&&'⠠')  { // Hebrew 
                let ch = prefix.next();                              // Russian/Greek Variant
                if ch == Some('⠠') {
                    n_bytes -= 6;
                }
            };
            if n_bytes < ch_index {
                // remove old highlight
                let replacement_range = ch_index..ch_index+3;
                let replacement_str = unhighlight(braille[replacement_range.clone()].chars().next().unwrap()).to_string();
                braille.replace_range(replacement_range, &replacement_str);

                // add new highlight
                let replacement_range = n_bytes..n_bytes+3;
                let replacement_str = highlight(braille[replacement_range.clone()].chars().next().unwrap()).to_string();
                braille.replace_range(replacement_range, &replacement_str);
            }
        }

    }
}



fn nemeth_cleanup(raw_braille: String) -> String {
    // Typeface: S: sans-serif, B: bold, T: script/blackboard, I: italic, R: Roman
    // Language: E: English, D: German, G: Greek, V: Greek variants, H: Hebrew, U: Russian
    // Indicators: C: capital, N: number, P: punctuation, M: multipurpose
    // Others:
    //      W -- whitespace that should be kept (e.g, in a numeral)
    //      𝑁 -- hack for special case of a lone decimal pt -- not considered a number but follows rules mostly 
    // SRE doesn't have H: Hebrew or U: Russian, so not encoded (yet)
    // Note: some "positive" patterns find cases to keep the char and transform them to the lower case version
    static NEMETH_INDICATOR_REPLACEMENTS: phf::Map<&str, &str> = phf_map! {
        "S" => "⠈⠰",    // sans-serif
        "B" => "⠸",     // bold
        "T" => "⠈",     // script/blackboard
        "I" => "⠨",     // italic
        "R" => "",      // roman
        "E" => "⠰",     // English
        "D" => "⠸",     // German (Deutsche)
        "G" => "⠨",     // Greek
        "V" => "⠨⠈",    // Greek Variants
        "H" => "⠠⠠",    // Hebrew
        "U" => "⠈⠈",    // Russian
        "C" => "⠠",     // capital
        "P" => "⠸",     // punctuation
        "L" => "",      // letter
        "M" => "",      // multipurpose indicator
        "m" => "⠐",     // required multipurpose indicator
        "N" => "",       // digit
        "n" => "⠼",     // required number indicator
        "𝑁" => "",      // long "." treated as a digit
        "W" => "⠀",     // whitespace
        "," => "⠠⠀",     // comma
        "b" => "⠐",     // baseline
        "↑" => "⠘",     // superscript
        "↓" => "⠰",     // supscript
    };

    lazy_static! {
        // Trim braille spaces before and after braille indicators
        // In order: fraction, /, cancellation, letter, baseline
        // Note: fraction over is not listed due to example 42(4) which shows a space before the "/"
        static ref REMOVE_SPACE_BEFORE_BRAILLE_INDICATORS: Regex = 
            Regex::new(r"(⠄⠄⠄|⠤⠤⠤)W+([⠼⠸⠪])").unwrap();
        static ref REMOVE_SPACE_AFTER_BRAILLE_INDICATORS: Regex = 
            Regex::new(r"([⠹⠻Lb])W+(⠄⠄⠄)").unwrap();

        // Multipurpose indicator insertion
        // 177.2 -- add after a letter and before a digit (or decimal pt) -- digits will start with N
        static ref MULTI_177_2: Regex = 
            Regex::new(r"(L.)[N𝑁]").unwrap();

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
            Regex::new(r"(?P<start>^|[,W])(?P<minus>⠤?)(?P<face>[SBTIR]*?)N").unwrap();  

        // FIX  add rule 9d after section mark, etc

        // Needed after a typeface change or interior shape modifier indicator
        static ref NUM_IND_9E: Regex = Regex::new(r"(?P<face>[SBTIR]+?)N").unwrap();  
        static ref NUM_IND_9E_SHAPE: Regex = Regex::new(r"(?P<mod>⠸⠫)N").unwrap();  

        // Punctuation chars (Rule 38.6 says don't use before ",", "hyphen", "-", "…")
        // Never use punctuation indicator before these (38-6)
        //      "…": "⠀⠄⠄⠄"
        //      "-": "⠸⠤" (hyphen and dash)
        //      ",": "⠠⠀"     -- spacing already added
        // Rule II.9b (add numeric indicator after punctuation [optional minus[optional .][digit]
        //  because this is run after the above rule, some cases are already caught, so don't
        //  match if there is already a numeric indicator
        static ref NUM_IND_AFTER_PUNCT: Regex = Regex::new(r"(?P<punct>P.)(?P<minus>⠤?)N").unwrap();  

        // Before 79b (punctuation)
        static ref REMOVE_LEVEL_IND_BEFORE_SPACE_COMMA_PUNCT: Regex = Regex::new(r"(?:[↑↓]+b?|b)([W,P]|$)").unwrap();

        static ref REMOVE_LEVEL_IND_BEFORE_BASELINE: Regex = Regex::new(r"(?:[↑↓]+b)").unwrap();

        // Except for the four chars above, the unicode rules always include a punctuation indicator.
        // The cases to remove them (that seem relevant to MathML) are:
        //   Beginning of line or after a space (V 38.1)
        //   After a word (38.4)
        //   2nd or subsequent punctuation (includes, "-", etc) (38.7)
        static ref REMOVE_PUNCT_IND: Regex = Regex::new(r"(^|W|\w)P(.)").unwrap();  

        static ref REPLACE_INDICATORS: Regex =Regex::new(r"([SBTIREDGVHPCLMmb↑↓Nn𝑁W,])").unwrap();  
            
        static ref COLLAPSE_SPACES: Regex = Regex::new(r"⠀⠀+").unwrap();
    }

  // debug!("Before:  \"{}\"", raw_braille);

    // Remove blanks before and after braille indicators
    let result = REMOVE_SPACE_BEFORE_BRAILLE_INDICATORS.replace_all(&raw_braille, "$1$2");
    let result = REMOVE_SPACE_AFTER_BRAILLE_INDICATORS.replace_all(&result, "$1$2");
  // debug!("spaces:  \"{}\"", result);

    // Multipurpose indicator
    let result = MULTI_177_2.replace_all(&result, "${1}m${2}");
    let result = MULTI_177_3.replace_all(&result, "${1}m$2");
    let result = MULTI_177_5.replace_all(&result, "${1}m$2");
  // debug!("MULTI:   \"{}\"", result);

    let result = NUM_IND_9A.replace_all(&result, "$start$minus${face}n");
  // debug!("IND_9A:  \"{}\"", result);

    let result = NUM_IND_9E.replace_all(&result, "${face}n");
    let result = NUM_IND_9E_SHAPE.replace_all(&result, "${mod}n");
  // debug!("IND_9E:  \"{}\"", result);

    // 9b: insert after punctuation (optional minus sign)
    // common punctuation adds a space, so 9a handled it. Here we deal with other "punctuation" 
    // FIX other punctuation and reference symbols (9d)
    let result = NUM_IND_AFTER_PUNCT.replace_all(&result, "$punct${minus}n");
  // debug!("A PUNCT: \"{}\"", &result);

    // strip level indicators
    // checks for punctuation char, so needs to before punctuation is stripped.
    
    let result = REMOVE_LEVEL_IND_BEFORE_SPACE_COMMA_PUNCT.replace_all(&result, "$1");
  // debug!("Punct  : \"{}\"", &result);
    let result = REMOVE_LEVEL_IND_BEFORE_BASELINE.replace_all(&result, "b");
  // debug!("Bseline: \"{}\"", &result);

    let result = REMOVE_PUNCT_IND.replace_all(&result, "$1$2");
  // debug!("Punct38: \"{}\"", &result);

    let result = REPLACE_INDICATORS.replace_all(&result, |cap: &Captures| {
        match NEMETH_INDICATOR_REPLACEMENTS.get(&cap[0]) {
            None => panic!("REPLACE_INDICATORS and NEMETH_INDICATOR_REPLACEMENTS are not in sync"),
            Some(&ch) => ch,
        }
    });

    // Remove unicode blanks at start and end -- do this after the substitutions because ',' introduces spaces
    let result = result.trim_start_matches('⠀').trim_end_matches('⠀');
    let result = COLLAPSE_SPACES.replace_all(&result, "⠀");
   
    return result.to_string();

}

// Typeface: S: sans-serif, B: bold, T: script/blackboard, I: italic, R: Roman
// Language: E: English, D: German, G: Greek, V: Greek variants, H: Hebrew, U: Russian
// Indicators: C: capital, N: number, P: punctuation, M: multipurpose
// Others:
//      W -- whitespace that should be kept (e.g, in a numeral)
//      𝑁 -- hack for special case of a lone decimal pt -- not considered a number but follows rules mostly 
// SRE doesn't have H: Hebrew or U: Russian, so not encoded (yet)
// Note: some "positive" patterns find cases to keep the char and transform them to the lower case version
static UEB_INDICATOR_REPLACEMENTS: phf::Map<&str, &str> = phf_map! {
    // "S" => "⠈⠰",    // sans-serif
    "B" => "⠘",     // bold
    "T" => "⠈",     // script/blackboard
    // "I" => "⠨",     // italic
    // "R" => "",      // roman
    // "E" => "⠰",     // English
    "1" => "⠰",     // Grade 1 symbol
    "𝟙" => "⠰⠰",     // Grade 1 word (1D7D9	MATHEMATICAL DOUBLE-STRUCK DIGIT ONE)
    "𝟏" => "⠰⠰⠰",     // Grade 1 passage (1D7CF	MATHEMATICAL BOLD DIGIT ONE)
    "𝟷" => "⠰⠄",     // Grade 1 terminator (1D7F7	MATHEMATICAL MONOSPACE DIGIT ONE)
    "L" => "⠰",     // Letter -- turns to grade 1 if not removed
    // "D" => "⠸",     // German (Deutsche)
    // "V" => "⠨⠈",    // Greek Variants
    // "H" => "⠠⠠",    // Hebrew
    // "U" => "⠈⠈",    // Russian
    "C" => "⠠",      // capital
    "𝐶" => "⠠⠠",     // capital word
    "N" => "⠼",     // number indicator
    "t" => "⠱",     // shape terminator
    "W" => "⠀",     // whitespace
    "," => "⠂",     // comma
    "." => "⠲",     // period
    "-" => "-",     // hyphen
    "—" => "⠠⠤",     // normal dash (2014) -- assume all normal dashes are unified here [RUEB appendix 3]
    "―" => "⠐⠠⠤",     // long dash (2015) -- assume all long dashes are unified here [RUEB appendix 3]
    "!" => "",      // signals end of script
};

static LETTERS: phf::Set<char> = phf_set! {
    '⠁', '⠃', '⠉', '⠙', '⠑', '⠋', '⠛', '⠓', '⠊', '⠚', '⠅', '⠇', '⠍', 
    '⠝', '⠕', '⠏', '⠟', '⠗', '⠎', '⠞', '⠥', '⠧', '⠺', '⠭', '⠽', '⠵',
};

fn ueb_cleanup(raw_braille: String) -> String {

    lazy_static! {
        // Trim braille spaces before and after braille indicators
        // In order: fraction, /, cancellation, letter, baseline
        // Note: fraction over is not listed due to example 42(4) which shows a space before the "/"
        // static ref REMOVE_SPACE_BEFORE_BRAILLE_INDICATORS: Regex = 
        //     Regex::new(r"(⠄⠄⠄|⠤⠤⠤)W+([⠼⠸⠪])").unwrap();
        static ref REPLACE_INDICATORS: Regex =Regex::new(r"([1𝟙𝟏𝟷SBTIREDGVHPCLMNW,.-—―!])").unwrap();  
            
        static ref COLLAPSE_SPACES: Regex = Regex::new(r"⠀⠀+").unwrap();
    }

    let result = pick_start_mode(&raw_braille);
    let result = result.replace("tW", "W");
    let result = REPLACE_INDICATORS.replace_all(&result, |cap: &Captures| {
        match UEB_INDICATOR_REPLACEMENTS.get(&cap[0]) {
            None => panic!("REPLACE_INDICATORS and NEMETH_INDICATOR_REPLACEMENTS are not in sync: missing '{}'", &cap[0]),
            Some(&ch) => ch,
        }
    });

    // Remove unicode blanks at start and end -- do this after the substitutions because ',' introduces spaces
    // let result = result.trim_start_matches('⠀').trim_end_matches('⠀');
    // let result = COLLAPSE_SPACES.replace_all(&result, "⠀");
   
    return result.to_string();

    fn pick_start_mode(raw_braille: &str) -> String {
        // Need to decide what the start mode should be
        // From http://www.brailleauthority.org/ueb/ueb_math_guidance/final_for_posting_ueb_math_guidance_may_2019_102419.pdf
        //   Unless a math expression can be correctly represented with only a grade 1 symbol indicator in the first three cells
        //   or before a single letter standing alone anywhere in the expression,
        //   begin the expression with a grade 1 word indicator (or a passage indicator if the expression includes spaces)
        // Apparently "only a grade 1 symbol..." means at most one grade 1 symbol based on some examples (GTM 6.4, example 4)
        debug!("raw braille:  '{}'", raw_braille);
        let grade2 = remove_unneeded_mode_changes(&raw_braille, UEB_Mode::Grade2, UEB_Duration::Symbol);
        debug!("Symbol mode:  '{}'", &grade2);
        if is_grade2_string_ok(&grade2) {
            return grade2;
        } else {
            let grade1_word = remove_unneeded_mode_changes(&raw_braille, UEB_Mode::Grade1, UEB_Duration::Word);
            debug!("Word mode:    '{}'", &grade1_word);
            if grade1_word.chars().any(|ch| ch == 'W') {
                let grade1_passage = remove_unneeded_mode_changes(&raw_braille, UEB_Mode::Grade1, UEB_Duration::Passage);
                debug!("Passage mode: '{}'", &grade1_passage);
                return "⠰⠰⠰".to_string() + &grade1_passage + "⠰⠄";
            } else {
                return "⠰⠰".to_string() + &grade1_word;
            }
        }

        fn is_grade2_string_ok(grade2_braille: &str) -> bool {
            // make sure there is not more than one grade one symbol in first three cells
            let mut chars = grade2_braille.chars();
            let mut count = 0;
            for _ in 0..3 {
                if let Some(ch) = chars.next() {
                    if ch == '1' {
                        count += 1;
                    }
                }
            }
            if count > 1 {
                return false;
            }

            let chars = chars.collect::<Vec<char>>();
            let mut i = 0;      // already skipped first three cells
            while i < chars.len() {
                let ch = chars[i];
                if ch == '1' {
                    return false;
                }
                i += 1;
            }
            return true;
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum UEB_Mode {
    Numeric,        // also includes Grade1
    Grade1,
    Grade2,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum UEB_Duration {
    // Standing alone: A braille symbol that is standing alone may have a contracted (grade 2) meaning.
    // A letter or unbroken sequence of letters is “standing alone” if the symbols before and after the letter or
    //   sequence are spaces, hyphens, dashes or any combination thereof, including some common punctuation.
    // Item: An “item” is defined as the next symbol or one of seven groupings listed in Rules of Unified English Braille, §11.4.1.
    Symbol,

    // The grade 1 word indicator sets grade 1 mode for the next word or symbol sequence.
    // A symbol sequence in UEB is defined as an unbroken string of braille signs,
    //   whether alphabetic or non-alphabetic, preceded and followed by a space.
    Word,
    Passage,
}

fn remove_unneeded_mode_changes(raw_braille: &str, start_mode: UEB_Mode, start_duration: UEB_Duration) -> String {
    static LETTER_NUMBERS: phf::Set<char> = phf_set! {
        '⠁', '⠃', '⠉', '⠙', '⠑', '⠋', '⠛', '⠓', '⠊', '⠚',
    };

    // FIX: need to be smarter about moving on wrt to typeforms/typefaces, caps, bold/italic. [maybe just let them loop through the default?]
    let mut mode = start_mode;
    let mut duration = start_duration;
    let mut result = String::default();
    let chars = raw_braille.chars().collect::<Vec<char>>();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        match mode {
            UEB_Mode::Numeric => {
                // Numeric Mode: (from https://uebmath.aphtech.org/lesson1.0 and lesson4.0)
                // Symbols that can appear within numeric mode include the ten digits, comma, period, simple fraction line,
                // line continuation indicator, and numeric space digit symbols.
                // A space or any other symbol not listed here terminates numeric mode.
                // Numeric mode is also terminated by the "!" -- used after a script
                //
                // The numeric indicator also turns on grade 1 mode.
                // When grade 1 mode is set by the numeric indicator,
                //   grade 1 indicators are not used unless a single lower-case letter a-j immediately follows a digit.
                // Grade 1 mode when set by the numeric indicator is terminated by a space, hyphen, dash, or a grade 1 indicator.
                println!("Numeric: ch={}, duration: {:?}", ch, duration);
                match ch {
                    'L' => {
                        // terminate numeric mode -- duration doesn't change
                        // let the default case handle pushing on the chars for the letter (which might include typeforms, etc)
                        if LETTER_NUMBERS.contains(&chars[i+1]) {
                            result.push('1');   // need to distinguish a-j from a digit
                        }
                        i += 1;
                        mode = UEB_Mode::Grade1;
                        // duration remains Word
                    },
                    '1' | '𝟙' => {
                        // numeric mode implies grade 1, so don't output indicator;
                        i += 1;
                        mode = UEB_Mode::Grade1;
                        if start_duration == UEB_Duration::Passage {
                            duration = UEB_Duration::Passage;      // otherwise it remains at Word
                        }
                    },
                    '!' => {
                        // terminate numeric mode -- duration doesn't change
                        i += 1;
                        if i+1 < chars.len() && chars[i] == 'L' && LETTER_NUMBERS.contains(&chars[i+1]) {
                            // special case where the script was numeric and a letter follows, so need to put out G1 indicator
                            result.push('1');
                            // the G1 case should work with 'L' now
                        }
                        mode = UEB_Mode::Grade1;
                    },
                    'N' => {
                        // stay in the same mode (includes numeric "," and "." space) -- don't let default get these chars
                        result.push(chars[i+1]);
                        i += 2;
                    },
                    _ => {
                        // moving out of numeric mode
                        result.push(ch);
                        i += 1;
                        mode = if "W-—―".contains(ch) {start_mode} else {UEB_Mode::Grade1};     // space, hyphen, dash(short & long) RUEB 6.5.1
                    },
                }
            },
            UEB_Mode::Grade1 => {
                // Grade 1 Mode:
                // The numeric indicator also sets grade 1 mode.
                // Grade 1 mode, when initiated by the numeric indicator, is terminated by a space, hyphen, dash or grade 1 terminator.
                // Grade 1 mode is also set by grade 1 indicators.
                println!("Grade 1: ch={}, duration: {:?}", ch, duration);
                match ch {
                    'L' => {
                        // note: be aware of '!' case for Numeric because '1' might already be generated
                        let prev_ch = if i > 1 {chars[i-1]} else {'1'};   // '1' -- anything beside ',' or '.'
                        if duration == UEB_Duration::Symbol || 
                            ( (prev_ch == ',' || prev_ch == '.') && LETTER_NUMBERS.contains(&chars[i+1]) ) {
                            result.push('1');        // need to retain grade 1 indicator (RUEB 6.5.2)
                        }
                        // let the default case handle pushing on the chars for the letter (which might include typeforms, etc)
                        i += 1;
                    },
                    '1' | '𝟙' => {
                        if ch == '𝟙' {
                            duration = UEB_Duration::Word;
                        }
                        // nothing to do -- let the default case handle the following chars
                        i += 1;
                    },
                    'N' => {
                        result.push(ch);
                        result.push(chars[i+1]);
                        i += 2;
                        mode = UEB_Mode::Numeric;
                        duration = UEB_Duration::Word;
                    },
                    'W' => {
                        // this terminates a word mode if there was one
                        result.push(ch);
                        i += 1;
                        if start_duration != UEB_Duration::Passage {
                            duration = UEB_Duration::Symbol;
                            mode = UEB_Mode::Grade2;
                        }
                    },
                    _ => {
                        result.push(ch);
                        i += 1;
                        mode = if "W-—―".contains(ch) {start_mode} else {UEB_Mode::Grade1};     // space, hyphen, dash(short & long) RUEB 6.5.1
                    }
                }
            },
            UEB_Mode::Grade2 => {
                // note: if we ended up using a '1', it only extends to the next char, which is also dealt with, so mode doesn't change
                println!("Grade 2: ch={}, duration: {:?}", ch, duration);
                match ch {
                    'L' => {
                        if stands_alone(&chars, i) {
                            result.push('L');    // leave the 'L' so we can check for 'stands_alone' latter
                        }
                        // let the default case handle pushing on the chars for the letter (which might include typeforms, etc)
                        i += 1;
                    },
                    '1' => {
                        result.push(ch);
                        result.push(chars[i+1]);
                        i += 2;
                    },
                    '𝟙' => {
                        result.push(ch);
                        mode = UEB_Mode::Grade1;
                        duration = UEB_Duration::Word;
                    },
                    'N' => {
                        result.push(ch);
                        result.push(chars[i+1]);
                        i += 2;
                        mode = UEB_Mode::Numeric;
                        duration = UEB_Duration::Word;
                    },
                    _ => {
                        result.push(ch);
                        i += 1;
                    }
                }
            },
        }
    }
    return result;
}

/// Returns true if the ith char "stands alone" (UEB 2..6)
/// This basically means surrounded by white space with some potentially intervening chars
/// The ith char should be an "L"
/// This assumes that there is whitespace before and after the character string
fn stands_alone(chars: &[char], i: usize) -> bool {
    // 1. we scan forward to find the end of the char (could be intervening cap/typeform indicators)
    // 2. we scan backward and check the conditions for "standing-alone"
    // 3. we scan forward and check the conditions for "standing-alone"
    // 4. if we don't find any, return false

    // loop to find the letter char
    let mut end = i + 1;
    while !LETTERS.contains(&chars[end]) {
        end += 1;
        if end == chars.len() {
            error!("Internal error: Didn't find a letter following 'L' at position {} in {}", i, chars.iter().collect::<String>());
            return false;
        }
    }
    let letter = chars[end];
    if letter == 'a' || letter == 'i' || letter == 'o' {
        return false;
    }

    return left_side_stands_alone(&chars[0..i]) && right_side_stands_alone(&chars[end+1..]);

    fn left_side_stands_alone(chars: &[char]) -> bool {
        if chars.len() == 0 {
            return true;
        }
        // FIX: add the rest of the conditions
        return chars[chars.len()-1] == 'W';
    }

    fn right_side_stands_alone(chars: &[char]) -> bool {
        if chars.len() == 0 {
            return true;
        }

        // FIX: add the rest of the conditions
        return chars[0] == 'W';
    }
}
