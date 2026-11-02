use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Label].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoLabel {}
#[godot_api]
impl DoLabel {
    /**Tweens the effective Label style. Uses LabelSettings when assigned (shared settings affect every user); otherwise reads the effective theme value and writes a local theme override. Does not change modulate.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_size)]
    fn r#font_size(node: Gd<Label>, to: i64, duration: f64) -> Gd<SpirePropertyInt> {
        let tween = node.do_font_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens the effective Label style. Uses LabelSettings when assigned (shared settings affect every user); otherwise reads the effective theme value and writes a local theme override. Does not change modulate.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_size)]
    fn r#outline_size(node: Gd<Label>, to: i64, duration: f64) -> Gd<SpirePropertyInt> {
        let tween = node.do_outline_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label.max_lines_visible] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = max_lines_visible)]
    fn r#max_lines_visible(
        node: Gd<Label>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_max_lines_visible(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label.visible_characters] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = visible_characters)]
    fn r#visible_characters(
        node: Gd<Label>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_visible_characters(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `r`. Tweens the effective Label style. Uses LabelSettings when assigned (shared settings affect every user); otherwise reads the effective theme value and writes a local theme override. Does not change modulate.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_color_r)]
    fn font_color_r(node: Gd<Label>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_font_color_r(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `g`. Tweens the effective Label style. Uses LabelSettings when assigned (shared settings affect every user); otherwise reads the effective theme value and writes a local theme override. Does not change modulate.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_color_g)]
    fn font_color_g(node: Gd<Label>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_font_color_g(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `b`. Tweens the effective Label style. Uses LabelSettings when assigned (shared settings affect every user); otherwise reads the effective theme value and writes a local theme override. Does not change modulate.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_color_b)]
    fn font_color_b(node: Gd<Label>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_font_color_b(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `a`. Tweens the effective Label style. Uses LabelSettings when assigned (shared settings affect every user); otherwise reads the effective theme value and writes a local theme override. Does not change modulate.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_color_a)]
    fn font_color_a(node: Gd<Label>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_font_color_a(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `r`. Tweens the effective Label style. Uses LabelSettings when assigned (shared settings affect every user); otherwise reads the effective theme value and writes a local theme override. Does not change modulate.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color_r)]
    fn outline_color_r(
        node: Gd<Label>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_color_r(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `g`. Tweens the effective Label style. Uses LabelSettings when assigned (shared settings affect every user); otherwise reads the effective theme value and writes a local theme override. Does not change modulate.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color_g)]
    fn outline_color_g(
        node: Gd<Label>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_color_g(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `b`. Tweens the effective Label style. Uses LabelSettings when assigned (shared settings affect every user); otherwise reads the effective theme value and writes a local theme override. Does not change modulate.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color_b)]
    fn outline_color_b(
        node: Gd<Label>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_color_b(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `a`. Tweens the effective Label style. Uses LabelSettings when assigned (shared settings affect every user); otherwise reads the effective theme value and writes a local theme override. Does not change modulate.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color_a)]
    fn outline_color_a(
        node: Gd<Label>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_color_a(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label.visible_ratio] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = visible_ratio)]
    fn r#visible_ratio(
        node: Gd<Label>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_visible_ratio(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens the effective Label style. Uses LabelSettings when assigned (shared settings affect every user); otherwise reads the effective theme value and writes a local theme override. Does not change modulate.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_color)]
    fn r#font_color(
        node: Gd<Label>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let tween = node.do_font_color(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens the effective Label style. Uses LabelSettings when assigned (shared settings affect every user); otherwise reads the effective theme value and writes a local theme override. Does not change modulate.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color)]
    fn r#outline_color(
        node: Gd<Label>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let tween = node.do_outline_color(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label.text] over [param duration] seconds.

Optional BBCode handling counts parsed characters and preserves balanced tags in all modes. Configure RichTextLabel.bbcode_enabled separately. Scrambling randomizes unrevealed characters in absolute mode; relative and speed-based modes warn once and ignore scrambling. Relative mode appends/removes characters. Speed-based mode measures character edits per second, excluding tags. Empty custom alphabets warn and use letters/digits. Options can also be changed with SpirePropertyString.set_text_options.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = text)]
    fn r#text(
        node: Gd<Label>,
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
    /// Replaces text with an interpolated integer. Does not parse existing text.
    /// Groups every three digits; an empty separator disables grouping. Replaces
    /// BBCode on RichTextLabel. Returns an already registered method tween.
    #[func]
    fn counter(
        node: Gd<Label>,
        from: i64,
        to: i64,
        duration: f64,
        #[opt(default = ",")]
        thousands_separator: GString,
    ) -> Gd<SpireMethodInt> {
        gd_from_native_tween(
            node.do_counter(from, to, duration, thousands_separator).register(),
        )
    }
}
