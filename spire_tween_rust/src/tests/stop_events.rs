//! Stop transitions and cleanup callbacks exercised inside a real SceneTree.
use godot::{classes::Node2D, prelude::*};
use spire_tween::prelude::{TweenState as State, *};

use super::{ownership::frames, util::*};
use crate::impl_test_base;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct StopEventsTests {
    base: Base<Node2D>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, StopEventsTests }

impl ITestClass for StopEventsTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/stop_events.tscn";

    fn test_list() -> Vec<NamedTest<Self>> {
        vec![
            ("test_stop_transitions_and_pause_after_stop", Self::transitions),
            ("test_stop_completion_order_and_loops", Self::loops),
            ("test_stop_forced_completion_and_replay", Self::forced),
            ("test_stop_invalid_callable_cancellation", Self::invalid_callables),
            ("test_stop_manager_completion_and_cancellation", Self::automatic),
            ("test_stop_dead_bindings_in_all_states", Self::dead_bindings),
            ("test_stop_failed_property_force_complete", Self::dead_property),
            ("test_stop_erased_connections_disconnect_and_clear", Self::connections),
            ("test_stop_deferred_and_one_shot_connections", Self::flags),
            ("test_stop_cleanup_before_godot_finished", Self::godot_cleanup),
            ("test_stop_nested_sequence_cancellation_and_replay", Self::nested),
            ("test_stop_sequence_adoption_rearms_children", Self::adoption),
            ("test_stop_unregister_and_drop_do_not_emit", Self::unregister_and_drop),
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

fn tween(kind: usize, node: &Gd<Node2D>) -> AnyTween {
    match kind {
        0 => {
            let mut seq = TweenSequence::new();
            seq.append_interval(1.0);
            TweenPtr::new(seq).into()
        }
        1 => TweenPtr::new(node.do_position(Vector2::ONE, 1.0)).into(),
        2 => TweenPtr::new(TweenMethod::<f64>::new(Callable::from_fn("sample", |_| {}), 0.0, 1.0, 1.0)).into(),
        3 => TweenPtr::new(TweenDelayedCall::new(Callable::from_fn("delayed", |_| {}), 1.0)).into(),
        _ => unreachable!(),
    }
}

fn events(tween: &mut AnyTween) -> RcPtr<Vec<&'static str>> {
    let events = RcPtr::new(Vec::new());
    let stop_events = events.clone();
    let weak = tween.downgrade();
    tween.stopped_connect(
        move || {
            assert!(weak.upgrade().unwrap().is_stopped(), "state must be stopped during cleanup");
            stop_events.to_mut().push("stopped");
        },
        SpireFlags::empty(),
    );
    let loop_events = events.clone();
    tween.loop_finished_connect(move || loop_events.to_mut().push("loop"), SpireFlags::empty());
    let finish_events = events.clone();
    tween.finished_connect(move || finish_events.to_mut().push("finished"), SpireFlags::empty());
    events
}

impl StopEventsTests {
    fn transitions(&mut self) -> PinnedTestTask {
        for kind in 0..4 {
            let mut tween = tween(kind, &self.base().clone());
            let events = events(&mut tween);
            let _ = tween.custom_step(0.25, false);
            tween.pause();
            tween.pause();
            assert!(events.is_empty());
            tween.stop();
            tween.stop();
            tween.pause();
            tween.set_state(State::Paused);
            tween.force_complete();
            assert!(tween.is_stopped());
            assert_eq!(&**events, &["stopped"]);
            tween.play();
            assert_eq!(tween.get_total_elapsed_time(), 0.0, "replay must reset the clock after stop/pause");
            assert!(matches!(tween.custom_step(0.25, false), TweenStepResult::Stepped(AdvanceTimeResult::Playing)));
            assert_eq!(&**events, &["stopped"], "replay must not finish early");
            tween.set_state(State::Stopped);
            assert_eq!(&**events, &["stopped", "stopped"]);
        }
        Box::pin(async {})
    }

    fn loops(&mut self) -> PinnedTestTask {
        for kind in 0..4 {
            let mut tween = tween(kind, &self.base().clone());
            tween.set_loops(2, LoopMode::Restart);
            let events = events(&mut tween);
            assert!(matches!(tween.custom_step(1.25, false), TweenStepResult::Stepped(AdvanceTimeResult::Playing)));
            assert_eq!(&**events, &["loop"], "an intermediate loop must not stop");
            assert!(matches!(
                tween.custom_step(1.25, false),
                TweenStepResult::Stepped(AdvanceTimeResult::Completed { .. })
            ));
            assert_eq!(&**events, &["loop", "loop", "stopped", "finished"]);
            let _ = tween.custom_step(10.0, false);
            tween.stop();
            assert_eq!(events.len(), 4);
        }
        Box::pin(async {})
    }

    fn forced(&mut self) -> PinnedTestTask {
        for kind in 0..4 {
            for paused in [false, true] {
                let mut tween = tween(kind, &self.base().clone());
                tween.set_loops(-1, LoopMode::Restart);
                let events = events(&mut tween);
                if paused {
                    tween.pause();
                }
                tween.force_complete();
                tween.force_complete();
                assert_eq!(&**events, &["stopped", "finished"]);
                tween.play();
                tween.force_complete();
                assert_eq!(&**events, &["stopped", "finished", "stopped", "finished"]);
            }
        }
        Box::pin(async {})
    }

    fn invalid_callables(&mut self) -> PinnedTestTask {
        for kind in 0..2 {
            for forced in [false, true] {
                for paused in [false, true] {
                    let mut tween: AnyTween = if kind == 0 {
                        TweenPtr::new(TweenDelayedCall::new(Callable::invalid(), 1.0)).into()
                    } else {
                        TweenPtr::new(TweenMethod::<f64>::new(Callable::invalid(), 0.0, 1.0, 1.0)).into()
                    };
                    let events = events(&mut tween);
                    if paused {
                        tween.pause();
                    }
                    if forced {
                        tween.force_complete();
                    } else {
                        if paused {
                            tween.play();
                        }
                        assert!(matches!(
                            tween.custom_step(0.0, false),
                            TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)
                        ));
                    }
                    tween.stop();
                    tween.force_complete();
                    assert_eq!(&**events, &["stopped"], "cancellation must never report successful completion");
                }
            }
        }
        Box::pin(async {})
    }

    fn automatic(&mut self) -> PinnedTestTask {
        Box::pin(async {
            for mode in [ProcessMode::Idle, ProcessMode::Physics] {
                let mut complete: AnyTween = TweenDelayedCall::new(Callable::from_fn("complete", |_| {}), 0.0)
                    .with_process_mode(mode)
                    .register()
                    .into();
                let completed = events(&mut complete);
                let mut invalid: AnyTween = TweenMethod::<f64>::new(Callable::invalid(), 0.0, 1.0, 100.0)
                    .with_process_mode(mode)
                    .register()
                    .into();
                let cancelled = events(&mut invalid);
                frames(8).await;
                assert_eq!(&**completed, &["loop", "stopped", "finished"]);
                assert_eq!(&**cancelled, &["stopped"]);
                assert!(!invalid.is_registered(), "automatic cancellation must consume ShouldDespawn");
                complete.unregister();
            }
        })
    }

    fn dead_bindings(&mut self) -> PinnedTestTask {
        for kind in 0..4 {
            for state in [State::Playing, State::Paused, State::Stopped] {
                let target = Node2D::new_alloc();
                self.base_mut().add_child(&target);
                let mut tween = tween(kind, &target);
                tween.bind_node(target.clone().upcast());
                let events = events(&mut tween);
                tween.set_state(state);
                target.free();
                assert!(matches!(tween.custom_step(0.0, false), TweenStepResult::NoLongerValid));
                assert!(tween.is_stopped());
                assert_eq!(&**events, &["stopped"], "invalidation must stop once, even for an already-stopped tween");
            }
        }
        Box::pin(async {})
    }

    fn dead_property(&mut self) -> PinnedTestTask {
        for paused in [false, true] {
            let node = Node2D::new_alloc();
            let mut tween: AnyTween = TweenPtr::new(node.do_position(Vector2::ONE, 1.0)).into();
            let events = events(&mut tween);
            if paused {
                tween.pause();
            }
            node.free();
            tween.force_complete();
            assert_eq!(&**events, &["stopped"], "failed property completion must cancel without finished");
        }
        Box::pin(async {})
    }

    fn connections(&mut self) -> PinnedTestTask {
        macro_rules! check_connections {
            ($tween:expr) => {{
                let mut tween = $tween;
                let count = RcPtr::new(0);
                let captured = count.clone();
                let call = Callable::from_fn("stop_connection", move |_| *captured.to_mut() += 1);
                tween.stopped_connect_callable(call.clone(), SpireFlags::empty());
                tween.stopped_disconnect_callable(&call);
                tween.stop();
                assert_eq!(*count, 0);
                tween.play();
                tween.stopped_connect_callable(call, SpireFlags::empty());
                tween.stopped_clear_connections();
                tween.stop();
                assert_eq!(*count, 0);
                tween.play();
                let captured = count.clone();
                tween.stopped_connect(move || *captured.to_mut() += 1, SpireFlags::empty());
                tween.stop();
                assert_eq!(*count, 1);
            }};
        }
        check_connections!(tween(0, &self.base().clone()));
        check_connections!(AnyMethodTween::from(TweenPtr::new(TweenMethod::<f64>::new(
            Callable::from_fn("sample", |_| {}),
            0.0,
            1.0,
            1.0
        ))));
        check_connections!(AnyPropertyTween::from(TweenPtr::new(self.base().clone().do_position(Vector2::ONE, 1.0))));
        Box::pin(async {})
    }

    fn flags(&mut self) -> PinnedTestTask {
        let mut tween = tween(0, &self.base().clone());
        let counts = RcPtr::new([0; 4]);
        for (idx, flags) in [
            SpireFlags::empty(),
            SpireFlags::ONE_SHOT,
            SpireFlags::DEFERRED,
            SpireFlags::DEFERRED | SpireFlags::ONE_SHOT,
        ]
        .into_iter()
        .enumerate()
        {
            let counts = counts.clone();
            tween.stopped_connect(move || counts.to_mut()[idx] += 1, flags);
        }
        tween.stop();
        tween.stop();
        tween.play();
        tween.stop();
        assert_eq!(*counts, [2, 1, 0, 0]);
        Box::pin(async move {
            frames(4).await;
            assert_eq!(*counts, [2, 1, 2, 1]);
        })
    }

    fn godot_cleanup(&mut self) -> PinnedTestTask {
        let (mut tween, handle) = TweenSequence::new()
            .with_process_mode(ProcessMode::Manual)
            .register_with_gd_handle();
        let cleaned = RcPtr::new(false);
        let stopped = RcPtr::new(0);
        let observed = RcPtr::new(false);
        let capture = cleaned.clone();
        tween
            .to_mut()
            .stopped_connect(move || *capture.to_mut() = true, SpireFlags::empty());
        let capture = stopped.clone();
        Signal::from_object_signal(&handle, "stopped")
            .connect(&Callable::from_fn("godot_stopped", move |_| *capture.to_mut() += 1));
        let capture = observed.clone();
        Signal::from_object_signal(&handle, "finished")
            .connect(&Callable::from_fn("godot_finished", move |_| *capture.to_mut() = *cleaned));
        tween.force_complete();
        tween.stop();
        tween.unregister();
        assert_eq!(*stopped, 1);
        assert!(*observed, "Godot finished listeners must observe synchronous stop cleanup");
        Box::pin(async {})
    }

    fn nested(&mut self) -> PinnedTestTask {
        let mut child = tween(3, &self.base().clone());
        let child_events = events(&mut child);
        let mut inserted = tween(2, &self.base().clone());
        let insert_events = events(&mut inserted);
        let mut inner = TweenSequence::new();
        inner.append_ptr(child.clone());
        inner.insert_ptr(0.0, inserted.clone());
        let mut inner: AnyTween = TweenPtr::new(inner).into();
        let inner_events = events(&mut inner);
        let calls = RcPtr::new(0);
        let capture = calls.clone();
        let factories = RcPtr::new(0);
        let factory_capture = factories.clone();
        let mut parent = TweenSequence::new();
        parent.append_ptr(inner);
        parent.append_fn(move || *capture.to_mut() += 1);
        parent.append_ctor(move || {
            *factory_capture.to_mut() += 1;
            TweenPtr::new(TweenSequence::new()).into()
        });
        let mut parent: AnyTween = TweenPtr::new(parent).into();
        let parent_events = events(&mut parent);
        parent.stop();
        parent.stop();
        for events in [&child_events, &insert_events, &inner_events, &parent_events] {
            assert_eq!(&***events, &["stopped"]);
        }
        assert_eq!(*factories, 0, "stopping must not invoke pending factories");
        parent.play();
        let _ = parent.custom_step(1.25, false);
        assert!(parent.is_stopped());
        assert_eq!(*calls, 1);
        assert_eq!(*factories, 1);
        assert_eq!(&**parent_events, &["stopped", "loop", "stopped", "finished"]);
        parent.play();
        parent.force_complete();
        assert_eq!(*calls, 2, "sequence callbacks must replay after stop bookkeeping");
        assert_eq!(*factories, 1, "accepted factories must remain one-shot across replay");
        Box::pin(async {})
    }

    fn adoption(&mut self) -> PinnedTestTask {
        for kind in 0..4 {
            for child_state in [State::Paused, State::Stopped] {
                for parent_state in [State::Playing, State::Paused, State::Stopped] {
                    let mut child = tween(kind, &self.base().clone());
                    child.set_state(child_state);
                    let events = events(&mut child);
                    let mut parent = TweenSequence::new();
                    parent.set_state(parent_state);
                    parent.append_ptr(child.clone());
                    if parent_state == State::Stopped {
                        parent.play();
                    }
                    assert!(child.is_playing(), "adoption/replay must arm the child");
                    parent.force_complete();
                    assert_eq!(&events[events.len() - 2..], &["stopped", "finished"]);
                }
            }
        }
        Box::pin(async {})
    }

    fn unregister_and_drop(&mut self) -> PinnedTestTask {
        let mut tween = TweenSequence::new().with_process_mode(ProcessMode::Manual).register();
        let weak = tween.downgrade();
        let count = RcPtr::new(0);
        let capture = count.clone();
        tween
            .to_mut()
            .stopped_connect(move || *capture.to_mut() += 1, SpireFlags::empty());
        tween.unregister();
        assert!(tween.is_playing());
        drop(tween);
        assert!(weak.is_expired());
        assert_eq!(*count, 0, "unregistering and dropping do not transition playback state");
        Box::pin(async {})
    }
}
