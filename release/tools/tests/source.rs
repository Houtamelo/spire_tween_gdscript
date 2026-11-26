//! Compile the delivered source, then reject an incomplete snapshot even when
//! its zip checksums were correctly regenerated.
mod common;
use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
#[ignore = "requires SPIRE_RELEASE_SOURCE_ARCHIVE from a real package"]
fn source_archive_compiles_and_rejects_a_missing_core_file() {
    let archive = PathBuf::from(std::env::var("SPIRE_RELEASE_SOURCE_ARCHIVE").unwrap());
    let directory = std::env::temp_dir().join(format!(
        "spire source integration {}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(&directory).unwrap();
    let run = |archive: &std::path::Path, output: &std::path::Path| {
        Command::new(env!("CARGO_BIN_EXE_spire-release"))
            .arg("test-source")
            .arg("--archive")
            .arg(archive)
            .arg("--out")
            .arg(output)
            .output()
            .unwrap()
    };
    let result = run(&archive, &directory.join("good"));
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert!(directory.join("good/source-validation.json").is_file());
    let missing = directory.join("missing-core.zip");
    common::without_file(&archive, &missing, "spire_tween/src/lib.rs");
    let result = run(&missing, &directory.join("bad"));
    assert!(!result.status.success());
    assert!(
        String::from_utf8_lossy(&result.stderr).contains("missing or mismatched source file: spire_tween/src/lib.rs")
    );
    assert!(!directory.join("bad/source-validation.json").exists());
    fs::remove_dir_all(directory).unwrap();
}
