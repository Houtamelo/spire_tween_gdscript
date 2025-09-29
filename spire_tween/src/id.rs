use std::sync::atomic::AtomicI64;

use godot::prelude::*;

use super::*;

/// Negative IDs are generated from the COUNTER static variable.
/// Positive IDs come from Godot Object instance IDs.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TweenId(pub i64);

static ID_COUNTER: AtomicI64 = AtomicI64::new(-4);

impl TweenId {
    pub fn new() -> Self { Self(ID_COUNTER.fetch_add(-1, std::sync::atomic::Ordering::Relaxed)) }

    pub fn from_handle<C: Inherits<RefCounted>>(handle: &Gd<C>) -> Self {
        Self(handle.instance_id_unchecked().to_i64())
    }
}

macro_rules! gd_handle_id {
    ($Gd:expr) => {{
        let gd = &$Gd;
        TweenId::from_handle(gd)
    }};
}

pub(crate) use gd_handle_id;
