use super::*;

register_enum! {
    [GD = "PathType"]
    PathType {
        /// Straight segments, traversed by distance.
        [RS = "Linear", GD = "PATH_TYPE_LINEAR"]
        #[default]
        Linear = 0,
        /// Uniform Catmull-Rom spline through the waypoints.
        [RS = "CatmullRom", GD = "PATH_TYPE_CATMULL_ROM"]
        CatmullRom = 1,
        /// Triples of destination, preceding handle, destination handle.
        [RS = "CubicBezier", GD = "PATH_TYPE_CUBIC_BEZIER"]
        CubicBezier = 2,
    }
}

register_enum! {
    [GD = "ScrambleMode"]
    ScrambleMode {
        /// Replace characters without scrambling.
        [RS = "None", GD = "SCRAMBLE_MODE_NONE"]
        #[default]
        None = 0,
        /// ASCII letters and digits.
        [RS = "All", GD = "SCRAMBLE_MODE_ALL"]
        All = 1,
        /// Uppercase ASCII letters.
        [RS = "Uppercase", GD = "SCRAMBLE_MODE_UPPERCASE"]
        Uppercase = 2,
        /// Lowercase ASCII letters.
        [RS = "Lowercase", GD = "SCRAMBLE_MODE_LOWERCASE"]
        Lowercase = 3,
        /// ASCII digits.
        [RS = "Numerals", GD = "SCRAMBLE_MODE_NUMERALS"]
        Numerals = 4,
        /// Unicode characters from the supplied alphabet.
        [RS = "Custom", GD = "SCRAMBLE_MODE_CUSTOM"]
        Custom = 5,
    }
}

register_enum! {
    [GD = "PathMode"]
    PathMode {
        /// Orient freely in 3D (or within the XY plane for Node2D).
        [RS = "Full3D", GD = "PATH_MODE_FULL_3D"]
        #[default]
        Full3D = 0,
        /// Leave rotation unchanged.
        [RS = "Ignore", GD = "PATH_MODE_IGNORE"]
        Ignore = 1,
        /// Orient within the XY plane.
        [RS = "SideScroller2D", GD = "PATH_MODE_SIDE_SCROLLER_2D"]
        SideScroller2D = 2,
        /// Orient within the XZ plane (XY for Node2D).
        [RS = "TopDown2D", GD = "PATH_MODE_TOP_DOWN_2D"]
        TopDown2D = 3,
    }
}
