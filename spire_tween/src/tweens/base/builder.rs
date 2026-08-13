use super::*;

impl<T: ITweenable> SpireTween<T> {
    /// Binds this tween to a node. If the node is freed, the tween stops.
    /// Pause behavior depends on [`PauseMode`]. Multiple nodes can be bound by chaining.
    pub fn bound_to(mut self, node: Gd<Node>) -> Self {
        // Binding process is finished in TweensMap::register
        self.bound_nodes.insert(node);
        self
    }

    /// Binds if the object can be cast to `Node`, otherwise no-op.
    pub fn maybe_bound(mut self, obj: Gd<Object>) -> Self {
        if let Ok(node) = obj.try_cast::<Node>() {
            self.bound_nodes.insert(node);
        }

        self
    }

    /// Chainable equivalent of [`set_delay`](SpireTween::set_delay). Sets the
    /// pre-play delay in seconds.
    pub fn with_delay(self, delay: f64) -> Self { Self { delay, ..self } }

    /// Chainable equivalent of [`set_speed_scale`](SpireTween::set_speed_scale).
    /// Multiplier applied to `delta_time` (`0.0` pauses, `2.0` runs twice as fast).
    pub fn with_speed_scale(self, speed_scale: f64) -> Self { Self { speed_scale, ..self } }

    /// Chainable equivalent of [`set_pause_mode`](SpireTween::set_pause_mode).
    /// Default is [`PauseMode::Bound`].
    pub fn with_pause_mode(self, pause_mode: PauseMode) -> Self { Self { pause_mode, ..self } }

    /// Chainable equivalent of [`set_process_mode`](SpireTween::set_process_mode).
    /// Default is [`ProcessMode::Idle`].
    pub fn with_process_mode(self, process_mode: ProcessMode) -> Self { Self { process_mode, ..self } }

    /// Attaches a GDScript-facing handle used to emit Godot signals.
    /// Rust-only users do not need this.
    pub fn with_handle(mut self, handle: T::GdHandle) -> Self {
        self.gd_handle = Some(handle);
        self
    }
}

impl<T: ITweenable> SpireTween<T>
where AnyTween: From<RcPtr<Self>>
{
    /// Submits this tween to the global `TweenManager` and returns an `RcPtr` handle.
    ///
    /// Pure Rust path: no GDScript-facing `Gd` handle is attached. Connect to
    /// `finished`/`loop_finished` events via `finished_connect`/`loop_finished_connect`
    /// (closure-based). Use this when no GDScript code needs to listen for the events.
    pub fn register(self) -> RcPtr<Self> {
        let tween = RcPtr::new(self);
        TM.tween_register(tween.clone());
        tween
    }

    /// Submits this tween to the global `TweenManager` and returns an `RcPtr` handle,
    /// also attaching a GDScript-facing `Gd` handle.
    ///
    /// Use this when GDScript code (or any consumer of Godot signals) needs to listen
    /// for `finished`/`loop_finished`. Pure Rust users should prefer `register` plus
    /// `finished_connect`/`loop_finished_connect`.
    pub fn register_with_gd_handle(self) -> RcPtr<Self> {
        let tween = RcPtr::new(self);
        if tween.to_mut().gd_handle.is_none() {
            T::attach_gd_handle(&tween);
        }
        TM.tween_register(tween.clone());
        tween
    }
}
