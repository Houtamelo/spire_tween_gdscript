# SpireTween

Copy the archive's `addons/spire_tween` directory into your Godot project's `addons` directory. Open or reopen the project. No plugin switch or autoload is needed.

The desktop release targets Godot 4.4 and newer supported Godot 4.x versions, Windows/Linux x86_64, and macOS Apple Silicon/Intel. A partial development candidate contains only the platforms named in its filename and extension manifest. Both editor/debug and exported release libraries are included for each packaged platform. `release.json` records the configured validation versions, available validation reports, source revision, Linux GLIBC requirement, macOS deployment target, and signing status. An archive with `candidate` in its name is a development candidate; consult its validation evidence before release. macOS candidates use ad-hoc signing and do not establish the normal downloaded-customer experience.

Attach this script to a scene Node and run it:

```gdscript
extends Node

func _ready() -> void:
    Spire.do_delayed_call(func(): print("SpireTween loaded"), 0.1)
```

SpireTween requires an installed `SceneTree` with an available root before the manager's first use. A scene Node's `_ready()` satisfies this requirement. In custom main-loop code, defer tween creation until the main loop has been installed. Initialization failure prints an error and does not retry; correct the startup call and restart the application.

Godot selects the appropriate library during export. Keep the whole addon directory, including its debug library, in your project. Public classes and methods include documentation in the editor's built-in help.

See the [project documentation and examples](https://github.com/Houtamelo/spire_tween_gdscript) for usage. The exact source revision and file hashes are in `release.json`; the build lockfile/toolchain snapshot is in `build/`. Examples and third-party art are not part of this archive.

`SHA256SUMS` records the SHA-256 hash of each delivered file except the checksum manifest itself. The project is MIT licensed; see `LICENSE`. Dependency notices, original license files and unmodified MPL dependency sources are under `third_party/<platform>/`.
