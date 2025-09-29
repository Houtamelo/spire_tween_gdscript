use super::*;
#[derive(Debug, Clone)]
pub struct AcceptDialogGStringData {
    pub property: AcceptDialogGStringKind,
    pub owner: Gd<AcceptDialog>,
    pub owner_obj_or_node: ObjectOrNode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcceptDialogGStringKind {
    DialogText,
    OkButtonText,
}
impl IProperty<GString> for AcceptDialogGStringData {
    #[inline]
    fn get_property_value(&self) -> GString {
        match self.property {
            <AcceptDialogGStringKind>::DialogText => {
                let obj = &self.owner;
                obj.get_text()
            }
            <AcceptDialogGStringKind>::OkButtonText => {
                let obj = &self.owner;
                obj.get_ok_button_text()
            }
        }
    }
    #[inline]
    fn set_property_value(&mut self, value: GString) {
        match self.property {
            <AcceptDialogGStringKind>::DialogText => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_text(&val);
            }
            <AcceptDialogGStringKind>::OkButtonText => {
                let obj = &mut self.owner;
                let val = value;
                obj.set_ok_button_text(&val);
            }
        }
    }
}
impl IPropertyData for AcceptDialogGStringData {
    type Target = AcceptDialog;
    #[inline]
    fn get_property_path(&self) -> NodePath {
        match self.property {
            <AcceptDialogGStringKind>::DialogText => NodePath::from("dialog_text"),
            <AcceptDialogGStringKind>::OkButtonText => NodePath::from("ok_button_text"),
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
impl TryFromPathAndObject for AcceptDialogGStringData {
    fn try_from_path_and_object(path: &str, object: Gd<Object>) -> Option<Self> {
        object
            .try_cast::<AcceptDialog>()
            .ok()
            .and_then(|owner| {
                match path {
                    "dialog_text" => {
                        Some(Self {
                            property: <AcceptDialogGStringKind>::DialogText,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    "ok_button_text" => {
                        Some(Self {
                            property: <AcceptDialogGStringKind>::OkButtonText,
                            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
                            owner,
                        })
                    }
                    _ => None,
                }
            })
    }
}
pub trait DoAcceptDialog<Marker = ()> {
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
impl<Class: Inherits<AcceptDialog> + Inherits<Object>> DoAcceptDialog<()> for Gd<Class> {
    fn do_dialog_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = AcceptDialogGStringData {
            property: <AcceptDialogGStringKind>::DialogText,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AcceptDialog>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_ok_button_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let data = AcceptDialogGStringData {
            property: <AcceptDialogGStringKind>::OkButtonText,
            owner: self.clone().upcast(),
            owner_obj_or_node: ObjectOrNode::Node(
                self.clone().upcast::<AcceptDialog>().upcast::<Node>(),
            ),
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
}
impl<
    T: WithBaseField + Inherits<AcceptDialog> + Inherits<Object>,
> DoAcceptDialog<BaseMarker> for T {
    fn do_dialog_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<AcceptDialog> = self.to_gd().upcast();
        let data = AcceptDialogGStringData {
            property: <AcceptDialogGStringKind>::DialogText,
            owner_obj_or_node: ObjectOrNode::Node(owner.upcast()),
            owner,
        };
        SpireTween::<
            LerpPropertyData<GString>,
        >::new(data.into(), Evaluator::Static(end_val), duration, AutoPlay(true))
    }
    fn do_ok_button_text(
        &self,
        end_val: GString,
        duration: f64,
    ) -> SpireTween<LerpPropertyData<GString>> {
        let owner: Gd<AcceptDialog> = self.to_gd().upcast();
        let data = AcceptDialogGStringData {
            property: <AcceptDialogGStringKind>::OkButtonText,
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
pub struct AcceptDialogTweener {}
#[godot_api]
impl AcceptDialogTweener {
    #[func]
    fn do_dialog_text(
        node: Gd<AcceptDialog>,
        to: GString,
        duration: f64,
    ) -> Gd<<GString as TyToGdTween>::GdTween> {
        let gd = <GString as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_dialog_text(to, duration), gd
        }
    }
    #[func]
    fn do_ok_button_text(
        node: Gd<AcceptDialog>,
        to: GString,
        duration: f64,
    ) -> Gd<<GString as TyToGdTween>::GdTween> {
        let gd = <GString as TyToGdTween>::GdTween::new_gd();
        register_gd_handle! {
            node.do_ok_button_text(to, duration), gd
        }
    }
}
