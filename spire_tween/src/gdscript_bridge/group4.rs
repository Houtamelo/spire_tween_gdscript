use super::*;

#[godot_api(secondary)]
impl Spire {
    /// Tweens a Vector4i property as (left, right, top, bottom) integer margins.
    /// Captures start on first step. Invalid types warn and return a stopped tween.
    #[func]
    pub fn do_property_margins(
        owner: Option<Gd<Object>>,
        path: NodePath,
        to: Vector4i,
        duration: f64,
    ) -> Option<Gd<SpireMethodFloat>> {
        let owner = owner.log_null_arg(|| "owner")?;
        Some(gd_from_native_tween(do_property_margins(owner, path, to, duration).register()))
    }
}
