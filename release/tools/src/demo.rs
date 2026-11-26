use super::*;

pub(super) fn build(stage: &Path, godot: &Path, templates: &Path, out: &Path) -> Result<()> {
    ensure!(!out.exists(), "demo output already exists");
    let record = verify(stage)?;
    let platform = record["platform"].as_str().context("missing platform")?;
    ensure!(platform == std::env::consts::OS, "export demo on its target OS");
    ensure!(source_identity()?["sha256"] == record["source"]["sha256"], "demo checkout differs from artifact source");
    let input = out.join("input");
    copy_tree(&root().join("release/demo"), &input)?;
    copy_tree(
        &root().join("spire_tween_gdscript/examples/effect_templates"),
        &input.join("examples/effect_templates"),
    )?;
    copy(&root().join("release/template_state.gd"), &input.join("template_state.gd"))?;
    let project = out.join("project");
    prepare(&input, &project, &stage.join("addon"), godot, &out.join("prepare"))?;
    let notices = out.join("godot-notices.txt");
    run_logged(
        engine_command(godot, &project)
            .args(["--", "--spire-demo-test"])
            .arg(format!("--spire-demo-notices={}", notices.display())),
        &out.join("project-test.log"),
        60,
        0,
        "SPIRE_DEMO_RESULT total=33 failed=0",
    )?;
    let (export_platform, debug_template, release_template, filename) = match platform {
        "linux" => ("Linux", "linux_debug.x86_64", "linux_release.x86_64", "SpireTweenDemo.x86_64"),
        "windows" => {
            ("Windows Desktop", "windows_debug_x86_64.exe", "windows_release_x86_64.exe", "SpireTweenDemo.exe")
        }
        "macos" => ("macOS", "macos.zip", "macos.zip", "SpireTweenDemo.zip"),
        _ => bail!("showcase exports are desktop-only"),
    };
    for template in [debug_template, release_template] {
        ensure!(templates.join(template).is_file(), "missing export template {template}");
    }
    let preset = format!(
        r#"[preset.0]
name="Showcase"
platform="{export_platform}"
runnable=true
export_filter="all_resources"
include_filter=""
exclude_filter=""
[preset.0.options]
custom_template/debug={}
custom_template/release={}
binary_format/architecture="{}"
application/bundle_identifier="com.houtamelo.spiretween.showcase"
codesign/codesign=1
codesign/entitlements/disable_library_validation=true
notarization/notarization=0
"#,
        serde_json::to_string(&templates.join(debug_template).to_string_lossy())?,
        serde_json::to_string(&templates.join(release_template).to_string_lossy())?,
        if platform == "macos" { "universal" } else { "x86_64" }
    );
    fs::write(project.join("export_presets.cfg"), preset)?;
    let exported = out.join("exported");
    fs::create_dir_all(&exported)?;
    run_logged(
        engine_command(godot, &project)
            .args(["--export-release", "Showcase"])
            .arg(exported.join(filename)),
        &out.join("export.log"),
        180,
        0,
        "Godot Engine",
    )?;
    if platform == "macos" {
        output(
            Command::new("ditto")
                .args(["-x", "-k"])
                .arg(exported.join(filename))
                .arg(&exported),
        )?;
        fs::remove_file(exported.join(filename))?;
    }
    let executable = executable(&exported, platform)?;
    let version: toml::Value = toml::from_str(&fs::read_to_string(root().join("spire_tween/Cargo.toml"))?)?;
    let mut payload = BTreeMap::new();
    for path in files(&exported)? {
        payload.insert(path.strip_prefix(&exported)?.to_string_lossy().replace('\\', "/"), fs::read(path)?);
    }
    for path in files(&stage.join("notices"))? {
        payload.insert(
            format!(
                "third_party/{platform}/{}",
                path.strip_prefix(stage.join("notices"))?
                    .to_string_lossy()
                    .replace('\\', "/")
            ),
            fs::read(path)?,
        );
    }
    payload.insert("third_party/godot.txt".into(), fs::read(notices)?);
    payload.insert("README.md".into(), fs::read(root().join("release/DEMO.md"))?);
    payload.insert("LICENSE".into(), fs::read(root().join("LICENSE"))?);
    payload.insert(
        "demo.json".into(),
        serde_json::to_vec_pretty(&json!({
            "schema":1,"kind":"demo","version":version["package"]["version"].as_str(),"platform":platform,
            "source":record["source"],"engine":output(Command::new(godot).arg("--version"))?.trim(),
            "executable":executable.strip_prefix(&exported)?.to_string_lossy().replace('\\', "/"),
        }))?,
    );
    payload.insert(
        "SHA256SUMS".into(),
        payload
            .iter()
            .map(|(name, data)| format!("{:x}  {name}\n", Sha256::digest(data)))
            .collect::<String>()
            .into_bytes(),
    );
    let archives = out.join("archives");
    fs::create_dir_all(&archives)?;
    let archive = archives.join(format!(
        "SpireTween-{}-{}-{}demo-{platform}.zip",
        version["package"]["version"].as_str().unwrap(),
        &record["source"]["revision"].as_str().unwrap()[..12],
        if record["source"]["dirty"] == true { "candidate-" } else { "" }
    ));
    write_archive(&archive, payload)?;
    test(&archive, &out.join("validation"))?;
    fs::write(
        archives.join("SHA256SUMS"),
        format!("{}  {}\n", hash(&archive)?, archive.file_name().unwrap().to_string_lossy()),
    )?;
    eprintln!("Showcase ready: {}", archive.display());
    Ok(())
}

fn executable(directory: &Path, platform: &str) -> Result<PathBuf> {
    if platform == "macos" {
        files(directory)?
            .into_iter()
            .find(|path| path.parent().is_some_and(|parent| parent.ends_with("Contents/MacOS")))
            .context("demo Mac executable missing")
    } else {
        let executable =
            directory.join(if platform == "windows" { "SpireTweenDemo.exe" } else { "SpireTweenDemo.x86_64" });
        ensure!(executable.is_file(), "demo executable missing");
        Ok(executable)
    }
}

pub(super) fn test(archive: &Path, out: &Path) -> Result<()> {
    let extracted = out.join("extracted");
    extract_archive(archive, &extracted)?;
    let record = read_json(&extracted.join("demo.json"))?;
    ensure!(record["kind"] == "demo" && record["platform"] == std::env::consts::OS, "test demo on its target OS");
    let executable = executable(&extracted, std::env::consts::OS)?;
    ensure!(
        executable
            .strip_prefix(&extracted)?
            .to_string_lossy()
            .replace('\\', "/")
            == record["executable"],
        "demo executable differs from manifest"
    );
    run_logged(
        Command::new(executable).args(["--headless", "--", "--spire-demo-test"]),
        &out.join("demo-run.log"),
        60,
        0,
        "SPIRE_DEMO_RESULT total=33 failed=0",
    )?;
    write_json(
        &out.join("demo-validation.json"),
        &json!({"archive_sha256":hash(archive)?,"platform":record["platform"],"source_sha256":record["source"]["sha256"],"passed":true}),
    )
}
