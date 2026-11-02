use super::*;

mod color;
mod float;
mod int;
mod lerp_mode;
mod quaternion;
mod rect2;
mod string;
mod text;
mod variant;
mod vector2;
mod vector2i;
mod vector3;
mod vector3i;
mod vector4;

#[allow(unused_imports)]
pub use self::{
    lerp_mode::*,
    text::{StringLerper, TextOptions},
    variant::*,
};

/// The minimum lerping operation: take `from`, `to`, and a weight, return the
/// interpolated value.
///
/// Built-in numeric and vector types use the unit `()` as their lerper — see the
/// many `impl BasicLerp<…> for ()` blocks in this module. Implement `BasicLerp` on a
/// new type when you want to plug a custom data type into [`LerpMethodData`] (the
/// minimum requirement for a method tween).
///
/// # Contract
///
/// `spire_lerp(from, to, weight)` should return:
/// - `from` when `weight == 0.0`
/// - `to` when `weight == 1.0`
/// - the value halfway between when `weight == 0.5`
/// - extrapolated values when `weight` falls outside `0.0 ~ 1.0` (this can happen
///   under [`LoopMode::Incremental`] or with custom easing functions)
///
/// For numbers and vectors the canonical formula is `from + (to - from) * weight`.
/// Quaternions satisfy the endpoint contract by orientation: `q` and `-q` are
/// equivalent even though their components differ.
/// For non-arithmetic types (strings, custom enums, …) the formula is up to you, as
/// long as the contract above holds.
pub trait BasicLerp<T> {
    fn spire_lerp(&mut self, from: &T, to: &T, t: f64) -> T;
}

/// Extends [`BasicLerp`] with relative and speed-based operations, plus endpoint
/// preparation for property completion.
///
/// You only need to implement `SpireLerp` (instead of just [`BasicLerp`]) when:
/// - the type is going to be plugged into [`LerpPropertyData`] (the property tween),
///   and
/// - users may want to use that type in relative or speed-based mode.
///
/// For [`LerpMethodData`] (method tweens), only `BasicLerp` is required.
///
/// The interpolation operations Spire calls per mode:
///
/// | Mode             | Calls                                     |
/// |------------------|-------------------------------------------|
/// | [`LerpMode::Absolute`]    | `spire_lerp` only                |
/// | [`LerpMode::Relative`]    | `spire_lerp` + `add_relative`    |
/// | [`LerpMode::SpeedBased`]  | `spire_step` + `spire_distance`  |
///
/// Completion paths that write a value without interpolating it use
/// [`prepare_completion_value`](Self::prepare_completion_value).
///
/// See each method's documentation for the contract.
pub trait SpireLerp<T>: BasicLerp<T> {
    /// Selects the active property mode before processing or forced completion.
    /// Stateful lerpers can adapt options to the mode. For example, string
    /// scrambling is ignored in relative and speed-based modes.
    fn prepare_mode(&mut self, _mode: &LerpMode<T>) {}

    /// Prepares a property value for a completion path that may bypass
    /// interpolation. Called before the write on forced completion (all modes)
    /// and when speed-based endpoints already coincide.
    ///
    /// The default preserves the value. Quaternion normalizes it to maintain
    /// the same unit-rotation contract as its interpolation. Ordinary updates
    /// and property-data setters do not call this hook.
    fn prepare_completion_value(&mut self, value: T) -> T { value }

    /// **Used in [`LerpMode::Relative`].**
    ///
    /// A relative tween needs to add an *increment* to the property's current value
    /// each tick — without overwriting other forces affecting the property. The
    /// challenge: how do you do that generically for any `T` (including types like
    /// `GString` where you can't multiply by a `f64`)?
    ///
    /// Spire solves this by computing two interpolation samples per tick:
    /// - `previous_relative = lerp(from, to, previous_weight)`
    /// - `new_relative      = lerp(from, to, current_weight)`
    ///
    /// Then `next_value = current_at_obj + (new_relative - previous_relative)`. The
    /// difference `(new - previous)` is the increment for this tick; adding it to
    /// `current_at_obj` lets the tween blend with whatever else has moved the
    /// property since last frame.
    ///
    /// For numeric / vector types the implementation is literally
    /// `current_at_obj + new_relative - previous_relative`. For exotic types you
    /// need an analogous "add the delta" operation.
    ///
    /// # Train/passenger analogy
    ///
    /// Imagine a train whose speed varies. A passenger inside walks toward the
    /// front at a constant 2 m/s *relative to the train*. Each frame the
    /// passenger's world position changes by both the train's movement *and* their
    /// own. We don't want to model the train (other forces); we just want to add
    /// our own 2 m/s × delta_time to the passenger's current world position.
    /// `add_relative` is "add my own movement to wherever they are now".
    fn add_relative(&mut self, current_at_obj: &T, previous_relative: &T, new_relative: &T) -> T;

    /// **Used in [`LerpMode::SpeedBased`].**
    ///
    /// Step `from` toward `to` by at most `speed * step` units of progress. Returns
    /// the new value plus a [`StepResult`] indicating whether the target was reached.
    ///
    /// Naive implementation `current += speed * step` fails for integer types: when
    /// `speed * step < 0.5`, the increment rounds to `0` and the value never
    /// changes — or rounds to `1` and overshoots. Spire models `speed * step` as
    /// "fuel" that callers can save up between ticks via [`StepResult::Unfinished::accumulated_time`]
    /// and spend later when there's enough for a whole-unit move.
    ///
    /// **Contract:**
    /// - Never overshoot — return `to` (or as close as your unit allows) and stop.
    /// - When the result equals `to`, return [`StepResult::Finished`] with the unspent
    ///   fuel as `excess_time`. Spire feeds that excess into the parent sequence so
    ///   it doesn't drift.
    /// - Otherwise return [`StepResult::Unfinished`] with the saved-up fuel as
    ///   `accumulated_time` (so the next call can spend it).
    /// - Spire trusts you on `is_finished`: it does not double-check by comparing
    ///   `value == to`.
    fn spire_step(&mut self, from: &T, to: &T, speed: f64, step: f64) -> (T, StepResult);

    /// **Used in [`LerpMode::SpeedBased`].**
    ///
    /// "How far apart are these two values?" — used both to compute speed-based
    /// progress (for easing) and to know when the tween has reached its target.
    ///
    /// For numbers: `(to - from).abs()`. For vectors: Euclidean distance.
    /// `Vector4` and `Color`: 4D Euclidean distance. `Rect2`: 4D Euclidean distance
    /// across position and size. `Quaternion`: shortest rotation angle in radians.
    /// For `GString`: the
    /// number of characters that differ (Spire's built-in impl).
    fn spire_distance(&mut self, from: &T, to: &T) -> f64;
}

/// Outcome of one [`SpireLerp::spire_step`] call. Carries leftover "fuel" so the
/// stepper can integer-quantize without losing time across ticks.
#[derive(Debug, Clone, Copy)]
pub enum StepResult {
    /// The target wasn't reached on this tick. `accumulated_time` is the unspent
    /// fuel that should be saved and added to the next tick's `step` argument.
    Unfinished { accumulated_time: f64 },
    /// The target was reached on this tick. `excess_time` is the overshoot that
    /// Spire forwards to the parent sequence (for clean handoff to the next block)
    /// or to the next loop.
    Finished { excess_time: f64 },
}

pub trait MoveTowards {
    fn move_towards(self, to: Self, abs_move: Self) -> Self;
}

/// Marker trait: "this type can be tweened, and here are the lerpers Spire should
/// use for it."
///
/// Spire ships `impl ILerpable for …` for: `i64`, `f64`, `GString`, `Color`,
/// `Vector2`, `Vector2i`, `Vector3`, `Vector3i`, `Vector4`, `Quaternion`, `Rect2`,
/// and `Variant`. Built-in lerpers default to `()`, except `GString` property
/// tweens use [`StringLerper`] to support optional BBCode handling and scrambling.
/// String method tweens use the plain `()` lerper.
///
/// `Vector4` interpolates its four components; `Rect2` interpolates position and
/// size together. Their speed-based distance is Euclidean across four scalars.
/// `Quaternion` uses normalized shortest-arc spherical interpolation, including
/// extrapolation. Its relative mode postmultiplies `previous.inverse() * next`,
/// and its speed is radians per second. Finite nonzero quaternion inputs are
/// normalized; zero/nonfinite inputs warn with the rejected value and use identity.
///
/// To tween a *new* type, implement `ILerpable` for it with appropriate lerpers:
///
/// ```ignore
/// impl ILerpable for MyType {
///     type BasicLerper = MyLerper; // implements BasicLerp<MyType>
///     type Lerper = MyLerper;      // implements SpireLerp<MyType>
/// }
/// ```
///
/// `BasicLerper` is the minimum (used by [`LerpMethodData`]); `Lerper` adds the
/// extra ops needed for relative & speed-based property tweens (used by
/// [`LerpPropertyData`]). They can be the same type, or two different types.
///
/// If you don't want to write a lerper at all but only need a one-off custom
/// property/method tween, you can skip implementing `ILerpable` and reach for
/// [`SpireTween::<LerpPropertyData<Variant>>::new_custom`] /
/// [`SpireTween::<LerpMethodData<Variant>>::new_custom`] with a [`CustomLerper`] /
/// [`CustomBasicLerper`].
pub trait ILerpable: Sized {
    /// Lerper used by [`LerpMethodData`] (method tweens). The minimum requirement.
    type BasicLerper: BasicLerp<Self>;
    /// Lerper used by [`LerpPropertyData`] (property tweens). Adds the operations
    /// needed for [`LerpMode::Relative`] and [`LerpMode::SpeedBased`] modes.
    type Lerper: SpireLerp<Self>;
}

// Shared by vectors, rect2, and quaternion.
//
// A continuous value needs no saved quantization time. Compute the fraction of
// the remaining distance and return only time actually left after arrival.
fn step_ratio(distance: f64, speed: f64, step: f64) -> (f64, StepResult) {
    if distance == 0.0 {
        return (1.0, StepResult::Finished { excess_time: step });
    }
    if speed <= 0.0 || step <= 0.0 {
        return (0.0, StepResult::Unfinished { accumulated_time: 0.0 });
    }
    if speed * step >= distance {
        (1.0, StepResult::Finished {
            excess_time: (step - distance / speed).max(0.0),
        })
    } else {
        (speed * step / distance, StepResult::Unfinished { accumulated_time: 0.0 })
    }
}
