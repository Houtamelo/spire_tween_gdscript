//! Character animator fixtures use real RichTextLabel rendering and checked tweens.
use godot::{classes::*, prelude::*};
use spire_tween::prelude::*;

use super::{
    misc::MiscTests,
    util::{PinnedTestTask, next_frame},
};

type Driver = TweenPtr<CharTweenData>;
fn manual(t: SpireTween<CharTweenData>) -> Driver {
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
    pub(super) fn test_characters_ease_kind(&mut self) -> PinnedTestTask {
        let value = EaseKind::Basic(Ease::Linear);
        near(value.sample(0.5), 0.5);
        let encoded = value.to_variant();
        let decoded = encoded.try_to::<EaseKind>().expect("basic curve round trip");
        assert!(matches!(decoded, EaseKind::Basic(Ease::Linear)));
        let basic = Ease::OutQuad.to_variant().to::<EaseKind>();
        near(basic.sample(0.5), 0.75);
        let mut curve = Curve::new_gd();
        curve.add_point(Vector2::new(0.0, 0.25));
        curve.add_point(Vector2::new(1.0, 0.25));
        let value = curve.to_variant().to::<EaseKind>();
        assert!(matches!(&value, EaseKind::GodotCurve(current) if current == &curve));
        near(value.sample(0.5), 0.25);
        assert_eq!(value.to_variant().to::<Gd<Curve>>(), curve);
        let callable = Callable::from_fn("character_delays", |args| {
            let x = args[0].to::<f64>();
            (x * 0.5).to_variant()
        });
        let from_callable = callable.to_variant().to::<EaseKind>();
        near(from_callable.sample(0.5), 0.25);
        assert_eq!(from_callable.to_variant().to::<Callable>(), callable);
        for invalid in [
            Variant::nil(),
            999_i64.to_variant(),
            "invalid".to_variant(),
            Resource::new_gd().to_variant(),
        ] {
            assert!(invalid.try_to::<EaseKind>().is_err());
        }
        let n = label(&mut self.base_mut(), "ABC", false);
        let mut t = manual(n.do_offset_chars(0, -1, Vector2::new(10.0, 0.0), 1.0, EaseKind::Basic(Ease::Linear)));
        near(t.get_duration(), 2.0);
        step(&t, 0.5);
        v2(SpireRichTextLabel::get_char_offset(n.clone(), 0), Vector2::new(5.0, 0.0));
        v2(SpireRichTextLabel::get_char_offset(n.clone(), 1), Vector2::ZERO);
        t.stop();
        SpireRichTextLabel::reset(n.clone());
        let t = manual(n.do_offset_chars(0, -1, Vector2::new(10.0, 0.0), 1.0, from_callable));
        near(t.get_duration(), 1.5);
        step(&t, 0.5);
        v2(SpireRichTextLabel::get_char_offset(n.clone(), 1), Vector2::new(2.5, 0.0));
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_ranges(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "ABC", false);
        let mut curve = Curve::new_gd();
        curve.add_point(Vector2::ZERO);
        curve.add_point(Vector2::ONE);
        curve.set_point_right_tangent(0, 1.0);
        curve.set_point_left_tangent(1, 1.0);
        // Borrowed-label constructors must also work without reborrowing Rust state.
        let tween = n
            .bind()
            .do_offset_chars(0, -1, Vector2::new(10.0, 0.0), 1.0, EaseKind::GodotCurve(curve.clone()));
        let t = manual(tween);
        let any: AnyTween = t.clone().into();
        assert!(matches!(any, AnyTween::Char(_)));
        let weak = any.downgrade();
        assert!(weak.upgrade().is_some());
        near(t.get_character_duration(), 1.0);
        near(t.get_duration(), 2.0);
        step(&t, 0.25);
        v2(SpireRichTextLabel::get_char_offset(n.clone(), 0), Vector2::new(2.5, 0.0));
        v2(SpireRichTextLabel::get_char_offset(n.clone(), 1), Vector2::ZERO);
        // Editing the Curve does not change this playback's sampled delays.
        curve.set_point_value(1, 0.0);
        step(&t, 1.0);
        assert_eq!(t.get_loops_finished(), 0);
        v2(SpireRichTextLabel::get_char_offset(n.clone(), 2), Vector2::new(2.5, 0.0));
        step(&t, 0.75);
        assert_eq!(t.get_loops_finished(), 1);
        assert!(t.is_stopped());
        let mut sequence = TweenSequence::new();
        sequence.append(n.do_scale_chars(0, -1, Vector2::splat(2.0), 0.5, EaseKind::Basic(Ease::Linear)));
        let mut sequence = TweenPtr::new(sequence.with_process_mode(ProcessMode::Manual));
        sequence.force_complete();
        v2(SpireRichTextLabel::get_char_scale(n.clone(), 2), Vector2::splat(2.0));
        let carried = manual(
            n.do_offset_chars(
                0,
                2,
                Vector2::ONE,
                1.0,
                EaseKind::GodotCurve({
                    let mut curve = Curve::new_gd();
                    curve.add_point(Vector2::ZERO);
                    curve.add_point(Vector2::new(1.0, 0.5));
                    curve.set_point_right_tangent(0, 0.5);
                    curve.set_point_left_tangent(1, 0.5);
                    curve
                }),
            )
            .with_loops(2, LoopMode::Restart),
        );
        assert!(matches!(carried.custom_step(4.0, false), TweenStepResult::Stepped(AdvanceTimeResult::Playing)));
        assert_eq!(carried.get_loops_finished(), 1);
        match carried.custom_step(0.0, false) {
            TweenStepResult::Stepped(AdvanceTimeResult::Completed { unscaled_excess_time }) => {
                near(unscaled_excess_time, 1.5)
            }
            _ => panic!("the next update must complete the second leg and preserve carried excess"),
        }
        n.free();
        drop(any);
        drop(t);
        assert!(weak.upgrade().is_none());
        Box::pin(async {})
    }

    pub(super) fn test_characters_range_failures(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "AB", false);
        let mut curve = Curve::new_gd();
        curve.set_min_value(-1.0);
        curve.add_point(Vector2::new(0.0, -0.5));
        curve.add_point(Vector2::new(1.0, -0.5));
        let invalid = manual(n.do_offset_chars(0, 1, Vector2::ONE, 1.0, EaseKind::GodotCurve(curve)));
        assert!(matches!(invalid.custom_step(0.25, false), TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
        v2(SpireRichTextLabel::get_char_offset(n.clone(), 0), Vector2::ZERO);
        let valid = manual(n.do_offset_chars(0, 1, Vector2::ONE, 1.0, EaseKind::Basic(Ease::Linear)));
        n.free();
        assert!(matches!(valid.custom_step(0.25, false), TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
        Box::pin(async {})
    }

    pub(super) fn test_text_presets_rust_api(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "[bob]A[/bob]", true);
        let plain = label(&mut self.base_mut(), "A", false);
        let mut offset = SpireTextEffect::new_gd();
        offset.bind_mut().set_kind(TextEffectKind::Offset);
        offset.bind_mut().set_from(Vector2::ZERO.to_variant());
        offset.bind_mut().set_from(Vector2::new(12.0, 0.0).to_variant());
        offset.bind_mut().set_to(Vector2::new(12.0, 0.0).to_variant());
        let mut preset = SpireRichTextEffectPreset::new_gd();
        preset.bind_mut().set_bbcode("bob".into());
        let mut entries = Array::<Option<Gd<SpireTextEffect>>>::new();
        entries.push(Some(&offset));
        preset.bind_mut().set_effects(entries);
        {
            let mut label = n.clone();
            let mut state = label.bind_mut();
            state.effect_presets.push(None::<&Gd<SpireRichTextEffectPreset>>);
            state.effect_presets.push(Some(&preset));
            assert!(state.effect_presets.at(0).is_none());
            assert!(state.effect_presets.at(1).is_some());
        }
        let animator = n.clone();
        let baseline = plain.clone();
        Box::pin(async move {
            for _ in 0..4 {
                next_frame().await;
            }
            assert_eq!(n.get_parsed_text(), GString::from("A"));
            assert_eq!(n.get_effects().len(), 1);
            assert_eq!(n.get_effects().at(0).to::<Gd<Resource>>().get_class(), GString::from("SpireRichTextEffect"));
            let before = SpireRichTextLabel::sample_for_test(animator.clone(), 0).unwrap().0;
            let origin = SpireRichTextLabel::sample_for_test(baseline.clone(), 0)
                .unwrap()
                .0
                .origin;
            v2(before.origin - origin, Vector2::new(12.0, 0.0));
            let mut tween = manual(animator.do_offset_char(0, Vector2::new(3.0, 0.0), 1.0));
            tween.force_complete();
            for _ in 0..4 {
                next_frame().await;
            }
            let after = SpireRichTextLabel::sample_for_test(animator.clone(), 0).unwrap().0;
            v2(after.origin - origin, Vector2::new(15.0, 0.0));
            v2(SpireRichTextLabel::get_char_offset(animator.clone(), 0), Vector2::new(3.0, 0.0));
            n.free();
            plain.free();
        })
    }

    pub(super) fn test_characters_translation(&mut self) -> PinnedTestTask {
        let mut server = TranslationServer::singleton();
        let locale = server.get_locale();
        server.set_locale("en");
        let mut translation = Translation::new_gd();
        translation.set_locale("es");
        translation.add_message("SPIRE_CHAR_TRANSLATION", "[color=red]猫AB[/color]");
        server.add_translation(&translation);
        let n = label(&mut self.base_mut(), "SPIRE_CHAR_TRANSLATION", true);
        let a = n.clone();
        server.set_locale("es");
        Box::pin(async move {
            for _ in 0..4 {
                next_frame().await;
            }
            let count = SpireRichTextLabel::get_character_count(a.clone());
            let source = n.get_text();
            let parsed = n.get_parsed_text();
            let sample = SpireRichTextLabel::sample_for_test(a.clone(), 0);
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
        let a = n.clone();
        let angle = std::f32::consts::TAU * 2.0 + 0.8;
        for mode in [RotateMode::Fast, RotateMode::FastBeyond360] {
            SpireRichTextLabel::reset(a.clone());
            let t = manual(a.do_rotate_char(0, Vector3::new(0.0, 0.0, angle), 1.0, mode));
            step(&t, 0.5);
            near(
                SpireRichTextLabel::get_char_rotation(a.clone(), 0).z as f64,
                if mode == RotateMode::Fast { 0.4 } else { angle as f64 / 2.0 },
            );
        }
        let from = Vector3::new(0.2, 0.3, 0.4);
        let by = Vector3::new(0.3, -0.4, 0.0);
        for mode in [RotateMode::WorldAxisAdd, RotateMode::LocalAxisAdd] {
            SpireRichTextLabel::set_char_rotation(a.clone(), 0, from);
            let mut t = manual(a.do_rotate_char(0, by, 1.0, mode));
            t.force_complete();
            let expected = if mode == RotateMode::WorldAxisAdd {
                Quaternion::from_euler(by) * Quaternion::from_euler(from)
            } else {
                Quaternion::from_euler(from) * Quaternion::from_euler(by)
            };
            assert!(
                Quaternion::from_euler(SpireRichTextLabel::get_char_rotation(a.clone(), 0))
                    .dot(expected)
                    .abs()
                    > 0.99999
            );
        }
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_effect_refresh(&mut self) -> PinnedTestTask {
        let mut n = label(&mut self.base_mut(), "A", false);
        let a = n.clone();
        for blendable in [true, false] {
            SpireRichTextLabel::reset(a.clone());
            let mut t = manual(a.do_punch_char_offset(0, Vector2::new(4.0, 0.0), 1.0, 4, 1.0, blendable));
            step(&t, 0.1);
            v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::new(4.0, 0.0));
            n.set_text(if blendable { "B" } else { "C" });
            SpireRichTextLabel::refresh(n.clone());
            v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::ZERO);
            step(&t, 0.2);
            v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::new(-3.0, 0.0));
            SpireRichTextLabel::reset(a.clone());
            t.force_complete();
            v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::ZERO);
        }
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_automatic(&mut self) -> PinnedTestTask {
        let mut n = label(&mut self.base_mut(), "A", false);
        let a = n.clone();
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
            v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::new(8.0, 0.0));
            n.set_text("B");
            for _ in 0..4 {
                next_frame().await;
            }
            // The sample must already exist: a getter must not be the action
            // that discovers the edit, resets state and installs the new scope.
            let sample = SpireRichTextLabel::sample_for_test(a.clone(), 0)
                .expect("label must maintain the effect without live tweens");
            v2(sample.0.a, Vector2::RIGHT);
            v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::ZERO);
            n.free();
        })
    }

    pub(super) fn test_characters_label(&mut self) -> PinnedTestTask {
        let mut n = label(&mut self.base_mut(), "[b]AB[/b]", false);
        let a = n.clone();
        let children = n.get_child_count();
        assert_eq!(children, 0, "character animation must not add child nodes");
        let plain = RichTextLabel::new_alloc();
        assert_eq!(
            n.get_child_count_ex().include_internal(true).done(),
            plain.get_child_count_ex().include_internal(true).done()
        );
        plain.free();
        assert_eq!(n.get_process_mode(), godot::classes::node::ProcessMode::INHERIT);
        let b = n.clone();
        assert_eq!(n.get_child_count(), children);
        assert_eq!(n.get_text(), GString::from("[b]AB[/b]"));
        assert!(!n.is_using_bbcode());
        assert_eq!(SpireRichTextLabel::get_character_count(a.clone()), 9);
        n.set_use_bbcode(true);
        SpireRichTextLabel::refresh(n.clone());
        assert_eq!(SpireRichTextLabel::get_character_count(a.clone()), 2);
        assert_eq!(n.get_parsed_text(), GString::from("AB"));
        SpireRichTextLabel::set_char_offset(a.clone(), 1, Vector2::ONE);
        v2(SpireRichTextLabel::get_char_offset(b.clone(), 1), Vector2::ONE);
        let copy = n
            .duplicate()
            .expect("character state must not block Node::duplicate")
            .cast::<SpireRichTextLabel>();
        self.base_mut().add_child(&copy);
        let other = copy.clone();
        SpireRichTextLabel::set_char_offset(other.clone(), 1, Vector2::new(4.0, 0.0));
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 1), Vector2::ONE);
        v2(SpireRichTextLabel::get_char_offset(other.clone(), 1), Vector2::new(4.0, 0.0));
        copy.free();
        assert!(!SpireRichTextLabel::is_valid_char(a.clone(), -1));
        assert!(!SpireRichTextLabel::is_valid_char(a.clone(), 2));
        // Both trait receivers build label-linked callables without reborrowing userdata.
        SpireRichTextLabel::set_char_offset(n.clone(), 1, Vector2::ZERO);
        let mut from_ref = manual(n.bind().do_offset_char(1, Vector2::new(6.0, 0.0), 1.0));
        assert_eq!(from_ref.get_owner().instance_id(), n.instance_id());
        step(&from_ref, 0.5);
        v2(SpireRichTextLabel::get_char_offset(n.clone(), 1), Vector2::new(3.0, 0.0));
        from_ref.force_complete();
        let builders = {
            let state = n.bind_mut();
            [
                state.do_offset_char(99, Vector2::ONE, 1.0),
                state.do_scale_char(99, Vector2::ONE, 1.0),
                state.do_color_char(99, Color::RED, 1.0),
                state.do_fade_char(99, 0.0, 1.0),
                state.do_rotate_char(99, Vector3::ONE, 1.0, RotateMode::Fast),
                state.do_punch_char_offset(99, Vector2::ONE, 1.0, 4, 1.0, true),
                state.do_punch_char_rotation(99, Vector3::ONE, 1.0, 4, 1.0, true),
                state.do_punch_char_scale(99, Vector2::ONE, 1.0, 4, 1.0, true),
                state.do_shake_char_offset(99, 1.0, Vector2::ONE, 4, 90.0, true, ShakeRandomnessMode::Full, true),
                state.do_shake_char_rotation(99, 1.0, Vector3::ONE, 4, 90.0, true, ShakeRandomnessMode::Full, true),
                state.do_shake_char_scale(99, 1.0, Vector2::ONE, 4, 90.0, true, ShakeRandomnessMode::Full, true),
            ]
        };
        for builder in builders {
            assert_eq!(builder.get_owner().instance_id(), n.instance_id());
            let mut tween = manual(builder);
            step(&tween, 0.25);
            assert_eq!(tween.get_state(), TweenState::Playing);
            tween.force_complete();
        }
        n.free();
        assert!(!a.is_instance_valid());
        assert!(!b.is_instance_valid());
        Box::pin(async {})
    }

    pub(super) fn test_characters_channels(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "ABC", false);
        let a = n.clone();
        SpireRichTextLabel::set_char_offset(a.clone(), 0, Vector2::new(2.0, 4.0));
        let mut offset = manual(a.do_offset_char(0, Vector2::new(10.0, 12.0), 1.0));
        let mut scale = manual(a.do_scale_char(1, Vector2::new(3.0, 5.0), 1.0));
        SpireRichTextLabel::set_char_color(a.clone(), 2, Color::from_rgba(1.0, 0.0, 0.0, 0.8));
        let mut color = manual(a.do_color_char(2, Color::from_rgba(0.0, 0.0, 1.0, 0.4), 1.0));
        for t in [&offset, &scale, &color] {
            step(t, 0.5);
        }
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::new(6.0, 8.0));
        v2(SpireRichTextLabel::get_char_scale(a.clone(), 1), Vector2::new(2.0, 3.0));
        near(SpireRichTextLabel::get_char_color(a.clone(), 2).r as f64, 0.5);
        near(SpireRichTextLabel::get_char_alpha(a.clone(), 2), 0.6);
        let mut fade = manual(a.do_fade_char(2, 0.0, 1.0));
        step(&fade, 0.5);
        near(SpireRichTextLabel::get_char_alpha(a.clone(), 2), 0.3);
        near(SpireRichTextLabel::get_char_color(a.clone(), 2).r as f64, 0.5);
        for t in [&mut offset, &mut scale, &mut color, &mut fade] {
            t.force_complete();
        }
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::new(10.0, 12.0));
        v2(SpireRichTextLabel::get_char_scale(a.clone(), 1), Vector2::new(3.0, 5.0));
        near(SpireRichTextLabel::get_char_alpha(a.clone(), 2), 0.0);
        SpireRichTextLabel::set_char_alpha(a.clone(), 2, 1.0);
        let mut overshoot = manual(a.do_fade_char(2, 0.0, 1.0));
        overshoot.set_ease(overshoot_ease());
        step(&overshoot, 0.5);
        assert!(
            SpireRichTextLabel::get_char_alpha(a.clone(), 2) < 0.0,
            "valid easing overshoot must not be treated as invalid input"
        );
        overshoot.force_complete();
        near(SpireRichTextLabel::get_char_alpha(a.clone(), 2), 0.0);
        SpireRichTextLabel::set_char_color(a.clone(), 2, Color::WHITE);
        let mut overshoot = manual(a.do_color_char(2, Color::from_rgba(0.0, 0.0, 1.0, 0.0), 1.0));
        overshoot.set_ease(overshoot_ease());
        step(&overshoot, 0.5);
        assert!(SpireRichTextLabel::get_char_alpha(a.clone(), 2) < 0.0);
        overshoot.force_complete();
        near(SpireRichTextLabel::get_char_alpha(a.clone(), 2), 0.0);
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_refresh(&mut self) -> PinnedTestTask {
        let mut n = label(&mut self.base_mut(), "ABC", false);
        let a = n.clone();
        let mut t = manual(a.do_offset_char(2, Vector2::new(8.0, 0.0), 1.0));
        step(&t, 0.25);
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 2), Vector2::new(2.0, 0.0));
        n.set_text("12345");
        SpireRichTextLabel::refresh(n.clone());
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 2), Vector2::ZERO);
        step(&t, 0.25);
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 2), Vector2::new(4.0, 0.0));
        near(t.get_total_elapsed_time(), 0.5);
        n.set_size(Vector2::new(100.0, 100.0));
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 2), Vector2::new(4.0, 0.0));
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
        SpireRichTextLabel::refresh(n.clone());
        assert!(matches!(t.custom_step(0.25, false), TweenStepResult::Stepped(AdvanceTimeResult::Playing)));
        assert_eq!(*events, [0, 0, 0]);
        near(t.get_total_elapsed_time(), 0.75);
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::ZERO);
        n.set_text("ABC");
        SpireRichTextLabel::refresh(n.clone());
        step(&t, 0.25);
        assert_eq!(*events, [1, 1, 1]);
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 2), Vector2::new(8.0, 0.0));
        assert_eq!(t.get_state(), TweenState::Stopped);
        let mut invalid = manual(a.do_offset_char(-1, Vector2::ONE, 1.0));
        assert_eq!(invalid.get_state(), TweenState::Playing);
        invalid.force_complete();
        assert_eq!(invalid.get_state(), TweenState::Stopped);
        assert_eq!(n.get_text(), GString::from("ABC"));
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_explicit_refresh(&mut self) -> PinnedTestTask {
        let mut n = SpireRichTextLabel::new_alloc();
        n.set_text("A");
        assert_eq!(SpireRichTextLabel::get_character_count(n.clone()), 0);
        assert!(!SpireRichTextLabel::is_valid_char(n.clone(), 0));
        SpireRichTextLabel::refresh(n.clone());
        assert_eq!(SpireRichTextLabel::get_character_count(n.clone()), 1);
        SpireRichTextLabel::set_char_offset(n.clone(), 0, Vector2::new(3.0, 4.0));
        n.set_text("BC");
        n.append_text("SENTINEL");
        let epoch = n.bind().char_renderer.bind().epoch;
        assert_eq!(SpireRichTextLabel::get_character_count(n.clone()), 1);
        v2(SpireRichTextLabel::get_char_offset(n.clone(), 0), Vector2::new(3.0, 4.0));
        let mut tween = manual(n.do_offset_char(0, Vector2::new(8.0, 9.0), 1.0));
        tween.force_complete();
        assert_eq!(n.bind().char_renderer.bind().epoch, epoch);
        assert_eq!(n.get_parsed_text(), GString::from("BCSENTINEL"));
        v2(SpireRichTextLabel::get_char_offset(n.clone(), 0), Vector2::new(8.0, 9.0));
        SpireRichTextLabel::reset(n.clone());
        assert_eq!(SpireRichTextLabel::get_character_count(n.clone()), 1);
        assert_eq!(n.get_parsed_text(), GString::from("BCSENTINEL"));
        SpireRichTextLabel::refresh(n.clone());
        assert_eq!(SpireRichTextLabel::get_character_count(n.clone()), 2);
        assert_eq!(n.get_parsed_text(), GString::from("BC"));
        v2(SpireRichTextLabel::get_char_offset(n.clone(), 0), Vector2::ZERO);
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_lifecycle(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "AB", false);
        let a = n.clone();
        let mut t = manual(a.do_offset_char(0, Vector2::new(8.0, 0.0), 1.0));
        step(&t, 0.25);
        t.pause();
        step(&t, 0.5);
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::new(2.0, 0.0));
        t.play();
        step(&t, 0.25);
        t.stop();
        t.force_complete();
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::new(4.0, 0.0));
        t.play();
        step(&t, 0.25);
        SpireRichTextLabel::reset(a.clone());
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::ZERO);
        step(&t, 0.25);
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::new(4.0, 0.0));
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
        let a = n.clone();
        let t = manual(a.do_fade_char(0, 0.0, 1.0));
        n.free();
        step(&t, 1.0);
        assert_eq!(t.get_state(), TweenState::Stopped);
        Box::pin(async {})
    }

    pub(super) fn test_characters_loops_sequence(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "A", false);
        let a = n.clone();
        for mode in [LoopMode::Restart, LoopMode::Yoyo, LoopMode::Incremental] {
            SpireRichTextLabel::reset(a.clone());
            let mut t = manual(a.do_offset_char(0, Vector2::new(8.0, 0.0), 1.0).with_loops(2, mode));
            step(&t, 1.0);
            step(&t, 0.25);
            let x = match mode {
                LoopMode::Restart => 2.0,
                LoopMode::Yoyo => 6.0,
                LoopMode::Incremental => 10.0,
            };
            near(SpireRichTextLabel::get_char_offset(a.clone(), 0).x as f64, x);
            t.stop();
        }
        SpireRichTextLabel::reset(a.clone());
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
        near(SpireRichTextLabel::get_char_offset(a.clone(), 0).x as f64, 10.0);
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_effects(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "A", false);
        let a = n.clone();
        let mut effects = [
            manual(a.do_punch_char_offset(0, Vector2::new(4.0, 0.0), 1.0, 4, 1.0, true)),
            manual(a.do_punch_char_scale(0, Vector2::new(0.0, 2.0), 1.0, 4, 1.0, true)),
            manual(a.do_punch_char_rotation(0, Vector3::new(0.3, 0.0, 0.0), 1.0, 4, 1.0, true)),
        ];
        for t in &effects {
            step(t, 0.1);
        }
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::new(4.0, 0.0));
        v2(SpireRichTextLabel::get_char_scale(a.clone(), 0), Vector2::new(1.0, 3.0));
        v3(SpireRichTextLabel::get_char_rotation(a.clone(), 0), Vector3::new(0.3, 0.0, 0.0));
        for t in &mut effects {
            t.force_complete();
        }
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::ZERO);
        v2(SpireRichTextLabel::get_char_scale(a.clone(), 0), Vector2::ONE);
        v3(SpireRichTextLabel::get_char_rotation(a.clone(), 0), Vector3::ZERO);
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
        assert!(SpireRichTextLabel::get_char_offset(a.clone(), 0).length() > 0.00001);
        assert!((SpireRichTextLabel::get_char_scale(a.clone(), 0) - Vector2::ONE).length() > 0.00001);
        assert!(SpireRichTextLabel::get_char_rotation(a.clone(), 0).length() > 0.00001);
        for t in &mut shakes {
            t.force_complete();
        }
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::ZERO);
        v2(SpireRichTextLabel::get_char_scale(a.clone(), 0), Vector2::ONE);
        v3(SpireRichTextLabel::get_char_rotation(a.clone(), 0), Vector3::ZERO);
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_invalid(&mut self) -> PinnedTestTask {
        let mut n = label(&mut self.base_mut(), "A", false);
        let a = n.clone();
        let mut t = manual(a.do_offset_char(0, Vector2::new(real::NAN, 4.0), f64::NAN));
        near(t.get_duration(), 1.0);
        t.force_complete();
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::new(0.0, 4.0));
        SpireRichTextLabel::set_char_alpha(a.clone(), 0, 3.0);
        near(SpireRichTextLabel::get_char_alpha(a.clone(), 0), 1.0);
        SpireRichTextLabel::set_char_offset(a.clone(), -1, Vector2::ONE);
        v2(SpireRichTextLabel::get_char_offset(a.clone(), -1), Vector2::ZERO);
        v2(SpireRichTextLabel::get_char_scale(a.clone(), 2), Vector2::ONE);
        v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::new(0.0, 4.0));
        SpireRichTextLabel::set_char_rotation(a.clone(), 0, Vector3::new(real::NAN, 0.0, 0.0));
        v3(SpireRichTextLabel::get_char_rotation(a.clone(), 0), Vector3::ZERO);
        let mut t =
            manual(a.do_shake_char_offset(0, 1.0, Vector2::ONE, 0, real::NAN, false, ShakeRandomnessMode::Full, true));
        n.set_text("");
        SpireRichTextLabel::refresh(n.clone());
        t.force_complete();
        assert_eq!(t.get_state(), TweenState::Stopped);
        let mut t = manual(a.do_fade_char(-1, 0.0, 1.0));
        assert_eq!(t.get_state(), TweenState::Playing);
        t.force_complete();
        n.free();
        Box::pin(async {})
    }

    pub(super) fn test_characters_render(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "[color=red]A[/color]B", true);
        let a = n.clone();
        Box::pin(async move {
            for _ in 0..4 {
                next_frame().await;
            }
            let original =
                SpireRichTextLabel::sample_for_test(a.clone(), 0).expect("engine must invoke the real RichTextEffect");
            near(original.1.r as f64, 1.0);
            near(original.1.g as f64, 0.0);
            near(SpireRichTextLabel::get_char_color(a.clone(), 0).r as f64, 1.0);
            SpireRichTextLabel::set_char_offset(a.clone(), 0, Vector2::new(10.0, 20.0));
            SpireRichTextLabel::set_char_scale(a.clone(), 0, Vector2::new(2.0, 3.0));
            SpireRichTextLabel::set_char_rotation(a.clone(), 0, Vector3::new(0.0, std::f32::consts::FRAC_PI_3, 0.0));
            SpireRichTextLabel::set_char_color(a.clone(), 0, Color::BLUE);
            SpireRichTextLabel::set_char_alpha(a.clone(), 0, 0.25);
            for _ in 0..3 {
                next_frame().await;
            }
            let sample = SpireRichTextLabel::sample_for_test(a.clone(), 0).unwrap();
            v2(sample.0.a, Vector2::RIGHT);
            v2(sample.0.b, Vector2::DOWN * 3.0);
            v2(sample.0.origin - original.0.origin, Vector2::new(10.0, 20.0));
            near(sample.1.b as f64, 1.0);
            near(sample.1.a as f64, 0.25);
            let other = SpireRichTextLabel::sample_for_test(a.clone(), 1).unwrap();
            v2(other.0.a, Vector2::RIGHT);
            v2(other.0.b, Vector2::DOWN);
            SpireRichTextLabel::reset(a.clone());
            for _ in 0..3 {
                next_frame().await;
            }
            let reset = SpireRichTextLabel::sample_for_test(a.clone(), 0).unwrap();
            v2(reset.0.a, Vector2::RIGHT);
            v2(reset.0.origin, original.0.origin);
            near(reset.1.r as f64, 1.0);
            SpireRichTextLabel::set_char_pivot(a.clone(), 0, Vector2::new(10.0, -6.0));
            SpireRichTextLabel::set_char_rotation(a.clone(), 0, Vector3::new(std::f32::consts::FRAC_PI_3, 0.0, 0.0));
            for _ in 0..3 {
                next_frame().await;
            }
            let tilted = SpireRichTextLabel::sample_for_test(a.clone(), 0).unwrap().0;
            v2(tilted.a, Vector2::RIGHT);
            v2(tilted.b, Vector2::DOWN * 0.5);
            v2(tilted.origin - original.0.origin, Vector2::new(0.0, -3.0));
            SpireRichTextLabel::set_char_pivot(a.clone(), 0, Vector2::ZERO);
            SpireRichTextLabel::set_char_rotation(
                a.clone(),
                0,
                Vector3::new(std::f32::consts::FRAC_PI_3, std::f32::consts::FRAC_PI_3, 0.0),
            );
            for _ in 0..3 {
                next_frame().await;
            }
            let combined = SpireRichTextLabel::sample_for_test(a.clone(), 0).unwrap().0;
            v2(combined.a, Vector2::new(0.5, 0.0));
            v2(combined.b, Vector2::new(0.75, 0.5));
            SpireRichTextLabel::set_char_rotation(a.clone(), 0, Vector3::new(0.0, 0.0, std::f32::consts::FRAC_PI_2));
            for _ in 0..3 {
                next_frame().await;
            }
            let rotated = SpireRichTextLabel::sample_for_test(a.clone(), 0).unwrap().0;
            v2(rotated.a, Vector2::DOWN);
            v2(rotated.b, Vector2::LEFT);
            n.free();
        })
    }

    pub(super) fn test_characters_unicode_render(&mut self) -> PinnedTestTask {
        let n = label(&mut self.base_mut(), "e\u{301}אב", false);
        let a = n.clone();
        assert_eq!(SpireRichTextLabel::get_character_count(a.clone()), 4);
        SpireRichTextLabel::set_char_offset(a.clone(), 1, Vector2::new(12.0, 3.0));
        Box::pin(async move {
            for _ in 0..4 {
                next_frame().await;
            }
            assert!(SpireRichTextLabel::sample_for_test(a.clone(), 0).is_some());
            v2(SpireRichTextLabel::get_char_offset(a.clone(), 0), Vector2::new(12.0, 3.0));
            v2(SpireRichTextLabel::get_char_offset(a.clone(), 1), Vector2::new(12.0, 3.0));
            SpireRichTextLabel::set_char_scale(a.clone(), 0, Vector2::splat(2.0));
            v2(SpireRichTextLabel::get_char_scale(a.clone(), 1), Vector2::splat(2.0));
            assert!(SpireRichTextLabel::is_valid_char(a.clone(), 2));
            assert!(SpireRichTextLabel::sample_for_test(a.clone(), 2).is_some());
            n.free();
        })
    }
}
