#!/usr/bin/env bash
set -euo pipefail

# Usage: scripts/test_blueprint.sh [godot-binary] [--rendered]
# SPIRE_BLUEPRINT_LIBRARY can reuse a built API-4.4 library for an engine matrix.
# SPIRE_BLUEPRINT_TEST_DIR selects the retained isolated fixture/evidence directory.
blueprint_repo=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
blueprint_godot=${1:-godot}
blueprint_rendered=${2:-}
blueprint_fixture=${SPIRE_BLUEPRINT_TEST_DIR:-$(mktemp -d /tmp/spire-blueprint-tests.XXXXXX)}
blueprint_library=${SPIRE_BLUEPRINT_LIBRARY:-}
if [[ -z "$blueprint_library" ]]; then
    cargo build --manifest-path "$blueprint_repo/Cargo.toml" --locked -p spire_tween \
        --features include_gdext_lib,include_gdscript_bridge,api-4-4
    blueprint_library="$blueprint_repo/target/debug/libspire_tween.so"
fi
mkdir -p "$blueprint_fixture/examples/tests" "$blueprint_fixture/.godot" "$blueprint_fixture/addons/blueprint_probe"
for blueprint_file in blueprint.gd blueprint.tscn blueprint_custom.gd blueprint_property_target.gd blueprint_runner.gd blueprint_editor.gd test_base.gd rich_text_label_process_override.gd rich_text_label_process_inherited.gd rich_text_label_no_process.gd; do
    cp -- "$blueprint_repo/spire_tween_gdscript/examples/tests/$blueprint_file" "$blueprint_fixture/examples/tests/$blueprint_file"
done
cp -- "$blueprint_repo/spire_tween_gdscript/examples/tests/blueprint_editor_fixture.tscn" "$blueprint_fixture/blueprint_editor_fixture.tscn"
cp -- "$blueprint_fixture/examples/tests/blueprint_editor.gd" "$blueprint_fixture/addons/blueprint_probe/probe.gd"
cat > "$blueprint_fixture/project.godot" <<'EOF'
config_version=5
[application]
config/name="SpireTween blueprint integration"
[rendering]
renderer/rendering_method="gl_compatibility"
EOF
cat > "$blueprint_fixture/spire_tween.gdextension" <<EOF
[configuration]
entry_symbol="gdext_rust_init"
compatibility_minimum="4.4"
[libraries]
linux.debug.x86_64="$blueprint_library"
linux.release.x86_64="$blueprint_library"
EOF
printf 'res://spire_tween.gdextension\n' > "$blueprint_fixture/.godot/extension_list.cfg"
cat > "$blueprint_fixture/.godot/global_script_class_cache.cfg" <<'EOF'
list=[{
"base": &"Node",
"class": &"TestBase",
"icon": "",
"is_abstract": false,
"is_tool": false,
"language": &"GDScript",
"path": "res://examples/tests/test_base.gd"
}]
EOF
export XDG_DATA_HOME="$blueprint_fixture/data"
export XDG_CONFIG_HOME="$blueprint_fixture/config"
export XDG_CACHE_HOME="$blueprint_fixture/cache"
timeout 60s "$blueprint_godot" --headless --path "$blueprint_fixture" \
    --script res://examples/tests/blueprint_runner.gd > "$blueprint_fixture/runtime.log" 2>&1
if rg -n 'SCRIPT ERROR:|^ERROR:|\[panic' "$blueprint_fixture/runtime.log"; then exit 1; fi
rg '^BLUEPRINT_RESULT total=[1-9][0-9]* failed=0$' "$blueprint_fixture/runtime.log"
cat >> "$blueprint_fixture/project.godot" <<'EOF'
[editor_plugins]
enabled=PackedStringArray("res://addons/blueprint_probe/plugin.cfg")
EOF
cat > "$blueprint_fixture/addons/blueprint_probe/plugin.cfg" <<'EOF'
[plugin]
name="Blueprint editor integration"
description="Isolated blueprint Inspector test"
author="SpireTween"
version="1"
script="probe.gd"
EOF
blueprint_editor_args=(--headless)
if [[ "$blueprint_rendered" == --rendered ]]; then
    Xvfb -displayfd 3 -screen 0 1440x1000x24 -nolisten tcp \
        3>"$blueprint_fixture/display-number" >"$blueprint_fixture/xvfb.log" 2>&1 </dev/null &
    blueprint_xvfb_pid=$!
    trap 'kill "$blueprint_xvfb_pid" 2>/dev/null || true' EXIT
    for blueprint_attempt in {1..40}; do
        if [[ -s "$blueprint_fixture/display-number" ]]; then break; fi
        sleep 0.05
    done
    read -r blueprint_display < "$blueprint_fixture/display-number"
    export DISPLAY=":$blueprint_display"
    blueprint_editor_args=()
fi
timeout 90s "$blueprint_godot" "${blueprint_editor_args[@]}" --editor --path "$blueprint_fixture" \
    --rendering-method gl_compatibility > "$blueprint_fixture/editor.log" 2>&1
if rg -n 'SCRIPT ERROR:|^ERROR:|\[panic' "$blueprint_fixture/editor.log"; then exit 1; fi
if rg -n 'WARNING:.*`animated_property` must be non-empty' "$blueprint_fixture/editor.log"; then
    printf 'Incomplete Inspector configuration must not emit repeated console warnings.\n' >&2
    exit 1
fi
rg '^BLUEPRINT_EDITOR_RESULT failed=0$' "$blueprint_fixture/editor.log"
printf 'Blueprint test evidence: %s\n' "$blueprint_fixture"
