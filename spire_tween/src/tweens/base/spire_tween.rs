use super::*;

#[must_use]
pub struct SpireTween<T> {
    pub bound_nodes: SmolSet<[Gd<Node>; 1]>,
    pub delay: f64,
    pub speed_scale: f64,
    pub elapsed_time: f64,
    pub cycle_count: u32,
    pub pause_mode: SpirePauseMode,
    pub process_mode: SpireProcessMode,
    pub loop_mode: LoopMode,
    pub calls_on_finish: Vec<DelayedCallData>,
    pub t: T,
    #[doc(hidden)]
    // Hidden because manually setting the state of a tween is not recommended.
    pub state: TweenState,
}

impl<T> SpireTween<T> {
    pub(crate) fn bound_nodes_mut(&mut self) -> &mut SmolSet<[Gd<Node>; 1]> {
        &mut self.bound_nodes
    }

    pub fn get_bound_nodes(&self) -> SmolSetIter<[Gd<Node>; 1]> { self.bound_nodes.iter() }

    pub fn clear_bound_nodes(&mut self) { self.bound_nodes.clear(); }

    pub fn get_delay(&self) -> f64 { self.delay }
    pub fn set_delay(&mut self, delay: f64) { self.delay = delay; }

    pub fn get_speed_scale(&self) -> f64 { self.speed_scale }
    pub fn set_speed_scale(&mut self, speed_scale: f64) { self.speed_scale = speed_scale; }

    pub fn get_elapsed_time(&self) -> f64 { self.elapsed_time }
    pub fn get_cycles_elapsed(&self) -> u32 { self.cycle_count }

    pub fn get_pause_mode(&self) -> SpirePauseMode { self.pause_mode }
    pub fn set_pause_mode(&mut self, pause_mode: SpirePauseMode) { self.pause_mode = pause_mode; }

    pub fn get_process_mode(&self) -> SpireProcessMode { self.process_mode }
    pub fn set_process_mode(&mut self, process_mode: SpireProcessMode) {
        self.process_mode = process_mode;
    }

    pub fn get_loop_mode(&self) -> LoopMode { self.loop_mode }
    pub fn set_loop_mode(&mut self, loop_mode: LoopMode) { self.loop_mode = loop_mode; }

    pub fn finished_connect(&mut self, f: impl FnMut() + 'static) {
        self.calls_on_finish.push(f.into());
    }

    pub fn finished_connect_callable(&mut self, callable: Callable) {
        self.calls_on_finish.push(callable.into());
    }

    pub fn finished_clear_connections(&mut self) { self.calls_on_finish.clear(); }
}

impl<T> SpireTween<T>
where Self: SpireTweener
{
    #[inline]
    pub(crate) fn handle_finished(&mut self) {
        self.stop();
        self.calls_on_finish
            .iter_mut()
            .for_each(DelayedCallData::invoke);
    }
}

impl<T> InnerTypeName for SpireTween<T> {
    fn inner_type_name(&self) -> &'static str { type_name::<T>() }
}
