use godot::classes::CollisionShape2D;
use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct MiscTests {
    base: Base<Node2D>,
    #[init(node = "BouncyBall/CollisionShape2D")]
    shape: OnReady<Gd<CollisionShape2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, MiscTests }

impl ITestClass for MiscTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/misc.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_force_complete,
            Self::test_dyn_target,
            Self::test_property_path,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl MiscTests {
    fn test_force_complete(&mut self) -> PinnedTestTask {
        let mut shape = self.shape.clone();
        let final_value = Vector2::ONE * 2.5;

        let mut handle = shape.do_scale(final_value, 5.0).register();
        handle.force_complete();

        assert!(handle.is_stopped());
        assert_eq!(shape.get_scale(), final_value);

        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            tracker.wait_seconds(2.0).await;

            shape.set_scale(Vector2::ONE);
            handle.play();

            wait_finished(&handle, &tracker, 7.0).await;
            assert_eq!(shape.get_scale(), final_value);
        })
    }

    fn test_dyn_target(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let initial_scale = shape.get_scale();
        let speed = 0.5;

        let dyn_target = RcPtr::new(Vector2::ONE * 5.0);
        let dyn_target_for_callable = dyn_target.clone();

        let mut tween = shape.do_scale(Vector2::ZERO, speed).as_speed_based();
        tween.set_dynamic_target(Callable::from_fn("dyn_target_fn", move |_| {
            let val: Vector2 = *dyn_target_for_callable;
            val.to_variant()
        }));
        let handle = tween.register();
        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            let wait_time = 3.0;
            tracker.wait_seconds(wait_time).await;

            let expected_scale = initial_scale.move_toward(*dyn_target, (speed * wait_time) as f32);
            assert!(
                shape.get_scale().distance_to(expected_scale) <= 0.01,
                "Expected near {expected_scale}, got {}", shape.get_scale(),
            );

            *dyn_target.to_mut() = Vector2::ONE;
            let expected_duration = shape.get_scale().distance_to(*dyn_target) as f64 / speed;

            wait_finished(&handle, &tracker, wait_time + expected_duration).await;
            assert_eq!(shape.get_scale(), *dyn_target);
        })
    }

    fn test_property_path(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();

        let scale_tween = shape.do_scale_x(5.0, 1.0);
        assert_eq!(scale_tween.get_property_path(), NodePath::from("scale:x"));

        let pos_tween = shape.do_move(Vector2::new(200.0, 500.0), 2.0);
        assert_eq!(pos_tween.get_property_path(), NodePath::from("global_position"));

        Box::pin(async move {})
    }
}
