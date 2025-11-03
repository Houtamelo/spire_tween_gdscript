use super::*;
#[derive(Debug, Clone)]
pub struct AcceptDialogStringData {
    pub property: AcceptDialogStringKind,
    pub owner: Gd<AcceptDialog>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcceptDialogStringKind {
    DialogText,
    OkButtonText,
}
impl IProperty<GString> for AcceptDialogStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <AcceptDialogStringKind>::DialogText => {
                let obj = &self.owner;
                obj.get_text()
            }
            <AcceptDialogStringKind>::OkButtonText => {
                let obj = &self.owner;
                obj.get_ok_button_text()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <AcceptDialogStringKind>::DialogText => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
            <AcceptDialogStringKind>::OkButtonText => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_ok_button_text(&val);
            }
        }
    }
}
impl IPropertyData for AcceptDialogStringData {
    type Target = AcceptDialog;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AcceptDialogStringKind>::DialogText => NodePath::from("dialog_text"),
            <AcceptDialogStringKind>::OkButtonText => NodePath::from("ok_button_text"),
        }
    }
    #[inline]
    fn get_owner(&self) -> &ObjectOrNode {
        &self.owner_obj_or_node
    }
    #[inline]
    fn try_set_owner(&mut self, owner: ObjectOrNode) -> bool {
        if let Some(casted) = match owner {
            ObjectOrNode::Object(obj) => obj.try_cast::<AcceptDialog>().ok(),
            ObjectOrNode::Node(obj) => obj.try_cast::<AcceptDialog>().ok(),
        } {
            self.owner = casted;
            self.owner_obj_or_node = ObjectOrNode::Node(casted.upcast());
            true
        } else {
            false
        }
    }
}
impl TryFromPathAndObject for AcceptDialogStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AcceptDialog>()
            .ok()
            .and_then(|owner| {
                match path {
                    "dialog_text" => {
                        Some(Self {
                            property: <AcceptDialogStringKind>::DialogText,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "ok_button_text" => {
                        Some(Self {
                            property: <AcceptDialogStringKind>::OkButtonText,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait SpireDoAcceptDialog<Marker = ()> {
    fn do_dialog_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
    fn do_ok_button_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>>;
}
impl<Class: Inherits<AcceptDialog> + Inherits<Object>> SpireDoAcceptDialog<()>
for Gd<Class> {
    fn do_dialog_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = AcceptDialogStringData {
            property: <AcceptDialogStringKind>::DialogText,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AcceptDialog>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ok_button_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = AcceptDialogStringData {
            property: <AcceptDialogStringKind>::OkButtonText,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AcceptDialog>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
impl<
    T: WithBaseField + Inherits<AcceptDialog> + Inherits<Object>,
> SpireDoAcceptDialog<BaseMarker> for T {
    fn do_dialog_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<AcceptDialog> = self.to_gd().upcast();
        let data = AcceptDialogStringData {
            property: <AcceptDialogStringKind>::DialogText,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
    fn do_ok_button_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<AcceptDialog> = self.to_gd().upcast();
        let data = AcceptDialogStringData {
            property: <AcceptDialogStringKind>::OkButtonText,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration)
    }
}
