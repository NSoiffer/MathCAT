# MathCAT: Math Capable Assistive Technology
<img src="logo.png" style="position: relative; top: 16px; z-index: -1;">

## Information for AT and Other Library Users

When using MathCAT, the general ordering of calls is:
1. The location of the MathCAT `Rules` directory is set [SetRulesDir]
1. Whatever preferences the AT needs to set, it is done with calls to [`SetPreference`]. Typically the `Language` and `TTS` engine to use (if any -- strongly recommended) are given. 
2. The MathML is sent over via [`SetMathML`].
3. AT calls to get the speech [`GetSpokenText`] and calls [`GetBraille`] to get the (Unicode) braille. If the id of a node is given, then the corresponding braille cells will be highlighted.

Navigation can be done via calls to either:
* [`DoNavigateKeyPress`] (takes key events as input)
* [`DoNavigateCommand`] (takes the commands the key events internally map to)

Both return a string to speak.
To highlight the current navigation node, 'id's are used. If they weren't already present,
[`SetMathML`] returns a string representing MathML that contains 'id's for any node that doesn't already
have an 'id' set. You can get the current node with
* [`GetNavigationMathMLId`]
* [`GetNavigationMathML`] -- returns a string representing the MathML for the selected node

Note: a second integer is returned by both of these calls. This number is the offset in characters for a leaf node.
This is needed when navigating by character for multi-symbol leaf nodes such as "sin" and "1234".
Currently the value is always '0' -- this feature needs more implementation work.

It is also possible to find out what preferences are currently set by calling [`GetPreference`]

All functions return a potential error code.

Note: MathCAT does a lot of work to clean up bad MathML. In particular, numbers with commas and periods are often split into pieces by MathML generators. MathCAT tries to put them back together, but to do that, it needs to know the locale's format for block separators and decimal separators. For example, in the US, "1,234.0" is a valid number, but in Europe, it is not a number because the `,` is a decimal separator. The locale is based on the country the document was authored for, not the language being used to speak the math. The two preferences that control what a legal number looks like are `BlockSeparators` and `DecimalSeparators`. Callers should set these values when they are known. They default to the US style of numbers.

## Rust Users
MathCAT is written in Rust, so all you need to do is build MathCAT and in your project's Cargo.toml file add something like
```
[dependencies.MathCAT]
mathcat = 0.2.0    # check for what the latest version is and use that
```

The exact function signatures are (with comments):
```
/// Set the Rules directory
/// IMPORTANT: this should be the very first call to MathCAT unless the environment var MathCATRulesDir is set
pub fn set_rules_dir(dir: String) -> Result<()>

/// Returns the version number (from Cargo.toml) of the build
pub fn get_version() -> String

/// This will override any previous MathML that was set.
/// This returns canonical MathML with 'id's set on any node that doesn't have an id.
/// The ids can be used for sync highlighting if the `Bookmark` API preference is true.
pub fn set_mathml(mathml_str: String) -> Result<String>

/// Get the spoken text of the MathML that was set.
/// The speech takes into account any AT or user preferences.
pub fn get_spoken_text() -> Result<String>

/// Get the spoken text for an overview of the MathML that was set.
/// The speech takes into account any AT or user preferences.
/// Note: this implementation for is currently minimal and should not be used.
pub fn get_overview_text() -> Result<String>

/// Get the value of the named preference.
/// None is returned if `name` is not a known preference.
pub fn get_preference(name: String) -> Result<String>

/// Set a MathCAT preference. The preference name should be a known preference name.
/// The value should either be a string or a number (depending upon the preference being set)
/// The list of known user preferences is in the MathCAT user documentation.
/// Here are common preferences set by programs (not settable by the user):
/// * TTS -- SSML, SAPI5, None
/// * Pitch -- normalized at '1.0'
/// * Rate -- words per minute (should match current speech rate).
///       There is a separate "MathRate" that is user settable that causes a relative percentage change from this rate.
/// * Volume -- default 100
/// * Voice -- set a voice to use (not implemented)
/// * Gender -- set pick any voice of the given gender (not implemented)
/// * Bookmark -- set to `true` if a `mark`/`bookmark` should be part of the returned speech (used for sync highlighting)
/// * CheckRuleFiles -- check to see if the rules files have changed since the last call. Values are "All", "Prefs"  (default) (only the system and user prefs.yaml files), and "None". There is about a 40% speedup changing from "All" to "None" and about a 10% speedup changing from "Prefs" to "None". 
///
/// These are use to control speech and pitch changes for capital letters:
/// * CapitalLetters_UseWord -- say "cap" (or whatever is appropriate for the language) [default: true]
/// * CapitalLetters_Pitch -- add a pitch change around a capital letter (normalized at '1.0' -- '1.0' [default] does nothing)
/// * CapitalLetters_Beep -- generates a fake SSML audio take with audio src='beep.mp4' -- used as a flag to beep in NVDA
///
/// * IntentErrorRecovery -- determines what should happen if the MathML contains illegal `intent` values. Options are a "Error" and "IgnoreIntent" (default)
///
/// Important: both the preference name and value are case-sensitive
/// 
/// This function can be called multiple times to set different values.
/// The values are persistent and extend beyond calls to [`set_mathml`].
/// A value can be overwritten by calling this function again with a different value.
/// 
/// FIX: Some preferences are both API and user preferences and something such as '!name' should be used for overrides. Not implemented yet.
pub fn set_preference(name: String, value: String) -> Result<()>

/// Given a key code along with the modifier keys, the current node is moved accordingly (or value reported in some cases).
/// `key` is the [keycode](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/keyCode#constants_for_keycode_value) for the key (in JavaScript, `ev.key_code`)
/// The spoken text for the new current node is returned.
pub fn do_navigate_keypress(key: usize, shift_key: bool, control_key: bool, alt_key: bool, meta_key: bool) -> Result<String>

/// Given a navigation command, the current node is moved accordingly.
/// This is a higher level interface than `do_navigate_keypress` for applications that want to interpret the keys themselves.
/// The valid commands are:
/// * Standard move commands:
/// `MovePrevious`, `MoveNext`, `MoveStart`, `MoveEnd`, `MoveLineStart`, `MoveLineEnd`
/// * Movement in a table or elementary math:
/// `MoveCellPrevious`, `MoveCellNext`, `MoveCellUp`, `MoveCellDown`, `MoveColumnStart`, `MoveColumnEnd`
/// * Moving into children or out to parents:
/// `ZoomIn`, `ZoomOut`, `ZoomOutAll`, `ZoomInAll`
/// * Undo the last movement command:
/// `MoveLastLocation`
/// * Read commands (standard speech):
/// `ReadPrevious`, `ReadNext`, `ReadCurrent`, `ReadCellCurrent`, `ReadStart`, `ReadEnd`, `ReadLineStart`, `ReadLineEnd`
/// * Describe commands (overview):
/// `DescribePrevious`, `DescribeNext`, `DescribeCurrent`
/// * Location information:
/// `WhereAmI`, `WhereAmIAll`
/// * Change navigation modes (circle up/down):
///  `ToggleZoomLockUp`, `ToggleZoomLockDown`
/// * Speak the current navigation mode
/// `ToggleSpeakMode`
/// 
/// There are 10 place markers that can be set/read/described or moved to.
/// * Setting:
/// `SetPlacemarker0`, `SetPlacemarker1`, `SetPlacemarker2`, `SetPlacemarker3`, `SetPlacemarker4`, `SetPlacemarker5`, `SetPlacemarker6`, `SetPlacemarker7`, `SetPlacemarker8`, `SetPlacemarker9`
/// * Reading:
/// `Read0`, `Read1`, `Read2`, `Read3`, `Read4`, `Read5`, `Read6`, `Read7`, `Read8`, `Read9`
/// * Describing:
/// `Describe0`, `Describe1`, `Describe2`, `Describe3`, `Describe4`, `Describe5`, `Describe6`, `Describe7`, `Describe8`, `Describe9`
/// * Moving:
/// `MoveTo0`, `MoveTo1`, `MoveTo2`, `MoveTo3`, `MoveTo4`, `MoveTo5`, `MoveTo6`, `MoveTo7`, `MoveTo8`, `MoveTo9`
/// 
/// When done with Navigation, call with `Exit`
pub fn do_navigate_command(command: String) -> Result<String>

/// Return the MathML associated with the current (navigation) node.
/// The returned result is the `id` of the node and the offset (0-based) from that node (not yet implemented)
/// The offset is needed for token elements that have multiple characters.
pub fn get_navigation_mathml() -> Result<(String, usize)> 

/// Return the `id` and `offset` (0-based) associated with the current (navigation) node.
/// `offset` (not yet implemented)
/// The offset is needed for token elements that have multiple characters.
pub fn get_navigation_mathml_id() -> Result<(String, usize)>


/// Convert the returned error from set_mathml, etc., to a useful string for display
pub fn errors_to_string(e:&Error) -> String 

```

## Python Users
You can build your own Python interface, or use the one that is built with the related project [MathCATForPython](https://github.com/NSoiffer/MathCATForPython). This uses the Rust package pyo3.

The Python interface is basically the same as the Rust interface. The Python interface uses CamelCase rather than Rust's snake_case. For example, `set_rules_dir` is `SetRulesDir` in the Python interface. When calling a function, it should be wrapped in
```
try:
    ...
except Exception as e:
    ...  # log the error 'e'
```


## Web Users
I built a web assembly version. Has a few compromises and requires some hand tweaks during the build process. Those need to be automated. It can be found at [MathCatDemo](https://github.com/NSoiffer/MathCATDemo). This builds a web page for demo purposes, so it is not a pure build for the Web. Nonetheless, it does demonstrate how that can be done.

## C/C++ Users
There is a C/C++ interface. It can be found at the related project [MathCatForC](https://github.com/NSoiffer/MathCATForC). Rust and C have separate memory managers, and so the interface is a little clunky because the memory needs to be free'd. That can be hidden by wrapping the calls in a small function as demonstrated by `SetMathCatPreference` in the [sample code](https://github.com/NSoiffer/MathCATForC/blob/main/c-example/test.cpp). Otherwise, it is easy to use. If someone knows a better way to deal with the memory issues, please let me know or submit a PR. This is new territory for me as a Rust programmer.