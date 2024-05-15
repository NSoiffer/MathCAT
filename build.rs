//! The build.rs file is necessary to generate rules.zip.
//! rules.zip are needed so there is a way to get the rules dir into the build since your can't get from the crate.
//! The expectation is that most builds (with the exception of WASM builds) will need a build.rs file to extract the rules.

use std::fs::{read_dir, File};
use std::io::{self, Read, Seek, Write};
use std::path::{Path, PathBuf};

// use zip::result::ZipResult;
use zip::write::{ZipWriter, SimpleFileOptions};
use zip::CompressionMethod;

fn zip_dir(parent_dir: &Path, options: SimpleFileOptions) -> Result<(), std::io::Error> {
    for entry in read_dir(parent_dir)? {
        let entry = entry?;
        let entry_path = entry.path();

        // .zip files return true for is_dir()
        if let Some(suffix) = entry_path.extension() {
            if suffix == "zip" {
                continue;
            }
        }
        // println!("trying dir entry {:?}", entry_path.to_str());
        if entry_path.is_dir(){
            if let Some(dir_name) = entry_path.components().last() {
                let dir_name = dir_name.as_os_str().to_str().unwrap();
                if  std::env::set_current_dir(&entry_path).is_err() {
                    println!("cargo::warning=couldn't change to directory '{}'", &entry_path.display());
                    continue;
                }
                let zip_name = dir_name.to_string() + ".zip";
                // println!("zip_name='{}')", &zip_name);
                let archive_path = PathBuf::from(&zip_name);
                // println!("archive_path: '{}'", archive_path.to_string_lossy());
                let zip_file = match File::create(archive_path) {
                    Ok(file) => file,
                    Err(e) => panic!("build.rs couldn't create {:?}: {}", &zip_name, e),
                };
                let mut zip = ZipWriter::new(zip_file);
                zip_entry(&mut zip, &PathBuf::from("."), options)?;
                zip.finish()?;
            }
        }
    }
    return Ok( () );
}

#[allow(clippy::unused_io_amount)]
fn zip_entry<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    path: &Path,
    options: SimpleFileOptions,
) -> io::Result<()> {

    // println!("current working dir: {}", std::env::current_dir()?.display());
    if path.is_dir() {
        for entry in read_dir(path)? {
            zip_entry(zip, &entry?.path(), options)?;
        }
    } else if let Some(suffix) = path.extension() {
        let suffix = suffix.to_ascii_lowercase();
        if suffix == "yaml" || suffix == "yml" {
            // println!("  adding '{}'", path.to_str().unwrap());
            zip.start_file_from_path(path, options)?;

            let mut file = File::open(&path)?;
            let mut buffer = Vec::new();
    
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            // println!("  ..finished '{}'", path.to_str().unwrap());
        }
    }

    Ok(())
}

/// Zip up each language and braille dir
/// Note: regional variations (including zh-cn and zh-tw) are zipped together into one zip file
fn main() {
    // This doesn't work because the build claims OUT_DIR is not defined(?)
    // let archive = PathBuf::from(concat!(env!("OUT_DIR"),"/rules.zip"));

    // let out_dir = std::env::var_os("OUT_DIR").unwrap();
    // let archive: PathBuf = [out_dir, std::ffi::OsString::from("rules.zip")].iter().collect();
    // eprintln!("zip file location: '{:?}'", archive.to_str());

    let zip_options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated)
                    .compression_level(Some(9));

    let rules_dir = match std::env::current_exe().unwrap().parent().unwrap()
                            .join("../../../../Rules/")
                            .canonicalize() {
                        Ok(path) => path,
                        Err(err) => panic!("Error canonicalizing path: {}", err),
                    };
    if let Err(e) = zip_dir(&rules_dir.join("Languages"), zip_options.clone()) {
        panic!("Error: {}", e);
    }
    if let Err(e) = zip_dir(&rules_dir.join("Braille"), zip_options) {
        panic!("Error: {}", e);
    }
    println!("cargo::rerun-if-changed=Rules");
}