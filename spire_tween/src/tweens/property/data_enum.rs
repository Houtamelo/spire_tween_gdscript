use super::*;

#[delegated_enum(impl_conversions)]
#[derive(Clone)]
pub enum PropertyTween {
    //i32(RcPtr<SpireTween<LerpPropertyData<i32>>>),
    int(RcPtr<SpireTween<LerpPropertyData<i64>>>),
    //f32(RcPtr<SpireTween<LerpPropertyData<f32>>>),
    float(RcPtr<SpireTween<LerpPropertyData<f64>>>),
    String(RcPtr<SpireTween<LerpPropertyData<GString>>>),
    Color(RcPtr<SpireTween<LerpPropertyData<Color>>>),
    Vector2(RcPtr<SpireTween<LerpPropertyData<Vector2>>>),
    Vector2i(RcPtr<SpireTween<LerpPropertyData<Vector2i>>>),
    Vector3(RcPtr<SpireTween<LerpPropertyData<Vector3>>>),
    Vector3i(RcPtr<SpireTween<LerpPropertyData<Vector3i>>>),
    Variant(RcPtr<SpireTween<LerpPropertyData<Variant>>>),
}

#[delegate_impl]
impl SpireTweener for PropertyTween {
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
impl InnerTypeName for PropertyTween {
    fn inner_type_name(&self) -> &'static str;
}

impl_from_enum! {
    AnyTween::Property {
        //LerpPropertyData<i32> => PropertyTween::i32,
        LerpPropertyData<i64> => PropertyTween::int,
        //LerpPropertyData<f32> => PropertyTween::f32,
        LerpPropertyData<f64> => PropertyTween::float,
        LerpPropertyData<GString> => PropertyTween::String,
        LerpPropertyData<Color> => PropertyTween::Color,
        LerpPropertyData<Vector2> => PropertyTween::Vector2,
        LerpPropertyData<Vector2i> => PropertyTween::Vector2i,
        LerpPropertyData<Vector3> => PropertyTween::Vector3,
        LerpPropertyData<Vector3i> => PropertyTween::Vector3i,
        LerpPropertyData<Variant> => PropertyTween::Variant,
    }
}

/*
impl Deref for PropertyTween {
    type Target = SpireTween<dyn Any>;

    fn deref(&self) -> &Self::Target {
        delegate_property_tween! { self => |arg| &**arg }
    }
}

impl DerefMut for PropertyTween {
    fn deref_mut(&mut self) -> &mut Self::Target {
        delegate_property_tween! { self => |arg| &mut **arg }
    }
}
*/

define_base_methods! { PropertyTween }

#[delegate_impl]
impl PropertyTween {
    pub fn get_duration(&mut self) -> f64;
    pub fn set_duration(&mut self, new_duration: f64);

    pub fn get_owner(&self) -> &ObjectOrNode;
    pub fn set_owner(&mut self, owner: ObjectOrNode);

    pub fn get_ease(&self) -> &EaseKind;
    pub fn set_ease(&mut self, ease: EaseKind);

    pub fn is_absolute(&self) -> bool;
    pub fn is_relative(&self) -> bool;
    pub fn is_speed_based(&self) -> bool;

    pub fn set_absolute(&mut self);
    pub fn set_speed_based(&mut self);

    pub fn get_property_path(&self) -> NodePath;
}

impl PropertyTween {
    pub fn set_property_path(&mut self, property_path: NodePath) {
        let owner = self.get_owner().to_object();

        let property_str = &property_path.to_string();

        match self {
            /*
            PropertyTween::i32(tween) => {
                tween.t.data = <i32 as PropertyType>::Data::from_path_and_owner(
                    property_str,
                    property_path,
                    owner,
                );
            }
            */
            PropertyTween::int(tween) => {
                tween.t.data = <i64 as PropertyType>::Data::from_path_and_owner(property_str, property_path, owner);
            }
            /*
            PropertyTween::f32(tween) => {
                tween.t.data = <f32 as PropertyType>::Data::from_path_and_owner(
                    property_str,
                    property_path,
                    owner,
                );
            }
            */
            PropertyTween::float(tween) => {
                tween.t.data = <f64 as PropertyType>::Data::from_path_and_owner(property_str, property_path, owner);
            }
            PropertyTween::String(tween) => {
                tween.t.data = <GString as PropertyType>::Data::from_path_and_owner(property_str, property_path, owner);
            }
            PropertyTween::Color(tween) => {
                tween.t.data = <Color as PropertyType>::Data::from_path_and_owner(property_str, property_path, owner);
            }
            PropertyTween::Vector2(tween) => {
                tween.t.data = <Vector2 as PropertyType>::Data::from_path_and_owner(property_str, property_path, owner);
            }
            PropertyTween::Vector2i(tween) => {
                tween.t.data =
                    <Vector2i as PropertyType>::Data::from_path_and_owner(property_str, property_path, owner);
            }
            PropertyTween::Vector3(tween) => {
                tween.t.data = <Vector3 as PropertyType>::Data::from_path_and_owner(property_str, property_path, owner);
            }
            PropertyTween::Vector3i(tween) => {
                tween.t.data =
                    <Vector3i as PropertyType>::Data::from_path_and_owner(property_str, property_path, owner);
            }
            PropertyTween::Variant(tween) => {
                tween.t.data = <Variant as PropertyType>::Data::from_path_and_owner(property_str, property_path, owner);
            }
        }
    }

    pub fn get_final_value(&mut self) -> Variant {
        delegate_property_tween! {
            self => |this| this.get_final_value().to_variant()
        }
    }

    pub fn set_final_value(&mut self, value: Variant) -> Result<(), ConvertError> {
        delegate_property_tween!(self => |this| {
            this.t.to = value.try_to_relaxed()?;
            Ok(())
        })
    }

    pub fn set_relative(&mut self, relative_to: Variant) {
        delegate_property_tween!(self => |this| {
            if let Some(relative_to) = relative_to.try_to_relaxed().log_if_err() {
                this.t.lerp_mode = LerpMode::Relative {
                    duration: 0.0 ,
                    relative_to,
                    previous_anim_pos: 0.,
                };
            }
        })
    }
}

#[delegated_enum(impl_conversions)]
#[derive(Clone)]
pub enum WeakPropertyTween {
    //i32(WeakPtr<SpireTween<LerpPropertyData<i32>>>),
    int(WeakPtr<SpireTween<LerpPropertyData<i64>>>),
    //f32(WeakPtr<SpireTween<LerpPropertyData<f32>>>),
    float(WeakPtr<SpireTween<LerpPropertyData<f64>>>),
    String(WeakPtr<SpireTween<LerpPropertyData<GString>>>),
    Color(WeakPtr<SpireTween<LerpPropertyData<Color>>>),
    Vector2(WeakPtr<SpireTween<LerpPropertyData<Vector2>>>),
    Vector2i(WeakPtr<SpireTween<LerpPropertyData<Vector2i>>>),
    Vector3(WeakPtr<SpireTween<LerpPropertyData<Vector3>>>),
    Vector3i(WeakPtr<SpireTween<LerpPropertyData<Vector3i>>>),
    Variant(WeakPtr<SpireTween<LerpPropertyData<Variant>>>),
}

impl WeakPropertyTween {
    #[inline]
    pub fn upgrade(&self) -> Option<PropertyTween> {
        delegate_weak_property_tween! { self => |arg| arg.upgrade().map(Into::into) }
    }
}

impl_from_enum_weak! {
    WeakAnyTween::Property {
        //LerpPropertyData<i32> => WeakPropertyTween::i32,
        LerpPropertyData<i64> => WeakPropertyTween::int,
        //LerpPropertyData<f32> => WeakPropertyTween::f32,
        LerpPropertyData<f64> => WeakPropertyTween::float,
        LerpPropertyData<GString> => WeakPropertyTween::String,
        LerpPropertyData<Color> => WeakPropertyTween::Color,
        LerpPropertyData<Vector2> => WeakPropertyTween::Vector2,
        LerpPropertyData<Vector2i> => WeakPropertyTween::Vector2i,
        LerpPropertyData<Vector3> => WeakPropertyTween::Vector3,
        LerpPropertyData<Vector3i> => WeakPropertyTween::Vector3i,
        LerpPropertyData<Variant> => WeakPropertyTween::Variant,
    }
}

define_tween_ptr_methods! { Strong = PropertyTween; Weak = WeakPropertyTween; }
