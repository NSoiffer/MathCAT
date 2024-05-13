//! The build.rs file is necessary to generate rules.zip.
//! rules.zip are needed so there is a way to get the rules dir into the build since your can't get from the crate.
//! The expectation is that most builds (with the exception of WASM builds) will need a build.rs file to extract the rules.

use std::fs::{read_dir, DirEntry, File};
use std::io::{self, Read, Seek, Write};
use std::path::Path;

// use zip::result::ZipResult;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

fn zip_dir(parent_dir: &Path, options: FileOptions) -> Result<(), std::io::Error> {
    for entry in read_dir(parent_dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        if let Some(suffix) = entry_path.extension() {
            if suffix == "zip" {
                continue;
            }
        }
        println!("trying dir entry {:?}", entry_path.to_str());
        if entry_path.is_dir(){
            if let Some(dir_name) = entry_path.components().last() {
                let zip_name = String::from(dir_name.as_os_str().to_string_lossy() + ".zip");
                let archive_path = entry_path.clone().join(&zip_name).canonicalize()?;
                println!("archive_path: '{}'", archive_path.to_string_lossy());
                let zip_file = match File::create(archive_path) {
                    Ok(file) => file,
                    Err(e) => panic!("build.rs couldn't create {:?}: {}", &zip_name, e),
                };
                println!("  created file");
                let mut zip = ZipWriter::new(zip_file);
                zip_entry(&mut zip, entry, options)?;
                zip.finish()?;
                println!("  after zip.finish()");
            }
        }
    }
    return Ok( () );
}

#[allow(clippy::unused_io_amount)]
fn zip_entry<T: Write + Seek>(
    zip: &mut ZipWriter<T>,
    entry: DirEntry,
    options: FileOptions,
) -> io::Result<()> {
    let path = entry.path();

    if path.is_dir() {
        for entry in read_dir(path)? {
            zip_entry(zip, entry?, options)?;
        }
    } else if let Some(suffix) = path.extension() {
        let suffix = suffix.to_ascii_lowercase();
        if suffix == "yaml" || suffix == "yml" {
            let file_name = path.to_str().unwrap();
            // println!("  adding '{}'", file_name);
            zip.start_file(file_name, options)?;

            let mut file = File::open(&path)?;
            let mut buffer = Vec::new();
    
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            // println!("  ..finished '{}'", file_name);
        }
    }

    Ok(())
}

fn main() {
    // This doesn't work because the build claims OUT_DIR is not defined(?)
    // let archive = PathBuf::from(concat!(env!("OUT_DIR"),"/rules.zip"));

    // let out_dir = std::env::var_os("OUT_DIR").unwrap();
    // let archive: PathBuf = [out_dir, std::ffi::OsString::from("rules.zip")].iter().collect();
    // eprintln!("zip file location: '{:?}'", archive.to_str());

    // let archive = match File::create(&archive) {
    //     Ok(file) => file,
    //     Err(e) => panic!("build.rs couldn't create {:?}: {}", archive.to_str(), e),
    // };
    // let root_dir = std::env::var_os("CARGO_MANIFEST_DIR ").unwrap(); 
    // let zip_directory: PathBuf = [root_dir.clone(), std::ffi::OsString::from("Rules")].iter().collect();
    // eprintln!("rules dir: '{:?}'", zip_directory.to_str());

    let zip_options = FileOptions::default().compression_method(CompressionMethod::Deflated)
                    .compression_level(Some(9));

    let rules_dir = std::env::current_exe().unwrap().parent().unwrap()
                    .join("../../../../Rules/")
                    .to_str().unwrap().to_string();
    
    println!("rules_dir = '{}'", rules_dir);
    if let Err(e) = zip_dir(Path::new(&(rules_dir.clone() + "Languages")), zip_options.clone()) {
        panic!("Error: {}", e);
    }
    if let Err(e) = zip_dir(Path::new(&(rules_dir + "Braille")), zip_options) {
        panic!("Error: {}", e);
    }
    println!("cargo:rerun-if-changed=Rules");
    panic!("Forced panic for debugging build.rs")
}