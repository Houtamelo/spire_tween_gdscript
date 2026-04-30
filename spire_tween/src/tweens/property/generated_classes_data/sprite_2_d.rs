use super::*;
#[derive(Debug, Clone)]
pub struct Sprite2DIntData {
    pub property: Sprite2DIntKind,
    pub owner: Gd<Sprite2D>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sprite2DIntKind {
    Frame,
}
impl IProperty<i64> for Sprite2DIntData {
    #[inline]
    fn get_property_value(&self) -> i64 {
        match self.property {
            <Sprite2DIntKind>::Frame => {
                let obj = &self.owner;
                (obj.get_frame()) as i64
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: i64) {
        match self.property {
            <Sprite2DIntKind>::Frame => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_frame(val as i32);
            }
        }
    }
}
impl IPropertyData for Sprite2DIntData {
    type Target = Sprite2D;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <Sprite2DIntKind>::Frame => NodePath::from("frame"),
        }
    }
    #[inline]
    fn get_owner(&self) -> Option<&ObjectOrNode> {
        Some(&self.owner_obj_or_node)
    }
}
impl TryFromPathAndObject for Sprite2DIntData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<Sprite2D>()
            .ok()
            .and_then(|owner| {
                match path {
                    "frame" => {
                        Some(Self {
                            property: <Sprite2DIntKind>::Frame,
                            owner_obj_or_node: ObjectOrNode::Node(
                                owner.clone().upcast(),
                            ),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoSprite2D<Marker = ()> {
    fn do_frame(&self, end_val: i64, duration: f64) -> SpireTween<LerpPropertyData<i64>>;
}
impl<Class: Inherits<Sprite2D> + Inherits<Object>> SpireDoSprite2D<()> for Gd<Class> {
    fn do_frame(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let data = Sprite2DIntData {
            property: <Sprite2DIntKind>::Frame,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<Sprite2D>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<Sprite2D> + Inherits<Object>,
> SpireDoSprite2D<BaseMarker> for T {
    fn do_frame(
        &self,
        end_val: i64,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<i64>> {
        let owner: Gd<Sprite2D> = self.to_gd().upcast();
        let data = Sprite2DIntData {
            property: <Sprite2DIntKind>::Frame,
            owner_obj_or_node: ObjectOrNode::Node(owner.clone().upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<i64>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
