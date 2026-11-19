use anyhow::{Context, Result, ensure};

use super::*;
use crate::tweens::CompiledTextEffect;

impl TweenBlueprint {
    pub(super) fn get_compiled_text_effect(&self) -> Result<CompiledTextEffect> {
        let resource = self
            .text_effect
            .as_ref()
            .context("`text_effect` must be a supported text effect resource.")?;
        CompiledTextEffect::compile(resource, "text_effect")
    }

    pub(super) fn resolve_effect_configuration(&self) -> Result<(CompiledTextEffect, Gd<SpireRichTextLabel>)> {
        let effect = self.get_compiled_text_effect()?;
        let label = self
            .resolve_target()
            .and_then(|node| node.try_cast::<SpireRichTextLabel>().ok())
            .context("`target` must resolve to a SpireRichTextLabel for `kind` Text.")?;

        ensure!(
            self.text_start_index >= 0
                && self.text_end_index >= -1
                && (self.text_end_index == -1 || self.text_end_index >= self.text_start_index),
            "`text_start_index` must be non-negative; `text_end_index` must be -1 or at least `text_start_index`."
        );
        let count = label.get_total_character_count() as i64;
        ensure!(
            self.text_start_index < count && self.text_end_index < count,
            "`text_start_index` and `text_end_index` must select existing logical char indices; use -1 for \
             `text_end_index` to follow the text length."
        );

        let last = if self.text_end_index == -1 { count - 1 } else { self.text_end_index };
        let delay_field = match self.text_delay_ease_source {
            EaseSource::Curve if self.text_delay_ease_curve.is_some() => "text_delay_ease_curve",
            _ => "text_delay_ease_enum",
        };
        resolve_char_start_delays(self.text_start_index, last, effect.duration(), &self.get_text_delay_ease())
            .with_context(|| format!("Invalid `{delay_field}`."))?;

        ensure!(self.text_loops >= 1, "`text_loops` must be at least 1.");

        if self.property_start_enabled
            && let Some(default) = effect.impulse_start_default()
        {
            ensure!(
                tweenable_property_validation::value_matches(&self.property_start_value, default.get_type()),
                "`property_start_value` must match `text_effect`: expected {}.",
                default.get_type().godot_type_name()
            );
        }

        Ok((effect, label))
    }

    pub(super) fn build_text(this: Gd<Self>, sequence: Gd<SpireSequence>) {
        let state = this.bind();
        let label = state
            .resolve_target()
            .and_then(|node| node.try_cast::<SpireRichTextLabel>().ok());
        // Range selection needs the current text; replay preserves unchanged state.
        if let Some(label) = &label {
            SpireRichTextLabel::refresh_text(label.clone(), false);
        }
        let (effect, label) = match state.resolve_effect_configuration() {
            Ok(configuration) => configuration,
            Err(warning) => {
                godot_warn!("[TweenBlueprint] {warning:#}");
                return;
            }
        };
        let first = state.text_start_index;
        let last = if state.text_end_index == -1 {
            SpireRichTextLabel::get_character_count(label.clone()) - 1
        } else {
            state.text_end_index
        };
        let start = state.property_start_value.clone();
        let explicit = state.property_start_enabled;
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
            .range_tween(label, first, last, state.get_text_delay_ease(), state.text_blendable)
            .with_loops(state.text_loops, effect.loop_mode());

        let ease = state
            .property_ease_source
            .resolve(state.property_ease_enum, state.property_ease_curve.as_ref());
        if !matches!(ease, EaseKind::Basic(Ease::Default)) {
            tween = tween.with_ease(ease);
        }

        sequence.bind().to_mut().append(tween);
    }
}
