use super::*;
#[derive(Debug, Clone)]
pub struct SubViewportI32Data {
    pub property: SubViewportI32Kind,
    pub owner: Gd<SubViewport>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubViewportI32Kind {
    SizeX,
    SizeY,
    Size2DOverrideX,
    Size2DOverrideY,
}
impl IProperty<i32> for SubViewportI32Data {
    fn get_property_value(&self) -> i32 {
        match self.property {
            <SubViewportI32Kind>::SizeX => {
                let obj = &self.owner;
                obj.get_size().x
            }
            <SubViewportI32Kind>::SizeY => {
                let obj = &self.owner;
                obj.get_size().y
            }
            <SubViewportI32Kind>::Size2DOverrideX => {
                let obj = &self.owner;
                obj.get_size_2d_override().x
            }
            <SubViewportI32Kind>::Size2DOverrideY => {
                let obj = &self.owner;
                obj.get_size_2d_override().y
            }
        }
    }
    fn set_property_value(&mut self, value: i32) {
        match self.property {
            <SubViewportI32Kind>::SizeX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.x = val;
                obj.set_size(prop_val);
            }
            <SubViewportI32Kind>::SizeY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size();
                prop_val.y = val;
                obj.set_size(prop_val);
            }
            <SubViewportI32Kind>::Size2DOverrideX => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size_2d_override();
                prop_val.x = val;
                obj.set_size_2d_override(prop_val);
            }
            <SubViewportI32Kind>::Size2DOverrideY => {
                let obj = &mut self.owner;
                let val = value;
                let mut prop_val = obj.get_size_2d_override();
                prop_val.y = val;
                obj.set_size_2d_override(prop_val);
            }
        }
    }
}
impl IPropertyData for SubViewportI32Data {
    type Target = SubViewport;
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <SubViewportI32Kind>::SizeX => NodePath::from("size:x"),
            <SubViewportI32Kind>::SizeY => NodePath::from("size:y"),
            <SubViewportI32Kind>::Size2DOverrideX => NodePath::from("size_2d_override:x"),
            <SubViewportI32Kind>::Size2DOverrideY => NodePath::from("size_2d_override:y"),
        }
    }
    fn get_owner(&self) -> &ObjectOrNode { self.owner.clone() }
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Ok(casted) = owner.try_cast::<SubViewport>() {
            self.owner = casted;
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for SubViewportI32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object.try_cast::<SubViewport>().ok().and_then(|owner| {
            match path {
                "size:x" => {
                    Some(Self {
                        property: <SubViewportI32Kind>::SizeX,
                        owner,
                    })
                }
                "size:y" => {
                    Some(Self {
                        property: <SubViewportI32Kind>::SizeY,
                        owner,
                    })
                }
                "size_2d_override:x" => {
                    Some(Self {
                        property: <SubViewportI32Kind>::Size2DOverrideX,
                        owner,
                    })
                }
                "size_2d_override:y" => {
                    Some(Self {
                        property: <SubViewportI32Kind>::Size2DOverrideY,
                        owner,
                    })
                }
                _ => None,
            }
        })
    }
}
#[derive(Debug, Clone)]
pub struct SubViewportVector2IData {
    pub property: SubViewportVector2IKind,
    pub owner: Gd<SubViewport>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubViewportVector2IKind {
    Size,
    Size2DOverride,
}
impl IProperty<Vector2i> for SubViewportVector2IData {
    fn get_property_value(&self) -> Vector2i {
        match self.property {
            <SubViewportVector2IKind>::Size => {
                let obj = &self.owner;
                obj.get_size()
            }
            <SubViewportVector2IKind>::Size2DOverride => {
                let obj = &self.owner;
                obj.get_size_2d_override()
            }
        }
    }
    fn set_property_value(&mut self, value: Vector2i) {
        match self.property {
            <SubViewportVector2IKind>::Size => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_size(val);
            }
            <SubViewportVector2IKind>::Size2DOverride => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_size_2d_override(val);
            }
        }
    }
}
impl IPropertyData for SubViewportVector2IData {
    type Target = SubViewport;
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <SubViewportVector2IKind>::Size => NodePath::from("size"),
            <SubViewportVector2IKind>::Size2DOverride => NodePath::from("size_2d_override"),
        }
    }
    fn get_owner(&self) -> &ObjectOrNode { self.owner.clone() }
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Ok(casted) = owner.try_cast::<SubViewport>() {
            self.owner = casted;
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for SubViewportVector2IData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object.try_cast::<SubViewport>().ok().and_then(|owner| {
            match path {
                "size" => {
                    Some(Self {
                        property: <SubViewportVector2IKind>::Size,
                        owner,
                    })
                }
                "size_2d_override" => {
                    Some(Self {
                        property: <SubViewportVector2IKind>::Size2DOverride,
                        owner,
                    })
                }
                _ => None,
            }
        })
    }
}
pub trait DoSubViewport<Marker = ()> {
    fn subview_do_size_x(&self, end_val: i32, duration: f64) -> SpireTween<LerpPropertyData<i32>>;
    fn subview_do_size_y(&self, end_val: i32, duration: f64) -> SpireTween<LerpPropertyData<i32>>;
    fn subview_do_size_2d_override_x(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>>;
    fn subview_do_size_2d_override_y(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>>;
    fn subview_do_size(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>>;
    fn subview_do_size_2d_override(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>>;
}
impl<Class: Inherits<SubViewport> + Inherits<Object>> DoSubViewport<()> for Gd<Class> {
    fn subview_do_size_x(&self, end_val: i32, duration: f64) -> SpireTween<LerpPropertyData<i32>> {
        let data = SubViewportI32Data {
            property: <SubViewportI32Kind>::SizeX,
            owner: self.clone().upcast(),
        };
        SpireTween::<LerpPropertyData<i32>>::new(
            data.into(),
            Evaluator::Static(end_val),
            duration,
            AutoPlay(true),
        )
        .maybe_bound(self)
    }
    fn subview_do_size_y(&self, end_val: i32, duration: f64) -> SpireTween<LerpPropertyData<i32>> {
        let data = SubViewportI32Data {
            property: <SubViewportI32Kind>::SizeY,
            owner: self.clone().upcast(),
        };
        SpireTween::<LerpPropertyData<i32>>::new(
            data.into(),
            Evaluator::Static(end_val),
            duration,
            AutoPlay(true),
        )
        .maybe_bound(self)
    }
    fn subview_do_size_2d_override_x(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let data = SubViewportI32Data {
            property: <SubViewportI32Kind>::Size2DOverrideX,
            owner: self.clone().upcast(),
        };
        SpireTween::<LerpPropertyData<i32>>::new(
            data.into(),
            Evaluator::Static(end_val),
            duration,
            AutoPlay(true),
        )
        .maybe_bound(self)
    }
    fn subview_do_size_2d_override_y(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let data = SubViewportI32Data {
            property: <SubViewportI32Kind>::Size2DOverrideY,
            owner: self.clone().upcast(),
        };
        SpireTween::<LerpPropertyData<i32>>::new(
            data.into(),
            Evaluator::Static(end_val),
            duration,
            AutoPlay(true),
        )
        .maybe_bound(self)
    }
    fn subview_do_size(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>> {
        let data = SubViewportVector2IData {
            property: <SubViewportVector2IKind>::Size,
            owner: self.clone().upcast(),
        };
        SpireTween::<LerpPropertyData<Vector2i>>::new(
            data.into(),
            Evaluator::Static(end_val),
            duration,
            AutoPlay(true),
        )
        .maybe_bound(self)
    }
    fn subview_do_size_2d_override(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>> {
        let data = SubViewportVector2IData {
            property: <SubViewportVector2IKind>::Size2DOverride,
            owner: self.clone().upcast(),
        };
        SpireTween::<LerpPropertyData<Vector2i>>::new(
            data.into(),
            Evaluator::Static(end_val),
            duration,
            AutoPlay(true),
        )
        .maybe_bound(self)
    }
}
impl<T: WithBaseField + Inherits<SubViewport> + Inherits<Object>> DoSubViewport<BaseMarker> for T {
    fn subview_do_size_x(&self, end_val: i32, duration: f64) -> SpireTween<LerpPropertyData<i32>> {
        let owner: Gd<SubViewport> = self.to_gd().upcast();
        let data = SubViewportI32Data {
            property: <SubViewportI32Kind>::SizeX,
            owner: owner.clone(),
        };
        SpireTween::<LerpPropertyData<i32>>::new(
            data.into(),
            Evaluator::Static(end_val),
            duration,
            AutoPlay(true),
        )
        .maybe_bound(&owner)
    }
    fn subview_do_size_y(&self, end_val: i32, duration: f64) -> SpireTween<LerpPropertyData<i32>> {
        let owner: Gd<SubViewport> = self.to_gd().upcast();
        let data = SubViewportI32Data {
            property: <SubViewportI32Kind>::SizeY,
            owner: owner.clone(),
        };
        SpireTween::<LerpPropertyData<i32>>::new(
            data.into(),
            Evaluator::Static(end_val),
            duration,
            AutoPlay(true),
        )
        .maybe_bound(&owner)
    }
    fn subview_do_size_2d_override_x(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let owner: Gd<SubViewport> = self.to_gd().upcast();
        let data = SubViewportI32Data {
            property: <SubViewportI32Kind>::Size2DOverrideX,
            owner: owner.clone(),
        };
        SpireTween::<LerpPropertyData<i32>>::new(
            data.into(),
            Evaluator::Static(end_val),
            duration,
            AutoPlay(true),
        )
        .maybe_bound(&owner)
    }
    fn subview_do_size_2d_override_y(
        &self,
        end_val: i32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i32>> {
        let owner: Gd<SubViewport> = self.to_gd().upcast();
        let data = SubViewportI32Data {
            property: <SubViewportI32Kind>::Size2DOverrideY,
            owner: owner.clone(),
        };
        SpireTween::<LerpPropertyData<i32>>::new(
            data.into(),
            Evaluator::Static(end_val),
            duration,
            AutoPlay(true),
        )
        .maybe_bound(&owner)
    }
    fn subview_do_size(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>> {
        let owner: Gd<SubViewport> = self.to_gd().upcast();
        let data = SubViewportVector2IData {
            property: <SubViewportVector2IKind>::Size,
            owner: owner.clone(),
        };
        SpireTween::<LerpPropertyData<Vector2i>>::new(
            data.into(),
            Evaluator::Static(end_val),
            duration,
            AutoPlay(true),
        )
        .maybe_bound(&owner)
    }
    fn subview_do_size_2d_override(
        &self,
        end_val: Vector2i,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<Vector2i>> {
        let owner: Gd<SubViewport> = self.to_gd().upcast();
        let data = SubViewportVector2IData {
            property: <SubViewportVector2IKind>::Size2DOverride,
            owner: owner.clone(),
        };
        SpireTween::<LerpPropertyData<Vector2i>>::new(
            data.into(),
            Evaluator::Static(end_val),
            duration,
            AutoPlay(true),
        )
        .maybe_bound(&owner)
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct SubViewportTweener {}
#[godot_api]
impl SubViewportTweener {
    #[func]
    fn subview_do_size_x(
        node: Gd<SubViewport>,
        to: i32,
        duration: f64,
    ) -> Gd<<i32 as TyToGdTween>::GdTween> {
        register_gd_handle! {
            node.subview_do_size_x(to, duration), < i32 as TyToGdTween >
            ::GdTween::new_gd()
        }
    }
    #[func]
    fn subview_do_size_y(
        node: Gd<SubViewport>,
        to: i32,
        duration: f64,
    ) -> Gd<<i32 as TyToGdTween>::GdTween> {
        register_gd_handle! {
            node.subview_do_size_y(to, duration), < i32 as TyToGdTween >
            ::GdTween::new_gd()
        }
    }
    #[func]
    fn subview_do_size_2d_override_x(
        node: Gd<SubViewport>,
        to: i32,
        duration: f64,
    ) -> Gd<<i32 as TyToGdTween>::GdTween> {
        register_gd_handle! {
            node.subview_do_size_2d_override_x(to, duration), < i32 as TyToGdTween >
            ::GdTween::new_gd()
        }
    }
    #[func]
    fn subview_do_size_2d_override_y(
        node: Gd<SubViewport>,
        to: i32,
        duration: f64,
    ) -> Gd<<i32 as TyToGdTween>::GdTween> {
        register_gd_handle! {
            node.subview_do_size_2d_override_y(to, duration), < i32 as TyToGdTween >
            ::GdTween::new_gd()
        }
    }
    #[func]
    fn subview_do_size(
        node: Gd<SubViewport>,
        to: Vector2i,
        duration: f64,
    ) -> Gd<<Vector2i as TyToGdTween>::GdTween> {
        register_gd_handle! {
            node.subview_do_size(to, duration), < Vector2i as TyToGdTween >
            ::GdTween::new_gd()
        }
    }
    #[func]
    fn subview_do_size_2d_override(
        node: Gd<SubViewport>,
        to: Vector2i,
        duration: f64,
    ) -> Gd<<Vector2i as TyToGdTween>::GdTween> {
        register_gd_handle! {
            node.subview_do_size_2d_override(to, duration), < Vector2i as TyToGdTween >
            ::GdTween::new_gd()
        }
    }
}
