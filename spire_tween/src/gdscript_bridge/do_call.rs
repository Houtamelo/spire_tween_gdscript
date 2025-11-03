use super::*;

macro_rules! define_manual_tween_builders {
    (
        $($FnName:ident, $TweenTy:ident, $GdName: literal;)*
    ) => {
        #[godot_api(secondary)]
        impl Spire {
            $(
                /// [b]Behavior:[/b] Once every update, invokes [param call], passing a single
                /// argument being the value computed by lerping [param from] towards [param to],
                /// based on how much time has passed relative to [param duration].
                ///
                /// [b]Returns:[/b] A handle that can be used to further customize the tween.
                ///
                /// [b]Note:[/b] The Callable must accept a single argument that should have the same
                /// type as [param from] and [param to]. If it requires more arguments,
                /// then either they should have default values, or you can use [method Callable.bind]
                /// to bind fixed arguments to the Callable.
                #[func]
                pub fn $FnName(
                    callable: Callable,
                    from: $TweenTy,
                    to: $TweenTy,
                    duration: f64,
                ) -> Gd<<$TweenTy as TyToMethodTween>::GdTween> {
                    <<$TweenTy as TyToMethodTween>::GdTween>::build(callable, from, to, duration)
                }
            )*
        }
    };
}

define_manual_tween_builders! {
    do_call, Variant, "Variant";
    //do_call_f32, f32, "f32(float)";
    do_call_float, f64, "float";
    //do_call_i32, i32, "i32(int)";
    do_call_int, i64, "int";
    do_call_vector2, Vector2, "Vector2";
    do_call_vector2i, Vector2i, "Vector2i";
    do_call_vector3, Vector3, "Vector3";
    do_call_vector3i, Vector3i, "Vector3i";
    do_call_color, Color, "Color";
    do_call_string, GString, "String";
}

#[godot_api(secondary)]
impl Spire {
    /// [b]Behavior:[/b] Once every update, invokes [param call], passing a single argument
    /// being the value computed by invoking [param lerp_func] with the arguments
    /// ([param from], [param to], and `weight`), where `weight` is a float
    /// that represents the progress of the tween towards completion (usually computed as: `elapsed_time/duration`).
    ///
    /// [b]Usage:[/b] This is an untyped way of creating a method tweener, if the type being tweened is one of
    /// [int, float, Vector2, Vector2i, Vector3, Vector3i, Color, String], then consider using the typed
    /// variants instead ([method do_call_int], [method do_call_float], etc.), they are more efficient and
    /// don't require specifying a `lerp` function.
    ///
    /// [b]Returns:[/b] A handle that can be used to further customize the tween.
    ///
    /// [b]Parameter [param lerp_func]:[/b] A Callable that takes three arguments: (`from`, `to`, `weight`),
    /// and returns the result of interpolating between `from` and `to` by `weight`. The return value will be
    /// used when invoking [param call].
    ///
    /// [b]Note:[/b] [param call] must accept a single argument. If it requires more arguments,
    /// then either they should have default values, or you can use [method Callable.bind] to bind fixed
    /// arguments to the Callable.
    ///
    /// [b]Note:[/b] Usually, the weight argument passed to [param lerp_func] will be in the range
    /// `0.0 ~ 1.0`, but that is not guaranteed, the Callable must be able to handle arguments outside that range.
    #[func]
    pub fn do_call_custom(
        callable: Callable,
        from: Variant,
        to: Variant,
        duration: f64,
        lerp_func: Callable,
    ) -> Gd<<Variant as TyToMethodTween>::GdTween> {
        <<Variant as TyToMethodTween>::GdTween>::build_custom(callable, from, to, duration, lerp_func)
    }
}
