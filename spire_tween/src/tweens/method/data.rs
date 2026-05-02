use super::*;

/// Tween data for invoking a [`Callable`] each tick with an interpolated value.
///
/// On every update Spire computes a value by lerping between [`from`](Self::from) and
/// [`to`](Self::to) based on how much time has passed relative to
/// [`duration`](Self::duration), then invokes [`callable`](Self::callable) with that
/// value as a single argument.
///
/// You usually don't construct this directly — use [`DoMethod::do_method`] /
/// [`DoVarMethod::do_var_method`].
pub struct LerpMethodData<T: ILerpable> {
    /// The object backing the [`Callable`], if any. When this object is freed, the
    /// tween auto-stops on its next tick.
    pub owner: Option<ObjectOrNode>,
    /// The callable invoked each tick. Must accept exactly one argument of type `T`
    /// (or implicitly-convertible). Bind extra arguments via `Callable::bind` if needed.
    pub callable: Callable,
    /// Starting value passed to the callable at time `0.0`.
    pub from: T,
    /// Final value the callable will be invoked with — at the time
    /// [`duration`](Self::duration) elapses.
    pub to: T,
    /// Total duration of the tween, in seconds.
    pub duration: f64,
    /// Easing function applied to the linear progress before lerping. See
    /// [`set_ease`](SpireTween::<LerpMethodData<T>>::set_ease).
    pub ease: EaseKind,
    /// The lerping math itself — `()` for built-in types, [`CustomBasicLerper`] for
    /// user-supplied lerping (see [`SpireTween::<LerpMethodData<Variant>>::new_custom`]).
    pub lerper: T::BasicLerper,
}

impl<T: ILerpable> SpireTween<LerpMethodData<T>>
where LerpMethodData<T>: ITweenable
{
    /// The [`Callable`] invoked each tick with the interpolated value.
    #[inline]
    pub fn get_callable(&self) -> &Callable { &self.t.callable }

    /// The object the [`Callable`] is bound to (if any). When this object is freed,
    /// the tween auto-stops.
    #[inline]
    pub fn get_owner(&self) -> Option<&ObjectOrNode> { self.t.owner.as_ref() }

    /// Total duration of the tween, in seconds. The `duration` you passed at
    /// construction — not the remaining time.
    #[inline]
    pub fn get_duration(&self) -> f64 { self.t.duration }

    /// Returns the easing in effect for this tween. May be a built-in [`Ease`] variant,
    /// a Godot `Curve`, or a user-supplied [`Callable`] — see [`EaseKind`].
    #[inline]
    pub fn get_ease(&self) -> &EaseKind { &self.t.ease }
    /// Sets the easing function used to remap linear progress before lerping.
    ///
    /// Without easing the value passed to the callable changes at a constant rate.
    /// Easing reshapes the linear progress `p = elapsed / duration` into a possibly
    /// non-linear weight, which is then used to compute `from + (to - from) * weight`.
    ///
    /// Spire ships standard easing curves as variants of [`Ease`] (each has an
    /// ASCII-art shape diagram in its variant docs). For custom easing you can also
    /// supply a `Curve` via [`EaseKind::GodotCurve`] or a `Fn(f32) -> f32` via
    /// [`EaseKind::Callable`].
    #[inline]
    pub fn set_ease(&mut self, ease: EaseKind) { self.t.ease = ease; }

    /// The starting value passed to the callable at time `0.0`. This is the `from`
    /// argument supplied at construction.
    #[inline]
    pub fn get_start_value(&self) -> T
    where T: Clone {
        self.t.from.clone()
    }

    /// Overrides the starting value supplied at construction.
    #[inline]
    pub fn set_start_value(&mut self, from: T) { self.t.from = from; }

    /// The final value the callable will be invoked with — at the time
    /// [`get_duration`](Self::get_duration) elapses.
    #[inline]
    pub fn get_final_value(&self) -> T
    where T: Clone {
        self.t.to.clone()
    }

    /// Overrides the final value supplied at construction.
    #[inline]
    pub fn set_final_value(&mut self, to: T) { self.t.to = to; }
}

impl<T: ILerpable + ToGodot> SpireTweener for SpireTween<LerpMethodData<T>>
where LerpMethodData<T>: ITweenable
{
    #[inline]
    fn play(&mut self) {
        if self.is_stopped() {
            self.reset_counters();
        }

        self.state = State::Playing;
    }

    #[inline]
    fn pause(&mut self) {
        if !self.is_stopped() {
            self.state = State::Paused;
        }
    }

    #[inline]
    fn stop(&mut self) { self.state = State::Stopped; }

    #[inline]
    fn force_complete(&mut self) {
        match self.state {
            | State::Playing | State::Paused => {
                self.seek_end();
                self.handle_finished();
            }
            State::Stopped => {}
        }
    }

    fn process(&mut self, delta_time: f64, _is_tree_paused: bool) -> AdvanceTimeResult {
        if !self.t.callable.is_valid() {
            return AdvanceTimeResult::ShouldDespawn;
        }

        if let Some(owner) = &self.t.owner {
            match self.check_owner_validity_and_pause_mode(owner) {
                ObjectValidityResult::CanProcess => {}
                ObjectValidityResult::DontProcess => return AdvanceTimeResult::Paused,
                ObjectValidityResult::SomeObjectsDead => {
                    self.stop();
                    return AdvanceTimeResult::ShouldDespawn;
                }
            }
        }

        if self.handle_time_step(delta_time).is_none() {
            return AdvanceTimeResult::Playing;
        }

        let anim_pos =
            calc_animation_position(self.t.duration, self.loop_time, self.loop_counter, self.loop_mode, &self.t.ease);

        let target_value = self.t.lerper.spire_lerp(&self.t.from, &self.t.to, anim_pos);

        self.t.callable.call(&[target_value.to_variant()]);

        let excess_time = self.loop_time - self.t.duration;
        if excess_time < 0. {
            AdvanceTimeResult::Playing
        } else if let Some(excess_time) = self.handle_loop_finished(excess_time) {
            AdvanceTimeResult::Completed { excess_time }
        } else {
            AdvanceTimeResult::Playing
        }
    }
}

impl<T: ILerpable> SpireTween<LerpMethodData<T>>
where
    T: ToGodot,
    LerpMethodData<T>: ITweenable,
{
    fn seek_end(&mut self) {
        if !self.t.callable.is_valid() {
            return;
        }

        let target_value = {
            let eased_ratio = self.t.ease.sample(1.);
            self.t.lerper.spire_lerp(&self.t.from, &self.t.to, eased_ratio)
        };

        self.t.callable.call(&[target_value.to_variant()]);
    }
}

// ----------------------------------------------------------------
// Builder methods

impl<T: ILerpable> SpireTween<LerpMethodData<T>>
where LerpMethodData<T>: ITweenable
{
    /// Chainable replacement for the duration. Useful for adjusting a pre-built tween
    /// before registering it.
    pub fn with_duration(self, duration: f64) -> Self {
        Self {
            t: LerpMethodData { duration, ..self.t },
            ..self
        }
    }

    /// Chainable equivalent of [`set_ease`](Self::set_ease).
    pub fn with_ease(self, ease: EaseKind) -> Self {
        Self {
            t: LerpMethodData { ease, ..self.t },
            ..self
        }
    }

    /// Chainable equivalent of [`set_final_value`](Self::set_final_value).
    pub fn with_end(self, end: T) -> Self {
        Self {
            t: LerpMethodData { to: end, ..self.t },
            ..self
        }
    }

    /// Chainable equivalent of [`set_start_value`](Self::set_start_value).
    pub fn with_begin(self, start: T) -> Self {
        Self {
            t: LerpMethodData { from: start, ..self.t },
            ..self
        }
    }
}

impl<T: ILerpable<BasicLerper: Default>> SpireTween<LerpMethodData<T>>
where
    AnyTween: From<RcPtr<Self>>,
    LerpMethodData<T>: ITweenable,
{
    /// Constructs a method tween. `owner` is auto-derived from the callable's
    /// underlying object (if any) so the tween auto-stops when that object is freed.
    /// Default ease is [`EaseKind::default()`]; default lerper is `T::BasicLerper::default()`.
    ///
    /// You usually don't call this directly — use [`DoMethod::do_method`] or one of
    /// its variants.
    pub fn new(callable: Callable, from: T, to: T, duration: f64) -> Self {
        let owner = callable.object().map(ObjectOrNode::from_unchecked_object);

        Self::new_with_data(LerpMethodData {
            owner,
            callable,
            duration,
            ease: Default::default(),
            from,
            to,
            lerper: Default::default(),
        })
    }

    /// Convenience: [`new`](Self::new) followed by [`register`](SpireTween::register).
    /// Submits the tween to the global `TweenManager` and returns the [`RcPtr`] handle.
    pub fn new_registered(callable: Callable, from: T, to: T, duration: f64) -> RcPtr<Self>
    where AnyTween: From<RcPtr<Self>> {
        Self::new(callable, from, to, duration).register()
    }
}

impl SpireTween<LerpMethodData<Variant>> {
    /// Constructs a `Variant`-typed method tween that uses a user-supplied [`Callable`]
    /// as the lerping function. Use this when the tweened type isn't one Spire
    /// natively supports.
    ///
    /// The `lerper` callable must have signature `func(from: Variant, to: Variant, weight: float) -> Variant`,
    /// returning the interpolated value.
    pub fn new_custom(callable: Callable, from: Variant, to: Variant, duration: f64, lerper: Callable) -> Self {
        let owner = callable.object().map(ObjectOrNode::from_unchecked_object);

        Self::new_with_data(LerpMethodData {
            owner,
            callable,
            duration,
            ease: Default::default(),
            from,
            to,
            lerper: CustomBasicLerper::new(lerper),
        })
    }
}
