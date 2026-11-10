# Rendered animation tests

`text_characters.tscn` exercises all eleven per-character constructors through
the real GDScript bridge. It renders eight labeled panels: staggered offset/fade,
BBCode color, X/Y/Z rotation, scale, punch and shake on all three channels,
sequence composition, and continued playback after a text rebuild removes an index.

On Linux x86_64, install Godot 4.4+, Xvfb, FFmpeg, ripgrep and util-linux. Build
the current addon with the bridge enabled, then run from the repository root:

```sh
CARGO_BUILD_JOBS=1 nice -n 10 cargo build --locked -j 1 -p spire_tween \
  --features api-4-4,include_gdscript_bridge,include_gdext_lib
bash scripts/render_text_characters.sh
```

The script defaults to the downloaded 4.4 engine under `dist/engine-4.4/editor/`
and `target/debug/libspire_tween.so`. Override these using `GODOT_BIN` and
`SPIRE_TWEEN_LIB`. An optional first argument selects a new output directory;
existing directories are rejected so previous captures remain intact.

The output contains `text_characters.gif` (960×820, 25 FPS, ten seconds), 250
lossless PNG frames, engine logs, pixel/state checks in `result.json`, and GIF
metadata in `gif.json`. The script creates an isolated minimal Godot project,
starts a private Xvfb server, and removes host display variables before rendering.
Rendering uses Mesa software OpenGL and at most two allowed CPUs. Rendering and
palette/encoding passes run serially at lowered priority. Only its own X server
is terminated afterward. It never uses a host window or screen capture.

Each captured frame follows `RenderingServer.frame_post_draw`. Tweens use manual
Spire processing at exactly 1/25 second per frame; machine speed cannot change the
timeline. Shake uses the public API's fresh random paths, so its exact pixels
intentionally vary between runs. This is a visual integration fixture with
activity and state assertions, not a byte-for-byte golden-image comparison.

The fixture measures fourteen text regions, including each punch/shake lane
individually. It fails if any region has fewer than three distinct rendered
samples, effects fail to restore, reset leaves deformations, or a missing index
emits completion. The runner also rejects engine/script errors and a missing
success marker, checks the frame count, and decodes the generated GIF fully.

This longer rendering fixture is separate from the ordinary headless test suite;
it does not change that suite's expected counts or require a display for it.
