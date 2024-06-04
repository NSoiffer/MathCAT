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
fn zip_dir(rules_dir: &Path, sub_dir_name: &str, archive_zip: &mut ZipWriter<File>, options: SimpleFileOptions, out_dir: &Path) -> Result<(), std::io::Error> {
    // There are a number of paths to keep track of so that the resulting zip archive reproduces the appropriate paths
    let relative_rules_sub_dir = PathBuf::from("Rules").join(sub_dir_name);
    let rules_sub_dir = rules_dir.join(sub_dir_name);
    println!("using out_dir {:?}", out_dir.to_str());
    for entry in read_dir(rules_sub_dir)? {
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
                if dir_name == "zz" {       // test dir
                    continue;
                }
                if  std::env::set_current_dir(&entry_path).is_err() {
                    println!("cargo::warning=couldn't change to directory '{}'", &entry_path.display());
                    continue;
                }
                let zip_name = dir_name.to_string() + ".zip";       // e.g., en.zip
                // println!("zip_name='{}'", &zip_name);
                let out_dir_zip_path = out_dir.join(&relative_rules_sub_dir).join(dir_name);  // e.g., ...out/Rules/Languages/en
                // println!("out_dir_zip_path='{}'", &out_dir_zip_path.to_string_lossy());

                // make sure the appropriate directory exists in 'out'
                DirBuilder::new().recursive(true).create(&out_dir_zip_path).unwrap();
                let out_dir_zip_path_file_name = out_dir_zip_path.join(&zip_name);  // e.g., ...out/Rules/Languages/en/en.zip
                let zip_file = match File::create(&out_dir_zip_path_file_name) {
                    Ok(file) => file,
                    Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other,
                                                             format!("build.rs couldn't create {:?}: {}", &zip_name, e))),
                };
                // println!("About to write temp zip file to ='{}'", &out_dir_zip_path_file_name.to_string_lossy());
                let mut zip = ZipWriter::new(zip_file);
                let n_files_in_zip = zip_entry(&mut zip, &PathBuf::from("."), options)?;
                zip.finish()?;
                if n_files_in_zip > 0 {
                    // Add the file to full archive with the proper relative path (e.g., Languages/en)
                    std::env::set_current_dir(&out_dir)?;
                    add_file_to_zip(archive_zip, &relative_rules_sub_dir.join(dir_name).join(zip_name), options)?; // e.g., Rules/Langauges/en/en.zip
                } else {
                    // delete the .zip file that isn't used -- doesn't really matter, but removes some clutter
                    remove_file(&out_dir_zip_path_file_name)?;
                    remove_dir(&out_dir_zip_path)?;
                }
            }
        }
    }
    return Ok( () );
}

/// This differs from `zip_dir` in that it does _not_ zip up the directory. Each individual file is zipped so that when the full archive
/// is unzipped, the directory contents do not need to be unzipped.
/// For example, Rules/prefs.yaml and Rules/Intent/general.yaml will exists in the unzipped archive.
fn zip_other_files(rules_dir: &Path, archive_zip: &mut ZipWriter<File>, options: SimpleFileOptions, out_dir: &Path, current_out_dir: &Path) -> Result<(), std::io::Error> {
    // FIX: this shares a lot in common with `zip_dir` -- can they be merged without creating something much more complicated?
    // this is used when zipping -- files are added to the zip file relative to this
    if  std::env::set_current_dir(&out_dir).is_err() {
        println!("cargo::warning=couldn't change to directory '{}'", &out_dir.display());
        return Err(std::io::Error::new(std::io::ErrorKind::Other,
            format!("couldn't change to directory '{}'", &out_dir.display())));
    }
    for entry in read_dir(rules_dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_name = entry_path.components().last().unwrap().as_os_str().to_str().unwrap();

        // println!("trying dir entry {:?}", entry_path.to_str());
        if entry_path.is_dir(){
            if entry_name == "Intent" {       // handled elsewhere
                zip_other_files(&rules_dir.join("Intent"), archive_zip, options, out_dir, &current_out_dir.join("Intent"))?;
            }
        } else if let Some(suffix) = &entry_path.extension() {
            let suffix = suffix.to_ascii_lowercase();
            if suffix == "yaml" || suffix == "yml" {
                // make sure the appropriate directory exists in 'out'
                DirBuilder::new().recursive(true).create(&current_out_dir).unwrap();
                let out_dir_file_name = current_out_dir.join(entry_name);  // e.g., ...out/Rules/prefs.yaml
                // println!("cargo::warning=out_dir_file_name '{}'", &out_dir_file_name.display());
                std::fs::copy(&entry_path, &out_dir_file_name)?;
                let relative_out_dir = out_dir_file_name.strip_prefix(out_dir).unwrap();
                add_file_to_zip(archive_zip, relative_out_dir, options)?;

            }
        }
    }
    return Ok( () );
}


fn add_file_to_zip<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    path: &Path,
    options: SimpleFileOptions,
) -> io::Result<()> {
    // println!("cargo::warning=add_file_to_zip: file='{}'", path.to_str().unwrap());

    zip.start_file_from_path(path, options)?;

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    zip.write_all(&buffer)?;
    // println!("  ..finished '{}'", path.to_str().unwrap());
    return Ok(());
}

#[allow(clippy::unused_io_amount)]
fn zip_entry<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    path: &Path,
    options: SimpleFileOptions
) -> io::Result<usize> {
    let mut n_files_in_zip = 0;
    println!("current working dir: {}", std::env::current_dir()?.display());
    if path.is_dir() {
        for entry in read_dir(path)? {
            n_files_in_zip += zip_entry(zip, &entry?.path(), options)?;
        }
    } else if let Some(suffix) = path.extension() {
        let suffix = suffix.to_ascii_lowercase();
        if suffix == "yaml" || suffix == "yml" {
            add_file_to_zip(zip, path, options)?;
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

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(&out_dir);
    let rules_out_dir = PathBuf::from(&out_dir).join("Rules");

    let archive_path = PathBuf::from(&out_dir).join("rules.zip");     // A zip file containing all the zip files.
    println!("zip file location: '{:?}'", archive_path.to_str());

    let zip_options = SimpleFileOptions::default().compression_method(CompressionMethod::BZIP2)
                    .compression_level(Some(9));

    let rules_dir = std::env::current_dir().unwrap().join("Rules") ;
    println!("rules directory '{:?}'", &rules_dir.to_string_lossy());
    let archive_zip_file = match File::create(&archive_path) {
        Ok(file) => file,
        Err(e) => panic!("build.rs couldn't create {:?}: {}", &archive_path.to_str(), e),
    };

    let mut archive_zip = ZipWriter::new(archive_zip_file);

        
    if let Err(e) = zip_other_files(&rules_dir, &mut archive_zip, zip_options, &out_dir, &rules_out_dir) {
        panic!("Error: {}", e);
    }
    
    if let Err(e) = zip_dir(&rules_dir.join("Languages"), &mut archive_zip, zip_options, &out_dir, &rules_out_dir) {
        panic!("Error: {}", e);
    }
    if let Err(e) = zip_dir(&rules_dir.join("Braille"), &mut archive_zip, zip_options, &out_dir, &rules_out_dir) {
        panic!("Error: {}", e);
    }
    if let Err(e) = archive_zip.finish() {
        panic!("Error in zip.finish(): {}", e);
    }
    println!("cargo::rerun-if-changed=Rules");
}