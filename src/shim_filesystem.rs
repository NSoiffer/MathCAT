//! This is used to paste over normal reading of the Rules files and building them into the code for web assembly (WASM) which
//! can't do file system access. For the latter, the Rules directory should be zipped up.
//! 
//! Note: if files are added or removed, the directory structure needs to be reflected here. This could be automated,
//! but changes are pretty rare and it didn't seem worth it (this may need to be revisited).

use std::path::{Path, PathBuf};

// The zipped files are needed by WASM builds.
// However, they are also useful for other builds because there really isn't another good way to get at the rules.
// Other build scripts can extract these files and unzip to their needed locations.
// I'm not thrilled with this solution as it seems hacky, but I don't know another way for crates to allow for each access to data.
pub static ZIPPED_RULE_FILES: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"),"/rules.zip"));


cfg_if! {
    if #[cfg(target_family = "wasm")] {
        use std::cell::RefCell;

        fn file_system_type_from(path: &Path) -> Option<&str> {
            // Return "file" or "dir" if a match, otherwise None
            use sxd_document::dom::*;
            use std::path::Component;
            use crate::interface::get_element;
            use crate::canonicalize::name;
        
            return DIRECTORY_TREE.with(|files| {
                let files = files.borrow();
                let files = get_element(&*files);
                // the path should be "Rules/..."
                // we don't use a for loop because if we hit a file, we need to check if there are further components (hence, no match)
                let mut children = vec![ChildOfElement::Element(files)];
                let mut components = path.components();
                let mut next_component = components.next();
                let mut matched_dir = None;
                // debug!("path='{}'", path.to_str().unwrap());
                while let Some(component) = next_component {
                    if let Component::Normal(os_str) = component {
                        let component_name = os_str.to_str().unwrap();
                        matched_dir = None;
                        for child in &children {
                            if let ChildOfElement::Element(child) = child {
                                if child.attribute_value("name").unwrap() == component_name {
                                    if name(&child) == "dir" {
                                        matched_dir = Some("dir");
                                        children = child.children();
                                        break;
                                    } else { // name = "file"
                                        return if components.next().is_none() {Some("file")} else {None};
                                    }
                                }    
                            }
                        };
                        if matched_dir.is_none() {
                            return matched_dir;
                        }
                    } else {
                        error!("Expected Component::Normal, found {:?}", component);
                        return None;
                    };
                    next_component = components.next();
                };
                // ran out of components -- must be at a "dir"
                return matched_dir;
            });
        }
        
        pub fn is_file_shim(path: &Path) -> bool {
            let fs = file_system_type_from(path);
            return match fs {
                None => false,
                Some(fs) => fs == "file",
            };
        }
        
        pub fn is_dir_shim(path: &Path) -> bool {
            let fs = file_system_type_from(path);
            return match fs {
                None => false,
                Some(fs) => fs == "dir",
            };
        }
        
        pub fn canonicalize_shim(path: &Path) -> std::io::Result<PathBuf> {
            // FIX:  need to deal with ".."???
            return Ok( path.to_path_buf() );
        }
        
        pub fn read_to_string_shim(path: &Path) -> Result<String, crate::errors::Error> {
            use std::io::Cursor;
            use std::io::Read;

            let file_name = path.to_str().unwrap().replace("\\", "/");
            if let Some(contents) = OVERRIDE_FILE_NAME.with(|override_name| {
                if file_name.as_str() == override_name.borrow().as_str() {
                    debug!("override read_to_string_shim{}",file_name);
                    return OVERRIDE_FILE_CONTENTS.with(|contents| return Some(contents.borrow().clone()));
                } else {
                    return None;
                }
            }) {
                return Ok(contents);
            };
            debug!("read_to_string_shim: {}",file_name);
            let buf_reader = Cursor::new(ZIPPED_RULE_FILES);
            let mut archive = zip::ZipArchive::new(buf_reader).unwrap();
            // for name in archive.file_names() {
            //     debug!(" File: {}", name);
            // };
            let mut file = match archive.by_name(&file_name) {
                Ok(file) => file,
                Err(..) => {
                    panic!("Didn't find {} in zip archive", file_name);
                }
            };

            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            return Ok(contents);
        }

        thread_local! {
            // For debugging rules files (mainly nav file)
            static OVERRIDE_FILE_NAME: RefCell<String> = RefCell::new("".to_string());
            static OVERRIDE_FILE_CONTENTS: RefCell<String> = RefCell::new("".to_string());
        }
        pub fn override_file_for_debugging_rules(file_name: &str, file_contents: &str) {
            // file_name should be path name starting at Rules dir: e.g, "Rules/en/navigate.yaml"
            OVERRIDE_FILE_NAME.with(|name| *name.borrow_mut() = file_name.to_string());
            OVERRIDE_FILE_CONTENTS.with(|contents| *contents.borrow_mut() = file_contents.to_string());
            crate::speech::NAVIGATION_RULES.with(|nav_rules|
                nav_rules.borrow_mut().invalidate(
                    crate::prefs::FilesChanged{
                        speech_rules: true, speech_unicode_short: false, speech_unicode_full: false, 
                        braille_rules: true, braille_unicode_short: false, braille_unicode_full: false, 
                        defs: false }
            ));
        }

        use sxd_document::parser;
        use sxd_document::Package;
        thread_local! {
            // FIX: use include! macro (static DIRECTORY_TREE: ... = include!(...))
            static DIRECTORY_TREE: RefCell<Package> = RefCell::new(
                    parser::parse(r"
                    <dir name='Rules'>
                    <file name='definitions.yaml'/>
                    <file name='intent.yaml'/>
                    <file name='prefs.yaml'/>
                    <dir name='Intent'>
                        <file name='general.yaml'/>
                        <file name='geometry.yaml'/>
                        <file name='linear-algebra.yaml'/>
                    </dir>
                    <dir name='Nemeth'>
                        <file name='Nemeth_Rules.yaml'/>
                        <file name='unicode.yaml'/>
                        <file name='unicode-full.yaml'/>
                    </dir>
                    <dir name='UEB'>
                        <file name='UEB_Rules.yaml'/>
                        <file name='unicode.yaml'/>
                        <file name='unicode-full.yaml'/>
                    </dir>
                    <dir name='en'>
                        <dir name='SharedRules'>
                            <file name='default.yaml'/>
                            <file name='general.yaml'/>
                            <file name='geometry.yaml'/>
                            <file name='linear-algebra.yaml'/>
                            <file name='menclose.yaml'/>
                        </dir>
                        <file name='ClearSpeak_Rules.yaml'/>
                        <file name='definitions.yaml'/>
                        <file name='navigate.yaml'/>
                        <file name='overview.yaml'/>
                        <file name='SimpleSpeak_Rules.yaml'/>
                        <file name='unicode.yaml'/>
                        <file name='unicode-full.yaml'/>
                    </dir>
                    </dir>")
                    .expect("Internal error in creating web assembly files: didn't parse initializer string")
            );
        }        
    } else {
        use crate::errors::*;
        pub fn is_file_shim(path: &Path) -> bool {
            return path.is_file();
        }
        
        pub fn is_dir_shim(path: &Path) -> bool {
            return path.is_dir();
        }
        
        pub fn canonicalize_shim(path: &Path) -> std::io::Result<PathBuf> {
            return path.canonicalize();
        }
        
        pub fn read_to_string_shim(path: &Path) -> Result<String> {
            return std::fs::read_to_string(path).chain_err(|| format!("while trying to read {}", path.to_str().unwrap()));
        }     
    }
}
