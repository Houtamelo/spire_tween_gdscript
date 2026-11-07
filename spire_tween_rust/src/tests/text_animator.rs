//! Character animator fixtures use real RichTextLabel rendering and checked tweens.
use godot::{classes::*, prelude::*};
use spire_tween::prelude::*;

use super::{
    misc::MiscTests,
    util::{PinnedTestTask, next_frame},
};

type Driver = TweenPtr<LerpMethodData<f64>>;
fn manual(t: SpireTween<LerpMethodData<f64>>) -> Driver {
    TweenPtr::new(
        t.with_process_mode(ProcessMode::Manual)
            .with_ease(EaseKind::Basic(Ease::Linear)),
    )
}
fn step(t: &Driver, delta: f64) { let _ = t.custom_step(delta, false); }
fn overshoot_ease() -> EaseKind {
    EaseKind::Callable(Callable::from_fn("character_overshoot", |args| {
        let p = args[0].to::<f64>();
        (p + 3.0 * p * (1.0 - p)).to_variant()
    }))
}
fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 0.0002, "{a} != {b}");
}
fn v2(a: Vector2, b: Vector2) { near(a.distance_to(b) as f64, 0.0); }
fn v3(a: Vector3, b: Vector3) { near(a.distance_to(b) as f64, 0.0); }
fn label(parent: &mut Node2D, text: &str, bbcode: bool) -> Gd<SpireRichTextLabel> {
    let mut n = SpireRichTextLabel::new_alloc();
    n.set_use_bbcode(bbcode);
    n.set_text(text);
    n.set_size(Vector2::new(500.0, 140.0));
    n.add_theme_color_override("default_color", Color::WHITE);
    n.add_theme_font_size_override("normal_font_size", 36);
    parent.add_child(&n);
    n
}

impl MiscTests {
    pub(super) fn test_characters_translation(&mut self) -> PinnedTestTask {
        let mut server = TranslationServer::singleton();
        let locale = server.get_locale();
        server.set_locale("en");
        let mut translation = Translation::new_gd();
        translation.set_locale("es");
        translation.add_message("SPIRE_CHAR_TRANSLATION", "[color=red]猫AB[/color]");
        server.add_translation(&translation);
        let n = label(&mut self.base_mut(), "SPIRE_CHAR_TRANSLATION", true);
        let a = TextAnimator::new(n.clone());
        server.set_locale("es");
        Box::pin(async move {
            for _ in 0..4 {
                next_frame().await;
            }
            let count = a.character_count();
            let source = n.get_text();
            let parsed = n.get_parsed_text();
            let sample = a.render_sample_for_test(0);
            server.set_locale(&locale);
            server.remove_translation(&translation);
            n.free();
            assert_eq!(count, 3);
            assert_eq!(source, GString::from("SPIRE_CHAR_TRANSLATION"));
            assert_eq!(parsed, GString::from("猫AB"));
            assert_eq!(sample.expect("translated text must retain its effect").1, Color::RED);
        })
    }

    pub(super) fn test_characters_rotation(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "A", false);
        let a = TextAnimator::new(n.clone());
        let angle = std::f32::consts::TAU * 2.0 + 0.8;
        for mode in [RotateMode::Fast, RotateMode::FastBeyond360] {
            a.reset();
            let t = manual(a.do_rotate_char(0, Vector3::new(0.0, 0.0, angle), 1.0, mode));
            step(&t, 0.5);
            near(a.get_char_rotation(0).z as f64, if mode == RotateMode::Fast { 0.4 } else { angle as f64 / 2.0 });
        }
        let from = Vector3::new(0.2, 0.3, 0.4);
        let by = Vector3::new(0.3, -0.4, 0.0);
        for mode in [RotateMode::WorldAxisAdd, RotateMode::LocalAxisAdd] {
            a.set_char_rotation(0, from);
            let mut t = manual(a.do_rotate_char(0, by, 1.0, mode));
            t.force_complete();
            let expected = if mode == RotateMode::WorldAxisAdd {
                Quaternion::from_euler(by) * Quaternion::from_euler(from)
            } else {
                Quaternion::from_euler(from) * Quaternion::from_euler(by)
            };
            assert!(Quaternion::from_euler(a.get_char_rotation(0)).dot(expected).abs() > 0.99999);
        }
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_effect_refresh(&mut self) -> PinnedTestTask {
        let mut n = label(&mut self.base_mut(), "A", false);
        let a = TextAnimator::new(n.clone());
        for blendable in [true, false] {
            a.reset();
            let mut t = manual(a.do_punch_char_offset(0, Vector2::new(4.0, 0.0), 1.0, 4, 1.0, blendable));
            step(&t, 0.1);
            v2(a.get_char_offset(0), Vector2::new(4.0, 0.0));
            n.set_text(if blendable { "B" } else { "C" });
            v2(a.get_char_offset(0), Vector2::ZERO);
            step(&t, 0.2);
            v2(a.get_char_offset(0), Vector2::new(-3.0, 0.0));
            a.reset();
            t.force_complete();
            v2(a.get_char_offset(0), Vector2::ZERO);
        }
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_automatic(&mut self) -> PinnedTestTask {
        let mut n = label(&mut self.base_mut(), "A", false);
        let a = TextAnimator::new(n.clone());
        let mut t = a.do_offset_char(0, Vector2::new(8.0, 0.0), 0.05).register();
        let events = RcPtr::new([0, 0]);
        t.stopped_connect(
            {
                let mut e = events.clone();
                move || e[0] += 1
            },
            SpireFlags::empty(),
        );
        t.finished_connect(
            {
                let mut e = events.clone();
                move || e[1] += 1
            },
            SpireFlags::empty(),
        );
        Box::pin(async move {
            for _ in 0..60 {
                next_frame().await;
                if t.get_state() == TweenState::Stopped {
                    break;
                }
            }
            assert_eq!(*events, [1, 1]);
            v2(a.get_char_offset(0), Vector2::new(8.0, 0.0));
            n.set_text("B");
            for _ in 0..4 {
                next_frame().await;
            }
            // The sample must already exist: a getter must not be the action
            // that discovers the edit, resets state and installs the new scope.
            let sample = a
                .render_sample_for_test(0)
                .expect("label must maintain the effect without live tweens");
            v2(sample.0.a, Vector2::RIGHT);
            v2(a.get_char_offset(0), Vector2::ZERO);
            n.free();
        })
    }

    pub(super) fn test_characters_label(&mut self) -> PinnedTestTask {
        let mut n = label(&mut self.base_mut(), "[b]AB[/b]", false);
        let a = TextAnimator::new(n.clone());
        let children = n.get_child_count();
        assert_eq!(children, 0, "character animation must not add child nodes");
        let plain = RichTextLabel::new_alloc();
        assert_eq!(
            n.get_child_count_ex().include_internal(true).done(),
            plain.get_child_count_ex().include_internal(true).done()
        );
        plain.free();
        assert_eq!(n.get_process_mode(), godot::classes::node::ProcessMode::INHERIT);
        let b = TextAnimator::new(n.clone());
        assert_eq!(n.get_child_count(), children);
        assert_eq!(n.get_text(), GString::from("[b]AB[/b]"));
        assert!(!n.is_using_bbcode());
        assert_eq!(a.character_count(), 9);
        n.set_use_bbcode(true);
        assert_eq!(a.character_count(), 2);
        assert_eq!(n.get_parsed_text(), GString::from("AB"));
        a.set_char_offset(1, Vector2::ONE);
        v2(b.get_char_offset(1), Vector2::ONE);
        let copy = n
            .duplicate()
            .expect("character state must not block Node::duplicate")
            .cast::<SpireRichTextLabel>();
        self.base_mut().add_child(&copy);
        let other = TextAnimator::new(copy.clone());
        other.set_char_offset(1, Vector2::new(4.0, 0.0));
        v2(a.get_char_offset(1), Vector2::ONE);
        v2(other.get_char_offset(1), Vector2::new(4.0, 0.0));
        copy.free();
        assert!(!a.is_valid_char(-1));
        assert!(!a.is_valid_char(2));
        n.free();
        assert_eq!(a.character_count(), 0);
        Box::pin(async {})
    }

    pub(super) fn test_characters_channels(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "ABC", false);
        let a = TextAnimator::new(n.clone());
        a.set_char_offset(0, Vector2::new(2.0, 4.0));
        let mut offset = manual(a.do_offset_char(0, Vector2::new(10.0, 12.0), 1.0));
        let mut scale = manual(a.do_scale_char(1, Vector2::new(3.0, 5.0), 1.0));
        a.set_char_color(2, Color::from_rgba(1.0, 0.0, 0.0, 0.8));
        let mut color = manual(a.do_color_char(2, Color::from_rgba(0.0, 0.0, 1.0, 0.4), 1.0));
        for t in [&offset, &scale, &color] {
            step(t, 0.5);
        }
        v2(a.get_char_offset(0), Vector2::new(6.0, 8.0));
        v2(a.get_char_scale(1), Vector2::new(2.0, 3.0));
        near(a.get_char_color(2).r as f64, 0.5);
        near(a.get_char_alpha(2), 0.6);
        let mut fade = manual(a.do_fade_char(2, 0.0, 1.0));
        step(&fade, 0.5);
        near(a.get_char_alpha(2), 0.3);
        near(a.get_char_color(2).r as f64, 0.5);
        for t in [&mut offset, &mut scale, &mut color, &mut fade] {
            t.force_complete();
        }
        v2(a.get_char_offset(0), Vector2::new(10.0, 12.0));
        v2(a.get_char_scale(1), Vector2::new(3.0, 5.0));
        near(a.get_char_alpha(2), 0.0);
        a.set_char_alpha(2, 1.0);
        let mut overshoot = manual(a.do_fade_char(2, 0.0, 1.0));
        overshoot.set_ease(overshoot_ease());
        step(&overshoot, 0.5);
        assert!(a.get_char_alpha(2) < 0.0, "valid easing overshoot must not be treated as invalid input");
        overshoot.force_complete();
        near(a.get_char_alpha(2), 0.0);
        a.set_char_color(2, Color::WHITE);
        let mut overshoot = manual(a.do_color_char(2, Color::from_rgba(0.0, 0.0, 1.0, 0.0), 1.0));
        overshoot.set_ease(overshoot_ease());
        step(&overshoot, 0.5);
        assert!(a.get_char_alpha(2) < 0.0);
        overshoot.force_complete();
        near(a.get_char_alpha(2), 0.0);
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_refresh(&mut self) -> PinnedTestTask {
        let mut n = label(&mut self.base_mut(), "ABC", false);
        let a = TextAnimator::new(n.clone());
        let mut t = manual(a.do_offset_char(2, Vector2::new(8.0, 0.0), 1.0));
        step(&t, 0.25);
        v2(a.get_char_offset(2), Vector2::new(2.0, 0.0));
        n.set_text("12345");
        v2(a.get_char_offset(2), Vector2::ZERO);
        step(&t, 0.25);
        v2(a.get_char_offset(2), Vector2::new(4.0, 0.0));
        near(t.get_total_elapsed_time(), 0.5);
        n.set_size(Vector2::new(100.0, 100.0));
        v2(a.get_char_offset(2), Vector2::new(4.0, 0.0));
        let events = RcPtr::new([0, 0, 0]);
        t.stopped_connect(
            {
                let mut e = events.clone();
                move || e[0] += 1
            },
            SpireFlags::empty(),
        );
        t.finished_connect(
            {
                let mut e = events.clone();
                move || e[1] += 1
            },
            SpireFlags::empty(),
        );
        t.loop_finished_connect(
            {
                let mut e = events.clone();
                move || e[2] += 1
            },
            SpireFlags::empty(),
        );
        n.set_text("A");
        assert!(matches!(t.custom_step(1.0, false), TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
        assert_eq!(*events, [1, 0, 0]);
        assert_eq!(t.get_state(), TweenState::Stopped);
        let mut invalid = manual(a.do_offset_char(2, Vector2::ONE, 1.0));
        assert_eq!(invalid.get_state(), TweenState::Stopped);
        invalid.play();
        invalid.force_complete();
        assert_eq!(invalid.get_state(), TweenState::Stopped);
        assert_eq!(n.get_text(), GString::from("A"));
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_lifecycle(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "AB", false);
        let a = TextAnimator::new(n.clone());
        let mut t = manual(a.do_offset_char(0, Vector2::new(8.0, 0.0), 1.0));
        step(&t, 0.25);
        t.pause();
        step(&t, 0.5);
        v2(a.get_char_offset(0), Vector2::new(2.0, 0.0));
        t.play();
        step(&t, 0.25);
        t.stop();
        t.force_complete();
        v2(a.get_char_offset(0), Vector2::new(4.0, 0.0));
        t.play();
        step(&t, 0.25);
        a.reset();
        v2(a.get_char_offset(0), Vector2::ZERO);
        step(&t, 0.25);
        v2(a.get_char_offset(0), Vector2::new(4.0, 0.0));
        let events = RcPtr::new([0, 0]);
        t.stopped_connect(
            {
                let mut e = events.clone();
                move || e[0] += 1
            },
            SpireFlags::empty(),
        );
        t.finished_connect(
            {
                let mut e = events.clone();
                move || e[1] += 1
            },
            SpireFlags::empty(),
        );
        n.free();
        step(&t, 1.0);
        assert_eq!(t.get_state(), TweenState::Stopped);
        assert_eq!(*events, [1, 0]);
        let n = label(&mut self.base_mut(), "A", false);
        let a = TextAnimator::new(n.clone());
        let t = manual(a.do_fade_char(0, 0.0, 1.0));
        n.free();
        step(&t, 1.0);
        assert_eq!(t.get_state(), TweenState::Stopped);
        Box::pin(async {})
    }

    pub(super) fn test_characters_loops_sequence(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "A", false);
        let a = TextAnimator::new(n.clone());
        for mode in [LoopMode::Restart, LoopMode::Yoyo, LoopMode::Incremental] {
            a.reset();
            let mut t = manual(a.do_offset_char(0, Vector2::new(8.0, 0.0), 1.0).with_loops(2, mode));
            step(&t, 1.0);
            step(&t, 0.25);
            let x = match mode {
                LoopMode::Restart => 2.0,
                LoopMode::Yoyo => 6.0,
                LoopMode::Incremental => 10.0,
            };
            near(a.get_char_offset(0).x as f64, x);
            t.stop();
        }
        a.reset();
        let mut child = a
            .do_offset_char(0, Vector2::new(8.0, 0.0), 1.0)
            .with_ease(EaseKind::Basic(Ease::Linear));
        child.set_speed_scale(2.0);
        let mut sequence = SpireTween::<Sequence>::new().with_process_mode(ProcessMode::Manual);
        sequence.append(child);
        sequence.append(a.do_offset_char(0, Vector2::new(16.0, 0.0), 1.0));
        sequence.set_default_ease(Ease::Linear);
        let seq = TweenPtr::new(sequence);
        let _ = seq.custom_step(0.75, false);
        near(a.get_char_offset(0).x as f64, 10.0);
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_effects(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "A", false);
        let a = TextAnimator::new(n.clone());
        let mut effects = [
            manual(a.do_punch_char_offset(0, Vector2::new(4.0, 0.0), 1.0, 4, 1.0, true)),
            manual(a.do_punch_char_scale(0, Vector2::new(0.0, 2.0), 1.0, 4, 1.0, true)),
            manual(a.do_punch_char_rotation(0, Vector3::new(0.3, 0.0, 0.0), 1.0, 4, 1.0, true)),
        ];
        for t in &effects {
            step(t, 0.1);
        }
        v2(a.get_char_offset(0), Vector2::new(4.0, 0.0));
        v2(a.get_char_scale(0), Vector2::new(1.0, 3.0));
        v3(a.get_char_rotation(0), Vector3::new(0.3, 0.0, 0.0));
        for t in &mut effects {
            t.force_complete();
        }
        v2(a.get_char_offset(0), Vector2::ZERO);
        v2(a.get_char_scale(0), Vector2::ONE);
        v3(a.get_char_rotation(0), Vector3::ZERO);
        let mut shakes = [
            manual(a.do_shake_char_offset(
                0,
                1.0,
                Vector2::new(4.0, 0.0),
                4,
                0.0,
                false,
                ShakeRandomnessMode::Full,
                true,
            )),
            manual(a.do_shake_char_scale(
                0,
                1.0,
                Vector2::new(0.0, 2.0),
                4,
                0.0,
                false,
                ShakeRandomnessMode::Harmonic,
                true,
            )),
            manual(a.do_shake_char_rotation(
                0,
                1.0,
                Vector3::new(0.3, 0.0, 0.0),
                4,
                0.0,
                false,
                ShakeRandomnessMode::Full,
                true,
            )),
        ];
        for t in &shakes {
            step(t, 0.125);
        }
        assert!(a.get_char_offset(0).length() > 0.00001);
        assert!((a.get_char_scale(0) - Vector2::ONE).length() > 0.00001);
        assert!(a.get_char_rotation(0).length() > 0.00001);
        for t in &mut shakes {
            t.force_complete();
        }
        v2(a.get_char_offset(0), Vector2::ZERO);
        v2(a.get_char_scale(0), Vector2::ONE);
        v3(a.get_char_rotation(0), Vector3::ZERO);
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_invalid(&mut self) -> PinnedTestTask {
        let mut n = label(&mut self.base_mut(), "A", false);
        let a = TextAnimator::new(n.clone());
        let mut t = manual(a.do_offset_char(0, Vector2::new(real::NAN, 4.0), f64::NAN));
        near(t.get_duration(), 1.0);
        t.force_complete();
        v2(a.get_char_offset(0), Vector2::new(0.0, 4.0));
        a.set_char_alpha(0, 3.0);
        near(a.get_char_alpha(0), 1.0);
        a.set_char_offset(-1, Vector2::ONE);
        v2(a.get_char_offset(-1), Vector2::ZERO);
        v2(a.get_char_scale(2), Vector2::ONE);
        v2(a.get_char_offset(0), Vector2::new(0.0, 4.0));
        a.set_char_rotation(0, Vector3::new(real::NAN, 0.0, 0.0));
        v3(a.get_char_rotation(0), Vector3::ZERO);
        let mut t =
            manual(a.do_shake_char_offset(0, 1.0, Vector2::ONE, 0, real::NAN, false, ShakeRandomnessMode::Full, true));
        n.set_text("");
        t.force_complete();
        assert_eq!(t.get_state(), TweenState::Stopped);
        let t = manual(a.do_fade_char(-1, 0.0, 1.0));
        assert_eq!(t.get_state(), TweenState::Stopped);
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_render(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "[color=red]A[/color]B", true);
        let a = TextAnimator::new(n.clone());
        Box::pin(async move {
            for _ in 0..4 {
                next_frame().await;
            }
            let original = a
                .render_sample_for_test(0)
                .expect("engine must invoke the real RichTextEffect");
            near(original.1.r as f64, 1.0);
            near(original.1.g as f64, 0.0);
            near(a.get_char_color(0).r as f64, 1.0);
            a.set_char_offset(0, Vector2::new(10.0, 20.0));
            a.set_char_scale(0, Vector2::new(2.0, 3.0));
            a.set_char_rotation(0, Vector3::new(0.0, std::f32::consts::FRAC_PI_3, 0.0));
            a.set_char_color(0, Color::BLUE);
            a.set_char_alpha(0, 0.25);
            for _ in 0..3 {
                next_frame().await;
            }
            let sample = a.render_sample_for_test(0).unwrap();
            v2(sample.0.a, Vector2::RIGHT);
            v2(sample.0.b, Vector2::DOWN * 3.0);
            v2(sample.0.origin - original.0.origin, Vector2::new(10.0, 20.0));
            near(sample.1.b as f64, 1.0);
            near(sample.1.a as f64, 0.25);
            let other = a.render_sample_for_test(1).unwrap();
            v2(other.0.a, Vector2::RIGHT);
            v2(other.0.b, Vector2::DOWN);
            a.reset();
            for _ in 0..3 {
                next_frame().await;
            }
            let reset = a.render_sample_for_test(0).unwrap();
            v2(reset.0.a, Vector2::RIGHT);
            v2(reset.0.origin, original.0.origin);
            near(reset.1.r as f64, 1.0);
            a.set_char_pivot(0, Vector2::new(10.0, -6.0));
            a.set_char_rotation(0, Vector3::new(std::f32::consts::FRAC_PI_3, 0.0, 0.0));
            for _ in 0..3 {
                next_frame().await;
            }
            let tilted = a.render_sample_for_test(0).unwrap().0;
            v2(tilted.a, Vector2::RIGHT);
            v2(tilted.b, Vector2::DOWN * 0.5);
            v2(tilted.origin - original.0.origin, Vector2::new(0.0, -3.0));
            a.set_char_pivot(0, Vector2::ZERO);
            a.set_char_rotation(0, Vector3::new(std::f32::consts::FRAC_PI_3, std::f32::consts::FRAC_PI_3, 0.0));
            for _ in 0..3 {
                next_frame().await;
            }
            let combined = a.render_sample_for_test(0).unwrap().0;
            v2(combined.a, Vector2::new(0.5, 0.0));
            v2(combined.b, Vector2::new(0.75, 0.5));
            a.set_char_rotation(0, Vector3::new(0.0, 0.0, std::f32::consts::FRAC_PI_2));
            for _ in 0..3 {
                next_frame().await;
            }
            let rotated = a.render_sample_for_test(0).unwrap().0;
            v2(rotated.a, Vector2::DOWN);
            v2(rotated.b, Vector2::LEFT);
            n.free();
        })
    }

    pub(super) fn test_characters_unicode_render(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "e\u{301}אב", false);
        let a = TextAnimator::new(n.clone());
        assert_eq!(a.character_count(), 4);
        a.set_char_offset(1, Vector2::new(12.0, 3.0));
        Box::pin(async move {
            for _ in 0..4 {
                next_frame().await;
            }
            assert!(a.render_sample_for_test(0).is_some());
            v2(a.get_char_offset(0), Vector2::new(12.0, 3.0));
            v2(a.get_char_offset(1), Vector2::new(12.0, 3.0));
            a.set_char_scale(0, Vector2::splat(2.0));
            v2(a.get_char_scale(1), Vector2::splat(2.0));
            assert!(a.is_valid_char(2));
            assert!(a.render_sample_for_test(2).is_some());
            n.free();
        })
    }
}
