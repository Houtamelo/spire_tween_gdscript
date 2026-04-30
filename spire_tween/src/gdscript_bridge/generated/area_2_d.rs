use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Area2D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoArea2D {}
#[godot_api]
impl DoArea2D {
    /**[b]Behavior: [/b]Tweens the property [member Area2D.gravity] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = gravity)]
    fn r#gravity(node: Gd<Area2D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_area_gravity(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Area2D.gravity_point_unit_distance] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = gravity_point_unit_distance)]
    fn r#gravity_point_unit_distance(
        node: Gd<Area2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_gravity_point_unit_distance(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Area2D.gravity_direction] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = gravity_direction_x)]
    fn gravity_direction_x(
        node: Gd<Area2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_gravity_direction_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Area2D.gravity_direction] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = gravity_direction_y)]
    fn gravity_direction_y(
        node: Gd<Area2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_gravity_direction_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Area2D.gravity_point_center] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = gravity_point_center_x)]
    fn gravity_point_center_x(
        node: Gd<Area2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_gravity_point_center_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Area2D.gravity_point_center] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = gravity_point_center_y)]
    fn gravity_point_center_y(
        node: Gd<Area2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_gravity_point_center_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Area2D.linear_damp] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = linear_damp)]
    fn r#linear_damp(
        node: Gd<Area2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_area_linear_damp(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Area2D.gravity_direction] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = gravity_direction)]
    fn r#gravity_direction(
        node: Gd<Area2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let tween = node.do_gravity_direction(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Area2D.gravity_point_center] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = gravity_point_center)]
    fn r#gravity_point_center(
        node: Gd<Area2D>,
        to: Vector2,
        duration: f64,
    ) -> Gd<SpirePropertyVector2> {
        let tween = node.do_gravity_point_center(to, duration).register();
        gd_from_native_tween(tween)
    }
}
