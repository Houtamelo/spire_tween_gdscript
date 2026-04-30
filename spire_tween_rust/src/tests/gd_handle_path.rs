//! Tests that explicitly exercise the Gd-handle path of `register_with_gd_handle()`.
//!
//! Most tests in the suite use `register()` (closure-based events, no Gd handle).
//! This file verifies the alternate path: Godot signal emission via the attached
//! `Gd<Spire...>` handle, used by GDScript code or any Godot signal listener.

use godot::classes::CollisionShape2D;
use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct GdHandlePathTests {
    base: Base<Node2D>,
    #[init(node = "BouncyBall/CollisionShape2D")]
    shape: OnReady<Gd<CollisionShape2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, GdHandlePathTests }

impl ITestClass for GdHandlePathTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/gd_handle_path.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_gd_handle_attached,
            Self::test_godot_signal_finished_fires,
            Self::test_godot_signal_loop_finished_fires,
            Self::test_is_registered_via_gd_handle,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl GdHandlePathTests {
    /// `register_with_gd_handle()` must populate `gd_handle`; `register()` must not.
    fn test_gd_handle_attached(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let final_value = Vector2::ONE * 2.0;

        let with_handle = shape.do_scale(final_value, 1.0).register_with_gd_handle();
        assert!(
            with_handle.gd_handle.is_some(),
            "register_with_gd_handle() must attach a Gd handle"
        );

        let without_handle = shape.do_scale(Vector2::ONE, 1.0).register();
        assert!(
            without_handle.gd_handle.is_none(),
            "register() must NOT attach a Gd handle"
        );

        // Stop both so they don't interfere with subsequent tests.
        with_handle.to_mut().stop();
        without_handle.to_mut().stop();

        Box::pin(async move {})
    }

    /// Godot's `finished` signal must fire when a `register_with_gd_handle()` tween
    /// finishes. We connect via `Signal::from_object_signal` (the GDScript-equivalent path).
    fn test_godot_signal_finished_fires(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let final_value = Vector2::ONE * 2.0;

        let handle = shape.do_scale(final_value, 1.0).register_with_gd_handle();
        let gd = handle.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);

        // Capture the timer at Godot-signal emission time.
        let timer_at_godot_emit = RcPtr::new(None::<f64>);
        let slot = timer_at_godot_emit.clone();
        let tracker_for_slot = tracker.clone();
        Signal::from_object_signal(&gd, "finished").connect_flags(
            &Callable::from_fn("godot_finished_capture", move |_| {
                *slot.to_mut() = Some(tracker_for_slot.timer);
                Variant::nil()
            }),
            godot::classes::object::ConnectFlags::ONE_SHOT,
        );

        Box::pin(async move {
            wait_finished(&handle, &tracker, 1.0).await;
            let captured = timer_at_godot_emit
                .to_mut()
                .take()
                .expect("Godot `finished` signal must fire on a tween registered with Gd handle");
            assert_within_tolerance(captured - 1.0, TIME_TOLERANCE);
            assert_eq!(shape.get_scale(), final_value);
        })
    }

    /// Same as above but for the `loop_finished` signal.
    fn test_godot_signal_loop_finished_fires(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let final_value = Vector2::ONE * 2.0;

        let mut tween = shape.do_scale(final_value, 1.0);
        tween.set_loops(2, LoopMode::Yoyo);
        let handle = tween.register_with_gd_handle();
        let gd = handle.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);

        let loop_emissions = RcPtr::new(0u32);
        let slot = loop_emissions.clone();
        Signal::from_object_signal(&gd, "loop_finished").connect(
            &Callable::from_fn("godot_loop_finished_capture", move |_| {
                *slot.to_mut() += 1;
                Variant::nil()
            }),
        );

        Box::pin(async move {
            wait_finished(&handle, &tracker, 2.0).await;
            assert_eq!(
                *loop_emissions, 2,
                "Godot `loop_finished` signal must fire twice for a 2-loop tween"
            );
        })
    }

    /// `is_registered()` on the Gd handle must reflect TweenManager state.
    fn test_is_registered_via_gd_handle(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let final_value = Vector2::ONE * 2.0;

        let handle = shape.do_scale(final_value, 2.0).register_with_gd_handle();
        let gd = handle.gd_handle.as_ref().unwrap().clone();

        assert!(gd.bind().is_registered(), "Newly registered tween must report is_registered() == true");

        gd.bind().unregister();
        assert!(!gd.bind().is_registered(), "After unregister(), is_registered() must be false");

        gd.bind().register();
        assert!(gd.bind().is_registered(), "After re-register(), is_registered() must be true again");

        handle.to_mut().stop();

        Box::pin(async move {})
    }
}
