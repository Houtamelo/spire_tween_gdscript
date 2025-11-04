use super::*;
#[derive(Debug, Clone)]
pub struct SubViewportIntData {
    pub property: SubViewportIntKind,
    pub owner: Gd<SubViewport>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubViewportIntKind {
    SizeX,
    SizeY,
    Size2DOverrideX,
    Size2DOverrideY,
}
impl IProperty<i64> for SubViewportIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <SubViewportIntKind>::SizeX => {
                let obj = &self.owner;
                (obj.get_size().x) as i64
            }
            <SubViewportIntKind>::SizeY => {
                let obj = &self.owner;
                (obj.get_size().y) as i64
            }
            <SubViewportIntKind>::Size2DOverrideX => {
                let obj = &self.owner;
                (obj.get_size_2d_override().x) as i64
            }
            <SubViewportIntKind>::Size2DOverrideY => {
                let obj = &self.owner;
                (obj.get_size_2d_override().y) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <SubViewportIntKind>::SizeX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.x = val as i32;
                obj.set_size(prop_val);
            }
            <SubViewportIntKind>::SizeY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.y = val as i32;
                obj.set_size(prop_val);
            }
            <SubViewportIntKind>::Size2DOverrideX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size_2d_override();
                prop_val.x = val as i32;
                obj.set_size_2d_override(prop_val);
            }
            <SubViewportIntKind>::Size2DOverrideY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size_2d_override();
                prop_val.y = val as i32;
                obj.set_size_2d_override(prop_val);
            }
        }
    }
}
impl IPropertyData for SubViewportIntData {
    type Target = SubViewport;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <SubViewportIntKind>::SizeX => NodePath::from("size:x"),
            <SubViewportIntKind>::SizeY => NodePath::from("size:y"),
            <SubViewportIntKind>::Size2DOverrideX => NodePath::from("size_2d_override:x"),
            <SubViewportIntKind>::Size2DOverrideY => NodePath::from("size_2d_override:y"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<SubViewport>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<SubViewport>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for SubViewportIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<SubViewport>()
            .ok()
            .and_then(|owner| {
                match path {
                    "size:x" => {
                        Some(Self {
                            property: <SubViewportIntKind>::SizeX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:y" => {
                        Some(Self {
                            property: <SubViewportIntKind>::SizeY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size_2d_override:x" => {
                        Some(Self {
                            property: <SubViewportIntKind>::Size2DOverrideX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size_2d_override:y" => {
                        Some(Self {
                            property: <SubViewportIntKind>::Size2DOverrideY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
#[derive(Debug, Clone)]
pub struct SubViewportVector2iData {
    pub property: SubViewportVector2iKind,
    pub owner: Gd<SubViewport>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubViewportVector2iKind {
    Size,
    Size2DOverride,
}
impl IProperty<Vector2i> for SubViewportVector2iData {
    #[inline]
    fn get_property_value(&self) -> Vector2i {
        match self.property {
            <SubViewportVector2iKind>::Size => {
                let obj = &self.owner;
                obj.get_size()
            }
            <SubViewportVector2iKind>::Size2DOverride => {
                let obj = &self.owner;
                obj.get_size_2d_override()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2i) {
        match self.property {
            <SubViewportVector2iKind>::Size => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_size(val);
            }
            <SubViewportVector2iKind>::Size2DOverride => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_size_2d_override(val);
            }
        }
    }
}
impl IPropertyData for SubViewportVector2iData {
    type Target = SubViewport;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <SubViewportVector2iKind>::Size => NodePath::from("size"),
            <SubViewportVector2iKind>::Size2DOverride => {
                NodePath::from("size_2d_override")
            }
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<SubViewport>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<SubViewport>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for SubViewportVector2iData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<SubViewport>()
            .ok()
            .and_then(|owner| {
                match path {
                    "size" => {
                        Some(Self {
                            property: <SubViewportVector2iKind>::Size,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size_2d_override" => {
                        Some(Self {
                            property: <SubViewportVector2iKind>::Size2DOverride,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoSubViewport<Marker = ()> {
    fn do_subview_do_size_x(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_subview_do_size_y(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_subview_do_size_2d_override_x(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_subview_do_size_2d_override_y(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_subview_do_size(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>>;
    fn do_subview_do_size_2d_override(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>>;
}
impl<Class: Inherits<SubViewport> + Inherits<Object>> SpireDoSubViewport<()>
for Gd<Class> {
    fn do_subview_do_size_x(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = SubViewportIntData {
            property: <SubViewportIntKind>::SizeX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SubViewport>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_subview_do_size_y(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = SubViewportIntData {
            property: <SubViewportIntKind>::SizeY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SubViewport>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_subview_do_size_2d_override_x(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = SubViewportIntData {
            property: <SubViewportIntKind>::Size2DOverrideX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SubViewport>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_subview_do_size_2d_override_y(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = SubViewportIntData {
            property: <SubViewportIntKind>::Size2DOverrideY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SubViewport>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_subview_do_size(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>> {
        let data = SubViewportVector2iData {
            property: <SubViewportVector2iKind>::Size,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SubViewport>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2i>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_subview_do_size_2d_override(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>> {
        let data = SubViewportVector2iData {
            property: <SubViewportVector2iKind>::Size2DOverride,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SubViewport>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2i>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<SubViewport> + Inherits<Object>,
> SpireDoSubViewport<BaseMarker> for T {
    fn do_subview_do_size_x(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<SubViewport> = self.to_gd().upcast();
        let data = SubViewportIntData {
            property: <SubViewportIntKind>::SizeX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_subview_do_size_y(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<SubViewport> = self.to_gd().upcast();
        let data = SubViewportIntData {
            property: <SubViewportIntKind>::SizeY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_subview_do_size_2d_override_x(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<SubViewport> = self.to_gd().upcast();
        let data = SubViewportIntData {
            property: <SubViewportIntKind>::Size2DOverrideX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_subview_do_size_2d_override_y(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<SubViewport> = self.to_gd().upcast();
        let data = SubViewportIntData {
            property: <SubViewportIntKind>::Size2DOverrideY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_subview_do_size(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>> {
        let owner: Gd<SubViewport> = self.to_gd().upcast();
        let data = SubViewportVector2iData {
            property: <SubViewportVector2iKind>::Size,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2i>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_subview_do_size_2d_override(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>> {
        let owner: Gd<SubViewport> = self.to_gd().upcast();
        let data = SubViewportVector2iData {
            property: <SubViewportVector2iKind>::Size2DOverride,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2i>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
