//! Integration tests for the tween templates that lacked coverage:
//! `do_follow`, `do_shake`, `do_contour_shape`, plus the `ignore_time_scale` flag.
//!
//! The 3D-only templates (`do_bone`, `do_ellipsis_3d`, `do_follow_3d`) are not
//! covered here -- they need a 3D scene with a Skeleton3D / Node3D, which would
//! require a separate test class and prefab. They're also straightforward
//! mirrors of the 2D versions logic-wise.

use godot::{
    classes::{Engine, Sprite2D},
    prelude::*,
};
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct TemplatesTests {
    base: Base<Node2D>,
    #[init(node = "Sprite2D")]
    sprite: OnReady<Gd<Sprite2D>>,
    #[init(node = "Leader")]
    leader: OnReady<Gd<Node2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, TemplatesTests }

impl ITestClass for TemplatesTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/templates.tscn";

    fn test_list() -> Vec<NamedTest<Self>> {
        vec![
            ("test_do_shake", Self::test_do_shake),
            ("test_do_shake_stays_within_radius", Self::test_do_shake_stays_within_radius),
            ("test_do_shake_returns_to_origin", Self::test_do_shake_returns_to_origin),
            ("test_shake_interpolation", Self::test_shake_interpolation),
            ("test_shake_numerical_fixtures", Self::test_shake_numerical_fixtures),
            ("test_shake_fade", Self::test_shake_fade),
            ("test_shake_blending", Self::test_shake_blending),
            ("test_shake_lifecycle", Self::test_shake_lifecycle),
            ("test_shake_loops", Self::test_shake_loops),
            ("test_shake_phase_and_cleanup", Self::test_shake_phase_and_cleanup),
            ("test_shake_owners_and_inputs", Self::test_shake_owners_and_inputs),
            ("test_do_contour_shape", Self::test_do_contour_shape),
            ("test_do_contour_shape_speed_based", Self::test_do_contour_shape_speed_based),
            ("test_do_contour_shape_empty", Self::test_do_contour_shape_empty),
            ("test_do_follow", Self::test_do_follow),
            // ignore_time_scale must be last because it touches Engine.time_scale globally.
            ("test_ignore_time_scale", Self::test_ignore_time_scale),
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl TemplatesTests {
    /// Verifies that `do_shake` actually shakes the node — i.e. the position
    /// deviates from the origin during the tween. The companion tests
    /// `test_do_shake_stays_within_radius` and `test_do_shake_returns_to_origin`
    /// cover the bound and restore guarantees separately.
    fn test_do_shake(&mut self) -> PinnedTestTask {
        let sprite = self.sprite.clone();
        let origin = sprite.get_position();
        let handle = sprite
            .do_shake(2.0, Vector2::new(15.0, 20.0), 30, 90.0, true, ShakeRandomnessMode::Full, true)
            .register();
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&handle, &tracker, 2.0);

        Box::pin(async move {
            // Sample the position every frame; track max deviation and unique positions.
            let mut max_deviation: f32 = 0.0;
            let mut last_pos = sprite.get_position();
            let mut distinct_positions = 1u32;

            while !handle.is_stopped() {
                let pos = sprite.get_position();
                let dev = pos.distance_to(origin);
                if dev > max_deviation {
                    max_deviation = dev;
                }
                if pos != last_pos {
                    distinct_positions += 1;
                    last_pos = pos;
                }
                next_frame().await;
            }

            assert!(max_deviation > 0.0, "do_shake must move the node at the tween");

            // Continuous waypoint interpolation must keep producing new positions.
            assert!(
                distinct_positions >= 10,
                "do_shake should continuously interpolate positions, only got {distinct_positions}"
            );

            let _ = &tracker;
        })
    }

    /// Waypoints and their linear interpolation remain within strength.length().
    fn test_do_shake_stays_within_radius(&mut self) -> PinnedTestTask {
        let mut sprite = self.sprite.clone();
        sprite.set_position(Vector2::ZERO);
        let origin = sprite.get_position();
        let radius_max: f32 = 25.0;
        let tolerance: f32 = 0.5;

        let handle = sprite
            .do_shake(1.0, Vector2::new(15.0, 20.0), 60, 90.0, true, ShakeRandomnessMode::Full, true)
            .register();
        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            let mut max_dev: f32 = 0.0;
            let mut max_dev_frame: u32 = 0;
            let mut frame: u32 = 0;
            while !handle.is_stopped() {
                let dev = sprite.get_position().distance_to(origin);
                if dev > max_dev {
                    max_dev = dev;
                    max_dev_frame = frame;
                }
                frame += 1;
                next_frame().await;
            }

            assert!(
                max_dev <= radius_max + tolerance,
                "do_shake must keep the node within radius_max ({radius_max}) of origin; saw {max_dev} on frame \
                 {max_dev_frame} of {frame} (overshoot of {} above tolerance)",
                max_dev - (radius_max + tolerance)
            );

            let _ = &tracker;
        })
    }

    /// Asserts the node ends at its origin position after the shake completes.
    ///
    fn test_do_shake_returns_to_origin(&mut self) -> PinnedTestTask {
        let mut sprite = self.sprite.clone();
        sprite.set_position(Vector2::ZERO);
        let origin = sprite.get_position();

        let handle = sprite
            .do_shake(1.0, Vector2::new(15.0, 20.0), 30, 90.0, true, ShakeRandomnessMode::Full, true)
            .register();
        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            wait_finished(&handle, &tracker, 1.0).await;
            // Give the tween machinery one extra frame to flush any deferred
            // final tick before reading the node's resting position.
            next_frame().await;

            let final_pos = sprite.get_position();
            let dev = final_pos.distance_to(origin);
            assert!(
                dev < 0.01,
                "do_shake must restore position to origin {origin} on completion; got {final_pos} (deviation {dev})"
            );
        })
    }

    /// Verifies that `do_contour_shape` walks the polyline and ends at the last vertex.
    /// `is_speed_based: false` — every segment runs for `duration_or_speed` seconds.
    /// Walks an equal-segment triangle and asserts the sprite passes through each
    /// vertex at the expected timestamp.
    fn test_do_contour_shape(&mut self) -> PinnedTestTask {
        let mut sprite = self.sprite.clone();
        sprite.set_position(Vector2::ZERO);
        let start = sprite.get_position();

        // Polyline: start -> A -> B -> C, each segment length 100.
        let a = start + Vector2::new(100.0, 0.0);
        let b = start + Vector2::new(100.0, 100.0);
        let c = start + Vector2::new(0.0, 100.0);

        let mut vertices: Array<Vector2> = Array::new();
        vertices.push(a);
        vertices.push(b);
        vertices.push(c);

        // 1.0s per segment, 3 segments -> 3.0s total.
        let seq = sprite.do_contour_shape(vertices, 1.0, false).register();
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&seq, &tracker, 3.0);

        Box::pin(async move {
            // 1 frame of jitter at 60fps × 100 units/s ≈ 1.7 units; 5 is generous.
            const POS_TOLERANCE: f32 = 5.0;

            // After segment 1 (start->A) the sprite should be at A.
            tracker.wait_seconds(1.0).await;
            let pos = sprite.get_position();
            assert!(
                pos.distance_to(a) < POS_TOLERANCE,
                "after 1.0s expected near A={a:?}, got {pos:?} (dist={:.2})",
                pos.distance_to(a)
            );

            // After segment 2 (A->B): at B.
            tracker.wait_seconds(1.0).await;
            let pos = sprite.get_position();
            assert!(
                pos.distance_to(b) < POS_TOLERANCE,
                "after 2.0s expected near B={b:?}, got {pos:?} (dist={:.2})",
                pos.distance_to(b)
            );

            wait_finished(&seq, &tracker, 3.0).await;
            // Final position snaps exactly to the last vertex.
            assert_eq!(sprite.get_position(), c);
        })
    }

    /// `is_speed_based: true` — `duration_or_speed` is the *total* path duration,
    /// distributed proportionally to each segment's length. Uses an unequal-segment
    /// polyline so the proportional distribution is observable.
    fn test_do_contour_shape_speed_based(&mut self) -> PinnedTestTask {
        let mut sprite = self.sprite.clone();
        sprite.set_position(Vector2::ZERO);
        let start = sprite.get_position();

        // Unequal segments: start->A is 300 units, A->B is 100 units. Perimeter 400.
        let a = start + Vector2::new(300.0, 0.0);
        let b = start + Vector2::new(300.0, 100.0);

        let mut vertices: Array<Vector2> = Array::new();
        vertices.push(a);
        vertices.push(b);

        // Total path duration = 2.0s. Distributed proportionally:
        //   - Segment 1 (300/400 of perimeter) -> 1.5s
        //   - Segment 2 (100/400 of perimeter) -> 0.5s
        let seq = sprite.do_contour_shape(vertices, 2.0, true).register();
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&seq, &tracker, 2.0);

        Box::pin(async move {
            // Higher tolerance: segment 1 runs at 200 units/s.
            const POS_TOLERANCE: f32 = 8.0;

            // At t=1.5s, segment 1 just finished -> sprite at A.
            tracker.wait_seconds(1.5).await;
            let pos = sprite.get_position();
            assert!(
                pos.distance_to(a) < POS_TOLERANCE,
                "after 1.5s expected near A={a:?}, got {pos:?} (dist={:.2})",
                pos.distance_to(a)
            );

            wait_finished(&seq, &tracker, 2.0).await;
            assert_eq!(sprite.get_position(), b);
        })
    }

    /// Empty-polyline guard: returns an empty sequence that finishes on its first
    /// tick without moving the sprite. Also emits a `godot_error!` (intentional —
    /// the caller passed nothing to animate).
    fn test_do_contour_shape_empty(&mut self) -> PinnedTestTask {
        let mut sprite = self.sprite.clone();
        sprite.set_position(Vector2::ZERO);
        let start = sprite.get_position();

        let vertices: Array<Vector2> = Array::new();
        let seq = sprite.do_contour_shape(vertices, 1.0, false).register();
        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            // A few frames is plenty: the empty sequence completes on its first
            // process tick.
            tracker.wait_seconds(0.1).await;
            assert_eq!(sprite.get_position(), start, "empty polyline must not move the sprite");
            assert!(seq.is_stopped(), "empty contour-shape sequence should auto-finish on first tick");
        })
    }

    /// Verifies that `do_follow`:
    /// - Catches up to a moving leader at the given speed.
    /// - Final position equals the leader's final position when the chase converges.
    fn test_do_follow(&mut self) -> PinnedTestTask {
        let leader = self.leader.clone();
        let sprite = self.sprite.clone();
        let leader_target = leader.get_global_position() + Vector2::new(500.0, 0.0);

        // Move the leader to its target over 2 seconds.
        let leader_tween = leader.clone().do_global_position(leader_target, 2.0).register();

        // Follower chases the leader at 200 px/sec.
        let follow_tween = sprite.do_follow(leader.clone(), 200.0).as_speed_based().register();

        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            // Wait for the leader to finish moving.
            wait_finished(&leader_tween, &tracker, 2.0).await;
            assert_eq!(leader.get_global_position(), leader_target);

            // Follower is still chasing. Poll until it stops (no timing assertion since
            // catch-up duration depends on the dynamic chase distance).
            while !follow_tween.is_stopped() {
                next_frame().await;
            }

            // Follower must have converged to the leader's final position.
            // Speed-based tween stops when distance reaches 0; allow only float epsilon.
            let dist = sprite.get_global_position().distance_to(leader_target);
            assert!(dist < 0.01, "follower failed to converge: distance {dist} from leader at {leader_target}");
        })
    }

    /// At zero time scale, only the unscaled tween can finish. After resuming
    /// at half speed, the control must consume its duration in Godot's scaled
    /// clock. These clocks need not stay proportional under CI frame stalls.
    fn test_ignore_time_scale(&mut self) -> PinnedTestTask {
        let sprite = self.sprite.clone();
        let leader = self.leader.clone();
        let initial = sprite.get_position();
        let control_initial = leader.get_position();
        let target_a = initial + Vector2::new(100.0, 0.0);
        let target_b = control_initial + Vector2::new(50.0, 0.0);
        struct RestoreTimeScale(f64);
        impl Drop for RestoreTimeScale {
            fn drop(&mut self) { Engine::singleton().set_time_scale(self.0); }
        }
        let restore_time_scale = RestoreTimeScale(Engine::singleton().get_time_scale());
        Engine::singleton().set_time_scale(0.0);

        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            // Let the new time scale take effect before registering either tween.
            next_frame().await;
            let mut ignored = sprite.do_position(target_a, 1.0);
            ignored.set_ignore_time_scale(true);
            let mut ignored_handle = ignored.register();
            let mut control_handle = leader.do_position(target_b, 1.0).register();
            let started = std::time::Instant::now();
            while !ignored_handle.is_stopped() && started.elapsed().as_secs_f64() < 10.0 {
                next_frame().await;
            }
            let ignored_completed = ignored_handle.is_stopped() && sprite.get_position() == target_a;
            let control_frozen = control_handle.is_playing()
                && control_handle.get_total_elapsed_time() == 0.0
                && leader.get_position() == control_initial;

            Engine::singleton().set_time_scale(0.5);
            let timer_at_control_start = tracker.timer;
            let control_timer = RcPtr::new(None::<(f64, f64)>);
            let slot = control_timer.clone();
            let tr = tracker.clone();
            let node = leader.clone();
            control_handle.to_mut().finished_connect(
                move || *slot.to_mut() = Some((tr.timer, node.get_process_delta_time())),
                SpireFlags::ONE_SHOT,
            );

            let started = std::time::Instant::now();
            while !control_handle.is_stopped() && started.elapsed().as_secs_f64() < 30.0 {
                next_frame().await;
            }
            let control_completed = control_handle.is_stopped() && leader.get_position() == target_b;
            ignored_handle.stop();
            control_handle.stop();
            sprite.clone().set_position(initial);
            leader.clone().set_position(control_initial);
            drop(restore_time_scale);

            assert!(ignored_completed, "ignore_time_scale=true must finish while Engine.time_scale is zero");
            assert!(control_frozen, "default tween must stay unchanged while Engine.time_scale is zero");
            assert!(control_completed, "default tween must finish after restoring a nonzero time scale");
            let (control_at_finish, final_frame_delta) = control_timer
                .to_mut()
                .take()
                .expect("control tween's finished closure must have fired");
            let control_delta = control_at_finish - timer_at_control_start;
            assert!(
                control_delta >= 1.0 - 1e-8 && control_delta <= 1.0 + final_frame_delta + 1e-8,
                "default tween must consume 1s of scaled time, with at most its final frame's overshoot; got \
                 {control_delta}, final frame {final_frame_delta}"
            );
        })
    }
}
