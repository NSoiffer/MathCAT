# <img src="logo.png" style="position: relative; top: 16px; z-index: -1;"> User Guide

## Advice on Choosing a Voice
In NVDA, you have a choice of using different synthesizers. This is found in NVDA `Preferences:Settings...` followed by choosing `Speech`. There are usually at least three choices: eSpeak NG, Microsoft Speech API, Windows OneCore voices. All synthesizers work, but the Windows OneCore voices don't support speaking "a" properly and the other options should be used. In particular, the Microsoft Speech API are a good substitute for the OneCore voices.

## Information for MathCAT users
MathCAT supports a number of options to control speech, braille, and navigation. These are described below.
Not all options are currently supported. The current state of support for an option is listed with the option and a ✓ is used before the option as a quick reference to indicate at least partial support for that option.

Note: currently, the options can only be changed by modifying a text file. Work is being done to provide a more friendly (accessible) GUI.
This documentation will be updated when the GUI is ready to be used.

MathCAT supports multiple modes of navigation. The means to begin navigating and end navigating will differ depending on the AT you are using. See the list below. The commands/key-strokes accepted by MathCAT are the same as those accepted by MathPlayer and are [listed in this document](https://docs.wiris.com/en/mathplayer/navigation_commands). MathCAT's navigation is the same in Word and in a browser.

To start navigation:
* NVDA:  press NVDA+Alt+M to enter math navigation mode, press Esc to exit

## Option List
The options are listed below. Most options allow only a limited set of values and those are indicated in the options.
The default value is given in \[brackets\].

### Speech Options
* ✓Impairment: [Blindness]
  * Options: Blindness, LowVision, LearningDisability
  * Description: this controls whether certain notations are disambiguated or not in speech.
  * Status: the focus has been on Blindness, but there is some support if a different value is used. That support needs to be improved.

* Language: [en]
  * Options: any known language code and sub-code. E.g., "en-uk".
    [This site gives a list of options](https://www.venea.net/web/culture_code).
  * Description: this value determines the language to be used.
    If the regional variant is not found among the speech rules, the speech will fall back to using the main language. If speech rules for the main language can not be found, English ("en") is used.
  * Status: currently only English is supported.
    Support for other languages will added with help from volunteers.

* ✓SpeechStyle: [ClearSpeak]
  * Options:  Any implemented speech style
  * Description: a style of speech speech or coordinated philosophy of how t speak an expression. ClearSpeak with developed by ETS for use on high stake tests such as the SAT. SimpleSpeak tries to minimize speech by speaking simple things quickly without bracketing words; these are distinguished from more complex expressions such as $\frac{a}{b+1}$ which will always have bracketing words. 
  * Status: currently only ClearSpeak and SimpleSpeak are implemented, but MathSpeak will likely be implemented at some point.

* ✓Verbosity: [Medium]  
    * Options: Terse, Medium, Verbose
    * Description: controls how much "extra" speech is used. E.g, square roots are verbosely spoken as "the square root of x" and tersely spoken as "square root x".
    * Status: supported, but there will likely be improvements made over time

* ✓MathRate: [100]
    * Options: Number between 1 and 1000(?)
    * Description: percentage speed change from standard speech engine rate. '100' means the math reading rate is the same as that of the text rate.
      This only works for implementations that tell MathCAT to generate speech engine tagging such as SSML.
    * Status: This should work in NVDA.


* SubjectArea: [General]
  * Status: this was used in MathPlayer but not yet currently implemented. I am waiting on further discussion in the MathML which might add this as a means of providing different default `intent` values.

* Chemistry: [SpellOut]
  * Options:  SpellOut, AsCompound, Off
  * Description:  controls how Chemical formulae are read. Examples for $\mathrm{H}_2\mathrm{0}$:
    * SpellOut: "H 2 0"
    * AsCompound: "Water"
    * Off "H sub 2 O"
  * Status: not yet implemented. Inferring Chemical notations is a bit tricky so MathCAT will sometimes not pick it up. The work of the MathML WG may make it substantially easier for authors to indicate that something is Chemistry.

SpeechOverrides:
* ✓CapitalLetters: "cap"     # word to say as a prefix for capital letters unless in unicode.yaml; empty string leaves it to screen reader
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
* MultiLineOverview: Auto, None, 
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


### Navigation Options
* ✓NavMode: Enhanced -- Enhanced, Simple, Character
* ResetNavMode: false -- remember previous value and use it
* Overview: false -- speak the expression or give a description/overview
* ResetOverView: true -- remember previous value and use it
* ✓NavVerbosity: Medium -- Terse, Medium, Full (words to say for nav command)
* ✓AutoZoomOut: true -- Auto zoom out of 2D exprs (use shift-arrow to force zoom out if unchecked)


### Braille Options
* Code: [Nemeth]
  * Options: Any implemented braille code
  * Description: the braille math code to use
  * Status: currently only Nemeth and UEB are supported. Other braille code support will depend upon help from others.
* BrailleNavHighlight: [EndPoints]
  * Options: Off, FirstChar, EndPoints, All
  * Description:  highlight with dots 7 & 8 the currently selected navigation node

