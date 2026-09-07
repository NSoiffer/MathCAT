/// Polish prepositions govern case: the denominator of a unit fraction is spoken
/// after "na", which requires the accusative - "na godzinę", not "na godzina".
use crate::common::*;
use anyhow::Result;

/// Feminine denominators: these are the ones whose form actually changes.
#[test]
fn per_fraction_feminine_denominator() -> Result<()> {
    let expr = r#"<math>
        <mn>62</mn><mfrac><mi intent=":unit">mi</mi><mi intent=":unit">hr</mi></mfrac><mo>,</mo>
        <mn>5</mn><mfrac><mi intent=":unit">m</mi><mi intent=":unit">s</mi></mfrac><mo>,</mo>
        <mn>2</mn><mfrac><mi intent=":unit">km</mi><mi intent=":unit">hr</mi></mfrac><mo>,</mo>
        <mn>7</mn><mfrac><mi intent=":unit">rad</mi><mi intent=":unit">min</mi></mfrac><mo>,</mo>
        <mn>3</mn><mfrac><mi intent=":unit">J</mi><mi intent=":unit">t</mi></mfrac>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "62 mile na godzinę, przecinek; 5 metrów na sekundę, przecinek; 2 kilo-metry na godzinę; przecinek; 7 radianów na minutę, przecinek; 3 dżule na tonę")?;
    Ok(())
}

/// Masculine and neuter denominators must NOT change: their accusative already
/// equals the nominative. This is the negative control for the change.
#[test]
fn per_fraction_masculine_denominator_unchanged() -> Result<()> {
    let expr = r#"<math>
        <mn>3</mn><mfrac><mi intent=":unit">g</mi><mi intent=":unit">l</mi></mfrac><mo>,</mo>
        <mn>1</mn><mfrac><mi intent=":unit">J</mi><mi intent=":unit">mol</mi></mfrac><mo>,</mo>
        <mn>5</mn><mfrac><mi intent=":unit">N</mi><mi intent=":unit">m</mi></mfrac><mo>,</mo>
        <mn>2</mn><mfrac><mi intent=":unit">V</mi><mi intent=":unit">A</mi></mfrac>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "3 gramy na litr, przecinek; 1 dżul na mol, przecinek; 5 niutonów na metr, przecinek; 2 wolty na amper")?;
    Ok(())
}

/// A unit standing on its own keeps the nominative - the accusative must not
/// leak outside the fraction denominator.
#[test]
fn standalone_unit_stays_nominative() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">hr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">s</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">t</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">min</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "1 godzina, przecinek; 1 sekunda, przecinek; 1 tona, przecinek; 1 minuta")?;
    Ok(())
}

/// Multi-word feminine units: the adjective agrees, a dependent noun does not.
#[test]
fn per_fraction_multiword_denominator() -> Result<()> {
    let expr = r#"<math>
        <mn>3</mn><mfrac><mi intent=":unit">g</mi><mi intent=":unit">sq ft</mi></mfrac><mo>,</mo>
        <mn>2</mn><mfrac><mi intent=":unit">m</mi><mi intent=":unit">arcmin</mi></mfrac>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "3 gramy na stopę kwadratową; przecinek; 2 metry na minutę łuku")?;
    Ok(())
}

/// A denominator raised to a power, as in m/s^2: the unit's parent is the
/// 'power' element, not the fraction, so this needs its own test.
#[test]
fn per_fraction_denominator_with_power() -> Result<()> {
    let expr = r#"<math>
        <mfrac>
            <mrow><mn>3</mn><mi intent=":unit">m</mi></mrow>
            <msup><mi intent=":unit">s</mi><mn>2</mn></msup>
        </mfrac>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "3 metry na sekundę do kwadratu")?;
    Ok(())
}
