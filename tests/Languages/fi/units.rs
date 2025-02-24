/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;

// The basic layout of the tests is:
// 1. Sweep through all the SI prefixes
// 2. Sweep through each group of SI units
//    a) with both singular and plural without prefixes
//    b) with both singular and plural with one prefix
// 3. Sweep through each group of units that don't take SI prefixes
// These are broken into chunks so it is easier to see errors, when there are errors

#[test]
fn prefix_sweep() {
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
    test("fi", "SimpleSpeak", expr, 
        "kvetta-grammaa, pilkku, \
                ronna-grammaa, pilkku, \
                jotta-grammaa, pilkku; \
                tsetta-grammaa, pilkku, \
                eksa-grammaa, pilkku, \
                peta-grammaa, pilkku, \
                tera-grammaa, pilkku, \
                giga-grammaa, pilkku, \
                mega-grammaa, pilkku, \
                kilo-grammaa, pilkku, \
                hehto-grammaa, pilkku, \
                deka-grammaa, pilkku, \
                desi-grammaa, pilkku; \
                sentti-grammaa, pilkku, \
                milli-grammaa, pilkku, \
                mikro-grammaa, pilkku, \
                nano-grammaa, pilkku, \
                piko-grammaa, pilkku, \
                femto-grammaa, pilkku, \
                atto-grammaa, pilkku, \
                zepto-grammaa, pilkku, \
                jokto-grammaa, pilkku, \
                ronto-grammaa, pilkku; \
                kvekto-grammaa");
}

#[test]
fn si_base() {
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
    test("fi", "SimpleSpeak", expr, 
        "1 ampeeri, pilkku; 2 ampeeria, pilkku, \
                1 kandela, pilkku; 2 kandelaa, pilkku, \
                1 kelvin, pilkku, 2 kelviniä, pilkku, \
                1 kelvin, pilkku, 2 kelviniä, pilkku, \
                1 gramma, pilkku; 2 grammaa, pilkku, \
                1 metri, pilkku, 2 metriä, pilkku, \
                1 mooli, pilkku; 2 moolia, pilkku, \
                1 sekunti, pilkku; 2 sekuntia, pilkku, \
                1 sekunti, pilkku; 2 sekuntia, pilkku, \
                1 sekunti, pilkku; 2 sekuntia, pilkku, \
                1 sekunti, pilkku; 2 sekuntia");
}

#[test]
fn si_base_with_prefixes() {
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
    test("fi", "SimpleSpeak", expr, 
        "1 kvetta-ampeeri, pilkku; 2 ronna-ampeeria, pilkku; \
                1 jotta-kandela, pilkku; 2 tsetta-kandelaa; pilkku; \
                1 eksa-kelvin, pilkku; 2 peta-kelviniä, pilkku; \
                1 tera-kelvin, pilkku; 2 giga-kelviniä, pilkku; \
                1 mega-gramma, pilkku; 2 kilo-grammaa, pilkku; \
                1 hehto-metri, pilkku; 2 deka-metriä, pilkku; \
                1 desi-mooli, pilkku; 2 sentti-moolia, pilkku; \
                1 milli-sekunti, pilkku; 2 mikro-sekuntia, pilkku; \
                1 nano-sekunti, pilkku; 2 piko-sekuntia");
}


#[test]
fn si_derived_1() {
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
    test("fi", "SimpleSpeak", expr, 
        "1 bekrel, pilkku, 2 bekreliä, pilkku, \
                1 kulombi, pilkku; 2 kulombia, pilkku; \
                1 aste celsiusta, pilkku; 2 astetta celsiusta, pilkku; \
                1 aste celsiusta, pilkku; 2 astetta celsiusta, pilkku, \
                1 faradi, pilkku; 2 faradia, pilkku, \
                1 grei, pilkku, 2 greitä, pilkku, \
                1 henry, pilkku, 2 henryä, pilkku, \
                1 hertsi, pilkku, 2 hertsiä, pilkku, \
                1 joule, pilkku; 2 joulea, pilkku, \
                1 kattel, pilkku, 2 kattelia, pilkku, \
                1 lumen, pilkku, 2 lumenia, pilkku, \
                1 luks, pilkku, 2 luksia");
}

#[test]
fn si_derived_1_with_prefixes() {
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
    test("fi", "SimpleSpeak", expr, 
        "1 kvetta-bekrel, pilkku; 2 ronna-bekreliä, pilkku; \
                1 jotta-kulombi, pilkku; 2 tsetta-kulombia; pilkku; \
                1 eksa-faradi, pilkku; 2 peta-faradia, pilkku; \
                1 tera-grei, pilkku; 2 giga-greitä, pilkku; \
                1 mega-henry, pilkku; 2 kilo-henryä, pilkku; \
                1 deka-hertsi, pilkku; 2 desi-hertsiä, pilkku; \
                1 sentti-joule, pilkku; 2 milli-joulea, pilkku; \
                1 mikro-kattel, pilkku; 2 nano-kattelia, pilkku; \
                1 piko-lumen, pilkku; 2 femto-lumenia, pilkku; \
                1 atto-luks, pilkku; 2 zepto-luksia, pilkku; \
                1 milli-aste celsiusta; pilkku; 2 mikro-astetta celsiusta; pilkku; \
                1 piko-aste celsiusta; pilkku; 2 nano-astetta celsiusta");
}

#[test]
fn si_derived_2() {
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
    test("fi", "SimpleSpeak", expr, 
        "1 newton, pilkku, 2 newtonia, pilkku, \
                1 ohm, pilkku, 2 ohmia, pilkku, \
                1 ohm, pilkku, 2 ohmia, pilkku, \
                1 pascal, pilkku, 2 pascalia, pilkku, \
                1 siemens, pilkku; 2 siemensiä, pilkku, \
                1 sievert, pilkku; 2 sievertiä, pilkku, \
                1 tesla, pilkku; 2 teslaa, pilkku, \
                1 volt, pilkku, 2 volttia, pilkku, \
                1 watti, pilkku, 2 wattia, pilkku, \
                1 weber, pilkku, 2 weberiä");
}

#[test]
fn si_derived_2_with_prefixes() {
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
    test("fi", "SimpleSpeak", expr, 
        "1 kvekto-newton, pilkku; 2 ronto-newtonia, pilkku; \
                1 jokto-ohm, pilkku; 2 zepto-ohmia, pilkku; \
                1 atto-ohm, pilkku; 2 femto-ohmia, pilkku; \
                1 piko-pascal, pilkku; 2 nano-pascalia, pilkku; \
                1 mikro-siemens, pilkku; 2 milli-siemensiä, pilkku; \
                1 sentti-sievert, pilkku; 2 desi-sievertiä, pilkku; \
                1 deka-tesla, pilkku; 2 hehto-teslaa, pilkku; \
                1 kilo-volt, pilkku; 2 mega-volttia, pilkku; \
                1 giga-watti, pilkku; 2 tera-wattia, pilkku; \
                1 peta-weber, pilkku; 2 eksa-weberiä");
}


#[test]
fn si_accepted() {
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
    test("fi", "SimpleSpeak", expr, 
        "1 litra, pilkku; 2 litraa, pilkku, \
                1 litra, pilkku; 2 litraa, pilkku, \
                1 litra, pilkku; 2 litraa, pilkku, \
                1 tonni, pilkku; 2 tonnia, pilkku, \
                1 dalton, pilkku, 2 daltonia, pilkku, \
                1 neper, pilkku, 2 neperiä, pilkku; \
                1 atomimassayksikkö, pilkku; 2 atomimassayksikköä, pilkku; \
                1 elektronivoltti, pilkku; 2 elektronivolttia, pilkku, \
                1 radiaani, pilkku; 2 radiaania, pilkku; \
                1 steradiaani, pilkku; 2 steradiaania, pilkku, \
                1 annum, pilkku, 2 annumia, pilkku; \
                1 kaarisekunti, pilkku; 2 kaarisekuntia, pilkku, \
                1 bitti, pilkku, 2 bittiä, pilkku, \
                1 tavu, pilkku, 2 tavua, pilkku, \
                1 baudi, pilkku; 2 baudia");
}

#[test]
fn si_accepted_with_prefixes() {
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
    test("fi", "SimpleSpeak", expr, 
        "1 kvetta-litra, pilkku; 2 ronna-litraa, pilkku; \
                1 jotta-litra, pilkku; 2 tsetta-litraa, pilkku; \
                1 eksa-litra, pilkku; 2 peta-litraa, pilkku; \
                1 tera-tonni, pilkku; 2 giga-tonnia, pilkku; \
                1 mega-dalton, pilkku; 2 kilo-daltonia, pilkku; \
                1 desi-neper, pilkku; 2 sentti-neperiä, pilkku; \
                1 hehto-atomimassayksikkö; pilkku; 2 deka-atomimassayksikköä; pilkku; \
                1 milli-elektronivoltti; pilkku; 2 mikro-elektronivolttia; pilkku; \
                1 nano-radiaani, pilkku; 2 piko-radiaania, pilkku; \
                1 femto-steradiaani, pilkku; 2 atto-steradiaania; pilkku; \
                1 giga-annum, pilkku; 2 mega-annumia, pilkku; \
                1 zepto-kaarisekunti, pilkku; 2 jokto-kaarisekuntia; pilkku; \
                1 kilo-bitti, pilkku; 2 mega-bittiä, pilkku; \
                1 giga-tavu, pilkku; 2 tera-tavua, pilkku; \
                1 tera-baudi, pilkku; 2 eksa-baudia");
}

#[test]
fn without_prefix_time() {
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
    test("fi", "SimpleSpeak", expr, 
        "1 sekunti, pilkku; 2 sekuntia, pilkku, \
                1 sekunti, pilkku; 2 sekuntia, pilkku, \
                1 minuutti, pilkku; 2 minuuttia, pilkku, \
                1 minuutti, pilkku; 2 minuuttia, pilkku, \
                1 minuutti, pilkku; 2 minuuttia, pilkku, \
                1 tunti, pilkku; 2 tuntia, pilkku, \
                1 tunti, pilkku; 2 tuntia, pilkku, \
                1 tunti, pilkku; 2 tuntia, pilkku; \
                1 vuorokausi, pilkku; 2 vuorokautta, pilkku; \
                1 vuorokausi, pilkku; 2 vuorokautta, pilkku, \
                1 viikko, pilkku; 2 viikkoa, pilkku, \
                1 viikko, pilkku; 2 viikkoa, pilkku, \
                1 vuosi, pilkku, 2 vuotta, pilkku, \
                1 vuosi, pilkku, 2 vuotta");
}

#[test]
fn without_prefix_angles() {
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
    test("fi", "SimpleSpeak", expr, 
        "1 aste, pilkku, 2 astetta, pilkku, \
                1 aste, pilkku, 2 astetta, pilkku; \
                1 kaariminuutti, pilkku; 2 kaariminuuttia, pilkku; \
                1 kaariminuutti, pilkku; 2 kaariminuuttia, pilkku; \
                1 kaariminuutti, pilkku; 2 kaariminuuttia, pilkku; \
                1 kaariminuutti, pilkku; 2 kaariminuuttia, pilkku; \
                1 kaarisekunti, pilkku; 2 kaarisekuntia, pilkku; \
                1 kaarisekunti, pilkku; 2 kaarisekuntia");
}

#[test]
fn without_prefix_distance() {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">au</mi><mo>,</mo><mn>2</mn><mi intent=":unit">au</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ltyr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ltyr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">pc</mi><mo>,</mo><mn>2</mn><mi intent=":unit">pc</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Å</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Å</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Å</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Å</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">fm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">fm</mi>
    </math>"#;
    test("fi", "SimpleSpeak", expr, 
        "1 astronominen yksikkö, pilkku; 2 astronomista yksikköä, pilkku, \
                1 valovuosi, pilkku; 2 valovuotta, pilkku, \
                1 parsek, pilkku, 2 parsekia, pilkku; \
                1 ångström, pilkku; 2 ångströmiä, pilkku; \
                1 ångström, pilkku; 2 ångströmiä, pilkku, \
                1 fermi, pilkku, 2 fermiä");
}

#[test]
fn without_prefix_other() {
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
        <mn>1</mn><mi intent=":unit">℧</mi><mo>,</mo><mn>2</mn><mi intent=":unit">℧</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dyn</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dyn</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">erg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">erg</mi>
    </math>"#;
    test("fi", "SimpleSpeak", expr, 
        "1 hehtaari, pilkku; 2 hehtaaria, pilkku, \
                1 desibeli, pilkku; 2 desibeliä, pilkku; \
                1 normaali-ilmakehä, pilkku; 2 normaali-ilmakehää, pilkku; \
                1 atomimassayksikkö, pilkku; 2 atomimassayksikköä, pilkku, \
                1 baari, pilkku; 2 baaria, pilkku, \
                1 kalori, pilkku; 2 kaloria, pilkku, \
                1 curie, pilkku, 2 curieta, pilkku, \
                1 gooni, pilkku; 2 goonia, pilkku; \
                1 molaarinen, pilkku; 2 molaarinen, pilkku, \
                1 röntgen, pilkku; 2 röntgeniä, pilkku; \
                1 kierros minuutissa, pilkku; 2 kierrosta minuutissa, pilkku, \
                1 mho, pilkku, 2 mhota, pilkku, \
                1 dyne, pilkku, 2 dyneä, pilkku, \
                1 ergi, pilkku, 2 ergiä");
}

#[test]
fn without_prefix_powers_of_2() {
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
    test("fi", "SimpleSpeak", expr, 
        "1 kibi-bitti, pilkku; 2 kibi-bittiä, pilkku; \
                1 mebi-bitti, pilkku; 2 mebi-bittiä, pilkku; \
                1 gibi-bitti, pilkku; 2 gibi-bittiä, pilkku; \
                1 tebi-bitti, pilkku; 2 tebi-bittiä, pilkku; \
                1 pebi-bitti, pilkku; 2 pebi-bittiä, pilkku; \
                1 eksbi-bitti, pilkku; 2 eksbi-bittiä, pilkku; \
                1 tsebi-bitti, pilkku; 2 tsebi-bittiä, pilkku; \
                1 jobi-bitti, pilkku; 2 jobi-bittiä, pilkku, \
                1 kibi-tavu, pilkku; 2 kibi-tavua, pilkku, \
                1 mebi-tavu, pilkku; 2 mebi-tavua, pilkku, \
                1 gibi-tavu, pilkku; 2 gibi-tavua, pilkku, \
                1 tebi-tavu, pilkku; 2 tebi-tavua, pilkku, \
                1 pebi-tavu, pilkku; 2 pebi-tavua, pilkku; \
                1 eksbi-tavu, pilkku; 2 eksbi-tavua, pilkku; \
                1 tsebi-tavu, pilkku; 2 tsebi-tavua, pilkku, \
                1 jobi-tavu, pilkku; 2 jobi-tavua");
}


#[test]
fn si_other_numbers() {
    let expr = r#"<math><mn>1,0</mn><mi intent=":unit">l</mi><mo>,</mo>
                            <mn>2,0</mn><mo>&#xA0;</mo><mi intent=":unit">m</mi><mo>,</mo>
                            <mi>x</mi><mo>&#xA0;</mo><mi intent=":unit">ms</mi><mo>,</mo>
                            <mi>y</mi><mi intent=":unit">µs</mi><mo>,</mo>
                            <mi intent=":unit">dag</mi><mo>,</mo>
                            <mn>1235</mn><mi intent=":unit">daN</mi><mo>,</mo>
                            <mn>2,5</mn><mi intent=":unit">&#xB5;sec</mi><mo>,</mo>
                            <mn>32,34</mn><mi intent=":unit">mol</mi></math>"#;
    test("fi", "SimpleSpeak", expr, 
        "1,0 litraa, pilkku, 2,0 metriä, pilkku; x milli-sekuntia, pilkku; y mikro-sekuntia, pilkku, \
                deka-grammaa, pilkku; 1235 deka-newtonia, pilkku; 2,5 mikro-sekuntia; pilkku; 32,34 moolia");
}


#[test]
fn test_mtext_inference() {
    let expr = r#"<math><mo>[</mo>
                <mn>1</mn><mtext>t</mtext><mo>,</mo>
                <mn>2</mn><mtext>PA</mtext><mo>,</mo>
                <mn>3</mn><mtext>Pa</mtext><mo>,</mo>
                <mn>4,5</mn><mtext>mT</mtext>
            <mo>]</mo></math>"#;
    test("fi", "SimpleSpeak", expr, 
        "auki hakasulku; 1 tonni, pilkku; 2 peta-ampeeria, pilkku, \
                3 pascalia, pilkku; 4,5 milli-teslaa; kiinni hakasulku");
}
