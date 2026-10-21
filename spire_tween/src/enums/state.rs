use super::*;

register_enum! {
    [GD = "State"]
    State {
        /// Completed, cancelled, or manually stopped. Entering this state emits
        /// `stopped`; successful completion also emits `finished`. Inside a
        /// sequence, the sequence moves on.
        [RS = "Stopped", GD = "STATE_STOPPED"]
        #[default]
        Stopped = 0,
        /// Not processing. Pausing an already-stopped tween leaves it stopped.
        /// A sequence resumes a paused child when its block is processed.
        [RS = "Paused", GD = "STATE_PAUSED"]
        Paused = 1,
        /// Allowed to process, but may be blocked by `PauseMode::Bound` if a bound
        /// node isn't processing. Inside a sequence, check the sequence's state instead.
        [RS = "Playing", GD = "STATE_PLAYING"]
        Playing = 2,
    }
}
