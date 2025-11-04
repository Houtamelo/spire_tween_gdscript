use super::*;
#[derive(Debug, Clone)]
pub struct ReflectionProbeFloatData {
    pub property: ReflectionProbeFloatKind,
    pub owner: Gd<ReflectionProbe>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionProbeFloatKind {
    AmbientColorR,
    AmbientColorG,
    AmbientColorB,
    AmbientColorA,
    AmbientColorEnergy,
    BlendDistance,
    Intensity,
    MaxDistance,
    OriginOffsetX,
    OriginOffsetY,
    OriginOffsetZ,
    SizeX,
    SizeY,
    SizeZ,
}
impl IProperty<f64> for ReflectionProbeFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <ReflectionProbeFloatKind>::AmbientColorR => {
                let obj = &self.owner;
                (obj.get_ambient_color().r) as f64
            }
            <ReflectionProbeFloatKind>::AmbientColorG => {
                let obj = &self.owner;
                (obj.get_ambient_color().g) as f64
            }
            <ReflectionProbeFloatKind>::AmbientColorB => {
                let obj = &self.owner;
                (obj.get_ambient_color().b) as f64
            }
            <ReflectionProbeFloatKind>::AmbientColorA => {
                let obj = &self.owner;
                (obj.get_ambient_color().a) as f64
            }
            <ReflectionProbeFloatKind>::AmbientColorEnergy => {
                let obj = &self.owner;
                (obj.get_ambient_color_energy()) as f64
            }
            <ReflectionProbeFloatKind>::BlendDistance => {
                let obj = &self.owner;
                (obj.get_blend_distance()) as f64
            }
            <ReflectionProbeFloatKind>::Intensity => {
                let obj = &self.owner;
                (obj.get_intensity()) as f64
            }
            <ReflectionProbeFloatKind>::MaxDistance => {
                let obj = &self.owner;
                (obj.get_max_distance()) as f64
            }
            <ReflectionProbeFloatKind>::OriginOffsetX => {
                let obj = &self.owner;
                (obj.get_origin_offset().x) as f64
            }
            <ReflectionProbeFloatKind>::OriginOffsetY => {
                let obj = &self.owner;
                (obj.get_origin_offset().y) as f64
            }
            <ReflectionProbeFloatKind>::OriginOffsetZ => {
                let obj = &self.owner;
                (obj.get_origin_offset().z) as f64
            }
            <ReflectionProbeFloatKind>::SizeX => {
                let obj = &self.owner;
                (obj.get_size().x) as f64
            }
            <ReflectionProbeFloatKind>::SizeY => {
                let obj = &self.owner;
                (obj.get_size().y) as f64
            }
            <ReflectionProbeFloatKind>::SizeZ => {
                let obj = &self.owner;
                (obj.get_size().z) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <ReflectionProbeFloatKind>::AmbientColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_ambient_color();
                prop_val.r = val as f32;
                obj.set_ambient_color(prop_val);
            }
            <ReflectionProbeFloatKind>::AmbientColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_ambient_color();
                prop_val.g = val as f32;
                obj.set_ambient_color(prop_val);
            }
            <ReflectionProbeFloatKind>::AmbientColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_ambient_color();
                prop_val.b = val as f32;
                obj.set_ambient_color(prop_val);
            }
            <ReflectionProbeFloatKind>::AmbientColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_ambient_color();
                prop_val.a = val as f32;
                obj.set_ambient_color(prop_val);
            }
            <ReflectionProbeFloatKind>::AmbientColorEnergy => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_ambient_color_energy(val as f32);
            }
            <ReflectionProbeFloatKind>::BlendDistance => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_blend_distance(val as f32);
            }
            <ReflectionProbeFloatKind>::Intensity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_intensity(val as f32);
            }
            <ReflectionProbeFloatKind>::MaxDistance => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_max_distance(val as f32);
            }
            <ReflectionProbeFloatKind>::OriginOffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_origin_offset();
                prop_val.x = val as f32;
                obj.set_origin_offset(prop_val);
            }
            <ReflectionProbeFloatKind>::OriginOffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_origin_offset();
                prop_val.y = val as f32;
                obj.set_origin_offset(prop_val);
            }
            <ReflectionProbeFloatKind>::OriginOffsetZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_origin_offset();
                prop_val.z = val as f32;
                obj.set_origin_offset(prop_val);
            }
            <ReflectionProbeFloatKind>::SizeX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.x = val as f32;
                obj.set_size(prop_val);
            }
            <ReflectionProbeFloatKind>::SizeY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.y = val as f32;
                obj.set_size(prop_val);
            }
            <ReflectionProbeFloatKind>::SizeZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.z = val as f32;
                obj.set_size(prop_val);
            }
        }
    }
}
impl IPropertyData for ReflectionProbeFloatData {
    type Target = ReflectionProbe;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ReflectionProbeFloatKind>::AmbientColorR => {
                NodePath::from("ambient_color:r")
            }
            <ReflectionProbeFloatKind>::AmbientColorG => {
                NodePath::from("ambient_color:g")
            }
            <ReflectionProbeFloatKind>::AmbientColorB => {
                NodePath::from("ambient_color:b")
            }
            <ReflectionProbeFloatKind>::AmbientColorA => {
                NodePath::from("ambient_color:a")
            }
            <ReflectionProbeFloatKind>::AmbientColorEnergy => {
                NodePath::from("ambient_color_energy")
            }
            <ReflectionProbeFloatKind>::BlendDistance => NodePath::from("blend_distance"),
            <ReflectionProbeFloatKind>::Intensity => NodePath::from("intensity"),
            <ReflectionProbeFloatKind>::MaxDistance => NodePath::from("max_distance"),
            <ReflectionProbeFloatKind>::OriginOffsetX => {
                NodePath::from("origin_offset:x")
            }
            <ReflectionProbeFloatKind>::OriginOffsetY => {
                NodePath::from("origin_offset:y")
            }
            <ReflectionProbeFloatKind>::OriginOffsetZ => {
                NodePath::from("origin_offset:z")
            }
            <ReflectionProbeFloatKind>::SizeX => NodePath::from("size:x"),
            <ReflectionProbeFloatKind>::SizeY => NodePath::from("size:y"),
            <ReflectionProbeFloatKind>::SizeZ => NodePath::from("size:z"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<ReflectionProbe>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<ReflectionProbe>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ReflectionProbeFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<ReflectionProbe>()
            .ok()
            .and_then(|owner| {
                match path {
                    "ambient_color:r" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::AmbientColorR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "ambient_color:g" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::AmbientColorG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "ambient_color:b" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::AmbientColorB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "ambient_color:a" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::AmbientColorA,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "ambient_color_energy" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::AmbientColorEnergy,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "blend_distance" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::BlendDistance,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "intensity" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::Intensity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "max_distance" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::MaxDistance,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "origin_offset:x" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::OriginOffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "origin_offset:y" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::OriginOffsetY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "origin_offset:z" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::OriginOffsetZ,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:x" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::SizeX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:y" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::SizeY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:z" => {
                        Some(Self {
                            property: <ReflectionProbeFloatKind>::SizeZ,
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
pub struct ReflectionProbeVector3Data {
    pub property: ReflectionProbeVector3Kind,
    pub owner: Gd<ReflectionProbe>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionProbeVector3Kind {
    OriginOffset,
    Size,
}
impl IProperty<Vector3> for ReflectionProbeVector3Data {
    #[inline]
    fn get_property_value(&self) -> Vector3 {
        match self.property {
            <ReflectionProbeVector3Kind>::OriginOffset => {
                let obj = &self.owner;
                obj.get_origin_offset()
            }
            <ReflectionProbeVector3Kind>::Size => {
                let obj = &self.owner;
                obj.get_size()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector3) {
        match self.property {
            <ReflectionProbeVector3Kind>::OriginOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_origin_offset(val);
            }
            <ReflectionProbeVector3Kind>::Size => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_size(val);
            }
        }
    }
}
impl IPropertyData for ReflectionProbeVector3Data {
    type Target = ReflectionProbe;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ReflectionProbeVector3Kind>::OriginOffset => NodePath::from("origin_offset"),
            <ReflectionProbeVector3Kind>::Size => NodePath::from("size"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<ReflectionProbe>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<ReflectionProbe>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ReflectionProbeVector3Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<ReflectionProbe>()
            .ok()
            .and_then(|owner| {
                match path {
                    "origin_offset" => {
                        Some(Self {
                            property: <ReflectionProbeVector3Kind>::OriginOffset,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size" => {
                        Some(Self {
                            property: <ReflectionProbeVector3Kind>::Size,
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
pub struct ReflectionProbeColorData {
    pub property: ReflectionProbeColorKind,
    pub owner: Gd<ReflectionProbe>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionProbeColorKind {
    AmbientColor,
}
impl IProperty<Color> for ReflectionProbeColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <ReflectionProbeColorKind>::AmbientColor => {
                let obj = &self.owner;
                obj.get_ambient_color()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <ReflectionProbeColorKind>::AmbientColor => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_ambient_color(val);
            }
        }
    }
}
impl IPropertyData for ReflectionProbeColorData {
    type Target = ReflectionProbe;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ReflectionProbeColorKind>::AmbientColor => NodePath::from("ambient_color"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<ReflectionProbe>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<ReflectionProbe>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ReflectionProbeColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<ReflectionProbe>()
            .ok()
            .and_then(|owner| {
                match path {
                    "ambient_color" => {
                        Some(Self {
                            property: <ReflectionProbeColorKind>::AmbientColor,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoReflectionProbe<Marker = ()> {
    fn do_ambient_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_ambient_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_ambient_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_ambient_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_ambient_color_energy(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_blend_distance(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_intensity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_max_distance(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_origin_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_origin_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_origin_offset_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_size_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_size_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_origin_offset(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_size(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
    fn do_ambient_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
}
impl<Class: Inherits<ReflectionProbe> + Inherits<Object>> SpireDoReflectionProbe<()>
for Gd<Class> {
    fn do_ambient_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::AmbientColorR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ambient_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::AmbientColorG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ambient_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::AmbientColorB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ambient_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::AmbientColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ambient_color_energy(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::AmbientColorEnergy,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_blend_distance(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::BlendDistance,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_intensity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::Intensity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_max_distance(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::MaxDistance,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_origin_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::OriginOffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_origin_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::OriginOffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_origin_offset_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::OriginOffsetZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::SizeX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::SizeY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::SizeZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_origin_offset(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = ReflectionProbeVector3Data {
            property: <ReflectionProbeVector3Kind>::OriginOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = ReflectionProbeVector3Data {
            property: <ReflectionProbeVector3Kind>::Size,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ambient_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = ReflectionProbeColorData {
            property: <ReflectionProbeColorKind>::AmbientColor,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ReflectionProbe>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<ReflectionProbe> + Inherits<Object>,
> SpireDoReflectionProbe<BaseMarker> for T {
    fn do_ambient_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::AmbientColorR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ambient_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::AmbientColorG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ambient_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::AmbientColorB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ambient_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::AmbientColorA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ambient_color_energy(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::AmbientColorEnergy,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_blend_distance(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::BlendDistance,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_intensity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::Intensity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_max_distance(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::MaxDistance,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_origin_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::OriginOffsetX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_origin_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::OriginOffsetY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_origin_offset_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::OriginOffsetZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::SizeX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::SizeY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeFloatData {
            property: <ReflectionProbeFloatKind>::SizeZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_origin_offset(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeVector3Data {
            property: <ReflectionProbeVector3Kind>::OriginOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeVector3Data {
            property: <ReflectionProbeVector3Kind>::Size,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ambient_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<ReflectionProbe> = self.to_gd().upcast();
        let data = ReflectionProbeColorData {
            property: <ReflectionProbeColorKind>::AmbientColor,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
