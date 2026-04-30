use godot::classes::Sprite2D;
use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct PlayPauseStopTests {
    base: Base<Node2D>,
    #[init(node = "Ball")]
    ball: OnReady<Gd<Sprite2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, PlayPauseStopTests }

impl ITestClass for PlayPauseStopTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/play_pause_stop.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_play_works,
            Self::test_pause_play_works,
            Self::test_play_stop_play_works,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl PlayPauseStopTests {
    fn test_play_works(&mut self) -> PinnedTestTask {
        let ball = self.ball.clone();
        let final_scale = Vector2::new(30.0, 30.0);

        let mut tween = ball.do_scale(final_scale, 2.0);
        tween.set_begin_value(Vector2::ZERO);
        let handle = tween.register();

        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&handle, &tracker, 2.0);

        Box::pin(async move {
            wait_loop_finished(&handle, &tracker, 2.0).await;
            if !handle.is_stopped() {
                wait_finished(&handle, &tracker, 2.0).await;
            }

            assert_eq!(ball.get_scale(), final_scale);
            assert!(handle.is_stopped());
        })
    }

    fn test_pause_play_works(&mut self) -> PinnedTestTask {
        let ball = self.ball.clone();
        let initial_scale = ball.get_scale();
        let final_scale = Vector2::new(30.0, 30.0);

        let mut tween = ball.do_scale(final_scale, 2.0);
        tween.set_begin_value(Vector2::ZERO);
        let mut handle = tween.register();
        handle.pause();

        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&handle, &tracker, 5.0);

        Box::pin(async move {
            tracker.wait_seconds(3.0).await;

            assert!(handle.is_paused());
            assert_eq!(handle.get_total_elapsed_time(), 0.0);
            assert_eq!(handle.get_loops_finished(), 0);
            assert_eq!(ball.get_scale(), initial_scale);

            handle.play();

            wait_loop_finished(&handle, &tracker, 5.0).await;
            if !handle.is_stopped() {
                wait_finished(&handle, &tracker, 5.0).await;
            }

            assert_eq!(ball.get_scale(), final_scale);
            assert!(handle.is_stopped());

            // Verify that a stopped tween never emits events again.
            assert_upon_loop_finished(&handle, || {
                panic!("Unexpected emission of `loop_finished` on stopped tween")
            });
            assert_upon_finished(&handle, || {
                panic!("Unexpected emission of `finished` on stopped tween")
            });
        })
    }

    fn test_play_stop_play_works(&mut self) -> PinnedTestTask {
        let ball = self.ball.clone();
        let final_scale = Vector2::new(30.0, 30.0);

        let mut tween = ball.do_scale(final_scale, 2.0);
        tween.set_begin_value(Vector2::ZERO);
        let mut handle = tween.register();

        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            wait_loop_finished(&handle, &tracker, 2.0).await;
            if !handle.is_stopped() {
                wait_finished(&handle, &tracker, 2.0).await;
            }

            assert_eq!(ball.get_scale(), final_scale);
            assert!(handle.is_stopped());
            handle.stop();

            tracker.wait_seconds(3.0).await;

            handle.play();

            wait_loop_finished(&handle, &tracker, 7.0).await;
            if !handle.is_stopped() {
                wait_finished(&handle, &tracker, 7.0).await;
            }

            assert_eq!(ball.get_scale(), final_scale);
            assert!(handle.is_stopped());
        })
    }
}
