use super::*;

register_enum! {
    [GD = "LoopMode"]
    LoopMode {
        /// Restarts from the beginning each loop.
        [RS = "Restart", GD = "LOOP_MODE_RESTART"]
        #[default]
        Restart = 0,
        /// Reverses direction each loop (even loops go forward, odd loops go backward).
        [RS = "Yoyo", GD = "LOOP_MODE_YOYO"]
        Yoyo = 1,
        /// Continues from the end value each loop, incrementing start/end by the original delta.
        [RS = "Incremental", GD = "LOOP_MODE_INCREMENTAL"]
        Incremental = 2,
    }
}
