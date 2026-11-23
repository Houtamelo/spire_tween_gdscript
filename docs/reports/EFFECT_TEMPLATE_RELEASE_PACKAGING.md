# Release packaging omits non-binary addon assets

The 33 supplied GDScript template scenes live in
`spire_tween_gdscript/addons/spire_tween/templates/`, alongside the source
project's existing addon. Copy that directory with the addon when distributing
or installing these templates manually.

The existing Rust release assembler in `release/tools/src/main.rs` currently
collects only `addon/lib` and notices from platform staging, then inserts the
generated `.gdextension` and a fixed set of metadata files. It does not collect
the source addon's GDScript, `.tscn`, `.gd.uid`, or catalog files. Its installed
smoke test only exercises native classes, so it cannot detect that omission.

No Rust files were changed while implementing the templates, following the
requested GDScript-and-scenes scope. Before a commercial archive containing
these templates is released, the release tooling needs to:

1. Copy the template directory into the addon stage and record its file hashes.
2. Include that directory once in the assembled desktop archive, with its
   scripts, scenes, UID sidecars, catalog, and shipped README.
3. Test an installed template from the extracted archive in actual Godot,
   including a missing-script negative control.
4. Include a usable gallery/example if example scenes are part of the package.

The source-project template tests use a private addon copy and real Godot
rendering/editor sessions. They verify authoring and playback, and do not claim
that the current release archive already distributes the new assets.
