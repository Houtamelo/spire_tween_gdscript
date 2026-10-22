use super::*;

pub(crate) fn owned_progress(
    owner: Gd<Object>,
    duration: f64,
    mut update: impl FnMut(f64) + 'static,
) -> SpireTween<LerpMethodData<f64>> {
    let callable = Callable::from_fn("spire_template_progress", move |args| {
        update(args[0].to::<f64>());
        Variant::nil()
    });
    let mut tween = SpireTween::<LerpMethodData<f64>>::new(callable, 0.0, 1.0, duration);
    tween.t.owner = Some(ObjectOrNode::from_unchecked_object(owner));
    tween
}

// An OutQuad rise followed by its Yoyo reversal is 4*q*(1-q).
// DOTween also eases the displacement between the initial and final heights.
fn jump_height(progress: f64, jumps: i32, height: f64) -> f64 {
    let phase = (progress * f64::from(jumps.max(1))).rem_euclid(1.0);
    4.0 * height * phase * (1.0 - phase)
}

macro_rules! motion {
    ($trait:ident, $class:ty, $vector:ty, $up:expr) => {
        /// Position/scale convenience templates. Native constructors return
        /// unregistered builders. Jump captures its start on first processing,
        /// uses linear horizontal travel and OutQuad vertical travel plus repeated
        /// OutQuad/Yoyo hops. Configure easing on the returned progress tween.
        /// Jump supports Restart and Yoyo; Incremental progress is not supported.
        pub trait $trait {
            /// Uniform absolute scale; equivalent to expanding `to` to a vector.
            fn do_uniform_scale(&self, to: real, duration: f64) -> SpireTween<LerpPropertyData<$vector>>;
            /// Adds a local-position delta, blending with other relative tweens.
            fn do_blendable_position(&self, by: $vector, duration: f64) -> SpireTween<LerpPropertyData<$vector>>;
            /// Adds a global-position delta.
            fn do_blendable_global_position(&self, by: $vector, duration: f64)
            -> SpireTween<LerpPropertyData<$vector>>;
            /// Adds a scale delta, rather than multiplying the scale.
            fn do_blendable_scale(&self, by: $vector, duration: f64) -> SpireTween<LerpPropertyData<$vector>>;
            /// Jumps to a local position. Height uses the class's upward axis;
            /// zero/negative jump counts become one. Does not perform physics movement.
            fn do_jump(&self, to: $vector, height: f64, jumps: i32, duration: f64) -> SpireTween<LerpMethodData<f64>>;
            /// Global-position counterpart of `do_jump`.
            fn do_global_jump(
                &self,
                to: $vector,
                height: f64,
                jumps: i32,
                duration: f64,
            ) -> SpireTween<LerpMethodData<f64>>;
        }

        impl<C: Inherits<$class> + Inherits<Object>> $trait for Gd<C> {
            fn do_uniform_scale(&self, to: real, duration: f64) -> SpireTween<LerpPropertyData<$vector>> {
                self.do_scale(<$vector>::splat(to), duration)
            }
            fn do_blendable_position(&self, by: $vector, duration: f64) -> SpireTween<LerpPropertyData<$vector>> {
                self.do_position(by, duration).as_relative(<$vector>::ZERO)
            }
            fn do_blendable_global_position(
                &self,
                by: $vector,
                duration: f64,
            ) -> SpireTween<LerpPropertyData<$vector>> {
                self.do_global_position(by, duration).as_relative(<$vector>::ZERO)
            }
            fn do_blendable_scale(&self, by: $vector, duration: f64) -> SpireTween<LerpPropertyData<$vector>> {
                self.do_scale(by, duration).as_relative(<$vector>::ZERO)
            }
            fn do_jump(&self, to: $vector, height: f64, jumps: i32, duration: f64) -> SpireTween<LerpMethodData<f64>> {
                let mut node = self.clone().upcast::<$class>();
                let mut start: Option<$vector> = None;
                owned_progress(node.clone().upcast(), duration, move |p| {
                    let start = *start.get_or_insert_with(|| node.get_position());
                    let mut position = start.lerp(to, p as real);
                    position.y = start.y + (to.y - start.y) * (p * (2.0 - p)) as real;
                    position += $up * jump_height(p, jumps, height) as real;
                    node.set_position(position);
                })
            }
            fn do_global_jump(
                &self,
                to: $vector,
                height: f64,
                jumps: i32,
                duration: f64,
            ) -> SpireTween<LerpMethodData<f64>> {
                let mut node = self.clone().upcast::<$class>();
                let mut start: Option<$vector> = None;
                owned_progress(node.clone().upcast(), duration, move |p| {
                    let start = *start.get_or_insert_with(|| node.get_global_position());
                    let mut position = start.lerp(to, p as real);
                    position.y = start.y + (to.y - start.y) * (p * (2.0 - p)) as real;
                    position += $up * jump_height(p, jumps, height) as real;
                    node.set_global_position(position);
                })
            }
        }
    };
}

motion!(DoMotion2D, Node2D, Vector2, Vector2::UP);
motion!(DoMotion3D, Node3D, Vector3, Vector3::UP);
motion!(DoMotionControl, Control, Vector2, Vector2::UP);

/// Paired normalized Control anchors. Offset and opposite-anchor behavior is
/// identical to the existing scalar anchor constructors. Returns a sequence.
pub trait DoControlAnchors {
    fn do_anchor_min(&self, to: Vector2, duration: f64) -> SpireTween<Sequence>;
    fn do_anchor_max(&self, to: Vector2, duration: f64) -> SpireTween<Sequence>;
}
impl<C: Inherits<Control> + Inherits<Object>> DoControlAnchors for Gd<C> {
    fn do_anchor_min(&self, to: Vector2, duration: f64) -> SpireTween<Sequence> {
        let mut seq = SpireTween::<Sequence>::new();
        seq.append(self.do_anchor_left(to.x as f64, duration));
        seq.join(self.do_anchor_top(to.y as f64, duration));
        seq
    }
    fn do_anchor_max(&self, to: Vector2, duration: f64) -> SpireTween<Sequence> {
        let mut seq = SpireTween::<Sequence>::new();
        seq.append(self.do_anchor_right(to.x as f64, duration));
        seq.join(self.do_anchor_bottom(to.y as f64, duration));
        seq
    }
}

/// Control ellipses/circles using the existing polar-radius parametrization.
/// Angles are radians; positive rotation is clockwise. Local and global centers
/// are explicit. Container layout may subsequently reposition a Control.
pub trait DoControlEllipsis {
    fn do_ellipsis(
        &self,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        from_radius: Vector2,
        to_radius: Vector2,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>>;
    fn do_global_ellipsis(
        &self,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        from_radius: Vector2,
        to_radius: Vector2,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>>;
    fn do_circle(
        &self,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        radius: f32,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>>;
    fn do_global_circle(
        &self,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        radius: f32,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>>;
}
impl<C: Inherits<Control> + Inherits<Object>> DoControlEllipsis for Gd<C> {
    fn do_ellipsis(
        &self,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        from_radius: Vector2,
        to_radius: Vector2,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>> {
        control_ellipse(self.clone().upcast(), center, from_angle, to_angle, from_radius, to_radius, duration, false)
    }
    fn do_global_ellipsis(
        &self,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        from_radius: Vector2,
        to_radius: Vector2,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>> {
        control_ellipse(self.clone().upcast(), center, from_angle, to_angle, from_radius, to_radius, duration, true)
    }
    fn do_circle(
        &self,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        radius: f32,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>> {
        self.do_ellipsis(center, from_angle, to_angle, Vector2::splat(radius), Vector2::splat(radius), duration)
    }
    fn do_global_circle(
        &self,
        center: Vector2,
        from_angle: f32,
        to_angle: f32,
        radius: f32,
        duration: f64,
    ) -> SpireTween<LerpMethodData<f64>> {
        self.do_global_ellipsis(center, from_angle, to_angle, Vector2::splat(radius), Vector2::splat(radius), duration)
    }
}
fn control_ellipse(
    mut node: Gd<Control>,
    center: Vector2,
    from_angle: f32,
    to_angle: f32,
    from_radius: Vector2,
    to_radius: Vector2,
    duration: f64,
    global: bool,
) -> SpireTween<LerpMethodData<f64>> {
    owned_progress(node.clone().upcast(), duration, move |p| {
        let angle = from_angle.lerp(to_angle, p as f32);
        let radii = from_radius.lerp(to_radius, p as real);
        let (sin, cos) = angle.sin_cos();
        let denominator = ((radii.y * cos).powi(2) + (radii.x * sin).powi(2)).sqrt();
        let radius = if denominator == 0.0 { 0.0 } else { radii.x * radii.y / denominator };
        let position = center + Vector2::new(radius * cos, radius * sin);
        if global {
            node.set_global_position(position);
        } else {
            node.set_position(position);
        }
    })
}
