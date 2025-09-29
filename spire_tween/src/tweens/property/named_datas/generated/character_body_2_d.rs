use super::*;
#[derive(Debug, Clone)]
pub struct CharacterBody2DF32Data {
    pub property: CharacterBody2DF32Kind,
    pub owner: Gd<CharacterBody2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterBody2DF32Kind {
    VelocityX,
    VelocityY,
}
impl IProperty<f32> for CharacterBody2DF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <CharacterBody2DF32Kind>::VelocityX => {
                let obj = &self.owner;
                obj.get_velocity().x
            }
            <CharacterBody2DF32Kind>::VelocityY => {
                let obj = &self.owner;
                obj.get_velocity().y
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <CharacterBody2DF32Kind>::VelocityX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_velocity();
                prop_val.x = val;
                obj.set_velocity(prop_val);
            }
            <CharacterBody2DF32Kind>::VelocityY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_velocity();
                prop_val.y = val;
                obj.set_velocity(prop_val);
            }
        }
    }
}
impl IPropertyData for CharacterBody2DF32Data {
    type Target = CharacterBody2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <CharacterBody2DF32Kind>::VelocityX => NodePath::from("velocity:x"),
            <CharacterBody2DF32Kind>::VelocityY => NodePath::from("velocity:y"),
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
impl TryFromPathAndObject for CharacterBody2DF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<CharacterBody2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "velocity:x" => {
                        Some(Self {
                            property: <CharacterBody2DF32Kind>::VelocityX,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "velocity:y" => {
                        Some(Self {
                            property: <CharacterBody2DF32Kind>::VelocityY,
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
pub trait DoCharacterBody2D<Marker = ()> {
    fn chara_do_velocity_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn chara_do_velocity_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
    fn chara_do_velocity(
        &self,
        end_val: Vector2,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2>>;
}
impl<Class: Inherits<CharacterBody2D> + Inherits<Object>> DoCharacterBody2D<()>
for Gd<Class> {
    fn chara_do_velocity_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CharacterBody2DF32Data {
            property: <CharacterBody2DF32Kind>::VelocityX,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CharacterBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn chara_do_velocity_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = CharacterBody2DF32Data {
            property: <CharacterBody2DF32Kind>::VelocityY,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<CharacterBody2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn chara_do_velocity(
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<CharacterBody2D> + Inherits<Object>,
> DoCharacterBody2D<BaseMarker> for T {
    fn chara_do_velocity_x(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CharacterBody2D> = self.to_gd().upcast();
        let data = CharacterBody2DF32Data {
            property: <CharacterBody2DF32Kind>::VelocityX,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn chara_do_velocity_y(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<CharacterBody2D> = self.to_gd().upcast();
        let data = CharacterBody2DF32Data {
            property: <CharacterBody2DF32Kind>::VelocityY,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn chara_do_velocity(
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
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct CharacterBody2DTweener {}
#[godot_api]
impl CharacterBody2DTweener {
    #[func]
    fn chara_do_velocity_x(
        node: Gd<CharacterBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.chara_do_velocity_x(to, duration), gd
        }
    }
    #[func]
    fn chara_do_velocity_y(
        node: Gd<CharacterBody2D>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.chara_do_velocity_y(to, duration), gd
        }
    }
    #[func]
    fn chara_do_velocity(
        node: Gd<CharacterBody2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<<Vector2 as TyToGdTween>::GdTween> {
        let gd = <Vector2 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.chara_do_velocity(to, duration), gd
        }
    }
}
