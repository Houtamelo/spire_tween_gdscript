//! Shake integration tests run against real Godot nodes and the canonical clock.
use godot::{classes::Control, prelude::*};
use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng};
use spire_tween::{prelude::*, unversioned_api::do_shake_internal};

use super::{templates_test::TemplatesTests, util::PinnedTestTask};

fn near(a: Vector2, b: Vector2) {
    assert!(a.distance_to(b) < 0.0001, "{a} != {b}");
}
fn magnitude(value: Vector2, expected: f32) {
    assert!((value.length() - expected).abs() < 0.0001, "{value} should have magnitude {expected}");
}
fn manual(tween: SpireTween<LerpMethodData<f64>>) -> TweenPtr<LerpMethodData<f64>> {
    TweenPtr::new(tween.with_process_mode(ProcessMode::Manual))
}
fn seeded_rng() -> Xoshiro256PlusPlus { Xoshiro256PlusPlus::seed_from_u64(12345) }

fn shake(node: &Gd<Node2D>, fade: bool, mode: ShakeRandomnessMode, blendable: bool) -> TweenPtr<LerpMethodData<f64>> {
    manual(do_shake_internal(
        node.clone(),
        Node2D::get_position,
        Node2D::set_position,
        1.0,
        Vector2::new(3.0, 4.0),
        4,
        0.0,
        fade,
        mode,
        blendable,
        seeded_rng(),
    ))
}

impl TemplatesTests {
    pub(super) fn test_shake_numerical_fixtures(&mut self) -> PinnedTestTask {
        // These fixtures evaluate the mathematical specification using the first
        // six draws of Xoshiro256PlusPlus seeded with 12345. Expected positions
        // are calculated from the equations, not recorded from the implementation.
        let fixtures = [
            (ShakeRandomnessMode::Full, [
                Vector2::new(-3.7853454846, -3.2666740827),
                Vector2::new(0.9489967272, -3.6279340142),
                Vector2::new(0.1929966527, 2.4925393261),
            ]),
            (ShakeRandomnessMode::Harmonic, [
                Vector2::new(-4.2717486508, -2.5984925368),
                Vector2::new(2.8244572807, 2.4667673319),
                Vector2::new(-0.6085566292, -2.4248007813),
            ]),
        ];
        for (mode, expected) in fixtures {
            let node = Node2D::new_alloc();
            self.base_mut().add_child(&node);
            let mut tween = manual(do_shake_internal(
                node.clone(),
                Node2D::get_position,
                Node2D::set_position,
                1.0,
                Vector2::new(3.0, 4.0),
                4,
                90.0,
                true,
                mode,
                true,
                seeded_rng(),
            ));
            let _ = tween.custom_step(0.05, false);
            near(node.get_position(), expected[0] * 0.5);
            let _ = tween.custom_step(0.05, false);
            near(node.get_position(), expected[0]);
            let _ = tween.custom_step(0.2, false);
            near(node.get_position(), expected[1]);
            let _ = tween.custom_step(0.3, false);
            near(node.get_position(), expected[2]);
            tween.force_complete();
            near(node.get_position(), Vector2::ZERO);
            node.free();
        }
        // An explicitly disabled axis must also remain disabled on Control.
        let node = Control::new_alloc();
        self.base_mut().add_child(&node);
        let tween = manual(do_shake_internal(
            node.clone(),
            Control::get_position,
            Control::set_position,
            1.0,
            Vector2::new(0.0, 4.0),
            2,
            0.0,
            false,
            ShakeRandomnessMode::Full,
            false,
            seeded_rng(),
        ));
        let _ = tween.custom_step(0.5, false);
        near(node.get_position(), Vector2::new(0.0, -4.0));
        let _ = tween.custom_step(0.5, false);
        near(node.get_position(), Vector2::ZERO);
        node.free();
        Box::pin(async {})
    }

    pub(super) fn test_shake_interpolation(&mut self) -> PinnedTestTask {
        for mode in [ShakeRandomnessMode::Full, ShakeRandomnessMode::Harmonic] {
            let node = Node2D::new_alloc();
            self.base_mut().add_child(&node);
            let tween = shake(&node, false, mode, true);
            let _ = tween.custom_step(0.125, false);
            let half = node.get_position();
            magnitude(half, 2.5);
            let _ = tween.custom_step(0.125, false);
            near(node.get_position(), half * 2.0);
            let _ = tween.custom_step(0.125, false);
            near(node.get_position(), Vector2::ZERO);
            let _ = tween.custom_step(0.125, false);
            near(node.get_position(), -half * 2.0);
            let _ = tween.custom_step(0.49999, false);
            let _ = tween.custom_step(0.00002, false);
            near(node.get_position(), Vector2::ZERO);
            assert!(tween.is_stopped());
            node.free();
        }
        Box::pin(async {})
    }

    pub(super) fn test_shake_fade(&mut self) -> PinnedTestTask {
        let node = Node2D::new_alloc();
        self.base_mut().add_child(&node);
        let mut tween = shake(&node, true, ShakeRandomnessMode::Full, true);
        // Four weighted segments end at .1, .3, .6, 1.0. Peak magnitudes
        // decrease by 5/4 at each nonfinal waypoint.
        let _ = tween.custom_step(0.05, false);
        let half = node.get_position();
        magnitude(half, 2.5);
        let _ = tween.custom_step(0.05, false);
        near(node.get_position(), half * 2.0);
        let _ = tween.custom_step(0.2, false);
        near(node.get_position(), half * -1.5);
        let _ = tween.custom_step(0.3, false);
        near(node.get_position(), half);
        tween.loop_finished_clear_connections();
        let _ = tween.custom_step(0.4, false);
        near(node.get_position(), Vector2::ZERO);
        assert!(tween.is_stopped());
        node.free();
        Box::pin(async {})
    }

    pub(super) fn test_shake_blending(&mut self) -> PinnedTestTask {
        let mut node = Node2D::new_alloc();
        self.base_mut().add_child(&node);
        let origin = Vector2::new(10.0, 20.0);
        let movement = Vector2::new(7.0, -4.0);
        let mut a = shake(&node, false, ShakeRandomnessMode::Full, true);
        let mut b = shake(&node, false, ShakeRandomnessMode::Harmonic, true);
        node.set_position(origin);
        let _ = a.custom_step(0.125, false);
        let offset_a = node.get_position() - origin;
        let _ = b.custom_step(0.125, false);
        let offset_b = node.get_position() - origin - offset_a;
        let moved = node.get_position() + movement;
        node.set_position(moved);
        let _ = a.custom_step(0.125, false);
        near(node.get_position(), origin + movement + offset_a * 2.0 + offset_b);
        a.force_complete();
        near(node.get_position(), origin + movement + offset_b);
        b.force_complete();
        near(node.get_position(), origin + movement);

        let mut absolute = shake(&node, false, ShakeRandomnessMode::Full, false);
        node.set_position(origin);
        let _ = absolute.custom_step(0.125, false);
        let half = node.get_position() - origin;
        node.set_position(Vector2::new(100.0, 200.0));
        let _ = absolute.custom_step(0.125, false);
        near(node.get_position(), origin + half * 2.0);
        absolute.force_complete();
        near(node.get_position(), origin);
        node.free();
        Box::pin(async {})
    }

    pub(super) fn test_shake_lifecycle(&mut self) -> PinnedTestTask {
        for blendable in [false, true] {
            let mut node = Node2D::new_alloc();
            self.base_mut().add_child(&node);
            let origin = Vector2::new(8.0, -3.0);
            node.set_position(origin);
            let mut tween = shake(&node, false, ShakeRandomnessMode::Full, blendable);
            tween.set_delay(0.25);
            let _ = tween.custom_step(0.125, false);
            near(node.get_position(), origin);
            let _ = tween.custom_step(0.25, false);
            let half = node.get_position() - origin;
            tween.pause();
            let _ = tween.custom_step(0.4, false);
            near(node.get_position(), origin + half);
            tween.play();
            let _ = tween.custom_step(0.125, false);
            near(node.get_position(), origin + half * 2.0);
            let frozen = node.get_position();
            tween.stop();
            near(node.get_position(), frozen);
            let _ = tween.custom_step(0.5, false);
            tween.force_complete(); // Stopped remains untouched, even explicitly forced.
            near(node.get_position(), frozen);
            tween.play();
            let _ = tween.custom_step(0.375, false);
            magnitude(node.get_position() - origin, 2.5);
            assert_ne!(node.get_position() - origin, half, "replay must generate a fresh path");
            let events = RcPtr::new(Vec::new());
            let stopped_events = events.clone();
            let stopped_node = node.clone();
            tween.stopped_connect(
                move || {
                    near(stopped_node.get_position(), origin);
                    stopped_events.to_mut().push("stopped");
                },
                SpireFlags::empty(),
            );
            let finished_events = events.clone();
            let finished_node = node.clone();
            tween.finished_connect(
                move || {
                    near(finished_node.get_position(), origin);
                    finished_events.to_mut().push("finished");
                },
                SpireFlags::empty(),
            );
            tween.force_complete();
            near(node.get_position(), origin);
            assert_eq!(&**events, &["stopped", "finished"]);
            node.free();
        }
        Box::pin(async {})
    }

    pub(super) fn test_shake_loops(&mut self) -> PinnedTestTask {
        for mode in [LoopMode::Restart, LoopMode::Incremental, LoopMode::Yoyo] {
            let node = Node2D::new_alloc();
            self.base_mut().add_child(&node);
            // A fixed seed makes both path reuse and fresh replay reproducible.
            let mut tween = manual(do_shake_internal(
                node.clone(),
                Node2D::get_position,
                Node2D::set_position,
                1.0,
                Vector2::new(3.0, 4.0),
                4,
                90.0,
                false,
                ShakeRandomnessMode::Full,
                true,
                seeded_rng(),
            ));
            tween.set_loops(4, mode);
            tween.loop_finished_clear_connections();
            let _ = tween.custom_step(0.125, false);
            let first = node.get_position();
            let _ = tween.custom_step(0.25, false);
            let second = node.get_position();
            let _ = tween.custom_step(0.625, false);
            near(node.get_position(), Vector2::ZERO);
            for loop_index in 1..4 {
                if mode == LoopMode::Yoyo && loop_index % 2 == 1 {
                    let _ = tween.custom_step(0.625, false);
                    near(node.get_position(), second);
                    let _ = tween.custom_step(0.25, false);
                    near(node.get_position(), first);
                    let _ = tween.custom_step(0.125, false);
                } else {
                    let _ = tween.custom_step(0.125, false);
                    near(node.get_position(), first);
                    let _ = tween.custom_step(0.25, false);
                    near(node.get_position(), second);
                    let _ = tween.custom_step(0.625, false);
                }
                near(node.get_position(), Vector2::ZERO);
            }
            assert!(tween.is_stopped());
            tween.play();
            let _ = tween.custom_step(0.125, false);
            let fresh_first = node.get_position();
            let _ = tween.custom_step(0.25, false);
            let fresh_second = node.get_position();
            assert!(first != fresh_first || second != fresh_second, "replay must regenerate for {mode:?}");
            tween.force_complete();
            near(node.get_position(), Vector2::ZERO);
            node.free();
        }
        Box::pin(async {})
    }

    pub(super) fn test_shake_phase_and_cleanup(&mut self) -> PinnedTestTask {
        for blendable in [false, true] {
            for forced in [false, true] {
                let mut node = Node2D::new_alloc();
                self.base_mut().add_child(&node);
                let origin = Vector2::new(10.0, -3.0);
                node.set_position(origin);
                let mut tween = shake(&node, false, ShakeRandomnessMode::Full, blendable);
                let weight = RcPtr::new(0.125_f64);
                let ease_weight = weight.clone();
                tween.set_ease(EaseKind::Callable(Callable::from_fn("shake_phase", move |_| {
                    (*ease_weight).to_variant()
                })));
                let _ = tween.custom_step(0.01, false);
                let first = node.get_position() - origin;
                magnitude(first, 2.5);
                for (value, factor) in [
                    (1.125, 1.0),
                    (-0.875, 1.0),
                    (0.5, -2.0),
                    (1.5, -2.0),
                    (-0.5, -2.0),
                    (0.75, 2.0),
                    (-0.25, 2.0),
                    (0.0, 0.0),
                    (1.0, 0.0),
                    (2.0, 0.0),
                    (-1.0, 0.0),
                ] {
                    *weight.to_mut() = value;
                    let _ = tween.custom_step(0.01, false);
                    near(node.get_position(), origin + first * factor);
                }
                // A nonzero eased endpoint proves that restoration comes from
                // the finish connection, after stopped and before later finish handlers.
                *weight.to_mut() = 0.125;
                let events = RcPtr::new(Vec::new());
                let stopped_events = events.clone();
                let stopped_node = node.clone();
                tween.stopped_connect(
                    move || {
                        near(stopped_node.get_position(), origin + first);
                        stopped_events.to_mut().push("stopped");
                    },
                    SpireFlags::empty(),
                );
                let finished_events = events.clone();
                let finished_node = node.clone();
                tween.finished_connect(
                    move || {
                        near(finished_node.get_position(), origin);
                        finished_events.to_mut().push("finished");
                    },
                    SpireFlags::empty(),
                );
                if forced {
                    tween.force_complete();
                } else {
                    let _ = tween.custom_step(1.0, false);
                }
                assert_eq!(&**events, &["stopped", "finished"]);
                near(node.get_position(), origin);
                node.free();
            }
        }
        let node = Node2D::new_alloc();
        self.base_mut().add_child(&node);
        let mut tween = shake(&node, false, ShakeRandomnessMode::Full, true);
        tween.set_ease(EaseKind::Callable(Callable::from_fn("shake_endpoint", |_| 0.125_f64.to_variant())));
        let _ = tween.custom_step(0.01, false);
        let offset = node.get_position();
        tween.finished_clear_connections();
        tween.force_complete();
        // Removing the template's connection deliberately removes its cleanup.
        near(node.get_position(), offset);
        magnitude(offset, 2.5);
        node.free();
        Box::pin(async {})
    }

    pub(super) fn test_shake_owners_and_inputs(&mut self) -> PinnedTestTask {
        for forced in [false, true] {
            let node = Node2D::new_alloc();
            self.base_mut().add_child(&node);
            let mut tween = shake(&node, false, ShakeRandomnessMode::Full, true);
            let events = RcPtr::new([0, 0]);
            let stopped = events.clone();
            tween.stopped_connect(move || stopped.to_mut()[0] += 1, SpireFlags::empty());
            let finished = events.clone();
            tween.finished_connect(move || finished.to_mut()[1] += 1, SpireFlags::empty());
            node.free();
            if forced {
                tween.force_complete();
            } else {
                let _ = tween.custom_step(0.125, false);
            }
            assert!(tween.is_stopped());
            assert_eq!(*events, [1, 0]);
        }
        for forced in [false, true] {
            let node = Node2D::new_alloc();
            self.base_mut().add_child(&node);
            let mut tween = shake(&node, false, ShakeRandomnessMode::Full, true);
            let stopped_node = node.clone();
            tween.stopped_connect(move || stopped_node.clone().free(), SpireFlags::empty());
            let finished = RcPtr::new(0);
            let finished_events = finished.clone();
            tween.finished_connect(move || *finished_events.to_mut() += 1, SpireFlags::empty());
            if forced {
                tween.force_complete();
            } else {
                let _ = tween.custom_step(1.0, false);
            }
            assert!(!node.is_instance_valid());
            assert_eq!(*finished, 1);
        }
        let node = Control::new_alloc();
        self.base_mut().add_child(&node);
        for vibrato in [-1, 0, 1] {
            let mut tween = manual(node.do_shake(
                1.0,
                Vector2::new(5.0, 0.0),
                vibrato,
                180.0,
                false,
                ShakeRandomnessMode::Harmonic,
                true,
            ));
            let _ = tween.custom_step(0.25, false);
            magnitude(node.get_position(), 2.5);
            assert_eq!(node.get_position().y, 0.0);
            tween.force_complete();
            near(node.get_position(), Vector2::ZERO);
        }
        // The count is capped at 65,536: this samples halfway to the first
        // waypoint, rather than the peak an uncapped 131,072 would produce.
        let mut bounded =
            manual(node.do_shake(1.0, Vector2::new(5.0, 0.0), 131_072, 0.0, false, ShakeRandomnessMode::Full, true));
        let _ = bounded.custom_step(0.5 / 65_536.0, false);
        magnitude(node.get_position(), 2.5);
        bounded.force_complete();
        near(node.get_position(), Vector2::ZERO);
        let expected_node = Control::new_alloc();
        self.base_mut().add_child(&expected_node);
        for (duration, strength, randomness, expected_strength, expected_randomness) in [
            (0.0, Vector2::ONE, 90.0, Vector2::ONE, 90.0),
            (-1.0, Vector2::ONE, 90.0, Vector2::ONE, 90.0),
            (f64::NAN, Vector2::ONE, 90.0, Vector2::ONE, 90.0),
            (f64::INFINITY, Vector2::ONE, 90.0, Vector2::ONE, 90.0),
            (f64::NEG_INFINITY, Vector2::ONE, 90.0, Vector2::ONE, 90.0),
            (1.0, Vector2::new(-1.0, 4.0), 90.0, Vector2::new(0.0, 4.0), 90.0),
            (1.0, Vector2::new(3.0, -1.0), 90.0, Vector2::new(3.0, 0.0), 90.0),
            (1.0, Vector2::new(f32::NAN, 4.0), 90.0, Vector2::new(0.0, 4.0), 90.0),
            (1.0, Vector2::new(3.0, f32::INFINITY), 90.0, Vector2::new(3.0, 0.0), 90.0),
            (1.0, Vector2::new(f32::NEG_INFINITY, f32::NAN), 90.0, Vector2::ZERO, 90.0),
            (1.0, Vector2::ONE, -1.0, Vector2::ONE, 0.0),
            (1.0, Vector2::ONE, 181.0, Vector2::ONE, 180.0),
            (1.0, Vector2::ONE, f32::NAN, Vector2::ONE, 90.0),
            (1.0, Vector2::ONE, f32::INFINITY, Vector2::ONE, 90.0),
            (1.0, Vector2::ONE, f32::NEG_INFINITY, Vector2::ONE, 90.0),
            (-1.0, Vector2::new(-1.0, 4.0), f32::NAN, Vector2::new(0.0, 4.0), 90.0),
        ] {
            // Compare real-node motion against explicitly normalized parameters
            // using independently owned RNGs initialized with the same seed.
            let tween = manual(do_shake_internal(
                node.clone(),
                Control::get_position,
                Control::set_position,
                duration,
                strength,
                4,
                randomness,
                true,
                ShakeRandomnessMode::Full,
                true,
                seeded_rng(),
            ));
            assert_eq!(tween.get_duration(), 1.0);
            let expected = manual(do_shake_internal(
                expected_node.clone(),
                Control::get_position,
                Control::set_position,
                1.0,
                expected_strength,
                4,
                expected_randomness,
                true,
                ShakeRandomnessMode::Full,
                true,
                seeded_rng(),
            ));
            near(node.get_position(), Vector2::ZERO);
            for step in [0.05, 0.05, 0.2, 0.3, 0.4] {
                let _ = tween.custom_step(step, false);
                let _ = expected.custom_step(step, false);
                near(node.get_position(), expected_node.get_position());
            }
            assert!(tween.is_stopped());
            near(node.get_position(), Vector2::ZERO);
        }
        expected_node.free();
        let mut zero = manual(node.do_shake(1.0, Vector2::ZERO, -1, 0.0, true, ShakeRandomnessMode::Full, true));
        let _ = zero.custom_step(0.5, false);
        zero.force_complete();
        near(node.get_position(), Vector2::ZERO);
        let mut dead = manual(node.do_shake(1.0, Vector2::ONE, 4, 90.0, true, ShakeRandomnessMode::Full, true));
        node.free();
        dead.force_complete();
        assert!(dead.is_stopped());
        Box::pin(async {})
    }
}
