# The editor root controller survives scene changes

Godot 4.7.2 keeps the same editor SceneTree, root Window, and native
TweensController while scenes are opened, switched, closed, and reopened. No
controller reattachment was needed. The initial experiments left production source
unchanged; the subsequent approved change adds `tool` to the production controller.

| Operation | Observed behavior |
|---|---|
| First tween with no saved scene open | Creates `/root/tweens_controller`; editor main loop is a SceneTree; `current_scene` is null. |
| Open A, then B | The editor root and controller stay the same. A is detached but remains alive in its background tab. |
| Leave B active | Both controller callbacks keep running; A's default node-bound position tween stops advancing. |
| Switch back to A | The same A instance re-enters the tree and its tween resumes. |
| Close A | A is freed; its tween emits `stopped` exactly once and does not emit `finished`. |
| Close every saved scene | The editor leaves an empty tab; root and controller survive and keep ticking. |
| Reopen A | A has a new instance ID; the original controller drives new sequences. |
| Save B | The root controller is not serialized into the edited scene. |

Both `#[class(init, base = Node, internal)]` and the same declaration with `tool`
passed. Each was tested in a headless editor and a rendered X11 editor using
Xvfb and Mesa llvmpipe. The libraries were built from isolated current source
copies with godot-rust 0.5.5, API 4.4, and the GDScript bridge enabled. This is
Godot 4.7.2 runtime evidence, not a Godot 4.4 runtime test.

The experimental code only adds process/physics callback counters; the second
variant additionally adds `tool`. The actual singleton initialization, attachment,
processing, property tween, and lifetime handling remain SpireTween's code.

The non-tool result is specific to the current construction path. Installed
godot-core 0.5.5 `src/obj/gd.rs:575-630` explains that `new_alloc()` currently
constructs a real native object in the editor even for a non-tool class, and
documents a planned v0.6 change to placeholder construction. See the
[upstream source](https://docs.rs/godot-core/0.5.5/src/godot_core/obj/gd.rs.html).
Explicit `tool` remains appropriate for an editor controller.

The follow-up regression probe also creates a controller through
`ClassDB.instantiate()`, which exercises tool registration rather than bypassing
placeholders through `new_alloc()`. The tool instance runs its native `_ready()`
and sets `ALWAYS` mode plus maximum process and physics priorities. The non-tool
placeholder does not. The temporary instance is removed before the next frame so
it cannot tick the global manager twice. This detects a missing `tool` declaration
even while internally constructed controllers happen to work without it.

The same harness accepts an uninstrumented production library; playback and target
lifetime checks remain mandatory while diagnostic-counter assertions are skipped.
For example, pass `target/debug/libspire_tween.so production gui` to the runner.

The `scene_closed` signal arrived before the closed node was actually freed;
the harness waits for deferred deletion before checking validity. Preview cleanup
should therefore use explicit lifecycle handling rather than treating that signal
as proof that the target is already destroyed. The documented editor hooks are
[scene_changed and scene_closed](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html#signals).

To reproduce from the repository root:

```bash
node release_audit_evidence/editor_controller_2026_11_03/build_variants.mjs
bash release_audit_evidence/editor_controller_2026_11_03/run_probe.sh \
  /home/houtamelo/bin/godot \
  target/editor_controller_probe_2026_11_03/artifacts/tool/libspire_tween.so tool gui
```

Use `no_tool` for the other library and variant; use `headless` for the other
display mode. Each run creates an isolated project, writes JSONL observations
and editor logs here, and exits unsuccessfully if an assertion fails. XDG data,
config, and cache are redirected under the ignored build directory. Headless runs
serialize a PackedScene directly to avoid the dummy renderer's thumbnail error;
rendered runs exercise `EditorInterface.save_scene()`.

`summary.json` records four comparison runs plus two follow-up runs against the
production library. The production build passes the tool-registration check in
both headless and rendered editors. Running the same check against the non-tool
library fails specifically on tool registration, as expected; its log is named
`without_tool_expected_failure_headless`.

These checks cover lifecycle and basic playback, not preview restoration,
arbitrary method side effects, extension hot reload, or physics simulation
behavior in the editor. The rendered runs emit
Xvfb input-method/vsync warnings; godot-rust emits its signal auto-disconnect
notice during shutdown. No engine errors or failed assertions occurred in the
accepted runs.
