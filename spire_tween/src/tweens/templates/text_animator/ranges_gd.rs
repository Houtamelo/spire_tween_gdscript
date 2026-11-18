use super::*;

#[godot_api(secondary)]
impl SpireRichTextLabel {
    /// Animate an inclusive range of logical character indices; last = -1 selects through the text end.
    /// [param duration] is seconds per character per leg. The Char tween finishes
    /// when the last character finishes, including its start delay and requested loops.
    /// [param delay_curve] supplies finite non-negative start delays in multiples
    /// of duration. Its samples are captured when constructing this tween.
    /// Missing indices ignore writes while playback continues.
    /// [b]Returns:[/b] One registered [SpireCharTween] for the whole range.
    #[func(gd_self)]
    pub fn offset_chars(
        this: Gd<Self>,
        first: i64,
        last: i64,
        to: Vector2,
        duration: f64,
        delay_curve: EaseKind,
    ) -> Gd<SpireCharTween> {
        gd_from_native_tween(this.do_offset_chars(first, last, to, duration, delay_curve).register())
    }

    /// Animate an inclusive range of logical character indices; last = -1 selects through the text end.
    /// [param duration] is seconds per character per leg. The Char tween finishes
    /// when the last character finishes, including its start delay and requested loops.
    /// [param delay_curve] supplies finite non-negative start delays in multiples
    /// of duration. Its samples are captured when constructing this tween.
    /// Missing indices ignore writes while playback continues.
    /// [b]Returns:[/b] One registered [SpireCharTween] for the whole range.
    #[func(gd_self)]
    pub fn scale_chars(
        this: Gd<Self>,
        first: i64,
        last: i64,
        to: Vector2,
        duration: f64,
        delay_curve: EaseKind,
    ) -> Gd<SpireCharTween> {
        gd_from_native_tween(this.do_scale_chars(first, last, to, duration, delay_curve).register())
    }

    /// Animate an inclusive range of logical character indices; last = -1 selects through the text end.
    /// [param duration] is seconds per character per leg. The Char tween finishes
    /// when the last character finishes, including its start delay and requested loops.
    /// [param delay_curve] supplies finite non-negative start delays in multiples
    /// of duration. Its samples are captured when constructing this tween.
    /// Missing indices ignore writes while playback continues.
    /// [b]Returns:[/b] One registered [SpireCharTween] for the whole range.
    #[func(gd_self)]
    pub fn color_chars(
        this: Gd<Self>,
        first: i64,
        last: i64,
        to: Color,
        duration: f64,
        delay_curve: EaseKind,
    ) -> Gd<SpireCharTween> {
        gd_from_native_tween(this.do_color_chars(first, last, to, duration, delay_curve).register())
    }

    /// Animate an inclusive range of logical character indices; last = -1 selects through the text end.
    /// [param duration] is seconds per character per leg. The Char tween finishes
    /// when the last character finishes, including its start delay and requested loops.
    /// [param delay_curve] supplies finite non-negative start delays in multiples
    /// of duration. Its samples are captured when constructing this tween.
    /// Missing indices ignore writes while playback continues.
    /// [b]Returns:[/b] One registered [SpireCharTween] for the whole range.
    #[func(gd_self)]
    pub fn fade_chars(
        this: Gd<Self>,
        first: i64,
        last: i64,
        to: f64,
        duration: f64,
        delay_curve: EaseKind,
    ) -> Gd<SpireCharTween> {
        gd_from_native_tween(this.do_fade_chars(first, last, to, duration, delay_curve).register())
    }

    /// Animate an inclusive range of logical character indices; last = -1 selects through the text end.
    /// [param duration] is seconds per character per leg. The Char tween finishes
    /// when the last character finishes, including its start delay and requested loops.
    /// [param delay_curve] supplies finite non-negative start delays in multiples
    /// of duration. Its samples are captured when constructing this tween.
    /// Missing indices ignore writes while playback continues.
    /// [b]Returns:[/b] One registered [SpireCharTween] for the whole range.
    #[func(gd_self)]
    pub fn rotate_chars(
        this: Gd<Self>,
        first: i64,
        last: i64,
        to: Vector3,
        duration: f64,
        delay_curve: EaseKind,
        #[opt(default = RotateMode::Fast)] mode: RotateMode,
    ) -> Gd<SpireCharTween> {
        gd_from_native_tween(
            this.do_rotate_chars(first, last, to, duration, mode, delay_curve)
                .register(),
        )
    }

    /// Animate an inclusive range of logical character indices; last = -1 selects through the text end.
    /// [param duration] is seconds per character per leg. The Char tween finishes
    /// when the last character finishes, including its start delay and requested loops.
    /// [param delay_curve] supplies finite non-negative start delays in multiples
    /// of duration. Its samples are captured when constructing this tween.
    /// Missing indices ignore writes while playback continues.
    /// [b]Returns:[/b] One registered [SpireCharTween] for the whole range.
    #[func(gd_self)]
    pub fn punch_chars_offset(
        this: Gd<Self>,
        first: i64,
        last: i64,
        punch: Vector2,
        duration: f64,
        delay_curve: EaseKind,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 1.0)] elasticity: f64,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireCharTween> {
        gd_from_native_tween(
            this.do_punch_chars_offset(first, last, punch, duration, vibrato, elasticity, blendable, delay_curve)
                .register(),
        )
    }

    /// Animate an inclusive range of logical character indices; last = -1 selects through the text end.
    /// [param duration] is seconds per character per leg. The Char tween finishes
    /// when the last character finishes, including its start delay and requested loops.
    /// [param delay_curve] supplies finite non-negative start delays in multiples
    /// of duration. Its samples are captured when constructing this tween.
    /// Missing indices ignore writes while playback continues.
    /// [b]Returns:[/b] One registered [SpireCharTween] for the whole range.
    #[func(gd_self)]
    pub fn punch_chars_rotation(
        this: Gd<Self>,
        first: i64,
        last: i64,
        punch: Vector3,
        duration: f64,
        delay_curve: EaseKind,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 1.0)] elasticity: f64,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireCharTween> {
        gd_from_native_tween(
            this.do_punch_chars_rotation(first, last, punch, duration, vibrato, elasticity, blendable, delay_curve)
                .register(),
        )
    }

    /// Animate an inclusive range of logical character indices; last = -1 selects through the text end.
    /// [param duration] is seconds per character per leg. The Char tween finishes
    /// when the last character finishes, including its start delay and requested loops.
    /// [param delay_curve] supplies finite non-negative start delays in multiples
    /// of duration. Its samples are captured when constructing this tween.
    /// Missing indices ignore writes while playback continues.
    /// [b]Returns:[/b] One registered [SpireCharTween] for the whole range.
    #[func(gd_self)]
    pub fn punch_chars_scale(
        this: Gd<Self>,
        first: i64,
        last: i64,
        punch: Vector2,
        duration: f64,
        delay_curve: EaseKind,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 1.0)] elasticity: f64,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireCharTween> {
        gd_from_native_tween(
            this.do_punch_chars_scale(first, last, punch, duration, vibrato, elasticity, blendable, delay_curve)
                .register(),
        )
    }

    /// Animate an inclusive range of logical character indices; last = -1 selects through the text end.
    /// [param duration] is seconds per character per leg. The Char tween finishes
    /// when the last character finishes, including its start delay and requested loops.
    /// [param delay_curve] supplies finite non-negative start delays in multiples
    /// of duration. Its samples are captured when constructing this tween.
    /// Missing indices ignore writes while playback continues.
    /// [b]Returns:[/b] One registered [SpireCharTween] for the whole range.
    #[func(gd_self)]
    pub fn shake_chars_offset(
        this: Gd<Self>,
        first: i64,
        last: i64,
        duration: f64,
        strength: Vector2,
        delay_curve: EaseKind,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 90.0)] randomness: real,
        #[opt(default = true)] fade_out: bool,
        #[opt(default = ShakeRandomnessMode::Full)] randomness_mode: ShakeRandomnessMode,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireCharTween> {
        gd_from_native_tween(
            this.do_shake_chars_offset(
                first,
                last,
                duration,
                strength,
                vibrato,
                randomness,
                fade_out,
                randomness_mode,
                blendable,
                delay_curve,
            )
            .register(),
        )
    }

    /// Animate an inclusive range of logical character indices; last = -1 selects through the text end.
    /// [param duration] is seconds per character per leg. The Char tween finishes
    /// when the last character finishes, including its start delay and requested loops.
    /// [param delay_curve] supplies finite non-negative start delays in multiples
    /// of duration. Its samples are captured when constructing this tween.
    /// Missing indices ignore writes while playback continues.
    /// [b]Returns:[/b] One registered [SpireCharTween] for the whole range.
    #[func(gd_self)]
    pub fn shake_chars_rotation(
        this: Gd<Self>,
        first: i64,
        last: i64,
        duration: f64,
        strength: Vector3,
        delay_curve: EaseKind,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 90.0)] randomness: real,
        #[opt(default = true)] fade_out: bool,
        #[opt(default = ShakeRandomnessMode::Full)] randomness_mode: ShakeRandomnessMode,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireCharTween> {
        gd_from_native_tween(
            this.do_shake_chars_rotation(
                first,
                last,
                duration,
                strength,
                vibrato,
                randomness,
                fade_out,
                randomness_mode,
                blendable,
                delay_curve,
            )
            .register(),
        )
    }

    /// Animate an inclusive range of logical character indices; last = -1 selects through the text end.
    /// [param duration] is seconds per character per leg. The Char tween finishes
    /// when the last character finishes, including its start delay and requested loops.
    /// [param delay_curve] supplies finite non-negative start delays in multiples
    /// of duration. Its samples are captured when constructing this tween.
    /// Missing indices ignore writes while playback continues.
    /// [b]Returns:[/b] One registered [SpireCharTween] for the whole range.
    #[func(gd_self)]
    pub fn shake_chars_scale(
        this: Gd<Self>,
        first: i64,
        last: i64,
        duration: f64,
        strength: Vector2,
        delay_curve: EaseKind,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 90.0)] randomness: real,
        #[opt(default = true)] fade_out: bool,
        #[opt(default = ShakeRandomnessMode::Full)] randomness_mode: ShakeRandomnessMode,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireCharTween> {
        gd_from_native_tween(
            this.do_shake_chars_scale(
                first,
                last,
                duration,
                strength,
                vibrato,
                randomness,
                fade_out,
                randomness_mode,
                blendable,
                delay_curve,
            )
            .register(),
        )
    }
}
