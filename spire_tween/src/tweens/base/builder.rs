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

    pub fn with_delay(self, delay: f64) -> Self { Self { delay, ..self } }

    pub fn with_speed_scale(self, speed_scale: f64) -> Self {
        Self {
            speed_scale,
            ..self
        }
    }

    pub fn with_pause_mode(self, pause_mode: PauseMode) -> Self { Self { pause_mode, ..self } }

    pub fn with_process_mode(self, process_mode: ProcessMode) -> Self {
        Self {
            process_mode,
            ..self
        }
    }

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
    /// Automatically attaches a GD handle so `finished`/`loop_finished` signals work.
    pub fn register(self) -> RcPtr<Self> {
        let tween = RcPtr::new(self);
        if tween.to_mut().gd_handle.is_none() {
            T::attach_gd_handle(&tween);
        }
        TM.tween_register(tween.clone());
        tween
    }
}
