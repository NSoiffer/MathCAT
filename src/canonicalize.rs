//! Converts the MathML to some sort of canonical MathML.
//!
//! Some changes made:
//! * extra whitespace at the start/end of tokens is trimmed.
//! * "equivalent" characters are converted to a chosen character.
//! * known "bad" MathML is cleaned up (this will likely be an ongoing effort)
//! * mrows are added based on operator priorities from the MathML Operator Dictionary
#![allow(clippy::needless_return)]
use crate::errors::*;
use sxd_document::dom::*;
use sxd_document::QName;
use phf::{phf_map, phf_set};
use crate::xpath_functions::IsBracketed;
use std::{ptr::eq as ptr_eq};
use crate::pretty_print::*;
use regex::Regex;

// FIX: DECIMAL_SEPARATOR should be set by env, or maybe language
const DECIMAL_SEPARATOR: &str = ".";
const CHANGED_ATTR: &str = "data-changed";
const ADDED_ATTR_VALUE: &str = "added";

// (perfect) hash of operators built from MathML's operator dictionary
static OPERATORS: phf::Map<&str, OperatorInfo> = include!("operator-info.in");


// The set of fence operators that can being either a left or right fence (or infix). For example: "|".
static AMBIGUOUS_OPERATORS: phf::Set<&str> = phf_set! {
	"|", "∥", "\u{2016}"
};

// static vars used when canonicalizing
lazy_static!{
	// lowest priority operator so it is never popped off the stack
	static ref LEFT_FENCEPOST: OperatorInfo = OperatorInfo{ op_type: OperatorTypes::LEFT_FENCE, priority: 0, next: &None };

	static ref INVISIBLE_FUNCTION_APPLICATION: &'static OperatorInfo = OPERATORS.get(&"\u{2061}").unwrap();
	static ref IMPLIED_TIMES: &'static OperatorInfo = OPERATORS.get(&"\u{2062}").unwrap();
	static ref IMPLIED_INVISIBLE_COMMA: &'static OperatorInfo = OPERATORS.get(&"\u{2063}").unwrap();
	static ref IMPLIED_INVISIBLE_PLUS: &'static OperatorInfo = OPERATORS.get(&"\u{2064}").unwrap();

	// FIX: any other operators that should act the same (e.g, plus-minus and minus-plus)?
	static ref PLUS: &'static OperatorInfo = OPERATORS.get(&"+").unwrap();
	static ref MINUS: &'static OperatorInfo = OPERATORS.get(&"-").unwrap();

	static ref TIMES_SIGN: &'static OperatorInfo = OPERATORS.get(&"×").unwrap();

	// IMPLIED_TIMES_HIGH_PRIORITY -- used in trig functions for things like sin 2x cos 2x where want > function app priority
	static ref IMPLIED_TIMES_HIGH_PRIORITY: OperatorInfo = OperatorInfo{
		op_type: OperatorTypes::INFIX, priority: 851, next: &None
	};

	// Useful static defaults to have available if there is no character match
	static ref DEFAULT_OPERATOR_INFO_PREFIX: &'static OperatorInfo = &OperatorInfo{
		op_type: OperatorTypes::PREFIX, priority: 260, next: &None
	};
	static ref DEFAULT_OPERATOR_INFO_INFIX: &'static OperatorInfo = &OperatorInfo{
		op_type: OperatorTypes::INFIX, priority: 260, next:& None
	};
	static ref DEFAULT_OPERATOR_INFO_POSTFIX: &'static OperatorInfo = &OperatorInfo{
		op_type: OperatorTypes::POSTFIX, priority: 260, next: &None
	};

	// avoids having to use Option<OperatorInfo> in some cases
	static ref ILLEGAL_OPERATOR_INFO: &'static OperatorInfo = &OperatorInfo{
		op_type: OperatorTypes::INFIX, priority: 999, next: &None
	};
}

// Operators are either PREFIX, INFIX, or POSTFIX, but can also have other properties such as LEFT_FENCE
bitflags! {
	struct OperatorTypes: u32 {
		const NONE		= 0x0;
		const PREFIX	= 0x1;
		const INFIX		= 0x2;
		const POSTFIX	= 0x4;
		const FENCE		= 0x8;
		const LEFT_FENCE= 0x9;
		const RIGHT_FENCE=0xc;
		const UNSPECIFIED=0xf;		// 'and-ing will match anything
	}
}

// OperatorInfo is a key structure for parsing.
// They OperatorInfo is this program's representation of MathML's Operator Dictionary.
// The OperatorTypes say how the operator can group (can be overridden with @form="..." on an element).
//   Basically, it says the operator can be at the start, middle, or end of an mrow.
// The priority field gives the relationships between operators so that lower priority operators are towards the root of the tree.
//   E.g.,  '=' is lower priority than (infix) '+', which in turn is lower priority than multiplication.
// The operator info is a linked list because some operators (not many) have alternatives (e.g, '+' is both prefix and infix)
// All OperatorInfo is static info, with some special static defaults to capture when it is not listed in the operator dictionary.
#[derive(Clone, Debug)]
struct OperatorInfo {
	op_type: OperatorTypes,		// can be set on <mo>
	priority: usize,			// not settable on an element
	next: &'static Option<OperatorInfo>,	// can be both prefix & infix (etc) -- chain of options
}

// The character is separated out from the OperatorInfo as this allows the OperatorInfo to be static (can use default values)
#[derive(Clone, Debug)]
struct OperatorPair<'op> {
	ch: &'op str,
	op: &'static OperatorInfo
}

impl<'op> OperatorPair<'op> {
	fn new() -> OperatorPair<'op> {
		return OperatorPair{
			ch: "illegal",					// value 'illegal' used only in debugging, if then
			op: *ILLEGAL_OPERATOR_INFO,		// ILLEGAL_OPERATOR_INFO avoids using <Option>
		};
	}
}

// OperatorVersions is a convenient data structure when looking to see whether the operator should be prefix, infix, or postfix.
// It is only used in one place in the code, so this could maybe be eliminated and the code localized to where it is used.
#[derive(Debug)]
struct OperatorVersions {
	prefix: Option<&'static OperatorInfo>,
	infix: Option<&'static OperatorInfo>,
	postfix: Option<&'static OperatorInfo>,
}

impl OperatorVersions {
	fn new(op: &'static OperatorInfo) -> OperatorVersions {
		let mut op = op;
		let mut prefix = None;
		let mut infix = None;
		let mut postfix = None;
		loop {
			if op.is_prefix() {
				prefix = Some( op );
			} else if op.is_infix() {
				infix = Some( op )
			} else if op.is_postfix() {
				postfix = Some( op );
			} else {
				panic!("OperatorVersions::new: operator is not prefix, infix, or postfix")
			}
			//let another_op = op.next;
			match &op.next {
				None => break,
				Some(alt_op) => op = alt_op,
			}
		}
		return OperatorVersions{prefix, infix, postfix};
	}
}


impl OperatorInfo {
	fn is_prefix(&self) -> bool {
		return (self.op_type.bits & OperatorTypes::PREFIX.bits) != 0;
	}

	fn is_infix(&self) -> bool {
		return (self.op_type.bits & OperatorTypes::INFIX.bits) != 0;
	}

	fn is_postfix(&self) -> bool {
		return (self.op_type.bits & OperatorTypes::POSTFIX.bits) != 0;
	}

	fn is_left_fence(&self) -> bool {
		return self.op_type.bits & OperatorTypes::LEFT_FENCE.bits == OperatorTypes::LEFT_FENCE.bits;
	}

	fn is_right_fence(&self) -> bool {
		return self.op_type.bits & OperatorTypes::RIGHT_FENCE.bits ==OperatorTypes::RIGHT_FENCE.bits;
	}

	fn is_operator_type(&self, op_type: OperatorTypes) -> bool {
		return self.op_type.bits & op_type.bits != 0;
	}

	fn is_plus_or_minus(&self) -> bool {
		return ptr_eq(self, *PLUS) || ptr_eq(self, *MINUS);
	}

	fn is_times(&self) -> bool {
		return ptr_eq(self, *IMPLIED_TIMES) || ptr_eq(self, *TIMES_SIGN);
	}

	fn is_nary(&self, previous_op: &OperatorInfo) -> bool {
		return	ptr_eq(previous_op,self) ||
				(previous_op.is_plus_or_minus() && self.is_plus_or_minus()) ||
				(previous_op.is_times() && self.is_times());
	}
}

// StackInfo contains all the needed information for deciding shift/reduce during parsing.
// The stack itself is just a Vec of StackInfo (since we only push, pop, and look at the top)
// There are a number of useful functions defined on StackInfo. 
struct StackInfo<'a, 'op>{
	mrow: Element<'a>,			// mrow being built
	op_pair: OperatorPair<'op>,	// last operator placed on stack
	is_operand: bool,			// true if child at end of mrow is an operand (as opposed to an operator)
}

impl<'a, 'op:'a> StackInfo<'a, 'op> {
	fn new(doc: Document<'a>) -> StackInfo<'a, 'op> {
		// println!("  new empty StackInfo");
		let mrow = create_mathml_element(&doc, "mrow") ;
		mrow.set_attribute_value(CHANGED_ATTR, ADDED_ATTR_VALUE);
		return StackInfo{
			mrow,
			op_pair: OperatorPair{ ch: "\u{E000}", op: &LEFT_FENCEPOST },
			is_operand: false,
		}
	}

	fn with_op<'d>(doc: &'d Document<'a>, node: Element<'a>, op_pair: OperatorPair<'op>) -> StackInfo<'a, 'op> {
		// println!("  new StackInfo with '{}' and operator {}/{}", name(&node), show_invisible_op_char(op_pair), op_pair.op.priority);
		let mrow = create_mathml_element(doc, "mrow");
		mrow.set_attribute_value(CHANGED_ATTR, ADDED_ATTR_VALUE);
		mrow.append_child(node);
		return StackInfo {
			mrow,
			op_pair,
			is_operand: false,
		}
	}

	fn priority(&self) -> usize {
		return self.op_pair.op.priority;
	}

	fn last_child_in_mrow(&self) -> Option<Element<'a>> {
		let children = self.mrow.children();
		if children.is_empty() {
			return None
		} else {
			return Some( as_element(children[children.len() - 1]) );
		}
	}

	fn add_child_to_mrow(&mut self, child: Element<'a>, child_op: OperatorPair<'op>) {
		// println!("  adding '{}' to mrow[{}], operator '{}/{}'",
		// 			element_summary(child), self.mrow.children().len(), show_invisible_op_char(child_op), child_op.priority);
		self.mrow.append_child(child);
		if ptr_eq(child_op.op, *ILLEGAL_OPERATOR_INFO) {
			assert!(!self.is_operand); 	// should not have two operands in a row
			self.is_operand = true;
		} else {
			self.op_pair = child_op;
			self.is_operand = false;
		}
	}

	fn remove_last_operand_from_mrow(&mut self) -> Element<'a> {
		let children = self.mrow.children();
		assert!( !children.is_empty() );
		assert!( self.is_operand || children.len()==1 );		// could be operator that is forced to be interpreted as operand -- eg, bad input like "x+("
		self.is_operand = false;
		let last_operand = as_element(children[children.len()-1]);
		// println!("  Removing last element '{}' from mrow[{}]",element_summary(last_operand), children.len());
		last_operand.remove_from_parent();
		return last_operand;
	}

}


fn create_mathml_element<'a>(doc: &Document<'a>, name: &str) -> Element<'a> {
	return doc.create_element(sxd_document::QName::with_namespace_uri(
		Some("http://www.w3.org/1998/Math/MathML"),
		name));
}

/// Canonicalize does several things:
/// 1. cleans up the tree so all extra white space is removed (should only have element and text nodes)
/// 2. normalize the characters
/// 3. clean up "bad" MathML based on known output from some converters (TODO: still a work in progress)
/// 4. the tree is "parsed" based on the mo (priority)/mi/mn's in an mrow
///    *  this adds mrows mrows and some invisible operators (implied times, function app, ...)
///    * extra mrows are removed
///    * implicit mrows are turned into explicit mrows (e.g, there will be a single child of 'math')
///
/// Canonicalize is pretty conservative in adding new mrows and won't do it if:
/// * there is an intent attr
/// * if the mrow starts and ends with a fence (e.g, French open interval "]0,1[")
///
/// An mrow is never deleted unless it is redundant.
pub fn canonicalize(mathml: Element) -> Element {
	let context = CanonicalizeContext::new();
	return context.canonicalize(mathml);
}

struct CanonicalizeContext {
}

impl CanonicalizeContext {
	fn new() -> CanonicalizeContext {
		return CanonicalizeContext{}
	}

	fn canonicalize<'a>(&self, mathml: Element<'a>) -> Element<'a> {
		// println!("MathML before canonicalize:\n{}", mml_to_string(&mathml));
		let converted_mathml = mathml;
	
		if name(&mathml) != "math" {
			// println!("Didn't start with <math> element -- attempting repair");
			let math_element = create_mathml_element(&converted_mathml.document(), "math");
			math_element.set_attribute_value(CHANGED_ATTR, ADDED_ATTR_VALUE);
			math_element.append_child(converted_mathml);
		}
		converted_mathml.set_name(QName::with_namespace_uri(Some("http://www.w3.org/1998/Math/MathML"), "math"));
		self.clean_mathml(converted_mathml);
		let children = converted_mathml.children();
		if children.len() > 1 {
			// start canonicalization by adding an mrow -- then the rest flows
			let mrow_element = create_mathml_element(&converted_mathml.document(), "mrow");
			mrow_element.set_attribute_value(CHANGED_ATTR, ADDED_ATTR_VALUE);
			mrow_element.append_children(children);
			converted_mathml.append_child(mrow_element);
		}
		let converted_mathml = self.canonicalize_mrows(converted_mathml);
		match converted_mathml {
			Ok(e) => {
				println!("\nMathML after canonicalize:\n{}", mml_to_string(&e));
				return e;
			},
			Err(e)  => {
				crate::speech::print_errors( &e.chain_err(|| mml_to_string(&mathml)) );
				return mathml;
			},
		};
	}
	
	// This function does some cleanup of MathML (mostly fixing bad MathML)
	// Unlike the main canonicalization routine, significant tree changes happen here
	// Changes to "good" MathML:
	// 1. mfenced -> mrow
	// 2. mspace and mtext with only whitespace are thrown out unless in a required element position
	fn clean_mathml<'a>(&self, mathml: Element<'a>) -> Option<Element<'a>> {
		lazy_static! {
            static ref IS_WHITESPACE: Regex = Regex::new(r"^\s+$").unwrap();    // only Unicode whitespace
        }
		const ELEMENTS_WITH_FIXED_NUMBER_OF_CHILDREN: [&str; 10] =
				["mfrac", "mroot", "msub", "msup", "msupsup","munder", "mover", "munderover", "mmultiscripts", "mlongdiv"];
		let parent_requires_child = 
			if name(&mathml) == "math" {
				false
			} else {
				ELEMENTS_WITH_FIXED_NUMBER_OF_CHILDREN.contains(
					&name(&mathml.parent().unwrap().element().unwrap())
				)
			};
		match name(&mathml) {
			"mi" | "mn" | "ms" | "mglyph" => {return Some(mathml);},
			"mo" => {
				// common bug: trig functions, lim should be mi
				let text = as_text(mathml);
				return crate::definitions::DEFINITIONS.with(|definitions| {
					if text == "lim" ||
					   definitions.trig_function_names.as_hashset().borrow().contains(text) {
						let mi = create_mathml_element(&mathml.document(), "mi");
						mi.set_text(text);
						return Some(mi);
					} else {
						return Some(mathml);
					}
				});
			},
			"mtext" => {
				let text = as_text(mathml);
				return if parent_requires_child || (!text.is_empty() && !IS_WHITESPACE.is_match(&text)) {Some(mathml)} else {None};
			},
			"mfenced" => {return Some( convert_mfenced_to_mrow(mathml) )} ,
			"mspace" | "mphantom" => {
				return if parent_requires_child {Some(mathml)} else {None};
			},
			_  => {
				let mut new_children = Vec::with_capacity(mathml.children().len());
				for child in mathml.children() {
					let child = as_element(child);			
					if let Some(new_child) = self.clean_mathml(child) {
						new_children.push(new_child);
					}
				}

				// Throw out mstyle -- to do this, we need to avoid mstyle being the arg of clean_mathml
				// FIX: should probably push the attrs down to the children (set in 'self')
				if name(&mathml) == "mstyle" {
					if new_children.len() == 1 {
						return Some( new_children[0] );
					} else {
						// wrap the children in an mrow
						let mrow = create_mathml_element(&mathml.document(), "mrow");
						mrow.append_children(new_children);
						return Some(mrow);
					}
				} else {
					mathml.clear_children();
					mathml.append_children(new_children);
					return Some(mathml);				
				}
			}
		}

		fn convert_mfenced_to_mrow(mathml: Element) -> Element {
			// FIX: implement this
			return mathml;
		}
	}

	fn canonicalize_mrows<'a>(&self, mathml: Element<'a>) -> Result<Element<'a>> {
		let tag_name = name(&mathml);
		mathml.set_name(QName::with_namespace_uri(Some("http://www.w3.org/1998/Math/MathML"), tag_name));
		match tag_name {
			"mi" | "ms" | "mtext" | "mspace" | "mglyph" => {
				self.canonicalize_plane1(mathml);
				return Ok( mathml ); },
			"mo" => {
				self.canonicalize_plane1(mathml);
				self.canonicalize_mo_text(mathml);
				return Ok( mathml );
			},
			"mn" => {
				// FIX: hack cleanup for mn's that are big numbers composed of digit blocks and ","s (still have mn's and mo's -- see canonicalize.tdl)
				// let bigNumValue;
				// if input->GetAttrValue(L"isBigNumber", bigNumValue)) {
				// 	return StripLeafNodes(input->Copy(True));
				// }
				self.canonicalize_plane1(mathml);
				return Ok( mathml );
			},
			"mrow" => {
				return Ok( self.canonicalize_mrows_in_mrow(mathml)? );
			},
			_ => {
				// recursively try to make mrows in other structures (eg, num/denom in fraction)
				let mut new_children = Vec::with_capacity(mathml.children().len());
				for child in mathml.children() {
					match child {
						ChildOfElement::Element(e) => {
							new_children.push( ChildOfElement::Element(self.canonicalize_mrows(e)? ));
						},
						_ => panic!("Should have been an element or text"),
					}
				}
				mathml.replace_children(new_children);
				return Ok( mathml );
			},
		}
	}
	
	fn canonicalize_plane1<'a>(&self, mi: Element<'a>) -> Element<'a> {
		// map names to start of Unicode blocks (lower case, upper case, digits)
		static MATH_VARIANTS: phf::Map<&str, (u32, u32, u32)> = phf_map! {
			// "normal" -- nothing to do
			// "italic" -- nothing to do
			"bold" => (0x1D41A, 0x1D400, 0x1D7CE),
			"bold-italic" => (0x1D482, 0x1D468, '0' as u32),
			"double-struck" => (0x1D552, 0x1D538, 0x1D7D8),
			"bold-fraktur" => (0x1D586, 0x1D56C, '0' as u32),
			"script" => (0x1D4B6, 0x1D49C, '0' as u32),
			"bold-script" => (0x1D4EA, 0x1D4D0, '0' as u32),
			"fraktur" => (0x1D51E, 0x1D504, '0' as u32),
			"sans-serif" => (0x1D51E, 0x1D5BA, 0x1D7E2),
			"bold-sans-serif" => (0x1D5EE, 0x1D5D4, 0x1D7EC),
			"sans-serif-italic" => (0x1D622, 0x1D608, 0x1D7E2),
			"sans-serif-bold-italic" => (0x1D656, 0x1D63C, 0x1D7EC),
			"monospace" => (0x1D68A, 0x1D670, 0x1D7F6),
		};

		let variant = mi.attribute_value("mathvariant");
		if variant.is_none() {
			return mi;
		}

		let mi_text = as_text(mi);
		let new_text = match MATH_VARIANTS.get(variant.unwrap()) {
			None => mi_text.to_string(),
			Some(start) => 
				shift_text(mi_text, start.0, start.1, start.2),
		};
		mi.remove_attribute("mathvariant");
		mi.set_text(&new_text);
		return mi;

		fn shift_text(old_text: &str, lower_start: u32, upper_start: u32, digit_start: u32) -> String {
			// if there is no block for something, use 'a', 'A', '0' as u32 as that will be a no-op
			let mut new_text = String::new();
			for ch in old_text.chars() {
				new_text.push(
					if ch.is_ascii_lowercase() {
						shift_char(ch, lower_start - 'a' as u32)
					} else if ch.is_uppercase() {
						shift_char(ch, upper_start - 'A' as u32)
					} else if ch.is_ascii_digit() {
						shift_char(ch, digit_start - '0' as u32)
					} else {
						ch
					}
				)
			}
			return new_text;

			fn shift_char(ch: char, shift: u32) -> char {
				// there are "holes" in the math alphanumerics due to legacy issues
				// this table maps the holes to their legacy location
				static EXCEPTIONS: phf::Map<u32, u32> = phf_map! {
					0x1D455u32 => 0x210Eu32,
					0x1D49Du32 => 0x212Cu32,
					0x1D4A0u32 => 0x2130u32,
					0x1D4A1u32 => 0x2131u32,
					0x1D4A3u32 => 0x210Bu32,
					0x1D4A4u32 => 0x2110u32,
					0x1D4A7u32 => 0x2112u32,
					0x1D4A8u32 => 0x2133u32,
					0x1D4ADu32 => 0x211Bu32,
					0x1D4BAu32 => 0x212Fu32,
					0x1D4BCu32 => 0x210Au32,
					0x1D4C4u32 => 0x2134u32,
					0x1D506u32 => 0x212Du32,
					0x1D50Bu32 => 0x210Cu32,
					0x1D50Cu32 => 0x2111u32,
					0x1D515u32 => 0x211Cu32,
					0x1D51Du32 => 0x2128u32,
					0x1D53Au32 => 0x2102u32,
					0x1D53Fu32 => 0x210Du32,
					0x1D545u32 => 0x2115u32,
					0x1D547u32 => 0x2119u32,
					0x1D548u32 => 0x211Au32,
					0x1D549u32 => 0x211Du32,
					0x1D551u32 => 0x2124u32,
				};
								
				let shifted_ch = ch as u32 + shift;
				return unsafe { char::from_u32_unchecked(
					match EXCEPTIONS.get(&shifted_ch) {
						None => shifted_ch,
						Some(exception_value) => *exception_value,
					}
				) }
			}
		}
	}

	fn canonicalize_mo_text<'a>(&self, mo: Element<'a>) {
		let parent_name = name(&mo);		// guaranteed to exist
		let mut mo_text = as_text(mo);
		if parent_name == "mover" || parent_name == "munder" || parent_name == "munderover" {
			// canonicalize various diacritics for munder, mover, munderover
			mo_text = match mo_text {
				"\u{02C9}"| "\u{0304}"| "\u{0305}"| "\u{2212}" => "_",
				"\u{02BC}" => "`",
				"\u{02DC}" => "~",
				"\u{02C6}"| "\u{0302}" => "^",
				"\u{02D9}"| "\u{0307}" => ".",
				"\u{0308}" => "¨",
				_ => mo_text,
			}
			// FIX: MathType generates the wrong version of union and intersection ops (binary instead of unary)
		}
		mo_text = match mo_text {
			"\u{2212}" => "-",
			"\u{00AF}"| "\u{02C9}"| "\u{0304}"| "\u{0305}" => "_",
			_ => mo_text,
		};
		mo.set_text(mo_text);
	}
	
		
	// Find the operator associated with the 'mo_node'
	// This is complicated by potentially needing to distinguish between the
	//   prefix, infix, or postfix version of the operator.
	// To figure out prefix, we need to look at the node on the left; for postfix, we need to look to the left
	// If the node of the left has been parsed, then this works.
	// For example, suppose we want to determine if the "+" in 'x < n!+1' is prefix or infix.
	//   If we simply looked left without parsing, we'd see an operator and choose prefix unless we could figure out that
	//   that "!" was postfix.  But if it had been parsed, we'd see an mrow (operand) and tree "+" as infix (as it should).
	// The same problem applies on the right for postfix operators, but a problem is rare for those
	//   e.g., n!!n -- ((n!)!)*n or (n!)*(!n)  -- the latter doesn't make semantic sense though
	// FIX:  the above ignores mspace and other nodes that need to be skipped to determine the right node to determine airity
	// FIX:  the postfix problem above should be addressed
	fn find_operator<'a>(&self, mo_node: Element<'a>, previous_operator: Option<&'static OperatorInfo>,
						previous_node: Option<Element<'a>>, next_node: Option<Element<'a>>) -> &'static OperatorInfo {
		// get the unicode value and return the OpKeyword associated with it
		assert!( name(&mo_node) == "mo");
	
		// if a form has been given, that takes precedence
		let form = mo_node.attribute_value("form");
		let op_type =  match form {
			None => compute_type_from_position(self, previous_operator, previous_node, next_node),
			Some(form) => match form.to_lowercase().as_str() {
				"prefix" => OperatorTypes::PREFIX,
				"postfix" => OperatorTypes::POSTFIX,
				_ => OperatorTypes::INFIX,
			}
		};	
	
		let operator_str = as_text(mo_node);
		let found_op_info = OPERATORS.get(operator_str);
		if found_op_info.is_none() {
			// no known operator -- return the unknown operator with the correct "fix" type
			return op_not_in_operator_dictionary(op_type);
		}
	
		let found_op_info = found_op_info.unwrap();
		let matching_op_info = find_operator_info(found_op_info, op_type, form.is_some());
		if ptr_eq(matching_op_info, *ILLEGAL_OPERATOR_INFO) {
			return op_not_in_operator_dictionary(op_type);
		} else {
			return matching_op_info;
		}

	
		fn compute_type_from_position<'a>(context: &CanonicalizeContext, previous_operator: Option<&'static OperatorInfo>, previous_node: Option<Element<'a>>, next_node: Option<Element<'a>>) -> OperatorTypes {
			// based on choices, pick one that fits the context
			// if there isn't an obvious one, we have parsed the left, but not the right, so discount that
		
			// Trig functions have some special syntax
			// We need to to treat '-' as prefix for things like "sin -2x"
			// Need to be careful because (sin - cos)(x) needs an infix '-'
			// Return either the prefix or infix version of the operator
			if next_node.is_some() &&
			   context.is_function_name(get_possible_embellished_node(next_node.unwrap()), None) {
				return OperatorTypes::INFIX;
			}
			if previous_node.is_some() &&
			   context.is_function_name(get_possible_embellished_node(previous_node.unwrap()), None) {
				return OperatorTypes::PREFIX;
			}
		
			// after that special case, start with the obvious cases...
			let operand_on_left = previous_operator.is_none() || previous_operator.unwrap().is_postfix();	// operand or postfix operator
			let operand_on_right = next_node.is_some() && name(&get_possible_embellished_node(next_node.unwrap())) !="mo";			// FIX:  could improve by checking if it is a prefix op
		
			if operand_on_left && operand_on_right {
				return OperatorTypes::INFIX;	// infix
			} else if !operand_on_left && operand_on_right {
				return OperatorTypes::PREFIX;	// prefix
			} else if operand_on_left && !operand_on_right {
				return OperatorTypes::POSTFIX;	// postfix
			} else {
				// either two operators in a row or right hand side not parsed so we don't really know what is right (same is true above)
				// since there is nothing good to return, assume right is an operand after parsing (thus infix case)
				return OperatorTypes::INFIX;
			}
		}

		fn find_operator_info(op_info: &OperatorInfo, op_type: OperatorTypes, from_form_attr: bool) -> &OperatorInfo {
			if op_info.is_operator_type(op_type) {
				return op_info;
			} else if let Some(next_op_info) = op_info.next {
				if next_op_info.is_operator_type(op_type) {
					return next_op_info;
				} else if let Some(last_op_info) = next_op_info.next {
					if last_op_info.is_operator_type(op_type) {
						return last_op_info;
					}
				}
			}

			// didn't find op_info that matches -- if type is not forced, then return first value (any is probably ok) 
			return if from_form_attr {*ILLEGAL_OPERATOR_INFO} else {op_info};
		}
	
		fn op_not_in_operator_dictionary(op_type: OperatorTypes) -> &'static OperatorInfo {
			return match op_type {
				OperatorTypes::PREFIX => *DEFAULT_OPERATOR_INFO_PREFIX,
				OperatorTypes::POSTFIX => *DEFAULT_OPERATOR_INFO_POSTFIX,
				_ => *DEFAULT_OPERATOR_INFO_INFIX,	// should only be infix
			};
		}
	}
	
	fn n_vertical_bars_on_right<'a>(&self, remaining_children: &[ChildOfElement], vert_bar_ch: &'a str) -> usize {
		// return the number of children that match 'vert_bar_op' not counting the first element
		let mut n = 0;
		for child_of_element in remaining_children {
			let child = as_element(*child_of_element);
			if name(&child) == "mo" {
				let operator_str = as_text(child);
				if operator_str == vert_bar_ch {
					n += 1;
				}
			}
		}
		return n;
	}
	
	
	fn determine_vertical_bar_op<'a>(&self, original_op: &'static OperatorInfo, mo_node: Element<'a>, 
				next_child: Option<Element<'a>>,
				parse_stack: &'a mut Vec<StackInfo>,
				n_vertical_bars_on_right: usize) -> &'static OperatorInfo {
		// if in a prefix location, it is a left fence
		// note:  if there is an operator on the top of the stack, it wants an operand (otherwise it would have been reduced)
		let operator_str = as_text(mo_node);
		let found_op_info = OPERATORS.get(operator_str);
		if found_op_info.is_none() {
			return original_op;
		}
		let op = found_op_info.unwrap();
		if !AMBIGUOUS_OPERATORS.contains(operator_str) {
			// println!("   op is not ambiguous");
			return original_op;
		};
	
		let operator_versions = OperatorVersions::new(op);
		if operator_versions.prefix.is_some() &&
		   top(&parse_stack).last_child_in_mrow().is_none() || !top(&parse_stack).is_operand {
			// println!("   is prefix");
			return operator_versions.prefix.unwrap();
		}
		
		// We have either a right fence or an infix operand at the top of the stack
		// If this is already parsed, we'd look to the right to see if there is an operand after this child.
		// But it isn't parsed and there might be a prefix operator which will eventually become an operand, so it is tricky.
		// It is even trickier because we might have an implicit times, so we can't really tell
		// For example:  |x|y|z| which can be '|x| y |z|' or '|x |y| z|', or even | (x|y)|z |'
		// We can't really know what is intended (without @intent).
		// It seems like the case where it could be paired with a matching vertical bar as what most people would choose, so we favor that.
	
		// If there is a matching open vertical bar, it is either at the top of the stack or the entry just below the top
		let mut has_left_match = ptr_eq(top(&parse_stack).op_pair.op, operator_versions.prefix.unwrap());						// match at top of stack? (empty matching bars)
		if !has_left_match && parse_stack.len() > 2 {
			// matching op is below top (operand between matching bars) -- pop, peek, push
			let old_top = parse_stack.pop().unwrap();																			// can only access top, so we need to pop off top and push back later
			has_left_match = ptr_eq(top(&parse_stack).op_pair.op, operator_versions.prefix.unwrap());
			parse_stack.push(old_top);
		}
	
		if operator_versions.postfix.is_some() && (next_child.is_none() || has_left_match) {
			// last child in row (must be a close) or we have a left match
			// println!("   is postfix");
			return operator_versions.postfix.unwrap();
		} else if next_child.is_none() {
			// operand on left, so prefer infix version
			return if operator_versions.infix.is_none() {op} else {operator_versions.infix.unwrap()};
		}
	
		let next_child = next_child.unwrap();
		if operator_versions.prefix.is_some() && (n_vertical_bars_on_right & 0x1 != 0) {
			// 	("   is prefix");
			return operator_versions.prefix.unwrap();		// odd number of vertical bars remain, so consider this the start of a pair
		}
	
		let next_child = get_possible_embellished_node(next_child);
		let next_child_op = if name(&next_child) != "mo" {
				None
			} else {
				let next_next_children = next_child.following_siblings();
				let next_next_child = if next_next_children.is_empty() { None } else { Some( as_element(next_next_children[0]) )};
				Some( self.find_operator(next_child, operator_versions.infix,
									top(&parse_stack).last_child_in_mrow(), next_next_child) )
			};
												  
		// If the next child is a prefix op or a left fence, it will reduce to an operand, so don't consider it an operator
		if next_child_op.is_some() && !next_child_op.unwrap().is_left_fence() && !next_child_op.unwrap().is_prefix() {
			if operator_versions.postfix.is_some() {
				// println!("   is postfix");
				return operator_versions.postfix.unwrap();	
			}
		} else if operator_versions.infix.is_some() {
			// println!("   is infix");
			return operator_versions.infix.unwrap();	
		}
	
		// nothing good to match
		return op;
	}


	// return true if 'node' is a chemical element and is followed by a state (solid, liquid, ...)
	fn is_likely_chemical_state<'a>(&self, node: Element<'a>, right_siblings:&[ChildOfElement]) -> bool {
		assert_eq!(name(&node.parent().unwrap().element().unwrap()), "mrow"); // should be here because we are parsing an mrow
	
		// println!("   is_likely_chemical_state: '{}'?",element_summary(node));
	
		
		// right side hasn't been parsed, so two cases to look at
		let next_sibling = as_element(right_siblings[0]);
		if name(&next_sibling) == "mrow" {
			return self.is_likely_chemical_state(node, next_sibling.children().as_slice());
		}
	
		if right_siblings.is_empty() {
			return false;
		}
	
		if right_siblings.len() < 3 {  // need at least '(' state ')
			return false;
		}
		// println!("    ....have enough siblings");
	
		if !is_chemical_element(node) {
			return false;
		}
		// println!("    ....found chemical element");
	
		let left_paren = as_element(right_siblings[0]);
		if name(&left_paren) != "mo" {
			return false;
		}
	
		// take care of special case of bad MathML for "aq" (split across two tokens)
		if right_siblings.len() > 3 {
			// check to make sure right kind of leaves then check the contents
			let a = as_element(right_siblings[1]);
			let q = as_element(right_siblings[2]);
			let right_paren = as_element(right_siblings[3]);
			if name(&a) == "mi" && as_text(a)== "a" && 
			   name(&q) == "mi" && as_text(q) == "q" &&
			   name(&right_paren) == "mo" {
				let left_paren = as_text(left_paren);
				let right_paren = as_text(right_paren);
				// since we matched 'a' and 'q' -- either is or isn't chem state
				return (left_paren == "(" && right_paren == ")") || (left_paren == "[" && right_paren == "]");
			}
		}
	
		let right_paren = as_element(right_siblings[2]);
		if name(&right_paren) != "mo" {
			return false;
		}
	
		if !( (as_text(left_paren) == "(" && as_text(right_paren) == ")") ||
			  (as_text(left_paren)== "[" && as_text(right_paren) == "]") ) {
			return false;
		}
	
		// have (xxx) or [xxx] -- check for "s, "l", "g", "aq"
		let state_node = as_element(right_siblings[1]);
		if name(&state_node) != "mi" {
			return false;
		}
		let state = as_text(state_node);
		if state == "s" || state == "l" || state == "g" || state == "aq" {
			return true;
		}
	
		// wasn't one of the cases that make it a chemical state
		return false;
	}
	
	// Try to figure out whether an <mi> is a function name or note.
	// There are two important cases depending upon whether parens/brackets are used or not.
	// E.g, sin x and f(x)
	// 1. If parens follow the name, then we use a more inclusive set of heuristics as it is more likely a function
	// The heuristics used are:
	//   - it is on the list of known function names (e.g., sin" and "log")
	//   - it is on the list of likely function names (e.g, f, g, h)
	//   - multi-char names that begin with a capital letter (e.g, "Tr")
	//   - there is a single token inside the parens (why else would someone use parens), any name (e.g, a(x))
	//
	// 2. If there are no parens, then only names on the known function list are used (e.g., "sin x") 
	fn is_function_name<'a>(&self, node: Element<'a>, right_siblings: Option<&[ChildOfElement<'a>]>) -> bool {
		let base_of_name = get_possible_embellished_node(node);
	
		// actually only 'mi' should be legal here, but some systems used 'mtext' for multi-char variables
		// FIX: need to allow for composition of function names. E.g, (f+g)(x) and (f^2/g)'(x)
		let node_name = name(&base_of_name);
		if node_name != "mi" && node_name != "mtext" {
			return false;
		}
	
		let node_str = as_text(base_of_name);
		if node_str.is_empty() {
			return false;
		}
		// println!("    is_function_name({}), {} following nodes", node_str, if right_siblings.is_none() {"No".to_string()} else {right_siblings.unwrap().len().to_string()});
		return crate::definitions::DEFINITIONS.with(|defs| {
			// names that are always function names (e.g, "sin" and "log")
			let names = defs.function_names.as_hashset().borrow();
			if names.contains(node_str) {
				return true;	// always treated as function names
			}
	
			if right_siblings.is_none() {
				return false;	// only accept known names, which is tested above
			}

			// make sure that what follows starts and ends with parens/brackets
			assert_eq!(name(&node.parent().unwrap().element().unwrap()), "mrow");
			let right_siblings = right_siblings.unwrap();
			if right_siblings.is_empty() {
				return false;
			}

			let first_child = as_element(right_siblings[0]);
			if name(&first_child) == "mrow" && is_left_paren(as_element(first_child.children()[0])) {
				return self.is_function_name(node, Some(&first_child.children()));
			}

			if right_siblings.len() < 2 {
				return false;	// can't be (...)
			}

			// at least two siblings are this point -- check that they are parens/brackets
			// we can only check the open paren/bracket because the right side is unparsed and we don't know the close location
			let first_sibling = as_element(right_siblings[0]);
			if name(&first_sibling) != "mo"  || !is_left_paren(first_sibling)  // '(' or '['
			{
				return false;
			}
	
			if self.is_likely_chemical_state(node, right_siblings) {
				// println!("    is_likely_chemical_state=true");
				return true;
			}
	
			let likely_names = defs.likely_function_names.as_hashset().borrow();
			if likely_names.contains(node_str) {
				return true;	// don't bother checking contents of parens, consider these as function names
			}
	
			if is_single_arg(right_siblings) {
				return true;	// if there is only a single arg, why else would you use parens?
			};
	
			// Names like "Tr" are likely function names, single letter names like "M" or "J" are iffy
			// This needs to be after the chemical state check above to rule out Cl(g), etc
			// This would be better if if were part of 'likely_names' as "[A-Za-z]+", but reg exprs don't work in HashSets.
			// FIX: create our own struct and write appropriate traits for it and then it could work
			let mut chars = node_str.chars();
			let first_char = chars.next().unwrap();		// we know there is at least one byte in it, hence one char
			if chars.next().is_some() && first_char.is_uppercase() {
				return true;
			}
	
			return false;		// didn't fit one of the above categories
		});
	
		fn is_single_arg<'a>(following_nodes: &[ChildOfElement<'a>]) -> bool {
			if following_nodes.len() == 1 {
				return true;		// "a(" might or might not be a function call -- treat as "is" because we can't see more 
			}
	
			let next_child = as_element(following_nodes[1]);
			if is_right_paren(next_child) {
				return true;		// no-arg case "a()"
			}
	
			// could be really picky and restrict to checking for only mi/mn
			// that might make more sense in stranger cases, but mfrac, msqrt, etc., probably shouldn't have parens if times 
			return following_nodes.len() > 2 && 
					name(&next_child) != "mrow" &&
					is_right_paren(as_element(following_nodes[2]));
		}
	
		fn is_left_paren(node: Element) -> bool {
			if name(&node) != "mo" {
				return false;
			}
			let text = as_text(node);
			return text == "(" || text == "[";
		}
	
		fn is_right_paren(node: Element) -> bool {
			if name(&node) != "mo" {
				return false;
			}
			let text = as_text(node);
			return text == ")" || text == "]";
		}
	}
	
	fn is_mixed_fraction<'a>(&self, integer_part: &'a Element<'a>, fraction_part: &'a Element<'a>) -> bool {
		if name(fraction_part) != "mfrac" {
			return false;
		}
	
		// integer part must be either 'n' or '-n' (in an mrow)
		let integer_part_name = name(integer_part);
		if integer_part_name == "mrow" {
			let children = integer_part.children();
			if children.len() == 2 &&
				name(&as_element(children[0])) == "mo" &&
				as_text(as_element(children[0])) == "-" {
					let integer_part = as_element(children[1]);
					if name(&integer_part) != "mn"  || as_text(integer_part).contains(DECIMAL_SEPARATOR) {
						return false;
					}
			}
		};
	
		if name(integer_part) != "mn"  || as_text(*integer_part).contains(DECIMAL_SEPARATOR) {
			return false;
		}
	
		// fraction_part needs to have integer numerator and denominator (already tested it is a frac)
		let fraction_children = fraction_part.children();
		if fraction_children.len() != 2 {
			return false;
		}
		let numerator = as_element(fraction_children[0]);
		if name(&numerator) != "mn" || as_text(numerator).contains(DECIMAL_SEPARATOR) {
			return false;
		}
		let denominator = as_element(fraction_children[1]);
		if name(&denominator) != "mn" || as_text(denominator).contains(DECIMAL_SEPARATOR) {
			return false;
		}
	
		return true;		// the only thing left is a mixed fraction
	}

	// implied comma when two numbers are adjacent and are in a script position
	fn is_implied_comma<'a>(&self, prev: &'a Element<'a>, current: &'a Element<'a>) -> bool {
		if name(prev) != "mn" || name(current) != "mn" {
			return false;
		}

		let mrow = current.parent().unwrap().element().unwrap();
		assert_eq!(name(&mrow), "mrow");
		let container = mrow.parent().unwrap().element().unwrap();
		let name = name(&container);

		// test for script position is that it is not the base and hence has a preceding sibling
		return (name == "msub" || name == "msubsup" || name == "msup") && !mrow.preceding_siblings().is_empty();
	}
	
	// Add the current operator if it's not n-ary to the stack
	// 'current_child' and it the operator to the stack.
	fn shift_stack<'s, 'a:'s, 'op:'a>(
				&self, parse_stack: &'s mut Vec<StackInfo<'a, 'op>>,
				current_child: Element<'a>, 
				current_op: OperatorPair<'op>) -> (Element<'a>, OperatorPair<'op>) {
		let mut new_current_child = current_child;
		let mut new_current_op = current_op.clone();
		let previous_op = top(&parse_stack).op_pair.clone();
		// println!(" shift_stack: mrow len={}", top(parse_stack).mrow.children().len().to_string());
		// println!(" shift_stack: shift on '{}'; ops: prev '{}/{}', cur '{}/{}'",
		//		element_summary(current_child),show_invisible_op_char(previous_op.ch), previous_op.op.priority,
		//		show_invisible_op_char(current_op.ch), current_op.op.priority);
		if !previous_op.op.is_nary(current_op.op) {
			// grab operand on top of stack (if there is one) and make it part of the new mrow since current op has higher precedence
			// if operators are the same and are binary, then this push makes them act as left associative
			let mut top_of_stack = parse_stack.pop().unwrap();
			if top_of_stack.mrow.children().is_empty() || (!top_of_stack.is_operand && !current_op.op.is_right_fence()) {
				// "bad" syntax - no operand on left -- don't grab operand (there is none)
				//   just start a new mrow beginning with operator
				// FIX -- check this shouldn't happen:  parse_stack.push(top_of_stack);
				parse_stack.push( StackInfo::new(current_child.document()) );
			} else if current_op.op.is_right_fence() {
				// likely, but not necessarily, there is a left fence to start the mrow
				// this is like the postfix case except we grab the entire mrow, push on the close, and make that the mrow
				// note:  the code does these operations on the stack for consistency, but it could be optimized without push/popping the stack
				let mrow = top_of_stack.mrow;
				top_of_stack.add_child_to_mrow(current_child, current_op);
				// println!("shift_stack: after adding right fence to mrow: {}", mml_to_string(&top_of_stack.mrow));
				new_current_op = OperatorPair::new();							// treat matched brackets as operand
				new_current_child = mrow;	
				let children = mrow.children();
				if  children.len() == 2 &&
					( name(&as_element(children[0])) != "mo" ||
					  !self.find_operator(as_element(children[0]),
								   None, Some(as_element(children[0])), Some(mrow) ).is_left_fence()) {
					// the mrow did *not* start with an open (hence no push)
					// since parser really wants balanced parens to keep stack state right, we do a push here
					parse_stack.push( StackInfo::new(mrow.document()) );
				} else if children.len() <= 3 {
					// the mrow started with some open fence (which caused a push) -- add the close, pop, and push on the "operand"
				} else {
					panic!("Wrong number of children in mrow when handling a close fence");
				}
			} else if current_op.op.is_postfix() {
				// grab the left operand and start a new mrow with it and the operator -- put those back on the stack
				// note:  the code does these operations on the stack for consistency, but it could be optimized without push/popping the stack
				let previous_child = top_of_stack.remove_last_operand_from_mrow();					// remove operand from mrow
				parse_stack.push(top_of_stack);
				let mut new_top_of_stack = StackInfo::with_op(&current_child.document(), previous_child, current_op.clone()); // begin new mrow with operand
				new_top_of_stack.add_child_to_mrow(current_child, current_op);	// add on operator
				new_current_child = new_top_of_stack.mrow;								// grab for pushing on old mrow
				new_current_op = OperatorPair::new();								// treat "reduced" postfix operator & operand as an operand
				// println!("shift_stack: after adding postfix to mrow has len: {}", new_current_child.children().len().to_string());
			} else {
				// normal infix op case -- grab the left operand and start a new mrow with it and the operator
				let previous_child = top_of_stack.remove_last_operand_from_mrow();
				parse_stack.push(top_of_stack);
				parse_stack.push( StackInfo::with_op(&current_child.document(),previous_child, current_op) );
			}
		}
		return (new_current_child, new_current_op);
	}
	
	
	fn reduce_stack<'s, 'a:'s, 'op:'a>(&self, parse_stack: &'s mut Vec<StackInfo<'a, 'op>>, current_priority: usize, stop_at_function_call: bool) {
		// stop_at_function_call -- hack to to deal with exceptional parsing for things like "sin -2x" (see comments around call of reduce_stack)
		let mut prev_priority = top(&parse_stack).priority();
		// println!(" reduce_stack: stack len={}, priority: prev={}, cur={}", parse_stack.len(), prev_priority, current_priority);
		while current_priority < prev_priority {					// pop off operators until we are back to the right level
			if stop_at_function_call && ptr_eq(top(&parse_stack).op_pair.op, *INVISIBLE_FUNCTION_APPLICATION) {
				break;
			}
	
			if parse_stack.len() == 1 {
				break;			// something went wrong -- break before popping too much
			}
			let mut top_of_stack = parse_stack.pop().unwrap();
			// println!(" ..popped len={} op:'{}/{}', operand: {}",
			// 		top_of_stack.mrow.children().len(),
			// 		show_invisible_op_char(top_of_stack.op), top_of_stack.op.priority,
			// 		top_of_stack.is_operand);
			let mut mrow = top_of_stack.mrow;
			if mrow.children().len() == 1 {
				// should have added at least operator and operand, but input might not be well-formed
				// in this case, unwrap the mrow and expose the single child for pushing onto stack
				let single_child = top_of_stack.remove_last_operand_from_mrow();
				mrow = single_child;
			}
	
			let mut top_of_stack = parse_stack.pop().unwrap();
			top_of_stack.add_child_to_mrow(mrow, OperatorPair::new());	// mrow on top is "parsed" -- now add it to previous
			prev_priority = top_of_stack.priority();
			parse_stack.push(top_of_stack);
		};
	}
	
	fn is_trig_arg<'a, 'op:'a>(&self, previous_child: Element<'a>, current_child: Element<'a>, parse_stack: &[StackInfo<'a, 'op>]) -> bool {
		// We have operand-operand and know we want multiplication at this point. 
		// Check for special case where we want multiplication to bind more tightly than function app (e.g, sin 2x, sin -2xy)
		// We only want to do this for simple args
		use crate::xpath_functions::IsNode;
		// println!("  is_trig_arg: prev {}, current {}, stack len={}; top len={}",
		//  element_summary(previous_child), element_summary(current_child),
		//  parse_stack.len(), top(parse_stack).mrow.children().len());
		if !IsNode::is_simple(&current_child) {
			return false;
		}
		// This only matters if we are not inside of parens
		if IsBracketed::is_bracketed(&previous_child, "(", ")", false) ||
		   IsBracketed::is_bracketed(&previous_child, "[", "]", false) {
			return false;
		}
	
		// Use lower priority multiplication if current_child is a function (e.g. "cos" in "sin x cos 3y")
		if self.is_function_name(current_child, None) {
			return false;
		}
	
		// Two cases:
		// 1. First operand-operand (e.g, sin 2x, where 'current_child' is 'x') -- top of stack is 'sin' 'apply func' '2'
		// 2. Subsequent operand-operand (e.g, sin 2xy, where 'current_child' is 'y') -- top of stack is '2' 'times' 'x'
		let op_on_top = &top(parse_stack).op_pair;
		return ptr_eq(op_on_top.op, *INVISIBLE_FUNCTION_APPLICATION) || ptr_eq(op_on_top.op, &*IMPLIED_TIMES_HIGH_PRIORITY);
	}
	
	
	/*
		canonicalize_mrows_in_mrow is a simple(ish) operator precedence parser.
		It works by keeping a stack of 'StackInfo':
		'StackInfo' has three parts:
		1. the mrow being build
		2. info about the operator in the mrow being build
		3. bool to say whether the last thing is an operator or an operand
	
		When the op priority increases (eg, have "=" and get "+"), we push on
		1. a new mrow -- if the operator has a left operand, we remove the last node in the mrow and it becomes
		   the first (only so far) child of the new mrow
		2. the operator info
	
		When the op priority decreases, we do the following loop until the this new priority > priority on top of stack
		1. pop the StackInfo
		2. add the StackInfo's mrow  as the last child to the new top of the stack
		We also do this when we hit the end of the mrow (we can treat this case as if we have a negative precedence)
	
		+/- are treated as nary operators and don't push/pop in those cases.
		consecutive operands such as nary times are also considered n-ary operators and don't push/pop in those cases.
	*/
	fn canonicalize_mrows_in_mrow<'a>(&self, mrow: Element<'a>) -> Result<Element<'a>> {
		let saved_mrow_attrs = mrow.attributes();	
		assert_eq!(name(&mrow), "mrow");
		let children = mrow.children();
		// println!("canonicalize_mrows_in_mrow: mrow len={}", children.len());
		if children.len() == 1 {
			return Ok(add_attrs_back(self.canonicalize_mrows(as_element(children[0]))?, saved_mrow_attrs));
		}
	
	
		// FIX: don't touch/canonicalize
		// 1. if intent is given -- anything intent references
		// 2. if the mrow starts or ends with a fence, don't merge into parent (parse children only) -- allows for "]a,b["
		let mut parse_stack = vec![];
		parse_stack.push(StackInfo::new(mrow.document()));
		let mut children = mrow.children();
		let num_children = children.len();
	
		for i_child in 0..num_children {
			// println!("\nDealing with child #{}: {}", i_child, mml_to_string(&as_element(children[i_child])));
			let mut current_child = self.canonicalize_mrows(as_element(children[i_child]))?;
			children[i_child] = ChildOfElement::Element( current_child );
			let base_of_child = get_possible_embellished_node(current_child);

			let mut current_op = OperatorPair::new();
			// figure what the current operator is -- it either comes from the 'mo' (if we have an 'mo') or it is implied
			if name(&base_of_child) == "mo" && !base_of_child.children().is_empty() { // shouldn't have empty mo node, but...
				let previous_op = if top(&parse_stack).is_operand {None} else {Some( top(&parse_stack).op_pair.op )};
				let next_node = if i_child + 1 < num_children {Some(as_element(children[i_child+1]))} else {None};
				current_op = OperatorPair{
					ch: as_text(base_of_child),
					op: &self.find_operator(base_of_child, previous_op,
							top(&parse_stack).last_child_in_mrow(), next_node)
				};
	
				// deal with vertical bars which might be infix, open, or close fences
				// note: mrow shrinks as we iterate through it (removing children from it)
				current_op.op = self.determine_vertical_bar_op(
					current_op.op,
					base_of_child,
					next_node,
					&mut parse_stack,
					self.n_vertical_bars_on_right(&children[i_child+1..], current_op.ch)
				);
			} else if top(&parse_stack).last_child_in_mrow().is_some() {
				let previous_child = top(&parse_stack).last_child_in_mrow().unwrap();
				let base_of_previous_child = get_possible_embellished_node(previous_child);
				if name(&base_of_previous_child) != "mo" {
					// consecutive operands -- add an invisible operator as appropriate
					// note: in Geometry, AB or ABC are common and would be more properly represented with an invisible op
					//    other than times (maybe with a super high priority). Rather than make up a new operator, we stick with times.
					current_op = if self.is_function_name(previous_child, Some(&children[i_child..])) {
								OperatorPair{ ch: "\u{2061}", op: &*INVISIBLE_FUNCTION_APPLICATION }
							} else if self.is_mixed_fraction(&previous_child, &current_child) {
								OperatorPair{ ch: "\u{2064}", op: &*IMPLIED_INVISIBLE_PLUS }
							} else if self.is_implied_comma(&previous_child, &current_child) {
								OperatorPair{ch: "\u{2063}", op: &*IMPLIED_INVISIBLE_COMMA }				  
							} else if self.is_trig_arg(base_of_previous_child, base_of_child, &parse_stack) {
								OperatorPair{ch: "\u{2062}", op: &*IMPLIED_TIMES_HIGH_PRIORITY }				  
							} else {
								OperatorPair{ ch: "\u{2062}", op: &*IMPLIED_TIMES }
							};
	
	
					// println!("  Found implicit op {}/{}", show_invisible_op_char(current_op.ch), current_op.op.priority);
					self.reduce_stack(&mut parse_stack, current_op.op.priority, !self.is_function_name(base_of_child, None));
	
					let implied_mo = create_mo(current_child.document(), current_op.ch);
					let shift_result = self.shift_stack(&mut parse_stack, implied_mo, current_op.clone());
					// ignore shift_result.0 which is just 'implied_mo'
					assert_eq!(implied_mo, shift_result.0);
					assert!( ptr_eq(current_op.op, shift_result.1.op) );
					let mut top_of_stack = parse_stack.pop().unwrap();
					top_of_stack.add_child_to_mrow(implied_mo, current_op);
					parse_stack.push(top_of_stack);
				}
				current_op = OperatorPair::new();
			}
	
			if !ptr_eq(current_op.op, *ILLEGAL_OPERATOR_INFO) {
				if current_op.op.is_left_fence() || current_op.op.is_prefix() {
					if top(&parse_stack).is_operand {
						// will end up with operand operand -- need to choose operator associated with prev child
						// we use the original input here because in this case, we need to look to the right of the ()s to deal with chemical states
						let implied_operator = if self.is_function_name(as_element(children[i_child-1]),
																	Some(&children[i_child..])) {
								OperatorPair{ ch: "\u{2061}", op: &*INVISIBLE_FUNCTION_APPLICATION }
							} else {
								OperatorPair{ ch: "\u{2062}", op: &*IMPLIED_TIMES }
							};
						// println!("  adding implied {}", if ptr_eq(implied_operator.op,*IMPLIED_TIMES) {"times"} else {"function apply"});
	
						let implied_mo = create_mo(current_child.document(), implied_operator.ch);
						let shift_result = self.shift_stack(&mut parse_stack, implied_mo, implied_operator.clone());
						// ignore shift_result.0 which is just 'implied_mo'
						assert_eq!(implied_mo, shift_result.0);
						assert!( ptr_eq(implied_operator.op, shift_result.1.op) );
						let mut top_of_stack = parse_stack.pop().unwrap();
						top_of_stack.add_child_to_mrow(implied_mo, implied_operator);
						parse_stack.push(top_of_stack);
					}
					// starting a new mrow
					parse_stack.push( StackInfo::new(current_child.document()) );
				} else {
					// One of infix, postfix, or right fence -- all should have a left operand
					// pop the stack if it is lower precedence (it forms an mrow)
					self.reduce_stack(&mut parse_stack, current_op.op.priority, false);
					// push new operator on stack (already handled n-ary case)
					let shift_result = self.shift_stack(&mut parse_stack, current_child, current_op);
					current_child = shift_result.0;
					current_op = shift_result.1;
				}
			}
			let mut top_of_stack = parse_stack.pop().unwrap();
			top_of_stack.add_child_to_mrow(current_child, current_op);
			parse_stack.push(top_of_stack);
		}
	
		// Reached the end -- force reduction of what's left on the stack
		self.reduce_stack(&mut parse_stack, LEFT_FENCEPOST.priority, false);
	
		// We essentially have 'terminator( mrow terminator)'
		//   in other words, we have an extra mrow with one child due to the initial start -- remove it
		let mut top_of_stack = parse_stack.pop().unwrap();
		assert_eq!(parse_stack.len(), 0);
	
		let mut parsed_mrow = top_of_stack.mrow;
		assert_eq!( name(&top_of_stack.mrow), "mrow");
		if parsed_mrow.children().len() == 1 {
			parsed_mrow = top_of_stack.remove_last_operand_from_mrow();
			// was synthesized, but is really the original top level mrow
		}
	
		parsed_mrow.remove_attribute(CHANGED_ATTR);
		return Ok( add_attrs_back(parsed_mrow, saved_mrow_attrs) );
	
		fn add_attrs_back<'a>(mrow: Element<'a>, attrs: Vec<Attribute>) -> Element<'a> {
			// println!(   "Adding back {} attr(s)", attrs.len());
			for attr in attrs {
				mrow.set_attribute_value(attr.name(), attr.value());
			}
			return mrow;
		}
	}
}

// ---------------- useful utility functions --------------------
fn top<'s, 'a:'s, 'op:'a>(vec: &'s[StackInfo<'a, 'op>]) -> &'s StackInfo<'a, 'op> {
	return &vec[vec.len()-1];
}


fn name<'a>(node: &'a Element<'a>) -> &str {
	return node.name().local_part();
}

// The child of a non-leaf element must be an element
// Note: can't use references as that results in 'returning use of local variable'
fn as_element(child: ChildOfElement) -> Element {
	return match child {
		ChildOfElement::Element(e) => e,
		_ => panic!("as_element: internal error -- found non-element child"),
	}
}

// The child of a leaf element must be text (previously trimmed)
// Note: trim() combines all the Text children into a single string
fn as_text(leaf_child: Element) -> &str {
	assert!(name(&leaf_child) == "mi" || name(&leaf_child) == "mo" || name(&leaf_child) == "mn" || name(&leaf_child) == "mtext" ||
			name(&leaf_child) == "ms" || name(&leaf_child) == "mspace" || name(&leaf_child) == "mglyph");
	let children = leaf_child.children();
	if children.is_empty() {
		return "";
	}
	assert!(children.len() == 1);
	return match children[0] {
		ChildOfElement::Text(t) => t.text(),
		_ => panic!("as_text: internal error -- found non-text child of leaf element"),	
	}
}

#[allow(dead_code)] // for debugging with println
fn is_leaf(leaf_child: Element) -> bool {
	return  name(&leaf_child) == "mi" || name(&leaf_child) == "mo" || name(&leaf_child) == "mn" || name(&leaf_child) == "mtext" ||
			name(&leaf_child) == "ms" || name(&leaf_child) == "mspace" || name(&leaf_child) == "mglyph";
}

#[allow(dead_code)] // for debugging with println
fn element_summary(mathml: Element) -> String {
	return format!("{}<{}>", name(&mathml), if is_leaf(mathml) {as_text(mathml).to_string()} else {mathml.children().len().to_string()});
}

fn create_mo<'a, 'd:'a>(doc: Document<'d>, ch: &'a str) -> Element<'d> {
	let implied_mo = create_mathml_element(&doc, "mo");
	implied_mo.set_attribute_value(CHANGED_ATTR, ADDED_ATTR_VALUE);
	let mo_text = doc.create_text(ch);
	implied_mo.append_child(mo_text);
	return implied_mo;
}

fn is_adorned_node<'a>(node: &'a Element<'a>) -> bool {
	let name = name(node);
	return	name == "msub" || name == "msup" || name == "msubsup" ||
			name == "munder" || name == "mover" || name == "munderover" ||
			name == "mmultiscripts";
}


fn get_possible_embellished_node(node: Element) -> Element {
	let mut node = node;
	while is_adorned_node(&node) {
		node = as_element(node.children()[0]);
	}
	return node;
}		

#[allow(dead_code)] // for debugging with println
fn show_invisible_op_char(ch: &str) -> &str {
	return match ch.chars().next().unwrap() {
		'\u{2061}' => "&#x2061;",
		'\u{2062}' => "&#x2062;",
		'\u{2063}' => "&#x2063;",
		'\u{2064}' => "&#x2064;",
		'\u{E000}' => "&#xE000;",
		_ 		   => ch
	};
}


static CHEMICAL_ELEMENTS: phf::Set<&str> = phf_set! {
	"Ac", "Ag", "Al", "Am", "Ar", "As", "At", "Au", "B", "Ba", "Be", "Bh", "Bi", "Bk", "Br",
	"C", "Ca", "Cd", "Ce", "Cf", "Cl", "Cm", "Cn", "Co", "Cr", "Cs", "Cu", "Db", "Ds", "Dy", 
	"Er", "Es", "Eu", "F", "Fe", "Fl", "Fm", "Fr", "Ga", "Gd", "Ge",
	"H", "He", "Hf", "Hg", "Ho", "Hs", "I", "In", "Ir", "K", "Kr",
	"La", "Li", "Lr", "Lu", "Lv", "Mc", "Md", "Mg", "Mn", "Mo", "Mt", 
	"N", "Na", "Nb", "Nd", "Ne", "Nh", "Ni", "No", "Np", "O", "Og", "Os", 
	"P", "Pa", "Pb", "Pd", "Pm", "Po", "Pr", "Pt", "Pu",
	"Ra", "Rb", "Re", "Rf", "Rg", "Rh", "Rn", "Ru", 
	"S", "Sb", "Sc", "Se", "Sg", "Si", "Sm", "Sn", "Sr",
	"Ta", "Tb", "Tc", "Te", "Th", "Ti", "Tl", "Tm", "Ts", 
	"U", "V", "W", "Xe", "Y", "Yb", "Zn", "Zr"};

fn is_chemical_element(node: Element) -> bool {
	// FIX: allow name to be in an mrow (e.g., <mi>N</mi><mi>a</mi>
	let name = name(&node);
	if name != "mi" && name != "mtext" {
		return false;
	}

	let text = as_text(node);
	return CHEMICAL_ELEMENTS.contains(text);
}



#[cfg(test)]
mod canonicalize_tests {
    use super::*;
    use sxd_document::parser;


    fn are_strs_canonically_equal(test: &str, target: &str) -> bool {
		use crate::interface::*;
		// this forces initialization
		crate::speech::SPEECH_RULES.with(|_| true);

        let package1 = &parser::parse(test).expect("Failed to parse test input");
		let mathml = get_element(package1);
		trim_element(&mathml);
		let mathml_test = canonicalize(mathml);
        
        let package2 = &parser::parse(target).expect("Failed to parse target input");
		let mathml_target = get_element(package2);
		trim_element(&mathml_target);
            
        return is_same_element(&mathml_test, &mathml_target);
    }

    #[test]
    fn canonical_same() {
        let target_str = "<math><mrow><mo>-</mo><mi>a</mi></mrow></math>";
        assert!(are_strs_canonically_equal(target_str, target_str));
    }

	
	#[test]
    fn plane1() {
        let test_str = "<math>
				<mi mathvariant='italic'>bB4</mi> <mo>,</mo>		<!-- shouldn't change -->
				<mi mathvariant='bold'>a</mi> <mo>,</mo>			<!-- single char id tests -->
				<mi mathvariant='bold'>Z</mi> <mo>,</mo>
				<mn mathvariant='bold'>19=&#x1D7D7;</mn> <mo>,</mo>	<!-- '=' and plane1 shouldn't change -->
				<mn mathvariant='double-struck'>024689</mn> <mo>,</mo>	<!-- '=' and plane1 shouldn't change -->
				<mi mathvariant='double-struck'>yzCHNPQRZ</mi> <mo>,</mo>
				<mi mathvariant='fraktur'>0yACHIRZ</mi> <mo>,</mo>	<!-- 0 stays as ASCII -->
				<mi mathvariant='bold-fraktur'>nC</mi> <mo>,</mo>
				<mi mathvariant='script'>ABEFHILMRegow</mi> <mo>,</mo>
				<mi mathvariant='bold-script'>fG*</mi>				<!-- '*' shouldn't change -->
			</math>";
        let target_str = "<math>
				<mrow data-changed='added'>
					<mi>bB4</mi>
					<mo>,</mo>
					<mi>𝐚</mi>
					<mo>,</mo>
					<mi>𝐙</mi>
					<mo>,</mo>
					<mn>𝟏𝟗=𝟗</mn>
					<mo>,</mo>
					<mn>𝟘𝟚𝟜𝟞𝟠𝟡</mn>
					<mo>,</mo>
					<mi>𝕪𝕫ℂℍℕℙℚℝℤ</mi>
					<mo>,</mo>
					<mi>0𝔶𝔄ℭℌℑℜℨ</mi>
					<mo>,</mo>
					<mi>𝖓𝕮</mi>
					<mo>,</mo>
					<mi>𝒜ℬℰℱℋℐℒℳℛℯℊℴ𝓌</mi>
					<mo>,</mo>
					<mi>𝓯𝓖*</mi>
				</mrow>
			</math>";
assert!(are_strs_canonically_equal(test_str, target_str));
	}

    #[test]
    fn canonical_one_element_mrow_around_mrow() {
        let test_str = "<math><mrow><mrow><mo>-</mo><mi>a</mi></mrow></mrow></math>";
        let target_str = "<math><mrow><mo>-</mo><mi>a</mi></mrow></math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }

    #[test]
    fn canonical_one_element_mrow_around_mo() {
        let test_str = "<math><mrow><mrow><mo>-</mo></mrow><mi>a</mi></mrow></math>";
        let target_str = "<math><mrow><mo>-</mo><mi>a</mi></mrow></math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }

    #[test]
    fn canonical_flat_to_times_and_plus() {
        let test_str = "<math><mi>c</mi><mo>+</mo><mi>x</mi><mi>y</mi></math>";
        let target_str = "<math>
		<mrow data-changed='added'><mi>c</mi><mo>+</mo>
		  <mrow data-changed='added'><mi>x</mi><mo data-changed='added'>&#x2062;</mo><mi>y</mi></mrow>
		</mrow></math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }

    #[test]
    fn canonical_prefix_and_infix() {
        let test_str = "<math><mrow><mo>-</mo><mi>a</mi><mo>-</mo><mi>b</mi></mrow></math>";
        let target_str = "<math>
		<mrow>
		  <mrow data-changed='added'>
			<mo>-</mo>
			<mi>a</mi>
		  </mrow>
		  <mo>-</mo>
		  <mi>b</mi>
		</mrow>
	   </math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }

    #[test]
    fn function_with_single_arg() {
        let test_str = "<math><mrow>
			<mi>sin</mi><mo>(</mo><mi>x</mi><mo>)</mo>
			<mo>+</mo>
			<mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo>
			<mo>+</mo>
			<mi>t</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
		</mrow></math>";
        let target_str = "<math>
		<mrow>
		  <mrow data-changed='added'>
			<mi>sin</mi>
			<mo data-changed='added'>&#x2061;</mo>
			<mrow data-changed='added'>
			  <mo>(</mo>
			  <mi>x</mi>
			  <mo>)</mo>
			</mrow>
		  </mrow>
		  <mo>+</mo>
		  <mrow data-changed='added'>
			<mi>f</mi>
			<mo data-changed='added'>&#x2061;</mo>
			<mrow data-changed='added'>
			  <mo>(</mo>
			  <mi>x</mi>
			  <mo>)</mo>
			</mrow>
		  </mrow>
		  <mo>+</mo>
		  <mrow data-changed='added'>
			<mi>t</mi>
			<mo data-changed='added'>&#x2061;</mo>
			<mrow>
			  <mo>(</mo>
			  <mi>x</mi>
			  <mo>)</mo>
			</mrow>
		  </mrow>
		</mrow>
	   </math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }

    #[test]
    fn function_with_multiple_args() {
        let test_str = "<math>
		<mi>sin</mi><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo>
			<mo>+</mo>
		 <mi>f</mi><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo>
			<mo>+</mo>
		 <mi>t</mi><mrow><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></mrow>
		</math>";
        let target_str = " <math>
		<mrow data-changed='added'>
		<mrow data-changed='added'>
		  <mi>sin</mi>
		  <mo data-changed='added'>&#x2061;</mo>
		  <mrow data-changed='added'>
			<mo>(</mo>
			<mrow data-changed='added'>
			  <mi>x</mi>
			  <mo>+</mo>
			  <mi>y</mi>
			</mrow>
			<mo>)</mo>
		  </mrow>
		</mrow>
		<mo>+</mo>
		<mrow data-changed='added'>
		  <mi>f</mi>
		  <mo data-changed='added'>&#x2061;</mo>
		  <mrow data-changed='added'>
			<mo>(</mo>
			<mrow data-changed='added'>
			  <mi>x</mi>
			  <mo>+</mo>
			  <mi>y</mi>
			</mrow>
			<mo>)</mo>
		  </mrow>
		</mrow>
		<mo>+</mo>
		<mrow data-changed='added'>
		  <mi>t</mi>
		  <mo data-changed='added'>&#x2062;</mo>
		  <mrow>
			<mo>(</mo>
			<mrow data-changed='added'>
			  <mi>x</mi>
			  <mo>+</mo>
			  <mi>y</mi>
			</mrow>
			<mo>)</mo>
			</mrow>
		  </mrow>
		</mrow>
	   </math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }

    #[test]
    fn function_with_no_args() {
        let test_str = "<math><mrow>
		<mi>sin</mi><mi>x</mi>
			<mo>+</mo>
		 <mi>f</mi><mi>x</mi>
			<mo>+</mo>
		 <mi>t</mi><mi>x</mi>
		</mrow></math>";
        let target_str = " <math>
		<mrow>
		  <mrow data-changed='added'>
			<mi>sin</mi>
			<mo data-changed='added'>&#x2061;</mo>
			<mi>x</mi>
		  </mrow>
		  <mo>+</mo>
		  <mrow data-changed='added'>
			<mi>f</mi>
			<mo data-changed='added'>&#x2062;</mo>
			<mi>x</mi>
		  </mrow>
		  <mo>+</mo>
		  <mrow data-changed='added'>
			<mi>t</mi>
			<mo data-changed='added'>&#x2062;</mo>
			<mi>x</mi>
		  </mrow>
		</mrow>
	   </math>";
        assert!(are_strs_canonically_equal(test_str, target_str));

	}


    #[test]
    fn implied_plus() {
        let test_str = "<math><mrow>
    <mn>2</mn><mfrac><mn>3</mn><mn>4</mn></mfrac>
    </mrow></math>";
        let target_str = "<math>
			<mrow>
				<mn>2</mn>
				<mo data-changed='added'>&#x2064;</mo>
				<mfrac>
					<mn>3</mn>
					<mn>4</mn>
				</mfrac>
			</mrow>
		</math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }

    #[test]
    fn implied_comma() {
        let test_str = "<math><msub><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></msub></math>";
        let target_str = "<math>
			 <msub><mi>b</mi><mrow><mn>1</mn><mo data-changed='added'>&#x2063;</mo><mn>2</mn></mrow></msub>
		</math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }

    #[test]
    fn no_implied_comma() {
        let test_str = "<math><mfrac><mi>b</mi><mrow><mn>1</mn><mn>2</mn></mrow></mfrac></math>";
        let target_str = "<math>
			 <mfrac><mi>b</mi><mrow><mn>1</mn><mo data-changed='added'>&#x2062;</mo><mn>2</mn></mrow></mfrac>
		</math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }

    #[test]
    fn vertical_bars() {
        let test_str = "<math>
		<mo>|</mo> <mi>x</mi> <mo>|</mo><mo>+</mo><mo>|</mo>
		 <mi>a</mi><mo>+</mo><mn>1</mn> <mo>|</mo>
	  </math>";
	  let target_str = " <math>
	  <mrow data-changed='added'>
		<mrow data-changed='added'>
		  <mo>|</mo>
		  <mi>x</mi>
		  <mo>|</mo>
		</mrow>
		<mo>+</mo>
		<mrow data-changed='added'>
		  <mo>|</mo>
		  <mrow data-changed='added'>
			<mi>a</mi>
			<mo>+</mo>
			<mn>1</mn>
		  </mrow>
		  <mo>|</mo>
		</mrow>
	  </mrow>
	 </math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }


    #[test]
    fn vertical_bars_nested() {
        let test_str = "<math><mo>|</mo><mi>x</mi><mo>|</mo><mi>y</mi><mo>|</mo><mi>z</mi><mo>|</mo></math>";
	  let target_str = "<math>
	  <mrow data-changed='added'>
		<mrow data-changed='added'>
			<mo>|</mo>
			<mi>x</mi>
			<mo>|</mo>
		</mrow>
		<mo data-changed='added'>&#x2062;</mo>
		<mi>y</mi>
		<mo data-changed='added'>&#x2062;</mo>
		<mrow data-changed='added'>
			<mo>|</mo>
			<mi>z</mi>
			<mo>|</mo>
		</mrow>
	  </mrow>
	 </math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }

    #[test]
    fn vertical_bar_such_that() {
        let test_str = "<math>
            <mo>{</mo> <mrow><mi>x</mi></mrow> <mo>|</mo><mi>a</mi><mo>}</mo>
            </math>";
        let target_str = "<math>
				<mrow data-changed='added'>
					<mo>{</mo>
					<mrow data-changed='added'>
						<mi>x</mi>
						<mo>|</mo>
						<mi>a</mi>
					</mrow>
					<mo>}</mo>
				</mrow>
			</math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }


    #[test]
    fn trig_mo() {
        let test_str = "<math><mo>sin</mo><mi>x</mi>
				<mo>+</mo><mo>cos</mo><mi>y</mi>
				<mo>+</mo><munder><mo>lim</mo><mi>D</mi></munder><mi>y</mi>
			</math>";
        let target_str = "<math>
		<mrow data-changed='added'>
		  <mrow data-changed='added'>
			<mi>sin</mi>
			<mo data-changed='added'>&#x2061;</mo>
			<mi>x</mi>
		  </mrow>
		  <mo>+</mo>
		  <mrow data-changed='added'>
			<mi>cos</mi>
			<mo data-changed='added'>&#x2061;</mo>
			<mi>y</mi>
		  </mrow>
		  <mo>+</mo>
		  <mrow data-changed='added'>
			<munder>
			  <mi>lim</mi>
			  <mi>D</mi>
			</munder>
			<mo data-changed='added'>&#x2061;</mo>
			<mi>y</mi>
		  </mrow>
		</mrow>
	   </math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }
    #[test]
    fn trig_negative_args() {
        let test_str = "<math><mi>sin</mi><mo>-</mo><mn>2</mn><mi>π</mi><mi>x</mi></math>";
        let target_str = "<math>
		<mrow data-changed='added'>
		  <mi>sin</mi>
		  <mo data-changed='added'>&#x2061;</mo>
		  <mrow data-changed='added'>
			<mrow data-changed='added'>
			  <mo>-</mo>
			  <mn>2</mn>
			</mrow>
			<mo data-changed='added'>&#x2062;</mo>
			<mi>π</mi>
			<mo data-changed='added'>&#x2062;</mo>
			<mi>x</mi>
		  </mrow>
		</mrow>
	   </math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }

    #[test]
    fn trig_function_composition() {
        let test_str = "<math><mo>(</mo><mi>sin</mi><mo>-</mo><mi>cos</mi><mo>)</mo><mi>x</mi></math>";
        let target_str = "<math>
		<mrow data-changed='added'>
		  <mrow data-changed='added'>
			<mo>(</mo>
			<mrow data-changed='added'>
			  <mi>sin</mi>
			  <mo>-</mo>
			  <mi>cos</mi>
			</mrow>
			<mo>)</mo>
		  </mrow>
		  <mo data-changed='added'>&#x2062;</mo>
		  <mi>x</mi>
		</mrow>
	   </math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
    }

	
	#[test]
    fn mtext_whitespace_string() {
        let test_str = "<math><mi>t</mi><mtext>&#x00A0;&#x205F;</mtext></math>";
        let target_str = "<math><mi>t</mi></math>";
		assert!(are_strs_canonically_equal(test_str, target_str));
	}
	
	#[test]
    fn remove_mtext_whitespace_1() {
        let test_str = "<math><mi>t</mi><mtext>&#x00A0;&#x205F;</mtext>
				<mrow><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></mrow></math>";
        let target_str = " <math>
		<mrow data-changed='added'>
		  <mi>t</mi>
		  <mo data-changed='added'>&#x2062;</mo>
		  <mrow>
			<mo>(</mo>
			<mrow data-changed='added'>
			  <mi>x</mi>
			  <mo>+</mo>
			  <mi>y</mi>
			</mrow>
			<mo>)</mo>
		  </mrow>
		</mrow>
	   </math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
	}

	#[test]
    fn remove_mtext_whitespace_2() {
        let test_str = "<math><mi>t</mi>
				<mrow><mtext>&#x2009;</mtext><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></mrow></math>";
        let target_str = " <math>
		<mrow data-changed='added'>
		  <mi>t</mi>
		  <mo data-changed='added'>&#x2062;</mo>
		  <mrow>
			<mo>(</mo>
			<mrow data-changed='added'>
			  <mi>x</mi>
			  <mo>+</mo>
			  <mi>y</mi>
			</mrow>
			<mo>)</mo>
		  </mrow>
		</mrow>
	   </math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
	}

	#[test]
    fn do_not_remove_all_whitespace() {
        let test_str = "<math><mfrac>
					<mrow><mspace width='3em'/></mrow>
					<mtext>&#x2009;</mtext>
				</mfrac></math>";
        let target_str = "<math><mfrac>
				<mspace width='3em'/>
				<mtext>&#x2009;</mtext>
			</mfrac></math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
	}

	#[test]
    fn do_not_remove_some_whitespace() {
        let test_str = "<math><mroot>
					<mrow><mi>b</mi><mphantom><mi>y</mi></mphantom></mrow>
					<mtext>&#x2009;</mtext>
				</mroot></math>";
        let target_str = "<math><mroot>
				<mi>b</mi>
				<mtext>&#x2009;</mtext>
			</mroot></math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
	}

	#[test]
    fn remove_all_extra_elements() {
        let test_str = "<math><msqrt>
					<mstyle> <mi>b</mi> </mstyle>
					<mphantom><mi>y</mi></mphantom>
					<mtext>&#x2009;</mtext>
					<mspace width='3em'/>
				</msqrt></math>";
        let target_str = "<math><msqrt>
				<mi>b</mi>
			</msqrt></math>";
        assert!(are_strs_canonically_equal(test_str, target_str));
	}
}

