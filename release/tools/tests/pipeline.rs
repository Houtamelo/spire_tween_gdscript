//! Requires the actual built platform libraries and Godot. CI runs these after
//! the full suites; local use is documented in release/README.md.
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{Read, Write},
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

fn assert_template_payload(zip: &mut zip::ZipArchive<fs::File>) {
    let mut names = BTreeSet::new();
    for index in 0..zip.len() {
        assert!(names.insert(zip.by_index(index).unwrap().name().to_owned()), "duplicate archive entry");
    }
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spire_tween_gdscript/addons/spire_tween/templates");
    fn visit(source: &Path, relative: &Path, zip: &mut zip::ZipArchive<fs::File>) {
        for entry in fs::read_dir(source.join(relative)).unwrap() {
            let path = entry.unwrap().path();
            let relative = relative.join(path.file_name().unwrap());
            if path.is_dir() {
                visit(source, &relative, zip);
            } else {
                let name = format!("addons/spire_tween/templates/{}", relative.to_string_lossy().replace('\\', "/"));
                let mut actual = Vec::new();
                zip.by_name(&name).unwrap().read_to_end(&mut actual).unwrap();
                assert_eq!(actual, fs::read(&path).unwrap(), "template bytes differ: {name}");
            }
        }
    }
    visit(&source, Path::new(""), zip);
}

fn archive_without_script(source: &Path, destination: &Path) {
    let mut zip = zip::ZipArchive::new(fs::File::open(source).unwrap()).unwrap();
    let mut payload = BTreeMap::new();
    for index in 0..zip.len() {
        let mut file = zip.by_index(index).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        payload.insert(file.name().to_owned(), (data, file.unix_mode().unwrap_or(0o644)));
    }
    assert!(
        payload
            .remove("addons/spire_tween/templates/common/template.gd")
            .is_some()
    );
    payload.remove("SHA256SUMS");
    // Keep integrity checks valid: the installed runtime must reject the
    // incomplete script dependency rather than relying on a stale checksum.
    let sums = payload
        .iter()
        .map(|(name, (data, _))| format!("{:x}  {name}\n", Sha256::digest(data)))
        .collect::<String>();
    payload.insert("SHA256SUMS".into(), (sums.into_bytes(), 0o644));
    let mut writer = zip::ZipWriter::new(fs::File::create(destination).unwrap());
    for (name, (data, mode)) in payload {
        writer
            .start_file(name, zip::write::SimpleFileOptions::default().unix_permissions(mode))
            .unwrap();
        writer.write_all(&data).unwrap();
    }
    writer.finish().unwrap();
}
#[test]
#[ignore = "requires SPIRE_RELEASE_STAGE and SPIRE_RELEASE_GODOT; runs actual Godot"]
fn verifies_real_artifacts_and_rejects_tampering_missing_files_and_build_debris() {
    let source = PathBuf::from(std::env::var("SPIRE_RELEASE_STAGE").unwrap());
    run(&["verify", "--stage", source.to_str().unwrap()], true);
    let stage = temp();
    clone_stage(&source, &stage);
    let original_record = fs::read(stage.join("build.json")).unwrap();
    let mut metadata: Value = serde_json::from_slice(&original_record).unwrap();
    if metadata["platform"] == "windows" {
        let name = metadata["files"]
            .as_object()
            .unwrap()
            .keys()
            .find(|n| n.starts_with("addon/") && n.ends_with(".dll"))
            .unwrap()
            .clone();
        let path = stage.join(&name);
        let original = fs::read(&path).unwrap();
        let mut corrupted = original.clone();
        let dependency = b"bcryptprimitives.dll\0";
        let mut found = false;
        while let Some(index) = corrupted.windows(dependency.len()).position(|s| s == dependency) {
            corrupted[index..index + dependency.len()].fill(0);
            corrupted[index..index + b"VCRUNTIME140.dll".len()].copy_from_slice(b"VCRUNTIME140.dll");
            found = true;
        }
        assert!(found, "pinned Windows stdlib import changed; update the real-binary failure probe");
        fs::remove_file(&path).unwrap(); // Break the hard link before changing bytes.
        fs::write(&path, &corrupted).unwrap();
        metadata["files"][&name] = Value::String(format!("{:x}", Sha256::digest(&corrupted)));
        fs::write(stage.join("build.json"), serde_json::to_vec(&metadata).unwrap()).unwrap();
        assert!(run(&["verify", "--stage", stage.to_str().unwrap()], false).contains("external Visual C++ runtime"));
        fs::write(&path, original).unwrap();
        fs::write(stage.join("build.json"), original_record).unwrap();
    }
    let manifest = stage.join("addon/spire_tween.gdextension");
    let original = fs::read(&manifest).unwrap();
    fs::write(&manifest, b"tampered").unwrap();
    assert!(run(&["verify", "--stage", stage.to_str().unwrap()], false).contains("hash mismatch"));
    fs::write(&manifest, &original).unwrap();
    fs::write(stage.join("addon/stale.rlib"), b"build debris").unwrap();
    assert!(run(&["verify", "--stage", stage.to_str().unwrap()], false).contains("unrecorded artifact"));
    fs::remove_file(stage.join("addon/stale.rlib")).unwrap();
    fs::remove_file(&manifest).unwrap();
    run(&["verify", "--stage", stage.to_str().unwrap()], false);
    fs::write(&manifest, original).unwrap();
    let missing = "addon/templates/common/template.gd";
    fs::remove_file(stage.join(missing)).unwrap();
    let mut metadata: Value = serde_json::from_slice(&fs::read(stage.join("build.json")).unwrap()).unwrap();
    metadata["files"].as_object_mut().unwrap().remove(missing);
    fs::write(stage.join("build.json"), serde_json::to_vec(&metadata).unwrap()).unwrap();
    assert!(
        run(&["verify", "--stage", stage.to_str().unwrap()], false).contains("missing or mismatched staged template")
    );
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

#[test]
#[ignore = "requires SPIRE_RELEASE_STAGE and SPIRE_RELEASE_GODOT; assembles and runs a real archive"]
fn candidate_archive_installs_and_rejects_extra_files() {
    let source = PathBuf::from(std::env::var("SPIRE_RELEASE_STAGE").unwrap());
    let godot = std::env::var("SPIRE_RELEASE_GODOT").unwrap();
    let metadata: Value = serde_json::from_slice(&fs::read(source.join("build.json")).unwrap()).unwrap();
    let platform = metadata["platform"].as_str().unwrap();
    let directory = temp();
    clone_stage(&source, &directory.join("staging").join(platform));
    let archive_dir = directory.join("archives");
    run(
        &[
            "package",
            "--staging",
            directory.join("staging").to_str().unwrap(),
            "--reports",
            directory.to_str().unwrap(),
            "--out",
            archive_dir.to_str().unwrap(),
            "--platforms",
            platform,
            "--candidate",
        ],
        true,
    );
    let archive = fs::read_dir(&archive_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .find(|p| p.extension().is_some_and(|s| s == "zip"))
        .unwrap();
    {
        let mut zip = zip::ZipArchive::new(fs::File::open(&archive).unwrap()).unwrap();
        assert!(!zip.is_empty());
        for i in 0..zip.len() {
            let file = zip.by_index(i).unwrap();
            assert!(!file.name().ends_with(".rlib") && !file.name().ends_with(".pdb"));
            assert!(!file.name().starts_with("tests/") && !file.name().contains("GirlKnight"));
        }
        zip.by_name("addons/spire_tween/spire_tween.gdextension").unwrap();
        zip.by_name("build/Cargo.lock").unwrap();
        zip.by_name("SHA256SUMS").unwrap();
        assert_template_payload(&mut zip);
    }
    run(
        &[
            "test-archive",
            "--archive",
            archive.to_str().unwrap(),
            "--godot",
            &godot,
            "--out",
            directory.join("good-archive").to_str().unwrap(),
        ],
        true,
    );
    let missing_script = directory.join("missing-script.zip");
    archive_without_script(&archive, &missing_script);
    let bad_script_output = directory.join("missing-script");
    run(
        &[
            "test-archive",
            "--archive",
            missing_script.to_str().unwrap(),
            "--godot",
            &godot,
            "--out",
            bad_script_output.to_str().unwrap(),
        ],
        false,
    );
    assert!(
        fs::read_to_string(bad_script_output.join("smoke-debug.log"))
            .unwrap()
            .contains("SPIRE_RELEASE_SMOKE_FAILED template script cannot load")
    );
    assert!(!bad_script_output.join("archive-validation.json").exists());
    let file = fs::OpenOptions::new().read(true).write(true).open(&archive).unwrap();
    let mut zip = zip::ZipWriter::new_append(file).unwrap();
    zip.start_file("unexpected.rlib", zip::write::SimpleFileOptions::default())
        .unwrap();
    zip.write_all(b"unrecorded build debris").unwrap();
    zip.finish().unwrap();
    let result = run(
        &[
            "test-archive",
            "--archive",
            archive.to_str().unwrap(),
            "--godot",
            &godot,
            "--out",
            directory.join("bad-archive").to_str().unwrap(),
        ],
        false,
    );
    assert!(result.contains("unrecorded archive file"));
    assert!(!directory.join("bad-archive/archive-validation.json").exists());
    fs::remove_dir_all(directory).unwrap();
}
