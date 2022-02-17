//! The build.rs file is necessary to generate rules.zip.
//! rules.zip are needed so there is a way to get the rules dir into the build since your can't get from the crate.
//! The expectation is that most builds (with the exception of WASM builds) will need a build.rs file to extract the rules.

use std::fs::{read_dir, DirEntry, File};
use std::io::{self, Read, Seek, Write};
use std::path::Path;

use zip::result::ZipResult;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

pub fn zip_dir<T: Write + Seek>(
    path: &Path,
    target: T,
    options: FileOptions,
) -> ZipResult<T> {
    let mut zip = ZipWriter::new(target);

    for entry in read_dir(path)? {
        zip_entry(&mut zip, entry?, options)?;
    }

    zip.finish()
}

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
    } else if let Some(suffix) =path.extension() {
        let suffix = suffix.to_ascii_lowercase();
        if suffix == "yaml" || suffix == "yml" {
            zip.start_file(path.to_str().unwrap(), options)?;

            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
    
            file.read_to_end(&mut buffer)?;
    
            zip.write(&buffer)?;
        }
    }

    Ok(())
}

fn main() {
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    let archive = match File::create("rules.zip") {
        Ok(file) => file,
        Err(e) => panic!("build.rs couldn't create rules.zip: {}", e),
    };
    let zip_directory = Path::new("Rules");
    let zip_options = FileOptions::default().compression_method(CompressionMethod::Bzip2);

    if let Err(e) = zip_dir(zip_directory, archive, zip_options) {
        panic!("Error: {}", e);
    }
    println!("cargo:rerun-if-changed=Rules");
}