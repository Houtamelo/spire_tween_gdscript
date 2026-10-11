# Shake template contract follow-up — 2026-10-10

The user clarified that `do_shake` is a template composed from public Spire
features. Its guaranteed loop mode is Restart; clearing its callback connections
may remove its behavior. Other loop modes and survival of connection clearing
are no longer treated as required outcomes for this template.

## Current implementation

The user's shared sampling-clock state now resets `prev_update` on loop finish,
and resets both sampling clocks and the applied offset on final finish. Native
finish callbacks now run before the Godot finished signal. Real-Godot checks
confirm restored position is visible inside Godot completion handlers during
normal and forced completion, and replay starts with a clean sampling clock.

The remaining defect is the accumulator arithmetic:

```rust
*delta_time_sum -= update_interval * (*delta_time_sum % update_interval);
```

`%` returns a remainder, not the number of whole intervals. For accumulator 1.0
and interval 0.25, it subtracts zero and retains 1.0. For accumulator 0.3, it leaves
0.2875 instead of the intended remainder 0.05. This causes early samples during
ordinary playback, after a Restart boundary, and following a large tick even
when the next tick has zero delta.

## Isolated candidate

Only the accumulator line was changed in a copied core crate:

```rust
*delta_time_sum %= update_interval;
```

The production implementation was not edited. Both variants were compiled and
run headlessly on Godot 4.7.2 using API 4.6, in debug and release:

| Variant | Per-profile result |
|---|---|
| Current source | 28 checks, 22 pass, 6 cadence failures; process exits 1 |
| Isolated remainder-only candidate | 28 checks, all pass; process exits 0 |

The same cases use real Node2D and Control objects: exact/near restoration,
Restart loops, active/paused forced completion, external movement, replay
restoration, restart/replay cadence, zero delta after a large tick, ordinary
sample spacing, and restored position inside native/Godot finish callbacks.
No engine errors or panics occurred. Only Restart is exercised; connection
clearing is deliberately excluded from the clarified acceptance checks.

The 112 recorded checks compare current source and candidate across two profiles.
These focused results do not certify the broader suite or release artifacts.
Installed addon libraries were not replaced. No graphical launch, fresh import,
export, API 4.4 runtime, or other platform test was performed.

## Reproduction

Scratch root: `/tmp/spire-shake-template-followup-c_wicb8j`.

```sh
cargo build --offline --locked --manifest-path /tmp/spire-shake-template-followup-c_wicb8j/Cargo.toml \
  --target-dir /home/houtamelo/Documents/projects/libraries/spire_tween_asset_plugin/target
cargo build --offline --locked --release --manifest-path /tmp/spire-shake-template-followup-c_wicb8j/Cargo.toml \
  --target-dir /home/houtamelo/Documents/projects/libraries/spire_tween_asset_plugin/target
cargo build --offline --locked --manifest-path /tmp/spire-shake-template-followup-c_wicb8j/candidate_probe/Cargo.toml \
  --target-dir /home/houtamelo/Documents/projects/libraries/spire_tween_asset_plugin/target
cargo build --offline --locked --release --manifest-path /tmp/spire-shake-template-followup-c_wicb8j/candidate_probe/Cargo.toml \
  --target-dir /home/houtamelo/Documents/projects/libraries/spire_tween_asset_plugin/target
godot --headless --path /tmp/spire-shake-template-followup-c_wicb8j/project --quit-after 20
godot --headless --path /tmp/spire-shake-template-followup-c_wicb8j/candidate_project --quit-after 20
```

Select each profile by pointing the corresponding project library symlink to that
profile's probe. Extension lists are seeded, with isolated XDG directories.
