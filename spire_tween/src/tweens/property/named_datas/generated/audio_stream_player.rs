use super::*;
#[derive(Debug, Clone)]
pub struct AudioStreamPlayerF32Data {
    pub property: AudioStreamPlayerF32Kind,
    pub owner: Gd<AudioStreamPlayer>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioStreamPlayerF32Kind {
    VolumeDb,
    VolumeLinear,
    PitchScale,
}
impl IProperty<f32> for AudioStreamPlayerF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <AudioStreamPlayerF32Kind>::VolumeDb => {
                let obj = &self.owner;
                obj.get_volume_db()
            }
            <AudioStreamPlayerF32Kind>::VolumeLinear => {
                let obj = &self.owner;
                obj.get_volume_linear()
            }
            <AudioStreamPlayerF32Kind>::PitchScale => {
                let obj = &self.owner;
                obj.get_pitch_scale()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <AudioStreamPlayerF32Kind>::VolumeDb => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_volume_db(val);
            }
            <AudioStreamPlayerF32Kind>::VolumeLinear => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_volume_linear(val);
            }
            <AudioStreamPlayerF32Kind>::PitchScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_pitch_scale(val);
            }
        }
    }
}
impl IPropertyData for AudioStreamPlayerF32Data {
    type Target = AudioStreamPlayer;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AudioStreamPlayerF32Kind>::VolumeDb => NodePath::from("volume_db"),
            <AudioStreamPlayerF32Kind>::VolumeLinear => NodePath::from("volume_linear"),
            <AudioStreamPlayerF32Kind>::PitchScale => NodePath::from("pitch_scale"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<AudioStreamPlayer>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<AudioStreamPlayer>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for AudioStreamPlayerF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AudioStreamPlayer>()
            .ok()
            .and_then(|owner| {
                match path {
                    "volume_db" => {
                        Some(Self {
                            property: <AudioStreamPlayerF32Kind>::VolumeDb,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "volume_linear" => {
                        Some(Self {
                            property: <AudioStreamPlayerF32Kind>::VolumeLinear,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "pitch_scale" => {
                        Some(Self {
                            property: <AudioStreamPlayerF32Kind>::PitchScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoAudioStreamPlayer<Marker = ()> {
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
impl<Class: Inherits<AudioStreamPlayer> + Inherits<Object>> DoAudioStreamPlayer<()>
for Gd<Class> {
    fn do_volume_db(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = AudioStreamPlayerF32Data {
            property: <AudioStreamPlayerF32Kind>::VolumeDb,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AudioStreamPlayer>().upcast::<Node>(),
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
        let data = AudioStreamPlayerF32Data {
            property: <AudioStreamPlayerF32Kind>::VolumeLinear,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AudioStreamPlayer>().upcast::<Node>(),
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
        let data = AudioStreamPlayerF32Data {
            property: <AudioStreamPlayerF32Kind>::PitchScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AudioStreamPlayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<AudioStreamPlayer> + Inherits<Object>,
> DoAudioStreamPlayer<BaseMarker> for T {
    fn do_volume_db(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<AudioStreamPlayer> = self.to_gd().upcast();
        let data = AudioStreamPlayerF32Data {
            property: <AudioStreamPlayerF32Kind>::VolumeDb,
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
        let owner: Gd<AudioStreamPlayer> = self.to_gd().upcast();
        let data = AudioStreamPlayerF32Data {
            property: <AudioStreamPlayerF32Kind>::VolumeLinear,
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
        let owner: Gd<AudioStreamPlayer> = self.to_gd().upcast();
        let data = AudioStreamPlayerF32Data {
            property: <AudioStreamPlayerF32Kind>::PitchScale,
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
pub struct AudioStreamPlayerTweener {}
#[godot_api]
impl AudioStreamPlayerTweener {
    #[func]
    fn do_volume_db(
        node: Gd<AudioStreamPlayer>,
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
        node: Gd<AudioStreamPlayer>,
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
        node: Gd<AudioStreamPlayer>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_pitch_scale(to, duration), gd
        }
    }
}
