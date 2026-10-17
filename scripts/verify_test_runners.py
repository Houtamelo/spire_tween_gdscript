#!/usr/bin/env python3
"""Exercise the Linux x86_64 Godot runners, including deliberate failures and cache reuse.

Build both test/addon libraries first. This script copies existing imported project
caches into temporary fixtures; it never imports an editor or edits project caches.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

REPO = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--godot", default="godot")
    parser.add_argument("--profiles", nargs="+", choices=["debug", "release"], default=["debug", "release"])
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    output = args.output or Path(tempfile.mkdtemp(prefix="spire-runner-checks-"))
    output.mkdir(parents=True, exist_ok=True)
    results = []

    def run(label, project, entry, flags, expected, marker="_TEST_RESULT", executed=None):
        command = [args.godot, "--headless", "--path", str(project), "--fixed-fps", "120", entry, "--", *flags]
        log = output / (label + ".log")
        with log.open("w") as stream:
            try:
                process = subprocess.run(command, stdout=stream, stderr=subprocess.STDOUT,
                                         env=dict(os.environ, XDG_DATA_HOME=str(output / "userdata" / label)), timeout=15)
                code = process.returncode
            except subprocess.TimeoutExpired:
                code = 124
        body = log.read_text()
        ok = code == expected and marker in body
        if executed is not None:
            ok = ok and (("CACHE_TEST_EXECUTED" in body) == executed)
        record = dict(label=label, command=command, exit_code=code, expected=expected, passed=ok)
        results.append(record)
        (output / "results.json").write_text(json.dumps(results, indent=2) + "\n")
        print(json.dumps(record), flush=True)
        if not ok:
            raise AssertionError(f"{label}: see {log}")

    for profile in args.profiles:
        projects = {}
        for kind, crate in [("native", "spire_tween_rust"), ("gdscript", "spire_tween")]:
            source = REPO / ("spire_tween_rust" if kind == "native" else "spire_tween_gdscript")
            project = output / (profile + "-" + kind)
            shutil.copytree(source, project, ignore=shutil.ignore_patterns("addons", "src", "results.json"))
            library = REPO / "target" / profile / ("lib" + crate + ".so")
            installed = project / "addons/spire_tween/lib" / library.name
            installed.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(library, installed)
            assert hashlib.sha256(library.read_bytes()).digest() == hashlib.sha256(installed.read_bytes()).digest()
            (project / "addons/spire_tween/spire_tween.gdextension").write_text(
                '[configuration]\nentry_symbol="gdext_rust_init"\ncompatibility_minimum="4.4.1"\n'
                f'[libraries]\nlinux.x86_64="res://addons/spire_tween/lib/{library.name}"\n')
            projects[kind] = project

        gd_entry = "res://examples/tests/runner_failure_entry.tscn"
        native_entry = "res://test_runner.tscn"
        for kind, cases in [
            ("gdscript", ["pass", "async_pass", "helper_failure", "other_helpers_fail", "false_return", "nil_return",
                          "body_assertion", "async_assertion", "callback_failure", "timeout", "paused_timeout"]),
            ("native", ["pass", "async_pass", "setup_panic", "async_panic", "callback_panic", "timeout", "paused_timeout"]),
        ]:
            entry = gd_entry if kind == "gdscript" else native_entry
            for case in cases:
                flags = ["--no-cache", "--runner-self-test=" + case, "--test-timeout=0.1"]
                run(f"{profile}-{kind}-{case}", projects[kind], entry, flags,
                    0 if case in ["pass", "async_pass"] else 1)
            run(f"{profile}-{kind}-suite-timeout", projects[kind], entry,
                ["--no-cache", "--runner-self-test=timeout", "--test-timeout=10", "--suite-timeout=0.1"],
                1, marker="TEST_SUITE_TIMEOUT")
            run(f"{profile}-{kind}-paused-suite-timeout", projects[kind], entry,
                ["--no-cache", "--runner-self-test=paused_timeout", "--test-timeout=10", "--suite-timeout=0.1"],
                1, marker="TEST_SUITE_TIMEOUT")
            run(f"{profile}-{kind}-unknown-test", projects[kind], entry,
                ["--no-cache", "--test=does_not_exist"], 1)
            run(f"{profile}-{kind}-invalid-timeout", projects[kind], entry,
                ["--no-cache", "--test-timeout=NaN"], 2, marker="Invalid timeout")

        project = projects["gdscript"]
        cache_flags = ["--runner-self-test=cache"]
        run(profile + "-cache-first", project, gd_entry, cache_flags, 0, executed=True)
        run(profile + "-cache-reuse", project, gd_entry, cache_flags, 0, executed=False)
        cache = project / "examples/tests/results.json"
        original = cache.read_bytes()
        run(profile + "-cache-bypass", project, gd_entry, cache_flags + ["--no-cache"], 0, executed=True)
        assert cache.read_bytes() == original, "--no-cache must neither read nor write results"

        # Change actual shared test source in the isolated fixture.
        shared = project / "examples/tests/test_base.gd"
        shared.write_text(shared.read_text() + "\n# cache invalidation integration check\n")
        run(profile + "-cache-source-change", project, gd_entry, cache_flags, 0, executed=True)
        run(profile + "-cache-source-reuse", project, gd_entry, cache_flags, 0, executed=False)

        # Common example scripts/scenes are also test inputs, even outside tests/.
        dependency = project / "examples/runner_cache_dependency.gd"
        dependency.write_text("extends Node\n")
        run(profile + "-cache-example-source-change", project, gd_entry, cache_flags, 0, executed=True)

        # Swap two valid addon artifacts, rather than pretending a library changed.
        other = "release" if profile == "debug" else "debug"
        replacement = REPO / "target" / other / "libspire_tween.so"
        installed = project / "addons/spire_tween/lib/libspire_tween.so"
        assert installed.read_bytes() != replacement.read_bytes(), "debug/release artifacts must differ"
        shutil.copy2(replacement, installed)
        run(profile + "-cache-binary-change", project, gd_entry, cache_flags, 0, executed=True)
        run(profile + "-cache-binary-reuse", project, gd_entry, cache_flags, 0, executed=False)

        data = json.loads(cache.read_text())
        data["identity"]["engine"] = "a different engine executable"
        cache.write_text(json.dumps(data))
        run(profile + "-cache-engine-change", project, gd_entry, cache_flags, 0, executed=True)

    print(f"RUNNER_REGRESSIONS_RESULT total={len(results)} failed=0 output={output}")


if __name__ == "__main__":
    main()
