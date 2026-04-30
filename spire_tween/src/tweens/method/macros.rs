use super::*;

#[allow(dead_code)]
pub trait TyToMethodTween {
    type GdTween;
}

macro_rules! gd_method_tween {
    ($GdName:ident, $T:ty, $P:ident, $CStrName:literal) => {
        impl TyToMethodTween for $T {
            type GdTween = $GdName;
        }

        /// A tween that invokes a [Callable] on every update, passing an argument that is calculated by
        /// lerping between [method get_start_value] and [method get_final_value] based on how much time has passed,
        /// relative to [method get_duration].
        ///
        /// [b]Note:[/b] You should not instantiate this class directly,
        /// instead use one of the `do_call_*` methods on [Spire].
        #[derive(GodotClass)]
        #[class(base = RefCounted, no_init)]
        pub struct $GdName {
            pub base:  ::godot::obj::Base<::godot::classes::RefCounted>,
            pub inner: UnsafeCell<RcPtr<SpireTween<LerpMethodData<$T>>>>,
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

            /// Returns the [Callable] that will be invoked on every update with
            /// the interpolated value (between [method get_start_value] and [method get_final_value]).
            #[func]
            pub fn get_callable(&self) -> Callable { self.to_ref().get_callable().clone() }

            /// Returns the total duration of the tween, in seconds.
            ///
            /// [b]Note:[/b] This is not the remaining time, this is the `duration` parameter
            /// you passed when constructing the tween.
            #[func]
            pub fn get_duration(&self) -> f64 { self.to_ref().get_duration() }

            /// Returns the starting value of the tween.
            /// This is the `from` parameter you passed when constructing the tween.
            /// It is the value the callable will be invoked with at the start of the tween (time `0.0`).
            #[func]
            pub fn get_start_value(&self) -> $T { self.to_ref().get_start_value() }

            /// Returns the final value of the tween.
            /// This is the `to` parameter you passed when constructing the tween.
            /// It is the value the last value the callable will be invoked with, which will happen after
            /// [method get_duration] seconds..
            #[func]
            pub fn get_final_value(&self) -> $T { self.to_ref().get_final_value() }

            /// [b]Returns:[/b] The easing used by this tween. Since easing can be represented by multiple
            /// types, the returned value is a [Variant] that can be one of the following:
            ///
            /// - [enum Spire.Ease]: which is assigned through [method set_ease].
            /// - [Curve]: which is assigned through [method set_ease_curve].
            /// - [Callable]: which is assigned through [method set_ease_func].
            ///
            /// See each method's documentation for more details.
            #[func]
            pub fn get_ease(&self) -> Variant { self.to_ref().get_ease().to_variant() }

            /// Defines the easing type to use for the tween.
            ///
            /// # What is Easing?
            ///
            /// A tween is basically a smooth transition from the start value ([method get_start_value])
            /// to the end value ([method get_final_value]) over a specified duration ([method get_duration]).
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
            /// through the [enum Spire.Ease]. Check the enum's documentation for visual examples of each easing type.
            ///
            /// # Custom Easing
            /// If none of the built-in easing types satisfy your use-case, you may also use one of the following:
            /// - [method set_ease_curve] to provide a [Curve] that will behave as the easing function,
            /// Spire will call [method Curve.sample_baked] with the linear percentage as the `offset` parameter.
            /// - [method set_ease_func] to provide a custom [Callable] that will
            /// be invoked as the easing function.
            #[func(gd_self)]
            pub fn set_ease(this: Gd<Self>, easing: Ease) -> Gd<Self> {
                this.bind().to_mut().set_ease(EaseKind::Basic(easing));
                this
            }

            /// Sets a [Curve] to be used as the easing function.
            ///
            /// Spire will use the curve to calculate the interpolation weight that is passed the lerping function.
            /// The curve is sampled using [method Curve.sample_baked] with `elapsed_time/duration`
            /// as the `offset` parameter.
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

            /// Sets a custom [Callable] to be used as the easing function.
            ///
            /// The callable must have the following signature: `func(float) -> float`, where the input
            /// is the completion percentage of the tween (`elapsed_time/duration`), and the output is
            /// the weight value (usually in the range `[0.0, 1.0]`) that will be passed to the lerping function.
            ///
            /// For more details on easing, see [method set_ease].
            #[func(gd_self)]
            pub fn set_ease_func(this: Gd<Self>, callable: Callable) -> Gd<Self> {
                this.bind().to_mut().set_ease(EaseKind::Callable(callable));
                this
            }
        }

        define_base_gd_methods! { $GdName, SpireTween<LerpMethodData<$T>>, LerpMethodData<$T> }
    };
}

//gd_method_tween! { SpireMethodI32, i32, i32, c"SpireMethodI32" }
gd_method_tween! { SpireMethodInt, i64, i64, c"SpireMethodInt" }
//gd_method_tween! { SpireMethodF32, f32, f32, c"SpireMethodF32" }
gd_method_tween! { SpireMethodFloat, f64, f64, c"SpireMethodFloat" }
gd_method_tween! { SpireMethodString, GString, GString, c"SpireMethodString" }
gd_method_tween! { SpireMethodColor, Color, Color, c"SpireMethodColor" }
gd_method_tween! { SpireMethodVector2, Vector2, Vector2, c"SpireMethodVector2" }
gd_method_tween! { SpireMethodVector2i, Vector2i, Vector2i, c"SpireMethodVector2i" }
gd_method_tween! { SpireMethodVector3, Vector3, Vector3, c"SpireMethodVector3" }
gd_method_tween! { SpireMethodVector3i, Vector3i, Vector3i, c"SpireMethodVector3i" }
gd_method_tween! { SpireMethod, Variant, Variant, c"SpireMethod" }

macro_rules! define_instantiate_fn {
    ($GdName:ident, $T:ty,Docs = $Docs:literal) => {
        #[godot_api(secondary)]
        impl $GdName {
            #[doc = $Docs]
            #[func]
            pub fn build(callable: Callable, from: $T, to: $T, duration: f64) -> Gd<Self> {
                let tween = SpireTween::<LerpMethodData<$T>>::new(callable, from, to, duration).register();
                gd_from_native_tween(tween)
            }
        }
    };
}

define_instantiate_fn! { SpireMethod, Variant, Docs = "See [method Spire.do_call]." }
//define_instantiate_fn! { SpireMethodI32, i32, Docs = "See [method Spire.do_call_i32]." }
define_instantiate_fn! { SpireMethodInt, i64, Docs = "See [method Spire.do_call_int]." }
//define_instantiate_fn! { SpireMethodF32, f32, Docs = "See [method Spire.do_call_f32]." }
define_instantiate_fn! { SpireMethodFloat, f64, Docs = "See [method Spire.do_call_float]." }
define_instantiate_fn! { SpireMethodString, GString, Docs = "See [method Spire.do_call_string]." }
define_instantiate_fn! { SpireMethodColor, Color, Docs = "See [method Spire.do_call_color]." }
define_instantiate_fn! { SpireMethodVector2, Vector2, Docs = "See [method Spire.do_call_vector2]." }
define_instantiate_fn! { SpireMethodVector2i, Vector2i, Docs = "See [method Spire.do_call_vector2i]." }
define_instantiate_fn! { SpireMethodVector3, Vector3, Docs = "See [method Spire.do_call_vector3]." }
define_instantiate_fn! { SpireMethodVector3i, Vector3i, Docs = "See [method Spire.do_call_vector3i]." }

#[godot_api(secondary)]
impl SpireMethod {
    /// See [method Spire.do_call_custom].
    #[func]
    pub fn build_custom(callable: Callable, from: Variant, to: Variant, duration: f64, lerper: Callable) -> Gd<Self> {
        let tween = SpireTween::<LerpMethodData<Variant>>::new_custom(callable, from, to, duration, lerper).register();
        gd_from_native_tween(tween)
    }
}
