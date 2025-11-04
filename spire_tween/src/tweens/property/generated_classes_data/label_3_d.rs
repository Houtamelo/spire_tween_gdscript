use super::*;
#[derive(Debug, Clone)]
pub struct Label3DIntData {
    pub property: Label3DIntKind,
    pub owner: Gd<Label3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Label3DIntKind {
    FontSize,
    OutlineSize,
}
impl IProperty<i64> for Label3DIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <Label3DIntKind>::FontSize => {
                let obj = &self.owner;
                (obj.get_font_size()) as i64
            }
            <Label3DIntKind>::OutlineSize => {
                let obj = &self.owner;
                (obj.get_outline_size()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <Label3DIntKind>::FontSize => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_font_size(val as i32);
            }
            <Label3DIntKind>::OutlineSize => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_outline_size(val as i32);
            }
        }
    }
}
impl IPropertyData for Label3DIntData {
    type Target = Label3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Label3DIntKind>::FontSize => NodePath::from("font_size"),
            <Label3DIntKind>::OutlineSize => NodePath::from("outline_size"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Label3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Label3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Label3DIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "font_size" => {
                        Some(Self {
                            property: <Label3DIntKind>::FontSize,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "outline_size" => {
                        Some(Self {
                            property: <Label3DIntKind>::OutlineSize,
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
pub struct Label3DFloatData {
    pub property: Label3DFloatKind,
    pub owner: Gd<Label3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Label3DFloatKind {
    LineSpacing,
    ModulateR,
    ModulateG,
    ModulateB,
    ModulateA,
    OffsetX,
    OffsetY,
    OutlineModulateR,
    OutlineModulateG,
    OutlineModulateB,
    OutlineModulateA,
    PixelSize,
}
impl IProperty<f64> for Label3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <Label3DFloatKind>::LineSpacing => {
                let obj = &self.owner;
                (obj.get_line_spacing()) as f64
            }
            <Label3DFloatKind>::ModulateR => {
                let obj = &self.owner;
                (obj.get_modulate().r) as f64
            }
            <Label3DFloatKind>::ModulateG => {
                let obj = &self.owner;
                (obj.get_modulate().g) as f64
            }
            <Label3DFloatKind>::ModulateB => {
                let obj = &self.owner;
                (obj.get_modulate().b) as f64
            }
            <Label3DFloatKind>::ModulateA => {
                let obj = &self.owner;
                (obj.get_modulate().a) as f64
            }
            <Label3DFloatKind>::OffsetX => {
                let obj = &self.owner;
                (obj.get_offset().x) as f64
            }
            <Label3DFloatKind>::OffsetY => {
                let obj = &self.owner;
                (obj.get_offset().y) as f64
            }
            <Label3DFloatKind>::OutlineModulateR => {
                let obj = &self.owner;
                (obj.get_outline_modulate().r) as f64
            }
            <Label3DFloatKind>::OutlineModulateG => {
                let obj = &self.owner;
                (obj.get_outline_modulate().g) as f64
            }
            <Label3DFloatKind>::OutlineModulateB => {
                let obj = &self.owner;
                (obj.get_outline_modulate().b) as f64
            }
            <Label3DFloatKind>::OutlineModulateA => {
                let obj = &self.owner;
                (obj.get_outline_modulate().a) as f64
            }
            <Label3DFloatKind>::PixelSize => {
                let obj = &self.owner;
                (obj.get_pixel_size()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <Label3DFloatKind>::LineSpacing => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_line_spacing(val as f32);
            }
            <Label3DFloatKind>::ModulateR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.r = val as f32;
                obj.set_modulate(prop_val);
            }
            <Label3DFloatKind>::ModulateG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.g = val as f32;
                obj.set_modulate(prop_val);
            }
            <Label3DFloatKind>::ModulateB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.b = val as f32;
                obj.set_modulate(prop_val);
            }
            <Label3DFloatKind>::ModulateA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.a = val as f32;
                obj.set_modulate(prop_val);
            }
            <Label3DFloatKind>::OffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_offset();
                prop_val.x = val as f32;
                obj.set_offset(prop_val);
            }
            <Label3DFloatKind>::OffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_offset();
                prop_val.y = val as f32;
                obj.set_offset(prop_val);
            }
            <Label3DFloatKind>::OutlineModulateR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_outline_modulate();
                prop_val.r = val as f32;
                obj.set_outline_modulate(prop_val);
            }
            <Label3DFloatKind>::OutlineModulateG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_outline_modulate();
                prop_val.g = val as f32;
                obj.set_outline_modulate(prop_val);
            }
            <Label3DFloatKind>::OutlineModulateB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_outline_modulate();
                prop_val.b = val as f32;
                obj.set_outline_modulate(prop_val);
            }
            <Label3DFloatKind>::OutlineModulateA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_outline_modulate();
                prop_val.a = val as f32;
                obj.set_outline_modulate(prop_val);
            }
            <Label3DFloatKind>::PixelSize => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_pixel_size(val as f32);
            }
        }
    }
}
impl IPropertyData for Label3DFloatData {
    type Target = Label3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Label3DFloatKind>::LineSpacing => NodePath::from("line_spacing"),
            <Label3DFloatKind>::ModulateR => NodePath::from("modulate:r"),
            <Label3DFloatKind>::ModulateG => NodePath::from("modulate:g"),
            <Label3DFloatKind>::ModulateB => NodePath::from("modulate:b"),
            <Label3DFloatKind>::ModulateA => NodePath::from("modulate:a"),
            <Label3DFloatKind>::OffsetX => NodePath::from("offset:x"),
            <Label3DFloatKind>::OffsetY => NodePath::from("offset:y"),
            <Label3DFloatKind>::OutlineModulateR => NodePath::from("outline_modulate:r"),
            <Label3DFloatKind>::OutlineModulateG => NodePath::from("outline_modulate:g"),
            <Label3DFloatKind>::OutlineModulateB => NodePath::from("outline_modulate:b"),
            <Label3DFloatKind>::OutlineModulateA => NodePath::from("outline_modulate:a"),
            <Label3DFloatKind>::PixelSize => NodePath::from("pixel_size"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Label3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Label3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Label3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "line_spacing" => {
                        Some(Self {
                            property: <Label3DFloatKind>::LineSpacing,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:r" => {
                        Some(Self {
                            property: <Label3DFloatKind>::ModulateR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:g" => {
                        Some(Self {
                            property: <Label3DFloatKind>::ModulateG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:b" => {
                        Some(Self {
                            property: <Label3DFloatKind>::ModulateB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:a" => {
                        Some(Self {
                            property: <Label3DFloatKind>::ModulateA,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "offset:x" => {
                        Some(Self {
                            property: <Label3DFloatKind>::OffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "offset:y" => {
                        Some(Self {
                            property: <Label3DFloatKind>::OffsetY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "outline_modulate:r" => {
                        Some(Self {
                            property: <Label3DFloatKind>::OutlineModulateR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "outline_modulate:g" => {
                        Some(Self {
                            property: <Label3DFloatKind>::OutlineModulateG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "outline_modulate:b" => {
                        Some(Self {
                            property: <Label3DFloatKind>::OutlineModulateB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "outline_modulate:a" => {
                        Some(Self {
                            property: <Label3DFloatKind>::OutlineModulateA,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "pixel_size" => {
                        Some(Self {
                            property: <Label3DFloatKind>::PixelSize,
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
pub struct Label3DVector2Data {
    pub property: Label3DVector2Kind,
    pub owner: Gd<Label3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Label3DVector2Kind {
    Offset,
}
impl IProperty<Vector2> for Label3DVector2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <Label3DVector2Kind>::Offset => {
                let obj = &self.owner;
                obj.get_offset()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <Label3DVector2Kind>::Offset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_offset(val);
            }
        }
    }
}
impl IPropertyData for Label3DVector2Data {
    type Target = Label3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Label3DVector2Kind>::Offset => NodePath::from("offset"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Label3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Label3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Label3DVector2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "offset" => {
                        Some(Self {
                            property: <Label3DVector2Kind>::Offset,
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
pub struct Label3DColorData {
    pub property: Label3DColorKind,
    pub owner: Gd<Label3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Label3DColorKind {
    Modulate,
    OutlineModulate,
}
impl IProperty<Color> for Label3DColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <Label3DColorKind>::Modulate => {
                let obj = &self.owner;
                obj.get_modulate()
            }
            <Label3DColorKind>::OutlineModulate => {
                let obj = &self.owner;
                obj.get_outline_modulate()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <Label3DColorKind>::Modulate => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_modulate(val);
            }
            <Label3DColorKind>::OutlineModulate => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_outline_modulate(val);
            }
        }
    }
}
impl IPropertyData for Label3DColorData {
    type Target = Label3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Label3DColorKind>::Modulate => NodePath::from("modulate"),
            <Label3DColorKind>::OutlineModulate => NodePath::from("outline_modulate"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Label3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Label3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Label3DColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "modulate" => {
                        Some(Self {
                            property: <Label3DColorKind>::Modulate,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "outline_modulate" => {
                        Some(Self {
                            property: <Label3DColorKind>::OutlineModulate,
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
pub struct Label3DStringData {
    pub property: Label3DStringKind,
    pub owner: Gd<Label3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Label3DStringKind {
    Text,
}
impl IProperty<GString> for Label3DStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <Label3DStringKind>::Text => {
                let obj = &self.owner;
                obj.get_text()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <Label3DStringKind>::Text => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
        }
    }
}
impl IPropertyData for Label3DStringData {
    type Target = Label3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Label3DStringKind>::Text => NodePath::from("text"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Label3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Label3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Label3DStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Label3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "text" => {
                        Some(Self {
                            property: <Label3DStringKind>::Text,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoLabel3D<Marker = ()> {
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
    fn do_line_spacing(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
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
    fn do_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_outline_modulate_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_outline_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_outline_modulate_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_outline_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_outline_modulate_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_outline_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_outline_modulate_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_outline_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_pixel_size(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
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
    fn do_outline_modulate(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
    fn do_outline_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
    fn do_label3d_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<Label3D> + Inherits<Object>> SpireDoLabel3D<()> for Gd<Class> {
    fn do_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = Label3DIntData {
            property: <Label3DIntKind>::FontSize,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DIntData {
            property: <Label3DIntKind>::OutlineSize,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_line_spacing(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::LineSpacing,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_modulate_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_modulate_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_modulate_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_modulate_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_pixel_size(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::PixelSize,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Label3DVector2Data {
            property: <Label3DVector2Kind>::Offset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = Label3DColorData {
            property: <Label3DColorKind>::Modulate,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DColorData {
            property: <Label3DColorKind>::Modulate,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_modulate(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = Label3DColorData {
            property: <Label3DColorKind>::OutlineModulate,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
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
        let data = Label3DColorData {
            property: <Label3DColorKind>::OutlineModulate,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_label3d_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = Label3DStringData {
            property: <Label3DStringKind>::Text,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Label3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<T: WithBaseField + Inherits<Label3D> + Inherits<Object>> SpireDoLabel3D<BaseMarker>
for T {
    fn do_font_size(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DIntData {
            property: <Label3DIntKind>::FontSize,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DIntData {
            property: <Label3DIntKind>::OutlineSize,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_line_spacing(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::LineSpacing,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateR,
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateR,
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateG,
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateG,
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateB,
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateB,
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateA,
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::ModulateA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OffsetX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OffsetY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_modulate_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_modulate_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_modulate_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_modulate_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::OutlineModulateA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_pixel_size(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DFloatData {
            property: <Label3DFloatKind>::PixelSize,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DVector2Data {
            property: <Label3DVector2Kind>::Offset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DColorData {
            property: <Label3DColorKind>::Modulate,
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DColorData {
            property: <Label3DColorKind>::Modulate,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_outline_modulate(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DColorData {
            property: <Label3DColorKind>::OutlineModulate,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
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
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DColorData {
            property: <Label3DColorKind>::OutlineModulate,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_label3d_do_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<Label3D> = self.to_gd().upcast();
        let data = Label3DStringData {
            property: <Label3DStringKind>::Text,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
