# MathCAT: Math Capable Assistive Technology
<img src="logo.png" style="position: relative; top: 16px; z-index: -1;">
is a library that supports conversion of MathML to:

* Speech strings with embedded speech engine commands
* Braille (Nemeth and eventually other braille math codes)
* Navigation of math (in multiple ways including overviews)

A goal of MathCAT is to be an easy to use library for screen readers and other assistive technology to use to produce high quality speech and/or braille from MathML. It is a follow-on project from MathPlayer (see below) and uses lessons learned from it to do to produce even higher quality speech, navigation, and braille. MathCAT takes advantage of some new ideas the [MathML Working Group](https://mathml-refresh.github.io/charter-drafts/math-2020.html) is developing to allow authors to express their intent when they use a notation. E.g., $(3, 6)$ could be a point in the plane or an open interval, or even a shorthand notation for the greatest common divisor. When that information is conveyed in the MathML, MathCAT will use it to generate more natural sounding speech.

Todo: incorporation of third party libraries to support a common subset of TeX math commands along with ASCIIMath.

MathCAT is written in Rust and can be built to interface with C/C++. It can also be built with a Python interface. The Python interface can be used by NVDA and by Orca. 

MathCAT uses a number of heuristics that try to repair poor MathML and put it in a recommended format. For example, TeX converters and WYSIWYG editors will take "1,234+1" and break the number "1,234" apart at the comma. MathCAT recognizes that and folds the number into a single `mn`. Other repairs are structural such as creating `mrow`s based on information from MathML's operator dictionary and adding invisible function application, multiplication, addition (mixed fractions), and separators (e.g, between the $i$ and $j$ in $a\_{ij}$) when it seems appropriate. This simplifies speech and Nemeth generation and may be useful to other apps. Currently the cleanup is not exposed in an API, but potentially it could be another service of MathCAT. In general, MathCAT is somewhat conservative in its repair. However, it likely will do the wrong thing in some cases, but the hope is it does the right thing much, much more frequently. Finding common mistakes of translators to MathML and patching up the poor MathML is an ongoing project.

## Current Status
MathCAT is under active development. Initial speech, navigation, and Nemeth generation is complete and a few people have looked at them and reported bugs. MathCAT will go to a wider group of people in early January and in February will be an NVDA add-on in February. It should be usable as a MathPlayer replacement for those using the English version. It will not be as complete or polished in some ways as MathPlayer though. The Nemeth braille generation will be substantially better and include integration with navigation.

A demo to show off some of MathCAT's features and also as an aid for debugging was developed. [Visit the demo](https://nsoiffer.github.io/MathCATDemo/) and please report any bugs you find. This demo is _not_ how AT users will typically interact with MathCAT but does show features that AT can potentially expose to end users such as highlighting of the speech, navigation, and braille.

Timeline:
* ✓ December/early January: prototype usage of preliminary MathML WG proposal for "intent"
* ✓ January: Distribute MathCAT to a small group of students and other users for feedback and bug reports
* late January/February: add more intent inference rules
* February: Release MathCAT as NVDA add-on
* February/March: Work on at least one translation of MathCAT to another language
* February/March: Work on MathML->UEB translation
* March/April: work on UEB->MathML translation and explore UEB->Nemeth math translator
* Late spring/summer: develop GUI interface for setting user preferences
* Summer, Fall, and Winter: work with translators to hopefully add many languages
* Fall: potentially work on 2D Nemeth generation along with Nemeth input

These plans are very tentative and will likely change based on feedback from users and AT developers.

# Documentation for different MathCAT Users

There are many different audiences for MathCAT and each audience has different interests/needs. Please see the following documentation for details based on your needs:
* AT users: [information about preferences you can set](users.md)
* AT developers/library users: [information about the API that MathCAT exposes](callers.md)
* Translators/Rule writers: [information about the files that need to be translated](helers.md)
* MathCAT developers: information about MathCAT's design

_TODO: split the text after the next section in the various components above and link them_

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
takes about 1ms to generate the ClearSpeak string
"_e raised to the exponent, negative 1 half times; open paren; the fraction with numerator; x minus mu; and denominator sigma; close paren squared, end exponent_" along with the Nemeth braille string "⠑⠘⠤⠹⠂⠌⠆⠼⠈⠡⠷⠹⠭⠤⠨⠍⠌⠨⠎⠼⠾⠘⠘⠆".
The MathML for this expression is:
```
<math>
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
```

MathCAT uses external rules to generate speech and braille.
These take about 35ms to load; this load only happens the first time the rules are used, or if the speech style, language, or other external preference is changed. An additional 50ms are required to load the full Unicode files for speech and braille,
but studies have shown that a vast majority of English K-14 math material uses a surprisingly few number of characters.
Using open source math books, the initial load should cover at least 99.99% of the characters used in expressions encountered in English K-14 math textbooks.

The library is about 2.6mb in size.

If you are working on an in-browser solution (i.e, you are using JavaScript or some other browser-based language), MathCAT is probably not the best tool for you (although I will probably add a Javascript interface). Instead, take a look at [Speech rule engine](https://github.com/zorkow/speech-rule-engine) (SRE) by Volker Sorge. It is written in TypeScript and will likely meet your needs for an in-browser solution.


# Acknowledgements
Several people helped out in various ways with the project:

* David Carlisle -- provided invaluable help figuring out some xpath matches
* Susan Jolly -- provided lots of guidance on Nemeth generation and feedback on what is right and wrong.
* Murray Sargent and Volker Sorge -- provided tables of Nemeth translations of characters

Translators:
This has yet to be done, but initial translations will come from MathPlayer. I hope others will help out so I can list you here...