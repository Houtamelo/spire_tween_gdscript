use super::*;

register_enum! {
    [GD = "State"]
    State {
        /// Completed or manually stopped. Inside a sequence, the sequence moves on.
        [RS = "Stopped", GD = "STATE_STOPPED"]
        #[default]
        Stopped = 0,
        /// Not processing. Inside a sequence, this means the tween's turn hasn't come yet;
        /// the sequence auto-promotes to `Playing` when it does.
        [RS = "Paused", GD = "STATE_PAUSED"]
        Paused = 1,
        /// Allowed to process, but may be blocked by `PauseMode::Bound` if a bound
        /// node isn't processing. Inside a sequence, check the sequence's state instead.
        [RS = "Playing", GD = "STATE_PLAYING"]
        Playing = 2,
    }
}
