use super::*;

/// Moves a `Node2D` along an elliptical path around `center` (global coords).
///
/// Parameters:
/// - `center`: the ellipse's center, in global coordinates.
/// - `from_angle` / `to_angle`: start and end angles, **in radians**. Use the
///   difference to control direction and number of revolutions (e.g. `0.0 → 4*PI`
///   completes two full clockwise turns).
/// - `from_radius` / `to_radius`: the X/Y radii at the start and end of the
///   animation — Spire lerps the radii with animation progress, so a non-equal
///   `from`/`to` produces a spiraling-out (or in) ellipse.
/// - `duration`: total seconds.
///
/// Equal start/end angles keep the direction fixed while the radii interpolate.
/// The receiver is bound to the tween and set as its method owner, so normal
/// playback cancels when the node is freed, including inside sequences.
///
/// For a circle (uniform X/Y radius), use the `circle` shortcut on `DoNode2D` (the
/// GDScript-facing namespace), which forwards here with `from_radius == to_radius`.
///
/// Implemented for any `Gd<T: Inherits<Node2D>>`. Returns an unregistered
/// [`SpireTween<LerpMethodData<f64>>`] (the inner data lerps progress from 0 to 1) — call
/// [`register`](SpireTween::register).
pub trait DoEllipsis2D<Marker = ()> {
    fn do_ellipsis(
        &self,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        from_radius: Vector2,
        to_radius: Vector2,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>>;
}

impl<T: Inherits<Node2D> + Inherits<Object>> DoEllipsis2D<()> for Gd<T> {
    fn do_ellipsis(
        &self,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        from_radius: Vector2,
        to_radius: Vector2,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>> {
        let node = self.clone().upcast::<Node2D>();

        let callable = Callable::from_fn(std::any::type_name_of_val(&Self::do_ellipsis), {
            let mut node = node.clone();
            move |args| {
                let progress = args
                    .first()
                    .and_then(|v| v.try_to_relaxed::<f32>().ok())
                    .unwrap_or_else(|| {
                        godot_error!("[do_ellipsis] BUG: Expected first argument to be of type `f64`.");
                        0.0
                    });

                let angle = f32::lerp(from_angle, to_angle, progress);

                let rx = f32::lerp(from_radius.x, to_radius.x, progress);
                let ry = f32::lerp(from_radius.y, to_radius.y, progress);

                let cos = angle.cos();
                let sin = angle.sin();
                let rx_sin = rx * sin;
                let ry_cos = ry * cos;
                let denom = f32::sqrt(ry_cos * ry_cos + rx_sin * rx_sin);
                let r = if denom != 0.0 { (rx * ry) / denom } else { 0.0 };

                let offset = Vector2 { x: r * cos, y: r * sin };
                node.set_global_position(center + offset);

                Variant::nil()
            }
        });

        let mut tween = SpireTween::<LerpMethodData<f64>>::new(callable, 0.0, 1.0, duration);
        // Custom callables do not expose their captured node. Set the owner so
        // method processing also checks its lifetime when driven by a sequence.
        tween.t.owner = Some(ObjectOrNode::Node(node.clone().upcast()));
        tween.bound_to(node)
    }
}
