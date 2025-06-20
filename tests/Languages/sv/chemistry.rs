/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;

#[test]
fn salt() {
  let expr = "<math><mi>Na</mi><mi>Cl</mi></math>";
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "versal n a, versal c l");
}

#[test]
fn water() {
  let expr = "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>";
  test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "versal h, 2 versal o");
  test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "versal h, nedsänkt 2, versal o");
  test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "versal h, nedre index 2, versal o");
}

#[test]
fn carbon() {
  let expr = "<math><mi>C</mi></math>";     // not enough to trigger recognition
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "versal c");
}

#[test]
fn sulfate() {
  let expr = "<math><mrow><msup>
          <mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
          <mrow><mn>2</mn><mo>&#x2212;</mo></mrow>
      </msup></mrow></math>";
  test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "start hak-parentes; versal s, versal o, nedsänkt 4; slut hak-parentes upphöjt 2 minus");
}

#[test]
fn aluminum_sulfate() {
  let expr = "<math><mrow><msub><mi>Al</mi><mn>2</mn></msub>
          <msub><mrow><mo>(</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>)</mo></mrow><mn>3</mn></msub></mrow></math>";
  test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "versal a l, 2; start-parentes; versal s, versal o, 4; slut-parentes 3");
  test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "versal a l, nedsänkt 2; vänster-parentes; versal s, versal o, nedsänkt 4; höger-parentes nedsänkt 3");
  test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "versal a l, nedre index 2; vänster-parentes; versal s, versal o, nedre index 4; höger-parentes nedre index 3");
}

#[test]
fn ethanol_bonds() {
  let expr = "<math>
          <mrow>
              <mi>C</mi>
              <msub>  <mi>H</mi> <mn>3</mn> </msub>
              <mo>&#x2212;</mo>
              <mi>C</mi>
              <msub>  <mi>H</mi> <mn>2</mn> </msub>
              <mo>&#x2212;</mo>
              <mi>O</mi>
              <mi>H</mi>
          </mrow>
      </math>";
  test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "versal c, versal h, 3 enkelbindning versal c, versal h, 2 enkelbindning versal o, versal h");

}

#[test]
fn dichlorine_hexoxide() {
  let expr = "<math><mrow>
      <msup>
        <mrow><mo>[</mo><mi>Cl</mi><msub><mi>O</mi><mn>2</mn></msub><mo>]</mo></mrow>
        <mo>+</mo>
      </msup>
      <msup>
        <mrow><mo>[</mo><mi>Cl</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
        <mo>-</mo>
      </msup>
    </mrow></math>";
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")], 
    expr, "start hak-parentes; versal c l, versal o, 2; slut hak-parentes plus; start hak-parentes; versal c l, versal o, 4; slut hak-parentes minus");
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Medium")], 
    expr, "start hak-parentes; versal c l, versal o, nedsänkt 2; slut hak-parentes upphöjt plus; start hak-parentes; versal c l, versal o, nedsänkt 4; slut hak-parentes upphöjt minus");
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Verbose")], 
    expr, "start hak-parentes; versal c l, versal o, nedre index 2; slut hak-parentes övre index plus; start hak-parentes; versal c l, versal o, nedre index 4; slut hak-parentes övre index minus");
}


#[test]
fn ethylene_with_bond() {
  let expr = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>=</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "versal h, 2 versal c, dubbelbindning versal c, versal h, 2");
}

#[test]
fn ferric_chloride_aq() {
  let expr = "<math><mrow>
        <mi>Fe</mi>
        <msub><mi>Cl</mi><mn>3</mn></msub>
        <mrow><mo>(</mo><mrow><mi>aq</mi></mrow><mo>)</mo></mrow>
    </mrow></math>";
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "versal f e, versal c l, 3 vattenlösning");
  }

#[test]
fn ethylene_with_colon_bond() {
  let expr = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>::</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "versal h, 2 versal c, dubbelbindning versal c, versal h, 2");
}

#[test]
fn beta_decay() {
  let expr = "<math>
      <mmultiscripts>
        <mtext>C</mtext>
        <mprescripts />
        <mn>6</mn>
        <mn>14</mn>
      </mmultiscripts>
      <mo>&#x2192;</mo>
      <mmultiscripts>
        <mtext>N</mtext>
        <mprescripts />
        <mn>7</mn>
        <mn>14</mn>
      </mmultiscripts>
      <mo>+</mo>
      <mmultiscripts>
        <mtext>e</mtext>
        <mprescripts />
        <mrow>
          <mo>&#x2212;</mo>
          <mn>1</mn>
        </mrow>
        <mn>0</mn>
      </mmultiscripts>
    </math>";
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Terse")], expr,
    "14, 6, versal c; bildar, 14, 7, versal n; plus 0, minus 1, e");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
    "upphöjt 14, nedsänkt 6, versal c; reaktion bildar; upphöjt 14, nedsänkt 7, versal n; plus, upphöjt 0, nedsänkt minus 1, e");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Verbose")], expr,
    "övre index 14, nedre index 6, versal c; reaktion bildar; övre index 14, nedre index 7, versal n; plus, övre index 0, nedre index minus 1, e");
}

#[test]
fn mhchem_beta_decay() {
  let expr = "<math>
      <mrow>
        <msubsup>
          <mrow>
            <mrow>
              <mpadded width='0'>
                <mphantom>
                  <mi>A</mi>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0' depth='0'>
                <mphantom>
                  <mn>6</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0' depth='0'>
                <mphantom>
                  <mn>14</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
        </msubsup>
        <mspace width='-0.083em'></mspace>
        <msubsup>
          <mrow>
            <mrow>
              <mpadded width='0'>
                <mphantom>
                  <mi>A</mi>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded width='0'>
                <mphantom>
                  <mn>2</mn>
                </mphantom>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded width='0' lspace='-1width'>
                <mrow>
                  <mpadded height='0'>
                    <mn>6</mn>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0'>
                <mrow>
                  <mpadded width='0'>
                    <mphantom>
                      <mn>2</mn>
                    </mphantom>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded width='0' lspace='-1width'>
                <mn>14</mn>
              </mpadded>
            </mrow>
          </mrow>
        </msubsup>
        <mrow>
          <mi mathvariant='normal'>C</mi>
        </mrow>
        <mrow></mrow>
        <mrow>
          <mo stretchy='false'>&#x27F6;</mo>
        </mrow>
        <mrow></mrow>
        <msubsup>
          <mrow>
            <mrow>
              <mpadded width='0'>
                <mphantom>
                  <mi>A</mi>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0' depth='0'>
                <mphantom>
                  <mn>7</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0' depth='0'>
                <mphantom>
                  <mn>14</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
        </msubsup>
        <mspace width='-0.083em'></mspace>
        <msubsup>
          <mrow>
            <mrow>
              <mpadded width='0'>
                <mphantom>
                  <mi>A</mi>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded width='0'>
                <mphantom>
                  <mn>2</mn>
                </mphantom>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded width='0' lspace='-1width'>
                <mrow>
                  <mpadded height='0'>
                    <mn>7</mn>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0'>
                <mrow>
                  <mpadded width='0'>
                    <mphantom>
                      <mn>2</mn>
                    </mphantom>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded width='0' lspace='-1width'>
                <mn>14</mn>
              </mpadded>
            </mrow>
          </mrow>
        </msubsup>
        <mrow>
          <mi mathvariant='normal'>N</mi>
        </mrow>
        <mrow></mrow>
        <mo>+</mo>
        <mrow></mrow>
        <msubsup>
          <mrow>
            <mrow>
              <mpadded width='0'>
                <mphantom>
                  <mi>A</mi>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0' depth='0'>
                <mphantom>
                  <mo>&#x2212;</mo>
                  <mn>1</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0' depth='0'>
                <mphantom>
                  <mn>0</mn>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
        </msubsup>
        <mspace width='-0.083em'></mspace>
        <msubsup>
          <mrow>
            <mrow>
              <mpadded width='0'>
                <mphantom>
                  <mi>A</mi>
                </mphantom>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded width='0'>
                <mphantom>
                  <mn>2</mn>
                </mphantom>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded width='0' lspace='-1width'>
                <mrow>
                  <mpadded height='0'>
                    <mo>&#x2212;</mo>
                    <mn>1</mn>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
          </mrow>
          <mrow>
            <mrow>
              <mpadded height='0'>
                <mrow>
                  <mpadded width='0'>
                    <mphantom>
                      <mn>2</mn>
                    </mphantom>
                  </mpadded>
                </mrow>
              </mpadded>
            </mrow>
            <mrow>
              <mpadded width='0' lspace='-1width'>
                <mn>0</mn>
              </mpadded>
            </mrow>
          </mrow>
        </msubsup>
        <mrow>
          <mi mathvariant='normal'>e</mi>
        </mrow>
      </mrow>
    </math>";
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Terse")], expr,
    "14, 6, versal c; bildar, 14, 7, versal n; plus 0, minus 1, e");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
    "upphöjt 14, nedsänkt 6, versal c; reaktion bildar; upphöjt 14, nedsänkt 7, versal n; plus, upphöjt 0, nedsänkt minus 1, e");
    test_prefs("sv", "ClearSpeak", vec![("Verbosity", "Verbose")], expr,
    "övre index 14, nedre index 6, versal c; reaktion bildar; övre index 14, nedre index 7, versal n; plus, övre index 0, nedre index minus 1, e");
}

#[test]
fn hcl_na_yields() {
    let expr = "<math> <mrow>
      <mn>2</mn><mi>H</mi><mi>Cl</mi><mo>+</mo><mn>2</mn><mtext>Na</mtext>
      <mo>&#x2192;</mo>
      <mn>2</mn><mtext>Na</mtext><mi>Cl</mi><mo>+</mo>
      <msub> <mi>H</mi> <mn>2</mn> </msub>
      </mrow>
    </math>";
    test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
    "2, versal h, versal c l; plus 2 versal n a; reaktion bildar; 2, versal n a, versal c l; plus versal h, nedre index 2");
}

#[test]
fn mhchem_so4_2plus() {
  let expr = "<math>
    <mrow>
      <mrow>
        <mi>SO</mi>
      </mrow>
      <msub>
        <mrow>
          <mrow>
            <mpadded width='0'>
              <mphantom>
                <mi>A</mi>
              </mphantom>
            </mpadded>
          </mrow>
        </mrow>
        <mrow>
          <mrow>
            <mpadded height='0'>
              <mn>4</mn>
            </mpadded>
          </mrow>
        </mrow>
      </msub>
      <msup>
        <mrow>
          <mrow>
            <mpadded width='0'>
              <mphantom>
                <mi>A</mi>
              </mphantom>
            </mpadded>
          </mrow>
        </mrow>
        <mrow>
          <mn>2</mn>
          <mo>+</mo>
        </mrow>
      </msup>
    </mrow>
  </math>";
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "versal s; versal o, 4, 2 plus");
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "versal s; versal o, nedsänkt 4, upphöjt 2 plus");
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "versal s; versal o, nedre index 4, övre index 2 plus");
}


#[test]
fn mhchem_hcl_aq_etc() {
  let expr = "<math>
    <mrow>
      <mn>2</mn>
      <mstyle scriptlevel='0'>
        <mspace width='0.167em'></mspace>
      </mstyle>
      <mrow>
        <mi>HCl</mi>
      </mrow>
      <mspace width='0.111em'></mspace>
      <mo stretchy='false'>(</mo>
      <mrow>
        <mi>aq</mi>
      </mrow>
      <mo stretchy='false'>)</mo>
      <mrow></mrow>
      <mo>+</mo>
      <mrow></mrow>
      <mn>2</mn>
      <mstyle scriptlevel='0'>
        <mspace width='0.167em'></mspace>
      </mstyle>
      <mrow>
        <mi>Na</mi>
      </mrow>
      <mspace width='0.111em'></mspace>
      <mo stretchy='false'>(</mo>
      <mrow>
        <mi mathvariant='normal'>s</mi>
      </mrow>
      <mo stretchy='false'>)</mo>
      <mrow></mrow>
      <mrow>
        <mo stretchy='false'>&#x27F6;</mo>
      </mrow>
      <mrow></mrow>
      <mn>2</mn>
      <mstyle scriptlevel='0'>
        <mspace width='0.167em'></mspace>
      </mstyle>
      <mrow>
        <mi>NaCl</mi>
      </mrow>
      <mspace width='0.111em'></mspace>
      <mo stretchy='false'>(</mo>
      <mrow>
        <mi>aq</mi>
      </mrow>
      <mo stretchy='false'>)</mo>
      <mrow></mrow>
      <mo>+</mo>
      <mrow></mrow>
      <mrow>
        <mi mathvariant='normal'>H</mi>
      </mrow>
      <msub>
        <mrow>
          <mrow>
            <mpadded width='0'>
              <mphantom>
                <mi>A</mi>
              </mphantom>
            </mpadded>
          </mrow>
        </mrow>
        <mrow>
          <mrow>
            <mpadded height='0'>
              <mn>2</mn>
            </mpadded>
          </mrow>
        </mrow>
      </msub>
      <mspace width='0.111em'></mspace>
      <mo stretchy='false'>(</mo>
      <mrow>
        <mi mathvariant='normal'>g</mi>
      </mrow>
      <mo stretchy='false'>)</mo>
    </mrow>
  </math>";
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "2, versal h, versal c l, vattenlösning; plus, 2, versal n a, fast; bildar; 2, versal n a, versal c l, vattenlösning; plus, versal h, 2; gas");

}


#[test]
fn mhchem_barbed_equilibrium() {
  let expr = "<math>
    <mrow data-mjx-texclass='ORD' data-chem-equation='14'>
      <mrow data-changed='added' data-chem-equation='3'>
        <mmultiscripts data-chem-formula='1'>
          <mi data-mjx-texclass='ORD' mathvariant='normal' data-chem-element='1'>H</mi>
          <mn data-mjx-texclass='ORD'>2</mn>
          <none></none>
        </mmultiscripts>
        <mo data-changed='added' data-function-guess='true'>&#x2063;</mo>
        <mrow data-changed='added' data-chem-equation='1'>
          <mo stretchy='false'>(</mo>
          <mi data-mjx-texclass='ORD' mathvariant='normal'>g</mi>
          <mo stretchy='false'>)</mo>
        </mrow>
      </mrow>
      <mo data-chem-equation-op='1'>+</mo>
      <mrow data-changed='added' data-chem-equation='10'>
        <mrow data-changed='added' data-chem-equation='3'>
          <mmultiscripts data-chem-formula='1'>
            <mi data-mjx-texclass='ORD' mathvariant='normal' data-chem-element='1'>I</mi>
            <mn data-mjx-texclass='ORD'>2</mn>
            <none></none>
          </mmultiscripts>
          <mo data-changed='added' data-function-guess='true'>&#x2063;</mo>
          <mrow data-changed='added' data-chem-equation='1'>
            <mo stretchy='false'>(</mo>
            <mi data-mjx-texclass='ORD' mathvariant='normal'>g</mi>
            <mo stretchy='false'>)</mo>
          </mrow>
        </mrow>
        <mo data-changed='added'>&#x2062;</mo>
        <mover data-mjx-texclass='REL'>
          <mrow data-mjx-texclass='ORD' depth='0' height='0' data-changed='added'>
            <mo data-mjx-texclass='ORD' stretchy='false'>↽</mo>
            <mo data-mjx-texclass='ORD'>-</mo>
          </mrow>
          <mrow data-mjx-texclass='ORD' displaystyle='false' scriptlevel='0' data-changed='added'>
            <mo data-mjx-texclass='ORD'>-</mo>
            <mo data-mjx-texclass='ORD' stretchy='false'>⇀</mo>
          </mrow>
        </mover>
        <mo data-changed='added'>&#x2062;</mo>
        <mn>2</mn>
        <mo data-changed='added'>&#x2062;</mo>
        <mrow data-changed='added' data-chem-equation='5'>
          <mi mathvariant='normal' data-chem-element='1'>H</mi>
          <mo data-changed='added'>&#x2063;</mo>
          <mi mathvariant='normal' data-chem-element='1'>I</mi>
          <mo data-changed='added' data-function-guess='true'>&#x2063;</mo>
          <mrow data-changed='added' data-chem-equation='1'>
            <mo stretchy='false'>(</mo>
            <mi data-mjx-texclass='ORD' mathvariant='normal'>g</mi>
            <mo stretchy='false'>)</mo>
          </mrow>
        </mrow>
      </mrow>
    </mrow>
  </math>";
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "versal h, 2; gas; plus; versal i, 2; gas; är i jämvikt med 2, versal h, versal i, gas");
}



#[test]
fn mhchem_roman_in_superscript() {
      let expr = "<math>
      <mrow>
        <mmultiscripts>
          <mi>Fe</mi>
          <none></none>
          <mi>II</mi>
        </mmultiscripts>
        <mo>&#x2063;</mo>
        <mmultiscripts>
          <mi>Fe</mi>
          <none></none>
          <mi>III</mi>
        </mmultiscripts>
        <mo>&#x2063;</mo>
        <mmultiscripts>
          <mi mathvariant='normal' >O</mi>
          <mn>4</mn>
          <none></none>
        </mmultiscripts>
      </mrow>
    </math>";
  test_prefs("sv", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "versal f e, 2; versal f e, 3; versal o, 4");
}


