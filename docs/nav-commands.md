# MathCAT Navigation Commands and their Key Bindings

There are two modes, each of which can set:
* Speech after move mode: whether the expression is <i>read</i> or <i>described</i>
(a summary/outline) after each move
* Navigation mode: navigate by subexpression, small piece, or
character (shift+down/up arrow will cycle to smaller/larger modes and then wrap
around).

See the [Navigation Modes section](#navigation-modes) after the table for explanations of these modes.

Note: while navigating an expression, "control+c" copies the math content of the current node in NVDA (as MathML, LaTeX, ASCIIMath, or Speech).



## Navigation Commands Table
<table class=MsoTableGrid border=1 cellspacing=0 cellpadding=0
 style='border-collapse:collapse;border:none'>
 <thead>
  <tr style='page-break-inside:avoid'>
   <td valign=top style='border:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
   <a
   name="Table_of_Keybindings"><b>Key</b></a>
   </td>
   <td valign=top style='border:solid 1.0pt;border-left:none;
   padding:0in 5.4pt 0in 5.4pt'>
   <b>Unmodified</b>
   </td>
   <td valign=top style='border:solid 1.0pt;border-left:none;
   padding:0in 5.4pt 0in 5.4pt'>
   <b>+ Ctrl</b>
   </td>
   <td valign=top style='border:solid 1.0pt;border-left:none;
   padding:0in 5.4pt 0in 5.4pt'>
   <b>+ Shift</b>
   </td>
   <td valign=top style='border:solid 1.0pt;border-left:none;
   padding:0in 5.4pt 0in 5.4pt'>
   <b>+Cntrl+Shift</b>
   </td>
  </tr>
 </thead>
 <tr style='page-break-inside:avoid'>
  <td valign=top style='border:solid 1.0pt;border-top:none;
  padding:0in 5.4pt 0in 5.4pt'>
  <b>Left</b>
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Move to
  previous
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  In table: move to previous cell<br/>
  In columnar math: move to previous digit<br/>
  Note: Ctrl+Alt+Left can also be used
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Read previous
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Describe
  previous
  </td>
 </tr>
 <tr style='page-break-inside:avoid'>
  <td valign=top style='border:solid 1.0pt;border-top:none;
  padding:0in 5.4pt 0in 5.4pt'>
  <b>Right</b>
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Move to next 
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  In table: move to next cell<br/>
  In columnar math: move to next digit<br/>
  Note: Ctrl+Alt+Right can also be used
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Read next
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Describe next
  </td>
 </tr>
 <tr style='page-break-inside:avoid'>
  <td valign=top style='border:solid 1.0pt;border-top:none;
  padding:0in 5.4pt 0in 5.4pt'>
  <b>Up</b>
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Zoom out
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  In table: move to cell above<br/>
  In columnar math: move to digit above<br/>
  Note: Ctrl+Alt+Up can also be used
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Change
  Navigation Mode (Enhanced/Simple/Character) to larger
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Zoom out all
  the way
  </td>
 </tr>
 <tr style='page-break-inside:avoid'>
  <td valign=top style='border:solid 1.0pt;border-top:none;
  padding:0in 5.4pt 0in 5.4pt'>
  <b>Down</b>
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Zoom in 
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  In table: move to cell below<br/>
  In columnar math: move to digit below<br/>
  Note: Ctrl+Alt+Down can also be used
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Change
  Navigation Mode (Enhanced/Simple/Character) to smaller
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Zoom in all
  the way
  </td>
 </tr>
 <tr style='page-break-inside:avoid'>
  <td valign=top style='border:solid 1.0pt;border-top:none;
  padding:0in 5.4pt 0in 5.4pt'>
  <b>Enter</b>
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Where am I
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Global Where
  am I
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  &nbsp;
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  &nbsp;
  </td>
 </tr>
 <tr style='page-break-inside:avoid'>
  <td valign=top style='border:solid 1.0pt;border-top:none;
  padding:0in 5.4pt 0in 5.4pt'>
  <b>Numbers<br>
  1-10 (0 is 10)</b>
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Jump to Place
  Marker
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Set
  placemarker
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Read
  Placemarker
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Describe
  Placemarker
  </td>
 </tr>
 <tr style='page-break-inside:avoid'>
  <td valign=top style='border:solid 1.0pt;border-top:none;
  padding:0in 5.4pt 0in 5.4pt'>
  <b>Space</b>
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Read current
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Read Current
  cell
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Toggle
  “speech mode” to read or describe
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Describe
  current
  </td>
 </tr>
 <tr style='page-break-inside:avoid'>
  <td valign=top style='border:solid 1.0pt;border-top:none;
  padding:0in 5.4pt 0in 5.4pt'>
  <b>Home</b>
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Move to start
  of expression
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Move to start
  of line
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Move to start
  of column
  Move to digit
  at top
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  NYI: Read from start of expression</span>
  </td>
 </tr>
 <tr style='page-break-inside:avoid'>
  <td valign=top style='border:solid 1.0pt;border-top:none;
  padding:0in 5.4pt 0in 5.4pt'>
  <b>End</b>
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Move to end
  of expression
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Move to end
  of line
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Move to end
  of column
  Move to digit
  at bottom
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  NYI: Read to end of expression</span>
  </td>
 </tr>
 <tr style='page-break-inside:avoid'>
  <td valign=top style='border:solid 1.0pt;border-top:none;
  padding:0in 5.4pt 0in 5.4pt'>
  <b>Backspace</b>
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  Move back to
  last position
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  &nbsp;
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  &nbsp;
  </td>
  <td valign=top style='border-top:none;border-left:none;border-bottom:solid 1.0pt;
  border-right:solid 1.0pt;padding:0in 5.4pt 0in 5.4pt'>
  <p style='margin-bottom:0in;line-height:normal;page-break-after:
  avoid'>&nbsp; </p>
  </td>
 </tr>
</table>

<span style='color:gray'>NYI</span> = Not Yet Implemented

## Navigation Modes

MathCAT supports three different navigation modes: enhanced, simple, and character. The first two modes of navigation follow the semantics of what was read for the entire expression except if LiteralSpeech is selected. For example $|x+y| > 0$ will not read the vertical lines that are used for the absolute value notation, but instead will say "absolute value". Zooming in will move directly saying "x plus y". In contrast, character mode will read this as "vertical line", "x", "plus", "y" "vertical line", "is greater than", "zero" as you move through the expression.

* _Enhanced mode_: navigation is by mathematically meaningful pieces (operators, delimiters, and operands)
* _Simple mode_: this moves by words except when you get to a
    2D notation (fractions, roots, ...), then it speaks the entire notation.
    Zooming in lets you explore the 2D notation in the same mode. Zooming out or
    moving out of the 2D notation brings you back to the outer/higher level of
    navigation.
* _Character mode_: this is actually two useful modes --
    word mode and character mode (zoom in to get &quot;real&quot; character mode).
    &nbsp;Moves by words/characters. &nbsp;This differs for numbers of more than
    one digit and function names such as &quot;sin&quot; that are multiple
    characters. Otherwise, word and character navigation is the same. Both will
    automatically zoom into fractions, etc.

## Typical Use

Typically, you will start at the first term of an expression and move right as needed.
You might move up and down levels if needed. This done with the arrow keys.
`alt+ctrl+arrow` is used to move around tabular entries.

<i>Backspace</i> will take you back to where you were, which
is not always the same as moving to the left. For example, if right arrow moved
you out of a fraction, backspace will take you back to where you were in the
denominator and left arrow will land on the entire fraction.

You will likely find one mode of navigation the most natural for you most of the time.
This can be set in the MathCAT settings.
However, at any time during navigation, you can switch the navigation modes using `shift+up/down arrow`.
This is useful because each mode of navigation has its strengths and weaknesses.

## Acknowledgements
A version of this document was produced as part of the ClearSpeak project.
ClearSpeak was supported by the Institute of Education Sciences, U.S. Department of Education, through Grant R324A110355 to the Educational Testing Service. 
