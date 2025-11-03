use super::*;

register_enum! {
    [GD = "ProcessMode"]
    ProcessMode {
        /// Process during physics time (e.g. [method Node._physics_process]).
        ///
        /// [b]Note:[/b] This does **NOT** mean that the tween will pause if the node it's bound to isn't
        /// processing (which is done with [method Node.set_process_mode]).
        /// Use `set_pause_mode` with [constant Spire.PAUSE_MODE_BOUND] if you want to make sure the tween
        /// only processes when the bound node is processing.
        [RS = "Physics", GD = "PROCESS_MODE_PHYSICS"]
        Physics = 0,
        /// Process during idle time (e.g. [method Node._process]).
        ///
        /// [b]Note:[/b] This does **NOT** mean that the tween will pause if the node it's bound to isn't
        /// processing (which is done with [method Node.set_process_mode]).
        /// Use `set_pause_mode` with [constant Spire.PAUSE_MODE_BOUND] if you want to make sure the tween
        /// only processes when the bound node is processing.
        [RS = "Idle", GD = "PROCESS_MODE_IDLE"]
        #[default]
        Idle = 1,
        /// Do not process automatically, must be manually done with `custom_step`. This is essentially the same
        /// as if the tween was unregistered.
        [RS = "Manual", GD = "PROCESS_MODE_MANUAL"]
        Manual = 2,
    }
}
