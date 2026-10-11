"""Exercise build-script startup and failure paths using real shell/Cargo tools."""

import hashlib
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPTS = REPO_ROOT / "build_scripts"


class BuildScriptChecks(unittest.TestCase):
    def run_script(self, name, variables=None, unset=()):
        env = os.environ.copy()
        env.update(variables or {})
        for key in unset:
            env.pop(key, None)
        return subprocess.run(
            [str(SCRIPTS / name)],
            cwd=tempfile.gettempdir(),
            env=env,
            capture_output=True,
            text=True,
            timeout=30,
        )

    def test_scripts_are_executable_and_parse(self):
        for name in ["build_linux.sh", "build_android.sh", "build_windows_64.sh", "build_wasm.sh"]:
            with self.subTest(script=name):
                self.assertTrue(os.access(SCRIPTS / name, os.X_OK))
                result = subprocess.run(["bash", "-n", str(SCRIPTS / name)], capture_output=True)
                self.assertEqual(result.returncode, 0, result.stderr)

    def test_android_requires_an_ndk_setting(self):
        result = self.run_script("build_android.sh", unset=("ANDROID_NDK_HOME",))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Set ANDROID_NDK_HOME", result.stderr)

    def test_android_rejects_an_invalid_ndk_directory(self):
        with tempfile.TemporaryDirectory() as directory:
            result = self.run_script("build_android.sh", {"ANDROID_NDK_HOME": directory})
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("does not contain an NDK toolchain", result.stderr)

    @unittest.skipIf(os.name == "nt", "Native Windows uses its MSVC environment")
    def test_cross_windows_requires_an_sdk_setting(self):
        result = self.run_script("build_windows_64.sh", unset=("XWIN_DIR",))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Set XWIN_DIR", result.stderr)

    @unittest.skipIf(os.name == "nt", "Native Windows uses its MSVC environment")
    def test_cross_windows_rejects_an_invalid_sdk_directory(self):
        with tempfile.TemporaryDirectory() as directory:
            result = self.run_script("build_windows_64.sh", {"XWIN_DIR": directory})
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Missing xwin SDK directory", result.stderr)

    def test_web_reports_a_missing_emscripten_tool(self):
        # This restricted PATH contains real executables, with no emcc installed.
        with tempfile.TemporaryDirectory() as directory:
            for name in ["bash", "dirname"]:
                Path(directory, name).symlink_to(shutil.which(name))
            result = self.run_script("build_wasm.sh", {"PATH": directory})
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Emscripten is required", result.stderr)

    def test_failed_build_stops_and_preserves_existing_artifacts(self):
        libraries = list((REPO_ROOT / "spire_tween_gdscript/addons/spire_tween/lib/x86_64-unknown-linux-gnu").glob("*/libspire_tween.so"))
        hashes = {path: hashlib.sha256(path.read_bytes()).hexdigest() for path in libraries}
        with tempfile.TemporaryDirectory() as directory:
            result = self.run_script("build_linux.sh", {"RUSTC": str(Path(directory, "missing-rustc"))})
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn("Finished `release`", result.stderr)
        self.assertEqual(hashes, {path: hashlib.sha256(path.read_bytes()).hexdigest() for path in libraries})

    def test_delivered_project_licenses_match(self):
        license_text = (REPO_ROOT / "LICENSE").read_bytes()
        for project in ["spire_tween_gdscript", "spire_tween_rust"]:
            self.assertEqual((REPO_ROOT / project / "addons/spire_tween/LICENSE").read_bytes(), license_text)


if __name__ == "__main__":
    unittest.main()
