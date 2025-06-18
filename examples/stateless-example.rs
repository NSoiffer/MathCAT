use env_logger;
use lazy_static::lazy_static;
use libmathcat::{errors::Error, stateless_interface::*};
use std::{ops::Deref, path::Path, sync::mpsc, thread};

const EXPR_1: &'static str = r#"<math><mrow><msup><mi>sin</mi><mn>2</mn></msup><mo>⁡</mo><mspace></mspace><mi>x</mi></mrow></math>"#;
const EXPR_2: &'static str = r#"<math><mrow><msup><mi>cos</mi><mn>2</mn></msup><mo>⁡</mo><mspace></mspace><mi>x</mi></mrow></math>"#;

struct MathCatHolder {
    language: &'static str,
    mathcat: MathCat,
}

// `mathcat` is full of RCs, need to figure out what to do here to explain to Rust
// that it's OK to use a non-mut MathCat across threads because all Rcs are actually
// owned by the `MathCat` instance.
unsafe impl Sync for MathCatHolder {}

fn build_mathcat(language: &'static str) -> Result<MathCatHolder, Error> {
    let rules_dir = std::env::current_exe().unwrap().parent().unwrap().join("../../../Rules");
    let rules_dir = rules_dir.as_os_str().to_str().unwrap().to_string();

    let mut builder = MathCatBuilder::new();
    builder.set_rules_dir(Path::new(&rules_dir));
    builder.set_pref("Language", language);
    Ok(MathCatHolder { language: language, mathcat: builder.build()? })
}

fn main() -> Result<(), Error> {
  // Run with RUST_LOG=debug to see some debugging information.l
  env_logger::builder()
      .format_timestamp(None)
      .format_module_path(false)
      .format_indent(Some(2))
      .format_level(false)
      .init();

    lazy_static! {
        static ref mathcat_en: MathCatHolder = build_mathcat("en").unwrap();
        static ref mathcat_es: MathCatHolder = build_mathcat("es").unwrap();
    }

    // Initialization is not thread-safe, ensure everything is initialized:
    let _ = mathcat_en.deref();
    let _ = mathcat_es.deref();

    // Once initialized, MathCat instances are thread-compatible.
    let (tx, rx) = mpsc::channel();
    let mut threads = Vec::<thread::JoinHandle<Result<(), mpsc::SendError<(&'static str, Result<String, libmathcat::errors::Error>)>>>>::new();
    {
        let tx = tx.clone();
        threads.push(thread::spawn(move || {
            tx.send((
                mathcat_en.language,
                mathcat_en.mathcat.mathml_to_spoken_text(EXPR_1)))
        }));
    }
    {
        let tx = tx.clone();
        threads.push(thread::spawn(move || {
            tx.send((
                mathcat_en.language,
                mathcat_en.mathcat.mathml_to_spoken_text(EXPR_2)))
        }));
    }
    {
        let tx = tx.clone();
        threads.push(thread::spawn(move || {
            tx.send((
                mathcat_es.language,
                mathcat_es.mathcat.mathml_to_spoken_text(EXPR_1)))
        }));
    }
    {
        let tx = tx.clone();
        threads.push(thread::spawn(move || {
            tx.send((
                mathcat_es.language,
                mathcat_es.mathcat.mathml_to_spoken_text(EXPR_2)))
        }));
    }

    let rcv_thread = thread::spawn(move || {
        let mut pending = 4;
        let mut has_errors = false;
        while let Ok((language, result)) = rx.recv() {
            match result {
                Ok(text) => println!("{}: {}", language, text),
                Err(e) => {
                    has_errors = true;
                    println!("{}: Error!\n{:?}", language, e);
                }
            };
            pending -= 1;
            if pending == 0 { break; }
        }
        has_errors
    });

    for thread in threads {
        let _ = thread.join().unwrap();
    }
    let has_errors = rcv_thread.join().unwrap();
    if has_errors { std::process::exit(1); }
    Ok(())
}
