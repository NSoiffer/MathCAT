/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;


#[test]
fn special_alphabet_chars() {
  let expr = "<math> <mi>ℌ</mi><mo>,</mo><mi>ℭ</mi></math>";
  test("fi", "SimpleSpeak", expr, "fraktuura iso h pilkku, fraktuura iso c");
  let expr = "<math> <mi>ℍ</mi><mo>,</mo><mi>ℿ</mi></math>";
  test("fi", "SimpleSpeak", expr, "kaksiviivainen iso h, pilkku, kaksiviivainen iso pii");
  let expr = "<math> <mi>ℐ</mi><mo>,</mo><mi>ℳ</mi></math>";
  test("fi", "SimpleSpeak", expr, "kauno iso i pilkku, kauno iso m");
}

#[test]
fn greek() {
    let expr = "<math> <mi>Α</mi><mo>,</mo><mi>Ω</mi></math>";
    test("fi", "SimpleSpeak", expr, "iso alfa pilkku, iso oomega");
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("fi", "SimpleSpeak", expr, "alfa pilkku, oomega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "kaksoisviivainen iso delta, pilkku; kaksoisviivainen iso ypsilon");
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("fi", "SimpleSpeak", expr, "alfa pilkku, oomega");
}

#[test]
fn cap_cyrillic() {
    let expr = "<math> <mi>А</mi><mo>,</mo><mi>Я</mi></math>";
    test("fi", "SimpleSpeak", expr, "iso a pilkku, iso jaa");
}

#[test]
fn parenthesized() {
    let expr = "<math> <mi>⒜</mi><mo>,</mo><mi>⒵</mi></math>";
    test("fi", "SimpleSpeak", expr, "suluissa a pilkku, suluissa z");
}

#[test]
fn circled() {
    let expr = "<math> <mi>Ⓐ</mi><mo>,</mo><mi>Ⓩ</mi></math>";
    test("fi", "SimpleSpeak", expr, "ympyröity iso a pilkku, ympyröity iso z");
    let expr = "<math> <mi>ⓐ</mi><mo>,</mo><mi>ⓩ</mi></math>";
    test("fi", "SimpleSpeak", expr, "ympyröity a pilkku, ympyröity z");
}

#[test]
fn fraktur() {
    let expr = "<math> <mi>𝔄</mi><mo>,</mo><mi>𝔜</mi></math>";
    test("fi", "SimpleSpeak", expr, "fraktuura iso a pilkku, fraktuura iso y");
    let expr = "<math> <mi>𝔞</mi><mo>,</mo><mi>𝔷</mi></math>";
    test("fi", "SimpleSpeak", expr, "fraktuura a pilkku, fraktuura z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "fraktuura iso a pilkku, fraktuura iso y");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "fraktuura a pilkku, fraktuura z");
}

#[test]
fn bold_fraktur() {
    let expr = "<math> <mi>𝕬</mi><mo>,</mo><mi>𝖅</mi></math>";
    test("fi", "SimpleSpeak", expr, "fraktuura lihavoitu iso a, pilkku; fraktuura lihavoitu iso z");
    let expr = "<math> <mi>𝖆</mi><mo>,</mo><mi>𝖟</mi></math>";
    test("fi", "SimpleSpeak", expr, "fraktuura lihavoitu a, pilkku, fraktuura lihavoitu z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "fraktuura lihavoitu iso a, pilkku; fraktuura lihavoitu iso z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "fraktuura lihavoitu a, pilkku, fraktuura lihavoitu z");
}

#[test]
fn double_struck() {
    let expr = "<math> <mi>𝔸</mi><mo>,</mo><mi>𝕐</mi></math>";
    test("fi", "SimpleSpeak", expr, "kaksoisviivainen iso a, pilkku, kaksoisviivainen iso y");
    let expr = "<math> <mi>𝕒</mi><mo>,</mo><mi>𝕫</mi></math>";
    test("fi", "SimpleSpeak", expr, "kaksoisviivainen a pilkku, kaksoisviivainen z");
    let expr = "<math> <mi>𝟘</mi><mo>,</mo><mi>𝟡</mi></math>";
    test("fi", "SimpleSpeak", expr, "kaksoisviivainen 0 pilkku, kaksoisviivainen 9");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "kaksoisviivainen iso a, pilkku, kaksoisviivainen iso y");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "kaksoisviivainen a pilkku, kaksoisviivainen z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "kaksoisviivainen 0 pilkku, kaksoisviivainen 9");
}

#[test]
fn script() {
    let expr = "<math> <mi>𝒜</mi><mo>,</mo><mi>𝒵</mi></math>";
    test("fi", "SimpleSpeak", expr, "kauno iso a pilkku, kauno iso z");
    let expr = "<math> <mi>𝒶</mi><mo>,</mo><mi>𝓏</mi></math>";
    test("fi", "SimpleSpeak", expr, "kauno a pilkku, kauno z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "kauno iso a pilkku, kauno iso z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "kauno a pilkku, kauno z");
}

#[test]
fn bold_script() {
    let expr = "<math> <mi>𝓐</mi><mo>,</mo><mi>𝓩</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu kauno iso a, pilkku, lihavoitu kauno iso z");
    let expr = "<math> <mi>𝓪</mi><mo>,</mo><mi>𝔃</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu kauno a pilkku, lihavoitu kauno z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu kauno iso a, pilkku, lihavoitu kauno iso z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu kauno a pilkku, lihavoitu kauno z");
}

#[test]
fn bold() {
    let expr = "<math> <mi>𝐀</mi><mo>,</mo><mi>𝐙</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso a pilkku, lihavoitu iso z");
    let expr = "<math> <mi>𝐚</mi><mo>,</mo><mi>𝐳</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu a pilkku, lihavoitu z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso a pilkku, lihavoitu iso z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu a pilkku, lihavoitu z");
}

#[test]
fn italic() {
    let expr = "<math> <mi>𝐴</mi><mo>,</mo><mi>𝑍</mi></math>";
    test("fi", "SimpleSpeak", expr, "iso a pilkku, iso z");
    let expr = "<math> <mi>𝑎</mi><mo>,</mo><mi>𝑧</mi></math>";
    test("fi", "SimpleSpeak", expr, "a pilkku, z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "iso a pilkku, iso z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "a pilkku, z");
}

#[test]
fn sans_serif() {
  let expr = "<math> <mi>𝖠</mi><mo>,</mo><mi>𝖹</mi></math>";
  test("fi", "SimpleSpeak", expr, "iso a pilkku, iso z");
  let expr = "<math> <mi>𝖺</mi><mo>,</mo><mi>𝗓</mi></math>";
  test("fi", "SimpleSpeak", expr, "a pilkku, z");
  // MathType private space versions
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("fi", "SimpleSpeak", expr, "iso a pilkku, iso z");
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("fi", "SimpleSpeak", expr, "a pilkku, z");
}

#[test]
fn sans_serif_bold() {
    let expr = "<math> <mi>𝗔</mi><mo>,</mo><mi>𝗭</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso a pilkku, lihavoitu iso z");
    let expr = "<math> <mi>𝗮</mi><mo>,</mo><mi>𝘇</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu a pilkku, lihavoitu z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso a pilkku, lihavoitu iso z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu a pilkku, lihavoitu z");
}

#[test]
fn sans_serif_italic() {
    let expr = "<math> <mi>𝘈</mi><mo>,</mo><mi>𝘡</mi></math>";
    test("fi", "SimpleSpeak", expr, "iso a pilkku, iso z");
    let expr = "<math> <mi>𝘢</mi><mo>,</mo><mi>𝘻</mi></math>";
    test("fi", "SimpleSpeak", expr, "a pilkku, z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "iso a pilkku, iso z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "a pilkku, z");
}

#[test]
fn sans_serif_bold_italic() {
    let expr = "<math> <mi>𝘼</mi><mo>,</mo><mi>𝙕</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso a pilkku, lihavoitu iso z");
    let expr = "<math> <mi>𝙖</mi><mo>,</mo><mi>𝙯</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu a pilkku, lihavoitu z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso a pilkku, lihavoitu iso z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu a pilkku, lihavoitu z");
}

#[test]
fn monospace() {
    let expr = "<math> <mi>𝙰</mi><mo>,</mo><mi>𝚉</mi></math>";
    test("fi", "SimpleSpeak", expr, "iso a pilkku, iso z");
    let expr = "<math> <mi>𝚊</mi><mo>,</mo><mi>𝚣</mi></math>";
    test("fi", "SimpleSpeak", expr, "a pilkku, z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "iso a pilkku, iso z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "a pilkku, z");
}


#[test]
fn bold_greek() {
    let expr = "<math> <mi>𝚨</mi><mo>,</mo><mi>𝛀</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso alfa, pilkku, lihavoitu iso oomega");
    let expr = "<math> <mi>𝛂</mi><mo>,</mo><mi>𝛚</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu alfa pilkku, lihavoitu oomega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso alfa, pilkku, lihavoitu iso oomega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu alfa pilkku, lihavoitu oomega");
}

#[test]
fn bold_greek_others() {
    let expr = "<math> <mi>𝛛</mi><mo>,</mo><mi>𝛡</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu osittaisderivaatta, pilkku, lihavoitu pii");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu osittaisderivaatta, pilkku, lihavoitu pii");
}


#[test]
fn italic_greek() {
    let expr = "<math> <mi>𝛢</mi><mo>,</mo><mi>𝛺</mi></math>";
    test("fi", "SimpleSpeak", expr, "iso alfa pilkku, iso oomega");
    let expr = "<math> <mi>𝛼</mi><mo>,</mo><mi>𝜔</mi></math>";
    test("fi", "SimpleSpeak", expr, "alfa pilkku, oomega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "iso alfa pilkku, iso oomega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "alfa pilkku, oomega");
}

#[test]
fn italic_greek_others() {
    let expr = "<math> <mi>𝜕</mi><mo>,</mo><mi>𝜛</mi></math>";
    test("fi", "SimpleSpeak", expr, "osittaisderivaatta pilkku, pii");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "osittaisderivaatta pilkku, pii");
    
}

#[test]
fn bold_italic_greek() {
    let expr = "<math> <mi>𝜜</mi><mo>,</mo><mi>𝜴</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso alfa, pilkku, lihavoitu iso oomega");
    let expr = "<math> <mi>𝜶</mi><mo>,</mo><mi>𝝎</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu alfa pilkku, lihavoitu oomega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso alfa, pilkku, lihavoitu iso oomega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu alfa pilkku, lihavoitu oomega");
}

#[test]
fn bold_italic_greek_others() {
    let expr = "<math> <mi>𝝏</mi><mo>,</mo><mi>𝝕</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu osittaisderivaatta, pilkku, lihavoitu pii");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu osittaisderivaatta, pilkku, lihavoitu pii");
}

#[test]
fn sans_serif_bold_greek() {
    let expr = "<math> <mi>𝝖</mi><mo>,</mo><mi>𝝮</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso alfa, pilkku, lihavoitu iso oomega");
    let expr = "<math> <mi>𝝰</mi><mo>,</mo><mi>𝞈</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu alfa pilkku, lihavoitu oomega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso alfa, pilkku, lihavoitu iso oomega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu alfa pilkku, lihavoitu oomega");
}

#[test]
fn sans_serif_bold_greek_others() {
    let expr = "<math> <mi>𝞉</mi><mo>,</mo><mi>𝞏</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu osittaisderivaatta, pilkku, lihavoitu pii");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu osittaisderivaatta, pilkku, lihavoitu pii");
}

#[test]
fn sans_serif_bold_italic_greek() {
    let expr = "<math> <mi>𝞐</mi><mo>,</mo><mi>𝞨</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso alfa, pilkku, lihavoitu iso oomega");
    let expr = "<math> <mi>𝞪</mi><mo>,</mo><mi>𝟂</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu alfa pilkku, lihavoitu oomega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu iso alfa, pilkku, lihavoitu iso oomega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu alfa pilkku, lihavoitu oomega");
}

#[test]
fn sans_serif_bold_italic_greek_others() {
    let expr = "<math> <mi>𝟃</mi><mo>,</mo><mi>𝟉</mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu osittaisderivaatta, pilkku, lihavoitu pii");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("fi", "SimpleSpeak", expr, "lihavoitu osittaisderivaatta, pilkku, lihavoitu pii");
}

#[test]
fn pua_regular() {
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("fi", "SimpleSpeak", expr, "iso a pilkku, iso z");
}

#[test]
fn turned() {
    let expr = "<math> <mi>Ⅎ</mi><mo>,</mo><mi>⅄</mi></math>";
    test("fi", "SimpleSpeak", expr, "käännetty iso f pilkku; käännetty sans-serif iso y");
  }

#[test]
fn enclosed_numbers() {
  let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
  test("fi", "SimpleSpeak", expr, "ympyröity 1 pilkku, ympyröity 9");
  let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
  test("fi", "SimpleSpeak", expr, "suluissa 1 pilkku, suluissa 9");
  let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
  test("fi", "SimpleSpeak", expr, "1 pisteellä pilkku, 9 pisteellä");
  let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
  test("fi", "SimpleSpeak", expr, "kaksinkertaisesti ympyröity 1, pilkku; kaksinkertaisesti ympyröity 9");
}
