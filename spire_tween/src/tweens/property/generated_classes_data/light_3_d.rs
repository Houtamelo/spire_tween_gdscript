use super::*;
#[derive(Debug, Clone)]
pub struct Light3DFloatData {
    pub property: Light3DFloatKind,
    pub owner: Gd<Light3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light3DFloatKind {
    LightEnergy,
    ShadowOpacity,
    LightColorR,
    LightColorG,
    LightColorB,
    LightColorA,
    LightTemperature,
}
impl IProperty<f64> for Light3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <Light3DFloatKind>::LightEnergy => {
                let obj = &self.owner;
                (obj.get_param(godot::classes::light_3d::Param::ENERGY)) as f64
            }
            <Light3DFloatKind>::ShadowOpacity => {
                let obj = &self.owner;
                (obj.get_param(godot::classes::light_3d::Param::SHADOW_OPACITY)) as f64
            }
            <Light3DFloatKind>::LightColorR => {
                let obj = &self.owner;
                (obj.get_color().r) as f64
            }
            <Light3DFloatKind>::LightColorG => {
                let obj = &self.owner;
                (obj.get_color().g) as f64
            }
            <Light3DFloatKind>::LightColorB => {
                let obj = &self.owner;
                (obj.get_color().b) as f64
            }
            <Light3DFloatKind>::LightColorA => {
                let obj = &self.owner;
                (obj.get_color().a) as f64
            }
            <Light3DFloatKind>::LightTemperature => {
                let obj = &self.owner;
                (obj.get_temperature()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <Light3DFloatKind>::LightEnergy => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_param(godot::classes::light_3d::Param::ENERGY, val as f32);
            }
            <Light3DFloatKind>::ShadowOpacity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_param(
                    godot::classes::light_3d::Param::SHADOW_OPACITY,
                    val as f32,
                );
            }
            <Light3DFloatKind>::LightColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.r = val as f32;
                let val = prop_val;
                obj.set_color(val);
            }
            <Light3DFloatKind>::LightColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.g = val as f32;
                let val = prop_val;
                obj.set_color(val);
            }
            <Light3DFloatKind>::LightColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.b = val as f32;
                let val = prop_val;
                obj.set_color(val);
            }
            <Light3DFloatKind>::LightColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.a = val as f32;
                let val = prop_val;
                obj.set_color(val);
            }
            <Light3DFloatKind>::LightTemperature => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_temperature(val as f32);
            }
        }
    }
}
impl IPropertyData for Light3DFloatData {
    type Target = Light3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Light3DFloatKind>::LightEnergy => NodePath::from("light_energy"),
            <Light3DFloatKind>::ShadowOpacity => NodePath::from("shadow_opacity"),
            <Light3DFloatKind>::LightColorR => NodePath::from("light_color:r"),
            <Light3DFloatKind>::LightColorG => NodePath::from("light_color:g"),
            <Light3DFloatKind>::LightColorB => NodePath::from("light_color:b"),
            <Light3DFloatKind>::LightColorA => NodePath::from("light_color:a"),
            <Light3DFloatKind>::LightTemperature => NodePath::from("light_temperature"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for Light3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Light3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "light_energy" => {
                        Some(Self {
                            property: <Light3DFloatKind>::LightEnergy,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "shadow_opacity" => {
                        Some(Self {
                            property: <Light3DFloatKind>::ShadowOpacity,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "light_color:r" => {
                        Some(Self {
                            property: <Light3DFloatKind>::LightColorR,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "light_color:g" => {
                        Some(Self {
                            property: <Light3DFloatKind>::LightColorG,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "light_color:b" => {
                        Some(Self {
                            property: <Light3DFloatKind>::LightColorB,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "light_color:a" => {
                        Some(Self {
                            property: <Light3DFloatKind>::LightColorA,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "light_temperature" => {
                        Some(Self {
                            property: <Light3DFloatKind>::LightTemperature,
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
pub struct Light3DColorData {
    pub property: Light3DColorKind,
    pub owner: Gd<Light3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light3DColorKind {
    LightColor,
}
impl IProperty<Color> for Light3DColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <Light3DColorKind>::LightColor => {
                let obj = &self.owner;
                obj.get_color()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <Light3DColorKind>::LightColor => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_color(val);
            }
        }
    }
}
impl IPropertyData for Light3DColorData {
    type Target = Light3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Light3DColorKind>::LightColor => NodePath::from("light_color"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for Light3DColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Light3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "light_color" => {
                        Some(Self {
                            property: <Light3DColorKind>::LightColor,
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
pub trait SpireDoLight3D<Marker = ()> {
    fn do_light_energy(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_shadow_opacity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_light_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_light_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_light_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_light_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_light_temperature(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_light_color(
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
impl<Class: Inherits<Light3D> + Inherits<Object>> SpireDoLight3D<()> for Gd<Class> {
    fn do_light_energy(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightEnergy,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_shadow_opacity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::ShadowOpacity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_light_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
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
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_light_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
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
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_light_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
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
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_light_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
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
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_light_temperature(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightTemperature,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_light_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = Light3DColorData {
            property: <Light3DColorKind>::LightColor,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
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
        let data = Light3DColorData {
            property: <Light3DColorKind>::LightColor,
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
impl<T: WithBaseField + Inherits<Light3D> + Inherits<Object>> SpireDoLight3D<BaseMarker>
for T {
    fn do_light_energy(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Light3D> = self.to_gd().upcast();
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightEnergy,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_shadow_opacity(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Light3D> = self.to_gd().upcast();
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::ShadowOpacity,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_light_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Light3D> = self.to_gd().upcast();
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorR,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
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
        let owner: Gd<Light3D> = self.to_gd().upcast();
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorR,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_light_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Light3D> = self.to_gd().upcast();
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorG,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
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
        let owner: Gd<Light3D> = self.to_gd().upcast();
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorG,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_light_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Light3D> = self.to_gd().upcast();
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorB,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
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
        let owner: Gd<Light3D> = self.to_gd().upcast();
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorB,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_light_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Light3D> = self.to_gd().upcast();
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorA,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
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
        let owner: Gd<Light3D> = self.to_gd().upcast();
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightColorA,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_light_temperature(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Light3D> = self.to_gd().upcast();
        let data = Light3DFloatData {
            property: <Light3DFloatKind>::LightTemperature,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_light_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<Light3D> = self.to_gd().upcast();
        let data = Light3DColorData {
            property: <Light3DColorKind>::LightColor,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
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
        let owner: Gd<Light3D> = self.to_gd().upcast();
        let data = Light3DColorData {
            property: <Light3DColorKind>::LightColor,
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
