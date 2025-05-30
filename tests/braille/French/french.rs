// French braille tests for the basic mathml tags
// These tests are from the French braille authority's publication "NOTATION PIATHÉMATIQUE BRAILLE" (Première édition janvier 2007)
//  https://www.avh.asso.fr/sites/default/files/notation_mathematique_braille2_0.pdf
use crate::common::*;

#[test]
fn test_6_4_01() {
    let expr= r#"<math><mrow><mi>E</mi><mo>=</mo><mrow><mo>{</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo>,</mo><mi>c</mi><mo>,</mo><mi>d</mi><mo>}</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠨⠑⠶⠨⠦⠁⠂⠃⠂⠉⠂⠙⠨⠴");
}

#[test]
fn test_6_4_02() {
    let expr= r#"<math><mrow><msup><mi>ℝ</mi><mo>*</mo></msup><mo>=</mo><mrow><mo>]</mo><mi>−</mi><mi>∞</mi><mo>,</mo><mn>0</mn><mo>[</mo></mrow><mo>∪</mo><mrow><mo>]</mo><mn>0</mn><mo>,</mo><mi>+</mi><mi>∞</mi><mo>[</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠨⠨⠗⠈⠐⠔⠶⠾⠤⠘⠉⠂⠼⠷⠸⠖⠾⠼⠂⠖⠘⠉⠷");
}

#[test]
fn test_6_4_03() {
    let expr= r#"<math><mrow><mi>x</mi><mo>∈</mo><mo stretchy="false">[</mo><mn>0</mn><mo>,</mo><mn>1</mn><mo>;</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo stretchy="false">]</mo></mrow></math>"#;
    test_braille("French", expr, "⠭⠘⠡⠷⠼⠂⠡⠆⠣⠂⠩⠾");
}

#[test]
fn test_6_4_04() {
    let expr= r#"<math><mrow><mi>x</mi><mo>∉</mo><msup><mi>ℝ</mi><mo>+</mo></msup></mrow></math>"#;
    test_braille("French", expr, "⠭⠘⠌⠨⠨⠗⠈⠖");
}

#[test]
fn test_6_4_05() {
    let expr= r#"<math><mrow><mi>D</mi><mo>⊂</mo><mi>F</mi></mrow></math>"#;
    test_braille("French", expr, "⠨⠙⠨⠡⠨⠋");
}

#[test]
fn test_6_4_06() {
    let expr= r#"<math><mrow><mi>D</mi><mo>⊄</mo><mi>E</mi></mrow></math>"#;
    test_braille("French", expr, "⠨⠙⠨⠌⠨⠑");
}

#[test]
fn test_6_4_07() {
    let expr= r#"<math><mrow><mi>A</mi><mo>⊆</mo><mi>B</mi></mrow></math>"#;
    test_braille("French", expr, "⠨⠁⠸⠨⠡⠨⠃");
}

#[test]
fn test_6_4_08() {
    let expr= r#"<math><mrow><mi>A</mi><mo>∩</mo><mi>B</mi><mo>=</mo><mi>⌀</mi></mrow></math>"#;
    test_braille("French", expr, "⠨⠁⠘⠖⠨⠃⠶⠘⠼");
}

#[test]
fn test_6_4_09() {
    let expr= r#"<math><mrow><msub><mi>∁</mi><mi>E</mi></msub><mi>F</mi></mrow></math>"#;
    test_braille("French", expr, "⠨⠩⠢⠨⠑⠨⠋");
}

#[test]
fn test_7_01() {
    let expr= r#"<math><mrow><mfrac><mn>2</mn><mn>3</mn></mfrac><mo>=</mo><mfrac><mn>4</mn><mn>6</mn></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠣⠌⠩⠶⠹⠌⠫");
}

#[test]
fn test_7_02() {
    let expr= r#"<math><mrow><mn>1</mn><mo>÷</mo><mn>2</mn><mo>=</mo><mn>0</mn><mo>,</mo><mn>5</mn></mrow></math>"#;
    test_braille("French", expr, "⠡⠒⠣⠶⠼⠂⠱");
}

#[test]
fn test_8_01() {
    let expr= r#"<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mi>a</mi></mfrac></math>"#;
    test_braille("French", expr, "⠠⠄⠰⠁⠖⠃⠆⠌⠁");
}

#[test]
fn test_8_02() {
    let expr= r#"<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mrow><mi>c</mi><mo>+</mo><mi>d</mi></mrow></mfrac></math>"#;
    test_braille("French", expr, "⠠⠄⠰⠁⠖⠃⠆⠌⠰⠉⠖⠙⠆");
}

#[test]
fn test_8_03() {
    let expr= r#"<math><mrow><mfrac><mi>a</mi><mi>b</mi></mfrac><mo>+</mo><mfrac><mi>c</mi><mi>d</mi></mfrac><mo>=</mo><mfrac><mrow><mi>a</mi><mi>d</mi><mo>+</mo><mi>b</mi><mi>c</mi></mrow><mrow><mi>b</mi><mi>d</mi></mrow></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠠⠄⠁⠌⠃⠖⠉⠌⠙⠶⠰⠁⠙⠖⠃⠉⠆⠌⠃⠙");
}

#[test]
fn test_8_04() {
    let expr= r#"<math><mrow><mi>C</mi><mi>a</mi><mi>r</mi><mi>d</mi><mspace width="0.333em"></mspace><mi>E</mi><mo>=</mo><mn>5</mn></mrow></math>"#;
    test_braille("French", expr, "⠠⠄⠨⠉⠁⠗⠙⠰⠨⠑⠆⠶⠱");
}

#[test]
fn test_8_05() {
    let expr= r#"<math><msup><mi>e</mi><mrow><mi>x</mi><mo>+</mo><mn>3</mn></mrow></msup></math>"#;
    test_braille("French", expr, "⠠⠄⠑⠈⠰⠭⠖⠩⠆");
}

#[test]
fn test_8_06() {
    let expr= r#"<math><msup><mi>e</mi><mn>7</mn></msup></math>"#;
    test_braille("French", expr, "⠠⠑⠈⠻");
}

#[test]
fn test_8_07() {
    let expr= r#"<math><msup><mi>e</mi><mrow><mi>−</mi><mi>x</mi></mrow></msup></math>"#;
    test_braille("French", expr, "⠠⠑⠈⠤⠭");
}

#[test]
fn test_8_08() {
    let expr= r#"<math><msup><mi>e</mi><mrow><mi>−</mi><mi>π</mi></mrow></msup></math>"#;
    test_braille("French", expr, "⠠⠑⠈⠤⠘⠏");
}

#[test]
fn test_8_09() {
    let expr= r#"<math><mrow><msup><mi>x</mi><mrow><mi>−</mi><mn>1</mn></mrow></msup><mo>=</mo><mfrac><mn>1</mn><mi>x</mi></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠠⠭⠈⠤⠡⠶⠡⠌⠭");
}

#[test]
fn test_9_1_01() {
    let expr= r#"<math><msup><mi>a</mi><mo>′</mo></msup></math>"#;
    test_braille("French", expr, "⠁⠄");
}

#[test]
fn test_9_1_02() {
    let expr= r#"<math><msup><mi>x</mi><mi>″</mi></msup></math>"#;
    test_braille("French", expr, "⠭⠄⠄");
}

#[test]
fn test_9_1_03() {
    let expr= r#"<math><mrow><msup><mi>a</mi><mo>′</mo></msup><mi>x</mi><mo>+</mo><msup><mi>a</mi><mi>″</mi></msup><mi>y</mi><mo>+</mo><msup><mi>a</mi><mi>‴</mi></msup><mi>z</mi></mrow></math>"#;
    test_braille("French", expr, "⠁⠄⠭⠖⠁⠄⠄⠽⠖⠁⠄⠄⠄⠵");
}

#[test]
fn test_9_2_01() {
    let expr= r#"<math><msub><mi>a</mi><mi>p</mi></msub></math>"#;
    test_braille("French", expr, "⠁⠢⠏");
}

#[test]
fn test_9_2_02() {
    let expr= r#"<math><msub><mi>x</mi><mn>0</mn></msub></math>"#;
    test_braille("French", expr, "⠭⠢⠼");
}

#[test]
fn test_9_2_03() {
    let expr= r#"<math><msub><mi>u</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub></math>"#;
    test_braille("French", expr, "⠥⠢⠰⠝⠖⠡⠆");
}

#[test]
fn test_9_3_01() {
    let expr= r#"<math><msup><mi>x</mi><mn>2</mn></msup></math>"#;
    test_braille("French", expr, "⠭⠈⠣");
}

#[test]
fn test_9_3_02() {
    let expr= r#"<math><msup><mi>x</mi><mrow><mi>p</mi><mo>+</mo><mi>q</mi></mrow></msup></math>"#;
    test_braille("French", expr, "⠭⠈⠰⠏⠖⠟⠆");
}

#[test]
fn test_9_3_03() {
    let expr= r#"<math><msup><mi>a</mi><mrow><mi>−</mi><mn>5</mn><mo>,</mo><mn>3</mn></mrow></msup></math>"#;
    test_braille("French", expr, "⠁⠈⠤⠱⠂⠩");
}

#[test]
fn test_9_3_04() {
    let expr= r#"<math><msup><mi>a</mi><mi>π</mi></msup></math>"#;
    test_braille("French", expr, "⠁⠈⠘⠏");
}

#[test]
fn test_9_3_05() {
    let expr= r#"<math><mrow><msup><mn>5</mn><mrow><mn>2</mn><msqrt><mn>3</mn></msqrt></mrow></msup><mo>≠</mo><msup><mn>5</mn><mrow><mn>2</mn><msqrt><mn>3</mn></msqrt></mrow></msup></mrow></math>"#;
    test_braille("French", expr, "⠱⠈⠣⠜⠩⠨⠶⠱⠈⠰⠣⠜⠩⠆");
}

#[test]
fn test_9_3_06() {
    let expr= r#"<math><mrow><msup><mi>e</mi><mrow><mi>−</mi><mn>5</mn><mi>x</mi></mrow></msup><mo>≠</mo><msup><mi>e</mi><mrow><mi>−</mi><mn>5</mn></mrow></msup><mi>x</mi></mrow></math>"#;
    test_braille("French", expr, "⠑⠈⠰⠤⠱⠭⠆⠨⠶⠰⠑⠈⠤⠱⠆⠭");
}

#[test]
fn test_9_4_01() {
    let expr= r#"<math><msub><msup><mi>a</mi><mo>′</mo></msup><mn>0</mn></msub></math>"#;
    test_braille("French", expr, "⠁⠄⠢⠼");
}

#[test]
fn test_9_4_02() {
    let expr= r#"<math><msup><msub><mi>x</mi><msup><mn>0</mn><mo>′</mo></msup></msub><mn>2</mn></msup></math>"#;
    test_braille("French", expr, "⠭⠄⠢⠼⠈⠣");
}

#[test]
fn test_9_4_03() {
    let expr= r#"<math><mrow><msubsup><mi>∁</mi><mi>m</mi><mi>p</mi></msubsup><mo>=</mo><msubsup><mi>∁</mi><mi>m</mi><mrow><mi>m</mi><mo>−</mo><mi>p</mi></mrow></msubsup></mrow></math>"#;
    test_braille("French", expr, "⠨⠉⠢⠍⠈⠏⠶⠨⠉⠢⠍⠈⠰⠍⠤⠏⠆");
}

#[test]
fn test_9_4_04() {
    let expr= r#"<math><msup><msub><mi>a</mi><msup><mrow><mi>m</mi><mo>+</mo><mi>n</mi></mrow><mo>′</mo></msup></msub><mrow><mi>p</mi><mo>+</mo><mi>q</mi></mrow></msup></math>"#;
    test_braille("French", expr, "⠁⠄⠄⠢⠰⠍⠖⠝⠆⠈⠰⠏⠖⠟⠆");
}

#[test]
fn test_9_4_05() {
    let expr= r#"<math><msubsup><mi>ℝ</mi><mo>+</mo><mo>*</mo></msubsup></math>"#;
    test_braille("French", expr, "⠨⠨⠗⠢⠖⠈⠐⠔");
}

#[test]
fn test_9_5_01() {
    let expr= r#"<math><mrow><msubsup><mi></mi><mrow></mrow><mi>t</mi></msubsup><mi>A</mi></mrow></math>"#;
    test_braille("French", expr, "⠈⠞⠨⠁");
}

#[test]
fn test_9_5_02() {
    let expr= r#"<math><mrow><msubsup><mi></mi><mn>2</mn><mrow></mrow></msubsup><mi>w</mi></mrow></math>"#;
    test_braille("French", expr, "⠢⠣⠺");
}

#[test]
fn test_9_5_03() {
    let expr= r#"<math><mrow><msubsup><mi></mi><mrow></mrow><mi>t</mi></msubsup><msubsup><mi>A</mi><mrow></mrow><mi>t</mi></msubsup><mi>B</mi></mrow></math>"#;
    test_braille("French", expr, "⠰⠈⠞⠨⠁⠆⠰⠈⠞⠨⠃⠆");
}

#[test]
fn test_9_5_04() {
    let expr= r#"<math><mrow><msubsup><mi></mi><mn>8</mn><mn>16</mn></msubsup><mi>O</mi></mrow></math>"#;
    test_braille("French", expr, "⠨⠕⠠⠢⠳⠠⠈⠡⠫");
}

#[test]
fn test_10_01() {
    let expr= r#"<math><msqrt><mn>2</mn></msqrt></math>"#;
    test_braille("French", expr, "⠜⠣");
}

#[test]
fn test_10_02() {
    let expr= r#"<math><mroot><mi>a</mi><mn>4</mn></mroot></math>"#;
    test_braille("French", expr, "⠈⠹⠜⠁");
}

#[test]
fn test_10_03() {
    let expr= r#"<math><mrow><mroot><mrow><mo stretchy="false">(</mo><mi>a</mi><mo>+</mo><mi>b</mi><msup><mo stretchy="false">)</mo><mn>3</mn></msup></mrow><mn>6</mn></mroot><mo>=</mo><msqrt><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow></msqrt></mrow></math>"#;
    test_braille("French", expr, "⠈⠫⠜⠰⠦⠁⠖⠃⠴⠈⠩⠆⠶⠜⠰⠁⠖⠃⠆");
}

#[test]
fn test_10_04() {
    let expr= r#"<math><mroot><mrow><mi>a</mi><mo>+</mo><msqrt><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow></msqrt></mrow><mn>3</mn></mroot></math>"#;
    test_braille("French", expr, "⠈⠩⠜⠰⠁⠖⠜⠰⠁⠖⠃⠆⠆");
}

#[test]
fn test_10_05() {
    let expr= r#"<math><mrow><mroot><mi>a</mi><mn>4</mn></mroot><mroot><mi>b</mi><mn>3</mn></mroot></mrow></math>"#;
    test_braille("French", expr, "⠰⠈⠹⠜⠁⠆⠰⠈⠩⠜⠃⠆");
}

#[test]
fn test_11_01() {
    let expr= r#"<math><mrow><mi>f</mi><mo>:</mo><mi>ℝ</mi><mo>→</mo><mrow><mi>ℝ</mi><mo mathvariant="double-struck">,</mo><mspace width="0.222em"></mspace></mrow><mspace width="0.222em"></mspace><mi>x</mi><mo>↦</mo><mi>y</mi><mo>=</mo><mfrac><mn>1</mn><mi>x</mi></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠒⠨⠨⠗⠸⠱⠨⠨⠗⠭⠐⠱⠽⠶⠡⠌⠭");
}

#[test]
fn test_12_1_01() {
    let expr= r#"<math><mover><mi>v</mi><mo accent="true">→</mo></mover></math>"#;
    test_braille("French", expr, "⠨⠒⠧");
}

#[test]
fn test_12_1_02() {
    let expr= r#"<math><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo accent="true">¯</mo></mover></math>"#;
    test_braille("French", expr, "⠸⠒⠨⠁⠨⠃");
}

#[test]
fn test_12_1_03() {
    let expr= r#"<math><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo accent="true">⏜</mo></mover></math>"#;
    test_braille("French", expr, "⠈⠒⠨⠁⠨⠃");
}

#[test]
fn test_12_1_04() {
    let expr= r#"<math><mover><mrow><mi>A</mi><mi>O</mi><mi>B</mi></mrow><mo accent="true">̂</mo></mover></math>"#;
    test_braille("French", expr, "⠘⠒⠨⠁⠨⠕⠨⠃");
}

#[test]
fn test_12_1_05() {
    let expr= r#"<math><mover><mrow><mo stretchy="false">(</mo><mi>O</mi><mi>x</mi><mo>,</mo><mi>O</mi><mi>y</mi><mo stretchy="false">)</mo></mrow><mo accent="true">̂</mo></mover></math>"#;
    test_braille("French", expr, "⠘⠒⠦⠨⠕⠭⠂⠨⠕⠽⠴");
}

#[test]
fn test_12_1_06() {
    let expr= r#"<math><mover><mrow><mo>(</mo><mover><mrow><mi>O</mi><mi>A</mi></mrow><mo accent="true">→</mo></mover><mo>,</mo><mover><mrow><mi>O</mi><mi>M</mi></mrow><mo accent="true">→</mo></mover><mo>)</mo></mrow><mo accent="true">̂</mo></mover></math>"#;
    test_braille("French", expr, "⠘⠒⠦⠨⠒⠨⠕⠨⠁⠂⠨⠒⠨⠕⠨⠍⠴");
}

#[test]
fn test_12_3_01() {
    let expr= r#"<math><mrow><mo stretchy="false">(</mo><mi>D</mi><mo stretchy="false">)</mo><mo>⫽</mo><mrow><mo>(</mo><mtext mathvariant="normal">Δ</mtext><mo>)</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠦⠨⠙⠴⠸⠳⠦⠨⠘⠙⠴");
}

#[test]
fn test_12_3_02() {
    let expr= r#"<math><mrow><mo stretchy="false">(</mo><mi>A</mi><mi>B</mi><mo stretchy="false">)</mo><mi>⊥</mi><mrow><mo>(</mo><msup><mi>A</mi><mo>′</mo></msup><msup><mi>B</mi><mo>′</mo></msup><mo>)</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠦⠨⠁⠨⠃⠴⠘⠳⠦⠨⠁⠄⠨⠃⠄⠴");
}

#[test]
fn test_13_2_01() {
    let expr= r#"<math><mrow><mo>|</mo><msub><mi>x</mi><mn>0</mn></msub><mo>|</mo></mrow></math>"#;
    test_braille("French", expr, "⠿⠭⠢⠼⠿");
}

#[test]
fn test_13_2_02() {
    let expr= r#"<math><mrow><mo stretchy="false">|</mo><mi>a</mi><mo>+</mo><mi>i</mi><mi>b</mi><mo stretchy="false">|</mo></mrow></math>"#;
    test_braille("French", expr, "⠿⠁⠖⠊⠃⠿");
}

#[test]
fn test_13_2_03() {
    let expr= r#"<math><mrow><mrow><mo>∥</mo><mi>k</mi><mover><mi>u</mi><mo accent="true">→</mo></mover><mo>∥</mo></mrow><mo>=</mo><mo stretchy="false">|</mo><mi>k</mi><mo stretchy="false">|</mo><mrow><mo>∥</mo><mover><mi>u</mi><mo accent="true">→</mo></mover><mo>∥</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠘⠿⠅⠨⠒⠥⠘⠿⠶⠿⠅⠿⠘⠿⠨⠒⠥⠘⠿");
}

#[test]
fn test_13_2_04() {
    let expr= r#"<math><mrow><mi>f</mi><mrow><mo>|</mo><mi>E</mi></mrow><mspace width="0.222em"></mspace></mrow></math>"#;
    test_braille("French", expr, "⠋⠿⠨⠑");
}

#[test]
fn test_14_1_01() {
    let expr= r#"<math><mrow><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo></mrow></math>"#;
    test_braille("French", expr, "⠋⠦⠭⠴");
}

#[test]
fn test_14_1_02() {
    let expr= r#"<math><mrow><msup><mi>g</mi><mo>′</mo></msup><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mo>=</mo><mn>3</mn><mi>x</mi><mo>+</mo><mn>4</mn></mrow></math>"#;
    test_braille("French", expr, "⠛⠄⠦⠭⠴⠶⠩⠭⠖⠹");
}

#[test]
fn test_14_1_03() {
    let expr= r#"<math><mrow><mi>f</mi><mo>∘</mo><mi>g</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mo>=</mo><mi>f</mi><mrow><mo>[</mo><mi>g</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mo>]</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠋⠸⠼⠛⠦⠭⠴⠶⠋⠷⠛⠦⠭⠴⠾");
}

#[test]
fn test_14_1_04() {
    let expr= r#"<math><mrow><mfrac><mrow><mi>∂</mi><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo stretchy="false">)</mo></mrow><mrow><mi>∂</mi><mi>x</mi></mrow></mfrac><mo>=</mo><mn>5</mn><mi>x</mi><mi>y</mi><mo>−</mo><mn>7</mn><mi>x</mi></mrow></math>"#;
    test_braille("French", expr, "⠐⠙⠋⠦⠭⠂⠽⠴⠌⠐⠙⠭⠶⠱⠭⠽⠤⠻⠭");
}

#[test]
fn test_14_2_01_corrected() {
    let expr= r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mrow><munder><mi>lim</mi><mrow><mi>x</mi><mo>→</mo><mi>+</mi><mi>∞</mi></mrow></munder><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mo>=</mo><mn>0</mn></mrow></math>"#;
    test_braille("French", expr, "⠇⠊⠍⠢⠢⠰⠭⠸⠱⠖⠘⠉⠆⠋⠦⠭⠴⠶⠼");
}

#[test]
fn test_14_2_02_corrected_1() {
    let expr= r#"<math  xmlns="http://www.w3.org/1998/Math/MathML"><mrow><msub><mi>lim</mi><mtable><mtr><mtd columnalign="right" style="text-align: right"><mi>x</mi><mo>→</mo><mn>4</mn></mtd></mtr><mtr><mtd columnalign="right" style="text-align: right"><mi>x</mi><mo>&gt;</mo><mn>4</mn></mtd></mtr></mtable></msub><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mo>=</mo><mi>−</mi><mi>∞</mi></mrow></math>"#;
    test_braille("French", expr, "⠇⠊⠍⠢⠢⠰⠭⠸⠱⠹⠂⠭⠐⠜⠹⠆⠋⠦⠭⠴⠶⠤⠘⠉");
}

#[test]
fn test_14_2_02_corrected_2() {
    let expr= r#"<math  xmlns="http://www.w3.org/1998/Math/MathML"><mrow><munder><munder><mi>lim</mi><mrow><mi>x</mi><mo>→</mo><mn>4</mn></mrow></munder><mrow><mi>x</mi><mo>&gt;</mo><mn>4</mn></mrow></munder><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mo>=</mo><mi>−</mi><mi>∞</mi></mrow></math>"#;
    test_braille("French", expr, "⠇⠊⠍⠢⠢⠰⠭⠸⠱⠹⠂⠭⠐⠜⠹⠆⠋⠦⠭⠴⠶⠤⠘⠉");
}

#[test]
fn test_14_2_03_corrected() {
    let expr= r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mrow><munder><mi>lim</mi><mrow><mi>x</mi><mo>→</mo><mi>+</mi><mi>∞</mi></mrow></munder><mfrac><mrow><mi>ln</mi><mo>⁡</mo><mo stretchy="false">(</mo><mi>x</mi><mo>+</mo><mn>2</mn><mo stretchy="false">)</mo></mrow><mi>x</mi></mfrac><mo>=</mo><mn>0</mn></mrow></math>"#;
    test_braille("French", expr, "⠇⠊⠍⠢⠢⠰⠭⠸⠱⠖⠘⠉⠆⠰⠇⠝⠦⠭⠖⠣⠴⠌⠭⠆⠶⠼");
}

#[test]
fn test_14_4_01() {
    let expr= r#"<math><mrow><msubsup><mo>∫</mo><mi>a</mi><mi>b</mi></msubsup><mrow><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mi>d</mi><mi>x</mi></mrow></mrow></math>"#;
    test_braille("French", expr, "⠯⠢⠁⠈⠃⠋⠦⠭⠴⠙⠭");
}

#[test]
fn test_14_4_03() {
    let expr= r#"<math><mrow><msubsup><mo>∫</mo><mn>1</mn><mn>2</mn></msubsup><mrow><mn>5</mn><mi>x</mi><mi>d</mi><mi>x</mi></mrow></mrow></math>"#;
    test_braille("French", expr, "⠯⠢⠡⠈⠣⠰⠱⠭⠆⠙⠭");
}

#[test]
fn test_14_4_04() {
    let expr= r#"<math><mrow><msubsup><mo>∫</mo><mrow><mo stretchy="false">(</mo><mi>c</mi><mo stretchy="false">)</mo></mrow><mrow></mrow></msubsup><mrow><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo stretchy="false">)</mo><mi>d</mi><mi>x</mi></mrow><mspace width="0.167em"></mspace></mrow></math>"#;
    test_braille("French", expr, "⠯⠢⠦⠨⠉⠴⠋⠦⠭⠴⠙⠭");
}

#[test]
fn test_14_4_05() {
    let expr= r#"<math><mrow><msubsup><mo>∯</mo><mi>S</mi><mrow></mrow></msubsup><mrow><mspace width="0.167em"></mspace><mi>f</mi><mo stretchy="false">(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo stretchy="false">)</mo><mi>d</mi><mi>x</mi><mi>d</mi><mi>y</mi></mrow></mrow></math>"#;
    test_braille("French", expr, "⠨⠯⠯⠢⠨⠎⠋⠦⠭⠂⠽⠴⠙⠭⠙⠽");
}

#[test]
fn test_14_5_01() {
    let expr= r#"<math><mrow><mi>ln</mi><mo>⁡</mo><mrow><mo>(</mo><mfrac><mi>a</mi><mi>b</mi></mfrac><mo>)</mo></mrow></mrow></math>"#;
    test_braille("French", expr, "⠇⠝⠦⠁⠌⠃⠴");
}

#[test]
fn test_14_5_02() {
    let expr= r#"<math><mrow><mi>ln</mi><mo>⁡</mo><mspace width="0.333em"></mspace><mi>x</mi><mi>y</mi><mo>=</mo><mi>ln</mi><mo>⁡</mo><mspace width="0.333em"></mspace><mi>x</mi><mo>+</mo><mi>ln</mi><mo>⁡</mo><mspace width="0.333em"></mspace><mi>y</mi></mrow></math>"#;
    test_braille("French", expr, "⠇⠝⠰⠭⠽⠆⠶⠇⠝⠰⠭⠆⠖⠇⠝⠰⠽⠆");
}

#[test]
fn test_14_5_03() {
    let expr= r#"<math><mrow><mi>l</mi><mi>o</mi><msub><mi>g</mi><mn>7</mn></msub><mspace width="0.333em"></mspace><mi>x</mi><mo>=</mo><mfrac><mrow><mi>ln</mi><mo>⁡</mo><mspace width="0.333em"></mspace><mi>x</mi></mrow><mrow><mi>ln</mi><mo>⁡</mo><mspace width="0.333em"></mspace><mn>7</mn></mrow></mfrac></mrow></math>"#;
    test_braille("French", expr, "⠇⠕⠛⠢⠻⠰⠭⠆⠶⠇⠝⠰⠭⠆⠌⠇⠝⠰⠻⠆");
}

#[test]
fn test_14_6_01() {
    let expr= r#"<math><mrow><mi>cos</mi><mo>⁡</mo><mo stretchy="false">(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo stretchy="false">)</mo><mo>=</mo><mi>c</mi><mi>o</mi><mi>s</mi><mi>a</mi><mi>c</mi><mi>o</mi><mi>s</mi><mi>b</mi><mo>−</mo><mi>s</mi><mi>i</mi><mi>n</mi><mi>a</mi><mi>s</mi><mi>i</mi><mi>n</mi><mi>b</mi></mrow></math>"#;
    test_braille("French", expr, "⠉⠕⠎⠦⠁⠖⠃⠴⠶⠉⠕⠎⠰⠁⠆⠉⠕⠎⠰⠃⠆⠐⠤⠎⠊⠝⠰⠁⠆⠎⠊⠝⠰⠃⠆");
}

#[test]
fn test_14_6_01b() {
    let expr= r#"<math><mrow><mi>cos</mi><mo>⁡</mo><mo stretchy="false">(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo stretchy="false">)</mo><mo>=</mo><mi>c</mi><mi>o</mi><mi>s</mi><mi>a</mi><mi>c</mi><mi>o</mi><mi>s</mi><mi>b</mi><mo>−</mo><mi>s</mi><mi>i</mi><mi>n</mi><mi>a</mi><mi>s</mi><mi>i</mi><mi>n</mi><mi>b</mi></mrow></math>"#;
    test_braille("French", expr, "⠨⠬⠦⠁⠖⠃⠴⠶⠨⠬⠁⠨⠬⠃⠤⠬⠁⠬⠃");
}

#[test]
fn test_14_7_01() {
    let expr= r#"<math><mrow><msup><mrow><mi>cos</mi><mo>⁡</mo><mi>h</mi></mrow><mn>2</mn></msup><mi>x</mi><mo>−</mo><msup><mrow><mi>sin</mi><mo>⁡</mo><mi>h</mi></mrow><mn>2</mn></msup><mi>x</mi><mo>=</mo><mn>1</mn></mrow></math>"#;
    test_braille("French", expr, "⠉⠓⠈⠣⠰⠭⠆⠤⠎⠓⠈⠣⠰⠭⠆⠶⠡");
}

#[test]
fn test_15_01() {
    let expr= r#"<math><mrow><mo>∃</mo><mi>x</mi><mo>:</mo><mspace width="0.333em"></mspace><mi>x</mi><mo>∈</mo><mi>A</mi></mrow></math>"#;
    test_braille("French", expr, "⠸⠡⠭⠒⠭⠘⠡⠨⠁");
}

#[test]
fn test_16_4_07() {
    let expr= r#"<math><mrow><mo stretchy="false">(</mo><mi>p</mi><mo>+</mo><mi>q</mi><msup><mo stretchy="false">)</mo><mrow><mi>i</mi><mi>è</mi><mi>m</mi><mi>e</mi></mrow></msup></mrow></math>"#;
    test_braille("French", expr, "⠦⠏⠖⠟⠴⠈⠊⠮⠍");
}

