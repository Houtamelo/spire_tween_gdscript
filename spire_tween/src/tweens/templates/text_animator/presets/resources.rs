use super::*;

/// Reusable looping effects applied to text enclosed by [member bbcode].
/// Assign this resource to [member SpireRichTextLabel.effect_presets].
/// Entries run in array order, followed by code-controlled character state.
#[derive(GodotClass)]
#[class(init, tool, base = Resource)]
pub struct SpireRichTextEffectPreset {
    base: Base<Resource>,
    /// Tag identifier, without brackets. Must not collide with another effect or a built-in tag.
    #[export]
    #[var(set = set_bbcode)]
    pub bbcode: GString,
    /// Ordered effect resources. Each entry has independent timing.
    #[var(set = set_effects, hint = TYPE_STRING,
        hint_string = "24/17:SpireTextColorEffect,SpireTextFadeEffect,SpireTextOffsetEffect,SpireTextRotationEffect,SpireTextScaleEffect,SpireTextPunchOffsetEffect,SpireTextPunchRotationEffect,SpireTextPunchScaleEffect,SpireTextShakeOffsetEffect,SpireTextShakeRotationEffect,SpireTextShakeScaleEffect",
        usage_flags = [DEFAULT])]
    pub effects: VarArray,
}

#[godot_api]
impl SpireRichTextEffectPreset {
    #[func]
    pub fn set_bbcode(&mut self, value: GString) {
        self.bbcode = value;
        self.base_mut().emit_changed();
    }
    #[func]
    pub fn set_effects(&mut self, value: VarArray) {
        self.effects = value;
        self.base_mut().emit_changed();
    }
}

// godot-rust does not support Rust extension classes deriving from one another.
// Generate concrete Resource classes with the same timing fields instead.
macro_rules! effect_resource {
    ($name:ident, $description:literal, {$($field:ident : $ty:ty = $default:expr => $setter:ident),* $(,)?}) => {
        #[doc = $description]
        #[derive(GodotClass)]
        #[class(init, tool, base = Resource)]
        pub struct $name {
            base: Base<Resource>,
            /// Seconds per animation leg for each character. Must be finite and
            /// positive. A Char tween's total duration includes its latest
            /// character start delay and every requested leg.
            #[export(range = (0.001, 10.0, or_greater))]
            #[var(set = set_duration)]
            #[init(val = 1.0)]
            pub duration: f64,
            #[export]
            #[var(set = set_ease)]
            #[init(val = Ease::Linear)]
            pub ease: Ease,
            /// Restart repeats; Yoyo reverses each leg; Incremental advances basic from/to effects.
            #[export]
            #[var(set = set_loop_mode)]
            pub loop_mode: LoopMode,
            /// Seconds of phase advancement per logical character within a preset tag.
            /// Blueprint character steps configure separate character start-delay easing.
            #[export]
            #[var(set = set_character_phase_offset)]
            pub character_phase_offset: f64,
            $(#[export]
            #[var(set = $setter)]
            #[init(val = $default)]
            pub $field: $ty,)*
        }
        #[godot_api]
        impl $name {
            #[func]
            pub fn set_duration(&mut self, value: f64) {
                self.duration = value;
                self.base_mut().emit_changed();
            }
            #[func]
            pub fn set_ease(&mut self, value: Ease) {
                self.ease = value;
                self.base_mut().emit_changed();
            }
            #[func]
            pub fn set_loop_mode(&mut self, value: LoopMode) {
                self.loop_mode = value;
                self.base_mut().emit_changed();
            }
            #[func]
            pub fn set_character_phase_offset(&mut self, value: f64) {
                self.character_phase_offset = value;
                self.base_mut().emit_changed();
            }
            $(#[func]
            pub fn $setter(&mut self, value: $ty) {
                self.$field = value;
                self.base_mut().emit_changed();
            })*
        }
    };
}

effect_resource!(SpireTextColorEffect, "Interpolates between two fill colors. Alpha affects fill and outline.", {
    from: Color = Color::WHITE => set_from,
    to: Color = Color::WHITE => set_to,
});
effect_resource!(SpireTextFadeEffect, "Interpolates between two alpha values, preserving incoming RGB.", {
    from: f64 = 0.0 => set_from,
    to: f64 = 1.0 => set_to,
});
effect_resource!(SpireTextOffsetEffect, "Interpolates between additional glyph offsets in local pixels.", {
    from: Vector2 = Vector2::ZERO => set_from,
    to: Vector2 = Vector2::ZERO => set_to,
});
effect_resource!(SpireTextRotationEffect, "Interpolates between YXZ Euler rotations in radians with orthographic projection.", {
    from: Vector3 = Vector3::ZERO => set_from,
    to: Vector3 = Vector3::ZERO => set_to,
    rotate_mode: RotateMode = RotateMode::Fast => set_rotate_mode,
    pivot: Vector2 = Vector2::ZERO => set_pivot,
});
effect_resource!(SpireTextScaleEffect, "Interpolates between per-axis glyph scale multipliers.", {
    from: Vector2 = Vector2::ONE => set_from,
    to: Vector2 = Vector2::ONE => set_to,
    pivot: Vector2 = Vector2::ZERO => set_pivot,
});
effect_resource!(SpireTextPunchOffsetEffect, "Punches glyph offset using Spire's punch sampler.", {
    punch: Vector2 = Vector2::ZERO => set_punch,
    vibrato: i32 = 10 => set_vibrato,
    elasticity: f64 = 1.0 => set_elasticity,
});
effect_resource!(SpireTextPunchRotationEffect, "Punches glyph rotation, in radians.", {
    punch: Vector3 = Vector3::ZERO => set_punch,
    vibrato: i32 = 10 => set_vibrato,
    elasticity: f64 = 1.0 => set_elasticity,
    pivot: Vector2 = Vector2::ZERO => set_pivot,
});
effect_resource!(SpireTextPunchScaleEffect, "Punches additive glyph scale deltas.", {
    punch: Vector2 = Vector2::ZERO => set_punch,
    vibrato: i32 = 10 => set_vibrato,
    elasticity: f64 = 1.0 => set_elasticity,
    pivot: Vector2 = Vector2::ZERO => set_pivot,
});
effect_resource!(SpireTextShakeOffsetEffect, "Shakes glyph offsets along a random path.", {
    strength: Vector2 = Vector2::ONE => set_strength,
    vibrato: i32 = 10 => set_vibrato,
    randomness: f64 = 90.0 => set_randomness,
    randomness_mode: ShakeRandomnessMode = ShakeRandomnessMode::Full => set_randomness_mode,
    fade_out: bool = true => set_fade_out,
});
effect_resource!(SpireTextShakeRotationEffect, "Shakes glyph rotation along a random path, in radians.", {
    strength: Vector3 = Vector3::ONE => set_strength,
    vibrato: i32 = 10 => set_vibrato,
    randomness: f64 = 90.0 => set_randomness,
    randomness_mode: ShakeRandomnessMode = ShakeRandomnessMode::Full => set_randomness_mode,
    fade_out: bool = true => set_fade_out,
    pivot: Vector2 = Vector2::ZERO => set_pivot,
});
effect_resource!(SpireTextShakeScaleEffect, "Shakes additive glyph scale deltas along a random path.", {
    strength: Vector2 = Vector2::ONE => set_strength,
    vibrato: i32 = 10 => set_vibrato,
    randomness: f64 = 90.0 => set_randomness,
    randomness_mode: ShakeRandomnessMode = ShakeRandomnessMode::Full => set_randomness_mode,
    fade_out: bool = true => set_fade_out,
    pivot: Vector2 = Vector2::ZERO => set_pivot,
});
