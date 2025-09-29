use super::*;
use crate::tweens::tween_callable::CallableData;

#[delegated_enum(impl_conversions)]
pub enum AnyTween {
    Property(PropertyTween),
    Method(MethodTween),
    Callable(SpireTween<CallableData>),
    DelayedCall(SpireTween<DelayedCallData>),
    Sequence(SpireTween<Sequence>),
}

#[delegate_impl]
impl SpireTweener for AnyTween {
    #[inline]
    fn get_state(&self) -> TweenState;
    #[inline]
    fn set_state(&mut self, state: TweenState);
}

#[delegate_impl]
impl AdvanceTime for AnyTween {
    fn advance_time(&mut self, delta_time: f64) -> f64;
    fn complete(&mut self);
}

#[delegate_impl]
impl InnerTypeName for AnyTween {
    fn inner_type_name(&self) -> &'static str;
}

define_base_methods! { AnyTween }
