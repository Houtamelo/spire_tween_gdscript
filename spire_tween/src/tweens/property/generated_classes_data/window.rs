use super::*;
#[derive(Debug, Clone)]
pub struct WindowIntData {
    pub property: WindowIntKind,
    pub owner: Gd<Window>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowIntKind {
    PositionX,
    PositionY,
    SizeX,
    SizeY,
}
impl IProperty<i64> for WindowIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <WindowIntKind>::PositionX => {
                let obj = &self.owner;
                (obj.get_position().x) as i64
            }
            <WindowIntKind>::PositionY => {
                let obj = &self.owner;
                (obj.get_position().y) as i64
            }
            <WindowIntKind>::SizeX => {
                let obj = &self.owner;
                (obj.get_size().x) as i64
            }
            <WindowIntKind>::SizeY => {
                let obj = &self.owner;
                (obj.get_size().y) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <WindowIntKind>::PositionX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_position();
                prop_val.x = val as i32;
                obj.set_position(prop_val);
            }
            <WindowIntKind>::PositionY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_position();
                prop_val.y = val as i32;
                obj.set_position(prop_val);
            }
            <WindowIntKind>::SizeX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.x = val as i32;
                obj.set_size(prop_val);
            }
            <WindowIntKind>::SizeY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.y = val as i32;
                obj.set_size(prop_val);
            }
        }
    }
}
impl IPropertyData for WindowIntData {
    type Target = Window;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <WindowIntKind>::PositionX => NodePath::from("position:x"),
            <WindowIntKind>::PositionY => NodePath::from("position:y"),
            <WindowIntKind>::SizeX => NodePath::from("size:x"),
            <WindowIntKind>::SizeY => NodePath::from("size:y"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Window>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Window>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for WindowIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Window>()
            .ok()
            .and_then(|owner| {
                match path {
                    "position:x" => {
                        Some(Self {
                            property: <WindowIntKind>::PositionX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "position:y" => {
                        Some(Self {
                            property: <WindowIntKind>::PositionY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:x" => {
                        Some(Self {
                            property: <WindowIntKind>::SizeX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:y" => {
                        Some(Self {
                            property: <WindowIntKind>::SizeY,
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
pub struct WindowVector2iData {
    pub property: WindowVector2iKind,
    pub owner: Gd<Window>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowVector2iKind {
    Position,
    Size,
}
impl IProperty<Vector2i> for WindowVector2iData {
    #[inline]
    fn get_property_value(&self) -> Vector2i {
        match self.property {
            <WindowVector2iKind>::Position => {
                let obj = &self.owner;
                obj.get_position()
            }
            <WindowVector2iKind>::Size => {
                let obj = &self.owner;
                obj.get_size()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2i) {
        match self.property {
            <WindowVector2iKind>::Position => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_position(val);
            }
            <WindowVector2iKind>::Size => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_size(val);
            }
        }
    }
}
impl IPropertyData for WindowVector2iData {
    type Target = Window;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <WindowVector2iKind>::Position => NodePath::from("position"),
            <WindowVector2iKind>::Size => NodePath::from("size"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Window>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Window>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for WindowVector2iData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Window>()
            .ok()
            .and_then(|owner| {
                match path {
                    "position" => {
                        Some(Self {
                            property: <WindowVector2iKind>::Position,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size" => {
                        Some(Self {
                            property: <WindowVector2iKind>::Size,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoWindow<Marker = ()> {
    fn do_position_x(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_position_y(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_size_x(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_size_y(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_position(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>>;
    fn do_size(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>>;
}
impl<Class: Inherits<Window> + Inherits<Object>> SpireDoWindow<()> for Gd<Class> {
    fn do_position_x(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = WindowIntData {
            property: <WindowIntKind>::PositionX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Window>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position_y(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = WindowIntData {
            property: <WindowIntKind>::PositionY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Window>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_x(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = WindowIntData {
            property: <WindowIntKind>::SizeX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Window>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_y(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = WindowIntData {
            property: <WindowIntKind>::SizeY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Window>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>> {
        let data = WindowVector2iData {
            property: <WindowVector2iKind>::Position,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Window>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2i>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>> {
        let data = WindowVector2iData {
            property: <WindowVector2iKind>::Size,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Window>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2i>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<T: WithBaseField + Inherits<Window> + Inherits<Object>> SpireDoWindow<BaseMarker>
for T {
    fn do_position_x(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<Window> = self.to_gd().upcast();
        let data = WindowIntData {
            property: <WindowIntKind>::PositionX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position_y(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<Window> = self.to_gd().upcast();
        let data = WindowIntData {
            property: <WindowIntKind>::PositionY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_x(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<Window> = self.to_gd().upcast();
        let data = WindowIntData {
            property: <WindowIntKind>::SizeX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_y(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<Window> = self.to_gd().upcast();
        let data = WindowIntData {
            property: <WindowIntKind>::SizeY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_position(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>> {
        let owner: Gd<Window> = self.to_gd().upcast();
        let data = WindowVector2iData {
            property: <WindowVector2iKind>::Position,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2i>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>> {
        let owner: Gd<Window> = self.to_gd().upcast();
        let data = WindowVector2iData {
            property: <WindowVector2iKind>::Size,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2i>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
