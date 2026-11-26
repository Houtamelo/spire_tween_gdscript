#!/usr/bin/env bash
set -euo pipefail

# Real editor, private display/project/user data; never replaces installed binaries.
preview_repo=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
preview_engine=${1:-godot}
preview_output=${2:-$(mktemp -d /tmp/spire-blueprint-preview.XXXXXX)}
preview_library=${SPIRE_PREVIEW_LIBRARY:-$preview_repo/target/debug/libspire_tween.so}
if [[ ! -f "$preview_library" ]]; then
    printf 'Build the GDScript addon first, or set SPIRE_PREVIEW_LIBRARY.\n' >&2
    exit 1
fi
preview_library=$(realpath -- "$preview_library")
mkdir -p "$preview_output/project/addons/preview_probe" "$preview_output/project/addons/spire_tween" "$preview_output/project/.godot" "$preview_output/project/examples/tests"
preview_output=$(cd -- "$preview_output" && pwd)
preview_project="$preview_output/project"
cp -- "$preview_repo/spire_tween_gdscript/examples/tests/blueprint_preview_editor.gd" "$preview_repo/spire_tween_gdscript/examples/tests/blueprint_preview_custom.gd" "$preview_project/examples/tests/"
cp -- "$preview_repo/spire_tween_gdscript/examples/tests/blueprint_preview_runtime.gd" "$preview_project/examples/tests/"
cp -- "$preview_repo/spire_tween_gdscript/examples/tests/blueprint_preview_uncaptured.gd" "$preview_project/examples/tests/"
cp -- "$preview_repo/spire_tween_gdscript/examples/tests/blueprint_preview_editor.gd" "$preview_project/addons/preview_probe/probe.gd"
cp -- "$preview_repo/spire_tween_gdscript/examples/tests/blueprint_preview_custom.gd" "$preview_project/addons/preview_probe/"
cp -- "$preview_repo/spire_tween_gdscript/examples/tests/blueprint_preview_uncaptured.gd" "$preview_project/addons/preview_probe/"
cp -a -- "$preview_repo/spire_tween_gdscript/addons/spire_tween/templates" "$preview_project/addons/spire_tween/"
cp -a -- "$preview_repo/spire_tween_gdscript/examples/effect_templates" "$preview_project/examples/"
cat > "$preview_project/project.godot" <<'EOF'
config_version=5
[application]
config/name="Spire Blueprint preview integration"
run/flush_stdout_on_print=true
[display]
window/size/viewport_width=1440
window/size/viewport_height=1000
[rendering]
renderer/rendering_method="gl_compatibility"
EOF
cat > "$preview_project/spire_tween.gdextension" <<EOF
[configuration]
entry_symbol="gdext_rust_init"
compatibility_minimum="4.4"
[libraries]
linux.debug.x86_64="$preview_library"
linux.release.x86_64="$preview_library"
EOF
printf 'res://spire_tween.gdextension\n' > "$preview_project/.godot/extension_list.cfg"
cat > "$preview_project/preview_scene.tscn" <<'EOF'
[gd_scene format=3]
[node name="PreviewScene" type="Node2D"]
[node name="Target" type="Node2D" parent="."]
position = Vector2(20, 30)
[node name="Visual" type="Polygon2D" parent="Target"]
polygon = PackedVector2Array(0, 0, 80, 0, 80, 60, 0, 60)
color = Color(0.3, 0.8, 0.6, 1)
EOF
cat > "$preview_project/other_scene.tscn" <<'EOF'
[gd_scene format=3]
[node name="OtherScene" type="Node2D"]
EOF
unset DISPLAY WAYLAND_DISPLAY XAUTHORITY
export XDG_DATA_HOME="$preview_output/xdg/data" XDG_CONFIG_HOME="$preview_output/xdg/config" XDG_CACHE_HOME="$preview_output/xdg/cache"
export LIBGL_ALWAYS_SOFTWARE=true LP_NUM_THREADS=2 OMP_NUM_THREADS=2
Xvfb -displayfd 3 -screen 0 1600x1200x24 -nolisten tcp 3>"$preview_output/display-number" >"$preview_output/xvfb.log" 2>&1 </dev/null &
preview_display_pid=$!
trap 'kill "$preview_display_pid" 2>/dev/null || true' EXIT
for _attempt in {1..100}; do
    [[ -s "$preview_output/display-number" ]] && break
    sleep 0.05
done
read -r preview_display < "$preview_output/display-number"
export DISPLAY=":$preview_display"
timeout -k 5s 60s "$preview_engine" --editor --path "$preview_project" --audio-driver Dummy --import > "$preview_output/import.log" 2>&1
if rg -n 'SCRIPT ERROR:|^ERROR:|\[panic' "$preview_output/import.log"; then exit 1; fi
timeout -k 5s 30s "$preview_engine" --headless --path "$preview_project" --script res://examples/tests/blueprint_preview_runtime.gd > "$preview_output/runtime.log" 2>&1
if rg -n 'SCRIPT ERROR:|^ERROR:|\[panic' "$preview_output/runtime.log"; then exit 1; fi
rg '^BLUEPRINT_PREVIEW_RUNTIME_RESULT checks=2 failed=0$' "$preview_output/runtime.log"
cat > "$preview_project/addons/preview_probe/plugin.cfg" <<'EOF'
[plugin]
name="Blueprint preview integration"
description="Real editor preview and restoration tests"
author="SpireTween"
version="1"
script="probe.gd"
EOF
cat >> "$preview_project/project.godot" <<'EOF'
[editor_plugins]
enabled=PackedStringArray("res://addons/preview_probe/plugin.cfg")
EOF
timeout -k 5s 120s "$preview_engine" --editor --path "$preview_project" --audio-driver Dummy > "$preview_output/editor.log" 2>&1
if rg -n 'SCRIPT ERROR:|^ERROR:|\[panic' "$preview_output/editor.log"; then exit 1; fi
rg '^BLUEPRINT_PREVIEW_EDITOR_RESULT checks=[1-9][0-9]* failed=0$' "$preview_output/editor.log"
printf 'Blueprint preview evidence: %s\n' "$preview_output"
