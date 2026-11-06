use super::*;

register_enum! {
    [GD = "LerpModeKind"]
    LerpModeKind {
        [RS = "Absolute", GD = "LERP_MODE_KIND_ABSOLUTE"]
        #[default]
        Absolute = 0,
        [RS = "SpeedBased", GD = "LERP_MODE_KIND_SPEED_BASED"]
        SpeedBased = 1,
        [RS = "Relative", GD = "LERP_MODE_KIND_RELATIVE"]
        Relative = 2,
    }
}
