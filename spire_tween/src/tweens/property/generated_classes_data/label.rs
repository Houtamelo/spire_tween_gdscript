use super::*;
#[derive(Debug, Clone)]
pub struct LabelIntData {
    pub property: LabelIntKind,
    pub owner: Gd<Label>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelIntKind {
    FontSize,
    OutlineSize,
    MaxLinesVisible,
    VisibleCharacters,
}
impl IProperty<i64> for LabelIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <LabelIntKind>::FontSize => {
                let obj = &self.owner;
                (obj
                    .get_label_settings()
                    .map(|settings| settings.get_font_size())
                    .unwrap_or_else(|| obj.get_theme_font_size("font_size"))) as i64
            }
            <LabelIntKind>::OutlineSize => {
                let obj = &self.owner;
                (obj
                    .get_label_settings()
                    .map(|settings| settings.get_outline_size())
                    .unwrap_or_else(|| obj.get_theme_constant("outline_size"))) as i64
            }
            <LabelIntKind>::MaxLinesVisible => {
                let obj = &self.owner;
                (obj.get_max_lines_visible()) as i64
            }
            <LabelIntKind>::VisibleCharacters => {
                let obj = &self.owner;
                (obj.get_visible_characters()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <LabelIntKind>::FontSize => {
                let obj = &mut self.owner;
                let val = value;
                if let Some(mut settings) = obj.get_label_settings() {
                    settings.set_font_size(val as i32);
                } else {
                    obj.add_theme_font_size_override("font_size", val as i32);
                }
            }
            <LabelIntKind>::OutlineSize => {
                let obj = &mut self.owner;
                let val = value;
                if let Some(mut settings) = obj.get_label_settings() {
                    settings.set_outline_size(val as i32);
                } else {
                    obj.add_theme_constant_override("outline_size", val as i32);
                }
            }
            <LabelIntKind>::MaxLinesVisible => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_max_lines_visible(val as i32);
            }
            <LabelIntKind>::VisibleCharacters => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_visible_characters(val as i32);
            }
        }
    }
}
impl IPropertyData for LabelIntData {
    type Target = Label;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LabelIntKind>::FontSize => NodePath::from("font_size"),
            <LabelIntKind>::OutlineSize => NodePath::from("outline_size"),
            <LabelIntKind>::MaxLinesVisible => NodePath::from("max_lines_visible"),
            <LabelIntKind>::VisibleCharacters => NodePath::from("visible_characters"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for LabelIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label>()
            .ok()
            .and_then(|owner| {
                match path {
                    "font_size" => {
                        Some(Self {
                            property: <LabelIntKind>::FontSize,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_size" => {
                        Some(Self {
                            property: <LabelIntKind>::OutlineSize,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "max_lines_visible" => {
                        Some(Self {
                            property: <LabelIntKind>::MaxLinesVisible,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "visible_characters" => {
                        Some(Self {
                            property: <LabelIntKind>::VisibleCharacters,
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
pub struct LabelFloatData {
    pub property: LabelFloatKind,
    pub owner: Gd<Label>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelFloatKind {
    FontColorR,
    FontColorG,
    FontColorB,
    FontColorA,
    OutlineColorR,
    OutlineColorG,
    OutlineColorB,
    OutlineColorA,
    VisibleRatio,
}
impl IProperty<f64> for LabelFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <LabelFloatKind>::FontColorR => {
                let obj = &self.owner;
                (obj
                    .get_label_settings()
                    .map(|settings| settings.get_font_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_color"))
                    .r) as f64
            }
            <LabelFloatKind>::FontColorG => {
                let obj = &self.owner;
                (obj
                    .get_label_settings()
                    .map(|settings| settings.get_font_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_color"))
                    .g) as f64
            }
            <LabelFloatKind>::FontColorB => {
                let obj = &self.owner;
                (obj
                    .get_label_settings()
                    .map(|settings| settings.get_font_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_color"))
                    .b) as f64
            }
            <LabelFloatKind>::FontColorA => {
                let obj = &self.owner;
                (obj
                    .get_label_settings()
                    .map(|settings| settings.get_font_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_color"))
                    .a) as f64
            }
            <LabelFloatKind>::OutlineColorR => {
                let obj = &self.owner;
                (obj
                    .get_label_settings()
                    .map(|settings| settings.get_outline_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_outline_color"))
                    .r) as f64
            }
            <LabelFloatKind>::OutlineColorG => {
                let obj = &self.owner;
                (obj
                    .get_label_settings()
                    .map(|settings| settings.get_outline_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_outline_color"))
                    .g) as f64
            }
            <LabelFloatKind>::OutlineColorB => {
                let obj = &self.owner;
                (obj
                    .get_label_settings()
                    .map(|settings| settings.get_outline_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_outline_color"))
                    .b) as f64
            }
            <LabelFloatKind>::OutlineColorA => {
                let obj = &self.owner;
                (obj
                    .get_label_settings()
                    .map(|settings| settings.get_outline_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_outline_color"))
                    .a) as f64
            }
            <LabelFloatKind>::VisibleRatio => {
                let obj = &self.owner;
                (obj.get_visible_ratio()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <LabelFloatKind>::FontColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj
                    .get_label_settings()
                    .map(|settings| settings.get_font_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_color"));
                prop_val.r = val as f32;
                let val = prop_val;
                if let Some(mut settings) = obj.get_label_settings() {
                    settings.set_font_color(val);
                } else {
                    obj.add_theme_color_override("font_color", val);
                }
            }
            <LabelFloatKind>::FontColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj
                    .get_label_settings()
                    .map(|settings| settings.get_font_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_color"));
                prop_val.g = val as f32;
                let val = prop_val;
                if let Some(mut settings) = obj.get_label_settings() {
                    settings.set_font_color(val);
                } else {
                    obj.add_theme_color_override("font_color", val);
                }
            }
            <LabelFloatKind>::FontColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj
                    .get_label_settings()
                    .map(|settings| settings.get_font_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_color"));
                prop_val.b = val as f32;
                let val = prop_val;
                if let Some(mut settings) = obj.get_label_settings() {
                    settings.set_font_color(val);
                } else {
                    obj.add_theme_color_override("font_color", val);
                }
            }
            <LabelFloatKind>::FontColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj
                    .get_label_settings()
                    .map(|settings| settings.get_font_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_color"));
                prop_val.a = val as f32;
                let val = prop_val;
                if let Some(mut settings) = obj.get_label_settings() {
                    settings.set_font_color(val);
                } else {
                    obj.add_theme_color_override("font_color", val);
                }
            }
            <LabelFloatKind>::OutlineColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj
                    .get_label_settings()
                    .map(|settings| settings.get_outline_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_outline_color"));
                prop_val.r = val as f32;
                let val = prop_val;
                if let Some(mut settings) = obj.get_label_settings() {
                    settings.set_outline_color(val);
                } else {
                    obj.add_theme_color_override("font_outline_color", val);
                }
            }
            <LabelFloatKind>::OutlineColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj
                    .get_label_settings()
                    .map(|settings| settings.get_outline_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_outline_color"));
                prop_val.g = val as f32;
                let val = prop_val;
                if let Some(mut settings) = obj.get_label_settings() {
                    settings.set_outline_color(val);
                } else {
                    obj.add_theme_color_override("font_outline_color", val);
                }
            }
            <LabelFloatKind>::OutlineColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj
                    .get_label_settings()
                    .map(|settings| settings.get_outline_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_outline_color"));
                prop_val.b = val as f32;
                let val = prop_val;
                if let Some(mut settings) = obj.get_label_settings() {
                    settings.set_outline_color(val);
                } else {
                    obj.add_theme_color_override("font_outline_color", val);
                }
            }
            <LabelFloatKind>::OutlineColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj
                    .get_label_settings()
                    .map(|settings| settings.get_outline_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_outline_color"));
                prop_val.a = val as f32;
                let val = prop_val;
                if let Some(mut settings) = obj.get_label_settings() {
                    settings.set_outline_color(val);
                } else {
                    obj.add_theme_color_override("font_outline_color", val);
                }
            }
            <LabelFloatKind>::VisibleRatio => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_visible_ratio(val as f32);
            }
        }
    }
}
impl IPropertyData for LabelFloatData {
    type Target = Label;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LabelFloatKind>::FontColorR => NodePath::from("font_color:r"),
            <LabelFloatKind>::FontColorG => NodePath::from("font_color:g"),
            <LabelFloatKind>::FontColorB => NodePath::from("font_color:b"),
            <LabelFloatKind>::FontColorA => NodePath::from("font_color:a"),
            <LabelFloatKind>::OutlineColorR => NodePath::from("outline_color:r"),
            <LabelFloatKind>::OutlineColorG => NodePath::from("outline_color:g"),
            <LabelFloatKind>::OutlineColorB => NodePath::from("outline_color:b"),
            <LabelFloatKind>::OutlineColorA => NodePath::from("outline_color:a"),
            <LabelFloatKind>::VisibleRatio => NodePath::from("visible_ratio"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for LabelFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label>()
            .ok()
            .and_then(|owner| {
                match path {
                    "font_color:r" => {
                        Some(Self {
                            property: <LabelFloatKind>::FontColorR,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "font_color:g" => {
                        Some(Self {
                            property: <LabelFloatKind>::FontColorG,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "font_color:b" => {
                        Some(Self {
                            property: <LabelFloatKind>::FontColorB,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "font_color:a" => {
                        Some(Self {
                            property: <LabelFloatKind>::FontColorA,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color:r" => {
                        Some(Self {
                            property: <LabelFloatKind>::OutlineColorR,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color:g" => {
                        Some(Self {
                            property: <LabelFloatKind>::OutlineColorG,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color:b" => {
                        Some(Self {
                            property: <LabelFloatKind>::OutlineColorB,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color:a" => {
                        Some(Self {
                            property: <LabelFloatKind>::OutlineColorA,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "visible_ratio" => {
                        Some(Self {
                            property: <LabelFloatKind>::VisibleRatio,
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
pub struct LabelColorData {
    pub property: LabelColorKind,
    pub owner: Gd<Label>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelColorKind {
    FontColor,
    OutlineColor,
}
impl IProperty<Color> for LabelColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <LabelColorKind>::FontColor => {
                let obj = &self.owner;
                obj.get_label_settings()
                    .map(|settings| settings.get_font_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_color"))
            }
            <LabelColorKind>::OutlineColor => {
                let obj = &self.owner;
                obj.get_label_settings()
                    .map(|settings| settings.get_outline_color())
                    .unwrap_or_else(|| obj.get_theme_color("font_outline_color"))
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <LabelColorKind>::FontColor => {
                let obj = &mut self.owner;
                let val = value;
                if let Some(mut settings) = obj.get_label_settings() {
                    settings.set_font_color(val);
                } else {
                    obj.add_theme_color_override("font_color", val);
                }
            }
            <LabelColorKind>::OutlineColor => {
                let obj = &mut self.owner;
                let val = value;
                if let Some(mut settings) = obj.get_label_settings() {
                    settings.set_outline_color(val);
                } else {
                    obj.add_theme_color_override("font_outline_color", val);
                }
            }
        }
    }
}
impl IPropertyData for LabelColorData {
    type Target = Label;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LabelColorKind>::FontColor => NodePath::from("font_color"),
            <LabelColorKind>::OutlineColor => NodePath::from("outline_color"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for LabelColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label>()
            .ok()
            .and_then(|owner| {
                match path {
                    "font_color" => {
                        Some(Self {
                            property: <LabelColorKind>::FontColor,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "outline_color" => {
                        Some(Self {
                            property: <LabelColorKind>::OutlineColor,
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
pub struct LabelStringData {
    pub property: LabelStringKind,
    pub owner: Gd<Label>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelStringKind {
    Text,
}
impl IProperty<GString> for LabelStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <LabelStringKind>::Text => {
                let obj = &self.owner;
                obj.get_text()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <LabelStringKind>::Text => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
        }
    }
}
impl IPropertyData for LabelStringData {
    type Target = Label;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <LabelStringKind>::Text => NodePath::from("text"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for LabelStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label>()
            .ok()
            .and_then(|owner| {
                match path {
                    "text" => {
                        Some(Self {
                            property: <LabelStringKind>::Text,
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
pub trait SpireDoLabel<Marker = ()> {
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
    fn do_max_lines_visible(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>>;
    fn do_visible_characters(
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
    fn do_visible_ratio(
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
    fn do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<Label> + Inherits<Object>> SpireDoLabel<()> for Gd<Class> {
    fn do_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = LabelIntData {
            property: <LabelIntKind>::FontSize,
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
        let data = LabelIntData {
            property: <LabelIntKind>::OutlineSize,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_max_lines_visible(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = LabelIntData {
            property: <LabelIntKind>::MaxLinesVisible,
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
        let data = LabelIntData {
            property: <LabelIntKind>::VisibleCharacters,
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
        let data = LabelFloatData {
            property: <LabelFloatKind>::FontColorR,
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
        let data = LabelFloatData {
            property: <LabelFloatKind>::FontColorG,
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
        let data = LabelFloatData {
            property: <LabelFloatKind>::FontColorB,
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
        let data = LabelFloatData {
            property: <LabelFloatKind>::FontColorA,
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
        let data = LabelFloatData {
            property: <LabelFloatKind>::OutlineColorR,
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
        let data = LabelFloatData {
            property: <LabelFloatKind>::OutlineColorG,
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
        let data = LabelFloatData {
            property: <LabelFloatKind>::OutlineColorB,
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
        let data = LabelFloatData {
            property: <LabelFloatKind>::OutlineColorA,
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
        let data = LabelFloatData {
            property: <LabelFloatKind>::VisibleRatio,
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
        let data = LabelColorData {
            property: <LabelColorKind>::FontColor,
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
        let data = LabelColorData {
            property: <LabelColorKind>::OutlineColor,
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
        let data = LabelStringData {
            property: <LabelStringKind>::Text,
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
impl<T: WithBaseField + Inherits<Label> + Inherits<Object>> SpireDoLabel<BaseMarker>
for T {
    fn do_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelIntData {
            property: <LabelIntKind>::FontSize,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelIntData {
            property: <LabelIntKind>::OutlineSize,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_max_lines_visible(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelIntData {
            property: <LabelIntKind>::MaxLinesVisible,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelIntData {
            property: <LabelIntKind>::VisibleCharacters,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelFloatData {
            property: <LabelFloatKind>::FontColorR,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelFloatData {
            property: <LabelFloatKind>::FontColorG,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelFloatData {
            property: <LabelFloatKind>::FontColorB,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelFloatData {
            property: <LabelFloatKind>::FontColorA,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelFloatData {
            property: <LabelFloatKind>::OutlineColorR,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelFloatData {
            property: <LabelFloatKind>::OutlineColorG,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelFloatData {
            property: <LabelFloatKind>::OutlineColorB,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelFloatData {
            property: <LabelFloatKind>::OutlineColorA,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelFloatData {
            property: <LabelFloatKind>::VisibleRatio,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelColorData {
            property: <LabelColorKind>::FontColor,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelColorData {
            property: <LabelColorKind>::OutlineColor,
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
        let owner: Gd<Label> = self.to_gd().upcast();
        let data = LabelStringData {
            property: <LabelStringKind>::Text,
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
