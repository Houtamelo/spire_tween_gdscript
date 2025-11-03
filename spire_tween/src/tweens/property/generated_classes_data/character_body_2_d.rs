use super::*;
#[derive(Debug, Clone)]
pub struct CharacterBody2DFloatData {
    pub property: CharacterBody2DFloatKind,
    pub owner: Gd<CharacterBody2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBody2DFloatKind {
    VelocityX,
    VelocityY,
}
impl IProperty<f64> for CharacterBody2DFloatData {
    #[inline]
    fn get_property_value(&self) -> f64 {
        match self.property {
            <CharacterBody2DFloatKind>::VelocityX => {
                let obj = &self.owner;
                (obj.get_velocity().x) as f64
            }
            <CharacterBody2DFloatKind>::VelocityY => {
                let obj = &self.owner;
                (obj.get_velocity().y) as f64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f64) {
        match self.property {
            <CharacterBody2DFloatKind>::VelocityX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_velocity();
                prop_val.x = val as f32;
                obj.set_velocity(prop_val);
            }
            <CharacterBody2DFloatKind>::VelocityY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_velocity();
                prop_val.y = val as f32;
                obj.set_velocity(prop_val);
            }
        }
    }
}
impl IPropertyData for CharacterBody2DFloatData {
    type Target = CharacterBody2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CharacterBody2DFloatKind>::VelocityX => NodePath::from("velocity:x"),
            <CharacterBody2DFloatKind>::VelocityY => NodePath::from("velocity:y"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<CharacterBody2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<CharacterBody2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for CharacterBody2DFloatData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CharacterBody2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "velocity:x" => {
                        Some(Self {
                            property: <CharacterBody2DFloatKind>::VelocityX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "velocity:y" => {
                        Some(Self {
                            property: <CharacterBody2DFloatKind>::VelocityY,
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
pub struct CharacterBody2DVector2Data {
    pub property: CharacterBody2DVector2Kind,
    pub owner: Gd<CharacterBody2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBody2DVector2Kind {
    Velocity,
}
impl IProperty<Vector2> for CharacterBody2DVector2Data {
    #[inline]
    fn get_property_value(&self) -> Vector2 {
        match self.property {
            <CharacterBody2DVector2Kind>::Velocity => {
                let obj = &self.owner;
                obj.get_velocity()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: Vector2) {
        match self.property {
            <CharacterBody2DVector2Kind>::Velocity => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_velocity(val);
            }
        }
    }
}
impl IPropertyData for CharacterBody2DVector2Data {
    type Target = CharacterBody2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CharacterBody2DVector2Kind>::Velocity => NodePath::from("velocity"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<CharacterBody2D>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<CharacterBody2D>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for CharacterBody2DVector2Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CharacterBody2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "velocity" => {
                        Some(Self {
                            property: <CharacterBody2DVector2Kind>::Velocity,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoCharacterBody2D<Marker = ()> {
    fn do_character_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_character_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>>;
    fn do_character_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<Class: Inherits<CharacterBody2D> + Inherits<Object>> SpireDoCharacterBody2D<()>
for Gd<Class> {
    fn do_character_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CharacterBody2DFloatData {
            property: <CharacterBody2DFloatKind>::VelocityX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CharacterBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_character_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let data = CharacterBody2DFloatData {
            property: <CharacterBody2DFloatKind>::VelocityY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CharacterBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_character_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let data = CharacterBody2DVector2Data {
            property: <CharacterBody2DVector2Kind>::Velocity,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CharacterBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<CharacterBody2D> + Inherits<Object>,
> SpireDoCharacterBody2D<BaseMarker> for T {
    fn do_character_velocity_x(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CharacterBody2D> = self.to_gd().upcast();
        let data = CharacterBody2DFloatData {
            property: <CharacterBody2DFloatKind>::VelocityX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_character_velocity_y(
        &self,
        end_val: f64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f64>> {
        let owner: Gd<CharacterBody2D> = self.to_gd().upcast();
        let data = CharacterBody2DFloatData {
            property: <CharacterBody2DFloatKind>::VelocityY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_character_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>> {
        let owner: Gd<CharacterBody2D> = self.to_gd().upcast();
        let data = CharacterBody2DVector2Data {
            property: <CharacterBody2DVector2Kind>::Velocity,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<Vector2>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
