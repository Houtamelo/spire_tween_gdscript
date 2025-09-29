use super::*;

#[delegated_enum(impl_conversions)]
pub enum PropertyTween {
    i32(SpireTween<LerpPropertyData<i32>>),
    i64(SpireTween<LerpPropertyData<i64>>),
    f32(SpireTween<LerpPropertyData<f32>>),
    f64(SpireTween<LerpPropertyData<f64>>),
    GString(SpireTween<LerpPropertyData<GString>>),
    Color(SpireTween<LerpPropertyData<Color>>),
    Vector2(SpireTween<LerpPropertyData<Vector2>>),
    Vector3(SpireTween<LerpPropertyData<Vector3>>),
    Variant(SpireTween<LerpPropertyData<Variant>>),
}

#[delegate_impl]
impl SpireTweener for PropertyTween {
    #[inline]
    fn get_state(&self) -> TweenState;
    #[inline]
    fn set_state(&mut self, state: TweenState);

    #[inline]
    fn play(&mut self);
    #[inline]
    fn pause(&mut self);
    #[inline]
    fn stop(&mut self);

    #[inline]
    fn is_playing(&self) -> bool;
    #[inline]
    fn is_paused(&self) -> bool;
    #[inline]
    fn is_stopped(&self) -> bool;
}

#[delegate_impl]
impl AdvanceTime for PropertyTween {
    fn advance_time(&mut self, delta_time: f64) -> f64;
    fn complete(&mut self);
}

#[delegate_impl]
impl InnerTypeName for PropertyTween {
    fn inner_type_name(&self) -> &'static str;
}

impl_from_enum! {
    AnyTween::Property {
        LerpPropertyData<i32> => PropertyTween::i32,
        LerpPropertyData<i64> => PropertyTween::i64,
        LerpPropertyData<f32> => PropertyTween::f32,
        LerpPropertyData<f64> => PropertyTween::f64,
        LerpPropertyData<GString> => PropertyTween::GString,
        LerpPropertyData<Color> => PropertyTween::Color,
        LerpPropertyData<Vector2> => PropertyTween::Vector2,
        LerpPropertyData<Vector3> => PropertyTween::Vector3,
        LerpPropertyData<Variant> => PropertyTween::Variant,
    }
}

define_base_methods! { PropertyTween }

#[delegate_impl]
impl PropertyTween {
    pub fn get_duration(&mut self) -> f64;
    pub fn set_duration(&mut self, new_duration: f64);

    pub fn get_owner(&self) -> &ObjectOrNode;
    pub fn set_owner(&mut self, owner: ObjectOrNode);

    pub fn get_ease(&self) -> Ease;
    pub fn set_ease(&mut self, ease: Ease);

    pub fn is_absolute(&self) -> bool;
    pub fn is_relative(&self) -> bool;
    pub fn is_speed_based(&self) -> bool;

    pub fn set_absolute(&mut self);
    pub fn set_speed_based(&mut self, speed: f64);

    pub fn get_property_path(&self) -> NodePath;
}

impl PropertyTween {
    pub fn set_property_path(&mut self, property_path: NodePath) {
        let owner = self.get_owner().to_object();

        let property_str = &property_path.to_string();

        match self {
            PropertyTween::i32(tween) => {
                tween.t.data = <i32 as PropertyType>::Data::from_path_and_owner(
                    property_str,
                    property_path,
                    owner,
                );
            }
            PropertyTween::i64(tween) => {
                tween.t.data = <i64 as PropertyType>::Data::from_path_and_owner(
                    property_str,
                    property_path,
                    owner,
                );
            }
            PropertyTween::f32(tween) => {
                tween.t.data = <f32 as PropertyType>::Data::from_path_and_owner(
                    property_str,
                    property_path,
                    owner,
                );
            }
            PropertyTween::f64(tween) => {
                tween.t.data = <f64 as PropertyType>::Data::from_path_and_owner(
                    property_str,
                    property_path,
                    owner,
                );
            }
            PropertyTween::GString(tween) => {
                tween.t.data = <GString as PropertyType>::Data::from_path_and_owner(
                    property_str,
                    property_path,
                    owner,
                );
            }
            PropertyTween::Color(tween) => {
                tween.t.data = <Color as PropertyType>::Data::from_path_and_owner(
                    property_str,
                    property_path,
                    owner,
                );
            }
            PropertyTween::Vector2(tween) => {
                tween.t.data = <Vector2 as PropertyType>::Data::from_path_and_owner(
                    property_str,
                    property_path,
                    owner,
                );
            }
            PropertyTween::Vector3(tween) => {
                tween.t.data = <Vector3 as PropertyType>::Data::from_path_and_owner(
                    property_str,
                    property_path,
                    owner,
                );
            }
            PropertyTween::Variant(tween) => {
                tween.t.data = <Variant as PropertyType>::Data::from_path_and_owner(
                    property_str,
                    property_path,
                    owner,
                );
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
            this.t.to = value.try_to()?;
            Ok(())
        })
    }

    pub fn set_relative(&mut self, relative_to: Variant) {
        delegate_property_tween!(self => |this| {
            if let Some(origin) = relative_to.try_to().log_if_err() {
                this.t.lerp_mode = LerpMode::Relative {
                    duration: 0.0 , origin,
                };
            }
        })
    }
}
