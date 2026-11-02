use super::*;
/**This class provides shortcut constructors to create tweens that animate a [LinkButton].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoLinkButton {}
#[godot_api]
impl DoLinkButton {
    /**[b]Behavior: [/b]Tweens the property [member LinkButton.text] over [param duration] seconds.

Optional BBCode handling counts parsed characters and preserves balanced tags in all modes. Configure RichTextLabel.bbcode_enabled separately. Scrambling randomizes unrevealed characters in absolute mode; relative and speed-based modes warn once and ignore scrambling. Relative mode appends/removes characters. Speed-based mode measures character edits per second, excluding tags. Empty custom alphabets warn and use letters/digits. Options can also be changed with SpirePropertyString.set_text_options.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = text)]
    fn r#text(
        node: Gd<LinkButton>,
        to: GString,
        duration: f64,
        #[opt(default = false)]
        rich_text_enabled: bool,
        #[opt(default = ScrambleMode::None)]
        scramble_mode: ScrambleMode,
        #[opt(default = "")]
        scramble_chars: GString,
    ) -> Gd<SpirePropertyString> {
        let tween = node
            .do_text(to, duration)
            .with_text_options(TextOptions {
                rich_text_enabled,
                scramble_mode,
                scramble_chars,
            })
            .register();
        gd_from_native_tween(tween)
    }
}
