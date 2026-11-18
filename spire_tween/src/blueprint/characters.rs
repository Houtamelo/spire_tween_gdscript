use anyhow::{Context, Result, ensure};

use super::*;
use crate::tweens::CompiledTextEffect;

impl TweenBlueprint {
    pub(super) fn compiled_character_effect(&self) -> Result<CompiledTextEffect> {
        let resource = self
            .character_effect
            .as_ref()
            .context("`character_effect` must be a supported text effect resource.")?;
        CompiledTextEffect::compile(resource, "character_effect")
    }

    pub(super) fn resolve_effect_configuration(&self) -> Result<(CompiledTextEffect, Gd<SpireRichTextLabel>)> {
        let effect = self.compiled_character_effect()?;
        let label = self
            .resolve_target()
            .and_then(|node| node.try_cast::<SpireRichTextLabel>().ok())
            .context("`target` must resolve to a SpireRichTextLabel for `kind` Characters.")?;

        ensure!(
            self.character_first >= 0
                && self.character_last >= -1
                && (self.character_last == -1 || self.character_last >= self.character_first),
            "`character_first` must be non-negative; `character_last` must be -1 or at least `character_first`."
        );
        let count = label.get_total_character_count() as i64;
        ensure!(
            self.character_first < count && self.character_last < count,
            "`character_first` and `character_last` must select existing logical character indices; use -1 for \
             `character_last` to follow the text length."
        );

        let last = if self.character_last == -1 { count - 1 } else { self.character_last };
        let delay_field = match self.character_delay_ease_source {
            EaseSource::Curve if self.character_delay_ease_curve.is_some() => "character_delay_ease_curve",
            _ => "character_delay_ease_enum",
        };
        character_start_delays(self.character_first, last, effect.duration(), &self.character_delay_ease())
            .with_context(|| format!("Invalid `{delay_field}`."))?;

        ensure!(self.character_loops >= 1, "`character_loops` must be at least 1.");

        if self.start_enabled
            && let Some(default) = effect.impulse_start_default()
        {
            ensure!(
                tweenable_property_validation::value_matches(&self.start_value, default.get_type()),
                "`start_value` must match `character_effect`: expected {}.",
                default.get_type().godot_type_name()
            );
        }

        Ok((effect, label))
    }

    pub(super) fn build_characters(this: Gd<Self>, sequence: Gd<SpireSequence>) {
        let state = this.bind();
        let label = state
            .resolve_target()
            .and_then(|node| node.try_cast::<SpireRichTextLabel>().ok());
        // Range selection needs the current text; replay preserves unchanged state.
        if let Some(label) = &label {
            SpireRichTextLabel::refresh_characters(label.clone(), false);
        }
        let (effect, label) = match state.resolve_effect_configuration() {
            Ok(configuration) => configuration,
            Err(warning) => {
                godot_warn!("[TweenBlueprint] {warning:#}");
                return;
            }
        };
        let first = state.character_first;
        let last = if state.character_last == -1 {
            SpireRichTextLabel::get_character_count(label.clone()) - 1
        } else {
            state.character_last
        };
        let start = state.start_value.clone();
        let explicit = state.start_enabled;
        let initialize = label.clone();
        // Initialize the whole selection when this step begins, before staggering.
        sequence.bind().to_mut().append_fn(move || {
            if !initialize.is_instance_valid() {
                return;
            }
            for index in first..=last {
                if SpireRichTextLabel::is_valid_char(initialize.clone(), index) {
                    effect.initialize(initialize.clone(), index, explicit, &start);
                }
            }
        });

        let mut tween = effect
            .range_tween(label, first, last, state.character_delay_ease(), state.character_blendable)
            .with_loops(state.character_loops, effect.loop_mode());

        let ease = state.ease_source.resolve(state.ease_enum, state.ease_curve.as_ref());
        if !matches!(ease, EaseKind::Basic(Ease::Default)) {
            tween = tween.with_ease(ease);
        }

        sequence.bind().to_mut().append(tween);
    }
}
