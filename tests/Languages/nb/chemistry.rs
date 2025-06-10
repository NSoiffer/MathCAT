/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;

#[test]
fn salt() {
  let expr = "<math><mi>Na</mi><mi>Cl</mi></math>";
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "stor n a, stor c l");
}

#[test]
fn water() {
  let expr = "<math><msub><mi>H</mi><mn>2</mn></msub><mi>O</mi></math>";
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "stor h, 2, stor o");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "stor h, senket 2, stor o");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "stor h, nedre indeks 2, stor o");
}

#[test]
fn carbon() {
  let expr = "<math><mi>C</mi></math>";     // not enough to trigger recognition
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "stor c");
}

#[test]
fn sulfate() {
  let expr = "<math><mrow><msup>
          <mrow><mo>[</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>]</mo></mrow>
          <mrow><mn>2</mn><mo>&#x2212;</mo></mrow>
      </msup></mrow></math>";
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "start hakeparentes; stor s, stor o, senket 4; slutt hakeparentes hevet 2 minus");
}

#[test]
fn aluminum_sulfate() {
  let expr = "<math><mrow><msub><mi>Al</mi><mn>2</mn></msub>
          <msub><mrow><mo>(</mo><mi>S</mi><msub><mi>O</mi><mn>4</mn></msub><mo>)</mo></mrow><mn>3</mn></msub></mrow></math>";
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "stor a l, 2; startparentes; stor s, stor o, 4; sluttparentes 3");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "stor a l, senket 2; startparentes; stor s, stor o, senket 4; sluttparentes senket 3");
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "stor a l, nedre indeks 2; startparentes; stor s, stor o, nedre indeks 4; sluttparentes nedre indeks 3");
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
  test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "stor c, stor h, 3; enkeltbinding stor c, stor h, 2; enkeltbinding stor o, stor h");

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
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], 
    expr, "start hakeparentes; stor c l, stor o, 2; slutt hakeparentes pluss; \
                          start hakeparentes; stor c l, stor o, 4; slutt hakeparentes minus");
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")], 
    expr, "start hakeparentes; stor c l, stor o, senket 2; slutt hakeparentes hevet pluss; \
                          start hakeparentes; stor c l, stor o, senket 4; slutt hakeparentes hevet minus");
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], 
    expr, "start hakeparentes; stor c l, stor o, nedre indeks 2; slutt hakeparentes øvre indeks pluss; \
                          start hakeparentes; stor c l, stor o, nedre indeks 4; slutt hakeparentes øvre indeks minus");
}


#[test]
fn ethylene_with_bond() {
  let expr = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>=</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "stor h, 2, stor c, dobbeltbinding stor c, stor h, 2");
}

#[test]
fn ferric_chloride_aq() {
  let expr = "<math><mrow>
        <mi>Fe</mi>
        <msub><mi>Cl</mi><mn>3</mn></msub>
        <mrow><mo>(</mo><mrow><mi>aq</mi></mrow><mo>)</mo></mrow>
    </mrow></math>";
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "stor f e, stor c l, 3; løst i vann");
  }

#[test]
fn ethylene_with_colon_bond() {
  let expr = "<math><mrow>
          <msub><mi>H</mi><mn>2</mn></msub><mi>C</mi>
          <mo>::</mo>
          <mi>C</mi><msub><mi>H</mi><mn>2</mn></msub>
      </mrow></math>";
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "stor h, 2, stor c, dobbeltbinding stor c, stor h, 2");
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
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Terse")], expr,
      "14, 6, stor c; danner, 14, 7, stor n; pluss 0, minus 1, e");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
      "hevet 14, senket 6, stor c; reaksjon danner; hevet 14, senket 7, stor n; pluss, hevet 0, senket minus 1, e");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Verbose")], expr,
      "øvre indeks 14, nedre indeks 6, stor c; reaksjon danner; øvre indeks 14, nedre indeks 7, stor n; pluss, øvre indeks 0, nedre indeks minus 1, e");
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
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Terse")], expr,
      "14, 6, stor c; danner, 14, 7, stor n; pluss 0, minus 1, e");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
      "hevet 14, senket 6, stor c; reaksjon danner; hevet 14, senket 7, stor n; pluss, hevet 0, senket minus 1, e");
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Verbose")], expr,
      "øvre indeks 14, nedre indeks 6, stor c; reaksjon danner; øvre indeks 14, nedre indeks 7, stor n; pluss, øvre indeks 0, nedre indeks minus 1, e");
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
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
        "2, stor h, stor c l; pluss 2 stor n a; reaksjon danner; 2, stor n a, stor c l; pluss stor h, nedre indeks 2");
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
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")], expr, "stor s; stor o, 4, 2 pluss");
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "stor s; stor o, senket 4, hevet 2 pluss");
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "stor s; stor o, nedre indeks 4, øvre indeks 2 pluss");
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
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "2, stor h, stor c l, løst i vann; pluss, 2, stor n a, fast; danner; 2, stor n a, stor c l, løst i vann; pluss, stor h, 2, gass");

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
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "stor h, 2, gass; pluss; stor i, 2, gass; er i likevekt med 2, stor h, stor i, gass");
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
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "stor f e, 2; stor f e, 3; stor o, 4");
}


#[test]
fn dropped_msubsup_bug_358() {
      let expr = r#"<math>
          <mrow class="MJX-TeXAtom-ORD">
              <mrow class="MJX-TeXAtom-ORD">
                <mn>2</mn>
                <mspace width="thinmathspace"></mspace>
                <msubsup>
                  <mtext>SO</mtext>
                  <mrow class="MJX-TeXAtom-ORD">
                    <mn>2</mn>
                  </mrow>
                  <mrow class="MJX-TeXAtom-ORD">
                    <mspace width="0pt" height="0pt" depth=".2em"></mspace>
                  </mrow>
                </msubsup>
                <mo>+</mo>
                <msubsup>
                  <mtext>O</mtext>
                  <mrow class="MJX-TeXAtom-ORD">
                    <mn>2</mn>
                  </mrow>
                  <mrow class="MJX-TeXAtom-ORD">
                    <mspace width="0pt" height="0pt" depth=".2em"></mspace>
                  </mrow>
                </msubsup>
                <mrow class="MJX-TeXAtom-REL">
                  <mover>
                    <mrow class="MJX-TeXAtom-OP MJX-fixedlimits">
                      <mrow class="MJX-TeXAtom-ORD">
                        <mpadded height="0" depth="0">
                          <mrow class="MJX-TeXAtom-ORD">
                            <mo stretchy="false">↽<!-- ↽ --></mo>
                          </mrow>
                          <mspace width="negativethinmathspace"></mspace>
                          <mspace width="negativethinmathspace"></mspace>
                          <mrow class="MJX-TeXAtom-ORD">
                            <mo>−<!-- − --></mo>
                          </mrow>
                        </mpadded>
                      </mrow>
                    </mrow>
                    <mrow class="MJX-TeXAtom-ORD">
                        <mrow class="MJX-TeXAtom-ORD">
                          <mrow class="MJX-TeXAtom-ORD">
                            <mo>−<!-- − --></mo>
                          </mrow>
                          <mspace width="negativethinmathspace"></mspace>
                          <mspace width="negativethinmathspace"></mspace>
                          <mrow class="MJX-TeXAtom-ORD">
                            <mo stretchy="false">⇀<!-- ⇀ --></mo>
                          </mrow>
                        </mrow>
                    </mrow>
                  </mover>
                </mrow>
                <mn>2</mn>
                <mspace width="thinmathspace"></mspace>
                <msubsup>
                  <mtext>SO</mtext>
                  <mrow class="MJX-TeXAtom-ORD">
                    <mn>3</mn>
                  </mrow>
                  <mrow class="MJX-TeXAtom-ORD">
                    <mspace width="0pt" height="0pt" depth=".2em"></mspace>
                  </mrow>
                </msubsup>
              </mrow>
          </mrow>
      </math>"#;
  test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")],
      expr, "2, stor s, stor o, 2; pluss; stor o, 2, er i likevekt med 2, stor s, stor o, 3");
}


