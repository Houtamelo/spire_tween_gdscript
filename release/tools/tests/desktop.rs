//! Runs against actual CI builds and their full Godot validation reports.
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

fn reports(directory: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            result.extend(reports(&path));
        } else if path.file_name().unwrap() == "validation.json" {
            result.push(path);
        }
    }
    result
}

fn package(staging: &Path, reports: &Path, out: &Path, success: bool) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_spire-release"))
        .arg("package")
        .arg("--staging")
        .arg(staging)
        .arg("--reports")
        .arg(reports)
        .arg("--out")
        .arg(out)
        .output()
        .unwrap();
    let text = format!("{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    assert_eq!(output.status.success(), success, "{text}");
    text
}

#[test]
#[ignore = "requires SPIRE_RELEASE_STAGING and SPIRE_RELEASE_REPORTS from the complete native CI matrix"]
fn desktop_archive_accepts_ad_hoc_macos_and_requires_full_validation() {
    let staging = PathBuf::from(std::env::var("SPIRE_RELEASE_STAGING").unwrap());
    let validation = PathBuf::from(std::env::var("SPIRE_RELEASE_REPORTS").unwrap());
    let macos: Value = serde_json::from_slice(&fs::read(staging.join("macos/build.json")).unwrap()).unwrap();
    assert_eq!(macos["signing"], "ad-hoc", "use the actual ad-hoc macOS build");
    let directory = std::env::temp_dir().join(format!(
        "spire desktop integration {}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    let good = directory.join("good");
    package(&staging, &validation, &good, true);
    let archive = fs::read_dir(&good)
        .unwrap()
        .map(|e| e.unwrap().path())
        .find(|p| p.extension().is_some_and(|s| s == "zip"))
        .unwrap();
    let mut zip = zip::ZipArchive::new(fs::File::open(archive).unwrap()).unwrap();
    let record: Value = serde_json::from_reader(zip.by_name("release.json").unwrap()).unwrap();
    assert_eq!(record["candidate"], false);
    let macos = record["platforms"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["platform"] == "macos")
        .unwrap();
    assert_eq!(macos["signing"], "ad-hoc");
    for profile in ["debug", "release"] {
        zip.by_name(&format!(
            "addons/spire_tween/lib/universal-apple-darwin/{profile}/spire_tween.framework/spire_tween"
        ))
        .unwrap();
    }
    drop(zip);

    // Remove real reports for one Mac/engine pair; never fabricate validation.
    let config: Value = serde_json::from_str(include_str!("../../config.json")).unwrap();
    let engine = format!("{}.stable.", config["godot_versions"][0].as_str().unwrap());
    let incomplete = directory.join("incomplete-reports");
    fs::create_dir_all(&incomplete).unwrap();
    let mut omitted = 0;
    for (index, path) in reports(&validation).into_iter().enumerate() {
        let value: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        if value["platform"] == "macos"
            && value["arch"] == "aarch64"
            && value["engine"].as_str().is_some_and(|s| s.starts_with(&engine))
        {
            omitted += 1;
            continue;
        }
        let destination = incomplete.join(index.to_string()).join("validation.json");
        fs::create_dir_all(destination.parent().unwrap()).unwrap();
        fs::copy(path, destination).unwrap();
    }
    assert!(omitted > 0, "the successful package must have Mac validation evidence");
    let bad = directory.join("bad");
    assert!(package(&staging, &incomplete, &bad, false).contains("missing full validation for macos/aarch64/Godot"));
    assert!(!bad.exists(), "missing validation must prevent archive creation");
    fs::remove_dir_all(directory).unwrap();
}
