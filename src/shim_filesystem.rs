#![allow(clippy::needless_return)]
//! This is used to paste over normal reading of the Rules files and building them into the code for web assembly (WASM) which
//! can't do file system access. For the latter, the Rules directory should be zipped up.
//! 
//! Note: if files are added or removed, the directory structure needs to be reflected here. This could be automated,
//! but changes are pretty rare and it didn't seem worth it (this may need to be revisited).

use std::path::{Path, PathBuf};
use crate::errors::*;


// The zipped files are needed by WASM builds.
// However, they are also useful for other builds because there really isn't another good way to get at the rules.
// Other build scripts can extract these files and unzip to their needed locations.
// I'm not thrilled with this solution as it seems hacky, but I don't know another way for crates to allow for each access to data.
#[cfg(feature = "include-zip")]
pub static ZIPPED_RULE_FILES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"),"/rules.zip"));


cfg_if! {
    if #[cfg(any(target_family = "wasm", feature = "include-zip"))] {
        // For the WASM build, we build a fake file system based on ZIPPED_RULE_FILES.
        // That stream encodes other zip files that must be unzipped.

        // We have a problem in that ZIPPED_RULE_FILES has a static lifetime but the contained zip files, when unzipped, are on the stack with a different lifetime.
        // One solution would be to introduce an enum that forks between the two.
        // The slightly hacky but slightly less code solution that is adopted is to use Option<>, with None representing the static case
        // Note: Rc is used because there are borrowing/lifetime issues without being able to clone the data that goes into the HashMap
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::io::Cursor;
        use std::io::Read;
        use std::collections::{HashSet, HashMap};

        #[derive(Debug)]
        struct FilesEntry {
            data: Rc<Option<Vec<u8>>>,
            index: usize,
        }
        thread_local! {
            // mapping the file names to whether they are are directory or a file (if a file, where to find it in the zip archive)
            static DIRECTORIES: RefCell<HashSet<String>> = RefCell::new(HashSet::with_capacity(63));
            static FILES: RefCell<HashMap< String, FilesEntry >> = RefCell::new(HashMap::with_capacity(1023));
        }
        
        fn read_zip_file(containing_dir: &Path, zip_file: Option<Vec<u8>>) -> Result<()> {
            // Return "file" or "dir" if a match, otherwise None
            let zip_file = Rc::new(zip_file);
            FILES.with(|files| {
                let mut files = files.borrow_mut();
                DIRECTORIES.with(|dirs| {
                    let mut dirs = dirs.borrow_mut();
                    let mut archive = match zip_file.as_ref() {
                        None => {
                            let buf_reader = Cursor::new(ZIPPED_RULE_FILES);
                            zip::ZipArchive::new(buf_reader).unwrap()
                        },
                        Some(zip_file) => {
                            let buf_reader = Cursor::new((zip_file).as_ref());
                            match zip::ZipArchive::new(buf_reader) {
                                Err(e) => bail!("read_zip_file: failed to create ZipArchive in dir {}: {}", containing_dir.display(), e),
                                Ok(archive) => archive,
                            }
                        }
                    };
                    for i in 0..archive.len() {
                        let file = archive.by_index(i).unwrap();
                        // A little bit of safety/sanity checking
                        let path = match file.enclosed_name() {
                            Some(path) => containing_dir.to_path_buf().join(path),
                            None => {
                                bail!("Entry {} has a suspicious path (outside of archive)", file.name());
                            }
                        };
                        // debug!("read_zip_file: file path='{}'", path.display());
                        // add all the dirs up to the containing dir -- skip the first one as that is a file
                        // for files like unicode.yaml, this loop is a no-op, but for files in the Shared folder, it will go one time.
                        for parent in path.ancestors().skip(1) {
                            if parent == containing_dir {
                                break;
                            }
                            dirs.insert(parent.to_str().unwrap_or_default().replace("/", std::path::MAIN_SEPARATOR_STR));
                        }
                        if file.is_file() {
                            files.insert(path.to_str().unwrap_or_default().replace("/", std::path::MAIN_SEPARATOR_STR), FilesEntry{ data: zip_file.clone(), index: i});
                        } else if file.is_dir() {
                            dirs.insert(path.to_str().unwrap_or_default().replace("/", std::path::MAIN_SEPARATOR_STR));
                        } else {
                            bail!("read_zip_file: {} is neither a file nor a directory", path.display());
                        }
                    };
                    // debug!("files={:?}", files.keys());
                    // debug!("dirs={:?}", dirs);
                    return Ok( () );
                })
            })
        }
        
        pub fn is_file_shim(path: &Path) -> bool {
            if FILES.with(|files| files.borrow().is_empty()) {
                let empty_path = PathBuf::new();
                read_zip_file(&empty_path, None).unwrap_or(());
            }
            return FILES.with(|files| files.borrow().contains_key(path.to_str().unwrap_or_default()) );
        }
        
        pub fn is_dir_shim(path: &Path) -> bool {
            if FILES.with(|files| files.borrow().is_empty()) {
                let empty_path = PathBuf::new();
                read_zip_file(&empty_path, None).unwrap_or(());
            }
            return DIRECTORIES.with(|dirs| dirs.borrow().contains(path.to_str().unwrap_or_default()) );
        }

        pub fn find_files_in_dir_that_ends_with_shim(dir: &Path, ending: &str) -> Vec<String> {
            // FIX: this is very inefficient -- maybe gather up all the info in read_zip_file()?
            // look for files that have 'path' as a prefix
            return FILES.with(|files| {
                let files = files.borrow();
                let mut answer = Vec::new();

                let dir_name = dir.to_str().unwrap_or_default();
                for file_name in files.keys() {
                    if let Some(dir_relative_name) = file_name.strip_prefix(dir_name) {
                        if file_name.ends_with(ending) {
                            // this could be (e.g.) xxx_Rules.yaml or it could be subdir/xxx_Rules.yaml
                            let file_name = dir_relative_name.split_once(std::path::MAIN_SEPARATOR).map(|(_, after)| after).unwrap_or(dir_relative_name);
                            answer.push( file_name.to_string() );
                        }
                    }
                }
                return answer;
            });
        }
        
        pub fn find_all_dirs_shim(dir: &Path, found_dirs: &mut Vec<PathBuf> ) {
            return DIRECTORIES.with(|dirs| {
                let dirs = dirs.borrow();

                let common_dir_name = dir.to_str().unwrap_or_default();
                for dir_name in dirs.iter() {
                    if dir_name.starts_with(common_dir_name) && !dir_name.contains("SharedRules") {
                        found_dirs.push(PathBuf::from(dir_name));
                    };
                }
            });
        }

        
        pub fn canonicalize_shim(path: &Path) -> std::io::Result<PathBuf> {
            use std::ffi::OsStr;
            let dot_dot = OsStr::new("..");
            let mut result = PathBuf::new();
            for part in path.iter() {
                if dot_dot == part {
                    result.pop();
                } else {
                    result.push(part);
                }
            }
            return Ok(result);
        }
        
        pub fn read_to_string_shim(path: &Path) -> Result<String> {
            let path = canonicalize_shim(path).unwrap();        // can't fail
            let file_name = path.to_str().unwrap_or_default();
            // Is this the debugging override?
            if let Some(contents) = OVERRIDE_FILE_NAME.with(|override_name| {
                if file_name == override_name.borrow().as_str() {
                    // debug!("override read_to_string_shim: {}",file_name);
                    return OVERRIDE_FILE_CONTENTS.with(|contents| return Some(contents.borrow().clone()));
                } else {
                    return None;
                }
            }) {
                return Ok(contents);
            };

            // debug!("read_to_string_shim: {}",file_name);

            return FILES.with(|files| {
                let files = files.borrow();
                let zip_file = match files.get(file_name) {
                    None => bail!("Didn't find file '{}'", file_name),
                    Some(data) => data,
                };
                let mut archive = match zip_file.data.as_ref() {
                    None => {
                        let buf_reader = Cursor::new(ZIPPED_RULE_FILES);
                        zip::ZipArchive::new(buf_reader).unwrap()
                    },
                    Some(zip_file) => {
                        let buf_reader = Cursor::new((zip_file).as_ref());
                        zip::ZipArchive::new(buf_reader).unwrap()
                    }
                };
                // for name in archive.file_names() {
                //     debug!(" File: {}", name);
                // };
                let mut file = match archive.by_index(zip_file.index) {
                    Ok(file) => file,
                    Err(..) => {
                        panic!("Didn't find {} in zip archive", file_name);
                    }
                };
    
                let mut contents = String::new();
                if let Err(e) = file.read_to_string(&mut contents) {
                    bail!("read_to_string: {}", e);
                }
                return Ok(contents);
            });
        }

        pub fn zip_extract_shim(dir: &Path, zip_file_name: &str) -> Result<bool> {
            let zip_file_path = dir.join(zip_file_name);
            let full_zip_file_name = zip_file_path.to_str().unwrap_or_default().replace(std::path::MAIN_SEPARATOR_STR, "/");

            // first, extract full_zip_file_name from ZIPPED_RULE_FILES
            let buf_reader = Cursor::new(ZIPPED_RULE_FILES);
            let mut archive = zip::ZipArchive::new(buf_reader).unwrap();
            let mut file = match archive.by_name(&full_zip_file_name) {
                Ok(file) => file,
                Err(..) => {
                    bail!("Didn't find {} in dir {} in zip archive", zip_file_name, dir.display());
                }
            };

            // now add them to FILES
            let mut zip_file_bytes: Vec<u8> = Vec::with_capacity(file.size() as usize);
            if let Err(e) = file.read_to_end(&mut zip_file_bytes) {
                bail!("Failed to extract file {} (size={}): {}", zip_file_path.display(), file.size(), e);
            }
            read_zip_file(dir, Some(zip_file_bytes))?;
            return Ok(true);
        }

        thread_local! {
            // For debugging rules files (mainly nav file)
            static OVERRIDE_FILE_NAME: RefCell<String> = RefCell::new("".to_string());
            static OVERRIDE_FILE_CONTENTS: RefCell<String> = RefCell::new("".to_string());
        }
        pub fn override_file_for_debugging_rules(file_name: &str, file_contents: &str) {
            // file_name should be path name starting at Rules dir: e.g, "Rules/en/navigate.yaml"
            OVERRIDE_FILE_NAME.with(|name| *name.borrow_mut() = file_name.to_string().replace("/", "\\"));
            OVERRIDE_FILE_CONTENTS.with(|contents| *contents.borrow_mut() = file_contents.to_string());
            crate::interface::set_rules_dir("Rules".to_string()).unwrap();       // force reinitialization after the change
        }
    } else {
        pub fn is_file_shim(path: &Path) -> bool {
            return path.is_file();
        }
        
        pub fn is_dir_shim(path: &Path) -> bool {
            return path.is_dir();
        }
        
        pub fn find_files_in_dir_that_ends_with_shim(dir: &Path, ending: &str) ->  Vec<String> {
            match dir.read_dir() {
                Err(_) => return vec![],    // empty
                Ok(read_dir) => {
                    let mut answer = Vec::new();
                    for dir_entry in read_dir.flatten() {
                        let file_name = dir_entry.file_name();
                        let file_name = file_name.to_string_lossy().to_string();
                        if file_name.ends_with(ending) {
                            // this could be (e.g.) xxx_Rules.yaml or it could be subdir/xxx_Rules.yaml
                            let file_name = file_name.split_once(std::path::MAIN_SEPARATOR).map(|(_, after)| after).unwrap_or(&file_name);
                            answer.push( file_name.to_string() );
                        }
                    }
                    return answer;
                }
            }
        }

        pub fn find_all_dirs_shim(dir: &Path, found_dirs: &mut Vec<PathBuf> ) {
            // FIX: this doesn't work for subdirectories that haven't been unzipped yet
            assert!(dir.is_dir(), "find_all_dirs_shim called with non-directory path: {}", dir.display());
            let mut found_rules_file = false;
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        // skip "SharedRules" directory
                        if let Some(dir_name) = path.file_name() {
                            if dir_name.to_str().unwrap_or_default() != "SharedRules" {
                                find_all_dirs_shim(&path, found_dirs);
                            }
                        }
                    } else {
                        let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
                        if !found_rules_file && (file_name.starts_with("unicode") || file_name.ends_with("_Rules.yaml") || file_name.ends_with(".zip")) {
                            found_dirs.push(path.parent().unwrap().to_path_buf());
                            found_rules_file = true;
                        }
                    }
                }
            }
        }
        
        pub fn canonicalize_shim(path: &Path) -> std::io::Result<PathBuf> {
            return path.canonicalize();
        }
        
        pub fn read_to_string_shim(path: &Path) -> Result<String> {
            let path = match path.canonicalize() {
                Ok(path) => path,
                Err(e) => bail!("Read error while trying to canonicalize in read_to_string_shim {}: {}", path.display(), e),
            };
            info!("Reading file '{}'", &path.display());
            match std::fs::read_to_string(&path) {
                Ok(str) => return Ok(str),
                Err(e) => bail!("Read error while trying to read {}: {}", &path.display(), e),
            }
        }

        pub fn zip_extract_shim(dir: &Path, zip_file_name: &str) -> Result<bool> {
            let zip_file = dir.join(zip_file_name);
            return match std::fs::read(zip_file) {
                Err(e) => {
                    // no zip file? -- maybe started out with all the files unzipped? See if there is a .yaml file
                    let yaml_files = find_files_in_dir_that_ends_with_shim(dir, ".yaml");
                    if yaml_files.is_empty() {
                        bail!("{}", e)
                    } else {
                        Ok(false)
                    }
                },
                Ok(contents) => {
                    let archive = std::io::Cursor::new(contents);
                    let mut zip_archive = zip::ZipArchive::new(archive).unwrap();
                    zip_archive.extract(dir).expect("Zip extraction failed");
                    Ok(true)
                },
            };
        }
    }
}
