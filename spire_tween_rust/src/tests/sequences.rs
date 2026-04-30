use godot::{classes::CollisionShape2D, prelude::*};
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct SequencesTests {
    base: Base<Node2D>,
    #[init(node = "BouncyBall/CollisionShape2D")]
    shape: OnReady<Gd<CollisionShape2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, SequencesTests }

impl ITestClass for SequencesTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/sequences.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_proper_ordering,
            Self::test_stopped_child_does_not_halt_sequence,
            Self::test_append_many,
            Self::test_remove,
            Self::test_remove_midway,
            Self::test_remove_call,
            Self::test_default_children_ease,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl SequencesTests {
    fn test_proper_ordering(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let msgs = RcPtr::new(Vec::<String>::new());

        let mut seq = SpireTween::<Sequence>::new();

        let msgs_c = msgs.clone();
        seq.append_call(Callable::from_fn("first_call", move |_| {
            msgs_c.to_mut().push("First call".into());
            Variant::nil()
        }));

        seq.append_interval(1.0);

        let msgs_c = msgs.clone();
        seq.append_call(Callable::from_fn("second_call", move |_| {
            msgs_c.to_mut().push("Second call".into());
            Variant::nil()
        }));

        let msgs_c = msgs.clone();
        seq.join_call(Callable::from_fn("same_block", move |_| {
            msgs_c.to_mut().push("Same block as second call".into());
            Variant::nil()
        }));

        let msgs_c = msgs.clone();
        let delayed = shape.do_delayed_call(
            move || { msgs_c.to_mut().push("Third call with delay".into()); },
            2.0,
        ).register();
        seq.append_ptr(delayed);

        seq.join_interval(5.0);

        let seq = seq.register();
        let seq_gd = seq.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&seq_gd, &tracker, 6.0);

        Box::pin(async move {
            tracker.wait_loop_finished(&seq_gd, 6.0).await;
            if !seq.is_stopped() {
                tracker.wait_finished(&seq_gd, 6.0).await;
            }

            let expected = vec![
                "First call",
                "Second call",
                "Same block as second call",
                "Third call with delay",
            ];
            assert_eq!(*msgs, expected);
        })
    }

    fn test_stopped_child_does_not_halt_sequence(&mut self) -> PinnedTestTask {
        let mut shape = self.shape.clone();

        let mut seq = SpireTween::<Sequence>::new();

        let stopped_child = shape.do_scale(Vector2::ZERO, 1.0).register();
        seq.append_ptr(stopped_child.clone());
        stopped_child.to_mut().stop();

        let good_child = shape.do_scale(Vector2::new(3.0, 3.0), 2.0).register();
        let good_gd = good_child.gd_handle.as_ref().unwrap().clone();
        seq.join_ptr(good_child);

        seq.append_interval(1.0);

        let dead_child = shape.do_position(Vector2::new(-500.0, 30.0), 0.5).register();
        seq.append_ptr(dead_child);

        seq.append_call(Callable::from_fn("last_block", |_| { Variant::nil() }));

        let seq = seq.register();
        let seq_gd = seq.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&seq_gd, &tracker, 3.0);

        // Snapshot scale at good_child finish (before next block modifies it)
        let shape_c = shape.clone();
        let scale_at_good_done = snapshot_on_signal(
            &Signal::from_object_signal(&good_gd, "finished"),
            move || shape_c.get_scale(),
        );

        Box::pin(async move {
            next_frame().await;
            assert!(stopped_child.is_stopped());

            tracker.wait_finished(&good_gd, 2.0).await;
            assert_eq!(scale_at_good_done.to_mut().take().unwrap(), Vector2::new(3.0, 3.0));

            shape.queue_free();

            tracker.wait_loop_finished(&seq_gd, 3.0).await;
            if !seq.is_stopped() {
                tracker.wait_finished(&seq_gd, 3.0).await;
            }
        })
    }

    fn test_append_many(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let shrink_scale = Vector2::ONE;
        let grown_scale = Vector2::ONE * 3.0;

        let first = shape.do_scale(grown_scale, 1.0).register();
        let second = shape.do_scale(shrink_scale, 3.0).register();
        let third = shape.do_scale(grown_scale, 2.0).register();
        let fourth = shape.do_scale(shrink_scale, 1.0).register();

        let mut seq = SpireTween::<Sequence>::new();
        seq.append_ptr(first.clone());
        seq.append_ptr(second.clone());
        seq.append_call(Callable::from_fn("wait_msg", |_| { Variant::nil() }));

        let wait_call = shape.do_delayed_call(|| {}, 5.0).register();
        seq.append_ptr(wait_call);

        seq.append_ptr(third.clone());
        seq.append_ptr(fourth.clone());
        seq.append_call(Callable::from_fn("wait_msg2", |_| { Variant::nil() }));
        seq.append_interval(2.0);
        seq.append_call(Callable::from_fn("done_msg", |_| { Variant::nil() }));

        let seq = seq.register();
        let seq_gd = seq.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&seq_gd, &tracker, 14.0);

        let first_gd = first.gd_handle.as_ref().unwrap().clone();
        let second_gd = second.gd_handle.as_ref().unwrap().clone();
        let third_gd = third.gd_handle.as_ref().unwrap().clone();
        let fourth_gd = fourth.gd_handle.as_ref().unwrap().clone();

        // Snapshot at each child finish
        let shape_c = shape.clone();
        let s1 = snapshot_on_signal(&Signal::from_object_signal(&first_gd, "finished"), move || shape_c.get_scale());
        let shape_c = shape.clone();
        let s2 = snapshot_on_signal(&Signal::from_object_signal(&second_gd, "finished"), move || shape_c.get_scale());
        let shape_c = shape.clone();
        let s3 = snapshot_on_signal(&Signal::from_object_signal(&third_gd, "finished"), move || shape_c.get_scale());
        let shape_c = shape.clone();
        let s4 = snapshot_on_signal(&Signal::from_object_signal(&fourth_gd, "finished"), move || shape_c.get_scale());

        Box::pin(async move {
            tracker.wait_finished(&first_gd, 1.0).await;
            assert_eq!(s1.to_mut().take().unwrap(), grown_scale);

            tracker.wait_finished(&second_gd, 4.0).await;
            assert_eq!(s2.to_mut().take().unwrap(), shrink_scale);

            tracker.wait_finished(&third_gd, 11.0).await;
            assert_eq!(s3.to_mut().take().unwrap(), grown_scale);

            tracker.wait_finished(&fourth_gd, 12.0).await;
            assert_eq!(s4.to_mut().take().unwrap(), shrink_scale);

            if !seq.is_stopped() {
                tracker.wait_finished(&seq_gd, 14.0).await;
            }
        })
    }

    fn test_remove(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let shrink_scale = Vector2::ONE;
        let grown_scale = Vector2::ONE * 3.0;

        let first = shape.do_scale(grown_scale, 1.0).register();
        let second = shape.do_scale(shrink_scale * 5.0, 3.0).register();
        let third = shape.do_scale(shrink_scale, 2.0).register();

        let mut seq = SpireTween::<Sequence>::new();
        seq.append_ptr(first.clone());
        seq.append_ptr(second.clone());
        seq.append_ptr(third.clone());

        assert!(seq.remove(&second));

        let seq = seq.register();
        let seq_gd = seq.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&seq_gd, &tracker, 3.0);

        let first_gd = first.gd_handle.as_ref().unwrap().clone();
        let third_gd = third.gd_handle.as_ref().unwrap().clone();

        let shape_c = shape.clone();
        let s1 = snapshot_on_signal(&Signal::from_object_signal(&first_gd, "finished"), move || shape_c.get_scale());

        Box::pin(async move {
            tracker.wait_finished(&first_gd, 1.0).await;
            assert_eq!(s1.to_mut().take().unwrap(), grown_scale);

            tracker.wait_finished(&third_gd, 3.0).await;
            assert_eq!(shape.get_scale(), shrink_scale);

            if !seq.is_stopped() {
                tracker.wait_finished(&seq_gd, 3.0).await;
            }
        })
    }

    fn test_remove_midway(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let shrink_scale = Vector2::ONE;
        let grown_scale = Vector2::ONE * 3.0;

        let first = shape.do_scale(grown_scale, 1.0).register();
        let second = shape.do_scale(grown_scale * 3.0, 3.0).register();
        let third = shape.do_scale(shrink_scale, 2.0).register();

        let mut seq = SpireTween::<Sequence>::new();
        seq.append_ptr(first.clone());
        seq.append_ptr(second.clone());
        seq.append_ptr(third.clone());
        let mut seq = seq.register();

        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(seq.gd_handle.as_ref().unwrap(), &tracker, 4.5);

        let first_gd = first.gd_handle.as_ref().unwrap().clone();
        let shape_c = shape.clone();
        let scale_at_first_done = snapshot_on_signal(
            &Signal::from_object_signal(&first_gd, "finished"),
            move || shape_c.get_scale(),
        );

        Box::pin(async move {
            tracker.wait_finished(&first_gd, 1.0).await;
            assert_eq!(scale_at_first_done.to_mut().take().unwrap(), grown_scale);

            tracker.wait_seconds(1.5).await;
            assert!(seq.remove(&second));

            tracker.wait_finished(third.gd_handle.as_ref().unwrap(), 4.5).await;
            assert_eq!(shape.get_scale(), shrink_scale);

            if !seq.is_stopped() {
                tracker.wait_finished(seq.gd_handle.as_ref().unwrap(), 4.5).await;
            }
        })
    }

    fn test_remove_call(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let shrink_scale = Vector2::ONE;
        let grown_scale = Vector2::ONE * 3.0;

        let first = shape.do_scale(grown_scale, 1.0).register();
        let third = shape.do_scale(shrink_scale, 2.0).register();

        let callable = Callable::from_fn("removable_call", |_| { Variant::nil() });

        let mut seq = SpireTween::<Sequence>::new();
        seq.append_ptr(first.clone());
        seq.append_call(callable.clone());
        seq.append_ptr(third.clone());

        assert!(seq.remove_call(&callable));

        let seq = seq.register();
        let seq_gd = seq.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&seq_gd, &tracker, 3.0);

        let first_gd = first.gd_handle.as_ref().unwrap().clone();
        let third_gd = third.gd_handle.as_ref().unwrap().clone();

        let shape_c = shape.clone();
        let s1 = snapshot_on_signal(&Signal::from_object_signal(&first_gd, "finished"), move || shape_c.get_scale());

        Box::pin(async move {
            tracker.wait_finished(&first_gd, 1.0).await;
            assert_eq!(s1.to_mut().take().unwrap(), grown_scale);

            tracker.wait_finished(&third_gd, 3.0).await;
            assert_eq!(shape.get_scale(), shrink_scale);

            if !seq.is_stopped() {
                tracker.wait_finished(&seq_gd, 3.0).await;
            }
        })
    }

    fn test_default_children_ease(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let initial_scale = shape.get_scale();
        let final_scale = Vector2::ONE * 3.0;

        let first = shape.do_scale(final_scale, 3.0).register();
        let second = shape.do_scale(initial_scale, 2.0).register();

        let mut seq = SpireTween::<Sequence>::new();
        seq.set_default_ease(Ease::InExpo);
        seq.append_ptr(first.clone());
        seq.append_ptr(second.clone());
        let seq = seq.register();

        assert!(matches!(first.get_ease(), EaseKind::Basic(Ease::InExpo)));
        assert!(matches!(second.get_ease(), EaseKind::Basic(Ease::InExpo)));

        let first_gd = first.gd_handle.as_ref().unwrap().clone();
        let seq_gd = seq.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&seq_gd, &tracker, 5.0);

        // Snapshot scale at first finish (before second modifies it)
        let shape_c = shape.clone();
        let scale_at_first_done = snapshot_on_signal(
            &Signal::from_object_signal(&first_gd, "finished"),
            move || shape_c.get_scale(),
        );

        Box::pin(async move {
            tracker.wait_seconds(2.0).await;
            let progress = first.get_animation_position() / 3.0;
            let weight = sample_in_expo(progress);
            let expected_val = initial_scale.lerp(final_scale, weight as f32);
            assert!(shape.get_scale().distance_to(expected_val) <= 0.001);

            tracker.wait_finished(&first_gd, 3.0).await;
            let progress = first.get_animation_position() / 3.0;
            assert_eq!(progress, 1.0);
            assert_eq!(scale_at_first_done.to_mut().take().unwrap(), final_scale);

            tracker.wait_seconds(1.0).await;
            let progress = second.get_animation_position() / 2.0;
            let weight = sample_in_expo(progress);
            let expected_val = final_scale.lerp(initial_scale, weight as f32);
            assert!(shape.get_scale().distance_to(expected_val) <= 0.001);

            if !seq.is_stopped() {
                tracker.wait_finished(&seq_gd, 5.0).await;
            }
            assert_eq!(shape.get_scale(), initial_scale);
        })
    }
}

fn sample_in_expo(x: f64) -> f64 {
    const B: f64 = 10.0;
    const C: f64 = 2.0;
    const D: f64 = 1.99804687;
    const J: f64 = 0.0004887581;
    2.0 * ((C.powf(B * x - B) / D) - J)
}
