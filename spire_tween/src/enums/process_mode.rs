use super::*;

register_enum! {
    [GD = "ProcessMode"]
    ProcessMode {
        /// Ticked during `_physics_process`. Does not imply the tween pauses when
        /// its bound node stops processing -- use `PauseMode::Bound` for that.
        [RS = "Physics", GD = "PROCESS_MODE_PHYSICS"]
        Physics = 0,
        /// Ticked during `_process`. Same caveat as `Physics` regarding bound node processing.
        [RS = "Idle", GD = "PROCESS_MODE_IDLE"]
        #[default]
        Idle = 1,
        /// Not ticked automatically; must be advanced manually with `custom_step`.
        [RS = "Manual", GD = "PROCESS_MODE_MANUAL"]
        Manual = 2,
    }
}
