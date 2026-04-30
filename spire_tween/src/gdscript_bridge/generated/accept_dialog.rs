use super::*;
/**This class provides shortcut constructors to create tweens that animate a [AcceptDialog].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoAcceptDialog {}
#[godot_api]
impl DoAcceptDialog {
    /**[b]Behavior: [/b]Tweens the property [member AcceptDialog.dialog_text] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = dialog_text)]
    fn r#dialog_text(
        node: Gd<AcceptDialog>,
        to: GString,
        duration: f64,
    ) -> Gd<SpirePropertyString> {
        let tween = node.do_dialog_text(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member AcceptDialog.ok_button_text] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = ok_button_text)]
    fn r#ok_button_text(
        node: Gd<AcceptDialog>,
        to: GString,
        duration: f64,
    ) -> Gd<SpirePropertyString> {
        let tween = node.do_ok_button_text(to, duration).register();
        gd_from_native_tween(tween)
    }
}
