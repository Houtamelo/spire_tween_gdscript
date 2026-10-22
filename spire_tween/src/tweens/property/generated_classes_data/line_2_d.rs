use super::*;
#[derive(Debug, Clone)]
pub struct Line2DFloatData {
    pub property: Line2DFloatKind,
    pub owner: Gd<Line2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Line2DFloatKind {
    DefaultColorR,
    DefaultColorG,
    DefaultColorB,
    DefaultColorA,
    Width,
}
impl IProperty<f64> for Line2DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <Line2DFloatKind>::DefaultColorR => {
                let obj = &self.owner;
                (obj.get_default_color().r) as f64
            }
            <Line2DFloatKind>::DefaultColorG => {
                let obj = &self.owner;
                (obj.get_default_color().g) as f64
            }
            <Line2DFloatKind>::DefaultColorB => {
                let obj = &self.owner;
                (obj.get_default_color().b) as f64
            }
            <Line2DFloatKind>::DefaultColorA => {
                let obj = &self.owner;
                (obj.get_default_color().a) as f64
            }
            <Line2DFloatKind>::Width => {
                let obj = &self.owner;
                (obj.get_width()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <Line2DFloatKind>::DefaultColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_default_color();
                prop_val.r = val as f32;
                obj.set_default_color(prop_val);
            }
            <Line2DFloatKind>::DefaultColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_default_color();
                prop_val.g = val as f32;
                obj.set_default_color(prop_val);
            }
            <Line2DFloatKind>::DefaultColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_default_color();
                prop_val.b = val as f32;
                obj.set_default_color(prop_val);
            }
            <Line2DFloatKind>::DefaultColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_default_color();
                prop_val.a = val as f32;
                obj.set_default_color(prop_val);
            }
            <Line2DFloatKind>::Width => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_width(val as f32);
            }
        }
    }
}
impl IPropertyData for Line2DFloatData {
    type Target = Line2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Line2DFloatKind>::DefaultColorR => NodePath::from("default_color:r"),
            <Line2DFloatKind>::DefaultColorG => NodePath::from("default_color:g"),
            <Line2DFloatKind>::DefaultColorB => NodePath::from("default_color:b"),
            <Line2DFloatKind>::DefaultColorA => NodePath::from("default_color:a"),
            <Line2DFloatKind>::Width => NodePath::from("width"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for Line2DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Line2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "default_color:r" => {
                        Some(Self {
                            property: <Line2DFloatKind>::DefaultColorR,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "default_color:g" => {
                        Some(Self {
                            property: <Line2DFloatKind>::DefaultColorG,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "default_color:b" => {
                        Some(Self {
                            property: <Line2DFloatKind>::DefaultColorB,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "default_color:a" => {
                        Some(Self {
                            property: <Line2DFloatKind>::DefaultColorA,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "width" => {
                        Some(Self {
                            property: <Line2DFloatKind>::Width,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
#[derive(Debug, Clone)]
pub struct Line2DColorData {
    pub property: Line2DColorKind,
    pub owner: Gd<Line2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Line2DColorKind {
    DefaultColor,
}
impl IProperty<Color> for Line2DColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <Line2DColorKind>::DefaultColor => {
                let obj = &self.owner;
                obj.get_default_color()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <Line2DColorKind>::DefaultColor => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_default_color(val);
            }
        }
    }
}
impl IPropertyData for Line2DColorData {
    type Target = Line2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Line2DColorKind>::DefaultColor => NodePath::from("default_color"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for Line2DColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Line2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "default_color" => {
                        Some(Self {
                            property: <Line2DColorKind>::DefaultColor,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoLine2D<Marker = ()> {
    fn do_default_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_default_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_default_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_default_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_width(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>>;
    fn do_default_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
}
impl<Class: Inherits<Line2D> + Inherits<Object>> SpireDoLine2D<()> for Gd<Class> {
    fn do_default_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Line2DFloatData {
            property: <Line2DFloatKind>::DefaultColorR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_default_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Line2DFloatData {
            property: <Line2DFloatKind>::DefaultColorG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_default_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Line2DFloatData {
            property: <Line2DFloatKind>::DefaultColorB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_default_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Line2DFloatData {
            property: <Line2DFloatKind>::DefaultColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_width(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Line2DFloatData {
            property: <Line2DFloatKind>::Width,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_default_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = Line2DColorData {
            property: <Line2DColorKind>::DefaultColor,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<T: WithBaseField + Inherits<Line2D> + Inherits<Object>> SpireDoLine2D<BaseMarker>
for T {
    fn do_default_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Line2D> = self.to_gd().upcast();
        let data = Line2DFloatData {
            property: <Line2DFloatKind>::DefaultColorR,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_default_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Line2D> = self.to_gd().upcast();
        let data = Line2DFloatData {
            property: <Line2DFloatKind>::DefaultColorG,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_default_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Line2D> = self.to_gd().upcast();
        let data = Line2DFloatData {
            property: <Line2DFloatKind>::DefaultColorB,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_default_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Line2D> = self.to_gd().upcast();
        let data = Line2DFloatData {
            property: <Line2DFloatKind>::DefaultColorA,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_width(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Line2D> = self.to_gd().upcast();
        let data = Line2DFloatData {
            property: <Line2DFloatKind>::Width,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_default_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<Line2D> = self.to_gd().upcast();
        let data = Line2DColorData {
            property: <Line2DColorKind>::DefaultColor,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
