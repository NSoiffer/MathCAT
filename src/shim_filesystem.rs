#![allow(clippy::needless_return)]
//! This is used to paste over normal reading of the Rules files and building them into the code for web assembly (WASM) which
//! can't do file system access. For the latter, the Rules directory is zipped up.

use std::path::{Path, PathBuf};
use crate::errors::*;
use cfg_if::cfg_if;

#[allow(unused_imports)]
use log::{debug};


// The zipped files are needed by WASM builds.
// However, they are also useful for other builds because there really isn't another good way to get at the rules.
// Other build scripts can extract these files and unzip to their needed locations.
// I'm not thrilled with this solution as it seems hacky, but I don't know another way for crates to allow for each access to data.
cfg_if! {
    if #[cfg(any(target_family = "wasm", feature = "include-zip"))] {
        // For the include-zip builds, we build a fake file system based on ZIPPED_RULE_FILES.
        // That stream encodes other zip files that must be unzipped.
        // Only one level of embedded zip files is supported.
        use zip::ZipArchive;
        pub static ZIPPED_RULE_FILES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"),"/rules.zip"));

        /// Struct to indicate where a file is located in the zip archive(s)
        #[derive(Debug, Copy, Clone)]
        struct ArchivePath {
            main: usize,                // index into ZIPPED_RULE_FILES
            zipped: Option<usize>,      // if Some, index into embedded zip file, None if top-level zip file
        }

        use std::cell::RefCell;
        use std::io::Cursor;
        use std::io::Read;
        use std::collections::{HashMap, HashSet};
        thread_local! {
            // mapping the file names to whether they are a directory or a file
            // Note: these are always stored with "/" as the path separator
            static DIRECTORIES: RefCell<HashSet<String>> = RefCell::new(HashSet::with_capacity(127));
            // if a file, we note whether it is in ZIPPED_RULE_FILES or the index of a zipped file within ZIPPED_RULE_FILES
            static FILES: RefCell<HashMap<String, ArchivePath>> = RefCell::new(HashMap::with_capacity(1023));
        }

        /// Canonicalize path separators to "/"
        fn canonicalize_path_separators(path: &Path) -> String {
            return path.to_str().unwrap_or_default().replace("\\", "/");
        }
        
        /// Return a zip archive given the zip bytes
        fn get_zip_archive(zip_bytes: &[u8]) -> Result<ZipArchive<Cursor<&[u8]>>> {
            let buf_reader = Cursor::new(zip_bytes);
            let archive = match zip::ZipArchive::new(buf_reader) {
                Err(e) => bail!("get_zip_archive: failed to create ZipArchive: {}", e),
                Ok(archive) => archive,
            };
            return Ok(archive);
        }

        /// Read ZIPPED_RULE_FILES and build up the FILES and DIRECTORIES static variables.
        /// This is called lazily when the first file or directory check is done.
        fn initialize_static_vars() -> Result<()> {
            let mut archive = get_zip_archive(ZIPPED_RULE_FILES)?;
            read_zip_file("", &mut archive, None)?;

            // Because of Rust's borrow checker, we can't recursively unzip contained zip files (FILES, etc., are borrowed mut)
            // Here we gather up the zip files that were found and iterate over them non-recursively.
            // Note: there shouldn't be embedded zip files in these files (if there are, they won't be unzipped)
            let zip_files = FILES.with(|files| files.borrow().iter()
                .filter_map(|(name, archive_path)| if name.ends_with(".zip") { Some((name.clone(), *archive_path)) } else { None } )
                .collect::<Vec<_>>()
            );
            // debug!("Found {:?} embedded zip files", zip_files);
            for (zip_file_name, archive_path) in zip_files.iter() {
                let bytes = get_bytes_from_index(&mut archive, archive_path.main)?;
                let mut inner_archive = get_zip_archive(bytes.as_slice())?;
                // debug!("  internal zip file {} has {} files", zip_file_name, inner_archive.len());
                let new_containing_dir = zip_file_name.rsplit_once("/").map(|(before, _)| before).unwrap_or("");
                read_zip_file(new_containing_dir, &mut inner_archive, Some(archive_path.main))?;
            }
            // FILES.with(|files| {
            //     let files = files.borrow();
            //     debug!("{} files={:?}", files.len(), files);
            // });
            return Ok(());
        }

        /// Get the bytes for a file in the zip archive (intended for embedded zip files)
        fn get_bytes_from_index(archive: &mut ZipArchive<Cursor<&[u8]>>, index: usize) -> Result<Vec<u8>> {
            let mut file = archive.by_index(index)
                .map_err(|e| anyhow!(format!("Error getting index={} from zip archive: {}", index, e)) )?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)
                .map_err(|e| anyhow!(format!("Error reading index={} from zip archive: {}", index, e)) )?;
            return Ok(contents);
        }
        /// Unzip the zip file (given by zip_archive) and record the file and dir names
        /// 'containing_dir' is the rule dir (RulesDir or a subdir) and establishes a full path for unzipped file(s)
        /// embedded_zip_file is index into ZIPPED_RULE_FILES if this is an embedded zip file, None if it is the top-level zip file
        fn read_zip_file(containing_dir: &str, zip_archive: &mut ZipArchive<Cursor<&[u8]>>, embedded_zip_file: Option<usize>) -> Result<()> {
            // debug!("read_zip_file: containing_dir='{}', zip_archive.len()={}", containing_dir, zip_archive.len());
            return FILES.with(|files| {
                let mut files = files.borrow_mut();
                return DIRECTORIES.with(|dirs| {
                    let mut dirs = dirs.borrow_mut();
                    for i in 0..zip_archive.len() {
                        let file = zip_archive.by_index(i).unwrap();
                        // A little bit of safety/sanity checking
                        let path = match file.enclosed_name() {
                            Some(path) => PathBuf::from(containing_dir).join(path),
                            None => {
                                bail!("Entry {} has a suspicious path (outside of archive)", file.name());
                            }
                        };
                        // debug!("read_zip_file: file path='{}'", path.display());
                        // add all the dirs up to the containing dir -- skip the first one as that is a file
                        // for files like unicode.yaml, this loop is a no-op, but for files in the Shared folder, it will go one time.
                        for parent in path.ancestors().skip(1) {
                            if parent.to_str().unwrap_or_default() == containing_dir {
                                break;
                            }
                            dirs.insert(canonicalize_path_separators(parent));
                        }
                        let file_name = canonicalize_path_separators(&path);
                        if file.is_file() {
                            let archive_path = match embedded_zip_file {
                                None => ArchivePath{ main: i, zipped: None },
                                Some(main) => ArchivePath{ main, zipped: Some(i) },
                            };
                            files.insert(file_name, archive_path);
                        } else if file.is_dir() {
                            dirs.insert(file_name);
                        } else {
                            bail!("read_zip_file: {} is neither a file nor a directory", path.display());
                        }
                    };
                    // debug!("{} files={:?}", files.len(), files);
                    // debug!("{} dirs={:?}", dirs.len(), dirs);
                    return Ok::<(), Error>( () );
                });
            });
        }

        pub fn is_file_shim(path: &Path) -> bool {
            if FILES.with(|files| files.borrow().is_empty()) {
                let _ignore_result = initialize_static_vars();
            }
            return FILES.with(|files| files.borrow().contains_key(&canonicalize_path_separators(path)) );
        }
        
        pub fn is_dir_shim(path: &Path) -> bool {
            if FILES.with(|files| files.borrow().is_empty()) {
                let _ignore_result = initialize_static_vars();
            }
            return DIRECTORIES.with(|dirs| dirs.borrow().contains(&canonicalize_path_separators(path)) );
        }

        /// Find files in 'dir' that end with 'ending' (e.g., "_Rules.yaml")
        pub fn find_files_in_dir_that_ends_with_shim(dir: &Path, ending: &str) -> Vec<String> {
            // FIX: this is very inefficient because it looks through all the files -- maybe dirs should list the files in them?
            // look for files that have 'path' as a prefix
            return FILES.with(|files| {
                let files = files.borrow();
                let mut answer = Vec::new();

                let dir_name = canonicalize_path_separators(dir);
                for file_name in files.keys() {
                    if let Some(dir_relative_name) = file_name.strip_prefix(&dir_name) &&
                       file_name.ends_with(ending)
                    {
                        // this could be (e.g.) xxx_Rules.yaml or it could be subdir/xxx_Rules.yaml
                        let file_name = dir_relative_name.split_once("/").map(|(_, after)| after).unwrap_or(dir_relative_name);
                        answer.push(file_name.to_string());
                    }
                }
                // debug!("find_files_in_dir_that_ends_with_shim: in dir '{}' found {:?}", dir.display(), answer);
                return answer;
            });
        }
        

        pub fn find_all_dirs_shim(dir: &Path, found_dirs: &mut Vec<PathBuf> ) {
            return DIRECTORIES.with(|dirs| {
                let dirs = dirs.borrow();

                let common_dir_name = canonicalize_path_separators(dir);
                for dir_name in dirs.iter() {
                    if dir_name.starts_with(&common_dir_name) && !dir_name.contains("SharedRules") {
                        found_dirs.push(PathBuf::from(&dir_name));
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
        
        /// Read the file at 'path' and return its contents as a String
        pub fn read_to_string_shim(path: &Path) -> Result<String> {
            let path = canonicalize_shim(path).unwrap();        // can't fail
            let file_name = canonicalize_path_separators(&path);
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

            let file_name = file_name.replace('\\', "/"); // zip files always use forward slash
            // top-level zip file or embedded zip file
            return FILES.with(|files| {
                let files = files.borrow();
                let inner_bytes;
                let (bytes, index) = match files.get(&file_name) {
                    Some(archive_path) => {
                        match &archive_path.zipped {
                            None => (ZIPPED_RULE_FILES, archive_path.main),
                            Some(i) => {
                                // debug!("read_to_string_shim: reading embedded zip file {} at index {}", file_name, *i);
                                let mut archive = get_zip_archive(ZIPPED_RULE_FILES)?;
                                inner_bytes = get_bytes_from_index(&mut archive, archive_path.main)?;  // need to hold temp value
                                (inner_bytes.as_slice(), *i)
                            }
                        }
                    },
                    None => bail!("read_to_string_shim: didn't find {} in zip archive", file_name),
                };
                let mut archive = get_zip_archive(bytes)?;
                let mut file = match archive.by_index(index) {
                    Ok(file) => {
                        // debug!("read_to_string_shim: want {}; name of zipped file={:?}", file_name, file.enclosed_name().unwrap());
                        file
                    },
                    Err(..) => {
                        bail!("Didn't find {} in zip archive", file_name);
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
            let full_zip_file_name = canonicalize_path_separators(&zip_file_path);
            match FILES.with(|files| files.borrow().contains_key(full_zip_file_name.as_str()) ) {
                true => Ok(true),
                false => bail!("zip_extract_shim: didn't find {} in zip archive", full_zip_file_name),
            }
        }

        thread_local! {
            // For debugging rules files (mainly nav file) via MathCATDemo
            static OVERRIDE_FILE_NAME: RefCell<String> = RefCell::new("".to_string());
            static OVERRIDE_FILE_CONTENTS: RefCell<String> = RefCell::new("".to_string());
        }
        pub fn override_file_for_debugging_rules(file_name: &str, file_contents: &str) {
            // file_name should be path name starting at Rules dir: e.g, "Rules/en/navigate.yaml"
            OVERRIDE_FILE_NAME.with(|name| *name.borrow_mut() = file_name.to_string().replace("/", "\\"));
            OVERRIDE_FILE_CONTENTS.with(|contents| *contents.borrow_mut() = file_contents.to_string());
            crate::interface::set_rules_dir("Rules").unwrap();       // force reinitialization after the change
        }
    } else {
        use std::collections::{HashMap, HashSet};
        use std::sync::RwLock;

        static IN_MEMORY_FILES: RwLock<Option<HashMap<String, String>>> = RwLock::new(None);
        static IN_MEMORY_DIRS: RwLock<Option<HashSet<String>>> = RwLock::new(None);

        pub fn add_in_memory_file(path: &str, content: &str) {
            {
                let mut files = IN_MEMORY_FILES.write().unwrap();
                files.get_or_insert_with(HashMap::new).insert(path.to_string(), content.to_string());
            }

            let path_obj = Path::new(path);
            {
                let mut dirs = IN_MEMORY_DIRS.write().unwrap();
                let dirs = dirs.get_or_insert_with(HashSet::new);
                for parent in path_obj.ancestors().skip(1) {
                    let parent_str = parent.to_str().unwrap_or_default();
                    if !parent_str.is_empty() {
                        dirs.insert(parent_str.to_string());
                    }
                }
            }
        }

        pub fn canonicalize_shim(path: &Path) -> std::io::Result<PathBuf> {
            use std::ffi::OsStr;
            let dot = OsStr::new(".");
            let dot_dot = OsStr::new("..");
            let mut result = PathBuf::new();
            for part in path.iter() {
                if dot == part {
                    continue;
                } else if dot_dot == part {
                    result.pop();
                } else {
                    result.push(part);
                }
            }
            return Ok(result);
        }

        fn get_in_memory_key(path: &Path) -> String {
            canonicalize_shim(path).unwrap_or_else(|_| path.to_path_buf()).to_str().unwrap_or_default().to_string()
        }

        pub fn is_file_shim(path: &Path) -> bool {
            if IN_MEMORY_FILES.read().unwrap().as_ref().map_or(false, |f| f.contains_key(&get_in_memory_key(path))) {
                return true;
            }
            return path.is_file();
        }
        
        pub fn is_dir_shim(path: &Path) -> bool {
            if IN_MEMORY_DIRS.read().unwrap().as_ref().map_or(false, |d| d.contains(&get_in_memory_key(path))) {
                return true;
            }
            return path.is_dir();
        }
        
        pub fn find_files_in_dir_that_ends_with_shim(dir: &Path, ending: &str) ->  Vec<String> {
            let dir_name = get_in_memory_key(dir);
            let mut answer = Vec::new();
            if let Some(files) = IN_MEMORY_FILES.read().unwrap().as_ref() {
                for file_name in files.keys() {
                    if file_name.starts_with(&dir_name) && file_name.ends_with(ending) {
                        let name = Path::new(file_name).file_name().unwrap_or_default().to_str().unwrap_or_default();
                        answer.push(name.to_string());
                    }
                }
            }
            if !answer.is_empty() {
                return answer;
            }

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
                        if let Some(dir_name) = path.file_name() &&
                           dir_name.to_str().unwrap_or_default() != "SharedRules" {
                            find_all_dirs_shim(&path, found_dirs);
                        }
                    } else {
                        let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
                        if !found_rules_file &&
                           (file_name.starts_with("unicode") || file_name.starts_with("definitions") || file_name.ends_with("_Rules.yaml") || file_name.ends_with(".zip")) {
                            found_dirs.push(path.parent().unwrap().to_path_buf());
                            // FIX: hack to get around not unzipping files and having zh/tw not found
                            if file_name == "zh.zip" {
                                let tw_dir = path.parent().unwrap().join("tw");
                                if !found_dirs.contains(&tw_dir) {
                                    found_dirs.push(tw_dir.to_path_buf());
                                }
                            }
                            found_rules_file = true;
                        }
                    }
                }
            }
        }
        
        // Tests extract language/braille zips in parallel. During extraction, individual YAML
        // files can briefly exist in a partially-written state (created/truncated before the
        // full contents are flushed). Guard both extraction and YAML reads with the same lock
        // so readers never observe truncated YAML.
        use std::sync::Mutex;
        static RULES_ZIP_EXTRACT_LOCK: Mutex<()> = Mutex::new(());

        fn should_lock_rules_yaml_path(path: &Path) -> bool {
            let s = path.to_string_lossy();
            let is_yaml = s.ends_with(".yaml") || s.ends_with(".yml");
            if !is_yaml {
                return false;
            }
            return s.contains("Rules/Languages/") || s.contains("Rules\\Languages\\")
                || s.contains("Rules/Braille/") || s.contains("Rules\\Braille\\");
        }

        pub fn read_to_string_shim(path: &Path) -> Result<String> {
            if let Some(content) = IN_MEMORY_FILES.read().unwrap().as_ref().and_then(|f| f.get(&get_in_memory_key(path)).cloned()) {
                return Ok(content);
            }

            let path = match path.canonicalize() {
                Ok(path) => path,
                Err(_) => path.to_path_buf(),
            };
            debug!("Reading file '{}'", path.display());
            if should_lock_rules_yaml_path(&path) {
                let _guard = RULES_ZIP_EXTRACT_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                match std::fs::read_to_string(&path) {
                    Ok(str) => return Ok(str),
                    Err(e) => bail!("Read error while trying to read {}: {}", path.display(), e),
                }
            } else {
                match std::fs::read_to_string(&path) {
                    Ok(str) => return Ok(str),
                    Err(e) => bail!("Read error while trying to read {}: {}", path.display(), e),
                }
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
                    let _guard = RULES_ZIP_EXTRACT_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                    let archive = std::io::Cursor::new(contents);
                    let mut zip_archive = zip::ZipArchive::new(archive).unwrap();
                    zip_archive.extract(dir).expect("Zip extraction failed");
                    Ok(true)
                },
            };
        }
    }
}

/// compare with config at the top of the file
#[cfg(all(test, not(any(target_family = "wasm", feature = "include-zip"))))]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::Builder;

    fn write_zip_file(dir: &Path, zip_name: &str, archived_file_name: &str, contents: &str) {
        let zip_path = dir.join(zip_name);
        let zip_file = File::create(zip_path).unwrap();
        let mut zip = zip::write::ZipWriter::new(zip_file);
        let options = zip::write::SimpleFileOptions::default();
        zip.start_file(archived_file_name, options).unwrap();
        zip.write_all(contents.as_bytes()).unwrap();
        zip.finish().unwrap();
    }

    /// Verifies the normal extraction path: when a requested zip file exists,
    /// `zip_extract_shim` unpacks it into the target directory and reports that
    /// extraction occurred.
    #[test]
    fn zip_extract_shim_extracts_when_zip_exists() {
        let yaml_path = "some/nested/path/rules.yaml";
        let test_dir = Builder::new().prefix("mathcat-shim-zip-success-").tempdir().unwrap();
        write_zip_file(test_dir.path(), "en.zip", yaml_path, "hello");

        let extracted: bool = zip_extract_shim(test_dir.path(), "en.zip").unwrap();
        assert!(extracted); // zip was extracted
        assert_eq!("hello", fs::read_to_string(test_dir.path().join(yaml_path)).unwrap());
    }

    /// Verifies the filesystem fallback path: if the zip is missing but YAML
    /// files already exist, `zip_extract_shim` should not fail and should return
    /// `false` to indicate that no extraction was needed.
    #[test]
    fn zip_extract_shim_returns_false_when_yaml_files_already_exist() {
        let test_dir = Builder::new().prefix("mathcat-shim-zip-fallback-").tempdir().unwrap();
        fs::write(test_dir.path().join("foo.yaml"), "x: y").unwrap(); // dummy yaml

        let extracted: bool = zip_extract_shim(test_dir.path(), "missing.zip").unwrap();
        assert!(!extracted); // nothing was extracted
    }

    /// Verifies the hard error path: if neither the zip file nor any YAML files
    /// are present, `zip_extract_shim` must return an error so callers can treat
    /// the rules directory as invalid.
    #[test]
    fn zip_extract_shim_errors_when_zip_and_yaml_missing() {
        let test_dir = Builder::new().prefix("mathcat-shim-zip-error-").tempdir().unwrap();
        let result: Result<bool> = zip_extract_shim(test_dir.path(), "missing.zip");

        assert!(result.is_err()); // not true/false, but an error instead
    }

    /// Verifies both `read_to_string_shim` outcomes in one test run:
    /// reading an existing file succeeds with exact contents, and reading a
    /// missing file returns an error.
    #[test]
    fn read_to_string_shim_reads_file_and_reports_missing_file() {
        let test_dir = Builder::new().prefix("mathcat-shim-read-").tempdir().unwrap();

        let existing_file = test_dir.path().join("sample.yaml");
        fs::write(&existing_file, "test-value").unwrap();

        assert_eq!("test-value", read_to_string_shim(&existing_file).unwrap());
        assert!(read_to_string_shim(&test_dir.path().join("does-not-exist.yaml")).is_err());
    }

    #[test]
    fn in_memory_filesystem_shim() {
        add_in_memory_file("virtual/rules/Languages/en/test_Rules.yaml", "key: value");
        let file_path = Path::new("virtual/rules/Languages/en/test_Rules.yaml");
        let dir_path = Path::new("virtual/rules/Languages/en");

        assert!(is_file_shim(file_path));
        assert!(is_dir_shim(dir_path));
        assert_eq!(read_to_string_shim(file_path).unwrap(), "key: value");

        let found = find_files_in_dir_that_ends_with_shim(dir_path, "_Rules.yaml");
        assert_eq!(found, vec!["test_Rules.yaml".to_string()]);
    }
}
