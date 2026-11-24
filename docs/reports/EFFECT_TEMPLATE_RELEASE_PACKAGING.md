# Release packaging includes the scene templates

The release builder stages the complete template directory from
`spire_tween_gdscript/addons/spire_tween/templates/`, including the 33 scenes,
shared GDScript, UID sidecars, catalog, and README. Only the authored template
file types are accepted. Their hashes join the platform's artifact inventory;
verification requires every source template file to have a matching staged file.

The desktop assembler includes these shared assets once, alongside the
platform-specific libraries. The extracted-archive smoke test loads every
catalogued scene, verifies its script inheritance, plays it on an appropriate
target, and checks restoration. The same smoke runs in debug and release exports.

Real-artifact integration tests compare the delivered template files with source
bytes and reject duplicate ZIP entries. They also remove a shared script from a
copy of the archive and regenerate its checksums: the installed runtime must
reject that incomplete addon before recording successful archive validation.
Another control removes a script and its artifact-hash entry from a stage;
source-inventory verification must still reject it.

The separate example gallery and third-party example art remain outside the
customer archive, as specified by the installation guide. Graphical source
template tests continue to cover rendered behavior and editor authoring; the
archive smoke covers the actual delivered files and exported playback.
