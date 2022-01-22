/*

*** see https://rust-cli.github.io/book/tutorial/cli-args.html#parsing-cli-arguments-with-structopt
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
*/

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
  // let expr = "<math><mo>(</mo><mrow><mn>451</mn><mo>,</mo><mn>231</mn></mrow><mo>)</mo></math>";
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
  let expr = "<math xml:lang='en'><mi>x</mi><mo>&lt;</mo><mn>2</mn><mtext>&nbsp;</mtext></math>";
  let instant = Instant::now();
  let rules_dir = std::env::current_exe().unwrap().parent().unwrap().join("..\\..\\..\\Rules");
  let rules_dir = rules_dir.as_os_str().to_str().unwrap().to_string();
  if SetRulesDir(rules_dir.clone()).is_err() {
    panic!("Didn't find Rules dir: {}", rules_dir);
  }
  if let Err(e) = SetMathML(expr.to_string()) {
    panic!("Error: exiting -- {}", e);
  };
  SetPreference("TTS".to_string(), StringOrFloat::AsString("SSML".to_string())).unwrap();
  // SetPreference("Bookmark".to_string(), StringOrFloat::AsString("true".to_string())).unwrap();
  SetPreference("SpeechStyle".to_string(), StringOrFloat::AsString("SimpleSpeak".to_string())).unwrap();

  match GetSpokenText() {
    Ok(speech) => info!("Computed speech string:\n   '{}'", speech),
    Err(e) => panic!("{}", errors_to_string(&e)),
  }
  
  // match GetBraille("".to_string()) {
  //   Ok(braille) => info!("Computed speech string:\n   '{}'", braille),
  //   Err(e) => panic!("{}", get_errors(&e)),
  // }
  info!("Time taken: {}ms", instant.elapsed().as_millis());
  // let instant = Instant::now();
  // match GetSpokenText() {
  //   Ok(speech) => info!("Computed speech string:\n   '{}'", speech),
  //   Err(e) => panic!("{}", get_errors(&e)),
  // }
  
  // match GetBraille("".to_string()) {
  //   Ok(braille) => info!("Computed speech string:\n   '{}'", braille),
  //   Err(e) => panic!("{}", get_errors(&e)),
  // }
  // info!("Time taken (second time): {}ms", instant.elapsed().as_millis());
}
