use imports::*;

use super::*;

#[allow(unused_imports)]
mod imports {
    pub use std::sync::OnceLock;

    pub use cap::ImplementsGodotExports;
    pub use godot::{
        meta::ClassName,
        obj::{Bounds, UserClass, bounds::DeclUser, cap},
        private::ClassConfig,
    };
}

macro_rules! nested_try_from_path_and_object {
    ($path_var: ident, $obj_var: ident, $($Tys:ty),*) => {{
        $(
            if let Some(data) = <$Tys>::try_from_path_and_object($path_var, $obj_var.clone()) {
                return data.into();
            }
        )*
    }};
}

pub(crate) use nested_try_from_path_and_object;

macro_rules! gd_property_tween {
    ($GdName:ident, $T:ty, $P:ident, $CStrName: literal, SpeedDocs = $SpeedDocs:literal) => {
        //#[derive(GodotClass)]
        //#[class(base = RefCounted, init)]
        pub struct $GdName {
            pub base: ::godot::obj::Base<::godot::classes::RefCounted>,
            pub inner: UnsafeCell<RcPtr<SpireTween<LerpPropertyData<$T>>>>,
        }

        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct $ {concat(__godot_, $GdName, _Funcs)} {}

        ::godot::sys::plugin_add!(
            :: godot :: private :: __GODOT_DOCS_REGISTRY ;
            :: godot :: private :: DocsPlugin :: new :: < $GdName > (
                :: godot :: private :: DocsItem :: Struct (
                    :: godot :: docs :: StructDocs {
                        base : "RefCounted" ,
                        description :
"A tween that interpolates the value of the property at [method get_property_path], by lerping between specific \
start and end values defined by the user. The weight of the lerp is based on how much time has passed \
relative to [method get_duration].\
\n\
[b]Note:[/b] You should not instantiate this class directly, instead use one of the `do_property_*` methods on [Spire].\
\n\
Spire also provides specialized constructors of this class for common properties, the constructors are present \
on special \"namespace\" classes for each node type, examples: [DoNode2D], [DoControl], \
[DoCanvasItem]. See those classes' methods for more details.",
                        experimental : "" ,
                        deprecated : "" ,
                        members : "" ,
                    }
                )
            )
        );
        ::godot::sys::plugin_add!(
            :: godot :: private :: __GODOT_PLUGIN_REGISTRY ;
            :: godot :: private :: ClassPlugin :: new :: < $GdName > (
                :: godot :: private :: PluginItem :: Struct (
                    :: godot :: private :: Struct :: new :: < $GdName > ()
                )
            )
        );

        ::godot::private::class_macros::inherit_from_RefCounted__ensure_class_exists!($GdName );

        impl ::godot::obj::WithBaseField for $GdName {
            fn to_gd(&self) -> ::godot::obj::Gd<$GdName> {
                let base = <$GdName as ::godot::obj::WithBaseField>::base_field(self);
                base.to_gd().cast()
            }

            fn base_field(&self) -> &::godot::obj::Base<<$GdName as ::godot::obj::GodotClass>::Base> {
                &self.base
            }
        }

        impl ::std::ops::Deref for $GdName {
            type Target = ::godot::classes::RefCounted;
            fn deref(&self) -> &Self::Target { &self.base }
        }

        impl ::std::ops::DerefMut for $GdName {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
        }

        impl GodotClass for $GdName {
            type Base = RefCounted;
            fn class_name() -> ClassName {
                static CLASS_NAME: OnceLock<ClassName> = OnceLock::new();
                let name: &'static ClassName =
                    CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii($CStrName));
                *name
            }
        }

        unsafe impl Bounds for $GdName {
            type Memory = <<Self as GodotClass>::Base as Bounds>::Memory;
            type DynMemory = <<Self as GodotClass>::Base as Bounds>::DynMemory;
            type Declarer = DeclUser;
            type Exportable = <<Self as GodotClass>::Base as Bounds>::Exportable;
        }

        impl ImplementsGodotExports for $GdName {
            fn __register_exports() { }
        }

        impl UserClass for $GdName {
            #[doc(hidden)]
            fn __config() -> ClassConfig { ClassConfig { is_tool: false } }
            #[doc(hidden)]
            fn __before_ready(&mut self) {}
        }

        macro_rules! ${concat(__godot_, $GdName, _vis_macro)} {
            ($$( #[$$meta:meta] )* struct $$($$tt:tt)+) => {
                $$( #[$$meta] )* pub struct $$($$tt)+
            };
        }
        macro_rules! ${concat(__godot_, $GdName, _has_base_field_macro)} {
           ($$( $$tt:tt )*) => { $$( $$tt )* };
        }

        macro_rules! ${concat(__deny_manual_init_, $GdName)} {
            () => {
                compile_error!(
                    "Class `$GdName` is marked with #[class(no_init)] but provides an init() method."
                );
            };
        }

        #[godot_api]
        impl $GdName {
            /// Emitted when this tween completes a loop.
            ///
            /// [b]Note:[/b] On the last loop of a tween, this is emitted before [signal finished].
            #[signal]
            pub fn loop_finished();

            /// Emitted when this tween finishes playing.
            /// This happens when the tween completes all its loops, or when [method force_complete] is called.
            ///
            /// [b]Note:[/b] This will never be emitted if the tween is set to loop infinitely
            /// (e.g. calling [method set_loops] with `-1`).
            #[signal]
            pub fn finished();

            /// [b]Returns:[/b] The object that owns the property this tween is animating.
            #[func]
            pub fn get_owner(&self) -> Gd<Object> {
                self.to_ref().get_owner().to_object()
            }

            /// [b]Returns:[/b] The path of the property that this tween is animating, relative to [method get_owner].
            #[func]
            pub fn get_property_path(&self) -> NodePath {
                self.to_ref().get_property_path().clone()
            }

            /// [b]Returns:[/b] The total duration of the tween, in seconds.
            ///
            /// [b]Note:[/b] This is not the remaining time, this is the `duration` parameter
            /// you passed when constructing the tween.
            ///
            /// [b]Note:[/b] For speed-based tweens, the returned value will be calculated by:
            /// `distance / speed`.
            #[func]
            pub fn get_duration(&self) -> f64 {
                self.to_mut().get_duration()
            }

            /// [b]Returns:[/b] The easing used by this tween. Since easing can be represented by multiple
            /// types, the returned value is a [Variant] that can be one of the following:
            ///
            /// - [enum Spire.Ease]: which is assigned through [method set_ease].
            /// - [Curve]: which is assigned through [method set_ease_curve].
            /// - [Callable]: which is assigned through [method set_ease_func].
            ///
            /// See each method's documentation for more details.
            #[func]
            pub fn get_ease(&self) -> Variant {
                self.to_ref().get_ease().to_variant()
            }

            /// Defines the easing type to use for the tween.
            ///
            /// # What is Easing?
            ///
            /// A tween is basically a smooth transition from the start value to the end value
            /// over time.
            ///
            /// On every update, the tween calculates the percentage of completion based on the elapsed time
            /// and the total duration: `percentage = elapsed_time / duration`. Then uses the percentage
            /// to interpolate between the start and end values: `current_value = from + (to - from) * percentage`.
            ///
            /// The formula above produces a linear interpolation, meaning the value changes
            /// at a constant rate over time, for example a percentage of `0.5` would result in the current value
            /// being exactly halfway between the start and end values.
            ///
            /// Easing is a pre-processing step that modifies the "percentage" value before it is passed
            /// to the interpolation formula: `current_value = from + (to - from) * ease(percentage)`.
            ///
            /// An easing function basically takes one float input (the linear percentage) and produces the
            /// modified percentage as output: `func ease(linear_percentage: float) -> float`.
            ///
            /// There are many different easing functions, and Spire provides the most common ones
            /// through [enum Spire.Ease]. Check the enum's documentation for visual examples of each easing type.
            ///
            /// # Custom Easing
            /// If none of the built-in easing types satisfy your use-case, you may also use one of the following:
            /// - [method set_ease_curve] to provide a [Curve] that will behave as the easing function,
            /// Spire will call [method Curve.sample_baked] with the linear percentage as the `offset` parameter.
            /// - [method set_ease_func] to provide a custom [Callable] that will
            /// be invoked as the easing function.
            ///
            /// # Easing and Speed-Based tweens
            ///
            /// Easing is usually applied to the percentage value: `elapsed_time / duration`, however,
            /// speed-based tweens do not have a fixed duration. Instead of not applying easing at all,
            /// Spire applies easing to the tween's speed, based on the distance ratio:
            /// `distance_traveled / total_distance`.
            ///
            /// This may sound confusing, but it will be much more intuitive when you see it in action,
            /// so I recommend trying it out yourself.
            #[func(gd_self)]
            pub fn set_ease(this: Gd<Self>, easing: Ease) -> Gd<Self> {
                this.bind().to_mut().t.ease = EaseKind::Basic(easing);
                this
            }

            /// [b]Behavior:[/b] Sets a [Curve] to be used as the easing function. Spire will use the curve to
            /// calculate the interpolation weight that is passed to the lerping function. The curve is sampled
            /// with [method Curve.sample_baked] where the `offset` argument is the tween's current progress,
            /// represented by a [b]positive[/b] number, usually in the range `0.0 ~ 1.0`, though it can exceed 1.0
            /// in some cases (e.g., [constant Spire.LOOP_MODE_INCREMENTAL]).
            ///
            /// For more details on easing, see [method set_ease].
            #[func(gd_self)]
            pub fn set_ease_curve(this: Gd<Self>, curve: Option<Gd<Curve>>) -> Gd<Self> {
                let Some(curve) = curve else {
                    godot_error!("Parameter `curve`: expected `Curve`, got: `null`.");
                    return this;
                };

                this.bind().to_mut().set_ease(EaseKind::GodotCurve(curve));
                this
            }

            /// [b]Behavior:[/b] Sets a custom [Callable] to be used as the easing function.
            ///
            /// [b]Parameter [param callable]:[/b] Must have the following signature: `func(float) -> float`,
            /// where:
            /// - Input: The tween's current progress, represented by a [b]positive[/b] number,
            /// usually in the range `0.0 ~ 1.0`, though it can exceed 1.0 in some cases
            /// (e.g., [constant Spire.LOOP_MODE_INCREMENTAL]). Spire guarantees it will never be negative.
            /// - Output: The `weight` value, usually in the range `0.0~1.0`, though values
            /// outside these bounds are also valid (including negative values, unlike the input). This weight value
            /// will be passed to the lerping function.
            ///
            /// For more details on easing, see [method set_ease].
            #[func(gd_self)]
            pub fn set_ease_func(this: Gd<Self>, callable: Callable) -> Gd<Self> {
                this.bind().to_mut().set_ease(EaseKind::Callable(callable));
                this
            }

            /// [b]Behavior:[/b] Sets the starting value of the tween.
            ///
            /// [b]Usage:[/b] By default, the starting value will be read from the property the first time the
            /// tween updates, calling this method overrides that behavior and forces the tween to start from the
            /// provided value.
            ///
            /// [b]Note:[/b] The starting value is ignored in Relative and Speed-Based tweens, a warning will be emitted
            /// if you call this method on such tweens.
            #[func(gd_self)]
            pub fn from(this: Gd<Self>, value: $T) -> Gd<Self> {
                this.bind().to_mut().set_begin_value(value);
                this
            }

            /// [b]Returns:[/b] The final value of the tween. This is the value the property should have when the
            /// animation completes (which happens after [method get_duration] seconds).
            ///
            /// [b]Note:[/b] If this tween is in the "Relative" mode, then `final_value` is the relative offset,
            /// not the absolute final value.
            #[func]
            pub fn get_final_value(&self) -> $T {
                self.to_mut().get_final_value()
            }

            /// [b]Usage:[/b] Instead of a fixed final value, use a callable to evaluate the final value on each update.
            /// This method allows you to define "moving" goals.
            ///
            /// [b]Parameter [param evaluator]:[/b] A callable that should return a value of the same type as the
            /// property being animated, and it should not require any arguments.
            ///
            /// # Example
            /// This creates a tween that will make the node `pursuer` follow the node `target` at a speed of
            /// 50 pixels per second:
            /// ```gdscript
            /// # Since we are setting a dynamic target, the `to` argument (here `Vector2(0,0)`) is ignored.
            /// # Note that `target.get_position` is a callable, we are not invoking it here.
            /// DoNode2D.position(pursuer, Vector2(0,0), 50.0) \
            ///     .set_dynamic_target(target.get_position) \
            ///     .as_speed_based()
            /// ```
            ///
            /// On each update, the tween will call `target.get_position()` to get the current position of the
            /// target node, and that will be considered the value to move towards.
            ///
            /// [b]Note:[/b] Although all modes support dynamic targets, they make the most sense when used with the
            /// speed-based mode, it should also work fine in relative mode. It will likely lead to weird
            /// behavior when used in absolute mode (the default one).
            #[func(gd_self)]
            pub fn set_dynamic_target(this: Gd<Self>, evaluator: Callable) -> Gd<Self> {
                this.bind().to_mut().set_dynamic_target(evaluator);
                this
            }

            /// [b]Behavior:[/b] Switches this tween's mode to "SpeedBased", interpolating the property at a fixed
            /// speed (rate). The value of `speed` is the `duration` argument you provided when constructing the tween.
            ///
            /// [b]Usage:[/b] When creating a regular(non-speed-based) tween, you provide a fixed
            /// `duration` and Spire simply uses a lerp function to interpolate the property over time. On the other
            /// hand, when creating speed-based tweens you provide a fixed `speed`, and Spire "bumps"(steps) the
            /// property's current value towards [method get_final_value], the increment on each
            /// update can be calculated by `increment = speed * delta_time`.
            ///
            /// [b]Note:[/b] Since speed-based tweens modify a property through incremental steps, they can blend
            /// with other forces/tweens modifying the same property(in a similar manner to relative-tweens).
            ///
            /// [b]Example:[/b]: Creating a speed-based tween that translates a node at a rate of `100 pixels/second`:
            /// ```gdscript
            /// DoNode2D.global_position(my_node, Vector2(500, 300), 100.0).as_speed_based()
            /// #                                                    ^^^^^
            /// #                                                    speed
            /// ```
            ///
            /// [b]Note:[/b] Speed-based tweens will never overshoot.
            ///
            /// [b]Note:[/b] The unit of measurement for `speed` depends on the type of property being animated.
            #[doc = $SpeedDocs]
            ///
            /// # What is a speed-based tween?
            ///
            /// Regular tweens have a fixed `from`, `to`, and `duration`. In simple terms, the value on
            /// each update is calculated by: `from + (to - from) * progress`.
            ///
            /// On the other hand, Speed-Based tweens constantly "bump"(step) the property's current value towards
            /// [method get_final_value]. The strength of each bump is calculated as: `speed * delta_time`.
            #[func(gd_self)]
            pub fn as_speed_based(this: Gd<Self>) -> Gd<Self> {
                this.bind().to_mut().set_speed_based();
                this
            }

            /// [b]Behavior:[/b] Switches this tween's mode to "Relative", which interpolates an offset to
            /// the property's value instead of forcing the property into a fixed `from->to` curve.
            ///
            /// # What is a relative-tween?
            ///
            /// Unlike a regular(sometimes referred to as "absolute") tween, a relative tween will "add" its
            /// interpolation to the property on each update.
            /// This means that the tween will "blend" with any external factors modifying the property.
            ///
            /// [b]Example:[/b] Creating a relative tween that translates a node by `Vector2(300, 20)` over the course
            /// of `5.0` seconds:
            /// ```gdscript
            /// DoNode2D.position(my_node, Vector2(300, 20), 5.0).as_relative()
            /// ```
            ///
            /// The tween above will "add" a total of `Vector2(300, 20)` to [member Node2D.position]
            /// over the course of 5.0 seconds.
            ///
            /// The initial/final position of the node doesn't matter, the tween will always uphold the contract
            /// of translating the node by `Vector2(300, 20)`.
            ///
            /// Assuming that `my_node`'s initial position is `Vector2(100, 50)`, and nothing else is modifying it,
            /// the node's position over time would be:
            /// - `0.0s` => `Vector2(100, 50)`
            /// - `2.5s` => `Vector2(250, 60)`
            /// - `5.0s` => `Vector2(400, 70)`
            ///
            /// Assuming that the node has a velocity of `Vector2(10, 0)` (moving right at 10 pixels/second),
            /// the node's position over time would be:
            /// - `0.0s` => `Vector2(100, 50)`
            /// - `2.5s` => `Vector2(275, 60)`
            /// - `5.0s` => `Vector2(450, 70)`
            ///
            /// As you can see, the tween "blends" with the node's velocity, acting as an additional force instead of
            /// overriding other forces. This makes relative tweens ideal for situations where more than one factor
            /// is modifying a property.
            ///
            /// [b]Note:[/b] Two or more relative tweens will blend with each-other, you can have as many relative
            /// tweens as you want on the same property, and they will all add their offsets together.
            ///
            /// [b]Note:[/b] If a single tween affecting that property is a "regular"(aka "absolute) tween, that tween
            /// will override all others, relative or not.
            #[func(gd_self)]
            pub fn as_relative(this: Gd<Self>) -> Gd<Self> {
                this.bind().to_mut().set_relative(<$T>::default());
                this
            }

            /// [b]Returns:[/b] `true` if this tween is the mode "Absolute", meaning it's not relative nor speed-based
            /// (Absolute is the default mode for all tweens); false otherwise.
            #[func]
            pub fn is_absolute(&self) -> bool {
                self.to_ref().is_absolute()
            }

            /// [b]Returns:[/b] `true` if this tween is in the mode "Relative"; false otherwise.
            /// See [method as_relative] for more details.
            #[func]
            pub fn is_relative(&self) -> bool {
                self.to_ref().is_relative()
            }

            /// [b]Returns:[/b] `true` if this tween is in the mode "SpeedBased"; false otherwise.
            /// See [method as_speed_based] for more details.
            #[func]
            pub fn is_speed_based(&self) -> bool {
                self.to_ref().is_speed_based()
            }
        }

        define_base_gd_methods! { $GdName => SpireTween<LerpPropertyData<$T>> }
    };
}

/*
gd_property_tween! { SpirePropertyI32, i32, i32, c"SpirePropertyI32", SpeedDocs =
"For [int], `speed` is measured in units per second, where `unit` is whichever unit the integer represents.\n\
Assuming a speed of `5.0`, it would take `2.0` seconds to go from `0` to `10`.\n" }
*/

gd_property_tween! { SpirePropertyInt, i64, i64, c"SpirePropertyInt", SpeedDocs =
"For [int], `speed` is measured in units per second, where `unit` is whichever unit the integer represents.\n\
Assuming a speed of `5.0`, it would take `2.0` seconds to go from `0` to `10`.\n" }

/*
gd_property_tween! { SpirePropertyF32, f32, f32, c"SpirePropertyF32", SpeedDocs =
"For [float], `speed` is measured in units per second, where `unit` is whichever unit the float represents.\n\
Assuming a speed of `7.5`, it would take `4.0` seconds to go from `0.0` to `30.0`.\n" }
*/

gd_property_tween! { SpirePropertyFloat, f64, f64, c"SpirePropertyFloat", SpeedDocs =
"For [float], `speed` is measured in units per second, where `unit` is whichever unit the float represents.\n\
Assuming a speed of `7.5`, it would take `4.0` seconds to go from `0.0` to `30.0`.\n" }

gd_property_tween! { SpirePropertyString, GString, GString, c"SpirePropertyString", SpeedDocs =
"For [String], `speed` is measured in chars per second.\n\
Assuming a speed of `2.0`, it would take `6.0` seconds to go from `\"\"`(no chars) to `\"Hello world!\"`(12 chars).\n" }

gd_property_tween! { SpirePropertyColor, Color, Color, c"SpirePropertyColor", SpeedDocs =
"For [Color], `speed` is measured in 4D-Euclidean-distance per second (A [Color] is simply a 4D Vector(r,g,b,a)).\n\
The distance between two colors `l` and `r` is defined as:  \n\t\
`d = sqrt((l.r - r.r)^2 + (l.g - r.g)^2 + (l.b - r.b)^2 + (l.a - r.a)^2)`\n\
Assuming a speed of `1.0`, it would take `2.0` seconds to go from `Color(0, 0, 0, 0)`(clear) \
to `Color(1, 1, 1, 1)`(white).\n" }

gd_property_tween! { SpirePropertyVector2, Vector2, Vector2, c"SpirePropertyVector2", SpeedDocs =
"For [Vector2], `speed` is measured in 2D-Euclidean-distance per second.\n\
The distance between two points `l` and `r` is defined as:  \n\t\
`d = sqrt((l.x - r.x)^2 + (l.y - r.y)^2)`\n\
Assuming a speed of `64.0`, it would take `1.4142` seconds to go from `Vector2(0, 0)` to `Vector2(64, 64)`.\n" }

gd_property_tween! { SpirePropertyVector2i, Vector2i, Vector2i, c"SpirePropertyVector2i", SpeedDocs =
"For [Vector2i], `speed` is measured in 2D-Euclidean-distance per second.\n\
The distance between two points `l` and `r` is defined as:  \n\t\
`d = sqrt((l.x - r.x)^2 + (l.y - r.y)^2)`\n\
Assuming a speed of `64.0`, it would take `1.4142` seconds to go from `Vector2i(0, 0)` to `Vector2i(64, 64)`.\n" }

gd_property_tween! { SpirePropertyVector3, Vector3, Vector3, c"SpirePropertyVector3", SpeedDocs =
"For [Vector3], `speed` is measured in 3D-Euclidean-distance per second.\n\
The distance `d` between two points `l` and `r` is defined as:  \n\t\
`d = sqrt((l.x - r.x)^2 + (l.y - r.y)^2 + (l.z - r.z)^2)`\n\
Assuming a speed of `64.0`, it would take `1.732` seconds to go from `Vector3(0, 0, 0)` to `Vector3(64, 64, 64)`.\n" }

gd_property_tween! { SpirePropertyVector3i, Vector3i, Vector3i, c"SpirePropertyVector3i", SpeedDocs =
"For [Vector3i], `speed` is measured in 3D-Euclidean-distance per second.\n\
The distance `d` between two points `l` and `r` is defined as:  \n\t\
`d = sqrt((l.x - r.x)^2 + (l.y - r.y)^2 + (l.z - r.z)^2)`\n\
Assuming a speed of `64.0`, it would take `1.732` seconds to go from `Vector3i(0, 0, 0)` to `Vector3i(64, 64, 64)`.\n" }

gd_property_tween! { SpireProperty, Variant, Variant, c"SpireProperty", SpeedDocs =
"Which interpolates the property at a fixed rate.\n" }

macro_rules! define_instantiate_fn {
    ($GdName:ident, $T:ty, $Suffix:literal,Docs = $Docs:literal) => {
        #[godot_api(secondary)]
        impl $GdName {
            #[doc = $Docs]
            #[func]
            pub fn build(owner: Gd<Object>, property_path: NodePath, to: $T, duration: f64) -> Gd<Self> {
                let property_str = &property_path.to_string();

                let data = <$T as PropertyType>::Data::from_path_and_owner(property_str, property_path, owner);

                let inner = UnsafeCell::new(
                    SpireTween::<LerpPropertyData<$T>>::new(data, Evaluator::Static(to), duration).register(),
                );
                let handle = Gd::from_init_fn(|base| Self { base, inner });
                let handle_clone = handle.clone();
                handle.bind().to_mut().gd_handle = Some(handle_clone);
                handle
            }
        }
    };
}

define_instantiate_fn! { SpireProperty, Variant, "Variant", Docs = "See [method Spire.do_property]." }
//define_instantiate_fn! { SpirePropertyI32, i32, "i32", Docs = "See [method Spire.do_property_i32]." }
define_instantiate_fn! { SpirePropertyInt, i64, "i64", Docs = "See [method Spire.do_property_int]." }
//define_instantiate_fn! { SpirePropertyF32, f32, "f32", Docs = "See [method Spire.do_property_f32]." }
define_instantiate_fn! { SpirePropertyFloat, f64, "f64", Docs = "See [method Spire.do_property_float]." }
define_instantiate_fn! { SpirePropertyString, GString, "string", Docs = "See [method Spire.do_property_string]." }
define_instantiate_fn! { SpirePropertyColor, Color, "color", Docs = "See [method Spire.do_property_color]." }
define_instantiate_fn! { SpirePropertyVector2, Vector2, "vec2", Docs = "See [method Spire.do_property_vector2]." }
define_instantiate_fn! { SpirePropertyVector2i, Vector2i, "vec2i", Docs = "See [method Spire.do_property_vector2i]." }
define_instantiate_fn! { SpirePropertyVector3, Vector3, "vec3", Docs = "See [method Spire.do_property_vector3]." }
define_instantiate_fn! { SpirePropertyVector3i, Vector3i, "vec3i", Docs = "See [method Spire.do_property_vector3i]." }

#[godot_api(secondary)]
impl SpireProperty {
    /// See [method Spire.do_property].
    #[func]
    pub fn build_custom(
        owner: Gd<Object>,
        property_path: NodePath,
        to: Variant,
        duration: f64,
        lerp_func: Callable,
        relative_func: Callable,
        step_func: Callable,
        distance_func: Callable,
    ) -> Gd<Self> {
        let lerper = CustomLerper::new(lerp_func, relative_func, step_func, distance_func);

        let inner = UnsafeCell::new(
            SpireTween::<LerpPropertyData<Variant>>::new_custom(
                &property_path,
                ObjectOrNode::from_unchecked_object(owner),
                Evaluator::Static(to),
                duration,
                lerper,
            )
            .register(),
        );

        let handle = Gd::from_init_fn(|base| Self { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
}
