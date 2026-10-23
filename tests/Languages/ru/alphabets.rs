/// Тесты для правил, общих для различных стилей речи:
/// * здесь собраны тесты, посвящённые различным алфавитам
use crate::common::*;

use anyhow::Result;

#[test]
fn traditional_latin_names() -> Result<()> {
    // Use the agreed Russian names for g, j and w, including uppercase letters.
    let expr = "<math><mi>g</mi><mo>,</mo><mi>j</mi><mo>,</mo><mi>w</mi><mo>,</mo><mi>G</mi><mo>,</mo><mi>J</mi><mo>,</mo><mi>W</mi></math>";
    test("ru", "SimpleSpeak", expr, "гэ, йот, дубль-вэ, заглавная гэ, заглавная йот, заглавная дубль-вэ")?;
    test("ru", "ClearSpeak", expr, "гэ, йот, дубль-вэ, заглавная гэ, заглавная йот, заглавная дубль-вэ")?;
    return Ok(());
}

#[test]
fn special_alphabet_chars() -> Result<()> {
    let expr = "<math> <mi>ℌ</mi><mo>,</mo><mi>ℭ</mi></math>";
    test("ru", "SimpleSpeak", expr, "фрактурная заглавная аш, фрактурная заглавная цэ")?;
    let expr = "<math> <mi>ℍ</mi><mo>,</mo><mi>ℿ</mi></math>";
    test("ru", "SimpleSpeak", expr, "двойная заглавная аш, двойная заглавная пи")?;
    let expr = "<math> <mi>ℐ</mi><mo>,</mo><mi>ℳ</mi></math>";
    test("ru", "SimpleSpeak", expr, "рукописная заглавная и, рукописная заглавная эм")?;
    return Ok(());
}

#[test]
fn greek() -> Result<()> {
    let expr = "<math> <mi>Α</mi><mo>,</mo><mi>Ω</mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная альфа, заглавная омега")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("ru", "SimpleSpeak", expr, "альфа, омега")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "двойная заглавная дельта, двойная заглавная ипсилон")?;
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("ru", "SimpleSpeak", expr, "альфа, омега")?;
    return Ok(());
}

#[test]
fn cap_cyrillic() -> Result<()> {
    let expr = "<math> <mi>А</mi><mo>,</mo><mi>Я</mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная а, заглавная я")?;
    return Ok(());
}

#[test]
fn parenthesized() -> Result<()> {
    let expr = "<math> <mi>⒜</mi><mo>,</mo><mi>⒵</mi></math>";
    test("ru", "SimpleSpeak", expr, "а в скобках, зэт в скобках")?;
    return Ok(());
}

#[test]
fn circled() -> Result<()> {
    let expr = "<math> <mi>Ⓐ</mi><mo>,</mo><mi>Ⓩ</mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная а в кружке, заглавная зэт в кружке")?;
    let expr = "<math> <mi>ⓐ</mi><mo>,</mo><mi>ⓩ</mi></math>";
    test("ru", "SimpleSpeak", expr, "а в кружке, зэт в кружке")?;
    return Ok(());
}

#[test]
fn fraktur() -> Result<()> {
    let expr = "<math> <mi>𝔄</mi><mo>,</mo><mi>𝔜</mi></math>";
    test("ru", "SimpleSpeak", expr, "фрактурная заглавная а, фрактурная заглавная игрек")?;
    let expr = "<math> <mi>𝔞</mi><mo>,</mo><mi>𝔷</mi></math>";
    test("ru", "SimpleSpeak", expr, "фрактурная а, фрактурная зэт")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "фрактурная заглавная а, фрактурная заглавная игрек")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "фрактурная а, фрактурная зэт")?;
    return Ok(());
}

#[test]
fn bold_fraktur() -> Result<()> {
    let expr = "<math> <mi>𝕬</mi><mo>,</mo><mi>𝖅</mi></math>";
    test("ru", "SimpleSpeak", expr, "фрактурная жирная заглавная а, фрактурная жирная заглавная зэт")?;
    let expr = "<math> <mi>𝖆</mi><mo>,</mo><mi>𝖟</mi></math>";
    test("ru", "SimpleSpeak", expr, "фрактурная жирная а, фрактурная жирная зэт")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "фрактурная жирная заглавная а, фрактурная жирная заглавная зэт")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "фрактурная жирная а, фрактурная жирная зэт")?;
    return Ok(());
}

#[test]
fn double_struck() -> Result<()> {
    let expr = "<math> <mi>𝔸</mi><mo>,</mo><mi>𝕐</mi></math>";
    test("ru", "SimpleSpeak", expr, "двойная заглавная а, двойная заглавная игрек")?;
    let expr = "<math> <mi>𝕒</mi><mo>,</mo><mi>𝕫</mi></math>";
    test("ru", "SimpleSpeak", expr, "двойная а, двойная зэт")?;
    let expr = "<math> <mi>𝟘</mi><mo>,</mo><mi>𝟡</mi></math>";
    test("ru", "SimpleSpeak", expr, "двойной ноль, двойная девять")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "двойная заглавная а, двойная заглавная игрек")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "двойная а, двойная зэт")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "двойной ноль, двойная девять")?;
    return Ok(());
}

#[test]
fn script() -> Result<()> {
    let expr = "<math> <mi>𝒜</mi><mo>,</mo><mi>𝒵</mi></math>";
    test("ru", "SimpleSpeak", expr, "рукописная заглавная а, рукописная заглавная зэт")?;
    let expr = "<math> <mi>𝒶</mi><mo>,</mo><mi>𝓏</mi></math>";
    test("ru", "SimpleSpeak", expr, "рукописная а, рукописная зэт")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "рукописная заглавная а, рукописная заглавная зэт")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "рукописная а, рукописная зэт")?;
    return Ok(());
}

#[test]
fn bold_script() -> Result<()> {
    let expr = "<math> <mi>𝓐</mi><mo>,</mo><mi>𝓩</mi></math>";
    test("ru", "SimpleSpeak", expr, "рукописная жирная заглавная а, рукописная жирная заглавная зэт")?;
    let expr = "<math> <mi>𝓪</mi><mo>,</mo><mi>𝔃</mi></math>";
    test("ru", "SimpleSpeak", expr, "рукописная жирная а, рукописная жирная зэт")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "рукописная жирная заглавная а, рукописная жирная заглавная зэт")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "рукописная жирная а, рукописная жирная зэт")?;
    return Ok(());
}

#[test]
fn bold() -> Result<()> {
    let expr = "<math> <mi>𝐀</mi><mo>,</mo><mi>𝐙</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная а, жирная заглавная зэт")?;
    let expr = "<math> <mi>𝐚</mi><mo>,</mo><mi>𝐳</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная а, жирная зэт")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная а, жирная заглавная зэт")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная а, жирная зэт")?;
    return Ok(());
}

#[test]
fn italic() -> Result<()> {
    let expr = "<math> <mi>𝐴</mi><mo>,</mo><mi>𝑍</mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная а, заглавная зэт")?;
    let expr = "<math> <mi>𝑎</mi><mo>,</mo><mi>𝑧</mi></math>";
    test("ru", "SimpleSpeak", expr, "а, зэт")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная а, заглавная зэт")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "а, зэт")?;
    return Ok(());
}

#[test]
fn sans_serif() -> Result<()> {
    let expr = "<math> <mi>𝖠</mi><mo>,</mo><mi>𝖹</mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная а, заглавная зэт")?;
    let expr = "<math> <mi>𝖺</mi><mo>,</mo><mi>𝗓</mi></math>";
    test("ru", "SimpleSpeak", expr, "а, зэт")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная а, заглавная зэт")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "а, зэт")?;
    return Ok(());
}

#[test]
fn sans_serif_bold() -> Result<()> {
    let expr = "<math> <mi>𝗔</mi><mo>,</mo><mi>𝗭</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная а, жирная заглавная зэт")?;
    let expr = "<math> <mi>𝗮</mi><mo>,</mo><mi>𝘇</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная а, жирная зэт")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная а, жирная заглавная зэт")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная а, жирная зэт")?;
    return Ok(());
}

#[test]
fn sans_serif_italic() -> Result<()> {
    let expr = "<math> <mi>𝘈</mi><mo>,</mo><mi>𝘡</mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная а, заглавная зэт")?;
    let expr = "<math> <mi>𝘢</mi><mo>,</mo><mi>𝘻</mi></math>";
    test("ru", "SimpleSpeak", expr, "а, зэт")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная а, заглавная зэт")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "а, зэт")?;
    return Ok(());
}

#[test]
fn sans_serif_bold_italic() -> Result<()> {
    let expr = "<math> <mi>𝘼</mi><mo>,</mo><mi>𝙕</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная а, жирная заглавная зэт")?;
    let expr = "<math> <mi>𝙖</mi><mo>,</mo><mi>𝙯</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная а, жирная зэт")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная а, жирная заглавная зэт")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная а, жирная зэт")?;
    return Ok(());
}

#[test]
fn monospace() -> Result<()> {
    let expr = "<math> <mi>𝙰</mi><mo>,</mo><mi>𝚉</mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная а, заглавная зэт")?;
    let expr = "<math> <mi>𝚊</mi><mo>,</mo><mi>𝚣</mi></math>";
    test("ru", "SimpleSpeak", expr, "а, зэт")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная а, заглавная зэт")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "а, зэт")?;
    return Ok(());
}

#[test]
fn bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝚨</mi><mo>,</mo><mi>𝛀</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная альфа, жирная заглавная омега")?;
    let expr = "<math> <mi>𝛂</mi><mo>,</mo><mi>𝛚</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная альфа, жирная омега")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная альфа, жирная заглавная омега")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная альфа, жирная омега")?;
    return Ok(());
}

#[test]
fn bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝛛</mi><mo>,</mo><mi>𝛡</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная частная производная, жирное пи")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная частная производная, жирное пи")?;
    return Ok(());
}

#[test]
fn italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝛢</mi><mo>,</mo><mi>𝛺</mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная альфа, заглавная омега")?;
    let expr = "<math> <mi>𝛼</mi><mo>,</mo><mi>𝜔</mi></math>";
    test("ru", "SimpleSpeak", expr, "альфа, омега")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная альфа, заглавная омега")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "альфа, омега")?;
    return Ok(());
}

#[test]
fn italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝜕</mi><mo>,</mo><mi>𝜛</mi></math>";
    test("ru", "SimpleSpeak", expr, "частная производная, пи")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "частная производная, пи")?;
    return Ok(());
}

#[test]
fn bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝜜</mi><mo>,</mo><mi>𝜴</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная альфа, жирная заглавная омега")?;
    let expr = "<math> <mi>𝜶</mi><mo>,</mo><mi>𝝎</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная альфа, жирная омега")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная альфа, жирная заглавная омега")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная альфа, жирная омега")?;
    return Ok(());
}

#[test]
fn bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝝏</mi><mo>,</mo><mi>𝝕</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная частная производная, жирное пи")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная частная производная, жирное пи")?;
    return Ok(());
}

#[test]
fn sans_serif_bold_greek() -> Result<()> {
    let expr = "<math> <mi>𝝖</mi><mo>,</mo><mi>𝝮</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная альфа, жирная заглавная омега")?;
    let expr = "<math> <mi>𝝰</mi><mo>,</mo><mi>𝞈</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная альфа, жирная омега")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная альфа, жирная заглавная омега")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная альфа, жирная омега")?;
    return Ok(());
}

#[test]
fn sans_serif_bold_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝞉</mi><mo>,</mo><mi>𝞏</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная частная производная, жирное пи")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная частная производная, жирное пи")?;
    return Ok(());
}

#[test]
fn sans_serif_bold_italic_greek() -> Result<()> {
    let expr = "<math> <mi>𝞐</mi><mo>,</mo><mi>𝞨</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная альфа, жирная заглавная омега")?;
    let expr = "<math> <mi>𝞪</mi><mo>,</mo><mi>𝟂</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная альфа, жирная омега")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная заглавная альфа, жирная заглавная омега")?;
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная альфа, жирная омега")?;
    return Ok(());
}

#[test]
fn sans_serif_bold_italic_greek_others() -> Result<()> {
    let expr = "<math> <mi>𝟃</mi><mo>,</mo><mi>𝟉</mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная частная производная, жирное пи")?;
    // Версии из частной области MathType
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "жирная частная производная, жирное пи")?;
    return Ok(());
}

#[test]
fn pua_regular() -> Result<()> {
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("ru", "SimpleSpeak", expr, "заглавная а, заглавная зэт")?;
    return Ok(());
}

#[test]
fn turned() -> Result<()> {
    let expr = "<math> <mi>Ⅎ</mi><mo>,</mo><mi>⅄</mi></math>";
    test("ru", "SimpleSpeak", expr, "перевернутая заглавная эф, перевернутая заглавная игрек без засечек")?;
    return Ok(());
}

#[test]
fn enclosed_numbers() -> Result<()> {
    let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
    test("ru", "SimpleSpeak", expr, "один в кружке, девять в кружке")?;
    let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
    test("ru", "SimpleSpeak", expr, "один в скобках, девять в скобках")?;
    let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
    test("ru", "SimpleSpeak", expr, "один с точкой, девять с точкой")?;
    let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
    test("ru", "SimpleSpeak", expr, "один в двойном кружке, девять в двойном кружке")?;
    return Ok(());
}
