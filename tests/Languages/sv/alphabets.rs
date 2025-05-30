/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;


#[test]
fn special_alphabet_chars() {
  let expr = "<math> <mi>ℌ</mi><mo>,</mo><mi>ℭ</mi></math>";
  test("sv", "SimpleSpeak", expr, "fraktur versal h komma, fraktur versal c");
  let expr = "<math> <mi>ℍ</mi><mo>,</mo><mi>ℿ</mi></math>";
  test("sv", "SimpleSpeak", expr, "dubbelslaget versal h, komma, dubbelslaget versal pi");
  let expr = "<math> <mi>ℐ</mi><mo>,</mo><mi>ℳ</mi></math>";
  test("sv", "SimpleSpeak", expr, "skrivstilt versal i, komma, skrivstilt versal m");
}

#[test]
fn greek() {
    let expr = "<math> <mi>Α</mi><mo>,</mo><mi>Ω</mi></math>";
    test("sv", "SimpleSpeak", expr, "versal alfa komma, versal omega");
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("sv", "SimpleSpeak", expr, "alfa komma, omega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "dubbelslaget versal dellta, komma; dubbelslaget versal ypsilon");
    let expr = "<math> <mi>α</mi><mo>,</mo><mi>ω</mi></math>";
    test("sv", "SimpleSpeak", expr, "alfa komma, omega");
}

#[test]
fn cap_cyrillic() {
    let expr = "<math> <mi>А</mi><mo>,</mo><mi>Я</mi></math>";
    test("sv", "SimpleSpeak", expr, "versal a komma, versal ja");
}

#[test]
fn parenthesized() {
    let expr = "<math> <mi>⒜</mi><mo>,</mo><mi>⒵</mi></math>";
    test("sv", "SimpleSpeak", expr, "a inom parentes komma, z inom parentes");
}

#[test]
fn circled() {
    let expr = "<math> <mi>Ⓐ</mi><mo>,</mo><mi>Ⓩ</mi></math>";
    test("sv", "SimpleSpeak", expr, "versal a i cirkel komma, versal z i cirkel");
    let expr = "<math> <mi>ⓐ</mi><mo>,</mo><mi>ⓩ</mi></math>";
    test("sv", "SimpleSpeak", expr, "a i cirkel komma, z i cirkel");
}

#[test]
fn fraktur() {
    let expr = "<math> <mi>𝔄</mi><mo>,</mo><mi>𝔜</mi></math>";
    test("sv", "SimpleSpeak", expr, "fraktur versal a komma, fraktur versal y");
    let expr = "<math> <mi>𝔞</mi><mo>,</mo><mi>𝔷</mi></math>";
    test("sv", "SimpleSpeak", expr, "fraktur a komma, fraktur z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fraktur versal a komma, fraktur versal y");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fraktur a komma, fraktur z");
}

#[test]
fn bold_fraktur() {
    let expr = "<math> <mi>𝕬</mi><mo>,</mo><mi>𝖅</mi></math>";
    test("sv", "SimpleSpeak", expr, "fraktur fetstilt versal a, komma, fraktur fetstilt versal z");
    let expr = "<math> <mi>𝖆</mi><mo>,</mo><mi>𝖟</mi></math>";
    test("sv", "SimpleSpeak", expr, "fraktur fetstilt a komma, fraktur fetstilt z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fraktur fetstilt versal a, komma, fraktur fetstilt versal z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fraktur fetstilt a komma, fraktur fetstilt z");
}

#[test]
fn double_struck() {
    let expr = "<math> <mi>𝔸</mi><mo>,</mo><mi>𝕐</mi></math>";
    test("sv", "SimpleSpeak", expr, "dubbelslaget versal a, komma, dubbelslaget versal y");
    let expr = "<math> <mi>𝕒</mi><mo>,</mo><mi>𝕫</mi></math>";
    test("sv", "SimpleSpeak", expr, "dubbelslaget a komma, dubbelslaget z");
    let expr = "<math> <mi>𝟘</mi><mo>,</mo><mi>𝟡</mi></math>";
    test("sv", "SimpleSpeak", expr, "dubbelslaget 0 komma, dubbelslaget 9");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "dubbelslaget versal a, komma, dubbelslaget versal y");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "dubbelslaget a komma, dubbelslaget z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "dubbelslaget 0 komma, dubbelslaget 9");
}

#[test]
fn script() {
    let expr = "<math> <mi>𝒜</mi><mo>,</mo><mi>𝒵</mi></math>";
    test("sv", "SimpleSpeak", expr, "skrivstilt versal a, komma, skrivstilt versal z");
    let expr = "<math> <mi>𝒶</mi><mo>,</mo><mi>𝓏</mi></math>";
    test("sv", "SimpleSpeak", expr, "skrivstilt a komma, skrivstilt z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "skrivstilt versal a, komma, skrivstilt versal z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "skrivstilt a komma, skrivstilt z");
}

#[test]
fn bold_script() {
    let expr = "<math> <mi>𝓐</mi><mo>,</mo><mi>𝓩</mi></math>";
    test("sv", "SimpleSpeak", expr, "skrivstilt fetstilt versal a, komma; skrivstilt fetstilt versal z");
    let expr = "<math> <mi>𝓪</mi><mo>,</mo><mi>𝔃</mi></math>";
    test("sv", "SimpleSpeak", expr, "skrivstilt fetstilt a, komma, skrivstilt fetstilt z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "skrivstilt fetstilt versal a, komma; skrivstilt fetstilt versal z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "skrivstilt fetstilt a, komma, skrivstilt fetstilt z");
}

#[test]
fn bold() {
    let expr = "<math> <mi>𝐀</mi><mo>,</mo><mi>𝐙</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal a komma, fetstilt versal z");
    let expr = "<math> <mi>𝐚</mi><mo>,</mo><mi>𝐳</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt a komma, fetstilt z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal a komma, fetstilt versal z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt a komma, fetstilt z");
}

#[test]
fn italic() {
    let expr = "<math> <mi>𝐴</mi><mo>,</mo><mi>𝑍</mi></math>";
    test("sv", "SimpleSpeak", expr, "versal a komma, versal z");
    let expr = "<math> <mi>𝑎</mi><mo>,</mo><mi>𝑧</mi></math>";
    test("sv", "SimpleSpeak", expr, "a komma, z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "versal a komma, versal z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "a komma, z");
}

#[test]
fn sans_serif() {
  let expr = "<math> <mi>𝖠</mi><mo>,</mo><mi>𝖹</mi></math>";
  test("sv", "SimpleSpeak", expr, "versal a komma, versal z");
  let expr = "<math> <mi>𝖺</mi><mo>,</mo><mi>𝗓</mi></math>";
  test("sv", "SimpleSpeak", expr, "a komma, z");
  // MathType private space versions
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("sv", "SimpleSpeak", expr, "versal a komma, versal z");
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("sv", "SimpleSpeak", expr, "a komma, z");
}

#[test]
fn sans_serif_bold() {
    let expr = "<math> <mi>𝗔</mi><mo>,</mo><mi>𝗭</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal a komma, fetstilt versal z");
    let expr = "<math> <mi>𝗮</mi><mo>,</mo><mi>𝘇</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt a komma, fetstilt z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal a komma, fetstilt versal z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt a komma, fetstilt z");
}

#[test]
fn sans_serif_italic() {
    let expr = "<math> <mi>𝘈</mi><mo>,</mo><mi>𝘡</mi></math>";
    test("sv", "SimpleSpeak", expr, "versal a komma, versal z");
    let expr = "<math> <mi>𝘢</mi><mo>,</mo><mi>𝘻</mi></math>";
    test("sv", "SimpleSpeak", expr, "a komma, z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "versal a komma, versal z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "a komma, z");
}

#[test]
fn sans_serif_bold_italic() {
    let expr = "<math> <mi>𝘼</mi><mo>,</mo><mi>𝙕</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal a komma, fetstilt versal z");
    let expr = "<math> <mi>𝙖</mi><mo>,</mo><mi>𝙯</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt a komma, fetstilt z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal a komma, fetstilt versal z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt a komma, fetstilt z");
}

#[test]
fn monospace() {
    let expr = "<math> <mi>𝙰</mi><mo>,</mo><mi>𝚉</mi></math>";
    test("sv", "SimpleSpeak", expr, "versal a komma, versal z");
    let expr = "<math> <mi>𝚊</mi><mo>,</mo><mi>𝚣</mi></math>";
    test("sv", "SimpleSpeak", expr, "a komma, z");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "versal a komma, versal z");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "a komma, z");
}


#[test]
fn bold_greek() {
    let expr = "<math> <mi>𝚨</mi><mo>,</mo><mi>𝛀</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal alfa, komma, fetstilt versal omega");
    let expr = "<math> <mi>𝛂</mi><mo>,</mo><mi>𝛚</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt alfa komma, fetstilt omega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal alfa, komma, fetstilt versal omega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt alfa komma, fetstilt omega");
}

#[test]
fn bold_greek_others() {
    let expr = "<math> <mi>𝛛</mi><mo>,</mo><mi>𝛡</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt dell komma, fetstilt pi");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt dell komma, fetstilt pi");
}


#[test]
fn italic_greek() {
    let expr = "<math> <mi>𝛢</mi><mo>,</mo><mi>𝛺</mi></math>";
    test("sv", "SimpleSpeak", expr, "versal alfa komma, versal omega");
    let expr = "<math> <mi>𝛼</mi><mo>,</mo><mi>𝜔</mi></math>";
    test("sv", "SimpleSpeak", expr, "alfa komma, omega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "versal alfa komma, versal omega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "alfa komma, omega");
}

#[test]
fn italic_greek_others() {
    let expr = "<math> <mi>𝜕</mi><mo>,</mo><mi>𝜛</mi></math>";
    test("sv", "SimpleSpeak", expr, "dell komma, pi");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "dell komma, pi");
}

#[test]
fn bold_italic_greek() {
    let expr = "<math> <mi>𝜜</mi><mo>,</mo><mi>𝜴</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal alfa, komma, fetstilt versal omega");
    let expr = "<math> <mi>𝜶</mi><mo>,</mo><mi>𝝎</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt alfa komma, fetstilt omega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal alfa, komma, fetstilt versal omega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt alfa komma, fetstilt omega");
}

#[test]
fn bold_italic_greek_others() {
    let expr = "<math> <mi>𝝏</mi><mo>,</mo><mi>𝝕</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt dell komma, fetstilt pi");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt dell komma, fetstilt pi");
}

#[test]
fn sans_serif_bold_greek() {
    let expr = "<math> <mi>𝝖</mi><mo>,</mo><mi>𝝮</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal alfa, komma, fetstilt versal omega");
    let expr = "<math> <mi>𝝰</mi><mo>,</mo><mi>𝞈</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt alfa komma, fetstilt omega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal alfa, komma, fetstilt versal omega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt alfa komma, fetstilt omega");
}

#[test]
fn sans_serif_bold_greek_others() {
    let expr = "<math> <mi>𝞉</mi><mo>,</mo><mi>𝞏</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt dell komma, fetstilt pi");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt dell komma, fetstilt pi");
}

#[test]
fn sans_serif_bold_italic_greek() {
    let expr = "<math> <mi>𝞐</mi><mo>,</mo><mi>𝞨</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal alfa, komma, fetstilt versal omega");
    let expr = "<math> <mi>𝞪</mi><mo>,</mo><mi>𝟂</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt alfa komma, fetstilt omega");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt versal alfa, komma, fetstilt versal omega");
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt alfa komma, fetstilt omega");
}

#[test]
fn sans_serif_bold_italic_greek_others() {
    let expr = "<math> <mi>𝟃</mi><mo>,</mo><mi>𝟉</mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt dell komma, fetstilt pi");
    // MathType private space versions
    let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
    test("sv", "SimpleSpeak", expr, "fetstilt dell komma, fetstilt pi");
}

#[test]
fn pua_regular() {
  let expr = "<math> <mi></mi><mo>,</mo><mi></mi></math>";
  test("sv", "SimpleSpeak", expr, "versal a komma, versal z");
}

#[test]
fn turned() {
    let expr = "<math> <mi>Ⅎ</mi><mo>,</mo><mi>⅄</mi></math>";
    test("sv", "SimpleSpeak", expr, "roterat versal f komma; roterat sans-serif versal y");
  }

#[test]
fn enclosed_numbers() {
  let expr = "<math> <mi>①</mi><mo>,</mo><mi>⑨</mi></math>";
  test("sv", "SimpleSpeak", expr, "1 i cirkel komma, 9 i cirkel");
  let expr = "<math> <mi>⑴</mi><mo>,</mo><mi>⑼</mi></math>";
  test("sv", "SimpleSpeak", expr, "1 inom parentes komma, 9 inom parentes");
  let expr = "<math> <mi>⒈</mi><mo>,</mo><mi>⒐</mi></math>";
  test("sv", "SimpleSpeak", expr, "1 med punkt komma, 9 med punkt");
  let expr = "<math> <mi>⓵</mi><mo>,</mo><mi>⓽</mi></math>";
  test("sv", "SimpleSpeak", expr, "1 i dubbel cirkel komma, 9 i dubbel cirkel");
}
