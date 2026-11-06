use anyhow::Result;

use super::*;

/// Resolves a writable, supported property, including indexed values such as `position:x`.
pub(super) fn property_descriptor(target: &Gd<Node>, path: &StringName) -> Result<PropertyInfo> {
    validate_target(target)?;

    let path_string = path.to_string();
    let path = path_string.trim();
    if path.is_empty() {
        bail!("`animated_property` must be non-empty.");
    }

    let (property_name, indexer) = match path.split_once(':') {
        Some((name, indexer)) => (name, Some(indexer)),
        None => (path, None),
    };
    let mut info = target
        .get_property_list()
        .iter_shared()
        .find(|dict| dict.get("name").is_some_and(|value| value.to_string() == property_name))
        .ok_or_else(|| anyhow!("`animated_property` (`{property_name}`) was not found on the resolved `target`."))
        .and_then(|dict| property_info_from_dict(&dict))?;

    validate_property_usage(&info)?;
    let owner_class = StringName::from(&target.get_class());
    validate_property_setter(&info, &owner_class, &mut ClassDb::singleton())?;

    if indexer.is_some() {
        let value = target.get_indexed(path);
        if value.is_nil() {
            bail!("`animated_property` (`{path}`) could not be read from the resolved `target`.");
        }

        info = PropertyInfo {
            variant_type: value.get_type(),
            class_name: StringName::default(),
            property_name: StringName::from(path),
            hint_info: PropertyHintInfo::none(),
            usage: Default::default(),
        };
    }

    validate_property_type(&info)?;
    Ok(info)
}

/// Uses the shared validation rules, then applies the additional suggestion exclusions.
pub(super) fn suggested_properties(target: &Gd<Node>) -> Result<PackedStringArray> {
    validate_target(target)?;
    let owner_class = StringName::from(&target.get_class());
    let mut class_db = ClassDb::singleton();

    Ok(PackedStringArray::from_iter(
        target
            .get_property_list()
            .iter_shared()
            .filter_map(|dict| property_info_from_dict(&dict).ok())
            .filter_map(|info| {
                should_suggest(&info, &owner_class, &mut class_db).then_some(info.property_name.to_gstring())
            }),
    ))
}

pub(super) fn value_matches(value: &Variant, expected: VariantType) -> bool {
    value.get_type() == expected
        || matches!(
            (value.get_type(), expected),
            (VariantType::INT, VariantType::FLOAT) | (VariantType::FLOAT, VariantType::INT)
        )
}

fn validate_target(target: &Gd<Node>) -> Result<()> {
    if !target.is_instance_valid() {
        bail!("`target` must reference a valid node.");
    }

    let blueprint_class = TweenBlueprint::class_id().to_gstring();
    if target.is_class(&blueprint_class) {
        bail!("`target` cannot reference a `{blueprint_class}`.");
    }

    Ok(())
}

fn validate_property_usage(info: &PropertyInfo) -> Result<()> {
    if info.usage.is_set(PropertyUsageFlags::READ_ONLY) {
        bail!("`animated_property` (`{}`) is read-only.", info.property_name);
    }

    Ok(())
}

fn validate_property_setter(info: &PropertyInfo, owner_class: &StringName, class_db: &mut Gd<ClassDb>) -> Result<()> {
    // Dynamic properties can be writable without a ClassDB setter. Only reject
    // properties that ClassDB explicitly exposes through a getter without a setter.
    if !class_db
        .class_get_property_getter(owner_class, &info.property_name)
        .is_empty()
        && class_db
            .class_get_property_setter(owner_class, &info.property_name)
            .is_empty()
    {
        bail!("`animated_property` (`{}`) must be writable.", info.property_name);
    }

    Ok(())
}

fn validate_property_type(info: &PropertyInfo) -> Result<()> {
    if !property_is_type_lerpable(info.variant_type) {
        bail!(
            "`animated_property` (`{}`) has unsupported type `{}`.",
            info.property_name,
            info.variant_type.godot_type_name()
        );
    }

    Ok(())
}

fn property_info_from_dict(dict: &VarDictionary) -> Result<PropertyInfo> {
    // Required fields
    let variant_type = dict
        .get("type")
        .and_then(|v| v.try_to::<VariantType>().ok())
        .ok_or_else(|| {
            anyhow!("Expected Dictionary to contain the entry `\"type\":VariantType`.\n\tDictionary: {dict}")
        })?;

    let property_name = dict
        .get("name")
        .and_then(|v| v.try_to_relaxed::<StringName>().ok())
        .ok_or_else(|| {
            anyhow!("Expected Dictionary to contain the entry `\"name\":String or StringName`.\n\tDictionary: {dict}")
        })?;

    // Optional fields
    let class_name = dict
        .get("class_name")
        .and_then(|v| v.try_to_relaxed::<StringName>().ok())
        .unwrap_or_default();

    let hint_info = {
        let hint = dict
            .get("hint")
            .and_then(|v| v.try_to::<PropertyHint>().ok())
            .unwrap_or(PropertyHint::NONE);
        let hint_string = dict
            .get("hint_string")
            .and_then(|v| v.try_to::<GString>().ok())
            .unwrap_or_default();
        PropertyHintInfo { hint, hint_string }
    };

    let usage = dict
        .get("usage")
        .and_then(|v| v.try_to::<PropertyUsageFlags>().ok())
        .unwrap_or(PropertyUsageFlags::NONE);

    Ok(PropertyInfo {
        variant_type,
        property_name,
        class_name,
        hint_info,
        usage,
    })
}

fn should_suggest(property_info: &PropertyInfo, owner_class: &StringName, class_db: &mut Gd<ClassDb>) -> bool {
    if validate_property_type(property_info).is_err() || validate_property_usage(property_info).is_err() {
        return false;
    }

    if property_has_hint_to_avoid(property_info.hint_info.hint) {
        return false;
    }

    if property_info.usage.is_set(PropertyUsageFlags::SCRIPT_VARIABLE) {
        true
    } else {
        !property_has_name_to_avoid(&property_info.property_name.to_string())
            && validate_property_setter(property_info, owner_class, class_db).is_ok()
    }
}

fn property_is_type_lerpable(ty: VariantType) -> bool {
    match ty {
        | VariantType::INT
        | VariantType::FLOAT
        | VariantType::STRING
        | VariantType::COLOR
        | VariantType::VECTOR2
        | VariantType::VECTOR2I
        | VariantType::VECTOR3
        | VariantType::VECTOR3I
        | VariantType::VECTOR4
        | VariantType::QUATERNION
        | VariantType::RECT2
        | VariantType::RECT2I => true,
        _ => false,
    }
}

fn property_has_hint_to_avoid(hint: PropertyHint) -> bool {
    match hint {
        | PropertyHint::ENUM
        | PropertyHint::ENUM_SUGGESTION
        | PropertyHint::EXP_EASING
        | PropertyHint::FLAGS
        | PropertyHint::LAYERS_2D_RENDER
        | PropertyHint::LAYERS_2D_PHYSICS
        | PropertyHint::LAYERS_2D_NAVIGATION
        | PropertyHint::LAYERS_3D_RENDER
        | PropertyHint::LAYERS_3D_PHYSICS
        | PropertyHint::LAYERS_3D_NAVIGATION
        | PropertyHint::LAYERS_AVOIDANCE
        | PropertyHint::FILE
        | PropertyHint::DIR
        | PropertyHint::GLOBAL_FILE
        | PropertyHint::GLOBAL_DIR
        | PropertyHint::OBJECT_ID
        | PropertyHint::TYPE_STRING
        | PropertyHint::SAVE_FILE
        | PropertyHint::GLOBAL_SAVE_FILE
        | PropertyHint::INT_IS_POINTER
        | PropertyHint::LOCALE_ID
        | PropertyHint::PASSWORD => true,
        #[cfg(not(feature = "api-4-4"))]
        PropertyHint::INPUT_NAME | PropertyHint::FILE_PATH => true,
        _ => false,
    }
}

fn property_has_name_to_avoid(name: &str) -> bool {
    let name_trimmed = match name.split_once(':') {
        Some((p, _)) => p,
        None => name,
    }
    .trim();

    match name_trimmed {
        // Node
        "editor_description" |
        "name" |
        "process_physics_priority" |
        "process_priority" |
        "process_thread_group_order" |
        "scene_file_path" |
        // Audio related
        "audio_max_polyphony" |
        "playback_auto_capture_duration" |
        "playback_default_blend_time" |
        "max_polyphony" |
        "audio_track" |
        "buffering_msec" |
        // Control
        "accessibility_description" |
        "accessibility_name" |
        "size_flags_stretch_ratio" |
        "current_tab" |
        "custom_word_separators" |
        "item_count" |
        "max_length" |
        "progress_bar_delay" |
        "search_bar_fuzzy_search_max_misses" |
        "search_bar_min_item_count" |
        "secret_character" |
        "selected" |
        "start_index" |
        "submenu_popup_delay" |
        "tab_count" |
        "tabs_rearrange_group" |
        "uri" |
        // Skeleton and IK configuration
        "bone" |
        "bone2d_index" |
        "bone_idx" |
        "chain_count" |
        "max_iterations" |
        "origin_bone" |
        "setting_count" |
        // Physics solver and reporting configuration
        "collision_priority" |
        "max_contacts_reported" |
        "max_slides" |
        "simulation_precision" |
        "solver_priority" |
        // Geometry, atlas layout, batching, and debug visualization
        "cell_octant_size" |
        "debug_color" |
        "debug_custom_color" |
        "gi_lightmap_texel_scale" |
        "hframes" |
        "internal_vertex_count" |
        "path_interval" |
        "path_simplify_angle" |
        "physics_quadrant_size" |
        "radial_segments" |
        "rendering_quadrant_size" |
        "ring_sides" |
        "rings" |
        "round_precision" |
        "sides" |
        "spin_sides" |
        "vframes" |
        // Particle allocation, simulation setup, and culling
        "amount" |
        "draw_passes" |
        "fixed_fps" |
        "preprocess" |
        "seed" |
        "trail_section_subdivisions" |
        "trail_sections" |
        "visibility_rect" |
        // Viewport and window configuration
        "current_screen" |
        "gui_drag_threshold" |
        "oversampling_override" |
        "positional_shadow_atlas_size" |
        "view_count" => true,
        _ => false,
    }
}
