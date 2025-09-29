use super::*;
#[derive(Debug, Clone)]
pub struct TextureProgressBarF32Data {
    pub property: TextureProgressBarF32Kind,
    pub owner: Gd<TextureProgressBar>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureProgressBarF32Kind {
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
impl IProperty<f32> for TextureProgressBarF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <TextureProgressBarF32Kind>::RadialInitialAngle => {
                let obj = &self.owner;
                obj.clone().get_radial_initial_angle()
            }
            <TextureProgressBarF32Kind>::RadialFillDegrees => {
                let obj = &self.owner;
                obj.clone().get_fill_degrees()
            }
            <TextureProgressBarF32Kind>::TintUnderR => {
                let obj = &self.owner;
                obj.get_tint_under().r
            }
            <TextureProgressBarF32Kind>::TintUnderG => {
                let obj = &self.owner;
                obj.get_tint_under().g
            }
            <TextureProgressBarF32Kind>::TintUnderB => {
                let obj = &self.owner;
                obj.get_tint_under().b
            }
            <TextureProgressBarF32Kind>::TintUnderA => {
                let obj = &self.owner;
                obj.get_tint_under().a
            }
            <TextureProgressBarF32Kind>::TintOverR => {
                let obj = &self.owner;
                obj.get_tint_over().r
            }
            <TextureProgressBarF32Kind>::TintOverG => {
                let obj = &self.owner;
                obj.get_tint_over().g
            }
            <TextureProgressBarF32Kind>::TintOverB => {
                let obj = &self.owner;
                obj.get_tint_over().b
            }
            <TextureProgressBarF32Kind>::TintOverA => {
                let obj = &self.owner;
                obj.get_tint_over().a
            }
            <TextureProgressBarF32Kind>::TintProgressR => {
                let obj = &self.owner;
                obj.get_tint_progress().r
            }
            <TextureProgressBarF32Kind>::TintProgressG => {
                let obj = &self.owner;
                obj.get_tint_progress().g
            }
            <TextureProgressBarF32Kind>::TintProgressB => {
                let obj = &self.owner;
                obj.get_tint_progress().b
            }
            <TextureProgressBarF32Kind>::TintProgressA => {
                let obj = &self.owner;
                obj.get_tint_progress().a
            }
            <TextureProgressBarF32Kind>::RadialCenterOffsetX => {
                let obj = &self.owner;
                obj.clone().get_radial_center_offset().x
            }
            <TextureProgressBarF32Kind>::RadialCenterOffsetY => {
                let obj = &self.owner;
                obj.clone().get_radial_center_offset().y
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <TextureProgressBarF32Kind>::RadialInitialAngle => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_radial_initial_angle(val);
            }
            <TextureProgressBarF32Kind>::RadialFillDegrees => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_fill_degrees(val);
            }
            <TextureProgressBarF32Kind>::TintUnderR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_under();
                prop_val.r = val;
                obj.set_tint_under(prop_val);
            }
            <TextureProgressBarF32Kind>::TintUnderG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_under();
                prop_val.g = val;
                obj.set_tint_under(prop_val);
            }
            <TextureProgressBarF32Kind>::TintUnderB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_under();
                prop_val.b = val;
                obj.set_tint_under(prop_val);
            }
            <TextureProgressBarF32Kind>::TintUnderA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_under();
                prop_val.a = val;
                obj.set_tint_under(prop_val);
            }
            <TextureProgressBarF32Kind>::TintOverR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_over();
                prop_val.r = val;
                obj.set_tint_over(prop_val);
            }
            <TextureProgressBarF32Kind>::TintOverG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_over();
                prop_val.g = val;
                obj.set_tint_over(prop_val);
            }
            <TextureProgressBarF32Kind>::TintOverB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_over();
                prop_val.b = val;
                obj.set_tint_over(prop_val);
            }
            <TextureProgressBarF32Kind>::TintOverA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_over();
                prop_val.a = val;
                obj.set_tint_over(prop_val);
            }
            <TextureProgressBarF32Kind>::TintProgressR => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_progress();
                prop_val.r = val;
                obj.set_tint_progress(prop_val);
            }
            <TextureProgressBarF32Kind>::TintProgressG => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_progress();
                prop_val.g = val;
                obj.set_tint_progress(prop_val);
            }
            <TextureProgressBarF32Kind>::TintProgressB => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_progress();
                prop_val.b = val;
                obj.set_tint_progress(prop_val);
            }
            <TextureProgressBarF32Kind>::TintProgressA => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_tint_progress();
                prop_val.a = val;
                obj.set_tint_progress(prop_val);
            }
            <TextureProgressBarF32Kind>::RadialCenterOffsetX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_radial_center_offset();
                prop_val.x = val;
                obj.set_radial_center_offset(prop_val);
            }
            <TextureProgressBarF32Kind>::RadialCenterOffsetY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_radial_center_offset();
                prop_val.y = val;
                obj.set_radial_center_offset(prop_val);
            }
        }
    }
}
impl IPropertyData for TextureProgressBarF32Data {
    type Target = TextureProgressBar;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <TextureProgressBarF32Kind>::RadialInitialAngle => {
                NodePath::from("radial_initial_angle")
            }
            <TextureProgressBarF32Kind>::RadialFillDegrees => {
                NodePath::from("radial_fill_degrees")
            }
            <TextureProgressBarF32Kind>::TintUnderR => NodePath::from("tint_under:r"),
            <TextureProgressBarF32Kind>::TintUnderG => NodePath::from("tint_under:g"),
            <TextureProgressBarF32Kind>::TintUnderB => NodePath::from("tint_under:b"),
            <TextureProgressBarF32Kind>::TintUnderA => NodePath::from("tint_under:a"),
            <TextureProgressBarF32Kind>::TintOverR => NodePath::from("tint_over:r"),
            <TextureProgressBarF32Kind>::TintOverG => NodePath::from("tint_over:g"),
            <TextureProgressBarF32Kind>::TintOverB => NodePath::from("tint_over:b"),
            <TextureProgressBarF32Kind>::TintOverA => NodePath::from("tint_over:a"),
            <TextureProgressBarF32Kind>::TintProgressR => {
                NodePath::from("tint_progress:r")
            }
            <TextureProgressBarF32Kind>::TintProgressG => {
                NodePath::from("tint_progress:g")
            }
            <TextureProgressBarF32Kind>::TintProgressB => {
                NodePath::from("tint_progress:b")
            }
            <TextureProgressBarF32Kind>::TintProgressA => {
                NodePath::from("tint_progress:a")
            }
            <TextureProgressBarF32Kind>::RadialCenterOffsetX => {
                NodePath::from("radial_center_offset:x")
            }
            <TextureProgressBarF32Kind>::RadialCenterOffsetY => {
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
impl TryFromPathAndObject for TextureProgressBarF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<TextureProgressBar>()
            .ok()
            .and_then(|owner| {
                match path {
                    "radial_initial_angle" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::RadialInitialAngle,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "radial_fill_degrees" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::RadialFillDegrees,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_under:r" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::TintUnderR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_under:g" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::TintUnderG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_under:b" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::TintUnderB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_under:a" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::TintUnderA,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_over:r" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::TintOverR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_over:g" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::TintOverG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_over:b" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::TintOverB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_over:a" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::TintOverA,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_progress:r" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::TintProgressR,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_progress:g" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::TintProgressG,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_progress:b" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::TintProgressB,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "tint_progress:a" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::TintProgressA,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "radial_center_offset:x" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::RadialCenterOffsetX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "radial_center_offset:y" => {
                        Some(Self {
                            property: <TextureProgressBarF32Kind>::RadialCenterOffsetY,
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
pub trait DoTextureProgressBar<Marker = ()> {
    fn do_radial_initial_angle(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_radial_fill_degrees(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_tint_under_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_tint_under_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_tint_under_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_tint_under_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_tint_over_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_tint_over_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_tint_over_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_tint_over_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_tint_progress_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_tint_progress_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_tint_progress_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_tint_progress_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_radial_center_offset_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn do_radial_center_offset_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
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
impl<Class: Inherits<TextureProgressBar> + Inherits<Object>> DoTextureProgressBar<()>
for Gd<Class> {
    fn do_radial_initial_angle(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::RadialInitialAngle,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_radial_fill_degrees(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::RadialFillDegrees,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_under_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintUnderR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_under_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintUnderG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_under_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintUnderB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_under_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintUnderA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_over_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintOverR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_over_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintOverG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_over_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintOverB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_over_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintOverA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_progress_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintProgressR,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_progress_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintProgressG,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_progress_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintProgressB,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_progress_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintProgressA,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_radial_center_offset_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::RadialCenterOffsetX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_radial_center_offset_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::RadialCenterOffsetY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<TextureProgressBar>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<TextureProgressBar> + Inherits<Object>,
> DoTextureProgressBar<BaseMarker> for T {
    fn do_radial_initial_angle(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::RadialInitialAngle,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_radial_fill_degrees(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::RadialFillDegrees,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_under_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintUnderR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_under_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintUnderG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_under_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintUnderB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_under_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintUnderA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_over_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintOverR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_over_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintOverG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_over_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintOverB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_over_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintOverA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_progress_r(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintProgressR,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_progress_g(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintProgressG,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_progress_b(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintProgressB,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_tint_progress_a(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::TintProgressA,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_radial_center_offset_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::RadialCenterOffsetX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_radial_center_offset_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<TextureProgressBar> = self.to_gd().upcast();
        let data = TextureProgressBarF32Data {
            property: <TextureProgressBarF32Kind>::RadialCenterOffsetY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct TextureProgressBarTweener {}
#[godot_api]
impl TextureProgressBarTweener {
    #[func]
    fn do_radial_initial_angle(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_radial_initial_angle(to, duration), gd
        }
    }
    #[func]
    fn do_radial_fill_degrees(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_radial_fill_degrees(to, duration), gd
        }
    }
    #[func]
    fn do_tint_under_r(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_under_r(to, duration), gd
        }
    }
    #[func]
    fn do_tint_under_g(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_under_g(to, duration), gd
        }
    }
    #[func]
    fn do_tint_under_b(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_under_b(to, duration), gd
        }
    }
    #[func]
    fn do_tint_under_a(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_under_a(to, duration), gd
        }
    }
    #[func]
    fn do_tint_over_r(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_over_r(to, duration), gd
        }
    }
    #[func]
    fn do_tint_over_g(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_over_g(to, duration), gd
        }
    }
    #[func]
    fn do_tint_over_b(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_over_b(to, duration), gd
        }
    }
    #[func]
    fn do_tint_over_a(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_over_a(to, duration), gd
        }
    }
    #[func]
    fn do_tint_progress_r(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_progress_r(to, duration), gd
        }
    }
    #[func]
    fn do_tint_progress_g(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_progress_g(to, duration), gd
        }
    }
    #[func]
    fn do_tint_progress_b(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_progress_b(to, duration), gd
        }
    }
    #[func]
    fn do_tint_progress_a(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_progress_a(to, duration), gd
        }
    }
    #[func]
    fn do_radial_center_offset_x(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_radial_center_offset_x(to, duration), gd
        }
    }
    #[func]
    fn do_radial_center_offset_y(
        node: Gd<TextureProgressBar>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_radial_center_offset_y(to, duration), gd
        }
    }
    #[func]
    fn do_radial_center_offset(
        node: Gd<TextureProgressBar>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_radial_center_offset(to, duration), gd
        }
    }
    #[func]
    fn do_tint_under(
        node: Gd<TextureProgressBar>,
        to: Color,
        duration: f64,
    ) -> Gd<<Color as TyToGdTween>::GdTween> {
        let gd = <Color as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_under(to, duration), gd
        }
    }
    #[func]
    fn do_tint_over(
        node: Gd<TextureProgressBar>,
        to: Color,
        duration: f64,
    ) -> Gd<<Color as TyToGdTween>::GdTween> {
        let gd = <Color as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_over(to, duration), gd
        }
    }
    #[func]
    fn do_tint_progress(
        node: Gd<TextureProgressBar>,
        to: Color,
        duration: f64,
    ) -> Gd<<Color as TyToGdTween>::GdTween> {
        let gd = <Color as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_tint_progress(to, duration), gd
        }
    }
}
