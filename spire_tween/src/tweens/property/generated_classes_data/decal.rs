use super::*;
#[derive(Debug, Clone)]
pub struct DecalFloatData {
    pub property: DecalFloatKind,
    pub owner: Gd<Decal>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecalFloatKind {
    AlbedoMix,
    EmissionEnergy,
    ModulateR,
    ModulateG,
    ModulateB,
    ModulateA,
    SizeX,
    SizeY,
    SizeZ,
}
impl IProperty<f64> for DecalFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <DecalFloatKind>::AlbedoMix => {
                let obj = &self.owner;
                (obj.get_albedo_mix()) as f64
            }
            <DecalFloatKind>::EmissionEnergy => {
                let obj = &self.owner;
                (obj.get_emission_energy()) as f64
            }
            <DecalFloatKind>::ModulateR => {
                let obj = &self.owner;
                (obj.get_modulate().r) as f64
            }
            <DecalFloatKind>::ModulateG => {
                let obj = &self.owner;
                (obj.get_modulate().g) as f64
            }
            <DecalFloatKind>::ModulateB => {
                let obj = &self.owner;
                (obj.get_modulate().b) as f64
            }
            <DecalFloatKind>::ModulateA => {
                let obj = &self.owner;
                (obj.get_modulate().a) as f64
            }
            <DecalFloatKind>::SizeX => {
                let obj = &self.owner;
                (obj.get_size().x) as f64
            }
            <DecalFloatKind>::SizeY => {
                let obj = &self.owner;
                (obj.get_size().y) as f64
            }
            <DecalFloatKind>::SizeZ => {
                let obj = &self.owner;
                (obj.get_size().z) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <DecalFloatKind>::AlbedoMix => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_albedo_mix(val as f32);
            }
            <DecalFloatKind>::EmissionEnergy => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_emission_energy(val as f32);
            }
            <DecalFloatKind>::ModulateR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.r = val as f32;
                obj.set_modulate(prop_val);
            }
            <DecalFloatKind>::ModulateG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.g = val as f32;
                obj.set_modulate(prop_val);
            }
            <DecalFloatKind>::ModulateB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.b = val as f32;
                obj.set_modulate(prop_val);
            }
            <DecalFloatKind>::ModulateA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_modulate();
                prop_val.a = val as f32;
                obj.set_modulate(prop_val);
            }
            <DecalFloatKind>::SizeX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.x = val as f32;
                obj.set_size(prop_val);
            }
            <DecalFloatKind>::SizeY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.y = val as f32;
                obj.set_size(prop_val);
            }
            <DecalFloatKind>::SizeZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.z = val as f32;
                obj.set_size(prop_val);
            }
        }
    }
}
impl IPropertyData for DecalFloatData {
    type Target = Decal;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <DecalFloatKind>::AlbedoMix => NodePath::from("albedo_mix"),
            <DecalFloatKind>::EmissionEnergy => NodePath::from("emission_energy"),
            <DecalFloatKind>::ModulateR => NodePath::from("modulate:r"),
            <DecalFloatKind>::ModulateG => NodePath::from("modulate:g"),
            <DecalFloatKind>::ModulateB => NodePath::from("modulate:b"),
            <DecalFloatKind>::ModulateA => NodePath::from("modulate:a"),
            <DecalFloatKind>::SizeX => NodePath::from("size:x"),
            <DecalFloatKind>::SizeY => NodePath::from("size:y"),
            <DecalFloatKind>::SizeZ => NodePath::from("size:z"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Decal>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Decal>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for DecalFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Decal>()
            .ok()
            .and_then(|owner| {
                match path {
                    "albedo_mix" => {
                        Some(Self {
                            property: <DecalFloatKind>::AlbedoMix,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "emission_energy" => {
                        Some(Self {
                            property: <DecalFloatKind>::EmissionEnergy,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:r" => {
                        Some(Self {
                            property: <DecalFloatKind>::ModulateR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:g" => {
                        Some(Self {
                            property: <DecalFloatKind>::ModulateG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:b" => {
                        Some(Self {
                            property: <DecalFloatKind>::ModulateB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "modulate:a" => {
                        Some(Self {
                            property: <DecalFloatKind>::ModulateA,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:x" => {
                        Some(Self {
                            property: <DecalFloatKind>::SizeX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:y" => {
                        Some(Self {
                            property: <DecalFloatKind>::SizeY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "size:z" => {
                        Some(Self {
                            property: <DecalFloatKind>::SizeZ,
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
pub struct DecalVector3Data {
    pub property: DecalVector3Kind,
    pub owner: Gd<Decal>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecalVector3Kind {
    Size,
}
impl IProperty<Vector3> for DecalVector3Data {
    #[inline]
    fn get_property_value(&self) -> Vector3 {
        match self.property {
            <DecalVector3Kind>::Size => {
                let obj = &self.owner;
                obj.get_size()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector3) {
        match self.property {
            <DecalVector3Kind>::Size => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_size(val);
            }
        }
    }
}
impl IPropertyData for DecalVector3Data {
    type Target = Decal;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <DecalVector3Kind>::Size => NodePath::from("size"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Decal>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Decal>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for DecalVector3Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Decal>()
            .ok()
            .and_then(|owner| {
                match path {
                    "size" => {
                        Some(Self {
                            property: <DecalVector3Kind>::Size,
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
pub struct DecalColorData {
    pub property: DecalColorKind,
    pub owner: Gd<Decal>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecalColorKind {
    Modulate,
}
impl IProperty<Color> for DecalColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <DecalColorKind>::Modulate => {
                let obj = &self.owner;
                obj.get_modulate()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <DecalColorKind>::Modulate => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_modulate(val);
            }
        }
    }
}
impl IPropertyData for DecalColorData {
    type Target = Decal;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <DecalColorKind>::Modulate => NodePath::from("modulate"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Decal>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Decal>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for DecalColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Decal>()
            .ok()
            .and_then(|owner| {
                match path {
                    "modulate" => {
                        Some(Self {
                            property: <DecalColorKind>::Modulate,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoDecal<Marker = ()> {
    fn do_albedo_mix(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_emission_energy(
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
    fn do_size(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
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
impl<Class: Inherits<Decal> + Inherits<Object>> SpireDoDecal<()> for Gd<Class> {
    fn do_albedo_mix(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = DecalFloatData {
            property: <DecalFloatKind>::AlbedoMix,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_emission_energy(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = DecalFloatData {
            property: <DecalFloatKind>::EmissionEnergy,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
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
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
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
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
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
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
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
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
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
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
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
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
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
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
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
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
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
        let data = DecalFloatData {
            property: <DecalFloatKind>::SizeX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
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
        let data = DecalFloatData {
            property: <DecalFloatKind>::SizeY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
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
        let data = DecalFloatData {
            property: <DecalFloatKind>::SizeZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = DecalVector3Data {
            property: <DecalVector3Kind>::Size,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = DecalColorData {
            property: <DecalColorKind>::Modulate,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
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
        let data = DecalColorData {
            property: <DecalColorKind>::Modulate,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Decal>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<T: WithBaseField + Inherits<Decal> + Inherits<Object>> SpireDoDecal<BaseMarker>
for T {
    fn do_albedo_mix(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalFloatData {
            property: <DecalFloatKind>::AlbedoMix,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_emission_energy(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalFloatData {
            property: <DecalFloatKind>::EmissionEnergy,
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
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateR,
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
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateR,
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
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateG,
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
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateG,
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
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateB,
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
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateB,
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
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateA,
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
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalFloatData {
            property: <DecalFloatKind>::ModulateA,
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
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalFloatData {
            property: <DecalFloatKind>::SizeX,
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
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalFloatData {
            property: <DecalFloatKind>::SizeY,
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
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalFloatData {
            property: <DecalFloatKind>::SizeZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_size(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalVector3Data {
            property: <DecalVector3Kind>::Size,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_modulate(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalColorData {
            property: <DecalColorKind>::Modulate,
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
        let owner: Gd<Decal> = self.to_gd().upcast();
        let data = DecalColorData {
            property: <DecalColorKind>::Modulate,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
