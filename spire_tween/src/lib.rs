#![feature(trait_alias)]
#![feature(let_chains)]
#![feature(type_changing_struct_update)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(slice_swap_unchecked)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::manual_try_fold)]
#![allow(clippy::result_large_err)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::new_without_default)]
#![doc = include_str!("../README.md")]

mod benchmarking;
mod cow_fn;
mod ease;
mod id;
mod modes;
mod object_or_node;
mod shared_ptr;
mod singleton;
mod tweens;
mod tweens_map;
mod util;

use internal_prelude::*;

pub mod prelude {
    #[doc(hidden)]
    pub struct BaseMarker;

    pub use id::TweenId;

    use crate::id;
    pub use crate::{
        ease::Ease,
        tweens::{
            AutoPlay,
            CompleteBoundTweens,
            DelayedCallData,
            DoDelayedCall,
            DoDelayedCallable,
            DoMethod,
            DoProperty,
            DoVarMethod,
            Duration,
            FetchError,
            KillBoundTweens,
            LerpMode,
            LerpPropertyData,
            LoopMode,
            MethodData,
            Sequence,
            SpireHandle,
            SpireLerp,
            SpireTween,
            SpireTweener,
            TweenState,
            generated_classes::*,
        },
    };

    pub type SpireSequence = SpireTween<Sequence>;
    pub type SpireDelayedCall = SpireTween<DelayedCallData>;
    pub type SpireProperty<T> = SpireTween<LerpPropertyData<T>>;
    pub type SpireMethod<T> = SpireTween<MethodData<T>>;
}

#[allow(unused_imports)]
pub(crate) mod internal_prelude {
    pub(crate) use std::{
        any::type_name,
        collections::HashMap,
        fmt::Debug,
        iter::Cloned,
        ops::{Deref, DerefMut},
    };

    pub(crate) use anyhow::{anyhow, bail};
    pub(crate) use derived_deref::{Deref, DerefMut};
    pub(crate) use godot::{
        classes::{
            tween::{TweenPauseMode, TweenProcessMode},
            *,
        },
        meta::AsArg,
        obj::WithBaseField,
        prelude::*,
    };
    pub(crate) use smolset::{SmolSet, SmolSetIter};
    pub(crate) use spire_enum::prelude::*;

    pub(crate) use crate::{
        cow_fn::*,
        ease::*,
        id::*,
        modes::*,
        object_or_node::*,
        prelude::*,
        shared_ptr::*,
        singleton::*,
        tweens::*,
        tweens_map::*,
        util::*,
    };
}
