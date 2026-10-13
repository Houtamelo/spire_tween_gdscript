//! Delayed-call cancellation and invocation against real Godot callables.
use std::panic::{AssertUnwindSafe, catch_unwind};

use godot::prelude::*;
use spire_tween::prelude::*;

use super::{
    ownership::{CatchTestPanic, frames},
    util::PinnedTestTask,
};

type TestFn = fn(&mut Gd<Node>) -> PinnedTestTask;
const CASES: &[(&str, TestFn)] = &[
    ("freed_node", freed_node),
    ("freed_refcounted", freed_refcounted),
    ("initially_invalid", initially_invalid),
    ("invalid_force_complete", invalid_force_complete),
    ("invalid_manual_processing", invalid_manual_processing),
    ("valid_custom_delay", valid_custom_delay),
    ("valid_force_complete_and_replay", valid_force_complete_and_replay),
    ("receiver_and_callable_target_differ", receiver_and_callable_target_differ),
    ("sequence_continues_after_invalid_child", sequence_continues_after_invalid_child),
];

fn node(parent: &mut Gd<Node>) -> Gd<Node> {
    let target = Node::new_alloc();
    parent.add_child(&target);
    target
}

fn assert_cancelled(tween: &TweenPtr<Callable>) {
    assert!(tween.is_stopped(), "invalid call must stop");
    assert!(!tween.is_registered(), "invalid call must unregister despite native ownership");
}

fn freed_node(parent: &mut Gd<Node>) -> PinnedTestTask {
    let mut target = node(parent);
    let call = Callable::from_object_method(&target, "queue_free");
    let tween = SpireTween::<Callable>::new(call.clone(), 100.0).register();
    target.queue_free();
    Box::pin(async move {
        frames(8).await;
        assert!(!call.is_valid());
        assert_cancelled(&tween);
    })
}

fn freed_refcounted(_: &mut Gd<Node>) -> PinnedTestTask {
    let target = RefCounted::new_gd();
    let call = Callable::from_object_method(&target, "get_instance_id");
    let tween = SpireTween::<Callable>::new(call.clone(), 100.0).register();
    drop(target);
    Box::pin(async move {
        frames(8).await;
        assert!(!call.is_valid());
        assert_cancelled(&tween);
    })
}

fn initially_invalid(parent: &mut Gd<Node>) -> PinnedTestTask {
    let target = node(parent);
    let empty = SpireTween::<Callable>::new(Callable::invalid(), 100.0).register();
    let missing =
        SpireTween::<Callable>::new(Callable::from_object_method(&target, "missing_delayed_method"), 100.0).register();
    Box::pin(async move {
        frames(8).await;
        assert_cancelled(&empty);
        assert_cancelled(&missing);
    })
}

fn invalid_force_complete(_: &mut Gd<Node>) -> PinnedTestTask {
    let events = RcPtr::new(0usize);
    for paused in [false, true] {
        let mut tween = SpireTween::<Callable>::new(Callable::invalid(), 100.0).register();
        let finished = events.clone();
        tween
            .to_mut()
            .finished_connect(move || *finished.to_mut() += 1, SpireFlags::empty());
        let looped = events.clone();
        tween
            .to_mut()
            .loop_finished_connect(move || *looped.to_mut() += 1, SpireFlags::empty());
        if paused {
            tween.pause();
        }
        tween.force_complete();
        assert_cancelled(&tween);
    }
    // A raw unregistered builder has no TweenPtr or Godot wrapper to recover.
    let mut builder = SpireTween::<Callable>::new(Callable::invalid(), 100.0);
    builder.force_complete();
    assert!(builder.is_stopped());
    Box::pin(async move {
        frames(4).await;
        assert_eq!(*events, 0, "cancellation must not emit completion events");
    })
}

fn invalid_manual_processing(_: &mut Gd<Node>) -> PinnedTestTask {
    let mut tween = SpireTween::<Callable>::new(Callable::invalid(), 100.0)
        .with_process_mode(ProcessMode::Manual)
        .register();
    let _ = tween.custom_step(0.0, false);
    assert_cancelled(&tween);
    Box::pin(async { frames(4).await })
}

fn valid_custom_delay(_: &mut Gd<Node>) -> PinnedTestTask {
    let calls = RcPtr::new(0usize);
    let counter = calls.clone();
    let callable = Callable::from_fn("valid_delay", move |_| {
        *counter.to_mut() += 1;
        Variant::nil()
    });
    assert!(callable.is_valid() && callable.object().is_none());
    let mut tween = SpireTween::<Callable>::new(callable, 0.2)
        .with_process_mode(ProcessMode::Manual)
        .register();
    let _ = tween.custom_step(0.1, false);
    assert_eq!(*calls, 0);
    assert!(tween.is_playing());
    let _ = tween.custom_step(0.11, false);
    assert_eq!(*calls, 1);
    assert!(tween.is_stopped());
    tween.unregister();
    Box::pin(async { frames(4).await })
}

fn valid_force_complete_and_replay(_: &mut Gd<Node>) -> PinnedTestTask {
    let calls = RcPtr::new(0usize);
    let counter = calls.clone();
    let callable = Callable::from_fn("valid_force", move |_| {
        *counter.to_mut() += 1;
        Variant::nil()
    });
    let mut tween = SpireTween::<Callable>::new(callable, 100.0).register();
    let events = RcPtr::new(0usize);
    let finished = events.clone();
    tween
        .to_mut()
        .finished_connect(move || *finished.to_mut() += 1, SpireFlags::empty());
    tween.force_complete();
    assert_eq!((*calls, *events), (1, 1));
    tween.play();
    tween.pause();
    tween.force_complete();
    assert_eq!((*calls, *events), (2, 2));
    assert!(tween.is_stopped());
    tween.unregister();
    Box::pin(async { frames(4).await })
}

fn receiver_and_callable_target_differ(parent: &mut Gd<Node>) -> PinnedTestTask {
    let receiver = node(parent);
    let mut target = node(parent);
    let call = Callable::from_object_method(&target, "queue_free");
    let tween = receiver.do_delayed_callable(call, 100.0).register();
    target.queue_free();
    Box::pin(async move {
        frames(8).await;
        assert!(receiver.is_instance_valid(), "receiver remains alive");
        assert_cancelled(&tween);
    })
}

fn sequence_continues_after_invalid_child(_: &mut Gd<Node>) -> PinnedTestTask {
    let child = SpireTween::<Callable>::new(Callable::invalid(), 100.0).register();
    let calls = RcPtr::new(0usize);
    let counter = calls.clone();
    let mut sequence = TweenSequence::new();
    sequence.append_ptr(child.clone());
    sequence.append_call(Callable::from_fn("after_invalid", move |_| {
        *counter.to_mut() += 1;
        Variant::nil()
    }));
    let mut parent = sequence.register();
    Box::pin(async move {
        frames(16).await;
        assert_cancelled(&child);
        assert_eq!(*calls, 1, "sequence must continue past the invalid child");
        assert!(parent.is_stopped());
        assert!(!parent.to_mut().remove(&child), "cancelled child is removed from the sequence");
        parent.unregister();
    })
}

pub(super) async fn run_delayed_call_validity_tests(parent: &mut Gd<Node>) -> usize {
    let mut failures = 0;
    for &(name, test) in CASES {
        let mut fixture = Node::new_alloc();
        parent.add_child(&fixture);
        godot_print!("NATIVE_DELAY_VALIDITY_START {name}");
        let task = catch_unwind(AssertUnwindSafe(|| test(&mut fixture)));
        let result = match task {
            Ok(task) => CatchTestPanic(task).await,
            Err(error) => Err(error),
        };
        if result.is_ok() {
            godot_print!("NATIVE_DELAY_VALIDITY_PASS {name}");
        } else {
            failures += 1;
            godot_print!("NATIVE_DELAY_VALIDITY_FAIL {name}");
        }
        fixture.queue_free();
        frames(4).await;
    }
    godot_print!("NATIVE_DELAY_VALIDITY_RESULT total={} failed={failures}", CASES.len());
    failures
}

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct DelayedCallValidityTestRunner {
    base: Base<Node>,
}

#[godot_api]
impl INode for DelayedCallValidityTestRunner {
    fn ready(&mut self) {
        let mut parent = self.base().clone();
        godot::task::spawn(async move {
            let failed = run_delayed_call_validity_tests(&mut parent).await;
            parent
                .get_tree()
                .quit_ex()
                .exit_code(if failed == 0 { 0 } else { 1 })
                .done();
        });
    }
}
