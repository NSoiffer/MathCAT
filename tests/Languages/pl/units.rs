/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;
use anyhow::Result;

// The basic layout of the tests is:
// 1. Sweep through all the SI prefixes
// 2. Sweep through each group of SI units
//    a) with both singular and plural without prefixes
//    b) with both singular and plural with one prefix
// 3. Sweep through each group of units that don't take SI prefixes
// These are broken into chunks so it is easier to see errors, when there are errors

#[test]
fn prefix_sweep() -> Result<()> {
    let expr = r#"<math>
        <mi intent=":unit">Qg</mi><mo>,</mo>
        <mi intent=":unit">Rg</mi><mo>,</mo>
        <mi intent=":unit">Yg</mi><mo>,</mo>
        <mi intent=":unit">Zg</mi><mo>,</mo>
        <mi intent=":unit">Eg</mi><mo>,</mo>
        <mi intent=":unit">Pg</mi><mo>,</mo>
        <mi intent=":unit">Tg</mi><mo>,</mo>
        <mi intent=":unit">Gg</mi><mo>,</mo>
        <mi intent=":unit">Mg</mi><mo>,</mo>
        <mi intent=":unit">kg</mi><mo>,</mo>
        <mi intent=":unit">hg</mi><mo>,</mo>
        <mi intent=":unit">dag</mi><mo>,</mo>
        <mi intent=":unit">dg</mi><mo>,</mo>
        <mi intent=":unit">cg</mi><mo>,</mo>
        <mi intent=":unit">mg</mi><mo>,</mo>
        <mi intent=":unit">µg</mi><mo>,</mo>
        <mi intent=":unit">ng</mi><mo>,</mo>
        <mi intent=":unit">pg</mi><mo>,</mo>
        <mi intent=":unit">fg</mi><mo>,</mo>
        <mi intent=":unit">ag</mi><mo>,</mo>
        <mi intent=":unit">zg</mi><mo>,</mo>
        <mi intent=":unit">yg</mi><mo>,</mo>
        <mi intent=":unit">rg</mi><mo>,</mo>
        <mi intent=":unit">qg</mi>
        </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "kwetta-gramów, przecinek; ronna-gramów, przecinek; jotta-gramów, przecinek; zetta-gramów, przecinek; eksa-gramów, przecinek; peta-gramów, przecinek; tera-gramów, przecinek; giga-gramów, przecinek; mega-gramów, przecinek; kilo-gramów, przecinek; hekto-gramów, przecinek; deka-gramów, przecinek; decy-gramów, przecinek; centy-gramów, przecinek; mili-gramów, przecinek; mikro-gramów, przecinek; nano-gramów, przecinek; piko-gramów, przecinek; femto-gramów, przecinek; atto-gramów, przecinek; zepto-gramów, przecinek; jokto-gramów, przecinek; ronto-gramów, przecinek; kwekto-gramów")?;
                return Ok(());

}

#[test]
fn si_base() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">A</mi><mo>,</mo><mn>2</mn><mi intent=":unit">A</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cd</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">K</mi><mo>,</mo><mn>2</mn><mi intent=":unit">K</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">K</mi><mo>,</mo><mn>2</mn><mi intent=":unit">K</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">g</mi><mo>,</mo><mn>2</mn><mi intent=":unit">g</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">m</mi><mo>,</mo><mn>2</mn><mi intent=":unit">m</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">mol</mi><mo>,</mo><mn>2</mn><mi intent=":unit">mol</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">s</mi><mo>,</mo><mn>2</mn><mi intent=":unit">s</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">″</mi><mo>,</mo><mn>2</mn><mi intent=":unit">″</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">&quot;</mi><mo>,</mo><mn>2</mn><mi intent=":unit">&quot;</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">sec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">sec</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "1 amper, przecinek; 2 ampery, przecinek; 1 kandela, przecinek; 2 kandele, przecinek; 1 kelwin, przecinek; 2 kelwiny, przecinek; 1 kelwin, przecinek; 2 kelwiny, przecinek; 1 gram, przecinek; 2 gramy, przecinek; 1 metr, przecinek; 2 metry, przecinek, 1 mol, przecinek; 2 mole, przecinek; 1 sekunda, przecinek; 2 sekundy, przecinek; 1 sekunda, przecinek; 2 sekundy, przecinek; 1 sekunda, przecinek; 2 sekundy, przecinek; 1 sekunda, przecinek; 2 sekundy")?;
                return Ok(());

}

#[test]
fn si_base_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">QA</mi><mo>,</mo><mn>2</mn><mi intent=":unit">RA</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ycd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Zcd</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">EK</mi><mo>,</mo><mn>2</mn><mi intent=":unit">PK</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TK</mi><mo>,</mo><mn>2</mn><mi intent=":unit">GK</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Mg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kg</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">hm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dam</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dmol</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cmol</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ms</mi><mo>,</mo><mn>2</mn><mi intent=":unit">µs</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">nsec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">psec</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "1 kwetta-amper, przecinek; 2 ronna-ampery, przecinek; 1 jotta-kandela, przecinek; 2 zetta-kandele, przecinek; 1 eksa-kelwin, przecinek; 2 peta-kelwiny, przecinek; 1 tera-kelwin, przecinek; 2 giga-kelwiny, przecinek; 1 mega-gram, przecinek; 2 kilo-gramy, przecinek; 1 hekto-metr, przecinek; 2 deka-metry, przecinek; 1 decy-mol, przecinek; 2 centy-mole, przecinek; 1 mili-sekunda, przecinek; 2 mikro-sekundy, przecinek; 1 nano-sekunda, przecinek; 2 piko-sekundy")?;
                return Ok(());

}


#[test]
fn si_derived_1() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">Bq</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Bq</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">C</mi><mo>,</mo><mn>2</mn><mi intent=":unit">C</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">°C</mi><mo>,</mo><mn>2</mn><mi intent=":unit">°C</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">℃</mi><mo>,</mo><mn>2</mn><mi intent=":unit">℃</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">F</mi><mo>,</mo><mn>2</mn><mi intent=":unit">F</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Gy</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Gy</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">H</mi><mo>,</mo><mn>2</mn><mi intent=":unit">H</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Hz</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Hz</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">J</mi><mo>,</mo><mn>2</mn><mi intent=":unit">J</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">kat</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kat</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">lm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">lm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">lx</mi><mo>,</mo><mn>2</mn><mi intent=":unit">lx</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "1 bekerel, przecinek; 2 bekerele, przecinek; 1 kulomb, przecinek; 2 kulomby, przecinek; 1 stopień Celsjusza, przecinek; 2 stopnie Celsjusza, przecinek; 1 stopień Celsjusza, przecinek; 2 stopnie Celsjusza, przecinek; 1 farad, przecinek; 2 farady, przecinek; 1 grej, przecinek; 2 greje, przecinek; 1 henr, przecinek; 2 henry, przecinek; 1 herc, przecinek; 2 herce, przecinek; 1 dżul, przecinek; 2 dżule, przecinek; 1 katal, przecinek; 2 katale, przecinek; 1 lumen, przecinek; 2 lumeny, przecinek; 1 luks, przecinek; 2 luksy")?;
                return Ok(());

}

#[test]
fn si_derived_1_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">QBq</mi><mo>,</mo><mn>2</mn><mi intent=":unit">RBq</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">YC</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ZC</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">EF</mi><mo>,</mo><mn>2</mn><mi intent=":unit">PF</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TGy</mi><mo>,</mo><mn>2</mn><mi intent=":unit">GGy</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MH</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kH</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">daHz</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dHz</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cJ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">mJ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">µkat</mi><mo>,</mo><mn>2</mn><mi intent=":unit">nkat</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">plm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">flm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">alx</mi><mo>,</mo><mn>2</mn><mi intent=":unit">zlx</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">m°C</mi><mo>,</mo><mn>2</mn><mi intent=":unit">µ°C</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">p℃</mi><mo>,</mo><mn>2</mn><mi intent=":unit">n℃</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "1 kwetta-bekerel, przecinek; 2 ronna-bekerele, przecinek; 1 jotta-kulomb, przecinek; 2 zetta-kulomby, przecinek; 1 eksa-farad, przecinek; 2 peta-farady, przecinek; 1 tera-grej, przecinek; 2 giga-greje, przecinek; 1 mega-henr, przecinek; 2 kilo-henry, przecinek; 1 deka-herc, przecinek; 2 decy-herce, przecinek; 1 centy-dżul, przecinek; 2 mili-dżule, przecinek; 1 mikro-katal, przecinek; 2 nano-katale, przecinek; 1 piko-lumen, przecinek; 2 femto-lumeny, przecinek; 1 atto-luks, przecinek; 2 zepto-luksy, przecinek; 1 mili-stopień Celsjusza; przecinek; 2 mikro-stopnie Celsjusza; przecinek; 1 piko-stopień Celsjusza; przecinek; 2 nano-stopnie Celsjusza")?;
                return Ok(());

}

#[test]
fn si_derived_2() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">N</mi><mo>,</mo><mn>2</mn><mi intent=":unit">N</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ω</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ω</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ω</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ω</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Pa</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Pa</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">S</mi><mo>,</mo><mn>2</mn><mi intent=":unit">S</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Sv</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Sv</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">T</mi><mo>,</mo><mn>2</mn><mi intent=":unit">T</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">V</mi><mo>,</mo><mn>2</mn><mi intent=":unit">V</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">W</mi><mo>,</mo><mn>2</mn><mi intent=":unit">W</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Wb</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Wb</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "1 niuton, przecinek; 2 niutony, przecinek, 1 om przecinek, 2 omy, przecinek, 1 om przecinek, 2 omy, przecinek; 1 paskal, przecinek; 2 paskale, przecinek; 1 siemens, przecinek; 2 siemensy, przecinek; 1 siwert, przecinek; 2 siwerty, przecinek; 1 tesla, przecinek; 2 tesle, przecinek; 1 wolt, przecinek; 2 wolty, przecinek, 1 wat, przecinek; 2 waty, przecinek; 1 weber, przecinek; 2 webery")?;
                return Ok(());

}

#[test]
fn si_derived_2_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">qN</mi><mo>,</mo><mn>2</mn><mi intent=":unit">rN</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">yΩ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">zΩ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">aΩ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">fΩ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">pPa</mi><mo>,</mo><mn>2</mn><mi intent=":unit">nPa</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">µS</mi><mo>,</mo><mn>2</mn><mi intent=":unit">mS</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cSv</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dSv</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">daT</mi><mo>,</mo><mn>2</mn><mi intent=":unit">hT</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">kV</mi><mo>,</mo><mn>2</mn><mi intent=":unit">MV</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">GW</mi><mo>,</mo><mn>2</mn><mi intent=":unit">TW</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">PWb</mi><mo>,</mo><mn>2</mn><mi intent=":unit">EWb</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "1 kwekto-niuton, przecinek; 2 ronto-niutony, przecinek; 1 jokto-om, przecinek; 2 zepto-omy, przecinek; 1 atto-om, przecinek; 2 femto-omy, przecinek; 1 piko-paskal, przecinek; 2 nano-paskale, przecinek; 1 mikro-siemens, przecinek; 2 mili-siemensy, przecinek; 1 centy-siwert, przecinek; 2 decy-siwerty, przecinek; 1 deka-tesla, przecinek; 2 hekto-tesle, przecinek; 1 kilo-wolt, przecinek; 2 mega-wolty, przecinek; 1 giga-wat, przecinek; 2 tera-waty, przecinek; 1 peta-weber, przecinek; 2 eksa-webery")?;
                return Ok(());

}


#[test]
fn si_accepted() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">l</mi><mo>,</mo><mn>2</mn><mi intent=":unit">l</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">L</mi><mo>,</mo><mn>2</mn><mi intent=":unit">L</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ℓ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ℓ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">t</mi><mo>,</mo><mn>2</mn><mi intent=":unit">t</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Da</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Da</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Np</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Np</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">u</mi><mo>,</mo><mn>2</mn><mi intent=":unit">u</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">eV</mi><mo>,</mo><mn>2</mn><mi intent=":unit">eV</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">rad</mi><mo>,</mo><mn>2</mn><mi intent=":unit">rad</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">sr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">sr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">a</mi><mo>,</mo><mn>2</mn><mi intent=":unit">a</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">as</mi><mo>,</mo><mn>2</mn><mi intent=":unit">as</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">b</mi><mo>,</mo><mn>2</mn><mi intent=":unit">b</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">B</mi><mo>,</mo><mn>2</mn><mi intent=":unit">B</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Bd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Bd</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "1 litr, przecinek; 2 litry, przecinek; 1 litr, przecinek; 2 litry, przecinek; 1 litr, przecinek; 2 litry, przecinek; 1 tona, przecinek; 2 tony, przecinek; 1 dalton, przecinek; 2 daltony, przecinek; 1 neper, przecinek; 2 nepery, przecinek; 1 jednostka masy atomowej, przecinek; 2 jednostki masy atomowej, przecinek; 1 elektronowolt, przecinek; 2 elektronowolty, przecinek; 1 radian, przecinek; 2 radiany, przecinek; 1 steradian, przecinek; 2 steradiany, przecinek, 1 rok, przecinek; 2 lata, przecinek; 1 sekunda łuku, przecinek; 2 sekundy łuku, przecinek, 1 bit, przecinek; 2 bity, przecinek; 1 bajt, przecinek; 2 bajty, przecinek, 1 bod, przecinek; 2 body")?;
                return Ok(());

}

#[test]
fn si_accepted_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">Ql</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Rl</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">YL</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ZL</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Eℓ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Pℓ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Tt</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Gt</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MDa</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kDa</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dNp</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cNp</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">hu</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dau</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">meV</mi><mo>,</mo><mn>2</mn><mi intent=":unit">µeV</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">nrad</mi><mo>,</mo><mn>2</mn><mi intent=":unit">prad</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">fsr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">asr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ga</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ma</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">zas</mi><mo>,</mo><mn>2</mn><mi intent=":unit">yas</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">kb</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Mb</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">GB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">TB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TBd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">EBd</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "1 kwetta-litr, przecinek; 2 ronna-litry, przecinek; 1 jotta-litr, przecinek; 2 zetta-litry, przecinek; 1 eksa-litr, przecinek; 2 peta-litry, przecinek; 1 tera-tona, przecinek; 2 giga-tony, przecinek; 1 mega-dalton, przecinek; 2 kilo-daltony, przecinek; 1 decy-neper, przecinek; 2 centy-nepery, przecinek; 1, hekto-jednostka masy atomowej; przecinek; 2, deka-jednostki masy atomowej; przecinek; 1 mili-elektronowolt; przecinek; 2 mikro-elektronowolty; przecinek; 1 nano-radian, przecinek; 2 piko-radiany, przecinek; 1 femto-steradian, przecinek; 2 atto-steradiany, przecinek; 1 giga-rok, przecinek; 2 mega-lata, przecinek; 1 zepto-sekunda łuku; przecinek; 2 jokto-sekundy łuku; przecinek; 1 kilo-bit, przecinek; 2 mega-bity, przecinek; 1 giga-bajt, przecinek; 2 tera-bajty, przecinek; 1 tera-bod, przecinek; 2 eksa-body")?;
                return Ok(());

}

#[test]
fn without_prefix_time() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">″</mi><mo>,</mo><mn>2</mn><mi intent=":unit">″</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">&quot;</mi><mo>,</mo><mn>2</mn><mi intent=":unit">&quot;</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">′</mi><mo>,</mo><mn>2</mn><mi intent=":unit">′</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">'</mi><mo>,</mo><mn>2</mn><mi intent=":unit">'</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">min</mi><mo>,</mo><mn>2</mn><mi intent=":unit">min</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">h</mi><mo>,</mo><mn>2</mn><mi intent=":unit">h</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">hr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">hr</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">Hr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Hr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">d</mi><mo>,</mo><mn>2</mn><mi intent=":unit">d</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dy</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dy</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">w</mi><mo>,</mo><mn>2</mn><mi intent=":unit">w</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">wk</mi><mo>,</mo><mn>2</mn><mi intent=":unit">wk</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">y</mi><mo>,</mo><mn>2</mn><mi intent=":unit">y</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">yr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">yr</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "1 sekunda, przecinek; 2 sekundy, przecinek; 1 sekunda, przecinek; 2 sekundy, przecinek; 1 minuta, przecinek; 2 minuty, przecinek; 1 minuta, przecinek; 2 minuty, przecinek; 1 minuta, przecinek; 2 minuty, przecinek; 1 godzina, przecinek; 2 godziny, przecinek; 1 godzina, przecinek; 2 godziny, przecinek; 1 godzina, przecinek; 2 godziny, przecinek; 1 dzień, przecinek, 2 dni, przecinek; 1 dzień, przecinek, 2 dni, przecinek; 1 tydzień, przecinek; 2 tygodnie, przecinek; 1 tydzień, przecinek; 2 tygodnie, przecinek, 1 rok, przecinek; 2 lata, przecinek, 1 rok, przecinek; 2 lata")?;
                return Ok(());

}

#[test]
fn without_prefix_angles() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">°</mi><mo>,</mo><mn>2</mn><mi intent=":unit">°</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">deg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">deg</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">arcmin</mi><mo>,</mo><mn>2</mn><mi intent=":unit">arcmin</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">amin</mi><mo>,</mo><mn>2</mn><mi intent=":unit">amin</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">am</mi><mo>,</mo><mn>2</mn><mi intent=":unit">am</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MOA</mi><mo>,</mo><mn>2</mn><mi intent=":unit">MOA</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">arcsec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">arcsec</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">asec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">asec</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "1 stopień, przecinek; 2 stopnie, przecinek; 1 stopień, przecinek; 2 stopnie, przecinek; 1 minuta łuku, przecinek; 2 minuty łuku, przecinek; 1 minuta łuku, przecinek; 2 minuty łuku, przecinek; 1 minuta łuku, przecinek; 2 minuty łuku, przecinek; 1 minuta łuku, przecinek; 2 minuty łuku, przecinek; 1 sekunda łuku, przecinek; 2 sekundy łuku, przecinek; 1 sekunda łuku, przecinek; 2 sekundy łuku")?;
                return Ok(());

}

#[test]
fn without_prefix_distance() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">au</mi><mo>,</mo><mn>2</mn><mi intent=":unit">au</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ltyr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ltyr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">pc</mi><mo>,</mo><mn>2</mn><mi intent=":unit">pc</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Å</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Å</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Å</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Å</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">fm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">fm</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "1 jednostka astronomiczna, przecinek; 2 jednostki astronomiczne, przecinek; 1 rok świetlny, przecinek; 2 lata świetlne, przecinek; 1 parsek, przecinek; 2 parseki, przecinek; 1 angstrem, przecinek; 2 angstremy, przecinek; 1 angstrem, przecinek; 2 angstremy, przecinek; 1 fermi, przecinek; 2 fermi")?;
                return Ok(());

}

#[test]
fn without_prefix_other() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">ha</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ha</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">atm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">atm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">amu</mi><mo>,</mo><mn>2</mn><mi intent=":unit">amu</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">bar</mi><mo>,</mo><mn>2</mn><mi intent=":unit">bar</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cal</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cal</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ci</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ci</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">grad</mi><mo>,</mo><mn>2</mn><mi intent=":unit">grad</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">M</mi><mo>,</mo><mn>2</mn><mi intent=":unit">M</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">R</mi><mo>,</mo><mn>2</mn><mi intent=":unit">R</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">rpm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">rpm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">fl dr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">fl dr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">℧</mi><mo>,</mo><mn>2</mn><mi intent=":unit">℧</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dyn</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dyn</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">erg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">erg</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "1 hektar, przecinek; 2 hektary, przecinek; 1 decybel, przecinek; 2 decybele, przecinek; 1 atmosfera, przecinek; 2 atmosfery, przecinek; 1 jednostka masy atomowej, przecinek; 2 jednostki masy atomowej, przecinek, 1 bar, przecinek; 2 bary, przecinek; 1 kaloria, przecinek; 2 kalorie, przecinek; 1 kiur, przecinek; 2 kiury, przecinek; 1 grad, przecinek; 2 grady, przecinek; 1 molowy, przecinek; 2 molowe, przecinek; 1 rentgen, przecinek; 2 rentgeny, przecinek; 1 obrót na minutę, przecinek; 2 obroty na minutę, przecinek; 1 drachma płynu, przecinek; 2 drachmy płynu, przecinek, 1 mho, przecinek, 2 mho, przecinek; 1 dyna, przecinek; 2 dyny, przecinek, 1 erg, przecinek; 2 ergi")?;
                return Ok(());

}

#[test]
fn without_prefix_powers_of_2() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">Kib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Kib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Mib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Mib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Gib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Gib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Tib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Tib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Pib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Pib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Eib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Eib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Zib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Zib</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">Yib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Yib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">KiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">KiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">MiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">GiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">GiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">TiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">PiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">PiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">EiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">EiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ZiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ZiB</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">YiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">YiB</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, 
        "1 kibibit, przecinek; 2 kibibity, przecinek; 1 mebibit, przecinek; 2 mebibity, przecinek; 1 gibibit, przecinek; 2 gibibity, przecinek; 1 tebibit, przecinek; 2 tebibity, przecinek; 1 pebibit, przecinek; 2 pebibity, przecinek; 1 eksbibit, przecinek; 2 eksbibity, przecinek; 1 zebibit, przecinek; 2 zebibity, przecinek; 1 jobibit, przecinek; 2 jobibity, przecinek; 1 kibibajt, przecinek; 2 kibibajty, przecinek; 1 mebibajt, przecinek; 2 mebibajty, przecinek; 1 gibibajt, przecinek; 2 gibibajty, przecinek; 1 tebibajt, przecinek; 2 tebibajty, przecinek; 1 pebibajt, przecinek; 2 pebibajty, przecinek; 1 eksbibajt, przecinek; 2 eksbibajty, przecinek; 1 zebibajt, przecinek; 2 zebibajty, przecinek; 1 jobibajt, przecinek; 2 jobibajty")?;
                return Ok(());

}


#[test]
fn si_other_numbers() -> Result<()> {
    let expr = r#"<math><mn>1.0</mn><mi intent=":unit">l</mi><mo>,</mo>
                            <mn>2.0</mn><mo>&#xA0;</mo><mi intent=":unit">m</mi><mo>,</mo>
                            <mi>x</mi><mo>&#xA0;</mo><mi intent=":unit">ms</mi><mo>,</mo>
                            <mi>y</mi><mi intent=":unit">µs</mi><mo>,</mo>
                            <mi intent=":unit">dag</mi><mo>,</mo>
                            <mn>1235</mn><mi intent=":unit">daN</mi><mo>,</mo>
                            <mn>2.5</mn><mi intent=":unit">&#xB5;sec</mi><mo>,</mo>
                            <mn>32.34</mn><mi intent=":unit">mol</mi></math>"#;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Terse")], expr,
            "10 l przecinek, 20 m przecinek; x mili-sekund, przecinek; y mikro-sekund, przecinek; deka-gramów, przecinek; 1235 deka-niutonów; przecinek; 25 mikro-sekundy, przecinek; 3234 mole")?;
    test_prefs("pl", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
            "10 litr, przecinek; 20 metry, przecinek; x mili-sekund, przecinek; y mikro-sekund, przecinek; deka-gramów, przecinek; 1235 deka-niutonów; przecinek; 25 mikro-sekundy, przecinek; 3234 mole")?;
    test_prefs("pl", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "10 litr, przecinek; 20 metry, przecinek; x mili-sekund, przecinek; y mikro-sekund, przecinek; deka-gramów, przecinek; 1235 deka-niutonów; przecinek; 25 mikro-sekundy, przecinek; 3234 mole")?;
                    return Ok(());

}


#[test]
fn test_mtext_inference() -> Result<()> {
    let expr = r#"<math><mo>[</mo>
                <mn>1</mn><mtext>t</mtext><mo>,</mo>
                <mn>2</mn><mtext>PA</mtext><mo>,</mo>
                <mn>3</mn><mtext>Pa</mtext><mo>,</mo>
                <mn>4.5</mn><mtext>mT</mtext>
            <mo>]</mo></math>"#;
    test("pl", "SimpleSpeak", expr, 
        "nawias kwadratowy otwierający; 1 tona, przecinek; 2 peta-ampery, przecinek; 3 paskale, przecinek; 45 mili-tesli; nawias kwadratowy zamykający")?;
                return Ok(());

}

    #[test]
    fn infer_unit() -> Result<()> {
        let expr = r#"<math>
            <mn>3</mn><mi mathvariant="normal">m</mi><mo>,</mo>
            <mn>1</mn><mi>km</mi><mo>,</mo>
            <mn>3</mn><mtext>m</mtext><mo>,</mo>
            <mfrac><mn>3</mn><mn>10</mn></mfrac><mi mathvariant="normal">F</mi><mo>,</mo>
            <msub><mi>m</mi><mi>min</mi></msub>
            </math>"#;
        test("pl", "SimpleSpeak", expr, 
            "3 metry, przecinek; 1 kilo-metr, przecinek; 3 metry, przecinek; 3 dziesiąte faradów, przecinek; m indeks dolny minimum koniec indeksu dolnego")?;
            return Ok(());

    }
