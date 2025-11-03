use super::*;

register_enum! {
    [GD = "State"]
    State {
        /// The tween either completed or was stopped.
        ///
        /// ## Inside sequences
        /// If inside a sequence, the sequence will ignore this tween and continue to the next one.
        [RS = "Stopped", GD = "STATE_STOPPED"]
        #[default]
        Stopped = 0,
        /// The tween is not processing updates.
        ///
        /// ## Inside sequences
        /// If inside a sequence, this state only means that this tween's turn hasn't come up yet.
        /// Also, the sequence will automatically switch this state to [constant Spire.STATE_PLAYING]
        /// when this tween's turn comes up.
        [RS = "Paused", GD = "STATE_PAUSED"]
        Paused = 1,
        /// The tween is **allowed** to process updates.
        ///
        /// However, it might not actually be doing so due to other conditions,
        /// such as if pause mode is [constant Spire.PAUSE_MODE_BOUND] and one of the bound nodes
        /// isn't processing.
        ///
        /// ## Inside sequences
        /// If inside a sequence, this state does not necessarily mean that this tween is currently being processed,
        /// it might be waiting for its turn. You should read the sequence's state instead.
        [RS = "Playing", GD = "STATE_PLAYING"]
        Playing = 2,
    }
}
