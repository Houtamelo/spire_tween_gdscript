use super::*;

impl<T: ITweenable> SpireTween<T> {
    pub fn bound_to(mut self, node: Gd<Node>) -> Self {
        // Binding process is finished in TweensMap::register
        self.bound_nodes.insert(node);
        self
    }

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

    pub fn with_handle(mut self, handle: T::GdHandle) -> Self {
        self.gd_handle = Some(handle);
        self
    }
}

impl<T: ITweenable> SpireTween<T>
where AnyTween: From<RcPtr<Self>>
{
    pub fn register(self) -> RcPtr<Self> {
        let tween = RcPtr::new(self);
        TM.tween_register(tween.clone());
        tween
    }
}
