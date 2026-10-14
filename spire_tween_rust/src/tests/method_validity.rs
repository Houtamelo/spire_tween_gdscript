//! Method cancellation and successful completion against real Godot callables.
use std::panic::{AssertUnwindSafe, catch_unwind};

use godot::prelude::*;
use spire_tween::prelude::*;
use spire_tween::unversioned_api::TM;

use super::{ownership::{CatchTestPanic, frames}, util::PinnedTestTask};

type TestFn = fn(&mut Gd<Node>) -> PinnedTestTask;
const CASES: &[(&str, TestFn)] = &[
    ("freed_node", freed_node),
    ("freed_object", freed_object),
    ("initially_invalid", initially_invalid),
    ("invalid_force_complete_playing_and_paused", invalid_force_complete),
    ("invalid_manual_processing", invalid_manual_processing),
    ("invalid_unregistered_builder", invalid_builder),
    ("invalid_custom_method_skips_lerper", invalid_custom),
    ("valid_delay_and_completion", valid_completion),
    ("valid_force_complete_and_replay", valid_force_complete),
    ("sequence_continues_after_invalid_child", sequence_child),
];

fn method(callable: Callable, mode: ProcessMode) -> TweenMethodPtr<f64> {
    TweenMethod::<f64>::new(callable, 0.0, 2.0, 100.0).with_process_mode(mode).register()
}

fn events(tween: &mut TweenMethodPtr<f64>) -> RcPtr<[usize; 2]> {
    let counts = RcPtr::new([0, 0]);
    let finished = counts.clone();
    tween.to_mut().finished_connect(move || finished.to_mut()[0] += 1, SpireFlags::empty());
    let looped = counts.clone();
    tween.to_mut().loop_finished_connect(move || looped.to_mut()[1] += 1, SpireFlags::empty());
    counts
}

fn assert_cancelled(tween: &mut TweenMethodPtr<f64>, counts: &RcPtr<[usize; 2]>) {
    let state = (tween.is_stopped(), tween.is_registered(), **counts);
    // Cleanup before asserting, including when testing the pre-fix manual path.
    tween.unregister();
    assert_eq!(state, (true, false, [0, 0]), "invalid methods must cancel without completion events");
}

fn freed_node(parent: &mut Gd<Node>) -> PinnedTestTask {
    let mut parent = parent.clone();
    Box::pin(async move {
        for mode in [ProcessMode::Idle, ProcessMode::Physics] {
            let target = Node2D::new_alloc();
            parent.add_child(&target);
            let call = target.callable("set_rotation");
            let mut tween = method(call.clone(), mode);
            let counts = events(&mut tween);
            target.free();
            frames(8).await;
            assert!(!call.is_valid());
            assert_cancelled(&mut tween, &counts);
        }
    })
}

fn freed_object(_: &mut Gd<Node>) -> PinnedTestTask {
    let target = Object::new_alloc();
    let call = target.callable("get_instance_id");
    let mut tween = method(call.clone(), ProcessMode::Manual);
    let counts = events(&mut tween);
    target.free();
    assert!(!call.is_valid());
    let result = tween.custom_step(0.0, false);
    assert!(matches!(result, TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
    // The manual processor consumes ShouldDespawn and removes its registration.
    tween.unregister();
    assert_cancelled(&mut tween, &counts);
    Box::pin(async { frames(4).await })
}

fn initially_invalid(parent: &mut Gd<Node>) -> PinnedTestTask {
    let target = Node::new_alloc();
    parent.add_child(&target);
    Box::pin(async move {
        for mode in [ProcessMode::Idle, ProcessMode::Physics] {
            for call in [Callable::invalid(), target.callable("missing_method_validity_target")] {
                let mut tween = method(call, mode);
                let counts = events(&mut tween);
                frames(8).await;
                assert_cancelled(&mut tween, &counts);
            }
        }
    })
}

fn invalid_force_complete(_: &mut Gd<Node>) -> PinnedTestTask {
    Box::pin(async move {
        for mode in [ProcessMode::Idle, ProcessMode::Physics, ProcessMode::Manual] {
            for paused in [false, true] {
                let target = Object::new_alloc();
                let mut tween = method(target.callable("get_instance_id"), mode);
                tween.set_delay(100.0);
                tween.set_loops(-1, LoopMode::Restart);
                let counts = events(&mut tween);
                let weak = tween.downgrade();
                if paused { tween.pause(); }
                target.free();
                tween.force_complete();
                assert!(tween.is_stopped());
                assert_eq!(*counts, [0, 0]);
                frames(8).await;
                assert!(tween.is_registered(), "forced stopping must preserve externally retained registration");
                tween.force_complete();
                assert_eq!(*counts, [0, 0], "repeated forced stopping must stay silent");
                drop(tween);
                frames(8).await;
                assert!(weak.is_expired(), "manager must release the stopped tween after its owner drops it");
            }
        }
    })
}

fn invalid_manual_processing(_: &mut Gd<Node>) -> PinnedTestTask {
    Box::pin(async move {
        let mut tween = method(Callable::invalid(), ProcessMode::Manual);
        let counts = events(&mut tween);
        assert!(matches!(tween.custom_step(0.0, false),
            TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
        assert!(tween.is_stopped());
        frames(8).await;
        assert!(tween.is_registered(), "manual stepping reports an outcome; it does not own registration");
        // The manual caller chooses to consume ShouldDespawn by unregistering.
        tween.unregister();
        assert_cancelled(&mut tween, &counts);
        assert!(matches!(tween.custom_step(1.0, false), TweenStepResult::AlreadyStopped));
        tween.force_complete();
        assert_eq!(*counts, [0, 0]);
        frames(4).await;
    })
}

fn invalid_builder(_: &mut Gd<Node>) -> PinnedTestTask {
    let counts = RcPtr::new(0usize);
    let finished = counts.clone();
    let mut tween = TweenMethod::<f64>::new(Callable::invalid(), 0.0, 1.0, 100.0);
    tween.finished_connect(move || *finished.to_mut() += 1, SpireFlags::empty());
    let before = TM.bookkeeping_snapshot();
    tween.force_complete();
    assert!(tween.is_stopped());
    assert_eq!(*counts, 0, "unregistered builders must also cancel silently");
    assert_eq!(TM.bookkeeping_snapshot(), before, "raw builder validation must not enqueue manager operations");
    Box::pin(async { frames(4).await })
}

fn invalid_custom(_: &mut Gd<Node>) -> PinnedTestTask {
    for forced in [false, true] {
        let counts = RcPtr::new([0usize, 0]);
        let lerped = counts.clone();
        let lerper = Callable::from_fn("method_validity_lerper", move |_| {
            lerped.to_mut()[0] += 1;
            1.0.to_variant()
        });
        let mut tween = TweenMethod::<Variant>::new_custom(
            Callable::invalid(), 0.0.to_variant(), 1.0.to_variant(), 100.0, lerper,
        ).with_process_mode(ProcessMode::Manual).register();
        let finished = counts.clone();
        tween.to_mut().finished_connect(move || finished.to_mut()[1] += 1, SpireFlags::empty());
        if forced {
            tween.force_complete();
        } else {
            assert!(matches!(tween.custom_step(0.25, false),
                TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
        }
        let state = (tween.is_stopped(), tween.is_registered(), *counts);
        tween.unregister();
        assert_eq!(state, (true, true, [0, 0]), "invalid targets must stop silently without choosing registration policy");
    }
    Box::pin(async { frames(4).await })
}

fn valid_completion(_: &mut Gd<Node>) -> PinnedTestTask {
    let samples = RcPtr::new(Vec::<f64>::new());
    let sampled = samples.clone();
    let call = Callable::from_fn("method_validity_sample", move |args| {
        sampled.to_mut().push(args[0].to::<f64>());
        Variant::nil()
    });
    let mut tween = TweenMethod::<f64>::new(call, 0.0, 2.0, 0.5)
        .with_delay(0.2).with_ease(EaseKind::Basic(Ease::Linear))
        .with_process_mode(ProcessMode::Manual).register();
    let counts = events(&mut tween);
    let _ = tween.custom_step(0.1, false);
    assert!(samples.is_empty());
    let _ = tween.custom_step(0.35, false);
    assert_eq!(samples.len(), 1);
    assert!(tween.is_playing());
    let result = tween.custom_step(0.3, false);
    let completed = matches!(result, TweenStepResult::Stepped(AdvanceTimeResult::Completed { .. }));
    tween.unregister();
    assert!(completed && tween.is_stopped());
    assert_eq!(samples.last().copied(), Some(2.0));
    assert_eq!(*counts, [1, 1], "successful completion must retain both events");
    Box::pin(async { frames(4).await })
}

fn valid_force_complete(_: &mut Gd<Node>) -> PinnedTestTask {
    let samples = RcPtr::new(Vec::<f64>::new());
    let sampled = samples.clone();
    let call = Callable::from_fn("method_validity_force_sample", move |args| {
        sampled.to_mut().push(args[0].to::<f64>());
        Variant::nil()
    });
    let mut tween = method(call, ProcessMode::Manual);
    tween.set_delay(100.0);
    tween.set_loops(-1, LoopMode::Restart);
    let counts = events(&mut tween);
    tween.force_complete();
    assert_eq!(*samples, vec![2.0]);
    assert_eq!(*counts, [1, 0]);
    assert!(tween.is_stopped());
    tween.play();
    tween.pause();
    tween.force_complete();
    tween.unregister();
    assert_eq!(*samples, vec![2.0, 2.0]);
    assert_eq!(*counts, [2, 0], "valid paused replay must still finish");
    Box::pin(async { frames(4).await })
}

fn sequence_child(_: &mut Gd<Node>) -> PinnedTestTask {
    let mut child = method(Callable::invalid(), ProcessMode::Manual);
    let counts = events(&mut child);
    let calls = RcPtr::new(0usize);
    let called = calls.clone();
    let mut sequence = TweenSequence::new().with_process_mode(ProcessMode::Manual);
    sequence.append_ptr(child.clone());
    sequence.append_fn(move || *called.to_mut() += 1);
    let mut parent = sequence.register();
    let _ = parent.custom_step(1.0, false);
    parent.unregister();
    assert_cancelled(&mut child, &counts);
    assert_eq!(*calls, 1, "sequence must continue past the invalid child");
    assert!(parent.is_stopped());
    assert!(!parent.to_mut().remove(&child), "invalid child must leave the sequence");
    Box::pin(async { frames(4).await })
}

pub(super) async fn run_method_validity_tests(parent: &mut Gd<Node>) -> usize {
    let mut failures = 0;
    for &(name, test) in CASES {
        let mut fixture = Node::new_alloc();
        parent.add_child(&fixture);
        godot_print!("NATIVE_METHOD_VALIDITY_START {name}");
        let task = catch_unwind(AssertUnwindSafe(|| test(&mut fixture)));
        let result = match task {
            Ok(task) => CatchTestPanic(task).await,
            Err(error) => Err(error),
        };
        if result.is_ok() { godot_print!("NATIVE_METHOD_VALIDITY_PASS {name}"); }
        else {
            failures += 1;
            godot_print!("NATIVE_METHOD_VALIDITY_FAIL {name}");
        }
        fixture.queue_free();
        frames(4).await;
    }
    godot_print!("NATIVE_METHOD_VALIDITY_RESULT total={} failed={failures}", CASES.len());
    failures
}

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct MethodValidityTestRunner { base: Base<Node> }

#[godot_api]
impl INode for MethodValidityTestRunner {
    fn ready(&mut self) {
        let mut parent = self.base().clone();
        godot::task::spawn(async move {
            let failed = run_method_validity_tests(&mut parent).await;
            parent.get_tree().quit_ex().exit_code(if failed == 0 { 0 } else { 1 }).done();
        });
    }
}
