//! Navigation has two public functions: [`do_navigate_key_press`] and [`get_navigation_mathml`].
//!
//! Navigation is controlled by a `Navigation_Rules.yaml` file in conjunction with preferences.
//! See preference documentation for more info on navigation preferences.
#![allow(clippy::needless_return)]

use std::cell::RefCell;
use sxd_xpath::{Context, Factory, Value};
use sxd_document::dom::Element;
use sxd_document::Package;

use std::fmt;
use crate::pretty_print::mml_to_string;
use crate::speech::{NAVIGATION_RULES, CONCAT_INDICATOR, CONCAT_STRING};
use std::time::{Instant};
use crate::errors::*;
use crate::canonicalize::as_element;
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

const ILLEGAL_NODE_ID: &'static str = "!not set";     // an illegal 'id' value
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
    command_stack: Vec<&'static str>,                 // all commands, so we can undo them
    place_markers: [NavigationPosition; MAX_PLACE_MARKERS],
    where_am_i: NavigationPosition,             // current 'where am i' location

    #[cfg(target_family = "wasm")]
    where_am_i_start_time: usize,           // FIX: for web
    #[cfg(not(target_family = "wasm"))]
    where_am_i_start_time: Instant,
    mode: String,                         // one of "Character", "Simple", or "Enhanced"
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
            speak_overview: false,                      // FIX should be $Overview
        };
    }

    pub fn reset(&mut self) {
        self.position_stack.clear();
        self.command_stack.clear();
        self.where_am_i = NavigationPosition::default();
        self.where_am_i_start_time;
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
                                position.current_node, mml_to_string(&mathml)),
                Some(found) => Ok( (found, position.current_node_offset) )
            };
        }
    }

    pub fn get_navigation_mathml_id<'a>(&self, mathml: Element<'a>) -> (String, usize) {
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
            let position = &self.position_stack[self.position_stack.len()-1];
            context.set_variable("NavNode", position.current_node.as_str());
            context.set_variable("NavNodeOffset", position.current_node_offset as f64);
        } else {
            context.set_variable("NavNode", self.where_am_i.current_node.as_str());
            context.set_variable("NavNodeOffset", self.where_am_i.current_node_offset as f64);
        }

        // get the index from command (e.g., '3' in 'SetPlacemarker3 or MoveTo3' and set 'PlaceMarker' to it's position)
        if command.ends_with(|ch: char| ch.is_ascii_digit()) {
            let index = convert_last_char_to_number(command);
            let position = &self.place_markers[index];
            context.set_variable("PlaceMarkerIndex", index as f64);
            context.set_variable("PlaceMarker", position.current_node.as_str());
            context.set_variable("PlaceMarkerOffset", position.current_node_offset as f64);
        }
           
        context.set_variable("ReadZoomLevel", (if self.mode == "Enhanced" {-1} else {1}) as f64);
        context.set_variable("MatchCounter", 0 as f64);						// default is to speak the expr after navigation

        if command == "MoveLastLocation" {
            let previous_command = match nav_state_top {
                None => "",
                Some( (_, previous_command) ) => previous_command,
            };
            context.set_variable("PreviousNavCommand", previous_command);
        }

        // used by nav rules for speech -- needs an initial value so tests don't fail
        context.set_variable("Move2D", "" );
        context.set_variable("SpeakExpression","true" );

        return;

        fn convert_last_char_to_number(str: &str) -> usize {
            let last_char = str.as_bytes()[str.len()-1];
            assert!( '0' as u8 <= last_char && last_char <= '9' as u8);
            return (last_char - '0' as u8) as usize;
        }
    }
}

// convert the last digit of a Placemarker command to an integer
fn convert_last_char_to_number(str: &str) -> usize {
    let last_char = str.as_bytes()[str.len()-1];
    assert!( '0' as u8 <= last_char && last_char <= '9' as u8);
    return (last_char - '0' as u8) as usize;
}


pub fn get_node_by_id<'a>(mathml: Element<'a>, id: &str) -> Option<Element<'a>> {
    if mathml.attribute_value("id").unwrap() == id {
        return Some(mathml);
    }

    if crate::xpath_functions::is_leaf(mathml) {
        return None;
    }

    for child in mathml.children() {
        let child = as_element(child);
        if let Some(found) = get_node_by_id(child, id) {
            return Some(found);
        }
    }
    return None;
}

fn context_get_variable<'c>(context: &Context<'c>, var_name: &str, mathml: Element<'c>) -> (Option<String>, Option<f64>) {
    // First return tuple value is string-value (if string, bool, or single node) or None
    // Second return tuple value is f64 if variable is a number or None
    // This is ridiculously complicated for what in the end is a hashmap lookup
    // There isn't an API that lets us get at the value, so we have to setup/build/evaluate an xpath
    // Note: mathml can be any node. It isn't really used but some Element needs to be part of Evaluate() 
    let factory = Factory::new();
    return match factory.build(&("$".to_string() + var_name)) {
        Err(_) => {
            error!("Could not compile XPath for variable: {}", var_name);
            (None, None)
        },
        Ok(xpath) => {
            let xpath = xpath.unwrap();
            return match xpath.evaluate(context, mathml) {
                Ok(val) => {
                    match val {
                        Value::String(s) => (Some(s), None),
                        Value::Number(f) => (None, Some(f)),
                        Value::Boolean(b) => (Some(format!("{}", b)), None),
                        Value::Nodeset(nodes) => {
                            if nodes.size() == 1 {
                                if let Some(attr) = nodes.document_order_first().unwrap().attribute() {
                                    return (Some(attr.value().to_string()), None);
                                }
                            };
                            error!("Variable '{}' set in navigate.yaml is nodeset and not an attribute (correct by using '.../@id'??):\n", var_name);
                            if nodes.size() == 0 {
                                debug!("0 nodes (false)");
                            } else {
                                let singular = nodes.size()==1;
                                debug!("{} node{}. {}:", nodes.size(),
                                    if singular {""} else {"s"},
                                    if singular {"Node is"} else {"Nodes are"});
                                    nodes.document_order()
                                        .iter()
                                        .enumerate()
                                        .for_each(|(i, node)| {
                                            match node {
                                                sxd_xpath::nodeset::Node::Element(mathml) => debug!("#{}:\n{}",
                                                        i, mml_to_string(mathml)),
                                                _ => debug!("'{:?}'", node),
                                            }   
                                        })    
                            };
                            (None, None)
                        },
                    }
                },
                Err(_) => (None, None),
            }
        }
    }
}

/// Given a key code along with the modifier keys, the current node is moved accordingly (or value reported in some cases).]
/// The spoken text for the new current node is returned.
pub fn do_navigate_key_press<'a>(mathml: Element<'a>,
            key: usize, shift_key: bool, control_key: bool, alt_key: bool, meta_key: bool) -> Result<String> {
    let (command, param) = key_press_to_command_and_param(key, shift_key, control_key, alt_key, meta_key)?;
    return do_navigate_command(mathml, command, param);
}

fn do_navigate_command<'a>(mathml: Element<'a>, command: NavigationCommand, param: NavigationParam) -> Result<String> {
    return do_navigate_command_string(mathml, navigation_command_string(command, param));
}

pub fn do_navigate_command_string<'a>(mathml: Element<'a>, nav_command: &'static str) -> Result<String> {
    // If no speech happened for some calls, we try the call the call again (e.g, no speech for invisible times).
    // To prevent to infinite loop, we limit the number of tries
    const LOOP_LIMIT: usize = 3;
    static TRY_AGAIN: &'static str = "try again";

    for loop_count in 0..LOOP_LIMIT {
        if mathml.children().is_empty() {
            bail!("MathML has not been set -- can't navigate");
        };
    
        let result: Result<String> = NAVIGATION_STATE.with(|nav_state| {
            let mut nav_state = nav_state.borrow_mut();
            // debug!("MathML: {}", mml_to_string(&mathml));
            if nav_state.position_stack.is_empty() {
                // initialize to root node
                nav_state.push(NavigationPosition{
                    current_node: mathml.attribute_value("id").unwrap().to_string(),
                    current_node_offset: 0
                }, "None")
            };
    
            return NAVIGATION_RULES.with(|rules| {
                {
                    let mut mut_rules = rules.borrow_mut();
                    mut_rules.update();    
                }
                let rules = rules.borrow();
                let new_package = Package::new();
                let mut rules_with_context = crate::speech::SpeechRulesWithContext::new(&rules, new_package.as_document(), "".to_string()); 
                
                // if nav_state.mode.is_empty() {
                    nav_state.mode = rules.pref_manager.as_ref().borrow().get_user_prefs().to_string("NavMode");
                // }

                nav_state.init_navigation_context(rules_with_context.get_context(), nav_command, nav_state.top());
                
                debug!("NavCommand: {}, NavMode {}", nav_command, nav_state.mode);

                // start navigation off at the right node
                let start_node_id = if nav_command == "MoveLastLocation" {
                    match nav_state.pop() {
                        None => mathml.attribute_value("id)").unwrap().to_string(),
                        Some( (position, _) ) => position.current_node.clone(),
                    }
                } else {
                    match nav_state.top() {
                        None => mathml.attribute_value("id").unwrap().to_string(),
                        Some( (position, _) ) => position.current_node.clone(),
                    }
                };
    
                let start_node = match get_node_by_id(mathml, &start_node_id) {
                    Some(node) => node,
                    None => {
                        error!("Didn't find id '{}'", &start_node_id);
                        bail!("No node found");
                    }
                };

                // Finally, apply the navigation rules
                let speech = match rules_with_context.match_pattern::<String>(start_node) {
                    Ok(speech_string) => {
                        rules.pref_manager.borrow().get_tts()
                            .merge_pauses(crate::speech::remove_optional_indicators(
                                &speech_string.replace(CONCAT_STRING, "")
                                                    .replace(CONCAT_INDICATOR, "")                            
                                            )
                            .trim())
                    },
                    Err(e)             => { 
                        crate::speech::print_errors(&e.chain_err(|| "Pattern match/replacement failure!"));
                        bail!("Error in speaking math navigation; see error log.")
                    }
                };
                debug!("Nav Speech: {}", speech);
    
                // FIX: add things that need to do
                // do a speech replacement based on some marker for "where am i" and others that loop ([Speak: id])???
                // what else needs to be done/set???
                let context = rules_with_context.get_context();
    
                // transfer some values that might have been set into the prefs
                nav_state.mode = context_get_variable(context, "NavMode", mathml).0.unwrap();
                rules.pref_manager.as_ref().borrow_mut().set_user_prefs("NavMode", &nav_state.mode);

                let nav_position = match context_get_variable(&context, "NavNode", mathml).0 {
                    None => NavigationPosition::default(),
                    Some(node) => NavigationPosition {
                        current_node: node,
                        current_node_offset: context_get_variable(&context, "NavNodeOffset", mathml).1.unwrap() as usize
                    }
                };
    
                // after a command, we either read or describe the new location (part of state)
                // also some commands are DescribeXXX/ReadXXX, so we need to look at the commands also
                let use_read_rules = if nav_command.starts_with("Read") {
                    true
                } else if nav_command.starts_with("Describe") {
                    false
                } else {
                    let overview = context_get_variable(context, "Overview", mathml).0.unwrap();
                    overview == "false"
                };
    
                if (nav_command.starts_with("Move") || nav_command.starts_with("Zoom")) && nav_command != "MoveLastLocation" {
                    // push the new location on the stack
                    if &nav_position != &NavigationPosition::default() {
                        debug!("nav_state: pushing on {}", &nav_position);
                        if &nav_position.current_node != ILLEGAL_NODE_ID {
                            nav_state.push(nav_position.clone(), nav_command);
                        }
                    }
                }

                if nav_command.starts_with("SetPlacemarker") {
                    if let Some(new_node_id) = context_get_variable(&context, "NavNode", mathml).0 {
                        let offset = context_get_variable(&context, "NavNodeOffset", mathml).1.unwrap() as usize;
                        nav_state.place_markers[convert_last_char_to_number(nav_command)] = NavigationPosition{ current_node: new_node_id, current_node_offset: offset};
                    }
                }
    
                debug!("{}", &nav_state);
                let nav_mathml = get_node_by_id(mathml, &nav_position.current_node);
                if nav_mathml.is_some() && context_get_variable(context, "SpeakExpression", mathml).0.unwrap() == "true" {
                    // Speak/Overview of where we landed (if we are supposed to speak it)
                    let node_speech = speak(&mut rules_with_context, nav_mathml.unwrap(), use_read_rules)?;
                    if !node_speech.is_empty() {
                        pop_stack(&mut nav_state, loop_count);
                        return Ok( speech + &node_speech );
                    } else {
                        // try again in loop
                        bail!(TRY_AGAIN);
                    }
                } else {
                    pop_stack(&mut nav_state, loop_count);
                    return Ok( speech );
                };
            });
        });
        match result {
            Ok(speech) => return Ok( speech ),
            Err(e) => {
                if e.to_string().as_str() != TRY_AGAIN {
                    return Err(e);
                }
            }
        }
    }
    bail!("Internal error: Navigation exceeded limit of number of times no speech generated.");

    fn pop_stack(nav_state: &mut NavigationState, count: usize) {
        // save the final state and pop the intermediate states that did nothing
        if count == 0 {
            return;
        }

        let (top_position, top_command) = nav_state.pop().unwrap();
        let mut count = count-1;
        loop {
            debug!("  ... loop count={}", count);
            let (_, nav_command) = nav_state.top().unwrap();
            if (nav_command.starts_with("Move") || nav_command.starts_with("Zoom")) && nav_command != "MoveLastLocation" {
                nav_state.pop();
            }
            if count == 0 {
                break;
            };
            count -= 1;
        };
        nav_state.push(top_position, top_command);
    }
}

fn speak<'r, 'c, 's:'c, 'm:'c>(rules_with_context: &'r mut crate::speech::SpeechRulesWithContext<'c,'s,'m>, mathml: Element<'c>, full_read: bool) -> Result<String> {
    if full_read {
        return Ok( crate::speech::speak_intent(crate::speech::intent_from_mathml(mathml, rules_with_context.get_document())) );
    } else {
        // FIX: overview not implemented
        return Ok( crate::speech::overview_mathml(mathml) );
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
        0x30|0x31|0x32|0x33|0x34|0x35|0x36|0x37|0x38|0x39 => {  // '0' ... '9'
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
    fn test_command(command: &'static str, mathml: Element, result_id: &str) -> Result<String> {
        let nav_speech = do_navigate_command_string(mathml, command)?;
        debug!("Full speech: {}", nav_speech);
        NAVIGATION_STATE.with(|nav_stack| {
            let (id, _) = nav_stack.borrow().get_navigation_mathml_id(mathml);
            assert_eq!(id, result_id);
        });

        return Ok( nav_speech );
    }

    #[test]
    fn zoom_in() -> Result<()> {
        // init_logger();
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        SetMathML(mathml_str.to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomIn", mathml, "msup")?;
            test_command("ZoomIn", mathml, "base")?;
            return Ok( () );
        });
    }
    
    #[test]
    fn zoom_in_parens() -> Result<()> {
        // init_logger();
        // (a+b)(c+d) + 1
        let mathml_str = " <math display='block' id='Ml2f6pjt-0' data-id-added='true'>
        <mrow data-changed='added' id='Ml2f6pjt-1' data-id-added='true'>
          <mrow data-changed='added' id='Ml2f6pjt-2' data-id-added='true'>
            <mrow data-changed='added' id='Ml2f6pjt-3' data-id-added='true'>
              <mo stretchy='false' id='Ml2f6pjt-4' data-id-added='true'>(</mo>
              <mrow data-changed='added' id='Ml2f6pjt-5' data-id-added='true'>
                <mi id='Ml2f6pjt-6' data-id-added='true'>a</mi>
                <mo id='Ml2f6pjt-7' data-id-added='true'>+</mo>
                <mi id='Ml2f6pjt-8' data-id-added='true'>b</mi>
              </mrow>
              <mo stretchy='false' id='Ml2f6pjt-9' data-id-added='true'>)</mo>
            </mrow>
            <mo data-changed='added' id='Ml2f6pjt-10' data-id-added='true'>&#x2062;</mo>
            <mrow data-changed='added' id='Ml2f6pjt-11' data-id-added='true'>
              <mo stretchy='false' id='Ml2f6pjt-12' data-id-added='true'>(</mo>
              <mrow data-changed='added' id='Ml2f6pjt-13' data-id-added='true'>
                <mi id='Ml2f6pjt-14' data-id-added='true'>c</mi>
                <mo id='Ml2f6pjt-15' data-id-added='true'>+</mo>
                <mi id='Ml2f6pjt-16' data-id-added='true'>d</mi>
              </mrow>
              <mo stretchy='false' id='Ml2f6pjt-17' data-id-added='true'>)</mo>
            </mrow>
          </mrow>
          <mo id='Ml2f6pjt-18' data-id-added='true'>+</mo>
          <mn id='Ml2f6pjt-19' data-id-added='true'>1</mn>
        </mrow>
       </math>";
        SetMathML(mathml_str.to_string()).unwrap();
        SetPreference("NavMode".to_string(), StringOrFloat::AsString("Enhanced".to_string()))?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomIn", mathml, "Ml2f6pjt-2")?;
            test_command("ZoomIn", mathml, "Ml2f6pjt-5")?;
            test_command("ZoomIn", mathml, "Ml2f6pjt-6")?;
            
            // repeat, but this time with "Simple
            SetPreference("NavMode".to_string(), StringOrFloat::AsString("Simple".to_string()))?;
            test_command("ZoomOutAll", mathml, "Ml2f6pjt-0")?;
            test_command("ZoomIn", mathml, "Ml2f6pjt-2")?;
            test_command("ZoomIn", mathml, "Ml2f6pjt-3")?;
            test_command("ZoomIn", mathml, "Ml2f6pjt-4")?;
            return Ok( () );
        });
    }
    
    #[test]
    fn zoom_in_all() -> Result<()> {
        // init_logger();
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        SetMathML(mathml_str.to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomInAll", mathml, "base")?;
            return Ok( () );
        });
    }

    
    #[test]
    fn zoom_out() -> Result<()> {
        // init_logger();
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        SetMathML(mathml_str.to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "base".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            test_command("ZoomOut", mathml, "msup")?;

            let nav_speech = do_navigate_command(mathml, NavigationCommand::Zoom, NavigationParam::Previous)?;
            debug!("Full speech: {}", nav_speech);
            NAVIGATION_STATE.with(|nav_stack| {
                let (id, _) = nav_stack.borrow().get_navigation_mathml_id(mathml);
                assert_eq!(id, "mfrac");
            });
            return Ok( () );
        });
    }
    
    #[test]
    fn zoom_out_all() -> Result<()> {
        // init_logger();
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        SetMathML(mathml_str.to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "base".to_string(),
                    current_node_offset: 0
                }, "None")
            });

            test_command("ZoomOutAll", mathml, "math")?;
            return Ok( () );
        });
    }
    
    #[test]
    fn move_to_start() -> Result<()> {
        // init_logger();
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        SetMathML(mathml_str.to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "denom".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            test_command("MoveLineStart", mathml, "denom")?;
            let nav_speech = do_navigate_command(mathml, NavigationCommand::Move, NavigationParam::Start)?;
            debug!("Full speech: {}", nav_speech);
            NAVIGATION_STATE.with(|nav_stack| {
                let (id, _) = nav_stack.borrow().get_navigation_mathml_id(mathml);
                assert_eq!(id, "mfrac");
            });
            return Ok( () );
        });
    }
    
    #[test]
    fn move_right_sup() -> Result<()> {
        // init_logger();
        let mathml_str = "<math display='block' id='Msowudr8-0' data-id-added='true'>
        <mrow data-changed='added' id='Msowudr8-1' data-id-added='true'>
          <msup id='Msowudr8-2' data-id-added='true'>
            <mn id='Msowudr8-3' data-id-added='true'>2</mn>
            <mi id='Msowudr8-4' data-id-added='true'>q</mi>
          </msup>
          <mo id='Msowudr8-5' data-id-added='true'>-</mo>
          <mi id='Msowudr8-6' data-id-added='true'>x</mi>
        </mrow>
       </math>";
        SetMathML(mathml_str.to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "Msowudr8-2".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            SetPreference("NavMode".to_string(), StringOrFloat::AsString("Enhanced".to_string()))?;
            test_command("MoveNext", mathml, "Msowudr8-5")?;

            // reset start and test Simple
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "Msowudr8-2".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            SetPreference("NavMode".to_string(), StringOrFloat::AsString("Simple".to_string()))?;
            test_command("MoveNext", mathml, "Msowudr8-5")?;

            // reset start and test Character
            NAVIGATION_STATE.with(|nav_stack| {
                nav_stack.borrow_mut().push(NavigationPosition{
                    current_node: "Msowudr8-3".to_string(),
                    current_node_offset: 0
                }, "None")
            });
            SetPreference("NavMode".to_string(), StringOrFloat::AsString("Character".to_string()))?;
            test_command("MoveNext", mathml, "Msowudr8-4")?;

            SetPreference("NavMode".to_string(), StringOrFloat::AsString("Character".to_string()))?;
            test_command("MoveNext", mathml, "Msowudr8-5")?;
            return Ok( () );
        });
    }

    
    #[test]
    fn move_right_char() -> Result<()> {
        // gger();
        let mathml_str = "<math id='Myt3m7mx-0' data-id-added='true'>
        <mrow displaystyle='true' id='Myt3m7mx-1' data-id-added='true'>
          <mi id='Myt3m7mx-2' data-id-added='true'>x</mi>
          <mo id='Myt3m7mx-3' data-id-added='true'>=</mo>
          <mrow data-changed='added' id='Myt3m7mx-4' data-id-added='true'>
            <mfrac id='Myt3m7mx-5' data-id-added='true'>
              <mn id='Myt3m7mx-6' data-id-added='true'>1</mn>
              <mrow id='Myt3m7mx-7' data-id-added='true'>
                <mi id='Myt3m7mx-8' data-id-added='true'>a</mi>
                <mo id='Myt3m7mx-9' data-id-added='true'>+</mo>
                <mn id='Myt3m7mx-10' data-id-added='true'>2</mn>
              </mrow>
            </mfrac>
            <mo id='Myt3m7mx-11' data-id-added='true'>+</mo>
            <mrow data-changed='added' id='Myt3m7mx-12' data-id-added='true'>
              <mn id='Myt3m7mx-13' data-id-added='true'>3</mn>
              <mo data-changed='added' id='Myt3m7mx-14' data-id-added='true'>&#x2062;</mo>
              <mi id='Myt3m7mx-15' data-id-added='true'>b</mi>
            </mrow>
          </mrow>
        </mrow>
       </math>";
        SetMathML(mathml_str.to_string()).unwrap();
        SetPreference("NavMode".to_string(), StringOrFloat::AsString("Character".to_string()))?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("ZoomInAll", mathml, "Myt3m7mx-2")?;
            test_command("MoveNext", mathml, "Myt3m7mx-3")?;
            test_command("MoveNext", mathml, "Myt3m7mx-6")?;
            test_command("MoveNext", mathml, "Myt3m7mx-8")?;
            test_command("MoveNext", mathml, "Myt3m7mx-9")?;
            test_command("MoveNext", mathml, "Myt3m7mx-10")?;
            test_command("MoveNext", mathml, "Myt3m7mx-11")?;
            test_command("MoveNext", mathml, "Myt3m7mx-13")?;
            test_command("MoveNext", mathml, "Myt3m7mx-15")?;
            test_command("MoveNext", mathml, "Myt3m7mx-15")?;

            return Ok( () );
        });
    }
    
    #[test]
    fn placemarker() -> Result<()> {
        // init_logger();
        let mathml_str = "<math display='block' id='math'>
        <mrow displaystyle='true' id='mrow'>
          <mi id='a'>a</mi>
          <mo id='plus-1'>+</mo>
          <mi id='b'>b</mi>
          <mo id='plus-2'>+</mo>
          <mi id='c'>c</mi>
        </mrow>
       </math>";
        SetMathML(mathml_str.to_string()).unwrap();
        SetPreference("NavMode".to_string(), StringOrFloat::AsString("Character".to_string()))?;
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            test_command("MoveStart", mathml, "a")?;
            test_command("SetPlacemarker0", mathml, "a")?;
            test_command("MoveEnd", mathml, "c")?;
            test_command("Read0", mathml, "c")?;
            test_command("Describe0", mathml, "c")?;
            test_command("SetPlacemarker1", mathml, "c")?;
            test_command("MoveTo0", mathml, "a")?;
            test_command("MoveTo1", mathml, "c")?;
            test_command("MoveLastLocation", mathml, "a")?;
            
            return Ok( () );
        });
    }

    #[test]
    fn where_am_i_all() -> Result<()> {
        // init_logger();
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        SetMathML(mathml_str.to_string()).unwrap();
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
            let speech =test_command("WhereAmIAll", mathml, "exp")?;
            // should be 2 "inside" strings corresponding to steps to the root
            assert_eq!(speech.matches("inside").map(|_| 1).sum::<i32>(), 2);
            return Ok( () );
        });
    }
}