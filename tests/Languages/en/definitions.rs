/// Tests for rules in definitions:
/// *  modified var
use crate::common::*;
use anyhow::Result;

#[test]
fn tuple_basic() -> Result<()> {
    // function 
    let expr = r#"
      <math>
        <mrow intent="tuple($x,$y)">
          <mi arg="x">x</mi>
          <mi arg="y">y</mi>
        </mrow>
      </math>
    "#;

    test("en", "ClearSpeak", expr, "the tuple of x comma, y")?;
    test("en", "SimpleSpeak", expr, "the tuple of x comma, y")?;

    return Ok(());
}

#[test]
fn my_set_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="set($x,$y)">
          <mi arg="x">x</mi>
          <mi arg="y">y</mi>
        </mrow>
      </math>
    "#;

    test("en", "ClearSpeak", expr, "the set of x comma, y")?;

    Ok(())
}

#[test]
fn fixed_test() -> Result<()> {
    let expr = r#"
      <math>
        <mi intent="set-of-reals">R
        </mi>
      </math>
    "#;

    test("en", "ClearSpeak", expr, "set of all real numbers")?;
    Ok(())
}

#[test]
fn i_test() -> Result<()> {
    let expr = r#"
      <math>
        <mi intent="imaginary-i">i
        </mi>
      </math>
    "#;

    test("en", "ClearSpeak", expr, "i")?;
    Ok(())
}


#[test]
fn floor_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="floor($x)">
          <mi arg="x">x</mi>
        </mrow>
      </math>
    "#;

    test("en", "ClearSpeak", expr, "floor of x")?;

    Ok(())
}

#[test]
fn ratio_function_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="ratio($x,$y)">
          <mi arg="x">6</mi>
          <mi arg="y">3</mi>
        </mrow>
      </math>
    "#;

    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "ratio 6 to 3")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "the ratio of 6 to 3")?;
    test_prefs("en", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "the ratio of 6 to 3")?;
    test("en", "ClearSpeak", expr, "the ratio of 6 to 3")?;
    Ok(())
}

#[test]
fn set_difference_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="set-difference($A,$B)">
          <mi arg="A">A</mi>
          <mo>&#x2216;</mo>
          <mi arg="B">B</mi>
        </mrow>
      </math>
    "#;

    test( "en", "ClearSpeak", expr, "set difference of cap eigh and cap b")?;

    Ok(())
}

#[test]
fn postfix_test() -> Result<()> {
    let tests = [
        (
            "transpose",
            r#"
            <msup intent="transpose($x)">
                <mi arg="x">x</mi>
                <mo>T</mo>
            </msup>
            "#,
            "x transpose",
        ),
        (
            "highlight",
            r#"
            <menclose intent="highlight($x)">
                <mi arg="x">x</mi>
            </menclose>
            "#,
            "x highlighted",
        ),
    ];

    // Loop through all test cases, name _, body, and expected result
    for (_, body, expected) in tests {
        let expr = format!(
            r#"
            <math>
                {}
            </math>
            "#,
            body
        );

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn prefix_test() -> Result<()> {
    // Prefix test limit, unit-vector, line-segment
    // Directed-line-segment, line, ray, arc
    let tests = [
        (
            "limit",
            r#"
            <mrow intent="limit($x)">
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "the limit as x",
        ),
        (
            "unit-vector",
            r#"
            <mover intent="unit-vector($x)">
                <mi arg="x">x</mi>
                <mo>^</mo>
            </mover>
            "#,
            "unit vector x",
        ),
        (
            "line-segment",
            r#"
            <mover intent="line-segment($x)">
                <mi arg="x">x</mi>
                <mo>¯</mo>
            </mover>
            "#,
            "line segment x",
        ),
    ];

    for (_, body, expected) in tests {
        let expr = format!(
            r#"
            <math>
                {}
            </math>
            "#,
            body
        );

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn functions_and_inverses_tests() -> Result<()> {
    let tests = vec![
        //("closed-interval", "closed interval between x and y"),
        //("closed-open-interval", "interval between x included and y"),
        //("open-closed-interval", "interval between x and y included"),
        //("open-interval", "open interval between x and y"),

        ("inverse", "inverse of x"),
        ("domain", "domain of x"),
        ("codomain", "codomain of x"),
        ("image", "image of x"),

        //("fraction", "fraction x over y end fraction"),
        ("mixed-fraction", "x and y"),
        ("quotient", "integer part of x divided by y"),
        ("evaluated-at", "x evaluated at y"),
        ("remainder", "the remainder of x divided by y"),

        ("max", "max of x comma, y comma, z"),
        ("min", "min of x comma, y comma, z"),

        ("power", "x to the y-th power"),

        // ("root", ...) skipped: ClearSpeak `tag: root` requires an index
        // (same as the Polish suite).

        ("greatest-common-divisor", "the gcd of x comma, y comma, z"),
        ("least-common-multiple", "the lcm of x comma, y comma, z"),

        ("absolute-value", "the absolute value of x"),

        ("complex-conjugate", "complex conjugate of x"),
        ("complex-arg", "arg of x"),
        ("real-part", "the real part of x"),
        ("imaginary-part", "the imaginary part of x"),

        ("polar-coordinate", "polar coordinate of x comma, y"),
        ("spherical-coordinate", "spherical coordinate of x comma, y comma, z"),
        ("cartesian-coordinate", "cartesian coordinate of x comma, y comma, z"),
        ("coordinate", "coordinate of x comma, y comma, z"),

        ("floor", "floor of x"),
        ("ceiling", "ceiling of x"),
        ("round", "rounded-value of x"),
        ("fractional-part", "fractional part of x"),

        
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            
            "max"
            | "min"
            | "greatest-common-divisor"
            | "least-common-multiple"
            | "spherical-coordinate"
            | "cartesian-coordinate"
            | "coordinate" => {
                format!(
                    "<math>
                        <mrow intent='{}($a,$b,$c)'>
                            <mi arg='a'>x</mi>
                            <mi arg='b'>y</mi>
                            <mi arg='c'>z</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }

            "power"
            | "mixed-fraction"
            | "quotient"
            | "evaluated-at"
            | "remainder"
            | "closed-interval"
            | "closed-open-interval"
            | "open-closed-interval"
            | "open-interval" 
            | "polar-coordinate" => {
                format!(
                    "<math>
                        <mrow intent='{}($a,$b)'>
                            <mi arg='a'>x</mi>
                            <mi arg='b'>y</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }

            _ => {
                format!(
                    "<math>
                        <mrow intent='{}($x)'>
                            <mi arg='x'>x</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
        };

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn calculus_tests() -> Result<()> {
    let tests = vec![
        //("derivative", "the derivative of x with respect to x"),
        //"definite-integral", "integral over x from x to x"),

        // prefix
        ("limit", "the limit as x"),

        // infix
        ("tends-to", "x tends to y"),
        ("tends-to-from-above", "x tends to from above y"),
        ("tends-to-from-below", "x tends to from below y"),
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            "limit" => {
                format!(
                    "<math>
                        <mrow intent='{}($x)'>
                            <mi arg='x'>x</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }

            _ => {
                // infix cases
                format!(
                    "<math>
                        <mrow intent='{}($a,$b)'>
                            <mi arg='a'>x</mi>
                            <mi arg='b'>y</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
        };

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn set_tests() -> Result<()> {
    let tests = vec![
        ("set", "the set of x"),
        // ("set-difference", "set difference of x and y"),
        ("complement", "complement of x"),
        //("empty-set", "empty set"),
        ("cardinality", "cardinality of x"),
        ("list", "list of x"),
        ("tuple", "the tuple of x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn sequence_and_series_intents() -> Result<()> {
    let tests = [
        (
            "sum-1",
            r#"
            <mrow intent="sum($x)">
                <mo>&#x2211;</mo>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "sum of x",
        ),
        (
            "sum-2",
            r#"
            <mrow intent="sum($i,$x)">
                <mo>&#x2211;</mo>
                <mi arg="i">i</mi>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "sum over i of x",
        ),
        (
            "sum-3",
            r#"
            <mrow intent="sum($i,$n,$x)">
                <mo>&#x2211;</mo>
                <mi arg="i">i</mi>
                <mi arg="n">n</mi>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "sum from i to n of x",
        ),
        (
            "product-1",
            r#"
            <mrow intent="product($x)">
                <mo>&#x220F;</mo>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "product of x",
        ),
        (
            "product-2",
            r#"
            <mrow intent="product($i,$x)">
                <mo>&#x220F;</mo>
                <mi arg="i">i</mi>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "product over i of x",
        ),
        (
            "product-3",
            r#"
            <mrow intent="product($i,$n,$x)">
                <mo>&#x220F;</mo>
                <mi arg="i">i</mi>
                <mi arg="n">n</mi>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "product from i to n of x",
        ),
    ];

    for (_, body, expected) in tests {
        let expr = format!(
            r#"
            <math>
                {}
            </math>
            "#,
            body
        );

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn elementary_classical_tests() -> Result<()> {
    let tests = vec![
        // Trig
        ("sine", "sine of x"),
        ("cosine", "cosine of x"),
        ("tangent", "tangent of x"),
        ("secant", "secant of x"),
        ("cosecant", "cosecant of x"),
        ("cotangent", "cotangent of x"),

        // Inverse trig
        ("arcsine", "arcsine of x"),
        ("arccosine", "arccosine of x"),
        ("arctangent", "arctangent of x"),
        ("arcsecant", "arcsecant of x"),
        ("arccosecant", "arc-cosecant of x"),
        ("arccotangent", "arc-cotangent of x"),

        // Hyperbolic trig
        ("hyperbolic-sine", "shine of x"),
        ("hyperbolic-cosine", "cosh of x"),
        ("hyperbolic-tangent", "tanch of x"),
        ("hyperbolic-secant", "sech of x"),
        ("hyperbolic-cosecant", "cosech of x"),
        ("hyperbolic-cotangent", "coth of x"),

        // Inverse hyperbolic trig
        ("arc-hyperbolic-sine", "arc shine of x"),
        ("arc-hyperbolic-cosine", "arc cosh of x"),
        ("arc-hyperbolic-tangent", "arc tanch of x"),
        ("arc-hyperbolic-secant", "arc sech of x"),
        ("arc-hyperbolic-cosecant", "arc cosech of x"),
        ("arc-hyperbolic-cotangent", "arc coth of x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn statistics_and_probability_tests() -> Result<()> {
    let tests = vec![
        ("mean", "mean of x"),
        ("standard-deviation", "standard deviation of x"),
        ("variance", "variance of x"),
        ("median", "median of x"),
        ("mode", "mode of x"),

        // conditional probability typically two arguments
        // ("conditional-probability", "probability of x given y"),
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            "conditional-probability" => format!(
                "<math>
                    <mrow intent='{}($A,$B)'>
                        <mi>P</mi>
                        <mo>(</mo>
                        <mrow>
                          <mi arg='x'>x</mi>
                          <mo>|</mo>
                          <mi arg='y'>y</mi>
                        </mrow>
                        <mo>)</mo>
                    </mrow>
                </math>",
                intent
            ),
            _ => format!(
                "<math>
                    <mrow intent='{}($x)'>
                        <mi arg='x'>x</mi>
                    </mrow>
                </math>",
                intent
            ),
        };

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn linear_algebra_tests() -> Result<()> {
    let tests = vec![
        ("vector", "vector of x"),
        // ("matrix", "matrix of x"),
        ("determinant", "determinant of x"),
        ("adjugate", "adjugate of x"),
        ("magnitude", "magnitude of x"),
        ("norm", "norm of x"),
        ("span", "span of x"),

        // transpose supports both postfix and function; we test function explicitly
        ("transpose", "transpose of x"),

        // dimensional product is infix
        ("dimensional-product", "x by y"),

        // unit-vector is prefix
        ("unit-vector", "unit vector x")
    ];

    for (intent, expected) in tests {
        let expr: String = match intent {
          "dimensional-product" => {
              "<math>
                  <mrow intent='dimensional-product($x,$y)'>
                      <mi arg='x'>x</mi>
                      <mi arg='y'>y</mi>
                  </mrow>
              </math>"
              .to_string()
          }
          "transpose" => "<math>
                  <mrow intent='transpose:function($x)'>
                      <mi arg='x'>x</mi>
                  </mrow>
              </math>".to_string(),
          _ => format!(
              "<math>
                  <mrow intent='{intent}($x)'>
                      <mi arg='x'>x</mi>
                  </mrow>
              </math>"
          ),
      };

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn nofix_set_tests() -> Result<()> {
    let tests = vec![
        ("set-of-integers", "ℤ", "set of all integers"),
        ("set-of-reals", "ℝ", "set of all real numbers"),
        ("set-of-rationals", "ℚ", "set of all rational numbers"),
        ("set-of-natural-numbers", "ℕ", "set of all natural numbers"),
        ("set-of-complex-numbers", "ℂ", "set of all complex numbers"),
        ("set-of-primes", "ℙ", "set of all prime numbers"),
    ];

    for (intent, symbol, expected) in tests {
        let expr = format!(
            "<math>
                <mi intent='{}'>{}</mi>
            </math>",
            intent,
            symbol
        );

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn geometry_prefix_multi_value_tests() -> Result<()> {
    let tests = vec![
        ("line-segment", "line segment x y"),
        ("directed-line-segment", "directed line segment x y"),
        ("line", "line x y"),
        ("ray", "ray x y"),
        ("arc", "arc x y"),
        ("point", "point x y z"),
    ];

    for (intent, expected) in tests {
        let expr = match intent{
            "point" => {
                format!(
                    "<math>
                        <mrow intent='{intent}($x,$y,$z)'>
                            <mi arg='x'>x</mi>
                            <mi arg='y'>y</mi>
                            <mi arg='z'>z</mi>
                        </mrow>
                    </math>"
                )
            }

            _ => { 
                format!(
                    "<math>
                        <mrow intent='{intent}($x,$y)'>
                            <mi arg='x'>x</mi>
                            <mi arg='y'>y</mi>
                        </mrow>
                    </math>"
                )
            }
        };

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn geometry_prefix_tests() -> Result<()> {
    let tests = vec![
        ("length", "length of x"),
        ("area", "area of x"),
        ("volume", "volume of x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{intent}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>"
        );

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn separator_tests() -> Result<()> {
  let expr = "<math>
            <mrow intent='time-separator($x,$y)'>
                <mi arg='x'>x</mi>
                <mi arg='y'>y</mi>
            </mrow>
        </math>".to_string();

    test("en", "ClearSpeak", &expr, "x y")?;
    Ok(())
}

#[test]
fn general_concepts_tests() -> Result<()> {
    let tests = vec![
        // Unary structural
        ("fenced-group", "fenced-group of x"),
        ("highlight", "x highlighted"),
        ("least-common-denominator", "least common denominator of x comma, y comma, z"), // add x, y , z ...
        ("pochhammer", "permutation of x"),
        ("permutation-cycle", "permutation cycle of x"),

        // Binary structural / infix-style
        // ("ordered-pair", "the pair of x and y"),
        ("rate", "x per y"),
        
        ("binomial-coefficient", "x choose y"),
        ("embellished-name", "x with annotation y"),
        ("indexed-by", "x sub y"),
        // ("translation", "translation by of x comma, y"), // "of" after "by" is awkward; needs dedicated rule or of-suppression
        ("constraint", "x with constraint y"),
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            "ordered-pair"
            | "rate"
            | "constraint"
            | "binomial-coefficient"
            | "embellished-name"
            | "translation"
            | "indexed-by" => {
                format!(
                    "<math>
                        <mrow intent='{}($a,$b)'>
                            <mi arg='a'>x</mi>
                            <mi arg='b'>y</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
            "least-common-denominator" => {
                format!(
                    "<math>
                        <mrow intent='{}($x,$y,$z)'>
                            <mi arg='x'>x</mi>
                            <mi arg='y'>y</mi>
                            <mi arg='z'>z</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }

            // unary cases
            _ => {
                format!(
                    "<math>
                        <mrow intent='{}($x)'>
                            <mi arg='x'>x</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
        };

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn grouping_tests() -> Result<()> {
    let tests = vec![
        ("annotation", "x which is y"),
        // ("braced-group", "grouped x end-grouped"),
        // ("repeating-decimal", "repeating decimal of x"),
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            "annotation" => {
                format!(
                    "<math>
                        <mrow intent='{}($x,$y)'>
                            <mi arg='x'>x</mi>
                            <mi arg='y'>y</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
            _ => {
                format!(
                    "<math>
                        <mrow intent='{}($x)'>
                            <mi arg='x'>x</mi>
                        </mrow>
                    </math>",
                    intent
                )
            } 
        };

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn function_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("curl", "curl of x"),
        ("divergence", "divergence of x"),
        ("gradient", "gradient of x"),
        ("laplacian", "laplacian of x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn prefix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("angle", "angle x"),
        ("angle-measure", "angle measure x"),
        ("change", "change in x"),
        ("for-all", "for all x"),
        ("measured-angle", "measured angle x"),
        ("not", "not x"),
        ("number-of", "number of x"),
        ("partial-derivative", "partial x"),
        ("right-angle", "right angle x"),
        ("square-root-of", "square root of x"),
        ("there-does-not-exist", "there does not exist x"),
        ("there-exists", "there exists x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn infix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("and", "x and y"),
        ("applied-to", "x applied to y"),
        ("approximately", "x approximately y"),
        ("congruent", "x congruent to y"),
        ("cartesian-product", "x cartesian product y"),
        ("composed-with", "x composed with y"),
        ("cross-product", "x cross product y"),
        ("defined-as", "x defined as y"),
        ("divided-by", "x divided by y"),
        ("divides", "x divides y"),
        ("does-not-belong-to", "x does not belong to y"),
        ("does-not-divide", "x does not divide y"),
        ("dot-product", "x dot product y"),
        ("downwards-diagonal-ellipsis", "x downwards diagonal ellipsis y"),
        ("direct-product", "x direct product y"),
        ("element-of", "x element of y"),
        ("ellipsis", "x ellipsis y"),
        ("equals", "x equals y"),
        ("equivalent-to", "x equivalent to y"),
        ("evaluates-to", "x evaluates to y"),
        ("given", "x given y"),
        ("greater-than", "x greater than y"),
        ("greater-than-or-equal-to", "x greater than or equal to y"),
        ("identically-equals", "x identically equals y"),
        ("if-and-only-if", "x if and only if y"),
        ("implies", "x implies y"),
        ("inner-product", "x inner product y"),
        ("intersection", "x intersection y"),
        ("less-than", "x less than y"),
        ("less-than-or-equal-to", "x less than or equal to y"),
        ("list-separator", "x comma y"),
        ("maps-to", "x maps to y"),
        ("member-of", "x member of y"),
        ("minus", "x minus y"),
        ("minus-or-plus", "x minus or plus y"),
        ("not-subset", "x not subset of y"),
        ("not-superset", "x not superset of y"),
        ("not-equal-to", "x not equal to y"),
        ("not-member-of", "x not member of y"),
        ("not-parallel-to", "x not parallel to y"),
        ("obtained-from", "x obtained from y"),
        ("or", "x or y"),
        ("outer-product", "x outer product y"),
        ("parallel-to", "x parallel to y"),
        ("perpendicular", "x perpendicular to y"),
        ("plus", "x plus y"),
        ("plus-or-minus", "x plus or minus y"),
        ("precedes", "x precedes y"),
        ("proportional", "x proportional to y"),
        ("range-separator", "x through y"),
        ("ratio", "the ratio of x to y"),
        ("similar", "x similar to y"),
        ("subset", "x subset of y"),
        ("subset-or-equal", "x subset or equal to y"),
        ("succeeds", "x succeeds y"),
        ("such-that", "x such that y"),
        ("superset", "x superset of y"),
        ("superset-or-equal", "x superset or equal to y"),
        ("tilde", "x tilde y"),
        ("times", "x times y"),
        ("union", "x union y"),
        ("upwards-diagonal-ellipsis", "x upwards diagonal ellipsis y"),
        ("vertical-ellipsis", "x vertical ellipsis y"),
        ("xor", "x exclusive or y"),
    ];
    
    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($a,$b)'>
                    <mi arg='a'>x</mi>
                    <mi arg='b'>y</mi>
                </mrow>
            </math>",
            intent
        );

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn postfix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("factorial", "x factorial"),
        ("percent", "x percent"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn nofix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("diameter", "d", "diameter"),
        ("distance", "D", "distance"),
        ("probability", "P", "probability"),
        ("radius", "r", "radius"),
        ("volume", "V", "volume"),
        ("exponential-e", "e", "e"),
        ("imaginary-i", "i", "i"),
        ("differential-d", "d", "d"),
        ("golden-ratio", "φ", "golden ratio"),
    ];

    for (intent, symbol, expected) in tests {
        let expr = format!(
            "<math>
                <mi intent='{}'>{}</mi>
            </math>",
            intent,
            symbol
        );

        test("en", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}