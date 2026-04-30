use godot::classes::Sprite2D;
use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct ErrorHandlingTests {
    base: Base<Node2D>,
    #[init(node = "BouncyBall/CollisionShape2D/Sprite2D")]
    sprite: OnReady<Gd<Sprite2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, ErrorHandlingTests }

impl ITestClass for ErrorHandlingTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/error_handling.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_invalid_sequence_adding,
            // test_free_while_playing must be last (frees sprite)
            Self::test_free_while_playing,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl ErrorHandlingTests {
    fn test_invalid_sequence_adding(&mut self) -> PinnedTestTask {
        let sprite = self.sprite.clone();
        let final_scale = Vector2::ONE * 3.0;

        let tween = sprite.do_scale(final_scale, 3.0).register();
        let tween_gd = tween.gd_handle.as_ref().unwrap().clone();

        let mut seq = SpireTween::<Sequence>::new();
        seq.append_ptr(tween.clone());
        seq.append_ptr(tween.clone());
        seq.append_interval(1.0);
        seq.join_ptr(tween.clone());
        let seq = seq.register();
        // Self-append via RcPtr -- should be rejected gracefully
        seq.to_mut().append_ptr(seq.clone());

        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            tracker.wait_finished(&tween_gd, 3.0).await;
            assert_eq!(sprite.get_scale(), final_scale);
        })
    }

    fn test_free_while_playing(&mut self) -> PinnedTestTask {
        let mut sprite = self.sprite.clone();

        let handle = sprite.do_color_g(0.0, 3.0).register();
        let gd = handle.gd_handle.as_ref().unwrap().clone();

        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            tracker.wait_seconds(2.0).await;
            sprite.queue_free();

            // Wait several frames for the TweenManager to process the freed node
            for _ in 0..4 {
                next_frame().await;
            }

            assert!(!gd.bind().is_registered());
            assert!(handle.is_stopped());
        })
    }
}
