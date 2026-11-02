use super::*;
/**This class provides shortcut constructors to create tweens that animate a [TextEdit].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoTextEdit {}
#[godot_api]
impl DoTextEdit {
    /**[b]Behavior: [/b]Tweens the property [member TextEdit.minimap_width] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = minimap_width)]
    fn r#minimap_width(
        node: Gd<TextEdit>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_minimap_width(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member TextEdit.scroll_horizontal] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_horizontal)]
    fn r#scroll_horizontal(
        node: Gd<TextEdit>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_scroll_horizontal(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member TextEdit.scroll_vertical] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_vertical)]
    fn r#scroll_vertical(
        node: Gd<TextEdit>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_scroll_vertical(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member TextEdit.text] over [param duration] seconds.

Optional BBCode handling counts parsed characters and preserves balanced tags in all modes. Configure RichTextLabel.bbcode_enabled separately. Scrambling randomizes unrevealed characters in absolute mode; relative and speed-based modes warn once and ignore scrambling. Relative mode appends/removes characters. Speed-based mode measures character edits per second, excluding tags. Empty custom alphabets warn and use letters/digits. Options can also be changed with SpirePropertyString.set_text_options.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = text)]
    fn r#text(
        node: Gd<TextEdit>,
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
    /**[b]Behavior: [/b]Tweens the property [member TextEdit.placeholder_text] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = placeholder_text)]
    fn r#placeholder_text(
        node: Gd<TextEdit>,
        to: GString,
        duration: f64,
    ) -> Gd<SpirePropertyString> {
        let tween = node.do_placeholder_text(to, duration).register();
        gd_from_native_tween(tween)
    }
}
