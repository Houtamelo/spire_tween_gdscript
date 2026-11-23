#!/usr/bin/env bash
set -euo pipefail

# Uses a private Xvfb and project copy; never changes an installed addon binary.
template_repo=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
template_engine=${1:-godot}
template_output=${2:-$(mktemp -d /tmp/spire-scene-templates.XXXXXX)}
template_library=${SPIRE_TEMPLATE_LIBRARY:-$template_repo/target/debug/libspire_tween.so}
if [[ ! -f "$template_library" ]]; then
    printf 'Build the GDScript addon first, or set SPIRE_TEMPLATE_LIBRARY.\n' >&2
    exit 1
fi
mkdir -p "$template_output/project/addons/spire_tween" "$template_output/project/examples/tests" "$template_output/project/.godot" "$template_output/screenshots"
template_output=$(cd -- "$template_output" && pwd)
template_project="$template_output/project"
cp -a -- "$template_repo/spire_tween_gdscript/addons/spire_tween/templates" "$template_project/addons/spire_tween/"
cp -a -- "$template_repo/spire_tween_gdscript/examples/effect_templates" "$template_project/examples/"
cp -- "$template_repo/spire_tween_gdscript/examples/tests/scene_templates_runner.gd" "$template_repo/spire_tween_gdscript/examples/tests/scene_templates_editor.gd" "$template_project/examples/tests/"
cp --reflink=auto -- "$template_library" "$template_project/libspire_tween.so"
cat > "$template_project/project.godot" <<'EOF'
config_version=5
[application]
config/name="Spire scene template integration"
run/flush_stdout_on_print=true
[display]
window/size/viewport_width=800
window/size/viewport_height=300
[rendering]
renderer/rendering_method="gl_compatibility"
EOF
cat > "$template_project/spire_tween.gdextension" <<'EOF'
[configuration]
entry_symbol="gdext_rust_init"
compatibility_minimum="4.4"
[libraries]
linux.debug.x86_64="res://libspire_tween.so"
linux.release.x86_64="res://libspire_tween.so"
EOF
printf 'res://spire_tween.gdextension\n' > "$template_project/.godot/extension_list.cfg"
unset DISPLAY WAYLAND_DISPLAY XAUTHORITY
export XDG_DATA_HOME="$template_output/xdg/data" XDG_CONFIG_HOME="$template_output/xdg/config" XDG_CACHE_HOME="$template_output/xdg/cache"
export LIBGL_ALWAYS_SOFTWARE=true LP_NUM_THREADS=2 OMP_NUM_THREADS=2
Xvfb -displayfd 3 -screen 0 1440x1000x24 -nolisten tcp \
    3>"$template_output/display-number" >"$template_output/xvfb.log" 2>&1 </dev/null &
template_display_pid=$!
trap 'kill "$template_display_pid" 2>/dev/null || true' EXIT
for _attempt in {1..100}; do
    [[ -s "$template_output/display-number" ]] && break
    sleep 0.05
done
read -r template_display < "$template_output/display-number"
export DISPLAY=":$template_display"
timeout -k 5s 45s "$template_engine" --editor --path "$template_project" --rendering-method gl_compatibility --audio-driver Dummy --import > "$template_output/import.log" 2>&1
if rg -n 'SCRIPT ERROR:|^ERROR:|\[panic' "$template_output/import.log"; then exit 1; fi
timeout -k 5s 90s "$template_engine" --path "$template_project" --rendering-method gl_compatibility --audio-driver Dummy \
    --script res://examples/tests/scene_templates_runner.gd -- --output="$template_output/screenshots" > "$template_output/runtime.log" 2>&1
if rg -n 'SCRIPT ERROR:|^ERROR:|\[panic' "$template_output/runtime.log"; then exit 1; fi
rg '^SCENE_TEMPLATE_RESULT total=33 failed=0$' "$template_output/runtime.log"
mkdir -p "$template_project/addons/template_probe"
cp -- "$template_project/examples/tests/scene_templates_editor.gd" "$template_project/addons/template_probe/probe.gd"
cat > "$template_project/addons/template_probe/plugin.cfg" <<'EOF'
[plugin]
name="Scene template Inspector integration"
description="Tests supplied GDScript templates in the real editor"
author="SpireTween"
version="1"
script="probe.gd"
EOF
cat >> "$template_project/project.godot" <<'EOF'
[editor_plugins]
enabled=PackedStringArray("res://addons/template_probe/plugin.cfg")
EOF
timeout -k 5s 90s "$template_engine" --editor --path "$template_project" --rendering-method gl_compatibility --audio-driver Dummy \
    > "$template_output/editor.log" 2>&1
if rg -n 'SCRIPT ERROR:|^ERROR:|\[panic' "$template_output/editor.log"; then exit 1; fi
rg '^SCENE_TEMPLATE_EDITOR_RESULT total=33 failed=0$' "$template_output/editor.log"
printf 'Scene template evidence: %s\n' "$template_output"
