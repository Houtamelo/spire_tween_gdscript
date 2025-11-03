use super::*;
#[derive(Debug, Clone)]
pub struct CanvasItemIntData {
    pub property: CanvasItemIntKind,
    pub owner: Gd<CanvasItem>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasItemIntKind {
    ZIndex,
}
impl IProperty<i64> for CanvasItemIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <CanvasItemIntKind>::ZIndex => {
                let obj = &self.owner;
                (obj.get_z_index()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <CanvasItemIntKind>::ZIndex => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_z_index(val as i32);
            }
        }
    }
}
impl IPropertyData for CanvasItemIntData {
    type Target = CanvasItem;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CanvasItemIntKind>::ZIndex => NodePath::from("z_index"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<CanvasItem>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<CanvasItem>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for CanvasItemIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CanvasItem>()
            .ok()
            .and_then(|owner| {
                match path {
                    "z_index" => {
                        Some(Self {
                            property: <CanvasItemIntKind>::ZIndex,
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
pub struct CanvasItemFloatData {
    pub property: CanvasItemFloatKind,
    pub owner: Gd<CanvasItem>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasItemFloatKind {
    ModulateR,
    ModulateG,
    ModulateB,
    ModulateA,
    SelfModulateR,
    SelfModulateG,
    SelfModulateB,
    SelfModulateA,
}
impl IProperty<f64> for CanvasItemFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <CanvasItemFloatKind>::ModulateR => {
                let obj = &self.owner;
                (obj.get_modulate().r) as f64
            }
            <CanvasItemFloatKind>::ModulateG => {
                let obj = &self.owner;
                (obj.get_modulate().g) as f64
            }
            <CanvasItemFloatKind>::ModulateB => {
                let obj = &self.owner;
                (obj.get_modulate().b) as f64
            }
            <CanvasItemFloatKind>::ModulateA => {
                let obj = &self.owner;
                (obj.get_modulate().a) as f64
            }
            <CanvasItemFloatKind>::SelfModulateR => {
                let obj = &self.owner;
                (obj.get_self_modulate().r) as f64
            }
            <CanvasItemFloatKind>::SelfModulateG => {
                let obj = &self.owner;
                (obj.get_self_modulate().g) as f64
            }
            <CanvasItemFloatKind>::SelfModulateB => {
                let obj = &self.owner;
                (obj.get_self_modulate().b) as f64
            }
            <CanvasItemFloatKind>::SelfModulateA => {
                let obj = &self.owner;
                (obj.get_self_modulate().a) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <CanvasItemFloatKind>::ModulateR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.r = val as f32;
                obj.set_modulate(prop_val);
            }
            <CanvasItemFloatKind>::ModulateG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.g = val as f32;
                obj.set_modulate(prop_val);
            }
            <CanvasItemFloatKind>::ModulateB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.b = val as f32;
                obj.set_modulate(prop_val);
            }
            <CanvasItemFloatKind>::ModulateA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.a = val as f32;
                obj.set_modulate(prop_val);
            }
            <CanvasItemFloatKind>::SelfModulateR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_self_modulate();
                prop_val.r = val as f32;
                obj.set_self_modulate(prop_val);
            }
            <CanvasItemFloatKind>::SelfModulateG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_self_modulate();
                prop_val.g = val as f32;
                obj.set_self_modulate(prop_val);
            }
            <CanvasItemFloatKind>::SelfModulateB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_self_modulate();
                prop_val.b = val as f32;
                obj.set_self_modulate(prop_val);
            }
            <CanvasItemFloatKind>::SelfModulateA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_self_modulate();
                prop_val.a = val as f32;
                obj.set_self_modulate(prop_val);
            }
        }
    }
}
impl IPropertyData for CanvasItemFloatData {
    type Target = CanvasItem;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CanvasItemFloatKind>::ModulateR => NodePath::from("modulate:r"),
            <CanvasItemFloatKind>::ModulateG => NodePath::from("modulate:g"),
            <CanvasItemFloatKind>::ModulateB => NodePath::from("modulate:b"),
            <CanvasItemFloatKind>::ModulateA => NodePath::from("modulate:a"),
            <CanvasItemFloatKind>::SelfModulateR => NodePath::from("self_modulate:r"),
            <CanvasItemFloatKind>::SelfModulateG => NodePath::from("self_modulate:g"),
            <CanvasItemFloatKind>::SelfModulateB => NodePath::from("self_modulate:b"),
            <CanvasItemFloatKind>::SelfModulateA => NodePath::from("self_modulate:a"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<CanvasItem>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<CanvasItem>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for CanvasItemFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CanvasItem>()
            .ok()
            .and_then(|owner| {
                match path {
                    "modulate:r" => {
                        Some(Self {
                            property: <CanvasItemFloatKind>::ModulateR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:g" => {
                        Some(Self {
                            property: <CanvasItemFloatKind>::ModulateG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:b" => {
                        Some(Self {
                            property: <CanvasItemFloatKind>::ModulateB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:a" => {
                        Some(Self {
                            property: <CanvasItemFloatKind>::ModulateA,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "self_modulate:r" => {
                        Some(Self {
                            property: <CanvasItemFloatKind>::SelfModulateR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "self_modulate:g" => {
                        Some(Self {
                            property: <CanvasItemFloatKind>::SelfModulateG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "self_modulate:b" => {
                        Some(Self {
                            property: <CanvasItemFloatKind>::SelfModulateB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "self_modulate:a" => {
                        Some(Self {
                            property: <CanvasItemFloatKind>::SelfModulateA,
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
pub struct CanvasItemColorData {
    pub property: CanvasItemColorKind,
    pub owner: Gd<CanvasItem>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasItemColorKind {
    Modulate,
    SelfModulate,
}
impl IProperty<Color> for CanvasItemColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <CanvasItemColorKind>::Modulate => {
                let obj = &self.owner;
                obj.get_modulate()
            }
            <CanvasItemColorKind>::SelfModulate => {
                let obj = &self.owner;
                obj.get_self_modulate()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <CanvasItemColorKind>::Modulate => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_modulate(val);
            }
            <CanvasItemColorKind>::SelfModulate => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_self_modulate(val);
            }
        }
    }
}
impl IPropertyData for CanvasItemColorData {
    type Target = CanvasItem;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CanvasItemColorKind>::Modulate => NodePath::from("modulate"),
            <CanvasItemColorKind>::SelfModulate => NodePath::from("self_modulate"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<CanvasItem>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<CanvasItem>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for CanvasItemColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CanvasItem>()
            .ok()
            .and_then(|owner| {
                match path {
                    "modulate" => {
                        Some(Self {
                            property: <CanvasItemColorKind>::Modulate,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "self_modulate" => {
                        Some(Self {
                            property: <CanvasItemColorKind>::SelfModulate,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoCanvasItem<Marker = ()> {
    fn do_z_index(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_modulate_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_modulate_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_modulate_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_modulate_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_self_modulate_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_self_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_self_modulate_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_self_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_self_modulate_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_self_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_self_modulate_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_self_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_modulate(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
    fn do_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
    fn do_self_modulate(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
    fn do_self_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
}
impl<Class: Inherits<CanvasItem> + Inherits<Object>> SpireDoCanvasItem<()>
for Gd<Class> {
    fn do_z_index(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = CanvasItemIntData {
            property: <CanvasItemIntKind>::ZIndex,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_modulate_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_modulate_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_modulate_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_modulate_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = CanvasItemColorData {
            property: <CanvasItemColorKind>::Modulate,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = CanvasItemColorData {
            property: <CanvasItemColorKind>::Modulate,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_modulate(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = CanvasItemColorData {
            property: <CanvasItemColorKind>::SelfModulate,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = CanvasItemColorData {
            property: <CanvasItemColorKind>::SelfModulate,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<CanvasItem> + Inherits<Object>,
> SpireDoCanvasItem<BaseMarker> for T {
    fn do_z_index(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemIntData {
            property: <CanvasItemIntKind>::ZIndex,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::ModulateA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_modulate_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_modulate_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_modulate_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_modulate_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemFloatData {
            property: <CanvasItemFloatKind>::SelfModulateA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemColorData {
            property: <CanvasItemColorKind>::Modulate,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemColorData {
            property: <CanvasItemColorKind>::Modulate,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_modulate(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemColorData {
            property: <CanvasItemColorKind>::SelfModulate,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_self_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemColorData {
            property: <CanvasItemColorKind>::SelfModulate,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
