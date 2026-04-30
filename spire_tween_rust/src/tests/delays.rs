use godot::classes::CollisionShape2D;
use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct DelaysTests {
    base: Base<Node2D>,
    #[init(node = "BouncyBall/CollisionShape2D")]
    shape: OnReady<Gd<CollisionShape2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, DelaysTests }

impl ITestClass for DelaysTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/delays.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_delay_is_respected,
            Self::test_extra_loops_dont_have_delay,
            Self::test_sequence_respects_delay,
            Self::test_delayed_call,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl DelaysTests {
    fn test_delay_is_respected(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let initial_value = shape.get_scale();
        let final_value = Vector2::ONE * 3.0;

        let tween = shape.do_scale(final_value, 1.0).with_delay(5.0).register();
        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            // At 4.9s the delay hasn't passed yet
            tracker.wait_seconds(4.9).await;
            assert_eq!(shape.get_scale(), initial_value);
            assert_eq!(tween.get_animation_position(), 0.0);

            wait_finished(&tween, &tracker, 6.0).await;
            assert_eq!(shape.get_scale(), final_value);
        })
    }

    fn test_extra_loops_dont_have_delay(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let initial_value = shape.get_scale();
        let final_value = Vector2::ONE * 3.0;

        let mut tween = shape.do_scale(final_value, 3.0).with_delay(4.0).register();
        tween.set_loops(2, LoopMode::Yoyo);
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&tween, &tracker, 10.0);

        Box::pin(async move {
            tracker.wait_seconds(3.9).await;
            assert_eq!(shape.get_scale(), initial_value);

            wait_loop_finished(&tween, &tracker, 7.0).await;
            assert_eq!(shape.get_scale(), final_value);

            wait_loop_finished(&tween, &tracker, 10.0).await;
            assert_eq!(shape.get_scale(), initial_value);
            assert_eq!(tween.get_loops_finished(), 2);

            if !tween.is_stopped() {
                wait_finished(&tween, &tracker, 10.0).await;
            }
            assert_eq!(tween.get_loops_finished(), 2);
        })
    }

    fn test_sequence_respects_delay(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let initial_value = shape.get_scale();
        let final_value = Vector2::ONE * 4.0;

        let first = shape.do_scale(final_value, 2.0).register();
        let second = shape.do_scale(initial_value, 2.0).register();

        let mut seq = SpireTween::<Sequence>::new().with_delay(3.0);
        seq.set_loops(2, LoopMode::Restart);
        seq.append_ptr(first.clone());
        seq.append_ptr(second.clone());
        let seq = seq.register();

        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&seq, &tracker, 11.0);

        // Capture scale at each child finish (closure-based, no Gd handle).
        let shape_c = shape.clone();
        let scale_at_first_done = snapshot_on_finished(&first, move || shape_c.get_scale());
        let shape_c = shape.clone();
        let scale_at_second_done = snapshot_on_finished(&second, move || shape_c.get_scale());

        Box::pin(async move {
            tracker.wait_seconds(2.9).await;
            assert_eq!(shape.get_scale(), initial_value);

            wait_finished(&first, &tracker, 5.0).await;
            assert_eq!(scale_at_first_done.to_mut().take().unwrap(), final_value);

            wait_finished(&second, &tracker, 7.0).await;
            assert_eq!(scale_at_second_done.to_mut().take().unwrap(), initial_value);

            if seq.get_loops_finished() < 1 {
                wait_loop_finished(&seq, &tracker, 7.0).await;
            }

            assert_eq!(shape.get_scale(), initial_value);
            assert!(seq.get_total_elapsed_time() >= 7.0);
            assert!(seq.get_total_elapsed_time() <= 7.1);

            // Loop 2: re-register snapshots for the second emission
            let shape_c = shape.clone();
            let scale_at_first_done_2 = snapshot_on_finished(&first, move || shape_c.get_scale());
            let shape_c = shape.clone();
            let scale_at_second_done_2 = snapshot_on_finished(&second, move || shape_c.get_scale());

            wait_finished(&first, &tracker, 9.0).await;
            assert_eq!(scale_at_first_done_2.to_mut().take().unwrap(), final_value);

            wait_finished(&second, &tracker, 11.0).await;
            assert_eq!(scale_at_second_done_2.to_mut().take().unwrap(), initial_value);

            if !seq.is_stopped() {
                wait_finished(&seq, &tracker, 11.0).await;
            }
            assert_eq!(shape.get_scale(), initial_value);
        })
    }

    fn test_delayed_call(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let final_val = Vector2::ONE * 3.0;

        let mut shape_for_call = shape.clone();
        let tween = shape.do_delayed_call(
            move || { shape_for_call.set_scale(final_val); },
            3.0,
        ).register();
        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            wait_finished(&tween, &tracker, 3.0).await;
            assert_eq!(shape.get_scale(), final_val);
        })
    }
}
