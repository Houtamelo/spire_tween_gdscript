use super::*;

#[godot_api(secondary)]
impl Spire {
    /// Creates a new, empty [SpireSequence] that can be used to chain and/or parallelize
    /// multiple tweens and calls.
    ///
    /// For more details on sequences, see their class documentation: [SpireSequence].
    #[func]
    pub fn sequence() -> Gd<SpireSequence> { SpireSequence::build() }
}
