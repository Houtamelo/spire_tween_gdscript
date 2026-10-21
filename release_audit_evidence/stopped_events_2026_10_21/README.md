# Stopped-event lifecycle verification

The tween lifecycle now emits `stopped` exactly once for a transition from playing
or paused to stopped. Successful completion also emits `finished`; cancellation
does not. Common playback guards and terminal-result handling live on `SpireTween`,
while `SpireTweener` supplies type-specific hooks.

## Integration coverage

Added 13 native and 11 GDScript test methods to the aggregate runners. Their
parameterized cases exercise real Godot objects, callables, signals, and SceneTree
processing:

- Explicit stop, repeated stop, stopped-to-paused rejection, restart clocks, and replay.
- Natural completion ordering, intermediate loops, and forced completion while playing
  or paused, including infinite-loop tweens.
- Invalid callables, freed bindings in every playback state, and failed property completion.
- Idle/physics manager processing, manual stepping, and nested sequence processing.
- Native and Godot deferred/one-shot connections, native disconnection/clearing,
  and synchronous cleanup visible to Godot completion listeners.
- Type-erased connection APIs, stopped/paused child adoption, descendant cancellation,
  interval/callback replay, and pending constructors remaining uninvoked on stop.
- Unregistration and pointer destruction without stop events.

Compile-only regressions cover concrete builders, pointers, and all three erased
handle types. The release runner's expected aggregate counts are now 81 native
and 163 GDScript methods; existing auxiliary native suites also run.

## Results

| Godot | Profile | Native aggregate | GDScript aggregate |
|---|---|---|---|
| 4.4 | Debug | 81 passed | 163 passed |
| 4.4 | Release | 81 passed | 163 passed |
| 4.7.2 | Debug | 81 passed | 163 passed |
| 4.7.2 | Release | 81 passed | 163 passed |

Both release-validation commands exited successfully, including their installed
smoke checks and deliberate harness failure controls. See [runtime markers](results.txt)
and the [4.4](validation-4.4.json) / [4.7.2](validation-4.7.2.json) validation records.
Full local logs are under `dist/stopped-events-20261021-validation/`.

## Commands

The standard release builder produced both Linux profiles with `api-4-4`, keeping
the ordinary development default unchanged:

```sh
cargo run --locked -p spire_release -- build --platform linux \
  --out dist/stopped-events-20261021-final/linux
cargo run --locked -p spire_release -- test \
  --stage dist/stopped-events-20261021-final/linux \
  --godot /absolute/path/to/Godot --out dist/stopped-events-20261021-validation/VERSION
```

Validation runs use Godot 4.4 and 4.7.2. Native aggregate tests run in real time;
GDScript aggregates use fixed 120 FPS. The driver also runs installed-addon smoke
checks and asynchronous harness success/failure controls in each profile.

Additional checks: default-API addon/native compilation, the `must_compile` target,
doctests, scoped rustfmt, `git diff --check`, five release-tool CLI integration tests,
and rustdoc with `-D rustdoc::broken_intra_doc_links`. Two pre-existing BBCode notes
in native rustdoc were converted to Markdown so the strict link check passes.
Other existing rustdoc warnings and aggregate GDScript shutdown warnings remain.
