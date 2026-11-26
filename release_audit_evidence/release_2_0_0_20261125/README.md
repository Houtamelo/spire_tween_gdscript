# SpireTween 2.0.0 local release validation

The Linux candidate passed validation on Godot 4.4 and 4.7.2. Windows and macOS
validation, including Intel execution of the universal macOS showcase, still
require a successful hosted workflow before publishing the complete desktop
downloads. These local archives are explicitly development candidates built
from the dirty working tree based on `713da10bf3ef`.

## CI repairs

The previous [workflow run](https://github.com/Houtamelo/spire_tween_gdscript/actions/runs/45508802427)
failed Linux graphical validation because `rg` was absent. Its template runtime
had actually completed all 33 cases. The workflow now explicitly installs
ripgrep, Xvfb and the Mesa software-rendering package.

macOS native cases failed fixed 0.017-second timing assertions. A local 30 FPS
run reproduced `test_pause_play_works` failing by 0.02943 seconds. Running the
native aggregate at fixed 120 FPS passed every ordinary case, but correctly
failed `test_ignore_time_scale`: that case intentionally measures wall time.

Release validation therefore runs 188 native simulation cases at fixed 120 FPS
and the one unscaled-time case with real pacing. Both profiles must pass both
groups. The native runner supports excluding a named test, and an intentional
empty-filter invocation must fail. All 189 cases remain required; production
tween clocks and the one-loop-per-update policy are unchanged.

## Passed checks

| Check | Godot 4.4 | Godot 4.7.2 |
|---|---|---|
| GDScript aggregate, debug and release | 343 each | 343 each |
| Native simulation aggregate, debug and release | 188 each | 188 each |
| Native wall-clock case, debug and release | 1 each | 1 each |
| Runner pass/fail controls, including empty filter | Passed | Passed |
| Template runtime and Inspector | 33 + 33 | 33 + 33 |
| Preview runtime and editor restoration | 2 + 228 | 2 + 228 |
| Rendered presets/characters, typed-array rejection and editor | Passed (66 rendered cases) | Passed (66 rendered cases) |
| Extracted addon debug/release installation and both game exports | Passed | Passed |

The source-archive test compiled the extracted workspace with locked Cargo inputs
and rejected a missing `spire_tween/src/lib.rs` even after ZIP checksums were
regenerated. The exported Godot 4.4 showcase exercised Play and Restore on all
33 actual gallery cards, and its failure control rejected an app missing its
real PCK file with valid checksums. The missing-PCK process required its bounded
deadline to terminate; the failure was not treated as a successful launch.

All three existing real-artifact pipeline tests passed, including missing native
libraries, staged template inventory, valid-checksum missing-script failure, and
extra-file rejection. Release-tool CLI checks, formatting for that tool,
actionlint 1.7.12, shell syntax and source-only `git diff --check` passed. Raw
engine logs retain their original whitespace.

The exported showcase was also rendered on a private Xvfb display with software
OpenGL. Its title, instructions, text cards, controls and scrolling layout were
inspected. No host display or installed example addon was changed.

## Candidate downloads and provenance

The reviewable local files are in `dist/release-2.0.0-local-linux/`:

- `SpireTween-2.0.0-713da10bf3ef-candidate-linux.zip`
- `SpireTween-2.0.0-713da10bf3ef-candidate-source.zip`
- `SpireTween-2.0.0-713da10bf3ef-candidate-demo-linux.zip`
- `showcase.png`

`SHA256SUMS`, `build.json`, engine validation reports, archive-validation reports,
demo-validation metadata, and representative raw logs are retained alongside
this report. The initial 30 FPS failure log is deliberate reproduction evidence.
The final hosted artifacts must be rebuilt from the committed source; these
Linux-only candidate files do not establish a complete desktop release.
