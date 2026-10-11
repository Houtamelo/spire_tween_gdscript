# Yoyo and Incremental shake verification — 2026-10-09

The production implementation remains unchanged. Two isolated native extensions
were built: one against the current working tree, one against a copied core crate
with exactly the two proposed assignments in the restoration branch. Both ran as
headless runtime scenes on Godot 4.7.2, with API 4.6, in the optimized debug
profile. Installed addon libraries were not replaced.

The probe uses real Node2D and Control instances, public native tween APIs, a
one-second duration, radius fixed at 20, vibration ratio 1, and frequency 30.
Randomness sets the initial direction; subsequent opposite offsets make residual
lengths deterministic apart from small floating-point errors. The original shake
callable is wrapped only to record its unchanged input. Observations use exact
and near-endpoint stepping, two/three loops, and forced completion during loop 2.

## Mode-specific causes

`calc_animation_position` reverses the normalized value on odd Yoyo loops, and
adds the completed loop count in Incremental mode. Because the shake method tween
lerps from 0 to the original duration, its callback receives **an animated value,
not monotonically increasing elapsed time**.

- Yoyo loop 2 supplies values such as 0.75, 0.5, 0.25, 0. At decreasing values,
  `time - prev_update < update_interval` suppresses sampling. Completion at zero
  also misses the `time.approx_eq(duration)` restoration branch.
- Incremental loop 2 supplies values such as 1.25, 1.5, 2. The later loop endpoint
  differs from the captured original duration, so the shake callback samples at
  completion rather than restoring.
- `force_complete` invokes the method endpoint 1 independently of loop mode/count;
  that still reaches the restoration branch. With the two assignments, forced
  completion restores correctly in both modes.

## Representative outcomes (both node types)

| Case | Current Yoyo | Proposal Yoyo | Current Incremental | Proposal Incremental |
|---|---|---|---|---|
| One loop, 0.5 + 0.5 | Restores | Restores | Restores | Restores |
| Two loops, half-second ticks | Final position zero but reverse loop never samples | Leaves offset of 20 | Final position happens to cancel with opposite offsets | Leaves offset of 20 |
| First loop 0.5 + 0.5; second 0.25 + 0.25 + 0.5 | Leaves residual 40 | Leaves residual 20 | Leaves residual 40 | Leaves residual 20 |
| Near-endpoint cleanup in first loop | Leaves residual 60 | Leaves residual 20 | Leaves residual 60 | Leaves residual 20 |
| Three loops | Leaves residual 20 | Final position zero, but reverse sampling remains broken | Leaves residual 40 | Leaves residual 20 |
| Force completion during second loop | Leaves residual 20 | Restores | Leaves residual 20 | Restores |

Four quarter-second second-loop steps were also checked. Yoyo samples only once
in that reverse loop. All 56 observations (7 cases x 2 modes x 2 node types x 2
source variants) matched the recorded predictions within 0.001. These are defect
observations, not 56 successful restoration regressions. Both runs exited 0 with
no engine error or panic messages. No release-profile run, graphical launch,
editor import, or export was performed in this investigation.

## Consequence for the proposed fix

Clearing `prev_offset` and `prev_update` is sufficient for the previously
investigated Restart/replay bookkeeping cases. It is insufficient for fully
supporting Yoyo and Incremental: sampling must handle the mode's value direction,
and restoration must recognize the actual loop endpoint rather than only the
original forward value. No expanded fix is implemented by this investigation.

## Reproduction

Scratch root: `/tmp/spire-shake-loop-modes-d8uxmbjt`.

```sh
cargo build --offline --locked --manifest-path /tmp/spire-shake-loop-modes-d8uxmbjt/Cargo.toml \
  --target-dir /home/houtamelo/Documents/projects/libraries/spire_tween_asset_plugin/target
godot --headless --path /tmp/spire-shake-loop-modes-d8uxmbjt/project --quit-after 20
cargo build --offline --locked --manifest-path /tmp/spire-shake-loop-modes-d8uxmbjt/candidate_probe/Cargo.toml \
  --target-dir /home/houtamelo/Documents/projects/libraries/spire_tween_asset_plugin/target
godot --headless --path /tmp/spire-shake-loop-modes-d8uxmbjt/candidate_project --quit-after 20
```

Each project seeds its extension list to load only the corresponding probe.
XDG config/cache/data directories are isolated under the scratch root. Source and
binary hashes, the exact proposal diff, and full callback/position logs are saved.
