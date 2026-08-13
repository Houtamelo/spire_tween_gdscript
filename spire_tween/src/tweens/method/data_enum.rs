use super::*;

#[delegated_enum(impl_conversions)]
#[derive(Clone)]
pub enum AnyMethodTween {
    int(RcPtr<SpireTween<LerpMethodData<i64>>>),
    float(RcPtr<SpireTween<LerpMethodData<f64>>>),
    String(RcPtr<SpireTween<LerpMethodData<GString>>>),
    Color(RcPtr<SpireTween<LerpMethodData<Color>>>),
    Vector2(RcPtr<SpireTween<LerpMethodData<Vector2>>>),
    Vector2i(RcPtr<SpireTween<LerpMethodData<Vector2i>>>),
    Vector3(RcPtr<SpireTween<LerpMethodData<Vector3>>>),
    Vector3i(RcPtr<SpireTween<LerpMethodData<Vector3i>>>),
    Variant(RcPtr<SpireTween<LerpMethodData<Variant>>>),
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
    fn process(&mut self, delta_time: f64, is_tree_paused: bool) -> AdvanceTimeResult;
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
    int(WeakPtr<SpireTween<LerpMethodData<i64>>>),
    float(WeakPtr<SpireTween<LerpMethodData<f64>>>),
    String(WeakPtr<SpireTween<LerpMethodData<GString>>>),
    Color(WeakPtr<SpireTween<LerpMethodData<Color>>>),
    Vector2(WeakPtr<SpireTween<LerpMethodData<Vector2>>>),
    Vector2i(WeakPtr<SpireTween<LerpMethodData<Vector2i>>>),
    Vector3(WeakPtr<SpireTween<LerpMethodData<Vector3>>>),
    Vector3i(WeakPtr<SpireTween<LerpMethodData<Vector3i>>>),
    Variant(WeakPtr<SpireTween<LerpMethodData<Variant>>>),
}

impl WeakMethodTween {
    #[inline]
    pub fn upgrade(&self) -> Option<AnyMethodTween> {
        delegate_weak_method_tween! { self => |arg| arg.upgrade().map(Into::into) }
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
