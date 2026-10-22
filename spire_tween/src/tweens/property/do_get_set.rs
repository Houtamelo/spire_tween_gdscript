use std::rc::Rc;

use super::*;

/// Builds an unregistered property tween from typed Rust accessors. The getter
/// supplies the starting value on first processing; relative mode reads it on
/// every update. Captured objects are not discoverable: use node
/// binding when closures access a manually freed Godot object.
///
/// This constructor and its component variants are intentionally Rust-only.
///
/// Accessor types must agree:
/// ```compile_fail
/// use godot::prelude::*;
/// use spire_tween::prelude::*;
/// do_get_set(|| Vector2::ZERO, |_: Color| {}, Vector2::ONE, 1.0);
/// ```
pub fn do_get_set<T>(
    getter: impl Fn() -> T + 'static,
    setter: impl FnMut(T) + 'static,
    to: T,
    duration: f64,
) -> SpireTween<LerpPropertyData<T>>
where
    T: PropertyType + FromGodot + ToGodot + Default + 'static,
    T::Data: From<PropertyDataViaCallable<T>>,
    T::Lerper: Default,
    LerpPropertyData<T>: ITweenable,
    AnyTween: From<TweenPtr<LerpPropertyData<T>>>,
{
    let data = typed_accessors(getter, setter);
    SpireTween::<LerpPropertyData<T>>::new(data.into(), Evaluator::Static(to), duration)
}

pub(crate) fn typed_accessors<T: FromGodot + ToGodot + 'static>(
    getter: impl Fn() -> T + 'static,
    mut setter: impl FnMut(T) + 'static,
) -> PropertyDataViaCallable<T> {
    PropertyDataViaCallable::new(
        Callable::from_fn("spire_typed_getter", move |_| getter().to_variant()),
        Callable::from_fn("spire_typed_setter", move |args| {
            setter(args[0].to::<T>());
            Variant::nil()
        }),
    )
}

/// Like [`do_get_set`], but changes only alpha. Each write rereads RGB so other
/// color-component animations are preserved. The target is not clamped.
pub fn do_get_set_alpha(
    getter: impl Fn() -> Color + 'static,
    mut setter: impl FnMut(Color) + 'static,
    to: f64,
    duration: f64,
) -> SpireTween<LerpPropertyData<f64>> {
    let getter = Rc::new(getter);
    let read = getter.clone();
    do_get_set(
        move || read().a as f64,
        move |alpha: f64| {
            let mut color = getter();
            color.a = alpha as f32;
            setter(color);
        },
        to,
        duration,
    )
}

macro_rules! axis_constructor {
    ($name:ident, $vector:ty, $axis:ty) => {
        /// Tweens one axis through typed Rust accessors, preserving other axes on
        /// each write. Captured object lifetimes require explicit ownership/binding.
        pub fn $name(
            getter: impl Fn() -> $vector + 'static,
            mut setter: impl FnMut($vector) + 'static,
            axis: $axis,
            to: f64,
            duration: f64,
        ) -> SpireTween<LerpPropertyData<f64>> {
            let getter = Rc::new(getter);
            let read = getter.clone();
            do_get_set(
                move || read()[axis] as f64,
                move |value: f64| {
                    let mut vector = getter();
                    vector[axis] = value as real;
                    setter(vector);
                },
                to,
                duration,
            )
        }
    };
}

axis_constructor!(do_get_set_vec2_axis, Vector2, godot::builtin::Vector2Axis);
axis_constructor!(do_get_set_vec3_axis, Vector3, godot::builtin::Vector3Axis);
