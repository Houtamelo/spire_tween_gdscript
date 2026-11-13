use std::cell::LazyCell;

use super::*;

/// A RichTextLabel with native per-character animation methods and state.
/// Create this node directly or use the editor's Change Type action on an
/// existing RichTextLabel. Rust callers use the SpireDoRichTextCharacters trait for builders.
/// Supply content through [member RichTextLabel.text]. Processing respects
/// [member Node.process_mode]; the addon never overrides it.
///
/// [b]Initialization:[/b] Wait for the label's first [signal CanvasItem.draw]
/// before starting color/fade tweens which must capture BBCode or other effect
/// colors. Before drawing, the theme default is used. Shaping clusters are also
/// learned during drawing.
///
/// [b]Transforms:[/b] Rotation is a three-axis YXZ Euler vector in radians with
/// orthographic projection. Rotation/scale default to the glyph pen origin;
/// use [method set_char_pivot] for an explicit pixel offset. Exact automatic
/// center pivots and perspective are not provided by this renderer.
#[derive(GodotClass)]
#[class(init, tool, base = RichTextLabel)]
pub struct SpireRichTextLabel {
    base: Base<RichTextLabel>,
    #[init(val = LazyCell::new(SpireCharRenderer::new_gd))]
    pub char_renderer: LazyCell<Gd<SpireCharRenderer>>,
    source: Option<GString>,
    bbcode: bool,
    presets: presets::Registry,
    /// Reusable looping effects selected by BBCode tags. Presets render before code-controlled state.
    #[export]
    pub effect_presets: Array<Option<Gd<SpireRichTextEffectPreset>>>,
}

impl SpireRichTextLabel {
    pub fn refresh_characters(mut this: Gd<Self>, force: bool) -> bool {
        if !this.is_instance_valid() {
            return false;
        }

        let mut label = this.clone().upcast::<RichTextLabel>();
        let source = label.get_text();
        let bbcode = label.is_using_bbcode();
        let installed_effects = label.get_effects();
        let (source_changed, presets_changed, mut effect) = {
            let state = &mut *this.bind_mut();
            let source_changed = state.source.as_ref() != Some(&source) || state.bbcode != bbcode;
            let presets_changed = state.presets.sync(&installed_effects, &state.effect_presets);
            (source_changed, presets_changed, state.char_renderer.clone())
        };

        if presets_changed && Engine::singleton().is_editor_hint() {
            label.call_deferred("update_configuration_warnings", &[]);
        }

        if !force && !source_changed && !presets_changed {
            return true;
        }

        // Release label/effect borrows before native calls, which may notify the label.
        // clear/add/append edit the rendering stack, not the source text property.
        let translated = label.atr(&source);
        let (authored_effects, registered) = this.bind_mut().presets.begin_parse();
        label.set_effects(&registered);
        // Install before rebuilding so correctness does not depend on whether
        // set_effects reparses the existing text.
        label.clear();
        // The outer scope makes the code layer run after nested preset effects.
        label.push_customfx(&effect, &VarDictionary::new());
        if bbcode {
            label.append_text(&translated);
        } else {
            label.add_text(&translated);
        }
        label.pop();
        if Engine::singleton().is_editor_hint() {
            // Keep generated adapters out of saved editor data. Runtime labels
            // retain their adapters so subsequent BBCode parses can find them.
            label.set_effects(&authored_effects);
        }
        let count = label.get_total_character_count().max(0) as usize;
        let color = label.get_theme_color("default_color");
        {
            let mut state = effect.bind_mut();
            if force || source_changed || state.chars.len() != count {
                state.chars.clear();
                state.epoch = state.epoch.wrapping_add(1);
                state.chars.extend((0..count).map(|i| CharState::new(i, color)));
            }
        }
        #[cfg(feature = "integration-tests")]
        testing::clear(effect.instance_id());
        {
            let mut state = this.bind_mut();
            state.source = Some(source);
            state.bbcode = bbcode;
        }
        label.queue_redraw();
        true
    }
}

#[godot_api]
impl IRichTextLabel for SpireRichTextLabel {
    #[func(gd_self)]
    fn ready(this: Gd<Self>) {
        if !Engine::singleton().is_editor_hint() || !this.bind().effect_presets.is_empty() {
            Self::refresh_characters(this, false);
        }
    }

    #[func(gd_self)]
    fn process(this: Gd<Self>, _delta: f64) {
        if !Engine::singleton().is_editor_hint() || {
            let bind = this.bind();
            !bind.effect_presets.is_empty() || LazyCell::get(&bind.char_renderer).is_some()
        } {
            Self::refresh_characters(this, false);
        }
    }

    fn on_notification(&mut self, what: notify::ControlNotification) {
        if what == notify::ControlNotification::TRANSLATION_CHANGED {
            // Defer rebuilding until processing or an explicit refresh.
            // The notification may arrive during a native RichTextLabel call.
            self.source = None;
        } else if what == notify::ControlNotification::READY && Engine::singleton().is_editor_hint() {
            let callback = self.base().callable("update_configuration_warnings");
            if !self.base().is_connected("script_changed", &callback) {
                self.base_mut().connect("script_changed", &callback);
            }
            self.base_mut().update_configuration_warnings();
        }
    }

    fn get_configuration_warnings(&self) -> PackedStringArray {
        let mut warnings = PackedStringArray::new();
        if !Engine::singleton().is_editor_hint() {
            return warnings;
        }
        for warning in &self.presets.warnings {
            warnings.push(warning.as_str());
        }
        let mut script = self.base().get_script();
        while let Some(current) = script {
            if current
                .get_script_method_list()
                .iter_shared()
                .any(|method| method.get("name").is_some_and(|name| name.to_string() == "_process"))
            {
                warnings.push(
                    "The attached script overrides `_process()`, disabling SpireRichTextLabel's automatic character \
                     refresh. `super._process()` cannot invoke the Rust implementation. Use another node for custom \
                     per-frame logic.",
                );
                break;
            }
            script = current.get_base_script();
        }
        warnings
    }
}

// Character accessors and registered constructors exposed to Godot.
#[godot_api]
impl SpireRichTextLabel {
    #[cfg(feature = "integration-tests")]
    #[func(gd_self)]
    fn preset_adapter_for_test(this: Gd<Self>, index: i64) -> Option<Gd<SpireRichTextEffect>> {
        this.bind().presets.adapter_for_test(usize::try_from(index).ok()?)
    }

    #[cfg(feature = "integration-tests")]
    #[func(gd_self)]
    pub fn render_sample_for_test(this: Gd<Self>, index: i64) -> VarDictionary {
        let mut result = VarDictionary::new();
        if let Some((transform, color)) = Self::sample_for_test(this, index) {
            result.set("transform", transform);
            result.set("color", color);
        }
        result
    }

    /// [b]Behavior:[/b] Rebuilds character data and resets all transformations.
    /// Text/BBCode/translation changes trigger this automatically. Active tweens
    /// retain their clocks and indices. Writes to missing indices are ignored while playback continues.
    #[func(gd_self)]
    pub fn refresh(this: Gd<Self>) { Self::refresh_characters(this, true); }

    /// [b]Behavior:[/b] Resets all deformations without rebuilding text, cancelling
    /// tweens or changing their playback positions.
    #[func(gd_self)]
    pub fn reset(mut this: Gd<Self>) {
        let mut effect = this.bind().char_renderer.clone();
        effect.bind_mut().reset();
        this.queue_redraw();
    }

    /// [b]Returns:[/b] The logical character count, including spaces and newlines,
    /// excluding BBCode syntax, from the last refresh. Call [method refresh] for current text synchronously.
    /// Indices follow parsed text order, not visual order.
    #[func(gd_self)]
    pub fn get_character_count(this: Gd<Self>) -> i64 { this.bind().char_renderer.bind().character_count() }

    /// [b]Returns:[/b] Whether [param index] exists in the current parsed text.
    /// Uses the last refreshed state. Visibility, clipping and visible_characters do not remove indices.
    #[func(gd_self)]
    pub fn is_valid_char(this: Gd<Self>, index: i64) -> bool { this.bind().char_renderer.bind().is_valid_char(index) }

    /// [b]Returns:[/b] The character's additional offset in pixels.
    #[func(gd_self)]
    pub fn get_char_offset(this: Gd<Self>, index: i64) -> Vector2 {
        this.bind().char_renderer.bind().get_char_offset(index)
    }

    /// [b]Behavior:[/b] Sets the character's additional offset in pixels.
    /// Invalid indices warn and do nothing. Nonfinite values warn and normalize.
    #[func(gd_self)]
    pub fn set_char_offset(mut this: Gd<Self>, index: i64, value: Vector2) {
        let mut effect = this.bind().char_renderer.clone();
        effect.bind_mut().set_char_offset(index, value);
        this.queue_redraw();
    }

    /// [b]Returns:[/b] The character's YXZ Euler rotation in radians, projected orthographically.
    #[func(gd_self)]
    pub fn get_char_rotation(this: Gd<Self>, index: i64) -> Vector3 {
        this.bind().char_renderer.bind().get_char_rotation(index)
    }

    /// [b]Behavior:[/b] Sets the character's YXZ Euler rotation in radians, projected orthographically.
    /// Invalid indices warn and do nothing. Nonfinite values warn and normalize.
    #[func(gd_self)]
    pub fn set_char_rotation(mut this: Gd<Self>, index: i64, value: Vector3) {
        let mut effect = this.bind().char_renderer.clone();
        effect.bind_mut().set_char_rotation(index, value);
        this.queue_redraw();
    }

    /// [b]Returns:[/b] The character's per-axis scale multiplier.
    #[func(gd_self)]
    pub fn get_char_scale(this: Gd<Self>, index: i64) -> Vector2 {
        this.bind().char_renderer.bind().get_char_scale(index)
    }

    /// [b]Behavior:[/b] Sets the character's per-axis scale multiplier.
    /// Invalid indices warn and do nothing. Nonfinite values warn and normalize.
    #[func(gd_self)]
    pub fn set_char_scale(mut this: Gd<Self>, index: i64, value: Vector2) {
        let mut effect = this.bind().char_renderer.clone();
        effect.bind_mut().set_char_scale(index, value);
        this.queue_redraw();
    }

    /// [b]Returns:[/b] The character's fill color and alpha.
    /// Before the first draw the theme default is used; drawing captures BBCode
    /// and preceding-effect colors.
    #[func(gd_self)]
    pub fn get_char_color(this: Gd<Self>, index: i64) -> Color {
        this.bind().char_renderer.bind().get_char_color(index)
    }

    /// [b]Behavior:[/b] Sets the character's fill color and alpha.
    /// Invalid indices warn and do nothing. Nonfinite values warn and normalize.
    #[func(gd_self)]
    pub fn set_char_color(mut this: Gd<Self>, index: i64, value: Color) {
        let mut effect = this.bind().char_renderer.clone();
        effect.bind_mut().set_char_color(index, value);
        this.queue_redraw();
    }

    /// [b]Returns:[/b] The character's alpha. Easing/Incremental playback may overshoot.
    /// Before the first draw this reads the theme default's alpha.
    #[func(gd_self)]
    pub fn get_char_alpha(this: Gd<Self>, index: i64) -> f64 { this.bind().char_renderer.bind().get_char_alpha(index) }

    /// [b]Behavior:[/b] Sets the character's alpha in [0,1], preserving RGB.
    /// Invalid indices warn and do nothing. Nonfinite values warn and normalize.
    #[func(gd_self)]
    pub fn set_char_alpha(mut this: Gd<Self>, index: i64, value: f64) {
        let mut effect = this.bind().char_renderer.clone();
        effect.bind_mut().set_char_alpha(index, value);
        this.queue_redraw();
    }

    /// [b]Returns:[/b] The character's pivot offset in pixels from the glyph pen origin.
    #[func(gd_self)]
    pub fn get_char_pivot(this: Gd<Self>, index: i64) -> Vector2 {
        this.bind().char_renderer.bind().get_char_pivot(index)
    }

    /// [b]Behavior:[/b] Sets the character's pivot offset in pixels from the glyph pen origin.
    /// Invalid indices warn and do nothing. Nonfinite values warn and normalize.
    #[func(gd_self)]
    pub fn set_char_pivot(mut this: Gd<Self>, index: i64, value: Vector2) {
        let mut effect = this.bind().char_renderer.clone();
        effect.bind_mut().set_char_pivot(index, value);
        this.queue_redraw();
    }

    /// [b]Behavior:[/b] Tweens the additional character offset in pixels. Captures the start on first processing.
    /// Missing character indices are ignored without stopping playback. Invalid duration becomes
    /// one second. Text edits reset deformations; surviving indices continue.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func(gd_self)]
    pub fn offset_char(this: Gd<Self>, index: i64, to: Vector2, duration: f64) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(this.do_offset_char(index, to, duration).register())
    }

    /// [b]Behavior:[/b] Tweens per-axis character scale multipliers. Negative values mirror. Captures the start on first processing.
    /// Missing character indices are ignored without stopping playback. Invalid duration becomes
    /// one second. Text edits reset deformations; surviving indices continue.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func(gd_self)]
    pub fn scale_char(this: Gd<Self>, index: i64, to: Vector2, duration: f64) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(this.do_scale_char(index, to, duration).register())
    }

    /// [b]Behavior:[/b] Tweens character fill RGBA. Target alpha clamps to [0,1]; RGB allows HDR. Captures the start on first processing.
    /// Wait for the label's first draw to capture its effective BBCode color.
    /// Missing character indices are ignored without stopping playback. Invalid duration becomes
    /// one second. Text edits reset deformations; surviving indices continue.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func(gd_self)]
    pub fn color_char(this: Gd<Self>, index: i64, to: Color, duration: f64) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(this.do_color_char(index, to, duration).register())
    }

    /// [b]Behavior:[/b] Tweens toward a character alpha target in [0,1], preserving RGB. Captures the start on first processing.
    /// Interpolated values may overshoot with easing or Incremental loops.
    /// Wait for the label's first draw to capture its effective BBCode alpha.
    /// Missing character indices are ignored without stopping playback. Invalid duration becomes
    /// one second. Text edits reset deformations; surviving indices continue.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func(gd_self)]
    pub fn fade_char(this: Gd<Self>, index: i64, to: f64, duration: f64) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(this.do_fade_char(index, to, duration).register())
    }

    /// [b]Behavior:[/b] Tweens three-axis YXZ Euler rotation in radians, projected
    /// orthographically. Fast uses shortest per-axis deltas; FastBeyond360 retains
    /// turns. WorldAxisAdd/LocalAxisAdd compose before/after the initial rotation.
    /// Missing character indices are ignored; invalid duration becomes one second.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func(gd_self)]
    pub fn rotate_char(
        this: Gd<Self>,
        index: i64,
        to: Vector3,
        duration: f64,
        #[opt(default = RotateMode::Fast)] mode: RotateMode,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(this.do_rotate_char(index, to, duration, mode).register())
    }

    /// [b]Behavior:[/b] Punches the character's additional pixel offset.
    /// Uses Spire's existing punch math. Stop leaves the last deformation;
    /// successful completion removes this effect's contribution.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func(gd_self)]
    pub fn punch_char_offset(
        this: Gd<Self>,
        index: i64,
        punch: Vector2,
        duration: f64,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 1.0)] elasticity: f64,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            this.do_punch_char_offset(index, punch, duration, vibrato, elasticity, blendable)
                .register(),
        )
    }

    /// [b]Behavior:[/b] Shakes the character's additional pixel offset.
    /// Uses Spire's interpolated waypoint sampler. Stop leaves the last
    /// deformation; successful completion removes this effect's contribution.
    /// Invalid inputs warn with rejected values and normalize.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func(gd_self)]
    pub fn shake_char_offset(
        this: Gd<Self>,
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
            this.do_shake_char_offset(
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
    #[func(gd_self)]
    pub fn punch_char_rotation(
        this: Gd<Self>,
        index: i64,
        punch: Vector3,
        duration: f64,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 1.0)] elasticity: f64,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            this.do_punch_char_rotation(index, punch, duration, vibrato, elasticity, blendable)
                .register(),
        )
    }

    /// [b]Behavior:[/b] Shakes the character's Euler rotation in radians.
    /// Uses Spire's interpolated waypoint sampler. Stop leaves the last
    /// deformation; successful completion removes this effect's contribution.
    /// Invalid inputs warn with rejected values and normalize.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func(gd_self)]
    pub fn shake_char_rotation(
        this: Gd<Self>,
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
            this.do_shake_char_rotation(
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
    #[func(gd_self)]
    pub fn punch_char_scale(
        this: Gd<Self>,
        index: i64,
        punch: Vector2,
        duration: f64,
        #[opt(default = 10)] vibrato: i32,
        #[opt(default = 1.0)] elasticity: f64,
        #[opt(default = true)] blendable: bool,
    ) -> Gd<SpireMethodFloat> {
        gd_from_native_tween(
            this.do_punch_char_scale(index, punch, duration, vibrato, elasticity, blendable)
                .register(),
        )
    }

    /// [b]Behavior:[/b] Shakes the character's scale using additive per-axis deltas.
    /// Uses Spire's interpolated waypoint sampler. Stop leaves the last
    /// deformation; successful completion removes this effect's contribution.
    /// Invalid inputs warn with rejected values and normalize.
    ///
    /// [b]Returns:[/b] A registered [SpireMethodFloat] progress tween.
    #[func(gd_self)]
    pub fn shake_char_scale(
        this: Gd<Self>,
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
            this.do_shake_char_scale(
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

#[cfg(feature = "integration-tests")]
impl Drop for SpireRichTextLabel {
    fn drop(&mut self) {
        if let Some(effect) = LazyCell::get(&self.char_renderer) {
            testing::clear(effect.instance_id());
        }
    }
}
