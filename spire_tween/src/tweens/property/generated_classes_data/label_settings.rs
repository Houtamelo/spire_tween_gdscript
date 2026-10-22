use super::*;
#[derive(Debug, Clone)]
pub struct LabelSettingsIntData {
    pub property: LabelSettingsIntKind,
    pub owner: Gd<LabelSettings>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelSettingsIntKind {
    FontSize,
    OutlineSize,
}
impl IProperty<i64> for LabelSettingsIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <LabelSettingsIntKind>::FontSize => {
                let obj = &self.owner;
                (obj.get_font_size()) as i64
            }
            <LabelSettingsIntKind>::OutlineSize => {
                let obj = &self.owner;
                (obj.get_outline_size()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <LabelSettingsIntKind>::FontSize => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_font_size(val as i32);
            }
            <LabelSettingsIntKind>::OutlineSize => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_outline_size(val as i32);
            }
        }
    }
}
impl IPropertyData for LabelSettingsIntData {
    type Target = LabelSettings;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LabelSettingsIntKind>::FontSize => NodePath::from("font_size"),
            <LabelSettingsIntKind>::OutlineSize => NodePath::from("outline_size"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for LabelSettingsIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<LabelSettings>()
            .ok()
            .and_then(|owner| {
                match path {
                    "font_size" => {
                        Some(Self {
                            property: <LabelSettingsIntKind>::FontSize,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_size" => {
                        Some(Self {
                            property: <LabelSettingsIntKind>::OutlineSize,
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
pub struct LabelSettingsFloatData {
    pub property: LabelSettingsFloatKind,
    pub owner: Gd<LabelSettings>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelSettingsFloatKind {
    FontColorR,
    FontColorG,
    FontColorB,
    FontColorA,
    OutlineColorR,
    OutlineColorG,
    OutlineColorB,
    OutlineColorA,
}
impl IProperty<f64> for LabelSettingsFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <LabelSettingsFloatKind>::FontColorR => {
                let obj = &self.owner;
                (obj.get_font_color().r) as f64
            }
            <LabelSettingsFloatKind>::FontColorG => {
                let obj = &self.owner;
                (obj.get_font_color().g) as f64
            }
            <LabelSettingsFloatKind>::FontColorB => {
                let obj = &self.owner;
                (obj.get_font_color().b) as f64
            }
            <LabelSettingsFloatKind>::FontColorA => {
                let obj = &self.owner;
                (obj.get_font_color().a) as f64
            }
            <LabelSettingsFloatKind>::OutlineColorR => {
                let obj = &self.owner;
                (obj.get_outline_color().r) as f64
            }
            <LabelSettingsFloatKind>::OutlineColorG => {
                let obj = &self.owner;
                (obj.get_outline_color().g) as f64
            }
            <LabelSettingsFloatKind>::OutlineColorB => {
                let obj = &self.owner;
                (obj.get_outline_color().b) as f64
            }
            <LabelSettingsFloatKind>::OutlineColorA => {
                let obj = &self.owner;
                (obj.get_outline_color().a) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <LabelSettingsFloatKind>::FontColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_font_color();
                prop_val.r = val as f32;
                obj.set_font_color(prop_val);
            }
            <LabelSettingsFloatKind>::FontColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_font_color();
                prop_val.g = val as f32;
                obj.set_font_color(prop_val);
            }
            <LabelSettingsFloatKind>::FontColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_font_color();
                prop_val.b = val as f32;
                obj.set_font_color(prop_val);
            }
            <LabelSettingsFloatKind>::FontColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_font_color();
                prop_val.a = val as f32;
                obj.set_font_color(prop_val);
            }
            <LabelSettingsFloatKind>::OutlineColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_outline_color();
                prop_val.r = val as f32;
                obj.set_outline_color(prop_val);
            }
            <LabelSettingsFloatKind>::OutlineColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_outline_color();
                prop_val.g = val as f32;
                obj.set_outline_color(prop_val);
            }
            <LabelSettingsFloatKind>::OutlineColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_outline_color();
                prop_val.b = val as f32;
                obj.set_outline_color(prop_val);
            }
            <LabelSettingsFloatKind>::OutlineColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_outline_color();
                prop_val.a = val as f32;
                obj.set_outline_color(prop_val);
            }
        }
    }
}
impl IPropertyData for LabelSettingsFloatData {
    type Target = LabelSettings;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LabelSettingsFloatKind>::FontColorR => NodePath::from("font_color:r"),
            <LabelSettingsFloatKind>::FontColorG => NodePath::from("font_color:g"),
            <LabelSettingsFloatKind>::FontColorB => NodePath::from("font_color:b"),
            <LabelSettingsFloatKind>::FontColorA => NodePath::from("font_color:a"),
            <LabelSettingsFloatKind>::OutlineColorR => NodePath::from("outline_color:r"),
            <LabelSettingsFloatKind>::OutlineColorG => NodePath::from("outline_color:g"),
            <LabelSettingsFloatKind>::OutlineColorB => NodePath::from("outline_color:b"),
            <LabelSettingsFloatKind>::OutlineColorA => NodePath::from("outline_color:a"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for LabelSettingsFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<LabelSettings>()
            .ok()
            .and_then(|owner| {
                match path {
                    "font_color:r" => {
                        Some(Self {
                            property: <LabelSettingsFloatKind>::FontColorR,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "font_color:g" => {
                        Some(Self {
                            property: <LabelSettingsFloatKind>::FontColorG,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "font_color:b" => {
                        Some(Self {
                            property: <LabelSettingsFloatKind>::FontColorB,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "font_color:a" => {
                        Some(Self {
                            property: <LabelSettingsFloatKind>::FontColorA,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color:r" => {
                        Some(Self {
                            property: <LabelSettingsFloatKind>::OutlineColorR,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color:g" => {
                        Some(Self {
                            property: <LabelSettingsFloatKind>::OutlineColorG,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color:b" => {
                        Some(Self {
                            property: <LabelSettingsFloatKind>::OutlineColorB,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color:a" => {
                        Some(Self {
                            property: <LabelSettingsFloatKind>::OutlineColorA,
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
pub struct LabelSettingsColorData {
    pub property: LabelSettingsColorKind,
    pub owner: Gd<LabelSettings>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelSettingsColorKind {
    FontColor,
    OutlineColor,
}
impl IProperty<Color> for LabelSettingsColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <LabelSettingsColorKind>::FontColor => {
                let obj = &self.owner;
                obj.get_font_color()
            }
            <LabelSettingsColorKind>::OutlineColor => {
                let obj = &self.owner;
                obj.get_outline_color()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <LabelSettingsColorKind>::FontColor => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_font_color(val);
            }
            <LabelSettingsColorKind>::OutlineColor => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_outline_color(val);
            }
        }
    }
}
impl IPropertyData for LabelSettingsColorData {
    type Target = LabelSettings;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LabelSettingsColorKind>::FontColor => NodePath::from("font_color"),
            <LabelSettingsColorKind>::OutlineColor => NodePath::from("outline_color"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for LabelSettingsColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<LabelSettings>()
            .ok()
            .and_then(|owner| {
                match path {
                    "font_color" => {
                        Some(Self {
                            property: <LabelSettingsColorKind>::FontColor,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color" => {
                        Some(Self {
                            property: <LabelSettingsColorKind>::OutlineColor,
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
pub trait SpireDoLabelSettings<Marker = ()> {
    fn do_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_outline_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_font_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_font_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_font_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_font_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_outline_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_outline_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_outline_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_outline_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_font_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
    fn do_outline_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
}
impl<Class: Inherits<LabelSettings> + Inherits<Object>> SpireDoLabelSettings<()>
for Gd<Class> {
    fn do_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = LabelSettingsIntData {
            property: <LabelSettingsIntKind>::FontSize,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = LabelSettingsIntData {
            property: <LabelSettingsIntKind>::OutlineSize,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_font_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::FontColorR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_font_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::FontColorG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_font_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::FontColorB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_font_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::FontColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::OutlineColorR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::OutlineColorG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::OutlineColorB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::OutlineColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_font_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = LabelSettingsColorData {
            property: <LabelSettingsColorKind>::FontColor,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = LabelSettingsColorData {
            property: <LabelSettingsColorKind>::OutlineColor,
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
impl<
    T: WithBaseField + Inherits<LabelSettings> + Inherits<Object>,
> SpireDoLabelSettings<BaseMarker> for T {
    fn do_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<LabelSettings> = self.to_gd().upcast();
        let data = LabelSettingsIntData {
            property: <LabelSettingsIntKind>::FontSize,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<LabelSettings> = self.to_gd().upcast();
        let data = LabelSettingsIntData {
            property: <LabelSettingsIntKind>::OutlineSize,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_font_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<LabelSettings> = self.to_gd().upcast();
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::FontColorR,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_font_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<LabelSettings> = self.to_gd().upcast();
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::FontColorG,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_font_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<LabelSettings> = self.to_gd().upcast();
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::FontColorB,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_font_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<LabelSettings> = self.to_gd().upcast();
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::FontColorA,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<LabelSettings> = self.to_gd().upcast();
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::OutlineColorR,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<LabelSettings> = self.to_gd().upcast();
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::OutlineColorG,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<LabelSettings> = self.to_gd().upcast();
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::OutlineColorB,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<LabelSettings> = self.to_gd().upcast();
        let data = LabelSettingsFloatData {
            property: <LabelSettingsFloatKind>::OutlineColorA,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_font_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<LabelSettings> = self.to_gd().upcast();
        let data = LabelSettingsColorData {
            property: <LabelSettingsColorKind>::FontColor,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<LabelSettings> = self.to_gd().upcast();
        let data = LabelSettingsColorData {
            property: <LabelSettingsColorKind>::OutlineColor,
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
