use super::*;
#[derive(Debug, Clone)]
pub struct AspectRatioContainerF32Data {
    pub property: AspectRatioContainerF32Kind,
    pub owner: Gd<AspectRatioContainer>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectRatioContainerF32Kind {
    Ratio,
}
impl IProperty<f32> for AspectRatioContainerF32Data {
    #[inline]
    fn get_property_value(&self) -> f32 {
        match self.property {
            <AspectRatioContainerF32Kind>::Ratio => {
                let obj = &self.owner;
                obj.get_ratio()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: f32) {
        match self.property {
            <AspectRatioContainerF32Kind>::Ratio => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_ratio(val);
            }
        }
    }
}
impl IPropertyData for AspectRatioContainerF32Data {
    type Target = AspectRatioContainer;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AspectRatioContainerF32Kind>::Ratio => NodePath::from("ratio"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<AspectRatioContainer>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<AspectRatioContainer>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for AspectRatioContainerF32Data {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AspectRatioContainer>()
            .ok()
            .and_then(|owner| {
                match path {
                    "ratio" => {
                        Some(Self {
                            property: <AspectRatioContainerF32Kind>::Ratio,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoAspectRatioContainer<Marker = ()> {
    fn container_do_ratio(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>>;
}
impl<Class: Inherits<AspectRatioContainer> + Inherits<Object>> DoAspectRatioContainer<()>
for Gd<Class> {
    fn container_do_ratio(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let data = AspectRatioContainerF32Data {
            property: <AspectRatioContainerF32Kind>::Ratio,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AspectRatioContainer>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<AspectRatioContainer> + Inherits<Object>,
> DoAspectRatioContainer<BaseMarker> for T {
    fn container_do_ratio(
        &self,
        end_val: f32,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<f32>> {
        let owner: Gd<AspectRatioContainer> = self.to_gd().upcast();
        let data = AspectRatioContainerF32Data {
            property: <AspectRatioContainerF32Kind>::Ratio,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<f32>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct AspectRatioContainerTweener {}
#[godot_api]
impl AspectRatioContainerTweener {
    #[func]
    fn container_do_ratio(
        node: Gd<AspectRatioContainer>,
        to: f32,
        duration: f64,
    ) -> Gd<<f32 as TyToGdTween>::GdTween> {
        let gd = <f32 as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.container_do_ratio(to, duration), gd
        }
    }
}
