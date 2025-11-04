use super::*;
#[derive(Debug, Clone)]
pub struct GeometryInstance3DFloatData {
    pub property: GeometryInstance3DFloatKind,
    pub owner: Gd<GeometryInstance3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryInstance3DFloatKind {
    Transparency,
}
impl IProperty<f64> for GeometryInstance3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <GeometryInstance3DFloatKind>::Transparency => {
                let obj = &self.owner;
                (obj.get_transparency()) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <GeometryInstance3DFloatKind>::Transparency => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_transparency(val as f32);
            }
        }
    }
}
impl IPropertyData for GeometryInstance3DFloatData {
    type Target = GeometryInstance3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <GeometryInstance3DFloatKind>::Transparency => NodePath::from("transparency"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<GeometryInstance3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<GeometryInstance3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for GeometryInstance3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<GeometryInstance3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "transparency" => {
                        Some(Self {
                            property: <GeometryInstance3DFloatKind>::Transparency,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoGeometryInstance3D<Marker = ()> {
    fn do_transparency(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
}
impl<
    Class: Inherits<GeometryInstance3D> + Inherits<Object>,
> SpireDoGeometryInstance3D<()> for Gd<Class> {
    fn do_transparency(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = GeometryInstance3DFloatData {
            property: <GeometryInstance3DFloatKind>::Transparency,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<GeometryInstance3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<GeometryInstance3D> + Inherits<Object>,
> SpireDoGeometryInstance3D<BaseMarker> for T {
    fn do_transparency(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<GeometryInstance3D> = self.to_gd().upcast();
        let data = GeometryInstance3DFloatData {
            property: <GeometryInstance3DFloatKind>::Transparency,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
