use super::*;

register_enum! {
    [GD = "ShakeRandomnessMode"]
    ShakeRandomnessMode {
        /// Perturbs each opposite-point turn in either direction.
        [RS = "Full", GD = "SHAKE_RANDOMNESS_MODE_FULL"]
        #[default]
        Full = 0,
        /// Perturbs opposite-point turns in one direction.
        [RS = "Harmonic", GD = "SHAKE_RANDOMNESS_MODE_HARMONIC"]
        Harmonic = 1,
    }
}
