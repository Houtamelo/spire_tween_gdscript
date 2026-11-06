#!/usr/bin/env bash
set -euo pipefail

if (( $# != 4 )); then
  printf 'Usage: bash run_probe.sh GODOT_BINARY LIBRARY VARIANT headless|gui\n' >&2
  exit 2
fi

godot_binary=$(realpath "$1")
library=$(realpath "$2")
variant=$3
mode=$4
if [[ ! "$variant" =~ ^[a-z0-9_]+$ ]]; then
  printf 'Variant must contain only lowercase letters, digits, and underscores.\n' >&2
  exit 2
fi
evidence_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_dir=$(git -C "$evidence_dir" rev-parse --show-toplevel)
work_dir="$repo_dir/target/editor_controller_probe_2026_11_03"
run_name="${variant}_${mode}"
mkdir -p "$work_dir/projects" "$work_dir/xdg/$run_name/data" "$work_dir/xdg/$run_name/config" "$work_dir/xdg/$run_name/cache"
project_dir=$(mktemp -d "$work_dir/projects/$run_name.XXXXXX")
cp -a "$evidence_dir/harness/." "$project_dir/"
ln -sfn "$library" "$project_dir/libspire_tween.so"
export XDG_DATA_HOME="$work_dir/xdg/$run_name/data"
export XDG_CONFIG_HOME="$work_dir/xdg/$run_name/config"
export XDG_CACHE_HOME="$work_dir/xdg/$run_name/cache"
export SPIRE_PROBE_OUTPUT="$evidence_dir/$run_name.jsonl"
export SPIRE_PROBE_VARIANT="$variant"
export SPIRE_EXPECT_TICKS=1
if [[ "$variant" == no_tool ]]; then
  export SPIRE_EXPECT_TOOL=0
else
  export SPIRE_EXPECT_TOOL=1
fi

editor_args=(--editor --path "$project_dir" --audio-driver Dummy --log-file "$evidence_dir/$run_name.log")
if [[ "$mode" == headless ]]; then
  editor_args+=(--headless)
elif [[ "$mode" == gui ]]; then
  display_file="$work_dir/$run_name.display"
  : > "$display_file"
  Xvfb -displayfd 3 -screen 0 1280x800x24 -nolisten tcp 3>"$display_file" >"$evidence_dir/$run_name.xvfb.log" 2>&1 &
  xvfb_pid=$!
  trap 'kill "$xvfb_pid" 2>/dev/null || true; wait "$xvfb_pid" 2>/dev/null || true' EXIT
  for ((attempt=0; attempt<100; attempt++)); do
    if [[ -s "$display_file" ]]; then break; fi
    sleep 0.05
  done
  read -r display_number < "$display_file"
  export DISPLAY=":$display_number"
  export LIBGL_ALWAYS_SOFTWARE=1
  editor_args+=(--rendering-method gl_compatibility --rendering-driver opengl3)
else
  printf 'Unknown mode: %s\n' "$mode" >&2
  exit 2
fi

timeout 45 "$godot_binary" "${editor_args[@]}" >"$evidence_dir/$run_name.stdout.log" 2>&1
