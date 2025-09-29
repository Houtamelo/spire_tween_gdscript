use super::*;
#[derive(Debug, Clone)]
pub struct RangeF64Data {
    pub property: RangeF64Kind,
    pub owner: Gd<Range>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeF64Kind {
    Value,
    Ratio,
    MinValue,
    MaxValue,
}
impl IProperty<f64> for RangeF64Data {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <RangeF64Kind>::Value => {
                let obj = &self.owner;
                obj.get_value()
            }
            <RangeF64Kind>::Ratio => {
                let obj = &self.owner;
                obj.get_as_ratio()
            }
            <RangeF64Kind>::MinValue => {
                let obj = &self.owner;
                obj.get_min()
            }
            <RangeF64Kind>::MaxValue => {
                let obj = &self.owner;
                obj.get_max()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <RangeF64Kind>::Value => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_value(val);
            }
            <RangeF64Kind>::Ratio => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_as_ratio(val);
            }
            <RangeF64Kind>::MinValue => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_min(val);
            }
            <RangeF64Kind>::MaxValue => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_max(val);
            }
        }
    }
}
impl IPropertyData for RangeF64Data {
    type Target = Range;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <RangeF64Kind>::Value => NodePath::from("value"),
            <RangeF64Kind>::Ratio => NodePath::from("ratio"),
            <RangeF64Kind>::MinValue => NodePath::from("min_value"),
            <RangeF64Kind>::MaxValue => NodePath::from("max_value"),
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
impl TryFromPathAndObject for RangeF64Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Range>()
            .ok()
            .and_then(|owner| {
                match path {
                    "value" => {
                        Some(Self {
                            property: <RangeF64Kind>::Value,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "ratio" => {
                        Some(Self {
                            property: <RangeF64Kind>::Ratio,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "min_value" => {
                        Some(Self {
                            property: <RangeF64Kind>::MinValue,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "max_value" => {
                        Some(Self {
                            property: <RangeF64Kind>::MaxValue,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoRange<Marker = ()> {
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
impl<Class: Inherits<Range> + Inherits<Object>> DoRange<()> for Gd<Class> {
    fn do_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RangeF64Data {
            property: <RangeF64Kind>::Value,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Range>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RangeF64Data {
            property: <RangeF64Kind>::Ratio,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Range>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_min_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RangeF64Data {
            property: <RangeF64Kind>::MinValue,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Range>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_max_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = RangeF64Data {
            property: <RangeF64Kind>::MaxValue,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Range>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<T: WithBaseField + Inherits<Range> + Inherits<Object>> DoRange<BaseMarker> for T {
    fn do_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Range> = self.to_gd().upcast();
        let data = RangeF64Data {
            property: <RangeF64Kind>::Value,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_ratio(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Range> = self.to_gd().upcast();
        let data = RangeF64Data {
            property: <RangeF64Kind>::Ratio,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_min_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Range> = self.to_gd().upcast();
        let data = RangeF64Data {
            property: <RangeF64Kind>::MinValue,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_max_value(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<Range> = self.to_gd().upcast();
        let data = RangeF64Data {
            property: <RangeF64Kind>::MaxValue,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct RangeTweener {}
#[godot_api]
impl RangeTweener {
    #[func]
    fn do_value(
        node: Gd<Range>,
        to: f64,
        duration: f64,
    ) -> Gd<<f64 as TyToGdTween>::GdTween> {
        let gd = <f64 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_value(to, duration), gd
        }
    }
    #[func]
    fn do_ratio(
        node: Gd<Range>,
        to: f64,
        duration: f64,
    ) -> Gd<<f64 as TyToGdTween>::GdTween> {
        let gd = <f64 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_ratio(to, duration), gd
        }
    }
    #[func]
    fn do_min_value(
        node: Gd<Range>,
        to: f64,
        duration: f64,
    ) -> Gd<<f64 as TyToGdTween>::GdTween> {
        let gd = <f64 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_min_value(to, duration), gd
        }
    }
    #[func]
    fn do_max_value(
        node: Gd<Range>,
        to: f64,
        duration: f64,
    ) -> Gd<<f64 as TyToGdTween>::GdTween> {
        let gd = <f64 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_max_value(to, duration), gd
        }
    }
}
