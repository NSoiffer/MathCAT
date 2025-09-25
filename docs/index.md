# MathCAT: Math Capable Assistive Technology
<img src="logo.png" style="position: relative; top: 16px; z-index: -1;">
is a library that supports conversion of MathML to:

* Speech strings with embedded speech engine commands
* Braille (Nemeth, UEB Technical, and eventually other braille math codes)
* Navigation of math (in multiple ways including overviews)

A goal of MathCAT is to be an easy to use library for screen readers and other assistive technology to use to produce high quality speech and/or braille from MathML. It is a follow-on project from MathPlayer (see below) and uses lessons learned from it to do to produce even higher quality speech, navigation, and braille. MathCAT takes advantage of some new ideas the [MathML Working Group](https://mathml-refresh.github.io/charter-drafts/math-2020.html) is developing to allow authors to express their intent when they use a notation. E.g., $(3, 6)$ could be a point in the plane or an open interval, or even a shorthand notation for the greatest common divisor. When that information is conveyed in the MathML, MathCAT will use it to generate more natural sounding speech.

Todo: incorporation of third party libraries to support a common subset of TeX math commands along with ASCIIMath.


# Documentation for different MathCAT Users

There are many different audiences for MathCAT and each audience has different interests/needs. Please see the following documentation for details based on your needs:
* AT users: [information about preferences you can set](users.md)
* AT developers/library users: [information about the API that MathCAT exposes](callers.md)
* Translators/Rule writers: [information about the files that need to be translated](helpers.md)
* MathCAT developers: information about MathCAT's design

# Some Technical Details
MathCAT is written in Rust and can be built to interface with many languages. To date there are interfaces for:
* [C/C++](https://github.com/NSoiffer/MathCATForC)
* [Python](https://github.com/NSoiffer/MathCATForPython) -- this is used by an [NVDA add-on](https://addons.nvda-project.org/addons/MathCAT.en.html). I hope to eventually get it incorporated into [Orca](https://help.gnome.org/users/orca/stable) which is written in Python.
* [Java](https://github.com/mwhapples/MathCAT4J) -- this is currently being used to experiment with MathCAT in [BrailleBlaster](https://www.brailleblaster.org/).
* [WebAssembly (Wasm, sort of JavaScript)](https://github.com/NSoiffer/MathCATDemo/) -- this is used for a web demo of MathCAT.

MathCAT uses a number of heuristics that try to repair poor MathML and put it in a recommended format. For example, TeX converters and WYSIWYG editors will take "1,234+1" and break the number "1,234" apart at the comma. MathCAT recognizes that and folds the number into a single `mn`. Other repairs are structural such as creating `mrow`s based on information from MathML's operator dictionary and adding invisible function application, multiplication, addition (mixed fractions), and separators (e.g, between the $i$ and $j$ in $a\_{ij}$) when it seems appropriate. This simplifies speech and Nemeth generation and may be useful to other apps. Currently the cleanup is not exposed in an API, but potentially it could be another service of MathCAT. In general, MathCAT is somewhat conservative in its repair. However, it likely will do the wrong thing in some cases, but the hope is it does the right thing much, much more frequently. Finding common mistakes of translators to MathML and patching up the poor MathML is an ongoing project.

## Current Status (updated 4/3/24)
MathCAT is under active development. Initial speech (English, Spanish, Indonesian, Vietnamese, Chinese(Traditional)), navigation, and braille (Nemeth, UEB, CMU, Vietnamese, German LaTeX) generation is complete and [NVDA add-on](https://addons.nvda-project.org/addons/MathCAT.en.html) now exists. It should be usable as a MathPlayer replacement for those using the English version or one of the supported translations. It is not as complete or polished in some ways as MathPlayer though. However, it supports both Nemeth and UEB technical braille generation. The Nemeth braille is substantially better than that provided by MathPlayer and other MathML → Nemeth translators. It also includes integration with navigation (uses dots 7 and 8 to indicate the navigation node) along with braille cursor routing during navigation. Because of the high quality braille output, [BrailleBlaster](https://www.brailleblaster.org/) uses MathCAT for braille generation from MathML.

A number of other AT are working to incorporate MathCAT into their products. Notable among these groups is Vispero/JAWS. To use MathCAT in JAWS 2024 ([What's New in JAWS 2024 Screen Reading Software (freedomscientific.com)](https://support.freedomscientific.com/downloads/jaws/JAWSWhatsNew)), enable it through the Early Adopter Program. Open the JAWS Menu with Insert + J, navigate to Options, select Early Adopter Program, check the MathCAT checkbox, and restart JAWS to use it. You can send Freedom Scientific feedback about their MathCat implementation by going back to the Early Adopter Program dialog and selecting the Send Feedback button, located after the MathCat checkbox. JAWS currently supports MathCat in English, with Nemeth Braille. UEB is on the way, so stay tuned. You can use all speech, navigation and Braille navigation command that MathCat provides from within the JAWS Math Viewer. MathCat Settings are available through the JAWS Settings Center.

[_Other companies_: if you have incorporated MathCAT into your product and would like to be mentioned here, please contact me by email or add an issue to update the documentation]

A demo to show off some of MathCAT's features and also as an aid for debugging was developed. [Visit the demo](https://nsoiffer.github.io/MathCATDemo/) and please report any bugs you find. This demo is _not_ how AT users will typically interact with MathCAT but does show features that AT can potentially expose to end users such as highlighting of the speech, navigation, and braille.

Timeline:

2022
* ✓ early January: prototype usage of preliminary MathML WG proposal for "intent"
* ✓ January: Distribute MathCAT to a small group of students and other users for feedback and bug reports
* ✓ February/March: Work on MathML → UEB translation
* ✓ April: Prosody implementation/compatibility with SAPI, One Core, eSpeak, and Eloquence voices
* late April/May: add more intent inference rules (ongoing)
* ✓ May: Release MathCAT as NVDA add-on
* ✓ June: C/C++ interface for MathCAT
* ✓ Late spring/summer: develop GUI interface for setting user preferences
* ✓ July - Oct: Add Chemistry-specific speech
* ✓ July/Aug/Sept: vacation 😎 and conference
* ✓ Nov/Dec: Work on at least one translation of MathCAT to another language (pushed back from late spring). Have Indonesian and Vietnamese translations.

2023
* Spring 2023: translation work
  * ✓ Create some tools to simplify generation of the Unicode files in different languages
  * Create some tools to help update other languages when the English version changes (adds new rules) [critical]
  * ✓ Add phrases so better starting points for translations can be generated
  * ✓ Work with translators and fix any problems they might turn up
  * Work with translators to hopefully add many languages (added Spanish translation)
* ✓ (mostly) Summer 2023: Vietnamese braille code 
* ✓August/Sept: Add CMU braille code (Spanish and Portuguese standard)

2024
* ✓ Jan: Automatic builds for the various repos
* Jan: Portuguese translation
* Winter-Spring: More translations (Swedish and Finnish speech and braille)
* Feb: Units and Currency
* Spring: add more inference/speech rules based on W3C's Math WG core list
* Spring: analyze books to better determine what should be in the Unicode short file (hopefully get someone to help with this)
* Spring: more translations
* ✓ (added) German LaTeX braille 
* ✓ (added) ASCIIMath (braille) 
* ✓ (added) Copy as options: LaTeX, ASCIIMath
* Spring/Summer/Fall: work on tools to help maintain translations
* Summer: explore adding ASCIIMath and LaTeX importers (there are Rust packages for these, but I haven't checked quality and completeness)
* July: vacation 😎 and ICCHP conference
* Fall: potentially work on UEB → MathML translation and explore UEB → Nemeth math translator

Longer term
* other braille input
* potentially work on 2D Nemeth generation along with other braille codes


These plans are very tentative and will likely change based on feedback from users and AT developers.
I also have commitments for working on the MathML spec, so that can also delay some of these dates.

## Why MathCAT?

MathCAT is a follow-on to MathPlayer. I developed MathPlayer's accessibility while at Design Science starting back in 2004 after I joined Design Science. At the time, MathPlayer was chiefly designed to be a C++ plugin to Internet Explorer (IE) that displayed MathML on web pages. For quite some time, it was the most complete MathML implementation available. The original work for display of math was done by Design Science's founder Paul Topping and their chief technology officer, the late Robert Miner. Eventually, for numerous reasons, IE withdrew the interface that MathPlayer used for display and did not implement a replacement as the world was moving towards using JavaScript in the browser and not allowing security threats posed by external code. This left MathPlayer as an accessibility-only library called by other programs (chiefly NVDA). MathPlayer was proprietary, but was given away for free.

In 2016, I left Design Science. In 2017, WIRIS bought Design Science. I volunteered to add bug fixes for free to MathPlayer and initially they were supportive of that. But when it came time to do a release, a number of the people around at the time of the buyout had left and the remaining team was not interested in supporting MathPlayer. That decision was not finalized until late 2020. In 2021, I started work on a replacement to MathPlayer. As a challenge, I decided to learn Rust and did the implementation in Rust. For those not familiar with Rust, it is a low level language that is type safe and memory safe, but not automatically garbage collected or reference counted. It is often touted as a safer replacement to C/C++.

Rust is quite efficient. On a Core I7-770K machine (higher end processor circa 2017), the moderate-size expression
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mrow>
    <msup>
      <mi>e</mi>
      <mrow>
        <mo>&#x2212;</mo>
        <mfrac>
          <mn>1</mn>
          <mn>2</mn>
        </mfrac>
        <msup>
          <mrow>
            <mrow>
              <mo>(</mo>
              <mrow>
                <mfrac>
                  <mrow>
                    <mi>x</mi>
                    <mo>&#x2212;</mo>
                    <mi>&#x03BC;</mi>
                  </mrow>
                  <mi>&#x03C3;</mi>
                </mfrac>
              </mrow>
              <mo>)</mo>
            </mrow>
          </mrow>
          <mn>2</mn>
        </msup>
      </mrow>
    </msup>
  </mrow>
</math>
takes about 4ms to generate the ClearSpeak string
"_e raised to the exponent, negative 1 half times; open paren; the fraction with numerator; x minus mu; and denominator sigma; close paren squared, end exponent_" along with the Nemeth braille string "⠑⠘⠤⠹⠂⠌⠆⠼⠈⠡⠷⠹⠭⠤⠨⠍⠌⠨⠎⠼⠾⠘⠘⠆".
This time is split approximately: 2ms to cleanup the MathML + 1ms for speech generation + 1ms for braille generation. This includes time to make sure all the rule files are up to date, which turns out is quite expensive. A preference can be set to turn the checks off (the file checks are mainly useful for debugging). With the check turned off, the time drops to 2.3ms.
<details>
<summary>Click to see the MathML for this expression</summary>
<pre>
&lt;math&gt;
  &lt;mrow&gt;
    &lt;msup&gt;
      &lt;mi&gt;e&lt;/mi&gt;
      &lt;mrow&gt;
        &lt;mo&gt;&#x2212;&lt;/mo&gt;
        &lt;mfrac&gt;
          &lt;mn&gt;1&lt;/mn&gt;
          &lt;mn&gt;2&lt;/mn&gt;
        &lt;/mfrac&gt;
        &lt;msup&gt;
          &lt;mrow&gt;
            &lt;mrow&gt;
              &lt;mo&gt;(&lt;/mo&gt;
              &lt;mrow&gt;
                &lt;mfrac&gt;
                  &lt;mrow&gt;
                    &lt;mi&gt;x&lt;/mi&gt;
                    &lt;mo&gt;&#x2212;&lt;/mo&gt;
                    &lt;mi&gt;&#x03BC;&lt;/mi&gt;
                  &lt;/mrow&gt;
                  &lt;mi&gt;&#x03C3;&lt;/mi&gt;
                &lt;/mfrac&gt;
              &lt;/mrow&gt;
              &lt;mo&gt;)&lt;/mo&gt;
            &lt;/mrow&gt;
          &lt;/mrow&gt;
          &lt;mn&gt;2&lt;/mn&gt;
        &lt;/msup&gt;
      &lt;/mrow&gt;
    &lt;/msup&gt;
  &lt;/mrow&gt;
&lt;/math&gt;
</pre>
</details>

MathCAT uses external rules to generate speech and braille.
These take about 40ms to load; this load only happens the first time the rules are used, or if the speech style, language, or other external preference is changed. An additional 50ms are required to load the full Unicode files for speech and braille,
but studies have shown that a vast majority of English K-14 math material uses a surprisingly few number of characters.
Using open source math books, the initial load should cover at least 99.99% of the characters used in expressions encountered in English K-14 math textbooks.

The library is about ~3mb in size.

If you are working on an in-browser solution (i.e, you are using JavaScript or some other browser-based language), MathCAT is probably not the best tool for you (although I will probably factor the [MathCATDemo](https://github.com/NSoiffer/MathCATDemo/) into a Javascript interface which the demo is built on top of). Instead, take a look at [Speech rule engine](https://github.com/zorkow/speech-rule-engine) (SRE) by Volker Sorge. It is written in TypeScript and will likely meet your needs for an in-browser solution unless braille is important; MathCAT supports multiple braille codes and at least for Nemeth Code, is higher quality.

# Acknowledgements
Several people helped out in various ways with the project. I am very grateful for all their help!

* David Carlisle -- provided invaluable help figuring out some xpath matches
* Susan Jolly -- provided lots of patient guidance on Nemeth and UEB generation along with feedback on what is right and wrong. On top of that, she also guided me as I tried to work out chemistry heuristics.
* Elaine A. Moore -- helped me to figure out what should and should not be said for chemistry, along with what makes sense as chemistry and what doesn't.
* Richard Orme -- did all the work for the MathCAT NVDA settings dialog.
* Sam Dooley, Murray Sargent, and Volker Sorge -- provided tables of Nemeth translations of characters and Nemeth tests

Translators:

* Chinese (Traditional) -- Hon-Jang Yang
* Finnish -- Sami Määttä, Accessibility Library Celia, and Essi Viippola, freelancer
* German --Nazli Andjic, Robert Graf and Paul Libbrecht (IU International University of Applied Sciences)
* Indonesian -- Dr. Pinta Deniyanti Sampoerno, M.Si; Dr. Meiliasari, S.Pd., M.Sc; and Ari Hendarno, S.Pd., M.kom
* Norwegian -- Marthe Gjelstad, National Library of Norway, Kvile
* Spanish -- Noelia Ruiz Martínez (also help with NVDA addon development) and María Allo Roldán
* Swedish -- Tim Arborealis Lötberg, Swedish Agency for Accessible Media (MTM) and Anders Eklund, SPSM
* Vietnamese -- Dang Hoai Phúc and Trang Pham
* Others??? -- please volunteer so I can list you here...

The initial translation of many braille characters for braille codes developed in 2024 and beyond was greatly helped by a spreadsheet given to me by Georgious Kouroupetroglou and is the work of a larger team. For more details, see:

* [MathBrailleCodes Repository](https://access.uoa.gr/mathbraille/index.php/en/), Speech and Accessibility Lab, National and Kapodistrian University of Athens, Greece: P. Riga, T. Antonakopoulou, D. Kouvaras, S. Lentas and G. Kouroupetroglou (2021) “[The BrailleMathCodes Repository](https://access.uoa.gr/mathbraille/index.php/en/)”, Proceedings of the 4th International Workshop on “[Digitization and e-Inclusion in Mathematics and Science 2021](https://workshop.sciaccess.net/deims2021/DEIMS2021_Proceedings.zip)” DEIMS2021, February 18-19, 2021, Tokyo, pp. 105-114. 

Thanks to everyone who volunteered!

# About me

I've been working on math accessibility since 2002. At the time, I worked on Mathematica's WYSIWYG math editor and other UI features. Prof. John Gardner, who had lost his sight 15 years earlier, asked whether I could make the Mathematica frontend accessible. I maybe got 80% of the way there, but the company wasn't interested in pursuing this and ultimately I left the company and the company removed the code. That was the start of my accessibility journey: one step forward, one step back, and then forward again because allowing _everyone_ to have a chance to find the joy of math and science has given purpose to my life.

I then joined Design Science, Inc (DSI) which had an interest in making math accessible. At the time, DSI had recently developed MathPlayer, a plugin for IE6 that displayed MathML. I worked on adding features to that and with the company's support, applied for and received an NSF grant to make MathPlayer accessible. That work was quite successful and in subsequent years I continued to add features to it. However, for security reasons, Internet Explorer removed the interface that MathPlayer depended upon. It's tempting to say that is what doomed IE... After that, MathPlayer became an accessibility-only NVDA add-on. Further work through an IES grant with ETS refined MathPlayer's capabilities; valuable insight was gained via user-studies funded by the grant.

For more information about what happened to MathPlayer and how MathCAT came to be, see the [Why MathCAT?](#why-mathcat) section.

All along, I've been pushing to make math work on the web and make it accessible. While at Wolfram Research, I helped get the W3C MathML effort started and have been involved with the working group ever since. I currently chair the W3C Math Working Group. I've been a member on several other committees over the years pushing strongly to make sure they incorporated math accessibility into their standards. Some of the these groups include NIMAS, EPUB, and PDF/UA.

I'm very honored that in 2023, the National Federation of the Blind gave me the <span>$</span>25,000 Jacob Bolotin award. I donated <span>$</span>15,000 of that to the _open collective_ to improve MathML support in browsers. [Click this link for how you can help improve MathML support in browsers](https://opencollective.com/mathml-core-support).