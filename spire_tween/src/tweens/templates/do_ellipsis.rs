use super::*;

/// Moves a `Node2D` along an elliptical path. Angles in radians, radii as `Vector2` (X/Y).
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
        let mut node = self.clone().upcast::<Node2D>();

        let callable = Callable::from_fn("do_ellipsis", move |args| {
            let theta = args
                .first()
                .and_then(|v| v.try_to_relaxed::<f32>().ok())
                .unwrap_or_else(|| {
                    godot_error!("[do_ellipsis] BUG: Expected first argument to be of type `f64`.");
                    0.0
                });

            let progress = (theta - from_angle) / (to_angle - from_angle);

            let rx = f32::lerp(from_radius.x, to_radius.x, progress);
            let ry = f32::lerp(from_radius.y, to_radius.y, progress);

            let cos = theta.cos();
            let sin = theta.sin();
            let rx_sin = rx * sin;
            let ry_cos = ry * cos;
            let denom = f32::sqrt(ry_cos * ry_cos + rx_sin * rx_sin);
            let r = if denom != 0.0 { (rx * ry) / denom } else { 0.0 };

            let offset = Vector2 { x: r * cos, y: r * sin };

            let point = center + offset;
            node.set_global_position(point);
            Variant::nil()
        });

        SpireTween::<LerpMethodData<f64>>::new(callable, from_angle as f64, to_angle as f64, duration)
    }
}
