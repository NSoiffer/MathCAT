// *** MathCAT doesn't normally want to build a binary ***
// *** This file is here because it is useful for trying out things ***


// const LOG_FILE: &'static str = "./MathCAT.log";
// lazy_static! {
//   static ref LOG: slog::Logger = create_log();
// }

// fn create_log() -> slog::Logger {
  // let file = OpenOptions::new()
  //   .create(true)
  //   .write(true)
  //   .truncate(true)
  //   .open(LOG_FILE)
  //   .unwrap();

  // let decorator = slog_term::PlainDecorator::new(file);
  // let drain = slog_term::FullFormat::new(decorator).build().fuse();
  // let drain = slog_async::Async::new(drain).build().fuse();

  // let logger = slog::Logger::root(drain, o!());
  // return logger;
// }

// Maybe also have this speak to test the TTS generation.
// There is a rust winapi crate that mirrors the WinPAI and has "Speak(...)" in it
fn main() {
  use libmathcat::interface::*;
  use log::*;
  use std::time::{Instant};
  env_logger::builder()
      .format_timestamp(None)
      .format_module_path(false)
      .format_indent(None)
      .format_level(false)
      .init();

  //  let expr = "
  //     <math display='block' xmlns='http://www.w3.org/1998/Math/MathML'>
  //     <mrow>
  //      <mrow><mo>[</mo>
  //        <mtable>
  //         <mtr>
  //          <mtd>
  //           <mn>3</mn>
  //          </mtd>
  //          <mtd>
  //           <mn>1</mn>
  //          </mtd>
  //          <mtd>
  //           <mn>4</mn>
  //          </mtd>
  //         </mtr>
  //         <mtr>
  //          <mtd>
  //           <mn>0</mn>
  //          </mtd>
  //          <mtd>
  //           <mn>2</mn>
  //          </mtd>
  //          <mtd>
  //           <mn>6</mn>
  //          </mtd>
  //         </mtr>
  //        </mtable>
  //      <mo>]</mo></mrow></mrow>
  //    </math>
  // ";

  // let expr = "
  //     <math><mover><mrow><mi>A</mi><mi>B</mi></mrow><mo>⌒</mo></mover></math>
  //   ";
  // let expr = "
  // <math><mo>&#x25B3;</mo><mi>ABC</mi></math>
  //   ";
  // let expr = "<math><mi>c</mi><mo>=</mo><mn>4</mn><mspace width=\"thinmathspace\"></mspace><mn>598</mn>
  //                 <mspace width=\"thinmathspace\"></mspace><mn>037</mn>
  //                 <mspace width=\"thinmathspace\"></mspace><mn>234</mn></math>";
  // let expr = "<math><mn>𝟏𝟐𝟑</mn></math>";
  let expr = "<math display='inline' xmlns='http://www.w3.org/1998/Math/MathML'>
      <mo>(</mo><mn>3</mn><mo>,</mo><mn>12</mn><mo>)</mo>
      </math>";
  // let expr = "
  // <math display='block' id='x0'> <semantics>
  //  <mrow displaystyle='true' id='x1'>
  //    <mn id='x2'>1<span> foo </span>9</mn>
  //    <mo id='x3'>+</mo>
  //    <mn id='x4'>2</mn>
  //  </mrow>
  //  </semantics></math>";
//   let expr = "
  //   <math xmlns='http://www.w3.org/1998/Math/MathML'  alttext='{\\displaystyle ax^{2}+bx+c=a(x-r)(x-s)=0}'>
  //   <semantics>
  //     <mrow class='MJX-TeXAtom-ORD'>
  //       <mstyle displaystyle='true' scriptlevel='0'>
  //         <mi>a</mi>
  //         <msup>
  //           <mi>x</mi>
  //           <mrow class='MJX-TeXAtom-ORD'>
  //             <mn>2</mn>
  //           </mrow>
  //         </msup>
  //         <mo>+</mo>
  //         <mi>b</mi>
  //         <mi>x</mi>
  //         <mo>+</mo>
  //         <mi>c</mi>
  //         <mo>=</mo>
  //         <mi>a</mi>
  //         <mo stretchy='false'>(</mo>
  //         <mi>x</mi>
  //         <mo>&#x2212;<!-- − --></mo>
  //         <mi>r</mi>
  //         <mo stretchy='false'>)</mo>
  //         <mo stretchy='false'>(</mo>
  //         <mi>x</mi>
  //         <mo>&#x2212;<!-- − --></mo>
  //         <mi>s</mi>
  //         <mo stretchy='false'>)</mo>
  //         <mo>=</mo>
  //         <mn>0</mn>
  //       </mstyle>
  //     </mrow>
  //     <annotation encoding='application/x-tex'>{\\displaystyle ax^{2}+bx+c=a(x-r)(x-s)=0}</annotation>
  //   </semantics>
  // </math>  ";
  // let expr = "
  // <math display='block'><mrow><mrow><mrow><mover accent='true'><mo fence='true' stretchy='false'>∥</mo><mo stretchy='false'>^</mo></mover><mo>⁢</mo><mi>f</mi><mo>⁢</mo><msub><mover accent='true'><mo fence='true' stretchy='false'>∥</mo><mo stretchy='false'>^</mo></mover><mi>p</mi></msub></mrow><mo>=</mo><msup><mrow><mo>(</mo><mrow><munder><mo largeop='true' movablelimits='false' symmetric='true'>∑</mo><mrow><mi>γ</mi><mo>∈</mo><mover accent='true'><msubsup><mi>𝔽</mi><mn>𝟚</mn><mi>𝕟</mi></msubsup><mo>^</mo></mover></mrow></munder><msup><mrow><mo stretchy='false'>|</mo><mrow><mover accent='true'><mi>f</mi><mo>^</mo></mover><mo>⁢</mo><mrow><mo stretchy='false'>(</mo><mi>γ</mi><mo stretchy='false'>)</mo></mrow></mrow><mo stretchy='false'>|</mo></mrow><mi>p</mi></msup></mrow><mo>)</mo></mrow><mrow><mn>1</mn><mo>/</mo><mi>p</mi></mrow></msup></mrow><mo>.</mo></mrow></math>
  //   ";
  // let expr = "<math><mn>1750</mn>
  //     <mo>&#xA0;</mo><mi mathvariant='normal' class='MathML-Unit'>cm</mi><mo>=</mo>
  //     <mo>&#xA0;</mo><mn>1</mn><mn>&#xBE;</mn>
  //     </math>";
  let instant = Instant::now();
  let rules_dir = std::env::current_exe().unwrap().parent().unwrap().join("../../../Rules");
  let rules_dir = rules_dir.as_os_str().to_str().unwrap().to_string();
  if let Err(e) = set_rules_dir(rules_dir.clone()) {
    panic!("Error: exiting -- {}", errors_to_string(&e));  }
  if let Err(e) = set_mathml(expr.to_string()) {
    panic!("Error: exiting -- {}", errors_to_string(&e));
  };
  set_preference("TTS".to_string(), "SSML".to_string()).unwrap();
  // set_preference("Bookmark".to_string(), "true".to_string()).unwrap();
  // set_preference("SpeechStyle".to_string(), "SimpleSpeak".to_string()).unwrap();

  match get_spoken_text() {
    Ok(speech) => info!("Computed speech string:\n   '{}'", speech),
    Err(e) => panic!("{}", errors_to_string(&e)),
  }
  info!("SpeechStyle: {:?}", get_preference("SpeechStyle".to_string()));
 
  // set_preference("BrailleCode".to_string(), "Nemeth".to_string()).unwrap();
  // match get_braille("".to_string()) {
  //   Ok(braille) => info!("Computed braille string:\n   '{}'", braille),
  //   Err(e) => panic!("{}", errors_to_string(&e)),
  // }

  // Note: the logger seems to be a huge time sync, so println! is used for timing
  info!("Time taken: {}ms", instant.elapsed().as_millis());
  let instant = Instant::now();
  match get_spoken_text() {
    Ok(speech) => info!("Computed speech string:\n   '{}'", speech),
    Err(e) => panic!("{}", errors_to_string(&e)),
  }
  info!("SpeechStyle: {:?}", get_preference("SpeechStyle".to_string()));
  
  // match get_braille("".to_string()) {
  //   Ok(braille) => info!("Computed braille string:\n   '{}'", braille),
  //   Err(e) => panic!("{}", errors_to_string(&e)),
  // }
  info!("Time taken (second time): {}ms", instant.elapsed().as_millis());
}
