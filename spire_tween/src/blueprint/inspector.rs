use super::{
    tweenable_property_validation::{property_descriptor, suggested_properties, value_matches},
    *,
};

#[godot_api(secondary)]
impl TweenBlueprint {
    #[func(gd_self)]
    pub(super) fn initialize_blueprint_values(mut this: Gd<Self>) {
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

    pub(super) fn validate_inspector_property(&self, property: &mut PropertyInfo) {
        let scripted = self.base().get_script().is_some();
        let name = property.property_name.to_string();
        let name = name.as_str();

        if let "kind" | "target" | "animated_property" | "start_enabled" | "lerp_mode" | "trigger_condition" = name {
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
            && self.is_managed_by_parent()
        {
            property.usage = property
                .usage
                .with(PropertyUsageFlags::READ_ONLY)
                .without(PropertyUsageFlags::EDITOR);
        }

        if let "trigger_source" = name
            && matches!(
                self.trigger_condition,
                TriggerCondition::Manual | TriggerCondition::AfterReady | TriggerCondition::AfterEnterTree
            )
        {
            property.usage = property.usage.without(PropertyUsageFlags::EDITOR);
        }

        if let "animated_property" | "speed_or_duration" | "start_enabled" | "start_value" | "end_value" | "lerp_mode"
        | "ease_basic" | "ease_curve" = name
        {
            property.usage = property.usage.with(PropertyUsageFlags::STORAGE);
            property.usage = if scripted || self.kind != BlueprintKind::Property {
                property.usage.without(PropertyUsageFlags::EDITOR)
            } else {
                property.usage.with(PropertyUsageFlags::EDITOR)
            };
        }

        if name == "start_enabled" && self.lerp_mode != LerpModeKind::Absolute
            || name == "start_value" && (!self.start_enabled || self.lerp_mode != LerpModeKind::Absolute)
        {
            property.usage = property.usage.without(PropertyUsageFlags::EDITOR);
        }

        let target = self.resolve_target();

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
            let Some(target) = target else { return };

            match property_descriptor(&target, &self.animated_property) {
                Ok(info) => {
                    let value = if let "start_value" = name { &self.start_value } else { &self.end_value };
                    // Retain incompatible payloads for explicit repair; inspection never rewrites authored values.
                    if value.is_nil() || value.get_type() == info.variant_type {
                        property.variant_type = info.variant_type;
                        property.class_name = info.class_name;
                        property.hint_info = info.hint_info;
                        property.usage = property.usage.without(PropertyUsageFlags::NIL_IS_VARIANT);
                    }
                }
                Err(err) => {
                    godot_warn!("{err}");
                }
            }
        }
    }

    pub(super) fn configuration_warnings(&self) -> PackedStringArray {
        let mut warnings = PackedStringArray::new();

        if !self.delay.is_finite() || self.delay < 0.0 {
            warnings.push("`delay` must be finite and non-negative.");
        }

        if !self.speed_scale.is_finite() || self.speed_scale < 0.0 {
            warnings.push("`speed_scale` must be finite and non-negative.");
        }

        if self.base().get_script().is_none()
            && let BlueprintKind::Property = self.kind
        {
            match self.resolve_target() {
                None => warnings
                    .push("`target` could not be resolved. Assign a valid node or add a non-blueprint ancestor."),
                Some(target) => match property_descriptor(&target, &self.animated_property) {
                    Ok(info) => {
                        if !value_matches(&self.end_value, info.variant_type) {
                            warnings.push(&format!(
                                "`end_value` must match the type selected by `animated_property`: expected {}, got {}.",
                                info.variant_type.godot_type_name(),
                                self.end_value.get_type().godot_type_name()
                            ));
                        }

                        if self.start_enabled
                            && self.lerp_mode == LerpModeKind::Absolute
                            && !value_matches(&self.start_value, info.variant_type)
                        {
                            warnings.push(&format!(
                                "`start_value` must match the type selected by `animated_property`: expected {}, got \
                                 {}.",
                                info.variant_type.godot_type_name(),
                                self.start_value.get_type().godot_type_name()
                            ));
                        }
                    }
                    Err(err) => {
                        warnings.push(&format!(
                            "Cannot resolve `animated_property` (`{}`):\n\t{}",
                            self.animated_property, err
                        ));
                    }
                },
            }

            if !self.speed_or_duration.is_finite() || self.speed_or_duration <= 0.0 {
                warnings.push("`speed_or_duration` must be finite and greater than 0.0.");
            }
        }

        if !self.is_managed_by_parent()
            && let Some(message) = self.trigger_warning()
        {
            warnings.push(&message);
        }

        warnings
    }
}
