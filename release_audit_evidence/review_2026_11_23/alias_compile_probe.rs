//! Compile-only witness: the public safe API permits overlapping references.
//! Do not execute these functions; compilation is not a runtime safety test.
use spire_tween::prelude::*;

pub fn rc_aliases(pointer: &RcPtr<i64>) -> (&mut i64, &mut i64) {
    (pointer.to_mut(), pointer.to_mut())
}

type PropertyTween = SpireTween<LerpPropertyData<f64>>;

pub fn tween_aliases(pointer: &TweenPtr<LerpPropertyData<f64>>) -> (&mut PropertyTween, &mut PropertyTween) {
    (pointer.to_mut(), pointer.to_mut())
}

pub fn writes_through_cloned_pointers() -> i64 {
    let mut first = RcPtr::new(0_i64);
    let mut second = first.clone();
    let a = &mut *first;
    let b = &mut *second;
    *a = 1;
    *b = 2;
    *a + *b
}
