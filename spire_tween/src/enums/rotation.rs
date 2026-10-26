use super::*;

register_enum! {
    [GD = "RotateMode"]
    RotateMode {
        /// Shortest signed Euler route on each axis.
        [RS = "Fast", GD = "ROTATE_MODE_FAST"]
        #[default]
        Fast = 0,
        /// Preserve all requested revolutions.
        [RS = "FastBeyond360", GD = "ROTATE_MODE_FAST_BEYOND_360"]
        FastBeyond360 = 1,
        /// Relative rotation about world axes.
        [RS = "WorldAxisAdd", GD = "ROTATE_MODE_WORLD_AXIS_ADD"]
        WorldAxisAdd = 2,
        /// Relative rotation about object-local axes.
        [RS = "LocalAxisAdd", GD = "ROTATE_MODE_LOCAL_AXIS_ADD"]
        LocalAxisAdd = 3,
    }
}
register_enum! {
    [GD = "RotationAxis"]
    RotationAxis {
        /// Change every Euler component.
        [RS = "All", GD = "ROTATION_AXIS_ALL"]
        #[default]
        All = 0,
        /// Change only X.
        [RS = "X", GD = "ROTATION_AXIS_X"]
        X = 1,
        /// Change only Y.
        [RS = "Y", GD = "ROTATION_AXIS_Y"]
        Y = 2,
        /// Change only Z.
        [RS = "Z", GD = "ROTATION_AXIS_Z"]
        Z = 4,
        /// Change X and Y.
        [RS = "XY", GD = "ROTATION_AXIS_XY"]
        XY = 3,
        /// Change X and Z.
        [RS = "XZ", GD = "ROTATION_AXIS_XZ"]
        XZ = 5,
        /// Change Y and Z.
        [RS = "YZ", GD = "ROTATION_AXIS_YZ"]
        YZ = 6,
    }
}
register_enum! {
    [GD = "SpiralMode"]
    SpiralMode {
        /// Increase the spiral radius throughout playback.
        [RS = "Expand", GD = "SPIRAL_MODE_EXPAND"]
        #[default]
        Expand = 0,
        /// Expand during the first half and contract during the second.
        [RS = "ExpandThenContract", GD = "SPIRAL_MODE_EXPAND_THEN_CONTRACT"]
        ExpandThenContract = 1,
    }
}
