use super::*;
#[derive(Debug, Clone)]
pub struct AudioStreamPlayer3DFloatData {
    pub property: AudioStreamPlayer3DFloatKind,
    pub owner: Gd<AudioStreamPlayer3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioStreamPlayer3DFloatKind {
    VolumeDb,
    VolumeLinear,
    PitchScale,
}
impl IProperty<f64> for AudioStreamPlayer3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <AudioStreamPlayer3DFloatKind>::VolumeDb => {
                let obj = &self.owner;
                (obj.get_volume_db()) as f64
            }
            <AudioStreamPlayer3DFloatKind>::VolumeLinear => {
                let obj = &self.owner;
                (obj.get_volume_linear()) as f64
            }
            <AudioStreamPlayer3DFloatKind>::PitchScale => {
                let obj = &self.owner;
                (obj.get_pitch_scale()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <AudioStreamPlayer3DFloatKind>::VolumeDb => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_volume_db(val as f32);
            }
            <AudioStreamPlayer3DFloatKind>::VolumeLinear => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_volume_linear(val as f32);
            }
            <AudioStreamPlayer3DFloatKind>::PitchScale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_pitch_scale(val as f32);
            }
        }
    }
}
impl IPropertyData for AudioStreamPlayer3DFloatData {
    type Target = AudioStreamPlayer3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AudioStreamPlayer3DFloatKind>::VolumeDb => NodePath::from("volume_db"),
            <AudioStreamPlayer3DFloatKind>::VolumeLinear => {
                NodePath::from("volume_linear")
            }
            <AudioStreamPlayer3DFloatKind>::PitchScale => NodePath::from("pitch_scale"),
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
impl TryFromPathAndObject for AudioStreamPlayer3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AudioStreamPlayer3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "volume_db" => {
                        Some(Self {
                            property: <AudioStreamPlayer3DFloatKind>::VolumeDb,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "volume_linear" => {
                        Some(Self {
                            property: <AudioStreamPlayer3DFloatKind>::VolumeLinear,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "pitch_scale" => {
                        Some(Self {
                            property: <AudioStreamPlayer3DFloatKind>::PitchScale,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoAudioStreamPlayer3D<Marker = ()> {
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
impl<
    Class: Inherits<AudioStreamPlayer3D> + Inherits<Object>,
> SpireDoAudioStreamPlayer3D<()> for Gd<Class> {
    fn do_volume_db(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = AudioStreamPlayer3DFloatData {
            property: <AudioStreamPlayer3DFloatKind>::VolumeDb,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AudioStreamPlayer3D>().upcast::<Node>(),
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
        let data = AudioStreamPlayer3DFloatData {
            property: <AudioStreamPlayer3DFloatKind>::VolumeLinear,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AudioStreamPlayer3D>().upcast::<Node>(),
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
        let data = AudioStreamPlayer3DFloatData {
            property: <AudioStreamPlayer3DFloatKind>::PitchScale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AudioStreamPlayer3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<AudioStreamPlayer3D> + Inherits<Object>,
> SpireDoAudioStreamPlayer3D<BaseMarker> for T {
    fn do_volume_db(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<AudioStreamPlayer3D> = self.to_gd().upcast();
        let data = AudioStreamPlayer3DFloatData {
            property: <AudioStreamPlayer3DFloatKind>::VolumeDb,
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
        let owner: Gd<AudioStreamPlayer3D> = self.to_gd().upcast();
        let data = AudioStreamPlayer3DFloatData {
            property: <AudioStreamPlayer3DFloatKind>::VolumeLinear,
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
        let owner: Gd<AudioStreamPlayer3D> = self.to_gd().upcast();
        let data = AudioStreamPlayer3DFloatData {
            property: <AudioStreamPlayer3DFloatKind>::PitchScale,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
