use super::*;
#[derive(Debug, Clone)]
pub struct RichTextLabelIntData {
    pub property: RichTextLabelIntKind,
    pub owner: Gd<RichTextLabel>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichTextLabelIntKind {
    NormalFontSize,
    BoldFontSize,
    ItalicsFontSize,
    BoldItalicsFontSize,
    MonoFontSize,
    OutlineSize,
    VisibleCharacters,
}
impl IProperty<i64> for RichTextLabelIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <RichTextLabelIntKind>::NormalFontSize => {
                let obj = &self.owner;
                (obj.get_theme_font_size("normal_font_size")) as i64
            }
            <RichTextLabelIntKind>::BoldFontSize => {
                let obj = &self.owner;
                (obj.get_theme_font_size("bold_font_size")) as i64
            }
            <RichTextLabelIntKind>::ItalicsFontSize => {
                let obj = &self.owner;
                (obj.get_theme_font_size("italics_font_size")) as i64
            }
            <RichTextLabelIntKind>::BoldItalicsFontSize => {
                let obj = &self.owner;
                (obj.get_theme_font_size("bold_italics_font_size")) as i64
            }
            <RichTextLabelIntKind>::MonoFontSize => {
                let obj = &self.owner;
                (obj.get_theme_font_size("mono_font_size")) as i64
            }
            <RichTextLabelIntKind>::OutlineSize => {
                let obj = &self.owner;
                (obj.get_theme_constant("outline_size")) as i64
            }
            <RichTextLabelIntKind>::VisibleCharacters => {
                let obj = &self.owner;
                (obj.get_visible_characters()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <RichTextLabelIntKind>::NormalFontSize => {
                let obj = &mut self.owner;
                let val = value;
                obj.add_theme_font_size_override("normal_font_size", val as i32);
            }
            <RichTextLabelIntKind>::BoldFontSize => {
                let obj = &mut self.owner;
                let val = value;
                obj.add_theme_font_size_override("bold_font_size", val as i32);
            }
            <RichTextLabelIntKind>::ItalicsFontSize => {
                let obj = &mut self.owner;
                let val = value;
                obj.add_theme_font_size_override("italics_font_size", val as i32);
            }
            <RichTextLabelIntKind>::BoldItalicsFontSize => {
                let obj = &mut self.owner;
                let val = value;
                obj.add_theme_font_size_override("bold_italics_font_size", val as i32);
            }
            <RichTextLabelIntKind>::MonoFontSize => {
                let obj = &mut self.owner;
                let val = value;
                obj.add_theme_font_size_override("mono_font_size", val as i32);
            }
            <RichTextLabelIntKind>::OutlineSize => {
                let obj = &mut self.owner;
                let val = value;
                obj.add_theme_constant_override("outline_size", val as i32);
            }
            <RichTextLabelIntKind>::VisibleCharacters => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_visible_characters(val as i32);
            }
        }
    }
}
impl IPropertyData for RichTextLabelIntData {
    type Target = RichTextLabel;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RichTextLabelIntKind>::NormalFontSize => NodePath::from("normal_font_size"),
            <RichTextLabelIntKind>::BoldFontSize => NodePath::from("bold_font_size"),
            <RichTextLabelIntKind>::ItalicsFontSize => {
                NodePath::from("italics_font_size")
            }
            <RichTextLabelIntKind>::BoldItalicsFontSize => {
                NodePath::from("bold_italics_font_size")
            }
            <RichTextLabelIntKind>::MonoFontSize => NodePath::from("mono_font_size"),
            <RichTextLabelIntKind>::OutlineSize => NodePath::from("outline_size"),
            <RichTextLabelIntKind>::VisibleCharacters => {
                NodePath::from("visible_characters")
            }
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for RichTextLabelIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RichTextLabel>()
            .ok()
            .and_then(|owner| {
                match path {
                    "normal_font_size" => {
                        Some(Self {
                            property: <RichTextLabelIntKind>::NormalFontSize,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "bold_font_size" => {
                        Some(Self {
                            property: <RichTextLabelIntKind>::BoldFontSize,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "italics_font_size" => {
                        Some(Self {
                            property: <RichTextLabelIntKind>::ItalicsFontSize,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "bold_italics_font_size" => {
                        Some(Self {
                            property: <RichTextLabelIntKind>::BoldItalicsFontSize,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "mono_font_size" => {
                        Some(Self {
                            property: <RichTextLabelIntKind>::MonoFontSize,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_size" => {
                        Some(Self {
                            property: <RichTextLabelIntKind>::OutlineSize,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "visible_characters" => {
                        Some(Self {
                            property: <RichTextLabelIntKind>::VisibleCharacters,
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
pub struct RichTextLabelFloatData {
    pub property: RichTextLabelFloatKind,
    pub owner: Gd<RichTextLabel>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichTextLabelFloatKind {
    DefaultColorR,
    DefaultColorG,
    DefaultColorB,
    DefaultColorA,
    OutlineColorR,
    OutlineColorG,
    OutlineColorB,
    OutlineColorA,
    VisibleRatio,
}
impl IProperty<f64> for RichTextLabelFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <RichTextLabelFloatKind>::DefaultColorR => {
                let obj = &self.owner;
                (obj.get_theme_color("default_color").r) as f64
            }
            <RichTextLabelFloatKind>::DefaultColorG => {
                let obj = &self.owner;
                (obj.get_theme_color("default_color").g) as f64
            }
            <RichTextLabelFloatKind>::DefaultColorB => {
                let obj = &self.owner;
                (obj.get_theme_color("default_color").b) as f64
            }
            <RichTextLabelFloatKind>::DefaultColorA => {
                let obj = &self.owner;
                (obj.get_theme_color("default_color").a) as f64
            }
            <RichTextLabelFloatKind>::OutlineColorR => {
                let obj = &self.owner;
                (obj.get_theme_color("font_outline_color").r) as f64
            }
            <RichTextLabelFloatKind>::OutlineColorG => {
                let obj = &self.owner;
                (obj.get_theme_color("font_outline_color").g) as f64
            }
            <RichTextLabelFloatKind>::OutlineColorB => {
                let obj = &self.owner;
                (obj.get_theme_color("font_outline_color").b) as f64
            }
            <RichTextLabelFloatKind>::OutlineColorA => {
                let obj = &self.owner;
                (obj.get_theme_color("font_outline_color").a) as f64
            }
            <RichTextLabelFloatKind>::VisibleRatio => {
                let obj = &self.owner;
                (obj.get_visible_ratio()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <RichTextLabelFloatKind>::DefaultColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_theme_color("default_color");
                prop_val.r = val as f32;
                let val = prop_val;
                obj.add_theme_color_override("default_color", val);
            }
            <RichTextLabelFloatKind>::DefaultColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_theme_color("default_color");
                prop_val.g = val as f32;
                let val = prop_val;
                obj.add_theme_color_override("default_color", val);
            }
            <RichTextLabelFloatKind>::DefaultColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_theme_color("default_color");
                prop_val.b = val as f32;
                let val = prop_val;
                obj.add_theme_color_override("default_color", val);
            }
            <RichTextLabelFloatKind>::DefaultColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_theme_color("default_color");
                prop_val.a = val as f32;
                let val = prop_val;
                obj.add_theme_color_override("default_color", val);
            }
            <RichTextLabelFloatKind>::OutlineColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_theme_color("font_outline_color");
                prop_val.r = val as f32;
                let val = prop_val;
                obj.add_theme_color_override("font_outline_color", val);
            }
            <RichTextLabelFloatKind>::OutlineColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_theme_color("font_outline_color");
                prop_val.g = val as f32;
                let val = prop_val;
                obj.add_theme_color_override("font_outline_color", val);
            }
            <RichTextLabelFloatKind>::OutlineColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_theme_color("font_outline_color");
                prop_val.b = val as f32;
                let val = prop_val;
                obj.add_theme_color_override("font_outline_color", val);
            }
            <RichTextLabelFloatKind>::OutlineColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_theme_color("font_outline_color");
                prop_val.a = val as f32;
                let val = prop_val;
                obj.add_theme_color_override("font_outline_color", val);
            }
            <RichTextLabelFloatKind>::VisibleRatio => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_visible_ratio(val as f32);
            }
        }
    }
}
impl IPropertyData for RichTextLabelFloatData {
    type Target = RichTextLabel;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RichTextLabelFloatKind>::DefaultColorR => NodePath::from("default_color:r"),
            <RichTextLabelFloatKind>::DefaultColorG => NodePath::from("default_color:g"),
            <RichTextLabelFloatKind>::DefaultColorB => NodePath::from("default_color:b"),
            <RichTextLabelFloatKind>::DefaultColorA => NodePath::from("default_color:a"),
            <RichTextLabelFloatKind>::OutlineColorR => NodePath::from("outline_color:r"),
            <RichTextLabelFloatKind>::OutlineColorG => NodePath::from("outline_color:g"),
            <RichTextLabelFloatKind>::OutlineColorB => NodePath::from("outline_color:b"),
            <RichTextLabelFloatKind>::OutlineColorA => NodePath::from("outline_color:a"),
            <RichTextLabelFloatKind>::VisibleRatio => NodePath::from("visible_ratio"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for RichTextLabelFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RichTextLabel>()
            .ok()
            .and_then(|owner| {
                match path {
                    "default_color:r" => {
                        Some(Self {
                            property: <RichTextLabelFloatKind>::DefaultColorR,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "default_color:g" => {
                        Some(Self {
                            property: <RichTextLabelFloatKind>::DefaultColorG,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "default_color:b" => {
                        Some(Self {
                            property: <RichTextLabelFloatKind>::DefaultColorB,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "default_color:a" => {
                        Some(Self {
                            property: <RichTextLabelFloatKind>::DefaultColorA,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color:r" => {
                        Some(Self {
                            property: <RichTextLabelFloatKind>::OutlineColorR,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color:g" => {
                        Some(Self {
                            property: <RichTextLabelFloatKind>::OutlineColorG,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color:b" => {
                        Some(Self {
                            property: <RichTextLabelFloatKind>::OutlineColorB,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color:a" => {
                        Some(Self {
                            property: <RichTextLabelFloatKind>::OutlineColorA,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "visible_ratio" => {
                        Some(Self {
                            property: <RichTextLabelFloatKind>::VisibleRatio,
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
pub struct RichTextLabelColorData {
    pub property: RichTextLabelColorKind,
    pub owner: Gd<RichTextLabel>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichTextLabelColorKind {
    DefaultColor,
    OutlineColor,
}
impl IProperty<Color> for RichTextLabelColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <RichTextLabelColorKind>::DefaultColor => {
                let obj = &self.owner;
                obj.get_theme_color("default_color")
            }
            <RichTextLabelColorKind>::OutlineColor => {
                let obj = &self.owner;
                obj.get_theme_color("font_outline_color")
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <RichTextLabelColorKind>::DefaultColor => {
                let obj = &mut self.owner;
                let val = value;
                obj.add_theme_color_override("default_color", val);
            }
            <RichTextLabelColorKind>::OutlineColor => {
                let obj = &mut self.owner;
                let val = value;
                obj.add_theme_color_override("font_outline_color", val);
            }
        }
    }
}
impl IPropertyData for RichTextLabelColorData {
    type Target = RichTextLabel;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RichTextLabelColorKind>::DefaultColor => NodePath::from("default_color"),
            <RichTextLabelColorKind>::OutlineColor => NodePath::from("outline_color"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for RichTextLabelColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RichTextLabel>()
            .ok()
            .and_then(|owner| {
                match path {
                    "default_color" => {
                        Some(Self {
                            property: <RichTextLabelColorKind>::DefaultColor,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color" => {
                        Some(Self {
                            property: <RichTextLabelColorKind>::OutlineColor,
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
pub struct RichTextLabelStringData {
    pub property: RichTextLabelStringKind,
    pub owner: Gd<RichTextLabel>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RichTextLabelStringKind {
    Text,
}
impl IProperty<GString> for RichTextLabelStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <RichTextLabelStringKind>::Text => {
                let obj = &self.owner;
                obj.get_text()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <RichTextLabelStringKind>::Text => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
        }
    }
}
impl IPropertyData for RichTextLabelStringData {
    type Target = RichTextLabel;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RichTextLabelStringKind>::Text => NodePath::from("text"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for RichTextLabelStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<RichTextLabel>()
            .ok()
            .and_then(|owner| {
                match path {
                    "text" => {
                        Some(Self {
                            property: <RichTextLabelStringKind>::Text,
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
pub trait SpireDoRichTextLabel<Marker = ()> {
    fn do_normal_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_bold_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_italics_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_bold_italics_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_mono_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_outline_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_visible_characters(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
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
    fn do_visible_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_default_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
    fn do_outline_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
    fn do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<RichTextLabel> + Inherits<Object>> SpireDoRichTextLabel<()>
for Gd<Class> {
    fn do_normal_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::NormalFontSize,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_bold_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::BoldFontSize,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_italics_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::ItalicsFontSize,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_bold_italics_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::BoldItalicsFontSize,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_mono_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::MonoFontSize,
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
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::OutlineSize,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_visible_characters(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::VisibleCharacters,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_default_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::DefaultColorR,
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
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::DefaultColorG,
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
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::DefaultColorB,
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
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::DefaultColorA,
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
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::OutlineColorR,
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
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::OutlineColorG,
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
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::OutlineColorB,
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
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::OutlineColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_visible_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::VisibleRatio,
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
        let data = RichTextLabelColorData {
            property: <RichTextLabelColorKind>::DefaultColor,
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
        let data = RichTextLabelColorData {
            property: <RichTextLabelColorKind>::OutlineColor,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = RichTextLabelStringData {
            property: <RichTextLabelStringKind>::Text,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<RichTextLabel> + Inherits<Object>,
> SpireDoRichTextLabel<BaseMarker> for T {
    fn do_normal_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::NormalFontSize,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_bold_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::BoldFontSize,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_italics_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::ItalicsFontSize,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_bold_italics_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::BoldItalicsFontSize,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_mono_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::MonoFontSize,
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
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::OutlineSize,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_visible_characters(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelIntData {
            property: <RichTextLabelIntKind>::VisibleCharacters,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_default_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::DefaultColorR,
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
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::DefaultColorG,
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
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::DefaultColorB,
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
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::DefaultColorA,
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
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::OutlineColorR,
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
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::OutlineColorG,
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
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::OutlineColorB,
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
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::OutlineColorA,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_visible_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelFloatData {
            property: <RichTextLabelFloatKind>::VisibleRatio,
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
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelColorData {
            property: <RichTextLabelColorKind>::DefaultColor,
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
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelColorData {
            property: <RichTextLabelColorKind>::OutlineColor,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<RichTextLabel> = self.to_gd().upcast();
        let data = RichTextLabelStringData {
            property: <RichTextLabelStringKind>::Text,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
