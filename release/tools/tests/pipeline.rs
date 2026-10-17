//! Requires the actual built platform libraries and Godot. CI runs these after
//! the full suites; local use is documented in release/README.md.
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;
use sha2::{Digest, Sha256};

fn temp() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "spire release integration {}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(&path).unwrap();
    path
}
fn clone_stage(source: &Path, target: &Path) {
    fs::create_dir_all(target).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let path = entry.unwrap().path();
        let destination = target.join(path.file_name().unwrap());
        if path.is_dir() {
            clone_stage(&path, &destination);
        } else if path
            .extension()
            .is_some_and(|s| s == "so" || s == "dll" || s == "dylib")
            || path.file_name().unwrap() == "spire_tween"
        {
            fs::hard_link(&path, &destination)
                .or_else(|_| fs::copy(&path, &destination).map(|_| ()))
                .unwrap();
        } else {
            fs::copy(&path, destination).unwrap();
        }
    }
}
fn run(args: &[&str], success: bool) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_spire-release"))
        .args(args)
        .output()
        .unwrap();
    let text = format!("{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    assert_eq!(output.status.success(), success, "{args:?}\n{text}");
    text
}
#[test]
#[ignore = "requires SPIRE_RELEASE_STAGE and SPIRE_RELEASE_GODOT; runs actual Godot"]
fn verifies_real_artifacts_and_rejects_tampering_missing_files_and_build_debris() {
    let source = PathBuf::from(std::env::var("SPIRE_RELEASE_STAGE").unwrap());
    run(&["verify", "--stage", source.to_str().unwrap()], true);
    let stage = temp();
    clone_stage(&source, &stage);
    let manifest = stage.join("addon/spire_tween.gdextension");
    let original = fs::read(&manifest).unwrap();
    fs::write(&manifest, b"tampered").unwrap();
    assert!(run(&["verify", "--stage", stage.to_str().unwrap()], false).contains("hash mismatch"));
    fs::write(&manifest, original).unwrap();
    fs::write(stage.join("addon/stale.rlib"), b"build debris").unwrap();
    assert!(run(&["verify", "--stage", stage.to_str().unwrap()], false).contains("unrecorded artifact"));
    fs::remove_file(stage.join("addon/stale.rlib")).unwrap();
    fs::remove_file(manifest).unwrap();
    run(&["verify", "--stage", stage.to_str().unwrap()], false);
    fs::remove_dir_all(stage).unwrap();
}
#[test]
#[ignore = "requires SPIRE_RELEASE_STAGE and SPIRE_RELEASE_GODOT; runs actual Godot"]
fn installed_smoke_passes_and_missing_library_cannot_false_pass() {
    let source = PathBuf::from(std::env::var("SPIRE_RELEASE_STAGE").unwrap());
    let godot = std::env::var("SPIRE_RELEASE_GODOT").unwrap();
    let directory = temp();
    run(
        &[
            "test",
            "--stage",
            source.to_str().unwrap(),
            "--godot",
            &godot,
            "--out",
            directory.join("good").to_str().unwrap(),
            "--smoke-only",
        ],
        true,
    );
    let stage = directory.join("bad-stage");
    clone_stage(&source, &stage);
    let manifest = stage.join("addon/spire_tween.gdextension");
    let text = fs::read_to_string(&manifest)
        .unwrap()
        .replace("./lib/", "./missing-library/");
    fs::write(&manifest, text).unwrap();
    let mut metadata: Value = serde_json::from_slice(&fs::read(stage.join("build.json")).unwrap()).unwrap();
    metadata["files"]["addon/spire_tween.gdextension"] =
        Value::String(format!("{:x}", Sha256::digest(fs::read(manifest).unwrap())));
    fs::write(stage.join("build.json"), serde_json::to_vec(&metadata).unwrap()).unwrap();
    run(
        &[
            "test",
            "--stage",
            stage.to_str().unwrap(),
            "--godot",
            &godot,
            "--out",
            directory.join("bad").to_str().unwrap(),
            "--smoke-only",
        ],
        false,
    );
    assert!(!directory.join("bad/validation.json").exists());
    fs::remove_dir_all(directory).unwrap();
}
