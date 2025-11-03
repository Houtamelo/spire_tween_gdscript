use super::*;
#[derive(Debug, Clone)]
pub struct PathFollow2DFloatData {
    pub property: PathFollow2DFloatKind,
    pub owner: Gd<PathFollow2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathFollow2DFloatKind {
    HOffset,
    VOffset,
    Progress,
    ProgressRatio,
}
impl IProperty<f64> for PathFollow2DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <PathFollow2DFloatKind>::HOffset => {
                let obj = &self.owner;
                (obj.get_h_offset()) as f64
            }
            <PathFollow2DFloatKind>::VOffset => {
                let obj = &self.owner;
                (obj.get_v_offset()) as f64
            }
            <PathFollow2DFloatKind>::Progress => {
                let obj = &self.owner;
                (obj.get_progress()) as f64
            }
            <PathFollow2DFloatKind>::ProgressRatio => {
                let obj = &self.owner;
                (obj.get_progress_ratio()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <PathFollow2DFloatKind>::HOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_h_offset(val as f32);
            }
            <PathFollow2DFloatKind>::VOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_v_offset(val as f32);
            }
            <PathFollow2DFloatKind>::Progress => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_progress(val as f32);
            }
            <PathFollow2DFloatKind>::ProgressRatio => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_progress_ratio(val as f32);
            }
        }
    }
}
impl IPropertyData for PathFollow2DFloatData {
    type Target = PathFollow2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <PathFollow2DFloatKind>::HOffset => NodePath::from("h_offset"),
            <PathFollow2DFloatKind>::VOffset => NodePath::from("v_offset"),
            <PathFollow2DFloatKind>::Progress => NodePath::from("progress"),
            <PathFollow2DFloatKind>::ProgressRatio => NodePath::from("progress_ratio"),
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
impl TryFromPathAndObject for PathFollow2DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<PathFollow2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "h_offset" => {
                        Some(Self {
                            property: <PathFollow2DFloatKind>::HOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "v_offset" => {
                        Some(Self {
                            property: <PathFollow2DFloatKind>::VOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "progress" => {
                        Some(Self {
                            property: <PathFollow2DFloatKind>::Progress,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "progress_ratio" => {
                        Some(Self {
                            property: <PathFollow2DFloatKind>::ProgressRatio,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoPathFollow2D<Marker = ()> {
    fn do_h_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_v_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_progress(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_progress_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<Class: Inherits<PathFollow2D> + Inherits<Object>> SpireDoPathFollow2D<()>
for Gd<Class> {
    fn do_h_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PathFollow2DFloatData {
            property: <PathFollow2DFloatKind>::HOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PathFollow2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_v_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PathFollow2DFloatData {
            property: <PathFollow2DFloatKind>::VOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PathFollow2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_progress(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PathFollow2DFloatData {
            property: <PathFollow2DFloatKind>::Progress,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PathFollow2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_progress_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = PathFollow2DFloatData {
            property: <PathFollow2DFloatKind>::ProgressRatio,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<PathFollow2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<PathFollow2D> + Inherits<Object>,
> SpireDoPathFollow2D<BaseMarker> for T {
    fn do_h_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PathFollow2D> = self.to_gd().upcast();
        let data = PathFollow2DFloatData {
            property: <PathFollow2DFloatKind>::HOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_v_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PathFollow2D> = self.to_gd().upcast();
        let data = PathFollow2DFloatData {
            property: <PathFollow2DFloatKind>::VOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_progress(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PathFollow2D> = self.to_gd().upcast();
        let data = PathFollow2DFloatData {
            property: <PathFollow2DFloatKind>::Progress,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_progress_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<PathFollow2D> = self.to_gd().upcast();
        let data = PathFollow2DFloatData {
            property: <PathFollow2DFloatKind>::ProgressRatio,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
