use super::*;

/// Marker trait: "this type can be tweened, and here are the lerpers Spire should
/// use for it."
///
/// Spire ships `impl ILerpable for …` for: `i64`, `f64`, `GString`, `Color`,
/// `Vector2`, `Vector2i`, `Vector3`, `Vector3i`, and `Variant`. For all built-in
/// types both lerpers default to `()`, which has the matching `BasicLerp` /
/// `SpireLerp` impl in [`crate::tweens::spire_lerp`].
///
/// To tween a *new* type, implement `ILerpable` for it with appropriate lerpers:
///
/// ```ignore
/// impl ILerpable for MyType {
///     type BasicLerper = MyLerper; // implements BasicLerp<MyType>
///     type Lerper = MyLerper;      // implements SpireLerp<MyType>
/// }
/// ```
///
/// `BasicLerper` is the minimum (used by [`LerpMethodData`]); `Lerper` adds the
/// extra ops needed for relative & speed-based property tweens (used by
/// [`LerpPropertyData`]). They can be the same type, or two different types.
///
/// If you don't want to write a lerper at all but only need a one-off custom
/// property/method tween, you can skip implementing `ILerpable` and reach for
/// [`SpireTween::<LerpPropertyData<Variant>>::new_custom`] /
/// [`SpireTween::<LerpMethodData<Variant>>::new_custom`] with a [`CustomLerper`] /
/// [`CustomBasicLerper`].
pub trait ILerpable: Sized {
    /// Lerper used by [`LerpMethodData`] (method tweens). The minimum requirement.
    type BasicLerper: BasicLerp<Self>;
    /// Lerper used by [`LerpPropertyData`] (property tweens). Adds the operations
    /// needed for [`LerpMode::Relative`] and [`LerpMode::SpeedBased`] modes.
    type Lerper: SpireLerp<Self>;
}

/*
impl ILerpable for i32 {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for f32 {
    type BasicLerper = ();
    type Lerper = ();
}
*/

impl ILerpable for i64 {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for f64 {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for GString {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for Color {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for Vector2 {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for Vector2i {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for Vector3 {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for Vector3i {
    type BasicLerper = ();
    type Lerper = ();
}

impl ILerpable for Variant {
    type BasicLerper = CustomBasicLerper;
    type Lerper = CustomLerper;
}
