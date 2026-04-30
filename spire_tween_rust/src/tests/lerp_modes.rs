use godot::classes::Sprite2D;
use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct LerpModesTests {
    base: Base<Node2D>,
    #[init(node = "Sprite2D")]
    ball: OnReady<Gd<Sprite2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, LerpModesTests }

impl ITestClass for LerpModesTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/lerp_modes.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_relative,
            Self::test_two_relatives,
            Self::test_speed_based,
            Self::test_speed_based_plus_relative,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl LerpModesTests {
    fn test_relative(&mut self) -> PinnedTestTask {
        let mut ball = self.ball.clone();
        let initial_pos = ball.get_global_position();
        let translation = Vector2::new(0.0, -500.0);

        let handle = ball.do_global_position(translation, 2.0)
            .as_relative(Vector2::ZERO)
            .register();
        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            tracker.wait_seconds(1.0).await;
            assert!(handle.is_relative());

            let progress = handle.get_animation_position() / 2.0;
            let expected_pos = initial_pos + translation * progress as f32;
            assert!(ball.get_global_position().distance_to(expected_pos) <= 0.1);

            let warp_pos = Vector2::new(600.0, 900.0);
            ball.set_global_position(warp_pos);

            wait_finished(&handle, &tracker, 2.0).await;
            let expected_pos = warp_pos + translation * (1.0 - progress as f32);
            assert!(ball.get_global_position().distance_to(expected_pos) <= 0.1);
        })
    }

    fn test_two_relatives(&mut self) -> PinnedTestTask {
        let ball = self.ball.clone();
        let initial_pos = ball.get_global_position();

        let first_trans = Vector2::new(0.0, -500.0);
        let first = ball.do_global_position(first_trans, 2.0)
            .as_relative(Vector2::ZERO)
            .register();

        let second_trans = Vector2::new(200.0, -100.0);
        let second = ball.do_global_position(second_trans, 3.0)
            .as_relative(Vector2::ZERO)
            .register();

        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            assert!(first.is_relative());

            wait_finished(&first, &tracker, 2.0).await;
            assert!(first.is_relative());
            assert!(second.is_relative());

            let second_progress = second.get_animation_position() / 3.0;
            let expected_pos = initial_pos + first_trans + second_trans * second_progress as f32;
            assert!(ball.get_global_position().distance_to(expected_pos) <= 0.1);

            wait_finished(&second, &tracker, 3.0).await;
            let expected_pos = initial_pos + first_trans + second_trans;
            assert!(ball.get_global_position().distance_to(expected_pos) <= 0.1);
        })
    }

    fn test_speed_based(&mut self) -> PinnedTestTask {
        let ball = self.ball.clone();
        let initial_pos = ball.get_global_position();
        let final_pos = Vector2::new(256.0, 256.0);
        let speed = 200.0;

        let handle = ball.do_move(final_pos, speed).as_speed_based().register();
        assert!(handle.is_speed_based());

        let distance = initial_pos.distance_to(final_pos) as f64;
        let expected_time = distance / speed;

        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            wait_finished(&handle, &tracker, expected_time).await;
            assert_eq!(ball.get_global_position(), final_pos);
        })
    }

    fn test_speed_based_plus_relative(&mut self) -> PinnedTestTask {
        let ball = self.ball.clone();
        let final_pos = Vector2::new(256.0, 256.0);
        let speed = 200.0;

        let speed_tween = ball.do_move(final_pos, speed).as_speed_based().register();

        let trans = Vector2::new(600.0, 200.0);
        let relative_tween = ball.do_move(trans, 3.0)
            .as_relative(Vector2::ZERO)
            .register();
        assert!(relative_tween.is_relative());

        let tracker = RcPtr::clone(&self.time_tracker);

        // Capture timer at speed_tween finish (synchronously, at emission time).
        let tracker_for_check = tracker.clone();
        let timer_at_speed_done = RcPtr::new(None::<f64>);
        let timer_slot = timer_at_speed_done.clone();
        speed_tween.to_mut().finished_connect(
            move || *timer_slot.to_mut() = Some(tracker_for_check.timer),
            SpireFlags::ONE_SHOT,
        );

        Box::pin(async move {
            wait_finished(&relative_tween, &tracker, 3.0).await;
            assert!(speed_tween.is_speed_based());
            assert!(speed_tween.is_playing());

            let distance = ball.get_global_position().distance_to(final_pos) as f64;
            let expected_time = distance / speed;

            // Wait for speed_tween to finish via its connection.
            while timer_at_speed_done.is_none() {
                next_frame().await;
            }
            let captured = timer_at_speed_done.to_mut().take().unwrap();
            assert_within_tolerance(captured - (expected_time + 3.0), TIME_TOLERANCE * 2.0);
            assert_eq!(ball.get_global_position(), final_pos);
        })
    }
}
