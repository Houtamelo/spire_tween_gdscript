use super::*;

/// Tween data for animating a property by lerping between specified `from` and `to`
/// values. The lerp weight is based on how much time has passed relative to
/// [`get_duration`](SpireTween::<LerpPropertyData<T>>::get_duration).
///
/// You usually don't construct this directly — instead use a `Do*` extension trait
/// (e.g. [`DoProperty::do_property`], or one of the per-property shortcuts on
/// extension traits like `DoNode2D`, `DoControl`, `DoCanvasItem`, etc.).
pub struct LerpPropertyData<T: PropertyType> {
    /// Adapter that knows how to read/write the underlying property — provides
    /// `get_property_value` / `set_property_value` plus the path & owner accessors.
    /// For built-in classes this is a generated enum; for custom properties it can be
    /// [`PropertyDataViaCallable`] or `PropertyDataCustom`.
    pub data: T::Data,
    /// Selects the interpolation strategy: absolute (default), speed-based, or
    /// relative. See [`LerpMode`] and the `as_*` builders for transitions.
    pub lerp_mode: LerpMode<T>,
    /// Easing function applied to the linear progress before lerping. See
    /// [`set_ease`](SpireTween::<LerpPropertyData<T>>::set_ease) for the full
    /// explainer.
    pub ease: EaseKind,
    /// How the target value is obtained each tick: a static `T`, a closure
    /// re-evaluated per tick, or a Godot `Callable`. See [`Evaluator`].
    pub to: Evaluator<T>,
    /// The lerping math itself — `()` for built-in types like `f64`/`Vector2`/`Color`,
    /// a [`CustomLerper`] for user-defined types.
    pub lerper: T::Lerper,
}

impl<T> SpireTween<LerpPropertyData<T>>
where
    T: PropertyType + Clone + Default + FromGodot,
    LerpPropertyData<T>: ITweenable,
{
    /// Returns the property path this tween animates, relative to
    /// [`get_owner`](Self::get_owner).
    #[inline]
    pub fn get_property_path(&self) -> NodePath { self.t.data.get_property_path() }

    /// Returns the object that owns the property being animated.
    #[inline]
    pub fn get_owner(&self) -> Option<&ObjectOrNode> { self.t.data.get_owner() }

    /// Returns the easing in effect for this tween. May be a built-in [`Ease`] variant,
    /// a Godot `Curve`, or a user-supplied [`Callable`] — see [`EaseKind`].
    #[inline]
    pub fn get_ease(&self) -> &EaseKind { &self.t.ease }
    /// Sets the easing function used to remap the linear progress before lerping.
    ///
    /// # What is easing?
    ///
    /// A tween is a smooth transition from a start value to an end value over a duration.
    /// On every update Spire computes a linear progress `p = elapsed / duration`, then
    /// computes the current value as `from + (to - from) * ease(p)`.
    ///
    /// Without easing (`p` passed straight through), the value changes at a constant
    /// rate. An easing function reshapes that — taking `p` as input and returning a
    /// possibly-non-linear weight.
    ///
    /// Spire ships the most common easing curves as variants of [`Ease`] (with
    /// ASCII-art shape diagrams in the docs of each variant). You can also set:
    /// - a Godot `Curve` via [`EaseKind::GodotCurve`] — sampled with `Curve::sample_baked`,
    /// - a custom function via [`EaseKind::Callable`] — must have signature `Fn(f32) -> f32`.
    ///
    /// # Easing in speed-based mode
    ///
    /// Speed-based tweens have no fixed duration, so easing is applied to the *speed*
    /// based on the distance ratio (`distance_traveled / total_distance`) instead of
    /// the time ratio. Easier to grok by trying it than by reading about it.
    #[inline]
    pub fn set_ease(&mut self, ease: EaseKind) { self.t.ease = ease; }

    /// Returns the tween's final value — the value the property should hold once the
    /// animation completes.
    ///
    /// **Note:** In [`LerpMode::Relative`] mode this is the relative offset, not the
    /// absolute final value.
    #[inline]
    pub fn get_final_value(&mut self) -> T
    where T: Clone + FromGodot + Default {
        self.t.to.eval()
    }

    /// Sets a fixed final value, overwriting any previous fixed value or dynamic
    /// target.
    #[inline]
    pub fn set_final_value(&mut self, to: T) { self.t.to = Evaluator::Static(to); }

    /// Replaces the fixed final value with a [`Callable`] re-evaluated on each tick —
    /// allowing "moving" goals.
    ///
    /// The callable must take no arguments and return a value of the same type as the
    /// property.
    ///
    /// # Example
    ///
    /// Make `pursuer` follow `target`'s position at 50 px/s. The `to` argument
    /// supplied at construction is ignored once a dynamic target is set.
    ///
    /// ```ignore
    /// pursuer
    ///     .do_position(Vector2::ZERO, 50.0)
    ///     .as_speed_based()
    ///     .set_dynamic_target(target.callable("get_position"))
    ///     .register();
    /// ```
    ///
    /// **Note:** Dynamic targets make the most sense in [`LerpMode::SpeedBased`] mode.
    /// They also work in [`LerpMode::Relative`] mode. They will likely behave oddly in
    /// the default [`LerpMode::Absolute`] mode.
    #[inline]
    pub fn set_dynamic_target(&mut self, evaluator: Callable) { self.t.to = Evaluator::Callable(evaluator); }

    /// Sets the tween's starting value, overriding the default behavior of reading the
    /// property's current value on the first tick.
    ///
    /// **Note:** Ignored in [`LerpMode::Relative`] and [`LerpMode::SpeedBased`] modes;
    /// a runtime warning is emitted if called on such a tween.
    pub fn set_begin_value(&mut self, value: T) {
        match &mut self.t.lerp_mode {
            LerpMode::Absolute { from, .. } => {
                *from = FromValue::Explicit(value);
            }
            LerpMode::SpeedBased { .. } => {
                godot_warn!(
                    "[b]Warning:[/b] Starting value(set by calling `from(value)`) is ignored in Speed-Based tweens \
                     (created with `as_speed_based`)."
                );
            }
            LerpMode::Relative { .. } => {
                godot_warn!(
                    "[b]Warning:[/b] Starting value is ignored in Relative tweens (created with `as_relative`)."
                );
            }
        }
    }
}

impl<T> SpireTween<LerpPropertyData<T>>
where
    T: PropertyType + Clone + Default + FromGodot,
    LerpPropertyData<T>: ITweenable,
{
    /// Switches this tween into [`LerpMode::Relative`].
    ///
    /// A relative tween *adds* its interpolated offset to the property each tick
    /// instead of forcing the property onto a fixed `from → to` curve. The tween
    /// "blends" with any other forces affecting the property.
    ///
    /// # Example
    ///
    /// A 5-second relative tween with target offset `(300, 20)`:
    ///
    /// ```ignore
    /// my_node.do_position(Vector2::new(300.0, 20.0), 5.0).as_relative(Vector2::ZERO);
    /// ```
    ///
    /// The tween is contractually obligated to *add* `(300, 20)` to `my_node.position`
    /// over 5 seconds, regardless of starting position or external forces.
    ///
    /// Starting at `(100, 50)` with no other influence, the position over time:
    /// - `0.0s` → `(100, 50)`
    /// - `2.5s` → `(250, 60)`
    /// - `5.0s` → `(400, 70)`
    ///
    /// Same starting position, but with an external velocity of `(10, 0)`:
    /// - `0.0s` → `(100, 50)`
    /// - `2.5s` → `(275, 60)`
    /// - `5.0s` → `(450, 70)`
    ///
    /// The tween blended with the velocity rather than overriding it.
    ///
    /// **Note:** Two or more relative tweens on the same property add their offsets
    /// together. A single non-relative tween on the same property will override all
    /// relative tweens.
    #[inline]
    pub fn set_relative(&mut self, relative_to_value: T) {
        match &mut self.t.lerp_mode {
            | &mut LerpMode::Absolute { duration, .. } | &mut LerpMode::SpeedBased { speed: duration, .. } => {
                self.t.lerp_mode = LerpMode::relative(duration, relative_to_value);
            }
            LerpMode::Relative { relative_to, .. } => {
                *relative_to = relative_to_value;
            }
        }
    }

    /// Switches this tween back into [`LerpMode::Absolute`] (the default mode).
    /// In absolute mode, on each tick the property is set to `from + (to - from) * ease(progress)`.
    #[inline]
    pub fn set_absolute(&mut self) {
        match &mut self.t.lerp_mode {
            | &mut LerpMode::Relative { duration, .. } | &mut LerpMode::SpeedBased { speed: duration, .. } => {
                self.t.lerp_mode = LerpMode::absolute(duration);
            }
            LerpMode::Absolute { .. } => {}
        }
    }

    /// Switches this tween into [`LerpMode::SpeedBased`], reinterpreting the original
    /// `duration` argument as `speed` (units per second).
    ///
    /// A regular (absolute) tween has fixed `from`, `to`, and `duration`, computing the
    /// current value as `from + (to - from) * progress`. A speed-based tween instead
    /// "bumps" the current value toward [`get_final_value`](Self::get_final_value) by
    /// `speed * delta_time` each tick, so it composes naturally with other forces
    /// (similar to relative tweens).
    ///
    /// # Example
    ///
    /// Translate a node toward `(500, 300)` at 100 px/s:
    ///
    /// ```ignore
    /// my_node.do_position(Vector2::new(500.0, 300.0), 100.0).as_speed_based();
    /// //                                              ^^^^^ now interpreted as speed
    /// ```
    ///
    /// **Note:** Speed-based tweens never overshoot.
    ///
    /// **Note:** The unit of `speed` depends on the tweened type:
    /// - `i64` / `f64`: units per second.
    /// - `Vector2` / `Vector2i` / `Vector3` / `Vector3i`: Euclidean distance per second.
    /// - `Color`: 4D Euclidean distance per second across `(r, g, b, a)`.
    /// - `GString`: characters per second (different number of differing characters).
    #[inline]
    pub fn set_speed_based(&mut self) {
        match &mut self.t.lerp_mode {
            LerpMode::Absolute { duration, .. } | LerpMode::Relative { duration, .. } => {
                self.t.lerp_mode = LerpMode::speed_based(*duration);
            }
            LerpMode::SpeedBased { .. } => {}
        }
    }
}

impl<T> SpireTween<LerpPropertyData<T>>
where
    T: PropertyType,
    LerpPropertyData<T>: ITweenable,
{
    /// Returns `true` if this tween is in [`LerpMode::Absolute`] (the default mode).
    #[inline]
    pub fn is_absolute(&self) -> bool { matches!(self.t.lerp_mode, LerpMode::Absolute { .. }) }

    /// Returns `true` if this tween is in [`LerpMode::Relative`] mode.
    /// See [`set_relative`](Self::set_relative).
    #[inline]
    pub fn is_relative(&self) -> bool { matches!(self.t.lerp_mode, LerpMode::Relative { .. }) }

    /// Returns `true` if this tween is in [`LerpMode::SpeedBased`] mode.
    /// See [`set_speed_based`](Self::set_speed_based).
    #[inline]
    pub fn is_speed_based(&self) -> bool { matches!(self.t.lerp_mode, LerpMode::SpeedBased { .. }) }

    /// Total duration of the tween in seconds. This is the `duration` you passed at
    /// construction — not the remaining time.
    ///
    /// **Note:** For speed-based tweens, this is computed as `distance / speed` from
    /// the current property value to the target.
    #[inline]
    pub fn get_duration(&mut self) -> f64
    where T: Clone + Default + FromGodot {
        match self.t.lerp_mode {
            LerpMode::Absolute { duration, .. } => duration,
            LerpMode::SpeedBased { speed, .. } => {
                let distance = self
                    .t
                    .lerper
                    .spire_distance(&self.t.data.get_property_value(), &self.t.to.eval());

                distance / speed
            }
            LerpMode::Relative { duration, .. } => duration,
        }
    }
}

impl<T: PropertyType> SpireTween<LerpPropertyData<T>>
where LerpPropertyData<T>: ITweenable
{
    fn try_seek_end(&mut self) -> anyhow::Result<()>
    where T: Clone + Default + FromGodot {
        if self.t.data.get_owner().is_some_and(|obj| !obj.is_instance_valid()) {
            let prop_name = &self.t.data.get_property_path();
            bail!("Cannot set property `{prop_name}` on Object, owner is no longer a valid instance.");
        }

        let target_value = match &mut self.t.lerp_mode {
            LerpMode::Absolute { .. } => self.t.to.eval(),
            LerpMode::SpeedBased { step_sum, .. } => {
                *step_sum = 0.;
                self.t.to.eval()
            }
            LerpMode::Relative {
                duration: _,
                relative_to,
                previous_anim_pos,
            } => {
                let val_at_obj = self.t.data.get_property_value();
                let end = self.t.to.eval();
                let previous_relative = self.t.lerper.spire_lerp(relative_to, &end, *previous_anim_pos);
                self.t.lerper.add_relative(&val_at_obj, &previous_relative, &end)
            }
        };

        self.t.data.set_property_value(target_value);
        Ok(())
    }
}

impl<T> SpireTweener for SpireTween<LerpPropertyData<T>>
where
    T: PropertyType + FromGodot + ToGodot + Clone + Default + Debug,
    LerpPropertyData<T>: ITweenable,
{
    #[inline]
    fn play(&mut self) {
        if self.is_stopped() {
            self.reset_counters();

            if let LoopMode::Restart = self.loop_mode {
                self.t.lerp_mode.reset_state();
            }
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
    fn stop(&mut self) {
        self.t.lerp_mode.reset_state();
        self.state = State::Stopped;
    }

    fn force_complete(&mut self) {
        match self.state {
            | State::Playing | State::Paused => {
                match self.try_seek_end() {
                    Ok(_) => {}
                    Err(err) => {
                        godot_error!("{err}");
                    }
                }

                self.handle_finished();
            }
            State::Stopped => {}
        }
    }

    fn process(&mut self, delta_time: f64, _is_tree_paused: bool) -> AdvanceTimeResult {
        if let Some(owner) = self.t.data.get_owner() {
            match self.check_owner_validity_and_pause_mode(owner) {
                ObjectValidityResult::CanProcess => {}
                ObjectValidityResult::DontProcess => return AdvanceTimeResult::Paused,
                ObjectValidityResult::SomeObjectsDead => {
                    self.stop();
                    return AdvanceTimeResult::ShouldDespawn;
                }
            }
        }

        let Some(step) = self.handle_time_step(delta_time) else { return AdvanceTimeResult::Playing };

        let set_fn = <T as PropertyType>::Data::set_property_value;

        let step_excess = match &mut self.t.lerp_mode {
            LerpMode::Absolute { duration, from } => {
                let start_val = match &from {
                    FromValue::AlreadyEvaluated(val) | FromValue::Explicit(val) => val,
                    FromValue::PendingEvaluation => {
                        let val_at_obj = self.t.data.get_property_value();
                        from.set_evaluated(val_at_obj)
                    }
                };

                let anim_pos =
                    calc_animation_position(*duration, self.loop_time, self.loop_counter, self.loop_mode, &self.t.ease);

                let target_val = self.t.lerper.spire_lerp(start_val, &self.t.to.eval(), anim_pos);

                set_fn(&mut self.t.data, target_val);
                self.loop_time - *duration
            }
            LerpMode::SpeedBased {
                speed,
                from,
                start_distance,
                step_sum,
            } => {
                let curr_val = self.t.data.get_property_value();

                let effective_end_val = match self.loop_mode {
                    LoopMode::Restart | LoopMode::Incremental => self.t.to.eval(),
                    LoopMode::Yoyo => {
                        if self.loop_counter % 2 == 0 {
                            self.t.to.eval()
                        } else {
                            from.get_or_evaluate(|| curr_val.clone()).clone()
                        }
                    }
                };

                let initial_end_val = self.t.to.eval();

                let start_distance_val = *start_distance.get_or_insert_with(|| {
                    let start_val = from.get_or_evaluate(|| curr_val.clone());
                    self.t.lerper.spire_distance(start_val, &initial_end_val)
                });

                let remaining_distance = self.t.lerper.spire_distance(&curr_val, &effective_end_val);

                if start_distance_val.is_zero_approx() {
                    *step_sum = 0.;
                    set_fn(&mut self.t.data, effective_end_val);
                    step
                } else {
                    const EPS: f64 = 0.01;
                    let distance_ratio = f64::max(0., 1. - (remaining_distance / start_distance_val));
                    let gain = (EPS + self.t.ease.sample(distance_ratio)) / (EPS + f64::abs(distance_ratio));

                    let actual_speed = *speed * gain;

                    let (target_val, step_result) =
                        self.t
                            .lerper
                            .spire_step(&curr_val, &effective_end_val, actual_speed, step + *step_sum);

                    set_fn(&mut self.t.data, target_val);

                    match step_result {
                        StepResult::Unfinished { accumulated_time } => {
                            *step_sum = accumulated_time;
                            -1.0
                        }
                        StepResult::Finished { excess_time } => {
                            *step_sum = 0.;
                            excess_time
                        }
                    }
                }
            }
            LerpMode::Relative {
                duration,
                relative_to,
                previous_anim_pos,
            } => {
                let curr_val = self.t.data.get_property_value();

                let next_anim_pos =
                    calc_animation_position(*duration, self.loop_time, self.loop_counter, self.loop_mode, &self.t.ease);

                let end = self.t.to.eval();

                let previous_relative = self.t.lerper.spire_lerp(relative_to, &end, *previous_anim_pos);
                let next_relative = self.t.lerper.spire_lerp(relative_to, &end, next_anim_pos);

                *previous_anim_pos = next_anim_pos;
                let target_val = self
                    .t
                    .lerper
                    .add_relative(&curr_val, &previous_relative, &next_relative);

                set_fn(&mut self.t.data, target_val);
                self.loop_time - *duration
            }
        };

        if step_excess < 0. {
            AdvanceTimeResult::Playing
        } else {
            if let LoopMode::Restart = self.loop_mode {
                self.t.lerp_mode.reset_state();
            }

            if let Some(excess_time) = self.handle_loop_finished(step_excess) {
                AdvanceTimeResult::Completed { excess_time }
            } else {
                AdvanceTimeResult::Playing
            }
        }
    }
}

// Builder Methods

// ------------------------------------------------------------
impl<T> SpireTween<LerpPropertyData<T>>
where
    T: PropertyType + Clone + Default + FromGodot,
    LerpPropertyData<T>: ITweenable,
{
    /// Chainable equivalent of [`set_ease`](Self::set_ease).
    #[inline]
    pub fn with_ease(mut self, ease: EaseKind) -> Self {
        self.set_ease(ease);
        self
    }

    /// Chainable equivalent of [`set_begin_value`](Self::set_begin_value).
    /// Forces the tween to start from `value` instead of reading the property's
    /// current value on the first tick.
    #[inline]
    pub fn begin_from(mut self, value: T) -> Self {
        self.set_begin_value(value);
        self
    }

    /// Chainable equivalent of [`set_final_value`](Self::set_final_value).
    /// Replaces any previously-set fixed target or dynamic-target callable.
    #[inline]
    pub fn end_at(mut self, value: T) -> Self {
        self.set_final_value(value);
        self
    }

    /// Chainable equivalent of [`set_absolute`](Self::set_absolute). Restores the
    /// default [`LerpMode::Absolute`] strategy.
    #[inline]
    pub fn as_absolute(mut self) -> Self
    where T: Default {
        self.set_absolute();
        self
    }

    /// Chainable equivalent of [`set_speed_based`](Self::set_speed_based). The
    /// `duration` argument supplied at construction is reinterpreted as `speed`
    /// (units per second).
    #[inline]
    pub fn as_speed_based(mut self) -> Self {
        self.set_speed_based();
        self
    }

    /// Chainable equivalent of [`set_relative`](Self::set_relative). The tween
    /// adds its interpolated offset to `relative_to` each tick instead of forcing
    /// the property onto a fixed `from → to` curve.
    #[inline]
    pub fn as_relative(mut self, relative_to: T) -> Self
    where T: Default + Clone + FromGodot {
        self.set_relative(relative_to);
        self
    }
}

impl<T> SpireTween<LerpPropertyData<T>>
where
    T: PropertyType,
    <T as ILerpable>::Lerper: Default,
    AnyTween: From<RcPtr<Self>>,
    LerpPropertyData<T>: ITweenable,
{
    /// Constructs a property tween in [`LerpMode::Absolute`] mode with the supplied
    /// data adapter, target evaluator, and duration. Default ease is
    /// [`EaseKind::default()`]; default lerper is `T::Lerper::default()`.
    ///
    /// You usually don't call this directly — use [`DoProperty::do_property`] or one
    /// of the per-property `Do*` shortcuts.
    pub fn new(data: T::Data, to: Evaluator<T>, duration: f64) -> Self {
        Self::new_with_data(LerpPropertyData {
            data,
            lerp_mode: LerpMode::absolute(duration),
            ease: Default::default(),
            to,
            lerper: Default::default(),
        })
    }

    /// Convenience: [`new`](Self::new) followed by [`register`](SpireTween::register).
    /// Submits the tween to the global `TweenManager` and returns the [`RcPtr`] handle.
    pub fn new_registered(data: T::Data, end: Evaluator<T>, duration: f64) -> RcPtr<Self>
    where AnyTween: From<RcPtr<Self>> {
        Self::new(data, end, duration).register()
    }

    pub fn new_typed(
        property_path: impl AsArg<NodePath>,
        owner: impl Into<ObjectOrNode>,
        to: T,
        duration: f64,
    ) -> Self
    where
        <T as PropertyType>::Data: From<PropertyDataCustom>,
    {
        Self::new_with_data(LerpPropertyData {
            data: PropertyDataCustom {
                path:  property_path.into_arg().cow_into_owned(),
                owner: owner.into(),
            }
            .into(),
            lerp_mode: LerpMode::absolute(duration),
            ease: Default::default(),
            to: Evaluator::Static(to),
            lerper: Default::default(),
        })
    }
}

// Variant Builder
impl SpireTween<LerpPropertyData<Variant>> {
    /// Constructs a `Variant`-typed property tween with a user-supplied
    /// [`CustomLerper`]. Use this when the tweened property has a type Spire doesn't
    /// natively support (anything outside `i64`/`f64`/`GString`/`Color`/`Vector2`/`Vector2i`/`Vector3`/`Vector3i`).
    ///
    /// The `CustomLerper` provides the four callbacks Spire needs (`lerp`,
    /// `add_relative`, `step`, `distance`); see its docs for which modes need which.
    pub fn new_custom(
        property_path: impl AsArg<NodePath>,
        owner: impl Into<ObjectOrNode>,
        to: Evaluator<Variant>,
        duration: f64,
        lerper: CustomLerper,
    ) -> Self {
        Self::new_with_data(LerpPropertyData {
            data: PropertyDataCustom {
                path:  property_path.into_arg().cow_into_owned(),
                owner: owner.into(),
            },
            lerp_mode: LerpMode::absolute(duration),
            ease: Default::default(),
            to,
            lerper,
        })
    }
}
