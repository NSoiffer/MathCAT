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
    test("sv", "SimpleSpeak", expr, 
        "quetta-gram, komma, \
                ronna-gram komma, \
                yotta-gram komma, \
                zetta-gram komma, \
                exa-gram komma, \
                peta-gram komma, \
                tera-gram komma, \
                giga-gram komma, \
                mega-gram komma, \
                kilo-gram komma, \
                hekto-gram komma, \
                deka-gram komma, \
                deci-gram komma, \
                centi-gram komma, \
                milli-gram komma, \
                mikro-gram komma, \
                nano-gram komma, \
                piko-gram komma, \
                femto-gram komma, \
                atto-gram komma, \
                zepto-gram komma, \
                yokto-gram komma, \
                ronto-gram komma, \
                quekto-gram");
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
        <mn>1</mn><mi intent=":unit">sek</mi><mo>,</mo><mn>2</mn><mi intent=":unit">sek</mi>
    </math>"#;
    test("sv", "SimpleSpeak", expr, 
        "1 ampère, komma, 2 ampère, komma, \
                1 candela, komma, 2 candela, komma, \
                1 kelvin, komma, 2 kelvin, komma, \
                1 kelvin, komma, 2 kelvin, komma, \
                ett gram, komma, 2 gram, komma, \
                1 meter, komma, 2 meter, komma, \
                1 mol, komma, 2 mol, komma, \
                1 sekund, komma, 2 sekunder, komma, \
                1 sekund, komma, 2 sekunder, komma, \
                1 sekund, komma, 2 sekunder, komma, \
                1 sekund, komma, 2 sekunder");
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
        <mn>1</mn><mi intent=":unit">nsek</mi><mo>,</mo><mn>2</mn><mi intent=":unit">psek</mi>
    </math>"#;
    test("sv", "SimpleSpeak", expr, 
        "1 quetta-ampère, komma; 2 ronna-ampère, komma; \
                1 yotta-candela, komma; 2 zetta-candela, komma; \
                1 exa-kelvin, komma; 2 peta-kelvin, komma; \
                1 tera-kelvin, komma; 2 giga-kelvin, komma; \
                ett mega-gram, komma; 2 kilo-gram, komma; \
                1 hekto-meter, komma; 2 deka-meter, komma; \
                1 deci-mol, komma; 2 centi-mol, komma; \
                1 milli-sekund, komma; 2 mikro-sekunder, komma; \
                1 nano-sekund, komma; 2 piko-sekunder");
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
    test("sv", "SimpleSpeak", expr, 
        "1 becquerel, komma, 2 becquerel, komma, \
                1 coulomb, komma, 2 coulomb, komma; \
                1 grad celsius, komma; 2 grader celsius, komma; \
                1 grad celsius, komma; 2 grader celsius, komma, \
                1 farad, komma, 2 farad, komma, \
                1 gray, komma, 2 gray, komma, \
                1 henry, komma, 2 henry, komma, \
                1 hertz, komma, 2 hertz, komma, \
                1 joule, komma, 2 joule, komma, \
                1 katal, komma, 2 katal, komma, \
                1 lumen, komma, 2 lumen, komma, \
                1 lux, komma, 2 lux");
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
    test("sv", "SimpleSpeak", expr, 
        "1 quetta-becquerel, komma; 2 ronna-becquerel, komma; \
                1 yotta-coulomb, komma; 2 zetta-coulomb, komma; \
                1 exa-farad, komma; 2 peta-farad, komma; \
                1 tera-gray, komma; 2 giga-gray, komma; \
                1 mega-henry, komma; 2 kilo-henry, komma; \
                1 deka-hertz, komma; 2 deci-hertz, komma; \
                1 centi-joule, komma; 2 milli-joule, komma; \
                1 mikro-katal, komma; 2 nano-katal, komma; \
                1 piko-lumen, komma; 2 femto-lumen, komma; \
                1 atto-lux, komma; 2 zepto-lux, komma; \
                1 milli-grad celsius, komma; 2 mikro-grader celsius; komma; \
                1 piko-grad celsius, komma; 2 nano-grader celsius");
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
    test("sv", "SimpleSpeak", expr, 
        "1 newton, komma, 2 newton, komma, \
                1 ohm, komma, 2 ohm, komma, \
                1 ohm, komma, 2 ohm, komma, \
                1 pascal, komma, 2 pascal, komma, \
                1 siemens, komma, 2 siemens, komma, \
                1 sievert, komma, 2 sievert, komma, \
                1 tesla, komma, 2 tesla, komma, \
                1 volt, komma, 2 volt, komma, \
                1 watt, komma, 2 watt, komma, \
                1 weber, komma, 2 weber");
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
    test("sv", "SimpleSpeak", expr, 
        "1 quekto-newton, komma; 2 ronto-newton, komma; \
                1 yokto-ohm, komma; 2 zepto-ohm, komma; \
                1 atto-ohm, komma; 2 femto-ohm, komma; \
                1 piko-pascal, komma; 2 nano-pascal, komma; \
                1 mikro-siemens, komma; 2 milli-siemens, komma; \
                1 centi-sievert, komma; 2 deci-sievert, komma; \
                1 deka-tesla, komma; 2 hekto-tesla, komma; \
                1 kilo-volt, komma; 2 mega-volt, komma; \
                1 giga-watt, komma; 2 tera-watt, komma; \
                1 peta-weber, komma; 2 exa-weber");
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
    test("sv", "SimpleSpeak", expr, 
        "1 liter, komma, 2 liter, komma, \
                1 liter, komma, 2 liter, komma, \
                1 liter, komma, 2 liter, komma, \
                ett tonn, komma, 2 tonn, komma, \
                1 dalton, komma, 2 dalton, komma, \
                1 neper, komma, 2 neper, komma; \
                1 atommassenhet, komma; 2 atommassenheter, komma; \
                1 elektronvolt, komma; 2 elektronvolt, komma, \
                1 radian, komma, 2 radianer, komma, \
                1 steradian, komma, 2 steradianer, komma, \
                ett annum, komma, 2 annum, komma, \
                1 bågsekund, komma; 2 bågsekunder, komma, \
                1 bit, komma, 2 bits, komma, \
                1 byte, komma, 2 bytes, komma, \
                1 baud, komma, 2 bauds");
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
    test("sv", "SimpleSpeak", expr, 
        "1 quetta-liter, komma; 2 ronna-liter, komma; \
                1 yotta-liter, komma; 2 zetta-liter, komma; \
                1 exa-liter, komma; 2 peta-liter, komma; \
                ett tera-tonn, komma; 2 giga-tonn, komma; \
                1 mega-dalton, komma; 2 kilo-dalton, komma; \
                1 deci-neper, komma; 2 centi-neper, komma; \
                1 hekto-atommassenhet; komma; 2 deka-atommassenheter; komma; \
                1 milli-elektronvolt, komma; 2 mikro-elektronvolt, komma; \
                1 nano-radian, komma; 2 piko-radianer, komma; \
                1 femto-steradian, komma; 2 atto-steradianer, komma; \
                ett giga-annum, komma; 2 mega-annum, komma; \
                1 zepto-bågsekund, komma; 2 yokto-bågsekunder, komma; \
                1 kilo-bit, komma; 2 mega-bits, komma; \
                1 giga-byte, komma; 2 tera-bytes, komma; \
                1 tera-baud, komma; 2 exa-bauds");
}

#[test]
fn without_prefix_timme() {
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
    test("sv", "SimpleSpeak", expr, 
        "1 sekund, komma, 2 sekunder, komma, \
                1 sekund, komma, 2 sekunder, komma, \
                1 minut, komma, 2 minuter, komma, \
                1 minut, komma, 2 minuter, komma, \
                1 minut, komma, 2 minuter, komma, \
                1 timme, komma, 2 timmar, komma, \
                1 timme, komma, 2 timmar, komma, \
                1 timme, komma, 2 timmar, komma, \
                1 dag, komma, 2 dagar, komma, \
                1 dag, komma, 2 dagar, komma, \
                1 vecka, komma, 2 veckor, komma, \
                1 vecka, komma, 2 veckor, komma, \
                ett år, komma, 2 år, komma, \
                ett år, komma, 2 år");
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
    test("sv", "SimpleSpeak", expr, 
        "1 grad, komma, 2 grader, komma, \
                1 grad, komma, 2 grader, komma, \
                1 bågminut, komma, 2 bågminuter, komma, \
                1 bågminut, komma, 2 bågminuter, komma, \
                1 bågminut, komma, 2 bågminuter, komma, \
                1 bågminut, komma, 2 bågminuter, komma, \
                1 bågsekund, komma; 2 bågsekunder, komma, \
                1 bågsekund, komma; 2 bågsekunder");
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
    test("sv", "SimpleSpeak", expr, 
        "1 astronomisk enhet, komma; 2 astronomiska enheter, komma, \
                ett ljusår, komma, 2 ljusår, komma, \
                1 parsek, komma, 2 parsek, komma, \
                1 ångström, komma, 2 ångström, komma, \
                1 ångström, komma, 2 ångström, komma, \
                1 fermi, komma, 2 fermi");
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
    test("sv", "SimpleSpeak", expr, 
        "1 hektar, komma, 2 hektar, komma, \
                1 decibel, komma, 2 decibel, komma, \
                1 atmosfär, komma, 2 atmosfärer, komma; \
                1 atommassenhet, komma; 2 atommassenheter, komma, \
                1 bar, komma, 2 bar, komma, \
                1 kalori, komma, 2 kalorier, komma, \
                1 curie, komma, 2 curie, komma, \
                1 gon, komma, 2 gon, komma, \
                1 molar, komma, 2 molar, komma, \
                1 röntgen, komma, 2 röntgen, komma; \
                ett varv per minut, komma; 2 varv per minut, komma, \
                1 m-h-o, komma, 2 m-h-o, komma, \
                1 dyn, komma, 2 dyn, komma, \
                1 erg, komma, 2 erg");
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
    test("sv", "SimpleSpeak", expr, 
        "1 kibi-bit, komma, 2 kibi-bits, komma, \
                1 mebi-bit, komma, 2 mebi-bits, komma, \
                1 gibi-bit, komma, 2 gibi-bits, komma, \
                1 tebi-bit, komma, 2 tebi-bits, komma, \
                1 pebi-bit, komma, 2 pebi-bits, komma, \
                1 exbi-bit, komma, 2 exbi-bits, komma, \
                1 zebi-bit, komma, 2 zebi-bits, komma, \
                1 yobi-bit, komma, 2 yobi-bits, komma, \
                1 kibi-byte, komma, 2 kibi-bytes, komma, \
                1 mebi-byte, komma, 2 mebi-bytes, komma, \
                1 gibi-byte, komma, 2 gibi-bytes, komma, \
                1 tebi-byte, komma, 2 tebi-bytes, komma, \
                1 pebi-byte, komma, 2 pebi-bytes, komma, \
                1 exbi-byte, komma, 2 exbi-bytes, komma, \
                1 zebi-byte, komma, 2 zebi-bytes, komma, \
                1 yobi-byte, komma, 2 yobi-bytes");
}


#[test]
fn si_other_numbers() {
    let expr = r#"<math><mn>1,0</mn><mi intent=":unit">l</mi><mo>,</mo>
                            <mn>2,0</mn><mo>&#xA0;</mo><mi intent=":unit">m</mi><mo>,</mo>
                            <mi>x</mi><mo>&#xA0;</mo><mi intent=":unit">ms</mi><mo>,</mo>
                            <mi>y</mi><mi intent=":unit">µs</mi><mo>,</mo>
                            <mi intent=":unit">dag</mi><mo>,</mo>
                            <mn>1235</mn><mi intent=":unit">daN</mi><mo>,</mo>
                            <mn>2,5</mn><mi intent=":unit">&#xB5;sek</mi><mo>,</mo>
                            <mn>32,34</mn><mi intent=":unit">mol</mi></math>"#;
    test("sv", "SimpleSpeak", expr, 
        "1,0 liter, komma, 2,0 meter, komma; x milli-sekunder, komma; y mikro-sekunder, komma, \
                deka-gram komma; 1235 deka-newton, komma; 2,5 mikro-sekunder, komma, 32,34 mol");
}


#[test]
fn test_mtext_inference() {
    let expr = r#"<math><mo>[</mo>
                <mn>1</mn><mtext>t</mtext><mo>,</mo>
                <mn>2</mn><mtext>PA</mtext><mo>,</mo>
                <mn>3</mn><mtext>Pa</mtext><mo>,</mo>
                <mn>4,5</mn><mtext>mT</mtext>
            <mo>]</mo></math>"#;
    test("sv", "SimpleSpeak", expr, 
        "start hak-parentes; ett tonn, komma; 2 peta-ampère, komma, 3 pascal, komma; 4,5 milli-tesla; slut hak-parentes");
}
/// Tests for fractions followed by units

#[test]
fn one_neuter_unit() {
    let expr = r#"<math>
    <mn>1</mn><mi mathvariant="normal" intent=":unit">min</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "1 minut");
    test("sv", "SimpleSpeak", expr, "1 minut");
}


#[test]
fn one_masculine_feminine_unit() {
    let expr = r#"<math>
    <mn>1</mn><mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "1 timme");
    test("sv", "SimpleSpeak", expr, "1 timme");
}


#[test]
fn half_neuter_unit() {
    let expr = r#"<math>
    <mfrac>
        <mn>1</mn><mn>2</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">sek</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "1 halv sekund");
    test("sv", "SimpleSpeak", expr, "1 halv sekund");
    
}


#[test]
fn half_masculine_feminine_unit() {
    let expr = r#"<math>
    <mfrac>
        <mn>1</mn><mn>2</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">l</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "1 halv liter");
    test("sv", "SimpleSpeak", expr, "1 halv liter");
}


#[test]
fn one_third_neuter_unit() {
    let expr = r#"<math>
    <mfrac>
        <mn>1</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">as</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "1 tredjedels bågsekund");
    test("sv", "SimpleSpeak", expr, "1 tredjedels bågsekund");
}


#[test]
fn one_third_masculine_feminine_unit() {
    let expr = r#"<math>
    <mfrac>
        <mn>1</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">m</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "1 tredjedels meter");
    test("sv", "SimpleSpeak", expr, "1 tredjedels meter");
}


#[test]
fn five_thirds_unit() {
    let expr = r#"<math>
    <mfrac>
        <mn>5</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "5 tredjedels timme");
    test("sv", "SimpleSpeak", expr, "5 tredjedels timme");
}


#[test]
fn four_eighths_unit() {
    let expr = r#"<math>
    <mfrac>
        <mn>4</mn><mn>8</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">g</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "4 åttondels gram");
    test("sv", "SimpleSpeak", expr, "4 åttondels gram");
}


#[test]
fn six_ninths_unit() {
    let expr = r#"<math>
    <mfrac>
        <mn>6</mn><mn>9</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "6 niondels timme");
    test("sv", "SimpleSpeak", expr, "6 niondels timme");
}


#[test]
fn mixed_fraction__half_neuter_unit() {
    let expr = r#"<math>
    <mn>3</mn>
    <mfrac>
        <mn>1</mn><mn>2</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">min</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "3 och 1 halv, minut");
    test("sv", "SimpleSpeak", expr, "3 och 1 halv, minut");
}


#[test]
fn mixed_fraction__half_masculine_feminine_unit() {
    let expr = r#"<math>
    <mn>3</mn>
    <mfrac>
        <mn>1</mn><mn>2</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "3 och 1 halv, timme");
    test("sv", "SimpleSpeak", expr, "3 och 1 halv, timme");
}


#[test]
fn mixed_fractions_third_neuter_unit() {
    let expr = r#"<math>
    <mn>1</mn>
    <mfrac>
        <mn>1</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">min</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "1 och 1 tredjedels, minut");
    test("sv", "SimpleSpeak", expr, "1 och 1 tredjedels, minut");
}


#[test]
fn mixed_fractions_third_masculine_feminine_unit() {
    let expr = r#"<math>
    <mn>1</mn>
    <mfrac>
        <mn>1</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "1 och 1 tredjedels, timme");
    test("sv", "SimpleSpeak", expr, "1 och 1 tredjedels, timme");
}

#[test]
fn mixed_fractions_two_and_two_thirds_unit() {
    let expr = r#"<math>
    <mn>2</mn>
    <mfrac>
        <mn>2</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "2 och 2 tredjedels, timme");
    test("sv", "SimpleSpeak", expr, "2 och 2 tredjedels, timme");
}

#[test]
fn mixed_fractions_four_and_five_sevenths_unit() {
    let expr = r#"<math>
    <mn>4</mn>
    <mfrac>
        <mn>5</mn><mn>7</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">min</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "4 och 5 sjundedels, minut");
    test("sv", "SimpleSpeak", expr, "4 och 5 sjundedels, minut");
}


#[test]
fn fraction_with_units_neuter_unit() {
    let expr = r#"<math>
    <mfrac><mn>1</mn><mn>2</mn></mfrac>
    <mfrac><mi mathvariant="normal" intent=":unit">g</mi><mi mathvariant="normal" intent=":unit">hr</mi></mfrac>
</math>"#;
    test("sv", "ClearSpeak", expr, "ett halvt gram per timme");
    test("sv", "SimpleSpeak", expr, "ett halvt gram per timme");
}


#[test]
fn fraction_with_units_masculine_feminine_unit() {
    let expr = r#"<math>
    <mfrac><mn>1</mn><mn>2</mn></mfrac>
    <mfrac><mi mathvariant="normal" intent=":unit">m</mi><mi mathvariant="normal" intent=":unit">sek</mi></mfrac>
</math>"#;
    test("sv", "ClearSpeak", expr, "1 halv meter per sekund");
    test("sv", "SimpleSpeak", expr, "1 halv meter per sekund");
}

#[test]
fn fraction_with_units_2_neuter_unit() {
    let expr = r#"<math>
    <mfrac>
        <mrow><mn>1</mn><mi mathvariant="normal" intent=":unit">g</mi></mrow>
        <mrow><mn>3</mn><mi mathvariant="normal" intent=":unit">hr</mi></mrow>
    </mfrac>
</math>"#;
    test("sv", "ClearSpeak", expr, "division med täljaren ett gram; och nämnaren 3 timmar");
    test("sv", "SimpleSpeak", expr, "division, ett gram, genom, 3 timmar, slut division");
}


#[test]
fn fraction_with_units_2_masculine_feminine_unit() {
    let expr = r#"<math>
    <mfrac>
        <mrow><mn>1</mn><mi mathvariant="normal" intent=":unit">m</mi></mrow>
        <mrow><mn>3</mn><mi mathvariant="normal" intent=":unit">sek</mi></mrow>
    </mfrac>
</math>"#;
    test("sv", "ClearSpeak", expr, "division med täljaren 1 meter; och nämnaren 3 sekunder");
    test("sv", "SimpleSpeak", expr, "division, 1 meter, genom, 3 sekunder, slut division");
}


#[test]
fn fraction_not_ordinal_units_neuter() {
    let expr = r#"<math>
    <mfrac><mn>3</mn><mn>11</mn></mfrac><mi mathvariant="normal" intent=":unit">min</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, ", bråk, 3 genom 11, slut bråk; minuter");
    test("sv", "SimpleSpeak", expr, "3 genom 11, minuter");
}


#[test]
fn fraction_not_ordinal_units_masculine_feminine() {
    let expr = r#"<math>
    <mfrac><mn>3</mn><mn>11</mn></mfrac><mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, ", bråk, 3 genom 11, slut bråk; timmar");
    test("sv", "SimpleSpeak", expr, "3 genom 11, timmar");
}

#[test]
fn fraction_not_ordinal_2_units_neuter() {
    let expr = r#"<math>
    <mfrac><mn>7</mn><mn>21</mn></mfrac><mi mathvariant="normal" intent=":unit">min</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, ", bråk, 7 genom 21, slut bråk; minuter");
    test("sv", "SimpleSpeak", expr, "7 genom 21, minuter");
}


#[test]
fn fraction_not_ordinal_2_units_masculine_feminine() {
    let expr = r#"<math>
    <mfrac><mn>7</mn><mn>21</mn></mfrac><mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, ", bråk, 7 genom 21, slut bråk; timmar");
    test("sv", "SimpleSpeak", expr, "7 genom 21, timmar");
}

/// Fractions followed by units with prefix. 

#[test]
fn one_neuter_unit_prefix() {
    let expr = r#"<math>
    <mn>1</mn><mi mathvariant="normal" intent=":unit">mg</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "ett milli-gram");
    test("sv", "SimpleSpeak", expr, "ett milli-gram");
}


#[test]
fn one_masculine_feminine_unit_prefix() {
    let expr = r#"<math>
    <mn>1</mn><mi mathvariant="normal" intent=":unit">µm</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "1 mikro-meter");
    test("sv", "SimpleSpeak", expr, "1 mikro-meter");
}


#[test]
fn half_neuter_unit_prefix() {
    let expr = r#"<math>
    <mfrac>
        <mn>1</mn><mn>2</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">nsek</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "1 halv nano-sekund");
    test("sv", "SimpleSpeak", expr, "1 halv nano-sekund");
}


#[test]
fn third_neuter_unit_prefix() {
    let expr = r#"<math>
    <mfrac>
        <mn>1</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">mt</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "ett tredjedels milli-tonn");
    test("sv", "SimpleSpeak", expr, "ett tredjedels milli-tonn");
}

#[test]
fn mixed_fractions_third_masculine_feminine_unit_prefix() {
    let expr = r#"<math>
    <mn>1</mn>
    <mfrac>
        <mn>1</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">dN</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "1 och 1 tredjedels, deci-newton");
    test("sv", "SimpleSpeak", expr, "1 och 1 tredjedels, deci-newton");
}


#[test]
fn mixed_fractions_four_and_five_sevenths_unit_prefix() {
    let expr = r#"<math>
    <mn>4</mn>
    <mfrac>
        <mn>5</mn><mn>7</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">nas</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, "4 och 5 sjundedels, nano-bågsekund");
    test("sv", "SimpleSpeak", expr, "4 och 5 sjundedels, nano-bågsekund");
}


#[test]
fn fraction_not_ordinal_units_neuter_prefix() {
    let expr = r#"<math>
    <mfrac><mn>7</mn><mn>21</mn></mfrac><mi mathvariant="normal" intent=":unit">nsek</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, ", bråk, 7 genom 21, slut bråk; nano-sekunder");
    test("sv", "SimpleSpeak", expr, "7 genom 21, nano-sekunder");
}


#[test]
fn fraction_not_ordinal_units_masculine_feminine_prefix() {
    let expr = r#"<math>
    <mfrac><mn>7</mn><mn>21</mn></mfrac><mi mathvariant="normal" intent=":unit">ml</mi>
</math>"#;
    test("sv", "ClearSpeak", expr, ", bråk, 7 genom 21, slut bråk; milli-liter");
    test("sv", "SimpleSpeak", expr, "7 genom 21, milli-liter");
}


///Some more complicated fractions with units

#[test]
fn acceleration_half() {
    let expr = r#"<math>
    <mfrac><mn>1</mn><mn>2</mn></mfrac>
    <mfrac><mi mathvariant="normal" intent=":unit">m</mi><msup><mi mathvariant="normal" intent=":unit">s</mi><mn>2</mn></msup></mfrac>
</math>"#;
    test("sv", "ClearSpeak", expr, "1 halv meter per sekund kvadrat");
    test("sv", "SimpleSpeak", expr, "1 halv meter per sekund kvadrat");
}


#[test]
fn acceleration_half_one_fraction() {
    let expr = r#"<math>
    <mfrac>
    <mrow><mn>1</mn><mi mathvariant="normal" intent=":unit">m</mi></mrow>
    <mrow><mn>2</mn><msup><mi mathvariant="normal" intent=":unit">s</mi><mn>2</mn></msup></mrow>
    </mfrac>
</math>"#;
    test("sv", "ClearSpeak", expr, "division med täljaren 1 meter; och nämnaren 2 sekunder kvadrat");
    test("sv", "SimpleSpeak", expr, "division, 1 meter, genom, 2 sekunder kvadrat, slut division");
}


#[test]
fn acceleration_four_sevenths() {
    let expr = r#"<math>
    <mfrac><mn>4</mn><mn>7</mn></mfrac>
    <mfrac><mi mathvariant="normal" intent=":unit">m</mi><msup><mi mathvariant="normal" intent=":unit">s</mi><mn>2</mn></msup></mfrac>
</math>"#;
    test("sv", "ClearSpeak", expr, "4 sjundedels meter per sekund kvadrat");
    test("sv", "SimpleSpeak", expr, "4 sjundedels meter per sekund kvadrat");
}


#[test]
fn density_thirds() {
    let expr = r#"<math>
    <mfrac><mn>1</mn><mn>3</mn></mfrac>
    <mfrac><mi mathvariant="normal" intent=":unit">kg</mi><msup><mi mathvariant="normal" intent=":unit">m</mi><mn>3</mn></msup></mfrac>
</math>"#;
    test("sv", "ClearSpeak", expr, "ett tredjedels kilo-gram per meter kubik");
    test("sv", "SimpleSpeak", expr, "ett tredjedels kilo-gram per meter kubik");
}


#[test]
fn density_thirds_one_fraction() {
    let expr = r#"<math>
    <mfrac>
    <mrow><mn>1</mn><mi mathvariant="normal" intent=":unit">kg</mi></mrow>
    <mrow><mn>3</mn><msup><mi mathvariant="normal" intent=":unit">m</mi><mn>3</mn></msup></mrow>
    </mfrac>
</math>"#;
    test("sv", "ClearSpeak", expr, "division med täljaren; ett kilo-gram; och nämnaren 3 meter kubik");
    test("sv", "SimpleSpeak", expr, "division, ett kilo-gram, genom, 3 meter kubik, slut division");
}


#[test]
fn density_two_fifths() {
    let expr = r#"<math>
    <mfrac><mn>2</mn><mn>5</mn></mfrac>
    <mfrac><mi mathvariant="normal" intent=":unit">kg</mi><msup><mi mathvariant="normal" intent=":unit">m</mi><mn>3</mn></msup></mfrac>
</math>"#;
    test("sv", "ClearSpeak", expr, "2 femtedels kilo-gram per meter kubik");
    test("sv", "SimpleSpeak", expr, "2 femtedels kilo-gram per meter kubik");
}