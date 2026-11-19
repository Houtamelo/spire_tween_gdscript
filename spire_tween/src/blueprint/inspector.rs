use super::{
    tweenable_property_validation::{property_descriptor, suggested_properties, value_matches},
    *,
};

#[godot_api(secondary)]
impl TweenBlueprint {
    #[func(gd_self)]
    pub(super) fn initialize_blueprint_values(mut this: Gd<Self>) {
        if this.bind().kind == BlueprintKind::Text {
            return;
        }
        let (target, name, unconfigured) = {
            let state = this.bind();
            (
                state.resolve_target(),
                state.property_to_animate.clone(),
                state.property_start_value.is_nil() && state.property_end_value.is_nil(),
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
            state.property_start_value = value.clone();
            state.property_end_value = value;
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
            Some(target) => match property_descriptor(&target, &bp.property_to_animate) {
                Ok(info) => {
                    if !value_matches(&bp.property_end_value, info.variant_type) {
                        warnings.push(&format!(
                            "`property_end_value` must match the type selected by `property_to_animate`: expected {}, \
                             got {}.",
                            info.variant_type.godot_type_name(),
                            bp.property_end_value.get_type().godot_type_name()
                        ));
                    }

                    if bp.property_start_enabled
                        && bp.property_lerp_mode == LerpModeKind::Absolute
                        && !value_matches(&bp.property_start_value, info.variant_type)
                    {
                        warnings.push(&format!(
                            "`property_start_value` must match the type selected by `property_to_animate`: expected \
                             {}, got {}.",
                            info.variant_type.godot_type_name(),
                            bp.property_start_value.get_type().godot_type_name()
                        ));
                    }
                }
                Err(err) => {
                    warnings.push(&format!(
                        "Cannot resolve `property_to_animate` (`{}`):\n\t{}",
                        bp.property_to_animate, err
                    ));
                }
            },
        }

        if !bp.property_speed_or_duration.is_finite() || bp.property_speed_or_duration <= 0.0 {
            warnings.push("`property_speed_or_duration` must be finite and greater than 0.0.");
        }
    }

    if bp.base().get_script().is_none()
        && bp.kind == BlueprintKind::Text
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
    let kind_is_scripted = bp.base().get_script().is_some();
    let kind_is_text = bp.kind == BlueprintKind::Text;
    let name = property.property_name.to_string();
    let name = name.as_str();

    if let "kind"
    | "target"
    | "property_to_animate"
    | "property_start_enabled"
    | "property_lerp_mode"
    | "trigger_condition"
    | "text_effect"
    | "property_ease_source"
    | "text_delay_ease_source" = name
    {
        property.usage = property.usage.with(PropertyUsageFlags::UPDATE_ALL_IF_MODIFIED);
    }

    if let "kind" = name
        && kind_is_scripted
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

    if let "property_to_animate"
    | "property_speed_or_duration"
    | "property_start_enabled"
    | "property_start_value"
    | "property_end_value"
    | "property_lerp_mode" = name
    {
        property.usage = property.usage.with(PropertyUsageFlags::STORAGE);
        let character_field = kind_is_text && matches!(name, "property_start_enabled" | "property_start_value");
        property.usage = if kind_is_scripted || !(bp.kind == BlueprintKind::Property || character_field) {
            property.usage.without(PropertyUsageFlags::EDITOR)
        } else {
            property.usage.with(PropertyUsageFlags::EDITOR)
        };
    }

    if matches!(name, "property_ease_source" | "property_ease_enum" | "property_ease_curve") {
        let visible = !kind_is_scripted
            && matches!(bp.kind, BlueprintKind::Property | BlueprintKind::Text)
            && match name {
                "property_ease_enum" => bp.property_ease_source == EaseSource::Enum,
                "property_ease_curve" => bp.property_ease_source == EaseSource::Curve,
                _ => true,
            };
        property.usage = property.usage.with(PropertyUsageFlags::STORAGE);
        property.usage = if visible {
            property.usage.with(PropertyUsageFlags::EDITOR)
        } else {
            property.usage.without(PropertyUsageFlags::EDITOR)
        };
    }

    let impulse_start = if kind_is_text {
        bp.get_compiled_text_effect()
            .ok()
            .and_then(|effect| effect.impulse_start_default())
    } else {
        None
    };

    if kind_is_text && name == "property_start_value" && impulse_start.is_none() {
        property.usage = property.usage.without(PropertyUsageFlags::EDITOR);
    }

    if matches!(
        name,
        "text_effect"
            | "text_start_index"
            | "text_end_index"
            | "text_delay_ease_source"
            | "text_delay_ease_enum"
            | "text_delay_ease_curve"
            | "text_loops"
            | "text_blendable"
    ) {
        let visible = !kind_is_scripted
            && kind_is_text
            && match name {
                "text_blendable" => impulse_start.is_some(),
                "text_delay_ease_enum" => bp.text_delay_ease_source == EaseSource::Enum,
                "text_delay_ease_curve" => bp.text_delay_ease_source == EaseSource::Curve,
                _ => true,
            };
        property.usage = property.usage.with(PropertyUsageFlags::STORAGE);
        if !visible {
            property.usage = property.usage.without(PropertyUsageFlags::EDITOR);
        }
    }

    if name == "property_start_enabled" && !kind_is_text && bp.property_lerp_mode != LerpModeKind::Absolute
        || name == "property_start_value"
            && (!bp.property_start_enabled || !kind_is_text && bp.property_lerp_mode != LerpModeKind::Absolute)
    {
        property.usage = property.usage.without(PropertyUsageFlags::EDITOR);
    }

    let target = bp.resolve_target();

    if let "property_to_animate" = name {
        property.hint_info = PropertyHintInfo::none();
        if let Some(target) = &target
            && let Ok(names) = suggested_properties(target)
        {
            let separator = GString::from(",");

            property.hint_info.hint = PropertyHint::ENUM_SUGGESTION;
            property.hint_info.hint_string = separator.join(&names);
        }
    } else if let "property_start_value" | "property_end_value" = name {
        if kind_is_scripted || !matches!(bp.kind, BlueprintKind::Property | BlueprintKind::Text) {
            return;
        }
        if kind_is_text {
            let Some(default) = impulse_start.filter(|_| name == "property_start_value") else { return };
            let ty = default.get_type();
            let value = &bp.property_start_value;
            if value.is_nil() || value.get_type() == ty {
                property.variant_type = ty;
                property.class_name = StringName::default();
                property.hint_info = PropertyHintInfo::none();
                property.usage = property.usage.without(PropertyUsageFlags::NIL_IS_VARIANT);
            }
            return;
        }
        let Some(target) = target else { return };

        match property_descriptor(&target, &bp.property_to_animate) {
            Ok(info) => {
                let value =
                    if let "property_start_value" = name { &bp.property_start_value } else { &bp.property_end_value };
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
