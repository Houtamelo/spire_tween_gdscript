use super::*;

register_enum! {
    [GD = "Spiral"]
    Spiral {
        /// Distances between turnings grow geometrically (shrink if `growth < 0`).
        [RS = "Logarithmic", GD = "SPIRAL_LOGARITHMIC"]
        #[default]
        Logarithmic = 0,
        /// Constant distance between turnings.
        [RS = "Archimedean", GD = "SPIRAL_ARCHIMEDEAN"]
        Archimedean = 1,
        /// Pitch angle increases with distance; approaches an asymptotic line.
        [RS = "Hyperbolic", GD = "SPIRAL_HYPERBOLIC"]
        Hyperbolic = 2,
        /// Constant area between consecutive full turns; distance between turns grows inversely.
        [RS = "Fermat", GD = "SPIRAL_FERMAT"]
        Fermat = 3,
    }
}
