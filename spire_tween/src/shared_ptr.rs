use std::{cell::UnsafeCell, rc::Rc};

use super::*;

pub struct PtrTween {
    pub id: TweenId,
    tween:  Rc<UnsafeCell<AnyTween>>,
}

impl PtrTween {
    #[inline]
    pub fn new(tween: AnyTween, id: TweenId) -> Self {
        Self {
            id,
            tween: Rc::new(UnsafeCell::new(tween)),
        }
    }
}

impl Clone for PtrTween {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            tween: self.tween.clone(),
        }
    }
}

impl Deref for PtrTween {
    type Target = AnyTween;
    #[inline]
    fn deref(&self) -> &Self::Target { unsafe { &*self.tween.get() } }
}

impl DerefMut for PtrTween {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { &mut *self.tween.get() } }
}

impl PartialEq for PtrTween {
    #[inline]
    fn eq(&self, other: &Self) -> bool { Rc::ptr_eq(&self.tween, &other.tween) }
}

pub enum MaybeShared {
    Owned(AnyTween),
    Shared(PtrTween),
}

impl Deref for MaybeShared {
    type Target = AnyTween;

    fn deref(&self) -> &Self::Target {
        match self {
            MaybeShared::Owned(tween) => tween,
            MaybeShared::Shared(tween) => tween,
        }
    }
}

impl DerefMut for MaybeShared {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MaybeShared::Owned(tween) => tween,
            MaybeShared::Shared(tween) => &mut *tween,
        }
    }
}
