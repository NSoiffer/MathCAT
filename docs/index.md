# MathCAT: Math Capable Assistive Technology
<img src="logo-inline.png" style="height: 5.25em; vertical-align: -0.13em; margin-right: -0.25em;" alt="Logo for MathCAT. a brown cat sits upright. Its tail swoops down to form the capital letter C in the word MathCAT."> is a library that supports conversion of MathML to:

* Speech strings with embedded speech engine commands
* Braille (Nemeth, UEB Technical, and eventually other braille math codes)
* Navigation of math (in multiple ways including overviews)

A goal of MathCAT is to be an easy-to-use library for screen readers and other assistive technology to use to produce high quality speech and/or braille from MathML. It is a follow-on project from MathPlayer (see below) and uses lessons learned from it to produce even higher quality speech, navigation, and braille. MathCAT takes advantage of some new ideas that the [MathML Working Group](https://mathml-refresh.github.io/charter-drafts/math-2020.html) is developing to allow authors to express their intent when they use a notation. E.g., $(3, 6)$ could be a point in the plane or an open interval, or even a shorthand notation for the greatest common divisor. When that information is conveyed in the MathML, MathCAT will use it to generate more natural sounding speech.

To do: incorporation of third party libraries to support a common subset of TeX math commands along with ASCIIMath.


# Documentation for different MathCAT Users

Watch the short explainer video below for an introduction to MathCAT.

<div style="padding:56.25% 0 0 0;position:relative;"><iframe src="https://player.vimeo.com/video/1228948748?badge=0&amp;autopause=0&amp;player_id=0&amp;app_id=58479" frameborder="0" allow="autoplay; fullscreen; picture-in-picture; clipboard-write; encrypted-media; web-share" referrerpolicy="strict-origin-when-cross-origin" style="position:absolute;top:0;left:0;width:100%;height:100%;" title="Introduction to MathCAT"></iframe></div>

There are many different audiences for MathCAT and each audience has different interests/needs. Please see the following documentation for details based on your needs:
* AT users: [information about preferences you can set](users.md)
* AT developers/library users: [information about the API that MathCAT exposes](callers.md)
* Translators/Rule writers: [information about the files that need to be translated](helpers.md)
* MathCAT developers: [information about development workflow and testing](developers.md)

# Some Technical Details
MathCAT is written in Rust and can be built to interface with many languages. To date there are interfaces for:
* [C/C++](https://github.com/NSoiffer/MathCATForC)
* [Python](https://github.com/NSoiffer/MathCATForPython) -- this is used by an [NVDA add-on](https://addons.nvda-project.org/addons/MathCAT.en.html) and the Linux screen reader [Orca](https://help.gnome.org/users/orca/stable), both of which are written in Python.
* [Java](https://github.com/mwhapples/MathCAT4J) -- this is currently being used to experiment with MathCAT in [BrailleBlaster](https://www.brailleblaster.org/).
* [WebAssembly (Wasm, sort of JavaScript)](https://github.com/NSoiffer/MathCATDemo/) -- this is used for a web demo of MathCAT.

MathCAT uses a number of heuristics that try to repair poor MathML and put it in a recommended format. For example, TeX converters and WYSIWYG editors will take "1,234+1" and break the number "1,234" apart at the comma. MathCAT recognizes that and folds the number into a single `mn`. Other repairs are structural such as creating `mrow`s based on information from MathML's operator dictionary and adding invisible function application, multiplication, addition (mixed fractions), and separators (e.g., between the $i$ and $j$ in $a\_{ij}$) when it seems appropriate. This simplifies speech and Nemeth generation and may be useful to other apps. Currently the cleanup is not exposed in an API, but potentially it could be another service of MathCAT. In general, MathCAT is somewhat conservative in its repair. However, it likely will do the wrong thing in some cases, but the hope is it does the right thing much, much more frequently. Finding common mistakes of translators to MathML and patching up the poor MathML is an ongoing project.

## Current Status (updated 3/27/26)

MathCAT is under active development. DAISY is actively participating in the development effort, and contributions are welcome. MathCAT is open source, and the [GitHub repository is available here](https://github.com/daisy/MathCAT). [NVDA-specific problems with the MathCAT add-on can be reported here](https://github.com/daisy/MathCATForPython/issues).

* MathCAT currently supports speech and navigation for
English, German, Spanish, Finnish, Indonesian, Norwegian, Russian, Swedish, Vietnamese, and Chinese (Traditional).
* MathCAT's braille support includes Nemeth, UEB, CMU, Vietnamese, German/Austrian LaTeX and ASCIIMath.
* An [NVDA add-on](https://addons.nvda-project.org/addons/MathCAT.en.html) exists. It should be usable as a MathPlayer replacement for those using the English version or one of the supported translations. As of NVDA 2026.1, MathCAT is built into NVDA and no download is needed.

MathCAT's Nemeth braille is substantially better than that provided by MathPlayer and other MathML → Nemeth translators. It also includes integration with navigation (uses dots 7 and 8 to indicate the navigation node) along with braille cursor routing during navigation. Because of the high quality braille output, [BrailleBlaster](https://www.brailleblaster.org/) uses MathCAT for Nemeth and UEB braille generation from MathML.

A number of other assistive technologies have incorporated MathCAT into their products. Notable among these is Vispero/JAWS. JAWS currently supports MathCAT's speech in English and Spanish; braille support includes Nemeth and UEB. Other languages and braille codes are on the way, so stay tuned. You can use all speech, navigation, and braille navigation commands that MathCAT provides from within the JAWS Math Viewer. MathCAT settings are available through the JAWS Settings Center.

Other assistive technology that use MathCAT:

* The Linux Orca screen reader incorporates MathCAT for speech, navigation, and braille.
* Dolphin EasyReader uses MathCAT.
* Kurzweil 3000 incorporates MathCAT’s speech and highlights the matching subexpression in dual colors as it is spoken, providing clear visual tracking for sighted users.
* Microsoft Narrator has announced they will be using MathCAT in an upcoming version.

[_Other companies_: if you have incorporated MathCAT into your product and would like to be mentioned here, please contact me by email or add an issue to update the documentation]

A demo to show off some of MathCAT's features and also as an aid for debugging was developed. [Visit the demo](https://nsoiffer.github.io/MathCATDemo/) and please report any bugs you find. This demo is _not_ how AT users will typically interact with MathCAT but does show features that AT can potentially expose to end users such as highlighting of the speech, navigation, and braille.

Future work includes:

* More languages. If you would like to translate to a language that is not currently supported, please create an issue in the [MathCAT GitHub repo](https://github.com/daisy/MathCAT/issues).
* More braille language support. To add support for another braille code, I need three things:
  * a spec for the language
  * access to a braille expert in that language who can answer questions that I have about the spec
  * someone who is willing to copy at least 200 examples from the spec into a form that is used for tests (MathML, Unicode braille string). That might take 30 or more hours.

  If you can bring together the required elements to help out, please create an issue in the [MathCAT GitHub repo](https://github.com/daisy/MathCAT/issues).
* Conversion _from_ braille _to_ MathML
* Work on 2D versions of the braille codes for use on multiline refreshable braille displays such as the Monarch and Canute 360.


## Why MathCAT?

MathCAT is a follow-on to MathPlayer. I developed MathPlayer's accessibility while at Design Science starting in 2004, shortly after I joined the company. At the time, MathPlayer was chiefly designed to be a C++ plugin to Internet Explorer (IE) that displayed MathML on web pages. For quite some time, it was the most complete MathML implementation available. The original work for display of math was done by Design Science's founder Paul Topping and its chief technology officer, the late Robert Miner. Eventually, for numerous reasons, IE withdrew the interface that MathPlayer used for display and did not implement a replacement as the world was moving towards using JavaScript in the browser and not allowing security threats posed by external code. This left MathPlayer as an accessibility-only library called by other programs (chiefly NVDA). MathPlayer was proprietary, but was given away for free.

In early 2017, I left Design Science. Later in the year, WIRIS bought Design Science. I volunteered to add bug fixes for free to MathPlayer and initially they were supportive of that. But when it came time to do a release, a number of the people around at the time of the buyout had left and the remaining team was not interested in supporting MathPlayer. That decision was not finalized until late 2020. In 2021, I started work on a replacement for MathPlayer. As a challenge, I decided to learn Rust and did the implementation in Rust. For those not familiar with Rust, it is a low-level language that is type safe and memory safe, but not automatically garbage collected or reference counted. It is often touted as a safer replacement for C/C++.

Rust is quite efficient. On a Core i7-770K machine (higher end processor circa 2017), the moderate-size expression
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
This time is split approximately: 2ms to clean up the MathML + 1ms for speech generation + 1ms for braille generation. This includes time to make sure all the rule files are up to date, which turns out to be quite expensive. A preference can be set to turn the checks off (the file checks are mainly useful for debugging). With the check turned off, the time drops to 2.3ms.
On a higher end (2025) Intel Core Ultra 9 285, the (single processor) time for generating speech + braille is ~1ms.

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
These take about 40ms to load; this load only happens the first time the rules are used, or if the speech style, language, or other external preference is changed. An additional 50ms is required to load the full Unicode files for speech and braille, but studies have shown that a vast majority of English K-14 math material uses a surprisingly small number of characters.
Using open source math books, the initial load should cover at least 99.99% of the characters used in expressions encountered in English K-14 math textbooks.

The library is ~3 MB in size.

If you are working on an in-browser solution (i.e., you are using JavaScript or some other browser-based language), MathCAT is probably not the best tool for you (although I will probably factor the [MathCATDemo](https://github.com/NSoiffer/MathCATDemo/) into a JavaScript interface on which the demo is built). Instead, take a look at [Speech rule engine](https://github.com/zorkow/speech-rule-engine) (SRE) by Volker Sorge. It is written in TypeScript and will likely meet your needs for an in-browser solution unless braille is important; MathCAT supports multiple braille codes and at least for Nemeth Code, is higher quality.

# Acknowledgements

Several people helped out in various ways with the project. I am very grateful for all their help!

* David Carlisle -- provided invaluable help figuring out some XPath matches
* Susan Jolly -- provided lots of patient guidance on Nemeth and UEB generation along with feedback on what is right and wrong. On top of that, she also guided me as I tried to work out chemistry heuristics.
* Elaine A. Moore -- helped me to figure out what should and should not be said for chemistry, along with what makes sense as chemistry and what doesn't.
* Richard Orme -- did all the work for the MathCAT NVDA settings dialog.
* Sam Dooley, Murray Sargent, and Volker Sorge -- provided tables of Nemeth translations of characters and Nemeth tests
* Moritz Groß ([Math4VIP](https://www.math4vip.de/), Marburg University) -- worked on various parts of the Rust code base, and built the Python tool for tracking localization progress.

Translators:

* Chinese (Traditional) -- Hon-Jang Yang
* Finnish -- Sami Määttä (Accessibility Library Celia), and Essi Viippola (freelancer)
* French -- Olivier Thiffault (Technologies Humanware)
* German -- Nazli Andjic, Robert Graf and Paul Libbrecht (IU International University of Applied Sciences), and Moritz Groß (Math4VIP, Marburg University)
* Greek -- Theodora Antonakopoulou and Paraskevi Riga (National and Kapodistrian University of Athens)
* Hungarian -- Attila Hammer (IT Foundation for the Visually Impaired)
* Indonesian -- Dr. Pinta Deniyanti Sampoerno, M.Si; Dr. Meiliasari, S.Pd., M.Sc; and Ari Hendarno, S.Pd., M.Kom.
* Norwegian -- Marthe Gjelstad (National Library of Norway) and Kvile
* Polish -- Michał Dziwisz (Fundacja Instytut Rozwoju Regionalnego)
* Russian (both speech and braille) -- Danil Kostenkov
* Spanish -- Noelia Ruiz Martínez (also helped with NVDA add-on development) and María Allo Roldán
* Swedish -- Tim Arborealis Lötberg (MTM) and Anders Eklund (SPSM)
* Vietnamese -- Dang Hoai Phúc and Trang Pham
* Others??? -- please volunteer so I can list you here...

The initial translation of many braille characters for braille codes developed in 2024 and beyond was greatly helped by a spreadsheet given to me by Georgios Kouroupetroglou and is the work of a larger team. For more details, see:

* [MathBrailleCodes Repository](https://access.uoa.gr/mathbraille/index.php/en/), Speech and Accessibility Lab, National and Kapodistrian University of Athens, Greece: P. Riga, T. Antonakopoulou, D. Kouvaras, S. Lentas and G. Kouroupetroglou (2021) “[The BrailleMathCodes Repository](https://access.uoa.gr/mathbraille/index.php/en/)”, Proceedings of the 4th International Workshop on “[Digitization and e-Inclusion in Mathematics and Science 2021](https://workshop.sciaccess.net/deims2021/DEIMS2021_Proceedings.zip)” DEIMS2021, February 18-19, 2021, Tokyo, pp. 105-114. 

Thanks to everyone who volunteered!

# About me

I've been working on math accessibility since 2002. At the time, I worked on Mathematica's WYSIWYG math editor and other UI features. Prof. John Gardner, who had lost his sight 15 years earlier, asked whether I could make the Mathematica frontend accessible. I maybe got 80% of the way there, but the company wasn't interested in pursuing this and ultimately I left the company and the company removed the code. That was the start of my accessibility journey: one step forward, one step back, and then forward again because allowing _everyone_ to have a chance to find the joy of math and science has given purpose to my life.

I then joined Design Science, Inc (DSI) which had an interest in making math accessible. At the time, DSI had recently developed MathPlayer, a plugin for IE6 that displayed MathML. I worked on adding features to that and with the company's support, applied for and received an NSF grant to make MathPlayer accessible. That work was quite successful and in subsequent years I continued to add features to it. However, for security reasons, Internet Explorer removed the interface that MathPlayer depended upon. It's tempting to say that is what doomed IE... After that, MathPlayer became an accessibility-only NVDA add-on. Further work through an IES grant with ETS refined MathPlayer's capabilities; valuable insight was gained via user-studies funded by the grant.

For more information about what happened to MathPlayer and how MathCAT came to be, see the [Why MathCAT?](#why-mathcat) section.

All along, I've been pushing to make math work on the web and make it accessible. While at Wolfram Research, I helped launch the W3C MathML effort and have been involved with the working group ever since. I currently co-chair the W3C Math Working Group. I've been a member on several other committees over the years pushing strongly to make sure they incorporated math accessibility into their standards. Some of these groups include NIMAS, EPUB, and PDF/UA.

I'm very honored that in 2023, the National Federation of the Blind gave me the <span>$</span>25,000 Jacob Bolotin award. I donated <span>$</span>15,000 of that to the _open collective_ to improve MathML support in browsers. [Click this link for how you can help improve MathML support in browsers](https://opencollective.com/mathml-core-support).
