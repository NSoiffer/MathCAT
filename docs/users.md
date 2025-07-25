# <img src="logo.png" style="position: relative; top: 16px; z-index: -1;" alt="MathCAT logo"> User Guide

## Advice on Choosing a Voice

In NVDA, you have a choice of using different synthesizers. This is found in NVDA `Preferences:Settings...` followed by choosing `Speech`. There are usually at least three choices: eSpeak NG, Microsoft Speech API, Windows OneCore voices. All synthesizers work, but the Windows OneCore voices don't support speaking "a" properly and the other options should be used. In particular, the Microsoft Speech API are a good substitute for the OneCore voices.

## Information for MathCAT users

MathCAT supports a number of options to control speech, braille, and navigation. These are described below.
Not all options are currently supported. The current state of support for an option is listed with the option and a ✓ is used before the option as a quick reference to indicate at least partial support for that option.

Note: in NVDA, the options can be set by using the MathCAT preferences dialog. This is accessed by going to the NVDA preferences, choosing "Preferences", and then "MathCAT settings...". The settings are divided into three categories: Speech, Navigation, and Braille. This division is reflected in the documentation below.

MathCAT supports multiple modes of navigation. The means to begin navigating and end navigating will differ depending on the AT you are using. See the list below. The commands/key-strokes accepted by MathCAT are the same as those accepted by MathPlayer and are [listed in this document](nav-commands.md).
The documentation describes many useful ways to navigate math. For those who just want to get started:

* Use the arrow keys to move left/right or up/down a mathematical structure (e.g., into/out of a fraction)
* If inside of a table, CTRL+arrow will move by cell
* Home/End moves to the start/end of the expression
* Space reads your current position
* Shift+up/down will change the mode of navigation (see [navigation documentation](nav-commands.md))

To start navigation:

* NVDA:  press NVDA+Alt+M or the space key to enter math navigation mode, press Esc to exit

MathCAT's navigation is the same in Word and in a browser.

While navigating an expression, "control+c" copies the math content of the current node in NVDA. The following formats are supported:

* MathML (Default)
* LaTeX
* ASCIIMath
* Speech

## Option List

The options are listed below. Most options allow only a limited set of values and those are indicated in the options.
The default value is given in \[brackets\].

### Speech Options

* ✓Impairment: [Blindness]
  * Options: Blindness, LowVision, LearningDisability
  * Description: this controls whether certain notations are disambiguated or not in speech.
  * Status: the focus has been on Blindness, but there is some support if a different value is used. That support needs to be improved.

* ✓Language: [en]
  * Options: any known language code and sub-code. E.g., "en-uk".
    [This site gives a list of options](https://www.venea.net/web/culture_code).
  * Description: this value determines the language to be used.
    If the regional variant is not found among the speech rules, the speech will fall back to using the main language. If speech rules for the main language can not be found, English ("en") is used.
  * Status: currently only English, Spanish, Finnish, Indonesian, Swedish, Vietnamese, and Chinese are supported.
    Support for other languages will be added with help from volunteers.

* ✓SpeechStyle: [ClearSpeak]
  * Options:  Any implemented speech style (currently only ClearSpeak and SimpleSpeak)
  * Description: a style of speech or coordinated philosophy about how to speak an expression.
    * ClearSpeak was developed by ETS for use on high-stakes tests such as the SAT. The [ClearSpeak spec details are in this Word document](ClearSpeakRulesAndPreferences.docx).
    * SimpleSpeak tries to minimize speech by speaking simple expressions such as $\frac{a}{b}$ quickly without bracketing words ("a over b"); these are distinguished from more complex expressions such as $\frac{a}{b+1}$ which will always have bracketing words ("fraction a over b plus 1 end fraction").
  * Status: currently only ClearSpeak and SimpleSpeak are implemented, but MathSpeak will likely be implemented at some point.

* ✓Verbosity: [Medium]  
  * Options: Terse, Medium, Verbose
  * Description: controls how much "extra" speech is used. E.g, square roots are verbosely spoken as "the square root of x" and tersely spoken as "square root x".
  * Status: supported, but there will likely be improvements made over time

* ✓MathRate: [100]
  * Options: Number between 1 and 100
  * Description: Changes the relative speech rate. The change is a percentage speed change from the standard speech engine's rate. '100' means the math reading rate is the same as that of the text rate.
    This only works for implementations that tell MathCAT to generate speech engine tagging such as SSML.
  * Status: This should work in NVDA.

* ✓PauseFactor: [50]
  * Options: Number between 0 and 100
  * Description: Changes the relative amount of pausing that MathCAT adds. 0 turns off all pausing and 100 makes the pauses 10 times longer than normal.
    This only works for implementations that tell MathCAT to generate speech engine tagging such as SSML.
  * Status: This should work in NVDA.

* ✓SpeechSound: [None]
  * Options: None, Beep
  * Description: a start and end beep occur before and after reading an expression.
  * Status: This should work in NVDA.

* SubjectArea: [General]
  * Status: this was used in MathPlayer but not yet currently implemented. I am waiting on further discussion in the MathML which might add this as a means of providing different default `intent` values.

* Chemistry: [SpellOut]
  * Options:  SpellOut, AsCompound, Off
  * Description:  controls how Chemical formulae are read. Examples for $\mathrm{H}_2\mathrm{O}$:
    * ✓SpellOut: "H 2 0" (verbosity controls whether "sub"/"super" is spoken)
    * AsCompound: "Water"
    * ✓Off "H sub 2 O"
  * Status: Many heuristics have been implemented to infer when some notation is chemistry or not. Inferring chemical notations is a bit tricky so MathCAT will sometimes not recognize them and may sometimes inadvertently classify something as chemistry. The work of the MathML WG may make it substantially easier for authors to indicate that something is chemistry.

SpeechOverrides:

* ✓CapitalLetters: "cap"    # word to say as a prefix for capital letters unless in unicode.yaml; empty string leaves it to screen reader
* LeftParen: ""             # word used as override
* RightParen: ""            # word used as override

ClearSpeak has a number of options. These were designed for authors to use, but can also be set by a user although they are not that useful.

* ✓CapitalLetters: Auto, SayCaps or use pitch
* ✓AbsoluteValue: Auto, AbsEnd, Cardinality, Determinant
* ✓Fraction: Auto, Ordinal, Over, FracOver, General, EndFrac, GeneralEndFrac, OverEndFrac, Per
* ✓Exponent: Auto, Ordinal, OrdinalPower, AfterPower
* ✓Roots: Auto, PosNegSqRoot, RootEnd, PosNegSqRootEnd
* ✓Functions: Auto, None
* ✓Trig: Auto, TrigInverse, ArcTrig
* ✓Log: Auto, LnAsNaturalLog 
* ✓ImpliedTimes: Auto, MoreImpliedTimes , None
* ✓Paren: Auto, Speak, SpeakNestingLevel, Silent, CoordPoint, Interval
* ✓Matrix: Auto, SpeakColNum, SilentColNum, EndMatrix, Vector, EndVector, Combinatorics
* ✓MultiLineLabel: Auto, Case, Constraint, Equation, Line, None, Row, Step 
* ✓MultiLineOverview: Auto, None, 
* ✓MultiLinePausesBetweenColumns: Short, Long
* ✓Sets: Auto, woAll, SilentBracket
* ✓MultSymbolX: Auto, By, Cross
* ✓MultSymbolDot: Auto, Dot
* ✓TriangleSymbol: Auto, Delta
* ✓Ellipses: Auto, AndSoOn, 
* ✓VerticalLine: Auto, SuchThat, Divides, Given
* ✓SetMemberSymbol: Auto, Belongs, Element, Member
* ✓Prime: Auto, Angle, Length
* ✓CombinationPermutation: Auto, ChoosePermute
* ✓Bar: Auto, Bar, Conjugate, Mean

### Navigation Options (see [navigation documentation](nav-commands.md))

* ✓NavMode: Enhanced -- Enhanced, Simple, Character
* ResetNavMode: false -- remember previous value and use it
* Overview: false -- speak the expression or give a description/overview
* ResetOverView: true -- remember previous value and use it
* ✓NavVerbosity: Medium -- Terse, Medium, Full (words to say for nav command)
* ✓AutoZoomOut: true -- Auto zoom out of 2D exprs (use shift-arrow to force zoom out if unchecked).
  * `true`: if you are at the edge of a 2D expression (e.g., a fraction or superscript) and you try to move (left or right) out of it, then the move is allowed and the zoom level is set to that of the preceding/following item. 
  * `false`: moving (left or right) past the edge of a 2D expression is not allowed; you need to zoom out (perhaps repeatedly) until you are not at an edge to be able to move (left or right).
* CopyMathAS: Determines the format in which to copy the math content of the current navigation node (MathML, LaTeX, ASCIIMath, or Speech).


### Braille Options

* ✓BrailleCode: [Nemeth]
  * Options: Any implemented braille code
  * Description: the braille math code to use
  * Status: currently ASCIIMath, ASCIIMath-Finnish, CMU, LaTeX, Nemeth, Swedish, UEB, and Vietnam are supported. Other braille code support will depend upon help from others.
* ✓BrailleNavHighlight: [EndPoints]
  * Options: Off, FirstChar, EndPoints, All
  * Description:  highlight with dots 7 & 8 the currently selected navigation node
* UEB:
  * ✓START_MODE: [Grade2] 
    * Options: Grade1, Grade2
    * Description: assumed starting mode UEB braille (Grade1 assumes we are in G1 passage mode)
  * ✓UseSpacesAroundAllOperators: [false]
    * Options: true/false
    * Description: The UEB guidelines suggest that for lower grades, adding space around operators such as `+` and `-` can be a good idea. Normally, space is only added around relational operators such as `=` and `<`.

Braille codes often have author-definable characters. MathCAT provides some options:

Nemeth defines the typeforms: Bold, Italic, SansSerif, and Script. That leaves out DoubleStruck (Blackboard Bold).
Here we provide an option to specify a transcriber-defined typeform changes, with the default mapping DoubleStruck to Italic

* Nemeth:
  * ✓SansSerif:    "⠠⠨"
  * ✓Bold:         "⠸"
  * ✓DoubleStruck: "⠨"
  * ✓Script:       "⠈"
  * ✓Italic:       "⠨"

The [UEB Guide to Technical Material](https://iceb.org/Guidelines_for_Technical_Material_2008-10.pdf) says to normally treat Fraktur and DoubleStruck as Script.
Here we provide an option to specify a transcriber-defined typeform prefix indicator instead.
Note: here are prefixes for 1st - 5th: "⠈⠼", "⠘⠼", "⠸⠼", "⠐⠼", "⠨⠼"

* UEB:
  * ✓DoubleStruck: "⠈"     [script]
  * ✓Fraktur:      "⠈"     [script]
  * ✓SansSerif:    "⠈⠼"    [first transcriber-defined typeform prefix indicator]
  * ✓GreekVariant: "⠨"     [default to Greek]

The characters for Vietnam that are definable is still be discussed. Likely, they will change some.

* Vietnam:
  * ✓UseDropNumbers: [false]
  * Options: true, false
  * Description: drop digits down a row in simple numeric fractions
  * ✓DoubleStruck: "⠈"     [script]
  * ✓Fraktur:      "⠈"     [script]
  * ✓SansSerif:    "⠈⠼"    [first transcriber-defined typeform prefix indicator]
  * ✓GreekVariant: "⠨"     [default to Greek]

### Other Options

MathCAT cleans up bad MathML. Numbers are frequently improperly marked up in MathML. In order to clean them up correctly, MathCAT needs to know locale information about what characters might be used to separate digit blocks and what characters are used a decimal separator. Typically this is set by AT based on the country code in the document. However, that may not be given and only the language code is given and so AT needs to guess based on that.

* DecimalSeparators: "." # [default]
* BlockSeparators: ", \u00A0\u202F" # [default -- includes two forms of non-breaking spaces]
