//! Integer margin templates driven by a float progress clock.
use super::{effect::duration_or_default, motion::owned_progress, *};

fn progress(duration: f64, mut update: impl FnMut(f64) + 'static) -> SpireTween<LerpMethodData<f64>> {
    let callable = Callable::from_fn("spire_value_progress", move |args| {
        update(args[0].to::<f64>());
        Variant::nil()
    });
    SpireTween::<LerpMethodData<f64>>::new(callable, 0.0, 1.0, duration_or_default(duration))
}

fn margins_lerp(from: Vector4i, to: Vector4i, p: f64) -> Vector4i {
    fn component(a: i32, b: i32, p: f64) -> i32 { (f64::from(a) + (f64::from(b) - f64::from(a)) * p).round() as i32 }
    Vector4i::new(
        component(from.x, to.x, p),
        component(from.y, to.y, p),
        component(from.z, to.z, p),
        component(from.w, to.w, p),
    )
}

/// Integer margins in (left, right, top, bottom) order. Signed margins are allowed.
/// Each component rounds independently. Captures start on first processing.
/// Rust-only accessors; bind captured Node lifetimes. Returns a float progress tween.
pub fn do_get_set_margins(
    getter: impl Fn() -> Vector4i + 'static,
    mut setter: impl FnMut(Vector4i) + 'static,
    to: Vector4i,
    duration: f64,
) -> SpireTween<LerpMethodData<f64>> {
    let mut from = None;
    progress(duration, move |p| setter(margins_lerp(*from.get_or_insert_with(&getter), to, p)))
}

/// Reflected Vector4i property template, using (left, right, top, bottom) order.
/// Invalid property types warn and return a stopped builder that performs no writes.
pub fn do_property_margins(
    mut owner: Gd<Object>,
    path: NodePath,
    to: Vector4i,
    duration: f64,
) -> SpireTween<LerpMethodData<f64>> {
    let valid = owner.get_indexed(&path).try_to::<Vector4i>().is_ok();
    if !valid {
        godot_warn!(
            "[spire margins] property {path} must be Vector4i, got {}; returning a stopped tween.",
            owner.get_indexed(&path)
        );
    }
    let mut from = None;
    let mut tween = owned_progress(owner.clone(), duration_or_default(duration), move |p| {
        if !valid {
            return;
        }
        let from = *from.get_or_insert_with(|| owner.get_indexed(&path).to::<Vector4i>());
        owner.set_indexed(&path, &margins_lerp(from, to, p).to_variant());
    });
    if !valid {
        tween.t.callable = Callable::invalid();
        tween.stop();
    }
    tween
}

/// Godot MarginContainer theme margins. Writes local overrides, retaining other
/// theme properties. Captures effective theme values on first processing.
pub trait DoMargins {
    fn do_margins(&self, to: Vector4i, duration: f64) -> SpireTween<LerpMethodData<f64>>;
}
impl<C: Inherits<MarginContainer> + Inherits<Object>> DoMargins for Gd<C> {
    fn do_margins(&self, to: Vector4i, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        let node = self.clone().upcast::<MarginContainer>();
        let mut writer = node.clone();
        let mut tween = do_get_set_margins(
            move || {
                Vector4i::new(
                    node.get_theme_constant("margin_left"),
                    node.get_theme_constant("margin_right"),
                    node.get_theme_constant("margin_top"),
                    node.get_theme_constant("margin_bottom"),
                )
            },
            move |v| {
                for (name, value) in [
                    ("margin_left", v.x),
                    ("margin_right", v.y),
                    ("margin_top", v.z),
                    ("margin_bottom", v.w),
                ] {
                    writer.add_theme_constant_override(name, value);
                }
            },
            to,
            duration,
        );
        tween.t.owner = Some(ObjectOrNode::from_unchecked_object(self.clone().upcast()));
        tween
    }
}
