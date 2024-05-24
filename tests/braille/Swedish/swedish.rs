// Swedish braille tests for the basic mathml tags
// These tests are from the Swedish braille authority's publication "Punktskriftens skrivregeler för matematik och naturvetenskap"
//  https://www.mtm.se/globalassets/punktskriftsnamnden/punktskriftens_skrivregler_matematik.pdf
use crate::common::*;

#[test]
fn ex_1_1 {
    let expr = "<math><mn>6</mn><mo>=</mo><mn>1</mn><mo>&#xD7;</mo><mn>2</mn><mo>&#xD7;</mo><mn>3</mn>
                <mo>=</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>+</mo><mn>3</mn></math>";
    test_braille("Swedish", expr, "⠼⠋⠀⠐⠶⠀⠼⠁⠐⠦⠼⠃⠐⠦⠼⠉⠀⠐⠶⠀⠼⠁⠐⠖⠼⠃⠐⠖⠼⠉");
}
#[test]
fn ex_1_1 {
    let expr = "<math><mn>6</mn><mo>=</mo><mn>1</mn><mo>&#xD7;</mo><mn>2</mn><mo>&#xD7;</mo><mn>3</mn>
                <mo>=</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>+</mo><mn>3</mn></math>";
    test_braille("Swedish", expr, "p3456;p124;⠀⠐⠶⠀⠼⠁⠐⠦⠼⠃⠐⠦⠼⠉⠀⠐⠶⠀⠼⠁⠐⠖⠼⠃⠐⠖⠼⠉");
}
