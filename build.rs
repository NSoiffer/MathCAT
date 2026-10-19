//! The build.rs file is necessary to generate rules.zip.
//! rules.zip are needed so there is a way to get the rules dir into the build since you can't get from the crate.
//! The expectation is that most builds (with the exception of WASM builds) will need a build.rs file to extract the rules.
#![allow(clippy::needless_return)]
#![allow(dead_code)] // include! of rules_archive.rs pulls in the downloadable packager too

include!("src/rules_archive.rs");

/// The `Rules/<kind>` directories whose `<prefix>-<name>` feature is on, or `None` when all of them
/// are, so a default build stages exactly what it always has.
///
/// Panics if a directory is missing from `<prefix>-all` in Cargo.toml. Otherwise a newly added
/// language or braille code would quietly drop out of every build, since nothing would turn it on.
fn subset_from_features(rules_dir: &Path, kind: &str, prefix: &str, manifest: &str) -> Option<Vec<String>> {
    let all_feature = format!("{}-all", prefix);
    // The list can span lines: `cargo publish` rewrites Cargo.toml with one entry per line.
    let all_list = manifest
        .split(&format!("
{} = [", all_feature))
        .nth(1)
        .and_then(|rest| rest.split(']').next())
        .unwrap_or_else(|| panic!("Cargo.toml has no `{}` feature", all_feature));
    let mut kept = Vec::new();
    let mut everything = true;
    for entry in read_dir(rules_dir.join(kind)).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !path.is_dir() || (kind == LANGUAGES_DIR && name == SKIP_LANGUAGE_DIR) {
            continue;
        }
        let feature = format!("{}-{}", prefix, name.to_ascii_lowercase());
        if !all_list.contains(&format!("\"{}\"", feature)) {
            panic!(
                "Rules/{}/{} has no feature: add `{} = []` to Cargo.toml and list it in `{}`",
                kind, name, feature, all_feature
            );
        }
        let env_var = format!("CARGO_FEATURE_{}", feature.to_ascii_uppercase().replace('-', "_"));
        if std::env::var_os(env_var).is_some() {
            kept.push(name);
        } else {
            everything = false;
        }
    }
    kept.sort();
    return if everything { None } else { Some(kept) };
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/rules_archive.rs");
    println!("cargo::rerun-if-changed=Rules");
    println!("cargo::rerun-if-changed=Cargo.toml");

    let rules_dir = std::env::current_dir().unwrap().join("Rules");
    let manifest = fs::read_to_string("Cargo.toml").unwrap();
    // Checked on every build, not only include-zip ones, so a rules directory added without a
    // feature is caught by whoever adds it.
    let languages = subset_from_features(&rules_dir, LANGUAGES_DIR, "lang", &manifest);
    let braille_codes = subset_from_features(&rules_dir, BRAILLE_DIR, "braille", &manifest);

    // This doesn't work because the build claims OUT_DIR is not defined(?)
    // let archive = PathBuf::from(concat!(env!("OUT_DIR"),"/rules.zip"));
    if std::env::var("CARGO_FEATURE_INCLUDE_ZIP").is_ok() {
        let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
        let staging = out_dir.join("rules_src");
        if staging.exists() {
            let _ = std::fs::remove_dir_all(&staging);
        }
        let subset = RulesSubset { languages, braille_codes };
        if let Some(langs) = &subset.languages {
            let mut langs = langs.clone();
            if !langs.iter().any(|l| l.eq_ignore_ascii_case("en")) {
                langs.push("en (the fallback)".to_string());
            }
            println!("cargo::warning=MathCAT rules limited to languages: {}", langs.join(", "));
        }
        if let Some(codes) = &subset.braille_codes {
            let codes = if codes.is_empty() { "UEB (the fallback)".to_string() } else { codes.join(", ") };
            println!("cargo::warning=MathCAT rules limited to braille codes: {}", codes);
        }
        if let Err(e) = copy_rules_subset(&rules_dir, &staging, true, &subset) {
            panic!("build.rs failed to stage minimized Rules: {}", e);
        }
        let archive_path = out_dir.join("rules.zip");
        if let Err(e) = write_rules_archive(&staging, &archive_path, include_zip_compression()) {
            panic!("build.rs failed to write {}: {}", archive_path.display(), e);
        }
    }
}
