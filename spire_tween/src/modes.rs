use godot::obj::EngineEnum;

use super::*;
use crate::internal_prelude::class_macros::meta::inspect::EnumConstant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, GodotConvert)]
#[godot(via = i32)]
#[repr(i32)]
pub enum SpireProcessMode {
    /// [TweenProcessMode::PHYSICS]
    /// Process during physics ticks (`_physics_process`).
    #[default]
    Physics = 0,
    /// [TweenProcessMode::IDLE]
    /// Process during idle time (`_process`).
    Idle = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, GodotConvert)]
#[godot(via = i32)]
#[repr(i32)]
pub enum SpirePauseMode {
    /// [TweenPauseMode::BOUND]
    /// Process as long as all bound nodes are processing.
    #[default]
    Bound = 0,
    /// [TweenPauseMode::STOP]
    /// Process only when SceneTree is not paused.
    Stop = 1,
    /// [TweenPauseMode::PROCESS]
    /// Always process, even when paused.
    Process = 2,
}

impl EngineEnum for SpireProcessMode {
    fn try_from_ord(ord: i32) -> Option<Self> {
        const PHYSICS: i32 = SpireProcessMode::Physics as i32;
        const IDLE: i32 = SpireProcessMode::Idle as i32;

        match ord {
            PHYSICS => Some(Self::Physics),
            IDLE => Some(Self::Idle),
            _ => None,
        }
    }

    fn ord(self) -> i32 { self as i32 }

    fn as_str(&self) -> &'static str {
        match *self {
            Self::Physics => "PHYSICS",
            Self::Idle => "IDLE",
        }
    }

    fn godot_name(&self) -> &'static str {
        match self {
            SpireProcessMode::Physics => "TWEEN_PROCESS_PHYSICS",
            SpireProcessMode::Idle => "TWEEN_PROCESS_IDLE",
        }
    }

    fn values() -> &'static [Self] { &[Self::Physics, Self::Idle] }

    fn all_constants() -> &'static [EnumConstant<Self>] {
        const {
            &[
                EnumConstant::new("PHYSICS", "TWEEN_PROCESS_PHYSICS", Self::Physics),
                EnumConstant::new("IDLE", "TWEEN_PROCESS_IDLE", Self::Idle),
            ]
        }
    }
}

impl EngineEnum for SpirePauseMode {
    fn try_from_ord(ord: i32) -> Option<Self> {
        const BOUND: i32 = SpirePauseMode::Bound as i32;
        const STOP: i32 = SpirePauseMode::Stop as i32;
        const PROCESS: i32 = SpirePauseMode::Process as i32;

        match ord {
            BOUND => Some(Self::Bound),
            STOP => Some(Self::Stop),
            PROCESS => Some(Self::Process),
            _ => None,
        }
    }

    fn ord(self) -> i32 { self as i32 }

    fn as_str(&self) -> &'static str {
        match *self {
            Self::Bound => "BOUND",
            Self::Stop => "STOP",
            Self::Process => "PROCESS",
        }
    }

    fn godot_name(&self) -> &'static str {
        match self {
            SpirePauseMode::Bound => "TWEEN_PAUSE_BOUND",
            SpirePauseMode::Stop => "TWEEN_PAUSE_STOP",
            SpirePauseMode::Process => "TWEEN_PAUSE_PROCESS",
        }
    }

    fn values() -> &'static [Self] { &[Self::Bound, Self::Stop, Self::Process] }

    fn all_constants() -> &'static [EnumConstant<Self>] {
        const {
            &[
                EnumConstant::new("BOUND", "TWEEN_PAUSE_BOUND", Self::Bound),
                EnumConstant::new("STOP", "TWEEN_PAUSE_STOP", Self::Stop),
                EnumConstant::new("PROCESS", "TWEEN_PAUSE_PROCESS", Self::Process),
            ]
        }
    }
}

impl From<SpireProcessMode> for TweenProcessMode {
    fn from(mode: SpireProcessMode) -> Self {
        match mode {
            SpireProcessMode::Physics => TweenProcessMode::PHYSICS,
            SpireProcessMode::Idle => TweenProcessMode::IDLE,
        }
    }
}

impl From<SpirePauseMode> for TweenPauseMode {
    fn from(mode: SpirePauseMode) -> Self {
        match mode {
            SpirePauseMode::Bound => TweenPauseMode::BOUND,
            SpirePauseMode::Stop => TweenPauseMode::STOP,
            SpirePauseMode::Process => TweenPauseMode::PROCESS,
        }
    }
}
