use super::*;
#[derive(Debug, Clone)]
pub struct ColorRectF32Data {
    pub property: ColorRectF32Kind,
    pub owner: Gd<ColorRect>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorRectF32Kind {
    ColorR,
    ColorG,
    ColorB,
    ColorA,
}
impl IProperty<f32> for ColorRectF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <ColorRectF32Kind>::ColorR => {
                let obj = &self.owner;
                obj.get_color().r
            }
            <ColorRectF32Kind>::ColorG => {
                let obj = &self.owner;
                obj.get_color().g
            }
            <ColorRectF32Kind>::ColorB => {
                let obj = &self.owner;
                obj.get_color().b
            }
            <ColorRectF32Kind>::ColorA => {
                let obj = &self.owner;
                obj.get_color().a
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <ColorRectF32Kind>::ColorR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.r = val;
                obj.set_color(prop_val);
            }
            <ColorRectF32Kind>::ColorG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.g = val;
                obj.set_color(prop_val);
            }
            <ColorRectF32Kind>::ColorB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.b = val;
                obj.set_color(prop_val);
            }
            <ColorRectF32Kind>::ColorA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_color();
                prop_val.a = val;
                obj.set_color(prop_val);
            }
        }
    }
}
impl IPropertyData for ColorRectF32Data {
    type Target = ColorRect;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ColorRectF32Kind>::ColorR => NodePath::from("color:r"),
            <ColorRectF32Kind>::ColorG => NodePath::from("color:g"),
            <ColorRectF32Kind>::ColorB => NodePath::from("color:b"),
            <ColorRectF32Kind>::ColorA => NodePath::from("color:a"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<ColorRect>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<ColorRect>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ColorRectF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<ColorRect>()
            .ok()
            .and_then(|owner| {
                match path {
                    "color:r" => {
                        Some(Self {
                            property: <ColorRectF32Kind>::ColorR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "color:g" => {
                        Some(Self {
                            property: <ColorRectF32Kind>::ColorG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "color:b" => {
                        Some(Self {
                            property: <ColorRectF32Kind>::ColorB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "color:a" => {
                        Some(Self {
                            property: <ColorRectF32Kind>::ColorA,
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
pub struct ColorRectColorData {
    pub property: ColorRectColorKind,
    pub owner: Gd<ColorRect>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorRectColorKind {
    Color,
}
impl IProperty<Color> for ColorRectColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <ColorRectColorKind>::Color => {
                let obj = &self.owner;
                obj.get_color()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <ColorRectColorKind>::Color => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_color(val);
            }
        }
    }
}
impl IPropertyData for ColorRectColorData {
    type Target = ColorRect;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ColorRectColorKind>::Color => NodePath::from("color"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<ColorRect>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<ColorRect>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ColorRectColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<ColorRect>()
            .ok()
            .and_then(|owner| {
                match path {
                    "color" => {
                        Some(Self {
                            property: <ColorRectColorKind>::Color,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoColorRect<Marker = ()> {
    fn rect_do_color_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn rect_do_color_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn rect_do_color_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn rect_do_color_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn rect_do_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
}
impl<Class: Inherits<ColorRect> + Inherits<Object>> DoColorRect<()> for Gd<Class> {
    fn rect_do_color_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ColorRectF32Data {
            property: <ColorRectF32Kind>::ColorR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ColorRect>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn rect_do_color_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ColorRectF32Data {
            property: <ColorRectF32Kind>::ColorG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ColorRect>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn rect_do_color_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ColorRectF32Data {
            property: <ColorRectF32Kind>::ColorB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ColorRect>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn rect_do_color_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = ColorRectF32Data {
            property: <ColorRectF32Kind>::ColorA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ColorRect>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn rect_do_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = ColorRectColorData {
            property: <ColorRectColorKind>::Color,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ColorRect>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<T: WithBaseField + Inherits<ColorRect> + Inherits<Object>> DoColorRect<BaseMarker>
for T {
    fn rect_do_color_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<ColorRect> = self.to_gd().upcast();
        let data = ColorRectF32Data {
            property: <ColorRectF32Kind>::ColorR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn rect_do_color_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<ColorRect> = self.to_gd().upcast();
        let data = ColorRectF32Data {
            property: <ColorRectF32Kind>::ColorG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn rect_do_color_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<ColorRect> = self.to_gd().upcast();
        let data = ColorRectF32Data {
            property: <ColorRectF32Kind>::ColorB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn rect_do_color_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<ColorRect> = self.to_gd().upcast();
        let data = ColorRectF32Data {
            property: <ColorRectF32Kind>::ColorA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn rect_do_color(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<ColorRect> = self.to_gd().upcast();
        let data = ColorRectColorData {
            property: <ColorRectColorKind>::Color,
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
pub struct ColorRectTweener {}
#[godot_api]
impl ColorRectTweener {
    #[func]
    fn rect_do_color_r(
        node: Gd<ColorRect>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.rect_do_color_r(to, duration), gd
        }
    }
    #[func]
    fn rect_do_color_g(
        node: Gd<ColorRect>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.rect_do_color_g(to, duration), gd
        }
    }
    #[func]
    fn rect_do_color_b(
        node: Gd<ColorRect>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.rect_do_color_b(to, duration), gd
        }
    }
    #[func]
    fn rect_do_color_a(
        node: Gd<ColorRect>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.rect_do_color_a(to, duration), gd
        }
    }
    #[func]
    fn rect_do_color(
        node: Gd<ColorRect>,
        to: Color,
        duration: f64,
    ) -> Gd<<Color as TyToGdTween>::GdTween> {
        let gd = <Color as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.rect_do_color(to, duration), gd
        }
    }
}
