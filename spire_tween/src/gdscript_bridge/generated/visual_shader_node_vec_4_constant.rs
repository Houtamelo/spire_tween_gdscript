use super::*;
/**This class provides shortcut constructors to create tweens that animate a [VisualShaderNodeVec4Constant].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoVisualShaderNodeVec4Constant {}
#[godot_api]
impl DoVisualShaderNodeVec4Constant {
    /**Tweens component `x`. Tweens the four numeric components of this vec4 shader constant. Godot exposes its storage as Quaternion; this adapter treats it as Vector4 data, not as a rotation.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = constant_x)]
    fn constant_x(
        node: Gd<VisualShaderNodeVec4Constant>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_constant_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `y`. Tweens the four numeric components of this vec4 shader constant. Godot exposes its storage as Quaternion; this adapter treats it as Vector4 data, not as a rotation.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = constant_y)]
    fn constant_y(
        node: Gd<VisualShaderNodeVec4Constant>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_constant_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `z`. Tweens the four numeric components of this vec4 shader constant. Godot exposes its storage as Quaternion; this adapter treats it as Vector4 data, not as a rotation.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = constant_z)]
    fn constant_z(
        node: Gd<VisualShaderNodeVec4Constant>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_constant_z(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `w`. Tweens the four numeric components of this vec4 shader constant. Godot exposes its storage as Quaternion; this adapter treats it as Vector4 data, not as a rotation.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = constant_w)]
    fn constant_w(
        node: Gd<VisualShaderNodeVec4Constant>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_constant_w(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens the four numeric components of this vec4 shader constant. Godot exposes its storage as Quaternion; this adapter treats it as Vector4 data, not as a rotation.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = constant)]
    fn r#constant(
        node: Gd<VisualShaderNodeVec4Constant>,
        to: Vector4,
        duration: f64,
    ) -> Gd<SpirePropertyVec4> {
        let tween = node.do_constant(to, duration).register();
        gd_from_native_tween(tween)
    }
}
