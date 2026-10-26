use super::{effect::*, *};

// Analytic knots avoid allocating a second waypoint/timing buffer for punch.
fn punch_sample(progress: f64, n: usize, elasticity: f64) -> f64 {
    if !(progress > 0.0 && progress < 1.0) {
        return 0.0;
    }
    let knot_time = |k: usize| k as f64 * (k + 1) as f64 / (n as f64 * (n + 1) as f64);
    let knot = |k: usize| {
        if k == 0 || k == n {
            0.0
        } else {
            (1.0 - (k - 1) as f64 / n as f64) * if k % 2 == 1 { 1.0 } else { -elasticity }
        }
    };
    let mut lo = 1;
    let mut hi = n;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if knot_time(mid) < progress {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let weight = (progress - knot_time(lo - 1)) / (knot_time(lo) - knot_time(lo - 1));
    knot(lo - 1) + (knot(lo) - knot(lo - 1)) * weight
}

pub(crate) fn punch_value<V: EffectValue>(
    owner: Option<Gd<Object>>,
    getter: impl Fn() -> V + 'static,
    setter: impl FnMut(V) + 'static,
    scale: impl Fn(f64) -> V + 'static,
    duration: f64,
    vibrato: i32,
    elasticity: f64,
    blendable: bool,
) -> SpireTween<LerpMethodData<f64>> {
    let duration = duration_or_default(duration);
    let n = segment_count(vibrato, duration).get();
    let elasticity = unit(elasticity, 1.0, "elasticity");
    effect(owner, getter, setter, move |p, _| scale(punch_sample(p, n, elasticity)), duration, blendable)
}

/// Generic Rust-only Vector3 punch. Accessors capture the start on first step.
/// Bind the builder to every manually freed node used by the accessors. Like
/// shake, stop leaves the last offset; successful completion removes it. Loops
/// repeat the path, Yoyo retraces it, and native finish hooks perform restoration.
/// If stopped callbacks can free captured objects before restoration, the
/// accessors themselves must guard that access with `is_instance_valid()`.
pub fn do_punch_vec3(
    getter: impl Fn() -> Vector3 + 'static,
    setter: impl FnMut(Vector3) + 'static,
    punch: Vector3,
    duration: f64,
    vibrato: i32,
    elasticity: f64,
    blendable: bool,
) -> SpireTween<LerpMethodData<f64>> {
    let punch = clean_vec3(punch, false);
    punch_value(None, getter, setter, move |p| punch * p as real, duration, vibrato, elasticity, blendable)
}

pub(crate) fn clean_vec3(value: Vector3, nonnegative: bool) -> Vector3 {
    Vector3::new(
        clean_component(value.x, nonnegative),
        clean_component(value.y, nonnegative),
        clean_component(value.z, nonnegative),
    )
}
pub(crate) fn clean_vec2(value: Vector2, nonnegative: bool) -> Vector2 {
    Vector2::new(clean_component(value.x, nonnegative), clean_component(value.y, nonnegative))
}
fn clean_component(value: real, nonnegative: bool) -> real {
    if !value.is_finite() || (nonnegative && value < 0.0) {
        godot_warn!("[spire effect] invalid vector component {value}; using zero.");
        0.0
    } else {
        value
    }
}

macro_rules! punch_2d {
    ($trait:ident, $class:ty) => {
        /// Local position, scale and rotation punch. Angles are radians. Invalid
        /// duration becomes one second; invalid components become zero; elasticity
        /// is clamped to `[0,1]`. Blendable defaults true in GDScript. Returns builders.
        pub trait $trait {
            /// Punches local position by the supplied vector.
            fn do_punch_position(
                &self,
                punch: Vector2,
                duration: f64,
                vibrato: i32,
                elasticity: f64,
                blendable: bool,
            ) -> SpireTween<LerpMethodData<f64>>;
            /// Punches local scale by an additive vector delta.
            fn do_punch_scale(
                &self,
                punch: Vector2,
                duration: f64,
                vibrato: i32,
                elasticity: f64,
                blendable: bool,
            ) -> SpireTween<LerpMethodData<f64>>;
            /// Punches local rotation by the supplied angle in radians.
            fn do_punch_rotation(
                &self,
                punch: f64,
                duration: f64,
                vibrato: i32,
                elasticity: f64,
                blendable: bool,
            ) -> SpireTween<LerpMethodData<f64>>;
        }
        impl<C: Inherits<$class> + Inherits<Object>> $trait for Gd<C> {
            fn do_punch_position(
                &self,
                punch: Vector2,
                duration: f64,
                vibrato: i32,
                elasticity: f64,
                blendable: bool,
            ) -> SpireTween<LerpMethodData<f64>> {
                let read = self.clone().upcast::<$class>();
                let mut write = read.clone();
                let punch = clean_vec2(punch, false);
                punch_value(
                    Some(read.clone().upcast()),
                    move || read.get_position(),
                    move |v| write.set_position(v),
                    move |p| punch * p as real,
                    duration,
                    vibrato,
                    elasticity,
                    blendable,
                )
            }
            fn do_punch_scale(
                &self,
                punch: Vector2,
                duration: f64,
                vibrato: i32,
                elasticity: f64,
                blendable: bool,
            ) -> SpireTween<LerpMethodData<f64>> {
                let read = self.clone().upcast::<$class>();
                let mut write = read.clone();
                let punch = clean_vec2(punch, false);
                punch_value(
                    Some(read.clone().upcast()),
                    move || read.get_scale(),
                    move |v| write.set_scale(v),
                    move |p| punch * p as real,
                    duration,
                    vibrato,
                    elasticity,
                    blendable,
                )
            }
            fn do_punch_rotation(
                &self,
                punch: f64,
                duration: f64,
                vibrato: i32,
                elasticity: f64,
                blendable: bool,
            ) -> SpireTween<LerpMethodData<f64>> {
                let read = self.clone().upcast::<$class>();
                let mut write = read.clone();
                let punch = finite(punch, 0.0, "punch rotation");
                punch_value(
                    Some(read.clone().upcast()),
                    move || read.get_rotation() as f64,
                    move |v| write.set_rotation(v as real),
                    move |p| punch * p,
                    duration,
                    vibrato,
                    elasticity,
                    blendable,
                )
            }
        }
    };
}
punch_2d!(DoPunch2D, Node2D);
punch_2d!(DoPunchControl, Control);

/// Node3D punch templates. Rotation contributions compose as local quaternions;
/// angles are radians and scale deltas are additive. See [`DoPunch2D`] for lifecycle.
pub trait DoPunch3D {
    /// Punches local position by the supplied vector.
    fn do_punch_position(
        &self,
        punch: Vector3,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>>;
    /// Punches local scale by an additive vector delta.
    fn do_punch_scale(
        &self,
        punch: Vector3,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>>;
    /// Punches object-local orientation by a YXZ Euler vector in radians.
    fn do_punch_rotation(
        &self,
        punch: Vector3,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>>;
    /// Additive quaternion-composition counterpart, with blendable always enabled.
    fn do_blendable_punch_rotation(
        &self,
        punch: Vector3,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
    ) -> SpireTween<LerpMethodData<f64>>;
}
impl<C: Inherits<Node3D> + Inherits<Object>> DoPunch3D for Gd<C> {
    fn do_punch_position(
        &self,
        punch: Vector3,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        let read = self.clone().upcast::<Node3D>();
        let mut write = read.clone();
        let punch = clean_vec3(punch, false);
        punch_value(
            Some(read.clone().upcast()),
            move || read.get_position(),
            move |v| write.set_position(v),
            move |p| punch * p as real,
            duration,
            vibrato,
            elasticity,
            blendable,
        )
    }
    fn do_punch_scale(
        &self,
        punch: Vector3,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        let read = self.clone().upcast::<Node3D>();
        let mut write = read.clone();
        let punch = clean_vec3(punch, false);
        punch_value(
            Some(read.clone().upcast()),
            move || read.get_scale(),
            move |v| write.set_scale(v),
            move |p| punch * p as real,
            duration,
            vibrato,
            elasticity,
            blendable,
        )
    }
    fn do_punch_rotation(
        &self,
        punch: Vector3,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        let read = self.clone().upcast::<Node3D>();
        let mut write = read.clone();
        let punch = clean_vec3(punch, false);
        punch_value(
            Some(read.clone().upcast()),
            move || read.get_quaternion(),
            move |v| write.set_quaternion(v),
            move |p| Quaternion::from_euler(punch * p as real),
            duration,
            vibrato,
            elasticity,
            blendable,
        )
    }
    fn do_blendable_punch_rotation(
        &self,
        punch: Vector3,
        duration: f64,
        vibrato: i32,
        elasticity: f64,
    ) -> SpireTween<LerpMethodData<f64>> {
        self.do_punch_rotation(punch, duration, vibrato, elasticity, true)
    }
}
