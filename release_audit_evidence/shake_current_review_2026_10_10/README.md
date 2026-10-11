# Current shake implementation review — 2026-10-10

This reviews the user's corrected working-tree implementation. The probe was
rebuilt against that source in debug and release and ran with real Node2D and
Control objects in Godot 4.7.2 (API 4.6). Production implementation and installed
addon libraries were not changed. The reviewed source snapshot and artifact
hashes identify the implementation tested.

## Verified repair

The applied offset is now assigned to `next_offset` after each sample, and
synchronous finish cleanup subtracts and clears that offset. All 44 restoration
checks pass in both profiles: exact and near-endpoint processing, two/three loops
in Restart/Yoyo/Incremental, active/paused forced completion, external movement,
and replay restoration. Native finish handlers registered after construction
also observe the restored position in the signal-order probes.

## Remaining findings

1. **Restart/replay sampling counts a time reset as elapsed movement**
   (`do_shake.rs:150`). A completed one-second leg leaves `prev_update` at 1.
   Restart/replay supplies 0.01 next, and `abs(0.01 - 1)` adds 0.99 to the sampling
   accumulator. At 4 Hz (interval 0.25), the node resamples after only 0.01 seconds
   of the next leg. This reproduces for both node types. Yoyo/Incremental controls
   with continuous values correctly do not resample after that 0.01 seconds.
   Completion cleanup only resets the offset, not either clock field.

2. **Large-step backlog permits sampling with zero additional elapsed time**
   (`do_shake.rs:199`). The sampler subtracts only one interval per callback.
   At 4 Hz, `process(1.0)` leaves 0.75 seconds queued; `process(0.0)` changes the
   position by 40 units in the fixed-radius opposite-bounce probe. This is the
   current catch-up policy. If intentional, it should be an explicit behavior;
   otherwise complete elapsed intervals need consuming without spreading stale
   sampling debt over subsequent ticks.

3. **Godot finished observers run before restoration**
   (`spire_tween.rs:567`, `do_shake.rs:208`). The dispatcher emits the Godot signal
   first, then invokes native connections. With a wrapped shake tween, normal
   and forced completion handlers see a 20-unit displacement. Later native
   handlers and the caller after processing returns see the restored position.
   This isolates notification order using a single applied sample, without the
   old offset-accounting defect.

4. **Clearing finish connections removes intrinsic restoration**
   (`do_shake.rs:208`). Calling the public `finished_clear_connections()` before
   processing clears the template's cleanup as well. Completion then leaves the
   node displaced by 20 units. The template currently relies on this removable
   connection to implement its documented restoration behavior.

## Results and limits

Each profile executes 60 checks: 48 pass, 12 fail. The failures are 4 reset-clock
checks, 2 zero-delta backlog checks, 4 Godot completion-observer checks, and 2
connection-clearing checks. Both runtime processes intentionally exit 1 to
report failures; neither logs an engine error or panic. The fixed radius of 20
and vibration ratio 1 make the observations independent of random direction.
Vector comparisons allow 0.001 error for floating-point roundoff.

These are focused review checks, not a passing full integration suite or release
certification. No graphical session, fresh import, export, API 4.4 run, or other
platform test was performed here.

## Reproduction

Scratch root: `/tmp/spire-shake-review-0__fezj0`.

```sh
cargo build --offline --locked --manifest-path /tmp/spire-shake-review-0__fezj0/Cargo.toml \
  --target-dir /home/houtamelo/Documents/projects/libraries/spire_tween_asset_plugin/target
cargo build --offline --locked --release --manifest-path /tmp/spire-shake-review-0__fezj0/Cargo.toml \
  --target-dir /home/houtamelo/Documents/projects/libraries/spire_tween_asset_plugin/target
godot --headless --path /tmp/spire-shake-review-0__fezj0/project --quit-after 20
```

Point the project library symlink to the corresponding debug/release probe.
The extension list is seeded and XDG directories are isolated under the scratch
root. No installed addon library was replaced.
