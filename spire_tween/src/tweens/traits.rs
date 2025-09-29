use crate::internal_prelude::*;

pub trait SpireTweener {
    fn get_state(&self) -> TweenState;
    fn set_state(&mut self, state: TweenState);

    #[inline]
    fn play(&mut self) { self.set_state(TweenState::Playing); }
    #[inline]
    fn pause(&mut self) { self.set_state(TweenState::Paused); }
    #[inline]
    fn stop(&mut self) { self.set_state(TweenState::Stopped); }

    #[inline]
    fn is_playing(&self) -> bool { matches!(self.get_state(), TweenState::Playing) }
    #[inline]
    fn is_paused(&self) -> bool { matches!(self.get_state(), TweenState::Paused) }
    #[inline]
    fn is_stopped(&self) -> bool { matches!(self.get_state(), TweenState::Stopped) }
}

pub trait AdvanceTime: SpireTweener {
    fn advance_time(&mut self, delta_time: f64) -> f64;
    fn complete(&mut self);
}

#[doc(hidden)]
pub trait InnerTypeName {
    fn inner_type_name(&self) -> &'static str;
}
