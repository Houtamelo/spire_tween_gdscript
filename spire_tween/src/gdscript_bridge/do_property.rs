use super::*;

macro_rules! define_manual_tween_builders {
    (
        $($FnName:ident, $TweenTy:ident, $GdName: literal;)*
    ) => {
        #[godot_api(secondary)]
        impl Spire {
            $(
                /// [b]Behavior:[/b] Once every update, sets the property at [param property_path] on the object [param owner],
                /// with a value that is calculated by lerping the property's initial value towards [param to]
                /// based on how much time has elapsed relative to [param duration].
                ///
                /// [b]Usage:[/b] This function provides a generic way to tween any property of a
                /// specific built-in type, use this when the property you want to animate isn't covered
                /// by any of Spire's `Do[ClassName].[property_name]` functions.
                ///
                /// [b]Returns:[/b] A handle that can be used to further customize the tween.
                ///
                /// [b]Note:[/b] If the property at [param property_path] is a "known"/"common" engine property
                /// (such as [member Node2D.position]), then Spire will automatically optimize the tween to directly
                /// use that property's getter/setter; otherwise it will fall back to [method Object.get_indexed]/
                /// [method Object.set_indexed] (the same methods that the built-in [Tween] uses).
                #[func]
                pub fn $FnName(
                    owner: Option<Gd<Object>>,
                    property_path: NodePath,
                    to: $TweenTy,
                    duration: f64,
                ) -> Option<Gd<<$TweenTy as TyToPropertyTween>::GdTween>> {
                    let owner = owner.log_null_arg(|| "owner")?;
                    Some(<<$TweenTy as TyToPropertyTween>::GdTween>::build(owner, property_path, to, duration))
                }
            )*
        }
    };
}

define_manual_tween_builders! {
    do_property, Variant, "Variant";
    //do_property_f32, f32, "f32(float)";
    do_property_float, f64, "float";
    //do_property_i32, i32, "i32(int)";
    do_property_int, i64, "int";
    do_property_vec2, Vector2, "Vector2";
    do_property_vec2i, Vector2i, "Vector2i";
    do_property_vec3, Vector3, "Vector3";
    do_property_vec3i, Vector3i, "Vector3i";
    do_property_color, Color, "Color";
    do_property_string, GString, "String";
}

#[godot_api(secondary)]
impl Spire {
    /// [b]Behavior:[/b] Once every update, sets the property at [param property_path] on the object [param owner],
    /// with a value that is calculated by lerping the property's initial value towards [param to]
    /// based on how much time has elapsed relative to [param duration].
    ///
    /// [b]Usage:[/b] This function provides a generic way to tween properties of a custom type,
    /// use this when the property you want to animate isn't covered by any of Spire's
    /// `Do[ClassName].[property_name]` functions, and it isn't one of the types natively supported by Spire
    /// (int, float, Vector2, Vector2i, Vector3, Vector3i, Color, String).
    ///
    /// [b]Returns:[/b] A handle that can be used to further customize the tween.
    ///
    /// [b]Lerp Modes - requirements:[/b]
    /// - Absolute: [param lerp_func].
    /// - Speed-based: [param step_func], [param distance_func].
    /// - Relative: [param lerp_func], [param add_relative_func].
    ///
    /// [b]Note:[/b] If the mode you're using doesn't require one of the callables you can just pass
    /// an invalid argument by writing `Callable()`.
    ///
    /// [b]Note:[/b] Unlike the typed variants of this function (such as [method do_property_vec2]),
    /// Spire will never attempt to optimize the tween for you,
    /// no assumptions are made about the type or property being tweened.
    ///
    /// [center]________________________________[/center]
    ///
    /// [b]Parameter [param distance_func]:[/b] A callable with the signature `func(from: T, to: T) -> float`.
    ///
    /// [i]This function is used in the `speed-based` mode.[/i]
    ///
    /// This callable is used to determine how "far apart" two values of type `T` are, the value it returns is used
    /// in internal calculations in variations of the formula `Speed = Distance / Time`.
    ///
    /// For numbers, it is the absolute difference between the two values: `abs(a, b)`.
    /// For vectors, it is calculated (literally) with `Vector::distance_to(a, b)`.
    /// A color is just a Vector4, so the same logic applies.
    /// For strings, Spire's implementation basically computes how many characters differ between the two strings,
    /// there are nuances though.
    ///
    /// [center]________________________________[/center]
    ///
    /// [b]Parameter [param lerp_func]:[/b] A callable with the signature `func(from: T, to: T, weight: float) -> T`.
    ///
    /// [i]This function is used in the `absolute` and `relative` modes.[/i]
    ///
    /// This callable is the standard linear interpolation function, it takes a `from`, `to`, and a `weight`, then returns
    /// the result of interpolating `from` towards `to` by `weight`.
    ///
    /// - If weight is `0.0`, it should return `from`.
    /// - If weight is `1.0`, it should return `to`.
    /// - If weight is `0.5`, it should return the value halfway between.
    /// - The function should be able to handle weights outside the 0..1 range, a weight of 2.0 should return
    /// from + (to - from) * 2.0, for example.
    ///
    /// For numbers/vectors, the formula is simply `from + (to - from) * weight`, but it gets complicated when dealing
    /// with non-numerical types(like strings). This is why Spire requests an explicit implementation from you.
    ///
    /// [center]________________________________[/center]
    ///
    /// [b]Parameter [param add_relative_func]:[/b] A callable with the signature
    /// `func(current: T, previous: T, next: T) -> T`.
    ///
    /// [i]This function is only used in the `relative` mode.[/i]
    ///
    /// This callable is a lot more complex, so I'll try to explain with an analogy, consider the following scenario:
    /// - A train is moving along a track, which has many turns and irregularities - the train's speed is not constant.
    /// - Inside the train there's a passenger that's walking towards the front of the train.
    /// - We want to update the passenger's position every frame, such that they are moving at a constant speed
    /// of 2 meters per second relative to the train, for a total of 10 seconds.
    /// - Since the passenger is inside the train, their position relative to the train isn't affected
    /// by the train's movement. However, their absolute position in the world is affected by both their movement
    /// and the train's movement.
    /// - We don't have to handle the train's movement, but we do have to account for it when calculating the
    /// passenger's global position.
    ///
    /// In that scenario, a simple `lerp` function won't work because the train's movement is constantly changing
    /// the passenger's global position. One might think that using the `speed_based` mode would solve the problem,
    /// but the issue is that both the `from` and `to` positions are both constantly changing due to the train's movement.
    ///
    /// If we consider that the game's running at 60 fps, then we can solve this by "bumping" the passenger's position
    /// by `2.0/60` meters every frame: `passenger.global_position += 2.0/60`. We just need to do that every frame
    /// for the total duration of 10 seconds.
    ///
    /// This works because we can split the movement into tiny increments that are independent of the train's movement.
    /// Since position is a vector, we have ways to add those increments to the current position.
    ///
    /// The generalized version of the formula above is: `current = current + distance * step / duration`.
    /// Where `distance = to - from` (`step` is basically `delta_time * tween.speed_scale`, but it's more complex  
    /// than that since it also depends on the easing algorithm).
    ///
    /// Then: `current = current + (to - from) * step / duration`.
    ///
    /// But how do we handle this in a generic way, for any type `T`? What if `T` is a string?
    /// How do you multiply a string by a float?
    ///
    /// The formula above requires addition, subtraction, and multiplication by a float.
    /// That is simple for numbers and vectors, but not for strings, and we wouldn't know what to do with other types.
    ///
    /// Spire solves this by looking at things from a different perspective:
    /// We already have the lerp function, which means we can calculate any point between `from` and `to`, like such:
    ///
    /// `previous_lerp = lerp(from, to, previous_weight)`
    /// `next_lerp = lerp(from, to, previous_weight + step)`
    ///
    /// Then, for any given frame, the increment we want to apply to `current` is:
    ///
    /// `increment = next_lerp - previous_lerp`
    ///
    /// The formula above basically says: "The increment is the difference between the interpolated value at
    /// the current weight and the value at the previous weight, where `previous_weight` is the weight
    /// used in the previous frame."
    ///
    /// By knowing the increment, we can simply do `current += increment`.
    ///
    /// So, where does the `add_relative_func` come into play?
    ///
    /// To tween the property, Spire needs the new value the `current` property should have,
    /// and it already is capable of calculating the following values:
    /// - `current` : Read directly from the property.
    /// - `previous_lerp` : Calculated using `lerp_func(from, to, previous_weight)`
    /// - `next_lerp` : Calculated using `lerp_func(from, to, previous_weight + step)`
    ///
    /// But it doesn't know how to subtract or add those values, and that's what the `add_relative_func`
    /// is responsible for.
    ///
    /// The signature is:
    /// `func(current: T, previous_lerp: T, next_lerp: T) -> T`
    ///
    /// We give you those 3 values and ask that you return the new value of the property.
    ///
    /// The implementation for numbers and vectors is simple, here's the one for `float`:
    /// ```gdscript
    /// func add_relative(current: float, previous_lerp: float, next_lerp: float) -> float:
    ///     return current + next_lerp - previous_lerp
    /// ```
    ///
    /// How that should work on your own types is up to you, but hopefully the explanation above
    /// makes it clear on what the function is supposed to do.
    ///
    /// [center]________________________________[/center]
    ///
    /// [b]Parameter [param step_func]:[/b] A callable with the signature
    /// `func(from: T, to: T, speed: float, step: float) -> Dictionary`.
    ///
    /// [i]This function is only used in the `speed_based` mode.[/i]
    ///
    /// This callable is responsible for stepping `from` towards `to` based on a given `speed` and `step`.
    ///
    /// One might think this is as simple as `current += speed * step`, but what if `current` is an integer?  
    /// `speed * step` will almost always be a tiny number less than 1, which means it will round down to 0
    /// when converted to an integer, resulting in no movement at all. If it's bigger than 0.5, then it would round
    /// up to 1, in the worst case this would make movement be twice as fast as it should have been.
    ///
    /// Spire solves this by thinking of `time` as some sort of "fuel", allowing the `step_func` to "save"/"store"
    /// unspent fuel as long as it needs to. So a function that wants to step integers can save fuel(time)
    /// until it has enough to make a whole integer move, then it can spend that fuel to make it happen.
    ///
    /// The signature is:
    /// `func(from: T, to: T, speed: float, fuel: float) -> Dictionary`
    ///
    /// Where `fuel` is the total amount of "fuel" available to spend.
    ///
    /// **We use a dictionary as a return value because GDScript doesn't support returning multiple values.**
    ///
    /// The dictionary must contain the following:
    /// - "value": T -> The new value after stepping.
    /// - "is_finished": bool -> Whether we've reached the `to` value.
    /// - "fuel": float -> The amount of "fuel" left after stepping.
    ///
    /// If we did not reach `to`, then "fuel" key contains the amount of fuel that should be saved for the next step.
    /// Otherwise, the "fuel" key should be the amount of unspent fuel.
    ///
    /// [b]Note:[/b] The function should never overshoot, so it is expected to have unspent fuel when finishing.
    ///
    /// [b]Note:[/b] Spire does not check if `value == to` when `is_finished == true`, that is your responsibility.
    ///
    /// Here's the implementation for integers (translated to GDScript):
    /// ```gdscript
    ///  func step(from: int, to: int, speed: float, fuel: float) -> Dictionary {
    ///     var result: Dictionary = {}
    ///
    ///     var remaining_distance: int = abs(to - from)
    ///     var max_step: float = speed * fuel
    ///     var abs_step: int = min(remaining_distance, floor(max_step))
    ///
    ///     result["value"] = move_towards(from, to, abs_step)
    ///     result["fuel"] = (max_step - abs_step) / speed
    ///     result["is_finished"] = max_step >= remaining_distance
    ///     
    ///     return result
    /// ```
    #[func]
    pub fn do_property_custom(
        owner: Option<Gd<Object>>,
        property_path: NodePath,
        to: Variant,
        duration: f64,
        distance_func: Callable,
        lerp_func: Callable,
        add_relative_func: Callable,
        step_func: Callable,
    ) -> Option<Gd<<Variant as TyToPropertyTween>::GdTween>> {
        let owner = owner.log_null_arg(|| "owner")?;

        Some(<<Variant as TyToPropertyTween>::GdTween>::build_custom(
            owner,
            property_path,
            to,
            duration,
            lerp_func,
            add_relative_func,
            step_func,
            distance_func,
        ))
    }
}
