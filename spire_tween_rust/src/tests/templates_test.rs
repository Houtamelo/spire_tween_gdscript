//! Integration tests for the tween templates that lacked coverage:
//! `do_follow`, `do_shake`, `do_contour_shape`, plus the `ignore_time_scale` flag.
//!
//! The 3D-only templates (`do_bone`, `do_ellipsis_3d`, `do_follow_3d`) are not
//! covered here -- they need a 3D scene with a Skeleton3D / Node3D, which would
//! require a separate test class and prefab. They're also straightforward
//! mirrors of the 2D versions logic-wise.

use godot::classes::{Engine, Sprite2D};
use godot::prelude::*;
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

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_do_shake,
            // The next two are EXPECTED TO FAIL until the corresponding
            // do_shake.rs bugs (documented on test_do_shake) are fixed.
            Self::test_do_shake_stays_within_radius,
            Self::test_do_shake_returns_to_origin,
            Self::test_do_contour_shape,
            Self::test_do_follow,
            // ignore_time_scale must be last because it touches Engine.time_scale globally.
            Self::test_ignore_time_scale,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl TemplatesTests {
    /// Verifies that `do_shake` actually shakes the node — i.e. the position
    /// deviates from the origin during the tween.
    ///
    /// NOTE: Two pre-existing bugs in `do_shake.rs` are NOT asserted by this test:
    /// 1. The first shake step uses an uninitialized `prev_offset` (set to a random
    ///    value but never applied to the node), so `next_pos = origin - prev_offset
    ///    + next_offset` can deviate up to `2 * radius_max` from origin on the very
    ///    first update. Subsequent updates correctly stay within `radius_max`.
    /// 2. The restore-to-origin branch (`if time.approx_eq(&duration)`) doesn't fire
    ///    reliably -- the final tween tick's `time` may not equal `duration` within
    ///    `approx_eq` tolerance, leaving the node at the last shake offset.
    fn test_do_shake(&mut self) -> PinnedTestTask {
        let mut sprite = self.sprite.clone();
        let origin = sprite.get_position();
        let radius_min = 5.0;
        let radius_max = 25.0;

        let handle = sprite.do_shake(radius_min, radius_max, 0.5, 30.0, 2.0).register();
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

            assert!(
                max_deviation > 0.0,
                "do_shake must move the node at the tween"
            );

            // At 30 Hz over 2 s the shake should produce ~60 update events. Allow plenty
            // of headroom for missed frames; a buggy shake that only fires once would
            // produce 1 distinct position (the initial origin), which fails this check.
            assert!(
                distinct_positions >= 10,
                "do_shake should produce many distinct positions over 2s @ 30Hz, only got {distinct_positions}"
            );

            let _ = &tracker;
        })
    }

    /// Asserts the do_shake "first-step uses uninitialized prev_offset" bug.
    ///
    /// The shake's `prev_offset` is seeded with a random vector at construction
    /// time but never written to the node. On the first update tick the formula
    /// `next_pos = origin - prev_offset + next_offset` therefore moves the node
    /// to `origin + (next_offset - prev_offset)`, whose magnitude can reach
    /// `2 * radius_max` instead of being bounded by `radius_max`.
    ///
    /// EXPECTED TO FAIL until `do_shake.rs` initialises `prev_offset` to ZERO
    /// (or otherwise prevents the unapplied offset from contaminating the first
    /// update).
    fn test_do_shake_stays_within_radius(&mut self) -> PinnedTestTask {
        let mut sprite = self.sprite.clone();
        sprite.set_position(Vector2::ZERO);
        let origin = sprite.get_position();
        let radius_min: f32 = 5.0;
        let radius_max: f32 = 25.0;
        let tolerance: f32 = 0.5;

        // vibratio=0.5 makes next_angle land near `prev_angle + PI`, so
        // prev_offset and next_offset point in roughly opposite directions and
        // their difference has magnitude near `prev_radius + next_radius`,
        // which is always > radius_max when both radii are above radius_avg.
        let handle = sprite
            .do_shake(radius_min, radius_max, 0.5, 60.0, 1.0)
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
                "do_shake must keep the node within radius_max ({radius_max}) of origin; \
                 saw {max_dev} on frame {max_dev_frame} of {frame} \
                 (overshoot of {} above tolerance)",
                max_dev - (radius_max + tolerance)
            );

            let _ = &tracker;
        })
    }

    /// Asserts the do_shake "restore-to-origin doesn't fire reliably" bug.
    ///
    /// The restore branch (`if time.approx_eq(&duration)`) compares the
    /// callable's `time` argument against `duration`; the final tick's `time`
    /// may not land exactly on `duration` within `approx_eq` tolerance, so the
    /// branch never executes and the node stays at the last shake offset.
    ///
    /// EXPECTED TO FAIL until `do_shake.rs` restores the origin via a more
    /// reliable signal (e.g. on tween `finished`, or a `time >= duration`
    /// inequality).
    fn test_do_shake_returns_to_origin(&mut self) -> PinnedTestTask {
        let mut sprite = self.sprite.clone();
        sprite.set_position(Vector2::ZERO);
        let origin = sprite.get_position();

        let handle = sprite
            .do_shake(5.0, 25.0, 0.5, 30.0, 1.0)
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
                "do_shake must restore position to origin {origin} on completion; \
                 got {final_pos} (deviation {dev})"
            );
        })
    }

    /// Verifies that `do_contour_shape` walks the polyline and ends at the last vertex.
    fn test_do_contour_shape(&mut self) -> PinnedTestTask {
        let sprite = self.sprite.clone();
        let start = sprite.get_position();

        // Polyline: start -> A -> B -> C
        let a = start + Vector2::new(100.0, 0.0);
        let b = start + Vector2::new(100.0, 100.0);
        let c = start + Vector2::new(0.0, 100.0);

        let mut vertices: Array<Vector2> = Array::new();
        vertices.push(a);
        vertices.push(b);
        vertices.push(c);

        // Per-segment duration of 1.0s, 3 segments => 3s total.
        let seq = sprite.do_contour_shape(vertices, 1.0, false).register();
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&seq, &tracker, 3.0);

        Box::pin(async move {
            wait_finished(&seq, &tracker, 3.0).await;
            // Final position must equal the last vertex.
            assert_eq!(sprite.get_position(), c);
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
            assert!(
                dist < 0.01,
                "follower failed to converge: distance {dist} from leader at {leader_target}"
            );
        })
    }

    /// Verifies `ignore_time_scale` against a control case:
    /// - With `Engine::set_time_scale(0.5)`, a default tween (`ignore_time_scale=false`)
    ///   advances at scaled rate, completing in 2 seconds of real time. tracker.timer
    ///   accumulates 1.0 over those 2 real seconds (since `_process(delta)` receives
    ///   scaled deltas). Captured timer at finish: ~1.0.
    /// - A tween with `ignore_time_scale=true` advances at real rate, completing in
    ///   1 second of real time. tracker.timer at finish: ~0.5.
    /// - The two captured timers must differ by ~0.5, proving the flag actually
    ///   affects the tween's processing rate.
    fn test_ignore_time_scale(&mut self) -> PinnedTestTask {
        let sprite = self.sprite.clone();
        let initial = sprite.get_position();
        let target_a = initial + Vector2::new(100.0, 0.0);
        let target_b = initial + Vector2::new(50.0, 0.0);

        // Slow down the engine.
        let mut engine = Engine::singleton();
        engine.set_time_scale(0.5);

        // ignore=true tween (advances at real-time rate).
        let mut ignored = sprite.do_position(target_a, 1.0);
        ignored.set_ignore_time_scale(true);
        let ignored_handle = ignored.register();

        let tracker = RcPtr::clone(&self.time_tracker);

        // Capture timer at finish (synchronously) for both tweens.
        let ignored_timer = RcPtr::new(None::<f64>);
        let slot = ignored_timer.clone();
        let tr = tracker.clone();
        ignored_handle.to_mut().finished_connect(
            move || *slot.to_mut() = Some(tr.timer),
            SpireFlags::ONE_SHOT,
        );

        Box::pin(async move {
            // Wait for the ignored tween (~1s real, ~0.5s scaled).
            while !ignored_handle.is_stopped() {
                next_frame().await;
            }
            let ignored_at_finish = ignored_timer.to_mut().take()
                .expect("ignored tween's finished closure must have fired");
            assert_eq!(sprite.get_position(), target_a);

            // Reset position for control tween.
            let mut sprite_mut = sprite.clone();
            sprite_mut.set_position(initial);

            // Control: ignore_time_scale=false (default), same 1.0s duration.
            // Should advance at scaled rate -> 2s real time -> tracker advances by 1.0.
            let timer_at_control_start = tracker.timer;
            let control_handle = sprite.do_position(target_b, 1.0).register();

            let control_timer = RcPtr::new(None::<f64>);
            let slot = control_timer.clone();
            let tr = tracker.clone();
            control_handle.to_mut().finished_connect(
                move || *slot.to_mut() = Some(tr.timer),
                SpireFlags::ONE_SHOT,
            );

            while !control_handle.is_stopped() {
                next_frame().await;
            }
            let control_at_finish = control_timer.to_mut().take()
                .expect("control tween's finished closure must have fired");
            assert_eq!(sprite.get_position(), target_b);

            // Restore time_scale before any assertion that might panic.
            Engine::singleton().set_time_scale(1.0);

            // Each tween advances tracker by its own duration in scaled time.
            // - ignored: advanced ~0.5 (1s real * 0.5 scale)
            // - control: advanced ~1.0 (2s real * 0.5 scale)
            let control_delta = control_at_finish - timer_at_control_start;
            assert!(
                (ignored_at_finish - 0.5).abs() < 0.1,
                "ignore_time_scale=true: expected ~0.5 scaled-timer advance, got {ignored_at_finish}"
            );
            assert!(
                (control_delta - 1.0).abs() < 0.1,
                "ignore_time_scale=false (control): expected ~1.0 scaled-timer advance, got {control_delta}"
            );
            // The whole point: the two cases must produce DIFFERENT timer advances,
            // by approximately 0.5 (the flag halves the tween's apparent scaled duration).
            let advance_diff = control_delta - ignored_at_finish;
            assert!(
                advance_diff > 0.3,
                "ignore_time_scale must affect timing: ignored advance ~{ignored_at_finish}, \
                 control advance ~{control_delta}, diff {advance_diff}"
            );

            // Cleanup: reset position.
            let mut sprite_mut = sprite.clone();
            sprite_mut.set_position(initial);
        })
    }
}
