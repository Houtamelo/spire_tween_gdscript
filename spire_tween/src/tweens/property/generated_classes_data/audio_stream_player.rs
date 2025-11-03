use super::*;
#[derive(Debug, Clone)]
pub struct AudioStreamPlayerFloatData {
    pub property: AudioStreamPlayerFloatKind,
    pub owner: Gd<AudioStreamPlayer>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioStreamPlayerFloatKind {
    VolumeDb,
    VolumeLinear,
    PitchScale,
}
impl IProperty<f64> for AudioStreamPlayerFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <AudioStreamPlayerFloatKind>::VolumeDb => {
                let obj = &self.owner;
                (obj.get_volume_db()) as f64
            }
            <AudioStreamPlayerFloatKind>::VolumeLinear => {
                let obj = &self.owner;
                (obj.get_volume_linear()) as f64
            }
            <AudioStreamPlayerFloatKind>::PitchScale => {
                let obj = &self.owner;
                (obj.get_pitch_scale()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <AudioStreamPlayerFloatKind>::VolumeDb => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_volume_db(val as f32);
            }
            <AudioStreamPlayerFloatKind>::VolumeLinear => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_volume_linear(val as f32);
            }
            <AudioStreamPlayerFloatKind>::PitchScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_pitch_scale(val as f32);
            }
        }
    }
}
impl IPropertyData for AudioStreamPlayerFloatData {
    type Target = AudioStreamPlayer;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AudioStreamPlayerFloatKind>::VolumeDb => NodePath::from("volume_db"),
            <AudioStreamPlayerFloatKind>::VolumeLinear => NodePath::from("volume_linear"),
            <AudioStreamPlayerFloatKind>::PitchScale => NodePath::from("pitch_scale"),
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
impl TryFromPathAndObject for AudioStreamPlayerFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AudioStreamPlayer>()
            .ok()
            .and_then(|owner| {
                match path {
                    "volume_db" => {
                        Some(Self {
                            property: <AudioStreamPlayerFloatKind>::VolumeDb,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "volume_linear" => {
                        Some(Self {
                            property: <AudioStreamPlayerFloatKind>::VolumeLinear,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "pitch_scale" => {
                        Some(Self {
                            property: <AudioStreamPlayerFloatKind>::PitchScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoAudioStreamPlayer<Marker = ()> {
    fn do_volume_db(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_volume_linear(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_pitch_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<Class: Inherits<AudioStreamPlayer> + Inherits<Object>> SpireDoAudioStreamPlayer<()>
for Gd<Class> {
    fn do_volume_db(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = AudioStreamPlayerFloatData {
            property: <AudioStreamPlayerFloatKind>::VolumeDb,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AudioStreamPlayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_volume_linear(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = AudioStreamPlayerFloatData {
            property: <AudioStreamPlayerFloatKind>::VolumeLinear,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AudioStreamPlayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_pitch_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = AudioStreamPlayerFloatData {
            property: <AudioStreamPlayerFloatKind>::PitchScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AudioStreamPlayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<AudioStreamPlayer> + Inherits<Object>,
> SpireDoAudioStreamPlayer<BaseMarker> for T {
    fn do_volume_db(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<AudioStreamPlayer> = self.to_gd().upcast();
        let data = AudioStreamPlayerFloatData {
            property: <AudioStreamPlayerFloatKind>::VolumeDb,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_volume_linear(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<AudioStreamPlayer> = self.to_gd().upcast();
        let data = AudioStreamPlayerFloatData {
            property: <AudioStreamPlayerFloatKind>::VolumeLinear,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_pitch_scale(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<AudioStreamPlayer> = self.to_gd().upcast();
        let data = AudioStreamPlayerFloatData {
            property: <AudioStreamPlayerFloatKind>::PitchScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
