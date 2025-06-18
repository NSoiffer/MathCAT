use crate::canonicalize::{as_element, name};
use crate::errors::*;
use crate::logs::enable_logs;
use crate::xpath_functions::{is_leaf, IsNode};
use lazy_static::lazy_static;
use regex::Regex;
use sxd_document::{Package};
use sxd_document::dom::*;

pub fn get_element(package: &Package) -> Element {
    enable_logs();
    let doc = package.as_document();
    let mut result = None;
    for root_child in doc.root().children() {
        if let ChildOfRoot::Element(e) = root_child {
            assert!(result.is_none());
            result = Some(e);
        }
    }
    return result.unwrap();
}

/// returns Ok() if two Documents are equal or some info where they differ in the Err
// Not really meant to be public -- used by tests in some packages
#[allow(dead_code)]
pub fn is_same_element(e1: Element, e2: Element) -> Result<()> {
    enable_logs();
    if name(e1) != name(e2) {
        bail!("Names not the same: {}, {}", name(e1), name(e2));
    }

    // assume 'e' doesn't have element children until proven otherwise
    // this means we keep Text children until we are proven they aren't needed
    if e1.children().len() != e2.children().len() {
        bail!(
            "Children of {} have {} != {} children",
            name(e1),
            e1.children().len(),
            e2.children().len()
        );
    }

    if let Err(e) = attrs_are_same(e1.attributes(), e2.attributes()) {
        bail!("In element {}, {}", name(e1), e);
    }

    for (i, (c1, c2)) in e1.children().iter().zip(e2.children().iter()).enumerate() {
        match c1 {
            ChildOfElement::Element(child1) => {
                if let ChildOfElement::Element(child2) = c2 {
                    is_same_element(*child1, *child2)?;
                } else {
                    bail!("{} child #{}, first is element, second is something else", name(e1), i);
                }
            }
            ChildOfElement::Comment(com1) => {
                if let ChildOfElement::Comment(com2) = c2 {
                    if com1.text() != com2.text() {
                        bail!("{} child #{} -- comment text differs", name(e1), i);
                    }
                } else {
                    bail!("{} child #{}, first is comment, second is something else", name(e1), i);
                }
            }
            ChildOfElement::ProcessingInstruction(p1) => {
                if let ChildOfElement::ProcessingInstruction(p2) = c2 {
                    if p1.target() != p2.target() || p1.value() != p2.value() {
                        bail!("{} child #{} -- processing instruction differs", name(e1), i);
                    }
                } else {
                    bail!(
                        "{} child #{}, first is processing instruction, second is something else",
                        name(e1),
                        i
                    );
                }
            }
            ChildOfElement::Text(t1) => {
                if let ChildOfElement::Text(t2) = c2 {
                    if t1.text() != t2.text() {
                        bail!("{} child #{} --  text differs", name(e1), i);
                    }
                } else {
                    bail!("{} child #{}, first is text, second is something else", name(e1), i);
                }
            }
        }
    }
    return Ok(());

    /// compares attributes -- '==' didn't seems to work
    fn attrs_are_same(attrs1: Vec<Attribute>, attrs2: Vec<Attribute>) -> Result<()> {
        if attrs1.len() != attrs2.len() {
            bail!("Attributes have different length: {:?} != {:?}", attrs1, attrs2);
        }
        // can't guarantee attrs are in the same order
        for attr1 in attrs1 {
            if let Some(found_attr2) = attrs2
                .iter()
                .find(|&attr2| attr1.name().local_part() == attr2.name().local_part())
            {
                if attr1.value() == found_attr2.value() {
                    continue;
                } else {
                    bail!(
                        "Attribute named {} has differing values:\n  '{}'\n  '{}'",
                        attr1.name().local_part(),
                        attr1.value(),
                        found_attr2.value()
                    );
                }
            } else {
                bail!(
                    "Attribute name {} not in [{}]",
                    print_attr(&attr1),
                    print_attrs(&attrs2)
                );
            }
        }
        return Ok(());

        fn print_attr(attr: &Attribute) -> String {
            return format!("@{}='{}'", attr.name().local_part(), attr.value());
        }
        fn print_attrs(attrs: &[Attribute]) -> String {
            return attrs.iter().map(print_attr).collect::<Vec<String>>().join(", ");
        }
    }
}

// used for testing trim
/// returns Ok() if two Documents are equal or some info where they differ in the Err
#[allow(dead_code)]
pub(crate) fn is_same_doc(doc1: &Document, doc2: &Document) -> Result<()> {
    // assume 'e' doesn't have element children until proven otherwise
    // this means we keep Text children until we are proven they aren't needed
    if doc1.root().children().len() != doc2.root().children().len() {
        bail!(
            "Children of docs have {} != {} children",
            doc1.root().children().len(),
            doc2.root().children().len()
        );
    }

    for (i, (c1, c2)) in doc1
        .root()
        .children()
        .iter()
        .zip(doc2.root().children().iter())
        .enumerate()
    {
        match c1 {
            ChildOfRoot::Element(e1) => {
                if let ChildOfRoot::Element(e2) = c2 {
                    is_same_element(*e1, *e2)?;
                } else {
                    bail!("child #{}, first is element, second is something else", i);
                }
            }
            ChildOfRoot::Comment(com1) => {
                if let ChildOfRoot::Comment(com2) = c2 {
                    if com1.text() != com2.text() {
                        bail!("child #{} -- comment text differs", i);
                    }
                } else {
                    bail!("child #{}, first is comment, second is something else", i);
                }
            }
            ChildOfRoot::ProcessingInstruction(p1) => {
                if let ChildOfRoot::ProcessingInstruction(p2) = c2 {
                    if p1.target() != p2.target() || p1.value() != p2.value() {
                        bail!("child #{} -- processing instruction differs", i);
                    }
                } else {
                    bail!(
                        "child #{}, first is processing instruction, second is something else",
                        i
                    );
                }
            }
        }
    }
    return Ok(());
}

/// Not really meant to be public -- used by tests in some packages
pub fn trim_element(e: Element, allow_structure_in_leaves: bool) {
    // "<mtext>this is text</mtext" results in 3 text children
    // these are combined into one child as it makes code downstream simpler

    // space, tab, newline, carriage return all get collapsed to a single space
    const WHITESPACE: &[char] = &[' ', '\u{0009}', '\u{000A}', '\u{000D}'];
    lazy_static! {
        static ref WHITESPACE_MATCH: Regex = Regex::new(r#"[ \u{0009}\u{000A}\u{000D}]+"#).unwrap();
    }

    if is_leaf(e) && (!allow_structure_in_leaves || IsNode::is_mathml(e)) {
        // Assume it is HTML inside of the leaf -- turn the HTML into a string
        make_leaf_element(e);
        return;
    }

    let mut single_text = "".to_string();
    for child in e.children() {
        match child {
            ChildOfElement::Element(c) => {
                trim_element(c, allow_structure_in_leaves);
            }
            ChildOfElement::Text(t) => {
                single_text += t.text();
                e.remove_child(child);
            }
            _ => {
                e.remove_child(child);
            }
        }
    }

    // CSS considers only space, tab, linefeed, and carriage return as collapsable whitespace
    if !(is_leaf(e) || name(e) == "intent-literal" || single_text.is_empty()) {
        // intent-literal comes from testing intent
        // FIX: we have a problem -- what should happen???
        // FIX: For now, just keep the children and ignore the text and log an error -- shouldn't panic/crash
        if !single_text.trim_matches(WHITESPACE).is_empty() {
            error!(
                "trim_element: both element and textual children which shouldn't happen -- ignoring text '{}'",
                single_text
            );
        }
        return;
    }
    if e.children().is_empty() && !single_text.is_empty() {
        // debug!("Combining text in {}: '{}' -> '{}'", e.name().local_part(), single_text, trimmed_text);
        e.set_text(&WHITESPACE_MATCH.replace_all(&single_text, " "));
    }

    fn make_leaf_element(mathml_leaf: Element) {
        // MathML leaves like <mn> really shouldn't have non-textual content, but you could have embedded HTML
        // Here, we take convert them to leaves by grabbing up all the text and making that the content
        // Potentially, we leave them and let (default) rules do something, but it makes other parts of the code
        //   messier because checking the text of a leaf becomes Option<&str> rather than just &str
        let children = mathml_leaf.children();
        if children.is_empty() {
            return;
        }

        // gather up the text
        let mut text = "".to_string();
        for child in children {
            let child_text = match child {
                ChildOfElement::Element(child) => {
                    if name(child) == "mglyph" {
                        child.attribute_value("alt").unwrap_or("").to_string()
                    } else {
                        gather_text(child)
                    }
                }
                ChildOfElement::Text(t) => {
                    // debug!("ChildOfElement::Text: '{}'", t.text());
                    t.text().to_string()
                }
                _ => "".to_string(),
            };
            if !child_text.is_empty() {
                text += &child_text;
            }
        }

        // get rid of the old children and replace with the text we just built
        mathml_leaf.clear_children();
        mathml_leaf.set_text(WHITESPACE_MATCH.replace_all(&text, " ").trim_matches(WHITESPACE));
        // debug!("make_leaf_element: text is '{}'", crate::canonicalize::as_text(mathml_leaf));

        /// gather up all the contents of the element and return them with a leading space
        fn gather_text(html: Element) -> String {
            let mut text = "".to_string(); // since we are throwing out the element tag, add a space between the contents
            for child in html.children() {
                match child {
                    ChildOfElement::Element(child) => {
                        text += &gather_text(child);
                    }
                    ChildOfElement::Text(t) => text += t.text(),
                    _ => (),
                }
            }
            // debug!("gather_text: '{}'", text);
            return text;
        }
    }
}

pub(crate) fn add_ids(mathml: Element) -> Element {
    use std::time::SystemTime;
    let time = if cfg!(target_family = "wasm") {
        fastrand::usize(..)
    } else {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize
    };
    let time_part = radix_fmt::radix(time, 36).to_string();
    let random_part = radix_fmt::radix(fastrand::u32(..), 36).to_string();
    let prefix = "M".to_string() + &time_part[time_part.len() - 3..] + &random_part[random_part.len() - 4..] + "-"; // begin with letter
    add_ids_to_all(mathml, &prefix, 0);
    return mathml;

    fn add_ids_to_all(mathml: Element, id_prefix: &str, count: usize) -> usize {
        let mut count = count;
        if mathml.attribute("id").is_none() {
            mathml.set_attribute_value("id", (id_prefix.to_string() + &count.to_string()).as_str());
            mathml.set_attribute_value("data-id-added", "true");
            count += 1;
        };

        if crate::xpath_functions::is_leaf(mathml) {
            return count;
        }

        for child in mathml.children() {
            let child = as_element(child);
            count = add_ids_to_all(child, id_prefix, count);
        }
        return count;
    }
}
