use super::*;
#[derive(Debug, Clone)]
pub struct BaseMaterial3DFloatData {
    pub property: BaseMaterial3DFloatKind,
    pub owner: Gd<BaseMaterial3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseMaterial3DFloatKind {
    AlbedoColorR,
    AlbedoColorG,
    AlbedoColorB,
    AlbedoColorA,
    Uv1OffsetX,
    Uv1OffsetY,
    Uv1OffsetZ,
    Uv1ScaleX,
    Uv1ScaleY,
    Uv1ScaleZ,
    Uv2OffsetX,
    Uv2OffsetY,
    Uv2OffsetZ,
    Uv2ScaleX,
    Uv2ScaleY,
    Uv2ScaleZ,
}
impl IProperty<f64> for BaseMaterial3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <BaseMaterial3DFloatKind>::AlbedoColorR => {
                let obj = &self.owner;
                (obj.get_albedo().r) as f64
            }
            <BaseMaterial3DFloatKind>::AlbedoColorG => {
                let obj = &self.owner;
                (obj.get_albedo().g) as f64
            }
            <BaseMaterial3DFloatKind>::AlbedoColorB => {
                let obj = &self.owner;
                (obj.get_albedo().b) as f64
            }
            <BaseMaterial3DFloatKind>::AlbedoColorA => {
                let obj = &self.owner;
                (obj.get_albedo().a) as f64
            }
            <BaseMaterial3DFloatKind>::Uv1OffsetX => {
                let obj = &self.owner;
                (obj.get_uv1_offset().x) as f64
            }
            <BaseMaterial3DFloatKind>::Uv1OffsetY => {
                let obj = &self.owner;
                (obj.get_uv1_offset().y) as f64
            }
            <BaseMaterial3DFloatKind>::Uv1OffsetZ => {
                let obj = &self.owner;
                (obj.get_uv1_offset().z) as f64
            }
            <BaseMaterial3DFloatKind>::Uv1ScaleX => {
                let obj = &self.owner;
                (obj.get_uv1_scale().x) as f64
            }
            <BaseMaterial3DFloatKind>::Uv1ScaleY => {
                let obj = &self.owner;
                (obj.get_uv1_scale().y) as f64
            }
            <BaseMaterial3DFloatKind>::Uv1ScaleZ => {
                let obj = &self.owner;
                (obj.get_uv1_scale().z) as f64
            }
            <BaseMaterial3DFloatKind>::Uv2OffsetX => {
                let obj = &self.owner;
                (obj.get_uv2_offset().x) as f64
            }
            <BaseMaterial3DFloatKind>::Uv2OffsetY => {
                let obj = &self.owner;
                (obj.get_uv2_offset().y) as f64
            }
            <BaseMaterial3DFloatKind>::Uv2OffsetZ => {
                let obj = &self.owner;
                (obj.get_uv2_offset().z) as f64
            }
            <BaseMaterial3DFloatKind>::Uv2ScaleX => {
                let obj = &self.owner;
                (obj.get_uv2_scale().x) as f64
            }
            <BaseMaterial3DFloatKind>::Uv2ScaleY => {
                let obj = &self.owner;
                (obj.get_uv2_scale().y) as f64
            }
            <BaseMaterial3DFloatKind>::Uv2ScaleZ => {
                let obj = &self.owner;
                (obj.get_uv2_scale().z) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <BaseMaterial3DFloatKind>::AlbedoColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_albedo();
                prop_val.r = val as f32;
                let val = prop_val;
                obj.set_albedo(val);
            }
            <BaseMaterial3DFloatKind>::AlbedoColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_albedo();
                prop_val.g = val as f32;
                let val = prop_val;
                obj.set_albedo(val);
            }
            <BaseMaterial3DFloatKind>::AlbedoColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_albedo();
                prop_val.b = val as f32;
                let val = prop_val;
                obj.set_albedo(val);
            }
            <BaseMaterial3DFloatKind>::AlbedoColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_albedo();
                prop_val.a = val as f32;
                let val = prop_val;
                obj.set_albedo(val);
            }
            <BaseMaterial3DFloatKind>::Uv1OffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_uv1_offset();
                prop_val.x = val as f32;
                obj.set_uv1_offset(prop_val);
            }
            <BaseMaterial3DFloatKind>::Uv1OffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_uv1_offset();
                prop_val.y = val as f32;
                obj.set_uv1_offset(prop_val);
            }
            <BaseMaterial3DFloatKind>::Uv1OffsetZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_uv1_offset();
                prop_val.z = val as f32;
                obj.set_uv1_offset(prop_val);
            }
            <BaseMaterial3DFloatKind>::Uv1ScaleX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_uv1_scale();
                prop_val.x = val as f32;
                obj.set_uv1_scale(prop_val);
            }
            <BaseMaterial3DFloatKind>::Uv1ScaleY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_uv1_scale();
                prop_val.y = val as f32;
                obj.set_uv1_scale(prop_val);
            }
            <BaseMaterial3DFloatKind>::Uv1ScaleZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_uv1_scale();
                prop_val.z = val as f32;
                obj.set_uv1_scale(prop_val);
            }
            <BaseMaterial3DFloatKind>::Uv2OffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_uv2_offset();
                prop_val.x = val as f32;
                obj.set_uv2_offset(prop_val);
            }
            <BaseMaterial3DFloatKind>::Uv2OffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_uv2_offset();
                prop_val.y = val as f32;
                obj.set_uv2_offset(prop_val);
            }
            <BaseMaterial3DFloatKind>::Uv2OffsetZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_uv2_offset();
                prop_val.z = val as f32;
                obj.set_uv2_offset(prop_val);
            }
            <BaseMaterial3DFloatKind>::Uv2ScaleX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_uv2_scale();
                prop_val.x = val as f32;
                obj.set_uv2_scale(prop_val);
            }
            <BaseMaterial3DFloatKind>::Uv2ScaleY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_uv2_scale();
                prop_val.y = val as f32;
                obj.set_uv2_scale(prop_val);
            }
            <BaseMaterial3DFloatKind>::Uv2ScaleZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_uv2_scale();
                prop_val.z = val as f32;
                obj.set_uv2_scale(prop_val);
            }
        }
    }
}
impl IPropertyData for BaseMaterial3DFloatData {
    type Target = BaseMaterial3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <BaseMaterial3DFloatKind>::AlbedoColorR => NodePath::from("albedo_color:r"),
            <BaseMaterial3DFloatKind>::AlbedoColorG => NodePath::from("albedo_color:g"),
            <BaseMaterial3DFloatKind>::AlbedoColorB => NodePath::from("albedo_color:b"),
            <BaseMaterial3DFloatKind>::AlbedoColorA => NodePath::from("albedo_color:a"),
            <BaseMaterial3DFloatKind>::Uv1OffsetX => NodePath::from("uv1_offset:x"),
            <BaseMaterial3DFloatKind>::Uv1OffsetY => NodePath::from("uv1_offset:y"),
            <BaseMaterial3DFloatKind>::Uv1OffsetZ => NodePath::from("uv1_offset:z"),
            <BaseMaterial3DFloatKind>::Uv1ScaleX => NodePath::from("uv1_scale:x"),
            <BaseMaterial3DFloatKind>::Uv1ScaleY => NodePath::from("uv1_scale:y"),
            <BaseMaterial3DFloatKind>::Uv1ScaleZ => NodePath::from("uv1_scale:z"),
            <BaseMaterial3DFloatKind>::Uv2OffsetX => NodePath::from("uv2_offset:x"),
            <BaseMaterial3DFloatKind>::Uv2OffsetY => NodePath::from("uv2_offset:y"),
            <BaseMaterial3DFloatKind>::Uv2OffsetZ => NodePath::from("uv2_offset:z"),
            <BaseMaterial3DFloatKind>::Uv2ScaleX => NodePath::from("uv2_scale:x"),
            <BaseMaterial3DFloatKind>::Uv2ScaleY => NodePath::from("uv2_scale:y"),
            <BaseMaterial3DFloatKind>::Uv2ScaleZ => NodePath::from("uv2_scale:z"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for BaseMaterial3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<BaseMaterial3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "albedo_color:r" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::AlbedoColorR,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "albedo_color:g" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::AlbedoColorG,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "albedo_color:b" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::AlbedoColorB,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "albedo_color:a" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::AlbedoColorA,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv1_offset:x" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::Uv1OffsetX,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv1_offset:y" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::Uv1OffsetY,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv1_offset:z" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::Uv1OffsetZ,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv1_scale:x" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::Uv1ScaleX,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv1_scale:y" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::Uv1ScaleY,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv1_scale:z" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::Uv1ScaleZ,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv2_offset:x" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::Uv2OffsetX,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv2_offset:y" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::Uv2OffsetY,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv2_offset:z" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::Uv2OffsetZ,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv2_scale:x" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::Uv2ScaleX,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv2_scale:y" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::Uv2ScaleY,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv2_scale:z" => {
                        Some(Self {
                            property: <BaseMaterial3DFloatKind>::Uv2ScaleZ,
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
pub struct BaseMaterial3DVec3Data {
    pub property: BaseMaterial3DVec3Kind,
    pub owner: Gd<BaseMaterial3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseMaterial3DVec3Kind {
    Uv1Offset,
    Uv1Scale,
    Uv2Offset,
    Uv2Scale,
}
impl IProperty<Vector3> for BaseMaterial3DVec3Data {
    #[inline]
    fn get_property_value(&self) -> Vector3 {
        match self.property {
            <BaseMaterial3DVec3Kind>::Uv1Offset => {
                let obj = &self.owner;
                obj.get_uv1_offset()
            }
            <BaseMaterial3DVec3Kind>::Uv1Scale => {
                let obj = &self.owner;
                obj.get_uv1_scale()
            }
            <BaseMaterial3DVec3Kind>::Uv2Offset => {
                let obj = &self.owner;
                obj.get_uv2_offset()
            }
            <BaseMaterial3DVec3Kind>::Uv2Scale => {
                let obj = &self.owner;
                obj.get_uv2_scale()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector3) {
        match self.property {
            <BaseMaterial3DVec3Kind>::Uv1Offset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_uv1_offset(val);
            }
            <BaseMaterial3DVec3Kind>::Uv1Scale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_uv1_scale(val);
            }
            <BaseMaterial3DVec3Kind>::Uv2Offset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_uv2_offset(val);
            }
            <BaseMaterial3DVec3Kind>::Uv2Scale => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_uv2_scale(val);
            }
        }
    }
}
impl IPropertyData for BaseMaterial3DVec3Data {
    type Target = BaseMaterial3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <BaseMaterial3DVec3Kind>::Uv1Offset => NodePath::from("uv1_offset"),
            <BaseMaterial3DVec3Kind>::Uv1Scale => NodePath::from("uv1_scale"),
            <BaseMaterial3DVec3Kind>::Uv2Offset => NodePath::from("uv2_offset"),
            <BaseMaterial3DVec3Kind>::Uv2Scale => NodePath::from("uv2_scale"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for BaseMaterial3DVec3Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<BaseMaterial3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "uv1_offset" => {
                        Some(Self {
                            property: <BaseMaterial3DVec3Kind>::Uv1Offset,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv1_scale" => {
                        Some(Self {
                            property: <BaseMaterial3DVec3Kind>::Uv1Scale,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv2_offset" => {
                        Some(Self {
                            property: <BaseMaterial3DVec3Kind>::Uv2Offset,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "uv2_scale" => {
                        Some(Self {
                            property: <BaseMaterial3DVec3Kind>::Uv2Scale,
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
pub struct BaseMaterial3DColorData {
    pub property: BaseMaterial3DColorKind,
    pub owner: Gd<BaseMaterial3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseMaterial3DColorKind {
    AlbedoColor,
}
impl IProperty<Color> for BaseMaterial3DColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <BaseMaterial3DColorKind>::AlbedoColor => {
                let obj = &self.owner;
                obj.get_albedo()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <BaseMaterial3DColorKind>::AlbedoColor => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_albedo(val);
            }
        }
    }
}
impl IPropertyData for BaseMaterial3DColorData {
    type Target = BaseMaterial3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <BaseMaterial3DColorKind>::AlbedoColor => NodePath::from("albedo_color"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for BaseMaterial3DColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<BaseMaterial3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "albedo_color" => {
                        Some(Self {
                            property: <BaseMaterial3DColorKind>::AlbedoColor,
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
pub trait SpireDoBaseMaterial3D<Marker = ()> {
    fn do_albedo_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_albedo_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_albedo_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_albedo_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_fade(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>>;
    fn do_uv1_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_uv1_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_uv1_offset_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_uv1_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_uv1_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_uv1_scale_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_uv2_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_uv2_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_uv2_offset_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_uv2_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_uv2_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_uv2_scale_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_uv1_offset(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_uv1_scale(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_uv2_offset(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_uv2_scale(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_albedo_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
}
impl<Class: Inherits<BaseMaterial3D> + Inherits<Object>> SpireDoBaseMaterial3D<()>
for Gd<Class> {
    fn do_albedo_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::AlbedoColorR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_albedo_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::AlbedoColorG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_albedo_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::AlbedoColorB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_albedo_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::AlbedoColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_fade(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::AlbedoColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv1OffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv1OffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_offset_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv1OffsetZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv1ScaleX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv1ScaleY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_scale_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv1ScaleZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv2OffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv2OffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_offset_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv2OffsetZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv2ScaleX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv2ScaleY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_scale_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv2ScaleZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_offset(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = BaseMaterial3DVec3Data {
            property: <BaseMaterial3DVec3Kind>::Uv1Offset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_scale(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = BaseMaterial3DVec3Data {
            property: <BaseMaterial3DVec3Kind>::Uv1Scale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_offset(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = BaseMaterial3DVec3Data {
            property: <BaseMaterial3DVec3Kind>::Uv2Offset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_scale(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = BaseMaterial3DVec3Data {
            property: <BaseMaterial3DVec3Kind>::Uv2Scale,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_albedo_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = BaseMaterial3DColorData {
            property: <BaseMaterial3DColorKind>::AlbedoColor,
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
    T: WithBaseField + Inherits<BaseMaterial3D> + Inherits<Object>,
> SpireDoBaseMaterial3D<BaseMarker> for T {
    fn do_albedo_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::AlbedoColorR,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_albedo_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::AlbedoColorG,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_albedo_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::AlbedoColorB,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_albedo_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::AlbedoColorA,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_fade(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::AlbedoColorA,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv1OffsetX,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv1OffsetY,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_offset_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv1OffsetZ,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv1ScaleX,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv1ScaleY,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_scale_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv1ScaleZ,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv2OffsetX,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv2OffsetY,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_offset_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv2OffsetZ,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_scale_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv2ScaleX,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_scale_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv2ScaleY,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_scale_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DFloatData {
            property: <BaseMaterial3DFloatKind>::Uv2ScaleZ,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_offset(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DVec3Data {
            property: <BaseMaterial3DVec3Kind>::Uv1Offset,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv1_scale(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DVec3Data {
            property: <BaseMaterial3DVec3Kind>::Uv1Scale,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_offset(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DVec3Data {
            property: <BaseMaterial3DVec3Kind>::Uv2Offset,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_uv2_scale(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DVec3Data {
            property: <BaseMaterial3DVec3Kind>::Uv2Scale,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_albedo_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<BaseMaterial3D> = self.to_gd().upcast();
        let data = BaseMaterial3DColorData {
            property: <BaseMaterial3DColorKind>::AlbedoColor,
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
