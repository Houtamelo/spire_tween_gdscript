use super::*;
#[derive(Debug, Clone)]
pub struct CanvasModulateFloatData {
    pub property: CanvasModulateFloatKind,
    pub owner: Gd<CanvasModulate>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasModulateFloatKind {
    ColorR,
    ColorG,
    ColorB,
    ColorA,
}
impl IProperty<f64> for CanvasModulateFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <CanvasModulateFloatKind>::ColorR => {
                let obj = &self.owner;
                (obj.get_color().r) as f64
            }
            <CanvasModulateFloatKind>::ColorG => {
                let obj = &self.owner;
                (obj.get_color().g) as f64
            }
            <CanvasModulateFloatKind>::ColorB => {
                let obj = &self.owner;
                (obj.get_color().b) as f64
            }
            <CanvasModulateFloatKind>::ColorA => {
                let obj = &self.owner;
                (obj.get_color().a) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <CanvasModulateFloatKind>::ColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.r = val as f32;
                obj.set_color(prop_val);
            }
            <CanvasModulateFloatKind>::ColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.g = val as f32;
                obj.set_color(prop_val);
            }
            <CanvasModulateFloatKind>::ColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.b = val as f32;
                obj.set_color(prop_val);
            }
            <CanvasModulateFloatKind>::ColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.a = val as f32;
                obj.set_color(prop_val);
            }
        }
    }
}
impl IPropertyData for CanvasModulateFloatData {
    type Target = CanvasModulate;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CanvasModulateFloatKind>::ColorR => NodePath::from("color:r"),
            <CanvasModulateFloatKind>::ColorG => NodePath::from("color:g"),
            <CanvasModulateFloatKind>::ColorB => NodePath::from("color:b"),
            <CanvasModulateFloatKind>::ColorA => NodePath::from("color:a"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<CanvasModulate>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<CanvasModulate>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for CanvasModulateFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CanvasModulate>()
            .ok()
            .and_then(|owner| {
                match path {
                    "color:r" => {
                        Some(Self {
                            property: <CanvasModulateFloatKind>::ColorR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "color:g" => {
                        Some(Self {
                            property: <CanvasModulateFloatKind>::ColorG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "color:b" => {
                        Some(Self {
                            property: <CanvasModulateFloatKind>::ColorB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "color:a" => {
                        Some(Self {
                            property: <CanvasModulateFloatKind>::ColorA,
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
pub struct CanvasModulateColorData {
    pub property: CanvasModulateColorKind,
    pub owner: Gd<CanvasModulate>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasModulateColorKind {
    Color,
}
impl IProperty<Color> for CanvasModulateColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <CanvasModulateColorKind>::Color => {
                let obj = &self.owner;
                obj.get_color()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <CanvasModulateColorKind>::Color => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_color(val);
            }
        }
    }
}
impl IPropertyData for CanvasModulateColorData {
    type Target = CanvasModulate;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CanvasModulateColorKind>::Color => NodePath::from("color"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<CanvasModulate>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<CanvasModulate>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for CanvasModulateColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CanvasModulate>()
            .ok()
            .and_then(|owner| {
                match path {
                    "color" => {
                        Some(Self {
                            property: <CanvasModulateColorKind>::Color,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoCanvasModulate<Marker = ()> {
    fn do_canvas_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_canvas_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_canvas_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_canvas_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_canvas_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
}
impl<Class: Inherits<CanvasModulate> + Inherits<Object>> SpireDoCanvasModulate<()>
for Gd<Class> {
    fn do_canvas_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasModulateFloatData {
            property: <CanvasModulateFloatKind>::ColorR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasModulate>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_canvas_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasModulateFloatData {
            property: <CanvasModulateFloatKind>::ColorG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasModulate>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_canvas_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasModulateFloatData {
            property: <CanvasModulateFloatKind>::ColorB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasModulate>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_canvas_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CanvasModulateFloatData {
            property: <CanvasModulateFloatKind>::ColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasModulate>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_canvas_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = CanvasModulateColorData {
            property: <CanvasModulateColorKind>::Color,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasModulate>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<CanvasModulate> + Inherits<Object>,
> SpireDoCanvasModulate<BaseMarker> for T {
    fn do_canvas_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasModulate> = self.to_gd().upcast();
        let data = CanvasModulateFloatData {
            property: <CanvasModulateFloatKind>::ColorR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_canvas_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasModulate> = self.to_gd().upcast();
        let data = CanvasModulateFloatData {
            property: <CanvasModulateFloatKind>::ColorG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_canvas_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasModulate> = self.to_gd().upcast();
        let data = CanvasModulateFloatData {
            property: <CanvasModulateFloatKind>::ColorB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_canvas_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CanvasModulate> = self.to_gd().upcast();
        let data = CanvasModulateFloatData {
            property: <CanvasModulateFloatKind>::ColorA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_canvas_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<CanvasModulate> = self.to_gd().upcast();
        let data = CanvasModulateColorData {
            property: <CanvasModulateColorKind>::Color,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
