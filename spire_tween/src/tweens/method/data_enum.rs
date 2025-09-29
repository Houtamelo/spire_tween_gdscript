use super::*;

#[delegated_enum(impl_conversions)]
pub enum MethodTween {
    i32(SpireTween<MethodData<i32>>),
    i64(SpireTween<MethodData<i64>>),
    f32(SpireTween<MethodData<f32>>),
    f64(SpireTween<MethodData<f64>>),
    String(SpireTween<MethodData<GString>>),
    Color(SpireTween<MethodData<Color>>),
    Vector2(SpireTween<MethodData<Vector2>>),
    Vector3(SpireTween<MethodData<Vector3>>),
    Variant(SpireTween<MethodData<Variant>>),
}

#[delegate_impl]
impl SpireTweener for MethodTween {
    #[inline]
    fn get_state(&self) -> TweenState;
    #[inline]
    fn set_state(&mut self, state: TweenState);
}

#[delegate_impl]
impl AdvanceTime for MethodTween {
    fn advance_time(&mut self, delta_time: f64) -> f64;
    fn complete(&mut self);
}

#[delegate_impl]
impl InnerTypeName for MethodTween {
    fn inner_type_name(&self) -> &'static str;
}

impl_from_enum! {
    AnyTween::Method {
        MethodData<i32> => MethodTween::i32,
        MethodData<i64> => MethodTween::i64,
        MethodData<f32> => MethodTween::f32,
        MethodData<f64> => MethodTween::f64,
        MethodData<GString> => MethodTween::String,
        MethodData<Color> => MethodTween::Color,
        MethodData<Vector2> => MethodTween::Vector2,
        MethodData<Vector3> => MethodTween::Vector3,
        MethodData<Variant> => MethodTween::Variant,
    }
}

define_base_methods! { MethodTween }

#[delegate_impl]
impl MethodTween {
    pub fn get_method_name(&self) -> StringName;
    pub fn set_method_name(&mut self, method_name: StringName);

    pub fn get_owner(&self) -> &ObjectOrNode;
    pub fn set_owner(&mut self, owner: impl Into<ObjectOrNode>);

    pub fn get_duration(&self) -> f64;
    pub fn set_duration(&mut self, duration: f64);

    pub fn get_ease(&self) -> Ease;
    pub fn set_ease(&mut self, ease: Ease);
}

impl MethodTween {
    pub fn get_start_value(&self) -> Variant {
        delegate_method_tween! { self.t.from.to_variant() }
    }

    pub fn set_start_value(&mut self, value: Variant) -> Result<(), ConvertError> {
        delegate_method_tween!(self => |this| {
            this.t.from = value.try_to()?;
            Ok(())
        })
    }

    pub fn get_final_value(&self) -> Variant {
        delegate_method_tween! { self.t.to.to_variant() }
    }

    pub fn set_final_value(&mut self, value: Variant) -> Result<(), ConvertError> {
        delegate_method_tween!(self => |this| {
            this.t.to = value.try_to()?;
            Ok(())
        })
    }
}
