# <img src="logo.png" style="position: relative; top: 16px; z-index: -1;"> Translator and Rule Developer Guide

## Information for MathCAT Rule Developers/Translators
This page is a work-in-progress.

## Getting Started
If you plan to work on MathCAT development, you need to make use of github:
1. Fork the MathCAT repo at `github.com/NSoiffer/MathCAT`
2. Clone the the forked copy so you have a local copy to work on.
3. Checkout the branch I create for your work (typically the country code for your translation) and work in that branch.

If you are unfamiliar with these steps, a simple search will turn up lots of places that describe how to do them. They are simple, so don't get put off by your unfamiliarity.


## Language Translators
If you are a translator, please contact @NSoiffer and he will set up an initial translation that could save a large amount of time. This initial translation will create files in Rules/Languages/xx, where 'xx' is your country code (e.g., fr, de, el, ...). This directory is where you will make your translations. There are four categories of files you should edit:
1. definitions.yaml: this has a number of translations for numbers, both cardinal and ordinal numbers. Look through those initial translations and make any corrections needed. These numbers are used for things like saying "three fifths". Languages start to have regular counting patterns at some point and so some of the lists in that file can be shortened and some may need additional entries. There are some more details in the English comments in the file.
2. The xxx_Rules.yaml files (currently `ClearSpeak_Rules.yaml` and `SimpleSpeak_Rules.yaml`). These represent different styles of speech. I strongly recommend you just pick one to start with. These files typically have the words that describe the structure such as "fraction" and "power" along with connective words such as "the", "of", and "from". Because there is a lot of similarity between the two styles of speech, there is also a `SharedRules` folder with rule files in it. These are included (via rules `- include file_name`)
into `ClearSpeak_Rules.yaml` and `SimpleSpeak_Rules.yaml`. They need to be translated also.
<br/>
<br/>
Note: The MathCAT settings dialog looks for files named `XXX_Rules.yaml` and adds them to pull down menu for the language. You don't need to use the SimpleSpeak and ClearSpeak names. If you only want to do one translation (e.g, SimpleSpeak), but don't want to delete `ClearSpeak_Rules.yaml` rename it to something like `ClearSpeak_Rules.yaml.untranslated`.
<br/>
<br/>
These files have auto-generated initial translations. Even though they are translated, `t:` (see below) is used, not the upper case `T:`. This is because each translation should be verified to be correct and when verified, then change to the uppercase version.
See below for more comments about the auto translations.

    * In some languages it doesn't make sense to says "_the_ square root of x" (and maybe "of"). If that is the case, just change those to empty strings.
    * Some languages, the word order changes -- feel free to move the words around, but pay attention to the indentation.
    Indentation is meaningful in YAML. 
    * In some languages, you may want to add words that aren't in the English version, perhaps before or after existing phrases. Feel free to add them -- they can be conditionally added using `test` if needed. Please contact @NSoiffer if you need help with this.
    * Pausing between words/phrases can greatly help make understandable. The pausing is choosen based on English. You should adjust pauses based on what sounds good in speech synthesizers for your language. It is very simple to add, remove, or change the amount of pauses. All pauses are scaled to the current speech rate.
3. The unicode files (`unicode.yaml` and `unicode-full.yaml`). These contain characters like `<` and `∫`.
    * You should start with translating `unicode.yaml`. These represent the vast majority of math symbols used. Currently the list is based on experience as to which are the most commonly used Unicode symbols, but I plan to make use of statistics from actual books to refine the list even further. There are about 270 characters to translate in `unicode.yaml`, although ~50 of them are Greek letters (which is hopefully simple).
    Just like the speech rule files, these files have auto-generated initial translations and the translations should be verified and the `t:` changed to `T:`.
    See below for more comments about the auto translations.
    * The `unicode-full.yaml` is thousands of lines long. Once you have done other translations, I would come back to this file and work through it until you reach a point of being exhausted -- most of these characters will only show up in very advanced mathematics, and even then, only very rarely. The most important of these characters are probably:
         * Some of the arrows that start at 0x2190
         * The characters in the math symbols block: 0x2200 - 0x22ff
         * Some accents: 0x2d8-0x2dd
         * Some of the simple black/white shapes starting at: 0x25a0 and also at 0x2b1a
4. The navigation files `navigate.yaml` and `overview.yaml`. Just translate `navigate.yaml`; `overview.yaml` is not ready to be used. Many of the words in `navigate.yaml` are repeated many times, so you probably want to do a global search/replace. I hope to rewrite the file at some point and isolate the words.

__NOTE__: I am most of the way through the process of changing the rules to make use of `intent`. This will move the complicated logic of recognizing things like absolute value and determinants into the `intent` folder which is language-independent. It makes translations simpler because the rule only needs to match the tag "absolute-value" or "determinant". The tests also should be separated out into an `intent` directory that is language independent.

### Marking text as translated
These files are YAML files and their content is described later in this page.
In all of these files, the text to translate will have the YAML key name `t` (and very rarely `ot`, `ct`, `spell`, `pronounce`, and `IfThenElse`). When you make a translation, you should capitalize them (e.g, `T`, `IFTHENELSE`) to indicate that the file has been translated.

As an example, here are two rules from `unicode.yaml`:
```
 - "=": [t: "equals"]                            # 0x3d
 - ">":                                          # 0x3e
     - test: 
         if: "$Verbosity!='Terse'"
         then: [t: "is"]
     - t: "greater than"
```
If you were translating this to French, the words after the `t:` would get changed to (probably):
```
 - "=": [T: "égale"]                            # 0x3d

 - ">":                                         # 0x3e
     - test: 
         if: "$Verbosity!='Terse'"
         then: [T: "est"]
     - T: "supérieur à"
```

Note: `IfThenElse` may not require a translation but should be changed regardless so you know that has been looked at. Here's an example where no translation is needed because the "then" and "else" parts (`count(*/*[1])` and `$LineCountTry` respectively) are not words:
```
 - LineCount: "IfThenElse($LineCountTry=0, count(*/*[1]), $LineCountTry)"
```

See below for a discussion of what can be used in a rule file.

### A note about the translated files
To derive an initial translation for the Unicode files, both MathPlayer's and SRE's translations are used. Google translate is also used.
If SRE and MathPlayer agree, or if only one of SRE or MathPlayer has a translation but that translation agrees with the google translation, then only the original English version will be part of a comment at the end. For example:
```
 - "!": [t: "factorielle"]                      	#  0x21	(en: 'factorial')
```

If the MathPlayer and SRE translations disagree, then the translations that agrees with the google translation will be chosen and the other translation included in a comment. For example:
```
        else: [t: "parenthèse gauche"]          	# 	(en: 'left paren', MathPlayer: 'parenthèse ouvrante')
```
If none of the translations agree, than one of the translations is picked and the other translations are in comment. For example:
```
            else: [t: "parenthèse gauche"]      	# 	(en: 'open paren', MathPlayer: 'parenthèse ouvrante', google: 'parenthèse ouverte')
```
Finally, if there there is no translation, then the google translation is given and is marked with a comment "google translation". There is a significant chance that this is not a good translation so pay special attention to those. Here is an example where there is only a google translation
```
          then: [t: "ligne verticale"]          	# 	(en: 'vertical line', google translation)
```


### Trying out your translation
Once you've done some translations and want to try them out, you can do so immediately if using NVDA. Assuming you have the MathCAT addon:
1. Copy your new translation directory to `%AppData%\nvda\addons\MathCAT\globalPlugins\MathCAT\Rules\Languages`.
2. Start NVDA and go to the MathCAT settings menu (NVDA preferences: MathCAT settings..).
3. Under the "Languages" drop down you should see your new language. Select that.
4. Try out the speech. Wikipedia pages are a good source for examples.
5. If there is an error (often you won't hear speech), open NVDA's log (in NVDA's "Tools" submenu). The error should be listed there. The error messages are explained below.
6. When you make a change, MathCAT should notice the file is changed and reload it. There is currently a bug that this is not done for files that are `include`d in from a file (e.g., all those in the Shared directory). If you make a change to one of those files, either reload MathCAT (NVDA Tools:Reload Plugins) or restart NVDA.

Translating the settings dialog: this is a separate process from translating the speech. This done by volunteers that do other addon translations also. See [this mailing list](https://groups.io/g/nvda-translations) for more info.

### Automatic tests for your translation
Testing is very important! MathCAT is written in Rust and has a large number of automated tests. These tests take advantage of the builtin Rust test system. Hence, to write and verify your own tests, you need to [download and install Rust](https://www.rust-lang.org/tools/install). You do not need to know Rust -- you will simply change some strings from what they are in English to what you think they should be in your language.

For the sake of discussion, let's assume you are doing a French translation, then your country code is `fr`.

To start, in the tests directory, open `languages.rs` and add the line `mod fr;` after `mod en;` or any other similar line for a different language.

In the `tests\Languages` directory, there is a file `en.rs` and a directory `en`. 
1. Copy `en.rs` to `fr.rs`.
2. Copy the `en` directory to `fr`.
3. If you only choose one speech style (e.g., "SimpleSpeak), edit `fr.rs` and remove the lines starting `mod ClearSpeak {` all the way down to the matching `}`. In the `fr` directory, remove the subdirectory `ClearSpeak`.
4. Although it is good translate all the files, it is probably ok to just translate a few of them, especially at the start. In `fr.rs`, comment out any untranslated file by adding `//` in front of the untranslated files. E.g., if you didn't translate the SimpleSpeak file `geometry.yaml`, then the line should look like `// mod geometry;`
5. Start editing the files, first doing a global change of `"en"` to `"fr"` and then replacing the English string with the appropriate French (or whatever language you added) string.

An example of a test is
```
#[test]
fn common_fraction_half() {
    let expr = "<math>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("en", "SimpleSpeak", expr, "1 half");
}
```
For French, the "test" line would change to:
```
    test("fr", "SimpleSpeak", expr, "un demi");
```

Now that you have some tests translated, try running the automated tests.
As a check that everything is set up properly, verify that the English version of the tests are working
```
cargo test Languages::en
```
If that is working, try your tests. Again assuming your created a `fr` version:
```
cargo test Languages::fr
```
MathCAT adds pausing in places and in the test strings, these appear as `,` and `;`. You may need to adjust your expected output by adding or removing those. If those pauses seem inappropriate, you will need to add or remove `pause: xxx` from the appropriate place in the one of the `Rules\fr` files.

__A suggestion__: it might be fastest if you run the tests in your language before changing the expected output. All the tests will fail but you will see failure messages that show the speech that MathCAT generated (in your language). _If it is correct_, simply copy it in place of the English. Once you've done that for all the "errors", rerun the tests and hopefully there won't be anymore errors.


### Keeping the translation up-to-date
To be written...

I hope to eventually have a tool that will
1. warn about missing translations
2. warn about rules in the `en` that have not been copied to another language (likely due to new rules having been added to English)

These tools will look for untranslated and translated text.


## Braille translators
If you want support for a new braille language, you probably need to start from scratch unless the language is similar to an existing braille language.
You will need to create three `.yaml` files in `Rules\Braille\your-braille-language`. This should mirror the files that are in the other braille directories:
1. xxx_Rules.yaml -- where 'xxx' is the name of your new braille language. These will contain the rules that translate MathML to braille
2. unicode.yaml -- this is a translation of the more common braille characters. Use `Nemeth\unicode.yaml` as a starting point for the the translation. Convert the `t: xxx` into what is appropriate for your language. You likely need to delete some logic or maybe add some of your own for characters that might be represented differently based on context. For example, in Nemeth, a "," is represented differently if it is part of a number.
3. unicode-full.yaml -- this is the rest of the character translations.

The reason for two separate unicode files is that having a shorter file for the most common characters means startup takes less time. The goal of that file is to capture 99.99% of the characters used.

For both UEB and Nemeth, some cleanup code needed to be written in Rust. If you are doing a braille translation and cleanup needs to be done, please file an issue and we can work together to get the code written.

To try out your braille translation, you can do so immediately. Please see the instructions above for doing a language translation where it instructs on copying the files to `%AppData%\nvda\addons\MathCAT\globalPlugins\MathCAT\Rules\Languages`. Change languages to `Braille` and most things will be the same.

For automated testing, the instructions above should be followed. The current tests are taken from braille guides for Nemeth/UEB, and you may want to do the same. See the tests in the Nemeth or UEB directories for examples of what braille tests look like.

## Understanding MathCAT Error Message
If there is a problem with a rule that causes an error, these print to the terminal console if you are running MathCAT directly or to NVDA's log if you are using NVDA.

The error messages can be confusing to understand. Here is a description of one and how to understand what is saying.

Because the library that is used in MathCAT to read YAML files does not keep lines numbers, MathCAT is not able to report line numbers.
Instead, it reports the file name and rule's `name` and `tag` within that file.
It then (recursively) reports which section of the rule has the error.

Here's an example of an error message where "test:" was  changed to "textx:" to cause an error:
```
caused by: in file "...\\MathCAT\\Rules\\Languages\\en\\ClearSpeak_Rules.yaml"
caused by: value for 'replace' in rule (fraction: fraction-over-simple). Replacements:
  - test:
      if: "$ClearSpeak_Fractions='FracOver'"
      then:
        - testx:
            if: "$Verbosity!='Terse'"
            then: [ot: the]
        - t: fraction
  - x: "*[1]"
  - t: over
  - x: "*[2]"
  - test:
      if: "$ClearSpeak_Fractions='OverEndFrac' or ($ClearSpeak_Fractions='EndFrac' and not( ($ClearSpeak_Fractions='Auto' or $ClearSpeak_Fractions='Ordinal' or $ClearSpeak_Fractions='EndFrac') and *[1][*[1][self::m:mn][not(contains(., '.')) and ($ClearSpeak_Fractions='Ordinal' or text()<20)]   and *[2][self::m:mn][not(contains(., '.')) and ($ClearSpeak_Fractions='Ordinal' or (2<= text() and text()<=10))] ] and *[2][*[1][self::m:mn][not(contains(., '.')) and ($ClearSpeak_Fractions='Ordinal' or text()<20)]   and *[2][self::m:mn][not(contains(., '.')) and ($ClearSpeak_Fractions='Ordinal' or (2<= text() and text()<=10))] ] ) )"
      then:
        - pause: short
        - t: end fraction
        - pause: short
caused by: replacement #1 of 5
caused by: replacement #1 of 2
caused by: Unknown 'replace' command (testx) with value:  if: "$Verbosity!='Terse'" then: [ot: the]
```
To give some explanation:
The first two lines tell you the file, and the "tag" and "name" values. Here's that rule:
```
- name: fraction-over-simple
  tag: fraction
  match:
  - "($ClearSpeak_Fractions='Over' or $ClearSpeak_Fractions='FracOver' or $ClearSpeak_Fractions='OverEndFrac') or"
  - "( not($ClearSpeak_Fractions='General' or $ClearSpeak_Fractions='GeneralEndFrac') and"
  - "  (IsNode(*[1],'simple') and IsNode(*[2],'simple')) )" # simple fraction in ClearSpeak spec
  replace:
  - test:
      if: "$ClearSpeak_Fractions='FracOver'"
      then:
      - testx:
          if: "$Verbosity!='Terse'"
          then: [{ot: "the"}]
      - t: "fraction"
  - x: "*[1]"
  - t: "over"
  - x: "*[2]"
  - test:
      # very ugly!!! -- replicate nested ordinal fraction as they are an exception
      if: "$ClearSpeak_Fractions='OverEndFrac' or ($ClearSpeak_Fractions='EndFrac' and not( ($ClearSpeak_Fractions='Auto' or $ClearSpeak_Fractions='Ordinal' or $ClearSpeak_Fractions='EndFrac') and *[1][*[1][self::m:mn][not(contains(., '.')) and ($ClearSpeak_Fractions='Ordinal' or text()<20)]   and *[2][self::m:mn][not(contains(., '.')) and ($ClearSpeak_Fractions='Ordinal' or (2<= text() and text()<=10))] ] and *[2][*[1][self::m:mn][not(contains(., '.')) and ($ClearSpeak_Fractions='Ordinal' or text()<20)]   and *[2][self::m:mn][not(contains(., '.')) and ($ClearSpeak_Fractions='Ordinal' or (2<= text() and text()<=10))] ] ) )"
      then:
      - pause: short
      - t: "end fraction"
      - pause: short
```

The next part of the message (`caused by: replacement #1 of 5`) says the problem happens in the first replacement (the first "-").
The next line (`caused by: replacement #1 of 2`) says inside of that, the error inside of the first part of that
The final line says that in there, the problem is `Unknown 'replace' command (testx) with value`. So now you can correct that problem.
It is often easiest to read the error from the bottom up.


## Rust Developers
To be written...

`build.rs` and files in `src`

## Testing
Whether you are developing code or writing rules, writing and running the tests is very important. It is how you know what you wrote works and also how you know what you wrote didn't break something else.

The `tests` directory is similar to the `Rules` directory. If you are a translator, see the section above that describes what you should do.


## Files
MathCAT reads the following files for critical information:
* Rules
  * intent.yaml -- rules that infer author intent from MathML. These are used by various speech styles (in various languages) to avoid duplicating the inference process. They add an `intent` attribute to the MathML.
  * definitions.yaml -- these define various lists used by MathCAT for canonicalization (inferring proper structure) and also rule matching. E.g., `TrigFunctionNames` is a list of names of trig functions such as `tan` and `lim`.
  * prefs.yaml -- system defaults for various preferences that are settable. MathCAT will also look for this file in a platform-specific user location so that individual users can set the values.
    * Windows: `%AppData%\prefs.yaml`
    * Linux:  `$XDG_CONFIG_HOME` or `$HOME/.config`
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
* scalar types like integers, floats, and strings (can be inside of single or double quotes or left unquoted in some cases)
* arrays (used inline as `["a", "b", "c"]`)
* dictionaries/maps (used inline as `{key: value, foo: bar}`)

Comments begin with a `#` and extend to the end of the line. There are no block comments in YAML.

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
 - "∞": [t: "infinity"]                          # 0x221e
 - '∞':
    - t: infinity                                # 0x222e

# Here are a few options for a more complex definition that involves a test
# This compact form is (compact) JSON syntax for the value
- 0x003C: [test: {if:Verbosity!='terse', then: [t: is]}, t: "less than"]

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
#      - intent:
#          name: string  name of intent rule
#          children: children of the intent rule
#      - insert:
#          nodes:  xpath (evaluate to nodes)
#          replace: [replacements]  values that are inserted between all the nodes
#      - translate: xpath   allow speech of an expression in the middle of a rule (used by "WhereAmI" for navigation)
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
#      - spell:  string that is an xpath (usually a single letter to be pronounced as the letter, `"'a'"`)
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
| `IsNode(nodes, type)   | Returns true if all of the nodes are of the same type. Type can be one of:<br/>  "simple" -- a defined set of elements in ClearSpeak <br/> "leaf" -- one of the MathML leaf elements <br/> "2D" -- a 2D nodes such as `mfrac` or `mroot` <br/> "modified" -- the node has a script or something over/under it <br/> "scripts" -- the node as a subscript and/or superscript<br/> "common_fraction" -- integer numerator and denominator |
| ToOrdinal |  |
| ToCommonFraction | |
| IsBracketed(openChar, closeChar, requiresComma) | |
| BaseNode(node) | Returns the base (recursively) of a scripted node |
| IsInDefinition(node, name) | Returns true if node is a member of the list 'name' (defined in definitions.yaml) |
| IfThenElse(test, then-part, else-part) | Returns `then-part` if the test is true, otherwise `else-part`. All arguments are xpath |
| DistanceFromLeaf(node, left_side, treat_2d_elements_as_tokens) |  Returns distance from the current node to the leftmost/rightmost leaf (if a char, then = 0, if token, then 1). If the node is a bracketed expr with the indicated left/right chars. If `left_side` is `true`, traverse leftmost child to leaf. If `treat2D_elements_as_tokens` is `true`, 2D notations such as fractions are treated like leaves. |
| EdgeNode(node, "left"/"right", stopNodeName) | Returns the stopNode if at left/right edge of named ancestor node. "stopNodeName' can also be "2D'. The  original node is returned if match isn't found. Note: if stopNodeName=="math", then punctuation is taken into account since it isn't really part of the math
 |
| DEBUG(xpath) | Really helpful for debugging -- it will be added to debug output |

These are used by Nemeth Rules:

| function | meaning |
| ----- | ---- |
| NestingChars | Used by mfrac, msqrt, and mroot rules to repeat the chars the appropriate number of times |
| BrailleChars | Used by token elements to deal with the complicated rearrangement of various Nemeth indicators such as capitalization and font face |
