#!/usr/bin/env bash
set -euo pipefail

# Real rendering on an isolated display; no host editor or project cache is used.
repo=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
engine=${1:-godot}
output=${2:-$(mktemp -d /tmp/spire-text-presets.XXXXXX)}
library=${SPIRE_PRESET_LIBRARY:-}
if [[ -z "$library" ]]; then
    cargo build --manifest-path "$repo/Cargo.toml" --locked -p spire_tween \
        --features include_gdext_lib,include_gdscript_bridge,api-4-4,integration-tests
    library="$repo/target/debug/libspire_tween.so"
fi
mkdir -p "$output/project/examples/tests" "$output/project/.godot" "$output/xdg"
output=$(cd -- "$output" && pwd)
project="$output/project"
cp --reflink=auto -- "$library" "$project/libspire_tween.so"
cp -a -- "$repo/spire_tween_gdscript/examples/." "$project/examples/"
cat > "$project/project.godot" <<'EOF'
config_version=5
[application]
config/name="Spire text preset integration"
run/flush_stdout_on_print=true
[display]
window/size/viewport_width=1000
window/size/viewport_height=700
window/vsync/vsync_mode=0
[rendering]
renderer/rendering_method="gl_compatibility"
EOF
cat > "$project/spire_tween.gdextension" <<EOF
[configuration]
entry_symbol="gdext_rust_init"
compatibility_minimum="4.4"
[libraries]
linux.debug.x86_64="res://libspire_tween.so"
linux.release.x86_64="res://libspire_tween.so"
EOF
printf 'res://spire_tween.gdextension\n' > "$project/.godot/extension_list.cfg"
cat > "$project/.godot/global_script_class_cache.cfg" <<'EOF'
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
unset DISPLAY WAYLAND_DISPLAY XAUTHORITY
export XDG_DATA_HOME="$output/xdg/data" XDG_CONFIG_HOME="$output/xdg/config" XDG_CACHE_HOME="$output/xdg/cache"
export LIBGL_ALWAYS_SOFTWARE=true LP_NUM_THREADS=2 OMP_NUM_THREADS=2
: > "$output/display-number"
Xvfb -displayfd 3 -screen 0 1440x1000x24 -nolisten tcp \
    3>"$output/display-number" >"$output/xvfb.log" 2>&1 </dev/null &
display_pid=$!
trap 'kill "$display_pid" 2>/dev/null || true' EXIT
for _attempt in {1..100}; do
    [[ -s "$output/display-number" ]] && break
    sleep 0.05
done
read -r display_number < "$output/display-number"
export DISPLAY=":$display_number"
timeout 90s "$engine" --path "$project" --rendering-method gl_compatibility --audio-driver Dummy \
    --max-fps 60 --script res://examples/tests/text_effect_presets_runner.gd >"$output/presets.log" 2>&1
if rg -n 'SCRIPT ERROR:|^ERROR:|\[panic' "$output/presets.log"; then exit 1; fi
rg '^TEXT_PRESET_RESULT total=[1-9][0-9]* failed=0$' "$output/presets.log"
# Invalid elements must be rejected by Godot without changing the typed array.
# Keep these expected engine errors separate from the strict rendering checks.
timeout -k 5s 30s "$engine" --headless --path "$project" --audio-driver Dummy \
    --script res://examples/tests/text_effect_presets_runner.gd -- --typed-array-rejections >"$output/type-rejections.log" 2>&1
awk '
    /^TEXT_PRESET_TYPE_REJECTION_BEGIN$/ { expected = 1; next }
    /^TEXT_PRESET_TYPE_REJECTION_END$/ { expected = 0; next }
    /SCRIPT ERROR:|\[panic/ { failed = 1; print }
    /^ERROR:/ {
        if (expected && /^ERROR: Attempted to (push_back|insert).*TypedArray/) count++
        else if (expected && /^ERROR: Condition "!_p->typed[.]validate\(value, "(push_back|insert)"\)" is true[.]/) guards++
        else { failed = 1; print }
    }
    END { if (failed || count != 4 || guards != 4 || expected) exit 1 }
' "$output/type-rejections.log"
rg '^TEXT_PRESET_TYPE_REJECTION_RESULT failed=0$' "$output/type-rejections.log"
mkdir -p "$project/addons/preset_probe"
cp -- "$project/examples/tests/text_effect_presets_editor.gd" "$project/addons/preset_probe/probe.gd"
cp -- "$project/examples/tests/text_effect_presets_editor_fixture.tscn" "$project/text_effect_presets_editor_fixture.tscn"
cat > "$project/addons/preset_probe/plugin.cfg" <<'EOF'
[plugin]
name="Text preset Inspector integration"
description="Isolated preset authoring probe"
author="SpireTween"
version="1"
script="probe.gd"
EOF
cat >> "$project/project.godot" <<'EOF'
[editor_plugins]
enabled=PackedStringArray("res://addons/preset_probe/plugin.cfg")
EOF
timeout 120s "$engine" --editor --path "$project" --rendering-method gl_compatibility --audio-driver Dummy \
    --max-fps 60 >"$output/editor.log" 2>&1
if rg -n 'SCRIPT ERROR:|^ERROR:|\[panic' "$output/editor.log"; then exit 1; fi
rg '^TEXT_PRESET_EDITOR_RESULT failed=0$' "$output/editor.log"
if [[ "${SPIRE_PRESET_FULL_SUITE:-0}" == 1 ]]; then
    timeout 900s "$engine" --path "$project" --rendering-method gl_compatibility --audio-driver Dummy \
        --max-fps 60 res://examples/tests/test_runner.tscn -- --all-tests --no-cache >"$output/full-suite.log" 2>&1
    rg '^GDSCRIPT_TEST_RESULT total=[1-9][0-9]* failed=0 skipped=0$' "$output/full-suite.log"
fi
printf 'Text preset evidence: %s\n' "$output"
