/// Polish unit plurals: the noun form follows the numeral, not just "is it one".
///   1 metr | 2, 3, 4 metry (nominative plural) | 5+ metrów (genitive plural)
/// The teens are the exception that breaks the naive rule: 12, 13, 14 metrów.
use crate::common::*;
use anyhow::Result;

/// The three numeral classes on a single unit, including the teens exception.
#[test]
fn unit_plural_numeral_classes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">m</mi><mo>,</mo><mn>2</mn><mi intent=":unit">m</mi><mo>,</mo>
        <mn>3</mn><mi intent=":unit">m</mi><mo>,</mo><mn>4</mn><mi intent=":unit">m</mi><mo>,</mo>
        <mn>5</mn><mi intent=":unit">m</mi><mo>,</mo><mn>11</mn><mi intent=":unit">m</mi><mo>,</mo>
        <mn>12</mn><mi intent=":unit">m</mi><mo>,</mo><mn>13</mn><mi intent=":unit">m</mi><mo>,</mo>
        <mn>14</mn><mi intent=":unit">m</mi><mo>,</mo><mn>22</mn><mi intent=":unit">m</mi><mo>,</mo>
        <mn>101</mn><mi intent=":unit">m</mi><mo>,</mo><mn>102</mn><mi intent=":unit">m</mi><mo>,</mo>
        <mn>112</mn><mi intent=":unit">m</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "1 metr, przecinek; 2 metry, przecinek; 3 metry, przecinek; 4 metry, przecinek; 5 metrów, przecinek; 11 metrów, przecinek; 12 metrów, przecinek; 13 metrów, przecinek; 14 metrów, przecinek; 22 metry, przecinek; 101 metrów, przecinek; 102 metry, przecinek; 112 metrów")?;
    Ok(())
}

/// SI base units: previously all of these took the English "s".
#[test]
fn unit_plural_si_base() -> Result<()> {
    let expr = r#"<math>
        <mn>2</mn><mi intent=":unit">g</mi><mo>,</mo><mn>5</mn><mi intent=":unit">g</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">s</mi><mo>,</mo><mn>5</mn><mi intent=":unit">s</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">A</mi><mo>,</mo><mn>5</mn><mi intent=":unit">A</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">K</mi><mo>,</mo><mn>5</mn><mi intent=":unit">K</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">mol</mi><mo>,</mo><mn>5</mn><mi intent=":unit">mol</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">cd</mi><mo>,</mo><mn>5</mn><mi intent=":unit">cd</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "2 gramy, przecinek; 5 gramów, przecinek; 2 sekundy, przecinek; 5 sekund, przecinek; 2 ampery, przecinek; 5 amperów, przecinek; 2 kelwiny, przecinek; 5 kelwinów, przecinek; 2 mole, przecinek; 5 moli, przecinek; 2 kandele, przecinek; 5 kandeli")?;
    Ok(())
}

/// With an SI prefix the unit goes through a different branch of the rule.
#[test]
fn unit_plural_with_prefix() -> Result<()> {
    let expr = r#"<math>
        <mn>2</mn><mi intent=":unit">km</mi><mo>,</mo><mn>5</mn><mi intent=":unit">km</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">mg</mi><mo>,</mo><mn>5</mn><mi intent=":unit">mg</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">daA</mi><mo>,</mo><mn>5</mn><mi intent=":unit">daA</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "2 kilo-metry, przecinek; 5 kilo-metrów, przecinek; 2 mili-gramy, przecinek; 5 mili-gramów, przecinek; 2 deka-ampery, przecinek; 5 deka-amperów")?;
    Ok(())
}

/// Irregular nouns: the plural cannot be derived from the ending.
#[test]
fn unit_plural_irregular() -> Result<()> {
    let expr = r#"<math>
        <mn>2</mn><mi intent=":unit">d</mi><mo>,</mo><mn>5</mn><mi intent=":unit">d</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">w</mi><mo>,</mo><mn>5</mn><mi intent=":unit">w</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">yr</mi><mo>,</mo><mn>5</mn><mi intent=":unit">yr</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">ft</mi><mo>,</mo><mn>5</mn><mi intent=":unit">ft</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "2 dni, przecinek, 5 dni, przecinek; 2 tygodnie, przecinek; 5 tygodni, przecinek; 2 lata, przecinek, 5 lat, przecinek; 2 stopy, przecinek; 5 stóp")?;
    Ok(())
}

/// Units whose base form was itself wrong (plural used as singular).
#[test]
fn unit_singular_forms_fixed() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">Hz</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Hz</mi><mo>,</mo>
        <mn>5</mn><mi intent=":unit">Hz</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">hp</mi><mo>,</mo><mn>2</mn><mi intent=":unit">hp</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">℧</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "1 herc, przecinek; 2 herce, przecinek; 5 herców, przecinek; 1 koń mechaniczny, przecinek; 2 konie mechaniczne, przecinek, 1 mho")?;
    Ok(())
}

/// Multi-word units inflect the head noun and keep the dependent word's case.
#[test]
fn unit_plural_multiword() -> Result<()> {
    let expr = r#"<math>
        <mn>2</mn><mi intent=":unit">ly</mi><mo>,</mo><mn>5</mn><mi intent=":unit">ly</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">au</mi><mo>,</mo><mn>5</mn><mi intent=":unit">au</mi><mo>,</mo>
        <mn>2</mn><mi intent=":unit">arcmin</mi><mo>,</mo><mn>5</mn><mi intent=":unit">arcmin</mi>
    </math>"#;
    test("pl", "SimpleSpeak", expr, "2 lata świetlne, przecinek; 5 lat świetlnych, przecinek; 2 jednostki astronomiczne, przecinek; 5 jednostek astronomicznych; przecinek; 2 minuty łuku, przecinek; 5 minut łuku")?;
    Ok(())
}
