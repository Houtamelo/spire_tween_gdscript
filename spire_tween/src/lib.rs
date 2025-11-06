#![feature(trait_alias)]
#![feature(type_changing_struct_update)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(slice_swap_unchecked)]
#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![feature(iter_intersperse)]
#![feature(never_type)]
#![feature(arbitrary_self_types)]
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

// TODO: Skeleton3D.do_bone_pose methods

mod benchmarking;
mod enums;
mod gdscript_bridge;
mod global;
mod object_or_node;
mod rc_ptr;
mod tweens;
mod util;

use ::godot::meta::Signature;
use enums::ProcessMode;
use internal_prelude::*;

pub mod prelude {
    #[doc(hidden)]
    pub struct BaseMarker;

    pub use crate::tweens::{BasicLerp, CustomLerper, LerpMode, SpireLerp};
    pub use crate::{
        //connection::Connection,
        enums::{EaseKind, Evaluator, LoopMode, PauseMode, ProcessMode, State},
        rc_ptr::*,
        tweens::{
            CompleteBoundTweens,
            DoDelayedCall,
            DoDelayedCallable,
            DoMethod,
            DoProperty,
            DoVarMethod,
            KillBoundTweens,
            LerpMethodData,
            LerpPropertyData,
            Sequence,
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
    pub(crate) use class_macros::{
        meta::{PropertyHintInfo, PropertyInfo},
        sys::GDExtensionClassMethodArgumentMetadata,
    };
    #[cfg(feature = "dashmap")]
    pub(crate) use dashmap::Equivalent;
    pub(crate) use derived_deref::{Deref, DerefMut};
    pub(crate) use godot::{
        classes::{
            tween::{TweenPauseMode, TweenProcessMode},
            *,
        },
        global::{PropertyHint, PropertyUsageFlags},
        meta::{AsArg, ClassName, GodotType, InParamTuple, ParamTuple, error::FromGodotError},
        obj::WithBaseField,
        prelude::*,
    };
    #[cfg(feature = "indexmap")]
    pub(crate) use indexmap::Equivalent;
    pub(crate) use replace_with::replace_with_or_abort;
    pub(crate) use smallvec::SmallVec;
    pub(crate) use smolset::{SmolSet, SmolSetIter};
    pub(crate) use spire_enum::prelude::*;

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

mod api_entry {
    use godot::prelude::*;

    struct SpireGdExtension;

    #[gdextension]
    unsafe impl ExtensionLibrary for SpireGdExtension {}
}
