use super::*;
#[derive(Debug, Clone)]
pub struct OmniLight3DFloatData {
    pub property: OmniLight3DFloatKind,
    pub owner: Gd<OmniLight3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OmniLight3DFloatKind {
    OmniAttenuation,
    OmniRange,
}
impl IProperty<f64> for OmniLight3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <OmniLight3DFloatKind>::OmniAttenuation => {
                let obj = &self.owner;
                (obj.get_omni_attenuation()) as f64
            }
            <OmniLight3DFloatKind>::OmniRange => {
                let obj = &self.owner;
                (obj.get_omni_range()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <OmniLight3DFloatKind>::OmniAttenuation => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_omni_attenuation(val as f32);
            }
            <OmniLight3DFloatKind>::OmniRange => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_omni_range(val as f32);
            }
        }
    }
}
impl IPropertyData for OmniLight3DFloatData {
    type Target = OmniLight3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <OmniLight3DFloatKind>::OmniAttenuation => NodePath::from("omni_attenuation"),
            <OmniLight3DFloatKind>::OmniRange => NodePath::from("omni_range"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<OmniLight3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<OmniLight3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for OmniLight3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<OmniLight3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "omni_attenuation" => {
                        Some(Self {
                            property: <OmniLight3DFloatKind>::OmniAttenuation,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "omni_range" => {
                        Some(Self {
                            property: <OmniLight3DFloatKind>::OmniRange,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoOmniLight3D<Marker = ()> {
    fn do_omni_attenuation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_omni_range(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<Class: Inherits<OmniLight3D> + Inherits<Object>> SpireDoOmniLight3D<()>
for Gd<Class> {
    fn do_omni_attenuation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = OmniLight3DFloatData {
            property: <OmniLight3DFloatKind>::OmniAttenuation,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<OmniLight3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_omni_range(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = OmniLight3DFloatData {
            property: <OmniLight3DFloatKind>::OmniRange,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<OmniLight3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<OmniLight3D> + Inherits<Object>,
> SpireDoOmniLight3D<BaseMarker> for T {
    fn do_omni_attenuation(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<OmniLight3D> = self.to_gd().upcast();
        let data = OmniLight3DFloatData {
            property: <OmniLight3DFloatKind>::OmniAttenuation,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_omni_range(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<OmniLight3D> = self.to_gd().upcast();
        let data = OmniLight3DFloatData {
            property: <OmniLight3DFloatKind>::OmniRange,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
