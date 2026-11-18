use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng};

use super::{
    effect::*,
    punch::{clean_vec2, clean_vec3},
    shake_path::ShakePath,
    *,
};

fn random_source() -> Xoshiro256PlusPlus {
    Xoshiro256PlusPlus::try_from_os_rng().unwrap_or_else(|_| {
        godot_warn!("[do_shake] OS randomness unavailable; using fixed seed.");
        Xoshiro256PlusPlus::seed_from_u64(1337)
    })
}
pub(super) fn angle(randomness: real) -> f64 {
    let value = finite(randomness as f64, 90.0, "randomness");
    if !(0.0..=180.0).contains(&value) {
        godot_warn!("[do_shake] randomness must be in [0,180], got {value}; clamping.");
    }
    value.clamp(0.0, 180.0)
}

fn shake_2d<V: EffectValue>(
    owner: Option<Gd<Object>>,
    getter: impl Fn() -> V + 'static,
    setter: impl FnMut(V) + 'static,
    map: impl Fn(Vector2) -> V + 'static,
    strength: Vector2,
    duration: f64,
    vibrato: i32,
    randomness: real,
    fade_out: bool,
    mode: ShakeRandomnessMode,
    blendable: bool,
) -> SpireTween<LerpMethodData<f64>> {
    let duration = duration_or_default(duration);
    let strength = clean_vec2(strength, true);
    let mut path = ShakePath::new(
        segment_count(vibrato, duration),
        [strength.x as f64, strength.y as f64],
        angle(randomness),
        fade_out,
        mode,
    );
    let mut rng = random_source();
    effect(
        owner,
        getter,
        setter,
        move |p, regenerate| {
            if regenerate {
                path.regenerate(&mut rng);
            }
            let v = path.sample(p);
            map(Vector2::new(v[0] as real, v[1] as real))
        },
        duration,
        blendable,
    )
}

fn shake_3d<V: EffectValue>(
    owner: Option<Gd<Object>>,
    getter: impl Fn() -> V + 'static,
    setter: impl FnMut(V) + 'static,
    map: impl FnMut(Vector3) -> V + 'static,
    strength: Vector3,
    duration: f64,
    vibrato: i32,
    randomness: real,
    fade_out: bool,
    mode: ShakeRandomnessMode,
    blendable: bool,
) -> SpireTween<LerpMethodData<f64>> {
    shake_3d_with_epoch(
        owner, getter, setter, map, strength, duration, vibrato, randomness, fade_out, mode, blendable, None,
    )
}

pub(super) fn shake_3d_with_epoch<V: EffectValue>(
    owner: Option<Gd<Object>>,
    getter: impl Fn() -> V + 'static,
    setter: impl FnMut(V) + 'static,
    mut map: impl FnMut(Vector3) -> V + 'static,
    strength: Vector3,
    duration: f64,
    vibrato: i32,
    randomness: real,
    fade_out: bool,
    mode: ShakeRandomnessMode,
    blendable: bool,
    epoch: Option<std::rc::Rc<dyn Fn() -> u64>>,
) -> SpireTween<LerpMethodData<f64>> {
    let duration = duration_or_default(duration);
    let strength = clean_vec3(strength, true);
    let mut path = ShakePath::new(
        segment_count(vibrato, duration),
        [strength.x as f64, strength.y as f64, strength.z as f64],
        angle(randomness),
        fade_out,
        mode,
    );
    let mut rng = random_source();
    effect_with_epoch(
        owner,
        getter,
        setter,
        move |p, regenerate| {
            if regenerate {
                path.regenerate(&mut rng);
            }
            let v = path.sample(p);
            map(Vector3::new(v[0] as real, v[1] as real, v[2] as real))
        },
        duration,
        blendable,
        epoch,
    )
}

/// Rust-only Vector3 shake through typed accessors. Bind manually freed nodes
/// captured by these closures. Uses the same input normalization, regeneration,
/// looping, easing and native completion restoration contract as [`DoShakeNode2D`].
/// Guard accessors when stopped callbacks can free captured objects before the
/// native finish restoration runs; those captured objects are not discoverable.
pub fn do_shake_vec3(
    getter: impl Fn() -> Vector3 + 'static,
    setter: impl FnMut(Vector3) + 'static,
    duration: f64,
    strength: Vector3,
    vibrato: i32,
    randomness: real,
    fade_out: bool,
    mode: ShakeRandomnessMode,
    blendable: bool,
) -> SpireTween<LerpMethodData<f64>> {
    shake_3d(None, getter, setter, |v| v, strength, duration, vibrato, randomness, fade_out, mode, blendable)
}

macro_rules! shake_2d_adapters {
    ($trait:ident, $class:ty) => {
        /// Scale and rotation counterparts of local-position shake. Rotation
        /// strength is in radians. Scale strength is an additive delta. GDScript
        /// defaults match position shake. Stop never restores; completion does.
        pub trait $trait {
            /// Shakes local scale with an additive per-axis strength.
            fn do_shake_scale(
                &self,
                duration: f64,
                strength: Vector2,
                vibrato: i32,
                randomness: real,
                fade_out: bool,
                mode: ShakeRandomnessMode,
                blendable: bool,
            ) -> SpireTween<LerpMethodData<f64>>;
            /// Shakes local rotation with strength in radians.
            fn do_shake_rotation(
                &self,
                duration: f64,
                strength: f64,
                vibrato: i32,
                randomness: real,
                fade_out: bool,
                mode: ShakeRandomnessMode,
                blendable: bool,
            ) -> SpireTween<LerpMethodData<f64>>;
        }
        impl<C: Inherits<$class> + Inherits<Object>> $trait for Gd<C> {
            fn do_shake_scale(
                &self,
                duration: f64,
                strength: Vector2,
                vibrato: i32,
                randomness: real,
                fade_out: bool,
                mode: ShakeRandomnessMode,
                blendable: bool,
            ) -> SpireTween<LerpMethodData<f64>> {
                let read = self.clone().upcast::<$class>();
                let mut write = read.clone();
                shake_2d(
                    Some(read.clone().upcast()),
                    move || read.get_scale(),
                    move |v| write.set_scale(v),
                    |v| v,
                    strength,
                    duration,
                    vibrato,
                    randomness,
                    fade_out,
                    mode,
                    blendable,
                )
            }
            fn do_shake_rotation(
                &self,
                duration: f64,
                strength: f64,
                vibrato: i32,
                randomness: real,
                fade_out: bool,
                mode: ShakeRandomnessMode,
                blendable: bool,
            ) -> SpireTween<LerpMethodData<f64>> {
                let read = self.clone().upcast::<$class>();
                let mut write = read.clone();
                let strength = finite(strength, 0.0, "rotation strength") as real;
                shake_2d(
                    Some(read.clone().upcast()),
                    move || read.get_rotation() as f64,
                    move |v| write.set_rotation(v as real),
                    |v| v.x as f64,
                    Vector2::new(strength, 0.0),
                    duration,
                    vibrato,
                    randomness,
                    fade_out,
                    mode,
                    blendable,
                )
            }
        }
    };
}
shake_2d_adapters!(DoShakeProperties2D, Node2D);
shake_2d_adapters!(DoShakePropertiesControl, Control);

/// Node3D shake, including Camera3D. All offsets are local. Rotation uses local
/// quaternion contributions with radians. Camera position shake uses its captured
/// camera-relative XY plane; Z strength is ignored. Other nodes shake in XYZ.
/// Lifecycle, normalization and defaults match [`DoShakeNode2D`].
pub trait DoShake3D {
    /// Shakes local position; Camera3D uses its captured relative XY plane.
    fn do_shake(
        &self,
        duration: f64,
        strength: Vector3,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>>;
    /// Shakes local scale using additive per-axis strength.
    fn do_shake_scale(
        &self,
        duration: f64,
        strength: Vector3,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>>;
    /// Shakes object-local orientation with per-axis strength in radians.
    fn do_shake_rotation(
        &self,
        duration: f64,
        strength: Vector3,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>>;
}
impl<C: Inherits<Node3D> + Inherits<Object>> DoShake3D for Gd<C> {
    fn do_shake(
        &self,
        duration: f64,
        strength: Vector3,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        let read = self.clone().upcast::<Node3D>();
        let mut write = read.clone();
        let owner = read.clone().upcast();
        if read.clone().try_cast::<Camera3D>().is_ok() {
            let camera = read.clone();
            let mut basis = None;
            // Capture orientation at first sampling, as with the position baseline.
            let duration = duration_or_default(duration);
            let strength = clean_vec2(Vector2::new(strength.x, strength.y), true);
            let mut path = ShakePath::new(
                segment_count(vibrato, duration),
                [strength.x as f64, strength.y as f64],
                angle(randomness),
                fade_out,
                mode,
            );
            let mut rng = random_source();
            return effect(
                Some(owner),
                move || read.get_position(),
                move |v| write.set_position(v),
                move |p, regenerate| {
                    if regenerate {
                        path.regenerate(&mut rng);
                    }
                    let basis = *basis.get_or_insert_with(|| camera.get_quaternion());
                    let v = path.sample(p);
                    basis * Vector3::new(v[0] as real, v[1] as real, 0.0)
                },
                duration,
                blendable,
            );
        }
        shake_3d(
            Some(owner),
            move || read.get_position(),
            move |v| write.set_position(v),
            |v| v,
            strength,
            duration,
            vibrato,
            randomness,
            fade_out,
            mode,
            blendable,
        )
    }
    fn do_shake_scale(
        &self,
        duration: f64,
        strength: Vector3,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        let read = self.clone().upcast::<Node3D>();
        let mut write = read.clone();
        shake_3d(
            Some(read.clone().upcast()),
            move || read.get_scale(),
            move |v| write.set_scale(v),
            |v| v,
            strength,
            duration,
            vibrato,
            randomness,
            fade_out,
            mode,
            blendable,
        )
    }
    fn do_shake_rotation(
        &self,
        duration: f64,
        strength: Vector3,
        vibrato: i32,
        randomness: real,
        fade_out: bool,
        mode: ShakeRandomnessMode,
        blendable: bool,
    ) -> SpireTween<LerpMethodData<f64>> {
        let read = self.clone().upcast::<Node3D>();
        let mut write = read.clone();
        shake_3d(
            Some(read.clone().upcast()),
            move || read.get_quaternion(),
            move |v| write.set_quaternion(v),
            Quaternion::from_euler,
            strength,
            duration,
            vibrato,
            randomness,
            fade_out,
            mode,
            blendable,
        )
    }
}
