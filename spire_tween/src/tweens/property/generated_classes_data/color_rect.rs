use super::*;
#[derive(Debug, Clone)]
pub struct ColorRectFloatData {
    pub property: ColorRectFloatKind,
    pub owner: Gd<ColorRect>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorRectFloatKind {
    ColorR,
    ColorG,
    ColorB,
    ColorA,
}
impl IProperty<f64> for ColorRectFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <ColorRectFloatKind>::ColorR => {
                let obj = &self.owner;
                (obj.get_color().r) as f64
            }
            <ColorRectFloatKind>::ColorG => {
                let obj = &self.owner;
                (obj.get_color().g) as f64
            }
            <ColorRectFloatKind>::ColorB => {
                let obj = &self.owner;
                (obj.get_color().b) as f64
            }
            <ColorRectFloatKind>::ColorA => {
                let obj = &self.owner;
                (obj.get_color().a) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <ColorRectFloatKind>::ColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.r = val as f32;
                obj.set_color(prop_val);
            }
            <ColorRectFloatKind>::ColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.g = val as f32;
                obj.set_color(prop_val);
            }
            <ColorRectFloatKind>::ColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.b = val as f32;
                obj.set_color(prop_val);
            }
            <ColorRectFloatKind>::ColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.a = val as f32;
                obj.set_color(prop_val);
            }
        }
    }
}
impl IPropertyData for ColorRectFloatData {
    type Target = ColorRect;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ColorRectFloatKind>::ColorR => NodePath::from("color:r"),
            <ColorRectFloatKind>::ColorG => NodePath::from("color:g"),
            <ColorRectFloatKind>::ColorB => NodePath::from("color:b"),
            <ColorRectFloatKind>::ColorA => NodePath::from("color:a"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<ColorRect>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<ColorRect>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ColorRectFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<ColorRect>()
            .ok()
            .and_then(|owner| {
                match path {
                    "color:r" => {
                        Some(Self {
                            property: <ColorRectFloatKind>::ColorR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "color:g" => {
                        Some(Self {
                            property: <ColorRectFloatKind>::ColorG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "color:b" => {
                        Some(Self {
                            property: <ColorRectFloatKind>::ColorB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "color:a" => {
                        Some(Self {
                            property: <ColorRectFloatKind>::ColorA,
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
pub struct ColorRectColorData {
    pub property: ColorRectColorKind,
    pub owner: Gd<ColorRect>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorRectColorKind {
    Color,
}
impl IProperty<Color> for ColorRectColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <ColorRectColorKind>::Color => {
                let obj = &self.owner;
                obj.get_color()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <ColorRectColorKind>::Color => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_color(val);
            }
        }
    }
}
impl IPropertyData for ColorRectColorData {
    type Target = ColorRect;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ColorRectColorKind>::Color => NodePath::from("color"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<ColorRect>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<ColorRect>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ColorRectColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<ColorRect>()
            .ok()
            .and_then(|owner| {
                match path {
                    "color" => {
                        Some(Self {
                            property: <ColorRectColorKind>::Color,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoColorRect<Marker = ()> {
    fn do_rect_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_rect_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_rect_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_rect_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_rect_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
}
impl<Class: Inherits<ColorRect> + Inherits<Object>> SpireDoColorRect<()> for Gd<Class> {
    fn do_rect_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ColorRectFloatData {
            property: <ColorRectFloatKind>::ColorR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ColorRect>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rect_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ColorRectFloatData {
            property: <ColorRectFloatKind>::ColorG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ColorRect>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rect_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ColorRectFloatData {
            property: <ColorRectFloatKind>::ColorB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ColorRect>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rect_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ColorRectFloatData {
            property: <ColorRectFloatKind>::ColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ColorRect>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rect_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = ColorRectColorData {
            property: <ColorRectColorKind>::Color,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ColorRect>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<ColorRect> + Inherits<Object>,
> SpireDoColorRect<BaseMarker> for T {
    fn do_rect_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ColorRect> = self.to_gd().upcast();
        let data = ColorRectFloatData {
            property: <ColorRectFloatKind>::ColorR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rect_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ColorRect> = self.to_gd().upcast();
        let data = ColorRectFloatData {
            property: <ColorRectFloatKind>::ColorG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rect_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ColorRect> = self.to_gd().upcast();
        let data = ColorRectFloatData {
            property: <ColorRectFloatKind>::ColorB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rect_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ColorRect> = self.to_gd().upcast();
        let data = ColorRectFloatData {
            property: <ColorRectFloatKind>::ColorA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_rect_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<ColorRect> = self.to_gd().upcast();
        let data = ColorRectColorData {
            property: <ColorRectColorKind>::Color,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
