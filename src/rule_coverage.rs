//! Rule-level coverage tracking for YAML speech/braille/intent rules.
//! Enabled only when the `rule-coverage` Cargo feature is active.
//! 
//! - When rules are loaded, each rule registers and gets a small integer id.
//! - When a rule matches, we record a hit for that id.
//! - A thread-local guard triggers a one-time LCOV export on program/test shutdown,
//!   so callers don’t need to remember to “flush” coverage.
//! All state is behind a Mutex for safety; exports land in `target/rule-coverage/*.info`.
//!
//! To view everything on one page, regenerate HTML with:
//!   genhtml --flat target/rule-coverage/lcov.info -o target/rule-coverage/html-flat
//! then open `target/rule-coverage/html-flat/index.html`.
#![cfg(feature = "rule-coverage")]

use std::io::{self, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, Mutex};

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
thread_local! {
    // One guard per thread. When the thread ends, its Drop runs; only the first guard
    // across all threads performs the export (see DID_EXPORT below). This avoids needing
    // an explicit "finish" call and still works when tests spawn threads.
    static EXPORT_GUARD: ExportGuard = ExportGuard;
}

#[derive(Default, Debug)]
struct RuleEntry {
    name: String, // pattern name (optionally with tag)
    hits: u64,
}

#[derive(Default, Debug)]
struct FileEntry {
    path: String,
    rules: Vec<RuleEntry>,
}

#[derive(Default, Debug)]
struct Coverage {
    files: Vec<FileEntry>,
    index: Vec<(usize, usize)>, // id -> (file, rule)
}
impl Coverage {
    fn clear(&mut self) { self.files.clear(); self.index.clear(); }
}

static COVERAGE: LazyLock<Mutex<Coverage>> = LazyLock::new(|| Mutex::new(Coverage::default()));

fn normalize_path(path: &str) -> String {
    let path = Path::new(path);
    if let Ok(cwd) = std::env::current_dir() {
        if let Ok(stripped) = path.strip_prefix(&cwd) {
            return stripped.to_string_lossy().into_owned();
        }
    }
    path.to_string_lossy().into_owned()
}

pub fn register_rule(file_path: &str, rule_name: &str, tag_name: &str) -> usize {
    ensure_guard();
    let mut cov = COVERAGE.lock().unwrap();
    let path = normalize_path(file_path);
    let file_index = match cov.files.iter().position(|f| f.path == path) {
        Some(i) => i,
        None => { cov.files.push(FileEntry { path: path.clone(), rules: Vec::new() }); cov.files.len() - 1 }
    };
    let composite = format!("{rule_name} ({tag_name})");
    if let Some(rule_index) = cov.files[file_index].rules.iter().position(|r| r.name == composite) {
        if let Some(id) = cov.index.iter().position(|&(f, r)| f == file_index && r == rule_index) { return id; }
    }
    let rule_index = cov.files[file_index].rules.len();
    cov.files[file_index].rules.push(RuleEntry { name: composite, hits: 0 });
    let id = cov.index.len();
    cov.index.push((file_index, rule_index));
    id
}

pub fn record_rule_hit(id: usize) {
    ensure_guard();
    let mut cov = COVERAGE.lock().unwrap();
    if let Some(&(f, r)) = cov.index.get(id) {
        if let Some(rule) = cov.files.get_mut(f).and_then(|ff| ff.rules.get_mut(r)) {
            rule.hits += 1;
        }
    }
}

pub fn reset_rule_coverage() { COVERAGE.lock().unwrap().clear(); }

/// Emit LCOV records:
///   FN/FNDA for rule declarations and hit counts
///   DA for per-rule line hits (one line per rule here)
///   LF/LH for lines found/hit; FNF/FNH for functions (rules) found/hit
///
/// See: https://manpages.debian.org/trixie/lcov/geninfo.1.en.html
pub fn export_rule_coverage_lcov<W: Write>(mut w: W) -> io::Result<()> {
    let cov = COVERAGE.lock().unwrap();
    for file in &cov.files {
        let total = file.rules.len();
        let covered = file.rules.iter().filter(|r| r.hits > 0).count();
        writeln!(w, "SF:{}", file.path)?;
        for (i, rule) in file.rules.iter().enumerate() {
            let line = i + 1;
            writeln!(w, "FN:{line},{}", rule.name)?;
        }
        for rule in &file.rules {
            writeln!(w, "FNDA:{},{}", rule.hits, rule.name)?;
        }
        for (i, rule) in file.rules.iter().enumerate() {
            let line = i + 1;
            writeln!(w, "DA:{line},{}", rule.hits)?;
        }
        writeln!(w, "LF:{total}")?;
        writeln!(w, "LH:{}", covered)?;
        writeln!(w, "FNF:{total}")?;
        writeln!(w, "FNH:{}", covered)?;
        writeln!(w, "end_of_record")?;
    }
    Ok(())
}

fn ensure_guard() {
    EXPORT_GUARD.with(|_| {});
    static DID_RESET: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
    let mut done = DID_RESET.lock().unwrap();
    if !*done {
        reset_rule_coverage();
        *done = true;
    }
}

// RAII helper: when an ExportGuard is dropped, it attempts to export coverage.
// A global AtomicBool ensures only the first drop across all threads writes the LCOV,
// so multiple threads shutting down won't duplicate the file.
struct ExportGuard;
static DID_EXPORT: AtomicBool = AtomicBool::new(false);

impl Drop for ExportGuard {
    fn drop(&mut self) {
        if DID_EXPORT.swap(true, Ordering::SeqCst) { return; }
        use std::fs::{create_dir_all, File};
        use std::path::PathBuf;
        let mut path = PathBuf::from(MANIFEST_DIR).join("target/rule-coverage");
        if create_dir_all(&path).is_err() { return; }
        let exe = std::env::current_exe().ok()
            .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
            .unwrap_or_else(|| "tests".to_string());
        path.push(format!("{exe}.info"));
        if let Ok(mut f) = File::create(&path) { let _ = export_rule_coverage_lcov(&mut f); }
    }
}
