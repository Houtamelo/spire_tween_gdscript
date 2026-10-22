use super::*;
/**This class provides shortcut constructors to create tweens that animate a [AudioServer].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoAudioServer {}
#[godot_api]
impl DoAudioServer {
    /// Tweens an existing named bus. Names are resolved each update so
    /// reordering is harmless. Removal/renaming cancels without completion.
    /// Linear gain: 0 = silence, 1 = unity, values above 1 are accepted.
    /// The db variant uses decibels. Invalid names log an error and return null.
    #[func]
    fn bus_volume_db(
        name: StringName,
        to: f64,
        duration: f64,
    ) -> Option<Gd<SpirePropertyFloat>> {
        let tween = do_bus_volume_db(name, to, duration).log_if_err()?;
        Some(gd_from_native_tween(tween.register()))
    }
    /// Tweens an existing named bus. Names are resolved each update so
    /// reordering is harmless. Removal/renaming cancels without completion.
    /// Linear gain: 0 = silence, 1 = unity, values above 1 are accepted.
    /// The db variant uses decibels. Invalid names log an error and return null.
    #[func]
    fn bus_volume_linear(
        name: StringName,
        to: f64,
        duration: f64,
    ) -> Option<Gd<SpirePropertyFloat>> {
        let tween = do_bus_volume_linear(name, to, duration).log_if_err()?;
        Some(gd_from_native_tween(tween.register()))
    }
}
