use super::*;
#[derive(Debug, Clone)]
pub struct CharacterBody3DFloatData {
    pub property: CharacterBody3DFloatKind,
    pub owner: Gd<CharacterBody3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBody3DFloatKind {
    VelocityX,
    VelocityY,
    VelocityZ,
}
impl IProperty<f64> for CharacterBody3DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <CharacterBody3DFloatKind>::VelocityX => {
                let obj = &self.owner;
                (obj.get_velocity().x) as f64
            }
            <CharacterBody3DFloatKind>::VelocityY => {
                let obj = &self.owner;
                (obj.get_velocity().y) as f64
            }
            <CharacterBody3DFloatKind>::VelocityZ => {
                let obj = &self.owner;
                (obj.get_velocity().z) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <CharacterBody3DFloatKind>::VelocityX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_velocity();
                prop_val.x = val as f32;
                obj.set_velocity(prop_val);
            }
            <CharacterBody3DFloatKind>::VelocityY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_velocity();
                prop_val.y = val as f32;
                obj.set_velocity(prop_val);
            }
            <CharacterBody3DFloatKind>::VelocityZ => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_velocity();
                prop_val.z = val as f32;
                obj.set_velocity(prop_val);
            }
        }
    }
}
impl IPropertyData for CharacterBody3DFloatData {
    type Target = CharacterBody3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CharacterBody3DFloatKind>::VelocityX => NodePath::from("velocity:x"),
            <CharacterBody3DFloatKind>::VelocityY => NodePath::from("velocity:y"),
            <CharacterBody3DFloatKind>::VelocityZ => NodePath::from("velocity:z"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<CharacterBody3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<CharacterBody3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for CharacterBody3DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CharacterBody3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "velocity:x" => {
                        Some(Self {
                            property: <CharacterBody3DFloatKind>::VelocityX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "velocity:y" => {
                        Some(Self {
                            property: <CharacterBody3DFloatKind>::VelocityY,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "velocity:z" => {
                        Some(Self {
                            property: <CharacterBody3DFloatKind>::VelocityZ,
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
pub struct CharacterBody3DVector3Data {
    pub property: CharacterBody3DVector3Kind,
    pub owner: Gd<CharacterBody3D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBody3DVector3Kind {
    Velocity,
}
impl IProperty<Vector3> for CharacterBody3DVector3Data {
    #[inline]
    fn get_property_value(&self) -> Vector3 {
        match self.property {
            <CharacterBody3DVector3Kind>::Velocity => {
                let obj = &self.owner;
                obj.get_velocity()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector3) {
        match self.property {
            <CharacterBody3DVector3Kind>::Velocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_velocity(val);
            }
        }
    }
}
impl IPropertyData for CharacterBody3DVector3Data {
    type Target = CharacterBody3D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CharacterBody3DVector3Kind>::Velocity => NodePath::from("velocity"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<CharacterBody3D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<CharacterBody3D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for CharacterBody3DVector3Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CharacterBody3D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "velocity" => {
                        Some(Self {
                            property: <CharacterBody3DVector3Kind>::Velocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoCharacterBody3D<Marker = ()> {
    fn do_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>>;
}
impl<Class: Inherits<CharacterBody3D> + Inherits<Object>> SpireDoCharacterBody3D<()>
for Gd<Class> {
    fn do_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CharacterBody3DFloatData {
            property: <CharacterBody3DFloatKind>::VelocityX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CharacterBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CharacterBody3DFloatData {
            property: <CharacterBody3DFloatKind>::VelocityY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CharacterBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CharacterBody3DFloatData {
            property: <CharacterBody3DFloatKind>::VelocityZ,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CharacterBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let data = CharacterBody3DVector3Data {
            property: <CharacterBody3DVector3Kind>::Velocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CharacterBody3D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<CharacterBody3D> + Inherits<Object>,
> SpireDoCharacterBody3D<BaseMarker> for T {
    fn do_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CharacterBody3D> = self.to_gd().upcast();
        let data = CharacterBody3DFloatData {
            property: <CharacterBody3DFloatKind>::VelocityX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CharacterBody3D> = self.to_gd().upcast();
        let data = CharacterBody3DFloatData {
            property: <CharacterBody3DFloatKind>::VelocityY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_velocity_z(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CharacterBody3D> = self.to_gd().upcast();
        let data = CharacterBody3DFloatData {
            property: <CharacterBody3DFloatKind>::VelocityZ,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_velocity(
        &self,
        end_val: Vector3,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector3>> {
        let owner: Gd<CharacterBody3D> = self.to_gd().upcast();
        let data = CharacterBody3DVector3Data {
            property: <CharacterBody3DVector3Kind>::Velocity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector3>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
