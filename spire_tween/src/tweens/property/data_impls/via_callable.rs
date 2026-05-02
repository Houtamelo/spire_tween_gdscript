use std::marker::PhantomData;

use super::*;

/// Property-data adapter backed by a pair of [`Callable`]s: one to read the value,
/// one to write it. Use this when the property you want to tween isn't a plain
/// reflectable field — for example when it's a private field exposed via getter/setter
/// methods, or when you need to bind extra arguments (such as a bone index for
/// `Skeleton3D::set_bone_pose_position`).
///
/// The `getter` must take no arguments and return `T`. The `setter` must take a single
/// `T` argument. Both are typically constructed via `Callable::from_fn` (Rust closures)
/// or `Callable::from_object_method` (object methods).
///
/// # Example: tween a Skeleton3D bone position
///
/// ```ignore
/// let getter_node = skeleton.clone();
/// let getter = Callable::from_fn("get_bone_pose_position_bound", move |_| {
///     getter_node.get_bone_pose_position(bone_idx).to_variant()
/// });
/// let mut setter_node = skeleton.clone();
/// let setter = Callable::from_fn("set_bone_pose_position_bound", move |args| {
///     let value: Vector3 = args.first()
///         .and_then(|v| v.try_to_relaxed::<Vector3>().ok())
///         .unwrap_or_default();
///     setter_node.set_bone_pose_position(bone_idx, value);
///     Variant::nil()
/// });
///
/// let data: PropertyDataVec3 = PropertyDataViaCallable::new(getter, setter).into();
/// SpireTween::<LerpPropertyData<Vector3>>::new(data, Evaluator::Static(target), duration)
///     .register();
/// ```
///
/// (See `tweens/templates/do_bone.rs` for the trait that wraps this pattern.)
///
/// **Note:** `Callable::bind` *appends* bound args after call args (per godot's
/// documented semantics), so it can't bind a *leading* parameter. When you need to
/// fix a leading argument, wrap with `Callable::from_fn` instead.
#[derive(Debug, Clone)]
pub struct PropertyDataViaCallable<T> {
    /// Auto-derived from the getter's underlying object (if any), so the resulting
    /// tween auto-stops when that object is freed. Override after construction if you
    /// need a different lifetime owner.
    pub owner: Option<ObjectOrNode>,
    /// Invoked each tick (or on `force_complete`) to read the property's current
    /// value. Must return `T`.
    pub getter: Callable,
    /// Invoked each tick with the new value. Must accept a single `T` argument.
    pub setter: Callable,
    /// Type-binding marker — the type tweened is determined by `T`, not by anything
    /// the callables encode at the type level.
    pub _pd: PhantomData<T>,
}

impl<T> PropertyDataViaCallable<T> {
    /// Constructs the adapter and auto-derives `owner` from the getter's underlying
    /// object (if any). Provide both callables fully formed — `bind` extra args via
    /// `Callable::from_fn` rather than `Callable::bind` (the latter appends, see the
    /// type-level note).
    pub fn new(getter: Callable, setter: Callable) -> Self {
        Self {
            owner: getter.object().map(ObjectOrNode::from_unchecked_object),
            getter,
            setter,
            _pd: PhantomData,
        }
    }
}

impl<T: Default + FromGodot + ToGodot> IProperty<T> for PropertyDataViaCallable<T> {
    fn get_property_value(&self) -> T {
        self.getter
            .call(&[])
            .try_to_relaxed::<T>()
            .log_if_err()
            .unwrap_or_default()
    }

    fn set_property_value(&mut self, value: T) { self.setter.call(&[value.to_variant()]); }
}

impl<T> IPropertyData for PropertyDataViaCallable<T> {
    type Target = Object;

    fn get_property_path(&self) -> NodePath { self.getter.method_name().as_ref().map(NodePath::from).unwrap_or_default() }

    fn get_owner(&self) -> Option<&ObjectOrNode> { self.owner.as_ref() }
}

impl<T> TryFromPathAndObject for PropertyDataViaCallable<T> {}
