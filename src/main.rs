// *** MathCAT doesn't normally want to build a binary ***
// *** This file is here because it is useful for trying out things ***
#![allow(clippy::needless_return)]

use libmathcat::interface::*;
use log::{debug, info};
use std::time::Instant;
use std::process::exit;


// Maybe also have this speak to test the TTS generation.
// There is a rust winapi crate that mirrors the WinPAI and has "Speak(...)" in it

// env RUST_LOG=DEBUG cargo run --features "include-zip"
cfg_if::cfg_if! {
    if #[cfg(feature = "include-zip")] {
        fn get_rules_dir() -> String {
          return "Rules".to_string();
        }
    } else {
        fn get_rules_dir() -> String {
          // for testing with zipped rules dir
          // let rules_path = std::env::current_exe().unwrap().parent().unwrap().join("../../../MathCATForPython/addon/globalPlugins/MathCAT/Rules");
          let rules_path = std::env::current_exe().unwrap().parent().unwrap().join("../../Rules");
          return rules_path.as_os_str().to_str().unwrap().to_string();
        }
    }
}

fn main() {
  env_logger::builder()
      .format_timestamp(None)
      .format_module_path(false)
      .format_indent(Some(2))
      .format_level(false)
      .init();

   let expr = r#"
 <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mi>A</mi>
  <mo>=</mo>
  <mi>&#x1D70B;</mi>
  <msup>
    <mrow>
      <mi>r</mi>
    </mrow>
    <mn>2</mn>
  </msup>
</math>
"#;
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

//   let expr = r#"
// <math>
//         <msub><mi mathvariant="normal">N</mi><mn>2</mn></msub>
//         <munderover><mo>&#x2192;</mo><mtext>Haber&#xA0;process</mtext><msub><mi mathvariant="normal">H</mi><mn>2</mn></msub></munderover>
//         <mi mathvariant="normal">N</mi>
//         <msub><mi mathvariant="normal">H</mi><mn>3</mn></msub>
//     </math>
//          "#;
  // let instant = Instant::now();

  // let rules_dir = "".to_string();    // Use MathCATRulesDir, potentially pointing to a zipped version
  if let Err(e) = set_rules_dir(get_rules_dir()) {
    eprintln!("Error: exiting -- {}", errors_to_string(&e)); exit(1);
  }
  debug!("Languages: {}", libmathcat::interface::get_supported_languages().unwrap_or_default().join(", "));

  #[cfg(feature = "include-zip")]
  info!("***********include-zip is present**********");
  info!("Version = '{}' using Rules dir {}", get_version(), get_rules_dir());
  set_preference("Language", "en").unwrap();
  set_preference("DecimalSeparator", "Auto").unwrap();
  set_preference("BrailleCode", "LaTeX").unwrap();
  set_preference("BrailleNavHighlight", "On").unwrap();
  set_preference("TTS", "None").unwrap();
  set_preference("Verbosity", "Medium").unwrap();
  set_preference("NavVerbosity", "Verbose").unwrap();
  set_preference("NavMode", "Enhanced").unwrap();
  set_preference("Impairment", "Blindness").unwrap();
  set_preference("SpeechOverrides_CapitalLetters", "").unwrap();
  set_preference("MathRate", "80").unwrap();
  // set_preference("CapitalLetters_UseWord", "true").unwrap();
  // set_preference("CapitalLetters_Pitch", "30").unwrap();
  set_preference("CapitalLetters_Beep", "true").unwrap();
  set_preference("IntentErrorRecovery", "Error").unwrap();
  // set_preference("MathRate", "77").unwrap();

  set_preference("Bookmark", "false").unwrap();
  set_preference("SpeechStyle", "ClearSpeak").unwrap();
  info!("Languages: {}", libmathcat::interface::get_supported_languages().unwrap_or_default().join(", "));
  info!("Speech styles: {}", libmathcat::interface::get_supported_speech_styles("ClearSpeak").unwrap_or_default().join(", "));
  info!("BrailleCodes: {}", libmathcat::interface::get_supported_braille_codes().unwrap_or_default().join(", "));
  // set_preference("DecimalSeparators", ",").unwrap();
  // set_preference("BlockSeparators", ". ").unwrap();
  if let Err(e) = set_mathml(expr) {
    eprintln!("Error: exiting -- {}", errors_to_string(&e)); exit(1); 
  };

  match do_navigate_command("ZoomIn")  {
    Err(e) => {eprintln!("Error: exiting -- {}", errors_to_string(&e)); exit(1);},
    Ok(speech) => info!("\nZoomIn speech: '{speech}'"),
  }
  match do_navigate_command("ZoomIn")  {
    Err(e) => {eprintln!("Error: exiting -- {}", errors_to_string(&e)); exit(1);},
    Ok(speech) => info!("\nZoomIn speech: '{speech}'"),
  }
  info!("\n");
  // match do_navigate_command("ToggleZoomLockUp") {
  //   Err(e) => eprintln!("Error: exiting -- {}", errors_to_string(&e)); exit(1);,
  //   Ok(speech) => info!("ToggleZoomLockUp speech: '{speech}'"),
  // }
  // match do_navigate_command("MovePrevious") {
  //   Err(e) => eprintln!("Error: exiting -- {}", errors_to_string(&e)); exit(1);,
  //   Ok(speech) => info!("MovePrevious speech: '{speech}'"),
  // }
  // match do_navigate_command("MovePrevious") {
  //   Err(e) => eprintln!("Error: exiting -- {}", errors_to_string(&e)); exit(1);,
  //   Ok(speech) => info!("MovePrevious speech: '{}'", speech),
  // }
  // match do_navigate_command("MovePrevious") {
  //   Err(e) => eprintln!("Error: exiting -- {}", errors_to_string(&e)); exit(1);,
  //   Ok(speech) => info!("MovePrevious speech: '{}'", speech),
  // }
  // match do_navigate_command("MoveNext") {
  //   Err(e) => eprintln!("Error: exiting -- {}", errors_to_string(&e)); exit(1);,
  //   Ok(speech) => info!("MoveNext speech: '{}'", speech),
  // }
  // match do_navigate_command("MoveNext") {
  //   Err(e) => eprintln!("Error: exiting -- {}", errors_to_string(&e)); exit(1);,
  //   Ok(speech) => info!("MoveNext speech: '{}'", speech),
  // }
  // match do_navigate_command("MoveNext") {
  //   Err(e) => eprintln!("Error: exiting -- {}", errors_to_string(&e)); exit(1);,
  //   Ok(speech) => info!("MoveNext speech: '{}'", speech),
  // }
  // match do_navigate_command("MoveNext") {
  //   Err(e) => eprintln!("Error: exiting -- {}", errors_to_string(&e)); exit(1);,
  //   Ok(speech) => info!("MoveNext speech: '{}'", speech),
  // }
  // match get_spoken_text() {
  //   Ok(speech) => info!("Computed speech string:\n   '{speech}'"),
  //   Err(e) => eprintln!("{}", errors_to_string(&e)); exit(1);,
  // }
  debug!("Speech language is {}", get_preference("Language").unwrap());
  debug!("DecimalSeparator: {:?}", get_preference("DecimalSeparator").unwrap());
  debug!("DecimalSeparators: {:?}, BlockSeparators: {:?}", get_preference("DecimalSeparators").unwrap(), get_preference("BlockSeparators").unwrap());
  debug!("SpeechStyle: {:?}", get_preference("SpeechStyle").unwrap());
  debug!("Verbosity: {:?}", get_preference("Verbosity").unwrap());
 
  // info!("Time taken for loading+speech+braille: {}ms", instant.elapsed().as_millis());
  // let instant = Instant::now();
  match get_spoken_text() {
    Ok(speech) => info!("Computed speech string:\n   '{}'", speech),
    Err(e) => {eprintln!("{}", errors_to_string(&e)); exit(1);},
  }
  // info!("Time taken (second time for speech): {}ms", instant.elapsed().as_millis());
  // info!("SpeechStyle: {:?}", get_preference("SpeechStyle"));

  match get_braille("") {
    Ok(braille) => info!("Computed braille string:\n   '{braille}'"),
    Err(e) => {eprintln!("{}", errors_to_string(&e)); exit(1);},
  }
  debug!("...using BrailleCode: {:?}", get_preference("BrailleCode").unwrap());
  // let xpath_counts = libmathcat::speech::xpath_count();
  // info!("#xpath = {}; duplicates = {}", xpath_counts.0, xpath_counts.1);
  // info!("Time taken (second time for speech + braille): {}ms", instant.elapsed().as_millis());
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
    if let Err(e) = set_mathml(expr) {
      eprintln!("Error: exiting -- {}", errors_to_string(&e));
    };
    match get_spoken_text() {
      Ok(_) =>( ),
      Err(e) => {eprintln!("{}", errors_to_string(&e)); exit(1);},
    }
    // match get_braille("") {
    //   Ok(_) => (),
    //   Err(e) => {eprintln!("{}", errors_to_string(&e)); exit(1);},
    // }
  }
  info!("Time taken (time for set, speech, {} braille averaged over {} loops): {}ms", get_preference("BrailleCode").unwrap(), n_loops, instant.elapsed().as_millis() as f64/n_loops_float);

  // let instant = Instant::now();
  // for _ in 0..n_loops {
  //   if let Err(e) = set_mathml(expr) {
  //     eprintln!("Error: exiting -- {}", errors_to_string(&e));
  //   };
  // }
  // info!("Time taken (time for set averaged over {} loops): {}ms", n_loops, instant.elapsed().as_millis() as f64/n_loops_float);

  // let instant = Instant::now();
  // for _ in 0..n_loops {
  //   match get_spoken_text() {
  //     Ok(_) =>( ),
  //     Err(e) => {eprintln!("{}", errors_to_string(&e)); exit(1);},
  //   }
  // }
  // info!("Time taken (time for get_spoken_text() averaged over {} loops): {}ms", n_loops, instant.elapsed().as_millis() as f64/n_loops_float);

  // set_preference("BrailleCode", "UEB").unwrap();
  // get_braille("").unwrap();
  // let instant = Instant::now();
  // for _ in 0..n_loops {
  //   match get_braille("") {
  //     Ok(_) => (),
  //     Err(e) => {eprintln!("{}", errors_to_string(&e)); exit(1);},
  //   }
  // }
  // info!("Time taken (time for {} braille averaged over {} loops): {}ms", get_preference("BrailleCode").unwrap(), n_loops, instant.elapsed().as_millis() as f64/n_loops_float);

  //   if let Err(e) = set_mathml(expr) {
  //     eprintln!("Error: exiting -- {}", errors_to_string(&e));
  //   };
  // set_preference("BrailleCode", "Nemeth").unwrap();
  // get_braille("").unwrap();
  // let instant = Instant::now();
  // for _ in 0..n_loops {
  //   match get_braille("") {
  //     Ok(_) => (),
  //     Err(e) => {eprintln!("{}", errors_to_string(&e)); exit(1);} ,
  //   }
  // }
  // info!("Time taken (time for {} braille averaged over {} loops): {}ms", get_preference("BrailleCode").unwrap(), n_loops, instant.elapsed().as_millis() as f64/n_loops_float);
}
