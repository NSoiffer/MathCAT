# MathCAT: Math Capable Assistive Technology
<img src="logo.png" style="position: relative; top: 16px; z-index: -1;">

## Information for MathCAT Rule Developers/Translators
This page is a work-in-progress.

## Files
MathCAT reads the following files for critical information:
* Rules
  * intent.yaml -- rules that infer author intent from MathML. These are used by various speech styles (in various languages) to avoid duplicating the inference process. They add an `intent` attribute to the MathML.
  * definitions.yaml -- these define various lists used by MathCAT for canonicalization (inferring proper structure) and also rule matching. E.g., `TrigFunctionNames` is a list of names of trig functions such as `tan` and `lim`.
  * prefs.yaml -- system defaults for various preferences that are settable. MathCAT will also look for this file in a platform-specific user location so that individual users can set the values. Eventually, there will be make changes.
    * Windows: %AppData%\\prefs.yaml|
    * Linux:  $XDG_CONFIG_HOME or $HOME/.config
  * definitions.yaml -- language independent definitions (e.g., trig function names).
* Rules/[lang]
  * Unicode.yaml -- a (long) list for how to pronounce each Unicode character that is encountered (not used for multi-char strings).
  * XXX_rules.yaml -- the rules used to speak math. MathCAT will scan every subdirectory of the `Rules` directory for files that have the suffix `_rules.yaml` and add them to the list of options for people to choose. The `XXX` should reflect the speech style. E.g., `ClearSpeak_rules.yaml` and `MathSpeak_rules.yaml` will result in user options to choose "ClearSpeak" and "MathSpeak" for the speech style.
  * definitions.yaml -- language specific definitions such as how to speak ordinal numbers ("first", "half", etc).
  * navigate.yaml -- rules that define what happens for each navigation command along with the speech that is said

The `lang` subdirectory should follow the two letter language and language-region [ISO naming convention](https://en.wikipedia.org/wiki/Language_localisation#Language_tags_and_codes). E.g, there is a `en` subdirectory of the `Rules` directory. If region-specific speech is needed, there can be a region subdirectory such as `gb` that will be used if the language specified is `en-gb`.

MathCAT will first read the main language rules and then read the region-specific rules. The region-specific rules will either replace or add existing rules in the corresponding `Unicode.yaml` and `XXX_rules.yaml` language files. 

MathCAT looks for the `Rules` directory in the following locations:
1. In the directory specified by the environment variable `MathCATRulesDir`
2. In the Rules subdirectory that is a sibling to the executable. Typically this is `C:\Program Files\MathCAT\Rules` on windows.

# File Format
The files (as the suffix implies) are [YAML files](https://lzone.de/cheat-sheet/YAML). For those who aren't familiar with YAML, it is a superset of JSON that offers options that can be more human readable and writeable.

## A YAML Introduction
The basic types in YAML are:
* scalar types like integers, floats, and strings (can be inside of ''s or ""s or left unquoted in some cases)
* arrays (used inline as `["a", "b", "c"]`)
* dictionaries/maps (used inline as `{key: value, foo: bar}`)

In the more verbose form of YAML syntax for arrays, indentation is used instead of brackets so the above array becomes
```
 - a
 - b
 - c
```
Notice that the strings don't need to be quoted in this form (although some text requires quotes).

The dictionary in the more verbose form looks like:
```
   key: value
   foo: bar
```

Here is a more real life example from the Unicode definitions showing various alternatives.
Pay attention to the indentation: all entries that are indented to the right of the line above are subentries in that array/dictionary.
```
# Two options for defining a simple replacement for the symbol '∞'.
# For brevity and clarity, the first form is preferred.
- '∞': {t: "infinity"}
- '∞':
    t: infinity

# Here are a few options for a more complex definition that involves a test
# This compact form is (compact) JSON syntax for the value
- 0x003C: [test: {if:Verbosity!='terse', then: {t: is}}, t: "less than"]

# This form emphasizes (to the reader) that there are two actions: a test followed by producing "less than"
- 0x003C:
    - test: [{if: Verbosity!='terse', then: {t: is}}]
    - t: "less than"

# This is slightly more long-winded but makes it clear what the parts of the test are
- 0x003C:
    - test: 
        if: Verbosity!='terse'
        then: [t: is]
    - t: less than

# This uses the most verbose form of YAML
- 0x003C:
    - test: 
        if: Verbosity!='terse'
        then:
        - t: is
    - t: less than
```
All forms are valid, but the second and third options are preferred as they seem to be good compromises between brevity and clarity

In case it wasn't obvious, "#" indicates a comment and the rest of the line is ignored. There are no block comments in YAML

Note: all YAML files begin with "---". That indicates the beginning of a "document".

## The Basic Parts of a Speech Rule

```
# rule:
#     name: <string> # name of the rule (name+tag should be unique)
#     tag: <string>
#     variables: [{name: value}, ...]
#      - name is a string, value is an XPath expr that evaluates to a string, number, or boolean
#      - inside the rule, the value is accessed as $name
#      - the variable's value is set _before_ testing "match", so it can be used in match
#      -   they are valid for the duration of the match
#     match: <string>  # xpath for the match
#      - can be a single string or
#      - an array of strings (for readability) that are joined together
#     replace:  [replacements] where replacements are one of the following
#      - t: some text
#          'T' is used to indicate text has been translated.
#      - ct: concatenate text without space in front
#           'CT' is used to indicate text has been translated
#      - ot: optional text (don't use text if it results in repeated words)
#           'OT' to indicate text has been translated
#           E.g., we don't want "t raised to the the fraction with ...."
#           Making "the" optional in the fraction rule prevents the repetition
#      - x: some xpath (as string)
#      - test:  values are conventional if/then/else with two twists:
#                the first twist is that there is an option to use either 'then_test:' or 'else_test'
#                  This avoids another level of 'test:'
#                the second twist is that any number of if/else_if pairs can be given;
#                  these are tested in order until one is true
#            The value of "test:" can either be an array of if/else_if/else keys or a single if/then/else key for convenience.
#              If an array, then the first entry should be 'if', the middle (and maybe last) 'else_if', and the optional
#              last one can be 'else'/'else_test'
#         if: <string> some xpath
#         then: [replacements]
#         then_test [replacements] used in place of 'then:' -- avoids needing to use 'test:' after the 'then:'
#         else: [replacements] # optional
#         else_test # optional, used in place of 'else:' -- avoids needing to use 'test:' after the 'else:'
#      - with:
#         variables: [name: value, ...] variables whose values are set during the execution of this clause
#         replace: [replacements]
#      - set_variables: [var: value, ...] global variable definitions.
#         These are available to the program after the rules have run; currently used for navigation which can change state.
#      - pause: string or number  # "short", "medium", "long", "auto", or number in milliseconds
#      - rate:  string/number or dict with 1 or 2 entries
#         value: float number with optional %
#         replace: [replacements]  # tts values need to scope contents 
#      - volume:  string/number or dict with 1 or 2 entries
#         value: float number with optional %
#         replace: [replacements]  # tts values need to scope contents 
#      - pitch:  string/number or dict with 1 or 2 entries
#         value: float number with optional %
#         replace: [replacements]  # tts values need to scope contents 
#      - gender:  string/number or dict with 1 or 2 entries
#         value: "male" # or "female"
#         replace: [replacements]  # tts values need to scope contents 
#      - voice:  string/number or dict with 1 or 2 entries
#         value: string
#         replace: [replacements]  # tts values need to scope contents 
#      - spell:  string (usually a single letter to be pronounced as the letter)
#      - bookmark: some xpath (as string) returns an 'id' that can be used for synchronized highlighting
```

Note: for "pause", the "auto" value will calculate a pausing amount based on the complexity of the surrounding parts. The more complex they are, the longer the pause (up to a limit). The basic idea is that you want to give the listener time to digest and separate out the two parts when one or both are more complicated.

In addition to having a named rule, the speech rule file supports including other speech rules files. This lets various speech speech rule styles share common features. Inclusion is done via an entry in place of a speech rule:
```
  -include: file_name
```
Any number of includes can occur in a file. They are processed as if the contents of the included file were in the original file. The file name may be located in the current directory of the rule file being processed in or some relative directory to the current directory.
## The Unicode Files

Unicode files are simplified versions of the speech rules. This makes it easier to specify rules for Unicode characters and also results in a significant speed boost. Rules on leaf elements such as `mo` will override any definition in the Unicode files. In general however, speech rules for Unicode characters should be in a Unicode file.

Like speech rules, Unicode files are YAML files. The main difference is that only the character is used for defining the rule. There is no need to specify a rule name, tag name, match expression, etc. The value of a rule can be anything that is value as a "replace:" value for speech rules.

Most rules are very simple. Here is an example:
```
 - "+": [t: plus]                                # 0x2b
```
This rule will translate the "+" character int the string "plus".

A more complicated rule is:
```
 - "[":                                          # 0x5b
    - test:
        if: $SpeechStyle = 'ClearSpeak'
        then: [t: open bracket]
        else: [t: left bracket]                            
```
This rule produces different speech depending on the current preference for the speech style.

It is also possible to share Unicode files via `- include: file_name` just as it is possible to do so with speech rules.


## The Prefs Files

Note: Preferences such as the ClearSpeak preferences is a dictionary within the `ClearSpeak`
entry in the YAML file. That would make setting the value and reading it difficult.
The solution adopted to convert it to a string with an "_" as a separator.
For example, the _name_ for the `ClearSpeak` `Fraction` preference is `ClearSpeak_Fraction`.
This is what should be used when setting its value via the API and when accessing its value in `ClearSpeak_Rules.yaml`.


## The Definition Files


## XPath
Many parts of a speech rule make use of xpath. This is a popular and well documented method for selecting parts on an XML document. A web search will turn up many tutorials. Those not familiar with xpath are encouraged to read some. The implementation of xpath used by MathCAT is a slightly extended version of XPATH 1.0.
Many parts of a speech rule make use of xpath. This is a popular and well documented method for selecting parts on an XML document. A web search will turn up many tutorials. Those not familiar with xpath are encouraged to read some. The implementation of xpath used by MathCAT is a slightly extended version of XPATH 1.0.

MathCAT usage tends to use only a few features of xpath. It also makes use of some custom functions. Here is a short explanation of common xpath usage:

| usage | meaning |
| ----- | ---- |
| `*`    | matches all children |
| `[...]` | selects nodes from the current match| 
|  `*[1]`  | selects first child|
| `*[self::m:mn]` | selects all children that are `mn` elements. Note that `m` is used to indicate that the element is in the MathML namespace.
| `*[1][self::m:mn]` | select the first child as long as it is an `mn` element |
| `*[1][self::m:mo][text()='-']` | select the first child as long as it is an `mo` element whose content is '-'. This could also be written as `*[1][text()='-']` because other nodes probably won't have the content `-`, but an `mtext` element could have that, so specifying the element name is safest. |
| `count(*[2]/*)` | the number of children of the second child |
| `count(preceding-sibling::*)+1` | add 1 to the number of siblings before the current element |

MathCAT adds some custom functions to make writing rules easier:
| function | meaning |
| ----- | ---- |
| `IsNode(nodes, type)   | Returns true if all of the nodes are of the same type. Type can be one of:<br/>  "simple" -- a defined set of elements in ClearSpeak <br/> "leaf" -- one of the MathML leaf elements <br/> "common_fraction" -- integer numerator and denominator<br/> "trig_name" -- sin, cos, tan, sinh, cosh, etc |
| ToOrdinal |  |
| ToCommonFraction | |
| IsLargeOp(nodes) | Returns true if all of the nodes are large operators |
| IsBracketed | |
| BaseNode(node) | Returns the base (recursively) of a scripted node |
| IsInDefinition(node, name) | Returns true if node is a member of the list 'name' (defined in definitions.yaml) |
| DEBUG(xpath) | |

These are used by Nemeth Rules
| function | meaning |
| NestingChars | Used by mfrac, msqrt, and mroot rules to repeat the chars the appropriate number of times |
| NemethChars | Used by token elements to deal with the complicated rearrangement of various Nemeth indicators such as capitalization and font face |