use super::*;

impl<T> SpireHandle<T>
where SpireTween<T>: FromEnum<AnyTween> + FromEnumMut<AnyTween> + FromEnumRef<AnyTween>
{
    pub fn get_bound_nodes(&self) -> Result<Vec<Gd<Node>>, FetchError> {
        self.map(|tween| tween.get_bound_nodes().cloned().collect())
    }

    pub fn clear_bound_nodes(&mut self) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.clear_bound_nodes())
    }

    pub fn bind_node(&mut self, node: Gd<Node>) -> Result<(), FetchError> {
        if TWEENS.bind_bothways(node, self.id) {
            Ok(())
        } else {
            Err(FetchError::NotFound)
        }
    }

    pub fn unbind_node(&mut self, node: Gd<Node>) -> Result<(), FetchError> {
        if TWEENS.node_unbind(node, self.id) {
            Ok(())
        } else {
            Err(FetchError::NotFound)
        }
    }

    pub fn get_delay(&self) -> Result<f64, FetchError> { self.map(|tween| tween.get_delay()) }

    pub fn set_delay(&mut self, delay: f64) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.set_delay(delay))
    }

    pub fn get_speed_scale(&self) -> Result<f64, FetchError> {
        self.map(|tween| tween.get_speed_scale())
    }

    pub fn set_speed_scale(&mut self, speed_scale: f64) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.set_speed_scale(speed_scale))
    }

    pub fn get_elapsed_time(&self) -> Result<f64, FetchError> {
        self.map(|tween| tween.get_elapsed_time())
    }

    pub fn get_cycles_elapsed(&self) -> Result<u32, FetchError> {
        self.map(|tween| tween.get_cycles_elapsed())
    }

    pub fn get_pause_mode(&self) -> Result<SpirePauseMode, FetchError> {
        self.map(|tween| tween.get_pause_mode())
    }

    pub fn set_pause_mode(&mut self, pause_mode: SpirePauseMode) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.set_pause_mode(pause_mode))
    }

    pub fn get_process_mode(&self) -> Result<SpireProcessMode, FetchError> {
        self.map(|tween| tween.get_process_mode())
    }

    pub fn set_process_mode(&mut self, process_mode: SpireProcessMode) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.set_process_mode(process_mode))
    }

    pub fn get_loop_mode(&self) -> Result<LoopMode, FetchError> {
        self.map(|tween| tween.get_loop_mode())
    }

    pub fn set_loop_mode(&mut self, loop_mode: LoopMode) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.set_loop_mode(loop_mode))
    }

    pub fn on_finish(&mut self, f: impl FnMut() + 'static) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.finished_connect(f))
    }

    pub fn on_finish_callable(&mut self, callable: Callable) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.finished_connect_callable(callable))
    }
}
