use super::*;

#[delegated_enum(impl_conversions)]
#[derive(Clone)]
pub enum AnyMethodTween {
    int(TweenPtr<LerpMethodData<i64>>),
    float(TweenPtr<LerpMethodData<f64>>),
    String(TweenPtr<LerpMethodData<GString>>),
    Color(TweenPtr<LerpMethodData<Color>>),
    Vector2(TweenPtr<LerpMethodData<Vector2>>),
    Vector2i(TweenPtr<LerpMethodData<Vector2i>>),
    Vector3(TweenPtr<LerpMethodData<Vector3>>),
    Vector3i(TweenPtr<LerpMethodData<Vector3i>>),
    Variant(TweenPtr<LerpMethodData<Variant>>),
}

#[delegate_impl]
impl SpireTweener for AnyMethodTween {
    #[inline]
    fn play(&mut self);
    #[inline]
    fn pause(&mut self);
    #[inline]
    fn stop(&mut self);
    #[inline]
    fn process_internal(&mut self, delta_time: f64, is_tree_paused: bool) -> AdvanceTimeResult;
    #[inline]
    fn force_complete(&mut self);
}

#[delegate_impl]
impl InnerTypeName for AnyMethodTween {
    fn inner_type_name(&self) -> &'static str;
}

impl_from_enum! {
    AnyTween::Method {
        LerpMethodData<i64> => AnyMethodTween::int,
        LerpMethodData<f64> => AnyMethodTween::float,
        LerpMethodData<GString> => AnyMethodTween::String,
        LerpMethodData<Color> => AnyMethodTween::Color,
        LerpMethodData<Vector2> => AnyMethodTween::Vector2,
        LerpMethodData<Vector2i> => AnyMethodTween::Vector2i,
        LerpMethodData<Vector3> => AnyMethodTween::Vector3,
        LerpMethodData<Vector3i> => AnyMethodTween::Vector3i,
        LerpMethodData<Variant> => AnyMethodTween::Variant,
    }
}

define_base_methods! { AnyMethodTween }

#[delegate_impl]
impl AnyMethodTween {
    #[inline]
    pub fn get_callable(&self) -> &Callable;

    #[inline]
    pub fn get_owner(&self) -> Option<&ObjectOrNode>;

    #[inline]
    pub fn get_duration(&self) -> f64;

    #[inline]
    pub fn get_ease(&self) -> &EaseKind;
    #[inline]
    pub fn set_ease(&mut self, ease: EaseKind);
}

impl AnyMethodTween {
    #[inline]
    pub fn get_start_value(&self) -> Variant {
        delegate_any_method_tween! { self.t.from.to_variant() }
    }

    #[inline]
    pub fn set_start_value(&mut self, value: Variant) -> Result<(), ConvertError> {
        delegate_any_method_tween!(self => |this| {
            this.t.from = value.try_to_relaxed()?;
            Ok(())
        })
    }

    #[inline]
    pub fn get_final_value(&self) -> Variant {
        delegate_any_method_tween! { self.t.to.to_variant() }
    }

    #[inline]
    pub fn set_final_value(&mut self, value: Variant) -> Result<(), ConvertError> {
        delegate_any_method_tween!(self => |this| {
            this.t.to = value.try_to_relaxed()?;
            Ok(())
        })
    }
}

#[delegated_enum(impl_conversions)]
#[derive(Clone)]
pub enum WeakMethodTween {
    int(WeakTweenPtr<LerpMethodData<i64>>),
    float(WeakTweenPtr<LerpMethodData<f64>>),
    String(WeakTweenPtr<LerpMethodData<GString>>),
    Color(WeakTweenPtr<LerpMethodData<Color>>),
    Vector2(WeakTweenPtr<LerpMethodData<Vector2>>),
    Vector2i(WeakTweenPtr<LerpMethodData<Vector2i>>),
    Vector3(WeakTweenPtr<LerpMethodData<Vector3>>),
    Vector3i(WeakTweenPtr<LerpMethodData<Vector3i>>),
    Variant(WeakTweenPtr<LerpMethodData<Variant>>),
}

impl WeakMethodTween {
    #[inline]
    pub fn upgrade(&self) -> Option<AnyMethodTween> {
        delegate_weak_method_tween! { self.upgrade().map(Into::into) }
    }
}

impl_from_enum_weak! {
    WeakAnyTween::Method {
        LerpMethodData<i64> => WeakMethodTween::int,
        LerpMethodData<f64> => WeakMethodTween::float,
        LerpMethodData<GString> => WeakMethodTween::String,
        LerpMethodData<Color> => WeakMethodTween::Color,
        LerpMethodData<Vector2> => WeakMethodTween::Vector2,
        LerpMethodData<Vector2i> => WeakMethodTween::Vector2i,
        LerpMethodData<Vector3> => WeakMethodTween::Vector3,
        LerpMethodData<Vector3i> => WeakMethodTween::Vector3i,
        LerpMethodData<Variant> => WeakMethodTween::Variant,
    }
}

define_tween_ptr_methods! { Strong = AnyMethodTween; Weak = WeakMethodTween; }
