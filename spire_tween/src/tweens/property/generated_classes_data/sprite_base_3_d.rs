use super::*;
#[derive(Debug, Clone)]
pub struct SpriteBase3DFloatData {
    pub property: SpriteBase3DFloatKind,
    pub owner: Gd<SpriteBase3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpriteBase3DFloatKind {
    ModulateR,
    ModulateG,
    ModulateB,
    ModulateA,
    OffsetX,
    OffsetY,
    PixelSize,
}
impl IProperty<f64> for SpriteBase3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <SpriteBase3DFloatKind>::ModulateR => {
                let obj = &self.owner;
                (obj.get_modulate().r) as f64
            }
            <SpriteBase3DFloatKind>::ModulateG => {
                let obj = &self.owner;
                (obj.get_modulate().g) as f64
            }
            <SpriteBase3DFloatKind>::ModulateB => {
                let obj = &self.owner;
                (obj.get_modulate().b) as f64
            }
            <SpriteBase3DFloatKind>::ModulateA => {
                let obj = &self.owner;
                (obj.get_modulate().a) as f64
            }
            <SpriteBase3DFloatKind>::OffsetX => {
                let obj = &self.owner;
                (obj.get_offset().x) as f64
            }
            <SpriteBase3DFloatKind>::OffsetY => {
                let obj = &self.owner;
                (obj.get_offset().y) as f64
            }
            <SpriteBase3DFloatKind>::PixelSize => {
                let obj = &self.owner;
                (obj.get_pixel_size()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <SpriteBase3DFloatKind>::ModulateR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.r = val as f32;
                obj.set_modulate(prop_val);
            }
            <SpriteBase3DFloatKind>::ModulateG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.g = val as f32;
                obj.set_modulate(prop_val);
            }
            <SpriteBase3DFloatKind>::ModulateB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.b = val as f32;
                obj.set_modulate(prop_val);
            }
            <SpriteBase3DFloatKind>::ModulateA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.a = val as f32;
                obj.set_modulate(prop_val);
            }
            <SpriteBase3DFloatKind>::OffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_offset();
                prop_val.x = val as f32;
                obj.set_offset(prop_val);
            }
            <SpriteBase3DFloatKind>::OffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_offset();
                prop_val.y = val as f32;
                obj.set_offset(prop_val);
            }
            <SpriteBase3DFloatKind>::PixelSize => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_pixel_size(val as f32);
            }
        }
    }
}
impl IPropertyData for SpriteBase3DFloatData {
    type Target = SpriteBase3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <SpriteBase3DFloatKind>::ModulateR => NodePath::from("modulate:r"),
            <SpriteBase3DFloatKind>::ModulateG => NodePath::from("modulate:g"),
            <SpriteBase3DFloatKind>::ModulateB => NodePath::from("modulate:b"),
            <SpriteBase3DFloatKind>::ModulateA => NodePath::from("modulate:a"),
            <SpriteBase3DFloatKind>::OffsetX => NodePath::from("offset:x"),
            <SpriteBase3DFloatKind>::OffsetY => NodePath::from("offset:y"),
            <SpriteBase3DFloatKind>::PixelSize => NodePath::from("pixel_size"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<SpriteBase3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<SpriteBase3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for SpriteBase3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<SpriteBase3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "modulate:r" => {
                        Some(Self {
                            property: <SpriteBase3DFloatKind>::ModulateR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:g" => {
                        Some(Self {
                            property: <SpriteBase3DFloatKind>::ModulateG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:b" => {
                        Some(Self {
                            property: <SpriteBase3DFloatKind>::ModulateB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:a" => {
                        Some(Self {
                            property: <SpriteBase3DFloatKind>::ModulateA,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "offset:x" => {
                        Some(Self {
                            property: <SpriteBase3DFloatKind>::OffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "offset:y" => {
                        Some(Self {
                            property: <SpriteBase3DFloatKind>::OffsetY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "pixel_size" => {
                        Some(Self {
                            property: <SpriteBase3DFloatKind>::PixelSize,
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
pub struct SpriteBase3DVector2Data {
    pub property: SpriteBase3DVector2Kind,
    pub owner: Gd<SpriteBase3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpriteBase3DVector2Kind {
    Offset,
}
impl IProperty<Vector2> for SpriteBase3DVector2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <SpriteBase3DVector2Kind>::Offset => {
                let obj = &self.owner;
                obj.get_offset()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <SpriteBase3DVector2Kind>::Offset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_offset(val);
            }
        }
    }
}
impl IPropertyData for SpriteBase3DVector2Data {
    type Target = SpriteBase3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <SpriteBase3DVector2Kind>::Offset => NodePath::from("offset"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<SpriteBase3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<SpriteBase3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for SpriteBase3DVector2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<SpriteBase3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "offset" => {
                        Some(Self {
                            property: <SpriteBase3DVector2Kind>::Offset,
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
pub struct SpriteBase3DColorData {
    pub property: SpriteBase3DColorKind,
    pub owner: Gd<SpriteBase3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpriteBase3DColorKind {
    Modulate,
}
impl IProperty<Color> for SpriteBase3DColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <SpriteBase3DColorKind>::Modulate => {
                let obj = &self.owner;
                obj.get_modulate()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <SpriteBase3DColorKind>::Modulate => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_modulate(val);
            }
        }
    }
}
impl IPropertyData for SpriteBase3DColorData {
    type Target = SpriteBase3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <SpriteBase3DColorKind>::Modulate => NodePath::from("modulate"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<SpriteBase3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<SpriteBase3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for SpriteBase3DColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<SpriteBase3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "modulate" => {
                        Some(Self {
                            property: <SpriteBase3DColorKind>::Modulate,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoSpriteBase3D<Marker = ()> {
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
}
impl<Class: Inherits<SpriteBase3D> + Inherits<Object>> SpireDoSpriteBase3D<()>
for Gd<Class> {
    fn do_modulate_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
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
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
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
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
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
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
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
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
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
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
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
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
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
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
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
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::OffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
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
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::OffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
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
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::PixelSize,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
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
        let data = SpriteBase3DVector2Data {
            property: <SpriteBase3DVector2Kind>::Offset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
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
        let data = SpriteBase3DColorData {
            property: <SpriteBase3DColorKind>::Modulate,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
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
        let data = SpriteBase3DColorData {
            property: <SpriteBase3DColorKind>::Modulate,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SpriteBase3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<SpriteBase3D> + Inherits<Object>,
> SpireDoSpriteBase3D<BaseMarker> for T {
    fn do_modulate_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateR,
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
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateR,
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
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateG,
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
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateG,
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
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateB,
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
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateB,
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
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateA,
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
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::ModulateA,
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
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::OffsetX,
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
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::OffsetY,
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
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DFloatData {
            property: <SpriteBase3DFloatKind>::PixelSize,
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
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DVector2Data {
            property: <SpriteBase3DVector2Kind>::Offset,
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
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DColorData {
            property: <SpriteBase3DColorKind>::Modulate,
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
        let owner: Gd<SpriteBase3D> = self.to_gd().upcast();
        let data = SpriteBase3DColorData {
            property: <SpriteBase3DColorKind>::Modulate,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
