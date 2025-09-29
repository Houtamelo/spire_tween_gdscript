use super::*;
#[derive(Debug, Clone)]
pub struct PathFollow2DF32Data {
    pub property: PathFollow2DF32Kind,
    pub owner: Gd<PathFollow2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathFollow2DF32Kind {
    HOffset,
    VOffset,
    Progress,
    ProgressRatio,
}
impl IProperty<f32> for PathFollow2DF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <PathFollow2DF32Kind>::HOffset => {
                let obj = &self.owner;
                obj.get_h_offset()
            }
            <PathFollow2DF32Kind>::VOffset => {
                let obj = &self.owner;
                obj.get_v_offset()
            }
            <PathFollow2DF32Kind>::Progress => {
                let obj = &self.owner;
                obj.get_progress()
            }
            <PathFollow2DF32Kind>::ProgressRatio => {
                let obj = &self.owner;
                obj.get_progress_ratio()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <PathFollow2DF32Kind>::HOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_h_offset(val);
            }
            <PathFollow2DF32Kind>::VOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_v_offset(val);
            }
            <PathFollow2DF32Kind>::Progress => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_progress(val);
            }
            <PathFollow2DF32Kind>::ProgressRatio => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_progress_ratio(val);
            }
        }
    }
}
impl IPropertyData for PathFollow2DF32Data {
    type Target = PathFollow2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <PathFollow2DF32Kind>::HOffset => NodePath::from("h_offset"),
            <PathFollow2DF32Kind>::VOffset => NodePath::from("v_offset"),
            <PathFollow2DF32Kind>::Progress => NodePath::from("progress"),
            <PathFollow2DF32Kind>::ProgressRatio => NodePath::from("progress_ratio"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<PathFollow2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<PathFollow2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for PathFollow2DF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<PathFollow2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "h_offset" => {
                        Some(Self {
                            property: <PathFollow2DF32Kind>::HOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "v_offset" => {
                        Some(Self {
                            property: <PathFollow2DF32Kind>::VOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "progress" => {
                        Some(Self {
                            property: <PathFollow2DF32Kind>::Progress,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "progress_ratio" => {
                        Some(Self {
                            property: <PathFollow2DF32Kind>::ProgressRatio,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoPathFollow2D<Marker = ()> {
    fn do_h_offset(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_v_offset(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_progress(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_progress_ratio(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
}
impl<Class: Inherits<PathFollow2D> + Inherits<Object>> DoPathFollow2D<()> for Gd<Class> {
    fn do_h_offset(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = PathFollow2DF32Data {
            property: <PathFollow2DF32Kind>::HOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PathFollow2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_v_offset(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = PathFollow2DF32Data {
            property: <PathFollow2DF32Kind>::VOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PathFollow2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_progress(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = PathFollow2DF32Data {
            property: <PathFollow2DF32Kind>::Progress,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PathFollow2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_progress_ratio(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = PathFollow2DF32Data {
            property: <PathFollow2DF32Kind>::ProgressRatio,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PathFollow2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<PathFollow2D> + Inherits<Object>,
> DoPathFollow2D<BaseMarker> for T {
    fn do_h_offset(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<PathFollow2D> = self.to_gd().upcast();
        let data = PathFollow2DF32Data {
            property: <PathFollow2DF32Kind>::HOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_v_offset(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<PathFollow2D> = self.to_gd().upcast();
        let data = PathFollow2DF32Data {
            property: <PathFollow2DF32Kind>::VOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_progress(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<PathFollow2D> = self.to_gd().upcast();
        let data = PathFollow2DF32Data {
            property: <PathFollow2DF32Kind>::Progress,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_progress_ratio(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<PathFollow2D> = self.to_gd().upcast();
        let data = PathFollow2DF32Data {
            property: <PathFollow2DF32Kind>::ProgressRatio,
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
pub struct PathFollow2DTweener {}
#[godot_api]
impl PathFollow2DTweener {
    #[func]
    fn do_h_offset(
        node: Gd<PathFollow2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_h_offset(to, duration), gd
        }
    }
    #[func]
    fn do_v_offset(
        node: Gd<PathFollow2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_v_offset(to, duration), gd
        }
    }
    #[func]
    fn do_progress(
        node: Gd<PathFollow2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_progress(to, duration), gd
        }
    }
    #[func]
    fn do_progress_ratio(
        node: Gd<PathFollow2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_progress_ratio(to, duration), gd
        }
    }
}
