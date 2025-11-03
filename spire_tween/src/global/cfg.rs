use super::*;

/// This namespace class provides functions to customize the default settings of all
/// tweens created by Spire, project-wide.
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct SpireGlobalSettings {}

#[godot_api]
impl SpireGlobalSettings {
    /// Returns the default easing function used by tweens.
    #[func]
    pub fn get_default_ease() -> Ease { unsafe { DEFAULT_EASE } }

    /// Sets the default easing function used by tweens.
    ///
    /// The default value for this setting is [constant Spire.EASE_LINEAR].
    #[func]
    pub fn set_default_ease(ease: Ease) { unsafe { DEFAULT_EASE = ease } }
}
