use super::*;
#[derive(Debug, Clone)]
pub struct CanvasItemF32Data {
    pub property: CanvasItemF32Kind,
    pub owner: Gd<CanvasItem>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasItemF32Kind {
    ModulateR,
    ModulateG,
    ModulateB,
    ModulateA,
    SelfModulateR,
    SelfModulateG,
    SelfModulateB,
    SelfModulateA,
}
impl IProperty<f32> for CanvasItemF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <CanvasItemF32Kind>::ModulateR => {
                let obj = &self.owner;
                obj.get_modulate().r
            }
            <CanvasItemF32Kind>::ModulateG => {
                let obj = &self.owner;
                obj.get_modulate().g
            }
            <CanvasItemF32Kind>::ModulateB => {
                let obj = &self.owner;
                obj.get_modulate().b
            }
            <CanvasItemF32Kind>::ModulateA => {
                let obj = &self.owner;
                obj.get_modulate().a
            }
            <CanvasItemF32Kind>::SelfModulateR => {
                let obj = &self.owner;
                obj.get_self_modulate().r
            }
            <CanvasItemF32Kind>::SelfModulateG => {
                let obj = &self.owner;
                obj.get_self_modulate().g
            }
            <CanvasItemF32Kind>::SelfModulateB => {
                let obj = &self.owner;
                obj.get_self_modulate().b
            }
            <CanvasItemF32Kind>::SelfModulateA => {
                let obj = &self.owner;
                obj.get_self_modulate().a
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <CanvasItemF32Kind>::ModulateR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.r = val;
                obj.set_modulate(prop_val);
            }
            <CanvasItemF32Kind>::ModulateG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.g = val;
                obj.set_modulate(prop_val);
            }
            <CanvasItemF32Kind>::ModulateB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.b = val;
                obj.set_modulate(prop_val);
            }
            <CanvasItemF32Kind>::ModulateA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.a = val;
                obj.set_modulate(prop_val);
            }
            <CanvasItemF32Kind>::SelfModulateR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_self_modulate();
                prop_val.r = val;
                obj.set_self_modulate(prop_val);
            }
            <CanvasItemF32Kind>::SelfModulateG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_self_modulate();
                prop_val.g = val;
                obj.set_self_modulate(prop_val);
            }
            <CanvasItemF32Kind>::SelfModulateB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_self_modulate();
                prop_val.b = val;
                obj.set_self_modulate(prop_val);
            }
            <CanvasItemF32Kind>::SelfModulateA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_self_modulate();
                prop_val.a = val;
                obj.set_self_modulate(prop_val);
            }
        }
    }
}
impl IPropertyData for CanvasItemF32Data {
    type Target = CanvasItem;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CanvasItemF32Kind>::ModulateR => NodePath::from("modulate:r"),
            <CanvasItemF32Kind>::ModulateG => NodePath::from("modulate:g"),
            <CanvasItemF32Kind>::ModulateB => NodePath::from("modulate:b"),
            <CanvasItemF32Kind>::ModulateA => NodePath::from("modulate:a"),
            <CanvasItemF32Kind>::SelfModulateR => NodePath::from("self_modulate:r"),
            <CanvasItemF32Kind>::SelfModulateG => NodePath::from("self_modulate:g"),
            <CanvasItemF32Kind>::SelfModulateB => NodePath::from("self_modulate:b"),
            <CanvasItemF32Kind>::SelfModulateA => NodePath::from("self_modulate:a"),
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
impl TryFromPathAndObject for CanvasItemF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CanvasItem>()
            .ok()
            .and_then(|owner| {
                match path {
                    "modulate:r" => {
                        Some(Self {
                            property: <CanvasItemF32Kind>::ModulateR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:g" => {
                        Some(Self {
                            property: <CanvasItemF32Kind>::ModulateG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:b" => {
                        Some(Self {
                            property: <CanvasItemF32Kind>::ModulateB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:a" => {
                        Some(Self {
                            property: <CanvasItemF32Kind>::ModulateA,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "self_modulate:r" => {
                        Some(Self {
                            property: <CanvasItemF32Kind>::SelfModulateR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "self_modulate:g" => {
                        Some(Self {
                            property: <CanvasItemF32Kind>::SelfModulateG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "self_modulate:b" => {
                        Some(Self {
                            property: <CanvasItemF32Kind>::SelfModulateB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "self_modulate:a" => {
                        Some(Self {
                            property: <CanvasItemF32Kind>::SelfModulateA,
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
pub struct CanvasItemI32Data {
    pub property: CanvasItemI32Kind,
    pub owner: Gd<CanvasItem>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasItemI32Kind {
    ZIndex,
}
impl IProperty<i32> for CanvasItemI32Data {
    #[inline]
    fn get_property_value(&self) -> i32 {
        match self.property {
            <CanvasItemI32Kind>::ZIndex => {
                let obj = &self.owner;
                obj.get_z_index()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i32) {
        match self.property {
            <CanvasItemI32Kind>::ZIndex => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_z_index(val);
            }
        }
    }
}
impl IPropertyData for CanvasItemI32Data {
    type Target = CanvasItem;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CanvasItemI32Kind>::ZIndex => NodePath::from("z_index"),
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
impl TryFromPathAndObject for CanvasItemI32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CanvasItem>()
            .ok()
            .and_then(|owner| {
                match path {
                    "z_index" => {
                        Some(Self {
                            property: <CanvasItemI32Kind>::ZIndex,
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
pub trait DoCanvasItem<Marker = ()> {
    fn do_modulate_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_color_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_modulate_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_color_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_modulate_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_color_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_modulate_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_color_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_self_modulate_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_self_color_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_self_modulate_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_self_color_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_self_modulate_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_self_color_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_self_modulate_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_self_color_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_z_index(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>>;
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
impl<Class: Inherits<CanvasItem> + Inherits<Object>> DoCanvasItem<()> for Gd<Class> {
    fn do_modulate_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_color_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_modulate_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_color_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_modulate_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_color_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_modulate_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_color_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_modulate_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_color_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_modulate_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_color_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_modulate_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_color_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_modulate_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_color_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_z_index(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let data = CanvasItemI32Data {
            property: <CanvasItemI32Kind>::ZIndex,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasItem>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<T: WithBaseField + Inherits<CanvasItem> + Inherits<Object>> DoCanvasItem<BaseMarker>
for T {
    fn do_modulate_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_color_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_modulate_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_color_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_modulate_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_color_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_modulate_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_color_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::ModulateA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_modulate_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_color_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_modulate_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_color_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_modulate_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_color_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_modulate_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_self_color_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemF32Data {
            property: <CanvasItemF32Kind>::SelfModulateA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_z_index(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let owner: Gd<CanvasItem> = self.to_gd().upcast();
        let data = CanvasItemI32Data {
            property: <CanvasItemI32Kind>::ZIndex,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct CanvasItemTweener {}
#[godot_api]
impl CanvasItemTweener {
    #[func]
    fn do_modulate_r(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_modulate_r(to, duration), gd
        }
    }
    #[func]
    fn do_color_r(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_modulate_r(to, duration), gd
        }
    }
    #[func]
    fn do_modulate_g(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_modulate_g(to, duration), gd
        }
    }
    #[func]
    fn do_color_g(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_modulate_g(to, duration), gd
        }
    }
    #[func]
    fn do_modulate_b(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_modulate_b(to, duration), gd
        }
    }
    #[func]
    fn do_color_b(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_modulate_b(to, duration), gd
        }
    }
    #[func]
    fn do_modulate_a(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_modulate_a(to, duration), gd
        }
    }
    #[func]
    fn do_color_a(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_modulate_a(to, duration), gd
        }
    }
    #[func]
    fn do_self_modulate_r(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_self_modulate_r(to, duration), gd
        }
    }
    #[func]
    fn do_self_color_r(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_self_modulate_r(to, duration), gd
        }
    }
    #[func]
    fn do_self_modulate_g(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_self_modulate_g(to, duration), gd
        }
    }
    #[func]
    fn do_self_color_g(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_self_modulate_g(to, duration), gd
        }
    }
    #[func]
    fn do_self_modulate_b(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_self_modulate_b(to, duration), gd
        }
    }
    #[func]
    fn do_self_color_b(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_self_modulate_b(to, duration), gd
        }
    }
    #[func]
    fn do_self_modulate_a(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_self_modulate_a(to, duration), gd
        }
    }
    #[func]
    fn do_self_color_a(
        node: Gd<CanvasItem>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_self_modulate_a(to, duration), gd
        }
    }
    #[func]
    fn do_z_index(
        node: Gd<CanvasItem>,
        to: i32,
        duration: f64,
    ) -> Gd<<i32 as TyToGdTween>::GdTween> {
        let gd = <i32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_z_index(to, duration), gd
        }
    }
    #[func]
    fn do_modulate(
        node: Gd<CanvasItem>,
        to: Color,
        duration: f64,
    ) -> Gd<<Color as TyToGdTween>::GdTween> {
        let gd = <Color as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_modulate(to, duration), gd
        }
    }
    #[func]
    fn do_color(
        node: Gd<CanvasItem>,
        to: Color,
        duration: f64,
    ) -> Gd<<Color as TyToGdTween>::GdTween> {
        let gd = <Color as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_modulate(to, duration), gd
        }
    }
    #[func]
    fn do_self_modulate(
        node: Gd<CanvasItem>,
        to: Color,
        duration: f64,
    ) -> Gd<<Color as TyToGdTween>::GdTween> {
        let gd = <Color as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_self_modulate(to, duration), gd
        }
    }
    #[func]
    fn do_self_color(
        node: Gd<CanvasItem>,
        to: Color,
        duration: f64,
    ) -> Gd<<Color as TyToGdTween>::GdTween> {
        let gd = <Color as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_self_modulate(to, duration), gd
        }
    }
}
