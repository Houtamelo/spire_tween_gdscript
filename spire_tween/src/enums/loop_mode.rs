use super::*;

register_enum! {
    [GD = "LoopMode"]
    LoopMode {
        /// The default mode, tweening will restart from the beginning each time it loops.
        ///
        /// # Example
        ///
        /// The tween:
        /// ```gdscript
        /// var origin := Vector2(0, 0)
        /// var target := Vector2(100, 0)
        /// var duration := 3.0
        /// var tween := DoNode2d(node, target, duration) \
        ///     .from(origin) \
        ///     .set_loops(3, Spire.LOOP_MODE_RESTART)
        /// ```
        ///
        /// Will result in:
        /// - Loop 1: `origin(  0, 0) -> target(100, 0)`.
        /// - Loop 2: `origin(  0, 0) -> target(100, 0)`.
        /// - Loop 3: `origin(  0, 0) -> target(100, 0)`.
        [RS = "Restart", GD = "LOOP_MODE_RESTART"]
        #[default]
        Restart = 0,
        /// Tweening will reverse direction each time it loops. If the tween is not configured to loop,
        /// then this is the same as [constant Spire.LOOP_MODE_RESTART].
        ///
        /// # Formula
        /// Here's the internal formula (translated to GDScript) used to determine
        /// the current animation position in this mode, where:
        /// - `loop_time` is the time elapsed in the current loop cycle (resets to 0.0 at the start of each loop).
        /// - `duration` is the total duration of one loop cycle (defined by the user when creating the tween).
        /// - `loop_count` is the number of completed loops (starting from 0).
        ///
        /// ```gdscript
        ///  var raw_position: float = loop_time / duration;
        ///  if loop_count % 2 == 0: # Even loop (0, 2, 4, ...)
        ///     return ease(raw_position)      # Normal progress
        ///  else:                   # Odd loop (1, 3, 5, ...)
        ///     return ease(1. - raw_position) # Yoyo simply reverses the progress
        ///
        /// ```
        ///
        /// # Example
        ///
        /// The tween:
        /// ```gdscript
        /// var origin := Vector2(0, 0)
        /// var target := Vector2(100, 0)
        /// var duration := 3.0
        /// var tween := DoNode2d(node, target, duration) \
        ///     .from(origin) \
        ///     .set_loops(3, Spire.LOOP_MODE_YOYO)
        /// ```
        ///
        /// Will result in:
        /// - Loop 1: `origin(  0, 0) -> target(100, 0)`.
        /// - Loop 2: `target(100, 0) -> origin(  0, 0)`.
        /// - Loop 3: `origin(  0, 0) -> target(100, 0)`.
        [RS = "Yoyo", GD = "LOOP_MODE_YOYO"]
        Yoyo = 1,
        /// Tweening will continue from the end value each time it loops,
        /// effectively incrementing the start and end values by the original difference.
        ///
        /// # Formula
        /// Here's the internal formula (translated to GDScript) used to determine
        /// the current animation position in this mode, where:
        /// - `loop_time` is the time elapsed in the current loop cycle (resets to 0.0 at the start of each loop).
        /// - `duration` is the total duration of one loop cycle (defined by the user when creating the tween).
        /// - `loop_count` is the number of completed loops (starting from 0).
        ///
        /// ```gdscript
        /// var raw_position: float = loop_time / duration;
        /// return loop_count + ease(raw_position)
        /// ```
        ///
        /// # Example
        ///
        /// The tween:
        /// ```gdscript
        /// var origin := Vector2(0, 0)
        /// var target := Vector2(100, 0)
        /// var duration := 3.0
        /// var tween := DoNode2d(node, target, duration) \
        ///     .from(origin) \
        ///     .set_loops(3, Spire.LOOP_MODE_INCREMENTAL)
        /// ```
        ///
        /// Will result in:
        /// - Loop 1: `origin(  0, 0) -> target(100, 0)`.
        /// - Loop 2: `target(100, 0) -> target(200, 0)`.
        /// - Loop 3: `target(200, 0) -> target(300, 0)`.
        [RS = "Incremental", GD = "LOOP_MODE_INCREMENTAL"]
        Incremental = 2,
    }
}
