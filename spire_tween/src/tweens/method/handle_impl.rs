use super::*;

impl<Val> SpireHandle<MethodData<Val>>
where
    SpireTween<MethodData<Val>>: FromEnumMut<AnyTween>,
    Val: Clone,
{
    pub fn get_method_name(&mut self) -> Result<StringName, FetchError> {
        self.map_mut(|tween| tween.t.method_name.clone())
    }

    pub fn set_method_name(&mut self, method_name: StringName) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.t.method_name = method_name)
    }

    pub fn get_owner(&mut self) -> Result<ObjectOrNode, FetchError> {
        self.map_mut(|tween| tween.t.owner.clone())
    }

    pub fn set_owner(&mut self, owner: ObjectOrNode) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.t.owner = owner)
    }

    pub fn get_duration(&mut self) -> Result<f64, FetchError> {
        self.map_mut(|tween| tween.t.duration)
    }

    pub fn set_duration(&mut self, duration: f64) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.t.duration = duration)
    }

    pub fn get_ease(&mut self) -> Result<Ease, FetchError> {
        self.map_mut(|tween| tween.t.ease.clone())
    }

    pub fn set_ease(&mut self, ease: Ease) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.t.ease = ease)
    }

    pub fn get_start_value(&mut self) -> Result<Val, FetchError> {
        self.map_mut(|tween| tween.t.from.clone())
    }

    pub fn set_start_value(&mut self, start: Val) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.t.from = start)
    }

    pub fn get_final_value(&mut self) -> Result<Val, FetchError> {
        self.map_mut(|tween| tween.t.to.clone())
    }

    pub fn set_final_value(&mut self, end: Val) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.t.to = end)
    }
}
