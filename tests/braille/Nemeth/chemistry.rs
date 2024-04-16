// Nemeth tests from the 2023 chemistry spec
//  https://www.brailleauthority.org/sites/default/files/chemistry/Chemical%20Notation%20Using%20the%20Nemeth%20Braille%20Code%202023.pdf
// The numbering refers to the sections in that reference.
use crate::common::*;

#[test]
fn bond_2_1() {
    let expr = r#"<math><mi mathvariant="normal">H</mi><mo>-</mo><mi>Be</mi><mo>-</mo><mi mathvariant="normal">H</mi></math>"#;
    test_braille("Nemeth", expr, "⠠⠓⠸⠒⠻⠠⠃⠑⠸⠒⠻⠠⠓");
}

#[test]
fn bond_2_2() {
    init_logger();
    let expr = r#"<math><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub><mi mathvariant="normal">C</mi><mo>=</mo>
                        <msub><mi>CH</mi><mn>2</mn></msub></math>"#;
    test_braille("Nemeth", expr, "⠠⠓⠆⠠⠉⠸⠶⠻⠠⠉⠠⠓⠆");
}

#[test]
fn bond_2_3() {
    let expr = r#"<math><mi mathvariant="normal">H</mi><mo>-</mo><mi mathvariant="normal">C</mi><mo>&#x2261;</mo>
                        <mi mathvariant="normal">C</mi><mo>-</mo><mi mathvariant="normal">H</mi></math>"#;
    test_braille("Nemeth", expr, "⠠⠓⠸⠒⠻⠠⠉⠸⠿⠻⠠⠉⠸⠒⠻⠠⠓");
}

#[test]
fn bond_2_5() {
    let expr = r#"<math><mi>Fe</mi><mo>+</mo><msub><mi>Cl</mi><mn>2</mn></msub><mo>&#xA0;</mo><mo>=</mo>
                            <msub><mi>FeCl</mi><mn>3</mn></msub></math>"#;
    test_braille("Nemeth", expr, "⠠⠋⠑⠬⠠⠉⠇⠆⠀⠨⠅⠀⠠⠋⠑⠠⠉⠇⠒");
}

