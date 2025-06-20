/// Tests for table properties
use crate::common::*;

#[test]
fn array() {
    let mathml = r#"<math><mfenced open="[" close="]"><mtable intent=":array">
            <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr>
            <mtr><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd><mtd><mn>6</mn></mtd></mtr>
        </mtable></mfenced></math>"#;
    let intent = " <math data-from-mathml='math'>
                        <array data-from-mathml='mtable' data-intent-property=':array:'>
                            <mtr data-from-mathml='mtr'>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>1</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>2</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>3</mn></mtd></mtr>
                            <mtr data-from-mathml='mtr'>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>4</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>5</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>6</mn></mtd>
                            </mtr>
                        </array>
                    </math>";
    test_intent(mathml, intent, vec![]);
}

#[test]
fn matrix_mtable_intent() {
    let mathml = r#"<math><mfenced open="[" close="]"><mtable intent=":matrix">
            <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr>
            <mtr><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd><mtd><mn>6</mn></mtd></mtr>
        </mtable></mfenced></math>"#;
    let intent = " <math data-from-mathml='math'>
                        <matrix data-from-mathml='mtable' data-intent-property=':matrix:'>
                            <mtr data-from-mathml='mtr'>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>1</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>2</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>3</mn></mtd></mtr>
                            <mtr data-from-mathml='mtr'>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>4</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>5</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>6</mn></mtd>
                            </mtr>
                        </matrix>
                    </math>";
    test_intent(mathml, intent, vec![]);
}

#[test]
fn matrix_mrow_intent() {
    let mathml = r#"<math><mfenced open="[" close="]" intent="$t"><mtable arg="t" intent=":matrix">
            <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr>
            <mtr><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd><mtd><mn>6</mn></mtd></mtr>
        </mtable></mfenced></math>"#;
    let intent = " <math data-from-mathml='math'>
                        <matrix data-from-mathml='mtable' arg='t' data-intent-property=':matrix:'>
                            <mtr data-from-mathml='mtr'>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>1</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>2</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>3</mn></mtd></mtr>
                            <mtr data-from-mathml='mtr'>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>4</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>5</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>6</mn></mtd>
                            </mtr>
                        </matrix>
                    </math>";
    test_intent(mathml, intent, vec![]);
}

#[test]
fn matrix_infer_intent() {
    let mathml = r#"<math><mfenced open="[" close="]" ><mtable>
            <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr>
            <mtr><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd><mtd><mn>6</mn></mtd></mtr>
        </mtable></mfenced></math>"#;
    let intent = " <math data-from-mathml='math'>
                        <matrix data-from-mathml='mtable'>
                            <mtr data-from-mathml='mtr'>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>1</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>2</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>3</mn></mtd></mtr>
                            <mtr data-from-mathml='mtr'>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>4</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>5</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>6</mn></mtd>
                            </mtr>
                        </matrix>
                    </math>";
    test_intent(mathml, intent, vec![]);
}

#[test]
fn determinant_not_matrix() {
    let mathml = r#"<math><mfenced open="[" close="]" ><mtable intent=":determinant">
            <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr>
            <mtr><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd></mtr>
        </mtable></mfenced></math>"#;
    let intent = " <math data-from-mathml='math'>
                        <determinant data-from-mathml='mtable' data-intent-property=':determinant:'>
                            <mtr data-from-mathml='mtr'>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>1</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>2</mn></mtd>
                            </mtr>
                            <mtr data-from-mathml='mtr'>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>4</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>5</mn></mtd>
                            </mtr>
                        </determinant>
                    </math>";
    test_intent(mathml, intent, vec![]);
}

#[test]
fn determinant_infer_intent() {
    let mathml = r#"<math><mfenced open="|" close="|" ><mtable>
            <mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr>
            <mtr><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd><mtd><mn>6</mn></mtd></mtr>
        </mtable></mfenced></math>"#;
    let intent = " <math data-from-mathml='math'>
                        <determinant data-from-mathml='mtable'>
                            <mtr data-from-mathml='mtr'>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>1</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>2</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>3</mn></mtd></mtr>
                            <mtr data-from-mathml='mtr'>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>4</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>5</mn></mtd>
                                <mtd data-from-mathml='mtd'><mn data-from-mathml='mn'>6</mn></mtd>
                            </mtr>
                        </determinant>
                    </math>";
    test_intent(mathml, intent, vec![]);
}

#[test]
fn system_of_equations() {
    let mathml = r#"<math>
            <mtable intent=':system-of-equations' columnalign="right left" columnspacing="0em" rowspacing="3pt">
                <mtr>
                    <mtd><mi>x</mi><mo>+</mo><mi>y</mi></mtd>
                    <mtd><mi/><mo>is equal to</mo><mi>y</mi><mo>+</mo><mn>1</mn></mtd>
                </mtr>
                <mtr>
                    <mtd><mi>x</mi><mo>&#x2212;</mo><mi>y</mi></mtd>
                    <mtd><mi/><mo>is equal to</mo><mn>4</mn></mtd>
                </mtr>
            </mtable>
            </math>"#;
    let intent = "<math data-from-mathml='math'>
                <system-of-equations data-from-mathml='mtable' columnalign='right left' columnspacing='0em' rowspacing='3pt' data-intent-property=':system-of-equations:'>
                    <mtr data-from-mathml='mtr'>
                    <mtd data-from-mathml='mtr'>
                        <mrow data-from-mathml='mtr'>
                        <mrow data-from-mathml='mrow' data-changed='added'>
                            <mi data-from-mathml='mi'>x</mi>
                            <mo data-from-mathml='mo'>+</mo>
                            <mi data-from-mathml='mi'>y</mi>
                        </mrow>
                        <mrow data-from-mathml='mrow' data-changed='added'>
                            <mo data-from-mathml='mo'>is equal to</mo>
                            <mrow data-from-mathml='mrow' data-changed='added'>
                            <mi data-from-mathml='mi'>y</mi>
                            <mo data-from-mathml='mo'>+</mo>
                            <mn data-from-mathml='mn'>1</mn>
                            </mrow>
                        </mrow>
                        </mrow>
                    </mtd>
                    </mtr>
                    <mtr data-from-mathml='mtr'>
                    <mtd data-from-mathml='mtr'>
                        <mrow data-from-mathml='mtr'>
                        <mrow data-from-mathml='mrow' data-changed='added'>
                            <mi data-from-mathml='mi'>x</mi>
                            <mo data-from-mathml='mo'>-</mo>
                            <mi data-from-mathml='mi'>y</mi>
                        </mrow>
                        <mrow data-from-mathml='mrow' data-changed='added'>
                            <mo data-from-mathml='mo'>is equal to</mo>
                            <mn data-from-mathml='mn'>4</mn>
                        </mrow>
                        </mrow>
                    </mtd>
                    </mtr>
                </system-of-equations>
            </math>";
    test_intent(mathml, intent, vec![]);
}

#[test]
fn system_of_equations_infer_intent() {
    let mathml = r#"<math>
            <mtable intent=":system-of-equations" columnalign="right left" columnspacing="0em" rowspacing="3pt">
                <mtr>
                    <mtd><mi>x</mi><mo>+</mo><mi>y</mi></mtd>
                    <mtd><mi/><mo>=</mo><mi>y</mi><mo>+</mo><mn>1</mn></mtd>
                </mtr>
                <mtr>
                    <mtd><mi>x</mi><mo>&#x2212;</mo><mi>y</mi></mtd>
                    <mtd><mi/><mo>=</mo><mn>4</mn></mtd>
                </mtr>
            </mtable>
            </math>"#;
    let intent = "<math data-from-mathml='math'>
                <system-of-equations data-from-mathml='mtable' columnalign='right left' columnspacing='0em' rowspacing='3pt' data-intent-property=':system-of-equations:'>
                    <mtr data-from-mathml='mtr'>
                    <mtd data-from-mathml='mtr'>
                        <mrow data-from-mathml='mtr'>
                        <mrow data-from-mathml='mrow' data-changed='added'>
                            <mi data-from-mathml='mi'>x</mi>
                            <mo data-from-mathml='mo'>+</mo>
                            <mi data-from-mathml='mi'>y</mi>
                        </mrow>
                        <mrow data-from-mathml='mrow' data-changed='added'>
                            <mo data-from-mathml='mo'>=</mo>
                            <mrow data-from-mathml='mrow' data-changed='added'>
                            <mi data-from-mathml='mi'>y</mi>
                            <mo data-from-mathml='mo'>+</mo>
                            <mn data-from-mathml='mn'>1</mn>
                            </mrow>
                        </mrow>
                        </mrow>
                    </mtd>
                    </mtr>
                    <mtr data-from-mathml='mtr'>
                    <mtd data-from-mathml='mtr'>
                        <mrow data-from-mathml='mtr'>
                        <mrow data-from-mathml='mrow' data-changed='added'>
                            <mi data-from-mathml='mi'>x</mi>
                            <mo data-from-mathml='mo'>-</mo>
                            <mi data-from-mathml='mi'>y</mi>
                        </mrow>
                        <mrow data-from-mathml='mrow' data-changed='added'>
                            <mo data-from-mathml='mo'>=</mo>
                            <mn data-from-mathml='mn'>4</mn>
                        </mrow>
                        </mrow>
                    </mtd>
                    </mtr>
                </system-of-equations>
            </math>";
    test_intent(mathml, intent, vec![]);
}

#[test]
fn lines() {
    let mathml = r#"<math>
            <mtable intent=":lines">
                <mtr>
                    <mtd><mi>x</mi><mo>+</mo><mi>y</mi></mtd>
                    <mtd><mi/><mo>=</mo><mi>y</mi><mo>+</mo><mn>1</mn></mtd>
                </mtr>
                <mtr>
                    <mtd><mi>x</mi><mo>&#x2212;</mo><mi>y</mi></mtd>
                    <mtd><mi/><mo>=</mo><mn>4</mn></mtd>
                </mtr>
            </mtable>
            </math>"#;
    let intent = "<math data-from-mathml='math'>
                <lines data-from-mathml='mtable' data-intent-property=':lines:'>
                    <mtr data-from-mathml='mtr'>
                    <mtd data-from-mathml='mtr'>
                        <mrow data-from-mathml='mtr'>
                        <mrow data-from-mathml='mrow' data-changed='added'>
                            <mi data-from-mathml='mi'>x</mi>
                            <mo data-from-mathml='mo'>+</mo>
                            <mi data-from-mathml='mi'>y</mi>
                        </mrow>
                        <mrow data-from-mathml='mrow' data-changed='added'>
                            <mo data-from-mathml='mo'>=</mo>
                            <mrow data-from-mathml='mrow' data-changed='added'>
                            <mi data-from-mathml='mi'>y</mi>
                            <mo data-from-mathml='mo'>+</mo>
                            <mn data-from-mathml='mn'>1</mn>
                            </mrow>
                        </mrow>
                        </mrow>
                    </mtd>
                    </mtr>
                    <mtr data-from-mathml='mtr'>
                    <mtd data-from-mathml='mtr'>
                        <mrow data-from-mathml='mtr'>
                        <mrow data-from-mathml='mrow' data-changed='added'>
                            <mi data-from-mathml='mi'>x</mi>
                            <mo data-from-mathml='mo'>-</mo>
                            <mi data-from-mathml='mi'>y</mi>
                        </mrow>
                        <mrow data-from-mathml='mrow' data-changed='added'>
                            <mo data-from-mathml='mo'>=</mo>
                            <mn data-from-mathml='mn'>4</mn>
                        </mrow>
                        </mrow>
                    </mtd>
                    </mtr>
                </lines>
            </math>";
    test_intent(mathml, intent, vec![]);
}
