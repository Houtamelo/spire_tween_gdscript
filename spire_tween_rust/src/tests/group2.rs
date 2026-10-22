//! Real-engine coverage for the simple adapter/template API.
use godot::{builtin::Side, classes::*, prelude::*};
use spire_tween::prelude::*;

use super::{misc::MiscTests, util::PinnedTestTask};

fn manual<T: ITweenable>(tween: SpireTween<T>) -> TweenPtr<T> {
    TweenPtr::new(tween.with_process_mode(ProcessMode::Manual))
}
fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 0.0001, "{a} != {b}");
}
fn vec2(a: Vector2, b: Vector2) { near(a.distance_to(b) as f64, 0.0); }
fn vec3(a: Vector3, b: Vector3) { near(a.distance_to(b) as f64, 0.0); }
fn ease() -> EaseKind { EaseKind::Basic(Ease::Linear) }

impl MiscTests {
    pub(super) fn test_group2_typed_accessors(&mut self) -> PinnedTestTask {
        let mut node = Node2D::new_alloc();
        self.base_mut().add_child(&node);
        let read = node.clone();
        let mut write = node.clone();
        let mut t = manual(
            do_get_set(move || read.get_rotation() as f64, move |v: f64| write.set_rotation(v as f32), 4.0, 1.0)
                .with_ease(ease()),
        );
        node.set_rotation(2.0);
        let _ = t.custom_step(0.5, false);
        near(node.get_rotation() as f64, 3.0);
        t.force_complete();
        near(node.get_rotation() as f64, 4.0);
        let read = node.clone();
        let mut write = node.clone();
        let mut axis = manual(
            do_get_set_vec2_axis(
                move || read.get_position(),
                move |v| write.set_position(v),
                godot::builtin::Vector2Axis::X,
                10.0,
                1.0,
            )
            .with_ease(ease()),
        );
        node.set_position(Vector2::new(2.0, 7.0));
        let _ = axis.custom_step(0.5, false);
        vec2(node.get_position(), Vector2::new(6.0, 7.0));
        node.set_position(Vector2::new(6.0, 19.0));
        axis.force_complete();
        vec2(node.get_position(), Vector2::new(10.0, 19.0));
        let read = node.clone();
        let mut write = node.clone();
        let mut alpha = manual(
            do_get_set_alpha(move || read.get_modulate(), move |v| write.set_modulate(v), 0.0, 1.0).with_ease(ease()),
        );
        node.set_modulate(Color::from_rgba(0.2, 0.3, 0.4, 1.0));
        let _ = alpha.custom_step(0.5, false);
        near(node.get_modulate().a as f64, 0.5);
        node.set_modulate(Color::from_rgba(0.6, 0.7, 0.8, 0.5));
        alpha.force_complete();
        assert_eq!(node.get_modulate(), Color::from_rgba(0.6, 0.7, 0.8, 0.0));
        let mut node3 = Node3D::new_alloc();
        self.base_mut().add_child(&node3);
        let read = node3.clone();
        let mut write = node3.clone();
        let axis3 = manual(
            do_get_set_vec3_axis(
                move || read.get_position(),
                move |v| write.set_position(v),
                godot::builtin::Vector3Axis::Z,
                8.0,
                1.0,
            )
            .with_ease(ease()),
        );
        node3.set_position(Vector3::new(2.0, 3.0, 0.0));
        let _ = axis3.custom_step(0.5, false);
        vec3(node3.get_position(), Vector3::new(2.0, 3.0, 4.0));
        drop((t, axis, alpha, axis3));
        macro_rules! typed_value {
            ($start:expr, $end:expr, $ty:ty) => {{
                node.set_meta("spire_typed_value", &($start).to_variant());
                let read = node.clone();
                let mut write = node.clone();
                let mut tween = manual(do_get_set(
                    move || read.get_meta("spire_typed_value").to::<$ty>(),
                    move |value: $ty| write.set_meta("spire_typed_value", &value.to_variant()),
                    $end,
                    1.0,
                ));
                tween.force_complete();
                assert_eq!(node.get_meta("spire_typed_value").to::<$ty>(), $end);
            }};
        }
        typed_value!(1_i64, 4_i64, i64);
        typed_value!(Vector2::ZERO, Vector2::ONE, Vector2);
        typed_value!(Vector3::ZERO, Vector3::ONE, Vector3);
        typed_value!(Vector2i::ZERO, Vector2i::ONE, Vector2i);
        typed_value!(Vector3i::ZERO, Vector3i::ONE, Vector3i);
        typed_value!(Color::from_rgba(0.0, 0.0, 0.0, 1.0), Color::from_rgba(1.0, 0.0, 0.0, 1.0), Color);
        typed_value!(GString::from("start"), GString::from("end"), GString);
        node.free();
        node3.free();
        Box::pin(async {})
    }

    pub(super) fn test_group2_callable_validity(&mut self) -> PinnedTestTask {
        for free_getter in [false, true] {
            for forced in [false, true] {
                let getter = Node2D::new_alloc();
                let setter = Node2D::new_alloc();
                self.base_mut().add_child(&getter);
                self.base_mut().add_child(&setter);
                let data: PropertyDataFloat =
                    PropertyDataViaCallable::new(getter.callable("get_rotation"), setter.callable("set_rotation"))
                        .into();
                let mut tween = manual(SpireTween::<LerpPropertyData<f64>>::new(data, Evaluator::Static(4.0), 1.0));
                let events = RcPtr::new([0, 0]);
                let stopped = events.clone();
                tween.stopped_connect(move || stopped.to_mut()[0] += 1, SpireFlags::empty());
                let finished = events.clone();
                tween.finished_connect(move || finished.to_mut()[1] += 1, SpireFlags::empty());
                if free_getter {
                    getter.clone().free();
                } else {
                    setter.clone().free();
                }
                if forced {
                    tween.force_complete();
                } else {
                    assert!(matches!(
                        tween.custom_step(0.5, false),
                        TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)
                    ));
                }
                assert_eq!(*events, [1, 0]);
                if free_getter {
                    setter.free();
                } else {
                    getter.free();
                }
            }
        }
        Box::pin(async {})
    }

    pub(super) fn test_group2_motion(&mut self) -> PinnedTestTask {
        let mut node = Node2D::new_alloc();
        self.base_mut().add_child(&node);
        let mut jump = manual(node.do_jump(Vector2::new(10.0, 10.0), 4.0, 1, 1.0).with_ease(ease()));
        node.set_position(Vector2::new(2.0, 2.0));
        let _ = jump.custom_step(0.25, false);
        vec2(node.get_position(), Vector2::new(4.0, 2.5));
        let _ = jump.custom_step(0.25, false);
        vec2(node.get_position(), Vector2::new(6.0, 4.0));
        jump.force_complete();
        vec2(node.get_position(), Vector2::new(10.0, 10.0));
        let node3 = Node3D::new_alloc();
        self.base_mut().add_child(&node3);
        let mut t = manual(
            node3
                .do_jump(Vector3::new(8.0, 8.0, 4.0), 4.0, 2, 1.0)
                .with_ease(ease()),
        );
        let _ = t.custom_step(0.25, false);
        vec3(node3.get_position(), Vector3::new(2.0, 7.5, 1.0));
        t.force_complete();
        vec3(node3.get_position(), Vector3::new(8.0, 8.0, 4.0));
        let scale = manual(node3.do_uniform_scale(3.0, 1.0).with_ease(ease()));
        let _ = scale.custom_step(0.5, false);
        vec3(node3.get_scale(), Vector3::splat(2.0));
        let ui = Control::new_alloc();
        self.base_mut().add_child(&ui);
        let mut circle = manual(
            ui.do_circle(Vector2::new(5.0, 6.0), 0.0, std::f32::consts::PI, 4.0, 1.0)
                .with_ease(ease()),
        );
        let _ = circle.custom_step(0.5, false);
        vec2(ui.get_position(), Vector2::new(5.0, 10.0));
        circle.force_complete();
        vec2(ui.get_position(), Vector2::new(1.0, 6.0));
        let mut anchors = manual(ui.do_anchor_min(Vector2::new(0.2, 0.3), 1.0));
        anchors.force_complete();
        near(ui.get_anchor(Side::LEFT) as f64, 0.2);
        near(ui.get_anchor(Side::TOP) as f64, 0.3);
        drop((jump, t, scale, circle, anchors));
        node.free();
        node3.free();
        ui.free();
        Box::pin(async {})
    }

    pub(super) fn test_group2_blendables(&mut self) -> PinnedTestTask {
        let mut node = Node2D::new_alloc();
        self.base_mut().add_child(&node);
        let mut a = manual(
            node.do_blendable_position(Vector2::new(4.0, 6.0), 1.0)
                .with_ease(ease()),
        );
        let mut b = manual(
            node.do_blendable_position(Vector2::new(2.0, 4.0), 1.0)
                .with_ease(ease()),
        );
        let _ = a.custom_step(0.5, false);
        let _ = b.custom_step(0.5, false);
        vec2(node.get_position(), Vector2::new(3.0, 5.0));
        node.set_position(Vector2::new(20.0, 30.0));
        a.force_complete();
        b.force_complete();
        vec2(node.get_position(), Vector2::new(23.0, 35.0));
        node.set_modulate(Color::from_rgba(0.2, 0.2, 0.2, 1.0));
        let mut c = manual(
            node.do_blendable_modulate(Color::from_rgba(0.4, 0.2, 0.2, 1.0), 1.0)
                .with_ease(ease()),
        );
        let mut d = manual(
            node.do_blendable_modulate(Color::from_rgba(0.2, 0.6, 0.2, 1.0), 1.0)
                .with_ease(ease()),
        );
        node.set_modulate(Color::from_rgba(0.3, 0.3, 0.3, 0.5));
        c.force_complete();
        d.force_complete();
        let color = node.get_modulate();
        near(color.r as f64, 0.5);
        near(color.g as f64, 0.7);
        near(color.a as f64, 0.5);
        drop((a, b, c, d));
        node.free();
        Box::pin(async {})
    }

    pub(super) fn test_group2_resources(&mut self) -> PinnedTestTask {
        let mut env = Environment::new_gd();
        let mut t = manual(
            env.do_background_color(Color::from_rgba(1.0, 0.0, 0.0, 0.5), 1.0)
                .with_ease(ease()),
        );
        env.set_bg_color(Color::from_rgba(0.0, 0.0, 0.0, 1.0));
        let _ = t.custom_step(0.5, false);
        near(env.get_bg_color().r as f64, 0.5);
        let mut material = StandardMaterial3D::new_gd();
        let mut fade = manual(material.do_fade(0.0, 1.0).with_ease(ease()));
        material.set_albedo(Color::from_rgba(0.2, 0.3, 0.4, 1.0));
        fade.force_complete();
        assert_eq!(material.get_albedo(), Color::from_rgba(0.2, 0.3, 0.4, 0.0));
        let uv = manual(material.do_uv1_scale(Vector3::splat(3.0), 1.0).with_ease(ease()));
        let _ = uv.custom_step(0.5, false);
        vec3(material.get_uv1_scale(), Vector3::splat(2.0));
        drop(material);
        t.force_complete(); // Resource handles are retained by their adapters.
        Box::pin(async {})
    }

    pub(super) fn test_group2_styles_counter(&mut self) -> PinnedTestTask {
        let mut label = Label::new_alloc();
        self.base_mut().add_child(&label);
        let mut settings = LabelSettings::new_gd();
        settings.set_font_size(20);
        label.set_label_settings(&settings);
        let mut font = manual(label.do_font_size(40, 1.0).with_ease(ease()));
        let _ = font.custom_step(0.5, false);
        assert_eq!(settings.get_font_size(), 30);
        font.stop();
        let mut generic_font = manual(
            label
                .do_property("font_size", Evaluator::Static(50_i64), 1.0)
                .with_ease(ease()),
        );
        let _ = generic_font.custom_step(0.5, false);
        assert_eq!(settings.get_font_size(), 40);
        generic_font.force_complete();
        assert_eq!(settings.get_font_size(), 50);
        let mut counter = manual(label.do_counter(-2000, 2000, 1.0, GString::from("·")).with_ease(ease()));
        let _ = counter.custom_step(0.5, false);
        assert_eq!(label.get_text(), GString::from("0"));
        counter.force_complete();
        assert_eq!(label.get_text(), GString::from("2·000"));
        let mut negative = manual(label.do_counter(0, -1234567, 1.0, GString::from(",")));
        negative.force_complete();
        assert_eq!(label.get_text(), GString::from("-1,234,567"));
        let ungrouped = manual(label.do_counter(-2000, 2000, 1.0, GString::new()).with_ease(ease()));
        let _ = ungrouped.custom_step(0.25, false);
        assert_eq!(label.get_text(), GString::from("-1000"));
        let _ = ungrouped.custom_step(0.75, false);
        assert_eq!(label.get_text(), GString::from("2000"));
        let rich = RichTextLabel::new_alloc();
        self.base_mut().add_child(&rich);
        let mut style = manual(rich.do_normal_font_size(32, 1.0));
        style.force_complete();
        assert_eq!(rich.get_theme_font_size("normal_font_size"), 32);
        let mut generic_style = manual(
            rich.do_property("normal_font_size", Evaluator::Static(48_i64), 1.0)
                .with_ease(ease()),
        );
        let _ = generic_style.custom_step(0.5, false);
        assert_eq!(rich.get_theme_font_size("normal_font_size"), 40);
        generic_style.force_complete();
        assert_eq!(rich.get_theme_font_size("normal_font_size"), 48);
        drop((font, generic_font, counter, negative, ungrouped, style, generic_style));
        label.free();
        rich.free();
        Box::pin(async {})
    }

    pub(super) fn test_group2_arrays(&mut self) -> PinnedTestTask {
        let node = Node2D::new_alloc();
        self.base_mut().add_child(&node);
        let mut seq = do_property_array(
            node.clone().upcast(),
            NodePath::from("position"),
            vec![Vector2::new(4.0, 0.0), Vector2::new(8.0, 0.0)],
            vec![1.0, 1.0],
        )
        .unwrap();
        seq.force_complete();
        vec2(node.get_position(), Vector2::new(8.0, 0.0));
        let seq = do_property_array(
            node.clone().upcast(),
            NodePath::from("position"),
            vec![Vector2::ZERO, Vector2::ONE],
            vec![1.0, 1.0],
        )
        .unwrap();
        let seq = manual(seq);
        match seq.custom_step(2.5, false) {
            TweenStepResult::Stepped(AdvanceTimeResult::Completed { unscaled_excess_time }) => {
                near(unscaled_excess_time, 0.5)
            }
            _ => panic!("array must complete with half a second remaining"),
        }
        vec2(node.get_position(), Vector2::ONE);
        assert!(
            do_property_array(node.clone().upcast(), NodePath::from("position"), vec![Vector2::ONE], vec![]).is_err()
        );
        for d in [-1.0, f64::NAN, f64::INFINITY] {
            assert!(
                do_property_array(node.clone().upcast(), NodePath::from("position"), vec![Vector2::ONE], vec![d])
                    .is_err()
            );
        }
        let read = node.clone();
        let mut write = node.clone();
        let mut seq = do_get_set_array(
            move || read.get_rotation() as f64,
            move |v: f64| write.set_rotation(v as f32),
            vec![1.0, 2.0],
            vec![0.0, 0.0],
        )
        .unwrap();
        seq.force_complete();
        near(node.get_rotation() as f64, 2.0);
        drop(seq);
        node.free();
        Box::pin(async {})
    }

    pub(super) fn test_group2_gradients_lines(&mut self) -> PinnedTestTask {
        let mut gradient = Gradient::new_gd();
        gradient.set_color(0, Color::from_rgba(0.0, 0.0, 0.0, 0.2));
        gradient.set_color(1, Color::from_rgba(1.0, 1.0, 1.0, 0.8));
        let mut node = Node2D::new_alloc();
        self.base_mut().add_child(&node);
        node.set_modulate(Color::from_rgba(0.1, 0.2, 0.3, 0.9));
        let t = manual(
            node.do_gradient_modulate(gradient.clone(), 1.0, false)
                .with_ease(ease()),
        );
        let _ = t.custom_step(0.5, false);
        near(node.get_modulate().r as f64, 0.5);
        near(node.get_modulate().a as f64, 0.9);
        let mut rgba = manual(node.do_gradient_modulate(gradient.clone(), 1.0, true).with_ease(ease()));
        rgba.force_complete();
        near(node.get_modulate().a as f64, 0.8);
        gradient.add_point(0.5, Color::from_rgba(0.0, 1.0, 0.0, 1.0));
        let mut line = Line2D::new_alloc();
        self.base_mut().add_child(&line);
        line.set_gradient(&gradient);
        let mut ends = manual(line.do_endpoint_colors(
            Color::from_rgba(1.0, 0.0, 0.0, 1.0),
            Color::from_rgba(0.0, 0.0, 1.0, 1.0),
            1.0,
        ));
        ends.force_complete();
        let copy = line.get_gradient().unwrap();
        assert_ne!(copy.instance_id(), gradient.instance_id());
        assert_eq!(copy.get_color(1), gradient.get_color(1));
        near(gradient.get_color(0).r as f64, 0.0);
        drop((t, rgba, ends));
        node.free();
        line.free();
        Box::pin(async {})
    }

    pub(super) fn test_group2_shader_bus(&mut self) -> PinnedTestTask {
        let mut shader = Shader::new_gd();
        shader.set_code(
            "shader_type canvas_item; uniform float gain=1.0; uniform vec3 tint=vec3(0.0); void fragment(){ \
             COLOR=vec4(tint*gain,1.0); }",
        );
        let mut material = ShaderMaterial::new_gd();
        material.set_shader(&shader);
        material.set_shader_parameter("gain", &1.0_f64.to_variant());
        let uniform = manual(
            do_shader_parameter(material.clone(), StringName::from("gain"), 3.0_f64, 1.0)
                .unwrap()
                .with_ease(ease()),
        );
        material.set_shader_parameter("gain", &1.0_f64.to_variant());
        let _ = uniform.custom_step(0.5, false);
        near(material.get_shader_parameter("gain").to::<f64>(), 2.0);
        assert!(do_shader_parameter(material.clone(), StringName::from("missing"), 1.0_f64, 1.0).is_err());
        let mut server = AudioServer::singleton();
        let index = server.get_bus_count();
        server.add_bus();
        server.set_bus_name(index, "SpireGroup2");
        server.set_bus_volume_linear(index, 1.0);
        let mut bus = manual(
            do_bus_volume_linear(StringName::from("SpireGroup2"), 3.0, 1.0)
                .unwrap()
                .with_ease(ease()),
        );
        let _ = bus.custom_step(0.5, false);
        near(server.get_bus_volume_linear(index) as f64, 2.0);
        server.add_bus();
        server.set_bus_name(index + 1, "SpireGroup2Other");
        server.move_bus(index, index + 1);
        let index = server.get_bus_index("SpireGroup2");
        bus.force_complete();
        near(server.get_bus_volume_linear(index) as f64, 3.0);
        let cancelled = manual(do_bus_volume_db(StringName::from("SpireGroup2"), -10.0, 1.0).unwrap());
        server.remove_bus(index);
        assert!(matches!(
            cancelled.custom_step(0.1, false),
            TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)
        ));
        assert!(do_bus_volume_db(StringName::from("SpireGroup2"), 0.0, 1.0).is_err());
        let other = server.get_bus_index("SpireGroup2Other");
        server.remove_bus(other);
        Box::pin(async {})
    }

    pub(super) fn test_group2_template_lifetimes(&mut self) -> PinnedTestTask {
        for forced in [false, true] {
            let node = Node2D::new_alloc();
            self.base_mut().add_child(&node);
            let mut jump = manual(node.do_jump(Vector2::ONE, 2.0, 1, 1.0));
            let events = RcPtr::new([0, 0]);
            let a = events.clone();
            jump.stopped_connect(move || a.to_mut()[0] += 1, SpireFlags::empty());
            let b = events.clone();
            jump.finished_connect(move || b.to_mut()[1] += 1, SpireFlags::empty());
            node.free();
            if forced {
                jump.force_complete();
            } else {
                let _ = jump.custom_step(0.5, false);
            }
            assert!(jump.is_stopped());
            assert_eq!(*events, [1, 0]);
            let mut skeleton = Skeleton3D::new_alloc();
            self.base_mut().add_child(&skeleton);
            skeleton.add_bone("SpireLifetimeBone");
            let mut bones = [
                manual(skeleton.do_bone_position(0, Vector3::ONE, 1.0)),
                manual(skeleton.do_bone_scale(0, Vector3::splat(2.0), 1.0)),
            ];
            let events = RcPtr::new([0, 0]);
            for bone in &mut bones {
                let stopped = events.clone();
                bone.stopped_connect(move || stopped.to_mut()[0] += 1, SpireFlags::empty());
                let finished = events.clone();
                bone.finished_connect(move || finished.to_mut()[1] += 1, SpireFlags::empty());
            }
            skeleton.free();
            for bone in &mut bones {
                if forced {
                    bone.force_complete();
                } else {
                    let _ = bone.custom_step(0.5, false);
                }
                assert!(bone.is_stopped());
            }
            assert_eq!(*events, [2, 0]);
        }
        Box::pin(async {})
    }
}
