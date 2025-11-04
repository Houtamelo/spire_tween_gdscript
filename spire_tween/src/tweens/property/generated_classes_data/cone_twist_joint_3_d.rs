use super::*;
#[derive(Debug, Clone)]
pub struct ConeTwistJoint3DFloatData {
    pub property: ConeTwistJoint3DFloatKind,
    pub owner: Gd<ConeTwistJoint3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConeTwistJoint3DFloatKind {
    Bias,
    Relaxation,
    Softness,
    SwingSpan,
    TwistSpan,
}
impl IProperty<f64> for ConeTwistJoint3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <ConeTwistJoint3DFloatKind>::Bias => {
                let obj = &self.owner;
                (obj.get_bias()) as f64
            }
            <ConeTwistJoint3DFloatKind>::Relaxation => {
                let obj = &self.owner;
                (obj.get_relaxation()) as f64
            }
            <ConeTwistJoint3DFloatKind>::Softness => {
                let obj = &self.owner;
                (obj.get_softness()) as f64
            }
            <ConeTwistJoint3DFloatKind>::SwingSpan => {
                let obj = &self.owner;
                (obj.get_swing_span()) as f64
            }
            <ConeTwistJoint3DFloatKind>::TwistSpan => {
                let obj = &self.owner;
                (obj.get_twist_span()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <ConeTwistJoint3DFloatKind>::Bias => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_bias(val as f32);
            }
            <ConeTwistJoint3DFloatKind>::Relaxation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_relaxation(val as f32);
            }
            <ConeTwistJoint3DFloatKind>::Softness => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_softness(val as f32);
            }
            <ConeTwistJoint3DFloatKind>::SwingSpan => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_swing_span(val as f32);
            }
            <ConeTwistJoint3DFloatKind>::TwistSpan => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_twist_span(val as f32);
            }
        }
    }
}
impl IPropertyData for ConeTwistJoint3DFloatData {
    type Target = ConeTwistJoint3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ConeTwistJoint3DFloatKind>::Bias => NodePath::from("bias"),
            <ConeTwistJoint3DFloatKind>::Relaxation => NodePath::from("relaxation"),
            <ConeTwistJoint3DFloatKind>::Softness => NodePath::from("softness"),
            <ConeTwistJoint3DFloatKind>::SwingSpan => NodePath::from("swing_span"),
            <ConeTwistJoint3DFloatKind>::TwistSpan => NodePath::from("twist_span"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<ConeTwistJoint3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<ConeTwistJoint3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ConeTwistJoint3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<ConeTwistJoint3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "bias" => {
                        Some(Self {
                            property: <ConeTwistJoint3DFloatKind>::Bias,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "relaxation" => {
                        Some(Self {
                            property: <ConeTwistJoint3DFloatKind>::Relaxation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "softness" => {
                        Some(Self {
                            property: <ConeTwistJoint3DFloatKind>::Softness,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "swing_span" => {
                        Some(Self {
                            property: <ConeTwistJoint3DFloatKind>::SwingSpan,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "twist_span" => {
                        Some(Self {
                            property: <ConeTwistJoint3DFloatKind>::TwistSpan,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoConeTwistJoint3D<Marker = ()> {
    fn do_bias(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>>;
    fn do_relaxation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_softness(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_swing_span(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_twist_span(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<Class: Inherits<ConeTwistJoint3D> + Inherits<Object>> SpireDoConeTwistJoint3D<()>
for Gd<Class> {
    fn do_bias(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let data = ConeTwistJoint3DFloatData {
            property: <ConeTwistJoint3DFloatKind>::Bias,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ConeTwistJoint3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_relaxation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ConeTwistJoint3DFloatData {
            property: <ConeTwistJoint3DFloatKind>::Relaxation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ConeTwistJoint3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_softness(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ConeTwistJoint3DFloatData {
            property: <ConeTwistJoint3DFloatKind>::Softness,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ConeTwistJoint3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_swing_span(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ConeTwistJoint3DFloatData {
            property: <ConeTwistJoint3DFloatKind>::SwingSpan,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ConeTwistJoint3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_twist_span(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = ConeTwistJoint3DFloatData {
            property: <ConeTwistJoint3DFloatKind>::TwistSpan,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ConeTwistJoint3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<ConeTwistJoint3D> + Inherits<Object>,
> SpireDoConeTwistJoint3D<BaseMarker> for T {
    fn do_bias(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ConeTwistJoint3D> = self.to_gd().upcast();
        let data = ConeTwistJoint3DFloatData {
            property: <ConeTwistJoint3DFloatKind>::Bias,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_relaxation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ConeTwistJoint3D> = self.to_gd().upcast();
        let data = ConeTwistJoint3DFloatData {
            property: <ConeTwistJoint3DFloatKind>::Relaxation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_softness(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ConeTwistJoint3D> = self.to_gd().upcast();
        let data = ConeTwistJoint3DFloatData {
            property: <ConeTwistJoint3DFloatKind>::Softness,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_swing_span(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ConeTwistJoint3D> = self.to_gd().upcast();
        let data = ConeTwistJoint3DFloatData {
            property: <ConeTwistJoint3DFloatKind>::SwingSpan,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_twist_span(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<ConeTwistJoint3D> = self.to_gd().upcast();
        let data = ConeTwistJoint3DFloatData {
            property: <ConeTwistJoint3DFloatKind>::TwistSpan,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
