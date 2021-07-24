use sxd_document::parser;
use sxd_document::Package;
use sxd_document::dom::*;

pub fn speak_mathml(mathml_str: &str) -> String {
    let package = parser::parse(mathml_str)
            .expect("failed to parse XML");
    let mathml = &package_to_element(&package);
    let mathml = crate::canonicalize::canonicalize(mathml);
    let mathml = crate::infer_intent::infer_intent(mathml);
    return crate::speech::speak_mathml(&mathml);
}

pub fn package_to_element(package: &Package) -> Element {
    let document = package.as_document();
    let mathml = get_element(&document);
    trim_element(&mathml);
    return mathml;
}


/*
// some C++ bindings
#[allow(non_camel_case_types)]
type VARIANT_BOOL = i16;
#[allow(non_camel_case_types)]
type long = i32;
#[allow(non_camel_case_types)]
type double = f64;

use bindings::{
    windows::win32::automation::BSTR,
    windows::win32::com::HRESULT,
};


// *****************************************************************
use com::interfaces;
use com::interfaces::IUnknown;

#[allow(non_camel_case_types)]
enum SpeechTagsEnum {
	msSpeechTagsNone		= 0,
	msSpeechTagsSAPI4		= 1,
	msSpeechTagsSAPI5		= 2,
	msSpeechTagsMac		    = 3,
	msSpeechTagsSSML		= 4,
	msSpeechTagsHTML		= 5,
	msSpeechTagsEloquence   = 6,
	msSpeechTagsCount
}

#[allow(non_camel_case_types)]
enum VerbosityEnum {
	msVerbosityLow		= 0,
	msVerbosityMedium	= 3,
	msVerbosityHigh	    = 5
}


com::interfaces! {
    #[uuid("12F66A2A-7614-11D4-BD11-00104BD3F987")]
    pub unsafe interface IDispatch: IUnknown {
    }
   
    #[uuid("32F66A2A-7614-11D4-BD11-00104BD3F987")]
    pub unsafe interface IMathSpeech: IDispatch {
        pub fn SetSpeechTags(&self, speech_tags: long) -> HRESULT; 
        pub fn SetVerbosity(&self, modify: VARIANT_BOOL , verbosityFlags: i64, verbosity: long) -> HRESULT;
        pub fn GetSpokenText(&self, spokenText: &mut BSTR) -> HRESULT;
    }
}




// Main interface
[
	object, oleautomation,
	uuid(32F66A2A-7614-11D4-BD11-00104BD3F987),
	helpstring("Design Science IMathSpeech Interface"),
	pointer_default(unique)
]
interface IMathSpeech : IDispatch {
	cpp_quote("// Set what speech engine standard to use.")
	HRESULT SetSpeechTags([in] SpeechTagsEnum speechTags); 

	cpp_quote("// IMathSpeech::SetVerbosity -- deprecated.  Use IMathSpeechSettings::SetSpeechVerbosity.")
	HRESULT SetVerbosity([in] VARIANT_BOOL modify, [in] long verbosityFlags, [in] VerbosityEnum verbosity); 

	cpp_quote("// Call after setting up")
	HRESULT GetSpokenText([out] BSTR *spokenText);
};

[
	object, oleautomation,
	uuid(32F66A2F-7614-11D4-BD11-00104BD3F987),
	helpstring("Design Science IMathSpeech2 Interface"),
	pointer_default(unique)
]
interface IMathSpeech2 : IMathSpeech {
	cpp_quote("// Call after setting up -- speaks (self-voicing) the expr")
	cpp_quote("// Set the focus if you want to do navigation")
	HRESULT Speak([in] VARIANT_BOOL interruptSpeech, [in] VARIANT_BOOL setFocus);
};


cpp_quote("")
cpp_quote("//=--------------------------------------------------------------------------=")
cpp_quote("// Interface to get/set global speech properties for speaking entire expressions.")
cpp_quote("// These values remain set until changed.")
cpp_quote("// The values set and returned are based on the value set for the the speech tag (eg, SAPI5 if msSpeechTagsSAPI5 set).")
cpp_quote("// If the speech tag is changed, the values will be recalculated to for the new tag.")
cpp_quote("// However, because the specifications are not precise for all standards, it is best to set the values after changing the speech tag.")
cpp_quote("//=--------------------------------------------------------------------------=")
cpp_quote("")
[
	object, oleautomation,
	uuid(32F66A29-7614-11D4-BD11-00104BD3F987),
	helpstring("Design Science IMathSpeechSettings Interface"),
	pointer_default(unique)
]
interface IMathSpeechSettings : IMathSpeech {
	cpp_quote("// Returns the TTS engine standard for which speech is targeted")
	HRESULT GetSpeechTags([out] SpeechTagsEnum *speechTags); 

	cpp_quote("// Set/Get the amount of verbosity used for speech")
	cpp_quote("// The value should be between 0 and 5, with 0 being 'terse' and 5 being 'very verbose'")
	cpp_quote("// This does not affect disambiguation such as 'begin fraction', but rather the verbosity")
	cpp_quote("//   used to speak the meaning.  Eg, x_1 might be 'x 1' or 'x sub 1' or 'x with superscript 1'")
	HRESULT SetSpeechVerbosity([in] long verbosity); 
	HRESULT GetSpeechVerbosity([out] long *verbosity); 

	cpp_quote("")
	cpp_quote("//Set/Get Pitch (default 140hz")
	cpp_quote("//SAPI4: Relative pitch")
	cpp_quote("//  Number is relative to the default/current pitch")
	cpp_quote("//  50 is 1/2 of the default/current pitch, 200 is 2 times the default/current pitch")
	cpp_quote("//  No range is specified by the spec")
	cpp_quote("//SAPI5: Relative pitch")
	cpp_quote("//  Number is in range -24 to 24")
	cpp_quote("//	 -24 is one octave below the default/current pitch, +24 is one octave above")
	cpp_quote("//	 changes are logarithmetic -- a change of +/-1 corresponds to multiplying/dividing by 24th root of 2")
	cpp_quote("//SSML: Relative pitch")
	cpp_quote("//  pitch in hertz (default/current man's voice is about 100hz, woman's 180hz)")
	cpp_quote("//  Note:  other legal values for SSML are not supported, and all numbers are interpreted as relative changes")
	cpp_quote("//ECI: Absolute pitch (relative pitch not supported by ECI)")
	cpp_quote("//  Range is 0 - 100.  Guess is that 0 ~= 42hz, 100 ~= 422hz based on supported \"sapi\" values")
	HRESULT SetPitch([in] double pitch);
	HRESULT GetPitch([out, retval] double* pitch);

	cpp_quote("")
	cpp_quote("//Set/Get Rate (default: 180 wpm)")
	cpp_quote("//SAPI4: Relative rate")
	cpp_quote("//  Number is relative to the default/current rate")
	cpp_quote("//  50 is 1/2 of the default/current rate, 200 is 2 times the default/current rate")
	cpp_quote("//  No range is specified by the spec")
	cpp_quote("//SAPI5: Relative rate")
	cpp_quote("//  Number is in range -10 to 10")
	cpp_quote("//	 -10 is 1/3 of the default/current speed; 10 3 times the default/current speech")
	cpp_quote("//	 changes are logarithmetic -- a change of +/-1 corresponds to multiplying/dividing by 10th root of 3")
	cpp_quote("//SSML: Relative rate")
	cpp_quote("//  0.5 is 1/2 of the default/current rate, 2.0 is 2 times the default/current rate")
	cpp_quote("//  Note:  other legal values for SSML are not supported, and all numbers are interpreted as relative changes")
	cpp_quote("//ECI: Absolute rate (relative rate not supported by ECI)")
	cpp_quote("//  Range is 0 - 250.  Guess is that 0 ~= 72words/min, 250 ~= 1297 words/min based on supported \"sapi\" values")
	HRESULT SetRate([in] double rate);
	HRESULT GetRate([out, retval] double* rate);

	cpp_quote("")
	cpp_quote("//Set/Get Volume (default: full)")
	cpp_quote("//SAPI4: Relative volume")
	cpp_quote("//  Number is relative to the default/current rate")
	cpp_quote("//  Range is 0 - 065535")
	cpp_quote("//SAPI5: Relative volume")
	cpp_quote("//  Number is in range 0 to 100")
	cpp_quote("//SSML: Relative volume")
	cpp_quote("//  Number is in range 0 to 100")
	cpp_quote("//  Note:  other legal values for SSML are not supported, and all numbers are interpreted as relative changes")
	cpp_quote("//ECI: Absolute volume (relative volume not supported by ECI)")
	cpp_quote("//  Range is 0 - 100")
	HRESULT SetVolume([in] double volume); 
	HRESULT GetVolume([out, retval] double* volume); 

	cpp_quote("// Specific name of voice to use (currently not used)")
	HRESULT SetVoice([in] BSTR voice);
	HRESULT GetVoice([out, retval] BSTR* voice);

	cpp_quote("// Target disability group for speech")
	HRESULT SetTargetGroup([in] BSTR target);
	HRESULT GetTargetGroup([out, retval] BSTR* target);


	cpp_quote("//Language strings follow what is used in HTML.  Typically 'en' or 'en-gb'.  Case doesn't matter")
	cpp_quote("//For the main language, see http://www.loc.gov/standards/iso639-2/php/code_list.php")
	cpp_quote("//For the subregion, see http://www.iso.org/iso/english_country_names_and_code_elements")
	cpp_quote("//If the language or speech style is not supported, E_INVALIDARG is returned")
	HRESULT SetLanguage([in] BSTR lang);
	HRESULT GetLanguage([out, retval] BSTR* lang);

	cpp_quote("// Name of speech style")
	HRESULT SetSpeechStyle([in] BSTR style);
	HRESULT GetSpeechStyle([out, retval] BSTR* style);

	cpp_quote("// Currently supported subjects are:  \"Geometry\", \"ProbabilityAndStatistics\", \"Calculus\"")
	cpp_quote("// The bool flag says whether only the given value should be used or added to the subject list")
	cpp_quote("// An empty string with only=VARIANT_TRUE will clear all the subjects")
	HRESULT SetSubjectArea([in] BSTR subject, VARIANT_BOOL only);
	HRESULT GetSubjectArea([out, retval] BSTR* subject);

	cpp_quote("// Where meaningful, use the language defined in the document")
	HRESULT SetUseDocumentLanguage(VARIANT_BOOL useDocLanguage);
	HRESULT GetUseDocumentLanguage([out, retval] VARIANT_BOOL* useDocLanguage);

};

[
	object, oleautomation,
	uuid(32F66A2B-7614-11D4-BD11-00104BD3F987),
	helpstring("Design Science IMathSynchronization Interface"),
	pointer_default(unique)
]
*/


fn get_element<'d>(doc: &Document<'d>) -> Element<'d> {
    let mut result = None;
    for root_child in doc.root().children() {
        if let ChildOfRoot::Element(e) = root_child {
           assert!(result == None);
           result = Some(e);
        }
    };
    let element = result.unwrap();
    element
}

#[allow(dead_code)]
fn trim_doc(doc: &Document) {
    for root_child in doc.root().children() {
        if let ChildOfRoot::Element(e) = root_child {
            trim_element(&e);
        } else {
            doc.root().remove_child(root_child);        // comment or processing instruction
        }
    };
}

fn trim_element(e: &Element) {
    // assume 'e' doesn't have element children until proven otherwise
    // this means we keep Text children until we are proven they aren't needed
    for child in e.children() {
        match child {
            ChildOfElement::Element(_) => {
                for child in e.children() {
                    if let ChildOfElement::Element(el) = child {
                        trim_element(&el);
                    } else {
                        e.remove_child(child);        // text, comment, or processing instruction
                    }
                }
                return;
            },
            ChildOfElement::Text(t) => {
                t.set_text( t.text().trim());
            },
            _ => {
                e.remove_child(child);
            }
        }
    }
}


// used for testing trim
// returns true if two Documents are equal
#[allow(dead_code)]
fn is_same_doc(doc1: &Document, doc2: &Document) -> bool {
    if doc1.root().children().len() != doc2.root().children().len() {
        return false;
    }
    for root_child in doc1.root().children().iter().zip(doc2.root().children().iter()) {
        let (c1, c2) = root_child;
        match c1 {
            ChildOfRoot::Element(e1) => {
                if let ChildOfRoot::Element(e2) = c2 {
                    if is_same_element(e1, e2) {
                        continue;
                    }
                }
                return false;
            },
            ChildOfRoot::Comment(com1) => {
                if let ChildOfRoot::Comment(com2) = c2 {
                    if com1.text() == com2.text() {
                        continue;
                    }
                }
                return false;
            }
            ChildOfRoot::ProcessingInstruction(p1) => {
                if let ChildOfRoot::ProcessingInstruction(p2) = c2 {
                    if p1.target() == p2.target() && p1.value() == p2.value() {
                        continue;
                    }
                }
                return false;
            }
        }
    };
    return true;
}

#[allow(dead_code)]
fn is_same_element(e1: &Element, e2: &Element) -> bool {
    // assume 'e' doesn't have element children until proven otherwise
    // this means we keep Text children until we are proven they aren't needed
    if e1.children().len() != e2.children().len() {
        return false;
    }
    for element_child in e1.children().iter().zip(e2.children().iter()) {
        let (c1, c2) = element_child;
        match c1 {
            ChildOfElement::Element(child1) => {
                if let ChildOfElement::Element(child2) = c2 {
                    if is_same_element(child1, child2) {
                        continue;
                    }
                }
                return false;
            },
            ChildOfElement::Comment(com1) => {
                if let ChildOfElement::Comment(com2) = c2 {
                    if com1.text() == com2.text() {
                        continue;
                    }
                }
                return false;
            }
            ChildOfElement::ProcessingInstruction(p1) => {
                if let ChildOfElement::ProcessingInstruction(p2) = c2 {
                    if p1.target() == p2.target() && p1.value() == p2.value() {
                        continue;
                    }
                }
                return false;
            }
            ChildOfElement::Text(t1) => {
                if let ChildOfElement::Text(t2) = c2 {
                    if t1.text() == t2.text() {
                        continue;
                    }
                }
                return false;
            }

        }
    };
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn are_parsed_strs_equal(str1: &str, str2: &str) -> bool {
        let package1 = &parser::parse(str1).expect("Failed to parse input");
        let doc1 = package1.as_document();
        trim_doc(&doc1);
        
        let package2 = parser::parse(str2).expect("Failed to parse input");
        let doc2 = package2.as_document();
        trim_doc(&doc2);
            
        is_same_doc(&doc1, &doc2)
    }

    #[test]
    fn trim_same() {
        let trimmed_str = "<math><mrow><mo>-</mo><mi>a</mi></mrow></math>";
        assert!(are_parsed_strs_equal(trimmed_str, trimmed_str));
    }

    #[test]
    fn trim_whitespace() {
        let trimmed_str = "<math><mrow><mo>-</mo><mi>a</mi></mrow></math>";
        let whitespace_str = "<math> <mrow ><mo>-</mo><mi> a </mi></mrow ></math>";
        assert!(are_parsed_strs_equal(trimmed_str, whitespace_str));
    }

    #[test]
    fn trim_comment() {
        let whitespace_str = "<math> <mrow ><mo>-</mo><mi> a </mi></mrow ></math>";
        let comment_str = "<math><mrow><mo>-</mo><!--a comment --><mi>a</mi></mrow></math>";
        assert!(are_parsed_strs_equal(comment_str, whitespace_str));
    }
 
    #[test]
    fn trim_differs() {
        let whitespace_str = "<math> <mrow ><mo>-</mo><mi> a </mi></mrow ></math>";
        let different_str = "<math> <mrow ><mo>-</mo><mi> b </mi></mrow ></math>";
        assert!(!are_parsed_strs_equal(different_str, whitespace_str));
    }
}