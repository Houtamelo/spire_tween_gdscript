use super::*;
use crate::modes::SpireProcessMode;

impl<T> SpireTween<T> {
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

    pub fn with_pause_mode(self, pause_mode: SpirePauseMode) -> Self { Self { pause_mode, ..self } }

    pub fn with_process_mode(self, process_mode: SpireProcessMode) -> Self {
        Self {
            process_mode,
            ..self
        }
    }

    pub fn run_once(self) -> Self {
        Self {
            loop_mode: LoopMode::Finite(0),
            ..self
        }
    }

    pub fn looped(self, loops: u32) -> Self {
        Self {
            loop_mode: LoopMode::Finite(loops),
            ..self
        }
    }

    pub fn infinite(self) -> Self {
        Self {
            loop_mode: LoopMode::Infinite,
            ..self
        }
    }

    pub fn on_finish(mut self, f: impl FnMut() + 'static) -> Self {
        self.calls_on_finish.push(f.into());
        self
    }

    pub fn on_finish_callable(mut self, callable: Callable) -> Self {
        self.calls_on_finish.push(callable.into());
        self
    }
}

impl<T> SpireTween<T>
where AnyTween: From<Self>
{
    pub fn register(self) -> SpireHandle<T> {
        let id = TWEENS.register(self.into());
        SpireHandle::new(id)
    }
}
