#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd -- "$repo_root"

cargo build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --target=x86_64-unknown-linux-gnu --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/x86_64-unknown-linux-gnu/debug"
cargo build -p spire_tween --features include_gdext_lib,include_gdscript_bridge --target=x86_64-unknown-linux-gnu --release --artifact-dir="spire_tween_gdscript/addons/spire_tween/lib/x86_64-unknown-linux-gnu/release"
