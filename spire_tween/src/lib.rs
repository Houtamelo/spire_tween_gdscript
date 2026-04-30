//! Tweening library for Godot 4 via gdext, inspired by DoTween.
//!
//! Import `spire_tween::prelude::*` and use the extension traits on `Gd<T>` or
//! `WithBaseField` types.
#![feature(type_changing_struct_update)]
#![feature(unboxed_closures)]
#![feature(arbitrary_self_types)]
#![feature(stmt_expr_attributes)]
#![cfg_attr(test, feature(test))]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::manual_try_fold)]
#![allow(clippy::result_large_err)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::new_without_default)]
#![allow(clippy::empty_docs)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(clippy::mut_from_ref)]
#![allow(clippy::infallible_try_from)]

mod benchmarking;
mod enums;
#[cfg(feature = "standalone")]
mod gdscript_bridge;
mod global;
mod object_or_node;
mod rc_ptr;
mod smol_set;
mod tweens;
mod util;

use ::godot::private::Signature;
use enums::ProcessMode;
use internal_prelude::*;

pub mod prelude {
    #[doc(hidden)]
    pub struct BaseMarker;

    pub use crate::tweens::{BasicLerp, CustomLerper, LerpMode, SpireLerp};
    pub use crate::{
        //connection::Connection,
        enums::{Ease, EaseKind, Evaluator, LoopMode, PauseMode, ProcessMode, Spiral, State},
        rc_ptr::*,
        tweens::{
            AnyTween,
            CompleteBoundTweens,
            DoBone,
            DoContourShape2D,
            DoDelayedCall,
            DoDelayedCallable,
            DoEllipsis2D,
            DoEllipsis3D,
            DoFollow2D,
            DoFollow3D,
            DoMethod,
            DoProperty,
            DoShakeControl,
            DoShakeNode2D,
            DoSpiral,
            DoVarMethod,
            ITweenable,
            KillBoundTweens,
            LerpMethodData,
            LerpPropertyData,
            Sequence,
            SpireFlags,
            SpireTween,
            SpireTweener,
            generated_classes_data::*,
        },
    };
}

#[allow(unused_imports)]
pub(crate) mod internal_prelude {
    pub(crate) use std::{
        any::{Any, type_name},
        cell::UnsafeCell,
        collections::HashMap,
        fmt::Debug,
        hash::{Hash, Hasher},
        iter::Cloned,
        ops::{Deref, DerefMut},
        ptr::addr_eq,
        sync::LazyLock,
    };

    pub(crate) use anyhow::{anyhow, bail};
    pub(crate) use godot::sys::GDExtensionClassMethodArgumentMetadata;
    #[cfg(feature = "dashmap")]
    pub(crate) use dashmap::Equivalent;
    pub(crate) use derived_deref::{Deref, DerefMut};
    pub(crate) use godot::{
        builtin::Side,
        classes::{
            tween::{TweenPauseMode, TweenProcessMode},
            *,
        },
        meta::{AsArg, ClassId, GodotType, conv::ByValue, shape::GodotShape},
        obj::WithBaseField,
        prelude::*,
        register::info::{PropertyHint, PropertyHintInfo, PropertyInfo, PropertyUsageFlags},
    };
    #[cfg(feature = "indexmap")]
    pub(crate) use indexmap::Equivalent;
    pub(crate) use replace_with::replace_with_or_abort;
    pub(crate) use smallvec::SmallVec;
    pub(crate) use spire_enum::prelude::*;

    pub(crate) use crate::smol_set::SmolSet;
    pub(crate) use crate::{
        // connection::*,
        // cow_fn::*,
        enums::*,
        global::*,
        object_or_node::*,
        prelude::*,
        rc_ptr::*,
        tweens::*,
        util::*,
    };
}

#[cfg(feature = "standalone")]
mod api_entry {
    use godot::prelude::*;

    struct SpireGdExtension;

    #[gdextension]
    unsafe impl ExtensionLibrary for SpireGdExtension {}
}
