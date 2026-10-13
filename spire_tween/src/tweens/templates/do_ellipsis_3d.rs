use super::*;

/// 3D analogue of [`DoEllipsis2D`]: moves a `Node3D` along an ellipse traced in the
/// plane perpendicular to `axis`, around `center` (global coords).
///
/// Parameters:
/// - `axis`: the rotation axis the ellipse is drawn around. The ellipse lies in
///   the plane perpendicular to this axis. Auto-normalized; defaults to
///   `Vector3::UP` if zero.
/// - `from_radius` / `to_radius`: per-axis radii (`x`, `y` for the in-plane
///   ellipse; `z` for displacement *along* the axis with animation progress — i.e.
///   helical motion).
/// - other params: same as [`DoEllipsis2D`] (angles in radians, lerped radii).
///
/// The returned method tween animates progress from 0 to 1; angles and radii are
/// evaluated from that progress. Equal angles still interpolate radii. Set both
/// radii's `z` components to zero for a path in the plane through `center`.
/// The `DoNode3D.circle` GDScript shortcut does this automatically.
///
/// The receiver is bound to the tween and set as its method owner, so normal
/// playback cancels when the node is freed, including inside sequences.
///
/// Implemented for any `Gd<T: Inherits<Node3D>>`. Returns an unregistered
/// [`SpireTween<LerpMethodData<f64>>`] — call [`register`](SpireTween::register).
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
        let node = self.clone().upcast::<Node3D>();
        let normalized_axis = axis.try_normalized().unwrap_or(Vector3::UP);

        let callable = Callable::from_fn("do_ellipsis_3d", {
            let mut node = node.clone();

            move |args| {
                let progress = args
                    .first()
                    .and_then(|v| v.try_to_relaxed::<f32>().ok())
                    .unwrap_or_else(|| {
                        godot_error!("[do_ellipsis_3d] BUG: Expected first argument to be of type `f64`.");
                        0.0
                    });

                let angle = f32::lerp(from_angle, to_angle, progress);

                let rx = f32::lerp(from_radius.x, to_radius.x, progress);
                let ry = f32::lerp(from_radius.y, to_radius.y, progress);
                let rz = f32::lerp(from_radius.z, to_radius.z, progress);

                let cos = angle.cos();
                let sin = angle.sin();

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
            }
        });

        let mut tween = SpireTween::<LerpMethodData<f64>>::new(callable, 0.0, 1.0, duration);
        // Custom callables do not expose their captured node. Set the owner so
        // method processing also checks its lifetime when driven by a sequence.
        tween.t.owner = Some(ObjectOrNode::Node(node.clone().upcast()));
        tween.bound_to(node)
    }
}
