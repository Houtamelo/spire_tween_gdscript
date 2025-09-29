use super::*;
#[derive(Debug, Clone)]
pub struct CanvasModulateF32Data {
    pub property: CanvasModulateF32Kind,
    pub owner: Gd<CanvasModulate>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasModulateF32Kind {
    ColorR,
    ColorG,
    ColorB,
    ColorA,
}
impl IProperty<f32> for CanvasModulateF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <CanvasModulateF32Kind>::ColorR => {
                let obj = &self.owner;
                obj.get_color().r
            }
            <CanvasModulateF32Kind>::ColorG => {
                let obj = &self.owner;
                obj.get_color().g
            }
            <CanvasModulateF32Kind>::ColorB => {
                let obj = &self.owner;
                obj.get_color().b
            }
            <CanvasModulateF32Kind>::ColorA => {
                let obj = &self.owner;
                obj.get_color().a
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <CanvasModulateF32Kind>::ColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.r = val;
                obj.set_color(prop_val);
            }
            <CanvasModulateF32Kind>::ColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.g = val;
                obj.set_color(prop_val);
            }
            <CanvasModulateF32Kind>::ColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.b = val;
                obj.set_color(prop_val);
            }
            <CanvasModulateF32Kind>::ColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.a = val;
                obj.set_color(prop_val);
            }
        }
    }
}
impl IPropertyData for CanvasModulateF32Data {
    type Target = CanvasModulate;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CanvasModulateF32Kind>::ColorR => NodePath::from("color:r"),
            <CanvasModulateF32Kind>::ColorG => NodePath::from("color:g"),
            <CanvasModulateF32Kind>::ColorB => NodePath::from("color:b"),
            <CanvasModulateF32Kind>::ColorA => NodePath::from("color:a"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<CanvasModulate>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<CanvasModulate>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for CanvasModulateF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CanvasModulate>()
            .ok()
            .and_then(|owner| {
                match path {
                    "color:r" => {
                        Some(Self {
                            property: <CanvasModulateF32Kind>::ColorR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "color:g" => {
                        Some(Self {
                            property: <CanvasModulateF32Kind>::ColorG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "color:b" => {
                        Some(Self {
                            property: <CanvasModulateF32Kind>::ColorB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "color:a" => {
                        Some(Self {
                            property: <CanvasModulateF32Kind>::ColorA,
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
pub struct CanvasModulateColorData {
    pub property: CanvasModulateColorKind,
    pub owner: Gd<CanvasModulate>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasModulateColorKind {
    Color,
}
impl IProperty<Color> for CanvasModulateColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <CanvasModulateColorKind>::Color => {
                let obj = &self.owner;
                obj.get_color()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <CanvasModulateColorKind>::Color => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_color(val);
            }
        }
    }
}
impl IPropertyData for CanvasModulateColorData {
    type Target = CanvasModulate;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CanvasModulateColorKind>::Color => NodePath::from("color"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<CanvasModulate>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<CanvasModulate>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for CanvasModulateColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CanvasModulate>()
            .ok()
            .and_then(|owner| {
                match path {
                    "color" => {
                        Some(Self {
                            property: <CanvasModulateColorKind>::Color,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoCanvasModulate<Marker = ()> {
    fn canvas_modulate_do_color_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn canvas_modulate_do_color_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn canvas_modulate_do_color_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn canvas_modulate_do_color_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn canvas_modulate_do_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
}
impl<Class: Inherits<CanvasModulate> + Inherits<Object>> DoCanvasModulate<()>
for Gd<Class> {
    fn canvas_modulate_do_color_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasModulateF32Data {
            property: <CanvasModulateF32Kind>::ColorR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasModulate>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_modulate_do_color_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasModulateF32Data {
            property: <CanvasModulateF32Kind>::ColorG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasModulate>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_modulate_do_color_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasModulateF32Data {
            property: <CanvasModulateF32Kind>::ColorB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasModulate>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_modulate_do_color_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CanvasModulateF32Data {
            property: <CanvasModulateF32Kind>::ColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasModulate>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_modulate_do_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = CanvasModulateColorData {
            property: <CanvasModulateColorKind>::Color,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CanvasModulate>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<CanvasModulate> + Inherits<Object>,
> DoCanvasModulate<BaseMarker> for T {
    fn canvas_modulate_do_color_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasModulate> = self.to_gd().upcast();
        let data = CanvasModulateF32Data {
            property: <CanvasModulateF32Kind>::ColorR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_modulate_do_color_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasModulate> = self.to_gd().upcast();
        let data = CanvasModulateF32Data {
            property: <CanvasModulateF32Kind>::ColorG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_modulate_do_color_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasModulate> = self.to_gd().upcast();
        let data = CanvasModulateF32Data {
            property: <CanvasModulateF32Kind>::ColorB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_modulate_do_color_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CanvasModulate> = self.to_gd().upcast();
        let data = CanvasModulateF32Data {
            property: <CanvasModulateF32Kind>::ColorA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn canvas_modulate_do_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<CanvasModulate> = self.to_gd().upcast();
        let data = CanvasModulateColorData {
            property: <CanvasModulateColorKind>::Color,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct CanvasModulateTweener {}
#[godot_api]
impl CanvasModulateTweener {
    #[func]
    fn canvas_modulate_do_color_r(
        node: Gd<CanvasModulate>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.canvas_modulate_do_color_r(to, duration), gd
        }
    }
    #[func]
    fn canvas_modulate_do_color_g(
        node: Gd<CanvasModulate>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.canvas_modulate_do_color_g(to, duration), gd
        }
    }
    #[func]
    fn canvas_modulate_do_color_b(
        node: Gd<CanvasModulate>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.canvas_modulate_do_color_b(to, duration), gd
        }
    }
    #[func]
    fn canvas_modulate_do_color_a(
        node: Gd<CanvasModulate>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.canvas_modulate_do_color_a(to, duration), gd
        }
    }
    #[func]
    fn canvas_modulate_do_color(
        node: Gd<CanvasModulate>,
        to: Color,
        duration: f64,
    ) -> Gd<<Color as TyToGdTween>::GdTween> {
        let gd = <Color as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.canvas_modulate_do_color(to, duration), gd
        }
    }
}
