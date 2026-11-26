//! Godot can validate and export a universal macOS bundle on Linux too. Exercise
//! the real exporter so mandatory project flags cannot regress unnoticed.
use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
#[ignore = "requires SPIRE_RELEASE_GODOT and SPIRE_RELEASE_TEMPLATES including macos.zip"]
fn universal_export_requires_astc_and_accepts_showcase_rendering_settings() {
    let godot = std::env::var("SPIRE_RELEASE_GODOT").unwrap();
    let templates = PathBuf::from(std::env::var("SPIRE_RELEASE_TEMPLATES").unwrap());
    let directory = std::env::temp_dir().join(format!(
        "spire universal export {}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(&directory).unwrap();
    let project = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../demo/project.godot")).unwrap();
    fs::write(directory.join("project.godot"), &project).unwrap();
    fs::write(directory.join("main.tscn"), "[gd_scene format=3]\n[node name=\"Main\" type=\"Node\"]\n").unwrap();
    let template = serde_json::to_string(&templates.join("macos.zip").to_string_lossy()).unwrap();
    fs::write(
        directory.join("export_presets.cfg"),
        format!(
            r#"[preset.0]
name="Mac"
platform="macOS"
runnable=true
export_filter="all_resources"
include_filter=""
exclude_filter=""
[preset.0.options]
custom_template/debug={template}
custom_template/release={template}
binary_format/architecture="universal"
application/bundle_identifier="com.houtamelo.spire-export-regression"
codesign/codesign=1
codesign/entitlements/disable_library_validation=true
notarization/notarization=0
"#
        ),
    )
    .unwrap();
    let export = |name: &str| {
        Command::new(&godot)
            .env("XDG_DATA_HOME", directory.join("data"))
            .env("XDG_CONFIG_HOME", directory.join("config"))
            .env("XDG_CACHE_HOME", directory.join("cache"))
            .arg("--headless")
            .arg("--path")
            .arg(&directory)
            .args(["--export-release", "Mac"])
            .arg(directory.join(name))
            .output()
            .unwrap()
    };
    let good = export("Good.zip");
    assert!(
        good.status.success(),
        "{}{}",
        String::from_utf8_lossy(&good.stdout),
        String::from_utf8_lossy(&good.stderr)
    );
    assert!(directory.join("Good.zip").is_file());
    assert!(!String::from_utf8_lossy(&good.stderr).contains("ERROR:"), "{}", String::from_utf8_lossy(&good.stderr));
    fs::write(
        directory.join("project.godot"),
        project.replace(
            "textures/vram_compression/import_etc2_astc=true",
            "textures/vram_compression/import_etc2_astc=false",
        ),
    )
    .unwrap();
    let bad = export("Bad.zip");
    assert!(!bad.status.success());
    assert!(String::from_utf8_lossy(&bad.stderr).contains("ETC2 ASTC texture format is disabled"));
    assert!(!directory.join("Bad.zip").exists());
    fs::remove_dir_all(directory).unwrap();
}
