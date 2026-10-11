mod handle;
mod strong;
mod weak;

use std::{
    cell::UnsafeCell,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

pub(crate) use handle::GdHandleSlot;
pub use strong::TweenPtr;
pub use weak::WeakTweenPtr;

use crate::internal_prelude::*;
