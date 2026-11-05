#!/usr/bin/env bash
# Linux rendered integration fixture; deliberately never connects to a host display.
set -euo pipefail

repo=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
godot=${GODOT_BIN:-"$repo/dist/engine-4.4/editor/Godot_v4.4-stable_linux.x86_64"}
library=${SPIRE_TWEEN_LIB:-"$repo/target/debug/libspire_tween.so"}
output=${1:-"$repo/dist/rendered-tests/text-characters-$(date +%Y%m%d-%H%M%S)"}
for tool in Xvfb ffmpeg ffprobe timeout taskset nice rg; do
    command -v "$tool" >/dev/null || { printf 'Missing dependency: %s\n' "$tool" >&2; exit 1; }
done
[[ -x "$godot" ]] || { printf 'Set GODOT_BIN to a Godot 4.4+ editor binary.\n' >&2; exit 1; }
[[ -f "$library" ]] || { printf 'Build the GDScript bridge first; see examples/render_tests/README.md.\n' >&2; exit 1; }
[[ ! -e "$output" ]] || { printf 'Output already exists; choose a fresh directory: %s\n' "$output" >&2; exit 1; }
mkdir -p -- "$output"
output=$(cd -- "$output" && pwd)
project="$output/project"
mkdir -p "$project/examples/render_tests" "$project/addons/spire_tween/lib/x86_64-unknown-linux-gnu/debug" \
    "$project/godot" "$output/frames" "$output/xdg/config" "$output/xdg/cache" "$output/xdg/data"
cp -- "$repo/spire_tween_gdscript/examples/render_tests/text_characters.gd" \
    "$repo/spire_tween_gdscript/examples/render_tests/text_characters.tscn" "$project/examples/render_tests/"
cp -- "$repo/spire_tween_gdscript/addons/spire_tween/spire_tween.gdextension" "$project/addons/spire_tween/"
cp --reflink=auto -- "$library" "$project/addons/spire_tween/lib/x86_64-unknown-linux-gnu/debug/libspire_tween.so"
cat > "$project/project.godot" <<'PROJECT'
config_version=5

[application]
config/name="Spire rendered character test"
config/use_hidden_project_data_directory=false
run/main_scene="res://examples/render_tests/text_characters.tscn"
boot_splash/show_image=false
run/flush_stdout_on_print=true

[display]
window/size/viewport_width=960
window/size/viewport_height=820
window/vsync/vsync_mode=0

[rendering]
renderer/rendering_method="gl_compatibility"
renderer/rendering_method.mobile="gl_compatibility"
threads/thread_model=1
PROJECT
# Loading the extension directly avoids an editor import pass for this asset-free fixture.
printf 'res://addons/spire_tween/spire_tween.gdextension\n' > "$project/godot/extension_list.cfg"

# Respect the invoking process's affinity; select at most its first two allowed CPUs.
allowed=$(awk '/Cpus_allowed_list:/ {print $2}' /proc/self/status)
cpus=()
IFS=',' read -ra ranges <<< "$allowed"
for range in "${ranges[@]}"; do
    first=${range%-*}
    last=${range#*-}
    for ((cpu=first; cpu<=last && ${#cpus[@]}<2; cpu++)); do cpus+=("$cpu"); done
    ((${#cpus[@]} >= 2)) && break
done
cpu_list=$(IFS=,; printf '%s' "${cpus[*]}")
limited=(nice -n 10 taskset -c "$cpu_list")

unset DISPLAY WAYLAND_DISPLAY XAUTHORITY
export XDG_CONFIG_HOME="$output/xdg/config" XDG_CACHE_HOME="$output/xdg/cache" XDG_DATA_HOME="$output/xdg/data"
export LIBGL_ALWAYS_SOFTWARE=true GALLIUM_DRIVER=llvmpipe LP_NUM_THREADS=2 OMP_NUM_THREADS=2
if ! "${limited[@]}" timeout 30 "$godot" --headless --path "$project" --check-only \
    --script res://examples/render_tests/text_characters.gd >"$output/parse.log" 2>&1; then
    cat "$output/parse.log" >&2
    exit 1
fi
xvfb_pid=''
cleanup() {
    if [[ -n "$xvfb_pid" ]]; then
        kill "$xvfb_pid" 2>/dev/null || true
        wait "$xvfb_pid" 2>/dev/null || true
    fi
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
"${limited[@]}" Xvfb -displayfd 3 -screen 0 1024x900x24 -nolisten tcp -noreset -nocursor \
    3>"$output/display-number" >"$output/xvfb.log" 2>&1 &
xvfb_pid=$!
for ((attempt=0; attempt<100; attempt++)); do
    [[ -s "$output/display-number" ]] && break
    kill -0 "$xvfb_pid" 2>/dev/null || { cat "$output/xvfb.log" >&2; exit 1; }
    sleep 0.1
done
display_number=$(<"$output/display-number")
[[ "$display_number" =~ ^[0-9]+$ ]] || { printf 'Xvfb did not become ready.\n' >&2; exit 1; }
export DISPLAY=":$display_number"
printf 'Rendering on private Xvfb %s, CPUs %s, software OpenGL.\n' "$DISPLAY" "$cpu_list"
if ! "${limited[@]}" timeout 180 "$godot" --path "$project" --display-driver x11 \
    --rendering-method gl_compatibility --rendering-driver opengl3 --audio-driver Dummy \
    --single-threaded-scene --render-thread safe --fixed-fps 25 --max-fps 25 --disable-vsync \
    --log-file "$output/engine.log" -- --frames-dir="$output/frames" >"$output/render.log" 2>&1; then
    cat "$output/render.log" >&2
    exit 1
fi
cleanup
xvfb_pid=''
if rg -n 'SCRIPT ERROR:|^ERROR:|Parse Error:' "$output/render.log"; then
    printf 'Godot reported errors; refusing to encode an unsuccessful render.\n' >&2
    exit 1
fi
rg -q '^RENDER_TEST_RESULT name=text_characters frames=250 panels=8 samples=14 failed=0 fps=25$' "$output/render.log"
shopt -s nullglob
frames=("$output"/frames/frame_*.png)
[[ ${#frames[@]} -eq 250 ]] || { printf 'Expected 250 PNG frames, got %d.\n' "${#frames[@]}" >&2; exit 1; }

# Two separate passes avoid buffering the entire animation in a split/palette filter.
"${limited[@]}" timeout 120 ffmpeg -nostdin -hide_banner -loglevel warning -threads 2 -filter_threads 1 \
    -framerate 25 -i "$output/frames/frame_%04d.png" -vf 'palettegen=stats_mode=diff' \
    -frames:v 1 -threads 1 -update 1 "$output/palette.png" >"$output/encode.log" 2>&1
"${limited[@]}" timeout 120 ffmpeg -nostdin -hide_banner -loglevel warning -threads 2 -filter_complex_threads 1 \
    -framerate 25 -i "$output/frames/frame_%04d.png" -i "$output/palette.png" \
    -lavfi 'paletteuse=dither=bayer:bayer_scale=3:diff_mode=rectangle' -loop 0 -threads 1 \
    "$output/text_characters.gif" >>"$output/encode.log" 2>&1
"${limited[@]}" ffprobe -v error -threads 1 -count_frames -select_streams v:0 \
    -show_entries stream=width,height,nb_read_frames:format=duration,size \
    -of json "$output/text_characters.gif" > "$output/gif.json"
"${limited[@]}" timeout 30 ffmpeg -nostdin -hide_banner -loglevel error -threads 1 \
    -i "$output/text_characters.gif" -f null - >>"$output/encode.log" 2>&1
printf 'GIF ready: %s\n' "$output/text_characters.gif"
cat "$output/result.json"
