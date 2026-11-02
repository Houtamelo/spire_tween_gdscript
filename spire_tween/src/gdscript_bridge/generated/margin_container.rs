use super::*;
/**This class provides shortcut constructors to create tweens that animate a [MarginContainer].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoMarginContainer {}
#[godot_api]
impl DoMarginContainer {
    /// Tweens effective theme margins to (left, right, top, bottom).
    /// Captures on first step and writes rounded local theme overrides.
    /// Signed values are supported. Already registered.
    #[func]
    fn margins(
        node: Gd<MarginContainer>,
        to: Vector4i,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(node.do_margins(to, duration).register())
    }
}
