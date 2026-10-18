//! Per-process YAML rule coverage events, enabled only by `rule-coverage`.

use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

struct Recorder {
    output: File,
    recorded: HashSet<(PathBuf, String, String)>,
}

static RECORDER: OnceLock<Mutex<Recorder>> = OnceLock::new();

fn recorder() -> &'static Mutex<Recorder> {
    RECORDER.get_or_init(|| {
        let event_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target/rule-coverage/events");
        fs::create_dir_all(&event_dir).expect("cannot create rule coverage event directory");
        let event_file = event_dir.join(format!("pid-{}.jsonl", std::process::id()));
        let output = OpenOptions::new().create(true).append(true).open(event_file)
            .expect("cannot open rule coverage event file");
        Mutex::new(Recorder { output, recorded: HashSet::new() })
    })
}

fn rule_relative_path(path: &Path) -> Option<PathBuf> {
    let rules_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Rules");
    let relative = path.strip_prefix(&rules_dir)
        .or_else(|_| path.strip_prefix("Rules"))
        .ok()?;
    if relative.as_os_str().is_empty() || !matches!(relative.extension().and_then(|s| s.to_str()), Some("yaml" | "yml")) {
        return None;
    }
    Some(relative.to_path_buf())
}

pub(crate) fn defined_rule(path: &Path, name: &str, tag: &str) {
    let Some(relative) = rule_relative_path(path) else { return };
    let mut recorder = recorder().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if !recorder.recorded.insert((relative.clone(), name.to_string(), tag.to_string())) {
        return;
    }
    let path = relative.to_string_lossy();
    let event = serde_json::json!({"kind": "defined-rule", "path": path, "name": name, "tag": tag});
    writeln!(recorder.output, "{event}").expect("cannot write rule coverage event");
}

pub(crate) fn matched_rule(path: &Path, name: &str, tag: &str) {
    let Some(relative) = rule_relative_path(path) else { return };
    let mut recorder = recorder().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let event = serde_json::json!({
        "kind": "matched-rule", "path": relative.to_string_lossy(), "name": name, "tag": tag,
        "test": std::thread::current().name().unwrap_or("(unnamed thread)"),
    });
    writeln!(recorder.output, "{event}").expect("cannot write rule coverage event");
}
