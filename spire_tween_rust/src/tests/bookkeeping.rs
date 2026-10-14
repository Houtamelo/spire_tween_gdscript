//! Native bookkeeping regressions driven by real Godot deletion and manager ticks.
use std::panic::{AssertUnwindSafe, catch_unwind};

use godot::prelude::*;
use spire_tween::{
    prelude::*,
    unversioned_api::{BookkeepingSnapshot, NodeStatus, TM},
};

use super::{ownership::{CatchTestPanic, frames}, util::PinnedTestTask};

type TestFn = fn(&mut BookkeepingTests) -> PinnedTestTask;
const CASES: &[(&str, TestFn)] = &[
    ("free_inside_tree_batches_plateau", BookkeepingTests::free_inside),
    ("free_outside_tree_batches_plateau", BookkeepingTests::free_outside),
    ("queue_free_inside_tree_batches_plateau", BookkeepingTests::queue_inside),
    ("queue_free_outside_tree_batches_plateau", BookkeepingTests::queue_outside),
    ("dead_records_wait_for_both_loops", BookkeepingTests::both_loops),
    ("expired_bindings_on_living_node_plateau", BookkeepingTests::expired_inside),
    ("expired_bindings_on_detached_node_plateau", BookkeepingTests::expired_outside),
    ("retained_bindings_survive_pruning", BookkeepingTests::retained_bindings),
    ("detached_node_reentry_preserves_bindings", BookkeepingTests::reentry),
    ("deletion_cancels_all_clock_and_playback_modes", BookkeepingTests::clock_modes),
    ("duplicating_nodes_does_not_delay_source_cleanup", BookkeepingTests::duplicated_nodes),
];

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct BookkeepingTests {
    base: Base<Node>,
}

fn sequence() -> TweenSequence {
    let mut tween = TweenSequence::new();
    tween.append_interval(10.0);
    tween
}

fn snapshot(label: &str) -> BookkeepingSnapshot {
    let state = TM.bookkeeping_snapshot();
    godot_print!("BOOKKEEPING_SNAPSHOT {label} {state:?}");
    state
}

async fn node_batches(mut parent: Gd<Node>, inside: bool, queued: bool) {
    frames(8).await;
    let baseline = snapshot("node-batches/baseline");
    for round in 1..=3 {
        for _ in 0..1000 {
            let target = Node::new_alloc();
            if inside { parent.add_child(&target); }
            let tween = sequence().bound_to(target.clone()).register();
            if queued { target.clone().queue_free(); } else { target.free(); }
            drop(tween);
        }
        frames(8).await;
        assert_eq!(snapshot(&format!("node-batches/round-{round}")), baseline,
            "node records, weak bindings, roots, and pending operations must return to baseline");
    }
}

async fn expired_batches(mut parent: Gd<Node>, inside: bool) {
    frames(8).await;
    let target = Node::new_alloc();
    if inside { parent.add_child(&target); }
    TM.node_get_status_fresh(target.clone());
    let baseline = snapshot("expired-bindings/baseline");
    let mut rounds = Vec::new();
    for round in 1..=3 {
        for _ in 0..1000 {
            let mut tween = sequence().bound_to(target.clone()).register();
            tween.force_complete();
            drop(tween);
        }
        frames(8).await;
        rounds.push(snapshot(&format!("expired-bindings/round-{round}")));
    }
    // Clean up before asserting so failed cases do not contaminate later tests.
    target.free();
    frames(8).await;
    for state in rounds {
        assert_eq!(state.binding_entries, baseline.binding_entries,
            "expired bindings must be pruned while their bound node remains alive");
        assert_eq!(state.expired_bindings, baseline.expired_bindings);
        assert_eq!(state.tracked_nodes, baseline.tracked_nodes);
        assert_eq!(state.root_tweens, baseline.root_tweens);
        assert_eq!(state.deferred_ops, baseline.deferred_ops);
    }
}

/// Restore the real controller's tick flags even if a test assertion unwinds.
struct ControllerFlags {
    node: Gd<Node>,
    process: bool,
    physics: bool,
}

impl ControllerFlags {
    fn new(parent: &Gd<Node>) -> Self {
        let controller = parent.get_tree().get_root().unwrap().get_node_as::<Node>("tweens_controller");
        Self { process: controller.is_processing(), physics: controller.is_physics_processing(), node: controller }
    }
}

impl Drop for ControllerFlags {
    fn drop(&mut self) {
        self.node.set_process(self.process);
        self.node.set_physics_process(self.physics);
    }
}

impl BookkeepingTests {
    fn free_inside(&mut self) -> PinnedTestTask {
        let parent = self.base().clone();
        Box::pin(node_batches(parent, true, false))
    }
    fn free_outside(&mut self) -> PinnedTestTask {
        let parent = self.base().clone();
        Box::pin(node_batches(parent, false, false))
    }
    fn queue_inside(&mut self) -> PinnedTestTask {
        let parent = self.base().clone();
        Box::pin(node_batches(parent, true, true))
    }
    fn queue_outside(&mut self) -> PinnedTestTask {
        let parent = self.base().clone();
        Box::pin(node_batches(parent, false, true))
    }
    fn expired_inside(&mut self) -> PinnedTestTask {
        let parent = self.base().clone();
        Box::pin(expired_batches(parent, true))
    }
    fn expired_outside(&mut self) -> PinnedTestTask {
        let parent = self.base().clone();
        Box::pin(expired_batches(parent, false))
    }

    fn both_loops(&mut self) -> PinnedTestTask {
        let mut parent = self.base().clone();
        Box::pin(async move {
            frames(8).await;
            let baseline = snapshot("both-loops/baseline");
            let mut flags = ControllerFlags::new(&parent);
            for disable_physics in [true, false] {
                flags.node.set_process(disable_physics);
                flags.node.set_physics_process(!disable_physics);
                let target = Node::new_alloc();
                parent.add_child(&target);
                let id = target.instance_id().to_i64();
                TM.node_get_status_fresh(target.clone());
                target.free();
                assert_eq!(TM.bookkeeping_node_status(id), Some(NodeStatus::Dead {
                    seen_by_process: false, seen_by_physics: false,
                }));
                frames(8).await;
                assert_eq!(TM.bookkeeping_node_status(id), Some(NodeStatus::Dead {
                    seen_by_process: disable_physics, seen_by_physics: !disable_physics,
                }), "one loop must not prune a record before the other sees it");
                flags.node.set_process(true);
                flags.node.set_physics_process(true);
                frames(8).await;
                assert_eq!(TM.bookkeeping_node_status(id), None);
                assert_eq!(snapshot("both-loops/settled"), baseline);
            }
        })
    }

    fn retained_bindings(&mut self) -> PinnedTestTask {
        let mut parent = self.base().clone();
        Box::pin(async move {
            frames(8).await;
            let target = Node::new_alloc();
            parent.add_child(&target);
            TM.node_get_status_fresh(target.clone());
            let baseline = snapshot("retained/baseline");
            let mut tween = sequence().bound_to(target.clone())
                .with_process_mode(ProcessMode::Manual).register();
            let weak = tween.downgrade();
            tween.force_complete();
            frames(8).await;
            assert!(tween.is_registered(), "retained completed handle must keep its registration");
            assert_eq!(snapshot("retained/completed").binding_entries, baseline.binding_entries + 1);
            tween.unregister();
            frames(8).await;
            assert!(weak.upgrade().is_some());
            let live = snapshot("retained/unregistered");
            assert_eq!(live.binding_entries, baseline.binding_entries + 1);
            assert_eq!(live.expired_bindings, baseline.expired_bindings);
            tween.play();
            TM.node_bound_tweens_kill(target.clone());
            assert!(tween.is_stopped(), "unregistered live bindings must still support node actions");
            tween.play();
            tween.re_register();
            frames(8).await;
            assert_eq!(snapshot("retained/reregistered").binding_entries, baseline.binding_entries + 1);
            target.free();
            frames(8).await;
            assert!(tween.is_stopped() && !tween.is_registered());
            assert_eq!(TM.bookkeeping_node_status(target_id(&tween)), None);
        })
    }

    fn reentry(&mut self) -> PinnedTestTask {
        let mut parent = self.base().clone();
        Box::pin(async move {
            frames(8).await;
            let target = Node::new_alloc();
            parent.add_child(&target);
            let id = target.instance_id().to_i64();
            let tween = sequence().bound_to(target.clone()).register();
            frames(8).await;
            parent.remove_child(&target);
            let elapsed = tween.get_total_elapsed_time();
            frames(8).await;
            assert_eq!(TM.bookkeeping_node_status(id), Some(NodeStatus::OutsideTree));
            assert!(tween.is_registered() && tween.is_playing());
            assert_eq!(tween.get_total_elapsed_time(), elapsed, "detached binding must block advancement");
            parent.add_child(&target);
            frames(8).await;
            assert_eq!(TM.bookkeeping_node_status(id), Some(NodeStatus::InsideTree));
            assert!(tween.get_total_elapsed_time() > elapsed, "re-entered binding must resume playback");
            target.free();
            frames(8).await;
            assert!(tween.is_stopped() && !tween.is_registered());
            assert_eq!(TM.bookkeeping_node_status(id), None);
        })
    }

    fn clock_modes(&mut self) -> PinnedTestTask {
        let mut parent = self.base().clone();
        Box::pin(async move {
            frames(8).await;
            for mode in [ProcessMode::Idle, ProcessMode::Physics, ProcessMode::Manual] {
                for state in [TweenState::Playing, TweenState::Paused, TweenState::Stopped] {
                    let target = Node::new_alloc();
                    parent.add_child(&target);
                    let id = target.instance_id().to_i64();
                    let mut tween = sequence().bound_to(target.clone()).with_process_mode(mode).register();
                    tween.set_state(state);
                    frames(4).await;
                    target.free();
                    assert!(tween.is_stopped() && !tween.is_registered(),
                        "deletion notification must cancel every clock/state combination");
                    frames(8).await;
                    assert_eq!(TM.bookkeeping_node_status(id), None);
                }
            }
        })
    }

    fn duplicated_nodes(&mut self) -> PinnedTestTask {
        let mut parent = self.base().clone();
        Box::pin(async move {
            frames(8).await;
            let baseline = snapshot("duplicate/baseline");
            for inside in [true, false] {
                for duplicate_first in [true, false] {
                    let source = Node::new_alloc();
                    if inside { parent.add_child(&source); }
                    let id = source.instance_id().to_i64();
                    let tween = sequence().bound_to(source.clone()).with_process_mode(ProcessMode::Manual).register();
                    frames(8).await;
                    let duplicate = source.duplicate_node();
                    let stores_metadata = source.has_meta("__tween_manager_lifetime_tracker")
                        || duplicate.has_meta("__tween_manager_lifetime_tracker");
                    if duplicate_first {
                        duplicate.clone().free();
                        frames(8).await;
                        assert!(tween.is_playing() && tween.is_registered(),
                            "freeing a duplicate must not cancel the source node's tweens");
                        assert_eq!(TM.bookkeeping_node_status(id), Some(if inside {
                            NodeStatus::InsideTree
                        } else {
                            NodeStatus::OutsideTree
                        }));
                    }
                    source.free();
                    let cancelled = tween.is_stopped() && !tween.is_registered();
                    frames(8).await;
                    let source_record = TM.bookkeeping_node_status(id);
                    let after_source_free = snapshot("duplicate/source-freed");
                    if !duplicate_first { duplicate.free(); }
                    frames(8).await;
                    assert_eq!(snapshot("duplicate/both-freed"), baseline);
                    assert!(!stores_metadata, "the tracker must be owned by its connection, not copied metadata");
                    assert!(cancelled, "duplication must not delay cancellation when the original node is freed");
                    assert_eq!(source_record, None, "the original node record must be pruned independently of its duplicate");
                    assert_eq!(after_source_free, baseline);
                }
            }
        })
    }
}

fn target_id(tween: &TweenSequencePtr) -> i64 {
    tween.get_bound_nodes().iter().next().unwrap().instance_id_unchecked().to_i64()
}

pub(super) async fn run_bookkeeping_tests(parent: &mut Gd<Node>) -> usize {
    // Access the manager only after Godot's SceneTree exists, then let its
    // deferred controller initialization and both real tick loops settle.
    let _ = TM.bookkeeping_snapshot();
    frames(8).await;
    let mut failures = 0;
    for &(name, test) in CASES {
        let mut node = BookkeepingTests::new_alloc();
        parent.add_child(&node);
        godot_print!("NATIVE_BOOKKEEPING_START {name}");
        let task = catch_unwind(AssertUnwindSafe(|| test(&mut node.bind_mut())));
        let result = match task {
            Ok(task) => CatchTestPanic(task).await,
            Err(error) => Err(error),
        };
        if result.is_ok() {
            godot_print!("NATIVE_BOOKKEEPING_PASS {name}");
        } else {
            failures += 1;
            godot_print!("NATIVE_BOOKKEEPING_FAIL {name}");
        }
        node.queue_free();
        frames(8).await;
    }
    godot_print!("NATIVE_BOOKKEEPING_RESULT total={} failed={failures}", CASES.len());
    failures
}

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct BookkeepingTestRunner {
    base: Base<Node>,
}

#[godot_api]
impl INode for BookkeepingTestRunner {
    fn ready(&mut self) {
        let mut parent = self.base().clone();
        godot::task::spawn(async move {
            let failed = run_bookkeeping_tests(&mut parent).await;
            parent.get_tree().quit_ex().exit_code(if failed == 0 { 0 } else { 1 }).done();
        });
    }
}
