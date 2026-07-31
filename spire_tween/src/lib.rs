//! Tweening library for Godot 4 via [`gdext`](https://github.com/godot-rust/gdext),
//! inspired by Unity's [DOTween](https://dotween.demigiant.com/).
//!
//! # Quickstart
//!
//! ```ignore
//! use spire_tween::prelude::*;
//!
//! // Tween a known property — returns a builder. Don't forget `.register()`.
//! let handle = my_node
//!     .do_position(Vector2::new(640.0, 360.0), 2.0)
//!     .with_ease(EaseKind::Basic(Ease::OutCubic))
//!     .as_relative(Vector2::ZERO)
//!     .register();
//!
//! // Hook a callback for when it finishes (closure-based, Rust-only path).
//! handle.to_mut().finished_connect(
//!     || godot_print!("done!"),
//!     SpireFlags::DEFERRED | SpireFlags::ONE_SHOT,
//! );
//!
//! // Sequence multiple tweens.
//! let mut seq = SpireTween::<Sequence>::new();
//! seq.append(my_node.do_position(target_a, 1.0));
//! seq.join(my_node.do_color(Color::RED, 1.0));   // parallel with the above
//! seq.append(my_node.do_position(target_b, 1.0));
//! seq.register();
//! ```
//!
//! # The `prelude`
//!
//! `use spire_tween::prelude::*` brings in the core surface:
//!
//! - **Core types** — [`SpireTween`], [`AnyTween`],
//!   [`Sequence`], [`SpireFlags`].
//! - **Pointer types** — [`RcPtr`], [`WeakPtr`].
//! - **Enums** — [`Ease`], [`EaseKind`], [`Evaluator`],
//!   [`LoopMode`], [`PauseMode`], [`ProcessMode`],
//!   [`Spiral`], [`State`].
//! - **Tween-data types** — [`LerpPropertyData`], [`LerpMethodData`],
//!   plus the generated per-class adapter enums (`PropertyDataFloat`,
//!   `PropertyDataVec2`, …).
//! - **Constructor traits** — [`DoProperty`], [`DoMethod`],
//!   [`DoVarMethod`], [`DoDelayedCall`], [`DoDelayedCallable`].
//! - **Template traits** — [`DoBone`], [`DoContourShape2D`],
//!   [`DoEllipsis2D`], [`DoEllipsis3D`], [`DoFollow2D`],
//!   [`DoFollow3D`], [`DoShakeNode2D`], [`DoShakeControl`],
//!   [`DoSpiral`].
//! - **Lifecycle helpers** — [`CompleteBoundTweens`], [`KillBoundTweens`].
//! - **Custom-lerper plumbing** — [`BasicLerp`], [`SpireLerp`],
//!   [`CustomLerper`], [`LerpMode`], [`ITweenable`],
//!   [`SpireTweener`].
//!
//! # Two register paths
//!
//! Pick based on who needs to listen for `finished` / `loop_finished`:
//! - [`SpireTween::register`] — pure-Rust path. Returns an [`RcPtr`] handle.
//!   Subscribe to events via [`SpireTween::finished_connect`] etc. (closure-based).
//! - [`SpireTween::register_with_gd_handle`] — also attaches a `Gd<Spire…>` wrapper
//!   so GDScript code (or any consumer of Godot signals) can connect to the
//!   `finished` / `loop_finished` Godot signals on that handle.
//!
//! # Threading
//!
//! Spire is single-threaded — same constraint as Godot's main loop. The internal
//! `RcPtr<T>` is a `Rc<UnsafeCell<T>>` wrapper that relies on Godot's main-thread
//! invariant; do not register or access tweens from worker threads.
//!
//! # In-editor docs (GDScript audience)
//!
//! When the `standalone` feature is enabled, the GDScript-facing classes are
//! registered with rich godot-flavored docs (extracted via the `register-docs`
//! gdext feature). Those are what shows up in the Godot editor's class browser. The
//! Rust API documented here mirrors the same concepts; the GDScript layer is built
//! on top.
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

    pub fn new_sequence() -> SpireTween<Sequence> { SpireTween::<Sequence>::new() }
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
        sys::GDExtensionClassMethodArgumentMetadata,
    };
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
