# Sequence child binding verification — 2026-10-13

The user's `process_tween` change validates bindings before handling stopped
children. Dead bindings stop and remove children regardless of state. Live blocked
bindings hold active children without advancing their clocks, while completed
children can pass without blocking the parent.

Six new parameterized tests in the regular `node_binding.gd` suite cover 192
configurations across method, delayed-call, property, and sequence children:

- Dead bindings on playing/paused/stopped children in append/join/insert placements,
  with and without another nesting level (72 configurations).
- Stopped children with live disabled/detached bindings remain available and do not
  block completion (24).
- Active children wait without advancing, then resume after enabling or re-entry (24).
- `PauseMode::Process` allows live disabled/detached bindings (24).
- Manual stepping treats `PauseMode::Stop` children as unpaused, even while the real
  SceneTree is paused (24).
- Lazy append/join/insert constructors discard children with dead bindings,
  including stopped children (24).

The expanded suite has 19 methods. Inserted children run independently of the
queue: a blocked insert keeps its parent playing but does not delay queue callbacks.
Fixtures explicitly preserve and check this existing distinction.

Testing found the companion pause-argument mismatch discussed during design:
`custom_step` passed `true` for `is_tree_paused`, contrary to its documented manual
override. Before correction, all other binding tests passed and the 24 Stop-mode
manual configurations failed. Changing that one argument to `false` resolves them.
The user's sequence processing implementation was not otherwise changed.

## Results

| Focused suite | Passing methods per profile |
|---|---:|
| Node binding | 19 |
| Sequence speed | 9 (121 parameter cases) |
| Sequence constructors | 18 |
| Ellipse/circle geometry and lifetime | 11 (119 parameter cases) |
| Sequence cycles | 6 |
| Ownership | 15 |
| Delayed-call validity | 9 |
| Total | 87 |

All seven suites pass in both fresh debug and release libraries, with no unexpected
script errors or native panics. Negative constructor/cycle fixtures emit their
expected diagnostics. Workspace/all-target compilation passes with six existing
native demo warnings. `final-suite-results.json` records commands and library hashes;
`manual-flag-before-results.json` records the focused failing reproduction.

Both installed Linux addon libraries were refreshed atomically. The expanded
19-method binding suite also passes in the actual debug project and through a
fixture linked to the installed release library. `installed-results.json` records
their hashes and original-library backups in `/tmp`.

```sh
godot --headless --path spire_tween_gdscript --fixed-fps 120 res://examples/tests/node_binding_runner.tscn
```

Engine checks used Godot 4.7.2, API 4.6 bindings, and existing project caches in
headless runtime mode. No editor imports, graphical runs, or other-platform builds
were performed. These are focused results, not a full aggregate-suite success claim.
