use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Control].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoControl {}
#[godot_api]
impl DoControl {
    /**[b]Behavior: [/b]Tweens the property [member Control.anchor_bottom] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = anchor_bottom)]
    fn r#anchor_bottom(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_anchor_bottom(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.anchor_left] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = anchor_left)]
    fn r#anchor_left(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_anchor_left(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.anchor_right] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = anchor_right)]
    fn r#anchor_right(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_anchor_right(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.anchor_top] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = anchor_top)]
    fn r#anchor_top(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_anchor_top(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.rotation] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation)]
    fn r#rotation(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_rotation(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.rotation_degrees] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = rotation_degrees)]
    fn r#rotation_degrees(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_rotation_degrees(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Control.custom_minimum_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = custom_minimum_size_x)]
    fn custom_minimum_size_x(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_custom_minimum_size_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Control.custom_minimum_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = custom_minimum_size_y)]
    fn custom_minimum_size_y(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_custom_minimum_size_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Control.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position_x)]
    fn position_x(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_position_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method position_x].
    #[func(rename = move_local_x)]
    fn move_local_x(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::position_x(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Control.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position_y)]
    fn position_y(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_position_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method position_y].
    #[func(rename = move_local_y)]
    fn move_local_y(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::position_y(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Control.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position_x)]
    fn global_position_x(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_position_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method global_position_x].
    #[func(rename = move_x)]
    fn move_x(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::global_position_x(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Control.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position_y)]
    fn global_position_y(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_global_position_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method global_position_y].
    #[func(rename = move_y)]
    fn move_y(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::global_position_y(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Control.pivot_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = pivot_offset_x)]
    fn pivot_offset_x(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_pivot_offset_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Control.pivot_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = pivot_offset_y)]
    fn pivot_offset_y(
        node: Gd<Control>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_pivot_offset_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Control.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale_x)]
    fn scale_x(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_scale_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Control.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale_y)]
    fn scale_y(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_scale_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Control.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_x)]
    fn size_x(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_size_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Control.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size_y)]
    fn size_y(node: Gd<Control>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_size_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.custom_minimum_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = custom_minimum_size)]
    fn r#custom_minimum_size(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let tween = node.do_custom_minimum_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = position)]
    fn r#position(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let tween = node.do_position(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method position].
    #[func(rename = move_local)]
    fn r#move_local(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        Self::r#position(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.global_position] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = global_position)]
    fn r#global_position(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let tween = node.do_global_position(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method global_position].
    #[func(rename = move)]
    fn r#move(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        Self::r#global_position(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.pivot_offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = pivot_offset)]
    fn r#pivot_offset(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let tween = node.do_pivot_offset(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scale)]
    fn r#scale(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let tween = node.do_scale(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Control.size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = size)]
    fn r#size(
        node: Gd<Control>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let tween = node.do_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    #[func]
    fn shake(
        node: Gd<Control>,
        radius_min: real,
        radius_max: real,
        vibratio: real,
        frequency: f64,
        duration: f64,
    ) -> Gd<SpireMethodFloat> {
        let inner = node
            .do_shake(radius_min, radius_max, vibratio, frequency, duration)
            .register();
        gd_from_native_tween(inner)
    }
}
