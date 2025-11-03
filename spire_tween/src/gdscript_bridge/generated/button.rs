use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Button].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoButton {}
#[godot_api]
impl DoButton {
    /**[b]Behavior: [/b]Tweens the property [member Button.text] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = text)]
    fn r#text(node: Gd<Button>, to: GString, duration: f64) -> Gd<SpirePropertyString> {
        let inner = UnsafeCell::new(node.do_text(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyString { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
}
