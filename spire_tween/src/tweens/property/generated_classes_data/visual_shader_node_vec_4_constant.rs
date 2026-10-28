use super::*;
#[derive(Debug, Clone)]
pub struct VisualShaderNodeVec4ConstantFloatData {
    pub property: VisualShaderNodeVec4ConstantFloatKind,
    pub owner: Gd<VisualShaderNodeVec4Constant>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualShaderNodeVec4ConstantFloatKind {
    ConstantX,
    ConstantY,
    ConstantZ,
    ConstantW,
}
impl IProperty<f64> for VisualShaderNodeVec4ConstantFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <VisualShaderNodeVec4ConstantFloatKind>::ConstantX => {
                let obj = &self.owner;
                ({
                    let v = obj.get_constant();
                    Vector4::new(v.x, v.y, v.z, v.w)
                }
                    .x) as f64
            }
            <VisualShaderNodeVec4ConstantFloatKind>::ConstantY => {
                let obj = &self.owner;
                ({
                    let v = obj.get_constant();
                    Vector4::new(v.x, v.y, v.z, v.w)
                }
                    .y) as f64
            }
            <VisualShaderNodeVec4ConstantFloatKind>::ConstantZ => {
                let obj = &self.owner;
                ({
                    let v = obj.get_constant();
                    Vector4::new(v.x, v.y, v.z, v.w)
                }
                    .z) as f64
            }
            <VisualShaderNodeVec4ConstantFloatKind>::ConstantW => {
                let obj = &self.owner;
                ({
                    let v = obj.get_constant();
                    Vector4::new(v.x, v.y, v.z, v.w)
                }
                    .w) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <VisualShaderNodeVec4ConstantFloatKind>::ConstantX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = {
                    let v = obj.get_constant();
                    Vector4::new(v.x, v.y, v.z, v.w)
                };
                prop_val.x = val as f32;
                let val = prop_val;
                obj.set_constant(Quaternion::new(val.x, val.y, val.z, val.w));
            }
            <VisualShaderNodeVec4ConstantFloatKind>::ConstantY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = {
                    let v = obj.get_constant();
                    Vector4::new(v.x, v.y, v.z, v.w)
                };
                prop_val.y = val as f32;
                let val = prop_val;
                obj.set_constant(Quaternion::new(val.x, val.y, val.z, val.w));
            }
            <VisualShaderNodeVec4ConstantFloatKind>::ConstantZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = {
                    let v = obj.get_constant();
                    Vector4::new(v.x, v.y, v.z, v.w)
                };
                prop_val.z = val as f32;
                let val = prop_val;
                obj.set_constant(Quaternion::new(val.x, val.y, val.z, val.w));
            }
            <VisualShaderNodeVec4ConstantFloatKind>::ConstantW => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = {
                    let v = obj.get_constant();
                    Vector4::new(v.x, v.y, v.z, v.w)
                };
                prop_val.w = val as f32;
                let val = prop_val;
                obj.set_constant(Quaternion::new(val.x, val.y, val.z, val.w));
            }
        }
    }
}
impl IPropertyData for VisualShaderNodeVec4ConstantFloatData {
    type Target = VisualShaderNodeVec4Constant;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <VisualShaderNodeVec4ConstantFloatKind>::ConstantX => {
                NodePath::from("constant:x")
            }
            <VisualShaderNodeVec4ConstantFloatKind>::ConstantY => {
                NodePath::from("constant:y")
            }
            <VisualShaderNodeVec4ConstantFloatKind>::ConstantZ => {
                NodePath::from("constant:z")
            }
            <VisualShaderNodeVec4ConstantFloatKind>::ConstantW => {
                NodePath::from("constant:w")
            }
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for VisualShaderNodeVec4ConstantFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<VisualShaderNodeVec4Constant>()
            .ok()
            .and_then(|owner| {
                match path {
                    "constant:x" => {
                        Some(Self {
                            property: <VisualShaderNodeVec4ConstantFloatKind>::ConstantX,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "constant:y" => {
                        Some(Self {
                            property: <VisualShaderNodeVec4ConstantFloatKind>::ConstantY,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "constant:z" => {
                        Some(Self {
                            property: <VisualShaderNodeVec4ConstantFloatKind>::ConstantZ,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "constant:w" => {
                        Some(Self {
                            property: <VisualShaderNodeVec4ConstantFloatKind>::ConstantW,
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
pub struct VisualShaderNodeVec4ConstantVec4Data {
    pub property: VisualShaderNodeVec4ConstantVec4Kind,
    pub owner: Gd<VisualShaderNodeVec4Constant>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualShaderNodeVec4ConstantVec4Kind {
    Constant,
}
impl IProperty<Vector4> for VisualShaderNodeVec4ConstantVec4Data {
    #[inline]
    fn get_property_value(&self) -> Vector4 {
        match self.property {
            <VisualShaderNodeVec4ConstantVec4Kind>::Constant => {
                let obj = &self.owner;
                {
                    let v = obj.get_constant();
                    Vector4::new(v.x, v.y, v.z, v.w)
                }
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector4) {
        match self.property {
            <VisualShaderNodeVec4ConstantVec4Kind>::Constant => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_constant(Quaternion::new(val.x, val.y, val.z, val.w));
            }
        }
    }
}
impl IPropertyData for VisualShaderNodeVec4ConstantVec4Data {
    type Target = VisualShaderNodeVec4Constant;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <VisualShaderNodeVec4ConstantVec4Kind>::Constant => {
                NodePath::from("constant")
            }
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for VisualShaderNodeVec4ConstantVec4Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<VisualShaderNodeVec4Constant>()
            .ok()
            .and_then(|owner| {
                match path {
                    "constant" => {
                        Some(Self {
                            property: <VisualShaderNodeVec4ConstantVec4Kind>::Constant,
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
pub trait SpireDoVisualShaderNodeVec4Constant<Marker = ()> {
    fn do_constant_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant_w(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_constant(
        &self,
        end_val: Vector4,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector4>>;
}
impl<
    Class: Inherits<VisualShaderNodeVec4Constant> + Inherits<Object>,
> SpireDoVisualShaderNodeVec4Constant<()> for Gd<Class> {
    fn do_constant_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = VisualShaderNodeVec4ConstantFloatData {
            property: <VisualShaderNodeVec4ConstantFloatKind>::ConstantX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = VisualShaderNodeVec4ConstantFloatData {
            property: <VisualShaderNodeVec4ConstantFloatKind>::ConstantY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = VisualShaderNodeVec4ConstantFloatData {
            property: <VisualShaderNodeVec4ConstantFloatKind>::ConstantZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_w(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = VisualShaderNodeVec4ConstantFloatData {
            property: <VisualShaderNodeVec4ConstantFloatKind>::ConstantW,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant(
        &self,
        end_val: Vector4,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector4>> {
        let data = VisualShaderNodeVec4ConstantVec4Data {
            property: <VisualShaderNodeVec4ConstantVec4Kind>::Constant,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector4>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<VisualShaderNodeVec4Constant> + Inherits<Object>,
> SpireDoVisualShaderNodeVec4Constant<BaseMarker> for T {
    fn do_constant_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<VisualShaderNodeVec4Constant> = self.to_gd().upcast();
        let data = VisualShaderNodeVec4ConstantFloatData {
            property: <VisualShaderNodeVec4ConstantFloatKind>::ConstantX,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<VisualShaderNodeVec4Constant> = self.to_gd().upcast();
        let data = VisualShaderNodeVec4ConstantFloatData {
            property: <VisualShaderNodeVec4ConstantFloatKind>::ConstantY,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<VisualShaderNodeVec4Constant> = self.to_gd().upcast();
        let data = VisualShaderNodeVec4ConstantFloatData {
            property: <VisualShaderNodeVec4ConstantFloatKind>::ConstantZ,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant_w(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<VisualShaderNodeVec4Constant> = self.to_gd().upcast();
        let data = VisualShaderNodeVec4ConstantFloatData {
            property: <VisualShaderNodeVec4ConstantFloatKind>::ConstantW,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_constant(
        &self,
        end_val: Vector4,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector4>> {
        let owner: Gd<VisualShaderNodeVec4Constant> = self.to_gd().upcast();
        let data = VisualShaderNodeVec4ConstantVec4Data {
            property: <VisualShaderNodeVec4ConstantVec4Kind>::Constant,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector4>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
