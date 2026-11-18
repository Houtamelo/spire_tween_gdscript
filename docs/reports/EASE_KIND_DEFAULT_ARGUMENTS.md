# EaseKind and GDScript method defaults

Character range constructors require an explicit `delay_curve: EaseKind`.
GDScript accepts an Ease integer, a scalar Godot `Curve`, or a `Callable`.
Rust additionally accepts custom `EasingFunction` implementations.

The installed godot-rust 0.5.5 implementation of `#[opt(default = ...)]`
requires the entire parameter type to implement `GodotImmutable`, even when
the default expression itself is a basic Ease. `EaseKind` can contain shared
mutable Curve resources, Callables, or custom easing implementations and does
not satisfy that unsafe trait's contract.

This restriction appears in `godot-macros-0.5.5`'s
`make_default_argument_vec` and `godot-core-0.5.5`'s `opt_default_value`.
The [trait documentation](https://docs.rs/godot/0.5.5/godot/meta/trait.GodotImmutable.html)
describes its safety requirements.

Blueprint exports use separate `EaseSource`, `Ease`, and optional `Curve`
properties, so they use Godot's standard Inspector without a Variant union
editor. Character start-delay settings default to Enum with Linear. Runtime
callers select Linear explicitly; `EaseKind::default()` follows the global
default Ease.
