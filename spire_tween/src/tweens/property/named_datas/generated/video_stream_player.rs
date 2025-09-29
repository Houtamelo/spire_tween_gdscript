use super::*;
#[derive(Debug, Clone)]
pub struct VideoStreamPlayerF32Data {
    pub property: VideoStreamPlayerF32Kind,
    pub owner: Gd<VideoStreamPlayer>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoStreamPlayerF32Kind {
    VolumeDb,
    Volume,
}
impl IProperty<f32> for VideoStreamPlayerF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <VideoStreamPlayerF32Kind>::VolumeDb => {
                let obj = &self.owner;
                obj.get_volume_db()
            }
            <VideoStreamPlayerF32Kind>::Volume => {
                let obj = &self.owner;
                obj.get_volume()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <VideoStreamPlayerF32Kind>::VolumeDb => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_volume_db(val);
            }
            <VideoStreamPlayerF32Kind>::Volume => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_volume(val);
            }
        }
    }
}
impl IPropertyData for VideoStreamPlayerF32Data {
    type Target = VideoStreamPlayer;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <VideoStreamPlayerF32Kind>::VolumeDb => NodePath::from("volume_db"),
            <VideoStreamPlayerF32Kind>::Volume => NodePath::from("volume"),
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
impl TryFromPathAndObject for VideoStreamPlayerF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<VideoStreamPlayer>()
            .ok()
            .and_then(|owner| {
                match path {
                    "volume_db" => {
                        Some(Self {
                            property: <VideoStreamPlayerF32Kind>::VolumeDb,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "volume" => {
                        Some(Self {
                            property: <VideoStreamPlayerF32Kind>::Volume,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoVideoStreamPlayer<Marker = ()> {
    fn video_do_volume_db(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn video_do_volume(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
}
impl<Class: Inherits<VideoStreamPlayer> + Inherits<Object>> DoVideoStreamPlayer<()>
for Gd<Class> {
    fn video_do_volume_db(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = VideoStreamPlayerF32Data {
            property: <VideoStreamPlayerF32Kind>::VolumeDb,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<VideoStreamPlayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn video_do_volume(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = VideoStreamPlayerF32Data {
            property: <VideoStreamPlayerF32Kind>::Volume,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<VideoStreamPlayer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<VideoStreamPlayer> + Inherits<Object>,
> DoVideoStreamPlayer<BaseMarker> for T {
    fn video_do_volume_db(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<VideoStreamPlayer> = self.to_gd().upcast();
        let data = VideoStreamPlayerF32Data {
            property: <VideoStreamPlayerF32Kind>::VolumeDb,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn video_do_volume(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<VideoStreamPlayer> = self.to_gd().upcast();
        let data = VideoStreamPlayerF32Data {
            property: <VideoStreamPlayerF32Kind>::Volume,
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
pub struct VideoStreamPlayerTweener {}
#[godot_api]
impl VideoStreamPlayerTweener {
    #[func]
    fn video_do_volume_db(
        node: Gd<VideoStreamPlayer>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.video_do_volume_db(to, duration), gd
        }
    }
    #[func]
    fn video_do_volume(
        node: Gd<VideoStreamPlayer>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.video_do_volume(to, duration), gd
        }
    }
}
