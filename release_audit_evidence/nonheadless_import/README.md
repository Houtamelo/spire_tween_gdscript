# Non-headless import comparison

Godot 4.7.2 on Linux, with the current installed API 4.6 addon libraries. Each minimal fixture starts without a project cache or prewritten extension list. Three separate fresh projects per library profile and mode were checked.

- Graphical X11 imports: debug 3/3 and release 3/3 exit 0.
- Matching headless imports: debug 3/3 and release 3/3 terminate with SIGABRT (exit -6), after the engine reports signal 11.
- Empty graphical control: exit 0.
- Full example project: fresh graphical editor startup and subsequent reopen both exit 0. These sessions ended before the runner sent its planned window-close request, so automatic window-close verification is not claimed.

Commands use `--editor --import`, `--audio-driver Dummy`, and `--rendering-method gl_compatibility`; graphical runs select `--display-driver x11`, while headless runs select `--headless`. The renderer reports Mesa llvmpipe. Release-profile libraries are placed at the isolated editor-selected debug manifest entry. XDG configuration/cache/data directories are isolated under `/tmp`.

All graphical runs, including the empty control, emit the same unsupported V-Sync graphics-driver warning. The original `clean_exit` flag treats any warning as false; `no_crash` separately records successful process exit. No addon errors appear in the graphical logs.

The user requested no further graphical runs. At the stop check, the runner and its Godot processes had already exited; no further GUI launches were made. These results support the upstream headless-import interpretation on the tested engine/display configuration. No addon code or engine source was changed.
