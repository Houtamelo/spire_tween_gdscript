use super::*;
/**This class provides shortcut constructors to create tweens that animate a [AudioStreamPlayer2D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoAudioStreamPlayer2D {}
#[godot_api]
impl DoAudioStreamPlayer2D {
    /**[b]Behavior: [/b]Tweens the property [member AudioStreamPlayer2D.volume_db] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = volume_db)]
    fn r#volume_db(
        node: Gd<AudioStreamPlayer2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_volume_db(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member AudioStreamPlayer2D.volume_linear] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = volume_linear)]
    fn r#volume_linear(
        node: Gd<AudioStreamPlayer2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_volume_linear(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member AudioStreamPlayer2D.pitch_scale] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = pitch_scale)]
    fn r#pitch_scale(
        node: Gd<AudioStreamPlayer2D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_pitch_scale(to, duration).register();
        gd_from_native_tween(tween)
    }
}
