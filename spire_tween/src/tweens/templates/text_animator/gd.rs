use super::*;

// The renderer/processing node is also available to native consumers. Only the
// registered tween constructors and Godot-facing convenience methods live here.
#[godot_api]
impl SpireTextAnimator {
    /// [b]Behavior:[/b] Attaches a per-character animator to [param label], or
    /// returns its existing animator. The label owns this processing child.
    ///
    /// [b]Note:[/b] Automatically maintains a custom-effect scope without changing
    /// the label's text property or BBCode setting. Supply content through
    /// [member RichTextLabel.text]; manual formatting-stack edits are not tracked.
    ///
    /// [b]Returns:[/b] The animator, or null when [param label] is null.
    #[func(rename = attach)]
    pub fn gd_attach(label: Option<Gd<RichTextLabel>>) -> Option<Gd<Self>> {
        let Some(label) = label else {
            godot_warn!("[spire characters] label is null; cannot attach an animator.");
            return None;
        };
        Some(Self::attach(label))
    }

    /// [b]Behavior:[/b] Rebuilds character data and resets all transformations.
    /// Text/BBCode/translation changes trigger this automatically. Active tweens
    /// retain their clocks and indices. Missing indices cancel on their next step.
    #[func(rename = refresh)]
    pub fn gd_refresh(&mut self) { self.animator.refresh(); }

    /// [b]Behavior:[/b] Resets all deformations without rebuilding text, cancelling
    /// tweens or changing their playback positions.
    #[func(rename = reset)]
    pub fn gd_reset(&mut self) { self.animator.reset(); }

    /// [b]Returns:[/b] The logical character count, including spaces and newlines,
    /// excluding BBCode syntax. Indices follow parsed text order, not visual order.
    #[func]
    pub fn get_character_count(&self) -> i64 { self.animator.character_count() }

    /// [b]Returns:[/b] Whether [param index] exists in the current parsed text.
    /// Visibility, clipping and visible_characters do not remove indices.
    #[func]
    pub fn is_valid_char(&self, index: i64) -> bool { self.animator.is_valid_char(index) }

    /// [b]Returns:[/b] The character's additional offset in pixels.
    #[func]
    pub fn get_char_offset(&self, index: i64) -> Vector2 { self.animator.get_char_offset(index) }

    /// [b]Behavior:[/b] Sets the character's additional offset in pixels.
    /// Invalid indices warn and do nothing. Nonfinite values warn and normalize.
    #[func]
    pub fn set_char_offset(&mut self, index: i64, value: Vector2) { self.animator.set_char_offset(index, value); }

    /// [b]Returns:[/b] The character's YXZ Euler rotation in radians, projected orthographically.
    #[func]
    pub fn get_char_rotation(&self, index: i64) -> Vector3 { self.animator.get_char_rotation(index) }

    /// [b]Behavior:[/b] Sets the character's YXZ Euler rotation in radians, projected orthographically.
    /// Invalid indices warn and do nothing. Nonfinite values warn and normalize.
    #[func]
    pub fn set_char_rotation(&mut self, index: i64, value: Vector3) { self.animator.set_char_rotation(index, value); }

    /// [b]Returns:[/b] The character's per-axis scale multiplier.
    #[func]
    pub fn get_char_scale(&self, index: i64) -> Vector2 { self.animator.get_char_scale(index) }

    /// [b]Behavior:[/b] Sets the character's per-axis scale multiplier.
    /// Invalid indices warn and do nothing. Nonfinite values warn and normalize.
    #[func]
    pub fn set_char_scale(&mut self, index: i64, value: Vector2) { self.animator.set_char_scale(index, value); }

    /// [b]Returns:[/b] The character's fill color and alpha.
    /// Before the first draw the theme default is used; drawing captures BBCode
    /// and preceding-effect colors.
    #[func]
    pub fn get_char_color(&self, index: i64) -> Color { self.animator.get_char_color(index) }

    /// [b]Behavior:[/b] Sets the character's fill color and alpha.
    /// Invalid indices warn and do nothing. Nonfinite values warn and normalize.
    #[func]
    pub fn set_char_color(&mut self, index: i64, value: Color) { self.animator.set_char_color(index, value); }

    /// [b]Returns:[/b] The character's alpha. Easing/Incremental playback may overshoot.
    /// Before the first draw this reads the theme default's alpha.
    #[func]
    pub fn get_char_alpha(&self, index: i64) -> f64 { self.animator.get_char_alpha(index) }

    /// [b]Behavior:[/b] Sets the character's alpha in [0,1], preserving RGB.
    /// Invalid indices warn and do nothing. Nonfinite values warn and normalize.
    #[func]
    pub fn set_char_alpha(&mut self, index: i64, value: f64) { self.animator.set_char_alpha(index, value); }

    /// [b]Returns:[/b] The character's pivot offset in pixels from the glyph pen origin.
    #[func]
    pub fn get_char_pivot(&self, index: i64) -> Vector2 { self.animator.get_char_pivot(index) }

    /// [b]Behavior:[/b] Sets the character's pivot offset in pixels from the glyph pen origin.
    /// Invalid indices warn and do nothing. Nonfinite values warn and normalize.
    #[func]
    pub fn set_char_pivot(&mut self, index: i64, value: Vector2) { self.animator.set_char_pivot(index, value); }

    /// [b]Behavior:[/b] Tweens the additional character offset in pixels. Captures the start on first processing.
    /// Invalid index warns and returns a stopped tween. Invalid duration becomes
    /// one second. Text edits reset deformations; surviving indices continue.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func]
    pub fn offset_char(&self, index: i64, to: Vector2, duration: f64) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(self.animator.do_offset_char(index, to, duration).register())
    }

    /// [b]Behavior:[/b] Tweens per-axis character scale multipliers. Negative values mirror. Captures the start on first processing.
    /// Invalid index warns and returns a stopped tween. Invalid duration becomes
    /// one second. Text edits reset deformations; surviving indices continue.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func]
    pub fn scale_char(&self, index: i64, to: Vector2, duration: f64) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(self.animator.do_scale_char(index, to, duration).register())
    }

    /// [b]Behavior:[/b] Tweens character fill RGBA. Target alpha clamps to [0,1]; RGB allows HDR. Captures the start on first processing.
    /// Wait for the label's first draw to capture its effective BBCode color.
    /// Invalid index warns and returns a stopped tween. Invalid duration becomes
    /// one second. Text edits reset deformations; surviving indices continue.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func]
    pub fn color_char(&self, index: i64, to: Color, duration: f64) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(self.animator.do_color_char(index, to, duration).register())
    }

    /// [b]Behavior:[/b] Tweens toward a character alpha target in [0,1], preserving RGB. Captures the start on first processing.
    /// Interpolated values may overshoot with easing or Incremental loops.
    /// Wait for the label's first draw to capture its effective BBCode alpha.
    /// Invalid index warns and returns a stopped tween. Invalid duration becomes
    /// one second. Text edits reset deformations; surviving indices continue.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func]
    pub fn fade_char(&self, index: i64, to: f64, duration: f64) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(self.animator.do_fade_char(index, to, duration).register())
    }

    /// [b]Behavior:[/b] Tweens three-axis YXZ Euler rotation in radians, projected
    /// orthographically. Fast uses shortest per-axis deltas; FastBeyond360 retains
    /// turns. WorldAxisAdd/LocalAxisAdd compose before/after the initial rotation.
    /// Invalid indices return stopped tweens; invalid duration becomes one second.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func]
    pub fn rotate_char(
        &self,
        index: i64,
        to: Vector3,
        duration: f64,
        #[opt(default = RotateMode::Fast)] mode: RotateMode,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(self.animator.do_rotate_char(index, to, duration, mode).register())
    }

    /// [b]Behavior:[/b] Punches the character's additional pixel offset.
    /// Uses Spire's existing punch math. Stop leaves the last deformation;
    /// successful completion removes this effect's contribution.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func]
    pub fn punch_char_offset(
        &self,
        index: i64,
        punch: Vector2,
        duration: f64,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 1.0)] elasticity: f64,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            self.animator
                .do_punch_char_offset(index, punch, duration, vibrato, elasticity, blendable)
                .register(),
        )
    }

    /// [b]Behavior:[/b] Shakes the character's additional pixel offset.
    /// Uses Spire's interpolated waypoint sampler. Stop leaves the last
    /// deformation; successful completion removes this effect's contribution.
    /// Invalid inputs warn with rejected values and normalize.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func]
    pub fn shake_char_offset(
        &self,
        index: i64,
        duration: f64,
        strength: Vector2,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 90.0)] randomness: real,
        #[opt(default = true)] fade_out: bool,
        #[opt(default = ShakeRandomnessMode::Full)] randomness_mode: ShakeRandomnessMode,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            self.animator
                .do_shake_char_offset(
                    index,
                    duration,
                    strength,
                    vibrato,
                    randomness,
                    fade_out,
                    randomness_mode,
                    blendable,
                )
                .register(),
        )
    }

    /// [b]Behavior:[/b] Punches the character's Euler rotation in radians.
    /// Uses Spire's existing punch math. Stop leaves the last deformation;
    /// successful completion removes this effect's contribution.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func]
    pub fn punch_char_rotation(
        &self,
        index: i64,
        punch: Vector3,
        duration: f64,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 1.0)] elasticity: f64,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            self.animator
                .do_punch_char_rotation(index, punch, duration, vibrato, elasticity, blendable)
                .register(),
        )
    }

    /// [b]Behavior:[/b] Shakes the character's Euler rotation in radians.
    /// Uses Spire's interpolated waypoint sampler. Stop leaves the last
    /// deformation; successful completion removes this effect's contribution.
    /// Invalid inputs warn with rejected values and normalize.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func]
    pub fn shake_char_rotation(
        &self,
        index: i64,
        duration: f64,
        strength: Vector3,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 90.0)] randomness: real,
        #[opt(default = true)] fade_out: bool,
        #[opt(default = ShakeRandomnessMode::Full)] randomness_mode: ShakeRandomnessMode,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            self.animator
                .do_shake_char_rotation(
                    index,
                    duration,
                    strength,
                    vibrato,
                    randomness,
                    fade_out,
                    randomness_mode,
                    blendable,
                )
                .register(),
        )
    }

    /// [b]Behavior:[/b] Punches the character's scale using additive per-axis deltas.
    /// Uses Spire's existing punch math. Stop leaves the last deformation;
    /// successful completion removes this effect's contribution.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func]
    pub fn punch_char_scale(
        &self,
        index: i64,
        punch: Vector2,
        duration: f64,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 1.0)] elasticity: f64,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            self.animator
                .do_punch_char_scale(index, punch, duration, vibrato, elasticity, blendable)
                .register(),
        )
    }

    /// [b]Behavior:[/b] Shakes the character's scale using additive per-axis deltas.
    /// Uses Spire's interpolated waypoint sampler. Stop leaves the last
    /// deformation; successful completion removes this effect's contribution.
    /// Invalid inputs warn with rejected values and normalize.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func]
    pub fn shake_char_scale(
        &self,
        index: i64,
        duration: f64,
        strength: Vector2,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 90.0)] randomness: real,
        #[opt(default = true)] fade_out: bool,
        #[opt(default = ShakeRandomnessMode::Full)] randomness_mode: ShakeRandomnessMode,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            self.animator
                .do_shake_char_scale(
                    index,
                    duration,
                    strength,
                    vibrato,
                    randomness,
                    fade_out,
                    randomness_mode,
                    blendable,
                )
                .register(),
        )
    }
}
