use super::*;
/**This class provides shortcut constructors to create tweens that animate a [CharacterBody2D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoCharacterBody2D {}
#[godot_api]
impl DoCharacterBody2D {
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member CharacterBody2D.velocity] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = velocity_x)]
    fn velocity_x(
        node: Gd<CharacterBody2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_character_velocity_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member CharacterBody2D.velocity] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = velocity_y)]
    fn velocity_y(
        node: Gd<CharacterBody2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_character_velocity_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member CharacterBody2D.velocity] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = velocity)]
    fn r#velocity(
        node: Gd<CharacterBody2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVec2> {
        let tween = node.do_character_velocity(to, duration).register();
        gd_from_native_tween(tween)
    }
}
