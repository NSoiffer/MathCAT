// These are additional Nemeth tests from other sources, mainly from bugs/issues
use crate::common::*;


#[test]
fn not_number_space_blocks() {
    // https://github.com/NSoiffer/MathCAT/issues/144
    let expr = "<math><mn>123</mn><mtext>&nbsp;&#x2063;</mtext><mn>456</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠆⠒⠀⠼⠲⠢⠖");
}

#[test]
fn space_between_digits() {
    // https://github.com/NSoiffer/MathCAT/issues/144
    let expr = "<math><mn>1</mn><mtext>&#x00a0;</mtext><mn>3</mn><mtext>&#x00a0;</mtext><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠀⠼⠒⠀⠼⠢");
}

#[test]
fn space_hack_between_digits() {
    // https://github.com/NSoiffer/MathCAT/issues/144
    let expr = "<math><mn>1</mn><mtext>&#x00a0;&#x2063;</mtext><mn>3</mn><mtext>&#x00a0;&#x2063;</mtext><mn>5</mn></math>";
    test_braille("Nemeth", expr, "⠼⠂⠀⠼⠒⠀⠼⠢");
}

#[test]
fn tilde_prefix_bug_244() {
    // https://github.com/NSoiffer/MathCAT/issues/244
    let expr = "<math> <mo>~</mo> <mi>p</mi> </math>";
    test_braille("Nemeth", expr, "⠈⠱⠏");
}

#[test]
fn double_struck_bug_334() {
    // https://github.com/NSoiffer/MathCAT/issues/334 -- double struck was problem (⠼ was missing); test all of the scripted numbers here
    let expr = "<math><mn>𝟙</mn><mo>,</mo><mn>𝟐</mn><mo>,</mo><mn>𝟯</mn><mo>,</mo><mn>𝟺</mn></math>";
    test_braille("Nemeth", expr, "⠨⠼⠂⠠⠀⠸⠼⠆⠠⠀⠠⠨⠸⠼⠒⠠⠀⠼⠲");
}

#[test]
fn extra_indicators_bug_343() {
    // https://github.com/NSoiffer/MathCAT/issues/343 -- extra indicators before baseline indicator due to -x^2, not there for x^2
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'>
                        <mrow>
                        <msup>
                            <mi>e</mi>
                            <mrow>
                            <mo>-</mo>
                            <msup>
                                <mi>x</mi>
                                <mn>2</mn>
                            </msup>
                            </mrow>
                        </msup>
                        <mo>+</mo>
                        <msub>
                            <mi>C</mi>
                            <mn>1</mn>
                        </msub>
                        </mrow>
                    </math>";
    test_braille("Nemeth", expr, "⠑⠘⠤⠭⠘⠘⠆⠐⠬⠠⠉⠂");
}


#[test]
fn find_baseline_indicator_bug_364() {
    use libmathcat::interface::*;

    // https://github.com/NSoiffer/MathCAT/issues/343 -- extra indicators before baseline indicator due to -x^2, not there for x^2
    let expr = r#" <math id='id-0'>
        <mrow data-changed='added' id='id-1'>
            <mi id='id-2'>π</mi>
            <mo id='id-3'>&#x2062;</mo>
            <msup id='id-4'>
                <mi id='id-5'>r</mi>
                <mn id='id-6'>2</mn>
            </msup>
        </mrow>
    </math>"#;
    set_rules_dir(abs_rules_dir_path()).unwrap();
    set_preference("BrailleNavHighlight".to_string(), "Off".to_string()).unwrap();
    set_preference("BrailleCode".to_string(), "Nemeth".to_string()).unwrap();
    if let Err(e) = set_mathml(expr.to_string()) {
        panic!("{}", errors_to_string(&e));
    };
    match get_navigation_node_from_braille_position(4) {
        Ok((node_id, offset)) => {
            assert_eq!(&node_id, "id-6");
            assert_eq!(offset, 0);
        }
        Err(e) => panic!("{}", errors_to_string(&e)),
    };
}

