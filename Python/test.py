import os
import shutil

if os.path.exists("mathcat.pyd"):
	os.remove("mathcat.pyd")
shutil.copy("..\\target\\i686-pc-windows-msvc\\debug\\mathcat.dll", "mathcat.pyd")

import mathcat

mathcat.SetMathML("<math><mfrac> <mn>1</mn> <mn>2</mn> </mfrac> </math>")
print("Spoken text is '", mathcat.GetSpokenText(), "'")

mathcat.SetMathML("<math><mfrac> <mn>1</mn> <mi>A</mi> </mfrac> </math>")
print("Spoken text is '", mathcat.GetSpokenText(), "'")

mathcat.SetPreference("TTS", "SAPI5")
mathcat.SetMathML("<math><mfrac> <mn>1</mn> <mi>A</mi> </mfrac> </math>")
print("Spoken text is '", mathcat.GetSpokenText(), "'")


mathcat.SetMathML("""
        <math>
        <mrow>
         <mfrac>
          <mrow>
           <mfrac>
             <mn>10</mn>
             <mi>n</mi>
           </mfrac>
           </mrow>
          <mrow>
           <mfrac>
            <mn>2</mn>
            <mi>n</mi>
           </mfrac>
           </mrow>
         </mfrac>
         </mrow>
       </math>""")
print("Spoken text is '", mathcat.GetSpokenText(), "'")

mathcat.SetMathML("""
        <math xmlns='http://www.w3.org/1998/Math/MathML'>
         <mrow>
          <mrow><mo>(</mo>
            <mtable>
             <mtr>
              <mtd>
               <mrow>
                <mi>x</mi>
               </mrow>
              </mtd>
             </mtr>
             <mtr>
              <mtd>
               <mrow>
                <mi>a</mi>
               </mrow>
              </mtd>
             </mtr>
             <mtr>
              <mtd>
               <mfrac>
                 <mi>x</mi>
                 <mrow>
                   <mi>x</mi><mo>+</mo><mn>1</mn>
                 </mrow>
                </mfrac>
              </mtd>
             </mtr>
            </mtable>
          <mo>)</mo></mrow></mrow>
        </math>""")
print("Spoken text is '", mathcat.GetSpokenText(), "'")
