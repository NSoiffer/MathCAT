// *** MathCAT doesn't normally want to build a binary ***
// *** This file is here because it is useful for trying out things ***

use libmathcat::interface::*;
use log::*;
use std::time::Instant;


// Maybe also have this speak to test the TTS generation.
// There is a rust winapi crate that mirrors the WinPAI and has "Speak(...)" in it
fn main() {
  env_logger::builder()
      .format_timestamp(None)
      .format_module_path(false)
      .format_indent(Some(2))
      .format_level(false)
      .init();

//    let expr = r#"
//    <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
//    <mrow>
//      <msup>
//        <mi>e</mi>
//        <mrow>
//          <mo>&#x2212;</mo>
//          <mfrac>
//            <mn>1</mn>
//            <mn>2</mn>
//          </mfrac>
//          <msup>
//            <mrow>
//              <mrow>
//                <mo>(</mo>
//                <mrow>
//                  <mfrac>
//                    <mrow>
//                      <mi>x</mi>
//                      <mo>&#x2212;</mo>
//                      <mi>&#x03BC;</mi>
//                    </mrow>
//                    <mi>&#x03C3;</mi>
//                  </mfrac>
//                </mrow>
//                <mo>)</mo>
//              </mrow>
//            </mrow>
//            <mn>2</mn>
//          </msup>
//        </mrow>
//      </msup>
//    </mrow>
//  </math>
// "#;
  // let expr = "<math display='inline' xmlns='http://www.w3.org/1998/Math/MathML'>
  //       <msup intent='power($base(2, $base),silly($exp,-1.))'>
  //       <mi arg='base'>x</mi>
  //       <mi arg='exp'>n</mi>
  //     </msup>
  //       </math>
  //     ";
  // let expr = "<mrow intent='pre@prefix(in@infix($a, x))(post@postfix($b))'>
  //     <mi arg='a'>A</mi>
  //     <mover>
  //         <mo intent='map'>⟶</mo>
  //         <mo intent='congruence'>≅</mo>
  //     </mover>
  //     <mi arg='b'>B</mi>
  //   </mrow>";
  // let expr = "<math><mi>Na</mi><mi>S</mi><mo>(</mo><mi>l</mi><mo>)</mo></math>";


  // let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML' display='block'>
  //     <mrow>
  //       <mo stretchy='false'>[</mo>
  //       <mrow>
  //         <mi>Co</mi>
  //       </mrow>
  //       <mo stretchy='false'>(</mo>
  //       <mrow>
  //         <mi>NH</mi>
  //       </mrow>
  //       <msub>
  //         <mrow>
  //           <mrow>
  //             <mpadded width='0'>
  //               <mphantom>
  //                 <mi>A</mi>
  //               </mphantom>
  //             </mpadded>
  //           </mrow>
  //         </mrow>
  //         <mrow>
  //           <mrow>
  //             <mpadded height='0'>
  //               <mn>3</mn>
  //             </mpadded>
  //           </mrow>
  //         </mrow>
  //       </msub>
  //       <mo stretchy='false'>)</mo>
  //       <msub>
  //         <mrow>
  //           <mrow>
  //             <mpadded width='0'>
  //               <mphantom>
  //                 <mi>A</mi>
  //               </mphantom>
  //             </mpadded>
  //           </mrow>
  //         </mrow>
  //         <mrow>
  //           <mrow>
  //             <mpadded height='0'>
  //               <mn>6</mn>
  //             </mpadded>
  //           </mrow>
  //         </mrow>
  //       </msub>
  //       <mo stretchy='false'>]</mo>
  //       <msup>
  //         <mrow>
  //           <mrow>
  //             <mpadded width='0'>
  //               <mphantom>
  //                 <mi>A</mi>
  //               </mphantom>
  //             </mpadded>
  //           </mrow>
  //         </mrow>
  //         <mrow>
  //           <mn>3</mn>
  //           <mo>+</mo>
  //         </mrow>
  //       </msup>
  //       <mtext>&#xA0;</mtext>
  //       <mo stretchy='false'>(</mo>
  //       <mrow>
  //         <mi>Cl</mi>
  //       </mrow>
  //       <msub>
  //         <mrow>
  //           <mrow>
  //             <mpadded width='0'>
  //               <mphantom>
  //                 <mi>A</mi>
  //               </mphantom>
  //             </mpadded>
  //           </mrow>
  //         </mrow>
  //         <mrow>
  //           <mrow>
  //             <mpadded height='0'>
  //               <mn>3</mn>
  //             </mpadded>
  //           </mrow>
  //         </mrow>
  //       </msub>
  //       <mo stretchy='false'>)</mo>
  //       <msup>
  //         <mrow>
  //           <mrow>
  //             <mpadded width='0'>
  //               <mphantom>
  //                 <mi>A</mi>
  //               </mphantom>
  //             </mpadded>
  //           </mrow>
  //         </mrow>
  //         <mrow>
  //           <mo>&#x2212;</mo>, 
  //         </mrow>
  //       </msup>
  //     </mrow>
  //   </math>";

  let expr = r#"
  <math xmlns="http://www.w3.org/1998/Math/MathML">
  <mi>i</mi>
  <mi>ω</mi>
  <mi>t</mi>
</math>
  "#;
  // let expr= "<math><mrow><mi>sin</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>+</mo><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow></math>";
  let instant = Instant::now();
  let rules_dir = std::env::current_exe().unwrap().parent().unwrap().join("../../Rules");
  let rules_dir = rules_dir.as_os_str().to_str().unwrap().to_string();
  // let rules_dir = "".to_string();    // Use MathCATRulesDir, potentially pointing to a zipped version
  if let Err(e) = set_rules_dir(rules_dir) {
    panic!("Error: exiting -- {}", errors_to_string(&e));  }

  info!("Version = '{}'", get_version());
  set_preference("Language".to_string(), "en".to_string()).unwrap();
  set_preference("DecimalSeparator".to_string(), "Auto".to_string()).unwrap();
  set_preference("BrailleCode".to_string(), "Nemeth".to_string()).unwrap();
  set_preference("TTS".to_string(), "None".to_string()).unwrap();
  set_preference("Verbosity".to_string(), "Terse".to_string()).unwrap();
  set_preference("NavVerbosity".to_string(), "Verbose".to_string()).unwrap();
  set_preference("NavMode".to_string(), "Simple".to_string()).unwrap();
  set_preference("Impairment".to_string(), "Blindness".to_string()).unwrap();
  set_preference("SpeechOverrides_CapitalLetters".to_string(), "".to_string()).unwrap();
  set_preference("MathRate".to_string(), "80".to_string()).unwrap();
  // set_preference("CapitalLetters_UseWord".to_string(), "true".to_string()).unwrap();
  // set_preference("CapitalLetters_Pitch".to_string(), "30".to_string()).unwrap();
  set_preference("CapitalLetters_Beep".to_string(), "true".to_string()).unwrap();
  set_preference("IntentErrorRecovery".to_string(), "Error".to_string()).unwrap();
  // set_preference("MathRate".to_string(), "77".to_string()).unwrap();
  
  set_preference("Bookmark".to_string(), "false".to_string()).unwrap();
  set_preference("SpeechStyle".to_string(), "MathSpeak".to_string()).unwrap();
  // set_preference("DecimalSeparators".to_string(), ",".to_string()).unwrap();
  // set_preference("BlockSeparators".to_string(), ". ".to_string()).unwrap();
  if let Err(e) = set_mathml(expr.to_string()) {
    panic!("Error: exiting -- {}", errors_to_string(&e));
  };

  // match do_navigate_command("ZoomIn".to_string())  {
  //   Err(e) => panic!("Error: exiting -- {}", errors_to_string(&e)),
  //   Ok(speech) => info!("\nZoomIn speech: '{}'", speech),
  // }
  // match do_navigate_command("MoveNext".to_string()) {
  //   Err(e) => panic!("Error: exiting -- {}", errors_to_string(&e)),
  //   Ok(speech) => info!("MoveNext speech: '{}'", speech),
  // }
  // match do_navigate_command("MoveNext".to_string()) {
  //   Err(e) => panic!("Error: exiting -- {}", errors_to_string(&e)),
  //   Ok(speech) => info!("MoveNext speech: '{}'", speech),
  // }
  debug!("Speech language is {}", get_preference("Language".to_string()).unwrap());
  debug!("DecimalSeparator: {:?}", get_preference("DecimalSeparator".to_string()).unwrap());
  debug!("DecimalSeparators: {:?}, BlockSeparators: {:?}", get_preference("DecimalSeparators".to_string()).unwrap(), get_preference("BlockSeparators".to_string()).unwrap());
  debug!("SpeechStyle: {:?}", get_preference("SpeechStyle".to_string()).unwrap());
  debug!("Verbosity: {:?}", get_preference("Verbosity".to_string()).unwrap());
  match get_spoken_text() {
    Ok(speech) => info!("Computed speech string:\n   '{}'", speech),
    Err(e) => panic!("{}", errors_to_string(&e)),
  }
 
  #[cfg(feature = "include-zip")]
  info!("***********include-zip is present**********");
  // info!("Time taken for loading+speech+braille: {}ms", instant.elapsed().as_millis());
  // let instant = Instant::now();
  // match get_spoken_text() {
  //   Ok(speech) => info!("Computed speech string:\n   '{}'", speech),
  //   Err(e) => panic!("{}", errors_to_string(&e)),
  // }
  // info!("Time taken (second time for speech): {}ms", instant.elapsed().as_millis());
  // info!("SpeechStyle: {:?}", get_preference("SpeechStyle".to_string()));
  
  debug!("BrailleCode: {:?}", get_preference("BrailleCode".to_string()).unwrap());
  match get_braille("".to_string()) {
    Ok(braille) => info!("Computed braille string:\n   '{}'", braille),
    Err(e) => panic!("{}", errors_to_string(&e)),
  }
  // let xpath_counts = libmathcat::speech::xpath_count();
  // info!("#xpath = {}; duplicates = {}", xpath_counts.0, xpath_counts.1);
  info!("Time taken (second time for speech + braille): {}ms", instant.elapsed().as_millis());
  // debug!("Hashmap sizes:\n{}", libmathcat::speech::SpeechRules::print_sizes());
  timing_test(expr, 0);

}

fn timing_test(expr: &str, n_loops: usize) {
  if n_loops == 0 {
    return;
  }
  
  let n_loops_float = n_loops as f64;
  let instant = Instant::now();
  for _ in 0..n_loops {
    if let Err(e) = set_mathml(expr.to_string()) {
      panic!("Error: exiting -- {}", errors_to_string(&e));
    };
    match get_spoken_text() {
      Ok(_) =>( ),
      Err(e) => panic!("{}", errors_to_string(&e)),
    }
    match get_braille("".to_string()) {
      Ok(_) => (),
      Err(e) => panic!("{}", errors_to_string(&e)),
    }
  }
  info!("Time taken (time for set, speech, {} braille averaged over {} loops): {}ms", get_preference("BrailleCode".to_string()).unwrap(), n_loops, instant.elapsed().as_millis() as f64/n_loops_float);

  let instant = Instant::now();
  for _ in 0..n_loops {
    if let Err(e) = set_mathml(expr.to_string()) {
      panic!("Error: exiting -- {}", errors_to_string(&e));
    };
  }
  info!("Time taken (time for set averaged over {} loops): {}ms", n_loops, instant.elapsed().as_millis() as f64/n_loops_float);

  let instant = Instant::now();
  for _ in 0..n_loops {
    match get_spoken_text() {
      Ok(_) =>( ),
      Err(e) => panic!("{}", errors_to_string(&e)),
    }
  }
  info!("Time taken (time for get_spoken_text() averaged over {} loops): {}ms", n_loops, instant.elapsed().as_millis() as f64/n_loops_float);

  set_preference("BrailleCode".to_string(), "UEB".to_string()).unwrap();
  get_braille("".to_string()).unwrap();
  let instant = Instant::now();
  for _ in 0..n_loops {
    match get_braille("".to_string()) {
      Ok(_) => (),
      Err(e) => panic!("{}", errors_to_string(&e)),
    }
  }
  info!("Time taken (time for {} braille averaged over {} loops): {}ms", get_preference("BrailleCode".to_string()).unwrap(), n_loops, instant.elapsed().as_millis() as f64/n_loops_float);

    if let Err(e) = set_mathml(expr.to_string()) {
      panic!("Error: exiting -- {}", errors_to_string(&e));
    };
  set_preference("BrailleCode".to_string(), "Nemeth".to_string()).unwrap();
  get_braille("".to_string()).unwrap();
  let instant = Instant::now();
  for _ in 0..n_loops {
    match get_braille("".to_string()) {
      Ok(_) => (),
      Err(e) => panic!("{}", errors_to_string(&e)),
    }
  }
  info!("Time taken (time for {} braille averaged over {} loops): {}ms", get_preference("BrailleCode".to_string()).unwrap(), n_loops, instant.elapsed().as_millis() as f64/n_loops_float);
}
