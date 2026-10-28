//! Built-in value families exercised through real Godot properties and callables.
use godot::{classes::*, prelude::*};
use spire_tween::prelude::*;

use super::{misc::MiscTests, util::PinnedTestTask};

#[derive(GodotClass)]
#[class(init, base = Node)]
struct TypedValueProbe {
    base: Base<Node>,
    #[var]
    vector4_value: Vector4,
    #[var]
    quaternion_value: Quaternion,
    #[var]
    rect2_value: Rect2,
    #[var]
    variant_value: Variant,
}

#[godot_api]
impl TypedValueProbe {
    #[func]
    fn receive_vector4(&mut self, value: Vector4) { self.vector4_value = value; }
    #[func]
    fn receive_quaternion(&mut self, value: Quaternion) { self.quaternion_value = value; }
    #[func]
    fn receive_rect2(&mut self, value: Rect2) { self.rect2_value = value; }
    #[func]
    fn receive_variant(&mut self, value: Variant) { self.variant_value = value; }
}

macro_rules! manual {
    ($tween:expr) => {
        TweenPtr::new(
            $tween
                .with_process_mode(ProcessMode::Manual)
                .with_ease(EaseKind::Basic(Ease::Linear)),
        )
    };
}

fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 0.00002, "{a} != {b}");
}
fn v4(a: Vector4, b: Vector4) { near(a.distance_to(b) as f64, 0.0); }
fn rect(a: Rect2, b: Rect2) {
    near(a.position.distance_to(b.position) as f64, 0.0);
    near(a.size.distance_to(b.size) as f64, 0.0);
}
fn quat(a: Quaternion, b: Quaternion) {
    near(a.length() as f64, 1.0);
    near(a.dot(b.normalized()).abs() as f64, 1.0);
}
fn rotation(angle: f32) -> Quaternion { Quaternion::from_axis_angle(Vector3::UP, angle) }
fn excess(result: TweenStepResult) -> f64 {
    match result {
        TweenStepResult::Stepped(AdvanceTimeResult::Completed { unscaled_excess_time }) => unscaled_excess_time,
        _ => panic!("expected successful completion"),
    }
}
fn done() -> PinnedTestTask { Box::pin(async {}) }

impl MiscTests {
    pub(super) fn test_typed_vec4_property(&mut self) -> PinnedTestTask {
        let mut source = VisualShaderNodeVec4Constant::new_gd();
        source.set_constant(Quaternion::new(1.0, 2.0, 3.0, 4.0));
        let to = Vector4::new(5.0, 6.0, 7.0, 8.0);
        let t = manual!(source.do_constant(to, 1.0));
        assert!(!t.is_registered());
        assert!(matches!(t.t.data, PropertyDataVec4::VisualShaderNodeVec4Constant(_)));
        let _ = t.custom_step(0.5, false);
        let q = source.get_constant();
        v4(Vector4::new(q.x, q.y, q.z, q.w), Vector4::new(3.0, 4.0, 5.0, 6.0));
        near(excess(t.custom_step(0.75, false)), 0.25);
        let mut component = manual!(source.do_constant_w(12.0, 1.0));
        component.force_complete();
        let q = source.get_constant();
        v4(Vector4::new(q.x, q.y, q.z, q.w), Vector4::new(5.0, 6.0, 7.0, 12.0));
        let t = source.do_property("constant", Evaluator::Static(to), 1.0);
        assert!(matches!(t.t.data, PropertyDataVec4::VisualShaderNodeVec4Constant(_)));
        done()
    }

    pub(super) fn test_typed_rect2_property(&mut self) -> PinnedTestTask {
        let mut node = Sprite2D::new_alloc();
        self.base_mut().add_child(&node);
        let to = Rect2::from_components(4.0, 6.0, 8.0, 10.0);
        let mut t = manual!(node.do_region_rect(to, 1.0));
        assert!(matches!(t.t.data, PropertyDataRect2::Sprite2D(_)));
        node.set_region_rect(Rect2::from_components(2.0, 4.0, 6.0, 8.0));
        let _ = t.custom_step(0.5, false);
        rect(node.get_region_rect(), Rect2::from_components(3.0, 5.0, 7.0, 9.0));
        t.force_complete();
        rect(node.get_region_rect(), to);
        let mut position = manual!(node.do_region_rect_position(Vector2::new(-2.0, -4.0), 1.0));
        position.force_complete();
        rect(node.get_region_rect(), Rect2::from_components(-2.0, -4.0, 8.0, 10.0));
        let mut size = manual!(node.do_region_rect_size(Vector2::new(12.0, 14.0), 1.0));
        size.force_complete();
        rect(node.get_region_rect(), Rect2::from_components(-2.0, -4.0, 12.0, 14.0));
        let t = node.do_property("region_rect", Evaluator::Static(to), 1.0);
        assert!(matches!(t.t.data, PropertyDataRect2::Sprite2D(_)));
        node.free();
        done()
    }

    pub(super) fn test_typed_quaternion_property(&mut self) -> PinnedTestTask {
        let mut node = Node3D::new_alloc();
        self.base_mut().add_child(&node);
        let scale = Vector3::new(2.0, 3.0, 4.0);
        node.set_scale(scale);
        let mut t = manual!(node.do_quaternion(rotation(2.0), 1.0));
        assert!(matches!(t.t.data, PropertyDataQuaternion::Node3D(_)));
        let _ = t.custom_step(0.5, false);
        quat(node.get_quaternion(), rotation(1.0));
        near(node.get_scale().distance_to(scale) as f64, 0.0);
        t.force_complete();
        quat(node.get_quaternion(), rotation(2.0));
        let t = node.do_property("quaternion", Evaluator::Static(rotation(0.5)), 1.0);
        assert!(matches!(t.t.data, PropertyDataQuaternion::Node3D(_)));
        node.free();
        done()
    }

    pub(super) fn test_typed_methods_and_accessors(&mut self) -> PinnedTestTask {
        let node = TypedValueProbe::new_alloc();
        self.base_mut().add_child(&node);
        macro_rules! check {
            ($ty:ty, $field:ident, $method:literal, $from:expr, $to:expr, $mid:expr, $assert:ident) => {{
                let mut t = manual!(node.do_method($method, $from, $to, 1.0));
                let _ = t.custom_step(0.5, false);
                $assert(node.bind().$field, $mid);
                t.force_complete();
                $assert(node.bind().$field, $to);
                let read = node.clone();
                let mut write = node.clone();
                let mut t = manual!(do_get_set(
                    move || read.bind().$field,
                    move |v: $ty| write.bind_mut().$field = v,
                    $from,
                    1.0
                ));
                let _ = t.custom_step(0.5, false);
                $assert(node.bind().$field, $mid);
                t.force_complete();
                $assert(node.bind().$field, $from);
            }};
        }
        check!(Vector4, vector4_value, "receive_vector4", Vector4::ZERO, Vector4::ONE, Vector4::ONE * 0.5, v4);
        check!(
            Quaternion,
            quaternion_value,
            "receive_quaternion",
            Quaternion::IDENTITY,
            rotation(2.0),
            rotation(1.0),
            quat
        );
        check!(
            Rect2,
            rect2_value,
            "receive_rect2",
            Rect2::default(),
            Rect2::from_components(2.0, 4.0, 6.0, 8.0),
            Rect2::from_components(1.0, 2.0, 3.0, 4.0),
            rect
        );
        node.free();
        done()
    }

    pub(super) fn test_typed_relative(&mut self) -> PinnedTestTask {
        let mut node = TypedValueProbe::new_alloc();
        self.base_mut().add_child(&node);
        node.bind_mut().vector4_value = Vector4::ONE * 10.0;
        node.bind_mut().rect2_value = Rect2::from_components(10.0, 20.0, 30.0, 40.0);
        let initial = Quaternion::from_axis_angle(Vector3::RIGHT, 0.7);
        node.bind_mut().quaternion_value = initial;
        let mut a = manual!(
            node.do_property("vector4_value", Evaluator::Static(Vector4::new(2.0, 4.0, 6.0, 8.0)), 1.0)
                .as_relative(Vector4::ZERO)
        );
        let mut b = manual!(
            node.do_property("rect2_value", Evaluator::Static(Rect2::from_components(2.0, 4.0, -6.0, -8.0)), 1.0)
                .as_relative(Rect2::default())
        );
        let mut c = manual!(
            node.do_property("quaternion_value", Evaluator::Static(rotation(1.0)), 1.0)
                .as_relative(Quaternion::IDENTITY)
        );
        let _ = a.custom_step(0.5, false);
        let _ = b.custom_step(0.5, false);
        let _ = c.custom_step(0.5, false);
        v4(node.bind().vector4_value, Vector4::new(11.0, 12.0, 13.0, 14.0));
        rect(node.bind().rect2_value, Rect2::from_components(11.0, 22.0, 27.0, 36.0));
        quat(node.bind().quaternion_value, initial * rotation(0.5));
        let external = Quaternion::from_axis_angle(Vector3::FORWARD, 0.3);
        let current = node.bind().quaternion_value;
        node.bind_mut().quaternion_value = external * current;
        // External writes between ticks must survive completion/restoration.
        node.bind_mut().vector4_value += Vector4::ONE * 100.0;
        node.bind_mut().rect2_value.position += Vector2::ONE * 100.0;
        a.force_complete();
        b.force_complete();
        c.force_complete();
        v4(node.bind().vector4_value, Vector4::new(112.0, 114.0, 116.0, 118.0));
        rect(node.bind().rect2_value, Rect2::from_components(112.0, 124.0, 24.0, 32.0));
        quat(node.bind().quaternion_value, external * initial * rotation(1.0));
        node.free();
        done()
    }

    pub(super) fn test_typed_speed(&mut self) -> PinnedTestTask {
        let node = TypedValueProbe::new_alloc();
        self.base_mut().add_child(&node);
        let mut a = manual!(
            node.do_property("vector4_value", Evaluator::Static(Vector4::new(0.0, 0.0, 3.0, 4.0)), 2.0)
                .as_speed_based()
        );
        let mut b = manual!(
            node.do_property("rect2_value", Evaluator::Static(Rect2::from_components(0.0, 0.0, 3.0, 4.0)), 2.0)
                .as_speed_based()
        );
        let mut c = manual!(
            node.do_property("quaternion_value", Evaluator::Static(rotation(2.0)), 0.8)
                .as_speed_based()
        );
        near(a.get_duration(), 2.5);
        near(b.get_duration(), 2.5);
        near(c.get_duration(), 2.5);
        let _ = a.custom_step(0.5, false);
        let _ = b.custom_step(0.5, false);
        let _ = c.custom_step(0.5, false);
        v4(node.bind().vector4_value, Vector4::new(0.0, 0.0, 0.6, 0.8));
        rect(node.bind().rect2_value, Rect2::from_components(0.0, 0.0, 0.6, 0.8));
        quat(node.bind().quaternion_value, rotation(0.4));
        near(excess(a.custom_step(3.0, false)), 1.0);
        near(excess(b.custom_step(3.0, false)), 1.0);
        near(excess(c.custom_step(3.0, false)), 1.0);
        quat(node.bind().quaternion_value, rotation(2.0));
        // Equal endpoints complete with all input time left, including q == -q.
        let same = manual!(
            node.do_property("quaternion_value", Evaluator::Static(-rotation(2.0)), 1.0)
                .as_speed_based()
        );
        near(excess(same.custom_step(0.75, false)), 0.75);
        node.free();
        done()
    }

    pub(super) fn test_typed_sequence_and_erasure(&mut self) -> PinnedTestTask {
        let mut node = TypedValueProbe::new_alloc();
        self.base_mut().add_child(&node);
        macro_rules! check {
            ($field:ident, $from:expr, $to:expr) => {{
                node.bind_mut().$field = $from;
                let mut seq = TweenSequence::new().with_process_mode(ProcessMode::Manual);
                let child = manual!(
                    node.do_property(stringify!($field), Evaluator::Static($to), 1.0)
                        .as_speed_based()
                        .with_speed_scale(2.0)
                );
                let weak = child.downgrade();
                let mut erased: AnyPropertyTween = child.clone().into();
                erased.set_final_value(($to).to_variant()).unwrap();
                seq.append_ptr(child);
                let seen = RcPtr::new(0.0);
                let write = seen.clone();
                seq.append(
                    TweenMethod::<f64>::new(
                        Callable::from_fn("typed_sequence", move |args| *write.to_mut() = args[0].to::<f64>()),
                        0.0,
                        1.0,
                        1.0,
                    )
                    .with_ease(EaseKind::Basic(Ease::Linear)),
                );
                let seq = TweenPtr::new(seq);
                // All these destinations are one unit from their starts.
                let _ = seq.custom_step(0.75, false);
                near(*seen, 0.25);
                drop(erased);
                drop(seq);
                assert!(weak.upgrade().is_none());
            }};
        }
        check!(vector4_value, Vector4::ZERO, Vector4::new(0.0, 0.0, 0.0, 1.0));
        check!(rect2_value, Rect2::default(), Rect2::from_components(0.0, 0.0, 0.0, 1.0));
        check!(quaternion_value, Quaternion::IDENTITY, rotation(1.0));
        node.free();
        done()
    }

    pub(super) fn test_typed_loops(&mut self) -> PinnedTestTask {
        let mut node = TypedValueProbe::new_alloc();
        self.base_mut().add_child(&node);
        macro_rules! check {
            ($field:ident, $from:expr, $to:expr, $half:expr, $increment:expr, $assert:ident) => {{
                for mode in [LoopMode::Yoyo, LoopMode::Incremental, LoopMode::Restart] {
                    node.bind_mut().$field = $from;
                    let mut t = manual!(
                        node.do_property(stringify!($field), Evaluator::Static($to), 1.0)
                            .begin_from($from)
                            .with_loops(2, mode)
                    );
                    let _ = t.custom_step(1.0, false);
                    let _ = t.custom_step(0.5, false);
                    $assert(node.bind().$field, if mode == LoopMode::Incremental { $increment } else { $half });
                    let _ = t.custom_step(0.5, false);
                    assert!(t.is_stopped());
                    t.play();
                    let _ = t.custom_step(0.5, false);
                    $assert(node.bind().$field, $half);
                    t.stop();
                }
            }};
        }
        check!(vector4_value, Vector4::ZERO, Vector4::ONE * 2.0, Vector4::ONE, Vector4::ONE * 3.0, v4);
        check!(
            rect2_value,
            Rect2::default(),
            Rect2::from_components(2.0, 4.0, 6.0, 8.0),
            Rect2::from_components(1.0, 2.0, 3.0, 4.0),
            Rect2::from_components(3.0, 6.0, 9.0, 12.0),
            rect
        );
        check!(quaternion_value, Quaternion::IDENTITY, rotation(1.0), rotation(0.5), rotation(1.5), quat);
        node.free();
        done()
    }

    pub(super) fn test_typed_dynamic_targets(&mut self) -> PinnedTestTask {
        let node = TypedValueProbe::new_alloc();
        self.base_mut().add_child(&node);
        macro_rules! check {
            ($field:ident, $a:expr, $b:expr, $assert:ident) => {{
                let target = RcPtr::new($a);
                let read = target.clone();
                let mut t =
                    manual!(node.do_property(stringify!($field), Evaluator::Dynamic(Box::new(move || *read)), 1.0));
                *target.to_mut() = $b;
                t.force_complete();
                $assert(node.bind().$field, $b);
            }};
        }
        check!(vector4_value, Vector4::ZERO, Vector4::ONE, v4);
        check!(quaternion_value, Quaternion::IDENTITY, rotation(1.0), quat);
        check!(rect2_value, Rect2::default(), Rect2::from_components(1.0, 2.0, 3.0, 4.0), rect);
        node.free();
        done()
    }

    pub(super) fn test_typed_cancellation(&mut self) -> PinnedTestTask {
        let node = TypedValueProbe::new_alloc();
        self.base_mut().add_child(&node);
        let counts = RcPtr::new([0, 0]);
        let mut tweens: Vec<AnyTween> = vec![
            manual!(node.do_property("vector4_value", Evaluator::Static(Vector4::ONE), 1.0)).into(),
            manual!(node.do_property("quaternion_value", Evaluator::Static(rotation(1.0)), 1.0)).into(),
            manual!(node.do_property(
                "rect2_value",
                Evaluator::Static(Rect2::from_components(1.0, 2.0, 3.0, 4.0)),
                1.0
            ))
            .into(),
            manual!(node.do_method("receive_vector4", Vector4::ZERO, Vector4::ONE, 1.0)).into(),
            manual!(node.do_method("receive_quaternion", Quaternion::IDENTITY, rotation(1.0), 1.0)).into(),
            manual!(node.do_method("receive_rect2", Rect2::default(), Rect2::from_components(1.0, 2.0, 3.0, 4.0), 1.0))
                .into(),
        ];
        for t in &mut tweens {
            let stopped = counts.clone();
            t.stopped_connect(move || stopped.to_mut()[0] += 1, SpireFlags::empty());
            let finished = counts.clone();
            t.finished_connect(move || finished.to_mut()[1] += 1, SpireFlags::empty());
        }
        node.free();
        for t in &mut tweens {
            assert!(matches!(t.custom_step(0.5, false), TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
            assert!(t.is_stopped());
            t.force_complete();
        }
        assert_eq!(*counts, [6, 0]);
        done()
    }

    pub(super) fn test_typed_quaternion_edges(&mut self) -> PinnedTestTask {
        let node = TypedValueProbe::new_alloc();
        self.base_mut().add_child(&node);
        for (from, to, expected) in [
            (Quaternion::IDENTITY, -rotation(1.0), rotation(0.5)),
            (rotation(0.3), -rotation(0.3), rotation(0.3)),
            (Quaternion::IDENTITY * 3.0, rotation(1.0) * 2.0, rotation(0.5)),
            (Quaternion::IDENTITY, rotation(0.001), rotation(0.0005)),
            (Quaternion::IDENTITY, Quaternion::new(0.0, 1.0, 0.0, 0.0), rotation(std::f32::consts::FRAC_PI_2)),
            (Quaternion::new(0.0, 0.0, 0.0, 0.0), rotation(1.0), rotation(0.5)),
            (Quaternion::new(f32::NAN, 0.0, 0.0, 1.0), rotation(1.0), rotation(0.5)),
        ] {
            let mut t = manual!(node.do_method("receive_quaternion", from, to, 1.0));
            let _ = t.custom_step(0.5, false);
            quat(node.bind().quaternion_value, expected);
            t.stop();
        }
        let mut t = manual!(node.do_method("receive_quaternion", Quaternion::IDENTITY, rotation(1.0), 1.0));
        t.set_ease(EaseKind::Callable(Callable::from_fn("quaternion_overshoot", |_| 1.5.to_variant())));
        let _ = t.custom_step(0.5, false);
        quat(node.bind().quaternion_value, rotation(1.5));
        t.set_ease(EaseKind::Callable(Callable::from_fn("quaternion_undershoot", |_| (-0.5).to_variant())));
        let _ = t.custom_step(0.0, false);
        quat(node.bind().quaternion_value, rotation(-0.5));
        t.stop();
        let mut small = manual!(
            node.do_property("quaternion_value", Evaluator::Static(rotation(0.001)), 0.0002)
                .as_speed_based()
        );
        // Check components rather than dot alone: identity must fail this fixture.
        node.clone().bind_mut().quaternion_value = Quaternion::IDENTITY;
        let _ = small.custom_step(1.0, false);
        near(node.bind().quaternion_value.y as f64, (0.0001_f64).sin());
        small.stop();
        let mut t = manual!(node.do_property("quaternion_value", Evaluator::Static(rotation(0.7) * 5.0), 1.0));
        t.force_complete();
        quat(node.bind().quaternion_value, rotation(0.7));
        node.free();
        done()
    }

    pub(super) fn test_typed_quaternion_completion(&mut self) -> PinnedTestTask {
        let mut node = TypedValueProbe::new_alloc();
        self.base_mut().add_child(&node);
        let initial = Quaternion::from_axis_angle(Vector3::RIGHT, 0.4);
        for mode in 0..3 {
            for paused in [false, true] {
                for (to, expected) in [
                    (rotation(0.7) * 5.0, rotation(0.7)),
                    (Quaternion::new(0.0, 0.0, 0.0, 0.0), Quaternion::IDENTITY),
                    (Quaternion::new(f32::NAN, 0.0, 0.0, 1.0), Quaternion::IDENTITY),
                ] {
                    node.bind_mut().quaternion_value = initial;
                    let mut builder = node.do_property("quaternion_value", Evaluator::Static(to), 1.0);
                    if mode == 1 {
                        builder.set_speed_based();
                    }
                    if mode == 2 {
                        builder.set_relative(Quaternion::IDENTITY);
                    }
                    let mut t = manual!(builder);
                    let counts = RcPtr::new([0, 0]);
                    let stopped = counts.clone();
                    t.stopped_connect(move || stopped.to_mut()[0] += 1, SpireFlags::empty());
                    let finished = counts.clone();
                    t.finished_connect(move || finished.to_mut()[1] += 1, SpireFlags::empty());
                    if paused {
                        t.pause();
                    }
                    t.force_complete();
                    quat(node.bind().quaternion_value, if mode == 2 { initial * expected } else { expected });
                    assert_eq!(*counts, [1, 1]);
                }
            }
        }

        // This natural-completion branch bypasses both spire_lerp and spire_step.
        for to in [rotation(0.7) * 5.0, -rotation(0.7) * 3.0] {
            node.bind_mut().quaternion_value = rotation(0.7);
            let t = manual!(
                node.do_property("quaternion_value", Evaluator::Static(to), 1.0)
                    .as_speed_based()
            );
            near(excess(t.custom_step(0.25, false)), 0.25);
            quat(node.bind().quaternion_value, rotation(0.7));
        }

        let raw = rotation(0.4) * 3.0;
        node.bind_mut().quaternion_value = raw;
        let mut stopped = manual!(node.do_property("quaternion_value", Evaluator::Static(rotation(0.7)), 1.0));
        stopped.stop();
        stopped.force_complete();
        assert_eq!(node.bind().quaternion_value, raw, "stopped completion must not normalize or write");

        let mut method = manual!(node.do_method("receive_quaternion", Quaternion::IDENTITY, rotation(0.7) * 5.0, 1.0));
        method.force_complete();
        quat(node.bind().quaternion_value, rotation(0.7));

        // Other/custom lerpers retain the raw-endpoint completion behavior.
        // Reusing spire_lerp(end, end, 1) here would incorrectly invoke this callback.
        let calls = RcPtr::new(0);
        let invoked = calls.clone();
        let mut custom = manual!(TweenProperty::<Variant>::new_custom(
            "variant_value",
            node.clone().upcast::<Node>(),
            Evaluator::Static(12.0.to_variant()),
            1.0,
            CustomLerper::new(
                Callable::from_fn("completion_lerper", move |_| {
                    *invoked.to_mut() += 1;
                    999.0.to_variant()
                }),
                Callable::invalid(),
                Callable::invalid(),
                Callable::invalid()
            ),
        ));
        custom.force_complete();
        assert_eq!(*calls, 0);
        near(node.bind().variant_value.to::<f64>(), 12.0);
        node.free();
        done()
    }

    pub(super) fn test_typed_shader_vec4(&mut self) -> PinnedTestTask {
        let mut shader = Shader::new_gd();
        shader.set_code("shader_type canvas_item; uniform vec4 typed_value; void fragment() { COLOR = typed_value; }");
        let mut material = ShaderMaterial::new_gd();
        material.set_shader(&shader);
        material.set_shader_parameter("typed_value", &Vector4::ZERO.to_variant());
        let mut t = manual!(
            do_shader_parameter(material.clone(), "typed_value".into(), Vector4::new(2.0, 4.0, 6.0, 8.0), 1.0).unwrap()
        );
        let _ = t.custom_step(0.5, false);
        v4(material.get_shader_parameter("typed_value").to(), Vector4::new(1.0, 2.0, 3.0, 4.0));
        t.force_complete();
        v4(material.get_shader_parameter("typed_value").to(), Vector4::new(2.0, 4.0, 6.0, 8.0));
        assert!(do_shader_parameter(material.clone(), "missing".into(), Vector4::ONE, 1.0).is_err());
        assert!(do_shader_parameter(material.clone(), "typed_value".into(), 1.0_f64, 1.0).is_err());
        let t = manual!(do_shader_parameter(material.clone(), "typed_value".into(), Vector4::ZERO, 1.0).unwrap());
        material.set_shader(&Shader::new_gd());
        assert!(matches!(t.custom_step(0.5, false), TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
        done()
    }

    pub(super) fn test_typed_variant_dispatch(&mut self) -> PinnedTestTask {
        let mut node = TypedValueProbe::new_alloc();
        self.base_mut().add_child(&node);
        macro_rules! check {
            ($field:ident, $from:expr, $to:expr, $mid:expr, $assert:ident) => {{
                for mode in 0..3 {
                    node.bind_mut().$field = $from;
                    let mut builder = node.do_property(stringify!($field), Evaluator::Static(($to).to_variant()), 1.0);
                    if mode == 1 {
                        builder.set_relative(($from).to_variant());
                    }
                    if mode == 2 {
                        builder.set_speed_based();
                    }
                    let mut t = manual!(builder);
                    let delta = if mode == 2 { 0.5 * t.get_duration() } else { 0.5 };
                    let _ = t.custom_step(delta, false);
                    $assert(node.bind().$field, $mid);
                    t.force_complete();
                    $assert(node.bind().$field, $to);
                }
            }};
        }
        check!(vector4_value, Vector4::ZERO, Vector4::ONE * 2.0, Vector4::ONE, v4);
        check!(
            rect2_value,
            Rect2::default(),
            Rect2::from_components(2.0, 4.0, 6.0, 8.0),
            Rect2::from_components(1.0, 2.0, 3.0, 4.0),
            rect
        );
        check!(quaternion_value, Quaternion::IDENTITY, rotation(1.0), rotation(0.5), quat);
        node.free();
        done()
    }

    pub(super) fn test_variant_common_types(&mut self) -> PinnedTestTask {
        let mut node = TypedValueProbe::new_alloc();
        self.base_mut().add_child(&node);
        macro_rules! case {
            ($from:expr, $to:expr, $expected:expr) => {
                (($from).to_variant(), ($to).to_variant(), ($expected).to_variant())
            };
        }
        for (from, to, expected) in [
            case!(0_i64, 3_i64, 2_i64),
            case!(0.0_f64, 3.0_f64, 1.5_f64),
            case!(1_i64, 4.0_f64, 2.5_f64),
            case!(1.0_f64, 4_i64, 2.5_f64),
            case!(Vector2::new(2.0, 4.0), Vector2::new(6.0, 10.0), Vector2::new(4.0, 7.0)),
            case!(Vector2i::new(0, 1), Vector2i::new(3, 4), Vector2i::new(2, 3)),
            case!(Vector3::new(1.0, 2.0, 3.0), Vector3::new(3.0, 6.0, 9.0), Vector3::new(2.0, 4.0, 6.0)),
            case!(Vector3i::ZERO, Vector3i::new(1, 3, 5), Vector3i::new(1, 2, 3)),
            case!(
                Color::from_rgba(0.0, 0.25, 0.5, 0.75),
                Color::from_rgba(1.0, 0.75, 0.0, 0.25),
                Color::from_rgba(0.5, 0.5, 0.25, 0.5)
            ),
            case!(GString::from("abcd"), GString::from("WXYZ"), GString::from("WXcd")),
            case!(StringName::from("abcd"), StringName::from("WXYZ"), GString::from("WXcd")),
            case!(NodePath::from("abcd"), NodePath::from("WXYZ"), GString::from("WXcd")),
            case!(GString::from("a😊界d"), GString::from("Ωx😁z"), GString::from("Ωx界d")),
        ] {
            for property in [false, true] {
                node.bind_mut().variant_value = from.clone();
                let tween: AnyTween = if property {
                    manual!(node.do_property("variant_value", Evaluator::Static(to.clone()), 1.0)).into()
                } else {
                    manual!(node.do_method("receive_variant", from.clone(), to.clone(), 1.0)).into()
                };
                let _ = tween.custom_step(0.5, false);
                let value = node.bind().variant_value.clone();
                assert_eq!(value.get_type(), expected.get_type(), "wrong output type for {from:?} -> {to:?}");
                assert_eq!(value, expected, "wrong midpoint for {from:?} -> {to:?}");
            }
        }
        node.free();
        done()
    }

    pub(super) fn test_variant_conversion_and_override(&mut self) -> PinnedTestTask {
        let node = TypedValueProbe::new_alloc();
        self.base_mut().add_child(&node);
        godot_print!("EXPECTED_VARIANT_CONVERSION_BEGIN");
        for from in [
            1_i64.to_variant(),
            Vector2::ONE.to_variant(),
            GString::from("text").to_variant(),
        ] {
            let tween =
                manual!(node.do_method("receive_variant", from.clone(), VarDictionary::new().to_variant(), 1.0));
            let _ = tween.custom_step(0.5, false);
            assert_eq!(node.bind().variant_value, from, "failed conversions must retain the starting value");
        }
        godot_print!("EXPECTED_VARIANT_CONVERSION_END");
        let mut custom = manual!(TweenMethod::<Variant>::new_custom(
            node.callable("receive_variant"),
            0_i64.to_variant(),
            3_i64.to_variant(),
            1.0,
            Callable::from_fn("variant_lerp_override", |_| 777.0_f64.to_variant()),
        ));
        let _ = custom.custom_step(0.5, false);
        near(node.bind().variant_value.to::<f64>(), 777.0);
        custom.stop();
        node.free();
        done()
    }
}
