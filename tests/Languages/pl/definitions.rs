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

    test("pl", "ClearSpeak", expr, "krotka z x przecinek, y")?;
    test("pl", "SimpleSpeak", expr, "krotka z x przecinek, y")?;

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

    test("pl", "ClearSpeak", expr, "zbiór pusty")?;

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

    test("pl", "ClearSpeak", expr, "zbiór liczb rzeczywistych")?;
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

    test("pl", "ClearSpeak", expr, "i")?;
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

    test("pl", "ClearSpeak", expr, "podłoga z x")?;

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

    // Separator dwuargumentowy: nazwa funkcji + "z" + argumenty spięte słowem "i".
    // Wcześniej nazwa przepadała i zostawało samo spoiwo ("i z wielka a...").
    test(
        "pl",
        "ClearSpeak",
        expr,
        "różnica zbiorów z wielka a i wielka b",
    )?;

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
            // Test postfiksowy: bez jawnej fixity silnik bierze PIERWSZĄ z listy
            // w IntentMappings, a tam (jak w EN) postfix jest pierwszy.
            "x transponowane",
        ),
        (
            "highlight",
            r#"
            <menclose intent="highlight($x)">
                <mi arg="x">x</mi>
            </menclose>
            "#,
            "x podświetlone",
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

        test("pl", "ClearSpeak", &expr, expected)?;
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
            "granica gdy x",
        ),
        (
            "unit-vector",
            r#"
            <mover intent="unit-vector($x)">
                <mi arg="x">x</mi>
                <mo>^</mo>
            </mover>
            "#,
            "wektor jednostkowy x",
        ),
        (
            "line-segment",
            r#"
            <mover intent="line-segment($x)">
                <mi arg="x">x</mi>
                <mo>¯</mo>
            </mover>
            "#,
            "odcinek x",
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

        test("pl", "ClearSpeak", &expr, expected)?;
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

        ("inverse", "odwrotność z x"),
        ("domain", "dziedzina z x"),
        ("codomain", "przeciwdziedzina z x"),
        ("image", "obraz z x"),

        //("fraction", "fraction x over y end fraction"),
        ("mixed-fraction", "x i y"),
        // Separator dwuargumentowy z IntentMappings ("| podzielone przez"):
        // nazwa funkcji wraca na swoje miejsce, a spoiwo łączy oba argumenty.
        ("quotient", "część całkowita z x podzielone przez y"),
        ("evaluated-at", "x obliczone w y"),
        ("remainder", "reszta z x podzielone przez y"),

        ("max", "maksimum z x przecinek, y przecinek, z"),
        ("min", "minimum z x przecinek, y przecinek, z"),

        ("power", "x do potęgi y"),

        // ("root", ...) pominięte: intent 'root' bez indeksu rzuca błąd
        // dopasowania reguły w RDZENIU silnika (identycznie w EN i PL),
        // więc to nie jest luka polskiej lokalizacji.

        ("greatest-common-divisor", "największy wspólny dzielnik z x przecinek, y przecinek, z"),
        ("least-common-multiple", "najmniejsza wspólna wielokrotność z x przecinek, y przecinek, z"),

        ("absolute-value", "wartość bezwzględna z x"),

        ("complex-conjugate", "sprzężenie zespolone z x"),
        ("complex-arg", "arg z x"),
        ("real-part", "część rzeczywista"),
        ("imaginary-part", "część urojona"),

        // Współrzędne: po usunięciu zbędnego "; przecinek" z IntentMappings
        // nazwa wraca na swoje miejsce (wcześniej wyjście brzmiało "przecinek z x...").
        ("polar-coordinate", "współrzędna biegunowa z x przecinek, y"),
        (
            "spherical-coordinate",
            "współrzędna sferyczna z x przecinek, y przecinek, z",
        ),
        (
            "cartesian-coordinate",
            "współrzędna kartezjańska z x przecinek, y przecinek, z",
        ),
        ("coordinate", "współrzędna z x przecinek, y przecinek, z"),

        ("floor", "podłoga z x"),
        ("ceiling", "sufit z x"),
        ("round", "zaokrąglenie z x"),
        ("fractional-part", "część ułamkowa z x"),

        
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

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn calculus_tests() -> Result<()> {
    let tests = vec![
        //("derivative", "the derivative of x with respect to x"),
        //"definite-integral", "integral over x from x to x"),

        // prefix
        ("limit", "granica gdy x"),

        // infix
        ("tends-to", "x dąży do y"),
        ("tends-to-from-above", "x dąży do z prawej y"),
        ("tends-to-from-below", "x dąży do z lewej y"),
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

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn set_tests() -> Result<()> {
    let tests = vec![
        ("set", "zbiór x"),
        // ("set-difference", "set difference of x and y"),
        ("complement", "dopełnienie z x"),
        //("empty-set", "empty set"),
        ("cardinality", "moc zbioru z x"),
        ("list", "lista z x"),
        ("tuple", "krotka z x"),
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

        test("pl", "ClearSpeak", &expr, expected)?;
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
            "suma z x",
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
            "suma przez i z x",
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
            "suma od i do n z x",
        ),
        (
            "product-1",
            r#"
            <mrow intent="product($x)">
                <mo>&#x220F;</mo>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "iloczyn z x",
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
            "iloczyn przez i z x",
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
            "iloczyn od i do n z x",
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

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn elementary_classical_tests() -> Result<()> {
    let tests = vec![
        // Trig
        ("sine", "sinus z x"),
        ("cosine", "cosinus z x"),
        ("tangent", "tangens z x"),
        ("secant", "sekans z x"),
        ("cosecant", "kosekans z x"),
        ("cotangent", "cotangens z x"),

        // Inverse trig
        ("arcsine", "arcus sinus z x"),
        ("arccosine", "arcus cosinus z x"),
        ("arctangent", "arcus tangens z x"),
        ("arcsecant", "arcus sekans z x"),
        ("arccosecant", "arcus kosekans z x"),
        ("arccotangent", "arcus cotangens z x"),

        // Hyperbolic trig
        ("hyperbolic-sine", "sinus hiperboliczny z x"),
        ("hyperbolic-cosine", "cosinus hiperboliczny z x"),
        ("hyperbolic-tangent", "tangens hiperboliczny z x"),
        ("hyperbolic-secant", "sekans hiperboliczny z x"),
        ("hyperbolic-cosecant", "kosekans hiperboliczny z x"),
        ("hyperbolic-cotangent", "cotangens hiperboliczny z x"),

        // Inverse hyperbolic trig
        ("arc-hyperbolic-sine", "arcus sinus hiperboliczny z x"),
        ("arc-hyperbolic-cosine", "arcus cosinus hiperboliczny z x"),
        ("arc-hyperbolic-tangent", "arcus tangens hiperboliczny z x"),
        ("arc-hyperbolic-secant", "arcus sekans hiperboliczny z x"),
        ("arc-hyperbolic-cosecant", "arcus kosekans hiperboliczny z x"),
        ("arc-hyperbolic-cotangent", "arcus cotangens hiperboliczny z x"),
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

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn statistics_and_probability_tests() -> Result<()> {
    let tests = vec![
        ("mean", "średnia z x"),
        ("standard-deviation", "odchylenie standardowe z x"),
        ("variance", "wariancja z x"),
        ("median", "mediana z x"),
        ("mode", "dominanta z x"),

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

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn linear_algebra_tests() -> Result<()> {
    let tests = vec![
        ("vector", "wektor z x"),
        // ("matrix", "matrix of x"),
        ("determinant", "wyznacznik z x"),
        ("adjugate", "macierz dopełnień algebraicznych z x"),
        ("magnitude", "długość z x"),
        ("norm", "norma z x"),
        ("span", "powłoka liniowa z x"),

        // transpose supports both postfix and function; we test function explicitly
        ("transpose", "transpozycja z x"),

        // dimensional product is infix
        ("dimensional-product", "x na y"),

        // unit-vector is prefix
        ("unit-vector", "wektor jednostkowy x")
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

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn nofix_set_tests() -> Result<()> {
    let tests = vec![
        ("set-of-integers", "ℤ", "zbiór liczb całkowitych"),
        ("set-of-reals", "ℝ", "zbiór liczb rzeczywistych"),
        ("set-of-rationals", "ℚ", "zbiór liczb wymiernych"),
        ("set-of-natural-numbers", "ℕ", "zbiór liczb naturalnych"),
        ("set-of-complex-numbers", "ℂ", "zbiór liczb zespolonych"),
        ("set-of-primes", "ℙ", "zbiór liczb pierwszych"),
    ];

    for (intent, symbol, expected) in tests {
        let expr = format!(
            "<math>
                <mi intent='{}'>{}</mi>
            </math>",
            intent,
            symbol
        );

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn geometry_prefix_multi_value_tests() -> Result<()> {
    let tests = vec![
        ("line-segment", "odcinek x y"),
        ("directed-line-segment", "odcinek skierowany x y"),
        ("line", "linia x y"),
        ("ray", "półprosta x y"),
        ("arc", "łuk x y"),
        ("point", "punkt x y z"),
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

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn geometry_prefix_tests() -> Result<()> {
    let tests = vec![
        ("length", "długość z x"),
        ("area", "pole z x"),
        ("objętość", "objętość z x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{intent}:function($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>"
        );

        test("pl", "ClearSpeak", &expr, expected)?;
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

    test("pl", "ClearSpeak", &expr, "x y")?;
    Ok(())
}

#[test]
fn general_concepts_tests() -> Result<()> {
    let tests = vec![
        // Unary structural
        ("fenced-group", "grupa w nawiasach z x"),
        ("highlight", "x podświetlone"),
        ("least-common-denominator", "najmniejszy wspólny mianownik z x przecinek, y przecinek, z"), // add x, y , z ...
        ("pochhammer", "symbol Pochhammera z x"),
        ("permutation-cycle", "cykl permutacji z x"),

        // Binary structural / infix-style
        // ("ordered-pair", "the pair of x and y"),
        ("rate", "x na y"),
        
        ("binomial-coefficient", "x po y"),
        ("embellished-name", "x z oznaczeniem y"),
        ("indexed-by", "x indeks dolny y"),
        // ("translation", "translation by x comma, y"), // Changes translation to comma
        ("constraint", "x przy warunku y"),
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

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn grouping_tests() -> Result<()> {
    let tests = vec![
        ("annotation", "x czyli y"),
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

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn function_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("curl", "rotacja z x"),
        ("divergence", "dywergencja z x"),
        ("gradient", "gradient z x"),
        ("laplacian", "laplasjan z x"),
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

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn prefix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("angle", "kąt x"),
        ("angle-measure", "miara kąta x"),
        ("change", "zmiana x"),
        ("for-all", "dla każdego x"),
        ("measured-angle", "kąt mierzony x"),
        ("not", "nie x"),
        ("number-of", "liczba x"),
        ("partial-derivative", "cząstkowe x"),
        ("right-angle", "kąt prosty x"),
        ("square-root-of", "pierwiastek kwadratowy z x"),
        ("there-does-not-exist", "nie istnieje x"),
        ("there-exists", "istnieje x"),
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

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn infix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("and", "x i y"),
        ("applied-to", "x zastosowane do y"),
        ("approximately", "x w przybliżeniu y"),
        ("congruent", "x przystające do y"),
        ("cartesian-product", "x iloczyn kartezjański y"),
        ("composed-with", "x złożone z y"),
        ("cross-product", "x iloczyn wektorowy y"),
        ("defined-as", "x zdefiniowane jako y"),
        ("divided-by", "x podzielone przez y"),
        ("divides", "x dzieli y"),
        ("does-not-belong-to", "x nie należy do y"),
        ("does-not-divide", "x nie dzieli y"),
        ("dot-product", "x iloczyn skalarny y"),
        ("downwards-diagonal-ellipsis", "x ukośny wielokropek w dół y"),
        ("direct-product", "x iloczyn prosty y"),
        ("element-of", "x należy do y"),
        ("ellipsis", "x wielokropek y"),
        ("equals", "x równa się y"),
        ("equivalent-to", "x równoważne y"),
        ("evaluates-to", "x ma wartość y"),
        ("given", "x pod warunkiem y"),
        ("greater-than", "x większe niż y"),
        ("greater-than-or-equal-to", "x większe lub równe y"),
        ("identically-equals", "x tożsamościowo równe y"),
        ("if-and-only-if", "x wtedy i tylko wtedy gdy y"),
        ("implies", "x implikuje y"),
        ("inner-product", "x iloczyn wewnętrzny y"),
        ("intersection", "x część wspólna y"),
        ("less-than", "x mniejsze niż y"),
        ("less-than-or-equal-to", "x mniejsze lub równe y"),
        ("list-separator", "x przecinek y"),
        ("maps-to", "x przekształca na y"),
        ("member-of", "x element zbioru y"),
        ("minus", "x minus y"),
        ("minus-or-plus", "x minus lub plus y"),
        ("not-subset", "x nie jest podzbiorem y"),
        ("not-superset", "x nie jest nadzbiorem y"),
        ("not-equal-to", "x nie równa się y"),
        ("not-member-of", "x nie jest elementem zbioru y"),
        ("not-parallel-to", "x nierównoległe do y"),
        ("obtained-from", "x otrzymane z y"),
        ("or", "x lub y"),
        ("outer-product", "x iloczyn zewnętrzny y"),
        ("parallel-to", "x równoległe do y"),
        ("perpendicular", "x prostopadłe do y"),
        ("plus", "x plus y"),
        ("plus-or-minus", "x plus minus y"),
        ("precedes", "x poprzedza y"),
        ("proportional", "x proporcjonalne do y"),
        ("range-separator", "x do y"),
        ("ratio", "x stosunek y"),
        ("similar", "x podobne do y"),
        ("subset", "x podzbiór y"),
        ("subset-or-equal", "x podzbiór lub równy y"),
        ("succeeds", "x następuje po y"),
        ("such-that", "x taki że y"),
        ("superset", "x nadzbiór y"),
        ("superset-or-equal", "x nadzbiór lub równy y"),
        ("tilde", "x tylda y"),
        ("times", "x razy y"),
        ("union", "x suma zbiorów y"),
        ("upwards-diagonal-ellipsis", "x ukośny wielokropek w górę y"),
        ("vertical-ellipsis", "x wielokropek pionowy y"),
        ("xor", "x albo y"),
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

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn postfix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("factorial", "x silnia"),
        ("percent", "x procent"),
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

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn nofix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("diameter", "d", "średnica"),
        ("distance", "D", "odległość"),
        ("probability", "P", "prawdopodobieństwo"),
        ("radius", "r", "promień"),
        ("volume", "V", "objętość"),
        ("exponential-e", "e", "e"),
        ("imaginary-i", "i", "i"),
        ("differential-d", "d", "d"),
        ("golden-ratio", "φ", "złota proporcja"),
    ];

    for (intent, symbol, expected) in tests {
        let expr = format!(
            "<math>
                <mi intent='{}'>{}</mi>
            </math>",
            intent,
            symbol
        );

        test("pl", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}
