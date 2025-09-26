//! Navigation is controlled by a `Navigation_Rules.yaml` file in conjunction with preferences.
//! See preference documentation for more info on navigation preferences.
#![allow(clippy::needless_return)]

use std::cell::{Ref, RefCell, RefMut};
use sxd_xpath::context::Evaluation;
use sxd_xpath::{Context, Value};
use sxd_document::dom::Element;
use sxd_document::Package;

use std::fmt;
use crate::canonicalize::{name, get_parent};
use crate::pretty_print::mml_to_string;
use crate::speech::{NAVIGATION_RULES, CONCAT_INDICATOR, CONCAT_STRING, SpeechRules, SpeechRulesWithContext};
use crate::infer_intent::add_fixity_children;
use crate::interface::copy_mathml;
#[cfg(not(target_family = "wasm"))]
use std::time::Instant;
use crate::errors::*;
use phf::phf_set;


const MAX_PLACE_MARKERS: usize = 10;

thread_local!{
    /// The current set of navigation rules
    pub static NAVIGATION_STATE: RefCell<NavigationState> =
            RefCell::new( NavigationState::new() );
}

pub static NAV_COMMANDS: phf::Set<&str> = phf_set! {
    "MovePrevious", "MoveNext", "MoveStart", "MoveEnd", "MoveLineStart", "MoveLineEnd", 
    "MoveCellPrevious", "MoveCellNext", "MoveCellUp", "MoveCellDown", "MoveColumnStart", "MoveColumnEnd", 
    "ZoomIn", "ZoomOut", "ZoomOutAll", "ZoomInAll", 
    "MoveLastLocation", 
    "ReadPrevious", "ReadNext", "ReadCurrent", "ReadCellCurrent", "ReadStart", "ReadEnd", "ReadLineStart", "ReadLineEnd", 
    "DescribePrevious", "DescribeNext", "DescribeCurrent", 
    "WhereAmI", "WhereAmIAll", 
    "ToggleZoomLockUp", "ToggleZoomLockDown", "ToggleSpeakMode", 
    "Exit", 
    "MoveTo0","MoveTo1","MoveTo2","MoveTo3","MoveTo4","MoveTo5","MoveTo6","MoveTo7","MoveTo8","MoveTo9",
    "Read0","Read1","Read2","Read3","Read4","Read5","Read6","Read7","Read8","Read9",
    "Describe0","Describe1","Describe2","Describe3","Describe4","Describe5","Describe6","Describe7","Describe8","Describe9",
    "SetPlacemarker0","SetPlacemarker1","SetPlacemarker2","SetPlacemarker3","SetPlacemarker4","SetPlacemarker5","SetPlacemarker6","SetPlacemarker7","SetPlacemarker8","SetPlacemarker9",
};

#[derive(Clone, PartialEq, Debug)]
struct NavigationPosition {
    current_node: String,           // id of current node
    current_node_offset: usize,     // for leaves, what char offset in leaf (default = 0)
}

impl fmt::Display for NavigationPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}[+{}]", self.current_node, self.current_node_offset);
    }
}

const ILLEGAL_NODE_ID: &str = "!not set";     // an illegal 'id' value
impl Default for NavigationPosition {
    fn default() -> Self {
        NavigationPosition {
            current_node: ILLEGAL_NODE_ID.to_string(), 
            current_node_offset: 0    
        }
     }
}

impl NavigationPosition {
    
}

#[derive(Debug, Clone)]
pub struct NavigationState {
    // it might be better to use a linked for the stacks, with the first node being the top
    // these two stacks should be kept in sync.
    position_stack: Vec<NavigationPosition>,    // all positions, so we can go back to them
    command_stack: Vec<&'static str>,           // all commands, so we can undo them
    place_markers: [NavigationPosition; MAX_PLACE_MARKERS],
    where_am_i: NavigationPosition,             // current 'where am i' location

    #[cfg(target_family = "wasm")]
    where_am_i_start_time: usize,               // FIX: for web
    #[cfg(not(target_family = "wasm"))]
    where_am_i_start_time: Instant,
    mode: String,                               // one of "Character", "Simple", or "Enhanced"
    speak_overview: bool,                       // true => describe after move; false => (standard) speech rules
}

impl fmt::Display for NavigationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "NavigationState{{")?;
        write!(f, "  Position Stack: ")?;
        for (i, nav_state) in self.position_stack.iter().enumerate() {
            write!(f, "{}{}", if i==0 {""} else {", "}, nav_state)?;
        }
        writeln!(f)?;
        write!(f, "  Command Stack: ")?;
        for (i, nav_state) in self.command_stack.iter().enumerate() {
            write!(f, "{}{}", if i==0 {""} else {", "}, *nav_state)?;
        }
        writeln!(f)?;
        writeln!(f, "  where_am_i: {}, start_time: {:?}", self.where_am_i, self.where_am_i_start_time)?;
        writeln!(f, "  mode: {}, speak_overview: {}", self.mode, self.speak_overview)?;
        writeln!(f, "}}")?;
        return Ok( () );
    }
}

impl NavigationState {
    fn new() -> NavigationState {
        return NavigationState {
            position_stack: Vec::with_capacity(1024),
            command_stack: Vec::with_capacity(1024),
            place_markers: Default::default(),
            where_am_i: NavigationPosition::default(),
            // FIX: figure this out for the web
            #[cfg(target_family = "wasm")]
            where_am_i_start_time: 0,           // FIX: for web
            #[cfg(not(target_family = "wasm"))]
            where_am_i_start_time: Instant::now(),      // need to give it some value, and "default()" isn't an option
            mode: "".to_string(),                       // set latter when we have some context
            speak_overview: false,                      // set latter when we have some context
        };
    }

    pub fn reset(&mut self) {
        self.position_stack.clear();
        self.command_stack.clear();
        self.where_am_i = NavigationPosition::default();
        self.reset_start_time()
    }


    // defining reset_start_time because of the following message if done inline
    // attributes on expressions are experimental
    // see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
    #[cfg(target_family = "wasm")]
    fn reset_start_time(&mut self) {
         self.where_am_i_start_time = 0;
    }

    #[cfg(not(target_family = "wasm"))]
    fn reset_start_time(&mut self) {
         self.where_am_i_start_time = Instant::now();      // need to give it some value, and "default()" isn't an option
    }


    fn push(&mut self, position: NavigationPosition, command: &'static str) {
        self.position_stack.push(position);
        self.command_stack.push(command);
    }

    fn pop(&mut self) -> Option<(NavigationPosition, &'static str)> {
        assert_eq!(self.position_stack.len(), self.command_stack.len());
        if self.position_stack.is_empty() {
            return None;
        } else {
            return Some( (self.position_stack.pop().unwrap(), self.command_stack.pop().unwrap()) );
        }
    }

    fn top(&self) -> Option<(&NavigationPosition, &'static str)> {
        if self.position_stack.is_empty() {
            return None;
        }
        let last = self.position_stack.len()-1;
        return Some( (&self.position_stack[last], self.command_stack[last]) );
    }

    pub fn get_navigation_mathml<'a>(&self, mathml: Element<'a>) -> Result<(Element<'a>, usize)> {
        if self.position_stack.is_empty() {
            return Ok( (mathml, 0) );
        } else {
            let (position, _) = self.top().unwrap();
            return match get_node_by_id(mathml, &position.current_node) {
                None => bail!("internal error: id '{}' was not found in mathml:\n{}",
                                position.current_node, mml_to_string(mathml)),
                Some(found) => Ok( (found, position.current_node_offset) )
            };
        }
    }

    pub fn get_navigation_mathml_id(&self, mathml: Element) -> (String, usize) {
        if self.position_stack.is_empty() {
            return (mathml.attribute_value("id").unwrap().to_string(), 0);
        } else {
            let (position, _) = self.top().unwrap();
            return (position.current_node.clone(), position.current_node_offset);
        }
    }

    fn init_navigation_context(&self, context: &mut Context, command: &'static str,
                               nav_state_top: Option<(&NavigationPosition, &'static str)>) {
        context.set_variable("NavCommand", command);

        if command == "WhereAmI" && self.where_am_i == NavigationPosition::default() {
            context.set_variable("NavNode", self.where_am_i.current_node.as_str());
            context.set_variable("NavNodeOffset", self.where_am_i.current_node_offset as f64);
        } else {
            let position = &self.position_stack[self.position_stack.len()-1];
            context.set_variable("NavNode", position.current_node.as_str());
            context.set_variable("NavNodeOffset", position.current_node_offset as f64);
        }

        // get the index from command (e.g., '3' in 'SetPlacemarker3 or MoveTo3' and set 'PlaceMarker' to it's position)
        if command.ends_with(|ch: char| ch.is_ascii_digit()) {
            let index = convert_last_char_to_number(command);
            let position = &self.place_markers[index];
            context.set_variable("PlaceMarkerIndex", index as f64);
            context.set_variable("PlaceMarker", position.current_node.as_str());
            context.set_variable("PlaceMarkerOffset", position.current_node_offset as f64);
        }
           
        context.set_variable("Overview", self.speak_overview);
        context.set_variable("ReadZoomLevel", (if self.mode == "Enhanced" {-1} else {1}) as f64);
        context.set_variable("MatchCounter", 0 as f64);

        if command == "MoveLastLocation" {
            let previous_command = match nav_state_top {
                None => "None",
                Some( (_, previous_command) ) => previous_command,
            };
            context.set_variable("PreviousNavCommand", previous_command);
        }

        // used by nav rules for speech -- needs an initial value so tests don't fail
        context.set_variable("SayCommand", "" );
        context.set_variable("Move2D", "" );
        context.set_variable("SpeakExpression", true );    // default is to speak the expr after navigation
        return;

        fn convert_last_char_to_number(str: &str) -> usize {
            let last_char = str.as_bytes()[str.len()-1];
            assert!( last_char.is_ascii_digit() );
            return (last_char - b'0') as usize;
        }
    }
}

// convert the last digit of a Placemarker command to an integer
fn convert_last_char_to_number(str: &str) -> usize {
    let last_char = str.as_bytes()[str.len()-1];
    assert!( last_char.is_ascii_digit() );
    return (last_char - b'0') as usize;
}

/// Get the node associated with 'id'
/// This can be called on an intent tree -- it does not make use of is_leaf()
fn get_node_by_id<'a>(mathml: Element<'a>, id: &str) -> Option<Element<'a>> {
    if let Some(mathml_id) = mathml.attribute_value("id") {
        if mathml_id == id {
            return Some(mathml);
        }
    }

    for child in mathml.children() {
        if let Some(child) = child.element() {
            if let Some(found) = get_node_by_id(child, id) {
                return Some(found);
            }
        }
    }
    return None;
}

/// Search the mathml for the id and set the navigation node to that id
/// Resets the navigation stack
pub fn set_navigation_node_from_id(mathml: Element, id: String, offset: usize) -> Result<()> {
    let node = get_node_by_id(mathml, &id);
    if let Some(node) = node {
        if !crate::xpath_functions::is_leaf(node) && offset != 0 {
            bail!("Id {} is not a leaf in the MathML tree but has non-zero offset={}. Referenced MathML node is {}", id, offset, mml_to_string(node));
        }
        return NAVIGATION_STATE.with(|nav_state| {
            let mut nav_state = nav_state.borrow_mut();
            nav_state.reset();
            nav_state.push(NavigationPosition{
                current_node: id,
                current_node_offset: offset
            }, "None");
            return Ok( () );
        })
    } else {
        bail!("Id {} not found in MathML {}", id, mml_to_string(mathml));
    }

}

/// Get's the Nav Node from the context, with some exceptions such as Toggle commands where it isn't set.
pub fn get_nav_node<'c>(context: &Context<'c>, var_name: &str, mathml: Element<'c>, start_node: Element<'c>, command: &str, nav_mode: &str) -> Result<(Option<String>, Option<f64>)> {
    let start_id = start_node.attribute_value("id").unwrap_or_default();
    if command.starts_with("Toggle") {
        return Ok( (Some(start_id.to_string()), None) );
    } else {
        return context_get_variable(context, var_name, mathml)
                .chain_err(|| format!("When trying to {} starting at id={} in {} mode",
                                                command, start_node.attribute_value("id").unwrap_or_default(), nav_mode));
    }
}

// FIX: think of a better place to put this, and maybe a better interface
pub fn context_get_variable<'c>(context: &Context<'c>, var_name: &str, mathml: Element<'c>) -> Result<(Option<String>, Option<f64>)> {
    // First return tuple value is string-value (if string, bool, or single node) or None
    // Second return tuple value is f64 if variable is a number or None
    // This is slightly roundabout because Context doesn't expose a way to get the values.
    // Instead, we create an "Evaluation", which is just one level of indirection.
    // Note: mathml can be any node. It isn't really used but some Element needs to be part of Evaluate()
    use sxd_xpath::nodeset::Node;
    let evaluation = Evaluation::new(context, Node::Element(mathml));
    return match evaluation.value_of(var_name.into()) {
        Some(value) => match value {
            Value::String(s) => Ok((Some(s.clone()), None)),
            Value::Number(f) => Ok((None, Some(*f))),
            Value::Boolean(b) => Ok((Some(format!("{b}")), None)),
            Value::Nodeset(nodes) => {
                if nodes.size() == 1 {
                    if let Some(attr) = nodes.document_order_first().unwrap().attribute() {
                        return Ok( (Some(attr.value().to_string()), None) );
                    }
                };
                let mut error_message = format!("Variable '{var_name}' set somewhere in navigate.yaml is nodeset and not an attribute: ");
                if nodes.size() == 0 {
                    error_message += &format!("0 nodes (false) -- {} set to non-existent node in\n{}",
                                              var_name, mml_to_string(mathml));
                } else {
                    let singular = nodes.size()==1;
                    error_message += &format!("{} node{}. {}:",
                            nodes.size(),
                            if singular {""} else {"s"},
                            if singular {"Node is"} else {"Nodes are"});
                    nodes.document_order()
                        .iter()
                        .enumerate()
                        .for_each(|(i, node)| {
                            match node {
                                sxd_xpath::nodeset::Node::Element(mathml) =>
                                    error_message += &format!("#{}:\n{}",i, mml_to_string(*mathml)),
                                _ => error_message += &format!("'{node:?}'"),
                            }   
                        })    
                };
                bail!(error_message);
            },
        },
        None => bail!("Could not find value for navigation variable '{}'", var_name),
    }
}

/// Given a key code along with the modifier keys, the current node is moved accordingly (or value reported in some cases).]
/// The spoken text for the new current node is returned.
pub fn do_mathml_navigate_key_press(mathml: Element,
            key: usize, shift_key: bool, control_key: bool, alt_key: bool, meta_key: bool) -> Result<String> {
    let (command, param) = key_press_to_command_and_param(key, shift_key, control_key, alt_key, meta_key)?;
    return do_navigate_command_and_param(mathml, command, param);
}

fn do_navigate_command_and_param(mathml: Element, command: NavigationCommand, param: NavigationParam) -> Result<String> {
    return do_navigate_command_string(mathml, navigation_command_string(command, param));
}

pub fn do_navigate_command_string(mathml: Element, nav_command: &'static str) -> Result<String> {   
    // first check to see if nav file has been changed -- don't bother checking in loop below
    NAVIGATION_RULES.with(|rules| {
        rules.borrow_mut().read_files()
    })?;

    if mathml.children().is_empty() {
        bail!("MathML has not been set -- can't navigate");
    };

    return NAVIGATION_STATE.with(|nav_state| {
        let mut nav_state = nav_state.borrow_mut();
        // debug!("MathML: {}", mml_to_string(mathml));
        if nav_state.position_stack.is_empty() {
            // initialize to root node
            nav_state.push(NavigationPosition{
                current_node: mathml.attribute_value("id").unwrap().to_string(),
                current_node_offset: 0
            }, "None")
        };

        return NAVIGATION_RULES.with(|rules| {
            let rules = rules.borrow();
            let new_package = Package::new();
            let mut rules_with_context = SpeechRulesWithContext::new(&rules, new_package.as_document(), "");
            
            nav_state.mode = rules.pref_manager.as_ref().borrow().pref_to_string("NavMode");
            nav_state.speak_overview = rules.pref_manager.as_ref().borrow().pref_to_string("Overview") == "true";

            nav_state.init_navigation_context(rules_with_context.get_context(), nav_command, nav_state.top());
            
            // start navigation off at the right node
            if nav_command == "MoveLastLocation" {
                nav_state.pop();
            }

            // If no speech happened for some calls, we try the call again (e.g, no speech for invisible times).
            // To prevent to infinite loop, we limit the number of tries
            const LOOP_LIMIT: usize = 3;
            let mut cumulative_speech = String::with_capacity(120);
            for loop_count in 0..LOOP_LIMIT {
                match apply_navigation_rules(mathml, nav_command, &rules, &mut rules_with_context, &mut nav_state, loop_count) {
                    Ok( (speech, done)) => {
                        cumulative_speech = cumulative_speech + if loop_count==0 {""} else {" "} + speech.trim();
                        if done {
                            let (tts, rate) = {
                                let prefs = rules.pref_manager.borrow();
                                (prefs.pref_to_string("TTS"), prefs.pref_to_string("MathRate"))
                            };
                            if rate != "100" {
                                match tts.as_str() {
                                    "SSML" => if !cumulative_speech.starts_with("<prosody rate") {
                                        cumulative_speech = format!("<prosody rate='{}%'>{}</prosody>", &rate, &cumulative_speech);
                                    }, 
                                    "SAPI5" => if !cumulative_speech.starts_with("<rate speed") {
                                        cumulative_speech = format!("<rate speed='{:.1}'>{}</rate>'>",
                                        10.0*(0.01*rate.parse::<f32>().unwrap_or(100.0)).log(3.0), cumulative_speech);
                                    },
                                    _ => (),  // do nothing
                                }
                            }
                                                return Ok( rules.pref_manager.borrow().get_tts()
                                            .merge_pauses(crate::speech::remove_optional_indicators(
                                                &cumulative_speech.replace(CONCAT_STRING, "")
                                                                    .replace(CONCAT_INDICATOR, "")                            
                                                            )
                                            .trim_start().trim_end_matches([' ', ',', ';'])) );
                        }
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            bail!("Internal error: Navigation exceeded limit of number of times no speech generated
                   when attempting to {} in {} mode start at id={} in this MathML:\n{}.",
                   nav_command, nav_state.mode, nav_state.top().unwrap().0.current_node, mml_to_string(mathml));
        });
    });

    fn get_start_node<'m>(mathml: Element<'m>, nav_state: &RefMut<NavigationState>) -> Result<Element<'m>>  {
        let start_node_id =  match nav_state.top() {
            None => mathml.attribute_value("id").unwrap(),
            Some( (position, _) ) => position.current_node.as_str(),
        };

        return match get_node_by_id(mathml, start_node_id) {
            Some(node) => Ok(node),
            None => {
                bail!("Internal Error: didn't find id '{}' while attempting to start navigation. MathML is\n{}",
                      &start_node_id, mml_to_string(mathml));
            }
        };
    }



    fn apply_navigation_rules<'c, 'm:'c>(mathml: Element<'m>, nav_command: &'static str,
            rules: &Ref<SpeechRules>, rules_with_context: &mut SpeechRulesWithContext<'c, '_, 'm>, nav_state: &mut RefMut<NavigationState>,
            loop_count: usize) -> Result<(String, bool)> {
        let context = rules_with_context.get_context();
        context.set_variable("MatchCounter", loop_count as f64);
        nav_state.mode = context_get_variable(context, "NavMode", mathml)?.0.unwrap();

        let mut add_literal = nav_state.mode == "Character";
        let (intent, nav_intent) = if add_literal {
            (mathml, mathml)
        } else {
            let intent = crate::speech::intent_from_mathml(mathml, rules_with_context.get_document())?;
            (intent, add_fixity_children(copy_mathml(intent)))
        };

        let mut properties = "";
        if add_literal {
            properties  = mathml.attribute_value("data-intent-property").unwrap_or_default();
            if properties.contains(":literal:") {
                add_literal = false;
            } else {
                mathml.set_attribute_value("data-intent-property", (":literal:".to_string() + properties).as_str());
            };
        }
        // we should always find the start node.
        // however, if were were navigating by character, then switched the NavMode, the intent tree might not have that node in it
        let start_node = match get_start_node(nav_intent, nav_state) {
            Ok(node) => node,
            Err(_) => {
                // find the node in the other tree (probably mathml) and walk up to find a parent that has an id in both
                let other_tree = if nav_state.mode == "Character" {nav_intent} else {mathml};
                let mut found_node = get_start_node(other_tree, nav_state)?;
                while name(found_node) != "math" {
                    found_node = get_parent(found_node);
                    // debug!("found_node:\n{}", mml_to_string(found_node));
                    if let Some(intent_node) = get_node_by_id(nav_intent, found_node.attribute_value("id").unwrap_or_default()) {
                        found_node = intent_node;
                        break;
                    }
                }
                found_node
            }
        };

        // debug!("intent=\n{}", mml_to_string(intent));
        // debug!("nav intent=\n{}", mml_to_string(nav_intent));
        // debug!("start_node id={}\n{}", nav_state.top().unwrap().0.current_node.as_str(), mml_to_string(start_node));
        // if name(start_node) != "math" {
        //     let mut parent= get_parent(start_node);
        //     if name(parent) != "math" {
        //         parent = get_parent(parent);
        //     }
        //     debug!("parent or grandparent of start_node:\n{}", mml_to_string(parent));
        // }

        let raw_speech_string = rules_with_context.match_pattern::<String>(start_node)
                    .chain_err(|| "Pattern match/replacement failure during math navigation!")?;
        let speech = rules.pref_manager.borrow().get_tts()
                    .merge_pauses(crate::speech::remove_optional_indicators(
                        &raw_speech_string.replace(CONCAT_STRING, "")
                                                .replace(CONCAT_INDICATOR, "")                            
                                    )
                    .trim());
        // debug!("Nav Speech: {}", speech);

        // FIX: add things that need to do a speech replacement based on some marker for "where am i" and others that loop ([Speak: id])???
        // what else needs to be done/set???

        // transfer some values that might have been set into the prefs
        let context = rules_with_context.get_context();
        nav_state.speak_overview = context_get_variable(context, "Overview", intent)?.0.unwrap() == "true";
        nav_state.mode = context_get_variable(context, "NavMode", intent)?.0.unwrap();
        rules.pref_manager.as_ref().borrow_mut().set_user_prefs("NavMode", &nav_state.mode)?;

        let nav_position = match get_nav_node(
                        context, "NavNode", intent, start_node, nav_command, &nav_state.mode)?.0 {
            None => NavigationPosition::default(),
            Some(node) => NavigationPosition {
                current_node: node,
                current_node_offset: context_get_variable(context, "NavNodeOffset", intent)?.1.unwrap() as usize
            }
        };

        // after a command, we either read or describe the new location (part of state)
        // also some commands are DescribeXXX/ReadXXX, so we need to look at the commands also
        let use_read_rules = if nav_command.starts_with("Read") {
            true
        } else if nav_command.starts_with("Describe") {
            false
        } else {
            !nav_state.speak_overview
        };

        { //if (nav_command.starts_with("Move") || nav_command.starts_with("Zoom")) && nav_command != "MoveLastLocation" {
            // push the new location on the stack
            if nav_position != NavigationPosition::default() {
                // debug!("nav_state: pushing on {}", &nav_position);
                let current_node = nav_position.current_node.as_str();
                if current_node != nav_state.top().unwrap().0.current_node && current_node != ILLEGAL_NODE_ID {
                    nav_state.push(nav_position.clone(), nav_command);
                }
            }
        }

        if nav_command.starts_with("SetPlacemarker") {
            if let Some(new_node_id) = get_nav_node(
                            context, "NavNode", intent, start_node, nav_command, &nav_state.mode)?.0 {
                let offset = context_get_variable(context, "NavNodeOffset", intent)?.1.unwrap() as usize;
                nav_state.place_markers[convert_last_char_to_number(nav_command)] = NavigationPosition{ current_node: new_node_id, current_node_offset: offset};
            }
        }

        let nav_mathml = get_node_by_id(intent, &nav_position.current_node);
        if nav_mathml.is_some() && context_get_variable(context, "SpeakExpression", intent)?.0.unwrap() == "true" {
            // Speak/Overview of where we landed (if we are supposed to speak it) -- use intent, not nav_intent
            // Note: NavMode might have changed, so we need to recheck the mode to see if we use LiteralSpeak
            let literal_speak = nav_state.mode == "Character";
            let node_speech = match speak(mathml, intent, &nav_position.current_node, literal_speak, use_read_rules) {
                Ok(speech) => speech,
                Err(e) => {
                    remove_literal_property(mathml, add_literal, properties);
                    if e.to_string() == crate::speech::NAV_NODE_SPEECH_NOT_FOUND {
                        bail!("Internal error: With {}/{} in {} mode, can't {} from expression with id '{}' inside:\n{}",
                              rules.pref_manager.as_ref().borrow().pref_to_string("Language"),
                              rules.pref_manager.as_ref().borrow().pref_to_string("SpeechStyle"),
                              &nav_state.mode, nav_command, &nav_position.current_node, mml_to_string(if literal_speak {mathml} else {intent}));
                    } else {
                        return Err(e);
                    }
                },
            };
            remove_literal_property(mathml, add_literal, properties);

            // debug!("node_speech: '{}'", node_speech);
            if node_speech.is_empty() {
                // try again in loop
                return Ok( (speech, false));
            } else {
                pop_stack(nav_state, loop_count, nav_command);
                // debug!("returning: '{}'", speech.clone() + " " + &node_speech);
                return Ok( (speech + " " + &node_speech, true) );
            }
        } else {
            remove_literal_property(mathml, add_literal, properties);
            pop_stack(nav_state, loop_count, nav_command);
            return Ok( (speech, true) );
        };

        fn remove_literal_property(mathml: Element, add_literal: bool, properties: &str) {
            if add_literal {
                if properties.is_empty() {
                    mathml.remove_attribute("data-intent-property");
                } else {
                    mathml.set_attribute_value("data-intent-property", properties);
                }
            }
        }

    }


    fn pop_stack(nav_state: &mut NavigationState, count: usize, nav_command: &'static str) {
        // save the final state and pop the intermediate states that did nothing
        let push_command_on_stack = (nav_command.starts_with("Move") && nav_command != "MoveLastLocation") || nav_command.starts_with("Zoom");
        // debug!("pop_stack: nav_command={}, count={}, push? {} stack=\n{}", nav_command, count, push_command_on_stack, nav_state);
        if count == 0 {
            if !push_command_on_stack && nav_command == nav_state.top().unwrap().1 {
                nav_state.pop();    // remove ReadXXX, SetPlacemarker, etc. commands that don't change the state
            }
            return;
        }
        let (top_position, top_command) = nav_state.pop().unwrap();
        let mut count = count - 1;
        loop {
            // debug!("  ... loop count={}", count);
            nav_state.pop();
            if count == 0 {
                break;
            };
            count -= 1;
        };
        if push_command_on_stack {
            nav_state.push(top_position, top_command);
        }
        // debug!("END pop_stack: stack=\n{}", nav_state);
    }
}

/// Speak the intent tree at the nav_node_id if that id exists in the intent tree; otherwise use the mathml tree.
/// If full_read is true, we speak the tree, otherwise we use the overview rules.
/// If literal_speak is true, we use the literal speak rules (and use the mathml tree).
fn speak(mathml: Element, intent: Element, nav_node_id: &str, literal_speak: bool, full_read: bool) -> Result<String> {
    if full_read {
        // In something like x^3, we might be looking for the '3', but it will be "cubed", so we don't find it.
        // Or we might be on a "(" surrounding a matrix and that isn't part of the intent
        // We are probably safer in terms of getting the same speech if we retry intent starting at the nav node,
        //  but the node to speak is almost certainly trivial.
        // By speaking the non-intent tree, we are certain to speak on the next try
        if !literal_speak && get_node_by_id(intent, nav_node_id).is_some() {
            // debug!("speak: nav_node_id={}, intent=\n{}", nav_node_id, mml_to_string(intent));
            match crate::speech::speak_mathml(intent, nav_node_id) {
                Ok(speech) => return Ok(speech),
                Err(e) => {
                    if e.to_string() != crate::speech::NAV_NODE_SPEECH_NOT_FOUND {
                        return Err(e);
                    }
                    // else could be something like '3' in 'x^3' ("cubed")
                },
            }
        }
        // debug!("speak (literal): nav_node_id={}, mathml=\n{}", nav_node_id, mml_to_string(mathml));
        let speech = crate::speech::speak_mathml(mathml, nav_node_id);
        // debug!("speech from speak: {:?}", speech);
        return speech;
    } else {
        return crate::speech::overview_mathml(mathml, nav_node_id);
    }
}


// MathPlayer's interface mentions these, so we keep them.
// These (KeyboardEvent.keyCode) are consistent across platforms (mostly?) but are deprecated.
//   KeyboardEvent.code is recommended instead (a string)
const VK_LEFT: usize = 0x25;
const VK_RIGHT: usize = 0x27;
const VK_UP: usize = 0x26;
const VK_DOWN: usize = 0x28;
const VK_RETURN: usize = 0x0D;
const VK_SPACE: usize = 0x20;
const VK_HOME: usize = 0x24;
const VK_END: usize = 0x23;
const VK_BACK: usize = 0x08;
const VK_ESCAPE: usize = 0x1B;

// Utilities that returns one of four commands/params based on shift/control key combinations

enum NavigationCommand {
    Move,
    Zoom,
    MoveLastLocation,
    Read,
    Describe,
    ReadTo,
    Locate,
    ChangeNavMode,
    ToggleSpeakMode,
    SetPlacemarker,
    Exit,
    Last,
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum NavigationParam {
    Placemarker0,
    Placemarker1,
    Placemarker2,
    Placemarker3,
    Placemarker4,
    Placemarker5,
    Placemarker6,
    Placemarker7,
    Placemarker8,
    Placemarker9,
    Previous,
    Current,
    Next,
    Start,
    End,
    LineStart,
    LineEnd,
    CellPrevious,
    CellCurrent,
    CellNext,
    ColStart,
    ColEnd,
    CellUp,
    CellDown,
    Last 
}


fn choose_command(
	shift_key: bool,
	control_key: bool,
	none: NavigationCommand,
	shift: NavigationCommand,
	control: NavigationCommand,
	shift_control: NavigationCommand
) -> NavigationCommand {
	   if shift_key && control_key {
		return shift_control;
    } else if control_key {
        return control;
    } else if shift_key {
		return shift;
	} else {
		return none;
    }
}

fn choose_param(
	shift_key: bool,
	control_key: bool,
	none: NavigationParam,
	shift: NavigationParam,
	control: NavigationParam,
	shift_control: NavigationParam
) -> NavigationParam {
    if shift_key && control_key {
		return shift_control;
    } else if control_key {
        return control;
    } else if shift_key {
		return shift;
	} else {
		return none;
    }
}

fn key_press_to_command_and_param(
    key: usize,
	shift_key: bool,
	control_key: bool,
	alt_key: bool,
	meta_key: bool,
) -> Result<(NavigationCommand, NavigationParam)> {
	// key press mapping should probably be stored externally (registry) with an app that allows changes
	// for now, we build in the defaults

    // this is a hack to map alt+ctl+arrow to ctl+arrow to change table mappings (github.com/NSoiffer/MathCAT/issues/105)
    // if this change sticks, choose_command() needs to be changed and this hack should go away
    let mut alt_key = alt_key;
    if alt_key && control_key && [VK_LEFT, VK_RIGHT, VK_UP, VK_DOWN].contains(&key) {
        alt_key = false;
    }
	if alt_key || meta_key {
        bail!("Invalid argument to key_press_to_command_and_param");
    }

    let command;
    let param;
	match key {
        VK_LEFT => {
            command = choose_command(shift_key, control_key, NavigationCommand::Move,   NavigationCommand::Read,	NavigationCommand::Move,	   NavigationCommand::Describe);
            param =   choose_param(  shift_key, control_key, NavigationParam::Previous, NavigationParam::Previous, NavigationParam::CellPrevious, NavigationParam::Previous);
            },
        VK_RIGHT => {
            command = choose_command(shift_key, control_key, NavigationCommand::Move,	NavigationCommand::Read, NavigationCommand::Move,	  NavigationCommand::Describe);
            param =   choose_param(  shift_key, control_key, NavigationParam::Next, NavigationParam::Next, NavigationParam::CellNext, NavigationParam::Next);
            },
        VK_UP => {
            command = choose_command(shift_key, control_key, NavigationCommand::Zoom,      NavigationCommand::ChangeNavMode, NavigationCommand::Move,   NavigationCommand::Zoom);
            param =   choose_param(  shift_key, control_key, NavigationParam::Previous,  NavigationParam::Previous,      NavigationParam::CellUp, NavigationParam::Start);
            },
        VK_DOWN => {
            command = choose_command(shift_key, control_key, NavigationCommand::Zoom, NavigationCommand::ChangeNavMode, NavigationCommand::Move,     NavigationCommand::Zoom);
            param =   choose_param(  shift_key, control_key, NavigationParam::Next, NavigationParam::Next,          NavigationParam::CellDown, NavigationParam::End);
            },
        VK_RETURN => {
            command = choose_command(shift_key, control_key, NavigationCommand::Locate,  NavigationCommand::Last, NavigationCommand::Locate, NavigationCommand::Last);
            param =   choose_param(  shift_key, control_key, NavigationParam::Previous,NavigationParam::Last, NavigationParam::Last,    NavigationParam::Last);
            },
        VK_SPACE => {
            command = choose_command(shift_key, control_key, NavigationCommand::Read,		NavigationCommand::ToggleSpeakMode,    NavigationCommand::Read,        NavigationCommand::Describe);
            param =   choose_param(  shift_key, control_key, NavigationParam::Current, NavigationParam::Last,                NavigationParam::CellCurrent, NavigationParam::Current);
            },
    
        VK_HOME => {
            command = choose_command(shift_key, control_key, NavigationCommand::Move, NavigationCommand::Move,	   NavigationCommand::Move,      NavigationCommand::ReadTo);
            param =   choose_param(  shift_key, control_key, NavigationParam::Start,NavigationParam::ColStart, NavigationParam::LineStart, NavigationParam::Start);
            },
        VK_END => {
            command = choose_command(shift_key, control_key, NavigationCommand::Move, NavigationCommand::Move,   NavigationCommand::Move,    NavigationCommand::ReadTo);
            param =   choose_param(  shift_key, control_key, NavigationParam::End,  NavigationParam::ColEnd, NavigationParam::LineEnd, NavigationParam::End);
            },
        VK_BACK => {
            command = NavigationCommand::MoveLastLocation;
            param = NavigationParam::Last;
            },
        VK_ESCAPE => {
            command = NavigationCommand::Exit;
            param = NavigationParam::Last;
            },
        0x30..=0x39 => {  // '0' ... '9'
            command = choose_command(shift_key, control_key, NavigationCommand::Move, NavigationCommand::Read, NavigationCommand::SetPlacemarker, NavigationCommand::Describe);
            static PLACE_MARKER: &[NavigationParam] = &[
                NavigationParam::Placemarker0,
                NavigationParam::Placemarker1,
                NavigationParam::Placemarker2,
                NavigationParam::Placemarker3,
                NavigationParam::Placemarker4,
                NavigationParam::Placemarker5,
                NavigationParam::Placemarker6,
                NavigationParam::Placemarker7,
                NavigationParam::Placemarker8,
                NavigationParam::Placemarker9,
            ];
            param = PLACE_MARKER[key-0x30];
        },
        _ => bail!("Unknown key press/command"),
    };
    
	return Ok( (command, param) );
}

// translate the key presses into commands


fn navigation_command_string(command: NavigationCommand, param: NavigationParam) -> &'static str {
	match command {
	    NavigationCommand::Move => {
            return match param {
                NavigationParam::Previous => "MovePrevious",
                NavigationParam::Next => "MoveNext",
                NavigationParam::Start => "MoveStart",
                NavigationParam::End => "MoveEnd",
                NavigationParam::LineStart => "MoveLineStart",
                NavigationParam::LineEnd => "MoveLineEnd",
                NavigationParam::CellPrevious => "MoveCellPrevious",
                NavigationParam::CellNext => "MoveCellNext",
                NavigationParam::CellUp => "MoveCellUp",
                NavigationParam::CellDown => "MoveCellDown",
                NavigationParam::ColStart => "MoveColumnStart",
                NavigationParam::ColEnd => "MoveColumnEnd",
                _ => {
                    if param < NavigationParam::Placemarker0 || param > NavigationParam::Placemarker9 {
                        panic!("Internal Error: Found illegal value for param of NavigationCommand::Move");
                    }
                    static MOVE_TO: &[&str] = &["MoveTo0","MoveTo1","MoveTo2","MoveTo3","MoveTo4","MoveTo5","MoveTo6","MoveTo7","MoveTo8","MoveTo9"];
                    return MOVE_TO[(param as usize) - (NavigationParam::Placemarker0 as usize)];
                }
            }
        },
        NavigationCommand::Zoom => {
            return match param {
                NavigationParam::Next => "ZoomIn",
                NavigationParam::Previous => "ZoomOut",
                NavigationParam::Start => "ZoomOutAll",
                NavigationParam::End => "ZoomInAll",
                _  => panic!("Illegal param for NavigationCommand::Zoom"),
            }
        },
        NavigationCommand::MoveLastLocation => {
            return "MoveLastLocation";
        },
        NavigationCommand::Read => {
            return match param {
                NavigationParam::Previous => "ReadPrevious",
                NavigationParam::Next => "ReadNext",
                NavigationParam::Current => "ReadCurrent",
                NavigationParam::CellCurrent => "ReadCellCurrent",
                NavigationParam::Start => "ReadStart",
                NavigationParam::End => "ReadEnd",
                NavigationParam::LineStart => "ReadLineStart",
                NavigationParam::LineEnd => "ReadLineEnd",
                _ => {
                    if param < NavigationParam::Placemarker0 || param > NavigationParam::Placemarker9 {
                        panic!("Internal Error: Found illegal value for param of NavigationCommand::Move");
                    }
                    static READ_PLACE_MARKERS: &[&str] = &["Read0","Read1","Read2","Read3","Read4","Read5","Read6","Read7","Read8","Read9"];
                    return READ_PLACE_MARKERS[(param as usize) - (NavigationParam::Placemarker0 as usize)];
                },
            }
        },
        NavigationCommand::Describe => {
            return match param {
                NavigationParam::Previous => "DescribePrevious",
                NavigationParam::Next => "DescribeNext",
                NavigationParam::Current => "DescribeCurrent",
                _ => {
                    if param < NavigationParam::Placemarker0 || param > NavigationParam::Placemarker9 {
                        panic!("Internal Error: Found illegal value for param of NavigationCommand::Describe");
                    }
                    static DESCRIBE_PLACE_MARKERS: &[&str] = &["Describe0","Describe1","Describe2","Describe3","Describe4","Describe5","Describe6","Describe7","Describe8","Describe9"];
                    return DESCRIBE_PLACE_MARKERS[(param as usize) - (NavigationParam::Placemarker0 as usize)];
                }
            }
        },
        NavigationCommand::ReadTo => {
            // FIX: implement
            return "Error";
        },
        NavigationCommand::Locate => {
            if param ==NavigationParam::Previous {
                return "WhereAmI";
            } else if param ==NavigationParam::Last {
                return "WhereAmIAll";
            }
        },
        NavigationCommand::ChangeNavMode => {
            if param ==NavigationParam::Previous {
                return "ToggleZoomLockUp";
            } else if param ==NavigationParam::Next {
                return "ToggleZoomLockDown";
            }
        },
        NavigationCommand::ToggleSpeakMode => {
            return "ToggleSpeakMode";
        },
        NavigationCommand::SetPlacemarker => {
            if param < NavigationParam::Placemarker0 || param > NavigationParam::Placemarker9 {
                panic!("Internal Error: Found illegal value for param of NavigationCommand::SetPlacemarker");
            }
            static SET_PLACE_MARKER: &[&str] = &["SetPlacemarker0","SetPlacemarker1","SetPlacemarker2","SetPlacemarker3","SetPlacemarker4","SetPlacemarker5","SetPlacemarker6","SetPlacemarker7","SetPlacemarker8","SetPlacemarker9"];
            return SET_PLACE_MARKER[(param as usize) - (NavigationParam::Placemarker0 as usize)];
        },
        NavigationCommand::Exit => {
            return "Exit";
        },
        NavigationCommand::Last => {
            return "Error";
        }
    };
    return "Error";
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::init_logger;
    use crate::interface::*;

    #[cfg(test)]
    /// Assert if result_id != '' and it doesn't match the id of the result of the move
    /// Returns the speech from the command
    fn test_command(command: &'static str, mathml: Element, result_id: &str) -> String {
        // debug!("\nCommand: {}", command);
        NAVIGATION_STATE.with(|nav_stack| {
            let (start_id, _) = nav_stack.borrow().get_navigation_mathml_id(mathml);
            match do_navigate_command_string(mathml, command) {
                Err(e) => panic!("\nStarting at '{}', '{} failed.\n{}",
                                        start_id, command, &crate::interface::errors_to_string(&e)),
                Ok(nav_speech) => {
                    let nav_speech = nav_speech.trim_end_matches(&[' ', ',', ';']);
                    // debug!("Full speech: {}", nav_speech);
                    if !result_id.is_empty() {
                        let (id, _) = nav_stack.borrow().get_navigation_mathml_id(mathml);
                        assert_eq!(result_id, id, "\nStarting at '{}', '{} failed.", start_id, command);
                    }
                    return nav_speech.to_string();
                }
            };
        })
    }

    fn init_default_prefs(mathml: &str, nav_mode_default: &str) {
        set_rules_dir(super::super::abs_rules_dir_path()).unwrap();
        set_preference("NavMode".to_string(), nav_mode_default.to_string()).unwrap();
        set_preference("NavVerbosity".to_string(), "Verbose".to_string()).unwrap();
        set_preference("AutoZoomOut".to_string(), "True".to_string()).unwrap();
        set_preference("Language".to_string(), "en".to_string()).unwrap();
        set_preference("SpeechStyle".to_string(), "SimpleSpeak".to_string()).unwrap();
        set_preference("Verbosity".to_string(), "Medium".to_string()).unwrap();
        set_preference("Overview".to_string(), "False".to_string()).unwrap();
        set_mathml(mathml.to_string()).unwrap();
    }

    #[test]
    fn zoom_in() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomIn", mathml, "msup");
            test_command("ZoomIn", mathml, "base");
            test_command("ZoomIn", mathml, "base");
            return Ok( () );
        });
    }

    #[test]
    fn test_init_navigate_move_right() -> Result<()> {
        // this is how navigation typically starts up
        let mathml_str = " <math display='block' id='id-0'>
            <mrow id='id-1'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mo id='id-3'>=</mo>
                <mrow id='id-4'>
                    <mi id='id-5'>a</mi>
                    <mo id='id-6'>-</mo>
                    <mn id='id-7'>2</mn>
                </mrow>
            </mrow>
        </math>";
        init_default_prefs(mathml_str, "Enhanced");
        debug!("--- Enhanced ---");
        MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomIn", mathml, "msup");
            test_command("MoveNext", mathml, "id-3");
        });

        init_default_prefs(mathml_str, "Simple");
        debug!("--- Simple ---");
        MATHML_INSTANCE.with(|package_instance: &RefCell<Package>| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomIn", mathml, "msup");
            test_command("MoveNext", mathml, "id-3");
        });
        
        init_default_prefs(mathml_str, "Character");
        debug!("--- Character ---");
        MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomIn", mathml, "base");
            test_command("MoveNext", mathml, "exp");
        });
        return Ok( () );
    }
    
    #[test]
    fn zoom_in_parens() -> Result<()> {
        // (a+b)(c+d) + 1
        let mathml_str = " <math display='block' id='id-0'>
            <mrow id='id-1'>
                <mrow id='id-2'>
                    <mrow id='id-3'>
                    <mo stretchy='false' id='id-4'>(</mo>
                    <mrow id='id-5'>
                        <mi id='id-6'>a</mi>
                        <mo id='id-7'>+</mo>
                        <mi id='id-8'>b</mi>
                    </mrow>
                    <mo stretchy='false' id='id-9'>)</mo>
                    </mrow>
                    <mo id='id-10'>&#x2062;</mo>
                    <mrow id='id-11'>
                    <mo stretchy='false' id='id-12'>(</mo>
                    <mrow id='id-13'>
                        <mi id='id-14'>c</mi>
                        <mo id='id-15'>+</mo>
                        <mi id='id-16'>d</mi>
                    </mrow>
                    <mo stretchy='false' id='id-17'>)</mo>
                    </mrow>
                </mrow>
                <mo id='id-18'>+</mo>
                <mn id='id-19'>1</mn>
            </mrow>
        </math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            set_preference("NavMode".to_string(), "Enhanced".to_string())?;
            debug!("\n------EnhancedMode----------");
            test_command("ZoomIn", mathml, "id-2");
            test_command("ZoomIn", mathml, "id-5");
            test_command("ZoomIn", mathml, "id-6");
            
            // repeat, but this time with "Simple
            set_preference("NavMode".to_string(), "Simple".to_string())?;
            debug!("\n------SimpleMode----------");
            test_command("ZoomOutAll", mathml, "id-1");
            test_command("ZoomIn", mathml, "id-4");
            test_command("ZoomIn", mathml, "id-4");
            return Ok( () );
        });
    }
    
    #[test]
    fn zoom_in_all() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomInAll", mathml, "base");
            return Ok( () );
        });
    }

    
    #[test]
    fn zoom_out() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
            init_default_prefs(mathml_str, "Enhanced");
            return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "base".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            test_command("ZoomOut", mathml, "msup");

            let _nav_speech = do_navigate_command_and_param(mathml, NavigationCommand::Zoom, NavigationParam::Previous)?;
            NAVIGATION_STATE.with(|nav_stack| {
                let (id, _) = nav_stack.borrow().get_navigation_mathml_id(mathml);
                assert_eq!(id, "mfrac");
            });
            return Ok( () );
        });
    }
    
    #[test]
    fn zoom_out_all() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
            init_default_prefs(mathml_str, "Enhanced");
            return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "base".to_string(),
                    current_node_offset: 0
                }, "None")
            });

            test_command("ZoomOutAll", mathml, "mfrac");
            return Ok( () );
        });
    }
    
    #[test]
    fn move_start_end() -> Result<()> {
        let mathml_str = " <math display='block' id='id-0'>
        <mrow id='id-1'>
          <mi id='id-2'>x</mi>
          <mo id='id-3'>=</mo>
          <mrow id='id-4'>
            <mi id='id-5'>a</mi>
            <mo id='id-6'>-</mo>
            <mn id='id-7'>2</mn>
          </mrow>
        </mrow>
       </math>";
       init_default_prefs(mathml_str, "Enhanced");
       return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "id-4".to_string(),
                    current_node_offset: 0
                }, "None")
            });

            set_preference("NavMode".to_string(), "Character".to_string())?;
            test_command("MoveStart", mathml, "id-2");
            test_command("MoveEnd", mathml, "id-7");
            set_preference("NavMode".to_string(), "Simple".to_string())?;
            test_command("MoveStart", mathml, "id-2");
            test_command("MoveEnd", mathml, "id-7");
            set_preference("NavMode".to_string(), "Enhanced".to_string())?;
            test_command("MoveStart", mathml, "id-2");
            test_command("MovePrevious", mathml, "id-2");
            test_command("MoveEnd", mathml, "id-4");
            test_command("MoveNext", mathml, "id-4");
            return Ok( () );
        });
    }
    
    #[test]
    fn move_line_start_end() -> Result<()> {
        let mathml_str = " <math display='block' id='id-0'>
        <mfrac displaystyle='true' id='id-1'>
          <mi id='id-2'>x</mi>
          <mrow id='id-3'>
            <msup id='id-4'>
              <mi id='id-5'>y</mi>
              <mn id='id-6'>2</mn>
            </msup>
            <mo id='id-7'>+</mo>
            <mn id='id-8'>1</mn>
          </mrow>
        </mfrac>
       </math>";
       init_default_prefs(mathml_str, "Enhanced");
       return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "id-7".to_string(),
                    current_node_offset: 0
                }, "None")
            });

            set_preference("NavMode".to_string(), "Character".to_string())?;
            test_command("MoveLineStart", mathml, "id-5");
            test_command("MoveLineEnd", mathml, "id-8");
            set_preference("NavMode".to_string(), "Simple".to_string())?;
            test_command("MoveLineStart", mathml, "id-4");
            test_command("MoveLineEnd", mathml, "id-8");
            set_preference("NavMode".to_string(), "Enhanced".to_string())?;
            test_command("MoveLineStart", mathml, "id-4");
            test_command("MoveLineEnd", mathml, "id-8");
            test_command("MoveEnd", mathml, "id-3");
            return Ok( () );
        });
    }
    
    #[test]
    fn text_extremes_and_move_last_location() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
            init_default_prefs(mathml_str, "Enhanced");
            return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "base".to_string(),
                    current_node_offset: 0
                }, "None")
            });

            test_command("ZoomOutAll", mathml, "mfrac");
            test_command("ZoomOut", mathml, "mfrac");
            test_command("MoveLastLocation", mathml, "base");       // second zoom out should do nothing

            test_command("ZoomOut", mathml, "msup");
            test_command("ZoomInAll", mathml, "base");
            test_command("ZoomIn", mathml, "base");
            test_command("MoveLastLocation", mathml, "msup");       // second zoom in should do nothing

            return Ok( () );
        });
    }
    
    #[test]
    fn move_to_start() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <mrow id='num'><msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup><mo id='factorial'>!</mo></mrow>
                <mi id='denom'>d</mi>
            </mfrac></math>";
            init_default_prefs(mathml_str, "Enhanced");
            return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "denom".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            test_command("MoveLineStart", mathml, "denom");

            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "factorial".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            test_command("MoveLineStart", mathml, "msup");

            let _nav_speech = do_navigate_command_and_param(mathml, NavigationCommand::Move, NavigationParam::Start)?;
            NAVIGATION_STATE.with(|nav_stack| {
                let (id, _) = nav_stack.borrow().get_navigation_mathml_id(mathml);
                assert_eq!(id, "num");
            });
            return Ok( () );
        });
    }
    
    #[test]
    fn move_right_sup() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
        <mrow id='id-1'>
          <msup id='id-2'>
            <mn id='id-3'>2</mn>
            <mi id='id-4'>q</mi>
          </msup>
          <mo id='id-5'>-</mo>
          <mi id='id-6'>x</mi>
        </mrow>
        </math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "id-2".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            set_preference("NavMode".to_string(), "Enhanced".to_string())?;
            test_command("MoveNext", mathml, "id-5");

            // reset start and test Simple
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "id-2".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            set_preference("NavMode".to_string(), "Simple".to_string())?;
            test_command("MoveNext", mathml, "id-5");

            // reset start and test Character
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "id-3".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            set_preference("NavMode".to_string(), "Character".to_string())?;
            test_command("MoveNext", mathml, "id-4");
            test_command("MoveNext", mathml, "id-5");
            return Ok( () );
        });
    }

        
    #[test]
    fn move_msubsup_char() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
        <mrow id='id-1'>
          <mn id='id-2'>1</mn>
          <mo id='id-3'>+</mo>
          <msubsup id='id-4'>
            <mi id='id-5'>x</mi>
            <mn id='id-6'>2</mn>
            <mn id='id-7'>3</mn>
          </msubsup>
          <mo id='id-8'>+</mo>
          <mn id='id-9'>4</mn>
        </mrow>
       </math>";
        init_default_prefs(mathml_str, "Character");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            assert_eq!("zoomed in all the way; 1", test_command("ZoomInAll", mathml, "id-2"));
            assert_eq!("move right; plus", test_command("MoveNext", mathml, "id-3"));
            assert_eq!("move right; in base; x", test_command("MoveNext", mathml, "id-5"));
            assert_eq!("move right; in subscript; 2", test_command("MoveNext", mathml, "id-6"));
            assert_eq!("move right; in superscript; 3", test_command("MoveNext", mathml, "id-7"));
            assert_eq!("move right; out of superscript; plus", test_command("MoveNext", mathml, "id-8"));
            assert_eq!("move left; in superscript; 3", test_command("MovePrevious", mathml, "id-7"));
            assert_eq!("move left; in subscript; 2", test_command("MovePrevious", mathml, "id-6"));
            assert_eq!("move left; in base; x", test_command("MovePrevious", mathml, "id-5"));
            assert_eq!("move left; out of base; plus", test_command("MovePrevious", mathml, "id-3"));

            return Ok( () );
        });
    }
        
    #[test]
    fn move_mmultiscripts_char() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
            <mmultiscripts data-mjx-texclass='ORD' data-chem-formula='5' id='id-1'>
                <mrow data-chem-formula='3' id='id-2'>
                    <mo stretchy='false' id='id-3'>[</mo>
                    <mmultiscripts data-chem-formula='3' id='id-4'>
                        <mi data-chem-element='3' id='id-5'>Co</mi>
                        <mn id='id-6'>6</mn>
                        <none id='id-7'></none>
                    </mmultiscripts>
                    <mo stretchy='false' id='id-8'>]</mo>
                </mrow>
                <none id='id-9'></none>
                <mrow id='id-10'>
                    <mn id='id-11'>3</mn>
                    <mo id='id-12'>+</mo>
                </mrow>
            </mmultiscripts>
            </math>";
            init_default_prefs(mathml_str, "Character");
            return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            assert_eq!("zoomed in all the way; in base; open bracket", test_command("ZoomInAll", mathml, "id-3"));
            assert_eq!("move right; in base; cap c o", test_command("MoveNext", mathml, "id-5"));
            assert_eq!("move right; in subscript; 6", test_command("MoveNext", mathml, "id-6"));
            assert_eq!("move right; out of subscript; close bracket", test_command("MoveNext", mathml, "id-8"));
            assert_eq!("move right; in superscript; 3", test_command("MoveNext", mathml, "id-11"));
            assert_eq!("move right; plus", test_command("MoveNext", mathml, "id-12"));
            assert_eq!("cannot move right, end of math", test_command("MoveNext", mathml, "id-12"));
            assert_eq!("move left; 3", test_command("MovePrevious", mathml, "id-11"));
            assert_eq!("move left; in base; close bracket", test_command("MovePrevious", mathml, "id-8"));
            assert_eq!("move left; in subscript; 6", test_command("MovePrevious", mathml, "id-6"));
            assert_eq!("move left; in base; cap c o", test_command("MovePrevious", mathml, "id-5"));
            assert_eq!("move left; out of base; open bracket", test_command("MovePrevious", mathml, "id-3"));

            return Ok( () );
        });
    }

    #[test]
    fn move_right_char() -> Result<()> {
        let mathml_str = "<math id='id-0'>
        <mrow displaystyle='true' id='id-1'>
          <mi id='id-2'>x</mi>
          <mo id='id-3'>=</mo>
          <mrow id='id-4'>
            <mfrac id='id-5'>
              <mn id='id-6'>1</mn>
              <mrow id='id-7'>
                <mi id='id-8'>a</mi>
                <mo id='id-9'>+</mo>
                <mn id='id-10'>2</mn>
              </mrow>
            </mfrac>
            <mo id='id-11'>+</mo>
            <mrow id='id-12'>
              <mn id='id-13'>3</mn>
              <mo id='id-14'>&#x2062;</mo>
              <mi id='id-15'>b</mi>
            </mrow>
          </mrow>
        </mrow>
        </math>";
        init_default_prefs(mathml_str, "Character");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomInAll", mathml, "id-2");
            test_command("MoveNext", mathml, "id-3");
            test_command("MoveNext", mathml, "id-6");
            test_command("MoveNext", mathml, "id-8");
            test_command("MoveNext", mathml, "id-9");
            test_command("MoveNext", mathml, "id-10");
            test_command("MoveNext", mathml, "id-11");
            test_command("MoveNext", mathml, "id-13");
            test_command("MoveNext", mathml, "id-15");
            test_command("MoveNext", mathml, "id-15");

            return Ok( () );
        });
    }

    #[test]
    fn char_mode_paren_test() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
            <mrow displaystyle='true' id='id-1'>
                <mrow id='id-2'>
                    <mo id='id-3'>(</mo>
                    <mi id='id-4'>a</mi>
                    <mo id='id-5'>)</mo>
                </mrow>
                <mo id='id-6'>&#x2062;</mo>
                <mrow id='id-7'>
                    <mo id='id-8'>(</mo>
                    <mi id='id-9'>b</mi>
                    <mo id='id-10'>)</mo>
                </mrow>
            </mrow>
        </math>";
        init_default_prefs(mathml_str, "Character");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            debug!("Character mode");
            do_commands(mathml)?;
            set_preference("NavMode".to_string(), "Simple".to_string()).unwrap();
            debug!("Simple mode");
            test_command("ZoomIn", mathml, "id-3");  // zooms to the first parenthesis
            do_commands(mathml)?;
            set_preference("NavMode".to_string(), "Enhanced".to_string()).unwrap();
            debug!("Enhanced mode");
            test_command("ZoomIn", mathml, "id-4");
            test_command("MoveNext", mathml, "id-6");
            test_command("MoveNext", mathml, "id-9");
            test_command("MovePrevious", mathml, "id-6");
            test_command("MovePrevious", mathml, "id-4");

            return Ok( () );
        });

        /// Simple and Character mode should behave the same
        fn do_commands(mathml: Element) -> Result<()> {
            test_command("ZoomIn", mathml, "id-3");
            test_command("MoveNext", mathml, "id-4");
            test_command("MoveNext", mathml, "id-5");
            test_command("MoveNext", mathml, "id-8");
            test_command("MoveNext", mathml, "id-9");
            test_command("MoveNext", mathml, "id-10");
            test_command("MovePrevious", mathml, "id-9");
            test_command("MovePrevious", mathml, "id-8");
            test_command("MovePrevious", mathml, "id-5");
            test_command("ZoomOutAll", mathml, "id-1");
            return Ok( () );
        }
    }

    #[test]
    fn char_mode_trig_test() -> Result<()> {
        let mathml_str = "<math id='id-0'>
            <mrow id='id-1'>
            <mi id='id-2'>sin</mi>
            <mo id='id-3'>&#x2061;</mo>
            <mrow id='id-4'>
                <mo id='id-5'>(</mo>
                <mi id='id-6'>x</mi>
                <mo id='id-7'>)</mo>
            </mrow>
            </mrow>
        </math>";
        init_default_prefs(mathml_str, "Character");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            do_commands(mathml)?;
            set_preference("NavMode".to_string(), "Simple".to_string()).unwrap();
            do_commands(mathml)?;
            set_preference("NavMode".to_string(), "Enhanced".to_string()).unwrap();
            test_command("ZoomIn", mathml, "id-2");
            test_command("MoveNext", mathml, "id-6");
            test_command("MovePrevious", mathml, "id-2");

            return Ok( () );
        });

        
        /// Simple and Character mode should behave the same
        fn do_commands(mathml: Element) -> Result<()> {
            test_command("ZoomIn", mathml, "id-2");
            test_command("MoveNext", mathml, "id-5");
            test_command("MoveNext", mathml, "id-6");
            test_command("MoveNext", mathml, "id-7");
            test_command("MovePrevious", mathml, "id-6");
            test_command("MovePrevious", mathml, "id-5");
            test_command("MovePrevious", mathml, "id-2");
            return Ok( () );
        }
    }
    
    #[test]
    fn move_char_speech() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
                <mrow id='id-1'>
                <mfrac id='id-2'>
                    <mi id='id-3'>x</mi>
                    <mi id='id-4'>y</mi>
                </mfrac>
                <mo id='id-5'>&#x2062;</mo>
                <mi id='id-6'>z</mi>
                </mrow>
            </math>";
            init_default_prefs(mathml_str, "Character");
            return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomInAll", mathml, "id-3");
            assert_eq!("move right; in denominator; y", test_command("MoveNext", mathml, "id-4"));
            assert_eq!("move right; out of denominator; z", test_command("MoveNext", mathml, "id-6"));
            assert_eq!("move left; in denominator; y", test_command("MovePrevious", mathml, "id-4"));
            assert_eq!("move left; in numerator; x", test_command("MovePrevious", mathml, "id-3"));

            return Ok( () );
        });
    }
    
    #[test]
    fn move_enhanced_times() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
        <mrow displaystyle='true' id='id-1'>
          <mn id='id-2'>2</mn>
          <mo id='id-3'>&#x2062;</mo>
          <mrow id='id-4'>
            <mo id='id-5'>(</mo>
            <mrow id='id-6'>
              <mn id='id-7'>1</mn>
              <mo id='id-8'>-</mo>
              <mi id='id-9'>x</mi>
            </mrow>
            <mo id='id-10'>)</mo>
          </mrow>
        </mrow>
       </math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomIn", mathml, "id-2");
            assert_eq!("move right; times", test_command("MoveNext", mathml, "id-3"));
            assert_eq!("move right; 1 minus x", test_command("MoveNext", mathml, "id-6"));
            assert_eq!("move left; times", test_command("MovePrevious", mathml, "id-3"));
            assert_eq!("move left; 2", test_command("MovePrevious", mathml, "id-2"));

            return Ok( () );
        });
    }
    
    #[test]
    fn move_simple_no_times() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
        <mrow displaystyle='true' id='id-1'>
          <mn id='id-2'>2</mn>
          <mo id='id-3'>&#x2062;</mo>
          <mrow id='id-4'>
            <mo id='id-5'>(</mo>
            <mrow id='id-6'>
              <mn id='id-7'>1</mn>
              <mo id='id-8'>-</mo>
              <mi id='id-9'>x</mi>
            </mrow>
            <mo id='id-10'>)</mo>
          </mrow>
        </mrow>
       </math>";
        init_default_prefs(mathml_str, "Simple");
        set_preference("SpeechStyle".to_string(), "ClearSpeak".to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomIn", mathml, "id-2");
            assert_eq!("move right; open paren", test_command("MoveNext", mathml, "id-5"));
            assert_eq!("move right; 1", test_command("MoveNext", mathml, "id-7"));
            assert_eq!("move left; open paren", test_command("MovePrevious", mathml, "id-5"));
            assert_eq!("move left; 2", test_command("MovePrevious", mathml, "id-2"));

            return Ok( () );
        });
    }
    
    
    #[test]
    fn move_cell() -> Result<()> {
        let mathml_str = "<math id='nav-0'>
        <mtable id='nav-1'>
          <mtr id='nav-2'>
            <mtd id='nav-3'> <mn id='nav-4'>1</mn></mtd>
            <mtd id='nav-5'> <mn id='nav-6'>2</mn></mtd>
            <mtd id='nav-7'><mn id='nav-8'>3</mn> </mtd>
          </mtr>
          <mtr id='nav-9'>
            <mtd id='nav-10'>
              <mrow id='nav-11'>
                <mi id='nav-12'>x</mi>
                <mo id='nav-13'>-</mo>
                <mi id='nav-14'>y</mi>
              </mrow>
            </mtd>
            <mtd id='nav-15'>
              <mfrac id='nav-16'>
                <mn id='nav-17'>1</mn>
                <mn id='nav-18'>2</mn>
              </mfrac>
            </mtd>
            <mtd id='nav-19'>
              <mi id='nav-20'>z</mi>
            </mtd>
          </mtr>
          <mtr id='nav-21'>
            <mtd id='nav-22'><mn id='nav-23'>7</mn> </mtd>
            <mtd id='nav-24'><mn id='nav-25'>8</mn> </mtd>
            <mtd id='nav-26'> <mn id='nav-27'>9</mn></mtd>
          </mtr>
          <mtr id='nav-28'>
            <mtd id='nav-29'>
              <mrow id='nav-30'>
                <mi id='nav-31'>sin</mi>
                <mo id='nav-32'>&#x2061;</mo>
                <mi id='nav-33'>x</mi>
              </mrow>
            </mtd>
            <mtd id='nav-34'>
              <msup id='nav-35'>
                <mi id='nav-36'>e</mi>
                <mi id='nav-37'>x</mi>
              </msup>
            </mtd>
            <mtd id='nav-38'>
              <mrow id='nav-39'>
                <mn id='nav-40'>2</mn>
                <mo id='nav-41'>-</mo>
                <mi id='nav-42'>y</mi>
              </mrow>
            </mtd>
          </mtr>
        </mtable>
       </math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomInAll", mathml, "nav-4");
            test_command("MoveCellNext", mathml, "nav-6");
            test_command("MoveCellNext", mathml, "nav-8");
            test_command("MoveCellNext", mathml, "nav-8");
            test_command("MoveCellDown", mathml, "nav-20");
            test_command("MoveCellDown", mathml, "nav-27");
            let speech = test_command("MoveCellDown", mathml, "nav-39");
            assert_eq!(speech, "move down, row 4, column 3; 2 minus y");
            let speech = test_command("MoveCellDown", mathml, "nav-39");
            assert_eq!(speech, "no next row");
            test_command("MoveCellPrevious", mathml, "nav-35");
            test_command("ZoomIn", mathml, "nav-36");
            test_command("MoveCellUp", mathml, "nav-25");
            test_command("MoveCellUp", mathml, "nav-16");
            test_command("MoveCellUp", mathml, "nav-6");
            test_command("MoveCellUp", mathml, "nav-6");

            return Ok( () );
        });
    }
    
    #[test]
    fn move_cell_char_mode() -> Result<()> {
        let mathml_str = "<math id='nav-0'>
        <mtable id='nav-1'>
          <mtr id='nav-2'>
            <mtd id='nav-3'> <mn id='nav-4'>1</mn></mtd>
            <mtd id='nav-5'> <mn id='nav-6'>2</mn></mtd>
            <mtd id='nav-7'><mn id='nav-8'>3</mn> </mtd>
          </mtr>
          <mtr id='nav-9'>
            <mtd id='nav-10'>
              <mrow id='nav-11'>
                <mi id='nav-12'>x</mi>
                <mo id='nav-13'>-</mo>
                <mi id='nav-14'>y</mi>
              </mrow>
            </mtd>
            <mtd id='nav-15'>
              <mfrac id='nav-16'>
                <mn id='nav-17'>1</mn>
                <mn id='nav-18'>2</mn>
              </mfrac>
            </mtd>
            <mtd id='nav-19'>
              <mi id='nav-20'>z</mi>
            </mtd>
          </mtr>
          <mtr id='nav-21'>
            <mtd id='nav-22'><mn id='nav-23'>7</mn> </mtd>
            <mtd id='nav-24'><mn id='nav-25'>8</mn> </mtd>
            <mtd id='nav-26'> <mn id='nav-27'>9</mn></mtd>
          </mtr>
          <mtr id='nav-28'>
            <mtd id='nav-29'>
              <mrow id='nav-30'>
                <mi id='nav-31'>sin</mi>
                <mo id='nav-32'>&#x2061;</mo>
                <mi id='nav-33'>x</mi>
              </mrow>
            </mtd>
            <mtd id='nav-34'>
              <msup id='nav-35'>
                <mi id='nav-36'>e</mi>
                <mi id='nav-37'>x</mi>
              </msup>
            </mtd>
            <mtd id='nav-38'>
              <mrow id='nav-39'>
                <mn id='nav-40'>2</mn>
                <mo id='nav-41'>-</mo>
                <mi id='nav-42'>y</mi>
              </mrow>
            </mtd>
          </mtr>
        </mtable>
       </math>";
       init_default_prefs(mathml_str, "Character");
       return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "nav-8".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            test_command("MoveNext", mathml, "nav-12");
            test_command("MoveNext", mathml, "nav-13");
            test_command("MoveNext", mathml, "nav-14");
            test_command("MoveNext", mathml, "nav-17");
            test_command("MovePrevious", mathml, "nav-14");
            test_command("MoveCellNext", mathml, "nav-17");
            test_command("MoveCellPrevious", mathml, "nav-14");
            test_command("MovePrevious", mathml, "nav-13");
            test_command("MovePrevious", mathml, "nav-12");
            test_command("MoveCellPrevious", mathml, "nav-12");
            test_command("MovePrevious", mathml, "nav-8");
            test_command("MoveCellDown", mathml, "nav-20");
            test_command("MoveCellDown", mathml, "nav-27");
            test_command("MoveCellDown", mathml, "nav-40");
            test_command("MoveCellDown", mathml, "nav-40");
            test_command("MoveCellPrevious", mathml, "nav-37");
            test_command("MoveCellUp", mathml, "nav-25");

            return Ok( () );
        });
    }
    
    #[test]
    fn placemarker() -> Result<()> {
        let mathml_str = "<math display='block' id='math'>
        <mrow displaystyle='true' id='mrow'>
          <mi id='a'>a</mi>
          <mo id='plus-1'>+</mo>
          <mi id='b'>b</mi>
          <mo id='plus-2'>+</mo>
          <mi id='c'>c</mi>
        </mrow>
        </math>";
        init_default_prefs(mathml_str, "Character");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("MoveStart", mathml, "a");
            test_command("SetPlacemarker0", mathml, "a");
            test_command("MoveEnd", mathml, "c");
            test_command("Read0", mathml, "c");
            test_command("Describe0", mathml, "c");
            test_command("SetPlacemarker1", mathml, "c");
            test_command("MoveTo0", mathml, "a");
            test_command("MoveTo1", mathml, "c");
            test_command("MoveLastLocation", mathml, "a");
            
            return Ok( () );
        });
    }

    #[test]
    fn where_am_i_all() -> Result<()> {
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        init_default_prefs(mathml_str, "Enhanced");
        set_preference("SpeechStyle".to_string(), "ClearSpeak".to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "exp".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            // WhereAmIAll doesn't change the stack
            let speech =test_command("WhereAmIAll", mathml, "exp");
            // should be 2 "inside" strings corresponding to steps to the root
            assert_eq!(speech, "2; inside; b squared; inside; the fraction with numerator; b squared; and denominator d");
            return Ok( () );
        });
    }

    #[test]
    fn auto_zoom_out_mrow() -> Result<()> {
        let mathml_str = "<math id='math'>
        <mrow id='id-1'>
          <mrow id='id-2'>
            <mrow id='2ax'>
              <mn id='2'>2</mn>
              <mo id='id-5'>&#x2062;</mo>
              <mi id='a'>a</mi>
              <mo id='id-7'>&#x2062;</mo>
              <mi id='x'>x</mi>
            </mrow>
            <mo id='plus'>+</mo>
            <mi id='b'>b</mi>
          </mrow>
          <mo id='equal'>=</mo>
          <mn id='10'>10</mn>
        </mrow>
       </math>";
        init_default_prefs(mathml_str, "Enhanced");
        set_preference("AutoZoomOut".to_string(), "False".to_string())?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomInAll", mathml, "2");
            test_command("MoveNext", mathml, "a");
            test_command("MoveNext", mathml, "x");
            test_command("MoveNext", mathml, "plus");
            test_command("MovePrevious", mathml, "2ax");
            return Ok( () );
        });
    }

    #[test]
    fn auto_zoom_out_fraction() -> Result<()> {
        let mathml_str = "<math id='math'>
            <mrow id='mrow'>
                <mfrac id='frac'>
                    <mrow id='num'><mi id='a'>a</mi><mo id='plus'>+</mo><mn id='1'>1</mn></mrow>
                    <mrow id='denom'><mn id='2'>2</mn><mo id='invisible-times'>&#x2062;</mo><mi id='b'>b</mi></mrow>
                </mfrac>
                <mo id='minus'>-</mo>
                <mn id='3'>3</mn>
            </mrow>
        </math>";
        init_default_prefs(mathml_str, "Enhanced");
        set_preference("AutoZoomOut".to_string(), "False".to_string())?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomIn", mathml, "frac");
            test_command("ZoomIn", mathml, "num");
            test_command("MoveNext", mathml, "denom");
            test_command("MoveNext", mathml, "denom");
            test_command("MovePrevious", mathml, "num");
            test_command("MovePrevious", mathml, "num");
            test_command("ZoomOut", mathml, "frac");
            test_command("MoveNext", mathml, "minus");
            return Ok( () );
        });
    }

    #[test]
    fn zoom_root() -> Result<()> {
        let mathml_str = r#"<math display='block' id='id-0'>
        <mrow id='id-1'>
            <mo id='id-9'>±</mo>
            <msqrt id='id-10'>
                <mrow id='id-11'>
                    <msup id='id-12'> <mi id='id-13'>b</mi> <mn id='id-14'>2</mn> </msup>
                    <mo id='id-15'>-</mo>
                    <mn id='id-17'>4</mn>
                </mrow>
            </msqrt>
        </mrow>
        </math>"#;

        test_mode(mathml_str, "Enhanced")?;
        test_mode(mathml_str, "Simple")?;
        test_mode(mathml_str, "Character")?;
        return Ok( () );

        fn test_mode(mathml_str: &str, mode: &str) -> Result<()> {
            init_default_prefs(mathml_str, mode);
            set_preference("AutoZoomOut".to_string(), "False".to_string())?;
            return MATHML_INSTANCE.with(|package_instance| {
                debug!("--- Testing mode {mode} ---");
                let package_instance = package_instance.borrow();
                let mathml = get_element(&*package_instance);
                test_command("ZoomIn", mathml, "id-9");
                debug!("\nStart zoom in");
                match mode {
                    "Enhanced" => {
                        test_command("MoveNext", mathml, "id-10");
                        let speech = test_command("ZoomIn", mathml, "id-11");
                        assert_eq!(speech, "zoom in; in root; b squared minus 4");  // only one arg, so don't say "in root"
                        let speech = test_command("ZoomIn", mathml, "id-12");
                        assert_eq!(speech, "zoom in; b squared");  // only one arg, so don't say "in root"
                        let speech = test_command("ZoomIn", mathml, "id-13");
                        assert_eq!(speech, "zoom in; in base; b");
                    },
                    "Simple" => {
                        test_command("MoveNext", mathml, "id-10");
                        let speech = test_command("ZoomIn", mathml, "id-12");
                        assert_eq!(speech, "zoom in; in root; b squared");
                        let speech = test_command("ZoomIn", mathml, "id-13");
                        assert_eq!(speech, "zoom in; in base; b");
                    },
                    _ => { // "Character"
                        let speech = test_command("MoveNext", mathml, "id-13");
                        assert_eq!(speech, "move right; in root; in base; b");
                    }
                }
                let squared_speech = if mode == "Character" {"b super 2 end super"} else {"b squared"};
                let sqrt_speech = if mode == "Character" {"root"} else {"square root"};
                let speech = test_command("ZoomOut", mathml, "id-12");
                assert_eq!(speech, format!("zoom out; out of base; {squared_speech}"));
                let speech = test_command("ZoomOut", mathml, "id-11");
                assert_eq!(speech, format!("zoom out; {squared_speech} minus 4"));
                let speech = test_command("ZoomOut", mathml, "id-10");
                assert_eq!(speech, format!("zoom out; out of root; the {sqrt_speech} of {squared_speech} minus 4, end root",));
                return Ok( () );
            });
        }
    }

    #[test]
    fn matrix_speech() -> Result<()> {
        let mathml_str = r#"<math id='math'>
            <mrow id='mrow'>
            <mo id='open'>[</mo>
            <mtable columnspacing='1em' rowspacing='4pt' id='table'>
                <mtr id='row-1'>
                    <mtd id='1-1'><mn id='id-6'>9</mn></mtd>
                    <mtd id='1-2'><mrow id='id-8'><mo id='id-9'>-</mo><mn id='id-10'>13</mn></mrow></mtd>
                </mtr>
                <mtr id='row-2'>
                    <mtd id='2-1'><mn id='id-13'>5</mn></mtd>
                    <mtd id='2-2'><mo id='id-16'>-</mo><mn id='id-17'>6</mn></mtd>
                </mtr>
            </mtable>
            <mo id='close'>]</mo>
            </mrow>
        </math>"#;
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomIn", mathml, "row-1");
            let speech = test_command("MoveNext", mathml, "row-2");
            assert_eq!(speech, "move right; row 2; 5, negative 6");
            let speech = test_command("ZoomIn", mathml, "id-13");
            assert_eq!(speech, "zoom in; column 1; 5");
            let speech = test_command("ZoomOut", mathml, "row-2");
            assert_eq!(speech, "zoom out; row 2; 5, negative 6");
            let speech = test_command("ZoomOut", mathml, "table");
            assert_eq!(speech, "zoom out; the 2 by 2 matrix; row 1; 9, negative 13; row 2; 5, negative 6");
        return Ok( () );
        });
    }

    #[test]
    fn chem_speech() -> Result<()> {
        // this comes from bug 218
        let mathml_str = "<math display='block' id='id-0'>
            <mrow data-chem-formula='5' id='id-1'>
                <msub data-chem-formula='1' id='id-2'>
                    <mi data-chem-element='1' id='id-3'>H</mi>
                    <mn id='id-4'>2</mn>
                </msub>
                <mo data-chem-formula-op='0' id='id-5'>&#x2063;</mo>
                <mi data-chem-element='1' id='id-6'>S</mi>
                <mo data-chem-formula-op='0' id='id-7'>&#x2063;</mo>
                <msub data-chem-formula='1' id='id-8'>
                    <mi data-chem-element='1' id='id-9'>O</mi>
                    <mn id='id-10'>4</mn>
                </msub>
            </mrow>
        </math>";
        init_default_prefs(mathml_str, "Enhanced");
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomIn", mathml, "id-2");
            let speech = test_command("MoveNext", mathml, "id-6");
            // tables need to check their parent for proper speech
            assert_eq!(speech, "move right; cap s");
            return Ok( () );
        });
    }

    #[test]
    fn determinant_speech() -> Result<()> {
        let mathml_str = "<math id='math'>
            <mrow id='mrow'>
            <mo id='open'>|</mo>
            <mtable columnspacing='1em' rowspacing='4pt' id='table'>
                <mtr id='row-1'>
                    <mtd id='1-1'><mn id='id-6'>9</mn></mtd>
                    <mtd id='1-2'><mrow id='id-8'><mo id='id-9'>-</mo><mn id='id-10'>13</mn></mrow></mtd>
                </mtr>
                <mtr id='row-2'>
                    <mtd id='2-1'><mn id='id-13'>5</mn></mtd>
                    <mtd id='2-2'><mrow id='row2-negative'><mo id='id-16'>-</mo><mn id='id-17'>6</mn></mrow></mtd>
                </mtr>
            </mtable>
            <mo id='close'>|</mo>
            </mrow>
        </math>";
        init_default_prefs(mathml_str, "Enhanced");
        set_preference("SpeechStyle".to_string(), "ClearSpeak".to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            let speech = test_command("ZoomIn", mathml, "row-1");
            assert_eq!(speech, "zoom in; row 1; 9, negative 13");
            let speech = test_command("MoveNext", mathml, "row-2");
            assert_eq!(speech, "move right; row 2; 5, negative 6");
            let speech = test_command("MoveNext", mathml, "row-2");
            assert_eq!(speech, "cannot move right, end of math");
            let speech = test_command("ZoomIn", mathml, "id-13");
            assert_eq!(speech, "zoom in; column 1; 5");
            let speech = test_command("MoveNext", mathml, "row2-negative");
            assert_eq!(speech, "move right; column 2, negative 6");
            let speech = test_command("ZoomOutAll", mathml, "table");
            assert_eq!(speech, "zoomed out all the way; the 2 by 2 determinant; row 1; 9, negative 13; row 2; 5, negative 6");
            return Ok( () );
        });
    }

    #[test]
    fn cases_speech() -> Result<()> {
        let mathml_str = "<math id='id-0'>
        <mrow id='id-1'>
          <mo id='open'>{</mo>
          <mtable columnalign='left left' columnspacing='1em' displaystyle='false' rowspacing='.2em' id='table'>
            <mtr id='row-1'>
              <mtd id='id-5'><mrow id='id-6'><mrow id='id-7'><mo id='id-8'>-</mo><mi id='id-9'>x</mi></mrow><mo id='id-10'>,</mo></mrow></mtd>
              <mtd id='id-11'><mrow id='id-12'><mrow id='id-13'><mtext id='id-14'>if</mtext><mo id='id-15'>&#x2062;</mo><mi id='id-16'>x</mi></mrow><mo id='id-17'>&lt;</mo><mn id='id-18'>0</mn></mrow></mtd>
            </mtr>
            <mtr id='row-2'>
              <mtd id='id-20'><mrow id='id-21'><mrow id='id-22'><mo id='id-23'>+</mo><mi id='id-24'>x</mi></mrow><mo id='id-25'>,</mo></mrow></mtd>
              <mtd id='id-26'><mrow id='id-27'><mrow id='id-28'><mtext id='id-29'>if</mtext><mo id='id-30'>&#x2062;</mo><mi id='id-31'>x</mi></mrow><mo id='id-32'>≥</mo><mn id='id-33'>0</mn></mrow></mtd>
            </mtr>
          </mtable>
        </mrow>
       </math>";
        init_default_prefs(mathml_str, "Enhanced");
        set_preference("SpeechStyle".to_string(), "ClearSpeak".to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomIn", mathml, "row-1");
            let speech = test_command("MovePrevious", mathml, "row-1");
            assert_eq!(speech, "move left; start of math");
            let speech = test_command("MoveNext", mathml, "row-2");
            assert_eq!(speech, "move right; case 2; positive x comma; if x, is greater than or equal to 0");
            let speech = test_command("ZoomOut", mathml, "table");
            assert_eq!(speech, "zoom out; 2 cases; case 1; negative x comma; if x is less than 0; case 2; positive x comma; if x, is greater than or equal to 0");
            let speech = test_command("ZoomIn", mathml, "row-1");
            assert_eq!(speech, "zoom in; case 1; negative x comma; if x is less than 0");
            set_preference("NavMode".to_string(), "Character".to_string()).unwrap();
            let speech = test_command("MovePrevious", mathml, "open");
            assert_eq!(speech, "move left; open brace");
            return Ok( () );
        });
    }

    #[test]
    fn base_superscript() -> Result<()> {
        // bug #217 -- zoom into base of parenthesized script 
        let mathml_str = "<math display='block' id='id-0'>
            <msup id='id-1'>
                <mrow id='id-2'>
                    <mo stretchy='false' id='id-3'>(</mo>
                    <mrow id='id-4'>
                        <mn id='id-5'>2</mn>
                        <mo id='id-6'>&#x2062;</mo>
                        <mi id='id-7'>x</mi>
                    </mrow>
                    <mo stretchy='false' id='id-8'>)</mo>
                </mrow>
                <mn id='id-9'>2</mn>
            </msup>
        </math>";
        init_default_prefs(mathml_str, "Enhanced");
        set_preference("SpeechStyle".to_string(), "ClearSpeak".to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            let speech = test_command("ZoomIn", mathml, "id-4");
            assert_eq!(speech, "zoom in; in base; 2 x");
            let speech = test_command("MoveNext", mathml, "id-9");
            assert_eq!(speech, "move right; in exponent; 2");
            return Ok( () );
        });
    }

    #[test]
    fn binomial_intent() -> Result<()> {
        let mathml_str = "<math display='block' id='id-0'>
                    <mrow intent='binomial($n,$k)' id='id-1'>
                        <mo id='id-2'>(</mo>
                        <mfrac linethickness='0pt' id='id-3'>
                            <mi arg='n' id='id-4'>n</mi>
                            <mi arg='k' id='id-5'>k</mi>
                        </mfrac>
                    <mo id='id-6'>)</mo>
                    </mrow>
                </math>";
        init_default_prefs(mathml_str, "Character");
        set_preference("SpeechStyle".to_string(), "ClearSpeak".to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            debug!("Character mode");
            let speech = test_command("MoveStart", mathml, "id-2");
            assert_eq!(speech, "move to start of math; open paren");
            let speech = test_command("MoveNext", mathml, "id-4");
            // I'm not keen on the use of numerator/denominator here, but character mode turns off intent
            assert_eq!(speech, "move right; in numerator; n");
            let speech = test_command("MoveNext", mathml, "id-5");
            assert_eq!(speech, "move right; in denominator; k");
            debug!("before zoom out");
            let speech = test_command("ZoomOut", mathml, "id-3");
            assert_eq!(speech, "zoom out; out of denominator; n over k");
            // let speech = test_command("ZoomOut", mathml, "id-1");
            // assert_eq!(speech, "zoom out; open paren n over k, close paren");

            set_preference("NavMode".to_string(), "Simple".to_string()).unwrap();
            debug!("Simple mode");
            let speech = test_command("ZoomIn", mathml, "id-4");
            assert_eq!(speech, "zoom in; in part 1; n");
            let speech = test_command("MoveNext", mathml, "id-5");
            assert_eq!(speech, "move right; in part 2; k");
            let speech = test_command("MoveNext", mathml, "id-5");
            assert_eq!(speech, "cannot move right, end of math");
            let speech = test_command("ZoomOut", mathml, "id-1");
            assert_eq!(speech, "zoom out; out of part 2; n choose k");

            set_preference("NavMode".to_string(), "Enhanced".to_string()).unwrap();
            debug!("Enhanced mode");
            let speech = test_command("ZoomIn", mathml, "id-4");
            assert_eq!(speech, "zoom in; in part 1; n");
            let speech = test_command("MoveNext", mathml, "id-5");
            assert_eq!(speech, "move right; in part 2; k");
            let speech = test_command("MoveNext", mathml, "id-5");
            assert_eq!(speech, "cannot move right, end of math");
            let speech = test_command("ZoomOut", mathml, "id-1");
            assert_eq!(speech, "zoom out; out of part 2; n choose k");

            return Ok( () );
        });
    }

    #[test]
    fn absolute_value() -> Result<()> {
        let mathml_str = "<math id='math'>
                <mrow id='expr'>
                    <mn id='2'>2</mn>
                    <mrow id='abs'>
                        <mo id='start'>|</mo>
                        <mi id='x'>x</mi>
                        <mo id='end'>|</mo>
                    </mrow>
                </mrow>
            </math>";
        init_default_prefs(mathml_str, "Enhanced");
        set_preference("SpeechStyle".to_string(), "ClearSpeak".to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            let speech = test_command("ZoomIn", mathml, "2");
            assert_eq!(speech, "zoom in; 2");
            let speech = test_command("MoveNext", mathml, "abs");
            assert_eq!(speech, "move right; the absolute value of x");
            let speech = test_command("ZoomIn", mathml, "x");
            assert_eq!(speech, "zoom in; in absolute value; x");
            let speech = test_command("MoveNext", mathml, "x");
            assert_eq!(speech, "cannot move right, end of math");
            set_preference("NavMode".to_string(), "Character".to_string()).unwrap();
            let speech = test_command("MoveNext", mathml, "end");
            assert_eq!(speech, "move right; vertical line");
            let speech = test_command("MoveLineStart", mathml, "2");
            assert_eq!(speech, "move to start of line; 2");
            let speech = test_command("MoveNext", mathml, "start");
            assert_eq!(speech, "move right; vertical line");
            return Ok( () );
        });
    }

    #[test]
    fn read_and_describe_fraction() -> Result<()> {
        let mathml_str = "<math id='math'>
            <mrow id='mrow'>
                <mfrac id='frac'>
                    <mrow id='numerator'><mi>b</mi><mo>+</mo><mn>1</mn></mrow>
                <mn id='denom'>3</mn>
                </mfrac>
                <mo id='minus'>-</mo>
                <mn id='3'>3</mn>
            </mrow>
        </math>";
        init_default_prefs(mathml_str, "Enhanced");
        set_preference("SpeechStyle".to_string(), "SimpleSpeak".to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomIn", mathml, "frac");
            let speech = test_command("ReadCurrent", mathml, "frac");
            assert_eq!(speech, "read current; fraction, b plus 1, over 3, end fraction");
            let speech = test_command("DescribeCurrent", mathml, "frac");
            assert_eq!(speech, "describe current; fraction");
            return Ok( () );
        });
    }


    #[test]
    fn read_and_describe_mrow() -> Result<()> {
        let mathml_str = "<math id='math'>
            <mrow id='mrow'>
                <mn>1</mn><mo>+</mo>
                <mn>2</mn><mo>+</mo>
                <mn>3</mn><mo>+</mo>
                <mn>4</mn><mo>+</mo>
                <mn>5</mn><mo>+</mo>
                <mn>6</mn><mo>+</mo>
                <mn>7</mn>
            </mrow>
        </math>";
        init_default_prefs(mathml_str, "Enhanced");
        set_preference("SpeechStyle".to_string(), "SimpleSpeak".to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            let speech = test_command("ZoomOutAll", mathml, "mrow");
            assert_eq!(speech, "zoomed out all the way; 1 plus 2 plus 3 plus 4 plus 5 plus 6 plus 7");
            let speech = test_command("ReadCurrent", mathml, "mrow");
            assert_eq!(speech, "read current; 1 plus 2 plus 3 plus 4 plus 5 plus 6 plus 7");
            let speech = test_command("DescribeCurrent", mathml, "mrow");
            assert_eq!(speech, "describe current; 1 plus 2 plus 3 and so on");
            return Ok( () );
        });
    }


    #[test]
    fn read_next_invisible_char() -> Result<()> {
        let mathml_str = "<math id='id-0'>
            <mrow id='id-1'>
                <mi id='id-2'>x</mi>
                <mo id='id-3'>&#x2062;</mo>
                <mi id='id-4'>y</mi>
            </mrow>
            </math>";
        init_default_prefs(mathml_str, "Simple");
        set_preference("SpeechStyle".to_string(), "SimpleSpeak".to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            let speech = test_command("ZoomIn", mathml, "id-2");
            assert_eq!(speech, "zoom in; x");
            let speech = test_command("ToggleZoomLockUp", mathml, "id-2");
            assert_eq!(speech, "enhanced mode; x");
            let speech = test_command("ReadNext", mathml, "id-2");
            assert_eq!(speech, "read right; y");
            return Ok( () );
        });
    }

    
    #[test]
    fn basic_language_test() -> Result<()> {
        // this is basically a sanity check that all the language's navigation.yaml files are at least syntactically correct
        // FIX: should look through the Languages dir and figure this is out
        let mathml_str = "<math id='math'>
                <mrow id='contents'>
                    <mrow id='lhs'>
                        <mrow id='term'>
                            <mn id='2'>2</mn>
                            <mo id='invisible-times'>&#x2062;</mo>
                            <msup id='msup'>
                                <mi id='x'>x</mi>
                                <mn id='3'>3</mn>
                            </msup>
                        </mrow>
                        <mo id='plus'>+</mo>
                        <mn id='1'>1</mn>
                    </mrow>
                <mo id='id-11'>=</mo>
                <mi id='id-12'>y</mi>
                </mrow>
            </math>";
        
        set_rules_dir(super::super::abs_rules_dir_path()).unwrap();
        for lang in get_supported_languages() {
            test_language(&lang, mathml_str);
        }
        return Ok( () );

        fn test_language(lang: &str, mathml_str: &str) {
            init_default_prefs(mathml_str, "Enhanced");
            set_preference("Language".to_string(), lang.to_string()).unwrap();

            set_preference("NavMode".to_string(), "Enhanced".to_string()).unwrap();
            MATHML_INSTANCE.with(|package_instance| {
                let package_instance = package_instance.borrow();
                let mathml = get_element(&*package_instance);
                test_command("ZoomInAll", mathml, "2");
                test_command("MoveNext", mathml, "msup");
                test_command("MoveNext", mathml, "plus");
                test_command("MovePrevious", mathml, "term");
                test_command("MovePrevious", mathml, "term");
                test_command("ZoomOutAll", mathml, "contents");
            });
    
            set_preference("NavMode".to_string(), "Simple".to_string()).unwrap();
            MATHML_INSTANCE.with(|package_instance: &RefCell<Package>| {
                let package_instance = package_instance.borrow();
                let mathml = get_element(&*package_instance);
                test_command("ZoomInAll", mathml, "2");
                test_command("MoveNext", mathml, "msup");
                test_command("MoveNext", mathml, "plus");
                test_command("MovePrevious", mathml, "msup");
                test_command("MovePrevious", mathml, "2");
                test_command("MovePrevious", mathml, "2");
                test_command("ZoomOutAll", mathml, "contents");
            });
            
            set_preference("NavMode".to_string(), "Character".to_string()).unwrap();
            MATHML_INSTANCE.with(|package_instance| {
                let package_instance = package_instance.borrow();
                let mathml = get_element(&*package_instance);
                test_command("ZoomIn", mathml, "2");
                test_command("MoveNext", mathml, "x");
                test_command("MoveNext", mathml, "3");
                test_command("MoveNext", mathml, "plus");
                test_command("MovePrevious", mathml, "3");
                test_command("MovePrevious", mathml, "x");
                test_command("MovePrevious", mathml, "2");
                test_command("MovePrevious", mathml, "2");
            });
            
            // simple sanity check that "overview.yaml" doesn't have a syntax error
            set_preference("Overview".to_string(), "True".to_string()).unwrap();
            set_preference("NavMode".to_string(), "Character".to_string()).unwrap();
            MATHML_INSTANCE.with(|package_instance| {
                let package_instance = package_instance.borrow();
                let mathml = get_element(&*package_instance);
                test_command("ZoomIn", mathml, "2");
            });
        }
    }
}