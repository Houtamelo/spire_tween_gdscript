use super::*;
#[derive(Debug, Clone)]
pub struct VideoStreamPlayerFloatData {
    pub property: VideoStreamPlayerFloatKind,
    pub owner: Gd<VideoStreamPlayer>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoStreamPlayerFloatKind {
    VolumeDb,
    Volume,
}
impl IProperty<f64> for VideoStreamPlayerFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <VideoStreamPlayerFloatKind>::VolumeDb => {
                let obj = &self.owner;
                (obj.get_volume_db()) as f64
            }
            <VideoStreamPlayerFloatKind>::Volume => {
                let obj = &self.owner;
                (obj.get_volume()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <VideoStreamPlayerFloatKind>::VolumeDb => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_volume_db(val as f32);
            }
            <VideoStreamPlayerFloatKind>::Volume => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_volume(val as f32);
            }
        }
    }
}
impl IPropertyData for VideoStreamPlayerFloatData {
    type Target = VideoStreamPlayer;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <VideoStreamPlayerFloatKind>::VolumeDb => NodePath::from("volume_db"),
            <VideoStreamPlayerFloatKind>::Volume => NodePath::from("volume"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<VideoStreamPlayer>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<VideoStreamPlayer>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for VideoStreamPlayerFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<VideoStreamPlayer>()
            .ok()
            .and_then(|owner| {
                match path {
                    "volume_db" => {
                        Some(Self {
                            property: <VideoStreamPlayerFloatKind>::VolumeDb,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "volume" => {
                        Some(Self {
                            property: <VideoStreamPlayerFloatKind>::Volume,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoVideoStreamPlayer<Marker = ()> {
    fn do_volume_db(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_volume(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<Class: Inherits<VideoStreamPlayer> + Inherits<Object>> SpireDoVideoStreamPlayer<()>
for Gd<Class> {
    fn do_volume_db(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = VideoStreamPlayerFloatData {
            property: <VideoStreamPlayerFloatKind>::VolumeDb,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<VideoStreamPlayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_volume(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = VideoStreamPlayerFloatData {
            property: <VideoStreamPlayerFloatKind>::Volume,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<VideoStreamPlayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<VideoStreamPlayer> + Inherits<Object>,
> SpireDoVideoStreamPlayer<BaseMarker> for T {
    fn do_volume_db(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<VideoStreamPlayer> = self.to_gd().upcast();
        let data = VideoStreamPlayerFloatData {
            property: <VideoStreamPlayerFloatKind>::VolumeDb,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_volume(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<VideoStreamPlayer> = self.to_gd().upcast();
        let data = VideoStreamPlayerFloatData {
            property: <VideoStreamPlayerFloatKind>::Volume,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
