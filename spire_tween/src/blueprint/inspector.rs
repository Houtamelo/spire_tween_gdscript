use super::{
    tweenable_property_validation::{property_descriptor, suggested_properties, value_matches},
    *,
};

#[godot_api(secondary)]
impl TweenBlueprint {
    #[func(gd_self)]
    pub(super) fn initialize_blueprint_values(mut this: Gd<Self>) {
        if this.bind().kind == BlueprintKind::Characters {
            return;
        }
        let (target, name, unconfigured) = {
            let state = this.bind();
            (
                state.resolve_target(),
                state.animated_property.clone(),
                state.start_value.is_nil() && state.end_value.is_nil(),
            )
        };

        if !unconfigured {
            return;
        }

        if let Some(target) = target
            && let Ok(info) = property_descriptor(&target, &name)
        {
            let value = target.get_indexed(&NodePath::from(&info.property_name));
            let mut state = this.bind_mut();
            state.start_value = value.clone();
            state.end_value = value;
            drop(state);
            this.notify_property_list_changed();
        }
    }
}

pub(super) fn get_configuration_warnings(bp: &TweenBlueprint) -> PackedStringArray {
    let mut warnings = PackedStringArray::new();

    if !bp.delay.is_finite() || bp.delay < 0.0 {
        warnings.push("`delay` must be finite and non-negative.");
    }

    if !bp.speed_scale.is_finite() || bp.speed_scale < 0.0 {
        warnings.push("`speed_scale` must be finite and non-negative.");
    }

    if bp.base().get_script().is_none()
        && let BlueprintKind::Property = bp.kind
    {
        match bp.resolve_target() {
            None => {
                warnings.push("`target` could not be resolved. Assign a valid node or add a non-blueprint ancestor.")
            }
            Some(target) => match property_descriptor(&target, &bp.animated_property) {
                Ok(info) => {
                    if !value_matches(&bp.end_value, info.variant_type) {
                        warnings.push(&format!(
                            "`end_value` must match the type selected by `animated_property`: expected {}, got {}.",
                            info.variant_type.godot_type_name(),
                            bp.end_value.get_type().godot_type_name()
                        ));
                    }

                    if bp.start_enabled
                        && bp.lerp_mode == LerpModeKind::Absolute
                        && !value_matches(&bp.start_value, info.variant_type)
                    {
                        warnings.push(&format!(
                            "`start_value` must match the type selected by `animated_property`: expected {}, got {}.",
                            info.variant_type.godot_type_name(),
                            bp.start_value.get_type().godot_type_name()
                        ));
                    }
                }
                Err(err) => {
                    warnings
                        .push(&format!("Cannot resolve `animated_property` (`{}`):\n\t{}", bp.animated_property, err));
                }
            },
        }

        if !bp.speed_or_duration.is_finite() || bp.speed_or_duration <= 0.0 {
            warnings.push("`speed_or_duration` must be finite and greater than 0.0.");
        }
    }

    if bp.base().get_script().is_none()
        && bp.kind == BlueprintKind::Characters
        && let Err(warning) = bp.resolve_effect_configuration()
    {
        warnings.push(format!("{warning:#}").as_str());
    }

    if !bp.is_managed_by_parent()
        && let Some(message) = bp.trigger_warning()
    {
        warnings.push(&message);
    }

    warnings
}

pub(super) fn on_validate_property(bp: &TweenBlueprint, property: &mut PropertyInfo) {
    let scripted = bp.base().get_script().is_some();
    let characters = bp.kind == BlueprintKind::Characters;
    let name = property.property_name.to_string();
    let name = name.as_str();

    if let "kind"
    | "target"
    | "animated_property"
    | "start_enabled"
    | "lerp_mode"
    | "trigger_condition"
    | "character_effect"
    | "ease_source"
    | "character_delay_ease_source" = name
    {
        property.usage = property.usage.with(PropertyUsageFlags::UPDATE_ALL_IF_MODIFIED);
    }

    if let "kind" = name
        && scripted
    {
        property.usage = property
            .usage
            .with(PropertyUsageFlags::READ_ONLY)
            .without(PropertyUsageFlags::EDITOR);
    }

    if let "trigger_condition" = name
        && bp.is_managed_by_parent()
    {
        property.usage = property
            .usage
            .with(PropertyUsageFlags::READ_ONLY)
            .without(PropertyUsageFlags::EDITOR);
    }

    if let "trigger_source" = name
        && matches!(
            bp.trigger_condition,
            TriggerCondition::Manual | TriggerCondition::AfterReady | TriggerCondition::AfterEnterTree
        )
    {
        property.usage = property.usage.without(PropertyUsageFlags::EDITOR);
    }

    if let "animated_property" | "speed_or_duration" | "start_enabled" | "start_value" | "end_value" | "lerp_mode" =
        name
    {
        property.usage = property.usage.with(PropertyUsageFlags::STORAGE);
        let character_field = characters && matches!(name, "start_enabled" | "start_value");
        property.usage = if scripted || !(bp.kind == BlueprintKind::Property || character_field) {
            property.usage.without(PropertyUsageFlags::EDITOR)
        } else {
            property.usage.with(PropertyUsageFlags::EDITOR)
        };
    }

    if matches!(name, "ease_source" | "ease_enum" | "ease_curve") {
        let visible = !scripted
            && matches!(bp.kind, BlueprintKind::Property | BlueprintKind::Characters)
            && match name {
                "ease_enum" => bp.ease_source == EaseSource::Enum,
                "ease_curve" => bp.ease_source == EaseSource::Curve,
                _ => true,
            };
        property.usage = property.usage.with(PropertyUsageFlags::STORAGE);
        property.usage = if visible {
            property.usage.with(PropertyUsageFlags::EDITOR)
        } else {
            property.usage.without(PropertyUsageFlags::EDITOR)
        };
    }

    let impulse_start = if characters {
        bp.compiled_character_effect()
            .ok()
            .and_then(|effect| effect.impulse_start_default())
    } else {
        None
    };

    if characters && name == "start_value" && impulse_start.is_none() {
        property.usage = property.usage.without(PropertyUsageFlags::EDITOR);
    }

    if matches!(
        name,
        "character_effect"
            | "character_first"
            | "character_last"
            | "character_delay_ease_source"
            | "character_delay_ease_enum"
            | "character_delay_ease_curve"
            | "character_loops"
            | "character_blendable"
    ) {
        let visible = !scripted
            && characters
            && match name {
                "character_blendable" => impulse_start.is_some(),
                "character_delay_ease_enum" => bp.character_delay_ease_source == EaseSource::Enum,
                "character_delay_ease_curve" => bp.character_delay_ease_source == EaseSource::Curve,
                _ => true,
            };
        property.usage = property.usage.with(PropertyUsageFlags::STORAGE);
        if !visible {
            property.usage = property.usage.without(PropertyUsageFlags::EDITOR);
        }
    }

    if name == "start_enabled" && !characters && bp.lerp_mode != LerpModeKind::Absolute
        || name == "start_value" && (!bp.start_enabled || !characters && bp.lerp_mode != LerpModeKind::Absolute)
    {
        property.usage = property.usage.without(PropertyUsageFlags::EDITOR);
    }

    let target = bp.resolve_target();

    if let "animated_property" = name {
        property.hint_info = PropertyHintInfo::none();
        if let Some(target) = &target
            && let Ok(names) = suggested_properties(target)
        {
            let separator = GString::from(",");

            property.hint_info.hint = PropertyHint::ENUM_SUGGESTION;
            property.hint_info.hint_string = separator.join(&names);
        }
    } else if let "start_value" | "end_value" = name {
        if scripted || !matches!(bp.kind, BlueprintKind::Property | BlueprintKind::Characters) {
            return;
        }
        if characters {
            let Some(default) = impulse_start.filter(|_| name == "start_value") else { return };
            let ty = default.get_type();
            let value = &bp.start_value;
            if value.is_nil() || value.get_type() == ty {
                property.variant_type = ty;
                property.class_name = StringName::default();
                property.hint_info = PropertyHintInfo::none();
                property.usage = property.usage.without(PropertyUsageFlags::NIL_IS_VARIANT);
            }
            return;
        }
        let Some(target) = target else { return };

        match property_descriptor(&target, &bp.animated_property) {
            Ok(info) => {
                let value = if let "start_value" = name { &bp.start_value } else { &bp.end_value };
                // Retain incompatible payloads for explicit repair; inspection never rewrites authored values.
                if value.is_nil() || value.get_type() == info.variant_type {
                    property.variant_type = info.variant_type;
                    property.class_name = info.class_name;
                    property.hint_info = info.hint_info;
                    property.usage = property.usage.without(PropertyUsageFlags::NIL_IS_VARIANT);
                }
            }
            // The Scene dock already presents configuration warnings. Metadata
            // queries also run while authoring incomplete steps and must stay quiet.
            Err(_) => {}
        }
    }
}
