//! Release automation. CI uses this binary and shell; no Python runtime is required.
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail, ensure};
use regex::Regex;
use serde_json::{Value, json};
use sha2::{Digest, Sha256, Sha512};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn config() -> Value { serde_json::from_str(include_str!("../../config.json")).unwrap() }
fn hash(path: &Path) -> Result<String> { Ok(format!("{:x}", Sha256::digest(fs::read(path)?))) }
fn write_json(path: &Path, value: &Value) -> Result<()> {
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, format!("{}\n", serde_json::to_string_pretty(value)?))?;
    Ok(())
}
fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path).with_context(|| path.display().to_string())?)?)
}
fn output(command: &mut Command) -> Result<String> {
    let result = command.output().with_context(|| format!("running {command:?}"))?;
    ensure!(result.status.success(), "{command:?}: {}", String::from_utf8_lossy(&result.stderr));
    Ok(String::from_utf8(result.stdout)?)
}
fn files(directory: &Path) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    if directory.is_dir() {
        for entry in fs::read_dir(directory)? {
            let path = entry?.path();
            ensure!(!path.is_symlink(), "unexpected symlink: {}", path.display());
            if path.is_dir() {
                result.extend(files(&path)?);
            } else {
                result.push(path);
            }
        }
    }
    result.sort();
    Ok(result)
}
fn copy(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination.parent().unwrap())?;
    fs::copy(source, destination)?;
    Ok(())
}
fn copy_tree(source: &Path, destination: &Path) -> Result<()> {
    for path in files(source)? {
        copy(&path, &destination.join(path.strip_prefix(source)?))?;
    }
    Ok(())
}
fn source_identity() -> Result<Value> {
    let repo = root();
    let paths = output(Command::new("git").current_dir(&repo).args([
        "ls-files",
        "-z",
        "--cached",
        "--others",
        "--exclude-standard",
    ]))?;
    let mut sources = BTreeMap::new();
    for name in paths.split('\0').filter(|s| !s.is_empty()) {
        if name.starts_with("release_audit") || name.contains("/addons/spire_tween/lib/") {
            continue;
        }
        let path = repo.join(name);
        if path.is_file() && !path.is_symlink() {
            sources.insert(name.to_owned(), hash(&path)?);
        }
    }
    let digest = format!("{:x}", Sha256::digest(serde_json::to_vec(&sources)?));
    Ok(json!({"revision": output(Command::new("git").current_dir(&repo).args(["rev-parse", "HEAD"]))?.trim(),
        "dirty": !output(Command::new("git").current_dir(&repo).args(["-c", "core.safecrlf=false", "status", "--porcelain"]))?.is_empty(),
        "sha256": digest, "files": sources}))
}
fn library(crate_name: &str, platform: &str) -> String {
    match platform {
        "windows" => format!("{crate_name}.dll"),
        "macos" => format!("lib{crate_name}.dylib"),
        _ => format!("lib{crate_name}.so"),
    }
}
fn manifest(platforms: &[String], native: bool) -> String {
    let cfg = config();
    let mut text = format!(
        r#"[configuration]
entry_symbol="gdext_rust_init"
compatibility_minimum="{}"
reloadable=false

[libraries]
"#,
        cfg["godot_minimum"].as_str().unwrap()
    );
    for platform in platforms {
        for target in cfg["platforms"][platform].as_array().unwrap() {
            let target = target.as_str().unwrap();
            for profile in ["debug", "release"] {
                let arch = if target.starts_with("aarch64") { "arm64" } else { "x86_64" };
                let name = library(if native { "spire_tween_rust" } else { "spire_tween" }, platform);
                if platform == "macos" && !native {
                    if arch == "x86_64" {
                        continue;
                    }
                    text.push_str(&format!(
                        "macos.{profile}=\"./lib/universal-apple-darwin/{profile}/spire_tween.framework\"\n"
                    ));
                } else {
                    text.push_str(&format!("{platform}.{profile}.{arch}=\"./lib/{target}/{profile}/{name}\"\n"));
                }
            }
        }
    }
    text
}
fn verify_macos_architectures(binary: &Path) -> Result<()> {
    // Apple's lipo consumes every argument after -verify_arch as an architecture.
    output(
        Command::new("lipo")
            .arg(binary)
            .args(["-verify_arch", "arm64", "x86_64"]),
    )?;
    Ok(())
}

#[cfg(all(test, target_os = "macos"))]
mod macos_integration_tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    fn universal_verification_accepts_both_architectures_and_rejects_thin_objects() {
        let directory = std::env::temp_dir().join(format!(
            "spire macos tooling {}-{}",
            std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&directory).unwrap();
        let source = directory.join("probe.c");
        fs::write(&source, "int spire_tooling_probe(void) { return 0; }\n").unwrap();
        let arm = directory.join("arm64.o");
        let intel = directory.join("x86_64.o");
        for (target, object) in [("arm64-apple-macos11", &arm), ("x86_64-apple-macos11", &intel)] {
            output(
                Command::new("clang")
                    .args(["-target", target, "-c"])
                    .arg(&source)
                    .arg("-o")
                    .arg(object),
            )
            .unwrap();
        }
        let universal = directory.join("universal.o");
        output(
            Command::new("lipo")
                .args(["-create", "-output"])
                .arg(&universal)
                .arg(&arm)
                .arg(&intel),
        )
        .unwrap();
        verify_macos_architectures(&universal).unwrap();
        assert!(verify_macos_architectures(&arm).is_err());
        assert!(verify_macos_architectures(&intel).is_err());
        fs::remove_dir_all(directory).unwrap();
    }
}

fn cargo_build(
    target: &str,
    profile: &str,
    native: bool,
    directory: &Path,
    log: &Path,
    android: bool,
) -> Result<BTreeSet<String>> {
    let mut command = Command::new("cargo");
    if android {
        command.args(["ndk", "-t", target]);
    }
    command.current_dir(root()).args([
        "build",
        "--locked",
        "-p",
        if native { "spire_tween_rust" } else { "spire_tween" },
        "--target",
        target,
        "--message-format=json-render-diagnostics",
        "--features",
        if native {
            "api-4-4,spire_tween/include_gdscript_bridge"
        } else {
            "api-4-4,include_gdext_lib,include_gdscript_bridge"
        },
    ]);
    if profile == "release" {
        command.arg("--release");
    }
    command.env("CARGO_TARGET_DIR", directory);
    if target.ends_with("windows-msvc") {
        // Keep the extension self-contained: do not require a separately
        // installed Visual C++ runtime. Preserve encoded SDK paths with spaces.
        if let Ok(flags) = std::env::var("CARGO_ENCODED_RUSTFLAGS") {
            let prefix = if flags.is_empty() { flags } else { format!("{flags}\x1f") };
            command.env("CARGO_ENCODED_RUSTFLAGS", format!("{prefix}-C\x1ftarget-feature=+crt-static"));
        } else {
            command.env(
                "RUSTFLAGS",
                format!("{} -C target-feature=+crt-static", std::env::var("RUSTFLAGS").unwrap_or_default()),
            );
        }
    }
    if target.ends_with("apple-darwin") {
        command.env("MACOSX_DEPLOYMENT_TARGET", config()["macos_deployment_target"].as_str().unwrap());
    }
    eprintln!("Building {} {target} {profile}", if native { "native tests" } else { "addon" });
    let result = command.output()?;
    fs::create_dir_all(log.parent().unwrap())?;
    fs::write(log, [&result.stdout[..], &result.stderr[..]].concat())?;
    ensure!(
        result.status.success(),
        "build failed; see {}: {}",
        log.display(),
        String::from_utf8_lossy(&result.stderr)
    );
    let mut packages = BTreeSet::new();
    for line in String::from_utf8(result.stdout)?.lines() {
        if let Ok(message) = serde_json::from_str::<Value>(line) {
            if message["reason"] == "compiler-artifact" {
                packages.insert(message["package_id"].as_str().unwrap().to_owned());
            }
        }
    }
    Ok(packages)
}
fn notices(out: &Path, packages: &BTreeSet<String>) -> Result<()> {
    let sysroot = output(Command::new("rustc").args(["--print", "sysroot"]))?;
    let rust_notices = Path::new(sysroot.trim()).join("share/doc/rust");
    copy(&rust_notices.join("COPYRIGHT-library.html"), &out.join("rust/COPYRIGHT-library.html"))?;
    copy_tree(&rust_notices.join("licenses"), &out.join("rust/licenses"))?;
    let metadata: Value = serde_json::from_str(&output(Command::new("cargo").current_dir(root()).args([
        "metadata",
        "--locked",
        "--format-version",
        "1",
    ]))?)?;
    let mut inventory = Vec::new();
    let mut text = String::from(
        "# Third-party build dependency notices\n\nInventory of crates compiled for this platform's addon, including \
         build dependencies. Native test-only crates are excluded. Original license files follow; unmodified MPL-2.0 \
         crate sources are included in `sources/`. Crate archives are also available at the recorded URLs.\n\n",
    );
    for package in metadata["packages"].as_array().unwrap() {
        let id = package["id"].as_str().unwrap();
        if !packages.contains(id) || package["name"] == "spire_tween" {
            continue;
        }
        let name = package["name"].as_str().unwrap();
        let version = package["version"].as_str().unwrap();
        let label = format!("{name}-{version}");
        let source = Path::new(package["manifest_path"].as_str().unwrap()).parent().unwrap();
        let license = package["license"]
            .as_str()
            .context("dependency has no declared license")?;
        let url = format!("https://crates.io/api/v1/crates/{name}/{version}/download");
        text.push_str(&format!("- **{label}**: {license}. [Source archive]({url}).\n"));
        for entry in fs::read_dir(source)? {
            let path = entry?.path();
            let upper = path.file_name().unwrap().to_string_lossy().to_ascii_uppercase();
            if path.is_file()
                && ["LICENSE", "LICENCE", "COPYING", "NOTICE"]
                    .iter()
                    .any(|prefix| upper.starts_with(prefix))
            {
                copy(&path, &out.join("licenses").join(&label).join(path.file_name().unwrap()))?;
            }
        }
        if license.contains("MPL-2.0") {
            copy_tree(source, &out.join("sources").join(&label))?;
        }
        inventory.push(json!({"name": name, "version": version, "license": license, "source": url}));
    }
    fs::create_dir_all(out)?;
    fs::write(out.join("THIRD_PARTY_NOTICES.md"), text)?;
    write_json(&out.join("inventory.json"), &json!(inventory))
}
fn build(platform: &str, stage: &Path) -> Result<()> {
    let cfg = config();
    let targets = cfg["platforms"][platform].as_array().context("unknown platform")?;
    ensure!(!stage.exists(), "output already exists; choose an empty staging path: {}", stage.display());
    if platform == "macos" {
        ensure!(std::env::consts::OS == "macos", "macOS builds require Xcode on a macOS host");
    }
    if platform == "android" {
        let ndk =
            std::env::var("ANDROID_NDK_HOME").context("Set ANDROID_NDK_HOME to the installed NDK version directory")?;
        ensure!(
            Path::new(&ndk).join("toolchains/llvm/prebuilt").is_dir(),
            "ANDROID_NDK_HOME does not contain an NDK toolchain"
        );
        ensure!(
            output(Command::new("cargo").args(["ndk", "--version"]))?.trim()
                == format!("cargo-ndk {}", cfg["cargo_ndk"].as_str().unwrap()),
            "cargo-ndk version differs from release/config.json"
        );
    }
    fs::create_dir_all(stage)?;
    let source = source_identity()?;
    let target_dir = root().join("target/release-pipeline");
    let mut packages = BTreeSet::new();
    for target in targets {
        let target = target.as_str().unwrap();
        for profile in ["debug", "release"] {
            for native in [false, true].into_iter().filter(|n| !*n || platform != "android") {
                let crate_name = if native { "spire_tween_rust" } else { "spire_tween" };
                let compiled = cargo_build(
                    target,
                    profile,
                    native,
                    &target_dir,
                    &stage.join(format!("logs/{target}-{profile}-{crate_name}.log")),
                    platform == "android",
                )?;
                if !native {
                    packages.extend(compiled);
                }
                let name = library(crate_name, platform);
                copy(
                    &target_dir.join(target).join(profile).join(&name),
                    &stage
                        .join(if native { "tests/lib" } else { "addon/lib" })
                        .join(target)
                        .join(profile)
                        .join(&name),
                )?;
                if platform == "linux" {
                    output(
                        Command::new("strip").arg("--strip-debug").arg(
                            stage
                                .join(if native { "tests/lib" } else { "addon/lib" })
                                .join(target)
                                .join(profile)
                                .join(&name),
                        ),
                    )?;
                } else if platform == "macos" {
                    output(
                        Command::new("strip").arg("-S").arg(
                            stage
                                .join(if native { "tests/lib" } else { "addon/lib" })
                                .join(target)
                                .join(profile)
                                .join(&name),
                        ),
                    )?;
                }
            }
        }
    }
    if platform == "macos" {
        let version: toml::Value = toml::from_str(&fs::read_to_string(root().join("spire_tween/Cargo.toml"))?)?;
        for profile in ["debug", "release"] {
            let framework = stage.join(format!("addon/lib/universal-apple-darwin/{profile}/spire_tween.framework"));
            fs::create_dir_all(framework.join("Resources"))?;
            output(
                Command::new("lipo")
                    .args(["-create", "-output"])
                    .arg(framework.join("spire_tween"))
                    .arg(stage.join(format!("addon/lib/aarch64-apple-darwin/{profile}/libspire_tween.dylib")))
                    .arg(stage.join(format!("addon/lib/x86_64-apple-darwin/{profile}/libspire_tween.dylib"))),
            )?;
            verify_macos_architectures(&framework.join("spire_tween"))?;
            let plist = format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?><plist \
                 version=\"1.0\"><dict><key>CFBundleExecutable</key><string>spire_tween</\
                 string><key>CFBundleIdentifier</key><string>com.houtamelo.spire-tween</\
                 string><key>CFBundlePackageType</key><string>FMWK</string><key>CFBundleVersion</key><string>{}</\
                 string><key>CFBundleShortVersionString</key><string>{}</string><key>LSMinimumSystemVersion</\
                 key><string>{}</string></dict></plist>",
                version["package"]["version"].as_str().unwrap(),
                version["package"]["version"].as_str().unwrap(),
                cfg["macos_deployment_target"].as_str().unwrap()
            );
            fs::write(framework.join("Resources/Info.plist"), plist)?;
            output(
                Command::new("codesign")
                    .args(["--force", "--sign", "-"])
                    .arg(&framework),
            )?;
        }
        for target in targets {
            fs::remove_dir_all(stage.join("addon/lib").join(target.as_str().unwrap()))?;
        }
    }
    fs::write(stage.join("addon/spire_tween.gdextension"), manifest(&[platform.into()], false))?;
    copy(&root().join("LICENSE"), &stage.join("addon/LICENSE"))?;
    notices(&stage.join("notices"), &packages)?;
    ensure!(
        source_identity()?["sha256"] == source["sha256"],
        "source changed while building; discard this staging directory"
    );
    let mut record = json!({"schema": 1, "platform": platform, "api": "4.4", "source": source,
        "rustc": output(Command::new("rustc").arg("--version"))?.trim(), "cargo": output(Command::new("cargo").arg("--version"))?.trim(),
        "host": std::env::consts::OS, "host_arch": std::env::consts::ARCH, "crt_static": platform == "windows",
        "signing": if platform == "macos" { "ad-hoc" } else { "not-applicable" }});
    record_environment(stage, platform, &mut record)?;
    seal(stage, &mut record)?;
    eprintln!("Build staged at {}", stage.display());
    Ok(())
}
fn seal(stage: &Path, record: &mut Value) -> Result<()> {
    let mut hashes = BTreeMap::new();
    for directory in ["addon", "tests", "notices"] {
        for path in files(&stage.join(directory))? {
            hashes.insert(path.strip_prefix(stage)?.to_string_lossy().replace('\\', "/"), hash(&path)?);
        }
    }
    record["files"] = json!(hashes);
    write_json(&stage.join("build.json"), record)
}
fn verify(stage: &Path) -> Result<Value> {
    let record = read_json(&stage.join("build.json"))?;
    ensure!(record["schema"] == 1 && record["api"] == "4.4", "unsupported artifact metadata");
    for (name, expected) in record["files"].as_object().context("missing artifact hashes")? {
        ensure!(
            Path::new(name)
                .components()
                .all(|c| matches!(c, std::path::Component::Normal(_))),
            "unsafe artifact path"
        );
        ensure!(hash(&stage.join(name))? == expected.as_str().unwrap(), "artifact hash mismatch: {name}");
    }
    for directory in ["addon", "tests", "notices"] {
        for path in files(&stage.join(directory))? {
            let name = path.strip_prefix(stage)?.to_string_lossy().replace('\\', "/");
            ensure!(record["files"].get(&name).is_some(), "unrecorded artifact: {name}");
        }
    }
    if record["platform"] == "windows" {
        for directory in ["addon", "tests"] {
            for path in files(&stage.join(directory))?
                .into_iter()
                .filter(|p| p.extension().is_some_and(|e| e == "dll"))
            {
                for name in pe_imports(&fs::read(&path)?)? {
                    let lower = name.to_ascii_lowercase();
                    ensure!(
                        !lower.starts_with("vcruntime") && !lower.starts_with("msvcp"),
                        "Windows artifact depends on external Visual C++ runtime: {name}"
                    );
                }
            }
        }
    }
    Ok(record)
}
fn pe_imports(data: &[u8]) -> Result<Vec<String>> {
    let word = |offset: usize| -> Result<u16> {
        Ok(u16::from_le_bytes(
            data.get(offset..offset + 2)
                .context("truncated PE header")?
                .try_into()?,
        ))
    };
    let dword = |offset: usize| -> Result<u32> {
        Ok(u32::from_le_bytes(
            data.get(offset..offset + 4)
                .context("truncated PE header")?
                .try_into()?,
        ))
    };
    ensure!(data.starts_with(b"MZ"), "not a Windows DLL");
    let pe = dword(0x3c)? as usize;
    ensure!(data.get(pe..pe + 4) == Some(b"PE\0\0") && word(pe + 4)? == 0x8664, "not an x86_64 PE binary");
    let optional = pe + 24;
    ensure!(word(optional)? == 0x20b, "not a PE32+ binary");
    let sections = word(pe + 6)? as usize;
    let table = optional + word(pe + 20)? as usize;
    let offset = |rva: u32| -> Result<usize> {
        for section in 0..sections {
            let header = table + section * 40;
            let start = dword(header + 12)?;
            let size = dword(header + 8)?.max(dword(header + 16)?);
            if rva >= start && rva - start < size {
                return Ok((dword(header + 20)? + rva - start) as usize);
            }
        }
        bail!("unmapped PE import address")
    };
    let import = dword(optional + 112 + 8)?;
    if import == 0 {
        return Ok(Vec::new());
    }
    let mut entry = offset(import)?;
    let mut names = Vec::new();
    for _ in 0..4096 {
        if data
            .get(entry..entry + 20)
            .context("truncated PE imports")?
            .iter()
            .all(|x| *x == 0)
        {
            return Ok(names);
        }
        let name = offset(dword(entry + 12)?)?;
        let rest = data.get(name..).context("truncated PE import name")?;
        let end = rest
            .iter()
            .position(|x| *x == 0)
            .context("unterminated PE import name")?;
        names.push(std::str::from_utf8(&rest[..end])?.to_owned());
        entry += 20;
    }
    bail!("too many PE imports")
}
fn run_logged(command: &mut Command, log: &Path, seconds: u64, expected: i32, marker: &str) -> Result<String> {
    fs::create_dir_all(log.parent().unwrap())?;
    let userdata = log.parent().unwrap().join("userdata");
    fs::create_dir_all(&userdata)?;
    if std::env::consts::OS == "linux" {
        command.env("XDG_DATA_HOME", &userdata);
        command.env("XDG_CONFIG_HOME", log.parent().unwrap().join("config"));
        command.env("XDG_CACHE_HOME", log.parent().unwrap().join("cache"));
    }
    if std::env::consts::OS == "windows" {
        command.env("APPDATA", &userdata);
    }
    let stream = fs::File::create(log)?;
    let mut process = command
        .stdout(stream.try_clone()?)
        .stderr(Stdio::from(stream))
        .spawn()?;
    let start = Instant::now();
    let status = loop {
        if let Some(status) = process.try_wait()? {
            break status;
        }
        if start.elapsed() > Duration::from_secs(seconds) {
            // The Windows console launcher starts the actual Godot executable.
            // Kill the child tree too, otherwise failed fixtures keep running
            // and hold files open after their launcher has been terminated.
            #[cfg(target_os = "windows")]
            output(Command::new("taskkill").args(["/PID", &process.id().to_string(), "/T", "/F"]))?;
            #[cfg(not(target_os = "windows"))]
            process.kill()?;
            process.wait()?;
            bail!("deadline exceeded: {}", log.display());
        }
        std::thread::sleep(Duration::from_millis(100));
    };
    let text = fs::read_to_string(log)?;
    ensure!(
        status.code() == Some(expected) && text.contains(marker),
        "unexpected result {:?}, expected {expected} and {marker}; see {}",
        status.code(),
        log.display()
    );
    Ok(text)
}
fn record_environment(stage: &Path, platform: &str, record: &mut Value) -> Result<()> {
    let cfg = config();
    record["rustc_verbose"] = json!(output(Command::new("rustc").arg("--version").arg("--verbose"))?);
    record["runner_image"] = json!({"os":std::env::var("ImageOS").ok(), "version":std::env::var("ImageVersion").ok()});
    if platform == "linux" {
        let mut versions = BTreeSet::new();
        let regex = Regex::new(r"GLIBC_(\d+)\.(\d+)")?;
        for path in files(&stage.join("addon"))?
            .into_iter()
            .filter(|p| p.extension().is_some_and(|x| x == "so"))
        {
            let report = output(Command::new("readelf").arg("--version-info").arg(&path))?;
            for found in regex.captures_iter(&report) {
                versions.insert((found[1].parse::<u32>()?, found[2].parse::<u32>()?));
            }
        }
        let (major, minor) = versions.last().context("no GLIBC version requirements found")?;
        record["glibc_minimum"] = json!(format!("{major}.{minor}"));
        record["c_compiler"] = json!(output(Command::new("cc").arg("--version"))?);
    } else if platform == "macos" {
        record["xcode"] = json!(output(Command::new("xcodebuild").arg("-version"))?);
        record["sdk"] = json!(output(Command::new("xcrun").args(["--sdk", "macosx", "--show-sdk-version"]))?);
        record["macos_minimum"] = cfg["macos_deployment_target"].clone();
    } else if platform == "android" {
        record["ndk"] =
            json!(fs::read_to_string(Path::new(&std::env::var("ANDROID_NDK_HOME")?).join("source.properties"))?);
        let expected = format!("Pkg.Revision = {}", cfg["android_ndk"].as_str().unwrap());
        ensure!(
            record["ndk"]
                .as_str()
                .unwrap()
                .lines()
                .any(|line| line.trim() == expected),
            "NDK revision differs from release/config.json"
        );
    }
    Ok(())
}
fn prepare(source: &Path, project: &Path, addon: &Path, godot: &Path, logs: &Path) -> Result<()> {
    // Import assets in an extension-free project. Never cold-import the addon:
    // Godot's upstream headless GDExtension importer can crash on that path.
    fs::create_dir_all(project)?;
    fn visit(source: &Path, project: &Path, relative: &Path) -> Result<()> {
        for entry in fs::read_dir(source.join(relative))? {
            let path = entry?.path();
            let name = path.file_name().unwrap();
            if [
                "addons",
                "godot",
                ".godot",
                "src",
                "target",
                "results.json",
                "Cargo.lock",
                "Cargo.toml",
            ]
            .iter()
            .any(|x| name == *x)
            {
                continue;
            }
            if path.is_symlink() {
                continue;
            }
            let next = relative.join(name);
            if path.is_dir() {
                visit(source, project, &next)?;
            } else {
                copy(&path, &project.join(next))?;
            }
        }
        Ok(())
    }
    visit(source, project, Path::new(""))?;
    let assets = project.with_extension("assets");
    fs::create_dir_all(&assets)?;
    let asset_extensions = ["png", "svg", "jpg", "jpeg", "webp", "ttf", "otf", "wav", "ogg"];
    for path in files(project)? {
        if asset_extensions.contains(&path.extension().and_then(|s| s.to_str()).unwrap_or("")) {
            copy(&path, &assets.join(path.strip_prefix(project)?))?;
        }
    }
    fs::write(
        assets.join("project.godot"),
        r#"config_version=5
[application]
config/use_hidden_project_data_directory=false
[rendering]
renderer/rendering_method="gl_compatibility"
"#,
    )?;
    run_logged(
        Command::new(godot)
            .args(["--headless", "--path"])
            .arg(&assets)
            .args(["--editor", "--import"]),
        &logs.join("asset-import.log"),
        120,
        0,
        "Godot Engine",
    )?;
    copy_tree(&assets.join("godot/imported"), &project.join("godot/imported"))?;
    for path in files(&assets)? {
        if path.extension().is_some_and(|e| e == "import") {
            copy(&path, &project.join(path.strip_prefix(&assets)?))?;
        }
    }
    let mut uids = BTreeMap::new();
    let header = Regex::new(r#"uid="(uid://[^"]+)""#)?;
    for path in files(project)? {
        let ext = path.extension().and_then(|x| x.to_str()).unwrap_or("");
        if ext == "uid" {
            uids.insert(
                fs::read_to_string(&path)?.trim().to_owned(),
                format!(
                    "res://{}",
                    path.with_extension("")
                        .strip_prefix(project)?
                        .to_string_lossy()
                        .replace('\\', "/")
                ),
            );
        }
        if ["tscn", "tres", "import"].contains(&ext) {
            let text = fs::read_to_string(&path)?;
            if let Some(found) = header.captures(&text) {
                let relative = if ext == "import" { path.with_extension("") } else { path.clone() };
                uids.insert(
                    found[1].to_owned(),
                    format!("res://{}", relative.strip_prefix(project)?.to_string_lossy().replace('\\', "/")),
                );
            }
        }
    }
    let class = Regex::new(r"(?m)^class_name\s+(\w+)")?;
    let extends = Regex::new(r"(?m)^extends\s+(\w+)")?;
    let uid_attribute = Regex::new(r#" uid="uid://[^"]+""#)?;
    let mut classes = Vec::new();
    for path in files(project)? {
        let ext = path.extension().and_then(|x| x.to_str()).unwrap_or("");
        if !["gd", "tscn", "tres", "godot"].contains(&ext) {
            continue;
        }
        let mut text = fs::read_to_string(&path)?;
        if ext == "gd" {
            if let Some(name) = class.captures(&text) {
                let base = extends
                    .captures(&text)
                    .map(|x| x[1].to_owned())
                    .unwrap_or_else(|| "RefCounted".into());
                classes.push(format!(
                    "{{\"base\": &\"{base}\", \"class\": &\"{}\", \"icon\": \"\", \"is_abstract\": false, \
                     \"is_tool\": {}, \"language\": &\"GDScript\", \"path\": \"res://{}\"}}",
                    &name[1],
                    text.contains("@tool"),
                    path.strip_prefix(project)?.to_string_lossy().replace('\\', "/")
                ));
            }
        }
        text = uid_attribute.replace_all(&text, "").into_owned();
        for (uid, resource) in &uids {
            text = text.replace(&format!("\"{uid}\""), &format!("\"{resource}\""));
        }
        fs::write(path, text)?;
    }
    fs::create_dir_all(project.join("godot"))?;
    fs::write(project.join("godot/global_script_class_cache.cfg"), format!("list=[{}]\n", classes.join(",\n")))?;
    fs::write(project.join("godot/extension_list.cfg"), "res://addons/spire_tween/spire_tween.gdextension\n")?;
    copy_tree(addon, &project.join("addons/spire_tween"))?;
    Ok(())
}
fn engine_command(godot: &Path, project: &Path) -> Command {
    let mut command = Command::new(godot);
    command.args(["--headless", "--path"]).arg(project);
    command
}
fn smoke(stage: &Path, godot: &Path, out: &Path, profile: &str) -> Result<PathBuf> {
    let project = out.join(format!("smoke-{profile}"));
    fs::create_dir_all(project.join("godot"))?;
    copy_tree(&stage.join("addon"), &project.join("addons/spire_tween"))?;
    select_profile(&project.join("addons/spire_tween/spire_tween.gdextension"), profile)?;
    fs::write(
        project.join("project.godot"),
        r#"config_version=5
[application]
config/name="SpireTween release smoke"
run/main_scene="res://smoke.tscn"
config/use_hidden_project_data_directory=false
[rendering]
renderer/rendering_method="gl_compatibility"
"#,
    )?;
    fs::write(project.join("godot/extension_list.cfg"), "res://addons/spire_tween/spire_tween.gdextension\n")?;
    copy(&root().join("release/smoke.gd"), &project.join("smoke.gd"))?;
    copy(&root().join("release/smoke.tscn"), &project.join("smoke.tscn"))?;
    run_logged(
        &mut engine_command(godot, &project),
        &out.join(format!("smoke-{profile}.log")),
        30,
        0,
        "SPIRE_RELEASE_SMOKE_OK",
    )?;
    Ok(project)
}
fn export_smoke(stage: &Path, godot: &Path, out: &Path, templates: &Path) -> Result<()> {
    let platform = std::env::consts::OS;
    let project = smoke(stage, godot, &out.join("export-project"), "debug")?;
    // Restore the real manifest so Godot's exporter, rather than a rewritten
    // editor mapping, chooses debug and release libraries.
    copy(&stage.join("addon/spire_tween.gdextension"), &project.join("addons/spire_tween/spire_tween.gdextension"))?;
    let (name, debug, release) = match platform {
        "linux" => ("Linux", "linux_debug.x86_64", "linux_release.x86_64"),
        "windows" => ("Windows Desktop", "windows_debug_x86_64.exe", "windows_release_x86_64.exe"),
        "macos" => ("macOS", "macos.zip", "macos.zip"),
        _ => bail!("export smoke is desktop-only"),
    };
    for file in [debug, release] {
        ensure!(templates.join(file).is_file(), "missing export template {file}");
    }
    let preset = format!(
        r#"[preset.0]
name="Smoke"
platform="{name}"
runnable=true
export_filter="all_resources"
include_filter=""
exclude_filter=""
[preset.0.options]
custom_template/debug={}
custom_template/release={}
binary_format/architecture="{}"
application/bundle_identifier="com.houtamelo.spire-release-smoke"
codesign/codesign=1
codesign/entitlements/disable_library_validation=true
notarization/notarization=0
"#,
        serde_json::to_string(&templates.join(debug).to_string_lossy())?,
        serde_json::to_string(&templates.join(release).to_string_lossy())?,
        if platform == "macos" { "universal" } else { "x86_64" }
    );
    fs::write(project.join("export_presets.cfg"), preset)?;
    for profile in ["debug", "release"] {
        let directory = out.join(format!("export-{profile}"));
        fs::create_dir_all(&directory)?;
        let path = directory.join(if platform == "windows" {
            "Smoke.exe"
        } else if platform == "macos" {
            "Smoke.zip"
        } else {
            "Smoke.x86_64"
        });
        run_logged(
            engine_command(godot, &project)
                .arg(format!("--export-{profile}"))
                .arg("Smoke")
                .arg(&path),
            &out.join(format!("export-{profile}.log")),
            180,
            0,
            "Godot Engine",
        )?;
        let executable = if platform == "macos" {
            output(Command::new("ditto").args(["-x", "-k"]).arg(&path).arg(&directory))?;
            files(&directory)?
                .into_iter()
                .find(|path| path.parent().is_some_and(|p| p.ends_with("Contents/MacOS")))
                .context("exported Mac executable missing")?
        } else {
            path
        };
        ensure!(executable.is_file(), "export did not create {}", executable.display());
        run_logged(
            Command::new(executable).arg("--headless"),
            &out.join(format!("export-{profile}-run.log")),
            30,
            0,
            "SPIRE_RELEASE_SMOKE_OK",
        )?;
    }
    Ok(())
}
fn select_profile(manifest: &Path, profile: &str) -> Result<()> {
    // The editor/runtime normally selects debug, so point its debug entries to
    // the chosen profile as well. Export tests retain the original manifest.
    if profile == "release" {
        fs::write(manifest, fs::read_to_string(manifest)?.replace("/debug/", "/release/"))?;
    }
    Ok(())
}
fn test(stage: &Path, godot: &Path, out: &Path, smoke_only: bool, templates: Option<&Path>) -> Result<()> {
    let record = verify(stage)?;
    let platform = record["platform"].as_str().unwrap();
    ensure!(
        platform == std::env::consts::OS || platform == "macos" && std::env::consts::OS == "macos",
        "test artifacts on their target OS"
    );
    let version = output(Command::new(godot).arg("--version"))?;
    fs::create_dir_all(out)?;
    for profile in ["debug", "release"] {
        smoke(stage, godot, out, profile)?;
        if smoke_only {
            continue;
        }
        for (kind, source, entry) in [
            ("gdscript", "spire_tween_gdscript", "res://examples/tests/test_runner.tscn"),
            ("native", "spire_tween_rust", "res://test_runner.tscn"),
        ] {
            let project = out.join(format!("{kind}-{profile}"));
            let addon = if kind == "native" {
                let addon = out.join("native-addon");
                copy_tree(&stage.join("tests/lib"), &addon.join("lib"))?;
                fs::write(addon.join("spire_tween.gdextension"), manifest(&[platform.into()], true))?;
                addon
            } else {
                stage.join("addon")
            };
            prepare(&root().join(source), &project, &addon, godot, &out.join(format!("{kind}-{profile}-prepare")))?;
            select_profile(&project.join("addons/spire_tween/spire_tween.gdextension"), profile)?;
            let mut command = engine_command(godot, &project);
            command
                .args(if kind == "native" { ["--max-fps", "120"] } else { ["--fixed-fps", "120"] })
                .arg(entry)
                .args(["--", "--all-tests", "--no-cache"]);
            let marker = format!(
                "{}_TEST_RESULT total={} failed=0",
                kind.to_ascii_uppercase(),
                config()["expected_results"][kind]
            );
            run_logged(&mut command, &out.join(format!("{kind}-{profile}.log")), 1000, 0, &marker)?;
            for (case, code) in [
                ("async_pass", 0),
                (if kind == "native" { "async_panic" } else { "helper_failure" }, 1),
            ] {
                let self_entry =
                    if kind == "native" { entry } else { "res://examples/tests/runner_failure_entry.tscn" };
                run_logged(
                    engine_command(godot, &project)
                        .arg("--fixed-fps")
                        .arg("120")
                        .arg(self_entry)
                        .args(["--", "--no-cache", &format!("--runner-self-test={case}")]),
                    &out.join(format!("{kind}-{profile}-{case}.log")),
                    30,
                    code,
                    &format!("{}_TEST_RESULT total=1 failed={code}", kind.to_ascii_uppercase()),
                )?;
            }
        }
    }
    if let Some(templates) = templates {
        export_smoke(stage, godot, out, templates)?;
    }
    write_json(
        &out.join("validation.json"),
        &json!({"schema":1,"platform":platform,"arch":std::env::consts::ARCH,"engine":version.trim(),"build_sha256":hash(&stage.join("build.json"))?,"full_suites":!smoke_only,"exports":templates.is_some(),"passed":true}),
    )?;
    eprintln!("Release validation passed: {} {}", platform, version.trim());
    Ok(())
}
fn download(version: &str, out: &Path, templates: bool) -> Result<()> {
    let checks: Value = serde_json::from_str(include_str!("../../godot-downloads.json"))?;
    ensure!(checks.get(version).is_some(), "Godot version has no pinned download checksums");
    fs::create_dir_all(out)?;
    let suffix = match std::env::consts::OS {
        "linux" => "linux.x86_64.zip",
        "windows" => "win64.exe.zip",
        "macos" => "macos.universal.zip",
        _ => bail!("unsupported download host"),
    };
    for suffix in [suffix, "export_templates.tpz"]
        .into_iter()
        .take(if templates { 2 } else { 1 })
    {
        let name = format!("Godot_v{version}-stable_{suffix}");
        let zip = out.join(&name);
        if !zip.exists() {
            let partial = out.join(format!("{name}.partial"));
            output(
                Command::new("curl")
                    .args(["--fail", "--location", "--retry", "3", "--silent", "--show-error", "--output"])
                    .arg(&partial)
                    .arg(format!(
                        "https://github.com/godotengine/godot-builds/releases/download/{version}-stable/{name}"
                    )),
            )?;
            fs::rename(partial, &zip)?;
        }
        ensure!(
            format!("{:x}", Sha512::digest(fs::read(&zip)?))
                == checks[version][&name].as_str().context("missing pinned checksum")?,
            "Godot download checksum mismatch: {name}"
        );
        let destination = if suffix.ends_with("tpz") { out.join("export") } else { out.join("editor") };
        if std::env::consts::OS == "macos" && !suffix.ends_with("tpz") {
            output(Command::new("ditto").args(["-x", "-k"]).arg(&zip).arg(&destination))?;
        } else {
            zip::ZipArchive::new(fs::File::open(zip)?)?.extract(&destination)?;
        }
    }
    let binary = match std::env::consts::OS {
        "windows" => out.join(format!("editor/Godot_v{version}-stable_win64_console.exe")),
        "macos" => out.join("editor/Godot.app/Contents/MacOS/Godot"),
        _ => out.join(format!("editor/Godot_v{version}-stable_linux.x86_64")),
    };
    ensure!(binary.is_file(), "downloaded Godot binary missing: {}", binary.display());
    fs::write(out.join("godot-path"), binary.canonicalize()?.to_string_lossy().as_bytes())?;
    Ok(())
}
fn package(staging: &Path, reports: &Path, out: &Path, candidate: bool, platforms: &[String]) -> Result<()> {
    let desktop = vec!["linux".to_owned(), "windows".to_owned(), "macos".to_owned()];
    ensure!(candidate || platforms == desktop, "commercial archive must include all desktop platforms");
    let mut seen = BTreeSet::new();
    for platform in platforms {
        ensure!(desktop.contains(platform) && seen.insert(platform), "invalid or duplicated desktop platform");
    }
    ensure!(!platforms.is_empty(), "empty platform list");
    let mut records = Vec::new();
    let mut source = Value::Null;
    let mut payload = BTreeMap::<String, Vec<u8>>::new();
    for platform in platforms {
        let platform = platform.as_str();
        let stage = staging.join(platform);
        let record = verify(&stage)?;
        ensure!(record["platform"] == platform, "platform metadata mismatch");
        if source.is_null() {
            source = record["source"].clone();
        }
        ensure!(
            source["sha256"] == record["source"]["sha256"] && source["revision"] == record["source"]["revision"],
            "mixed source revisions in package"
        );
        if !candidate {
            ensure!(source["dirty"] == false, "release artifacts were built from a dirty checkout");
            for version in config()["godot_versions"].as_array().unwrap() {
                for arch in if platform == "macos" { vec!["aarch64", "x86_64"] } else { vec!["x86_64"] } {
                    let mut matched = false;
                    for report in files(reports)?
                        .into_iter()
                        .filter(|p| p.file_name().is_some_and(|n| n == "validation.json"))
                    {
                        let value = read_json(&report)?;
                        if value["platform"] == platform
                            && value["arch"] == arch
                            && value["engine"]
                                .as_str()
                                .is_some_and(|s| s.starts_with(&format!("{}.stable.", version.as_str().unwrap())))
                            && value["build_sha256"] == hash(&stage.join("build.json"))?
                            && value["full_suites"] == true
                            && value["exports"] == true
                            && value["passed"] == true
                        {
                            matched = true;
                        }
                    }
                    ensure!(matched, "missing full validation for {platform}/{arch}/Godot {version}");
                }
            }
        }
        for (folder, prefix) in [
            ("addon/lib", "addons/spire_tween/lib".to_owned()),
            ("notices", format!("third_party/{platform}")),
        ] {
            for path in files(&stage.join(folder))? {
                let name = format!(
                    "{prefix}/{}",
                    path.strip_prefix(stage.join(folder))?
                        .to_string_lossy()
                        .replace('\\', "/")
                );
                ensure!(payload.insert(name, fs::read(path)?).is_none(), "duplicate package path");
            }
        }
        records.push(record);
    }
    ensure!(
        source_identity()?["sha256"] == source["sha256"],
        "packaging checkout does not match the artifact source snapshot"
    );
    payload.insert("addons/spire_tween/spire_tween.gdextension".into(), manifest(platforms, false).into_bytes());
    for (file, name) in [
        ("LICENSE", "LICENSE"),
        ("LICENSE", "addons/spire_tween/LICENSE"),
        ("release/INSTALL.md", "README.md"),
        ("release/README.md", "RELEASE.md"),
        ("Cargo.lock", "build/Cargo.lock"),
        ("rust-toolchain.toml", "build/rust-toolchain.toml"),
        ("release/config.json", "build/config.json"),
    ] {
        payload.insert(name.into(), fs::read(root().join(file))?);
    }
    let mut validations = Vec::new();
    for path in files(reports)?
        .into_iter()
        .filter(|p| p.file_name().is_some_and(|n| n == "validation.json"))
    {
        validations.push(read_json(&path)?);
    }
    payload.insert(
        "release.json".into(),
        serde_json::to_vec_pretty(
            &json!({"schema":1,"candidate":candidate,"platforms":records,"required_validation_versions":config()["godot_versions"],"validations":validations}),
        )?,
    );
    let sums = payload
        .iter()
        .map(|(name, data)| format!("{:x}  {name}\n", Sha256::digest(data)))
        .collect::<String>();
    payload.insert("SHA256SUMS".into(), sums.into_bytes());
    fs::create_dir_all(out)?;
    let package: toml::Value = toml::from_str(&fs::read_to_string(root().join("spire_tween/Cargo.toml"))?)?;
    let label = if platforms == desktop {
        if candidate { "candidate".into() } else { "desktop".into() }
    } else {
        format!("candidate-{}", platforms.join("-"))
    };
    let name = format!(
        "SpireTween-{}-{}-{}.zip",
        package["package"]["version"].as_str().unwrap(),
        &source["revision"].as_str().unwrap()[..12],
        label
    );
    let archive = out.join(name);
    let mut zip = zip::ZipWriter::new(fs::File::create(&archive)?);
    for (name, data) in payload {
        let executable = name.ends_with(".so") || name.ends_with(".dll") || name.ends_with("/spire_tween");
        zip.start_file(
            name,
            zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .unix_permissions(if executable { 0o755 } else { 0o644 }),
        )?;
        zip.write_all(&data)?;
    }
    zip.finish()?;
    fs::write(
        out.join("SHA256SUMS"),
        format!("{}  {}\n", hash(&archive)?, archive.file_name().unwrap().to_string_lossy()),
    )?;
    eprintln!("Packaged {}", archive.display());
    Ok(())
}
fn archive_test(archive: &Path, godot: &Path, out: &Path, templates: Option<&Path>) -> Result<()> {
    let extracted = out.join("extracted");
    ensure!(!extracted.exists(), "archive test output already exists");
    zip::ZipArchive::new(fs::File::open(archive)?)?.extract(&extracted)?;
    let sums = fs::read_to_string(extracted.join("SHA256SUMS"))?;
    let mut expected = BTreeSet::new();
    for line in sums.lines() {
        let (digest, name) = line.split_once("  ").context("invalid checksum manifest")?;
        ensure!(expected.insert(name), "duplicate checksum path");
        ensure!(
            Path::new(name)
                .components()
                .all(|c| matches!(c, std::path::Component::Normal(_))),
            "unsafe archive path"
        );
        ensure!(hash(&extracted.join(name))? == digest, "archive checksum mismatch: {name}");
    }
    for file in files(&extracted)? {
        let name = file.strip_prefix(&extracted)?.to_string_lossy().replace('\\', "/");
        ensure!(name == "SHA256SUMS" || expected.contains(name.as_str()), "unrecorded archive file: {name}");
    }
    let stage = out.join("installed");
    copy_tree(&extracted.join("addons/spire_tween"), &stage.join("addon"))?;
    for profile in ["debug", "release"] {
        smoke(&stage, godot, out, profile)?;
    }
    if let Some(templates) = templates {
        export_smoke(&stage, godot, out, templates)?;
    }
    write_json(
        &out.join("archive-validation.json"),
        &json!({"archive_sha256":hash(archive)?,"engine":output(Command::new(godot).arg("--version"))?.trim(),"platform":std::env::consts::OS,"arch":std::env::consts::ARCH,"exports":templates.is_some(),"passed":true}),
    )
}
fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let command = args
        .next()
        .context("usage: spire-release build|test|download|verify|package [--option value]")?;
    let mut values = BTreeMap::new();
    let mut flags = BTreeSet::new();
    while let Some(key) = args.next() {
        ensure!(key.starts_with("--"), "expected --option, got {key}");
        if ["--candidate", "--download-templates", "--smoke-only"].contains(&key.as_str()) {
            flags.insert(key);
        } else {
            ensure!(
                values
                    .insert(key.clone(), args.next().context(format!("{key} requires a value"))?)
                    .is_none(),
                "duplicate option {key}"
            );
        }
    }
    let allowed: &[&str] = match command.as_str() {
        "build" => &["--platform", "--out"],
        "test" => &["--stage", "--godot", "--out", "--templates", "--smoke-only"],
        "test-archive" => &["--archive", "--godot", "--out", "--templates"],
        "verify" => &["--stage"],
        "download" => &["--version", "--out", "--download-templates"],
        "package" => &["--staging", "--reports", "--out", "--candidate", "--platforms"],
        _ => bail!("unknown command {command}"),
    };
    for key in values.keys().chain(flags.iter()) {
        ensure!(allowed.contains(&key.as_str()), "unknown option {key}");
    }
    let cwd = std::env::current_dir()?;
    for (key, value) in &mut values {
        if ["--stage", "--staging", "--reports", "--archive", "--out", "--templates"].contains(&key.as_str())
            || key == "--godot" && Path::new(value).components().count() > 1
        {
            *value = cwd.join(&*value).to_string_lossy().into_owned();
        }
    }
    let get = |key: &str| -> Result<&str> {
        values
            .get(key)
            .map(String::as_str)
            .with_context(|| format!("missing {key}"))
    };
    match command.as_str() {
        "build" => build(get("--platform")?, Path::new(get("--out")?)),
        "test" => test(
            Path::new(get("--stage")?),
            Path::new(get("--godot")?),
            Path::new(get("--out")?),
            flags.contains("--smoke-only"),
            values.get("--templates").map(Path::new),
        ),
        "test-archive" => archive_test(
            Path::new(get("--archive")?),
            Path::new(get("--godot")?),
            Path::new(get("--out")?),
            values.get("--templates").map(Path::new),
        ),
        "verify" => {
            verify(Path::new(get("--stage")?))?;
            Ok(())
        }
        "download" => download(get("--version")?, Path::new(get("--out")?), flags.contains("--download-templates")),
        "package" => package(
            Path::new(get("--staging")?),
            Path::new(get("--reports")?),
            Path::new(get("--out")?),
            flags.contains("--candidate"),
            &values
                .get("--platforms")
                .map(|s| s.split(',').map(str::to_owned).collect::<Vec<_>>())
                .unwrap_or_else(|| vec!["linux".into(), "windows".into(), "macos".into()]),
        ),
        _ => bail!("unknown command {command}"),
    }
}
