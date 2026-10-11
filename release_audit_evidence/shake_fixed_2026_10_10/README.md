# Verified shake repair — 2026-10-10

The user's current production source uses remainder-only accumulator consumption
(`*delta_time_sum %= update_interval`), shared sampling-clock reset connections,
correct applied-offset bookkeeping, and synchronous restoration before the Godot
finished signal. This verification rebuilt against the actual working-tree
source, not the previous isolated candidate.

Both debug and release probes pass all 28 focused real-Godot checks on Godot
4.7.2 with API 4.6. Both processes exit 0 without engine errors or panics. The
cases cover Node2D and Control, exact/near completion, two/three Restart loops,
active/paused forced completion, external movement, replay restoration,
restart/replay cadence, a zero-delta tick after a large step, ordinary sample
spacing, and native/Godot completion handlers observing restored position.

The accepted contract is a Restart-only template composed from public Spire
features. Other loop modes and survival of clearing the template's connections
are not guaranteed. Finding 7's confirmed restoration defect is repaired under
this contract; prior investigation logs remain historical evidence.

This does not certify the full integration suite, first editor import, export,
other platform artifacts, or the installed addon binaries. No graphical session
was launched and no installed addon library was replaced. Source implementation
was not edited by this verification.

Scratch root: `/tmp/spire-shake-template-followup-c_wicb8j`.

```sh
cargo build --offline --locked --manifest-path /tmp/spire-shake-template-followup-c_wicb8j/Cargo.toml \
  --target-dir /home/houtamelo/Documents/projects/libraries/spire_tween_asset_plugin/target
cargo build --offline --locked --release --manifest-path /tmp/spire-shake-template-followup-c_wicb8j/Cargo.toml \
  --target-dir /home/houtamelo/Documents/projects/libraries/spire_tween_asset_plugin/target
godot --headless --path /tmp/spire-shake-template-followup-c_wicb8j/project --quit-after 20
```

Point the project library symlink to the corresponding debug/release probe.
The extension list is seeded; XDG directories are isolated under the scratch root.
