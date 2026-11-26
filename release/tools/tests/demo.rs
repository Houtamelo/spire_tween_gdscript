//! Exercise Play/Restore for every card in the exported app, then remove its
//! real PCK dependency while retaining valid archive checksums.
mod common;
use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
#[ignore = "requires SPIRE_RELEASE_DEMO_ARCHIVE from a real export on this OS"]
fn exported_demo_runs_all_cards_and_rejects_a_missing_pck() {
    let archive = PathBuf::from(std::env::var("SPIRE_RELEASE_DEMO_ARCHIVE").unwrap());
    let directory = std::env::temp_dir().join(format!(
        "spire demo integration {}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(&directory).unwrap();
    let run = |archive: &std::path::Path, output: &std::path::Path| {
        Command::new(env!("CARGO_BIN_EXE_spire-release"))
            .arg("test-demo")
            .arg("--archive")
            .arg(archive)
            .arg("--out")
            .arg(output)
            .output()
            .unwrap()
    };
    let result = run(&archive, &directory.join("good"));
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert!(directory.join("good/demo-validation.json").is_file());
    let mut zip = zip::ZipArchive::new(fs::File::open(&archive).unwrap()).unwrap();
    let pck = (0..zip.len())
        .map(|index| zip.by_index(index).unwrap().name().to_owned())
        .find(|name| name.ends_with(".pck"))
        .unwrap();
    drop(zip);
    let missing = directory.join("missing-pck.zip");
    common::without_file(&archive, &missing, &pck);
    let result = run(&missing, &directory.join("bad"));
    assert!(!result.status.success());
    assert!(!directory.join("bad/demo-validation.json").exists());
    fs::remove_dir_all(directory).unwrap();
}
