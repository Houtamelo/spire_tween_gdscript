use super::*;
#[derive(Debug, Clone)]
pub struct ConfirmationDialogGStringData {
    pub property: ConfirmationDialogGStringKind,
    pub owner: Gd<ConfirmationDialog>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationDialogGStringKind {
    CancelButtonText,
}
impl IProperty<GString> for ConfirmationDialogGStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <ConfirmationDialogGStringKind>::CancelButtonText => {
                let obj = &self.owner;
                obj.get_cancel_button_text()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <ConfirmationDialogGStringKind>::CancelButtonText => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_cancel_button_text(&val);
            }
        }
    }
}
impl IPropertyData for ConfirmationDialogGStringData {
    type Target = ConfirmationDialog;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <ConfirmationDialogGStringKind>::CancelButtonText => {
                NodePath::from("cancel_button_text")
            }
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<ConfirmationDialog>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<ConfirmationDialog>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for ConfirmationDialogGStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<ConfirmationDialog>()
            .ok()
            .and_then(|owner| {
                match path {
                    "cancel_button_text" => {
                        Some(Self {
                            property: <ConfirmationDialogGStringKind>::CancelButtonText,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoConfirmationDialog<Marker = ()> {
    fn do_cancel_button_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<ConfirmationDialog> + Inherits<Object>> DoConfirmationDialog<()>
for Gd<Class> {
    fn do_cancel_button_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = ConfirmationDialogGStringData {
            property: <ConfirmationDialogGStringKind>::CancelButtonText,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<ConfirmationDialog>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<ConfirmationDialog> + Inherits<Object>,
> DoConfirmationDialog<BaseMarker> for T {
    fn do_cancel_button_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<ConfirmationDialog> = self.to_gd().upcast();
        let data = ConfirmationDialogGStringData {
            property: <ConfirmationDialogGStringKind>::CancelButtonText,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
#[derive(GodotClass)]
#[class(base = RefCounted, init)]
pub struct ConfirmationDialogTweener {}
#[godot_api]
impl ConfirmationDialogTweener {
    #[func]
    fn do_cancel_button_text(
        node: Gd<ConfirmationDialog>,
        to: GString,
        duration: f64,
    ) -> Gd<<GString as TyToGdTween>::GdTween> {
        let gd = <GString as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_cancel_button_text(to, duration), gd
        }
    }
}
