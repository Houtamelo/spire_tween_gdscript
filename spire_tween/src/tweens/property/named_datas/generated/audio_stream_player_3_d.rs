use super::*;
#[derive(Debug, Clone)]
pub struct AudioStreamPlayer3DF32Data {
    pub property: AudioStreamPlayer3DF32Kind,
    pub owner: Gd<AudioStreamPlayer3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioStreamPlayer3DF32Kind {
    VolumeDb,
    VolumeLinear,
    PitchScale,
}
impl IProperty<f32> for AudioStreamPlayer3DF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <AudioStreamPlayer3DF32Kind>::VolumeDb => {
                let obj = &self.owner;
                obj.get_volume_db()
            }
            <AudioStreamPlayer3DF32Kind>::VolumeLinear => {
                let obj = &self.owner;
                obj.get_volume_linear()
            }
            <AudioStreamPlayer3DF32Kind>::PitchScale => {
                let obj = &self.owner;
                obj.get_pitch_scale()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <AudioStreamPlayer3DF32Kind>::VolumeDb => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_volume_db(val);
            }
            <AudioStreamPlayer3DF32Kind>::VolumeLinear => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_volume_linear(val);
            }
            <AudioStreamPlayer3DF32Kind>::PitchScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_pitch_scale(val);
            }
        }
    }
}
impl IPropertyData for AudioStreamPlayer3DF32Data {
    type Target = AudioStreamPlayer3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AudioStreamPlayer3DF32Kind>::VolumeDb => NodePath::from("volume_db"),
            <AudioStreamPlayer3DF32Kind>::VolumeLinear => NodePath::from("volume_linear"),
            <AudioStreamPlayer3DF32Kind>::PitchScale => NodePath::from("pitch_scale"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<AudioStreamPlayer3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<AudioStreamPlayer3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for AudioStreamPlayer3DF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AudioStreamPlayer3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "volume_db" => {
                        Some(Self {
                            property: <AudioStreamPlayer3DF32Kind>::VolumeDb,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "volume_linear" => {
                        Some(Self {
                            property: <AudioStreamPlayer3DF32Kind>::VolumeLinear,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "pitch_scale" => {
                        Some(Self {
                            property: <AudioStreamPlayer3DF32Kind>::PitchScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoAudioStreamPlayer3D<Marker = ()> {
    fn do_volume_db(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_volume_linear(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_pitch_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
}
impl<Class: Inherits<AudioStreamPlayer3D> + Inherits<Object>> DoAudioStreamPlayer3D<()>
for Gd<Class> {
    fn do_volume_db(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = AudioStreamPlayer3DF32Data {
            property: <AudioStreamPlayer3DF32Kind>::VolumeDb,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AudioStreamPlayer3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_volume_linear(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = AudioStreamPlayer3DF32Data {
            property: <AudioStreamPlayer3DF32Kind>::VolumeLinear,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AudioStreamPlayer3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_pitch_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = AudioStreamPlayer3DF32Data {
            property: <AudioStreamPlayer3DF32Kind>::PitchScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AudioStreamPlayer3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<AudioStreamPlayer3D> + Inherits<Object>,
> DoAudioStreamPlayer3D<BaseMarker> for T {
    fn do_volume_db(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<AudioStreamPlayer3D> = self.to_gd().upcast();
        let data = AudioStreamPlayer3DF32Data {
            property: <AudioStreamPlayer3DF32Kind>::VolumeDb,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_volume_linear(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<AudioStreamPlayer3D> = self.to_gd().upcast();
        let data = AudioStreamPlayer3DF32Data {
            property: <AudioStreamPlayer3DF32Kind>::VolumeLinear,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_pitch_scale(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<AudioStreamPlayer3D> = self.to_gd().upcast();
        let data = AudioStreamPlayer3DF32Data {
            property: <AudioStreamPlayer3DF32Kind>::PitchScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct AudioStreamPlayer3DTweener {}
#[godot_api]
impl AudioStreamPlayer3DTweener {
    #[func]
    fn do_volume_db(
        node: Gd<AudioStreamPlayer3D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_volume_db(to, duration), gd
        }
    }
    #[func]
    fn do_volume_linear(
        node: Gd<AudioStreamPlayer3D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_volume_linear(to, duration), gd
        }
    }
    #[func]
    fn do_pitch_scale(
        node: Gd<AudioStreamPlayer3D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_pitch_scale(to, duration), gd
        }
    }
}
