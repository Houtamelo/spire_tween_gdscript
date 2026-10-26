use super::*;
/**This class provides shortcut constructors to create tweens that animate a [ScrollContainer].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoScrollContainer {}
#[godot_api]
impl DoScrollContainer {
    /**[b]Behavior: [/b]Tweens the property [member ScrollContainer.scroll_horizontal] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_horizontal)]
    fn r#scroll_horizontal(
        node: Gd<ScrollContainer>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_scroll_horizontal(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member ScrollContainer.scroll_vertical] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_vertical)]
    fn r#scroll_vertical(
        node: Gd<ScrollContainer>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_scroll_vertical(to, duration).register();
        gd_from_native_tween(tween)
    }
    /// Normalized scroll using live scrollbar min/max/page. X: left=0,
    /// right=1; Y: bottom=0, top=1. Empty ranges stay at minimum. Axis variants
    /// preserve the other scrollbar. Invalid targets warn and clamp to 0-1.
    #[func]
    fn normalized_position(
        node: Gd<ScrollContainer>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(node.do_normalized_position(to, duration).register())
    }
    /// Normalized scroll using live scrollbar min/max/page. X: left=0,
    /// right=1; Y: bottom=0, top=1. Empty ranges stay at minimum. Axis variants
    /// preserve the other scrollbar. Invalid targets warn and clamp to 0-1.
    #[func]
    fn horizontal_normalized_position(
        node: Gd<ScrollContainer>,
        to: f64,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node.do_horizontal_normalized_position(to, duration).register(),
        )
    }
    /// Normalized scroll using live scrollbar min/max/page. X: left=0,
    /// right=1; Y: bottom=0, top=1. Empty ranges stay at minimum. Axis variants
    /// preserve the other scrollbar. Invalid targets warn and clamp to 0-1.
    #[func]
    fn vertical_normalized_position(
        node: Gd<ScrollContainer>,
        to: f64,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            node.do_vertical_normalized_position(to, duration).register(),
        )
    }
}
