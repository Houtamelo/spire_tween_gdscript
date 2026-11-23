# Author animations with supplied templates

SpireTween supplies 33 reusable GDScript `TweenBlueprint` scenes for text,
general visual effects, and interaction feedback. Instance a scene beneath its
target, configure the root, and connect a signal to `play()`.

See the [catalog and authoring guide](../spire_tween_gdscript/addons/spire_tween/templates/README.md)
for all templates, controls, timing, composition, and restoration behavior.
Open the [editable gallery](../spire_tween_gdscript/examples/effect_templates/gallery.tscn)
to try every template using Play/Restart and Stop & Restore.

The templates use the existing native tween runtime. Their implementation is
entirely GDScript and Godot scenes. Counters, progress bars, floating combat text,
and other gameplay-specific compositions remain user-authored combinations.
