use super::*;
/**This class provides shortcut constructors to create tweens that animate a [Label3D].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoLabel3D {}
#[godot_api]
impl DoLabel3D {
    /**[b]Behavior: [/b]Tweens the property [member Label3D.font_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = font_size)]
    fn r#font_size(node: Gd<Label3D>, to: i64, duration: f64) -> Gd<SpirePropertyInt> {
        let tween = node.do_font_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label3D.outline_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_size)]
    fn r#outline_size(
        node: Gd<Label3D>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let tween = node.do_outline_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label3D.line_spacing] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = line_spacing)]
    fn r#line_spacing(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_line_spacing(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member Label3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_r)]
    fn modulate_r(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_modulate_r(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method modulate_r].
    #[func(rename = color_r)]
    fn color_r(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_r(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member Label3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_g)]
    fn modulate_g(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_modulate_g(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method modulate_g].
    #[func(rename = color_g)]
    fn color_g(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_g(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member Label3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_b)]
    fn modulate_b(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_modulate_b(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method modulate_b].
    #[func(rename = color_b)]
    fn color_b(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_b(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member Label3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate_a)]
    fn modulate_a(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_modulate_a(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method modulate_a].
    #[func(rename = color_a)]
    fn color_a(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_a(node, to, duration)
    }
    ///Alias for [method modulate_a].
    #[func(rename = fade)]
    fn r#fade(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        Self::modulate_a(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `x` component of the property [member Label3D.offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = offset_x)]
    fn offset_x(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_offset_x(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `y` component of the property [member Label3D.offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = offset_y)]
    fn offset_y(node: Gd<Label3D>, to: f64, duration: f64) -> Gd<SpirePropertyFloat> {
        let tween = node.do_offset_y(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the `r` component of the property [member Label3D.outline_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_modulate_r)]
    fn outline_modulate_r(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_modulate_r(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method outline_modulate_r].
    #[func(rename = outline_color_r)]
    fn outline_color_r(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::outline_modulate_r(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `g` component of the property [member Label3D.outline_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_modulate_g)]
    fn outline_modulate_g(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_modulate_g(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method outline_modulate_g].
    #[func(rename = outline_color_g)]
    fn outline_color_g(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::outline_modulate_g(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `b` component of the property [member Label3D.outline_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_modulate_b)]
    fn outline_modulate_b(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_modulate_b(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method outline_modulate_b].
    #[func(rename = outline_color_b)]
    fn outline_color_b(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::outline_modulate_b(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the `a` component of the property [member Label3D.outline_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_modulate_a)]
    fn outline_modulate_a(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_outline_modulate_a(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method outline_modulate_a].
    #[func(rename = outline_color_a)]
    fn outline_color_a(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        Self::outline_modulate_a(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label3D.pixel_size] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = pixel_size)]
    fn r#pixel_size(
        node: Gd<Label3D>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let tween = node.do_pixel_size(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label3D.offset] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = offset)]
    fn r#offset(node: Gd<Label3D>, to: Vector2, duration: f64) -> Gd<SpirePropertyVec2> {
        let tween = node.do_offset(to, duration).register();
        gd_from_native_tween(tween)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label3D.modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = modulate)]
    fn r#modulate(
        node: Gd<Label3D>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let tween = node.do_modulate(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method modulate].
    #[func(rename = color)]
    fn r#color(node: Gd<Label3D>, to: Color, duration: f64) -> Gd<SpirePropertyColor> {
        Self::r#modulate(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label3D.outline_modulate] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = outline_modulate)]
    fn r#outline_modulate(
        node: Gd<Label3D>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        let tween = node.do_outline_modulate(to, duration).register();
        gd_from_native_tween(tween)
    }
    ///Alias for [method outline_modulate].
    #[func(rename = outline_color)]
    fn r#outline_color(
        node: Gd<Label3D>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        Self::r#outline_modulate(node, to, duration)
    }
    /**[b]Behavior: [/b]Tweens the property [member Label3D.text] over [param duration] seconds.

Optional BBCode handling counts parsed characters and preserves balanced tags in all modes. Configure RichTextLabel.bbcode_enabled separately. Scrambling randomizes unrevealed characters in absolute mode; relative and speed-based modes warn once and ignore scrambling. Relative mode appends/removes characters. Speed-based mode measures character edits per second, excluding tags. Empty custom alphabets warn and use letters/digits. Options can also be changed with SpirePropertyString.set_text_options.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = text)]
    fn r#text(
        node: Gd<Label3D>,
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
            .do_label3d_do_text(to, duration)
            .with_text_options(TextOptions {
                rich_text_enabled,
                scramble_mode,
                scramble_chars,
            })
            .register();
        gd_from_native_tween(tween)
    }
    /// Captures the current color at construction, then adds [param to] minus
    /// that baseline over time. Equivalent to a relative color property tween.
    #[func]
    fn blendable_modulate(
        node: Gd<Label3D>,
        to: Color,
        duration: f64,
    ) -> Gd<SpirePropertyColor> {
        gd_from_native_tween(node.do_blendable_modulate(to, duration).register())
    }
    /// Samples the supplied Gradient with eased progress. Includes alpha by
    /// default; disabling it preserves the target's current alpha on each write.
    /// Uses the Gradient's interpolation mode/color space and samples it live.
    #[func]
    fn gradient_modulate(
        node: Gd<Label3D>,
        gradient: Option<Gd<Gradient>>,
        duration: f64,
        #[opt(default = true)]
        include_alpha: bool,
    ) -> Option<Gd<SpireMethodFloat>> {
        let gradient = gradient.log_null_arg(|| "gradient")?;
        Some(
            gd_from_native_tween(
                node.do_gradient_modulate(gradient, duration, include_alpha).register(),
            ),
        )
    }
    /// Replaces text with an interpolated integer. Does not parse existing text.
    /// Groups every three digits; an empty separator disables grouping. Replaces
    /// BBCode on RichTextLabel. Returns an already registered method tween.
    #[func]
    fn counter(
        node: Gd<Label3D>,
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
