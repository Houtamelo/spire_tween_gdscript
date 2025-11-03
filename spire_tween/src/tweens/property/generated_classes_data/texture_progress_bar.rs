use super::*;
#[derive(Debug, Clone)]
pub struct TextureProgressBarFloatData {
    pub property: TextureProgressBarFloatKind,
    pub owner: Gd<TextureProgressBar>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureProgressBarFloatKind {
    RadialInitialAngle,
    RadialFillDegrees,
    TintUnderR,
    TintUnderG,
    TintUnderB,
    TintUnderA,
    TintOverR,
    TintOverG,
    TintOverB,
    TintOverA,
    TintProgressR,
    TintProgressG,
    TintProgressB,
    TintProgressA,
    RadialCenterOffsetX,
    RadialCenterOffsetY,
}
impl IProperty<f64> for TextureProgressBarFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <TextureProgressBarFloatKind>::RadialInitialAngle => {
                let obj = &self.owner;
                (obj.clone().get_radial_initial_angle()) as f64
            }
            <TextureProgressBarFloatKind>::RadialFillDegrees => {
                let obj = &self.owner;
                (obj.clone().get_fill_degrees()) as f64
            }
            <TextureProgressBarFloatKind>::TintUnderR => {
                let obj = &self.owner;
                (obj.get_tint_under().r) as f64
            }
            <TextureProgressBarFloatKind>::TintUnderG => {
                let obj = &self.owner;
                (obj.get_tint_under().g) as f64
            }
            <TextureProgressBarFloatKind>::TintUnderB => {
                let obj = &self.owner;
                (obj.get_tint_under().b) as f64
            }
            <TextureProgressBarFloatKind>::TintUnderA => {
                let obj = &self.owner;
                (obj.get_tint_under().a) as f64
            }
            <TextureProgressBarFloatKind>::TintOverR => {
                let obj = &self.owner;
                (obj.get_tint_over().r) as f64
            }
            <TextureProgressBarFloatKind>::TintOverG => {
                let obj = &self.owner;
                (obj.get_tint_over().g) as f64
            }
            <TextureProgressBarFloatKind>::TintOverB => {
                let obj = &self.owner;
                (obj.get_tint_over().b) as f64
            }
            <TextureProgressBarFloatKind>::TintOverA => {
                let obj = &self.owner;
                (obj.get_tint_over().a) as f64
            }
            <TextureProgressBarFloatKind>::TintProgressR => {
                let obj = &self.owner;
                (obj.get_tint_progress().r) as f64
            }
            <TextureProgressBarFloatKind>::TintProgressG => {
                let obj = &self.owner;
                (obj.get_tint_progress().g) as f64
            }
            <TextureProgressBarFloatKind>::TintProgressB => {
                let obj = &self.owner;
                (obj.get_tint_progress().b) as f64
            }
            <TextureProgressBarFloatKind>::TintProgressA => {
                let obj = &self.owner;
                (obj.get_tint_progress().a) as f64
            }
            <TextureProgressBarFloatKind>::RadialCenterOffsetX => {
                let obj = &self.owner;
                (obj.clone().get_radial_center_offset().x) as f64
            }
            <TextureProgressBarFloatKind>::RadialCenterOffsetY => {
                let obj = &self.owner;
                (obj.clone().get_radial_center_offset().y) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <TextureProgressBarFloatKind>::RadialInitialAngle => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_radial_initial_angle(val as f32);
            }
            <TextureProgressBarFloatKind>::RadialFillDegrees => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_fill_degrees(val as f32);
            }
            <TextureProgressBarFloatKind>::TintUnderR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_under();
                prop_val.r = val as f32;
                obj.set_tint_under(prop_val);
            }
            <TextureProgressBarFloatKind>::TintUnderG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_under();
                prop_val.g = val as f32;
                obj.set_tint_under(prop_val);
            }
            <TextureProgressBarFloatKind>::TintUnderB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_under();
                prop_val.b = val as f32;
                obj.set_tint_under(prop_val);
            }
            <TextureProgressBarFloatKind>::TintUnderA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_under();
                prop_val.a = val as f32;
                obj.set_tint_under(prop_val);
            }
            <TextureProgressBarFloatKind>::TintOverR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_over();
                prop_val.r = val as f32;
                obj.set_tint_over(prop_val);
            }
            <TextureProgressBarFloatKind>::TintOverG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_over();
                prop_val.g = val as f32;
                obj.set_tint_over(prop_val);
            }
            <TextureProgressBarFloatKind>::TintOverB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_over();
                prop_val.b = val as f32;
                obj.set_tint_over(prop_val);
            }
            <TextureProgressBarFloatKind>::TintOverA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_over();
                prop_val.a = val as f32;
                obj.set_tint_over(prop_val);
            }
            <TextureProgressBarFloatKind>::TintProgressR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_progress();
                prop_val.r = val as f32;
                obj.set_tint_progress(prop_val);
            }
            <TextureProgressBarFloatKind>::TintProgressG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_progress();
                prop_val.g = val as f32;
                obj.set_tint_progress(prop_val);
            }
            <TextureProgressBarFloatKind>::TintProgressB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_progress();
                prop_val.b = val as f32;
                obj.set_tint_progress(prop_val);
            }
            <TextureProgressBarFloatKind>::TintProgressA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_progress();
                prop_val.a = val as f32;
                obj.set_tint_progress(prop_val);
            }
            <TextureProgressBarFloatKind>::RadialCenterOffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_radial_center_offset();
                prop_val.x = val as f32;
                obj.set_radial_center_offset(prop_val);
            }
            <TextureProgressBarFloatKind>::RadialCenterOffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_radial_center_offset();
                prop_val.y = val as f32;
                obj.set_radial_center_offset(prop_val);
            }
        }
    }
}
impl IPropertyData for TextureProgressBarFloatData {
    type Target = TextureProgressBar;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <TextureProgressBarFloatKind>::RadialInitialAngle => {
                NodePath::from("radial_initial_angle")
            }
            <TextureProgressBarFloatKind>::RadialFillDegrees => {
                NodePath::from("radial_fill_degrees")
            }
            <TextureProgressBarFloatKind>::TintUnderR => NodePath::from("tint_under:r"),
            <TextureProgressBarFloatKind>::TintUnderG => NodePath::from("tint_under:g"),
            <TextureProgressBarFloatKind>::TintUnderB => NodePath::from("tint_under:b"),
            <TextureProgressBarFloatKind>::TintUnderA => NodePath::from("tint_under:a"),
            <TextureProgressBarFloatKind>::TintOverR => NodePath::from("tint_over:r"),
            <TextureProgressBarFloatKind>::TintOverG => NodePath::from("tint_over:g"),
            <TextureProgressBarFloatKind>::TintOverB => NodePath::from("tint_over:b"),
            <TextureProgressBarFloatKind>::TintOverA => NodePath::from("tint_over:a"),
            <TextureProgressBarFloatKind>::TintProgressR => {
                NodePath::from("tint_progress:r")
            }
            <TextureProgressBarFloatKind>::TintProgressG => {
                NodePath::from("tint_progress:g")
            }
            <TextureProgressBarFloatKind>::TintProgressB => {
                NodePath::from("tint_progress:b")
            }
            <TextureProgressBarFloatKind>::TintProgressA => {
                NodePath::from("tint_progress:a")
            }
            <TextureProgressBarFloatKind>::RadialCenterOffsetX => {
                NodePath::from("radial_center_offset:x")
            }
            <TextureProgressBarFloatKind>::RadialCenterOffsetY => {
                NodePath::from("radial_center_offset:y")
            }
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<TextureProgressBar>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<TextureProgressBar>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for TextureProgressBarFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<TextureProgressBar>()
            .ok()
            .and_then(|owner| {
                match path {
                    "radial_initial_angle" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::RadialInitialAngle,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "radial_fill_degrees" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::RadialFillDegrees,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_under:r" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::TintUnderR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_under:g" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::TintUnderG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_under:b" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::TintUnderB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_under:a" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::TintUnderA,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_over:r" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::TintOverR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_over:g" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::TintOverG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_over:b" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::TintOverB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_over:a" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::TintOverA,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_progress:r" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::TintProgressR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_progress:g" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::TintProgressG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_progress:b" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::TintProgressB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_progress:a" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::TintProgressA,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "radial_center_offset:x" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::RadialCenterOffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "radial_center_offset:y" => {
                        Some(Self {
                            property: <TextureProgressBarFloatKind>::RadialCenterOffsetY,
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
pub struct TextureProgressBarVector2Data {
    pub property: TextureProgressBarVector2Kind,
    pub owner: Gd<TextureProgressBar>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureProgressBarVector2Kind {
    RadialCenterOffset,
}
impl IProperty<Vector2> for TextureProgressBarVector2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <TextureProgressBarVector2Kind>::RadialCenterOffset => {
                let obj = &self.owner;
                obj.clone().get_radial_center_offset()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <TextureProgressBarVector2Kind>::RadialCenterOffset => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_radial_center_offset(val);
            }
        }
    }
}
impl IPropertyData for TextureProgressBarVector2Data {
    type Target = TextureProgressBar;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <TextureProgressBarVector2Kind>::RadialCenterOffset => {
                NodePath::from("radial_center_offset")
            }
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<TextureProgressBar>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<TextureProgressBar>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for TextureProgressBarVector2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<TextureProgressBar>()
            .ok()
            .and_then(|owner| {
                match path {
                    "radial_center_offset" => {
                        Some(Self {
                            property: <TextureProgressBarVector2Kind>::RadialCenterOffset,
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
pub struct TextureProgressBarColorData {
    pub property: TextureProgressBarColorKind,
    pub owner: Gd<TextureProgressBar>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureProgressBarColorKind {
    TintUnder,
    TintOver,
    TintProgress,
}
impl IProperty<Color> for TextureProgressBarColorData {
    #[inline]
    fn get_property_value(&self) -> Color {
        match self.property {
            <TextureProgressBarColorKind>::TintUnder => {
                let obj = &self.owner;
                obj.get_tint_under()
            }
            <TextureProgressBarColorKind>::TintOver => {
                let obj = &self.owner;
                obj.get_tint_over()
            }
            <TextureProgressBarColorKind>::TintProgress => {
                let obj = &self.owner;
                obj.get_tint_progress()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Color) {
        match self.property {
            <TextureProgressBarColorKind>::TintUnder => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_tint_under(val);
            }
            <TextureProgressBarColorKind>::TintOver => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_tint_over(val);
            }
            <TextureProgressBarColorKind>::TintProgress => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_tint_progress(val);
            }
        }
    }
}
impl IPropertyData for TextureProgressBarColorData {
    type Target = TextureProgressBar;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <TextureProgressBarColorKind>::TintUnder => NodePath::from("tint_under"),
            <TextureProgressBarColorKind>::TintOver => NodePath::from("tint_over"),
            <TextureProgressBarColorKind>::TintProgress => {
                NodePath::from("tint_progress")
            }
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<TextureProgressBar>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<TextureProgressBar>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for TextureProgressBarColorData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<TextureProgressBar>()
            .ok()
            .and_then(|owner| {
                match path {
                    "tint_under" => {
                        Some(Self {
                            property: <TextureProgressBarColorKind>::TintUnder,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_over" => {
                        Some(Self {
                            property: <TextureProgressBarColorKind>::TintOver,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_progress" => {
                        Some(Self {
                            property: <TextureProgressBarColorKind>::TintProgress,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoTextureProgressBar<Marker = ()> {
    fn do_radial_initial_angle(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_radial_fill_degrees(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_tint_under_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_tint_under_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_tint_under_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_tint_under_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_tint_over_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_tint_over_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_tint_over_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_tint_over_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_tint_progress_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_tint_progress_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_tint_progress_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_tint_progress_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_radial_center_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_radial_center_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_radial_center_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
    fn do_tint_under(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
    fn do_tint_over(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
    fn do_tint_progress(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>>;
}
impl<
    Class: Inherits<TextureProgressBar> + Inherits<Object>,
> SpireDoTextureProgressBar<()> for Gd<Class> {
    fn do_radial_initial_angle(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::RadialInitialAngle,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_radial_fill_degrees(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::RadialFillDegrees,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_under_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintUnderR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_under_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintUnderG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_under_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintUnderB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_under_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintUnderA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_over_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintOverR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_over_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintOverG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_over_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintOverB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_over_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintOverA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_progress_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintProgressR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_progress_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintProgressG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_progress_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintProgressB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_progress_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintProgressA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_radial_center_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::RadialCenterOffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_radial_center_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::RadialCenterOffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_radial_center_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = TextureProgressBarVector2Data {
            property: <TextureProgressBarVector2Kind>::RadialCenterOffset,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_under(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = TextureProgressBarColorData {
            property: <TextureProgressBarColorKind>::TintUnder,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_over(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = TextureProgressBarColorData {
            property: <TextureProgressBarColorKind>::TintOver,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_progress(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let data = TextureProgressBarColorData {
            property: <TextureProgressBarColorKind>::TintProgress,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<TextureProgressBar> + Inherits<Object>,
> SpireDoTextureProgressBar<BaseMarker> for T {
    fn do_radial_initial_angle(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::RadialInitialAngle,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_radial_fill_degrees(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::RadialFillDegrees,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_under_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintUnderR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_under_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintUnderG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_under_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintUnderB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_under_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintUnderA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_over_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintOverR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_over_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintOverG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_over_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintOverB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_over_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintOverA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_progress_r(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintProgressR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_progress_g(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintProgressG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_progress_b(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintProgressB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_progress_a(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::TintProgressA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_radial_center_offset_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::RadialCenterOffsetX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_radial_center_offset_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarFloatData {
            property: <TextureProgressBarFloatKind>::RadialCenterOffsetY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_radial_center_offset(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarVector2Data {
            property: <TextureProgressBarVector2Kind>::RadialCenterOffset,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_under(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarColorData {
            property: <TextureProgressBarColorKind>::TintUnder,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_over(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarColorData {
            property: <TextureProgressBarColorKind>::TintOver,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_tint_progress(
        &self,
        end_val: Color,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Color>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarColorData {
            property: <TextureProgressBarColorKind>::TintProgress,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Color>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
