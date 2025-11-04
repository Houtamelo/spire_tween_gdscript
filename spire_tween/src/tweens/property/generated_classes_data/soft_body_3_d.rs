use super::*;
#[derive(Debug, Clone)]
pub struct SoftBody3DFloatData {
    pub property: SoftBody3DFloatKind,
    pub owner: Gd<SoftBody3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoftBody3DFloatKind {
    ShrinkingFactor,
    TotalMass,
}
impl IProperty<f64> for SoftBody3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <SoftBody3DFloatKind>::ShrinkingFactor => {
                let obj = &self.owner;
                (obj.get_shrinking_factor()) as f64
            }
            <SoftBody3DFloatKind>::TotalMass => {
                let obj = &self.owner;
                (obj.get_total_mass()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <SoftBody3DFloatKind>::ShrinkingFactor => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_shrinking_factor(val as f32);
            }
            <SoftBody3DFloatKind>::TotalMass => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_total_mass(val as f32);
            }
        }
    }
}
impl IPropertyData for SoftBody3DFloatData {
    type Target = SoftBody3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <SoftBody3DFloatKind>::ShrinkingFactor => NodePath::from("shrinking_factor"),
            <SoftBody3DFloatKind>::TotalMass => NodePath::from("total_mass"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<SoftBody3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<SoftBody3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for SoftBody3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<SoftBody3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "shrinking_factor" => {
                        Some(Self {
                            property: <SoftBody3DFloatKind>::ShrinkingFactor,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "total_mass" => {
                        Some(Self {
                            property: <SoftBody3DFloatKind>::TotalMass,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoSoftBody3D<Marker = ()> {
    fn do_shrinking_factor(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_total_mass(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<Class: Inherits<SoftBody3D> + Inherits<Object>> SpireDoSoftBody3D<()>
for Gd<Class> {
    fn do_shrinking_factor(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = SoftBody3DFloatData {
            property: <SoftBody3DFloatKind>::ShrinkingFactor,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SoftBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_total_mass(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = SoftBody3DFloatData {
            property: <SoftBody3DFloatKind>::TotalMass,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<SoftBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<SoftBody3D> + Inherits<Object>,
> SpireDoSoftBody3D<BaseMarker> for T {
    fn do_shrinking_factor(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<SoftBody3D> = self.to_gd().upcast();
        let data = SoftBody3DFloatData {
            property: <SoftBody3DFloatKind>::ShrinkingFactor,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_total_mass(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<SoftBody3D> = self.to_gd().upcast();
        let data = SoftBody3DFloatData {
            property: <SoftBody3DFloatKind>::TotalMass,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
