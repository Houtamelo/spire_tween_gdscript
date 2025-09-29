use super::*;

impl<Val> SpireHandle<LerpPropertyData<Val>>
where
    Val: PropertyType,
    SpireTween<LerpPropertyData<Val>>:
        FromEnum<AnyTween> + FromEnumMut<AnyTween> + FromEnumRef<AnyTween>,
{
    pub fn get_property_path(&mut self) -> Result<NodePath, FetchError> {
        self.map_mut(|tween| tween.t.data.get_property_path())
    }

    pub fn set_property_path(&mut self, property_path: NodePath) -> Result<(), FetchError> {
        self.map_mut(|tween| {
            let owner = tween.t.data.get_owner().to_object();
            let property_str = &property_path.to_string();

            tween.t.data =
                IGeneralPropertyData::from_path_and_owner(property_str, property_path, owner);
        })
    }

    pub fn get_owner(&mut self) -> Result<ObjectOrNode, FetchError> {
        self.map_mut(|tween| tween.t.data.get_owner().clone())
    }

    pub fn set_owner(&mut self, owner: Gd<Object>) -> Result<(), FetchError> {
        self.map_mut(|tween| {
            if !tween.t.data.try_set_owner(ObjectOrNode::from_unchecked_object(owner)) {
                godot_error!("Failed to set owner, object does not match the property's expect owner type `{}`.", tween.t.data.owner_class());
            }
        })
    }

    pub fn get_lerp_mode(&mut self) -> Result<LerpMode<Val>, FetchError>
    where LerpMode<Val>: Clone {
        self.map_mut(|tween| tween.t.lerp_mode.clone())
    }

    pub fn set_lerp_mode(&mut self, lerp_mode: LerpMode<Val>) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.t.lerp_mode = lerp_mode)
    }

    pub fn get_ease(&mut self) -> Result<Ease, FetchError> {
        self.map_mut(|tween| tween.t.ease.clone())
    }

    pub fn set_ease(&mut self, ease: Ease) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.t.ease = ease)
    }

    pub fn get_final_value(&mut self) -> Result<Val, FetchError>
    where Val: Clone + Default + FromGodot {
        self.map_mut(|tween| tween.t.to.eval())
    }

    pub fn set_final_value(&mut self, value: Val) -> Result<(), FetchError> {
        self.map_mut(|tween| tween.t.to = Evaluator::Static(value))
    }

    pub fn is_absolute(&mut self) -> Result<bool, FetchError> {
        self.map_mut(|tween| tween.is_absolute())
    }

    pub fn is_relative(&mut self) -> Result<bool, FetchError> {
        self.map_mut(|tween| tween.is_relative())
    }

    pub fn is_speed_based(&mut self) -> Result<bool, FetchError> {
        self.map_mut(|tween| tween.is_speed_based())
    }
}
