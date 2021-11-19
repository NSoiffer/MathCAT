//! Navigation has two public functions: [`do_navigate_key_press`] and [`get_navigation_mathml`].
//!
//! Navigation is controlled by a `Navigation_Rules.yaml` file in conjunction with preferences.
//! See preference documentation for more info on navigation preferences.
#![allow(clippy::needless_return)]

use std::cell::RefCell;
use sxd_xpath::{Context, Factory, Value};
use sxd_document::dom::Element;
use std::fmt;
use crate::pretty_print::mml_to_string;
use crate::speech::{NAVIGATION_RULES, CONCAT_INDICATOR, CONCAT_STRING};
use std::time::{Instant};
use crate::errors::*;
use crate::canonicalize::as_element;



const MAX_PLACE_MARKERS: usize = 10;

//use sxd_document::dom::*;
thread_local!{
    /// The current set of navigation rules
    pub static NAVIGATION_STATE: RefCell<NavigationState> =
            RefCell::new( NavigationState::new() );
}

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

impl Default for NavigationPosition {
    fn default() -> Self {
        NavigationPosition {
            current_node: "!not set".to_string(),   // an illegal 'id' value
            current_node_offset: 0    
        }
     }
}

impl NavigationPosition {
    
}

#[derive(Debug)]
pub struct NavigationState {
    // it might be better to use a linked for the stacks, with the first node being the top
    // these two stacks should be kept in sync.
    position_stack: Vec<NavigationPosition>,    // all positions, so we can go back to them
    command_stack: Vec<&'static str>,                 // all commands, so we can undo them
    place_markers: Vec<NavigationPosition>,
    where_am_i: NavigationPosition,             // current 'where am i' location

    #[cfg(target_family = "wasm")]
    where_am_i_start_time: usize,           // FIX: for web
    #[cfg(not(target_family = "wasm"))]
    where_am_i_start_time: Instant,
    mode: &'static str,                         // one of "Character", "Simple", or "Enhanced"
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
            place_markers: Vec::with_capacity(MAX_PLACE_MARKERS),
            where_am_i: NavigationPosition::default(),
            // FIX: figure this out for the web
            #[cfg(target_family = "wasm")]
            where_am_i_start_time: 0,           // FIX: for web
            #[cfg(not(target_family = "wasm"))]
            where_am_i_start_time: Instant::now(),      // need to give it some value, and "default()" isn't an option
            mode: "enhanced",                           // FIX: should be 'if $RestartMode then $StartMode else stored previous mode		
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

        // get the index from command (e.g., '0' in 'SetPlacemarker3')
        if command.starts_with("SetPlacemarker") {
            context.set_variable("PlaceMarkerIndex", convert_last_char_to_number(command) );
        } else if command.len()==7 && command.starts_with("MoveTo") {
            context.set_variable("PlaceMarkerIndex", convert_last_char_to_number(command) );
        } else if command.len()==5 && command.starts_with("Read") {
            context.set_variable("PlaceMarkerIndex", convert_last_char_to_number(command) );
        } else if command.len()==9 && command.starts_with("Describe") {
            context.set_variable("PlaceMarkerIndex", convert_last_char_to_number(command) );        
        } else {
            context.set_variable("PlaceMarkerIndex", 10 as f64);				// an illegal value
        }
           
        context.set_variable("ReadZoomLevel", (if self.mode == "enhanced" {-1} else {1}) as f64);
        context.set_variable("MatchCounter", 0 as f64);						// default is to speak the expr after navigation

        if command == "MoveLastLocation" {
            let previous_command = match nav_state_top {
                None => "",
                Some( (_, previous_command) ) => previous_command,
            };
            context.set_variable("PreviousNavCommand", previous_command);
        }
        return;

        fn convert_last_char_to_number(str: &str) -> f64 {
            let last_char = str.as_bytes()[str.len()-1];
            assert!( '0' as u8 <= last_char && last_char <= '9' as u8);
            return (last_char - '0' as u8) as f64;
        }
    }
}

fn get_node_by_id<'a>(mathml: Element<'a>, id: &str) -> Option<Element<'a>> {
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

fn context_get_variable<'c>(context: &Context<'c>, var_name: &str, mathml: Element<'c>) -> Option<String> {
    // This is ridiculously complicated for what in the end is a hashmap lookup
    // There isn't an API that lets us get at the value, so we have to setup/build/evaluate an xpath
    // Note: mathml can be any node. It isn't really used but some Element needs to be part of Evaluate() 
    let factory = Factory::new();
    return match factory.build(&("$".to_string() + var_name)) {
        Err(_) => {
            error!("Could not compile XPath for variable: {}", var_name);
            None
        },
        Ok(xpath) => {
            let xpath = xpath.unwrap();
            return match xpath.evaluate(context, mathml) {
                Ok(val) => {
                    match val {
                        Value::String(s) => Some(s),
                        Value::Number(f) => Some(format!("{}", f)),
                        Value::Boolean(b) => Some(format!("{}", b)),
                        Value::Nodeset(nodes) => {
                            if nodes.size() == 1 {
                                if let Some(attr) = nodes.document_order_first().unwrap().attribute() {
                                    return Some(attr.value().to_string());
                                }
                            };
                            error!("Variable '{}' set in navigate.yaml is nodeset and not an attribute (correct by using '.../@{}'??):\n", var_name, var_name);
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
                            None
                        },
                    }
                },
                Err(_) => None,
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

fn do_navigate_command_string<'a>(mathml: Element<'a>, nav_command: &'static str) -> Result<String> {
    debug!("NavCommand = {}", nav_command);
    NAVIGATION_STATE.with(|nav_state| {
        let mut nav_state = nav_state.borrow_mut();
        if nav_state.position_stack.is_empty() {
            // initialize to root node
            nav_state.push(NavigationPosition{
                current_node: mathml.attribute_value("id").unwrap().to_string(),
                current_node_offset: 0
            }, "None")
        };
        NAVIGATION_RULES.with(|rules| {
            {
                let mut mut_rules = rules.borrow_mut();
                mut_rules.update();    
            }
            let rules = rules.borrow();
            let mut rules_with_context = crate::speech::SpeechRulesWithContext::new(&rules); 
            let nav_state_top = nav_state.top();
            nav_state.init_navigation_context(rules_with_context.get_context(), nav_command, nav_state_top);
    
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

            let start_node = get_node_by_id(mathml, &start_node_id).unwrap();
            let speech = match rules_with_context.match_pattern(&start_node) {
                Ok(speech_string) => {
                    rules.pref_manager.get_tts()
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
            // context_get_variable
            // if SpeakAfterMove do speech/overview for current node
            // do a speech replacement based on some marker for "where am i" and others that loop ([Speak: id])???
            // what else needs to be done/set???
            let context = rules_with_context.get_context();
                    // after a command, we either read or describe the new location (part of state)
            // also some commands are DescribeXXX/ReadXXX, so we need to look at the commands also
            let mut use_read_rules = if nav_command.starts_with("Read") {
                true
            } else if nav_command.starts_with("Describe") {
                false
            } else {
                !nav_state.speak_overview
            };
            if let Some(val) = context_get_variable(context, "DescribeAfterMove", mathml) {
                nav_state.speak_overview = val == "true";
                use_read_rules = nav_state.speak_overview;
            };

            if nav_command != "MoveLastLocation" {
                if let Some(new_node_id) = context_get_variable(&context, "NavNode", mathml) {
                    nav_state.push(NavigationPosition{ current_node: new_node_id, current_node_offset: 0}, nav_command);
                }
            }
            debug!("{}", &nav_state);
            let (nav_mathml, _) = nav_state.get_navigation_mathml(mathml)?;
            return Ok( speech + &speak(nav_mathml, use_read_rules) );
        })
    })
}

fn speak(mathml: Element, full_read: bool) -> String {
    if full_read {
        return crate::speech::speak_mathml(mathml);
    } else {
        // FIX: overview not implemented
        return crate::speech::overview_mathml(mathml);
    }
}


// MathPlayer's interface mentions these, so we keep them.
// FIX: these need to be different values (I think) for linux
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
    use crate::interface::*;

    #[cfg(test)]
    pub fn init_logger() {
        env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .format_indent(None)
        .format_level(false)
        .init();
    }
    
    #[test]
    fn zoom_in() -> Result<()> {
        init_logger();
        let mathml_str = "<math id='math'><mfrac id='mfrac'>
                <msup id='msup'><mi id='base'>b</mi><mn id='exp'>2</mn></msup>
                <mi id='denom'>d</mi>
            </mfrac></math>";
        SetMathML(mathml_str.to_string()).unwrap();
        return MATHML_INSTANCE.with(|package_instance| {
            let package_instance = package_instance.borrow();
            let mathml = get_element(&*package_instance);
            let nav_speech = do_navigate_command_string(mathml, "ZoomIn")?;
            debug!("Full speech (first zoom): {}", nav_speech);
            NAVIGATION_STATE.with(|nav_stack| {
                let (id, _) = nav_stack.borrow().get_navigation_mathml_id(mathml);
                assert!(id == "msup");
            });

            let nav_speech = do_navigate_command(mathml, NavigationCommand::Zoom, NavigationParam::Next)?;
            debug!("Full speech (second zoom): {}", nav_speech);
            NAVIGATION_STATE.with(|nav_stack| {
                let (id, _) = nav_stack.borrow().get_navigation_mathml_id(mathml);
                assert!(id == "base");
            });
            return Ok( () );
        });
    }
}