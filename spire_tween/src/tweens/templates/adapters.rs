use std::{cell::RefCell, rc::Rc};

use super::*;

/// Builds a sequence visiting each destination, applying easing per segment.
/// Arrays must be nonempty and equally sized; durations must be finite and
/// nonnegative. Validation happens before any child is constructed. Each segment
/// reads its starting value when reached, using ordinary sequence replay rules.
/// Returns an unregistered sequence. Generic accessors are Rust-only.
pub fn do_get_set_array<T>(
    getter: impl Fn() -> T + 'static,
    setter: impl FnMut(T) + 'static,
    values: Vec<T>,
    durations: Vec<f64>,
) -> anyhow::Result<SpireTween<Sequence>>
where
    T: PropertyType + FromGodot + ToGodot + Default + 'static,
    T::Data: From<PropertyDataViaCallable<T>>,
    T::Lerper: Default,
    LerpPropertyData<T>: ITweenable,
    AnyTween: From<TweenPtr<LerpPropertyData<T>>>,
{
    let getter = Rc::new(getter);
    let setter = Rc::new(RefCell::new(setter));
    sequence_from_values(values, durations, move |to, duration| {
        let getter = getter.clone();
        let setter = setter.clone();
        do_get_set(move || getter(), move |value| setter.borrow_mut()(value), to, duration)
    })
}

/// Property-path counterpart of [`do_get_set_array`], suitable for both Nodes and
/// Resources. GDScript exposes the Vector2 and Vector3 forms on `Spire`.
pub fn do_property_array<T>(
    owner: Gd<Object>,
    path: NodePath,
    values: Vec<T>,
    durations: Vec<f64>,
) -> anyhow::Result<SpireTween<Sequence>>
where
    T: PropertyType,
    T::Lerper: Default,
    LerpPropertyData<T>: ITweenable,
    AnyTween: From<TweenPtr<LerpPropertyData<T>>>,
{
    sequence_from_values(values, durations, move |to, duration| {
        owner.do_property(&path, Evaluator::Static(to), duration)
    })
}

fn sequence_from_values<T: PropertyType>(
    values: Vec<T>,
    durations: Vec<f64>,
    mut build: impl FnMut(T, f64) -> SpireTween<LerpPropertyData<T>>,
) -> anyhow::Result<SpireTween<Sequence>>
where
    LerpPropertyData<T>: ITweenable,
    AnyTween: From<TweenPtr<LerpPropertyData<T>>>,
{
    if values.is_empty() || values.len() != durations.len() {
        bail!("Destinations and durations must be nonempty and have the same length.");
    }
    if durations.iter().any(|d| !d.is_finite() || *d < 0.0) {
        bail!("Segment durations must be finite and nonnegative.");
    }
    let mut sequence = SpireTween::<Sequence>::new();
    for (to, duration) in values.into_iter().zip(durations) {
        sequence.append(build(to, duration));
    }
    Ok(sequence)
}

/// Creates an unregistered typed shader-uniform tween. The existing uniform must
/// be convertible to `T`; invalid names/types return an error before construction.
/// The material is retained. Shared materials are animated for all their users.
/// Renderer defaults are used when no override exists. If unavailable (including
/// headless mode), explicitly initialize the uniform with `set_shader_parameter`.
/// Replacing the Shader resource cancels existing uniform tweens.
pub fn do_shader_parameter<T>(
    material: Gd<ShaderMaterial>,
    parameter: StringName,
    to: T,
    duration: f64,
) -> anyhow::Result<SpireTween<LerpPropertyData<T>>>
where
    T: PropertyType + FromGodot + ToGodot + Default + 'static,
    T::Data: From<PropertyDataViaCallable<T>>,
    T::Lerper: Default,
    LerpPropertyData<T>: ITweenable,
    AnyTween: From<TweenPtr<LerpPropertyData<T>>>,
{
    let shader_id = validate_shader_parameter::<T>(&material, &parameter)?;
    let read = material.clone();
    let read_name = parameter.clone();
    let check = material.clone();
    let check_name = parameter.clone();
    let mut write = material.clone();
    let mut data = typed_accessors(
        move || {
            shader_value(&read, &read_name)
                .try_to_relaxed::<T>()
                .expect("validated shader uniform")
        },
        move |value: T| write.set_shader_parameter(&parameter, &value.to_variant()),
    );
    data.owner = Some(ObjectOrNode::Object(material.upcast()));
    data.validation = Some(Callable::from_fn("spire_shader_uniform_valid", move |_| {
        (check
            .get_shader()
            .is_some_and(|shader| shader.instance_id() == shader_id)
            && shader_value(&check, &check_name).try_to_relaxed::<T>().is_ok())
        .to_variant()
    }));
    Ok(SpireTween::<LerpPropertyData<T>>::new(data.into(), Evaluator::Static(to), duration))
}

/// Animates only a Color shader uniform's alpha, preserving its current RGB on
/// every write. Missing/incompatible uniforms return an error; the material is
/// retained, and replacing the uniform with an incompatible type cancels playback.
pub fn do_shader_parameter_alpha(
    material: Gd<ShaderMaterial>,
    parameter: StringName,
    to: f64,
    duration: f64,
) -> anyhow::Result<SpireTween<LerpPropertyData<f64>>> {
    let shader_id = validate_shader_parameter::<Color>(&material, &parameter)?;
    let read = material.clone();
    let read_name = parameter.clone();
    let check = material.clone();
    let check_name = parameter.clone();
    let mut write = material.clone();
    let mut data = typed_accessors(
        move || shader_value(&read, &read_name).to::<Color>().a as f64,
        move |alpha: f64| {
            let mut color = shader_value(&write, &parameter).to::<Color>();
            color.a = alpha as f32;
            write.set_shader_parameter(&parameter, &color.to_variant());
        },
    );
    data.owner = Some(ObjectOrNode::Object(material.upcast()));
    data.validation = Some(Callable::from_fn("spire_shader_color_valid", move |_| {
        (check
            .get_shader()
            .is_some_and(|shader| shader.instance_id() == shader_id)
            && shader_value(&check, &check_name).try_to::<Color>().is_ok())
        .to_variant()
    }));
    Ok(SpireTween::<LerpPropertyData<f64>>::new(data.into(), Evaluator::Static(to), duration))
}

fn shader_value(material: &Gd<ShaderMaterial>, parameter: &StringName) -> Variant {
    let value = material.get_shader_parameter(parameter);
    if !value.is_nil() {
        return value;
    }
    material
        .get_shader()
        .map(|shader| RenderingServer::singleton().shader_get_parameter_default(shader.get_rid(), parameter))
        .unwrap_or_default()
}

fn validate_shader_parameter<T: Default + FromGodot + ToGodot>(
    material: &Gd<ShaderMaterial>,
    parameter: &StringName,
) -> anyhow::Result<InstanceId> {
    let shader = material
        .get_shader()
        .ok_or_else(|| anyhow!("ShaderMaterial has no shader."))?;
    let expected = T::default().to_variant().get_type().ord() as i64;
    let declared = shader.get_shader_uniform_list().iter_shared().any(|uniform| {
        let uniform = uniform.to::<VarDictionary>();
        uniform.get("name").is_some_and(|value| {
            value
                .try_to::<GString>()
                .is_ok_and(|name| name == GString::from(parameter))
        }) && uniform
            .get("type")
            .is_some_and(|value| value.try_to::<i64>().is_ok_and(|kind| kind == expected))
    });
    if !declared {
        bail!("Shader parameter `{parameter}` is missing or has an incompatible declared type.");
    }
    if shader_value(material, parameter).try_to_relaxed::<T>().is_err() {
        bail!(
            "Shader parameter `{parameter}` has no readable compatible value. Set its initial value with \
             set_shader_parameter (required when renderer defaults are unavailable, including headless mode)."
        );
    }
    Ok(shader.instance_id())
}

/// Tweens an existing named audio bus in decibels. Resolves its name on each
/// update, so bus reordering is harmless. Removal/renaming cancels playback;
/// no completion signal is emitted. Returns an unregistered property tween.
pub fn do_bus_volume_db(name: StringName, to: f64, duration: f64) -> anyhow::Result<SpireTween<LerpPropertyData<f64>>> {
    bus_volume(name, to, duration, false)
}

/// Linear-gain counterpart of [`do_bus_volume_db`]. Zero is silence, one is unity
/// gain, and targets above one are accepted without clamping.
pub fn do_bus_volume_linear(
    name: StringName,
    to: f64,
    duration: f64,
) -> anyhow::Result<SpireTween<LerpPropertyData<f64>>> {
    bus_volume(name, to, duration, true)
}

fn bus_volume(
    name: StringName,
    to: f64,
    duration: f64,
    linear: bool,
) -> anyhow::Result<SpireTween<LerpPropertyData<f64>>> {
    if AudioServer::singleton().get_bus_index(&name) < 0 {
        bail!("Audio bus `{name}` does not exist.");
    }
    let read_name = name.clone();
    let write_name = name.clone();
    let mut data = typed_accessors(
        move || {
            let server = AudioServer::singleton();
            let index = server.get_bus_index(&read_name);
            if linear { server.get_bus_volume_linear(index) as f64 } else { server.get_bus_volume_db(index) as f64 }
        },
        move |value: f64| {
            let mut server = AudioServer::singleton();
            let index = server.get_bus_index(&write_name);
            if linear {
                server.set_bus_volume_linear(index, value as f32);
            } else {
                server.set_bus_volume_db(index, value as f32);
            }
        },
    );
    data.validation = Some(Callable::from_fn("spire_bus_exists", move |_| {
        (AudioServer::singleton().get_bus_index(&name) >= 0).to_variant()
    }));
    Ok(SpireTween::<LerpPropertyData<f64>>::new(data.into(), Evaluator::Static(to), duration))
}
