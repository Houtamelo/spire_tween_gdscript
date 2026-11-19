use super::*;

register_enum! {
    [GD = "TextEffectKind"]
    TextEffectKind {
        [RS = "Color", GD = "TEXT_EFFECT_KIND_COLOR"]
        #[default]
        Color = 0,
        [RS = "Fade", GD = "TEXT_EFFECT_KIND_FADE"]
        Fade = 1,
        [RS = "Offset", GD = "TEXT_EFFECT_KIND_OFFSET"]
        Offset = 2,
        [RS = "Rotation", GD = "TEXT_EFFECT_KIND_ROTATION"]
        Rotation = 3,
        [RS = "Scale", GD = "TEXT_EFFECT_KIND_SCALE"]
        Scale = 4,
        [RS = "PunchOffset", GD = "TEXT_EFFECT_KIND_PUNCH_OFFSET"]
        PunchOffset = 5,
        [RS = "PunchRotation", GD = "TEXT_EFFECT_KIND_PUNCH_ROTATION"]
        PunchRotation = 6,
        [RS = "PunchScale", GD = "TEXT_EFFECT_KIND_PUNCH_SCALE"]
        PunchScale = 7,
        [RS = "ShakeOffset", GD = "TEXT_EFFECT_KIND_SHAKE_OFFSET"]
        ShakeOffset = 8,
        [RS = "ShakeRotation", GD = "TEXT_EFFECT_KIND_SHAKE_ROTATION"]
        ShakeRotation = 9,
        [RS = "ShakeScale", GD = "TEXT_EFFECT_KIND_SHAKE_SCALE"]
        ShakeScale = 10,
    }
}
