// Shared Rules archive writer used by `build.rs` (include-zip) and the `package-rules` binary.
//
// Layout matches native MathCAT extraction:
// - `Rules/*.yaml` and `Rules/Intent/**` stay loose in the outer archive
// - each `Rules/Languages/<lang>/` and `Rules/Braille/<code>/` directory is stored as `<name>/<name>.zip`
// - `Rules/Languages/zz` (synthetic test language) is omitted
//
// Inner language/braille zips always use BZIP2. Downloadable `Rules.zip` uses DEFLATE for the
// outer archive so Windows Explorer / `unzip` can open it. The include-zip embed uses BZIP2
// for both layers (MathCAT's `zip` crate reads it; zip 9.x bzip2 is pure Rust and works on wasm).

use std::fs::{self, File, read_dir};
use std::io::{self, Cursor, Read, Seek, Write};
use std::path::{Path, PathBuf};

use yaml_rust::{Yaml, YamlLoader};
use zip::write::{SimpleFileOptions, ZipWriter};
use zip::CompressionMethod;

const YAML_SUFFIXES: [&str; 2] = ["yaml", "yml"];
const SKIP_LANGUAGE_DIR: &str = "zz";
const ARCHIVE_ROOT: &str = "Rules";
const LANGUAGES_DIR: &str = "Languages";
const BRAILLE_DIR: &str = "Braille";
const PREFS_FILE: &str = "prefs.yaml";
const BRAILLE_CODE_PREF: &str = "BrailleCode";
/// `find_file` falls back to this language by name, so it cannot be pruned away.
const FALLBACK_LANGUAGE: &str = "en";
/// `set_rules_dir` fails unless the default braille code's directory exists, even for a caller that
/// never asks for braille, so a subset that keeps no braille code keeps this one.
const FALLBACK_BRAILLE_CODE: &str = "UEB";

/// Which `Languages/` and `Braille/` directories to keep, by directory name.
///
/// The full Rules tree is about 10 MB, and a caller that only ever asks for one language and one
/// braille code pays for all of it: `include-zip` embeds every directory in the binary. `None`
/// means keep everything, which is what every existing caller gets. `build.rs` fills this in from
/// the `lang-*` and `braille-*` features.
#[derive(Clone, Default)]
pub struct RulesSubset {
    pub languages: Option<Vec<String>>,
    pub braille_codes: Option<Vec<String>>,
}

impl RulesSubset {
    /// Keeps everything, the behaviour before this existed.
    pub fn all() -> Self {
        return RulesSubset::default();
    }

    /// Regional variants such as `en/gb` and `zh/tw` live inside their language's directory, so
    /// keeping a language keeps them too.
    fn keeps(wanted: &Option<Vec<String>>, name: &str) -> bool {
        let Some(wanted) = wanted else {
            return true;
        };
        return wanted.iter().any(|w| w.eq_ignore_ascii_case(name));
    }

    fn keeps_language(&self, name: &str) -> bool {
        // `find_file` falls back to "en" by name from inside the library, so a subset without it
        // cannot resolve anything.
        return name.eq_ignore_ascii_case(FALLBACK_LANGUAGE) || RulesSubset::keeps(&self.languages, name);
    }

    fn keeps_braille_code(&self, name: &str) -> bool {
        if self.braille_codes.as_ref().is_some_and(|codes| codes.is_empty()) {
            return name.eq_ignore_ascii_case(FALLBACK_BRAILLE_CODE);
        }
        return RulesSubset::keeps(&self.braille_codes, name);
    }
}

/// Compression methods for nested language zips vs the outer archive.
#[derive(Clone, Copy)]
pub struct ArchiveCompression {
    pub inner: CompressionMethod,
    pub outer: CompressionMethod,
}

/// Inner BZIP2 / outer BZIP2 — used for the include-zip embed (only MathCAT reads it).
pub fn include_zip_compression() -> ArchiveCompression {
    return ArchiveCompression {
        inner: CompressionMethod::BZIP2,
        outer: CompressionMethod::BZIP2,
    };
}

/// Inner BZIP2 / outer DEFLATE — used for downloadable `Rules.zip` / `Rules-minimized.zip`.
pub fn downloadable_compression() -> ArchiveCompression {
    return ArchiveCompression {
        inner: CompressionMethod::BZIP2,
        outer: CompressionMethod::DEFLATE,
    };
}

fn zip_options(method: CompressionMethod) -> SimpleFileOptions {
    return SimpleFileOptions::default()
        .compression_method(method)
        .compression_level(Some(9));
}

fn is_yaml_file(path: &Path) -> bool {
    return path
        .extension()
        .map(|ext| {
            let ext = ext.to_ascii_lowercase();
            YAML_SUFFIXES.iter().any(|s| ext == *s)
        })
        .unwrap_or(false);
}

fn is_zip_file(path: &Path) -> bool {
    return path.extension().map(|ext| ext.eq_ignore_ascii_case("zip")).unwrap_or(false);
}

fn file_name_str(path: &Path) -> io::Result<String> {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| io::Error::other(format!("non-UTF-8 file name: {}", path.display())))?;
    return Ok(name.to_string());
}
/// Load YAML, drop comments, re-emit in flow style. Returns `Err` if yaml-rust cannot parse the file.
pub fn minimize_yaml_text(text: &str) -> Result<String, String> {
    let normalized = text.replace('\t', "    ");
    let docs = YamlLoader::load_from_str(&normalized).map_err(|e| e.to_string())?;
    if docs.is_empty() {
        return Ok(String::new());
    }
    let mut out = String::new();
    for (i, doc) in docs.iter().enumerate() {
        if i > 0 {
            out.push_str("\n---\n");
        }
        emit_flow(&mut out, doc)?;
    }
    out.push('\n');
    // yaml-rust quotes scalars that parse as f64 (including "infinity"); keep a guard in case
    // a future emitter change stops doing that — unquoted `infinity` becomes a float on load.
    if looks_like_unquoted_infinity(&out) {
        return Err("emitted unquoted 'infinity' (yaml-rust would treat it as a float)".to_string());
    }
    return Ok(out);
}

fn emit_flow(out: &mut String, yaml: &Yaml) -> Result<(), String> {
    match yaml {
        Yaml::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                emit_flow(out, item)?;
            }
            out.push(']');
        }
        Yaml::Hash(hash) => {
            out.push('{');
            for (i, (key, value)) in hash.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                emit_flow(out, key)?;
                out.push_str(": ");
                emit_flow(out, value)?;
            }
            out.push('}');
        }
        Yaml::String(s) => emit_yaml_string(out, s),
        Yaml::Boolean(true) => out.push_str("true"),
        Yaml::Boolean(false) => out.push_str("false"),
        Yaml::Integer(i) => out.push_str(&i.to_string()),
        Yaml::Real(s) => out.push_str(s),
        Yaml::Null | Yaml::BadValue => out.push('~'),
        Yaml::Alias(_) => return Err("cannot minimize YAML that contains aliases".to_string()),
    }
    return Ok(());
}

fn emit_yaml_string(out: &mut String, s: &str) {
    if need_quotes(s) {
        escape_yaml_string(out, s);
    } else {
        out.push_str(s);
    }
}

/// Same quoting rules as yaml-rust 0.4's emitter, so load/emit round-trips.
fn need_quotes(string: &str) -> bool {
    string.is_empty()
        || string.starts_with(' ')
        || string.ends_with(' ')
        || string.starts_with(|character: char| {
            matches!(character, '&' | '*' | '?' | '|' | '-' | '<' | '>' | '=' | '!' | '%' | '@')
        })
        || string.contains(|character: char| {
            matches!(
                character,
                ':' | '{' | '}' | '[' | ']' | ',' | '#' | '`' | '"' | '\'' | '\\'
                    | '\0'..='\x06'
                    | '\t'
                    | '\n'
                    | '\r'
                    | '\x0e'..='\x1a'
                    | '\x1c'..='\x1f'
            )
        })
        || [
            "yes", "Yes", "YES", "no", "No", "NO", "True", "TRUE", "true", "False", "FALSE",
            "false", "on", "On", "ON", "off", "Off", "OFF", "null", "Null", "NULL", "~",
        ]
        .contains(&string)
        || string.starts_with('.')
        || string.starts_with("0x")
        || string.parse::<i64>().is_ok()
        || string.parse::<f64>().is_ok()
}

fn escape_yaml_string(out: &mut String, v: &str) {
    out.push('"');
    let mut start = 0;
    for (i, byte) in v.bytes().enumerate() {
        let escaped = match byte {
            b'"' => "\\\"",
            b'\\' => "\\\\",
            b'\x00' => "\\u0000",
            b'\x01' => "\\u0001",
            b'\x02' => "\\u0002",
            b'\x03' => "\\u0003",
            b'\x04' => "\\u0004",
            b'\x05' => "\\u0005",
            b'\x06' => "\\u0006",
            b'\x07' => "\\u0007",
            b'\x08' => "\\b",
            b'\t' => "\\t",
            b'\n' => "\\n",
            b'\x0b' => "\\u000b",
            b'\x0c' => "\\f",
            b'\r' => "\\r",
            b'\x0e' => "\\u000e",
            b'\x0f' => "\\u000f",
            b'\x10' => "\\u0010",
            b'\x11' => "\\u0011",
            b'\x12' => "\\u0012",
            b'\x13' => "\\u0013",
            b'\x14' => "\\u0014",
            b'\x15' => "\\u0015",
            b'\x16' => "\\u0016",
            b'\x17' => "\\u0017",
            b'\x18' => "\\u0018",
            b'\x19' => "\\u0019",
            b'\x1a' => "\\u001a",
            b'\x1b' => "\\u001b",
            b'\x1c' => "\\u001c",
            b'\x1d' => "\\u001d",
            b'\x1e' => "\\u001e",
            b'\x1f' => "\\u001f",
            b'\x7f' => "\\u007f",
            _ => continue,
        };
        if start < i {
            out.push_str(&v[start..i]);
        }
        out.push_str(escaped);
        start = i + 1;
    }
    if start != v.len() {
        out.push_str(&v[start..]);
    }
    out.push('"');
}

fn looks_like_unquoted_infinity(yaml: &str) -> bool {
    // Block-style leftovers and flow mappings (`: infinity}` / `: infinity,`).
    for line in yaml.lines() {
        let trimmed = line.trim();
        if trimmed == "infinity" || trimmed.ends_with(": infinity") || trimmed.ends_with("- infinity") {
            return true;
        }
    }
    return yaml.contains(": infinity,") || yaml.contains(": infinity}") || yaml.contains(": infinity]");
}

fn minimize_yaml_file(src: &Path, dst: &Path) -> Result<(), String> {
    let text = fs::read_to_string(src).map_err(|e| e.to_string())?;
    let minimized = minimize_yaml_text(&text)?;
    // One document after a successful load+emit — MathCAT requires docs.len() == 1.
    let docs = YamlLoader::load_from_str(&minimized).map_err(|e| e.to_string())?;
    if docs.len() != 1 {
        return Err(format!("minimized YAML is {} documents, expected 1", docs.len()));
    }
    let _ = docs;
    fs::write(dst, minimized).map_err(|e| e.to_string())?;
    return Ok(());
}

/// Copy a Rules tree. When `minify` is true, rewrite every YAML file as comment-free flow YAML.
/// `Languages/zz` is omitted. Returns the number of files successfully minimized.
pub fn copy_rules_tree(src: &Path, dst: &Path, minify: bool) -> io::Result<usize> {
    return copy_rules_subset(src, dst, minify, &RulesSubset::all());
}

/// As [`copy_rules_tree`], keeping only the languages and braille codes `subset` asks for.
pub fn copy_rules_subset(src: &Path, dst: &Path, minify: bool, subset: &RulesSubset) -> io::Result<usize> {
    fs::create_dir_all(dst)?;
    let minimized = copy_dir(src, dst, minify, None, subset)?;
    if let Some(codes) = &subset.braille_codes {
        set_default_braille_code(&dst.join(PREFS_FILE), codes)?;
    }
    return Ok(minimized);
}

/// Points the `BrailleCode` default in a staged `prefs.yaml` at the first of `kept` (or at
/// [`FALLBACK_BRAILLE_CODE`] when it is empty) when the default was pruned, and leaves it alone when
/// it was kept.
///
/// Without this, the default (Nemeth) and the library's own fallback (UEB) are both missing from a
/// subset that asked for neither, and `set_rules_dir` reports that it could not read the braille
/// directory. Keeping Nemeth and UEB instead would cost about 50 KB for rules nothing asks for.
fn set_default_braille_code(prefs: &Path, kept: &[String]) -> io::Result<()> {
    let text = fs::read_to_string(prefs)?;
    let docs = YamlLoader::load_from_str(&text).map_err(io::Error::other)?;
    let current = docs.iter().find_map(|doc| find_value(doc, BRAILLE_CODE_PREF));
    if current.is_some_and(|current| kept.iter().any(|code| code.eq_ignore_ascii_case(current))) {
        return Ok(());
    }
    let code = kept.first().map_or(FALLBACK_BRAILLE_CODE, |code| code.as_str());
    let mut out = String::new();
    for (i, doc) in docs.iter().enumerate() {
        if i > 0 {
            out.push_str("\n---\n");
        }
        emit_flow(&mut out, &with_replaced_value(doc, BRAILLE_CODE_PREF, code)).map_err(io::Error::other)?;
    }
    out.push('\n');
    fs::write(prefs, out)?;
    return Ok(());
}

/// The string value of the first `key` entry in `yaml`, at any depth.
fn find_value<'a>(yaml: &'a Yaml, key: &str) -> Option<&'a str> {
    match yaml {
        Yaml::Hash(hash) => {
            return hash.iter().find_map(|(k, v)| match (k, v) {
                (Yaml::String(name), Yaml::String(value)) if name == key => Some(value.as_str()),
                _ => find_value(v, key),
            });
        }
        Yaml::Array(items) => return items.iter().find_map(|item| find_value(item, key)),
        _ => return None,
    }
}

/// `yaml` with the value of every `key` entry, at any depth, replaced by `value`.
fn with_replaced_value(yaml: &Yaml, key: &str, value: &str) -> Yaml {
    match yaml {
        Yaml::Hash(hash) => {
            let mut out = yaml_rust::yaml::Hash::new();
            for (k, v) in hash {
                let replace = matches!(k, Yaml::String(name) if name == key);
                let v = if replace { Yaml::String(value.to_string()) } else { with_replaced_value(v, key, value) };
                out.insert(k.clone(), v);
            }
            return Yaml::Hash(out);
        }
        Yaml::Array(items) => {
            return Yaml::Array(items.iter().map(|item| with_replaced_value(item, key, value)).collect());
        }
        other => return other.clone(),
    }
}

fn copy_dir(
    src: &Path,
    dst: &Path,
    minify: bool,
    parent: Option<&str>,
    subset: &RulesSubset,
) -> io::Result<usize> {
    let mut minimized = 0usize;
    for entry in read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let name = file_name_str(&src_path)?;
        if parent == Some(LANGUAGES_DIR) && (name == SKIP_LANGUAGE_DIR || !subset.keeps_language(&name)) {
            continue;
        }
        if parent == Some(BRAILLE_DIR) && src_path.is_dir() && !subset.keeps_braille_code(&name) {
            continue;
        }
        if is_zip_file(&src_path) {
            continue;
        }
        let dst_path = dst.join(&name);
        if src_path.is_dir() {
            fs::create_dir_all(&dst_path)?;
            minimized += copy_dir(&src_path, &dst_path, minify, Some(name.as_str()), subset)?;
        } else if minify && is_yaml_file(&src_path) {
            match minimize_yaml_file(&src_path, &dst_path) {
                Ok(()) => minimized += 1,
                Err(e) => {
                    eprintln!(
                        "warning: could not minimize {}; copied verbatim ({})",
                        src_path.display(),
                        e
                    );
                    fs::copy(&src_path, &dst_path)?;
                }
            }
        } else {
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&src_path, &dst_path)?;
        }
    }
    return Ok(minimized);
}

fn add_file_to_zip<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    in_path: &Path,
    zip_path: &Path,
    options: SimpleFileOptions,
) -> io::Result<()> {
    zip.start_file_from_path(zip_path, options)
        .map_err(io::Error::other)?;
    let mut file = File::open(in_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    zip.write_all(&buffer)?;
    return Ok(());
}

fn add_bytes_to_zip<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    zip_path: &Path,
    bytes: &[u8],
    options: SimpleFileOptions,
) -> io::Result<()> {
    zip.start_file_from_path(zip_path, options)
        .map_err(io::Error::other)?;
    zip.write_all(bytes)?;
    return Ok(());
}

fn zip_entry<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    full_path: &Path,
    relative_path: &Path,
    options: SimpleFileOptions,
) -> io::Result<usize> {
    let mut n_files_in_zip = 0;
    if full_path.is_dir() {
        for entry in read_dir(full_path)? {
            let entry_path = entry?.path();
            if is_zip_file(&entry_path) {
                continue;
            }
            let entry_name = file_name_str(&entry_path)?;
            let child_rel = relative_path.join(entry_name);
            n_files_in_zip += zip_entry(zip, &entry_path, &child_rel, options)?;
        }
    } else if is_yaml_file(full_path) {
        add_file_to_zip(zip, full_path, relative_path, options)?;
        n_files_in_zip += 1;
    }
    return Ok(n_files_in_zip);
}

fn write_inner_zip(source_dir: &Path, inner: SimpleFileOptions) -> io::Result<Option<Vec<u8>>> {
    let cursor = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(cursor);
    let n_files = zip_entry(&mut zip, source_dir, Path::new(""), inner)?;
    let cursor = zip.finish().map_err(io::Error::other)?;
    if n_files == 0 {
        return Ok(None);
    }
    return Ok(Some(cursor.into_inner()));
}

/// Zip each immediate subdirectory of Languages/ or Braille/ into `<name>/<name>.zip`.
fn zip_dir<W: Write + Seek>(
    rules_section: &Path,
    archive_zip: &mut ZipWriter<W>,
    inner: SimpleFileOptions,
    outer: SimpleFileOptions,
    archive_prefix: &Path,
) -> io::Result<()> {
    if !rules_section.is_dir() {
        return Ok(());
    }
    for entry in read_dir(rules_section)? {
        let entry_path = entry?.path();
        if is_zip_file(&entry_path) {
            continue;
        }
        if entry_path.is_dir() {
            let dir_name = file_name_str(&entry_path)?;
            if dir_name == SKIP_LANGUAGE_DIR {
                continue;
            }
            if let Some(bytes) = write_inner_zip(&entry_path, inner)? {
                let zip_path = archive_prefix.join(&dir_name).join(format!("{dir_name}.zip"));
                add_bytes_to_zip(archive_zip, &zip_path, &bytes, outer)?;
            }
        } else if is_yaml_file(&entry_path) {
            // e.g. Rules/Braille/definitions.yaml
            let name = file_name_str(&entry_path)?;
            let zip_path = archive_prefix.join(name);
            add_file_to_zip(archive_zip, &entry_path, &zip_path, outer)?;
        }
    }
    return Ok(());
}

/// Loose YAML at Rules/ and Rules/Intent/ (not zipped per-directory).
fn zip_other_files<W: Write + Seek>(
    rules_dir: &Path,
    archive_zip: &mut ZipWriter<W>,
    outer: SimpleFileOptions,
    archive_prefix: &Path,
) -> io::Result<()> {
    for entry in read_dir(rules_dir)? {
        let entry_path = entry?.path();
        let entry_name = file_name_str(&entry_path)?;
        if entry_path.is_dir() {
            if entry_name == "Intent" {
                zip_other_files(
                    &rules_dir.join("Intent"),
                    archive_zip,
                    outer,
                    &archive_prefix.join("Intent"),
                )?;
            }
        } else if is_yaml_file(&entry_path) {
            let zip_path = archive_prefix.join(entry_name);
            add_file_to_zip(archive_zip, &entry_path, &zip_path, outer)?;
        }
    }
    return Ok(());
}

/// Write a MathCAT Rules archive from an already-staged tree (`rules_dir` is the `Rules` folder).
pub fn write_rules_archive(
    rules_dir: &Path,
    output: &Path,
    compression: ArchiveCompression,
) -> io::Result<()> {
    if let Some(parent) = output.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    let inner = zip_options(compression.inner);
    let outer = zip_options(compression.outer);
    let archive_zip_file = File::create(output)?;
    let mut archive_zip = ZipWriter::new(archive_zip_file);
    let archive_root = Path::new(ARCHIVE_ROOT);

    zip_other_files(rules_dir, &mut archive_zip, outer, archive_root)?;
    zip_dir(
        &rules_dir.join("Languages"),
        &mut archive_zip,
        inner,
        outer,
        &archive_root.join("Languages"),
    )?;
    zip_dir(
        &rules_dir.join("Braille"),
        &mut archive_zip,
        inner,
        outer,
        &archive_root.join("Braille"),
    )?;

    archive_zip.finish().map_err(io::Error::other)?;
    return Ok(());
}

/// Copy (optionally minify) `source` and write a downloadable or embeddable Rules archive.
pub fn package_rules(
    source: &Path,
    output: &Path,
    minify: bool,
    compression: ArchiveCompression,
) -> io::Result<usize> {
    if !source.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Rules source directory not found: {}", source.display()),
        ));
    }

    let staging_root = tempfile_staging_dir()?;
    let staged_rules = staging_root.join(ARCHIVE_ROOT);
    let minimized = copy_rules_tree(source, &staged_rules, minify)?;
    write_rules_archive(&staged_rules, output, compression)?;
    let _ = fs::remove_dir_all(&staging_root);
    return Ok(minimized);
}

fn tempfile_staging_dir() -> io::Result<PathBuf> {
    let base = std::env::temp_dir().join(format!(
        "mathcat-rules-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&base)?;
    return Ok(base);
}
