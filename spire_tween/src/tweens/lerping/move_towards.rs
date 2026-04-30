#[allow(unused_imports)]
use super::*;

pub trait MoveTowards {
    fn move_towards(self, to: Self, abs_move: Self) -> Self;
}

impl MoveTowards for f32 {
    #[inline]
    fn move_towards(self, to: Self, abs_move: Self) -> Self {
        if self < to { Self::min(self + abs_move, to) } else { Self::max(self - abs_move, to) }
    }
}

impl MoveTowards for i32 {
    #[inline]
    fn move_towards(self, to: Self, abs_move: Self) -> Self {
        if self < to { Self::min(self + abs_move, to) } else { Self::max(self - abs_move, to) }
    }
}

impl MoveTowards for f64 {
    #[inline]
    fn move_towards(self, to: Self, abs_move: Self) -> Self {
        if self < to { Self::min(self + abs_move, to) } else { Self::max(self - abs_move, to) }
    }
}

impl MoveTowards for i64 {
    #[inline]
    fn move_towards(self, to: Self, abs_move: Self) -> Self {
        if self < to { Self::min(self + abs_move, to) } else { Self::max(self - abs_move, to) }
    }
}
