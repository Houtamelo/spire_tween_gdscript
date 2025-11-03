mod strong;
mod weak;

use std::{
    cell::UnsafeCell,
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
};

pub use strong::*;
pub use weak::*;

use super::*;

pub trait Address {
    fn address(&self) -> *const ();
}
