//! Useful functions for debugging and error messages.
#![allow(clippy::needless_return)]

use sxd_document::dom::*;

// #[allow(dead_code)]
// pub fn pp_doc(doc: &Document) {
//     for root_child in doc.root().children() {
//         if let ChildOfRoot::Element(e) = root_child {
//             format_element(&e, 0);
//             break;
//         }
//     };
// }

/// Pretty-print the MathML represented by `element`.
pub fn mml_to_string(e: Element) -> String {
    return format_element(e, 0);
}

/// Pretty-print the MathML represented by `element`.
/// * `indent` -- the amount of indentation to start with
pub fn format_element(e: Element, indent: usize) -> String {
    // let namespace = match e.name().namespace_uri() {
    //     None => "".to_string(),
    //     Some(prefix) => prefix.to_string() + ":",
    // };
    // let namespace = namespace.as_str();
    let namespace = "";
    let mut answer = format!("{:in$}<{ns}{name}{attrs}>", " ", in=2*indent, ns=namespace, name=e.name().local_part(), attrs=format_attrs(&e.attributes()));
    let children = e.children();
    let has_element = children.iter().find(|&&c| matches!(c, ChildOfElement::Element(_x)));
    if has_element.is_none() {
        // print text content
        let content = children.iter()
                .map(|c| if let ChildOfElement::Text(t) = c {t.text()} else {""})
                .collect::<Vec<&str>>()
                .join("");
        return format!("{}{}</{}{}>\n", answer, &handle_special_chars(&content), namespace, e.name().local_part());
        // for child in children {
        //     if let ChildOfElement::Text(t) = child {
        //         return format!("{}{}</{}{}>\n", answer, &make_invisible_chars_visible(t.text()), namespace, e.name().local_part());
        //     }
        // };
    } else {
       answer += "\n";        // tag with children should start on new line
        // recurse on each Element child
        for c in e.children() {
            if let ChildOfElement::Element(e) = c {
                answer += &format_element(e, indent+1);
            }
        }
    }
    return answer + &format!("{:in$}</{ns}{name}>\n", " ", in=2*indent, ns=namespace, name=e.name().local_part());

    // Use the &#x....; representation for invisible chars when printing
}

/// Format a vector of attributes as a string with a leading space
pub fn format_attrs(attrs: &[Attribute]) -> String {
    let mut result = String::new();
    for attr in attrs {
        result += format!(" {}='{}'", attr.name().local_part(), &handle_special_chars(attr.value())).as_str();
    }
    result
}

fn handle_special_chars(text: &str) -> String {
    return text.chars().map(|ch|
        match ch {
            '"' => "&quot;".to_string(),
            '&' => "&amp;".to_string(),
            '\'' => "&apos;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '\u{2061}' => "&#x2061;".to_string(),
            '\u{2062}' => "&#x2062;".to_string(),
            '\u{2063}' => "&#x2063;".to_string(),
            '\u{2064}' => "&#x2064;".to_string(),
            _ => ch.to_string(),
        }
    ).collect::<Vec<String>>().join("");
}


// /// Pretty print an xpath value.
// /// If the value is a `NodeSet`, the MathML for the node/element is returned.
// pub fn pp_xpath_value(value: Value) {
//     use sxd_xpath::Value;
//     use sxd_xpath::nodeset::Node;
//     debug!("XPath value:");
//     if let Value::Nodeset(nodeset) = &value {
//         for node in nodeset.document_order() {
//             match node {
//                 Node::Element(el) => {debug!("{}", crate::pretty_print::format_element(&el, 1))},
//                 Node::Text(t) =>  {debug!("found Text value: {}", t.text())},
//                 _ => {debug!("found unexpected node type")}
//             }
//         }
//     }
// }

/// Convert YAML to a string using with `indent` amount of space.
pub fn yaml_to_string(yaml: &Yaml, indent: usize) -> String {
    let mut result = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut result);
        emitter.compact(true);
        emitter.emit_node(yaml).unwrap(); // dump the YAML object to a String
    }
    if indent == 0 {
        return result;
    }
    let indent_str = format!("{:in$}", " ", in=2*indent);
    result = result.replace('\n',&("\n".to_string() + &indent_str)); // add indentation to all but first line
    return indent_str + result.trim_end();  // add indent to first line and remove an extra indent at end
}

/* --------------------- Tweaked pretty printer for YAML (from YAML code) --------------------- */

// Changed: new function to determine if more compact notation can be used (when child is a one entry simple array/hash). Writes
// -foo [bar: bletch]
// -foo {bar: bletch}
fn is_scalar(v: &Yaml) -> bool {
    return !matches!(v, Yaml::Hash(_) | Yaml::Array(_));
}

fn is_complex(v: &Yaml) -> bool {
    return match v {
        Yaml::Hash(h) => {
            return match h.len() {
                0 => false,
                1 => {
                    let (key,val) = h.iter().next().unwrap();
                    return !(is_scalar(key) && is_scalar(val))
                },
                _ => true,
            }
        },
        Yaml::Array(v) => {
            return match v.len() {
                0 => false,
                1 => {
                    let hash = v[0].as_hash();
                    if let Some(hash) = hash {
                        return match hash.len() {
                            0 => false,
                            1 => {
                                let (key, val) = hash.iter().next().unwrap();
                                return !(is_scalar(key) && is_scalar(val));
                            },
                            _ => true,
                        }
                    } else {
                        return !is_scalar(&v[0]);
                    }    
                },
                _ => true,
            }
        },
        _ => false,
    }
}

use std::error::Error;
use std::fmt::{self, Display};
extern crate yaml_rust;
use yaml_rust::{Yaml, yaml::Hash};

//use crate::yaml::{Hash, Yaml};

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)] // from original YAML code (isn't used here)
enum EmitError {
    FmtError(fmt::Error),
    BadHashmapKey,
}

impl Error for EmitError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl Display for EmitError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EmitError::FmtError(ref err) => Display::fmt(err, formatter),
            EmitError::BadHashmapKey => formatter.write_str("bad hashmap key"),
        }
    }
}

impl From<fmt::Error> for EmitError {
    fn from(f: fmt::Error) -> Self {
        EmitError::FmtError(f)
    }
}

struct YamlEmitter<'a> {
    writer: &'a mut dyn fmt::Write,
    best_indent: usize,
    compact: bool,

    level: isize,
}

type EmitResult = Result<(), EmitError>;

// from serialize::json
fn escape_str(wr: &mut dyn fmt::Write, v: &str) -> Result<(), fmt::Error> {
    wr.write_str("\"")?;

    let mut start = 0;

    for (i, byte) in v.bytes().enumerate() {
        let escaped = match byte {
            b'"' => "\\\"",
            b'\\' => "\\\\",
            b'\x00' => "\\u0000",
            b'\x01' => "\\u0001",
            b'\x02' => "\\u0002",
            b'\x03' => "\\u0003",
            b'\x04' => "\\u0004",
            b'\x05' => "\\u0005",
            b'\x06' => "\\u0006",
            b'\x07' => "\\u0007",
            b'\x08' => "\\b",
            b'\t' => "\\t",
            b'\n' => "\\n",
            b'\x0b' => "\\u000b",
            b'\x0c' => "\\f",
            b'\r' => "\\r",
            b'\x0e' => "\\u000e",
            b'\x0f' => "\\u000f",
            b'\x10' => "\\u0010",
            b'\x11' => "\\u0011",
            b'\x12' => "\\u0012",
            b'\x13' => "\\u0013",
            b'\x14' => "\\u0014",
            b'\x15' => "\\u0015",
            b'\x16' => "\\u0016",
            b'\x17' => "\\u0017",
            b'\x18' => "\\u0018",
            b'\x19' => "\\u0019",
            b'\x1a' => "\\u001a",
            b'\x1b' => "\\u001b",
            b'\x1c' => "\\u001c",
            b'\x1d' => "\\u001d",
            b'\x1e' => "\\u001e",
            b'\x1f' => "\\u001f",
            b'\x7f' => "\\u007f",
            _ => continue,
        };

        if start < i {
            wr.write_str(&v[start..i])?;
        }

        wr.write_str(escaped)?;

        start = i + 1;
    }

    if start != v.len() {
        wr.write_str(&v[start..])?;
    }

    wr.write_str("\"")?;
    Ok(())
}

impl<'a> YamlEmitter<'a> {
    pub fn new(writer: &'a mut dyn fmt::Write) -> YamlEmitter<'a> {
        YamlEmitter {
            writer,
            best_indent: 2,
            compact: true,
            level: -1,
        }
    }

    /// Set 'compact inline notation' on or off, as described for block
    /// [sequences](http://www.yaml.org/spec/1.2/spec.html#id2797382)
    /// and
    /// [mappings](http://www.yaml.org/spec/1.2/spec.html#id2798057).
    ///
    /// In this form, blocks cannot have any properties (such as anchors
    /// or tags), which should be OK, because this emitter doesn't
    /// (currently) emit those anyways.
    pub fn compact(&mut self, compact: bool) {
        self.compact = compact;
    }

    /// Determine if this emitter is using 'compact inline notation'.
    #[allow(dead_code)]   // not all fields are used in this program
    pub fn is_compact(&self) -> bool {
        self.compact
    }

    // fn dump(&mut self, doc: &Yaml) -> EmitResult {
    //     // write DocumentStart
    //     writeln!(self.writer, "---")?;
    //     self.level = -1;
    //     self.emit_node(doc)
    // }

    fn write_indent(&mut self) -> EmitResult {
        if self.level <= 0 {
            return Ok(());
        }
        for _ in 0..self.level {
            for _ in 0..self.best_indent {
                write!(self.writer, " ")?;
            }
        }
        Ok(())
    }

    fn emit_node(&mut self, node: &Yaml) -> EmitResult {
        match *node {
            Yaml::Array(ref v) => self.emit_array(v),
            Yaml::Hash(ref h) => self.emit_hash(h),
            Yaml::String(ref v) => {
                if need_quotes(v) {
                    escape_str(self.writer, v)?;
                } else {
                    write!(self.writer, "{v}")?;
                }
                Ok(())
            }
            Yaml::Boolean(v) => {
                if v {
                    self.writer.write_str("true")?;
                } else {
                    self.writer.write_str("false")?;
                }
                Ok(())
            }
            Yaml::Integer(v) => {
                write!(self.writer, "{v}")?;
                Ok(())
            }
            Yaml::Real(ref v) => {
                write!(self.writer, "{v}")?;
                Ok(())
            }
            Yaml::Null | Yaml::BadValue => {
                write!(self.writer, "~")?;
                Ok(())
            }
            // XXX(chenyh) Alias
            _ => Ok(()),
        }
    }

    fn emit_array(&mut self, v: &[Yaml]) -> EmitResult {
        if v.is_empty() {
            write!(self.writer, "[]")?;
        } else if v.len() == 1 && !is_complex(&v[0]) {
            // changed -- for arrays that have only one simple element, make them more compact by using [...] notation
            write!(self.writer, "[")?;
            self.emit_val(true, &v[0])?;
            write!(self.writer, "]")?;
        } else {
            self.level += 1;
            
            for (cnt, x) in v.iter().enumerate() {
                if cnt > 0 {
                    writeln!(self.writer)?;
                    self.write_indent()?;
                }
                write!(self.writer, "- ")?;
                self.emit_val(true, x)?;
            }
            self.level -= 1;
        }
        return Ok(());
    }

    fn emit_hash(&mut self, h: &Hash) -> EmitResult {
        if h.is_empty() {
            self.writer.write_str("{}")?;
        } else {
          // changed -- for hashmaps that have only one simple element, make them more compact by using {...}} notation
            self.level += 1;
            for (cnt, (k, v)) in h.iter().enumerate() {
                // changed: use new function is_scalar()
                // let complex_key = match *k {
                //     Yaml::Hash(_) | Yaml::Array(_) => true,
                //     _ => false,
                // };
                if cnt > 0 {
                    writeln!(self.writer)?;
                    self.write_indent()?;
                }
                if !is_scalar(k) {
                    write!(self.writer, "? ")?;
                    self.emit_val(true, k)?;
                    writeln!(self.writer)?;
                    self.write_indent()?;
                    write!(self.writer, ": ")?;
                    self.emit_val(true, v)?;
                } else {
                    self.emit_node(k)?;
                    write!(self.writer, ": ")?;

                    // changed to use braces in some cases
                    let complex_value = is_complex(v);
                    if !complex_value && v.as_hash().is_some() {
                        write!(self.writer, "{{")?;
                    }
                    // changed to use complex_value from 'false'
                    self.emit_val(!complex_value, v)?;
                    if !complex_value && v.as_hash().is_some() {
                        write!(self.writer, "}}")?;
                    }
                }
            }
            self.level -= 1;
        }   
        Ok(())
    }

    /// Emit a yaml as a hash or array value: i.e., which should appear
    /// following a ":" or "-", either after a space, or on a new line.
    /// If `inline` is true, then the preceding characters are distinct
    /// and short enough to respect the compact flag.
    // changed: use to always emit ' ' for inline -- that is now handled elsewhere
    fn emit_val(&mut self, inline: bool, val: &Yaml) -> EmitResult {
        match *val {
            Yaml::Array(ref v) => {
                if !((inline && self.compact) || v.is_empty()) {
                    writeln!(self.writer)?;
                    self.level += 1;
                    self.write_indent()?;
                    self.level -= 1;
                }
                self.emit_array(v)
            }
            Yaml::Hash(ref h) => {
                if !((inline && self.compact) || h.is_empty()) {
                    writeln!(self.writer)?;
                    self.level += 1;
                    self.write_indent()?;
                    self.level -= 1;
                }
                self.emit_hash(h)
            }
            _ => {
           //     write!(self.writer, " ")?;
                self.emit_node(val)
            }
        }
    }
}

/// Check if the string requires quoting.
/// Strings starting with any of the following characters must be quoted.
/// :, &, *, ?, |, -, <, >, =, !, %, @
/// Strings containing any of the following characters must be quoted.
/// {, }, [, ], ,, #, `
///
/// If the string contains any of the following control characters, it must be escaped with double quotes:
/// \0, \x01, \x02, \x03, \x04, \x05, \x06, \a, \b, \t, \n, \v, \f, \r, \x0e, \x0f, \x10, \x11, \x12, \x13, \x14, \x15, \x16, \x17, \x18, \x19, \x1a, \e, \x1c, \x1d, \x1e, \x1f, \N, \_, \L, \P
///
/// Finally, there are other cases when the strings must be quoted, no matter if you're using single or double quotes:
/// * When the string is true or false (otherwise, it would be treated as a boolean value);
/// * When the string is null or ~ (otherwise, it would be considered as a null value);
/// * When the string looks like a number, such as integers (e.g. 2, 14, etc.), floats (e.g. 2.6, 14.9) and exponential numbers (e.g. 12e7, etc.) (otherwise, it would be treated as a numeric value);
/// * When the string looks like a date (e.g. 2014-12-31) (otherwise it would be automatically converted into a Unix timestamp).
fn need_quotes(string: &str) -> bool {
    fn need_quotes_spaces(string: &str) -> bool {
        string.starts_with(' ') || string.ends_with(' ')
    }

    string.is_empty()
        || need_quotes_spaces(string)
        || string.starts_with(['&', '*', '?', '|', '-', '<', '>', '=', '!', '%', '@'])
        || string.contains(|character: char| matches!(character,
            ':'
            | '{'
            | '}'
            | '['
            | ']'
            | ','
            | '#'
            | '`'
            | '\"'
            | '\''
            | '\\'
            | '\0'..='\x06'
            | '\t'
            | '\n'
            | '\r'
            | '\x0e'..='\x1a'
            | '\x1c'..='\x1f') )
        || [
            // http://yaml.org/type/bool.html
            // Note: 'y', 'Y', 'n', 'N', is not quoted deliberately, as in libyaml. PyYAML also parse
            // them as string, not booleans, although it is violating the YAML 1.1 specification.
            // See https://github.com/dtolnay/serde-yaml/pull/83#discussion_r152628088.
            "yes", "Yes", "YES", "no", "No", "NO", "True", "TRUE", "true", "False", "FALSE",
            "false", "on", "On", "ON", "off", "Off", "OFF",
            // http://yaml.org/type/null.html
            "null", "Null", "NULL", "~",
        ]
        .contains(&string)
        || string.starts_with('.')
        || string.starts_with("0x")
        || string.parse::<i64>().is_ok()
        || string.parse::<f64>().is_ok()
}
