use super::*;

fn included(name: &str) -> bool {
    [
        "Cargo.toml",
        "Cargo.lock",
        "rust-toolchain.toml",
        "rustfmt.toml",
        "LICENSE",
        "README.md",
        "CHANGELOG.md",
        "tweenable_properties.json",
        ".gitignore",
        "spire_tween/Cargo.toml",
        "spire_tween/README.md",
        "spire_tween/LICENSE",
        "spire_tween_rust/Cargo.toml",
    ]
    .contains(&name)
        || [
            ".cargo/",
            "build_scripts/",
            "gdscript_bindgen/",
            "release/",
            "spire_tween/src/",
            "spire_tween/tests/",
            "spire_tween_rust/src/",
            "spire_tween_gdscript/addons/spire_tween/",
            "spire_tween_gdscript/examples/effect_templates/",
        ]
        .iter()
        .any(|prefix| name.starts_with(prefix))
        || name.starts_with("docs/") && name.ends_with(".md")
}

pub(super) fn package(out: &Path, source: &Value, version: &str, candidate: bool) -> Result<PathBuf> {
    let mut payload = BTreeMap::new();
    for (name, digest) in source["files"].as_object().context("missing source inventory")? {
        if included(name) {
            let data = fs::read(root().join(name)).with_context(|| format!("source file {name}"))?;
            ensure!(format!("{:x}", Sha256::digest(&data)) == digest.as_str().unwrap(), "source changed: {name}");
            payload.insert(name.clone(), data);
        }
    }
    payload.insert("SOURCE_README.md".into(), fs::read(root().join("release/SOURCE.md"))?);
    payload.insert(
        "source.json".into(),
        serde_json::to_vec_pretty(&json!({
            "schema":1, "kind":"source", "version":version, "candidate":candidate, "source":source,
        }))?,
    );
    let sums = payload
        .iter()
        .map(|(name, bytes)| format!("{:x}  {name}\n", Sha256::digest(bytes)))
        .collect::<String>();
    payload.insert("SHA256SUMS".into(), sums.into_bytes());
    let archive = out.join(format!(
        "SpireTween-{version}-{}-{}source.zip",
        &source["revision"].as_str().context("missing source revision")?[..12],
        if candidate { "candidate-" } else { "" }
    ));
    write_archive(&archive, payload)?;
    eprintln!("Packaged {}", archive.display());
    Ok(archive)
}

pub(super) fn test(archive: &Path, out: &Path) -> Result<()> {
    let extracted = out.join("extracted");
    extract_archive(archive, &extracted)?;
    let record = read_json(&extracted.join("source.json"))?;
    ensure!(record["kind"] == "source", "expected a source archive");
    let inventory = record["source"]["files"]
        .as_object()
        .context("missing source inventory")?;
    ensure!(
        format!("{:x}", Sha256::digest(serde_json::to_vec(inventory)?)) == record["source"]["sha256"],
        "source inventory checksum mismatch"
    );
    let mut expected = BTreeSet::new();
    for (name, digest) in inventory {
        if included(name) {
            expected.insert(name.as_str());
            ensure!(
                extracted.join(name).is_file() && hash(&extracted.join(name))? == digest.as_str().unwrap(),
                "missing or mismatched source file: {name}"
            );
        }
    }
    for required in [
        "Cargo.toml",
        "Cargo.lock",
        "rust-toolchain.toml",
        "spire_tween/src/lib.rs",
        "release/SOURCE.md",
    ] {
        ensure!(expected.contains(required), "missing required source inventory: {required}");
    }
    for file in files(&extracted)? {
        let name = file.strip_prefix(&extracted)?.to_string_lossy().replace('\\', "/");
        ensure!(
            expected.contains(name.as_str())
                || ["source.json", "SOURCE_README.md", "SHA256SUMS"].contains(&name.as_str()),
            "unexpected source archive file: {name}"
        );
    }
    let package: toml::Value = toml::from_str(&fs::read_to_string(extracted.join("spire_tween/Cargo.toml"))?)?;
    ensure!(package["package"]["version"].as_str() == record["version"].as_str(), "source version mismatch");
    run_logged(
        Command::new("cargo")
            .args([
                "check",
                "--locked",
                "-p",
                "spire_tween",
                "--features",
                "api-4-4,include_gdext_lib,include_gdscript_bridge",
                "--manifest-path",
            ])
            .arg(extracted.join("Cargo.toml"))
            .arg("--target-dir")
            .arg(root().join("target/source-validation")),
        &out.join("cargo-check.log"),
        900,
        0,
        "Finished",
    )?;
    write_json(
        &out.join("source-validation.json"),
        &json!({"archive_sha256":hash(archive)?, "source_sha256":record["source"]["sha256"], "passed":true}),
    )
}
