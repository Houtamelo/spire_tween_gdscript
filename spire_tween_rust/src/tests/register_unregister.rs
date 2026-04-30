use godot::classes::CollisionShape2D;
use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct RegisterUnregisterTests {
    base: Base<Node2D>,
    #[init(node = "BouncyBall/CollisionShape2D")]
    shape: OnReady<Gd<CollisionShape2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, RegisterUnregisterTests }

impl ITestClass for RegisterUnregisterTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/register_unregister.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_unregistered_does_not_affect_game,
            Self::test_unregistered_can_be_manually_stepped,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl RegisterUnregisterTests {
    fn test_unregistered_does_not_affect_game(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let initial_scale = shape.get_scale();
        let final_scale = Vector2::new(3.0, 3.0);

        let handle = shape.do_scale(final_scale, 5.0).register();
        let gd = handle.gd_handle.as_ref().unwrap().clone();
        gd.bind().unregister();

        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&gd, &tracker, 7.0);

        Box::pin(async move {
            tracker.wait_seconds(2.0).await;

            assert_eq!(shape.get_scale(), initial_scale);
            assert_eq!(handle.get_total_elapsed_time(), 0.0);
            assert!(handle.is_playing());
            assert!(!gd.bind().is_registered());

            gd.bind().register();
            assert!(gd.bind().is_registered());

            tracker.wait_loop_finished(&gd, 7.0).await;
            if !handle.is_stopped() {
                tracker.wait_finished(&gd, 7.0).await;
            }

            assert_eq!(shape.get_scale(), final_scale);
            assert!(handle.is_stopped());
            assert!(gd.bind().is_registered());
        })
    }

    fn test_unregistered_can_be_manually_stepped(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let final_scale = Vector2::new(3.0, 3.0);

        let mut handle = shape.do_scale(final_scale, 5.0).register();
        let gd = handle.gd_handle.as_ref().unwrap().clone();
        gd.bind().unregister();

        Box::pin(async move {
            let mut time = godot::classes::Time::singleton().get_ticks_msec();

            while handle.is_playing() {
                next_frame().await;
                let curr_time = godot::classes::Time::singleton().get_ticks_msec();
                let delta = (curr_time - time) as f64 / 1000.0;
                time = curr_time;
                let _ = handle.process(delta, true);
            }

            assert!(handle.is_stopped());
            assert!(!gd.bind().is_registered());
            assert_eq!(shape.get_scale(), final_scale);
        })
    }
}
