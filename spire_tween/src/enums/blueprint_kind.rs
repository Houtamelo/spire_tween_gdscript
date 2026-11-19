use super::*;

register_enum! {
    [GD = "BlueprintKind"]
    BlueprintKind {
        [RS = "ChainBlock", GD = "BLUEPRINT_KIND_CHAIN_BLOCK"]
        #[default]
        ChainBlock = 0,
        [RS = "ParallelBlock", GD = "BLUEPRINT_KIND_PARALLEL_BLOCK"]
        ParallelBlock = 1,
        [RS = "Property", GD = "BLUEPRINT_KIND_PROPERTY"]
        Property = 2,
        /// Animate selected logical characters of a RichTextLabel.
        [RS = "Text", GD = "BLUEPRINT_KIND_TEXT"]
        Text = 3,
    }
}
