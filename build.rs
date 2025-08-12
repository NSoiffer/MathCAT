//! The build.rs file is necessary to generate rules.zip.
//! rules.zip are needed so there is a way to get the rules dir into the build since your can't get from the crate.
//! The expectation is that most builds (with the exception of WASM builds) will need a build.rs file to extract the rules.
#![allow(clippy::needless_return)]

use std::fs::{read_dir, DirBuilder, File, remove_file, remove_dir};
use std::io::{self, Read, Seek, Write};
use std::path::{Path, PathBuf};


// use zip::result::ZipResult;
use zip::write::{ZipWriter, SimpleFileOptions};
use zip::CompressionMethod;

/// Zip up all the yaml files in the directories in `rules_dir/sub_dir_name`` and add the zipped file to `archive_zip`.
/// `rules_dir` is the full path to MathCAT's Rules dir (the input), with `sub_dir_name` the (relative) name of the sub dir to look into.
/// `out_dir`` is the full to the build dir's "out" file (where we are allowed to write)
/// For example, if we have the input ".../Rules/Langauges", then we create a zip file for each subdir with yaml files in it.
///    So we get out_dir/Rules/Langauges/en/en.zip, out_dir/Rules/Langauges/es/es.zip, etc.
/// The resulting archive will reproduce the Rules tree but with the yaml files zipped up per/directory.
fn zip_dir(rules_dir: &Path, archive_zip: &mut ZipWriter<File>, options: SimpleFileOptions, out_dir: &Path, current_out_dir: &Path) -> Result<(), std::io::Error> {
    for entry in read_dir(rules_dir)?.flatten() {
        let entry_path = entry.path();

        // .zip files return true for is_dir() -- test in case there is some leftover zip files
        if let Some(suffix) = entry_path.extension() {
            if suffix == "zip" {
                continue;
            }
        }
        // println!("trying dir entry {:?}", entry_path.to_str());
        if entry_path.is_dir(){
            if let Some(dir_name) = entry_path.components().next_back() {
                let dir_name = dir_name.as_os_str().to_str().unwrap();
                if dir_name == "zz" {       // test dir
                    continue;
                }
                let zip_name = dir_name.to_string() + ".zip";       // e.g., en.zip
                // println!("cargo::warning=zip_name='{}'", &zip_name);
                let current_out_dir_zip_path = current_out_dir.join(dir_name);  // e.g., ...out/Rules/Languages/en
                // println!("cargo::warning=current_out_dir_zip_path='{}'", &current_out_dir_zip_path.to_string_lossy());

                // make sure the appropriate directory exists in 'out'
                DirBuilder::new().recursive(true).create(&current_out_dir_zip_path).unwrap();
                let zipped_dir_file_name = current_out_dir_zip_path.join(&zip_name);  // e.g., ...out/Rules/Languages/en/en.zip
                let zip_file = match File::create(&zipped_dir_file_name) {
                    Ok(file) => file,
                    Err(e) => return Err(std::io::Error::other(
                                                    format!("build.rs couldn't create {:?}: {}", &zip_name, e))),
                };
                let mut zip = ZipWriter::new(zip_file);
                let n_files_in_zip = zip_entry(&mut zip, &entry_path, &PathBuf::from("."), options)?;
                zip.finish()?;
                if n_files_in_zip > 0 {
                    // Add the file to full archive with the proper relative path (e.g., Languages/en)
                    let relative_out_dir = current_out_dir_zip_path.strip_prefix(out_dir).unwrap();     // e.g., Rules/Langauges/en
                    add_file_to_zip(archive_zip, &zipped_dir_file_name, &relative_out_dir.join(&zip_name), options)?;
                } else {
                    // delete the .zip file that isn't used -- doesn't really matter, but removes some clutter
                    remove_file(&zipped_dir_file_name)?;
                    remove_dir(&current_out_dir_zip_path)?;
                }
            }
        } else if let Some(suffix) = &entry_path.extension() {
            // definitions.yaml, others???
            let suffix = suffix.to_ascii_lowercase();
            if suffix == "yaml" || suffix == "yml" {
                // make sure the appropriate directory exists in 'out'
                DirBuilder::new().recursive(true).create(current_out_dir).unwrap();
                let entry_name = entry_path.components().next_back().unwrap().as_os_str().to_str().unwrap();
                let out_dir_file_name = current_out_dir.join(entry_name);  // e.g., ...out/Rules/prefs.yaml
                std::fs::copy(&entry_path, &out_dir_file_name)?;
                let relative_out_dir = out_dir_file_name.strip_prefix(out_dir).unwrap();
                add_file_to_zip(archive_zip, relative_out_dir, relative_out_dir, options)?;

            }
        }
    }
    return Ok( () );
}

/// This differs from `zip_dir` in that it does _not_ zip up the directory. Each individual file is zipped so that when the full archive
/// is unzipped, the directory contents do not need to be unzipped.
/// For example, Rules/prefs.yaml and Rules/Intent/general.yaml will exists in the unzipped archive.
fn zip_other_files(rules_dir: &Path, archive_zip: &mut ZipWriter<File>, options: SimpleFileOptions, out_dir: &Path, current_out_dir: &Path) -> Result<(), std::io::Error> {
    for entry in read_dir(rules_dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_name = entry_path.components().next_back().unwrap().as_os_str().to_str().unwrap();

        if entry_path.is_dir(){
            if entry_name == "Intent" {       // handled elsewhere
                zip_other_files(&rules_dir.join("Intent"), archive_zip, options, out_dir, &current_out_dir.join("Intent"))?;
            }
        } else if let Some(suffix) = &entry_path.extension() {
            let suffix = suffix.to_ascii_lowercase();
            if suffix == "yaml" || suffix == "yml" {
                // make sure the appropriate directory exists in 'out'
                DirBuilder::new().recursive(true).create(current_out_dir).unwrap();
                let out_dir_file_name = current_out_dir.join(entry_name);  // e.g., ...out/Rules/prefs.yaml
                std::fs::copy(&entry_path, &out_dir_file_name)?;
                let relative_out_dir = out_dir_file_name.strip_prefix(out_dir).unwrap();
                add_file_to_zip(archive_zip, relative_out_dir, relative_out_dir, options)?;

            }
        }
    }
    return Ok( () );
}

/// Adds the `path` relative to the outdir to the `zip`` archive
fn add_file_to_zip<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    in_dir_path: &Path,
    out_dir_path: &Path,
    options: SimpleFileOptions,
) -> io::Result<()> {
    zip.start_file_from_path(out_dir_path, options)?;
    let mut file = File::open(in_dir_path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    zip.write_all(&buffer)?;
    return Ok(());
}

#[allow(clippy::unused_io_amount)]
fn zip_entry<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    full_path: &Path,
    relative_path: &Path,
    options: SimpleFileOptions
) -> io::Result<usize> {
    let mut n_files_in_zip = 0;
    // println!("zip_entry:\n  full_path: {}\n  relative_path: {}\n  current working dir: {}", full_path.display(), relative_path.display(), std::env::current_dir()?.display());
    if full_path.is_dir() {
        for entry in read_dir(full_path)? {
            let entry_path = entry?.path();
            let entry_name = entry_path.components().next_back().unwrap().as_os_str().to_str().unwrap();
            n_files_in_zip += zip_entry(zip, &entry_path, &relative_path.join(entry_name), options)?;
        }
    } else if let Some(suffix) = full_path.extension() {
        let suffix = suffix.to_ascii_lowercase();
        if suffix == "yaml" || suffix == "yml" {
            add_file_to_zip(zip, full_path, relative_path, options)?;
            n_files_in_zip += 1;
        }
    }

    return Ok(n_files_in_zip);
}

/// Zip up each language and braille dir
/// Note: regional variations (including zh-cn and zh-tw) are zipped together into one zip file
fn main() {
    // This doesn't work because the build claims OUT_DIR is not defined(?)
    // let archive = PathBuf::from(concat!(env!("OUT_DIR"),"/rules.zip"));
    if std::env::var("CARGO_FEATURE_INCLUDE_ZIP").is_ok() {
        let out_dir = std::env::var_os("OUT_DIR").unwrap();
        let out_dir = PathBuf::from(&out_dir);
        let rules_dir = std::env::current_dir().unwrap().join("Rules") ;
        let rules_out_dir = PathBuf::from(&out_dir).join("Rules");

        if  std::env::set_current_dir(&out_dir).is_err() {
            println!("cargo::warning=couldn't change to directory '{}'", &out_dir.display());
            return;
        }
        let archive_path = PathBuf::from("rules.zip");     // A zip file containing all the zip files.
        // println!("cargo::warning=zip file location: '{:?}'", archive_path.to_str());
        let compile_target = std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap(); // build.rs has target_family = build machine, not real target
        let compression_method = if compile_target == "wasm" {
            CompressionMethod::DEFLATE     // although BZIP2 seems better in terms of smaller, it won't compile for WASM
        } else {
            CompressionMethod::BZIP2
        };
        let zip_options = SimpleFileOptions::default().compression_method(compression_method)
                        .compression_level(Some(9));
        // let zip_options = SimpleFileOptions::default()
        //                 .compression_level(Some(9));

        // println!("cargo::warning=rules directory '{:?}'", &rules_dir.to_string_lossy());
        let archive_zip_file = match File::create(&archive_path) {
            Ok(file) => file,
            Err(e) => panic!("build.rs couldn't create {:?}: {}", &archive_path.to_str(), e),
        };

        let mut archive_zip = ZipWriter::new(archive_zip_file);

            
        if let Err(e) = zip_other_files(&rules_dir, &mut archive_zip, zip_options, &out_dir, &rules_out_dir) {
            panic!("Error: {}", e);
        }
        
        if let Err(e) = zip_dir(&rules_dir.join("Languages"), &mut archive_zip, zip_options, &out_dir, &rules_out_dir.join("Languages")) {
            panic!("Error: {}", e);
        }
        if let Err(e) = zip_dir(&rules_dir.join("Braille"), &mut archive_zip, zip_options, &out_dir, &rules_out_dir.join("Braille")) {
            panic!("Error: {}", e);
        }
        if let Err(e) = archive_zip.finish() {
            panic!("Error in zip.finish(): {}", e);
        }
        println!("cargo::rerun-if-changed=Rules");
    }
}