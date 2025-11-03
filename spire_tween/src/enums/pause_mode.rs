use super::*;

register_enum! {
    [GD = "PauseMode"]
    PauseMode {
        /// Process as long as all bound nodes are processing.
        [RS = "Bound", GD = "PAUSE_MODE_BOUND"]
        #[default]
        Bound = 0,
        /// Process only when [SceneTree] is not paused.
        [RS = "Stop", GD = "PAUSE_MODE_STOP"]
        Stop = 1,
        /// Always process, even when [SceneTree] is paused.
        [RS = "Process", GD = "PAUSE_MODE_PROCESS"]
        Process = 2,
    }
}

impl From<PauseMode> for TweenPauseMode {
    fn from(mode: PauseMode) -> Self {
        match mode {
            PauseMode::Bound => TweenPauseMode::BOUND,
            PauseMode::Stop => TweenPauseMode::STOP,
            PauseMode::Process => TweenPauseMode::PROCESS,
        }
    }
}
