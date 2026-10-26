use godot::builtin::Vector3Axis;

use super::{effect::*, motion::owned_progress, punch::clean_vec3, *};

fn rotation(node: &Node3D, global: bool) -> Quaternion {
    if global { node.get_global_basis().orthonormalized().get_quaternion().normalized() } else { node.get_quaternion() }
}
fn set_rotation(node: &mut Node3D, value: Quaternion, global: bool) {
    if global {
        let scale = node.get_global_basis().get_scale();
        // Scale columns (R*S). Basis::scaled premultiplies (S*R), which would
        // shear a rotated nonuniform scale and change the requested orientation.
        node.set_global_basis(Basis::from_quaternion(value.normalized()) * Basis::from_scale(scale));
    } else {
        node.set_quaternion(value.normalized());
    }
}
fn normalized(value: Quaternion) -> Quaternion {
    if value.is_finite() && value.length_squared() > 0.00000001 {
        value.normalized()
    } else {
        godot_warn!("[spire rotation] quaternion must be finite and nonzero, got {value:?}; using identity.");
        Quaternion::IDENTITY
    }
}

fn quaternion_tween(
    mut node: Gd<Node3D>,
    to: Quaternion,
    duration: f64,
    global: bool,
) -> SpireTween<LerpMethodData<f64>> {
    let to = normalized(to);
    let mut start = None;
    owned_progress(node.clone().upcast(), duration_or_default(duration), move |p| {
        let start = *start.get_or_insert_with(|| rotation(&node, global));
        set_rotation(&mut node, start.slerp(to, p as real), global);
    })
}

fn euler_tween(
    mut node: Gd<Node3D>,
    to: Vector3,
    duration: f64,
    mode: RotateMode,
    global: bool,
) -> SpireTween<LerpMethodData<f64>> {
    let to = clean_vec3(to, false);
    let mut start = None;
    let mut initial_euler = None;
    let mut initial_world = None;
    owned_progress(node.clone().upcast(), duration_or_default(duration), move |p| {
        let q = *start.get_or_insert_with(|| rotation(&node, global));
        let from =
            *initial_euler.get_or_insert_with(|| if global { node.get_global_rotation() } else { node.get_rotation() });
        let world_start = *initial_world.get_or_insert_with(|| rotation(&node, true));
        match mode {
            RotateMode::WorldAxisAdd => {
                // Compose in world space even for a local destination constructor.
                set_rotation(&mut node, Quaternion::from_euler(to * p as real) * world_start, true);
            }
            RotateMode::LocalAxisAdd => set_rotation(&mut node, q * Quaternion::from_euler(to * p as real), global),
            RotateMode::Fast | RotateMode::FastBeyond360 => {
                let mut delta = to - from;
                if mode == RotateMode::Fast {
                    delta = shortest_delta(delta);
                }
                set_rotation(&mut node, Quaternion::from_euler(from + delta * p as real), global);
            }
        }
    })
}

fn blendable_tween(mut node: Gd<Node3D>, by: Vector3, duration: f64, global: bool) -> SpireTween<LerpMethodData<f64>> {
    let by = clean_vec3(by, false);
    let mut previous = Quaternion::IDENTITY;
    owned_progress(node.clone().upcast(), duration_or_default(duration), move |p| {
        let next = Quaternion::from_euler(by * p as real);
        let current = rotation(&node, global);
        let value = if global { next * previous.inverse() * current } else { current * previous.inverse() * next };
        previous = next;
        set_rotation(&mut node, value, global);
    })
}

fn look_rotation(direction: Vector3, up: Vector3, model_front: bool) -> Option<Quaternion> {
    if !direction.is_finite() || direction.length_squared() < 0.00000001 {
        return None;
    }
    let z = direction.normalized() * if model_front { 1.0 } else { -1.0 };
    let x = up.cross(z);
    if x.length_squared() < 0.00000001 {
        return None;
    }
    let x = x.normalized();
    Some(
        Basis::from_cols(x, z.cross(x).normalized(), z)
            .get_quaternion()
            .normalized(),
    )
}
fn shortest_delta(mut delta: Vector3) -> Vector3 {
    for axis in [Vector3Axis::X, Vector3Axis::Y, Vector3Axis::Z] {
        delta[axis] = delta[axis].rem_euclid(std::f64::consts::TAU as real);
        if delta[axis] > std::f64::consts::PI as real {
            delta[axis] -= std::f64::consts::TAU as real;
        }
    }
    delta
}
fn look_tween(
    mut node: Gd<Node3D>,
    mut target: impl FnMut() -> Vector3 + 'static,
    duration: f64,
    up: Vector3,
    axis: RotationAxis,
    model_front: bool,
    dynamic: bool,
) -> SpireTween<LerpMethodData<f64>> {
    let up = if up.is_finite() && up.length_squared() > 0.00000001 {
        up.normalized()
    } else {
        godot_warn!("[spire look-at] up must be finite and nonzero, got {up}; using Vector3::UP.");
        Vector3::UP
    };
    let mut start = None;
    let mut destination = None;
    let mut warned = false;
    owned_progress(node.clone().upcast(), duration_or_default(duration), move |p| {
        let start = *start.get_or_insert_with(|| rotation(&node, true));
        if destination.is_none() || dynamic {
            if let Some(q) = look_rotation(target() - node.get_global_position(), up, model_front) {
                destination = Some(q);
            } else if !warned {
                godot_warn!(
                    "[spire look-at] coincident target or parallel up vector; retaining the last valid orientation."
                );
                warned = true;
            }
        }
        if let Some(to) = destination {
            let from = start.get_euler();
            let mut delta = shortest_delta(to.get_euler() - from);
            let mask = if axis == RotationAxis::All { 7 } else { axis as i32 };
            for (i, axis) in [Vector3Axis::X, Vector3Axis::Y, Vector3Axis::Z].into_iter().enumerate() {
                if mask & (1 << i) == 0 {
                    delta[axis] = 0.0;
                }
            }
            set_rotation(&mut node, Quaternion::from_euler(from + delta * p as real), true);
        }
    })
}

/// 3D orientation templates driven by an ordinary float method tween. Capture
/// start on first processing; radians and YXZ Euler order follow Godot. Global
/// constructors use world coordinates and preserve scale. Invalid inputs warn
/// and normalize. Ordinary easing remaps progress. Native constructors are builders.
pub trait DoOrientation3D {
    /// Local Euler target or axis-add delta, interpreted according to `mode`.
    fn do_rotate(&self, to: Vector3, duration: f64, mode: RotateMode) -> SpireTween<LerpMethodData<f64>>;
    /// World-space counterpart of [`Self::do_rotate`].
    fn do_global_rotate(&self, to: Vector3, duration: f64, mode: RotateMode) -> SpireTween<LerpMethodData<f64>>;
    /// Normalized shortest-arc interpolation to a local quaternion target.
    fn do_rotate_quaternion(&self, to: Quaternion, duration: f64) -> SpireTween<LerpMethodData<f64>>;
    /// Normalized shortest-arc interpolation to a world quaternion target.
    fn do_global_rotate_quaternion(&self, to: Quaternion, duration: f64) -> SpireTween<LerpMethodData<f64>>;
    /// Applies successive quaternion deltas on the right, about object-local axes.
    fn do_blendable_rotation(&self, by: Vector3, duration: f64) -> SpireTween<LerpMethodData<f64>>;
    /// Applies successive quaternion deltas on the left, about world axes.
    fn do_blendable_global_rotation(&self, by: Vector3, duration: f64) -> SpireTween<LerpMethodData<f64>>;
    /// Captures the orientation towards a world-space point on first processing.
    /// Uses shortest Euler routes and preserves constrained components every step.
    fn do_look_at(
        &self,
        towards: Vector3,
        duration: f64,
        up: Vector3,
        axis: RotationAxis,
        use_model_front: bool,
    ) -> SpireTween<LerpMethodData<f64>>;
    /// Recomputes orientation towards the world-space point as this node moves.
    fn do_dynamic_look_at(
        &self,
        towards: Vector3,
        duration: f64,
        up: Vector3,
        axis: RotationAxis,
        use_model_front: bool,
    ) -> SpireTween<LerpMethodData<f64>>;
    /// Also follows a moving node and cancels when that node is freed. Binding
    /// the target additionally applies the tween's configured bound-node pause rules.
    fn do_look_at_target(
        &self,
        target: Gd<Node3D>,
        duration: f64,
        up: Vector3,
        axis: RotationAxis,
        use_model_front: bool,
    ) -> SpireTween<LerpMethodData<f64>>;
}
impl<C: Inherits<Node3D> + Inherits<Object>> DoOrientation3D for Gd<C> {
    fn do_rotate(&self, to: Vector3, duration: f64, mode: RotateMode) -> SpireTween<LerpMethodData<f64>> {
        euler_tween(self.clone().upcast(), to, duration, mode, false)
    }
    fn do_global_rotate(&self, to: Vector3, duration: f64, mode: RotateMode) -> SpireTween<LerpMethodData<f64>> {
        euler_tween(self.clone().upcast(), to, duration, mode, true)
    }
    fn do_rotate_quaternion(&self, to: Quaternion, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        quaternion_tween(self.clone().upcast(), to, duration, false)
    }
    fn do_global_rotate_quaternion(&self, to: Quaternion, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        quaternion_tween(self.clone().upcast(), to, duration, true)
    }
    fn do_blendable_rotation(&self, by: Vector3, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        blendable_tween(self.clone().upcast(), by, duration, false)
    }
    fn do_blendable_global_rotation(&self, by: Vector3, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        blendable_tween(self.clone().upcast(), by, duration, true)
    }
    fn do_look_at(
        &self,
        towards: Vector3,
        duration: f64,
        up: Vector3,
        axis: RotationAxis,
        front: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        look_tween(self.clone().upcast(), move || towards, duration, up, axis, front, false)
    }
    fn do_dynamic_look_at(
        &self,
        towards: Vector3,
        duration: f64,
        up: Vector3,
        axis: RotationAxis,
        front: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        look_tween(self.clone().upcast(), move || towards, duration, up, axis, front, true)
    }
    fn do_look_at_target(
        &self,
        target: Gd<Node3D>,
        duration: f64,
        up: Vector3,
        axis: RotationAxis,
        front: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        let binding = target.clone();
        look_tween(self.clone().upcast(), move || target.get_global_position(), duration, up, axis, front, true)
            .bound_to(binding)
    }
}
