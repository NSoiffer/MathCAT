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