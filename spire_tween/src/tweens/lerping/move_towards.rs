#[allow(unused_imports)]
use super::*;

pub trait MoveTowards {
    fn move_towards(from: Self, to: Self, abs_move: Self) -> Self;
}

impl MoveTowards for f32 {
    #[inline]
    fn move_towards(from: Self, to: Self, abs_move: Self) -> Self {
        if from < to { Self::min(from + abs_move, to) } else { Self::max(from - abs_move, to) }
    }
}

impl MoveTowards for i32 {
    #[inline]
    fn move_towards(from: Self, to: Self, abs_move: Self) -> Self {
        if from < to { Self::min(from + abs_move, to) } else { Self::max(from - abs_move, to) }
    }
}

impl MoveTowards for f64 {
    #[inline]
    fn move_towards(from: Self, to: Self, abs_move: Self) -> Self {
        if from < to { Self::min(from + abs_move, to) } else { Self::max(from - abs_move, to) }
    }
}

impl MoveTowards for i64 {
    #[inline]
    fn move_towards(from: Self, to: Self, abs_move: Self) -> Self {
        if from < to { Self::min(from + abs_move, to) } else { Self::max(from - abs_move, to) }
    }
}
