use super::{effect::*, motion::owned_progress, *};

/// Local-position Archimedean spiral about an axis in parent coordinates.
/// Captures the initial position on first processing. `travel=duration*speed*p`,
/// `angle=frequency*travel`, radius is travel (or expands then contracts), and
/// axial travel is depth*travel. Angles are radians. Snapping rounds position.
/// Invalid axis defaults to +Z, duration/frequency to 1/10, nonfinite speed/depth
/// to 1/0, with warnings. This is independently specified from the public Pro
/// controls; its undocumented numerical implementation is not reproduced.
pub trait DoSpiral3D {
    /// Builds the local-position spiral with the explicit controls described above.
    fn do_spiral(
        &self,
        duration: f64,
        axis: Vector3,
        mode: SpiralMode,
        speed: f64,
        frequency: f64,
        depth: f64,
        snapping: bool,
    ) -> SpireTween<LerpMethodData<f64>>;
}
impl<C: Inherits<Node3D> + Inherits<Object>> DoSpiral3D for Gd<C> {
    fn do_spiral(
        &self,
        duration: f64,
        axis: Vector3,
        mode: SpiralMode,
        speed: f64,
        frequency: f64,
        depth: f64,
        snapping: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        let duration = duration_or_default(duration);
        let axis = if axis.is_finite() && axis.length_squared() > 0.00000001 {
            axis.normalized()
        } else {
            godot_warn!("[spire spiral] axis must be finite and nonzero, got {axis}; using +Z.");
            Vector3::BACK
        };
        let speed = finite(speed, 1.0, "spiral speed");
        let frequency = if frequency.is_finite() && frequency > 0.0 {
            frequency
        } else {
            godot_warn!("[spire spiral] frequency must be finite and positive, got {frequency}; using 10.");
            10.0
        };
        let depth = finite(depth, 0.0, "spiral depth");
        let reference = if axis.dot(Vector3::UP).abs() < 0.99 { Vector3::UP } else { Vector3::RIGHT };
        let x = reference.cross(axis).normalized();
        let y = axis.cross(x);
        let mut node = self.clone().upcast::<Node3D>();
        let mut start = None;
        owned_progress(node.clone().upcast(), duration, move |p| {
            let start = *start.get_or_insert_with(|| node.get_position());
            let travel = duration * speed * p;
            let radius = match mode {
                SpiralMode::Expand => travel,
                SpiralMode::ExpandThenContract => duration * speed * (1.0 - (2.0 * p - 1.0).abs()),
            };
            let angle = frequency * travel;
            let position = start
                + (x * angle.cos() as real + y * angle.sin() as real) * radius as real
                + axis * (depth * travel) as real;
            node.set_position(if snapping { position.round() } else { position });
        })
    }
}
