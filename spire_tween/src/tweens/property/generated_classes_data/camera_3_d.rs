use super::*;
#[derive(Debug, Clone)]
pub struct Camera3DFloatData {
    pub property: Camera3DFloatKind,
    pub owner: Gd<Camera3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Camera3DFloatKind {
    Fov,
    Size,
    FrustumOffsetX,
    FrustumOffsetY,
    HOffset,
    VOffset,
}
impl IProperty<f64> for Camera3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <Camera3DFloatKind>::Fov => {
                let obj = &self.owner;
                (obj.get_fov()) as f64
            }
            <Camera3DFloatKind>::Size => {
                let obj = &self.owner;
                (obj.get_size()) as f64
            }
            <Camera3DFloatKind>::FrustumOffsetX => {
                let obj = &self.owner;
                (obj.get_frustum_offset().x) as f64
            }
            <Camera3DFloatKind>::FrustumOffsetY => {
                let obj = &self.owner;
                (obj.get_frustum_offset().y) as f64
            }
            <Camera3DFloatKind>::HOffset => {
                let obj = &self.owner;
                (obj.get_h_offset()) as f64
            }
            <Camera3DFloatKind>::VOffset => {
                let obj = &self.owner;
                (obj.get_v_offset()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <Camera3DFloatKind>::Fov => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_fov(val as f32);
            }
            <Camera3DFloatKind>::Size => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_size(val as f32);
            }
            <Camera3DFloatKind>::FrustumOffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_frustum_offset();
                prop_val.x = val as f32;
                obj.set_frustum_offset(prop_val);
            }
            <Camera3DFloatKind>::FrustumOffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_frustum_offset();
                prop_val.y = val as f32;
                obj.set_frustum_offset(prop_val);
            }
            <Camera3DFloatKind>::HOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_h_offset(val as f32);
            }
            <Camera3DFloatKind>::VOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_v_offset(val as f32);
            }
        }
    }
}
impl IPropertyData for Camera3DFloatData {
    type Target = Camera3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Camera3DFloatKind>::Fov => NodePath::from("fov"),
            <Camera3DFloatKind>::Size => NodePath::from("size"),
            <Camera3DFloatKind>::FrustumOffsetX => NodePath::from("frustum_offset:x"),
            <Camera3DFloatKind>::FrustumOffsetY => NodePath::from("frustum_offset:y"),
            <Camera3DFloatKind>::HOffset => NodePath::from("h_offset"),
            <Camera3DFloatKind>::VOffset => NodePath::from("v_offset"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Camera3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Camera3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Camera3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Camera3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "fov" => {
                        Some(Self {
                            property: <Camera3DFloatKind>::Fov,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size" => {
                        Some(Self {
                            property: <Camera3DFloatKind>::Size,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "frustum_offset:x" => {
                        Some(Self {
                            property: <Camera3DFloatKind>::FrustumOffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "frustum_offset:y" => {
                        Some(Self {
                            property: <Camera3DFloatKind>::FrustumOffsetY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "h_offset" => {
                        Some(Self {
                            property: <Camera3DFloatKind>::HOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "v_offset" => {
                        Some(Self {
                            property: <Camera3DFloatKind>::VOffset,
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
pub struct Camera3DVector2Data {
    pub property: Camera3DVector2Kind,
    pub owner: Gd<Camera3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Camera3DVector2Kind {
    FrustumOffset,
}
impl IProperty<Vector2> for Camera3DVector2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <Camera3DVector2Kind>::FrustumOffset => {
                let obj = &self.owner;
                obj.get_frustum_offset()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <Camera3DVector2Kind>::FrustumOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_frustum_offset(val);
            }
        }
    }
}
impl IPropertyData for Camera3DVector2Data {
    type Target = Camera3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Camera3DVector2Kind>::FrustumOffset => NodePath::from("frustum_offset"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Camera3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Camera3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for Camera3DVector2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Camera3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "frustum_offset" => {
                        Some(Self {
                            property: <Camera3DVector2Kind>::FrustumOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoCamera3D<Marker = ()> {
    fn do_fov(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>>;
    fn do_size(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>>;
    fn do_frustum_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_frustum_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_h_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_v_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_frustum_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<Class: Inherits<Camera3D> + Inherits<Object>> SpireDoCamera3D<()> for Gd<Class> {
    fn do_fov(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera3DFloatData {
            property: <Camera3DFloatKind>::Fov,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera3DFloatData {
            property: <Camera3DFloatKind>::Size,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_frustum_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera3DFloatData {
            property: <Camera3DFloatKind>::FrustumOffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_frustum_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera3DFloatData {
            property: <Camera3DFloatKind>::FrustumOffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_h_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera3DFloatData {
            property: <Camera3DFloatKind>::HOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_v_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Camera3DFloatData {
            property: <Camera3DFloatKind>::VOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_frustum_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = Camera3DVector2Data {
            property: <Camera3DVector2Kind>::FrustumOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Camera3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<Camera3D> + Inherits<Object>,
> SpireDoCamera3D<BaseMarker> for T {
    fn do_fov(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera3D> = self.to_gd().upcast();
        let data = Camera3DFloatData {
            property: <Camera3DFloatKind>::Fov,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera3D> = self.to_gd().upcast();
        let data = Camera3DFloatData {
            property: <Camera3DFloatKind>::Size,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_frustum_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera3D> = self.to_gd().upcast();
        let data = Camera3DFloatData {
            property: <Camera3DFloatKind>::FrustumOffsetX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_frustum_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera3D> = self.to_gd().upcast();
        let data = Camera3DFloatData {
            property: <Camera3DFloatKind>::FrustumOffsetY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_h_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera3D> = self.to_gd().upcast();
        let data = Camera3DFloatData {
            property: <Camera3DFloatKind>::HOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_v_offset(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Camera3D> = self.to_gd().upcast();
        let data = Camera3DFloatData {
            property: <Camera3DFloatKind>::VOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_frustum_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<Camera3D> = self.to_gd().upcast();
        let data = Camera3DVector2Data {
            property: <Camera3DVector2Kind>::FrustumOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
