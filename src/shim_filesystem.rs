use std::path::{Path, PathBuf};
cfg_if! {
    if #[cfg(target_family = "wasm")] {
        use std::cell::RefCell;

        fn debug_hack_to_path(path: &Path) -> Option<&Path> {
            // Hack to debug web file system in this library -- comment out if not using
            let path_str = path.to_str().unwrap();
            let index = path_str.find("Rules");
            if index.is_none() {
                return None;
            }
            let rules_part = &path_str[index.unwrap()..];
            return Some( Path::new(rules_part) );
        }

        fn file_system_type_from(path: &Path) -> Option<&str> {
            // Return "file" or "dir" if a match, otherwise None
            use sxd_document::dom::*;
            use std::path::Component;
            use crate::interface::get_element;
            use crate::canonicalize::name;
            // Hack to debug web file system in this library -- comment out if not using
            let path = match debug_hack_to_path(path) {
                None => return None,
                Some(p) => p,
            };            
        
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
        
        pub fn read_to_string_shim(path: &PathBuf) -> Result<String, crate::errors::Error> {
            use std::io::Cursor;
            use std::io::Read;
            static ZIPPED_FILES: &'static [u8] = include_bytes!("..\\Rules.zip");
            // Hack to debug web file system in this library -- comment out if not using
            let path = match debug_hack_to_path(path) {
                None => panic!("Invalid path: {}", path.to_str().unwrap()),
                Some(p) => p,
            };            
            
            let file_name = path.to_str().unwrap().replace("\\", "/");
            debug!("read_to_string_shim: {}",file_name);
            let buf_reader = Cursor::new(ZIPPED_FILES);
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
        // pub fn read_to_string_shim(path: &PathBuf) -> Result<String, crate::errors::Error> {
        //     let url = format!("https://raw.githubusercontent.com/NSoiffer/MathCATDemo/gh-pages/{}", path.to_str().unwrap());
        //     debug!("read_to_string_shim({})", url);
        
        //     let result = read_to_string_shim_async(&url);
        //     debug!("back in sync fn");
            
        //     let output = futures::executor::block_on(result);
        //     return match output {
        //         Ok(s) => Ok(s),
        //         Err(e) => bail!("Error while attempting to 'fetch' from {}: {}", &url, e.as_string().unwrap()),
        //     };
        // }
        
        
        // use wasm_bindgen::JsCast;
        // // use wasm_bindgen::prelude::*;
        // // use yew::services::fetch::{Request};
        // // use yew::services::fetch::{FetchTask, FetchService, Response};
        // use wasm_bindgen::JsValue;
        // async fn read_to_string_shim_async(url: &str) -> Result<String, JsValue> {
        //     use wasm_bindgen_futures::JsFuture;
        //     use web_sys::{Request, RequestInit, RequestMode, Response}; 
            
        //     let mut opts = RequestInit::new();
        //     opts.method("GET");
        //     opts.mode(RequestMode::Cors);
        //     debug!("Before request");
        //     let request = Request::new_with_str_and_init(&url, &opts)?;

        //     request
        //         .headers()
        //         .set("Accept", "text/*")?;
        //     debug!("After request");
        
        //     let window = web_sys::window().unwrap();
        //     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        //     debug!("After fetch");

        //     // `resp_value` is a `Response` object.
        //     assert!(resp_value.is_instance_of::<Response>());
        //     let resp: Response = resp_value.dyn_into().unwrap();
        
        //     // Convert this other `Promise` into a rust `Future`.
        //     let result = JsFuture::from(resp.text()?).await?;
        //     debug!("After JsFuture::from");

        //     return Ok( result.as_string().unwrap() );
        // }
                        
        // FIX: the phf_map! {...} should be in an external file generated by a python script
        // static FILES: phf::Map<&str, &[u8]> = phf_map! {
        //     "Rules/prefs.yaml" => include_bytes!("..\\Rules\\prefs.zip"),
        //     "Rules/definitions.yaml" => include_bytes!("..\\Rules\\definitions.zip"),
        //     "Rules/en/definitions.yaml" => include_bytes!("..\\Rules\\en\\ClearSpeak_Rules.zip"),
        //     "Rules/en/unicode.yaml" => include_bytes!("..\\Rules\\en\\unicode.zip"),
        // };

        use sxd_document::parser;
        use sxd_document::Package;
        thread_local! {
            // FIX: use include! macro (pub static DIRECTORY_TREE: ... = include!(...))
            pub static DIRECTORY_TREE: RefCell<Package> = RefCell::new(
                    parser::parse(r"
                    <dir name='Rules'>
                    <file name='definitions.yaml'/>
                    <file name='infer.yaml'/>
                    <file name='prefs.yaml'/>
                    <dir name='Nemeth'>
                        <file name='Nemeth_Rules.yaml'/>
                        <file name='unicode.yaml'/>
                    </dir>
                    <dir name='en'>
                        <file name='ClearSpeak_Rules.yaml'/>
                        <file name='menclose.yaml'/>
                        <file name='unicode.yaml'/>
                        <file name='common-rules.yaml'/>
                        <file name='navigate.yaml'/>
                        <file name='unicode.yaml'/>
                        <file name='unicode-all.yaml'/>
                        <file name='default-rules.yaml'/>
                        <file name='overview.yaml'/>
                        <file name='definitions.yaml'/>
                        <file name='SimpleSpeak_Rules.yaml'/>
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
        
        pub fn read_to_string_shim(path: &PathBuf) -> Result<String> {
            return std::fs::read_to_string(path).chain_err(|| format!("while trying to read {}", path.to_str().unwrap()));
        }     
    }
}
