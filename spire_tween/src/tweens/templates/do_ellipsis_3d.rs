use super::*;

/// Moves a `Node3D` along an ellipsoidal path around `axis`. Angles in radians.
pub trait DoEllipsis3D<Marker = ()> {
    fn do_ellipsis(
        &self,
        center: Vector3,
        from_angle: f32,
        to_angle: f32,
        from_radius: Vector3,
        to_radius: Vector3,
        axis: Vector3,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>>;
}

impl<T: Inherits<Node3D> + Inherits<Object>> DoEllipsis3D<()> for Gd<T> {
    fn do_ellipsis(
        &self,
        center: Vector3,
        from_angle: f32,
        to_angle: f32,
        from_radius: Vector3,
        to_radius: Vector3,
        axis: Vector3,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>> {
        let mut node = self.clone().upcast::<Node3D>();
        let normalized_axis = axis.try_normalized().unwrap_or(Vector3::UP);

        let callable = Callable::from_fn("do_ellipsis_3d", move |args| {
            let theta = args
                .first()
                .and_then(|v| v.try_to_relaxed::<f32>().ok())
                .unwrap_or_else(|| {
                    godot_error!("[do_ellipsis_3d] BUG: Expected first argument to be of type `f64`.");
                    0.0
                });

            let progress = (theta - from_angle) / (to_angle - from_angle);

            let rx = f32::lerp(from_radius.x, to_radius.x, progress);
            let ry = f32::lerp(from_radius.y, to_radius.y, progress);
            let rz = f32::lerp(from_radius.z, to_radius.z, progress);

            let cos = theta.cos();
            let sin = theta.sin();

            // Create a local coordinate system perpendicular to the axis
            let tangent = if normalized_axis.x.abs() > normalized_axis.z.abs() {
                Vector3::new(-normalized_axis.y, normalized_axis.x, 0.0)
            } else {
                Vector3::new(0.0, -normalized_axis.z, normalized_axis.y)
            }
            .normalized();

            let u = normalized_axis.cross(tangent).normalized();
            let v = normalized_axis.cross(u);

            let offset = u * (rx * cos) + v * (ry * sin) + normalized_axis * (rz * progress);

            let point = center + offset;
            node.set_global_position(point);
            Variant::nil()
        });

        SpireTween::<LerpMethodData<f64>>::new(callable, from_angle as f64, to_angle as f64, duration)
    }
}
