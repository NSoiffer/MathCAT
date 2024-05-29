//! The build.rs file is necessary to generate rules.zip.
//! rules.zip are needed so there is a way to get the rules dir into the build since your can't get from the crate.
//! The expectation is that most builds (with the exception of WASM builds) will need a build.rs file to extract the rules.

use std::fs::{read_dir, DirEntry, File};
use std::io::{self, Read, Seek, Write};
use std::path::Path;
use std::path::PathBuf;

use zip::result::ZipResult;
use zip::write::{FileOptions, SimpleFileOptions};
use zip::CompressionMethod;
use zip::ZipWriter;

fn zip_dir<T: Write + Seek>(
    path: &Path,
    target: T,
    options: SimpleFileOptions
) -> ZipResult<T> {
    let mut zip = ZipWriter::new(target);

    for entry in read_dir(path)? {
        let entry = entry?;
        eprintln!("trying dir entry {:?}", entry.path().to_str());
        zip_entry(&mut zip, entry, options)?;
    }

    zip.finish()
}

#[allow(clippy::unused_io_amount)]
fn zip_entry<T: Write + Seek>(
    zip: &mut ZipWriter<T>,
    entry: DirEntry,
    options: SimpleFileOptions,
) -> io::Result<()> {
    let path = entry.path();

    if path.is_dir() {
        for entry in read_dir(path)? {
            zip_entry(zip, entry?, options)?;
        }
    } else if let Some(suffix) =path.extension() {
        let suffix = suffix.to_ascii_lowercase();
        if suffix == "yaml" || suffix == "yml" {
            let file_name = path.to_str().unwrap();
            zip.start_file(file_name, options)?;

            let mut file = File::open(&path)?;
            let mut buffer = Vec::new();
    
            file.read_to_end(&mut buffer)?;
    
            zip.write_all(&buffer)?;
        }
    }

    Ok(())
}

fn main() {
    // This doesn't work because the build claims OUT_DIR is not defined(?)
    // let archive = PathBuf::from(concat!(env!("OUT_DIR"),"/rules.zip"));

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let archive: PathBuf = [out_dir, std::ffi::OsString::from("rules.zip")].iter().collect();
    eprintln!("zip file location: '{:?}'", archive.to_str());

    let archive = match File::create(&archive) {
        Ok(file) => file,
        Err(e) => panic!("build.rs couldn't create {:?}: {}", archive.to_str(), e),
    };
    // let root_dir = std::env::var_os("CARGO_MANIFEST_DIR ").unwrap(); 
    // let zip_directory: PathBuf = [root_dir.clone(), std::ffi::OsString::from("Rules")].iter().collect();
    // eprintln!("rules dir: '{:?}'", zip_directory.to_str());

    let zip_directory = Path::new("Rules");
    let zip_options = FileOptions::default().compression_method(CompressionMethod::Deflated)
                    .compression_level(Some(9));

    if let Err(e) = zip_dir(zip_directory, archive, zip_options) {
        panic!("Error: {}", e);
    }
    println!("cargo:rerun-if-changed=Rules");
}