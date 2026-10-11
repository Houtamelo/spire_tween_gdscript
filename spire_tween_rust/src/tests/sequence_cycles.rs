//! Sequence admission and traversal regressions inside a real Godot SceneTree.
use std::panic::{AssertUnwindSafe, catch_unwind};

use godot::prelude::*;
use spire_tween::{prelude::*, unversioned_api::TM};

use super::{
    ownership::{CatchTestPanic, frames},
    util::PinnedTestTask,
};

type TestFn = fn(&mut SequenceCycleTests) -> PinnedTestTask;
const CASES: &[(&str, TestFn)] = &[
    ("two_node_cycles", SequenceCycleTests::two_node_cycles),
    ("deeper_mixed_cycles", SequenceCycleTests::deeper_mixed_cycles),
    ("rejection_preserves_child", SequenceCycleTests::rejection_preserves_child),
    ("valid_nested_playback_and_bindings", SequenceCycleTests::valid_nested_playback_and_bindings),
    ("repeated_incoming_graph", SequenceCycleTests::repeated_incoming_graph),
    ("registration_rejects_repeated_graph", SequenceCycleTests::registration_rejects_repeated_graph),
    ("iterator_forbidden_terminates", SequenceCycleTests::iterator_forbidden_terminates),
    ("iterator_repeated_terminates", SequenceCycleTests::iterator_repeated_terminates),
    ("repeated_after_heap_spill", SequenceCycleTests::repeated_after_heap_spill),
    ("wide_and_deep_hierarchies", SequenceCycleTests::wide_and_deep_hierarchies),
];

#[derive(Clone, Copy)]
enum AddOp {
    Append,
    Join,
    Insert,
}
const OPS: [AddOp; 3] = [AddOp::Append, AddOp::Join, AddOp::Insert];

fn sequence(duration: f64) -> TweenSequence {
    let mut value = TweenSequence::new();
    value.append_interval(duration);
    value
}

fn add(parent: &TweenPtr<Sequence>, child: TweenPtr<Sequence>, operation: AddOp) {
    match operation {
        AddOp::Append => parent.to_mut().append_ptr(child),
        AddOp::Join => parent.to_mut().join_ptr(child),
        AddOp::Insert => parent.to_mut().insert_ptr(0.0, child),
    }
}

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct SequenceCycleTests {
    base: Base<Node>,
}

impl SequenceCycleTests {
    fn two_node_cycles(&mut self) -> PinnedTestTask {
        for operation in OPS {
            let a = sequence(0.03).register();
            let b = sequence(0.01).register();
            a.to_mut().append_ptr(b.clone());
            add(&b, a.clone(), operation);
            let registered = a.is_registered();
            let playing = a.is_playing();
            // Break the cycle before asserting when testing the unfixed library.
            let accepted = b.to_mut().remove(&a);
            a.to_mut().stop();
            b.to_mut().stop();
            assert!(!accepted, "two-node cycle must be rejected");
            assert!(registered && playing, "rejected child must keep its registration and state");
            a.to_mut().play();
            TM.tween_register(a.clone());
            a.to_mut().force_complete();
            assert!(a.is_stopped() && b.is_stopped());
        }
        Box::pin(async { frames(4).await })
    }

    fn deeper_mixed_cycles(&mut self) -> PinnedTestTask {
        for operation in OPS {
            let a = sequence(0.03).register();
            let b = sequence(0.02).register();
            let c = sequence(0.01).register();
            a.to_mut().append_ptr(b.clone());
            b.to_mut().join_ptr(c.clone());
            add(&c, a.clone(), operation);
            let registered = a.is_registered();
            let accepted = c.to_mut().remove(&a);
            a.to_mut().stop();
            b.to_mut().stop();
            c.to_mut().stop();
            assert!(!accepted, "indirect cycle through a third sequence must be rejected");
            assert!(registered);
            a.to_mut().play();
            TM.tween_register(a.clone());
            a.to_mut().force_complete();
            assert!(a.is_stopped() && b.is_stopped() && c.is_stopped());
        }
        Box::pin(async { frames(4).await })
    }

    fn rejection_preserves_child(&mut self) -> PinnedTestTask {
        let mut target = Node::new_alloc();
        self.base_mut().add_child(&target);
        let calls = RcPtr::new(0usize);
        let counter = calls.clone();
        let a = sequence(0.03)
            .bound_to(target.clone())
            .on_finish(move || *counter.to_mut() += 1)
            .register();
        let b = sequence(0.01).register();
        a.to_mut().append_ptr(b.clone());
        b.to_mut().set_default_ease(Ease::InSine);
        add(&b, a.clone(), AddOp::Insert);
        let registered = a.is_registered();
        let playing = a.is_playing();
        let ease = a.t.default_ease;
        let accepted = b.to_mut().remove(&a);
        // Clean up before assertions so even a baseline failure cannot leak a cycle.
        target.complete_bound_tweens();
        target.queue_free();
        assert!(!accepted);
        assert!(registered && playing);
        assert!(ease.is_none(), "rejected child must not inherit its proposed parent's ease");
        assert_eq!(a.get_bound_nodes().len(), 1);
        Box::pin(async move {
            frames(4).await;
            assert_eq!(*calls, 1, "existing manager binding must still complete exactly once");
            drop((a, b));
        })
    }

    fn valid_nested_playback_and_bindings(&mut self) -> PinnedTestTask {
        let mut target = Node2D::new_alloc();
        self.base_mut().add_child(&target);
        let calls = RcPtr::new(0usize);
        let mut children = Vec::new();
        let root = sequence(0.05).register();
        for operation in OPS {
            let counter = calls.clone();
            let child = sequence(0.01)
                .bound_to(target.clone().upcast::<Node>())
                .on_finish(move || *counter.to_mut() += 1)
                .register();
            add(&root, child.clone(), operation);
            children.push(child);
        }
        let property = target.do_position(Vector2::ONE, 0.01).register();
        children[0].to_mut().append_ptr(property.clone());
        TM.tween_register(root.clone());
        Box::pin(async move {
            frames(20).await;
            assert!(root.is_stopped());
            assert_eq!(*calls, 3);
            assert_eq!(target.get_position(), Vector2::ONE);
            assert!(
                children
                    .iter()
                    .all(|child| child.is_stopped() && !child.is_registered())
            );
            TM.tween_unregister(&root);
            frames(4).await;
            TM.tween_register(root.clone());
            root.to_mut().play();
            frames(20).await;
            assert_eq!(*calls, 6);
            assert!(root.is_stopped());
            target.queue_free();
            frames(4).await;
        })
    }

    fn repeated_candidate(&mut self) -> (TweenPtr<Sequence>, TweenPtr<Sequence>, TweenPtr<Sequence>) {
        let shared = sequence(0.01).register();
        let left = sequence(0.02).register();
        let right = sequence(0.02).register();
        left.to_mut().append_ptr(shared.clone());
        right.to_mut().append_ptr(shared.clone());
        let mut candidate = sequence(0.03);
        candidate.append_ptr(left);
        candidate.append_ptr(right.clone());
        (TweenPtr::new(candidate), right, shared)
    }

    fn repeated_incoming_graph(&mut self) -> PinnedTestTask {
        let (candidate, right, shared) = self.repeated_candidate();
        let parent = sequence(0.05).register();
        parent.to_mut().append_ptr(candidate.clone());
        let accepted = parent.to_mut().remove(&candidate);
        let playing = candidate.is_playing();
        right.to_mut().remove(&shared);
        parent.to_mut().stop();
        candidate.to_mut().stop();
        assert!(!accepted, "incoming repeated tween must reject insertion");
        assert!(playing && !candidate.is_registered());
        Box::pin(async { frames(4).await })
    }

    fn registration_rejects_repeated_graph(&mut self) -> PinnedTestTask {
        let (candidate, right, shared) = self.repeated_candidate();
        TM.tween_register(candidate.clone());
        let registered = candidate.is_registered();
        TM.tween_unregister(&candidate);
        right.to_mut().remove(&shared);
        candidate.to_mut().stop();
        assert!(!registered, "invalid graph must not be added to the manager");
        Box::pin(async { frames(4).await })
    }

    fn iterator_forbidden_terminates(&mut self) -> PinnedTestTask {
        let root = TweenPtr::new(sequence(0.01));
        let sibling = TweenPtr::new(sequence(0.01));
        // Deliberately malformed, unregistered fixture for the iterator only.
        root.to_mut()
            .t
            .queue
            .push(vec![AnyTween::from(sibling).into(), AnyTween::from(root.clone()).into()]);
        let mut iter = root.iter_inner_tweens_recursive();
        let rejected = iter.next().is_some_and(|item| item.is_err());
        let terminated = iter.next().is_none();
        root.to_mut().remove(&root);
        assert!(rejected && terminated, "forbidden parent must emit one error and terminate");
        Box::pin(async { frames(4).await })
    }

    fn iterator_repeated_terminates(&mut self) -> PinnedTestTask {
        let root = TweenPtr::new(sequence(0.01));
        let shared = TweenPtr::new(sequence(0.01));
        let sibling = TweenPtr::new(sequence(0.01));
        root.to_mut().t.queue.push(vec![
            AnyTween::from(sibling).into(),
            AnyTween::from(shared.clone()).into(),
            AnyTween::from(shared.clone()).into(),
        ]);
        let mut iter = root.iter_inner_tweens_recursive();
        assert!(iter.next().is_some_and(|item| item.is_ok()));
        assert!(iter.next().is_some_and(|item| item.is_err()));
        assert!(iter.next().is_none());
        root.to_mut().t.queue.clear();
        Box::pin(async { frames(4).await })
    }

    fn repeated_after_heap_spill(&mut self) -> PinnedTestTask {
        let root = TweenPtr::new(sequence(0.01));
        let shared = TweenPtr::new(sequence(0.01));
        let sibling = TweenPtr::new(sequence(0.01));
        root.to_mut().t.queue.push(vec![
            AnyTween::from(sibling).into(),
            AnyTween::from(shared.clone()).into(),
            AnyTween::from(shared.clone()).into(),
        ]);
        for _ in 0..24 {
            root.to_mut().append_ptr(TweenPtr::new(sequence(0.01)));
        }
        let mut iter = root.iter_inner_tweens_recursive();
        for _ in 0..25 {
            assert!(iter.next().is_some_and(|item| item.is_ok()));
        }
        assert!(iter.next().is_some_and(|item| item.is_err()));
        assert!(iter.next().is_none());
        root.to_mut().t.queue.clear();
        Box::pin(async { frames(4).await })
    }

    fn wide_and_deep_hierarchies(&mut self) -> PinnedTestTask {
        let wide = sequence(0.001).register();
        for _ in 0..24 {
            wide.to_mut().append_ptr(sequence(0.001).register());
        }
        let deep = sequence(0.001).register();
        let mut cursor = deep.clone();
        for _ in 0..32 {
            let child = sequence(0.001).register();
            cursor.to_mut().append_ptr(child.clone());
            cursor = child;
        }
        assert_eq!(wide.iter_inner_tweens_recursive().filter(|item| item.is_ok()).count(), 24);
        assert_eq!(deep.iter_inner_tweens_recursive().filter(|item| item.is_ok()).count(), 32);
        TM.tween_register(wide.clone());
        TM.tween_register(deep.clone());
        Box::pin(async move {
            frames(30).await;
            assert!(wide.is_stopped() && deep.is_stopped());
        })
    }
}

pub(super) async fn run_sequence_cycle_tests(parent: &mut Gd<Node>) -> usize {
    let mut failures = 0;
    for &(name, test) in CASES {
        let mut node = SequenceCycleTests::new_alloc();
        parent.add_child(&node);
        godot_print!("NATIVE_CYCLE_START {name}");
        let task = catch_unwind(AssertUnwindSafe(|| test(&mut node.bind_mut())));
        let result = match task {
            Ok(task) => CatchTestPanic(task).await,
            Err(error) => Err(error),
        };
        if result.is_ok() {
            godot_print!("NATIVE_CYCLE_PASS {name}");
        } else {
            failures += 1;
            godot_print!("NATIVE_CYCLE_FAIL {name}");
        }
        node.queue_free();
        frames(4).await;
    }
    godot_print!("NATIVE_CYCLE_RESULT total={} failed={failures}", CASES.len());
    failures
}

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct SequenceCycleTestRunner {
    base: Base<Node>,
}

#[godot_api]
impl INode for SequenceCycleTestRunner {
    fn ready(&mut self) {
        let mut parent = self.base().clone();
        godot::task::spawn(async move {
            let failed = run_sequence_cycle_tests(&mut parent).await;
            parent
                .get_tree()
                .quit_ex()
                .exit_code(if failed == 0 { 0 } else { 1 })
                .done();
        });
    }
}
