use super::*;

/// "How to read & write the value of a property of type `T`." The minimum interface
/// a property-data adapter must satisfy.
///
/// Spire's generated bridge adapters (one per Godot class × tweenable property) all
/// implement this. For one-off custom properties, use [`PropertyDataViaCallable`]
/// (which takes two callables) instead of writing a fresh impl.
///
/// `get_property_value` is called every tick (and on `force_complete` — to seek to
/// the final value). `set_property_value` is called every tick with the interpolated
/// value.
pub trait IProperty<T> {
    fn get_property_value(&self) -> T;
    fn set_property_value(&mut self, value: T);
}

/// Auxiliary metadata that lets a property-data adapter participate in the wider
/// system: what node path it animates (for diagnostics & `get_property_path`) and
/// who owns it (for auto-stop on free + [`PauseMode::Bound`] checks).
///
/// `Target` narrows the kind of object the property belongs to (e.g. `Node2D` for
/// `position`-style properties). The auto-bind plumbing on the `Do*` traits uses
/// this to constrain the right `Inherits<…>` bound.
pub trait IPropertyData {
    /// Most specific class the property is defined on. For a property that only
    /// exists on `Node2D`, set this to `Node2D`. Used to narrow generic bounds.
    type Target: Inherits<Object>;

    /// Path of the property relative to [`get_owner`](Self::get_owner). For built-in
    /// properties this is the bare property name (e.g. `"position"`); for custom
    /// adapters returned via callables it can be empty.
    fn get_property_path(&self) -> NodePath;
    /// Owner object, used for liveness checks (auto-stop on free) and pause-mode
    /// gating. `None` means the tween has no owner-based lifetime.
    fn get_owner(&self) -> Option<&ObjectOrNode>;
}

/// Constructor used by [`DoProperty::do_property`] to look up the right specialized
/// data adapter for a `(NodePath, owner)` pair.
///
/// Generated bridge adapters dispatch this through a pre-baked table keyed on the
/// path string; if no specialization matches, a `PropertyDataCustom` fallback is
/// produced.
///
/// You typically don't implement this yourself — the `gdscript_bindgen` codegen
/// emits it for every `(class, property)` pair declared in
/// `tweenable_properties.json`.
pub trait IGeneralPropertyData: Sized {
    fn from_path_and_owner(path_str: &str, path: NodePath, owner: Gd<Object>) -> Self;
}

/// Type-level wiring: "this is a tweenable type, and here is the specific
/// property-data adapter `do_property` should pick when targeting it."
///
/// `Data` is the type the bridge constructs via [`IGeneralPropertyData::from_path_and_owner`]
/// for properties of this `Self`. Spire's generated code ties built-in types
/// (`f64`, `Vector2`, …) to enums like `PropertyDataFloat`, `PropertyDataVec2`, etc.
///
/// You only implement `PropertyType` if you're adding support for a brand-new
/// tweened type — the generator handles built-ins.
pub trait PropertyType: Sized + ILerpable {
    type Data: IProperty<Self> + IPropertyData + IGeneralPropertyData;
}

/// "Given a tweened type, what `Spire*` GDScript-facing handle wraps the
/// corresponding tween?" Used by the `register` paths to attach the right
/// `Gd<Spire…>` handle when the user opts into Godot-signal dispatch.
#[allow(dead_code)]
pub trait TyToPropertyTween {
    type GdTween;
}

pub(crate) trait TryFromPathAndObject {
    fn try_from_path_and_object(_path: &str, _object: Gd<Object>) -> Option<Self>
    where Self: Sized {
        None
    }
}
