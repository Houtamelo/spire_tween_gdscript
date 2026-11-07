use super::*;

register_enum! {
    [GD = "CharacterAnimation"]
    CharacterAnimation {
        [RS = "Offset", GD = "CHARACTER_ANIMATION_OFFSET"]
        #[default]
        Offset = 0,
        [RS = "Fade", GD = "CHARACTER_ANIMATION_FADE"]
        Fade = 1,
        [RS = "Color", GD = "CHARACTER_ANIMATION_COLOR"]
        Color = 2,
        [RS = "Rotation", GD = "CHARACTER_ANIMATION_ROTATION"]
        Rotation = 3,
        [RS = "Scale", GD = "CHARACTER_ANIMATION_SCALE"]
        Scale = 4,
        [RS = "PunchOffset", GD = "CHARACTER_ANIMATION_PUNCH_OFFSET"]
        PunchOffset = 5,
        [RS = "PunchRotation", GD = "CHARACTER_ANIMATION_PUNCH_ROTATION"]
        PunchRotation = 6,
        [RS = "PunchScale", GD = "CHARACTER_ANIMATION_PUNCH_SCALE"]
        PunchScale = 7,
        [RS = "ShakeOffset", GD = "CHARACTER_ANIMATION_SHAKE_OFFSET"]
        ShakeOffset = 8,
        [RS = "ShakeRotation", GD = "CHARACTER_ANIMATION_SHAKE_ROTATION"]
        ShakeRotation = 9,
        [RS = "ShakeScale", GD = "CHARACTER_ANIMATION_SHAKE_SCALE"]
        ShakeScale = 10,
    }
}
