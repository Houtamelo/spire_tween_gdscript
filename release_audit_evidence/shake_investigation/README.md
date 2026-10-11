# Shake restoration investigation — 2026-10-09

This is a cause investigation of finding 7, with no production source changes.
The isolated native extension was rebuilt against the current working-tree
`spire_tween` and run headlessly in Godot 4.7.2 using API 4.6. It exercises real
Node2D/Control objects and public native tween APIs. A wrapper around the original
shake callable logs its argument without changing its value or owner. The probe
uses a fixed radius of 20; direction is random, but each assertion derives the
expected residual from the offset actually applied in that run.

## Cause

`do_shake.rs:154` treats approximate equality between its interpolated time
argument and the captured duration as restoration. It subtracts `prev_offset`
from the node but leaves that offset recorded as applied. Method tween processing
only completes when `loop_time - duration >= 0` (`method/data.rs:153`). Thus an
approximately final callback can restore while the tween remains Playing, and a
subsequent final callback subtracts the same offset again. The recorded offset
no longer describes the offset on the node.

The callable also retains `prev_update` across completion, whereas method tween
looping and replay reset their native elapsed counters. The next run can skip
updates until its local time exceeds the previous run's update timestamp; its
restoration still subtracts the old offset. Both Node2D and Control use the same
helper. This extends the investigation to Restart loops and replay; other loop
modes/custom easing were not runtime-tested here.

## Observations

| Case | Observed behavior |
|---|---|
| Exact 0.5 + 0.5 completion | Restores to zero |
| 0.5 + 0.499999999 + 0.01 | Restores while Playing, then finishes at minus the last offset |
| 0.5 + 0.499995 + 0.000001 + 0.01 | Three restoration calls; final error has magnitude 40 |
| Forced completion at 0.5 | Restores to zero |
| Forced completion after the near-endpoint cleanup | Finishes at minus the last offset |
| Two Restart loops, each 0.5 + 0.5 | Second midpoint produces no shake; final error magnitude 20 |
| Replay after exact completion | Replay midpoint produces no shake; final error magnitude 20 |
| External movement after early cleanup | Final position is external movement minus the stale offset |
| Control near-endpoint completion | Same residual-offset behavior as Node2D |

All nine observation assertions matched the predicted cause; this confirms the
current defect, not successful restoration. The runtime exited 0, with no engine
errors or panic messages. Only a headless runtime scene was launched. The seeded
extension list bypasses editor import and makes no first-import claim.

## Commands

Scratch crate: `/tmp/spire-shake-investigation-6r0uax4m`.

```sh
cargo build --offline --locked \
  --manifest-path /tmp/spire-shake-investigation-6r0uax4m/Cargo.toml \
  --target-dir /home/houtamelo/Documents/projects/libraries/spire_tween_asset_plugin/target
godot --headless --path /tmp/spire-shake-investigation-6r0uax4m/project --quit-after 20
```

Runtime XDG config/cache/data directories were isolated under the scratch crate.
The project manifest references the investigation extension; installed addon
libraries were not replaced.
