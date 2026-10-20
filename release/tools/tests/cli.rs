//! Real subprocess/filesystem checks; no mocked Cargo, Godot, or shell commands.
use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "spire release cli {}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(&path).unwrap();
    path
}
fn fails(args: &[&str], marker: &str) {
    let output = Command::new(env!("CARGO_BIN_EXE_spire-release"))
        .args(args)
        .output()
        .unwrap();
    assert!(!output.status.success(), "unexpected success: {args:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains(marker), "{}", String::from_utf8_lossy(&output.stderr));
}
#[test]
fn rejects_unknown_missing_and_duplicate_arguments() {
    fails(&["seal-signed-macos", "--stage", "unused"], "unknown command");
    fails(&["build", "--platform", "unknown", "--out", "unused"], "unknown platform");
    fails(&["build", "--platform", "linux"], "missing --out");
    fails(&["build", "--platform", "linux", "--out", "unused", "--typo", "true"], "unknown option");
    fails(&["build", "--platform", "linux", "--platform", "macos"], "duplicate option");
}
#[test]
fn refuses_existing_output_and_missing_artifacts() {
    let path = temp();
    let text = path.to_str().unwrap();
    fails(&["build", "--platform", "linux", "--out", text], "output already exists");
    fails(&["verify", "--stage", text], "build.json");
    fails(&["package", "--staging", text, "--reports", text, "--out", text], "build.json");
    fails(
        &[
            "package",
            "--staging",
            text,
            "--reports",
            text,
            "--out",
            text,
            "--platforms",
            "linux",
        ],
        "must include all desktop",
    );
    fails(
        &[
            "package",
            "--staging",
            text,
            "--reports",
            text,
            "--out",
            text,
            "--platforms",
            "linux,linux",
            "--candidate",
        ],
        "duplicated desktop platform",
    );
    fs::remove_dir_all(path).unwrap();
}
#[test]
fn rejects_corrupted_cached_engine_before_extraction() {
    let path = temp();
    let suffix = match std::env::consts::OS {
        "macos" => "macos.universal.zip",
        "windows" => "win64.exe.zip",
        _ => "linux.x86_64.zip",
    };
    fs::write(path.join(format!("Godot_v4.4-stable_{suffix}")), b"corrupted download").unwrap();
    fails(&["download", "--version", "4.4", "--out", path.to_str().unwrap()], "checksum mismatch");
    assert!(!path.join("editor").exists());
    fs::remove_dir_all(path).unwrap();
}

#[test]
fn shell_wrappers_parse_and_fail_before_building_without_required_sdks() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    for name in [
        "build_linux.sh",
        "build_windows_64.sh",
        "build_macos.sh",
        "build_android.sh",
        "build_wasm.sh",
    ] {
        let output = Command::new("bash")
            .arg("-n")
            .arg(repo.join("build_scripts").join(name))
            .output()
            .unwrap();
        assert!(output.status.success(), "{name}: {}", String::from_utf8_lossy(&output.stderr));
    }
    let directory = temp();
    let output = Command::new("bash")
        .arg(repo.join("build_scripts/build_android.sh"))
        .env_remove("ANDROID_NDK_HOME")
        .current_dir(&directory)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Set ANDROID_NDK_HOME"));
    if std::env::consts::OS != "windows" {
        let output = Command::new("bash")
            .arg(repo.join("build_scripts/build_windows_64.sh"))
            .env_remove("XWIN_DIR")
            .current_dir(&directory)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Set XWIN_DIR"));
    }
    if std::env::consts::OS != "macos" {
        let output = Command::new("bash")
            .arg(repo.join("build_scripts/build_macos.sh"))
            .current_dir(&directory)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("macOS host"));
    }
    fs::remove_dir_all(directory).unwrap();
}
