#!/usr/bin/env bash
# Real-engine assertions for normalization diagnostics. Build/install the addon first.
set -euo pipefail
if [[ $# != 2 ]]; then
  printf 'Usage: bash %s GODOT_EXECUTABLE IMPORTED_GDSCRIPT_PROJECT\n' "$0" >&2
  exit 2
fi
spire_warnings_dir=$(mktemp -d "${TMPDIR:-/tmp}/spire-template-warnings.XXXXXX")
trap 'rm -rf -- "$spire_warnings_dir"' EXIT
spire_warnings_log="$spire_warnings_dir/output.log"
if ! timeout 60 "$1" --headless --log-file "$spire_warnings_dir/engine.log" \
  --path "$2" --fixed-fps 120 res://examples/tests/test_runner.tscn -- \
  --test=test_group3_invalid_and_cancellation --no-cache > "$spire_warnings_log" 2>&1; then
  cat "$spire_warnings_log"
  exit 1
fi
rg -q '^GDSCRIPT_TEST_RESULT total=1 failed=0 skipped=0$' "$spire_warnings_log"
spire_warnings_checks=1
while IFS= read -r spire_warning_expected; do
  if ! rg -Fq -- "$spire_warning_expected" "$spire_warnings_log"; then
    printf 'Missing warning: %s\n' "$spire_warning_expected" >&2
    cat "$spire_warnings_log"
    exit 1
  fi
  spire_warnings_checks=$((spire_warnings_checks + 1))
done <<'EXPECTED'
[do_shake] randomness must be in [0,180], got -12.5; clamping.
[do_shake] randomness must be in [0,180], got 245.75; clamping.
[spire template] elasticity must be in [0, 1], got -0.25; clamping.
[spire template] elasticity must be in [0, 1], got 1.25; clamping.
[spire effect] duration must be finite and positive, got -2.5; using 1 second.
[spire effect] duration must be finite and positive, got NaN; using 1 second.
[spire template] randomness must be finite, got NaN; using 90.
[spire spiral] frequency must be finite and positive, got -3.25; using 10.
[spire template] spiral speed must be finite, got NaN; using 1.
[spire template] spiral depth must be finite, got NaN; using 0.
EXPECTED
for spire_warning_prefix in 'spire spiral] axis' 'spire rotation] quaternion' 'spire look-at] up'; do
  rg -q "${spire_warning_prefix} must be finite and nonzero, got [^;]+;" "$spire_warnings_log"
  spire_warnings_checks=$((spire_warnings_checks + 1))
done
awk '/^TEMPLATE_WARNINGS_VALID_BEGIN$/ { valid=1; begin=1; next }
     /^TEMPLATE_WARNINGS_VALID_END$/ { valid=0; end=1; next }
     /WARNING:/ { if (valid) exit 1 }
     /SCRIPT ERROR|TEST_FAIL|panicked at|ObjectDB|resources still|Leaked instance:/ { exit 1 }
     END { if (!begin || !end) exit 1 }' "$spire_warnings_log"
printf 'TEMPLATE_WARNINGS_RESULT checks=%s failed=0\n' "$((spire_warnings_checks + 1))"
