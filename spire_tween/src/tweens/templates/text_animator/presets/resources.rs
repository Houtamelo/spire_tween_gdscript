use godot::classes::IResource;

use super::*;

/// Reusable looping effects applied to text enclosed by [member bbcode].
/// Entries run in array order, followed by code-controlled character state.
#[derive(GodotClass)]
#[class(init, tool, base = Resource)]
pub struct SpireRichTextEffectPreset {
    base: Base<Resource>,
    /// Tag identifier, without brackets. Must not collide with another effect or a built-in tag.
    #[export]
    #[var(set = set_bbcode)]
    pub bbcode: GString,
    /// Ordered effects with independent timing. Empty slots are allowed while authoring.
    #[export]
    #[var(set = set_effects)]
    pub effects: Array<Option<Gd<SpireTextEffect>>>,
}

#[godot_api]
impl SpireRichTextEffectPreset {
    #[func]
    pub fn set_bbcode(&mut self, value: GString) {
        self.bbcode = value;
        self.base_mut().emit_changed();
    }
    #[func]
    pub fn set_effects(&mut self, value: Array<Option<Gd<SpireTextEffect>>>) {
        self.effects = value;
        self.base_mut().emit_changed();
    }
}

/// Shared configuration for looping text presets and finite Blueprint character tweens.
/// [member kind] selects the visible fields. Hidden values remain serialized.
/// Changing kind preserves authored endpoints; incompatible values must be repaired.
#[derive(GodotClass)]
#[class(init, tool, base = Resource)]
pub struct SpireTextEffect {
    base: Base<Resource>,
    #[export]
    #[var(set = set_kind)]
    pub kind: TextEffectKind,
    /// Seconds per animation leg for each character; total range duration includes start delays and loops.
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
    #[export]
    #[var(set = set_character_phase_offset)]
    pub character_phase_offset: f64,
    #[export]
    #[var(set = set_from)]
    #[init(val = Color::WHITE.to_variant())]
    pub from: Variant,
    #[export]
    #[var(set = set_to)]
    #[init(val = Color::WHITE.to_variant())]
    pub to: Variant,
    /// Additive punch strength. Offset and scale effects ignore Z.
    #[export]
    #[var(set = set_punch)]
    #[init(val = Vector3::ZERO)]
    pub punch: Vector3,
    /// Shake strength. Offset and scale effects ignore Z.
    #[export]
    #[var(set = set_strength)]
    #[init(val = Vector3::ONE)]
    pub strength: Vector3,
    #[export]
    #[var(set = set_vibrato)]
    #[init(val = 10)]
    pub vibrato: i32,
    #[export]
    #[var(set = set_elasticity)]
    #[init(val = 1.0)]
    pub elasticity: f64,
    #[export]
    #[var(set = set_randomness)]
    #[init(val = 90.0)]
    pub randomness: f64,
    #[export]
    #[var(set = set_randomness_mode)]
    #[init(val = ShakeRandomnessMode::Full)]
    pub randomness_mode: ShakeRandomnessMode,
    #[export]
    #[var(set = set_fade_out)]
    #[init(val = true)]
    pub fade_out: bool,
    #[export]
    #[var(set = set_pivot)]
    #[init(val = Vector2::ZERO)]
    pub pivot: Vector2,
    #[export]
    #[var(set = set_rotate_mode)]
    pub rotate_mode: RotateMode,
}

impl TextEffectKind {
    pub(crate) fn endpoint_type(self) -> Option<VariantType> {
        Some(match self {
            Self::Color => VariantType::COLOR,
            Self::Fade => VariantType::FLOAT,
            Self::Offset | Self::Scale => VariantType::VECTOR2,
            Self::Rotation => VariantType::VECTOR3,
            _ => return None,
        })
    }

    pub(crate) fn endpoint_default(self, end: bool) -> Option<Variant> {
        Some(match self {
            Self::Color => Color::WHITE.to_variant(),
            Self::Fade => (if end { 1.0_f64 } else { 0.0_f64 }).to_variant(),
            Self::Offset => Vector2::ZERO.to_variant(),
            Self::Scale => Vector2::ONE.to_variant(),
            Self::Rotation => Vector3::ZERO.to_variant(),
            _ => return None,
        })
    }
}

#[godot_api]
impl IResource for SpireTextEffect {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        let name = property.property_name.to_string();
        if name == "kind" {
            property.usage = property.usage.with(PropertyUsageFlags::UPDATE_ALL_IF_MODIFIED);
            return;
        }
        let visible = match name.as_str() {
            "from" | "to" => {
                if let Some(ty) = self.kind.endpoint_type() {
                    let value = if name == "from" { &self.from } else { &self.to };
                    if value.is_nil() || value.get_type() == ty {
                        property.variant_type = ty;
                        property.class_name = StringName::default();
                        property.hint_info = PropertyHintInfo::none();
                        property.usage = property.usage.without(PropertyUsageFlags::NIL_IS_VARIANT);
                    }
                    true
                } else {
                    false
                }
            }
            "punch" | "elasticity" => {
                matches!(
                    self.kind,
                    TextEffectKind::PunchOffset | TextEffectKind::PunchRotation | TextEffectKind::PunchScale
                )
            }
            "strength" | "randomness" | "randomness_mode" | "fade_out" => {
                matches!(
                    self.kind,
                    TextEffectKind::ShakeOffset | TextEffectKind::ShakeRotation | TextEffectKind::ShakeScale
                )
            }
            "vibrato" => matches!(
                self.kind,
                TextEffectKind::PunchOffset
                    | TextEffectKind::PunchRotation
                    | TextEffectKind::PunchScale
                    | TextEffectKind::ShakeOffset
                    | TextEffectKind::ShakeRotation
                    | TextEffectKind::ShakeScale
            ),
            "pivot" => matches!(
                self.kind,
                TextEffectKind::Rotation
                    | TextEffectKind::Scale
                    | TextEffectKind::PunchRotation
                    | TextEffectKind::PunchScale
                    | TextEffectKind::ShakeRotation
                    | TextEffectKind::ShakeScale
            ),
            "rotate_mode" => self.kind == TextEffectKind::Rotation,
            _ => return,
        };
        property.usage = property.usage.with(PropertyUsageFlags::STORAGE);
        if !visible {
            property.usage = property.usage.without(PropertyUsageFlags::EDITOR);
        }
    }

    fn on_property_get_revert(&self, property: StringName) -> Option<Variant> {
        match property.to_string().as_str() {
            "from" => self.kind.endpoint_default(false),
            "to" => self.kind.endpoint_default(true),
            _ => None,
        }
    }
}

#[godot_api]
impl SpireTextEffect {
    #[func]
    pub fn set_kind(&mut self, value: TextEffectKind) {
        self.kind = value;
        self.base_mut().notify_property_list_changed();
        self.base_mut().emit_changed();
    }
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
    #[func]
    pub fn set_from(&mut self, value: Variant) {
        let type_changed = self.from.get_type() != value.get_type();
        self.from = value;
        if type_changed {
            self.base_mut().notify_property_list_changed();
        }
        self.base_mut().emit_changed();
    }
    #[func]
    pub fn set_to(&mut self, value: Variant) {
        let type_changed = self.to.get_type() != value.get_type();
        self.to = value;
        if type_changed {
            self.base_mut().notify_property_list_changed();
        }
        self.base_mut().emit_changed();
    }
    #[func]
    pub fn set_punch(&mut self, value: Vector3) {
        self.punch = value;
        self.base_mut().emit_changed();
    }
    #[func]
    pub fn set_strength(&mut self, value: Vector3) {
        self.strength = value;
        self.base_mut().emit_changed();
    }
    #[func]
    pub fn set_vibrato(&mut self, value: i32) {
        self.vibrato = value;
        self.base_mut().emit_changed();
    }
    #[func]
    pub fn set_elasticity(&mut self, value: f64) {
        self.elasticity = value;
        self.base_mut().emit_changed();
    }
    #[func]
    pub fn set_randomness(&mut self, value: f64) {
        self.randomness = value;
        self.base_mut().emit_changed();
    }
    #[func]
    pub fn set_randomness_mode(&mut self, value: ShakeRandomnessMode) {
        self.randomness_mode = value;
        self.base_mut().emit_changed();
    }
    #[func]
    pub fn set_fade_out(&mut self, value: bool) {
        self.fade_out = value;
        self.base_mut().emit_changed();
    }
    #[func]
    pub fn set_pivot(&mut self, value: Vector2) {
        self.pivot = value;
        self.base_mut().emit_changed();
    }
    #[func]
    pub fn set_rotate_mode(&mut self, value: RotateMode) {
        self.rotate_mode = value;
        self.base_mut().emit_changed();
    }
}
