//! Native/Godot ownership regressions. All cases run inside a real SceneTree.
use std::{
    any::Any,
    panic::{AssertUnwindSafe, catch_unwind},
    pin::Pin,
    task::{Context, Poll},
};

use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::{PinnedTestTask, next_frame};

type TestFn = fn(&mut TweenOwnershipTests) -> PinnedTestTask;
type SequenceHandle = <Sequence as ITweenable>::GdHandle;

const CASES: &[(&str, TestFn)] = &[
    ("native_pointer_counts", TweenOwnershipTests::native_pointer_counts),
    ("last_native_owner_releases_wrapper", TweenOwnershipTests::last_native_owner_releases_wrapper),
    ("last_godot_owner_releases_tween", TweenOwnershipTests::last_godot_owner_releases_tween),
    ("clone_promotes_handle", TweenOwnershipTests::clone_promotes_handle),
    ("weak_upgrade_promotes_handle", TweenOwnershipTests::weak_upgrade_promotes_handle),
    ("weak_pointer_does_not_keep_completed_tween", TweenOwnershipTests::weak_pointer_does_not_keep_completed_tween),
    ("native_owner_preserves_godot_signals", TweenOwnershipTests::native_owner_preserves_godot_signals),
    ("unregistered_handle_can_register_again", TweenOwnershipTests::unregistered_handle_can_register_again),
    ("pointer_upgrade_during_signal", TweenOwnershipTests::pointer_upgrade_during_signal),
    ("external_signal_handle", TweenOwnershipTests::external_signal_handle),
];

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct TweenOwnershipTests {
    base: Base<Node>,
}

impl TweenOwnershipTests {
    fn native_pointer_counts(&mut self) -> PinnedTestTask {
        Box::pin(async move {
            let pointer: TweenPtr<Sequence> = TweenPtr::new(TweenSequence::new());
            let weak: WeakTweenPtr<Sequence> = pointer.downgrade();
            assert_eq!(pointer.strong_count(), 1);
            let copy = pointer.clone();
            assert_eq!(pointer.strong_count(), 2);
            assert!(pointer == copy);
            assert!(pointer == weak);
            drop(copy);
            assert_eq!(pointer.strong_count(), 1);
            let upgraded = weak.upgrade().expect("live native pointer upgrades");
            assert_eq!(pointer.strong_count(), 2);
            drop(upgraded);
            drop(pointer);
            assert!(weak.upgrade().is_none(), "expired native pointer must not upgrade");
        })
    }

    fn last_native_owner_releases_wrapper(&mut self) -> PinnedTestTask {
        let (mut pointer, handle) = TweenSequence::new().register_with_gd_handle();
        let id = handle.instance_id();
        let weak = pointer.downgrade();
        pointer.unregister();
        drop(handle);
        Box::pin(async move {
            frames(4).await;
            assert_eq!(pointer.strong_count(), 2, "wrapper plus Rust owner");
            assert!(SequenceHandle::try_from_instance_id(id).is_ok());
            drop(pointer);
            frames(4).await;
            assert!(SequenceHandle::try_from_instance_id(id).is_err());
            assert!(weak.upgrade().is_none());
        })
    }

    fn last_godot_owner_releases_tween(&mut self) -> PinnedTestTask {
        let (mut pointer, handle) = TweenSequence::new().register_with_gd_handle();
        let id = handle.instance_id();
        let weak = pointer.downgrade();
        pointer.unregister();
        Box::pin(async move {
            frames(4).await;
            drop(pointer);
            assert_eq!(handle.upcast_ref::<godot::classes::RefCounted>().get_reference_count(), 1);
            assert!(!handle.bind().is_registered());
            drop(handle);
            frames(4).await;
            assert!(SequenceHandle::try_from_instance_id(id).is_err());
            assert!(weak.upgrade().is_none());
        })
    }

    fn clone_promotes_handle(&mut self) -> PinnedTestTask {
        let (mut pointer, handle) = TweenSequence::new().register_with_gd_handle();
        let weak = pointer.downgrade();
        let id = handle.instance_id();
        pointer.unregister();
        Box::pin(async move {
            frames(4).await;
            drop(pointer);
            // The wrapper is now the sole native owner. Clone its pointer,
            // then discard the caller's Godot reference.
            let copy = weak.upgrade().unwrap();
            drop(copy);
            // The public conversion used by Spire.register clones the native
            // pointer. Exercise that Clone path through Gd registration.
            handle.bind().register();
            drop(handle);
            frames(4).await;
            let retained = SequenceHandle::try_from_instance_id(id);
            // An empty sequence completes immediately; collection is expected.
            assert!(retained.is_err());
            frames(4).await;
            assert!(weak.upgrade().is_none());
        })
    }

    fn weak_upgrade_promotes_handle(&mut self) -> PinnedTestTask {
        let (mut pointer, handle) = TweenSequence::new().register_with_gd_handle();
        let weak = pointer.downgrade();
        let id = handle.instance_id();
        pointer.unregister();
        Box::pin(async move {
            frames(4).await;
            drop(pointer);
            let upgraded = weak.upgrade().expect("caller-held wrapper owns a live native tween");
            drop(handle);
            assert!(SequenceHandle::try_from_instance_id(id).is_ok());
            assert_eq!(upgraded.strong_count(), 2, "upgraded owner plus wrapper");
            drop(upgraded);
            frames(4).await;
            assert!(SequenceHandle::try_from_instance_id(id).is_err());
            assert!(weak.upgrade().is_none());
        })
    }

    fn weak_pointer_does_not_keep_completed_tween(&mut self) -> PinnedTestTask {
        let (pointer, handle) = TweenSequence::new().register_with_gd_handle();
        let weak = pointer.downgrade();
        let id = handle.instance_id();
        pointer.to_mut().force_complete();
        drop(handle);
        drop(pointer);
        Box::pin(async move {
            frames(8).await;
            assert!(weak.upgrade().is_none(), "weak observers must not prevent collection");
            assert!(SequenceHandle::try_from_instance_id(id).is_err());
        })
    }

    fn native_owner_preserves_godot_signals(&mut self) -> PinnedTestTask {
        let mut sequence = TweenSequence::new();
        sequence.append_interval(0.02);
        let (pointer, handle) = sequence.register_with_gd_handle();
        let weak = pointer.downgrade();
        let id = handle.instance_id();
        let calls = RcPtr::new(0u32);
        let counter = calls.clone();
        handle.signals().finished().connect(move || *counter.to_mut() += 1);
        drop(handle);
        Box::pin(async move {
            frames(20).await;
            assert_eq!(*calls, 1, "a retained native owner must preserve Godot signal delivery");
            assert!(pointer.is_registered());
            assert!(SequenceHandle::try_from_instance_id(id).is_ok());
            drop(pointer);
            frames(8).await;
            assert!(weak.upgrade().is_none());
            assert!(SequenceHandle::try_from_instance_id(id).is_err());
        })
    }

    fn unregistered_handle_can_register_again(&mut self) -> PinnedTestTask {
        let (mut pointer, handle) = TweenSequence::new().register_with_gd_handle();
        let weak = pointer.downgrade();
        let id = handle.instance_id();
        pointer.unregister();
        Box::pin(async move {
            frames(4).await;
            drop(pointer);
            handle.bind().register();
            assert!(handle.bind().is_registered());
            handle.bind().force_complete();
            frames(4).await;
            assert!(handle.bind().is_registered(), "a caller-held completed wrapper remains registered");
            drop(handle);
            frames(4).await;
            assert!(weak.upgrade().is_none());
            assert!(SequenceHandle::try_from_instance_id(id).is_err());
        })
    }

    fn pointer_upgrade_during_signal(&mut self) -> PinnedTestTask {
        let (mut pointer, handle) = TweenSequence::new().register_with_gd_handle();
        pointer.unregister();
        let weak = pointer.downgrade();
        let observer = weak.clone();
        let calls = RcPtr::new(0u32);
        let counter = calls.clone();
        handle.signals().finished().connect(move || {
            // The force_complete call already holds a mutable native borrow.
            // Upgrade/promotion must touch only the separate handle-control slot.
            let temporary = observer.upgrade().unwrap();
            *counter.to_mut() += 1;
            drop(temporary);
        });
        Box::pin(async move {
            frames(4).await;
            drop(pointer);
            handle.bind().force_complete();
            assert_eq!(*calls, 1);
            assert_eq!(handle.upcast_ref::<godot::classes::RefCounted>().get_reference_count(), 1);
            drop(handle);
            frames(4).await;
            assert!(weak.upgrade().is_none());
        })
    }

    fn external_signal_handle(&mut self) -> PinnedTestTask {
        let (mut emitter, handle) = TweenSequence::new().register_with_gd_handle();
        emitter.unregister();
        drop(emitter);
        let calls = RcPtr::new(0u32);
        let counter = calls.clone();
        handle.signals().finished().connect(move || *counter.to_mut() += 1);
        let mut sequence = TweenSequence::new().with_handle(handle.clone());
        sequence.append_interval(0.02);
        let pointer = sequence.register();
        let weak = pointer.downgrade();
        drop(pointer);
        Box::pin(async move {
            frames(20).await;
            assert_eq!(*calls, 1, "with_handle must still deliver through a separate emitter");
            assert!(weak.upgrade().is_none(), "a separate emitter is not an owner of this native tween");
            drop(handle);
            frames(4).await;
        })
    }
}

pub(super) async fn frames(count: usize) {
    let mut tree = godot::classes::Engine::singleton()
        .get_main_loop()
        .unwrap()
        .cast::<godot::classes::SceneTree>();
    let timer = tree.create_timer(0.03);
    #[cfg(feature = "api-4-4")]
    let timer = timer.expect("SceneTree::create_timer returned null");
    Signal::from_object_signal(&timer, "timeout").to_future::<()>().await;
    drop(timer);
    for _ in 0..count {
        next_frame().await;
    }
}

/// gdext normally catches async panics itself. Catch each case here so the
/// focused release gate reports its own failure count and a nonzero exit code.
pub(super) struct CatchTestPanic(pub(super) PinnedTestTask);

impl Future for CatchTestPanic {
    type Output = Result<(), Box<dyn Any + Send>>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match catch_unwind(AssertUnwindSafe(|| self.0.as_mut().poll(cx))) {
            Ok(Poll::Ready(())) => Poll::Ready(Ok(())),
            Ok(Poll::Pending) => Poll::Pending,
            Err(error) => Poll::Ready(Err(error)),
        }
    }
}

pub(super) async fn run_ownership_tests(parent: &mut Gd<Node>) -> usize {
    let mut failures = 0;
    for &(name, test) in CASES {
        let mut node = TweenOwnershipTests::new_alloc();
        parent.add_child(&node);
        godot_print!("NATIVE_OWNERSHIP_START {name}");
        let task = catch_unwind(AssertUnwindSafe(|| test(&mut node.bind_mut())));
        let result = match task {
            Ok(task) => CatchTestPanic(task).await,
            Err(error) => Err(error),
        };
        if result.is_ok() {
            godot_print!("NATIVE_OWNERSHIP_PASS {name}");
        } else {
            failures += 1;
            godot_print!("NATIVE_OWNERSHIP_FAIL {name}");
        }
        node.queue_free();
        frames(4).await;
    }
    godot_print!("NATIVE_OWNERSHIP_RESULT total={} failed={failures}", CASES.len());
    failures
}

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct OwnershipTestRunner {
    base: Base<Node>,
}

#[godot_api]
impl INode for OwnershipTestRunner {
    fn ready(&mut self) {
        let mut parent = self.base().clone();
        godot::task::spawn(async move {
            let failed = run_ownership_tests(&mut parent).await;
            parent
                .get_tree()
                .quit_ex()
                .exit_code(if failed == 0 { 0 } else { 1 })
                .done();
        });
    }
}
