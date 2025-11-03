use super::*;
#[derive(Debug, Clone)]
pub struct RangeFloatData {
    pub property: RangeFloatKind,
    pub owner: Gd<Range>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeFloatKind {
    Value,
    Ratio,
    MinValue,
    MaxValue,
}
impl IProperty<f64> for RangeFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <RangeFloatKind>::Value => {
                let obj = &self.owner;
                obj.get_value()
            }
            <RangeFloatKind>::Ratio => {
                let obj = &self.owner;
                obj.get_as_ratio()
            }
            <RangeFloatKind>::MinValue => {
                let obj = &self.owner;
                obj.get_min()
            }
            <RangeFloatKind>::MaxValue => {
                let obj = &self.owner;
                obj.get_max()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <RangeFloatKind>::Value => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_value(val);
            }
            <RangeFloatKind>::Ratio => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_as_ratio(val);
            }
            <RangeFloatKind>::MinValue => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_min(val);
            }
            <RangeFloatKind>::MaxValue => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_max(val);
            }
        }
    }
}
impl IPropertyData for RangeFloatData {
    type Target = Range;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RangeFloatKind>::Value => NodePath::from("value"),
            <RangeFloatKind>::Ratio => NodePath::from("ratio"),
            <RangeFloatKind>::MinValue => NodePath::from("min_value"),
            <RangeFloatKind>::MaxValue => NodePath::from("max_value"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<Range>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<Range>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for RangeFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Range>()
            .ok()
            .and_then(|owner| {
                match path {
                    "value" => {
                        Some(Self {
                            property: <RangeFloatKind>::Value,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "ratio" => {
                        Some(Self {
                            property: <RangeFloatKind>::Ratio,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "min_value" => {
                        Some(Self {
                            property: <RangeFloatKind>::MinValue,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "max_value" => {
                        Some(Self {
                            property: <RangeFloatKind>::MaxValue,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoRange<Marker = ()> {
    fn do_value(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>>;
    fn do_ratio(&self, end_val: f64, duration: f64) -> SpireTween<LerpPropertyData<f64>>;
    fn do_min_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_max_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<Class: Inherits<Range> + Inherits<Object>> SpireDoRange<()> for Gd<Class> {
    fn do_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RangeFloatData {
            property: <RangeFloatKind>::Value,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Range>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RangeFloatData {
            property: <RangeFloatKind>::Ratio,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Range>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_min_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RangeFloatData {
            property: <RangeFloatKind>::MinValue,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Range>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_max_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RangeFloatData {
            property: <RangeFloatKind>::MaxValue,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Range>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<T: WithBaseField + Inherits<Range> + Inherits<Object>> SpireDoRange<BaseMarker>
for T {
    fn do_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Range> = self.to_gd().upcast();
        let data = RangeFloatData {
            property: <RangeFloatKind>::Value,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Range> = self.to_gd().upcast();
        let data = RangeFloatData {
            property: <RangeFloatKind>::Ratio,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_min_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Range> = self.to_gd().upcast();
        let data = RangeFloatData {
            property: <RangeFloatKind>::MinValue,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_max_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Range> = self.to_gd().upcast();
        let data = RangeFloatData {
            property: <RangeFloatKind>::MaxValue,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
