# MathCAT: Math Capable Assistive Technology
<img src="logo.png" style="position: relative; top: 16px; z-index: -1;">

## Information for AT and Other Library Users

When calling from python, the general ordering is:
1. The location of the MathCAT `Rules` directory is set [SetRulesDir]
1. Whatever preferences the AT needs to set, it is done with calls to [`SetPreference`]. Typically the `Language` and TTS engine to use (if any -- strongly recommended) are given. 
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
  This is needed when navigating by character for multi-symbol leaf nodes such as "sin" and "1234"

It is also possible to find out what preferences are currently set by calling [`GetPreference`]

All functions return a potential error code.

## Rust Users
MathCAT is written in Rust, so all you need to do is build MathCAT and in your project's Cargo.toml file add something like
```
[dependencies.MathCAT]
path = "../MathCAT/"
```

The exact function signatures are (with comments):
```
/// Set the Rules directory
/// IMPORTANT: this should be the very first call to MathCAT unless the environment var MathCATRulesDir is set
pub fn SetRulesDir(dir: String) -> Result<()>

/// The MathML to be spoken, brailled, or navigated.
///
/// This will override any previous MathML that was set.
pub fn SetMathML(mathml_str: String) -> Result<String> 

/// Get the spoken text of the MathML that was set.
/// The speech takes into account any AT or user preferences.
pub fn GetSpokenText() -> Result<String> }

/// Get the spoken text of the MathML that was set.
/// The speech takes into account any AT or user preferences.
pub fn GetOverviewText() -> Result<String> 

/// Set an API preference. The preference name should be a known preference name.
/// The value should either be a string or a number (depending upon the preference being set)
/// If 'name' is not known, or the value is not legal, an error is returned.
///
/// This function can be called multiple times to set different values.
/// The values are persistent but can be overwritten by setting a preference with the same name and a different value.
pub enum StringOrFloat {
    AsString(String),
    AsFloat(f64),
}
pub fn SetPreference(name: String, value: StringOrFloat) -> Result<()> {

/// Get the preference. If 'name' is not known, None is returned.
pub fn GetPreference(name: String) -> Option<String> 

```

## Python Users
You can build your own Python interface, or use the one that is built with the related project [MathCATForPython](https://github.com/NSoiffer/MathCATForPython). This uses the Rust package pyo3.

The Python interface is basically the same as the Rust interface. When calling a function, it should be wrapped in
```
try:
    ...
except Exception as e:
    ...  # log the error 'e'
```


## Web Users
I built a web assembly version. Has a few compromises and requires some hand tweaks during the build process. Those need to be automated. It can be found at [MathCatDemo](https://github.com/NSoiffer/MathCATDemo). This builds a web page for demo purposes, so it is not a pure build for the Web. Nonetheless, it does demonstrate how that can be done.

## C/C++ Users
It should be easy to add a C/C++ interface, but I haven't done this yet. Please contact me if you are interested in using MathCAT from a C/C++ application.