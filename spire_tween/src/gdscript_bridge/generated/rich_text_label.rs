use super::*;
/**This class provides shortcut constructors to create tweens that animate a [RichTextLabel].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoRichTextLabel {}
#[godot_api]
impl DoRichTextLabel {
    /**Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = normal_font_size)]
    fn r#normal_font_size(
        node: Gd<RichTextLabel>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_normal_font_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = bold_font_size)]
    fn r#bold_font_size(
        node: Gd<RichTextLabel>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_bold_font_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = italics_font_size)]
    fn r#italics_font_size(
        node: Gd<RichTextLabel>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_italics_font_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = bold_italics_font_size)]
    fn r#bold_italics_font_size(
        node: Gd<RichTextLabel>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_bold_italics_font_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = mono_font_size)]
    fn r#mono_font_size(
        node: Gd<RichTextLabel>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_mono_font_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_size)]
    fn r#outline_size(
        node: Gd<RichTextLabel>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_outline_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member RichTextLabel.visible_characters] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = visible_characters)]
    fn r#visible_characters(
        node: Gd<RichTextLabel>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_visible_characters(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `r`. Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = default_color_r)]
    fn default_color_r(
        node: Gd<RichTextLabel>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_default_color_r(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `g`. Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = default_color_g)]
    fn default_color_g(
        node: Gd<RichTextLabel>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_default_color_g(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `b`. Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = default_color_b)]
    fn default_color_b(
        node: Gd<RichTextLabel>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_default_color_b(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `a`. Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = default_color_a)]
    fn default_color_a(
        node: Gd<RichTextLabel>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_default_color_a(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `r`. Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color_r)]
    fn outline_color_r(
        node: Gd<RichTextLabel>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_color_r(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `g`. Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color_g)]
    fn outline_color_g(
        node: Gd<RichTextLabel>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_color_g(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `b`. Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color_b)]
    fn outline_color_b(
        node: Gd<RichTextLabel>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_color_b(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens component `a`. Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color_a)]
    fn outline_color_a(
        node: Gd<RichTextLabel>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_color_a(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member RichTextLabel.visible_ratio] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = visible_ratio)]
    fn r#visible_ratio(
        node: Gd<RichTextLabel>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_visible_ratio(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = default_color)]
    fn r#default_color(
        node: Gd<RichTextLabel>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let tween = node.do_default_color(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**Tweens a RichTextLabel theme style, reading the effective theme value on first processing and writing a local override. Explicit BBCode formatting takes precedence.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_color)]
    fn r#outline_color(
        node: Gd<RichTextLabel>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let tween = node.do_outline_color(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member RichTextLabel.text] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = text)]
    fn r#text(
        node: Gd<RichTextLabel>,
        to: GString,
        duration: f64,
    ) -> Gd<SpirePropertyString> {
        let tween = node.do_text(to, duration).register();
        gd_from_native_tween(tween)
    }
    /// Replaces text with an interpolated integer. Does not parse existing text.
    /// Groups every three digits; an empty separator disables grouping. Replaces
    /// BBCode on RichTextLabel. Returns an already registered method tween.
    #[func]
    fn counter(
        node: Gd<RichTextLabel>,
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
