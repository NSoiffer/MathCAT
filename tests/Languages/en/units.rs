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
    test("en", "SimpleSpeak", expr, 
        "quetta-grams, comma, \
                ronna-grams, comma, \
                yotta-grams, comma, \
                zetta-grams, comma, \
                exa-grams, comma, \
                peta-grams, comma, \
                tera-grams, comma, \
                giga-grams, comma, \
                mega-grams, comma, \
                kilo-grams, comma, \
                hecto-grams, comma, \
                deka-grams, comma, \
                deci-grams, comma, \
                centi-grams, comma, \
                milli-grams, comma, \
                micro-grams, comma, \
                nano-grams, comma, \
                pico-grams, comma, \
                femto-grams, comma, \
                atto-grams, comma, \
                zepto-grams, comma, \
                yocto-grams, comma, \
                ronto-grams, comma, \
                quecto-grams");
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
    test("en", "SimpleSpeak", expr, 
        "1 amp, comma, 2 amps, comma, \
                1 candela, comma; 2 candelas, comma, \
                1 kelvin, comma, 2 kelvins, comma, \
                1 kelvin, comma, 2 kelvins, comma, \
                1 gram, comma, 2 grams, comma, \
                1 metre, comma, 2 metres, comma, \
                1 mole, comma, 2 moles, comma, \
                1 second, comma, 2 seconds, comma, \
                1 second, comma, 2 seconds, comma, \
                1 second, comma, 2 seconds, comma, \
                1 second, comma, 2 seconds");
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
    test("en", "SimpleSpeak", expr, 
        "1 quetta-amp, comma; 2 ronna-amps, comma; \
                1 yotta-candela, comma; 2 zetta-candelas, comma; \
                1 exa-kelvin, comma; 2 peta-kelvins, comma; \
                1 tera-kelvin, comma; 2 giga-kelvins, comma; \
                1 mega-gram, comma; 2 kilo-grams, comma; \
                1 hecto-metre, comma; 2 deka-metres, comma; \
                1 deci-mole, comma; 2 centi-moles, comma; \
                1 milli-second, comma; 2 micro-seconds, comma; \
                1 nano-second, comma; 2 pico-seconds");
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
    test("en", "SimpleSpeak", expr, 
        "1 becquerel, comma; 2 becquerels, comma, \
                1 coulomb, comma; 2 coulombs, comma; \
                1 degree celsius, comma; 2 degrees celsius, comma; \
                1 degree celsius, comma; 2 degrees celsius, comma, \
                1 farad, comma, 2 farads, comma, \
                1 gray, comma, 2 grays, comma, \
                1 henry, comma, 2 henrys, comma, \
                1 hertz, comma, 2 hertz, comma, \
                1 joule, comma, 2 joules, comma, \
                1 kattel, comma, 2 kattels, comma, \
                1 lumen, comma, 2 lumens, comma, \
                1 lux, comma, 2 luxs");
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
    test("en", "SimpleSpeak", expr, 
        "1 quetta-becquerel, comma; 2 ronna-becquerels; comma; \
                1 yotta-coulomb, comma; 2 zetta-coulombs, comma; \
                1 exa-farad, comma; 2 peta-farads, comma; \
                1 tera-gray, comma; 2 giga-grays, comma; \
                1 mega-henry, comma; 2 kilo-henrys, comma; \
                1 deka-hertz, comma; 2 deci-hertz, comma; \
                1 centi-joule, comma; 2 milli-joules, comma; \
                1 micro-kattel, comma; 2 nano-kattels, comma; \
                1 pico-lumen, comma; 2 femto-lumens, comma; \
                1 atto-lux, comma; 2 zepto-luxs, comma; \
                1 milli-degree celsius; comma; 2 micro-degrees celsius; comma; \
                1 pico-degree celsius; comma; 2 nano-degrees celsius");
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
    test("en", "SimpleSpeak", expr, 
        "1 newton, comma, 2 newtons, comma, \
                1 ohm, comma, 2 ohms, comma, \
                1 ohm, comma, 2 ohms, comma, \
                1 pascal, comma, 2 pascals, comma, \
                1 siemens, comma, 2 siemens, comma, \
                1 sievert, comma; 2 sieverts, comma, \
                1 tesla, comma, 2 teslas, comma, \
                1 volt, comma, 2 volts, comma, \
                1 watt, comma, 2 watts, comma, \
                1 weber, comma, 2 webers");
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
    test("en", "SimpleSpeak", expr, 
        "1 quecto-newton, comma; 2 ronto-newtons, comma; \
                1 yocto-ohm, comma; 2 zepto-ohms, comma; \
                1 atto-ohm, comma; 2 femto-ohms, comma; \
                1 pico-pascal, comma; 2 nano-pascals, comma; \
                1 micro-siemens, comma; 2 milli-siemens, comma; \
                1 centi-sievert, comma; 2 deci-sieverts, comma; \
                1 deka-tesla, comma; 2 hecto-teslas, comma; \
                1 kilo-volt, comma; 2 mega-volts, comma; \
                1 giga-watt, comma; 2 tera-watts, comma; \
                1 peta-weber, comma; 2 exa-webers");
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
    test("en", "SimpleSpeak", expr, 
        "1 litre, comma, 2 litres, comma, \
                1 litre, comma, 2 litres, comma, \
                1 litre, comma, 2 litres, comma, \
                1 metric ton, comma; 2 metric tons, comma, \
                1 dalton, comma, 2 daltons, comma, \
                1 neper, comma, 2 nepers, comma; \
                1 atomic mass unit, comma; 2 atomic mass units, comma; \
                1 electronvolt, comma; 2 electronvolts, comma, \
                1 radian, comma, 2 radians, comma, \
                1 sterradion, comma; 2 sterradions, comma, \
                1 annum, comma, 2 annums, comma, \
                1 arcsecond, comma; 2 arcseconds, comma, \
                1 bit, comma, 2 bits, comma, \
                1 byte, comma, 2 bytes, comma, \
                1 baud, comma, 2 bauds");
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
    test("en", "SimpleSpeak", expr, 
        "1 quetta-litre, comma; 2 ronna-litres, comma; \
                1 yotta-litre, comma; 2 zetta-litres, comma; \
                1 exa-litre, comma; 2 peta-litres, comma; \
                1 tera-metric ton, comma; 2 giga-metric tons; comma; \
                1 mega-dalton, comma; 2 kilo-daltons, comma; \
                1 deci-neper, comma; 2 centi-nepers, comma; \
                1 hecto-atomic mass unit; comma; 2 deka-atomic mass units; comma; \
                1 milli-electronvolt, comma; 2 micro-electronvolts; comma; \
                1 nano-radian, comma; 2 pico-radians, comma; \
                1 femto-sterradion, comma; 2 atto-sterradions; comma; \
                1 giga-annum, comma; 2 mega-annums, comma; \
                1 zepto-arcsecond, comma; 2 yocto-arcseconds; comma; \
                1 kilo-bit, comma; 2 mega-bits, comma; \
                1 giga-byte, comma; 2 tera-bytes, comma; \
                1 tera-baud, comma; 2 exa-bauds");
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
    test("en", "SimpleSpeak", expr, 
        "1 second, comma, 2 seconds, comma, \
                1 second, comma, 2 seconds, comma, \
                1 minute, comma, 2 minutes, comma, \
                1 minute, comma, 2 minutes, comma, \
                1 minute, comma, 2 minutes, comma, \
                1 hour, comma, 2 hours, comma, \
                1 hour, comma, 2 hours, comma, \
                1 hour, comma, 2 hours, comma, \
                1 day, comma, 2 days, comma, \
                1 day, comma, 2 days, comma, \
                1 week, comma, 2 weeks, comma, \
                1 week, comma, 2 weeks, comma, \
                1 year, comma, 2 years, comma, \
                1 year, comma, 2 years");
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
    test("en", "SimpleSpeak", expr, 
        "1 degree, comma, 2 degrees, comma, \
                1 degree, comma, 2 degrees, comma, \
                1 arcminute, comma; 2 arcminutes, comma, \
                1 arcminute, comma; 2 arcminutes, comma, \
                1 arcminute, comma; 2 arcminutes, comma, \
                1 arcminute, comma; 2 arcminutes, comma, \
                1 arcsecond, comma; 2 arcseconds, comma, \
                1 arcsecond, comma; 2 arcseconds");
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
    test("en", "SimpleSpeak", expr, 
        "1 astronomical unit, comma; 2 astronomical units, comma, \
                1 light year, comma; 2 light years, comma, \
                1 parsec, comma, 2 parsecs, comma, \
                1 angstrom, comma; 2 angstroms, comma, \
                1 angstrom, comma; 2 angstroms, comma, \
                1 fermi, comma, 2 fermis");
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
    test("en", "SimpleSpeak", expr, 
        "1 hectare, comma; 2 hectares, comma, \
                1 decibel, comma; 2 decibels, comma, \
                1 atmosphere, comma; 2 atmospheres, comma; \
                1 atomic mass unit, comma; 2 atomic mass units, comma, \
                1 bar, comma, 2 bars, comma, \
                1 calorie, comma; 2 calories, comma, \
                1 curie, comma, 2 curies, comma, \
                1 gradian, comma; 2 gradians, comma, \
                1 molar, comma, 2 molars, comma, \
                1 roentgen, comma; 2 roentgens, comma; \
                1 revolution per minute, comma; 2 revolutions per minute, comma, \
                1 m-h-o, comma, 2 m-h-os, comma, \
                1 dyne, comma, 2 dynes, comma, \
                1 erg, comma, 2 ergs");
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
    test("en", "SimpleSpeak", expr, 
        "1 kibi-bit, comma; 2 kibi-bits, comma, \
                1 mebi-bit, comma; 2 mebi-bits, comma, \
                1 gibi-bit, comma; 2 gibi-bits, comma, \
                1 tebi-bit, comma; 2 tebi-bits, comma, \
                1 pebi-bit, comma; 2 pebi-bits, comma, \
                1 exbi-bit, comma; 2 exbi-bits, comma, \
                1 zebi-bit, comma; 2 zebi-bits, comma, \
                1 yobi-bit, comma; 2 yobi-bits, comma, \
                1 kibi-byte, comma; 2 kibi-bytes, comma, \
                1 mebi-byte, comma; 2 mebi-bytes, comma, \
                1 gibi-byte, comma; 2 gibi-bytes, comma, \
                1 tebi-byte, comma; 2 tebi-bytes, comma, \
                1 pebi-byte, comma; 2 pebi-bytes, comma, \
                1 exbi-byte, comma; 2 exbi-bytes, comma, \
                1 zebi-byte, comma; 2 zebi-bytes, comma, \
                1 yobi-byte, comma; 2 yobi-bytes");
}


#[test]
fn si_other_numbers() {
    let expr = r#"<math><mn>1.0</mn><mi intent=":unit">l</mi><mo>,</mo>
                            <mn>2.0</mn><mo>&#xA0;</mo><mi intent=":unit">m</mi><mo>,</mo>
                            <mi>x</mi><mo>&#xA0;</mo><mi intent=":unit">ms</mi><mo>,</mo>
                            <mi>y</mi><mi intent=":unit">µs</mi><mo>,</mo>
                            <mi intent=":unit">dag</mi><mo>,</mo>
                            <mn>1235</mn><mi intent=":unit">daN</mi><mo>,</mo>
                            <mn>2.5</mn><mi intent=":unit">&#xB5;sec</mi><mo>,</mo>
                            <mn>32.34</mn><mi intent=":unit">mol</mi></math>"#;
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Terse")], expr,
            "1.0 l comma, 2.0 m comma; x milli-seconds, comma; y micro-seconds, comma, \
                    deka-grams, comma; 1235 deka-newtons; comma; 2.5 micro-seconds; comma; 32.34 moles");
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
            "1.0 litre, comma; 2.0 metres, comma; x milli-seconds, comma; y micro-seconds, comma, \
                    deka-grams, comma; 1235 deka-newtons; comma; 2.5 micro-seconds; comma; 32.34 moles");
    test_prefs("en", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "1.0 litre, comma; 2.0 metres, comma; x milli-seconds, comma; y micro-seconds, comma, \
                    deka-grams, comma; 1235 deka-newtons; comma; 2.5 micro-seconds; comma; 32.34 moles");
}


#[test]
fn test_mtext_inference() {
    let expr = r#"<math><mo>[</mo>
                <mn>1</mn><mtext>t</mtext><mo>,</mo>
                <mn>2</mn><mtext>PA</mtext><mo>,</mo>
                <mn>3</mn><mtext>Pa</mtext><mo>,</mo>
                <mn>4.5</mn><mtext>mT</mtext>
            <mo>]</mo></math>"#;
    test("en", "SimpleSpeak", expr, 
        "open bracket; 1 metric ton, comma; 2 peta-amps, comma, \
                3 pascals, comma; 4.5 milli-teslas; close bracket");
}

    #[test]
    fn infer_unit() {
        let expr = r#"<math>
            <mn>3</mn><mi mathvariant="normal">m</mi><mo>,</mo>
            <mn>1</mn><mi>km</mi><mo>,</mo>
            <mn>3</mn><mtext>m</mtext><mo>,</mo>
            <mfrac><mn>3</mn><mn>10</mn></mfrac><mi mathvariant="normal">F</mi><mo>,</mo>
            <msub><mi>m</mi><mi>min</mi></msub>
            </math>"#;
        test("en", "SimpleSpeak", expr, 
            "3 metres, comma; 1 kilo-metre, comma, 3 metres, comma; 3 tenths farads, comma; m sub min end sub");
    }
