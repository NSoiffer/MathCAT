// Marburg tests
// Most of these tests come from 
//   https://ore.edu.pl/images/files/pdf/Brajlowska%20notacja%20matematyczna%20fizyczna%20chemiczna%20wyd%20II.pdf
use crate::common::*;

#[test]
fn Intro_1() {
    let expr = r#"<math><msqrt><mn>16</mn></msqrt></math>"#;
    test_braille("Marburg", expr, "⠩⠼⠁⠋");
}

#[test]
fn Intro_2() {
    let expr = r#"<math><msqrt><mn>81</mn></msqrt><mo>=</mo><mn>9</mn></math>"#;
    test_braille("Marburg", expr, "⠩⠼⠓⠁⠀⠶⠼⠊");
}

#[test]
fn Intro_3() {
    let expr = r#"<math><mroot><mn>27</mn><mn>3</mn></mroot><mo>=</mo><mn>3</mn></math>"#;
    test_braille("Marburg", expr, "⠌⠒⠩⠼⠃⠛⠀⠶⠼⠉");
}

#[test]
fn Intro_4() {
    let expr = r#"<math><mroot><mn>27</mn><mn>3</mn></mroot><mo>=</mo><mn>3</mn></math>"#;
    test_braille("Marburg", expr, "⠌⠒⠩⠼⠃⠛⠀⠶⠼⠉");
}

#[test]
fn decimal_numbers_1() {
    let expr = r#"<math><mn>7</mn><mo>,</mo><mn>29</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠛⠃⠊");
}

#[test]
fn decimal_numbers_2() {
    let expr = r#"<math><mn>0</mn><mo>,</mo><mn>072</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠚⠂⠚⠛⠃");
}

#[test]
fn decimal_numbers_3() {
    let expr = r#"<math><mn>50</mn><mo>,</mo><mn>347</mn><mo>.</mo><mn>296</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠑⠚⠂⠉⠙⠛⠄⠃⠊⠋");
}

#[test]
fn decimal_numbers_4() {
    let expr = r#"<math><mn>0</mn><mo>,</mo><mn>333</mn><mo>.</mo><mo>.</mo><mo>.</mo><mo>=</mo><mn>0</mn><mo>,</mo><mo>(</mo><mn>3</mn><mo>)</mo></math>"#;
    test_braille("Marburg", expr, "⠼⠚⠂⠉⠉⠉⠄⠄⠄⠀⠶⠼⠚⠂⠣⠼⠉⠜");
}

#[test]
fn percents_and_promiles_1() {
    let expr = r#"<math><mn>0</mn><mo>,</mo><mn>25</mn><mo>=</mo><mn>25</mn><mo>%</mo></math>"#;
    test_braille("Marburg", expr, "⠼⠚⠂⠃⠑⠀⠶⠼⠃⠑⠼⠚⠴");
}

#[test]
fn percents_and_promiles_2() {
    let expr = r#"<math><mn>48</mn><mo>%</mo><mo>=</mo><mn>480</mn><mo>&#x2030;</mo></math>"#;
    test_braille("Marburg", expr, "⠼⠙⠓⠼⠚⠴⠀⠶⠼⠙⠓⠚⠼⠚⠴⠴");
}

#[test]
fn units_of_measurement_1() {
    let expr = r#"<math><mn>1</mn><mi>m</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠻⠍");
}

#[test]
fn units_of_measurement_2() {
    let expr = r#"<math><mn>1</mn><mi>k</mi><mi>m</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠻⠅⠍");
}

#[test]
fn units_of_measurement_3() {
    let expr = r#"<math><mn>5</mn><mfrac><mi>m</mi><mi>s</mi></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠑⠻⠍⠳⠎");
}

#[test]
fn units_of_measurement_4() {
    let expr = r#"<math><mn>230</mn><mi>V</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠉⠚⠻⠨⠧");
}

#[test]
fn units_of_measurement_5() {
    let expr = r#"<math><mn>2</mn><mi>m</mi><mi>i</mi><mi>n</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠻⠍⠊⠝");
}

#[test]
fn units_of_measurement_6() {
    let expr = r#"<math><mn>5</mn><mi>N</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠑⠻⠨⠝");
}

#[test]
fn units_of_measurement_7() {
    let expr = r#"<math><mn>2</mn><mo>,</mo><mn>5</mn><msup><mi>m</mi><mn>2</mn></msup></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠂⠑⠻⠍⠬⠆");
}

#[test]
fn units_of_measurement_8() {
    let expr = r#"<math><mn>2</mn><mo>&#xA0;</mo><mi>z</mi><mi>&#x142;</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠀⠵⠣");
}

#[test]
fn units_of_measurement_9() {
    let expr = r#"<math><mn>2</mn><mo>&#xA0;</mo><mi>P</mi><mi>L</mi><mi>N</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠀⠨⠏⠨⠇⠨⠝");
}

#[test]
fn units_of_measurement_10() {
    let expr = r#"<math><mn>5</mn><mo>&#xA0;</mo><mi>z</mi><mi>&#x142;</mi><mo>&#xA0;</mo><mn>50</mn><mo>&#xA0;</mo><mi>g</mi><mi>r</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠑⠀⠵⠣⠀⠼⠑⠚⠀⠛⠗");
}

#[test]
fn units_of_measurement_11() {
    let expr = r#"<math><mn>5</mn><mo>,</mo><mn>50</mn><mo>&#xA0;</mo><mi>z</mi><mi>&#x142;</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠑⠂⠑⠚⠀⠵⠣");
}

#[test]
fn units_of_measurement_12() {
    let expr = r#"<math><mn>2</mn><mo>&#xA0;</mo><mo>&#x20AC;</mo></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠀⠈⠑");
}

#[test]
fn date_1() {
    let expr = r#"<math><mn>15</mn><mo>.</mo><mn>03</mn><mo>.</mo><mn>2002</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠑⠄⠚⠉⠄⠃⠚⠚⠃");
}

#[test]
fn date_2() {
    let expr = r#"<math><mn>98</mn><mo>/</mo><mn>08</mn><mo>/</mo><mn>26</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠊⠓⠲⠚⠓⠲⠃⠋");
}

#[test]
fn date_3() {
    let expr = r#"<math><mn>2002</mn><mo>-</mo><mn>03</mn><mo>-</mo><mn>15</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠚⠚⠃⠤⠚⠉⠤⠁⠑");
}

#[test]
fn date_4() {
    let expr = r#"<math><mn>15</mn><mo>&#xA0;</mo><mi>II</mi><mo>&#xA0;</mo><mn>2011</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠑⠀⠨⠊⠊⠀⠼⠃⠚⠁⠁");
}

#[test]
fn time_1() {
    let expr = r#"<math><msup><mn>0</mn><mn>20</mn></msup></math>"#;
    test_braille("Marburg", expr, "⠼⠼⠚⠄⠃⠚");
}

#[test]
fn time_2() {
    let expr = r#"<math><mn>05</mn><mo>:</mo><mn>40</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠚⠑⠒⠙⠚");
}

#[test]
fn time_3() {
    let expr = r#"<math><mn>18</mn><mo>.</mo><mn>25</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠓⠄⠃⠑");
}

#[test]
fn signs_of_action_1() {
    let expr = r#"<math><mn>5</mn><mo>+</mo><mi>x</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠑⠀⠖⠠⠭");
}

#[test]
fn signs_of_action_2() {
    let expr = r#"<math><mn>67</mn><mo>:</mo><mn>14</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠋⠛⠀⠲⠼⠁⠙");
}

#[test]
fn signs_of_action_3() {
    let expr = r#"<math><mn>24</mn><mo>,</mo><mn>6</mn><mo>+</mo><mn>2</mn><mo>-</mo><mn>4</mn><mo>,</mo><mn>8</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠙⠂⠋⠀⠖⠼⠃⠀⠤⠼⠙⠂⠓");
}

#[test]
fn signs_of_action_4() {
    let expr = r#"<math><mn>12</mn><mo>&#xB7;</mo><mn>3</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠃⠄⠼⠉");
}

#[test]
fn relations_1() {
    let expr = r#"<math><mn>7</mn><mo>-</mo><mn>4</mn><mo>=</mo><mn>3</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠛⠀⠤⠼⠙⠀⠶⠼⠉");
}

#[test]
fn relations_2() {
    let expr = r#"<math><mn>2</mn><mo>&#xB7;</mo><mn>5</mn><mo>&lt;</mo><mn>47</mn><mo>:</mo><mn>3</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠄⠼⠑⠀⠪⠄⠼⠙⠛⠀⠲⠼⠉");
}

#[test]
fn brackets_1() {
    let expr = r#"<math><mo>(</mo><mn>14</mn><mo>-</mo><mn>5</mn><mo>)</mo><mo>+</mo><mn>7</mn><mo>&gt;</mo><mo>-</mo><mo>[</mo><mn>1</mn><mo>+</mo><mn>2</mn><mo>(</mo><mn>5</mn><mo>+</mo><mn>3</mn><mo>)</mo><mo>]</mo></math>"#;
    test_braille("Marburg", expr, "⠣⠼⠁⠙⠀⠤⠼⠑⠜⠀⠖⠼⠛⠀⠕⠂⠤⠷⠼⠁⠀⠖⠼⠃⠣⠼⠑⠀⠖⠼⠉⠜⠾");
}

#[test]
fn brackets_2() {
    let expr = r#"<math><mn>2</mn><mo>+</mo><mo>{</mo><mn>4</mn><mo>-</mo><mo>[</mo><mn>5</mn><mo>+</mo><mo>(</mo><mn>6</mn><mo>-</mo><mn>2</mn><mo>)</mo><mo>]</mo><mo>+</mo><mn>3</mn><mo>(</mo><mn>6</mn><mo>+</mo><mn>4</mn><mo>)</mo><mo>}</mo><mo>=</mo><mn>2</mn><mo>+</mo><mo>{</mo><mn>4</mn><mo>-</mo><mo>[</mo><mn>5</mn><mo>+</mo><mn>4</mn><mo>]</mo><mo>+</mo><mn>30</mn><mo>}</mo></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠀⠖⠪⠼⠙⠀⠤⠷⠼⠑⠀⠖⠣⠼⠋⠀⠤⠼⠃⠜⠾⠀⠖⠼⠉⠣⠼⠋⠀⠖⠼⠙⠜⠕⠀⠶⠼⠃⠀⠖⠪⠼⠙⠀⠤⠷⠼⠑⠀⠖⠼⠙⠾⠀⠖⠼⠉⠚⠕");
}

#[test]
fn brackets_3() {
    let expr = r#"<math><mn>5</mn><mo>-</mo><mn>3</mn><mo>=</mo><mn>2</mn><mo>&#xA0;</mo><mo>(</mo><mi>bo</mi><mo>&#xA0;</mo><mn>2</mn><mo>+</mo><mn>3</mn><mo>=</mo><mn>5</mn><mo>)</mo></math>"#;
    test_braille("Marburg", expr, "⠼⠑⠀⠤⠼⠉⠀⠶⠼⠃⠀⠠⠶⠃⠕⠀⠼⠃⠀⠖⠼⠉⠀⠶⠼⠑⠠⠶");
}

#[test]
fn absolute_value_1() {
    let expr = r#"<math><mfenced open="|" close="|"><mrow><mo>-</mo><mn>5</mn></mrow></mfenced><mo>=</mo><mn>5</mn></math>"#;
    test_braille("Marburg", expr, "⠈⠇⠤⠼⠑⠸⠀⠶⠼⠑");
}

#[test]
fn absolute_value_2() {
    let expr = r#"<math><mo>-</mo><mfenced open="|" close="|"><mrow><mo>-</mo><mo>(</mo><mn>7</mn><mo>+</mo><mn>4</mn><mo>)</mo></mrow></mfenced><mo>&lt;</mo><mfenced open="|" close="|"><mrow><mo>(</mo><mn>4</mn><mo>-</mo><mn>7</mn><mo>)</mo></mrow></mfenced></math>"#;
    test_braille("Marburg", expr, "⠤⠈⠇⠤⠣⠼⠛⠀⠖⠼⠙⠜⠸⠀⠪⠄⠈⠇⠣⠼⠙⠀⠤⠼⠛⠜⠸");
}

#[test]
fn number_divisors_1() {
    let expr = r#"<math><mn>5</mn><mo>|</mo><mn>25</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠑⠈⠇⠼⠃⠑");
}

#[test]
fn number_divisors_2() {
    let expr = r#"<math><mn>5</mn><mo>&#x2224;</mo><mn>27</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠑⠀⠔⠈⠇⠼⠃⠛");
}

#[test]
fn number_separation_1() {
    let expr = r#"<math><mn>12</mn><mo>;</mo></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠃⠠⠰");
}

#[test]
fn number_separation_2() {
    let expr = r#"<math><mn>12</mn><mo>?</mo></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠃⠠⠢");
}

#[test]
fn number_separation_3() {
    let expr = r#"<math><mn>12</mn><mo>!</mo></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠃⠠⠖");
}

#[test]
fn number_separation_4() {
    let expr = r#"<math><mi>P</mi><mo>=</mo><mo>(</mo><mn>3</mn><mo>,</mo><mn>5</mn><mo>)</mo></math>"#;
    test_braille("Marburg", expr, "⠨⠏⠀⠶⠣⠼⠉⠠⠂⠀⠼⠑⠜");
}

#[test]
fn number_separation_5() {
    let expr = r#"<math><mi>&#x3A7;</mi><mo>=</mo><mo>{</mo><mn>1</mn><mo>,</mo><mo>&#xA0;</mo><mn>2</mn><mo>,</mo><mo>&#xA0;</mo><mn>3</mn><mo>,</mo><mo>&#xA0;</mo><mn>4</mn><mo>}</mo></math>"#;
    test_braille("Marburg", expr, "⠭⠀⠶⠪⠼⠁⠠⠂⠀⠼⠃⠠⠂⠀⠼⠉⠠⠂⠀⠼⠙⠕");
}

#[test]
fn number_separation_6() {
    let expr = r#"<math><mo>(</mo><mn>3</mn><mo>,</mo><mn>2</mn><mo>&#xA0;</mo><mo>;</mo><mo>&#xA0;</mo><mn>5</mn><mo>,</mo><mn>4</mn><mo>&gt;</mo></math>"#;
    test_braille("Marburg", expr, "⠣⠼⠉⠂⠃⠠⠆⠀⠼⠑⠂⠙⠠⠾");
}

#[test]
fn number_separation_7() {
    let expr = r#"<math><mi>A</mi><mo>=</mo><mo>&lt;</mo><mn>2</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>)</mo></math>"#;
    test_braille("Marburg", expr, "⠨⠁⠀⠶⠷⠄⠼⠃⠠⠂⠀⠼⠑⠜");
}

#[test]
fn number_separation_8() {
    let expr = r#"<math><mi>B</mi><mo>=</mo><mo>{</mo><mn>2</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>,</mo><mo>&#xA0;</mo><mn>6</mn><mo>}</mo><mo>,</mo></math>"#;
    test_braille("Marburg", expr, "⠨⠃⠀⠶⠪⠼⠃⠠⠂⠀⠼⠑⠠⠂⠀⠼⠋⠕⠠⠂");
}

// I'have skipped "Line divisions" chapter. I have no idea how to enter the multiline expression into the format.

#[test]
fn algebraic_expressions_1() {
    let expr = r#"<math><mn>3</mn><mi>a</mi><mo>+</mo><mi>b</mi><mo>+</mo><mi>c</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠉⠠⠁⠀⠖⠃⠀⠖⠉");
}

#[test]
fn algebraic_expressions_2() {
    let expr = r#"<math><mn>3</mn><mi>x</mi><mo>+</mo><mi>y</mi><mo>+</mo><mi>z</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠉⠠⠭⠀⠖⠽⠀⠖⠵");
}

#[test]
fn algebraic_expressions_3() {
    let expr = r#"<math><mn>3</mn><mi>a</mi><mo>+</mo><mn>2</mn><mi>b</mi><mo>+</mo><mn>4</mn><mi>c</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠉⠠⠁⠀⠖⠼⠃⠠⠃⠀⠖⠼⠙⠠⠉");
}

#[test]
fn algebraic_expressions_4() {
    let expr = r#"<math><mn>3</mn><mi>x</mi><mo>+</mo><mn>2</mn><mi>y</mi><mo>+</mo><mn>4</mn><mi>z</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠉⠠⠭⠀⠖⠼⠃⠽⠀⠖⠼⠙⠵");
}

#[test]
fn algebraic_expressions_5() {
    let expr = r#"<math><mn>2</mn><mi>a</mi><mi>C</mi><mo>+</mo><mn>4</mn><mi>B</mi><mi>c</mi><mo>-</mo><mn>3</mn><mo>,</mo><mn>2</mn><mi>B</mi><mi>D</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠠⠁⠨⠉⠀⠖⠼⠙⠨⠃⠠⠉⠀⠤⠼⠉⠂⠃⠨⠃⠙");
}

#[test]
fn algebraic_expressions_6() {
    let expr = r#"<math><mn>2</mn><mi>a</mi><mi>C</mi><mo>+</mo><mn>4</mn><mi>B</mi><mi>c</mi><mo>-</mo><mn>3</mn><mo>,</mo><mn>2</mn><mi>B</mi><mi>D</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠠⠁⠃⠉⠀⠤⠼⠉⠨⠃⠉⠙⠀⠤⠃⠙");
}

#[test]
fn algebraic_expressions_7() {
    let expr = r#"<math><mn>2</mn><mi>a</mi><mi>b</mi><mi>c</mi><mo>-</mo><mn>3</mn><mi>B</mi><mi>C</mi><mi>d</mi><mo>-</mo><mi>b</mi><mi>d</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠠⠁⠃⠉⠀⠤⠼⠉⠨⠃⠉⠠⠙⠀⠤⠃⠙");
}

#[test]
fn algebraic_expressions_8() {
    let expr = r#"<math><mn>2</mn><mi>a</mi><mi>b</mi><mi>c</mi><mo>-</mo><mn>3</mn><mi>&#x3B1;</mi><mi>&#x3B2;</mi><mi>&#x3B3;</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠠⠁⠃⠉⠀⠤⠼⠉⠰⠁⠃⠛");
}

// I have skipped alternative notations of algebraic_expressions with big letters

#[test]
fn sets_1() {
    let expr = r#"<math><mi>N</mi><mo>&#x2282;</mo><mi>C</mi><mo>&#x2282;</mo><mi>W</mi><mo>&#x2282;</mo><mi>R</mi></math>"#;
    test_braille("Marburg", expr, "⠨⠨⠝⠀⠣⠄⠨⠨⠉⠀⠣⠄⠨⠨⠺⠀⠣⠄⠨⠨⠗");
}

#[test]
fn sets_2() {
    let expr = r#"<math><mi>W</mi><mo>&#x2284;</mo><mi>I</mi><mi>W</mi></math>"#;
    test_braille("Marburg", expr, "⠨⠨⠺⠀⠔⠣⠄⠨⠨⠊⠺");
}

#[test]
fn sets_3() {
    let expr = r#"<math><mi>R</mi><mo>&#x2283;</mo><mi>N</mi></math>"#;
    test_braille("Marburg", expr, "⠨⠨⠗⠜⠂⠨⠨⠝");
}

#[test]
fn sets_4() {
    let expr = r#"<math><mn>2</mn><mo>&#x2208;</mo><msup><mi>C</mi><mo>+</mo></msup></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠈⠑⠨⠨⠉⠖");
}

#[test]
fn sets_5() {
    let expr = r#"<math><mi>C</mi><mo>=</mo><mi>A</mi><mo>&#x222A;</mo><mi>C</mi></math>"#;
    test_braille("Marburg", expr, "⠨⠉⠀⠶⠁⠀⠩⠄⠉");
}

#[test]
fn sets_6() {
    let expr = r#"<math><mi>B</mi><mo>=</mo><mi>D</mi><mo>&#x2216;</mo><mi>C</mi></math>"#;
    test_braille("Marburg", expr, "⠨⠃⠀⠶⠙⠀⠡⠄⠉");
}

#[test]
fn sets_7() {
    let expr = r#"<math><mi>B</mi><mo>=</mo><mo>{</mo><mi>x</mi><mo>:</mo><mo>&#xA0;</mo><mi>x</mi><mo>&#x2208;</mo><mi>N</mi><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mi>x</mi><mo>&lt;</mo><mn>7</mn><mo>}</mo></math>"#;
    test_braille("Marburg", expr, "⠨⠃⠀⠶⠪⠠⠭⠒⠀⠭⠀⠈⠑⠨⠨⠝⠀⠊⠀⠠⠭⠀⠪⠄⠼⠛⠕");
}

#[test]
fn sets_8() {
    let expr = r#"<math><mi>B</mi><mo>=</mo><mo>{</mo><mi>x</mi><mo>:</mo><mo>&#xA0;</mo><mi>x</mi><mo>&#x2208;</mo><mi>N</mi><mo>&#xA0;</mo><mo>&#x2227;</mo><mo>&#xA0;</mo><mi>x</mi><mo>&lt;</mo><mn>7</mn><mo>}</mo></math>"#;
    test_braille("Marburg", expr, "⠨⠃⠀⠶⠪⠠⠭⠒⠀⠭⠀⠈⠑⠨⠨⠝⠀⠬⠂⠠⠭⠀⠪⠄⠼⠛⠕");
}

#[test]
fn sets_9() {
    let expr = r#"<math><mi>W</mi><mo>=</mo><mo>{</mo><mi>x</mi><mo>:</mo><mo>&#xA0;</mo><mi>x</mi><mo>=</mo><mfrac><mi>p</mi><mi>q</mi></mfrac><mo>&#x2227;</mo><mi>p</mi><mo>&#x2208;</mo><mi>C</mi><mo>&#x2227;</mo><mi>q</mi><mo>&#x2208;</mo><mi>C</mi><mo>&#x2216;</mo><mo>{</mo><mn>0</mn><mo>}</mo><mo>}</mo></math>"#;
    test_braille("Marburg", expr, "⠨⠨⠺⠀⠶⠪⠠⠭⠒⠀⠭⠀⠶⠏⠳⠟⠀⠬⠂⠏⠀⠈⠑⠨⠨⠉⠀⠬⠂⠠⠟⠀⠈⠑⠨⠨⠉⠀⠡⠄⠪⠼⠚⠕⠕");
}

#[test]
fn sets_10() {
    let expr = r#"<math><mo>(</mo><mo>-</mo><mo>&#x221E;</mo><mo>;</mo><mo>&#xA0;</mo><mi>a</mi><mo>)</mo><mo>=</mo><mo>{</mo><mi>x</mi><mo>:</mo><mo>&#xA0;</mo><mi>x</mi><mo>&#x2208;</mo><mi>R</mi><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mi>x</mi><mo>&lt;</mo><mi>a</mi><mo>}</mo></math>"#;
    test_braille("Marburg", expr, "⠣⠤⠼⠿⠆⠀⠠⠁⠜⠀⠶⠪⠭⠒⠀⠭⠀⠈⠑⠨⠨⠗⠀⠊⠀⠠⠭⠀⠪⠄⠁⠕");
}

#[test]
fn sets_11() {
    let expr = r#"<math><mi>x</mi><mo>&#x2208;</mo><mi>A</mi><mo>&#x2216;</mo><mi>B</mi><mo>&#xA0;</mo><mi>w</mi><mi>t</mi><mi>e</mi><mi>d</mi><mi>y</mi><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mi>t</mi><mi>y</mi><mi>l</mi><mi>k</mi><mi>o</mi><mo>&#xA0;</mo><mi>w</mi><mi>t</mi><mi>e</mi><mi>d</mi><mi>y</mi><mo>&#xA0;</mo><mi>g</mi><mi>d</mi><mi>y</mi><mo>(</mo><mi>x</mi><mo>&#x2208;</mo><mi>A</mi><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mi>x</mi><mo>&#x2209;</mo><mi>B</mi><mo>)</mo></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠀⠈⠑⠨⠁⠀⠡⠄⠨⠃⠀⠺⠞⠑⠙⠽⠀⠊⠀⠞⠽⠇⠅⠕⠀⠺⠞⠑⠙⠽⠀⠛⠙⠽⠀⠣⠠⠭⠀⠈⠑⠨⠁⠀⠊⠀⠠⠭⠀⠔⠈⠑⠨⠃⠜");
}

#[test]
fn sets_12() {
    let expr = r#"<math><mi>x</mi><mo>&#x2208;</mo><mi>A</mi><mo>&#x2216;</mo><mi>B</mi><mo>&#xA0;</mo><mo>&#x21D4;</mo><mo>(</mo><mi>x</mi><mo>&#x2208;</mo><mi>A</mi><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mi>x</mi><mo>&#x2209;</mo><mi>B</mi><mo>)</mo></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠀⠈⠑⠨⠁⠀⠡⠄⠨⠃⠀⠐⠶⠂⠣⠠⠭⠀⠈⠑⠨⠁⠀⠬⠂⠠⠭⠀⠔⠈⠑⠨⠃⠜");
}

#[test]
fn sets_13() {
    let expr = r#"<math><mi>A</mi><mo>=</mo><mo>{</mo><mi>x</mi><mo>:</mo><mo>&#xA0;</mo><mi>x</mi><mo>&#x2208;</mo><mi>C</mi><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mn>0</mn><mo>&lt;</mo><mi>x</mi><mo>&lt;</mo><mn>5</mn><mo>}</mo></math>"#;
    test_braille("Marburg", expr, "⠨⠁⠀⠶⠪⠠⠭⠒⠀⠭⠀⠈⠑⠨⠨⠉⠀⠊⠀⠼⠚⠀⠪⠄⠭⠀⠪⠄⠼⠑⠕");
}

#[test]
fn sets_14() {
    let expr = r#"<math><mo>(</mo><mi>a</mi><mo>,</mo><mo>&#xA0;</mo><mi>b</mi><mo>)</mo><mo>&#x2208;</mo><mo>&#xA0;</mo><mi>A</mi><mo>&#xD7;</mo><mi>B</mi><mo>&#xA0;</mo><mo>&#x21D4;</mo><mo>&#xA0;</mo><mo>(</mo><mi>a</mi><mo>&#x2208;</mo><mo>&#xA0;</mo><mi>A</mi><mo>&#x2227;</mo><mo>&#xA0;</mo><mi>b</mi><mo>&#x2208;</mo><mi>B</mi><mo>)</mo></math>"#;
    test_braille("Marburg", expr, "⠣⠠⠁⠂⠀⠃⠜⠀⠈⠑⠨⠁⠀⠦⠨⠃⠀⠐⠶⠂⠣⠠⠁⠀⠈⠑⠨⠁⠀⠬⠂⠠⠃⠀⠈⠑⠨⠃⠜");
}

#[test]
fn sets_15() {
    let expr = r#"<math><mo>(</mo><mi>a</mi><mo>,</mo><mo>&#xA0;</mo><mi>b</mi><mo>)</mo><mo>&#x2208;</mo><mo>&#xA0;</mo><mi>A</mi><mo>&#xD7;</mo><mi>B</mi><mo>&#xA0;</mo><mo>&#x21D4;</mo><mo>&#xA0;</mo><mo>(</mo><mi>a</mi><mo>&#x2208;</mo><mo>&#xA0;</mo><mi>A</mi><mo>&#x2227;</mo><mo>&#xA0;</mo><mi>b</mi><mo>&#x2208;</mo><mi>B</mi><mo>)</mo></math>"#;
    test_braille("Marburg", expr, "⠨⠁⠀⠣⠄⠨⠭");
}

#[test]
fn sets_16() {
    let expr = r#"<math><mi>A</mi><mo>'</mo><mo>=</mo><mi>X</mi><mo>&#x2216;</mo><mo>&#xA0;</mo><mi>A</mi></math>"#;
    test_braille("Marburg", expr, "⠨⠁⠔⠀⠶⠨⠭⠀⠡⠄⠨⠁");
}

#[test]
fn sets_17() {
    let expr = r#"<math><mi>J</mi><mi>e</mi><mi>&#x17C;</mi><mi>e</mi><mi>l</mi><mi>i</mi><mo>&#xA0;</mo><mi>X</mi><mo>=</mo><mo>{</mo><mn>1</mn><mo>,</mo><mo>&#xA0;</mo><mn>2</mn><mo>,</mo><mo>&#xA0;</mo><mn>3</mn><mo>,</mo><mo>&#xA0;</mo><mn>4</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>,</mo><mo>&#xA0;</mo><mn>6</mn><mo>}</mo><mo>,</mo></math>"#;
    test_braille("Marburg", expr, "⠨⠚⠑⠯⠑⠇⠊⠀⠨⠭⠀⠶⠪⠼⠁⠠⠂⠀⠼⠃⠠⠂⠀⠼⠉⠠⠂⠀⠼⠙⠠⠂⠀⠼⠑⠠⠂⠀⠼⠋⠕⠠⠂⠀⠔⠼⠃⠑");
}

#[test]
fn sets_18() {
    let expr = r#"<math><mi>A</mi><mo>&#x2282;</mo><mi>X</mi><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mi>B</mi><mo>&#x2282;</mo><mi>X</mi><mo>,</mo></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠀⠣⠄⠨⠭⠀⠊⠀⠨⠃⠀⠣⠄⠨⠭⠂");
}

#[test]
fn sets_19() {
    let expr = r#"<math><mi>A</mi><mo>=</mo><mo>{</mo><mn>1</mn><mo>,</mo><mo>&#xA0;</mo><mn>2</mn><mo>,</mo><mo>&#xA0;</mo><mn>3</mn><mo>,</mo><mo>&#xA0;</mo><mn>4</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>}</mo><mo>&#xA0;</mo><mi>i</mi><mo>&#xA0;</mo><mi>B</mi><mo>=</mo><mo>{</mo><mn>4</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>}</mo><mo>,</mo><mo>&#xA0;</mo><mi>t</mi><mi>o</mi><mo>:</mo></math>"#;
    test_braille("Marburg", expr, "⠨⠁⠀⠶⠪⠼⠁⠠⠂⠀⠼⠃⠠⠂⠀⠼⠉⠠⠂⠀⠼⠙⠠⠂⠀⠼⠑⠕⠀⠊⠀⠨⠃⠀⠶⠪⠼⠙⠠⠂⠀⠼⠑⠕⠠⠂⠀⠞⠕⠒");
}

#[test]
fn sets_20() {
    let expr = r#"<math><mi>A</mi><mo>&#x222A;</mo><mi>B</mi><mo>=</mo><mo>{</mo><mn>1</mn><mo>,</mo><mo>&#xA0;</mo><mn>2</mn><mo>,</mo><mo>&#xA0;</mo><mn>3</mn><mo>,</mo><mo>&#xA0;</mo><mn>4</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>}</mo><mo>,</mo></math>"#;
    test_braille("Marburg", expr, "⠨⠁⠀⠩⠄⠨⠃⠀⠶⠪⠼⠁⠠⠂⠀⠼⠃⠠⠂⠀⠼⠉⠠⠂⠀⠼⠙⠠⠂⠀⠼⠑⠕⠠⠂");
}

#[test]
fn sets_21() {
    let expr = r#"<math><mo>(</mo><mi>A</mi><mo>&#x222A;</mo><mi>B</mi><mo>)</mo><mo>'</mo><mo>=</mo><mo>{</mo><mn>6</mn><mo>}</mo><mo>,</mo></math>"#;
    test_braille("Marburg", expr, "⠣⠨⠁⠀⠩⠄⠨⠃⠜⠔⠀⠶⠪⠼⠋⠕⠠⠂");
}

#[test]
fn sets_22() {
    let expr = r#"<math><mo>(</mo><mi>A</mi><mo>&#x2216;</mo><mi>B</mi><mo>)</mo><mo>'</mo><mo>=</mo><mo>{</mo><mn>4</mn><mo>,</mo><mo>&#xA0;</mo><mn>5</mn><mo>,</mo><mo>&#xA0;</mo><mn>6</mn><mo>}</mo><mo>,</mo></math>"#;
    test_braille("Marburg", expr, "⠣⠨⠁⠀⠡⠄⠨⠃⠜⠔⠀⠶⠪⠼⠙⠠⠂⠀⠼⠑⠠⠂⠀⠼⠋⠕⠠⠂");
}

#[test]
fn sets_23() {
    let expr = r#"<math><mi>B</mi><mo>&#x2216;</mo><mi>A</mi><mo>=</mo><mi>&#xD8;</mi></math>"#;
    test_braille("Marburg", expr, "⠨⠃⠀⠡⠄⠨⠁⠀⠶⠯⠕⠠⠂");
}

#[test]
fn sets_24() {
    let expr = r#"<math><mi>A</mi><mo>&#x2229;</mo><mi>B</mi><mo>=</mo><mo>{</mo><mn>4</mn><mo>,</mo><mn>5</mn><mo>}</mo><mo>.</mo></math>"#;
    test_braille("Marburg", expr, "⠨⠁⠀⠬⠄⠨⠃⠀⠶⠪⠼⠙⠠⠂⠀⠼⠑⠕⠄");
}

// I've skipped chapter on Graphical representation of intervals, as they require multiline representation.

#[test]
fn simple_projectors_1() {
    let expr = r#"<math><msup><mi>a</mi><mrow><mi>n</mi><mo>+</mo><mi>m</mi></mrow></msup><mo>=</mo><msup><mi>a</mi><mi>n</mi></msup><mo>&#xB7;</mo><msup><mi>a</mi><mi>m</mi></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠬⠝⠈⠖⠍⠀⠶⠁⠬⠝⠱⠄⠁⠬⠍");
}

#[test]
fn simple_projectors_2() {
    let expr = r#"<math><msub><mi>a</mi><mi>n</mi></msub><mo>=</mo><mo>&#xA0;</mo><mstyle displaystyle="false"><mfrac><mrow><msub><mi>a</mi><mrow><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msub><mo>+</mo><msub><mi>a</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub></mrow><mn>2</mn></mfrac></mstyle></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠡⠝⠀⠶⠆⠁⠡⠝⠈⠤⠼⠁⠱⠈⠖⠁⠡⠝⠈⠖⠼⠁⠳⠆");
}

#[test]
fn simple_projectors_3() {
    let expr = r#"<math><msup><mrow><mo>(</mo><msub><mi>a</mi><mi>k</mi></msub><mo>)</mo></mrow><mn>2</mn></msup><mo>=</mo><mo>&#xA0;</mo><msub><mi>a</mi><mi>k</mi></msub><mo>&#xB7;</mo><msub><mi>a</mi><mi>k</mi></msub></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠡⠅⠬⠆⠀⠶⠁⠡⠅⠀⠄⠁⠡⠅");
}

#[test]
fn simple_projectors_4() {
    let expr = r#"<math><mfrac><msqrt><mn>3</mn></msqrt><mn>2</mn></mfrac><mo>=</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>&#xB7;</mo><msqrt><mn>3</mn></msqrt></math>"#;
    test_braille("Marburg", expr, "⠩⠼⠉⠳⠆⠀⠶⠼⠁⠆⠄⠩⠼⠉");
}

#[test]
fn simple_projectors_5() {
    let expr = r#"<math><mo>(</mo><mn>3</mn><mo>+</mo><msub><mi>a</mi><mrow><mi>n</mi><mo>+</mo><mn>2</mn></mrow></msub><mo>)</mo><mo>&#xB7;</mo><mn>5</mn></math>"#;
    test_braille("Marburg", expr, "⠣⠼⠉⠀⠖⠠⠁⠡⠝⠈⠖⠼⠃⠜⠄⠼⠑");
}

#[test]
fn simple_projectors_6() {
    let expr = r#"<math><msub><mi>f</mi><mi>n</mi></msub><mo>(</mo><mi>x</mi><mo>)</mo><mo>=</mo><mi>n</mi><mi>x</mi></math>"#;
    test_braille("Marburg", expr, "⠠⠋⠡⠝⠱⠣⠭⠜⠀⠶⠝⠭");
}

#[test]
fn compound_projectors_1() {
    let expr = r#"<math><msqrt><mfrac><mi>x</mi><mi>y</mi></mfrac></msqrt><mo>=</mo><mfrac><msqrt><mi>x</mi></msqrt><msqrt><mi>y</mi></msqrt></mfrac></math>"#;
    test_braille("Marburg", expr, "⠐⠩⠠⠭⠳⠽⠀⠶⠩⠭⠳⠩⠽");
}

#[test]
fn compound_projectors_2() {
    let expr = r#"<math><msub><mi>u</mi><mi>n</mi></msub><mo>=</mo><mroot><mrow><msup><mn>3</mn><mi>n</mi></msup><mo>+</mo><msup><mn>2</mn><mi>n</mi></msup></mrow><mi>n</mi></mroot></math>"#;
    test_braille("Marburg", expr, "⠠⠥⠡⠝⠀⠶⠌⠝⠐⠩⠼⠉⠬⠝⠱⠈⠖⠼⠃⠬⠝");
}

#[test]
fn compound_projectors_3() {
    let expr = r#"<math><msup><mi>e</mi><mfrac><mi>x</mi><mn>2</mn></mfrac></msup><mo>=</mo><msqrt><msup><mi>e</mi><mi>x</mi></msup></msqrt></math>"#;
    test_braille("Marburg", expr, "⠠⠑⠐⠬⠭⠳⠆⠀⠶⠐⠩⠑⠬⠭");
}

#[test]
fn detailed_projectors_1() {
    let expr = r#"<math><msup><mi>e</mi><mrow><mroot><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mn>3</mn></mroot><mo>+</mo><mn>2</mn></mrow></msup><mo>&#xB7;</mo><msup><mi>e</mi><mi>x</mi></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠑⠨⠬⠌⠒⠩⠭⠈⠖⠼⠁⠀⠖⠼⠃⠨⠱⠄⠑⠬⠭");
}

#[test]
fn detailed_projectors_2() {
    let expr = r#"<math><mi>u</mi><mo>=</mo><mroot><mrow><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><msqrt><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mn>1</mn></msqrt><mo>+</mo><mn>8</mn></mrow><mn>3</mn></mroot></math>"#;
    test_braille("Marburg", expr, "⠠⠥⠀⠶⠌⠒⠨⠩⠭⠬⠆⠀⠖⠐⠩⠭⠬⠆⠈⠖⠼⠁⠀⠖⠼⠓⠨⠱");
}

#[test]
fn detailed_projectors_3() {
    let expr = r#"<math><mn>3</mn><mi>x</mi><mo>-</mo><mn>7</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠉⠠⠭⠀⠤⠼⠛");
}

#[test]
fn detailed_projectors_4() {
    let expr = r#"<math><mfrac><mn>2</mn><mi>x</mi></mfrac><mi>x</mi><mo>+</mo><mn>1</mn><mfrac><mn>2</mn><mn>3</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠒⠠⠭⠀⠖⠼⠁⠼⠃⠒");
}

#[test]
fn detailed_projectors_5() {
    let expr = r#"<math><mn>2</mn><mi>&#x3C0;</mi><mi>r</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠰⠏⠠⠗");
}

#[test]
fn detailed_projectors_6() {
    let expr = r#"<math><msup><mrow><mo>(</mo><mi>x</mi><mo>+</mo><mi>y</mi><mo>)</mo></mrow><mn>2</mn></msup></math>"#;
    test_braille("Marburg", expr, "⠣⠠⠭⠀⠖⠽⠜⠬⠆");
}

#[test]
fn detailed_projectors_7() {
    let expr = r#"<math><msup><mi>e</mi><mfrac><mi>x</mi><mn>2</mn></mfrac></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠑⠐⠬⠭⠳⠆");
}

#[test]
fn detailed_projectors_8() {
    let expr = r#"<math><msub><mi>a</mi><msub><mi>i</mi><mi>j</mi></msub></msub></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠐⠡⠊⠡⠚");
}

#[test]
fn detailed_projectors_9() {
    let expr = r#"<math><mfrac><mrow><mn>1</mn><mo>+</mo><mstyle displaystyle="true"><mfrac><mi>b</mi><mi>a</mi></mfrac></mstyle></mrow><mrow><mn>1</mn><mo>-</mo><mstyle displaystyle="true"><mfrac><mi>b</mi><mi>a</mi></mfrac></mstyle></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠼⠁⠀⠖⠆⠠⠃⠳⠁⠀⠳⠀⠼⠁⠀⠤⠆⠃⠳⠁⠰");
}

#[test]
fn detailed_projectors_10() {
    let expr = r#"<math><msqrt><mfrac><mi>x</mi><mi>y</mi></mfrac></msqrt></math>"#;
    test_braille("Marburg", expr, "⠐⠩⠠⠭⠳⠽");
}

//Fractions

#[test]
fn fractions_1() {
    let expr = r#"<math><mfrac><mrow><mn>2</mn><mi>x</mi><mo>+</mo><mn>3</mn><mi>y</mi><mo>-</mo><mn>4</mn></mrow><mrow><mn>4</mn><mi>x</mi><mo>-</mo><mn>5</mn><mi>y</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠼⠃⠠⠭⠀⠖⠼⠉⠽⠀⠤⠼⠙⠀⠳⠀⠼⠙⠭⠀⠤⠼⠑⠽⠰");
}

#[test]
fn fractions_2a() {
    let expr = r#"<math><mfrac><mn>2</mn><mn>3</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠳⠼⠉");
}

#[test]
fn fractions_2b() {
    let expr = r#"<math><mfrac><mn>2</mn><mn>3</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠼⠃⠀⠳⠀⠼⠉⠰");
}

#[test]
fn fractions_3a() {
    let expr = r#"<math><mfrac><mi>x</mi><mi>y</mi></mfrac></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠳⠽");
}

#[test]
fn fractions_3b() {
    let expr = r#"<math><mfrac><mi>x</mi><mi>y</mi></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠠⠭⠀⠳⠀⠽⠰");
}

#[test]
fn fractions_4() {
    let expr = r#"<math><mfrac><mrow><mn>3</mn><mi>x</mi></mrow><mn>4</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠉⠠⠭⠳⠲");
}

#[test]
fn fractions_5() {
    let expr = r#"<math><mfrac><mrow><mn>2</mn><mi>a</mi></mrow><mn>7</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠠⠁⠳⠶");
}

#[test]
fn fractions_6() {
    let expr = r#"<math><mfrac><mrow><mi>a</mi><mi>b</mi><mo>+</mo><mi>c</mi><mi>d</mi></mrow><mn>4</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠠⠁⠃⠀⠖⠉⠙⠀⠳⠲");
}

#[test]
fn fractions_7() {
    let expr = r#"<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠆");
}

#[test]
fn fractions_8() {
    let expr = r#"<math><mfrac><mn>3</mn><mn>14</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠉⠂⠲");
}

#[test]
fn fractions_9() {
    let expr = r#"<math><mfrac><mn>17</mn><mn>5</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠛⠢");
}

#[test]
fn fractions_10() {
    let expr = r#"<math><mfrac><mn>138</mn><mn>43</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠉⠓⠲⠒");
}

#[test]
fn fractions_11() {
    let expr = r#"<math><mn>2</mn><mfrac><mn>3</mn><mn>4</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠼⠉⠲");
}

#[test]
fn fractions_12() {
    let expr = r#"<math><mn>4</mn><mfrac><mn>7</mn><mn>15</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠙⠼⠛⠂⠢");
}

#[test]
fn fractions_13() {
    let expr = r#"<math><mn>12</mn><mfrac><mn>14</mn><mn>17</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠃⠼⠁⠙⠂⠶");
}

#[test]
fn fractions_14a() {
    let expr = r#"<math><mfrac><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mrow><mi>x</mi><mo>-</mo><mi>y</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠈⠖⠽⠳⠭⠈⠤⠽");
}

#[test]
fn fractions_14b() {
    let expr = r#"<math><mfrac><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mrow><mi>x</mi><mo>-</mo><mi>y</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠠⠭⠀⠖⠽⠀⠳⠀⠭⠀⠤⠽⠰");
}

#[test]
fn fractions_15() {
    let expr = r#"<math><mfrac><mrow><mo>-</mo><mi>p</mi><mo>-</mo><mi>q</mi></mrow><mi>n</mi></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠤⠠⠏⠈⠤⠟⠳⠝");
}

#[test]
fn fractions_16() {
    let expr = r#"<math><mfrac><mrow><mi>p</mi><mo>+</mo><mi>q</mi></mrow><mrow><mo>-</mo><mi>n</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠠⠏⠈⠖⠟⠳⠈⠤⠝");
}

#[test]
fn fractions_17() {
    let expr = r#"<math><mfrac><mrow><mo>(</mo><mi>n</mi><mo>+</mo><mn>1</mn><mo>)</mo><mo>!</mo></mrow><mrow><mn>2</mn><mi>n</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠣⠠⠝⠀⠖⠼⠁⠜⠫⠈⠳⠼⠃⠝⠀⠔⠼⠃⠋");
}

#[test]
fn fractions_18() {
    let expr = r#"<math><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>+</mo><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>=</mo><mfrac><mn>6</mn><mn>4</mn></mfrac><mo>=</mo><mfrac><mn>3</mn><mn>2</mn></mfrac><mo>=</mo><mn>1</mn><mfrac><mn>1</mn><mn>2</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠉⠲⠀⠖⠼⠉⠲⠀⠶⠼⠋⠲⠀⠶⠼⠉⠆⠀⠶⠼⠁⠼⠁⠆");
}

#[test]
fn fractions_19() {
    let expr = r#"<math><mn>2</mn><mo>,</mo><mn>6</mn><mo>-</mo><mfrac><mn>6</mn><mn>15</mn></mfrac><mo>=</mo><mn>2</mn><mfrac><mn>18</mn><mn>30</mn></mfrac><mo>-</mo><mfrac><mn>12</mn><mn>30</mn></mfrac><mo>=</mo><mn>2</mn><mfrac><mn>6</mn><mn>30</mn></mfrac><mo>=</mo><mn>2</mn><mfrac><mn>1</mn><mn>5</mn></mfrac><mo>=</mo><mn>2</mn><mo>,</mo><mn>2</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠂⠋⠀⠤⠼⠋⠂⠢⠀⠶⠼⠃⠼⠁⠓⠒⠴⠀⠤⠼⠁⠃⠒⠴⠀⠶⠼⠃⠼⠋⠒⠴⠀⠶⠼⠃⠼⠁⠢⠀⠶⠼⠃⠂⠃");
}

#[test]
fn fractions_20a() {
    let expr = r#"<math><mn>4</mn><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>&#xB7;</mo><mo>(</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>-</mo><mn>0</mn><mo>,</mo><mn>8</mn><mo>)</mo><mo>=</mo><mn>4</mn><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>&#xB7;</mo><mn>0</mn><mo>,</mo><mn>4</mn><mo>=</mo><mfrac><mn>19</mn><mn>4</mn></mfrac><mo>&#xB7;</mo><mfrac><mn>4</mn><mn>10</mn></mfrac><mo>=</mo><mfrac><mn>19</mn><mn>10</mn></mfrac><mo>=</mo><mn>1</mn><mo>,</mo><mn>9</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠙⠼⠉⠲⠄⠣⠼⠁⠂⠃⠀⠤⠼⠚⠂⠓⠜⠀⠶⠼⠙⠼⠉⠲⠄⠼⠚⠂⠙⠀⠶⠼⠁⠊⠲⠄⠼⠙⠂⠴⠀⠶⠼⠁⠊⠂⠴⠀⠶⠼⠁⠂⠊");
}

#[test]
fn fractions_20b() {
    let expr = r#"<math><mn>4</mn><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>&#xB7;</mo><mo>(</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>-</mo><mn>0</mn><mo>,</mo><mn>8</mn><mo>)</mo><mo>=</mo><mn>4</mn><mfrac><mn>3</mn><mn>4</mn></mfrac><mo>&#xB7;</mo><mn>0</mn><mo>,</mo><mn>4</mn><mo>=</mo><mfrac><mn>19</mn><mn>4</mn></mfrac><mo>&#xB7;</mo><mfrac><mn>4</mn><mn>10</mn></mfrac><mo>=</mo><mfrac><mn>19</mn><mn>10</mn></mfrac><mo>=</mo><mn>1</mn><mo>,</mo><mn>9</mn></math>"#;
    test_braille("Marburg", expr, "⠼⠙⠼⠉⠲⠀⠄⠣⠼⠁⠂⠃⠀⠤⠼⠚⠂⠓⠜⠀⠶⠼⠙⠼⠉⠲⠀⠄⠼⠚⠂⠙⠀⠶⠼⠁⠊⠲⠀⠄⠼⠙⠂⠴⠀⠶⠼⠁⠊⠂⠴⠀⠶⠼⠁⠂⠊");
}

#[test]
fn fractions_21() {
    let expr = r#"<math><mfrac><mrow><mn>3</mn><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mi>c</mi></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠉⠠⠁⠈⠖⠃⠳⠉");
}

#[test]
fn fractions_22() {
    let expr = r#"<math><mfrac><mrow><mn>3</mn><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mi>z</mi></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠉⠠⠭⠈⠖⠽⠳⠵");
}

#[test]
fn fractions_23() {
    let expr = r#"<math><mfrac><mrow><mn>3</mn><mi>a</mi><mo>+</mo><mn>2</mn><mi>b</mi></mrow><mrow><mn>4</mn><mi>c</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠼⠉⠠⠁⠈⠖⠼⠃⠠⠃⠳⠼⠙⠠⠉");
}

#[test]
fn fractions_24() {
    let expr = r#"<math><mfrac><mrow><mn>0</mn><mo>,</mo><mn>6</mn><mi>a</mi><mo>+</mo><mn>1</mn><mo>,</mo><mn>4</mn><mi>b</mi></mrow><mrow><mn>5</mn><mi>a</mi><mo>-</mo><mn>6</mn><mi>b</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠼⠚⠂⠋⠠⠁⠖⠼⠁⠂⠙⠠⠃⠳⠼⠑⠠⠁⠤⠼⠋⠠⠃");
}

#[test]
fn fractions_25() {
    let expr = r#"<math><mi>y</mi><mo>=</mo><mfrac><mrow><mi>&#x3B1;</mi><mo>+</mo><mi>&#x3B2;</mi></mrow><mn>2</mn></mfrac><mo>=</mo><mfrac><mi>&#x3B1;</mi><mn>2</mn></mfrac><mo>+</mo><mfrac><mi>&#x3B2;</mi><mn>2</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠰⠛⠀⠶⠰⠁⠈⠖⠰⠃⠳⠆⠀⠶⠰⠁⠳⠆⠀⠖⠰⠃⠳⠆");
}

#[test]
fn fractions_26() {
    let expr = r#"<math><mn>2</mn><mfrac><mn>2</mn><mn>3</mn></mfrac><mo>+</mo><mfrac><mrow><mn>3</mn><mo>,</mo><mn>2</mn><mi>p</mi><mo>-</mo><mn>1</mn><mo>,</mo><mn>6</mn><mi>q</mi></mrow><mrow><mn>1</mn><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>5</mn></mfrac></mstyle><mo>+</mo><mstyle displaystyle="true"><mfrac><mn>1</mn><mn>2</mn></mfrac></mstyle><mi>r</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠼⠃⠒⠀⠖⠆⠼⠉⠂⠃⠠⠏⠈⠤⠼⠁⠂⠋⠟⠳⠼⠁⠼⠃⠢⠈⠖⠼⠁⠆⠗");
}

#[test]
fn fractions_27() {
    let expr = r#"<math><mn>2</mn><mfrac><mn>2</mn><mn>3</mn></mfrac><mo>+</mo><mn>3</mn><mi>p</mi><mo>-</mo><mfrac><mrow><mn>1</mn><mo>,</mo><mn>6</mn><mi>q</mi></mrow><mrow><mn>1</mn><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>5</mn></mfrac></mstyle><mo>+</mo><mstyle displaystyle="true"><mfrac><mn>1</mn><mn>2</mn></mfrac></mstyle><mi>r</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠼⠃⠒⠀⠖⠼⠉⠠⠏⠀⠤⠼⠁⠂⠋⠟⠳⠼⠁⠼⠃⠢⠈⠖⠼⠁⠆⠗");
}

#[test]
fn fractions_28() {
    let expr = r#"<math><mfrac><mrow><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>3</mn></mfrac></mstyle><mo>+</mo><mn>3</mn><mo>,</mo><mn>2</mn><mi>p</mi><mo>-</mo><mn>1</mn><mo>,</mo><mn>6</mn><mi>q</mi></mrow><mrow><mn>1</mn><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>5</mn></mfrac></mstyle></mrow></mfrac><mo>+</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mi>r</mi></math>"#;
    test_braille("Marburg", expr, "⠆⠼⠃⠒⠈⠖⠼⠉⠂⠃⠠⠏⠈⠤⠼⠁⠂⠋⠟⠳⠼⠁⠼⠃⠢⠀⠖⠼⠁⠆⠗");
}

#[test]
fn fractions_29() {
    let expr = r#"<math><mfrac><mrow><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>3</mn></mfrac></mstyle><mo>+</mo><mn>3</mn><mo>,</mo><mn>2</mn><mi>p</mi><mo>-</mo><mn>1</mn><mo>,</mo><mn>6</mn><mi>q</mi><mo>+</mo><mstyle displaystyle="true"><mfrac><mn>1</mn><mn>2</mn></mfrac></mstyle><mi>r</mi></mrow><mrow><mn>1</mn><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>5</mn></mfrac></mstyle></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠼⠃⠒⠈⠖⠼⠉⠂⠃⠠⠏⠈⠤⠼⠁⠂⠋⠟⠈⠖⠼⠁⠆⠗⠳⠼⠁⠼⠃⠢");
}

#[test]
fn fractions_30a() {
    let expr = r#"<math><mfrac><mrow><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>3</mn></mfrac></mstyle><mi>x</mi><mo>-</mo><mn>1</mn><mo>,</mo><mn>5</mn><mi>y</mi></mrow><mrow><mn>2</mn><mi>y</mi><mo>+</mo><mstyle displaystyle="true"><mfrac><mn>3</mn><mn>8</mn></mfrac></mstyle><mi>z</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠼⠃⠒⠠⠭⠀⠤⠼⠁⠂⠑⠽⠀⠳⠀⠼⠃⠽⠀⠖⠼⠉⠦⠵⠰");
}

#[test]
fn fractions_30b() {
    let expr = r#"<math><mfrac><mrow><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>3</mn></mfrac></mstyle><mi>x</mi><mo>-</mo><mn>1</mn><mo>,</mo><mn>5</mn><mi>y</mi></mrow><mrow><mn>2</mn><mi>y</mi><mo>+</mo><mstyle displaystyle="true"><mfrac><mn>3</mn><mn>8</mn></mfrac></mstyle><mi>z</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠼⠃⠒⠠⠭⠈⠤⠼⠁⠂⠑⠽⠳⠼⠃⠽⠈⠖⠼⠉⠦⠵");
}

#[test]
fn fractions_31() {
    let expr = r#"<math><mo>-</mo><mfrac><mrow><mi>x</mi><mo>-</mo><mi>y</mi></mrow><mn>2</mn></mfrac><mo>=</mo><mfrac><mrow><mo>-</mo><mo>(</mo><mi>x</mi><mo>-</mo><mi>y</mi><mo>)</mo></mrow><mn>2</mn></mfrac><mo>=</mo><mfrac><mrow><mo>-</mo><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mn>2</mn></mfrac><mo>=</mo><mfrac><mrow><mi>y</mi><mo>-</mo><mi>x</mi></mrow><mn>2</mn></mfrac></math>"#;
    test_braille("Marburg", expr, "⠤⠆⠠⠭⠈⠤⠽⠳⠆⠀⠶⠆⠤⠣⠭⠀⠤⠽⠜⠳⠆⠀⠶⠆⠤⠭⠈⠖⠽⠳⠆⠀⠶⠽⠈⠤⠭⠳⠆");
}

#[test]
fn fractions_32() {
    let expr = r#"<math><mfrac><mrow><mn>2</mn><mi>x</mi></mrow><mrow><mn>3</mn><mi>y</mi><mo>&#xB7;</mo><mi>z</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠠⠭⠳⠼⠉⠽⠄⠵");
}

#[test]
fn fractions_33() {
    let expr = r#"<math><mfrac><mrow><mn>2</mn><mi>x</mi></mrow><mrow><mn>3</mn><mi>y</mi></mrow></mfrac><mo>&#xB7;</mo><mi>z</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠠⠭⠳⠼⠉⠽⠀⠄⠵");
}

#[test]
fn fractions_34() {
    let expr = r#"<math><msup><mi>e</mi><mrow><mi>x</mi><mo>&#xB7;</mo><mi>y</mi></mrow></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠑⠬⠭⠄⠽");
}

#[test]
fn fractions_35() {
    let expr = r#"<math><mfrac><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mrow><mi>a</mi><mo>-</mo><mi>b</mi></mrow></mfrac><mo>=</mo><mfrac><mrow><mn>1</mn><mo>+</mo><mstyle displaystyle="true"><mfrac><mi>b</mi><mi>a</mi></mfrac></mstyle></mrow><mrow><mn>1</mn><mo>-</mo><mstyle displaystyle="true"><mfrac><mi>b</mi><mi>a</mi></mfrac></mstyle></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠈⠖⠃⠳⠁⠈⠤⠃⠀⠶⠆⠼⠁⠀⠖⠆⠃⠳⠈⠀⠳⠀⠼⠁⠀⠤⠆⠃⠳⠁⠰");
}

#[test]
fn fractions_36() {
    let expr = r#"<math><mfrac><mrow><mstyle displaystyle="true"><mfrac><mi>a</mi><mn>3</mn></mfrac></mstyle><mo>-</mo><mstyle displaystyle="true"><mfrac><mi>b</mi><mn>4</mn></mfrac></mstyle></mrow><mstyle displaystyle="true"><mfrac><mi>x</mi><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow></mfrac></mstyle></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠆⠠⠁⠳⠒⠀⠤⠆⠃⠳⠲⠀⠳⠀⠰⠭⠳⠭⠈⠖⠽⠰");
}

#[test]
fn fractions_37() {
    let expr = r#"<math><mfrac><mrow><mi>x</mi><mo>+</mo><mstyle displaystyle="true"><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>y</mi><mo>-</mo><mn>1</mn></mrow></mfrac></mstyle></mrow><mrow><mi>x</mi><mo>-</mo><mstyle displaystyle="true"><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow></mfrac></mstyle></mrow></mfrac><mo>=</mo><mfrac><mstyle displaystyle="true"><mfrac><mrow><mi>x</mi><mo>(</mo><mi>y</mi><mo>-</mo><mn>1</mn><mo>)</mo><mo>+</mo><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow><mrow><mi>y</mi><mo>-</mo><mn>1</mn></mrow></mfrac></mstyle><mstyle displaystyle="true"><mfrac><mrow><mi>x</mi><mo>(</mo><mi>y</mi><mo>+</mo><mn>1</mn><mo>)</mo><mo>-</mo><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow></mfrac></mstyle></mfrac><mo>=</mo><mfrac><mstyle displaystyle="true"><mfrac><mrow><mi>x</mi><mi>y</mi><mo>-</mo><mi>x</mi><mo>+</mo><mi>x</mi><mo>-</mo><mn>1</mn></mrow><mrow><mi>y</mi><mo>-</mo><mn>1</mn></mrow></mfrac></mstyle><mstyle displaystyle="true"><mfrac><mrow><mi>x</mi><mi>y</mi><mo>+</mo><mi>x</mi><mo>-</mo><mi>x</mi><mo>-</mo><mn>1</mn></mrow><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow></mfrac></mstyle></mfrac><mo>=</mo><mfrac><mrow><mi>y</mi><mo>+</mo><mn>1</mn></mrow><mrow><mi>y</mi><mo>-</mo><mn>1</mn></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠠⠭⠀⠖⠆⠭⠈⠤⠼⠁⠳⠽⠈⠤⠼⠁⠀⠳⠀⠭⠀⠤⠆⠭⠈⠖⠼⠁⠳⠽⠈⠖⠼⠁⠰⠀⠶⠆⠆⠭⠣⠽⠀⠤⠼⠁⠜⠈⠖⠣⠭⠀⠤⠼⠁⠜⠳⠽⠈⠤⠼⠁⠀⠳⠀⠆⠭⠣⠽⠀⠖⠼⠁⠜⠈⠤⠣⠭⠀⠖⠼⠁⠜⠳⠽⠈⠖⠼⠁⠰⠠⠶⠆⠆⠭⠽⠈⠤⠭⠈⠖⠭⠈⠤⠼⠁⠳⠽⠈⠤⠼⠁⠀⠳⠀⠆⠭⠽⠈⠖⠭⠈⠤⠭⠈⠤⠼⠁⠳⠽⠈⠖⠼⠁⠰⠀⠶⠽⠈⠖⠼⠁⠳⠽⠈⠤⠼⠁");
}

#[test]
fn fractions_38() {
    let expr = r#"<math><mfrac><mrow><mn>2</mn><mstyle displaystyle="true"><mfrac><mn>2</mn><mn>3</mn></mfrac></mstyle><mo>+</mo><mstyle displaystyle="true"><mfrac><mrow><mn>1</mn><mo>,</mo><mn>2</mn><mo>-</mo><mn>0</mn><mo>,</mo><mn>7</mn></mrow><mrow><mn>2</mn><mo>,</mo><mn>6</mn></mrow></mfrac></mstyle></mrow><mrow><mstyle displaystyle="true"><mfrac><mrow><mn>1</mn><mo>,</mo><mn>2</mn><mo>+</mo><mn>0</mn><mo>,</mo><mn>6</mn></mrow><mrow><mn>0</mn><mo>,</mo><mn>8</mn></mrow></mfrac></mstyle><mo>-</mo><mn>3</mn><mstyle displaystyle="true"><mfrac><mn>4</mn><mn>5</mn></mfrac></mstyle></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠼⠃⠼⠃⠒⠀⠖⠆⠼⠁⠂⠃⠈⠤⠼⠚⠂⠛⠳⠼⠃⠂⠋⠀⠳⠀⠆⠼⠁⠂⠃⠈⠖⠼⠚⠂⠋⠳⠼⠚⠂⠓⠀⠤⠼⠉⠼⠙⠢⠰");
}

#[test]
fn fractions_39() {
    let expr = r#"<math><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>+</mo><mfrac><mn>1</mn><mrow><mn>2</mn><mo>&#xB7;</mo><mn>3</mn></mrow></mfrac><mo>+</mo><mfrac><mn>1</mn><mrow><mn>2</mn><mo>&#xB7;</mo><mn>3</mn><mo>&#xB7;</mo><mn>4</mn></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠆⠀⠖⠼⠁⠳⠼⠃⠄⠼⠉⠀⠖⠼⠁⠳⠼⠃⠄⠼⠉⠄⠼⠙");
}

// exponents and indices

#[test]
fn exponents_indices_1() {
    let expr = r#"<math><msub><mi>a</mi><mrow><mo>-</mo><mn>3</mn></mrow></msub></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠡⠤⠒");
}

#[test]
fn exponents_indices_2() {
    let expr = r#"<math><msub><mi>a</mi><mn>0</mn></msub></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠡⠴");
}

#[test]
fn exponents_indices_3() {
    let expr = r#"<math><msub><mi>a</mi><mn>12</mn></msub></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠡⠂⠆");
}

#[test]
fn exponents_indices_4() {
    let expr = r#"<math><msup><mi>b</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠃⠌⠤⠂");
}

#[test]
fn exponents_indices_5() {
    let expr = r#"<math><msup><mi>b</mi><mn>4</mn></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠃⠌⠲");
}

#[test]
fn exponents_indices_6() {
    let expr = r#"<math><msup><mi>b</mi><mn>31</mn></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠃⠌⠒⠂");
}

#[test]
fn exponents_indices_7() {
    let expr = r#"<math><msup><mi>x</mi><mrow><mo>-</mo><mn>3</mn></mrow></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠬⠤⠒");
}

#[test]
fn exponents_indices_8() {
    let expr = r#"<math><msup><mn>5</mn><mn>4</mn></msup></math>"#;
    test_braille("Marburg", expr, "⠼⠑⠬⠲");
}

#[test]
fn exponents_indices_9() {
    let expr = r#"<math><msup><mi>b</mi><mn>10</mn></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠃⠬⠂⠴");
}

#[test]
fn exponents_indices_10() {
    let expr = r#"<math><msub><mi>a</mi><mn>11</mn></msub><msub><mi>a</mi><mn>22</mn></msub><mo>-</mo><msub><mi>a</mi><mn>12</mn></msub><msub><mi>a</mi><mn>21</mn></msub></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠡⠂⠂⠁⠡⠆⠆⠀⠤⠁⠡⠂⠆⠁⠡⠆⠂");
}

#[test]
fn exponents_indices_11() {
    let expr = r#"<math><msup><mi>a</mi><mn>2</mn></msup><msup><mi>b</mi><mn>3</mn></msup><msup><mi>c</mi><mn>2</mn></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠬⠆⠃⠬⠒⠉⠬⠆");
}

#[test]
fn exponents_indices_12() {
    let expr = r#"<math><mn>2</mn><msup><mi>x</mi><mn>2</mn></msup><msqrt><mi>y</mi></msqrt></math>"#;
    test_braille("Marburg", expr, "⠼⠃⠠⠭⠬⠆⠩⠽");
}

#[test]
fn exponents_indices_13() {
    let expr = r#"<math><mfrac><msup><mi>x</mi><mn>3</mn></msup><mrow><mn>3</mn><mi>y</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠬⠒⠳⠼⠉⠽");
}

#[test]
fn exponents_indices_14() {
    let expr = r#"<math><msup><mfenced><mfrac><mn>1</mn><mn>3</mn></mfrac></mfenced><mrow><mo>-</mo><mn>4</mn></mrow></msup></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠒⠬⠤⠼⠙");
}

#[test]
fn exponents_indices_15() {
    let expr = r#"<math><msup><mi>a</mi><mfrac><mn>1</mn><mn>2</mn></mfrac></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠬⠼⠁⠆");
}

#[test]
fn exponents_indices_16() {
    let expr = r#"<math><msup><mn>8</mn><mrow><mo>-</mo><mn>3</mn></mrow></msup><mo>=</mo><mfrac><mn>1</mn><msup><mn>8</mn><mn>3</mn></msup></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠓⠬⠤⠒⠀⠶⠼⠁⠳⠼⠓⠬⠒");
}

#[test]
fn exponents_indices_17() {
    let expr = r#"<math><msup><mi>x</mi><mi>n</mi></msup><mo>+</mo><mn>9</mn></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠬⠝⠀⠖⠼⠊");
}

#[test]
fn exponents_indices_18() {
    let expr = r#"<math><msup><mi>y</mi><mrow><mn>2</mn><mi>n</mi><mo>-</mo><mn>3</mn></mrow></msup><mo>&#xB7;</mo><mi>z</mi></math>"#;
    test_braille("Marburg", expr, "⠠⠽⠬⠼⠃⠝⠈⠤⠼⠉⠀⠄⠵");
}

#[test]
fn exponents_indices_19() {
    let expr = r#"<math><msub><mi>a</mi><mi>n</mi></msub><mo>+</mo><mn>2</mn></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠡⠝⠀⠖⠼⠃");
}

#[test]
fn exponents_indices_20() {
    let expr = r#"<math><msub><mi>a</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub><mo>-</mo><mn>5</mn></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠡⠝⠈⠖⠼⠁⠀⠤⠼⠑");
}

#[test]
fn exponents_indices_21() {
    let expr = r#"<math><msub><mi>x</mi><mi>i</mi></msub><mo>&#xB7;</mo><msub><mi>x</mi><mi>j</mi></msub></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠡⠊⠱⠄⠭⠡⠚");
}

#[test]
fn exponents_indices_22() {
    let expr = r#"<math><msup><mi>y</mi><mrow><mn>2</mn><mi>n</mi><mo>-</mo><mn>3</mn></mrow></msup><mo>&#xB7;</mo><mi>z</mi></math>"#;
    test_braille("Marburg", expr, "⠠⠽⠬⠼⠃⠝⠈⠤⠼⠉⠱⠄⠵");
}

#[test]
fn exponents_indices_23() {
    let expr = r#"<math><msub><mi>f</mi><mi>n</mi></msub><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("Marburg", expr, "⠠⠋⠡⠝⠱⠣⠭⠜");
}

#[test]
fn exponents_indices_24() {
    let expr = r#"<math><msub><mi>g</mi><mrow><mi>i</mi><mi>j</mi></mrow></msub><mo>(</mo><mi>y</mi><mo>)</mo></math>"#;
    test_braille("Marburg", expr, "⠠⠛⠡⠊⠚⠱⠣⠽⠜");
}

#[test]
fn exponents_indices_25() {
    let expr = r#"<math><msup><mrow><mo>(</mo><msub><mi>a</mi><mi>n</mi></msub><mo>)</mo></mrow><mi>k</mi></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠡⠝⠬⠅");
}

#[test]
fn exponents_indices_26() {
    let expr = r#"<math><msup><mrow><mo>(</mo><msub><mi>P</mi><mrow><mn>2</mn><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msub><mo>)</mo></mrow><mi>m</mi></msup></math>"#;
    test_braille("Marburg", expr, "⠨⠏⠡⠼⠃⠠⠝⠈⠤⠼⠁⠬⠍");
}

#[test]
fn exponents_indices_27() {
    let expr = r#"<math><mfrac><msup><mi>x</mi><mi>n</mi></msup><mrow><mi>n</mi><mo>!</mo></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠬⠝⠳⠝⠫");
}

#[test]
fn exponents_indices_28() {
    let expr = r#"<math><mfrac><mn>1</mn><mrow><mn>4</mn><mo>+</mo><msub><mi>x</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠁⠳⠼⠙⠈⠖⠠⠭⠡⠝⠈⠖⠼⠁⠰");
}

#[test]
fn exponents_indices_29() {
    let expr = r#"<math><msup><mrow><mo>(</mo><mn>2</mn><mo>+</mo><msub><mi>x</mi><mrow><mi>n</mi><mo>+</mo><mn>5</mn></mrow></msub><mo>)</mo></mrow><mn>2</mn></msup></math>"#;
    test_braille("Marburg", expr, "⠣⠼⠃⠀⠖⠠⠭⠡⠝⠈⠖⠼⠑⠜⠬⠆");
}

#[test]
fn exponents_indices_30() {
    let expr = r#"<math><msub><mi>f</mi><msub><mi>n</mi><mi>k</mi></msub></msub><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("Marburg", expr, "⠠⠋⠐⠡⠠⠝⠡⠅⠐⠱⠣⠭⠜");
}

#[test]
fn exponents_indices_31() {
    let expr = r#"<math><msub><mi>P</mi><mn>1</mn></msub><mo>,</mo><mo>&#xA0;</mo><msub><mi>P</mi><mn>4</mn></msub><mo>,</mo><mo>&#xA0;</mo><msub><mi>P</mi><mn>9</mn></msub><mo>,</mo><mo>&#xA0;</mo><mo>.</mo><mo>.</mo><mo>.</mo><mo>,</mo><mo>&#xA0;</mo><msub><mi>P</mi><msup><mi>n</mi><mn>2</mn></msup></msub></math>"#;
    test_braille("Marburg", expr, "⠨⠏⠡⠂⠠⠂⠀⠨⠏⠡⠲⠠⠂⠀⠨⠏⠡⠔⠠⠂⠀⠄⠄⠄⠂⠠⠀⠨⠏⠐⠡⠠⠝⠬⠆⠐⠱");
}

#[test]
fn exponents_indices_32() {
    let expr = r#"<math><msub><mi>x</mi><msub><mi>n</mi><mn>1</mn></msub></msub><mo>,</mo><mo>&#xA0;</mo><msub><mi>x</mi><msub><mi>n</mi><mn>2</mn></msub></msub><mo>,</mo><mo>&#xA0;</mo><msub><mi>x</mi><msub><mi>n</mi><mn>4</mn></msub></msub><mo>,</mo><mo>&#xA0;</mo><mo>.</mo><mo>.</mo><mo>.</mo><mo>,</mo><mo>&#xA0;</mo><msub><mi>x</mi><msub><mi>n</mi><msup><mn>2</mn><mi>k</mi></msup></msub></msub></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠐⠡⠝⠡⠂⠐⠱⠠⠂⠀⠭⠐⠡⠝⠡⠆⠐⠱⠠⠂⠠⠀⠭⠐⠡⠝⠡⠲⠐⠱⠠⠂⠀⠄⠄⠄⠂⠀⠭⠨⠡⠝⠐⠡⠼⠃⠬⠅⠨⠱");
}

#[test]
fn exponents_indices_33() {
    let expr = r#"<math><msub><mi>A</mi><mn>1</mn></msub><mo>=</mo><mi>a</mi><mo>,</mo><mo>&#xA0;</mo><msub><mi>A</mi><mn>2</mn></msub><mo>=</mo><msup><mi>a</mi><mn>7</mn></msup><mo>,</mo><mo>&#xA0;</mo><msub><mi>A</mi><mn>3</mn></msub><mo>=</mo><msup><mi>a</mi><mn>31</mn></msup><mo>,</mo><mo>&#xA0;</mo><mo>.</mo><mo>.</mo><mo>.</mo><mo>,</mo><mo>&#xA0;</mo><msub><mi>A</mi><mi>k</mi></msub><mo>=</mo><msup><mi>a</mi><mrow><msup><mn>2</mn><mrow><mn>2</mn><mi>k</mi><mo>-</mo><mn>1</mn></mrow></msup><mo>-</mo><mn>1</mn></mrow></msup></math>"#;
    test_braille("Marburg", expr, "⠨⠁⠡⠂⠀⠶⠠⠁⠂⠀⠨⠁⠡⠆⠀⠶⠠⠁⠬⠶⠠⠂⠀⠨⠁⠡⠒⠀⠠⠁⠬⠒⠂⠠⠂⠀⠄⠄⠄⠂⠀⠨⠁⠡⠠⠅⠀⠶⠠⠁⠐⠬⠼⠃⠬⠼⠃⠅⠈⠤⠼⠁⠱⠈⠤⠼⠁⠐⠱");
}

#[test]
fn exponents_indices_34() {
    let expr = r#"<math><mfrac><msup><mi>e</mi><mstyle displaystyle="true"><mfrac><msup><mi>x</mi><mn>2</mn></msup><mn>2</mn></mfrac></mstyle></msup><msqrt><mn>2</mn><mi>&#x3C0;</mi></msqrt></mfrac></math>"#;
    test_braille("Marburg", expr, "⠆⠠⠑⠐⠬⠈⠆⠭⠬⠆⠳⠆⠐⠱⠀⠳⠀⠩⠼⠃⠰⠏⠰");
}

#[test]
fn exponents_indices_35() {
    let expr = r#"<math><msup><mi>x</mi><msub><mi>n</mi><mn>1</mn></msub></msup><mo>,</mo><mo>&#xA0;</mo><msup><mi>x</mi><msub><mi>n</mi><mn>2</mn></msub></msup><mo>,</mo><mo>&#xA0;</mo><msup><mi>x</mi><msub><mi>n</mi><mn>4</mn></msub></msup><mo>,</mo><mo>&#xA0;</mo><mo>.</mo><mo>.</mo><mo>.</mo><mo>,</mo><mo>&#xA0;</mo><msup><mi>x</mi><msub><mi>n</mi><msup><mn>2</mn><mi>k</mi></msup></msub></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠐⠬⠝⠡⠂⠠⠂⠀⠭⠐⠝⠡⠆⠠⠂⠀⠭⠐⠬⠝⠡⠲⠠⠂⠠⠄⠄⠄⠂⠀⠠⠭⠨⠬⠝⠐⠡⠼⠃⠬⠅⠨⠱");
}

#[test]
fn exponents_indices_36() {
    let expr = r#"<math><msup><mrow><mo>(</mo><msubsup><mi>x</mi><mi>n</mi><mi>i</mi></msubsup><mo>)</mo></mrow><mi>r</mi></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠡⠝⠌⠊⠬⠗");
}

#[test]
fn exponents_indices_37() {
    let expr = r#"<math><msup><mrow><mo>(</mo><msub><mi>x</mi><msup><mi>n</mi><mi>i</mi></msup></msub><mo>)</mo></mrow><mi>r</mi></msup></math>"#;
    test_braille("Marburg", expr, "⠄⠭⠐⠡⠝⠌⠊⠐⠱⠬⠗");
}

#[test]
fn exponents_indices_38() {
    let expr = r#"<math><msub><mi>x</mi><msub><mi>n</mi><msup><mi>j</mi><mi>r</mi></msup></msub></msub></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠨⠡⠝⠐⠡⠊⠬⠗⠨⠱");
}

#[test]
fn exponents_indices_39() {
    let expr = r#"<math><msup><mrow><mo>(</mo><msubsup><mi>P</mi><msub><mi>a</mi><mi>j</mi></msub><msub><mi>a</mi><mi>k</mi></msub></msubsup><mo>)</mo></mrow><mi>n</mi></msup></math>"#;
    test_braille("Marburg", expr, "⠨⠏⠐⠡⠠⠁⠡⠊⠐⠌⠁⠡⠅⠐⠱⠬⠝");
}

// Tak zwane „znaczki” (page 34)

#[test]
fn exponents_indices_40() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mi>R</mi><mo>+</mo></msup></math>"#;
    test_braille("Marburg", expr, "⠨⠨⠗⠖");
}

#[test]
fn exponents_indices_41() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mi>A</mi><mo>''</mo></math>"#;
    test_braille("Marburg", expr, "⠨⠁⠔⠔");
}

#[test]
fn exponents_indices_42() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mover><mi>C</mi><mo>^</mo></mover></math>"#;
    test_braille("Marburg", expr, "⠨⠉⠬");
}

#[test]
fn exponents_indices_43() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mover><mi>A</mi><mo>~</mo></mover></math>"#;
    test_braille("Marburg", expr, "⠨⠁⠢");
}

#[test]
fn exponents_indices_44() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mi>b</mi><mo>&#x2192;</mo></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠃⠒⠂");
}

#[test]
fn exponents_indices_45() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mi>A</mi><msup><mi>B</mi><mo>&#x2192;</mo></msup></math>"#;
    test_braille("Marburg", expr, "⠨⠁⠃⠒⠂");
}

#[test]
fn exponents_indices_46() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msub><mover><mi>x</mi><mo>&#x2D9;</mo></mover><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></msub></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠆⠡⠝⠈⠖⠼⠁");
}

#[test]
fn exponents_indices_47() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msubsup><mi>a</mi><mi>n</mi><mrow><mo>'</mo><mo>'</mo></mrow></msubsup></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠔⠔⠡⠝");
}

#[test]
fn exponents_indices_48() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mover><mi>V</mi><mo>&#xAF;</mo></mover><mn>2</mn></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠧⠒⠬⠆");
}

#[test]
fn exponents_indices_49() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msubsup><mi>y</mi><mn>1</mn><mo>'</mo></msubsup></math>"#;
    test_braille("Marburg", expr, "⠠⠽⠡⠂⠱⠔");
}

#[test]
fn exponents_indices_50() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msubsup><mi>x</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow><msup><mrow/><mo>&#x2032;&#x2032;</mo></msup></msubsup></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠡⠝⠈⠖⠼⠁⠱⠔⠔");
}

#[test]
fn exponents_indices_51() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mover accent="true"><mi>AB</mi><mo>&#x2192;</mo></mover></math>"#;
    test_braille("Marburg", expr, "⠨⠒⠂⠨⠁⠃");
}

#[test]
fn exponents_indices_52() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mover accent="true"><mi>CD</mi><mo accent="false">&#xAF;</mo></mover></math>"#;
    test_braille("Marburg", expr, "⠨⠒⠨⠉⠙");
}

#[test]
fn exponents_indices_53() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><munder><mrow><msub><mi>A</mi><mn>1</mn></msub><msub><mi>B</mi><mn>1</mn></msub><msub><mi>C</mi><mn>1</mn></msub></mrow><mo>&#x23DD;</mo></munder></math>"#;
    test_braille("Marburg", expr, "⠸⠣⠨⠁⠡⠂⠃⠡⠂⠉⠡⠂");
}

#[test]
fn exponents_indices_54() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mroot><mn>8</mn><mn>3</mn></mroot><mo>=</mo><mn>2</mn></math>"#;
    test_braille("Marburg", expr, "⠌⠒⠩⠼⠓⠀⠶⠼⠃");
}

#[test]
fn exponents_indices_55() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mroot><mi>x</mi><mi>n</mi></mroot></math>"#;
    test_braille("Marburg", expr, "⠌⠠⠝⠩⠭");
}

#[test]
fn exponents_indices_56() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mroot><mi>y</mi><mrow><mi>n</mi><mo>+</mo><mn>1</mn></mrow></mroot></math>"#;
    test_braille("Marburg", expr, "⠌⠠⠝⠈⠖⠼⠁⠩⠽");
}

#[test]
fn exponents_indices_57() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mi>l</mi><mi>o</mi><msub><mi>g</mi><mn>2</mn></msub><mn>8</mn><mo>=</mo><mn>3</mn></math>"#;
    test_braille("Marburg", expr, "⠌⠆⠫⠇⠼⠓⠀⠶⠼⠉");
}

// Roots (page 37)

#[test]
fn roots_1() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mn>16</mn></msqrt></math>"#;
    test_braille("Marburg", expr, "⠩⠼⠁⠋");
}

#[test]
fn roots_2() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mn>81</mn></msqrt><mo>=</mo><mn>9</mn></math>"#;
    test_braille("Marburg", expr, "⠩⠼⠓⠁⠀⠶⠼⠊");
}

#[test]
fn roots_3() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mroot><mn>27</mn><mn>3</mn></mroot><mo>=</mo><mn>3</mn></math>"#;
    test_braille("Marburg", expr, "⠌⠒⠩⠼⠃⠛⠀⠶⠼⠉");
}

#[test]
fn roots_4() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mi>x</mi><mo>+</mo><mfrac><mn>1</mn><mn>2</mn></mfrac></msqrt></math>"#;
    test_braille("Marburg", expr, "⠩⠠⠭⠈⠖⠼⠁⠆");
}

#[test]
fn roots_5() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mn>3</mn><msqrt><mn>2</mn><mi>x</mi></msqrt></math>"#;
    test_braille("Marburg", expr, "⠼⠉⠩⠼⠃⠠⠭");
}

#[test]
fn roots_6() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mi>x</mi></msqrt><mo>+</mo><mi>y</mi></math>"#;
    test_braille("Marburg", expr, "⠩⠠⠭⠀⠖⠽");
}

#[test]
fn roots_7() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mi>x</mi><mo>+</mo><mi>y</mi></msqrt><mo>+</mo><msqrt><mi>x</mi><mo>-</mo><mi>y</mi></msqrt></math>"#;
    test_braille("Marburg", expr, "⠩⠠⠭⠈⠖⠽⠀⠖⠩⠭⠈⠤⠽");
}

#[test]
fn roots_8() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mi>x</mi><mo>+</mo><mi>y</mi></msqrt><mo>&#xB7;</mo><msup><mi mathvariant="normal">e</mi><mi>z</mi></msup></math>"#;
    test_braille("Marburg", expr, "⠩⠠⠭⠈⠖⠽⠱⠄⠑⠬⠵");
}

#[test]
fn roots_9() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mi mathvariant="normal">x</mi><mo>+</mo><mi mathvariant="normal">y</mi></msqrt><mo>&#xB7;</mo><msqrt><mi mathvariant="normal">x</mi><mo>-</mo><mi mathvariant="normal">y</mi></msqrt></math>"#;
    test_braille("Marburg", expr, "⠩⠠⠭⠈⠖⠽⠱⠄⠩⠭⠈⠤⠽");
}

#[test]
fn roots_10() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mn>3</mn><msqrt><mn>2</mn></msqrt><mi mathvariant="normal">x</mi></math>"#;
    test_braille("Marburg", expr, "⠼⠉⠩⠼⠃⠱⠠⠭");
}

#[test]
fn roots_11() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mn>0</mn><mo>,</mo><mn>5</mn><msqrt><mi mathvariant="normal">x</mi></msqrt><mfrac><mrow><mi mathvariant="normal">x</mi><mo>+</mo><mi mathvariant="normal">y</mi></mrow><mrow><mi mathvariant="normal">x</mi><mo>-</mo><mi mathvariant="normal">y</mi></mrow></mfrac></math>"#;
    test_braille("Marburg", expr, "⠼⠚⠂⠑⠩⠠⠭⠱⠆⠭⠀⠖⠽⠀⠳⠀⠭⠀⠤⠽⠰");
}

#[test]
fn roots_12() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mi>ab</mi></msqrt><mo>=</mo><msqrt><mi mathvariant="normal">a</mi></msqrt><msqrt><mi mathvariant="normal">b</mi></msqrt></math>"#;
    test_braille("Marburg", expr, "⠩⠠⠁⠃⠀⠶⠩⠁⠩⠃");
}

#[test]
fn roots_13() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mi mathvariant="normal">a</mi><msqrt><mn>2</mn><mi mathvariant="normal">a</mi></msqrt><msqrt><mn>3</mn><mi mathvariant="normal">b</mi></msqrt></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠩⠼⠃⠠⠁⠩⠼⠉⠠⠃");
}

#[test]
fn roots_14() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mo>(</mo><mi mathvariant="normal">a</mi><mo>+</mo><msqrt><mi>ab</mi></msqrt><msup><mo>)</mo><mn>2</mn></msup><mo>-</mo><mi mathvariant="normal">b</mi></math>"#;
    test_braille("Marburg", expr, "⠣⠠⠁⠀⠖⠩⠁⠃⠜⠬⠆⠀⠤⠃");
}

#[test]
fn roots_15() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mfrac><msqrt><mn>2</mn></msqrt><mn>2</mn></mfrac><mo>&#x2248;</mo><mn>0</mn><mo>,</mo><mn>7071</mn></math>"#;
    test_braille("Marburg", expr, "⠩⠼⠃⠳⠼⠃⠀⠢⠢⠼⠚⠂⠛⠚⠛⠁");
}

#[test]
fn roots_16() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mn>2</mn><msqrt><mn>2</mn></msqrt></msqrt><mo>=</mo><msqrt><mn>2</mn></msqrt><mo>&#xB7;</mo><mroot><mn>2</mn><mn>4</mn></mroot></math>"#;
    test_braille("Marburg", expr, "⠐⠩⠼⠃⠩⠼⠃⠀⠶⠩⠼⠃⠀⠄⠌⠲⠩⠼⠃");
}

#[test]
fn roots_17() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mfrac><mi>a</mi><mi>b</mi></mfrac></msqrt><mo>=</mo><mfrac><msqrt><mi>a</mi></msqrt><msqrt><mi>b</mi></msqrt></mfrac></math>"#;
    test_braille("Marburg", expr, "⠐⠩⠠⠁⠳⠃⠀⠶⠩⠁⠳⠩⠃");
}

#[test]
fn roots_18() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mi>x</mi><mo>+</mo><mi>y</mi><mo>-</mo><mn>2</mn><msqrt><mi>x</mi><mi>y</mi></msqrt></msqrt></math>"#;
    test_braille("Marburg", expr, "⠐⠩⠠⠭⠈⠖⠽⠈⠤⠼⠃⠩⠭⠽");
}

#[test]
fn roots_19() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mn>2</mn><mi>x</mi><mo>+</mo><msqrt><mi>x</mi><mo>-</mo><mn>2</mn></msqrt><mo>-</mo><mn>3</mn></msqrt></math>"#;
    test_braille("Marburg", expr, "⠐⠩⠼⠃⠠⠭⠈⠖⠩⠭⠈⠤⠼⠃⠱⠈⠤⠼⠉");
}

#[test]
fn roots_20() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mn>2</mn><mi>x</mi><msqrt><mi>x</mi><mo>+</mo><msqrt><mn>2</mn><mo>-</mo><mi>x</mi></msqrt></msqrt><mo>+</mo><mn>5</mn></msqrt></math>"#;
    test_braille("Marburg", expr, "⠨⠩⠼⠃⠠⠭⠐⠩⠭⠈⠖⠩⠼⠃⠈⠤⠭⠐⠱⠀⠖⠼⠑⠨⠱");
}

#[test]
fn roots_21() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mi>x</mi><mo>+</mo><mi>y</mi><mo>-</mo><mn>2</mn><msqrt><mi>x</mi><mo>+</mo><mi>y</mi></msqrt></msqrt></math>"#;
    test_braille("Marburg", expr, "⠨⠩⠠⠭⠀⠖⠽⠀⠤⠼⠃⠩⠭⠈⠖⠽⠨⠱");
}

#[test]
fn roots_22() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><mi>x</mi><mo>+</mo><mi>y</mi><mo>-</mo><mn>2</mn><msqrt><mi>x</mi><mo>+</mo><mi>y</mi></msqrt><mo>+</mo><msqrt><mi>x</mi></msqrt></msqrt></math>"#;
    test_braille("Marburg", expr, "⠨⠩⠠⠭⠀⠖⠽⠀⠤⠼⠃⠩⠭⠈⠖⠽⠀⠖⠩⠭⠨⠱");
}

#[test]
fn roots_23() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msqrt><msqrt><msqrt><mfrac><mn>1</mn><mn>8</mn></mfrac></msqrt><mo>-</mo><mfrac><mn>1</mn><mn>8</mn></mfrac></msqrt><mo>-</mo><mfrac><mn>1</mn><mn>8</mn></mfrac></msqrt></math>"#;
    test_braille("Marburg", expr, "⠨⠩⠐⠩⠩⠼⠁⠦⠱⠈⠤⠼⠁⠦⠀⠤⠼⠁⠦⠨⠱");
}

#[test]
fn roots_24() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><msup><mo>)</mo><mn>2</mn></msup><mo>=</mo><msup><mi>a</mi><mn>2</mn></msup><mo>+</mo><mn>2</mn><mi>a</mi><mi>b</mi><mo>+</mo><msup><mi>b</mi><mn>2</mn></msup></math>"#;
    test_braille("Marburg", expr, "⠣⠠⠁⠀⠖⠃⠜⠬⠆⠀⠶⠁⠬⠆⠀⠖⠼⠃⠠⠁⠃⠀⠖⠃⠬⠆");
}

#[test]
fn roots_25() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mi>C</mi><mi>k</mi></msup></math>"#;
    test_braille("Marburg", expr, "⠨⠉⠌⠠⠅");
}

#[test]
fn roots_26() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msubsup><mi>V</mi><mi>n</mi><mi>k</mi></msubsup></math>"#;
    test_braille("Marburg", expr, "⠨⠧⠌⠠⠅⠡⠝");
}

#[test]
fn roots_27() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mfrac><msup><mi>a</mi><mi>x</mi></msup><mi>y</mi></mfrac></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠬⠭⠳⠽");
}

#[test]
fn roots_28() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mi>a</mi><mfrac><mi>x</mi><mi>y</mi></mfrac></msup></math>"#;
    test_braille("Marburg", expr, "⠠⠁⠐⠬⠭⠳⠽");
}

#[test]
fn roots_29() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msub><mi>x</mi><mn>2</mn></msub><mo>=</mo><mfrac><mrow><mo>-</mo><mi>b</mi><mo>+</mo><msqrt><msup><mi>b</mi><mn>2</mn></msup><mo>-</mo><mn>4</mn><mi>a</mi><mi>c</mi></msqrt></mrow><mrow><mn>2</mn><mi>a</mi></mrow></mfrac><mo>,</mo></math>"#;
    test_braille("Marburg", expr, "⠠⠭⠡⠆⠀⠶⠆⠤⠃⠀⠖⠀⠩⠃⠬⠆⠈⠤⠼⠙⠠⠁⠉⠀⠳⠀⠼⠃⠄⠁⠰");
}

// Functions (page 40)

#[test]
fn functions_1() {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mi>y</mi><mo>=</mo><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></math>"#;
    test_braille("Marburg", expr, "⠠⠽⠀⠶⠋⠣⠭⠜");
}
