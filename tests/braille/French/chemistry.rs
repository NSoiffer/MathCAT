// French braille tests for the basic mathml tags
// These tests are from the French braille authority's publication "NOTATION PIATHÉMATIQUE BRAILLE" (Première édition janvier 2007)
//  https://www.avh.asso.fr/sites/default/files/notation_mathematique_braille2_0.pdf
use crate::common::*;

#[test]
fn bond_I_4_1() {
    let expr= r#"<math><mi mathvariant="normal">H</mi><mo>-</mo><mi mathvariant="normal">H</mi></math>"#;
    test_braille("French", expr, "⠨⠓⠤⠨⠓");
}

#[test]
fn bond_I_4_2() {
    let expr= r#"<math><mi mathvariant="normal">O</mi><mo>=</mo><mi mathvariant="normal">O</mi></math>"#;
    test_braille("French", expr, "⠨⠕⠶⠨⠕");
}

#[test]
fn ion_II_1_3() {
    let expr= r#"<math><msubsup><mi>SO</mi><mn>4</mn><mrow><mn>2</mn><mo>-</mo></mrow></msubsup></math>"#;
    test_braille("French", expr, "⠨⠎⠨⠕⠢⠹⠈⠰⠣⠤⠆");
}

#[test]
fn equilibrium_III_5_1() {
    let expr= r#"<math><msub><mi mathvariant="normal">N</mi><mn>2</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo><mo>+</mo><mn>3</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo><mo>&#x21CC;</mo><mn>2</mn><msub><mi>NH</mi><mn>3</mn></msub><mo>(</mo><mi mathvariant="normal">g</mi><mo>)</mo></math>"#;
    test_braille("French", expr, "⠨⠝⠣⠦⠛⠴⠖⠩⠨⠓⠣⠦⠛⠴⠸⠻⠣⠨⠝⠨⠓⠩⠦⠛⠴");
}

#[test]
fn equilibrium_III_5_2() {
    let expr= r#"<math><mn>2</mn><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi>
        <munderover><mo>&#x21CC;</mo><mrow><mo>(</mo><mn>2</mn><mo>)</mo></mrow><mrow><mo>(</mo><mn>1</mn><mo>)</mo></mrow></munderover
        ><msub><mi mathvariant="normal">H</mi><mn>3</mn></msub><msup><mi mathvariant="normal">O</mi><mo>+</mo></msup><mo>+</mo><msup><mi>OH</mi><mo>-</mo></msup></math>"#;
    test_braille("French", expr, " ⠣⠨⠓⠣⠨⠕⠰⠸⠻⠆⠢⠢⠦⠣⠴⠈⠈⠦⠡⠴⠨⠓⠩⠨⠕⠈⠰⠖⠆⠖⠨⠕⠨⠓⠈⠰⠤⠆");
}

#[test]
fn equilibrium_III_5_3() {
    let expr= r#"<math><mi mathvariant="normal">R</mi><mo>-</mo><mi>CO</mi><mo>-</mo><mi mathvariant="normal">O</mi><mo>-</mo>
                             <mi mathvariant="normal">H</mi><mo>+</mo><mi mathvariant="normal">R</mi><mo>'</mo><mo>-</mo><mi>OH</mi>
                    <munderover><mo>&#x21CC;</mo><mi>hydrolyse</mi><mi>est&#xE9;rification</mi></munderover>
                    <mi mathvariant="normal">R</mi><mo>-</mo><mi>CO</mi><mo>-</mo><mi mathvariant="normal">O</mi><mo>-</mo>
                    <mi mathvariant="normal">R</mi><mo>'</mo><mo>+</mo>
                            <msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">O</mi></math>"#;
    test_braille("French", expr, " ⠨⠗⠤⠨⠉⠨⠕⠤⠨⠕⠤⠨⠓⠖⠨⠗⠄⠤⠨⠕⠨⠓  ⠸⠻⠨⠢⠰⠓⠽⠙⠗⠕⠇⠽⠎⠑⠆⠨⠔⠈⠰⠑⠎⠞⠗⠊⠋⠊⠉⠁⠞⠊⠕⠝⠆⠨⠗⠤⠨⠉⠨⠕⠤⠨⠕⠤⠨⠗⠄⠖⠨⠓⠢⠣⠨⠕");
}
