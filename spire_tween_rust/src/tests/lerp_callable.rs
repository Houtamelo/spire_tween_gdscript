use godot::classes::Sprite2D;
use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct LerpCallableTests {
    base: Base<Node2D>,
    #[init(node = "Sprite2D")]
    ball: OnReady<Gd<Sprite2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, LerpCallableTests }

impl ITestClass for LerpCallableTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/lerp_callable.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_lerp_call,
            Self::test_lerp_call_float,
            Self::test_lerp_call_with_ease,
            Self::test_lerp_call_with_loop,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl LerpCallableTests {
    fn test_lerp_call(&mut self) -> PinnedTestTask {
        let ball = self.ball.clone();
        let initial_pos = Vector2::ZERO;
        let final_pos = Vector2::new(1920.0, 1080.0);

        let call_count = RcPtr::new(0u64);
        let call_count_c = call_count.clone();
        let mut ball_c = ball.clone();

        let callable = Callable::from_fn("custom_call", move |args| {
            let val: Vector2 = args[0].to();
            ball_c.set_global_position(val);
            *call_count_c.to_mut() += 1;
            Variant::nil()
        });

        let tween = SpireTween::<LerpMethodData<Vector2>>::new(
            callable.clone(), initial_pos, final_pos, 5.0,
        ).register();
        assert_eq!(tween.get_start_value(), initial_pos);
        assert_eq!(tween.get_final_value(), final_pos);
        assert_eq!(tween.get_duration(), 5.0);
        assert_eq!(*tween.get_callable(), callable);
        let gd = tween.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            tracker.wait_finished(&gd, 5.0).await;
            let frames = tracker.frames_since_start();
            let diff = (frames as i64 - *call_count as i64).unsigned_abs();
            assert!(diff <= 1, "Call count should match frames: {frames} vs {}", *call_count);
            assert_eq!(ball.get_global_position(), final_pos);
        })
    }

    fn test_lerp_call_float(&mut self) -> PinnedTestTask {
        let ball = self.ball.clone();
        let initial_val = 0.0f64;
        let final_val = 1920.0f64;

        let call_count = RcPtr::new(0u64);
        let call_count_c = call_count.clone();
        let mut ball_c = ball.clone();

        let callable = Callable::from_fn("custom_call_float", move |args| {
            let val: f64 = args[0].to();
            let mut pos = ball_c.get_global_position();
            pos.x = val as f32;
            ball_c.set_global_position(pos);
            *call_count_c.to_mut() += 1;
            Variant::nil()
        });

        let tween = SpireTween::<LerpMethodData<f64>>::new(
            callable.clone(), initial_val, final_val, 5.0,
        ).register();
        assert_eq!(tween.get_start_value(), initial_val);
        assert_eq!(tween.get_final_value(), final_val);
        assert_eq!(tween.get_duration(), 5.0);
        assert_eq!(*tween.get_callable(), callable);
        let gd = tween.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            tracker.wait_finished(&gd, 5.0).await;
            let frames = tracker.frames_since_start();
            let diff = (frames as i64 - *call_count as i64).unsigned_abs();
            assert!(diff <= 1, "Call count should match frames");
            assert_eq!(ball.get_global_position().x, final_val as f32);
        })
    }

    fn test_lerp_call_with_ease(&mut self) -> PinnedTestTask {
        let ball = self.ball.clone();
        let initial_pos = Vector2::ZERO;
        let final_pos = Vector2::new(1920.0, 1080.0);

        let call_count = RcPtr::new(0u64);
        let call_count_c = call_count.clone();
        let mut ball_c = ball.clone();

        let callable = Callable::from_fn("eased_call", move |args| {
            let val: Vector2 = args[0].to();
            ball_c.set_global_position(val);
            *call_count_c.to_mut() += 1;
            Variant::nil()
        });

        let mut eased = SpireTween::<LerpMethodData<Vector2>>::new(
            callable, initial_pos, final_pos, 5.0,
        );
        eased.set_ease(EaseKind::Basic(Ease::InOutCubic));
        let eased_handle = eased.register();
        let eased_gd = eased_handle.gd_handle.as_ref().unwrap().clone();

        let second_ball = ball.duplicate_node().cast::<Sprite2D>();
        let mut second_ball_c = second_ball.clone();
        let linear_callable = Callable::from_fn("linear_call", move |args| {
            let val: Vector2 = args[0].to();
            second_ball_c.set_global_position(val);
            Variant::nil()
        });
        let linear_handle = SpireTween::<LerpMethodData<Vector2>>::new(
            linear_callable, initial_pos, final_pos, 5.0,
        ).register();
        let linear_gd = linear_handle.gd_handle.as_ref().unwrap().clone();

        let tracker = RcPtr::clone(&self.time_tracker);
        let mut base = self.base().clone();

        Box::pin(async move {
            base.add_child(&second_ball);

            // Poll each frame: eased and linear should diverge mid-animation
            while eased_handle.is_playing() {
                next_frame().await;
                let t = tracker.timer;
                if (t - 2.5).abs() > 0.2 && t > 0.2 && t < 4.8 {
                    assert_ne!(
                        ball.get_global_position(),
                        second_ball.get_global_position(),
                        "Eased and linear should differ at t={t}"
                    );
                }
            }

            if linear_handle.is_playing() {
                tracker.wait_finished(&linear_gd, 5.0).await;
            }

            let frames = tracker.frames_since_start();
            let diff = (frames as i64 - *call_count as i64).unsigned_abs();
            assert!(diff <= 1, "Call count should match frames");
            assert_eq!(ball.get_global_position(), final_pos);
            assert_eq!(second_ball.get_global_position(), final_pos);
        })
    }

    fn test_lerp_call_with_loop(&mut self) -> PinnedTestTask {
        let ball = self.ball.clone();
        let initial_pos = Vector2::ZERO;
        let final_pos = Vector2::new(1920.0, 1080.0);

        let call_count = RcPtr::new(0u64);
        let call_count_c = call_count.clone();
        let mut ball_c = ball.clone();

        let callable = Callable::from_fn("loop_call", move |args| {
            let val: Vector2 = args[0].to();
            ball_c.set_global_position(val);
            *call_count_c.to_mut() += 1;
            Variant::nil()
        });

        let mut tween = SpireTween::<LerpMethodData<Vector2>>::new(
            callable, initial_pos, final_pos, 2.0,
        );
        tween.set_loops(2, LoopMode::Yoyo);
        let handle = tween.register();
        let gd = handle.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&gd, &tracker, 4.0);

        Box::pin(async move {
            tracker.wait_loop_finished(&gd, 2.0).await;
            let frames = tracker.frames_since_start();
            let diff = (frames as i64 - *call_count as i64).unsigned_abs();
            assert!(diff <= 1, "Call count should match frames");
            assert_eq!(ball.get_global_position(), final_pos);

            tracker.wait_loop_finished(&gd, 4.0).await;
            if !handle.is_stopped() {
                tracker.wait_finished(&gd, 4.0).await;
            }
            let frames = tracker.frames_since_start();
            let diff = (frames as i64 - *call_count as i64).unsigned_abs();
            assert!(diff <= 1, "Call count should match frames after yoyo");
            assert_eq!(ball.get_global_position(), initial_pos);
        })
    }
}
