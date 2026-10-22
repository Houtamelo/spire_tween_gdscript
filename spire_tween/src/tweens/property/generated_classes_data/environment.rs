use super::*;
#[derive(Debug, Clone)]
pub struct EnvironmentFloatData {
    pub property: EnvironmentFloatKind,
    pub owner: Gd<Environment>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentFloatKind {
    BackgroundColorR,
    BackgroundColorG,
    BackgroundColorB,
    BackgroundColorA,
}
impl IProperty<f64> for EnvironmentFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <EnvironmentFloatKind>::BackgroundColorR => {
                let obj = &self.owner;
                (obj.get_bg_color().r) as f64
            }
            <EnvironmentFloatKind>::BackgroundColorG => {
                let obj = &self.owner;
                (obj.get_bg_color().g) as f64
            }
            <EnvironmentFloatKind>::BackgroundColorB => {
                let obj = &self.owner;
                (obj.get_bg_color().b) as f64
            }
            <EnvironmentFloatKind>::BackgroundColorA => {
                let obj = &self.owner;
                (obj.get_bg_color().a) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <EnvironmentFloatKind>::BackgroundColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_bg_color();
                prop_val.r = val as f32;
                let val = prop_val;
                obj.set_bg_color(val);
            }
            <EnvironmentFloatKind>::BackgroundColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_bg_color();
                prop_val.g = val as f32;
                let val = prop_val;
                obj.set_bg_color(val);
            }
            <EnvironmentFloatKind>::BackgroundColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_bg_color();
                prop_val.b = val as f32;
                let val = prop_val;
                obj.set_bg_color(val);
            }
            <EnvironmentFloatKind>::BackgroundColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_bg_color();
                prop_val.a = val as f32;
                let val = prop_val;
                obj.set_bg_color(val);
            }
        }
    }
}
impl IPropertyData for EnvironmentFloatData {
    type Target = Environment;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <EnvironmentFloatKind>::BackgroundColorR => {
                NodePath::from("background_color:r")
            }
            <EnvironmentFloatKind>::BackgroundColorG => {
                NodePath::from("background_color:g")
            }
            <EnvironmentFloatKind>::BackgroundColorB => {
                NodePath::from("background_color:b")
            }
            <EnvironmentFloatKind>::BackgroundColorA => {
                NodePath::from("background_color:a")
            }
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for EnvironmentFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Environment>()
            .ok()
            .and_then(|owner| {
                match path {
                    "background_color:r" => {
                        Some(Self {
                            property: <EnvironmentFloatKind>::BackgroundColorR,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "background_color:g" => {
                        Some(Self {
                            property: <EnvironmentFloatKind>::BackgroundColorG,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "background_color:b" => {
                        Some(Self {
                            property: <EnvironmentFloatKind>::BackgroundColorB,
                            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                                owner.clone().upcast::<Object>(),
                            ),
                            owner,
                        })
                    }
                    "background_color:a" => {
                        Some(Self {
                            property: <EnvironmentFloatKind>::BackgroundColorA,
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
pub struct EnvironmentColorData {
    pub property: EnvironmentColorKind,
    pub owner: Gd<Environment>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentColorKind {
    BackgroundColor,
}
impl IProperty<Color> for EnvironmentColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <EnvironmentColorKind>::BackgroundColor => {
                let obj = &self.owner;
                obj.get_bg_color()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <EnvironmentColorKind>::BackgroundColor => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_bg_color(val);
            }
        }
    }
}
impl IPropertyData for EnvironmentColorData {
    type Target = Environment;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <EnvironmentColorKind>::BackgroundColor => NodePath::from("background_color"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for EnvironmentColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Environment>()
            .ok()
            .and_then(|owner| {
                match path {
                    "background_color" => {
                        Some(Self {
                            property: <EnvironmentColorKind>::BackgroundColor,
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
pub trait SpireDoEnvironment<Marker = ()> {
    fn do_background_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_background_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_background_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_background_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_background_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
}
impl<Class: Inherits<Environment> + Inherits<Object>> SpireDoEnvironment<()>
for Gd<Class> {
    fn do_background_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = EnvironmentFloatData {
            property: <EnvironmentFloatKind>::BackgroundColorR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_background_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = EnvironmentFloatData {
            property: <EnvironmentFloatKind>::BackgroundColorG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_background_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = EnvironmentFloatData {
            property: <EnvironmentFloatKind>::BackgroundColorB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_background_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = EnvironmentFloatData {
            property: <EnvironmentFloatKind>::BackgroundColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                self.clone().upcast::<Object>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_background_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = EnvironmentColorData {
            property: <EnvironmentColorKind>::BackgroundColor,
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
    T: WithBaseField + Inherits<Environment> + Inherits<Object>,
> SpireDoEnvironment<BaseMarker> for T {
    fn do_background_color_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Environment> = self.to_gd().upcast();
        let data = EnvironmentFloatData {
            property: <EnvironmentFloatKind>::BackgroundColorR,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_background_color_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Environment> = self.to_gd().upcast();
        let data = EnvironmentFloatData {
            property: <EnvironmentFloatKind>::BackgroundColorG,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_background_color_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Environment> = self.to_gd().upcast();
        let data = EnvironmentFloatData {
            property: <EnvironmentFloatKind>::BackgroundColorB,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_background_color_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Environment> = self.to_gd().upcast();
        let data = EnvironmentFloatData {
            property: <EnvironmentFloatKind>::BackgroundColorA,
            owner_obj_or_node: ObjectOrNode::from_unchecked_object(
                owner.clone().upcast::<Object>(),
            ),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_background_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<Environment> = self.to_gd().upcast();
        let data = EnvironmentColorData {
            property: <EnvironmentColorKind>::BackgroundColor,
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
