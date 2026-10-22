use super::*;
/**This class provides shortcut constructors to create tweens that animate a [ShaderMaterial].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoShaderMaterial {}
#[godot_api]
impl DoShaderMaterial {
    /// Tweens a Color shader uniform's alpha, preserving current RGB each
    /// write. Missing/non-Color uniforms return null with an error.
    #[func]
    fn parameter_color_a(
        node: Gd<ShaderMaterial>,
        parameter: StringName,
        to: f64,
        duration: f64,
    ) -> Option<Gd<SpirePropertyFloat>> {
        let tween = do_shader_parameter_alpha(node, parameter, to, duration)
            .log_if_err()?;
        Some(gd_from_native_tween(tween.register()))
    }
    /// Tweens an existing shader uniform of the indicated type. Reads its
    /// start on first processing. Missing/incompatible uniforms return null
    /// with a diagnostic. Retains the material; shared users see the changes.
    /// Without an override, reads the renderer default. Initialize the value
    /// explicitly in headless mode, where defaults are unavailable. Replacing
    /// the Shader resource cancels existing uniform tweens.
    #[func]
    fn parameter_float(
        node: Gd<ShaderMaterial>,
        parameter: StringName,
        to: f64,
        duration: f64,
    ) -> Option<Gd<SpirePropertyFloat>> {
        let tween = do_shader_parameter(node, parameter, to, duration).log_if_err()?;
        Some(gd_from_native_tween(tween.register()))
    }
    /// Tweens an existing shader uniform of the indicated type. Reads its
    /// start on first processing. Missing/incompatible uniforms return null
    /// with a diagnostic. Retains the material; shared users see the changes.
    /// Without an override, reads the renderer default. Initialize the value
    /// explicitly in headless mode, where defaults are unavailable. Replacing
    /// the Shader resource cancels existing uniform tweens.
    #[func]
    fn parameter_int(
        node: Gd<ShaderMaterial>,
        parameter: StringName,
        to: i64,
        duration: f64,
    ) -> Option<Gd<SpirePropertyInt>> {
        let tween = do_shader_parameter(node, parameter, to, duration).log_if_err()?;
        Some(gd_from_native_tween(tween.register()))
    }
    /// Tweens an existing shader uniform of the indicated type. Reads its
    /// start on first processing. Missing/incompatible uniforms return null
    /// with a diagnostic. Retains the material; shared users see the changes.
    /// Without an override, reads the renderer default. Initialize the value
    /// explicitly in headless mode, where defaults are unavailable. Replacing
    /// the Shader resource cancels existing uniform tweens.
    #[func]
    fn parameter_vec2(
        node: Gd<ShaderMaterial>,
        parameter: StringName,
        to: Vector2,
        duration: f64,
    ) -> Option<Gd<SpirePropertyVec2>> {
        let tween = do_shader_parameter(node, parameter, to, duration).log_if_err()?;
        Some(gd_from_native_tween(tween.register()))
    }
    /// Tweens an existing shader uniform of the indicated type. Reads its
    /// start on first processing. Missing/incompatible uniforms return null
    /// with a diagnostic. Retains the material; shared users see the changes.
    /// Without an override, reads the renderer default. Initialize the value
    /// explicitly in headless mode, where defaults are unavailable. Replacing
    /// the Shader resource cancels existing uniform tweens.
    #[func]
    fn parameter_vec3(
        node: Gd<ShaderMaterial>,
        parameter: StringName,
        to: Vector3,
        duration: f64,
    ) -> Option<Gd<SpirePropertyVec3>> {
        let tween = do_shader_parameter(node, parameter, to, duration).log_if_err()?;
        Some(gd_from_native_tween(tween.register()))
    }
    /// Tweens an existing shader uniform of the indicated type. Reads its
    /// start on first processing. Missing/incompatible uniforms return null
    /// with a diagnostic. Retains the material; shared users see the changes.
    /// Without an override, reads the renderer default. Initialize the value
    /// explicitly in headless mode, where defaults are unavailable. Replacing
    /// the Shader resource cancels existing uniform tweens.
    #[func]
    fn parameter_color(
        node: Gd<ShaderMaterial>,
        parameter: StringName,
        to: Color,
        duration: f64,
    ) -> Option<Gd<SpirePropertyColor>> {
        let tween = do_shader_parameter(node, parameter, to, duration).log_if_err()?;
        Some(gd_from_native_tween(tween.register()))
    }
    /// Tweens an existing shader uniform of the indicated type. Reads its
    /// start on first processing. Missing/incompatible uniforms return null
    /// with a diagnostic. Retains the material; shared users see the changes.
    /// Without an override, reads the renderer default. Initialize the value
    /// explicitly in headless mode, where defaults are unavailable. Replacing
    /// the Shader resource cancels existing uniform tweens.
    #[func]
    fn parameter_vec2i(
        node: Gd<ShaderMaterial>,
        parameter: StringName,
        to: Vector2i,
        duration: f64,
    ) -> Option<Gd<SpirePropertyVec2i>> {
        let tween = do_shader_parameter(node, parameter, to, duration).log_if_err()?;
        Some(gd_from_native_tween(tween.register()))
    }
    /// Tweens an existing shader uniform of the indicated type. Reads its
    /// start on first processing. Missing/incompatible uniforms return null
    /// with a diagnostic. Retains the material; shared users see the changes.
    /// Without an override, reads the renderer default. Initialize the value
    /// explicitly in headless mode, where defaults are unavailable. Replacing
    /// the Shader resource cancels existing uniform tweens.
    #[func]
    fn parameter_vec3i(
        node: Gd<ShaderMaterial>,
        parameter: StringName,
        to: Vector3i,
        duration: f64,
    ) -> Option<Gd<SpirePropertyVec3i>> {
        let tween = do_shader_parameter(node, parameter, to, duration).log_if_err()?;
        Some(gd_from_native_tween(tween.register()))
    }
}
