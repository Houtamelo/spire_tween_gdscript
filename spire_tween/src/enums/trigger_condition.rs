use super::*;

register_enum! {
    [GD = "TriggerCondition"]
    TriggerCondition {
        [RS = "Manual", GD = "TRIGGER_CONDITION_MANUAL"]
        #[default]
        Manual = 0,
        [RS = "AfterReady", GD = "TRIGGER_CONDITION_AFTER_READY"]
        AfterReady = 1,
        [RS = "AfterEnterTree", GD = "TRIGGER_CONDITION_AFTER_ENTER_TREE"]
        AfterEnterTree = 2,
        [RS = "MouseEnter", GD = "TRIGGER_CONDITION_MOUSE_ENTER"]
        MouseEnter = 3,
        [RS = "MouseExit", GD = "TRIGGER_CONDITION_MOUSE_EXIT"]
        MouseExit = 4,
        [RS = "MouseClickLeft", GD = "TRIGGER_CONDITION_MOUSE_CLICK_LEFT"]
        MouseClickLeft = 5,
        [RS = "MouseClickRight", GD = "TRIGGER_CONDITION_MOUSE_CLICK_RIGHT"]
        MouseClickRight = 6,
    }
}
