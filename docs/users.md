---
layout: default
lang: en
ref: users
title: User guide for MathCAT
---
# User guide for MathCAT

MathCAT is a tool used together with a screen reader. Through MathCAT, you can have mathematical expressions read aloud. MatCAT can also make mathematics accessible through a braille display.

In screen readers NVDA and JAWS, MathCAT comes built-in. This means you don’t have to install it as an addon.

## Get started with MathCAT

Use your screen reader on a web page or in an e-book as usual. When you get to a mathematical expression, the screen reader will read it out automatically. If you want to investigate the expression more closely you can activate the navigation mode by pressing Space. In NVDA, the NVDA-key+Alt+M can also be used. If you want to leave the navigation mode, press Escape.

### The most common keyboard shortcuts are these

- Move left, right, upp or down inside a mathematical expression, use the arrow keys.
- Inside a table, move between cells by using CTRL+arrow keys.
- Move to the beginning of an expression by pressing Home. Move to the end of an expression by pressing End.
- To hear your current position, press Space.
- To change navigation mode, press Shift+up/down. The navigation modes are described here: Navigation

When you navigate inside an expression you can use CTRL+C to copy the MathML code for the current node of the expression.

There are many more possibilities when you navigate inside mathematical expressions. All features are described here: [All navigation commands](#all-navigation-commands)

## Adapt MathCAT to your needs

There are many possibilities to adjust MathCAT to fit your needs. You can find the settings by pressing the NVDA-key+N, then Settings, then MathCAT settings. Below Categories there are three options: Speech, Navigation and Braille.

### Speech

You can choose between the following settings for speech. Below each setting you can find a list of all options with a short description. The default setting, the one that is active if you don’t make a choice, is written inside square brackets.

- Disability:
  - \[Blindness.\] The readout is unambiguous.
  - Low vision. The readout is shorter.
  - Learning disability. The readout is shorter.
- Language: (the default setting is the same as the language the screen reader is set to)
  - \[English (en)\]
  - German (de)
  - Greek (el)
  - Spanish (es)
  - Finnish (fi)
  - French (fr)
  - Hungarian (hu)
  - Indonesian (id)
  - Norwegian bokmål (nb)
  - Polish (pl)
  - Brazilian Portugese (pt)
  - Russian (ru)
  - Swedish (sv)
  - Vietnamese (vi)
  - Chinese, traditional (zh-tw)
- Speech style:
  - \[ClearSpeak.\] Expressions are spoken approximately like a teacher would have said them in a classroom.
  - SimpleSpeak. Expressions are spoken more concicely. The reading is sometimes ambiguous.
- Verbosity:
  - Terse. “Extra” words such as “the” and “of” in “the square root of x” have been removed.
  - \[Medium.\] A compromise between Terse and Verbose.
  - Verbose. Everything is read out. The readout is unambiguous.
- MathRate:
  - \[100\], can be adjusted between 1 and 1000. Changes how fast math is spoken compared with the screen reader’s velocity. The number means percent. 100 corresponds to the same speed, less means slower and more means faster.
- Pause factor:
  - \[1\], can be adjusted between 0 and 10. Changes how long the pauses are when math is read out.
- Math Sound:
  - \[None.\]
  - Beep. A beep is played before and after every math expression.
- Chemistry:
  - \[Read out.\] Chemical formulae are read out, e.g. “H two O” for $H_2O$.
  - Off. “H sub 2 O” for $H_2O$.

### Navigation

Through MathCAT you can investigate an expression in detail by navigating inside it, that is, moving around and reading it piece-by-piece. In the settings for Navigation you can choose how to do this, and how detailed information you want.

Below each setting you can find a list of all options with a short description. The default setting, the one that is active if you don’t make a choice, is written inside square brackets.

- Navigation mode used when you enter an expression:
  - \[Enhanced mode.\] Move between mathematically meaningful parts of an expression (e.g. numerator, denominator, exponents, expressions within parentheses).
  - Simple mode. Move between words, except when you get to a defined expression, e.g. a square root. Then the entire expression is read out.
  - Character mode. Move between words or numbers. Zoom in to read each letter or digit separately.

If you want to change navigation mode while navigating an expression you can use Shift+Up to go to Enhanced from Simple (or from Character mode to Simple). If you use Shift+Down you go from Enhanced to Simple (or from Simple to Character mode). I.e., upwards means overview while downwards means more detailed.

There is also a tick box which you can select for the navigation mode to reset every time you enter an expression. By default it is not selected.

- Speech after move mode:
  - \[Read.\] Reads out the part of the expression where you are.
  - Describe. Gives an overview of the selected expression.

There is also a tick box which you can select for the speech after move mode to be reset every time you enter an expression. By default it is selected.

- Zoom out automatically when part of an expression (e.g. a root) has been read out.
  - \[On.\] (box selected)
  - Off. (box not selected)
- Verbosity for navigation:
  - Terse. “Extra” words such as “the” and “of” in “the square root of x” have been removed.
  - \[Medium.\] A compromise between Terse and Verbose.
  - Verbose. Everything is read out. The readout is unambiguous.

### Braille

You can choose between the following settings for braille. Below each setting you can find a list of all options. The default setting, the one that is active if you don’t make a choice, is written inside square brackets.

- Braille notation for mathematics to be shown on the braille display:
  - CMU.
  - \[Nemeth.\]
  - Swedish.
  - UEB.
  - Vietnam.
- Dot 7 & 8 mark the following position in the navigation mode:
  - Off.
  - The first characters.
  - \[End points.\]
  - All.

## All navigation commands

The table shows all the commands you can use for navigating inside a mathematical expression. In the first column is the name of a key. In the second column is described what happens if you press the key. In the third column is described what happens if you press Control and then the key. In the fourth column is described what happens if you press Shift and then the key. In the fifth column is described what happens if you press Control and Shift and the key.

Note: tabular math means mathematical content in a table structure such as a matrix or a system of equations. They can be navigated like a table.

| Key | Unmodified | \+ Ctrl | \+ Shift | +Ctrl+Shift |
| --- | --- | --- | --- | --- |
| Left | Move to previous | In table: move to previous cell In tabular math: move to previous element  <br>Note: Ctrl+Alt+Left can also be used | Read previous | Describe previous |
| Right | Move to next | In table: move to next cell  <br>In tabular math: move to next element  <br>Note: Ctrl+Alt+Right can also be used | Read next | Describe next |
| Up  | Zoom out | In table: move to cell above  <br>In tabular math: move to element above  <br>Note: Ctrl+Alt+Up can also be used | Change Navigation Mode (Enhanced/Simple/Character) to larger | Zoom out all the way |
| Down | Zoom in | In table: move to cell below  <br>In tabular math: move to element below  <br>Note: Ctrl+Alt+Down can also be used | Change Navigation Mode (Enhanced/Simple/Character) to smaller | Zoom in all the way |
| Enter | Where am I | Global Where am I | &nbsp; | &nbsp; |
| Numbers  <br>1-10 (0 is 10) | Jump to Place Marker | Set placemarker | Read Placemarker | Describe Placemarker |
| Space | Read current | Read Current cell | Toggle “speech mode” to read or describe | Describe current |
| Home | Move to start of expression | Move to start of line | In tabular math: Move to start of column<br><br>In column: Move to element at top |     |
| End | Move to end of expression | Move to end of line | In tabular math: Move to end of column<br><br>In column: Move to element at bottom |     |
| Backspace | Move back to last position |     |     |     |

## Do you have feedback on MathCAT?

MathCAT is under active development, and we want your feedback. Do you have a feature request, or have you found any bugs? Please create an issue on [MathCAT's GitHub page](https://github.com/daisy/MathCAT/issues). You are also welcome to get in touch by signing up to [the mailing list of the DAISY MathCAT working group](https://www.surveymonkey.com/r/HXTLXGN).

## How to install MathCAT as an addon in NVDA

To use MathCAT as an add-on, do the following:

1. Open the NVDA menu.
2. Choose Tools and then Add-on store.
3. In the add-on store, choose Available add-ons. 

   The channel settings have the following options:
    - Stable: Will only search for stable add-ons
    - All: Will search for all add-ons
    - Beta: Will search for add-ons that are in beta
    - Dev: Will search for add-ons in development

    Choose option "All".

4. Search for MathCAT and install.

5. In the settings menu, you can control how add-ons are updated:

    - Open the NVDA menu
    - Choose settings and Add-on Store

      For Automatic updates, you can choose between:
        - Notify
        - Update Automatically
        - Deactivated

      For Default update channel, you have the following options:

        - Same: If automatic updates is set to “Update Automatically” and your add-on is a beta, it will update to the first available beta.
        -  Any: It will update to any version of the add-on.
        - Do not update
        - Stable: It will only update to stable versions.
        - Beta or dev: It will update to beta or dev versions.
        - Beta: It will update to beta versions.
        - Dev: It will update to dev versions.
